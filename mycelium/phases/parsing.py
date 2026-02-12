"""Phase 2: Tree-sitter AST to symbol extraction."""

from __future__ import annotations

import logging
import os

import tree_sitter

from mycelium.config import AnalysisConfig, SymbolType
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.namespace_index import NamespaceIndex
from mycelium.graph.symbol_table import SymbolTable
from mycelium.languages import get_analyser

logger = logging.getLogger(__name__)

# Cache parsers per language to avoid re-creating
_parsers: dict[str, tree_sitter.Parser] = {}


def _get_parser(analyser) -> tree_sitter.Parser | None:
    """Get or create a parser for the given analyser."""
    key = analyser.language_name
    if key not in _parsers:
        try:
            lang = analyser.get_language()
            _parsers[key] = tree_sitter.Parser(lang)
        except (RuntimeError, Exception) as e:
            logger.warning(f"Failed to initialise parser for {key}: {e}")
            return None
    return _parsers[key]


def run_parsing_phase(
    config: AnalysisConfig, kg: KnowledgeGraph, st: SymbolTable,
    ns_index: NamespaceIndex | None = None,
) -> None:
    """Parse source files and extract symbols."""
    _sym_counter = 0

    for file_data in kg.get_files():
        language = file_data.get("language")
        if not language:
            continue

        file_path = file_data["path"]
        ext = os.path.splitext(file_path)[1].lower()

        # Apply language filter
        if config.languages and language not in config.languages:
            continue

        analyser = get_analyser(ext)
        if analyser is None:
            continue

        # Check if analyser has is_available method (e.g., VB.NET)
        if hasattr(analyser, "is_available") and not analyser.is_available():
            continue

        parser = _get_parser(analyser)
        if parser is None:
            continue

        # Read file
        full_path = os.path.join(config.repo_path, file_path)
        try:
            with open(full_path, "rb") as f:
                source = f.read()
        except OSError as e:
            logger.warning(f"Failed to read {file_path}: {e}")
            continue

        # Parse
        try:
            tree = parser.parse(source)
        except Exception as e:
            logger.warning(f"Failed to parse {file_path}: {e}")
            continue

        # Extract symbols
        try:
            symbols = analyser.extract_symbols(tree, source, file_path)
        except Exception as e:
            logger.warning(f"Failed to extract symbols from {file_path}: {e}")
            continue

        # Assign IDs and add to graph + symbol table
        for symbol in symbols:
            _sym_counter += 1
            symbol.id = f"sym_{_sym_counter:04d}"
            symbol.language = language
            kg.add_symbol(symbol)
            st.add(symbol)

            # Register namespaces in the namespace index
            if ns_index and symbol.type == SymbolType.NAMESPACE:
                ns_index.register(symbol.name, file_path)
