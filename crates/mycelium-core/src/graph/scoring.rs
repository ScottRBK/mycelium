//! Entry point scoring for process detection.

use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::graph::knowledge_graph::KnowledgeGraph;

/// Name patterns that suggest entry points.
static ENTRY_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i).*Controller$").unwrap(),
        Regex::new(r"(?i).*Handler$").unwrap(),
        Regex::new(r"(?i).*Endpoint$").unwrap(),
        Regex::new(r"(?i).*Middleware$").unwrap(),
        Regex::new(r"(?i)^Main$").unwrap(),
        Regex::new(r"(?i)^Startup$").unwrap(),
        Regex::new(r"(?i)^Configure.*$").unwrap(),
        Regex::new(r"(?i)^Map.*Endpoints$").unwrap(),
        Regex::new(r"(?i).*Route$").unwrap(),
        Regex::new(r"(?i).*Listener$").unwrap(),
        Regex::new(r"(?i)^handle.*$").unwrap(),
        Regex::new(r"^on[A-Z].*$").unwrap(),
        Regex::new(r"(?i)^process.*$").unwrap(),
    ]
});

/// Path segments that indicate utility functions.
static UTILITY_SEGMENTS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    [
        "utils",
        "helpers",
        "extensions",
        "common",
        "shared",
        "utilities",
    ]
    .into_iter()
    .collect()
});

/// Path patterns that indicate test files.
static TEST_PATH_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)(?:^|[/\\])tests?[/\\]").unwrap(),
        Regex::new(r"(?i)(?:^|[/\\])specs?[/\\]").unwrap(),
        Regex::new(r"(?i)(?:^|[/\\])__tests__[/\\]").unwrap(),
        Regex::new(r"(?i)(?:^|[/\\])TestHarness[/\\]").unwrap(),
        Regex::new(r"(?i)(?:Tests?|Specs?|_test|_spec)\.").unwrap(),
        Regex::new(r"(?i)\.Tests?[/\\]").unwrap(),
    ]
});

/// Framework types that should never be entry points.
static FRAMEWORK_TYPE_EXCLUSIONS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    [
        "Task",
        "ValueTask",
        "ILogger",
        "IConfiguration",
        "IServiceCollection",
        "IServiceProvider",
        "CancellationToken",
        "HttpClient",
    ]
    .into_iter()
    .collect()
});

/// Quick BFS probe to measure reachable depth from a symbol.
fn probe_depth(kg: &KnowledgeGraph, sym_id: &str, max_hops: usize) -> usize {
    let mut visited = HashSet::new();
    visited.insert(sym_id.to_string());
    let mut frontier: Vec<String> = vec![sym_id.to_string()];
    let mut depth = 0;

    for _ in 0..max_hops {
        let mut next_frontier = Vec::new();
        for node in &frontier {
            for callee in kg.get_callees(node) {
                if !visited.contains(&callee.id) {
                    visited.insert(callee.id.clone());
                    next_frontier.push(callee.id);
                }
            }
        }
        if next_frontier.is_empty() {
            break;
        }
        frontier = next_frontier;
        depth += 1;
    }
    depth
}

/// Score all symbols as potential entry points.
///
/// Returns sorted (symbol_id, score) pairs, highest score first.
///
/// score = base_score * export_multiplier * name_multiplier * utility_penalty * depth_bonus
pub fn score_entry_points(kg: &KnowledgeGraph) -> Vec<(String, f64)> {
    let mut scores: Vec<(String, f64)> = Vec::new();

    for sym in kg.get_symbols() {
        // Only score methods, functions, constructors
        if sym.symbol_type != "Method"
            && sym.symbol_type != "Function"
            && sym.symbol_type != "Constructor"
        {
            continue;
        }

        // Skip framework types
        if FRAMEWORK_TYPE_EXCLUSIONS.contains(sym.name.as_str()) {
            continue;
        }

        // Skip test file symbols
        if TEST_PATH_PATTERNS.iter().any(|p| p.is_match(&sym.file)) {
            continue;
        }

        // Base score: callees / (callers + 1)
        let callees = kg.get_callees(&sym.id);
        let callers = kg.get_callers(&sym.id);
        let out_degree = callees.len() as f64;
        let in_degree = callers.len() as f64;
        let base_score = out_degree / (in_degree + 1.0);

        if base_score == 0.0 {
            continue;
        }

        // Export multiplier
        let export_mult = if sym.exported { 2.0 } else { 1.0 };

        // Name multiplier
        let mut name_mult: f64 = 1.0;
        for pattern in ENTRY_PATTERNS.iter() {
            if pattern.is_match(&sym.name) {
                name_mult = 1.5;
                break;
            }
        }

        // Also check parent class name for controller patterns
        if let Some(ref parent) = sym.parent {
            for pattern in ENTRY_PATTERNS.iter() {
                if pattern.is_match(parent) {
                    name_mult = name_mult.max(1.3);
                    break;
                }
            }
        }

        // Utility penalty
        let mut utility_penalty = 1.0;
        let file_lower = sym.file.to_lowercase();
        for segment in UTILITY_SEGMENTS.iter() {
            if file_lower.contains(segment) {
                utility_penalty = 0.3;
                break;
            }
        }

        // Depth bonus: reward symbols that can reach deeper call chains
        let depth = probe_depth(kg, &sym.id, 3);
        let depth_bonus = 1.0 + (depth as f64 * 0.5);

        let score = base_score * export_mult * name_mult * utility_penalty * depth_bonus;
        scores.push((sym.id.clone(), score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scores
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CallEdge, Symbol, SymbolType, Visibility};

    fn make_method(id: &str, name: &str, file: &str, exported: bool) -> Symbol {
        Symbol {
            id: id.to_string(),
            name: name.to_string(),
            symbol_type: SymbolType::Method,
            file: file.to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported,
            parent: None,
            language: Some("C#".to_string()),
            byte_range: None,
            parameter_types: None,
        }
    }

    #[test]
    fn entry_point_scoring_basic() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method(
            "sym:A",
            "HandleRequest",
            "api/handler.cs",
            true,
        ));
        kg.add_symbol(&make_method("sym:B", "Process", "services/worker.cs", true));
        kg.add_symbol(&make_method("sym:C", "Helper", "utils/helper.cs", false));
        kg.add_call(&CallEdge {
            from_symbol: "sym:A".to_string(),
            to_symbol: "sym:B".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 5,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:B".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 10,
        });

        let scores = score_entry_points(&kg);
        assert!(!scores.is_empty());
        // HandleRequest should score highest (exported + entry pattern + depth)
        assert_eq!(scores[0].0, "sym:A");
    }

    #[test]
    fn test_files_excluded() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:T", "RunTest", "tests/test_main.cs", true));
        kg.add_symbol(&make_method("sym:R", "Run", "src/main.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:T".to_string(),
            to_symbol: "sym:R".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "test".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:R".to_string(),
            to_symbol: "sym:T".to_string(),
            confidence: 0.5,
            tier: "C".to_string(),
            reason: "fuzzy".to_string(),
            line: 1,
        });

        let scores = score_entry_points(&kg);
        // test file symbol should be excluded
        assert!(scores.iter().all(|(id, _)| id != "sym:T"));
    }

    #[test]
    fn export_multiplier() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:Pub", "Run", "src/main.cs", true));
        kg.add_symbol(&make_method(
            "sym:Priv",
            "RunPrivate",
            "src/main.cs",
            false,
        ));
        kg.add_symbol(&make_method("sym:C", "Target", "src/target.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:Pub".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:Priv".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        let pub_score = scores.iter().find(|(id, _)| id == "sym:Pub").unwrap().1;
        let priv_score = scores.iter().find(|(id, _)| id == "sym:Priv").unwrap().1;
        assert!(pub_score > priv_score, "Exported should score higher");
    }

    #[test]
    fn name_pattern_multiplier() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method(
            "sym:Handler",
            "RequestHandler",
            "src/api.cs",
            true,
        ));
        kg.add_symbol(&make_method("sym:Worker", "DoWork", "src/worker.cs", true));
        kg.add_symbol(&make_method("sym:C", "Target", "src/target.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:Handler".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:Worker".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        let handler_score = scores
            .iter()
            .find(|(id, _)| id == "sym:Handler")
            .unwrap()
            .1;
        let worker_score = scores.iter().find(|(id, _)| id == "sym:Worker").unwrap().1;
        assert!(
            handler_score > worker_score,
            "Handler pattern should score higher"
        );
    }

    #[test]
    fn utility_penalty() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method(
            "sym:U",
            "FormatDate",
            "utils/formatter.cs",
            true,
        ));
        kg.add_symbol(&make_method("sym:S", "Process", "services/worker.cs", true));
        kg.add_symbol(&make_method("sym:C", "Target", "src/target.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:U".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:S".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        let util_score = scores.iter().find(|(id, _)| id == "sym:U").unwrap().1;
        let service_score = scores.iter().find(|(id, _)| id == "sym:S").unwrap().1;
        assert!(
            service_score > util_score,
            "Utility files should score lower"
        );
    }

    #[test]
    fn depth_bonus() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:Deep", "DeepCaller", "src/api.cs", true));
        kg.add_symbol(&make_method("sym:B", "MidCall", "src/mid.cs", true));
        kg.add_symbol(&make_method("sym:C", "LeafCall", "src/leaf.cs", true));
        kg.add_symbol(&make_method(
            "sym:Shallow",
            "ShallowCaller",
            "src/shallow.cs",
            true,
        ));
        kg.add_symbol(&make_method("sym:D", "LeafOnly", "src/leaf2.cs", true));
        // Deep chain: Deep -> B -> C
        kg.add_call(&CallEdge {
            from_symbol: "sym:Deep".to_string(),
            to_symbol: "sym:B".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:B".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        // Shallow chain: Shallow -> D
        kg.add_call(&CallEdge {
            from_symbol: "sym:Shallow".to_string(),
            to_symbol: "sym:D".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        let deep_score = scores.iter().find(|(id, _)| id == "sym:Deep").unwrap().1;
        let shallow_score = scores
            .iter()
            .find(|(id, _)| id == "sym:Shallow")
            .unwrap()
            .1;
        assert!(
            deep_score > shallow_score,
            "Deeper call chains should score higher"
        );
    }

    #[test]
    fn framework_types_excluded() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:Task", "Task", "src/main.cs", true));
        kg.add_symbol(&make_method("sym:Other", "Other", "src/other.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:Task".to_string(),
            to_symbol: "sym:Other".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        assert!(
            scores.iter().all(|(id, _)| id != "sym:Task"),
            "Framework types should be excluded"
        );
    }

    #[test]
    fn zero_out_degree_excluded() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:A", "NoCalls", "src/main.cs", true));
        // No call edges at all
        let scores = score_entry_points(&kg);
        assert!(scores.is_empty(), "Symbols with zero out-degree excluded");
    }

    #[test]
    fn scores_sorted_descending() {
        let mut kg = KnowledgeGraph::new();
        kg.add_symbol(&make_method("sym:A", "Handler", "src/api.cs", true));
        kg.add_symbol(&make_method("sym:B", "Worker", "src/worker.cs", true));
        kg.add_symbol(&make_method("sym:C", "Target", "src/target.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:A".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:B".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        for window in scores.windows(2) {
            assert!(
                window[0].1 >= window[1].1,
                "Scores should be sorted descending"
            );
        }
    }

    #[test]
    fn parent_controller_pattern() {
        let mut kg = KnowledgeGraph::new();
        let mut method = make_method("sym:A", "Index", "src/api.cs", true);
        method.parent = Some("UserController".to_string());
        kg.add_symbol(&method);
        let mut method2 = make_method("sym:B", "Index", "src/other.cs", true);
        method2.parent = Some("UserHelper".to_string());
        kg.add_symbol(&method2);
        kg.add_symbol(&make_method("sym:C", "Target", "src/target.cs", true));
        kg.add_call(&CallEdge {
            from_symbol: "sym:A".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        kg.add_call(&CallEdge {
            from_symbol: "sym:B".to_string(),
            to_symbol: "sym:C".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import".to_string(),
            line: 1,
        });
        let scores = score_entry_points(&kg);
        let controller_score = scores.iter().find(|(id, _)| id == "sym:A").unwrap().1;
        let helper_score = scores.iter().find(|(id, _)| id == "sym:B").unwrap().1;
        assert!(
            controller_score > helper_score,
            "Parent controller pattern should boost score"
        );
    }
}
