use mycelium_core::config::{AnalysisConfig, AnalysisResult};
use mycelium_core::mermaid::{export_mermaid, DetailMode, MermaidOptions};
use mycelium_core::pipeline::run_pipeline;

// Preserve the historical detailed-output assertions explicitly.
fn full_options() -> MermaidOptions {
    MermaidOptions {
        detail: DetailMode::Full,
        ..Default::default()
    }
}

fn analyze(files: &[(&str, &str)]) -> AnalysisResult {
    let repo = tempfile::tempdir().unwrap();
    for (path, source) in files {
        let target = repo.path().join(path);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(target, source).unwrap();
    }
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    // Export must work from saved JSON after the source directory has been removed.
    serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap()
}

fn export(result: &AnalysisResult) -> String {
    export_mermaid(result, &full_options()).unwrap()
}

fn box_id<'a>(output: &'a str, file: &str, name: &str) -> &'a str {
    let location = format!("`{name}` — `{file}`:");
    output
        .lines()
        .find(|line| line.starts_with("- c") && line.contains(&location))
        .unwrap_or_else(|| panic!("Missing {location}:\n{output}"))
        .strip_prefix("- ")
        .unwrap()
        .split(':')
        .next()
        .unwrap()
}

fn assert_edge(output: &str, from: (&str, &str), to: (&str, &str), arrow: &str, label: &str) {
    let from = box_id(output, from.0, from.1);
    let to = box_id(output, to.0, to.1);
    let edge = format!("- {from} {arrow} {to}: `{label}`");
    assert!(output.contains(&edge), "Missing {edge}:\n{output}");
}

#[test]
fn explicit_imports_select_each_duplicate_type_before_a_same_directory_decoy() {
    // Arrange: the import, not the class spelling or directory, identifies each enum.
    let mut result = analyze(&[
        ("models/activity.py", "class EntityType: pass\n"),
        ("models/entity.py", "class EntityType: pass\n"),
        ("services/decoy.py", "class EntityType: pass\n"),
        (
            "services/activity.py",
            concat!(
                "from models.activity import EntityType\n",
                "class ActivityService:\n",
                "    def log(self, kind: EntityType) -> EntityType: pass\n",
            ),
        ),
        (
            "services/entity.py",
            concat!(
                "from models.entity import EntityType\n",
                "class EntityService:\n    kind: EntityType\n",
            ),
        ),
    ]);

    // Act.
    let output = export(&result);

    // Assert: verify endpoints against source locations, not duplicate display names.
    let activity = box_id(&output, "models/activity.py", "EntityType");
    let entity = box_id(&output, "models/entity.py", "EntityType");
    let activity_service = box_id(&output, "services/activity.py", "ActivityService");
    let entity_service = box_id(&output, "services/entity.py", "EntityService");
    assert!(output.contains(&format!("{activity_service} ..> {activity} : type in log")));
    assert!(output.contains(&format!("{entity_service} --> {entity} : field kind")));
    assert!(!output.contains("Ambiguous type"), "{output}");
    let decoy = box_id(&output, "services/decoy.py", "EntityType");
    assert!(!output.contains(&format!("..> {decoy} :")), "{output}");
    assert!(!output.contains(&format!("--> {decoy} :")), "{output}");
    result.class_diagram.as_mut().unwrap().classes.reverse();
    result.imports.file_imports.reverse();
    assert_eq!(output, export(&result));
}

#[test]
fn aliases_relative_imports_and_qualified_modules_resolve_fields_signatures_and_bases() {
    // Arrange: both enum names are used in one file, with an aliased base and a local type.
    let result = analyze(&[
        ("app/models/activity.py", "class EntityType: pass\n"),
        ("app/models/entity.py", "class EntityType: pass\n"),
        ("app/models/__init__.py", "class Base: pass\n"),
        (
            "app/services/entity.py",
            concat!(
                "from ..models.activity import (\n",
                "    EntityType as ActivityEntityType,  # keep the binding across comments\n)\n",
                "from app.models.entity import EntityType\n",
                "from ..models import Base as Parent\n",
                "import app.models.activity as activity\n",
                "import app.models.entity\n",
                "from ..models import entity as entity_module\n",
                "class Local: pass\n",
                "class Service(Parent):\n",
                "    activity: ActivityEntityType\n",
                "    entity: EntityType\n",
                "    qualified: activity.EntityType\n",
                "    dotted: app.models.entity.EntityType\n",
                "    module: entity_module.EntityType\n",
                "    local: Local\n",
                "    def log(self, kind: 'ActivityEntityType') -> list[EntityType]: pass\n",
            ),
        ),
    ]);

    // Act.
    let output = export(&result);

    // Assert.
    let service = ("app/services/entity.py", "Service");
    for (file, name, arrow, label) in [
        (
            "app/models/activity.py",
            "EntityType",
            "-->",
            "field activity",
        ),
        ("app/models/entity.py", "EntityType", "-->", "field entity"),
        (
            "app/models/activity.py",
            "EntityType",
            "-->",
            "field qualified",
        ),
        ("app/models/entity.py", "EntityType", "-->", "field dotted"),
        ("app/models/entity.py", "EntityType", "-->", "field module"),
        ("app/models/activity.py", "EntityType", "..>", "type in log"),
        ("app/models/entity.py", "EntityType", "..>", "type in log"),
        ("app/models/__init__.py", "Base", "--|>", "inherits"),
        ("app/services/entity.py", "Local", "-->", "field local"),
    ] {
        assert_edge(&output, service, (file, name), arrow, label);
    }
    assert!(!output.contains("Ambiguous type"), "{output}");
    assert!(
        !output.contains("Unresolved or out-of-scope base"),
        "{output}"
    );
}

#[test]
fn uncertain_bindings_never_fall_back_to_a_nearby_type() {
    for imports in [
        "from external import EntityType\n",
        "from models.activity import EntityType\nfrom models.entity import EntityType\n",
        "from models.activity import EntityType\nEntityType = object()\n",
        "from models.activity import EntityType\nclass EntityType: pass\n",
        "from models.activity import *\n",
        "from models.activity import EntityType\nfrom external import *\n",
        "if TYPE_CHECKING:\n    from models.activity import EntityType\n",
        "def setup():\n    from models.activity import EntityType\n",
        "from models.activity import EntityType\ndef setup(EntityType): pass\n",
        "from models.activity import EntityType\nEntityType, other = object(), None\n",
        "from models.activity import EntityType\nfor EntityType in values: pass\n",
        "from models.activity import EntityType\nwith resource() as EntityType: pass\n",
        "from models.activity import EntityType\ndel EntityType\n",
        "from models.activity import EntityType\nbroken = (\n",
    ] {
        // Arrange: even an otherwise unique name must not override binding evidence.
        let source = format!("{imports}class Service(EntityType):\n    kind: EntityType\n");
        let result = analyze(&[
            ("models/activity.py", "class EntityType: pass\n"),
            ("models/entity.py", "class EntityType: pass\n"),
            ("services/decoy.py", "class EntityType: pass\n"),
            ("services/service.py", &source),
        ]);

        // Act.
        let output = export(&result);

        // Assert: call heuristics are separate; check only declaration relationships.
        assert!(!output.contains("`field kind`"), "{imports}\n{output}");
        assert!(!output.contains(": `inherits`"), "{imports}\n{output}");
    }
}

#[test]
fn imported_targets_must_be_unambiguous_module_level_class_definitions() {
    for (activity, extra_file, extra_source) in [
        (
            "class Outer:\n    class EntityType: pass\n",
            "unused.py",
            "",
        ),
        (
            "class EntityType: pass\nEntityType = factory()\n",
            "unused.py",
            "",
        ),
        (
            "class EntityType: pass\n",
            "models/activity/__init__.py",
            "class EntityType: pass\n",
        ),
    ] {
        // Arrange: nested classes, rebinding and conflicting modules are uncertain.
        let result = analyze(&[
            ("models/activity.py", activity),
            ("models/entity.py", "class EntityType: pass\n"),
            (extra_file, extra_source),
            (
                "services/service.py",
                "from models.activity import EntityType\nclass Service:\n    kind: EntityType\n",
            ),
        ]);

        // Act / Assert.
        let output = export(&result);
        assert!(!output.contains("`field kind`"), "{activity}\n{output}");
    }
}

#[test]
fn filtering_an_imported_target_never_redirects_the_relationship() {
    // Arrange: filtering must happen after binding resolution, while identities are still complete.
    let result = analyze(&[
        ("models/activity.py", "class EntityType: pass\n"),
        ("services/decoy.py", "class EntityType: pass\n"),
        (
            "services/service.py",
            concat!(
                "from models.activity import EntityType\n",
                "class Service(EntityType):\n    kind: EntityType\n",
            ),
        ),
    ]);

    // Act / Assert.
    for options in [
        MermaidOptions {
            detail: DetailMode::Full,
            path: "services".into(),
            ..Default::default()
        },
        MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec!["models/activity.py".into()],
            ..Default::default()
        },
    ] {
        let output = export_mermaid(&result, &options).unwrap();
        assert!(!output.contains("`field kind`"), "{output}");
        assert!(!output.contains(": `inherits`"), "{output}");
        assert!(!output.contains("Ambiguous type"), "{output}");
        if !options.test_paths.is_empty() {
            assert!(
                output.contains("Type relationships removed: 2."),
                "{output}"
            );
            assert!(
                !output.contains("Unresolved or out-of-scope base"),
                "{output}"
            );
        }
    }
}

#[test]
fn older_maps_keep_same_file_resolution_and_request_reanalysis_for_import_bindings() {
    // Arrange: remove the additive metadata exactly as in an older saved map.
    let result = analyze(&[(
        "models.py",
        "class User: pass\nclass Service:\n    user: User\n",
    )]);
    let mut saved = serde_json::to_value(result).unwrap();
    saved["class_diagram"]
        .as_object_mut()
        .unwrap()
        .remove("python_bindings");
    let result: AnalysisResult = serde_json::from_value(saved).unwrap();

    // Act.
    let output = export(&result);

    // Assert.
    assert_edge(
        &output,
        ("models.py", "Service"),
        ("models.py", "User"),
        "-->",
        "field user",
    );
    assert!(
        output.contains("Python import bindings unavailable; rerun analysis"),
        "{output}"
    );
}

#[test]
fn dotted_imports_bind_only_the_modules_that_were_imported() {
    for (imports, activity_imported, entity_imported) in [
        ("import models.activity\n", true, false),
        ("import models.activity\nimport models.entity\n", true, true),
        ("import models\n", false, false),
        ("import models as models\n", false, false),
    ] {
        // Arrange: loading one submodule does not establish bindings to its siblings.
        let source = format!(
            "{imports}{}",
            concat!(
                "class Service:\n",
                "    activity: models.activity.EntityType\n",
                "    entity: models.entity.EntityType\n",
            )
        );
        let result = analyze(&[
            ("models/activity.py", "class EntityType: pass\n"),
            ("models/entity.py", "class EntityType: pass\n"),
            ("service.py", &source),
        ]);

        // Act.
        let output = export(&result);

        // Assert: shared package roots must neither hide nor invent an imported module.
        if activity_imported {
            assert_edge(
                &output,
                ("service.py", "Service"),
                ("models/activity.py", "EntityType"),
                "-->",
                "field activity",
            );
        } else {
            assert!(!output.contains("`field activity`"), "{imports}\n{output}");
        }
        assert_eq!(
            output.contains("`field entity`"),
            entity_imported,
            "{imports}"
        );
    }
}

#[test]
fn python_type_parameter_and_alias_bindings_do_not_select_an_imported_class() {
    for declaration in [
        "type EntityType = int\nclass Service:\n    kind: EntityType\n",
        "class Service[EntityType]:\n    kind: EntityType\n",
        "class Service:\n    def run[EntityType](self, value: EntityType) -> EntityType: pass\n",
    ] {
        // Arrange: Python 3.12 type bindings can shadow a normal import.
        let source = format!("from models import EntityType\n{declaration}");
        let result = analyze(&[
            ("models.py", "class EntityType: pass\n"),
            ("service.py", &source),
        ]);

        // Act.
        let output = export(&result);

        // Assert.
        assert!(output.contains("[\"Service\"]"), "{output}");
        assert!(!output.contains("`field kind`"), "{output}");
        assert!(!output.contains("`type in run`"), "{output}");
    }
}

#[test]
fn same_file_nested_classes_keep_unambiguous_relationships() {
    // Arrange: a nested declaration is not a conflicting module-level binding.
    let result = analyze(&[(
        "models.py",
        "class Order:\n    class Status: pass\n    status: Status\n",
    )]);

    // Act.
    let output = export(&result);

    // Assert.
    assert_edge(
        &output,
        ("models.py", "Order"),
        ("models.py", "Status"),
        "-->",
        "field status",
    );
}

#[test]
fn imported_package_attributes_are_not_mistaken_for_submodules() {
    for package in [
        "class models:\n    class Entity: pass\n",
        "models = factory()\n",
        "from external import models\n",
        "if flag:\n    models = factory()\n",
        "from external import *\n",
    ] {
        // Arrange: from-import checks the package attribute before loading a submodule.
        let result = analyze(&[
            ("pkg/__init__.py", package),
            ("pkg/models.py", "class Entity: pass\n"),
            (
                "service.py",
                concat!(
                    "from pkg import models\n",
                    "import pkg.models as direct\n",
                    "class Service:\n",
                    "    uncertain: models.Entity\n",
                    "    direct: direct.Entity\n",
                ),
            ),
        ]);

        // Act.
        let output = export(&result);

        // Assert: an explicit module import still identifies the actual module.
        assert!(!output.contains("`field uncertain`"), "{package}\n{output}");
        assert_edge(
            &output,
            ("service.py", "Service"),
            ("pkg/models.py", "Entity"),
            "-->",
            "field direct",
        );
    }
}

#[test]
fn reexports_follow_original_class_and_module_bindings() {
    // Arrange: aliases cross package boundaries and preserve the class/module distinction.
    let result = analyze(&[
        ("models/events.py", "class EventBus: pass\n"),
        (
            "api/bridge.py",
            "from models.events import EventBus as Exported\n",
        ),
        (
            "api/__init__.py",
            concat!(
                "from .bridge import Exported as EventBus\n",
                "import models.events as events\n",
            ),
        ),
        ("api/events.py", "class EventBus: pass\n"),
        (
            "service.py",
            concat!(
                "from api import EventBus, events\n",
                "class Service(EventBus):\n",
                "    bus: EventBus\n",
                "    qualified: events.EventBus\n",
            ),
        ),
    ]);

    // Act.
    let output = export(&result);

    // Assert: the re-exported module must win over api/events.py too.
    for (arrow, label) in [
        ("--|>", "inherits"),
        ("-->", "field bus"),
        ("-->", "field qualified"),
    ] {
        assert_edge(
            &output,
            ("service.py", "Service"),
            ("models/events.py", "EventBus"),
            arrow,
            label,
        );
    }
}

#[test]
fn typing_guards_preserve_annotation_imports_through_reexports() {
    for (import, guard) in [
        ("from typing import TYPE_CHECKING", "TYPE_CHECKING"),
        ("from typing import TYPE_CHECKING as checking", "checking"),
        ("import typing", "typing.TYPE_CHECKING"),
        ("import typing as t", "t.TYPE_CHECKING"),
    ] {
        // Arrange: TYPE_CHECKING imports provide evidence for static annotations.
        let source = format!(
            "{import}\nif {guard}:\n    from events import EventBus\n{}",
            "class Service:\n    def __init__(self, bus: 'EventBus | None'): pass\n"
        );
        let result = analyze(&[
            ("events/bus.py", "class EventBus: pass\n"),
            ("events/__init__.py", "from .bus import EventBus\n"),
            ("service.py", &source),
        ]);

        // Act.
        let output = export(&result);

        // Assert.
        assert_edge(
            &output,
            ("service.py", "Service"),
            ("events/bus.py", "EventBus"),
            "..>",
            "type in __init__",
        );
    }
}

#[test]
fn source_layouts_resolve_unique_module_paths_without_choosing_between_roots() {
    for prefix in ["src", "backend"] {
        // Arrange: analyse from above the Python import root, as in src layouts and monorepos.
        let models = format!("{prefix}/pkg/models.py");
        let service = format!("{prefix}/pkg/service.py");
        let mut files = vec![
            (models.as_str(), "class User: pass\n"),
            (
                service.as_str(),
                "from pkg.models import User\nclass Service:\n    user: User\n",
            ),
        ];

        // Act / Assert: a unique dotted module suffix identifies the full module path.
        let output = export(&analyze(&files));
        assert_edge(
            &output,
            (&service, "Service"),
            (&models, "User"),
            "-->",
            "field user",
        );

        // A competing module/package at the inferred root must prevent guessing.
        let conflicting = format!("{prefix}/pkg/models/__init__.py");
        files.push((&conflicting, "class User: pass\n"));
        let output = export(&analyze(&files));
        assert!(!output.contains("`field user`"), "{output}");
    }
}

#[test]
fn unresolved_bindings_explain_missing_arrows_without_claiming_name_ambiguity() {
    for import in [
        "from external import EntityType",
        "from external import EntityType as Alias",
        "if flag:\n    from one import EntityType",
    ] {
        // Arrange: local lookalikes cannot satisfy an explicit external/uncertain binding.
        let name = if import.ends_with("as Alias") {
            "Alias"
        } else {
            "EntityType"
        };
        let source = format!("{import}\nclass Service({name}):\n    kind: {name}\n");
        let result = analyze(&[
            ("one.py", "class EntityType: pass\n"),
            ("two.py", "class EntityType: pass\n"),
            ("service.py", &source),
        ]);

        // Act.
        let output = export(&result);

        // Assert: distinguish a blocked binding from heuristic name ambiguity.
        assert!(
            output.contains(&format!("Unresolved Python binding: Service uses {name}")),
            "{output}"
        );
        assert!(
            output.contains(&format!("Unresolved Python binding: Service base {name}")),
            "{output}"
        );
        assert!(!output.contains("Ambiguous type:"), "{output}");
    }
}

#[test]
fn unverified_or_conflicting_typing_guards_remain_unresolved() {
    for guard in [
        "if TYPE_CHECKING:\n    from events import EventBus\n",
        "from custom import TYPE_CHECKING\nif TYPE_CHECKING:\n    from events import EventBus\n",
        concat!(
            "from typing import TYPE_CHECKING\nTYPE_CHECKING = flag\n",
            "if TYPE_CHECKING:\n    from events import EventBus\n",
        ),
        concat!(
            "from typing import TYPE_CHECKING\ndef shadow(TYPE_CHECKING): pass\n",
            "if TYPE_CHECKING:\n    from events import EventBus\n",
        ),
        concat!(
            "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n",
            "    from events import EventBus\nelse:\n    from other import EventBus\n",
        ),
        concat!(
            "from typing import TYPE_CHECKING\nif TYPE_CHECKING:\n",
            "    if flag:\n        from events import EventBus\n",
        ),
        concat!(
            "import typing\ntyping.TYPE_CHECKING = flag\n",
            "if typing.TYPE_CHECKING:\n    from events import EventBus\n",
        ),
    ] {
        // Arrange: the spelling alone does not establish a supported typing guard.
        let source = format!("{guard}class Service:\n    bus: 'EventBus'\n");
        let result = analyze(&[
            ("events.py", "class EventBus: pass\n"),
            ("service.py", &source),
        ]);

        // Act / Assert.
        let output = export(&result);
        assert!(!output.contains("`field bus`"), "{guard}\n{output}");
        assert!(
            output.contains("Unresolved Python binding: Service uses EventBus"),
            "{output}"
        );
    }
}

#[test]
fn cyclic_reexports_do_not_choose_an_unrelated_declaration() {
    // Arrange: each module re-exports the other; a same-directory class is a decoy.
    let result = analyze(&[
        ("one.py", "from two import User\n"),
        ("two.py", "from one import User\n"),
        ("decoy.py", "class User: pass\n"),
        (
            "service.py",
            "from one import User\nclass Service:\n    user: User\n",
        ),
    ]);

    // Act / Assert: termination and diagnostics are both observable through export.
    let output = export(&result);
    assert!(!output.contains("`field user`"), "{output}");
    assert!(
        output.contains("Unresolved Python binding: Service uses User"),
        "{output}"
    );
}

#[test]
fn module_suffixes_do_not_turn_external_imports_into_local_wrappers() {
    for (consumer, package) in [
        ("app/models.py", true),
        ("app/models.py", false),
        ("app/compat/models.py", true),
    ] {
        // Arrange: a local compatibility module is not the import root of the external name.
        let mut files = vec![
            ("app/compat/enum.py", "class StrEnum: pass\n"),
            (
                consumer,
                "from enum import StrEnum\nclass Status(StrEnum):\n    kind: StrEnum\n",
            ),
        ];
        if package {
            files.extend([("app/__init__.py", ""), ("app/compat/__init__.py", "")]);
        }

        // Act / Assert.
        let output = export(&analyze(&files));
        assert!(!output.contains(": `inherits`"), "{output}");
        assert!(!output.contains("`field kind`"), "{output}");
    }
}

#[test]
fn a_package_can_reexport_its_own_submodule() {
    // Arrange: a relative package import loads the child; it is not a re-export cycle.
    let result = analyze(&[
        ("pkg/__init__.py", "from . import models\n"),
        ("pkg/models.py", "class Entity: pass\n"),
        (
            "service.py",
            "from pkg import models\nclass Service:\n    entity: models.Entity\n",
        ),
    ]);

    // Act.
    let output = export(&result);

    // Assert.
    assert_edge(
        &output,
        ("service.py", "Service"),
        ("pkg/models.py", "Entity"),
        "-->",
        "field entity",
    );
}
