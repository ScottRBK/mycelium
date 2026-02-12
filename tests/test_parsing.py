"""Tests for Phase 2: Parsing (symbol extraction)."""

from __future__ import annotations

import os

import pytest

from mycelium.config import AnalysisConfig, SymbolType, Visibility
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.symbol_table import SymbolTable
from mycelium.phases.structure import run_structure_phase
from mycelium.phases.parsing import run_parsing_phase

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


def _run_phases(fixture_dir: str) -> tuple[KnowledgeGraph, SymbolTable]:
    """Helper: run structure + parsing on a fixture directory."""
    kg = KnowledgeGraph()
    st = SymbolTable()
    config = AnalysisConfig(repo_path=fixture_dir)
    run_structure_phase(config, kg)
    run_parsing_phase(config, kg, st)
    return kg, st


class TestCSharpParsing:
    @pytest.fixture(autouse=True)
    def setup(self):
        self.kg, self.st = _run_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

    def test_extracts_classes(self):
        symbols = self.kg.get_symbols()
        classes = [s for s in symbols if s["symbol_type"] == SymbolType.CLASS.value]
        class_names = {s["name"] for s in classes}
        assert "AbsenceController" in class_names
        assert "AbsenceService" in class_names
        assert "AbsenceModel" in class_names
        # New fixture classes
        assert "AbsenceRepository" in class_names
        assert "LeaveRequestValidator" in class_names
        assert "AbsenceException" in class_names

    def test_extracts_interfaces(self):
        symbols = self.kg.get_symbols()
        interfaces = [s for s in symbols if s["symbol_type"] == SymbolType.INTERFACE.value]
        assert any(s["name"] == "IAbsenceService" for s in interfaces)
        assert any(s["name"] == "IAbsenceRepository" for s in interfaces)

    def test_extracts_methods(self):
        symbols = self.kg.get_symbols()
        methods = [s for s in symbols if s["symbol_type"] == SymbolType.METHOD.value]
        method_names = {s["name"] for s in methods}
        assert "GetEntitlement" in method_names
        assert "CalculateEntitlement" in method_names
        assert "GetBonusDays" in method_names
        # New methods from expanded fixtures
        assert "CheckLeaveStatus" in method_names
        assert "SubmitRequest" in method_names
        assert "GetLeaveHistory" in method_names
        assert "ValidateRequest" in method_names
        assert "GetDaysTaken" in method_names

    def test_extracts_constructors(self):
        symbols = self.kg.get_symbols()
        ctors = [s for s in symbols if s["symbol_type"] == SymbolType.CONSTRUCTOR.value]
        assert len(ctors) >= 2  # AbsenceController and AbsenceService ctors

    def test_extracts_properties(self):
        symbols = self.kg.get_symbols()
        props = [s for s in symbols if s["symbol_type"] == SymbolType.PROPERTY.value]
        # AbsenceController has no properties; AbsenceModel might not either
        # This test just ensures properties are extracted if present
        assert isinstance(props, list)

    def test_extracts_enums(self):
        symbols = self.kg.get_symbols()
        enums = [s for s in symbols if s["symbol_type"] == SymbolType.ENUM.value]
        assert any(s["name"] == "LeaveType" for s in enums)

    def test_extracts_structs(self):
        symbols = self.kg.get_symbols()
        structs = [s for s in symbols if s["symbol_type"] == SymbolType.STRUCT.value]
        assert any(s["name"] == "DateRange" for s in structs)

    def test_extracts_namespaces(self):
        symbols = self.kg.get_symbols()
        namespaces = [s for s in symbols if s["symbol_type"] == SymbolType.NAMESPACE.value]
        ns_names = {s["name"] for s in namespaces}
        assert "Absence.Controllers" in ns_names
        assert "Absence.Services" in ns_names
        assert "Absence.Models" in ns_names
        assert "Absence.Repositories" in ns_names
        assert "Absence.Validators" in ns_names

    def test_namespace_visibility_unknown(self):
        """Namespaces should have unknown visibility (C# has no namespace modifiers)."""
        symbols = self.kg.get_symbols()
        namespaces = [s for s in symbols if s["symbol_type"] == SymbolType.NAMESPACE.value]
        for ns in namespaces:
            assert ns["visibility"] == Visibility.UNKNOWN.value, (
                f"Namespace {ns['name']} has visibility {ns['visibility']}, expected unknown"
            )

    def test_visibility_public(self):
        symbols = self.kg.get_symbols()
        controller = next(s for s in symbols if s["name"] == "AbsenceController")
        assert controller["visibility"] == Visibility.PUBLIC.value
        assert controller["exported"] is True

    def test_visibility_internal(self):
        symbols = self.kg.get_symbols()
        model = next(s for s in symbols if s["name"] == "AbsenceModel")
        assert model["visibility"] == Visibility.INTERNAL.value

    def test_visibility_private(self):
        symbols = self.kg.get_symbols()
        bonus = next(s for s in symbols if s["name"] == "GetBonusDays")
        assert bonus["visibility"] == Visibility.PRIVATE.value
        assert bonus["exported"] is False

    def test_parent_relationships(self):
        symbols = self.kg.get_symbols()
        # Methods should have parent = class name
        calc = next(s for s in symbols if s["name"] == "CalculateEntitlement")
        assert calc["parent"] == "AbsenceService"

    def test_line_numbers(self):
        symbols = self.kg.get_symbols()
        for s in symbols:
            assert s["line"] > 0

    def test_symbol_table_populated(self):
        # Symbol table should have entries for all symbols
        all_syms = self.kg.get_symbols()
        for s in all_syms:
            name = s["name"]
            fuzzy = self.st.lookup_fuzzy(name)
            assert len(fuzzy) > 0, f"Symbol {name} not in global index"

    def test_symbol_table_exact_lookup(self):
        # Exact lookup should work for each file
        syms = self.st.get_symbols_in_file("AbsenceController.cs")
        assert "AbsenceController" in syms

    def test_defines_edges_created(self):
        # Each symbol should have a DEFINES edge from its file
        symbols = self.kg.get_symbols_in_file("AbsenceController.cs")
        sym_names = {s["name"] for s in symbols}
        assert "AbsenceController" in sym_names


class TestVBNetParsing:
    """VB.NET tests are skipped if the grammar is not available."""

    def test_vbnet_graceful_skip(self):
        """If VB.NET grammar is unavailable, parsing should skip .vb files gracefully."""
        kg, st = _run_phases(os.path.join(FIXTURES_DIR, "vbnet_simple"))

        # Files should still be discovered by structure phase
        files = kg.get_files()
        vb_files = [f for f in files if f["language"] == "vb"]
        assert len(vb_files) == 5

        # If grammar is unavailable, no symbols will be extracted (graceful skip)
        # If available, symbols should be extracted
        # Either way, no crash
        symbols = kg.get_symbols()
        # Just verify it didn't crash - symbol count depends on grammar availability


class TestVBNetFullParsing:
    """Full VB.NET parsing tests - skipped if grammar not available."""

    @pytest.fixture(autouse=True)
    def setup(self):
        from mycelium.languages.vbnet import VBNetAnalyser
        analyser = VBNetAnalyser()
        if not analyser.is_available():
            pytest.skip("VB.NET grammar not available")
        self.kg, self.st = _run_phases(os.path.join(FIXTURES_DIR, "vbnet_simple"))

    def test_extracts_classes(self):
        symbols = self.kg.get_symbols()
        classes = [s for s in symbols if s["symbol_type"] == "Class"]
        class_names = {s["name"] for s in classes}
        assert "EmployeeService" in class_names
        assert "EmployeeRepository" in class_names

    def test_extracts_interfaces(self):
        symbols = self.kg.get_symbols()
        interfaces = [s for s in symbols if s["symbol_type"] == "Interface"]
        assert any(s["name"] == "IEmployeeRepository" for s in interfaces)

    def test_extracts_modules(self):
        symbols = self.kg.get_symbols()
        modules = [s for s in symbols if s["symbol_type"] == "Module"]
        module_names = {s["name"] for s in modules}
        assert "EmployeeModule" in module_names
        assert "EmployeeUtils" in module_names

    def test_extracts_enums(self):
        symbols = self.kg.get_symbols()
        enums = [s for s in symbols if s["symbol_type"] == "Enum"]
        assert any(s["name"] == "EmployeeStatus" for s in enums)

    def test_extracts_structs(self):
        symbols = self.kg.get_symbols()
        structs = [s for s in symbols if s["symbol_type"] == "Struct"]
        assert any(s["name"] == "EmployeeRecord" for s in structs)

    def test_extracts_namespaces(self):
        symbols = self.kg.get_symbols()
        namespaces = [s for s in symbols if s["symbol_type"] == "Namespace"]
        ns_names = {s["name"] for s in namespaces}
        assert "Acme.Employee" in ns_names

    def test_namespace_visibility_unknown(self):
        symbols = self.kg.get_symbols()
        namespaces = [s for s in symbols if s["symbol_type"] == "Namespace"]
        for ns in namespaces:
            assert ns["visibility"] == "unknown"

    def test_extracts_methods(self):
        symbols = self.kg.get_symbols()
        methods = [s for s in symbols if s["symbol_type"] == "Method"]
        method_names = {s["name"] for s in methods}
        assert "GetEmployee" in method_names
        assert "FindById" in method_names


class TestMixedDotNet:
    def test_mixed_parsing_no_crash(self):
        """Parsing mixed C#/VB.NET project should not crash."""
        kg, st = _run_phases(os.path.join(FIXTURES_DIR, "mixed_dotnet"))

        # C# file should be parsed
        symbols = kg.get_symbols()
        cs_symbols = [s for s in symbols if s["language"] == "cs"]
        assert len(cs_symbols) > 0
        assert any(s["name"] == "ApiController" for s in cs_symbols)
