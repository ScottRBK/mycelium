"""TypeScript/JavaScript language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_typescript as ts_typescript
import tree_sitter_javascript as ts_javascript

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility

_TYPE_MAP = {
    "class_declaration": SymbolType.CLASS,
    "interface_declaration": SymbolType.INTERFACE,
    "enum_declaration": SymbolType.ENUM,
    "function_declaration": SymbolType.FUNCTION,
    "type_alias_declaration": SymbolType.TYPE_ALIAS,
}

_CONTAINER_TYPES = {"class_declaration"}


class TypeScriptAnalyser:
    extensions = [".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"]
    language_name = "ts"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_typescript.language_typescript())

    def get_language_for_ext(self, ext: str) -> tree_sitter.Language:
        if ext == ".tsx":
            return tree_sitter.Language(ts_typescript.language_tsx())
        if ext in (".js", ".jsx", ".mjs", ".cjs"):
            return tree_sitter.Language(ts_javascript.language())
        return self.get_language()

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._walk_node(tree.root_node, source, file_path, symbols, parent_id=None)
        return symbols

    def _walk_node(self, node, source, file_path, symbols, parent_id):
        for child in node.children:
            exported = False
            decl = child

            # Check for export_statement wrapper
            if child.type == "export_statement":
                exported = True
                for c in child.children:
                    if c.type in _TYPE_MAP or c.type in ("lexical_declaration",):
                        decl = c
                        break

            sym_type = _TYPE_MAP.get(decl.type)
            if sym_type is not None:
                name = self._get_name(decl)
                if name:
                    sym = Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=sym_type,
                        file=file_path,
                        line=decl.start_point[0] + 1,
                        visibility=Visibility.PUBLIC if exported else Visibility.PRIVATE,
                        exported=exported,
                        parent=parent_id,
                    )
                    symbols.append(sym)

                    if decl.type in _CONTAINER_TYPES:
                        for c in decl.children:
                            if c.type == "class_body":
                                self._extract_class_members(c, file_path, symbols, name)

            elif decl.type == "lexical_declaration":
                # const/let with arrow functions
                for vc in decl.children:
                    if vc.type == "variable_declarator":
                        vname = None
                        is_fn = False
                        for c in vc.children:
                            if c.type == "identifier":
                                vname = c.text.decode("utf-8")
                            if c.type == "arrow_function":
                                is_fn = True
                        if vname and is_fn:
                            symbols.append(Symbol(
                                id=f"_pending_{len(symbols)}",
                                name=vname,
                                type=SymbolType.FUNCTION,
                                file=file_path,
                                line=vc.start_point[0] + 1,
                                visibility=Visibility.PUBLIC if exported else Visibility.PRIVATE,
                                exported=exported,
                                parent=parent_id,
                            ))

    def _extract_class_members(self, body_node, file_path, symbols, parent_name):
        for child in body_node.children:
            if child.type == "method_definition":
                name = None
                for c in child.children:
                    if c.type == "property_identifier":
                        name = c.text.decode("utf-8")
                        break
                if name:
                    sym_type = SymbolType.CONSTRUCTOR if name == "constructor" else SymbolType.METHOD
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=sym_type,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_name,
                    ))
            elif child.type == "public_field_definition":
                name = None
                for c in child.children:
                    if c.type == "property_identifier":
                        name = c.text.decode("utf-8")
                        break
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.PROPERTY,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_name,
                    ))

    def _get_name(self, node) -> str | None:
        for child in node.children:
            if child.type in ("identifier", "type_identifier"):
                return child.text.decode("utf-8")
        return None

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports = []
        for child in tree.root_node.children:
            if child.type == "import_statement":
                source_path = self._extract_string_source(child)
                if source_path:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8").rstrip(";").strip(),
                        target_name=source_path,
                        line=child.start_point[0] + 1,
                    ))
            # Re-exports: export { X } from './module'
            elif child.type == "export_statement":
                source_path = self._extract_string_source(child)
                if source_path:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8").rstrip(";").strip(),
                        target_name=source_path,
                        line=child.start_point[0] + 1,
                    ))
        return imports

    def _extract_string_source(self, node) -> str | None:
        """Extract the string path from an import or export statement node."""
        for c in node.children:
            if c.type == "string":
                for sc in c.children:
                    if sc.type == "string_fragment":
                        return sc.text.decode("utf-8")
        return None

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        calls: list[RawCall] = []
        exclusions = self.builtin_exclusions()
        self._find_calls(tree.root_node, file_path, calls, exclusions)
        return calls

    def _find_calls(self, node, file_path, calls, exclusions):
        if node.type in ("call_expression", "new_expression"):
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
        if first.type == "new":
            # new_expression: skip 'new' keyword
            for c in node.children[1:]:
                if c.type in ("identifier", "type_identifier"):
                    return c.text.decode("utf-8"), None
            return None, None
        if first.type in ("identifier", "type_identifier"):
            return first.text.decode("utf-8"), None
        if first.type == "member_expression":
            parts = []
            for c in first.children:
                if c.type in ("identifier", "property_identifier", "type_identifier"):
                    parts.append(c.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        return None, None

    def _find_enclosing(self, node):
        current = node.parent
        while current:
            if current.type in ("method_definition", "function_declaration"):
                for c in current.children:
                    if c.type in ("identifier", "property_identifier"):
                        return c.text.decode("utf-8")
            if current.type == "variable_declarator":
                for c in current.children:
                    if c.type == "identifier":
                        return c.text.decode("utf-8")
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            "console.log", "console.error", "console.warn", "console.info",
            "console.debug", "console.trace", "console.dir",
            "setTimeout", "setInterval", "clearTimeout", "clearInterval",
            "Promise.resolve", "Promise.reject", "Promise.all", "Promise.race",
            "JSON.stringify", "JSON.parse",
            "Array.from", "Array.isArray",
            "Object.keys", "Object.values", "Object.entries", "Object.assign",
            "Object.freeze", "Object.create",
            "parseInt", "parseFloat", "isNaN", "isFinite",
            "encodeURIComponent", "decodeURIComponent",
            "require",
        }
