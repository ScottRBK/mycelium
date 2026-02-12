"""C and C++ language analysers."""

from __future__ import annotations

import tree_sitter
import tree_sitter_c as ts_c
import tree_sitter_cpp as ts_cpp

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility


class _CBaseMixin:
    """Shared functionality for C and C++ analysers."""

    _PREPROC_CONTAINERS = {"preproc_ifdef", "preproc_ifndef", "preproc_if", "preproc_else", "preproc_elif"}

    def _extract_symbols_from_node(self, node, source, file_path, symbols, parent_id=None):
        for child in node.children:
            if child.type == "function_definition":
                name = self._get_func_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.FUNCTION,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type == "struct_specifier":
                name = self._get_type_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.STRUCT,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type == "enum_specifier":
                name = self._get_type_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.ENUM,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type == "type_definition":
                name = self._get_typedef_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.TYPEDEF,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type == "declaration":
                # Forward declarations of functions
                name = self._get_func_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.FUNCTION,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type in self._PREPROC_CONTAINERS:
                self._extract_symbols_from_node(child, source, file_path, symbols, parent_id)

    def _get_func_name(self, node) -> str | None:
        for child in node.children:
            if child.type == "function_declarator":
                for c in child.children:
                    if c.type == "identifier":
                        return c.text.decode("utf-8")
            if child.type == "pointer_declarator":
                result = self._get_func_name(child)
                if result:
                    return result
            if child.type == "identifier":
                return child.text.decode("utf-8")
        return None

    def _get_type_name(self, node) -> str | None:
        for child in node.children:
            if child.type == "type_identifier":
                return child.text.decode("utf-8")
        return None

    def _get_typedef_name(self, node) -> str | None:
        for child in node.children:
            if child.type == "type_identifier":
                return child.text.decode("utf-8")
        return None

    def _extract_includes(self, tree, file_path):
        imports = []
        for child in tree.root_node.children:
            if child.type == "preproc_include":
                path = None
                for c in child.children:
                    if c.type == "string_literal":
                        for sc in c.children:
                            if sc.type == "string_content":
                                path = sc.text.decode("utf-8")
                    elif c.type == "system_lib_string":
                        path = c.text.decode("utf-8").strip("<>")
                if path:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8").strip(),
                        target_name=path,
                        line=child.start_point[0] + 1,
                    ))
        return imports

    def _find_call_expressions(self, node, file_path, calls, exclusions):
        if node.type == "call_expression":
            callee_name, qualifier = self._extract_callee(node)
            if callee_name and callee_name not in exclusions:
                qualified = f"{qualifier}.{callee_name}" if qualifier else callee_name
                if qualified not in exclusions:
                    caller = self._find_enclosing_func(node)
                    calls.append(RawCall(
                        caller_file=file_path,
                        caller_name=caller or "<module>",
                        callee_name=callee_name,
                        line=node.start_point[0] + 1,
                        qualifier=qualifier,
                    ))
        for child in node.children:
            self._find_call_expressions(child, file_path, calls, exclusions)

    def _extract_callee(self, node):
        first = node.children[0] if node.children else None
        if first is None:
            return None, None
        if first.type == "identifier":
            return first.text.decode("utf-8"), None
        if first.type == "field_expression":
            parts = []
            for c in first.children:
                if c.type in ("identifier", "field_identifier"):
                    parts.append(c.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        if first.type == "qualified_identifier":
            text = first.text.decode("utf-8")
            parts = text.rsplit("::", 1)
            if len(parts) == 2:
                return parts[1], parts[0]
            return text, None
        return None, None

    def _find_enclosing_func(self, node):
        current = node.parent
        while current:
            if current.type == "function_definition":
                return self._get_func_name(current)
            current = current.parent
        return None


class CAnalyser(_CBaseMixin):
    extensions = [".c", ".h"]
    language_name = "c"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_c.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._extract_symbols_from_node(tree.root_node, source, file_path, symbols)
        return symbols

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        return self._extract_includes(tree, file_path)

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        calls: list[RawCall] = []
        self._find_call_expressions(tree.root_node, file_path, calls, self.builtin_exclusions())
        return calls

    def builtin_exclusions(self) -> set[str]:
        return {
            "printf", "fprintf", "sprintf", "snprintf", "scanf", "sscanf",
            "malloc", "calloc", "realloc", "free",
            "memcpy", "memset", "memmove", "memcmp",
            "strlen", "strcmp", "strncmp", "strcpy", "strncpy", "strcat",
            "sizeof", "assert", "exit", "abort",
            "fopen", "fclose", "fread", "fwrite", "fgets", "fputs",
        }


class CppAnalyser(_CBaseMixin):
    extensions = [".cpp", ".cc", ".cxx", ".hpp", ".hxx", ".hh"]
    language_name = "cpp"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_cpp.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._extract_cpp_symbols(tree.root_node, source, file_path, symbols, parent_id=None)
        return symbols

    def _extract_cpp_symbols(self, node, source, file_path, symbols, parent_id):
        self._extract_symbols_from_node(node, source, file_path, symbols, parent_id)

        for child in node.children:
            if child.type == "class_specifier":
                name = self._get_type_name(child)
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.CLASS,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
            elif child.type == "namespace_definition":
                name = None
                for c in child.children:
                    if c.type == "namespace_identifier":
                        name = c.text.decode("utf-8")
                        break
                if name:
                    symbols.append(Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.NAMESPACE,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.PUBLIC,
                        exported=True,
                        parent=parent_id,
                    ))
                    # Recurse into namespace
                    for c in child.children:
                        if c.type == "declaration_list":
                            self._extract_cpp_symbols(c, source, file_path, symbols, parent_id=name)

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        return self._extract_includes(tree, file_path)

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        calls: list[RawCall] = []
        self._find_call_expressions(tree.root_node, file_path, calls, self.builtin_exclusions())
        return calls

    def builtin_exclusions(self) -> set[str]:
        return {
            "printf", "malloc", "free", "memcpy", "memset", "strlen", "strcmp",
            "sizeof", "assert", "exit", "abort",
            "std::cout", "std::cerr", "std::endl",
            "std::move", "std::forward",
            "std::make_shared", "std::make_unique", "std::make_pair",
            "std::sort", "std::find", "std::begin", "std::end",
            "std::string", "std::to_string",
        }
