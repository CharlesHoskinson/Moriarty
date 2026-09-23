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
CHECKER = ROOT / "scripts" / "check_u0_stage_schema.py"


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
    for rel in (SCHEMA, EMBEDDINGS, JUDGMENTS):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes((ROOT / rel).read_bytes())
    language = root / "experiments" / "moriarty-language"
    language.parent.mkdir(parents=True, exist_ok=True)
    language.symlink_to(ROOT / "experiments" / "moriarty-language")
    return root


def write_json(path: Path, payload: object) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def test_real_artifacts_pass() -> None:
    process = run_checker()
    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stdout == "OK: 84 leaf fields, 13 present, 71 absent\n"
    assert process.stderr == ""


def test_deleted_embedding_row_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = json.loads(path.read_text(encoding="utf-8"))
    payload["rows"] = payload["rows"][1:]
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL:" in process.stdout
    assert "OK:" not in process.stdout


def test_citation_of_missing_symbol_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / EMBEDDINGS
    payload = json.loads(path.read_text(encoding="utf-8"))
    target = next(row for row in payload["rows"] if row["schemaField"] == "programIdentity.coreRef")
    target["coreSymbol"] = "NOT_A_LIFECYCLE_SYMBOL"
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: programIdentity.coreRef core symbol NOT_A_LIFECYCLE_SYMBOL does not occur" in process.stdout


def test_forbidden_judgment_status_fails(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    path = root / SCHEMA
    payload = json.loads(path.read_text(encoding="utf-8"))
    status = payload["properties"]["judgments"]["properties"]["stage"]["properties"]["status"]
    status["enum"] = ["held", "green", "unchecked"]
    write_json(path, payload)

    process = run_checker("--root", str(root))

    assert process.returncode == 1
    assert "FAIL: judgments.stage.status enum is ['held', 'green', 'unchecked']" in process.stdout
