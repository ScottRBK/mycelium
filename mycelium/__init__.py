"""Mycelium - Static analysis tool for mapping codebase connections."""

from mycelium._mycelium_rust import analyze, export_mermaid, version, PyAnalysisConfig

__version__ = version()
__all__ = ["analyze", "export_mermaid", "version", "PyAnalysisConfig"]
