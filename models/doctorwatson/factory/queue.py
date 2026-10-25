"""FIFO ticket queue for the heavy lock: ordered admission, priorities, leases, status.

State lives in DW_QUEUE_DIR (default ~/.cache/doctorwatson/queue) as queue.json guarded by
queue.lock. The heavy flock stays the authority on mutual exclusion; the queue decides *who
tries next*. heavy.py calls acquire_turn() instead of failing with "host unfit" and finish()
after the run. Priorities and reordering are coordinator-only (DW_ROLE=coordinator).
"""

import contextlib
import fcntl
import getpass
import json
import os
import sys
import time

DEFAULT_WALL = 3600
TERMINAL = ("done", "cancelled", "crashed")
_mine = {}


# ------------------------------------------------------------------ state
def qdir():
    return os.environ.get("DW_QUEUE_DIR") or os.path.expanduser("~/.cache/doctorwatson/queue")


def _empty():
    return {"schema": "doctorwatson.queue/1", "seq": 0, "hold": False, "tickets": []}


def load():
    try:
        with open(os.path.join(qdir(), "queue.json")) as fh:
            return json.load(fh)
    except (OSError, ValueError):
        return _empty()


@contextlib.contextmanager
def _state():
    os.makedirs(qdir(), exist_ok=True)
    lk = os.open(os.path.join(qdir(), "queue.lock"), os.O_RDWR | os.O_CREAT, 0o644)
    try:
        fcntl.flock(lk, fcntl.LOCK_EX)
        st = load()
        yield st
        tmp = os.path.join(qdir(), "queue.json.tmp%d" % os.getpid())
        with open(tmp, "w") as fh:
            json.dump(st, fh, indent=1)
        os.replace(tmp, os.path.join(qdir(), "queue.json"))
    finally:
        os.close(lk)


def me():
    return os.environ.get("DW_AGENT") or getpass.getuser()


def is_coordinator():
    return os.environ.get("DW_ROLE") == "coordinator"


def _pid_start(pid):
    try:
        with open("/proc/%d/stat" % pid, "rb") as fh:
            s = fh.read().decode("utf-8", "replace")
        return int(s[s.rindex(")") + 2:].split()[19])
    except (OSError, ValueError, IndexError):
        return None


def _alive(pid, start):
    cur = _pid_start(pid)
    return cur is not None and (start is None or cur == start)


def _reap(st):
    """Mark tickets whose owning process is gone: running -> crashed, waiting -> cancelled."""
    for t in st["tickets"]:
        if t["state"] == "running" and not _alive(t["pid"], t.get("pidStart")):
            t.update(state="crashed", ended=time.time())
        elif t["state"] == "queued" and t.get("waiterPid") and not _alive(t["waiterPid"], t.get("waiterStart")):
            t.update(state="cancelled", ended=time.time(), why="waiter exited")


# ------------------------------------------------------------------ ordering
def resolve(st, ref):
    for t in st["tickets"]:
        if t["id"] == ref:
            return t
    hits = [t for t in st["tickets"] if t["label"] == ref]
    live = [t for t in hits if t["state"] not in TERMINAL]
    return (live or hits or [None])[-1]


def _blocked(st, t):
    """Reason the ticket can never run (dependency failed), or None."""
    for ref in t.get("after") or []:
        d = resolve(st, ref)
        if d is None:
            return "dependency %s unknown" % ref
        if d["state"] in ("cancelled", "crashed"):
            return "dependency %s %s" % (d["id"], d["state"])
        if d["state"] == "done" and d.get("exit") != 0:
            return "dependency %s failed" % d["id"]
    return None


def _ready(st, t):
    if t["state"] != "queued" or t.get("held") or _blocked(st, t):
        return False
    return all((resolve(st, r) or {}).get("state") == "done" for r in t.get("after") or [])


def ordered(st):
    q = [t for t in st["tickets"] if t["state"] == "queued"]
    return sorted(q, key=lambda t: (-t.get("prio", 0), t.get("rank", 0)))


def head(st):
    if st.get("hold"):
        return None
    for t in ordered(st):
        if _ready(st, t):
            return t
    return None


def _running(st):
    return next((t for t in st["tickets"] if t["state"] == "running"), None)


# ------------------------------------------------------------------ operations
class QueueError(Exception):
    pass


def _new(st, label, cmd, max_wall, prio, after, budget, needs, waiter=False):
    if not max_wall or max_wall <= 0:
        raise QueueError("admission refused: --max-wall SECONDS is required")
    if prio and not is_coordinator():
        raise QueueError("priority is coordinator-only (DW_ROLE=coordinator)")
    st["seq"] += 1
    t = {"id": "t%04d-%s" % (st["seq"], label), "label": label, "cmd": cmd, "maxWall": max_wall,
         "prio": prio or 0, "rank": float(st["seq"]), "after": list(after or []), "budget": budget,
         "needs": needs or ["heavy"], "owner": me(), "state": "queued", "held": False,
         "submitted": time.time()}
    if waiter:
        t.update(waiterPid=os.getpid(), waiterStart=_pid_start(os.getpid()))
    st["tickets"].append(t)
    return t


def submit(label, max_wall, cmd=None, prio=0, after=None, budget=None, needs=None):
    with _state() as st:
        _reap(st)
        t = _new(st, label, cmd, max_wall, prio, after, budget, needs)
        pos = [x["id"] for x in ordered(st)].index(t["id"]) + 1
        return t["id"], pos


def lock_all(fds, try_lock=None):
    """try_lock for several flocks (heavy then gpu): all or nothing, never holds a partial set."""
    if try_lock is None:
        from . import preflight
        try_lock = preflight.try_lock
    got = []
    for f in fds:
        if not try_lock(f):
            for g in got:
                fcntl.flock(g, fcntl.LOCK_UN)
            return False
        got.append(f)
    return True


def acquire_turn(fd, label, cmd, max_wall=None, try_lock=None, poll=1.0, timeout=None, after=None, budget=None):
    """Wait for this job's turn, then take the heavy flock on fd. Returns (ok, reason).

    Reuses a pre-submitted queued ticket with the same label, else files one. Falls back to the
    old non-blocking behaviour when DW_QUEUE=off.
    """
    if try_lock is None:
        from . import preflight
        try_lock = preflight.try_lock
    if os.environ.get("DW_QUEUE") == "off":
        return (True, None) if try_lock(fd) else (False, "host unfit: another process holds the heavy lock")
    if timeout is None and os.environ.get("DW_QUEUE_TIMEOUT"):
        timeout = float(os.environ["DW_QUEUE_TIMEOUT"])
    wall = max_wall or float(os.environ.get("DW_QUEUE_DEFAULT_WALL", DEFAULT_WALL))
    with _state() as st:
        _reap(st)
        t = next((x for x in ordered(st) if x["label"] == label), None)
        if t is None:
            t = _new(st, label, cmd, wall, 0, after, budget, None, waiter=True)
        else:
            t.update(waiterPid=os.getpid(), waiterStart=_pid_start(os.getpid()))
        tid = t["id"]
    deadline = None if timeout is None else time.monotonic() + timeout
    try:
        while True:
            with _state() as st:
                _reap(st)
                t = resolve(st, tid)
                if t["state"] != "queued":
                    return False, "ticket %s is %s: %s" % (tid, t["state"], t.get("why", ""))
                h = head(st)
                if h is not None and h["id"] == tid and _running(st) is None and try_lock(fd):
                    now = time.time()
                    t.update(state="running", pid=os.getpid(), pidStart=_pid_start(os.getpid()),
                             started=now, deadline=now + t["maxWall"], maxWall=t["maxWall"])
                    _mine[label] = tid
                    return True, None
                if deadline is not None and time.monotonic() >= deadline:
                    t.update(state="cancelled", ended=time.time(), why="timeout")
                    return False, "timeout waiting for turn on ticket %s (head: %s)" % (
                        tid, h["id"] if h else ("held" if st.get("hold") else "none"))
            time.sleep(poll)
    except BaseException:
        with _state() as st:
            t = resolve(st, tid)
            if t and t["state"] == "queued":
                t.update(state="cancelled", ended=time.time(), why="waiter interrupted")
        raise


def finish(ref, exit_code):
    with _state() as st:
        t = resolve(st, _mine.get(ref, ref))
        if t and t["state"] == "running":
            t.update(state="done", exit=exit_code, ended=time.time())
            return True
    return False


def _need_coordinator(what):
    if not is_coordinator():
        raise QueueError("%s is coordinator-only (DW_ROLE=coordinator)" % what)


def cancel(ref):
    with _state() as st:
        t = resolve(st, ref)
        if t is None or t["state"] in TERMINAL:
            raise QueueError("no live ticket %s" % ref)
        if t["state"] == "running":
            raise QueueError("%s is running; cancel does not signal processes (lease/reap handles it)" % t["id"])
        if t["owner"] != me():
            _need_coordinator("cancelling another agent's ticket")
        t.update(state="cancelled", ended=time.time(), why="cancelled by %s" % me())
        return t["id"]


def reorder(ref, first=False, before=None, prio=None):
    _need_coordinator("reorder")
    with _state() as st:
        t = resolve(st, ref)
        if t is None or t["state"] != "queued":
            raise QueueError("no queued ticket %s" % ref)
        ranks = [x.get("rank", 0) for x in st["tickets"] if x["state"] == "queued" and x is not t]
        if first:
            t["rank"] = min(ranks + [t["rank"]]) - 1
        if before:
            o = resolve(st, before)
            if o is None or o["state"] != "queued":
                raise QueueError("no queued ticket %s" % before)
            t["rank"] = o["rank"] - 0.5
        if prio is not None:
            t["prio"] = prio
        return t["id"]


def hold(ref=None, all_=False, on=True):
    _need_coordinator("hold/release")
    with _state() as st:
        if all_:
            st["hold"] = on
            return "all"
        t = resolve(st, ref)
        if t is None or t["state"] != "queued":
            raise QueueError("no queued ticket %s" % ref)
        t["held"] = on
        return t["id"]


def status(st=None):
    st = st or load()
    now = time.time()
    r = _running(st)
    run = None
    if r:
        run = dict(r, elapsed=round(now - r["started"]), overrun=now > r["deadline"])
    h = head(st)
    q = []
    for t in ordered(st):
        q.append(dict(t, blocked=_blocked(st, t), ready=_ready(st, t), held=bool(t.get("held")),
                      waitedSeconds=round(now - t["submitted"])))
    return {"head": h["id"] if h else None, "hold": bool(st.get("hold")), "running": run, "queue": q}


def format_status(s):
    out = ["queue HOLD (nothing admitted)" if s["hold"] else "queue open"]
    r = s["running"]
    if r:
        out.append("RUNNING %s owner=%s pid=%s %ds/%ds%s" % (r["id"], r["owner"], r["pid"], r["elapsed"], r["maxWall"],
                   "  OVERRUN" if r["overrun"] else ""))
    else:
        out.append("RUNNING none")
    for i, t in enumerate(s["queue"], 1):
        flag = "HEAD" if t["id"] == s["head"] else ("HELD" if t["held"] else (t["blocked"] or ("after " + ",".join(t["after"]) if t["after"] else "")))
        out.append("%2d. %s prio=%d wall<=%ds owner=%s waited=%ds %s" % (
            i, t["id"], t["prio"], t["maxWall"], t["owner"], t["waitedSeconds"], flag))
    return "\n".join(out)


def wait(ref, timeout=None):
    end = None if timeout is None else time.monotonic() + timeout
    while True:
        st = load()
        t = resolve(st, ref)
        if t is None:
            raise QueueError("no ticket %s" % ref)
        h = head(st)
        if t["state"] != "queued" or (h and h["id"] == t["id"]):
            return 0
        if end is not None and time.monotonic() >= end:
            return 124
        time.sleep(0.2)


# ------------------------------------------------------------------ CLI
def add_parser(sub):
    p = sub.add_parser("q", help="heavy-lock ticket queue: submit|status|cancel|reorder|hold|release|wait")
    qs = p.add_subparsers(dest="qcmd", required=True)
    s = qs.add_parser("submit", help="file a ticket (position in line); heavy --id LABEL later claims it")
    s.add_argument("--id", required=True)
    s.add_argument("--max-wall", type=float, help="seconds the job may hold the lock (required)")
    s.add_argument("--prio", type=int, default=0, help="coordinator-only")
    s.add_argument("--after", action="append", default=[], help="ticket id or label that must finish OK first")
    s.add_argument("--budget")
    s.add_argument("--need", action="append", dest="needs", help="heavy (default), gpu")
    s.add_argument("--cmd", help="description of the command, informational")
    st = qs.add_parser("status", help="running job, queue order, head, overruns")
    st.add_argument("--json", action="store_true")
    for name in ("cancel", "wait"):
        c = qs.add_parser(name)
        c.add_argument("ticket")
        if name == "wait":
            c.add_argument("--timeout", type=float)
    r = qs.add_parser("reorder", help="coordinator only")
    r.add_argument("ticket")
    r.add_argument("--first", action="store_true")
    r.add_argument("--before")
    r.add_argument("--prio", type=int)
    for name in ("hold", "release"):
        h = qs.add_parser(name, help="coordinator only; --all holds/releases the whole queue")
        h.add_argument("ticket", nargs="?")
        h.add_argument("--all", action="store_true")
    p.set_defaults(fn=run_cli)


def run_cli(args):
    try:
        c = args.qcmd
        if c == "submit":
            tid, pos = submit(args.id, args.max_wall, args.cmd, args.prio, args.after, args.budget, args.needs)
            print("%s position %d" % (tid, pos))
        elif c == "status":
            s = status()
            print(json.dumps(s, indent=1) if args.json else format_status(s))
        elif c == "cancel":
            print("cancelled %s" % cancel(args.ticket))
        elif c == "wait":
            return wait(args.ticket, args.timeout)
        elif c == "reorder":
            print("reordered %s" % reorder(args.ticket, args.first, args.before, args.prio))
        else:
            if not args.all and not args.ticket:
                raise QueueError("give a ticket or --all")
            print("%s %s" % (c, hold(args.ticket, args.all, c == "hold")))
    except QueueError as exc:
        print("q %s: %s" % (args.qcmd, exc), file=sys.stderr)
        return 2
    return 0
