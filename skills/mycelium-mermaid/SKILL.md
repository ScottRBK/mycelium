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

## 2. Check the available CLI

Run `mycelium-map export --help` and verify that the required options are available. Both the Python
and native CLIs use this command name and the same Rust exporter.

If Mycelium is absent, install `mycelium-map` in a tool environment or virtual environment using
Python 3.12 or newer. The skill installer installs instructions only. Source builds need Rust.
If an installed release lacks `export` or a needed option, use an up-to-date Mycelium checkout:

```bash
cargo run --manifest-path /path/to/mycelium/Cargo.toml -p mycelium-cli -- export --help
```

Use that Cargo command prefix in place of `mycelium-map` for the following commands when needed.
This step is complete when a working CLI exposes the required export options.

## 3. Analyse and export

For a source repository, write the intermediate map into a temporary directory. Keep it outside the
source tree so a large analysis artifact does not become part of the requested deliverable.
For example, in a POSIX shell, substitute the selected repository and output paths:

```bash
diagram_tmp=$(mktemp -d)
mycelium-map analyze /path/to/repo -o "$diagram_tmp/map.json" --quiet
mycelium-map export "$diagram_tmp/map.json" --format mermaid -o /path/to/class-diagram.md
```

For a supplied map, export it directly:

```bash
mycelium-map export /path/to/map.json --format mermaid -o /path/to/class-diagram.md
```

Analyse the whole repository, then apply `--path` at export time for a narrower view; this keeps
repository context available to analysis. Automatic language detection handles mixed repositories.
If a map has no declaration data, rerun analysis with an export-capable build. If the source is
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
