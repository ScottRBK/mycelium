//! PyO3 bindings for Mycelium analysis engine.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use mycelium_core::config::AnalysisConfig;
use mycelium_core::pipeline;

/// Python-visible analysis configuration.
#[pyclass]
#[derive(Clone)]
struct PyAnalysisConfig {
    #[pyo3(get, set)]
    repo_path: String,
    #[pyo3(get, set)]
    output_path: Option<String>,
    #[pyo3(get, set)]
    languages: Option<Vec<String>>,
    #[pyo3(get, set)]
    resolution: f64,
    #[pyo3(get, set)]
    max_processes: usize,
    #[pyo3(get, set)]
    max_depth: usize,
    #[pyo3(get, set)]
    max_branching: usize,
    #[pyo3(get, set)]
    min_steps: usize,
    #[pyo3(get, set)]
    exclude_patterns: Vec<String>,
    #[pyo3(get, set)]
    verbose: bool,
    #[pyo3(get, set)]
    quiet: bool,
    #[pyo3(get, set)]
    max_file_size: u64,
    #[pyo3(get, set)]
    max_community_size: usize,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl PyAnalysisConfig {
    #[new]
    #[pyo3(signature = (
        repo_path = String::new(),
        output_path = None,
        languages = None,
        resolution = 1.0,
        max_processes = 75,
        max_depth = 10,
        max_branching = 4,
        min_steps = 2,
        exclude_patterns = Vec::new(),
        verbose = false,
        quiet = false,
        max_file_size = 1_000_000,
        max_community_size = 50,
    ))]
    fn new(
        repo_path: String,
        output_path: Option<String>,
        languages: Option<Vec<String>>,
        resolution: f64,
        max_processes: usize,
        max_depth: usize,
        max_branching: usize,
        min_steps: usize,
        exclude_patterns: Vec<String>,
        verbose: bool,
        quiet: bool,
        max_file_size: u64,
        max_community_size: usize,
    ) -> Self {
        Self {
            repo_path,
            output_path,
            languages,
            resolution,
            max_processes,
            max_depth,
            max_branching,
            min_steps,
            exclude_patterns,
            verbose,
            quiet,
            max_file_size,
            max_community_size,
        }
    }
}

impl From<PyAnalysisConfig> for AnalysisConfig {
    fn from(py_config: PyAnalysisConfig) -> Self {
        AnalysisConfig {
            repo_path: py_config.repo_path,
            output_path: py_config.output_path,
            languages: py_config.languages,
            resolution: py_config.resolution,
            max_processes: py_config.max_processes,
            max_depth: py_config.max_depth,
            max_branching: py_config.max_branching,
            min_steps: py_config.min_steps,
            exclude_patterns: py_config.exclude_patterns,
            verbose: py_config.verbose,
            quiet: py_config.quiet,
            max_file_size: py_config.max_file_size,
            max_community_size: py_config.max_community_size,
        }
    }
}

/// Analyse a source code repository and return the result as a Python dict.
#[pyfunction]
#[pyo3(signature = (path, config = None, progress = None))]
fn analyze(
    py: Python<'_>,
    path: &str,
    config: Option<PyAnalysisConfig>,
    progress: Option<PyObject>,
) -> PyResult<Py<PyDict>> {
    let analysis_config = match config {
        Some(c) => {
            let mut cfg: AnalysisConfig = c.into();
            cfg.repo_path = path.to_string();
            cfg
        }
        None => AnalysisConfig {
            repo_path: path.to_string(),
            ..Default::default()
        },
    };

    // Wrap the Python callable as a Rust ProgressCallback
    let progress_callback = progress.map(|py_cb| -> pipeline::ProgressCallback {
        Box::new(move |phase: &str, label: &str| {
            Python::with_gil(|py| {
                let _ = py_cb.call1(py, (phase, label));
            });
        })
    });

    let result = pipeline::run_pipeline(&analysis_config, progress_callback)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    // Serialize to JSON then parse into Python dict
    let json_str = serde_json::to_string(&result)
        .map_err(|e: serde_json::Error| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    let json_module = py.import("json")?;
    let py_dict = json_module
        .call_method1("loads", (json_str,))?
        .extract::<Py<PyDict>>()?;

    Ok(py_dict)
}

/// Return the Mycelium engine version.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Mycelium Rust analysis engine.
#[pymodule]
fn _mycelium_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<PyAnalysisConfig>()?;
    Ok(())
}
