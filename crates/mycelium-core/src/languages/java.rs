//! Java language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "System.out.println",
        "System.out.print",
        "System.err.println",
        "System.out.printf",
        "System.exit",
        "System.currentTimeMillis",
        "System.nanoTime",
        "System.arraycopy",
        "System.getenv",
        "System.getProperty",
        "String.valueOf",
        "String.format",
        "String.join",
        "Integer.parseInt",
        "Integer.valueOf",
        "Integer.toString",
        "Long.parseLong",
        "Double.parseDouble",
        "Boolean.parseBoolean",
        "Math.max",
        "Math.min",
        "Math.abs",
        "Math.sqrt",
        "Math.round",
        "Arrays.asList",
        "Arrays.sort",
        "Arrays.copyOf",
        "Collections.sort",
        "Collections.unmodifiableList",
        "Collections.emptyList",
        "Collections.singletonList",
        "Objects.requireNonNull",
        "Objects.equals",
        "Objects.hash",
        "Optional.of",
        "Optional.ofNullable",
        "Optional.empty",
        "Thread.sleep",
        "Thread.currentThread",
        "Logger.getLogger",
        "toString",
        "equals",
        "hashCode",
        "getClass",
        "println",
        "printf",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

fn node_to_symbol_type(node_type: &str) -> Option<SymbolType> {
    match node_type {
        "class_declaration" => Some(SymbolType::Class),
        "interface_declaration" => Some(SymbolType::Interface),
        "enum_declaration" => Some(SymbolType::Enum),
        "method_declaration" => Some(SymbolType::Method),
        "constructor_declaration" => Some(SymbolType::Constructor),
        "record_declaration" => Some(SymbolType::Record),
        "annotation_type_declaration" => Some(SymbolType::Annotation),
        _ => None,
    }
}

fn is_container(node_type: &str) -> bool {
    matches!(
        node_type,
        "class_declaration" | "interface_declaration" | "enum_declaration"
    )
}

fn get_visibility(node: &Node, source: &[u8]) -> Visibility {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "modifiers" {
                for j in 0..child.child_count() {
                    if let Some(m) = child.child(j) {
                        if m.child_count() == 0 {
                            let text = m.utf8_text(source).unwrap_or("").to_lowercase();
                            match text.as_str() {
                                "public" => return Visibility::Public,
                                "private" => return Visibility::Private,
                                "protected" => return Visibility::Protected,
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    Visibility::Internal // Java default is package-private
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

pub struct JavaAnalyser;

impl Default for JavaAnalyser {
    fn default() -> Self {
        Self
    }
}

impl JavaAnalyser {
    pub fn new() -> Self {
        Self
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

            let name = match get_name(&child, source) {
                Some(n) => n,
                None => continue,
            };

            let vis = get_visibility(&child, source);
            let exported = vis == Visibility::Public;

            symbols.push(Symbol {
                id: format!("_pending_{}", symbols.len()),
                name: name.clone(),
                symbol_type: sym_type,
                file: file_path.to_string(),
                line: child.start_position().row + 1,
                visibility: vis,
                exported,
                parent: parent_id.map(|s| s.to_string()),
                language: Some("Java".to_string()),
                byte_range: Some((child.byte_range().start, child.byte_range().end)),
                parameter_types: None,
            });

            // Recurse into container bodies
            if is_container(child.kind()) {
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "class_body"
                            || c.kind() == "interface_body"
                            || c.kind() == "enum_body"
                        {
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
        if node.kind() == "method_invocation" {
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
        } else if node.kind() == "object_creation_expression" {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" || child.kind() == "type_identifier" {
                        if let Ok(name) = child.utf8_text(source) {
                            let name = name.to_string();
                            if !exclusions.contains(&name) {
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

    fn extract_callee(&self, node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
        // method_invocation children: [object, '.', method_name, argument_list]
        // or just [method_name, argument_list] for unqualified calls
        let has_dot = (0..node.child_count())
            .any(|i| node.child(i).map(|c| c.kind() == ".").unwrap_or(false));

        if has_dot {
            let mut parts = Vec::new();
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" || child.kind() == "field_access" {
                        if let Ok(text) = child.utf8_text(source) {
                            parts.push(text.to_string());
                        }
                    }
                }
            }
            if parts.len() >= 2 {
                let name = parts.last().cloned();
                let qualifier = parts.get(parts.len() - 2).cloned();
                return (name, qualifier);
            } else if !parts.is_empty() {
                return (Some(parts.remove(0)), None);
            }
        } else {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" {
                        return (child.utf8_text(source).ok().map(|s| s.to_string()), None);
                    }
                }
            }
        }
        (None, None)
    }

    fn find_enclosing(&self, node: &Node, source: &[u8]) -> Option<String> {
        let mut current = node.parent();
        while let Some(n) = current {
            if n.kind() == "method_declaration" || n.kind() == "constructor_declaration" {
                return get_name(&n, source);
            }
            current = n.parent();
        }
        None
    }
}

impl LanguageAnalyser for JavaAnalyser {
    fn extensions(&self) -> &[&str] {
        &["java"]
    }

    fn language_name(&self) -> &str {
        "Java"
    }

    fn get_language(&self) -> Language {
        tree_sitter_java::LANGUAGE.into()
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
                if child.kind() == "import_declaration" {
                    let mut target = None;
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "scoped_identifier" {
                                target = c.utf8_text(source).ok().map(|s| s.to_string());
                            }
                        }
                    }
                    if let Some(target) = target {
                        imports.push(ImportStatement {
                            file: file_path.to_string(),
                            statement: child
                                .utf8_text(source)
                                .unwrap_or("")
                                .trim_end_matches(';')
                                .trim()
                                .to_string(),
                            target_name: target,
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
