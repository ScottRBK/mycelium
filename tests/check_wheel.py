"""Exercise a built wheel outside the checkout, with no Rust tools on PATH.

Run with each supported Python: python tests/check_wheel.py /path/to/wheel-directory
Requires uv on PATH. CI and release jobs pass the same wheel to every Python version.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


def main():
    # Arrange: use only the supplied wheel, fresh environments, and copied test inputs.
    wheels = list(Path(sys.argv[1]).resolve().glob("*.whl"))
    assert len(wheels) == 1, f"Expected exactly one wheel, found {wheels}"
    wheel = wheels[0]
    uv = shutil.which("uv")
    assert uv, "Install uv before running the wheel checks"
    tests = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="mycelium-wheel-") as directory:
        work = Path(directory)
        shutil.copy2(tests / "test_bindings.py", work / "test_bindings.py")
        for fixture in ("csharp_simple", "python_simple", "compact_csharp"):
            shutil.copytree(tests / "fixtures" / fixture, work / "fixtures" / fixture)
        shutil.copy2(tests / "fixtures" / "compact_csharp.full.md", work / "fixtures")
        env = os.environ.copy()
        for key in ("PYTHONPATH", "PYTHONHOME", "VIRTUAL_ENV"):
            env.pop(key, None)
        # Keep ordinary OS utilities: uv's POSIX launchers may need dirname and realpath.
        runtime_path = os.pathsep.join(
            entry for entry in os.get_exec_path()
            if not any(shutil.which(tool, path=entry) for tool in ("rustc", "cargo"))
        )
        env.update(
            PATH=runtime_path,
            UV_CACHE_DIR=str(work / "cache"),
            UV_TOOL_DIR=str(work / "tools"),
            UV_PYTHON_DOWNLOADS="never",
        )
        assert shutil.which("rustc", path=env["PATH"]) is None
        assert shutil.which("cargo", path=env["PATH"]) is None

        def run(*args):
            subprocess.run(args, cwd=work, env=env, check=True)

        # Act: install without compiling and run the existing public binding tests.
        venv = work / "venv"
        python = venv / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
        run(uv, "venv", "--no-config", "--python", sys.executable, str(venv))
        run(uv, "pip", "install", "--no-config", "--no-build", "--no-deps", "--python",
            str(python), str(wheel))
        run(uv, "pip", "install", "--no-config", "--no-build", "--python", str(python),
            str(wheel), "pytest")
        run(str(python), "-I", "-c",
            "import mycelium, sys; from pathlib import Path; "
            "assert Path(mycelium.__file__).is_relative_to(sys.prefix)")
        run(str(python), "-I", "-m", "pytest", "test_bindings.py", "-v")

        # uv tool run is uvx. Use the exact artifact, never a published fallback.
        cli = (uv, "tool", "run", "--no-config", "--isolated", "--no-build", "--python",
               sys.executable, "--from", str(wheel), "mycelium-map")
        run(*cli, "analyze", "fixtures/csharp_simple", "-o", "map.json", "--quiet")
        run(*cli, "export", "map.json", "-o", "diagram.md")
        run(*cli, "export", "map.json", "-o", "diagram-again.md")

        # Assert: meaningful analysis, Mermaid content, and deterministic saved-map export.
        result = json.loads((work / "map.json").read_text(encoding="utf-8"))
        assert result["stats"]["files"] > 0
        markdown = (work / "diagram.md").read_bytes()
        assert b"classDiagram" in markdown
        assert b"AbsenceController" in markdown
        assert markdown == (work / "diagram-again.md").read_bytes()
        assert "-cp312-abi3-" in wheel.name, f"Expected a stable ABI wheel: {wheel.name}"
        print(f"Passed wheel checks on Python {sys.version.split()[0]}: {wheel.name}")


if __name__ == "__main__":
    main()
