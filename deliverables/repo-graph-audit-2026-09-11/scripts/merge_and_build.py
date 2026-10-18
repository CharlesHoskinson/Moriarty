#!/usr/bin/env python3
"""Merge AST + semantic chunks, build graph, health-check, export JSON.

Community labels and HTML are applied by a later step. Writes under
deliverables/repo-graph-audit-2026-09-11/graphify-out only.
"""
from __future__ import annotations

import glob
import json
import os
import sys
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
AUDIT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
GOUT = AUDIT / "graphify-out"
os.environ["GRAPHIFY_OUT"] = "graphify-out"


def load(path: Path, default=None):
    if not path.exists():
        return default
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    ast = load(GOUT / ".graphify_ast.json")
    if not ast:
        print("ERROR: missing AST extraction", file=sys.stderr)
        return 2

    chunks = sorted(glob.glob(str(GOUT / ".graphify_chunk_*.json")))
    all_nodes, all_edges, all_hyper = [], [], []
    total_in = total_out = 0
    valid_chunks = []
    failed_chunks = []
    for c in chunks:
        try:
            d = json.loads(Path(c).read_text(encoding="utf-8"))
        except Exception as e:
            failed_chunks.append({"path": c, "error": f"json:{e}"})
            continue
        if not isinstance(d, dict) or "nodes" not in d or "edges" not in d:
            failed_chunks.append({"path": c, "error": "schema"})
            continue
        all_nodes += d.get("nodes") or []
        all_edges += d.get("edges") or []
        all_hyper += d.get("hyperedges") or []
        total_in += int(d.get("input_tokens") or 0)
        total_out += int(d.get("output_tokens") or 0)
        valid_chunks.append(c)

    new_sem = {
        "nodes": all_nodes,
        "edges": all_edges,
        "hyperedges": all_hyper,
        "input_tokens": total_in,
        "output_tokens": total_out,
        "extractor": "grok-4.6-host-subagents",
        "model": "grok-4.6",
        "gemini_used": False,
        "valid_chunks": len(valid_chunks),
        "failed_chunks": failed_chunks,
    }
    (GOUT / ".graphify_semantic_new.json").write_text(
        json.dumps(new_sem, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Merged {len(valid_chunks)} chunks ({len(failed_chunks)} failed): {total_in:,} in / {total_out:,} out")

    cached = load(GOUT / ".graphify_cached.json") or {"nodes": [], "edges": [], "hyperedges": []}
    seen = set()
    deduped = []
    for n in (cached.get("nodes") or []) + new_sem["nodes"]:
        nid = n.get("id")
        if nid not in seen:
            seen.add(nid)
            deduped.append(n)
    sem = {
        "nodes": deduped,
        "edges": (cached.get("edges") or []) + new_sem["edges"],
        "hyperedges": (cached.get("hyperedges") or []) + new_sem["hyperedges"],
        "input_tokens": total_in,
        "output_tokens": total_out,
        "extractor": "grok-4.6 + semantic-cache",
        "model": "grok-4.6",
        "cached_nodes": len(cached.get("nodes") or []),
        "new_nodes": len(new_sem["nodes"]),
    }
    (GOUT / ".graphify_semantic.json").write_text(
        json.dumps(sem, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(
        f"Semantic: {len(deduped)} nodes, {len(sem['edges'])} edges "
        f"({sem['cached_nodes']} cached, {sem['new_nodes']} new)"
    )

    # Part C merge
    extra = load(GOUT / ".graphify_unsupported_file_nodes.json") or {"nodes": [], "edges": []}
    seen = {n["id"] for n in ast.get("nodes") or []}
    merged_nodes = list(ast.get("nodes") or [])
    for n in (extra.get("nodes") or []) + sem["nodes"]:
        if n.get("id") not in seen:
            merged_nodes.append(n)
            seen.add(n.get("id"))
    merged = {
        "nodes": merged_nodes,
        "edges": (ast.get("edges") or []) + sem["edges"],
        "hyperedges": sem.get("hyperedges") or [],
        "input_tokens": sem.get("input_tokens", 0),
        "output_tokens": sem.get("output_tokens", 0),
        "ast_nodes": len(ast.get("nodes") or []),
        "semantic_nodes": len(sem["nodes"]),
        "extractor_layers": {
            "ast": "graphify.extract deterministic tree-sitter",
            "semantic_new": "grok-4.6",
            "semantic_cache": "prior graphify semantic cache (mixed historical backends)",
        },
    }
    (GOUT / ".graphify_extract.json").write_text(
        json.dumps(merged, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(f"Merged extract: {len(merged_nodes)} nodes, {len(merged['edges'])} edges")

    from graphify.build import build_from_json
    from graphify.cluster import cluster, score_all
    from graphify.analyze import god_nodes, surprising_connections, suggest_questions
    from graphify.report import generate
    from graphify.export import to_json

    detection = load(GOUT / ".graphify_detect.json")
    G = build_from_json(merged, root=str(REPO), directed=False)
    if G.number_of_nodes() == 0:
        print("ERROR: Graph is empty - extraction produced no nodes.")
        return 1
    communities = cluster(G)
    cohesion = score_all(G, communities)
    tokens = {"input": merged.get("input_tokens", 0), "output": merged.get("output_tokens", 0)}
    gods = god_nodes(G)
    surprises = surprising_connections(G, communities)
    labels = {cid: "Community " + str(cid) for cid in communities}
    questions = suggest_questions(G, communities, labels)
    wrote = to_json(G, communities, str(GOUT / "graph.json"))
    if not wrote:
        print("ERROR: refused to shrink graph.json")
        return 1
    report = generate(
        G,
        communities,
        cohesion,
        labels,
        gods,
        surprises,
        detection,
        tokens,
        str(REPO),
        suggested_questions=questions,
    )
    (GOUT / "GRAPH_REPORT.md").write_text(report, encoding="utf-8")
    analysis = {
        "communities": {str(k): v for k, v in communities.items()},
        "cohesion": {str(k): v for k, v in cohesion.items()},
        "gods": gods,
        "surprises": surprises,
        "questions": questions,
        "valid_chunks": len(valid_chunks),
        "failed_chunks": failed_chunks,
    }
    (GOUT / ".graphify_analysis.json").write_text(
        json.dumps(analysis, indent=2, ensure_ascii=False), encoding="utf-8"
    )
    print(
        f"Graph: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges, {len(communities)} communities"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
