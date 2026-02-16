//! VB.NET language analyser.
//!
//! The VB.NET tree-sitter grammar is not available on crates.io.
//! This analyser is feature-gated and will report as unavailable
//! when the grammar is not compiled.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "Console.WriteLine",
        "Console.Write",
        "Console.ReadLine",
        "Console.ReadKey",
        "Debug.WriteLine",
        "Debug.Write",
        "Debug.Assert",
        "MessageBox.Show",
        "String.IsNullOrEmpty",
        "String.IsNullOrWhiteSpace",
        "String.Format",
        "String.Join",
        "String.Concat",
        "Math.Max",
        "Math.Min",
        "Math.Abs",
        "Convert.ToInt32",
        "Convert.ToString",
        "Convert.ToBoolean",
        "CStr",
        "CInt",
        "CLng",
        "CDbl",
        "CBool",
        "CType",
        "DirectCast",
        "TryCast",
        "Task.Run",
        "Task.WhenAll",
        "Task.WhenAny",
        "Task.Delay",
        "Task.FromResult",
        "Task.CompletedTask",
        "ValueTask.FromResult",
        "ValueTask.CompletedTask",
        "ArgumentNullException.ThrowIfNull",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

#[derive(Default)]
pub struct VbNetAnalyser {
    available: bool,
}

impl VbNetAnalyser {
    pub fn new() -> Self {
        // VB.NET grammar is not yet vendored in the Rust port
        Self { available: false }
    }
}

impl LanguageAnalyser for VbNetAnalyser {
    fn extensions(&self) -> &[&str] {
        &["vb"]
    }

    fn language_name(&self) -> &str {
        "VB.NET"
    }

    fn get_language(&self) -> Language {
        // This will only be called if is_available() returns true.
        // For now, return a placeholder — will be replaced when grammar is vendored.
        // Using C# as a stand-in to keep the code compiling.
        tree_sitter_c_sharp::LANGUAGE.into()
    }

    fn extract_symbols(&self, _tree: &Tree, _source: &[u8], _file_path: &str) -> Vec<Symbol> {
        Vec::new()
    }

    fn extract_imports(
        &self,
        _tree: &Tree,
        _source: &[u8],
        _file_path: &str,
    ) -> Vec<ImportStatement> {
        Vec::new()
    }

    fn extract_calls(&self, _tree: &Tree, _source: &[u8], _file_path: &str) -> Vec<RawCall> {
        Vec::new()
    }

    fn builtin_exclusions(&self) -> &HashSet<String> {
        &BUILTIN_EXCLUSIONS
    }

    fn is_available(&self) -> bool {
        self.available
    }
}
