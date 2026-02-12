# Mycelium Architecture

Mycelium is a standalone CLI tool that performs deterministic static analysis on source code repositories, producing a single JSON file containing a complete structural map: file tree, symbols, imports, call graph with confidence scores, community clusters, and execution flows.

The name reflects its purpose -- like fungal mycelium, it maps the hidden network of connections beneath the surface of a codebase.

## Goals

- **Deterministic pre-processing for LLM consumption.** Mycelium produces a structured map that shifts an LLM's role from "explorer" to "interpreter", replacing blind code exploration with a pre-computed structural overview.
- **Single repo analysis with cross-project awareness.** Analyse one repo at a time, but parse `.sln` and `.csproj` files to capture project-to-project references. This allows consumers to stitch a cross-repo graph later without Mycelium needing multi-repo orchestration.
- **Language-agnostic pipeline, language-specific parsers.** The six-phase pipeline is the same regardless of language. Language-specific logic is isolated to parser modules and import/call resolution strategies.
- **Zero infrastructure.** No databases, no servers, no containers. Python CLI in, JSON file out.

## Non-Goals

- **Not a knowledge base.** Mycelium produces structural facts. Semantic meaning, business context, and architectural judgments are the LLM's job.
- **Not an integration with any specific tool.** Mycelium produces files. Other tools consume them.
- **Not incremental (v1).** Full analysis on every run. Tree-sitter parses at milliseconds per file, so even large repos should complete in seconds. The architecture does not preclude adding caching later.
- **Not a linter or security scanner.** We extract structure, not quality judgments.

## Supported Languages

| Language | Tree-sitter Grammar | Import Resolution | Call Resolution | Priority |
|---|---|---|---|---|
| C# | `tree-sitter-c-sharp` | `using` statements + `.csproj` ProjectReference/PackageReference | Namespace-qualified, method invocations | Must-have |
| VB.NET | `tree-sitter-vb-net` | `Imports` statements + `.csproj` ProjectReference/PackageReference | Namespace-qualified, method invocations | Must-have |
| TypeScript | `tree-sitter-typescript` | ES `import`/`require`, `tsconfig.json` path aliases | Import-resolved, same-file, fuzzy | Must-have |
| JavaScript | `tree-sitter-javascript` | ES `import`/`require` | Import-resolved, same-file, fuzzy | Must-have |
| Python | `tree-sitter-python` | `import`/`from...import` | Import-resolved, same-file, fuzzy | Should-have |
| Java | `tree-sitter-java` | `import` statements, package structure | Package-qualified, method invocations | Should-have |
| Go | `tree-sitter-go` | `import` paths | Package-qualified | Should-have |
| Rust | `tree-sitter-rust` | `use`/`mod` statements, `Cargo.toml` | Module-qualified, trait resolution | Should-have |
| C | `tree-sitter-c` | `#include` directives | Header-resolved, same-file, fuzzy | Should-have |
| C++ | `tree-sitter-cpp` | `#include` directives, `using namespace` | Namespace-qualified, header-resolved | Should-have |
| SQL | `tree-sitter-sql` | N/A (no import system) | Procedure/function calls | Nice-to-have |

Language detection is automatic based on file extension. Repos with mixed languages are handled naturally -- each file is parsed with its corresponding grammar, and cross-language calls are resolved where the type system allows it (e.g., C# calling a VB.NET assembly via shared interfaces).

## Architecture

```
mycelium/
├── mycelium/
│   ├── __init__.py
│   ├── pipeline.py             # Orchestrates the 6 phases sequentially
│   ├── config.py               # Analysis configuration, language registry
│   ├── output.py               # JSON serialisation and schema
│   ├── phases/
│   │   ├── __init__.py
│   │   ├── structure.py        # Phase 1: File tree construction
│   │   ├── parsing.py          # Phase 2: Tree-sitter AST → symbol extraction
│   │   ├── imports.py          # Phase 3: Import/dependency resolution
│   │   ├── calls.py            # Phase 4: Call graph with confidence scoring
│   │   ├── communities.py      # Phase 5: Louvain clustering
│   │   └── processes.py        # Phase 6: BFS execution flow detection
│   ├── languages/
│   │   ├── __init__.py         # Language registry and detection
│   │   ├── base.py             # Abstract base: LanguageAnalyser protocol
│   │   ├── csharp.py           # C# symbol extraction, import/call resolution
│   │   ├── vbnet.py            # VB.NET symbol extraction, import/call resolution
│   │   ├── typescript.py       # TypeScript/JavaScript
│   │   ├── python.py           # Python
│   │   ├── java.py             # Java
│   │   ├── go.py               # Go
│   │   ├── rust.py             # Rust
│   │   └── c_cpp.py            # C and C++ (shared header resolution)
│   ├── dotnet/
│   │   ├── __init__.py
│   │   ├── solution.py         # .sln parser → project list
│   │   ├── project.py          # .csproj/.vbproj parser → references, packages
│   │   └── assembly.py         # Assembly/namespace mapping for cross-project resolution
│   └── graph/
│       ├── __init__.py
│       ├── knowledge_graph.py  # In-memory graph (networkx)
│       ├── symbol_table.py     # Dual HashMap: file-scoped + global index
│       └── scoring.py          # Entry point scoring for process detection
└── tests/
    ├── __init__.py
    ├── test_pipeline.py
    ├── test_structure.py
    ├── test_parsing.py
    ├── test_imports.py
    ├── test_calls.py
    ├── test_communities.py
    ├── test_processes.py
    └── fixtures/               # Small test repos per language
        ├── csharp_simple/
        ├── vbnet_simple/
        ├── typescript_simple/
        └── mixed_dotnet/       # .sln with C# + VB.NET projects
```

## The Six-Phase Pipeline

Each phase reads from the in-memory knowledge graph built by previous phases and adds to it. The graph is a `networkx.DiGraph` with typed nodes and edges.

### Phase 1: Structure

**Input:** Repository root path
**Output:** `Folder` and `File` nodes with `CONTAINS` edges

Walk the file tree, skipping ignored paths (`bin/`, `obj/`, `node_modules/`, `.git/`, etc.). Create a node per file and folder. Record file extension, size, and language classification.

For .NET repos, also identify `.sln` and `.csproj`/`.vbproj` files for Phase 3.

### Phase 2: Parsing

**Input:** `File` nodes from Phase 1
**Output:** `Symbol` nodes (Function, Class, Interface, Method, Struct, Enum, Namespace, Property, Constructor, etc.) with `DEFINES` edges from their containing file

For each source file, load the appropriate Tree-sitter grammar and parse to AST. Language-specific extractors walk the AST to identify symbol definitions, recording:
- Name, type (class/function/method/etc.), line number, byte range
- Parent symbol (for methods within classes)
- Visibility (public/private/internal/protected) where detectable
- Export status (is it accessible outside its module?)

Symbols are registered in the **Symbol Table** (dual HashMap):
- **File index:** `Map<file_path, Map<symbol_name, node_id>>` for exact lookups
- **Global index:** `Map<symbol_name, List[SymbolDefinition]>` for fuzzy fallback

### Phase 3: Imports

**Input:** `File` and `Symbol` nodes, `.sln`/`.csproj` files
**Output:** `IMPORTS` edges (file→file), `PROJECT_REFERENCES` edges (project→project), `PACKAGE_REFERENCES` (project→package)

Three levels of resolution:

**Level 1 -- Source file imports:**
Parse `using`/`Imports`/`import`/`#include` statements. Resolve to target files where possible using the file index and language-specific module resolution (e.g., TypeScript path aliases from `tsconfig.json`, Python package structure, C# namespace-to-file mapping).

**Level 2 -- Project references (.csproj/.vbproj):**
Parse `<ProjectReference>` elements to build a project dependency graph. This captures cross-project relationships within a solution -- e.g., a Web project referencing a Core library project.

**Level 3 -- Package references (.csproj/.vbproj):**
Parse `<PackageReference>` elements to record NuGet dependencies. These are external edges (the target isn't in the repo) but are valuable for understanding what third-party libraries the project depends on and for cross-repo stitching later.

Solution files (`.sln`) are parsed to discover which projects belong together and their relative paths.

### Phase 4: Calls

**Input:** `Symbol` nodes, `IMPORTS` edges, Symbol Table
**Output:** `CALLS` edges with confidence scores

Walk the AST of each source file to find call expressions. For each call site, identify the enclosing function/method (the caller) and resolve the callee using a three-tier confidence system:

| Tier | Confidence | Condition |
|---|---|---|
| A | 0.9 | **Import-resolved.** The callee was imported from a known file, and the symbol exists in that file's index. |
| B | 0.85 | **Same-file.** The callee is defined in the same file as the caller. |
| C | 0.5 / 0.3 | **Fuzzy-global.** Found in the global symbol table. 0.5 if unique match, 0.3 if ambiguous (multiple definitions with the same name). |

Each `CALLS` edge records: `from_symbol`, `to_symbol`, `confidence`, `tier`, `reason`, `line`.

Built-in/runtime calls (e.g., `Console.WriteLine`, `string.Format`, `print()`) are filtered out via a per-language exclusion list to reduce noise.

### Phase 5: Communities

**Input:** `CALLS`, `EXTENDS`, `IMPLEMENTS` edges
**Output:** `Community` nodes with `MEMBER_OF` edges

Build an undirected weighted graph from call/inheritance edges. Run the **Louvain algorithm** (`networkx.community` or `community-louvain` package) with configurable resolution (default 1.0).

Each community gets:
- **Auto-generated label** derived from the most common folder path or shared name prefix of its members
- **Cohesion score** -- internal edge density vs total possible internal edges
- **Member count**
- **Primary language** (majority language of member symbols)

Singleton communities (1 member) are discarded.

### Phase 6: Processes

**Input:** Full graph with all edges and communities
**Output:** `Process` nodes with `STEP` edges

Detect execution flows via BFS from scored entry points.

**Entry point scoring:**
```
score = base_score × export_multiplier × name_multiplier × framework_multiplier
```

- `base_score` = callees / (callers + 1) -- nodes that call many things but are called by few are likely entry points
- `export_multiplier` = 2.0 if the symbol is exported/public
- `name_multiplier` = 1.5 for patterns like `*Controller`, `*Handler`, `*Endpoint`, `Main`, `Startup`
- `framework_multiplier` = for known patterns (ASP.NET controllers, Express routes, etc.)
- Utility penalty = 0.3 for helper/utility functions

**BFS parameters (configurable):**
- `max_trace_depth` = 10
- `max_branching` = 4 (follow top-4 callees by confidence at each step)
- `max_processes` = 75
- `min_steps` = 2

Each process records:
- Entry symbol and terminal symbol
- Ordered steps with symbol references
- Whether it is `intra_community` or `cross_community`
- Total confidence (product of edge confidences along the path)

Deduplication: traces that are strict subsets of longer traces are removed.

## .NET-Specific Handling

The `dotnet/` module provides specialised support for the .NET ecosystem.

### Solution Parsing (`solution.py`)
Parse `.sln` files to extract:
- Project entries (name, path, GUID, type GUID)
- Solution configurations
- Project nesting (solution folders)

### Project Parsing (`project.py`)
Parse `.csproj`/`.vbproj` files to extract:
- `<ProjectReference>` → internal dependency edges
- `<PackageReference>` → external NuGet dependencies (name + version)
- `<RootNamespace>` → for namespace-to-project mapping
- `<AssemblyName>` → for assembly-level reference resolution
- Target framework(s)

### Assembly Mapping (`assembly.py`)
Build a mapping from namespace → project, using `<RootNamespace>` from `.csproj` and observed namespaces in source files. This enables cross-project call resolution: when a C# file `using MyApp.Services` resolves to the Services project, calls to `OrderService.Process()` can be traced across project boundaries even within a single repo.

## Output Schema

Single JSON file written to `<repo_name>.mycelium.json` (or specified via `--output`).

```json
{
  "version": "1.0",
  "metadata": {
    "repo_name": "my-project",
    "repo_path": "/path/to/repo",
    "analysed_at": "2026-02-05T14:30:00Z",
    "mycelium_version": "0.1.0",
    "commit_hash": "abc123",
    "analysis_duration_ms": 1250
  },
  "stats": {
    "files": 142,
    "folders": 23,
    "symbols": 876,
    "calls": 2341,
    "imports": 198,
    "communities": 8,
    "processes": 12,
    "languages": {"cs": 97, "ts": 45}
  },
  "structure": {
    "files": [
      {"path": "src/Services/OrderService.cs", "language": "cs", "size": 4520, "lines": 156}
    ],
    "folders": [
      {"path": "src/Services/", "file_count": 8}
    ]
  },
  "symbols": [
    {
      "id": "sym_001",
      "name": "OrderService",
      "type": "Class",
      "file": "src/Services/OrderService.cs",
      "line": 15,
      "visibility": "public",
      "exported": true,
      "parent": null,
      "language": "cs"
    },
    {
      "id": "sym_002",
      "name": "ProcessOrder",
      "type": "Method",
      "file": "src/Services/OrderService.cs",
      "line": 42,
      "visibility": "public",
      "exported": true,
      "parent": "sym_001",
      "language": "cs"
    }
  ],
  "imports": {
    "file_imports": [
      {"from": "src/Controllers/OrderController.cs", "to": "src/Services/OrderService.cs", "statement": "using MyApp.Services"}
    ],
    "project_references": [
      {"from": "Web.csproj", "to": "../Core/Core.csproj", "type": "ProjectReference"}
    ],
    "package_references": [
      {"project": "Web.csproj", "package": "Newtonsoft.Json", "version": "13.0.3"}
    ]
  },
  "calls": [
    {
      "from": "sym_010",
      "to": "sym_002",
      "confidence": 0.9,
      "tier": "A",
      "reason": "import-resolved",
      "line": 28
    }
  ],
  "communities": [
    {
      "id": "community_0",
      "label": "Order Processing",
      "members": ["sym_001", "sym_002", "sym_003"],
      "cohesion": 0.72,
      "primary_language": "cs"
    }
  ],
  "processes": [
    {
      "id": "process_0",
      "entry": "sym_010",
      "terminal": "sym_025",
      "steps": ["sym_010", "sym_002", "sym_003", "sym_025"],
      "type": "cross_community",
      "total_confidence": 0.68
    }
  ]
}
```

## CLI Interface

```
mycelium-map analyze <path> [OPTIONS]

Arguments:
  path                    Path to repository root

Options:
  -o, --output PATH       Output JSON file path (default: <repo_name>.mycelium.json)
  -l, --languages LANGS   Comma-separated language filter (default: auto-detect)
  --resolution FLOAT      Louvain resolution parameter (default: 1.0)
  --max-processes INT     Maximum execution flows to detect (default: 75)
  --max-depth INT         Maximum BFS trace depth (default: 10)
  --exclude PATTERNS      Additional glob patterns to exclude
  --verbose               Show per-phase progress and timing
  --quiet                 Suppress all output except errors
```

Example usage:
```bash
# Analyse a single repo
mycelium-map analyze ./my-project

# Analyse with custom output path
mycelium-map analyze ./my-project -o my_project.mycelium.json

# Analyse with finer community granularity
mycelium-map analyze ./my-project --resolution 1.5

# Analyse only C# and TypeScript files
mycelium-map analyze ./my-project --languages cs,ts
```

## Dependencies

| Package | Purpose |
|---|---|
| `tree-sitter` | AST parsing engine |
| `tree-sitter-c-sharp` | C# grammar |
| `tree-sitter-vb-net` | VB.NET grammar |
| `tree-sitter-typescript` | TypeScript grammar |
| `tree-sitter-javascript` | JavaScript grammar |
| `tree-sitter-python` | Python grammar |
| `tree-sitter-java` | Java grammar |
| `tree-sitter-go` | Go grammar |
| `tree-sitter-rust` | Rust grammar |
| `tree-sitter-c` | C grammar |
| `tree-sitter-cpp` | C++ grammar |
| `networkx` | In-memory graph + Louvain community detection |
| `community-louvain` (or `networkx.community`) | Louvain algorithm |
| `click` | CLI framework |
| `rich` | Progress bars and terminal output |

All dependencies are pure Python or provide pre-built wheels. No system-level installs required.

## Future Considerations

These are explicitly out of scope for v1 but the architecture should not preclude them:

- **Multi-repo merge mode.** A `mycelium merge *.mycelium.json` command that combines per-repo JSONs, resolves cross-repo calls via project/package references, and re-runs community detection and process discovery across the unified graph.
- **Incremental analysis.** Cache ASTs and symbol tables, re-parse only changed files (by mtime or git diff), and recompute affected edges.
- **MCP server mode.** Expose the graph via MCP tools so Claude Code can query it interactively (search symbols, run Cypher-like queries, get impact analysis).
- **Embedding generation.** Add semantic embeddings per symbol for hybrid search (BM25 + vector).
- **Visualisation.** Export to formats consumable by graph visualisation tools (Graphviz DOT, Sigma.js JSON, Mermaid).

## Implementation Plan

The build is structured into seven milestones. Each milestone produces working, testable code. Earlier milestones are dependencies for later ones, but within a milestone the order of sub-tasks is flexible.

### Milestone 1: Project Scaffolding and Core Infrastructure

**Objective:** Establish the project structure, build system, and shared data types so all subsequent work has a foundation to build on.

**Tasks:**

1. **Set up the package structure.** Create the directory layout as described in the Architecture section (`mycelium/`, `mycelium/phases/`, `mycelium/languages/`, `mycelium/dotnet/`, `mycelium/graph/`, `tests/`, `tests/fixtures/`). Populate `__init__.py` files.

2. **Configure `pyproject.toml`.** Add all dependencies (`tree-sitter`, grammar packages, `networkx`, `community-louvain`, `click`, `rich`). Add dev dependencies (`pytest`, `pytest-cov`). Configure the `mycelium` CLI entry point via `[project.scripts]`.

3. **Define core data types in `config.py`.** Create dataclasses/TypedDicts for the shared vocabulary:
   - `FileNode`, `FolderNode` (Phase 1 output)
   - `Symbol` with fields: `id`, `name`, `type` (enum: Class, Function, Method, Interface, Struct, Enum, Namespace, Property, Constructor, etc.), `file`, `line`, `visibility`, `exported`, `parent`, `language`
   - `CallEdge` with fields: `from_symbol`, `to_symbol`, `confidence`, `tier`, `reason`, `line`
   - `ImportEdge`, `ProjectReference`, `PackageReference`
   - `Community` with fields: `id`, `label`, `members`, `cohesion`, `primary_language`
   - `Process` with fields: `id`, `entry`, `terminal`, `steps`, `type`, `total_confidence`
   - `AnalysisResult` as the top-level container matching the output schema

4. **Implement the `LanguageAnalyser` protocol in `languages/base.py`.** Define the abstract interface that all language modules must implement:
   ```python
   class LanguageAnalyser(Protocol):
       extensions: list[str]
       grammar_name: str
       def extract_symbols(self, tree: Tree, source: bytes, file_path: str) -> list[Symbol]
       def extract_imports(self, tree: Tree, source: bytes, file_path: str) -> list[ImportStatement]
       def extract_calls(self, tree: Tree, source: bytes, file_path: str) -> list[RawCall]
       def builtin_exclusions(self) -> set[str]
   ```

5. **Implement the language registry in `languages/__init__.py`.** Map file extensions to `LanguageAnalyser` implementations. Auto-detect language from extension. Handle the `--languages` CLI filter.

6. **Implement `graph/knowledge_graph.py`.** Wrapper around `networkx.DiGraph` providing typed methods for adding nodes and edges:
   - `add_file(FileNode)`, `add_folder(FolderNode)`
   - `add_symbol(Symbol)`, `add_call(CallEdge)`, `add_import(ImportEdge)`
   - `add_community(Community)`, `add_process(Process)`
   - Query helpers: `get_symbols_in_file(path)`, `get_callers(symbol_id)`, `get_callees(symbol_id)`

7. **Implement `graph/symbol_table.py`.** The dual HashMap:
   - `file_index: dict[str, dict[str, str]]` (file_path -> symbol_name -> node_id)
   - `global_index: dict[str, list[SymbolDefinition]]` (symbol_name -> definitions)
   - Methods: `add(symbol)`, `lookup_exact(file_path, name)`, `lookup_fuzzy(name)`

8. **Implement `output.py`.** Serialise `AnalysisResult` to the JSON schema defined in the architecture. Handle the `--output` flag.

9. **Implement `pipeline.py`.** The orchestrator that runs phases 1-6 sequentially, passing the knowledge graph and symbol table through each phase. Collect timing per phase. Return `AnalysisResult`.

10. **Wire up `main.py` with Click.** Implement `mycelium analyze <path>` with all CLI options. Parse args, call `pipeline.run()`, write output, print summary stats.

11. **Write scaffolding tests.** Verify the CLI runs without errors on an empty directory. Verify JSON output matches the schema. Verify the symbol table add/lookup round-trips.

---

### Milestone 2: Phase 1 (Structure) and Phase 2 (Parsing) for C# and VB.NET

**Objective:** Parse .NET source files into a symbol table. This is the foundation that all other phases depend on, targeting the two must-have languages first.

**Tasks:**

1. **Implement `phases/structure.py`.** Walk the repo directory tree. Skip ignored paths (configurable default list: `.git`, `bin`, `obj`, `node_modules`, `packages`, `.vs`, `.idea`, `TestResults`). Create `FileNode` and `FolderNode` entries. Classify each file by language using the registry. Collect `.sln`, `.csproj`, `.vbproj` paths for later use.

2. **Implement `languages/csharp.py`.** Load `tree-sitter-c-sharp` grammar. Implement `extract_symbols`:
   - Walk AST for `class_declaration`, `interface_declaration`, `struct_declaration`, `enum_declaration`, `method_declaration`, `constructor_declaration`, `property_declaration`, `namespace_declaration`, `record_declaration`, `delegate_declaration`.
   - Extract name, line, visibility from modifiers (`public`, `private`, `internal`, `protected`).
   - Track parent class/struct for methods and properties.
   - Determine export status from visibility.

3. **Implement `languages/vbnet.py`.** Load `tree-sitter-vb-net` grammar. Implement `extract_symbols`:
   - Walk AST for `class_statement`, `interface_statement`, `structure_statement`, `enum_statement`, `sub_statement`, `function_statement`, `property_statement`, `namespace_statement`, `module_statement`.
   - Extract name, line, visibility (`Public`, `Private`, `Friend`, `Protected`).
   - VB.NET has `Module` as a first-class construct (static class equivalent) -- handle this as a symbol type.

4. **Implement `phases/parsing.py`.** For each `FileNode` with a recognised language, parse with Tree-sitter, call the language analyser's `extract_symbols`, add results to the knowledge graph and symbol table. Create `DEFINES` edges (File -> Symbol).

5. **Create test fixtures.** Write small but representative C# and VB.NET files in `tests/fixtures/`:
   - `csharp_simple/`: A controller, service, model, and interface. Public and internal classes. Nested methods.
   - `vbnet_simple/`: A module, class with subs and functions, enum. Public and Friend visibility.
   - `mixed_dotnet/`: A `.sln` with one C# project and one VB.NET project referencing each other.

6. **Write Phase 1 and Phase 2 tests.** Verify structure phase finds the right files, skips ignored directories. Verify parsing extracts expected symbols from fixtures, with correct types, lines, visibility, and parent relationships.

---

### Milestone 3: Phase 3 (Imports) with .NET Project/Solution Support

**Objective:** Resolve file-level imports and project-level references for .NET codebases.

**Tasks:**

1. **Implement `dotnet/solution.py`.** Parse `.sln` files (they're a custom text format, not XML). Extract project entries: name, relative path, project type GUID, project GUID. Handle solution folders (nested projects).

2. **Implement `dotnet/project.py`.** Parse `.csproj`/`.vbproj` files (XML with MSBuild schema). Extract:
   - `<ProjectReference Include="...">` with resolved relative paths
   - `<PackageReference Include="..." Version="...">`
   - `<RootNamespace>` (defaults to project name if absent)
   - `<AssemblyName>` (defaults to project name if absent)
   - `<TargetFramework>` / `<TargetFrameworks>`

3. **Implement `dotnet/assembly.py`.** Build the namespace-to-project mapping:
   - Seed from `<RootNamespace>` in each `.csproj`/`.vbproj`
   - Supplement by scanning `namespace` declarations in parsed source files
   - Map: `dict[str, str]` (namespace -> project_path)
   - Method: `resolve_namespace(namespace: str) -> Optional[str]` returns the project that owns it

4. **Implement C# import extraction in `languages/csharp.py`.** Parse `using` directives from AST. Resolve each to a target file where possible:
   - First try namespace-to-project mapping to identify the target project
   - Then search the file index within that project for matching symbols
   - Record unresolved imports (external namespaces from NuGet packages) separately

5. **Implement VB.NET import extraction in `languages/vbnet.py`.** Parse `Imports` statements. Same resolution strategy as C#.

6. **Implement `phases/imports.py`.** Orchestrate all three levels:
   - Parse `.sln` files discovered in Phase 1 to build the project graph
   - Parse `.csproj`/`.vbproj` files for project and package references
   - Process source file imports using language analysers
   - Add `IMPORTS`, `PROJECT_REFERENCES`, and `PACKAGE_REFERENCES` edges to the graph

7. **Write tests.** Verify `.sln` parsing extracts project entries correctly. Verify `.csproj` parsing captures ProjectReference and PackageReference. Verify `using`/`Imports` resolution against the `mixed_dotnet` fixture. Verify unresolved external imports are recorded but don't cause errors.

---

### Milestone 4: Phase 4 (Calls) with Confidence Scoring

**Objective:** Build the call graph with three-tier confidence scoring.

**Tasks:**

1. **Implement C# call extraction in `languages/csharp.py`.** Walk AST for `invocation_expression`, `object_creation_expression`, `member_access_expression`. For each call site:
   - Identify the callee name (method name, possibly qualified with class/namespace)
   - Identify the enclosing method/function (walk up the AST to find the containing `method_declaration` or `constructor_declaration`)
   - Return `RawCall(caller_file, caller_name, callee_name, line, qualifier)`

2. **Implement VB.NET call extraction in `languages/vbnet.py`.** Walk AST for `invocation_expression`, `member_access_expression`. VB.NET also has `Call` keyword and `RaiseEvent` -- handle both. Same output format as C#.

3. **Implement C# and VB.NET built-in exclusion lists.** Filter out noise: `Console.WriteLine`, `Console.ReadLine`, `String.Format`, `String.IsNullOrEmpty`, `Convert.*`, `Math.*`, `Object.ToString`, `Object.Equals`, `Object.GetHashCode`, `Debug.*`, `Trace.*`, `GC.*`, common LINQ methods (`Select`, `Where`, `FirstOrDefault`, etc.), `Task.Run`, `Task.WhenAll`, etc.

4. **Implement `phases/calls.py`.** For each source file, get raw calls from the language analyser. Resolve each call through the three tiers:
   - **Tier A:** Check if the callee's qualifier matches an import in the file. If so, look up the imported file in the file index for the callee name. Confidence 0.9.
   - **Tier B:** Check if the callee is defined in the same file (file index lookup). Confidence 0.85.
   - **Tier C:** Fall back to global index. If exactly one match, confidence 0.5. If multiple matches, confidence 0.3.
   - If no match at any tier, discard the call (likely a framework/runtime call not in the exclusion list).

   Add `CALLS` edges to the graph.

5. **Write tests.** Verify call extraction from C# and VB.NET fixtures. Verify tier assignment is correct (import-resolved vs same-file vs fuzzy). Verify built-ins are filtered. Verify cross-language calls within `mixed_dotnet` fixture (C# calling VB.NET class via project reference).

---

### Milestone 5: Phase 5 (Communities) and Phase 6 (Processes)

**Objective:** Cluster the call graph and detect execution flows.

**Tasks:**

1. **Implement `phases/communities.py`.**
   - Build an undirected `networkx.Graph` from `CALLS`, `EXTENDS`, `IMPLEMENTS` edges. Weight edges by confidence score.
   - Run `community.louvain_communities()` (networkx 3.x built-in) or `community_louvain.best_partition()` with the configurable resolution parameter.
   - For each community, compute:
     - Label: most common directory prefix among members, or longest common prefix of member names. Fall back to `"Community N"`.
     - Cohesion: `internal_edges / (n * (n-1) / 2)` where n = member count.
     - Primary language: mode of member languages.
   - Discard singleton communities.
   - Add `Community` nodes and `MEMBER_OF` edges to the graph.

2. **Implement `graph/scoring.py`.** Entry point scoring:
   - For each symbol, compute `base_score = out_degree / (in_degree + 1)`.
   - Apply `export_multiplier = 2.0` if symbol is public/exported.
   - Apply `name_multiplier = 1.5` if name matches entry point patterns. Patterns for .NET: `*Controller`, `*Handler`, `*Endpoint`, `*Middleware`, `Main`, `Startup`, `Configure*`, `Map*Endpoints`. General: `*Route`, `*Listener`, `handle*`, `on*`, `process*`.
   - Apply `framework_multiplier`: detect ASP.NET controller base classes, `[HttpGet]`/`[HttpPost]` attributes, `IHostedService` implementations. (This requires checking the symbol's AST context or parent class -- store enough metadata in Phase 2.)
   - Apply utility penalty `0.3` for symbols in paths containing `Utils`, `Helpers`, `Extensions`, `Common`.
   - Return ranked list of entry points.

3. **Implement `phases/processes.py`.**
   - Take the top N entry points by score (N = `max_processes * 2` to allow for deduplication).
   - BFS from each entry point following `CALLS` edges:
     - At each node, follow the top `max_branching` callees sorted by edge confidence.
     - Stop at `max_depth` or when no more callees.
     - Record the path as a process.
   - Filter out processes shorter than `min_steps`.
   - Deduplicate: remove processes that are strict subsequences of longer processes.
   - Classify each process as `intra_community` (all steps in same community) or `cross_community`.
   - Compute `total_confidence` as the product of edge confidences along the path.
   - Cap at `max_processes` (keep highest total_confidence).

4. **Write tests.** Build small synthetic graphs in tests (no need for Tree-sitter fixtures -- construct the knowledge graph directly). Verify Louvain produces expected clusters for a graph with two obvious communities. Verify entry point scoring ranks controllers above utility functions. Verify BFS traces the expected path. Verify deduplication removes subset traces.

---

### Milestone 6: Additional Language Support

**Objective:** Implement language analysers for the remaining 8 languages. Each follows the same pattern established by C# and VB.NET.

**Tasks:**

1. **Implement `languages/typescript.py`.** Covers both TypeScript and JavaScript (JavaScript is a subset). Symbol extraction: `function_declaration`, `class_declaration`, `method_definition`, `arrow_function` (when assigned to a variable), `interface_declaration`, `type_alias_declaration`, `enum_declaration`. Import resolution: ES `import`/`export`, `require()`, `tsconfig.json` path aliases (`compilerOptions.paths`). Call extraction: `call_expression`, `new_expression`. Built-in exclusions: `console.*`, `setTimeout`, `setInterval`, `Promise.*`, `JSON.*`, `Array.*`, `Object.*`.

2. **Implement `languages/python.py`.** Symbol extraction: `function_definition`, `class_definition`, `decorated_definition`. Import resolution: `import_statement`, `import_from_statement`, resolve via package directory structure and `__init__.py`. Call extraction: `call` expressions. Built-in exclusions: `print`, `len`, `range`, `enumerate`, `zip`, `map`, `filter`, `isinstance`, `type`, `super`, `str`, `int`, `float`, `list`, `dict`, `set`, `tuple`.

3. **Implement `languages/java.py`.** Symbol extraction: `class_declaration`, `interface_declaration`, `method_declaration`, `constructor_declaration`, `enum_declaration`, `annotation_type_declaration`, `record_declaration`. Import resolution: `import_declaration`, map to files via package directory convention (`com.example.Foo` -> `com/example/Foo.java`). Call extraction: `method_invocation`, `object_creation_expression`. Built-in exclusions: `System.out.*`, `Objects.*`, `Arrays.*`, `Collections.*`, `String.*`, `Integer.*`, `Math.*`.

4. **Implement `languages/go.py`.** Symbol extraction: `function_declaration`, `method_declaration`, `type_declaration` (struct, interface), `const_declaration`. Import resolution: `import_declaration`, resolve via Go module path and directory structure. Call extraction: `call_expression`. Built-in exclusions: `fmt.*`, `log.*`, `append`, `make`, `len`, `cap`, `close`, `delete`, `new`, `panic`, `recover`.

5. **Implement `languages/rust.py`.** Symbol extraction: `function_item`, `struct_item`, `enum_item`, `impl_item`, `trait_item`, `type_item`, `const_item`, `static_item`, `mod_item`, `macro_definition`. Import resolution: `use_declaration`, `mod_item`, resolve via module tree and `Cargo.toml`. Call extraction: `call_expression`, `macro_invocation`. Built-in exclusions: `println!`, `eprintln!`, `format!`, `vec!`, `assert*!`, `todo!`, `unimplemented!`, `panic!`, `dbg!`, `String::from`, `Into::into`, `From::from`, `Clone::clone`.

6. **Implement `languages/c_cpp.py`.** Shared module for C and C++. Symbol extraction: `function_definition`, `struct_specifier`, `class_specifier` (C++), `enum_specifier`, `namespace_definition` (C++), `template_declaration` (C++), `typedef_declaration`. Import resolution: `#include` directives, resolve `"local.h"` relative to file, `<system.h>` against include paths (best-effort). Call extraction: `call_expression`. Built-in exclusions: `printf`, `malloc`, `free`, `memcpy`, `memset`, `strlen`, `strcmp`, `sizeof`, `assert`, `std::cout`, `std::cerr`, `std::endl`, `std::move`, `std::make_shared`, `std::make_unique`.

7. **Create test fixtures per language.** Small representative files in `tests/fixtures/` for each language. Each fixture should have at least: two files with cross-file imports, a class/struct with methods, and a function call chain.

8. **Write tests per language.** Verify symbol extraction, import resolution, and call extraction for each language's fixtures.

---

### Milestone 7: Integration Testing, Polish, and CLI Refinement

**Objective:** End-to-end testing against real repos, performance validation, and CLI polish.

**Tasks:**

1. **End-to-end test against a small repo.** Pick a repo with a handful of source files. Run the full pipeline. Manually verify the JSON output: are the symbols correct? Are the calls plausible? Do the communities make sense?

2. **End-to-end test against a medium repo.** Pick a repo with more complexity (e.g., 500+ source files). Verify the pipeline handles larger codebases without errors or excessive memory use.

3. **Performance benchmarking.** Time each phase on repos of varying size. Identify bottlenecks. Target: under 30 seconds for a 500-file repo, under 5 minutes for a 3000-file repo. Tree-sitter parsing should be negligible; call resolution and community detection are the likely bottlenecks.

4. **Error handling and resilience.** Ensure the pipeline handles gracefully:
   - Files that fail to parse (malformed syntax, encoding issues) -- log and skip
   - Missing `.sln` or `.csproj` files -- proceed without project-level resolution
   - Empty repos or repos with no recognised source files -- produce a valid but sparse JSON
   - Very large files (generated code, minified JS) -- skip files above a configurable size threshold (default 1MB)

5. **CLI polish with Rich.** Add progress bars per phase (using Rich `Progress`). Show a summary table after completion (files parsed, symbols found, calls resolved, communities detected, processes traced). Implement `--verbose` (per-file progress) and `--quiet` (errors only) modes.

6. **Validate the output schema.** Write a JSON Schema definition for the `.mycelium.json` format. Add a test that validates output against the schema. This ensures consumers can rely on the format.

7. **Write a CLAUDE.md for Mycelium.** Document commands (`uv sync`, `uv run mycelium analyze`, `uv run pytest`), key patterns, and the project structure for future Claude Code sessions.

---

### Milestone Summary

| Milestone | Builds on | Delivers |
|---|---|---|
| **1: Scaffolding** | Nothing | Project structure, data types, graph, symbol table, CLI skeleton, pipeline orchestrator |
| **2: Structure + Parsing** | M1 | Phase 1 + 2 for C# and VB.NET. File tree and symbol extraction working. |
| **3: Imports** | M2 | Phase 3 with full .NET support (.sln, .csproj, namespace resolution). |
| **4: Calls** | M3 | Phase 4 with three-tier confidence scoring. Call graph complete for .NET. |
| **5: Communities + Processes** | M4 | Phase 5 + 6. Louvain clustering and BFS execution flow detection. Full pipeline working end-to-end for .NET repos. |
| **6: Additional Languages** | M1 | TypeScript, JavaScript, Python, Java, Go, Rust, C, C++. Each independent of the others. |
| **7: Polish** | M5 | End-to-end testing on real repos, performance, error handling, CLI output, schema validation. |

Milestones 1-5 are sequential (each depends on the previous). Milestone 6 can be worked on in parallel with milestones 3-5 since each language module only depends on the scaffolding from milestone 1. Milestone 7 depends on milestone 5 being complete.
