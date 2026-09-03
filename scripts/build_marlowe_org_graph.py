#!/usr/bin/env python3
"""Merge and analyze the reproducible Marlowe organization graph."""

from __future__ import annotations

import hashlib
import json
import csv
import re
from importlib.metadata import version
from pathlib import Path

from graphify.analyze import god_nodes, suggest_questions, surprising_connections
from graphify.build import build_from_json
from graphify.cluster import cluster, label_communities_by_hub, score_all
from graphify.diagnostics import diagnose_extraction, format_diagnostic_report
from graphify.export import to_json
from graphify.report import generate


REPO_ROOT = Path(__file__).resolve().parents[1]
GRAPH_ROOT = REPO_ROOT / "graphs" / "marlowe-org-full"
OUTPUT = GRAPH_ROOT / "graphify-out"
SCAN_ROOT = Path("/tmp/marlowe-org-tracked.20260902-2135")
INPUTS = (
    (GRAPH_ROOT / ".graphify_code.json", "ast"),
    (GRAPH_ROOT / ".graphify_semantic.json", "semantic"),
    (GRAPH_ROOT / ".graphify_images.json", "image-path-index"),
    (REPO_ROOT / "graphs" / "marlowe-live-docs" / ".graphify_semantic.json", "live-docs"),
    (REPO_ROOT / "graphs" / "marlowe-online-sources" / ".graphify_semantic.json", "online-source"),
    (REPO_ROOT / "graphs" / "defi-taxonomy-report" / ".graphify_semantic.json", "user-research"),
)

ONLINE_SOURCE_URLS = {
    "cardano-docs-marlowe-entry-2026-09-02.md": "https://docs.cardano.org/developer-resources/smart-contracts/marlowe",
    "intersect-budget-marlowe-v2-2026-09-02.md": "https://hydra-voting.intersectmbo.org/votes/cardano-budget-2026/69fc8a71b05ff80adc7d5c4f",
    "marlowe-blog-index-2026-09-02.md": "https://marlowe-lang.org/blog/",
    "marlowe-docs-home-2026-09-02.md": "https://docs.marlowe-lang.org/",
    "marlowe-v2-final-report-2026-09-02.txt": "https://docs.google.com/document/d/1Ex7SyN2hys8CD-hmULiwYNkKumGELlEAl15KWkMIpKs/edit",
    "marlowe-v2-sessions-report-2026-09-02.md": "https://marlowe-lang.org/blog/marlowe-v2-sessions-report/",
    "marlowe-website-home-2026-09-02.md": "https://marlowe-lang.org/",
    "project-catalyst-marlowe-v2-2026-09-02.md": "https://projectcatalyst.io/funds/13/cardano-use-cases-concept/marlowe-2025-marlowe-v2",
}


def load_and_stamp(path: Path, origin: str) -> dict[str, object]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    live_docs: dict[str, dict[str, object]] = {}
    if origin == "live-docs":
        manifest = json.loads(
            (REPO_ROOT / "evidence" / "marlowe-docs-live-acquisition-2026-09-02.json").read_text(
                encoding="utf-8"
            )
        )
        live_docs = {
            Path(str(capture["text_path"])).name: capture for capture in manifest["captures"]
        }
    for node in payload.get("nodes", []):
        node.setdefault("_origin", origin)
        source_name = Path(str(node.get("source_file", ""))).name
        if origin == "live-docs" and source_name in live_docs:
            capture = live_docs[source_name]
            node.setdefault("source_url", capture.get("final_url"))
            node.setdefault("captured_at", manifest.get("retrieved_at_utc"))
        elif origin == "online-source" and source_name in ONLINE_SOURCE_URLS:
            node.setdefault("source_url", ONLINE_SOURCE_URLS[source_name])
            node.setdefault("captured_at", "2026-09-02")
        elif origin == "user-research":
            node.setdefault(
                "source_document",
                "raw/sources/reclassifying-defi-taxonomy-report-2026-09-02.pdf",
            )
            node.setdefault("captured_at", "2026-09-02T23:07:02Z")
    for edge in payload.get("edges", []):
        edge.setdefault("_origin", origin)
    for hyperedge in payload.get("hyperedges", []):
        hyperedge.setdefault("_origin", origin)
    return payload


def repo_from_source(value: object) -> str | None:
    source = str(value or "").replace("\\", "/")
    marker = "/marlowe-org-tracked.20260902-2135/"
    if marker in source:
        source = source.split(marker, 1)[1]
    source = source.lstrip("./")
    if not source:
        return None
    candidate = source.split("/", 1)[0]
    return candidate if (REPO_ROOT / "repos" / candidate).is_dir() else None


def annotate_repositories(graph) -> None:
    memberships: dict[str, set[str]] = {str(node): set() for node in graph.nodes}
    for node, attrs in graph.nodes(data=True):
        repo = repo_from_source(attrs.get("source_file"))
        if repo:
            memberships[str(node)].add(repo)
    for source, target, attrs in graph.edges(data=True):
        repo = repo_from_source(attrs.get("source_file"))
        if repo:
            origin = str(attrs.get("_src", source))
            memberships.setdefault(origin, set()).add(repo)
    for node, repos in memberships.items():
        if repos:
            graph.nodes[node]["repository"] = sorted(repos)[0]
            if len(repos) > 1:
                graph.nodes[node]["repositories"] = sorted(repos)


def repository_records() -> list[dict[str, str]]:
    lock_path = REPO_ROOT / "evidence" / "repository-locks-2026-09-02.tsv"
    with lock_path.open(encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def add_repository_layer(graph) -> None:
    records = repository_records()
    org_id = "marlowe_lang_organization"
    graph.add_node(
        org_id,
        label="marlowe-lang GitHub organization",
        file_type="concept",
        source_file="evidence/repository-locks-2026-09-02.tsv",
        semantic_status="repository inventory root",
    )
    repo_ids: dict[str, str] = {}
    for record in records:
        name = record["repository"]
        node_id = "repository_" + re.sub(r"[^a-z0-9]+", "_", name.lower()).strip("_")
        repo_ids[name] = node_id
        graph.add_node(
            node_id,
            label=f"Repository: {name}",
            file_type="concept",
            source_file=f"repos/{name}",
            repository=name,
            remote=record["remote"],
            branch=record["branch"],
            commit=record["commit"],
            commit_date=record["commit_date"],
            semantic_status="pinned repository metadata",
        )
        graph.add_edge(
            org_id,
            node_id,
            relation="contains_repository",
            confidence="EXTRACTED",
            source_file="evidence/repository-locks-2026-09-02.tsv",
            weight=1.0,
            _src=org_id,
            _tgt=node_id,
        )

    dependencies: dict[tuple[str, str], int] = {}
    for source, target, attrs in list(graph.edges(data=True)):
        directed_source = str(attrs.get("_src", source))
        directed_target = str(attrs.get("_tgt", target))
        source_repo = graph.nodes[directed_source].get("repository")
        target_repo = graph.nodes[directed_target].get("repository")
        if source_repo and target_repo and source_repo != target_repo:
            dependencies[(str(source_repo), str(target_repo))] = dependencies.get(
                (str(source_repo), str(target_repo)), 0
            ) + 1
    for (source_repo, target_repo), references in sorted(dependencies.items()):
        graph.add_edge(
            repo_ids[source_repo],
            repo_ids[target_repo],
            relation="cross_repository_reference",
            confidence="EXTRACTED",
            source_file="graphs/marlowe-org-full/graphify-out/graph.json",
            weight=float(references),
            references=references,
            _src=repo_ids[source_repo],
            _tgt=repo_ids[target_repo],
        )


def add_online_source_layer(graph) -> None:
    groups = (
        (
            "marlowe_live_documentation_corpus",
            "Live Marlowe Documentation Corpus",
            "live-docs/",
            "evidence/marlowe-docs-live-acquisition-2026-09-02.json",
        ),
        (
            "marlowe_official_entry_point_corpus",
            "Official Marlowe Entry-Point Corpus",
            "external-sources/",
            "raw/sources/",
        ),
        (
            "defiformal_taxonomy_research_report",
            "DeFiFormal Taxonomy Research Report",
            "user-research/",
            "raw/sources/reclassifying-defi-taxonomy-report-2026-09-02.pdf.receipt.json",
        ),
    )
    for root_id, label, prefix, source_file in groups:
        graph.add_node(
            root_id,
            label=label,
            file_type="document",
            source_file=source_file,
            semantic_status="Scrapling-acquired corpus root",
        )
        files: dict[str, list[str]] = {}
        for node, attrs in graph.nodes(data=True):
            path = str(attrs.get("source_file", ""))
            if path.startswith(prefix):
                files.setdefault(path, []).append(str(node))
        for path, nodes in sorted(files.items()):
            document_nodes = [
                node for node in nodes if graph.nodes[node].get("file_type") == "document"
            ]
            target = sorted(document_nodes or nodes)[0]
            graph.add_edge(
                root_id,
                target,
                relation="indexes_source",
                confidence="EXTRACTED",
                source_file=source_file,
                weight=1.0,
                _src=root_id,
                _tgt=target,
            )


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    merged: dict[str, object] = {
        "nodes": [],
        "edges": [],
        "hyperedges": [],
        "input_tokens": 0,
        "output_tokens": 0,
    }
    input_hashes: dict[str, str] = {}
    for path, origin in INPUTS:
        payload = load_and_stamp(path, origin)
        input_hashes[path.name] = hashlib.sha256(path.read_bytes()).hexdigest()
        for key in ("nodes", "edges", "hyperedges"):
            merged[key].extend(payload.get(key, []))
        merged["input_tokens"] += int(payload.get("input_tokens", 0))
        merged["output_tokens"] += int(payload.get("output_tokens", 0))

    extraction_path = OUTPUT / ".graphify_extract.json"
    extraction_path.write_text(json.dumps(merged, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    diagnostic = diagnose_extraction(
        merged,
        directed=False,
        root=SCAN_ROOT,
        extract_path=extraction_path,
    )
    (OUTPUT / "DIAGNOSTICS.md").write_text(
        format_diagnostic_report(diagnostic) + "\n",
        encoding="utf-8",
    )

    graph = build_from_json(merged, directed=False, root=SCAN_ROOT)
    annotate_repositories(graph)
    add_repository_layer(graph)
    add_online_source_layer(graph)
    communities = cluster(graph)
    cohesion = score_all(graph, communities)
    labels = label_communities_by_hub(graph, communities)
    analysis_graph = graph.copy()
    analysis_noise_labels = {
        "Button()",
        "ICON_SIZES",
        "PAGES",
        "SIZE",
        "compilerOptions",
        "isEmpty()",
    }
    metadata_nodes = {
        node
        for node in analysis_graph.nodes
        if str(node).startswith("repository_")
        or str(analysis_graph.nodes[node].get("label", "")) in analysis_noise_labels
        or node
        in {
            "marlowe_lang_organization",
            "marlowe_live_documentation_corpus",
            "marlowe_official_entry_point_corpus",
            "defiformal_taxonomy_research_report",
        }
    }
    analysis_graph.remove_nodes_from(metadata_nodes)
    hubs = god_nodes(analysis_graph, top_n=20)
    surprises = surprising_connections(analysis_graph, communities, top_n=15)
    questions = suggest_questions(analysis_graph, communities, labels, top_n=12)

    lock_hash = hashlib.sha256((REPO_ROOT / "evidence" / "repository-locks-2026-09-02.tsv").read_bytes()).hexdigest()
    build_id = f"marlowe-org-36@locks-sha256:{lock_hash}"
    graph_path = OUTPUT / "graph.json"
    to_json(
        graph,
        communities,
        str(graph_path),
        force=True,
        built_at_commit=build_id,
        community_labels=labels,
    )

    detection = json.loads((GRAPH_ROOT / ".graphify_detect.json").read_text(encoding="utf-8"))
    report = generate(
        graph,
        communities,
        cohesion,
        labels,
        hubs,
        surprises,
        detection,
        {"input": merged["input_tokens"], "output": merged["output_tokens"]},
        str(SCAN_ROOT),
        suggested_questions=questions,
        min_community_size=3,
        built_at_commit=lock_hash,
    )
    (OUTPUT / "GRAPH_REPORT.md").write_text(report, encoding="utf-8")
    (OUTPUT / ".graphify_labels.json").write_text(
        json.dumps({str(key): value for key, value in labels.items()}, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    analysis = {
        "build_id": build_id,
        "tool_versions": {"graphify_api": version("graphifyy")},
        "input_hashes": input_hashes,
        "raw_counts": {key: len(merged[key]) for key in ("nodes", "edges", "hyperedges")},
        "built_counts": {
            "nodes": graph.number_of_nodes(),
            "edges": graph.number_of_edges(),
            "hyperedges": len(graph.graph.get("hyperedges", [])),
            "communities": len(communities),
        },
        "repository_coverage": sorted(
            {attrs["repository"] for _, attrs in graph.nodes(data=True) if attrs.get("repository")}
        ),
        "god_nodes": hubs,
        "surprising_connections": surprises,
        "suggested_questions": questions,
        "cohesion_scores": {str(key): value for key, value in cohesion.items()},
        "community_labels": {str(key): value for key, value in labels.items()},
        "diagnostic": diagnostic,
        "token_cost": {
            "input_tokens": merged["input_tokens"],
            "output_tokens": merged["output_tokens"],
        },
    }
    (OUTPUT / ".graphify_analysis.json").write_text(
        json.dumps(analysis, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(json.dumps(analysis["built_counts"], sort_keys=True))
    print(json.dumps({"repository_coverage": len(analysis["repository_coverage"])}, sort_keys=True))


if __name__ == "__main__":
    main()
