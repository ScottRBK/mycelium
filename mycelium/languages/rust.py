"""Rust language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_rust as ts_rust

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility

_TYPE_MAP = {
    "function_item": SymbolType.FUNCTION,
    "struct_item": SymbolType.STRUCT,
    "enum_item": SymbolType.ENUM,
    "trait_item": SymbolType.TRAIT,
    "impl_item": SymbolType.IMPL,
    "type_item": SymbolType.TYPE_ALIAS,
    "const_item": SymbolType.CONSTANT,
    "static_item": SymbolType.STATIC,
    "mod_item": SymbolType.MODULE,
    "macro_definition": SymbolType.MACRO,
}


class RustAnalyser:
    extensions = [".rs"]
    language_name = "rust"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_rust.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._walk_node(tree.root_node, source, file_path, symbols, parent_id=None)
        return symbols

    def _walk_node(self, node, source, file_path, symbols, parent_id):
        for child in node.children:
            sym_type = _TYPE_MAP.get(child.type)
            if sym_type is not None:
                name = self._get_name(child)
                if name is None:
                    continue

                is_pub = self._is_pub(child)
                symbols.append(Symbol(
                    id=f"_pending_{len(symbols)}",
                    name=name,
                    type=sym_type,
                    file=file_path,
                    line=child.start_point[0] + 1,
                    visibility=Visibility.PUBLIC if is_pub else Visibility.PRIVATE,
                    exported=is_pub,
                    parent=parent_id,
                ))

                # Recurse into impl blocks and mod blocks
                if child.type in ("impl_item", "mod_item"):
                    for c in child.children:
                        if c.type == "declaration_list":
                            self._walk_node(c, source, file_path, symbols, parent_id=name)

    def _get_name(self, node) -> str | None:
        for child in node.children:
            if child.type in ("identifier", "type_identifier"):
                return child.text.decode("utf-8")
        return None

    def _is_pub(self, node) -> bool:
        for child in node.children:
            if child.type == "visibility_modifier":
                return True
        return False

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports = []
        for child in tree.root_node.children:
            if child.type == "use_declaration":
                # Extract the use path
                path = None
                for c in child.children:
                    if c.type in ("scoped_identifier", "identifier", "use_wildcard", "scoped_use_list"):
                        path = c.text.decode("utf-8")
                        break
                if path:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8").rstrip(";").strip(),
                        target_name=path,
                        line=child.start_point[0] + 1,
                    ))
        return imports

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        calls: list[RawCall] = []
        exclusions = self.builtin_exclusions()
        self._find_calls(tree.root_node, file_path, calls, exclusions)
        return calls

    def _find_calls(self, node, file_path, calls, exclusions):
        if node.type == "call_expression":
            callee_name, qualifier = self._extract_callee(node)
            if callee_name and callee_name not in exclusions:
                qualified = f"{qualifier}::{callee_name}" if qualifier else callee_name
                if qualified not in exclusions:
                    caller = self._find_enclosing(node)
                    calls.append(RawCall(
                        caller_file=file_path,
                        caller_name=caller or "<module>",
                        callee_name=callee_name,
                        line=node.start_point[0] + 1,
                        qualifier=qualifier,
                    ))
        elif node.type == "macro_invocation":
            for c in node.children:
                if c.type == "identifier":
                    name = c.text.decode("utf-8")
                    if name not in exclusions and f"{name}!" not in exclusions:
                        caller = self._find_enclosing(node)
                        calls.append(RawCall(
                            caller_file=file_path,
                            caller_name=caller or "<module>",
                            callee_name=name,
                            line=node.start_point[0] + 1,
                        ))
                    break
        for child in node.children:
            self._find_calls(child, file_path, calls, exclusions)

    def _extract_callee(self, node):
        first = node.children[0] if node.children else None
        if first is None:
            return None, None
        if first.type == "identifier":
            return first.text.decode("utf-8"), None
        if first.type == "scoped_identifier":
            parts = []
            for c in first.children:
                if c.type in ("identifier", "type_identifier"):
                    parts.append(c.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        if first.type == "field_expression":
            parts = []
            for c in first.children:
                if c.type in ("identifier", "field_identifier"):
                    parts.append(c.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        return None, None

    def _find_enclosing(self, node):
        current = node.parent
        while current:
            if current.type == "function_item":
                return self._get_name(current)
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            "println!", "eprintln!", "format!", "vec!", "dbg!",
            "assert!", "assert_eq!", "assert_ne!", "todo!", "unimplemented!",
            "panic!", "unreachable!", "write!", "writeln!",
            "println", "eprintln", "format", "vec", "dbg",
            "assert", "assert_eq", "assert_ne", "todo", "unimplemented",
            "panic", "unreachable", "write", "writeln",
            "String::from", "Into::into", "From::from",
            "Clone::clone", "Default::default",
            "Some", "None", "Ok", "Err",
        }
