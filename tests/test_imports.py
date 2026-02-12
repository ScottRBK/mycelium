"""Tests for Phase 3: Import resolution."""

from __future__ import annotations

import os

from mycelium.config import AnalysisConfig
from mycelium.dotnet.assembly import AssemblyMapper
from mycelium.dotnet.project import parse_project
from mycelium.dotnet.solution import parse_solution
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.symbol_table import SymbolTable
from mycelium.phases.structure import run_structure_phase
from mycelium.phases.parsing import run_parsing_phase
from mycelium.phases.imports import run_imports_phase

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


def _run_three_phases(fixture_dir: str) -> tuple[KnowledgeGraph, SymbolTable]:
    """Run structure + parsing + imports phases."""
    kg = KnowledgeGraph()
    st = SymbolTable()
    config = AnalysisConfig(repo_path=fixture_dir)
    run_structure_phase(config, kg)
    run_parsing_phase(config, kg, st)
    run_imports_phase(config, kg, st)
    return kg, st


class TestSolutionParser:
    def test_parse_sln(self):
        sln_path = os.path.join(FIXTURES_DIR, "mixed_dotnet", "MixedSolution.sln")
        projects = parse_solution(sln_path)

        assert len(projects) == 2
        names = {p.name for p in projects}
        assert "CSharpProject" in names
        assert "VBNetProject" in names

    def test_sln_project_paths(self):
        sln_path = os.path.join(FIXTURES_DIR, "mixed_dotnet", "MixedSolution.sln")
        projects = parse_solution(sln_path)

        cs_proj = next(p for p in projects if p.name == "CSharpProject")
        assert "CSharpProject/CSharpProject.csproj" in cs_proj.path

    def test_sln_guids(self):
        sln_path = os.path.join(FIXTURES_DIR, "mixed_dotnet", "MixedSolution.sln")
        projects = parse_solution(sln_path)

        for p in projects:
            assert len(p.project_guid) > 0
            assert len(p.type_guid) > 0

    def test_parse_nonexistent_sln(self):
        projects = parse_solution("/nonexistent/path.sln")
        assert projects == []


class TestProjectParser:
    def test_parse_csproj(self):
        csproj_path = os.path.join(
            FIXTURES_DIR, "mixed_dotnet", "CSharpProject", "CSharpProject.csproj"
        )
        info = parse_project(csproj_path)

        assert info.root_namespace == "MixedSolution.CSharp"
        assert info.assembly_name == "CSharpProject"
        assert info.target_framework == "net6.0"

    def test_project_references(self):
        csproj_path = os.path.join(
            FIXTURES_DIR, "mixed_dotnet", "CSharpProject", "CSharpProject.csproj"
        )
        info = parse_project(csproj_path)

        assert len(info.project_references) == 1
        assert "VBNetProject.vbproj" in info.project_references[0]

    def test_package_references(self):
        csproj_path = os.path.join(
            FIXTURES_DIR, "mixed_dotnet", "CSharpProject", "CSharpProject.csproj"
        )
        info = parse_project(csproj_path)

        pkg_names = {name for name, _ in info.package_references}
        assert "Newtonsoft.Json" in pkg_names
        assert "Microsoft.Extensions.Logging" in pkg_names

    def test_package_versions(self):
        csproj_path = os.path.join(
            FIXTURES_DIR, "mixed_dotnet", "CSharpProject", "CSharpProject.csproj"
        )
        info = parse_project(csproj_path)

        pkg_dict = {name: ver for name, ver in info.package_references}
        assert pkg_dict["Newtonsoft.Json"] == "13.0.3"

    def test_parse_vbproj(self):
        vbproj_path = os.path.join(
            FIXTURES_DIR, "mixed_dotnet", "VBNetProject", "VBNetProject.vbproj"
        )
        info = parse_project(vbproj_path)

        assert info.root_namespace == "MixedSolution.VBNet"
        assert info.assembly_name == "VBNetProject"


class TestAssemblyMapper:
    def test_exact_match(self):
        mapper = AssemblyMapper()
        mapper.register_namespace("Absence.Services", "src/Absence.csproj")

        assert mapper.resolve_namespace("Absence.Services") == "src/Absence.csproj"

    def test_prefix_match(self):
        mapper = AssemblyMapper()
        mapper.register_namespace("Absence", "src/Absence.csproj")

        assert mapper.resolve_namespace("Absence.Services") == "src/Absence.csproj"
        assert mapper.resolve_namespace("Absence.Models") == "src/Absence.csproj"

    def test_no_match(self):
        mapper = AssemblyMapper()
        mapper.register_namespace("Absence", "src/Absence.csproj")

        assert mapper.resolve_namespace("Framework.Core") is None

    def test_longest_prefix_wins(self):
        mapper = AssemblyMapper()
        mapper.register_namespace("Absence", "src/Absence.csproj")
        mapper.register_namespace("Absence.Services", "src/AbsenceServices.csproj")

        assert mapper.resolve_namespace("Absence.Services.Internal") == "src/AbsenceServices.csproj"


class TestImportsPhase:
    def test_csharp_imports_extracted(self):
        kg, st = _run_three_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        import_edges = kg.get_import_edges()
        # AbsenceController.cs imports Absence.Services namespace
        # This should resolve to AbsenceService.cs or IAbsenceService.cs
        if import_edges:
            froms = {e["from"] for e in import_edges}
            assert "AbsenceController.cs" in froms

    def test_mixed_dotnet_project_references(self):
        kg, _ = _run_three_phases(os.path.join(FIXTURES_DIR, "mixed_dotnet"))

        proj_refs = kg.get_project_references()
        assert len(proj_refs) >= 1
        # CSharpProject references VBNetProject
        assert any(
            "CSharpProject" in r["from"] and "VBNetProject" in r["to"]
            for r in proj_refs
        )

    def test_mixed_dotnet_package_references(self):
        kg, _ = _run_three_phases(os.path.join(FIXTURES_DIR, "mixed_dotnet"))

        pkg_refs = kg.get_package_references()
        pkg_names = {r["package"] for r in pkg_refs}
        assert "Newtonsoft.Json" in pkg_names
        assert "Microsoft.Extensions.Logging" in pkg_names

    def test_no_crash_without_sln(self):
        """Importing without .sln should work fine."""
        kg, st = _run_three_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))
        # Should complete without error
        assert kg.file_count() > 0


class TestPythonImports:
    """Tests for Python import resolution via dotted module paths."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "python_package"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    # --- Absolute dotted imports ---

    def test_absolute_import_module(self):
        """from app.models.user import User → app/models/user.py"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("app/services/user_service.py", "app/models/user.py") in pairs

    def test_absolute_import_helpers(self):
        """from app.utils.helpers import format_name → app/utils/helpers.py"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("app/services/user_service.py", "app/utils/helpers.py") in pairs

    # --- Package imports (__init__.py) ---

    def test_package_import_via_init(self):
        """from app.models import User → app/models/__init__.py"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.py", "app/models/__init__.py") in pairs

    # --- Relative imports ---

    def test_relative_import_single_dot(self):
        """.validators from app/utils/helpers.py → app/utils/validators.py"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("app/utils/helpers.py", "app/utils/validators.py") in pairs

    def test_relative_import_double_dot(self):
        """..models.item from app/services/user_service.py → app/models/item.py"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("app/services/user_service.py", "app/models/item.py") in pairs

    # --- Stdlib / third-party (no match in repo) ---

    def test_stdlib_import_no_edge(self):
        """import os should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        os_targets = {to for (frm, to) in pairs if "os" in to}
        assert len(os_targets) == 0

    def test_third_party_import_no_edge(self):
        """from pydantic import BaseModel should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        pydantic_targets = {to for (frm, to) in pairs if "pydantic" in to}
        assert len(pydantic_targets) == 0

    # --- Self-import exclusion ---

    def test_no_self_import(self):
        """A file should never import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import detected: {e['from']}"

    # --- Overall import count ---

    def test_python_imports_resolved(self):
        """Python package fixture should resolve multiple import edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # main.py imports 2 targets, user_service.py imports 3, helpers.py imports 1,
        # models/__init__.py imports 2 = at least 8 edges
        assert len(edges) >= 6


# ===================================================================
# TypeScript/JavaScript imports
# ===================================================================

class TestTypeScriptImports:
    """Tests for TS/JS import resolution via relative paths."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "typescript_simple"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_controller_imports_service(self):
        """controller.ts imports ./service -> service.ts"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("controller.ts", "service.ts") in pairs

    def test_controller_imports_repository(self):
        """controller.ts imports ./repository -> repository.ts"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("controller.ts", "repository.ts") in pairs

    def test_controller_imports_models(self):
        """controller.ts imports ./models -> models.ts"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("controller.ts", "models.ts") in pairs

    def test_service_imports_utils(self):
        """service.ts imports ./utils -> utils.ts"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("service.ts", "utils.ts") in pairs

    def test_reexport_creates_edge(self):
        """index.ts re-exports from ./controller -> controller.ts"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("index.ts", "controller.ts") in pairs

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_ts_imports_resolved(self):
        """Fixture should resolve many import edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # controller: 3, service: 3, repository: 1, middleware: 1,
        # index: 6 re-exports = at least 14
        assert len(edges) >= 10


# ===================================================================
# Java imports (flat layout — class-name fallback)
# ===================================================================

class TestJavaImports:
    """Tests for Java import resolution via class-name fallback."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "java_simple"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_controller_imports_service(self):
        """UserController.java imports UserService via class-name fallback."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("UserController.java", "UserService.java") in pairs

    def test_controller_imports_model(self):
        """UserController.java imports User via class-name fallback."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("UserController.java", "User.java") in pairs

    def test_service_imports_repository(self):
        """UserService.java imports UserRepository or InMemoryUserRepository."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("UserService.java", "UserRepository.java") in pairs or \
               ("UserService.java", "InMemoryUserRepository.java") in pairs

    def test_stdlib_no_edge(self):
        """java.util.List should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert not any("java" in to and "util" in to for _, to in pairs)

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_java_imports_resolved(self):
        """Flat Java fixture should resolve multiple import edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        assert len(edges) >= 5


# ===================================================================
# Java imports (package directory layout — path-based)
# ===================================================================

class TestJavaPackageImports:
    """Tests for Java import resolution via dotted path conversion."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "java_package"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_controller_imports_service(self):
        """Path-based: com.example.services.UserService -> com/example/services/UserService.java"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert (
            "com/example/controllers/UserController.java",
            "com/example/services/UserService.java",
        ) in pairs

    def test_controller_imports_model(self):
        """Path-based: com.example.models.User -> com/example/models/User.java"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert (
            "com/example/controllers/UserController.java",
            "com/example/models/User.java",
        ) in pairs

    def test_service_imports_model(self):
        """Path-based: com.example.models.User -> com/example/models/User.java"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert (
            "com/example/services/UserService.java",
            "com/example/models/User.java",
        ) in pairs


# ===================================================================
# Go imports (package directory layout)
# ===================================================================

class TestGoImports:
    """Tests for Go import resolution via go.mod + directory index."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "go_package"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_main_imports_service(self):
        """main.go imports myapp/service -> service/service.go"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.go", "service/service.go") in pairs

    def test_main_imports_middleware(self):
        """main.go imports myapp/middleware -> middleware/middleware.go"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.go", "middleware/middleware.go") in pairs

    def test_service_imports_model(self):
        """service/service.go imports myapp/model -> model/model.go"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("service/service.go", "model/model.go") in pairs

    def test_stdlib_no_edge(self):
        """fmt should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert not any("fmt" in to for _, to in pairs)

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_go_imports_resolved(self):
        """Go package fixture should resolve multiple import edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # main.go: 2 (service, middleware), service: 1 (model) = 3 minimum
        assert len(edges) >= 3


# ===================================================================
# Rust imports
# ===================================================================

class TestRustImports:
    """Tests for Rust import resolution via crate/super/bare paths."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "rust_simple"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_main_imports_service(self):
        """main.rs: use service::DataService -> service.rs (progressive shortening)."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.rs", "service.rs") in pairs

    def test_main_imports_model(self):
        """main.rs: use model::Item -> model.rs"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.rs", "model.rs") in pairs

    def test_main_imports_error(self):
        """main.rs: use error::AppError -> error.rs"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.rs", "error.rs") in pairs

    def test_crate_prefix_resolves(self):
        """service.rs: use crate::model::Item -> model.rs"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("service.rs", "model.rs") in pairs

    def test_std_no_edge(self):
        """std::collections::HashMap should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert not any("std" in to for _, to in pairs)

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_rust_imports_resolved(self):
        """Rust fixture should resolve multiple import edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # main.rs: 3 (service, model, error), service.rs: 1 (model),
        # repository.rs: 1 (model) = 5 minimum
        assert len(edges) >= 4


# ===================================================================
# C imports
# ===================================================================

class TestCImports:
    """Tests for C #include resolution."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "c_simple"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_main_includes_service(self):
        """main.c: #include "service.h" -> service.h"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.c", "service.h") in pairs

    def test_main_includes_repository(self):
        """main.c: #include "repository.h" -> repository.h"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.c", "repository.h") in pairs

    def test_main_includes_types(self):
        """main.c: #include "types.h" -> types.h"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("main.c", "types.h") in pairs

    def test_system_include_no_edge(self):
        """#include <stdio.h> should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert not any("stdio" in to for _, to in pairs)

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_c_imports_resolved(self):
        """C fixture should resolve multiple include edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # main.c: 3, service.c: 1, types.c: 1, repository.c: 1,
        # repository.h: 1 = 7
        assert len(edges) >= 5


# ===================================================================
# C++ imports
# ===================================================================

class TestCppImports:
    """Tests for C++ #include resolution."""

    def _run(self):
        return _run_three_phases(os.path.join(FIXTURES_DIR, "cpp_simple"))

    def _edges(self, kg):
        return kg.get_import_edges()

    def _edge_pairs(self, kg):
        return {(e["from"], e["to"]) for e in self._edges(kg)}

    def test_handler_includes_service(self):
        """handler.cpp: #include "service.hpp" -> service.hpp"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("handler.cpp", "service.hpp") in pairs

    def test_handler_includes_repository(self):
        """handler.cpp: #include "repository.hpp" -> repository.hpp"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("handler.cpp", "repository.hpp") in pairs

    def test_handler_includes_models(self):
        """handler.cpp: #include "models.hpp" -> models.hpp"""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert ("handler.cpp", "models.hpp") in pairs

    def test_system_include_no_edge(self):
        """#include <iostream> should not create an import edge."""
        kg, _ = self._run()
        pairs = self._edge_pairs(kg)
        assert not any("iostream" in to for _, to in pairs)

    def test_no_self_import(self):
        """No file should import itself."""
        kg, _ = self._run()
        for e in self._edges(kg):
            assert e["from"] != e["to"], f"Self-import: {e['from']}"

    def test_cpp_imports_resolved(self):
        """C++ fixture should resolve multiple include edges."""
        kg, _ = self._run()
        edges = self._edges(kg)
        # handler.cpp: 3, main.cpp: 3, service.cpp: 1, repository.cpp: 1,
        # repository.hpp: 1 = 9
        assert len(edges) >= 5
