"""Collect a loaded systemd main-exit observation and persist it before stop.

Import does not call systemctl, docker, or a network. The authenticated loan
executor calls retain_exit_evidence with an explicit transport and explicit
remaining-time bounds. Persistence may run in a bounded worker process so a
stalled write or fsync is killed instead of hanging the caller; a killed
worker leaves the artifact incomplete and never counts as durable.
"""
import datetime
import hashlib
import json
import os
import pathlib
import re
import signal
import subprocess
import time

from .launch_contract import (
    MAX_SHOW_SECONDS,
    MAX_STOP_SECONDS,
    SCOPE,
    TERMINAL_SHOW_PROPERTIES,
    describe_contract,
    explicit_stop_argv,
    require_bound_seconds,
    require_startup_identity,
    require_unit_name,
    terminal_show_argv,
)
from .terminal_predicate import classify_main_exit, require_exit_zero

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
    "PERSISTENCE_TIMEOUT",
    "RECORD_TOO_LARGE",
    "ARGV_REQUIRED",
    "PROPERTIES_REQUIRED",
    "EXPLICIT_RUNNER_REQUIRED",
    "STOP_EVIDENCE_PRESENT",
    "STOP_FAILED",
    "STOP_RECEIPT_PERSISTENCE_FAILED",
    "BOUND_REQUIRED",
}
_PROPERTY_KEY = re.compile(r"[A-Za-z][A-Za-z0-9]*")
MAX_RECORD_BYTES = 65536


class CommandResult(object):
    def __init__(self, argv, returncode, stdout="", stderr="", timed_out=False):
        self.argv = list(argv)
        self.returncode = None if returncode is None else int(returncode)
        self.stdout = stdout if isinstance(stdout, str) else ""
        self.stderr = stderr if isinstance(stderr, str) else ""
        self.timed_out = bool(timed_out)


class PersistenceTimeout(OSError):
    """A bounded persistence worker missed its bound and was killed."""


class BoundedResult(object):
    def __init__(self, status, returncode=None, detail=None, elapsed=None):
        self.status = status  # completed | failed | timeout
        self.returncode = returncode
        self.detail = detail
        self.elapsed = elapsed

    @property
    def completed(self):
        return self.status == "completed"


def run_bounded(work, timeout_seconds, poll_interval=0.005):
    """Run work() in a forked worker; kill it if it misses timeout_seconds.

    The worker exits 0 on success and 1 on an exception, reporting the error
    class through a pipe. This is process supervision: a blocking write or
    fsync inside work() cannot be interrupted by a clock check alone.
    """
    if type(timeout_seconds) in (int, float) and timeout_seconds <= 0:
        return BoundedResult("timeout", None, "NONPOSITIVE_BOUND", 0.0)
    read_fd, write_fd = os.pipe()
    started = time.monotonic()
    pid = os.fork()
    if pid == 0:  # pragma: no cover - worker body runs in the child process
        os.close(read_fd)
        code = 0
        try:
            value = work()
            try:
                message = b"ok:" + json.dumps(value, sort_keys=True).encode("utf-8")
            except (TypeError, ValueError):
                message = b"ok:null"
        except BaseException as error:  # noqa: BLE001 - report any failure class
            code = 1
            message = (type(error).__name__ + ":" + str(error)[:200]).encode("utf-8", "replace")
        try:
            os.write(write_fd, message[:65536])
        finally:
            os._exit(code)
    os.close(write_fd)
    deadline = started + float(timeout_seconds)
    status = None
    try:
        while True:
            waited, wait_status = os.waitpid(pid, os.WNOHANG)
            if waited == pid:
                status = wait_status
                break
            if time.monotonic() >= deadline:
                os.kill(pid, signal.SIGKILL)
                os.waitpid(pid, 0)
                return BoundedResult("timeout", None, "PERSISTENCE_TIMEOUT", time.monotonic() - started)
            time.sleep(poll_interval)
        chunks = []
        while True:
            chunk = os.read(read_fd, 4096)
            if not chunk:
                break
            chunks.append(chunk)
    finally:
        os.close(read_fd)
    detail = b"".join(chunks).decode("utf-8", "replace")
    code = os.waitstatus_to_exitcode(status)
    if code == 0 and detail.startswith("ok:"):
        try:
            value = json.loads(detail[3:])
        except ValueError:
            value = None
        result = BoundedResult("completed", 0, detail, time.monotonic() - started)
        result.value = value
        return result
    return BoundedResult("failed", code, detail, time.monotonic() - started)


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


def _write_exclusive_sync(path, raw):
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


def encode_record(value):
    raw = (json.dumps(value, indent=2) + "\n").encode("utf-8")
    if len(raw) > MAX_RECORD_BYTES:
        raise ValueError("RECORD_TOO_LARGE")
    return raw


def durable_exclusive_save(path, value, timeout_seconds=None):
    """Exclusive no-follow write with file and directory fsync.

    With timeout_seconds the write runs in a bounded worker. A worker that
    misses the bound is killed and PersistenceTimeout is raised; the partial
    file, if any, remains as evidence and is never treated as durable.
    """
    path = pathlib.Path(path)
    if path.exists() or path.is_symlink():
        raise FileExistsError("PERSISTENCE_FAILED")
    raw = encode_record(value)
    if timeout_seconds is None:
        _write_exclusive_sync(path, raw)
        return
    outcome = run_bounded(lambda: _write_exclusive_sync(path, raw), timeout_seconds)
    if outcome.status == "timeout":
        raise PersistenceTimeout("PERSISTENCE_TIMEOUT")
    if outcome.status != "completed":
        raise OSError("PERSISTENCE_FAILED:" + str(outcome.detail))


def _now_utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def _sha256_file(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def _code(error):
    if isinstance(error, ValueError) and str(error) in KNOWN_CODES:
        return str(error)
    if isinstance(error, PersistenceTimeout):
        return "PERSISTENCE_TIMEOUT"
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
    The authenticated executor passes a bounded command runner. This class is
    not constructed on import or by cli.py.
    """

    def __init__(self, runner):
        if runner is None or not callable(runner):
            raise TypeError("EXPLICIT_RUNNER_REQUIRED")
        self._runner = runner

    def show(self, unit, properties, show_seconds=None):
        if not isinstance(properties, str) or properties == "":
            raise ValueError("PROPERTIES_REQUIRED")
        argv = ["/usr/bin/systemctl", "--user", "show", require_unit_name(unit), "--property=" + properties]
        if show_seconds is not None:
            argv = terminal_show_argv(unit, show_seconds)[:3] + argv
        return _coerce_result(self._runner(argv), argv)

    def stop(self, unit, stop_seconds=MAX_STOP_SECONDS):
        argv = explicit_stop_argv(unit, stop_seconds)
        return _coerce_result(self._runner(argv), argv, "STOP_FAILED")


def _stop_status(stop_return_code, stop_error_class, stop_receipt_persisted):
    if stop_return_code == 0 and stop_error_class is None and stop_receipt_persisted:
        return "EXIT_RETAINED_THEN_STOP_COMPLETED"
    if stop_return_code == 0 and stop_error_class is None:
        return "STOP_RECEIPT_PERSISTENCE_FAILED"
    return "STOP_FAILED_AFTER_RETENTION"


def _call_show(transport, unit, show_seconds):
    if show_seconds is None:
        return transport.show(unit, TERMINAL_SHOW_PROPERTIES)
    return transport.show(unit, TERMINAL_SHOW_PROPERTIES, show_seconds)


def _call_stop(transport, unit, stop_seconds):
    if stop_seconds is None:
        return transport.stop(unit)
    return transport.stop(unit, stop_seconds)


def _failure(status, sequence, unit, invocation, rejected, observation_path=None,
             observation_digest=None, stop_path=None, classification=None, error=None, pending=False):
    result = {
        "status": status,
        "pending": pending,
        "exitRetained": False,
        "explicitStopCompleted": False,
        "stopAttempted": False,
        "sequence": sequence,
        "unit": unit,
        "startupInvocationId": invocation,
        "predicate": None,
        "rejected": rejected,
        "classification": classification,
        "terminalObservationPath": observation_path,
        "terminalObservationSha256": observation_digest,
        "explicitStopPath": stop_path,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "sp05Complete": False,
        "admission": None,
        "scope": SCOPE,
    }
    if error is not None:
        result["errorClass"] = type(error).__name__
    return result


def retain_exit_evidence(unit, startup_observed, transport, evidence_dir, now_utc=None,
                         show_seconds=None, stop_seconds=None, persist_timeout_seconds=None, save=None):
    """Bind startup InvocationID, observe, persist raw/validated evidence, then stop.

    Stop is called only after matching terminal-zero evidence is durably
    saved. A same-invocation still-running unit returns NOT_TERMINAL without
    final files or stop so a parent monitor may call again under its original
    deadline. Failures do not stop the unit; the authenticated caller's own
    identity-checked safety cleanup and the independent timer remain
    responsible. The closed classification of any parsed terminal observation
    is retained in the record and result; only the unchanged zero predicate
    sets exitRetained. FINANCIAL_COMPLETE text is not consulted.
    """
    if not __debug__:
        raise RuntimeError("ASSERTIONS_REQUIRED")
    clock = now_utc or _now_utc
    # save(path, value, timeout_seconds) replaces only the write I/O; the
    # persist-before-stop ordering and exclusivity checks stay here.
    save = save or durable_exclusive_save
    sequence = []
    invocation = None
    rejected = None
    predicate = None
    classification = None
    persist_failed = False
    observation_digest = None
    show_result = None
    record = {
        "scope": SCOPE,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "admission": None,
        "predicate": None,
        "classification": None,
        "rejected": None,
        "observed": None,
    }

    try:
        unit = require_unit_name(unit)
        evidence_dir = require_evidence_dir(evidence_dir)
        if show_seconds is not None:
            show_seconds = require_bound_seconds(show_seconds, MAX_SHOW_SECONDS)
        if stop_seconds is not None:
            stop_seconds = require_bound_seconds(stop_seconds, MAX_STOP_SECONDS)
    except ValueError as error:
        return _failure("RETENTION_FAILED", sequence, None, None, _code(error), error=error)

    observation_path = evidence_dir / "terminal-observation.json"
    stop_path = evidence_dir / "explicit-stop.json"
    if stop_path.exists() or stop_path.is_symlink():
        return _failure("RETENTION_FAILED", sequence, unit, None, "STOP_EVIDENCE_PRESENT",
                        stop_path=str(stop_path))
    show_argv = terminal_show_argv(unit, show_seconds)
    stop_argv = explicit_stop_argv(unit, MAX_STOP_SECONDS if stop_seconds is None else stop_seconds)

    try:
        invocation = require_startup_identity(startup_observed)
    except ValueError as error:
        return _failure("RETENTION_FAILED", sequence, unit, None, _code(error), error=error)

    sequence.append("show")
    try:
        show_result = _coerce_result(_call_show(transport, unit, show_seconds), show_argv, "SHOW_FAILED")
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
        record["timedOut"] = False
    else:
        record["argv"] = list(show_result.argv)
        record["returnCode"] = show_result.returncode
        record["stdout"] = show_result.stdout
        record["stderr"] = show_result.stderr
        record["timedOut"] = bool(getattr(show_result, "timed_out", False))
    record["showBoundSeconds"] = show_seconds
    record["startupInvocationId"] = invocation
    record["rejected"] = rejected

    show_ok = show_result is not None and show_result.returncode == 0 and not record["timedOut"]
    if not show_ok and rejected is None:
        rejected = "SHOW_FAILED"
        record["rejected"] = rejected
    if not show_ok:
        classification = {"rawMainExit": {"kind": "unknown", "code": None},
                          "status": "PROCESS_UNKNOWN", "failureCode": "MAIN_OBSERVATION_UNAVAILABLE"}
        record["classification"] = classification

    if show_ok:
        try:
            fields = parse_systemctl_show(record["stdout"])
            record["observed"] = fields
            classification = classify_main_exit(fields, invocation)
            record["classification"] = classification
            if invocation is not None and rejected is None:
                # One closed predicate decides polling: only the exact running
                # row is pending; contradictory rows are terminal unknown.
                if classification["status"] == "pending":
                    return _failure("NOT_TERMINAL", sequence, unit, invocation, None,
                                    classification=classification, pending=True)
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
            if classification is None:
                classification = {"rawMainExit": {"kind": "unknown", "code": None},
                                  "status": "PROCESS_UNKNOWN", "failureCode": "MAIN_OBSERVATION_INVALID"}
                record["classification"] = classification

    sequence.append("persist")
    try:
        save(observation_path, record, persist_timeout_seconds)
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
        result = _failure("RETENTION_FAILED", sequence, unit, invocation, rejected,
                          observation_path=str(observation_path) if observation_path.is_file() else None,
                          observation_digest=observation_digest, classification=classification)
        result["persistFailed"] = persist_failed
        return result

    sequence.append("stop")
    stop_return_code = None
    stop_error_class = None
    stop_error = None
    stop_receipt_persisted = False
    stop_receipt_error_class = None
    stop_receipt_error = None
    stop_argv_used = list(stop_argv)
    stop_stdout = ""
    stop_stderr = ""
    stop_timed_out = False
    try:
        stop_result = _coerce_result(_call_stop(transport, unit, stop_seconds), stop_argv, "STOP_FAILED")
        stop_return_code = stop_result.returncode
        stop_argv_used = list(stop_result.argv)
        stop_stdout = stop_result.stdout
        stop_stderr = stop_result.stderr
        stop_timed_out = bool(getattr(stop_result, "timed_out", False))
        if stop_timed_out or stop_return_code is None:
            stop_error_class = "TimeoutExpired"
            stop_error = "stop command missed its bound"
    except Exception as error:
        stop_error_class = type(error).__name__
        stop_error = str(error)
    stop_record = {
        "observedAtUtc": clock(),
        "argv": stop_argv_used,
        "returnCode": stop_return_code,
        "stdout": stop_stdout,
        "stderr": stop_stderr,
        "timedOut": stop_timed_out,
        "stopBoundSeconds": MAX_STOP_SECONDS if stop_seconds is None else stop_seconds,
        "terminalBeforeStopSha256": observation_digest,
        "exitRetained": True,
        "financialAcceptance": False,
        "containmentAcceptance": False,
        "stopErrorClass": stop_error_class,
        "stopError": stop_error,
    }
    try:
        save(stop_path, stop_record, persist_timeout_seconds)
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
        "stopAttempted": True,
        "stopReturnCode": stop_return_code,
        "stopErrorClass": stop_error_class,
        "stopError": stop_error,
        "stopArgv": stop_argv_used,
        "stopReceiptPersisted": stop_receipt_persisted,
        "stopReceiptErrorClass": stop_receipt_error_class,
        "stopReceiptError": stop_receipt_error,
        "sequence": sequence,
        "unit": unit,
        "startupInvocationId": invocation,
        "predicate": predicate,
        "classification": classification,
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
