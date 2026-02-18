//! Shared test helpers for integration tests.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use mycelium_core::config::AnalysisConfig;
use mycelium_core::graph::knowledge_graph::KnowledgeGraph;
use mycelium_core::graph::namespace_index::NamespaceIndex;
use mycelium_core::graph::symbol_table::SymbolTable;
use mycelium_core::languages::{AnalyserRegistry, LanguageAnalyser};

// ---------------------------------------------------------------------------
// Fixture path resolution
// ---------------------------------------------------------------------------

/// Resolve `tests/fixtures/{name}` relative to the workspace root.
pub fn fixture_path(name: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("../../tests/fixtures")
        .join(name)
        .canonicalize()
        .unwrap_or_else(|_| {
            Path::new(manifest_dir)
                .join("../../tests/fixtures")
                .join(name)
        })
}

// ---------------------------------------------------------------------------
// Phase runners
// ---------------------------------------------------------------------------

pub struct PhaseResult {
    pub kg: KnowledgeGraph,
    pub st: SymbolTable,
    pub ns_index: NamespaceIndex,
    pub config: AnalysisConfig,
}

/// Run Phase 1 (structure) on a fixture directory.
pub fn run_structure(fixture_name: &str) -> PhaseResult {
    let path = fixture_path(fixture_name);
    let config = AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let mut kg = KnowledgeGraph::new();
    mycelium_core::phases::structure::run_structure_phase(&config, &mut kg);
    PhaseResult {
        kg,
        st: SymbolTable::new(),
        ns_index: NamespaceIndex::new(),
        config,
    }
}

/// Run Phases 1-2 (structure + parsing) on a fixture directory.
pub fn run_two_phases(fixture_name: &str) -> PhaseResult {
    let mut r = run_structure(fixture_name);
    mycelium_core::phases::parsing::run_parsing_phase(
        &r.config,
        &mut r.kg,
        &mut r.st,
        &mut r.ns_index,
    );
    r
}

/// Run Phases 1-3 (structure + parsing + imports) on a fixture directory.
pub fn run_three_phases(fixture_name: &str) -> PhaseResult {
    let mut r = run_two_phases(fixture_name);
    mycelium_core::phases::imports::run_imports_phase(
        &r.config,
        &mut r.kg,
        &mut r.st,
        &mut r.ns_index,
    );
    r
}

/// Run Phases 1-4 (structure + parsing + imports + calls) on a fixture directory.
pub fn run_four_phases(fixture_name: &str) -> PhaseResult {
    let mut r = run_three_phases(fixture_name);
    mycelium_core::phases::calls::run_calls_phase(&r.config, &mut r.kg, &mut r.st, &mut r.ns_index);
    r
}

/// Run all 6 phases on a fixture directory.
pub fn run_all_phases(fixture_name: &str) -> PhaseResult {
    let mut r = run_four_phases(fixture_name);
    mycelium_core::phases::communities::run_communities_phase(&r.config, &mut r.kg);
    mycelium_core::phases::processes::run_processes_phase(&r.config, &mut r.kg);
    r
}

// ---------------------------------------------------------------------------
// Extractors from KnowledgeGraph
// ---------------------------------------------------------------------------

/// Extract all symbol names from the knowledge graph.
pub fn symbol_names(kg: &KnowledgeGraph) -> Vec<String> {
    kg.get_symbols().into_iter().map(|s| s.name).collect()
}

/// Extract symbol names in a specific file.
pub fn symbol_names_in_file(kg: &KnowledgeGraph, file_path: &str) -> Vec<String> {
    kg.get_symbols_in_file(file_path)
        .into_iter()
        .map(|s| s.name)
        .collect()
}

/// Extract import edge pairs (from_file, to_file).
pub fn import_targets(kg: &KnowledgeGraph) -> Vec<(String, String)> {
    kg.get_import_edges()
        .into_iter()
        .map(|(from, to, _)| (from, to))
        .collect()
}

/// Extract call edge pairs (from_symbol_name, to_symbol_name).
pub fn call_pairs(kg: &KnowledgeGraph) -> Vec<(String, String)> {
    let syms = kg.get_symbols();
    let id_to_name: std::collections::HashMap<String, String> =
        syms.into_iter().map(|s| (s.id, s.name)).collect();

    kg.get_call_edges()
        .into_iter()
        .filter_map(|(from, to, _, _, _, _)| {
            let from_name = id_to_name.get(&from)?;
            let to_name = id_to_name.get(&to)?;
            Some((from_name.clone(), to_name.clone()))
        })
        .collect()
}

/// Get all file paths from the knowledge graph.
pub fn file_paths(kg: &KnowledgeGraph) -> Vec<String> {
    kg.get_files()
        .into_iter()
        .filter_map(|n| {
            if let mycelium_core::graph::knowledge_graph::NodeData::File { path, .. } = n {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Get all folder paths from the knowledge graph.
pub fn folder_paths(kg: &KnowledgeGraph) -> Vec<String> {
    kg.get_folders()
        .into_iter()
        .filter_map(|n| {
            if let mycelium_core::graph::knowledge_graph::NodeData::Folder { path, .. } = n {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Get file languages as a set.
pub fn file_languages(kg: &KnowledgeGraph) -> HashSet<String> {
    kg.get_files()
        .into_iter()
        .filter_map(|n| {
            if let mycelium_core::graph::knowledge_graph::NodeData::File {
                language: Some(lang),
                ..
            } = n
            {
                Some(lang.clone())
            } else {
                None
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Single-file parsers (for language analyser tests)
// ---------------------------------------------------------------------------

/// Parse a single file and return extracted symbols.
pub fn parse_file_symbols(
    fixture_name: &str,
    file_name: &str,
) -> Vec<mycelium_core::config::Symbol> {
    let path = fixture_path(fixture_name).join(file_name);
    let source = std::fs::read(&path).expect("Failed to read fixture file");
    let ext = Path::new(file_name)
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    let registry = AnalyserRegistry::new();
    let analyser = registry
        .get_by_extension(&ext)
        .expect("No analyser for extension");

    let mut parser = tree_sitter::Parser::new();
    let language = analyser.get_language_for_ext(&ext);
    parser
        .set_language(&language)
        .expect("Failed to set language");
    let tree = parser.parse(&source, None).expect("Failed to parse");

    analyser.extract_symbols(&tree, &source, file_name)
}

/// Parse a single file and return extracted imports.
pub fn parse_file_imports(
    fixture_name: &str,
    file_name: &str,
) -> Vec<mycelium_core::config::ImportStatement> {
    let path = fixture_path(fixture_name).join(file_name);
    let source = std::fs::read(&path).expect("Failed to read fixture file");
    let ext = Path::new(file_name)
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    let registry = AnalyserRegistry::new();
    let analyser = registry
        .get_by_extension(&ext)
        .expect("No analyser for extension");

    let mut parser = tree_sitter::Parser::new();
    let language = analyser.get_language_for_ext(&ext);
    parser
        .set_language(&language)
        .expect("Failed to set language");
    let tree = parser.parse(&source, None).expect("Failed to parse");

    analyser.extract_imports(&tree, &source, file_name)
}

/// Parse a single file and return extracted calls.
pub fn parse_file_calls(
    fixture_name: &str,
    file_name: &str,
) -> Vec<mycelium_core::config::RawCall> {
    let path = fixture_path(fixture_name).join(file_name);
    let source = std::fs::read(&path).expect("Failed to read fixture file");
    let ext = Path::new(file_name)
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    let registry = AnalyserRegistry::new();
    let analyser = registry
        .get_by_extension(&ext)
        .expect("No analyser for extension");

    let mut parser = tree_sitter::Parser::new();
    let language = analyser.get_language_for_ext(&ext);
    parser
        .set_language(&language)
        .expect("Failed to set language");
    let tree = parser.parse(&source, None).expect("Failed to parse");

    analyser.extract_calls(&tree, &source, file_name)
}
