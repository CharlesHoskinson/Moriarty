"""Run a command with measurement, tee-to-trace, budget caps and the heavy lock."""

import datetime
import fcntl
import json
import os
import re
import shlex
import signal
import stat
import subprocess
import sys
import threading
import time

from . import budgets, bus, head_sha, now_iso, preflight, queue, repo_root, status

EXIT_UNFIT = 3
EXIT_BUDGET = 4
EXIT_HYPOTHESIS = 5
EXIT_NOBUDGET = 6
DEFAULT_MAX_WALL = 4 * 3600  # hard ceiling when a caller gives only a budget key or deadline
LEASE_SCHEMA = "doctorwatson.lease/1"
LOCK_HOOKS = []  # hook(event, lease) with event "acquired" or "released"; exceptions are swallowed
GIB = 1024 ** 3
MAX_RSS_CAP = 48 * GIB  # owner decision 2026-11-08: 48 GB per process (SRC-016)
MAX_AGGREGATE_CAP = 100 * GIB  # owner decision 2026-11-08: 100 GB aggregate (SRC-016, E11)
DEFAULT_RUST_TEST_THREADS = "4"
_GUARD_PROBE = {}


def default_rss_cap():
    """DW_DEFAULT_RSS_CAP (GiB, default 48), never above MAX_RSS_CAP."""
    try:
        gib = float(os.environ.get("DW_DEFAULT_RSS_CAP") or 48)
    except ValueError:
        gib = 48.0
    return min(int(gib * GIB), MAX_RSS_CAP)


def clamp_rss_cap(cap_rss):
    """Every heavy run has a memory cap: the budget cap, else the default; never above 48 GiB."""
    if not cap_rss:
        return default_rss_cap()
    return min(int(cap_rss), MAX_RSS_CAP)


def cgroup_guard_available():
    """True when `systemd-run --user --scope` can place a job under a kernel memory limit here.

    DW_MEM_GUARD=poll forces the RSS-poll fallback. The probe runs once per process.
    """
    if os.environ.get("DW_MEM_GUARD") == "poll":
        return False
    if "ok" not in _GUARD_PROBE:
        try:
            r = subprocess.run(["systemd-run", "--user", "--scope", "--collect", "-q",
                                "-p", "MemoryMax=64M", "-p", "MemorySwapMax=0", "--", "true"],
                               stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL, timeout=15)
            _GUARD_PROBE["ok"] = r.returncode == 0
        except (OSError, subprocess.SubprocessError):
            _GUARD_PROBE["ok"] = False
    return _GUARD_PROBE["ok"]


def scope_unit(marker):
    safe = re.sub(r"[^A-Za-z0-9_.-]", "_", marker or f"{os.getpid()}-{time.time_ns()}")
    return f"dw-{safe}.scope"


def parse_size(text):
    """'100G', '100GiB', '512M', '1T' (binary units) or a bare number of GiB -> bytes."""
    m = re.fullmatch(r"\s*([0-9]*\.?[0-9]+)\s*([KMGT]?)(?:i?B)?\s*", str(text), re.I)
    if not m:
        raise ValueError(f"bad size {text!r}")
    unit = m.group(2).upper() or "G"
    return int(float(m.group(1)) * 1024 ** "KMGT".index(unit) * 1024)


def slice_name(marker):
    """Transient parent slice for a concurrent run. No '-' in the stem: systemd nests slices on dashes."""
    return "dw_" + re.sub(r"[^A-Za-z0-9_]", "_", marker) + ".slice"


def _systemctl(*args):
    try:
        return subprocess.run(["systemctl", "--user", *args], stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                              stderr=subprocess.DEVNULL, timeout=15).returncode == 0
    except (OSError, subprocess.SubprocessError):
        return False


def _unit_cgroup(unit):
    """/sys/fs/cgroup path of a user unit, or None."""
    try:
        out = subprocess.run(["systemctl", "--user", "show", "-p", "ControlGroup", "--value", unit],
                             stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=15).stdout.strip()
    except (OSError, subprocess.SubprocessError):
        return None
    return "/sys/fs/cgroup" + out if out else None


def guard_argv(argv, cap_rss, unit, slice_=None):
    """Wrap argv so the kernel enforces cap_rss (no swap) on the whole job tree; OOM kills the scope.

    systemd-run execs argv in place (same pid, same session); --expand-environment=no keeps `$` literal.
    """
    return ["systemd-run", "--user", "--scope", "--collect", "-q", "--expand-environment=no", f"--unit={unit}",
            *([f"--slice={slice_}"] if slice_ else []),
            "-p", f"MemoryMax={int(cap_rss)}", "-p", "MemorySwapMax=0", "-p", "OOMPolicy=kill",
            "--", *argv]


def _cgroup_dir(pid, unit):
    try:
        with open(f"/proc/{pid}/cgroup") as fh:
            for line in fh:
                path = line.rstrip("\n").split(":", 2)[-1]
                if path.endswith("/" + unit):
                    return "/sys/fs/cgroup" + path
    except OSError:
        pass
    return None


def _cgroup_read(cgdir):
    """Return (oom_kill count, memory.peak bytes) of a scope, None where unreadable."""
    oom = peak = None
    try:
        with open(os.path.join(cgdir, "memory.events")) as fh:
            for line in fh:
                k, _, v = line.partition(" ")
                if k == "oom_kill":
                    oom = int(v)
    except (OSError, ValueError):
        pass
    try:
        with open(os.path.join(cgdir, "memory.peak")) as fh:
            peak = int(fh.read().split()[0])
    except (OSError, ValueError, IndexError):
        pass
    return oom, peak


def _cgroup_kill(cgdir, unit=None):
    """SIGKILL everything left in the job's scope (cgroup v2 cgroup.kill), then stop the unit.

    Escapees cannot leave a scope. The stop removes a scope that systemd did not collect.
    """
    if cgdir:
        try:
            with open(os.path.join(cgdir, "cgroup.kill"), "w") as fh:
                fh.write("1")
        except OSError:
            pass
    if unit and (cgdir is None or os.path.isdir(cgdir)):
        try:
            subprocess.run(["systemctl", "--user", "stop", "--no-block", unit], stdin=subprocess.DEVNULL,
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10)
        except (OSError, subprocess.SubprocessError):
            pass


def _is_cargo_test(argv):
    names = [os.path.basename(a) for a in argv]
    for i, n in enumerate(names):
        if n == "cargo":
            rest = [a for a in argv[i + 1:] if not a.startswith("+")]
            return bool(rest) and rest[0] in ("test", "nextest")
    return False


class Terminated(BaseException):
    """Raised by the signal handler so run_command unwinds, kills the job tree and releases locks."""


def _echo(chunk):
    buf = getattr(sys.stdout, "buffer", None)
    if buf is not None:
        buf.write(chunk)
        buf.flush()
    else:
        sys.stdout.write(chunk.decode("utf-8", "replace"))
        sys.stdout.flush()


def group_rss(pgid):
    """Sum resident bytes of every process in the process group."""
    page = os.sysconf("SC_PAGE_SIZE")
    total = 0
    try:
        names = os.listdir("/proc")
    except OSError:
        return 0
    for name in names:
        if not name.isdigit():
            continue
        try:
            with open(f"/proc/{name}/stat", "rb") as fh:
                stat = fh.read().decode("utf-8", "replace")
            fields = stat[stat.rindex(")") + 2:].split()
            if int(fields[2]) != pgid:
                continue
            with open(f"/proc/{name}/statm", "rb") as fh:
                total += int(fh.read().split()[1]) * page
        except (OSError, ValueError, IndexError):
            continue
    return total


def _proc_fields(pid):
    with open(f"/proc/{pid}/stat", "rb") as fh:
        stat = fh.read().decode("utf-8", "replace")
    return stat[stat.rindex(")") + 2:].split()  # [0]=state [2]=pgrp [3]=session [19]=starttime


def proc_start(pid):
    try:
        return int(_proc_fields(pid)[19])
    except (OSError, ValueError, IndexError):
        return None


def _sweep(sid, marker):
    """SIGKILL every live process in session sid or carrying marker in its environment."""
    me = os.getpid()
    found = 0
    for name in os.listdir("/proc"):
        if not name.isdigit() or int(name) == me:
            continue
        pid = int(name)
        try:
            hit = sid is not None and int(_proc_fields(pid)[3]) == sid
            if not hit and marker:
                with open(f"/proc/{pid}/environ", "rb") as fh:
                    hit = ("DW_JOB_ID=" + marker).encode() in fh.read().split(b"\0")
            if hit:
                os.kill(pid, signal.SIGKILL)
                found += 1
        except (OSError, ValueError, IndexError):
            continue
    return found


def kill_job_tree(sid, marker=None):
    """Kill a job tree: killpg, then sweep /proc by session id and env marker (catches setsid escapees)."""
    if sid:
        _killpg(sid)
    for _ in range(5):
        if not _sweep(sid, marker):
            break
        time.sleep(0.05)


def _killpg(pgid):
    try:
        os.killpg(pgid, signal.SIGKILL)
    except (ProcessLookupError, PermissionError):
        pass


def run_command(argv, cwd, trace_path, echo, cap=None, marker=None, on_start=None, watch_parent=False, pipe_stdin=False,
                mem_guard=False, extra_env=None, slice_=None):
    """Run argv in its own session; tee merged stdout/stderr to trace_path (and stdout if echo).

    mem_guard ("cgroup", "poll", or True to probe) with a maxRssBytes cap runs the job in a systemd user scope with
    MemoryMax=cap and MemorySwapMax=0 (the kernel kills it, not the machine), falling back
    to the RSS poll when systemd-run is unavailable. The poll stays on as the measurement.

    Returns {exit, wallSeconds, maxRssBytes, exceeded, memGuard, memCapBytes, ...} where
    exceeded is None, "wall" or "rss".
    """
    cap = cap or {}
    cap_wall = cap.get("wallSeconds")
    cap_rss = cap.get("maxRssBytes")
    guard = "none"
    unit = None
    if mem_guard and cap_rss:
        if mem_guard == "cgroup" or (mem_guard is True and cgroup_guard_available()):
            guard = "cgroup"
            unit = scope_unit(marker)
            argv = guard_argv(argv, cap_rss, unit, slice_)
        else:
            guard = "poll"
    os.makedirs(os.path.dirname(trace_path), exist_ok=True)
    log = open(trace_path, "wb")
    t0 = time.monotonic()
    env = None
    if marker or extra_env:
        env = dict(os.environ)
        env.update(extra_env or {})
        if marker:
            env["DW_JOB_ID"] = marker
    ppid0 = os.getppid()
    old_handlers = {}

    def _terminate(signum, _frame):
        raise Terminated(signum)

    for sig in (signal.SIGTERM, signal.SIGHUP, signal.SIGINT):
        try:
            old_handlers[sig] = signal.signal(sig, _terminate)
        except ValueError:  # not the main thread
            break
    try:
        proc = subprocess.Popen(
            argv, cwd=cwd, stdin=None if pipe_stdin else subprocess.DEVNULL, env=env,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True,
        )
    except OSError as exc:
        msg = f"cannot start {argv[0]!r}: {exc}\n".encode()
        log.write(msg)
        log.close()
        if echo:
            _echo(msg)
        _restore(old_handlers)
        return {"exit": 127, "wallSeconds": 0.0, "maxRssBytes": 0, "exceeded": None,
                "memGuard": guard, "memCapBytes": cap_rss}
    pgid = proc.pid
    if on_start:
        on_start(proc.pid)
    fd = proc.stdout.fileno()

    def pump():
        while True:
            try:
                chunk = os.read(fd, 65536)
            except OSError:
                break
            if not chunk:
                break
            log.write(chunk)
            log.flush()
            if echo:
                _echo(chunk)

    thread = threading.Thread(target=pump, daemon=True)
    thread.start()
    exceeded = None
    peak = 0
    cg = {"dir": None, "oom": 0, "peak": None}

    def sample_cgroup():
        if unit is None:
            return
        if cg["dir"] is None:
            cg["dir"] = _cgroup_dir(proc.pid, unit)
        if cg["dir"] is not None:
            oom, cpeak = _cgroup_read(cg["dir"])
            if oom is not None:
                cg["oom"] = max(cg["oom"], oom)
            if cpeak is not None:
                cg["peak"] = max(cg["peak"] or 0, cpeak)

    try:
        if cap_wall is None and cap_rss is None and not watch_parent:
            _, st, ru = os.wait4(proc.pid, 0)
        else:
            next_rss = 0.0
            while True:
                pid, st, ru = os.wait4(proc.pid, os.WNOHANG)
                if pid:
                    break
                now = time.monotonic()
                if watch_parent and os.getppid() != ppid0:
                    raise Terminated(signal.SIGHUP)
                if exceeded is None:
                    if cap_wall is not None and now - t0 > cap_wall:
                        exceeded = "wall"
                        kill_job_tree(pgid, marker)
                    elif cap_rss is not None and now >= next_rss:
                        next_rss = now + 0.2
                        peak = max(peak, group_rss(pgid))
                        sample_cgroup()
                        if peak > cap_rss or cg["oom"]:
                            exceeded = "rss"
                            kill_job_tree(pgid, marker)
                time.sleep(0.02)
    except BaseException:
        if unit is not None and cg["dir"] is None:
            cg["dir"] = _cgroup_dir(proc.pid, unit)
        _cgroup_kill(cg["dir"], unit)
        kill_job_tree(pgid, marker)
        try:
            os.wait4(proc.pid, 0)
        except OSError:
            pass
        raise
    finally:
        wall = time.monotonic() - t0
        _restore(old_handlers)
    proc.returncode = 0  # reaped by wait4; keep Popen from warning
    _cgroup_kill(cg["dir"], unit)
    kill_job_tree(pgid, marker)  # stragglers (also setsid escapees) that would hold the pipe or locks
    thread.join(5)
    log.close()
    proc.stdout.close()
    code = os.waitstatus_to_exitcode(st)
    if code < 0:
        code = 128 - code
    rss = max(peak, int(ru.ru_maxrss) * 1024)
    if exceeded is None:
        if cap_wall is not None and wall > cap_wall:
            exceeded = "wall"
        elif cap_rss is not None and rss > cap_rss:
            exceeded = "rss"
        elif guard == "cgroup" and (cg["oom"] or (code == 137 and max(rss, cg["peak"] or 0) >= 0.9 * cap_rss)):
            exceeded = "rss"  # kernel OOM kill inside the scope (events counter, or SIGKILL at the cap)
    res = {"exit": code, "wallSeconds": round(wall, 3), "maxRssBytes": rss, "exceeded": exceeded,
           "memGuard": guard, "memCapBytes": cap_rss}
    if unit is not None:
        res.update(memScope=unit, cgroupPeakBytes=cg["peak"], oomKills=cg["oom"])
    return res


def run_concurrent(commands, cwd, trace_path, echo, cap, aggregate, marker, mem_guard, extra_env=None,
                   on_start=None, watch_parent=False):
    """E11: run the commands at once, each under the per-process cap, all under the aggregate cap.

    With the cgroup guard every child scope sits in one transient slice with MemoryMax=aggregate,
    so the kernel enforces the sum; the main thread also polls the summed RSS (the measure, and the
    guard when the slice is unavailable). Child traces are <trace>.<slot>.log. Returns the
    run_command shape (wallSeconds is the makespan, maxRssBytes the aggregate peak) plus children.
    """
    n = len(commands)
    slice_ = None
    if mem_guard == "cgroup":
        name = slice_name(marker)
        if _systemctl("set-property", "--runtime", name, f"MemoryMax={int(aggregate)}", "MemorySwapMax=0"):
            slice_ = name
    base = trace_path[:-4] if trace_path.endswith(".log") else trace_path
    traces = [f"{base}.{i}.log" for i in range(n)]
    results = [None] * n
    pgids = {}
    ppid0 = os.getppid()

    def started(i, pid):
        pgids[i] = pid
        if on_start and len(pgids) == 1:
            on_start(pid)

    def worker(i):
        env = dict(extra_env or {}, DW_SLOT=str(i), DW_SLOTS=str(n))
        try:
            results[i] = run_command(commands[i], cwd, traces[i], False, cap, marker=f"{marker}.{i}",
                                     on_start=lambda pid: started(i, pid), mem_guard=mem_guard,
                                     extra_env=env, slice_=slice_)
        except BaseException as exc:  # a thread never raises into the runner; record as a crash
            results[i] = {"exit": 127, "wallSeconds": 0.0, "maxRssBytes": 0, "exceeded": None,
                          "error": repr(exc)}

    def kill_all():
        if slice_:
            _systemctl("kill", "--signal=SIGKILL", slice_)
        for i, pg in list(pgids.items()):
            kill_job_tree(pg, f"{marker}.{i}")

    old_handlers = {}

    def _terminate(signum, _frame):
        raise Terminated(signum)

    for sig in (signal.SIGTERM, signal.SIGHUP, signal.SIGINT):
        try:
            old_handlers[sig] = signal.signal(sig, _terminate)
        except ValueError:
            break
    t0 = time.monotonic()
    threads = [threading.Thread(target=worker, args=(i,), daemon=True) for i in range(n)]
    agg_peak = 0
    agg_exceeded = False
    slice_dir = None
    slice_oom = 0
    slice_peak = None
    try:
        for t in threads:
            t.start()
        while any(t.is_alive() for t in threads):
            if watch_parent and os.getppid() != ppid0:
                raise Terminated(signal.SIGHUP)
            total = sum(group_rss(pg) for pg in list(pgids.values()))
            agg_peak = max(agg_peak, total)
            if slice_ and slice_dir is None and pgids:
                d = _cgroup_dir(next(iter(pgids.values())), scope_unit(f"{marker}.{next(iter(pgids))}"))
                slice_dir = os.path.dirname(d) if d else None
            if slice_dir:
                oom, pk = _cgroup_read(slice_dir)
                slice_oom = max(slice_oom, oom or 0)
                slice_peak = max(slice_peak or 0, pk or 0) if pk is not None else slice_peak
            if not agg_exceeded and total > aggregate:
                agg_exceeded = True
                kill_all()
            time.sleep(0.2)
    except BaseException:
        kill_all()
        for t in threads:
            t.join(10)
        raise
    finally:
        wall = time.monotonic() - t0
        _restore(old_handlers)
        if slice_:
            d = slice_dir or _unit_cgroup(slice_)
            if d:
                oom, pk = _cgroup_read(d)
                slice_oom = max(slice_oom, oom or 0)
                if pk is not None:
                    slice_peak = max(slice_peak or 0, pk)
            _systemctl("stop", slice_)
            _systemctl("revert", slice_)
    for t in threads:
        t.join(5)
    children = []
    for i, r in enumerate(results):
        r = r or {"exit": 127, "wallSeconds": 0.0, "maxRssBytes": 0, "exceeded": None}
        c = {"slot": i, "cmd": shlex.join(commands[i]), "trace": os.path.relpath(traces[i], cwd)}
        c.update({k: r[k] for k in ("exit", "wallSeconds", "maxRssBytes", "exceeded", "memScope",
                                    "cgroupPeakBytes", "oomKills", "error") if k in r})
        children.append(c)
    if slice_oom and not agg_exceeded:
        agg_exceeded = any(c["exit"] == 137 and not c.get("exceeded") for c in children)
    exceeded = "rss" if agg_exceeded or any(c.get("exceeded") == "rss" for c in children) else (
        "wall" if any(c.get("exceeded") == "wall" for c in children) else None)
    code = next((c["exit"] for c in children if c["exit"] != 0), 0)
    with open(trace_path, "w") as fh:
        for c in children:
            fh.write(f"slot {c['slot']}: exit={c['exit']} wall={c['wallSeconds']}s rss={c['maxRssBytes']}B "
                     f"exceeded={c.get('exceeded')} trace={c['trace']} cmd={c['cmd']}\n")
        fh.write(f"makespan={wall:.3f}s aggregatePeak={max(agg_peak, slice_peak or 0)}B "
                 f"aggregateCap={int(aggregate)}B slice={slice_} aggregateExceeded={agg_exceeded}\n")
    if echo:
        with open(trace_path, "rb") as fh:
            _echo(fh.read())
    return {"exit": code, "wallSeconds": round(wall, 3), "maxRssBytes": max(agg_peak, slice_peak or 0),
            "exceeded": exceeded, "memGuard": mem_guard, "memCapBytes": cap.get("maxRssBytes"),
            "aggregateCapBytes": int(aggregate), "memSlice": slice_, "aggregateExceeded": agg_exceeded,
            "sliceOomKills": slice_oom, "concurrent": n, "children": children}


def _restore(handlers):
    for sig, old in handlers.items():
        try:
            signal.signal(sig, old)
        except (ValueError, TypeError):
            pass


def run_id(pid_label, repo):
    safe = re.sub(r"[^A-Za-z0-9._-]", "_", pid_label)
    stamp = datetime.datetime.now(datetime.timezone.utc)
    rid = f"{stamp.strftime('%Y%m%dT%H%M%SZ')}-{safe}"
    if os.path.exists(os.path.join(repo, "qa/sherlock/traces", rid + ".log")):
        rid = f"{stamp.strftime('%Y%m%dT%H%M%S.%fZ')}-{safe}"
    return rid


def trace_rel(rid):
    return f"qa/sherlock/traces/{rid}.log"


def last_run_of(pid_label, repo):
    last = None
    for rec in status.read_runs(repo):
        if rec.get("id") == pid_label:
            last = rec
    return last


def hypothesis_required(pid_label, cmd_str, head, hypothesis, repo):
    """Return the prior failed run if the retry rule demands a hypothesis, else None."""
    if hypothesis:
        return None
    last = last_run_of(pid_label, repo)
    if last and last.get("exit") != 0 and last.get("cmd") == cmd_str and last.get("head") == head:
        return last
    return None


def run_record(rid, pid_label, cmd_str, hypothesis, res, exit_code, pre, head, repo, extra=None):
    rec = {
        "run": rid, "id": pid_label, "cmd": cmd_str, "hypothesis": hypothesis or None,
        "exit": exit_code, "wallSeconds": res["wallSeconds"], "maxRssBytes": res["maxRssBytes"],
        "trace": trace_rel(rid), "preflight": pre, "head": head, "at": now_iso(),
    }
    if extra:
        rec.update(extra)
    return rec


def lease_path(kind):
    return (preflight.gpu_lock_path() if kind == "gpu" else preflight.lock_path()) + ".lease"


def read_lease(kind):
    try:
        with open(lease_path(kind)) as fh:
            return json.load(fh)
    except (OSError, ValueError):
        return None


def lease_state(kind):
    """{"lease", "holderAlive" (pid+starttime match), "expired" (deadline passed), "locked" (flock probe)}."""
    lease = read_lease(kind)
    alive = bool(lease) and proc_start(lease.get("holderPid", -1)) == lease.get("holderStart")
    expired = bool(lease) and time.time() > float(lease.get("deadline") or 0)
    p = preflight.gpu_lock_path() if kind == "gpu" else preflight.lock_path()
    fd = preflight.open_lock(p)
    try:
        locked = not preflight.try_lock(fd)
    finally:
        os.close(fd)
    return {"lease": lease, "holderAlive": alive, "expired": expired, "locked": locked}


def _write_lease(kinds, lease):
    for kind in kinds:
        path = lease_path(kind)
        tmp = f"{path}.{os.getpid()}.tmp"
        with open(tmp, "w") as fh:
            json.dump(dict(lease, lock=kind), fh)
        os.replace(tmp, path)


def _remove_lease(kinds):
    for kind in kinds:
        try:
            os.unlink(lease_path(kind))
        except OSError:
            pass


def _hook(event, lease):
    for fn in list(LOCK_HOOKS):
        try:
            fn(event, dict(lease))
        except Exception:
            pass


def eff_wall(budget_key, deadline, max_wall, repo):
    cap = budgets.limits_for(budget_key, repo) if budget_key else None
    return effective_wall(cap, deadline, max_wall) or DEFAULT_MAX_WALL


def effective_wall(cap, deadline, max_wall):
    """Smallest of the budget cap, the caller deadline (epoch) and the hard default; None if all absent."""
    vals = []
    if cap and cap.get("wallSeconds") is not None:
        vals.append(float(cap["wallSeconds"]))
    if deadline is not None:
        vals.append(max(0.5, deadline - time.time()))
    if max_wall is not None:
        vals.append(float(max_wall))
    return min(vals) if vals else None


def _env_float(name):
    try:
        return float(os.environ[name]) if os.environ.get(name) else None
    except ValueError:
        return None


def run_heavy(pid_label, argv, cmd_str, budget_key=None, hypothesis=None, repo=None, echo=True, err=None,
              gpu=False, deadline=None, max_wall=None, pipe_stdin=False, commands=None, aggregate_cap=None):
    """Run one heavy command, with bus visibility (advisory claim, job.start/end) around it.

    The bus hooks never raise, print or change the result; the flock stays authoritative.
    gpu=True also takes the gpu lock in-process (both released together). deadline is an epoch; max_wall
    seconds. With no budget key, deadline or max_wall (flag or DW_MAX_WALL) the run is refused (exit 6).
    """
    repo = repo or repo_root()
    hb = bus.heavy_bus(pid_label, budget_key, repo)
    hb.begin()
    try:
        return _run_heavy(pid_label, argv, cmd_str, budget_key, hypothesis, repo, echo, err, hb,
                          gpu, deadline, max_wall, pipe_stdin, commands, aggregate_cap)
    finally:
        hb.close()


def _run_heavy(pid_label, argv, cmd_str, budget_key, hypothesis, repo, echo, err, hb,
               gpu=False, deadline=None, max_wall=None, pipe_stdin=False, commands=None, aggregate_cap=None):
    """Run one heavy command. Returns {exit, run, trace, wallSeconds, maxRssBytes, message}.

    exit is the child's exit, 3 (host unfit), 4 (budget exceeded), 5 (hypothesis required) or 6 (no wall bound).
    Exit 3, 5 and 6 mean nothing ran, so nothing is logged and the retry rule is unaffected.
    """
    repo = repo or repo_root()
    err = err or sys.stderr
    head = head_sha(repo)
    if deadline is None:
        deadline = _env_float("DW_DEADLINE")
    env_max = _env_float("DW_MAX_WALL")
    if max_wall is None:
        max_wall = env_max
    if not budget_key and deadline is None and max_wall is None:
        return {"exit": EXIT_NOBUDGET, "message": "refused: no wall budget; give --budget KEY, --max-wall SECONDS, "
                "--deadline EPOCH or set DW_MAX_WALL (an unbounded heavy job can hold the locks forever)"}
    if max_wall is None:
        max_wall = DEFAULT_MAX_WALL

    prior = hypothesis_required(pid_label, cmd_str, head, hypothesis, repo)
    if prior is not None:
        msg = (f"hypothesis required: last run {prior.get('run')} of {pid_label} failed "
               f"(exit {prior.get('exit')}) with the same command at the same head; "
               f"read {prior.get('trace')} and retry with --hypothesis TEXT")
        return {"exit": EXIT_HYPOTHESIS, "message": msg}

    aggregate = None
    if commands:
        bcap = (budgets.limits_for(budget_key, repo) or {}) if budget_key else {}
        aggregate = min(int(aggregate_cap or bcap.get("aggregateRssBytes") or MAX_AGGREGATE_CAP), MAX_AGGREGATE_CAP)
        per = clamp_rss_cap(bcap.get("maxRssBytes"))
        if len(commands) * per > aggregate:
            return {"exit": EXIT_UNFIT, "message": (
                f"host unfit: {len(commands)} x per-process cap {per / GIB:.1f} GiB = {len(commands) * per / GIB:.1f} GiB"
                f" > aggregate cap {aggregate / GIB:.1f} GiB (lower the cap with a budget or DW_DEFAULT_RSS_CAP)")}
    guard_mode = "cgroup" if cgroup_guard_available() else "poll"  # probe before any lock or signal window
    kinds = ["heavy", "gpu"] if gpu else ["heavy"]
    fd = preflight.open_lock()
    gfd = None
    leased = False
    queued = False
    last_exit = [None]
    try:
        bc = (budgets.limits_for(budget_key, repo) or {}) if budget_key else {}
        admitted = clamp_rss_cap(bc.get("maxRssBytes")) * (len(commands) if commands else 1)
        ok, why = queue.acquire_turn(fd, pid_label, cmd_str, max_wall=eff_wall(budget_key, deadline, max_wall, repo),
                                     budget=budget_key, mem_cap=admitted, mem_limit=MAX_AGGREGATE_CAP,
                                     timeout=None if deadline is None else max(0.0, deadline - time.time()))
        if not ok:
            return {"exit": EXIT_UNFIT, "message": "host unfit: another process holds the heavy lock (" + (why or "queue") + ")"}
        queued = True
        if gpu:
            gfd = preflight.open_lock(preflight.gpu_lock_path())
            if not preflight.try_lock(gfd):
                return {"exit": EXIT_UNFIT, "message": "host unfit: another process holds the gpu lock"}
        cap = budgets.limits_for(budget_key, repo) if budget_key else None
        rss_cap = clamp_rss_cap((cap or {}).get("maxRssBytes"))
        pre = preflight.run(repo, lock_held_by_us=True, skip=os.environ.get("DW_SKIP_PREFLIGHT") == "1",
                            mem_need=rss_cap * len(commands) if commands else rss_cap)
        if not pre["ok"]:
            return {"exit": EXIT_UNFIT, "message": "host unfit: " + "; ".join(pre["reasons"]), "preflight": pre}

        wall_cap = effective_wall(cap, deadline, max_wall)
        run_cap = dict(cap or {})
        run_cap["wallSeconds"] = wall_cap
        run_cap["maxRssBytes"] = rss_cap
        extra_env = {}
        if any(_is_cargo_test(a) for a in (commands or [argv])) and not os.environ.get("RUST_TEST_THREADS"):
            extra_env["RUST_TEST_THREADS"] = os.environ.get("DW_RUST_TEST_THREADS") or DEFAULT_RUST_TEST_THREADS
        rid = run_id(pid_label, repo)
        trace_abs = os.path.join(repo, trace_rel(rid))
        now = time.time()
        lease = {"schema": LEASE_SCHEMA, "v": 1, "pid": None, "sid": None, "starttime": None, "id": pid_label, "run": rid, "cmd": cmd_str,
                 "holderPid": os.getpid(), "holderStart": proc_start(os.getpid()), "jobPid": None,
                 "jobSid": None, "jobMarker": "DW_JOB_ID=" + rid, "startedAt": now,
                 "deadline": now + (wall_cap or DEFAULT_MAX_WALL), "locks": kinds}
        _write_lease(kinds, lease)
        leased = True
        _hook("acquired", lease)

        def on_start(pid):
            lease.update(jobPid=pid, jobSid=pid, pid=pid, sid=pid, starttime=proc_start(pid))
            _write_lease(kinds, lease)

        hb.started(cap, hypothesis)
        if commands:
            res = run_concurrent(commands, repo, trace_abs, echo, run_cap, aggregate, rid, guard_mode, extra_env,
                                 on_start=on_start, watch_parent=os.environ.get("DW_WATCH_PARENT") == "1")
        else:
            res = run_command(argv, repo, trace_abs, echo, run_cap, marker=rid, on_start=on_start,
                              watch_parent=os.environ.get("DW_WATCH_PARENT") == "1",
                              pipe_stdin=pipe_stdin, mem_guard=guard_mode, extra_env=extra_env)
        code = res["exit"]
        last_exit[0] = code
        extra = {k: res[k] for k in ("memGuard", "memCapBytes", "memScope", "cgroupPeakBytes", "oomKills",
                                     "aggregateCapBytes", "memSlice", "aggregateExceeded", "sliceOomKills",
                                     "concurrent", "children") if k in res}
        extra["cgroup"] = res.get("memGuard") == "cgroup"
        if commands:
            extra.update(concurrency=res.get("concurrent"), aggregateRssBytes=res["maxRssBytes"])
        if extra_env:
            extra["env"] = extra_env
        if res["exceeded"]:
            extra.update(childExit=code, capExceeded=res["exceeded"])
            code = EXIT_BUDGET
        rec = run_record(rid, pid_label, cmd_str, hypothesis, res, code, pre, head, repo, extra)
        status.append_run(rec, repo)
        hb.finished(rec)
        if budget_key and (cap is None or cap.get("fixed")) and code == 0:
            budgets.record_measurement(budget_key, res["wallSeconds"],
                                       max((c.get("maxRssBytes") or 0) for c in res["children"]) if commands
                                       else res["maxRssBytes"], rid, rec["at"], repo,
                                       aggregate_rss_bytes=res["maxRssBytes"] if commands else None,
                                       concurrency=len(commands) if commands else None)
        msg = (f"heavy {pid_label}: exit={code} wall={res['wallSeconds']}s "
               f"rss={res['maxRssBytes']}B run={rid} trace={rec['trace']}")
        if res["exceeded"]:
            msg += f" BUDGET EXCEEDED ({res['exceeded']}; cap={run_cap})"
        return {"exit": code, "run": rid, "trace": rec["trace"], "wallSeconds": res["wallSeconds"],
                "maxRssBytes": res["maxRssBytes"], "message": msg, "head": head}
    finally:
        if queued:
            try:
                queue.finish(pid_label, last_exit[0])
            except Exception:
                pass
        if leased:
            _remove_lease(kinds)
            _hook("released", lease)
        for f in (gfd, fd):
            if f is not None:
                try:
                    fcntl.flock(f, fcntl.LOCK_UN)
                finally:
                    os.close(f)


def concurrent_commands(argv, concurrent=None, commands_file=None):
    """E11 command list: the lines of commands_file (shell words, # comments), or P copies of argv."""
    if commands_file:
        with open(commands_file) as fh:
            cmds = [shlex.split(line) for line in fh if line.strip() and not line.lstrip().startswith("#")]
        if argv:
            raise ValueError("give either --commands FILE or a command after --, not both")
        if concurrent and concurrent != len(cmds):
            raise ValueError(f"--concurrent {concurrent} but {commands_file} has {len(cmds)} commands")
        return cmds
    if not argv:
        raise ValueError("heavy needs a command after `--`")
    return [list(argv) for _ in range(concurrent)]


def cmd_heavy(pid_label, budget_key, hypothesis, argv, gpu=False, deadline=None, max_wall=None, pipe_stdin=False,
              concurrent=None, commands_file=None, aggregate_cap=None):
    """CLI entry for `heavy --id ID [--budget KEY] [--hypothesis TEXT] [--concurrent P] -- CMD...`."""
    from . import predicates

    commands = None
    if concurrent or commands_file:
        if pipe_stdin:
            print("--pipe-stdin cannot be combined with --concurrent", file=sys.stderr)
            return 2
        try:
            commands = concurrent_commands(argv, concurrent, commands_file)
            aggregate_cap = parse_size(aggregate_cap) if aggregate_cap else None
        except (OSError, ValueError) as exc:
            print(f"heavy {pid_label}: {exc}", file=sys.stderr)
            return 2
        if len(commands) < 1:
            print(f"heavy {pid_label}: no commands", file=sys.stderr)
            return 2
        if aggregate_cap and aggregate_cap > MAX_AGGREGATE_CAP:
            print(f"heavy {pid_label}: aggregate cap clamped to {MAX_AGGREGATE_CAP // GIB} GiB", file=sys.stderr)

    if pipe_stdin:
        mode = os.fstat(0).st_mode
        if not (stat.S_ISFIFO(mode) or stat.S_ISSOCK(mode)):
            print("--pipe-stdin requires a pipe or socket; input is never traced", file=sys.stderr)
            return 2
    repo = repo_root()
    preds = None
    try:
        preds = predicates.load(repo)
    except predicates.PredicateError:
        pass
    known = {p.id: p for p in preds} if preds else {}
    if not budget_key and pid_label in known:
        budget_key = known[pid_label].budget
    try:
        cmd_str = shlex.join(argv) if commands is None else (
            f"concurrent {len(commands)}: " + " && ".join(shlex.join(c) for c in commands))
        r = run_heavy(pid_label, argv or commands[0], cmd_str, budget_key, hypothesis, repo,
                      gpu=gpu, deadline=deadline, max_wall=max_wall, pipe_stdin=pipe_stdin,
                      commands=commands, aggregate_cap=aggregate_cap)
    except Terminated as exc:
        print(f"heavy {pid_label}: terminated by signal {exc.args[0]}; job tree killed, locks released",
              file=sys.stderr)
        return 128 + int(exc.args[0])
    print(r["message"], file=sys.stderr)
    if r["exit"] not in (EXIT_UNFIT, EXIT_HYPOTHESIS, EXIT_NOBUDGET) and pid_label in known:
        status.update(preds, {pid_label: {
            "passes": r["exit"] == 0, "exit": r["exit"], "at": now_iso(), "run": r["run"],
        }}, increment=False, repo=repo)
    return r["exit"]
