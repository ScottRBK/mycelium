"""Tests for all language analysers (Milestone 6)."""

from __future__ import annotations

import os
from pathlib import Path

import pytest
import tree_sitter

from mycelium.config import SymbolType, Visibility

FIXTURES = Path(__file__).parent / "fixtures"


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _parse(analyser, fixture_path: str):
    """Parse a fixture file and return (tree, source, file_path)."""
    full = str(FIXTURES / fixture_path)
    source = Path(full).read_bytes()
    lang = analyser.get_language()
    parser = tree_sitter.Parser(lang)
    tree = parser.parse(source)
    return tree, source, full


def _symbol_names(symbols, sym_type=None):
    """Get set of symbol names, optionally filtered by type."""
    if sym_type:
        return {s.name for s in symbols if s.type == sym_type}
    return {s.name for s in symbols}


def _import_targets(imports):
    """Get set of import target names."""
    return {i.target_name for i in imports}


def _call_names(calls):
    """Get set of callee names from raw calls."""
    return {c.callee_name for c in calls}


# ===========================================================================
# TypeScript / JavaScript
# ===========================================================================


class TestTypeScriptAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.typescript import TypeScriptAnalyser
        return TypeScriptAnalyser()

    def test_extract_symbols_controller(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserController" in names  # class

    def test_extract_symbols_models(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/models.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "User" in names  # interface
        assert "UserRole" in names  # enum
        assert "UserDTO" in names  # type alias

    def test_exported_class(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        ctrl = next(s for s in symbols if s.name == "UserController")
        assert ctrl.exported is True
        assert ctrl.type == SymbolType.CLASS

    def test_class_methods(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "handleGetUser" in methods
        assert "handleCreateUser" in methods

    def test_constructor_detected(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        constructors = _symbol_names(symbols, SymbolType.CONSTRUCTOR)
        assert "constructor" in constructors

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/service.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserService" in names
        assert "findUser" in names
        assert "createUser" in names
        assert "generateId" in names

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "./service" in targets

    def test_extract_calls_controller(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "UserService" in callee_names  # new UserService()
        assert "findUser" in callee_names
        assert "createUser" in callee_names

    def test_extract_calls_service(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/service.ts")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "generateId" in callee_names
        assert "findById" in callee_names
        assert "save" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "console.log" in excl
        assert "JSON.parse" in excl

    def test_enum_and_type_alias(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/models.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        enums = _symbol_names(symbols, SymbolType.ENUM)
        type_aliases = _symbol_names(symbols, SymbolType.TYPE_ALIAS)
        assert "UserRole" in enums
        assert "UserDTO" in type_aliases

    # --- New fixture file tests ---

    def test_extract_symbols_repository(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/repository.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserRepository" in names
        assert "findById" in names
        assert "findAll" in names
        assert "save" in names
        assert "delete" in names
        assert "findByFilter" in names
        assert "count" in names
        assert "exists" in names

    def test_extract_symbols_middleware(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/middleware.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "RequestContext" in names  # interface
        assert "AuthMiddleware" in names  # class
        assert "authenticate" in names
        assert "authorize" in names
        assert "createSession" in names
        assert "generateToken" in names

    def test_middleware_interface(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/middleware.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        ctx = next(s for s in symbols if s.name == "RequestContext")
        assert ctx.type == SymbolType.INTERFACE

    def test_extract_symbols_utils(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/utils.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "hashPassword" in names
        assert "validateEmail" in names
        assert "formatDate" in names
        assert "paginate" in names
        assert "slugify" in names

    def test_utils_are_exported_functions(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/utils.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        for sym in symbols:
            if sym.name in ("hashPassword", "validateEmail", "formatDate", "paginate", "slugify"):
                assert sym.exported is True
                assert sym.type == SymbolType.FUNCTION

    def test_models_interfaces_and_types(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/models.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        interfaces = _symbol_names(symbols, SymbolType.INTERFACE)
        type_aliases = _symbol_names(symbols, SymbolType.TYPE_ALIAS)
        assert "User" in interfaces
        assert "CreateUserRequest" in interfaces
        assert "PaginatedResponse" in interfaces
        assert "UserFilter" in type_aliases

    def test_extract_imports_repository(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/repository.ts")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "./models" in targets

    def test_extract_imports_middleware(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/middleware.ts")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "./models" in targets

    def test_extract_imports_service_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/service.ts")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "./models" in targets
        assert "./repository" in targets
        assert "./utils" in targets

    def test_controller_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "handleDeleteUser" in methods
        assert "handleListUsers" in methods
        assert "validateRequest" in methods

    def test_controller_validate_method(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/controller.ts")
        symbols = analyser.extract_symbols(tree, source, fp)
        validate = next(s for s in symbols if s.name == "validateRequest")
        assert validate.type == SymbolType.METHOD
        assert validate.parent == "UserController"

    def test_extract_calls_middleware(self, analyser):
        tree, source, fp = _parse(analyser, "typescript_simple/middleware.ts")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "generateToken" in callee_names


# ===========================================================================
# Python
# ===========================================================================


class TestPythonAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.python_lang import PythonAnalyser
        return PythonAnalyser()

    def test_extract_symbols_handler(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "RequestHandler" in names
        assert "__init__" in names
        assert "handle_get" in names
        assert "handle_create" in names
        assert "_validate" in names

    def test_class_type(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        cls = next(s for s in symbols if s.name == "RequestHandler")
        assert cls.type == SymbolType.CLASS

    def test_constructor_type(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        init = next(s for s in symbols if s.name == "__init__")
        assert init.type == SymbolType.CONSTRUCTOR

    def test_method_type(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        method = next(s for s in symbols if s.name == "handle_get")
        assert method.type == SymbolType.METHOD
        assert method.parent == "RequestHandler"

    def test_private_method(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        priv = next(s for s in symbols if s.name == "_validate")
        assert priv.visibility == Visibility.PRIVATE
        assert priv.exported is False

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/service.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "DataService" in names
        assert "get_item" in names
        assert "create_item" in names
        assert "delete_item" in names

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "service" in targets

    def test_extract_calls_handler(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "DataService" in callee_names
        assert "get_item" in callee_names
        assert "create_item" in callee_names
        assert "_validate" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "print" in excl
        assert "len" in excl
        assert "isinstance" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_models(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/models.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "ItemCategory" in names  # enum class
        assert "Item" in names  # dataclass
        assert "CreateItemRequest" in names
        assert "PaginatedResult" in names
        assert "to_dict" in names  # method on Item

    def test_extract_symbols_repository(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/repository.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "ItemRepository" in names
        assert "__init__" in names
        assert "find_by_id" in names
        assert "find_all" in names
        assert "save" in names
        assert "delete" in names
        assert "find_by_category" in names
        assert "count" in names
        assert "exists" in names
        assert "find_by_name" in names
        assert "clear" in names

    def test_extract_symbols_exceptions(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/exceptions.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "AppError" in names
        assert "NotFoundError" in names
        assert "ConflictError" in names
        assert "ForbiddenError" in names

    def test_exception_class_types(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/exceptions.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        for cls_name in ("AppError", "NotFoundError", "ConflictError", "ForbiddenError"):
            cls = next(s for s in symbols if s.name == cls_name)
            assert cls.type == SymbolType.CLASS

    def test_handler_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "handle_delete" in names
        assert "handle_list" in names
        assert "handle_update" in names

    def test_handler_error_classes(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/handler.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "ItemNotFoundError" in names
        assert "ValidationError" in names

    def test_service_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/service.py")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "list_items" in names
        assert "update_item" in names
        assert "search" in names

    def test_extract_imports_service(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/service.py")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "models" in targets
        assert "repository" in targets

    def test_extract_calls_service(self, analyser):
        tree, source, fp = _parse(analyser, "python_simple/service.py")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "find_by_id" in callee_names or "save" in callee_names


# ===========================================================================
# Java
# ===========================================================================


class TestJavaAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.java import JavaAnalyser
        return JavaAnalyser()

    def test_extract_symbols_controller(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserController" in names
        assert "getUser" in names
        assert "createUser" in names
        assert "logAction" in names

    def test_class_visibility(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        cls = next(s for s in symbols if s.name == "UserController")
        assert cls.type == SymbolType.CLASS
        assert cls.visibility == Visibility.PUBLIC

    def test_private_method_visibility(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        log = next(s for s in symbols if s.name == "logAction")
        assert log.visibility == Visibility.PRIVATE

    def test_constructor_detected(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        constructors = _symbol_names(symbols, SymbolType.CONSTRUCTOR)
        assert "UserController" in constructors

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserService.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserService" in names
        assert "findById" in names
        assert "create" in names
        assert "generateId" in names
        assert "UserRepository" in names

    def test_interface_detection(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserService.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        interfaces = _symbol_names(symbols, SymbolType.INTERFACE)
        assert "UserRepository" in interfaces

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "com.example.services.UserService" in targets

    def test_extract_calls_controller(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "UserService" in callee_names  # new UserService()
        assert "findById" in callee_names
        assert "create" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "toString" in excl
        assert "equals" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_model(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/User.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "User" in names
        assert "getId" in names
        assert "setId" in names
        assert "getName" in names
        assert "getEmail" in names
        assert "isActive" in names
        assert "setActive" in names

    def test_model_class_visibility(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/User.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        user = next(s for s in symbols if s.name == "User" and s.type == SymbolType.CLASS)
        assert user.visibility == Visibility.PUBLIC

    def test_model_constructor(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/User.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        ctors = _symbol_names(symbols, SymbolType.CONSTRUCTOR)
        assert "User" in ctors

    def test_extract_symbols_mapper(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserMapper.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserMapper" in names
        assert "toDto" in names
        assert "formatDisplayName" in names

    def test_mapper_private_method(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserMapper.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        fmt = next(s for s in symbols if s.name == "formatDisplayName")
        assert fmt.visibility == Visibility.PRIVATE

    def test_extract_symbols_repository_interface(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserRepository.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "UserRepository" in names
        interfaces = _symbol_names(symbols, SymbolType.INTERFACE)
        assert "UserRepository" in interfaces

    def test_repository_interface_methods(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserRepository.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "findById" in methods
        assert "findAll" in methods
        assert "save" in methods
        assert "delete" in methods
        assert "count" in methods

    def test_extract_imports_mapper(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserMapper.java")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "com.example.models.User" in targets
        assert "com.example.models.UserDto" in targets

    def test_controller_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserController.java")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "deleteUser" in methods
        assert "listUsers" in methods
        assert "handleError" in methods

    def test_extract_calls_mapper(self, analyser):
        tree, source, fp = _parse(analyser, "java_simple/UserMapper.java")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "UserDto" in callee_names or "formatDisplayName" in callee_names


# ===========================================================================
# Go
# ===========================================================================


class TestGoAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.go import GoAnalyser
        return GoAnalyser()

    def test_extract_symbols_handler(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Handler" in names
        assert "NewHandler" in names
        assert "HandleGet" in names
        assert "HandleCreate" in names
        assert "main" in names

    def test_struct_type(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        handler = next(s for s in symbols if s.name == "Handler")
        assert handler.type == SymbolType.STRUCT

    def test_exported_function(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        func = next(s for s in symbols if s.name == "NewHandler")
        assert func.exported is True
        assert func.visibility == Visibility.PUBLIC

    def test_unexported_function(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        func = next(s for s in symbols if s.name == "main")
        assert func.exported is False
        assert func.visibility == Visibility.PRIVATE

    def test_method_detection(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "HandleGet" in methods
        assert "HandleCreate" in methods

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/service.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "DataService" in names
        assert "NewDataService" in names
        assert "GetItem" in names
        assert "CreateItem" in names

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "fmt" in targets
        assert "myapp/service" in targets

    def test_extract_calls_handler(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "NewHandler" in callee_names
        assert "NewDataService" in callee_names or "GetItem" in callee_names
        assert "HandleGet" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "append" in excl
        assert "make" in excl
        assert "fmt.Println" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_model(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/model.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Item" in names
        assert "ItemFilter" in names
        assert "PaginatedResult" in names
        assert "NewItem" in names

    def test_model_structs(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/model.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        structs = _symbol_names(symbols, SymbolType.STRUCT)
        assert "Item" in structs
        assert "ItemFilter" in structs
        assert "PaginatedResult" in structs

    def test_model_exported_function(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/model.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        new_item = next(s for s in symbols if s.name == "NewItem")
        assert new_item.exported is True
        assert new_item.type == SymbolType.FUNCTION

    def test_extract_symbols_repository(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/repository.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Repository" in names  # interface
        assert "InMemoryRepository" in names  # struct
        assert "NewInMemoryRepository" in names  # constructor func

    def test_repository_interface(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/repository.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        repo = next(s for s in symbols if s.name == "Repository")
        assert repo.type == SymbolType.INTERFACE

    def test_repository_methods(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/repository.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "FindById" in methods
        assert "FindAll" in methods
        assert "Save" in methods
        assert "Delete" in methods
        assert "Count" in methods

    def test_extract_symbols_middleware(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/middleware.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Logger" in names
        assert "RequestTimer" in names
        assert "NewLogger" in names
        assert "Info" in names
        assert "Warn" in names
        assert "Error" in names

    def test_handler_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/handler.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        methods = _symbol_names(symbols, SymbolType.METHOD)
        assert "HandleDelete" in methods
        assert "HandleList" in methods

    def test_service_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "go_simple/service.go")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "DeleteItem" in names
        assert "ListItems" in names
        assert "UpdateItem" in names


# ===========================================================================
# Rust
# ===========================================================================


class TestRustAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.rust import RustAnalyser
        return RustAnalyser()

    def test_extract_symbols_main(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Handler" in names
        assert "main" in names

    def test_pub_struct(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        handler = next(s for s in symbols if s.name == "Handler")
        assert handler.type == SymbolType.STRUCT
        assert handler.exported is True
        assert handler.visibility == Visibility.PUBLIC

    def test_private_function(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        main_fn = next(s for s in symbols if s.name == "main")
        assert main_fn.exported is False
        assert main_fn.visibility == Visibility.PRIVATE

    def test_impl_block(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        impl_syms = [s for s in symbols if s.type == SymbolType.IMPL]
        assert len(impl_syms) >= 1

    def test_impl_methods(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        fn_names = _symbol_names(symbols, SymbolType.FUNCTION)
        # new, handle_get, handle_create should be inside impl Handler
        assert "new" in fn_names or "handle_get" in fn_names

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/service.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "DataService" in names
        assert "Repository" in names

    def test_trait_detection(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/service.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        traits = _symbol_names(symbols, SymbolType.TRAIT)
        assert "Repository" in traits

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "service::DataService" in targets

    def test_extract_calls_main(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        # Should detect Handler::new(), handler.handle_create(), DataService::new(), etc.
        assert "new" in callee_names or "handle_create" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "println" in excl
        assert "String::from" in excl
        assert "Some" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_model(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/model.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Item" in names
        assert "ItemFilter" in names

    def test_model_structs(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/model.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        structs = _symbol_names(symbols, SymbolType.STRUCT)
        assert "Item" in structs
        assert "ItemFilter" in structs

    def test_model_impl_methods(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/model.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        fn_names = _symbol_names(symbols, SymbolType.FUNCTION)
        assert "new" in fn_names
        assert "with_category" in fn_names
        assert "deactivate" in fn_names

    def test_extract_symbols_error(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/error.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "AppError" in names

    def test_error_is_enum(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/error.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        err = next(s for s in symbols if s.name == "AppError")
        assert err.type == SymbolType.ENUM

    def test_error_impl_methods(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/error.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        fn_names = _symbol_names(symbols, SymbolType.FUNCTION)
        assert "not_found" in fn_names
        assert "validation" in fn_names
        assert "internal" in fn_names

    def test_extract_symbols_repository(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/repository.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "InMemoryRepository" in names
        assert "find_by_id" in names or "save" in names

    def test_repository_struct(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/repository.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        repo = next(s for s in symbols if s.name == "InMemoryRepository")
        assert repo.type == SymbolType.STRUCT

    def test_service_expanded_methods(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/service.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        fn_names = _symbol_names(symbols, SymbolType.FUNCTION)
        assert "delete_item" in fn_names
        assert "list_items" in fn_names
        assert "update_item" in fn_names
        assert "count" in fn_names

    def test_main_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "rust_simple/main.rs")
        symbols = analyser.extract_symbols(tree, source, fp)
        fn_names = _symbol_names(symbols, SymbolType.FUNCTION)
        assert "handle_delete" in fn_names or "handle_list" in fn_names


# ===========================================================================
# C
# ===========================================================================


class TestCAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.c_cpp import CAnalyser
        return CAnalyser()

    def test_extract_symbols_main(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Handler" in names  # struct Handler
        assert "handle_request" in names
        assert "handle_create" in names
        assert "main" in names

    def test_struct_detection(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        handler = next(s for s in symbols if s.name == "Handler")
        assert handler.type == SymbolType.STRUCT

    def test_extract_symbols_header(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/service.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "get_item" in names
        assert "create_item" in names
        assert "delete_item" in names
        assert "Item" in names  # typedef struct
        assert "ItemStatus" in names  # enum

    def test_typedef_detection(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/service.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        item = next(s for s in symbols if s.name == "Item")
        assert item.type == SymbolType.TYPEDEF

    def test_enum_detection(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/service.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        enum_sym = next(s for s in symbols if s.name == "ItemStatus")
        assert enum_sym.type == SymbolType.ENUM

    def test_extract_symbols_impl(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/service.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "get_item" in names
        assert "create_item" in names
        assert "delete_item" in names

    def test_extract_includes(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "service.h" in targets
        assert "stdio.h" in targets

    def test_extract_calls(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "get_item" in callee_names
        assert "create_item" in callee_names
        assert "handle_request" in callee_names
        assert "handle_create" in callee_names
        # printf should be excluded
        assert "printf" not in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "printf" in excl
        assert "malloc" in excl
        assert "free" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_types_header(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/types.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Config" in names  # typedef struct
        assert "LogLevel" in names  # enum
        assert "default_config" in names
        assert "log_message" in names

    def test_types_enum(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/types.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        log_level = next(s for s in symbols if s.name == "LogLevel")
        assert log_level.type == SymbolType.ENUM

    def test_types_typedef(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/types.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        config = next(s for s in symbols if s.name == "Config")
        assert config.type == SymbolType.TYPEDEF

    def test_extract_symbols_repository_header(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/repository.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "Repository" in names  # typedef struct
        assert "repo_create" in names
        assert "repo_destroy" in names
        assert "repo_add" in names
        assert "repo_find" in names
        assert "repo_remove" in names
        assert "repo_count" in names

    def test_extract_symbols_repository_impl(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/repository.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "repo_create" in names
        assert "repo_destroy" in names
        assert "repo_add" in names
        assert "repo_find" in names
        assert "repo_remove" in names
        assert "repo_count" in names

    def test_extract_symbols_types_impl(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/types.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "default_config" in names
        assert "log_message" in names

    def test_header_expanded_functions(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/service.h")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "update_item" in names
        assert "list_items" in names
        assert "item_count" in names

    def test_main_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "handle_delete" in names
        assert "handle_list" in names

    def test_extract_includes_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "repository.h" in targets
        assert "types.h" in targets

    def test_extract_calls_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "c_simple/main.c")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "handle_delete" in callee_names or "handle_list" in callee_names
        assert "log_message" in callee_names or "default_config" in callee_names


# ===========================================================================
# C++
# ===========================================================================


class TestCppAnalyser:
    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.c_cpp import CppAnalyser
        return CppAnalyser()

    def test_extract_symbols_handler(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "app" in names  # namespace
        assert "Handler" in names  # class
        assert "main" in names

    def test_namespace_detection(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        ns = next(s for s in symbols if s.name == "app")
        assert ns.type == SymbolType.NAMESPACE

    def test_class_detection(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        cls = next(s for s in symbols if s.name == "Handler")
        assert cls.type == SymbolType.CLASS

    def test_namespace_child(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        handler = next(s for s in symbols if s.name == "Handler")
        assert handler.parent == "app"

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/service.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "DataService" in names  # class
        assert "ItemRecord" in names  # struct
        assert "Status" in names  # enum

    def test_class_and_struct_types(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/service.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        ds = next(s for s in symbols if s.name == "DataService")
        ir = next(s for s in symbols if s.name == "ItemRecord")
        assert ds.type == SymbolType.CLASS
        assert ir.type == SymbolType.STRUCT

    def test_enum_class_detection(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/service.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        status = next(s for s in symbols if s.name == "Status")
        assert status.type == SymbolType.ENUM

    def test_extract_includes(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "service.hpp" in targets
        assert "iostream" in targets

    def test_extract_calls_handler(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        assert "getItem" in callee_names or "createItem" in callee_names
        assert "handleCreate" in callee_names or "handleGet" in callee_names

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "printf" in excl
        assert "std::move" in excl
        assert "std::make_shared" in excl

    # --- New fixture file tests ---

    def test_extract_symbols_repository_header(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/repository.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "ItemRepository" in names

    def test_repository_class_type(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/repository.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        repo = next(s for s in symbols if s.name == "ItemRepository")
        assert repo.type == SymbolType.CLASS

    def test_extract_symbols_models(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/models.hpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        structs = _symbol_names(symbols, SymbolType.STRUCT)
        assert "AppConfig" in structs
        assert "ErrorResponse" in structs
        assert "PaginatedRequest" in structs

    def test_extract_symbols_main(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/main.cpp")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "printUsage" in names
        assert "runApp" in names

    def test_extract_includes_handler_expanded(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/handler.cpp")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "repository.hpp" in targets
        assert "models.hpp" in targets

    def test_extract_includes_main(self, analyser):
        tree, source, fp = _parse(analyser, "cpp_simple/main.cpp")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "service.hpp" in targets
        assert "repository.hpp" in targets
        assert "models.hpp" in targets


# ===========================================================================
# VB.NET
# ===========================================================================


class TestVBNetAnalyser:
    """VB.NET tests - skipped if grammar not available."""

    @pytest.fixture()
    def analyser(self):
        from mycelium.languages.vbnet import VBNetAnalyser
        a = VBNetAnalyser()
        if not a.is_available():
            pytest.skip("VB.NET grammar not available")
        return a

    def test_extract_symbols_service(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeService.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "EmployeeService" in names
        assert "GetEmployee" in names

    def test_class_type(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeService.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        svc = next(s for s in symbols if s.name == "EmployeeService")
        assert svc.type == SymbolType.CLASS

    def test_extract_symbols_module(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeModule.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "EmployeeModule" in names
        assert "LoadEmployee" in names

    def test_module_type(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeModule.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        mod = next(s for s in symbols if s.name == "EmployeeModule")
        assert mod.type == SymbolType.MODULE

    def test_extract_symbols_types(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeTypes.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "EmployeeStatus" in names
        assert "EmployeeRecord" in names

    def test_enum_type(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeTypes.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        enum_sym = next(s for s in symbols if s.name == "EmployeeStatus")
        assert enum_sym.type == SymbolType.ENUM

    def test_struct_type(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeTypes.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        struct_sym = next(s for s in symbols if s.name == "EmployeeRecord")
        assert struct_sym.type == SymbolType.STRUCT

    def test_extract_symbols_repository(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeRepository.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "EmployeeRepository" in names
        assert "IEmployeeRepository" in names

    def test_interface_type(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeRepository.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        iface = next(s for s in symbols if s.name == "IEmployeeRepository")
        assert iface.type == SymbolType.INTERFACE

    def test_extract_symbols_utils(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeUtils.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        names = _symbol_names(symbols)
        assert "EmployeeUtils" in names
        assert "FormatEmployeeName" in names
        assert "CalculateAge" in names

    def test_extract_imports(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeService.vb")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "System" in targets

    def test_extract_imports_repository(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeRepository.vb")
        imports = analyser.extract_imports(tree, source, fp)
        targets = _import_targets(imports)
        assert "System" in targets

    def test_extract_calls_service(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeService.vb")
        calls = analyser.extract_calls(tree, source, fp)
        callee_names = _call_names(calls)
        # Should detect _repository.FindById and EmployeeRepository constructor
        assert len(callee_names) > 0

    def test_builtin_exclusions(self, analyser):
        excl = analyser.builtin_exclusions()
        assert "MsgBox" in excl
        assert "CStr" in excl
        assert "DirectCast" in excl
        assert "Console.WriteLine" in excl
        assert "ToString" in excl

    def test_namespace_extraction(self, analyser):
        tree, source, fp = _parse(analyser, "vbnet_simple/EmployeeService.vb")
        symbols = analyser.extract_symbols(tree, source, fp)
        ns_syms = [s for s in symbols if s.type == SymbolType.NAMESPACE]
        ns_names = {s.name for s in ns_syms}
        assert "Acme.Employee" in ns_names


# ===========================================================================
# VB.NET E2E
# ===========================================================================


class TestVBNetE2E:
    def test_vbnet_e2e(self):
        """Full pipeline on VB.NET fixtures."""
        from mycelium.languages.vbnet import VBNetAnalyser
        if not VBNetAnalyser().is_available():
            pytest.skip("VB.NET grammar not available")

        from mycelium.pipeline import run_pipeline
        from mycelium.config import AnalysisConfig
        cfg = AnalysisConfig(repo_path=str(FIXTURES / "vbnet_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "EmployeeService" in sym_names
        assert "EmployeeModule" in sym_names


# ===========================================================================
# Language Registry
# ===========================================================================


class TestLanguageRegistry:
    def test_all_extensions_registered(self):
        from mycelium.languages import supported_extensions
        exts = supported_extensions()
        expected = {
            ".cs", ".vb",  # .NET
            ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs",  # TypeScript/JS
            ".py",  # Python
            ".java",  # Java
            ".go",  # Go
            ".rs",  # Rust
            ".c", ".h",  # C
            ".cpp", ".cc", ".cxx", ".hpp", ".hxx", ".hh",  # C++
        }
        for ext in expected:
            assert ext in exts, f"Extension {ext} not registered"

    def test_get_analyser_returns_correct_type(self):
        from mycelium.languages import get_analyser
        from mycelium.languages.typescript import TypeScriptAnalyser
        from mycelium.languages.python_lang import PythonAnalyser
        from mycelium.languages.java import JavaAnalyser
        from mycelium.languages.go import GoAnalyser
        from mycelium.languages.rust import RustAnalyser
        from mycelium.languages.c_cpp import CAnalyser, CppAnalyser

        assert isinstance(get_analyser(".ts"), TypeScriptAnalyser)
        assert isinstance(get_analyser(".py"), PythonAnalyser)
        assert isinstance(get_analyser(".java"), JavaAnalyser)
        assert isinstance(get_analyser(".go"), GoAnalyser)
        assert isinstance(get_analyser(".rs"), RustAnalyser)
        assert isinstance(get_analyser(".c"), CAnalyser)
        assert isinstance(get_analyser(".cpp"), CppAnalyser)

    def test_get_language_name(self):
        from mycelium.languages import get_language
        assert get_language(".ts") == "ts"
        assert get_language(".py") == "py"
        assert get_language(".java") == "java"
        assert get_language(".go") == "go"
        assert get_language(".rs") == "rust"
        assert get_language(".c") == "c"
        assert get_language(".cpp") == "cpp"

    def test_unknown_extension_returns_none(self):
        from mycelium.languages import get_analyser, get_language
        assert get_analyser(".xyz") is None
        assert get_language(".xyz") is None


# ===========================================================================
# E2E: Parse each language fixture directory
# ===========================================================================


class TestE2ELanguageParsing:
    """End-to-end tests running the full pipeline on each language fixture."""

    @pytest.fixture()
    def config(self):
        from mycelium.config import AnalysisConfig
        return AnalysisConfig

    def test_typescript_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "typescript_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        assert len(result.calls) >= 0
        sym_names = {s["name"] for s in result.symbols}
        assert "UserController" in sym_names
        assert "UserService" in sym_names
        assert "UserRepository" in sym_names
        assert "AuthMiddleware" in sym_names
        assert "hashPassword" in sym_names

    def test_python_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "python_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "RequestHandler" in sym_names
        assert "DataService" in sym_names
        assert "ItemRepository" in sym_names
        assert "AppError" in sym_names
        assert "ItemCategory" in sym_names

    def test_java_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "java_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "UserController" in sym_names
        assert "UserService" in sym_names
        assert "User" in sym_names
        assert "UserMapper" in sym_names
        # Separate file interface
        assert "UserRepository" in sym_names

    def test_go_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "go_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "Handler" in sym_names
        assert "DataService" in sym_names
        assert "Item" in sym_names
        assert "Repository" in sym_names
        assert "Logger" in sym_names

    def test_rust_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "rust_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "Handler" in sym_names
        assert "DataService" in sym_names
        assert "Item" in sym_names
        assert "AppError" in sym_names
        assert "InMemoryRepository" in sym_names

    def test_c_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "c_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "handle_request" in sym_names
        assert "get_item" in sym_names
        assert "Config" in sym_names
        assert "Repository" in sym_names
        assert "repo_create" in sym_names

    def test_cpp_e2e(self, config):
        from mycelium.pipeline import run_pipeline
        cfg = config(repo_path=str(FIXTURES / "cpp_simple"), quiet=True)
        result = run_pipeline(cfg)
        assert len(result.symbols) > 0
        sym_names = {s["name"] for s in result.symbols}
        assert "DataService" in sym_names
        assert "ItemRepository" in sym_names
        assert "AppConfig" in sym_names
        assert "printUsage" in sym_names
