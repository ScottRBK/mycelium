"""In-memory knowledge graph backed by networkx.DiGraph."""

from __future__ import annotations

import networkx as nx

from mycelium.config import (
    CallEdge,
    Community,
    FileNode,
    FolderNode,
    ImportEdge,
    PackageReference,
    Process,
    ProjectReference,
    Symbol,
)


class KnowledgeGraph:
    """Wrapper around networkx.DiGraph with typed node/edge methods."""

    def __init__(self) -> None:
        self.graph = nx.DiGraph()

    # --- Node addition ---

    def add_file(self, node: FileNode) -> None:
        self.graph.add_node(
            f"file:{node.path}",
            node_type="file",
            path=node.path,
            language=node.language,
            size=node.size,
            lines=node.lines,
        )

    def add_folder(self, node: FolderNode) -> None:
        self.graph.add_node(
            f"folder:{node.path}",
            node_type="folder",
            path=node.path,
            file_count=node.file_count,
        )

    def add_symbol(self, symbol: Symbol) -> None:
        attrs = {
            "node_type": "symbol",
            "name": symbol.name,
            "symbol_type": symbol.type.value,
            "file": symbol.file,
            "line": symbol.line,
            "visibility": symbol.visibility.value,
            "exported": symbol.exported,
            "parent": symbol.parent,
            "language": symbol.language,
        }
        if symbol.parameter_types:
            attrs["parameter_types"] = symbol.parameter_types
        self.graph.add_node(symbol.id, **attrs)
        # DEFINES edge: file -> symbol
        self.graph.add_edge(
            f"file:{symbol.file}",
            symbol.id,
            edge_type="DEFINES",
        )

    def add_call(self, edge: CallEdge) -> None:
        self.graph.add_edge(
            edge.from_symbol,
            edge.to_symbol,
            edge_type="CALLS",
            confidence=edge.confidence,
            tier=edge.tier,
            reason=edge.reason,
            line=edge.line,
        )

    def add_import(self, edge: ImportEdge) -> None:
        self.graph.add_edge(
            f"file:{edge.from_file}",
            f"file:{edge.to_file}",
            edge_type="IMPORTS",
            statement=edge.statement,
        )

    def add_project_reference(self, ref: ProjectReference) -> None:
        self.graph.add_edge(
            f"project:{ref.from_project}",
            f"project:{ref.to_project}",
            edge_type="PROJECT_REFERENCE",
            ref_type=ref.ref_type,
        )

    def add_package_reference(self, ref: PackageReference) -> None:
        pkg_id = f"package:{ref.package}"
        if not self.graph.has_node(pkg_id):
            self.graph.add_node(pkg_id, node_type="package", name=ref.package)
        self.graph.add_edge(
            f"project:{ref.project}",
            pkg_id,
            edge_type="PACKAGE_REFERENCE",
            version=ref.version,
        )

    def add_community(self, community: Community) -> None:
        self.graph.add_node(
            community.id,
            node_type="community",
            label=community.label,
            cohesion=community.cohesion,
            primary_language=community.primary_language,
        )
        for member in community.members:
            self.graph.add_edge(
                member,
                community.id,
                edge_type="MEMBER_OF",
            )

    def add_process(self, process: Process) -> None:
        self.graph.add_node(
            process.id,
            node_type="process",
            entry=process.entry,
            terminal=process.terminal,
            process_type=process.type,
            total_confidence=process.total_confidence,
        )
        for i, step in enumerate(process.steps):
            self.graph.add_edge(
                process.id,
                step,
                edge_type="STEP",
                order=i,
            )

    # --- Queries ---

    def get_files(self) -> list[dict]:
        return [
            data for _, data in self.graph.nodes(data=True) if data.get("node_type") == "file"
        ]

    def get_folders(self) -> list[dict]:
        return [
            data for _, data in self.graph.nodes(data=True) if data.get("node_type") == "folder"
        ]

    def get_symbols(self) -> list[dict]:
        return [
            {"id": nid, **data}
            for nid, data in self.graph.nodes(data=True)
            if data.get("node_type") == "symbol"
        ]

    def get_symbols_in_file(self, path: str) -> list[dict]:
        file_id = f"file:{path}"
        result = []
        for _, target, data in self.graph.out_edges(file_id, data=True):
            if data.get("edge_type") == "DEFINES":
                node_data = self.graph.nodes[target]
                result.append({"id": target, **node_data})
        return result

    def get_callers(self, symbol_id: str) -> list[dict]:
        result = []
        for source, _, data in self.graph.in_edges(symbol_id, data=True):
            if data.get("edge_type") == "CALLS":
                result.append({"id": source, **data})
        return result

    def get_callees(self, symbol_id: str) -> list[dict]:
        result = []
        for _, target, data in self.graph.out_edges(symbol_id, data=True):
            if data.get("edge_type") == "CALLS":
                result.append({"id": target, **data})
        return result

    def get_call_edges(self) -> list[dict]:
        return [
            {"from": src, "to": tgt, **data}
            for src, tgt, data in self.graph.edges(data=True)
            if data.get("edge_type") == "CALLS"
        ]

    def get_import_edges(self) -> list[dict]:
        return [
            {"from": src.removeprefix("file:"), "to": tgt.removeprefix("file:"), **data}
            for src, tgt, data in self.graph.edges(data=True)
            if data.get("edge_type") == "IMPORTS"
        ]

    def get_project_references(self) -> list[dict]:
        return [
            {
                "from": src.removeprefix("project:"),
                "to": tgt.removeprefix("project:"),
                "type": data.get("ref_type", "ProjectReference"),
            }
            for src, tgt, data in self.graph.edges(data=True)
            if data.get("edge_type") == "PROJECT_REFERENCE"
        ]

    def get_package_references(self) -> list[dict]:
        return [
            {
                "project": src.removeprefix("project:"),
                "package": self.graph.nodes[tgt].get("name", ""),
                "version": data.get("version", ""),
            }
            for src, tgt, data in self.graph.edges(data=True)
            if data.get("edge_type") == "PACKAGE_REFERENCE"
        ]

    def get_communities(self) -> list[dict]:
        results = []
        for nid, data in self.graph.nodes(data=True):
            if data.get("node_type") == "community":
                members = [
                    src
                    for src, tgt, edata in self.graph.in_edges(nid, data=True)
                    if edata.get("edge_type") == "MEMBER_OF"
                ]
                results.append({
                    "id": nid,
                    "label": data.get("label", ""),
                    "members": members,
                    "cohesion": data.get("cohesion", 0.0),
                    "primary_language": data.get("primary_language", ""),
                })
        return results

    def get_processes(self) -> list[dict]:
        results = []
        for nid, data in self.graph.nodes(data=True):
            if data.get("node_type") == "process":
                steps = sorted(
                    [
                        (edata.get("order", 0), tgt)
                        for _, tgt, edata in self.graph.out_edges(nid, data=True)
                        if edata.get("edge_type") == "STEP"
                    ]
                )
                results.append({
                    "id": nid,
                    "entry": data.get("entry", ""),
                    "terminal": data.get("terminal", ""),
                    "steps": [s[1] for s in steps],
                    "type": data.get("process_type", "intra_community"),
                    "total_confidence": data.get("total_confidence", 0.0),
                })
        return results

    def symbol_count(self) -> int:
        return sum(1 for _, d in self.graph.nodes(data=True) if d.get("node_type") == "symbol")

    def file_count(self) -> int:
        return sum(1 for _, d in self.graph.nodes(data=True) if d.get("node_type") == "file")

    def folder_count(self) -> int:
        return sum(1 for _, d in self.graph.nodes(data=True) if d.get("node_type") == "folder")
