#!/usr/bin/env python3
"""Build a file-level repository map from the existing graphify detect corpus.

Companion to graphify-out/graph.json (semantic + AST). This script does not
parse ASTs, does not run semantic extraction, and does not edit product, wiki,
raw, graphify-out, or AUDIT.md files.

Writes only:
  deliverables/repo-graph-audit-2026-09-11/repository-file-map.json
  deliverables/repo-graph-audit-2026-09-11/FILE_MAP.md
  deliverables/repo-graph-audit-2026-09-11/repository-file-map.html
"""
from __future__ import annotations

import json
import os
import re
import stat
import subprocess
import sys
import time
from collections import Counter, defaultdict
from datetime import datetime, timezone
from fnmatch import fnmatch
from pathlib import Path, PurePosixPath

REPO = Path("/home/charl/Moriarty").resolve()
OUT = Path("/home/charl/Moriarty/deliverables/repo-graph-audit-2026-09-11").resolve()
DETECT_PATH = OUT / "graphify-out" / ".graphify_detect.json"
GRAPHIFY_SITE = Path(
    "/home/charl/.local/share/uv/tools/graphifyy/lib/python3.13/site-packages"
)

# Copied from inventory_and_detect.py. Re-applied here; this script does not
# modify that file.
EXTRA_EXCLUDES = [
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
    ".scrapling/",
    "downloads/",
    ".vault-meta/",
    ".codex/",
    ".foreman/*.db",
    ".foreman/*.db-*",
    "deliverables/repo-graph-audit-2026-09-11/graphify-out-prior/",
    "deliverables/repo-graph-audit-2026-09-11/graphify-out/",
    ".git/",
]

# This packet's own outputs, caches, and working files stay out of the map.
PACKET_PREFIX = "deliverables/repo-graph-audit-2026-09-11/"
OWN_OUTPUTS = {
    PACKET_PREFIX + "repository-file-map.json",
    PACKET_PREFIX + "repository-file-map.html",
    PACKET_PREFIX + "FILE_MAP.md",
}

TEXT_EXTRACT_PREFIX_DENY = (
    "repos/",
    "raw/",
    ".raw/",
    "evidence/",
    "graphs/",
    "corpus/",
    "inbox/",
    PACKET_PREFIX,
)
TEXT_EXTRACT_EXTS = {".md", ".mdx", ".rst"}
TEXT_EXTRACT_MAX_BYTES = 512 * 1024

MD_LINK_RE = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
WIKI_LINK_RE = re.compile(r"\[\[([^\]|#]+)(?:#[^\]|]+)?(?:\|[^\]]+)?\]\]")
FILE_REF_RE = re.compile(
    r"`([A-Za-z0-9_./+-]+(?:/[A-Za-z0-9_./+-]+)+\.[A-Za-z0-9._+-]{1,12})`"
)
SCHEME_RE = re.compile(r"^[a-zA-Z][a-zA-Z0-9+.-]*:")

TYPE_CHAR = {
    "code": "c",
    "document": "d",
    "paper": "p",
    "image": "i",
    "video": "v",
    "unclassified": "u",
    "directory": "D",
}
CHAR_TYPE = {v: k for k, v in TYPE_CHAR.items()}
XREF_KINDS = ("markdown-link", "wikilink", "file-reference")


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def git_head() -> str:
    try:
        return subprocess.check_output(
            ["git", "-C", str(REPO), "rev-parse", "HEAD"],
            text=True,
            stderr=subprocess.DEVNULL,
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def to_rel(raw: str) -> str | None:
    """Exact detect path relative to the repo; do not resolve symlinks.

    IDs must be the repo-relative path recorded by detect, not the canonical
    target of an in-repo symlink. Escape checks happen separately.
    """
    text = str(raw).replace("\\", "/")
    repo_s = str(REPO).replace("\\", "/")
    prefix = repo_s + "/"
    if text == repo_s:
        return None
    if text.startswith(prefix):
        rel = text[len(prefix) :]
    elif text.startswith("/"):
        return None
    else:
        rel = text[2:] if text.startswith("./") else text
    rel = PurePosixPath(rel).as_posix()
    if rel in {".", ""}:
        return None
    if ".." in PurePosixPath(rel).parts:
        return None
    return rel


def extra_exclude_match(rel: str) -> bool:
    posix = rel.replace("\\", "/")
    name = posix.rsplit("/", 1)[-1]
    parts = posix.split("/")
    for pat in EXTRA_EXCLUDES:
        body = pat.rstrip("/")
        if pat.endswith("/") and (posix == body or posix.startswith(body + "/")):
            return True
        if fnmatch(posix, pat) or fnmatch(name, pat) or fnmatch(posix, body):
            return True
        if pat.startswith("**/"):
            rest = pat[3:]
            if fnmatch(posix, rest) or fnmatch(name, rest) or fnmatch(posix, pat):
                return True
            if rest.endswith("/**"):
                mid = rest[:-3].rstrip("/")
                if mid in parts or posix == mid or posix.startswith(mid + "/"):
                    return True
            for i, part in enumerate(parts):
                if fnmatch(part, rest.rstrip("/")) or fnmatch("/".join(parts[i:]), rest):
                    return True
        elif "/" in pat.rstrip("/"):
            if fnmatch(posix, pat) or fnmatch(posix, body):
                return True
    return False


def load_graphify_helpers():
    sys.path.insert(0, str(GRAPHIFY_SITE))
    from graphify.detect import (  # type: ignore
        _SKIP_DIRS,
        _SKIP_FILES,
        _is_noise_dir,
        _is_sensitive,
        _resolves_under_root,
        ignored_predicate,
    )

    predicate = ignored_predicate(REPO, extra_excludes=EXTRA_EXCLUDES, gitignore=False)
    return {
        "skip_dirs": _SKIP_DIRS,
        "skip_files": _SKIP_FILES,
        "is_noise_dir": _is_noise_dir,
        "is_sensitive": _is_sensitive,
        "resolves_under_root": _resolves_under_root,
        "ignored": predicate,
    }


def path_has_skip_dir(rel: str, helpers: dict) -> bool:
    parts = rel.split("/")
    skip = helpers["skip_dirs"]
    parent = REPO
    for i, part in enumerate(parts[:-1]):
        if part in skip or part in {".git", ".worktrees", "graphify-out"}:
            return True
        if helpers["is_noise_dir"](part, parent):
            return True
        parent = parent / part
    if parts[-1] in {".git"}:
        return True
    return False


def classify_drop(rel: str, abs_path: Path, helpers: dict) -> str | None:
    if rel in OWN_OUTPUTS or rel.startswith(PACKET_PREFIX):
        return "deliverable_packet"
    if rel.startswith(".worktrees/") or "/.worktrees/" in rel:
        return "worktree"
    if extra_exclude_match(rel):
        return "extra_exclude"
    if path_has_skip_dir(rel, helpers):
        return "skip_dir"
    if abs_path.name in helpers["skip_files"]:
        return "skip_file"
    try:
        if helpers["ignored"](abs_path):
            return "extra_exclude"
    except (OSError, ValueError):
        pass
    try:
        if abs_path.exists():
            st = abs_path.lstat()
            if stat.S_ISLNK(st.st_mode):
                if not helpers["resolves_under_root"](abs_path, REPO):
                    return "symlink_escape"
            elif not stat.S_ISREG(st.st_mode):
                return "not_regular"
            if not helpers["resolves_under_root"](abs_path, REPO):
                return "symlink_escape"
        else:
            # Missing files remain in the detect corpus unless the path itself
            # escapes the repo prefix.
            if ".." in PurePosixPath(rel).parts:
                return "symlink_escape"
    except (OSError, RuntimeError):
        return "symlink_escape"
    try:
        if helpers["is_sensitive"](abs_path):
            return "sensitive"
    except (OSError, ValueError):
        pass
    return None


def dir_id(rel_dir: str) -> str:
    if rel_dir in {"", "."}:
        return "."
    return rel_dir if rel_dir.endswith("/") else rel_dir + "/"


def parent_dir_id(rel: str) -> str:
    parent = str(PurePosixPath(rel).parent)
    if parent in {".", ""}:
        return "."
    return parent + "/"


def load_detect() -> dict:
    if not DETECT_PATH.is_file():
        raise SystemExit(f"missing detect corpus: {DETECT_PATH}")
    return json.loads(DETECT_PATH.read_text(encoding="utf-8"))


def union_corpus(detect: dict) -> tuple[dict[str, dict], dict]:
    rows: dict[str, dict] = {}
    collisions = []
    unsupported_abs = []
    files = detect.get("files") or {}
    for detect_type, paths in files.items():
        for raw in paths or []:
            rel = to_rel(raw)
            if rel is None:
                unsupported_abs.append(raw)
                continue
            if rel in rows and rows[rel]["detect_type"] != detect_type:
                collisions.append(rel)
            rows[rel] = {
                "rel": rel,
                "raw": raw,
                "detect_type": detect_type,
                "scope": "supported",
            }
    for raw in detect.get("unclassified") or []:
        rel = to_rel(raw)
        if rel is None:
            unsupported_abs.append(raw)
            continue
        if rel in rows:
            collisions.append(rel)
            continue
        rows[rel] = {
            "rel": rel,
            "raw": raw,
            "detect_type": "unclassified",
            "scope": "unclassified",
        }
    coverage = {
        "detect_total_files": detect.get("total_files"),
        "detect_total_words": detect.get("total_words"),
        "detect_warning": detect.get("warning"),
        "supported_by_type": {k: len(v or []) for k, v in files.items()},
        "supported_total": sum(len(v or []) for v in files.values()),
        "unclassified_total": len(detect.get("unclassified") or []),
        "union_unique": len(rows),
        "path_collisions_in_union": len(collisions),
        "absolute_outside_repo": len(unsupported_abs),
        "skipped_sensitive_detect": len(detect.get("skipped_sensitive") or []),
        "ignored_detect": len(detect.get("ignored") or []),
        "pruned_noise_dirs_detect": len(detect.get("pruned_noise_dirs") or []),
        "walk_errors_detect": len(detect.get("walk_errors") or []),
        "scan_root": detect.get("scan_root"),
    }
    return rows, coverage


def strip_href(raw: str) -> str | None:
    href = raw.strip().strip("<>").split()[0].strip("\"'")
    if not href or href.startswith("#"):
        return None
    if SCHEME_RE.match(href):
        return None
    href = href.split("#", 1)[0].split("?", 1)[0]
    if not href:
        return None
    return href


def resolve_against(source_rel: str, href: str) -> str | None:
    href = href.replace("\\", "/")
    if href.startswith("/"):
        cand = href.lstrip("/")
    else:
        base = str(PurePosixPath(source_rel).parent)
        cand = str(PurePosixPath(base) / href) if base not in {".", ""} else href
    cand = PurePosixPath(os.path.normpath(cand)).as_posix()
    if cand.startswith("../") or cand == ".." or cand.startswith("/"):
        return None
    if cand.startswith("./"):
        cand = cand[2:]
    return cand


def extract_literal_refs(source_rel: str, text: str, in_scope: set[str], dir_ids: set[str]) -> list[tuple[str, str, str]]:
    found: list[tuple[str, str, str]] = []
    seen: set[tuple[str, str, str]] = set()

    def add(target: str | None, kind: str, raw: str) -> None:
        if not target:
            return
        if target not in in_scope:
            as_dir = dir_id(target) if not target.endswith("/") else target
            if as_dir in dir_ids:
                target = as_dir
            elif target + ".md" in in_scope:
                target = target + ".md"
            else:
                return
        key = (source_rel, target, kind)
        if key in seen or source_rel == target:
            return
        seen.add(key)
        found.append((source_rel, target, kind))

    for match in MD_LINK_RE.finditer(text):
        href = strip_href(match.group(1))
        if href:
            add(resolve_against(source_rel, href), "markdown-link", href)
    for match in WIKI_LINK_RE.finditer(text):
        inner = match.group(1).strip()
        if not inner or SCHEME_RE.match(inner):
            continue
        inner = inner.replace("\\", "/")
        candidates = []
        resolved = resolve_against(source_rel, inner)
        if resolved:
            candidates.extend([resolved, resolved + ".md"])
        candidates.extend([inner, inner + ".md", "wiki/" + inner, "wiki/" + inner + ".md"])
        for cand in candidates:
            if cand in in_scope or dir_id(cand) in dir_ids:
                add(cand if cand in in_scope else dir_id(cand), "wikilink", inner)
                break
    for match in FILE_REF_RE.finditer(text):
        inner = match.group(1).strip().replace("\\", "/")
        resolved = resolve_against(source_rel, inner) if not inner.startswith("/") else inner.lstrip("/")
        add(resolved if resolved in in_scope else (inner if inner in in_scope else None), "file-reference", inner)
    return found


def bounded_text_source(rel: str, detect_type: str) -> bool:
    if detect_type not in {"document", "unclassified"}:
        suffix = Path(rel).suffix.lower()
        if suffix not in TEXT_EXTRACT_EXTS:
            return False
    suffix = Path(rel).suffix.lower()
    if suffix not in TEXT_EXTRACT_EXTS:
        return False
    if rel.startswith(TEXT_EXTRACT_PREFIX_DENY):
        return False
    if "graphify-out/" in rel:
        return False
    return True


def collect_dirs(file_rels: list[str]) -> list[str]:
    dirs: set[str] = set()
    for rel in file_rels:
        parts = rel.split("/")
        acc = []
        for part in parts[:-1]:
            acc.append(part)
            dirs.add("/".join(acc) + "/")
    return sorted(dirs)


def build_nodes_and_links(
    kept: dict[str, dict],
    xrefs: list[tuple[str, str, str]],
) -> tuple[list[dict], list[dict], list[str]]:
    file_rels = sorted(kept)
    dir_rels = collect_dirs(file_rels)
    nodes: list[dict] = []
    links: list[dict] = []

    nodes.append(
        {
            "id": ".",
            "label": ".",
            "file_type": "directory",
            "source_file": ".",
            "source_location": None,
            "_origin": "file-map",
            "_callable": False,
            "extraction_layer": "file-map",
            "detect_type": "directory",
            "scope": "directory",
        }
    )
    for d in dir_rels:
        nodes.append(
            {
                "id": d,
                "label": PurePosixPath(d.rstrip("/")).name,
                "file_type": "directory",
                "source_file": d,
                "source_location": None,
                "_origin": "file-map",
                "_callable": False,
                "extraction_layer": "file-map",
                "detect_type": "directory",
                "scope": "directory",
            }
        )
        links.append(
            {
                "source": parent_dir_id(d.rstrip("/")),
                "target": d,
                "relation": "contains",
                "confidence": "EXTRACTED",
                "confidence_score": 1.0,
                "context": "directory-containment",
                "source_file": parent_dir_id(d.rstrip("/")),
                "_origin": "file-map",
                "weight": 1.0,
            }
        )
    for rel in file_rels:
        rec = kept[rel]
        nodes.append(
            {
                "id": rel,
                "label": PurePosixPath(rel).name,
                "file_type": rec["detect_type"] if rec["detect_type"] != "unclassified" else "unclassified",
                "source_file": rel,
                "source_location": None,
                "_origin": "file-map",
                "_callable": False,
                "extraction_layer": "file-map",
                "detect_type": rec["detect_type"],
                "scope": rec["scope"],
            }
        )
        links.append(
            {
                "source": parent_dir_id(rel),
                "target": rel,
                "relation": "contains",
                "confidence": "EXTRACTED",
                "confidence_score": 1.0,
                "context": "directory-containment",
                "source_file": parent_dir_id(rel),
                "_origin": "file-map",
                "weight": 1.0,
            }
        )
    for src, tgt, kind in xrefs:
        links.append(
            {
                "source": src,
                "target": tgt,
                "relation": "references",
                "confidence": "EXTRACTED",
                "confidence_score": 1.0,
                "context": kind,
                "source_file": src,
                "_origin": "file-map",
                "extraction_note": "EXTRACTED literal in-repo reference; not a symbol call",
                "weight": 1.0,
            }
        )
    return nodes, links, dir_rels


def validate_graph(nodes: list[dict], links: list[dict], kept: dict[str, dict]) -> dict:
    ids = [n["id"] for n in nodes]
    id_set = set(ids)
    dupes = [i for i, c in Counter(ids).items() if c > 1]
    file_nodes = [n for n in nodes if n.get("scope") != "directory"]
    path_id_mismatch = [n["id"] for n in file_nodes if n["id"] != n.get("source_file")]
    missing_endpoints = []
    for link in links:
        if link["source"] not in id_set or link["target"] not in id_set:
            missing_endpoints.append(link)
    file_id_set = {n["id"] for n in file_nodes}
    kept_mismatch = sorted(set(kept) ^ file_id_set)
    return {
        "node_count": len(nodes),
        "link_count": len(links),
        "file_node_count": len(file_nodes),
        "directory_node_count": len(nodes) - len(file_nodes),
        "duplicate_ids": dupes,
        "duplicate_id_count": len(dupes),
        "path_id_mismatch_count": len(path_id_mismatch),
        "missing_endpoint_count": len(missing_endpoints),
        "kept_vs_file_node_mismatch_count": len(kept_mismatch),
        "zero_path_id_collisions": len(dupes) == 0 and len(path_id_mismatch) == 0,
        "endpoints_valid": len(missing_endpoints) == 0,
        "ok": (
            len(dupes) == 0
            and len(path_id_mismatch) == 0
            and len(missing_endpoints) == 0
            and len(kept_mismatch) == 0
        ),
    }


def write_json(path: Path, payload: dict) -> int:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, separators=(",", ":"))
        handle.write("\n")
    return path.stat().st_size


def write_html(
    path: Path,
    kept: dict[str, dict],
    xrefs: list[tuple[str, str, str]],
    meta: dict,
) -> int:
    files_sorted = sorted(kept)
    type_chars = "".join(TYPE_CHAR.get(kept[rel]["detect_type"], "u") for rel in files_sorted)
    scope_chars = "".join("s" if kept[rel]["scope"] == "supported" else "u" for rel in files_sorted)
    index_of = {rel: i for i, rel in enumerate(files_sorted)}
    compact_xrefs = []
    for src, tgt, kind in xrefs:
        if src in index_of and tgt in index_of:
            compact_xrefs.append([index_of[src], index_of[tgt], XREF_KINDS.index(kind)])
    payload = {
        "paths": files_sorted,
        "types": type_chars,
        "scopes": scope_chars,
        "xrefs": compact_xrefs,
        "kinds": list(XREF_KINDS),
        "typeLegend": CHAR_TYPE,
        "meta": {
            "fileCount": len(files_sorted),
            "xrefCount": len(compact_xrefs),
            "generatedAt": meta["generated_at"],
            "commit": meta["built_at_commit"],
        },
    }
    data = json.dumps(payload, ensure_ascii=False, separators=(",", ":")).replace("<", "\\u003c")
    html = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Moriarty file-level repository map</title>
<style>
:root { --bg:#10110f; --panel:#1a1c18; --ink:#e8e4d8; --muted:#9a9484; --line:#2c2f28; --acc:#c4b48a; --ok:#8fba8a; --warn:#d0a06a; }
* { box-sizing:border-box; }
html,body { margin:0; height:100%; background:var(--bg); color:var(--ink); font:14px/1.45 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace; }
header { padding:16px 20px 12px; border-bottom:1px solid var(--line); }
h1 { font-size:16px; font-weight:600; margin:0 0 6px; letter-spacing:.02em; }
.note { color:var(--muted); max-width:88ch; }
.note strong { color:var(--acc); font-weight:600; }
.stats { display:flex; gap:16px; flex-wrap:wrap; margin-top:10px; color:var(--muted); }
.stats b { color:var(--ink); font-weight:600; }
.layout { display:grid; grid-template-columns:minmax(280px,1fr) minmax(320px,1fr); height:calc(100% - 150px); }
.col { overflow:auto; padding:12px 16px 32px; }
.col + .col { border-left:1px solid var(--line); background:var(--panel); }
label { display:block; color:var(--muted); font-size:12px; margin-bottom:6px; }
input[type=search] { width:100%; background:#0d0e0c; color:var(--ink); border:1px solid var(--line); padding:8px 10px; }
.filters { display:flex; gap:8px; flex-wrap:wrap; margin:8px 0 12px; color:var(--muted); font-size:12px; }
.dir { margin:0; }
.dir > summary { cursor:pointer; list-style:none; padding:2px 0; color:var(--acc); }
.dir > summary::-webkit-details-marker { display:none; }
.dir > summary::before { content:'▸ '; color:var(--muted); }
.dir[open] > summary::before { content:'▾ '; }
.file { display:block; padding:1px 0 1px 14px; color:var(--ink); text-decoration:none; white-space:nowrap; }
.file:hover, .file.active { background:#252820; }
.count { color:var(--muted); font-size:12px; }
.panel h2 { font-size:13px; margin:0 0 8px; color:var(--acc); }
.kv { display:grid; grid-template-columns:120px 1fr; gap:4px 10px; margin-bottom:14px; }
.kv span { color:var(--muted); }
a.ref { color:var(--ok); text-decoration:none; display:block; padding:2px 0; }
a.ref:hover { text-decoration:underline; }
.empty { color:var(--muted); }
.hits { margin-top:10px; }
.hits a { display:block; color:var(--ink); text-decoration:none; padding:2px 0; }
.hits a:hover { color:var(--acc); }
</style>
</head>
<body>
<header>
<h1>File-level repository map</h1>
<p class="note">Companion to <strong>graphify-out/graph.json</strong> (semantic + AST). This view lists in-scope files from the detect corpus with directory containment and EXTRACTED literal in-repo references. It is <strong>not</strong> a claim that every file was AST-parsed or semantically reviewed.</p>
<div class="stats" id="stats"></div>
</header>
<div class="layout">
  <div class="col">
    <label for="q">Search paths</label>
    <input id="q" type="search" placeholder="substring match on repo-relative path" autocomplete="off">
    <div class="filters" id="filters"></div>
    <div id="hits" class="hits" hidden></div>
    <div id="tree"></div>
  </div>
  <div class="col" id="detail">
    <p class="empty">Select a file or directory. Children render on expand; the full node list is not mounted.</p>
  </div>
</div>
<script type="application/json" id="idx">""" + data + """</script>
<script>
const DATA = JSON.parse(document.getElementById('idx').textContent);
const PATHS = DATA.paths;
const TYPES = DATA.types;
const SCOPES = DATA.scopes;
const KINDS = DATA.kinds;
const ROOT = '../../';
const children = new Map();
const dirCount = new Map();
function parentOf(p){
  const i = p.lastIndexOf('/');
  return i === -1 ? '' : p.slice(0, i+1);
}
for (let i=0;i<PATHS.length;i++){
  const p = PATHS[i];
  const parts = p.split('/');
  let acc = '';
  for (let j=0;j<parts.length-1;j++){
    const dir = acc + parts[j] + '/';
    const par = acc;
    if (!children.has(par)) children.set(par, {dirs:new Set(), files:[]});
    children.get(par).dirs.add(dir);
    dirCount.set(dir, (dirCount.get(dir)||0)+1);
    acc = dir;
  }
  const par = parentOf(p);
  if (!children.has(par)) children.set(par, {dirs:new Set(), files:[]});
  children.get(par).files.push(i);
}
const outRefs = new Map();
const inRefs = new Map();
for (const [a,b,k] of DATA.xrefs){
  if (!outRefs.has(a)) outRefs.set(a, []);
  if (!inRefs.has(b)) inRefs.set(b, []);
  outRefs.get(a).push([b,k]);
  inRefs.get(b).push([a,k]);
}
function typeName(i){ return DATA.typeLegend[TYPES[i]] || 'unclassified'; }
function hrefFor(p){ return ROOT + p.split('/').map(encodeURIComponent).join('/'); }
const treeEl = document.getElementById('tree');
const hitsEl = document.getElementById('hits');
const detailEl = document.getElementById('detail');
const statsEl = document.getElementById('stats');
statsEl.innerHTML = '<span><b>'+DATA.meta.fileCount.toLocaleString()+'</b> files</span>'
  + '<span><b>'+DATA.xrefs.length.toLocaleString()+'</b> EXTRACTED literal refs</span>'
  + '<span>commit <b>'+DATA.meta.commit.slice(0,12)+'</b></span>'
  + '<span>'+DATA.meta.generatedAt+'</span>';
const filtersEl = document.getElementById('filters');
const enabled = new Set(Object.values(DATA.typeLegend));
enabled.add('unclassified');
['code','document','paper','image','video','unclassified'].forEach(t => {
  const lab = document.createElement('label');
  lab.innerHTML = '<input type="checkbox" checked data-t="'+t+'"> '+t;
  filtersEl.appendChild(lab);
});
filtersEl.addEventListener('change', e => {
  const t = e.target.getAttribute('data-t');
  if (!t) return;
  if (e.target.checked) enabled.add(t); else enabled.delete(t);
  renderRoot();
});
function renderDir(dir, host){
  const rec = children.get(dir);
  if (!rec) return;
  const dirs = Array.from(rec.dirs).sort();
  for (const d of dirs){
    const det = document.createElement('details');
    det.className = 'dir';
    const sum = document.createElement('summary');
    sum.innerHTML = d + ' <span class="count">'+(dirCount.get(d)||0)+'</span>';
    sum.addEventListener('click', (ev) => { ev.stopPropagation(); showDir(d); });
    det.appendChild(sum);
    const inner = document.createElement('div');
    det.appendChild(inner);
    det.addEventListener('toggle', () => {
      if (det.open && inner.childNodes.length === 0) renderDir(d, inner);
    });
    host.appendChild(det);
  }
  for (const i of rec.files){
    if (!enabled.has(typeName(i))) continue;
    const a = document.createElement('a');
    a.className = 'file';
    a.href = hrefFor(PATHS[i]);
    a.textContent = PATHS[i].slice(dir.length);
    a.dataset.i = String(i);
    a.addEventListener('click', ev => { ev.preventDefault(); showFile(i); });
    host.appendChild(a);
  }
}
function renderRoot(){
  treeEl.innerHTML = '';
  renderDir('', treeEl);
}
function showDir(d){
  const n = dirCount.get(d)||0;
  detailEl.innerHTML = '<div class="panel"><h2>directory</h2><div class="kv">'
    + '<span>id</span><div>'+d+'</div>'
    + '<span>files beneath</span><div>'+n+'</div>'
    + '<span>parent</span><div>'+(d==='.'||d===''?'.':parentOf(d.endsWith('/')?d.slice(0,-1):d) || '.')+'</div>'
    + '</div><p class="empty">Containment edges are EXTRACTED from the detect corpus. Expand the tree to load children.</p></div>';
}
function refList(title, items){
  if (!items || !items.length) return '<p class="empty">'+title+': none EXTRACTED</p>';
  return '<h2>'+title+'</h2>' + items.map(([idx,k]) =>
    '<a class="ref" data-i="'+idx+'" href="'+hrefFor(PATHS[idx])+'">'+PATHS[idx]+' <span class="count">'+KINDS[k]+'</span></a>'
  ).join('');
}
function showFile(i){
  document.querySelectorAll('.file.active').forEach(n => n.classList.remove('active'));
  const p = PATHS[i];
  const outs = outRefs.get(i)||[];
  const ins = inRefs.get(i)||[];
  detailEl.innerHTML = '<div class="panel"><h2>file</h2><div class="kv">'
    + '<span>id / path</span><div>'+p+'</div>'
    + '<span>detect type</span><div>'+typeName(i)+'</div>'
    + '<span>scope</span><div>'+(SCOPES[i]==='s'?'supported':'unclassified')+'</div>'
    + '<span>parent</span><div>'+(parentOf(p)||'.')+'</div>'
    + '<span>source</span><div><a class="ref" href="'+hrefFor(p)+'">open path</a></div>'
    + '</div>'
    + refList('EXTRACTED outbound literal references', outs)
    + refList('EXTRACTED inbound literal references', ins)
    + '<p class="empty">No symbol-call edges are invented here. Neighbor navigation uses containment plus literal Markdown/file references only.</p></div>';
  detailEl.querySelectorAll('a.ref[data-i]').forEach(a => {
    a.addEventListener('click', ev => { ev.preventDefault(); showFile(Number(a.dataset.i)); });
  });
}
document.getElementById('q').addEventListener('input', ev => {
  const q = ev.target.value.trim().toLowerCase();
  if (!q){ hitsEl.hidden = true; treeEl.hidden = false; return; }
  treeEl.hidden = true; hitsEl.hidden = false; hitsEl.innerHTML = '';
  let n = 0;
  for (let i=0;i<PATHS.length && n<200;i++){
    if (!enabled.has(typeName(i))) continue;
    if (PATHS[i].toLowerCase().indexOf(q) === -1) continue;
    const a = document.createElement('a');
    a.href = hrefFor(PATHS[i]);
    a.textContent = PATHS[i];
    a.addEventListener('click', e => { e.preventDefault(); showFile(i); });
    hitsEl.appendChild(a);
    n++;
  }
  const more = document.createElement('p');
  more.className = 'empty';
  more.textContent = n>=200 ? 'First 200 matches. Refine the query.' : n+' match(es).';
  hitsEl.appendChild(more);
});
renderRoot();
</script>
</body>
</html>
"""
    path.write_text(html, encoding="utf-8")
    return path.stat().st_size


def write_markdown(path: Path, report: dict) -> int:
    cov = report["source_coverage"]
    val = report["validation"]
    drops = report["dropped"]
    kept_types = report["kept_by_type"]
    top = report["kept_top_level"]
    lines = [
        "# File-level repository map",
        "",
        "This packet is a **file-level repository graph** companion to `graphify-out/graph.json` (semantic + AST).",
        "It does not claim that every listed file was AST-parsed or semantically reviewed.",
        "",
        "## Identity",
        "",
        f"- Requested extraction identity: `{report['requested_model']}`",
        f"- Host-declared runtime identity: `{report['host_declared_identity']}`",
        f"- Observed environment model fields: `{report['env_model_fields'] or 'none'}`",
        f"- Builder: `deliverables/repo-graph-audit-2026-09-11/scripts/build_file_map.py`",
        f"- Generated at: `{report['generated_at']}`",
        f"- Repository HEAD: `{report['built_at_commit']}`",
        f"- Wall-clock seconds: `{report['wall_clock_seconds']}`",
        f"- Detect corpus: `deliverables/repo-graph-audit-2026-09-11/graphify-out/.graphify_detect.json` (read-only)",
        "",
        "No LLM completion was used to invent nodes, edges, or symbol calls.",
        f"The measured builder time is {report['wall_clock_seconds']} seconds of local Python work.",
        "That is not a token invoice and not a claim of zero cost.",
        "",
        "## Source coverage preserved from detect",
        "",
        f"- Supported files reported by detect: **{cov['supported_total']}**",
        f"- Unclassified files reported by detect: **{cov['unclassified_total']}**",
        f"- Union of unique repo-relative paths before this script's re-filter: **{cov['union_unique']}**",
        f"- Detect `total_files`: {cov['detect_total_files']}",
        f"- Detect `total_words`: {cov['detect_total_words']}",
        f"- Detect skipped_sensitive: {cov['skipped_sensitive_detect']}",
        f"- Detect ignored entries: {cov['ignored_detect']}",
        f"- Detect pruned_noise_dirs: {cov['pruned_noise_dirs_detect']}",
        f"- Detect walk_errors: {cov['walk_errors_detect']}",
        f"- Union path collisions (same path in more than one detect list): {cov['path_collisions_in_union']}",
        f"- Absolute paths outside the repo prefix: {cov['absolute_outside_repo']}",
        "",
        "Supported files by detect type:",
        "",
        "| Type | Detect count |",
        "| --- | ---: |",
    ]
    for k, v in sorted(cov["supported_by_type"].items()):
        lines.append(f"| {k} | {v} |")
    lines += [
        "",
        "Detect warning (verbatim):",
        "",
        f"> {cov['detect_warning']}",
        "",
        "## In-scope file map after re-filter",
        "",
        f"- In-scope files: **{report['kept_files']}**",
        f"- Directory nodes: **{report['directory_nodes']}**",
        f"- Total nodes: **{val['node_count']}**",
        f"- Total links: **{val['link_count']}** (containment + EXTRACTED literal references)",
        f"- EXTRACTED literal reference edges: **{report['xref_edges']}**",
        f"- Bounded textual sources scanned: **{report['text_sources_scanned']}**",
        f"- Text sources skipped as too large: **{report['text_sources_too_large']}**"
        + (
            " (`"
            + "`, `".join(report.get("text_sources_too_large_paths") or [])
            + "`)"
            if report.get("text_sources_too_large_paths")
            else ""
        ),
        "",
        "Kept files by detect type:",
        "",
        "| Type | Kept |",
        "| --- | ---: |",
    ]
    for k, v in sorted(kept_types.items(), key=lambda kv: (-kv[1], kv[0])):
        lines.append(f"| {k} | {v} |")
    lines += [
        "",
        "Kept files by first path segment:",
        "",
        "| Segment | Kept |",
        "| --- | ---: |",
    ]
    for k, v in top:
        lines.append(f"| `{k}` | {v} |")
    lines += [
        "",
        "## Exclusion policy",
        "",
        "Authoritative input is the existing detect JSON. This script does not rescan the live tree for membership.",
        "Each union path is re-checked and dropped on the first matching reason:",
        "",
        "1. This deliverable packet (`deliverables/repo-graph-audit-2026-09-11/`), including this map's own outputs.",
        "2. `.worktrees/` checkout duplicates.",
        "3. Main-script `EXTRA_EXCLUDES` (secrets, witnesses, wallets, selected caches, this packet's `graphify-out` / `graphify-out-prior`, `.git/`).",
        "4. Graphify builtin skip/noise directories (`_SKIP_DIRS` / `_is_noise_dir`), including `.git`, `.worktrees`, `graphify-out`, dependency and cache dirs.",
        "5. Graphify `_SKIP_FILES` lockfiles.",
        "6. Graphify `ignored_predicate(..., extra_excludes=EXTRA_EXCLUDES, gitignore=False)`.",
        "7. Symlink targets that resolve outside `/home/charl/Moriarty`.",
        "8. Non-regular files.",
        "9. Graphify `_is_sensitive` builtin credential heuristics.",
        "",
        "Dropped from the union by this re-filter:",
        "",
        "| Reason | Count |",
        "| --- | ---: |",
    ]
    for k, v in sorted(drops.items()):
        lines.append(f"| {k} | {v} |")
    lines += [
        "",
        f"- Total dropped from union: **{report['dropped_total']}**",
        "",
        "Dropped paths:",
        "",
    ]
    for reason, paths in sorted(report["dropped_paths"].items()):
        lines.append(f"**{reason}**")
        lines.append("")
        for rel in paths:
            lines.append(f"- `{rel}`")
        lines.append("")
    lines += [
        "Detect already recorded skipped_sensitive, ignored, and pruned_noise_dirs. Those counts stay as source coverage.",
        "They are not double-counted as in-scope files.",
        "",
        "## Identifiers and edges",
        "",
        "- File node `id` is the exact repo-relative POSIX path from detect. It is not a lossy slug and is not the resolved symlink target.",
        "- Directory node `id` is that directory path with a trailing `/`. The repository root is `.`.",
        "- `source_file` on a file node equals `id`.",
        "- Directory containment edges use `relation: contains`, `confidence: EXTRACTED`.",
        "- Literal Markdown links, wikilinks, and backtick file references from bounded textual sources use `relation: references`, `confidence: EXTRACTED`.",
        "- No `calls` / symbol edges are created.",
        "",
        "## Validation",
        "",
        f"- Duplicate node IDs: {val['duplicate_id_count']}",
        f"- Path/ID mismatches: {val['path_id_mismatch_count']}",
        f"- Links with missing endpoints: {val['missing_endpoint_count']}",
        f"- Kept-path vs file-node mismatch: {val['kept_vs_file_node_mismatch_count']}",
        f"- Zero path/ID collisions: {val['zero_path_id_collisions']}",
        f"- Endpoints valid: {val['endpoints_valid']}",
        f"- Validation ok: {val['ok']}",
        "",
        "## Outputs",
        "",
        f"- `{report['json_path']}` ({report['json_bytes']} bytes)",
        f"- `{report['html_path']}` ({report['html_bytes']} bytes)",
        f"- `{report['md_path']}`",
        "",
        "The JSON uses the graphify exported node-link shape: `directed`, `multigraph`, `graph`, `nodes`, `links`, `hyperedges`, `built_at_commit`.",
        "Node IDs in this file are paths. They do not merge with slug IDs in `graphify-out/graph.json`.",
        "",
        "## HTML",
        "",
        "The HTML is a self-contained offline directory tree. It embeds a compact path index.",
        "Directories render children only when expanded. Search lists at most 200 path matches.",
        "Selecting a file shows parent containment and EXTRACTED literal cross-references.",
        "It does not mount tens of thousands of DOM nodes at once and does not load a force-directed graph of the whole map.",
        "",
        "## Limitations",
        "",
        "- Membership is the detect snapshot, not a live `os.walk` of the dirty tree after detect.",
        "- In-repo symlinks keep their own path IDs. Only symlink targets that resolve outside the repository are dropped.",
        "- `repos/` is the largest first segment; pinned clones dominate file counts.",
        "- Unclassified files are listed. Their bytes were not parsed as source.",
        "- Literal references are extracted only from bounded first-party `.md` / `.mdx` / `.rst` files (not `repos/`, `raw/`, `evidence/`, `graphs/`, `corpus/`, `inbox/`, or this packet).",
        "- Wikilinks resolve only when the target path or `path.md` is already in-scope. Missing targets are omitted, not invented.",
        "- Sensitive-name heuristics can drop documents whose filenames look like credential stores.",
        "- This map is not product acceptance, not financial evidence, and not an AST coverage claim.",
        "",
    ]
    path.write_text("\n".join(lines), encoding="utf-8")
    return path.stat().st_size


def env_model_fields() -> dict:
    keys = [
        "GROK_MODEL",
        "XAI_MODEL",
        "LLM_MODEL",
        "MODEL",
        "AI_MODEL",
        "GROK_MODEL_NAME",
    ]
    return {k: os.environ.get(k) for k in keys if os.environ.get(k)}


def main() -> int:
    started = time.perf_counter()
    if not str(OUT).startswith(str(REPO)):
        raise SystemExit("refusing to write outside the repository")
    helpers = load_graphify_helpers()
    detect = load_detect()
    rows, coverage = union_corpus(detect)

    dropped: Counter[str] = Counter()
    dropped_paths: dict[str, list[str]] = defaultdict(list)
    kept: dict[str, dict] = {}
    missing_on_disk = 0
    for rel in sorted(rows):
        rec = rows[rel]
        abs_path = REPO / rel
        reason = classify_drop(rel, abs_path, helpers)
        if reason:
            dropped[reason] += 1
            dropped_paths[reason].append(rel)
            continue
        if not abs_path.exists():
            missing_on_disk += 1
        kept[rel] = rec

    dir_rels = collect_dirs(sorted(kept))
    dir_ids = {"."} | set(dir_rels)
    in_scope = set(kept)
    xrefs: list[tuple[str, str, str]] = []
    text_scanned = 0
    text_too_large = 0
    text_too_large_paths: list[str] = []
    for rel, rec in kept.items():
        if not bounded_text_source(rel, rec["detect_type"]):
            continue
        abs_path = REPO / rel
        try:
            size = abs_path.stat().st_size
        except OSError:
            continue
        if size > TEXT_EXTRACT_MAX_BYTES:
            text_too_large += 1
            text_too_large_paths.append(rel)
            continue
        try:
            text = abs_path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        text_scanned += 1
        xrefs.extend(extract_literal_refs(rel, text, in_scope, dir_ids))
    xrefs.sort()
    # stable unique
    uniq = []
    seen = set()
    for item in xrefs:
        if item in seen:
            continue
        seen.add(item)
        uniq.append(item)
    xrefs = uniq

    nodes, links, dir_rels = build_nodes_and_links(kept, xrefs)
    validation = validate_graph(nodes, links, kept)

    kept_by_type = Counter(rec["detect_type"] for rec in kept.values())
    top_level = Counter((rel.split("/", 1)[0] if "/" in rel else "(root)") for rel in kept)
    generated_at = utc_now()
    commit = git_head()
    host_identity = "Grok 4.6 released by xAI"
    requested = "grok-4.6"
    env_fields = env_model_fields()

    graph_payload = {
        "directed": False,
        "multigraph": False,
        "graph": {
            "kind": "file-level-repository-map",
            "companion_to": "graphify-out/graph.json",
            "not_ast_parsed": True,
            "not_semantic_review": True,
            "detect_corpus": "deliverables/repo-graph-audit-2026-09-11/graphify-out/.graphify_detect.json",
            "generated_at": generated_at,
            "requested_model": requested,
            "host_declared_identity": host_identity,
            "source_coverage": coverage,
            "kept_files": len(kept),
            "directory_nodes": len(dir_rels) + 1,
            "dropped": dict(dropped),
            "missing_on_disk": missing_on_disk,
            "hyperedges": [],
        },
        "nodes": nodes,
        "links": links,
        "hyperedges": [],
        "built_at_commit": commit,
    }

    json_path = OUT / "repository-file-map.json"
    html_path = OUT / "repository-file-map.html"
    md_path = OUT / "FILE_MAP.md"
    json_bytes = write_json(json_path, graph_payload)

    # Reload-check endpoints on the written file without keeping two full copies
    # longer than needed: verify counts from the in-memory payload, then spot-check
    # the written document parses and matches counts.
    written = json.loads(json_path.read_text(encoding="utf-8"))
    if len(written["nodes"]) != len(nodes) or len(written["links"]) != len(links):
        raise SystemExit("written JSON count mismatch")
    written_ids = {n["id"] for n in written["nodes"]}
    if len(written_ids) != len(written["nodes"]):
        raise SystemExit("written JSON has duplicate IDs")
    for link in written["links"]:
        if link["source"] not in written_ids or link["target"] not in written_ids:
            raise SystemExit("written JSON has invalid endpoints")
    del written

    html_bytes = write_html(
        html_path,
        kept,
        xrefs,
        {"generated_at": generated_at, "built_at_commit": commit},
    )
    wall = round(time.perf_counter() - started, 3)
    report = {
        "generated_at": generated_at,
        "built_at_commit": commit,
        "requested_model": requested,
        "host_declared_identity": host_identity,
        "env_model_fields": env_fields or None,
        "wall_clock_seconds": wall,
        "source_coverage": coverage,
        "kept_files": len(kept),
        "directory_nodes": len(dir_rels) + 1,
        "dropped": dict(dropped),
        "dropped_paths": {k: v for k, v in sorted(dropped_paths.items())},
        "dropped_total": int(sum(dropped.values())),
        "missing_on_disk": missing_on_disk,
        "kept_by_type": dict(kept_by_type),
        "kept_top_level": top_level.most_common(20),
        "xref_edges": len(xrefs),
        "text_sources_scanned": text_scanned,
        "text_sources_too_large": text_too_large,
        "text_sources_too_large_paths": text_too_large_paths,
        "validation": validation,
        "json_path": str(json_path.relative_to(REPO)),
        "html_path": str(html_path.relative_to(REPO)),
        "md_path": str(md_path.relative_to(REPO)),
        "json_bytes": json_bytes,
        "html_bytes": html_bytes,
    }
    md_bytes = write_markdown(md_path, report)
    report["md_bytes"] = md_bytes
    print(json.dumps(report, indent=2, ensure_ascii=False))
    return 0 if validation["ok"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
