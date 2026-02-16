//! Phase 2: Tree-sitter AST parsing, extract symbols into SymbolTable + KnowledgeGraph + NamespaceIndex.

use std::collections::HashSet;
use std::path::Path;

use crate::config::AnalysisConfig;
use crate::graph::knowledge_graph::{KnowledgeGraph, NodeData};
use crate::graph::namespace_index::NamespaceIndex;
use crate::graph::symbol_table::SymbolTable;
use crate::languages::AnalyserRegistry;

/// Run the parsing phase: parse all source files and extract symbols.
pub fn run_parsing_phase(
    config: &AnalysisConfig,
    kg: &mut KnowledgeGraph,
    st: &mut SymbolTable,
    ns_index: &mut NamespaceIndex,
) {
    let registry = AnalyserRegistry::new();

    // Collect file paths from the knowledge graph
    let files: Vec<(String, Option<String>)> = kg
        .get_files()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::File { path, language, .. } = n {
                Some((path.clone(), language.clone()))
            } else {
                None
            }
        })
        .collect();

    // Track used symbol IDs for deduplication
    let mut used_ids = HashSet::new();

    for (file_path, _language) in &files {
        let ext = Path::new(file_path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let analyser = match registry.get_by_extension(&ext) {
            Some(a) => a,
            None => continue,
        };

        let abs_path = Path::new(&config.repo_path).join(file_path);
        let source = match std::fs::read(&abs_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Parse with tree-sitter, using extension-specific language
        let mut parser = tree_sitter::Parser::new();
        let language = analyser.get_language_for_ext(&ext);
        if parser.set_language(&language).is_err() {
            continue;
        }
        let tree = match parser.parse(&source, None) {
            Some(t) => t,
            None => continue,
        };

        // Extract symbols
        let mut symbols = analyser.extract_symbols(&tree, &source, file_path);

        // Fix up symbol IDs: replace _pending_N with proper IDs
        for symbol in &mut symbols {
            let base_id = if let Some(ref parent) = symbol.parent {
                format!("{}:{}.{}", file_path, parent, symbol.name)
            } else {
                format!("{}:{}", file_path, symbol.name)
            };

            // Deduplicate IDs
            let mut id = base_id.clone();
            let mut counter = 1;
            while used_ids.contains(&id) {
                id = format!("{}_{}", base_id, counter);
                counter += 1;
            }
            used_ids.insert(id.clone());
            symbol.id = id;
        }

        for symbol in &symbols {
            kg.add_symbol(symbol);
            st.add(symbol);

            // Register namespaces
            if symbol.symbol_type.as_str() == "Namespace" {
                ns_index.register(&symbol.name, &symbol.file);
            }
        }
    }
}
