"""Checker and content tests for the U0 trust premises and backend matrix."""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts" / "check_u0_trust_backend_matrix.py"
GENERATOR = ROOT / "scripts" / "build_u0_backend_matrix.py"
SOURCE_REL = "docs/MORIARTY-BACKEND-REQUIREMENTS.md"
MATRIX_REL = "deliverables/u0-semantic-contract-2026-09-23/backend-requirement-matrix.json"
TRUST_REL = "deliverables/u0-semantic-contract-2026-09-23/trust-premises.json"
MATRIX_SCHEMA_REL = (
    "openspec/changes/consolidated-language-kernel/schemas/"
    "backend-requirement-matrix.schema.json"
)
TRUST_SCHEMA_REL = (
    "openspec/changes/consolidated-language-kernel/schemas/"
    "trust-premises.schema.json"
)


def run_script(script: Path, root: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(script), "--root", str(root), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def materialize(tmp_path: Path) -> Path:
    root = tmp_path / "repo"
    premises = json.loads((ROOT / TRUST_REL).read_text(encoding="utf-8"))
    relative_paths = {
        SOURCE_REL,
        MATRIX_REL,
        TRUST_REL,
        MATRIX_SCHEMA_REL,
        TRUST_SCHEMA_REL,
    }
    for premise in premises["premises"]:
        for ref in premise["sourceRefs"]:
            relative_paths.add(ref["path"])
    for relative in relative_paths:
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((ROOT / relative).read_bytes())
    return root


def write_json(path: Path, payload: object) -> None:
    path.write_text(
        json.dumps(payload, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def test_id_lists_expand_ranges_and_short_refines() -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    from build_u0_backend_matrix import expand_id_list, expand_refines

    assert expand_id_list("ZR01–ZR03") == ["ZR01", "ZR02", "ZR03"]
    assert expand_id_list("ZR04, ZR07") == ["ZR04", "ZR07"]
    assert expand_id_list("ZR12, ZR13") == ["ZR12", "ZR13"]
    assert expand_refines("ZR01/03/06", "MNR01") == ["ZR01", "ZR03", "ZR06"]
    assert expand_refines("ZR04/07/08/09/16", "MNR02") == [
        "ZR04",
        "ZR07",
        "ZR08",
        "ZR09",
        "ZR16",
    ]


def test_checker_accepts_committed_artifacts() -> None:
    generated = run_script(GENERATOR, ROOT, "--check")
    checked = run_script(CHECKER, ROOT)

    assert generated.returncode == 0, generated.stdout + generated.stderr
    assert generated.stdout == "OK: backend matrix matches source\n"
    assert checked.returncode == 0, checked.stdout + checked.stderr
    assert checked.stdout == "OK: 24 backend rows specified-only, 7 trust premises\n"
    assert checked.stderr == ""


def test_matrix_rows_match_source_document_text() -> None:
    source = (ROOT / SOURCE_REL).read_text(encoding="utf-8")
    matrix = json.loads((ROOT / MATRIX_REL).read_text(encoding="utf-8"))
    digest = hashlib.sha256((ROOT / SOURCE_REL).read_bytes()).hexdigest()

    assert matrix["sourceSha256"] == digest
    assert [row["id"] for row in matrix["rows"]] == [
        f"ZR{number:02d}" for number in range(1, 17)
    ] + [f"MNR{number:02d}" for number in range(1, 9)]
    by_id = {row["id"]: row for row in matrix["rows"]}
    assert by_id["ZR01"]["title"] == "C, versioned interface"
    assert by_id["ZR01"]["class"] == "C"
    assert by_id["ZR13"]["class"] == "C/P"
    assert by_id["ZR13"]["title"] == "C/P, honest cost and feasibility reporting"
    assert by_id["ZR01"]["owner"] == (
        "ZKIR/compiler/proof-runtime and ledger release interface"
    )
    assert "U0/U1" in by_id["ZR01"]["milestone"]
    assert by_id["MNR01"]["class"] is None
    assert by_id["MNR01"]["owner"] is None
    assert by_id["MNR01"]["milestone"] is None
    assert by_id["MNR01"]["presentSupport"] is None
    assert by_id["MNR01"]["refines"] == ["ZR01", "ZR03", "ZR06"]
    assert by_id["MNR08"]["refines"] == ["ZR01", "ZR06", "ZR14", "ZR16"]
    for row in matrix["rows"]:
        assert row["status"] == "specified-only"
        assert row["requirement"] in source
        assert row["acceptance"] in source
        assert "SHALL" in row["requirement"]


def test_trust_premises_cover_required_topics() -> None:
    premises = json.loads((ROOT / TRUST_REL).read_text(encoding="utf-8"))
    by_id = {premise["id"]: premise for premise in premises["premises"]}

    assert list(by_id) == [f"TP{number:02d}" for number in range(1, 8)]
    assert by_id["TP01"]["kind"] == "unresolved-interface"
    assert "ZKIRv3" in by_id["TP01"]["statement"]
    assert by_id["TP02"]["kind"] == "planning-assumption"
    assert "March 2027" in by_id["TP02"]["statement"]
    assert by_id["TP03"]["kind"] == "trust-assumption"
    assert "oracle" in by_id["TP03"]["statement"]
    assert "federation" in by_id["TP04"]["statement"].lower()
    assert "Moriarty can also run on Midnight without this federation." in by_id["TP04"]["statement"]
    assert by_id["TP04"]["relatedIds"] == ["MPLR-030", "UNI-011", "ZR14"]
    assert all(
        "no federation requirement for direct Midnight use" not in ref["quote"]
        for ref in by_id["TP04"]["sourceRefs"]
    )
    assert any(
        ref["path"] == "docs/MORIARTY-CONSOLIDATED-DESIGN.md" for ref in by_id["TP04"]["sourceRefs"]
    )
    assert any(ref["path"].endswith("/proposal.md") for ref in by_id["TP04"]["sourceRefs"])
    assert "ZR15" not in by_id["TP04"]["statement"]
    assert "UNI-015" not in by_id["TP04"]["statement"]
    assert "nonexecution" in by_id["TP05"]["statement"]
    assert "witness" in by_id["TP06"]["statement"]
    assert "circuit" in by_id["TP07"]["statement"]
    for premise in premises["premises"]:
        assert premise["status"] in {"open", "accepted-assumption"}
        for ref in premise["sourceRefs"]:
            assert len(ref["quote"]) >= 20
            assert ref["quote"] in (ROOT / ref["path"]).read_text(encoding="utf-8")


def test_checker_rejects_deleted_matrix_row(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    matrix_path = root / MATRIX_REL
    matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
    del matrix["rows"][0]
    write_json(matrix_path, matrix)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "Traceback" not in result.stderr
    assert "FAIL: backend matrix drifted" in result.stdout
    assert "FAIL: backend row ids are not exactly ZR01-ZR16 and MNR01-MNR08" in result.stdout


def test_checker_rejects_forbidden_status(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    matrix_path = root / MATRIX_REL
    matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
    matrix["rows"][0]["status"] = "executed"
    write_json(matrix_path, matrix)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "Traceback" not in result.stderr
    assert "FAIL: ZR01 status is not specified-only" in result.stdout


def test_checker_rejects_quote_not_in_source(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0]["sourceRefs"][0]["quote"] = (
        "THIS QUOTE IS NOT IN THE SOURCE FILE XXX"
    )
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "Traceback" not in result.stderr
    assert (
        "FAIL: TP01 quote not in docs/MORIARTY-PRODUCT-CONTRACT.md" in result.stdout
    )
    assert "OK:" not in result.stdout


def test_split_cells_unescapes_only_pipes() -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    from build_u0_backend_matrix import split_cells

    assert split_cells("| a\\|b | c\\d |") == ["a|b", "c\\d"]
    assert split_cells("| a\\\\|b | c |") == ["a\\", "b", "c"]
    assert split_cells("| a\\\\\\|b |") == ["a\\|b"]


def test_generator_rejects_changed_capability_header(tmp_path: Path) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    from build_u0_backend_matrix import CAPABILITY_HEADER

    root = materialize(tmp_path)
    source = root / SOURCE_REL
    text = source.read_text(encoding="utf-8")
    assert text.count(CAPABILITY_HEADER) == 1
    source.write_text(
        text.replace(CAPABILITY_HEADER, CAPABILITY_HEADER + " changed", 1),
        encoding="utf-8",
    )
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    result = run_script(GENERATOR, root)

    assert result.returncode == 1
    assert result.stdout == f"FAIL: table format changed: {CAPABILITY_HEADER}\n"
    assert "Traceback" not in result.stderr
    assert matrix.read_bytes() == before


def test_generator_rejects_duplicate_capability_header(tmp_path: Path) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    from build_u0_backend_matrix import CAPABILITY_HEADER

    root = materialize(tmp_path)
    source = root / SOURCE_REL
    text = source.read_text(encoding="utf-8")
    source.write_text(
        text.replace(CAPABILITY_HEADER, CAPABILITY_HEADER + "\n" + CAPABILITY_HEADER, 1),
        encoding="utf-8",
    )
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    result = run_script(GENERATOR, root)

    assert result.returncode == 1
    assert result.stdout == f"FAIL: table format changed: {CAPABILITY_HEADER}\n"
    assert "Traceback" not in result.stderr
    assert matrix.read_bytes() == before


def test_source_edit_fails_check_and_reports_hash_mismatch(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    source = root / SOURCE_REL
    text = source.read_text(encoding="utf-8")
    assert text.count("SRS provenance") == 1
    source.write_text(text.replace("SRS provenance", "SRS provenances", 1), encoding="utf-8")
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    generated = run_script(GENERATOR, root, "--check")
    checked = run_script(CHECKER, root)

    assert generated.returncode == 1
    assert generated.stdout == "FAIL: backend matrix drifted\n"
    assert checked.returncode == 1
    assert "FAIL: backend matrix drifted" in checked.stdout
    assert (
        "FAIL: sourceSha256 does not match docs/MORIARTY-BACKEND-REQUIREMENTS.md"
        in checked.stdout
    )
    assert "OK:" not in checked.stdout
    assert "Traceback" not in generated.stderr
    assert "Traceback" not in checked.stderr
    assert matrix.read_bytes() == before


def test_generator_rejects_responsibility_id_outside_requirement_set(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    source = root / SOURCE_REL
    text = source.read_text(encoding="utf-8")
    needle = (
        "| ZR16 | Native library/tooling and actual ledger configuration | "
        "Independent retained proof and effect verification; U4/U7 |"
    )
    assert needle in text
    source.write_text(
        text.replace(needle, needle + "\n| ZR99 | Extra owner | Extra milestone |", 1),
        encoding="utf-8",
    )
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    result = run_script(GENERATOR, root)

    assert result.returncode == 1
    assert result.stdout == (
        "FAIL: responsibility table ids outside requirement set: ZR99\n"
    )
    assert "Traceback" not in result.stderr
    assert matrix.read_bytes() == before


def test_checker_rejects_unresolved_related_ids(tmp_path: Path) -> None:
    cases = (
        ("ZR99", "FAIL: TP01 related id ZR99 is not in the backend matrix"),
        ("MNR42", "FAIL: TP01 related id MNR42 is not in the backend matrix"),
        (
            "UNI-999",
            "FAIL: TP01 related id UNI-999 is not a requirement heading or traceability row",
        ),
    )
    for related_id, message in cases:
        root = materialize(tmp_path / related_id)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premises["premises"][0]["relatedIds"] = [related_id]
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert message in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_related_id_missing_from_quotes(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    premise["sourceRefs"] = [
        ref
        for ref in premise["sourceRefs"]
        if all(token not in ref["quote"] for token in ("ZR01", "UNI-003", "UNI-017"))
    ]
    assert premise["sourceRefs"]
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "FAIL: TP01 related id ZR01 is not in a cited quote" in result.stdout
    assert "FAIL: TP01 related id UNI-017 is not in a cited quote" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_deleted_trust_premises(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"] = premises["premises"][:1]
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "FAIL: missing trust premise TP02" in result.stdout
    assert "FAIL: missing trust premise TP07" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_false_premise_claim(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    premise["statement"] += " This release is verified and closed."
    assert "a comprehensive compatible release tuple is unknown." in premise["statement"]
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "FAIL: TP01 statement does not match the source-backed contract" in result.stdout
    assert "lacks source-backed claim" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_claim_not_in_cited_quote(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    needle = "a comprehensive compatible release tuple is unknown."
    replacement = (
        "Separate local pins and the historical contract/inner transcript mismatch are known"
    )
    replaced = False
    for ref in premise["sourceRefs"]:
        if needle in ref["quote"]:
            ref["quote"] = ref["quote"].replace(needle, replacement)
            replaced = True
    assert replaced
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert (
        "FAIL: TP01 claim is not in a cited quote: "
        "a comprehensive compatible release tuple is unknown."
        in result.stdout
    )
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_unsupported_premise_status(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    assert premises["premises"][0]["status"] == "open"
    premises["premises"][0]["status"] = "accepted-assumption"
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "FAIL: TP01 status is not open" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_schema_that_is_not_an_object(tmp_path: Path) -> None:
    for name, relative in (("trust", TRUST_SCHEMA_REL), ("matrix", MATRIX_SCHEMA_REL)):
        root = materialize(tmp_path / name)
        (root / relative).write_text("[]\n", encoding="utf-8")

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert f"FAIL: {relative} schema is not an object" in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_generator_rejects_refines_outside_zr_set(tmp_path: Path) -> None:
    cases = (
        (
            "zr99",
            "| ZR01/03/99 |",
            "FAIL: refines id outside ZR01-ZR16: MNR01: ZR99\n",
        ),
        (
            "zr00",
            "| ZR00/03/06 |",
            "FAIL: refines id outside ZR01-ZR16: MNR01: ZR00\n",
        ),
    )
    for name, replacement, message in cases:
        root = materialize(tmp_path / name)
        source = root / SOURCE_REL
        text = source.read_text(encoding="utf-8")
        needle = "| ZR01/03/06 |"
        assert text.count(needle) == 1
        source.write_text(text.replace(needle, replacement, 1), encoding="utf-8")
        matrix = root / MATRIX_REL
        before = matrix.read_bytes()

        result = run_script(GENERATOR, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert result.stdout == message
        assert "Traceback" not in result.stderr
        assert matrix.read_bytes() == before


def test_generator_rejects_pipe_after_two_backslashes(tmp_path: Path) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    from build_u0_backend_matrix import CAPABILITY_HEADER

    root = materialize(tmp_path)
    source = root / SOURCE_REL
    text = source.read_text(encoding="utf-8")
    needle = "The native interface SHALL identify"
    assert text.count(needle) == 1
    source.write_text(text.replace(needle, needle + "\\\\|", 1), encoding="utf-8")
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    result = run_script(GENERATOR, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert result.stdout == f"FAIL: table format changed: {CAPABILITY_HEADER}\n"
    assert "Traceback" not in result.stderr
    assert matrix.read_bytes() == before


def test_generator_blocks_when_source_missing(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    (root / SOURCE_REL).unlink()
    matrix = root / MATRIX_REL
    before = matrix.read_bytes()

    result = run_script(GENERATOR, root)

    assert result.returncode == 2
    assert result.stdout == f"blocked: missing {SOURCE_REL}\n"
    assert "Traceback" not in result.stderr
    assert matrix.read_bytes() == before


def test_generator_blocks_when_matrix_missing_on_check(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    matrix = root / MATRIX_REL
    matrix.unlink()

    result = run_script(GENERATOR, root, "--check")

    assert result.returncode == 2
    assert result.stdout == f"blocked: missing {MATRIX_REL}\n"
    assert "Traceback" not in result.stderr
    assert not matrix.exists()


def test_checker_blocks_when_input_missing(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    (root / TRUST_REL).unlink()

    result = run_script(CHECKER, root)

    assert result.returncode == 2
    assert result.stdout == f"blocked: missing {TRUST_REL}\n"
    assert "Traceback" not in result.stderr


def test_checker_rejects_related_id_supported_only_by_heading(tmp_path: Path) -> None:
    cases = (
        (
            "TP04",
            "docs/MORIARTY-BACKEND-REQUIREMENTS.md",
            "ZR14 — C, private continuation interface",
            "FAIL: TP04 related id ZR14 is not tied to a premise claim in a cited quote",
        ),
        (
            "TP03",
            "openspec/changes/partial-and-conditional-transactions/specs/"
            "partial-conditional-transactions/spec.md",
            "Requirement: MPLR-010 Time finality and unresolved outcomes",
            "FAIL: TP03 related id MPLR-010 is not tied to a premise claim in a cited quote",
        ),
    )
    for premise_id, path, quote, message in cases:
        root = materialize(tmp_path / premise_id)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = next(item for item in premises["premises"] if item["id"] == premise_id)
        premise["sourceRefs"] = [{"path": path, "quote": quote}]
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert message in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_noncanonical_trust_premises(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    trust_path.write_text(
        json.dumps(premises, indent=4, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: trust premises are not canonically serialised" in result.stdout
    assert "key order does not match the schema" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_trust_premise_key_order(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    premises["premises"][0] = {
        "status": premise["status"],
        "id": premise["id"],
        "statement": premise["statement"],
        "kind": premise["kind"],
        "sourceRefs": premise["sourceRefs"],
        "relatedIds": premise["relatedIds"],
    }
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: TP01 key order does not match the schema" in result.stdout
    assert "FAIL: trust premises are not canonically serialised" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_source_ref_key_order(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    ref = premises["premises"][0]["sourceRefs"][0]
    premises["premises"][0]["sourceRefs"][0] = {"quote": ref["quote"], "path": ref["path"]}
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: TP01 source ref key order does not match the schema" in result.stdout
    assert "FAIL: trust premises are not canonically serialised" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr
