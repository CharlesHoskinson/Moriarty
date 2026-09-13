"""Identity-safe cleanup helper for the loan executor and its independent timer.

Import performs no service, Docker, wallet, network or store operation. The
helper acts only on resources whose current identity exactly matches the
ownership receipts produced by startup observation: the financial unit by its
retained InvocationID and the prover container by exact ID plus StartedAt.
A reused name, a replacement generation, a changed InvocationID or a missing
receipt is retained as a mismatch and never killed as this invocation.

Cleanup is not exit proof: it records its distinct purpose and leaves the
main outcome to the collector. Its containment observations are what the
executor and runner treat as C (independently observed containment).
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

if __package__ in (None, ""):  # direct script execution adds only scripts/
    _SCRIPTS = Path(__file__).resolve().parents[1]
    if str(_SCRIPTS) not in sys.path:
        sys.path.insert(0, str(_SCRIPTS))

from moriarty_dev.loan_exit_retention.exit_retention import CommandResult, durable_exclusive_save
from moriarty_dev.loan_exit_retention.launch_contract import (
    SYSTEMCTL,
    require_bound_seconds,
    require_unit_name,
    timeout_prefix,
)

INTENT_SCHEMA = "moriarty.loan-cleanup-intent/1"
INTENT_KEYS = (
    "schema", "allocationId", "unit", "timerUnit", "containerId", "invocationSha256", "chargeId", "reservationId",
    "evidenceDirectory", "proverReceiptPath", "unitReceiptPath",
)
PROVER_RECEIPT_SCHEMA = "moriarty.loan-prover-ownership/1"
PROVER_RECEIPT_KEYS = (
    "schema", "allocationId", "invocationSha256", "chargeId", "reservationId", "containerId",
    "containerStartedAt", "startArgv", "startReturnCode", "observedAtUtc",
)
UNIT_RECEIPT_SCHEMA = "moriarty.loan-unit-ownership/1"
UNIT_RECEIPT_KEYS = (
    "schema", "allocationId", "invocationSha256", "chargeId", "reservationId", "unit",
    "unitInvocationId", "unitControlGroup", "unitMainPid", "observedAtUtc",
)
DOCKER = "/usr/bin/docker"
SYSTEMD_RUN = "/usr/bin/systemd-run"
MAX_INTENT_BYTES = 65536
DEFAULT_BOUND_SECONDS = 25
_TIMER_NAME = re.compile(r"[A-Za-z0-9][A-Za-z0-9:_.@-]*")
_CONTAINER_ID = re.compile(r"[0-9a-f]{64}")
_INVOCATION = re.compile(r"[0-9a-f]{32}")
_STARTED_AT = re.compile(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?Z")


def require_timer_name(value):
    if not isinstance(value, str) or _TIMER_NAME.fullmatch(value) is None or value.endswith((".service", ".timer")):
        raise ValueError("TIMER_UNIT_REQUIRED")
    return value


def _exact(record, keys, code):
    if type(record) is not dict or set(record) != set(keys):
        raise ValueError(code)
    return record


def _read_nofollow(path, limit):
    fd = os.open(str(path), os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        data = os.read(fd, limit + 1)
    finally:
        os.close(fd)
    if len(data) > limit:
        raise ValueError("RECORD_TOO_LARGE")
    return data


def load_intent(path, expected_sha256):
    """Immutable cleanup intent: exact bytes by digest, closed schema."""
    if not isinstance(expected_sha256, str) or re.fullmatch(r"[0-9a-f]{64}", expected_sha256) is None:
        raise ValueError("INTENT_DIGEST_REQUIRED")
    path = Path(path)
    if not path.is_absolute() or path.is_symlink() or not path.is_file():
        raise ValueError("INTENT_PATH_REQUIRED")
    data = _read_nofollow(path, MAX_INTENT_BYTES)
    if hashlib.sha256(data).hexdigest() != expected_sha256:
        raise ValueError("INTENT_DIGEST_MISMATCH")
    intent = json.loads(data.decode("utf-8"))
    return validate_intent(intent)


def validate_intent(intent):
    _exact(intent, INTENT_KEYS, "INTENT_FIELDS")
    if intent["schema"] != INTENT_SCHEMA:
        raise ValueError("INTENT_SCHEMA")
    require_unit_name(intent["unit"])
    require_timer_name(intent["timerUnit"])
    if _CONTAINER_ID.fullmatch(str(intent["containerId"])) is None:
        raise ValueError("INTENT_CONTAINER_ID")
    if re.fullmatch(r"[0-9a-f]{64}", str(intent["invocationSha256"])) is None:
        raise ValueError("INTENT_INVOCATION")
    for key in ("evidenceDirectory", "proverReceiptPath", "unitReceiptPath"):
        value = intent[key]
        if not isinstance(value, str) or not value.startswith("/"):
            raise ValueError("INTENT_PATH:" + key)
    if not isinstance(intent["allocationId"], str) or not intent["allocationId"]:
        raise ValueError("INTENT_ALLOCATION")
    for key in ("chargeId", "reservationId"):
        if not isinstance(intent[key], str) or not intent[key]:
            raise ValueError("INTENT_CLAIM:" + key)
    return intent


def receipt_matches_intent(record, keys, schema, intent):
    """Exact receipt identity against the immutable intent: claim, invocation, resource."""
    try:
        _exact(record, keys, "RECEIPT_FIELDS")
    except ValueError:
        return False
    if record["schema"] != schema or record["allocationId"] != intent["allocationId"] \
            or record["invocationSha256"] != intent["invocationSha256"] \
            or record["chargeId"] != intent["chargeId"] or record["reservationId"] != intent["reservationId"]:
        return False
    if schema == PROVER_RECEIPT_SCHEMA:
        return record["containerId"] == intent["containerId"] \
            and _STARTED_AT.fullmatch(str(record["containerStartedAt"])) is not None
    return record["unit"] == intent["unit"] and _INVOCATION.fullmatch(str(record["unitInvocationId"])) is not None \
        and isinstance(record["unitControlGroup"], str) and record["unitControlGroup"].startswith("/")


def load_receipt(path, keys, schema, intent):
    """Ownership receipt or None when absent/unreadable/mismatched (never guessed)."""
    path = Path(path)
    try:
        if path.is_symlink() or not path.is_file():
            return None
        record = json.loads(_read_nofollow(path, MAX_INTENT_BYTES).decode("utf-8"))
    except (OSError, ValueError):
        return None
    if not receipt_matches_intent(record, keys, schema, intent):
        return None
    return record


# --- argv builders (the retained argv is the argv invoked) --------------------

def docker_inspect_argv(container_id, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [DOCKER, "inspect", "--format",
                                    "{{.Id}}|{{.State.StartedAt}}|{{.State.Running}}|{{.State.Status}}", container_id]


def docker_kill_argv(container_id, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [DOCKER, "kill", container_id]


def unit_identity_argv(unit, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "show", require_unit_name(unit),
                                    "--property=LoadState,ActiveState,SubState,MainPID,ControlGroup,InvocationID"]


def unit_kill_argv(unit, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "kill", "--signal=SIGKILL", "--kill-whom=all",
                                    require_unit_name(unit)]


def unit_stop_argv(unit, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "stop", require_unit_name(unit)]


def timer_arm_argv(timer_unit, delay_seconds, python, script, intent_path, intent_sha256):
    require_timer_name(timer_unit)
    if type(delay_seconds) is not int or delay_seconds <= 0:
        raise ValueError("TIMER_DELAY_REQUIRED")
    return [SYSTEMD_RUN, "--user", "--unit=" + timer_unit, "--on-active=%ds" % delay_seconds,
            "--timer-property=AccuracySec=1s", "--", str(python), str(script),
            "--intent", str(intent_path), "--sha256", intent_sha256]


def timer_show_argv(timer_unit, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "show", require_timer_name(timer_unit) + ".timer",
                                    "--property=LoadState,ActiveState,SubState,NextElapseUSecMonotonic,Unit"]


def timer_cancel_argv(timer_unit, seconds):
    bound = require_bound_seconds(seconds, DEFAULT_BOUND_SECONDS)
    return timeout_prefix(bound) + [SYSTEMCTL, "--user", "stop", require_timer_name(timer_unit) + ".timer"]


# --- observations -----------------------------------------------------------

def _run(run_command, argv, seconds, log):
    try:
        result = run_command(argv, seconds)
    except Exception as error:  # noqa: BLE001 - the transport failure is evidence
        record = {"argv": list(argv), "returnCode": None, "errorClass": type(error).__name__,
                  "stdout": "", "stderr": str(error)[:2000], "timedOut": False}
        log.append(record)
        return None
    record = {"argv": list(getattr(result, "argv", argv)), "returnCode": getattr(result, "returncode", None),
              "errorClass": None, "stdout": (getattr(result, "stdout", "") or "")[:4096],
              "stderr": (getattr(result, "stderr", "") or "")[:2000],
              "timedOut": bool(getattr(result, "timed_out", False))}
    log.append(record)
    if record["timedOut"] or record["returnCode"] is None:
        return None
    return result


def parse_unit_show(stdout):
    fields = {}
    for line in (stdout or "").splitlines():
        if "=" not in line:
            return None
        key, value = line.split("=", 1)
        if key in fields:
            return None
        fields[key] = value
    return fields or None


def parse_container_inspect(stdout):
    parts = (stdout or "").strip().split("|")
    if len(parts) != 4 or _CONTAINER_ID.fullmatch(parts[0]) is None:
        return None
    if parts[2] not in ("true", "false"):
        return None
    return {"Id": parts[0], "StartedAt": parts[1], "Running": parts[2] == "true", "Status": parts[3]}


def _unit_contained(fields):
    if fields is None:
        return False
    if fields.get("LoadState") not in ("loaded", "not-found"):
        return False
    if fields.get("LoadState") == "not-found":
        return True
    return fields.get("ActiveState") in ("inactive", "failed") and fields.get("MainPID") == "0" \
        and fields.get("ControlGroup", "") == ""


class _Budget(object):
    """One absolute monotonic cutoff shared by every command and write."""

    def __init__(self, deadline_ns, monotonic_ns):
        self.deadline_ns = int(deadline_ns)
        self.monotonic_ns = monotonic_ns

    def remaining(self, maximum=DEFAULT_BOUND_SECONDS):
        left = int((self.deadline_ns - self.monotonic_ns()) / 1_000_000_000)
        if left < 1:
            return None
        return min(maximum, left)


def contain_owned(intent, prover_receipt, unit_receipt, run_command, deadline_ns, purpose="safety-cleanup",
                  unit_started=None, prover_started=None, monotonic_ns=None):
    """Identity-checked bounded termination and independent containment observation.

    deadline_ns is the absolute monotonic cutoff shared by all commands; no
    command starts once it has passed. Receipts must match the intent exactly.
    unit_started/prover_started are in-process startup observations by the
    executor itself and may stand in for a receipt whose persistence failed in
    the same process; the detached timer never has them.
    """
    monotonic_ns = monotonic_ns or __import__("time").monotonic_ns
    budget = _Budget(deadline_ns, monotonic_ns)
    log = []
    facts = {"purpose": purpose, "observedAtUtc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
             "deadlineMonotonicNs": int(deadline_ns), "unit": {}, "prover": {}, "commands": log,
             "outstandingOwners": []}
    unit = intent["unit"]
    unit_facts = facts["unit"]
    prover_facts = facts["prover"]

    def exhausted(resource_facts, owner, resource):
        resource_facts["disposition"] = "cleanup-budget-exhausted"
        resource_facts["contained"] = False
        facts["outstandingOwners"].append({"owner": owner, "resource": resource, "reason": "cleanup-budget-exhausted"})

    def command(argv_builder):
        bound = budget.remaining()
        if bound is None:
            return None
        return _run(run_command, argv_builder(bound), bound, log)

    # --- financial unit -------------------------------------------------------
    if unit_receipt is not None and not receipt_matches_intent(unit_receipt, UNIT_RECEIPT_KEYS, UNIT_RECEIPT_SCHEMA, intent):
        unit_facts["disposition"] = "receipt-identity-mismatch"
        unit_facts["contained"] = False
        unit_receipt = None
        facts["outstandingOwners"].append({"owner": "timer:" + intent["timerUnit"], "resource": "unit:" + unit,
                                           "reason": "receipt-identity-mismatch"})
    else:
        expected_invocation = None
        expected_group = None
        if unit_receipt is not None:
            expected_invocation = unit_receipt["unitInvocationId"]
            expected_group = unit_receipt["unitControlGroup"]
        elif unit_started is not None:
            expected_invocation = unit_started.get("InvocationID")
            expected_group = unit_started.get("ControlGroup")
        unit_facts["expectedInvocationId"] = expected_invocation
        unit_facts["expectedControlGroup"] = expected_group
        show = command(lambda b: unit_identity_argv(unit, b))
        fields = parse_unit_show(show.stdout) if show is not None and show.returncode == 0 else None
        unit_facts["before"] = fields
        if show is None and budget.remaining() is None:
            exhausted(unit_facts, "timer:" + intent["timerUnit"], "unit:" + unit)
        elif fields is None:
            unit_facts["disposition"] = "observation-unavailable"
            unit_facts["contained"] = False
        elif _unit_contained(fields):
            unit_facts["disposition"] = "absent-or-inactive"
            unit_facts["contained"] = True
        elif expected_invocation is None:
            unit_facts["disposition"] = "no-ownership-receipt"
            unit_facts["contained"] = False
        elif fields.get("InvocationID") != expected_invocation or fields.get("ControlGroup") != expected_group:
            unit_facts["disposition"] = "invocation-mismatch-not-killed"
            unit_facts["contained"] = False
        else:
            killed = command(lambda b: unit_kill_argv(unit, b))
            if killed is None:
                exhausted(unit_facts, "timer:" + intent["timerUnit"], "unit:" + unit)
            else:
                # Requery immediately before the second destructive command.
                again = command(lambda b: unit_identity_argv(unit, b))
                again_fields = parse_unit_show(again.stdout) if again is not None and again.returncode == 0 else None
                unit_facts["afterKill"] = again_fields
                if again is None:
                    exhausted(unit_facts, "timer:" + intent["timerUnit"], "unit:" + unit)
                elif again_fields is None:
                    unit_facts["disposition"] = "observation-unavailable-after-kill"
                    unit_facts["contained"] = False
                elif _unit_contained(again_fields):
                    unit_facts["disposition"] = "killed"
                    unit_facts["contained"] = True
                elif again_fields.get("InvocationID") != expected_invocation:
                    unit_facts["disposition"] = "invocation-changed-after-kill"
                    unit_facts["contained"] = False
                else:
                    stopped = command(lambda b: unit_stop_argv(unit, b))
                    if stopped is None:
                        exhausted(unit_facts, "timer:" + intent["timerUnit"], "unit:" + unit)
                    else:
                        after = command(lambda b: unit_identity_argv(unit, b))
                        after_fields = parse_unit_show(after.stdout) if after is not None and after.returncode == 0 else None
                        unit_facts["after"] = after_fields
                        if after is None:
                            exhausted(unit_facts, "timer:" + intent["timerUnit"], "unit:" + unit)
                        elif after_fields is not None and after_fields.get("LoadState") == "loaded" \
                                and after_fields.get("InvocationID") not in ("", expected_invocation):
                            unit_facts["disposition"] = "replacement-observed-after-stop"
                            unit_facts["contained"] = False
                        else:
                            unit_facts["disposition"] = "killed-and-stopped"
                            unit_facts["contained"] = _unit_contained(after_fields)
        if not unit_facts.get("contained") and unit_facts.get("disposition") != "cleanup-budget-exhausted":
            facts["outstandingOwners"].append({"owner": "timer:" + intent["timerUnit"], "resource": "unit:" + unit,
                                               "reason": unit_facts["disposition"]})

    # --- prover container ---------------------------------------------------------
    container = intent["containerId"]
    if prover_receipt is not None and not receipt_matches_intent(prover_receipt, PROVER_RECEIPT_KEYS,
                                                                PROVER_RECEIPT_SCHEMA, intent):
        prover_facts["disposition"] = "receipt-identity-mismatch"
        prover_facts["contained"] = False
        facts["outstandingOwners"].append({"owner": "pid1:" + container[:12], "resource": "container:" + container,
                                           "reason": "receipt-identity-mismatch"})
    else:
        expected_started = None
        if prover_receipt is not None:
            expected_started = prover_receipt["containerStartedAt"]
        elif prover_started is not None:
            expected_started = prover_started.get("StartedAt")
        prover_facts["expectedStartedAt"] = expected_started
        inspect = command(lambda b: docker_inspect_argv(container, b))
        state = parse_container_inspect(inspect.stdout) if inspect is not None and inspect.returncode == 0 else None
        prover_facts["before"] = state
        if inspect is None and budget.remaining() is None:
            exhausted(prover_facts, "pid1:" + container[:12], "container:" + container)
        elif inspect is not None and inspect.returncode != 0 and "No such" in (inspect.stderr or ""):
            prover_facts["disposition"] = "absent"
            prover_facts["contained"] = True
        elif state is None:
            prover_facts["disposition"] = "observation-unavailable"
            prover_facts["contained"] = False
        elif state["Id"] != container:
            prover_facts["disposition"] = "id-mismatch-not-killed"
            prover_facts["contained"] = False
        elif not state["Running"]:
            prover_facts["disposition"] = "not-running"
            prover_facts["contained"] = expected_started is None or state["StartedAt"] == expected_started
            if not prover_facts["contained"]:
                prover_facts["disposition"] = "replacement-generation-not-running"
        elif expected_started is None:
            prover_facts["disposition"] = "no-ownership-receipt"
            prover_facts["contained"] = False
        elif state["StartedAt"] != expected_started:
            prover_facts["disposition"] = "replacement-generation-not-killed"
            prover_facts["contained"] = False
        else:
            killed = command(lambda b: docker_kill_argv(container, b))
            if killed is None:
                exhausted(prover_facts, "pid1:" + container[:12], "container:" + container)
            else:
                again = command(lambda b: docker_inspect_argv(container, b))
                after_state = parse_container_inspect(again.stdout) if again is not None and again.returncode == 0 else None
                prover_facts["after"] = after_state
                if again is None:
                    exhausted(prover_facts, "pid1:" + container[:12], "container:" + container)
                else:
                    prover_facts["disposition"] = "killed"
                    prover_facts["contained"] = after_state is not None and after_state["Id"] == container \
                        and after_state["StartedAt"] == expected_started and not after_state["Running"]
                    if after_state is not None and after_state["StartedAt"] != expected_started:
                        prover_facts["disposition"] = "replacement-generation-after-kill"
        if not prover_facts.get("contained") and prover_facts.get("disposition") != "cleanup-budget-exhausted":
            facts["outstandingOwners"].append({"owner": "pid1:" + container[:12], "resource": "container:" + container,
                                               "reason": prover_facts["disposition"]})
    facts["complete"] = bool(unit_facts.get("contained")) and bool(prover_facts.get("contained"))
    return facts


def production_run_command(argv, timeout_seconds):
    try:
        completed = subprocess.run(list(argv), capture_output=True, text=True, timeout=timeout_seconds + 1,
                                   check=False, stdin=subprocess.DEVNULL)
    except subprocess.TimeoutExpired as error:
        return CommandResult(argv, None, (error.stdout or b"").decode("utf-8", "replace") if isinstance(error.stdout, bytes) else (error.stdout or ""),
                             "", timed_out=True)
    return CommandResult(argv, completed.returncode, completed.stdout, completed.stderr)


def main(argv=None):
    parser = argparse.ArgumentParser(prog="loan_cleanup", add_help=True,
                                     description="Identity-safe timer cleanup for one loan invocation")
    parser.add_argument("--intent", required=True)
    parser.add_argument("--sha256", required=True)
    parser.add_argument("--bound", type=int, default=DEFAULT_BOUND_SECONDS)
    args = parser.parse_args(argv)
    try:
        intent = load_intent(args.intent, args.sha256)
    except (OSError, ValueError) as error:
        sys.stderr.write("cleanup refused: %s\n" % error)
        return 2
    prover_receipt = load_receipt(intent["proverReceiptPath"], PROVER_RECEIPT_KEYS, PROVER_RECEIPT_SCHEMA, intent)
    unit_receipt = load_receipt(intent["unitReceiptPath"], UNIT_RECEIPT_KEYS, UNIT_RECEIPT_SCHEMA, intent)
    import time
    facts = contain_owned(intent, prover_receipt, unit_receipt, production_run_command,
                          time.monotonic_ns() + args.bound * 1_000_000_000, purpose="timer-cleanup")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    target = Path(intent["evidenceDirectory"]) / ("timer-cleanup-%s.json" % stamp)
    try:
        durable_exclusive_save(target, facts, timeout_seconds=args.bound)
    except Exception as error:  # noqa: BLE001 - retention failure is reported, not hidden
        sys.stderr.write("cleanup facts not retained: %s\n" % error)
        return 3
    return 0 if facts["complete"] else 3


if __name__ == "__main__":
    raise SystemExit(main())
