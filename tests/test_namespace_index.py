"""Tests for NamespaceIndex."""

from mycelium.graph.namespace_index import NamespaceIndex


class TestNamespaceIndex:
    def test_register_and_lookup(self):
        idx = NamespaceIndex()
        idx.register("Absence.Services", "AbsenceService.cs")
        assert idx.get_files_for_namespace("Absence.Services") == ["AbsenceService.cs"]

    def test_multiple_files_per_namespace(self):
        idx = NamespaceIndex()
        idx.register("Absence.Services", "AbsenceService.cs")
        idx.register("Absence.Services", "IAbsenceService.cs")
        files = idx.get_files_for_namespace("Absence.Services")
        assert len(files) == 2
        assert "AbsenceService.cs" in files
        assert "IAbsenceService.cs" in files

    def test_no_match(self):
        idx = NamespaceIndex()
        assert idx.get_files_for_namespace("NonExistent") == []

    def test_file_imports(self):
        idx = NamespaceIndex()
        idx.register_file_import("Controller.cs", "Absence.Services")
        assert idx.get_imported_namespaces("Controller.cs") == ["Absence.Services"]

    def test_no_duplicate_registrations(self):
        idx = NamespaceIndex()
        idx.register("Absence.Services", "AbsenceService.cs")
        idx.register("Absence.Services", "AbsenceService.cs")
        assert len(idx.get_files_for_namespace("Absence.Services")) == 1

    def test_file_to_namespace_mapping(self):
        idx = NamespaceIndex()
        idx.register("Absence.Services", "AbsenceService.cs")
        idx.register("Absence.Models", "AbsenceService.cs")
        assert len(idx.file_to_ns["AbsenceService.cs"]) == 2
