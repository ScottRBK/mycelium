"""Tests for Phase 1: Structure."""

from __future__ import annotations

import os

from mycelium.config import AnalysisConfig
from mycelium.graph.knowledge_graph import KnowledgeGraph
from mycelium.phases.structure import run_structure_phase

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


class TestStructurePhase:
    def test_finds_csharp_files(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        files = kg.get_files()
        file_paths = {f["path"] for f in files}

        assert "AbsenceController.cs" in file_paths
        assert "AbsenceService.cs" in file_paths
        assert "IAbsenceService.cs" in file_paths
        assert "AbsenceModel.cs" in file_paths
        assert "IAbsenceRepository.cs" in file_paths
        assert "AbsenceRepository.cs" in file_paths
        assert "LeaveRequestValidator.cs" in file_paths
        assert "AbsenceException.cs" in file_paths
        assert len(files) == 8

    def test_detects_language(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        files = kg.get_files()
        for f in files:
            assert f["language"] == "cs"

    def test_counts_lines(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        files = {f["path"]: f for f in kg.get_files()}
        # AbsenceController.cs has 27 lines
        assert files["AbsenceController.cs"]["lines"] > 0

    def test_creates_folders(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        folders = kg.get_folders()
        assert len(folders) >= 1  # At least the root folder

    def test_skips_ignored_directories(self):
        """Ensure bin, obj, etc. are skipped."""
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        file_paths = {f["path"] for f in kg.get_files()}
        for path in file_paths:
            assert "bin/" not in path
            assert "obj/" not in path
            assert ".git/" not in path

    def test_mixed_dotnet_finds_all_file_types(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "mixed_dotnet"))
        run_structure_phase(config, kg)

        files = kg.get_files()
        file_paths = {f["path"] for f in files}

        # Should find .cs, .vb, .csproj, .vbproj, .sln files
        assert any(p.endswith(".cs") for p in file_paths)
        assert any(p.endswith(".vb") for p in file_paths)
        assert any(p.endswith(".csproj") for p in file_paths)
        assert any(p.endswith(".vbproj") for p in file_paths)
        assert any(p.endswith(".sln") for p in file_paths)

    def test_records_file_size(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "csharp_simple"))
        run_structure_phase(config, kg)

        files = kg.get_files()
        for f in files:
            assert f["size"] > 0

    def test_vbnet_files_detected(self):
        kg = KnowledgeGraph()
        config = AnalysisConfig(repo_path=os.path.join(FIXTURES_DIR, "vbnet_simple"))
        run_structure_phase(config, kg)

        files = kg.get_files()
        vb_files = [f for f in files if f["language"] == "vb"]
        assert len(vb_files) == 5
