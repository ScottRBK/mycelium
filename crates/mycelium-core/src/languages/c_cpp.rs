//! C and C++ language analysers (shared implementation).

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static C_BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        "printf", "fprintf", "sprintf", "snprintf", "scanf", "fscanf", "sscanf", "malloc",
        "calloc", "realloc", "free", "memcpy", "memmove", "memset", "memcmp", "strlen", "strcpy",
        "strncpy", "strcat", "strncat", "strcmp", "strncmp", "fopen", "fclose", "fread", "fwrite",
        "fgets", "fputs", "exit", "abort", "atexit", "assert", "sizeof", "offsetof",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

static CPP_BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = C_BUILTIN_EXCLUSIONS.clone();
    for name in [
        "std::cout",
        "std::cerr",
        "std::endl",
        "std::make_shared",
        "std::make_unique",
        "std::make_pair",
        "std::move",
        "std::forward",
        "std::swap",
        "std::sort",
        "std::find",
        "std::transform",
        "std::begin",
        "std::end",
        "std::string",
        "std::to_string",
        "std::stoi",
        "std::stof",
        "std::vector",
        "std::map",
        "std::set",
        "std::unordered_map",
        "static_cast",
        "dynamic_cast",
        "reinterpret_cast",
        "const_cast",
        "new",
        "delete",
    ] {
        set.insert(name.to_string());
    }
    set
});

const PREPROC_CONTAINERS: &[&str] = &[
    "preproc_ifdef",
    "preproc_ifndef",
    "preproc_if",
    "preproc_else",
    "preproc_elif",
];

fn is_preproc_container(kind: &str) -> bool {
    PREPROC_CONTAINERS.contains(&kind)
}

// ---- Shared C/C++ helpers ----

fn get_func_name(node: &Node, source: &[u8]) -> Option<String> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "function_declarator" {
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "identifier" {
                            return c.utf8_text(source).ok().map(|s| s.to_string());
                        }
                    }
                }
            }
            if child.kind() == "pointer_declarator" {
                let result = get_func_name(&child, source);
                if result.is_some() {
                    return result;
                }
            }
            if child.kind() == "identifier" {
                return child.utf8_text(source).ok().map(|s| s.to_string());
            }
        }
    }
    None
}

fn get_type_name(node: &Node, source: &[u8]) -> Option<String> {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "type_identifier" {
                return child.utf8_text(source).ok().map(|s| s.to_string());
            }
        }
    }
    None
}

fn extract_c_symbols(
    node: &Node,
    source: &[u8],
    file_path: &str,
    symbols: &mut Vec<Symbol>,
    parent_id: Option<&str>,
    lang: &str,
) {
    for i in 0..node.child_count() {
        let child = match node.child(i) {
            Some(c) => c,
            None => continue,
        };

        if child.kind() == "function_definition" {
            if let Some(name) = get_func_name(&child, source) {
                symbols.push(Symbol {
                    id: format!("_pending_{}", symbols.len()),
                    name,
                    symbol_type: SymbolType::Function,
                    file: file_path.to_string(),
                    line: child.start_position().row + 1,
                    visibility: Visibility::Public,
                    exported: true,
                    parent: parent_id.map(|s| s.to_string()),
                    language: Some(lang.to_string()),
                    byte_range: Some((child.byte_range().start, child.byte_range().end)),
                    parameter_types: None,
                });
            }
        } else if child.kind() == "struct_specifier" {
            if let Some(name) = get_type_name(&child, source) {
                symbols.push(Symbol {
                    id: format!("_pending_{}", symbols.len()),
                    name,
                    symbol_type: SymbolType::Struct,
                    file: file_path.to_string(),
                    line: child.start_position().row + 1,
                    visibility: Visibility::Public,
                    exported: true,
                    parent: parent_id.map(|s| s.to_string()),
                    language: Some(lang.to_string()),
                    byte_range: Some((child.byte_range().start, child.byte_range().end)),
                    parameter_types: None,
                });
            }
        } else if child.kind() == "enum_specifier" {
            if let Some(name) = get_type_name(&child, source) {
                symbols.push(Symbol {
                    id: format!("_pending_{}", symbols.len()),
                    name,
                    symbol_type: SymbolType::Enum,
                    file: file_path.to_string(),
                    line: child.start_position().row + 1,
                    visibility: Visibility::Public,
                    exported: true,
                    parent: parent_id.map(|s| s.to_string()),
                    language: Some(lang.to_string()),
                    byte_range: Some((child.byte_range().start, child.byte_range().end)),
                    parameter_types: None,
                });
            }
        } else if child.kind() == "type_definition" {
            // typedef
            if let Some(name) = get_type_name(&child, source) {
                symbols.push(Symbol {
                    id: format!("_pending_{}", symbols.len()),
                    name,
                    symbol_type: SymbolType::Typedef,
                    file: file_path.to_string(),
                    line: child.start_position().row + 1,
                    visibility: Visibility::Public,
                    exported: true,
                    parent: parent_id.map(|s| s.to_string()),
                    language: Some(lang.to_string()),
                    byte_range: Some((child.byte_range().start, child.byte_range().end)),
                    parameter_types: None,
                });
            }
        } else if child.kind() == "declaration" {
            // Forward declarations of functions
            if let Some(name) = get_func_name(&child, source) {
                symbols.push(Symbol {
                    id: format!("_pending_{}", symbols.len()),
                    name,
                    symbol_type: SymbolType::Function,
                    file: file_path.to_string(),
                    line: child.start_position().row + 1,
                    visibility: Visibility::Public,
                    exported: true,
                    parent: parent_id.map(|s| s.to_string()),
                    language: Some(lang.to_string()),
                    byte_range: Some((child.byte_range().start, child.byte_range().end)),
                    parameter_types: None,
                });
            }
        } else if is_preproc_container(child.kind()) {
            extract_c_symbols(&child, source, file_path, symbols, parent_id, lang);
        }
    }
}

fn extract_includes(tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement> {
    let mut imports = Vec::new();
    let root = tree.root_node();
    for i in 0..root.child_count() {
        if let Some(child) = root.child(i) {
            if child.kind() == "preproc_include" {
                let mut path = None;
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "string_literal" {
                            for k in 0..c.child_count() {
                                if let Some(sc) = c.child(k) {
                                    if sc.kind() == "string_content" {
                                        path = sc.utf8_text(source).ok().map(|s| s.to_string());
                                    }
                                }
                            }
                        } else if c.kind() == "system_lib_string" {
                            if let Ok(text) = c.utf8_text(source) {
                                path =
                                    Some(text.trim_matches(|c| c == '<' || c == '>').to_string());
                            }
                        }
                    }
                }
                if let Some(path) = path {
                    imports.push(ImportStatement {
                        file: file_path.to_string(),
                        statement: child.utf8_text(source).unwrap_or("").trim().to_string(),
                        target_name: path,
                        line: child.start_position().row + 1,
                    });
                }
            }
        }
    }
    imports
}

fn find_c_calls(
    node: &Node,
    source: &[u8],
    file_path: &str,
    calls: &mut Vec<RawCall>,
    exclusions: &HashSet<String>,
) {
    if node.kind() == "call_expression" {
        let (callee_name, qualifier) = extract_c_callee(node, source);
        if let Some(ref name) = callee_name {
            if !exclusions.contains(name) {
                let qualified = if let Some(ref q) = qualifier {
                    format!("{}.{}", q, name)
                } else {
                    name.clone()
                };
                if !exclusions.contains(&qualified) {
                    let caller = find_enclosing_func(node, source);
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
            find_c_calls(&child, source, file_path, calls, exclusions);
        }
    }
}

fn extract_c_callee(node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
    let first = match node.child(0) {
        Some(c) => c,
        None => return (None, None),
    };

    if first.kind() == "identifier" {
        return (first.utf8_text(source).ok().map(|s| s.to_string()), None);
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

    if first.kind() == "qualified_identifier" {
        if let Ok(text) = first.utf8_text(source) {
            let text = text.to_string();
            if let Some(pos) = text.rfind("::") {
                return (
                    Some(text[pos + 2..].to_string()),
                    Some(text[..pos].to_string()),
                );
            }
            return (Some(text), None);
        }
    }

    (None, None)
}

fn find_enclosing_func(node: &Node, source: &[u8]) -> Option<String> {
    let mut current = node.parent();
    while let Some(n) = current {
        if n.kind() == "function_definition" {
            return get_func_name(&n, source);
        }
        current = n.parent();
    }
    None
}

// ---- C Analyser ----

/// C language analyser.
pub struct CAnalyser;

impl Default for CAnalyser {
    fn default() -> Self {
        Self
    }
}

impl CAnalyser {
    pub fn new() -> Self {
        Self
    }
}

impl LanguageAnalyser for CAnalyser {
    fn extensions(&self) -> &[&str] {
        &["c", "h"]
    }

    fn language_name(&self) -> &str {
        "C"
    }

    fn get_language(&self) -> Language {
        tree_sitter_c::LANGUAGE.into()
    }

    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        extract_c_symbols(
            &tree.root_node(),
            source,
            file_path,
            &mut symbols,
            None,
            "C",
        );
        symbols
    }

    fn extract_imports(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement> {
        extract_includes(tree, source, file_path)
    }

    fn extract_calls(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<RawCall> {
        let mut calls = Vec::new();
        let exclusions = self.builtin_exclusions();
        find_c_calls(&tree.root_node(), source, file_path, &mut calls, exclusions);
        calls
    }

    fn builtin_exclusions(&self) -> &HashSet<String> {
        &C_BUILTIN_EXCLUSIONS
    }
}

// ---- C++ Analyser ----

/// C++ language analyser.
pub struct CppAnalyser;

impl Default for CppAnalyser {
    fn default() -> Self {
        Self
    }
}

impl CppAnalyser {
    pub fn new() -> Self {
        Self
    }

    fn extract_cpp_symbols(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
        symbols: &mut Vec<Symbol>,
        parent_id: Option<&str>,
    ) {
        // Extract base C symbols (functions, structs, enums, typedefs)
        extract_c_symbols(node, source, file_path, symbols, parent_id, "C++");

        // C++-specific: classes and namespaces
        for i in 0..node.child_count() {
            let child = match node.child(i) {
                Some(c) => c,
                None => continue,
            };

            if child.kind() == "class_specifier" {
                if let Some(name) = get_type_name(&child, source) {
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name,
                        symbol_type: SymbolType::Class,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: Visibility::Public,
                        exported: true,
                        parent: parent_id.map(|s| s.to_string()),
                        language: Some("C++".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });
                }
            } else if child.kind() == "namespace_definition" {
                let mut name = None;
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "namespace_identifier" {
                            name = c.utf8_text(source).ok().map(|s| s.to_string());
                            break;
                        }
                    }
                }
                if let Some(ref ns_name) = name {
                    symbols.push(Symbol {
                        id: format!("_pending_{}", symbols.len()),
                        name: ns_name.clone(),
                        symbol_type: SymbolType::Namespace,
                        file: file_path.to_string(),
                        line: child.start_position().row + 1,
                        visibility: Visibility::Public,
                        exported: true,
                        parent: parent_id.map(|s| s.to_string()),
                        language: Some("C++".to_string()),
                        byte_range: Some((child.byte_range().start, child.byte_range().end)),
                        parameter_types: None,
                    });
                    // Recurse into namespace body
                    for j in 0..child.child_count() {
                        if let Some(c) = child.child(j) {
                            if c.kind() == "declaration_list" {
                                self.extract_cpp_symbols(
                                    &c,
                                    source,
                                    file_path,
                                    symbols,
                                    Some(ns_name),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

impl LanguageAnalyser for CppAnalyser {
    fn extensions(&self) -> &[&str] {
        &["cpp", "cxx", "cc", "hpp", "hxx", "hh"]
    }

    fn language_name(&self) -> &str {
        "C++"
    }

    fn get_language(&self) -> Language {
        tree_sitter_cpp::LANGUAGE.into()
    }

    fn extract_symbols(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        self.extract_cpp_symbols(&tree.root_node(), source, file_path, &mut symbols, None);
        symbols
    }

    fn extract_imports(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<ImportStatement> {
        extract_includes(tree, source, file_path)
    }

    fn extract_calls(&self, tree: &Tree, source: &[u8], file_path: &str) -> Vec<RawCall> {
        let mut calls = Vec::new();
        let exclusions = self.builtin_exclusions();
        find_c_calls(&tree.root_node(), source, file_path, &mut calls, exclusions);
        calls
    }

    fn builtin_exclusions(&self) -> &HashSet<String> {
        &CPP_BUILTIN_EXCLUSIONS
    }
}
