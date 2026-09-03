import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GRAPHIFY_PYTHON = Path("/home/charl/.local/share/uv/tools/graphifyy/bin/python3")


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
    assert any("Moriarty DeFi Kernel Deep Research" in label for label in labels)
