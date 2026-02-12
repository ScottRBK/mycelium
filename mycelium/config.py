"""Core data types and configuration for Mycelium analysis."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class SymbolType(str, Enum):
    CLASS = "Class"
    FUNCTION = "Function"
    METHOD = "Method"
    INTERFACE = "Interface"
    STRUCT = "Struct"
    ENUM = "Enum"
    NAMESPACE = "Namespace"
    PROPERTY = "Property"
    CONSTRUCTOR = "Constructor"
    MODULE = "Module"
    RECORD = "Record"
    DELEGATE = "Delegate"
    TYPE_ALIAS = "TypeAlias"
    CONSTANT = "Constant"
    VARIABLE = "Variable"
    TRAIT = "Trait"
    IMPL = "Impl"
    MACRO = "Macro"
    TEMPLATE = "Template"
    TYPEDEF = "Typedef"
    ANNOTATION = "Annotation"
    STATIC = "Static"


class Visibility(str, Enum):
    PUBLIC = "public"
    PRIVATE = "private"
    INTERNAL = "internal"
    PROTECTED = "protected"
    FRIEND = "friend"
    UNKNOWN = "unknown"


@dataclass
class FileNode:
    path: str
    language: str | None = None
    size: int = 0
    lines: int = 0


@dataclass
class FolderNode:
    path: str
    file_count: int = 0


@dataclass
class Symbol:
    id: str
    name: str
    type: SymbolType
    file: str
    line: int
    visibility: Visibility = Visibility.UNKNOWN
    exported: bool = False
    parent: str | None = None
    language: str | None = None
    byte_range: tuple[int, int] | None = None
    parameter_types: list[tuple[str, str]] | None = None


@dataclass
class ImportStatement:
    """Raw import statement extracted from source."""
    file: str
    statement: str
    target_name: str
    line: int


@dataclass
class RawCall:
    """Raw call site extracted from source."""
    caller_file: str
    caller_name: str
    callee_name: str
    line: int
    qualifier: str | None = None


@dataclass
class CallEdge:
    from_symbol: str
    to_symbol: str
    confidence: float
    tier: str
    reason: str
    line: int


@dataclass
class ImportEdge:
    from_file: str
    to_file: str
    statement: str


@dataclass
class ProjectReference:
    from_project: str
    to_project: str
    ref_type: str = "ProjectReference"


@dataclass
class PackageReference:
    project: str
    package: str
    version: str = ""


@dataclass
class Community:
    id: str
    label: str
    members: list[str] = field(default_factory=list)
    cohesion: float = 0.0
    primary_language: str = ""


@dataclass
class Process:
    id: str
    entry: str
    terminal: str
    steps: list[str] = field(default_factory=list)
    type: str = "intra_community"
    total_confidence: float = 0.0


@dataclass
class AnalysisConfig:
    repo_path: str = ""
    output_path: str | None = None
    languages: list[str] | None = None
    resolution: float = 1.0
    max_processes: int = 75
    max_depth: int = 10
    max_branching: int = 4
    min_steps: int = 2
    exclude_patterns: list[str] = field(default_factory=list)
    verbose: bool = False
    quiet: bool = False
    max_file_size: int = 1_000_000  # 1MB
    max_community_size: int = 50


@dataclass
class AnalysisResult:
    version: str = "1.0"
    metadata: dict[str, Any] = field(default_factory=dict)
    stats: dict[str, Any] = field(default_factory=dict)
    structure: dict[str, list[dict]] = field(default_factory=lambda: {"files": [], "folders": []})
    symbols: list[dict] = field(default_factory=list)
    imports: dict[str, list[dict]] = field(default_factory=lambda: {
        "file_imports": [],
        "project_references": [],
        "package_references": [],
    })
    calls: list[dict] = field(default_factory=list)
    communities: list[dict] = field(default_factory=list)
    processes: list[dict] = field(default_factory=list)
