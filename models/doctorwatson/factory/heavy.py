"""Run a command with measurement, tee-to-trace, budget caps and the heavy lock."""

import datetime
import fcntl
import os
import re
import shlex
import signal
import subprocess
import sys
import threading
import time

from . import budgets, bus, head_sha, now_iso, preflight, repo_root, status

EXIT_UNFIT = 3
EXIT_BUDGET = 4
EXIT_HYPOTHESIS = 5


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


def _killpg(pgid):
    try:
        os.killpg(pgid, signal.SIGKILL)
    except (ProcessLookupError, PermissionError):
        pass


def run_command(argv, cwd, trace_path, echo, cap=None):
    """Run argv in its own session; tee merged stdout/stderr to trace_path (and stdout if echo).

    Returns {exit, wallSeconds, maxRssBytes, exceeded} where exceeded is None, "wall" or "rss".
    """
    cap = cap or {}
    cap_wall = cap.get("wallSeconds")
    cap_rss = cap.get("maxRssBytes")
    os.makedirs(os.path.dirname(trace_path), exist_ok=True)
    log = open(trace_path, "wb")
    t0 = time.monotonic()
    try:
        proc = subprocess.Popen(
            argv, cwd=cwd, stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True,
        )
    except OSError as exc:
        msg = f"cannot start {argv[0]!r}: {exc}\n".encode()
        log.write(msg)
        log.close()
        if echo:
            _echo(msg)
        return {"exit": 127, "wallSeconds": 0.0, "maxRssBytes": 0, "exceeded": None}
    pgid = proc.pid
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
        if cap_wall is None and cap_rss is None:
            _, st, ru = os.wait4(proc.pid, 0)
        else:
            next_rss = 0.0
            while True:
                pid, st, ru = os.wait4(proc.pid, os.WNOHANG)
                if pid:
                    break
                now = time.monotonic()
                if exceeded is None:
                    if cap_wall is not None and now - t0 > cap_wall:
                        exceeded = "wall"
                        _killpg(pgid)
                    elif cap_rss is not None and now >= next_rss:
                        next_rss = now + 0.2
                        peak = max(peak, group_rss(pgid))
                        if peak > cap_rss:
                            exceeded = "rss"
                            _killpg(pgid)
                time.sleep(0.02)
    except BaseException:
        _killpg(pgid)
        try:
            os.wait4(proc.pid, 0)
        except OSError:
            pass
        raise
    finally:
        wall = time.monotonic() - t0
    proc.returncode = 0  # reaped by wait4; keep Popen from warning
    _killpg(pgid)  # stragglers that would hold the pipe open
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


def run_heavy(pid_label, argv, cmd_str, budget_key=None, hypothesis=None, repo=None, echo=True, err=None):
    """Run one heavy command, with bus visibility (advisory claim, job.start/end) around it.

    The bus hooks never raise, print or change the result; the flock stays authoritative.
    """
    repo = repo or repo_root()
    hb = bus.heavy_bus(pid_label, budget_key, repo)
    hb.begin()
    try:
        return _run_heavy(pid_label, argv, cmd_str, budget_key, hypothesis, repo, echo, err, hb)
    finally:
        hb.close()


def _run_heavy(pid_label, argv, cmd_str, budget_key, hypothesis, repo, echo, err, hb):
    """Run one heavy command. Returns {exit, run, trace, wallSeconds, maxRssBytes, message}.

    exit is the child's exit, 3 (host unfit), 4 (budget exceeded) or 5 (hypothesis required).
    Exit 3 and 5 mean nothing ran, so nothing is logged and the retry rule is unaffected.
    """
    repo = repo or repo_root()
    err = err or sys.stderr
    head = head_sha(repo)

    prior = hypothesis_required(pid_label, cmd_str, head, hypothesis, repo)
    if prior is not None:
        msg = (f"hypothesis required: last run {prior.get('run')} of {pid_label} failed "
               f"(exit {prior.get('exit')}) with the same command at the same head; "
               f"read {prior.get('trace')} and retry with --hypothesis TEXT")
        return {"exit": EXIT_HYPOTHESIS, "message": msg}

    fd = preflight.open_lock()
    try:
        if not preflight.try_lock(fd):
            return {"exit": EXIT_UNFIT, "message": "host unfit: another process holds the heavy lock"}
        pre = preflight.run(repo, lock_held_by_us=True, skip=os.environ.get("DW_SKIP_PREFLIGHT") == "1")
        if not pre["ok"]:
            return {"exit": EXIT_UNFIT, "message": "host unfit: " + "; ".join(pre["reasons"]), "preflight": pre}

        cap = budgets.limits_for(budget_key, repo) if budget_key else None
        rid = run_id(pid_label, repo)
        trace_abs = os.path.join(repo, trace_rel(rid))
        hb.started(cap, hypothesis)
        res = run_command(argv, repo, trace_abs, echo, cap)
        code = res["exit"]
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
            msg += f" BUDGET EXCEEDED ({res['exceeded']}; cap={cap})"
        return {"exit": code, "run": rid, "trace": rec["trace"], "wallSeconds": res["wallSeconds"],
                "maxRssBytes": res["maxRssBytes"], "message": msg, "head": head}
    finally:
        try:
            fcntl.flock(fd, fcntl.LOCK_UN)
        finally:
            os.close(fd)


def cmd_heavy(pid_label, budget_key, hypothesis, argv):
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
    r = run_heavy(pid_label, argv, shlex.join(argv), budget_key, hypothesis, repo)
    print(r["message"], file=sys.stderr)
    if r["exit"] not in (EXIT_UNFIT, EXIT_HYPOTHESIS) and pid_label in known:
        status.update(preds, {pid_label: {
            "passes": r["exit"] == 0, "exit": r["exit"], "at": now_iso(), "run": r["run"],
        }}, increment=False, repo=repo)
    return r["exit"]
