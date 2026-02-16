//! Phase 1: Walk file tree, build FileNode/FolderNode graph.

use std::collections::HashMap;
use std::path::Path;

use walkdir::WalkDir;

use crate::config::{AnalysisConfig, FileNode, FolderNode};
use crate::graph::knowledge_graph::KnowledgeGraph;
use crate::languages::AnalyserRegistry;

/// Default patterns to exclude from analysis.
const DEFAULT_EXCLUDES: &[&str] = &[
    ".git",
    "node_modules",
    "__pycache__",
    ".vs",
    ".vscode",
    ".idea",
    "bin",
    "obj",
    "dist",
    "build",
    "target",
    ".mycelium.json",
    "packages",
    "TestResults",
    ".mypy_cache",
    ".pytest_cache",
    ".tox",
    ".eggs",
    ".venv",
    "venv",
    ".env",
];

/// Run the structure phase: walk the file tree and populate the graph.
pub fn run_structure_phase(config: &AnalysisConfig, kg: &mut KnowledgeGraph) {
    let repo_path = Path::new(&config.repo_path);
    let registry = AnalyserRegistry::new();
    let mut folder_file_counts: HashMap<String, usize> = HashMap::new();

    let exclude_patterns: Vec<&str> = DEFAULT_EXCLUDES
        .iter()
        .copied()
        .chain(config.exclude_patterns.iter().map(|s| s.as_str()))
        .collect();

    for entry in WalkDir::new(repo_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Skip explicitly excluded names
            if exclude_patterns.iter().any(|p| name == *p) {
                return false;
            }
            // Skip hidden directories (starting with .) like Python does,
            // except the repo root itself
            if e.depth() > 0 && e.file_type().is_dir() && name.starts_with('.') {
                return false;
            }
            true
        })
    {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let abs_path = entry.path();
        let rel_path = abs_path
            .strip_prefix(repo_path)
            .unwrap_or(abs_path)
            .to_string_lossy()
            .replace('\\', "/");

        if rel_path.is_empty() {
            continue;
        }

        if entry.file_type().is_dir() {
            // Folder — will set file_count after traversal
            folder_file_counts.entry(rel_path).or_insert(0);
        } else if entry.file_type().is_file() {
            let ext = abs_path
                .extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();

            let language = registry.language_for_extension(&ext).map(String::from);

            // Apply language filter if specified
            if let Some(ref lang_filter) = config.languages {
                if let Some(ref lang) = language {
                    if !lang_filter.iter().any(|f| f.eq_ignore_ascii_case(lang)) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Skip files over size limit
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            if size > config.max_file_size {
                continue;
            }

            // Count lines
            let lines = if language.is_some() {
                std::fs::read_to_string(abs_path)
                    .map(|content| content.lines().count())
                    .unwrap_or(0)
            } else {
                0
            };

            kg.add_file(&FileNode {
                path: rel_path.clone(),
                language,
                size,
                lines,
            });

            // Increment parent folder counts
            if let Some(parent) = Path::new(&rel_path).parent() {
                let parent_str = parent.to_string_lossy().replace('\\', "/");
                if !parent_str.is_empty() {
                    *folder_file_counts.entry(parent_str).or_insert(0) += 1;
                }
            }
        }
    }

    // Add folders
    for (path, file_count) in folder_file_counts {
        kg.add_folder(&FolderNode { path, file_count });
    }
}
