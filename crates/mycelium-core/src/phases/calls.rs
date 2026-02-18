//! Phase 4: Build call graph with tiered confidence.

use std::collections::HashMap;

use crate::config::{AnalysisConfig, CallEdge};
use crate::graph::knowledge_graph::KnowledgeGraph;
use crate::graph::namespace_index::NamespaceIndex;
use crate::graph::symbol_table::SymbolTable;
use crate::languages::AnalyserRegistry;

/// Run the calls phase: build call graph with tiered confidence.
pub fn run_calls_phase(
    config: &AnalysisConfig,
    kg: &mut KnowledgeGraph,
    st: &mut SymbolTable,
    _ns_index: &mut NamespaceIndex,
) {
    let repo_root = &config.repo_path;
    let registry = AnalyserRegistry::new();

    // Build a map of file imports for Tier A resolution
    let import_map = build_import_map(kg);

    // Build field-type maps per file for DI resolution (lazy)
    let mut field_type_maps: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Collect file info first to avoid borrowing issues
    let files: Vec<(String, String)> = kg
        .get_files()
        .iter()
        .filter_map(|nd| {
            if let crate::graph::knowledge_graph::NodeData::File { path, language, .. } = nd {
                language.as_ref().map(|l| (path.clone(), l.clone()))
            } else {
                None
            }
        })
        .collect();

    for (file_path, language) in &files {
        if let Some(ref langs) = config.languages {
            if !langs.contains(language) {
                continue;
            }
        }

        let ext = file_path.rsplit('.').next().unwrap_or("").to_string();

        let analyser = match registry.get_by_extension(&ext) {
            Some(a) => a,
            None => continue,
        };

        if !analyser.is_available() {
            continue;
        }

        // Read and parse
        let full_path = format!("{repo_root}/{file_path}");
        let source = match std::fs::read(&full_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let lang_ts = analyser.get_language_for_ext(&ext);
        let mut parser = tree_sitter::Parser::new();
        if parser.set_language(&lang_ts).is_err() {
            continue;
        }

        let tree = match parser.parse(&source, None) {
            Some(t) => t,
            None => continue,
        };

        // Extract raw calls
        let raw_calls = analyser.extract_calls(&tree, &source, file_path);

        // Build field-type map for this file (lazy, once per file)
        if !field_type_maps.contains_key(file_path.as_str()) {
            let ftm = build_field_type_map(file_path, kg);
            field_type_maps.insert(file_path.clone(), ftm);
        }

        // Resolve each call
        let ftm = field_type_maps
            .get(file_path.as_str())
            .cloned()
            .unwrap_or_default();

        for raw_call in &raw_calls {
            if let Some(edge) = resolve_call(raw_call, file_path, st, &import_map, kg, &ftm) {
                kg.add_call(&edge);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build a map from source file -> list of imported file paths.
fn build_import_map(kg: &KnowledgeGraph) -> HashMap<String, Vec<String>> {
    let mut import_map: HashMap<String, Vec<String>> = HashMap::new();
    for (from_file, to_file, _stmt) in kg.get_import_edges() {
        import_map.entry(from_file).or_default().push(to_file);
    }
    import_map
}

/// Build field/parameter name -> type name map for DI resolution.
fn build_field_type_map(file_path: &str, kg: &KnowledgeGraph) -> HashMap<String, String> {
    let mut field_map = HashMap::new();
    for sym in kg.get_symbols_in_file(file_path) {
        if let Some(ref param_types) = sym.parameter_types {
            for (param_name, type_name) in param_types {
                field_map.insert(param_name.clone(), type_name.clone());
                // Convention: _paramName field stores the DI-injected service
                field_map.insert(format!("_{param_name}"), type_name.clone());
            }
        }
    }
    field_map
}

/// Check if this is an interface calling its own method definition.
fn is_interface_self_call(
    caller_name: &str,
    callee_name: &str,
    target_id: &str,
    kg: &KnowledgeGraph,
) -> bool {
    if caller_name != callee_name {
        return false;
    }
    let syms = kg.get_symbols();
    let target = syms.iter().find(|s| s.id == target_id);
    let parent_name = match target.and_then(|s| s.parent.as_ref()) {
        Some(p) => p,
        None => return false,
    };
    syms.iter()
        .any(|s| s.name == *parent_name && s.symbol_type == "Interface")
}

/// Check if a symbol is a method declared in an interface.
fn is_interface_method(target_id: &str, kg: &KnowledgeGraph) -> bool {
    let syms = kg.get_symbols();
    let target = syms.iter().find(|s| s.id == target_id);
    let parent_name = match target.and_then(|s| s.parent.as_ref()) {
        Some(p) => p,
        None => return false,
    };
    syms.iter()
        .any(|s| s.name == *parent_name && s.symbol_type == "Interface")
}

/// Find a concrete implementation of an interface method.
fn find_implementation(
    callee_name: &str,
    interface_target_id: &str,
    st: &SymbolTable,
    import_map: &HashMap<String, Vec<String>>,
    file_path: &str,
    kg: &KnowledgeGraph,
) -> Option<String> {
    let syms = kg.get_symbols();
    let interface_file = syms
        .iter()
        .find(|s| s.id == interface_target_id)
        .map(|s| s.file.clone())
        .unwrap_or_default();

    let imported_files = import_map.get(file_path).cloned().unwrap_or_default();

    for imported_file in &imported_files {
        if imported_file == &interface_file {
            continue;
        }
        if let Some(target_id) = st.lookup_exact(imported_file, callee_name) {
            if target_id != interface_target_id && !is_interface_method(target_id, kg) {
                return Some(target_id.to_string());
            }
        }
    }

    // Fuzzy lookup for implementations not in direct imports
    let fuzzy_matches = st.lookup_fuzzy(callee_name);
    for m in fuzzy_matches {
        if m.symbol_id != interface_target_id
            && m.file != interface_file
            && !is_interface_method(&m.symbol_id, kg)
        {
            return Some(m.symbol_id.clone());
        }
    }

    None
}

/// Resolve a raw call to a CallEdge with tiered confidence scoring.
fn resolve_call(
    raw_call: &crate::config::RawCall,
    file_path: &str,
    st: &SymbolTable,
    import_map: &HashMap<String, Vec<String>>,
    kg: &KnowledgeGraph,
    field_type_map: &HashMap<String, String>,
) -> Option<CallEdge> {
    let callee_name = &raw_call.callee_name;
    let caller_name = &raw_call.caller_name;
    let qualifier = raw_call.qualifier.as_deref();

    // Find the caller's symbol ID
    let caller_id = if let Some(id) = st.lookup_exact(file_path, caller_name) {
        id.to_string()
    } else {
        let fuzzy = st.lookup_fuzzy(caller_name);
        let file_match = fuzzy.iter().find(|m| m.file == file_path);
        match file_match {
            Some(m) => m.symbol_id.clone(),
            None => return None,
        }
    };

    // --- Tier A: Import-resolved ---
    if let Some(imported_files) = import_map.get(file_path) {
        for imported_file in imported_files {
            if let Some(target_id) = st.lookup_exact(imported_file, callee_name) {
                if target_id == caller_id {
                    continue;
                }
                if is_interface_self_call(caller_name, callee_name, target_id, kg) {
                    continue;
                }
                // If target is an interface method, try to find implementation
                if is_interface_method(target_id, kg) {
                    if let Some(impl_id) =
                        find_implementation(callee_name, target_id, st, import_map, file_path, kg)
                    {
                        return Some(CallEdge {
                            from_symbol: caller_id,
                            to_symbol: impl_id,
                            confidence: 0.85,
                            tier: "A".to_string(),
                            reason: "impl-resolved".to_string(),
                            line: raw_call.line,
                        });
                    }
                }
                return Some(CallEdge {
                    from_symbol: caller_id,
                    to_symbol: target_id.to_string(),
                    confidence: 0.9,
                    tier: "A".to_string(),
                    reason: "import-resolved".to_string(),
                    line: raw_call.line,
                });
            }
        }
    }

    // --- Tier A-DI: DI-resolved (qualifier is a field name) ---
    if let Some(q) = qualifier {
        if let Some(type_name) = field_type_map.get(q) {
            if let Some(imported_files) = import_map.get(file_path) {
                for imported_file in imported_files {
                    if st.lookup_exact(imported_file, type_name).is_some() {
                        if let Some(target_id) = st.lookup_exact(imported_file, callee_name) {
                            if target_id == caller_id {
                                continue;
                            }
                            if is_interface_self_call(caller_name, callee_name, target_id, kg) {
                                continue;
                            }
                            if is_interface_method(target_id, kg) {
                                if let Some(impl_id) = find_implementation(
                                    callee_name,
                                    target_id,
                                    st,
                                    import_map,
                                    file_path,
                                    kg,
                                ) {
                                    return Some(CallEdge {
                                        from_symbol: caller_id,
                                        to_symbol: impl_id,
                                        confidence: 0.85,
                                        tier: "A".to_string(),
                                        reason: "di-impl-resolved".to_string(),
                                        line: raw_call.line,
                                    });
                                }
                            }
                            return Some(CallEdge {
                                from_symbol: caller_id,
                                to_symbol: target_id.to_string(),
                                confidence: 0.9,
                                tier: "A".to_string(),
                                reason: "di-resolved".to_string(),
                                line: raw_call.line,
                            });
                        }
                    }
                }
            }
        }
    }

    // --- Tier B: Same-file ---
    if let Some(target_id) = st.lookup_exact(file_path, callee_name) {
        if target_id != caller_id {
            return Some(CallEdge {
                from_symbol: caller_id,
                to_symbol: target_id.to_string(),
                confidence: 0.85,
                tier: "B".to_string(),
                reason: "same-file".to_string(),
                line: raw_call.line,
            });
        }
    }

    // --- Tier C: Fuzzy global ---
    let fuzzy_matches = st.lookup_fuzzy(callee_name);
    let filtered: Vec<_> = fuzzy_matches
        .iter()
        .filter(|m| m.file != file_path)
        .collect();

    if filtered.len() == 1 {
        let target_id = &filtered[0].symbol_id;
        if is_interface_self_call(caller_name, callee_name, target_id, kg) {
            return None;
        }
        return Some(CallEdge {
            from_symbol: caller_id,
            to_symbol: target_id.clone(),
            confidence: 0.5,
            tier: "C".to_string(),
            reason: "fuzzy-unique".to_string(),
            line: raw_call.line,
        });
    } else if filtered.len() > 1 {
        let target_id = &filtered[0].symbol_id;
        if is_interface_self_call(caller_name, callee_name, target_id, kg) {
            return None;
        }
        return Some(CallEdge {
            from_symbol: caller_id,
            to_symbol: target_id.clone(),
            confidence: 0.3,
            tier: "C".to_string(),
            reason: "fuzzy-ambiguous".to_string(),
            line: raw_call.line,
        });
    }

    None
}
