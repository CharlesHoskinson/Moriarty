"""Negative and positive checks for the U0 target-pin ledger."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


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


def stage_ledger(tmp_path: Path) -> tuple[Path, dict[str, object]]:
    ledger_path = ROOT / LEDGER_REL
    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    relatives = {SCHEMA_REL, LEDGER_REL}
    for row in ledger["pins"]:
        for relative in row["sourceEvidence"]:
            relatives.add(Path(relative))
    for relative in relatives:
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((ROOT / relative).read_bytes())
    return tmp_path, ledger


def write_ledger(root: Path, ledger: dict[str, object]) -> None:
    path = root / LEDGER_REL
    path.write_text(json.dumps(ledger, indent=2) + "\n", encoding="utf-8")


def test_deleted_row_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    ledger["pins"] = [row for row in pins if row["component"] != "ledger"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout.startswith("FAIL:")


def test_forbidden_status_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    zkir = next(row for row in pins if row["component"] == "zkir")
    zkir["status"] = "current"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "current" in process.stdout


def test_pin_missing_from_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    zkir = next(row for row in pins if row["component"] == "zkir")
    zkir["pin"] = "0123456789abcdef0123456789abcdef01234567"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not occur" in process.stdout


def test_missing_evidence_file_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    ledger_row = next(row for row in pins if row["component"] == "ledger")
    ledger_row["sourceEvidence"] = [
        "deliverables/u0-semantic-contract-2026-09-23/missing-evidence.md"
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not exist" in process.stdout
