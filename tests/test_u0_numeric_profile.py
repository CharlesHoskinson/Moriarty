import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

import pytest


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
    "prorata-principal-share",
    "repayment-settlement-conversion",
]
SHARED_LINE_ROWS = {
    "expression-obligation-division": "obligation",
    "expression-receipt-division": "receipt",
    "origination-settlement-conversion": "receipt",
    "repayment-settlement-conversion": "obligation",
}


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


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    assert count == 1, (old, count)
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


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
    assert "primitives/0/conformance" in process.stdout
    assert "closed" in process.stdout
    assert process.stderr == ""


def test_false_conforms_on_unposted_remainder_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    for row in profile["primitives"]:
        if row["id"] == "prorata-principal-share":
            row["conformance"] = "conforms"
            row["gapNote"] = None
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "prorata-principal-share" in process.stdout
    assert "remainder is not posted to a protocol reserve" in process.stdout
    assert "source requires open-gap" in process.stdout
    assert process.stderr == ""


def test_closure_reserve_posting_stays_open(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product / total; closureReserve += remainder;",
    )
    profile = load_profile(root)
    for row in profile["primitives"]:
        if row["id"] == "prorata-principal-share":
            row["conformance"] = "conforms"
            row["gapNote"] = None
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "remainder is not posted to a protocol reserve" in process.stdout
    assert process.stderr == ""


def test_protocol_reserve_posting_can_conform(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product / total; protocolReserve += remainder;",
    )
    profile = load_profile(root)
    for row in profile["primitives"]:
        if row["id"] == "prorata-principal-share":
            row["conformance"] = "conforms"
            row["gapNote"] = None
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stdout == "OK: 6 primitives, 5 open conformance gaps\n"
    assert process.stderr == ""


def test_ceil_path_requires_beneficiary_caveat(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    for row in profile["primitives"]:
        if row["id"] == "accrual-interest":
            row["gapNote"] = (
                "AccrualTerms.rounding lets the author select floor or ceil. "
                "A floor selection is an open conformance gap."
            )
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "accrual-interest" in process.stdout
    assert "remainder is not posted to a protocol reserve" in process.stdout
    assert process.stderr == ""


@pytest.mark.parametrize("dropped,role", list(SHARED_LINE_ROWS.items()))
def test_removing_shared_line_row_fails(
    tmp_path: Path, dropped: str, role: str
) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"] = [
        row for row in profile["primitives"] if row["id"] != dropped
    ]
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"role {role} " in process.stdout
    assert "is not covered by a primitive within 15 lines" in process.stdout
    assert process.stderr == ""


def test_width_bound_uses_numeric_fits_shift(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-expression-types-v1.ts"
    replace_once(
        path,
        "['UInt256','AmountProduct','ScaledAmount'].includes(tag) ? 256n : 128n",
        "['UInt256','AmountProduct','ScaledAmount'].includes(tag) ? 256n : 129n",
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "UInt128" in process.stdout
    assert "numericFits" in process.stdout
    assert process.stderr == ""


def test_signed_width_requires_shift_expression(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-expression-types-v1.ts"
    replace_once(
        path,
        "return n >= -(1n << 127n) && n < (1n << 127n);",
        "return n >= -(127n) && n < (127n);",
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "SInt128" in process.stdout
    assert "1n << 127n" in process.stdout
    assert process.stderr == ""


def test_unspaced_division_is_covered(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    addition = (
        "\nfunction unspacedQuotient(a: bigint, b: bigint): bigint {\n"
        "  return a/b;\n"
        "}\n"
    )
    text = path.read_text(encoding="utf-8") + addition
    path.write_text(text, encoding="utf-8")
    line_no = text.splitlines().index("  return a/b;") + 1

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"financial-lifecycle.ts:{line_no}" in process.stdout
    assert "division site" in process.stdout
    assert process.stderr == ""


def test_comment_and_string_slashes_are_not_divisions(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    path.write_text(
        path.read_text(encoding="utf-8")
        + "\nfunction notADivision(label: string): string {\n"
        + "  // return a / b;\n"
        + "  /* return a / b */\n"
        + "  const path = 'a/b';\n"
        + "  const ident = /[a/b]/u;\n"
        + "  return path + ident.source + label;\n"
        + "}\n",
        encoding="utf-8",
    )

    process = run(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""


@pytest.mark.parametrize(
    "relative",
    [
        "docs/decisions/u0-numeric-profile-decision.md",
        "deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json",
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts",
    ],
)
def test_non_utf8_input_fails(tmp_path: Path, relative: str) -> None:
    root = copy_tree(tmp_path)
    target = root / relative
    target.write_bytes(target.read_bytes() + b"\xff")

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout == f"FAIL: {relative} is not UTF-8\n"
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
