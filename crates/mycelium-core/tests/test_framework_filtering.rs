use mycelium_core::config::{AnalysisConfig, AnalysisResult};
use mycelium_core::mermaid::{export_mermaid, MermaidOptions};
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
    // Export must work after removing the source checkout and round-tripping saved JSON.
    serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap()
}

#[test]
fn python_framework_evidence_filters_direct_containers_and_fixtures_only() {
    // Arrange: test containers do not confer their role on independently named nested types.
    let result = analyze(&[(
        "mixed.py",
        r#"
import unittest as ut
from unittest import TestCase as Case
import pytest as pt
from pytest import fixture as setup
class Checks(ut.TestCase):
    value: int
    def helper(self): pass
    class Payload:
        def serialize(self): pass
class MoreChecks(Case):
    def lifecycle(self): pass
class Indirect(Checks):
    def application(self): pass
@pt.fixture(scope="module")
def support(): pass
@setup
def other_support(): pass
@pt.mark.slow
def test_convention(): pass
def application(): pass
"#,
    )]);
    // Act: the checkout has already been deleted by analyze().
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    let include = export_mermaid(
        &result,
        &MermaidOptions {
            tests: mycelium_core::mermaid::TestMode::Include,
            ..Default::default()
        },
    )
    .unwrap();
    // Assert: check saved source facts as well as the rendered view.
    for hidden in ["helper()", "lifecycle()", "support()", "other_support()"] {
        assert!(include.contains(hidden), "Missing fixture fact: {hidden}");
        assert!(!output.contains(hidden), "Leaked {hidden}: {output}");
    }
    for kept in [
        "Payload",
        "serialize()",
        "Indirect",
        "application()",
        "test_convention()",
    ] {
        assert!(output.contains(kept), "Lost {kept}: {output}");
    }
    assert!(output.contains("python.unittest-case"));
    assert!(output.contains("python.pytest-fixture"));
}

#[test]
fn python_uncertain_framework_bindings_and_local_modules_stay_visible() {
    // Arrange: each fixture has a supported spelling whose binding is uncertain.
    for shadow in [
        "ut = Local()",
        "def configure(ut): pass",
        "for ut in things: pass",
        "from local import *",
        "from local import ut",
        "ut.TestCase = Local",
        "del ut",
    ] {
        let source = format!(
            "import unittest as ut\n{shadow}\nclass Kept(ut.TestCase):\n    def work(self): pass\n"
        );
        let result = analyze(&[("app.py", &source)]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(output.contains("work()"), "Hidden by {shadow}: {output}");
    }
    for module in ["unittest.py", "src/unittest/__init__.py"] {
        let result = analyze(&[
            (module, "class TestCase: pass"),
            (
                "app.py",
                "import unittest\nclass Kept(unittest.TestCase):\n    def work(self): pass\n",
            ),
        ]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(
            output.contains("work()"),
            "Local {module} misidentified: {output}"
        );
    }
    let malformed = analyze(&[(
        "broken.py",
        "import unittest\nclass Kept(unittest.TestCase):\n    def work(self): pass\n! invalid\n",
    )]);
    let output = export_mermaid(&malformed, &MermaidOptions::default()).unwrap();
    assert!(output.contains("work()"));
    assert!(output.contains("syntax-derived test detection disabled"));
}

#[test]
fn csharp_framework_markers_distinguish_containers_from_test_methods() {
    // Arrange: direct namespace imports, type/namespace aliases and fully qualified attributes.
    let result = analyze(&[(
        "mixed.cs",
        r#"
using Xunit;
using Check = Xunit.FactAttribute;
using N = NUnit.Framework;
using Microsoft.VisualStudio.TestTools.UnitTesting;
[TestClass] class Container {
    void helper() {}
    class Payload { public void serialize() {} }
}
[N.TestFixture] class Fixture { void setup() {} }
class Mixed {
    [Fact(Skip="disabled")] void fact() {}
    [TheoryAttribute] void theory() {}
    [Check] void alias() {}
    [N.Test] void nunit() {}
    [N.TestCase(1)] void parameterized() {}
    [N.TestCaseSource("Rows")] void sourced() {}
    [global::Microsoft.VisualStudio.TestTools.UnitTesting.TestMethod] void mstest() {}
    void application() {}
}
partial class Parts { [Xunit.Fact] void test_part() {} }
partial class Parts { void app_part() {} }
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    for hidden in [
        "helper()",
        "setup()",
        "fact()",
        "theory()",
        "alias()",
        "nunit()",
        "parameterized()",
        "sourced()",
        "mstest()",
        "test_part()",
    ] {
        assert!(!output.contains(hidden), "Leaked {hidden}: {output}");
    }
    for kept in [
        "Payload",
        "serialize()",
        "Mixed",
        "application()",
        "app_part()",
    ] {
        assert!(output.contains(kept), "Lost {kept}: {output}");
    }
    assert!(output.contains("dotnet.mstest-class"));
    assert!(output.contains("dotnet.nunit-class"));
    assert!(output.contains("dotnet.xunit-method"));
}

#[test]
fn csharp_bindings_check_other_files_namespace_scopes_and_alias_conflicts() {
    // Arrange: same-namespace declarations can change attribute binding across files.
    let cases = [
        (
            "using Xunit; namespace App; class Mixed { [Fact] void kept() {} }",
            "namespace App; class FactAttribute : System.Attribute {}",
        ),
        (
            "using Xunit; namespace App; class Mixed { [Fact] void kept() {} }",
            "namespace App; class Fact : System.Attribute {}",
        ),
        (
            "namespace App; class Mixed { [Xunit.Fact] void kept() {} }",
            "namespace App; class Xunit { public class FactAttribute : System.Attribute {} }",
        ),
        (
            "class Mixed { [global::Xunit.Fact] void kept() {} }",
            "namespace Xunit; class FactAttribute : System.Attribute {}",
        ),
        (
            "using F = Xunit.FactAttribute; using F = Other.Fact; class C { [F] void kept() {} }",
            "class Other {}",
        ),
        (
            "namespace A { using Xunit; class Hidden { [Fact] void check() {} } }\n\
          namespace B { class Mixed { [Fact] void kept() {} } }",
            "class Other {}",
        ),
    ];
    for (source, conflict) in cases {
        let result = analyze(&[("a.cs", source), ("z.cs", conflict)]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(
            output.contains("kept()"),
            "False positive for {source}: {output}"
        );
    }
    let result = analyze(&[
        (
            "a.cs",
            "namespace A { using Xunit; class Mixed { [Fact] void check() {} } }",
        ),
        (
            "z.cs",
            "namespace B; class FactAttribute : System.Attribute {}",
        ),
    ]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(
        !output.contains("check()"),
        "Unrelated namespace blocked detection: {output}"
    );
}

#[test]
fn vb_framework_markers_support_case_insensitive_imports_aliases_and_global_names() {
    // Arrange: paths stay case-sensitive; VB identifiers do not.
    let result = analyze(&[(
        "Mixed.vb",
        r#"
Imports X = Xunit
Imports Check = Microsoft.VisualStudio.TestTools.UnitTesting.TestMethodAttribute
Imports NUnit.Framework
<TestFixture>
Public Class Fixture
    Public Sub Setup()
    End Sub
End Class
Public Class Mixed
    <x.fAcT()>
    Public Sub FactCase()
    End Sub
    <cHeCk>
    Public Sub MethodCase()
    End Sub
    <Global.Xunit.TheoryAttribute>
    Public Sub TheoryCase()
    End Sub
    Public Sub Application()
    End Sub
End Class
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    for hidden in ["Setup()", "FactCase()", "MethodCase()", "TheoryCase()"] {
        assert!(!output.contains(hidden), "Leaked {hidden}: {output}");
    }
    assert!(output.contains("Application()"));
    assert!(
        !output.contains("syntax errors"),
        "Parser did not accept fixture: {output}"
    );
    let kept = export_mermaid(
        &result,
        &MermaidOptions {
            keep_paths: vec!["Mixed.vb".into()],
            ..Default::default()
        },
    )
    .unwrap();
    assert!(kept.contains("FactCase()"));
    assert!(export_mermaid(
        &result,
        &MermaidOptions {
            keep_paths: vec!["mixed.vb".into()],
            ..Default::default()
        }
    )
    .is_err());
}

#[test]
fn java_direct_junit_annotations_filter_methods_with_verified_bindings() {
    // Arrange: supported annotations, a mixed class and a fixture lifecycle method.
    let result = analyze(&[(
        "Mixed.java",
        r#"
package app;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.api.RepeatedTest;
import org.junit.jupiter.api.TestFactory;
import org.junit.jupiter.api.TestTemplate;
class Mixed {
    @Test void check() {}
    @ParameterizedTest void parameters() {}
    @RepeatedTest(3) void repeated() {}
    @TestFactory Object factory() { return null; }
    @TestTemplate void template() {}
    @org.junit.Test void legacy() {}
    @org.junit.jupiter.api.BeforeEach void setup() {}
    void application() {}
}
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    for hidden in [
        "check()",
        "parameters()",
        "repeated()",
        "factory()",
        "template()",
        "legacy()",
    ] {
        assert!(!output.contains(hidden), "Leaked {hidden}: {output}");
    }
    assert!(output.contains("Mixed") && output.contains("setup()"));
    assert!(output.contains("application()"));
    for (source, conflict) in [
        (
            "package app; import org.junit.jupiter.api.Test; class C { @Test void kept() {} }",
            "package app; public @interface Test {}",
        ),
        (
            "package app; import org.junit.jupiter.api.*; class C { @Test void kept() {} }",
            "",
        ),
        ("package app; class C { @Test void kept() {} }", ""),
        (
            "package app; class C { @org.junit.Test void kept() {} }",
            "package app; class org {}",
        ),
    ] {
        let result = analyze(&[("Mixed.java", source), ("Other.java", conflict)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(
            output.contains("kept()"),
            "False positive for {source}: {output}"
        );
    }
}

#[test]
fn js_and_ts_filter_existing_declarations_inside_direct_imported_callbacks() {
    // Arrange: anonymous tests are not invented as diagram members; existing lexical helpers are.
    for extension in ["ts", "tsx", "js", "jsx"] {
        let source = r#"
import {test as check, describe} from 'node:test';
import {it as example, suite} from 'vitest';
import {test as jestTest} from '@jest/globals';
check('case', {timeout: 100}, () => {
    class Fake { work() {} }
    function support() {}
});
describe('suite', function () { class SuiteHelper {} });
example('vitest', () => { class VitestHelper {} });
suite('vitest suite', {timeout: 100}, () => { class VitestSuiteHelper {} });
jestTest('jest', () => { class JestHelper {} }, 100);
function shared() {}
check('shared', shared);
check.skip('modifier deferred', () => { class Deferred {} });
class Application { run() {} }
"#;
        let result = analyze(&[(&format!("mixed.{extension}"), source)]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        for hidden in [
            "Fake",
            "support()",
            "SuiteHelper",
            "VitestHelper",
            "JestHelper",
            "VitestSuiteHelper",
        ] {
            assert!(
                !output.contains(hidden),
                "Leaked {hidden} in {extension}: {output}"
            );
        }
        for kept in ["shared()", "Deferred", "Application", "run()"] {
            assert!(
                output.contains(kept),
                "Lost {kept} in {extension}: {output}"
            );
        }
    }
}

#[test]
fn javascript_commonjs_and_shadowing_are_checked_without_executing_imports() {
    // Arrange: only literal const destructuring is accepted for CommonJS.
    let result = analyze(&[(
        "mixed.js",
        r#"
const {test: check, describe} = require('node:test');
check('case', () => { class Fake {} });
describe('suite', () => { function helper() {} });
class Application {}
"#,
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(!output.contains("Fake"), "{output}");
    assert!(!output.contains("helper()"), "{output}");
    assert!(output.contains("Application"));
    for declaration in [
        "const {test: check} = require('node:test'); function require(name) {}",
        "const {test: check} = require(moduleName);",
        "const {test: check} = require('node:test'); check = application;",
        "import {test as check} from 'node:test'; function configure(check) {}",
        "import {test as check} from 'node:test'; { let check = application; }",
        "import {test as check} from './local';",
        "import {test as check} from 'node:test'; ({check} = application);",
        "import {test as check} from 'node:test'; for (check of applications) {}",
        "import {test as check} from 'node:test'; import check from './other';",
    ] {
        let source = format!("{declaration}\ncheck('case', () => {{ class Kept {{}} }});");
        let result = analyze(&[("lookalike.js", &source)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(
            output.contains("Kept"),
            "False positive for {declaration}: {output}"
        );
    }
}

#[test]
fn old_detector_versions_keep_saved_rules_without_claiming_framework_detection() {
    // Arrange: version 1 maps retain their existing Rust/Go evidence.
    let mut result = analyze(&[("lib.rs", "#[test] fn check() {} fn app() {}")]);
    result
        .class_diagram
        .as_mut()
        .unwrap()
        .test_detection
        .as_mut()
        .unwrap()
        .version = 1;
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(!output.contains("check()"));
    assert!(output.contains("app()"));
    assert!(output.contains("rerun analysis to enable framework detection"));
    let current = analyze(&[(
        "app.py",
        "import pytest\n@pytest.fixture\ndef support(): pass",
    )]);
    assert_eq!(
        current
            .class_diagram
            .as_ref()
            .unwrap()
            .test_detection
            .as_ref()
            .unwrap()
            .version,
        2
    );
    let mut old = current.clone();
    old.class_diagram
        .as_mut()
        .unwrap()
        .test_detection
        .as_mut()
        .unwrap()
        .version = 1;
    assert!(export_mermaid(&old, &MermaidOptions::default())
        .unwrap()
        .contains("support()"));
}

#[test]
fn dotnet_global_aliases_and_vb_local_markers_prevent_false_positives() {
    // Arrange: global aliases are deliberately unresolved and must also prevent name guesses.
    let result = analyze(&[
        ("global.cs", "global using Xunit = Local;"),
        ("app.cs", "class Mixed { [Xunit.Fact] void kept() {} }"),
    ]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("kept()"), "{output}");
    let result = analyze(&[
        (
            "app.vb",
            "Imports Xunit\nNamespace App\nPublic Class Mixed\n\
            <Fact>\nPublic Sub Kept()\nEnd Sub\nEnd Class\nEnd Namespace\n",
        ),
        (
            "local.vb",
            "Namespace App\nPublic Class fAcTaTtRiBuTe\nEnd Class\nEnd Namespace\n",
        ),
    ]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept()"), "{output}");
}

#[test]
fn node_default_import_is_a_test_binding_but_type_only_imports_are_not() {
    // Arrange: the default import is the form used by the pinned Pi extension.
    for ext in ["js", "ts"] {
        let result = analyze(&[(
            &format!("default.{ext}"),
            "import check from 'node:test'; check('case', () => { class Fake {} });",
        )]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(!output.contains("Fake"), "{output}");
    }
    for source in [
        "import type {test} from 'node:test'; test('case', () => { class Kept {} });",
        "import {type test} from 'node:test'; test('case', () => { class Kept {} });",
    ] {
        let result = analyze(&[("typeonly.ts", source)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(output.contains("Kept"), "{output}");
    }
}

#[test]
fn recovered_conflicting_files_still_prevent_framework_name_assumptions() {
    // Arrange: an incomplete neighbouring file may still declare a shadowing type.
    let result = analyze(&[
        (
            "app.cs",
            "using Xunit; namespace App; class C { [Fact] void kept() {} }",
        ),
        (
            "incomplete.cs",
            "namespace App; class FactAttribute {} class Broken {",
        ),
    ]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("kept()"), "{output}");
}

#[test]
fn pattern_captures_and_typescript_value_declarations_can_shadow_frameworks() {
    // Arrange: these bindings are not ordinary assignments.
    let result = analyze(&[(
        "pattern.py",
        "import unittest as ut\n\
        match value:\n    case ut:\n        pass\n\
        class Kept(ut.TestCase):\n    def work(self): pass\n",
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("work()"), "{output}");
    let result = analyze(&[(
        "shadow.ts",
        "import {test} from 'node:test';\n\
        { enum test { Value }; test('case', () => { class Kept {} }); }",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept"), "{output}");
}

#[test]
fn framework_selection_is_saved_repeatable_and_independent_of_record_order() {
    // Arrange: qualified calls and declared types exercise filtering beyond box selection.
    let files = [
        (
            "a.py",
            "import pytest\nclass Service:\n    def run(self): pass\n\
            @pytest.fixture\ndef fixture() -> Service:\n    return Service()\n\
            def application():\n    return fixture()\n",
        ),
        (
            "b.cs",
            "using Xunit; namespace App; class C { [Fact] void hidden() {} void run() {} }",
        ),
        (
            "c.java",
            "package app; class C { @org.junit.Test void hidden() {} void run() {} }",
        ),
        (
            "d.js",
            "import {test} from 'node:test'; test('case', () => { class Fake {} });",
        ),
    ];
    let result = analyze(&files);
    let mut reversed_files = files;
    reversed_files.reverse();
    let again = analyze(&reversed_files);
    let mut shuffled = result.clone();
    let diagram = shuffled.class_diagram.as_mut().unwrap();
    diagram.classes.reverse();
    for class in &mut diagram.classes {
        class.members.reverse();
    }
    diagram
        .test_detection
        .as_mut()
        .unwrap()
        .diagnostics
        .reverse();
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    let reordered = export_mermaid(&shuffled, &MermaidOptions::default()).unwrap();
    let repeated = export_mermaid(&again, &MermaidOptions::default()).unwrap();
    // Assert.
    assert_eq!(output, reordered);
    assert_eq!(output, repeated);
    assert!(!output.contains("fixture() calls") && !output.contains("calls fixture()"));
    assert!(!output.contains("hidden()") && !output.contains("Fake"));
    assert!(
        // Both application -> fixture and fixture -> Service construction are filtered.
        output.contains("Calls removed by test filtering: 2."),
        "{output}"
    );
}

#[test]
fn dotnet_relative_import_targets_are_checked_before_accepting_framework_identity() {
    // Arrange: a namespace-local using can bind a local namespace with a framework-like name.
    for directive in ["using Xunit;", "using Fact = Xunit.FactAttribute;"] {
        let source =
            format!("namespace App {{ {directive} class C {{ [Fact] void kept() {{}} }} }}");
        let result = analyze(&[
            ("app.cs", &source),
            (
                "local.cs",
                "namespace App.Xunit { class FactAttribute : System.Attribute {} }",
            ),
        ]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(output.contains("kept()"), "{output}");
    }
    let result = analyze(&[(
        "global.cs",
        "using Xunit = Other;\n\
        class C { [global::Xunit.Fact] void hidden() {} }\nclass Other {}",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(
        !output.contains("hidden()"),
        "Global qualification must bypass aliases: {output}"
    );
}

#[test]
fn imported_require_and_commented_import_modifiers_remain_conservative() {
    // Arrange: imports can shadow require, and comments do not change keyword roles.
    let result = analyze(&[(
        "loader.js",
        "import require from './loader';\n\
        const {test} = require('node:test'); test('case', () => { class Kept {} });",
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("Kept"), "{output}");
    let result = analyze(&[
        ("global.cs", "global/* comment */using Xunit = Local;"),
        ("app.cs", "class Mixed { [Xunit.Fact] void kept() {} }"),
    ]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("kept()"), "{output}");
    for import in [
        "import/*comment*/type {test}",
        "import {type/*comment*/test}",
    ] {
        let source = format!(
            "{import} from 'node:test';\n\
            test('case', () => {{ class Kept {{}} }});"
        );
        let result = analyze(&[("typeonly.ts", &source)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(output.contains("Kept"), "{output}");
    }
}

#[test]
fn java_direct_annotations_work_alongside_static_assertion_imports() {
    // Arrange: JUnit's ordinary static assertion imports do not shadow an explicit Test import.
    for assertions in [
        "import static org.junit.jupiter.api.Assertions.*;",
        "import static org.junit.jupiter.api.Assertions.assertEquals;",
        "import other.*;",
    ] {
        let source = format!(
            "import org.junit.jupiter.api.Test; {assertions}\n\
            class Mixed {{ @Test void hidden() {{}} void application() {{}} }}"
        );
        let result = analyze(&[("Mixed.java", &source)]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(!output.contains("hidden()"), "{output}");
        assert!(output.contains("application()"));
    }
}

#[test]
fn vb_nested_types_and_global_namespace_lookalikes_do_not_hide_application_code() {
    // Arrange: nested type attributes belong to that type, never its application container.
    let result = analyze(&[(
        "nested.vb",
        "Imports NUnit.Framework\n\
        Public Class Service\nPublic Sub Run()\nEnd Sub\n\
        <TestFixture>\nPublic Class Tests\nEnd Class\nEnd Class\n",
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("Run()"), "{output}");
    assert!(
        !output.contains("syntax-derived test detection disabled"),
        "{output}"
    );
    for (namespace, local) in [
        (
            "App",
            "Namespace Global.App\nPublic Class FactAttribute\nEnd Class\nEnd Namespace\n",
        ),
        (
            "App",
            "Namespace App\nModule Helpers\nPublic Class FactAttribute\n\
            End Class\nEnd Module\nEnd Namespace\n",
        ),
    ] {
        let source = format!(
            "Imports Xunit\nNamespace {namespace}\nPublic Class Mixed\n\
            <Fact>\nPublic Sub Kept()\nEnd Sub\nEnd Class\nEnd Namespace\n"
        );
        let result = analyze(&[("mixed.vb", &source), ("local.vb", local)]);
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        assert!(output.contains("Kept()"), "{local}: {output}");
    }
}

#[test]
fn vb_option_blank_lines_and_commented_js_arguments_are_supported() {
    // Arrange: ordinary formatting must not disable otherwise unambiguous evidence.
    let result = analyze(&[(
        "options.vb",
        "Option Strict On\n\nImports Xunit\n\
        Public Class Mixed\n<Fact>\nPublic Sub Hidden()\nEnd Sub\nEnd Class\n",
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(!output.contains("Hidden()"), "{output}");
    assert!(!output.contains("malformed syntax"), "{output}");
    let result = analyze(&[(
        "comments.js",
        "import {test} from 'node:test';\n\
        test('case', /* comment */ () => { class Hidden {} });",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(!output.contains("Hidden"), "{output}");
}

#[test]
fn dotnet_nested_interfaces_and_attribute_targets_remain_independent() {
    // Arrange: a type inside a test container has its own identity; return targets are not tests.
    let result = analyze(&[(
        "nested.vb",
        "Imports NUnit.Framework\n\
        <TestFixture>\nPublic Class Tests\n\
        Public Interface Contract\nEnd Interface\nEnd Class\n",
    )]);
    // Act.
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    // Assert.
    assert!(output.contains("Contract"), "{output}");
    assert!(
        !output.contains("syntax-derived test detection disabled"),
        "{output}"
    );
    let result = analyze(&[(
        "targets.cs",
        "namespace App; using Xunit;\n\
        class Mixed { [return: Fact] object Kept() => null; [Fact] void Hidden() {} }",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept()"), "{output}");
    assert!(!output.contains("Hidden()"), "{output}");
}

#[test]
fn inherited_and_static_member_type_bindings_are_left_for_explicit_paths() {
    // Arrange: inherited/imported member types can beat a framework import.
    for source in [
        "using Xunit; class Base { public class FactAttribute : System.Attribute {} }\n\
        class Mixed : Base { [Fact] void Kept() {} [global::Xunit.Fact] void Hidden() {} }",
        "using Xunit; using static Local;\n\
        class Local { public class FactAttribute : System.Attribute {} }\n\
        class Mixed { [Fact] void Kept() {} [global::Xunit.Fact] void Hidden() {} }",
    ] {
        let result = analyze(&[("bindings.cs", source)]);
        // Act.
        let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
        // Assert.
        assert!(output.contains("Kept()"), "{output}");
        assert!(!output.contains("Hidden()"), "{output}");
    }
    let result = analyze(&[(
        "Bindings.java",
        "import org.junit.Test;\n\
        class Base { @interface Test {} }\n\
        class Mixed extends Base { @Test void Kept() {} }",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept()"), "{output}");
    let result = analyze(&[
        (
            "imports.cs",
            "global using static Local;\n\
            class Local { public class FactAttribute : System.Attribute {} }",
        ),
        (
            "consumer.cs",
            "using Xunit; class Mixed { [Fact] void Kept() {} }",
        ),
    ]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept()"), "{output}");
}

#[test]
fn vb_nested_member_ownership_and_inherited_lookalikes_survive_export() {
    // Arrange: nesting affects declaration ownership as well as test-evidence boundaries.
    let result = analyze(&[(
        "nested.vb",
        "Imports NUnit.Framework\n\
        <TestFixture>\nPublic Class Tests\nPublic Sub Hidden()\nEnd Sub\n\
        Public Class Service\nPublic Sub Run()\nEnd Sub\nEnd Class\n\
        Public Structure Value\nPublic Number As Integer\nEnd Structure\n\
        Public Enum State\nReady\nEnd Enum\nEnd Class\n",
    )]);
    // Act.
    let excluded = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    let included = export_mermaid(
        &result,
        &MermaidOptions {
            tests: mycelium_core::mermaid::TestMode::Include,
            ..Default::default()
        },
    )
    .unwrap();
    // Assert: verify source facts and each owning box, not just text anywhere in the document.
    assert!(
        !excluded.contains("syntax-derived test detection disabled"),
        "{excluded}"
    );
    assert!(!excluded.contains("Hidden()"), "{excluded}");
    for (owner, member) in [
        ("Tests", "Hidden"),
        ("Service", "Run"),
        ("Value", "Number"),
        ("State", "Ready"),
    ] {
        let class = result
            .class_diagram
            .as_ref()
            .unwrap()
            .classes
            .iter()
            .find(|c| c.name == owner)
            .unwrap();
        assert_eq!(class.members.len(), 1, "{class:?}");
        assert_eq!(class.members[0].name, member);
        let box_body = included
            .split(&format!("[\"{owner}\"] {{"))
            .nth(1)
            .unwrap()
            .split("\n    }")
            .next()
            .unwrap();
        assert!(box_body.contains(member), "{box_body}");
        if owner != "Tests" {
            assert!(
                class.test.is_none() && class.members[0].test.is_none(),
                "{class:?}"
            );
            assert!(excluded.contains(member), "{excluded}");
        }
    }
    let result = analyze(&[(
        "inherited.vb",
        "Imports Xunit\n\
        Public Class Base\nPublic Class FactAttribute\nInherits System.Attribute\n\
        End Class\nEnd Class\nPublic Class Mixed\nInherits Base\n\
        <Fact>\nPublic Sub Kept()\nEnd Sub\nEnd Class\n",
    )]);
    let output = export_mermaid(&result, &MermaidOptions::default()).unwrap();
    assert!(output.contains("Kept()"), "{output}");
    let member = result
        .class_diagram
        .as_ref()
        .unwrap()
        .classes
        .iter()
        .flat_map(|c| &c.members)
        .find(|m| m.name == "Kept")
        .unwrap();
    assert!(member.test.is_none());
}
