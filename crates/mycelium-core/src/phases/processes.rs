//! Phase 6: Multi-branch BFS trace detection.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::config::{AnalysisConfig, Process};
use crate::graph::knowledge_graph::KnowledgeGraph;
use crate::graph::scoring::score_entry_points;

/// Run the processes phase: trace execution flows from scored entry points.
pub fn run_processes_phase(config: &AnalysisConfig, kg: &mut KnowledgeGraph) {
    let max_processes = config.max_processes;
    let max_depth = config.max_depth;
    let max_branching = config.max_branching;
    let min_steps = config.min_steps;

    // Score entry points
    let entry_points = score_entry_points(kg);
    if entry_points.is_empty() {
        return;
    }

    // Take top N candidates (2x max to allow for deduplication)
    let candidates: Vec<_> = entry_points.into_iter().take(max_processes * 2).collect();

    // BFS from each entry point (multi-branch)
    let mut traces: Vec<Vec<String>> = Vec::new();
    for (entry_id, _score) in &candidates {
        let new_traces = bfs_traces(kg, entry_id, max_depth, max_branching, min_steps);
        traces.extend(new_traces);
    }

    // Deduplicate
    traces = deduplicate(traces);

    // Build community membership map for classification
    let community_map = build_community_map(kg);

    // Compute confidence for each trace
    let mut process_data: Vec<(Vec<String>, f64)> = traces
        .into_iter()
        .map(|trace| {
            let conf = compute_total_confidence(kg, &trace);
            (trace, conf)
        })
        .collect();

    // Sort by normalised confidence (geometric mean per hop), tiebreak by length
    process_data.sort_by(|a, b| {
        let key_a = sort_key(&a.0, a.1);
        let key_b = sort_key(&b.0, b.1);
        key_b
            .partial_cmp(&key_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Depth-diverse selection: prioritise multi-step traces
    let deep: Vec<_> = process_data
        .iter()
        .filter(|(t, _)| t.len() > 2)
        .cloned()
        .collect();
    let shallow: Vec<_> = process_data
        .iter()
        .filter(|(t, _)| t.len() <= 2)
        .cloned()
        .collect();
    let max_deep = max_processes / 2;
    let selected_deep: Vec<_> = deep.into_iter().take(max_deep).collect();
    let remaining = max_processes - selected_deep.len();
    let mut selected: Vec<_> = selected_deep;
    selected.extend(shallow.into_iter().take(remaining));

    // Re-sort by normalised confidence
    selected.sort_by(|a, b| {
        let key_a = sort_key(&a.0, a.1);
        let key_b = sort_key(&b.0, b.1);
        key_b
            .partial_cmp(&key_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Create Process objects
    for (i, (trace, total_conf)) in selected.into_iter().enumerate() {
        let process_type = classify_process(&trace, &community_map);
        let process = Process {
            id: format!("process_{i}"),
            entry: trace[0].clone(),
            terminal: trace.last().cloned().unwrap_or_default(),
            steps: trace,
            process_type,
            total_confidence: (total_conf * 10000.0).round() / 10000.0,
        };
        kg.add_process(&process);
    }
}

// ---------------------------------------------------------------------------
// BFS trace collection
// ---------------------------------------------------------------------------

/// Multi-branch BFS from a starting symbol.
///
/// Returns multiple traces per entry point. Each trace follows a different
/// branch of callees. Per-path cycle detection allows two paths to visit
/// the same node.
fn bfs_traces(
    kg: &KnowledgeGraph,
    start: &str,
    max_depth: usize,
    max_branching: usize,
    min_steps: usize,
) -> Vec<Vec<String>> {
    let mut traces: Vec<Vec<String>> = Vec::new();
    let max_traces = max_branching * 3;
    let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
    queue.push_back((start.to_string(), vec![start.to_string()]));

    while let Some((current, path)) = queue.pop_front() {
        if traces.len() >= max_traces {
            break;
        }

        let callees = kg.get_callees(&current);
        if callees.is_empty() || path.len() >= max_depth {
            if path.len() >= min_steps {
                traces.push(path);
            }
            continue;
        }

        // Sort by confidence descending
        let mut sorted_callees = callees;
        sorted_callees.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut extended = false;
        for callee in sorted_callees.iter().take(max_branching) {
            if !path.contains(&callee.id) {
                let mut new_path = path.clone();
                new_path.push(callee.id.clone());
                queue.push_back((callee.id.clone(), new_path));
                extended = true;
            }
        }

        if !extended && path.len() >= min_steps {
            traces.push(path);
        }
    }

    traces
}

// ---------------------------------------------------------------------------
// Deduplication
// ---------------------------------------------------------------------------

/// Remove traces that are strict subsequences of longer traces.
fn deduplicate(mut traces: Vec<Vec<String>>) -> Vec<Vec<String>> {
    traces.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut result: Vec<Vec<String>> = Vec::new();
    for trace in traces {
        let trace_set: HashSet<&str> = trace.iter().map(|s| s.as_str()).collect();
        let is_subset = result.iter().any(|existing| {
            let existing_set: HashSet<&str> = existing.iter().map(|s| s.as_str()).collect();
            trace_set.is_subset(&existing_set) && trace_set != existing_set
        });
        if !is_subset {
            result.push(trace);
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Classification + confidence
// ---------------------------------------------------------------------------

/// Build symbol_id -> community_id mapping.
fn build_community_map(kg: &KnowledgeGraph) -> HashMap<String, String> {
    let mut community_map = HashMap::new();
    for (comm_id, _label, members, _cohesion, _lang) in kg.get_communities() {
        for member in members {
            community_map.insert(member, comm_id.clone());
        }
    }
    community_map
}

/// Classify process as intra_community or cross_community.
fn classify_process(trace: &[String], community_map: &HashMap<String, String>) -> String {
    let communities_seen: HashSet<&str> = trace
        .iter()
        .filter_map(|s| community_map.get(s).map(|c| c.as_str()))
        .collect();

    if communities_seen.len() <= 1 {
        "intra_community".to_string()
    } else {
        "cross_community".to_string()
    }
}

/// Compute total confidence as product of edge confidences along the trace.
fn compute_total_confidence(kg: &KnowledgeGraph, trace: &[String]) -> f64 {
    if trace.len() < 2 {
        return 1.0;
    }

    let mut total = 1.0;
    for i in 0..trace.len() - 1 {
        let callees = kg.get_callees(&trace[i]);
        let edge_conf = callees
            .iter()
            .find(|c| c.id == trace[i + 1])
            .map(|c| c.confidence)
            .unwrap_or(0.5);
        total *= edge_conf;
    }

    total
}

/// Sort key: (normalised_confidence, trace_length).
fn sort_key(trace: &[String], total_conf: f64) -> (f64, usize) {
    let n_edges = trace.len().saturating_sub(1);
    if n_edges == 0 {
        return (1.0, 0);
    }
    let normalised = total_conf.powf(1.0 / n_edges as f64);
    (normalised, trace.len())
}
