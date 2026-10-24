"""`doctorwatson spawn`: run one vendor CLI (claude | codex | grok) as a bus participant.

The wrapper, not the model, speaks on the bus: it mints or reuses a run identity, publishes
agent.start / agent.heartbeat / agent.done bookends, enforces the per-vendor concurrency
governor and appends one record to qa/sherlock/agents.jsonl. With the bus down, off or
unprovisionable it degrades to a plain run. The brief is passed to the vendor byte for byte;
no bus text ever reaches a prompt. Child API-key variables are removed (subscriptions only).
"""

import contextlib
import fcntl
import hashlib
import json
import os
import re
import shlex
import shutil
import signal
import subprocess
import sys
import threading
import time

from . import bus, head_sha, now_iso, preflight, repo_root, sherlock_path
from . import busevents as ev

SCRUB = ("OPENAI_API_KEY", "CODEX_API_KEY", "XAI_API_KEY")
ROLES = ("implementer", "quint", "auditor", "lead", "helper")
EFFORTS = ("low", "medium", "high", "xhigh")
EXIT_USAGE = 2
EXIT_NO_SLOT = 10
EXIT_NO_BIN = 127
NAME_RE = re.compile(r"^[a-z][a-z0-9_-]{0,31}$")
VENDORS = {
    "codex": {"env": "DW_CODEX_BIN", "bin": "codex", "model": "gpt-6.1-sol", "slots_env": "DW_SLOTS_CODEX", "slots": 10},
    "grok": {"env": "DW_GROK_BIN", "bin": "grok", "model": "grok-4.7", "slots_env": "DW_SLOTS_GROK", "slots": 4},
    "claude": {"env": "DW_CLAUDE_BIN", "bin": "claude", "model": None, "slots_env": "DW_SLOTS_CLAUDE", "slots": 4},
}


def vendor_dir():
    return os.path.join(os.path.dirname(preflight.lock_path()), "vendor")


def slot_limit(vendor):
    v = VENDORS[vendor]
    try:
        return max(1, int(os.environ.get(v["slots_env"], v["slots"])))
    except ValueError:
        return v["slots"]


def acquire_slot(vendor, wait_s=0.0):
    """flock-counted governor: hold one of N lock files for the child's lifetime.
    Returns (fd, n) or None when all N are busy after wait_s. A governor I/O error fails open."""
    d = vendor_dir()
    try:
        os.makedirs(d, exist_ok=True)
    except OSError:
        return -1, -1
    limit = slot_limit(vendor)
    deadline = time.monotonic() + wait_s
    while True:
        for n in range(limit):
            try:
                fd = os.open(os.path.join(d, f"{vendor}.{n}.lock"), os.O_RDWR | os.O_CREAT, 0o644)
            except OSError:
                return -1, -1
            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                return fd, n
            except OSError:
                os.close(fd)
        if time.monotonic() >= deadline:
            return None
        time.sleep(0.25)


def release_slot(slot):
    if slot and slot[0] >= 0:
        with contextlib.suppress(OSError):
            fcntl.flock(slot[0], fcntl.LOCK_UN)
            os.close(slot[0])


def child_env(base=None):
    env = dict(os.environ if base is None else base)
    for k in SCRUB:
        env.pop(k, None)
    for k in ("AGENTBUS_IDENTITY_DIR", "AGENTBUS_RUN", "AGENTBUS_AGENT"):
        env.pop(k, None)
    return env


def build_argv(vendor, brief, out_dir, repo, effort, model, sandbox, extra):
    """Return (argv, stdin_path or None). Locates the vendor binary via DW_<V>_BIN, else PATH."""
    v = VENDORS[vendor]
    exe = os.environ.get(v["env"]) or v["bin"]
    model = model or v["model"]
    if vendor == "codex":
        argv = [exe, "exec", "-m", model, "-c", f'model_reasoning_effort="{effort}"', "-s", sandbox,
                "--skip-git-repo-check", "--ephemeral", "-C", repo, "--json",
                "-o", os.path.join(out_dir, "last.txt"), *extra, "-"]
        return argv, brief
    if vendor == "grok":
        mode = ["--sandbox", "read-only"] if sandbox == "read-only" else ["--allow", "Write", "--allow", "Edit", "--no-leader"]
        argv = [exe, "--no-auto-update", "--prompt-file", brief, "-m", model, "--effort", effort,
                "--output-format", "json", *mode, "--cwd", repo, *extra]
        return argv, None
    argv = [exe, "-p", "--output-format", "text", "--permission-mode",
            "plan" if sandbox == "read-only" else "acceptEdits"]
    if model:
        argv += ["--model", model]
    return argv + list(extra), brief


def served_model(out_dir):
    try:
        with open(os.path.join(out_dir, "stderr.log"), "rb") as fh:
            m = re.search(rb"(?m)^model:\s*([\w.\-]+)", fh.read(1 << 20))
        if m:
            return m.group(1).decode()
        with open(os.path.join(out_dir, "stdout.log"), "rb") as fh:
            fh.seek(0, 2)
            fh.seek(max(0, fh.tell() - 65536))
            m = re.findall(rb'"model"\s*:\s*"([^"]+)"', fh.read())
        return m[-1].decode() if m else None
    except OSError:
        return None


def sha256_file(path):
    try:
        with open(path, "rb") as fh:
            return hashlib.sha256(fh.read()).hexdigest()
    except OSError:
        return None


def agents_log_path(repo):
    return os.environ.get("DW_AGENTS_LOG") or sherlock_path("agents.jsonl", repo)


def append_agent(rec, repo):
    p = agents_log_path(repo)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "a", encoding="utf-8") as fh:
        fcntl.flock(fh, fcntl.LOCK_EX)
        fh.write(json.dumps(rec, separators=(",", ":")) + "\n")
        fh.flush()


def _identity_root(repo):
    return os.environ.get("DW_SPAWN_IDENTITIES") or os.path.join(repo, ".agentbus", "agents")


def _spawn_spec(repo):
    return (bus.load_roles(repo).get("spawned") or {})


def _bookend(etype, ident, repo, name, role, **data):
    """Publish one agent.* event under the worker's own identity. Never raises."""
    if ident is None:
        return False
    try:
        body = ev.fit(ev.agent_event(etype, name, role, head_sha(repo), now_iso(), **data))
        return bool(bus.publish_ex(etype, None, repo=repo, ident=ident, body=body)[0])
    except Exception:
        return False


def run(a):
    """Entry point. `a` has vendor, name, role, brief, effort, model, sandbox, out, slot_wait,
    dry_run, extra. Returns the child's exit code (2 usage, 10 no slot, 127 no binary)."""
    repo = repo_root()
    if not NAME_RE.match(a.name or ""):
        print("error: --as NAME must match [a-z][a-z0-9_-]{0,31}", file=sys.stderr)
        return EXIT_USAGE
    if not os.path.isfile(a.brief):
        print(f"error: brief not found: {a.brief}", file=sys.stderr)
        return EXIT_USAGE
    brief = os.path.abspath(a.brief)
    sandbox = a.sandbox or ("workspace-write" if a.role in ("implementer", "lead") else "read-only")
    stamp = time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())
    out_dir = os.path.abspath(a.out) if a.out else os.path.join(repo, "qa", "sherlock", "agents", f"{a.name}-{stamp}")
    argv, stdin_path = build_argv(a.vendor, brief, out_dir, repo, a.effort, a.model, sandbox, a.extra)
    if a.dry_run:
        print(json.dumps({"argv": argv, "stdin": stdin_path, "cwd": repo, "out": out_dir, "sandbox": sandbox,
                          "removedEnv": [k for k in SCRUB],
                          "cmd": shlex.join(argv)}, indent=2))
        return 0
    if not shutil.which(argv[0]) and not os.path.isfile(argv[0]):
        print(f"error: vendor binary not found: {argv[0]}", file=sys.stderr)
        return EXIT_NO_BIN
    os.makedirs(out_dir, exist_ok=True)

    slot = acquire_slot(a.vendor, a.slot_wait)
    if slot is None:
        print(f"error: all {slot_limit(a.vendor)} {a.vendor} slots busy (exit {EXIT_NO_SLOT}); "
              f"raise {VENDORS[a.vendor]['slots_env']} or retry with --slot-wait SECS", file=sys.stderr)
        return EXIT_NO_SLOT

    # Bus participation: best effort, degrades to a plain run.
    bus_state = "off"
    ident = None
    try:
        if bus.enabled() and shutil.which("agentbus") and not a.no_bus:
            spec = _spawn_spec(repo)
            ident = bus.mint(a.name, a.role, os.path.join(_identity_root(repo), a.name),
                             chans=spec.get("chans", ["agents"]), claims=spec.get("claims", []))
            bus_state = "on" if ident else "down"
    except Exception:
        ident, bus_state = None, "down"
    env = child_env()
    if ident:
        env.update(AGENTBUS_IDENTITY_DIR=ident.dir)
    if ident and not _bookend("agent.start", ident, repo, a.name, a.role, vendor=a.vendor,
                              modelRequested=a.model or VENDORS[a.vendor]["model"], effort=a.effort):
        bus_state = "down"

    t0 = time.monotonic()
    code = None
    proc = None
    stop_hb = threading.Event()
    old = signal.signal(signal.SIGTERM, lambda *_: (_ for _ in ()).throw(KeyboardInterrupt()))
    try:
        stdin = open(stdin_path, "rb") if stdin_path else subprocess.DEVNULL
        with open(os.path.join(out_dir, "stdout.log"), "wb") as so, open(os.path.join(out_dir, "stderr.log"), "wb") as se:
            try:
                proc = subprocess.Popen(argv, cwd=repo, env=env, stdin=stdin, stdout=so, stderr=se,
                                        start_new_session=True)
            except OSError as exc:
                se.write(f"cannot start {argv[0]!r}: {exc}\n".encode())
                code = EXIT_NO_BIN
            if hasattr(stdin, "close"):
                stdin.close()
            if proc is not None:
                if ident:
                    def beat():
                        last = -1
                        interval = float(os.environ.get("DW_SPAWN_HEARTBEAT", "300"))
                        while not stop_hb.wait(interval):
                            size = sum(os.path.getsize(os.path.join(out_dir, f)) for f in ("stdout.log", "stderr.log"))
                            if size != last:  # only when the stream moved
                                last = size
                                _bookend("agent.heartbeat", ident, repo, a.name, a.role, bytes=size)
                    threading.Thread(target=beat, daemon=True).start()
                try:
                    st = proc.wait()
                    code = st if st >= 0 else 128 - st
                except BaseException:
                    with contextlib.suppress(Exception):
                        os.killpg(proc.pid, signal.SIGKILL)
                    with contextlib.suppress(Exception):
                        proc.wait(5)
                    code = 130
                    raise
                finally:
                    with contextlib.suppress(Exception):
                        os.killpg(proc.pid, signal.SIGKILL)  # stragglers
    finally:
        stop_hb.set()
        signal.signal(signal.SIGTERM, old)
        wall = round(time.monotonic() - t0, 3)
        code = 130 if code is None else code
        model_served = served_model(out_dir)
        out_sha = sha256_file(os.path.join(out_dir, "last.txt")) or sha256_file(os.path.join(out_dir, "stdout.log"))
        if ident and not _bookend("agent.done", ident, repo, a.name, a.role, exit=code, wallSeconds=wall,
                                  modelServed=model_served, outSha256=out_sha):
            bus_state = "down" if bus_state == "on" else bus_state
        release_slot(slot)
        with contextlib.suppress(Exception):
            append_agent({
                "schema": "doctorwatson.agent/1", "name": a.name, "role": a.role, "vendor": a.vendor,
                "modelRequested": a.model or VENDORS[a.vendor]["model"], "modelServed": model_served,
                "effort": a.effort, "sandbox": sandbox, "exit": code, "wallSeconds": wall,
                "bus": bus_state, "did": ident.did if ident else None, "slot": slot[1] if slot else None,
                "brief": brief, "briefSha256": sha256_file(brief), "out": out_dir, "outSha256": out_sha,
                "head": head_sha(repo), "at": now_iso(), "run": bus.run_name() if bus_state != "off" else None,
            }, repo)
    return code


def main(a):
    try:
        return run(a)
    except KeyboardInterrupt:
        return 130
