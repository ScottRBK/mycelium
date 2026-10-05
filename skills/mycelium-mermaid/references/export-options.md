# Export options

Add these to the skill's export command. Path selectors are relative to the analysed repository.

| Purpose | Option |
|---|---|
| Select a file or directory | `--path src/services` |
| Set boxes per diagram | `--max-classes 1000` |
| Include recognised tests | `--tests include` |
| Exclude an additional test directory | `--test-path tests` |
| Keep shared code inside that directory | `--keep-path tests/shared` |
| Explain test selection | `--explain-tests` |

The CLI defaults to eight boxes and forty members per box. Increase `--max-classes` if needed for
one diagram; the fixed member limit can still create continuation blocks. Report that limit without
dropping declarations. All resolved connections are retained, including a list across blocks.

Tests default to excluded; unknown cases remain visible. Inspect files before adding test paths.
Repeat test/keep selectors as needed. They match whole path components, not globs; absolute paths,
`..`, and unmatched selectors fail. Keep paths override test evidence but cannot restore files
absent from the map or outside the scope. Include mode validates selectors but ignores filtering.

For language-specific detection questions, read the [test filtering rules][rules]. Use the matching
release tag for a pinned package. Reanalyse older maps to obtain newer detection evidence.

[rules]: https://github.com/ScottRBK/mycelium/blob/v0.4.2/docs/mermaid-export.md#test-filtering
