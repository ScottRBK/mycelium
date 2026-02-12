"""Tests for Phase 6: Processes (BFS execution flow detection)."""

from __future__ import annotations

import os

from mycelium.config import (
    AnalysisConfig, CallEdge, Community, FileNode, Symbol, SymbolType, Visibility,
)
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.scoring import score_entry_points
from mycelium.phases.communities import run_communities_phase
from mycelium.phases.processes import run_processes_phase, _deduplicate, _bfs_traces

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


def _build_process_graph() -> KnowledgeGraph:
    """Build a graph with a clear entry point and call chain."""
    kg = KnowledgeGraph()
    kg.add_file(FileNode(path="controller.cs", language="cs"))
    kg.add_file(FileNode(path="service.cs", language="cs"))

    # Controller (entry point) -> Service -> Repository
    kg.add_symbol(Symbol(
        id="ctrl", name="HandleRequest", type=SymbolType.METHOD,
        file="controller.cs", line=1, visibility=Visibility.PUBLIC,
        exported=True, parent="ApiController", language="cs",
    ))
    kg.add_symbol(Symbol(
        id="svc", name="ProcessData", type=SymbolType.METHOD,
        file="service.cs", line=1, visibility=Visibility.PUBLIC,
        exported=True, language="cs",
    ))
    kg.add_symbol(Symbol(
        id="repo", name="SaveData", type=SymbolType.METHOD,
        file="service.cs", line=10, visibility=Visibility.PUBLIC,
        exported=True, language="cs",
    ))
    kg.add_symbol(Symbol(
        id="helper", name="FormatData", type=SymbolType.METHOD,
        file="service.cs", line=20, visibility=Visibility.PRIVATE,
        exported=False, language="cs",
    ))

    # Call chain: ctrl -> svc -> repo, svc -> helper
    kg.add_call(CallEdge(from_symbol="ctrl", to_symbol="svc", confidence=0.9, tier="A", reason="import-resolved", line=5))
    kg.add_call(CallEdge(from_symbol="svc", to_symbol="repo", confidence=0.85, tier="B", reason="same-file", line=3))
    kg.add_call(CallEdge(from_symbol="svc", to_symbol="helper", confidence=0.85, tier="B", reason="same-file", line=4))

    return kg


class TestEntryPointScoring:
    def test_controllers_rank_higher(self):
        kg = _build_process_graph()
        scores = score_entry_points(kg)

        assert len(scores) > 0
        # HandleRequest (in ApiController, public, has callees, no callers)
        # should rank highest
        top_id = scores[0][0]
        assert top_id == "ctrl"

    def test_scoring_formula(self):
        kg = _build_process_graph()
        scores = score_entry_points(kg)

        score_map = dict(scores)
        # ctrl: base = 1/(0+1) = 1, export=2.0, name_mult=1.3 (parent=ApiController)
        ctrl_score = score_map.get("ctrl", 0)
        assert ctrl_score > 0

    def test_no_entry_points_without_calls(self):
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="s1", name="Foo", type=SymbolType.METHOD,
            file="a.cs", line=1, language="cs",
        ))
        scores = score_entry_points(kg)
        assert scores == []

    def test_test_files_excluded_from_entry_points(self):
        """Methods in test files should not be scored as entry points."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="Tests/AbsenceTests.cs", language="cs"))
        kg.add_file(FileNode(path="Controllers/Absence.cs", language="cs"))
        # Test method with outgoing calls
        kg.add_symbol(Symbol(
            id="test_m", name="TestGetAbsence", type=SymbolType.METHOD,
            file="Tests/AbsenceTests.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        # Production method with outgoing calls
        kg.add_symbol(Symbol(
            id="prod_m", name="GetAbsence", type=SymbolType.METHOD,
            file="Controllers/Absence.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, parent="AbsenceController", language="cs",
        ))
        kg.add_symbol(Symbol(
            id="helper", name="Helper", type=SymbolType.METHOD,
            file="Controllers/Absence.cs", line=10, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="test_m", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=1))
        kg.add_call(CallEdge(from_symbol="prod_m", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=2))

        scores = score_entry_points(kg)
        scored_ids = {s[0] for s in scores}
        assert "test_m" not in scored_ids, "Test method should not be an entry point"
        assert "prod_m" in scored_ids, "Production method should be an entry point"

    def test_dot_separated_test_project_excluded(self):
        """Methods in dotted .Tests/ paths (e.g. FrameworkAPI.Tests/) should be excluded."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="FrameworkAPI.Tests/ServiceTests.cs", language="cs"))
        kg.add_file(FileNode(path="FrameworkAPI/Services/Api.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="test_m", name="TestService", type=SymbolType.METHOD,
            file="FrameworkAPI.Tests/ServiceTests.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="prod_m", name="GetService", type=SymbolType.METHOD,
            file="FrameworkAPI/Services/Api.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, parent="ApiController", language="cs",
        ))
        kg.add_symbol(Symbol(
            id="helper", name="Help", type=SymbolType.METHOD,
            file="FrameworkAPI/Services/Api.cs", line=10, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="test_m", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=1))
        kg.add_call(CallEdge(from_symbol="prod_m", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=2))

        scores = score_entry_points(kg)
        scored_ids = {s[0] for s in scores}
        assert "test_m" not in scored_ids, "Dot-separated .Tests/ method should not be entry point"
        assert "prod_m" in scored_ids

    def test_test_harness_excluded_from_entry_points(self):
        """Methods in TestHarness directories should not be scored."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="TestHarness/Form1.cs", language="cs"))
        kg.add_file(FileNode(path="Controllers/Api.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="harness", name="RunTest", type=SymbolType.METHOD,
            file="TestHarness/Form1.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="api", name="GetData", type=SymbolType.METHOD,
            file="Controllers/Api.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, parent="ApiController", language="cs",
        ))
        kg.add_symbol(Symbol(
            id="helper", name="Help", type=SymbolType.METHOD,
            file="Controllers/Api.cs", line=10, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="harness", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=1))
        kg.add_call(CallEdge(from_symbol="api", to_symbol="helper", confidence=0.9, tier="B", reason="same-file", line=2))

        scores = score_entry_points(kg)
        scored_ids = {s[0] for s in scores}
        assert "harness" not in scored_ids, "TestHarness method should not be an entry point"
        assert "api" in scored_ids

    def test_depth_bonus_rewards_deep_chains(self):
        """Methods with deeper call chains should score higher."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        # Shallow: calls 1 method
        kg.add_symbol(Symbol(
            id="shallow", name="Shallow", type=SymbolType.METHOD,
            file="a.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="s_target", name="STarget", type=SymbolType.METHOD,
            file="a.cs", line=5, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="shallow", to_symbol="s_target", confidence=0.9, tier="B", reason="same-file", line=1))

        # Deep: calls method that calls another
        kg.add_symbol(Symbol(
            id="deep", name="Deep", type=SymbolType.METHOD,
            file="a.cs", line=20, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="d1", name="D1", type=SymbolType.METHOD,
            file="a.cs", line=30, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="d2", name="D2", type=SymbolType.METHOD,
            file="a.cs", line=40, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="d3", name="D3", type=SymbolType.METHOD,
            file="a.cs", line=50, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="deep", to_symbol="d1", confidence=0.9, tier="B", reason="same-file", line=20))
        kg.add_call(CallEdge(from_symbol="d1", to_symbol="d2", confidence=0.9, tier="B", reason="same-file", line=30))
        kg.add_call(CallEdge(from_symbol="d2", to_symbol="d3", confidence=0.9, tier="B", reason="same-file", line=40))

        scores = score_entry_points(kg)
        score_map = dict(scores)
        # Deep should score higher due to depth bonus
        assert score_map.get("deep", 0) > score_map.get("shallow", 0)


class TestProcessDetection:
    def test_detects_process(self):
        kg = _build_process_graph()
        config = AnalysisConfig()
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        assert len(processes) >= 1

    def test_process_starts_at_entry(self):
        kg = _build_process_graph()
        config = AnalysisConfig()
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        # The main process should start at the controller
        entries = {p["entry"] for p in processes}
        assert "ctrl" in entries

    def test_process_has_steps(self):
        kg = _build_process_graph()
        config = AnalysisConfig()
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        for proc in processes:
            assert len(proc["steps"]) >= 2

    def test_process_has_confidence(self):
        kg = _build_process_graph()
        config = AnalysisConfig()
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        for proc in processes:
            assert 0.0 < proc["total_confidence"] <= 1.0

    def test_process_classification(self):
        kg = _build_process_graph()
        config = AnalysisConfig()
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        for proc in processes:
            assert proc["type"] in ("intra_community", "cross_community")

    def test_min_steps_filter(self):
        """Processes shorter than min_steps should be filtered."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="s1", name="DoThing", type=SymbolType.METHOD,
            file="a.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="s2", name="Helper", type=SymbolType.METHOD,
            file="a.cs", line=5, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="s1", to_symbol="s2", confidence=0.9, tier="B", reason="same-file", line=2))

        config = AnalysisConfig(min_steps=3)
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        for proc in processes:
            assert len(proc["steps"]) >= 3


class TestMultiBranchBFS:
    def test_branching_produces_multiple_traces(self):
        """Entry point calling 3 methods should produce multiple traces."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))

        # Entry point calls 3 different methods
        kg.add_symbol(Symbol(
            id="entry", name="HandleRequest", type=SymbolType.METHOD,
            file="a.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, parent="Controller", language="cs",
        ))
        for i in range(1, 4):
            kg.add_symbol(Symbol(
                id=f"m{i}", name=f"Method{i}", type=SymbolType.METHOD,
                file="a.cs", line=i * 10, visibility=Visibility.PUBLIC,
                exported=True, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol="entry", to_symbol=f"m{i}",
                confidence=0.9, tier="B", reason="same-file", line=i,
            ))
            # Each method calls a sub-method
            kg.add_symbol(Symbol(
                id=f"sub{i}", name=f"Sub{i}", type=SymbolType.METHOD,
                file="a.cs", line=i * 10 + 5, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol=f"m{i}", to_symbol=f"sub{i}",
                confidence=0.85, tier="B", reason="same-file", line=i * 10,
            ))

        traces = _bfs_traces(kg, "entry", max_depth=10, max_branching=4, min_steps=2)
        assert len(traces) >= 3, f"Expected at least 3 traces, got {len(traces)}"

    def test_cycle_detection(self):
        """A -> B -> A should not loop."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="a", name="A", type=SymbolType.METHOD,
            file="a.cs", line=1, language="cs",
        ))
        kg.add_symbol(Symbol(
            id="b", name="B", type=SymbolType.METHOD,
            file="a.cs", line=10, language="cs",
        ))
        kg.add_call(CallEdge(from_symbol="a", to_symbol="b", confidence=0.9, tier="B", reason="same-file", line=1))
        kg.add_call(CallEdge(from_symbol="b", to_symbol="a", confidence=0.9, tier="B", reason="same-file", line=10))

        traces = _bfs_traces(kg, "a", max_depth=10, max_branching=4, min_steps=2)
        # Should produce a trace but not loop
        for trace in traces:
            assert len(trace) <= 10

    def test_max_traces_cap(self):
        """Highly branching graph should stay bounded."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        kg.add_symbol(Symbol(
            id="root", name="Root", type=SymbolType.METHOD,
            file="a.cs", line=1, language="cs",
        ))
        # Root calls 10 methods, each calls 10 more
        for i in range(10):
            kg.add_symbol(Symbol(
                id=f"l1_{i}", name=f"Level1_{i}", type=SymbolType.METHOD,
                file="a.cs", line=i * 100, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol="root", to_symbol=f"l1_{i}",
                confidence=0.9, tier="B", reason="same-file", line=i,
            ))
            for j in range(10):
                kg.add_symbol(Symbol(
                    id=f"l2_{i}_{j}", name=f"Level2_{i}_{j}", type=SymbolType.METHOD,
                    file="a.cs", line=i * 100 + j, language="cs",
                ))
                kg.add_call(CallEdge(
                    from_symbol=f"l1_{i}", to_symbol=f"l2_{i}_{j}",
                    confidence=0.85, tier="B", reason="same-file", line=i * 100 + j,
                ))

        traces = _bfs_traces(kg, "root", max_depth=10, max_branching=4, min_steps=2)
        # Should be capped at max_branching * 3 = 12
        assert len(traces) <= 12


class TestDeduplication:
    def test_removes_subsets(self):
        traces = [
            ["a", "b", "c"],
            ["a", "b"],
            ["x", "y", "z"],
        ]
        result = _deduplicate(traces)
        assert ["a", "b"] not in result
        assert len(result) == 2

    def test_keeps_non_subsets(self):
        traces = [
            ["a", "b", "c"],
            ["x", "y", "z"],
        ]
        result = _deduplicate(traces)
        assert len(result) == 2

    def test_keeps_identical(self):
        traces = [
            ["a", "b", "c"],
            ["a", "b", "c"],
        ]
        result = _deduplicate(traces)
        assert len(result) == 2  # Same set but order preserved


class TestNormalisedConfidenceSorting:
    def test_longer_traces_not_crowded_out(self):
        """Longer traces with same per-hop confidence should compete with shorter ones."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="api.cs", language="cs"))

        # Create many 2-step traces (entry -> leaf)
        for i in range(80):
            kg.add_symbol(Symbol(
                id=f"short_entry_{i}", name=f"ShortHandler{i}", type=SymbolType.METHOD,
                file="api.cs", line=i + 1, visibility=Visibility.PUBLIC,
                exported=True, parent="ShortController", language="cs",
            ))
            kg.add_symbol(Symbol(
                id=f"short_leaf_{i}", name=f"ShortLeaf{i}", type=SymbolType.METHOD,
                file="api.cs", line=1000 + i, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol=f"short_entry_{i}", to_symbol=f"short_leaf_{i}",
                confidence=0.9, tier="B", reason="same-file", line=i + 1,
            ))

        # Create a deep 5-step trace: deep -> d1 -> d2 -> d3 -> d4
        kg.add_symbol(Symbol(
            id="deep", name="DeepHandler", type=SymbolType.METHOD,
            file="api.cs", line=2000, visibility=Visibility.PUBLIC,
            exported=True, parent="DeepController", language="cs",
        ))
        for j in range(1, 5):
            kg.add_symbol(Symbol(
                id=f"d{j}", name=f"Deep{j}", type=SymbolType.METHOD,
                file="api.cs", line=2000 + j * 10, language="cs",
            ))
        kg.add_call(CallEdge(from_symbol="deep", to_symbol="d1", confidence=0.9, tier="A", reason="import-resolved", line=2001))
        kg.add_call(CallEdge(from_symbol="d1", to_symbol="d2", confidence=0.9, tier="B", reason="same-file", line=2011))
        kg.add_call(CallEdge(from_symbol="d2", to_symbol="d3", confidence=0.9, tier="B", reason="same-file", line=2021))
        kg.add_call(CallEdge(from_symbol="d3", to_symbol="d4", confidence=0.9, tier="B", reason="same-file", line=2031))

        config = AnalysisConfig(max_processes=75, min_steps=2)
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        # The deep trace should be present despite 80 competing 2-step traces
        deep_processes = [p for p in processes if p["entry"] == "deep"]
        assert len(deep_processes) > 0, "Deep trace should not be crowded out by short traces"
        assert len(deep_processes[0]["steps"]) >= 5

    def test_normalised_confidence_favours_deeper(self):
        """Two traces with same per-hop conf: longer should sort higher (tiebreak)."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))

        # 2-step: a -> b (conf 0.9)
        kg.add_symbol(Symbol(id="a", name="A", type=SymbolType.METHOD, file="a.cs", line=1,
                             visibility=Visibility.PUBLIC, exported=True, parent="Controller", language="cs"))
        kg.add_symbol(Symbol(id="b", name="B", type=SymbolType.METHOD, file="a.cs", line=2, language="cs"))
        kg.add_call(CallEdge(from_symbol="a", to_symbol="b", confidence=0.9, tier="B", reason="same-file", line=1))

        # 3-step: x -> y -> z (conf 0.9, 0.9)
        kg.add_symbol(Symbol(id="x", name="X", type=SymbolType.METHOD, file="a.cs", line=10,
                             visibility=Visibility.PUBLIC, exported=True, parent="Controller", language="cs"))
        kg.add_symbol(Symbol(id="y", name="Y", type=SymbolType.METHOD, file="a.cs", line=11, language="cs"))
        kg.add_symbol(Symbol(id="z", name="Z", type=SymbolType.METHOD, file="a.cs", line=12, language="cs"))
        kg.add_call(CallEdge(from_symbol="x", to_symbol="y", confidence=0.9, tier="B", reason="same-file", line=10))
        kg.add_call(CallEdge(from_symbol="y", to_symbol="z", confidence=0.9, tier="B", reason="same-file", line=11))

        config = AnalysisConfig(max_processes=75, min_steps=2)
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        assert len(processes) >= 2
        # Longer trace should be first (same normalised conf, length tiebreak)
        three_step = [p for p in processes if len(p["steps"]) == 3]
        two_step = [p for p in processes if len(p["steps"]) == 2]
        assert len(three_step) > 0
        assert len(two_step) > 0
        # Find their positions
        proc_ids = [p["id"] for p in processes]
        three_idx = proc_ids.index(three_step[0]["id"])
        two_idx = proc_ids.index(two_step[0]["id"])
        assert three_idx < two_idx, "3-step trace should rank before 2-step with same per-hop confidence"


class TestDepthDiverseSelection:
    def test_deep_traces_guaranteed_slots(self):
        """Multi-step traces should be guaranteed slots even when outnumbered by 2-step."""
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="api.cs", language="cs"))

        # Create 200 2-step traces (far more than max_processes)
        for i in range(200):
            kg.add_symbol(Symbol(
                id=f"s_entry_{i}", name=f"Handler{i}", type=SymbolType.METHOD,
                file="api.cs", line=i + 1, visibility=Visibility.PUBLIC,
                exported=True, parent="Controller", language="cs",
            ))
            kg.add_symbol(Symbol(
                id=f"s_leaf_{i}", name=f"Leaf{i}", type=SymbolType.METHOD,
                file="api.cs", line=5000 + i, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol=f"s_entry_{i}", to_symbol=f"s_leaf_{i}",
                confidence=0.9, tier="B", reason="same-file", line=i + 1,
            ))

        # Create 20 3-step traces
        for i in range(20):
            kg.add_symbol(Symbol(
                id=f"d_entry_{i}", name=f"DeepHandler{i}", type=SymbolType.METHOD,
                file="api.cs", line=3000 + i * 10, visibility=Visibility.PUBLIC,
                exported=True, parent="DeepController", language="cs",
            ))
            kg.add_symbol(Symbol(
                id=f"d_mid_{i}", name=f"Mid{i}", type=SymbolType.METHOD,
                file="api.cs", line=3000 + i * 10 + 1, language="cs",
            ))
            kg.add_symbol(Symbol(
                id=f"d_end_{i}", name=f"End{i}", type=SymbolType.METHOD,
                file="api.cs", line=3000 + i * 10 + 2, language="cs",
            ))
            kg.add_call(CallEdge(
                from_symbol=f"d_entry_{i}", to_symbol=f"d_mid_{i}",
                confidence=0.9, tier="A", reason="import-resolved", line=3000 + i * 10,
            ))
            kg.add_call(CallEdge(
                from_symbol=f"d_mid_{i}", to_symbol=f"d_end_{i}",
                confidence=0.9, tier="B", reason="same-file", line=3000 + i * 10 + 1,
            ))

        config = AnalysisConfig(max_processes=75, min_steps=2)
        run_processes_phase(config, kg)

        processes = kg.get_processes()
        multi_step = [p for p in processes if len(p["steps"]) > 2]
        # All 20 deep traces should be included (20 < 75//2 = 37)
        assert len(multi_step) >= 15, (
            f"Expected most multi-step traces to be included, got {len(multi_step)}"
        )


class TestEndToEnd:
    def test_full_pipeline_with_processes(self):
        """Full pipeline on C# fixtures produces processes."""
        from mycelium.graph.symbol_table import SymbolTable
        from mycelium.phases.structure import run_structure_phase
        from mycelium.phases.parsing import run_parsing_phase
        from mycelium.phases.imports import run_imports_phase
        from mycelium.phases.calls import run_calls_phase

        kg = KnowledgeGraph()
        st = SymbolTable()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)
        run_parsing_phase(config, kg, st)
        run_imports_phase(config, kg, st)
        run_calls_phase(config, kg, st)
        run_communities_phase(config, kg)
        run_processes_phase(config, kg)

        # The pipeline should run without errors
        processes = kg.get_processes()
        assert isinstance(processes, list)
