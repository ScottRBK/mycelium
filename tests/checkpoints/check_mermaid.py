"""Check real repository source pinned to commits, without reading or changing working trees."""

import argparse
import hashlib
import json
import re
import subprocess
import tempfile
from pathlib import Path


def run(*args):
    return subprocess.check_output(args, stderr=subprocess.PIPE)


def check_compact(binary, analysis, full, options):
    """Check the default view twice from the same saved facts as the historical full view."""
    destination = analysis.with_suffix(".compact.md")
    outputs = []
    for _ in range(2):
        run(str(binary), "export", str(analysis), "-o", str(destination), *options)
        outputs.append(destination.read_bytes())
    assert outputs[0] == outputs[1], "Compact export is not repeatable"
    compact = outputs[0].decode()
    full = full.decode()

    def compact_row(row):
        if row.startswith("<<"):
            return row
        # Types containing parentheses use aliases. The final group is the method's arguments;
        # any earlier parentheses belong to its name (for example, C++ operator()).
        method = re.fullmatch(r"(.*)\([^()]*\)(?: .*)?", row)
        name = method.group(1) if method else row.rsplit(": ", 1)[0]
        name = name.replace("(", "#40;").replace(")", "#41;")
        return name + ("()" if method else "")

    def boxes(markdown, names_only=False):
        found = []
        pattern = r'^    class (c\d+)\["([^"]+)"\] \{(.*?)^    }'
        for box_id, label, body in re.findall(pattern, markdown, re.MULTILINE | re.DOTALL):
            rows = [line.strip() for line in body.splitlines() if line.strip()]
            if names_only:
                # Full retains the member name even when signature values use a key.
                rows = [compact_row(row) for row in rows]
            found.append((box_id, label, rows))
        return found

    # Lists retain order and duplicate overload rows, unlike name sets.
    assert boxes(compact) == boxes(full, names_only=True), (
        "Compact changed boxes, stereotypes, visibility, member names or row counts"
    )
    for heading in ("Source index", "Test filtering", "Extraction warnings"):
        pattern = rf"^## {heading}\n(.*?)(?=^## |\Z)"
        sections = []
        for markdown in (full, compact):
            match = re.search(pattern, markdown, re.MULTILINE | re.DOTALL)
            sections.append(match.group(1).strip() if match else "")
        assert sections[0] == sections[1], f"Compact changed {heading}"
    assert re.findall(r"^Included:.*", full, re.MULTILINE) == re.findall(
        r"^Included:.*", compact, re.MULTILINE
    ), "Compact changed included/omitted endpoint counts"
    for heading in ("Relationships", "Type key", "Signature key"):
        assert f"## {heading}\n" not in compact, f"Compact retained full-only {heading}"
    return outputs[0]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=Path("target/debug/mycelium-map"))
    parser.add_argument("--repo", action="append", default=[], metavar="NAME=PATH")
    parser.add_argument("--only", action="append", default=[])
    parser.add_argument("--artifacts", type=Path, required=True)
    args = parser.parse_args()
    overrides = dict(value.split("=", 1) for value in args.repo)
    checkpoints = json.loads(Path(__file__).with_name("mermaid.json").read_text())
    unknown = (set(args.only) | set(overrides)) - {c["name"] for c in checkpoints}
    if unknown:
        parser.error(f"Unknown checkpoint names: {', '.join(sorted(unknown))}")
    binary = args.binary.resolve()
    args.artifacts.mkdir(parents=True, exist_ok=True)
    failures = []
    for checkpoint in checkpoints:
        name = checkpoint["name"]
        if args.only and name not in args.only:
            continue
        try:
            repo = Path(overrides.get(name, checkpoint["local_path"])).expanduser()
            commit = checkpoint["commit"]
            actual = run("git", "-C", str(repo), "rev-parse", f"{commit}^{{commit}}")
            assert actual.decode().strip() == commit, "Checkpoint must pin a full commit ID"
            with tempfile.TemporaryDirectory(prefix=f"mycelium-{name}-") as directory:
                root = Path(directory)
                source = root / "source"
                source.mkdir()
                for filename in checkpoint["files"]:
                    path = source / filename
                    assert path.is_relative_to(source) and ".." not in Path(filename).parts
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(
                        run("git", "-C", str(repo), "show", f"{commit}:{filename}")
                    )
                outputs = []
                for repeat in range(2):
                    analysis = root / f"map-{repeat}.json"
                    markdown = root / f"diagram-{repeat}.md"
                    run(str(binary), "analyze", str(source), "--quiet", "-o", str(analysis))
                    run(str(binary), "export", str(analysis), "--detail", "full",
                        "--tests", "include", "-o", str(markdown))
                    outputs.append(markdown.read_bytes())
                assert outputs[0] == outputs[1], "Repeated analysis/export differs"
                output = outputs[0]
                (args.artifacts / f"{name}.md").write_bytes(output)
                for expected in checkpoint["contains"]:
                    message = f"Missing independently checked fact: {expected}"
                    assert expected.encode() in output, message
                for unexpected in checkpoint.get("absent", []):
                    assert unexpected.encode() not in output, f"Unexpected fact: {unexpected}"
                locations = {
                    (file, name): box_id
                    for box_id, name, file in re.findall(
                        r"^- (c\d+): `([^`]+)` — `([^`]+)`:", output.decode(), re.MULTILINE
                    )
                }
                for relationship in checkpoint.get("relationships", []):
                    source_id = locations[tuple(relationship["from"])]
                    target_id = locations[tuple(relationship["to"])]
                    expected = (
                        f"- {source_id} {relationship['arrow']} {target_id}: "
                        f"`{relationship['label']}`"
                    )
                    assert expected.encode() in output, f"Missing relationship: {relationship}"
                for owner, expected_members in checkpoint["members"].items():
                    pattern = r'class \w+\["' + re.escape(owner) + r'"\] \{(.*?)\n    }'
                    boxes = "\n".join(re.findall(pattern, output.decode(), re.DOTALL))
                    assert boxes, f"Missing owning class: {owner}"
                    for member in expected_members:
                        assert member in boxes, f"Missing {owner} member: {member}"
                digest = hashlib.sha256(output).hexdigest()
                assert digest == checkpoint["sha256"], f"Output changed: SHA256 {digest}"
                compact = check_compact(binary, analysis, output, ["--tests", "include"])
                (args.artifacts / f"{name}-compact.md").write_bytes(compact)
                filtering = checkpoint.get("filtering")
                if filtering:
                    for filename in filtering["files"]:
                        path = source / filename
                        assert path.is_relative_to(source) and ".." not in Path(filename).parts
                        path.parent.mkdir(parents=True, exist_ok=True)
                        path.write_bytes(
                            run("git", "-C", str(repo), "show", f"{commit}:{filename}")
                        )
                    filtered_outputs = []
                    for repeat in range(2):
                        run(str(binary), "analyze", str(source), "--quiet", "-o", str(analysis))
                        run(str(binary), "export", str(analysis), "--detail", "full",
                            "-o", str(markdown), *filtering["options"])
                        filtered_outputs.append(markdown.read_bytes())
                    assert filtered_outputs[0] == filtered_outputs[1], "Filtering is not repeatable"
                    filtered = filtered_outputs[0]
                    (args.artifacts / f"{name}-filtered.md").write_bytes(filtered)
                    run(str(binary), "export", str(analysis), "--detail", "full",
                        "--tests", "include", "-o", str(markdown))
                    included = markdown.read_bytes()
                    (args.artifacts / f"{name}-filtering-include.md").write_bytes(included)
                    for expected in filtering["contains"]:
                        assert expected in " ".join(filtered.decode().split()), (
                            f"Missing retained fact: {expected}"
                        )
                    for removed in filtering["absent"]:
                        assert removed.encode() in included, f"Invalid removal assertion: {removed}"
                        assert removed.encode() not in filtered, f"Test fact leaked: {removed}"
                    filtered_hash = hashlib.sha256(filtered).hexdigest()
                    assert filtered_hash == filtering["sha256"], (
                        f"Filtered output changed: SHA256 {filtered_hash}"
                    )
                    compact = check_compact(binary, analysis, filtered, filtering["options"])
                    (args.artifacts / f"{name}-filtered-compact.md").write_bytes(compact)
                    automatic = filtering.get("automatic")
                    if automatic:
                        automatic_outputs = []
                        for repeat in range(2):
                            run(str(binary), "analyze", str(source), "--quiet", "-o", str(analysis))
                            run(str(binary), "export", str(analysis), "--detail", "full",
                                "-o", str(markdown))
                            automatic_outputs.append(markdown.read_bytes())
                        assert automatic_outputs[0] == automatic_outputs[1], (
                            "Framework detection is not repeatable"
                        )
                        automatic_output = automatic_outputs[0]
                        (args.artifacts / f"{name}-automatic.md").write_bytes(automatic_output)
                        for expected in automatic["contains"]:
                            assert expected in " ".join(automatic_output.decode().split()), (
                                f"Missing automatic retained fact: {expected}"
                            )
                        for removed in automatic["absent"]:
                            assert removed.encode() in included, f"Invalid source fact: {removed}"
                            assert removed.encode() not in automatic_output, (
                                f"Framework test fact leaked: {removed}"
                            )
                        automatic_hash = hashlib.sha256(automatic_output).hexdigest()
                        assert automatic_hash == automatic["sha256"], (
                            f"Automatic output changed: SHA256 {automatic_hash}"
                        )
                        compact = check_compact(binary, analysis, automatic_output, [])
                        (args.artifacts / f"{name}-automatic-compact.md").write_bytes(compact)
            print(f"PASS {name} @ {commit[:12]}")
        except (AssertionError, KeyError, OSError, subprocess.CalledProcessError) as error:
            failures.append(name)
            detail = (
                error.stderr.decode() if isinstance(error, subprocess.CalledProcessError) else error
            )
            print(f"FAIL {name}: {detail}")
    if failures:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
