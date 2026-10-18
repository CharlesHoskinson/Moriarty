#!/usr/bin/env python3
"""Add inventory-only file nodes for detector-unclassified product families.

Graphify AST does not parse .compact / .k / .zkir. This pass records the files
as EXTRACTED file nodes so they are navigable. It does not parse their syntax.
"""
from __future__ import annotations

import json
import re
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
GOUT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11/graphify-out")
EXTS = {".compact", ".k", ".zkir", ".kore"}
PREFIXES = (
    "experiments/",
    "openspec/",
    "plugins/",
    "docs/",
    "wiki/",
    "graphs/k-framework/",
    "graphs/zkir-k/",
)


def nid(rel: str) -> str:
    stem = rel.rsplit(".", 1)[0]
    s = re.sub(r"[^a-z0-9]+", "_", stem.lower()).strip("_")
    return f"{s}_file"


def main() -> int:
    detect = json.loads((GOUT / ".graphify_detect.json").read_text(encoding="utf-8"))
    nodes = []
    for u in detect.get("unclassified") or []:
        rel = str(Path(u).resolve().relative_to(REPO)) if Path(u).is_absolute() else u
        if not rel.startswith(PREFIXES):
            continue
        ext = Path(rel).suffix.lower()
        if ext not in EXTS:
            continue
        nodes.append(
            {
                "id": nid(rel),
                "label": Path(rel).name,
                "file_type": "code",
                "source_file": u,
                "source_location": None,
                "source_url": None,
                "captured_at": None,
                "author": None,
                "contributor": None,
                "extraction_layer": "inventory-only-unsupported-extension",
                "note": "Detector classified this extension as unclassified; no AST parse.",
            }
        )
    out = GOUT / ".graphify_unsupported_file_nodes.json"
    out.write_text(json.dumps({"nodes": nodes, "edges": [], "count": len(nodes)}, indent=2), encoding="utf-8")
    print(f"unsupported product file nodes: {len(nodes)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
