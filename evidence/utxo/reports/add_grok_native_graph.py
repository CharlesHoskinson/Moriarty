#!/usr/bin/env python3
"""Merge grok-native-evidence.json into full graph.json without rewriting Luna nodes."""
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
GROK = OUT / "grok-native-evidence.json"
BACKUP = OUT / "graph.json.pre-grok-native-evidence"
TMP = OUT / "graph.json.grok-tmp"
RECEIPT = OUT / "GROK-NATIVE-GRAPH-VERIFICATION.json"
FOCUS = OUT / "knowledge-focus.json"
LUNA = OUT / "luna-experiments.json"


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
    closure_backup = OUT / "graph.json.pre-grok-native-closure"
    if not closure_backup.exists():
        print("copying current full graph to", closure_backup)
        shutil.copy2(FULL, closure_backup)
    if not BACKUP.exists():
        print("copying prior full graph to", BACKUP)
        shutil.copy2(FULL, BACKUP)
    original_backup_sha = sha256_file(BACKUP)
    prior_sha = sha256_file(FULL)
    grok = json.loads(GROK.read_text())
    luna = json.loads(LUNA.read_text())
    full = json.loads(FULL.read_text())
    nodes = full["nodes"]
    links = full.get("links") or full.get("edges") or []
    prior_link_count = len(links)
    by_id = {n["id"]: n for n in nodes}
    original_ids = set(by_id)
    added_nodes = 0
    merged_attrs = 0
    refreshed_grok = 0
    attr_conflicts = []
    for n in grok["nodes"]:
        nid = n["id"]
        if nid in by_id:
            existing = by_id[nid]
            grok_owned = (
                existing.get("coverage_layer") == "grok_native_evidence"
                and existing.get("author") == "grok-4.6"
            )
            for k, v in n.items():
                if k == "id":
                    continue
                if grok_owned and k not in ("community", "_origin"):
                    if existing.get(k) != v:
                        existing[k] = v
                        refreshed_grok += 1
                    continue
                if k not in existing:
                    existing[k] = v
                    merged_attrs += 1
                elif existing[k] != v and k not in ("community", "norm_label", "_origin"):
                    attr_conflicts.append({"id": nid, "key": k})
        else:
            node = dict(n)
            node.setdefault("_origin", "grok-native-final-20261012")
            node.setdefault("norm_label", str(n.get("label", "")).lower())
            node.setdefault("community", 6165)
            node.setdefault("author", "grok-4.6")
            nodes.append(node)
            by_id[nid] = node
            added_nodes += 1
    existing_keys = {edge_key(e) for e in links}
    added_edges = 0
    skipped_missing = 0
    for e in grok.get("edges") or []:
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
            "_origin": "grok-native-final-20261012",
            "weight": 1,
        }
        links.append(link)
        existing_keys.add(k)
        added_edges += 1
    dangling = [
        e
        for e in links
        if e.get("source") not in by_id or e.get("target") not in by_id
    ]
    if dangling:
        raise SystemExit(f"dangling edges after grok merge: {len(dangling)}")
    full["nodes"] = nodes
    full["links"] = links
    TMP.write_text(json.dumps(full, ensure_ascii=False))
    os.replace(TMP, FULL)

    luna_ids = {n["id"] for n in luna["nodes"]}
    grok_ids = {n["id"] for n in grok["nodes"]}
    focus_ids = luna_ids | grok_ids
    focus_nodes = [by_id[i] for i in focus_ids if i in by_id]
    focus_links = [
        e for e in links if e.get("source") in focus_ids and e.get("target") in focus_ids
    ]
    FOCUS.write_text(
        json.dumps(
            {
                "directed": True,
                "multigraph": True,
                "graph": {
                    "layer": "luna-experiments-02+grok-native-final-20261012",
                    "luna_authorship_preserved": True,
                    "grok_author": "grok-4.6",
                    "qualification": "Luna did not review Grok final native facts. Old candidate nodes remain historical.",
                },
                "nodes": focus_nodes,
                "links": focus_links,
            },
            ensure_ascii=False,
            indent=2,
        )
        + "\n"
    )
    receipt = {
        "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "prior_graph_copy": str(BACKUP),
        "prior_graph_sha256": original_backup_sha,
        "merged_graph_sha256": sha256_file(FULL),
        "prior_nodes": len(original_ids),
        "prior_links": prior_link_count,
        "nodes": len(nodes),
        "links": len(links),
        "grok_nodes_input": len(grok["nodes"]),
        "grok_edges_input": len(grok.get("edges") or []),
        "added_nodes": added_nodes,
        "added_edges": added_edges,
        "refreshed_grok_fields": refreshed_grok,
        "merged_extra_attrs": merged_attrs,
        "prior_graph_sha256_this_run": prior_sha,
        "closure_backup": str(closure_backup),
        "attr_conflicts": attr_conflicts[:20],
        "attr_conflict_count": len(attr_conflicts),
        "dangling_export_edges": 0,
        "skipped_missing_endpoints": skipped_missing,
        "focus_nodes": len(focus_nodes),
        "focus_links": len(focus_links),
        "qualification": "Grok native evidence nodes added. Luna node IDs/path/hash/attrs preserved. Not a full-body semantic review. Luna did not review these native facts.",
        "semantic_model_requested": "gpt-6-luna",
        "author_this_merge": "grok-4.6",
    }
    RECEIPT.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({k: receipt[k] for k in ("nodes", "links", "added_nodes", "added_edges", "attr_conflict_count", "focus_nodes", "focus_links")}, indent=2))


if __name__ == "__main__":
    main()
