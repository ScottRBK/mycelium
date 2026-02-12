"""Namespace-to-file index for namespace-aware import resolution."""

from __future__ import annotations


class NamespaceIndex:
    """Maps namespaces to files and tracks file imports."""

    def __init__(self) -> None:
        self.ns_to_files: dict[str, list[str]] = {}
        self.file_to_ns: dict[str, list[str]] = {}
        self.file_imports: dict[str, list[str]] = {}

    def register(self, namespace: str, file_path: str) -> None:
        """Register that a file declares the given namespace."""
        if namespace not in self.ns_to_files:
            self.ns_to_files[namespace] = []
        if file_path not in self.ns_to_files[namespace]:
            self.ns_to_files[namespace].append(file_path)

        if file_path not in self.file_to_ns:
            self.file_to_ns[file_path] = []
        if namespace not in self.file_to_ns[file_path]:
            self.file_to_ns[file_path].append(namespace)

    def get_files_for_namespace(self, namespace: str) -> list[str]:
        """Get all files that declare the given namespace."""
        return self.ns_to_files.get(namespace, [])

    def register_file_import(self, file_path: str, namespace: str) -> None:
        """Record that a file imports the given namespace."""
        if file_path not in self.file_imports:
            self.file_imports[file_path] = []
        if namespace not in self.file_imports[file_path]:
            self.file_imports[file_path].append(namespace)

    def get_imported_namespaces(self, file_path: str) -> list[str]:
        """Get all namespace names imported by a file."""
        return self.file_imports.get(file_path, [])
