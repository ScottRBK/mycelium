//! C# language analyser.

use std::collections::HashSet;
use std::sync::LazyLock;

use tree_sitter::{Language, Node, Tree};

use super::LanguageAnalyser;
use crate::config::{ImportStatement, RawCall, Symbol, SymbolType, Visibility};

static BUILTIN_EXCLUSIONS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    [
        // Framework types
        "Task",
        "ValueTask",
        // Console
        "Console.WriteLine",
        "Console.ReadLine",
        "Console.Write",
        "Console.ReadKey",
        "Console.Clear",
        // String
        "String.Format",
        "String.IsNullOrEmpty",
        "String.IsNullOrWhiteSpace",
        "String.Join",
        "String.Concat",
        "String.Compare",
        "string.Format",
        "string.IsNullOrEmpty",
        "string.IsNullOrWhiteSpace",
        "string.Join",
        "string.Concat",
        "string.Compare",
        // Convert
        "Convert.ToInt32",
        "Convert.ToString",
        "Convert.ToDecimal",
        "Convert.ToDouble",
        "Convert.ToBoolean",
        "Convert.ToDateTime",
        // Math
        "Math.Abs",
        "Math.Max",
        "Math.Min",
        "Math.Round",
        "Math.Floor",
        "Math.Ceiling",
        "Math.Pow",
        "Math.Sqrt",
        // Object
        "ToString",
        "Equals",
        "GetHashCode",
        "GetType",
        "ReferenceEquals",
        "MemberwiseClone",
        "Finalize",
        // Debug/Trace
        "Debug.WriteLine",
        "Debug.Write",
        "Debug.Assert",
        "Debug.Print",
        "Trace.WriteLine",
        "Trace.TraceInformation",
        // GC
        "GC.Collect",
        "GC.SuppressFinalize",
        // Task
        "Task.Run",
        "Task.WhenAll",
        "Task.WhenAny",
        "Task.Delay",
        "Task.FromResult",
        "Task.CompletedTask",
        "ValueTask.FromResult",
        "ValueTask.CompletedTask",
        // Parsing
        "int.Parse",
        "int.TryParse",
        "Guid.NewGuid",
        "Guid.Parse",
        "Guid.TryParse",
        // Keywords
        "nameof",
        "typeof",
        "sizeof",
        "default",
        "ArgumentNullException.ThrowIfNull",
        // LINQ
        "Select",
        "Where",
        "FirstOrDefault",
        "First",
        "Last",
        "LastOrDefault",
        "SingleOrDefault",
        "Single",
        "Any",
        "All",
        "Count",
        "Sum",
        "Average",
        "Min",
        "Max",
        "OrderBy",
        "OrderByDescending",
        "GroupBy",
        "ToList",
        "ToArray",
        "ToDictionary",
        "AsEnumerable",
        "AsQueryable",
        "Skip",
        "Take",
        "Distinct",
        "Union",
        "Intersect",
        "Except",
        "Aggregate",
        "Zip",
        "SelectMany",
        "Contains",
        // Common framework
        "Dispose",
        "Close",
    ]
    .into_iter()
    .map(String::from)
    .collect()
});

fn node_to_symbol_type(node_type: &str) -> Option<SymbolType> {
    match node_type {
        "class_declaration" => Some(SymbolType::Class),
        "interface_declaration" => Some(SymbolType::Interface),
        "struct_declaration" => Some(SymbolType::Struct),
        "enum_declaration" => Some(SymbolType::Enum),
        "namespace_declaration" | "file_scoped_namespace_declaration" => {
            Some(SymbolType::Namespace)
        }
        "record_declaration" => Some(SymbolType::Record),
        "delegate_declaration" => Some(SymbolType::Delegate),
        "method_declaration" => Some(SymbolType::Method),
        "constructor_declaration" => Some(SymbolType::Constructor),
        "property_declaration" => Some(SymbolType::Property),
        _ => None,
    }
}

fn is_container(node_type: &str) -> bool {
    matches!(
        node_type,
        "class_declaration"
            | "struct_declaration"
            | "interface_declaration"
            | "record_declaration"
            | "namespace_declaration"
            | "file_scoped_namespace_declaration"
    )
}

fn get_visibility(node: &Node, source: &[u8]) -> Visibility {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "modifier" {
                let mod_text = child.utf8_text(source).unwrap_or("").to_lowercase();
                match mod_text.as_str() {
                    "public" => return Visibility::Public,
                    "private" => return Visibility::Private,
                    "internal" => return Visibility::Internal,
                    "protected" => return Visibility::Protected,
                    _ => {}
                }
            }
        }
    }
    Visibility::Private // C# default
}

fn get_name(node: &Node, source: &[u8]) -> Option<String> {
    if let Some(name_node) = node.child_by_field_name("name") {
        return name_node.utf8_text(source).ok().map(|s| s.to_string());
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "identifier" {
                return child.utf8_text(source).ok().map(|s| s.to_string());
            }
        }
    }
    None
}

pub struct CSharpAnalyser;

impl Default for CSharpAnalyser {
    fn default() -> Self {
        Self
    }
}

impl CSharpAnalyser {
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

            let mut vis = get_visibility(&child, source);
            let mut exported = vis == Visibility::Public;

            // Namespaces have no visibility modifiers
            if sym_type == SymbolType::Namespace {
                vis = Visibility::Unknown;
                exported = true;
            }

            // Extract parameter types for constructors (DI tracking)
            let parameter_types = if child.kind() == "constructor_declaration" {
                extract_parameter_types(&child, source)
            } else {
                None
            };

            symbols.push(Symbol {
                id: format!("_pending_{}", symbols.len()),
                name: name.clone(),
                symbol_type: sym_type,
                file: file_path.to_string(),
                line: child.start_position().row + 1,
                visibility: vis,
                exported,
                parent: parent_id.map(|s| s.to_string()),
                language: Some("C#".to_string()),
                byte_range: Some((child.byte_range().start, child.byte_range().end)),
                parameter_types,
            });

            // Recurse into containers
            if is_container(child.kind()) {
                for j in 0..child.child_count() {
                    if let Some(c) = child.child(j) {
                        if c.kind() == "declaration_list" {
                            self.walk_node(&c, source, file_path, symbols, Some(&name));
                            break;
                        }
                    }
                }
            }
        }
    }

    fn extract_using(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
    ) -> Option<ImportStatement> {
        let mut name_node = None;
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                match child.kind() {
                    "identifier" | "qualified_name" | "name" => {
                        name_node = Some(child);
                        break;
                    }
                    _ => {}
                }
            }
        }
        let name_node = name_node?;
        let target = name_node.utf8_text(source).ok()?.to_string();
        let statement = node
            .utf8_text(source)
            .ok()?
            .trim_end_matches(';')
            .trim()
            .to_string();
        Some(ImportStatement {
            file: file_path.to_string(),
            statement,
            target_name: target,
            line: node.start_position().row + 1,
        })
    }

    fn find_calls(
        &self,
        node: &Node,
        source: &[u8],
        file_path: &str,
        calls: &mut Vec<RawCall>,
        exclusions: &HashSet<String>,
    ) {
        if node.kind() == "invocation_expression" {
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
        } else if node.kind() == "object_creation_expression" {
            let mut callee_name = None;
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "identifier" || child.kind() == "qualified_name" {
                        callee_name = child.utf8_text(source).ok().map(|s| s.to_string());
                        break;
                    }
                }
            }
            if let Some(ref name) = callee_name {
                if !exclusions.contains(name) {
                    let caller = find_enclosing_method(node, source);
                    calls.push(RawCall {
                        caller_file: file_path.to_string(),
                        caller_name: caller.unwrap_or_else(|| "<module>".to_string()),
                        callee_name: name.clone(),
                        line: node.start_position().row + 1,
                        qualifier: None,
                    });
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

fn extract_parameter_types(node: &Node, source: &[u8]) -> Option<Vec<(String, String)>> {
    let param_list = node.child_by_field_name("parameters")?;
    let mut params = Vec::new();
    for i in 0..param_list.child_count() {
        if let Some(child) = param_list.child(i) {
            if child.kind() == "parameter" {
                let type_node = child.child_by_field_name("type");
                let name_node = child.child_by_field_name("name");
                if let (Some(tn), Some(nn)) = (type_node, name_node) {
                    if let (Ok(type_name), Ok(param_name)) =
                        (tn.utf8_text(source), nn.utf8_text(source))
                    {
                        params.push((param_name.to_string(), type_name.to_string()));
                    }
                }
            }
        }
    }
    if params.is_empty() {
        None
    } else {
        Some(params)
    }
}

fn extract_callee(inv_node: &Node, source: &[u8]) -> (Option<String>, Option<String>) {
    let first_child = match inv_node.child(0) {
        Some(c) => c,
        None => return (None, None),
    };

    match first_child.kind() {
        "identifier" => {
            let name = first_child.utf8_text(source).ok().map(|s| s.to_string());
            (name, None)
        }
        "member_access_expression" => {
            let mut parts = Vec::new();
            for i in 0..first_child.child_count() {
                if let Some(child) = first_child.child(i) {
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
                (None, None)
            }
        }
        "qualified_name" => {
            let text = first_child.utf8_text(source).unwrap_or("");
            if let Some(pos) = text.rfind('.') {
                (
                    Some(text[pos + 1..].to_string()),
                    Some(text[..pos].to_string()),
                )
            } else {
                (Some(text.to_string()), None)
            }
        }
        _ => (None, None),
    }
}

fn find_enclosing_method(node: &Node, source: &[u8]) -> Option<String> {
    let mut current = node.parent();
    while let Some(n) = current {
        match n.kind() {
            "method_declaration" | "constructor_declaration" | "local_function_statement" => {
                if let Some(name_node) = n.child_by_field_name("name") {
                    return name_node.utf8_text(source).ok().map(|s| s.to_string());
                }
                for i in 0..n.child_count() {
                    if let Some(child) = n.child(i) {
                        if child.kind() == "identifier" {
                            return child.utf8_text(source).ok().map(|s| s.to_string());
                        }
                    }
                }
            }
            // Stop at property/event/operator boundaries
            "property_declaration"
            | "event_declaration"
            | "operator_declaration"
            | "indexer_declaration" => {
                return None;
            }
            _ => {}
        }
        current = n.parent();
    }
    None
}

impl LanguageAnalyser for CSharpAnalyser {
    fn extensions(&self) -> &[&str] {
        &["cs"]
    }

    fn language_name(&self) -> &str {
        "C#"
    }

    fn get_language(&self) -> Language {
        tree_sitter_c_sharp::LANGUAGE.into()
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
                if child.kind() == "using_directive" {
                    if let Some(imp) = self.extract_using(&child, source, file_path) {
                        imports.push(imp);
                    }
                } else if child.kind() == "namespace_declaration"
                    || child.kind() == "file_scoped_namespace_declaration"
                {
                    // Check for using directives inside namespace
                    for j in 0..child.child_count() {
                        if let Some(ns_child) = child.child(j) {
                            if ns_child.kind() == "declaration_list" {
                                for k in 0..ns_child.child_count() {
                                    if let Some(decl_child) = ns_child.child(k) {
                                        if decl_child.kind() == "using_directive" {
                                            if let Some(imp) =
                                                self.extract_using(&decl_child, source, file_path)
                                            {
                                                imports.push(imp);
                                            }
                                        }
                                    }
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
