"""Java language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_java as ts_java

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility

_TYPE_MAP = {
    "class_declaration": SymbolType.CLASS,
    "interface_declaration": SymbolType.INTERFACE,
    "enum_declaration": SymbolType.ENUM,
    "method_declaration": SymbolType.METHOD,
    "constructor_declaration": SymbolType.CONSTRUCTOR,
    "record_declaration": SymbolType.RECORD,
    "annotation_type_declaration": SymbolType.ANNOTATION,
}

_VISIBILITY_MAP = {
    "public": Visibility.PUBLIC,
    "private": Visibility.PRIVATE,
    "protected": Visibility.PROTECTED,
}

_CONTAINER_TYPES = {"class_declaration", "interface_declaration", "enum_declaration"}


class JavaAnalyser:
    extensions = [".java"]
    language_name = "java"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_java.language())

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
                visibility = self._get_visibility(child)
                exported = visibility == Visibility.PUBLIC

                symbols.append(Symbol(
                    id=f"_pending_{len(symbols)}",
                    name=name,
                    type=sym_type,
                    file=file_path,
                    line=child.start_point[0] + 1,
                    visibility=visibility,
                    exported=exported,
                    parent=parent_id,
                ))

                if child.type in _CONTAINER_TYPES:
                    for c in child.children:
                        if c.type in ("class_body", "interface_body", "enum_body"):
                            self._walk_node(c, source, file_path, symbols, parent_id=name)

    def _get_name(self, node) -> str | None:
        for child in node.children:
            if child.type == "identifier":
                return child.text.decode("utf-8")
        return None

    def _get_visibility(self, node) -> Visibility:
        for child in node.children:
            if child.type == "modifiers":
                for mod in child.children:
                    text = mod.text.decode("utf-8").lower() if mod.child_count == 0 else ""
                    vis = _VISIBILITY_MAP.get(text)
                    if vis:
                        return vis
        return Visibility.INTERNAL  # Java default is package-private

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports = []
        for child in tree.root_node.children:
            if child.type == "import_declaration":
                target = None
                for c in child.children:
                    if c.type == "scoped_identifier":
                        target = c.text.decode("utf-8")
                if target:
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=child.text.decode("utf-8").rstrip(";").strip(),
                        target_name=target,
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
        if node.type == "method_invocation":
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
        elif node.type == "object_creation_expression":
            for c in node.children:
                if c.type in ("identifier", "type_identifier"):
                    name = c.text.decode("utf-8")
                    if name not in exclusions:
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
        # method_invocation children: [object, '.', method_name, argument_list]
        # or just [method_name, argument_list] for unqualified calls
        has_dot = any(c.type == "." for c in node.children)
        if has_dot:
            # Qualified call: collect identifiers, last one is the method name
            parts = []
            for child in node.children:
                if child.type == "identifier":
                    parts.append(child.text.decode("utf-8"))
                elif child.type == "field_access":
                    parts.append(child.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif parts:
                return parts[0], None
        else:
            for child in node.children:
                if child.type == "identifier":
                    return child.text.decode("utf-8"), None
        return None, None

    def _find_enclosing(self, node):
        current = node.parent
        while current:
            if current.type in ("method_declaration", "constructor_declaration"):
                for c in current.children:
                    if c.type == "identifier":
                        return c.text.decode("utf-8")
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            "System.out.println", "System.out.print", "System.err.println",
            "System.out.printf", "System.exit",
            "Objects.equals", "Objects.hash", "Objects.requireNonNull",
            "Arrays.asList", "Arrays.sort", "Arrays.copyOf",
            "Collections.sort", "Collections.unmodifiableList",
            "String.valueOf", "String.format", "String.join",
            "Integer.parseInt", "Integer.valueOf",
            "Math.abs", "Math.max", "Math.min", "Math.round",
            "toString", "equals", "hashCode", "getClass",
            "println", "printf",
        }
