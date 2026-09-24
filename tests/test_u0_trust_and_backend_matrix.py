"""Checker and content tests for the U0 trust premises and backend matrix."""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
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
CLOSURE_WORDS = (
    "closed",
    "closes",
    "complete",
    "completed",
    "completes",
    "verified",
    "verifies",
    "proven",
    "proves",
    "proved",
    "resolved",
    "resolves",
    "guaranteed",
    "guarantees",
    "established",
    "establishes",
    "satisfied",
    "satisfies",
    "validated",
    "certified",
)


def git_env() -> dict[str, str]:
    env = os.environ.copy()
    env.pop("GIT_DIR", None)
    env.pop("GIT_WORK_TREE", None)
    return env


def git_add(root: Path, *paths: str) -> None:
    result = subprocess.run(
        ["git", "add", "--", *paths],
        cwd=root,
        env=git_env(),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr)


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
    init = subprocess.run(
        ["git", "init", "-q"],
        cwd=root,
        env=git_env(),
        capture_output=True,
        text=True,
        check=False,
    )
    if init.returncode != 0:
        raise RuntimeError(init.stderr)
    git_add(root, ".")
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
    assert checked.stdout == "OK: 24 backend rows specified-only, 9 trust premises\n"
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
    sentence = re.compile(r"^[^.!?;\n]+[.!?]$")

    assert premises["limitation"] == (
        "Semantic fit of a quote to its statement label is a reviewed claim, not "
        "mechanically proven. The checker verifies structure, literal quote occurrence, "
        "identifier resolution, required topics, label length, one sentence without "
        "', and' or a semicolon, and closure-word absence."
    )
    assert "reviewed claim" in premises["limitation"]
    assert "not mechanically proven" in premises["limitation"]
    assert list(by_id) == [f"TP{number:02d}" for number in range(1, 10)]
    assert [premise["topic"] for premise in premises["premises"]] == [
        "native-target",
        "recursion-horizon",
        "observations-finality",
        "federation-optional",
        "timeout-not-nonexecution",
        "private-handoff",
        "intent-auth-boundary",
        "witness-availability",
        "compatible-tuple",
    ]
    assert by_id["TP01"]["kind"] == "unresolved-interface"
    assert by_id["TP01"]["status"] == "open"
    assert by_id["TP01"]["statement"] == "The native execution target is Midnight ZKIRv3."
    assert by_id["TP09"]["topic"] == "compatible-tuple"
    assert by_id["TP09"]["kind"] == "unresolved-interface"
    assert by_id["TP09"]["status"] == "open"
    assert by_id["TP09"]["statement"] == "A comprehensive compatible release tuple is unknown."
    assert by_id["TP09"]["relatedIds"] == ["ZR01"]
    assert by_id["TP02"]["kind"] == "planning-assumption"
    assert by_id["TP02"]["status"] == "accepted-assumption"
    assert "March 2027" in by_id["TP02"]["statement"]
    assert by_id["TP02"]["statement"].count("independently") == 0
    assert by_id["TP03"]["kind"] == "trust-assumption"
    assert by_id["TP03"]["status"] == "open"
    assert "oracle honesty" in by_id["TP03"]["statement"]
    assert by_id["TP03"]["relatedIds"] == ["MPLR-010", "UNI-007", "UNI-008", "ZR03"]
    owner_quote = next(
        ref["quote"] for ref in by_id["TP03"]["sourceRefs"] if "UNI-007,008" in ref["quote"]
    )
    assert "UNI-008" not in owner_quote
    assert by_id["TP04"]["kind"] == "trust-assumption"
    assert by_id["TP04"]["status"] == "accepted-assumption"
    assert by_id["TP04"]["statement"] == (
        "Moriarty can also run on Midnight without this federation."
    )
    assert by_id["TP04"]["relatedIds"] == ["MPLR-030", "UNI-011", "ZR14"]
    assert all(
        "no federation requirement for direct Midnight use" not in ref["quote"]
        for ref in by_id["TP04"]["sourceRefs"]
    )
    assert any(
        ref["path"] == "docs/MORIARTY-CONSOLIDATED-DESIGN.md" for ref in by_id["TP04"]["sourceRefs"]
    )
    assert any(ref["path"].endswith("/proposal.md") for ref in by_id["TP04"]["sourceRefs"])
    assert by_id["TP05"]["kind"] == "trust-assumption"
    assert by_id["TP05"]["status"] == "accepted-assumption"
    assert by_id["TP05"]["statement"] == "Timeout is not evidence of nonexecution."
    assert by_id["TP05"]["statement"].count("Timeout is not evidence of nonexecution") == 1
    assert by_id["TP05"]["relatedIds"] == ["MPLR-005", "MPLR-010", "UNI-007", "UNI-008"]
    assert by_id["TP06"]["kind"] == "unresolved-interface"
    assert by_id["TP06"]["status"] == "open"
    assert by_id["TP06"]["statement"] == (
        "Native ZK is a foundation, not a private handoff theorem."
    )
    assert by_id["TP06"]["statement"].count("private handoff theorem") == 1
    assert by_id["TP08"]["topic"] == "witness-availability"
    assert by_id["TP08"]["kind"] == "unresolved-interface"
    assert by_id["TP08"]["status"] == "open"
    assert by_id["TP08"]["statement"] == (
        "Witness availability is an explicit liveness assumption."
    )
    assert by_id["TP08"]["relatedIds"] == ["ZR14"]
    assert by_id["TP07"]["kind"] == "unresolved-interface"
    assert by_id["TP07"]["status"] == "open"
    assert "circuit" in by_id["TP07"]["statement"]
    assert "do not make this choice" not in by_id["TP07"]["statement"]
    for premise in premises["premises"]:
        assert premise["status"] in {"open", "accepted-assumption"}
        assert len(premise["statement"]) <= 160
        assert ", and " not in premise["statement"]
        assert ";" not in premise["statement"]
        assert sentence.fullmatch(premise["statement"])
        for word in CLOSURE_WORDS:
            assert re.search(rf"\b{word}\b", premise["statement"], re.IGNORECASE) is None
        assert premise["relatedIds"] == sorted(premise["relatedIds"])
        assert len(premise["relatedIds"]) == len(set(premise["relatedIds"]))
        assert any(ref["quoteRole"] == "states-premise" for ref in premise["sourceRefs"])
        for ref in premise["sourceRefs"]:
            assert ref["quoteRole"] in {"states-premise", "states-limitation", "states-owner"}
            assert list(ref) == ["path", "quoteRole", "quote"]
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
    assert f"FAIL: {MATRIX_REL} rows:" in result.stdout
    assert "is too short" in result.stdout
    assert "FAIL: backend matrix drifted" not in result.stdout
    assert "OK:" not in result.stdout


def test_checker_rejects_matrix_id_outside_expected_set(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    matrix_path = root / MATRIX_REL
    matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
    assert matrix["rows"][0]["id"] == "ZR01"
    matrix["rows"][0]["id"] = "ZR99"
    write_json(matrix_path, matrix)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "Traceback" not in result.stderr
    assert "FAIL: backend matrix drifted" in result.stdout
    assert "FAIL: backend row ids are not exactly ZR01-ZR16 and MNR01-MNR08" in result.stdout
    assert "OK:" not in result.stdout


def test_checker_rejects_forbidden_status(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    matrix_path = root / MATRIX_REL
    matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
    matrix["rows"][0]["status"] = "executed"
    write_json(matrix_path, matrix)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert "Traceback" not in result.stderr
    assert f"FAIL: {MATRIX_REL} rows/0/status:" in result.stdout
    assert "'specified-only' was expected" in result.stdout
    assert "FAIL: ZR01 status is not specified-only" not in result.stdout
    assert "OK:" not in result.stdout


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
    assert f"FAIL: {TRUST_REL} premises:" in result.stdout
    assert "is too short" in result.stdout
    assert "FAIL: missing required topic" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_false_premise_claim(tmp_path: Path) -> None:
    cases = (
        (
            "TP01",
            "ZKIRv3 recursion is released and closed.",
            "FAIL: TP01 statement contains closure word: closed",
        ),
        (
            "TP02",
            "UNI-009 is an independently verified release.",
            "FAIL: TP02 statement contains closure word: verified",
        ),
        (
            "TP01",
            "This tuple is not verified.",
            "FAIL: TP01 statement contains closure word: verified",
        ),
        (
            "TP01",
            "The tuple is completed and proves soundness.",
            "FAIL: TP01 statement contains closure word: completed",
        ),
    )
    for premise_id, statement, message in cases:
        root = materialize(tmp_path / premise_id / statement[:12])
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = next(item for item in premises["premises"] if item["id"] == premise_id)
        premise["statement"] = statement
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert message in result.stdout
        assert "not source-backed" not in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_each_closure_word(tmp_path: Path) -> None:
    for word in CLOSURE_WORDS:
        root = materialize(tmp_path / word)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premises["premises"][0]["statement"] = f"This label is {word}."
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert f"FAIL: TP01 statement contains closure word: {word}" in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_statement_over_160_characters(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    statement = "A" * 160 + "."
    assert len(statement) == 161
    premises["premises"][0]["statement"] = statement
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert f"FAIL: {TRUST_REL} premises/0/statement:" in result.stdout
    assert "is too long" in result.stdout
    assert "FAIL: TP01 statement exceeds 160 characters" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_premise_without_states_premise_quote(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    assert any(ref["quoteRole"] == "states-premise" for ref in premise["sourceRefs"])
    for ref in premise["sourceRefs"]:
        ref["quoteRole"] = "states-limitation"
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: TP01 has no states-premise quote" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_missing_required_topic(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = next(item for item in premises["premises"] if item["id"] == "TP05")
    assert premise["topic"] == "timeout-not-nonexecution"
    premise["topic"] = "timeout-note"
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: missing required topic timeout-not-nonexecution" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_spliced_unsupported_claim(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = premises["premises"][0]
    premise["statement"] += (
        " ZR01 is fully implemented and certified; "
        "a comprehensive compatible release tuple is unknown. "
        "Pinned tuple evidence is absent from this label."
    )
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert f"FAIL: {TRUST_REL} premises/0/statement:" in result.stdout
    assert "does not match" in result.stdout
    assert "FAIL: TP01 statement is spliced" not in result.stdout
    assert "not source-backed" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_unsupported_premise_status(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    assert premises["premises"][0]["status"] == "open"
    premises["premises"][0]["status"] = "closed"
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1
    assert f"FAIL: {TRUST_REL} premises/0/status:" in result.stdout
    assert "is not one of" in result.stdout
    assert "FAIL: TP01 status is not open or accepted-assumption" not in result.stdout
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
            "FAIL: TP04 related id ZR14 is not tied to a non-heading span in a cited quote",
        ),
        (
            "TP03",
            "openspec/changes/partial-and-conditional-transactions/specs/"
            "partial-conditional-transactions/spec.md",
            "Requirement: MPLR-010 Time finality and unresolved outcomes",
            "FAIL: TP03 related id MPLR-010 is not tied to a non-heading span in a cited quote",
        ),
    )
    for premise_id, path, quote, message in cases:
        root = materialize(tmp_path / premise_id)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = next(item for item in premises["premises"] if item["id"] == premise_id)
        premise["sourceRefs"] = [
            {"path": path, "quoteRole": "states-premise", "quote": quote}
        ]
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
        "topic": premise["topic"],
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


def test_checker_rejects_missing_cited_file(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    relative = "docs/MORIARTY-PRODUCT-CONTRACT.md"
    cited = root / relative
    assert cited.is_file()
    cited.unlink()
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = next(item for item in premises["premises"] if item["id"] == "TP02")
    assert premise["status"] == "accepted-assumption"
    premise["statement"] = "This planning note is verified."
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert f"FAIL: TP01 cited file missing: {relative}" in result.stdout
    assert "FAIL: TP02 statement contains closure word: verified" in result.stdout
    assert f"blocked: missing {relative}" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_missing_requirement_catalog(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    for premise in premises["premises"]:
        premise["sourceRefs"] = [
            ref for ref in premise["sourceRefs"] if not ref["path"].startswith("openspec/")
        ]
        assert premise["sourceRefs"]
    closed = next(item for item in premises["premises"] if item["id"] == "TP02")
    closed["statement"] = "This planning note is verified."
    write_json(trust_path, premises)
    changes = root / "openspec" / "changes"
    catalog_files = [
        *changes.glob("*/specs/**/spec.md"),
        *changes.glob("*/traceability.md"),
    ]
    assert catalog_files
    for path in catalog_files:
        path.unlink()

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert (
        "FAIL: requirement catalog missing: openspec/changes/*/specs/**/spec.md"
        in result.stdout
    )
    assert (
        "FAIL: requirement catalog missing: openspec/changes/*/traceability.md"
        in result.stdout
    )
    assert "FAIL: TP02 statement contains closure word: verified" in result.stdout
    assert "blocked:" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_accepts_extra_source_backed_premise(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    quote = (
        "ZR03 — C, complete statement binding | Recursive verification SHALL bind "
        "canonical public inputs or binding commitments for program/semantics, property, "
        "signed intent"
    )
    assert quote in (root / SOURCE_REL).read_text(encoding="utf-8")
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"].append(
        {
            "id": "TP10",
            "topic": "signed-intent-note",
            "statement": "Signed intent stays bound to canonical public inputs.",
            "kind": "unresolved-interface",
            "sourceRefs": [
                {"path": SOURCE_REL, "quoteRole": "states-premise", "quote": quote}
            ],
            "relatedIds": ["ZR03"],
            "status": "open",
        }
    )
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 0, result.stdout + result.stderr
    assert result.stdout == "OK: 24 backend rows specified-only, 10 trust premises\n"


def test_checker_rejects_related_id_tied_only_by_broad_topic(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    weak_rel = "docs/zr14-broad-topic.md"
    weak_quote = "ZR14 \u2014 selection of the federated kernel"
    (root / weak_rel).write_text(weak_quote + "\n", encoding="utf-8")
    git_add(root, weak_rel)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premise = next(item for item in premises["premises"] if item["id"] == "TP04")
    replaced = False
    for ref in premise["sourceRefs"]:
        if "ZR14" in ref["quote"] and "without the federated kernel" in ref["quote"]:
            ref["path"] = weak_rel
            ref["quoteRole"] = "states-limitation"
            ref["quote"] = weak_quote
            replaced = True
    assert replaced
    premise["sourceRefs"].append(
        {
            "path": "docs/MORIARTY-CONSOLIDATED-DESIGN.md",
            "quoteRole": "states-premise",
            "quote": (
                "MC06 private handoff and split/join must be demonstrable between "
                "independently controlled participants without the federated kernel."
            ),
        }
    )
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert (
        "FAIL: TP04 related id ZR14 is not tied to a non-heading span in a cited quote"
        in result.stdout
    )
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_imports_when_scripts_directory_is_not_on_path(tmp_path: Path) -> None:
    probe = (
        "import importlib.util\n"
        "from pathlib import Path\n"
        f"path = Path({str(CHECKER)!r})\n"
        "spec = importlib.util.spec_from_file_location('u0_checker_probe', path)\n"
        "module = importlib.util.module_from_spec(spec)\n"
        "assert spec.loader is not None\n"
        "spec.loader.exec_module(module)\n"
        "assert module.build_matrix.__name__ == 'build_matrix'\n"
    )
    result = subprocess.run(
        [sys.executable, "-c", probe],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        check=False,
    )

    assert result.returncode == 0, result.stdout + result.stderr


def test_checker_runs_as_package_module() -> None:
    result = subprocess.run(
        [sys.executable, "-m", "scripts.check_u0_trust_backend_matrix", "--root", str(ROOT)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )

    assert result.returncode == 0, result.stdout + result.stderr
    assert result.stdout == "OK: 24 backend rows specified-only, 9 trust premises\n"
    assert "Traceback" not in result.stderr


def test_checker_rejects_source_ref_key_order(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    ref = premises["premises"][0]["sourceRefs"][0]
    premises["premises"][0]["sourceRefs"][0] = {
        "quote": ref["quote"],
        "path": ref["path"],
        "quoteRole": ref["quoteRole"],
    }
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: TP01 source ref key order does not match the schema" in result.stdout
    assert "FAIL: trust premises are not canonically serialised" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_statement_joined_with_comma_and(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    statement = "The native target is Midnight ZKIRv3, and the release tuple is unknown."
    assert ", and " in statement
    assert len(statement) <= 160
    premises["premises"][0]["statement"] = statement
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: TP01 statement joins claims with ', and'" in result.stdout
    assert "FAIL: TP01 statement is spliced" not in result.stdout
    assert "FAIL: TP01 statement is not one sentence" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_self_cited_and_forbidden_sources(tmp_path: Path) -> None:
    script_rel = "scripts/check_u0_trust_backend_matrix.py"
    test_rel = "tests/test_u0_trust_and_backend_matrix.py"
    cases = (
        (TRUST_REL, None),
        (TRUST_SCHEMA_REL, "moriarty-u0-trust-premises/1"),
        (script_rel, "Validate U0 trust premises and the generated backend requirement matrix."),
        (test_rel, "Checker and content tests for the U0 trust premises and backend matrix."),
    )
    for relative, quote in cases:
        root = materialize(tmp_path / relative.replace("/", "-"))
        if quote is not None and not (root / relative).is_file():
            target = root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / relative).read_bytes())
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = premises["premises"][0]
        used_quote = premise["statement"] if quote is None else quote
        assert len(used_quote) >= 20
        source = root / relative
        assert source.is_file()
        assert used_quote in source.read_text(encoding="utf-8")
        premise["sourceRefs"].append(
            {"path": relative, "quoteRole": "states-premise", "quote": used_quote}
        )
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert (
            f"FAIL: TP01 source ref path is not an allowed document: {relative}"
            in result.stdout
        )
        assert f"quote not in {relative}" not in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_wrong_typed_source_path(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0]["sourceRefs"][0]["path"] = 12
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert (
        f"FAIL: {TRUST_REL} premises/0/sourceRefs/0/path: 12 is not of type 'string'"
        in result.stdout
    )
    assert "quote not in" not in result.stdout
    assert "source ref is incomplete" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_untracked_source(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    quote = "repositoryformatversion = 0"
    assert quote in (root / ".git" / "config").read_text(encoding="utf-8")
    report_rel = "FOREMAN_REPORT.md"
    report_quote = "This untracked report is not a repository document."
    (root / report_rel).write_text(report_quote + "\n", encoding="utf-8")
    cases = (
        (".git/config", quote),
        (report_rel, report_quote),
    )
    for relative, used_quote in cases:
        trust_path = root / TRUST_REL
        premises = json.loads((ROOT / TRUST_REL).read_text(encoding="utf-8"))
        premise = premises["premises"][0]
        premise["sourceRefs"] = [
            {"path": relative, "quoteRole": "states-premise", "quote": used_quote}
        ]
        premises["premises"][1]["statement"] = "This planning note is verified."
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert (
            "FAIL: TP01 source ref path is not a tracked repository document: "
            f"{relative}" in result.stdout
        )
        assert "FAIL: TP02 statement contains closure word: verified" in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_symlink_into_forbidden_tree(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    quote = "Validate U0 trust premises and the generated backend requirement matrix."
    target_rel = "scripts/check_u0_trust_backend_matrix.py"
    target = root / target_rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(quote + "\n", encoding="utf-8")
    link_rel = "docs/alias-to-checker.md"
    (root / link_rel).symlink_to(Path("..") / "scripts" / "check_u0_trust_backend_matrix.py")
    git_add(root, target_rel, link_rel)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0]["sourceRefs"] = [
        {"path": link_rel, "quoteRole": "states-premise", "quote": quote}
    ]
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert (
        f"FAIL: TP01 source ref path is not an allowed document: {link_rel}"
        in result.stdout
    )
    assert f"quote not in {link_rel}" not in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_symlink_that_escapes_root(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    quote = "This outside file is not inside the repository."
    outside = tmp_path / "outside.md"
    outside.write_text(quote + "\n", encoding="utf-8")
    link_rel = "docs/outside-link.md"
    (root / link_rel).symlink_to(outside)
    git_add(root, link_rel)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0]["sourceRefs"] = [
        {"path": link_rel, "quoteRole": "states-premise", "quote": quote}
    ]
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert f"FAIL: TP01 quote path escapes root: {link_rel}" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_blocks_when_git_metadata_missing(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    shutil.rmtree(root / ".git")

    result = run_script(CHECKER, root)

    assert result.returncode == 2, result.stdout + result.stderr
    assert result.stdout == "blocked: git metadata unavailable\n"
    assert "Traceback" not in result.stderr


def test_checker_accepts_neutral_words_outside_closure_list(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    statement = "The plan will provide a closing completeness note for a provable bound."
    assert len(statement) <= 160
    assert ", and " not in statement
    assert ";" not in statement
    for word in CLOSURE_WORDS:
        assert re.search(rf"\b{word}\b", statement, re.IGNORECASE) is None
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0]["statement"] = statement
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 0, result.stdout + result.stderr
    assert result.stdout == "OK: 24 backend rows specified-only, 9 trust premises\n"


def test_main_reports_internal_error(monkeypatch, capsys) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    import check_u0_trust_backend_matrix as checker

    def boom(root: Path) -> tuple[int, list[str]]:
        raise RuntimeError("boom")

    monkeypatch.setattr(checker, "check", boom)
    code = checker.main(["--root", str(ROOT)])
    captured = capsys.readouterr()

    assert code == 1
    assert captured.out == "FAIL: internal error: RuntimeError: boom\n"
    assert captured.err == ""


def test_generator_main_reports_internal_error(monkeypatch, capsys) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    import build_u0_backend_matrix as generator

    def boom(source: bytes) -> dict[str, object]:
        raise RuntimeError("boom")

    monkeypatch.setattr(generator, "build_matrix", boom)
    code = generator.main(["--root", str(ROOT), "--check"])
    captured = capsys.readouterr()

    assert code == 1
    assert captured.out == "FAIL: internal error: RuntimeError: boom\n"
    assert captured.err == ""


def invoke(script: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(script), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def test_checker_rejects_invalid_arguments() -> None:
    cases = (
        (
            ["--definitely-unsupported"],
            "FAIL: invalid arguments: unrecognized arguments: --definitely-unsupported\n",
        ),
        (
            ["--root"],
            "FAIL: invalid arguments: argument --root: expected one argument\n",
        ),
    )
    for args, stdout in cases:
        result = invoke(CHECKER, *args)

        assert result.returncode == 1, result.stdout + result.stderr
        assert result.stdout == stdout
        assert result.stderr == ""
        assert "Traceback" not in result.stderr


def test_generator_rejects_invalid_arguments() -> None:
    cases = (
        (
            ["--definitely-unsupported"],
            "FAIL: invalid arguments: unrecognized arguments: --definitely-unsupported\n",
        ),
        (
            ["--check=nope"],
            "FAIL: invalid arguments: argument --check: ignored explicit argument 'nope'\n",
        ),
    )
    for args, stdout in cases:
        result = invoke(GENERATOR, *args)

        assert result.returncode == 1, result.stdout + result.stderr
        assert result.stdout == stdout
        assert result.stderr == ""
        assert "Traceback" not in result.stderr


def test_checker_rejects_swapped_premises(tmp_path: Path) -> None:
    root = materialize(tmp_path)
    trust_path = root / TRUST_REL
    premises = json.loads(trust_path.read_text(encoding="utf-8"))
    premises["premises"][0], premises["premises"][1] = (
        premises["premises"][1],
        premises["premises"][0],
    )
    write_json(trust_path, premises)

    result = run_script(CHECKER, root)

    assert result.returncode == 1, result.stdout + result.stderr
    assert "FAIL: trust premises are not contiguous TP01..TP09" in result.stdout
    assert "OK:" not in result.stdout
    assert "Traceback" not in result.stderr


def test_checker_rejects_unsorted_and_duplicate_related_ids(tmp_path: Path) -> None:
    cases = (
        (
            "unsorted",
            lambda ids: list(reversed(ids)),
            "FAIL: TP03 relatedIds are not sorted",
        ),
        (
            "duplicate",
            lambda ids: sorted([*ids, ids[0]]),
            "FAIL: TP03 relatedIds contain a duplicate",
        ),
    )
    for name, mutate, message in cases:
        root = materialize(tmp_path / name)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = next(item for item in premises["premises"] if item["id"] == "TP03")
        assert premise["relatedIds"] == sorted(set(premise["relatedIds"]))
        premise["relatedIds"] = mutate(premise["relatedIds"])
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert message in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_checker_rejects_related_id_omitted_from_cited_quote(tmp_path: Path) -> None:
    cases = (
        (
            "TP03",
            "UNI-008",
            "FAIL: TP03 related id UNI-008 appears in a cited quote "
            "but is missing from relatedIds",
        ),
        (
            "TP05",
            "MPLR-005",
            "FAIL: TP05 related id MPLR-005 appears in a cited quote "
            "but is missing from relatedIds",
        ),
    )
    for premise_id, omitted, message in cases:
        root = materialize(tmp_path / premise_id)
        trust_path = root / TRUST_REL
        premises = json.loads(trust_path.read_text(encoding="utf-8"))
        premise = next(item for item in premises["premises"] if item["id"] == premise_id)
        assert omitted in premise["relatedIds"]
        premise["relatedIds"] = [item for item in premise["relatedIds"] if item != omitted]
        assert premise["relatedIds"] == sorted(premise["relatedIds"])
        write_json(trust_path, premises)

        result = run_script(CHECKER, root)

        assert result.returncode == 1, result.stdout + result.stderr
        assert message in result.stdout
        assert "OK:" not in result.stdout
        assert "Traceback" not in result.stderr


def test_git_ls_files_keeps_ownership_check(monkeypatch) -> None:
    sys.path.insert(0, str(ROOT / "scripts"))
    import check_u0_trust_backend_matrix as checker

    commands: list[list[str]] = []
    real_run = subprocess.run

    def record(command, **kwargs):
        if isinstance(command, list):
            commands.append([str(part) for part in command])
        return real_run(command, **kwargs)

    monkeypatch.setattr(checker.subprocess, "run", record)
    tracked, blocked = checker.load_tracked_paths(ROOT)

    assert blocked is None
    assert tracked is not None
    assert TRUST_REL in tracked
    assert commands == [["git", "-C", str(ROOT), "ls-files", "-z"]]
    assert "safe.directory=*" not in commands[0]


def test_git_refusal_blocks_metadata(monkeypatch, tmp_path: Path) -> None:
    root = materialize(tmp_path)
    sys.path.insert(0, str(ROOT / "scripts"))
    import check_u0_trust_backend_matrix as checker

    def refuse(command, **kwargs):
        return subprocess.CompletedProcess(
            command,
            128,
            b"",
            b"fatal: detected dubious ownership in repository",
        )

    monkeypatch.setattr(checker.subprocess, "run", refuse)
    code, lines = checker.check(root)

    assert code == 2
    assert lines == ["blocked: git metadata unavailable"]
