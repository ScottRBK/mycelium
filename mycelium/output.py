"""JSON serialisation matching the output schema."""

from __future__ import annotations

import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from mycelium.config import AnalysisConfig, AnalysisResult
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.symbol_table import SymbolTable


def _get_commit_hash(repo_path: str) -> str | None:
    """Try to get the current git commit hash."""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=repo_path,
            capture_output=True,
            text=True,
            timeout=5,
        )
        if result.returncode == 0:
            return result.stdout.strip()[:12]
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass
    return None


def _count_languages(kg: KnowledgeGraph) -> dict[str, int]:
    """Count files per language."""
    counts: dict[str, int] = {}
    for f in kg.get_files():
        lang = f.get("language")
        if lang:
            counts[lang] = counts.get(lang, 0) + 1
    return counts


def build_result(
    config: AnalysisConfig,
    kg: KnowledgeGraph,
    st: SymbolTable,
    timings: dict[str, float],
    total_ms: float,
) -> AnalysisResult:
    """Build the AnalysisResult from the knowledge graph."""
    repo_path = Path(config.repo_path).resolve()
    repo_name = repo_path.name

    calls = kg.get_call_edges()
    import_edges = kg.get_import_edges()
    communities = kg.get_communities()
    processes = kg.get_processes()

    result = AnalysisResult(
        version="1.0",
        metadata={
            "repo_name": repo_name,
            "repo_path": str(repo_path),
            "analysed_at": datetime.now(timezone.utc).isoformat(),
            "mycelium_version": "0.1.0",
            "commit_hash": _get_commit_hash(str(repo_path)),
            "analysis_duration_ms": round(total_ms, 1),
            "phase_timings": timings,
        },
        stats={
            "files": kg.file_count(),
            "folders": kg.folder_count(),
            "symbols": kg.symbol_count(),
            "calls": len(calls),
            "imports": len(import_edges),
            "communities": len(communities),
            "processes": len(processes),
            "languages": _count_languages(kg),
        },
        structure={
            "files": [
                {"path": f["path"], "language": f.get("language"), "size": f.get("size", 0), "lines": f.get("lines", 0)}
                for f in kg.get_files()
            ],
            "folders": [
                {"path": f["path"], "file_count": f.get("file_count", 0)}
                for f in kg.get_folders()
            ],
        },
        symbols=[
            {
                "id": s["id"],
                "name": s.get("name", ""),
                "type": s.get("symbol_type", ""),
                "file": s.get("file", ""),
                "line": s.get("line", 0),
                "visibility": s.get("visibility", "unknown"),
                "exported": s.get("exported", False),
                "parent": s.get("parent"),
                "language": s.get("language"),
            }
            for s in kg.get_symbols()
        ],
        imports={
            "file_imports": [
                {"from": e["from"], "to": e["to"], "statement": e.get("statement", "")}
                for e in import_edges
            ],
            "project_references": kg.get_project_references(),
            "package_references": kg.get_package_references(),
        },
        calls=[
            {
                "from": c["from"],
                "to": c["to"],
                "confidence": c.get("confidence", 0.0),
                "tier": c.get("tier", ""),
                "reason": c.get("reason", ""),
                "line": c.get("line", 0),
            }
            for c in calls
        ],
        communities=communities,
        processes=processes,
    )

    return result


def write_output(result: AnalysisResult, output_path: str) -> None:
    """Write the analysis result to a JSON file."""
    from dataclasses import asdict

    data = asdict(result)
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(data, f, indent=2, default=str)
