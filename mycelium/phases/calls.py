"""Phase 4: Call graph with confidence scoring."""

from __future__ import annotations

import logging
import os

from mycelium.config import AnalysisConfig, CallEdge, SymbolType
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.namespace_index import NamespaceIndex
from mycelium.graph.symbol_table import SymbolTable
from mycelium.languages import get_analyser
from mycelium.phases.parsing import _get_parser

logger = logging.getLogger(__name__)


def run_calls_phase(
    config: AnalysisConfig, kg: KnowledgeGraph, st: SymbolTable,
    ns_index: NamespaceIndex | None = None,
) -> None:
    """Build the call graph with three-tier confidence scoring."""
    repo_root = config.repo_path

    # Build a map of file imports for Tier A resolution
    import_map = _build_import_map(kg)

    # Build field-type maps per file for DI resolution
    field_type_maps: dict[str, dict[str, str]] = {}

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

        # Read and parse
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

        # Extract raw calls
        try:
            raw_calls = analyser.extract_calls(tree, source, file_path)
        except Exception as e:
            logger.warning(f"Failed to extract calls from {file_path}: {e}")
            continue

        # Build field-type map for this file (lazy, once per file)
        if file_path not in field_type_maps:
            field_type_maps[file_path] = _build_field_type_map(file_path, kg)

        # Resolve each call
        for raw_call in raw_calls:
            edge = _resolve_call(
                raw_call, file_path, st, import_map, kg,
                field_type_maps.get(file_path, {}),
            )
            if edge:
                kg.add_call(edge)


def _build_import_map(kg: KnowledgeGraph) -> dict[str, list[str]]:
    """Build a map from source file -> list of imported file paths."""
    import_map: dict[str, list[str]] = {}
    for edge in kg.get_import_edges():
        from_file = edge["from"]
        to_file = edge["to"]
        if from_file not in import_map:
            import_map[from_file] = []
        import_map[from_file].append(to_file)
    return import_map


def _build_field_type_map(file_path: str, kg: KnowledgeGraph) -> dict[str, str]:
    """Build a mapping from field/parameter names to their types for DI resolution.

    Reads constructor parameter_types from the knowledge graph and maps
    _paramName -> TypeName and paramName -> TypeName.
    """
    field_map: dict[str, str] = {}
    for sym in kg.get_symbols_in_file(file_path):
        param_types = kg.graph.nodes[sym["id"]].get("parameter_types")
        if param_types:
            for param_name, type_name in param_types:
                field_map[param_name] = type_name
                # Convention: _paramName field stores the DI-injected service
                field_map[f"_{param_name}"] = type_name
    return field_map


def _is_interface_self_call(caller_name: str, callee_name: str, target_id: str, kg: KnowledgeGraph) -> bool:
    """Check if this is an interface calling its own method definition.

    Returns True if the caller and callee have the same name AND the target's
    parent symbol is an Interface.
    """
    if caller_name != callee_name:
        return False
    target_data = kg.graph.nodes.get(target_id, {})
    parent_name = target_data.get("parent")
    if not parent_name:
        return False
    # Find the parent symbol and check if it's an interface
    for sym in kg.get_symbols():
        if sym["name"] == parent_name and sym["symbol_type"] == SymbolType.INTERFACE.value:
            return True
    return False


def _is_interface_method(target_id: str, kg: KnowledgeGraph) -> bool:
    """Check if the target symbol is a method declared in an interface."""
    target_data = kg.graph.nodes.get(target_id, {})
    parent_name = target_data.get("parent")
    if not parent_name:
        return False
    for sym in kg.get_symbols():
        if sym["name"] == parent_name and sym["symbol_type"] == SymbolType.INTERFACE.value:
            return True
    return False


def _find_implementation(
    callee_name: str, interface_target_id: str, st: SymbolTable,
    import_map: dict, file_path: str, kg: KnowledgeGraph,
) -> str | None:
    """Find a concrete implementation of an interface method.

    When a call resolves to an interface method (which has no outgoing call edges),
    try to find a class method with the same name in imported files.
    """
    interface_file = kg.graph.nodes.get(interface_target_id, {}).get("file", "")
    imported_files = import_map.get(file_path, [])

    for imported_file in imported_files:
        if imported_file == interface_file:
            continue  # Skip the interface file itself
        target_id = st.lookup_exact(imported_file, callee_name)
        if target_id and target_id != interface_target_id:
            # Verify it's not another interface method
            if not _is_interface_method(target_id, kg):
                return target_id

    # Also try fuzzy lookup for implementations not in direct imports
    fuzzy_matches = st.lookup_fuzzy(callee_name)
    for match in fuzzy_matches:
        if match.symbol_id != interface_target_id and match.file != interface_file:
            if not _is_interface_method(match.symbol_id, kg):
                return match.symbol_id

    return None


def _resolve_call(
    raw_call, file_path: str, st: SymbolTable, import_map: dict,
    kg: KnowledgeGraph, field_type_map: dict[str, str] | None = None,
):
    """Resolve a raw call to a CallEdge with confidence scoring."""
    callee_name = raw_call.callee_name
    caller_name = raw_call.caller_name
    qualifier = raw_call.qualifier

    # Find the caller's symbol ID
    caller_id = st.lookup_exact(file_path, caller_name)
    if not caller_id:
        # Try fuzzy lookup for the caller
        caller_matches = st.lookup_fuzzy(caller_name)
        caller_matches = [m for m in caller_matches if m.file == file_path]
        if caller_matches:
            caller_id = caller_matches[0].symbol_id
        else:
            return None

    # --- Tier A: Import-resolved (no qualifier gate) ---
    if file_path in import_map:
        for imported_file in import_map[file_path]:
            target_id = st.lookup_exact(imported_file, callee_name)
            if target_id and target_id != caller_id:
                if _is_interface_self_call(caller_name, callee_name, target_id, kg):
                    continue
                # If target is an interface method, try to find the implementation
                if _is_interface_method(target_id, kg):
                    impl_id = _find_implementation(
                        callee_name, target_id, st, import_map, file_path, kg,
                    )
                    if impl_id:
                        # Create edge to implementation (more useful for BFS)
                        return CallEdge(
                            from_symbol=caller_id,
                            to_symbol=impl_id,
                            confidence=0.85,
                            tier="A",
                            reason="impl-resolved",
                            line=raw_call.line,
                        )
                return CallEdge(
                    from_symbol=caller_id,
                    to_symbol=target_id,
                    confidence=0.9,
                    tier="A",
                    reason="import-resolved",
                    line=raw_call.line,
                )

    # --- Tier A-DI: DI-resolved (qualifier is a field name) ---
    if qualifier and field_type_map:
        type_name = field_type_map.get(qualifier)
        if type_name and file_path in import_map:
            # Find the type in imported files, then look for callee in that type's file
            for imported_file in import_map[file_path]:
                type_id = st.lookup_exact(imported_file, type_name)
                if type_id:
                    # Look for callee in the same file as the type
                    target_id = st.lookup_exact(imported_file, callee_name)
                    if target_id and target_id != caller_id:
                        if _is_interface_self_call(caller_name, callee_name, target_id, kg):
                            continue
                        # If DI type is an interface, try implementation
                        if _is_interface_method(target_id, kg):
                            impl_id = _find_implementation(
                                callee_name, target_id, st, import_map, file_path, kg,
                            )
                            if impl_id:
                                return CallEdge(
                                    from_symbol=caller_id,
                                    to_symbol=impl_id,
                                    confidence=0.85,
                                    tier="A",
                                    reason="di-impl-resolved",
                                    line=raw_call.line,
                                )
                        return CallEdge(
                            from_symbol=caller_id,
                            to_symbol=target_id,
                            confidence=0.9,
                            tier="A",
                            reason="di-resolved",
                            line=raw_call.line,
                        )

    # --- Tier B: Same-file ---
    target_id = st.lookup_exact(file_path, callee_name)
    if target_id and target_id != caller_id:
        return CallEdge(
            from_symbol=caller_id,
            to_symbol=target_id,
            confidence=0.85,
            tier="B",
            reason="same-file",
            line=raw_call.line,
        )

    # --- Tier C: Fuzzy global ---
    fuzzy_matches = st.lookup_fuzzy(callee_name)
    # Exclude same-file matches already tried
    fuzzy_matches = [m for m in fuzzy_matches if m.file != file_path]
    if fuzzy_matches:
        if len(fuzzy_matches) == 1:
            target_id = fuzzy_matches[0].symbol_id
            if _is_interface_self_call(caller_name, callee_name, target_id, kg):
                return None
            return CallEdge(
                from_symbol=caller_id,
                to_symbol=target_id,
                confidence=0.5,
                tier="C",
                reason="fuzzy-unique",
                line=raw_call.line,
            )
        else:
            # Ambiguous - pick the first but with low confidence
            target_id = fuzzy_matches[0].symbol_id
            if _is_interface_self_call(caller_name, callee_name, target_id, kg):
                return None
            return CallEdge(
                from_symbol=caller_id,
                to_symbol=target_id,
                confidence=0.3,
                tier="C",
                reason="fuzzy-ambiguous",
                line=raw_call.line,
            )

    # No match at any tier - likely a framework/runtime call
    return None
