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
    // Export must work after removing the source checkout and round-tripping saved JSON.
    serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap()
}

#[test]
fn explicit_paths_filter_saved_occurrences_and_keep_overrides_win() {
    // Arrange: a test impl extends an application type; a similarly named path is application code.
    let result = analyze(&[
        (
            "src/lib.rs",
            "pub struct Service {}\nimpl Service { pub fn run(&self) {} }",
        ),
        (
            "tests/check.rs",
            "impl Service { fn check(&self) {} }\nstruct Fake {}",
        ),
        ("tests/shared.rs", "struct Shared {}"),
        ("testsupport/util.rs", "struct Useful {}"),
    ]);
    let before = serde_json::to_string(&result).unwrap();
    let options = MermaidOptions {
        detail: DetailMode::Full,
        test_paths: vec!["./tests/".into()],
        keep_paths: vec!["tests/shared.rs".into()],
        ..Default::default()
    };

    // Act.
    let output = export_mermaid(&result, &options).unwrap();

    // Assert: neither the test member nor its source/relationship metadata leaks into the view.
    for kept in ["Service", "run()", "Shared", "Useful"] {
        assert!(output.contains(kept), "Missing {kept}: {output}");
    }
    for hidden in ["Fake", "check()", "Unresolved implementation"] {
        assert!(!output.contains(hidden), "Unexpected {hidden}: {output}");
    }
    assert_eq!(before, serde_json::to_string(&result).unwrap());
}

#[test]
fn rules_validate_files_normalize_order_and_preserve_include_output() {
    // Arrange: empty source files are valid selectors, even outside the diagram's path.
    let result = analyze(&[
        ("src/app.py", "class App: pass"),
        ("tests/check.py", "class Check: pass"),
        ("tests/empty.py", "# no declarations"),
    ]);
    let options = MermaidOptions {
        detail: DetailMode::Full,
        path: "src".into(),
        test_paths: vec!["./tests//".into(), "tests/empty.py".into(), "tests".into()],
        ..Default::default()
    };
    // Act.
    let report = mycelium_core::mermaid::export_mermaid_report(&result, &options).unwrap();
    let normalized = MermaidOptions {
        detail: DetailMode::Full,
        test_paths: vec!["tests/empty.py".into(), "tests".into()],
        ..options.clone()
    };
    // Assert.
    assert_eq!(
        report.markdown,
        export_mermaid(&result, &normalized).unwrap()
    );
    assert!(report.notices.iter().any(|n| n.contains("outside")));
    for path in ["missing", "../tests", "/tests", "C:\\tests", "Tests"] {
        let bad = MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec![path.into()],
            ..options.clone()
        };
        assert!(export_mermaid(&result, &bad).is_err(), "Accepted {path}");
    }
    let include = MermaidOptions {
        detail: DetailMode::Full,
        tests: mycelium_core::mermaid::TestMode::Include,
        ..Default::default()
    };
    let with_rules = MermaidOptions {
        detail: DetailMode::Full,
        test_paths: vec!["tests".into()],
        keep_paths: vec!["src".into()],
        explain_tests: true,
        ..include.clone()
    };
    assert_eq!(
        export_mermaid(&result, &include).unwrap(),
        export_mermaid(&result, &with_rules).unwrap()
    );
}

#[test]
fn ids_stay_stable_hidden_endpoints_disappear_and_empty_scope_is_valid() {
    // Arrange: the first type sorts before the application type.
    let result = analyze(&[
        ("a_test.py", "class Fake: pass"),
        ("z_app.py", "class App:\n    fake: Fake\n"),
    ]);
    let include = MermaidOptions {
        detail: DetailMode::Full,
        tests: mycelium_core::mermaid::TestMode::Include,
        ..Default::default()
    };
    // Act.
    let full = export_mermaid(&result, &include).unwrap();
    let filtered = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec!["a_test.py".into()],
            ..Default::default()
        },
    )
    .unwrap();
    let empty = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec![".".into()],
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    assert!(full.contains("class c0001[\"App\"]"));
    assert!(filtered.contains("class c0001[\"App\"]"));
    assert!(!filtered.contains(" --> "));
    assert!(!empty.contains("classDiagram"));
    assert!(empty.contains("No declarations remain"));
}

#[test]
fn rust_saved_evidence_removes_inline_tests_and_test_impls_only() {
    // Arrange: mixed code on the same line must not share a role by line number.
    let result = analyze(&[(
        "lib.rs",
        r#"
pub trait DebugOnly { fn inspect(&self); }
pub struct Service { pub normal: u32, #[cfg(test)] scratch: String }
impl Service { pub fn run(&self) {} }
#[cfg(test)] impl DebugOnly for Service { fn inspect(&self) {} }
#[test] fn check() { application(); } fn application() {}
#[cfg(test)] mod checks {
    struct Fake { value: String }
    fn helper() { application(); }
    #[tokio::test] async fn check_async() {}
}
#[cfg(any(test, feature = "tools"))] fn either() {}
#[cfg(not(test))] fn normal() {}
macro_rules! test { () => {} }
#[test] fn builtin_despite_bang_macro() {}
"#,
    )]);
    let before = serde_json::to_string(&result).unwrap();
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    let include = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            tests: mycelium_core::mermaid::TestMode::Include,
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    for kept in [
        "Service",
        "run()",
        "application()",
        "either()",
        "normal()",
        "DebugOnly",
    ] {
        assert!(output.contains(kept), "Missing {kept}: {output}");
    }
    for hidden in [
        "Fake",
        "scratch:",
        "check()",
        "helper()",
        "check_async()",
        "builtin_despite_bang_macro()",
        "..|>",
    ] {
        assert!(!output.contains(hidden), "Unexpected {hidden}: {output}");
        assert!(include.contains(hidden), "Missing from include: {hidden}");
    }
    assert!(output.contains("rust.cfg-test"));
    assert!(output.contains("rust.test"));
    assert!(
        output.contains("Calls removed by test filtering: 2."),
        "{output}"
    );
    assert_eq!(before, serde_json::to_string(&result).unwrap());
}

#[test]
fn go_test_files_remove_helpers_and_receiver_methods_without_name_guessing() {
    // Arrange: test-file receiver methods belong to a production type.
    let result = analyze(&[
        (
            "app.go",
            "package app\ntype Service struct {}\nfunc (s Service) Run() {}\n",
        ),
        (
            "app_test.go",
            "package app\ntype Fake struct {}\nfunc (s Service) Check() {}\n",
        ),
        (
            "testdata/app.go",
            "package app\ntype TestConnection struct {}\n",
        ),
        (
            "_ignored_test.go",
            "package app\ntype IgnoredByGo struct {}\n",
        ),
    ]);
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    let kept = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            keep_paths: vec!["app_test.go".into()],
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    for name in ["Service", "Run()", "TestConnection", "IgnoredByGo"] {
        assert!(output.contains(name), "Missing {name}: {output}");
    }
    assert!(!output.contains("Fake"));
    assert!(!output.contains("Check()"));
    assert!(output.contains("go.test-file"));
    assert!(kept.contains("Fake"));
    assert!(kept.contains("Check()"));
    assert!(kept.contains("keep-path override"));
}

#[test]
fn rust_inner_attributes_work_but_recovery_and_imported_attributes_stay_visible() {
    // Arrange: an incomplete module must not swallow apparent application declarations.
    let result = analyze(&[
        (
            "inner.rs",
            "#![cfg(test)]\nstruct CrateHelper {}\nfn helper() {}",
        ),
        (
            "module.rs",
            "mod checks { #![cfg(test)] struct InnerHelper {} }\nstruct App {}",
        ),
        ("broken.rs", "#[cfg(test)] mod checks { struct MaybeApp {}"),
        ("shadow.rs", "use custom::test;\n#[test] fn retained() {}"),
        ("glob.rs", "use custom::*;\n#[test] fn uncertain() {}"),
        ("external.rs", "#[cfg(test)] mod external;\n"),
        ("external/child.rs", "struct ExternalKept {}"),
    ]);
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    // Assert.
    for name in [
        "App",
        "MaybeApp",
        "retained()",
        "uncertain()",
        "ExternalKept",
    ] {
        assert!(output.contains(name), "Missing {name}: {output}");
    }
    for name in ["CrateHelper", "InnerHelper", "helper()"] {
        assert!(!output.contains(name), "Unexpected {name}: {output}");
    }
    assert!(
        output.contains("syntax-derived test detection disabled"),
        "{output}"
    );
    assert!(
        output.contains("test attribute binding is uncertain"),
        "{output}"
    );
}

#[test]
fn explicit_paths_cover_all_ten_languages_without_inferring_test_names() {
    // Arrange: language-neutral path selection must not require framework recognition.
    let cases = [
        ("rs", "struct TestApplication {}", "struct Hidden {}"),
        (
            "go",
            "package app\ntype TestApplication struct {}",
            "package app\ntype Hidden struct {}",
        ),
        ("py", "class TestApplication: pass", "class Hidden: pass"),
        ("cs", "class TestApplication {}", "class Hidden {}"),
        (
            "vb",
            "Public Class TestApplication\nEnd Class",
            "Public Class Hidden\nEnd Class",
        ),
        ("java", "class TestApplication {}", "class Hidden {}"),
        ("ts", "class TestApplication {}", "class Hidden {}"),
        ("js", "class TestApplication {}", "class Hidden {}"),
        (
            "c",
            "struct TestApplication { int x; };",
            "struct Hidden { int x; };",
        ),
        ("cpp", "class TestApplication {};", "class Hidden {};"),
    ];
    for (extension, application, test) in cases {
        let result = analyze(&[
            (&format!("src/app.{extension}"), application),
            (&format!("tests/check.{extension}"), test),
        ]);
        // Act.
        let output = export_mermaid(
            &result,
            &MermaidOptions {
                detail: DetailMode::Full,
                test_paths: vec!["tests".into()],
                ..Default::default()
            },
        )
        .unwrap();
        // Assert.
        assert!(output.contains("TestApplication"), "{extension}: {output}");
        assert!(!output.contains("Hidden"), "{extension}: {output}");
    }
}

#[test]
fn legacy_and_unknown_detector_maps_use_explicit_paths_without_guessing() {
    // Arrange: a saved map from before detection, or a newer unsupported detector.
    let result = analyze(&[("app.rs", "struct App {}\n#[test] fn check() {}")]);
    for version in [None, Some(999)] {
        let mut saved = serde_json::to_value(&result).unwrap();
        if let Some(version) = version {
            saved["class_diagram"]["test_detection"]["version"] = version.into();
        } else {
            saved["class_diagram"]
                .as_object_mut()
                .unwrap()
                .remove("test_detection");
        }
        let legacy: AnalysisResult = serde_json::from_value(saved).unwrap();
        // Act.
        let output = export_mermaid(&legacy, &full_options()).unwrap();
        let excluded = export_mermaid(
            &legacy,
            &MermaidOptions {
                detail: DetailMode::Full,
                test_paths: vec!["app.rs".into()],
                ..Default::default()
            },
        )
        .unwrap();
        // Assert.
        assert!(output.contains("check()"));
        assert!(output.contains("Automatic test detection unavailable"));
        assert!(!excluded.contains("[\"App\"]"));
    }
}

#[test]
fn filtering_preserves_ambiguity_and_removes_edges_to_hidden_members() {
    // Arrange: filtering cannot turn two equally named types into a unique target.
    let result = analyze(&[
        ("a.rs", "pub struct Item {}\n"),
        ("b.rs", "#[cfg(test)] pub struct Item {}\n"),
        (
            "c.rs",
            "struct Service { value: Item }\nfn run() { check(); }\n#[test] fn check() {}",
        ),
    ]);
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    // Assert.
    assert!(output.contains("Ambiguous type: Service uses Item"));
    assert!(!output.contains(" --> "));
    assert!(!output.contains("check()"));
    assert!(output.contains("run()"));
    assert!(output.contains("Calls removed by test filtering: 1."));
}

#[test]
fn test_relationships_are_removed_by_origin_before_merging() {
    // Arrange: one test-only trait impl, one trait implemented in both roles.
    let result = analyze(&[(
        "lib.rs",
        r#"
trait OnlyTests { fn check(&self); }
trait Shared { fn shared(&self); }
struct Service {}
impl Shared for Service { fn shared(&self) {} }
#[cfg(test)] impl Shared for Service { fn shared(&self) {} }
#[cfg(test)] impl OnlyTests for Service { fn check(&self) {} }
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    // Assert: the retained Shared relationship must not inherit the removed duplicate's role.
    assert!(output.contains("c0001 ..|> c0002"), "{output}");
    assert!(!output.contains("c0001 ..|> c0000"), "{output}");
    assert!(
        output.contains("Type relationships removed: 1."),
        "{output}"
    );
}

#[test]
fn missing_occurrence_locations_stay_visible_with_a_diagnostic() {
    // Arrange: legacy maps may lack a member's independent origin; never assume its owner's file.
    let result = analyze(&[(
        "tests/mixed.py",
        "class App:\n    def retained(self): pass\n",
    )]);
    let mut saved = serde_json::to_value(&result).unwrap();
    saved["class_diagram"]["classes"][0]["members"][0]
        .as_object_mut()
        .unwrap()
        .remove("file");
    let saved: AnalysisResult = serde_json::from_value(saved).unwrap();
    // Act.
    let output = export_mermaid(
        &saved,
        &MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec!["tests".into()],
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    assert!(output.contains("retained()"));
    assert!(output.contains("invalid source location; retained"));
}

#[test]
fn explanations_and_saved_order_are_deterministic() {
    // Arrange: repeated/reordered selectors, mixed source, and independent saved record ordering.
    let mut result = analyze(&[
        (
            "src/app.rs",
            "struct App {}\n#[test] fn check() {}\nfn run() {}",
        ),
        ("tests/helper.rs", "struct Helper {}"),
    ]);
    let options = MermaidOptions {
        detail: DetailMode::Full,
        test_paths: vec!["tests".into(), "src".into()],
        keep_paths: vec!["./src/".into()],
        explain_tests: true,
        ..Default::default()
    };
    // Act.
    let output = export_mermaid(&result, &options).unwrap();
    let diagram = result.class_diagram.as_mut().unwrap();
    diagram.classes.reverse();
    for class in &mut diagram.classes {
        class.members.reverse();
        class.bases.reverse();
    }
    result.structure.files.reverse();
    result.calls.reverse();
    result.symbols.reverse();
    let repeated = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec!["./src/".into(), "tests".into(), "src".into()],
            ..options
        },
    )
    .unwrap();
    // Assert.
    assert_eq!(output, repeated);
    assert!(output.contains("<details>"));
    assert!(output.contains("keep-path override"));
    assert!(output.contains("src/app.rs:2 check"), "{output}");
}

#[test]
fn rust_tuple_fields_and_attribute_comments_keep_exact_source_scopes() {
    // Arrange: a public tuple field's visibility is between its attribute and type node.
    let result = analyze(&[(
        "lib.rs",
        r#"
struct Item {}
struct Pair(#[cfg(/* only tests */ test)] pub Item, pub u32);
#[cfg(test)] /* attached comment */ fn check() {}
fn retained() {}
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    // Assert.
    assert!(!output.contains("+0: Item"), "{output}");
    assert!(output.contains("+1: u32"));
    assert!(!output.contains("check()"));
    assert!(output.contains("retained()"));
}

#[test]
fn include_mode_preserves_empty_maps_without_filter_messages() {
    // Arrange.
    let result = analyze(&[("empty.py", "# no declarations")]);
    // Act.
    let output = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            tests: mycelium_core::mermaid::TestMode::Include,
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    assert!(!output.contains("after test filtering"));
    assert!(!output.contains("Test filtering"));
    let default = export_mermaid(&result, &full_options()).unwrap();
    assert!(!default.contains("No declarations remain after test filtering"));
}

#[test]
fn same_line_types_in_distinct_rust_modules_keep_distinct_box_ids() {
    // Arrange: raw legacy IDs lack a column or module qualifier and can collide.
    let result = analyze(&[(
        "lib.rs",
        concat!(
            "mod app { struct Item { normal: u32 } } ",
            "#[cfg(test)] mod checks { struct Item { scratch: u32 } }"
        ),
    )]);
    // Act.
    let full = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            tests: mycelium_core::mermaid::TestMode::Include,
            ..Default::default()
        },
    )
    .unwrap();
    let filtered = export_mermaid(&result, &full_options()).unwrap();
    // Assert.
    assert!(full.contains("class c0000[\"Item\"]"), "{full}");
    assert!(full.contains("class c0001[\"Item\"]"), "{full}");
    assert!(filtered.contains("class c0000[\"Item\"]"), "{filtered}");
    assert!(!filtered.contains("scratch:"));
}

#[test]
fn rust_only_reports_bindings_that_actually_prevent_detection_within_scope() {
    // Arrange: wildcard imports inside established test scopes do not create uncertainty.
    let result = analyze(&[
        (
            "src/lib.rs",
            concat!(
                "fn run() {}\n#[cfg(test)] mod checks { ",
                "use super::*; #[test] fn check() {} }"
            ),
        ),
        ("other/lib.rs", "use custom::*;\n#[test] fn uncertain() {}"),
    ]);
    // Act.
    let scoped = mycelium_core::mermaid::export_mermaid_report(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            path: "src".into(),
            ..Default::default()
        },
    )
    .unwrap();
    let full = mycelium_core::mermaid::export_mermaid_report(&result, &full_options()).unwrap();
    // Assert.
    assert!(scoped.notices.is_empty(), "{:?}", scoped.notices);
    assert!(!scoped.markdown.contains("uncertain"));
    assert_eq!(full.notices.len(), 1, "{:?}", full.notices);
    assert!(full.notices[0].starts_with("other/lib.rs:"));
    let metadata = serde_json::to_value(&result).unwrap();
    assert_eq!(
        metadata["class_diagram"]["test_detection"]["diagnostics"][0]["file"],
        "other/lib.rs"
    );
}

#[test]
fn retained_impl_keeps_an_owner_shell_and_cross_file_filter_counts_follow_the_owner_scope() {
    // Arrange: a kept impl must not silently disappear with its owner's excluded declaration.
    let result = analyze(&[
        ("tests/fake.rs", "struct Fake { scratch: u32 }"),
        ("tests/support.rs", "impl Fake { fn build() {} }"),
        ("src/lib.rs", "struct Service {}"),
        ("tests/service.rs", "impl Service { fn check() {} }"),
    ]);
    // Act.
    let kept = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            test_paths: vec!["tests".into()],
            keep_paths: vec!["tests/support.rs".into()],
            ..Default::default()
        },
    )
    .unwrap();
    let scoped = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            path: "src".into(),
            test_paths: vec!["tests".into()],
            ..Default::default()
        },
    )
    .unwrap();
    // Assert.
    assert!(kept.contains("build()"), "{kept}");
    assert!(!kept.contains("scratch:"));
    assert!(kept.contains("Owner retained as context"));
    assert!(!scoped.contains("check()"));
    assert!(
        scoped.contains("`tests/service.rs`: 2 source occurrences"),
        "{scoped}"
    );
}

#[test]
fn rust_variants_filter_and_cpp_prototypes_do_not_inflate_removed_relationships() {
    // Arrange: Rust handles variants directly; shared enum_member is for other languages.
    let rust = analyze(&[("lib.rs", "enum Mode { Normal, #[cfg(test)] Scratch }")]);
    let cpp = analyze(&[
        (
            "model.hpp",
            "struct Item {}; class App { public: Item* run(Item* a); };",
        ),
        ("model.cpp", "Item* App::run(Item* b) { return b; }"),
    ]);
    // Act.
    let rust = export_mermaid(&rust, &full_options()).unwrap();
    let cpp = export_mermaid(&cpp, &full_options()).unwrap();
    // Assert.
    assert!(rust.contains("Normal:"));
    assert!(!rust.contains("Scratch:"));
    assert!(cpp.contains("Type relationships removed: 0."), "{cpp}");
}
