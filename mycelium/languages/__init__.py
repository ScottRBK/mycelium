"""Language registry - maps file extensions to language analysers."""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mycelium.languages.base import LanguageAnalyser

_REGISTRY: dict[str, LanguageAnalyser] = {}
_INITIALISED = False


def _init_registry() -> None:
    global _INITIALISED
    if _INITIALISED:
        return

    from mycelium.languages.csharp import CSharpAnalyser
    from mycelium.languages.vbnet import VBNetAnalyser
    from mycelium.languages.typescript import TypeScriptAnalyser
    from mycelium.languages.python_lang import PythonAnalyser
    from mycelium.languages.java import JavaAnalyser
    from mycelium.languages.go import GoAnalyser
    from mycelium.languages.rust import RustAnalyser
    from mycelium.languages.c_cpp import CAnalyser, CppAnalyser

    analysers: list[LanguageAnalyser] = [
        CSharpAnalyser(),
        VBNetAnalyser(),
        TypeScriptAnalyser(),
        PythonAnalyser(),
        JavaAnalyser(),
        GoAnalyser(),
        RustAnalyser(),
        CAnalyser(),
        CppAnalyser(),
    ]

    for analyser in analysers:
        for ext in analyser.extensions:
            _REGISTRY[ext] = analyser

    _INITIALISED = True


def get_analyser(extension: str) -> LanguageAnalyser | None:
    """Get the language analyser for a file extension (e.g. '.cs')."""
    _init_registry()
    return _REGISTRY.get(extension)


def get_language(extension: str) -> str | None:
    """Get the language name for a file extension."""
    analyser = get_analyser(extension)
    return analyser.language_name if analyser else None


def supported_extensions() -> set[str]:
    """Return all supported file extensions."""
    _init_registry()
    return set(_REGISTRY.keys())
