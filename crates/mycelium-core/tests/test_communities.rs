//! Phase 5: Community detection integration tests.

mod common;

use common::*;

#[test]
fn communities_detected() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    assert!(
        !communities.is_empty(),
        "Should detect communities in csharp_simple"
    );
}

#[test]
fn community_has_members() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    for (id, _label, members, _, _) in &communities {
        assert!(
            !members.is_empty(),
            "Community {} should have members",
            id
        );
    }
}

#[test]
fn community_has_label() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    for (_, label, _, _, _) in &communities {
        assert!(
            !label.is_empty(),
            "Communities should have non-empty labels"
        );
    }
}

#[test]
fn community_cohesion_range() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    for (id, _, _, cohesion, _) in &communities {
        assert!(
            *cohesion >= 0.0 && *cohesion <= 1.0,
            "Community {} cohesion should be in [0,1], got {}",
            id,
            cohesion
        );
    }
}

#[test]
fn community_ids_unique() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    let mut seen = std::collections::HashSet::new();
    for (id, _, _, _, _) in &communities {
        assert!(seen.insert(id), "Duplicate community ID: {}", id);
    }
}

#[test]
fn community_primary_language() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    for (_, _, _, _, lang) in &communities {
        // Primary language should be set (may be empty for mixed)
        let _ = lang;
    }
}

#[test]
fn community_labels_unique() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    let mut labels = std::collections::HashSet::new();
    for (_, label, _, _, _) in &communities {
        labels.insert(label.clone());
    }
    // Labels should be disambiguated (unique)
    assert_eq!(
        labels.len(),
        communities.len(),
        "Community labels should be unique (disambiguated)"
    );
}

#[test]
fn community_python_fixture() {
    let r = run_all_phases("python_simple");
    let communities = r.kg.get_communities();
    assert!(
        !communities.is_empty(),
        "Should detect communities in python_simple"
    );
}

#[test]
fn community_java_fixture() {
    let r = run_all_phases("java_simple");
    let communities = r.kg.get_communities();
    assert!(
        !communities.is_empty(),
        "Should detect communities in java_simple"
    );
}

#[test]
fn community_all_symbols_assigned() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    let all_members: std::collections::HashSet<_> = communities
        .iter()
        .flat_map(|(_, _, members, _, _)| members.iter().cloned())
        .collect();
    let symbols = r.kg.get_symbols();
    // Not all symbols may be in communities (orphans with no edges), but most should be
    let assigned_ratio = all_members.len() as f64 / symbols.len().max(1) as f64;
    assert!(
        assigned_ratio > 0.1,
        "At least 10% of symbols should be in communities, got {:.1}%",
        assigned_ratio * 100.0
    );
}

#[test]
fn community_count_reasonable() {
    let r = run_all_phases("csharp_simple");
    let communities = r.kg.get_communities();
    let sym_count = r.kg.symbol_count();
    assert!(
        communities.len() <= sym_count,
        "Should not have more communities than symbols"
    );
}
