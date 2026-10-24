"""AgentBus bridge. The only module that touches the `agentbus` CLI.

Laws (wiki-llm/doctorwatson-agentbus/DECISION.md): the bus is never on the critical path.
Every call here has a hard timeout (DW_BUS_TIMEOUT, default 1 s), swallows every error,
prints nothing (unless DW_BUS_DEBUG=1) and returns None/False on any failure. Rate caps and
a circuit breaker live in a small state file; bus failures never reach runs.jsonl,
iterationsSinceFlip or the stop rule. Messages are doorbells, never state, and no bus text
is ever returned to an agent prompt.
"""

import contextlib
import fcntl
import json
import math
import os
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time

from . import busevents as ev
from . import head_sha, now_iso, preflight, repo_root, sherlock_path

CAP_PER_MIN = 1
CAP_PER_HOUR = 30
BREAKER_FAILS = 3
BREAKER_COOLDOWN = 900.0
T3_MAX_WAIT = 3600
WAIT_CHUNK_MAX = 240
STOP_REPEAT_S = 3600
_BAD_EXIT_OK = (4,)  # a denied/queued answer means the bus is up
_latched = False  # one failure in this process silences the rest of the process


# ------------------------------------------------------------------ configuration
def debug(msg):
    if os.environ.get("DW_BUS_DEBUG") == "1":
        print(f"bus: {msg}", file=sys.stderr)


def enabled():
    return os.environ.get("DW_BUS", "auto").strip().lower() not in ("off", "0", "false", "no")


def run_name():
    return os.environ.get("DW_BUS_RUN") or "dw-moriarty"


def timeout():
    try:
        t = float(os.environ.get("DW_BUS_TIMEOUT", "1"))
        return t if 0.01 <= t <= 30 else 1.0
    except ValueError:
        return 1.0


def state_dir():
    return os.environ.get("DW_BUS_STATE") or os.path.join(os.path.dirname(preflight.lock_path()), "bus")


def cooldown():
    try:
        return float(os.environ.get("DW_BUS_COOLDOWN", BREAKER_COOLDOWN))
    except ValueError:
        return BREAKER_COOLDOWN


class Ident:
    """A pre-provisioned AgentBus binding: a directory that holds .agentbus/identity.json."""

    def __init__(self, dir, agent, run, did=None):
        self.dir, self.agent, self.run, self.did = dir, agent, run, did


def read_identity(dir):
    try:
        with open(os.path.join(dir, ".agentbus", "identity.json"), encoding="utf-8") as fh:
            d = json.load(fh)
        if isinstance(d, dict) and d.get("agent") and d.get("run"):
            return Ident(dir, str(d["agent"]), str(d["run"]), d.get("did"))
    except (OSError, ValueError):
        pass
    return None


def identity(repo=None):
    """AGENTBUS_IDENTITY_DIR, else <repo>/.agentbus. Never searched upward; never minted here."""
    try:
        d = os.environ.get("AGENTBUS_IDENTITY_DIR") or os.path.join(repo or repo_root(), "")
        d = os.path.abspath(d)
        ident = read_identity(d)
        return ident if ident and ident.run == run_name() else None
    except Exception:
        return None


# ------------------------------------------------------------------ state: breaker, caps, dedupe
@contextlib.contextmanager
def _state():
    d = state_dir()
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(d, "state.lock"), "a") as lk:
        fcntl.flock(lk, fcntl.LOCK_EX)
        path = os.path.join(d, "state.json")
        try:
            with open(path, encoding="utf-8") as fh:
                st = json.load(fh)
            if not isinstance(st, dict):
                st = {}
        except (OSError, ValueError):
            st = {}
        try:
            yield st
        finally:
            tmp = f"{path}.tmp.{os.getpid()}"
            try:
                with open(tmp, "w", encoding="utf-8") as fh:
                    json.dump(st, fh)
                os.replace(tmp, path)
            except OSError:
                pass
            fcntl.flock(lk, fcntl.LOCK_UN)


def breaker_open():
    """True while 3+ consecutive failures are younger than the cooldown."""
    try:
        with _state() as st:
            b = st.get("breaker") or {}
            return b.get("fails", 0) >= BREAKER_FAILS and time.time() - b.get("at", 0) < cooldown()
    except Exception:
        return False


def _note(ok):
    global _latched
    if not ok:
        _latched = True
    try:
        with _state() as st:
            b = st.setdefault("breaker", {"fails": 0, "at": 0})
            if ok:
                b["fails"] = 0
            else:
                b["fails"] = b.get("fails", 0) + 1
                b["at"] = time.time()
    except Exception:
        pass


def breaker_reset():
    global _latched
    _latched = False
    try:
        with _state() as st:
            st["breaker"] = {"fails": 0, "at": 0}
    except Exception:
        pass


def reset_process_latch():
    global _latched
    _latched = False


def allow_emit(key, now=None):
    """Cap: at most 1 per `key` per minute and 30 per hour. Records the emit if allowed."""
    now = time.time() if now is None else now
    try:
        with _state() as st:
            marks = [t for t in (st.setdefault("emits", {}).get(key) or []) if now - t < 3600]
            if marks and now - marks[-1] < 60:
                st["emits"][key] = marks
                return False
            if len(marks) >= CAP_PER_HOUR:
                st["emits"][key] = marks
                return False
            marks.append(now)
            st["emits"][key] = marks
            return True
    except Exception:
        return False


# ------------------------------------------------------------------ process runner
def _exec(argv, ident=None, tmo=None, want_out=False, env_extra=None):
    """Run `agentbus ARGV`. Returns (rc, stdout_text). rc is None on timeout/OSError/any error.

    Own session + killpg on timeout, so an orphaned grandchild cannot hold us past the limit.
    """
    tmo = timeout() if tmo is None else tmo
    out = None
    proc = None
    try:
        exe = shutil.which("agentbus")
        if not exe:
            return None, ""
        env = dict(os.environ)
        for k in ("AGENTBUS_RUN", "AGENTBUS_AGENT", "AGENTBUS_IDENTITY_DIR"):
            env.pop(k, None)
        if ident is not None:
            env["AGENTBUS_IDENTITY_DIR"] = ident.dir
        env.update(env_extra or {})
        out = tempfile.TemporaryFile() if want_out else None
        proc = subprocess.Popen([exe, *argv], stdin=subprocess.DEVNULL,
                                stdout=out if out is not None else subprocess.DEVNULL,
                                stderr=subprocess.DEVNULL, env=env, start_new_session=True,
                                cwd=os.path.expanduser("~"))
        try:
            rc = proc.wait(tmo)
        except subprocess.TimeoutExpired:
            debug(f"timeout {argv[0]}")
            return None, ""
        text = ""
        if out is not None:
            out.seek(0)
            text = out.read(1 << 20).decode("utf-8", "replace")
        return rc, text
    except BaseException as exc:  # includes KeyboardInterrupt only to clean up, then re-raised
        debug(f"error {argv[:1]}: {exc!r}")
        if isinstance(exc, (KeyboardInterrupt, SystemExit)):
            raise
        return None, ""
    finally:
        if proc is not None:
            if proc.poll() is None:
                with contextlib.suppress(Exception):
                    os.killpg(proc.pid, signal.SIGKILL)
                with contextlib.suppress(Exception):
                    proc.wait(1)
            else:
                with contextlib.suppress(Exception):
                    os.killpg(proc.pid, signal.SIGKILL)  # stragglers
        if out is not None:
            with contextlib.suppress(Exception):
                out.close()


def available(repo=None, ident=None):
    """No network call: DW_BUS on, agentbus on PATH, an identity for DW_BUS_RUN, breaker closed."""
    try:
        if not enabled() or _latched or not shutil.which("agentbus"):
            return False
        if (ident or identity(repo)) is None:
            return False
        return not breaker_open()
    except Exception:
        return False


def call(argv, ident=None, want_out=False, tmo=None):
    """Guarded agentbus call. Returns (ok, rc, text); ok False means skipped or failed."""
    try:
        if not available(ident=ident):
            return False, None, ""
        ident = ident or identity()
        rc, text = _exec(argv, ident, tmo, want_out)
        good = rc == 0 or rc in _BAD_EXIT_OK
        _note(good)
        return rc == 0, rc, text
    except Exception:
        return False, None, ""


# ------------------------------------------------------------------ primitives
def publish_ex(etype, data, repo=None, ident=None, to=None, role="driver", body=None):
    """Returns (message id or None, reason). reason: sent|off|capped|failed|oversize."""
    try:
        repo = repo or repo_root()
        ident = ident or identity(repo)
        if not available(repo, ident):
            return None, "off"
        target, ttl = ev.ROUTES[etype]
        target = to or target
        b = body or ev.fit(ev.make_event(etype, data, head_sha(repo), now_iso(), role))
        if b is None:
            return None, "oversize"
        if not allow_emit(f"{ident.agent}:{etype}"):
            return None, "capped"
        ok, rc, text = call(["send", target, etype, "--body", ev.canonical(b), "--ttl", str(ttl)], ident, True)
        if not ok:
            return None, "failed"
        try:
            return json.loads(text.strip().splitlines()[-1]).get("sent") or "sent", "sent"
        except (ValueError, IndexError, AttributeError):
            return "sent", "sent"
    except Exception:
        return None, "failed"


def publish(etype, data, **kw):
    return publish_ex(etype, data, **kw)[0]


def publish_body(body, repo=None, ident=None, to=None):
    """Publish a prebuilt event body (used by `bus publish` and by spawn)."""
    return publish_ex(body.get("type"), body.get("data"), repo=repo, ident=ident, to=to, body=body)


def kv_set(key, value, ident=None, repo=None):
    try:
        ident = ident or identity(repo)
        if not available(repo, ident) or not allow_emit(f"{ident.agent}:kv.{key}"):
            return False
        s = ev.canonical(value)
        if len(s.encode()) > ev.MAX_BODY:
            return False
        return call(["kv", "set", key, s], ident)[0]
    except Exception:
        return False


def kv_get(key, ident=None, repo=None):
    try:
        ok, rc, text = call(["kv", "get", key], ident or identity(repo), True)
        if not ok:
            return None
        rec = json.loads(text.strip().splitlines()[-1])
        val = rec.get("value")
        return json.loads(val) if isinstance(val, str) else val
    except Exception:
        return None


def claim(resource, lease_min=30, ident=None, repo=None):
    """Non-blocking advisory claim (no wait). Returns the hold token on a grant, else None.
    A queued request is withdrawn at once: a claim here is visibility, never a queue slot."""
    try:
        ident = ident or identity(repo)
        ok, rc, text = call(["claim", resource, "--lease", str(int(lease_min))], ident, True)
        rec = _last_json(text)
        if ok and rec.get("kind") == "grant":
            return rec.get("hold") or rec.get("request")
        if rc == 4 and rec.get("status") == "queued":
            call(["release", resource], ident)
    except Exception:
        pass
    return None


def release(resource, hold, ident=None, repo=None):
    try:
        argv = ["release", resource] + (["--hold", hold] if hold else [])
        return call(argv, ident or identity(repo))[0]
    except Exception:
        return False


def _last_json(text):
    for line in reversed((text or "").strip().splitlines()):
        try:
            d = json.loads(line)
            if isinstance(d, dict):
                return d
        except ValueError:
            continue
    return {}


def _jsonl(text):
    out = []
    for line in (text or "").splitlines():
        try:
            d = json.loads(line)
        except ValueError:
            continue
        if isinstance(d, dict):
            out.append(d)
    return out


def inbox(kind=None, ident=None, repo=None):
    """Peek the identity's inbox (never consumes). Returns raw envelope dicts, [] on failure."""
    try:
        argv = ["recv", "--peek"] + (["--kind", kind] if kind else [])
        ok, rc, text = call(argv, ident or identity(repo), True, tmo=max(timeout(), 3.0))
        return _jsonl(text) if ok else []
    except Exception:
        return []


def log_lines(since="1h", grep="doctorwatson.bus/1", ident=None, repo=None):
    try:
        ok, rc, text = call(["log", "--since", since, "--grep", grep], ident or identity(repo), True,
                            tmo=max(timeout(), 3.0))
        return _jsonl(text) if ok else []
    except Exception:
        return []


def mint(name, role, dir, chans=(), claims=(), ttl="2d", tmo=None):
    """Provision a worker identity (operator-level act, spawn only). Reuses an existing binding.
    Returns Ident or None. Not subject to the 1 s limit (it creates keys and a broker session)."""
    try:
        have = read_identity(dir)
        if have and have.agent == name:
            return have
        if not enabled() or breaker_open():
            return None
        os.makedirs(dir, exist_ok=True)
        argv = ["id", "new", "--run", run_name(), "--as", name, "--role", role,
                "--identity-dir", dir, "--ttl", ttl, "--no-register"]
        for c in chans:
            argv += ["--chan", c]
        for c in claims:
            argv += ["--claims", c]
        tmo = tmo or float(os.environ.get("DW_BUS_MINT_TIMEOUT", "15"))
        rc, _ = _exec(argv, None, tmo)
        _note(rc == 0)
        return read_identity(dir) if rc == 0 else None
    except Exception:
        return None


# ------------------------------------------------------------------ roles.json
def load_roles(repo=None):
    try:
        with open(sherlock_path("roles.json", repo), encoding="utf-8") as fh:
            d = json.load(fh)
        return d if isinstance(d, dict) else {}
    except (OSError, ValueError):
        return {}


# ------------------------------------------------------------------ hooks (called from cli/heavy/stoprule)
def safe(fn, *a, **kw):
    """Run a hook; nothing it does may raise or print."""
    try:
        return fn(*a, **kw)
    except Exception as exc:
        debug(f"hook error: {exc!r}")
        return None


def snapshot(repo=None):
    """status.json as it is before `check` rewrites it (None on any problem)."""
    from . import status
    return safe(status.load, repo)


def after_check(repo, prev, new, results):
    """status.flip / status.tick after `check` has written status.json and runs.jsonl."""
    from . import status
    if not new or not available(repo):
        return
    prev_entries = status.entries_of(prev)
    body = ev.check_event(prev_entries, new, results, head_sha(repo), now_iso())
    etype = body["type"]
    publish_ex(etype, body["data"], repo=repo, body=ev.fit(body))
    kv_set("status", ev.digest(new), repo=repo)
    if etype == "status.flip":
        _clear_stop(repo)


def _clear_stop(repo):
    try:
        with _state() as st:
            was = st.pop("stop_open", None)
        if was:
            kv_set("stop", {"open": False}, repo=repo)
    except Exception:
        pass


def hook_stop(result, repo=None):
    """stop.escalate on the CONTINUE->STOP transition; repeated at most hourly per key."""
    from . import status
    repo = repo or repo_root()
    if not result.get("stop") or not available(repo):
        return
    st = status.load(repo)
    key = ev.stop_key(result, st)
    now = time.time()
    with _state() as s:
        last = (s.get("dedupe") or {}).get("stop.escalate") or {}
        if last.get("key") == key and now - last.get("at", 0) < STOP_REPEAT_S:
            return
    body = ev.stop_event(result, st, head_sha(repo), now_iso())
    mid, why = publish_ex("stop.escalate", body["data"], repo=repo, body=ev.fit(body))
    if mid:
        with _state() as s:
            s.setdefault("dedupe", {})["stop.escalate"] = {"key": key, "at": now}
            s["stop_open"] = key
        kv_set("stop", {"open": True, "key": key, "firstFailing": body["data"].get("firstFailing"),
                        "trace": body["data"].get("trace")}, repo=repo)


class HeavyBus:
    """Visibility around one heavy run. Every method is safe; the flock stays authoritative."""

    def __init__(self, pid, budget_key, repo):
        self.pid, self.budget_key, self.repo = pid, budget_key, repo
        self.slot = os.environ.get("DW_HEAVY_SLOT") or (
            "gpu0" if "gpu" in f"{pid} {budget_key or ''}".lower() else "cpu-heavy")
        self.hold = None
        self.started_ok = False

    def begin(self):
        if not safe(available, self.repo):
            return
        lease = 60
        try:
            from . import budgets
            cap = budgets.limits_for(self.budget_key, self.repo) if self.budget_key else None
            if cap and cap.get("wallSeconds"):
                lease = min(240, max(10, math.ceil(cap["wallSeconds"] / 60) + 5))
        except Exception:
            pass
        self.hold = safe(claim, f"slot:{self.slot}", lease, repo=self.repo)

    def started(self, cap, hypothesis):
        def go():
            if not available(self.repo):
                return
            wall = (cap or {}).get("wallSeconds")
            body = ev.job_start_event(self.pid, self.slot, self.budget_key, wall, hypothesis,
                                      head_sha(self.repo), now_iso())
            publish_ex("job.start", body["data"], repo=self.repo, body=ev.fit(body))
            self.started_ok = True
        safe(go)

    def finished(self, rec):
        def go():
            if not self.started_ok or not available(self.repo):
                return
            body = ev.job_end_event(rec, self.slot, head_sha(self.repo), now_iso())
            publish_ex("job.end", body["data"], repo=self.repo, body=ev.fit(body))
        safe(go)

    def close(self):
        if self.hold:
            safe(release, f"slot:{self.slot}", self.hold, repo=self.repo)
            self.hold = None


def heavy_bus(pid, budget_key, repo):
    try:
        return HeavyBus(pid, budget_key, repo)
    except Exception:
        class Null:
            begin = close = lambda self: None
            started = finished = lambda self, *a: None
        return Null()


# ------------------------------------------------------------------ `bus` subcommands
def _ident_line(ident):
    return f"{ident.agent} (run {ident.run}) at {ident.dir}" if ident else "none"


def cmd_doctor(accept_off=False):
    repo = repo_root()
    ident = identity(repo)
    exe = shutil.which("agentbus")
    print(f"DW_BUS={os.environ.get('DW_BUS', 'auto')} run={run_name()} timeout={timeout()}s")
    print(f"agentbus: {exe or 'not on PATH'}")
    print(f"identity: {_ident_line(ident)}")
    roles = load_roles(repo)
    if ident:
        pinned = (roles.get("agents") or {}).get(ident.agent)
        print(f"roles.json: {'pinned as ' + pinned.get('agentbusRole', '?') if pinned else 'identity not pinned'}")
    with _state() as st:
        b = st.get("breaker") or {}
    print(f"breaker: fails={b.get('fails', 0)}" + (" OPEN" if breaker_open() else " closed"))
    if not enabled() or not exe or not ident:
        why = "DW_BUS=off" if not enabled() else ("agentbus not installed" if not exe else
              f"no identity for run {run_name()} (mint one: agentbus id new --run {run_name()} "
              f"--as dw-driver --role implementer --claims slot: --chan '*' --identity-dir {repo})")
        print(f"bus off: {why}")
        return 0 if accept_off else 5
    rc, text = _exec(["doctor"], ident, float(os.environ.get("DW_BUS_DOCTOR_TIMEOUT", "20")), True)
    for rec in _jsonl(text):
        print(f"  {'ok  ' if rec.get('ok') else 'FAIL'} {rec.get('check')}")
    if rc == 0:
        breaker_reset()
        print("bus doctor: ok")
        return 0
    print("bus doctor: UNAVAILABLE (informational: the factory continues without the bus)")
    return 5


def cmd_status():
    repo = repo_root()
    if not available(repo):
        print("bus unavailable")
        return 5
    got = False
    for key in ("status", "stop"):
        val = kv_get(key, repo=repo)
        if isinstance(val, dict):
            got = True
            print(f"kv {key}: " + json.dumps(ev._safe(val), separators=(",", ":"), sort_keys=True))
    if not got:
        print("kv: (empty or unavailable)")
    return 0


def _print_events(envs, repo):
    from . import status
    st = status.load(repo)
    seen = set()
    pinned = load_roles(repo).get("agents") or {}
    n = 0
    for env in envs:
        label, view = ev.classify(env, seen, (st or {}).get("updated"), pinned)
        print(ev.render(label, view))
        n += 1
    return n


def cmd_tail(since):
    repo = repo_root()
    if not available(repo):
        print("bus unavailable")
        return 5
    since = since if re.match(r"^\d{1,4}[smhd]$", since or "") else "1h"
    envs = log_lines(since, repo=repo)
    if not _print_events(envs, repo):
        print("(no doctorwatson.bus/1 events)")
    return 0


def cmd_inbox():
    repo = repo_root()
    if not available(repo):
        print("bus unavailable")
        return 5
    envs = inbox(repo=repo)
    if not _print_events(envs, repo):
        print("(inbox empty)")
    return 0


def cmd_publish(etype, data_json, to):
    repo = repo_root()
    if etype not in ev.TYPES:
        print(f"error: unknown event type {etype!r}; one of {', '.join(ev.TYPES)}", file=sys.stderr)
        return 2
    try:
        data = json.loads(data_json or "{}")
        if not isinstance(data, dict):
            raise ValueError("not an object")
    except ValueError as exc:
        print(f"error: --data must be a JSON object ({exc})", file=sys.stderr)
        return 2
    mid, why = publish_ex(etype, data, repo=repo, to=to, role="lead")
    if mid:
        print(f"sent {etype} {mid}")
        return 0
    print(f"not sent: {why}")
    return 4 if why == "capped" else 5


def t3_wait():
    """3600 s, or a lower DW_T3_WAIT (tests). The limit can be lowered, never raised."""
    try:
        return max(0.0, min(float(os.environ.get("DW_T3_WAIT", T3_MAX_WAIT)), T3_MAX_WAIT))
    except ValueError:
        return float(T3_MAX_WAIT)


def wait_plan(total=None, chunk=WAIT_CHUNK_MAX):
    """Chunk lengths (seconds) for the one bounded wait; each <= 240, sum <= 3600."""
    total = min(t3_wait() if total is None else total, T3_MAX_WAIT)
    out = []
    while total > 0:
        c = min(chunk, WAIT_CHUNK_MAX, total)
        out.append(c)
        total -= c
    return out


def cmd_review(rng, t3, out, base_kind="cold-diff"):
    """review.request; with --t3 also run the cold reviewer and wait (bounded) for it.

    Never blocks anything but itself and always exits 0. The only timed wait in the factory:
    at most 3600 s, polled in chunks of <= 240 s, then `proceeding: no review in 1h`.
    """
    repo = repo_root()
    base = (rng or "").partition("..")[0]
    kind = "t3-promotion" if t3 else base_kind
    body = ev.review_request_event(kind, base or None, rng, head_sha(repo), now_iso())
    mid, why = publish_ex("review.request", body["data"], repo=repo, body=ev.fit(body), role="lead")
    print(f"review.request {'sent ' + str(mid) if mid else 'not sent (' + why + ')'}")
    if not t3:
        return 0
    out = out or os.path.join(repo, "qa/sherlock/traces", f"review-{time.strftime('%Y%m%dT%H%M%SZ', time.gmtime())}.txt")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    prompt = (f"You are a cold reviewer with no prior context. Review the changes in this repository from "
              f"{base or 'origin/main'} to HEAD (use git diff {base or 'origin/main'}...HEAD and git log "
              f"{base or 'origin/main'}..HEAD). Look for soundness bugs, unchecked assumptions, regressions, "
              f"tests that cannot fail, and gates that can be bypassed. List findings by severity with file "
              f"and line. Do not modify anything.")
    env = {k: v for k, v in os.environ.items() if k not in ("OPENAI_API_KEY", "CODEX_API_KEY", "XAI_API_KEY")}
    codex = os.environ.get("DW_CODEX_BIN") or "codex"
    argv = [codex, "exec", "-m", "gpt-6.1-sol", "-c", 'model_reasoning_effort="high"', "-s", "read-only",
            "--skip-git-repo-check", "--ephemeral", "-C", repo, "-o", out, prompt]
    try:
        proc = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                                stderr=subprocess.DEVNULL, env=env, start_new_session=True)
    except OSError as exc:
        print(f"review not started: {exc}")
        print("proceeding: no review")
        return 0
    print(f"T3 review running (pid {proc.pid}) -> {out}")
    deadline = time.monotonic() + t3_wait()
    for chunk in wait_plan():
        try:
            proc.wait(min(chunk, max(0.0, deadline - time.monotonic())))
            break
        except subprocess.TimeoutExpired:
            if mid and any(_corr_of(e) == mid for e in inbox("msg", repo=repo)):
                break  # an external reviewer answered
    if proc.poll() is None:
        with contextlib.suppress(Exception):
            os.killpg(proc.pid, signal.SIGKILL)
        print("proceeding: no review in 1h")
        return 0
    sha = ""
    try:
        import hashlib
        with open(out, "rb") as fh:
            sha = hashlib.sha256(fh.read()).hexdigest()
    except OSError:
        pass
    verdict = "completed" if sha else "empty"
    reply = ev.review_reply_event(mid, verdict, os.path.relpath(out, repo), sha, head_sha(repo), now_iso())
    publish_ex("review.reply", reply["data"], repo=repo, body=ev.fit(reply), role="reviewer")
    print(f"review finished exit={proc.returncode} -> {out}")
    return 0


def _corr_of(env):
    try:
        b = json.loads(env.get("body") or "")
        return b["data"].get("corr") if b.get("type") == "review.reply" else None
    except (ValueError, AttributeError, KeyError, TypeError):
        return None


def cmd_watch(max_time):
    """Wake the lead: agentbus watch --exit-on-first on stop.escalate | review.reply | status.flip."""
    repo = repo_root()
    ident = identity(repo)
    exe = shutil.which("agentbus")
    if not (enabled() and exe and ident):
        print("bus off: nothing to watch")
        return 75
    types = "|".join(re.escape(t) for t in ev.WAKE_TYPES)
    run_dir = os.path.join(state_dir(), "watch")
    try:  # agentbus watch looks for <run-dir>/<agent>/.agentbus/identity.json
        os.makedirs(os.path.join(run_dir, ident.agent), exist_ok=True)
        link = os.path.join(run_dir, ident.agent, ".agentbus")
        if not os.path.lexists(link):
            os.symlink(os.path.join(ident.dir, ".agentbus"), link)
    except OSError:
        pass
    argv = [exe, "--run", ident.run, "--as", ident.agent, "watch", "--run-dir", run_dir, "--to", f"{ident.agent},*", "--kinds", "msg",
            "--match", rf"(?:^|\s)(?:{types})$", "--exit-on-first"]
    if max_time:
        argv += ["--max-time", str(int(max_time))]
    env = dict(os.environ, AGENTBUS_IDENTITY_DIR=ident.dir)
    env.pop("AGENTBUS_RUN", None)
    env.pop("AGENTBUS_AGENT", None)
    try:  # agentbus prints peer subjects; only typed, allowlisted fields go to the lead's context
        r = subprocess.run(argv, env=env, stdin=subprocess.DEVNULL, capture_output=True, text=True)
    except OSError:
        return 75
    pat = re.compile(rf"^\S+ msg ([A-Za-z0-9_.-]{{1,64}})->\S+ ({types.replace(chr(92), '')})(?:\s|$)")
    for line in r.stdout.splitlines():
        m = pat.match(line)
        if m:
            print(f"[wake] type={m.group(2)} from={m.group(1)}")
    return r.returncode
