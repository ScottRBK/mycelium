{
  description = "Static analysis tool that maps the hidden network of connections in codebases";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    # crane declares no inputs of its own; it is instantiated against the
    # consumer's nixpkgs via crane.mkLib below. A `follows` here is invalid.
    crane.url = "github:ipetkov/crane";

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-compat.follows = "flake-compat";
      };
    };

    # default.nix / shell.nix entry points for nix-build and nix-shell
    flake-compat = {
      url = "github:NixOS/flake-compat";
      flake = false;
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.git-hooks-nix.flakeModule ];

      # Deliberately a literal list, not `inputs.systems.default` from
      # github:nix-systems/default. That input was declared here, consumed by nothing, and
      # has since been removed: its flake exports `outputs = _: { }`, so there is no
      # `.default` attribute to read at all - the pattern needs `flake = false` plus
      # `import`, which is more indirection than a three-element list is worth. Do not
      # re-add the input without a consumer for it.
      #
      # House practice (Forgetful #85 precedent): x86_64-darwin omitted as deprecated
      # upstream. NOTE: aarch64-linux and aarch64-darwin are UNVERIFIED - no builder or
      # aarch64-darwin host was available when this flake was written, so those entries
      # are documentation until someone with that hardware runs nix build/flake check.
      # See docs/nix.md.
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          pkgs,
          ...
        }:
        let
          inherit (pkgs) lib;

          # A14: single source of truth is the Cargo manifest, read at eval time from
          # the RAW path (never a filtered fileset). Deliberately NOT a literal here -
          # a version literal in flake.nix drifts silently behind releases.
          # A14 version identity — house pattern (forgetful / agent-shell): an env
          # override for release builds, with a default that always resolves so pure
          # builds never crash on missing metadata.
          #
          # Rust-specific divergence, stated plainly: the Python projects inject
          # SETUPTOOLS_SCM_PRETEND_VERSION because hatch-vcs cannot see .git inside a
          # Nix build. Cargo has no such problem — the version is a literal in
          # Cargo.toml, and `env!("CARGO_PKG_VERSION")` is baked into the binary at
          # COMPILE time. So MYCELIUM_VERSION controls the Nix derivation name and
          # nothing more; it cannot change what the built binary reports. Anyone
          # needing a different embedded version must change Cargo.toml.
          #
          # Release (needs --impure so getEnv is visible under a pure evaluator):
          #   MYCELIUM_VERSION=$(git describe --tags --abbrev=0) nix build --impure
          # `--abbrev=0` matters: raw `git describe --tags` output
          # (v0.4.5-3-g96a9361) is not a clean version; the bare tag is.
          cargoVersion =
            let
              lines = builtins.filter (l: lib.hasPrefix "version = " l) (
                lib.splitString "\n" (builtins.readFile ./crates/mycelium-cli/Cargo.toml)
              );
              bare = lib.removeSuffix "\"" (
                lib.removePrefix "\"" (lib.removePrefix "version = " (lib.head lines))
              );
            in
            if lines == [ ] then
              throw "Could not find a 'version = \"...\"' line in crates/mycelium-cli/Cargo.toml"
            else
              bare;

          version =
            let
              r = builtins.tryEval (builtins.getEnv "MYCELIUM_VERSION");
              e = if r.success then r.value else "";
            in
            if e != "" then e else cargoVersion;

          # A9: deny-list, NOT an allow-list. This tree is upstream-controlled, so an
          # allow-list would silently drop any new upstream directory the build starts
          # reading. Only build-irrelevant droppings are removed. `vendor/` MUST survive
          # (Cargo path dependency on the vendored VB.NET grammar) and `tests/` MUST
          # survive (integration tests read tests/fixtures at runtime).
          src = lib.cleanSourceWith {
            src = lib.cleanSource ./.;
            filter =
              path: _type:
              let
                base = baseNameOf (toString path);
                # Path relative to the flake root, e.g. "/README.md" or "/docs/nix.md".
                inner = lib.removePrefix "/" (lib.removePrefix (toString ./.) (toString path));
                atRoot = !lib.hasInfix "/" inner;
              in
              !(lib.hasPrefix "result" base)
              && !(lib.hasSuffix ".mycelium.json" base)
              && !(base == ".venv" || base == "venv" || base == "__pycache__")
              # Proven build-irrelevant, so a docs edit does not invalidate the cache.
              # Verified nothing reads these: the only .md inputs the tests embed are
              # tests/fixtures/*.md (include_str!), which this filter keeps. If upstream
              # ever reads root Markdown or docs/ from Rust, the build fails loudly on a
              # missing file rather than silently disagreeing - the intended failure mode.
              && !(atRoot && lib.hasSuffix ".md" inner)
              && !(inner == "docs" || lib.hasPrefix "docs/" inner)
              && !(inner == ".github" || lib.hasPrefix ".github/" inner);
          };

          # The grammars are built from C by the `cc` crate, so a C compiler is required
          # even though the payload is pure Rust.
          commonArgs = {
            # Pinned for every derivation (not just the package): the workspace root
            # Cargo.toml carries no [package] version, so letting crane derive it here
            # yields a warning and a name that disagrees with the shipped binary.
            pname = "mycelium-map";
            inherit src version;
            # Integrity note: this crane release derives per-crate hashes from the
            # `checksum` fields in Cargo.lock (verified: 0 git-sourced deps, every
            # crates.io entry carries a checksum). It does NOT accept a `cargoHash`
            # argument — passing one is silently ignored, so none is set here.
            cargoLock = ./Cargo.lock;
            nativeBuildInputs = [
              pkgs.stdenv.cc
              pkgs.pkg-config
            ];
          };

          craneLib = inputs.crane.mkLib pkgs;

          # Dependencies only: cached separately so an ordinary source edit does not
          # recompile ten tree-sitter grammars.
          depsArtifacts = craneLib.buildDepsOnly (
            commonArgs
            // {
              pname = "mycelium-deps";
              doCheck = false;
            }
          );

          cliPackage = craneLib.buildPackage (
            commonArgs
            // {
              pname = "mycelium-map";
              # Build the workspace but skip the wheel cdylib: mycelium-python is a
              # PyO3 artifact for the wheel, not part of the CLI's runtime closure.
              cargoExtraArgs = "--locked --workspace --exclude mycelium-python";
              cargoArtifacts = depsArtifacts;
              meta = {
                description = "Map the hidden network of connections in a source code repository";
                homepage = "https://github.com/ScottRBK/mycelium";
                license = lib.licenses.mit;
                mainProgram = "mycelium-map";
                # Unverified beyond x86_64-linux - see the systems comment above.
                platforms = [
                  "x86_64-linux"
                  "aarch64-linux"
                  "aarch64-darwin"
                ];
              };
            }
          );

          # ---------------------------------------------------------------------
          # Python module (task: make the PyO3 binding tests reachable as checks)
          #
          # pyo3 is built with `extension-module` + `abi3-py312`, so the cdylib does
          # NOT link libpython and needs no interpreter at build time. That makes the
          # module a file-placement problem: build the .so with crane, drop it beside
          # the pure-Python sources in a site-packages tree. No maturin in the build.
          # ---------------------------------------------------------------------
          inherit (pkgs) python3;

          # click + rich are the package's only declared runtime deps; pytest is test-only.
          pyEnv = python3.withPackages (ps: [
            ps.click
            ps.rich
            ps.pytest
          ]);

          pythonModule = craneLib.mkCargoDerivation {
            pname = "python-mycelium-map";
            inherit version src;
            cargoArtifacts = depsArtifacts;
            nativeBuildInputs = [
              pkgs.stdenv.cc
              pkgs.binutils-unwrapped
            ];

            # crane's default hook archives the whole target/ directory into
            # $out/target.tar.zst so it can be reused as cargoArtifacts downstream. This
            # output is a Python module, not an intermediate for another build, so that
            # archive is pure dead weight - measured at ~360 MiB of build artefacts in a
            # 41 MiB output, and it would be pushed to any binary cache we publish.
            doInstallCargoArtifacts = false;

            # RUSTFLAGS remap: cargo embeds the vendored crate paths (file!() in panic
            # metadata) into the binary - 141 occurrences measured - and stdenv's
            # reference scan then pins the ~244 MiB vendor directory into this output's
            # runtime closure. Remapping to a stable prefix drops that reference and
            # makes the build more reproducible as a side effect.
            buildPhaseCargoCommand = ''
              RUSTFLAGS="--remap-path-prefix=$cargoVendorDir=/vendor --remap-path-prefix=$(pwd)=/build" \
                cargo build --locked --release -p mycelium-python
            '';

            # pyo3's abi3 extension module must import as `mycelium._mycelium_rust`, so
            # the cdylib is renamed from lib_mycelium_rust.so to _mycelium_rust.so and
            # placed beside the two pure-Python sources.
            installPhaseCommand = ''
              target="$out/${python3.sitePackages}/mycelium"
              mkdir -p "$target"
              cp target/release/lib_mycelium_rust.so "$target/_mycelium_rust.so"
              cp mycelium/__init__.py mycelium/cli.py "$target/"

              # Strip debug info so absolute build paths embedded in the binary are not
              # recorded as runtime references by stdenv's scan. --strip-debug keeps
              # .dynsym, so the extension still loads.
              strip --strip-debug "$target/_mycelium_rust.so"
            '';
          };

          # Tier-2 check: the public binding tests, against the Nix-built module.
          pythonBindingsTest = pkgs.stdenv.mkDerivation {
            pname = "mycelium-python-bindings-test";
            inherit version src;
            nativeBuildInputs = [ pyEnv ];
            phases = [
              "unpackPhase"
              "checkPhase"
              "installPhase"
            ];
            doCheck = true;
            checkPhase = ''
              runHook preCheck
              export HOME="$TMPDIR"
              # Run from a copy that deliberately omits the source-level `mycelium/`
              # package: pytest puts the invocation directory on sys.path, so running in
              # the tree would shadow the installed module (and its compiled extension).
              work="$TMPDIR/bindings-work"
              mkdir -p "$work"
              cp -r tests/test_bindings.py "$work/"
              cp -r tests/fixtures "$work/fixtures"
              chmod -R u+w "$work"
              cd "$work"
              PYTHONPATH="${pythonModule}/${python3.sitePackages}" python -m pytest test_bindings.py -q
              runHook postCheck
            '';
            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              echo "PyO3 binding suite passed against the Nix-built module" > "$out/passstamp"
              runHook postInstall
            '';
          };

          # Tier-3 equivalent: install-boundary correctness, proven hermetically.
          #
          # tests/check_wheel.py asserts that a *packaged* install works with no Rust on
          # PATH: the module resolves under sys.prefix, the binding suite passes against the
          # installed copy, the console script runs, and export is deterministic. It gets
          # there with uv + a built wheel, which needs network index resolution - so it
          # cannot run in the sandbox as written. What it ASSERTS does not need uv: this
          # check reproduces the same assertions by installing the module into a venv the
          # same way a wheel install would (files under the prefix, no compilation).
          #
          # Deliberately NOT covered here, and left to maturin + test-wheels.yml:
          #   - that the published .whl is well-formed / manylinux-conformant
          #   - that `uv tool run` resolves the distribution from an index
          # Neither is achievable from Nix: a Nix-built binary references /nix/store paths
          # and cannot pass auditwheel, and only one of the five release targets is buildable.
          pythonInstallBoundary = pkgs.stdenv.mkDerivation {
            pname = "mycelium-python-install-boundary";
            inherit version src;
            nativeBuildInputs = [
              pyEnv
              pkgs.gnutar
            ];
            phases = [
              "unpackPhase"
              "checkPhase"
              "installPhase"
            ];
            doCheck = true;
            checkPhase = ''
              runHook preCheck
              export HOME="$TMPDIR"

              venv="$TMPDIR/installed"
              ${pyEnv}/bin/python -m venv --without-pip --system-site-packages "$venv"
              site="$venv/lib/${python3.libPrefix}/site-packages"
              mkdir -p "$site/mycelium"
              # Copy, do not symlink: a wheel install materialises files under the prefix,
              # and the assertion below depends on that distinction.
              cp ${pythonModule}/${python3.sitePackages}/mycelium/*.py "$site/mycelium/"
              cp ${pythonModule}/${python3.sitePackages}/mycelium/_mycelium_rust.so "$site/mycelium/"

              # Console script equivalent for `mycelium-map = mycelium.cli:cli`.
              export PY_BIN="$venv/bin/python"
              cat > "$venv/bin/mycelium-map" <<'SHIM'
              #!/bin/sh
              exec "$PY_BIN" -m mycelium.cli "$@"
              SHIM
              chmod +x "$venv/bin/mycelium-map"

              # Run from outside the source tree so the source-level `mycelium/` package
              # cannot shadow the installed one.
              work="$TMPDIR/consumer"
              mkdir -p "$work"
              cp -r tests/test_bindings.py "$work/"
              cp -r tests/fixtures "$work/fixtures"
              chmod -R u+w "$work"
              cd "$work"

              # No Rust toolchain may be reachable - that is the point of the check.
              test -z "$(command -v rustc || true)"
              test -z "$(command -v cargo || true)"

              "$venv/bin/python" -I -c 'import mycelium, sys
              from pathlib import Path
              assert Path(mycelium.__file__).is_relative_to(sys.prefix), mycelium.__file__
              # abi3 forward compatibility: built against the 3.12 limited API, imported here.
              import sys
              assert sys.version_info >= (3, 12)
              print("installed under prefix:", mycelium.__file__)'

              "$venv/bin/python" -I -m pytest test_bindings.py -q

              "$venv/bin/mycelium-map" analyze fixtures/csharp_simple -o map.json --quiet
              "$venv/bin/mycelium-map" export map.json -o diagram.md
              "$venv/bin/mycelium-map" export map.json -o diagram-again.md
              cmp diagram.md diagram-again.md
              grep -q classDiagram diagram.md

              runHook postCheck
            '';
            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              echo "installed-prefix + console-script checks passed with no Rust on PATH" > "$out/passstamp"
              runHook postInstall
            '';
            # The check consumes the module derivation built above rather than
            # recompiling the extension.
          };

          # Tier-1b: the repo's own determinism harness (pinned full-vs-compact
          # Mermaid comparisons + SHA-256 of reviewed Markdown), run against the
          # Nix-built CLI. Upstream relies on the default target/debug path; the
          # MYCELIUM_BIN override exists precisely so this can run elsewhere.
          checkpointExports = pkgs.stdenv.mkDerivation {
            pname = "mycelium-checkpoint-exports";
            inherit version src;
            nativeBuildInputs = [ python3 ];
            phases = [
              "unpackPhase"
              "checkPhase"
              "installPhase"
            ];
            doCheck = true;
            checkPhase = ''
              runHook preCheck
              export HOME="$TMPDIR"
              export MYCELIUM_BIN="${cliPackage}/bin/mycelium-map"
              # The suite imports tests.checkpoints.check_mermaid, so the tree root
              # must be importable.
              export PYTHONPATH="$PWD"
              python -m unittest tests.test_checkpoint_exports -v
              runHook postCheck
            '';
            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              echo "checkpoint comparisons passed against the Nix-built CLI" > "$out/passstamp"
              runHook postInstall
            '';
          };

          # Authored domain check, in the spirit of agent-shell's `zero-runtime-deps`:
          # assert the analysis actually produces the expected facts, rather than only
          # that cargo exited 0. Fixture is the C# controller/service/repository case the
          # README documents.
          smoke = pkgs.runCommand "mycelium-map-smoke" { } ''
            export HOME="$TMPDIR"
            work="$(mktemp -d)"
            cp -r ${./tests/fixtures/csharp_simple} "$work/src"
            chmod -R u+w "$work/src"
            ${cliPackage}/bin/mycelium-map analyze "$work/src" -o "$work/map.json" --quiet

            ${pkgs.jq}/bin/jq -e '
              (.stats.files > 0) and
              (.stats.symbols > 0) and
              (.stats.calls > 0) and
              (.stats.communities > 0) and
              (.metadata.commit_hash == null)
            ' "$work/map.json" > /dev/null

            ${cliPackage}/bin/mycelium-map export "$work/map.json" -o "$work/diagram.md"
            grep -q 'classDiagram' "$work/diagram.md"
            grep -q 'AbsenceController' "$work/diagram.md"

            # Determinism: the same map exported twice must be byte-identical.
            ${cliPackage}/bin/mycelium-map export "$work/map.json" -o "$work/diagram-again.md"
            cmp "$work/diagram.md" "$work/diagram-again.md"

            mkdir -p "$out"
            echo "analysis + deterministic export verified" > "$out/passstamp"
          '';
        in
        {
          packages = {
            mycelium-map = cliPackage;
            default = cliPackage;
            # Importable module tree (site-packages layout). Consumers set
            # PYTHONPATH="$(nix build --no-link --print-out-paths .#pythonModule)/lib/
            # pythonX.Y/site-packages". The PyPI wheel remains maturin's job - Nix cannot
            # emit manylinux-conformant wheels.
            inherit pythonModule;
          };

          apps.mycelium-map = {
            type = "app";
            program = lib.getExe cliPackage;
          };

          # A6: hooks that would fail on the existing tree are pinned to the manual
          # stage ONLY - never pre-push, which still runs on git push.
          pre-commit.settings.hooks = {
            rustfmt.enable = true;
            nixfmt.enable = true;
            deadnix.enable = true;
            statix.enable = true;
            # clippy runs a full cargo build per invocation, which is too heavy for a
            # per-commit hook; CI already gates `-D warnings`. Promote if that ever
            # stops being true.
            clippy.enable = true;
            clippy.stages = [ "manual" ];
          };

          formatter = pkgs.writeShellScriptBin "fmt" ''
            ${lib.getExe config.pre-commit.settings.package} run --all-files --config ${config.pre-commit.settings.configFile}
          '';

          devShells.default = pkgs.mkShell {
            # Reuse the build environment so `cargo test` in the shell matches the check.
            shellHook = ''
              ${config.pre-commit.shellHook}
              export CARGO_TARGET_DIR="''${CARGO_TARGET_DIR:-$PWD/target}"
            '';
            packages = [
              pkgs.cargo
              pkgs.rustc
              pkgs.clippy
              pkgs.rustfmt
              pkgs.rust-analyzer
              pkgs.stdenv.cc
              pkgs.pkg-config
              pkgs.git
              pkgs.jq
              pkgs.python3
              pkgs.maturin
            ]
            ++ config.pre-commit.settings.enabledPackages;
          };

          # git-hooks.nix's flakeModule contributes `checks.pre-commit` automatically;
          # do not re-declare it here or the option types collide.
          checks = {
            cargo-fmt = craneLib.cargoFmt commonArgs;
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                cargoArtifacts = depsArtifacts;
                # Deliberately mirrors CI (`.github/workflows/ci.yml`:
                # `cargo clippy --workspace -- -D warnings`) rather than adding
                # `--all-targets`. --all-targets lints every integration test target
                # and fails on dead code in the shared tests/common helpers, i.e. a
                # stricter gate than upstream opted into. Promote it only if CI adopts it.
                cargoClippyExtraArgs = "--workspace -- -D warnings";
              }
            );
            cargo-test = craneLib.cargoTest (
              commonArgs
              // {
                cargoArtifacts = depsArtifacts;
                # mycelium-python is a cdylib for the wheel; its coverage is the
                # Python-side binding tests, not cargo test.
                cargoTestExtraArgs = "--workspace --exclude mycelium-python";
              }
            );
            inherit smoke;
            python-bindings = pythonBindingsTest;
            python-install-boundary = pythonInstallBoundary;
            checkpoint-exports = checkpointExports;
          };
        };
    };
}
