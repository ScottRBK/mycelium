"""Dual HashMap symbol table: file-scoped index + global index."""

from __future__ import annotations

from dataclasses import dataclass, field

from mycelium.config import Symbol


@dataclass
class SymbolDefinition:
    """Lightweight record for the global index."""
    symbol_id: str
    name: str
    file: str
    symbol_type: str
    language: str | None = None


class SymbolTable:
    """Dual HashMap for symbol lookups.

    file_index: file_path -> symbol_name -> symbol_id
    global_index: symbol_name -> list[SymbolDefinition]
    """

    def __init__(self) -> None:
        self.file_index: dict[str, dict[str, str]] = {}
        self.global_index: dict[str, list[SymbolDefinition]] = {}

    def add(self, symbol: Symbol) -> None:
        # File index
        if symbol.file not in self.file_index:
            self.file_index[symbol.file] = {}
        self.file_index[symbol.file][symbol.name] = symbol.id

        # Global index
        defn = SymbolDefinition(
            symbol_id=symbol.id,
            name=symbol.name,
            file=symbol.file,
            symbol_type=symbol.type.value,
            language=symbol.language,
        )
        if symbol.name not in self.global_index:
            self.global_index[symbol.name] = []
        self.global_index[symbol.name].append(defn)

    def lookup_exact(self, file_path: str, name: str) -> str | None:
        """Look up a symbol by file path and name. Returns symbol_id or None."""
        file_syms = self.file_index.get(file_path)
        if file_syms:
            return file_syms.get(name)
        return None

    def lookup_fuzzy(self, name: str) -> list[SymbolDefinition]:
        """Look up a symbol name in the global index. Returns all matching definitions."""
        return self.global_index.get(name, [])

    def get_symbols_in_file(self, file_path: str) -> dict[str, str]:
        """Return all symbol_name -> symbol_id mappings for a file."""
        return self.file_index.get(file_path, {})
