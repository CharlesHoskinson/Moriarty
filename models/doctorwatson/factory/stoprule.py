"""Escalation rule: stop after 3 iterations without a flip or 24 h since the last one."""

import datetime
import json

from . import bus, repo_root, status

MAX_ITERATIONS = 3
MAX_AGE_HOURS = 24


def _parse(ts):
    try:
        return datetime.datetime.strptime(ts, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=datetime.timezone.utc)
    except (TypeError, ValueError):
        return None


def evaluate(repo=None, now=None):
    repo = repo or repo_root()
    st = status.load(repo) or {}
    iters = int(st.get("iterationsSinceFlip") or 0)
    flip = st.get("lastFlip")
    now = now or datetime.datetime.now(datetime.timezone.utc)
    reasons = []
    if iters >= MAX_ITERATIONS:
        reasons.append(f"{iters} iterations since the last flip (limit {MAX_ITERATIONS})")
    flip_at = _parse((flip or {}).get("at"))
    if flip_at is not None and now - flip_at > datetime.timedelta(hours=MAX_AGE_HOURS):
        hours = (now - flip_at).total_seconds() / 3600
        reasons.append(f"{hours:.1f} h since the last flip (limit {MAX_AGE_HOURS} h)")
    trace = None
    for rec in status.read_runs(repo):
        if rec.get("exit") not in (0, None) and rec.get("trace"):
            trace = rec["trace"]
    return {"stop": bool(reasons), "reasons": reasons, "iterationsSinceFlip": iters,
            "lastFlip": flip, "trace": trace}


def main(as_json=False):
    r = evaluate()
    if as_json:
        print(json.dumps(r, indent=2))
    elif r["stop"]:
        print("STOP: escalate to owner")
        for reason in r["reasons"]:
            print(f"  reason: {reason}")
        print(f"  last failing trace: {r['trace'] or '(none recorded)'}")
    else:
        lf = (r["lastFlip"] or {}).get("at") or "never"
        print(f"CONTINUE: iterationsSinceFlip={r['iterationsSinceFlip']} lastFlip={lf}")
    bus.safe(bus.hook_stop, r)  # doorbell only; never raises, prints or changes the exit code
    return 6 if r["stop"] else 0
