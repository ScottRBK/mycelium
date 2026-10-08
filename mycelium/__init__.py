"""Mycelium - Static analysis tool for mapping codebase connections."""

from mycelium._mycelium_rust import PyAnalysisConfig, analyze, export_mermaid, version

__version__ = version()
__all__ = ["PyAnalysisConfig", "analyze", "export_mermaid", "version"]
