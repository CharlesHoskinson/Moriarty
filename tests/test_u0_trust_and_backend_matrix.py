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
