import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json"
)
EMBEDDINGS = Path("deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json")
JUDGMENTS = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
DESIGN_DOC = Path("docs/MORIARTY-CONSOLIDATED-DESIGN.md")
CHECKER = ROOT / "scripts" / "check_u0_stage_schema.py"
LANGUAGE = Path("experiments/moriarty-language/src/successor/financial-lifecycle.ts")
FRONTEND = Path("experiments/moriarty-language/src/successor/frontend.ts")


def run_checker(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(CHECKER), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def copy_root(tmp_path: Path) -> Path:
    root = tmp_path / "root"
    for rel in (SCHEMA, EMBEDDINGS, JUDGMENTS, DESIGN_DOC):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((ROOT / rel).read_bytes())
    language = root / "experiments" / "moriarty-language"
    language.parent.mkdir(parents=True, exist_ok=True)
    language.symlink_to(ROOT / "experiments" / "moriarty-language")
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
    assert process.stdout == "OK: 84 leaf fields, 5 present, 79 absent\n"
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
        "FAIL: authority.remaining core symbol remain does not occur in "
        "experiments/moriarty-language/src/successor/financial-lifecycle.ts"
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
    core = embedding_row(payload, "programIdentity.coreRef")
    core["present"] = True
    core["coreFile"] = LANGUAGE.as_posix()
    core["coreSymbol"] = "LIFECYCLE_VERSION"
    source = embedding_row(payload, "programIdentity.sourceRef")
    source["present"] = True
    source["sourceFile"] = FRONTEND.as_posix()
    source["sourceSymbol"] = "FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE"
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
