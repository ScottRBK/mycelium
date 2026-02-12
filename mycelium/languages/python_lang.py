"""Python language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_python as ts_python

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility


class PythonAnalyser:
    extensions = [".py"]
    language_name = "py"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_python.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._walk_node(tree.root_node, source, file_path, symbols, parent_id=None)
        return symbols

    def _walk_node(self, node, source, file_path, symbols, parent_id):
        for child in node.children:
            if child.type == "class_definition":
                name = self._get_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.CLASS,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=not name.startswith("_"),
                        parent=parent_id,
                    ))
                    # Recurse into class body
                    for c in child.children:
                        if c.type == "block":
                            self._walk_node(c, source, file_path, symbols, parent_id=name)

            elif child.type == "function_definition":
                name = self._get_name(child)
                if name:
                    # Determine if it's a method (has parent) or function
                    sym_type = SymbolType.METHOD if parent_id else SymbolType.FUNCTION
                    if name == "__init__":
                        sym_type = SymbolType.CONSTRUCTOR

                    vis = Visibility.PRIVATE if name.startswith("_") and not name.startswith("__") else Visibility.PUBLIC

                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=sym_type,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=vis,
                        exported=not name.startswith("_"),
                        parent=parent_id,
                    ))

            elif child.type == "decorated_definition":
                # Decorated class or function
                for c in child.children:
                    if c.type in ("class_definition", "function_definition"):
                        self._walk_node(child, source, file_path, symbols, parent_id)
                        break

    def _get_name(self, node) -> str | None:
        for child in node.children:
            if child.type == "identifier":
                return child.text.decode("utf-8")
        return None

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports = []
        for child in tree.root_node.children:
            if child.type == "import_statement":
                # import foo, import foo.bar
                for c in child.children:
                    if c.type == "dotted_name":
                        target = c.text.decode("utf-8")
                        imports.append(ImportStatement(
                            file=file_path,
                            statement=child.text.decode("utf-8"),
                            target_name=target,
                            line=child.start_point[0] + 1,
                        ))
            elif child.type == "import_from_statement":
                # from foo import bar
                module = None
                for c in child.children:
                    if c.type == "dotted_name":
                        module = c.text.decode("utf-8")
                        break
                    if c.type == "relative_import":
                        module = c.text.decode("utf-8")
                        break
                if module:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8"),
                        target_name=module,
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
        if node.type == "call":
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
        if first.type == "attribute":
            parts = []
            for c in first.children:
                if c.type == "identifier":
                    parts.append(c.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        return None, None

    def _find_enclosing(self, node):
        current = node.parent
        while current:
            if current.type == "function_definition":
                for c in current.children:
                    if c.type == "identifier":
                        return c.text.decode("utf-8")
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            "print", "len", "range", "enumerate", "zip", "map", "filter",
            "isinstance", "issubclass", "type", "super", "str", "int",
            "float", "list", "dict", "set", "tuple", "bool", "bytes",
            "sorted", "reversed", "any", "all", "min", "max", "sum",
            "abs", "round", "hash", "id", "repr", "format", "open",
            "getattr", "setattr", "hasattr", "delattr", "callable",
            "iter", "next", "input", "ord", "chr", "hex", "oct", "bin",
            "property", "staticmethod", "classmethod",
            "ValueError", "TypeError", "KeyError", "IndexError",
            "RuntimeError", "AttributeError", "Exception",
        }
