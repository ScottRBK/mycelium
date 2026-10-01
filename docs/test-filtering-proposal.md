# Test filtering proposal

Status: Scott approved the reduced first pass on 2026-09-30 and the remaining language rules on
2026-10-01. Detector version 2 implements the bounded framework rules described below; C/C++ retain
explicit paths. [Mermaid export](mermaid-export.md) documents exact current behaviour and limits.
This document preserves the original design rationale; separate views and suffix selection remain
proposals.

Mycelium should offer a diagram of application code with recognised test code removed, while
keeping the complete analysis map. The same saved facts should also support a combined view and
a separate test view. Detection must be explainable, conservative, and reproducible.

The recommended first release is explicit path settings for every language, plus narrow Rust and
Go rules. Uncertain code stays visible. Framework rules and a separate test view are later options,
documented below for each language rather than included in the first implementation scope.
Supporting a language does not mean reproducing every test runner's discovery behaviour.

Opus 5.5 reviewed the first draft and returned **ready with revisions**. This draft incorporates
the review responses below. See the [review report](reviews/test-filtering-opus-5.5.md).

## Behaviour and controls

Proposed export options, shared by the native CLI, Python CLI, and Python API:

| Option | Proposed behaviour |
|---|---|
| `--tests exclude` | Default. Hide declarations with accepted test evidence. |
| `--tests include` | Include all declarations, preserving today's selection behaviour. |
| `--tests separate` | Later option: application and test sections in the same Markdown. |
| `--test-path PATH` | Repeatable repository-relative file or directory declared to be tests. |
| `--keep-path PATH` | Repeatable file or directory whose declarations must stay visible. |

These are proposed interfaces, not commands available today. Paths use the existing `--path`
normalisation: forward slashes internally, optional leading `./`, no absolute paths or `..`.
Directory matching uses path components, so `tests` cannot match `testsupport`. Start with exact
files and directory prefixes, without introducing another configuration file or glob language.
Matching is case-sensitive on every operating system. Keep rules take priority over test rules
and automatic evidence; conflicting rules produce an explanation rather than depending on order.
For example, `--keep-path src --test-path src/testutil` keeps all of `src`, including `testutil`.
Sort and deduplicate option values, so repetition or argument order cannot change output.

Validate test/keep paths against the saved file inventory, before applying the `--path` scope.
A typo matching no analysed file is an error. A matched file with no extracted declarations is
valid, and a rule outside the selected scope produces a notice rather than an error. In include
mode, rules are validated but do not remove anything. Analysis exclusions cannot be undone here.

For tests scattered alongside source, individual file paths can become cumbersome. Before adding
framework inference, consider a repeatable `--test-suffix .test.ts`: an explicit user-selected
filename suffix, with keep paths still winning. This is an optional follow-up, not an automatic
naming convention or a required part of the first release.

The existing `--path` scope still applies. Keep rules cannot add files outside that scope or files
omitted during analysis. Classification happens on saved facts plus explicit export options;
export never rereads the checkout. Exclude/separate output records normalised options and the
saved detector version. Include mode preserves existing Markdown bytes, without adding a filter
header or diagnostics to the document; option-validation notices belong on stderr.

Examples and benchmarks remain included unless explicitly selected as test paths. Go test files
are an exception: their test-only build role also covers benchmarks and examples inside them.

## Evidence and boundaries

Accept a declaration as test code when it has one of these reasons:

- An explicit test-path rule, unless a keep-path rule overrides it.
- A supported language feature that places it in test-only code.
- Later: a supported, unambiguously identified framework marker on that declaration.
- Lexical containment inside a scope already established as test-only.

A familiar filename, directory, class suffix, assertion call, or installed test package is not
enough on its own. Do not score guesses or use the existing heuristic call resolver to identify
framework names. Detection does not mean a test runs, passes, is selected, or supplies coverage.

Future framework rules would use syntax plus a small explicit import/alias table. Support direct
qualified references and direct imports with straightforward aliases. Check local declarations
and rebindings that could shadow them. This includes same-namespace types in other files; do not
finalise framework evidence before checking the complete declaration index. Missing context keeps
the candidate unresolved. Wildcard imports, unresolved aliases, re-exports, generated
imports, custom annotations, and uncertain bindings remain unclassified unless a path rule applies.
Package presence alone never labels a whole project. Do not execute builds, imports, test runners,
configuration scripts, macros, or discovery hooks.

Scope matters:

- A test method labels that method, not its whole class or file.
- A direct test-container marker can label the container's own members. Nested types with their
  own identities remain independent unless the enclosing scope is language-level test-only or
  selected by a test-path rule. A nested DTO may be shared with application code.
- A helper inside a Rust test-only module inherits its status. An external helper merely called
  by a test does not. Neither imports nor calls propagate test status.
- A test-only implementation block labels its methods and implementation relationship, not the
  application struct it extends. C++ definitions and declarations need the same care.
- Mixed classes remain in the application view with retained members. An empty ordinary type is
  still a type; an artificial module box with no retained members can disappear.
- A supported marker on a skipped/disabled test still identifies test code. Runtime selection is
  outside this feature.

If a file's syntax tree contains any error or missing node, disable all syntax-derived evidence
for that file and report incomplete detection. A missing closing brace must not let parser recovery
hide application code under an apparent test module. Explicit paths and Go's filename rule remain
usable because they do not depend on the recovered syntax. Do not claim that no tests exist.
Full language binding would require compilers and is outside Mycelium's current architecture.

## Rules for each language

The table distinguishes the first release from future automatic detection. Every row supports
explicit test and keep paths in the first release. Framework entries below are future design
options that need their own implementation decision and binding tests.

| Language | Automatic evidence and delivery scope |
|---|---|
| Rust | Built-in `#[test]`; literal `#[cfg(test)]` on a declaration or enclosing scope. |
| Go | Files ending in `_test.go`, excluding Go's ignored leading `.` or `_` filenames. |
| Python | Later: direct `unittest.TestCase` inheritance and bound pytest fixture decorators. |
| C# | Later: direct MSTest, NUnit, and xUnit markers with verified framework bindings. |
| VB.NET | Later: the same framework rules through VB syntax and identifier casing. |
| Java | Later: direct JUnit 4 and Jupiter annotations with verified framework bindings. |
| TypeScript | Later: direct test/suite callbacks bound to supported testing imports. |
| JavaScript | Later: the same callback rules, with a limited literal CommonJS import form. |
| C | Explicit paths initially; no automatic whole-file classification from test macros. |
| C++ | Explicit paths initially; no automatic whole-file classification from test macros. |

### Rust

Recognise built-in `#[test]` on its function, and literal outer or inner `cfg(test)` attributes on
items/scopes. Inherit the latter through inline modules, nested types, fields, and impl blocks.
Retain source evidence before impl blocks are merged with their owners. A locally shadowed `test`
attribute must not be assumed to be the built-in attribute.

Initially leave compound predicates, `cfg_attr`, proc-macro wrappers, and external module-file
propagation unresolved. In particular, `cfg(any(test, feature = "tools"))` does not establish
test-only code, and `cfg(not(test))` means the opposite. `tokio::test` can be a later bounded rule;
Ferox's enclosing `cfg(test)` module already handles its current inline async tests.

Use an explicit `--test-path tests` for integration tests and helpers. Do not infer the whole
directory from Cargo defaults: targets can be customised, disabled, or share source. `test = true`
on a library does not make the library test-only. Cargo-target evaluation is deferred.

Regression cases: inline module helpers disappear; the owner of a test-only impl remains; mixed
source keeps production functions; compound/negative cfg stays visible; external modules require
a path override; examples and application methods called by tests remain. A `macro_rules! test`
declaration must not suppress recognition of the built-in test attribute.

Sources: [Rust test attributes][rust-test], [conditional compilation][rust-cfg],
[Cargo targets][cargo].

### Go

The `_test.go` suffix has a defined build role: normal package builds omit those files and test
builds include them. Mark their extracted declarations as test code, including helper structs,
benchmarks, examples, and methods. This works for both same-package and external-package tests.

Do not classify `TestConnection` in an ordinary `.go` file by its name. Do not treat build tags,
package names, or a `testdata` directory as proof by themselves. Ignored filenames beginning with
`.` or `_` remain outside this automatic rule; existing source-scan limitations remain explicit.

Regression cases: receiver methods from a test file disappear without removing their production
owner; `_test.go` helpers disappear; ordinary Go files and misleading names remain.

Source: [Go build and test rules][go].

### Python

The bounded rules below are implemented in detector version 2; explicit paths remain available.

A class directly inheriting an unambiguously bound `unittest.TestCase` is a test container.
Direct imports and aliases are supported; transitive inheritance is deferred. Its own helper and
lifecycle methods inherit that status. A directly bound `pytest.fixture` decorator labels the
decorated function as test support, including when decorator arguments are present.

Plain pytest `test_*` functions and `Test*` classes require explicit test paths initially. Their
names are configurable, and hooks can customise collection. `conftest.py`, imports of pytest,
assert statements, and a `test_harness` directory do not independently make a file test-only.
Do not treat arbitrary `pytest.mark.*` decorators as discovery markers.

Regression cases: direct and aliased TestCase; a local `TestCase` lookalike; rebound `unittest`;
fixtures beside application functions; custom pytest names removed through a path setting; shared
helpers kept through an exception. Include a local module shadowing a standard-library import.

Sources: [unittest][unittest], [pytest collection][pytest], [pytest fixtures][pytest-fixtures].

### C Sharp

The bounded rules below are implemented in detector version 2; explicit paths remain available.

Recognise framework-qualified or unambiguously imported MSTest `TestClass`/`TestMethod`, NUnit
`TestFixture`/`Test`/`TestCase`/`TestCaseSource`, and xUnit `Fact`/`Theory`. Handle the optional
`Attribute` suffix. Explicit class markers label the class's own members; method markers label
only methods. The initial rule does not promote an xUnit class simply because it contains a Fact.

Support direct file-local `using` directives and simple aliases. Global imports, imports generated
from project `<Using>` items, custom derived attributes, inherited fixtures, and conditional MSBuild
evaluation are deferred. Use explicit paths for those cases and for fixture/helper classes.
An `IsTestProject` property or a test dependency alone does not identify every source file's role;
linked source can belong to both application and test projects.

SurelyCRM's pinned test project uses `<Using Include="Xunit" />` in its project file, so its
initial checkpoint must use a test path. This is a deliberate limit, not a claim that direct
attribute spelling proves its binding. Partial declarations are classified by source part;
retaining one part must not accidentally restore excluded members from another part.
For a logical partial type, hide the whole type only if every source part is excluded. Current
separate source-part boxes need not be merged by this feature. A class marker on one part therefore
does not automatically classify other parts, even though the language applies it to the type.

Regression cases: qualified/aliased markers; a local FactAttribute; mixed methods; partial types;
shared linked files; project-generated imports requiring explicit paths.

Sources: [MSTest markers][mstest], [NUnit fixtures][nunit], [xUnit tests][xunit].

### Visual Basic NET

The bounded rules below are implemented in detector version 2; explicit paths remain available.

Apply the C# framework policy using VB attribute syntax, `Imports`, aliases, `Global.` names, and
case-insensitive identifier matching. Identifier matching must not alter path matching. Test
containers and members carry the same scope rules as C#.

Project-level imports, custom attributes, inherited fixtures, conditional compilation, and partial
type composition receive no extra inference. Existing VB grammar recovery can leave attributes
ambiguous; preserve such declarations with a diagnostic and allow explicit paths.

Regression cases: `<TestClass>` and `<TestMethod()>`; mixed-case aliases; a local marker lookalike;
partial classes; malformed/recovered attribute syntax; path casing remains consistent across OSes.

The marker semantics come from [MSTest][mstest], [NUnit][nunit], and [xUnit][xunit]; VB syntax
handling must be verified against small parser fixtures before enabling each rule.

### Java

The bounded rules below are implemented in detector version 2; explicit paths remain available.

Recognise direct JUnit 4 `org.junit.Test` and Jupiter's `Test`, `ParameterizedTest`, `RepeatedTest`,
`TestFactory`, and `TestTemplate` annotations by their complete package identity or a direct import.
Do not classify an unrelated `@Test`. The enclosing class remains unless separately configured as
test code. A factory is one source method, not a set of inferred runtime-generated tests.

Maven/Gradle test roots, wildcard imports, composed annotations, inherited tests, JUnit 3 base
classes, and TestNG are deferred to explicit paths. Do not run Gradle or interpret build scripts.
Lifecycle methods without their own accepted test evidence stay visible unless covered by a path.

Regression cases: Jupiter and JUnit 4 imports; custom Test annotation; wildcard ambiguity; dynamic
test factory; a mixed class; fixture base class remaining visible without an explicit path.

Sources: [JUnit annotations][junit], [JUnit 4 Test][junit4].

### TypeScript

The bounded rules below are implemented in detector version 2; explicit paths remain available.

Recognise direct callback positions for `test`/`it` and `describe`/`suite`, where the callee binds
to an explicitly supported export from `node:test`, `vitest`, or `@jest/globals`. Each framework
gets its own exact supported names and callback positions. Handle direct import aliases; do not
assume a bare global `test` belongs to a framework. Never propagate the role into an arbitrary
named callback passed by reference, because that function may have other uses.

Only existing declarations lexically inside a recognised inline callback receive test evidence.
Importing a test function or having one test call does not label the whole file. Inline test
callbacks are not currently first-class class-diagram members; this feature must not invent them
or claim a complete list of test cases. Paths remain the useful way to exclude entire test files.

Defer `.each`, chained modifiers, custom wrappers, global APIs, re-exports, Playwright/Cypress,
and executable Jest/Vitest configuration. `*.test.ts`, `*.spec.ts`, and `__tests__` alone remain
conventions. Their frameworks allow configurable discovery.

Regression cases: imported alias; shadowed test parameter; a production `test()` method; an inline
helper class; a shared named callback; exported application utilities in a mixed file.

Sources: [Node test runner][node], [Vitest API][vitest-api], [Vitest file selection][vitest],
[Jest API][jest-api], [Jest configuration][jest].

### JavaScript

The bounded rules below are implemented in detector version 2; explicit paths remain available.

Use the same framework and scope rules as TypeScript. Additionally recognise simple literal
CommonJS bindings such as `const { test: check } = require('node:test')`, only when `require` and
the bound name are not shadowed or reassigned. Dynamic `require`, computed properties, and imported
wrappers remain unresolved. Syntax detection must select the JavaScript grammar by extension.

Initially cover registered `.js` and `.jsx` files. `.mjs` and `.cjs` are not currently registered
with the pipeline; adding them would be a separate change rather than a hidden part of filtering.

Regression cases: ES modules; CommonJS alias; local require function; reassigned binding; mixed
application/test file; a JS-only fixture so shared TS code cannot mask a JS regression.

Source: [Node test runner examples for both module systems][node].

### C

Use explicit test and keep paths initially. There is no universal C test marker. Frameworks such
as cmocka register functions; a name like `test_connection`, an assertion, or a framework include
does not establish that all declarations in the file are test-only.

Recognising direct cmocka registration could be a later rule, but aliases, wrapper macros,
preprocessor branches, and shared function pointers need their own design. Do not expand macros,
evaluate CMake, or relabel included headers in this feature. This is usable filtering for C with
explicit configuration, without promising automatic discovery.

Regression cases: configured test translation unit; shared header retained; mixed file retained
unless explicitly selected; a function called `test_connection` that is application code.

Source: [cmocka test registration][cmocka].

### C Plus Plus

Use explicit paths initially. GoogleTest's `TEST`/`TEST_F` and Catch2's `TEST_CASE` are macros;
their generated classes/functions are not reliably represented by today's declaration model.
Seeing a macro or `testing::Test` base must not label an entire translation unit, shared base, or
header. Do not synthesise missing test classes as part of filtering.

Framework-aware macro extraction and registration are separate future work. Keep declarations
and out-of-line definitions distinct until their role is known. If an exported member has both
retained and test-only source occurrences, preserve it in the application view and filter calls
by their actual occurrence; do not let one excluded definition erase a retained declaration.

Regression cases: configured test file; shared fixture header with a keep override; same-named
application macro; prototype/definition split; a production class with test-only extra methods.

Sources: [GoogleTest fixtures][gtest], [Catch2 test cases][catch].

## Changes to the existing design

Keep detection in Rust core and keep the wrappers thin. This is a proposed change list, not an
instruction to start implementation.

1. Extend declaration facts with optional test evidence on classes, members, and base/implementation
   relationships. Evidence needs a stable rule identifier and repository-relative source location.
   `Class` and `Member` currently lack this information in `declarations.rs`, lines 12 and 30.
   Members already keep their own source files. Bases currently have no independent location
   (`declarations.rs:23`); attach the enclosing source-part/impl origin before merging. Keep
   duplicate relationships with different origins until filtering, then deduplicate retained ones.
2. Capture evidence during extraction, where syntax and enclosing scopes still exist. The parsing
   phase already passes the syntax tree into declaration extraction at `phases/parsing.rs:65`.
   Add a small shared policy module; keep syntax recognition in the language-specific visitors.
3. Store a detector version and deterministic diagnostics in `ClassDiagram`. Absent metadata means
   an old map, not a completed analysis that found no tests. A supported older version renders its
   saved evidence without recomputing it. Unknown versions use the missing-evidence fallback.
4. In `mermaid.rs:47`, select retained source occurrences before impl merging, member deduplication,
   relationship grouping, or display limits. Retain a full identity index for resolution, but never
   turn a previously ambiguous reference into a unique one merely by removing tests.
   Assign Mermaid node IDs from the full sorted identity index within the selected path scope,
   before test filtering, so a retained type keeps its ID across include/exclude/separate modes.
5. Filter calls at member/occurrence level. The current lookup at `mermaid.rs:202` iterates members
   from the unfiltered class collection, which would otherwise restore calls from hidden methods.
   Filter signature/field edges and implementation relationships by their originating evidence too.
   In exclude mode, suppress every edge with a hidden endpoint, including calls from retained code
   to hidden code. Count intentional removals separately from unresolved endpoints in every mode.
6. Apply matching options in both CLIs and the Python binding. Keep changes to declaration
   model separate from the existing call graph; filtering must not change analysis graph contents.

Show counts by file and rule in the default Markdown summary. An opt-in `--explain-tests` adds a
collapsed per-declaration list containing the source location, rule, and explicit override.
Store underlying evidence in the map even when explanations are not requested. Hidden tests
must not leak into application relationship lists, source indexes, or type/signature keys. Treat
intentionally filtered calls separately from unresolved calls in diagnostic counts.

For an old map with declaration facts but no supported test metadata, include still works and
exclude/separate applies explicit file rules only. Emit one deterministic notice that automatic
detection is unavailable and reanalysis is needed to enable it. Do not infer new Go/Rust evidence
during export. Older maps with no declaration facts retain today's reanalysis error.

The current raw map keeps impl/source parts and member file locations before export merges them;
explicit path filtering can therefore work on those older declaration maps. Validate missing or
malformed occurrence locations and retain uncertain occurrences with a diagnostic. Do not treat
missing origins as belonging to the owner file. An empty filtered scope produces an empty-view
message. Only the existing `--path` selection requires matching declarations; test/keep rules use
the file inventory as specified above.

### Separate test view as a later option

Deliver include/exclude first. The following view design is retained for a later decision.

Application view uses the same filtering as exclude. Test view contains selected test declarations
and direct application types referenced by their recorded calls or declared type relationships.
Referenced application types are labelled reference boxes, with names and source links, without
copying their whole member lists or recursively expanding their dependencies.

Mixed classes can appear in both sections with their respective members. Keep-path declarations
belong to the application view, even when automatic evidence exists. A call remains a possible
static connection, never a claim of test coverage. Unresolved callback/macro tests remain a stated
limitation. If no test declarations exist, omit the empty test diagram and explain why.
Within the test section, a mixed class that is also referenced appears once as a partial member
box; do not duplicate it as an application reference box.

Filtering and separation do not change current diagram size limits. One application section may
still contain several diagrams under those limits; Ferox can keep its chosen `--max-classes 1000`.
Separate is an explicit request for distinct application/test views, not a change to that setting.

## Validation and delivery slices

Use TDD through source -> analysis JSON -> public export, with independent assertions about what
must remain and what must disappear. A snapshot alone can preserve incorrect classification.

1. Shared policy plus explicit paths: positive, negative, overlapping, normalised, and empty cases;
   unchanged analysis facts; old maps; errors and option parity across all public interfaces.
   Include a file with no declarations, an out-of-scope rule, duplicate/reordered flags, and keep
   precedence. Verify include-mode Markdown byte-for-byte against the previous exporter.
2. Rust and Go: inline scopes, cross-file members, impl relationships, occurrence-level calls,
   retained ambiguity, and no automatic propagation through call edges.
   Include parser recovery stretching an inline scope; two same-named methods with different roles;
   matching test/non-test trait impls; visible-to-hidden calls; stable IDs across filtering modes.

These two slices constitute the recommended first release, with path fixtures for all ten languages.
After evaluating it on Scott's repositories, optional later slices are:

3. Literal filename suffix selection for colocated tests, before more complex framework detection.
4. Python, C#, VB.NET, and Java: direct supported bindings and paired misleading-name/alias cases.
   Each framework rule is enabled only after its source fixtures pass.
5. TypeScript and JavaScript: direct imported callbacks, shadowing, mixed files, and current limits
   on representing anonymous tests. C and C++ explicit-path fixtures run from the first slice.
6. Separate view: correct reference boxes, mixed classes, no recursive expansion, no dangling edges,
   stable identities, consistent explanation counts, and rendered output inspection.

Run repeatability checks with different fixture file-creation orders, reordered saved records,
and two fresh analyses. Existing public-seam tests already use the first two techniques; no new
production discovery-order injection hook is needed. Assert identical
Markdown for identical source, detector version, and normalised options. Tests must include both
false positives and missed detections; correctness includes keeping uncertain application code.

Extend the existing local checkpoint source slices at their existing full commits. Keep the old
slices as regression coverage and add filtering slices; do not silently repin repositories.
The following files/layouts were checked at those commits while drafting this proposal:

| Repository | Commit |
|---|---|
| Forgetful | `6618ae56abc718f873d0ae78f0c957f74c05020b` |
| Ferox | `4132d8f3ed8b8a2ecc9809a2682a07b6a4a9c5ad` |
| eval-harness | `ffb0f5b92f9af027d0ce742be7be2d534390a887` |
| SurelyCRM | `edf6e4c13bf22e547db9f5adf3f9230b0ae3d0df` |
| Pi Forgetful extension | `94070d236deaced4242a7c7f2ecb5096cf404360` |
| Pi web search | `b13f874337e4cdcb34993d79800437a6cdac4dd9` |

- Ferox: add `src/adapters/providers/openai_compatible/client.rs` and the integration-test files.
  Prove that inline tests/helpers and configured integration tests disappear while the client stays.
- Forgetful: include a `tests/e2e` slice and `test_harness/config.py`. Configure only the selected
  test directory and prove that the harness configuration remains despite its name.
- eval-harness: include `tests/unit/test_eval_config.py`, `src/models.py`, and an example
  evaluation.
  Preserve evaluation/application types. Use `--keep-path example_evals` so evaluated sample code
  remains even if it contains real Rust or Go tests. A directory containing `test` is insufficient.
- SurelyCRM: include `tests/Sure.Common.Tests/Extensions/StringExtensionsTests.cs`, its project
  file, and production extension source. Prove that explicit paths work with generated Xunit
  imports, while local fixtures separately test direct-attribute recognition.
- Pi Forgetful: include `test/model.test.ts` and `src/model.ts`. Retain the application model;
  exclude test helpers by path without claiming the diagram enumerates every callback test.
- Pi web search: include `agent/extensions/web-search/tests/test_web_search_helper.py` alongside
  the existing TS source slice. This checkpoint is mixed Python/TypeScript, not a TS test fixture.

Use portable fixtures for Go, VB.NET, Java, JavaScript, C, and C++, which these pinned slices do
not adequately cover. Freeze any new real repositories at explicit commits before adding them.
Assert retained/removed names, member ownership, reasons, edges, and counts before recording hashes.
No test runners or application services need to execute for these static-analysis checkpoints.

## Review outcome and remaining choices

Opus reviewed the first draft using proposal text only; its findings are preserved in the linked
report. The author checked relevant implementation claims locally before revising this document.
The revised proposal has not received a second Opus pass.

Accepted revisions:

- Reduce the first release to explicit paths plus Rust/Go, with include/exclude modes. Framework
  methods alone leave fixture shells and fakes visible, and robust name binding needs more work.
- Keep legacy declaration maps usable with explicit rules and a clear automatic-detection notice.
- Clarify that test/keep rules validate files, independently of the main path's declaration scope.
  The review read the original path paragraph more broadly than intended; it was underspecified.
- Disable syntax-based detection for an entire malformed file; add a parser-recovery regression.
- Add an explicit keep rule for evaluation-input fixtures that may themselves contain real tests.
- Preserve per-occurrence relationship origins, suppress hidden endpoints in all modes, preserve
  node IDs, and test flag ordering and include-mode compatibility.
- Use compact counts by default, with detailed explanations requested explicitly.

Source checks confirmed that raw maps preserve impls and member file locations, while bases need
origin tracking before merging. Relationship summaries are generated during export rather than
saved as an already aggregated diagram graph. Current IDs are assigned after path filtering at
`mermaid.rs:129`; test-mode partitioning must happen after identity assignment. Existing tests
already cover record order and file-creation order in `test_mermaid.rs:522` and `:584`.

One reviewer suggestion was rejected after checking the language reference: `macro_rules! test`
does not shadow `#[test]`. Rust resolves bang macros and attributes in separate sub-namespaces.
Add a regression for that distinction; unresolved attribute imports still need careful handling.
Source: [Rust macro sub-namespaces][rust-names].

Scott subsequently approved the reduced first pass and requested Opus review followed by independent
GPT-6.1-sol testing. The remaining languages' framework rules are for a follow-up. Separate views,
framework configuration evaluation, and complete test-case enumeration remain outside this release.

## Sources

Official documentation checked while drafting on 2026-09-30. Context7 was attempted but unavailable;
the linked official sources were used directly. Detection choices above are proposals, not claims
that every framework's full discovery algorithm can be reproduced statically.

[rust-test]: https://doc.rust-lang.org/reference/attributes/testing.html
[rust-cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html
[rust-names]: https://doc.rust-lang.org/reference/names/namespaces.html#sub-namespaces
[cargo]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[go]: https://pkg.go.dev/cmd/go#hdr-Test_packages
[unittest]: https://docs.python.org/3/library/unittest.html
[pytest]: https://docs.pytest.org/en/stable/example/pythoncollection.html
[pytest-fixtures]: https://docs.pytest.org/en/stable/how-to/fixtures.html
[mstest]: https://learn.microsoft.com/en-us/dotnet/core/testing/unit-testing-mstest-writing-tests
[nunit]: https://docs.nunit.org/articles/nunit/writing-tests/attributes/testfixture.html
[xunit]: https://xunit.net/docs/getting-started/v3/getting-started
[junit]: https://docs.junit.org/6.1.3/writing-tests/annotations.html
[junit4]: https://junit.org/junit4/javadoc/latest/org/junit/Test.html
[node]: https://nodejs.org/api/test.html
[vitest]: https://vitest.dev/config/include
[vitest-api]: https://vitest.dev/api/test
[jest]: https://jestjs.io/docs/configuration#testmatch-arraystring
[jest-api]: https://jestjs.io/docs/api
[cmocka]: https://api.cmocka.org/group__cmocka__exec.html
[gtest]: https://google.github.io/googletest/primer.html
[catch]: https://catch2-temp.readthedocs.io/en/latest/test-cases-and-sections.html
