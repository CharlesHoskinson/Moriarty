#!/usr/bin/env python3
"""Check semantic cache and write priority uncached lists for Grok extraction."""
from __future__ import annotations

import json
import os
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
AUDIT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
GOUT = AUDIT / "graphify-out"
SPEC = Path("/home/charl/.agents/skills/graphify/references/extraction-spec.md")
os.environ["GRAPHIFY_OUT"] = "graphify-out"

PRIORITY_PREFIXES = (
    "openspec/",
    "docs/",
    "wiki/",
    "plugins/",
    "experiments/",
    "site/",
    "tests/",
    "scripts/",
    "AGENTS.md",
    "ROADMAP.md",
    "README.md",
    "WIKI_SCHEMA.md",
    "CLAUDE.md",
    "GEMINI.md",
    "LICENSE",
    "pyproject.toml",
    "raw/assignments/",
    "deliverables/afk-live-financial-execution-2026-09-11/",
    "deliverables/pcd-midnight-native-2026-09-11/",
    "deliverables/roadmap-loop-2026-09-10/",
    "deliverables/openspec-roadmap-review-2026-09-10/",
    "deliverables/plugin-execution-focus-2026-09-11/",
    "deliverables/sp05-loan-executor-grok-2026-09-11/",
    "deliverables/sp05-loan-exit-retention-grok-2026-09-10/",
    "deliverables/successor-next-obligation-2026-09-10/",
    "evidence/pcd-midnight-native-2026-09-11/",
    "evidence/midnight-preview-2026-09-07/",
    "evidence/moriarty-completion-program-2026-09-07/",
    "evidence/source-inventory.csv",
)

PRIORITY_NAMES = {
    "RESULT.md",
    "REVIEWED-RESULT.md",
    "ACCEPTED.md",
    "REPORT.md",
    "README.md",
    "SKILL.md",
    "GRAPH_REPORT.md",
    "AUDIT.md",
    "ACTIVE-ROUTING.md",
}


def rel(path: str) -> str:
    p = Path(path)
    try:
        return p.resolve().relative_to(REPO).as_posix()
    except Exception:
        return path


def is_priority(path: str) -> bool:
    r = rel(path)
    if r.startswith(PRIORITY_PREFIXES):
        return True
    name = Path(r).name
    if name in PRIORITY_NAMES and (
        r.startswith("deliverables/") or r.startswith("evidence/") or r.startswith("openspec/")
    ):
        return True
    return False


def main() -> int:
    from graphify.cache import check_semantic_cache

    detect = json.loads((GOUT / ".graphify_detect.json").read_text(encoding="utf-8"))
    all_files = [
        f
        for cat in ("document", "paper", "image")
        for f in detect["files"].get(cat, [])
    ]
    cached_nodes, cached_edges, cached_hyperedges, uncached = check_semantic_cache(
        all_files,
        root=str(REPO),
        prompt_file=str(SPEC),
        cache_root=AUDIT,
    )
    if cached_nodes or cached_edges or cached_hyperedges:
        (GOUT / ".graphify_cached.json").write_text(
            json.dumps(
                {
                    "nodes": cached_nodes,
                    "edges": cached_edges,
                    "hyperedges": cached_hyperedges,
                    "extractor": "semantic-cache",
                    "model": "cached-prior-run",
                },
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
    else:
        (GOUT / ".graphify_cached.json").unlink(missing_ok=True)

    uncached_paths = [u for u in uncached if u]
    (GOUT / ".graphify_uncached.txt").write_text("\n".join(uncached_paths), encoding="utf-8")

    priority = [u for u in uncached_paths if is_priority(u)]
    # Drop images from live Grok extraction unless they are tiny and in docs/openspec
    priority_docs = []
    priority_images = []
    for u in priority:
        suf = Path(u).suffix.lower()
        if suf in {".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"}:
            priority_images.append(u)
        else:
            priority_docs.append(u)
    (GOUT / ".graphify_priority_uncached.txt").write_text("\n".join(priority_docs), encoding="utf-8")
    (GOUT / ".graphify_priority_images.txt").write_text("\n".join(priority_images), encoding="utf-8")

    indexed_only = [u for u in uncached_paths if u not in set(priority_docs)]
    (GOUT / ".graphify_indexed_only.txt").write_text("\n".join(indexed_only), encoding="utf-8")

    summary = {
        "semantic_detected": len(all_files),
        "cache_hits": len(all_files) - len(uncached_paths),
        "uncached": len(uncached_paths),
        "priority_docs_for_grok": len(priority_docs),
        "priority_images_inventory_only": len(priority_images),
        "indexed_only_not_semantically_read": len(indexed_only),
        "cached_nodes": len(cached_nodes),
        "cached_edges": len(cached_edges),
        "cached_hyperedges": len(cached_hyperedges),
        "prompt_file": str(SPEC),
        "model_for_new_extraction": "grok-4.6",
        "gemini_used": False,
    }
    (GOUT / ".graphify_semantic_plan.json").write_text(
        json.dumps(summary, indent=2), encoding="utf-8"
    )
    print(json.dumps(summary, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
