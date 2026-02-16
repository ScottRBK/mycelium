//! TypeScript/JavaScript language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "console.log",
        "console.error",
        "console.warn",
        "console.info",
        "console.debug",
        "console.trace",
        "console.dir",
        "JSON.parse",
        "JSON.stringify",
        "parseInt",
        "parseFloat",
        "isNaN",
        "isFinite",
        "encodeURIComponent",
        "decodeURIComponent",
        "setTimeout",
        "setInterval",
        "clearTimeout",
        "clearInterval",
        "Promise.resolve",
        "Promise.reject",
        "Promise.all",
        "Promise.race",
        "Array.isArray",
        "Array.from",
        "Array.of",
        "Object.keys",
        "Object.values",
        "Object.entries",
        "Object.assign",
        "Object.freeze",
        "Object.create",
        "Math.max",
        "Math.min",
        "Math.abs",
        "Math.floor",
        "Math.ceil",
        "Math.round",
        "String.fromCharCode",
        "Number.isInteger",
        "Number.isFinite",
        "require",
        "module.exports",
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
        "function_declaration" => Some(SymbolType::Function),
        "type_alias_declaration" => Some(SymbolType::TypeAlias),
        _ => None,
    }
}

/// Shared analyser for TypeScript, TSX, JavaScript, JSX.
pub struct TypeScriptAnalyser;

impl Default for TypeScriptAnalyser {
    fn default() -> Self {
        Self
    }
}

impl TypeScriptAnalyser {
    pub fn new() -> Self {
        Self
    }

    fn get_ts_language() -> Language {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }

    fn get_tsx_language() -> Language {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }

    fn get_js_language() -> Language {
        tree_sitter_javascript::LANGUAGE.into()
    }

    fn language_for_path(file_path: &str) -> &'static str {
        if file_path.ends_with(".js")
            || file_path.ends_with(".jsx")
            || file_path.ends_with(".mjs")
            || file_path.ends_with(".cjs")
        {
            "JavaScript"
        } else {
            "TypeScript"
        }
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

    fn walk_node(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        parent_id: Option<&str>,
    ) {
        let lang = Self::language_for_path(file_path);

        for i in 0..node.child_count() {
            let child = match node.child(i) {
                Some(c) => c,
                None => continue,
            };

            let mut exported = false;
            let mut decl = child;

            // Check for export_statement wrapper
            if child.kind() == "export_statement" {
                exported = true;
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if node_to_symbol_type(c.kind()).is_some()
                            || c.kind() == "lexical_declaration"
                        {
                            decl = c;
                            break;
                        }
                    }
                }
            }

            if let Some(sym_type) = node_to_symbol_type(decl.kind()) {
                if let Some(name) = Self::get_name(&decl, source) {
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name: name.clone(),
                        symbol_type: sym_type,
                        file: file_path.to_string(),
                        line: decl.start_position().row + 1,
                        visibility: if exported {
                            Visibility::Public
                        } else {
                            Visibility::Private
                        },
                        exported,
                        parent: parent_id.map(|s| s.to_string()),
                        language: Some(lang.to_string()),
                        byte_range: Some((decl.byte_range().start, decl.byte_range().end)),
                        parameter_types: None,
                    });

                    // Extract class members
                    if decl.kind() == "class_declaration" {
                        for j in 0..decl.child_count() {
                            if let Some(c) = decl.child(j) {
                                if c.kind() == "class_body" {
                                    self.extract_class_members(
                                        &c, file_path, source, symbols, &name, lang,
                                    );
                                }
                            }
                        }
                    }
                }
            } else if decl.kind() == "lexical_declaration" {
                // const/let with arrow functions
                for j in 0..decl.child_count() {
                    if let Some(vc) = decl.child(j) {
                        if vc.kind() == "variable_declarator" {
                            let mut vname = None;
                            let mut is_fn = false;
                            for k in 0..vc.child_count() {
                                if let Some(c) = vc.child(k) {
                                    if c.kind() == "identifier" {
                                        vname = c.utf8_text(source).ok().map(|s| s.to_string());
                                    }
                                    if c.kind() == "arrow_function" {
                                        is_fn = true;
                                    }
                                }
                            }
                            if let Some(name) = vname {
                                if is_fn {
                                    symbols.push(Symbol {
                                        id: format!("_pending_{}", symbols.len()),
                                        name,
                                        symbol_type: SymbolType::Function,
                                        file: file_path.to_string(),
                                        line: vc.start_position().row + 1,
                                        visibility: if exported {
                                            Visibility::Public
                                        } else {
                                            Visibility::Private
                                        },
                                        exported,
                                        parent: parent_id.map(|s| s.to_string()),
                                        language: Some(lang.to_string()),
                                        byte_range: Some((
                                            vc.byte_range().start,
                                            vc.byte_range().end,
                                        )),
                                        parameter_types: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_class_members(
        &self,
        body_node: &Node,
        file_path: &str,
        source: &[u8],
        symbols: &mut Vec<Symbol>,
        parent_name: &str,
        lang: &str,
    ) {
        for i in 0..body_node.child_count() {
            if let Some(child) = body_node.child(i) {
                if child.kind() == "method_definition" {
                    let mut name = None;
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "property_identifier" {
                                name = c.utf8_text(source).ok().map(|s| s.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(name) = name {
                        let sym_type = if name == "constructor" {
                            SymbolType::Constructor
                        } else {
                            SymbolType::Method
                        };
                        symbols.push(Symbol {
                            id: format!("_pending_{}", symbols.len()),
                            name,
                            symbol_type: sym_type,
                            file: file_path.to_string(),
                            line: child.start_position().row + 1,
                            visibility: Visibility::Public,
                            exported: true,
                            parent: Some(parent_name.to_string()),
                            language: Some(lang.to_string()),
                            byte_range: Some((child.byte_range().start, child.byte_range().end)),
                            parameter_types: None,
                        });
                    }
                } else if child.kind() == "public_field_definition" {
                    let mut name = None;
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "property_identifier" {
                                name = c.utf8_text(source).ok().map(|s| s.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(name) = name {
                        symbols.push(Symbol {
                            id: format!("_pending_{}", symbols.len()),
                            name,
                            symbol_type: SymbolType::Property,
                            file: file_path.to_string(),
                            line: child.start_position().row + 1,
                            visibility: Visibility::Public,
                            exported: true,
                            parent: Some(parent_name.to_string()),
                            language: Some(lang.to_string()),
                            byte_range: Some((child.byte_range().start, child.byte_range().end)),
                            parameter_types: None,
                        });
                    }
                }
            }
        }
    }

    fn extract_string_source(node: &Node, source: &[u8]) -> Option<String> {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                if child.kind() == "string" {
                    for j in 0..child.child_count() {
                        if let Some(sc) = child.child(j) {
                            if sc.kind() == "string_fragment" {
                                return sc.utf8_text(source).ok().map(|s| s.to_string());
                            }
                        }
                    }
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
        if node.kind() == "call_expression" || node.kind() == "new_expression" {
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

        if first.kind() == "new" {
            // new_expression: skip 'new' keyword
            for i in 1..node.child_count() {
                if let Some(c) = node.child(i) {
                    if c.kind() == "identifier" || c.kind() == "type_identifier" {
                        return (c.utf8_text(source).ok().map(|s| s.to_string()), None);
                    }
                }
            }
            return (None, None);
        }

        if first.kind() == "identifier" || first.kind() == "type_identifier" {
            return (first.utf8_text(source).ok().map(|s| s.to_string()), None);
        }

        if first.kind() == "member_expression" {
            let mut parts = Vec::new();
            for i in 0..first.child_count() {
                if let Some(c) = first.child(i) {
                    if c.kind() == "identifier"
                        || c.kind() == "property_identifier"
                        || c.kind() == "type_identifier"
                    {
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
            if n.kind() == "method_definition" || n.kind() == "function_declaration" {
                for i in 0..n.child_count() {
                    if let Some(c) = n.child(i) {
                        if c.kind() == "identifier" || c.kind() == "property_identifier" {
                            return c.utf8_text(source).ok().map(|s| s.to_string());
                        }
                    }
                }
            }
            if n.kind() == "variable_declarator" {
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

impl LanguageAnalyser for TypeScriptAnalyser {
    fn extensions(&self) -> &[&str] {
        &["ts", "tsx", "js", "jsx"]
    }

    fn language_name(&self) -> &str {
        "TypeScript"
    }

    fn get_language(&self) -> Language {
        Self::get_ts_language()
    }

    fn get_language_for_ext(&self, ext: &str) -> Language {
        match ext {
            "tsx" => Self::get_tsx_language(),
            "js" | "jsx" | "mjs" | "cjs" => Self::get_js_language(),
            _ => Self::get_ts_language(),
        }
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
                    if let Some(source_path) = Self::extract_string_source(&child, source) {
                        let statement = child
                            .utf8_text(source)
                            .unwrap_or("")
                            .trim_end_matches(';')
                            .trim()
                            .to_string();
                        imports.push(ImportStatement {
                            file: file_path.to_string(),
                            statement,
                            target_name: source_path,
                            line: child.start_position().row + 1,
                        });
                    }
                } else if child.kind() == "export_statement" {
                    // Re-exports: export { X } from './module'
                    if let Some(source_path) = Self::extract_string_source(&child, source) {
                        let statement = child
                            .utf8_text(source)
                            .unwrap_or("")
                            .trim_end_matches(';')
                            .trim()
                            .to_string();
                        imports.push(ImportStatement {
                            file: file_path.to_string(),
                            statement,
                            target_name: source_path,
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
