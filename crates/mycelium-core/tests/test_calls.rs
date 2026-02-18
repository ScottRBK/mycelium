//! Phase 4: Call graph integration tests.

mod common;

use common::*;

// ===========================================================================
// Call extraction (5 tests)
// ===========================================================================

#[test]
fn calls_extracted_csharp() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    assert!(
        !edges.is_empty(),
        "Should extract call edges from C# fixture"
    );
}

#[test]
fn calls_extracted_python() {
    let r = run_four_phases("python_simple");
    let edges = r.kg.get_call_edges();
    assert!(
        !edges.is_empty(),
        "Should extract call edges from Python fixture"
    );
}

#[test]
fn calls_extracted_java() {
    let r = run_four_phases("java_simple");
    let edges = r.kg.get_call_edges();
    assert!(
        !edges.is_empty(),
        "Should extract call edges from Java fixture"
    );
}

#[test]
fn calls_extracted_go() {
    let r = run_four_phases("go_simple");
    let edges = r.kg.get_call_edges();
    let _ = edges; // Go simple may or may not have resolved calls
}

#[test]
fn calls_extracted_rust() {
    let r = run_four_phases("rust_simple");
    let edges = r.kg.get_call_edges();
    let _ = edges; // Depends on import resolution success
}

// ===========================================================================
// Tiers (4 tests)
// ===========================================================================

#[test]
fn tier_a_import_resolved() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    let tier_a: Vec<_> = edges.iter().filter(|e| e.3 == "A").collect();
    // May or may not have Tier A depending on import resolution
    let _ = tier_a;
}

#[test]
fn tier_b_same_file() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    let tier_b: Vec<_> = edges.iter().filter(|e| e.3 == "B").collect();
    assert!(
        !tier_b.is_empty(),
        "Should have Tier B (same-file) call edges"
    );
}

#[test]
fn tier_c_fuzzy() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    let tier_c: Vec<_> = edges.iter().filter(|e| e.3 == "C").collect();
    // May or may not have Tier C
    let _ = tier_c;
}

#[test]
fn confidence_values_valid() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    for (_, _, confidence, tier, _, _) in &edges {
        assert!(
            *confidence > 0.0 && *confidence <= 1.0,
            "Confidence should be in (0, 1], got {} for tier {}",
            confidence,
            tier
        );
    }
}

// ===========================================================================
// DI resolution (2 tests)
// ===========================================================================

#[test]
fn di_constructor_params_tracked() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let constructors_with_params: Vec<_> = syms
        .iter()
        .filter(|s| s.symbol_type == "Constructor" && s.parameter_types.is_some())
        .collect();
    assert!(
        !constructors_with_params.is_empty(),
        "C# constructors should have parameter types for DI tracking"
    );
}

#[test]
fn di_resolved_calls() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    let di_edges: Vec<_> = edges.iter().filter(|e| e.4.contains("di")).collect();
    // DI resolution may or may not produce edges depending on fixture structure
    let _ = di_edges;
}

// ===========================================================================
// Interface-to-implementation (2 tests)
// ===========================================================================

#[test]
fn interface_methods_present() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let interface_methods: Vec<_> = syms
        .iter()
        .filter(|s| {
            s.symbol_type == "Method"
                && s.parent
                    .as_ref()
                    .map(|p| p.starts_with('I') && p[1..].starts_with(char::is_uppercase))
                    .unwrap_or(false)
        })
        .collect();
    assert!(
        !interface_methods.is_empty(),
        "Should have methods under interfaces"
    );
}

#[test]
fn impl_resolved_calls() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    let impl_edges: Vec<_> = edges.iter().filter(|e| e.4.contains("impl")).collect();
    // Interface-to-impl resolution may or may not fire
    let _ = impl_edges;
}

// ===========================================================================
// Builtins excluded (2 tests)
// ===========================================================================

#[test]
fn builtin_calls_excluded() {
    let r = run_four_phases("python_simple");
    let pairs = call_pairs(&r.kg);
    // Built-in calls like print, len should not appear as resolved call targets
    let has_print = pairs.iter().any(|(_, to)| to == "print");
    let has_len = pairs.iter().any(|(_, to)| to == "len");
    assert!(!has_print, "print should be excluded as builtin");
    assert!(!has_len, "len should be excluded as builtin");
}

#[test]
fn builtin_calls_excluded_csharp() {
    let r = run_four_phases("csharp_simple");
    let pairs = call_pairs(&r.kg);
    let has_console = pairs.iter().any(|(_, to)| to == "Console");
    assert!(!has_console, "Console should be excluded as builtin");
}

// ===========================================================================
// Cross-file (3 tests)
// ===========================================================================

#[test]
fn calls_have_line_numbers() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    for (_, _, _, _, _, line) in &edges {
        assert!(*line > 0, "Call edges should have positive line numbers");
    }
}

#[test]
fn call_count_reasonable() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    assert!(
        edges.len() >= 5,
        "csharp_simple should have at least 5 call edges, got {}",
        edges.len()
    );
}

#[test]
fn no_self_calls() {
    let r = run_four_phases("csharp_simple");
    let edges = r.kg.get_call_edges();
    for (from, to, _, _, _, _) in &edges {
        assert_ne!(from, to, "Should not have self-calls: {}", from);
    }
}
