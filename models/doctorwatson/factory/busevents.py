"""Pure builders and validators for `doctorwatson.bus/1` events. No I/O, no clock, no AgentBus.

A bus event is a doorbell: "file X changed at HEAD Y". It is never state (see CONTRACT.md,
section "AgentBus (v1)"). Bodies carry counts, ids, paths and a git sha, never free text.
"""

import hashlib
import json
import re

SCHEMA = "doctorwatson.bus/1"
MAX_BODY = 1024  # bytes, canonical JSON

# type -> (target, ttl seconds). '*' reaches every inbox; '#chan' is log-only until subscribed.
ROUTES = {
    "status.flip": ("*", 3600),
    "status.tick": ("#status", 900),
    "stop.escalate": ("*", 86400),
    "job.start": ("#jobs", 3600),
    "job.end": ("#jobs", 3600),
    "review.request": ("#review", 3600),
    "review.reply": ("*", 3600),
    "agent.start": ("#agents", 3600),
    "agent.heartbeat": ("#agents", 900),
    "agent.done": ("*", 86400),
}
TYPES = tuple(ROUTES)
WAKE_TYPES = ("stop.escalate", "review.reply", "status.flip")

# Per-type typed fields shown by `bus inbox`/`bus tail` (everything else is dropped).
SHOWN = {
    "status.flip": ("id", "flipped", "iterationsSinceFlip"),
    "status.tick": ("iterationsSinceFlip",),
    "stop.escalate": ("firstFailing", "trace", "iterationsSinceFlip"),
    "job.start": ("id", "slot", "budget"),
    "job.end": ("id", "exit", "wallSeconds", "trace"),
    "review.request": ("kind", "base", "range"),
    "review.reply": ("verdict", "findingsRef", "sha256"),
    "agent.start": ("name", "vendor", "role"),
    "agent.heartbeat": ("name", "bytes"),
    "agent.done": ("name", "exit", "wallSeconds", "modelServed"),
}

_SAFE_TOKEN = re.compile(r"^[A-Za-z0-9_.:/+=-]{0,200}$")
_HEX = re.compile(r"^[0-9a-f]{7,64}$|^unknown$")


def canonical(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def event_id(body):
    """sha256 over {schema,type,head,data}. `at` and `event_id` are excluded so that a
    re-send of the same logical event keeps its id (that is what dedupe keys on)."""
    core = {k: body.get(k) for k in ("schema", "type", "head", "data")}
    return hashlib.sha256(canonical(core).encode()).hexdigest()


def make_event(etype, data, head, at, role="driver"):
    body = {"schema": SCHEMA, "type": etype, "head": head or "unknown", "at": at,
            "role": role, "data": data}
    body["event_id"] = event_id(body)
    return body


def fit(body, limit=MAX_BODY):
    """Return body shrunk to <= limit bytes of canonical JSON, or None if it cannot fit.

    Shrinks by truncating lists, then dropping the longest string fields of `data`
    one by one. The event_id is recomputed because `data` changed.
    """
    def size(b):
        return len(canonical(b).encode())

    if size(body) <= limit:
        return body
    data = dict(body["data"])
    for _ in range(4):
        for k, v in list(data.items()):
            if isinstance(v, list) and len(v) > 1:
                data[k] = v[: max(1, len(v) // 2)]
        body = dict(body, data=data)
        body["event_id"] = event_id(body)
        if size(body) <= limit:
            return body
    while data and size(body) > limit:
        victim = max(data, key=lambda k: len(canonical(data[k])))
        del data[victim]
        body = dict(body, data=data)
        body["event_id"] = event_id(body)
    return body if size(body) <= limit else None


def _first_failing(st):
    ff = (st or {}).get("firstFailing") or {}
    return {"id": ff.get("id"), "exit": ff.get("exit")} if ff else None


def digest(st):
    """kv `status` value: a labelled projection of status.json, never read by a decision."""
    st = st or {}
    return {"schema": SCHEMA, "head": st.get("head"), "updated": st.get("updated"),
            "counts": st.get("counts"), "firstFailing": _first_failing(st),
            "iterationsSinceFlip": st.get("iterationsSinceFlip")}


def flipped_ids(prev_entries, results):
    out = []
    for pid, e in (results or {}).items():
        if (e or {}).get("passes") is True and ((prev_entries or {}).get(pid) or {}).get("passes") is not True:
            out.append(pid)
    return out


def check_event(prev_entries, new_status, results, head, at):
    """status.flip when any predicate turned to pass in this check, else status.tick."""
    flips = flipped_ids(prev_entries, results)
    base = {"counts": new_status.get("counts"), "firstFailing": _first_failing(new_status),
            "iterationsSinceFlip": new_status.get("iterationsSinceFlip"),
            "updated": new_status.get("updated")}
    if flips:
        data = dict(base, id=flips[-1], flipped=flips[:20],
                    run=((results.get(flips[-1]) or {}).get("run")))
        return make_event("status.flip", data, head, at)
    ran = sorted(results or {})[:20]
    return make_event("status.tick", dict(base, ran=ran), head, at)


def stop_key(result, st):
    ff = (_first_failing(st) or {}).get("id")
    return f"{ff}|{((result.get('lastFlip') or {}).get('at'))}"


def stop_event(result, st, head, at):
    data = {"reasons": [str(r)[:160] for r in (result.get("reasons") or [])][:3],
            "firstFailing": (_first_failing(st) or {}).get("id"),
            "trace": result.get("trace"),
            "iterationsSinceFlip": result.get("iterationsSinceFlip"),
            "lastFlip": result.get("lastFlip")}
    return make_event("stop.escalate", data, head, at)


def job_start_event(pid, slot, budget, expected_wall, has_hypothesis, head, at):
    return make_event("job.start", {"id": pid, "slot": slot, "budget": budget,
                                    "expectedWallSeconds": expected_wall,
                                    "hypothesis": bool(has_hypothesis)}, head, at)


def job_end_event(rec, slot, head, at):
    return make_event("job.end", {"id": rec.get("id"), "slot": slot, "exit": rec.get("exit"),
                                  "wallSeconds": rec.get("wallSeconds"),
                                  "maxRssBytes": rec.get("maxRssBytes"),
                                  "run": rec.get("run"), "trace": rec.get("trace")}, head, at)


def review_request_event(kind, base, rng, head, at):
    return make_event("review.request", {"kind": kind, "base": base, "range": rng}, head, at, role="lead")


def review_reply_event(corr, verdict, findings_ref, sha256, head, at):
    return make_event("review.reply", {"corr": corr, "verdict": verdict,
                                       "findingsRef": findings_ref, "sha256": sha256},
                      head, at, role="reviewer")


def agent_event(etype, name, role, head, at, **data):
    return make_event(etype, dict(data, name=name, role=role), head, at, role="worker")


def validate_event(body):
    """Return a list of schema errors (empty = valid)."""
    if not isinstance(body, dict):
        return ["body is not an object"]
    errs = []
    if body.get("schema") != SCHEMA:
        errs.append("schema")
    if body.get("type") not in TYPES:
        errs.append("type")
    if not isinstance(body.get("data"), dict):
        errs.append("data")
    if not isinstance(body.get("head"), str) or not _HEX.match(body["head"]):
        errs.append("head")
    if not isinstance(body.get("at"), str) or not re.match(r"^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$", body["at"]):
        errs.append("at")
    if body.get("event_id") != event_id(body):
        errs.append("event_id")
    if len(canonical(body).encode()) > MAX_BODY:
        errs.append("size")
    return errs


def _safe(v):
    if isinstance(v, bool) or v is None or isinstance(v, (int, float)):
        return v
    if isinstance(v, str):
        return v if _SAFE_TOKEN.match(v) else "(redacted)"
    if isinstance(v, list):
        return [_safe(x) for x in v[:20]]
    if isinstance(v, dict):
        return {str(k)[:32]: _safe(x) for k, x in list(v.items())[:8]}
    return None


def classify(env, seen=None, status_updated=None, pinned=None):
    """Classify one received AgentBus envelope (a dict). Returns (label, view).

    label is invalid | duplicate | stale | advisory | unpinned. view holds only typed,
    charset-checked fields; subject and body text are never passed through.
    `pinned` is roles.json's agents map (name -> {mayPublish}); a pinned sender publishing
    a type outside its list is invalid. An unpinned sender is flagged, not rejected.
    """
    name = str((env or {}).get("from_name") or "")
    name = name if re.match(r"^[A-Za-z0-9_.-]{1,64}$", name) else "(invalid-name)"
    try:
        body = json.loads(env.get("body") or "")
    except (ValueError, TypeError, AttributeError):
        return "invalid", {"from": name}
    if validate_event(body):
        return "invalid", {"from": name}
    etype = body["type"]
    pin = (pinned or {}).get(name)
    if pin is not None and etype not in (pin.get("mayPublish") or []):
        return "invalid", {"from": name, "type": etype}
    view = {"from": name, "type": etype, "head": body["head"][:12], "event": body["event_id"][:12]}
    for k in SHOWN[etype]:
        if k in body["data"]:
            view[k] = _safe(body["data"][k])
    if seen is not None:
        if body["event_id"] in seen:
            return "duplicate", view
        seen.add(body["event_id"])
    if status_updated and etype.startswith(("status.", "stop.")) and body["at"] < status_updated:
        return "stale", view
    return ("advisory" if pin is not None else "unpinned"), view


def render(label, view):
    parts = [f"[{label}]"] + [f"{k}={json.dumps(v, separators=(',', ':'))}" if not isinstance(v, str) else f"{k}={v}"
                              for k, v in view.items()]
    return " ".join(parts)
