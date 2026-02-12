"""Scaffolding tests for Milestone 1."""

from __future__ import annotations

import json
import os
import tempfile

from click.testing import CliRunner

from mycelium.cli import cli
from mycelium.config import (
    AnalysisConfig,
    AnalysisResult,
    CallEdge,
    Community,
    FileNode,
    FolderNode,
    ImportEdge,
    PackageReference,
    Process,
    ProjectReference,
    Symbol,
    SymbolType,
    Visibility,
)
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.graph.symbol_table import SymbolTable, SymbolDefinition
from mycelium.output import build_result, write_output
from mycelium.pipeline import run_pipeline


class TestSymbolTable:
    def test_add_and_lookup_exact(self):
        st = SymbolTable()
        sym = Symbol(
            id="sym_001",
            name="MyClass",
            type=SymbolType.CLASS,
            file="src/MyClass.cs",
            line=10,
            language="cs",
        )
        st.add(sym)

        assert st.lookup_exact("src/MyClass.cs", "MyClass") == "sym_001"
        assert st.lookup_exact("src/MyClass.cs", "Other") is None
        assert st.lookup_exact("other.cs", "MyClass") is None

    def test_lookup_fuzzy(self):
        st = SymbolTable()
        sym1 = Symbol(
            id="sym_001", name="Calculate", type=SymbolType.METHOD,
            file="a.cs", line=1, language="cs",
        )
        sym2 = Symbol(
            id="sym_002", name="Calculate", type=SymbolType.METHOD,
            file="b.cs", line=5, language="cs",
        )
        st.add(sym1)
        st.add(sym2)

        results = st.lookup_fuzzy("Calculate")
        assert len(results) == 2
        assert results[0].symbol_id == "sym_001"
        assert results[1].symbol_id == "sym_002"

    def test_lookup_fuzzy_not_found(self):
        st = SymbolTable()
        assert st.lookup_fuzzy("NonExistent") == []

    def test_get_symbols_in_file(self):
        st = SymbolTable()
        st.add(Symbol(id="s1", name="A", type=SymbolType.CLASS, file="x.cs", line=1))
        st.add(Symbol(id="s2", name="B", type=SymbolType.METHOD, file="x.cs", line=10))
        st.add(Symbol(id="s3", name="C", type=SymbolType.CLASS, file="y.cs", line=1))

        x_syms = st.get_symbols_in_file("x.cs")
        assert len(x_syms) == 2
        assert x_syms["A"] == "s1"
        assert x_syms["B"] == "s2"


class TestKnowledgeGraph:
    def test_add_file_and_query(self):
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="src/main.cs", language="cs", size=100, lines=20))

        files = kg.get_files()
        assert len(files) == 1
        assert files[0]["path"] == "src/main.cs"
        assert files[0]["language"] == "cs"

    def test_add_folder_and_query(self):
        kg = KnowledgeGraph()
        kg.add_folder(FolderNode(path="src/", file_count=5))

        folders = kg.get_folders()
        assert len(folders) == 1
        assert folders[0]["path"] == "src/"
        assert folders[0]["file_count"] == 5

    def test_add_symbol_creates_defines_edge(self):
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs", language="cs"))
        sym = Symbol(
            id="sym_001", name="Foo", type=SymbolType.CLASS,
            file="a.cs", line=1, visibility=Visibility.PUBLIC,
            exported=True, language="cs",
        )
        kg.add_symbol(sym)

        symbols = kg.get_symbols_in_file("a.cs")
        assert len(symbols) == 1
        assert symbols[0]["id"] == "sym_001"
        assert symbols[0]["name"] == "Foo"

    def test_add_call_and_query(self):
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs"))
        kg.add_symbol(Symbol(id="s1", name="A", type=SymbolType.METHOD, file="a.cs", line=1))
        kg.add_symbol(Symbol(id="s2", name="B", type=SymbolType.METHOD, file="a.cs", line=10))
        kg.add_call(CallEdge(
            from_symbol="s1", to_symbol="s2",
            confidence=0.9, tier="A", reason="import-resolved", line=5,
        ))

        callees = kg.get_callees("s1")
        assert len(callees) == 1
        assert callees[0]["id"] == "s2"

        callers = kg.get_callers("s2")
        assert len(callers) == 1
        assert callers[0]["id"] == "s1"

    def test_counts(self):
        kg = KnowledgeGraph()
        kg.add_file(FileNode(path="a.cs"))
        kg.add_file(FileNode(path="b.cs"))
        kg.add_folder(FolderNode(path="src/"))
        kg.add_symbol(Symbol(id="s1", name="A", type=SymbolType.CLASS, file="a.cs", line=1))

        assert kg.file_count() == 2
        assert kg.folder_count() == 1
        assert kg.symbol_count() == 1


class TestPipeline:
    def test_run_on_empty_directory(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            config = AnalysisConfig(repo_path=tmpdir)
            result = run_pipeline(config)

            assert result.version == "1.0"
            assert result.metadata["repo_path"] == tmpdir
            assert result.stats["files"] == 0
            assert result.stats["symbols"] == 0


class TestOutput:
    def test_write_and_read_json(self):
        result = AnalysisResult(
            version="1.0",
            metadata={"repo_name": "test", "mycelium_version": "0.1.0"},
            stats={"files": 0, "symbols": 0},
        )

        with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
            path = f.name

        try:
            write_output(result, path)
            with open(path) as f:
                data = json.load(f)

            assert data["version"] == "1.0"
            assert data["metadata"]["repo_name"] == "test"
        finally:
            os.unlink(path)


class TestCLI:
    def test_analyze_empty_dir(self):
        runner = CliRunner()
        with tempfile.TemporaryDirectory() as tmpdir:
            output_path = os.path.join(tmpdir, "out.mycelium.json")
            result = runner.invoke(cli, ["analyze", tmpdir, "-o", output_path])
            assert result.exit_code == 0
            assert os.path.exists(output_path)

            with open(output_path) as f:
                data = json.load(f)
            assert data["version"] == "1.0"

    def test_analyze_quiet(self):
        runner = CliRunner()
        with tempfile.TemporaryDirectory() as tmpdir:
            output_path = os.path.join(tmpdir, "out.json")
            result = runner.invoke(cli, ["analyze", tmpdir, "-o", output_path, "--quiet"])
            assert result.exit_code == 0
            assert result.output == ""
