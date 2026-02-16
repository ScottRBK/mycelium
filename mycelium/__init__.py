"""Mycelium - Static analysis tool for mapping codebase connections."""

from mycelium._mycelium_rust import analyze, version, PyAnalysisConfig

__version__ = version()
__all__ = ["analyze", "version", "PyAnalysisConfig"]
