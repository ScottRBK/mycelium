//! VB.NET language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};
use tree_sitter_language::LanguageFn;

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

// Work around mismatched extern symbol in the grammar crate's auto-generated bindings.
// The parser.c exports `tree_sitter_vb_dotnet` but lib.rs declares `tree_sitter_tree_sitter_vb_dotnet`.
#[link(name = "tree-sitter-vb-dotnet", kind = "static")]
extern "C" {
    fn tree_sitter_vb_dotnet() -> *const ();
}

/// The tree-sitter LanguageFn for VB.NET, using the correct symbol name.
const VBNET_LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_vb_dotnet) };

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

fn node_to_symbol_type(node_type: &str) -> Option<SymbolType> {
    match node_type {
        "class_block" => Some(SymbolType::Class),
        "module_block" => Some(SymbolType::Module),
        "structure_block" => Some(SymbolType::Struct),
        "interface_block" => Some(SymbolType::Interface),
        "enum_block" => Some(SymbolType::Enum),
        "namespace_block" => Some(SymbolType::Namespace),
        "method_declaration" => Some(SymbolType::Method),
        "constructor_declaration" => Some(SymbolType::Constructor),
        "property_declaration" => Some(SymbolType::Property),
        "delegate_declaration" => Some(SymbolType::Delegate),
        _ => None,
    }
}

fn is_container(node_type: &str) -> bool {
    matches!(
        node_type,
        "namespace_block"
            | "class_block"
            | "module_block"
            | "structure_block"
            | "interface_block"
    )
}

fn get_visibility(node: &Node, source: &[u8]) -> Visibility {
    if let Some(modifiers) = node.child_by_field_name("modifiers") {
        for i in 0..modifiers.child_count() {
            if let Some(child) = modifiers.child(i) {
                let text = child.utf8_text(source).unwrap_or("");
                match text {
                    "Public" => return Visibility::Public,
                    "Private" => return Visibility::Private,
                    "Friend" => return Visibility::Internal,
                    "Protected" => return Visibility::Protected,
                    _ => {}
                }
            }
        }
    }
    // Also check direct children for modifier-like tokens
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "modifier" || child.kind() == "access_modifier" {
                let text = child.utf8_text(source).unwrap_or("");
                match text {
                    "Public" => return Visibility::Public,
                    "Private" => return Visibility::Private,
                    "Friend" => return Visibility::Internal,
                    "Protected" => return Visibility::Protected,
                    _ => {}
                }
            }
        }
    }
    Visibility::Private // VB.NET default
}

fn get_name(node: &Node, source: &[u8]) -> Option<String> {
    // For constructors, return "New"
    if node.kind() == "constructor_declaration" {
        return Some("New".to_string());
    }
    // Try field name first — the name field could be an identifier or namespace_name
    if let Some(name_node) = node.child_by_field_name("name") {
        return name_node.utf8_text(source).ok().map(|s| s.to_string());
    }
    // Fallback: look for identifier child
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "identifier" {
                return child.utf8_text(source).ok().map(|s| s.to_string());
            }
        }
    }
    None
}

pub struct VbNetAnalyser;

impl Default for VbNetAnalyser {
    fn default() -> Self {
        Self
    }
}

impl VbNetAnalyser {
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

            // type_declaration is a wrapper node — recurse into it to find the actual type
            if child.kind() == "type_declaration" {
                self.walk_node(&child, source, file_path, symbols, parent_id);
                continue;
            }

            let sym_type = match node_to_symbol_type(child.kind()) {
                Some(t) => t,
                None => continue,
            };

            let name = match get_name(&child, source) {
                Some(n) => n,
                None => continue,
            };

            let mut vis = get_visibility(&child, source);
            let mut exported = vis == Visibility::Public;

            // Namespaces have no visibility modifiers
            if sym_type == SymbolType::Namespace {
                vis = Visibility::Unknown;
                exported = true;
            }

            symbols.push(Symbol {
                id: format!("_pending_{}", symbols.len()),
                name: name.clone(),
                symbol_type: sym_type,
                file: file_path.to_string(),
                line: child.start_position().row + 1,
                visibility: vis,
                exported,
                parent: parent_id.map(|s| s.to_string()),
                language: Some("VB.NET".to_string()),
                byte_range: Some((child.byte_range().start, child.byte_range().end)),
                parameter_types: None,
            });

            // Recurse into containers
            if is_container(child.kind()) {
                self.walk_node(&child, source, file_path, symbols, Some(&name));
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
        if node.kind() == "invocation" {
            let (callee_name, qualifier) = extract_callee(node, source);
            if let Some(ref name) = callee_name {
                if !exclusions.contains(name) {
                    let qualified = if let Some(ref q) = qualifier {
                        format!("{}.{}", q, name)
                    } else {
                        name.clone()
                    };
                    if !exclusions.contains(&qualified) {
                        let caller = find_enclosing_method(node, source);
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
}

fn extract_callee(node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
    // Try the "target" field first
    let target = node
        .child_by_field_name("target")
        .or_else(|| node.child(0));

    let target = match target {
        Some(t) => t,
        None => return (None, None),
    };

    match target.kind() {
        "identifier" => {
            let name = target.utf8_text(source).ok().map(|s| s.to_string());
            (name, None)
        }
        "member_access" | "member_access_expression" => {
            let mut parts = Vec::new();
            for i in 0..target.child_count() {
                if let Some(child) = target.child(i) {
                    if child.kind() == "identifier" {
                        if let Ok(text) = child.utf8_text(source) {
                            parts.push(text.to_string());
                        }
                    }
                }
            }
            if parts.len() >= 2 {
                let name = parts.pop();
                let qualifier = parts.pop();
                (name, qualifier)
            } else if parts.len() == 1 {
                (Some(parts.remove(0)), None)
            } else {
                // Fallback: try to split the full text on "."
                let text = target.utf8_text(source).unwrap_or("");
                if let Some(pos) = text.rfind('.') {
                    (
                        Some(text[pos + 1..].to_string()),
                        Some(text[..pos].to_string()),
                    )
                } else {
                    (Some(text.to_string()), None)
                }
            }
        }
        "qualified_name" => {
            let text = target.utf8_text(source).unwrap_or("");
            if let Some(pos) = text.rfind('.') {
                (
                    Some(text[pos + 1..].to_string()),
                    Some(text[..pos].to_string()),
                )
            } else {
                (Some(text.to_string()), None)
            }
        }
        _ => {
            // Fallback: try to get text and split
            let text = target.utf8_text(source).unwrap_or("");
            if text.contains('.') {
                if let Some(pos) = text.rfind('.') {
                    (
                        Some(text[pos + 1..].to_string()),
                        Some(text[..pos].to_string()),
                    )
                } else {
                    (Some(text.to_string()), None)
                }
            } else if !text.is_empty() {
                (Some(text.to_string()), None)
            } else {
                (None, None)
            }
        }
    }
}

fn find_enclosing_method(node: &Node, source: &[u8]) -> Option<String> {
    let mut current = node.parent();
    while let Some(n) = current {
        match n.kind() {
            "method_declaration" | "constructor_declaration" => {
                return get_name(&n, source);
            }
            "property_declaration" => {
                return None;
            }
            _ => {}
        }
        current = n.parent();
    }
    None
}

impl LanguageAnalyser for VbNetAnalyser {
    fn extensions(&self) -> &[&str] {
        &["vb"]
    }

    fn language_name(&self) -> &str {
        "VB.NET"
    }

    fn get_language(&self) -> Language {
        VBNET_LANGUAGE.into()
    }

    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        self.walk_node(&tree.root_node(), source, file_path, &mut symbols, None);
        symbols
    }

    fn extract_imports(
        &self,
        tree: &Tree,
        source: &[u8],
        file_path: &str,
    ) -> Vec<ImportStatement> {
        let mut imports = Vec::new();
        let root = tree.root_node();
        for i in 0..root.child_count() {
            if let Some(child) = root.child(i) {
                if child.kind() == "imports_statement" {
                    // Try the "namespace" field
                    if let Some(ns_node) = child.child_by_field_name("namespace") {
                        if let Ok(target) = ns_node.utf8_text(source) {
                            let statement = child
                                .utf8_text(source)
                                .unwrap_or("")
                                .trim()
                                .to_string();
                            imports.push(ImportStatement {
                                file: file_path.to_string(),
                                statement,
                                target_name: target.to_string(),
                                line: child.start_position().row + 1,
                            });
                        }
                    } else {
                        // Fallback: extract from the full statement text
                        if let Ok(text) = child.utf8_text(source) {
                            let text = text.trim();
                            let target = text
                                .strip_prefix("Imports ")
                                .or_else(|| text.strip_prefix("imports "))
                                .unwrap_or(text)
                                .trim()
                                .to_string();
                            if !target.is_empty() {
                                imports.push(ImportStatement {
                                    file: file_path.to_string(),
                                    statement: text.to_string(),
                                    target_name: target,
                                    line: child.start_position().row + 1,
                                });
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

    fn is_available(&self) -> bool {
        true
    }
}
