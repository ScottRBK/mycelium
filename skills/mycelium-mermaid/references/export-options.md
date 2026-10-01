# Export options

Apply these to `mycelium-map export INPUT -o OUTPUT`. Use repository-relative paths for selectors;
input and output file paths are ordinary filesystem paths.

## Scope and diagram size

`--path src/services` selects a file or directory within the analysed repository. Analyse the full
repository first. An unmatched scope is an error, not an empty successful diagram.

`--max-classes N` controls boxes per diagram; it must be positive and defaults to eight. A Markdown
file can contain several diagram blocks. Each box also has a fixed limit of forty members, so large
types continue in later blocks. There is no CLI override for the member limit.

When the user requests one diagram, raise `--max-classes` to cover all boxes in the selected view,
rerun export, and count the blocks. A value such as `1000` is a practical starting point for a large
view, not a guarantee. If member continuation still creates multiple blocks, explain the limit;
preserve the requested scope unless the user chooses a smaller one.

```bash
mycelium-map export map.json -o services.md --path src/services --max-classes 1000
```

Every resolved connection between boxes in a block is drawn; there is no connection cap.
The complete relationship list includes connections across blocks. A box without arrows may have no
connections in that view; retain it rather than inventing relationships or discarding declarations.

## Test selection

`--tests exclude` is the default and hides declarations with recognised test evidence. Unknown cases
remain visible. Inspect actual files before selecting additional test paths; class names alone do
not establish that they are tests.

```bash
mycelium-map export map.json -o application.md --test-path tests --explain-tests
mycelium-map export map.json -o application.md --test-path tests --keep-path tests/shared
mycelium-map export map.json -o everything.md --tests include
```

- Repeat `--test-path` and `--keep-path` for multiple files or directories. These are exact path
  selectors, not globs. They match whole components: `tests` does not match `testsupport`.
- Keep paths override test paths and automatic evidence. They cannot restore files absent from the
  map or outside `--path`. Absolute paths and `..` are rejected; unmatched selectors are errors.
- `--explain-tests` adds source-level selection reasons. Review these when tests remain unexpectedly
  visible or application declarations seem missing.
- `--tests include` restores all declarations within the selected scope. Test and keep selectors
  still undergo validation, but have no filtering effect in this mode.

Automatic rules cover Rust test attributes/literal `cfg(test)`, Go `_test.go`, and bounded direct
framework markers in Python, C#/VB.NET, Java, and JavaScript/TypeScript. Ordinary pytest test names,
generated/global imports, custom framework wrappers, and C/C++ test macros need explicit test paths.
Older maps retain only their saved evidence; reanalyse for newer detector rules.

For language-specific detection problems, consult the [test filtering rules][test-rules].
Use the documentation from the same Mycelium revision when working with a pinned build.

[test-rules]: https://github.com/ScottRBK/mycelium/blob/master/docs/mermaid-export.md#test-filtering
