//! Python language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "print",
        "len",
        "range",
        "enumerate",
        "zip",
        "map",
        "filter",
        "sorted",
        "reversed",
        "list",
        "dict",
        "set",
        "tuple",
        "str",
        "int",
        "float",
        "bool",
        "bytes",
        "type",
        "isinstance",
        "issubclass",
        "getattr",
        "setattr",
        "hasattr",
        "delattr",
        "callable",
        "super",
        "property",
        "staticmethod",
        "classmethod",
        "open",
        "input",
        "format",
        "repr",
        "hash",
        "id",
        "abs",
        "min",
        "max",
        "sum",
        "round",
        "pow",
        "divmod",
        "all",
        "any",
        "iter",
        "next",
        "ord",
        "chr",
        "hex",
        "oct",
        "bin",
        "vars",
        "dir",
        "globals",
        "locals",
        "ValueError",
        "TypeError",
        "KeyError",
        "IndexError",
        "RuntimeError",
        "AttributeError",
        "Exception",
        "logging.getLogger",
        "logging.info",
        "logging.debug",
        "logging.warning",
        "logging.error",
        "logging.critical",
        "os.path.join",
        "os.path.exists",
        "os.path.dirname",
        "json.loads",
        "json.dumps",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

pub struct PythonAnalyser;

impl Default for PythonAnalyser {
    fn default() -> Self {
        Self
    }
}

impl PythonAnalyser {
    pub fn new() -> Self {
        Self
    }

    fn get_name(node: &Node, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "identifier" {
                    return child.utf8_text(source).ok().map(|s| s.to_string());
                }
            }
        }
        None
    }

    fn walk_node(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        parent_id: Option<&str>,
    ) {
        for i in 0..node.child_count() {
            let child = match node.child(i) {
                Some(c) => c,
                None => continue,
            };

            if child.kind() == "class_definition" {
                if let Some(name) = Self::get_name(&child, source) {
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name: name.clone(),
                        symbol_type: SymbolType::Class,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: Visibility::Public,
                        exported: !name.starts_with('_'),
                        parent: parent_id.map(|s| s.to_string()),
                        language: Some("Python".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });

                    // Recurse into class body
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "block" {
                                self.walk_node(&c, source, file_path, symbols, Some(&name));
                            }
                        }
                    }
                }
            } else if child.kind() == "function_definition" {
                if let Some(name) = Self::get_name(&child, source) {
                    let sym_type = if parent_id.is_some() {
                        if name == "__init__" {
                            SymbolType::Constructor
                        } else {
                            SymbolType::Method
                        }
                    } else {
                        SymbolType::Function
                    };

                    let vis = if name.starts_with('_') && !name.starts_with("__") {
                        Visibility::Private
                    } else {
                        Visibility::Public
                    };

                    let exported = !name.starts_with('_');
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name,
                        symbol_type: sym_type,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: vis,
                        exported,
                        parent: parent_id.map(|s| s.to_string()),
                        language: Some("Python".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });
                }
            } else if child.kind() == "decorated_definition" {
                // Decorated class or function — recurse into the decorated_definition
                // which contains the actual class_definition or function_definition
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "class_definition" || c.kind() == "function_definition" {
                            self.walk_node(&child, source, file_path, symbols, parent_id);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn find_calls(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
        calls: &mut Vec<RawCall>,
        exclusions: &HashSet<String>,
    ) {
        if node.kind() == "call" {
            let (callee_name, qualifier) = Self::extract_callee(node, source);
            if let Some(ref name) = callee_name {
                if !exclusions.contains(name) {
                    let qualified = if let Some(ref q) = qualifier {
                        format!("{}.{}", q, name)
                    } else {
                        name.clone()
                    };
                    if !exclusions.contains(&qualified) {
                        let caller = Self::find_enclosing(node, source);
                        calls.push(RawCall {
                            caller_file: file_path.to_string(),
                            caller_name: caller.unwrap_or_else(|| "<module>".to_string()),
                            callee_name: name.clone(),
                            line: node.start_position().row + 1,
                            qualifier,
                        });
                    }
                }
            }
        }
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.find_calls(&child, source, file_path, calls, exclusions);
            }
        }
    }

    fn extract_callee(node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
        let first = match node.child(0) {
            Some(c) => c,
            None => return (None, None),
        };

        if first.kind() == "identifier" {
            return (first.utf8_text(source).ok().map(|s| s.to_string()), None);
        }

        if first.kind() == "attribute" {
            let mut parts = Vec::new();
            for i in 0..first.child_count() {
                if let Some(c) = first.child(i) {
                    if c.kind() == "identifier" {
                        if let Ok(text) = c.utf8_text(source) {
                            parts.push(text.to_string());
                        }
                    }
                }
            }
            if parts.len() >= 2 {
                let name = parts.pop();
                let qualifier = parts.pop();
                return (name, qualifier);
            } else if !parts.is_empty() {
                return (Some(parts.remove(0)), None);
            }
        }

        (None, None)
    }

    fn find_enclosing(node: &Node, source: &[u8]) -> Option<String> {
        let mut current = node.parent();
        while let Some(n) = current {
            if n.kind() == "function_definition" {
                for i in 0..n.child_count() {
                    if let Some(c) = n.child(i) {
                        if c.kind() == "identifier" {
                            return c.utf8_text(source).ok().map(|s| s.to_string());
                        }
                    }
                }
            }
            current = n.parent();
        }
        None
    }
}

impl LanguageAnalyser for PythonAnalyser {
    fn extensions(&self) -> &[&str] {
        &["py"]
    }

    fn language_name(&self) -> &str {
        "Python"
    }

    fn get_language(&self) -> Language {
        tree_sitter_python::LANGUAGE.into()
    }

    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        self.walk_node(&tree.root_node(), source, file_path, &mut symbols, None);
        symbols
    }

    fn extract_imports(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement> {
        let mut imports = Vec::new();
        let root = tree.root_node();
        for i in 0..root.child_count() {
            if let Some(child) = root.child(i) {
                if child.kind() == "import_statement" {
                    // import foo, import foo.bar
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "dotted_name" {
                                if let Ok(target) = c.utf8_text(source) {
                                    imports.push(ImportStatement {
                                        file: file_path.to_string(),
                                        statement: child
                                            .utf8_text(source)
                                            .unwrap_or("")
                                            .to_string(),
                                        target_name: target.to_string(),
                                        line: child.start_position().row + 1,
                                    });
                                }
                            }
                        }
                    }
                } else if child.kind() == "import_from_statement" {
                    // from foo import bar
                    let mut module = None;
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "dotted_name" || c.kind() == "relative_import" {
                                module = c.utf8_text(source).ok().map(|s| s.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(module) = module {
                        imports.push(ImportStatement {
                            file: file_path.to_string(),
                            statement: child.utf8_text(source).unwrap_or("").to_string(),
                            target_name: module,
                            line: child.start_position().row + 1,
                        });
                    }
                }
            }
        }
        imports
    }

    fn extract_calls(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<RawCall> {
        let mut calls = Vec::new();
        let exclusions = self.builtin_exclusions();
        self.find_calls(&tree.root_node(), source, file_path, &mut calls, exclusions);
        calls
    }

    fn builtin_exclusions(&self) -> &HashSet<String> {
        &BUILTIN_EXCLUSIONS
    }
}
