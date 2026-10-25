"""Host fitness check before heavy work."""

import fcntl
import json
import os
import shutil

from . import repo_root

GIB = 1024 ** 3


def lock_path():
    return os.environ.get("DW_LOCK") or os.path.expanduser("~/.cache/doctorwatson/heavy.lock")


def gpu_lock_path():
    return os.environ.get("DW_GPU_LOCK") or os.path.join(os.path.dirname(lock_path()), "gpu.lock")


def open_lock(p=None):
    p = p or lock_path()
    os.makedirs(os.path.dirname(p), exist_ok=True)
    return os.open(p, os.O_RDWR | os.O_CREAT, 0o644)


def try_lock(fd):
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return True
    except OSError:
        return False


def lock_is_held():
    fd = open_lock()
    try:
        if try_lock(fd):
            fcntl.flock(fd, fcntl.LOCK_UN)
            return False
        return True
    finally:
        os.close(fd)


def run(repo=None, lock_held_by_us=False, skip=False):
    """Return a dict with the measured numbers and ok/reasons.

    `skip` (tests only, DW_SKIP_PREFLIGHT=1) bypasses the disk and load checks; the
    lock check is never skipped.
    """
    repo = repo or repo_root()
    min_free = float(os.environ.get("DW_MIN_FREE_GIB", "150"))
    free_gib = shutil.disk_usage(repo).free / GIB
    load1 = os.getloadavg()[0]
    cpus = os.cpu_count() or 1
    max_load = float(os.environ.get("DW_MAX_LOAD", cpus))
    held = False if lock_held_by_us else lock_is_held()
    reasons = []
    if not skip:
        if free_gib < min_free:
            reasons.append(f"disk free {free_gib:.1f} GiB < {min_free:g} GiB")
        if not load1 < max_load:
            reasons.append(f"1-minute load {load1:.2f} >= {max_load:g}")
    if held:
        reasons.append("another process holds the heavy lock")
    return {
        "ok": not reasons,
        "skipped": bool(skip),
        "diskFreeGiB": round(free_gib, 2),
        "minFreeGiB": min_free,
        "load1": round(load1, 2),
        "maxLoad": max_load,
        "cpus": cpus,
        "lockHeld": held,
        "lock": lock_path(),
        "reasons": reasons,
    }


def format_text(r):
    lines = [
        f"disk free: {r['diskFreeGiB']} GiB (min {r['minFreeGiB']:g})",
        f"load 1m:   {r['load1']} (limit {r['maxLoad']:g}, cpus {r['cpus']})",
        f"heavy lock: {'HELD by another process' if r['lockHeld'] else 'free'} ({r['lock']})",
    ]
    if r["skipped"]:
        lines.append("(disk and load checks skipped: DW_SKIP_PREFLIGHT=1)")
    if r["ok"]:
        lines.append("PREFLIGHT OK")
    else:
        lines.append("PREFLIGHT UNFIT: " + "; ".join(r["reasons"]))
    return "\n".join(lines)


def main(as_json=False):
    r = run(skip=os.environ.get("DW_SKIP_PREFLIGHT") == "1")
    print(json.dumps(r, indent=2) if as_json else format_text(r))
    return 0 if r["ok"] else 3
