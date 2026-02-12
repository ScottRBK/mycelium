"""Namespace-to-project mapping for cross-project resolution."""

from __future__ import annotations


class AssemblyMapper:
    """Maps namespaces to project paths for cross-project resolution.

    Seeded from <RootNamespace> in .csproj/.vbproj files, then supplemented
    by observed namespace declarations in source files.
    """

    def __init__(self) -> None:
        self.namespace_map: dict[str, str] = {}

    def register_namespace(self, namespace: str, project_path: str) -> None:
        """Register a namespace as belonging to a project."""
        self.namespace_map[namespace] = project_path

    def resolve_namespace(self, namespace: str) -> str | None:
        """Resolve a namespace to the project that owns it.

        Tries exact match first, then prefix matching (e.g.,
        'Absence.Services.Internal' matches 'Absence.Services' if
        'Absence' project registered 'Absence' as root namespace).
        """
        # Exact match
        if namespace in self.namespace_map:
            return self.namespace_map[namespace]

        # Prefix match: find the longest matching namespace prefix
        best_match = None
        best_len = 0
        for ns, project in self.namespace_map.items():
            if namespace.startswith(ns) and len(ns) > best_len:
                # Ensure it matches at a dot boundary
                if len(namespace) == len(ns) or namespace[len(ns)] == ".":
                    best_match = project
                    best_len = len(ns)

        return best_match

    def get_all_namespaces(self) -> dict[str, str]:
        """Return the full namespace-to-project mapping."""
        return dict(self.namespace_map)
