from __future__ import annotations

import importlib.util
import json
import shutil
import subprocess
import sys
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
SCHEMA = Path(
    "openspec/changes/consolidated-language-kernel/schemas/k-reconciliation.schema.json"
)
ARTIFACT = Path("deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json")
SPEC = Path(
    "openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
)
JUDGMENTS = Path("deliverables/u0-semantic-contract-2026-09-23/judgments.json")
K_ROOT = Path("experiments/moriarty-language/formal/k")
EXECUTION = Path("deliverables/k-lifecycle-execution-2026-09-17")
CHECKER = MORIARTY_ROOT / "scripts" / "check_u0_k_reconciliation.py"
LIMITATION = (
    "Coverage is a reviewed claim; the checker proves citations and evidence quotes only."
)
PREVIEW_QUOTE = (
    "The separate September 17 Preview delivery targets the existing fixed LAM test-asset loan."
)
ROUND_QUOTE = '{"case": 33, "id": "round-floor", "matchesExpected": true}'


def run_checker(root: Path | None = None) -> subprocess.CompletedProcess[str]:
    command = [sys.executable, str(CHECKER)]
    if root is not None:
        command.extend(["--root", str(root)])
    return subprocess.run(
        command,
        cwd=MORIARTY_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def copy_root(tmp_path: Path) -> Path:
    root = tmp_path / "root"
    for rel in (SCHEMA, ARTIFACT, SPEC, JUDGMENTS):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(MORIARTY_ROOT / rel, destination)
    for rel in (K_ROOT, EXECUTION):
        destination = root / rel
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(MORIARTY_ROOT / rel, destination, symlinks=False)
    return root


def load_checker():
    spec = importlib.util.spec_from_file_location("check_u0_k_reconciliation", CHECKER)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def load(root: Path) -> dict:
    return json.loads((root / ARTIFACT).read_text(encoding="utf-8"))


def write(root: Path, payload: dict) -> None:
    (root / ARTIFACT).write_text(
        json.dumps(payload, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def row(payload: dict, row_id: str) -> dict:
    return next(item for item in payload["rows"] if item["id"] == row_id)


def test_real_artifacts_pass() -> None:
    process = run_checker()
    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    artifact = json.loads((MORIARTY_ROOT / ARTIFACT).read_text(encoding="utf-8"))
    statuses = [item["status"] for item in artifact["rows"]]
    assert statuses.count("covered") == 0
    assert statuses.count("partial") == 5
    assert statuses.count("not-covered") == 18
    assert process.stdout == (
        "OK: 23 rows, 0 covered, 5 partial, 18 not-covered\n"
        f"{LIMITATION}\n"
    )
    assert len(CHECKER.read_text(encoding="utf-8").splitlines()) <= 400
    assert artifact["limitation"] == LIMITATION
    assert artifact["kRoots"] == ["experiments/moriarty-language/formal/k"]
    permission = row(artifact, "UNI-001")
    assert permission["kind"] == "uni"
    assert permission["title"] == "Permissionless public pipeline"
    assert permission["status"] == "not-covered"
    assert permission["kCitations"] == []
    assert permission["evidence"] == []
    assert "lacks" in permission["note"]
    financial = row(artifact, "UNI-005")
    assert financial["status"] == "partial"
    assert "Covers:" in financial["note"] and "Missing:" in financial["note"]
    assert {item["symbol"] for item in financial["kCitations"]} >= {
        "lcTransfer",
        "lcOriginate",
        "lcAccrue",
        "lcRepay",
        "<effects>",
    }
    assert row(artifact, "UNI-014")["kCitations"][0]["symbol"] == "lcRound"
    assert row(artifact, "effect")["status"] == "partial"
    assert row(artifact, "authority")["kCitations"][0]["symbol"] == "lcTransfer"
    assert row(artifact, "failure")["kCitations"][0]["symbol"] == "lcKernelRejected"
    assert row(artifact, "stage")["status"] == "not-covered"
    assert row(artifact, "intent")["kCitations"] == []
    assert row(artifact, "history")["kCitations"] == []
    assert [item["id"] for item in artifact["rows"][:17]] == [f"UNI-{index:03d}" for index in range(1, 18)]
    assert [item["id"] for item in artifact["rows"][17:]] == [
        "stage",
        "intent",
        "effect",
        "authority",
        "history",
        "failure",
    ]


def test_c1_schema_rejects_extra_property(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    payload["extra"] = True
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "schema" in process.stdout


def test_c2_title_must_match_spec(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "UNI-001")["title"] = "Wrong title"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: UNI-001 title is 'Wrong title'" in process.stdout


def test_c3_covered_requires_citation_and_evidence(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "UNI-001")["status"] = "covered"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: UNI-001 covered requires a K citation and an execution quote" in process.stdout


def test_c4_missing_citation_file_is_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "UNI-005")["kCitations"][0]["file"] = (
        "experiments/moriarty-language/formal/k/missing-declaration.k"
    )
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 2, process.stdout + process.stderr
    assert (
        "blocked: missing experiments/moriarty-language/formal/k/missing-declaration.k"
        in process.stdout
    )


def test_c5_unknown_symbol_is_not_a_declaration(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    citation = row(payload, "UNI-005")["kCitations"][0]
    citation["symbol"] = "notDeclared"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol 'notDeclared' is not declared in module" in process.stdout


def test_c6_evidence_quote_must_occur_verbatim(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    item = row(payload, "UNI-005")["evidence"][0]
    item["quote"] = "this quote is not in the execution record"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert f"FAIL: UNI-005 evidence quote does not occur in {item['path']}" in process.stdout


def test_c7_execution_quote_must_occur_verbatim(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    payload["executionEvidence"]["quotes"][0] = "this execution quote is absent"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "FAIL: executionEvidence quote does not occur in "
        "deliverables/k-lifecycle-execution-2026-09-17/RESULT.md"
    ) in process.stdout


def test_c1_null_artifact_is_validated(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    (root / ARTIFACT).write_text("null\n", encoding="utf-8")
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "schema" in process.stdout


def test_c1_null_schema_is_validated(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    (root / SCHEMA).write_text("null\n", encoding="utf-8")
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: reconciliation schema is not an object" in process.stdout


def test_c3_not_covered_note_must_name_the_gap(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "UNI-001")["note"] = "lacks"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: UNI-001 not-covered note does not say what K lacks" in process.stdout
    row(payload, "UNI-001")["note"] = "n/a"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: UNI-001 not-covered note does not say what K lacks" in process.stdout


def test_c3_covered_unrelated_quote_is_not_execution(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "UNI-014")
    target["status"] = "covered"
    target["evidence"] = [
        {
            "path": "deliverables/k-lifecycle-execution-2026-09-17/RESULT.md",
            "quote": PREVIEW_QUOTE,
        }
    ]
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "FAIL: UNI-014 covered evidence does not show execution of a cited declaration"
        in process.stdout
    )
    target["evidence"] = [
        {
            "path": "deliverables/k-lifecycle-execution-2026-09-17/RESULT.md",
            "quote": "Both complete suites exited zero.",
        }
    ]
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert (
        "FAIL: UNI-014 covered evidence does not show execution of a cited declaration"
        in process.stdout
    )


def test_c3_covered_round_floor_record_passes(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    target = row(payload, "UNI-014")
    target["status"] = "covered"
    target["evidence"] = [
        {
            "path": "deliverables/k-lifecycle-execution-2026-09-17/lifecycle104-02.stdout",
            "quote": ROUND_QUOTE,
        }
    ]
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 0, process.stdout + process.stderr


def test_symlink_outside_root_is_rejected(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    outside = tmp_path / "outside.md"
    outside.write_text(PREVIEW_QUOTE + "\n", encoding="utf-8")
    link = root / "deliverables" / "outside-evidence.md"
    link.symlink_to(Path("..") / ".." / "outside.md")
    payload = load(root)
    item = row(payload, "UNI-005")["evidence"][0]
    item["path"] = "deliverables/outside-evidence.md"
    item["quote"] = PREVIEW_QUOTE
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL: UNI-005 path escapes the root: deliverables/outside-evidence.md" in process.stdout


def test_failures_print_before_blocked(tmp_path: Path) -> None:
    root = copy_root(tmp_path)
    payload = load(root)
    row(payload, "UNI-001")["title"] = "Wrong title"
    row(payload, "UNI-005")["kCitations"][0]["file"] = (
        "experiments/moriarty-language/formal/k/missing-declaration.k"
    )
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 2, process.stdout + process.stderr
    assert "FAIL: UNI-001 title is 'Wrong title'" in process.stdout
    assert (
        "blocked: missing experiments/moriarty-language/formal/k/missing-declaration.k"
        in process.stdout
    )
    assert process.stdout.index("FAIL:") < process.stdout.index("blocked:")


def test_invalid_arguments_are_blocked() -> None:
    process = subprocess.run(
        [sys.executable, str(CHECKER), "--not-a-real-flag"],
        cwd=MORIARTY_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    assert process.returncode == 2, process.stdout + process.stderr
    assert process.stdout.startswith("blocked:")


def test_c5_quoted_terminal_is_not_a_constructor(tmp_path: Path) -> None:
    checker = load_checker()
    declared = checker.module_symbols('module M\n syntax S ::= "fake()" | real()\nendmodule')
    assert declared == {"M": {"M", "real"}}
    lexical = checker.module_symbols(
        'module M\n syntax lexical L ::= r"fake()" | lexicalCtor()\n syntax S ::= real()\nendmodule'
    )
    assert "fake" not in lexical["M"]
    assert "lexicalCtor" not in lexical["M"]
    assert "real" in lexical["M"]
    root = copy_root(tmp_path)
    k_rel = K_ROOT / "quoted-terminal.k"
    (root / k_rel).write_text(
        "module QUOTED-TERMINAL\n"
        '  syntax lexical Lex ::= r"fake()" | lexicalCtor()\n'
        '  syntax S ::= "fake()" | real()\n'
        "endmodule\n",
        encoding="utf-8",
    )
    payload = load(root)
    citation = row(payload, "UNI-005")["kCitations"][0]
    citation["file"] = k_rel.as_posix()
    citation["context"] = "QUOTED-TERMINAL"
    citation["symbol"] = "fake"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 1, process.stdout + process.stderr
    assert "symbol 'fake' is not declared in module QUOTED-TERMINAL" in process.stdout
    citation["symbol"] = "real"
    write(root, payload)
    process = run_checker(root)
    assert process.returncode == 0, process.stdout + process.stderr
