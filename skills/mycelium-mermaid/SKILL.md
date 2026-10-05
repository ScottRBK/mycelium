---
name: mycelium-mermaid
description: >-
  Generate Mermaid class diagrams as Markdown with Mycelium. Use when exporting a source repository
  or saved Mycelium map to Markdown with typed fields, signatures, type relationships, and calls.
---

# Mycelium Mermaid Markdown

Use Mycelium's deterministic exporter to produce the diagram content. The agent selects the scope
and checks the result; declarations and relationships come from the saved analysis facts.

## 1. Select the input and destination

Identify the source repository or user-supplied Mycelium JSON map and the destination Markdown file.
Default to the current repository and `class-diagram.md` in the working directory when unspecified.
For a request about current code, analyse the current checkout. Use a supplied map as its saved
snapshot; exporting never rereads source files.

Keep the default test exclusion unless the user requests tests. For a scoped view, test overrides,
or a request for one diagram, read [export options](references/export-options.md).

This step is complete when the input, output path, scope, and test selection are known.

## 2. Use the PyPI package

The skill installer installs instructions only. Run the PyPI package through `uvx` and verify that
the required export options are available:

```bash
uvx --no-build mycelium-map export --help
```

Version 0.4.2+ provides wheels for standard CPython 3.12+ on Linux with glibc (x86_64, aarch64),
macOS (x86_64, aarch64), and Windows (x86_64). They include the Rust engine; no Rust toolchain is
needed. `--no-build` prevents an implicit source build. If no compatible wheel is available, check
the Python version and platform against the release files on PyPI. Refresh an outdated cached
package with `uvx --refresh --no-build mycelium-map export --help`.

An existing `mycelium-map` installation can replace the `uvx --no-build mycelium-map` prefix in the
commands below. If uv is unavailable, install `mycelium-map>=0.4.2` with
`python -m pip install --upgrade --only-binary=mycelium-map 'mycelium-map>=0.4.2'` in an activated
virtual environment, then check `mycelium-map export --help`. If a needed option is still absent,
report the limitation. Use source builds only for tasks developing or testing Mycelium itself.

This step is complete when a working CLI exposes the required export options.

## 3. Analyse and export

For a source repository, write the intermediate map into a temporary directory. Keep it outside the
source tree so a large analysis artifact does not become part of the requested deliverable.
For example, in a POSIX shell, substitute the selected repository and output paths:

```bash
diagram_tmp=$(mktemp -d)
uvx --no-build mycelium-map analyze /path/to/repo -o "$diagram_tmp/map.json" --quiet
uvx --no-build mycelium-map export "$diagram_tmp/map.json" -o /path/to/class-diagram.md
```

For a supplied map, export it directly:

```bash
uvx --no-build mycelium-map export /path/to/map.json -o /path/to/class-diagram.md
```

Analyse the whole repository, then apply `--path` at export time for a narrower view; this keeps
repository context available to analysis. Automatic language detection handles mixed repositories.
If a map has no declaration data, rerun analysis with version 0.4.0 or newer. If the source is
unavailable, report that limitation instead of reconstructing the diagram by hand.

This step is complete when the command succeeds and writes the requested Markdown file.

## 4. Check and deliver the Markdown

Inspect the generated file for Mermaid fences containing `classDiagram`, selection summaries,
warnings, and the number of diagram blocks. If all declarations were filtered, report the empty
selection and its reason; do not silently switch to including tests. Investigate an unexpectedly
empty analysis using the selected paths, registered extensions, and analysis exclusions.

Preserve the exporter's full names, type/signature keys, source locations, and complete relationship
list. Adjust the result by changing options and rerunning export; hand edits break reproducibility.
Calls are static heuristics, and field references do not prove ownership.

For a determinism check, export the same saved map with identical options to a second temporary file
and compare the bytes. A rendered preview is optional; state whether rendering was actually checked.

Finish with a link to the Markdown, its scope and diagram count, and any material warnings.
Keep temporary artifacts outside the repository unless the user wants them retained.
