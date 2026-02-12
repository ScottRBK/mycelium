"""Phase 6: BFS execution flow detection."""

from __future__ import annotations

import logging

from mycelium.config import AnalysisConfig, Process
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.scoring import score_entry_points

logger = logging.getLogger(__name__)


def run_processes_phase(config: AnalysisConfig, kg: KnowledgeGraph) -> None:
    """Detect execution flows via BFS from scored entry points."""
    max_processes = config.max_processes
    max_depth = config.max_depth
    max_branching = config.max_branching
    min_steps = config.min_steps

    # Score entry points
    entry_points = score_entry_points(kg)
    if not entry_points:
        return

    # Take top N candidates (2x max to allow for deduplication)
    candidates = entry_points[:max_processes * 2]

    # BFS from each entry point (multi-branch)
    traces: list[list[str]] = []
    for entry_id, score in candidates:
        new_traces = _bfs_traces(kg, entry_id, max_depth, max_branching, min_steps)
        traces.extend(new_traces)

    # Deduplicate: remove traces that are strict subsequences of longer traces
    traces = _deduplicate(traces)

    # Build community membership map for classification
    community_map = _build_community_map(kg)

    # Create Process objects, capped at max_processes
    # Sort by normalised confidence (geometric mean per hop), tiebreak by length
    process_data = []
    for trace in traces:
        total_conf = _compute_total_confidence(kg, trace)
        process_data.append((trace, total_conf))

    def _sort_key(item: tuple[list[str], float]) -> tuple[float, int]:
        trace, total_conf = item
        n_edges = len(trace) - 1
        if n_edges <= 0:
            return (1.0, 0)
        # Geometric mean: total_conf^(1/n_edges) normalises by path length
        normalised = total_conf ** (1.0 / n_edges)
        return (normalised, len(trace))

    process_data.sort(key=_sort_key, reverse=True)

    # Depth-diverse selection: prioritise multi-step traces so they aren't
    # crowded out by the sheer volume of 2-step traces.
    deep = [item for item in process_data if len(item[0]) > 2]
    shallow = [item for item in process_data if len(item[0]) <= 2]
    max_deep = max_processes // 2
    selected_deep = deep[:max_deep]
    selected = selected_deep + shallow[:max_processes - len(selected_deep)]
    selected.sort(key=_sort_key, reverse=True)
    process_data = selected

    for i, (trace, total_conf) in enumerate(process_data):
        process_type = _classify_process(trace, community_map)
        process = Process(
            id=f"process_{i}",
            entry=trace[0],
            terminal=trace[-1],
            steps=trace,
            type=process_type,
            total_confidence=round(total_conf, 4),
        )
        kg.add_process(process)


def _bfs_traces(
    kg: KnowledgeGraph, start: str, max_depth: int, max_branching: int,
    min_steps: int,
) -> list[list[str]]:
    """Multi-branch BFS from a starting symbol.

    Returns multiple traces per entry point. Each trace follows a different
    branch of callees. Per-path cycle detection allows two paths to visit
    the same node.
    """
    traces: list[list[str]] = []
    max_traces = max_branching * 3  # Cap per entry point
    queue: list[tuple[str, list[str]]] = [(start, [start])]

    while queue and len(traces) < max_traces:
        current, path = queue.pop(0)  # FIFO

        callees = kg.get_callees(current)
        if not callees or len(path) >= max_depth:
            if len(path) >= min_steps:
                traces.append(path)
            continue

        # Sort by confidence descending
        callees.sort(key=lambda c: c.get("confidence", 0), reverse=True)

        extended = False
        for callee in callees[:max_branching]:
            callee_id = callee["id"]
            if callee_id not in path:  # Per-path cycle detection
                queue.append((callee_id, path + [callee_id]))
                extended = True

        if not extended and len(path) >= min_steps:
            traces.append(path)  # All branches were cycles

    return traces


def _deduplicate(traces: list[list[str]]) -> list[list[str]]:
    """Remove traces that are strict subsequences of longer traces."""
    # Sort by length descending
    traces.sort(key=len, reverse=True)

    result = []
    for trace in traces:
        trace_set = set(trace)
        is_subset = False
        for existing in result:
            existing_set = set(existing)
            if trace_set.issubset(existing_set) and trace_set != existing_set:
                is_subset = True
                break
        if not is_subset:
            result.append(trace)

    return result


def _build_community_map(kg: KnowledgeGraph) -> dict[str, str]:
    """Build symbol_id -> community_id mapping."""
    community_map: dict[str, str] = {}
    for comm in kg.get_communities():
        comm_id = comm["id"]
        for member in comm.get("members", []):
            community_map[member] = comm_id
    return community_map


def _classify_process(trace: list[str], community_map: dict[str, str]) -> str:
    """Classify process as intra_community or cross_community."""
    communities_seen = set()
    for sym_id in trace:
        comm = community_map.get(sym_id)
        if comm:
            communities_seen.add(comm)

    if len(communities_seen) <= 1:
        return "intra_community"
    return "cross_community"


def _compute_total_confidence(kg: KnowledgeGraph, trace: list[str]) -> float:
    """Compute total confidence as product of edge confidences along the trace."""
    if len(trace) < 2:
        return 1.0

    total = 1.0
    for i in range(len(trace) - 1):
        from_id = trace[i]
        to_id = trace[i + 1]
        # Find the call edge
        callees = kg.get_callees(from_id)
        edge_conf = 0.5  # Default if edge not found directly
        for callee in callees:
            if callee["id"] == to_id:
                edge_conf = callee.get("confidence", 0.5)
                break
        total *= edge_conf

    return total
