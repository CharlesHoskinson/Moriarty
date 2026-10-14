#!/usr/bin/env python3
"""Merge luna-experiments.json into the full graph.json without Graphify rebuild."""
from __future__ import annotations

import datetime
import hashlib
import json
import os
import shutil
from pathlib import Path

ROOT = Path("/home/charl/moriarty-eutxo-validation-2026-10-12")
OUT = ROOT / "graphify-out"
FULL = OUT / "graph.json"
LUNA = OUT / "luna-experiments.json"
BACKUP = OUT / "graph.json.pre-luna-02-merge"
TMP = OUT / "graph.json.merge-tmp"
RECEIPT = OUT / "LUNA-02-MERGE-VERIFICATION.json"
FOCUS = OUT / "knowledge-focus.json"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def edge_key(e: dict) -> tuple:
    return (
        e.get("source"),
        e.get("target"),
        e.get("relation"),
        e.get("source_location"),
        e.get("source_file"),
    )


def main() -> None:
    if not BACKUP.exists():
        print("copying prior full graph to", BACKUP)
        shutil.copy2(FULL, BACKUP)
    prior_sha = sha256_file(BACKUP)
    luna = json.loads(LUNA.read_text())
    full = json.loads(FULL.read_text())
    nodes = full["nodes"]
    links = full.get("links") or full.get("edges") or []
    prior_link_count = len(links)
    by_id = {n["id"]: n for n in nodes}
    original_ids = set(by_id)
    added_nodes = 0
    merged_attrs = 0
    attr_conflicts = []
    for n in luna["nodes"]:
        nid = n["id"]
        if nid in by_id:
            existing = by_id[nid]
            for k, v in n.items():
                if k == "id":
                    continue
                if k not in existing:
                    existing[k] = v
                    merged_attrs += 1
                elif existing[k] != v and k not in ("community", "norm_label", "_origin"):
                    attr_conflicts.append({"id": nid, "key": k})
        else:
            node = dict(n)
            node.setdefault("_origin", "luna-experiments-02")
            node.setdefault("norm_label", str(n.get("label", "")).lower())
            node.setdefault("community", 6164)
            nodes.append(node)
            by_id[nid] = node
            added_nodes += 1
    existing_keys = {edge_key(e) for e in links}
    added_edges = 0
    skipped_missing = 0
    for e in luna.get("edges") or []:
        if e.get("source") not in by_id or e.get("target") not in by_id:
            skipped_missing += 1
            continue
        k = edge_key(e)
        if k in existing_keys:
            continue
        link = {
            "source": e["source"],
            "target": e["target"],
            "relation": e.get("relation"),
            "confidence": e.get("confidence", "EXTRACTED"),
            "source_file": e.get("source_file"),
            "source_location": e.get("source_location"),
            "_origin": "luna-experiments-02",
            "weight": 1,
        }
        if "confidence_score" in e:
            link["confidence_score"] = e["confidence_score"]
        links.append(link)
        existing_keys.add(k)
        added_edges += 1
    dangling = [
        e
        for e in links
        if e.get("source") not in by_id or e.get("target") not in by_id
    ]
    if dangling:
        raise SystemExit(f"dangling edges after merge: {len(dangling)}")
    full["nodes"] = nodes
    full["links"] = links
    TMP.write_text(json.dumps(full, ensure_ascii=False))
    os.replace(TMP, FULL)
    focus_nodes = [n for n in nodes if n["id"] not in original_ids or n.get("_origin") == "luna-experiments-02" or str(n.get("id", "")).startswith(("pattern_", "designer_", "01_", "02_", "03_", "04_", "05_"))]
    luna_ids = {n["id"] for n in luna["nodes"]}
    focus_nodes = [by_id[i] for i in luna_ids]
    focus_links = [e for e in links if e.get("source") in luna_ids and e.get("target") in luna_ids]
    FOCUS.write_text(json.dumps({"directed": True, "multigraph": True, "graph": {"layer": "luna-experiments-02"}, "nodes": focus_nodes, "links": focus_links}, ensure_ascii=False, indent=2) + "\n")
    receipt = {
        "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "prior_graph_copy": str(BACKUP),
        "prior_graph_sha256": prior_sha,
        "merged_graph_sha256": sha256_file(FULL),
        "prior_nodes": len(original_ids),
        "prior_links": prior_link_count,
        "nodes": len(nodes),
        "links": len(links),
        "luna_nodes_input": len(luna["nodes"]),
        "luna_edges_input": len(luna.get("edges") or []),
        "added_nodes": added_nodes,
        "added_edges": added_edges,
        "merged_extra_attrs": merged_attrs,
        "attr_conflicts": attr_conflicts[:20],
        "attr_conflict_count": len(attr_conflicts),
        "dangling_export_edges": 0,
        "skipped_missing_endpoints": skipped_missing,
        "qualification": "Exact node/edge merge of luna-experiments into full inventory graph. Luna authorship preserved. Not a full-body semantic review.",
        "semantic_model_requested": "gpt-6-luna",
        "author_this_merge": "grok-4.6",
    }
    RECEIPT.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({k: receipt[k] for k in ("nodes", "links", "added_nodes", "added_edges", "merged_extra_attrs", "attr_conflict_count")}, indent=2))


if __name__ == "__main__":
    main()
