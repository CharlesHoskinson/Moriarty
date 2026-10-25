"""Run a command with measurement, tee-to-trace, budget caps and the heavy lock."""

import datetime
import fcntl
import json
import os
import re
import shlex
import signal
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


def run_command(argv, cwd, trace_path, echo, cap=None, marker=None, on_start=None, watch_parent=False):
    """Run argv in its own session; tee merged stdout/stderr to trace_path (and stdout if echo).

    Returns {exit, wallSeconds, maxRssBytes, exceeded} where exceeded is None, "wall" or "rss".
    """
    cap = cap or {}
    cap_wall = cap.get("wallSeconds")
    cap_rss = cap.get("maxRssBytes")
    os.makedirs(os.path.dirname(trace_path), exist_ok=True)
    log = open(trace_path, "wb")
    t0 = time.monotonic()
    env = dict(os.environ, DW_JOB_ID=marker) if marker else None
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
            argv, cwd=cwd, stdin=subprocess.DEVNULL, env=env,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True,
        )
    except OSError as exc:
        msg = f"cannot start {argv[0]!r}: {exc}\n".encode()
        log.write(msg)
        log.close()
        if echo:
            _echo(msg)
        _restore(old_handlers)
        return {"exit": 127, "wallSeconds": 0.0, "maxRssBytes": 0, "exceeded": None}
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
                        if peak > cap_rss:
                            exceeded = "rss"
                            kill_job_tree(pgid, marker)
                time.sleep(0.02)
    except BaseException:
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
    return {"exit": code, "wallSeconds": round(wall, 3), "maxRssBytes": rss, "exceeded": exceeded}


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
              gpu=False, deadline=None, max_wall=None):
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
                          gpu, deadline, max_wall)
    finally:
        hb.close()


def _run_heavy(pid_label, argv, cmd_str, budget_key, hypothesis, repo, echo, err, hb,
               gpu=False, deadline=None, max_wall=None):
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

    kinds = ["heavy", "gpu"] if gpu else ["heavy"]
    fd = preflight.open_lock()
    gfd = None
    leased = False
    queued = False
    last_exit = [None]
    try:
        ok, why = queue.acquire_turn(fd, pid_label, cmd_str, max_wall=eff_wall(budget_key, deadline, max_wall, repo),
                                     budget=budget_key,
                                     timeout=None if deadline is None else max(0.0, deadline - time.time()))
        if not ok:
            return {"exit": EXIT_UNFIT, "message": "host unfit: another process holds the heavy lock (" + (why or "queue") + ")"}
        queued = True
        if gpu:
            gfd = preflight.open_lock(preflight.gpu_lock_path())
            if not preflight.try_lock(gfd):
                return {"exit": EXIT_UNFIT, "message": "host unfit: another process holds the gpu lock"}
        pre = preflight.run(repo, lock_held_by_us=True, skip=os.environ.get("DW_SKIP_PREFLIGHT") == "1")
        if not pre["ok"]:
            return {"exit": EXIT_UNFIT, "message": "host unfit: " + "; ".join(pre["reasons"]), "preflight": pre}

        cap = budgets.limits_for(budget_key, repo) if budget_key else None
        wall_cap = effective_wall(cap, deadline, max_wall)
        run_cap = dict(cap or {})
        run_cap["wallSeconds"] = wall_cap
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
        res = run_command(argv, repo, trace_abs, echo, run_cap, marker=rid, on_start=on_start,
                          watch_parent=os.environ.get("DW_WATCH_PARENT") == "1")
        code = res["exit"]
        last_exit[0] = code
        extra = {}
        if res["exceeded"]:
            extra = {"childExit": code, "capExceeded": res["exceeded"]}
            code = EXIT_BUDGET
        rec = run_record(rid, pid_label, cmd_str, hypothesis, res, code, pre, head, repo, extra)
        status.append_run(rec, repo)
        hb.finished(rec)
        if budget_key and cap is None and code == 0:
            budgets.record_measurement(budget_key, res["wallSeconds"], res["maxRssBytes"], rid, rec["at"], repo)
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


def cmd_heavy(pid_label, budget_key, hypothesis, argv, gpu=False, deadline=None, max_wall=None):
    """CLI entry for `heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD...`."""
    from . import predicates

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
        r = run_heavy(pid_label, argv, shlex.join(argv), budget_key, hypothesis, repo,
                      gpu=gpu, deadline=deadline, max_wall=max_wall)
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
