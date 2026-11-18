# Nix

Mycelium builds and tests with Nix. The flake is the source of truth; `default.nix` and
`shell.nix` are compatibility entry points generated from it.

## Prerequisites

Nix with the `flakes` and `nix-command` experimental features.

```sh
nix --version   # verified against Nix 2.34.9
```

No host Rust toolchain is required — the flake and dev shell supply `cargo`, `rustc`,
`clippy` and `rustfmt`.

## Development

```sh
nix develop          # cargo, rustc, clippy, rustfmt, rust-analyzer, cc, pkg-config,
                     # git, jq, python3, maturin, plus the pre-commit hooks
nix fmt              # format the tree via the declared formatter
cargo test --workspace
```

Cargo already puts build outputs in the workspace-root `target/`, even when invoked from a
workspace member directory. The dev shell leaves that default intact, so builds share the same
target directory.

## Build and run

```sh
nix build            # result/bin/mycelium-map
nix run              # same binary, executed
./result/bin/mycelium-map analyze <path> -o map.json
```

`packages.mycelium-map`, `packages.default` and `apps.mycelium-map` all refer to the CLI.

The build compiles the workspace but excludes `mycelium-python`: that crate is the PyO3
cdylib for the PyPI wheel and is not part of the CLI's runtime closure.

## Versioning

An env override exists for release builds, with a default that always resolves, so pure
builds never crash on missing metadata.

```sh
# default — read from crates/mycelium-cli/Cargo.toml
nix build

# release — run on a clean checkout of the tag
MYCELIUM_VERSION=$(git describe --tags --abbrev=0) nix build --impure
```

`--abbrev=0` matters: raw `git describe --tags` output (`v0.4.5-3-g96a9361`) is not a clean
version; the bare tag is.

There is deliberately **no version literal in `flake.nix`**. If the `version = "..."` line
moves, evaluation fails naming the file rather than silently producing a stale version
(verified: renaming the line yields `Could not find a 'version = "..."' line in
crates/mycelium-cli/Cargo.toml`).

Cargo reads the version from `Cargo.toml` and bakes it into the binary at compile time via
`env!("CARGO_PKG_VERSION")`. `MYCELIUM_VERSION` changes the Nix derivation version only; changing
the CLI's reported version requires changing `Cargo.toml`.

## Checks

`nix flake check` runs eight checks:

| check | what it proves |
|---|---|
| `pre-commit` | nixfmt / deadnix / statix / rustfmt hooks pass on the tree |
| `cargo-fmt` | `cargo fmt --check` is clean |
| `clippy` | `cargo clippy --workspace -- -D warnings`, mirroring CI |
| `cargo-test` | `cargo test --workspace --exclude mycelium-python` in the Nix sandbox |
| `python-bindings` | `pytest tests/test_bindings.py` against the Nix-built Python module |
| `checkpoint-exports` | the repo's own determinism harness (`tests.test_checkpoint_exports`) run against the Nix-built CLI via `MYCELIUM_BIN` |
| `python-install-boundary` | the module resolves under `sys.prefix`, the binding suite passes against the *installed* copy, and the console script analyses + exports deterministically — with no `rustc` or `cargo` reachable |
| `smoke` | analysis of `tests/fixtures/csharp_simple` yields non-zero files/symbols/calls/communities, `commit_hash` is null under Nix, export emits a `classDiagram` containing `AbsenceController`, and a repeat export is byte-identical |

The CLI package build is compile-only (`doCheck = false`). The separate `cargo-test` check owns
the Rust test suite, so `nix build` does not rerun it and `nix flake check` runs it once.

Two deliberate choices:

- **clippy mirrors CI rather than exceeding it.** `--all-targets` was tried and rejected: it
  additionally lints every integration-test target and fails on dead code in the shared
  `tests/common` helpers, i.e. a stricter gate than upstream CI opted into. Promote it only
  if CI adopts it.
- **`cargo-test` excludes `mycelium-python`.** Its coverage is the Python binding tests, which
  is exactly what the `python-bindings` check runs.

The smoke check is a domain assertion, not an exit-code check: it asserts the analysis
actually produced the expected facts, and that export is deterministic.

## Python module

The wheel is still built by maturin for PyPI — Nix cannot produce manylinux-conformant
wheels, and the five-target release matrix stays CI's job. What Nix *does* provide is an
importable module, so `from mycelium import analyze` works in a Nix environment and the
binding suite becomes a check.

This is cheap here because pyo3 is built with `extension-module` + `abi3-py312`: the cdylib
needs no interpreter at build time and does not link libpython. Verified on the built
extension — `DT_NEEDED` is `libgcc_s.so.1`, `libm.so.6`, `libc.so.6`, `ld-linux-x86-64.so.2`,
and no libpython appears anywhere in the closure. So the module is a file-placement problem:
crane builds the cdylib, and it is renamed `lib_mycelium_rust.so` → `_mycelium_rust.so` and
placed beside `__init__.py` and `cli.py` in a site-packages tree. No maturin in the build.

To use it in a shell, point Python at it:

```sh
mod=$(nix build --no-link --print-out-paths .#pythonModule)
sp="$mod/lib/$(ls "$mod/lib")/site-packages"
# Run from a directory that has no `mycelium/` package in it - pytest/`python -c` put the
# invocation directory on sys.path and would shadow the installed module.
cd /tmp
nix-shell -p python3 python3Packages.click python3Packages.rich \
  --run "cd /tmp && PYTHONPATH=$sp python -c 'import mycelium; print(mycelium.version())'"
```

**Closure hygiene.** Two build artefacts were measured leaking into this output's runtime
closure, and both are disabled deliberately:

- `doInstallCargoArtifacts = false` — crane's default hook archives the whole `target/`
  directory into `$out/target.tar.zst` for reuse as `cargoArtifacts` downstream. This output
  is a Python module, not an intermediate for another build, so that archive is dead weight.
- `RUSTFLAGS=--remap-path-prefix=...` — cargo embeds the vendored crate paths (`file!()` in
  panic metadata; 141 occurrences measured) into the binary, and stdenv's reference scan then
  pins the vendored crate directory into the runtime closure. Remapping to a stable prefix
  drops that reference and makes the build more reproducible as a side effect.

Measured module closure: **400.9 MiB before both fixes, 68.3 MiB after** — in line with the
CLI's 69.1 MiB.

The binding suite runs from a copy of `tests/` that omits the source-level `mycelium/`
package: pytest puts the invocation directory on `sys.path`, so running in the tree would
shadow the installed module and its compiled extension.

## Non-flake usage

`default.nix` and `shell.nix` use [flake-compat] so the project also works under stable Nix
without `experimental-features`, and under tooling with no native flake support. Both derive
flake-compat's location from `flake.lock`, so `nix flake update` keeps them in agreement, and
`shell.nix` delegates to the flake's dev shell rather than re-declaring it.

```sh
nix-build .                    # builds the default package (the CLI)
nix-shell --run 'cargo --version'
```

Note `nix-build .` takes no `-A`: flake-compat exposes the flake's *default* package, so
`nix-build -A mycelium-map .` does not resolve.

## Tier-3: what Nix covers and what stays with CI

`tests/check_wheel.py` validates an installed wheel, including its generated console entry
point and the wheel's layout. `python-install-boundary` covers a narrower boundary: it copies
the Nix-built module into a venv, checks that imports resolve under `sys.prefix`, runs the
binding tests and a hand-written CLI shim, compares repeated exports, and checks that `rustc`
and `cargo` are absent from `PATH`. It does not install or validate a wheel; that remains the
job of maturin and `test-wheels.yml`.

Deliberately **not** covered, left to maturin and `test-wheels.yml`:

- that the published `.whl` is well-formed / manylinux-conformant — a Nix-built binary
  references `/nix/store` paths and cannot pass `auditwheel`;
- that `uv tool run` resolves the distribution from an index;
- the macOS and Windows wheel targets, which have no builder here.

## Consumed as a flake input

Verified that this flake still builds when a consumer pins a **different** nixpkgs channel,
which matters because this project pins `nixpkgs-unstable` while downstream consumers may be on
a release branch.

A scratch consumer flake pinning `nixos-26.05` with `mycelium.inputs.nixpkgs.follows = "nixpkgs"`
builds and runs the CLI, and the resulting binary comes from the store (referencing that
channel's glibc and libstdc++), not from the source tree:

```nix
inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
inputs.mycelium = {
  url = "git+file:///path/to/mycelium";   # or github:ScottRBK/mycelium
  inputs.nixpkgs.follows = "nixpkgs";
};
```

`follows` is what makes this work: without it the flake would pull its own nixpkgs and end up
with two copies of nixpkgs in the same evaluation.

## Packaging decisions

| area | choice | why |
|---|---|---|
| channel | `nixpkgs-unstable` | chosen at creation. Cost: the input is younger than the stable release branch a consumer is likely to pin, so `26.11`-era breakage arrives early |
| upstream | contribution to `ScottRBK/mycelium` | hooks and checks stay upstream-compatible; nothing here imposes enforcement upstream did not choose |
| follows | `flake-parts → nixpkgs-lib`, `git-hooks-nix → { nixpkgs, flake-compat }` | keeps one lock node per shared input. `crane` declares **no** inputs of its own and is instantiated with `crane.mkLib pkgs`, so a `follows` on it is invalid |
| lock nodes | 6 (`root`, `nixpkgs`, `flake-parts`, `crane`, `git-hooks-nix`, `flake-compat`) | was 7 until 2026-11-08, when the unconsumed `systems` input was removed; `nix flake lock` reported `Removed input 'systems'` and the built CLI/store path was unchanged |
| systems source | literal list in `flake-parts`, not `inputs.systems` | see the architectures row; the `github:nix-systems/default` input was removed 2026-11-08 |
| source filter | **deny-list** via `lib.cleanSourceWith` | the build may start reading files added in a future upstream change; an allow-list could omit them until updated. This filter retains new paths by default and removes only known build-irrelevant files and directories |
| filter specifics | additionally drops root `*.md`, `docs/`, `.github/` | verified nothing reads them from Rust; the only embedded `.md` inputs are `tests/fixtures/*.md` (`include_str!`), which are kept. Measured: `drvPath` is unchanged across a docs-only edit. If upstream later reads a dropped path, the build fails loudly on a missing file |
| must survive | `vendor/tree-sitter-vb-dotnet`, `tests/fixtures/` | the former is a Cargo path dependency; the latter is read at test runtime via `CARGO_MANIFEST_DIR` |
| dependency integrity | `Cargo.lock` checksums; **no `cargoHash`** | this crane release derives per-crate hashes from `Cargo.lock` and silently ignores a `cargoHash` argument (verified: two different bogus hashes both built). 140/140 crates.io deps carry checksums and there are no git-sourced deps, so integrity is genuinely enforced |
| architectures | `x86_64-linux`, `aarch64-linux`, written as a literal list; `x86_64-darwin` omitted as deprecated upstream; `aarch64-darwin` **excluded** | The literal list is deliberate, not an abandoned `inputs.systems`: `github:nix-systems/default` exports `outputs = _: { }`, so it has no `.default` to read, and nothing referenced it. **`aarch64-linux` is UNVERIFIED.** No builder was available, and `nix flake check` says so directly: *"The check omitted these incompatible systems: aarch64-linux."* It is documentation until someone with that hardware runs `nix build` and `nix flake check`. Elevated risk here because the build compiles ten tree-sitter grammars of C per target. **`aarch64-darwin` was previously listed and has been removed**: the install phase copies `target/release/lib_mycelium_rust.so` (cargo emits `.dylib` on macOS) and runs GNU `strip`, which cannot read Mach-O, so `pythonModule` and its two checks failed and `nix flake check` failed on an advertised platform. That listing was wrong, not merely unverified - it is restored only with a copy emitting `.so` on both platforms, a `strip` that handles Mach-O, and a darwin builder to prove it |
| hooks | `nixfmt`, `deadnix`, `statix`, `rustfmt` at default stage; `clippy` at **`manual` only** | clippy runs a full cargo build per invocation — too heavy per commit, and CI already gates `-D warnings`. `manual` and never `pre-push`, which still runs on `git push` |
| binary cache | public Cachix cache at `mycelium.cachix.org`, configured by the flake | The cache exists and is public. Measured compressed CLI and Python module closures are 69.1 MiB and 68.3 MiB; CI publishing is a separate change from the flake |

[flake-compat]: https://github.com/NixOS/flake-compat
