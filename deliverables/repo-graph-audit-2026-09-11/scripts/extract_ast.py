#!/usr/bin/env python3
"""Deterministic AST extraction for all detected code files.

Writes graphify-out/.graphify_ast.json under the audit deliverable. Reuses
copied AST cache when hashes match. Does not modify product code.
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
AUDIT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
GOUT = AUDIT / "graphify-out"
DETECT = GOUT / ".graphify_detect.json"

# Keep cache under the audit graphify-out, not the preserved repo graphify-out.
os.environ["GRAPHIFY_OUT"] = "graphify-out"


def main() -> int:
    detect = json.loads(DETECT.read_text(encoding="utf-8"))
    from graphify.extract import collect_files, extract

    MAX_BYTES = 1_000_000
    SKIP_NAME_FRAGMENTS = (
        "chain-spec",
        "compiled.json",
        "kast.json",
        "source-collection.json",
        "marlowe-org-code-",
        "midnightntwrk-key-",
        ".graphify_code.json",
        "lace_wallet/js/",
    )
    skipped_large: list[dict] = []
    code_files: list[Path] = []
    for f in detect.get("files", {}).get("code", []):
        p = Path(f)
        paths = collect_files(p) if p.is_dir() else ([p] if p.is_file() else [])
        for path in paths:
            try:
                sz = path.stat().st_size
            except OSError:
                continue
            rel = str(path)
            if sz > MAX_BYTES or any(frag in rel for frag in SKIP_NAME_FRAGMENTS):
                skipped_large.append({"path": rel, "bytes": sz, "reason": "generated-or-oversize-not-AST-parsed"})
                continue
            code_files.append(path)
    (GOUT / ".graphify_ast_skipped_large.json").write_text(
        json.dumps({"count": len(skipped_large), "files": skipped_large}, indent=2),
        encoding="utf-8",
    )
    print(f"AST inputs: {len(code_files)} files (skipped {len(skipped_large)} oversize/generated)", flush=True)
    if not code_files:
        GOUT.joinpath(".graphify_ast.json").write_text(
            json.dumps({"nodes": [], "edges": [], "input_tokens": 0, "output_tokens": 0, "extractor": "ast-empty"}),
            encoding="utf-8",
        )
        print("No code files")
        return 0
    result = extract(
        code_files,
        cache_root=AUDIT,
        root=REPO,
        parallel=True,
        max_workers=8,
    )
    result["extractor"] = "graphify.extract.AST"
    result["model"] = None
    result["confidence_policy"] = "deterministic-AST"
    result["code_file_count"] = len(code_files)
    GOUT.joinpath(".graphify_ast.json").write_text(
        json.dumps(result, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )
    print(f"AST: {len(result.get('nodes', []))} nodes, {len(result.get('edges', []))} edges", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
