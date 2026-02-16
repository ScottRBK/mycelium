//! Sequential phase orchestrator with timing.

use std::collections::HashMap;
use std::time::Instant;

use crate::config::{AnalysisConfig, AnalysisResult};
use crate::graph::knowledge_graph::KnowledgeGraph;
use crate::graph::namespace_index::NamespaceIndex;
use crate::graph::symbol_table::SymbolTable;
use crate::output::build_result;
use crate::phases;

/// Phase labels for progress reporting.
const PHASE_LABELS: &[(&str, &str)] = &[
    ("structure", "Mapping file tree"),
    ("parsing", "Parsing source files"),
    ("imports", "Resolving imports"),
    ("calls", "Building call graph"),
    ("communities", "Detecting communities"),
    ("processes", "Tracing execution flows"),
];

/// Progress callback type: (phase_name, label).
pub type ProgressCallback = Box<dyn FnMut(&str, &str)>;

/// Type alias for phase function closures to keep signatures readable.
type PhaseFn = Box<
    dyn FnOnce(
        &AnalysisConfig,
        &mut KnowledgeGraph,
        &mut SymbolTable,
        &mut NamespaceIndex,
    ) -> Result<(), Box<dyn std::error::Error>>,
>;

/// Execute the six-phase analysis pipeline and return the result.
pub fn run_pipeline(
    config: &AnalysisConfig,
    mut progress_callback: Option<ProgressCallback>,
) -> Result<AnalysisResult, Box<dyn std::error::Error>> {
    let mut kg = KnowledgeGraph::new();
    let mut st = SymbolTable::new();
    let mut ns_index = NamespaceIndex::new();
    let mut timings: HashMap<String, f64> = HashMap::new();
    let total_start = Instant::now();

    let phase_fns: Vec<(&str, PhaseFn)> = vec![
        (
            "structure",
            Box::new(|config, kg, _st, _ns| {
                phases::structure::run_structure_phase(config, kg);
                Ok(())
            }),
        ),
        (
            "parsing",
            Box::new(|config, kg, st, ns| {
                phases::parsing::run_parsing_phase(config, kg, st, ns);
                Ok(())
            }),
        ),
        (
            "imports",
            Box::new(|config, kg, st, ns| {
                phases::imports::run_imports_phase(config, kg, st, ns);
                Ok(())
            }),
        ),
        (
            "calls",
            Box::new(|config, kg, st, ns| {
                phases::calls::run_calls_phase(config, kg, st, ns);
                Ok(())
            }),
        ),
        (
            "communities",
            Box::new(|config, kg, _st, _ns| {
                phases::communities::run_communities_phase(config, kg);
                Ok(())
            }),
        ),
        (
            "processes",
            Box::new(|config, kg, _st, _ns| {
                phases::processes::run_processes_phase(config, kg);
                Ok(())
            }),
        ),
    ];

    for (name, phase_fn) in phase_fns {
        // Report progress
        if let Some(ref mut cb) = progress_callback {
            let label = PHASE_LABELS
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, l)| *l)
                .unwrap_or(name);
            cb(name, label);
        }

        let start = Instant::now();
        phase_fn(config, &mut kg, &mut st, &mut ns_index)?;
        timings.insert(name.to_string(), start.elapsed().as_secs_f64());
    }

    let total_ms = total_start.elapsed().as_secs_f64() * 1000.0;

    Ok(build_result(config, &kg, &st, &timings, total_ms))
}
