//! Source declarations retained for diagram export, independently of call-resolution heuristics.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const TEST_DETECTOR_VERSION: u32 = 2;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassDiagram {
    pub classes: Vec<Class>,
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_detection: Option<TestDetection>,
    /// Saved module bindings allow Python type resolution without reopening source files.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub python_bindings: BTreeMap<String, PythonBindings>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PythonBindings {
    pub names: BTreeMap<String, PythonBinding>,
    /// Wildcard imports or malformed syntax prevent reliable binding resolution.
    pub uncertain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PythonBinding {
    /// A from-import, retaining the original name behind an alias.
    Import(String),
    /// A directly imported module with an alias, such as `import app.models as models`.
    Module(String),
    /// Unaliased module imports sharing a root, such as `import app.models, app.services`.
    Modules(BTreeSet<String>),
    /// Line of an unambiguous module-level class declaration.
    Class(usize),
    /// A conflicting or unsupported binding; never fall back to a name-based guess.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub line: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<TestEvidence>,
    pub members: Vec<Member>,
    pub bases: Vec<Base>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Base {
    pub name: String,
    /// `inherits` or `implements`; a field is never evidence of exclusive ownership.
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Member {
    #[serde(default)]
    pub file: String,
    pub name: String,
    pub kind: String,
    pub visibility: String,
    pub parameters: Vec<Parameter>,
    pub value_type: Option<String>,
    #[serde(default)]
    pub line: usize,
    #[serde(default)]
    pub end_line: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test: Option<TestEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value_type: Option<String>,
}

/// Versioned, saved detection facts. Missing metadata means detection was not performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDetection {
    pub version: u32,
    pub diagnostics: Vec<TestDiagnostic>,
}

impl Default for TestDetection {
    fn default() -> Self {
        Self {
            version: TEST_DETECTOR_VERSION,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TestEvidence {
    pub rule: String,
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDiagnostic {
    pub file: String,
    pub message: String,
}

/// Version 1 recorded Rust/Go only; version 2 adds conservative framework rules.
pub(crate) fn supported_test_rule(version: u32, rule: &str) -> bool {
    if !(1..=TEST_DETECTOR_VERSION).contains(&version) {
        return false;
    }
    if matches!(rule, "rust.test" | "rust.cfg-test" | "go.test-file") {
        return true;
    }
    version >= 2
        && matches!(
            rule,
            "python.unittest-case"
                | "python.pytest-fixture"
                | "dotnet.mstest-class"
                | "dotnet.mstest-method"
                | "dotnet.nunit-class"
                | "dotnet.nunit-method"
                | "dotnet.xunit-method"
                | "java.junit-method"
                | "javascript.node-callback"
                | "javascript.vitest-callback"
                | "javascript.jest-callback"
        )
}
