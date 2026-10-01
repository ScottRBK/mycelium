# Opus 5.5 review of the test filtering proposal

Reviewed on 2026-09-30 using Claude Code, with reported model `claude-opus-5-5`.
The reviewer received only the proposal text, with file access and other tools disabled.
The review completed successfully. It did not inspect repository source independently.

Verdict: **ready with revisions**, recommending a smaller first release.
The review below covers the first draft. The current proposal incorporates the author's
responses under its review section; those revisions have not received another Opus pass.

Reviewed draft SHA256:
`1c12de4a3450daaa8a9334b725962d6a22dab50e7047dcd04a722f5b092e011e`

## Reviewer response

# Review: test filtering proposal

I reviewed only the proposal text. I didn't open the repository, Forgetful or Context7, as you
asked.

**Verdict: ready with revisions, but only if the first release is cut down.** The core design is
sound: facts saved at analysis time, filtering at export time, uncertain code stays visible, and
per-occurrence filtering. The ten-language framework scope is too much for a first release. It
also makes the default view worse in several languages (see M1).

## Scope: yes, paths plus Rust/Go is the better first release

Rust and Go are the two languages where:
- the language itself defines what "test-only" means, so there's no guessing, **and**
- paths alone can't do the job. Rust inline tests sit inside production files. Go `_test.go`
  files sit next to production files, and there are no globs to pick them out.

For Python, C#, VB.NET, Java, TypeScript and JavaScript the opposite is true:
- Tests almost always live in test directories, which one `--test-path` already covers.
- The framework rules only label methods, so test classes stay visible as empty-looking shells (M1).
- Each language needs its own import, alias and shadowing logic. That's six sets of rules with
  little payoff.

---

## Blockers

**B1 · Blocker · Old maps / default mode**
- Example: a user runs today's plain export on an existing map. The new default is `exclude`, and
  the proposal says exclude on an old map "reports that analysis must be rerun". So a command that
  works today now fails.
- Consequence: breaks existing scripts. It also changes every existing checkpoint's recorded hash.
- Fix: path rules only need file locations, which are export options. So on an old map, apply the
  path rules and print one fixed notice: "no automatic evidence, map predates detector vN". This
  depends on old maps saving a source file for each occurrence (see V3). If they don't, keep
  `include` as the default until maps are regenerated.

**B2 · Blocker · "A path that never matched declarations remains an error"**
- Example: `test/model.test.ts` contains only `describe`/`it` callbacks, which produce no
  declarations. `--test-path test/model.test.ts` then errors, and that's your Pi Forgetful
  checkpoint.
- Second example: a shared script passes `--path src/api --test-path tests`, and it errors because
  of the scope.
- Fix: only error when the path matches no analysed *file* in the saved map. A match that falls
  outside the current `--path` scope should be a notice, not an error.

**B3 · Blocker · Parse errors can stretch a test scope over application code**
- Example: `#[cfg(test)] mod tests {` is missing its closing `}`. Parser recovery can place the
  production functions that follow *inside* that module, so they get hidden. A Python indentation
  error inside a `TestCase` class can do the same.
- Consequence: application code silently disappears. That's exactly the error the design promises
  never to make.
- Fix: if any syntax node between the evidence scope and the declaration has a parse error, drop
  the inherited evidence and emit a diagnostic. Add a fixture for this.

**B4 · Blocker (checkpoint plan) · eval-harness expectation contradicts the automatic rules**
- Example: the fixture repositories used as evaluation inputs are sample code. If they contain
  `_test.go` files or `#[test]` functions, the Go and Rust rules hide them. That contradicts the
  proposal's "remain unless explicitly excluded".
- Fix: check what those fixtures contain. Then either add `--keep-path <fixtures>` to that
  checkpoint or reword the expectation.
- Same check for Forgetful: if `test_harness/config.py` contains a `pytest.fixture` or a
  `TestCase`, then "remains despite its name" is false while the Python rules are active.

## Major

**M1 · Major · Framework rules leave empty-looking test classes (Java, xUnit, pytest, Jupiter)**
- Example: `class FooTest { @Mock Repo repo; @BeforeEach setUp(); @Test works(); }`. Only `works()`
  is hidden. The default view still shows `FooTest` with `repo`, `setUp()`, and an edge to `Repo`.
- Worse example: an NUnit fixture with a nested `FakeRepo : IRepo`. The nested class stays
  visible (by design), so the application view shows a fake as a real implementation of `IRepo`.
- Consequence: the default view becomes misleading rather than cleaner.
- Fix: defer these rules and use paths, which cover whole files including fakes. If they're kept,
  document these empty shells as known output.

**M2 · Major · Paths without any pattern can't handle tests stored next to source**
- Example: TypeScript `src/a.ts` + `src/a.test.ts` (×200), Python `foo_test.py` next to `foo.py`,
  or Rust `#[cfg(test)] mod tests;` pointing at a separate file (common). Each needs its own flag.
- Consequence: "paths remain the useful way" isn't true for these layouts.
- Fix, only if these layouts matter to you: a repeatable `--test-suffix .test.ts`. It's a literal
  match on the end of the filename and the user declares it, so it isn't a guess. For Rust, a later
  slice could record `#[cfg(test)] mod x;` and look for the two standard file locations (`x.rs` or
  `x/mod.rs`). Note that `#![cfg(test)]` at the top of a file already works under the proposal.

**M3 · Major · Edges must record which member or occurrence they came from (validate first)**
- Example: `impl Service { #[cfg(test)] fn with_clock(c: MockClock) }` creates an edge
  `Service→MockClock`. If saved edges are totalled per class, export can't remove it.
- Fixtures to add:
  - `#[cfg(test)] impl Clock for Foo` alongside `#[cfg(not(test))] impl Clock for Foo`: the edge
    must stay.
  - Two `fn now()` where one is test-only: the non-test one must stay.

**M4 · Major · Edges from visible code to hidden code are undefined in exclude mode**
- Example: `class FooTests(BaseTest)` where only `BaseTest` is hidden (because indirect inheritance
  is deferred). The inheritance arrow now points at nothing.
- "No dangling edges" is only promised for the separate view.
- Fix: in every mode, drop edges whose target is hidden and count them in the summary.

**M5 · Major · Shadowing across files can't be seen while extracting one file**
- Example: `using Xunit;`, while another file in the same namespace declares a `FactAttribute`. In
  C#, that local type wins over the import. Per-file extraction can't see it.
- Fix: record the candidate evidence along with the name it was bound to. Confirm it against all
  declared type names at the end of analysis, or document the gap. This is another reason to defer
  C# and VB.NET.

**M6 · Major · C# partial classes**
- In C#, an attribute on any part applies to the whole type. With `[TestClass]` on part A, part B's
  members stay visible. That's safe (errs towards showing), but inconsistent.
- The proposal doesn't say when the class box itself is hidden.
- Fix: state "the class is hidden only if every part is test code", and document the part-B gap.

**M7 · Major · `include` isn't byte-identical to today**
- The output now includes the options and the detector version, so "preserving today's behaviour"
  can't be proven by comparing hashes.
- Fix: in `include` mode, either leave out the header or strip it when comparing. Assert that the
  diagram body matches the existing checkpoint hashes exactly.

## Minor

- **Option order:** repeatability tests shuffle file discovery but not flags. Add a test that
  reordered or duplicated `--test-path` flags produce identical Markdown.
- **Keep vs test:** "keep always wins" means `--keep-path src --test-path src/testutil` keeps
  `testutil`. That's fine and simple. State it and add a test for it.
- **`--test-path` with `--tests include`:** say what happens. I suggest the paths are accepted and
  still checked, but have no effect.
- **Rust "shadowed `test`":** define it narrowly: an explicit `use …::test` or a local
  `macro_rules! test`. If `use super::*` counts as possible shadowing, detection stops working in
  the most common test pattern.
- **Diagram node IDs:** if IDs or collision suffixes come from the set of classes shown, hiding a
  test `Config` renames the application's `Config_2`. Build IDs from the full index instead.
- **Explanation list:** listing every hidden item could run to hundreds of lines for Ferox. Default
  to counts per file and per rule.
- **Separate view:** a class that has test members *and* is referenced by tests should appear once,
  as a partial box, not also as a reference box.
- **Detector version:** say that export never recalculates evidence. Maps built by an older
  detector render with the evidence and version they were saved with.

## Deliberate limitations (fine as written)

These are clearly documented and aren't problems:
- compound `cfg` conditions, `cfg_attr` and `tokio::test`
- Cargo target and Maven root evaluation
- pytest naming conventions, C# global usings, TypeScript `.each`
- C/C++ macros, listing anonymous tests, indirect inheritance, `.mjs`/`.cjs`

Leaving `cfg(any(test, …))` visible is correct. `cfg(all(test, …))` could safely count as test
later.

## Claims that need checking against the source

- V1: `mermaid.rs:47` merges impls, removes duplicates and groups before filtering could run.
- V2: `mermaid.rs:202` looks up calls on unfiltered members.
- V3: the saved JSON keeps a source file for each occurrence: merged Rust impls, Go receiver
  methods, C# partial parts, C++ declaration vs out-of-line definition. B1 depends on this.
- V4: saved edges record the member they came from (M3).
- V5: the TypeScript/JavaScript extractor produces *any* declarations inside callbacks. If not, the
  TS/JS rule does nothing.
- V6: node IDs are independent of which classes are shown.
- V7: VB parser recovery behaviour. SurelyCRM's `<Using Include="Xunit"/>`. Ferox's async tests sit
  inside a `cfg(test)` module. `.mjs`/`.cjs` aren't registered.
- V8: tests can inject a file discovery order (needed for the shuffle check).
- V9: `declarations.rs:12/30` and `parsing.rs:65` are as described.

## What to cut

1. Framework rules for Python, C#, VB.NET, Java, TypeScript and JavaScript. Keep the fixtures as
   design notes for later.
2. Move the separate view to its own later release. Include and exclude cover the core need.
3. Replace the full explanation list with counts.

What stays is the real requirement: evidence per occurrence, path rules, the Rust and Go rules,
old-map handling, and the determinism tests.

## Recommended first slice

1. **Slice 1:** the evidence model per occurrence, `--test-path`/`--keep-path`, and
   `include`/`exclude`, with B1, B2 and M7 fixed. Filter calls and edges per occurrence (M3, M4).
   Path fixtures for all ten languages, including C++ declaration/definition splits and B3.
2. **Slice 2:** Rust (`#[test]`, literal `cfg(test)`) and Go (`_test.go`), with Ferox and a small
   Go fixture as checkpoints.
3. After using it on your real repos, decide whether a framework rule or `--test-suffix` is worth
   adding.
