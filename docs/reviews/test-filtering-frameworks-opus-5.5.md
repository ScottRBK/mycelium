# Framework test-filtering review

Date: 2026-10-01. Scope: the approved follow-up to Rust/Go/path filtering, adding direct framework
rules for Python, C#, VB.NET, Java, TypeScript and JavaScript. C/C++ remain explicit-path only.

Claude Code reported `claude-opus-5-5`, using high effort, in session
`5517daf0-1498-41af-a38d-c946af05e469`. Both submissions contain the changes for this follow-up,
relative to the previously reviewed filtering implementation. The second submission contains the
changes since the first review. Tools are disabled: the reviewer does not execute tests or access
the repository. Generated VB parser C/JSON/headers are excluded from the review payload.

## First review and corrections

The first review requested changes. Its blockers concerned Java assertion imports, nested VB type
attributes, and types promoted from VB Modules. The following cases now have regression coverage:

- Explicit Java JUnit imports work alongside static assertion imports and unrelated wildcards.
- VB nested class attributes belong to the nested class, and nested interfaces remain independent.
- VB `Namespace Global.App` resets the namespace prefix; Module types also get promoted lookup keys.
- Blank lines between VB Option and Imports do not disable detection.
- C# return-target attributes do not mark the method as a test. Using directives after a file-scoped
  namespace still work.
- Inherited member types and static imports cause conservative retention where binding is uncertain.
  Explicit global .NET markers still work. Cross-file global static imports veto relative markers.
- Commented JS arguments retain the correct callback position.

The original VB grammar could not represent nested nominal types, so the first suspected nested
attribute bug initially fell back to malformed-syntax retention. The grammar now accepts nested
classes, structures, interfaces and enums; the regression checks successful parsing as well as
retention of the outer application class. Parser artifacts use Tree-sitter CLI 0.26.10, ABI 14.

Additional checks during implementation found and corrected relative C# using/alias targets,
imported JS `require` shadows, and commented global/type-only import keywords. Each correction was
introduced with a failing source-to-export regression. Framework syntax validity is computed once.

Some review questions required unchanged-code context: extraction fills each member's file before
repository conflict checks; all current entrypoints use the pipeline; declaration extraction was
already behind a crate-private module. The existing child-node helper excludes comments. None of
these required a production change. The redundant immutable-value test assertion was removed.

Supported CommonJS forms remain direct `const` destructuring in `.js`/`.jsx`; default `require`
bindings and `.cjs`/`.mjs` registration are deferred. These limits are explicit in the export guide.

## Follow-up verdict

The second review concluded: **Ready, no blockers from this diff**, conditional on the checkpoint,
Python parity and parser-regeneration checks completing successfully. All three subsequently passed.
Both review runs completed successfully and reported the requested model.

The reviewer accepted the fixes and noted that inherited scopes, static imports, and broad relative
namespace guards can retain real tests. That conservative tradeoff is intentional for this version;
explicit paths handle those cases. Narrowing these guards can be considered separately.

The suggested VB coverage was added after the follow-up: include views put nested methods, fields
and enum variants in the correct boxes; nested members do not inherit the enclosing test marker.
A VB inherited lookalike also stays visible. The grammar names are `inherits_clause` and
`implements_clause`; ordinary multiline inheritance still uses the existing malformed-syntax
fallback. No production changes were needed after the second review. Documentation was reflowed.

## Executed validation

These checks were executed by the implementing agent, independently of the diff-only review:

- 488 Rust workspace tests, including 22 new framework integration tests.
- Clippy with warnings denied, rustfmt, diff whitespace checks, and changed handwritten line widths.
- Rebuilt Python extension; all nine binding tests pass.
- Native CLI, Python CLI and Python API produce identical include/exclude Markdown for all six
  new language variants, using saved maps after source removal.
- All six pinned repositories pass: Forgetful, Ferox, eval-harness, SurelyCRM, pi-forgetful and
  pi-web-search. No commits were repinned. Four checkpoints add automatic-only filtering views.
- All 12 prior include-mode artifacts remain byte-identical. The six existing filtered views change
  only the detector version line. Four new automatic views validate independently checked source
  facts and frozen hashes. All 22 Markdown artifacts stayed unchanged through the review fixes.
- The six generated VB C/JSON/header artifacts reproduce byte-for-byte from the grammar.
- All ten diagram blocks in the new automatic checkpoint views were rendered and inspected.
  Long names/signatures can still overlap; complete signatures are retained.

The checkpoints cover selected source files pinned to commits, not every file in those repositories.
SurelyCRM's generated Xunit import remains outside direct source detection and needs an explicit
`--test-path`. Detection saves version 2 evidence; older version 1 maps preserve their Rust/Go rules
and recommend reanalysis for the new framework rules. Analysis facts remain complete.

## Review payload hashes

- `mycelium-framework.patch`:
  `a1f21fed779f3ab65781d6b9166b347621adaaa8b3afa3a964c3ed33cf341c00`
- `mycelium-framework-followup.patch`:
  `dfbc0c6a58f54f3378810681d7d63202c5ab0e0cbfdeaebc10d3c47407458079`
