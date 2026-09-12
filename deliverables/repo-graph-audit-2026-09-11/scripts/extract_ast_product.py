#!/usr/bin/env python3
"""AST extract product-owned source only.

Full-repo extract hung on generated JSON/JS dumps. This pass covers Moriarty
product trees. Remaining detected code is inventoried, not AST-parsed.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
AUDIT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
GOUT = AUDIT / "graphify-out"
os.environ["GRAPHIFY_OUT"] = "graphify-out"

PRODUCT_PREFIXES = (
    "experiments/",
    "plugins/",
    "scripts/",
    "site/",
    "tests/",
    "openspec/",
    "docs/",
    "wiki/",
)
DELIVERABLE_CODE_EXT = {".py", ".ts", ".tsx", ".js", ".mjs", ".cjs", ".rs", ".go"}
MAX_BYTES = 1_000_000


def main() -> int:
    detect = json.loads((GOUT / ".graphify_detect.json").read_text(encoding="utf-8"))
    from graphify.extract import extract

    kept: list[Path] = []
    skipped: list[dict] = []
    for f in detect.get("files", {}).get("code", []):
        p = Path(f)
        if not p.is_file():
            continue
        try:
            rel = p.resolve().relative_to(REPO).as_posix()
            sz = p.stat().st_size
        except Exception:
            continue
        ok = False
        if rel.startswith(PRODUCT_PREFIXES) and sz <= MAX_BYTES:
            ok = True
        elif rel.startswith("deliverables/") and p.suffix.lower() in DELIVERABLE_CODE_EXT and sz <= MAX_BYTES:
            ok = True
        if ok:
            kept.append(p)
        else:
            skipped.append({"path": rel, "bytes": sz, "reason": "not-product-owned-or-oversize"})
    (GOUT / ".graphify_ast_scope.json").write_text(
        json.dumps({"kept": len(kept), "skipped": len(skipped), "skipped_sample": skipped[:50]}, indent=2),
        encoding="utf-8",
    )
    print(f"AST product-owned: {len(kept)} files (skipped {len(skipped)} detected code files)", flush=True)
    result = extract(kept, cache_root=AUDIT, root=REPO, parallel=True, max_workers=8)
    result["extractor"] = "graphify.extract.AST"
    result["model"] = None
    result["confidence_policy"] = "deterministic-AST"
    result["code_file_count"] = len(kept)
    result["detected_code_skipped"] = len(skipped)
    (GOUT / ".graphify_ast.json").write_text(json.dumps(result, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"AST: {len(result.get('nodes', []))} nodes, {len(result.get('edges', []))} edges", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
