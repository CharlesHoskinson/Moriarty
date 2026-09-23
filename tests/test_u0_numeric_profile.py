import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts" / "check_u0_numeric_profile.py"
DECISION = Path("docs/decisions/u0-numeric-profile-decision.md")
SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/numeric-profile.schema.json"
)
PROFILE = Path("deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json")
SUCCESSOR = Path("experiments/moriarty-language/src/successor")

EXPECTED_IDS = [
    "accrual-interest",
    "expression-obligation-division",
    "expression-receipt-division",
    "origination-settlement-conversion",
    "prorata-principal-share",
    "repayment-settlement-conversion",
]
OPEN_GAPS = [
    "accrual-interest",
    "expression-obligation-division",
    "expression-receipt-division",
    "origination-settlement-conversion",
    "repayment-settlement-conversion",
]


def run(root: Path | None = None) -> subprocess.CompletedProcess[str]:
    command = [sys.executable, str(CHECKER)]
    if root is not None:
        command.extend(["--root", str(root)])
    return subprocess.run(command, cwd=ROOT, capture_output=True, text=True)


def copy_tree(tmp_path: Path) -> Path:
    root = tmp_path / "repo"
    for relative in (DECISION, SCHEMA, PROFILE):
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / relative, target)
    successor = root / SUCCESSOR
    successor.mkdir(parents=True, exist_ok=True)
    for source in (ROOT / SUCCESSOR).glob("*.ts"):
        shutil.copy2(source, successor / source.name)
    return root


def write_profile(root: Path, profile: dict[str, Any]) -> None:
    (root / PROFILE).write_text(json.dumps(profile, indent=2) + "\n", encoding="utf-8")


def load_profile(root: Path) -> dict[str, Any]:
    return json.loads((root / PROFILE).read_text(encoding="utf-8"))


def test_checker_accepts_real_profile() -> None:
    process = run()
    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    profile = load_profile(ROOT)
    ids = [row["id"] for row in profile["primitives"]]
    gaps = [
        row["id"]
        for row in profile["primitives"]
        if row["conformance"] == "open-gap"
    ]
    assert ids == EXPECTED_IDS
    assert gaps == OPEN_GAPS
    assert profile["overrides"] == []
    assert process.stdout == (
        f"OK: {len(ids)} primitives, {len(gaps)} open conformance gaps\n"
    )


def test_missing_decision_file_is_blocked(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    (root / DECISION).unlink()

    process = run(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert process.stdout == "blocked: decision file missing\n"
    assert process.stderr == ""


def test_deleted_primitive_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"] = [
        row
        for row in profile["primitives"]
        if row["id"] != "accrual-interest"
    ]
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "financial-lifecycle.ts:1781" in process.stdout
    assert process.stderr == ""


def test_nonexistent_symbol_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"][0]["symbol"] = "NotARealSymbol"
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "NotARealSymbol" in process.stdout
    assert process.stderr == ""


def test_forbidden_conformance_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"][0]["conformance"] = "closed"
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert process.stderr == ""


def test_wrong_conversion_vector_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    vector = profile["defiformalConversion"]["testVectors"][0]
    vector["expectedBasePerQuoteMantissa"] += 1
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "formula gives" in process.stdout
    assert process.stderr == ""
