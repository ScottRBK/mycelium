//! Pipeline orchestration and E2E integration tests.

mod common;

use common::*;

// ===========================================================================
// Pipeline orchestration (5 tests)
// ===========================================================================

#[test]
fn pipeline_runs_all_phases() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert_eq!(result.version, "1.0");
    assert!(!result.symbols.is_empty(), "Pipeline should produce symbols");
}

#[test]
fn pipeline_with_progress_callback() {
    let path = fixture_path("python_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let mut phases_seen = Vec::new();
    let callback: mycelium_core::pipeline::ProgressCallback =
        Box::new(move |phase, _label| {
            phases_seen.push(phase.to_string());
        });
    let result = mycelium_core::pipeline::run_pipeline(&config, Some(callback)).unwrap();
    assert!(!result.symbols.is_empty());
}

#[test]
fn pipeline_metadata() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(result.metadata.contains_key("repo_name"));
    assert!(result.metadata.contains_key("analysed_at"));
    assert!(result.metadata.contains_key("analysis_duration_ms"));
    assert!(result.metadata.contains_key("phase_timings"));
}

#[test]
fn pipeline_stats() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    let expected_keys = [
        "files", "folders", "symbols", "calls", "imports", "communities", "processes", "languages",
    ];
    for key in &expected_keys {
        assert!(result.stats.contains_key(*key), "Missing stat key: {key}");
    }
}

#[test]
fn pipeline_phase_timings() {
    let path = fixture_path("python_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    let timings = result
        .metadata
        .get("phase_timings")
        .and_then(|v| v.as_object());
    assert!(timings.is_some(), "Should have phase_timings in metadata");
    let timings = timings.unwrap();
    let expected_phases = [
        "structure",
        "parsing",
        "imports",
        "calls",
        "communities",
        "processes",
    ];
    for phase in &expected_phases {
        assert!(
            timings.contains_key(*phase),
            "Missing phase timing: {}",
            phase
        );
    }
}

// ===========================================================================
// Output JSON (4 tests)
// ===========================================================================

#[test]
fn output_json_roundtrip() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    let json = serde_json::to_string_pretty(&result).unwrap();
    let parsed: mycelium_core::config::AnalysisResult = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.version, result.version);
    assert_eq!(parsed.symbols.len(), result.symbols.len());
}

#[test]
fn output_write_and_read() {
    let path = fixture_path("python_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let out_path = tmp.path().to_string_lossy().to_string();
    mycelium_core::output::write_output(&result, &out_path).unwrap();

    let content = std::fs::read_to_string(&out_path).unwrap();
    let parsed: mycelium_core::config::AnalysisResult = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.version, "1.0");
    assert_eq!(parsed.symbols.len(), result.symbols.len());
}

#[test]
fn output_structure_files() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(!result.structure.files.is_empty(), "Should have file nodes");
}

#[test]
fn output_structure_folders() {
    let path = fixture_path("python_package");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(
        !result.structure.folders.is_empty(),
        "Should have folder nodes for nested package"
    );
}

// ===========================================================================
// E2E multi-language (4 tests)
// ===========================================================================

#[test]
fn e2e_csharp_all_sections_populated() {
    let path = fixture_path("csharp_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(!result.structure.files.is_empty(), "files");
    assert!(!result.symbols.is_empty(), "symbols");
    assert!(!result.calls.is_empty(), "calls");
    assert!(!result.communities.is_empty(), "communities");
    assert!(!result.processes.is_empty(), "processes");
}

#[test]
fn e2e_python_all_sections_populated() {
    let path = fixture_path("python_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(!result.structure.files.is_empty(), "files");
    assert!(!result.symbols.is_empty(), "symbols");
    assert!(!result.calls.is_empty(), "calls");
    assert!(!result.communities.is_empty(), "communities");
    assert!(!result.processes.is_empty(), "processes");
}

#[test]
fn e2e_java_all_sections_populated() {
    let path = fixture_path("java_simple");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(!result.structure.files.is_empty(), "files");
    assert!(!result.symbols.is_empty(), "symbols");
    assert!(!result.calls.is_empty(), "calls");
    assert!(!result.communities.is_empty(), "communities");
    assert!(!result.processes.is_empty(), "processes");
}

#[test]
fn e2e_mixed_dotnet_pipeline() {
    let path = fixture_path("mixed_dotnet");
    let config = mycelium_core::config::AnalysisConfig {
        repo_path: path.to_string_lossy().to_string(),
        ..Default::default()
    };
    let result = mycelium_core::pipeline::run_pipeline(&config, None).unwrap();
    assert!(!result.structure.files.is_empty(), "files");
    // mixed_dotnet has limited code so some sections may be sparse
    let _ = result;
}
