//! Rust language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "println!",
        "print!",
        "eprintln!",
        "eprint!",
        "format!",
        "write!",
        "writeln!",
        "vec!",
        "todo!",
        "unimplemented!",
        "unreachable!",
        "panic!",
        "assert!",
        "assert_eq!",
        "assert_ne!",
        "debug_assert!",
        "debug_assert_eq!",
        "debug_assert_ne!",
        "dbg!",
        "cfg!",
        "env!",
        "include!",
        "include_str!",
        "include_bytes!",
        // Also without bang (call_expression vs macro_invocation)
        "println",
        "eprintln",
        "format",
        "vec",
        "dbg",
        "assert",
        "assert_eq",
        "assert_ne",
        "todo",
        "unimplemented",
        "panic",
        "unreachable",
        "write",
        "writeln",
        "String::new",
        "String::from",
        "String::with_capacity",
        "Vec::new",
        "Vec::with_capacity",
        "HashMap::new",
        "HashSet::new",
        "BTreeMap::new",
        "BTreeSet::new",
        "Box::new",
        "Rc::new",
        "Arc::new",
        "Cell::new",
        "RefCell::new",
        "Mutex::new",
        "RwLock::new",
        "Option::Some",
        "Option::None",
        "Result::Ok",
        "Result::Err",
        "Ok",
        "Err",
        "Some",
        "None",
        "Clone::clone",
        "Default::default",
        "Drop::drop",
        "Iterator::next",
        "IntoIterator::into_iter",
        "Display::fmt",
        "Debug::fmt",
        "From::from",
        "Into::into",
        "TryFrom::try_from",
        "TryInto::try_into",
        "AsRef::as_ref",
        "AsMut::as_mut",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

fn node_to_symbol_type(node_type: &str) -> Option<SymbolType> {
    match node_type {
        "function_item" => Some(SymbolType::Function),
        "struct_item" => Some(SymbolType::Struct),
        "enum_item" => Some(SymbolType::Enum),
        "trait_item" => Some(SymbolType::Trait),
        "impl_item" => Some(SymbolType::Impl),
        "type_item" => Some(SymbolType::TypeAlias),
        "const_item" => Some(SymbolType::Constant),
        "static_item" => Some(SymbolType::Static),
        "mod_item" => Some(SymbolType::Module),
        "macro_definition" => Some(SymbolType::Macro),
        _ => None,
    }
}

pub struct RustAnalyser;

impl Default for RustAnalyser {
    fn default() -> Self {
        Self
    }
}

impl RustAnalyser {
    pub fn new() -> Self {
        Self
    }

    fn get_name(node: &Node, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "identifier" || child.kind() == "type_identifier" {
                    return child.utf8_text(source).ok().map(|s| s.to_string());
                }
            }
        }
        None
    }

    fn is_pub(node: &Node) -> bool {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "visibility_modifier" {
                    return true;
                }
            }
        }
        false
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

            let sym_type = match node_to_symbol_type(child.kind()) {
                Some(t) => t,
                None => continue,
            };

            let name = match Self::get_name(&child, source) {
                Some(n) => n,
                None => continue,
            };

            let is_pub = Self::is_pub(&child);

            symbols.push(Symbol {
                id: format!("_pending_{}", symbols.len()),
                name: name.clone(),
                symbol_type: sym_type,
                file: file_path.to_string(),
                line: child.start_position().row + 1,
                visibility: if is_pub {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                exported: is_pub,
                parent: parent_id.map(|s| s.to_string()),
                language: Some("Rust".to_string()),
                byte_range: Some((child.byte_range().start, child.byte_range().end)),
                parameter_types: None,
            });

            // Recurse into impl blocks and mod blocks
            if child.kind() == "impl_item" || child.kind() == "mod_item" {
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "declaration_list" {
                            self.walk_node(&c, source, file_path, symbols, Some(&name));
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
        if node.kind() == "call_expression" {
            let (callee_name, qualifier) = Self::extract_callee(node, source);
            if let Some(ref name) = callee_name {
                if !exclusions.contains(name) {
                    let qualified = if let Some(ref q) = qualifier {
                        format!("{}::{}", q, name)
                    } else {
                        name.clone()
                    };
                    if !exclusions.contains(&qualified) {
                        let caller = self.find_enclosing(node, source);
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
        } else if node.kind() == "macro_invocation" {
            // macro_invocation: identifier ! token_tree
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" {
                        if let Ok(name) = child.utf8_text(source) {
                            let name = name.to_string();
                            let with_bang = format!("{}!", name);
                            if !exclusions.contains(&name) && !exclusions.contains(&with_bang) {
                                let caller = self.find_enclosing(node, source);
                                calls.push(RawCall {
                                    caller_file: file_path.to_string(),
                                    caller_name: caller.unwrap_or_else(|| "<module>".to_string()),
                                    callee_name: name,
                                    line: node.start_position().row + 1,
                                    qualifier: None,
                                });
                            }
                        }
                        break;
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

        if first.kind() == "scoped_identifier" {
            let mut parts = Vec::new();
            for i in 0..first.child_count() {
                if let Some(c) = first.child(i) {
                    if c.kind() == "identifier" || c.kind() == "type_identifier" {
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

        if first.kind() == "field_expression" {
            let mut parts = Vec::new();
            for i in 0..first.child_count() {
                if let Some(c) = first.child(i) {
                    if c.kind() == "identifier" || c.kind() == "field_identifier" {
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

    fn find_enclosing(&self, node: &Node, source: &[u8]) -> Option<String> {
        let mut current = node.parent();
        while let Some(n) = current {
            if n.kind() == "function_item" {
                return Self::get_name(&n, source);
            }
            current = n.parent();
        }
        None
    }
}

impl LanguageAnalyser for RustAnalyser {
    fn extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn language_name(&self) -> &str {
        "Rust"
    }

    fn get_language(&self) -> Language {
        tree_sitter_rust::LANGUAGE.into()
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
                if child.kind() == "use_declaration" {
                    let mut path = None;
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "scoped_identifier"
                                || c.kind() == "identifier"
                                || c.kind() == "use_wildcard"
                                || c.kind() == "scoped_use_list"
                            {
                                path = c.utf8_text(source).ok().map(|s| s.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(path) = path {
                        imports.push(ImportStatement {
                            file: file_path.to_string(),
                            statement: child
                                .utf8_text(source)
                                .unwrap_or("")
                                .trim_end_matches(';')
                                .trim()
                                .to_string(),
                            target_name: path,
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
