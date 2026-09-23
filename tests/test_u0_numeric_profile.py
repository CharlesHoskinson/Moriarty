import hashlib
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
    "checked-add-u128",
    "checked-add-u64",
    "checked-mul-u128",
    "checked-mul-u64",
    "checked-sub-u128",
    "expression-checked-add",
    "expression-checked-mul",
    "expression-checked-sub",
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
MISSING_RESERVE = "missing protocol-reserve posting"
RESERVE_SENTENCE = (
    "The successor sources have a missing protocol-reserve posting."
)
LIFECYCLE = "experiments/moriarty-language/src/successor/financial-lifecycle.ts"
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
    (root / PROFILE).write_text(
        json.dumps(profile, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


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
    assert profile["reserveMechanism"]["status"] == "absent"
    assert profile["reserveMechanism"]["citations"] == []
    for row in profile["primitives"]:
        if row["requiredDirection"] == "none":
            assert row["conformance"] == "conforms"
            assert row["gapNote"] is None
        else:
            assert row["conformance"] == "open-gap"
            assert MISSING_RESERVE in row["gapNote"]
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


def mark_conforms(profile: dict[str, Any], primitive_id: str) -> None:
    for row in profile["primitives"]:
        if row["id"] == primitive_id:
            row["conformance"] = "conforms"
            row["gapNote"] = None


def test_floor_match_stays_open_while_reserve_is_absent(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    mark_conforms(profile, "prorata-principal-share")
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "prorata-principal-share" in process.stdout
    assert "protocol-reserve posting is absent" in process.stdout
    assert process.stderr == ""


def test_local_reserve_variable_does_not_clear_the_gap(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product / total; const divisionRemainder = product % total; "
        "let protocolReserve = 0n; protocolReserve += divisionRemainder; "
        "if (protocolReserve > product) return bad('INVARIANT');",
    )
    profile = load_profile(root)
    mark_conforms(profile, "prorata-principal-share")
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "prorata-principal-share" in process.stdout
    assert "protocol-reserve posting is absent" in process.stdout
    assert "not a declaration" not in process.stdout
    assert process.stderr == ""


def test_local_variable_is_not_a_reserve_declaration(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    replace_once(
        path,
        "    dP = product / total;",
        "    dP = product / total;\n"
        "    let protocolReserve = 0n; protocolReserve += 0n;\n"
        "    if (protocolReserve > product) return bad('INVARIANT');",
    )
    line_no = (
        path.read_text(encoding="utf-8").splitlines().index(
            "    let protocolReserve = 0n; protocolReserve += 0n;"
        )
        + 1
    )
    profile = load_profile(root)
    profile["reserveMechanism"]["status"] = "present"
    profile["reserveMechanism"]["citations"] = [
        {"file": LIFECYCLE, "symbol": "protocolReserve", "line": line_no}
    ]
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "protocolReserve" in process.stdout
    assert "is not a declaration" in process.stdout
    assert process.stderr == ""


def test_declared_symbol_lifts_absence_without_a_posting(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    line_no = add_reserve_function(root)
    profile = load_profile(root)
    cite_reserve(profile, line_no)
    for row in profile["primitives"]:
        if isinstance(row["gapNote"], str):
            row["gapNote"] = row["gapNote"].replace(RESERVE_SENTENCE, "").strip()
        if row["id"] == "prorata-principal-share":
            row["conformance"] = "conforms"
            row["gapNote"] = None
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stdout == "OK: 14 primitives, 5 open conformance gaps\n"
    assert process.stderr == ""


def test_caller_local_reserve_does_not_clear_conversion_gaps(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "  const settlement = convertNominal(nominal, obligation.conversion);",
        "  const settlement = convertNominal(nominal, obligation.conversion); "
        "let protocolReserve = 0n; const remainder = nominal % 2n; "
        "protocolReserve += remainder; "
        "if (protocolReserve > nominal) return bad('INVARIANT');",
    )
    profile = load_profile(root)
    for primitive_id in (
        "origination-settlement-conversion",
        "repayment-settlement-conversion",
        "prorata-principal-share",
    ):
        mark_conforms(profile, primitive_id)
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "origination-settlement-conversion" in process.stdout
    assert "repayment-settlement-conversion" in process.stdout
    assert "prorata-principal-share" in process.stdout
    assert "protocol-reserve posting is absent" in process.stdout
    assert process.stderr == ""


def test_local_remainder_write_does_not_remove_reserve_gap_note(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "  const remainder = product % divisor;",
        "  const remainder = product % divisor; let protocolReserve = 0n; "
        "protocolReserve += remainder; "
        "if (protocolReserve > divisor) return bad('INVARIANT');",
    )
    profile = load_profile(root)
    for row in profile["primitives"]:
        if row["id"] in {
            "origination-settlement-conversion",
            "repayment-settlement-conversion",
        }:
            row["gapNote"] = (
                "Conversion.rounding lets the author select none, floor, or ceil."
            )
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "origination-settlement-conversion" in process.stdout
    assert "repayment-settlement-conversion" in process.stdout
    assert MISSING_RESERVE in process.stdout
    assert process.stderr == ""


def test_absent_reserve_gap_note_is_required(tmp_path: Path) -> None:
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
    assert MISSING_RESERVE in process.stdout
    assert process.stderr == ""


def add_reserve_function(root: Path) -> int:
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    addition = (
        "\nexport function protocolReserveAccount(): bigint {\n"
        "  return 0n;\n"
        "}\n"
    )
    text = path.read_text(encoding="utf-8") + addition
    path.write_text(text, encoding="utf-8")
    return text.splitlines().index("export function protocolReserveAccount(): bigint {") + 1


def cite_reserve(profile: dict[str, Any], line_no: int) -> None:
    profile["reserveMechanism"]["status"] = "present"
    profile["reserveMechanism"]["citations"] = [
        {
            "file": LIFECYCLE,
            "symbol": "protocolReserveAccount",
            "line": line_no,
        }
    ]


def write_decision(root: Path, text: str) -> None:
    (root / DECISION).write_text(text, encoding="utf-8")
    profile = load_profile(root)
    profile["decisionSha256"] = hashlib.sha256(text.encode("utf-8")).hexdigest()
    write_profile(root, profile)


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


def test_scale_above_uint256_digits_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["defiformalConversion"]["testVectors"][0]["quotePerBaseScale"] = 78
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "quotePerBaseScale" in process.stdout
    assert "maximum" in process.stdout
    assert process.stderr == ""


def test_scale_guard_rejects_huge_exponent_without_schema_maximum(
    tmp_path: Path,
) -> None:
    root = copy_tree(tmp_path)
    schema = json.loads((root / SCHEMA).read_text(encoding="utf-8"))
    vector = schema["properties"]["defiformalConversion"]["properties"]["testVectors"][
        "items"
    ]["properties"]
    del vector["quotePerBaseScale"]["maximum"]
    del vector["targetScale"]["maximum"]
    (root / SCHEMA).write_text(
        json.dumps(schema, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    profile = load_profile(root)
    profile["defiformalConversion"]["testVectors"][0]["quotePerBaseScale"] = 1_000_000
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "scale exceeds 77" in process.stdout
    assert process.stderr == ""


def test_reciprocal_above_bound_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["defiformalConversion"]["testVectors"][0][
        "expectedBasePerQuoteMantissa"
    ] = 10**154 + 1
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "expectedBasePerQuoteMantissa" in process.stdout
    assert "maximum" in process.stdout
    assert process.stderr == ""


def test_non_ascii_profile_is_accepted(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["priceOrientation"]["note"] += " — base per quote."
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""


def test_price_mapping_is_read_from_source(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-expression-v1.ts",
        "if(amount && other[0]==='Price' && other[2]===amount[1]) return ['ScaledAmount',other[1],other[3]];",
        "if(amount && other[0]==='Price' && other[2]===amount[1]) return ['Amount',other[1]];",
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "ScaledAmount" in process.stdout
    assert process.stderr == ""


def test_price_note_must_cite_the_mapping_line(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["priceOrientation"]["note"] = profile["priceOrientation"]["note"].replace(
        "financial-expression-v1.ts:341",
        "financial-expression-v1.ts:999",
    )
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "financial-expression-v1.ts:341" in process.stdout
    assert process.stderr == ""


def test_deleting_checked_add_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"] = [
        row for row in profile["primitives"] if row["id"] != "checked-add-u128"
    ]
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "financial-lifecycle.ts:342" in process.stdout
    assert "addU128" in process.stdout
    assert process.stderr == ""


def test_deleting_one_expression_operation_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    profile["primitives"] = [
        row for row in profile["primitives"] if row["id"] != "expression-checked-mul"
    ]
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "financial-expression-v1.ts:407" in process.stdout
    assert "Mul" in process.stdout
    assert process.stderr == ""


def test_new_overflow_helper_is_classified(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    addition = (
        "\nfunction addU256(a: bigint, b: bigint): bigint | null {\n"
        "  const sum = a + b;\n"
        "  return sum > UINT128_MAX ? null : sum;\n"
        "}\n"
    )
    text = path.read_text(encoding="utf-8") + addition
    path.write_text(text, encoding="utf-8")
    lines = text.splitlines()
    line_no = max(
        number
        for number, line in enumerate(lines, start=1)
        if line == "  return sum > UINT128_MAX ? null : sum;"
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"financial-lifecycle.ts:{line_no}" in process.stdout
    assert "addU256" in process.stdout
    assert process.stderr == ""


def test_role_evidence_must_match_source(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "principal - parts.value.dP",
        "principal - parts.value.share",
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "role evidence" in process.stdout
    assert "1423" in process.stdout
    assert "receipt" in process.stdout
    assert process.stderr == ""


@pytest.mark.parametrize(
    "prefix",
    [
        "  return /a\"/;\n",
        "  const kind = typeof /a\"/;\n",
        "  void /a\"/;\n",
        "  throw /a\"/;\n",
        "  yield /a\"/;\n",
        "  await /a\"/;\n",
        "  delete /a\"/;\n",
        "  else /a\"/;\n",
        "  new /a\"/;\n",
        "  do /a\"/;\n",
        "  case /a\"/:\n",
        "  const member = left in /a\"/;\n",
        "  const item = left of /a\"/;\n",
        "  const value = left instanceof /a\"/;\n",
    ],
)
def test_keyword_prefixed_regex_does_not_hide_a_division(
    tmp_path: Path, prefix: str
) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-lifecycle.ts"
    addition = (
        "\nfunction keywordRegex(left: bigint, right: bigint): bigint {\n"
        + prefix
        + "  return left / right;\n"
        + "}\n"
    )
    text = path.read_text(encoding="utf-8") + addition
    path.write_text(text, encoding="utf-8")
    lines = text.splitlines()
    division_line = lines.index("  return left / right;") + 1
    regex_line = lines.index(prefix.rstrip("\n")) + 1

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"financial-lifecycle.ts:{division_line}" in process.stdout
    assert f"financial-lifecycle.ts:{regex_line}" not in process.stdout
    assert process.stderr == ""


def test_reversed_obligation_keeps_vocabulary_and_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    text = (root / DECISION).read_text(encoding="utf-8")
    text = "`ceil` remains as vocabulary.\n\n" + text.replace(
        "rounds **up** (`ceil`)",
        "rounds **up** (`floor`)",
        1,
    )
    write_decision(root, text)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "defaultPolicy.obligation does not match derived D2 value floor"
        in process.stdout
    )
    assert process.stderr == ""


def test_reversed_beneficiary_keeps_vocabulary_and_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    text = (root / DECISION).read_text(encoding="utf-8")
    text = "protocol reserve remains as vocabulary.\n\n" + text.replace(
        "accrues to the **protocol reserve**",
        "accrues to the **solver wallet**",
        1,
    )
    write_decision(root, text)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "defaultPolicy.remainderBeneficiary does not match derived D2 value solver-wallet"
        in process.stdout
    )
    assert process.stderr == ""


def test_reversed_representation_keeps_vocabulary_and_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    text = (root / DECISION).read_text(encoding="utf-8")
    text = "exact domain-qualified integers remains as vocabulary.\n\n" + text.replace(
        "Amounts are exact domain-qualified integers",
        "Amounts are modular residues",
        1,
    )
    write_decision(root, text)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "units.representation does not match derived D4 value modular-residue"
        in process.stdout
    )
    assert process.stderr == ""


def test_reversed_field_element_rule_keeps_vocabulary_and_fails(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    text = (root / DECISION).read_text(encoding="utf-8")
    text = "They never become field elements remains as vocabulary.\n\n" + text.replace(
        "They never become field elements",
        "They can become field elements",
        1,
    )
    write_decision(root, text)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "D4 field-element rule is missing or ambiguous" in process.stdout
    assert process.stderr == ""


def test_two_d1_bolds_are_ambiguous(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    text = (root / DECISION).read_text(encoding="utf-8")
    text = text.replace(
        "**base-per-quote**",
        "**base-per-quote** and **quote-per-base**",
        1,
    )
    write_decision(root, text)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "D1 canonical orientation is missing or ambiguous" in process.stdout
    assert process.stderr == ""


def test_constructor_names_alone_are_not_role_evidence(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / SUCCESSOR / "financial-expression-v1.ts"
    replace_once(
        path,
        "n = k === 'CeilDiv' && r !== 0n ? q + 1n : q;",
        "n = q;",
    )
    source = path.read_text(encoding="utf-8")
    assert "FloorDiv" in source
    assert "CeilDiv" in source

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "author-selected-both-roles" in process.stdout
    assert "financial-expression-v1.ts:404" in process.stdout
    assert process.stderr == ""


def test_shared_division_rejects_a_second_obligation_row(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    profile = load_profile(root)
    original = next(
        row
        for row in profile["primitives"]
        if row["id"] == "expression-obligation-division"
    )
    extra = dict(original)
    extra["id"] = "expression-obligation-division-extra"
    profile["primitives"].append(extra)
    profile["primitives"].sort(key=lambda row: row["id"])
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "requires exactly one obligation row at that line" in process.stdout
    assert process.stderr == ""


def test_comment_does_not_satisfy_operation_regex(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product; // scale div floor ceil Rounding",
    )

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "has no division, rounding, scaling, or overflow-checked operation"
        in process.stdout
    )
    assert process.stderr == ""


def lift_reserve(root: Path, profile: dict[str, Any]) -> None:
    line_no = add_reserve_function(root)
    cite_reserve(profile, line_no)
    for row in profile["primitives"]:
        if isinstance(row["gapNote"], str):
            row["gapNote"] = row["gapNote"].replace(RESERVE_SENTENCE, "").strip()
        if row["id"] == "prorata-principal-share":
            row["conformance"] = "conforms"
            row["gapNote"] = None


def test_unrelated_ceil_tokens_do_not_flip_floor(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product / total; if (n !== 0n) { const bump = n + 1n; void bump; }",
    )
    profile = load_profile(root)
    lift_reserve(root, profile)
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stdout == "OK: 14 primitives, 5 open conformance gaps\n"
    assert process.stderr == ""


def test_ceil_increment_must_use_the_parsed_remainder(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    replace_once(
        root / SUCCESSOR / "financial-lifecycle.ts",
        "    dP = product / total;",
        "    dP = product / total; const dust = product % total; "
        "if (dust !== 0n) { dP = dP + 1n; }",
    )
    profile = load_profile(root)
    lift_reserve(root, profile)
    write_profile(root, profile)

    process = run(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "prorata-principal-share" in process.stdout
    assert "fixed direction ceil differs from required floor" in process.stdout
    assert "protocol-reserve posting is absent" not in process.stdout
    assert process.stderr == ""
