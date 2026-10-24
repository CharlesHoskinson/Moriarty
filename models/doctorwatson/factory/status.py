"""qa/sherlock/status.json and runs.jsonl."""

import contextlib
import fcntl
import json
import os

from . import head_sha, now_iso, repo_root, sherlock_path
from .predicates import SETS, topo_order

SCHEMA = "doctorwatson.status/1"


def path(repo=None):
    return sherlock_path("status.json", repo)


def load(repo=None):
    """Return the status dict, or None if missing or unreadable."""
    try:
        with open(path(repo), encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, ValueError):
        return None
    return data if isinstance(data, dict) else None


def entries_of(status):
    if not status or not isinstance(status.get("predicates"), dict):
        return {}
    return status["predicates"]


def view(preds, entries):
    """Return (counts, firstFailing) from predicate definitions plus result entries."""
    counts = {s: [0, 0] for s in SETS}
    for p in preds:
        counts[p.set][1] += 1
        if (entries.get(p.id) or {}).get("passes") is True:
            counts[p.set][0] += 1
    first = None
    for p in topo_order(preds):
        e = entries.get(p.id) or {}
        if e.get("passes") is not True:
            first = {"id": p.id, "check": p.check, "exit": e.get("exit")}
            break
    return counts, first


def write_atomic(p, text):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    tmp = f"{p}.tmp.{os.getpid()}"
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.write(text)
    os.replace(tmp, p)


@contextlib.contextmanager
def _locked(repo):
    """Serialize read-modify-write of status.json across concurrent `check` runs."""
    lock = sherlock_path(".status.lock", repo)
    os.makedirs(os.path.dirname(lock), exist_ok=True)
    with open(lock, "a") as fh:
        fcntl.flock(fh, fcntl.LOCK_EX)
        try:
            yield
        finally:
            fcntl.flock(fh, fcntl.LOCK_UN)


def update(preds, results, increment, repo=None):
    repo = repo or repo_root()
    with _locked(repo):
        return _update(preds, results, increment, repo)


def _update(preds, results, increment, repo):
    """Merge `results` (id -> entry) into status.json and rewrite it atomically.

    A flip is a predicate changing passes false (or unknown) to true. A flip resets
    iterationsSinceFlip to 0 and sets lastFlip. Otherwise iterationsSinceFlip goes up
    by one when `increment` is true (one per `check` invocation).
    """
    prev = load(repo)
    ids = {p.id for p in preds}
    old = {k: v for k, v in entries_of(prev).items() if k in ids}
    entries = dict(old)
    flipped = None
    for pid, entry in results.items():
        if entry.get("passes") is True and (old.get(pid) or {}).get("passes") is not True:
            flipped = {"id": pid, "at": entry.get("at") or now_iso()}
        entries[pid] = entry
    last_flip = flipped or (prev or {}).get("lastFlip")
    since = int((prev or {}).get("iterationsSinceFlip") or 0)
    if flipped:
        since = 0
    elif increment:
        since += 1
    counts, first = view(preds, entries)
    status = {
        "schema": SCHEMA,
        "updated": now_iso(),
        "head": head_sha(repo),
        "counts": counts,
        "firstFailing": first,
        "predicates": entries,
        "lastFlip": last_flip,
        "iterationsSinceFlip": since,
    }
    write_atomic(path(repo), json.dumps(status, indent=2) + "\n")
    return status


def runs_path(repo=None):
    return sherlock_path("runs.jsonl", repo)


def append_run(record, repo=None):
    p = runs_path(repo)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "a", encoding="utf-8") as fh:
        fh.write(json.dumps(record, separators=(",", ":")) + "\n")


def read_runs(repo=None):
    out = []
    try:
        with open(runs_path(repo), encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                try:
                    rec = json.loads(line)
                except ValueError:
                    continue
                if isinstance(rec, dict):
                    out.append(rec)
    except OSError:
        pass
    return out
