"""C# language analyser."""

from __future__ import annotations

import tree_sitter
import tree_sitter_c_sharp as ts_csharp

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility

# Map C# AST node types to our SymbolType
_TYPE_MAP = {
    "class_declaration": SymbolType.CLASS,
    "interface_declaration": SymbolType.INTERFACE,
    "struct_declaration": SymbolType.STRUCT,
    "enum_declaration": SymbolType.ENUM,
    "method_declaration": SymbolType.METHOD,
    "constructor_declaration": SymbolType.CONSTRUCTOR,
    "property_declaration": SymbolType.PROPERTY,
    "namespace_declaration": SymbolType.NAMESPACE,
    "record_declaration": SymbolType.RECORD,
    "delegate_declaration": SymbolType.DELEGATE,
}

# C# modifier keywords to Visibility
_VISIBILITY_MAP = {
    "public": Visibility.PUBLIC,
    "private": Visibility.PRIVATE,
    "internal": Visibility.INTERNAL,
    "protected": Visibility.PROTECTED,
}

# Node types that can contain child symbols
_CONTAINER_TYPES = {
    "class_declaration", "interface_declaration", "struct_declaration",
    "record_declaration", "namespace_declaration",
}


class CSharpAnalyser:
    extensions = [".cs"]
    language_name = "cs"

    def get_language(self) -> tree_sitter.Language:
        return tree_sitter.Language(ts_csharp.language())

    def extract_symbols(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[Symbol]:
        symbols: list[Symbol] = []
        self._walk_node(tree.root_node, source, file_path, symbols, parent_id=None)
        return symbols

    def _walk_node(
        self,
        node: tree_sitter.Node,
        source: bytes,
        file_path: str,
        symbols: list[Symbol],
        parent_id: str | None,
    ) -> None:
        """Recursively walk AST nodes to extract symbol definitions."""
        for child in node.children:
            symbol_type = _TYPE_MAP.get(child.type)
            if symbol_type is not None:
                name = self._get_name(child)
                if name is None:
                    continue

                visibility = self._get_visibility(child)

                # Namespaces don't have visibility modifiers in C#
                if symbol_type == SymbolType.NAMESPACE:
                    visibility = Visibility.UNKNOWN

                exported = visibility in (Visibility.PUBLIC, Visibility.INTERNAL)

                # Use a placeholder ID - will be replaced by parsing phase
                sym_id = f"_pending_{len(symbols)}"
                # Extract constructor parameter types for DI resolution
                param_types = None
                if child.type == "constructor_declaration":
                    param_types = self._extract_parameter_types(child)

                sym = Symbol(
                    id=sym_id,
                    name=name,
                    type=symbol_type,
                    file=file_path,
                    line=child.start_point[0] + 1,
                    visibility=visibility,
                    exported=exported,
                    parent=parent_id,
                    byte_range=(child.start_byte, child.end_byte),
                    parameter_types=param_types,
                )
                symbols.append(sym)

                # Recurse into containers for nested symbols
                if child.type in _CONTAINER_TYPES:
                    decl_list = child.child_by_field_name("body")
                    if decl_list is None:
                        # Try declaration_list for namespaces/classes
                        for c in child.children:
                            if c.type == "declaration_list":
                                decl_list = c
                                break
                    if decl_list:
                        self._walk_node(decl_list, source, file_path, symbols, parent_id=name)
            elif child.type == "file_scoped_namespace_declaration":
                # Handle file-scoped namespaces (C# 10+): namespace MyApp;
                name = self._get_name(child)
                if name:
                    sym = Symbol(
                        id=f"_pending_{len(symbols)}",
                        name=name,
                        type=SymbolType.NAMESPACE,
                        file=file_path,
                        line=child.start_point[0] + 1,
                        visibility=Visibility.UNKNOWN,
                        exported=True,
                        parent=parent_id,
                    )
                    symbols.append(sym)
                    # File-scoped namespace contains the rest of the file
                    self._walk_node(child, source, file_path, symbols, parent_id=name)

    def _get_name(self, node: tree_sitter.Node) -> str | None:
        """Extract the identifier name from a declaration node."""
        # Prefer the 'name' field - handles methods returning custom types
        # where the first identifier is the return type, not the name
        name_node = node.child_by_field_name("name")
        if name_node:
            return name_node.text.decode("utf-8")
        # Fallback for constructors and other nodes without a name field
        for child in node.children:
            if child.type == "identifier":
                return child.text.decode("utf-8")
            if child.type == "qualified_name":
                return child.text.decode("utf-8")
        return None

    def _get_visibility(self, node: tree_sitter.Node) -> Visibility:
        """Extract visibility from modifier nodes."""
        for child in node.children:
            if child.type == "modifier":
                mod_text = child.text.decode("utf-8").lower()
                vis = _VISIBILITY_MAP.get(mod_text)
                if vis:
                    return vis
        return Visibility.PRIVATE  # C# default is private

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports: list[ImportStatement] = []
        for child in tree.root_node.children:
            if child.type == "using_directive":
                # Extract the namespace from the using directive
                name_node = None
                for c in child.children:
                    if c.type in ("identifier", "qualified_name"):
                        name_node = c
                        break
                    if c.type == "name":
                        name_node = c
                        break
                if name_node:
                    target = name_node.text.decode("utf-8")
                    statement = child.text.decode("utf-8").rstrip(";").strip()
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=statement,
                        target_name=target,
                        line=child.start_point[0] + 1,
                    ))
            elif child.type in ("namespace_declaration", "file_scoped_namespace_declaration"):
                # Check for using directives inside namespace
                for ns_child in child.children:
                    if ns_child.type == "declaration_list":
                        for decl_child in ns_child.children:
                            if decl_child.type == "using_directive":
                                name_node = None
                                for c in decl_child.children:
                                    if c.type in ("identifier", "qualified_name"):
                                        name_node = c
                                        break
                                if name_node:
                                    target = name_node.text.decode("utf-8")
                                    statement = decl_child.text.decode("utf-8").rstrip(";").strip()
                                    imports.append(ImportStatement(
                                        file=file_path,
                                        statement=statement,
                                        target_name=target,
                                        line=decl_child.start_point[0] + 1,
                                    ))
        return imports

    def extract_calls(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[RawCall]:
        calls: list[RawCall] = []
        exclusions = self.builtin_exclusions()
        self._find_calls(tree.root_node, source, file_path, calls, exclusions)
        return calls

    def _find_calls(
        self,
        node: tree_sitter.Node,
        source: bytes,
        file_path: str,
        calls: list[RawCall],
        exclusions: set[str],
    ) -> None:
        """Recursively find call expressions in the AST."""
        if node.type == "invocation_expression":
            callee_name, qualifier = self._extract_callee(node)
            if callee_name and callee_name not in exclusions:
                qualified = f"{qualifier}.{callee_name}" if qualifier else callee_name
                if qualified not in exclusions:
                    caller = self._find_enclosing_method(node)
                    calls.append(RawCall(
                        caller_file=file_path,
                        caller_name=caller or "<module>",
                        callee_name=callee_name,
                        line=node.start_point[0] + 1,
                        qualifier=qualifier,
                    ))
        elif node.type == "object_creation_expression":
            # new SomeClass() is a call to the constructor
            callee_name = None
            for child in node.children:
                if child.type == "identifier":
                    callee_name = child.text.decode("utf-8")
                    break
                if child.type == "qualified_name":
                    callee_name = child.text.decode("utf-8")
                    break
            if callee_name and callee_name not in exclusions:
                caller = self._find_enclosing_method(node)
                calls.append(RawCall(
                    caller_file=file_path,
                    caller_name=caller or "<module>",
                    callee_name=callee_name,
                    line=node.start_point[0] + 1,
                    qualifier=None,
                ))

        for child in node.children:
            self._find_calls(child, source, file_path, calls, exclusions)

    def _extract_callee(self, inv_node: tree_sitter.Node) -> tuple[str | None, str | None]:
        """Extract callee name and qualifier from an invocation_expression."""
        first_child = inv_node.children[0] if inv_node.children else None
        if first_child is None:
            return None, None

        if first_child.type == "identifier":
            return first_child.text.decode("utf-8"), None
        elif first_child.type == "member_access_expression":
            # e.g., _service.CalculateEntitlement or Console.WriteLine
            parts = []
            for child in first_child.children:
                if child.type == "identifier":
                    parts.append(child.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif len(parts) == 1:
                return parts[0], None
        elif first_child.type == "qualified_name":
            text = first_child.text.decode("utf-8")
            parts = text.rsplit(".", 1)
            if len(parts) == 2:
                return parts[1], parts[0]
            return text, None

        return None, None

    def _find_enclosing_method(self, node: tree_sitter.Node) -> str | None:
        """Walk up the AST to find the enclosing method or constructor.

        Stops at property/event/operator declarations to prevent attributing
        calls in accessors to the wrong symbol.
        """
        current = node.parent
        while current:
            if current.type in ("method_declaration", "constructor_declaration",
                                "local_function_statement"):
                name_node = current.child_by_field_name("name")
                if name_node:
                    return name_node.text.decode("utf-8")
                for child in current.children:
                    if child.type == "identifier":
                        return child.text.decode("utf-8")
            # Stop at property/event/operator boundaries - calls inside
            # these should not be attributed to an outer method
            if current.type in ("property_declaration", "event_declaration",
                                "operator_declaration", "indexer_declaration"):
                return None
            current = current.parent
        return None

    def _extract_parameter_types(self, node: tree_sitter.Node) -> list[tuple[str, str]] | None:
        """Extract (param_name, type_name) pairs from a constructor's parameter_list."""
        param_list = node.child_by_field_name("parameters")
        if param_list is None:
            return None
        params = []
        for child in param_list.children:
            if child.type == "parameter":
                type_node = child.child_by_field_name("type")
                name_node = child.child_by_field_name("name")
                if type_node and name_node:
                    type_name = type_node.text.decode("utf-8")
                    param_name = name_node.text.decode("utf-8")
                    params.append((param_name, type_name))
        return params if params else None

    def builtin_exclusions(self) -> set[str]:
        return {
            # Framework types (not real calls)
            "Task", "ValueTask",
            # Console
            "Console.WriteLine", "Console.ReadLine", "Console.Write",
            "Console.ReadKey", "Console.Clear",
            # String
            "String.Format", "String.IsNullOrEmpty", "String.IsNullOrWhiteSpace",
            "String.Join", "String.Concat", "String.Compare",
            "string.Format", "string.IsNullOrEmpty", "string.IsNullOrWhiteSpace",
            "string.Join", "string.Concat", "string.Compare",
            # Convert
            "Convert.ToInt32", "Convert.ToString", "Convert.ToDecimal",
            "Convert.ToDouble", "Convert.ToBoolean", "Convert.ToDateTime",
            # Math
            "Math.Abs", "Math.Max", "Math.Min", "Math.Round",
            "Math.Floor", "Math.Ceiling", "Math.Pow", "Math.Sqrt",
            # Object
            "ToString", "Equals", "GetHashCode", "GetType",
            # Debug/Trace
            "Debug.WriteLine", "Debug.Assert", "Debug.Print",
            "Trace.WriteLine", "Trace.TraceInformation",
            # GC
            "GC.Collect", "GC.SuppressFinalize",
            # Task
            "Task.Run", "Task.WhenAll", "Task.WhenAny", "Task.Delay",
            "Task.FromResult", "Task.CompletedTask",
            # LINQ
            "Select", "Where", "FirstOrDefault", "First", "Last",
            "LastOrDefault", "SingleOrDefault", "Single", "Any", "All",
            "Count", "Sum", "Average", "Min", "Max", "OrderBy",
            "OrderByDescending", "GroupBy", "ToList", "ToArray",
            "ToDictionary", "AsEnumerable", "AsQueryable",
            "Skip", "Take", "Distinct", "Union", "Intersect", "Except",
            "Aggregate", "Zip", "SelectMany", "Contains",
            # Common framework
            "Dispose", "Close",
        }
