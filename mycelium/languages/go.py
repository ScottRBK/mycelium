"""Go language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_go as ts_go

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility


class GoAnalyser:
    extensions = [".go"]
    language_name = "go"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_go.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        for child in tree.root_node.children:
            if child.type == "function_declaration":
                name = self._get_name(child, "identifier")
                if name:
                    exported = name[0].isupper()
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.FUNCTION,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC if exported else Visibility.PRIVATE,
                        exported=exported,
                    ))
            elif child.type == "method_declaration":
                name = self._get_name(child, "field_identifier")
                if name:
                    exported = name[0].isupper()
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.METHOD,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC if exported else Visibility.PRIVATE,
                        exported=exported,
                    ))
            elif child.type == "type_declaration":
                for spec in child.children:
                    if spec.type == "type_spec":
                        name = self._get_name(spec, "type_identifier")
                        if name:
                            # Determine if struct, interface, or other
                            sym_type = SymbolType.TYPE_ALIAS
                            for c in spec.children:
                                if c.type == "struct_type":
                                    sym_type = SymbolType.STRUCT
                                elif c.type == "interface_type":
                                    sym_type = SymbolType.INTERFACE
                            exported = name[0].isupper()
                            symbols.append(Symbol(
                                id=f"_pending_{len(symbols)}",
                                name=name,
                                type=sym_type,
                                file=file_path,
                                line=spec.start_point[0] + 1,
                                visibility=Visibility.PUBLIC if exported else Visibility.PRIVATE,
                                exported=exported,
                            ))
            elif child.type == "const_declaration":
                for spec in child.children:
                    if spec.type == "const_spec":
                        name = self._get_name(spec, "identifier")
                        if name:
                            symbols.append(Symbol(
                                id=f"_pending_{len(symbols)}",
                                name=name,
                                type=SymbolType.CONSTANT,
                                file=file_path,
                                line=spec.start_point[0] + 1,
                                visibility=Visibility.PUBLIC if name[0].isupper() else Visibility.PRIVATE,
                                exported=name[0].isupper(),
                            ))
        return symbols

    def _get_name(self, node, target_type) -> str | None:
        for child in node.children:
            if child.type == target_type:
                return child.text.decode("utf-8")
        return None

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports = []
        for child in tree.root_node.children:
            if child.type == "import_declaration":
                for spec in child.children:
                    if spec.type == "import_spec":
                        path = self._extract_string(spec)
                        if path:
                            imports.append(ImportStatement(
                                file=file_path,
                                statement=f'import "{path}"',
                                target_name=path,
                                line=spec.start_point[0] + 1,
                            ))
                    elif spec.type == "import_spec_list":
                        for sub in spec.children:
                            if sub.type == "import_spec":
                                path = self._extract_string(sub)
                                if path:
                                    imports.append(ImportStatement(
                                        file=file_path,
                                        statement=f'import "{path}"',
                                        target_name=path,
                                        line=sub.start_point[0] + 1,
                                    ))
                    elif spec.type == "interpreted_string_literal":
                        path = self._extract_string_content(spec)
                        if path:
                            imports.append(ImportStatement(
                                file=file_path,
                                statement=f'import "{path}"',
                                target_name=path,
                                line=spec.start_point[0] + 1,
                            ))
        return imports

    def _extract_string(self, node) -> str | None:
        for child in node.children:
            if child.type == "interpreted_string_literal":
                return self._extract_string_content(child)
        return None

    def _extract_string_content(self, node) -> str | None:
        for child in node.children:
            if child.type == "interpreted_string_literal_content":
                return child.text.decode("utf-8")
        return None

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
                qualified = f"{qualifier}.{callee_name}" if qualifier else callee_name
                if qualified not in exclusions:
                    caller = self._find_enclosing(node)
                    calls.append(RawCall(
                        caller_file=file_path,
                        caller_name=caller or "<module>",
                        callee_name=callee_name,
                        line=node.start_point[0] + 1,
                        qualifier=qualifier,
                    ))
        for child in node.children:
            self._find_calls(child, file_path, calls, exclusions)

    def _extract_callee(self, node):
        first = node.children[0] if node.children else None
        if first is None:
            return None, None
        if first.type == "identifier":
            return first.text.decode("utf-8"), None
        if first.type == "selector_expression":
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
            if current.type == "function_declaration":
                return self._get_name(current, "identifier")
            if current.type == "method_declaration":
                return self._get_name(current, "field_identifier")
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            "fmt.Println", "fmt.Printf", "fmt.Sprintf", "fmt.Fprintf",
            "fmt.Errorf", "fmt.Print",
            "log.Println", "log.Printf", "log.Fatal", "log.Fatalf",
            "append", "make", "len", "cap", "close", "delete",
            "new", "panic", "recover", "copy",
        }
