import json
import subprocess
import sys
from pathlib import Path

import pytest

from moriarty.evidence import validate_e00_evidence


ROOT = Path(__file__).resolve().parents[1]
GRAPHIFY_PYTHON = Path("/home/charl/.local/share/uv/tools/graphifyy/bin/python3")


def test_e00_graph_gate_rejects_a_failed_stop_test() -> None:
    certificate = json.loads(
        (ROOT / "experiments/moriarty-core-swap/translation-certificate.json").read_text(
            encoding="utf-8"
        )
    )
    toolchain = json.loads(
        (ROOT / "experiments/moriarty-core-swap/toolchain-results.json").read_text(
            encoding="utf-8"
        )
    )
    certificate["stop_test_passed"] = False

    with pytest.raises(SystemExit, match="did not pass"):
        validate_e00_evidence(certificate, toolchain)


def test_moriarty_decision_graph_builds_from_semantic_extraction() -> None:
    process = subprocess.run(
        [
            str(GRAPHIFY_PYTHON if GRAPHIFY_PYTHON.exists() else Path(sys.executable)),
            str(ROOT / "scripts" / "build_moriarty_decision_graph.py"),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    result = json.loads(process.stdout)
    assert result["tool_versions"]["graphify_api"] == "0.9.53"
    assert result["source_files"] == 6
    assert result["built_counts"]["nodes"] >= 36
    assert result["built_counts"]["edges"] >= 44
    assert result["built_counts"]["missing_endpoint_edges"] == 0

    graph = json.loads((ROOT / result["graph"]).read_text(encoding="utf-8"))
    labels = {node["label"] for node in graph["nodes"]}
    assert "Moriarty Decision and Research Corpus" in labels
    assert "Moriarty E00 Atomic Swap" in labels
    assert "E00 Translation Certificate" in labels
    assert any("Moriarty DeFi Kernel Deep Research" in label for label in labels)
    relations = {link["relation"] for link in graph["links"]}
    assert "lowers_to" in relations
    assert "compiles_to" in relations
    assert "validates_against" in relations
