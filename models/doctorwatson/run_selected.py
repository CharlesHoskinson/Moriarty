"""DoctorWatson explicit single-operation execution bridge.

One Root-selected task is dispatched at most once through a fixed adapter
registry to an existing operation boundary. Plan parsing and operation
fingerprint are the ordinary consumer's own ``delivery.load_plan`` /
``delivery.fingerprint``; prelaunch readiness is the ONE common path
``reconciliation.load_readiness`` (delivery.assess folded with validated
same-original-attempt reconciliations over digest-pinned raw
plan/history/reconciliations bytes), the same path the shell
``--inspect`` route consumes. This bridge has no second fingerprint
and no private readiness fold: the readiness identity resolved here is
pinned in PREEXEC and rechecked at actual use before dispatch.
Task/plan JSON can never supply argv, a shell string, caps, limits,
environment, or a success boolean: delivery's exact-field plan schema
refuses such keys.

After the single dispatch, fixed source-specific collectors convert the
actual raw producer files into the consumer Result shape exactly
(``delivery.load_result`` fields). The original PREEXEC and raw execution
receipt are preserved in the attempt directory and pinned by digest in a
collector sidecar; they are never renamed into a Result. Collectors:

- ``moriarty-sdk-build``: selected existing guard
  ``bounded_sdk_compile.py --cached-current-f`` packet contract
  (F_CACHED_CURRENT_RESULT / F_QUARANTINED_OUTPUTS /
  CACHED_POST_COMPILE_CHECK / F.stdout / F.stderr / fresh backend ELF).
  Exit 0 plus present files is not enough; see decode_cached_current_f.
- ``handkey-verify-pis``: normalized observation mechanically derived
  from the actual stdout JSON {verifies, verify_s} and the selected
  key/proof/statement pins. Recl9 text output is not accepted here.
- ``sherlock-anchor-settle-readback``: normalized observation derived
  only via settle_observation.decode_sherlock_settle_result from the
  raw harness source-result. Launch remains Root-only (blocked here);
  collection from Root-produced raw files is implemented.

Every other delivery adapter is an exact blocked interface: zero child
calls, never stub success. Importing or unit-testing this file with an
injected runner performs no native dispatch.
"""
from __future__ import annotations

import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
from typing import Any, Callable

import delivery  # sibling ordinary consumer in models/doctorwatson

import reconciliation
import settle_observation

MAX_JSON_BYTES = 4 * 1024 * 1024
HEX = re.compile(r"[0-9a-f]{64}\Z")
NAME = re.compile(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,127}\Z")

EXIT_OK = 0          # dispatched once; collector outcome is succeeded
EXIT_FAILED = 1      # dispatched once; operation/collector failed
EXIT_BAD_INPUT = 2   # schema/identity refusal before any attempt write
EXIT_BLOCKED = 3     # blocked before launch; zero child calls guaranteed
EXIT_UNKNOWN = 4     # dispatched (or possibly); outcome unknown-reconcile

# Fixed registry; adapter ids are exactly delivery.ROLES ids. ``launchable``
# is a code fact, never settable from JSON. Collectors are fixed per adapter.
ADAPTERS: dict[str, dict[str, Any]] = {
    "moriarty-sdk-build": {
        "launchable": True,
        "outerSequences": ("bounded-sdk-compile-cached-current-f/1",),
        "rawOutputRoles": ("f-result", "f-outputs", "post-compile-check",
                           "stdout", "stderr", "backend-elf",
                           "resource-receipt"),
        "collector": "collect_cached_current_f",
    },
    "handkey-verify-pis": {
        "launchable": False,
        "blockedReason": "selected independent handkey invocation scope "
                         "absent; collector implemented, launch blocked",
        "rawOutputRoles": ("stdout", "stderr", "resource-receipt",
                           "key", "proof", "statement"),
        "collector": "collect_handkey",
    },
    "sherlock-anchor-settle-readback": {
        "launchable": False,
        "blockedReason": "target execution is Root-only and unselected "
                         "here; no wallet/network authority in this bridge",
        "rawOutputRoles": ("stdout", "stderr", "resource-receipt",
                           "source-result", "post-state"),
        "collector": "collect_settle",
    },
}
for _adapter in delivery.ROLES:
    ADAPTERS.setdefault(_adapter, {
        "launchable": False,
        "blockedReason": f"no Root-selected operation/collector for "
                         f"{_adapter} in this revision",
        "rawOutputRoles": (),
        "collector": None,
    })


class BadInput(ValueError):
    """Malformed/unsupported input; EXIT_BAD_INPUT, zero calls."""


class Blocked(RuntimeError):
    """Valid input, blocked before launch; EXIT_BLOCKED, zero calls."""


class CollectError(ValueError):
    """Raw producer files do not decode; Result terminal is failed."""


def _pairs(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise BadInput(f"duplicate JSON member: {key}")
        out[key] = value
    return out


def _float(text):
    value = float(text)
    if not math.isfinite(value):
        raise BadInput("nonfinite JSON number")
    return value


def _constant(text):
    raise BadInput(f"non-JSON numeric constant: {text}")


def _canonical(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      allow_nan=False)


def _digest(value) -> str:
    return hashlib.sha256(_canonical(value).encode()).hexdigest()


def _read_raw(path: str, what: str, cap=MAX_JSON_BYTES) -> bytes:
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except OSError as exc:
        raise CollectError(f"{what}: cannot open {path}: {exc}") from exc
    try:
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_size > cap:
            raise CollectError(f"{what}: not a bounded regular file")
        raw = b""
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            raw += chunk
            if len(raw) > cap:
                raise CollectError(f"{what}: size limit exceeded")
        return raw
    finally:
        os.close(fd)


def _load_json_file(path: str, what: str):
    raw = _read_raw(path, what)
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=_pairs,
                          parse_constant=_constant, parse_float=_float)
    except (UnicodeError, json.JSONDecodeError, RecursionError,
            ValueError) as exc:
        raise BadInput(f"{what}: invalid JSON: {exc}") from exc


def _sha(value, at):
    if type(value) is not str or not HEX.fullmatch(value):
        raise BadInput(f"{at}: expected lowercase SHA-256")
    return value


def _file_ref(role: str, path: str) -> dict:
    try:
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    except OSError as exc:
        raise CollectError(f"{role}: output unavailable: {exc}") from exc
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode):
            raise CollectError(f"{role}: not a regular file")
        digest, total = hashlib.sha256(), 0
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            total += len(chunk)
            digest.update(chunk)
        after = os.fstat(fd)
        stamp = lambda s: (s.st_dev, s.st_ino, s.st_size, s.st_mtime_ns,
                           s.st_ctime_ns)
        if stamp(before) != stamp(after):
            raise CollectError(f"{role}: changed while reading")
        return {"role": role, "path": path, "bytes": total,
                "sha256": digest.hexdigest()}
    finally:
        os.close(fd)


def _verify_pinned(ref: dict, at: str) -> dict:
    """Verify a Root-selected {path,bytes,sha256} pin; refuse symlinks."""
    if type(ref) is not dict or set(ref) != {"path", "bytes", "sha256"}:
        raise BadInput(f"{at}: expected exactly path/bytes/sha256")
    path = ref["path"]
    if (type(path) is not str or not Path(path).is_absolute()
            or str(Path(os.path.normpath(path))) != path):
        raise BadInput(f"{at}.path: normalized absolute path required")
    size = ref["bytes"]
    if type(size) is not int or not 0 <= size <= 1024 * 1024 * 1024:
        raise BadInput(f"{at}.bytes: bounded integer required")
    want = _sha(ref["sha256"], at + ".sha256")
    try:
        got = _file_ref(at, path)
    except CollectError as exc:
        raise Blocked(f"{at}: selected bytes unavailable: {exc}") from exc
    if got["bytes"] != size or got["sha256"] != want:
        raise Blocked(f"{at}: selected bytes do not match pin")
    return {"path": path, "bytes": size, "sha256": want}


def resolve_task(plan_path: str, task_id: str, history_path: str | None = None,
                 reconciliations_path: str | None = None) -> dict:
    """Resolve one task through the common reconciled readiness path.

    Consumes exactly reconciliation.load_readiness (digest-pinned raw
    plan/history/reconciliations bytes; delivery.assess folded with
    validated reconciliations) -- the same function and file identities
    as shell --inspect. Anything other than a current selection
    candidate for this task is Blocked, zero calls. Root retains the
    actual selection/admission decision via the external selection
    file; readiness here never grants native permission by itself.
    Reconciliations without their raw history are BadInput.
    """
    try:
        readiness = reconciliation.load_readiness(
            plan_path, history_path, reconciliations_path)
    except (OSError, delivery.SchemaError,
            reconciliation.ReconciliationError,
            settle_observation.SettleDecodeError) as exc:
        raise BadInput(f"readiness inputs: {exc}") from exc
    plan = readiness["plan"]
    if task_id not in plan.by_id:
        raise BadInput(f"task {task_id!r}: not in plan")
    task = plan.by_id[task_id]
    if task.adapter not in ADAPTERS:
        raise BadInput(f"task.adapter: unknown adapter {task.adapter!r}")
    fingerprint = delivery.fingerprint(plan, task_id)
    row = next(r for r in readiness["report"]["tasks"] if r["id"] == task_id)
    if row["fingerprint"] != fingerprint:
        raise Blocked("readiness row fingerprint != consumer fingerprint")
    resolved = {"id": task.id, "adapter": task.adapter,
                "attempt": task.attempt, "fingerprint": fingerprint,
                "inputs": task.inputs, "plan": plan, "task_obj": task,
                "readiness": row,
                "readiness_identity": readiness["identity"]}
    if task.disposition != "active":
        raise Blocked(f"task disposition is {task.disposition}; not active")
    if not row["selection_candidate"]:
        raise Blocked(
            f"task not a current selection candidate "
            f"(state={row['state']}; blockers={row['blockers']})")
    return resolved


def _recheck_at_use(task: dict, selection: dict, plan_path: str,
                    selection_path: str, history_path, reconciliations_path):
    """Re-run the common readiness path at actual use; compare pins.

    Any drift in plan/history/reconciliations bytes, fingerprint,
    readiness, selection digest or guard/config/output pins since the
    first resolution blocks with zero dispatch. Returns the fresh
    (task, selection) so dispatch uses only rechecked values.
    """
    fresh_task = resolve_task(plan_path, task["id"], history_path,
                              reconciliations_path)
    fresh_selection = resolve_selection(selection_path, fresh_task)
    if (fresh_task["fingerprint"] != task["fingerprint"]
            or fresh_task["attempt"] != task["attempt"]
            or fresh_task["adapter"] != task["adapter"]
            or fresh_task["readiness_identity"] != task["readiness_identity"]
            or fresh_selection["selectionSha256"]
            != selection["selectionSha256"]
            or fresh_selection["guard"] != selection["guard"]
            or fresh_selection["config"] != selection["config"]
            or fresh_selection["outputs"] != selection["outputs"]
            or fresh_selection["outerSequence"]
            != selection["outerSequence"]):
        raise Blocked("readiness/selection identity changed before "
                      "dispatch; pinned at resolve, rechecked at use")
    return fresh_task, fresh_selection


def resolve_selection(selection_path: str, task: dict) -> dict:
    """Validate the external Root selection against the resolved task.

    The selection pins existing guard/config bytes and declares the raw
    output paths the selected operation writes. It never carries argv:
    the outer sequence id is resolved by code below. Raw output roles
    must exactly equal the fixed collector's required raw membership.
    """
    sel = _load_json_file(selection_path, "selection")
    keys = {"schema", "task", "adapter", "fingerprint", "outerSequence",
            "guard", "config", "outputs"}
    if type(sel) is not dict or set(sel) != keys:
        raise BadInput(f"selection: expected exactly {sorted(keys)}")
    if sel["schema"] != "doctorwatson.execution-selection/1":
        raise BadInput("selection.schema: unsupported")
    if sel["task"] != task["id"] or sel["adapter"] != task["adapter"]:
        raise Blocked("selection task/adapter does not match plan task")
    if sel["fingerprint"] != task["fingerprint"]:
        raise Blocked("selection fingerprint stale vs consumer fingerprint "
                      "(source/dependency/input mismatch); zero dispatch")
    spec = ADAPTERS[task["adapter"]]
    if not spec["launchable"]:
        raise Blocked(f"adapter {task['adapter']}: {spec['blockedReason']}")
    if sel["outerSequence"] not in spec["outerSequences"]:
        raise BadInput("selection.outerSequence: not a fixed sequence "
                       f"for {task['adapter']}")
    guard = _verify_pinned(sel["guard"], "selection.guard")
    config = _verify_pinned(sel["config"], "selection.config")
    outputs = sel["outputs"]
    if type(outputs) is not list or not outputs:
        raise BadInput("selection.outputs: nonempty array required")
    seen, norm_outputs = set(), []
    for item in outputs:
        if type(item) is not dict or set(item) != {"role", "path"}:
            raise BadInput("selection.outputs[]: exactly role/path")
        role, path = item["role"], item["path"]
        if type(role) is not str or not role:
            raise BadInput("outputs.role: nonempty string required")
        if (type(path) is not str or not Path(path).is_absolute()
                or str(Path(os.path.normpath(path))) != path):
            raise BadInput("outputs.path: normalized absolute required")
        if role in seen:
            raise BadInput(f"outputs: duplicate role {role}")
        seen.add(role)
        norm_outputs.append({"role": role, "path": path})
    if seen != set(spec["rawOutputRoles"]):
        raise Blocked(f"selection.outputs roles {sorted(seen)} != fixed "
                      f"collector raw membership "
                      f"{sorted(spec['rawOutputRoles'])}")
    return {"outerSequence": sel["outerSequence"], "guard": guard,
            "config": config, "outputs": norm_outputs,
            "selectionSha256": _digest(sel)}


def build_argv(task: dict, selection: dict) -> list[str]:
    """Code-fixed argv for the one launchable outer sequence."""
    assert selection["outerSequence"] == "bounded-sdk-compile-cached-current-f/1"
    return [sys.executable, selection["guard"]["path"],
            "--cached-current-f",
            "--config", selection["config"]["path"],
            "--config-sha256", selection["config"]["sha256"]]


def _write_json_atomic(path: Path, value) -> dict:
    tmp = path.with_suffix(path.suffix + ".tmp")
    data = (_canonical(value) + "\n").encode()
    fd = os.open(tmp, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    try:
        os.write(fd, data)
        os.fsync(fd)
    finally:
        os.close(fd)
    os.rename(tmp, path)
    return {"path": str(path), "bytes": len(data),
            "sha256": hashlib.sha256(data).hexdigest()}


def _terminal_fields(returncode):
    """Map a process returncode to consumer terminal/exit/signal fields."""
    if returncode == 0:
        return "succeeded", 0, None
    if returncode > 0:
        return "failed", min(returncode, 255), None
    return "failed", None, min(-returncode, 128)


def _check_cleanup_receipt(task: dict, raw_by_role: dict) -> str:
    """Validate the actual cleanup-observation receipt; return its state.

    The guard packet emits no cleanup/restoration receipt of its own;
    the outer operator's doctorwatson.cleanup-observation/1 (the
    consumer's existing contract) is the cleanup evidence. Anything
    else yields 'unknown'/'owed', never a synthesized complete.
    """
    path = raw_by_role.get("resource-receipt")
    if path is None:
        return "unknown"
    try:
        value = json.loads(_read_raw(path, "resource-receipt").decode(),
                           object_pairs_hook=_pairs,
                           parse_constant=_constant, parse_float=_float)
    except Exception:
        return "unknown"
    expected = {"schema": "doctorwatson.cleanup-observation/1",
                "task": task["id"], "attempt": task["attempt"],
                "fingerprint": task["fingerprint"]}
    if type(value) is not dict or set(value) != {
            "schema", "task", "attempt", "fingerprint", "cleanup",
            "restoration"}:
        return "unknown"
    if any(value[k] != v for k, v in expected.items()):
        return "unknown"
    states = (value["cleanup"], value["restoration"])
    if any(s not in ("complete", "owed", "unknown") for s in states):
        return "unknown"
    return "complete" if states == ("complete", "complete") else \
        ("owed" if "owed" in states else "unknown")


def _observation_ref(task: dict, payload: dict, attempt_dir: Path) -> dict:
    value = {"schema": "doctorwatson." + task["adapter"] + "-observation/1",
             "task": task["id"], "attempt": task["attempt"],
             "fingerprint": task["fingerprint"], **payload}
    ref = _write_json_atomic(attempt_dir / "observation.json", value)
    return {"role": "observation", **ref}


def _result_dict(task, terminal, exit_code, signal, cleanup, submission,
                 artifacts):
    value = {"task": task["id"], "attempt": task["attempt"],
             "fingerprint": task["fingerprint"],
             "adapter": task["adapter"], "terminal": terminal,
             "exit_code": exit_code, "signal": signal,
             "cleanup": cleanup, "submission": submission,
             "artifacts": artifacts, "review": None, "adoption": None}
    delivery.load_result(value)  # collector output must load as-is
    return value


def decode_cached_current_f(raw_by_role: dict) -> dict:
    """Decode the selected guard's actual cached-current-F packet.

    Contract read from the current native-compile-owner successor guard
    (bounded_sdk_compile.py e0f7dbf5..., cached_current_build.py
    190e1c56...): supervisor --cached-current-f writes packet
    F_CACHED_CURRENT_RESULT.json {phase:F, route:cached-current-f,
    base_head, cargo_lock_sha256, historical_debit_state_start/end,
    source_home_reconstitution, cached_target_seed_counts,
    post_compile_check, f_result{phase,processes[{argv,audit_worker,
    child_exit}],environment,wall_seconds,log_bytes}}; the F audit
    worker writes F_QUARANTINED_OUTPUTS.json {all_outputs{rel:sha256},
    host_outputs, host_fingerprints, build_script_outputs,
    acceptance:UNACCEPTED...}; verify_post_compile_no_script_reexec
    writes CACHED_POST_COMPILE_CHECK.json
    {host_outputs_verified_unchanged,
    build_script_run_units_verified_not_reexecuted,
    fresh_mori_backend_executables[{relative,sha256,bytes}]}.
    A packet TERMINAL_FAILURE.json, any child_exit != 0, an ELF absent
    from fresh/all_outputs, or a stale seeded ELF all fail here. The
    acceptance string stays UNACCEPTED: built is an observation, not
    acceptance. Test bodies are never executed by compile (cargo
    --no-run phase F), so test_bodies_executed is False by the guard's
    own operation definition, not by an author flag.
    """
    def load(role):
        raw = _read_raw(raw_by_role[role], role)
        try:
            return json.loads(raw.decode(), object_pairs_hook=_pairs,
                              parse_constant=_constant, parse_float=_float)
        except Exception as exc:
            raise CollectError(f"{role}: invalid JSON: {exc}") from exc

    f_result_doc = load("f-result")
    keys = {"phase", "route", "base_head", "cargo_lock_sha256",
            "historical_debit_state_start", "historical_debit_state_end",
            "source_home_reconstitution", "cached_target_seed_counts",
            "post_compile_check", "f_result"}
    if type(f_result_doc) is not dict or set(f_result_doc) != keys:
        raise CollectError("F_CACHED_CURRENT_RESULT: exact fields required")
    if f_result_doc["phase"] != "F" \
            or f_result_doc["route"] != "cached-current-f":
        raise CollectError("F_CACHED_CURRENT_RESULT: phase/route mismatch")
    inner = f_result_doc["f_result"]
    if type(inner) is not dict or set(inner) != {
            "phase", "processes", "environment", "wall_seconds",
            "log_bytes"} or inner["phase"] != "F":
        raise CollectError("f_result: exact launch record required")
    processes = inner["processes"]
    if type(processes) is not list or not processes or any(
            type(p) is not dict or set(p) != {"argv", "audit_worker",
                                              "child_exit"}
            or p["child_exit"] != 0 or type(p["audit_worker"]) is not bool
            for p in processes):
        raise CollectError("f_result.processes: every child_exit must be 0")
    check = load("post-compile-check")
    if check != f_result_doc["post_compile_check"]:
        raise CollectError("post_compile_check != CACHED_POST_COMPILE_CHECK")
    if type(check) is not dict or set(check) != {
            "host_outputs_verified_unchanged",
            "build_script_run_units_verified_not_reexecuted",
            "fresh_mori_backend_executables"}:
        raise CollectError("CACHED_POST_COMPILE_CHECK: exact fields required")
    fresh = check["fresh_mori_backend_executables"]
    if type(fresh) is not list or not fresh:
        raise CollectError("no fresh mori_backend executable recorded")
    outputs = load("f-outputs")
    if type(outputs) is not dict or set(outputs) != {
            "all_outputs", "host_outputs", "host_fingerprints",
            "build_script_outputs", "acceptance"}:
        raise CollectError("F_QUARANTINED_OUTPUTS: exact fields required")
    if outputs["acceptance"] != ("UNACCEPTED: independent result review "
                                  "required; no artifact executed"):
        raise CollectError("F outputs acceptance must remain UNACCEPTED")
    elf = _file_ref("backend-elf", raw_by_role["backend-elf"])
    matches = [f for f in fresh if type(f) is dict
               and set(f) == {"relative", "sha256", "bytes"}
               and f["sha256"] == elf["sha256"]
               and f["bytes"] == elf["bytes"]
               and outputs["all_outputs"].get(f["relative"]) == elf["sha256"]]
    if len(matches) != 1:
        raise CollectError("backend ELF is not the unique fresh F output")
    if elf["bytes"] == 0 or not elf["path"].endswith(matches[0]["relative"]):
        raise CollectError("backend ELF path/emptiness mismatch")
    return {"built": True, "test_bodies_executed": False}


def collect_cached_current_f(task, raw_by_role, returncode, attempt_dir):
    terminal, exit_code, signal = _terminal_fields(returncode)
    cleanup = _check_cleanup_receipt(task, raw_by_role)
    if terminal != "succeeded":
        raise CollectError(f"guard terminal exit {returncode}")
    payload = decode_cached_current_f(raw_by_role)
    artifacts = [_observation_ref(task, payload, attempt_dir),
                 _file_ref("stdout", raw_by_role["stdout"]),
                 _file_ref("stderr", raw_by_role["stderr"]),
                 _file_ref("resource-receipt",
                           raw_by_role["resource-receipt"]),
                 _file_ref("backend-elf", raw_by_role["backend-elf"])]
    return _result_dict(task, terminal, exit_code, signal, cleanup,
                        "not-applicable", artifacts)


def decode_handkey_stdout(raw: bytes) -> dict:
    """Exact native handkey stdout: {verifies: bool, verify_s: number}."""
    try:
        value = json.loads(raw.decode(), object_pairs_hook=_pairs,
                           parse_constant=_constant, parse_float=_float)
    except Exception as exc:
        raise CollectError(f"handkey stdout: invalid JSON: {exc}") from exc
    if type(value) is not dict or set(value) != {"verifies", "verify_s"}:
        raise CollectError("handkey stdout: exactly verifies/verify_s")
    if type(value["verifies"]) is not bool:
        raise CollectError("handkey verifies: exact boolean required")
    duration = value["verify_s"]
    if type(duration) not in (int, float) or isinstance(duration, bool) \
            or not 0 <= duration <= 2**63 - 1 or not math.isfinite(duration):
        raise CollectError("handkey verify_s: finite nonnegative required")
    return value


def collect_handkey(task, raw_by_role, returncode, attempt_dir):
    terminal, exit_code, signal = _terminal_fields(returncode)
    cleanup = _check_cleanup_receipt(task, raw_by_role)
    if terminal != "succeeded":
        raise CollectError(f"handkey terminal exit {returncode}")
    verdict = decode_handkey_stdout(_read_raw(raw_by_role["stdout"],
                                               "stdout"))
    pins = {}
    for role in ("key", "proof", "statement"):
        ref = _file_ref(role, raw_by_role[role])
        if ref["sha256"] != task["inputs"][role + "_sha256"]:
            raise CollectError(f"{role}: selected pin mismatch")
        pins[role + "_sha256"] = ref["sha256"]
    # Mechanical derivation only: observation carries the decoded stdout
    # verdict (which may be False -> consumer predicate-failed) plus the
    # actually hashed selected tuple. No separate author truth source.
    payload = {"verifies": verdict["verifies"], **pins}
    artifacts = [_observation_ref(task, payload, attempt_dir),
                 _file_ref("stdout", raw_by_role["stdout"]),
                 _file_ref("stderr", raw_by_role["stderr"]),
                 _file_ref("resource-receipt",
                           raw_by_role["resource-receipt"]),
                 _file_ref("key", raw_by_role["key"]),
                 _file_ref("proof", raw_by_role["proof"]),
                 _file_ref("statement", raw_by_role["statement"])]
    return _result_dict(task, "succeeded" if verdict["verifies"] else "failed",
                        exit_code, signal, cleanup, "not-applicable",
                        artifacts)


def collect_settle(task, raw_by_role, returncode, attempt_dir):
    terminal, exit_code, signal = _terminal_fields(returncode)
    cleanup = _check_cleanup_receipt(task, raw_by_role)
    if terminal != "succeeded":
        raise CollectError(f"settle harness terminal exit {returncode}")
    post_ref = _file_ref("post-state", raw_by_role["post-state"])
    raw = _read_raw(raw_by_role["source-result"], "source-result")
    try:
        decoded = settle_observation.decode_sherlock_settle_result(
            raw, task["inputs"], post_state_sha256=post_ref["sha256"])
    except settle_observation.SettleDecodeError as exc:
        raise CollectError(str(exc)) from exc
    payload = {k: decoded[k] for k in
               ("accepted", "settled", "phase", "post_state_sha256",
                "network", "anchor", "deployment_sha256", "binding_sha256",
                "batch", "expected_root")}
    artifacts = [_observation_ref(task, payload, attempt_dir),
                 _file_ref("stdout", raw_by_role["stdout"]),
                 _file_ref("stderr", raw_by_role["stderr"]),
                 _file_ref("resource-receipt",
                           raw_by_role["resource-receipt"]),
                 _file_ref("source-result", raw_by_role["source-result"]),
                 post_ref]
    return _result_dict(task, terminal, exit_code, signal, cleanup,
                        "included", artifacts)


COLLECTORS = {"collect_cached_current_f": collect_cached_current_f,
              "collect_handkey": collect_handkey,
              "collect_settle": collect_settle}


def collect_result(task, raw_outputs, returncode, attempt_dir: Path,
                   raw_receipt_ref: dict):
    """Run the fixed collector; write consumer Result + pinned sidecar."""
    spec = ADAPTERS[task["adapter"]]
    if spec["collector"] is None:
        raise CollectError(f"no collector for {task['adapter']}")
    raw_by_role = {o["role"]: o["path"] for o in raw_outputs}
    if set(raw_by_role) != set(spec["rawOutputRoles"]):
        raise CollectError("raw output membership != fixed collector roles")
    result = COLLECTORS[spec["collector"]](task, raw_by_role, returncode,
                                           attempt_dir)
    result_ref = _write_json_atomic(attempt_dir / "consumer-result.json",
                                    result)
    sidecar = {"schema": "doctorwatson.collector-sidecar/1",
               "task": task["id"], "attempt": task["attempt"],
               "fingerprint": task["fingerprint"],
               "adapter": task["adapter"], "collector": spec["collector"],
               "rawReceipt": raw_receipt_ref,
               "rawOutputs": [_file_ref(o["role"], o["path"])
                              for o in raw_outputs],
               "consumerResult": result_ref}
    _write_json_atomic(attempt_dir / "collector.json", sidecar)
    return result


def _default_runner(argv: list[str]) -> int:
    """The single subprocess site. No shell, no added timeout wrapper:
    process/resource limits belong to the selected existing guard."""
    completed = subprocess.run(argv, shell=False, check=False)
    return completed.returncode


def run_selected_once(task_id: str, plan_path: str, selection_path: str,
                      attempt_dir: str,
                      runner: Callable[[list[str]], int] | None = None,
                      history_path: str | None = None,
                      reconciliations_path: str | None = None
                      ) -> tuple[int, dict]:
    """Dispatch one selected operation at most once.

    PREEXEC (consumer fingerprint, selection hash, verified guard/config
    pins, resolved argv, declared raw outputs, and the pinned
    readinessIdentity -- exact raw plan/history/reconciliations digests
    from the common load_readiness path) is fsynced before the single
    dispatch, and that identity is rechecked at actual use immediately
    before the runner call. The raw execution receipt is preserved as
    doctorwatson.execution-result/3 with consumer terminal vocabulary;
    the fixed collector then produces the importable consumer Result.
    Return values: (bridge exit, raw receipt); the consumer Result, when
    produced, is at attempt/consumer-result.json and its digest is in
    the raw receipt. A missing receipt after possible dispatch is
    unknown-reconcile (local-process scope for compile), never retried.
    """
    if runner is None:
        runner = _default_runner
    if reconciliations_path is not None and history_path is None:
        raise BadInput("reconciliations require the raw history they "
                       "reference (--history)")
    task = resolve_task(plan_path, task_id, history_path,
                        reconciliations_path)
    selection = resolve_selection(selection_path, task)
    task, selection = _recheck_at_use(task, selection, plan_path,
                                      selection_path, history_path,
                                      reconciliations_path)
    argv = build_argv(task, selection)
    attempt = Path(attempt_dir)
    try:
        attempt.mkdir(parents=False, exist_ok=False)
    except FileExistsError as exc:
        raise Blocked("attempt directory already exists (consumed/stale/"
                      "unknown); no retry, zero dispatch") from exc
    except OSError as exc:
        raise BadInput(f"attempt directory unusable: {exc}") from exc
    preexec = {
        "schema": "doctorwatson.preexec/3", "task": task["id"],
        "adapter": task["adapter"], "attempt": task["attempt"],
        "fingerprint": task["fingerprint"],
        "selectionSha256": selection["selectionSha256"],
        "guard": selection["guard"], "config": selection["config"],
        "outerSequence": selection["outerSequence"], "argv": argv,
        "declaredOutputs": selection["outputs"],
        "readinessIdentity": task["readiness_identity"],
    }
    preexec_ref = _write_json_atomic(attempt / "PREEXEC.json", preexec)
    base = {**preexec, "schema": "doctorwatson.execution-result/3",
            "preexec": preexec_ref}
    try:
        task, selection = _recheck_at_use(task, selection, plan_path,
                                          selection_path, history_path,
                                          reconciliations_path)
        if build_argv(task, selection) != argv:
            raise Blocked("resolved argv changed at actual-use recheck")
    except (BadInput, Blocked) as exc:
        _write_json_atomic(attempt / "result-blocked.json",
                           {**base, "terminal": "refused",
                            "exit_code": None, "signal": None,
                            "unknownScope": None,
                            "collectorDetail": f"actual-use recheck: {exc}",
                            "consumerResult": None})
        raise
    try:
        returncode = runner(argv)
    except Exception as exc:  # includes timeout-like runner failures
        record = {**base, "terminal": "unknown-submission",
                  "exit_code": None, "signal": None,
                  "unknownScope": "local-process",
                  "collectorDetail": f"runner raised: {exc}",
                  "consumerResult": None}
        _write_json_atomic(attempt / "result.json", record)
        return EXIT_UNKNOWN, record
    if type(returncode) is not int or isinstance(returncode, bool):
        record = {**base, "terminal": "unknown-submission",
                  "exit_code": None, "signal": None,
                  "unknownScope": "local-process",
                  "collectorDetail": "runner returned non-integer terminal",
                  "consumerResult": None}
        _write_json_atomic(attempt / "result.json", record)
        return EXIT_UNKNOWN, record
    terminal, exit_code, signal = _terminal_fields(returncode)
    raw_record = {**base, "terminal": terminal, "exit_code": exit_code,
                  "signal": signal, "unknownScope": None,
                  "collectorDetail": None, "consumerResult": None}
    raw_ref = _write_json_atomic(attempt / "result.json", raw_record)
    try:
        result = collect_result(task, selection["outputs"], returncode,
                                attempt, raw_ref)
    except CollectError as exc:
        raw_record = {**raw_record, "terminal": "failed",
                      "collectorDetail": str(exc)}
        _write_json_atomic(attempt / "result-collect-failed.json",
                           raw_record)
        return EXIT_FAILED, raw_record
    raw_record = {**raw_record, "terminal": result["terminal"],
                  "consumerResult": _file_ref(
                      "consumer-result",
                      str(attempt / "consumer-result.json"))}
    _write_json_atomic(attempt / "result-final.json", raw_record)
    return (EXIT_OK if result["terminal"] == "succeeded" else EXIT_FAILED), \
        raw_record


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    usage = ("usage: run_selected.py --execute TASK --plan PATH "
             "--selection PATH --attempt DIR [--history PATH] "
             "[--reconciliations PATH]")
    try:
        if len(argv) < 8 or argv[0] != "--execute" or argv[2] != "--plan" \
                or argv[4] != "--selection" or argv[6] != "--attempt":
            print(usage, file=sys.stderr)
            return EXIT_BAD_INPUT
        task_id, plan_path, selection_path, attempt_dir = \
            argv[1], argv[3], argv[5], argv[7]
        history = None
        reconciliations = None
        rest = argv[8:]
        index = 0
        while index < len(rest):
            token = rest[index]
            name, eq, attached = token.partition("=")
            if name not in ("--history", "--reconciliations"):
                print(usage, file=sys.stderr)
                return EXIT_BAD_INPUT
            if eq:
                value = attached
                index += 1
            else:
                if index + 1 >= len(rest):
                    print(usage, file=sys.stderr)
                    return EXIT_BAD_INPUT
                value = rest[index + 1]
                index += 2
            if not value:
                print(usage, file=sys.stderr)
                return EXIT_BAD_INPUT
            if name == "--history":
                if history is not None:
                    print(usage, file=sys.stderr)
                    return EXIT_BAD_INPUT
                history = value
            else:
                if reconciliations is not None:
                    print(usage, file=sys.stderr)
                    return EXIT_BAD_INPUT
                reconciliations = value
        code, _record = run_selected_once(
            task_id, plan_path, selection_path, attempt_dir,
            history_path=history, reconciliations_path=reconciliations)
        return code
    except BadInput as exc:
        print(f"run_selected: bad input: {exc}", file=sys.stderr)
        return EXIT_BAD_INPUT
    except Blocked as exc:
        print(f"run_selected: blocked (zero dispatch): {exc}",
              file=sys.stderr)
        return EXIT_BLOCKED


if __name__ == "__main__":
    raise SystemExit(main())
