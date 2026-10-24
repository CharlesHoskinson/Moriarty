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


def limits_for(key, repo=None):
    """Return the cap dict {wallSeconds, maxRssBytes}, or None while unmeasured."""
    if not key:
        return None
    entry = load(repo or repo_root())["budgets"].get(key)
    if not is_measured(entry):
        return None
    cap = entry.get("cap") or {}
    return {"wallSeconds": cap.get("wallSeconds"), "maxRssBytes": cap.get("maxRssBytes")}


def record_measurement(key, wall_seconds, max_rss_bytes, run, at, repo=None):
    """Store the measurement and set cap = ceil(2 x measured). Keeps any seeded `k`."""
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
    entry["cap"] = {
        "wallSeconds": math.ceil(2 * wall),
        "maxRssBytes": math.ceil(2 * int(max_rss_bytes)),
    }
    save(data, repo)
    return entry["cap"]
