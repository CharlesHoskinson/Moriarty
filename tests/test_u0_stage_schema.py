from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA = Path("openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json")
EMBEDDINGS = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
CHECKER = MORIARTY_ROOT / "scripts" / "check_u0_stage_schema.py"
LANGUAGE = "experiments/moriarty-language/src/successor/financial-lifecycle.ts"
GRAMMAR = "experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf"
FRONTEND = "experiments/moriarty-language/src/successor/frontend.ts"
OUTSIDE = "experiments/moriarty-developer-mock/src/model.ts"
SOURCE_PROFILE = "moriarty-financial-agreement-source/5"
CORE_PROFILE = "moriarty-financial-lifecycle/1"
LIMITATION = (
    "absence means not found by this recorded search; it is not a proof of non-realisation. "
    "Semantic adequacy of a cited declaration is a reviewed claim; the checker proves "
    "declaration, context, and profile membership only."
)


def run_checker(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(CHECKER), *args],
        cwd=MORIARTY_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def copy_root(tmp_path: Path) -> Path:
    root = tmp_path / "root"
    for rel in (SCHEMA, EMBEDDINGS, JUDGMENTS):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((MORIARTY_ROOT / rel).read_bytes())
    language = root / "experiments" / "moriarty-language"
    language.parent.mkdir(parents=True, exist_ok=True)
    language.symlink_to(MORIARTY_ROOT / "experiments" / "moriarty-language")
    return root


def write_json(path: Path, payload: object) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def load(root: Path, rel: Path) -> dict:
    return json.loads((root / rel).read_text(encoding="utf-8"))


def embedding_row(payload: dict, field: str) -> dict:
    return next(row for row in payload["rows"] if row["schemaField"] == field)


def test_real_artifacts_pass() -> None:
    process = run_checker()
    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    assert len(CHECKER.read_text(encoding="utf-8").splitlines()) <= 500
    embeddings = json.loads((MORIARTY_ROOT / EMBEDDINGS).read_text(encoding="utf-8"))
    rows = embeddings["rows"]
    present = sum(row["realisation"] == "present" for row in rows)
    partial = sum(row["realisation"] == "partial" for row in rows)
    absent = sum(row["realisation"] == "absent" for row in rows)
    assert (present, partial, absent) == (1, 17, 66)
    digest = hashlib.sha256((MORIARTY_ROOT / SCHEMA).read_bytes()).hexdigest()
    assert embeddings["stageSchemaSha256"] == digest
    assert process.stdout == (
        f"OK: 84 leaf fields, {present} present, {partial} partial, {absent} absent\n"
        f"{LIMITATION}\n"
    )
    assert all("searchTerms" not in row and "reviewedNonRealisations" not in row for row in rows)
    classes = {entry["context"]: entry["realisationClass"] for entry in embeddings["classificationTable"]}
    assert classes["TransferAction"] == "partial"
    assert classes["LifecycleObligation"] == "partial"
    assert classes["ProfileDecl"] == "present"
    for row in rows:
        assert row["present"] is (row["realisation"] != "absent")
        for context in (row["sourceContext"], row["coreContext"]):
            if context is not None:
                assert classes[context] == row["realisation"]
    account = embedding_row(embeddings, "effects.gross[].account")
    assert account["realisation"] == "partial"
    assert account["coreContext"] == "TransferAction"
    assert account["coreSymbol"] == "from"
    for field, symbol in (
        ("effects.gross[].asset", "asset"),
        ("effects.gross[].amount", "amount"),
        ("liabilities.opening[].asset", "settlementAsset"),
        ("liabilities.closing[].asset", "settlementAsset"),
        ("liabilities.opening[].amount", "outstanding"),
        ("liabilities.closing[].amount", "outstanding"),
        ("liabilities.opening[].debtor", "debtor"),
        ("liabilities.closing[].liabilityId", "id"),
    ):
        cited = embedding_row(embeddings, field)
        assert cited["realisation"] == "partial"
        assert cited["coreSymbol"] == symbol
        assert "Exists:" in cited["note"] and "; Missing:" in cited["note"]
    opening_amount = embedding_row(embeddings, "liabilities.opening[].amount")
    assert opening_amount["sourceContext"] is None
    assert opening_amount["sourceSymbol"] is None
    assert opening_amount["coreContext"] == "LifecycleObligation"
    assert opening_amount["coreSymbol"] == "outstanding"
    terminals = {"allowance_spent", "allowance_remaining", "outstanding"}
    assert all(row["sourceSymbol"] not in terminals for row in rows)
    assert "financialRead" not in classes
    profile = embedding_row(embeddings, "profiles.semanticProfile")
    assert profile["realisation"] == "present"
    assert profile["sourceContext"] == "ProfileDecl"
    assert profile["sourceSymbol"] == "value"


def test_c1_open_object_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    schema = load(root, SCHEMA)
    schema["additionalProperties"] = True
    del schema["properties"]["signedIntent"]
    schema["required"] = [key for key in schema["required"] if key != "signedIntent"]
    write_json(root / SCHEMA, schema)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: stageSchemaSha256 does not match the schema bytes" in process.stdout
    assert "FAIL: <root> additionalProperties is not false" in process.stdout
    assert "OK:" not in process.stdout


def test_c2_missing_row_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    payload["rows"] = [row for row in payload["rows"] if row["schemaField"] != "authority.consumed"]
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: embeddings missing leaf fields ['authority.consumed']" in process.stdout
    assert "OK:" not in process.stdout


def test_c3_partial_note_form_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    embedding_row(payload, "authority.consumed")["note"] = "Allowance.spent exists without the required form"
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed partial note is not 'Exists: ...; Missing: ...'" in process.stdout


def test_c4_cited_file_outside_roots_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path / "outside")
    mock = root / OUTSIDE
    mock.parent.mkdir(parents=True)
    mock.write_text("export interface Allowance {\n  spent: string;\n}\n", encoding="utf-8")
    payload = load(root, EMBEDDINGS)
    cited = embedding_row(payload, "authority.consumed")
    cited["coreFile"] = OUTSIDE
    payload["profileFiles"][CORE_PROFILE].append(OUTSIDE)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"FAIL: authority.consumed cited file {OUTSIDE} is outside the declared roots" in process.stdout
    assert f"FAIL: profileFiles entry {OUTSIDE} is outside the declared roots" in process.stdout
    assert "blocked:" not in process.stdout
    assert process.stderr == ""

    root = copy_root(tmp_path / "dotdot")
    escaped = "experiments/moriarty-language/src/successor/../successor/financial-lifecycle.ts"
    payload = load(root, EMBEDDINGS)
    cited = embedding_row(payload, "authority.consumed")
    cited["coreFile"] = escaped
    payload["profileFiles"][CORE_PROFILE].append(escaped)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"FAIL: authority.consumed cited file {escaped} is outside the declared roots" in process.stdout

    root = copy_root(tmp_path / "missing")
    payload = load(root, EMBEDDINGS)
    missing = "experiments/moriarty-language/src/successor/missing-lifecycle.ts"
    embedding_row(payload, "authority.consumed")["coreFile"] = missing
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 2
    assert process.stdout == f"blocked: missing {missing}\n"


def test_c5_ternary_branch_is_not_a_declaration(tmp_path: Path) -> None:
    root = copy_root(tmp_path / "terminal")
    payload = load(root, EMBEDDINGS)
    cited = embedding_row(payload, "authority.consumed")
    cited["sourceFile"] = GRAMMAR
    cited["sourceSymbol"] = "allowance_spent"
    cited["sourceContext"] = "financialRead"
    payload["classificationTable"].append({
        "context": "financialRead",
        "realisationClass": "partial",
        "rationale": "Probe: a financialRead terminal is not a production name.",
    })
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "FAIL: authority.consumed source symbol allowance_spent is not a declaration in "
        f"financialRead in {GRAMMAR}"
    ) in process.stdout

    root = copy_root(tmp_path / "kprobe")
    link = root / "experiments" / "moriarty-language"
    target = link.resolve()
    link.unlink()
    shutil.copytree(target, link, symlinks=True)
    k_rel = "experiments/moriarty-language/formal/k/u0-parser-probe.k"
    (root / k_rel).write_text(
        "module PROBE\n"
        "  syntax Exp ::= Int | Exp\n"
        "  syntax Exp ::=\n"
        "      bar(Int)\n"
        "    | \"lit\"\n"
        "endmodule\n",
        encoding="utf-8",
    )
    payload = load(root, EMBEDDINGS)
    cited = embedding_row(payload, "authority.consumed")
    cited["coreFile"] = k_rel
    cited["coreSymbol"] = "Int"
    cited["coreContext"] = "Exp"
    payload["profileFiles"][CORE_PROFILE].append(k_rel)
    payload["classificationTable"].append({
        "context": "Exp",
        "realisationClass": "partial",
        "rationale": "Probe: a bare sort reference is not a K constructor.",
    })
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        f"FAIL: authority.consumed core symbol Int is not a declaration in Exp in {k_rel}"
    ) in process.stdout
    cited["coreSymbol"] = "bar"
    write_json(root / EMBEDDINGS, payload)
    process = run_checker("--root", str(root))
    assert process.returncode == 0, process.stdout + process.stderr

    root = copy_root(tmp_path / "ternary")
    link = root / "experiments" / "moriarty-language"
    target = link.resolve()
    link.unlink()
    shutil.copytree(target, link, symlinks=True)
    source = root / LANGUAGE
    source.write_text(
        source.read_text(encoding="utf-8")
        + "\nexport function ternaryDemo(c: boolean): string {\n"
        + "  return c ? ghostBranch : otherBranch;\n"
        + "}\n",
        encoding="utf-8",
    )
    payload = load(root, EMBEDDINGS)
    cited = embedding_row(payload, "authority.consumed")
    cited["coreSymbol"] = "ghostBranch"
    cited["coreContext"] = "ternaryDemo"
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.consumed core symbol ghostBranch is not a declaration in "
        f"ternaryDemo in {LANGUAGE}"
    ) in process.stdout


def test_c6_unlisted_profile_file_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    listed = payload["profileFiles"][SOURCE_PROFILE]
    payload["profileFiles"][SOURCE_PROFILE] = [item for item in listed if item != FRONTEND]
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        f"FAIL: profiles.semanticProfile source file {FRONTEND} is not listed for {SOURCE_PROFILE}"
    ) in process.stdout


def test_c7_context_class_mismatch_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    for entry in payload["classificationTable"]:
        if entry["context"] == "TransferAction":
            entry["realisationClass"] = "present"
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: classificationTable context TransferAction is present "
        "but effects.gross[].account realisation is partial"
    ) in process.stdout


def test_c8_missing_root_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    missing = "experiments/moriarty-language/formal/missing-k"
    payload["absenceSearch"]["roots"] = [
        "experiments/moriarty-language/spec/successor",
        "experiments/moriarty-language/src/successor",
        missing,
    ]
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 2
    assert process.stdout == f"blocked: missing {missing}\n"


def test_c9_judgment_order_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path / "order")
    payload = load(root, JUDGMENTS)
    payload["judgments"][0], payload["judgments"][1] = payload["judgments"][1], payload["judgments"][0]
    write_json(root / JUDGMENTS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: judgments keys are ['intent', 'stage', 'effect', 'authority', 'history', 'failure']"
    ) in process.stdout

    root = copy_root(tmp_path / "type")
    payload = load(root, JUDGMENTS)
    payload["judgments"][0]["schemaFields"] = 42
    write_json(root / JUDGMENTS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: judgments.stage schemaFields is not a list" in process.stdout
    assert process.stderr == ""

    root = copy_root(tmp_path / "object")
    write_json(root / EMBEDDINGS, [1, 2, 3])

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert f"FAIL: {EMBEDDINGS.as_posix()} is not a JSON object" in process.stdout
    assert process.stderr == ""

    root = copy_root(tmp_path / "utf")
    (root / JUDGMENTS).write_bytes(b"\xff\xfe")

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert f"FAIL: {JUDGMENTS.as_posix()} is not UTF-8" in process.stdout
    assert "Traceback" not in process.stderr
    assert process.stderr == ""
