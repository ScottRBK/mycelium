//! Go language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "fmt.Println",
        "fmt.Printf",
        "fmt.Sprintf",
        "fmt.Fprintf",
        "fmt.Errorf",
        "fmt.Print",
        "log.Println",
        "log.Printf",
        "log.Fatal",
        "log.Fatalf",
        "log.Panic",
        "log.Panicf",
        "len",
        "cap",
        "make",
        "new",
        "append",
        "copy",
        "delete",
        "close",
        "panic",
        "recover",
        "print",
        "println",
        "errors.New",
        "errors.Is",
        "errors.As",
        "errors.Unwrap",
        "context.Background",
        "context.TODO",
        "context.WithCancel",
        "context.WithTimeout",
        "context.WithValue",
        "strings.Contains",
        "strings.HasPrefix",
        "strings.HasSuffix",
        "strings.Join",
        "strings.Split",
        "strings.TrimSpace",
        "strconv.Itoa",
        "strconv.Atoi",
        "strconv.FormatInt",
        "sync.Mutex",
        "sync.WaitGroup",
        "time.Now",
        "time.Since",
        "time.Sleep",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

pub struct GoAnalyser;

impl Default for GoAnalyser {
    fn default() -> Self {
        Self
    }
}

impl GoAnalyser {
    pub fn new() -> Self {
        Self
    }

    fn get_name_by_kind(node: &Node, target_kind: &str, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == target_kind {
                    return child.utf8_text(source).ok().map(|s| s.to_string());
                }
            }
        }
        None
    }

    fn is_exported(name: &str) -> bool {
        name.chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
    }

    fn extract_string(node: &Node, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "interpreted_string_literal" {
                    return Self::extract_string_content(&child, source);
                }
            }
        }
        None
    }

    fn extract_string_content(node: &Node, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "interpreted_string_literal_content" {
                    return child.utf8_text(source).ok().map(|s| s.to_string());
                }
            }
        }
        None
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
            let (callee_name, qualifier) = self.extract_callee(node, source);
            if let Some(ref name) = callee_name {
                if !exclusions.contains(name) {
                    let qualified = if let Some(ref q) = qualifier {
                        format!("{}.{}", q, name)
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
        }
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                self.find_calls(&child, source, file_path, calls, exclusions);
            }
        }
    }

    fn extract_callee(&self, node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
        let first = match node.child(0) {
            Some(c) => c,
            None => return (None, None),
        };

        if first.kind() == "identifier" {
            return (first.utf8_text(source).ok().map(|s| s.to_string()), None);
        }

        if first.kind() == "selector_expression" {
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
            if n.kind() == "function_declaration" {
                return Self::get_name_by_kind(&n, "identifier", source);
            }
            if n.kind() == "method_declaration" {
                return Self::get_name_by_kind(&n, "field_identifier", source);
            }
            current = n.parent();
        }
        None
    }
}

impl LanguageAnalyser for GoAnalyser {
    fn extensions(&self) -> &[&str] {
        &["go"]
    }

    fn language_name(&self) -> &str {
        "Go"
    }

    fn get_language(&self) -> Language {
        tree_sitter_go::LANGUAGE.into()
    }

    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let root = tree.root_node();

        for i in 0..root.child_count() {
            let child = match root.child(i) {
                Some(c) => c,
                None => continue,
            };

            if child.kind() == "function_declaration" {
                if let Some(name) = Self::get_name_by_kind(&child, "identifier", source) {
                    let exported = Self::is_exported(&name);
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name,
                        symbol_type: SymbolType::Function,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: if exported {
                            Visibility::Public
                        } else {
                            Visibility::Private
                        },
                        exported,
                        parent: None,
                        language: Some("Go".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });
                }
            } else if child.kind() == "method_declaration" {
                if let Some(name) = Self::get_name_by_kind(&child, "field_identifier", source) {
                    let exported = Self::is_exported(&name);
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name,
                        symbol_type: SymbolType::Method,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: if exported {
                            Visibility::Public
                        } else {
                            Visibility::Private
                        },
                        exported,
                        parent: None,
                        language: Some("Go".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });
                }
            } else if child.kind() == "type_declaration" {
                for j in 0..child.child_count() {
                    if let Some(spec) = child.child(j) {
                        if spec.kind() == "type_spec" {
                            if let Some(name) =
                                Self::get_name_by_kind(&spec, "type_identifier", source)
                            {
                                let mut sym_type = SymbolType::TypeAlias;
                                for k in 0..spec.child_count() {
                                    if let Some(c) = spec.child(k) {
                                        if c.kind() == "struct_type" {
                                            sym_type = SymbolType::Struct;
                                        } else if c.kind() == "interface_type" {
                                            sym_type = SymbolType::Interface;
                                        }
                                    }
                                }
                                let exported = Self::is_exported(&name);
                                symbols.push(Symbol {
                                    id: format!("_pending_{}", symbols.len()),
                                    name,
                                    symbol_type: sym_type,
                                    file: file_path.to_string(),
                                    line: spec.start_position().row + 1,
                                    visibility: if exported {
                                        Visibility::Public
                                    } else {
                                        Visibility::Private
                                    },
                                    exported,
                                    parent: None,
                                    language: Some("Go".to_string()),
                                    byte_range: Some((
                                        spec.byte_range().start,
                                        spec.byte_range().end,
                                    )),
                                    parameter_types: None,
                                });
                            }
                        }
                    }
                }
            } else if child.kind() == "const_declaration" {
                for j in 0..child.child_count() {
                    if let Some(spec) = child.child(j) {
                        if spec.kind() == "const_spec" {
                            if let Some(name) = Self::get_name_by_kind(&spec, "identifier", source)
                            {
                                let exported = Self::is_exported(&name);
                                symbols.push(Symbol {
                                    id: format!("_pending_{}", symbols.len()),
                                    name,
                                    symbol_type: SymbolType::Constant,
                                    file: file_path.to_string(),
                                    line: spec.start_position().row + 1,
                                    visibility: if exported {
                                        Visibility::Public
                                    } else {
                                        Visibility::Private
                                    },
                                    exported,
                                    parent: None,
                                    language: Some("Go".to_string()),
                                    byte_range: Some((
                                        spec.byte_range().start,
                                        spec.byte_range().end,
                                    )),
                                    parameter_types: None,
                                });
                            }
                        }
                    }
                }
            }
        }
        symbols
    }

    fn extract_imports(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement> {
        let mut imports = Vec::new();
        let root = tree.root_node();

        for i in 0..root.child_count() {
            if let Some(child) = root.child(i) {
                if child.kind() == "import_declaration" {
                    for j in 0..child.child_count() {
                        if let Some(spec) = child.child(j) {
                            if spec.kind() == "import_spec" {
                                if let Some(path) = Self::extract_string(&spec, source) {
                                    imports.push(ImportStatement {
                                        file: file_path.to_string(),
                                        statement: format!("import \"{}\"", path),
                                        target_name: path,
                                        line: spec.start_position().row + 1,
                                    });
                                }
                            } else if spec.kind() == "import_spec_list" {
                                for k in 0..spec.child_count() {
                                    if let Some(sub) = spec.child(k) {
                                        if sub.kind() == "import_spec" {
                                            if let Some(path) = Self::extract_string(&sub, source) {
                                                imports.push(ImportStatement {
                                                    file: file_path.to_string(),
                                                    statement: format!("import \"{}\"", path),
                                                    target_name: path,
                                                    line: sub.start_position().row + 1,
                                                });
                                            }
                                        }
                                    }
                                }
                            } else if spec.kind() == "interpreted_string_literal" {
                                if let Some(path) = Self::extract_string_content(&spec, source) {
                                    imports.push(ImportStatement {
                                        file: file_path.to_string(),
                                        statement: format!("import \"{}\"", path),
                                        target_name: path,
                                        line: spec.start_position().row + 1,
                                    });
                                }
                            }
                        }
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
