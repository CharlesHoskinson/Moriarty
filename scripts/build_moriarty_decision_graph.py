#!/usr/bin/env python3
"""Build a queryable graph for the Moriarty decision and research corpus."""

from __future__ import annotations

import hashlib
import json
import re
from importlib.metadata import version
from pathlib import Path

from graphify.build import build_from_json
from graphify.cluster import cluster, label_communities_by_hub
from graphify.export import to_json


ROOT = Path(__file__).resolve().parents[1]
GRAPH_ROOT = ROOT / "graphs" / "moriarty-decision-corpus"
SEMANTIC = GRAPH_ROOT / ".graphify_semantic.json"
DETECT = GRAPH_ROOT / ".graphify_detect.json"
OUTPUT = GRAPH_ROOT / "graphify-out"


def identifier(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", value.lower()).strip("_")


def main() -> None:
    extraction = json.loads(SEMANTIC.read_text(encoding="utf-8"))
    detection = json.loads(DETECT.read_text(encoding="utf-8"))
    expected_files = {
        str(Path(filename).resolve())
        for kind in ("document", "paper")
        for filename in detection.get("files", {}).get(kind, [])
    }
    extracted_files = {
        str(Path(str(node["source_file"])).resolve())
        for node in extraction.get("nodes", [])
    }
    if expected_files != extracted_files:
        missing = sorted(expected_files - extracted_files)
        extra = sorted(extracted_files - expected_files)
        raise SystemExit(
            f"Semantic extraction is stale: missing={missing!r}, extra={extra!r}"
        )

    node_ids = {str(node["id"]) for node in extraction.get("nodes", [])}
    missing_edges = [
        edge
        for edge in extraction.get("edges", [])
        if str(edge.get("source")) not in node_ids or str(edge.get("target")) not in node_ids
    ]
    if missing_edges:
        raise SystemExit(f"Semantic extraction has {len(missing_edges)} missing-endpoint edges")

    graph = build_from_json(extraction, directed=True, root=ROOT)
    root_id = "moriarty_decision_research_corpus"
    graph.add_node(
        root_id,
        label="Moriarty Decision and Research Corpus",
        file_type="concept",
        source_file="graphs/moriarty-decision-corpus/.graphify_detect.json",
        semantic_status="curated corpus root",
    )
    for filename in sorted(expected_files):
        relative = str(Path(filename).relative_to(ROOT))
        source_id = f"moriarty_source_{identifier(relative)}"
        graph.add_node(
            source_id,
            label=f"Source: {relative}",
            file_type="document",
            source_file=relative,
            semantic_status="graph input",
        )
        graph.add_edge(
            root_id,
            source_id,
            relation="indexes_source",
            confidence="EXTRACTED",
            confidence_score=1.0,
            source_file="graphs/moriarty-decision-corpus/.graphify_detect.json",
            weight=1.0,
            _src=root_id,
            _tgt=source_id,
        )
        matching = sorted(
            str(node)
            for node, attrs in graph.nodes(data=True)
            if str(Path(str(attrs.get("source_file", ""))).resolve()) == filename
            and str(node) != source_id
        )
        if matching:
            target = matching[0]
            graph.add_edge(
                source_id,
                target,
                relation="contains_concept",
                confidence="EXTRACTED",
                confidence_score=1.0,
                source_file=relative,
                weight=1.0,
                _src=source_id,
                _tgt=target,
            )

    communities = cluster(graph)
    labels = label_communities_by_hub(graph, communities)
    OUTPUT.mkdir(parents=True, exist_ok=True)
    graph_path = OUTPUT / "graph.json"
    source_hash = hashlib.sha256(SEMANTIC.read_bytes()).hexdigest()
    to_json(
        graph,
        communities,
        str(graph_path),
        force=True,
        built_at_commit=f"semantic-sha256:{source_hash}",
        community_labels=labels,
    )

    result = {
        "schema_version": 1,
        "tool_versions": {"graphify_api": version("graphifyy")},
        "semantic_sha256": source_hash,
        "source_files": len(expected_files),
        "built_counts": {
            "nodes": graph.number_of_nodes(),
            "edges": graph.number_of_edges(),
            "hyperedges": len(graph.graph.get("hyperedges", [])),
            "communities": len(communities),
            "missing_endpoint_edges": len(missing_edges),
        },
        "graph": str(graph_path.relative_to(ROOT)),
        "semantic": str(SEMANTIC.relative_to(ROOT)),
        "qualification": (
            "Semantic relations were extracted by an agy worker under the Graphify "
            "schema. INFERRED and AMBIGUOUS edges are hypotheses, not authority."
        ),
    }
    evidence_path = ROOT / "evidence" / "moriarty-decision-graph-2026-09-03.json"
    evidence_path.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
