"""Smoke tests for PyO3 bindings."""

import os
from pathlib import Path

import pytest

from mycelium._mycelium_rust import analyze, version, PyAnalysisConfig

FIXTURES = Path(__file__).parent / "fixtures"


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
    from mycelium import analyze as a, version as v, PyAnalysisConfig as C
    assert callable(a)
    assert callable(v)
    assert C is not None
