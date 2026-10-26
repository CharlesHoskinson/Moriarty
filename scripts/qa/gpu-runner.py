#!/usr/bin/env python3
"""GPU runner: executes allowlisted cargo commands on the host GPU for sandboxed agents.

Sandboxed Codex agents cannot open the WSL GPU device. They send this runner a request
over AgentBus and get the result back:

    agentbus send gpu-runner 'RUN <tag>' --body '{"worktree": "<abs path>", "args": ["test", "--release", "--features", "cuda"]}'

The runner replies to the sender with subject 'RESULT <tag>' and a JSON body
{"exit", "wallSeconds", "log"}; the full log is written inside the worktree under
target/gpu-runs/ so the sandboxed agent can read it.

Add "heavy": true (optional "gpu": true, "budget": "KEY") to route the job through
`doctorwatson heavy --deadline`: the runner then owns heavy.lock (and gpu.lock) via that
supervisor, queues on a busy lock with a retry, and a timed-out job tree is killed (SIGTERM,
grace, SIGKILL by session and DW_JOB_ID marker) so the locks are always released.

Messages are data: the body is parsed as JSON, the worktree must sit under an allowed
root, the command is always `cargo <subcommand> ...` with subcommand in ALLOWED, it runs
without a shell, one job at a time (flock), with a wall-clock limit.
"""

import argparse
import fcntl
import json
import os
import re
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

ALLOWED = {"test", "bench", "run", "build", "check", "fetch", "generate-lockfile", "fuzz"}
SAFE_ARG = re.compile(r"^[A-Za-z0-9_.,:=/+@-]+$")
KILL_GRACE = 20  # seconds between SIGTERM and SIGKILL of a timed-out job tree
HEAVY_RETRY = 15  # seconds between retries while heavy/gpu lock is busy (heavy exit 3)
DOCTORWATSON = Path(__file__).resolve().parents[2] / "scripts" / "doctorwatson"
LOCK = Path.home() / ".cache" / "doctorwatson" / "gpu.lock"


def _session_members(sid, marker):
    mine = os.getpid()
    out = []
    for name in os.listdir("/proc"):
        if not name.isdigit() or int(name) == mine:
            continue
        try:
            with open(f"/proc/{name}/stat", "rb") as fh:
                st = fh.read().decode("utf-8", "replace")
            hit = int(st[st.rindex(")") + 2:].split()[3]) == sid
            if not hit and marker:
                with open(f"/proc/{name}/environ", "rb") as fh:
                    hit = f"DW_JOB_ID={marker}".encode() in fh.read().split(b"\0")
            if hit:
                out.append(int(name))
        except (OSError, ValueError, IndexError):
            continue
    return out


def kill_tree(sid, marker=None, grace=KILL_GRACE):
    """SIGTERM the job's process group (lets doctorwatson heavy kill its own tree and unlock), wait up
    to grace seconds, then SIGKILL the group and every process in the session or carrying the marker."""
    try:
        os.killpg(sid, signal.SIGTERM)
    except (ProcessLookupError, PermissionError):
        pass
    end = time.monotonic() + grace
    while time.monotonic() < end and _session_members(sid, marker):
        time.sleep(0.1)
    for _ in range(5):
        left = _session_members(sid, marker)
        if not left:
            break
        try:
            os.killpg(sid, signal.SIGKILL)
        except (ProcessLookupError, PermissionError):
            pass
        for pid in left:
            try:
                os.kill(pid, signal.SIGKILL)
            except OSError:
                pass
        time.sleep(0.1)


def run_job(argv, cwd, env, out, timeout, marker=None, grace=KILL_GRACE):
    """Run argv in its own session; on timeout kill the whole tree (not just the direct child). Returns rc, 124 on timeout."""
    env = dict(env, DW_WATCH_PARENT="1", DW_DEADLINE=str(time.time() + timeout))
    if marker:
        env["DW_JOB_ID"] = marker
    proc = subprocess.Popen(argv, cwd=cwd, env=env, stdout=out, stderr=subprocess.STDOUT,
                            start_new_session=True)
    try:
        rc = proc.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        kill_tree(proc.pid, marker, grace)
        proc.wait()
        return 124
    # the job returned; reap stragglers it left behind so they cannot keep locks
    if _session_members(proc.pid, marker):
        kill_tree(proc.pid, marker, grace=2)
    return rc


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
    via_heavy = bool(req.get("heavy"))
    sub = args[1] if args and args[0].startswith("+") and len(args) > 1 else (args[0] if args else "")
    if sub not in ALLOWED or not all(SAFE_ARG.match(a) for a in args):
        return reply(ident, sender, tag, {"exit": 2, "error": "command not allowed"})
    logdir = wt / "target" / "gpu-runs"
    logdir.mkdir(parents=True, exist_ok=True)
    log = logdir / f"{time.strftime('%Y%m%dT%H%M%S')}-{re.sub(r'[^A-Za-z0-9_-]', '_', tag)}.log"
    LOCK.parent.mkdir(parents=True, exist_ok=True)
    env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
    # Only GPU jobs take the device lock; fetch/check/CPU tests run at once.
    needs_gpu = bool(req.get("gpu")) if via_heavy else any("cuda" in a or "gpu" in a for a in args)
    marker = f"{os.getpid()}-{time.time_ns()}"
    if via_heavy:
        # The runner owns the locks: doctorwatson heavy takes heavy.lock (and gpu.lock with --gpu) in-process
        # with a deadline, so a timed-out job tree is killed and both locks are released together.
        budget = str(req.get("budget") or "")
        if budget and not SAFE_ARG.match(budget):
            return reply(ident, sender, tag, {"exit": 2, "error": "bad budget"})
        hid = re.sub(r"[^A-Za-z0-9_.-]", "_", f"gpu-runner-{tag}")[:80]
        deadline = time.time() + limit
        t0 = time.monotonic()
        queued = 0.0
        with open(log, "w") as out:
            while True:
                hv = [str(DOCTORWATSON), "heavy", "--id", hid, "--deadline", str(deadline)]
                if budget:
                    hv += ["--budget", budget]
                if needs_gpu:
                    hv.append("--gpu")
                hv += ["--", "cargo", *args]
                q = time.monotonic()
                rc = run_job(hv, wt, dict(env, DW_REPO=str(wt)), out, max(1, deadline - time.time()), marker)
                if rc != 3 or time.time() + HEAVY_RETRY >= deadline:  # 3 = lock busy / host unfit
                    break
                queued += time.monotonic() - q + HEAVY_RETRY
                time.sleep(HEAVY_RETRY)
        wall = round(time.monotonic() - t0 - queued, 3)
        queued = round(queued, 3)
        return reply(ident, sender, tag, {"exit": rc, "wallSeconds": wall, "queuedSeconds": queued,
                                          "gpu": needs_gpu, "log": str(log), "via": "heavy"})
    with open(LOCK, "a") as lk, open(log, "w") as out:
        q0 = time.monotonic()
        if needs_gpu:
            fcntl.flock(lk, fcntl.LOCK_EX)
        queued = round(time.monotonic() - q0, 3)
        t0 = time.monotonic()
        rc = run_job(["cargo", *args], wt, env, out, limit, marker)
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
        # Drain the queue and poll; the factory never blocks on a bus wait (F11).
        r = bus(a.identity, "recv", "--all", timeout=60)
        if r.returncode != 0:
            time.sleep(30)
            continue
        if not r.stdout.strip():
            time.sleep(5)
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
