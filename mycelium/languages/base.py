"""Abstract base for language analysers."""

from __future__ import annotations

from typing import Protocol, runtime_checkable

import tree_sitter

from mycelium.config import ImportStatement, RawCall, Symbol


@runtime_checkable
class LanguageAnalyser(Protocol):
    """Protocol that all language analysers must implement."""

    extensions: list[str]
    language_name: str

    def get_language(self) -> tree_sitter.Language:
        """Return the tree-sitter Language object for this analyser."""
        ...

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        """Extract symbol definitions from a parsed AST."""
        ...

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        """Extract import statements from a parsed AST."""
        ...

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        """Extract call sites from a parsed AST."""
        ...

    def builtin_exclusions(self) -> set[str]:
        """Return set of built-in/runtime names to exclude from call graph."""
        ...
