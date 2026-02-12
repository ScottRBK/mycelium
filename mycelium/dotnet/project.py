"""Parse .csproj/.vbproj files (XML with MSBuild schema)."""

from __future__ import annotations

import xml.etree.ElementTree as ET
from dataclasses import dataclass, field


@dataclass
class ProjectInfo:
    """Parsed information from a .csproj/.vbproj file."""
    path: str
    root_namespace: str = ""
    assembly_name: str = ""
    target_framework: str = ""
    project_references: list[str] = field(default_factory=list)
    package_references: list[tuple[str, str]] = field(default_factory=list)  # (name, version)


def parse_project(project_path: str) -> ProjectInfo:
    """Parse a .csproj/.vbproj file and return project info.

    Handles both SDK-style and legacy project formats.
    """
    info = ProjectInfo(path=project_path)

    try:
        tree = ET.parse(project_path)
        root = tree.getroot()
    except (ET.ParseError, OSError):
        return info

    # Strip namespace from tags for easier querying
    ns = ""
    if root.tag.startswith("{"):
        ns = root.tag.split("}")[0] + "}"

    # PropertyGroup elements
    for pg in root.iter(f"{ns}PropertyGroup"):
        rns = pg.find(f"{ns}RootNamespace")
        if rns is not None and rns.text:
            info.root_namespace = rns.text.strip()

        asm = pg.find(f"{ns}AssemblyName")
        if asm is not None and asm.text:
            info.assembly_name = asm.text.strip()

        tf = pg.find(f"{ns}TargetFramework")
        if tf is not None and tf.text:
            info.target_framework = tf.text.strip()

        tfs = pg.find(f"{ns}TargetFrameworks")
        if tfs is not None and tfs.text and not info.target_framework:
            # Use the first framework listed
            info.target_framework = tfs.text.strip().split(";")[0]

    # ProjectReference elements
    for pr in root.iter(f"{ns}ProjectReference"):
        include = pr.get("Include", "")
        if include:
            # Normalise path separators
            include = include.replace("\\", "/")
            info.project_references.append(include)

    # PackageReference elements
    for pkg in root.iter(f"{ns}PackageReference"):
        name = pkg.get("Include", "")
        version = pkg.get("Version", "")
        if not version:
            # Version might be a child element
            ver_elem = pkg.find(f"{ns}Version")
            if ver_elem is not None and ver_elem.text:
                version = ver_elem.text.strip()
        if name:
            info.package_references.append((name, version))

    # Defaults: if no RootNamespace/AssemblyName, derive from file name
    import os
    project_name = os.path.splitext(os.path.basename(project_path))[0]
    if not info.root_namespace:
        info.root_namespace = project_name
    if not info.assembly_name:
        info.assembly_name = project_name

    return info
