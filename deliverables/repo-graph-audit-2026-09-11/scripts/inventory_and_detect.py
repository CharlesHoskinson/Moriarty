#!/usr/bin/env python3
"""Inventory the entire Moriarty checkout, then run graphify detect.

Writes artifacts under deliverables/repo-graph-audit-2026-09-11/. Does not
modify product code, wiki, hooks, or raw receipts.
"""
from __future__ import annotations

import json
import os
import stat
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path

REPO = Path("/home/charl/Moriarty").resolve()
OUT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
GRAPHIFY_OUT = OUT / "graphify-out"
PYTHON = Path("/home/charl/.local/share/uv/tools/graphifyy/bin/python3")

# Extra gitignore-style excludes passed to detect(). These win over .gitignore
# un-ignore rules. Built-in graphify _SKIP_DIRS already prune .git, node_modules,
# __pycache__, graphify-out, venv, dist, build, target, etc.
EXTRA_EXCLUDES = [
    # Secrets / credentials / private witnesses
    "*.pem",
    "*.key",
    "*.p12",
    "*.pfx",
    "*.cookies.json",
    "*.local.md",
    ".env",
    ".env.*",
    "id_rsa",
    "id_ed25519",
    "*.wallet.json",
    "**/witness.json",
    "**/private-witness*",
    "**/*-witness.json",
    "**/.midnight-wallet/**",
    "**/secrets/**",
    "**/credentials/**",
    # Build / dependency / acquisition caches not always in _SKIP_DIRS
    ".scrapling/",
    "downloads/",
    ".vault-meta/",
    ".codex/",
    ".foreman/*.db",
    ".foreman/*.db-*",
    # This audit's backup of the prior graph (do not re-ingest)
    "deliverables/repo-graph-audit-2026-09-11/graphify-out-prior/",
    "deliverables/repo-graph-audit-2026-09-11/graphify-out/",
    # Git internals (also pruned by name, recorded explicitly)
    ".git/",
]

# Inventory-only skip: never even count contents of these as readable corpus.
INVENTORY_PRUNE_NAMES = {
    ".git",
    "node_modules",
    "__pycache__",
    ".venv",
    "venv",
    "site-packages",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".tox",
    ".nox",
    ".next",
    ".turbo",
    ".cache",
    ".parcel-cache",
    ".obsidian",
    ".scrapling",
    "graphify-out",
    ".graphify",
}

SOURCE_FAMILIES = {
    "code": "product and experiment source",
    "docs": "repository documentation",
    "specifications": "OpenSpec / sprint contracts",
    "evidence": "retained results and reviews",
    "history_navigation": "graphs/, repos/ pinned clones, graphify snapshots",
    "research_raw": "raw/ immutable receipts",
    "deliverables": "dated work packets",
    "wiki": "canonical wiki pages",
    "plugins": "moriarty-dev plugin",
    "site": "developer site",
    "tests": "repository tests",
    "inbox": "unprocessed intake",
    "corpus": "captured document corpus",
    "unsupported": "binary/cache/unknown (inventory only)",
}


def classify_top(rel: str) -> str:
    top = rel.split("/", 1)[0] if rel else "(root)"
    mapping = {
        "experiments": "code",
        "plugins": "plugins",
        "scripts": "code",
        "site": "site",
        "tests": "tests",
        "docs": "docs",
        "openspec": "specifications",
        "evidence": "evidence",
        "deliverables": "deliverables",
        "wiki": "wiki",
        "raw": "research_raw",
        "graphs": "history_navigation",
        "repos": "history_navigation",
        "inbox": "inbox",
        "corpus": "corpus",
        "reports": "evidence",
    }
    if top in mapping:
        return mapping[top]
    if rel.endswith((".md", ".txt")) and "/" not in rel:
        return "docs"
    if rel.endswith((".py", ".ts", ".mjs", ".js", ".rs", ".compact")):
        return "code"
    return "unsupported"


def walk_inventory(root: Path) -> dict:
    by_family = defaultdict(lambda: {"files": 0, "bytes": 0, "ext": Counter()})
    by_top = defaultdict(lambda: {"files": 0, "bytes": 0})
    pruned = []
    errors = []
    sensitive_hits = []
    total_files = 0
    total_bytes = 0
    # Do not follow symlinks.
    for dirpath, dirnames, filenames in os.walk(root, followlinks=False, onerror=lambda e: errors.append(str(e))):
        p = Path(dirpath)
        rel_dir = p.relative_to(root).as_posix() if p != root else ""
        # Prune
        keep = []
        for d in list(dirnames):
            if d in INVENTORY_PRUNE_NAMES or d == "graphify-out-prior":
                pruned.append(str((p / d).relative_to(root)) + "/")
                continue
            keep.append(d)
        dirnames[:] = keep
        for fname in filenames:
            fp = p / fname
            try:
                st = fp.lstat()
            except OSError as e:
                errors.append(f"{fp}: {e}")
                continue
            if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode):
                continue
            rel = fp.relative_to(root).as_posix()
            # Sensitive filename heuristics (inventory flag, not ingest)
            low = fname.lower()
            if any(
                s in low
                for s in (
                    ".pem",
                    ".p12",
                    ".pfx",
                    "id_rsa",
                    "id_ed25519",
                    "cookies.json",
                    ".local.md",
                )
            ) or low in {".env", "credentials.json", "secrets.json"}:
                sensitive_hits.append(rel)
                continue
            family = classify_top(rel)
            top = rel.split("/", 1)[0] if "/" in rel else "(root)"
            ext = fp.suffix.lower() or "(none)"
            by_family[family]["files"] += 1
            by_family[family]["bytes"] += st.st_size
            by_family[family]["ext"][ext] += 1
            by_top[top]["files"] += 1
            by_top[top]["bytes"] += st.st_size
            total_files += 1
            total_bytes += st.st_size
    return {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "root": str(root),
        "total_files": total_files,
        "total_bytes": total_bytes,
        "families": {
            k: {
                "files": v["files"],
                "bytes": v["bytes"],
                "top_extensions": v["ext"].most_common(12),
                "description": SOURCE_FAMILIES.get(k, ""),
            }
            for k, v in sorted(by_family.items(), key=lambda kv: -kv[1]["files"])
        },
        "top_level": {
            k: dict(v) for k, v in sorted(by_top.items(), key=lambda kv: -kv[1]["files"])
        },
        "pruned_directories": pruned,
        "sensitive_filename_skipped": sensitive_hits,
        "walk_errors": errors[:50],
        "walk_error_count": len(errors),
    }


def main() -> int:
    GRAPHIFY_OUT.mkdir(parents=True, exist_ok=True)
    (GRAPHIFY_OUT / ".graphify_python").write_text(str(PYTHON), encoding="utf-8")
    (GRAPHIFY_OUT / ".graphify_root").write_text(str(REPO), encoding="utf-8")

    print("=== inventory walk ===", flush=True)
    inv = walk_inventory(REPO)
    (OUT / "inventory.json").write_text(json.dumps(inv, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"Inventory: {inv['total_files']} files, {inv['total_bytes']} bytes", flush=True)
    for fam, data in inv["families"].items():
        print(f"  {fam:22s} {data['files']:7d} files", flush=True)
    print("Top-level dirs:", flush=True)
    for i, (name, data) in enumerate(list(inv["top_level"].items())[:12]):
        print(f"  {name:40s} {data['files']:7d}", flush=True)

    sys.path.insert(0, str(PYTHON.parent.parent / "lib" / "python3.13" / "site-packages"))
    from graphify.detect import detect

    print("=== graphify detect (gitignore=False, extra_excludes) ===", flush=True)
    result = detect(
        REPO,
        extra_excludes=EXTRA_EXCLUDES,
        cache_root=GRAPHIFY_OUT,
        gitignore=False,
        follow_symlinks=False,
    )
    detect_path = GRAPHIFY_OUT / ".graphify_detect.json"
    detect_path.write_text(json.dumps(result, ensure_ascii=False), encoding="utf-8")
    print(f"Detected {result.get('total_files')} files, words={result.get('total_words')}", flush=True)
    files = result.get("files") or {}
    for cat, lst in files.items():
        print(f"  {cat}: {len(lst)}", flush=True)
    skipped = result.get("skipped_sensitive") or []
    print(f"skipped_sensitive: {len(skipped)}", flush=True)
    ignored = result.get("ignored") or []
    print(f"ignored: {len(ignored)}", flush=True)
    pruned = result.get("pruned_noise") or result.get("pruned") or []
    print(f"pruned: {len(pruned)}", flush=True)
    unclassified = result.get("unclassified") or []
    print(f"unclassified: {len(unclassified)}", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
