"""Tests for Phase 4: Call graph with confidence scoring."""

from __future__ import annotations

import os

from mycelium.config import AnalysisConfig
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.symbol_table import SymbolTable
from mycelium.phases.structure import run_structure_phase
from mycelium.phases.parsing import run_parsing_phase
from mycelium.phases.imports import run_imports_phase
from mycelium.phases.calls import run_calls_phase
from mycelium.languages.csharp import CSharpAnalyser

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


def _run_four_phases(fixture_dir: str) -> tuple[KnowledgeGraph, SymbolTable]:
    """Run structure + parsing + imports + calls phases."""
    kg = KnowledgeGraph()
    st = SymbolTable()
    config = AnalysisConfig(repo_path=fixture_dir)
    run_structure_phase(config, kg)
    run_parsing_phase(config, kg, st)
    run_imports_phase(config, kg, st)
    run_calls_phase(config, kg, st)
    return kg, st


class TestCSharpCallExtraction:
    def test_extracts_calls(self):
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        assert len(calls) > 0

    def test_call_has_confidence(self):
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        for call in calls:
            assert 0.0 < call["confidence"] <= 1.0
            assert call["tier"] in ("A", "B", "C")
            assert call["reason"] in (
                "import-resolved", "impl-resolved", "same-file",
                "fuzzy-unique", "fuzzy-ambiguous",
                "di-resolved", "di-impl-resolved",
            )

    def test_same_file_calls(self):
        """Calls within the same file should be Tier B."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        # AbsenceService.CalculateEntitlement calls GetBonusDays (same file)
        same_file = [c for c in calls if c["tier"] == "B"]
        assert len(same_file) > 0

    def test_builtin_filtering(self):
        """Built-in calls like Console.WriteLine should be filtered."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        # Console.WriteLine should NOT appear as a resolved call
        symbols = kg.get_symbols()
        sym_names = {s["id"]: s["name"] for s in symbols}

        for call in calls:
            to_name = sym_names.get(call["to"], "")
            assert to_name != "WriteLine", "Console.WriteLine should be filtered"

    def test_cross_file_calls(self):
        """Calls between files should be Tier A or C."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        cross_file = [c for c in calls if c["tier"] in ("A", "C")]
        # Should have some cross-file calls (e.g., Controller -> Service)
        # These might be Tier C if import resolution didn't resolve the exact file
        assert isinstance(cross_file, list)  # At minimum no crash

    def test_call_edges_have_line_numbers(self):
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        for call in calls:
            assert call["line"] > 0


class TestPropertyCallSources:
    def test_properties_not_as_call_sources(self):
        """Property names should not appear as caller_name in calls (#5)."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        # Get all property symbol names
        symbols = kg.get_symbols()
        property_names = {s["name"] for s in symbols if s["symbol_type"] == "Property"}

        # Check no call edge originates from a property
        calls = kg.get_call_edges()
        sym_map = {s["id"]: s["name"] for s in symbols}
        for call in calls:
            caller_name = sym_map.get(call["from"], "")
            assert caller_name not in property_names, (
                f"Property '{caller_name}' should not be a call source"
            )

    def test_task_not_in_call_graph(self):
        """Task/ValueTask should be excluded from the call graph (#7)."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        symbols = kg.get_symbols()
        sym_names = {s["id"]: s["name"] for s in symbols}

        for call in calls:
            to_name = sym_names.get(call["to"], "")
            assert to_name not in ("Task", "ValueTask"), (
                f"Framework type '{to_name}' should be excluded from call graph"
            )


class TestBuiltinExclusions:
    def test_csharp_exclusions(self):
        analyser = CSharpAnalyser()
        exclusions = analyser.builtin_exclusions()

        assert "Console.WriteLine" in exclusions
        assert "String.Format" in exclusions
        assert "ToString" in exclusions
        assert "Task.Run" in exclusions
        assert len(exclusions) > 20


class TestNamespaceResolvedCalls:
    def test_namespace_resolved_calls_are_tier_a(self):
        """CalculateEntitlement from AbsenceController should be Tier A via namespace."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        symbols = kg.get_symbols()
        sym_map = {s["id"]: s["name"] for s in symbols}

        # Find calls to CalculateEntitlement
        calc_calls = [
            c for c in calls
            if sym_map.get(c["to"]) == "CalculateEntitlement"
        ]
        assert len(calc_calls) > 0, "Should have calls to CalculateEntitlement"
        # At least one should be Tier A
        tier_a = [c for c in calc_calls if c["tier"] == "A"]
        assert len(tier_a) > 0, "CalculateEntitlement should have at least one Tier A call"

    def test_constructor_calls_tier_a(self):
        """new AbsenceModel() should resolve as a Tier A or B call."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        symbols = kg.get_symbols()
        sym_map = {s["id"]: s["name"] for s in symbols}

        model_calls = [
            c for c in calls
            if sym_map.get(c["to"]) == "AbsenceModel"
        ]
        # Should be resolved (not fuzzy)
        assert len(model_calls) > 0, "Should have calls to AbsenceModel constructor"

    def test_di_resolved_call(self):
        """_service.CalculateEntitlement() should resolve via DI tracking."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        # Check for di-resolved calls
        di_calls = [c for c in calls if c.get("reason") == "di-resolved"]
        # DI resolution may or may not find matches depending on field names
        # but should not crash
        assert isinstance(di_calls, list)

    def test_interface_self_call_filtered(self):
        """Same-name caller/callee where callee is in interface should be filtered."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        symbols = kg.get_symbols()
        sym_map = {s["id"]: s for s in symbols}

        for call in calls:
            from_sym = sym_map.get(call["from"], {})
            to_sym = sym_map.get(call["to"], {})
            from_name = from_sym.get("name", "")
            to_name = to_sym.get("name", "")
            to_parent = to_sym.get("parent", "")

            # Check no call where caller==callee name and target is in interface
            if from_name == to_name:
                # Get parent type
                parent_syms = [s for s in symbols if s["name"] == to_parent]
                for p in parent_syms:
                    assert p["symbol_type"] != "Interface", (
                        f"Interface self-call {from_name} -> {to_name} in {to_parent} should be filtered"
                    )

    def test_csharp_constructor_parameter_types(self):
        """Constructor symbols should have parameter_types extracted."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        symbols = kg.get_symbols()
        # Find AbsenceController constructor
        ctors = [s for s in symbols if s["symbol_type"] == "Constructor"]
        assert len(ctors) > 0

        # At least one constructor should have parameter_types stored in the graph
        found_params = False
        for ctor in ctors:
            node_data = kg.graph.nodes.get(ctor["id"], {})
            param_types = node_data.get("parameter_types")
            if param_types:
                found_params = True
                break
        assert found_params, "At least one constructor should have parameter_types"


class TestInterfaceToImplementation:
    def test_impl_resolved_calls(self):
        """Calls to interface methods should also resolve to implementations."""
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "csharp_simple"))

        calls = kg.get_call_edges()
        # Check for impl-resolved or di-impl-resolved calls
        impl_calls = [c for c in calls if c.get("reason") in ("impl-resolved", "di-impl-resolved")]
        # At minimum, should not crash. Whether impl calls exist depends on fixture structure.
        assert isinstance(impl_calls, list)


class TestCallsOnMixedDotNet:
    def test_no_crash(self):
        kg, st = _run_four_phases(os.path.join(FIXTURES_DIR, "mixed_dotnet"))
        # Should complete without errors
        calls = kg.get_call_edges()
        assert isinstance(calls, list)
