"""Smoke tests for PyO3 bindings."""

from pathlib import Path

import pytest
from mycelium._mycelium_rust import PyAnalysisConfig, analyze, version

FIXTURES = Path(__file__).parent / "fixtures"


def test_csharp_compact_default_full_and_invalid_detail(tmp_path):
    # Arrange: JSON-only export through both public Python boundaries.
    import json

    from click.testing import CliRunner

    from mycelium import export_mermaid
    from mycelium.cli import cli

    result = analyze(str(FIXTURES / "compact_csharp"))
    before = json.dumps(result, sort_keys=True)
    saved = tmp_path / "map.json"
    saved.write_text(json.dumps(result))
    output = tmp_path / "diagram.md"

    # Act / Assert: names remain, including each overload and the interface member.
    default = export_mermaid(result)
    assert "        +Current\n" in default
    assert default.count("        +Run()\n") == 3
    assert default == export_mermaid(result, detail="compact")
    full = export_mermaid(result, detail="full")
    assert full == (FIXTURES / "compact_csharp.full.md").read_text()
    for args, expected in [([], default), (["--detail", "compact"], default),
                           (["--detail", "full"], full)]:
        invocation = CliRunner().invoke(
            cli, ["export", str(saved), "-o", str(output), *args]
        )
        assert invocation.exit_code == 0, invocation.output
        assert output.read_text() == expected
    with pytest.raises(ValueError, match="detail must be compact or full"):
        export_mermaid(result, detail="invalid")
    rejected = CliRunner().invoke(
        cli, ["export", str(saved), "-o", str(output), "--detail", "invalid"]
    )
    assert rejected.exit_code == 2
    assert "--detail" in rejected.output
    assert output.read_text() == full
    assert before == json.dumps(result, sort_keys=True)


def test_version_returns_string():
    v = version()
    assert isinstance(v, str)
    assert "." in v


def test_analyze_returns_dict():
    result = analyze(str(FIXTURES / "csharp_simple"))
    assert isinstance(result, dict)
    assert "metadata" in result
    assert "stats" in result
    assert "structure" in result
    assert "symbols" in result
    assert "imports" in result
    assert "calls" in result
    assert "communities" in result
    assert "processes" in result


def test_analyze_with_config():
    config = PyAnalysisConfig(repo_path="ignored", verbose=True)
    result = analyze(str(FIXTURES / "csharp_simple"), config=config)
    assert result["stats"]["files"] > 0


def test_analyze_with_language_filter():
    config = PyAnalysisConfig(languages=["Python"])
    result = analyze(str(FIXTURES / "python_simple"), config=config)
    assert result["stats"]["files"] > 0


def test_progress_callback():
    phases = []

    def on_phase(name, label):
        phases.append(name)

    analyze(str(FIXTURES / "csharp_simple"), progress=on_phase)
    assert "structure" in phases
    assert "parsing" in phases
    assert "imports" in phases
    assert "calls" in phases
    assert "communities" in phases
    assert "processes" in phases


def test_init_re_exports():
    from mycelium import PyAnalysisConfig as C
    from mycelium import analyze as a
    from mycelium import version as v
    assert callable(a)
    assert callable(v)
    assert C is not None


def test_export_mermaid_and_python_cli(tmp_path):
    # Arrange: exercise the public Python API and saved-map CLI, with real Rust analysis.
    import json

    from click.testing import CliRunner

    from mycelium import export_mermaid
    from mycelium.cli import cli

    source = tmp_path / "source"
    source.mkdir()
    (source / "model.py").write_text("class User:\n    name: str\n")
    result = analyze(str(source))
    saved = tmp_path / "map.json"
    saved.write_text(json.dumps(result))
    destination = tmp_path / "diagram.md"

    # Act.
    markdown = export_mermaid(result, detail="full")
    invocation = CliRunner().invoke(
        cli, ["export", str(saved), "-o", str(destination), "--detail", "full"]
    )

    # Assert.
    assert "+name: str" in markdown
    assert invocation.exit_code == 0, invocation.output
    assert destination.read_text(encoding="utf-8") == markdown
    with pytest.raises(ValueError, match="rerun analysis"):
        export_mermaid({})
    with pytest.raises(ValueError, match="max_classes"):
        export_mermaid(result, max_classes=0)


def test_test_filter_options_match_python_cli_and_validate(tmp_path):
    # Arrange: keep Rust test helpers explicitly, while removing another language's test file.
    import json

    from click.testing import CliRunner

    from mycelium import export_mermaid
    from mycelium.cli import cli

    source = tmp_path / "source"
    source.mkdir()
    (source / "app.rs").write_text("struct App {}\n#[test] fn check() {}")
    (source / "check.py").write_text("class Fake: pass")
    result = analyze(str(source))
    saved = tmp_path / "map.json"
    saved.write_text(json.dumps(result))
    destination = tmp_path / "diagram.md"

    # Act.
    markdown = export_mermaid(result, test_paths=["check.py"], keep_paths=["app.rs"],
                              explain_tests=True)
    invocation = CliRunner().invoke(cli, [
        "export", str(saved), "-o", str(destination), "--test-path", "check.py",
        "--keep-path", "app.rs", "--explain-tests",
    ])

    # Assert.
    assert '["Fake"]' not in markdown
    assert "check()" in markdown
    assert "<details>" in markdown
    assert invocation.exit_code == 0, invocation.output
    assert destination.read_text(encoding="utf-8") == markdown
    assert "check()" not in export_mermaid(result)
    assert "check()" in export_mermaid(result, tests="include")
    with pytest.raises(ValueError, match="tests must"):
        export_mermaid(result, tests="separate")
    with pytest.raises(ValueError, match="matches no analysed file"):
        export_mermaid(result, keep_paths=["missing"])


def test_python_cli_reports_promoted_notices_as_normal_errors(tmp_path):
    # Arrange: Python's user-selected warnings-as-errors policy should not cause a traceback.
    import json
    import warnings

    from click.testing import CliRunner

    from mycelium.cli import cli

    (tmp_path / "app.rs").write_text("struct App {}")
    (tmp_path / "check.rs").write_text("struct Check {}")
    saved = tmp_path / "map.json"
    saved.write_text(json.dumps(analyze(str(tmp_path))))
    destination = tmp_path / "diagram.md"
    # Act.
    with warnings.catch_warnings():
        warnings.simplefilter("error")
        run = CliRunner().invoke(cli, [
            "export", str(saved), "-o", str(destination), "--path", "app.rs",
            "--test-path", "check.rs",
        ])
    # Assert.
    assert run.exit_code == 1
    assert "Mermaid export failed" in run.output
    assert "outside the selected scope" in run.output
    assert not destination.exists()
