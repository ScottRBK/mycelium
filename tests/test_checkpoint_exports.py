"""Portable checkpoint checks using the locally built native CLI; no external repositories."""

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from tests.checkpoints.check_mermaid import check_compact

BINARY = Path(__file__).resolve().parents[1] / "target/debug/mycelium-map"


class CheckpointExportTests(unittest.TestCase):
    def test_quoted_typescript_field_preserves_colon_in_name(self):
        # Arrange: a valid quoted field name contains the same separator as full field types.
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "fields.ts"
            source.write_text('class Fields { "field: callback" = 1; }\n')
            analysis = root / "map.json"
            full_path = root / "full.md"
            subprocess.run(
                [str(BINARY), "analyze", str(root), "--quiet", "-o", str(analysis)],
                check=True, capture_output=True,
            )
            source.unlink()
            subprocess.run(
                [str(BINARY), "export", str(analysis), "--detail", "full",
                 "-o", str(full_path)],
                check=True, capture_output=True,
            )
            full = full_path.read_bytes()
            self.assertIn(b"#34;field: callback#34;: unknown", full)

            # Act: compare real full and compact CLI exports, without reopening source.
            compact = check_compact(BINARY, analysis, full, [])

            # Assert: the separator within the field name must not be removed as a type.
            self.assertIn(b"        #34;field: callback#34;\n", compact)

    def test_cpp_call_operator_keeps_its_name_and_overload_rows(self):
        # Arrange: a real C++ functor with two extracted overloads.
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "functor.cpp"
            source.write_text(
                "struct Functor {\n"
                "    int state;\n"
                "    int operator()(int arg) { return arg; }\n"
                "    double operator()(double arg) { return arg; }\n"
                "};\n"
            )
            analysis = root / "map.json"
            full_path = root / "full.md"
            subprocess.run(
                [str(BINARY), "analyze", str(root), "--quiet", "-o", str(analysis)],
                check=True, capture_output=True,
            )
            source.unlink()
            # C++ has no parenthesized field identifier; exercise that saved-map name explicitly.
            saved = json.loads(analysis.read_text())
            functor = next(c for c in saved["class_diagram"]["classes"] if c["name"] == "Functor")
            next(m for m in functor["members"] if m["name"] == "state")["name"] = "field(callback)"
            analysis.write_text(json.dumps(saved))
            subprocess.run(
                [str(BINARY), "export", str(analysis), "--detail", "full",
                 "-o", str(full_path)],
                check=True, capture_output=True,
            )
            full = full_path.read_bytes()
            self.assertIn(b"operator()(arg: int) int", full)
            self.assertIn(b"operator()(arg: double) double", full)
            self.assertIn(b"field(callback): int", full)

            # Act: exercise the checkpoint comparison using real native exports of saved facts.
            compact = check_compact(BINARY, analysis, full, [])

            # Assert: the full name and both overload rows survive.
            self.assertEqual(compact.count(b"        operator#40;#41;()\n"), 2)
            self.assertIn(b"        field#40;callback#41;\n", compact)

            # The comparison must still reject a lost name suffix or a missing overload row.
            for change in ("name", "overload"):
                with self.subTest(change=change):
                    altered = json.loads(json.dumps(saved))
                    members = next(c for c in altered["class_diagram"]["classes"]
                                   if c["name"] == "Functor")["members"]
                    if change == "name":
                        for member in members:
                            if member["name"] == "operator()":
                                member["name"] = "operator"
                    else:
                        members.remove(next(m for m in members if m["name"] == "operator()"))
                    analysis.write_text(json.dumps(altered))
                    with self.assertRaisesRegex(AssertionError, "Compact changed boxes"):
                        check_compact(BINARY, analysis, full, [])


if __name__ == "__main__":
    unittest.main()
