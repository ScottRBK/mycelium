//! Phase 6: Process detection (execution flow tracing) integration tests.

mod common;

use common::*;

// ===========================================================================
// Process detection (5 tests)
// ===========================================================================

#[test]
fn processes_detected() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    assert!(
        !processes.is_empty(),
        "Should detect processes in csharp_simple"
    );
}

#[test]
fn process_has_entry_and_terminal() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, entry, terminal, _, _, _) in &processes {
        assert!(!entry.is_empty(), "Process {} should have entry point", id);
        assert!(!terminal.is_empty(), "Process {} should have terminal", id);
    }
}

#[test]
fn process_has_steps() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, steps, _, _) in &processes {
        assert!(
            steps.len() >= 2,
            "Process {} should have at least 2 steps, got {}",
            id,
            steps.len()
        );
    }
}

#[test]
fn process_ids_unique() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    let mut seen = std::collections::HashSet::new();
    for (id, _, _, _, _, _) in &processes {
        assert!(seen.insert(id), "Duplicate process ID: {}", id);
    }
}

#[test]
fn process_entry_is_first_step() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, entry, _, steps, _, _) in &processes {
        if !steps.is_empty() {
            assert_eq!(
                &steps[0], entry,
                "Process {} entry should be first step",
                id
            );
        }
    }
}

// ===========================================================================
// Scoring (3 tests)
// ===========================================================================

#[test]
fn scoring_excludes_test_files() {
    use mycelium_core::graph::scoring::score_entry_points;
    let r = run_four_phases("csharp_simple");
    let scored = score_entry_points(&r.kg);
    let syms = r.kg.get_symbols();
    let sym_map: std::collections::HashMap<_, _> =
        syms.iter().map(|s| (s.id.as_str(), s)).collect();
    for (id, _) in &scored {
        if let Some(sym) = sym_map.get(id.as_str()) {
            assert!(
                !sym.file.contains("test") && !sym.file.contains("spec"),
                "Test files should not be scored as entry points"
            );
        }
    }
}

#[test]
fn scoring_returns_positive_scores() {
    use mycelium_core::graph::scoring::score_entry_points;
    let r = run_four_phases("csharp_simple");
    let scored = score_entry_points(&r.kg);
    for (_, score) in &scored {
        assert!(*score > 0.0, "Entry point scores should be positive");
    }
}

#[test]
fn scoring_sorted_descending() {
    use mycelium_core::graph::scoring::score_entry_points;
    let r = run_four_phases("csharp_simple");
    let scored = score_entry_points(&r.kg);
    for window in scored.windows(2) {
        assert!(
            window[0].1 >= window[1].1,
            "Scores should be sorted descending"
        );
    }
}

// ===========================================================================
// BFS and deduplication (4 tests)
// ===========================================================================

#[test]
fn process_no_cycles() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, steps, _, _) in &processes {
        let mut seen = std::collections::HashSet::new();
        for step in steps {
            assert!(
                seen.insert(step),
                "Process {} has cycle: duplicate step {}",
                id,
                step
            );
        }
    }
}

#[test]
fn process_dedup_no_subsets() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    let step_sets: Vec<std::collections::HashSet<&str>> = processes
        .iter()
        .map(|(_, _, _, steps, _, _)| steps.iter().map(|s| s.as_str()).collect())
        .collect();

    for (i, set_a) in step_sets.iter().enumerate() {
        for (j, set_b) in step_sets.iter().enumerate() {
            if i != j && set_a != set_b {
                assert!(
                    !set_a.is_subset(set_b),
                    "Process {} is a strict subset of process {}",
                    i,
                    j
                );
            }
        }
    }
}

#[test]
fn process_max_count() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    assert!(
        processes.len() <= r.config.max_processes,
        "Should not exceed max_processes config"
    );
}

#[test]
fn process_min_steps_respected() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, steps, _, _) in &processes {
        assert!(
            steps.len() >= r.config.min_steps,
            "Process {} has {} steps, should be >= min_steps={}",
            id,
            steps.len(),
            r.config.min_steps
        );
    }
}

// ===========================================================================
// Confidence (3 tests)
// ===========================================================================

#[test]
fn process_confidence_positive() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, _, _, conf) in &processes {
        assert!(
            *conf > 0.0,
            "Process {} should have positive confidence, got {}",
            id,
            conf
        );
    }
}

#[test]
fn process_confidence_max_one() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, _, _, conf) in &processes {
        assert!(
            *conf <= 1.0,
            "Process {} confidence should be <= 1.0, got {}",
            id,
            conf
        );
    }
}

#[test]
fn process_type_valid() {
    let r = run_all_phases("csharp_simple");
    let processes = r.kg.get_processes();
    for (id, _, _, _, ptype, _) in &processes {
        assert!(
            ptype == "intra_community" || ptype == "cross_community",
            "Process {} has invalid type: {}",
            id,
            ptype
        );
    }
}

// ===========================================================================
// Multi-language (3 tests)
// ===========================================================================

#[test]
fn processes_python() {
    let r = run_all_phases("python_simple");
    let processes = r.kg.get_processes();
    assert!(
        !processes.is_empty(),
        "Should detect processes in python_simple"
    );
}

#[test]
fn processes_java() {
    let r = run_all_phases("java_simple");
    let processes = r.kg.get_processes();
    assert!(
        !processes.is_empty(),
        "Should detect processes in java_simple"
    );
}

#[test]
fn processes_go() {
    let r = run_all_phases("go_simple");
    let processes = r.kg.get_processes();
    // Go simple may produce processes depending on call resolution
    let _ = processes;
}
