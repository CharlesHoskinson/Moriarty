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

The SDK-build adapter dispatches ONE code-fixed provisioned-host outer
sequence (selection/2, ``bounded-sdk-compile-cached-current-f/2``): OP0..OP6
are external host-provisioning prerequisites already completed prior to
config.final.json issuance (so execution_root, config_dir, and Q-OBS already
exist on the provisioned host and op0..op6 are never staged or dispatched
here). Entry starts at the original operator ``prelaunch-verify`` (verifying
live mounts, cgroups, and tools against Q-OBS and the operator request), raw
E/S/P ``counters-before``, the native guard child placed alone in the
supervisor leaf S by the exact original sudo/sh/setpriv template (this
factory/controller process never enters S; the guard's
S/cgroup.procs == [guard PID] predicate is unchanged), ``counters-after``
before any removal, durable unexecuted preservation of the unique fresh
backend ELF resolved from the authenticated packet (never a guessed
historical suffix), the original operator ``collect-evidence``, then
``teardown`` (or ``abort-prelaunch-cleanup`` only when native was dispatched
and ONE_SHOT_STARTED is still absent). Any prelaunch refusal before
native/factory host mutation (e.g., prelaunch-verify or counters-before
failing) consumes the attempt and retains the raw failure receipt WITHOUT
running abort-prelaunch-cleanup or teardown against the preexisting
provisioned host. A typed cleanup-observation receipt is issued from those
actual stage outcomes and a live restoration probe of both the cgroup
envelope (E/S/P) and all 3 request mountpoints in /proc/self/mountinfo --
never a synthesized complete -- and only then does the fixed collector run
over the retained outputs. The direct ``[python, guard, --cached-current-f]``
argv of selection/1 is retired: outside S it can never satisfy the guard, so
it is never selected.

The handkey-selection/1 branch dispatches only the fixed public verifier tuple
through a dedicated forked ordinary supervisor. PR_SET_CHILD_SUBREAPER must
succeed before workload launch; actual wait4 reaping to ECHILD and observed
group absence establish cleanup. Fixed per-process limits do not establish
aggregate native resource admission. Execution requires an explicit complete
history file and durably reserves attempts beside it, retaining originals
before a separately selected changed attempt. This source wiring grants no
native execution permission.

After the outer sequence, fixed source-specific collectors convert the
actual raw producer files into the consumer Result shape exactly
(``delivery.load_result`` fields). The original PREEXEC and raw execution
receipt are preserved in the attempt directory and pinned by digest in a
collector sidecar; they are never renamed into a Result. Collectors:

- ``moriarty-sdk-build``: selected existing guard
  ``bounded_sdk_compile.py --cached-current-f`` packet contract
  (F_CACHED_CURRENT_RESULT / F_QUARANTINED_OUTPUTS /
  CACHED_POST_COMPILE_CHECK / F.stdout / F.stderr / fresh backend ELF),
  current helper shape including supervisor_self_observation and, on
  the adopted-policy route, its five policy fields. Exit 0 plus present
  files is not enough; see decode_cached_current_f.
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

import copy
import ctypes
import fcntl
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import resource
import selectors
import signal
import stat
import subprocess
import sys
import time
from typing import Any, Callable

from capabilities import adapter_capability

import delivery  # sibling ordinary consumer in models/doctorwatson

import reconciliation
import settle_observation

MAX_JSON_BYTES = 4 * 1024 * 1024
MAX_ELF_BYTES = 1024 * 1024 * 1024
HEX = re.compile(r"[0-9a-f]{64}\Z")
HEX40 = re.compile(r"[0-9a-f]{40}\Z")
NAME = re.compile(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,127}\Z")
# Paths interpolated into the fixed /bin/sh -c S-placement script must be
# incapable of breaking out of their single word: no spaces, quotes,
# shell metacharacters or empty/dot segments (checked per segment too).
SAFE_PATH = re.compile(r"/[A-Za-z0-9._@/-]+\Z")
CG_ENVELOPE = re.compile(r"/[A-Za-z0-9_][A-Za-z0-9_.@-]{0,127}\Z")
CG_NAME = re.compile(r"[a-z][a-z0-9-]{0,62}\Z")
SUPERVISOR_UID = 1000
OUTER_SEQUENCE = "bounded-sdk-compile-cached-current-f/2"
SELECTION_SCHEMA = "doctorwatson.execution-selection/2"
FORBIDDEN_PROVISION_SUBCOMMANDS: frozenset[str] = frozenset(
    {"op0", "op1", "op2", "op3", "op4", "op5", "op6"})
PROVISION_STEPS: tuple[tuple[str, bool], ...] = (
    ("prelaunch-verify", False),
)
PACKET_FILES = {"f-result": "F_CACHED_CURRENT_RESULT.json",
                "f-outputs": "F_QUARANTINED_OUTPUTS.json",
                "post-compile-check": "CACHED_POST_COMPILE_CHECK.json",
                "stdout": "F.stdout", "stderr": "F.stderr"}

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
        "outerSequences": (OUTER_SEQUENCE,),
        "rawOutputRoles": ("f-result", "f-outputs", "post-compile-check",
                           "stdout", "stderr", "backend-elf",
                           "resource-receipt"),
        "collector": "collect_cached_current_f",
    },
    "handkey-verify-pis": {
        "launchable": True,
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


class StageUnknown(RuntimeError):
    """A stage was dispatched but its termination is not proven.

    Includes runner timeout and a capture() return whose actual_exit is
    null or whose controller_failure is set: a capture return alone
    never proves termination, so it is never promoted to a terminal.
    """


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
                "readiness_identity": readiness["identity"],
                "history_results": readiness["results"]}
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
            or fresh_selection != selection):
        raise Blocked("readiness/selection identity changed before "
                      "dispatch; pinned at resolve, rechecked at use")
    return fresh_task, fresh_selection


def _abs_norm(value, at: str) -> str:
    if (type(value) is not str or not Path(value).is_absolute()
            or str(Path(os.path.normpath(value))) != value):
        raise BadInput(f"{at}: normalized absolute path required")
    return value


def _safe_word_path(value, at: str) -> str:
    """Absolute path safe as one word in the fixed sh -c script."""
    _abs_norm(value, at)
    if not SAFE_PATH.fullmatch(value) or any(
            seg in ("", ".", "..") for seg in value.split("/")[1:]):
        raise BadInput(f"{at}: path not safe for fixed outer script")
    return value


def _bounded_int(value, at: str, lo: int, hi: int) -> int:
    if type(value) is not int or isinstance(value, bool) \
            or not lo <= value <= hi:
        raise BadInput(f"{at}: integer in [{lo},{hi}] required")
    return value


def _guard_constants(guard_path: str) -> dict:
    """Extract BASE/LOCK/helper/policy/floor/prior pins from the pinned guard.

    The bytes were already digest-verified against the selection pin,
    so constants read here are the selected guard's own, never a
    selection-supplied paraphrase.
    """
    try:
        text = _read_raw(guard_path, "selection.guard").decode("utf-8")
    except (CollectError, UnicodeError) as exc:
        raise Blocked(f"selection.guard: cannot read pinned guard: {exc}") \
            from exc

    def grab(name, pattern, required):
        m = re.search(rf"^{name}\s*=\s*'({pattern})'", text, re.M)
        if m is None:
            if required:
                raise Blocked(f"selection.guard: {name} constant absent "
                              "from pinned guard bytes")
            return None
        return m.group(1)

    def grab_int(name):
        m = re.search(rf"^{name}\s*=\s*([0-9]+)\b", text, re.M)
        return int(m.group(1)) if m is not None else None

    def grab_int_triple(name):
        m = re.search(
            rf"^{name}\s*=\s*\(\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*,?\s*\)",
            text, re.M)
        if m is None:
            return None
        return (int(m.group(1)), int(m.group(2)), int(m.group(3)))

    return {
        "baseHead": grab("BASE", r"[0-9a-f]{40}", True),
        "cargoLockSha256": grab("LOCK", r"[0-9a-f]{64}", True),
        "cachedCurrentBuildSha256": grab(
            "CACHED_CURRENT_BUILD_SHA256", r"[0-9a-f]{64}", True),
        "policyCandidateSha256": grab(
            "CANDIDATE_DISPOSITION_SHA256", r"[0-9a-f]{64}", False),
        "expectedPriorEspUs": grab_int_triple("EXPECTED_PRIOR_ESP_US"),
        "expectedPriorResidualUs": grab_int("EXPECTED_PRIOR_RESIDUAL_US"),
        "expectedNative21DeltasEspUs": grab_int_triple(
            "EXPECTED_NATIVE21_DELTAS_ESP_US"),
        "expectedReserveAbUs": grab_int("EXPECTED_RESERVE_AB_US"),
        "expectedFloorsS": grab_int_triple("EXPECTED_FLOORS_S"),
        "policyHoldbackS": grab_int("POLICY_HOLDBACK_S"),
        "policySupervisorAllowanceS": grab_int(
            "POLICY_SUPERVISOR_ALLOWANCE_S"),
        "policyTotalWallMaxS": grab_int("POLICY_TOTAL_WALL_MAX_S"),
        "policyFWallMaxS": grab_int("POLICY_F_WALL_MAX_S"),
    }


def _helper_constants(helper_path: str) -> dict:
    """Extract PRESERVED_PACKET_PINS from the digest-verified helper."""
    try:
        text = _read_raw(
            helper_path, "selection.outer.helper").decode("utf-8")
    except (CollectError, UnicodeError) as exc:
        raise Blocked(
            f"selection.outer.helper: cannot read pinned helper: {exc}"
        ) from exc
    m_block = re.search(
        r"^PRESERVED_PACKET_PINS\s*=\s*\{(?P<body>[^}]+)\}", text, re.M)
    if m_block is None:
        raise Blocked("selection.outer.helper: PRESERVED_PACKET_PINS "
                      "absent from pinned helper bytes")
    body = m_block.group("body")

    def grab_pin(fname: str) -> str:
        m = re.search(
            rf"['\"]{re.escape(fname)}['\"]\s*:\s*['\"]([0-9a-f]{{64}})['\"]",
            body)
        if m is None:
            raise Blocked("selection.outer.helper: PRESERVED_PACKET_PINS["
                          f"{fname!r}] absent from pinned helper bytes")
        return m.group(1)

    return {
        "preservedDFreezeSha256": grab_pin("D_FREEZE.json"),
        "preservedEResolvedSha256": grab_pin("E_RESOLVED.json"),
    }


def _config_facts(config_ref: dict, guard_ref: dict) -> dict:
    """Typed facts the outer sequence/decoder need from the pinned config."""
    cfg = _load_json_file(config_ref["path"], "selection.config content")
    if type(cfg) is not dict:
        raise BadInput("config: object required")
    cg = cfg.get("cgroup")
    if type(cg) is not dict:
        raise BadInput("config.cgroup: object required")
    envelope = cg.get("envelope_path")
    if type(envelope) is not str or not CG_ENVELOPE.fullmatch(envelope):
        raise BadInput("config.cgroup.envelope_path: invalid")
    leaf = cg.get("supervisor_leaf_name")
    phases = cg.get("phase_parent_name")
    for v, at in ((leaf, "supervisor_leaf_name"),
                  (phases, "phase_parent_name")):
        if type(v) is not str or not CG_NAME.fullmatch(v):
            raise BadInput(f"config.cgroup.{at}: invalid")
    if leaf == phases:
        raise BadInput("config.cgroup: leaf/phase names must differ")
    paths = cfg.get("paths")
    if type(paths) is not dict:
        raise BadInput("config.paths: object required")
    cpaths = {k: _safe_word_path(paths.get(k), f"config.paths.{k}")
              for k in ("execution_root", "packet", "target", "tmp",
                        "source_home", "empty_home", "mask_empty",
                        "worktree", "config")}
    if cpaths["config"] != config_ref["path"]:
        raise Blocked("config.paths.config != selected config path")
    runner = (cfg.get("bindings") or {}).get("runner") \
        if type(cfg.get("bindings")) is dict else None
    if type(runner) is not dict or runner.get("path") != guard_ref["path"] \
            or runner.get("sha256") != guard_ref["sha256"]:
        raise Blocked("config.bindings.runner != selected guard pin")
    limits_f = (cfg.get("limits") or {}).get("F") \
        if type(cfg.get("limits")) is dict else None
    if type(limits_f) is not dict:
        raise BadInput("config.limits.F: object required")
    total_wall_s = _bounded_int(
        cfg.get("total_wall_seconds"), "total_wall_seconds", 1, 100000)
    sup_cpu_s = _bounded_int(
        cfg.get("supervisor_cpu_seconds"), "supervisor_cpu_seconds",
        1, 10 ** 9)
    f_wall_s = _bounded_int(
        limits_f.get("wall_seconds"), "limits.F.wall_seconds", 1, 100000)
    rd = cfg.get("resource_disposition")
    if type(rd) is not dict:
        raise BadInput("config.resource_disposition: object required")
    prior_e = _bounded_int(
        rd.get("adopted_prior_E_us"), "adopted_prior_E_us", 0, 10 ** 18)
    prior_s = _bounded_int(
        rd.get("adopted_prior_S_us"), "adopted_prior_S_us", 0, 10 ** 18)
    prior_p = _bounded_int(
        rd.get("adopted_prior_P_us"), "adopted_prior_P_us", 0, 10 ** 18)
    prior_res = _bounded_int(
        rd.get("adopted_prior_residual_us"), "adopted_prior_residual_us",
        0, 10 ** 18)
    delta_e = _bounded_int(
        rd.get("native21_delta_E_us"), "native21_delta_E_us", 0, 10 ** 18)
    delta_s = _bounded_int(
        rd.get("native21_delta_S_us"), "native21_delta_S_us", 0, 10 ** 18)
    delta_p = _bounded_int(
        rd.get("native21_delta_P_us"), "native21_delta_P_us", 0, 10 ** 18)
    reserve_ab = _bounded_int(
        rd.get("reserve_ab_us"), "reserve_ab_us", 1, 10 ** 18)
    sup_floor_s = _bounded_int(
        rd.get("supervisor_floor_before_native21_s"),
        "supervisor_floor_before_native21_s", 1, 10 ** 9)
    cum_e = prior_e + delta_e
    cum_s = prior_s + delta_s
    cum_p = prior_p + delta_p
    if cum_e - cum_s - cum_p != prior_res + (delta_e - delta_s - delta_p):
        raise Blocked(
            "config.resource_disposition: residual arithmetic mismatch")
    status = rd.get("wrapper_c_status")
    if status == "CARRIED_UNKNOWN_POLICY_SPEND":
        if rd.get("wrapper_c_us") is not None:
            raise Blocked("config policy route: wrapper_c_us must be "
                          "null (c/exact-S unknown, never numeric)")
        adoption = rd.get("policy_adoption")
        if type(adoption) is not dict:
            raise BadInput("config.policy_adoption: object required")
        holdback_s = _bounded_int(
            rd.get("policy_holdback_s"), "policy_holdback_s", 0, 10 ** 9)
        allowance_s = _bounded_int(
            rd.get("policy_supervisor_allowance_s"),
            "policy_supervisor_allowance_s", 1, 10 ** 9)
        rem_total_max_s = _bounded_int(
            rd.get("remaining_total_wall_seconds_max"),
            "remaining_total_wall_seconds_max", 1, 100000)
        rem_f_max_s = _bounded_int(
            rd.get("remaining_f_wall_seconds_max"),
            "remaining_f_wall_seconds_max", 1, 100000)
        charged_s_us = delta_s + reserve_ab
        upper_bound_s = sup_floor_s - (-(-charged_s_us // 1_000_000))
        if upper_bound_s - holdback_s != allowance_s:
            raise Blocked("config.resource_disposition: policy supervisor "
                          "allowance != upper_bound_s - policy_holdback_s")
        if (total_wall_s > rem_total_max_s
                or f_wall_s > rem_f_max_s
                or sup_cpu_s > allowance_s):
            raise Blocked("config limits exceed policy remaining wall "
                          "maxima or supervisor allowance")
        derived_disp = {
            "disposition_kind": "CARRIED_UNKNOWN_POLICY_SPEND",
            "cumulative_before_wrapper_E_us": cum_e,
            "cumulative_before_wrapper_S_us": cum_s,
            "cumulative_before_wrapper_P_us": cum_p,
            "effective_prior_E_us": cum_e + reserve_ab,
            "effective_prior_S_us": cum_s + reserve_ab,
            "effective_prior_P_us": cum_p,
            "wrapper_c_us": None,
            "exact_remaining_supervisor_s": None,
            "policy_holdback_s": holdback_s,
            "policy_supervisor_allowance_s": allowance_s,
            "policy_remaining_total_wall_max_s": rem_total_max_s,
            "policy_remaining_f_wall_max_s": rem_f_max_s,
        }
        route = {
            "kind": "policy",
            "policyAdoptionSha256": _sha(adoption.get("sha256"),
                                         "policy_adoption.sha256"),
            "policyHoldbackS": holdback_s,
            "policyAllowanceS": allowance_s,
            "policyRemainingTotalWallMaxS": rem_total_max_s,
            "policyRemainingFWallMaxS": rem_f_max_s,
            "priorEspUs": (prior_e, prior_s, prior_p),
            "priorResidualUs": prior_res,
            "native21DeltasEspUs": (delta_e, delta_s, delta_p),
            "reserveAbUs": reserve_ab,
            "supervisorFloorBeforeNative21S": sup_floor_s,
            "derivedCurrentDisposition": derived_disp,
        }
    elif status == "ESTABLISHED_PINNED":
        wrapper_c = _bounded_int(
            rd.get("wrapper_c_us"), "wrapper_c_us", 0, 10 ** 15)
        rem_total_max_s = _bounded_int(
            rd.get("remaining_total_wall_seconds_max"),
            "remaining_total_wall_seconds_max", 1, 100000)
        rem_f_max_s = _bounded_int(
            rd.get("remaining_f_wall_seconds_max"),
            "remaining_f_wall_seconds_max", 1, 100000)
        charged_s_us = delta_s + reserve_ab + wrapper_c
        rem_sup_s = sup_floor_s - (-(-charged_s_us // 1_000_000))
        if (rem_sup_s <= 0 or total_wall_s > rem_total_max_s
                or f_wall_s > rem_f_max_s or sup_cpu_s > rem_sup_s):
            raise Blocked("config limits exceed established remaining "
                          "wall or supervisor balance")
        derived_disp = {
            "cumulative_before_wrapper_E_us": cum_e,
            "cumulative_before_wrapper_S_us": cum_s,
            "cumulative_before_wrapper_P_us": cum_p,
            "effective_prior_E_us": cum_e + reserve_ab + wrapper_c,
            "effective_prior_S_us": cum_s + reserve_ab + wrapper_c,
            "effective_prior_P_us": cum_p,
            "remaining_supervisor_cpu_seconds": rem_sup_s,
            "exact_remaining_supervisor_s": rem_sup_s,
            "supervisor_spend_ceiling_s": rem_sup_s,
            "disposition_kind": "ESTABLISHED_PINNED",
        }
        route = {
            "kind": "established",
            "wrapperCUs": wrapper_c,
            "priorEspUs": (prior_e, prior_s, prior_p),
            "priorResidualUs": prior_res,
            "native21DeltasEspUs": (delta_e, delta_s, delta_p),
            "reserveAbUs": reserve_ab,
            "supervisorFloorBeforeNative21S": sup_floor_s,
            "derivedCurrentDisposition": derived_disp,
        }
    else:
        raise Blocked(f"config wrapper_c_status {status!r}: no spendable "
                      "route; zero dispatch")
    return {"envelopePath": envelope, "supervisorLeaf": leaf,
            "phaseParent": phases, "paths": cpaths, "route": route,
            "totalWallSeconds": total_wall_s,
            "supervisorCpuSeconds": sup_cpu_s,
            "fWallSeconds": f_wall_s}


def _request_facts(request_ref: dict, cfg_facts: dict) -> dict:
    """Typed facts from the pinned original operator request."""
    req = _load_json_file(request_ref["path"],
                           "selection.outer.operatorRequest content")
    if type(req) is not dict:
        raise BadInput("operatorRequest: object required")
    if req.get("schema_version") != 1 or req.get("operator_uid") != 1000 \
            or req.get("operator_gid") != 1000:
        raise BadInput("operatorRequest: schema_version/operator ids")
    cg = req.get("cgroup")
    if type(cg) is not dict or cg.get("envelope_path") \
            != cfg_facts["envelopePath"] \
            or cg.get("supervisor_leaf_name") != cfg_facts["supervisorLeaf"] \
            or cg.get("phase_parent_name") != cfg_facts["phaseParent"]:
        raise Blocked("operatorRequest.cgroup != config cgroup")
    paths = req.get("paths")
    if type(paths) is not dict:
        raise BadInput("operatorRequest.paths: object required")
    rpaths = {k: _abs_norm(paths.get(k), f"operatorRequest.paths.{k}")
              for k in ("execution_root", "packet", "target", "tmp",
                        "source_home", "empty_home", "mask_empty",
                        "config_dir", "evidence_dir",
                        "observation_output")}
    for k in ("execution_root", "packet", "target", "tmp",
              "source_home", "empty_home", "mask_empty"):
        if rpaths[k] != cfg_facts["paths"][k]:
            raise Blocked(f"operatorRequest.paths.{k} != config path")
    if str(Path(cfg_facts["paths"]["config"]).parent) \
            != rpaths["config_dir"]:
        raise Blocked("config parent != operatorRequest config_dir")
    if str(Path(rpaths["observation_output"]).parent) \
            != rpaths["config_dir"]:
        raise Blocked("observation_output outside config_dir")
    return {"paths": rpaths}


def _derived_outputs(req_facts: dict, retained_dir: str) -> list[dict]:
    evidence = Path(req_facts["paths"]["evidence_dir"])
    out = [{"role": role, "path": str(evidence / "packet" / name)}
           for role, name in PACKET_FILES.items()]
    out.append({"role": "backend-elf",
                "path": str(Path(retained_dir) / "backend-elf")})
    out.append({"role": "resource-receipt",
                "path": str(Path(retained_dir) / "cleanup.json")})
    return out


def _require_real_parent_dir(path_str: str, label: str) -> None:
    """Require path_str's parent to exist as a real non-symlink directory."""
    parent = str(Path(path_str).parent)
    try:
        st = os.lstat(parent)
    except OSError as exc:
        raise Blocked(
            f"{label} parent directory absent at entry: {exc}") from exc
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISDIR(st.st_mode):
        raise Blocked(
            f"{label} parent must be an existing real non-symlink directory")


def resolve_selection(selection_path: str, task: dict) -> dict:
    """Validate the external Root selection against the resolved task.

    Selection/2 pins the guard, its helper, the config, the original
    operator + request, the bounded capture module and the /usr/bin/time
    tool Root wraps around this controller, plus the retained-output
    directory. Every one of the 7 pinned file refs (guard, config,
    outer.operator, outer.operatorRequest, outer.capture, outer.helper,
    outer.timeTool) must be bound in the plan task's identity.input_files
    so delivery.fingerprint(plan, task_id) cryptographically covers all
    outer pins. Moreover, outer.helper must be the guard's sibling
    cached_current_build.py and match the CACHED_CURRENT_BUILD_SHA256
    constant extracted from the digest-verified guard bytes.
    """
    sel = _load_json_file(selection_path, "selection")
    if type(sel) is dict and sel.get("schema") == HANDKEY_SELECTION_SCHEMA:
        return _resolve_handkey_selection(sel, task)
    if task["adapter"] == "handkey-verify-pis":
        raise BadInput("handkey adapter requires strict handkey-selection/1")
    keys = {"schema", "task", "adapter", "fingerprint", "outerSequence",
            "guard", "config", "outer", "outputs"}
    if type(sel) is not dict or set(sel) != keys:
        raise BadInput(f"selection: expected exactly {sorted(keys)}")
    if sel["schema"] != SELECTION_SCHEMA:
        raise BadInput("selection.schema: unsupported (selection/1 direct "
                       "guard dispatch is retired; never selected)")
    if sel["task"] != task["id"] or sel["adapter"] != task["adapter"]:
        raise Blocked("selection task/adapter does not match plan task")
    if sel["fingerprint"] != task["fingerprint"]:
        raise Blocked("selection fingerprint stale vs consumer fingerprint "
                      "(source/dependency/input mismatch); zero dispatch")
    spec = ADAPTERS[task["adapter"]]
    capability = adapter_capability(task["adapter"])
    if not capability["executor_available"]:
        raise Blocked(f"adapter {task['adapter']}: {capability['blocked_reason']}")
    if sel["outerSequence"] not in spec["outerSequences"]:
        raise BadInput("selection.outerSequence: not a fixed sequence "
                       f"for {task['adapter']}")
    guard = _verify_pinned(sel["guard"], "selection.guard")
    _safe_word_path(guard["path"], "selection.guard.path")
    config = _verify_pinned(sel["config"], "selection.config")
    _safe_word_path(config["path"], "selection.config.path")
    outer_raw = sel["outer"]
    outer_keys = {"operator", "operatorRequest", "capture", "helper",
                  "retainedDir", "timeOutput", "timeTool"}
    if type(outer_raw) is not dict or set(outer_raw) != outer_keys:
        raise BadInput(f"selection.outer: expected exactly "
                       f"{sorted(outer_keys)} (no argv/shell/env/hooks)")
    outer = {
        "operator": _verify_pinned(outer_raw["operator"],
                                    "selection.outer.operator"),
        "operatorRequest": _verify_pinned(
            outer_raw["operatorRequest"],
            "selection.outer.operatorRequest"),
        "capture": _verify_pinned(outer_raw["capture"],
                                   "selection.outer.capture"),
        "helper": _verify_pinned(outer_raw["helper"],
                                  "selection.outer.helper"),
        "timeTool": _verify_pinned(outer_raw["timeTool"],
                                    "selection.outer.timeTool"),
        "retainedDir": _abs_norm(outer_raw["retainedDir"],
                                  "selection.outer.retainedDir"),
        "timeOutput": _abs_norm(outer_raw["timeOutput"],
                                 "selection.outer.timeOutput"),
    }
    _require_real_parent_dir(outer["retainedDir"],
                             "selection.outer.retainedDir")
    if Path(outer["timeTool"]["path"]).name != "time":
        raise BadInput("selection.outer.timeTool: pinned GNU time tool "
                       "required (Root pin: /usr/bin/time)")
    task_obj = task.get("task_obj")
    if task_obj is None or not hasattr(task_obj, "input_files"):
        raise Blocked("task lacks resolved delivery.Task input_files")
    bound_inputs = {(ref.path, ref.bytes, ref.sha256)
                    for ref in task_obj.input_files}
    for label, pin_obj in (
        ("selection.guard", guard),
        ("selection.config", config),
        ("selection.outer.operator", outer["operator"]),
        ("selection.outer.operatorRequest", outer["operatorRequest"]),
        ("selection.outer.capture", outer["capture"]),
        ("selection.outer.helper", outer["helper"]),
        ("selection.outer.timeTool", outer["timeTool"]),
    ):
        if (pin_obj["path"], pin_obj["bytes"],
                pin_obj["sha256"]) not in bound_inputs:
            raise Blocked(f"{label} not bound in plan task.identity "
                          "(consumer fingerprint must cover all 7 pins)")
    cfg_facts = _config_facts(config, guard)
    req_facts = _request_facts(outer["operatorRequest"], cfg_facts)
    constants = _guard_constants(guard["path"])
    expected_helper_path = str(
        Path(guard["path"]).parent / "cached_current_build.py")
    if outer["helper"]["path"] != expected_helper_path:
        raise Blocked("selection.outer.helper.path != guard sibling "
                      f"cached_current_build.py ({expected_helper_path})")
    if outer["helper"]["sha256"] != constants["cachedCurrentBuildSha256"]:
        raise Blocked("selection.outer.helper.sha256 != guard "
                      "CACHED_CURRENT_BUILD_SHA256")
    helper_constants = _helper_constants(outer["helper"]["path"])
    route = cfg_facts["route"]
    floors = constants["expectedFloorsS"]
    if (constants["expectedPriorEspUs"] != route["priorEspUs"]
            or constants["expectedPriorResidualUs"]
            != route["priorResidualUs"]
            or constants["expectedNative21DeltasEspUs"]
            != route["native21DeltasEspUs"]
            or constants["expectedReserveAbUs"] != route["reserveAbUs"]
            or floors is None
            or floors[2] != route["supervisorFloorBeforeNative21S"]):
        raise Blocked("pinned guard retained-ledger constants absent or "
                      "mismatched with config.resource_disposition")
    if route["kind"] == "policy":
        if constants["policyCandidateSha256"] is None:
            raise Blocked("pinned guard has no policy candidate constant for "
                          "the adopted-policy config route")
        if (constants["policyHoldbackS"] != route["policyHoldbackS"]
                or constants["policySupervisorAllowanceS"]
                != route["policyAllowanceS"]
                or constants["policyTotalWallMaxS"]
                != route["policyRemainingTotalWallMaxS"]
                or constants["policyFWallMaxS"]
                != route["policyRemainingFWallMaxS"]
                or route["policyRemainingTotalWallMaxS"]
                != floors[0] - route["policyHoldbackS"]
                or route["policyRemainingFWallMaxS"]
                != floors[1] - route["policyHoldbackS"]):
            raise Blocked("pinned guard policy constants absent or "
                          "mismatched with config.resource_disposition")
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
    derived = _derived_outputs(req_facts, outer["retainedDir"])
    if {o["role"]: o["path"] for o in norm_outputs} \
            != {o["role"]: o["path"] for o in derived}:
        raise Blocked("selection.outputs != code-derived retained paths "
                      "(evidence packet copies + retained ELF/receipt)")
    return {"outerSequence": sel["outerSequence"], "guard": guard,
            "config": config, "outer": outer, "outputs": norm_outputs,
            "configFacts": cfg_facts, "requestFacts": req_facts,
            "guardConstants": constants,
            "helperConstants": helper_constants,
            "selectionSha256": _digest(sel)}


def _cgroup_fs(cfg_facts: dict) -> dict:
    base = "/sys/fs/cgroup" + cfg_facts["envelopePath"]
    return {"E": base, "S": base + "/" + cfg_facts["supervisorLeaf"],
            "P": base + "/" + cfg_facts["phaseParent"]}


def build_native_argv(selection: dict) -> list[str]:
    """Exact original S-placement argv (NATIVE-COMMAND template).

    Only the child shell enters S (echo $$ > S/cgroup.procs) and then
    execs setpriv to UID1000/no-caps before the guard leaf argv. This
    factory process, its parent and the sudo prefix never join S, so
    the guard observes S/cgroup.procs == [guard PID] alone. The leaf
    guard argv/caps are unchanged from selection/1.
    """
    cfg = selection["configFacts"]
    s_dir = _cgroup_fs(cfg)["S"]
    script = (
        f"echo $$ > {s_dir}/cgroup.procs\n"
        "exec /usr/bin/setpriv --reuid=1000 --regid=1000 --clear-groups "
        "--inh-caps=-all --bounding-set=-all --no-new-privs "
        "-- /usr/bin/env -i LANG=C PATH=/usr/bin:/bin "
        f"/usr/bin/python3 -I -B -S {selection['guard']['path']} "
        f"--cached-current-f --config {selection['config']['path']} "
        f"--config-sha256 {selection['config']['sha256']}")
    return ["/usr/bin/sudo", "-n", "--", "/bin/sh", "-eu", "-c", script]


def _operator_argv(selection: dict, step: str, sudo: bool) -> list[str]:
    outer = selection["outer"]
    argv = ["/usr/bin/timeout", "--signal=TERM", "--kill-after=2s", "30s"]
    if sudo:
        argv += ["/usr/bin/sudo", "-n", "--"]
    return argv + ["/usr/bin/python3", "-I", "-B", "-S",
                   outer["operator"]["path"], "--request",
                   outer["operatorRequest"]["path"], "--request-sha256",
                   outer["operatorRequest"]["sha256"], step]


def _counters_argv(selection: dict) -> list[str]:
    nodes = _cgroup_fs(selection["configFacts"])
    script = (
        "import json,pathlib,time,os\n"
        f"nodes={nodes!r}\n"
        "print(json.dumps({'monotonic_ns':time.monotonic_ns(),"
        "'observer_pid':os.getpid(),'nodes':{label:{'path':base,"
        "'raw':{name:(pathlib.Path(base)/name).read_text() for name in "
        "('cpu.stat','memory.events','pids.events','cgroup.procs',"
        "'cgroup.events')}} for label,base in nodes.items()}},"
        "sort_keys=True))\n")
    return ["/usr/bin/python3", "-I", "-B", "-S", "-c", script]


def build_outer_stages(selection: dict, attempt: Path) -> dict[str, dict]:
    """Code-fixed stage table for the one launchable outer sequence.

    Runner contract: ``runner(stage) -> int`` where stage has exactly
    stage/argv/cwd/env/wallSeconds/stdout/stderr (native additionally
    carries the pinned capture op). Nothing here is selection-supplied
    beyond the verified pins/facts resolve_selection derived.
    """
    cfg = selection["configFacts"]
    home = os.path.expanduser("~")
    if not Path(home).is_absolute():
        raise Blocked("controller HOME is not absolute; zero dispatch")
    env = {"HOME": home, "LANG": "C.UTF-8", "PATH": "/usr/bin:/bin"}
    streams = attempt / "streams"

    def stage(name, argv, cwd, wall, stdout=None):
        return {"stage": name, "argv": argv, "cwd": cwd, "env": dict(env),
                "wallSeconds": wall,
                "stdout": str(stdout) if stdout else None,
                "stderr": str(streams / (name + ".stderr"))}

    stages = {}
    for step, sudo in PROVISION_STEPS:
        stages[step] = stage(step, _operator_argv(selection, step, sudo),
                             cfg["paths"]["worktree"], 35,
                             streams / (step + ".stdout"))
    for label in ("counters-before", "counters-after"):
        stages[label] = stage(label, _counters_argv(selection), home, 10,
                              attempt / (label + ".json"))
        stages[label]["env"] = {"PATH": "/usr/bin:/bin",
                                "LANG": "C.UTF-8"}
    native_argv = build_native_argv(selection)
    capture_op = {"id": "current-linked-producer-compile-one-attempt",
                  "argv": native_argv, "cwd": cfg["paths"]["worktree"],
                  "environment": dict(env),
                  "wallSeconds": cfg["totalWallSeconds"],
                  "keptBytesPerStream": 33554432}
    native = stage("native", native_argv, cfg["paths"]["worktree"],
                   cfg["totalWallSeconds"])
    native["stdout"] = str(attempt / "native-capture" / "stdout.bin")
    native["stderr"] = str(attempt / "native-capture" / "stderr.bin")
    native["capture"] = {"module": selection["outer"]["capture"],
                         "phaseDir": str(attempt / "native-capture"),
                         "op": capture_op}
    stages["native"] = native
    stages["collect-evidence"] = stage(
        "collect-evidence",
        _operator_argv(selection, "collect-evidence", False),
        cfg["paths"]["worktree"], 35,
        streams / "collect-evidence.stdout")
    stages["teardown"] = stage(
        "teardown", _operator_argv(selection, "teardown", True),
        cfg["paths"]["worktree"], 35, streams / "teardown.stdout")
    stages["abort-prelaunch-cleanup"] = stage(
        "abort-prelaunch-cleanup",
        _operator_argv(selection, "abort-prelaunch-cleanup", True),
        cfg["paths"]["worktree"], 35,
        streams / "abort-prelaunch-cleanup.stdout")
    return stages


def _decoder_expected(selection: dict) -> dict:
    return {
        "route": selection["configFacts"]["route"],
        "derivedCurrentDisposition":
            selection["configFacts"]["route"]["derivedCurrentDisposition"],
        "baseHead": selection["guardConstants"]["baseHead"],
        "cargoLockSha256":
            selection["guardConstants"]["cargoLockSha256"],
        "policyCandidateSha256":
            selection["guardConstants"]["policyCandidateSha256"],
        "policyHoldbackS":
            selection["guardConstants"]["policyHoldbackS"],
        "policySupervisorAllowanceS":
            selection["guardConstants"]["policySupervisorAllowanceS"],
        "policyTotalWallMaxS":
            selection["guardConstants"]["policyTotalWallMaxS"],
        "policyFWallMaxS":
            selection["guardConstants"]["policyFWallMaxS"],
        "expectedFloorsS":
            selection["guardConstants"]["expectedFloorsS"],
        "preservedDFreezeSha256":
            selection["helperConstants"]["preservedDFreezeSha256"],
        "preservedEResolvedSha256":
            selection["helperConstants"]["preservedEResolvedSha256"],
        "paths": selection["configFacts"]["paths"],
        "totalWallSeconds": selection["configFacts"]["totalWallSeconds"],
        "fWallSeconds": selection["configFacts"]["fWallSeconds"],
    }


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


F_BASE_KEYS = {"phase", "route", "base_head", "cargo_lock_sha256",
               "historical_debit_state_start",
               "historical_debit_state_end",
               "source_home_reconstitution", "cached_target_seed_counts",
               "post_compile_check", "f_result"}
F_POLICY_KEYS = {"supervisorCeilingKind", "dispositionKind",
                 "policy_candidate_sha256", "policy_adoption_sha256",
                 "policy_holdback_s_withheld"}
OBSERVATION_BASIS = ("monotonic_ns + RUSAGE_SELF/CHILDREN at "
                     "cached_current_f entry and before result receipt; "
                     "outer Root process tail excluded (P-6 remains "
                     "Root record)")
DEBIT_STATE_KEYS = {
    "historical_debits",
    "post_p10_debits",
    "post_p11_debits",
    "current_disposition",
    "wrapper_c_us",
    "live_envelope_usec_at_start",
    "live_supervisor_usec_at_start",
    "live_phases_usec_at_start",
    "effective_envelope_usec_at_start",
    "effective_supervisor_usec_at_start",
    "effective_phases_usec_at_start",
}
HISTORICAL_POST_R3_DEBIT_KEYS = {
    "prior_post_r3_envelope_usec",
    "prior_post_r3_supervisor_usec",
    "prior_post_r3_phases_usec",
    "retired_p2_p3_envelope_usec",
    "retired_p2_p3_supervisor_usec",
    "retired_p2_p3_phases_usec",
    "retired_p4_envelope_usec",
    "retired_p4_supervisor_usec",
    "retired_p4_phases_usec",
    "retired_p5_envelope_usec",
    "retired_p5_supervisor_usec",
    "retired_p5_phases_usec",
    "retired_p6_envelope_usec",
    "retired_p6_supervisor_usec",
    "retired_p6_phases_usec",
    "retired_p7_envelope_usec",
    "retired_p7_supervisor_usec",
    "retired_p7_phases_usec",
    "retired_p8_envelope_usec",
    "retired_p8_supervisor_usec",
    "retired_p8_phases_usec",
    "retired_p9_envelope_usec",
    "retired_p9_supervisor_usec",
    "retired_p9_phases_usec",
    "retired_cached_attempts_envelope_usec",
    "retired_cached_attempts_supervisor_usec",
    "retired_cached_attempts_phases_usec",
    "envelope_usec",
    "supervisor_usec",
    "supervisor_cpu_seconds_ceil",
    "phases_usec",
    "f_stdout_bytes",
    "f_stderr_bytes",
    "r3_outer_wall_seconds_ceil",
    "r3_inner_wall_seconds_ceil",
    "prior_a0_to_e_wall_seconds_ceil",
    "p2_refusal_wall_seconds_ceil",
    "p3_refusal_wall_seconds_ceil",
    "p4_refusal_wall_seconds_ceil",
    "p5_refusal_wall_seconds_ceil",
    "p6_refusal_wall_seconds_ceil",
    "p7_outer_wall_seconds_ceil",
    "p8_outer_wall_seconds_ceil",
    "p9_outer_wall_seconds_ceil",
    "remaining_total_wall_seconds_max",
    "remaining_f_wall_seconds_max",
    "remaining_supervisor_cpu_seconds_max",
    "gross_supervisor_cpu_seconds_max",
}
POST_P10_DEBIT_KEYS = {
    "prior_post_p9_envelope_usec",
    "prior_post_p9_supervisor_usec",
    "prior_post_p9_phases_usec",
    "retired_p10_envelope_usec",
    "retired_p10_supervisor_usec",
    "retired_p10_phases_usec",
    "envelope_usec",
    "supervisor_usec",
    "supervisor_cpu_seconds_ceil",
    "phases_usec",
    "independent_envelope_minus_supervisor_minus_phases_usec",
    "p10_f_stdout_bytes",
    "p10_f_stderr_bytes",
    "f_stdout_bytes",
    "f_stderr_bytes",
    "p10_prephase_outer_wall_seconds_ceil",
    "p10_outer_wall_seconds_ceil",
    "remaining_total_wall_seconds_max",
    "remaining_f_wall_seconds_max",
    "remaining_supervisor_cpu_seconds_max",
    "gross_supervisor_cpu_seconds_max",
}
POST_P11_DEBIT_KEYS = {
    "retired_p11_envelope_usec",
    "retired_p11_supervisor_usec",
    "retired_p11_phases_usec",
    "retired_p11_independent_residual_usec",
    "envelope_usec",
    "supervisor_usec",
    "supervisor_cpu_seconds_ceil",
    "phases_usec",
    "independent_envelope_minus_supervisor_minus_phases_usec",
    "p11_f_stdout_bytes",
    "p11_f_stderr_bytes",
    "p11_outer_controller_stderr_bytes_separate",
    "f_stdout_bytes",
    "f_stderr_bytes",
    "p11_outer_wall_seconds_ceil",
    "remaining_total_wall_seconds_max",
    "remaining_f_wall_seconds_max",
    "remaining_supervisor_cpu_seconds_max",
    "gross_supervisor_cpu_seconds_max",
}
POLICY_DISPOSITION_KEYS = {
    "disposition_kind",
    "cumulative_before_wrapper_E_us",
    "cumulative_before_wrapper_S_us",
    "cumulative_before_wrapper_P_us",
    "effective_prior_E_us",
    "effective_prior_S_us",
    "effective_prior_P_us",
    "wrapper_c_us",
    "exact_remaining_supervisor_s",
    "policy_holdback_s",
    "policy_supervisor_allowance_s",
    "policy_remaining_total_wall_max_s",
    "policy_remaining_f_wall_max_s",
}
ESTABLISHED_DISPOSITION_KEYS = {
    "disposition_kind",
    "cumulative_before_wrapper_E_us",
    "cumulative_before_wrapper_S_us",
    "cumulative_before_wrapper_P_us",
    "effective_prior_E_us",
    "effective_prior_S_us",
    "effective_prior_P_us",
    "remaining_supervisor_cpu_seconds",
    "exact_remaining_supervisor_s",
    "supervisor_spend_ceiling_s",
}
SOURCE_HOME_RECONSTITUTION_KEYS = {
    "preserved_d_freeze_sha256",
    "new_d_freeze_sha256",
    "preserved_h_s",
    "new_h_s",
    "authenticated_source_files_verified",
    "expected_generated_files_verified",
    "reconstructed_cargo_ok_count",
    "reconstructed_cargo_ok",
    "bookkeeping_hash_check_sha256",
    "git_control_snapshot_projection_sha256",
    "dated_bookkeeping_deltas",
    "source_epoch_ns",
    "frozen_index_cohort",
    "current_host_units_remapped",
    "preserved_e_resolved_sha256",
}
CACHED_TARGET_SEED_COUNTS_KEYS = {
    "owner_literal_workspace_fingerprint",
    "owner_literal_workspace_deps",
    "owner_literal_workspace_incremental",
    "unadopted_other_cache_candidate",
}
F_ENVIRONMENT_KEYS = {
    "PATH", "CARGO_HOME", "HOME", "TMPDIR", "LANG", "CARGO_TARGET_DIR",
}


def _is_int(value) -> bool:
    return type(value) is int and not isinstance(value, bool)


def _is_safe_rel(value) -> bool:
    return (type(value) is str and bool(value) and not value.startswith("/")
            and str(Path(value)) == value and ".." not in Path(value).parts)


def _is_norm_abs(value) -> bool:
    return (type(value) is str and Path(value).is_absolute()
            and str(Path(os.path.normpath(value))) == value)


def _validate_wrapper_observation(obs, at: str) -> dict:
    if type(obs) is not dict or set(obs) != {
            "monotonic_ns", "self_usec", "children_usec"}:
        raise CollectError(f"{at}: exact _wrapper_observation required")
    if not _is_int(obs["monotonic_ns"]) or obs["monotonic_ns"] < 0:
        raise CollectError(f"{at}.monotonic_ns: nonnegative int required")
    pair = (obs["self_usec"], obs["children_usec"])
    if pair == (None, None):
        return obs  # helper permits null rusage; it stays null
    if any(not _is_int(v) or v < 0 for v in pair):
        raise CollectError(f"{at}: rusage usec both null or both "
                           "nonnegative ints")
    return obs


def _validate_self_observation(value) -> dict:
    if type(value) is not dict or set(value) != {
            "start_at_cached_current_f_entry",
            "end_before_result_receipt", "duration_monotonic_ns",
            "basis", "tail_after_receipt_excluded"}:
        raise CollectError("supervisor_self_observation: exact fields")
    start = _validate_wrapper_observation(
        value["start_at_cached_current_f_entry"],
        "supervisor_self_observation.start")
    end = _validate_wrapper_observation(
        value["end_before_result_receipt"],
        "supervisor_self_observation.end")
    duration = value["duration_monotonic_ns"]
    if not _is_int(duration) or duration < 0 or duration \
            != end["monotonic_ns"] - start["monotonic_ns"]:
        raise CollectError("supervisor_self_observation: duration != "
                           "end-start monotonic")
    if value["basis"] != OBSERVATION_BASIS \
            or value["tail_after_receipt_excluded"] is not True:
        raise CollectError("supervisor_self_observation: basis/tail "
                           "must stay the helper's own wording")
    return value


def _validate_debit_state(state, at: str, route_kind: str,
                          expected: dict | None = None) -> dict:
    if type(state) is not dict or set(state) != DEBIT_STATE_KEYS:
        raise CollectError(f"{at}: exact 11-key helper debit state required")
    for layer_name, expected_keys in (
        ("historical_debits", HISTORICAL_POST_R3_DEBIT_KEYS),
        ("post_p10_debits", POST_P10_DEBIT_KEYS),
        ("post_p11_debits", POST_P11_DEBIT_KEYS),
    ):
        layer = state[layer_name]
        if (type(layer) is not dict or set(layer) != expected_keys
                or any(not _is_int(v) or v < 0 for v in layer.values())):
            raise CollectError(
                f"{at}.{layer_name}: exact nonnegative int map required")
    for field in ("live_envelope_usec_at_start",
                  "live_supervisor_usec_at_start",
                  "live_phases_usec_at_start",
                  "effective_envelope_usec_at_start",
                  "effective_supervisor_usec_at_start",
                  "effective_phases_usec_at_start"):
        if not _is_int(state[field]) or state[field] < 0:
            raise CollectError(f"{at}.{field}: nonnegative int required")
    disp = state["current_disposition"]
    if type(disp) is not dict:
        raise CollectError(f"{at}.current_disposition: object required")
    wrapper_c = state["wrapper_c_us"]
    if route_kind == "policy":
        if "remaining_supervisor_cpu_seconds" in disp:
            raise CollectError(f"{at}: policy allowance must not be "
                               "encoded as a remaining balance")
        if set(disp) != POLICY_DISPOSITION_KEYS:
            raise CollectError(
                f"{at}.current_disposition: exact 13 policy keys required")
        if disp["disposition_kind"] != "CARRIED_UNKNOWN_POLICY_SPEND":
            raise CollectError(f"{at}: policy disposition_kind required")
        if wrapper_c is not None or disp["wrapper_c_us"] is not None:
            raise CollectError(f"{at}: policy c must stay null (never "
                               "0 or any numeric c/exact-S promotion)")
        if disp["exact_remaining_supervisor_s"] is not None:
            raise CollectError(f"{at}: policy exact_remaining_supervisor_s "
                               "must stay null")
        for k in ("cumulative_before_wrapper_E_us",
                  "cumulative_before_wrapper_S_us",
                  "cumulative_before_wrapper_P_us",
                  "effective_prior_E_us",
                  "effective_prior_S_us",
                  "effective_prior_P_us"):
            if not _is_int(disp[k]) or disp[k] < 0:
                raise CollectError(f"{at}.current_disposition.{k}: int >= 0")
        reserve_e = (disp["effective_prior_E_us"]
                     - disp["cumulative_before_wrapper_E_us"])
        reserve_s = (disp["effective_prior_S_us"]
                     - disp["cumulative_before_wrapper_S_us"])
        if (reserve_e <= 0 or reserve_e != reserve_s
                or disp["effective_prior_P_us"]
                != disp["cumulative_before_wrapper_P_us"]):
            raise CollectError(
                f"{at}.current_disposition: policy prior reserve arithmetic")
        holdback_s = disp["policy_holdback_s"]
        allowance_s = disp["policy_supervisor_allowance_s"]
        rem_total_s = disp["policy_remaining_total_wall_max_s"]
        rem_f_s = disp["policy_remaining_f_wall_max_s"]
        if not _is_int(holdback_s) or holdback_s < 0:
            raise CollectError(f"{at}: policy_holdback_s int >= 0 required")
        if not _is_int(allowance_s) or allowance_s <= 0:
            raise CollectError(
                f"{at}: policy_supervisor_allowance_s int > 0 required")
        if (not _is_int(rem_total_s) or rem_total_s <= 0
                or not _is_int(rem_f_s) or rem_f_s <= 0
                or rem_f_s > rem_total_s):
            raise CollectError(
                f"{at}: policy_remaining_*_wall_max_s positive ints required")
        if expected is not None:
            derived_disp = expected.get("derivedCurrentDisposition")
            pol_total_s = expected.get("policyTotalWallMaxS")
            pol_f_s = expected.get("policyFWallMaxS")
            pol_holdback_s = expected.get("policyHoldbackS")
            pol_allowance_s = expected.get("policySupervisorAllowanceS")
            tot_cfg = expected.get("totalWallSeconds")
            f_cfg = expected.get("fWallSeconds")
            floors = expected.get("expectedFloorsS")
            if (type(derived_disp) is not dict
                    or not _is_int(pol_total_s)
                    or not _is_int(pol_f_s)
                    or not _is_int(pol_holdback_s)
                    or not _is_int(pol_allowance_s)
                    or not _is_int(tot_cfg)
                    or not _is_int(f_cfg)
                    or type(floors) is not tuple or len(floors) != 3
                    or any(not _is_int(x) for x in floors)):
                raise CollectError(
                    f"{at}: expected policy guard/config constants missing")
            if (holdback_s != pol_holdback_s
                    or allowance_s != pol_allowance_s
                    or rem_total_s != pol_total_s
                    or rem_f_s != pol_f_s
                    or rem_total_s != floors[0] - holdback_s
                    or rem_f_s != floors[1] - holdback_s):
                raise CollectError(
                    f"{at}: policy wall maxima or holdback/allowance "
                    "mismatch against pinned guard/config")
            if tot_cfg > rem_total_s:
                raise CollectError(
                    f"{at}: total_wall_seconds > policy_remaining_total")
            if f_cfg > rem_f_s:
                raise CollectError(
                    f"{at}: f_wall_seconds > policy_remaining_f")
            if disp != derived_disp:
                raise CollectError(
                    f"{at}.current_disposition != derived policy disposition")
    else:
        if set(disp) != ESTABLISHED_DISPOSITION_KEYS:
            raise CollectError(
                f"{at}.current_disposition: exact 10 established keys")
        if disp["disposition_kind"] != "ESTABLISHED_PINNED":
            raise CollectError(f"{at}: established disposition required")
        if not _is_int(wrapper_c) or wrapper_c < 0:
            raise CollectError(f"{at}.wrapper_c_us: established int")
        for k in ("cumulative_before_wrapper_E_us",
                  "cumulative_before_wrapper_S_us",
                  "cumulative_before_wrapper_P_us",
                  "effective_prior_E_us",
                  "effective_prior_S_us",
                  "effective_prior_P_us"):
            if not _is_int(disp[k]) or disp[k] < 0:
                raise CollectError(f"{at}.current_disposition.{k}: int >= 0")
        reserve_e = (disp["effective_prior_E_us"]
                     - disp["cumulative_before_wrapper_E_us"])
        reserve_s = (disp["effective_prior_S_us"]
                     - disp["cumulative_before_wrapper_S_us"])
        if (reserve_e < wrapper_c or reserve_e != reserve_s
                or disp["effective_prior_P_us"]
                != disp["cumulative_before_wrapper_P_us"]):
            raise CollectError(
                f"{at}.current_disposition: established prior arithmetic")
        exact = disp["exact_remaining_supervisor_s"]
        if (not _is_int(exact) or exact <= 0
                or disp["remaining_supervisor_cpu_seconds"] != exact
                or disp["supervisor_spend_ceiling_s"] != exact):
            raise CollectError(f"{at}: established exact-S inconsistent")
        if expected is not None:
            derived_disp = expected.get("derivedCurrentDisposition")
            if type(derived_disp) is not dict or disp != derived_disp:
                raise CollectError(
                    f"{at}.current_disposition != derived established "
                    "disposition")
    if (state["effective_envelope_usec_at_start"]
            != disp["effective_prior_E_us"]
            + state["live_envelope_usec_at_start"]
            or state["effective_supervisor_usec_at_start"]
            != disp["effective_prior_S_us"]
            + state["live_supervisor_usec_at_start"]
            or state["effective_phases_usec_at_start"]
            != disp["effective_prior_P_us"]
            + state["live_phases_usec_at_start"]):
        raise CollectError(
            f"{at}: effective != effective_prior + live arithmetic mismatch")
    return disp


def _validate_source_home_reconstitution(
        value, expected: dict | None = None) -> dict:
    if (type(value) is not dict
            or set(value) != SOURCE_HOME_RECONSTITUTION_KEYS):
        raise CollectError(
            "source_home_reconstitution: exact 15 helper keys required")
    for sha_key in ("preserved_d_freeze_sha256", "new_d_freeze_sha256",
                    "preserved_h_s", "new_h_s",
                    "bookkeeping_hash_check_sha256",
                    "git_control_snapshot_projection_sha256",
                    "preserved_e_resolved_sha256"):
        v = value[sha_key]
        if type(v) is not str or not HEX.fullmatch(v):
            raise CollectError(
                f"source_home_reconstitution.{sha_key}: 64-hex required")
    for pos_key in ("authenticated_source_files_verified",
                    "expected_generated_files_verified",
                    "source_epoch_ns",
                    "current_host_units_remapped"):
        v = value[pos_key]
        if not _is_int(v) or v <= 0:
            raise CollectError(
                f"source_home_reconstitution.{pos_key}: int > 0 required")
    ok_count = value["reconstructed_cargo_ok_count"]
    ok_list = value["reconstructed_cargo_ok"]
    if (not _is_int(ok_count) or ok_count < 0 or type(ok_list) is not list
            or len(ok_list) != ok_count):
        raise CollectError(
            "source_home_reconstitution.reconstructed_cargo_ok mismatch")
    for item in ok_list:
        if (type(item) is not dict
                or set(item) != {"relative", "bytes", "sha256"}
                or not _is_safe_rel(item["relative"])
                or not _is_int(item["bytes"]) or item["bytes"] < 0
                or type(item["sha256"]) is not str
                or not HEX.fullmatch(item["sha256"])):
            raise CollectError(
                "source_home_reconstitution.reconstructed_cargo_ok[] invalid")
    deltas = value["dated_bookkeeping_deltas"]
    if type(deltas) is not list or not deltas:
        raise CollectError(
            "source_home_reconstitution.dated_bookkeeping_deltas required")
    for item in deltas:
        if type(item) is not dict:
            raise CollectError("dated_bookkeeping_deltas[]: object required")
        kind = item.get("kind")
        extra_sha_key = ("bookkeeping_check_sha256"
                         if kind == "cargo-lockfile"
                         else ("git_control_projection_sha256"
                               if kind == "git-control" else None))
        req_keys = {"relative", "kind", "historical_d_sha256",
                    "current_sha256", "bytes", "immutable_snapshot_path"}
        if extra_sha_key is None or set(item) != (req_keys | {extra_sha_key}):
            raise CollectError("dated_bookkeeping_deltas[]: invalid keys")
        if (not _is_safe_rel(item["relative"])
                or not _is_int(item["bytes"]) or item["bytes"] <= 0
                or not _is_norm_abs(item["immutable_snapshot_path"])
                or any(type(item[k]) is not str or not HEX.fullmatch(item[k])
                       for k in ("historical_d_sha256", "current_sha256",
                                 extra_sha_key))):
            raise CollectError("dated_bookkeeping_deltas[]: invalid fields")
    cohort = value["frozen_index_cohort"]
    if (type(cohort) is not dict
            or set(cohort) != {"manifest_sha256", "file_count",
                               "index_destinations_staged_from_cohort"}
            or type(cohort["manifest_sha256"]) is not str
            or not HEX.fullmatch(cohort["manifest_sha256"])
            or not _is_int(cohort["file_count"]) or cohort["file_count"] <= 0
            or cohort["index_destinations_staged_from_cohort"]
            != cohort["file_count"]):
        raise CollectError(
            "source_home_reconstitution.frozen_index_cohort invalid")
    if expected is not None:
        exp_d = expected.get("preservedDFreezeSha256")
        exp_e = expected.get("preservedEResolvedSha256")
        if (type(exp_d) is not str or not HEX.fullmatch(exp_d)
                or type(exp_e) is not str or not HEX.fullmatch(exp_e)):
            raise CollectError(
                "expected helper PRESERVED_PACKET_PINS D/E digests missing")
        if value["preserved_d_freeze_sha256"] != exp_d:
            raise CollectError(
                "preserved_d_freeze_sha256 != helper PRESERVED_PACKET_PINS")
        if value["preserved_e_resolved_sha256"] != exp_e:
            raise CollectError(
                "preserved_e_resolved_sha256 != helper PRESERVED_PACKET_PINS")
    return value


def _validate_cached_target_seed_counts(value) -> dict:
    if (type(value) is not dict
            or set(value) != CACHED_TARGET_SEED_COUNTS_KEYS
            or any(not _is_int(v) or v <= 0 for v in value.values())):
        raise CollectError(
            "cached_target_seed_counts: exact 4 positive int counts required")
    return value


def _validate_f_launch_result(inner, expected: dict | None = None) -> dict:
    if type(inner) is not dict or set(inner) != {
            "phase", "processes", "environment", "wall_seconds",
            "log_bytes"} or inner["phase"] != "F":
        raise CollectError("f_result: exact launch record required")
    env = inner["environment"]
    if (type(env) is not dict or set(env) != F_ENVIRONMENT_KEYS
            or any(type(v) is not str or not v for v in env.values())
            or env["LANG"] != "C"
            or any(not _is_norm_abs(env[k])
                   for k in ("CARGO_HOME", "HOME", "TMPDIR",
                             "CARGO_TARGET_DIR"))
            or any(not _is_norm_abs(p) for p in env["PATH"].split(":"))):
        raise CollectError("f_result.environment: exact cargo_env required")
    if expected is not None and type(expected.get("paths")) is dict:
        cpaths = expected["paths"]
        if (env["CARGO_HOME"] != cpaths.get("source_home")
                or env["CARGO_TARGET_DIR"] != cpaths.get("target")
                or env["HOME"] != cpaths.get("empty_home")
                or env["TMPDIR"] != cpaths.get("tmp")):
            raise CollectError(
                "f_result.environment paths != pinned config paths")
    processes = inner["processes"]
    if type(processes) is not list or not processes or len(processes) > 16:
        raise CollectError("f_result.processes: bounded nonempty list")
    has_cargo_proc = False
    for idx, p in enumerate(processes):
        if (type(p) is not dict
                or set(p) != {"argv", "audit_worker", "child_exit"}
                or not _is_int(p["child_exit"]) or p["child_exit"] != 0
                or type(p["audit_worker"]) is not bool):
            raise CollectError(
                "f_result.processes: exact fields and child_exit==0 required")
        argv = p["argv"]
        if (type(argv) is not list or not 1 <= len(argv) <= 4096
                or any(type(a) is not str or not a for a in argv)):
            raise CollectError(
                f"f_result.processes[{idx}].argv: bounded nonempty str list "
                "(1..4096) required")
        if argv[0] != "--die-with-parent":
            raise CollectError(
                f"f_result.processes[{idx}].argv[0]: expected "
                "'--die-with-parent' from guard sandbox_args")
        if "--" not in argv:
            raise CollectError(
                f"f_result.processes[{idx}].argv: missing '--' bwrap "
                "separator")
        sep = len(argv) - 1 - argv[::-1].index("--")
        if (sep <= 0 or sep + 4 >= len(argv)
                or argv[sep + 1:sep + 5]
                != ["/usr/bin/python3", "-I", "-B", "-S"]):
            raise CollectError(
                f"f_result.processes[{idx}].argv: expected "
                "['/usr/bin/python3', '-I', '-B', '-S'] after '--'")
        if p["audit_worker"] is False:
            has_cargo_proc = True
    if not has_cargo_proc:
        raise CollectError(
            "f_result.processes: at least one non-audit Cargo process required")
    wall = inner["wall_seconds"]
    if type(wall) not in (int, float) or isinstance(wall, bool) \
            or not math.isfinite(wall) or wall < 0:
        raise CollectError("f_result.wall_seconds: finite nonnegative")
    logb = inner["log_bytes"]
    if type(logb) is not dict or set(logb) != {"stdout", "stderr"} \
            or any(not _is_int(v) or v < 0 for v in logb.values()):
        raise CollectError("f_result.log_bytes: exact stdout/stderr ints")
    return inner


def _validate_quarantined_outputs(outputs) -> dict:
    if type(outputs) is not dict or set(outputs) != {
            "all_outputs", "host_outputs", "host_fingerprints",
            "build_script_outputs", "acceptance"}:
        raise CollectError("F_QUARANTINED_OUTPUTS: exact fields required")
    if outputs["acceptance"] != ("UNACCEPTED: independent result review "
                                  "required; no artifact executed"):
        raise CollectError("F outputs acceptance must remain UNACCEPTED")
    all_outputs = outputs["all_outputs"]
    if type(all_outputs) is not dict or not all_outputs \
            or len(all_outputs) > 1000000 or any(
                not _is_safe_rel(k) or type(v) is not str
                or not HEX.fullmatch(v) for k, v in all_outputs.items()):
        raise CollectError("all_outputs: bounded rel->sha256 map required")
    host_outputs = outputs["host_outputs"]
    if type(host_outputs) is not list:
        raise CollectError("host_outputs: list required")
    for h in host_outputs:
        if (type(h) is not dict
                or set(h) != {"path", "source_root", "identity", "name",
                              "target_name", "kind", "sha256"}
                or not _is_safe_rel(h["path"])
                or h["kind"] not in ("build-script", "proc-macro")
                or type(h["sha256"]) is not str
                or not HEX.fullmatch(h["sha256"])
                or all_outputs.get(h["path"]) != h["sha256"]):
            raise CollectError("host_outputs[]: invalid entry")
    host_fps = outputs["host_fingerprints"]
    if type(host_fps) is not list:
        raise CollectError("host_fingerprints: list required")
    for fp in host_fps:
        if (type(fp) is not dict
                or set(fp) != {"path", "name", "kind", "source_root",
                               "identity"}
                or not _is_safe_rel(fp["path"])
                or fp["kind"] not in ("build-script", "proc-macro")):
            raise CollectError("host_fingerprints[]: invalid entry")
    bs_outs = outputs["build_script_outputs"]
    if type(bs_outs) is not list:
        raise CollectError("build_script_outputs: list required")
    for b in bs_outs:
        if (type(b) is not dict
                or set(b) != {"path", "identity", "source_root", "files"}
                or not _is_safe_rel(b["path"])
                or type(b["files"]) is not dict
                or any(not _is_safe_rel(k) or type(v) is not str
                       or not HEX.fullmatch(v)
                       for k, v in b["files"].items())):
            raise CollectError("build_script_outputs[]: invalid entry")
    return all_outputs


def _fresh_entries(check: dict) -> list[dict]:
    fresh = check["fresh_mori_backend_executables"]
    if type(fresh) is not list or not fresh or len(fresh) > 64:
        raise CollectError("fresh_mori_backend_executables: bounded "
                           "nonempty list required")
    for entry in fresh:
        if type(entry) is not dict or set(entry) != {
                "relative", "sha256", "bytes"}:
            raise CollectError("fresh entry: exactly relative/sha256/"
                               "bytes")
        rel = entry["relative"]
        if not _is_safe_rel(rel):
            raise CollectError("fresh entry.relative: safe relative "
                               "path required")
        if type(entry["sha256"]) is not str \
                or not HEX.fullmatch(entry["sha256"]):
            raise CollectError("fresh entry.sha256: lowercase SHA-256")
        if not _is_int(entry["bytes"]) or not 0 < entry["bytes"] \
                <= MAX_ELF_BYTES:
            raise CollectError("fresh entry.bytes: bounded positive int")
    return fresh


def decode_cached_current_f(raw_by_role: dict, expected: dict | None = None,
                            preservation: dict | None = None) -> dict:
    """Decode the current helper's actual cached-current-F packet."""
    def load(role):
        raw = _read_raw(raw_by_role[role], role)
        try:
            return json.loads(raw.decode(), object_pairs_hook=_pairs,
                              parse_constant=_constant, parse_float=_float)
        except Exception as exc:
            raise CollectError(f"{role}: invalid JSON: {exc}") from exc

    f_result_doc = load("f-result")
    established_keys = F_BASE_KEYS | {"supervisor_self_observation"}
    policy_keys = established_keys | F_POLICY_KEYS
    if type(f_result_doc) is not dict or set(f_result_doc) \
            not in (established_keys, policy_keys):
        raise CollectError("F_CACHED_CURRENT_RESULT: exact current "
                           "helper fields required (base+observation, "
                           "plus exactly the five policy fields on the "
                           "adopted-policy route; ten-key old shape and "
                           "arbitrary extras refused)")
    route_kind = "policy" if set(f_result_doc) == policy_keys \
        else "established"
    if f_result_doc["phase"] != "F" \
            or f_result_doc["route"] != "cached-current-f":
        raise CollectError("F_CACHED_CURRENT_RESULT: phase/route mismatch")
    if type(f_result_doc["base_head"]) is not str or not HEX40.fullmatch(
            f_result_doc["base_head"]) \
            or type(f_result_doc["cargo_lock_sha256"]) is not str \
            or not HEX.fullmatch(f_result_doc["cargo_lock_sha256"]):
        raise CollectError("F_CACHED_CURRENT_RESULT: base/lock pins")
    observation = _validate_self_observation(
        f_result_doc["supervisor_self_observation"])
    start_state = f_result_doc["historical_debit_state_start"]
    end_state = f_result_doc["historical_debit_state_end"]
    disp_start = _validate_debit_state(
        start_state, "historical_debit_state_start", route_kind, expected)
    disp_end = _validate_debit_state(
        end_state, "historical_debit_state_end", route_kind, expected)
    if (disp_start != disp_end
            or start_state["wrapper_c_us"] != end_state["wrapper_c_us"]
            or start_state["historical_debits"]
            != end_state["historical_debits"]
            or start_state["post_p10_debits"]
            != end_state["post_p10_debits"]
            or start_state["post_p11_debits"]
            != end_state["post_p11_debits"]):
        raise CollectError(
            "historical_debit_state start/end disposition or history mismatch")
    if (end_state["live_envelope_usec_at_start"]
            < start_state["live_envelope_usec_at_start"]
            or end_state["live_supervisor_usec_at_start"]
            < start_state["live_supervisor_usec_at_start"]
            or end_state["live_phases_usec_at_start"]
            < start_state["live_phases_usec_at_start"]):
        raise CollectError(
            "historical_debit_state live counters regressed from start to end")
    reconstitution = _validate_source_home_reconstitution(
        f_result_doc["source_home_reconstitution"], expected)
    seed_counts = _validate_cached_target_seed_counts(
        f_result_doc["cached_target_seed_counts"])
    receipt = {"routeKind": route_kind,
               "baseHead": f_result_doc["base_head"],
               "cargoLockSha256": f_result_doc["cargo_lock_sha256"],
               "supervisorSelfObservation": observation,
               "supervisorCeilingKind": None, "dispositionKind": None,
               "policyCandidateSha256": None,
               "policyAdoptionSha256": None,
               "policyHoldbackSWithheld": None,
               "wrapperCUs": start_state["wrapper_c_us"],
               "exactRemainingSupervisorS":
                   disp_start["exact_remaining_supervisor_s"],
               "historicalDebitStateStart": start_state,
               "historicalDebitStateEnd": end_state,
               "sourceHomeReconstitution": reconstitution,
               "cachedTargetSeedCounts": seed_counts}
    if route_kind == "policy":
        if f_result_doc["supervisorCeilingKind"] \
                != "POLICY_ALLOWANCE_NOT_REMAINING_BALANCE" \
                or f_result_doc["dispositionKind"] \
                != "CARRIED_UNKNOWN_POLICY_SPEND":
            raise CollectError("policy ceiling/disposition kinds must "
                               "stay the helper's exact strings")
        for key in ("policy_candidate_sha256",
                    "policy_adoption_sha256"):
            if type(f_result_doc[key]) is not str \
                    or not HEX.fullmatch(f_result_doc[key]):
                raise CollectError(f"{key}: lowercase SHA-256 required")
        withheld = f_result_doc["policy_holdback_s_withheld"]
        if not _is_int(withheld) or withheld < 0:
            raise CollectError("policy_holdback_s_withheld: int >= 0")
        for disp in (disp_start, disp_end):
            if disp["policy_holdback_s"] != withheld \
                    or not _is_int(disp["policy_supervisor_allowance_s"]) \
                    or disp["policy_supervisor_allowance_s"] <= 0:
                raise CollectError("debit disposition holdback/allowance "
                                   "!= receipt policy fields")
        receipt.update({
            "supervisorCeilingKind":
                f_result_doc["supervisorCeilingKind"],
            "dispositionKind": f_result_doc["dispositionKind"],
            "policyCandidateSha256":
                f_result_doc["policy_candidate_sha256"],
            "policyAdoptionSha256":
                f_result_doc["policy_adoption_sha256"],
            "policyHoldbackSWithheld": withheld,
            "policySupervisorAllowanceS":
                disp_start["policy_supervisor_allowance_s"]})
    if expected is not None:
        exp_route = expected["route"]
        if exp_route["kind"] != route_kind:
            raise CollectError(
                f"receipt route {route_kind} != selected config route "
                f"{exp_route['kind']}")
        if f_result_doc["base_head"] != expected["baseHead"] \
                or f_result_doc["cargo_lock_sha256"] \
                != expected["cargoLockSha256"]:
            raise CollectError("receipt base/lock != pinned guard "
                               "constants")
        if route_kind == "policy":
            if receipt["policyCandidateSha256"] \
                    != expected["policyCandidateSha256"] \
                    or receipt["policyAdoptionSha256"] \
                    != exp_route["policyAdoptionSha256"] \
                    or receipt["policyHoldbackSWithheld"] \
                    != exp_route["policyHoldbackS"] \
                    or receipt["policySupervisorAllowanceS"] \
                    != exp_route["policyAllowanceS"]:
                raise CollectError("receipt policy pins != pinned "
                                   "guard/config adoption")
        elif receipt["wrapperCUs"] != exp_route["wrapperCUs"]:
            raise CollectError("receipt wrapper c != pinned config")
    _validate_f_launch_result(f_result_doc["f_result"], expected)
    check = load("post-compile-check")
    if check != f_result_doc["post_compile_check"]:
        raise CollectError("post_compile_check != CACHED_POST_COMPILE_CHECK")
    if type(check) is not dict or set(check) != {
            "host_outputs_verified_unchanged",
            "build_script_run_units_verified_not_reexecuted",
            "fresh_mori_backend_executables"}:
        raise CollectError("CACHED_POST_COMPILE_CHECK: exact fields required")
    for count_field in ("host_outputs_verified_unchanged",
                        "build_script_run_units_verified_not_reexecuted"):
        if not _is_int(check[count_field]) or check[count_field] < 0:
            raise CollectError(f"CACHED_POST_COMPILE_CHECK.{count_field}")
    fresh = _fresh_entries(check)
    outputs = load("f-outputs")
    all_outputs = _validate_quarantined_outputs(outputs)
    elf = _file_ref("backend-elf", raw_by_role["backend-elf"])
    candidates = [f for f in fresh
                  if all_outputs.get(f["relative"]) == f["sha256"]]
    matches = [f for f in candidates
               if f["sha256"] == elf["sha256"]
               and f["bytes"] == elf["bytes"]]
    if len(candidates) != 1 or len(matches) != 1:
        raise CollectError("backend ELF is not the unique fresh F output "
                           "(stale/seed/missing/multiple candidates "
                           "refused)")
    fresh_one = matches[0]
    if elf["bytes"] == 0 or elf["bytes"] > MAX_ELF_BYTES:
        raise CollectError("backend ELF emptiness/size mismatch")
    if elf["path"].endswith("/" + fresh_one["relative"]):
        pass  # direct live path: provenance is the path itself
    elif preservation is not None:
        if type(preservation) is not dict \
                or preservation.get("actualRelative") \
                != fresh_one["relative"] \
                or preservation.get("retainedPath") != elf["path"] \
                or preservation.get("sha256") != elf["sha256"] \
                or preservation.get("bytes") != elf["bytes"] \
                or type(preservation.get("sourcePath")) is not str \
                or not preservation["sourcePath"].endswith(
                    "/" + fresh_one["relative"]):
            raise CollectError("retained ELF alias lacks a validated "
                               "preservation mapping to the actual "
                               "fresh relative")
    else:
        raise CollectError("backend ELF path is neither the actual "
                           "fresh relative nor a preserved alias")
    return {"built": True, "test_bodies_executed": False,
            "receipt": receipt,
            "freshElf": {"relative": fresh_one["relative"],
                         "sha256": fresh_one["sha256"],
                         "bytes": fresh_one["bytes"],
                         "retainedPath": elf["path"]}}


def collect_cached_current_f(task, raw_by_role, returncode, attempt_dir,
                             expected=None, preservation=None):
    terminal, exit_code, signal = _terminal_fields(returncode)
    cleanup = _check_cleanup_receipt(task, raw_by_role)
    if terminal != "succeeded":
        raise CollectError(f"guard terminal exit {returncode}")
    decoded = decode_cached_current_f(raw_by_role, expected, preservation)
    _write_json_atomic(attempt_dir / "native-receipt-decoded.json", {
        "schema": "doctorwatson.native-receipt-decoded/1",
        "task": task["id"], "attempt": task["attempt"],
        "fingerprint": task["fingerprint"],
        "receipt": decoded["receipt"], "freshElf": decoded["freshElf"],
        "acceptance": "UNACCEPTED: independent result review required; "
                      "no artifact executed"})
    payload = {"built": decoded["built"],
               "test_bodies_executed": decoded["test_bodies_executed"]}
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
                   raw_receipt_ref: dict, expected=None,
                   preservation=None):
    """Run the fixed collector; write consumer Result + pinned sidecar."""
    spec = ADAPTERS[task["adapter"]]
    if spec["collector"] is None:
        raise CollectError(f"no collector for {task['adapter']}")
    raw_by_role = {o["role"]: o["path"] for o in raw_outputs}
    if set(raw_by_role) != set(spec["rawOutputRoles"]):
        raise CollectError("raw output membership != fixed collector roles")
    if spec["collector"] == "collect_cached_current_f":
        result = collect_cached_current_f(
            task, raw_by_role, returncode, attempt_dir,
            expected=expected, preservation=preservation)
    else:
        result = COLLECTORS[spec["collector"]](task, raw_by_role,
                                               returncode, attempt_dir)
    result_ref = _write_json_atomic(attempt_dir / "consumer-result.json",
                                    result)
    sidecar = {"schema": "doctorwatson.collector-sidecar/2",
               "task": task["id"], "attempt": task["attempt"],
               "fingerprint": task["fingerprint"],
               "adapter": task["adapter"], "collector": spec["collector"],
               "rawReceipt": raw_receipt_ref,
               "rawOutputs": [_file_ref(o["role"], o["path"])
                              for o in raw_outputs],
               "consumerResult": result_ref,
               "decodedReceipt": None, "preservation": None}
    decoded_path = attempt_dir / "native-receipt-decoded.json"
    if decoded_path.is_file():
        sidecar["decodedReceipt"] = _file_ref("decoded-receipt",
                                              str(decoded_path))
    preservation_path = attempt_dir / "preservation.json"
    if preservation_path.is_file():
        sidecar["preservation"] = _file_ref("preservation",
                                            str(preservation_path))
    _write_json_atomic(attempt_dir / "collector.json", sidecar)
    return result


def _load_capture_module(ref: dict):
    spec = importlib.util.spec_from_file_location(
        "doctorwatson_pinned_capture", ref["path"])
    if spec is None or spec.loader is None:
        raise StageUnknown("pinned capture module cannot be loaded")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    if not callable(getattr(module, "capture", None)):
        raise StageUnknown("pinned capture module has no capture()")
    return module


def _default_runner(stage: dict) -> int:
    """The single subprocess site (plus the pinned capture for native).

    Ordinary stages: one subprocess call with the fixed argv/cwd/env,
    streams to the stage's attempt files, bounded by wallSeconds+5. A
    timeout kills only the direct child, so it is StageUnknown, never a
    proven terminal. The native stage goes through the pinned original
    capture(op, phase): actual_exit null or any controller_failure is
    StageUnknown -- a capture return alone never proves termination.
    No shell is constructed here; the only /bin/sh is the fixed
    S-placement argv built by build_native_argv.
    """
    capture = stage.get("capture")
    if capture is not None:
        module = _load_capture_module(capture["module"])
        phase = Path(capture["phaseDir"])
        phase.mkdir(parents=False, exist_ok=False)
        result = module.capture(capture["op"], phase)
        if type(result) is not dict or not {
                "id", "actual_exit", "elapsed_seconds",
                "controller_failure", "streams"} <= set(result):
            raise StageUnknown("capture returned a malformed record")
        _write_json_atomic(Path(capture["phaseDir"]).parent
                           / "native-capture-result.json", result)
        if result["actual_exit"] is None \
                or result["controller_failure"] is not None:
            raise StageUnknown(
                f"capture actual_exit={result['actual_exit']!r} "
                f"controller_failure={result['controller_failure']!r}: "
                "termination not proven")
        if type(result["actual_exit"]) is not int \
                or isinstance(result["actual_exit"], bool):
            raise StageUnknown("capture actual_exit is not an integer")
        return result["actual_exit"]
    out = open(stage["stdout"], "xb") if stage["stdout"] else None
    err = open(stage["stderr"], "xb") if stage["stderr"] else None
    try:
        completed = subprocess.run(
            stage["argv"], cwd=stage["cwd"], env=stage["env"],
            shell=False, check=False, stdin=subprocess.DEVNULL,
            stdout=out, stderr=err,
            timeout=stage["wallSeconds"] + 5)
    except subprocess.TimeoutExpired as exc:
        raise StageUnknown(f"stage {stage['stage']} exceeded runner "
                           "bound; direct child killed, descendants "
                           "unproven") from exc
    finally:
        for stream in (out, err):
            if stream is not None:
                stream.close()
    return completed.returncode


def _rusage_snapshot() -> dict:
    def snap(which):
        try:
            ru = resource.getrusage(which)
        except Exception:
            return None
        return {"ru_utime": ru.ru_utime, "ru_stime": ru.ru_stime,
                "ru_maxrss": ru.ru_maxrss, "ru_inblock": ru.ru_inblock,
                "ru_oublock": ru.ru_oublock, "ru_nvcsw": ru.ru_nvcsw,
                "ru_nivcsw": ru.ru_nivcsw}

    return {"monotonic_ns": time.monotonic_ns(), "pid": os.getpid(),
            "uid": os.getuid(),
            "rusage_self": snap(resource.RUSAGE_SELF),
            "rusage_children": snap(resource.RUSAGE_CHILDREN)}


def _cgroup_absent(cfg_facts: dict) -> bool:
    """Restoration probe helper: envelope cgroup node is gone."""
    return not os.path.lexists(_cgroup_fs(cfg_facts)["E"])


def _read_mountinfo() -> str:
    return Path("/proc/self/mountinfo").read_text(
        encoding="utf-8", errors="replace")


def _unescape_mountinfo_path(raw: str) -> str:
    return (raw.replace("\\040", " ")
            .replace("\\011", "\t")
            .replace("\\012", "\n")
            .replace("\\134", "\\"))


def _restoration_probe(selection: dict) -> dict:
    """Verify actual cgroup node absence AND absence of all 3 request
    mountpoints (execution_root, source_home, packet) in mountinfo."""
    cfg_facts = selection["configFacts"]
    req_paths = selection["requestFacts"]["paths"]
    cg_nodes = _cgroup_fs(cfg_facts)
    cg_present = {k: bool(os.path.lexists(v))
                  for k, v in cg_nodes.items()}
    cgroup_absent = (bool(_cgroup_absent(cfg_facts))
                     and not any(cg_present.values()))
    mountpoints = [
        os.path.normpath(req_paths["execution_root"]),
        os.path.normpath(req_paths["source_home"]),
        os.path.normpath(req_paths["packet"]),
    ]
    active_mounts: list[str] = []
    matching_lines: list[str] = []
    mountinfo_error = None
    try:
        raw_mountinfo = _read_mountinfo()
        for line in raw_mountinfo.splitlines():
            pre, sep, _post = line.partition(" - ")
            if not sep:
                continue
            fields = pre.split()
            if len(fields) < 5:
                continue
            mp = os.path.normpath(_unescape_mountinfo_path(fields[4]))
            for req_mp in mountpoints:
                if mp == req_mp or mp.startswith(req_mp + "/"):
                    if req_mp not in active_mounts:
                        active_mounts.append(req_mp)
                    matching_lines.append(line)
        mounts_absent = len(active_mounts) == 0
    except Exception as exc:
        mounts_absent = False
        mountinfo_error = f"{type(exc).__name__}: {exc}"
    return {
        "cgroupNodes": cg_nodes,
        "cgroupNodePresent": cg_present,
        "cgroupAbsent": cgroup_absent,
        "requiredAbsentMountpoints": mountpoints,
        "activeRequestMountpoints": active_mounts,
        "matchingMountinfoRows": matching_lines,
        "mountsAbsent": mounts_absent,
        "mountinfoError": mountinfo_error,
        "restored": bool(cgroup_absent and mounts_absent),
    }


def _require_existing_provisioned_host(selection: dict) -> None:
    """Provisioned-host entry: OP0..OP6 are external prerequisites, so
    execution_root, config_dir, and Q-OBS observation_output must already
    exist, while ONE_SHOT_STARTED must be absent."""
    _require_real_parent_dir(selection["outer"]["retainedDir"],
                             "selection.outer.retainedDir")
    req_paths = selection["requestFacts"]["paths"]
    exec_root = req_paths["execution_root"]
    cfg_dir = req_paths["config_dir"]
    obs_out = req_paths["observation_output"]
    for label, dpath in (("execution_root", exec_root),
                         ("config_dir", cfg_dir)):
        try:
            st = os.lstat(dpath)
        except OSError as exc:
            raise Blocked(f"provisioned host {label} absent at entry: "
                          f"{exc}") from exc
        if stat.S_ISLNK(st.st_mode) or not stat.S_ISDIR(st.st_mode):
            raise Blocked(f"provisioned host {label} must be a real "
                          "non-symlink directory")
    try:
        q_obs = _load_json_file(
            obs_out, "preexisting Q-OBS observation_output")
    except (CollectError, BadInput) as exc:
        raise Blocked("preexisting Q-OBS observation_output absent or "
                      f"invalid: {exc}") from exc
    if type(q_obs) is not dict:
        raise Blocked("preexisting Q-OBS observation_output must be a "
                      "JSON object")
    if os.path.lexists(Path(exec_root) / "ONE_SHOT_STARTED"):
        raise Blocked("ONE_SHOT_STARTED already exists at selection "
                      "(stale/consumed); zero dispatch")


def _check_time_output_preexec(time_output: str) -> str:
    """Allow absent timeOutput OR a 0-byte regular non-symlink file
    precreated by wrapping /usr/bin/time -v -o <timeOutput>; refuse
    symlinks, non-regular files, or non-empty stale files."""
    try:
        st = os.lstat(time_output)
    except FileNotFoundError:
        return "absent"
    except OSError as exc:
        raise Blocked(f"timeOutput unreadable at selection: {exc}") \
            from exc
    if stat.S_ISLNK(st.st_mode) or not stat.S_ISREG(st.st_mode):
        raise Blocked("timeOutput already exists as symlink or "
                      "non-regular file; zero dispatch")
    if st.st_size != 0:
        raise Blocked("timeOutput already exists and is non-empty "
                      "(stale/consumed); zero dispatch")
    return "precreated-zero-byte"


def _marker_present(selection: dict) -> bool:
    root = Path(selection["requestFacts"]["paths"]["execution_root"])
    return os.path.lexists(root / "ONE_SHOT_STARTED")


def _fsync_dir(path: Path) -> None:
    fd = os.open(path, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def _copy_preserve(source: str, retained: Path) -> dict:
    """Durable byte-exact copy (fsync file + dir); never executes."""
    src = _file_ref("preserve-source", source)
    if src["bytes"] == 0 or src["bytes"] > MAX_ELF_BYTES:
        raise CollectError("preserve: source size outside bounds")
    fd_in = os.open(source, os.O_RDONLY | os.O_NOFOLLOW)
    fd_out = os.open(retained, os.O_WRONLY | os.O_CREAT | os.O_EXCL,
                     0o644)
    digest, total = hashlib.sha256(), 0
    try:
        while True:
            chunk = os.read(fd_in, 1024 * 1024)
            if not chunk:
                break
            total += len(chunk)
            if total > MAX_ELF_BYTES:
                raise CollectError("preserve: size limit exceeded")
            digest.update(chunk)
            view = memoryview(chunk)
            while view:
                written = os.write(fd_out, view)
                view = view[written:]
        os.fsync(fd_out)
    finally:
        os.close(fd_in)
        os.close(fd_out)
    _fsync_dir(retained.parent)
    if total != src["bytes"] or digest.hexdigest() != src["sha256"]:
        raise CollectError("preserve: copied bytes != source bytes")
    return src


def _preserve_elf(selection: dict, attempt: Path) -> dict:
    """Resolve and durably preserve the unique fresh backend ELF."""
    req = selection["requestFacts"]["paths"]
    packet = Path(req["packet"])
    target = Path(selection["configFacts"]["paths"]["target"])

    def load_packet(name, what):
        raw = _read_raw(str(packet / name), what)
        try:
            return json.loads(raw.decode(), object_pairs_hook=_pairs,
                              parse_constant=_constant,
                              parse_float=_float)
        except Exception as exc:
            raise CollectError(f"{what}: invalid JSON: {exc}") from exc

    check = load_packet("CACHED_POST_COMPILE_CHECK.json",
                         "preserve post-compile-check")
    if type(check) is not dict or set(check) != {
            "host_outputs_verified_unchanged",
            "build_script_run_units_verified_not_reexecuted",
            "fresh_mori_backend_executables"}:
        raise CollectError("preserve: post-compile-check exact fields")
    fresh = _fresh_entries(check)
    outputs = load_packet("F_QUARANTINED_OUTPUTS.json",
                          "preserve f-outputs")
    all_outputs = outputs.get("all_outputs") \
        if type(outputs) is dict else None
    if type(all_outputs) is not dict:
        raise CollectError("preserve: all_outputs object required")
    if len(fresh) != 1:
        raise CollectError("preserve: fresh backend ELF is not unique "
                           f"({len(fresh)} fresh candidates)")
    matches = []
    for entry in fresh:
        if all_outputs.get(entry["relative"]) != entry["sha256"]:
            raise CollectError("preserve: fresh entry absent from "
                               "all_outputs (stale/seed refused)")
        source = target / entry["relative"]
        ref = _file_ref("preserve-candidate", str(source))
        if ref["sha256"] == entry["sha256"] \
                and ref["bytes"] == entry["bytes"]:
            matches.append((entry, str(source)))
    if len(matches) != 1:
        raise CollectError("preserve: fresh backend ELF is not unique "
                           f"({len(matches)} live matches)")
    entry, source = matches[0]
    live_packet = {}
    for role, name in PACKET_FILES.items():
        live_packet[role] = _file_ref(role, str(packet / name))
    retained_dir = Path(selection["outer"]["retainedDir"])
    retained_dir.mkdir(parents=False, exist_ok=False)
    retained = retained_dir / "backend-elf"
    src = _copy_preserve(source, retained)
    record = {"schema": "doctorwatson.elf-preservation/1",
              "actualRelative": entry["relative"], "sourcePath": source,
              "retainedPath": str(retained), "bytes": src["bytes"],
              "sha256": src["sha256"], "executed": False,
              "livePacket": live_packet}
    _write_json_atomic(attempt / "preservation.json", record)
    return record


def _verify_evidence(selection: dict, preservation: dict) -> None:
    """Evidence packet copies must equal the live bytes hashed at
    preservation, and the operator manifest must bind them."""
    evidence = Path(selection["requestFacts"]["paths"]["evidence_dir"])
    manifest_raw = _read_raw(str(evidence
                                  / "EVIDENCE_SHA256_MANIFEST.json"),
                              "evidence manifest")
    try:
        manifest = json.loads(manifest_raw.decode(),
                              object_pairs_hook=_pairs,
                              parse_constant=_constant,
                              parse_float=_float)
    except Exception as exc:
        raise CollectError(f"evidence manifest: invalid JSON: {exc}") \
            from exc
    outer = selection["outer"]
    if type(manifest) is not dict \
            or manifest.get("request_sha256") \
            != outer["operatorRequest"]["sha256"] \
            or manifest.get("provisioner_sha256") \
            != outer["operator"]["sha256"]:
        raise CollectError("evidence manifest provenance mismatch")
    files = manifest.get("files")
    if type(files) is not dict or "ONE_SHOT_STARTED" not in files:
        raise CollectError("evidence manifest missing ONE_SHOT_STARTED")
    for role, name in PACKET_FILES.items():
        copied = _file_ref(role, str(evidence / "packet" / name))
        live = preservation["livePacket"][role]
        if copied["sha256"] != live["sha256"] \
                or copied["bytes"] != live["bytes"] \
                or files.get("packet/" + name) != live["sha256"]:
            raise CollectError(f"evidence copy != live packet: {role}")


def _validate_counters(path: str | None) -> bool:
    if path is None:
        return False
    try:
        value = json.loads(_read_raw(path, "counters").decode(),
                           object_pairs_hook=_pairs,
                           parse_constant=_constant, parse_float=_float)
    except Exception:
        return False
    if type(value) is not dict or not _is_int(value.get("monotonic_ns")):
        return False
    nodes = value.get("nodes")
    if type(nodes) is not dict or set(nodes) != {"E", "S", "P"}:
        return False
    return all(type(n) is dict and type(n.get("raw")) is dict
               and set(n["raw"]) == {"cpu.stat", "memory.events",
                                     "pids.events", "cgroup.procs",
                                     "cgroup.events"}
               for n in nodes.values())


HANDKEY_SELECTION_SCHEMA = "doctorwatson.handkey-selection/1"
HANDKEY_PROFILE = {"wallSeconds": 30, "cpuSeconds": 25,
                   "addressSpaceBytes": 2 * 1024**3,
                   "fileBytes": 64 * 1024**2, "coreBytes": 0,
                   "streamBytes": 4 * 1024**2}


def _resolve_handkey_selection(sel: dict, task: dict) -> dict:
    keys = {"schema", "task", "adapter", "attempt", "fingerprint",
            "tool", "key", "proof", "statement", "outputRoot"}
    if set(sel) != keys:
        raise BadInput("handkey selection: exact fixed fields required; no argv/env/hooks/caps")
    if task["adapter"] != "handkey-verify-pis":
        raise Blocked("handkey selection adapter mismatch")
    if (sel["task"] != task["id"] or sel["adapter"] != task["adapter"]
            or sel["attempt"] != task["attempt"]
            or sel["fingerprint"] != task["fingerprint"]):
        raise Blocked("handkey selection task/attempt/fingerprint mismatch")
    if not adapter_capability(task["adapter"])["executor_available"]:
        raise Blocked("handkey executor unavailable")
    pins = {k: _verify_pinned(sel[k], "selection." + k)
            for k in ("tool", "key", "proof", "statement", "outputRoot")}
    obj = task["task_obj"]
    bound = {(r.path, r.bytes, r.sha256) for r in obj.input_files}
    identity = json.loads(obj.identity_json)
    tools = {(r["path"], r["bytes"], r["sha256"]) for r in identity["tools"]}
    configs = {(r["path"], r["bytes"], r["sha256"]) for r in identity["config"]}
    for name, pin in pins.items():
        triple = (pin["path"], pin["bytes"], pin["sha256"])
        if triple not in bound or (name == "tool" and triple not in tools) \
                or (name == "outputRoot" and triple not in configs):
            raise Blocked(f"selection.{name}: not bound in plan identity")
    for name in ("key", "proof", "statement"):
        if pins[name]["sha256"] != task["inputs"][name + "_sha256"]:
            raise Blocked(f"selection.{name}: task HANDKEY_PINS mismatch")
    if not os.access(pins["tool"]["path"], os.X_OK):
        raise Blocked("handkey tool is not executable")
    root_raw = _read_raw(pins["outputRoot"]["path"], "handkey output root")
    if (len(root_raw) != pins["outputRoot"]["bytes"] or
            hashlib.sha256(root_raw).hexdigest() != pins["outputRoot"]["sha256"]):
        raise Blocked("handkey outputRoot changed before parsing pinned bytes")
    try:
        root = json.loads(root_raw.decode(), object_pairs_hook=_pairs,
                          parse_constant=_constant, parse_float=_float)
    except (ValueError, UnicodeError) as exc:
        raise BadInput(f"handkey outputRoot: invalid JSON: {exc}") from exc
    if type(root) is not dict or set(root) != {"schema", "path"} \
            or root["schema"] != "doctorwatson.handkey-output-root/1":
        raise BadInput("handkey outputRoot: exact versioned root descriptor required")
    output_root = _abs_norm(root["path"], "handkey outputRoot.path")
    if not Path(output_root).is_dir() or Path(output_root).resolve() != Path(output_root):
        raise Blocked("handkey output root must be an existing real directory")
    return {**sel, **pins, "outputRootPath": output_root,
            "selectionSha256": _digest(sel)}


def _handkey_limits():
    os.setsid()
    for kind, limit in ((resource.RLIMIT_CPU, HANDKEY_PROFILE["cpuSeconds"]),
                        (resource.RLIMIT_AS, HANDKEY_PROFILE["addressSpaceBytes"]),
                        (resource.RLIMIT_FSIZE, HANDKEY_PROFILE["fileBytes"]),
                        (resource.RLIMIT_CORE, 0)):
        resource.setrlimit(kind, (limit, limit))


def _write_handkey_json(path: Path, value) -> dict:
    ref = _write_json_atomic(path, value)
    _fsync_dir(path.parent)
    return ref


def _wait4_retry(pid, options):
    while True:
        try:
            return os.wait4(pid, options)
        except InterruptedError:
            continue


def _kill_adopted_handkey_children():
    children = Path(f"/proc/self/task/{os.getpid()}/children").read_text().split()
    for child in children:
        try:
            os.kill(int(child), signal.SIGKILL)
        except ProcessLookupError:
            pass


def _capture_handkey_group(stage: dict) -> dict:
    """Fixed ordinary capture, called only in an isolated child subreaper.

    The supervisor owns only this operation's tree. Linux __WALL includes
    non-SIGCHLD clone children in the final ECHILD observation. Per-process
    inherited rlimits do not impose aggregate CPU or memory limits.
    """
    started = time.monotonic()
    streams = selectors.DefaultSelector()
    outputs = {}
    counts = {"stdout": 0, "stderr": 0}
    process = None
    status = usage = None
    reason = None
    killed = False
    drained = False
    adopted = []
    wait_all = os.WNOHANG | 0x40000000  # Linux __WALL
    try:
        # Refuse output ownership problems before launching any verifier.
        for name in counts:
            outputs[name] = open(stage[name], "xb")
        process = subprocess.Popen(stage["argv"], cwd=stage["cwd"],
                                   env=stage["env"], stdin=subprocess.DEVNULL,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   shell=False, preexec_fn=_handkey_limits)
        _write_handkey_json(Path(stage["cwd"]) / "launched.json", {
            "pid": process.pid, "processGroup": process.pid,
            "supervisorPid": os.getpid()})
        for name in counts:
            pipe = getattr(process, name)
            os.set_blocking(pipe.fileno(), False)
            streams.register(pipe, selectors.EVENT_READ, name)
        while status is None or streams.get_map():
            if reason is None and time.monotonic() - started >= HANDKEY_PROFILE["wallSeconds"]:
                reason = "wall-timeout"
            if reason is not None and not killed:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                killed = True
            if status is None:
                waited, raw_status, ru = _wait4_retry(process.pid, wait_all)
                if waited:
                    status, usage = raw_status, ru
                    process.returncode = os.waitstatus_to_exitcode(status)
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
            if status is not None:
                _kill_adopted_handkey_children()
            for key, _ in streams.select(0.02):
                chunk = os.read(key.fileobj.fileno(), 65536)
                if not chunk:
                    streams.unregister(key.fileobj)
                    key.fileobj.close()
                    continue
                name = key.data
                remaining = HANDKEY_PROFILE["streamBytes"] - counts[name]
                outputs[name].write(chunk[:max(0, remaining)])
                counts[name] += len(chunk)
                if counts[name] > HANDKEY_PROFILE["streamBytes"]:
                    reason = "stream-limit"
            if time.monotonic() - started > HANDKEY_PROFILE["wallSeconds"] + 2:
                reason = reason or "unresolved-descendant-streams"
                break
        drain_deadline = min(started + HANDKEY_PROFILE["wallSeconds"] + 2,
                             time.monotonic() + 1)
        while status is not None and time.monotonic() < drain_deadline:
            try:
                pid, child_status, child_usage = _wait4_retry(-1, wait_all)
            except ChildProcessError:
                drained = True
                break
            if pid:
                adopted.append({"pid": pid, "returncode": os.waitstatus_to_exitcode(child_status),
                                "userSeconds": child_usage.ru_utime,
                                "systemSeconds": child_usage.ru_stime,
                                "maxRssKiB": child_usage.ru_maxrss})
                continue
            _kill_adopted_handkey_children()
            time.sleep(0.01)
        group_absent = False
        try:
            os.killpg(process.pid, 0)
        except ProcessLookupError:
            group_absent = True
        cleanup = "complete" if drained and group_absent else "unknown"
        return {"returncode": process.returncode, "reason": reason,
                "pid": process.pid, "processGroup": process.pid,
                "groupAbsentObserved": group_absent,
                "subreaperDrainECHILD": drained, "adoptedChildren": adopted,
                "descendantsContained": drained, "cleanup": cleanup,
                "wallSeconds": time.monotonic() - started,
                "usage": None if usage is None else {
                    "userSeconds": usage.ru_utime, "systemSeconds": usage.ru_stime,
                    "maxRssKiB": usage.ru_maxrss},
                "streamBytesObserved": counts,
                "usageScope": "direct child wait4; adopted rows may include descendant accounting, never summed",
                "limitScope": "per-process rlimits inherited by descendants; no aggregate CPU/memory limit",
                "supervisorOverheadUsage": None}
    except BaseException as exc:
        identity = "not launched" if process is None else f"pid={process.pid} pgid={process.pid}"
        raise StageUnknown(f"capture failure ({identity}): {type(exc).__name__}: {exc}") from exc
    finally:
        # Any I/O/controller failure still attempts to stop this owned tree;
        # only the normal observed drain above may certify cleanup complete.
        if process is not None and not drained:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            stop = time.monotonic() + 1
            while time.monotonic() < stop:
                try:
                    _kill_adopted_handkey_children()
                    pid, _, _ = _wait4_retry(-1, wait_all)
                except ChildProcessError:
                    break
                if not pid:
                    time.sleep(0.01)
        for key in list(streams.get_map().values()):
            key.fileobj.close()
        streams.close()
        if process is not None:
            for name in counts:
                pipe = getattr(process, name)
                if pipe is not None:
                    pipe.close()
        for stream in outputs.values():
            try:
                stream.flush()
                os.fsync(stream.fileno())
            finally:
                stream.close()


def _enable_handkey_subreaper():
    # Linux PR_SET_CHILD_SUBREAPER. Never called in the root/controller.
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(36, 1, 0, 0, 0) != 0:
        raise OSError(ctypes.get_errno(), "PR_SET_CHILD_SUBREAPER failed")


def _capture_handkey(stage: dict) -> dict:
    """Run the fixed capture in a fresh, isolated child subreaper.

    The supervisor owns no unrelated children. Only its kernel ECHILD after
    reaping the direct child and every adopted descendant can prove cleanup.
    Failure before prctl never launches the verifier. Parent deadline failure
    remains unknown, with the supervisor identity retained for reconciliation.
    """
    receipt = Path(stage["cwd"]) / "supervisor.json"
    supervisor = os.fork()
    if supervisor == 0:
        try:
            _enable_handkey_subreaper()
            value = _capture_handkey_group(stage)
        except BaseException as exc:
            value = {"returncode": None, "cleanup": "unknown",
                     "reason": f"supervisor failure: {type(exc).__name__}: {exc}",
                     "launchStatus": "unknown; supervisor failed"}
            launch = Path(stage["cwd"]) / "launched.json"
            try:
                value["launchIdentity"] = _load_json_file(str(launch), "launch identity")
            except Exception:
                value["launchIdentity"] = None
        try:
            _write_handkey_json(receipt, value)
        finally:
            os._exit(0)
    deadline = time.monotonic() + HANDKEY_PROFILE["wallSeconds"] + 4
    while time.monotonic() < deadline:
        pid, status = os.waitpid(supervisor, os.WNOHANG)
        if pid:
            if os.waitstatus_to_exitcode(status) != 0:
                raise StageUnknown(f"handkey supervisor {supervisor} failed")
            observed = _load_json_file(str(receipt), "supervisor result")
            return {**observed, "supervisorPid": supervisor,
                    "supervisorReaped": True}
        time.sleep(0.01)
    # Do not turn supervisor timeout into a fictitious cleanup success.
    raise StageUnknown(f"handkey supervisor {supervisor} termination/descendants unknown")


def _reserve_handkey_attempt(task, history_path):
    """Atomically reserve one attempt against the complete history journal.

    A durable reservation survives deletion of the caller's output directory.
    Prior attempts must be present with their exact saved Result in supplied
    history; common readiness retains its original reconciliation/debt guards.
    An incomplete/crashed reservation blocks until explicitly recovered.
    """
    history = Path(_abs_norm(history_path, "history"))
    if history.parent.resolve() != history.parent:
        raise Blocked("handkey history parent must be a real directory")
    reservations = history.parent / (history.name + ".attempts")
    reservations.mkdir(exist_ok=True)
    _fsync_dir(history.parent)
    if reservations.is_symlink() or not reservations.is_dir():
        raise Blocked("history reservation namespace is not a real directory")
    lock_fd = os.open(reservations / "LOCK", os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX)
        prior_results = task["history_results"]
        token = _digest({"task": task["id"], "attempt": task["attempt"]})
        reservation = reservations / token
        if reservation.exists():
            raise Blocked("task attempt already reserved; reconcile original, no retry")
        entries = list(reservations.iterdir())
        if len(entries) > 4097:
            raise Blocked("history reservation journal exceeds bounded inspection")
        for sibling in entries:
            if sibling.name == "LOCK":
                continue
            if sibling.is_symlink() or not sibling.is_dir() or not HEX.fullmatch(sibling.name):
                raise Blocked("unrecognized history reservation entry")
            try:
                original = _load_json_file(str(sibling / "reservation.json"), "prior reservation")
                if original["task"] != task["id"]:
                    continue
                saved = delivery.load_result(_load_json_file(
                    str(sibling / "history-result.json"), "prior attempt result"))
                if not any(reconciliation.result_digest(r) == reconciliation.result_digest(saved)
                           for r in prior_results):
                    raise Blocked("authoritative history omitted or changed a reserved original attempt")
            except (KeyError, BadInput, CollectError, delivery.SchemaError) as exc:
                raise Blocked("prior reservation incomplete; reconcile original attempt") from exc
        reservation.mkdir()
        _write_handkey_json(reservation / "reservation.json", {
            "task": task["id"], "attempt": task["attempt"],
            "fingerprint": task["fingerprint"], "history": str(history)})
        _fsync_dir(reservations)
        return reservation
    finally:
        os.close(lock_fd)


def _run_handkey_once(task, selection, plan_path, selection_path, attempt_dir,
                      history_path, reconciliations_path):
    attempt = Path(_abs_norm(attempt_dir, "attempt"))
    if attempt.parent != Path(selection["outputRootPath"]):
        raise Blocked("handkey attempt must be a direct child of the identity-bound output root")
    reservation = _reserve_handkey_attempt(task, history_path)
    try:
        attempt.mkdir(exist_ok=False)
        _fsync_dir(attempt.parent)
    except FileExistsError as exc:
        raise Blocked("attempt directory already consumed; no retry") from exc
    argv = [selection["tool"]["path"], "verify-pis"] + [
        selection[k]["path"] for k in ("key", "proof", "statement")]
    stage = {"stage": "handkey-verify-pis", "argv": argv,
             "cwd": str(attempt), "env": {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"},
             "stdout": str(attempt / "stdout"), "stderr": str(attempt / "stderr"),
             "profile": HANDKEY_PROFILE}
    preexec = {"schema": "doctorwatson.handkey-preexec/1",
               "task": task["id"], "attempt": task["attempt"],
               "adapter": task["adapter"], "fingerprint": task["fingerprint"],
               "readinessIdentity": task["readiness_identity"],
               "selection": selection, "stage": stage,
               "scope": "ordinary local verification protocol; no native resource grant"}
    preexec_ref = _write_handkey_json(attempt / "PREEXEC.json", preexec)
    _write_handkey_json(reservation / "PREEXEC.json", {"preexec": preexec_ref})
    task, selection = _recheck_at_use(task, selection, plan_path, selection_path,
                                     history_path, reconciliations_path)
    base = {**preexec, "schema": "doctorwatson.handkey-execution-result/1",
            "preexec": preexec_ref, "consumerResult": None,
            "cleanupState": "unknown"}
    result = None
    observed = None
    try:
        observed = _capture_handkey(stage)
        base["cleanupState"] = observed["cleanup"]
        _write_handkey_json(attempt / "resources.json", observed)
        _write_handkey_json(attempt / "cleanup.json", {
            "schema": "doctorwatson.cleanup-observation/1", "task": task["id"],
            "attempt": task["attempt"], "fingerprint": task["fingerprint"],
            "cleanup": observed["cleanup"], "restoration": observed["cleanup"]})
        raw = [{"role": name, "path": stage[name]} for name in ("stdout", "stderr")]
        raw += [{"role": "resource-receipt", "path": str(attempt / "cleanup.json")}]
        raw += [{"role": name, "path": selection[name]["path"]}
                for name in ("key", "proof", "statement")]
        record = {**base, "observation": observed, "rawOutputs": raw,
                  "terminal": "unknown-submission", "unknownScope": "local-process"}
        raw_ref = _write_handkey_json(attempt / "execution.json", record)
        rc = observed["returncode"]
        # Decode actual full output only after the supervisor proves cleanup;
        # no unknown containment record can become a successful consumer Result.
        result = collect_result(task, raw, rc, attempt, raw_ref) if rc is not None \
            and observed["reason"] is None and observed["cleanup"] == "complete" else None
        if observed["cleanup"] != "complete":
            (attempt / "consumer-result.json").unlink(missing_ok=True)
            (attempt / "collector.json").unlink(missing_ok=True)
            record["collectorDetail"] = "descendant cleanup not proven by supervisor drain"
            code = EXIT_UNKNOWN
        elif result is not None:
            record.update(terminal=result["terminal"], unknownScope=None,
                          consumerResult=_file_ref("consumer-result", str(attempt / "consumer-result.json")))
            code = EXIT_OK if result["terminal"] == "succeeded" else EXIT_FAILED
        else:
            record.update(terminal="failed", unknownScope=None,
                          collectorDetail=observed["reason"] or "verifier result unavailable")
            code = EXIT_FAILED
    except CollectError as exc:
        record = {**record, "terminal": "failed", "unknownScope": None, "collectorDetail": str(exc)}
        code = EXIT_FAILED
    except Exception as exc:
        record = {**base, "terminal": "unknown-submission", "unknownScope": "local-process",
                  "collectorDetail": f"capture/record failure: {type(exc).__name__}: {exc}"}
        code = EXIT_UNKNOWN
    # Save the exact original history Result separately from mutable plan
    # selection; later attempts must retain it, including all uncertainty/debt.
    if result is None or record["terminal"] == "unknown-submission":
        returncode = None if observed is None else observed.get("returncode")
        _, exit_code, child_signal = _terminal_fields(returncode) if returncode is not None else (None, None, None)
        artifacts = []
        try:
            artifacts.append(_file_ref("resource-receipt", str(attempt / "cleanup.json")))
        except CollectError:
            pass  # No receipt was observed; existing cleanup-debt rules apply.
        result = _result_dict(task, record["terminal"], exit_code, child_signal,
                              record["cleanupState"], "not-applicable", artifacts)
    try:
        history_ref = _write_handkey_json(reservation / "history-result.json", result)
        record["historyResult"] = history_ref
        _write_handkey_json(attempt / "result-final.json", record)
        _write_handkey_json(reservation / "result-final.json", record)
    except OSError as exc:
        # The reservation and PREEXEC remain consumed. Preserve the actual
        # process/cleanup observations, but never report durable completion
        # when a terminal record could not be persisted. No retry is selected.
        record = {**record, "observedCollectorTerminal": record["terminal"],
                  "observedConsumerResult": record.get("consumerResult"),
                  "terminal": "unknown-submission", "unknownScope": "local-record",
                  "consumerResult": None,
                  "persistenceDetail": f"terminal record persistence failed: {type(exc).__name__}: {exc}"}
        for path in (attempt / "result-persistence-unknown.json",
                     reservation / "result-persistence-unknown.json"):
            try:
                _write_handkey_json(path, record)
            except OSError:
                pass
        return EXIT_UNKNOWN, record
    return code, record


def run_selected_once(task_id: str, plan_path: str, selection_path: str,
                      attempt_dir: str,
                      runner: Callable[[dict], int] | None = None,
                      history_path: str | None = None,
                      reconciliations_path: str | None = None
                      ) -> tuple[int, dict]:
    """Dispatch one selected outer operation at most once.

    PREEXEC/4 (consumer fingerprint, selection hash, verified
    guard/config/outer pins, the full code-fixed stage table, declared
    retained outputs, and the pinned readinessIdentity from the common
    load_readiness path) is fsynced before the first stage, and that
    identity is rechecked at actual use immediately before dispatch.
    Stages then run in fixed provisioned-host order (OP0..OP6 are
    external prerequisites, never staged or dispatched here), each at
    most once, each recorded in the stage ledger with its actual
    outcome:

      prelaunch-verify -> counters-before -> native (only child in S,
      via pinned capture) -> counters-after (before any removal) ->
      [marker present] preserve fresh ELF durably -> collect-evidence
      -> teardown, or [marker absent after native]
      abort-prelaunch-cleanup.

    If prelaunch-verify or counters-before fails/is unknown before
    native is ever dispatched, the adapter does NOT automatically invoke
    abort-prelaunch-cleanup or teardown on the preexisting provisioned
    host; it records the raw failure/unknown in the attempt history with
    cleanupState 'owed'/'unknown' so Root can inspect or explicitly
    abort.

    A typed cleanup-observation receipt is issued from those actual
    outcomes (complete only when preservation/evidence, counters and
    teardown/abort all actually succeeded and the cgroup envelope and
    all 3 request mountpoints are observed absent); only then, and only
    if the native stage terminally exited 0 with preservation and
    evidence verified, does the fixed collector produce the consumer
    Result from retained outputs. Any stage failure/unknown is recorded,
    never retried, never promoted: the raw receipt is
    doctorwatson.execution-result/4 with cleanupState and the stage
    ledger. Root wraps this whole controller lifetime in the pinned
    /usr/bin/time -v -o <timeOutput>; this process records its own
    monotonic/RUSAGE brackets through cleanup and states the excluded
    tails (time's own output write, sudo/orphan CPU unknown).
    """
    if history_path is None:
        raise BadInput("execution requires authoritative complete --history")
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
    if task["adapter"] == "handkey-verify-pis":
        return _run_handkey_once(task, selection, plan_path, selection_path,
                                 attempt_dir, history_path, reconciliations_path)
    outer = selection["outer"]
    _require_existing_provisioned_host(selection)
    for stale, what in ((outer["retainedDir"], "retainedDir"),
                        (selection["requestFacts"]["paths"]
                         ["evidence_dir"], "evidence_dir")):
        if os.path.lexists(stale):
            raise Blocked(f"{what} already exists at selection "
                          "(stale/consumed); zero dispatch")
    time_output_state = _check_time_output_preexec(outer["timeOutput"])
    attempt = Path(attempt_dir)
    try:
        attempt.mkdir(parents=False, exist_ok=False)
        (attempt / "streams").mkdir()
    except FileExistsError as exc:
        raise Blocked("attempt directory already exists (consumed/stale/"
                      "unknown); no retry, zero dispatch") from exc
    except OSError as exc:
        raise BadInput(f"attempt directory unusable: {exc}") from exc
    stages = build_outer_stages(selection, attempt)
    stage_order = [s for s, _ in PROVISION_STEPS] + [
        "counters-before", "native", "counters-after",
        "collect-evidence", "teardown", "abort-prelaunch-cleanup"]
    preexec = {
        "schema": "doctorwatson.preexec/4", "task": task["id"],
        "adapter": task["adapter"], "attempt": task["attempt"],
        "fingerprint": task["fingerprint"],
        "selectionSha256": selection["selectionSha256"],
        "guard": selection["guard"], "config": selection["config"],
        "outer": outer, "outerSequence": selection["outerSequence"],
        "configFacts": selection["configFacts"],
        "requestFacts": selection["requestFacts"],
        "guardConstants": selection["guardConstants"],
        "helperConstants": selection["helperConstants"],
        "stages": [stages[name] for name in stage_order],
        "declaredOutputs": selection["outputs"],
        "readinessIdentity": task["readiness_identity"],
        "outerTimeWrapper": {
            "tool": outer["timeTool"], "output": outer["timeOutput"],
            "timeOutputStateAtPreexec": time_output_state,
            "argvPrefix": [outer["timeTool"]["path"], "-v", "-o",
                           outer["timeOutput"], "--"],
            "scope": "Root wraps this entire controller lifetime "
                     "(native+preserve+collect+cleanup+collector); "
                     "time's own output-writing tail and sudo/orphan "
                     "CPU are outside/unknown, never asserted"},
    }
    try:
        preexec_ref = _write_json_atomic(attempt / "PREEXEC.json", preexec)
    except OSError as exc:
        raise BadInput(f"PREEXEC write failed: {exc}") from exc
    base = {**preexec, "schema": "doctorwatson.execution-result/4",
            "preexec": preexec_ref}
    try:
        task, selection = _recheck_at_use(task, selection, plan_path,
                                          selection_path, history_path,
                                          reconciliations_path)
        _require_existing_provisioned_host(selection)
        _check_time_output_preexec(outer["timeOutput"])
        if build_outer_stages(selection, attempt) != stages:
            raise Blocked("resolved stages changed at actual-use "
                          "recheck")
    except (BadInput, Blocked) as exc:
        _write_json_atomic(attempt / "result-blocked.json",
                           {**base, "terminal": "refused",
                            "exit_code": None, "signal": None,
                            "unknownScope": None,
                            "collectorDetail": f"actual-use recheck: {exc}",
                            "cleanupState": "unknown",
                            "outerStages": [],
                            "consumerResult": None})
        raise

    ledger: list[dict] = []

    def run_stage(name: str) -> dict:
        stage = stages[name]
        try:
            rc = runner(stage)
        except Exception as exc:
            entry = {"stage": name, "argv": stage["argv"],
                     "exit": None, "outcome": "unknown",
                     "detail": f"{type(exc).__name__}: {exc}"}
            ledger.append(entry)
            return entry
        if type(rc) is not int or isinstance(rc, bool):
            entry = {"stage": name, "argv": stage["argv"],
                     "exit": None, "outcome": "unknown",
                     "detail": "runner returned non-integer terminal"}
            ledger.append(entry)
            return entry
        entry = {"stage": name, "argv": stage["argv"], "exit": rc,
                 "outcome": "ok" if rc == 0 else "failed",
                 "detail": None}
        ledger.append(entry)
        return entry

    def outcome(name: str):
        for entry in ledger:
            if entry["stage"] == name:
                return entry["outcome"]
        return None

    obs_start = _rusage_snapshot()
    provision_ok = True
    for step, _sudo in PROVISION_STEPS:
        if run_stage(step)["outcome"] != "ok":
            provision_ok = False
            break
    native_entry = None
    counters_before_ok = False
    if provision_ok:
        entry = run_stage("counters-before")
        counters_before_ok = entry["outcome"] == "ok" and \
            _validate_counters(stages["counters-before"]["stdout"])
        if entry["outcome"] == "ok" and not counters_before_ok:
            entry["outcome"] = "failed"
            entry["detail"] = "counters-before output malformed"
    if provision_ok and counters_before_ok:
        native_entry = run_stage("native")
    counters_after_ok = False
    marker = False
    preservation = None
    preservation_detail = None
    cleanup_stage = None
    evidence_ok = False
    restoration_probe = None
    if native_entry is not None:
        entry = run_stage("counters-after")
        counters_after_ok = entry["outcome"] == "ok" and \
            _validate_counters(stages["counters-after"]["stdout"])
        if entry["outcome"] == "ok" and not counters_after_ok:
            entry["outcome"] = "failed"
            entry["detail"] = "counters-after output malformed"
        marker = _marker_present(selection)
        if marker:
            try:
                preservation = _preserve_elf(selection, attempt)
            except (CollectError, OSError) as exc:
                preservation_detail = str(exc)
            run_stage("collect-evidence")
            run_stage("teardown")
            cleanup_stage = "teardown"
        else:
            run_stage("abort-prelaunch-cleanup")
            cleanup_stage = "abort-prelaunch-cleanup"
        if marker and preservation is not None \
                and outcome("collect-evidence") == "ok":
            try:
                _verify_evidence(selection, preservation)
                evidence_ok = True
            except CollectError as exc:
                preservation_detail = \
                    (preservation_detail + "; " if preservation_detail
                     else "") + str(exc)
        cleanup_outcome = outcome(cleanup_stage)
        try:
            restoration_probe = _restoration_probe(selection)
            restored = bool(restoration_probe["restored"])
        except Exception as exc:
            restoration_probe = {"restored": False,
                                 "probeError": f"{type(exc).__name__}: {exc}"}
            restored = False
        if cleanup_outcome == "ok" and restored:
            restoration_state = "complete"
        elif cleanup_outcome == "failed":
            restoration_state = "owed"
        else:
            restoration_state = "unknown"
        if marker:
            cleanup_complete = (
                preservation is not None and evidence_ok
                and counters_after_ok and cleanup_outcome == "ok"
                and restoration_state == "complete")
            cleanup_known_failed = (
                preservation is None or not evidence_ok
                or cleanup_outcome == "failed")
        else:
            cleanup_complete = (
                cleanup_outcome == "ok"
                and restoration_state == "complete")
            cleanup_known_failed = cleanup_outcome == "failed"
        cleanup_state = "complete" if cleanup_complete else \
            ("owed" if cleanup_known_failed else "unknown")
    else:
        # Prelaunch refusal before native/factory host mutation: do NOT
        # automatically abort or tear down the preexisting provisioned
        # host; retain the raw failed attempt history with owed/unknown
        # cleanup state so Root can inspect or explicitly abort.
        first_prelaunch_bad = next(
            (e for e in ledger if e["outcome"] != "ok"), None)
        if first_prelaunch_bad and first_prelaunch_bad["outcome"] == "failed":
            restoration_state = "owed"
            cleanup_state = "owed"
        else:
            restoration_state = "unknown"
            cleanup_state = "unknown"
    retained_dir = Path(outer["retainedDir"])
    cleanup_write_error = None
    try:
        retained_dir.mkdir(parents=False, exist_ok=True)
        receipt_path = retained_dir / "cleanup.json"
        _write_json_atomic(receipt_path, {
            "schema": "doctorwatson.cleanup-observation/1",
            "task": task["id"], "attempt": task["attempt"],
            "fingerprint": task["fingerprint"],
            "cleanup": cleanup_state, "restoration": restoration_state})
        _fsync_dir(retained_dir)
    except OSError as exc:
        cleanup_write_error = f"{type(exc).__name__}: {exc}"
        cleanup_state = "unknown"
        restoration_state = "unknown"
    obs_end = _rusage_snapshot()
    obs_doc = {
        "schema": "doctorwatson.outer-observations/1",
        "start": obs_start, "endAfterCleanup": obs_end,
        "countersBefore": stages["counters-before"]["stdout"]
        if counters_before_ok else None,
        "countersAfter": stages["counters-after"]["stdout"]
        if counters_after_ok else None,
        "restorationProbe": restoration_probe,
        "scope": {
            "rusageChildren": "waited-for children of this controller "
                              "only; sudo-prefix/orphan completeness "
                              "not asserted",
            "sudoPrefixCpuUs": None, "orphanCpuUs": None,
            "exactAllDescendantCpuUs": None,
            "cgroupESP": "separate raw scopes in counters files; "
                         "never summed with rusage children",
            "excludedTail": "collector + record writes after "
                            "endAfterCleanup, and the outer time "
                            "process's own output write"}}
    stages_doc = {
        "schema": "doctorwatson.outer-stages/1", "stages": ledger,
        "preservationDetail": preservation_detail,
        "markerPresent": marker,
        "restorationProbe": restoration_probe}
    ledger_ref = None
    try:
        _write_json_atomic(attempt / "outer-observations.json", obs_doc)
        ledger_ref = _write_json_atomic(
            attempt / "outer-stages.json", stages_doc)
        if cleanup_write_error is not None:
            record = {**base, "terminal": "unknown-submission",
                      "exit_code": None, "signal": None,
                      "unknownScope": "local-process",
                      "collectorDetail":
                          "post-dispatch cleanup receipt write failed: "
                          + cleanup_write_error,
                      "cleanupState": "unknown",
                      "outerStages": ledger_ref,
                      "consumerResult": None}
            _write_json_atomic(attempt / "result.json", record)
            return EXIT_UNKNOWN, record
        native_unknown = native_entry is not None \
            and native_entry["outcome"] == "unknown"
        if native_entry is None:
            first_bad = next((e for e in ledger
                              if e["outcome"] != "ok"), None)
            rc_for_terminal = first_bad["exit"] if first_bad else None
            detail = ("native never dispatched: "
                      + (f"{first_bad['stage']} {first_bad['outcome']}"
                         if first_bad else "no stage ran"))
        else:
            rc_for_terminal = native_entry["exit"]
            detail = native_entry["detail"]
        if native_unknown or rc_for_terminal is None:
            record = {**base, "terminal": "unknown-submission",
                      "exit_code": None, "signal": None,
                      "unknownScope": "local-process",
                      "collectorDetail": detail,
                      "cleanupState": cleanup_state,
                      "outerStages": ledger_ref,
                      "consumerResult": None}
            _write_json_atomic(attempt / "result.json", record)
            return EXIT_UNKNOWN, record
        terminal, exit_code, signal = _terminal_fields(rc_for_terminal)
        raw_record = {**base, "terminal": terminal,
                      "exit_code": exit_code, "signal": signal,
                      "unknownScope": None, "collectorDetail": detail,
                      "cleanupState": cleanup_state,
                      "outerStages": ledger_ref, "consumerResult": None}
        raw_ref = _write_json_atomic(attempt / "result.json", raw_record)
        collector_gate = (
            native_entry is not None and native_entry["outcome"] == "ok"
            and preservation is not None and evidence_ok)
        if not collector_gate:
            gate_detail = (
                "collector not run: "
                f"native={native_entry['outcome'] if native_entry else 'not-run'} "
                f"preservation={'ok' if preservation else 'missing'} "
                f"evidence={'ok' if evidence_ok else 'unverified'}"
                + (f"; {preservation_detail}" if preservation_detail
                   else ""))
            raw_record = {**raw_record, "terminal": "failed",
                          "collectorDetail": gate_detail}
            _write_json_atomic(attempt / "result-collect-failed.json",
                               raw_record)
            return EXIT_FAILED, raw_record
        try:
            result = collect_result(task, selection["outputs"],
                                    rc_for_terminal, attempt, raw_ref,
                                    expected=_decoder_expected(selection),
                                    preservation=preservation)
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
        return (EXIT_OK if result["terminal"] == "succeeded"
                else EXIT_FAILED), raw_record
    except OSError as exc:
        try:
            (attempt / "consumer-result.json").unlink(missing_ok=True)
        except OSError:
            pass
        if ledger_ref is None:
            try:
                ledger_ref = _write_json_atomic(
                    attempt / "outer-stages.json", stages_doc)
            except OSError:
                ledger_ref = None
        record = {**base, "terminal": "unknown-submission",
                  "exit_code": None, "signal": None,
                  "unknownScope": "local-process",
                  "collectorDetail":
                      f"post-dispatch record write OSError: "
                      f"{type(exc).__name__}: {exc}",
                  "cleanupState": "unknown",
                  "outerStages": ledger_ref,
                  "consumerResult": None}
        try:
            (attempt / "result.json.tmp").unlink(missing_ok=True)
            _write_json_atomic(attempt / "result.json", record)
        except OSError:
            pass
        return EXIT_UNKNOWN, record


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    usage = ("usage: run_selected.py --execute TASK --plan PATH "
             "--selection PATH --attempt DIR --history PATH "
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
