# Independent GPT-6.1-sol test report

Date: 2026-09-30
Workspace: `/home/scott/development/ai/mycelium`

No confirmed functional regressions or blockers were reproduced in the agreed first-pass scope.
All 223 independent executable checks passed after resolving two incorrect probe expectations.
The checks used 49 fresh source fixtures, saved JSON, the native CLI, Python API, and Python CLI.
Repository source, documentation, tests, and memories were left unchanged.

## Execution and evidence

The current native CLI was refreshed with `cargo build -p mycelium-cli`.
Python checks used the rebuilt release binding through `/tmp/mycelium-venv/bin/python`.
The independent scripts and their complete results are saved under:

- `/tmp/mycelium-independent-testing/probe.py`: 86 checks; all passed.
- `/tmp/mycelium-independent-testing/advanced.py`: 137 checks; all passed.
- `/tmp/mycelium-independent-testing/results.json`
- `/tmp/mycelium-independent-testing/advanced_results.json`
- `/tmp/mycelium-independent-testing/probe.log`
- `/tmp/mycelium-independent-testing/advanced.log`

These were new probes, rather than a replay of the repository's filtering tests.
Of the 223 checks, 100 compare shuffled saved records across five export configurations, and six
compare reordered, repeated, normalized selectors. The other 117 cover the cases below.

The parent separately reported passing the existing 465 Rust tests, nine Python tests, lint,
formatting, and pinned checkpoints. I did not independently rerun those suites, and their counts
are not included in the 223 checks above.

## Cases exercised

### Rust evidence and uncertainty

- Bare, raw, aliased, nested, wildcard, late, and conditionally imported `test` bindings.
- Renamed/discarded imports, comments, string literals, and a bang macro named `test`.
- Literal outer and inner `cfg(test)`, crate/module/function/impl scopes, stacked attributes,
  comments between attribute tokens, documentation comments, and nested sibling scopes.
- Enum variants and multiple attributed tuple fields, including restricted visibility.
- Compound predicates, `cfg_attr`, qualified attributes, and raw attribute spellings stay visible
  under the intentionally narrow first-pass detector.
- Wildcard uncertainty leaves bare test functions visible; literal `cfg(test)` still removes them.
- No uncertainty diagnostic appears for a wildcard that prevents no classification.
- Malformed syntax preserves apparent declarations and emits the saved detection notice.
- Saved detector notices respect export scope.

### Paths and language coverage

- A single combined fixture covers Rust, Go, Python, C#, VB.NET, Java, TypeScript, JavaScript,
  C, and C++. Explicit test paths remove the expected boxes, keep paths retain exceptions,
  and component-neighbor paths remain visible.
- Repeated selectors, changed argument ordering, `./`, duplicate separators, backslashes,
  root selectors, and empty source files.
- Absolute, parent-traversing, drive-qualified, unmatched, and case-mismatched selectors fail,
  including when `--tests include` is selected.
- Selectors validate against saved files before scope selection and issue out-of-scope notices.
- Go filename controls include `_test.go`, leading dot/underscore, casing differences, and
  names with additional suffix components.
- Naming conventions in every language, framework-like filenames, and callers of tests do not
  create automatic test roles. Rust integration-test directories need explicit selectors.

### Saved-map export and relationships

- Export succeeds after deleting the complete source fixture directory.
- Export leaves both JSON files and Python analysis dictionaries unchanged.
- Missing detector metadata and future detector versions retain automatic-test occurrences,
  accept explicit selectors, and issue the reanalysis notice.
- Unknown evidence rules, invalid evidence files/lines, missing occurrence files/lines, and
  zero occurrence lines remain conservative. Missing declarations produce the reanalysis error.
- Same-line duplicate type identities retain distinct boxes and stable IDs after filtering.
- Twenty shuffled versions of classes, members, bases, files, symbols, calls, warnings, and
  diagnostics preserve exact Markdown in each of five configurations.
- A kept Rust impl retains its excluded owner's shell, its own members, its base relationship,
  and its argument relationship while excluded owner fields and other impl members disappear.
- A filtered C++ implementation preserves the production prototype and does not inflate the
  removed relationship count.
- A kept Go receiver method joins its production owner without creating a duplicate box.
- Displayed relationship endpoints refer to existing boxes in include and exclude views.
- Box IDs stay consistent between the complete and filtered views of the same scope.

### Compatibility and Python wrappers

- Native `--tests include` matches `/tmp/mycelium-map-before-filtering` byte-for-byte on the
  independently generated combined ten-language map.
- Valid selectors and `--explain-tests` have no filtering effect on include output.
- Python include output also matches that prior exporter exactly.
- Native CLI, Python API, and Python CLI produce identical filtered Markdown on the combined map.
- Python API notices are `UserWarning`; warnings-as-errors raise normally.
- Python CLI warnings-as-errors produce a clean error without a traceback.
- Python validates include-mode selectors and rejects unknown test modes.

## Large-file scaling

The debug native CLI analysed independent Rust files with no call edges. Every function was
retained in the saved declarations. Wall-clock timings from the final run were:

| Functions in one file | Complete analysis wall time |
|---|---:|
| 2,000 | 0.137 seconds |
| 4,000 | 0.272 seconds |
| 8,000 | 0.535 seconds |

The observed increase was close to linear for this fixture. This is a scaling control, not a
general performance benchmark. Full measurements and saved metadata are in
`/tmp/mycelium-independent-testing/scaling.json`.

## Withdrawn namespace concern

An initial probe expected `use custom::test::{self}` to disable bare `#[test]` recognition.
That expectation was too broad: unaliased `self` imports only the type namespace.

I verified this with real compiler executions:

- A local module named `custom::test` plus its `{self}` import compiled successfully;
  `rustc --test` and `--list` identified `probe` as a built-in test.
- A real proc-macro crate exporting an attribute named `test` could not be imported through
  `{self}`; rustc rejected that import with E0432.

The original concern was withdrawn and these cases now serve as namespace controls. Source files
and compiler artifacts are under `/tmp/mycelium-independent-testing/rust_import_compiler/`.
The module import is at `module.rs:5`; its built-in test function is at `module.rs:8`.

The other corrected probe expected an exact unaliased C++ signature display. The exporter correctly
uses its existing `Type1` key for `Item*`; the independent check now verifies the retained source
parameter and relationship rather than imposing a different display format.

## Limits of this result

This report covers the agreed first pass. It does not validate deferred framework detection,
compound Rust predicates, external module propagation, suffix selectors, or separate test views.
It does not establish compiler completeness, runtime test coverage, or general call-resolution
accuracy. It does not independently render all Mermaid blocks or rerun the commit-pinned external
repositories. The timings cover simple functions below the configured source-file size limit.

No fixes were made by this testing agent. No unresolved first-pass failure remains in the
independent probe results.
