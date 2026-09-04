import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def test_explicit_output_preserves_historical_receipt(tmp_path, capsys) -> None:
    from scripts import write_marlowe_graph_evidence as writer

    historical = writer.OUTPUT.read_bytes()
    output = tmp_path / "graph-evidence.json"
    writer.main(["--output", str(output)])
    assert writer.OUTPUT.read_bytes() == historical
    assert json.loads(output.read_text()) == json.loads(capsys.readouterr().out)


def test_graph_evidence_manifest_matches_built_graph(tmp_path) -> None:
    output = tmp_path / "graph-evidence.json"
    process = subprocess.run(
        [
            sys.executable,
            str(ROOT / "scripts" / "write_marlowe_graph_evidence.py"),
            "--output",
            str(output),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    assert process.returncode == 0, process.stderr

    manifest = json.loads(process.stdout)
    assert json.loads(output.read_text()) == manifest
    assert manifest["tool_versions"]["graphify_api"] == "0.9.53"
    assert manifest["built_counts"]["nodes"] == 8864
    assert manifest["built_counts"]["edges"] == 10888
    assert manifest["repository_coverage"] == 36
    assert manifest["diagnostics"]["missing_endpoint_edges"] == 0
    assert manifest["artifacts"]["graph.json"]["bytes"] > 0
    assert len(manifest["artifacts"]["graph.json"]["sha256"]) == 64
    assert manifest["artifacts"]["cost.json"]["bytes"] > 0
