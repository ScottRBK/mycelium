//! Language analyser trait and registry.

use std::collections::{HashMap, HashSet};

use tree_sitter::{Language, Tree};

use crate::config::{ImportStatement, RawCall, Symbol};

pub mod c_cpp;
pub mod csharp;
pub mod go_lang;
pub mod java;
pub mod python;
pub mod rust_lang;
pub mod typescript;
pub mod vbnet;

/// Trait that all language analysers implement.
pub trait LanguageAnalyser: Send + Sync {
    /// File extensions this analyser handles (e.g. &["cs"]).
    fn extensions(&self) -> &[&str];

    /// Human-readable language name (e.g. "C#").
    fn language_name(&self) -> &str;

    /// Get the tree-sitter Language for parsing.
    fn get_language(&self) -> Language;

    /// Extract symbols (classes, methods, etc.) from a parsed AST.
    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol>;

    /// Extract import statements from a parsed AST.
    fn extract_imports(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement>;

    /// Extract raw call sites from a parsed AST.
    fn extract_calls(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<RawCall>;

    /// Names that should be excluded from call resolution (builtins, framework methods, etc.).
    fn builtin_exclusions(&self) -> &HashSet<String>;

    /// Get the tree-sitter Language for a specific file extension.
    /// Override for analysers that handle multiple languages (e.g. TypeScript/JavaScript).
    fn get_language_for_ext(&self, _ext: &str) -> Language {
        self.get_language()
    }

    /// Whether this analyser is available (e.g. VB.NET grammar may not be compiled).
    fn is_available(&self) -> bool {
        true
    }
}

/// Registry mapping file extensions to analysers.
pub struct AnalyserRegistry {
    analysers: Vec<Box<dyn LanguageAnalyser>>,
    extension_map: HashMap<String, usize>,
}

impl AnalyserRegistry {
    /// Build the registry with all available language analysers.
    pub fn new() -> Self {
        let analysers: Vec<Box<dyn LanguageAnalyser>> = vec![
            Box::new(csharp::CSharpAnalyser::new()),
            Box::new(typescript::TypeScriptAnalyser::new()),
            Box::new(python::PythonAnalyser::new()),
            Box::new(java::JavaAnalyser::new()),
            Box::new(go_lang::GoAnalyser::new()),
            Box::new(rust_lang::RustAnalyser::new()),
            Box::new(c_cpp::CAnalyser::new()),
            Box::new(c_cpp::CppAnalyser::new()),
            // VB.NET is conditionally available
            Box::new(vbnet::VbNetAnalyser::new()),
        ];

        let mut extension_map = HashMap::new();
        for (i, analyser) in analysers.iter().enumerate() {
            if analyser.is_available() {
                for ext in analyser.extensions() {
                    extension_map.insert(ext.to_string(), i);
                }
            }
        }

        Self {
            analysers,
            extension_map,
        }
    }

    /// Get the analyser for a given file extension, if one exists.
    pub fn get_by_extension(&self, ext: &str) -> Option<&dyn LanguageAnalyser> {
        self.extension_map
            .get(ext)
            .map(|&i| self.analysers[i].as_ref())
    }

    /// Get the language name for a file extension.
    pub fn language_for_extension(&self, ext: &str) -> Option<&str> {
        self.get_by_extension(ext).map(|a| a.language_name())
    }

    /// Get all registered extensions.
    pub fn extensions(&self) -> Vec<&str> {
        self.extension_map.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for AnalyserRegistry {
    fn default() -> Self {
        Self::new()
    }
}
