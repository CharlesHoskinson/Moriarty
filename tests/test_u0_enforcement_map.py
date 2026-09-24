from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
MAP_SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/enforcement-map.schema.json"
)
STAGE_SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
ARTIFACT = Path("deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json")
CHECKER = MORIARTY_ROOT / "scripts" / "check_u0_enforcement_map.py"
HOST_FILE = "experiments/moriarty-language/src/checker.ts"


def run_checker(root: Path | None = None) -> subprocess.CompletedProcess[str]:
    args = [sys.executable, str(CHECKER)]
    if root is not None:
        args.extend(["--root", str(root)])
    return subprocess.run(args, cwd=MORIARTY_ROOT, capture_output=True, text=True, check=False)


CITED_FILES = (
    "experiments/moriarty-language/src/checker.ts",
    "experiments/moriarty-language/src/runtime-types.ts",
)


def copy_root(tmp_path: Path) -> Path:
    """Copy the map inputs and only the cited host files. Native roots are real directories."""

    root = tmp_path / "root"
    for rel in (MAP_SCHEMA, STAGE_SCHEMA, ARTIFACT):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((MORIARTY_ROOT / rel).read_bytes())
    artifact = json.loads((root / ARTIFACT).read_text(encoding="utf-8"))
    for native in artifact["nativeRoots"]:
        (root / native).mkdir(parents=True, exist_ok=True)
    for rel in CITED_FILES:
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((MORIARTY_ROOT / rel).read_bytes())
    return root


def cite(payload: dict, field: str, mechanism: dict, status: str) -> None:
    target = row(payload, field)
    target["status"] = status
    target["mechanisms"] = [mechanism]


def load(root: Path) -> dict:
    return json.loads((root / ARTIFACT).read_text(encoding="utf-8"))


def write(root: Path, payload: dict) -> None:
    (root / ARTIFACT).write_text(
        json.dumps(payload, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def row(payload: dict, field: str) -> dict:
    return next(item for item in payload["rows"] if item["schemaField"] == field)


def test_real_artifacts_pass() -> None:
    process = run_checker()
    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    artifact = json.loads((MORIARTY_ROOT / ARTIFACT).read_text(encoding="utf-8"))
    assert artifact["schemaVersion"] == "moriarty-u0-enforcement-map/1"
    assert artifact["stageSchema"] == STAGE_SCHEMA.as_posix()
    assert artifact["nativeRoots"] == [
        "experiments/moriarty-language/compact",
        "experiments/moriarty-midnight-financial",
        "experiments/moriarty-midnight-network",
        "experiments/moriarty-native-ivc-r3",
    ]
    notes = {item["schemaField"]: item["note"] for item in artifact["rows"]}
    for field in (
        "signedIntent.grossDebitCap",
        "signedIntent.minNetOutcome",
        "signedIntent.recipients[]",
    ):
        assert "evaluate.ts:290-295" in notes[field]
        assert "OutcomeStatement" in notes[field]
        assert "does not bind" in notes[field] or "Neither check binds" in notes[field]
    assert "grossDebitCaps" in notes["signedIntent.grossDebitCap"]
    assert "minimumNetCredits" in notes["signedIntent.minNetOutcome"]
    assert "kernel.compact:74" in notes["signedIntent.minNetOutcome"]
    assert "permittedRecipients" in notes["signedIntent.recipients[]"]
    assert "kernel.compact:32" in notes["signedIntent.recipients[]"]
    assert "program-local lifetime counter" in notes["authority.remaining"]
    assert "observations.o0" in notes["observations[].time"]
    assert len(artifact["rows"]) == 84
    assert [item["schemaField"] for item in artifact["rows"]] == sorted(
        item["schemaField"] for item in artifact["rows"]
    )
    assert {item["status"] for item in artifact["rows"]} == {"NOT_ENFORCED"}
    assert all(item["mechanisms"] == [] for item in artifact["rows"])
    assert process.stdout == "OK: 84 leaf fields, 0 enforced, 0 host-only, 84 NOT_ENFORCED\n"


def test_c1_invalid_status_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "schemaVersion")["status"] = "closed"
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1
    assert "FAIL:" in process.stdout
    assert "closed" in process.stdout


def test_c2_wrong_schema_hash_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    payload["stageSchemaSha256"] = "0" * 64
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1
    assert "FAIL: stageSchemaSha256 does not equal the sha256 of the frozen schema" in process.stdout


def test_c3_deleted_row_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    payload["rows"] = [item for item in payload["rows"] if item["schemaField"] != "schemaVersion"]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1
    assert "not a bijection" in process.stdout
    assert "schemaVersion" in process.stdout


def test_c4_enforced_without_native_mechanism_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "schemaVersion")["status"] = "enforced"
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1
    assert "status enforced requires a non-host mechanism" in process.stdout


def test_c5_line_outside_file_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": HOST_FILE,
            "line": 10_000_000,
            "symbol": "semanticProfile",
            "note": "The cited line is outside the host file.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "line 10000000 is outside the file" in process.stdout


def test_c5_missing_cited_file_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    missing = "experiments/moriarty-language/src/missing-u0-citation.ts"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": missing,
            "line": 1,
            "symbol": "missing",
            "note": "The cited host file does not exist.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert f"blocked: missing {missing}" in process.stdout


def test_c6_symbol_absent_on_line_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": HOST_FILE,
            "line": 17,
            "symbol": "notASymbol",
            "note": "The cited symbol is absent from the host line.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol notASymbol is not an identifier on line 17" in process.stdout


def test_c6_symbol_inside_multiline_comment_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": "experiments/moriarty-language/src/runtime-types.ts",
            "line": 34,
            "symbol": "MC05",
            "note": "MC05 occurs only inside a block comment opened on line 33.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol MC05 is not an identifier on line 34" in process.stdout


def test_c6_symbol_after_closed_block_comment_passes(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": "experiments/moriarty-language/src/runtime-types.ts",
            "line": 42,
            "symbol": "authenticate",
            "note": "authenticate is code after the block comment closes on line 40.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_c7_circuit_file_outside_native_roots_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "enforced"
    target["mechanisms"] = [
        {
            "kind": "circuit",
            "file": HOST_FILE,
            "line": 17,
            "symbol": "semanticProfile",
            "note": "The cited host file is outside every declared native root.",
        }
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "circuit file is outside nativeRoots" in process.stdout


def test_c7_missing_native_root_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    payload["nativeRoots"] = [*payload["nativeRoots"], "experiments/moriarty-u0-absent-native-root"]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert "blocked: missing native root experiments/moriarty-u0-absent-native-root" in process.stdout


def test_wrong_typed_line_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    cite(
        payload,
        "schemaVersion",
        {
            "kind": "hostCheck",
            "file": HOST_FILE,
            "line": "17",
            "symbol": "semanticProfile",
            "note": "The line field is a string, not an integer.",
        },
        "host-only",
    )
    write(root, payload)
    index = next(
        position
        for position, item in enumerate(payload["rows"])
        if item["schemaField"] == "schemaVersion"
    )

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"FAIL: rows/{index}/mechanisms/0/line:" in process.stdout
    assert "Traceback" not in process.stderr
    assert process.stderr == ""


def test_internal_error_is_one_fail_line(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    schema = json.loads((root / MAP_SCHEMA).read_text(encoding="utf-8"))
    items = schema["properties"]["rows"]["items"]
    del items["properties"]
    # additionalProperties would reject every row before the missing-properties KeyError.
    del items["additionalProperties"]
    (root / MAP_SCHEMA).write_text(json.dumps(schema), encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout.startswith("FAIL: internal error: KeyError:")
    assert "\n" not in process.stdout.rstrip("\n")
    assert "Traceback" not in process.stderr
    assert process.stderr == ""


def test_c6_markers_inside_strings_stay_visible(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    relative = "experiments/moriarty-language/src/string-markers.ts"
    text = "\n".join(
        (
            'const label = "/* http://example.com/#hash"; const semanticProfile = 1;',
            "class Box { #hidden = semanticProfile; }",
            "",
        )
    )
    destination = root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(text, encoding="utf-8")
    payload = load(root)
    target = row(payload, "schemaVersion")
    target["status"] = "host-only"
    target["mechanisms"] = [
        {
            "kind": "hostCheck",
            "file": relative,
            "line": 1,
            "symbol": "semanticProfile",
            "note": "Comment markers inside a string do not hide the later identifier.",
        },
        {
            "kind": "hostCheck",
            "file": relative,
            "line": 2,
            "symbol": "semanticProfile",
            "note": "A TypeScript private-name hash is not a line comment.",
        },
    ]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert "Traceback" not in process.stderr


def test_c6_python_hash_hides_symbol(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    relative = "experiments/moriarty-language/src/hash_comment.py"
    destination = root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text('value = "kept"  # semanticProfile\n', encoding="utf-8")
    payload = load(root)
    cite(
        payload,
        "schemaVersion",
        {
            "kind": "hostCheck",
            "file": relative,
            "line": 1,
            "symbol": "semanticProfile",
            "note": "A Python hash comment hides the cited symbol.",
        },
        "host-only",
    )
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol semanticProfile is not an identifier on line 1" in process.stdout
    assert "Traceback" not in process.stderr


def test_c6_shell_hash_hides_symbol(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    relative = "experiments/moriarty-language/src/hash_comment.sh"
    destination = root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text('echo "kept"  # semanticProfile\n', encoding="utf-8")
    payload = load(root)
    cite(
        payload,
        "schemaVersion",
        {
            "kind": "hostCheck",
            "file": relative,
            "line": 1,
            "symbol": "semanticProfile",
            "note": "A shell hash comment hides the cited symbol.",
        },
        "host-only",
    )
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol semanticProfile is not an identifier on line 1" in process.stdout
    assert "Traceback" not in process.stderr


def test_c5_symlink_citation_outside_repository_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    outside = tmp_path / "outside.ts"
    outside.write_text("export const semanticProfile = 1;\n", encoding="utf-8")
    relative = "experiments/moriarty-language/src/escaped.ts"
    link = root / relative
    link.parent.mkdir(parents=True, exist_ok=True)
    link.symlink_to(outside)
    payload = load(root)
    cite(
        payload,
        "schemaVersion",
        {
            "kind": "hostCheck",
            "file": relative,
            "line": 1,
            "symbol": "semanticProfile",
            "note": "The citation symlink resolves outside the repository.",
        },
        "host-only",
    )
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "citation file resolves outside the repository" in process.stdout
    assert "Traceback" not in process.stderr


def test_c7_symlink_native_root_outside_repository_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    outside = tmp_path / "outside-native"
    outside.mkdir()
    relative = "experiments/moriarty-u0-escaped-native"
    (root / relative).symlink_to(outside, target_is_directory=True)
    payload = load(root)
    payload["nativeRoots"] = [relative]
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert f"blocked: missing native root {relative}" in process.stdout


def test_c7_symlink_file_leaves_native_root_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    relative = "experiments/moriarty-language/compact/escaped.compact"
    link = root / relative
    link.parent.mkdir(parents=True, exist_ok=True)
    link.symlink_to(root / HOST_FILE)
    payload = load(root)
    cite(
        payload,
        "schemaVersion",
        {
            "kind": "circuit",
            "file": relative,
            "line": 17,
            "symbol": "semanticProfile",
            "note": "The path is under a native root only until the symlink is resolved.",
        },
        "enforced",
    )
    write(root, payload)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "circuit file is outside nativeRoots" in process.stdout
    assert "Traceback" not in process.stderr
