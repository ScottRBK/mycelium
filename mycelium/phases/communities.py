"""Phase 5: Louvain community detection."""

from __future__ import annotations

import logging
import os
from collections import Counter

import networkx as nx

from mycelium.config import AnalysisConfig, Community
from mycelium.graph.knowledge_graph import KnowledgeGraph

logger = logging.getLogger(__name__)


def _split_oversized(
    community: set, undirected: nx.Graph, max_size: int,
) -> list[set]:
    """Recursively split an oversized community using Louvain on its subgraph."""
    if len(community) <= max_size:
        return [community]

    subgraph = undirected.subgraph(community).copy()
    if subgraph.number_of_edges() == 0:
        return [community]

    # Run Louvain at increasingly high resolution on the subgraph
    resolution = 2.0
    for _ in range(8):
        try:
            sub_communities = nx.community.louvain_communities(
                subgraph, resolution=resolution, seed=42,
            )
        except Exception:
            return [community]

        # If we got more than 1 partition, recurse on any still-oversized parts
        if len(sub_communities) > 1:
            result = []
            for sc in sub_communities:
                result.extend(_split_oversized(sc, undirected, max_size))
            return result

        resolution *= 2

    # Louvain can't split further - return as-is
    return [community]


def run_communities_phase(config: AnalysisConfig, kg: KnowledgeGraph) -> None:
    """Cluster the call graph using Louvain community detection."""
    # Build an undirected weighted graph from CALLS edges
    undirected = nx.Graph()

    call_edges = kg.get_call_edges()
    if not call_edges:
        return

    # Add edges from call graph
    for edge in call_edges:
        src = edge["from"]
        tgt = edge["to"]
        conf = edge.get("confidence", 0.5)
        if undirected.has_edge(src, tgt):
            # Increase weight for multiple calls
            undirected[src][tgt]["weight"] += conf
        else:
            undirected.add_edge(src, tgt, weight=conf)

    if undirected.number_of_nodes() < 2:
        return

    # Run Louvain community detection with auto-tuning
    resolution = config.resolution
    max_resolution = 10.0

    try:
        communities = nx.community.louvain_communities(
            undirected,
            resolution=resolution,
            seed=42,
        )
    except Exception as e:
        logger.warning(f"Community detection failed: {e}")
        return

    # Auto-tune: double resolution until largest is under threshold
    largest = max(len(c) for c in communities) if communities else 0
    while largest > config.max_community_size and resolution < max_resolution:
        resolution *= 2
        logger.info(
            f"Largest community has {largest} members (threshold {config.max_community_size}), "
            f"re-running at resolution {resolution:.1f}"
        )
        try:
            communities = nx.community.louvain_communities(
                undirected,
                resolution=resolution,
                seed=42,
            )
        except Exception as e:
            logger.warning(f"Community detection failed at resolution {resolution}: {e}")
            break
        largest = max(len(c) for c in communities) if communities else 0

    # Recursive splitting for any communities still over the threshold
    final_communities: list[set] = []
    for comm in communities:
        if len(comm) > config.max_community_size:
            final_communities.extend(
                _split_oversized(comm, undirected, config.max_community_size)
            )
        else:
            final_communities.append(comm)

    # Process each community and track labels for deduplication
    label_counts: dict[str, int] = {}
    pending: list[tuple[str, list[str], float, str]] = []

    for i, members in enumerate(final_communities):
        member_list = list(members)

        # Discard singletons
        if len(member_list) <= 1:
            continue

        # Auto-generate label
        label = _generate_label(member_list, kg)

        # Compute cohesion
        cohesion = _compute_cohesion(member_list, undirected)

        # Determine primary language
        primary_lang = _primary_language(member_list, kg)

        label_counts[label] = label_counts.get(label, 0) + 1
        pending.append((label, member_list, cohesion, primary_lang))

    # Disambiguate duplicate labels
    used_labels: set[str] = set()
    for i, (label, member_list, cohesion, primary_lang) in enumerate(pending):
        final_label = label
        if label_counts[label] > 1 or label in used_labels:
            final_label = _disambiguate_label(label, member_list, kg, used_labels)
        used_labels.add(final_label)

        community = Community(
            id=f"community_{i}",
            label=final_label,
            members=member_list,
            cohesion=round(cohesion, 3),
            primary_language=primary_lang,
        )
        kg.add_community(community)


_STRIP_DIR_SEGMENTS = {"src", "source", "sourcecode", "lib", "app"}


def _generate_label(members: list[str], kg: KnowledgeGraph) -> str:
    """Auto-generate a community label from member symbols."""
    member_set = set(members)
    file_paths = []
    names = []
    parents = []
    for sym in kg.get_symbols():
        if sym["id"] in member_set:
            file_paths.append(sym.get("file", ""))
            names.append(sym.get("name", ""))
            parent = sym.get("parent", "")
            if parent:
                parents.append(parent)

    # Strategy 1: Most common parent (namespace/class) if >= 30% coverage
    # Use most-specific (deepest) parent to avoid overly broad names
    if parents:
        parent_counts = Counter(parents)
        most_common_parent, count = parent_counts.most_common(1)[0]
        if count >= len(members) * 0.3:
            # Use the short name (last segment) if it's a dotted namespace
            short_name = most_common_parent.rsplit(".", 1)[-1]
            return short_name

    # Strategy 2: Most specific directory component after stripping common prefixes
    if file_paths:
        dirs = [os.path.dirname(p) for p in file_paths if p]
        if dirs:
            dir_counts = Counter(dirs)
            most_common_dir = dir_counts.most_common(1)[0][0]
            if most_common_dir:
                parts = most_common_dir.replace("\\", "/").split("/")
                # Strip common uninformative segments
                parts = [p for p in parts if p.lower() not in _STRIP_DIR_SEGMENTS]
                if parts:
                    return parts[-1]  # Most specific directory component

    # Strategy 3: Common name prefix if >= 3 chars
    if names:
        prefix = os.path.commonprefix(names)
        if len(prefix) >= 3:
            return prefix.rstrip("_")

    return f"Community ({len(members)} members)"


def _disambiguate_label(
    label: str, members: list[str], kg: KnowledgeGraph,
    used_labels: set[str],
) -> str:
    """Create a unique label when multiple communities share the same base label."""
    member_set = set(members)
    file_paths = []
    names = []
    parents = []
    for sym in kg.get_symbols():
        if sym["id"] in member_set:
            file_paths.append(sym.get("file", ""))
            names.append(sym.get("name", ""))
            parent = sym.get("parent", "")
            if parent:
                parents.append(parent)

    # Try using a secondary parent if available
    if parents:
        parent_counts = Counter(parents)
        if len(parent_counts) > 1:
            items = parent_counts.most_common()
            secondary = items[1][0].rsplit(".", 1)[-1]
            candidate = f"{label}/{secondary}"
            if candidate not in used_labels:
                return candidate

    # Try using directory as disambiguator
    if file_paths:
        dirs = [os.path.dirname(p) for p in file_paths if p]
        if dirs:
            most_common_dir = Counter(dirs).most_common(1)[0][0]
            if most_common_dir:
                parts = most_common_dir.replace("\\", "/").split("/")
                parts = [p for p in parts if p.lower() not in _STRIP_DIR_SEGMENTS and p != label]
                if parts:
                    candidate = f"{label}/{parts[-1]}"
                    if candidate not in used_labels:
                        return candidate

    # Try using a distinguishing member name
    if names:
        for name in sorted(names, key=len, reverse=True):
            if name != label:
                candidate = f"{label}:{name}"
                if candidate not in used_labels:
                    return candidate

    # Fallback: append ordinal
    idx = 1
    while f"{label} #{idx}" in used_labels:
        idx += 1
    return f"{label} #{idx}"


def _compute_cohesion(members: list[str], graph: nx.Graph) -> float:
    """Compute internal edge density for the community."""
    n = len(members)
    if n < 2:
        return 0.0

    member_set = set(members)
    internal_edges = 0
    for u, v in graph.edges():
        if u in member_set and v in member_set:
            internal_edges += 1

    max_possible = n * (n - 1) / 2
    if max_possible == 0:
        return 0.0

    return internal_edges / max_possible


def _primary_language(members: list[str], kg: KnowledgeGraph) -> str:
    """Determine the most common language among community members."""
    langs = []
    for sym in kg.get_symbols():
        if sym["id"] in members:
            lang = sym.get("language", "")
            if lang:
                langs.append(lang)

    if not langs:
        return ""

    return Counter(langs).most_common(1)[0][0]
