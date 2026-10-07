# Mermaid class diagram export

Analyse once, then export the saved facts with either the native or Python CLI:

```bash
mycelium-map analyze /path/to/repo -o map.json --quiet
mycelium-map export map.json --format mermaid -o diagrams.md
mycelium-map export map.json -o services.md --path src/services --max-classes 6
```

`--path` selects a repository-relative file or directory. It accepts leading `./` and normalizes
backslashes. A nonempty scope with no declarations reports an error. The default hides declarations
with supported test evidence. It does not select “important” classes or ask an agent to
summarise them. Large exports remain large documents:

- Eight boxes by default, configurable with `--max-classes`.
- Every box keeps all its members in the same diagram, with no member cap.
- Every resolved connection between boxes in a diagram is drawn, with no connection cap.
  Parallel arrows are summarized to avoid overlapping labels.
- A complete list of resolved relationships, including those crossing diagram boundaries.
- Type and signature keys for notation that would be too long or unsafe inside Mermaid.

For one diagram, set `--max-classes` to at least the number of included boxes (for example, 1000).

Class, module, method, and field names are shown in full, including module file paths.
Names, ordering, partitioning, and abbreviations follow fixed rules. Timestamps, local repository
paths, timings, community assignments, and process traces do not enter the Markdown. Given the
same extracted declarations, recorded calls, exporter version, and options, the output is identical.
Analysis walks files in sorted order, and Java/Go import fallbacks use stable path ordering.
These remain heuristic choices; stable ordering makes them repeatable rather than compiler-proven.
The repository checkpoints also run fresh analysis twice, so they exercise more than rendering.

```python
from mycelium import analyze, export_mermaid

result = analyze("/path/to/repo")
markdown = export_mermaid(result, path="src/services", max_classes=6)
```

## Test filtering

Both CLIs support these options:

```bash
# Recognised test declarations are filtered automatically in new maps.
mycelium-map export map.json -o application.md --test-path tests
# Explicit exceptions win, including against a broader test path.
mycelium-map export map.json -o application.md --test-path tests --keep-path tests/shared
# Restore the complete pre-filtering view.
mycelium-map export map.json -o everything.md --tests include
# Add a collapsed list of source locations and selection reasons.
mycelium-map export map.json -o explained.md --test-path tests --explain-tests
```

`--tests exclude` is the default. Repeat `--test-path` and `--keep-path` for additional files or
folders. Paths are repository-relative, case-sensitive on every OS, and match whole components:
`tests` does not match `testsupport`. Leading `./`, duplicate separators, and backslashes normalize;
absolute paths and `..` are rejected. `.` selects every analysed file. A selector matching no saved
file is an error; an empty source file is a valid match. Keep rules always win, regardless of order.
Rules cannot restore files excluded from analysis or outside `--path`. Out-of-scope rules produce
notices. Both include and exclude validate selectors; include ignores their filtering effect.

Automatic evidence is deliberately narrow:

- Rust: literal outer/inner `cfg(test)` and built-in `#[test]`. Inline test scopes include their
  helpers, fields, and impls. Compound predicates, `cfg_attr`, async framework attributes, and
  external module propagation are deferred. Any malformed syntax disables syntax detection for
  that file. An import binding `test`, or a wildcard import anywhere in the file, conservatively
  disables `#[test]` recognition for that file; `cfg(test)` still works. A bang macro named `test`
  does not shadow the attribute.
- Go: filenames ending in `_test.go`, excluding names beginning with `.` or `_`. All declarations
  in those files have a test-only build role, including helpers and receiver methods. Names such
  as `TestConnection` elsewhere do not establish that role.
- Python: direct `unittest.TestCase` inheritance and `pytest.fixture` decorators, including direct
  imports and aliases. A container's own fields/helpers are tests; nested types remain independent.
  Pytest naming conventions, marks, transitive inheritance and `pytest_asyncio` are not inferred.
- C#/VB.NET: direct MSTest `TestClass`/`TestMethod`, NUnit
  `TestFixture`/`Test`/`TestCase`/`TestCaseSource`, and xUnit `Fact`/`Theory`. Namespace imports,
  simple aliases, qualified names and the optional `Attribute` suffix are supported. VB identifiers
  are case-insensitive. Method markers hide only their methods; explicit class markers also hide
  the class's own members. Nested types and other partial declarations remain independent.
- Java: direct JUnit 4 `org.junit.Test` and Jupiter `Test`, `RepeatedTest`, `TestFactory`,
  `TestTemplate`, and `org.junit.jupiter.params.ParameterizedTest`. Direct imports and fully
  qualified annotations are supported. Wildcard imports do not establish framework identity;
  ordinary static assertion imports do not prevent an explicit annotation import from matching.
  Composed annotations, lifecycle methods and inherited tests remain unclassified.
- TypeScript/JavaScript: direct named imports of `test`/`it`/`describe` from `node:test`, `vitest`,
  or `@jest/globals`, plus `suite` from Node/Vitest and Node's default test import. Only existing
  declarations inside supported inline callbacks receive evidence; anonymous tests are not added
  as diagram members. Node uses the final argument (one to three arguments), Jest the second
  argument, and Vitest the second or third after a literal options object. Named callbacks,
  namespace imports, chained modifiers, wrappers and globals remain unclassified. JavaScript also
  supports file-level `const` destructuring from a literal `require()` of those packages in `.js`
  and `.jsx`. Default CommonJS bindings and unregistered `.cjs`/`.mjs` extensions are deferred.
- C/C++: explicit test and keep paths. Test macros and registrations do not label whole files.
  Explicit paths remain available for every language.

Binding checks use syntax, never the heuristic call resolver. Python/JS checks cover the whole file:
an unrelated local shadow can cause a test to remain visible. Local Python
modules named `unittest` or `pytest` anywhere in the analysed inventory prevent that framework's
inference. .NET/Java candidates are checked against declarations across all analysed files before
being saved, including same-namespace lookalikes. .NET conflict comparisons conservatively ignore
case. Recovered declarations can disprove a binding in another file but cannot prove test evidence.
Any malformed file disables its syntax-derived rules and reports incomplete detection.

Inherited member types require compiler binding, so Java methods inside types with explicit bases
remain visible. This also applies to .NET declarations in inherited or C# static-import scopes,
unless the marker uses `global::` / VB `Global.`. Global static imports conservatively veto relative
.NET markers repository-wide. Explicit attribute targets are not inferred as test declarations.
VB namespace guards respect `Global` and types promoted from a `Module`.

These checks do not implement a compiler or runtime loader. Project/global/generated imports,
project root namespaces, inherited/custom attributes, re-exports, path aliases, dynamic rebinding
and framework configuration remain outside automatic detection. Use explicit paths for these cases.
SurelyCRM's pinned project generates its Xunit import and therefore still requires a test path.

Detection does not follow imports/calls, run frameworks, read build configuration, or claim test
coverage. Rust integration-test directories need explicit paths. Unknown cases remain visible.
Separate application/test diagrams and suffix selectors are deferred.

```python
markdown = export_mermaid(
    result, tests="exclude", test_paths=["tests"], keep_paths=["tests/shared"],
    explain_tests=True,
)
```

Analysis keeps every declaration and call. It saves detector version 2 and per-occurrence evidence
in `class_diagram`. Detection notices store separate file/message fields and respect `--path`.
Uncertain-binding notices appear only when that uncertainty actually prevents classification.
Export uses only that saved map and the supplied options. Filtering happens
before impl merging and C++ prototype deduplication; each raw base relationship retains its
source-part origin through its enclosing class/impl. A test-only impl cannot remove the application
owner. If the owner's own declaration is excluded but an impl is kept, export retains an owner
shell with only the kept impl's members and bases, and explains it. An ordinary empty type remains;
a synthetic module with no remaining members disappears.
Type resolution and call-endpoint ambiguity still use the full identity set. Hidden endpoints
cannot leave dangling arrows. Box IDs remain stable within the same `--path` across include/exclude.

Default summaries count source occurrences by file/rule, removed calls, and removed distinct type
relationships. Class/impl records and member records each count as occurrences; these are not test
case counts. `--explain-tests` includes declaration names and locations, with keep overrides.
Counts follow the selected owner boxes, including members supplied by other source files.
Unresolved calls remain separate from intentionally removed calls. Include preserves the previous
Markdown bytes and adds no filtering summary. Colliding legacy declaration IDs are disambiguated
internally so separate boxes do not collapse into one Mermaid node. CLI notices go to stderr; the
Python API emits
`UserWarning`; Python's warnings-as-errors policy is honoured, with a normal CLI error if enabled.
Existing extraction warnings remain available even for excluded source. Rust callers needing notices
can use `export_mermaid_report()`.

Version 1 maps retain their saved Rust/Go rules and advise reanalysis to enable framework rules.
Legacy maps with declarations but no supported detector version still accept explicit paths and
report that reanalysis enables automatic detection. Unknown detector versions use the same
fallback; export does not infer new evidence from filenames. Missing/malformed occurrence locations
remain visible with a notice. Maps without declarations still require reanalysis.

## Data flow

The existing parsing phase reuses its Tree-sitter tree to extract a separate declaration model.
Language-specific visitors live in `crates/mycelium-core/src/languages/declarations/`. They record
classes, interfaces, traits, structs, enums, module functions, members, types, and declared bases.
The shared model lives in `declarations.rs`; `KnowledgeGraph` carries it to `output::build_result`.
JSON maps contain the additive `class_diagram` section. Older maps still deserialize, but exporting
one without declarations reports “rerun analysis”; missing signatures are never invented.
Python files also save `class_diagram.python_bindings`: imports, their local aliases, module-level
class locations, and uncertain bindings. Export reads these facts from JSON without reopening or
executing source. Older maps without these facts retain the previous name heuristics and report
that reanalysis is needed for import-aware resolution.

`mermaid::export_mermaid()` owns resolution, scoping, ordering, partitioning, and escaping. Both
CLIs and the Python binding call that Rust function. CLI wrappers contain no diagram logic.
Existing symbols and calls keep their schema; declarations do not change call-resolution inputs.
The TypeScript caller lookup was fixed where a local variable initializer hid its enclosing
function. C/C++ symbol extraction also recognizes qualified out-of-line function definitions,
so their call edges can reach the owning class. Qualified C++ caller names and owner lookup
prevent a same-named method in another class from replacing an explicit same-owner match.

## What the arrows establish

Declared bases produce inheritance or explicit interface/trait implementation arrows when their
endpoints resolve. Field types produce associations; argument and return types produce dependencies.
No composition or implicit Go/Python/TypeScript interface satisfaction is inferred.

Python imports take precedence over name heuristics for fields, signatures and bases. Direct
`from module import Type`, aliases, relative imports and qualified module imports select the
original declaration, even when another directory contains a type with the same name. Targets
must be unambiguous module-level classes in analysed files; `module.py` and `module/__init__.py`
are supported. Filtering a target never redirects the relationship to another class.

Plain re-exports are followed through saved bindings, with cycle detection and a limit of 64 binding
lookups per reference. A from-import checks package attributes before submodules; an imported class
cannot be mistaken for a same-named module. Imports directly inside a top-level `if TYPE_CHECKING:`
block also provide annotation bindings when the guard is an unshadowed import from `typing` and
has no `else` or `elif`. Direct aliases and `typing.TYPE_CHECKING` spellings are supported.

Module lookup first uses the repository-relative dotted path. If it has no exact match, a unique
dotted suffix supports `src/` layouts and projects beneath a parent directory. Its omitted prefix
must contain the importing file and have no `__init__.py`, so unrelated compatibility modules cannot
stand in for external imports. Multiple eligible locations remain unresolved. This is static
import-root inference; Python search paths and runtime import hooks are not executed. Class
locations and module suffixes are indexed once per export.

Conflicting imports, rebinding, other conditional/local imports, wildcard imports and malformed
syntax are conservative: uncertain names produce no type arrow. Rebinding checks are file-wide,
so an unrelated local shadow can also suppress a relationship. Imported nested types are not
followed; unambiguous same-file nested classes keep their existing name-based resolution. Importing
a package does not establish bindings to unimported submodules. External imports do not fall back
to local lookalikes. These rules apply to declaration relationships; call edges retain their own
heuristics.

When a blocked Python binding has a plausible class in the analysed repository, export reports
`Unresolved Python binding: Service uses Type` (or `Service base Type` for a base class).
`Ambiguous type` is reserved for name-based resolution without usable binding evidence. External
member types with no repository candidate remain quiet; unresolved external bases keep their
existing warning. Successfully resolved targets hidden by test filtering do not produce warnings.

Without binding evidence, type names resolve to a unique same-file declaration, then a unique
declaration in the same directory, then across the same language family. C and C++ share a family,
as do TypeScript and JavaScript. Ambiguous names remain unresolved; qualified names are not split
into unqualified tokens. Python quoted annotations resolve as forward references; `Literal[...]`
strings and `Annotated` metadata remain values. Rust lifetime names do not hide the adjacent type.
This is name-based static resolution, not a compiler's import and type checker. External bases and
out-of-scope bases are reported in warnings. The complete relationship list means all relationships
resolved by this exporter, not every relationship that might exist at runtime.

Calls use the existing heuristic call edges. Member endpoints match by source file, line, and name;
arrows connect their owning boxes. Calls targeting class, struct, or record declarations connect
directly to those boxes, labelled `build() constructs Widget`. This also covers implicit and
inherited constructors. Calls resolved to explicit constructor members retain their member labels.
The original JSON retains confidence, tier, reason, and call-site line.

Calls involving an excluded source occurrence in the selected scope count as removed by test
filtering even if the other endpoint cannot be displayed. Explicit test paths also apply when a
call's declaration is missing from the saved declaration model. Other unmatched or out-of-scope
endpoints count as omitted. Include mode and keep-path overrides retain their usual precedence.

Analysis rejects type-only call targets such as interfaces and traits, and rejects matches between
unrelated languages. C/C++, C#/VB.NET, and TypeScript/JavaScript remain compatible families. A Rust
impl record cannot hide a unique callable declaration of the same name. These checks apply to
import, same-file, global, and interface-implementation resolution; name matching remains heuristic.
Reanalyse existing maps to correct their call targets or capture C++/VB.NET `new` expressions.
Already-recorded class-target calls can be exported from saved maps without reanalysis.

## Coverage and limits

Extraction has regression examples for all ten existing languages: Rust, Python, C#, VB.NET,
TypeScript, JavaScript, Java, Go, C, and C++. Language coverage does not imply compiler
completeness. Tests include abstract TypeScript classes and constructor properties, C typedefs,
enum members, C#/Java positional records, Go embedding/generic receivers, and Rust tuple payloads.
The display focuses on declared field types and parameter/return types. It omits receiver syntax,
default values, generic constraints, and most language-specific method modifiers. Python visibility
uses underscore conventions. Missing annotations display `unknown`; these are not guessed as `void`.

Generated members, macro expansion, dynamic attribute creation, implicit interface satisfaction,
full namespace/import resolution, and arbitrary metaprogramming are outside this exporter's scope.
Namespace-qualified C++ implementations may remain unresolved; their members are kept in a
labelled `unresolved_impl` box with a warning. The existing registry parses `.h` with the C grammar;
use C++ extensions such as `.hpp` for C++ declarations. Access-section visibility in C++, operator
qualifiers, and advanced declaration forms remain limited. Missing external types are not invented.
The vendored VB.NET grammar misparses normal multiline base clauses; the declaration visitor
recovers explicit leading `Inherits`/`Implements` lines while retaining the parse warning.

The editable result is Markdown. Rendering needs a Mermaid-capable viewer. The checkpoint previews
were validated with Mermaid CLI 11.16.0. Very long member names can still wrap or overlap in
Mermaid's layout; the complete Markdown signatures remain available. A single full-repository
view may require substantial zoom. See the official
[class diagram syntax](https://mermaid.js.org/syntax/classDiagram.html).

## Regression checkpoints

`tests/checkpoints/mermaid.json` pins full commits, selected source paths, independently checked
facts, and the SHA-256 of the reviewed Markdown for Forgetful, Ferox, eval-harness, SurelyCRM,
Pi Forgetful, and Pi web-search. These are representative slices, not full-repository snapshots.
Source files are read using `git show COMMIT:PATH` into temporary directories. Working-tree edits,
branch movement, untracked files, and local environment files cannot affect their inputs.

```bash
cargo build -p mycelium-cli
python3 tests/checkpoints/check_mermaid.py --artifacts /tmp/mycelium-checkpoints
# Different checkout locations are supported:
python3 tests/checkpoints/check_mermaid.py --artifacts /tmp/checkpoints \
  --only forgetful --repo forgetful=/path/to/forgetful
```

The runner checks two complete analysis/export passes byte-for-byte, source-fact assertions inside
the expected owning boxes, then the reviewed output hash. The original slices use `--tests include`
with separate hashes for the complete view. Additional slices at the same commits verify removed
tests, retained application declarations, and keep exceptions, then check repeatability and their
own hashes. Additional automatic-only views verify Python fixtures, `TestCase` containers and inline
Node helpers, and retain SurelyCRM's unresolved Xunit methods. The eval-harness chess fixture
deliberately keeps real tests used as evaluation input. Portable integration fixtures cover every
framework rule, misleading names, aliases, malformed syntax, saved-map compatibility and C/C++
paths. Missing repositories or commits fail explicitly. It neither downloads repositories nor
executes their source. Real repositories remain outside this checkout; no private repository sources
are vendored. Regular CI runs the portable language and CLI fixtures; the pinned suite requires
local copies containing the listed commits, including access to private repositories.

When changing a checkpoint, inspect the exact pinned source, review the generated Markdown diff,
render every changed block, then update its hash deliberately. The runner has no automatic accept
mode. A hash alone can preserve a bug; the independent source assertions and small language tests
must remain alongside it.

## Framework rule references

Checked against official documentation on 2026-10-01; Context7 was unavailable.

- [Python unittest](https://docs.python.org/3/library/unittest.html) and
  [pytest fixtures](https://docs.pytest.org/en/stable/how-to/fixtures.html).
- [MSTest](https://learn.microsoft.com/en-us/dotnet/core/testing/unit-testing-mstest-writing-tests),
  [NUnit](https://docs.nunit.org/articles/nunit/writing-tests/attributes/testfixture.html), and
  [xUnit](https://xunit.net/docs/getting-started/v3/getting-started).
- [JUnit annotations](https://docs.junit.org/6.1.3/writing-tests/annotations.html).
- [Node test runner](https://nodejs.org/api/test.html), [Vitest](https://vitest.dev/api/test),
  and [Jest](https://jestjs.io/docs/api).

The vendored VB grammar accepts import aliases, blank lines around Option/Imports, and nested
class/structure/interface/enum declarations. Regenerate its checked-in parser with
`tree-sitter generate --abi 14` from `vendor/tree-sitter-vb-dotnet/`.
The current generated files use Tree-sitter CLI 0.26.10; generated C/JSON/header formatting is
retained.
