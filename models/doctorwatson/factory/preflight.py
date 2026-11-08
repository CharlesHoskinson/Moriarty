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


def meminfo():
    """Return (MemAvailable, swap used) in bytes from /proc/meminfo, or (None, None)."""
    vals = {}
    try:
        with open("/proc/meminfo") as fh:
            for line in fh:
                key, _, rest = line.partition(":")
                parts = rest.split()
                if parts:
                    vals[key] = int(parts[0]) * 1024
    except (OSError, ValueError):
        return None, None
    avail = vals.get("MemAvailable")
    swap = vals["SwapTotal"] - vals["SwapFree"] if "SwapTotal" in vals and "SwapFree" in vals else None
    return avail, swap


def default_mem_need():
    from . import heavy
    return heavy.default_rss_cap()


def run(repo=None, lock_held_by_us=False, skip=False, mem_need=None):
    """Return a dict with the measured numbers and ok/reasons.

    `skip` (tests only, DW_SKIP_PREFLIGHT=1) bypasses the disk, load and memory checks; the
    lock check is never skipped. `mem_need` is the memory cap (bytes) the job will run under;
    admission needs MemAvailable >= mem_need + DW_MEM_RESERVE_GIB and swap used <= DW_MAX_SWAP_GIB.
    """
    repo = repo or repo_root()
    if mem_need is None:
        mem_need = default_mem_need()
    reserve = float(os.environ.get("DW_MEM_RESERVE_GIB", "20")) * GIB
    max_swap = float(os.environ.get("DW_MAX_SWAP_GIB", "4")) * GIB
    avail, swap = meminfo()
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
        if avail is None:
            reasons.append("cannot read MemAvailable from /proc/meminfo")
        elif avail < mem_need + reserve:
            reasons.append(f"MemAvailable {avail / GIB:.1f} GiB < cap {mem_need / GIB:.1f} GiB"
                           f" + reserve {reserve / GIB:g} GiB")
        if swap is not None and swap > max_swap:
            reasons.append(f"swap in use {swap / GIB:.1f} GiB > {max_swap / GIB:g} GiB")
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
        "memAvailableGiB": None if avail is None else round(avail / GIB, 2),
        "memNeedGiB": round(mem_need / GIB, 2),
        "memReserveGiB": round(reserve / GIB, 2),
        "swapUsedGiB": None if swap is None else round(swap / GIB, 2),
        "maxSwapGiB": round(max_swap / GIB, 2),
        "lockHeld": held,
        "lock": lock_path(),
        "reasons": reasons,
    }


def format_text(r):
    lines = [
        f"disk free: {r['diskFreeGiB']} GiB (min {r['minFreeGiB']:g})",
        f"load 1m:   {r['load1']} (limit {r['maxLoad']:g}, cpus {r['cpus']})",
        f"memory:    {r['memAvailableGiB']} GiB available (need cap {r['memNeedGiB']:g} + reserve {r['memReserveGiB']:g})",
        f"swap used: {r['swapUsedGiB']} GiB (limit {r['maxSwapGiB']:g})",
        f"heavy lock: {'HELD by another process' if r['lockHeld'] else 'free'} ({r['lock']})",
    ]
    if r["skipped"]:
        lines.append("(disk, load and memory checks skipped: DW_SKIP_PREFLIGHT=1)")
    if r["ok"]:
        lines.append("PREFLIGHT OK")
    else:
        lines.append("PREFLIGHT UNFIT: " + "; ".join(r["reasons"]))
    return "\n".join(lines)


def main(as_json=False):
    r = run(skip=os.environ.get("DW_SKIP_PREFLIGHT") == "1")
    print(json.dumps(r, indent=2) if as_json else format_text(r))
    return 0 if r["ok"] else 3
