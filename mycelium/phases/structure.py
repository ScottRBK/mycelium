"""Phase 1: File tree construction."""

from __future__ import annotations

import os
from pathlib import Path

from mycelium.config import AnalysisConfig, FileNode, FolderNode
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.languages import get_language

DEFAULT_IGNORE = {
    ".git", "bin", "obj", "node_modules", "packages", ".vs", ".idea",
    "TestResults", "__pycache__", ".mypy_cache", ".pytest_cache",
    ".tox", "dist", "build", ".eggs", "*.egg-info", "target",
    ".venv", "venv", ".env",
}


def _should_ignore(name: str, ignore_set: set[str]) -> bool:
    """Check if a directory or file name matches ignore patterns."""
    return name in ignore_set or name.startswith(".")


def run_structure_phase(config: AnalysisConfig, kg: KnowledgeGraph) -> None:
    """Walk the repo directory tree and build file/folder nodes."""
    root = Path(config.repo_path)
    if not root.is_dir():
        return

    ignore_set = set(DEFAULT_IGNORE)
    ignore_set.update(config.exclude_patterns)

    # Track .sln, .csproj, .vbproj paths as file metadata
    for dirpath, dirnames, filenames in os.walk(root):
        # Filter ignored directories in-place
        dirnames[:] = [
            d for d in sorted(dirnames)
            if not _should_ignore(d, ignore_set)
        ]

        rel_dir = os.path.relpath(dirpath, root)
        if rel_dir == ".":
            rel_dir = ""

        # Add folder node
        folder_path = rel_dir + "/" if rel_dir else ""
        file_count = len([f for f in filenames if not f.startswith(".")])
        kg.add_folder(FolderNode(path=folder_path, file_count=file_count))

        # Add file nodes
        for filename in sorted(filenames):
            if filename.startswith("."):
                continue

            full_path = os.path.join(dirpath, filename)
            rel_path = os.path.join(rel_dir, filename) if rel_dir else filename
            # Normalise path separators
            rel_path = rel_path.replace("\\", "/")

            ext = os.path.splitext(filename)[1].lower()
            language = get_language(ext)

            # Get file size
            try:
                size = os.path.getsize(full_path)
            except OSError:
                size = 0

            # Skip files over max size
            if size > config.max_file_size:
                continue

            # Count lines
            lines = 0
            if language:
                try:
                    with open(full_path, "rb") as f:
                        lines = sum(1 for _ in f)
                except OSError:
                    pass

            kg.add_file(FileNode(
                path=rel_path,
                language=language,
                size=size,
                lines=lines,
            ))

            # Also add non-source but structurally relevant files
            if ext in (".sln", ".csproj", ".vbproj"):
                if language is None:
                    kg.add_file(FileNode(
                        path=rel_path,
                        language=None,
                        size=size,
                        lines=0,
                    ))
