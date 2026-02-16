//! Phase 1: Structure phase integration tests.

mod common;

use common::*;

#[test]
fn discovers_csharp_files() {
    let r = run_structure("csharp_simple");
    let files = file_paths(&r.kg);
    assert!(
        files.iter().any(|f| f.ends_with(".cs")),
        "Should discover .cs files"
    );
    assert!(files.len() >= 8, "csharp_simple has 8 .cs files");
}

#[test]
fn detects_languages() {
    let r = run_structure("csharp_simple");
    let langs = file_languages(&r.kg);
    assert!(langs.contains("C#"), "Should detect C# language");
}

#[test]
fn counts_lines() {
    let r = run_structure("csharp_simple");
    for file_data in r.kg.get_files() {
        if let mycelium_core::graph::knowledge_graph::NodeData::File {
            language: Some(_),
            lines,
            ..
        } = file_data
        {
            assert!(*lines > 0, "Source files should have line counts > 0");
        }
    }
}

#[test]
fn creates_folders() {
    let r = run_structure("python_package");
    let folders = folder_paths(&r.kg);
    assert!(!folders.is_empty(), "Should create folder nodes");
}

#[test]
fn ignores_default_excluded_dirs() {
    let r = run_structure("python_simple");
    let files = file_paths(&r.kg);
    assert!(
        files.iter().all(|f| !f.contains("__pycache__")),
        "Should skip __pycache__"
    );
}

#[test]
fn multi_language_detection() {
    let r = run_structure("mixed_dotnet");
    let langs = file_languages(&r.kg);
    assert!(langs.contains("C#"), "Should detect C#");
    // .vb files won't have language if VB.NET analyser is unavailable
    // .sln and .csproj aren't source languages
}

#[test]
fn file_size_tracked() {
    let r = run_structure("csharp_simple");
    for file_data in r.kg.get_files() {
        if let mycelium_core::graph::knowledge_graph::NodeData::File { size, .. } = file_data {
            assert!(*size > 0, "Files should have non-zero size");
        }
    }
}

#[test]
fn language_filter() {
    let path = fixture_path("python_package");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        languages: Some(vec!["Python".to_string()]),
        ..Default::default()
    };
    let mut kg = mycelium_core::graph::knowledge_graph::KnowledgeGraph::new();
    mycelium_core::phases::structure::run_structure_phase(&config, &mut kg);

    let langs = file_languages(&kg);
    assert!(langs.contains("Python"));
    // Should only contain Python files
    for file_data in kg.get_files() {
        if let mycelium_core::graph::knowledge_graph::NodeData::File {
            language: Some(lang),
            ..
        } = file_data
        {
            assert_eq!(lang, "Python", "Filter should only include Python");
        }
    }
}
