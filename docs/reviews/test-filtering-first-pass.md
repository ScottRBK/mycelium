# Test filtering first-pass review

Scope: explicit test/keep paths for all ten languages, include/exclude modes, and narrow Rust/Go
automatic detection. Other framework rules, suffix selectors, and separate diagrams are deferred.

Claude Code reported model `claude-opus-5-5` for both passes, with successful completion events.
Session: `99cc2f31-8962-4541-8c21-b437c0468aa5`.

Both reviews used the filtering changes only, with tools disabled. Opus had no repository access
and did not execute tests. An initial attempt to allow broader read-only repository access was
rejected by automatic approval review; the narrower diff-only review was approved.

First review: three blockers (Rust scan cost, noisy binding notices, and unscoped detection
notices).
These were fixed with regression tests. A duplicate-ID issue was independently reproduced and fixed.
The second review concluded: **The blockers are fixed and the review can close.**

Before closeout, the Ferox filtered hash was updated after verifying its only change was removal
of an unnecessary binding notice. The original six include-mode hashes remain unchanged. Markdown
widths and checkpoint-runner indentation were corrected. Interim development maps in `/tmp` were
regenerated after the unshipped detector diagnostic schema changed; no released format was changed.

Rendered checks covered all 12 filtered checkpoint diagrams, the updated AGENTS export diagram,
and the full Ferox application diagram. Long member names can still wrap/overlap in Mermaid;
full Ferox remains a wide view requiring zoom. Filtering preserves the complete Markdown signatures.

Validation after the review fixes: 465 Rust tests, nine Python binding tests, formatting, Clippy,
and all six commit-pinned repository checkpoints passed. The checkpoint runner verifies both the
original include-mode hashes and filtered output with independent retained/removed declarations.

GPT-6.1-sol then completed independent testing: **223 executable checks passed across 49 fresh
source fixtures, with no confirmed blockers**. This includes 100 saved-record shuffle checks and
six selector-order checks. It verified native/Python parity, prior include-mode compatibility,
saved-only exports, all ten languages' explicit selectors, and Rust/Go evidence. A simple Rust
scaling fixture grew from 0.137 seconds for 2,000 functions to 0.535 seconds for 8,000 functions.
Two incorrect probe expectations were corrected after checking compiler/display behaviour; neither
required a production change. The tester did not rerun the parent's existing suites or checkpoints.
See the [complete independent report](test-filtering-gpt-6.1-sol.md) for evidence and limits.

## Opus first review

Scottesh, this review is based on the diff only. I read no other source and ran nothing. Where a
finding depends on code the diff doesn't show, I say so.

## Fix before freezing the filtered hashes

**1. Medium-High: repeated scanning makes big Rust files very slow to analyse.**
`test_detection.rs:98-107`
- **Problem:** for every member, the code walks up to the file root. At each level it scans *all*
children looking for `#![cfg(test)]`. The cost grows with members × top-level items.
- **Repro:** a committed bindgen-style `bindings.rs` with about 50k items. `analyze` visits billions
of nodes.
- **Fix:** inner attributes must come first in a block. Stop scanning at the first child that isn't
a comment or an `inner_attribute_item`.

**2. Medium: wildcard imports flood the output with notices.** `test_detection.rs:26`, `mod.rs:~31`
- **Problem:** `use super::*;` inside a `#[cfg(test)] mod tests` marks the whole file "uncertain".
Almost every Rust file with unit tests has this. Each one adds a notice to stderr and to the
Markdown, even though nothing was missed: `cfg(test)` already catches those tests.
- **Repro:** `fn run(){}\n#[cfg(test)] mod t { use super::*; #[test] fn a(){} }` prints "binding is
uncertain".
- **Fix:** only record the notice when a `#[test]` was actually skipped (lines 84-86) and no
`cfg(test)` caught that item. For example, set a flag and push the notice after `rust_walk`.

**3. Medium: notices ignore `--path`, and the saved map stores them as plain strings.**
`filtering.rs:64`
- **Problem:** every export's summary includes notices for the whole repo. So `--path src/a` output
changes when an unrelated file changes. This can't be filtered, because v1 saves a `Vec<String>`.
- **Fix:** change the saved format now, before v1 is frozen. Save `{file, message}` and filter with
`matches_path(file, scope)`.

**4. Medium (verify): two boxes can get the same box ID.** `mermaid.rs:~133`
- **Problem:** `stable_ids` is keyed by `Class.id`, which is `file:line:kind:name`. Two same-named,
same-kind classes on one line share an ID. Both boxes get the same `cNNNN`, so Mermaid merges them,
and one number is skipped. The old code numbered boxes by position, so for these inputs this also
breaks the "include output unchanged" promise.
- **Repro:**
  - C++: `namespace a { struct S{}; } namespace b { struct S{}; }` on one line.
  - Minified JS with two scoped `class A{}` on one line.
  - Rust: two unresolved `impl X for Missing` on one line.
- **Caveat:** this depends on extractors I can't see.
- **Fix:** make IDs unique at extraction (add the column), or key stable IDs by (id, occurrence
number).

**5. Medium (verify): "Type relationships removed" may compare two different things.**
`mermaid.rs:~180`
- **Problem:** `edges` is built from `classes` *after* the unchanged member-processing loop.
`full_edges` is built from unprocessed `scoped` clones. If that loop removes duplicates, rewrites or
truncates members, the count includes edges that filtering never removed. I can't see the loop.
- **Fix:** process both inputs the same way.

## Lower severity

6. **Location checks run even with no rules** (`filtering.rs:93`). In the default mode, with no test
paths and no evidence, every item is still validated. Any extractor that saves `line: 0` would add
"invalid source location" notices to every export. Fix: return early when no path matches and
`evidence` is None.
7. **A keep path doesn't win for an impl whose owner is hidden** (`merge_classes`: `if let
Some(owner)=…find` has no else). Repro: `--test-path tests --keep-path tests/support.rs`, with
`struct Fake` in `tests/fake.rs` and `impl Fake { fn build() }` in `tests/support.rs`. `build()`
silently disappears and isn't counted. Fix: warn and count it, or fall back to an `unresolved_impl`
box.
8. **Misleading "No declarations remain after test filtering"** (`mermaid.rs:~278`). A default
export of a map with no declarations says this even though nothing was filtered. Fix: also require
`!scoped.is_empty()`.
9. **Counts use the item's file, but scope uses the owner's file** (`filtering.rs:120`). With
`--path src`, a test impl in `tests/` attached to a `src` type is hidden but not counted.
10. **Enum variants never get test evidence** (`enum_member` sets `test: None`). In `enum M { A,
#[cfg(test)] B }`, B is shown. This errs on the safe side; document it or pass the node.
11. **A bad selector gives a generic error.** `--test-path ../x` returns `InvalidOptions`. Your test
implies that message mentions max_classes, which is confusing. Fix: add an error variant that
carries the path.
12. **Hidden test files still appear in "Extraction warnings"**, e.g. `tests/x.py: syntax errors`.
Decide whether to filter these.
13. **Python treats warnings as errors under `-W error`.** `warnings.warn` then raises, and the
Python CLI only catches OSError/ValueError/TypeError, so the user gets a traceback.
14. **CLI test temp dir (verify)**: `mycelium-cli-tests-{pid}`. If the existing test in that file
uses the same name, the two tests share a PID when run in parallel and one deletes the other's
folder. Use `tempfile`.

## Nits
- `malformed()` repeats `root.has_error()`, which I believe already covers MISSING nodes.
- The detector version `1` is hard-coded three times. Use one constant.
- The `unreachable!()` arm in `Display` can go if every arm is matched.
- The CLI converts the `--tests` string by hand even though `FromStr` exists.
- Path normalisation now exists twice (`prefix` and `normalize_path`). If they differ, `--path` and
the selectors accept different input.
- In the checkpoint script, `contains` normalises whitespace but `absent` doesn't. Make them match.
- `docs/mermaid-export.md` has broken line-wrapping: "repeatability" and "evaluation" sit alone on
their own lines.
- The module-drop rule (`filtering.rs:164`) also drops modules that were empty before filtering.
Harmless if these generated modules are never empty; I can't confirm that.

## Deliberate limits, not bugs
- Wildcard imports and `use … test` turn off `#[test]` detection.
- External `mod x;` isn't followed.
- Combined cfg conditions, `cfg_attr` and `tokio::test` aren't recognised.
- Go files are judged by name only, so `_test.go` files under `testdata/` or `_dir/` still count as
tests.

## Looks right
- Selection happens on raw source items before impls are merged.
- Impls are matched against the full set of owners.
- Name ambiguity is kept.
- Hidden base classes don't warn.
- A call arrow needs both ends kept.
- Include mode returns the raw input with no summary.
- Selectors are checked in both modes.
- A failed export doesn't overwrite the output file.

## Verdict
The design matches the approved scope, and the core selection logic looks sound. It's **not ready to
freeze the filtered hashes**.
- **Blockers:** 1, 2 and 3. They affect analyse time, the saved v1 format and the Markdown bytes.
- **Check before sign-off:** 4 and 5.
- **Can follow later:** the rest.

**Suggested tests for GPT-6.1-sol:**
- Time `analyze` on a large Rust file.
- A file with `use super::*` should produce no notice.
- A `--path` export shouldn't change when an unrelated file's diagnostics change.
- Same-line duplicate classes should get distinct box IDs.
- An empty map in default mode shouldn't claim declarations were filtered.


## Opus follow-up

Scottesh, this is again based on the diff only. I read nothing else and ran nothing.

## My earlier blockers
- **1. Slow scan on big Rust files: resolved.** The inner-attribute scan now skips comments and the
shebang, then stops at the first real item. Rust only allows inner attributes before items, so
nothing valid is missed.
- **2. Notice spam from wildcard imports: resolved.** A skipped `#[test]` is only recorded once the
walk has passed every ancestor without finding `cfg(test)`. It finds `#[test]` first on
`#[cfg(test)] #[test] fn` too, and this still works.
- **3. Notices ignoring `--path`: resolved.** Diagnostics are now saved as file plus message and
filtered by `--path`.
- **4. Duplicate box IDs: resolved.** Colliding IDs are made unique from the complete records before
filtering, so box IDs match between include and exclude. Maps without collisions keep their existing
IDs, so their include output is unchanged.
- **5. Removed-relationship count: accepted** on your evidence plus the new regression test.

The other fixes also look right: the empty-view message (8), the owner-based counts (9), `--path`
now uses the same path normaliser (the old checks matched it), and the new `Display` code.

## Loose ends (none block the review)
1. **Ferox hash, before close.** You say its filtered hash still has to be updated. Until then, the
new required `sha256` check fails that checkpoint.
2. **Low: maps saved by the interim build won't load.** Those maps stored `diagnostics` as plain
strings under version 1. They now fail to load outright instead of falling back. This is fine if no
such maps were shared. If some were, bump the version or accept both shapes.
3. **Low: the owner-shell warning ignores `--path`.** It's added inside `merge_classes`, before
scope filtering. So `--path src --test-path tests --keep-path tests/support.rs` shows a warning
about a box outside `src`. The existing "Unresolved implementation" warning already behaves this
way, so it's consistent, not new breakage.
4. **Low: `unique_occurrences` re-sorts members.** For colliding classes it sorts `members` and
`bases` in place. If the display keeps source order, those boxes now list members alphabetically.
Suggest sorting a copy used only as the tie-break. These inputs were broken before, so this isn't a
regression.
5. **Nits:**
   - Three new lines in `docs/mermaid-export.md` are well over 100 columns.
   - In `check_mermaid.py`, the second `path.write_bytes(` block is indented wrongly. It's still
valid Python, just misleading.
   - A `filtering` entry without `sha256` would now raise a `KeyError` that the runner doesn't
catch.

## Regressions from the fixes
I found no concrete one. The owner shell only appears in exclude mode, when an impl is kept but its
owner is hidden. The full merge and include mode never trigger it, so the include output is
unchanged.

## Verdict
**The blockers are fixed and the review can close.** Update the Ferox hash and fix the doc line
widths before merging. GPT-6.1-sol can start its tests.


## Review payloads

- `mycelium-first-pass.patch`: `8d38ca039b1021903b6ee8939060c5394d7a34902ed517a14260a2914ad6093c`
- `mycelium-first-pass-followup.patch`:
`76c1c98ff726b966e23c1a8259a7324dc27ddb8ba8762c8edbc9ea34d470603f`
