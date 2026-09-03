import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def test_god_nodes_exclude_configuration_and_ui_scaffolding() -> None:
    analysis = json.loads(
        (
            ROOT
            / "graphs"
            / "marlowe-org-full"
            / "graphify-out"
            / ".graphify_analysis.json"
        ).read_text(encoding="utf-8")
    )
    labels = {node["label"] for node in analysis["god_nodes"]}

    assert labels.isdisjoint(
        {"compilerOptions", "ICON_SIZES", "PAGES", "SIZE", "Button()", "isEmpty()"}
    )
    assert analysis["tool_versions"]["graphify_api"] == "0.9.53"

    report = (
        ROOT / "graphs" / "marlowe-org-full" / "graphify-out" / "GRAPH_REPORT.md"
    ).read_text(encoding="utf-8")
    assert "Token cost: 2,234,501 input · 1,280,583 output" in report
    assert "Built from commit: `352afcf2`" in report
