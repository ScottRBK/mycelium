"""Entry point scoring for process detection."""

from __future__ import annotations

import os
import re

from mycelium.graph.knowledge_graph import KnowledgeGraph

# Name patterns that suggest entry points
_ENTRY_PATTERNS = [
    re.compile(r".*Controller$", re.IGNORECASE),
    re.compile(r".*Handler$", re.IGNORECASE),
    re.compile(r".*Endpoint$", re.IGNORECASE),
    re.compile(r".*Middleware$", re.IGNORECASE),
    re.compile(r"^Main$", re.IGNORECASE),
    re.compile(r"^Startup$", re.IGNORECASE),
    re.compile(r"^Configure.*$", re.IGNORECASE),
    re.compile(r"^Map.*Endpoints$", re.IGNORECASE),
    re.compile(r".*Route$", re.IGNORECASE),
    re.compile(r".*Listener$", re.IGNORECASE),
    re.compile(r"^handle.*$", re.IGNORECASE),
    re.compile(r"^on[A-Z].*$"),
    re.compile(r"^process.*$", re.IGNORECASE),
]

# Path segments that indicate utility functions
_UTILITY_SEGMENTS = {"utils", "helpers", "extensions", "common", "shared", "utilities"}

# Path patterns that indicate test files
_TEST_PATH_PATTERNS = [
    re.compile(r"(?:^|[/\\])tests?[/\\]", re.IGNORECASE),
    re.compile(r"(?:^|[/\\])specs?[/\\]", re.IGNORECASE),
    re.compile(r"(?:^|[/\\])__tests__[/\\]", re.IGNORECASE),
    re.compile(r"(?:^|[/\\])TestHarness[/\\]", re.IGNORECASE),
    re.compile(r"(?:Tests?|Specs?|_test|_spec)\.", re.IGNORECASE),
    re.compile(r"\.Tests?[/\\]", re.IGNORECASE),  # dot-separated: Project.Tests/
]

# Framework types that should never be entry points
_FRAMEWORK_TYPE_EXCLUSIONS = {
    "Task", "ValueTask", "ILogger", "IConfiguration",
    "IServiceCollection", "IServiceProvider", "CancellationToken", "HttpClient",
}


def _probe_depth(kg: KnowledgeGraph, sym_id: str, max_hops: int = 3) -> int:
    """Quick BFS probe to measure reachable depth from a symbol."""
    visited = {sym_id}
    frontier = [sym_id]
    depth = 0
    for _ in range(max_hops):
        next_frontier = []
        for node in frontier:
            for callee in kg.get_callees(node):
                cid = callee["id"]
                if cid not in visited:
                    visited.add(cid)
                    next_frontier.append(cid)
        if not next_frontier:
            break
        frontier = next_frontier
        depth += 1
    return depth


def score_entry_points(kg: KnowledgeGraph) -> list[tuple[str, float]]:
    """Score all symbols as potential entry points.

    Returns sorted (symbol_id, score) pairs, highest score first.

    score = base_score * export_multiplier * name_multiplier * utility_penalty * depth_bonus
    """
    scores: list[tuple[str, float]] = []

    for sym in kg.get_symbols():
        sym_id = sym["id"]
        name = sym.get("name", "")
        file_path = sym.get("file", "")
        exported = sym.get("exported", False)
        sym_type = sym.get("symbol_type", "")

        # Only score methods, functions, constructors
        if sym_type not in ("Method", "Function", "Constructor"):
            continue

        # Skip framework types
        if name in _FRAMEWORK_TYPE_EXCLUSIONS:
            continue

        # Skip test file symbols
        if any(p.search(file_path) for p in _TEST_PATH_PATTERNS):
            continue

        # Base score: callees / (callers + 1)
        callees = kg.get_callees(sym_id)
        callers = kg.get_callers(sym_id)
        out_degree = len(callees)
        in_degree = len(callers)
        base_score = out_degree / (in_degree + 1)

        if base_score == 0:
            continue

        # Export multiplier
        export_mult = 2.0 if exported else 1.0

        # Name multiplier
        name_mult = 1.0
        for pattern in _ENTRY_PATTERNS:
            if pattern.match(name):
                name_mult = 1.5
                break

        # Also check parent class name for controller patterns
        parent = sym.get("parent", "")
        if parent:
            for pattern in _ENTRY_PATTERNS:
                if pattern.match(parent):
                    name_mult = max(name_mult, 1.3)
                    break

        # Utility penalty
        utility_penalty = 1.0
        file_lower = file_path.lower()
        for segment in _UTILITY_SEGMENTS:
            if segment in file_lower:
                utility_penalty = 0.3
                break

        # Depth bonus: reward symbols that can reach deeper call chains
        depth = _probe_depth(kg, sym_id)
        depth_bonus = 1.0 + (depth * 0.5)  # 1.0, 1.5, 2.0, 2.5

        score = base_score * export_mult * name_mult * utility_penalty * depth_bonus
        scores.append((sym_id, score))

    scores.sort(key=lambda x: x[1], reverse=True)
    return scores
