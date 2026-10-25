"""Reap an expired heavy/GPU job through the tool that owns it.

Dry-run by default. The kill path (--execute) needs the owner's standing rule,
signalled by the file named in DW_REAP_ENABLE (default
~/.config/doctorwatson/reap-enabled). It only ever signals processes in the
session recorded in an EXPIRED lease, after a pid+starttime identity check, and
never the caller's own session.

Lease format (INTERFACE lease-json, agreed with heavy.py): next to each lock a
file <lockpath>.lease holding one JSON object:
  {"v":1,"id":..,"lock":"heavy"|"gpu","pid":<session leader>,
   "starttime":<field 22 of /proc/PID/stat>,"sid":<session id>,
   "deadline":<epoch seconds>,"cmd":..}
Unknown keys are ignored.
"""

import json
import os
import signal
import time

from . import preflight

EXIT_NOT_ENABLED = 3


def gpu_lock_path():
    return os.environ.get("DW_GPU_LOCK") or os.path.expanduser("~/.cache/doctorwatson/gpu.lock")


def enable_path():
    return os.environ.get("DW_REAP_ENABLE") or os.path.expanduser("~/.config/doctorwatson/reap-enabled")


def enabled():
    return os.path.exists(enable_path())


def lease_paths():
    return [("heavy", preflight.lock_path() + ".lease"), ("gpu", gpu_lock_path() + ".lease")]


def _stat(pid):
    """(starttime, sid) of a live pid, or None."""
    try:
        with open(f"/proc/{pid}/stat") as fh:
            rest = fh.read().rsplit(")", 1)[1].split()
    except (OSError, IndexError):
        return None
    if rest[0] == "Z":
        return None
    return int(rest[19]), int(rest[3])


def session_members(sid):
    """[(pid, starttime)] of live processes in session sid."""
    out = []
    for name in os.listdir("/proc"):
        if name.isdigit():
            st = _stat(int(name))
            if st and st[1] == sid:
                out.append((int(name), st[0]))
    return out


def assess(kind, path, now=None):
    """Classify one lease: dict with state ok|expired|stale|invalid and the members to reap."""
    now = time.time() if now is None else now
    rec = {"lock": kind, "lease": path, "state": "invalid", "killed": False, "members": [], "why": ""}
    try:
        with open(path) as fh:
            lease = json.load(fh)
        pid, sid = int(lease["pid"]), int(lease["sid"])
        start, deadline = int(lease["starttime"]), float(lease["deadline"])
    except (OSError, ValueError, KeyError, TypeError) as exc:
        rec["why"] = f"unreadable lease: {exc}"
        return rec
    rec.update(id=lease.get("id"), pid=pid, sid=sid, deadline=deadline, overdue=round(now - deadline, 1))
    members = session_members(sid)
    leader = _stat(pid)
    if leader is not None and leader[0] != start:
        rec.update(state="refused", why=f"pid {pid} starttime {leader[0]} != lease {start} (pid reused)")
        return rec
    if leader is not None and leader[1] != sid:
        rec.update(state="refused", why=f"pid {pid} is in session {leader[1]}, lease says {sid}")
        return rec
    # a session id cannot be recycled while any member lives, so members with this sid
    # belong to the leased job even if the leader already exited (orphaned descendants).
    members = [m for m in members if m[1] >= start]
    rec["members"] = members
    if sid == os.getsid(0):
        rec.update(state="refused", why="lease session is the caller's own session")
        return rec
    if not members:
        rec.update(state="stale", why="no live process in session; lock holder is not the leased job")
        return rec
    rec["state"] = "expired" if now > deadline else "ok"
    return rec


def _sig_members(members, sig):
    for pid, start in members:
        st = _stat(pid)
        if st and st[0] == start:  # re-verify identity at signal time
            try:
                os.kill(pid, sig)
            except OSError:
                pass


def execute(rec, grace):
    sid = rec["sid"]
    _sig_members(rec["members"], signal.SIGTERM)
    end = time.monotonic() + grace
    while time.monotonic() < end:
        if not session_members(sid):
            break
        time.sleep(0.1)
    left = [m for m in session_members(sid) if m in rec["members"]]
    _sig_members(left, signal.SIGKILL)
    time.sleep(0.2)
    rec["killed"] = not [m for m in session_members(sid) if m in rec["members"]]
    return rec["killed"]


def format_rec(rec, do_exec):
    base = f"{rec['lock']}: {rec.get('id')} sid={rec.get('sid')} pid={rec.get('pid')}"
    st = rec["state"]
    if st == "expired":
        verb = "reaped" if rec["killed"] else ("FAILED to reap" if do_exec else "would reap")
        return f"EXPIRED {base} overdue {rec['overdue']}s members={[p for p, _ in rec['members']]}: {verb}"
    if st == "ok":
        return f"ok {base} deadline in {-rec['overdue']}s"
    if st == "stale":
        return f"STALE {base}: {rec['why']}"
    return f"{st.upper()} {base}: {rec['why']}"


def cmd_reap(do_exec, grace, as_json):
    recs = []
    for kind, path in lease_paths():
        if os.path.exists(path):
            recs.append(assess(kind, path))
    refused = do_exec and not enabled()
    if do_exec and not refused:
        for r in recs:
            if r["state"] == "expired":
                execute(r, grace)
    if as_json:
        print(json.dumps(recs, indent=1))
    else:
        if not recs:
            print("no lease files found")
        for r in recs:
            print(format_rec(r, do_exec and not refused))
        if not do_exec and any(r["state"] == "expired" for r in recs):
            print(f"dry run: nothing signalled. --execute needs the owner rule ({'enabled' if enabled() else 'NOT enabled'}).")
    if refused:
        print(f"reap --execute: not enabled; owner standing rule not granted (create {enable_path()} to enable)")
        return EXIT_NOT_ENABLED
    return 1 if any(r["state"] == "expired" and not r["killed"] and do_exec for r in recs) else 0
