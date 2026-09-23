"""Negative and positive checks for the U0 target-pin ledger."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts" / "check_u0_target_pins.py"
LEDGER_REL = Path("deliverables/u0-semantic-contract-2026-09-23/target-pins.json")
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/target-pins.schema.json"
)
COMPONENTS = [
    "moriarty-compiler",
    "compact-compiler",
    "zkir",
    "native-proof-system",
    "verifier",
    "proving-keys",
    "verifier-keys",
    "srs-parameters",
    "ledger",
    "proof-server",
    "k-reference-toolchain",
]
PATH_RE = re.compile(
    r"(?<![A-Za-z0-9_./-])"
    r"(?:deliverables|docs|experiments|openspec|scripts|tests)"
    r"(?:/[A-Za-z0-9_+-]+)+(?:\.[A-Za-z0-9]+)+"
)
CONFLICT_HASH = "2ffe2d17bbb736aec36fb300aeaca679a10d2278"


def run_checker(root: Path | None = None) -> subprocess.CompletedProcess[str]:
    command = [sys.executable, str(CHECKER)]
    if root is not None:
        command.extend(["--root", str(root)])
    return subprocess.run(command, cwd=ROOT, capture_output=True, text=True)


def test_real_ledger_passes() -> None:
    process = run_checker()
    ledger = json.loads((ROOT / LEDGER_REL).read_text(encoding="utf-8"))

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    assert (
        process.stdout
        == "OK: 8 historical, 3 absent, compatible tuple NOT established\n"
    )
    assert ledger["schemaVersion"] == "moriarty-u0-target-pins/1"
    assert ledger["asOf"] == "2026-09-23"
    assert ledger["compatibleTupleEstablished"] is False
    assert [row["component"] for row in ledger["pins"]] == COMPONENTS
    assert any("transcript mismatch" in item for item in ledger["unresolved"])
    assert any("recency order" in item for item in ledger["unresolved"])
    historical = [row["component"] for row in ledger["pins"] if row["status"] == "historical"]
    absent = [row["component"] for row in ledger["pins"] if row["status"] == "absent"]
    assert historical == [
        "compact-compiler",
        "zkir",
        "native-proof-system",
        "verifier",
        "srs-parameters",
        "ledger",
        "proof-server",
        "k-reference-toolchain",
    ]
    assert absent == ["moriarty-compiler", "proving-keys", "verifier-keys"]
    zkir = next(row for row in ledger["pins"] if row["component"] == "zkir")
    ledger_row = next(row for row in ledger["pins"] if row["component"] == "ledger")
    srs = next(row for row in ledger["pins"] if row["component"] == "srs-parameters")
    assert "ancestor" not in zkir["note"]
    assert "ancestor" not in ledger_row["note"]
    assert "midnight-ledger checkout" not in ledger_row["note"]
    assert "bls_midnight_2p17" in srs["note"]
    assert "a8ab82ba2124c36f92795c683e70bd888bc1d1fb" in srs["note"]


def stage_ledger(tmp_path: Path) -> tuple[Path, dict[str, object]]:
    ledger_path = ROOT / LEDGER_REL
    raw = ledger_path.read_text(encoding="utf-8")
    ledger = json.loads(raw)
    relatives = {SCHEMA_REL, LEDGER_REL}
    for match in PATH_RE.finditer(raw):
        relatives.add(Path(match.group(0)))
    pins = ledger["pins"]
    assert isinstance(pins, list)
    for row in pins:
        for relative in row["sourceEvidence"]:
            relatives.add(Path(relative))
    for relative in relatives:
        source = ROOT / relative
        if not source.is_file():
            continue
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())
    return tmp_path, ledger


def write_ledger(root: Path, ledger: dict[str, object]) -> None:
    path = root / LEDGER_REL
    path.write_text(json.dumps(ledger, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def row(ledger: dict[str, object], component: str) -> dict[str, object]:
    pins = ledger["pins"]
    assert isinstance(pins, list)
    return next(item for item in pins if item["component"] == component)


def test_deleted_row_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    ledger["pins"] = [item for item in pins if item["component"] != "ledger"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "exactly one row per component" in process.stdout


def test_forbidden_status_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["status"] = "current"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "current" in process.stdout


def test_pin_missing_from_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["pin"] = "0123456789abcdef0123456789abcdef01234567"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not contain pin" in process.stdout


def test_missing_evidence_file_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "ledger")["sourceEvidence"] = [
        "deliverables/consolidated-design-2026-09-19/missing-evidence.md"
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not exist" in process.stdout


def test_conflict_hash_typo_in_note_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    note = zkir["note"]
    assert isinstance(note, str)
    assert CONFLICT_HASH in note
    zkir["note"] = note.replace(CONFLICT_HASH, CONFLICT_HASH[:-1] + "9")
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert CONFLICT_HASH[:-1] + "9" in process.stdout


def test_pin_inside_longer_token_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    relative = "deliverables/consolidated-design-2026-09-19/longer-token.txt"
    evidence = tmp_path / relative
    evidence.write_text("compiler 10.31.12 and token xx0.31.1yy\n", encoding="utf-8")
    evidence_rows = compact["sourceEvidence"]
    assert isinstance(evidence_rows, list)
    compact["sourceEvidence"] = [*evidence_rows, relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not contain pin" in process.stdout
    assert relative in process.stdout


def test_ledger_self_citation_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["sourceEvidence"] = [LEDGER_REL.as_posix()]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "generated artifact" in process.stdout


def test_absent_compact_compiler_fails_when_source_records_pin(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "compact-compiler" in process.stdout
    assert "absent" in process.stdout
    assert "deliverables/lifecycle-corpus-2026-09-17/environment.json" in process.stdout


def test_reverified_compatible_note_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["note"] = "This pin was re-verified and the tuple is compatible."
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "re-verification or compatibility" in process.stdout


def test_malformed_pin_kind_fails_without_traceback(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["pinKind"] = {"not": "a-string"}
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "Traceback" not in process.stderr
    assert "Traceback" not in process.stdout


def test_non_ascii_note_is_canonical(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    note = zkir["note"]
    assert isinstance(note, str)
    zkir["note"] = note.replace("local inspection", "local inspection \u2013 snapshot")
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_non_canonical_json_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    path = root / LEDGER_REL
    path.write_text(json.dumps(ledger, indent=4, ensure_ascii=False) + "\n", encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "not canonical 2-space JSON" in process.stdout


@pytest.mark.parametrize(
    ("mutate", "expected"),
    [
        ("compatible-true", "compatibleTupleEstablished is not false"),
        ("empty-unresolved", "unresolved is empty"),
        ("absent-with-pin", "absent <=> pin null <=> pinKind none"),
    ],
)
def test_ledger_rule_failures(tmp_path: Path, mutate: str, expected: str) -> None:
    root, ledger = stage_ledger(tmp_path)
    if mutate == "compatible-true":
        ledger["compatibleTupleEstablished"] = True
    elif mutate == "empty-unresolved":
        ledger["unresolved"] = []
    elif mutate == "absent-with-pin":
        proving = row(ledger, "proving-keys")
        proving["pin"] = "not-a-recorded-pin"
    else:
        raise AssertionError(mutate)
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert expected in process.stdout


def test_absent_row_with_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    proving = row(ledger, "proving-keys")
    proving["sourceEvidence"] = [
        "experiments/moriarty-language/package.json",
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "absent row has sourceEvidence" in process.stdout


def test_missing_schema_is_blocked(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).unlink()

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert "blocked:" in process.stdout
