"""Parse .sln files (custom text format, not XML)."""

from __future__ import annotations

import re
from dataclasses import dataclass


@dataclass
class SolutionProject:
    """A project entry from a .sln file."""
    type_guid: str
    name: str
    path: str
    project_guid: str


# Regex to match Project lines in .sln files
# Project("{TYPE-GUID}") = "Name", "Path\To\Project.csproj", "{PROJECT-GUID}"
_PROJECT_RE = re.compile(
    r'^Project\(\"\{([^}]+)\}\"\)\s*=\s*\"([^\"]+)\"\s*,\s*\"([^\"]+)\"\s*,\s*\"\{([^}]+)\}\"',
    re.MULTILINE,
)

# Known project type GUIDs
_CSHARP_GUID = "FAE04EC0-301F-11D3-BF4B-00C04F79EFBC"
_VBNET_GUID = "F184B08F-C81C-45F6-A57F-5ABD9991F28F"
_SOLUTION_FOLDER_GUID = "2150E333-8FDC-42A3-9474-1A3956D46DE8"


def parse_solution(sln_path: str) -> list[SolutionProject]:
    """Parse a .sln file and return project entries.

    Excludes solution folders (virtual projects for organising).
    """
    try:
        with open(sln_path, "r", encoding="utf-8-sig") as f:
            content = f.read()
    except OSError:
        return []

    projects = []
    for match in _PROJECT_RE.finditer(content):
        type_guid = match.group(1).upper()
        name = match.group(2)
        path = match.group(3)
        project_guid = match.group(4).upper()

        # Skip solution folders
        if type_guid == _SOLUTION_FOLDER_GUID:
            continue

        # Normalise path separators
        path = path.replace("\\", "/")

        projects.append(SolutionProject(
            type_guid=type_guid,
            name=name,
            path=path,
            project_guid=project_guid,
        ))

    return projects
