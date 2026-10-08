use mycelium_core::config::{AnalysisConfig, AnalysisResult};
use mycelium_core::mermaid::{
    export_mermaid, export_mermaid_report, DetailMode, ExportError, MermaidOptions, TestMode,
};
use mycelium_core::pipeline::run_pipeline;

fn csharp_map() -> AnalysisResult {
    analyze(
        "Model.cs",
        include_str!("../../../tests/fixtures/compact_csharp/Model.cs"),
    )
}

fn analyze(file: &str, source: &str) -> AnalysisResult {
    analyze_files(&[(file, source)])
}

fn analyze_files(files: &[(&str, &str)]) -> AnalysisResult {
    let repo = tempfile::tempdir().unwrap();
    for (file, source) in files {
        std::fs::write(repo.path().join(file), source).unwrap();
    }
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    // Exercise saved facts; the source directory is gone before callers export.
    serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap()
}

#[test]
fn compact_python_initializer_calls_do_not_construct_the_base() {
    // Arrange: calling an initializer on an existing object differs from creating Thing.
    let result = analyze_files(&[
        (
            "base.py",
            r#"class Base:
    def __init__(self):
        self.ready = True
class Thing:
    value: int
"#,
        ),
        (
            "child.py",
            r#"from base import Base, Thing
class Other(Base):
    def __init__(self):
        Base.__init__(self)
    def make(self):
        return Thing()
"#,
        ),
    ]);
    let initializer = result
        .symbols
        .iter()
        .find(|s| s.file == "base.py" && s.symbol_type == "Constructor")
        .unwrap();
    assert!(result.calls.iter().any(|c| c.to == initializer.id));

    // Act: helpers round-trip JSON and remove source files before export.
    let compact = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    let full = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            ..Default::default()
        },
    )
    .unwrap();

    // Assert: only Thing() creates an object; the base initializer remains a call.
    assert!(full.contains("__init__() calls __init__()"), "{full}");
    assert!(full.contains("make() constructs Thing"), "{full}");
    assert!(compact.contains("c0002 ..> c0000 : calls\n"), "{compact}");
    assert!(
        !compact.contains("c0002 ..> c0000 : constructs\n"),
        "{compact}"
    );
    assert!(
        compact.contains("c0002 ..> c0001 : constructs\n"),
        "{compact}"
    );
}

#[test]
fn compact_groups_explicit_and_implicit_constructors_as_construction() {
    for file in ["Factory.cs", "Factory.java"] {
        // Arrange: real constructor calls, exported from saved facts after source removal.
        let result = analyze(
            file,
            r#"
class Explicit { public Explicit(int value) {} }
class Plain {}
class Factory {
    public void Make() { new Explicit(1); new Plain(); }
}
"#,
        );
        let constructor = result
            .symbols
            .iter()
            .find(|s| s.name == "Explicit" && s.symbol_type == "Constructor")
            .unwrap();
        assert!(result.calls.iter().any(|c| c.to == constructor.id));

        // Act.
        let compact = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        let full = export_mermaid(
            &result,
            &MermaidOptions {
                detail: DetailMode::Full,
                ..Default::default()
            },
        )
        .unwrap();

        // Assert: full retains its historical member label; compact describes construction.
        assert!(
            full.contains("c0001 ..> c0000 : Make() calls Explicit()"),
            "{full}"
        );
        assert!(
            full.contains("c0001 ..> c0002 : Make() constructs Plain"),
            "{full}"
        );
        assert!(
            compact.contains("c0001 ..> c0000 : constructs\n"),
            "{file}: {compact}"
        );
        assert!(
            compact.contains("c0001 ..> c0002 : constructs\n"),
            "{file}: {compact}"
        );
        assert!(!compact.contains(" : calls\n"), "{file}: {compact}");
    }
}

#[test]
fn compact_vb_new_expressions_remain_construction() {
    // Arrange: the current VB grammar resolves New expressions to declaration endpoints.
    let result = analyze(
        "Factory.vb",
        r#"Public Class Explicit
    Public Sub New(value As Integer)
    End Sub
End Class
Public Class Plain
End Class
Public Module Factory
    Public Function Make() As Explicit
        Return New Explicit(1)
    End Function
    Public Function MakePlain() As Plain
        Return New Plain()
    End Function
End Module
"#,
    );

    // Act.
    let compact = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: both explicit and implicit constructor declarations still create objects.
    assert!(result
        .symbols
        .iter()
        .any(|s| s.symbol_type == "Constructor"));
    assert!(
        compact.contains("c0001 ..> c0000 : constructs\n"),
        "{compact}"
    );
    assert!(
        compact.contains("c0001 ..> c0002 : constructs\n"),
        "{compact}"
    );
    assert!(!compact.contains(" : calls\n"), "{compact}");
}

#[test]
fn compact_keeps_cpp_overloads_after_existing_prototype_deduplication() {
    // Arrange: two overloads, each declared and defined, plus a file module function.
    let result = analyze(
        "service.cpp",
        r#"
struct Service { int run(int n); int run(double n); };
int Service::run(int n) { return n; }
int Service::run(double n) { return 1; }
int launch() { return 0; }
"#,
    );

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: prototype/definition pairs still produce one row per overload.
    assert_eq!(
        members(&output, "Service")
            .matches("        run()\n")
            .count(),
        2
    );
    assert!(members(&output, "service.cpp").contains("<<module>>"));
    assert!(members(&output, "service.cpp").contains("        launch()\n"));
}

#[test]
fn compact_keeps_traits_modules_and_calls_within_one_box() {
    // Arrange: trait members merge into their owner; two free functions share a module edge.
    let result = analyze(
        "lib.rs",
        r#"
pub trait Store { fn save(&self); }
pub struct Cache;
impl Store for Cache { fn save(&self) {} }
pub fn start(cache: Cache) { stop(); }
fn stop() {}
"#,
    );

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: keep every kind of box/member and the self-call in the diagram.
    assert!(members(&output, "Store").contains("<<trait>>"));
    assert!(members(&output, "Store").contains("        +save()\n"));
    assert!(members(&output, "Cache").contains("        +save()\n"));
    assert!(members(&output, "lib.rs").contains("        +start()\n"));
    assert!(output.contains(" ..|> "));
    assert!(output.contains(" : uses type\n"));
    assert!(output.contains("    c0002 ..> c0002 : calls\n"), "{output}");
    assert!(!output.contains("## Cross-diagram relationships"));
}

#[test]
fn compact_preserves_unsafe_names_without_signature_aliases() {
    // Arrange: saved names are source-controlled, even when they contain Mermaid syntax.
    let mut result = csharp_map();
    let worker = result
        .class_diagram
        .as_mut()
        .unwrap()
        .classes
        .iter_mut()
        .find(|c| c.name == "Worker")
        .unwrap();
    worker.members[0].name = "field(callback)".into();
    worker.members[2].name = "method\"}\nclick exploit".into();

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: source punctuation remains visible as escaped text, never executable statements.
    assert!(output.contains("        -field#40;callback#41;\n"));
    assert!(output.contains("        +method#34;#125;#10;click exploit()\n"));
    assert!(!output.contains("\nclick exploit"));
    assert!(!output.contains("Signature key"));
    assert!(!output.contains("Type key"));
}

#[test]
fn compact_keeps_filtering_warnings_notices_and_empty_selection_behaviour() {
    // Arrange: a saved C# map with test evidence, extraction and resolution warnings.
    let mut result = csharp_map();
    let diagram = result.class_diagram.as_mut().unwrap();
    diagram.warnings.push("Saved parse warning".into());
    let worker = diagram
        .classes
        .iter_mut()
        .find(|c| c.name == "Worker")
        .unwrap();
    worker.bases.push(mycelium_core::declarations::Base {
        name: "UnknownBase".into(),
        relation: "inherits".into(),
    });
    let method = worker.members.iter_mut().find(|m| m.name == "Run").unwrap();
    method.test = Some(mycelium_core::declarations::TestEvidence {
        rule: "dotnet.xunit-method".into(),
        file: method.file.clone(),
        line: method.line,
    });
    let mut other = result.structure.files[0].clone();
    other.path = "elsewhere.cs".into();
    result.structure.files.push(other);
    let options = MermaidOptions {
        path: "Model.cs".into(),
        test_paths: vec!["elsewhere.cs".into()],
        explain_tests: true,
        ..Default::default()
    };

    // Act.
    let compact = export_mermaid_report(&result, &options).unwrap();
    let full = export_mermaid_report(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            ..options.clone()
        },
    )
    .unwrap();

    // Assert: display mode cannot change selection, diagnostics or stable source identities.
    assert_eq!(compact.notices, full.notices);
    assert!(compact
        .notices
        .iter()
        .any(|n| n.contains("outside the selected scope")));
    for section in ["Test filtering", "Source index", "Extraction warnings"] {
        let section_text = |s: &str| {
            s.split_once(&format!("## {section}\n"))
                .unwrap()
                .1
                .split("\n## ")
                .next()
                .unwrap()
                .trim()
                .to_string()
        };
        assert_eq!(
            section_text(&compact.markdown),
            section_text(&full.markdown)
        );
    }
    assert!(compact.markdown.contains("UnknownBase"));
    assert!(compact.markdown.contains("Saved parse warning"));
    assert_eq!(
        members(&compact.markdown, "Worker")
            .matches("+Run()\n")
            .count(),
        1
    );
    for (tests, keep_paths, expected) in [
        (TestMode::Exclude, vec![], 0),
        (TestMode::Exclude, vec!["Model.cs".into()], 2),
        (TestMode::Include, vec![], 2),
    ] {
        let output = export_mermaid(
            &result,
            &MermaidOptions {
                tests,
                keep_paths,
                test_paths: vec!["Model.cs".into()],
                ..Default::default()
            },
        )
        .unwrap();
        if expected == 0 {
            assert!(output.contains("No declarations remain after test filtering"));
            assert!(!output.contains("classDiagram"));
            assert!(!output.contains("## Cross-diagram relationships"));
        } else {
            assert_eq!(
                members(&output, "Worker").matches("+Run()\n").count(),
                expected
            );
        }
    }
    assert!(matches!(
        export_mermaid(
            &result,
            &MermaidOptions {
                path: "missing".into(),
                ..Default::default()
            }
        ),
        Err(ExportError::NoMatchingPath)
    ));
    assert!(matches!(
        export_mermaid(
            &result,
            &MermaidOptions {
                max_classes: 0,
                ..Default::default()
            }
        ),
        Err(ExportError::InvalidOptions)
    ));
}

fn members<'a>(markdown: &'a str, name: &str) -> &'a str {
    markdown
        .split_once(&format!("[\"{name}\"] {{\n"))
        .unwrap()
        .1
        .split_once("    }")
        .unwrap()
        .0
}

#[test]
fn compact_default_keeps_csharp_members_without_signatures_or_keys() {
    // Arrange.
    let result = csharp_map();
    let before = serde_json::to_value(&result).unwrap();

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: overloads, constructors, properties and visibility survive unchanged in number.
    let worker = members(&output, "Worker");
    assert!(worker.contains("        -_token\n"), "{output}");
    assert!(worker.contains("        +Current\n"));
    assert!(worker.contains("        +Worker()\n"));
    assert_eq!(worker.matches("        +Run()\n").count(), 2);
    assert!(worker.contains("        +Convert()\n"));
    assert!(members(&output, "IRunner").contains("<<interface>>"));
    assert!(members(&output, "State").contains("        Ready\n"));
    assert!(members(&output, "Position").contains("        +X\n"));
    assert!(!output.contains("Type key"));
    assert!(!output.contains("Signature key"));
    assert_eq!(before, serde_json::to_value(&result).unwrap());
}

#[test]
fn full_preserves_historical_csharp_bytes_and_detail_rejects_invalid_values() {
    // Arrange: the literal was exported before compact rendering was implemented.
    let result = csharp_map();
    let options = MermaidOptions {
        detail: "full".parse().unwrap(),
        ..Default::default()
    };

    // Act / Assert.
    assert_eq!(
        export_mermaid(&result, &options).unwrap(),
        include_str!("../../../tests/fixtures/compact_csharp.full.md")
    );
    assert_eq!(
        "compact".parse::<DetailMode>().unwrap(),
        DetailMode::Compact
    );
    assert!(matches!(
        "summary".parse::<DetailMode>(),
        Err(ExportError::InvalidDetailMode)
    ));
}

#[test]
fn compact_groups_by_meaning_and_lists_only_cross_diagram_connections() {
    // Arrange: Worker -> Token has calls, construction, fields and signature dependencies.
    let mut result = csharp_map();
    result
        .class_diagram
        .as_mut()
        .unwrap()
        .warnings
        .push("Saved extraction warning".into());
    let full = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            ..Default::default()
        },
    )
    .unwrap();

    // Act: four boxes puts Worker/Token together but separates their other neighbours.
    let output = export_mermaid(
        &result,
        &MermaidOptions {
            max_classes: 4,
            ..Default::default()
        },
    )
    .unwrap();

    // Assert: semantic groups survive without method labels or signature/type appendices.
    for meaning in ["calls", "constructs", "uses type"] {
        assert!(
            output.contains(&format!("    c0006 ..> c0005 : {meaning}\n")),
            "{output}"
        );
    }
    assert!(output.contains("    c0006 --> c0005 : field\n"));
    let cross = output
        .split_once("## Cross-diagram relationships\n\n")
        .unwrap()
        .1;
    assert!(cross.contains("- c0006 ..> c0002: `uses type`"));
    assert!(cross.contains("- c0006 --|> c0000: `inherits`"));
    assert!(cross.contains("- c0006 ..|> c0001: `implements`"));
    assert!(cross.contains("- c0001 ..> c0005: `uses type`"));
    assert!(!cross.contains("- c0006 ..> c0005:"));
    assert!(!cross.contains("- c0006 --> c0005:"));
    for absent in [
        "## Relationships",
        "Type key",
        "Signature key",
        "see list",
        "calls Touch",
    ] {
        assert!(!output.contains(absent), "{absent}: {output}");
    }
    let source_index = |s: &str| {
        s.split("## Source index\n")
            .nth(1)
            .unwrap()
            .split("\n## ")
            .next()
            .unwrap()
            .trim()
            .to_string()
    };
    assert_eq!(source_index(&output), source_index(&full));
    assert_eq!(
        output.split_once("## Extraction warnings").unwrap().1,
        full.split_once("## Extraction warnings").unwrap().1,
    );
    assert!(!export_mermaid(&result, &MermaidOptions::default())
        .unwrap()
        .contains("## Cross-diagram relationships"));
    result.class_diagram.as_mut().unwrap().classes.reverse();
    result.calls.reverse();
    assert_eq!(
        output,
        export_mermaid(
            &result,
            &MermaidOptions {
                max_classes: 4,
                ..Default::default()
            }
        )
        .unwrap()
    );
}
