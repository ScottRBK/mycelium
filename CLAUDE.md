# Mycelium - Developer Notes

## Project Overview

Mycelium is a CLI static analysis tool that produces a JSON structural map of a source code repository. It uses tree-sitter for AST parsing and NetworkX for graph operations.

## Quick Start

```bash
uv sync                                      # Install dependencies
uv run pytest                                 # Run all tests (332 pass)
uv run mycelium-map analyze <path>            # Analyse a repo
uv run mycelium-map analyze <path> --verbose  # With phase timing breakdown
uv run mycelium-map analyze <path> --quiet    # No output except errors
```

## Architecture

Six-phase pipeline, all phases run sequentially:

1. **Structure** (`phases/structure.py`) - Walk file tree, build FileNode/FolderNode graph
2. **Parsing** (`phases/parsing.py`) - Tree-sitter AST parse, extract symbols into SymbolTable + KnowledgeGraph + NamespaceIndex
3. **Imports** (`phases/imports.py`) - Multi-language import resolution: C#/VB.NET (NamespaceIndex), Python (dotted paths), TS/JS (relative + extension probing), Java (path + basename fallback), Go (go.mod + dir index), Rust (crate/super/self + progressive shortening), C/C++ (user includes); plus .NET project/package references
4. **Calls** (`phases/calls.py`) - Build call graph with tiered confidence (A: import/DI/impl-resolved 0.85-0.9, B: same-file 0.85, C: fuzzy 0.5/0.3), interface-to-implementation resolution
5. **Communities** (`phases/communities.py`) - Louvain clustering with auto-tuning resolution, recursive splitting for oversized communities, disambiguated labels
6. **Processes** (`phases/processes.py`) - Multi-branch BFS from scored entry points (test files excluded, depth-bonus scoring), deduplication

## Key Modules

- `config.py` - All dataclasses (Symbol, FileNode, CallEdge, etc.) and enums (SymbolType, Visibility)
- `graph/knowledge_graph.py` - NetworkX DiGraph wrapper with typed accessors
- `graph/symbol_table.py` - Dual HashMap (file index + global index) for symbol lookups
- `graph/namespace_index.py` - Namespace-to-file index for namespace-aware import resolution
- `graph/scoring.py` - Entry point scoring formula with framework type exclusions
- `output.py` - JSON serialisation
- `pipeline.py` - Phase orchestrator with timing and progress callbacks
- `languages/__init__.py` - Extension-to-analyser registry (lazy init)

## Language Analysers

Each analyser in `languages/` implements: `get_language()`, `extract_symbols()`, `extract_imports()`, `extract_calls()`, `builtin_exclusions()`.

| File | Language(s) | Notes |
|------|-------------|-------|
| `csharp.py` | C# | Namespace declarations, using directives, DI parameter type extraction |
| `vbnet.py` | VB.NET | Full analyser; needs `tree-sitter-vb-dotnet` (requires gcc to build) |
| `typescript.py` | TS, TSX, JS, JSX | Shared analyser for TypeScript/JavaScript family |
| `python_lang.py` | Python | Named to avoid stdlib collision |
| `java.py` | Java | Visibility from modifiers node |
| `go.py` | Go | Export by capitalisation convention |
| `rust.py` | Rust | Visibility from visibility_modifier, recurses into impl/mod |
| `c_cpp.py` | C, C++ | Shared mixin, recurses into preproc_ifdef, handles pointer_declarator |

## .NET Support

- `dotnet/solution.py` - .sln text format parser
- `dotnet/project.py` - .csproj/.vbproj XML parser (PropertyGroup, ProjectReference, PackageReference)
- `dotnet/assembly.py` - Namespace-to-project mapping

## Testing

Tests are in `tests/` with fixtures in `tests/fixtures/`:
- `test_pipeline.py` - Core infrastructure (13 tests)
- `test_structure.py` - File tree phase (8 tests)
- `test_parsing.py` - Parsing phase with C# and VB.NET (27 tests, 8 skipped without VB grammar)
- `test_imports.py` - Import resolution for all languages (67 tests)
- `test_calls.py` - Call graph (18 tests)
- `test_communities.py` - Community detection (11 tests)
- `test_processes.py` - Execution flow tracing (18 tests)
- `test_languages.py` - All language analysers + E2E per language (160 tests, 16 skipped without VB grammar)
- `test_namespace_index.py` - NamespaceIndex (6 tests)

## Known Limitations

- VB.NET grammar requires `gcc` to build from source (`pip install git+https://github.com/CodeAnt-AI/tree-sitter-vb-dotnet.git`); analyser gracefully skips .vb files when unavailable
- No incremental analysis (full scan every run)
- No `tsconfig.json` path alias resolution yet
