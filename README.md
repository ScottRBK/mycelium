# Mycelium

Static analysis CLI that maps the connections in a source code repository. Produces a single JSON file containing file structure, symbols, imports, call graph, community clusters, and execution flows.

## Install

```bash
uvx mycelium-map
```

Or install locally:

```bash
pip install mycelium-map
```

## Usage

```bash
mycelium-map analyze <path>                          # Analyse a repo
mycelium-map analyze <path> -o output.json           # Custom output path
mycelium-map analyze <path> --verbose                # Show phase timing breakdown
mycelium-map analyze <path> --quiet                  # No terminal output
mycelium-map analyze <path> -l cs,ts                 # Only analyse C# and TypeScript
mycelium-map analyze <path> --exclude vendor,legacy  # Skip directories
mycelium-map analyze <path> --resolution 1.5         # Louvain resolution (higher = more communities)
mycelium-map analyze <path> --max-processes 50       # Limit execution flows
mycelium-map analyze <path> --max-depth 8            # Limit BFS trace depth
```

Default output file: `<repo-name>.mycelium.json`

## Supported Languages

| Language | Extensions |
|---|---|
| C# | `.cs` |
| VB.NET | `.vb` (requires grammar build) |
| TypeScript | `.ts`, `.tsx` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |
| Python | `.py` |
| Java | `.java` |
| Go | `.go` |
| Rust | `.rs` |
| C | `.c`, `.h` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`, `.hh` |

## Output Schema

The JSON output contains these top-level sections:

### `metadata`

```json
{
  "repo_name": "my-project",
  "repo_path": "/absolute/path",
  "analysed_at": "2026-02-05T18:33:12Z",
  "mycelium_version": "0.1.0",
  "commit_hash": "a1b2c3d4e5f6",
  "analysis_duration_ms": 42.3,
  "phase_timings": { "structure": 0.004, "parsing": 0.001, ... }
}
```

### `stats`

Summary counts: `files`, `folders`, `symbols`, `calls`, `imports`, `communities`, `processes`, and a `languages` breakdown by file count.

### `structure`

File tree with language, size, and line counts.

```json
{
  "files": [{ "path": "src/main.cs", "language": "cs", "size": 1024, "lines": 45 }],
  "folders": [{ "path": "src/", "file_count": 3 }]
}
```

### `symbols`

Every extracted symbol: classes, methods, interfaces, functions, structs, enums, etc.

```json
{
  "id": "sym_0001",
  "name": "UserController",
  "type": "Class",
  "file": "Controllers/UserController.cs",
  "line": 8,
  "visibility": "public",
  "exported": true,
  "parent": "MyApp.Controllers",
  "language": "cs"
}
```

Symbol types: `Class`, `Function`, `Method`, `Interface`, `Struct`, `Enum`, `Namespace`, `Property`, `Constructor`, `Module`, `Record`, `Delegate`, `TypeAlias`, `Constant`, `Trait`, `Impl`, `Macro`, `Typedef`, `Annotation`.

Visibility: `public`, `private`, `internal`, `protected`.

### `imports`

Three categories of dependency edges:

```json
{
  "file_imports": [{ "from": "Controller.cs", "to": "Service.cs", "statement": "using MyApp.Services" }],
  "project_references": [{ "from_project": "Web.csproj", "to_project": "Core.csproj", "ref_type": "ProjectReference" }],
  "package_references": [{ "project": "Web.csproj", "package": "Newtonsoft.Json", "version": "13.0.1" }]
}
```

Project and package references are extracted from `.csproj`/`.vbproj` files.

### `calls`

Call graph edges with three-tier confidence scoring:

```json
{
  "from": "sym_0004",
  "to": "sym_0015",
  "confidence": 0.9,
  "tier": "A",
  "reason": "import-resolved",
  "line": 17
}
```

| Tier | Confidence | Meaning |
|------|-----------|---------|
| A | 0.9 | Callee found in an imported file |
| B | 0.85 | Callee found in the same file |
| C | 0.5 | Unique fuzzy match across the codebase |
| C | 0.3 | Ambiguous fuzzy match (multiple candidates) |

### `communities`

Clusters of symbols that frequently call each other, detected via Louvain algorithm.

```json
{
  "id": "community_0",
  "label": "Absence",
  "members": ["sym_0004", "sym_0015", "sym_0016"],
  "cohesion": 0.8,
  "primary_language": "cs"
}
```

### `processes`

Execution flows traced from entry points (controllers, handlers, main functions) via BFS through the call graph.

```json
{
  "id": "process_0",
  "entry": "sym_0004",
  "terminal": "sym_0016",
  "steps": ["sym_0004", "sym_0015", "sym_0016"],
  "type": "intra_community",
  "total_confidence": 0.765
}
```

`type` is `intra_community` when all steps are in the same community, or `cross_community` when the flow spans multiple.

`total_confidence` is the product of all edge confidences along the path.

## Development

```bash
uv sync                 # Install dependencies
uv run pytest           # Run tests
uv run pytest -v        # Verbose output
```

## Releasing

Releases are automated via GitHub Actions. Push a semver tag to trigger a release:

```bash
git tag v0.2.0
git push origin v0.2.0
```

This will:
1. Update the version in `pyproject.toml` to match the tag
2. Commit the version bump back to `master`
3. Build and publish the package to PyPI
