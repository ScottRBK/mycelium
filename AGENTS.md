# Mycelium

A static analysis tool that maps the connections in a source code repository. A Rust engine runs
six analysis phases and produces a JSON map of files, symbols, imports, calls, communities, and
execution flows. Both the native CLI and the Python package use the same engine.

## Architecture

The diagrams show the current Rust implementation. Boxes are structs unless marked `module`,
`trait`, or `enum`; module operations are free functions. The main fields show their types, and
method signatures show parameter and return types. `..>` means uses, `*--` means owns, and `..|>`
means implements. See the
[Mermaid class diagram syntax](https://mermaid.js.org/syntax/classDiagram.html).

`+` means public and `-` means private. Method receivers (`&self` and `&mut self`) are omitted;
an omitted return type means `()`. `build_result()` lists parameter types in source order without
their names to fit within 100 columns. Enum variants omit their payloads. The type key below the
first diagram defines abbreviations used in both diagrams.

Selected calls are labelled `caller() calls callee()`, with the arrow pointing to the callee's
box. Mermaid class diagrams connect boxes; the labels identify the individual methods or free
functions involved. These calls have been checked against the source.

```mermaid
classDiagram
    direction TB

    class pipeline {
        <<module>>
        +run_pipeline(config: &AnalysisConfig, callback: Option~ProgressCallback~) PipelineResult
    }

    class AnalysisConfig {
        +repo_path: String
        +output_path: Option~String~
        +languages: Option~Vec~String~~
        +exclude_patterns: Vec~String~
        +resolution: f64
        +max_community_size: usize
        +max_processes: usize
        +max_depth: usize
        +max_branching: usize
        +min_steps: usize
        +verbose: bool
        +quiet: bool
        +max_file_size: u64
    }

    class KnowledgeGraph {
        -class_diagram: ClassDiagram
        -graph: Graph
        -id_index: Map~NodeIndex~
        +new() KnowledgeGraph
        +add_file(node: &FileNode)
        +add_symbol(symbol: &Symbol)
        +add_import(edge: &ImportEdge)
        +add_call(edge: &CallEdge)
        +add_community(community: &Community)
        +add_process(process: &Process)
        +get_symbols() Vec~SymbolInfo~
        +get_call_edges() Vec~CallRecord~
        +get_callers(symbol_id: &str) Vec~CallInfo~
        +get_callees(symbol_id: &str) Vec~CallInfo~
        +inner_graph() &Graph
    }

    class SymbolTable {
        -file_index: Map~Map~String~~
        -global_index: Map~Vec~SymbolDefinition~~
        +new() SymbolTable
        +add(symbol: &Symbol)
        +lookup_exact(file_path: &str, name: &str) Option~&str~
        +lookup_fuzzy(name: &str) &[SymbolDefinition]
    }

    class NamespaceIndex {
        -ns_to_files: Map~Vec~String~~
        -file_to_ns: Map~Vec~String~~
        -file_imports: Map~Vec~String~~
        +new() NamespaceIndex
        +register(namespace: &str, file_path: &str)
        +get_files_for_namespace(namespace: &str) &[String]
        +register_file_import(file_path: &str, namespace: &str)
        +get_imported_namespaces(file_path: &str) &[String]
    }

    class NodeData {
        <<enum>>
        File
        Folder
        Symbol
        Community
        Process
        Package
        Project
    }

    class EdgeData {
        <<enum>>
        Defines
        Imports
        Calls
        ProjectReference
        PackageReference
        MemberOf
        Step
        Contains
    }

    class output {
        <<module>>
        +build_result(&AnalysisConfig, &KnowledgeGraph, &SymbolTable, &Map~f64~, f64) AnalysisResult
        +write_output(result: &AnalysisResult, output_path: &str) IoResult
    }

    class AnalysisResult {
        +class_diagram: Option~ClassDiagram~
        +version: String
        +metadata: Map~Value~
        +stats: Map~Value~
        +structure: StructureOutput
        +symbols: Vec~SymbolOutput~
        +imports: ImportsOutput
        +calls: Vec~CallOutput~
        +communities: Vec~CommunityOutput~
        +processes: Vec~ProcessOutput~
    }

    pipeline ..> AnalysisConfig : reads
    pipeline ..> KnowledgeGraph : run_pipeline() calls new()
    pipeline ..> SymbolTable : run_pipeline() calls new()
    pipeline ..> NamespaceIndex : run_pipeline() calls new()
    pipeline ..> output : run_pipeline() calls build_result()
    pipeline ..> AnalysisResult : returns
    KnowledgeGraph *-- NodeData : stores nodes
    KnowledgeGraph *-- EdgeData : stores edges
    output ..> KnowledgeGraph : build_result() calls get_symbols()
    output ..> KnowledgeGraph : build_result() calls get_call_edges()
    output ..> AnalysisResult : builds and serializes
```

Type key: Mermaid writes generic brackets as `~T~`. The abbreviations below keep signatures
readable and avoid Mermaid's limitation on comma-separated generic parameters. Only
`ProgressCallback` is an existing project type alias; `Value` is the JSON library's type.

| Diagram type | Rust type |
|---|---|
| `Map<T>` | `HashMap<String, T>` |
| `Graph` | `DiGraph<NodeData, EdgeData>` |
| `PipelineResult` | `Result<AnalysisResult, Box<dyn std::error::Error>>` |
| `IoResult` | `std::io::Result<()>` |
| `CallRecord` | `(String, String, f64, String, String, usize)` |
| `ProgressCallback` | `Box<dyn FnMut(&str, &str)>` |
| `Value` | `serde_json::Value` |

`CallRecord` holds `(from, to, confidence, tier, reason, line)`. The callback receives the phase
name and label. `build_result()` receives `(config, kg, st, timings, total_ms)` in that order.

`run_pipeline()` creates one graph and two lookup indexes for each run. The phases execute in
order, enriching the shared graph. `SymbolTable` finds symbols by file/name or across the whole
repository; `NamespaceIndex` connects namespace declarations and imports to files. These indexes
support resolution without repeatedly searching the entire graph.

The graph stores `NodeData` and `EdgeData` enum values. Input records such as `Symbol`, `CallEdge`,
`Community`, and `Process` are converted into graph nodes and edges. Community membership and
process steps are stored as edges. `output::build_result()` converts the graph into the separate
output structs in `AnalysisResult`.

`run_pipeline()` returns data; the CLI writes the output file. The Python binding converts the
result to a Python dictionary. Calling `mycelium.analyze()` alone does not write a JSON file.

### Mermaid Export

The parsing phase also extracts declaration facts from the same syntax tree, using language-specific
visitors in `languages/declarations/`. `KnowledgeGraph` carries the `ClassDiagram` to the additive
`AnalysisResult.class_diagram` field. Existing call-resolution symbols remain separate from this
richer model. `mermaid::export_mermaid()` resolves owners, filters scope, and writes deterministic
Markdown. Both CLIs expose `export`; the Python binding exposes `export_mermaid(result)`.

```mermaid
classDiagram
    direction TB
    class ClassDiagram {
        +classes: Vec~Class~
        +warnings: Vec~String~
        +test_detection: Option~TestDetection~
    }
    class Class {
        +id: String
        +name: String
        +kind: String
        +file: String
        +line: usize
        +test: Option~TestEvidence~
        +members: Vec~Member~
        +bases: Vec~Base~
    }
    class Member {
        +test: Option~TestEvidence~
        +file: String
        +name: String
        +kind: String
        +parameters: Vec~Parameter~
        +value_type: Option~String~
    }
    class MermaidOptions {
        +path: String
        +max_classes: usize
        +tests: TestMode
        +test_paths: Vec~String~
        +keep_paths: Vec~String~
        +explain_tests: bool
    }
    class mermaid {
        <<module>>
        +export_mermaid(result: &AnalysisResult, options: &MermaidOptions) ExportResult
    }
    ClassDiagram *-- Class : contains declaration records
    Class *-- Member : contains member records
    mermaid ..> ClassDiagram : reads
    mermaid ..> MermaidOptions : reads
```

The map stores versioned test evidence without removing analysis facts. Export defaults to excluding
Rust `#[test]`/literal `cfg(test)`, Go `_test.go`, and supported Python, .NET, Java and JS/TS
markers. Framework bindings are checked conservatively, including repository declaration conflicts.
Detector version 2 adds framework rules; version 1 maps retain their saved Rust/Go evidence. All
languages support explicit `--test-path` and overriding `--keep-path`; `--tests include` preserves
the complete view. Selection uses raw source occurrences before impl merging, with identities kept
for ambiguity and stable box IDs. `--explain-tests` adds source-level reasons. C/C++ use explicit
paths. Custom/global framework discovery and separate test diagrams remain deferred. See the export
guide for exact rules.

`ExportResult` abbreviates `Result<String, ExportError>`. An older map without declarations
reports that analysis must be rerun. Views bound class/member counts and retain resolved
relationships in a complete list. Calls remain heuristic; fields do not imply exclusive ownership.
See [Mermaid export](docs/mermaid-export.md) for coverage and checkpoint validation.

### Language Analysers

Structure, parsing, imports, and calls each create an `AnalyserRegistry`. It selects a boxed
`LanguageAnalyser` implementation by file extension. Phase functions own Tree-sitter parsing;
analysers select the grammar and extract language-specific records from the syntax tree.

```mermaid
classDiagram
    direction TB

    class AnalyserRegistry {
        -analysers: Vec~Box~dyn LanguageAnalyser~~
        -extension_map: Map~usize~
        +new() AnalyserRegistry
        +get_by_extension(ext: &str) Option~&dyn LanguageAnalyser~
        +language_for_extension(ext: &str) Option~&str~
        +extensions() Vec~&str~
    }

    class LanguageAnalyser {
        <<trait>>
        +extensions() &[&str]
        +language_name() &str
        +get_language() Language
        +get_language_for_ext(ext: &str) Language
        +extract_symbols(tree: &Tree, source: &[u8], file_path: &str) Vec~Symbol~
        +extract_imports(tree: &Tree, source: &[u8], file_path: &str) Vec~ImportStatement~
        +extract_calls(tree: &Tree, source: &[u8], file_path: &str) Vec~RawCall~
        +builtin_exclusions() &HashSet~String~
        +is_available() bool
    }

    class CSharpAnalyser
    class VbNetAnalyser
    class TypeScriptAnalyser
    class PythonAnalyser
    class JavaAnalyser
    class GoAnalyser
    class RustAnalyser
    class CAnalyser
    class CppAnalyser

    AnalyserRegistry *-- LanguageAnalyser
    AnalyserRegistry ..> LanguageAnalyser : language_for_extension() calls language_name()
    CSharpAnalyser ..|> LanguageAnalyser
    VbNetAnalyser ..|> LanguageAnalyser
    TypeScriptAnalyser ..|> LanguageAnalyser
    PythonAnalyser ..|> LanguageAnalyser
    JavaAnalyser ..|> LanguageAnalyser
    GoAnalyser ..|> LanguageAnalyser
    RustAnalyser ..|> LanguageAnalyser
    CAnalyser ..|> LanguageAnalyser
    CppAnalyser ..|> LanguageAnalyser
```

Each analyser explicitly implements the Rust trait. The trait requires `Send + Sync`, but the
current pipeline and file-processing loops run sequentially.
There is no analyser `parse()` method: parsing, imports, and calls each parse the source they need.

## Analysis Pipeline

Phase implementations live in `crates/mycelium-core/src/phases/`.

| Phase | Module | Responsibility |
|---|---|---|
| 1. Structure | `structure.rs` | Walk files, apply exclusions and size limits, record languages. |
| 2. Parsing | `parsing.rs` | Extract symbols; populate the graph and both lookup indexes. |
| 3. Imports | `imports.rs` | Resolve file imports and .NET project/package references. |
| 4. Calls | `calls.rs` | Resolve call sites using imports, types, local names, and global names. |
| 5. Communities | `communities.rs` | Cluster the weighted call graph with Louvain. |
| 6. Processes | `processes.rs` | Trace call paths from entry points with breadth-first search. |

Call resolution records both a confidence and a reason:

| Tier | Confidence | Resolution |
|---|---|---|
| A | 0.9 | Imported symbol or dependency-injection parameter type. |
| A | 0.85 | Interface-to-implementation match. |
| B | 0.85 | Same-file symbol; qualified C++ methods first check their explicit owner. |
| C | 0.5 / 0.3 | Unique / ambiguous global name match. |

These are static heuristics. A call edge or execution flow describes a possible connection, not
proof of runtime behaviour. Community detection uses the local Rust Louvain implementation, with
resolution tuning and splitting of oversized groups. Process tracing limits depth and branching,
deduplicates traces, and stores the product of call-edge confidences as `total_confidence`.

## Supported Languages

All analyser source files below are in `crates/mycelium-core/src/languages/`.

| Analyser | Languages | Registered extensions | Source |
|---|---|---|---|
| `CSharpAnalyser` | C# | `.cs` | `csharp.rs` |
| `VbNetAnalyser` | VB.NET | `.vb` | `vbnet.rs` |
| `TypeScriptAnalyser` | TypeScript, JavaScript | `.ts`, `.tsx`, `.js`, `.jsx` | `typescript.rs` |
| `PythonAnalyser` | Python | `.py` | `python.rs` |
| `JavaAnalyser` | Java | `.java` | `java.rs` |
| `GoAnalyser` | Go | `.go` | `go_lang.rs` |
| `RustAnalyser` | Rust | `.rs` | `rust_lang.rs` |
| `CAnalyser` | C | `.c`, `.h` | `c_cpp.rs` |
| `CppAnalyser` | C++ | `.cpp`, `.cxx`, `.cc`, `.hpp`, `.hxx`, `.hh` | `c_cpp.rs` |

TypeScript and JavaScript share an analyser, which reports `TypeScript` as its language name and
selects the grammar by extension. `.mjs` and `.cjs` are handled in its grammar selector but are
not registered, so they are not currently parsed by the pipeline despite the README listing them.
The VB.NET grammar is vendored in `vendor/tree-sitter-vb-dotnet/` for Tree-sitter ABI compatibility.

## Project Structure

```text
crates/
  mycelium-core/src/
    config.rs           Shared records, configuration, and output structs
    declarations.rs     Typed declaration model stored in analysis maps
    mermaid.rs          Deterministic class diagram export and display limits
    pipeline.rs         Six-phase orchestration, timing, and progress callbacks
    graph/              KnowledgeGraph, SymbolTable, NamespaceIndex, entry-point scoring
    phases/             Analysis stages
    languages/          LanguageAnalyser trait, registry, and implementations
    dotnet/             Solution, project, and assembly reference support
    output.rs           Build AnalysisResult and write JSON
  mycelium-core/tests/  Rust integration tests
  mycelium-cli/         Native CLI using clap
  mycelium-python/      PyO3 bindings: analyze(), version(), PyAnalysisConfig
mycelium/               Python exports and Click/Rich CLI
tests/
  fixtures/             Source repositories shared by Rust and Python tests
  test_bindings.py      Python binding smoke tests
vendor/                 Vendored VB.NET grammar
```

Keep analysis logic in `mycelium-core`; both command-line interfaces should remain thin wrappers.
`ARCHITECTURE.md` is an earlier Python design proposal. Use the Rust source and this guide for
the implemented architecture, and consult that proposal for historical intent.

## Development

```bash
# Native CLI; writes a structural map to the chosen path
cargo run -p mycelium-cli -- analyze tests/fixtures/csharp_simple -o /tmp/mycelium-map.json

# Python bindings: create a development environment, then build the Rust extension
python3 -m venv .venv
source .venv/bin/activate
python -m pip install maturin pytest click rich
maturin develop --release

# Python CLI (uses the same Rust engine)
mycelium-map analyze tests/fixtures/csharp_simple -o /tmp/mycelium-map.json --verbose
```

Rebuild with `maturin develop --release` after Rust changes before testing the Python interface.
Python requires 3.12 or newer. Maturin builds the extension as `mycelium._mycelium_rust`.
The extension uses `abi3-py312`: one wheel per platform supports standard CPython 3.12+.

## Test Philosophy

Validate observable analysis results using small source fixtures or deliberately constructed
graphs. For features and bug fixes, use TDD: reproduce the missing behaviour with a failing test,
implement a complete vertical slice, and run the relevant regression tests. A parser fix should
check the extracted records; a resolution fix should also check the resulting graph edges.

| Suite | Location | Purpose | Runs in CI |
|---|---|---|---|
| Rust unit | Inline `#[cfg(test)]` modules | Data types, indexes, and algorithm helpers. | Yes |
| Rust integration | `crates/mycelium-core/tests/` | Fixtures, phases, pipeline, JSON. | Yes |
| Mermaid checkpoints | `tests/checkpoints/` | Commit-pinned external source to Markdown. | Local |
| Python bindings | `tests/test_bindings.py` | Extension API, config, progress, and exports. | Yes |
| Wheel install | `tests/check_wheel.py` | Installed bindings and uvx without Rust. | Yes |

```bash
# All Rust tests, including fixture-based full-pipeline checks
cargo test --workspace

# Focused examples while iterating
cargo test -p mycelium-core --test test_pipeline
cargo test -p mycelium-core --test test_calls

# CI lint and formatting checks
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check

# After building the extension in the active virtual environment
python -m pytest tests/test_bindings.py -v

# Build once, then run with each Python version; requires uv on PATH
maturin build --release -i python3.12 --out /tmp/mycelium-wheels
python3.13 tests/check_wheel.py /tmp/mycelium-wheels
```

Keep code and Markdown lines within 100 columns. Update these diagrams when the main types or
their relationships change. When modifying third-party library usage, consult the Context7 skill.

## CI/CD

- `.github/workflows/ci.yml`: pushes and pull requests to `master` run Rust tests, Clippy,
  formatting checks, and build one wheel on Ubuntu with Python 3.12. `test-wheels.yml` installs
  that same artifact on Python 3.12, 3.13, and 3.14, outside the checkout and without Rust on PATH.
- `.github/workflows/release.yml`: `v*` tags build Linux and macOS wheels for x86_64 and aarch64,
  Windows wheels for x86_64, and a source distribution. Publication to PyPI as `mycelium-map`
  requires each platform's wheel to pass `test-wheels.yml` on all three Python versions.

## Known Limitations

- Every run scans the repository afresh; there is no incremental analysis or shared syntax cache.
- TypeScript import resolution handles relative paths; `tsconfig.json` path aliases are unsupported.
- Static call resolution is heuristic, especially when several symbols share the same name.
