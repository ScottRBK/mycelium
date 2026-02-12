"""Sequential phase orchestrator with timing."""

from __future__ import annotations

import time
from pathlib import Path

from mycelium.config import AnalysisConfig, AnalysisResult
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.namespace_index import NamespaceIndex
from mycelium.graph.symbol_table import SymbolTable
from mycelium.output import build_result
from mycelium.phases.structure import run_structure_phase
from mycelium.phases.parsing import run_parsing_phase
from mycelium.phases.imports import run_imports_phase
from mycelium.phases.calls import run_calls_phase
from mycelium.phases.communities import run_communities_phase
from mycelium.phases.processes import run_processes_phase


_PHASE_LABELS = {
    "structure": "Mapping file tree",
    "parsing": "Parsing source files",
    "imports": "Resolving imports",
    "calls": "Building call graph",
    "communities": "Detecting communities",
    "processes": "Tracing execution flows",
}


def run_pipeline(
    config: AnalysisConfig,
    progress_callback=None,
) -> AnalysisResult:
    """Execute the six-phase analysis pipeline and return the result.

    Args:
        config: Analysis configuration.
        progress_callback: Optional callable(phase_name, label) invoked
            when each phase starts. Used by the CLI for Rich progress.
    """
    kg = KnowledgeGraph()
    st = SymbolTable()
    ns_index = NamespaceIndex()
    timings: dict[str, float] = {}
    total_start = time.monotonic()

    phases = [
        ("structure", lambda: run_structure_phase(config, kg)),
        ("parsing", lambda: run_parsing_phase(config, kg, st, ns_index)),
        ("imports", lambda: run_imports_phase(config, kg, st, ns_index)),
        ("calls", lambda: run_calls_phase(config, kg, st, ns_index)),
        ("communities", lambda: run_communities_phase(config, kg)),
        ("processes", lambda: run_processes_phase(config, kg)),
    ]

    for name, phase_fn in phases:
        if progress_callback:
            progress_callback(name, _PHASE_LABELS.get(name, name))
        start = time.monotonic()
        phase_fn()
        timings[name] = time.monotonic() - start

    total_ms = (time.monotonic() - total_start) * 1000

    return build_result(config, kg, st, timings, total_ms)
