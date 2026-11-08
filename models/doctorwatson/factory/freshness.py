"""E7 (SAS-002): freshness of recorded results by declared dependency closure.

A predicate row may declare

    deps:
      paths: ["crates/mori-backend/**", "openspec/.../spec.md"]  # source, spec, contract files
      pins: preview                     # target-pins.json networks.<row> (whole row)
      chain: preview                    # target-pins.json networks.<row>.chain (chain identity)
      manifests: ["target/keys/node.manifest.json"]             # key manifests, hashed by content
      receipts: ["qa/sherlock/receipt-traces/OP/receipt.json"]  # relied-on receipts, by content

When a passing result is written, `stamp` records `closure`: one hash per dependency
(plus `row`, the hash of the row's own definition) and the run `head` for provenance.
A result is stale iff any recorded hash differs from the current one, a `depends` row
is stale or not passing, or no closure was recorded. HEAD equality plays no part, so
a commit that only records factory state or undeclared receipts keeps results fresh.

`paths` hashes the git blob ids of the working-tree files that match (tracked and
untracked, not ignored): content-addressed, so a check run on a dirty tree stays fresh
after its change is committed. A pattern without glob characters matches the path or
everything under it; a glob uses fnmatch, where `*` also crosses `/` (over-matching is
the safe direction). `manifests` and `receipts` hash the named files directly, so a
gitignored key manifest still counts.

A row without `deps:` gets the DEFAULT closure: every working-tree path except
factory state and DEFAULT_EXCLUDE (receipt locations, `budgets.json`, and
`predicates.yaml`, whose per-row content the `row` hash covers). It is conservative
(any source, spec or pins change makes it stale) and a state-only or receipt-only
commit keeps it eligible. It behaves close to a HEAD gate, so it is an interim
signal: `next` and `validate` report such rows as `default (undeclared)`.

Run results are never a dependency of any row (STATE_FILES, STATE_DIRS): status.json,
runs.jsonl, agents.jsonl and the gitignored traces/ and agents/ directories. A row may
still declare `budgets.json`, `predicates.yaml` or a receipt explicitly.
"""

import bisect
import fnmatch
import hashlib
import json
import os
import subprocess

from . import head_sha

PINS = "qa/sherlock/target-pins.json"
STATE_FILES = frozenset((
    "qa/sherlock/status.json", "qa/sherlock/runs.jsonl", "qa/sherlock/agents.jsonl",
    "qa/sherlock/.status.lock",
))
STATE_DIRS = ("qa/sherlock/traces/", "qa/sherlock/agents/")  # gitignored run output
# Default closure only. Receipts: the factory location (workflow.md: committed
# qa/sherlock/receipt-traces/<op>/) and receipt-named files or directories elsewhere.
DEFAULT_EXCLUDE = (
    "qa/sherlock/receipt-traces/*", "*/receipts/*", "receipts/*",
    "*/receipt.json", "receipt.json", "*-receipt.json", "*.receipt.json",
    "qa/sherlock/budgets.json", "qa/sherlock/predicates.yaml",
)
DEFAULT_DEPS = {"paths": ["."], "exclude": DEFAULT_EXCLUDE}
UNDECLARED = "default (undeclared)"
ROW_FIELDS = ("id", "set", "tier", "check", "heavy", "budget", "depends", "holdout", "deps")
NO_CLOSURE = "no recorded closure"
GLOB = "*?["


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _canon(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()


def _blob_id(path):
    """git blob id of a file or symlink (the same id `git add` would store)."""
    data = os.readlink(path).encode() if os.path.islink(path) else open(path, "rb").read()
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()


def _is_state(rel):
    return rel in STATE_FILES or rel.startswith(STATE_DIRS) or ".tmp." in os.path.basename(rel)


def _as_list(v):
    return [v] if isinstance(v, str) else list(v or [])


def row_hash(p):
    return _sha(_canon({k: getattr(p, k, None) for k in ROW_FIELDS}))


def closure_of(p):
    return p.deps if p.deps is not None else DEFAULT_DEPS


def undeclared(preds):
    """Non-retired rows on the default closure (interim signal; declare `deps:`)."""
    return [p.id for p in preds if p.deps is None and p.retired is None]


class Snapshot:
    """Current dependency hashes of one repo; the file map is built lazily, once."""

    def __init__(self, repo):
        self.repo = repo
        self._files = None
        self._pins = None
        self._cache = {}

    # -- working-tree file map: sorted [(path, blob id)] -------------------------
    def _git(self, *args):
        return subprocess.run(["git", "-C", self.repo, *args], capture_output=True, check=True).stdout

    def _git_files(self):
        top = self._git("rev-parse", "--show-toplevel").decode().strip()
        if os.path.realpath(top) != os.path.realpath(self.repo):
            raise OSError("not the toplevel of a git work tree")
        files = {}
        for rec in self._git("ls-files", "-s", "-z").split(b"\0"):
            if rec:
                meta, path = rec.split(b"\t", 1)
                files[path.decode()] = meta.split()[1].decode()
        out = self._git("status", "--porcelain", "-z", "--untracked-files=all").split(b"\0")
        i = 0
        while i < len(out):
            rec = out[i].decode()
            i += 1
            if len(rec) < 4:
                continue
            if rec[0] in "RC":
                i += 1  # skip the rename/copy source
            rel = rec[3:]
            full = os.path.join(self.repo, rel)
            if os.path.isfile(full) or os.path.islink(full):
                files[rel] = _blob_id(full)
            elif not os.path.isdir(full):
                files.pop(rel, None)
        return files

    def _walk_files(self):
        files = {}
        for root, dirs, names in os.walk(self.repo):
            dirs[:] = [d for d in dirs if d != ".git"]
            for n in names:
                full = os.path.join(root, n)
                rel = os.path.relpath(full, self.repo).replace(os.sep, "/")
                if not _is_state(rel):
                    files[rel] = _blob_id(full)
        return files

    def files(self):
        if self._files is None:
            try:
                files = self._git_files()
            except (OSError, subprocess.CalledProcessError):
                files = self._walk_files()
            items = sorted((k, v) for k, v in files.items() if not _is_state(k))
            self._files = ([k for k, _ in items], [v for _, v in items])
        return self._files

    def _matching(self, patterns):
        paths, ids = self.files()
        hit = set()
        for pat in patterns:
            pat = pat.strip().lstrip("/")
            if pat in ("", ".", "**", "*"):
                return list(range(len(paths)))
            cut = min([pat.find(c) for c in GLOB if c in pat] or [len(pat)])
            prefix = pat[:cut]
            lo = bisect.bisect_left(paths, prefix)
            for i in range(lo, len(paths)):
                path = paths[i]
                if not path.startswith(prefix):
                    break
                if cut == len(pat):
                    base = pat.rstrip("/")
                    if path == base or path.startswith(base + "/"):
                        hit.add(i)
                elif fnmatch.fnmatchcase(path, pat):
                    hit.add(i)
        return sorted(hit)

    def paths_hash(self, patterns, exclude=()):
        key = ("paths", tuple(patterns), tuple(exclude))
        if key not in self._cache:
            paths, ids = self.files()
            h = hashlib.sha256()
            for i in self._matching(patterns):
                if any(fnmatch.fnmatchcase(paths[i], x) for x in exclude):
                    continue
                h.update(f"{paths[i]}\0{ids[i]}\n".encode())
            self._cache[key] = h.hexdigest()
        return self._cache[key]

    # -- hashed artifacts -------------------------------------------------------
    def file_hash(self, rel):
        full = os.path.join(self.repo, rel)
        try:
            with open(full, "rb") as fh:
                return _sha(fh.read())
        except OSError:
            return "absent"

    def pins_row(self, name, part=None):
        if self._pins is None:
            try:
                with open(os.path.join(self.repo, PINS), encoding="utf-8") as fh:
                    self._pins = json.load(fh)
            except (OSError, ValueError):
                self._pins = {}
        rowv = ((self._pins.get("networks") or {}) if isinstance(self._pins, dict) else {}).get(name)
        if part is not None:
            rowv = rowv.get(part) if isinstance(rowv, dict) else None
        return "absent" if rowv is None else _sha(_canon(rowv))

    def closure(self, p):
        """Return {dependency label: hash} for predicate `p`."""
        deps = closure_of(p)
        out = {"row": row_hash(p)}
        if "paths" in deps:
            out["paths"] = self.paths_hash(_as_list(deps["paths"]), deps.get("exclude", ()))
        for name in _as_list(deps.get("pins")):
            out[f"pins:{name}"] = self.pins_row(name)
        for name in _as_list(deps.get("chain")):
            out[f"chain:{name}"] = self.pins_row(name, "chain")
        for rel in _as_list(deps.get("manifests")):
            out[f"manifest:{rel}"] = self.file_hash(rel)
        for rel in _as_list(deps.get("receipts")):
            out[f"receipt:{rel}"] = self.file_hash(rel)
        return out


def stamp(preds, results, repo):
    """Record `head` (provenance) and the current `closure` on each passing result."""
    by_id = {p.id: p for p in preds}
    snap = Snapshot(repo)
    head = None
    for pid, entry in results.items():
        if entry.get("passes") is not True or pid not in by_id or "closure" in entry:
            continue
        if head is None:
            head = head_sha(repo)
        entry.setdefault("head", head)
        entry["closure"] = snap.closure(by_id[pid])
    return results


def stale_reasons(preds, entries, repo):
    """{id: [reason, ...]} for every passing entry that is not fresh.

    Rows are walked in `depends` order, so a row whose dependency is stale or not
    passing is stale too (`depends:<id>`).
    """
    from .predicates import topo_order

    snap = Snapshot(repo)
    out = {}
    for p in topo_order(preds):
        e = entries.get(p.id) or {}
        if e.get("passes") is not True:
            continue
        reasons = []
        rec = e.get("closure")
        if not isinstance(rec, dict):
            reasons.append(NO_CLOSURE)
        else:
            cur = snap.closure(p)
            reasons += [k for k in sorted(set(cur) | set(rec)) if cur.get(k) != rec.get(k)]
        for d in p.depends:
            if d in out or (entries.get(d) or {}).get("passes") is not True:
                reasons.append(f"depends:{d}")
        if reasons:
            out[p.id] = reasons
    return out
