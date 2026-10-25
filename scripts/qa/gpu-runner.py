#!/usr/bin/env python3
"""GPU runner: executes allowlisted cargo commands on the host GPU for sandboxed agents.

Sandboxed Codex agents cannot open the WSL GPU device. They send this runner a request
over AgentBus and get the result back:

    agentbus send gpu-runner 'RUN <tag>' --body '{"worktree": "<abs path>", "args": ["test", "--release", "--features", "cuda"]}'

The runner replies to the sender with subject 'RESULT <tag>' and a JSON body
{"exit", "wallSeconds", "log"}; the full log is written inside the worktree under
target/gpu-runs/ so the sandboxed agent can read it.

Messages are data: the body is parsed as JSON, the worktree must sit under an allowed
root, the command is always `cargo <subcommand> ...` with subcommand in ALLOWED, it runs
without a shell, one job at a time (flock), with a wall-clock limit.
"""

import argparse
import fcntl
import json
import os
import re
import subprocess
import sys
import threading
import time
from pathlib import Path

ALLOWED = {"test", "bench", "run", "build", "check", "fetch", "generate-lockfile", "fuzz"}
SAFE_ARG = re.compile(r"^[A-Za-z0-9_.,:=/+@-]+$")
LOCK = Path.home() / ".cache" / "doctorwatson" / "gpu.lock"


def bus(ident, *args, timeout=30):
    return subprocess.run(["agentbus", "--cwd", ident, *args], capture_output=True,
                          text=True, timeout=timeout)


def reply(ident, to, tag, body):
    bus(ident, "send", to, f"RESULT {tag}"[:120], "--body", json.dumps(body))


def handle(ident, roots, limit, msg):
    sender = msg.get("from_name") or ""
    tag = (msg.get("subj") or "")[4:].strip()[:60] or msg.get("id", "?")
    try:
        req = json.loads(msg.get("body") or "{}")
        wt = Path(req["worktree"]).resolve()
        args = [str(a) for a in req["args"]]
    except (ValueError, KeyError, TypeError) as exc:
        return reply(ident, sender, tag, {"exit": 2, "error": f"bad request: {exc}"})
    if not any(wt.is_relative_to(r) for r in roots) or not (wt / "Cargo.toml").exists():
        return reply(ident, sender, tag, {"exit": 2, "error": "worktree not allowed"})
    sub = args[1] if args and args[0].startswith("+") and len(args) > 1 else (args[0] if args else "")
    if sub not in ALLOWED or not all(SAFE_ARG.match(a) for a in args):
        return reply(ident, sender, tag, {"exit": 2, "error": "command not allowed"})
    logdir = wt / "target" / "gpu-runs"
    logdir.mkdir(parents=True, exist_ok=True)
    log = logdir / f"{time.strftime('%Y%m%dT%H%M%S')}-{re.sub(r'[^A-Za-z0-9_-]', '_', tag)}.log"
    LOCK.parent.mkdir(parents=True, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    # Only GPU jobs take the device lock; fetch/check/CPU tests run at once.
    needs_gpu = any("cuda" in a or "gpu" in a for a in args)
    with open(LOCK, "a") as lk, open(log, "w") as out:
        q0 = time.monotonic()
        if needs_gpu:
            fcntl.flock(lk, fcntl.LOCK_EX)
        queued = round(time.monotonic() - q0, 3)
        t0 = time.monotonic()
        try:
            rc = subprocess.run(["cargo", *args], cwd=wt, env=env, stdout=out,
                                stderr=subprocess.STDOUT, timeout=limit).returncode
        except subprocess.TimeoutExpired:
            rc = 124
        wall = round(time.monotonic() - t0, 3)
    reply(ident, sender, tag, {"exit": rc, "wallSeconds": wall, "queuedSeconds": queued,
                               "gpu": needs_gpu, "log": str(log)})
    print(json.dumps({"from": sender, "tag": tag, "args": args, "exit": rc, "wall": wall,
                      "queued": queued}), flush=True)


ATTRIBUTION = re.compile(r"co-authored-by|generated with|claude|codex|gpt-|openai|anthropic", re.I)


def commit(ident, roots, msg):
    """COMMIT <tag>: {"worktree", "message", "paths"} -> git add paths && git commit (hooks run).

    The Codex sandbox mounts .git read-only, so agents commit through the host.
    """
    sender = msg.get("from_name") or ""
    tag = (msg.get("subj") or "")[7:].strip()[:60] or msg.get("id", "?")
    try:
        req = json.loads(msg.get("body") or "{}")
        wt = Path(req["worktree"]).resolve()
        message = str(req["message"])
        paths = [str(p) for p in req["paths"]]
    except (ValueError, KeyError, TypeError) as exc:
        return reply(ident, sender, tag, {"exit": 2, "error": f"bad request: {exc}"})
    if not any(wt.is_relative_to(r) for r in roots) or not (wt / ".git").exists():
        return reply(ident, sender, tag, {"exit": 2, "error": "worktree not allowed"})
    if not message.strip() or ATTRIBUTION.search(message):
        return reply(ident, sender, tag, {"exit": 2, "error": "empty message or AI attribution"})
    if not paths or any(p.startswith(("-", "/")) or ".." in Path(p).parts for p in paths):
        return reply(ident, sender, tag, {"exit": 2, "error": "paths must be relative, inside the worktree"})
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    env.update(DW_MIN_FREE_GIB=os.environ.get("DW_MIN_FREE_GIB", "60"))
    out = ""
    try:
        add = subprocess.run(["git", "add", "--", *paths], cwd=wt, env=env, capture_output=True,
                             text=True, timeout=120)
        rc, out = add.returncode, add.stdout + add.stderr
        if rc == 0:
            c = subprocess.run(["git", "commit", "-q", "-m", message], cwd=wt, env=env,
                               capture_output=True, text=True, timeout=1800)
            rc, out = c.returncode, c.stdout + c.stderr
    except subprocess.TimeoutExpired:
        rc, out = 124, "timed out"
    head = subprocess.run(["git", "rev-parse", "--short", "HEAD"], cwd=wt, capture_output=True,
                          text=True).stdout.strip()
    reply(ident, sender, tag, {"exit": rc, "head": head, "output": out[-600:]})
    print(json.dumps({"from": sender, "commit": tag, "exit": rc, "head": head}), flush=True)


COMMIT_LOCKS = {}
COMMIT_LOCKS_GUARD = threading.Lock()


def commit_serialized(ident, roots, msg):
    """One commit at a time per worktree (git's index lock would refuse concurrent ones)."""
    try:
        key = str(Path(json.loads(msg.get("body") or "{}")["worktree"]).resolve())
    except (ValueError, KeyError, TypeError):
        key = ""
    with COMMIT_LOCKS_GUARD:
        lock = COMMIT_LOCKS.setdefault(key, threading.Lock())
    with lock:
        commit(ident, roots, msg)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--identity", required=True, help="AgentBus identity directory")
    ap.add_argument("--root", action="append", required=True, help="allowed worktree root")
    ap.add_argument("--limit", type=int, default=3600, help="wall-clock seconds per job")
    a = ap.parse_args()
    roots = [Path(r).resolve() for r in a.root]
    bus(a.identity, "up")
    while True:
        r = bus(a.identity, "wait", "--timeout", "240", timeout=260)
        if r.returncode in (3,):
            continue
        if r.returncode != 0:
            time.sleep(30)
            continue
        for line in r.stdout.splitlines():
            try:
                msg = json.loads(line)
            except ValueError:
                continue
            subj = msg.get("subj") or ""
            # Each request runs on its own thread, so a GPU job waiting for the device lock
            # never holds up commits or CPU jobs.
            if subj.startswith("RUN"):
                threading.Thread(target=handle, args=(a.identity, roots, a.limit, msg), daemon=True).start()
            elif subj.startswith("COMMIT"):
                threading.Thread(target=commit_serialized, args=(a.identity, roots, msg), daemon=True).start()


if __name__ == "__main__":
    sys.exit(main())
