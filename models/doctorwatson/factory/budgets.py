"""qa/sherlock/budgets.json: measured costs and caps (measure, then cap)."""

import json
import math
import os

from . import repo_root, sherlock_path

SCHEMA = "doctorwatson.budgets/1"


def path(repo=None):
    return sherlock_path("budgets.json", repo)


def load(repo=None):
    try:
        with open(path(repo), encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, ValueError):
        data = {}
    if not isinstance(data, dict):
        data = {}
    data.setdefault("schema", SCHEMA)
    if not isinstance(data.get("budgets"), dict):
        data["budgets"] = {}
    return data


def save(data, repo=None):
    p = path(repo)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    tmp = f"{p}.tmp.{os.getpid()}"
    with open(tmp, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2)
        fh.write("\n")
    os.replace(tmp, p)


def is_measured(entry):
    return bool(entry) and (entry.get("measured") or {}).get("wallSeconds") is not None


def is_fixed(entry):
    """E17: a `fixed: true` entry is a requirement cap, enforced from the first run, never rewritten."""
    return bool(entry) and entry.get("fixed") is True


def limits_for(key, repo=None):
    """Return the cap dict {wallSeconds, maxRssBytes[, aggregateRssBytes][, fixed: True]}, or None while unmeasured.

    A fixed entry returns its cap even while `measured.wallSeconds` is null.
    """
    if not key:
        return None
    entry = load(repo or repo_root())["budgets"].get(key)
    if not (is_measured(entry) or is_fixed(entry)):
        return None
    cap = entry.get("cap") or {}
    out = {"wallSeconds": cap.get("wallSeconds"), "maxRssBytes": cap.get("maxRssBytes")}
    if is_fixed(entry):
        out["fixed"] = True
    if cap.get("aggregateRssBytes") is not None:
        out["aggregateRssBytes"] = cap["aggregateRssBytes"]
    return out


def record_measurement(key, wall_seconds, max_rss_bytes, run, at, repo=None, aggregate_rss_bytes=None,
                       concurrency=None):
    """Store the measurement; set cap = ceil(2 x measured) unless the entry is fixed. Keeps any seeded `k`."""
    repo = repo or repo_root()
    data = load(repo)
    entry = data["budgets"].setdefault(key, {})
    old = entry.get("measured") or {}
    wall = round(wall_seconds, 3)
    entry["measured"] = {
        "wallSeconds": wall,
        "maxRssBytes": int(max_rss_bytes),
        "k": old.get("k"),
        "at": at,
        "run": run,
    }
    if concurrency:
        entry["measured"].update(aggregateRssBytes=int(aggregate_rss_bytes or max_rss_bytes),
                                 concurrency=int(concurrency))
    if is_fixed(entry):
        save(data, repo)
        return entry.get("cap")
    entry["cap"] = {
        "wallSeconds": math.ceil(2 * wall),
        "maxRssBytes": math.ceil(2 * int(max_rss_bytes)),
    }
    save(data, repo)
    return entry["cap"]
