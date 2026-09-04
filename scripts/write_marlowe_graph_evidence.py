#!/usr/bin/env python3
"""Write a compact, hash-addressed evidence manifest for the Marlowe graph."""

from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GRAPH_OUT = ROOT / "graphs" / "marlowe-org-full" / "graphify-out"
OUTPUT = ROOT / "evidence" / "marlowe-org-full-graph-2026-09-02.json"


def artifact(path: Path) -> dict[str, object]:
    return {
        "path": str(path.relative_to(ROOT)),
        "bytes": path.stat().st_size,
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=OUTPUT)
    output = parser.parse_args(argv).output
    analysis = json.loads((GRAPH_OUT / ".graphify_analysis.json").read_text(encoding="utf-8"))
    diagnostic = analysis["diagnostic"]
    artifact_names = (
        "graph.json",
        "GRAPH_REPORT.md",
        "DIAGNOSTICS.md",
        ".graphify_analysis.json",
        ".graphify_extract.json",
        ".graphify_labels.json",
        "graph.html",
        "GRAPH_TREE.html",
        "cost.json",
    )
    manifest = {
        "schema_version": 1,
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "build_id": analysis["build_id"],
        "tool_versions": analysis["tool_versions"],
        "source_acquisition": {
            "method": "Scrapling for online sources; git fast-forward fetch for repositories",
            "repository_inventory": "raw/sources/marlowe-lang-repositories-sprint-2026-09-02.json.receipt.json",
            "repository_locks": "evidence/repository-locks-2026-09-02.tsv",
            "live_documentation": "evidence/marlowe-docs-live-acquisition-2026-09-02.json",
            "official_entry_points": "graphs/marlowe-online-sources/.graphify_semantic.json",
            "user_taxonomy_report": "raw/sources/reclassifying-defi-taxonomy-report-2026-09-02.pdf.receipt.json",
        },
        "input_hashes": analysis["input_hashes"],
        "raw_counts": analysis["raw_counts"],
        "built_counts": analysis["built_counts"],
        "repository_coverage": len(analysis["repository_coverage"]),
        "repository_names": analysis["repository_coverage"],
        "token_cost": analysis["token_cost"],
        "diagnostics": {
            key: diagnostic[key]
            for key in (
                "node_count",
                "raw_edge_count",
                "valid_candidate_edges",
                "missing_endpoint_edges",
                "dangling_endpoint_edges",
                "self_loop_edges",
                "undirected_same_endpoint_collapsed_edges",
                "post_build_node_count",
                "post_build_edge_count",
            )
        },
        "coverage_limits": [
            "Three R files were unsupported by the AST extractor.",
            "Two TSX files had partial syntax errors and one tsconfig.json was invalid.",
            "Twenty-four demonstration fixture paths with key/address material were excluded from semantic prose extraction.",
            "marlowe-runner/e2e/artifacts was excluded as generated output.",
            "The 783 images are path-indexed and were not vision-analyzed.",
            "Seventeen pinned repository worktrees contain untracked research-generated Graphify output/cache files; tracked trees still match their locked commits.",
            "Inferred semantic links are discovery leads, not source-backed proof of correspondence.",
            "The installed Graphify skill text is 0.9.48 while the build API and CLI are 0.9.53.",
        ],
        "artifacts": {name: artifact(GRAPH_OUT / name) for name in artifact_names},
    }
    output.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    main()
