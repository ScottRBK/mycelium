use mycelium_core::config::{AnalysisConfig, AnalysisResult};
use mycelium_core::mermaid::{export_mermaid, MermaidOptions, TestMode};
use mycelium_core::pipeline::run_pipeline;

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
    // Exercise saved maps too; the source checkout is gone before export.
    serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap()
}

#[test]
fn construction_does_not_resolve_to_type_only_interfaces_in_any_lookup_tier() {
    for (label, caller, other) in [
        (
            "global",
            "export function build() { return new Request('https://example.invalid'); }",
            "export interface Request { marker: string; }",
        ),
        (
            "imported",
            concat!(
                "import { marker } from './other';\n",
                "export function build() { return new Request('https://example.invalid'); }"
            ),
            "export const marker = 1; interface Request { marker: string; }",
        ),
        (
            "same file",
            concat!(
                "interface Request { marker: string; }\n",
                "export function build() { return new Request('https://example.invalid'); }"
            ),
            "",
        ),
    ] {
        // Arrange / Act: the interface describes a type, never the runtime Fetch constructor.
        let result = analyze(&[("src/app.ts", caller), ("src/other.ts", other)]);

        // Assert: unresolved external calls must not invent repository connections.
        assert_eq!(
            result.imports.file_imports.len(),
            usize::from(label == "imported")
        );
        assert!(result.calls.is_empty(), "{label}: {:?}", result.calls);
    }
}

#[test]
fn calls_never_match_unrelated_languages_but_keep_local_functions() {
    for (file, source) in [
        (
            "app.ts",
            "function helper() {}\nfunction build() { external.FetchPeer(); helper(); }",
        ),
        (
            "app.js",
            "function helper() {}\nfunction build() { external.FetchPeer(); helper(); }",
        ),
        (
            "app.py",
            "import external\ndef helper(): pass\ndef build(): external.FetchPeer(); helper()",
        ),
        (
            "app.rs",
            "fn helper() {}\nfn build() { external::FetchPeer(); helper(); }",
        ),
        (
            "app.go",
            concat!(
                "package app\nimport \"external\"\nfunc helper() {}\n",
                "func build() { external.FetchPeer(); helper() }"
            ),
        ),
        (
            "app.c",
            concat!(
                "#include <external.h>\nvoid helper(void) {}\n",
                "void build(void) { FetchPeer(); helper(); }"
            ),
        ),
        (
            "app.cpp",
            concat!(
                "#include <external.hpp>\nvoid helper() {}\n",
                "void build() { external::FetchPeer(); helper(); }"
            ),
        ),
        (
            "app.cs",
            "class App { void helper() {} void Build() { External.FetchPeer(); helper(); } }",
        ),
        (
            "app.java",
            "class App { void helper() {} void build() { External.FetchPeer(); helper(); } }",
        ),
        (
            "app.vb",
            concat!(
                "Public Module App\nPublic Sub helper()\nEnd Sub\nPublic Sub Build()\n",
                "external.FetchPeer()\nhelper()\nEnd Sub\nEnd Module\n"
            ),
        ),
    ] {
        // Arrange: an external call has a coincidental name match in another language.
        let foreign = if file.ends_with(".py") {
            ("other.ts", "export function FetchPeer() {}")
        } else {
            ("other.py", "def FetchPeer(): pass")
        };

        // Act.
        let result = analyze(&[(file, source), foreign]);

        // Assert: ordinary calls survive; cross-language coincidences do not.
        assert_eq!(result.calls.len(), 1, "{file}: {:?}", result.calls);
        let target = result
            .symbols
            .iter()
            .find(|s| s.id == result.calls[0].to)
            .unwrap();
        assert_eq!(target.name, "helper", "{file}");
        assert_eq!(target.file, file);
    }
}

#[test]
fn a_local_request_class_survives_an_imported_interface_collision() {
    // Arrange: importing another value must not turn the interface into a constructor.
    let result = analyze(&[
        (
            "src/app.ts",
            concat!(
                "import { marker } from './other';\n",
                "class Request {}\nfunction build() { return new Request(); }"
            ),
        ),
        (
            "src/other.ts",
            "export const marker = 1; interface Request { value: string; }",
        ),
    ]);

    // Act / Assert: inspect the public analysis result, including the chosen target's kind.
    assert_eq!(result.imports.file_imports.len(), 1);
    assert_eq!(result.calls.len(), 1);
    let target = result
        .symbols
        .iter()
        .find(|s| s.id == result.calls[0].to)
        .unwrap();
    assert_eq!(
        (&*target.file, &*target.name, &*target.symbol_type),
        ("src/app.ts", "Request", "Class")
    );
}

#[test]
fn saved_class_targets_export_construction_relationships_without_constructor_members() {
    for (file, source) in [
        (
            "app.ts",
            "class Widget {}\nfunction build() { return new Widget(); }",
        ),
        (
            "app.js",
            "class Widget {}\nfunction build() { return new Widget(); }",
        ),
        ("app.py", "class Widget: pass\ndef build(): return Widget()"),
        (
            "app.cs",
            "class Widget {}\nclass App { Widget build() { return new Widget(); } }",
        ),
        (
            "app.java",
            "class Widget {}\nclass App { Widget build() { return new Widget(); } }",
        ),
        (
            "app.cpp",
            "class Widget {};\nWidget build() { return Widget(); }",
        ),
        (
            "app.rs",
            "struct Widget(i32);\nfn build() -> Widget { Widget(1) }",
        ),
        (
            "inherited.ts",
            concat!(
                "class Base { constructor() {} }\nclass Widget extends Base {}\n",
                "function build() { return new Widget(); }"
            ),
        ),
        (
            "explicit.py",
            "class Widget:\n    def __init__(self): pass\ndef build(): return Widget()",
        ),
    ] {
        // Arrange: these calls already exist in saved maps, even without an explicit constructor.
        let result = analyze(&[(file, source)]);
        assert_eq!(result.calls.len(), 1, "{file}: {:?}", result.calls);

        // Act: the map can be exported with no source files present.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

        // Assert: construction connects to the class itself; no synthetic member is needed.
        assert!(
            output.contains("build() constructs Widget"),
            "{file}: {output}"
        );
        assert!(
            output.contains("Calls without in-scope endpoints: 0."),
            "{file}: {output}"
        );
        assert_eq!(
            output,
            export_mermaid(&result, &MermaidOptions::default()).unwrap()
        );
        let mut reordered = result.clone();
        reordered.symbols.reverse();
        reordered.calls.reverse();
        reordered.class_diagram.as_mut().unwrap().classes.reverse();
        assert_eq!(
            output,
            export_mermaid(&reordered, &MermaidOptions::default()).unwrap()
        );
    }
}

#[test]
fn rust_tuple_construction_targets_the_struct_even_when_it_has_an_impl() {
    // Arrange: the impl shares its name with the constructible declaration.
    let result = analyze(&[(
        "lib.rs",
        concat!(
            "struct Widget(i32);\nimpl Widget { fn new() -> Self { Self(1) } }\n",
            "fn build() -> Widget { Widget(1) }\nfn factory() -> Widget { Widget::new() }"
        ),
    )]);

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: both construction forms survive, with different source meanings.
    assert_eq!(result.calls.len(), 2, "{:?}", result.calls);
    assert!(output.contains("build() constructs Widget"), "{output}");
    assert!(output.contains("factory() calls new()"), "{output}");
    for call in &result.calls {
        let target = result.symbols.iter().find(|s| s.id == call.to).unwrap();
        assert_ne!(target.symbol_type, "Impl");
    }
}

#[test]
fn excluded_constructions_are_counted_even_with_missing_declaration_endpoints() {
    // Arrange: saved maps may contain call targets that their declaration model cannot describe.
    let complete = analyze(&[
        (
            "src/app.ts",
            "export class Widget {}\nfunction build() { return new Widget(); }",
        ),
        (
            "test/check.ts",
            concat!(
                "import { Widget } from '../src/app';\n",
                "function fixture() { return new Widget(); }"
            ),
        ),
    ]);
    for missing in ["none", "target", "caller"] {
        let mut result = complete.clone();
        result
            .class_diagram
            .as_mut()
            .unwrap()
            .classes
            .retain(|c| match missing {
                "target" => c.name != "Widget",
                "caller" => c.file != "test/check.ts",
                _ => true,
            });
        let before = serde_json::to_string(&result).unwrap();
        let options = MermaidOptions {
            test_paths: vec!["test".into()],
            ..Default::default()
        };

        // Act: exclusion, explicit keep, and include all operate on the same saved map.
        let excluded = export_mermaid(&result, &options).unwrap();
        let kept = export_mermaid(
            &result,
            &MermaidOptions {
                keep_paths: vec!["test".into()],
                ..options.clone()
            },
        )
        .unwrap();
        let included = export_mermaid(
            &result,
            &MermaidOptions {
                tests: TestMode::Include,
                ..options
            },
        )
        .unwrap();

        // Assert: intentional exclusion takes precedence over an unsupported endpoint.
        assert!(
            excluded.contains("Calls removed by test filtering: 1."),
            "{missing}: {excluded}"
        );
        let omitted = usize::from(missing == "target");
        assert!(excluded.contains(&format!("Calls without in-scope endpoints: {omitted}.")));
        assert!(
            kept.contains("Calls removed by test filtering: 0."),
            "{kept}"
        );
        let unfiltered_omitted = match missing {
            "target" => 2,
            "caller" => 1,
            _ => 0,
        };
        for output in [&kept, &included] {
            assert!(
                output.contains(&format!(
                    "Calls without in-scope endpoints: {unfiltered_omitted}."
                )),
                "{output}"
            );
        }
        assert_eq!(before, serde_json::to_string(&result).unwrap());
    }
}

#[test]
fn automatically_excluded_callers_do_not_require_a_displayable_target() {
    // Arrange: retain the call and symbols, but omit the target's declaration as in a partial map.
    let mut result = analyze(&[(
        "lib.rs",
        concat!(
            "struct Widget(i32);\nfn build() -> Widget { Widget(1) }\n",
            "#[test] fn check() { Widget(2); }"
        ),
    )]);
    result
        .class_diagram
        .as_mut()
        .unwrap()
        .classes
        .retain(|c| c.name != "Widget");

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: one production omission, one intentional test exclusion.
    assert!(
        output.contains("Calls without in-scope endpoints: 1."),
        "{output}"
    );
    assert!(
        output.contains("Calls removed by test filtering: 1."),
        "{output}"
    );
}

#[test]
fn cpp_and_vb_object_creation_reaches_both_the_map_and_diagram() {
    for (file, source) in [
        (
            "app.cpp",
            "class Widget {};\nWidget* build() { return new Widget(); }",
        ),
        (
            "app.vb",
            concat!(
                "Public Class Widget\nEnd Class\nPublic Module App\n",
                "Public Function build() As Widget\nReturn New Widget()\n",
                "End Function\nEnd Module\n"
            ),
        ),
    ] {
        // Arrange: use the real language grammar and public extraction boundary.
        let registry = mycelium_core::languages::AnalyserRegistry::new();
        let ext = file.rsplit('.').next().unwrap();
        let analyser = registry.get_by_extension(ext).unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&analyser.get_language_for_ext(ext))
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        assert!(!tree.root_node().has_error(), "{file}");

        // Act.
        let raw = analyser.extract_calls(&tree, source.as_bytes(), file);
        let result = analyze(&[(file, source)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

        // Assert: extraction, resolution and display all retain the construction.
        assert_eq!(raw.len(), 1, "{file}: {raw:?}");
        assert_eq!(
            (&*raw[0].caller_name, &*raw[0].callee_name),
            ("build", "Widget")
        );
        assert_eq!(result.calls.len(), 1, "{file}");
        assert!(
            output.contains("build() constructs Widget"),
            "{file}: {output}"
        );
        assert!(
            output.contains("Calls without in-scope endpoints: 0."),
            "{output}"
        );
    }
}

#[test]
fn typescript_and_javascript_calls_share_one_runtime_language_family() {
    for (caller_ext, target_ext) in [("ts", "js"), ("js", "ts"), ("tsx", "jsx"), ("jsx", "tsx")] {
        for imported in [true, false] {
            // Arrange: mixed-language projects share both functions and constructor values.
            let caller_file = format!("src/app.{caller_ext}");
            let target_file = format!("src/helper.{target_ext}");
            let import = if imported {
                "import { helper, Widget } from './helper';\n"
            } else {
                ""
            };
            let caller =
                format!("{import}export function build() {{ helper(); return new Widget(); }}");
            let result = analyze(&[
                (&caller_file, &caller),
                (
                    &target_file,
                    "export function helper() {}\nexport class Widget {}",
                ),
            ]);

            // Act: export the round-tripped analysis after its source has been removed.
            let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

            // Assert: both lookup tiers preserve each cross-language runtime connection.
            assert_eq!(
                result.calls.len(),
                2,
                "{caller_ext}->{target_ext}, imported={imported}: {:?}",
                result.calls
            );
            for call in &result.calls {
                let target = result.symbols.iter().find(|s| s.id == call.to).unwrap();
                assert_eq!(target.file, target_file);
                assert!(matches!(target.name.as_str(), "helper" | "Widget"));
                assert_eq!(
                    call.reason,
                    if imported {
                        "import-resolved"
                    } else {
                        "fuzzy-unique"
                    },
                    "{caller_ext}->{target_ext}, imported={imported}: {call:?}"
                );
            }
            assert!(output.contains("build() calls helper()"), "{output}");
            assert!(output.contains("build() constructs Widget"), "{output}");
            assert!(
                output.contains("Calls without in-scope endpoints: 0."),
                "{output}"
            );
        }
    }
}

#[test]
fn compatible_languages_keep_c_cpp_and_dotnet_calls() {
    for files in [
        [
            ("app.cpp", "void build() { helper(); }"),
            ("helper.c", "void helper(void) {}"),
        ],
        [
            ("app.cs", "class App { void Build() { Helpers.helper(); } }"),
            (
                "helper.vb",
                "Public Module Helpers\nPublic Sub helper()\nEnd Sub\nEnd Module\n",
            ),
        ],
        [
            (
                "app.vb",
                concat!(
                "Public Module App\nPublic Sub Build()\nHelpers.helper()\nEnd Sub\nEnd Module\n"
            ),
            ),
            (
                "helper.cs",
                "public class Helpers { public static void helper() {} }",
            ),
        ],
    ] {
        // Arrange / Act: these language families can share runtime declarations.
        let result = analyze(&files);

        // Assert: compatibility checks must preserve supported cross-language calls.
        assert_eq!(result.calls.len(), 1, "{}: {:?}", files[0].0, result.calls);
        let target = result
            .symbols
            .iter()
            .find(|s| s.id == result.calls[0].to)
            .unwrap();
        assert_eq!(target.file, files[1].0);
        assert_eq!(target.name, "helper");
    }
}

#[test]
fn keeping_an_impl_does_not_restore_construction_of_a_filtered_type() {
    // Arrange: an owner box may survive solely as context for retained impl members.
    let result = analyze(&[
        ("widget.rs", "#[cfg(test)] pub struct Widget(pub i32);"),
        (
            "app.rs",
            concat!(
                "impl Widget { fn ping(&self) {} }\n",
                "fn build() -> Widget { Widget(1) }"
            ),
        ),
    ]);

    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: box context does not undo selection of the original class declaration.
    assert!(output.contains("Owner retained as context"), "{output}");
    assert!(!output.contains("constructs Widget"), "{output}");
    assert!(
        output.contains("Calls removed by test filtering: 1."),
        "{output}"
    );
}

#[test]
fn c_struct_tags_are_not_runtime_function_targets() {
    // Arrange: C has separate tag and function names; the header's function is external.
    let source = concat!(
        "#include <external.h>\nstruct Fetch { int value; };\n",
        "void build(void) { Fetch(); }"
    );

    // Act.
    let result = analyze(&[("app.c", source)]);

    // Assert: a same-named tag is never evidence of a C constructor call.
    assert!(result.calls.is_empty(), "{:?}", result.calls);
}

#[test]
fn function_valued_properties_remain_possible_runtime_call_targets() {
    // Arrange: value declarations can hold callbacks even though they are not methods.
    let source = concat!(
        "class Worker {\n    handler: (value: string) => void;\n",
        "    run() { this.handler('ready'); }\n}"
    );

    // Act.
    let result = analyze(&[("app.ts", source)]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

    // Assert: rejecting type-only declarations must preserve ordinary callback calls.
    assert_eq!(result.calls.len(), 1, "{:?}", result.calls);
    assert!(output.contains("run() calls handler()"), "{output}");
}

#[test]
fn explicit_constructor_members_and_ordinary_methods_keep_their_call_labels() {
    for (file, source) in [
        (
            "app.cs",
            concat!(
                "class Widget {\npublic Widget() {}\npublic void ping() {}\n}\n",
                "class App { Widget build() { return new Widget(); }\n",
                "void exercise(Widget w) { w.ping(); } }"
            ),
        ),
        (
            "app.java",
            concat!(
                "class Widget {\nWidget() {}\nvoid ping() {}\n}\n",
                "class App { Widget build() { return new Widget(); }\n",
                "void exercise(Widget w) { w.ping(); } }"
            ),
        ),
    ] {
        // Arrange / Act: constructor symbols already match declared members in these languages.
        let result = analyze(&[(file, source)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();

        // Assert: type endpoints extend export without replacing existing member endpoints.
        assert_eq!(result.calls.len(), 2, "{file}: {:?}", result.calls);
        assert!(
            output.contains("build() calls Widget()"),
            "{file}: {output}"
        );
        assert!(
            output.contains("exercise() calls ping()"),
            "{file}: {output}"
        );
        assert!(
            output.contains("Calls without in-scope endpoints: 0."),
            "{output}"
        );
    }
}
