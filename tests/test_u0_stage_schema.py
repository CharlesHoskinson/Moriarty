from __future__ import annotations

import json
import shutil
import subprocess
import sys
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
EMBEDDINGS = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
DESIGN_DOC = Path("docs/MORIARTY-CONSOLIDATED-DESIGN.md")
CHECKER = MORIARTY_ROOT / "scripts" / "check_u0_stage_schema.py"
LANGUAGE = Path("experiments/moriarty-language/src/successor/financial-lifecycle.ts")
FRONTEND = Path("experiments/moriarty-language/src/successor/frontend.ts")


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
    for rel in (SCHEMA, EMBEDDINGS, JUDGMENTS, DESIGN_DOC):
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


LIMITATION = (
    "absence means not found by this recorded search; it is not a proof of non-realisation. "
    "Semantic adequacy of a cited declaration is a reviewed claim; the checker proves "
    "declaration, context, and profile membership only."
)
CLASSIFICATION_RULE = (
    "present = the cited declaration(s) realise the field's full meaning in the named context; "
    "partial = a declaration in the declared source/Core profiles realises a named aspect "
    "(note: `Exists: ...; Missing: ...`); absent = no declaration found"
)


def embedding_row(payload: dict, field: str) -> dict:
    return next(row for row in payload["rows"] if row["schemaField"] == field)


def replace_row(payload: dict, field: str, row: dict) -> None:
    payload["rows"] = [
        row if item["schemaField"] == field else item for item in payload["rows"]
    ]


def citation_row(row: dict, realisation: str) -> dict:
    return {
        "schemaField": row["schemaField"],
        "sourceFile": None,
        "sourceSymbol": None,
        "sourceContext": None,
        "coreFile": None,
        "coreSymbol": None,
        "coreContext": None,
        "realisation": realisation,
        "present": realisation != "absent",
        "note": row["note"],
    }


def absent_row(row: dict, terms: list[str], note: str) -> dict:
    return {
        "schemaField": row["schemaField"],
        "sourceFile": None,
        "sourceSymbol": None,
        "sourceContext": None,
        "coreFile": None,
        "coreSymbol": None,
        "coreContext": None,
        "realisation": "absent",
        "present": False,
        "searchTerms": terms,
        "reviewedNonRealisations": [],
        "note": note,
    }


def materialize_language(root: Path) -> None:
    link = root / "experiments" / "moriarty-language"
    if not link.is_symlink():
        return
    target = link.resolve()
    link.unlink()
    shutil.copytree(target, link, symlinks=True)


def test_real_artifacts_pass() -> None:
    process = run_checker()
    assert process.returncode == 0, process.stdout + process.stderr
    embeddings = json.loads((MORIARTY_ROOT / EMBEDDINGS).read_text(encoding="utf-8"))
    rows = embeddings["rows"]
    full = sum(row["realisation"] == "present" for row in rows)
    partial = sum(row["realisation"] == "partial" for row in rows)
    absent = sum(row["realisation"] == "absent" for row in rows)
    reviewed = sum(len(row.get("reviewedNonRealisations") or []) for row in rows)
    assert (full, partial, absent) == (1, 12, 71)
    assert process.stdout == (
        f"OK: 84 leaf fields, {full + partial} present, {absent} absent\n"
        f"partial: {partial}\n"
        f"reviewed non-realisations: {reviewed}\n"
        f"{LIMITATION}\n"
    )
    assert embeddings["classificationRule"] == CLASSIFICATION_RULE
    assert embeddings["absenceSearch"]["limitation"] == LIMITATION
    consumed = embedding_row(embeddings, "authority.consumed")
    assert consumed["realisation"] == "partial"
    assert consumed["present"] is True
    assert consumed["sourceSymbol"] == "allowance_spent"
    assert consumed["sourceContext"] == "financialRead"
    assert consumed["coreSymbol"] == "spent"
    assert consumed["coreContext"] == "Allowance"
    assert "Exists:" in consumed["note"]
    assert "Missing:" in consumed["note"]
    remaining = embedding_row(embeddings, "authority.remaining")
    assert remaining["coreSymbol"] == "remaining"
    assert remaining["coreContext"] == "Allowance"
    replay = embedding_row(embeddings, "authority.replayState")
    assert replay["coreSymbol"] == "usedTransferIds"
    assert replay["coreContext"] == "LifecycleState"
    profile = embedding_row(embeddings, "profiles.semanticProfile")
    assert profile["realisation"] == "present"
    assert profile["sourceSymbol"] == "value"
    assert profile["sourceContext"] == "ProfileDecl"
    debtor = embedding_row(embeddings, "liabilities.opening[].debtor")
    assert debtor["realisation"] == "partial"
    assert debtor["coreSymbol"] == "debtor"
    assert debtor["coreContext"] == "LifecycleObligation"
    program = embedding_row(embeddings, "programIdentity.programId")
    assert program["realisation"] == "absent"
    assert program["present"] is False
    assert program["sourceSymbol"] is None
    assert "programId" in program["searchTerms"]
    assert "program" in program["searchTerms"]
    version = embedding_row(embeddings, "schemaVersion")
    assert version["realisation"] == "absent"
    assert version["present"] is False
    assert version["coreFile"] is None
    assert "schemaVersion" in version["searchTerms"]
    assert any(
        item["symbol"] == "schemaVersion" and item["file"].endswith("/core.ts")
        for item in version["reviewedNonRealisations"]
    )
    gross_amount = embedding_row(embeddings, "effects.gross[].amount")
    assert gross_amount["realisation"] == "partial"
    assert gross_amount["present"] is True
    assert gross_amount["coreSymbol"] == "amount"
    assert gross_amount["coreContext"] == "TransferAction"
    gross_asset = embedding_row(embeddings, "effects.gross[].asset")
    assert gross_asset["realisation"] == "partial"
    assert gross_asset["coreContext"] == "TransferAction"
    liability = embedding_row(embeddings, "liabilities.opening[].liabilityId")
    assert liability["realisation"] == "partial"
    assert liability["coreSymbol"] == "id"
    assert liability["coreContext"] == "LifecycleObligation"
    closing = embedding_row(embeddings, "liabilities.closing[].liabilityId")
    assert closing["realisation"] == "partial"
    assert closing["coreContext"] == "LifecycleObligation"
    chain = embedding_row(embeddings, "domain.chainId")
    assert "chainId" in chain["searchTerms"]
    assert any(
        item["symbol"] == "chainId" and item["file"].endswith("/build-lifecycle.mjs")
        for item in chain["reviewedNonRealisations"]
    )
    assert "K constructor lcKernelRejected" in embedding_row(
        embeddings, "failurePolicy.phasePolicy"
    )["note"]
    assert process.stderr == ""


def test_deleted_embedding_row_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    payload["rows"] = [
        row for row in payload["rows"] if row["schemaField"] != "authority.consumed"
    ]
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: embeddings missing leaf fields ['authority.consumed']" in process.stdout
    assert "OK:" not in process.stdout


def test_citation_of_missing_symbol_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["coreSymbol"] = "NOT_A_LIFECYCLE_SYMBOL"
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed core symbol NOT_A_LIFECYCLE_SYMBOL does not occur" in process.stdout


def test_partial_identifier_citation_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.remaining")
    target["coreSymbol"] = "remain"
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.remaining core symbol remain does not occur as a declaration in "
        "Allowance in experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    ) in process.stdout


def test_whitespace_symbol_citation_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["coreSymbol"] = " "
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed core symbol ' ' is not an identifier or K cell" in process.stdout


def test_prose_symbol_citation_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["coreSymbol"] = "not signing"
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.consumed core symbol 'not signing' is not an identifier or K cell"
    ) in process.stdout


def test_duplicate_embedding_row_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    index = payload["rows"].index(target)
    payload["rows"].insert(index, dict(target))
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: duplicate embedding rows for ['authority.consumed']" in process.stdout


def test_absent_row_with_source_file_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "circuitIdentity.circuitId")
    target["sourceFile"] = LANGUAGE.as_posix()
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: circuitIdentity.circuitId is absent but sourceFile or sourceSymbol is set"
    ) in process.stdout


def test_judgment_schema_field_must_be_leaf(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / JUDGMENTS
    payload = load(root, JUDGMENTS)
    effect = next(row for row in payload["judgments"] if row["key"] == "effect")
    effect["schemaFields"].append("effects.gross")
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: judgments.effect schemaFields entry 'effects.gross' is not a leaf" in process.stdout


def test_forbidden_judgment_status_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / SCHEMA
    payload = load(root, SCHEMA)
    status = payload["properties"]["judgments"]["properties"]["stage"]["properties"]["status"]
    status["enum"] = ["held", "green", "unchecked"]
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: judgments.stage.status enum is ['held', 'green', 'unchecked']" in process.stdout


def test_removed_mandatory_field_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    schema = load(root, SCHEMA)
    circuit = schema["properties"]["circuitIdentity"]
    del circuit["properties"]["verifierKeyId"]
    circuit["required"] = [key for key in circuit["required"] if key != "verifierKeyId"]
    write_json(root / SCHEMA, schema)

    embeddings = load(root, EMBEDDINGS)
    embeddings["rows"] = [
        row for row in embeddings["rows"] if row["schemaField"] != "circuitIdentity.verifierKeyId"
    ]
    write_json(root / EMBEDDINGS, embeddings)

    judgments = load(root, JUDGMENTS)
    for row in judgments["judgments"]:
        row["schemaFields"] = [
            field for field in row["schemaFields"] if field != "circuitIdentity.verifierKeyId"
        ]
    write_json(root / JUDGMENTS, judgments)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: missing required property circuitIdentity.verifierKeyId" in process.stdout
    assert "OK:" not in process.stdout


def test_profile_version_constant_is_not_program_identity(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    core = citation_row(embedding_row(payload, "programIdentity.coreRef"), "present")
    core["coreFile"] = LANGUAGE.as_posix()
    core["coreSymbol"] = "LIFECYCLE_VERSION"
    core["coreContext"] = "LIFECYCLE_VERSION"
    core["note"] = "LIFECYCLE_VERSION is a profile version."
    source = citation_row(embedding_row(payload, "programIdentity.sourceRef"), "present")
    source["sourceFile"] = FRONTEND.as_posix()
    source["sourceSymbol"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE"
    source["sourceContext"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE"
    source["note"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE is a profile version."
    replace_row(payload, "programIdentity.coreRef", core)
    replace_row(payload, "programIdentity.sourceRef", source)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: programIdentity.coreRef core symbol LIFECYCLE_VERSION "
        "is a profile version constant, not a program identity"
    ) in process.stdout
    assert (
        "FAIL: programIdentity.sourceRef source symbol FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE "
        "is a profile version constant, not a program identity"
    ) in process.stdout


def test_disclosure_fields_must_be_a_list(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    schema = load(root, SCHEMA)
    schema["properties"]["disclosures"]["items"]["properties"]["fields"] = {"type": "string"}
    write_json(root / SCHEMA, schema)

    embeddings = load(root, EMBEDDINGS)
    row = embedding_row(embeddings, "disclosures[].fields[]")
    row["schemaField"] = "disclosures[].fields"
    write_json(root / EMBEDDINGS, embeddings)

    judgments = load(root, JUDGMENTS)
    for judgment in judgments["judgments"]:
        judgment["schemaFields"] = [
            "disclosures[].fields" if field == "disclosures[].fields[]" else field
            for field in judgment["schemaFields"]
        ]
    write_json(root / JUDGMENTS, judgments)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: disclosures[].fields type is 'string'" in process.stdout
    assert "OK:" not in process.stdout


def test_missing_language_tree_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    (root / "experiments" / "moriarty-language").unlink()

    process = run_checker("--root", str(root))

    assert process.returncode == 2
    assert process.stdout == "blocked: missing experiments/moriarty-language\n"


def test_missing_design_doc_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    (root / DESIGN_DOC).unlink()

    process = run_checker("--root", str(root))

    assert process.returncode == 2
    assert process.stdout == "blocked: missing docs/MORIARTY-CONSOLIDATED-DESIGN.md\n"


def test_design_doc_phrase_must_be_in_canonical_section(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / DESIGN_DOC
    text = path.read_text(encoding="utf-8")
    start = text.index("## Canonical stage statement")
    end = text.index("## Assets, authority, obligations and arithmetic")
    section = text[start:end].replace("compliant history", "recorded history", 1)
    path.write_text(text[:start] + section + text[end:], encoding="utf-8")

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: Canonical stage statement lacks 'compliant history'" in process.stdout


def test_design_doc_failure_phrase_must_occur(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / DESIGN_DOC
    text = path.read_text(encoding="utf-8")
    updated = text.replace("Rejection/partial failure", "Refusal outcome", 1)
    assert updated != text
    path.write_text(updated, encoding="utf-8")

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: Canonical stage statement lacks 'Rejection/partial failure'" in process.stdout


def test_work_spent_does_not_fill_authority(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["coreContext"] = "Balance"
    target["coreSymbol"] = "spent"
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.consumed core symbol spent does not occur as a declaration in "
        "Balance in experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    ) in process.stdout


def test_comment_only_citation_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = citation_row(embedding_row(payload, "circuitIdentity.circuitId"), "present")
    target["coreFile"] = LANGUAGE.as_posix()
    target["coreSymbol"] = "signing"
    target["coreContext"] = "Allowance"
    target["note"] = "signing appears only in a file comment."
    replace_row(payload, "circuitIdentity.circuitId", target)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: circuitIdentity.circuitId core symbol signing does not occur as a declaration in "
        "Allowance in experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    ) in process.stdout


def test_false_absence_of_realised_authority_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = absent_row(
        embedding_row(payload, "authority.consumed"),
        ["authority.consumed", "spent", "allowance_spent"],
        "No allowance or spent field exists.",
    )
    replace_row(payload, "authority.consumed", target)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed search term 'spent' is declared" in process.stdout
    assert "FAIL: authority.consumed search term 'allowance_spent' is declared" in process.stdout


def test_false_absence_denies_declared_leaf_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = absent_row(
        embedding_row(payload, "liabilities.opening[].debtor"),
        ["liabilities.opening[].debtor", "debtor"],
        "No debtor exists in source/5 or core/1.",
    )
    replace_row(payload, "liabilities.opening[].debtor", target)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: liabilities.opening[].debtor search term 'debtor' is declared"
    ) in process.stdout


def test_imported_profile_constant_is_not_identity(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    core = citation_row(embedding_row(payload, "programIdentity.coreRef"), "present")
    core["coreFile"] = "experiments/moriarty-language/src/successor/funded-expression-source-v1.ts"
    core["coreSymbol"] = "LIFECYCLE_VERSION"
    core["coreContext"] = "LIFECYCLE_VERSION"
    core["note"] = "LIFECYCLE_VERSION is a profile version."
    source = citation_row(embedding_row(payload, "profiles.semanticProfile"), "present")
    source["sourceFile"] = (
        "experiments/moriarty-language/src/successor/financial-agreement-source-compiler.ts"
    )
    source["sourceSymbol"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE"
    source["sourceContext"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE"
    source["note"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE is a profile version."
    replace_row(payload, "programIdentity.coreRef", core)
    replace_row(payload, "profiles.semanticProfile", source)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: programIdentity.coreRef core symbol LIFECYCLE_VERSION "
        "is a profile version constant, not a program identity"
    ) in process.stdout
    assert (
        "FAIL: profiles.semanticProfile source symbol FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE "
        "is a profile version constant, not a program identity"
    ) in process.stdout


def test_unapproved_schema_keyword_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    schema = load(root, SCHEMA)
    schema["not"] = {}
    write_json(root / SCHEMA, schema)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: unapproved schema keyword 'not' at <root>" in process.stdout
    assert "OK:" not in process.stdout


def test_noncanonical_indent_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = load(root, EMBEDDINGS)
    path.write_text(json.dumps(payload, indent=4, ensure_ascii=False) + "\n", encoding="utf-8")

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json "
        "is not canonical UTF-8 JSON (2-space indent, trailing newline)"
    ) in process.stdout


def test_comment_only_mention_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    materialize_language(root)
    path = root / "experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    text = path.read_text(encoding="utf-8")
    old = "export interface Allowance {\n  party: Identifier;\n"
    new = "export interface Allowance {\n  // ghostSpent: Identifier;\n  party: Identifier;\n"
    assert old in text
    path.write_text(text.replace(old, new, 1), encoding="utf-8")
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["coreSymbol"] = "ghostSpent"
    target["coreContext"] = "Allowance"
    target["note"] = (
        "Exists: ghostSpent is named only in a comment inside Allowance. "
        "Missing: Allowance does not declare ghostSpent."
    )
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.consumed core symbol ghostSpent does not occur as a declaration in "
        "Allowance in experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    ) in process.stdout


def test_same_leaf_in_different_record_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = citation_row(embedding_row(payload, "effects.fees[].amount"), "partial")
    target["coreFile"] = LANGUAGE.as_posix()
    target["coreSymbol"] = "amount"
    target["coreContext"] = "Allowance"
    target["note"] = (
        "Exists: amount is a field of Balance. "
        "Missing: Allowance does not declare amount, so this is not a fee amount."
    )
    replace_row(payload, "effects.fees[].amount", target)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: effects.fees[].amount core symbol amount does not occur as a declaration in "
        "Allowance in experiments/moriarty-language/src/successor/financial-lifecycle.ts"
    ) in process.stdout


def test_absent_search_term_declared_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "circuitIdentity.circuitId")
    target["searchTerms"] = ["circuitIdentity.circuitId", "spent"]
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: circuitIdentity.circuitId search term 'spent' is declared" in process.stdout


def test_partial_empty_note_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["note"] = ""
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed partial note is empty" in process.stdout


def test_present_boolean_must_match_realisation(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["present"] = False
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: authority.consumed present is False, expected True" in process.stdout


def test_absent_present_boolean_must_stay_false(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "domain.chainId")
    target["present"] = True
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: domain.chainId present is True, expected False" in process.stdout


def test_absent_search_must_include_bare_leaf(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "domain.chainId")
    target["searchTerms"] = ["domain.chainId", "chain"]
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: domain.chainId searchTerms does not include the bare leaf chainId" in process.stdout


def test_comma_declarator_requires_review(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "domain.chainId")
    target["reviewedNonRealisations"] = []
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: domain.chainId search term 'chainId' is declared in "
        "experiments/moriarty-language/formal/k/fixtures/build-lifecycle.mjs "
        "without reviewedNonRealisations"
    ) in process.stdout


def test_other_profile_citation_is_not_a_realisation(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = citation_row(embedding_row(payload, "schemaVersion"), "partial")
    target["coreFile"] = "experiments/moriarty-language/src/successor/core.ts"
    target["coreSymbol"] = "schemaVersion"
    target["coreContext"] = "FundedCore"
    target["note"] = (
        "Exists: FundedCore.schemaVersion is a funded-source document version. "
        "Missing: it is not moriarty-stage-relation/1."
    )
    replace_row(payload, "schemaVersion", target)
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: schemaVersion core file does not belong to moriarty-financial-lifecycle/1"
    ) in process.stdout


def test_lowercase_note_mention_must_be_declared(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "authority.consumed")
    target["note"] = (
        "Exists: financialRead.allowance_spent and Allowance.spent are cited, "
        "and financialRead.notATerminal is not a terminal. "
        "Missing: there is no authority record."
    )
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert (
        "FAIL: authority.consumed note names financialRead.notATerminal "
        "but it is not a declaration"
    ) in process.stdout


def test_invented_compound_synonym_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root, EMBEDDINGS)
    target = embedding_row(payload, "effects.fees[].amount")
    target["searchTerms"] = ["effects.fees[].amount", "amount", "effectsFeesAmount"]
    target["reviewedNonRealisations"] = []
    write_json(root / EMBEDDINGS, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: effects.fees[].amount searchTerms synonym is an invented compound" in process.stdout
    assert "FAIL: effects.fees[].amount search term 'amount' is declared" in process.stdout
