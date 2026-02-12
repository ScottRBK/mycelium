"""Phase 3: Import/dependency resolution."""

from __future__ import annotations

import logging
import os
from collections import defaultdict

import tree_sitter

from mycelium.config import (
    AnalysisConfig,
    ImportEdge,
    PackageReference,
    ProjectReference,
)
from mycelium.dotnet.assembly import AssemblyMapper
from mycelium.dotnet.project import parse_project
from mycelium.dotnet.solution import parse_solution
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.namespace_index import NamespaceIndex
from mycelium.graph.symbol_table import SymbolTable
from mycelium.languages import get_analyser
from mycelium.phases.parsing import _get_parser

logger = logging.getLogger(__name__)


def run_imports_phase(
    config: AnalysisConfig, kg: KnowledgeGraph, st: SymbolTable,
    ns_index: NamespaceIndex | None = None,
) -> None:
    """Resolve file imports, project references, and package references."""
    repo_root = config.repo_path
    assembly_mapper = AssemblyMapper()

    # --- Level 2 & 3: Parse .sln and .csproj/.vbproj files ---
    _process_dotnet_projects(config, kg, assembly_mapper)

    # --- Supplement assembly mapper with observed namespaces from parsed symbols ---
    _register_observed_namespaces(kg, st, assembly_mapper, repo_root)

    # --- Level 1: Parse source file imports ---
    _process_source_imports(config, kg, st, assembly_mapper, ns_index)


def _process_dotnet_projects(
    config: AnalysisConfig, kg: KnowledgeGraph, assembly_mapper: AssemblyMapper
) -> None:
    """Parse .sln files and .csproj/.vbproj files for project/package references."""
    repo_root = config.repo_path

    # Find .sln files
    sln_files = []
    project_files = []
    for file_data in kg.get_files():
        path = file_data["path"]
        if path.endswith(".sln"):
            sln_files.append(path)
        elif path.endswith((".csproj", ".vbproj")):
            project_files.append(path)

    # Parse solutions to discover projects
    for sln_path in sln_files:
        full_sln_path = os.path.join(repo_root, sln_path)
        sln_projects = parse_solution(full_sln_path)
        for sp in sln_projects:
            logger.debug(f"Solution project: {sp.name} -> {sp.path}")

    # Parse each project file
    project_infos = {}
    for proj_path in project_files:
        full_proj_path = os.path.join(repo_root, proj_path)
        info = parse_project(full_proj_path)
        project_infos[proj_path] = info

        # Register root namespace
        if info.root_namespace:
            assembly_mapper.register_namespace(info.root_namespace, proj_path)

        # Add project references
        for ref_path in info.project_references:
            # Resolve relative path from project file location
            proj_dir = os.path.dirname(full_proj_path)
            resolved = os.path.normpath(os.path.join(proj_dir, ref_path))
            rel_resolved = os.path.relpath(resolved, repo_root).replace("\\", "/")
            kg.add_project_reference(ProjectReference(
                from_project=proj_path,
                to_project=rel_resolved,
            ))

        # Add package references
        for pkg_name, pkg_version in info.package_references:
            kg.add_package_reference(PackageReference(
                project=proj_path,
                package=pkg_name,
                version=pkg_version,
            ))


def _register_observed_namespaces(
    kg: KnowledgeGraph, st: SymbolTable, assembly_mapper: AssemblyMapper, repo_root: str
) -> None:
    """Supplement assembly mapper with namespace declarations found during parsing."""
    from mycelium.config import SymbolType

    for sym_data in kg.get_symbols():
        if sym_data.get("symbol_type") == SymbolType.NAMESPACE.value:
            ns_name = sym_data["name"]
            file_path = sym_data["file"]

            # Find which project this file belongs to
            # Walk up from the file to find a .csproj/.vbproj
            project = _find_project_for_file(file_path, kg)
            if project:
                assembly_mapper.register_namespace(ns_name, project)


def _find_project_for_file(file_path: str, kg: KnowledgeGraph) -> str | None:
    """Find the .csproj/.vbproj that contains this source file."""
    file_dir = os.path.dirname(file_path)
    # Look for project files in same or parent directories
    for file_data in kg.get_files():
        path = file_data["path"]
        if path.endswith((".csproj", ".vbproj")):
            proj_dir = os.path.dirname(path)
            if file_dir.startswith(proj_dir) or proj_dir == "":
                return path
    return None


def _process_source_imports(
    config: AnalysisConfig,
    kg: KnowledgeGraph,
    st: SymbolTable,
    assembly_mapper: AssemblyMapper,
    ns_index: NamespaceIndex | None = None,
) -> None:
    """Parse source file imports and resolve to target files."""
    repo_root = config.repo_path

    # Build file set once for O(1) lookups
    file_set = {f["path"] for f in kg.get_files()}

    # --- Pre-processing: build language-specific indexes ---

    # Go: parse go.mod and build directory index
    go_module = _parse_go_mod(file_set, repo_root)
    go_dir_index = _build_go_dir_index(file_set) if go_module else {}

    # Java: build basename index for class-name fallback resolution
    java_basename_index: dict[str, list[str]] = defaultdict(list)
    for path in file_set:
        if path.endswith(".java"):
            basename = os.path.basename(path)
            java_basename_index[basename].append(path)

    for file_data in kg.get_files():
        language = file_data.get("language")
        if not language:
            continue

        file_path = file_data["path"]
        ext = os.path.splitext(file_path)[1].lower()

        if config.languages and language not in config.languages:
            continue

        analyser = get_analyser(ext)
        if analyser is None:
            continue

        if hasattr(analyser, "is_available") and not analyser.is_available():
            continue

        parser = _get_parser(analyser)
        if parser is None:
            continue

        # Read and parse file
        full_path = os.path.join(repo_root, file_path)
        try:
            with open(full_path, "rb") as f:
                source = f.read()
        except OSError:
            continue

        try:
            tree = parser.parse(source)
        except Exception:
            continue

        # Extract imports
        try:
            imports = analyser.extract_imports(tree, source, file_path)
        except Exception as e:
            logger.warning(f"Failed to extract imports from {file_path}: {e}")
            continue

        # Resolve each import
        for imp in imports:
            # --- C#/VB.NET: namespace index ---
            if ns_index and language in ("cs", "vb"):
                ns_files = ns_index.get_files_for_namespace(imp.target_name)
                if ns_files:
                    ns_index.register_file_import(file_path, imp.target_name)
                    for target in ns_files:
                        if target != file_path:
                            kg.add_import(ImportEdge(
                                from_file=file_path,
                                to_file=target,
                                statement=imp.statement,
                            ))
                    continue

            # --- Python: dotted module paths ---
            if language == "py":
                target = _resolve_python_import(
                    imp.target_name, file_path, file_set,
                )
                if target and target != file_path:
                    kg.add_import(ImportEdge(
                        from_file=file_path,
                        to_file=target,
                        statement=imp.statement,
                    ))
                continue

            # --- TypeScript/JavaScript: relative path + extension probing ---
            if language == "ts":
                target = _resolve_ts_import(imp.target_name, file_path, file_set)
                if target and target != file_path:
                    kg.add_import(ImportEdge(
                        from_file=file_path,
                        to_file=target,
                        statement=imp.statement,
                    ))
                continue

            # --- Java: dotted path + class-name fallback ---
            if language == "java":
                target = _resolve_java_import(
                    imp.target_name, file_path, file_set, java_basename_index,
                )
                if target and target != file_path:
                    kg.add_import(ImportEdge(
                        from_file=file_path,
                        to_file=target,
                        statement=imp.statement,
                    ))
                continue

            # --- Go: package-level directory resolution ---
            if language == "go":
                targets = _resolve_go_import(
                    imp.target_name, file_path, go_module, go_dir_index,
                )
                for target in targets:
                    if target != file_path:
                        kg.add_import(ImportEdge(
                            from_file=file_path,
                            to_file=target,
                            statement=imp.statement,
                        ))
                continue

            # --- Rust: crate/super/self prefix + progressive shortening ---
            if language == "rust":
                target = _resolve_rust_import(
                    imp.target_name, file_path, file_set,
                )
                if target and target != file_path:
                    kg.add_import(ImportEdge(
                        from_file=file_path,
                        to_file=target,
                        statement=imp.statement,
                    ))
                continue

            # --- C/C++: relative include resolution ---
            if language in ("c", "cpp"):
                target = _resolve_c_include(
                    imp.target_name, imp.statement, file_path, file_set,
                )
                if target and target != file_path:
                    kg.add_import(ImportEdge(
                        from_file=file_path,
                        to_file=target,
                        statement=imp.statement,
                    ))
                continue

            # Fallback: original resolution logic (kept for safety)
            target = _resolve_import(imp.target_name, file_path, st, assembly_mapper, kg)
            if target and target != file_path:
                kg.add_import(ImportEdge(
                    from_file=file_path,
                    to_file=target,
                    statement=imp.statement,
                ))
                if ns_index and language in ("cs", "vb"):
                    ns_index.register_file_import(file_path, imp.target_name)


# ---------------------------------------------------------------------------
# Python resolvers (existing)
# ---------------------------------------------------------------------------

def _resolve_python_import(
    target_name: str, source_file: str, file_set: set[str],
) -> str | None:
    """Resolve a Python import target to a file path in the repo.

    Handles:
    - Absolute dotted imports: ``app.config.settings`` -> ``app/config/settings.py``
      or ``app/config/settings/__init__.py``
    - Bare module imports: ``models`` -> ``models.py`` or ``models/__init__.py``
    - Relative imports (leading dots): ``.sibling`` -> sibling relative to source
    """
    if target_name.startswith("."):
        return _resolve_python_relative(target_name, source_file, file_set)

    # Convert dots to path separators
    path = target_name.replace(".", "/")

    # Check as a module file first, then as a package __init__
    candidate = f"{path}.py"
    if candidate in file_set:
        return candidate

    candidate = f"{path}/__init__.py"
    if candidate in file_set:
        return candidate

    return None


def _resolve_python_relative(
    target_name: str, source_file: str, file_set: set[str],
) -> str | None:
    """Resolve a relative Python import (leading dots) to a file path."""
    # Count leading dots
    dots = 0
    for ch in target_name:
        if ch == ".":
            dots += 1
        else:
            break
    remainder = target_name[dots:]

    # Navigate up from source file's directory
    base = os.path.dirname(source_file)
    for _ in range(dots - 1):
        base = os.path.dirname(base)

    if not base:
        if not remainder:
            return None

    if remainder:
        path = os.path.join(base, remainder.replace(".", "/")) if base else remainder.replace(".", "/")
    else:
        if base:
            candidate = f"{base}/__init__.py"
            if candidate in file_set:
                return candidate
        return None

    path = path.replace("\\", "/")

    candidate = f"{path}.py"
    if candidate in file_set:
        return candidate

    candidate = f"{path}/__init__.py"
    if candidate in file_set:
        return candidate

    return None


# ---------------------------------------------------------------------------
# TypeScript/JavaScript resolver
# ---------------------------------------------------------------------------

def _resolve_ts_import(
    target_name: str, source_file: str, file_set: set[str],
) -> str | None:
    """Resolve a TS/JS import path to a file in the repo.

    - Bare specifiers (no ``./`` or ``../`` prefix) are external packages -> None.
    - Relative paths are resolved against the source file's directory.
    - Extension probing: ``.ts``, ``.tsx``, ``.js``, ``.jsx``.
    - Index file probing: ``{path}/index.{ts,tsx,js,jsx}``.
    """
    if not target_name.startswith(("./", "../")):
        return None  # bare / scoped package

    source_dir = os.path.dirname(source_file)
    resolved = os.path.normpath(os.path.join(source_dir, target_name)).replace("\\", "/")

    # If the path already has an extension that matches, check directly
    if resolved in file_set:
        return resolved

    # Extension probing
    for ext in (".ts", ".tsx", ".js", ".jsx"):
        candidate = resolved + ext
        if candidate in file_set:
            return candidate

    # Index file probing
    for ext in (".ts", ".tsx", ".js", ".jsx"):
        candidate = f"{resolved}/index{ext}"
        if candidate in file_set:
            return candidate

    return None


# ---------------------------------------------------------------------------
# Java resolver
# ---------------------------------------------------------------------------

def _resolve_java_import(
    target_name: str,
    source_file: str,
    file_set: set[str],
    basename_index: dict[str, list[str]],
) -> str | None:
    """Resolve a Java import to a file in the repo.

    - Primary: convert dots to ``/``, append ``.java``, check ``file_set``.
    - Fallback: extract class name (last segment), look up in ``basename_index``.
    - Stdlib / third-party imports naturally won't be in ``file_set``.
    """
    # Primary: path-based resolution (works for proper package dirs)
    path = target_name.replace(".", "/") + ".java"
    if path in file_set:
        return path

    # Fallback: class-name basename lookup (works for flat layouts)
    class_name = target_name.rsplit(".", 1)[-1]
    basename = f"{class_name}.java"
    candidates = basename_index.get(basename, [])
    for candidate in candidates:
        if candidate != source_file:
            return candidate

    return None


# ---------------------------------------------------------------------------
# Go resolver
# ---------------------------------------------------------------------------

def _parse_go_mod(file_set: set[str], repo_root: str) -> str | None:
    """Find go.mod and extract the module directive."""
    for path in file_set:
        if os.path.basename(path) == "go.mod":
            full = os.path.join(repo_root, path)
            try:
                with open(full) as f:
                    for line in f:
                        line = line.strip()
                        if line.startswith("module "):
                            return line.split(None, 1)[1].strip()
            except OSError:
                pass
    return None


def _build_go_dir_index(file_set: set[str]) -> dict[str, list[str]]:
    """Map directories to their .go files."""
    index: dict[str, list[str]] = defaultdict(list)
    for path in file_set:
        if path.endswith(".go"):
            d = os.path.dirname(path)
            index[d].append(path)
    return dict(index)


def _resolve_go_import(
    target_name: str,
    source_file: str,
    go_module: str | None,
    go_dir_index: dict[str, list[str]],
) -> list[str]:
    """Resolve a Go import to all .go files in the target package directory.

    - Single-segment (no ``/``) -> stdlib, return ``[]``.
    - Must start with ``go_module`` prefix -> strip to get relative dir.
    - Look up directory in ``go_dir_index`` -> return all ``.go`` files.
    """
    if not go_module:
        return []

    # Stdlib imports have no slash (fmt, os, strings, etc.)
    if "/" not in target_name:
        return []

    # Must be part of this module
    if not target_name.startswith(go_module):
        return []

    # Strip module prefix to get relative directory
    rel_dir = target_name[len(go_module):]
    if rel_dir.startswith("/"):
        rel_dir = rel_dir[1:]

    return go_dir_index.get(rel_dir, [])


# ---------------------------------------------------------------------------
# Rust resolver
# ---------------------------------------------------------------------------

_RUST_EXTERNAL_PREFIXES = ("std::", "core::", "alloc::")


def _resolve_rust_import(
    target_name: str, source_file: str, file_set: set[str],
) -> str | None:
    """Resolve a Rust ``use`` path to a file in the repo.

    - ``std::``, ``core::``, ``alloc::`` -> external, return None.
    - ``crate::`` -> resolve from crate root (repo root).
    - ``super::`` -> navigate up from source dir.
    - ``self::`` -> resolve from current dir.
    - Bare path -> resolve from source dir.
    - Progressive shortening: last segments may be symbol names, not modules.
      Try full path, then drop trailing segments one at a time.
    """
    if target_name.startswith(_RUST_EXTERNAL_PREFIXES):
        return None

    source_dir = os.path.dirname(source_file)

    if target_name.startswith("crate::"):
        remainder = target_name[len("crate::"):]
        base = ""
    elif target_name.startswith("super::"):
        # Count consecutive super:: prefixes
        remainder = target_name
        base = source_dir
        while remainder.startswith("super::"):
            remainder = remainder[len("super::"):]
            base = os.path.dirname(base)
    elif target_name.startswith("self::"):
        remainder = target_name[len("self::"):]
        base = source_dir
    else:
        # Bare path (e.g. service::DataService) -> resolve from source dir
        remainder = target_name
        base = source_dir

    # Split on :: and try progressive shortening
    segments = remainder.split("::")

    # Try from longest path to shortest (drop trailing symbol names)
    for end in range(len(segments), 0, -1):
        path_segments = segments[:end]
        rel_path = "/".join(path_segments)
        if base:
            full_rel = f"{base}/{rel_path}"
        else:
            full_rel = rel_path
        full_rel = full_rel.replace("\\", "/")

        # Try as {path}.rs
        candidate = f"{full_rel}.rs"
        if candidate in file_set:
            return candidate

        # Try as {path}/mod.rs
        candidate = f"{full_rel}/mod.rs"
        if candidate in file_set:
            return candidate

    return None


# ---------------------------------------------------------------------------
# C/C++ resolver
# ---------------------------------------------------------------------------

def _resolve_c_include(
    target_name: str,
    statement: str,
    source_file: str,
    file_set: set[str],
) -> str | None:
    """Resolve a C/C++ #include to a file in the repo.

    - System includes (``<...>``) -> return None.
    - User includes (``"..."``) -> resolve relative to source dir, then repo root.
    """
    # System includes contain < in the statement text
    if "<" in statement:
        return None

    source_dir = os.path.dirname(source_file)

    # Resolve relative to source file directory
    if source_dir:
        candidate = os.path.normpath(f"{source_dir}/{target_name}").replace("\\", "/")
    else:
        candidate = target_name
    if candidate in file_set:
        return candidate

    # Fallback: resolve from repo root
    candidate = os.path.normpath(target_name).replace("\\", "/")
    if candidate in file_set:
        return candidate

    return None


# ---------------------------------------------------------------------------
# Fallback resolver (original, kept for safety)
# ---------------------------------------------------------------------------

def _resolve_import(
    target_name: str,
    source_file: str,
    st: SymbolTable,
    assembly_mapper: AssemblyMapper,
    kg: KnowledgeGraph,
) -> str | None:
    """Try to resolve an import target name to a file path."""
    # For C#/VB.NET: target_name is a namespace (e.g., "Absence.Services")
    # Try to find files that declare this namespace
    matches = st.lookup_fuzzy(target_name)
    if matches:
        # Return the first matching file
        return matches[0].file

    # Try assembly mapper
    project = assembly_mapper.resolve_namespace(target_name)
    if project:
        # Find files in this project that might match
        for file_data in kg.get_files():
            file_path = file_data["path"]
            if file_path.endswith((".cs", ".vb")):
                proj_dir = os.path.dirname(project)
                if file_path.startswith(proj_dir) or proj_dir == "":
                    # Check if this file has symbols in the target namespace
                    file_syms = st.get_symbols_in_file(file_path)
                    if file_syms:
                        return file_path

    return None
