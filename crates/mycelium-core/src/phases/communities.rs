//! Phase 5: Community detection via Louvain algorithm.
//!
//! Pure Rust implementation — no external community detection library needed.

use std::collections::{HashMap, HashSet};

use crate::config::{AnalysisConfig, Community};
use crate::graph::knowledge_graph::KnowledgeGraph;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Run the communities phase: detect communities using Louvain clustering.
pub fn run_communities_phase(config: &AnalysisConfig, kg: &mut KnowledgeGraph) {
    // Build undirected weighted graph from call edges
    let call_edges = kg.get_call_edges();
    if call_edges.is_empty() {
        return;
    }

    let mut adj = AdjList::new();
    for (src, tgt, conf, _tier, _reason, _line) in &call_edges {
        adj.add_edge(src, tgt, *conf);
    }

    if adj.nodes.len() < 2 {
        return;
    }

    // Run Louvain with auto-tuning resolution
    let mut resolution = config.resolution;
    let max_resolution = 10.0;

    let mut communities = louvain(&adj, resolution);

    // Auto-tune: double resolution until largest community <= max_community_size
    let mut largest = communities.iter().map(|c| c.len()).max().unwrap_or(0);
    while largest > config.max_community_size && resolution < max_resolution {
        resolution *= 2.0;
        communities = louvain(&adj, resolution);
        largest = communities.iter().map(|c| c.len()).max().unwrap_or(0);
    }

    // Recursive splitting for any communities still over threshold
    let mut final_communities: Vec<Vec<String>> = Vec::new();
    for comm in communities {
        if comm.len() > config.max_community_size {
            let sub = split_oversized(&comm, &adj, config.max_community_size);
            final_communities.extend(sub);
        } else {
            final_communities.push(comm);
        }
    }

    // Build communities, track labels for disambiguation
    let mut label_counts: HashMap<String, usize> = HashMap::new();
    let mut pending: Vec<(String, Vec<String>, f64, String)> = Vec::new();

    for members in &final_communities {
        if members.len() <= 1 {
            continue;
        }

        let label = generate_label(members, kg);
        let cohesion = compute_cohesion(members, &adj);
        let primary_lang = primary_language(members, kg);

        *label_counts.entry(label.clone()).or_insert(0) += 1;
        pending.push((label, members.clone(), cohesion, primary_lang));
    }

    // Disambiguate duplicate labels
    let mut used_labels: HashSet<String> = HashSet::new();
    for (i, (label, members, cohesion, primary_lang)) in pending.into_iter().enumerate() {
        let final_label =
            if label_counts.get(&label).copied().unwrap_or(0) > 1 || used_labels.contains(&label) {
                disambiguate_label(&label, &members, kg, &used_labels)
            } else {
                label
            };
        used_labels.insert(final_label.clone());

        let community = Community {
            id: format!("community_{i}"),
            label: final_label,
            members,
            cohesion: (cohesion * 1000.0).round() / 1000.0,
            primary_language: primary_lang,
        };
        kg.add_community(&community);
    }
}

// ---------------------------------------------------------------------------
// Adjacency list for undirected weighted graph
// ---------------------------------------------------------------------------

struct AdjList {
    /// node_id -> index
    node_map: HashMap<String, usize>,
    /// index -> node_id
    nodes: Vec<String>,
    /// adjacency: index -> Vec<(neighbour_index, weight)>
    adj: Vec<Vec<(usize, f64)>>,
}

impl AdjList {
    fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
        }
    }

    fn ensure_node(&mut self, id: &str) -> usize {
        if let Some(&idx) = self.node_map.get(id) {
            idx
        } else {
            let idx = self.nodes.len();
            self.node_map.insert(id.to_string(), idx);
            self.nodes.push(id.to_string());
            self.adj.push(Vec::new());
            idx
        }
    }

    fn add_edge(&mut self, a: &str, b: &str, weight: f64) {
        let ai = self.ensure_node(a);
        let bi = self.ensure_node(b);
        // Check if edge already exists — add weight
        if let Some(entry) = self.adj[ai].iter_mut().find(|(n, _)| *n == bi) {
            entry.1 += weight;
        } else {
            self.adj[ai].push((bi, weight));
        }
        if let Some(entry) = self.adj[bi].iter_mut().find(|(n, _)| *n == ai) {
            entry.1 += weight;
        } else {
            self.adj[bi].push((ai, weight));
        }
    }

    fn total_weight(&self) -> f64 {
        let mut total = 0.0;
        for neighbours in &self.adj {
            for &(_, w) in neighbours {
                total += w;
            }
        }
        total / 2.0 // Each edge counted twice
    }
}

// ---------------------------------------------------------------------------
// Louvain algorithm
// ---------------------------------------------------------------------------

/// Run the Louvain community detection algorithm with multi-level aggregation.
///
/// Standard Louvain repeats two phases until convergence:
///   Phase 1 — local node moves to maximise modularity gain
///   Phase 2 — contract the graph (merge communities into super-nodes)
///
/// Returns a list of communities, each being a Vec of node IDs.
fn louvain(adj: &AdjList, resolution: f64) -> Vec<Vec<String>> {
    let n = adj.nodes.len();
    if n == 0 {
        return Vec::new();
    }

    let m = adj.total_weight();
    if m == 0.0 {
        // No edges: each node is its own community
        return adj.nodes.iter().map(|id| vec![id.clone()]).collect();
    }
    let m2 = m * 2.0; // constant across all levels

    // groups[i] = original-graph node indices belonging to current super-node i
    let mut groups: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();

    // Current-level compact adjacency (indices are super-node IDs at this level)
    let mut cur_adj: Vec<Vec<(usize, f64)>> = adj.adj.clone();
    let mut cur_n = n;

    // Outer loop: repeat Phase 1 + Phase 2 until convergence
    loop {
        if cur_n < 2 {
            break;
        }

        // Weighted degree at current level
        let degree: Vec<f64> = (0..cur_n)
            .map(|i| cur_adj[i].iter().map(|&(_, w)| w).sum())
            .collect();

        // ---- Phase 1: local node moves ----
        let mut community: Vec<usize> = (0..cur_n).collect();
        let mut sigma_tot: Vec<f64> = degree.clone();
        let mut any_moved = false;

        let mut improved = true;
        let mut iters = 0;
        while improved && iters < 100 {
            improved = false;
            iters += 1;

            for i in 0..cur_n {
                let ci = community[i];
                let ki = degree[i];

                // Sum of edge weights from i to each neighbouring community
                let mut comm_weights: HashMap<usize, f64> = HashMap::new();
                for &(j, w) in &cur_adj[i] {
                    if j == i {
                        continue; // skip self-loops
                    }
                    *comm_weights.entry(community[j]).or_insert(0.0) += w;
                }

                let ki_in = comm_weights.get(&ci).copied().unwrap_or(0.0);

                // Temporarily remove i from its community
                sigma_tot[ci] -= ki;

                let mut best_comm = ci;
                let mut best_gain = 0.0;

                for (&cj, &kj_in) in &comm_weights {
                    let gain = kj_in - resolution * sigma_tot[cj] * ki / m2;
                    let loss = ki_in - resolution * sigma_tot[ci] * ki / m2;
                    let delta = gain - loss;

                    if delta > best_gain || (delta == best_gain && cj < best_comm) {
                        best_gain = delta;
                        best_comm = cj;
                    }
                }

                if best_gain <= 0.0 {
                    best_comm = ci;
                }

                community[i] = best_comm;
                sigma_tot[best_comm] += ki;

                if best_comm != ci {
                    improved = true;
                    any_moved = true;
                }
            }
        }

        if !any_moved {
            break; // converged — no moves at this level
        }

        // Compact community labels to 0..new_n
        let mut label_map: HashMap<usize, usize> = HashMap::new();
        let mut next_label = 0usize;
        for &c in &community {
            label_map.entry(c).or_insert_with(|| {
                let l = next_label;
                next_label += 1;
                l
            });
        }
        let mapped: Vec<usize> = community.iter().map(|c| label_map[c]).collect();
        let new_n = next_label;

        if new_n == cur_n {
            break; // no contraction possible
        }

        // Merge original-node groups according to new communities
        let mut new_groups: Vec<Vec<usize>> = vec![Vec::new(); new_n];
        for (i, &c) in mapped.iter().enumerate() {
            new_groups[c].extend_from_slice(&groups[i]);
        }
        groups = new_groups;

        // ---- Phase 2: contract graph ----
        let mut new_adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); new_n];
        for i in 0..cur_n {
            let ci = mapped[i];
            for &(j, w) in &cur_adj[i] {
                let cj = mapped[j];
                if ci == cj {
                    continue; // drop intra-community edges
                }
                if let Some(entry) = new_adj[ci].iter_mut().find(|(nb, _)| *nb == cj) {
                    entry.1 += w;
                } else {
                    new_adj[ci].push((cj, w));
                }
            }
        }

        cur_adj = new_adj;
        cur_n = new_n;
    }

    // Convert groups back to original node IDs
    groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|idx| adj.nodes[idx].clone())
                .collect()
        })
        .collect()
}

/// Recursively split an oversized community using Louvain on its subgraph.
fn split_oversized(community: &[String], adj: &AdjList, max_size: usize) -> Vec<Vec<String>> {
    if community.len() <= max_size {
        return vec![community.to_vec()];
    }

    // Build subgraph
    let member_set: HashSet<&str> = community.iter().map(|s| s.as_str()).collect();
    let mut sub_adj = AdjList::new();

    for member in community {
        sub_adj.ensure_node(member);
    }

    for member in community {
        if let Some(&idx) = adj.node_map.get(member.as_str()) {
            for &(nbr_idx, w) in &adj.adj[idx] {
                let nbr = &adj.nodes[nbr_idx];
                if member_set.contains(nbr.as_str()) && nbr > member {
                    sub_adj.add_edge(member, nbr, w);
                }
            }
        }
    }

    if sub_adj.total_weight() == 0.0 {
        return vec![community.to_vec()];
    }

    // Try Louvain at increasing resolution
    let mut resolution = 2.0;
    for _ in 0..8 {
        let sub_communities = louvain(&sub_adj, resolution);
        if sub_communities.len() > 1 {
            let mut result = Vec::new();
            for sc in &sub_communities {
                result.extend(split_oversized(sc, adj, max_size));
            }
            return result;
        }
        resolution *= 2.0;
    }

    // Can't split further
    vec![community.to_vec()]
}

// ---------------------------------------------------------------------------
// Label generation + disambiguation
// ---------------------------------------------------------------------------

const STRIP_DIR_SEGMENTS: &[&str] = &["src", "source", "sourcecode", "lib", "app"];

/// Auto-generate a community label from member symbols.
fn generate_label(members: &[String], kg: &KnowledgeGraph) -> String {
    let member_set: HashSet<&str> = members.iter().map(|s| s.as_str()).collect();
    let mut file_paths = Vec::new();
    let mut names = Vec::new();
    let mut parents = Vec::new();

    for sym in kg.get_symbols() {
        if member_set.contains(sym.id.as_str()) {
            file_paths.push(sym.file.clone());
            names.push(sym.name.clone());
            if let Some(ref p) = sym.parent {
                parents.push(p.clone());
            }
        }
    }

    // Strategy 1: Most common parent (namespace/class) if >= 30% coverage
    if !parents.is_empty() {
        let mut parent_counts: HashMap<&str, usize> = HashMap::new();
        for p in &parents {
            *parent_counts.entry(p.as_str()).or_insert(0) += 1;
        }
        let (best_parent, count) = parent_counts
            .iter()
            .max_by_key(|&(_, c)| *c)
            .map(|(p, c)| (*p, *c))
            .unwrap();
        if count >= (members.len() * 3) / 10 {
            // Short name: last segment of dotted namespace
            let short = best_parent.rsplit('.').next().unwrap_or(best_parent);
            return short.to_string();
        }
    }

    // Strategy 2: Most specific directory component
    if !file_paths.is_empty() {
        let mut dir_counts: HashMap<String, usize> = HashMap::new();
        for fp in &file_paths {
            if let Some(pos) = fp.rfind('/') {
                let dir = &fp[..pos];
                *dir_counts.entry(dir.to_string()).or_insert(0) += 1;
            }
        }
        if let Some((best_dir, _)) = dir_counts.iter().max_by_key(|&(_, c)| *c) {
            let parts: Vec<&str> = best_dir
                .split('/')
                .filter(|p| !STRIP_DIR_SEGMENTS.contains(&p.to_lowercase().as_str()))
                .collect();
            if let Some(last) = parts.last() {
                return (*last).to_string();
            }
        }
    }

    // Strategy 3: Common name prefix if >= 3 chars
    if !names.is_empty() {
        let prefix = common_prefix(&names);
        if prefix.len() >= 3 {
            return prefix.trim_end_matches('_').to_string();
        }
    }

    format!("Community ({} members)", members.len())
}

/// Create a unique label when multiple communities share the same base label.
fn disambiguate_label(
    label: &str,
    members: &[String],
    kg: &KnowledgeGraph,
    used_labels: &HashSet<String>,
) -> String {
    let member_set: HashSet<&str> = members.iter().map(|s| s.as_str()).collect();
    let mut file_paths = Vec::new();
    let mut names = Vec::new();
    let mut parents = Vec::new();

    for sym in kg.get_symbols() {
        if member_set.contains(sym.id.as_str()) {
            file_paths.push(sym.file.clone());
            names.push(sym.name.clone());
            if let Some(ref p) = sym.parent {
                parents.push(p.clone());
            }
        }
    }

    // Try secondary parent
    if !parents.is_empty() {
        let mut parent_counts: HashMap<&str, usize> = HashMap::new();
        for p in &parents {
            *parent_counts.entry(p.as_str()).or_insert(0) += 1;
        }
        let mut sorted: Vec<_> = parent_counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        if sorted.len() > 1 {
            let secondary = sorted[1].0.rsplit('.').next().unwrap_or(sorted[1].0);
            let candidate = format!("{label}/{secondary}");
            if !used_labels.contains(&candidate) {
                return candidate;
            }
        }
    }

    // Try directory disambiguator
    if !file_paths.is_empty() {
        let mut dir_counts: HashMap<String, usize> = HashMap::new();
        for fp in &file_paths {
            if let Some(pos) = fp.rfind('/') {
                let dir = &fp[..pos];
                *dir_counts.entry(dir.to_string()).or_insert(0) += 1;
            }
        }
        if let Some((best_dir, _)) = dir_counts.iter().max_by_key(|&(_, c)| *c) {
            let parts: Vec<&str> = best_dir
                .split('/')
                .filter(|p| !STRIP_DIR_SEGMENTS.contains(&p.to_lowercase().as_str()) && *p != label)
                .collect();
            if let Some(last) = parts.last() {
                let candidate = format!("{label}/{last}");
                if !used_labels.contains(&candidate) {
                    return candidate;
                }
            }
        }
    }

    // Try distinguishing member name
    if !names.is_empty() {
        let mut sorted_names = names.clone();
        sorted_names.sort_by_key(|b| std::cmp::Reverse(b.len()));
        for name in &sorted_names {
            if name != label {
                let candidate = format!("{label}:{name}");
                if !used_labels.contains(&candidate) {
                    return candidate;
                }
            }
        }
    }

    // Fallback: append ordinal
    let mut idx = 1;
    loop {
        let candidate = format!("{label} #{idx}");
        if !used_labels.contains(&candidate) {
            return candidate;
        }
        idx += 1;
    }
}

/// Compute internal edge density (cohesion) for a community.
fn compute_cohesion(members: &[String], adj: &AdjList) -> f64 {
    let n = members.len();
    if n < 2 {
        return 0.0;
    }

    let member_set: HashSet<&str> = members.iter().map(|s| s.as_str()).collect();
    let mut internal_edges = 0usize;

    for member in members {
        if let Some(&idx) = adj.node_map.get(member.as_str()) {
            for &(nbr_idx, _) in &adj.adj[idx] {
                if member_set.contains(adj.nodes[nbr_idx].as_str()) {
                    internal_edges += 1;
                }
            }
        }
    }

    // Each edge counted twice in undirected graph
    internal_edges /= 2;
    let max_possible = n * (n - 1) / 2;
    if max_possible == 0 {
        return 0.0;
    }

    internal_edges as f64 / max_possible as f64
}

/// Determine the primary language among community members.
fn primary_language(members: &[String], kg: &KnowledgeGraph) -> String {
    let member_set: HashSet<&str> = members.iter().map(|s| s.as_str()).collect();
    let mut lang_counts: HashMap<String, usize> = HashMap::new();

    for sym in kg.get_symbols() {
        if member_set.contains(sym.id.as_str()) {
            if let Some(ref lang) = sym.language {
                *lang_counts.entry(lang.clone()).or_insert(0) += 1;
            }
        }
    }

    lang_counts
        .into_iter()
        .max_by_key(|&(_, c)| c)
        .map(|(l, _)| l)
        .unwrap_or_default()
}

/// Find common prefix of a list of strings.
fn common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = &strings[0];
    let mut len = first.len();
    for s in &strings[1..] {
        len = len.min(s.len());
        for (i, (a, b)) in first.bytes().zip(s.bytes()).enumerate() {
            if a != b {
                len = len.min(i);
                break;
            }
        }
    }
    first[..len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a simple adj list for testing
    fn build_test_adj(edges: &[(&str, &str, f64)]) -> AdjList {
        let mut adj = AdjList::new();
        for &(a, b, w) in edges {
            adj.add_edge(a, b, w);
        }
        adj
    }

    #[test]
    fn louvain_two_cliques() {
        // Two separate cliques should be detected
        let adj = build_test_adj(&[
            ("a1", "a2", 1.0),
            ("a2", "a3", 1.0),
            ("a1", "a3", 1.0),
            ("b1", "b2", 1.0),
            ("b2", "b3", 1.0),
            ("b1", "b3", 1.0),
        ]);
        let communities = louvain(&adj, 1.0);
        assert!(
            communities.len() >= 2,
            "Should detect at least 2 communities"
        );
    }

    #[test]
    fn louvain_single_node() {
        let mut adj = AdjList::new();
        adj.ensure_node("lonely");
        let communities = louvain(&adj, 1.0);
        assert_eq!(communities.len(), 1);
    }

    #[test]
    fn louvain_empty() {
        let adj = AdjList::new();
        let communities = louvain(&adj, 1.0);
        assert!(communities.is_empty());
    }

    #[test]
    fn louvain_fully_connected() {
        // Fully connected graph should produce 1 community
        let adj = build_test_adj(&[
            ("a", "b", 1.0),
            ("b", "c", 1.0),
            ("a", "c", 1.0),
        ]);
        let communities = louvain(&adj, 1.0);
        assert_eq!(communities.len(), 1);
    }

    #[test]
    fn compute_cohesion_complete() {
        let adj = build_test_adj(&[
            ("a", "b", 1.0),
            ("b", "c", 1.0),
            ("a", "c", 1.0),
        ]);
        let members: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let cohesion = compute_cohesion(&members, &adj);
        assert!((cohesion - 1.0).abs() < 0.01, "Complete graph cohesion = 1.0");
    }

    #[test]
    fn compute_cohesion_sparse() {
        let adj = build_test_adj(&[("a", "b", 1.0)]);
        let members: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let cohesion = compute_cohesion(&members, &adj);
        // 1 edge out of 3 possible = 0.333
        assert!(cohesion < 0.5, "Sparse graph should have low cohesion");
    }

    #[test]
    fn compute_cohesion_single_member() {
        let adj = AdjList::new();
        let members: Vec<String> = vec!["a".into()];
        assert_eq!(compute_cohesion(&members, &adj), 0.0);
    }

    #[test]
    fn common_prefix_basic() {
        let strings = vec![
            "UserService".to_string(),
            "UserController".to_string(),
            "UserRepository".to_string(),
        ];
        assert_eq!(common_prefix(&strings), "User");
    }

    #[test]
    fn common_prefix_empty() {
        let strings: Vec<String> = vec![];
        assert_eq!(common_prefix(&strings), "");
    }

    #[test]
    fn split_oversized_basic() {
        // Build a graph with two cliques connected by one weak edge
        let adj = build_test_adj(&[
            ("a1", "a2", 5.0),
            ("a2", "a3", 5.0),
            ("a1", "a3", 5.0),
            ("b1", "b2", 5.0),
            ("b2", "b3", 5.0),
            ("b1", "b3", 5.0),
            ("a3", "b1", 0.1),
        ]);
        let all: Vec<String> = vec![
            "a1".into(),
            "a2".into(),
            "a3".into(),
            "b1".into(),
            "b2".into(),
            "b3".into(),
        ];
        let result = split_oversized(&all, &adj, 3);
        assert!(
            result.len() >= 2,
            "Should split into at least 2 sub-communities"
        );
    }

    #[test]
    fn total_weight_correct() {
        let adj = build_test_adj(&[("a", "b", 2.0), ("b", "c", 3.0)]);
        assert!((adj.total_weight() - 5.0).abs() < 0.001);
    }
}
