"""Collect a loaded systemd main-exit observation and persist it before stop.

Import does not call systemctl, docker, or a network. A separately admitted
loan executor calls retain_exit_evidence with an explicit transport.
"""
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess

from launch_contract import (
    SCOPE,
    TERMINAL_SHOW_PROPERTIES,
    describe_contract,
    explicit_stop_argv,
    require_startup_identity,
    require_unit_name,
    terminal_show_argv,
)
from terminal_predicate import require_exit_zero

KNOWN_CODES = {
    "INVOCATION_REQUIRED",
    "EXIT_ZERO_NOT_ESTABLISHED",
    "STARTUP_IDENTITY_REQUIRED",
    "UNIT_REQUIRED",
    "EVIDENCE_DIR_REQUIRED",
    "MALFORMED_SHOW",
    "DUPLICATE_FIELD",
    "SHOW_FAILED",
    "PERSISTENCE_FAILED",
    "RECORD_TOO_LARGE",
    "ARGV_REQUIRED",
    "PROPERTIES_REQUIRED",
    "EXPLICIT_RUNNER_REQUIRED",
    "STOP_EVIDENCE_PRESENT",
    "STOP_FAILED",
    "STOP_RECEIPT_PERSISTENCE_FAILED",
}
_PROPERTY_KEY = re.compile(r"[A-Za-z][A-Za-z0-9]*")


class CommandResult(object):
    def __init__(self, argv, returncode, stdout="", stderr=""):
        self.argv = list(argv)
        self.returncode = int(returncode)
        self.stdout = stdout if isinstance(stdout, str) else ""
        self.stderr = stderr if isinstance(stderr, str) else ""


def parse_systemctl_show(stdout):
    if not isinstance(stdout, str) or stdout == "":
        raise ValueError("MALFORMED_SHOW")
    fields = {}
    saw_line = False
    for line in stdout.splitlines():
        if line == "":
            continue
        saw_line = True
        if "=" not in line:
            raise ValueError("MALFORMED_SHOW")
        key, value = line.split("=", 1)
        if key == "" or _PROPERTY_KEY.fullmatch(key) is None:
            raise ValueError("MALFORMED_SHOW")
        if key in fields:
            raise ValueError("DUPLICATE_FIELD")
        fields[key] = value
    if not saw_line or not fields:
        raise ValueError("MALFORMED_SHOW")
    return fields


def require_evidence_dir(path):
    evidence = pathlib.Path(path)
    if evidence.is_symlink() or not evidence.is_dir():
        raise ValueError("EVIDENCE_DIR_REQUIRED")
    return evidence


def durable_exclusive_save(path, value):
    path = pathlib.Path(path)
    if path.exists() or path.is_symlink():
        raise FileExistsError("PERSISTENCE_FAILED")
    raw = (json.dumps(value, indent=2) + "\n").encode("utf-8")
    if len(raw) > 65536:
        raise ValueError("RECORD_TOO_LARGE")
    fd = os.open(str(path), os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        remaining = memoryview(raw)
        while remaining:
            n = os.write(fd, remaining)
            if n <= 0:
                raise OSError("PERSISTENCE_FAILED")
            remaining = remaining[n:]
        os.fsync(fd)
    finally:
        os.close(fd)
    dirfd = os.open(str(path.parent), os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(dirfd)
    finally:
        os.close(dirfd)


def _now_utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def _sha256_file(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def _code(error):
    if isinstance(error, ValueError) and str(error) in KNOWN_CODES:
        return str(error)
    if isinstance(error, FileExistsError):
        return "PERSISTENCE_FAILED"
    if isinstance(error, OSError):
        return "PERSISTENCE_FAILED"
    return type(error).__name__


def _coerce_result(result, argv, failed="SHOW_FAILED"):
    if isinstance(result, CommandResult):
        return result
    returncode = getattr(result, "returncode", None)
    if returncode is None:
        raise ValueError(failed)
    try:
        argv_list = list(getattr(result, "args", argv) or argv)
        stdout = getattr(result, "stdout", "") or ""
        stderr = getattr(result, "stderr", "") or ""
        if not isinstance(stdout, str) or not isinstance(stderr, str):
            raise TypeError(failed)
        return CommandResult(argv_list, returncode, stdout, stderr)
    except (TypeError, ValueError):
        raise ValueError(failed)


def invoke_user_systemctl(argv, timeout=30):
    """Run argv with subprocess.run. Not called on import or by this packet CLI."""
    if not isinstance(argv, (list, tuple)) or not argv:
        raise ValueError("ARGV_REQUIRED")
    command = []
    for part in argv:
        if not isinstance(part, str):
            raise ValueError("ARGV_REQUIRED")
        command.append(part)
    completed = subprocess.run(
        command,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    return CommandResult(command, completed.returncode, completed.stdout, completed.stderr)


class UserSystemctlTransport(object):
    """User-instance systemctl transport.

    Construct with an explicit runner. Offline tests pass a scripted runner.
    A separately admitted executor may pass invoke_user_systemctl. This class
    is not constructed on import or by cli.py in this packet.
    """

    def __init__(self, runner):
        if runner is None or not callable(runner):
            raise TypeError("EXPLICIT_RUNNER_REQUIRED")
        self._runner = runner

    def show(self, unit, properties):
        if not isinstance(properties, str) or properties == "":
            raise ValueError("PROPERTIES_REQUIRED")
        argv = [
            "/usr/bin/systemctl",
            "--user",
            "show",
            require_unit_name(unit),
            "--property=" + properties,
        ]
        return _coerce_result(self._runner(argv), argv)

    def stop(self, unit):
        argv = explicit_stop_argv(unit)
        return _coerce_result(self._runner(argv), argv, "STOP_FAILED")


def _stop_status(stop_return_code, stop_error_class, stop_receipt_persisted):
    if stop_return_code == 0 and stop_error_class is None and stop_receipt_persisted:
        return "EXIT_RETAINED_THEN_STOP_COMPLETED"
    if stop_return_code == 0 and stop_error_class is None:
        return "STOP_RECEIPT_PERSISTENCE_FAILED"
    return "STOP_FAILED_AFTER_RETENTION"


def _same_invocation_running(fields, invocation):
    if not isinstance(fields, dict) or not isinstance(invocation, str):
        return False
    pid = fields.get("MainPID")
    return (
        fields.get("LoadState") == "loaded"
        and fields.get("ActiveState") == "active"
        and fields.get("SubState") == "running"
        and fields.get("InvocationID") == invocation
        and isinstance(pid, str)
        and re.fullmatch(r"[1-9][0-9]*", pid) is not None
    )


def retain_exit_evidence(unit, startup_observed, transport, evidence_dir, now_utc=None):
    """Bind startup InvocationID, observe, persist raw/validated evidence, then stop.

    Stop is called only after matching terminal-zero evidence is durably
    saved. A same-invocation still-running unit returns NOT_TERMINAL without
    final files or stop so a parent monitor may call again under its original
    deadline. Failures do not stop the unit; existing parent cleanup/timer
    remains responsible. FINANCIAL_COMPLETE text is not consulted.
    """
    if not __debug__:
        raise RuntimeError("ASSERTIONS_REQUIRED")
    clock = now_utc or _now_utc
    sequence = []
    invocation = None
    rejected = None
    predicate = None
    persist_failed = False
    observation_digest = None
    stop_attempted = False
    explicit_stop_completed = False
    show_result = None
    stop_result = None
    observation_path = None
    stop_path = None
    record = {
        "scope": SCOPE,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "admission": None,
        "predicate": None,
        "rejected": None,
        "observed": None,
    }

    try:
        unit = require_unit_name(unit)
        evidence_dir = require_evidence_dir(evidence_dir)
    except ValueError as error:
        return {
            "status": "RETENTION_FAILED",
            "pending": False,
            "exitRetained": False,
            "explicitStopCompleted": False,
            "stopAttempted": False,
            "sequence": sequence,
            "unit": None,
            "startupInvocationId": None,
            "predicate": None,
            "rejected": _code(error),
            "terminalObservationPath": None,
            "terminalObservationSha256": None,
            "explicitStopPath": None,
            "financialAcceptance": False,
            "containmentAcceptance": False,
            "sp05Complete": False,
            "admission": None,
            "scope": SCOPE,
            "errorClass": type(error).__name__,
        }

    observation_path = evidence_dir / "terminal-observation.json"
    stop_path = evidence_dir / "explicit-stop.json"
    if stop_path.exists() or stop_path.is_symlink():
        return {
            "status": "RETENTION_FAILED",
            "pending": False,
            "exitRetained": False,
            "explicitStopCompleted": False,
            "stopAttempted": False,
            "sequence": sequence,
            "unit": unit,
            "startupInvocationId": None,
            "predicate": None,
            "rejected": "STOP_EVIDENCE_PRESENT",
            "terminalObservationPath": None,
            "terminalObservationSha256": None,
            "explicitStopPath": str(stop_path),
            "financialAcceptance": False,
            "containmentAcceptance": False,
            "sp05Complete": False,
            "admission": None,
            "scope": SCOPE,
        }
    show_argv = terminal_show_argv(unit)
    stop_argv = explicit_stop_argv(unit)

    try:
        invocation = require_startup_identity(startup_observed)
    except ValueError as error:
        return {
            "status": "RETENTION_FAILED",
            "pending": False,
            "exitRetained": False,
            "explicitStopCompleted": False,
            "stopAttempted": False,
            "sequence": sequence,
            "unit": unit,
            "startupInvocationId": None,
            "predicate": None,
            "rejected": _code(error),
            "terminalObservationPath": None,
            "terminalObservationSha256": None,
            "explicitStopPath": None,
            "financialAcceptance": False,
            "containmentAcceptance": False,
            "sp05Complete": False,
            "admission": None,
            "scope": SCOPE,
            "errorClass": type(error).__name__,
        }

    sequence.append("show")
    try:
        show_result = _coerce_result(
            transport.show(unit, TERMINAL_SHOW_PROPERTIES), show_argv, "SHOW_FAILED"
        )
    except Exception as error:
        if rejected is None:
            rejected = "SHOW_FAILED" if not isinstance(error, ValueError) else _code(error)
        record["errorClass"] = type(error).__name__
        show_result = None

    record["observedAtUtc"] = clock()
    if show_result is None:
        record["argv"] = list(show_argv)
        record["returnCode"] = 1
        record["stdout"] = ""
        record["stderr"] = ""
    else:
        record["argv"] = list(show_result.argv)
        record["returnCode"] = int(show_result.returncode)
        record["stdout"] = show_result.stdout
        record["stderr"] = show_result.stderr
    record["startupInvocationId"] = invocation
    record["rejected"] = rejected

    show_ok = show_result is not None and int(show_result.returncode) == 0
    if not show_ok and rejected is None:
        rejected = "SHOW_FAILED"
        record["rejected"] = rejected

    if show_ok:
        try:
            fields = parse_systemctl_show(record["stdout"])
            record["observed"] = fields
            if invocation is not None and rejected is None:
                if _same_invocation_running(fields, invocation):
                    return {
                        "status": "NOT_TERMINAL",
                        "pending": True,
                        "exitRetained": False,
                        "explicitStopCompleted": False,
                        "stopAttempted": False,
                        "sequence": sequence,
                        "unit": unit,
                        "startupInvocationId": invocation,
                        "predicate": None,
                        "rejected": None,
                        "terminalObservationPath": None,
                        "terminalObservationSha256": None,
                        "explicitStopPath": None,
                        "financialAcceptance": False,
                        "containmentAcceptance": False,
                        "sp05Complete": False,
                        "admission": None,
                        "scope": SCOPE,
                    }
                predicate = require_exit_zero(fields, invocation)
                record["predicate"] = predicate
        except ValueError as error:
            rejected = _code(error)
            record["rejected"] = rejected
            record["predicate"] = None
            if record["observed"] is None:
                try:
                    record["observed"] = parse_systemctl_show(record["stdout"])
                except ValueError:
                    record["observed"] = None

    sequence.append("persist")
    try:
        durable_exclusive_save(observation_path, record)
        observation_digest = _sha256_file(observation_path)
    except Exception as error:
        persist_failed = True
        if rejected is None:
            rejected = _code(error)
        record["rejected"] = rejected

    exit_retained = (
        predicate is not None
        and predicate.get("status") == "EXIT_ZERO_OBSERVED"
        and predicate.get("exitCode") == 0
        and predicate.get("externalContainmentEstablished") is False
        and not persist_failed
        and rejected is None
        and observation_path is not None
        and observation_path.is_file()
        and observation_digest is not None
    )
    if not exit_retained:
        predicate = None
        return {
            "status": "RETENTION_FAILED",
            "pending": False,
            "exitRetained": False,
            "explicitStopCompleted": False,
            "stopAttempted": False,
            "sequence": sequence,
            "unit": unit,
            "startupInvocationId": invocation,
            "predicate": None,
            "rejected": rejected,
            "terminalObservationPath": str(observation_path) if observation_path.is_file() else None,
            "terminalObservationSha256": observation_digest,
            "explicitStopPath": None,
            "financialAcceptance": False,
            "containmentAcceptance": False,
            "sp05Complete": False,
            "admission": None,
            "scope": SCOPE,
        }

    sequence.append("stop")
    stop_attempted = True
    stop_return_code = None
    stop_error_class = None
    stop_error = None
    stop_receipt_persisted = False
    stop_receipt_error_class = None
    stop_receipt_error = None
    stop_argv_used = list(stop_argv)
    stop_stdout = ""
    stop_stderr = ""
    try:
        stop_result = _coerce_result(transport.stop(unit), stop_argv, "STOP_FAILED")
        stop_return_code = int(stop_result.returncode)
        stop_argv_used = list(stop_result.argv)
        stop_stdout = stop_result.stdout
        stop_stderr = stop_result.stderr
    except Exception as error:
        stop_error_class = type(error).__name__
        stop_error = str(error)
    stop_record = {
        "observedAtUtc": clock(),
        "argv": stop_argv_used,
        "returnCode": stop_return_code,
        "stdout": stop_stdout,
        "stderr": stop_stderr,
        "terminalBeforeStopSha256": observation_digest,
        "exitRetained": True,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "stopErrorClass": stop_error_class,
        "stopError": stop_error,
    }
    try:
        durable_exclusive_save(stop_path, stop_record)
        stop_receipt_persisted = stop_path.is_file()
    except Exception as error:
        stop_receipt_error_class = type(error).__name__
        stop_receipt_error = str(error) if str(error) else _code(error)
        stop_receipt_persisted = False
    explicit_stop_completed = (
        stop_return_code == 0 and stop_error_class is None and stop_receipt_persisted
    )

    return {
        "status": _stop_status(stop_return_code, stop_error_class, stop_receipt_persisted),
        "pending": False,
        "exitRetained": True,
        "explicitStopCompleted": explicit_stop_completed,
        "stopAttempted": stop_attempted,
        "stopReturnCode": stop_return_code,
        "stopErrorClass": stop_error_class,
        "stopError": stop_error,
        "stopReceiptPersisted": stop_receipt_persisted,
        "stopReceiptErrorClass": stop_receipt_error_class,
        "stopReceiptError": stop_receipt_error,
        "sequence": sequence,
        "unit": unit,
        "startupInvocationId": invocation,
        "predicate": predicate,
        "rejected": None,
        "terminalObservationPath": str(observation_path) if observation_path.is_file() else None,
        "terminalObservationSha256": observation_digest,
        "explicitStopPath": str(stop_path) if stop_path.is_file() else None,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "sp05Complete": False,
        "admission": None,
        "scope": SCOPE,
    }
