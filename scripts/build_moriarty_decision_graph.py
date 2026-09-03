#!/usr/bin/env python3
"""Build a queryable graph for the Moriarty decision and research corpus."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from importlib.metadata import version
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from graphify.build import build_from_json
from graphify.cluster import cluster, label_communities_by_hub
from graphify.export import to_json

from moriarty.evidence import validate_e00_evidence


SEMANTIC = ROOT / "evidence/moriarty-decision-semantic-2026-09-03.json"
DETECT = ROOT / "evidence/moriarty-decision-detection-2026-09-03.json"
GRAPH_PATH = ROOT / "evidence/moriarty-decision-graph-data-2026-09-03.json"
E00_CERTIFICATE = ROOT / "experiments" / "moriarty-core-swap" / "translation-certificate.json"
E00_TOOLCHAIN = ROOT / "experiments" / "moriarty-core-swap" / "toolchain-results.json"


def identifier(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", value.lower()).strip("_")


def source_path(value: str) -> Path:
    path = Path(value)
    return path if path.is_absolute() else ROOT / path


def add_extracted_edge(graph, source: str, target: str, relation: str, evidence: str) -> None:
    graph.add_edge(
        source,
        target,
        relation=relation,
        confidence="EXTRACTED",
        confidence_score=1.0,
        source_file=evidence,
        weight=1.0,
        _src=source,
        _tgt=target,
    )


def add_e00_evidence(graph, root_id: str) -> None:
    certificate = json.loads(E00_CERTIFICATE.read_text(encoding="utf-8"))
    toolchain = json.loads(E00_TOOLCHAIN.read_text(encoding="utf-8"))
    validate_e00_evidence(
        certificate,
        toolchain,
        artifact_directory=E00_CERTIFICATE.parent,
    )

    certificate_source = str(E00_CERTIFICATE.relative_to(ROOT))
    toolchain_source = str(E00_TOOLCHAIN.relative_to(ROOT))
    nodes = {
        "moriarty_e00_atomic_swap": (
            "Moriarty E00 Atomic Swap",
            "experiment",
            certificate_source,
        ),
        "moriarty_e00_core": (
            "E00 Finite Moriarty Core",
            "semantics",
            certificate_source,
        ),
        "moriarty_e00_compact": (
            "E00 Generated Compact",
            "code",
            toolchain_source,
        ),
        "moriarty_e00_manifest_machine": (
            "E00 Independent Manifest Machine",
            "code",
            certificate_source,
        ),
        "moriarty_e00_zkir": (
            "E00 Four ZKIR 3 Circuits",
            "artifact",
            toolchain_source,
        ),
        "moriarty_e00_certificate": (
            "E00 Translation Certificate",
            "evidence",
            certificate_source,
        ),
        "moriarty_e00_decision_disclosure": (
            "E00 Public Decision Disclosure",
            "security",
            toolchain_source,
        ),
    }
    for node_id, (label, file_type, source_file) in nodes.items():
        graph.add_node(
            node_id,
            label=label,
            file_type=file_type,
            source_file=source_file,
            semantic_status="EXTRACTED",
        )

    add_extracted_edge(
        graph,
        root_id,
        "moriarty_e00_atomic_swap",
        "indexes_experiment",
        certificate_source,
    )
    for target in (
        "moriarty_e00_core",
        "moriarty_e00_compact",
        "moriarty_e00_manifest_machine",
        "moriarty_e00_zkir",
        "moriarty_e00_certificate",
        "moriarty_e00_decision_disclosure",
    ):
        add_extracted_edge(
            graph,
            "moriarty_e00_atomic_swap",
            target,
            "contains_evidence",
            certificate_source,
        )
    add_extracted_edge(
        graph,
        "moriarty_e00_core",
        "moriarty_e00_compact",
        "lowers_to",
        toolchain_source,
    )
    add_extracted_edge(
        graph,
        "moriarty_e00_compact",
        "moriarty_e00_zkir",
        "compiles_to",
        toolchain_source,
    )
    add_extracted_edge(
        graph,
        "moriarty_e00_certificate",
        "moriarty_e00_core",
        "validates_against",
        certificate_source,
    )
    add_extracted_edge(
        graph,
        "moriarty_e00_certificate",
        "moriarty_e00_manifest_machine",
        "validates_against",
        certificate_source,
    )
    add_extracted_edge(
        graph,
        "moriarty_e00_decision_disclosure",
        "moriarty_e00_compact",
        "declared_in",
        toolchain_source,
    )


def main() -> None:
    extraction = json.loads(SEMANTIC.read_text(encoding="utf-8"))
    detection = json.loads(DETECT.read_text(encoding="utf-8"))
    expected_files = {
        str(source_path(filename).resolve())
        for kind in ("document", "paper")
        for filename in detection.get("files", {}).get(kind, [])
    }
    extracted_files = {
        str(source_path(str(node["source_file"])).resolve())
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
        source_file=str(DETECT.relative_to(ROOT)),
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
            source_file=str(DETECT.relative_to(ROOT)),
            weight=1.0,
            _src=root_id,
            _tgt=source_id,
        )
        matching = sorted(
            str(node)
            for node, attrs in graph.nodes(data=True)
            if str(source_path(str(attrs.get("source_file", ""))).resolve())
            == filename
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

    add_e00_evidence(graph, root_id)

    communities = cluster(graph)
    labels = label_communities_by_hub(graph, communities)
    source_hash = hashlib.sha256(SEMANTIC.read_bytes()).hexdigest()
    to_json(
        graph,
        communities,
        str(GRAPH_PATH),
        force=True,
        built_at_commit=f"semantic-sha256:{source_hash}",
        community_labels=labels,
    )

    result = {
        "schema_version": 1,
        "tool_versions": {"graphify_api": version("graphifyy")},
        "semantic_sha256": source_hash,
        "source_files": len(expected_files),
        "curated_experiments": ["E00"],
        "built_counts": {
            "nodes": graph.number_of_nodes(),
            "edges": graph.number_of_edges(),
            "hyperedges": len(graph.graph.get("hyperedges", [])),
            "communities": len(communities),
            "missing_endpoint_edges": len(missing_edges),
        },
        "graph": str(GRAPH_PATH.relative_to(ROOT)),
        "semantic": str(SEMANTIC.relative_to(ROOT)),
        "detection": str(DETECT.relative_to(ROOT)),
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
