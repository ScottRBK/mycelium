"""VB.NET language analyser.

The VB.NET grammar requires tree-sitter-vb-dotnet or tree-sitter-language-pack
with VB.NET support. If unavailable, VB.NET files are skipped with a warning.
To build from source: pip install git+https://github.com/CodeAnt-AI/tree-sitter-vb-dotnet.git
"""

from __future__ import annotations

import logging

import tree_sitter

from mycelium.config import ImportStatement, RawCall, Symbol, SymbolType, Visibility

logger = logging.getLogger(__name__)

_vbnet_language: tree_sitter.Language | None = None
_vbnet_init_attempted = False

# Map VB.NET AST node types to our SymbolType
# These match the CodeAnt-AI/tree-sitter-vb-dotnet grammar
_TYPE_MAP = {
    "class_block": SymbolType.CLASS,
    "interface_block": SymbolType.INTERFACE,
    "structure_block": SymbolType.STRUCT,
    "enum_block": SymbolType.ENUM,
    "method_declaration": SymbolType.METHOD,
    "constructor_declaration": SymbolType.CONSTRUCTOR,
    "property_declaration": SymbolType.PROPERTY,
    "namespace_block": SymbolType.NAMESPACE,
    "module_block": SymbolType.MODULE,
    "delegate_declaration": SymbolType.DELEGATE,
    # Fallback for older/alternative grammars
    "class_statement": SymbolType.CLASS,
    "interface_statement": SymbolType.INTERFACE,
    "structure_statement": SymbolType.STRUCT,
    "enum_statement": SymbolType.ENUM,
    "sub_statement": SymbolType.METHOD,
    "function_statement": SymbolType.METHOD,
    "property_statement": SymbolType.PROPERTY,
    "namespace_statement": SymbolType.NAMESPACE,
    "module_statement": SymbolType.MODULE,
}

# VB.NET modifier keywords to Visibility
_VISIBILITY_MAP = {
    "public": Visibility.PUBLIC,
    "private": Visibility.PRIVATE,
    "friend": Visibility.FRIEND,
    "protected": Visibility.PROTECTED,
    "internal": Visibility.INTERNAL,
}

_CONTAINER_TYPES = {
    "class_block", "interface_block", "structure_block",
    "module_block", "namespace_block",
    # Fallback types
    "class_statement", "interface_statement", "structure_statement",
    "module_statement", "namespace_statement",
}


def _try_load_vbnet() -> tree_sitter.Language | None:
    """Try to load VB.NET grammar."""
    global _vbnet_language, _vbnet_init_attempted

    if _vbnet_init_attempted:
        return _vbnet_language

    _vbnet_init_attempted = True

    # Try tree_sitter_vb_net package
    try:
        import tree_sitter_vb_net as ts_vbnet
        _vbnet_language = tree_sitter.Language(ts_vbnet.language())
        logger.info("Loaded VB.NET grammar from tree-sitter-vb-net")
        return _vbnet_language
    except ImportError:
        pass

    # Try tree_sitter_vb_dotnet package
    try:
        import tree_sitter_vb_dotnet as ts_vbdotnet
        _vbnet_language = tree_sitter.Language(ts_vbdotnet.language())
        logger.info("Loaded VB.NET grammar from tree-sitter-vb-dotnet")
        return _vbnet_language
    except ImportError:
        pass

    # Try the doubled-prefix name (CodeAnt-AI scaffolding bug)
    try:
        import tree_sitter_tree_sitter_vb_dotnet as ts_vb_doubled
        _vbnet_language = tree_sitter.Language(ts_vb_doubled.language())
        logger.info("Loaded VB.NET grammar from tree-sitter-tree-sitter-vb-dotnet")
        return _vbnet_language
    except ImportError:
        pass

    # Try language pack
    try:
        from tree_sitter_language_pack import get_language
        for name in ("visual_basic", "vb_net", "vb"):
            try:
                _vbnet_language = get_language(name)
                logger.info(f"Loaded VB.NET grammar from language-pack as '{name}'")
                return _vbnet_language
            except Exception:
                continue
    except ImportError:
        pass

    logger.warning(
        "VB.NET grammar not available. Install tree-sitter-vb-dotnet "
        "(pip install git+https://github.com/CodeAnt-AI/tree-sitter-vb-dotnet.git) or install "
        "tree-sitter-language-pack with VB.NET support. "
        "VB.NET files will be skipped."
    )
    return None


class VBNetAnalyser:
    extensions = [".vb"]
    language_name = "vb"

    def get_language(self) -> tree_sitter.Language:
        lang = _try_load_vbnet()
        if lang is None:
            raise RuntimeError("VB.NET grammar not available")
        return lang

    def is_available(self) -> bool:
        return _try_load_vbnet() is not None

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
        for child in node.children:
            symbol_type = _TYPE_MAP.get(child.type)
            if symbol_type is not None:
                name = self._get_name(child)
                if name is None:
                    continue

                visibility = self._get_visibility(child)

                # Namespaces don't have visibility modifiers in VB.NET
                if symbol_type == SymbolType.NAMESPACE:
                    visibility = Visibility.UNKNOWN

                exported = visibility in (Visibility.PUBLIC, Visibility.FRIEND)

                # Extract constructor parameter types
                param_types = None
                if symbol_type == SymbolType.CONSTRUCTOR:
                    param_types = self._extract_parameter_types(child)

                sym = Symbol(
                    id=f"_pending_{len(symbols)}",
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

                if child.type in _CONTAINER_TYPES:
                    self._walk_node(child, source, file_path, symbols, parent_id=name)
            else:
                # Recurse into unknown containers
                if child.child_count > 0:
                    self._walk_node(child, source, file_path, symbols, parent_id=parent_id)

    def _get_name(self, node: tree_sitter.Node) -> str | None:
        # Try 'name' field first (most reliable)
        name_node = node.child_by_field_name("name")
        if name_node:
            return name_node.text.decode("utf-8")
        # Try direct identifier children
        for child in node.children:
            if child.type == "identifier" or child.type == "name":
                return child.text.decode("utf-8")
            if child.type == "qualified_name" or child.type == "namespace_name":
                return child.text.decode("utf-8")
        return None

    def _get_visibility(self, node: tree_sitter.Node) -> Visibility:
        for child in node.children:
            if child.type in ("access_modifier", "modifier"):
                mod_text = child.text.decode("utf-8").lower()
                vis = _VISIBILITY_MAP.get(mod_text)
                if vis:
                    return vis
            elif child.type in ("Public", "Private", "Friend", "Protected"):
                return _VISIBILITY_MAP.get(child.type.lower(), Visibility.PUBLIC)
        return Visibility.PUBLIC  # VB.NET default is Public

    def _extract_parameter_types(self, node: tree_sitter.Node) -> list[tuple[str, str]] | None:
        """Extract parameter (name, type) pairs from a constructor/method."""
        params = []
        for child in node.children:
            if child.type in ("parameter_list", "parameters"):
                for param in child.children:
                    if param.type == "parameter":
                        p_name = None
                        p_type = None
                        name_node = param.child_by_field_name("name")
                        type_node = param.child_by_field_name("type")
                        if name_node:
                            p_name = name_node.text.decode("utf-8")
                        if type_node:
                            p_type = type_node.text.decode("utf-8")
                        # VB.NET: As clause for type
                        if not p_type:
                            for pc in param.children:
                                if pc.type == "as_clause":
                                    for tc in pc.children:
                                        if tc.type in ("identifier", "qualified_name", "predefined_type"):
                                            p_type = tc.text.decode("utf-8")
                                            break
                        if not p_name:
                            for pc in param.children:
                                if pc.type == "identifier":
                                    p_name = pc.text.decode("utf-8")
                                    break
                        if p_name and p_type:
                            params.append((p_name, p_type))
        return params if params else None

    def extract_imports(
        self, tree: tree_sitter.Tree, source: bytes, file_path: str
    ) -> list[ImportStatement]:
        imports: list[ImportStatement] = []
        self._find_imports(tree.root_node, source, file_path, imports)
        return imports

    def _find_imports(
        self,
        node: tree_sitter.Node,
        source: bytes,
        file_path: str,
        imports: list[ImportStatement],
    ) -> None:
        """Find Imports statements in VB.NET source."""
        for child in node.children:
            if child.type in ("imports_statement", "imports_clause"):
                target = self._extract_import_target(child)
                if target:
                    statement = child.text.decode("utf-8").strip()
                    imports.append(ImportStatement(
                        file=file_path,
                        statement=statement,
                        target_name=target,
                        line=child.start_point[0] + 1,
                    ))
            else:
                # Recurse into containers (namespace blocks etc.)
                if child.child_count > 0:
                    self._find_imports(child, source, file_path, imports)

    def _extract_import_target(self, node: tree_sitter.Node) -> str | None:
        """Extract the namespace target from an imports statement."""
        # Try namespace field
        ns_node = node.child_by_field_name("namespace")
        if ns_node:
            return ns_node.text.decode("utf-8")
        # Look for namespace_name, qualified_name, or identifier children
        for child in node.children:
            if child.type in ("namespace_name", "qualified_name", "identifier"):
                return child.text.decode("utf-8")
            # Handle aliased imports: Imports alias = Namespace
            if child.type == "imports_clause":
                return self._extract_import_target(child)
            # Handle simple member imports
            if child.type == "simple_member_import":
                for sc in child.children:
                    if sc.type in ("namespace_name", "qualified_name", "identifier"):
                        return sc.text.decode("utf-8")
        return None

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
        """Recursively find call expressions in VB.NET AST."""
        if node.type in ("invocation", "invocation_expression"):
            callee_name, qualifier = self._extract_vb_callee(node)
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
        elif node.type in ("new_expression", "object_creation_expression"):
            # New SomeClass() constructor call
            callee_name = self._extract_type_name(node)
            if callee_name and callee_name not in exclusions:
                caller = self._find_enclosing_method(node)
                calls.append(RawCall(
                    caller_file=file_path,
                    caller_name=caller or "<module>",
                    callee_name=callee_name,
                    line=node.start_point[0] + 1,
                    qualifier=None,
                ))
        elif node.type == "call_statement":
            # Legacy VB Call keyword
            callee_name, qualifier = self._extract_vb_callee(node)
            if callee_name and callee_name not in exclusions:
                caller = self._find_enclosing_method(node)
                calls.append(RawCall(
                    caller_file=file_path,
                    caller_name=caller or "<module>",
                    callee_name=callee_name,
                    line=node.start_point[0] + 1,
                    qualifier=qualifier,
                ))

        for child in node.children:
            self._find_calls(child, source, file_path, calls, exclusions)

    def _extract_vb_callee(self, node: tree_sitter.Node) -> tuple[str | None, str | None]:
        """Extract callee name and qualifier from a VB.NET call node."""
        # Try target field
        target = node.child_by_field_name("target")
        if target is None:
            target = node.children[0] if node.children else None
        if target is None:
            return None, None

        if target.type == "identifier":
            return target.text.decode("utf-8"), None
        elif target.type in ("member_access", "member_access_expression"):
            parts = []
            for child in target.children:
                if child.type == "identifier":
                    parts.append(child.text.decode("utf-8"))
            if len(parts) >= 2:
                return parts[-1], parts[-2]
            elif len(parts) == 1:
                return parts[0], None
        elif target.type in ("qualified_name",):
            text = target.text.decode("utf-8")
            parts = text.rsplit(".", 1)
            if len(parts) == 2:
                return parts[1], parts[0]
            return text, None

        # Fallback: find first identifier
        for child in target.children:
            if child.type == "identifier":
                return child.text.decode("utf-8"), None
        return None, None

    def _extract_type_name(self, node: tree_sitter.Node) -> str | None:
        """Extract type name from a New expression."""
        type_node = node.child_by_field_name("type")
        if type_node:
            return type_node.text.decode("utf-8")
        for child in node.children:
            if child.type in ("identifier", "qualified_name"):
                return child.text.decode("utf-8")
        return None

    def _find_enclosing_method(self, node: tree_sitter.Node) -> str | None:
        """Walk up the AST to find the enclosing method or constructor."""
        current = node.parent
        while current:
            if current.type in (
                "method_declaration", "constructor_declaration",
                "sub_statement", "function_statement",
                "local_function_statement",
            ):
                name_node = current.child_by_field_name("name")
                if name_node:
                    return name_node.text.decode("utf-8")
                for child in current.children:
                    if child.type == "identifier":
                        return child.text.decode("utf-8")
            current = current.parent
        return None

    def builtin_exclusions(self) -> set[str]:
        return {
            # Framework types
            "Task", "ValueTask",
            # VB-specific functions
            "MsgBox", "InputBox", "CType", "CStr", "CInt", "CDbl", "CBool",
            "CLng", "CSng", "CDate", "CByte", "CShort", "CUInt", "CULng",
            "CObj", "CDec", "CChar", "CSByte", "CUShort",
            "DirectCast", "TryCast", "IsNothing", "IsDBNull",
            "Val", "Asc", "Chr", "Len", "Mid", "Left", "Right",
            "LCase", "UCase", "Trim", "LTrim", "RTrim",
            "InStr", "Replace", "Split", "Join",
            # My namespace
            "My.Computer", "My.Application", "My.Settings", "My.User",
            # Console
            "Console.WriteLine", "Console.ReadLine", "Console.Write",
            # String
            "String.Format", "String.IsNullOrEmpty", "String.IsNullOrWhiteSpace",
            # Convert
            "Convert.ToInt32", "Convert.ToString", "Convert.ToDecimal",
            # Object
            "ToString", "Equals", "GetHashCode", "GetType",
            # Debug
            "Debug.WriteLine", "Debug.Assert", "Debug.Print",
            # LINQ
            "Select", "Where", "FirstOrDefault", "First", "Last",
            "SingleOrDefault", "Any", "All", "Count", "Sum",
            "OrderBy", "OrderByDescending", "GroupBy",
            "ToList", "ToArray", "ToDictionary",
            # Common framework
            "Dispose", "Close",
        }
