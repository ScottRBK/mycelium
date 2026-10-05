---
name: mycelium-architecture
description: >-
  Add or refresh a repository's Mycelium class diagram and its AGENTS.md architecture pointer.
  Use when setting up diagram-based architectural guidance for coding agents.
---

# Mycelium architecture guidance

1. Use the installed `mycelium-mermaid` skill to generate the target repository's diagram at
   `docs/assets/mycelium_class_diagram.md`. All output paths are relative to that repository's root.
   If the companion skill is missing, install it first:

   ```bash
   npx skills add ScottRBK/mycelium --skill mycelium-mermaid -y
   ```

2. After verifying a nonempty diagram, add or update this note in the root `AGENTS.md`, creating the
   file if needed. The link below belongs in the target repository, not the installed skill folder:

   ```markdown
   ## Architecture

   Before changing code or reasoning about this repository's architecture, read the
   [Mycelium class diagram](docs/assets/mycelium_class_diagram.md).
   ```

   Reuse an existing architecture section when present. Preserve unrelated instructions; rerunning
   this skill updates the same note without duplicating it. Keep the diagram in its own file.

3. Verify the AGENTS.md link resolves to the generated file. Report both paths and any generation
   warnings; an unsuccessful or empty export must not create a new pointer.
