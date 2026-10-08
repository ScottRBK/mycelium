# Export options

Add these to the skill's export command. Path selectors are relative to the analysed repository.

| Purpose | Option |
|---|---|
| Names-only members and relationships grouped by meaning (default) | `--detail compact` |
| Previous detailed output, including signatures and keys | `--detail full` |
| Select a file or directory | `--path src/services` |
| Set boxes per diagram | `--max-classes 1000` |
| Include recognised tests | `--tests include` |
| Exclude an additional test directory | `--test-path tests` |
| Keep shared code inside that directory | `--keep-path tests/shared` |
| Explain test selection | `--explain-tests` |

The CLI defaults to eight boxes per diagram. For one diagram, set `--max-classes` to at least the
included box count. Each box keeps all its members in either detail mode, including separate
overload rows. Compact retains visibility, stereotypes, source paths/lines, and individual warnings.
Calls, constructs, type dependencies, and fields have separate arrow groups. Connections between
blocks are listed once; connections drawn inside a block are not repeated in that list.

Full restores the previous Markdown bytes with the same map and other options, including original
relationship labels and complete type/signature keys. Use it for consumers expecting that format.
Detail affects presentation only; switching modes needs no reanalysis. Python accepts
`export_mermaid(result, detail="full")`; detail and test selection are independent.

Tests default to excluded; unknown cases remain visible. Inspect files before adding test paths.
Repeat test/keep selectors as needed. They match whole path components, not globs; absolute paths,
`..`, and unmatched selectors fail. Keep paths override test evidence but cannot restore files
absent from the map or outside the scope. Include mode validates selectors but ignores filtering.

For language-specific detection questions, read the [test filtering rules][rules]. Reanalyse older
maps to obtain newer detection evidence.

[rules]: https://github.com/ScottRBK/mycelium/blob/master/docs/mermaid-export.md#test-filtering
