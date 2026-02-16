//! JSON serialisation matching the Python output schema.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use chrono::Utc;

use crate::config::{
    AnalysisConfig, AnalysisResult, CallOutput, CommunityOutput, FileOutput, FolderOutput,
    ImportOutput, ImportsOutput, PackageRefOutput, ProcessOutput, ProjectRefOutput,
    StructureOutput, SymbolOutput,
};
use crate::graph::knowledge_graph::{KnowledgeGraph, NodeData};
use crate::graph::symbol_table::SymbolTable;

/// Try to get the current git commit hash (first 12 chars).
fn get_commit_hash(repo_path: &str) -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Some(hash[..hash.len().min(12)].to_string())
            } else {
                None
            }
        })
}

/// Count files per language.
fn count_languages(kg: &KnowledgeGraph) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for file in kg.get_files() {
        if let NodeData::File {
            language: Some(lang),
            ..
        } = file
        {
            *counts.entry(lang.clone()).or_insert(0) += 1;
        }
    }
    counts
}

/// Build the AnalysisResult from the knowledge graph.
pub fn build_result(
    config: &AnalysisConfig,
    kg: &KnowledgeGraph,
    _st: &SymbolTable,
    timings: &HashMap<String, f64>,
    total_ms: f64,
) -> AnalysisResult {
    let repo_path = Path::new(&config.repo_path)
        .canonicalize()
        .unwrap_or_else(|_| Path::new(&config.repo_path).to_path_buf());
    let repo_name = repo_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let call_edges = kg.get_call_edges();
    let import_edges = kg.get_import_edges();
    let communities = kg.get_communities();
    let processes = kg.get_processes();
    let languages = count_languages(kg);

    // Build metadata
    let mut metadata = HashMap::new();
    metadata.insert(
        "repo_name".to_string(),
        serde_json::Value::String(repo_name),
    );
    metadata.insert(
        "repo_path".to_string(),
        serde_json::Value::String(repo_path.to_string_lossy().to_string()),
    );
    metadata.insert(
        "analysed_at".to_string(),
        serde_json::Value::String(Utc::now().to_rfc3339()),
    );
    metadata.insert(
        "mycelium_version".to_string(),
        serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()),
    );
    if let Some(hash) = get_commit_hash(&config.repo_path) {
        metadata.insert("commit_hash".to_string(), serde_json::Value::String(hash));
    } else {
        metadata.insert("commit_hash".to_string(), serde_json::Value::Null);
    }
    metadata.insert(
        "analysis_duration_ms".to_string(),
        serde_json::json!(((total_ms * 10.0).round() / 10.0)),
    );
    metadata.insert(
        "phase_timings".to_string(),
        serde_json::to_value(timings).unwrap_or_default(),
    );

    // Build stats
    let mut stats = HashMap::new();
    stats.insert("files".to_string(), serde_json::json!(kg.file_count()));
    stats.insert("folders".to_string(), serde_json::json!(kg.folder_count()));
    stats.insert("symbols".to_string(), serde_json::json!(kg.symbol_count()));
    stats.insert("calls".to_string(), serde_json::json!(call_edges.len()));
    stats.insert("imports".to_string(), serde_json::json!(import_edges.len()));
    stats.insert(
        "communities".to_string(),
        serde_json::json!(communities.len()),
    );
    stats.insert("processes".to_string(), serde_json::json!(processes.len()));
    stats.insert(
        "languages".to_string(),
        serde_json::to_value(&languages).unwrap_or_default(),
    );

    // Build structure
    let files: Vec<FileOutput> = kg
        .get_files()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::File {
                path,
                language,
                size,
                lines,
            } = n
            {
                Some(FileOutput {
                    path: path.clone(),
                    language: language.clone(),
                    size: *size,
                    lines: *lines,
                })
            } else {
                None
            }
        })
        .collect();

    let folders: Vec<FolderOutput> = kg
        .get_folders()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::Folder { path, file_count } = n {
                Some(FolderOutput {
                    path: path.clone(),
                    file_count: *file_count,
                })
            } else {
                None
            }
        })
        .collect();

    // Build symbols
    let symbols: Vec<SymbolOutput> = kg
        .get_symbols()
        .into_iter()
        .map(|s| SymbolOutput {
            id: s.id,
            name: s.name,
            symbol_type: s.symbol_type,
            file: s.file,
            line: s.line,
            visibility: s.visibility,
            exported: s.exported,
            parent: s.parent,
            language: s.language,
        })
        .collect();

    // Build imports
    let file_imports: Vec<ImportOutput> = import_edges
        .into_iter()
        .map(|(from, to, statement)| ImportOutput {
            from,
            to,
            statement,
        })
        .collect();

    let project_references: Vec<ProjectRefOutput> = kg
        .get_project_references()
        .into_iter()
        .map(|(from, to, ref_type)| ProjectRefOutput { from, to, ref_type })
        .collect();

    let package_references: Vec<PackageRefOutput> = kg
        .get_package_references()
        .into_iter()
        .map(|(project, package, version)| PackageRefOutput {
            project,
            package,
            version,
        })
        .collect();

    // Build calls
    let calls: Vec<CallOutput> = call_edges
        .into_iter()
        .map(|(from, to, confidence, tier, reason, line)| CallOutput {
            from,
            to,
            confidence,
            tier,
            reason,
            line,
        })
        .collect();

    // Build communities
    let community_output: Vec<CommunityOutput> = communities
        .into_iter()
        .map(
            |(id, label, members, cohesion, primary_language)| CommunityOutput {
                id,
                label,
                members,
                cohesion,
                primary_language,
            },
        )
        .collect();

    // Build processes
    let process_output: Vec<ProcessOutput> = processes
        .into_iter()
        .map(
            |(id, entry, terminal, steps, process_type, total_confidence)| ProcessOutput {
                id,
                entry,
                terminal,
                steps,
                process_type,
                total_confidence,
            },
        )
        .collect();

    AnalysisResult {
        version: "1.0".to_string(),
        metadata,
        stats,
        structure: StructureOutput { files, folders },
        symbols,
        imports: ImportsOutput {
            file_imports,
            project_references,
            package_references,
        },
        calls,
        communities: community_output,
        processes: process_output,
    }
}

/// Write the analysis result to a JSON file.
pub fn write_output(result: &AnalysisResult, output_path: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(result).map_err(std::io::Error::other)?;
    std::fs::write(output_path, json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FileNode, Symbol, SymbolType, Visibility};

    #[test]
    fn build_result_basic() {
        let config = AnalysisConfig {
            repo_path: "/tmp/test-repo".to_string(),
            ..Default::default()
        };
        let mut kg = KnowledgeGraph::new();
        kg.add_file(&FileNode {
            path: "src/main.cs".to_string(),
            language: Some("C#".to_string()),
            size: 100,
            lines: 10,
        });
        kg.add_symbol(&Symbol {
            id: "sym:Main".to_string(),
            name: "Main".to_string(),
            symbol_type: SymbolType::Method,
            file: "src/main.cs".to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: Some("C#".to_string()),
            byte_range: None,
            parameter_types: None,
        });

        let st = SymbolTable::new();
        let timings = HashMap::new();

        let result = build_result(&config, &kg, &st, &timings, 100.0);

        assert_eq!(result.version, "1.0");
        assert_eq!(result.symbols.len(), 1);
        assert_eq!(result.symbols[0].name, "Main");

        // Verify JSON roundtrip
        let json = serde_json::to_string_pretty(&result).unwrap();
        let parsed: AnalysisResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.symbols.len(), 1);
    }

    #[test]
    fn json_roundtrip_preserves_all_fields() {
        let config = AnalysisConfig {
            repo_path: "/tmp/test-repo".to_string(),
            ..Default::default()
        };
        let mut kg = KnowledgeGraph::new();
        kg.add_file(&FileNode {
            path: "src/main.cs".to_string(),
            language: Some("C#".to_string()),
            size: 100,
            lines: 10,
        });
        let st = SymbolTable::new();
        let timings = HashMap::new();
        let result = build_result(&config, &kg, &st, &timings, 50.0);

        let json = serde_json::to_string_pretty(&result).unwrap();
        let parsed: AnalysisResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.version, "1.0");
        assert!(parsed.metadata.contains_key("repo_name"));
        assert!(parsed.metadata.contains_key("analysed_at"));
        assert!(parsed.metadata.contains_key("mycelium_version"));
        assert!(parsed.metadata.contains_key("analysis_duration_ms"));
    }

    #[test]
    fn stats_keys_present() {
        let config = AnalysisConfig {
            repo_path: "/tmp/test-repo".to_string(),
            ..Default::default()
        };
        let kg = KnowledgeGraph::new();
        let st = SymbolTable::new();
        let timings = HashMap::new();
        let result = build_result(&config, &kg, &st, &timings, 10.0);

        let expected_keys = [
            "files",
            "folders",
            "symbols",
            "calls",
            "imports",
            "communities",
            "processes",
            "languages",
        ];
        for key in &expected_keys {
            assert!(result.stats.contains_key(*key), "Missing stat key: {key}");
        }
    }
}
