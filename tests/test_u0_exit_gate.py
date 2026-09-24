"""Exercise the U0 receipt through its public command-line interface."""

from __future__ import annotations

import json
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
DELIVERABLE = Path("deliverables/u0-semantic-contract-2026-09-23")
CHECKER = Path("scripts/check_u0_exit_gate.py")


def run_checker(root: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(root / CHECKER), "--root", str(root), *args],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )


def copy_tree(tmp_path: Path) -> Path:
    target = tmp_path / "repo"
    # A real copy keeps citation paths inside the temporary repository and
    # leaves the source checkout untouched by adversarial mutations.
    shutil.copytree(
        ROOT,
        target,
        copy_function=shutil.copy2,
        ignore=shutil.ignore_patterns(".harness", ".pytest_cache", "__pycache__"),
    )
    return target


def replace_text(path: Path, text: str) -> None:
    replacement = path.with_name(path.name + ".replacement")
    replacement.write_text(text, encoding="utf-8")
    replacement.replace(path)


def mutate_json(root: Path, name: str, change: object) -> None:
    path = root / DELIVERABLE / name
    payload = json.loads(path.read_text(encoding="utf-8"))
    change(payload)
    replace_text(path, json.dumps(payload, ensure_ascii=False, indent=2) + "\n")


def assert_failure(result: subprocess.CompletedProcess[str], marker: str) -> None:
    assert result.returncode == 1, (result.stdout, result.stderr)
    assert marker in result.stdout
    assert "Traceback" not in result.stderr


def test_real_gate_is_green_and_receipt_is_current() -> None:
    result = run_checker(ROOT)
    assert result.returncode == 0, (result.stdout, result.stderr)
    assert "OK: U0 exit gate" in result.stdout
    receipt = (ROOT / DELIVERABLE / "EXIT-GATE.md").read_text(encoding="utf-8")
    assert "U0 contract recorded; capabilities open" in receipt
    assert receipt.count("| OPEN |") == 8
    assert "## Checker runs" in receipt
    assert "## Limitations" in receipt


@pytest.mark.parametrize(
    ("script", "mutation", "marker"),
    [
        ("check_u0_numeric_profile.py", "print('FAIL: forced checker failure')\nraise SystemExit(1)\n", "G1"),
    ],
)
def test_g1_subchecker_failure(tmp_path: Path, script: str, mutation: str, marker: str) -> None:
    root = copy_tree(tmp_path)
    replace_text(root / "scripts" / script, mutation)
    assert_failure(run_checker(root), marker)


def test_g2_schema_hash_agreement(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "enforcement-map.json", lambda data: data.__setitem__("stageSchemaSha256", "0" * 64))
    assert_failure(run_checker(root), "G2")


def test_g3_enforcement_rows_equal_embedding_rows(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "enforcement-map.json", lambda data: data["rows"].pop())
    assert_failure(run_checker(root), "G3")


def test_g4_k_rows_match_judgment_and_uni_keys(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "k-reconciliation.json", lambda data: data["rows"].pop())
    assert_failure(run_checker(root), "G4")


def test_g5_numeric_decision_hash(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "numeric-profile.json", lambda data: data.__setitem__("decisionSha256", "0" * 64))
    assert_failure(run_checker(root), "G5")


def test_g6_open_target_tuple(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "backend-requirement-matrix.json", lambda data: data["rows"][0].__setitem__("id", "ZR02"))
    assert_failure(run_checker(root), "G6")


def test_g7_gap_cannot_be_reported_closed(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / DELIVERABLE / "EXIT-GATE.md"
    receipt = path.read_text(encoding="utf-8")
    replace_text(path, receipt.replace("| OPEN |", "| CLOSED |", 1))
    assert_failure(run_checker(root), "G7")


def test_g8_receipt_drift(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    path = root / DELIVERABLE / "EXIT-GATE.md"
    replace_text(path, path.read_text(encoding="utf-8") + "drift\n")
    assert_failure(run_checker(root), "G8")


def test_wrong_typed_input_fails_before_checks(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    mutate_json(root, "source-core-embeddings.json", lambda data: data["rows"][0].__setitem__("sourceFile", 17))
    result = run_checker(root)
    assert_failure(result, "sourceFile")
    assert "G1" not in result.stdout


def test_missing_input_blocks(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    (root / DELIVERABLE / "judgments.json").unlink()
    result = run_checker(root)
    assert result.returncode == 2
    assert "blocked: missing" in result.stdout


def test_unavailable_git_metadata_blocks(tmp_path: Path) -> None:
    root = copy_tree(tmp_path)
    (root / ".git").unlink()
    result = run_checker(root)
    assert result.returncode == 2, (result.stdout, result.stderr)
    assert "blocked: git metadata unavailable" in result.stdout


@pytest.mark.parametrize("args", [("--unknown",), ("--root",)])
def test_invalid_arguments_fail_without_traceback(args: tuple[str, ...]) -> None:
    result = subprocess.run(
        [sys.executable, str(ROOT / CHECKER), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    assert_failure(result, "FAIL: invalid arguments:")
