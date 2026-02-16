# Mycelium - Developer Notes

## Project Overview

Mycelium is a CLI static analysis tool that produces a JSON structural map of a source code repository. The engine is written in Rust using tree-sitter for AST parsing and petgraph for graph operations. Python bindings are provided via PyO3, distributed as binary wheels on PyPI.

## Quick Start

```bash
# Rust development
cargo test --workspace                        # Run all Rust tests
cargo run -p mycelium-cli -- analyze <path>   # Run Rust CLI directly
cargo clippy --workspace                      # Lint check

# Python bindings
pip install maturin
maturin develop --release                     # Build + install locally
pytest tests/test_bindings.py -v              # Run binding smoke tests
mycelium-map analyze <path> --verbose         # Python CLI (calls Rust engine)
```

## Architecture

Rust engine with a six-phase sequential pipeline:

1. **Structure** (`phases/structure.rs`) - Walk file tree, build FileNode/FolderNode graph
2. **Parsing** (`phases/parsing.rs`) - Tree-sitter AST parse, extract symbols into SymbolTable + KnowledgeGraph + NamespaceIndex
3. **Imports** (`phases/imports.rs`) - Multi-language import resolution: C#/VB.NET (NamespaceIndex), Python (dotted paths), TS/JS (relative + extension probing), Java (path + basename fallback), Go (go.mod + dir index), Rust (crate/super/self + progressive shortening), C/C++ (user includes); plus .NET project/package references
4. **Calls** (`phases/calls.rs`) - Build call graph with tiered confidence (A: import/DI/impl-resolved 0.85-0.9, B: same-file 0.85, C: fuzzy 0.5/0.3), interface-to-implementation resolution
5. **Communities** (`phases/communities.rs`) - Louvain clustering with auto-tuning resolution, recursive splitting for oversized communities, disambiguated labels
6. **Processes** (`phases/processes.rs`) - Multi-branch BFS from scored entry points (test files excluded, depth-bonus scoring), deduplication

## Crate Structure

```
crates/
  mycelium-core/    <- Pure Rust library (config, graph, phases, languages, output, pipeline)
  mycelium-cli/     <- clap CLI binary (mycelium-map)
  mycelium-python/  <- PyO3 bindings (_mycelium_rust)
```

### Python Layer (thin wrapper)

```
mycelium/
  __init__.py       <- Re-exports from _mycelium_rust (analyze, version, PyAnalysisConfig)
  cli.py            <- click CLI wrapper calling Rust analyze()
  py.typed          <- PEP 561 marker
```

## Key Rust Modules

- `config.rs` - All structs (Symbol, FileNode, CallEdge, etc.) and enums (SymbolType, Visibility)
- `graph/knowledge_graph.rs` - petgraph DiGraph wrapper with typed accessors
- `graph/symbol_table.rs` - Dual HashMap (file index + global index) for symbol lookups
- `graph/namespace_index.rs` - Namespace-to-file index for namespace-aware import resolution
- `graph/scoring.rs` - Entry point scoring formula with framework type exclusions
- `output.rs` - JSON serialisation
- `pipeline.rs` - Phase orchestrator with timing and progress callbacks
- `languages/mod.rs` - `LanguageAnalyser` trait + `AnalyserRegistry` for extension dispatch

## Language Analysers

Each analyser in `languages/` implements the `LanguageAnalyser` trait: `language_name()`, `extensions()`, `parse()`, `extract_symbols()`, `extract_imports()`, `extract_calls()`, `builtin_exclusions()`.

| File | Language(s) | Notes |
|------|-------------|-------|
| `csharp.rs` | C# | Namespace declarations, using directives, DI parameter type extraction |
| `vbnet.rs` | VB.NET | Stub (`is_available() -> false`); grammar not yet vendored |
| `typescript.rs` | TS, TSX, JS, JSX | Shared analyser for TypeScript/JavaScript family |
| `python.rs` | Python | |
| `java.rs` | Java | Visibility from modifiers node |
| `go.rs` | Go | Export by capitalisation convention |
| `rust.rs` | Rust | Visibility from visibility_modifier, recurses into impl/mod |
| `c_cpp.rs` | C, C++ | Shared, recurses into preproc_ifdef, handles pointer_declarator |

## Build System

- **maturin** builds PyO3 bindings into wheels
- `pyproject.toml` configures maturin with `manifest-path = "crates/mycelium-python/Cargo.toml"`
- Dependencies: only `click>=8.1` and `rich>=13.0` (all analysis is Rust-native)

## Testing

- **Rust tests**: `cargo test --workspace` (unit tests across all crates)
- **Python binding tests**: `tests/test_bindings.py` (smoke tests for PyO3 interface)
- **Fixtures**: `tests/fixtures/` (13 directories, shared between Rust and Python tests)

## CI/CD

- `.github/workflows/ci.yml` - Rust tests + clippy + fmt, Python binding tests
- `.github/workflows/release.yml` - Multi-platform wheel builds (Linux/macOS/Windows) via maturin-action, publish to PyPI

## Known Limitations

- VB.NET grammar not vendored; analyser returns `is_available() -> false`
- No incremental analysis (full scan every run)
- No `tsconfig.json` path alias resolution yet
