"""Loan executor: the callable production consumer of the loan lifecycle boundary.

Entry: ``main(argv=None, dependencies=None)`` accepting exactly
``--plan <contained-wrapper> --sha256 <wrapper-digest> --admission <contained-immutable-authority>``.
Import and ``--help`` perform no wallet, network, store mutation or service
operation. Only ``main`` constructs production transports when dependencies
are absent; injected transports replace external I/O, never parsing,
authentication predicates, the collector, persistence ordering or the final
classification.

``execute_once(plan, admission, dependencies)`` runs one authenticated
invocation: invocation exchange with the runner over its private AF_UNIX
channel, current-chain validation through ``accounting``, prover lifetime
control, timer arming, single Docker start, financial unit launch, the
installed collector, identity-safe cleanup, and the durable
``moriarty.loan-process-result/1`` document at the immutable resultPath.
"""
import argparse
import datetime
import hashlib
import hmac
import json
import os
from pathlib import Path
import re
import select
import socket
import stat
import subprocess
import sys
import time

if __package__ in (None, ""):  # direct script execution adds only scripts/
    _SCRIPTS = Path(__file__).resolve().parents[1]
    if str(_SCRIPTS) not in sys.path:
        sys.path.insert(0, str(_SCRIPTS))

from moriarty_dev import accounting, loan_cleanup, records, store
from moriarty_dev.accounting import Authenticated, Refusal
from moriarty_dev.loan_exit_retention.exit_retention import (
    CommandResult,
    PersistenceTimeout,
    UserSystemctlTransport,
    parse_systemctl_show,
    run_bounded,
)
from moriarty_dev.loan_exit_retention.launch_contract import (
    MAX_SHOW_SECONDS,
    MAX_STOP_SECONDS,
    SYSTEMCTL,
    require_startup_identity,
    require_unit_name,
    startup_show_argv,
    timeout_prefix,
)
from moriarty_dev.loan_exit_retention.loan_exit_operator import retain_loan_main_exit

PLAN_SCHEMA = "moriarty.loan-executor-plan/1"
FINANCIAL_SCHEMA = "moriarty.preview-financial-launch/1"
RESULT_SCHEMA = "moriarty.loan-process-result/1"
HANDSHAKE_SCHEMA = "moriarty.loan-handshake/1"
ANSWER_SCHEMA = "moriarty.loan-invocation-answer/1"
PROGRESS_SCHEMA = "moriarty.loan-progress/1"
PLAN_KEYS = (
    "schema", "allocationId", "actionId", "candidateHash", "financialPlan", "command", "closure",
    "authority", "unit", "timerUnit", "prover", "evidenceDirectory", "resultPath", "limits",
)
PROVER_KEYS = ("containerId", "imageDigest", "memoryLimitBytes", "lifetime")
LIFETIME_KEYS = ("wrapper", "controlDirectory", "controlName", "containerControlPath")
FIXED_LIMITS = {
    "outerSeconds": 1800, "graceSeconds": 5, "setupSeconds": 120, "operationDeadlineSeconds": 1600,
    "bootstrapExitSeconds": 1606, "collectionDeadlineSeconds": 1618, "timerSeconds": 1620,
    "cleanupSeconds": 1740, "showSeconds": 5, "stopSeconds": 25,
}
RESULT_KEYS = (
    "schema", "allocationId", "actionId", "candidateHash", "runnerDigest", "chargeId", "reservationId",
    "invocationSha256", "unit", "invocationId", "status", "rawMainExit", "terminalEvidencePersisted",
    "stopReturnCode", "stopErrorClass", "stopReceiptPersisted", "containmentComplete",
    "timerCancelReturnCode", "timerCancelReceiptPersisted", "failureCode", "evidence",
    "outstandingOwners", "retryAllowed", "financialAcceptance",
)
STATUS_EXIT = {"PROCESS_SUCCESS": 0, "PROCESS_FAILED": 1, "REFUSED": 2, "PROCESS_UNKNOWN": 3}
RUNTIME_PLACEHOLDER = "{runtimeFinancialPlan}"
ENV_SOCKET = "MORIARTY_INVOCATION_SOCKET"
ENV_NONCE = "MORIARTY_INVOCATION_NONCE"
NS = 1_000_000_000
PROVER_CONTROL_SCHEMA = "moriarty.prover-lifetime/1"
NEVER_STARTED = "0001-01-01T00:00:00Z"
DOCKER = loan_cleanup.DOCKER
SYSTEMD_RUN = loan_cleanup.SYSTEMD_RUN
MAX_LINE = 65536
_IDENT = re.compile(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,127}")
_UUID = re.compile(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")


class ExecutorRefusal(Exception):
    def __init__(self, code, detail=""):
        super().__init__(code + (": " + detail if detail else ""))
        self.code = code
        self.detail = detail


class DurableWriteResult(object):
    def __init__(self, persisted, path, sha256=None, error_class=None, detail=None):
        self.persisted = bool(persisted)
        self.path = str(path)
        self.sha256 = sha256
        self.error_class = error_class
        self.detail = detail


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _now_utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def _no_symlink_components(root, requested):
    """Every component of the requested path below root must not be a symlink."""
    current = root
    for part in Path(requested).parts:
        if part in ("", "."):
            continue
        current = current / part
        if current.is_symlink():
            return False
    return True


def _contained_dir(root, relative, kind):
    if type(relative) is not str or relative == "" or relative.startswith("/") or ".." in relative.split("/"):
        raise ExecutorRefusal("PLAN_INVALID", kind + " must be a contained relative directory")
    path = (root / relative)
    if path.is_symlink() or not path.is_dir():
        raise ExecutorRefusal("PLAN_INVALID", kind + " is not a directory: " + relative)
    resolved = path.resolve()
    if not resolved.is_relative_to(root) or not _no_symlink_components(root, relative):
        raise ExecutorRefusal("PLAN_INVALID", kind + " escapes the repository")
    return resolved


def _reference(root, value, kind, require_json=False):
    missing = []
    loaded = records._reference(root, value, missing, kind)
    if loaded is None:
        raise ExecutorRefusal("PLAN_INVALID", kind + ": " + ", ".join(missing))
    path, data = loaded
    if not _no_symlink_components(root, value["path"]):
        raise ExecutorRefusal("PLAN_INVALID", kind + " reaches through a symlink")
    if require_json:
        payload = records._parse_json(data, missing, kind)
        if type(payload) is not dict:
            raise ExecutorRefusal("PLAN_INVALID", kind + " is not a JSON object")
        return path, data, payload
    return path, data


def load_plan(root, relative, expected_sha256):
    """Parse and validate the immutable executor plan; nothing is started."""
    root = Path(root).resolve()
    if type(expected_sha256) is not str or not records._hex64(expected_sha256):
        raise ExecutorRefusal("PLAN_INVALID", "--sha256 must be lowercase SHA-256")
    path, data = _reference(root, {"path": relative, "sha256": expected_sha256}, "plan")
    missing = []
    plan = records._parse_json(data, missing, "plan")
    if type(plan) is not dict or not records._exact_keys(plan, PLAN_KEYS, missing, "plan"):
        raise ExecutorRefusal("PLAN_INVALID", "plan fields: " + ", ".join(missing))
    if plan["schema"] != PLAN_SCHEMA:
        raise ExecutorRefusal("PLAN_INVALID", "plan schema")
    for key in ("allocationId", "actionId"):
        if type(plan[key]) is not str or _IDENT.fullmatch(plan[key]) is None:
            raise ExecutorRefusal("PLAN_INVALID", "invalid identifier: " + key)
    if not records._hex64(plan["candidateHash"]):
        raise ExecutorRefusal("PLAN_INVALID", "candidateHash")
    fin_path, fin_bytes, financial = _reference(root, plan["financialPlan"], "financialPlan", require_json=True)
    if financial.get("schema") != FINANCIAL_SCHEMA or type(financial.get("limits")) is not dict \
            or type(financial["limits"].get("deadlineMs")) is not int:
        raise ExecutorRefusal("PLAN_INVALID", "financialPlan is not an unchanged preview-financial-launch/1 document")
    _reference(root, plan["authority"], "authority")
    command = plan["command"]
    if type(command) is not list or len(command) < 2 or any(type(x) is not str or x == "" for x in command):
        raise ExecutorRefusal("PLAN_INVALID", "command argv")
    if command.count(RUNTIME_PLACEHOLDER) != 1 or command[0] == RUNTIME_PLACEHOLDER:
        raise ExecutorRefusal("PLAN_INVALID", "command must contain exactly one runtimeFinancialPlan operand")
    if any(RUNTIME_PLACEHOLDER in x and x != RUNTIME_PLACEHOLDER for x in command):
        raise ExecutorRefusal("PLAN_INVALID", "placeholder must be a whole operand")
    closure = plan["closure"]
    if type(closure) is not dict or not closure:
        raise ExecutorRefusal("PLAN_INVALID", "closure")
    for name, digest in closure.items():
        if type(name) is not str or not records._hex64(digest):
            raise ExecutorRefusal("PLAN_INVALID", "closure entry")
        target = Path(name) if name.startswith("/") else root / name
        if target.is_symlink() or not target.is_file():
            raise ExecutorRefusal("PLAN_INVALID", "closure file missing: " + name)
        if _sha(target.read_bytes()) != digest:
            raise ExecutorRefusal("PLAN_INVALID", "closure file changed: " + name)
    if command[0] not in closure or not command[0].startswith("/") or not os.access(command[0], os.X_OK):
        raise ExecutorRefusal("PLAN_INVALID", "command executable must be an absolute hash-bound executable")
    try:
        require_unit_name(plan["unit"])
        loan_cleanup.require_timer_name(plan["timerUnit"])
    except ValueError as error:
        raise ExecutorRefusal("PLAN_INVALID", str(error))
    if plan["timerUnit"] + ".service" == plan["unit"]:
        raise ExecutorRefusal("PLAN_INVALID", "timer name collides with unit name")
    prover = plan["prover"]
    if not records._exact_keys(prover, PROVER_KEYS, missing, "prover"):
        raise ExecutorRefusal("PLAN_INVALID", "prover fields: " + ", ".join(missing))
    if not records._hex64(prover["containerId"]):
        raise ExecutorRefusal("PLAN_INVALID", "prover containerId")
    image = prover["imageDigest"]
    if type(image) is not str or not image.startswith("sha256:") or not records._hex64(image[7:]):
        raise ExecutorRefusal("PLAN_INVALID", "prover imageDigest")
    if type(prover["memoryLimitBytes"]) is not int or prover["memoryLimitBytes"] <= 0:
        raise ExecutorRefusal("PLAN_INVALID", "prover memoryLimitBytes")
    lifetime = prover["lifetime"]
    if not records._exact_keys(lifetime, LIFETIME_KEYS, missing, "lifetime"):
        raise ExecutorRefusal("PLAN_INVALID", "lifetime fields: " + ", ".join(missing))
    wrapper_path, _ = _reference(root, lifetime["wrapper"], "wrapper")
    if not os.access(wrapper_path, os.X_OK):
        raise ExecutorRefusal("PLAN_INVALID", "wrapper is not executable")
    control_dir = _contained_dir(root, lifetime["controlDirectory"], "controlDirectory")
    if type(lifetime["controlName"]) is not str or _IDENT.fullmatch(lifetime["controlName"]) is None \
            or "/" in lifetime["controlName"]:
        raise ExecutorRefusal("PLAN_INVALID", "controlName")
    if type(lifetime["containerControlPath"]) is not str or not lifetime["containerControlPath"].startswith("/"):
        raise ExecutorRefusal("PLAN_INVALID", "containerControlPath")
    evidence = _contained_dir(root, plan["evidenceDirectory"], "evidenceDirectory")
    for entry in evidence.iterdir():
        # Only the runner's private channel directory may already exist.
        details = entry.lstat()
        if not (entry.name.startswith("channel-") and stat.S_ISDIR(details.st_mode)
                and stat.S_IMODE(details.st_mode) == 0o700 and details.st_uid == os.getuid()):
            raise ExecutorRefusal("PLAN_INVALID", "evidenceDirectory is already used: " + entry.name)
    result_rel = plan["resultPath"]
    if type(result_rel) is not str or result_rel.startswith("/") or ".." in result_rel.split("/"):
        raise ExecutorRefusal("PLAN_INVALID", "resultPath must be contained")
    result_path = (root / result_rel)
    if not result_path.parent.resolve() == evidence:
        raise ExecutorRefusal("PLAN_INVALID", "resultPath must sit directly inside evidenceDirectory")
    if result_path.exists() or result_path.is_symlink():
        raise ExecutorRefusal("PLAN_INVALID", "resultPath already exists")
    if plan["limits"] != FIXED_LIMITS:
        raise ExecutorRefusal("PLAN_INVALID", "limits differ from the fixed schedule")
    validated = Authenticated(plan, expected_sha256, path)
    validated.root = root
    validated.financial = financial
    validated.financial_bytes = fin_bytes
    validated.financial_path = fin_path
    validated.wrapper_path = wrapper_path
    validated.control_dir = control_dir
    validated.evidence_dir = evidence
    validated.result_path = result_path
    return validated


def load_admission(root, relative, plan):
    """The immutable resource amendment; its digest must equal plan.authority.sha256."""
    root = Path(root).resolve()
    expected = plan["authority"]["sha256"]
    if plan["authority"]["path"] != relative:
        raise ExecutorRefusal("AUTHORITY_INVALID", "--admission path differs from plan.authority")
    path, data, payload = _reference(root, {"path": relative, "sha256": expected}, "admission", require_json=True)
    extra = [k for k in payload if k not in records.RESOURCE_AMENDMENT_KEYS]
    if extra:
        raise ExecutorRefusal("AUTHORITY_INVALID", "unknown amendment fields: " + ",".join(sorted(extra)))
    if type(payload.get("resources")) is not dict:
        raise ExecutorRefusal("AUTHORITY_INVALID", "amendment carries no resources object")
    return Authenticated(payload, expected, path)


# --- parent channel (child side) ---------------------------------------------

class ParentChannel(object):
    """Authenticated transient channel to the invoking runner."""

    def __init__(self, sock, invocation, invocation_sha256):
        self._sock = sock
        self.invocation = invocation
        self.invocation_sha256 = invocation_sha256
        self.lost = False

    def parent_alive(self):
        if self.lost or self._sock is None:
            return False
        try:
            readable, _, _ = select.select([self._sock], [], [], 0)
            if readable:
                data = self._sock.recv(1, socket.MSG_PEEK)
                if data == b"":
                    self.lost = True
                    return False
        except OSError:
            self.lost = True
            return False
        return True

    def send(self, phase, detail=None):
        if self._sock is None or self.lost:
            return False
        line = json.dumps({"schema": PROGRESS_SCHEMA, "phase": phase, "detail": detail},
                          sort_keys=True, separators=(",", ":")) + "\n"
        try:
            self._sock.sendall(line.encode("utf-8"))
        except OSError:
            self.lost = True
            return False
        return True

    def close(self):
        if self._sock is not None:
            try:
                self._sock.close()
            except OSError:
                pass
            self._sock = None


def _recv_line(sock, timeout_seconds, limit=MAX_LINE):
    sock.settimeout(timeout_seconds)
    chunks = []
    total = 0
    while True:
        piece = sock.recv(4096)
        if piece == b"":
            raise ExecutorRefusal("HANDSHAKE_DENIED", "channel closed before answer")
        chunks.append(piece)
        total += len(piece)
        if total > limit:
            raise ExecutorRefusal("HANDSHAKE_DENIED", "answer too large")
        if b"\n" in piece:
            break
    return b"".join(chunks).split(b"\n", 1)[0]


def _socket_private(path):
    details = os.lstat(path)
    if not stat.S_ISSOCK(details.st_mode) or details.st_uid != os.getuid() \
            or stat.S_IMODE(details.st_mode) != 0o600:
        return False
    parent = os.lstat(os.path.dirname(path))
    return stat.S_ISDIR(parent.st_mode) and parent.st_uid == os.getuid() and stat.S_IMODE(parent.st_mode) == 0o700


def connect_parent_channel(admission, environ, boot_id, monotonic_ns, answer_timeout=30.0):
    """Production exchange_invocation: single handshake over the runner's private socket."""
    path = environ.get(ENV_SOCKET)
    nonce_hex = environ.get(ENV_NONCE)
    if not path or not nonce_hex or re.fullmatch(r"[0-9a-f]{64}", nonce_hex) is None:
        return Refusal("HANDSHAKE_DENIED", "invocation channel environment missing")
    try:
        if not _socket_private(path):
            return Refusal("HANDSHAKE_DENIED", "socket is not a private same-owner channel")
    except OSError as error:
        return Refusal("HANDSHAKE_DENIED", "socket unavailable: " + type(error).__name__)
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    dir_fd = None
    try:
        sock.settimeout(10.0)
        # Connect through the directory descriptor: the evidence directory may
        # exceed the 108-byte AF_UNIX address limit.
        dir_fd = os.open(os.path.dirname(path), os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
        sock.connect("/proc/self/fd/%d/%s" % (dir_fd, os.path.basename(path)))
        request = {"schema": HANDSHAKE_SCHEMA, "nonce": nonce_hex, "authoritySha256": admission.sha256}
        sock.sendall((json.dumps(request, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8"))
        line = _recv_line(sock, answer_timeout)
        answer = json.loads(line.decode("utf-8"))
        if type(answer) is not dict or answer.get("schema") != ANSWER_SCHEMA:
            raise ExecutorRefusal("HANDSHAKE_DENIED", "malformed answer")
        if "denied" in answer:
            raise ExecutorRefusal("HANDSHAKE_DENIED", str(answer["denied"]))
        payload = answer.get("payload")
        digest = answer.get("sha256")
        shape = accounting.validate_invocation_shape(payload)
        if shape is not None:
            raise ExecutorRefusal("HANDSHAKE_DENIED", shape.detail)
        if store.canonical_digest(payload) != digest:
            raise ExecutorRefusal("HANDSHAKE_DENIED", "payload digest mismatch")
        if not hmac.compare_digest(payload["nonceSha256"], _sha(bytes.fromhex(nonce_hex))):
            raise ExecutorRefusal("HANDSHAKE_DENIED", "nonce digest mismatch")
        if not hmac.compare_digest(payload["authoritySha256"], admission.sha256):
            raise ExecutorRefusal("HANDSHAKE_DENIED", "authority mismatch")
        if payload["bootId"] != boot_id():
            raise ExecutorRefusal("HANDSHAKE_DENIED", "boot mismatch")
        if monotonic_ns() >= payload["outerDeadlineMonotonic"]:
            raise ExecutorRefusal("HANDSHAKE_DENIED", "outer deadline already passed")
        sock.settimeout(None)
        return ParentChannel(sock, payload, digest)
    except ExecutorRefusal as error:
        sock.close()
        return Refusal(error.code, error.detail)
    except (OSError, ValueError) as error:
        sock.close()
        return Refusal("HANDSHAKE_DENIED", type(error).__name__ + ": " + str(error)[:200])
    finally:
        if dir_fd is not None:
            os.close(dir_fd)


# --- production transports -------------------------------------------------------

def _argv_bound(argv, default=30):
    if len(argv) >= 3 and argv[0].endswith("timeout") and argv[2].endswith("s"):
        try:
            return int(argv[2][:-1]) + 1
        except ValueError:
            return default
    return default


def production_run_command(argv, timeout_seconds, service_table=None):
    """Bounded subprocess for service commands; argv is the retained argv.

    service_table maps the fixed executable paths to substitutes. It routes
    OS/service I/O only and is never consulted for any predicate.
    """
    actual = list(argv)
    if service_table:
        actual = [service_table.get(part, part) if index < 6 else part for index, part in enumerate(actual)]
    try:
        completed = subprocess.run(actual, capture_output=True, text=True, timeout=max(1, timeout_seconds),
                                   check=False, stdin=subprocess.DEVNULL)
    except subprocess.TimeoutExpired:
        return CommandResult(argv, None, "", "", timed_out=True)
    except OSError as error:
        return CommandResult(argv, 127, "", type(error).__name__ + ": " + str(error))
    return CommandResult(argv, completed.returncode, completed.stdout, completed.stderr)


def _write_exclusive_bytes(path, data, mode):
    fd = os.open(str(path), os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, mode)
    try:
        view = memoryview(data)
        while view:
            written = os.write(fd, view)
            if written <= 0:
                raise OSError("short write")
            view = view[written:]
        os.fsync(fd)
    finally:
        os.close(fd)
    dirfd = os.open(str(Path(path).parent), os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(dirfd)
    finally:
        os.close(dirfd)


def production_write_exclusive(path, data, mode=0o600, timeout_seconds=25):
    """Exclusive no-follow write plus file and directory fsync in a bounded worker."""
    path = Path(path)
    if path.exists() or path.is_symlink():
        return DurableWriteResult(False, path, error_class="FileExistsError", detail="EVIDENCE_PREEXISTS")
    outcome = run_bounded(lambda: _write_exclusive_bytes(path, data, mode), timeout_seconds)
    if outcome.status == "timeout":
        return DurableWriteResult(False, path, error_class="PersistenceTimeout", detail="PERSISTENCE_TIMEOUT")
    if outcome.status != "completed":
        return DurableWriteResult(False, path, error_class="OSError", detail=outcome.detail)
    return DurableWriteResult(True, path, sha256=_sha(data))


def production_boot_id():
    return Path("/proc/sys/kernel/random/boot_id").read_text().strip()


def production_dependencies(root, service_table=None):
    root = Path(root).resolve()
    environ = dict(os.environ)

    def run_command(argv, timeout_seconds):
        return production_run_command(argv, timeout_seconds, service_table)

    def validate_current(plan, admission, invocation, wall_time_ms=None, financial_plan=None):
        return accounting.validate_executor_context(plan, admission, invocation, root=root,
                                                    wall_time_ms=wall_time_ms, financial_plan=financial_plan)

    def exchange_invocation(admission):
        return connect_parent_channel(admission, environ, production_boot_id, time.monotonic_ns)

    def observe_containment(ownership, deadline_ns, purpose="safety-cleanup"):
        return loan_cleanup.contain_owned(ownership["intent"], ownership.get("proverReceipt"),
                                          ownership.get("unitReceipt"), run_command, deadline_ns, purpose,
                                          unit_started=ownership.get("unitStarted"),
                                          prover_started=ownership.get("proverStarted"),
                                          monotonic_ns=time.monotonic_ns)

    return {
        "root": root,
        "validate_current": validate_current,
        "exchange_invocation": exchange_invocation,
        "run_command": run_command,
        "monotonic": time.monotonic,
        "wall_time_ms": lambda: int(time.time() * 1000),
        "boot_id": production_boot_id,
        "write_exclusive": production_write_exclusive,
        "observe_containment": observe_containment,
        "sleep": time.sleep,
        "python": sys.executable,
        "cleanup_script": str(Path(__file__).resolve().parent / "loan_cleanup.py"),
    }


# --- execution ---------------------------------------------------------------------

class Session(object):
    """State of one invocation; every retained fact lives here until the result."""

    def __init__(self, plan, admission, deps):
        self.plan = plan
        self.admission = admission
        self.deps = deps
        self.root = Path(deps["root"]).resolve()
        self.evidence = []
        self.owners = []
        self.limitations = []
        self.channel = None
        self.invocation = None
        self.invocation_sha256 = None
        self.current = None
        self.status = "PROCESS_UNKNOWN"
        self.failure_code = None
        self.known_failure = False
        self.raw_main_exit = {"kind": "unknown", "code": None}
        self.unit_invocation_id = None
        self.terminal_persisted = False
        self.stop_return_code = None
        self.stop_error_class = None
        self.stop_receipt_persisted = False
        self.containment_complete = False
        self.timer_cancel_rc = None
        self.timer_cancel_receipt = False
        self.timer_armed = False
        self.resources_started = False
        self.prover_started = None
        self.unit_started = None
        self.prover_receipt = None
        self.unit_receipt = None
        self.cleanup_intent = None
        self.cleanup_intent_path = None
        self.cuts = None
        self.parent_lost = False
        self.validated_ns = None
        self.observation_age_ms = None

    # --- clocks -------------------------------------------------------------
    def now_ns(self):
        return int(self.deps["monotonic"]() * NS)

    def remaining_seconds(self, cut_ns, maximum):
        remaining = int((cut_ns - self.now_ns()) / NS)
        return max(1, min(maximum, remaining)) if remaining > 0 else None

    def require_parent(self, stage):
        if self.channel is not None and not self.channel.parent_alive():
            self.parent_lost = True
            raise ExecutorRefusal("PARENT_LOST", stage)

    def require_setup_window(self, stage):
        if self.now_ns() >= self.cuts["setup"]:
            raise ExecutorRefusal("DEADLINE_EXCEEDED", "setup window passed at " + stage)

    # --- persistence -------------------------------------------------------------
    def persist(self, name, payload, kind, mode=0o600, bound_cut=None, raw=None):
        """Exclusive durable write of one evidence artifact; failure is returned, never hidden."""
        path = self.plan.evidence_dir / name if not isinstance(name, Path) else name
        data = raw if raw is not None else (json.dumps(payload, indent=2, sort_keys=True) + "\n").encode("utf-8")
        bound = self.remaining_seconds(bound_cut or self.cuts["cleanup"], MAX_STOP_SECONDS)
        if bound is None:
            # The shared phase cutoff has passed: no new write budget is granted.
            outcome = DurableWriteResult(False, path, error_class="DeadlineExceeded", detail="DEADLINE_EXCEEDED")
        else:
            outcome = self.deps["write_exclusive"](path, data, mode, bound)
        if outcome.persisted:
            self.evidence.append({"kind": kind, "path": str(path.relative_to(self.root)), "sha256": outcome.sha256})
        else:
            self.limitations.append({"artifact": kind, "path": str(path), "error": outcome.error_class,
                                     "detail": outcome.detail})
        return outcome

    # --- commands ----------------------------------------------------------------
    def run(self, argv, bound):
        result = self.deps["run_command"](argv, bound + 1)
        record = {"argv": list(argv), "returnCode": getattr(result, "returncode", None),
                  "timedOut": bool(getattr(result, "timed_out", False)),
                  "stdout": (getattr(result, "stdout", "") or "")[:8192],
                  "stderr": (getattr(result, "stderr", "") or "")[:2048]}
        return result, record


def _refused(session, code, detail):
    session.status = "REFUSED"
    session.failure_code = code
    session.limitations.append({"refusal": code, "detail": detail})


def _unknown(session, code, detail=None):
    if session.known_failure:
        session.limitations.append({"afterKnownFailure": code, "detail": detail})
        return
    if session.status == "REFUSED":
        return
    session.status = "PROCESS_UNKNOWN"
    if session.failure_code is None or session.failure_code in ("MAIN_EXIT_UNAVAILABLE",):
        session.failure_code = code
    session.limitations.append({"unknown": code, "detail": detail})


def _known_failure(session, classification):
    session.status = "PROCESS_FAILED"
    session.failure_code = classification["failureCode"]
    session.raw_main_exit = dict(classification["rawMainExit"])
    session.known_failure = True


def _derive_runtime_plan(session):
    plan = session.plan
    original = plan.financial
    mono = session.now_ns()
    wall = session.deps["wall_time_ms"]()
    deadline_ms = wall + int((session.cuts["operation"] - mono) / 1_000_000)
    runtime = json.loads(plan.financial_bytes.decode("utf-8"))
    runtime["limits"] = dict(runtime["limits"])
    runtime["limits"]["deadlineMs"] = deadline_ms
    changed = [key for key in set(original) | set(runtime) if original.get(key) != runtime.get(key)]
    changed_limits = [key for key in set(original["limits"]) | set(runtime["limits"])
                      if original["limits"].get(key) != runtime["limits"].get(key)]
    if changed != ["limits"] or changed_limits != ["deadlineMs"]:
        raise ExecutorRefusal("PLAN_INVALID", "runtime copy would change more than limits.deadlineMs")
    raw = (json.dumps(runtime, indent=2) + "\n").encode("utf-8")
    outcome = session.persist("runtime-financial-plan.json", None, "runtime-financial-plan",
                              bound_cut=session.cuts["setup"], raw=raw)
    if not outcome.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "runtime financial plan")
    comparison = {"originalPath": plan["financialPlan"]["path"], "originalSha256": plan["financialPlan"]["sha256"],
                  "runtimeSha256": outcome.sha256, "changedFields": ["limits.deadlineMs"],
                  "originalDeadlineMs": original["limits"]["deadlineMs"], "runtimeDeadlineMs": deadline_ms,
                  "wallNowMs": wall, "monotonicNowNs": mono, "operationCutNs": session.cuts["operation"]}
    compared = session.persist("runtime-plan-comparison.json", comparison, "runtime-plan-comparison",
                               bound_cut=session.cuts["setup"])
    if not compared.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "runtime plan comparison")
    return Path(outcome.path), outcome.sha256, deadline_ms


def _check_unoccupied(session):
    bound = session.remaining_seconds(session.cuts["setup"], MAX_SHOW_SECONDS)
    if bound is None:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "no time to check unit occupancy")
    for name, argv in (("unit", loan_cleanup.unit_identity_argv(session.plan["unit"], bound)),
                       ("timer", loan_cleanup.timer_show_argv(session.plan["timerUnit"], bound))):
        result, record = session.run(argv, bound)
        fields = parse_systemctl_show(result.stdout) if result is not None and result.returncode == 0 and result.stdout else None
        if fields is None or fields.get("LoadState") != "not-found":
            session.limitations.append({"occupied": name, "observation": record})
            raise ExecutorRefusal("UNIT_OCCUPIED", name + " name is not free")


def _control_record(session):
    boot = session.deps["boot_id"]()
    if _UUID.fullmatch(boot) is None:
        raise ExecutorRefusal("PROVER_CONTROL_INVALID", "boot id")
    inode = os.stat("/proc/self/ns/time").st_ino
    latest = session.cuts["setup"]
    kill = session.cuts["timer"]
    if session.invocation["outerDeadlineMonotonic"] < kill:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "kill deadline exceeds admitted outer bound")
    text = "%s\n%s\n%d\n%d\n%d\n%s\n" % (PROVER_CONTROL_SCHEMA, boot, inode, latest, kill, session.invocation_sha256)
    raw = text.encode("ascii")
    if len(raw) > 512:
        raise ExecutorRefusal("PROVER_CONTROL_INVALID", "control record too large")
    return raw


def _write_prover_control(session):
    path = session.plan.control_dir / session.plan["prover"]["lifetime"]["controlName"]
    raw = _control_record(session)
    outcome = session.persist(path, None, "prover-control", mode=0o400, bound_cut=session.cuts["setup"], raw=raw)
    if not outcome.persisted:
        raise ExecutorRefusal("PROVER_CONTROL_INVALID", "control write: " + str(outcome.detail))
    return outcome.sha256


def _validate_container(session, control_sha):
    prover = session.plan["prover"]
    bound = session.remaining_seconds(session.cuts["setup"], MAX_SHOW_SECONDS)
    if bound is None:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "container validation")
    argv = timeout_prefix(bound) + [DOCKER, "inspect", "--format", "{{json .}}", prover["containerId"]]
    result, record = session.run(argv, bound)
    if result is None or result.returncode != 0 or getattr(result, "timed_out", False):
        session.limitations.append({"containerInspect": record})
        raise ExecutorRefusal("CONTAINMENT_UNSUPPORTED", "container inspect unavailable")
    try:
        config = json.loads(result.stdout)
    except ValueError:
        raise ExecutorRefusal("CONTAINMENT_UNSUPPORTED", "container inspect malformed")
    wrapper_rel = session.plan["prover"]["lifetime"]["wrapper"]["path"]
    wrapper_host = str(session.plan.wrapper_path)
    control_host = str(session.plan.control_dir)
    lifetime = prover["lifetime"]
    expected = session.current["proverContainer"]
    host = config.get("HostConfig", {}) if isinstance(config, dict) else {}
    state = config.get("State", {}) if isinstance(config, dict) else {}
    mounts = config.get("Mounts", []) if isinstance(config, dict) else []
    problems = []
    # Identities must originate in the reviewed plan and the reviewed expectation.
    if expected["containerId"] != prover["containerId"] or expected["imageDigest"] != prover["imageDigest"] \
            or expected["memoryLimitBytes"] != prover["memoryLimitBytes"]:
        problems.append("expectation-plan-mismatch")
    if config.get("Id") != prover["containerId"]:
        problems.append("id")
    if config.get("Image") != prover["imageDigest"]:
        problems.append("image")
    if state.get("Status") != "created" or state.get("StartedAt") != NEVER_STARTED or state.get("Running"):
        problems.append("already-started")
    if host.get("RestartPolicy", {}).get("Name") != "no":
        problems.append("restart-policy")
    if host.get("PidMode", "") != "" or host.get("Privileged") or host.get("CapAdd"):
        problems.append("namespace-or-privilege")
    if host.get("Memory") != prover["memoryLimitBytes"]:
        problems.append("memory")
    observed_mounts = sorted((str(m.get("Source")), str(m.get("Destination")), not bool(m.get("RW")))
                             for m in mounts if isinstance(m, dict))
    expected_mounts = sorted((m["source"], m["destination"], bool(m["readOnly"])) for m in expected["mounts"])
    if observed_mounts != expected_mounts or len(mounts) != 2:
        problems.append("mounts")
    elif any(not read_only for _, _, read_only in expected_mounts):
        problems.append("writable-mount")
    sources = {m["source"]: m["destination"] for m in expected["mounts"]}
    if sources.get(wrapper_host) is None or sources.get(control_host) is None:
        problems.append("mount-sources")
    else:
        if not str(lifetime["containerControlPath"]).startswith(sources[control_host].rstrip("/") + "/"):
            problems.append("control-path")
        if expected["entrypoint"] != sources[wrapper_host]:
            problems.append("entrypoint-expectation")
    # The full actual argv must equal the reviewed one: control path, separator
    # and the exact pinned proof-server executable and arguments.
    if expected["args"][:2] != [lifetime["containerControlPath"], "--"] or len(expected["args"]) < 3:
        problems.append("expected-args")
    if config.get("Path") != expected["entrypoint"] or config.get("Args") != expected["args"]:
        problems.append("entrypoint-or-args")
    if problems:
        session.limitations.append({"containerConfig": problems})
        raise ExecutorRefusal("CONTAINMENT_UNSUPPORTED", "container config: " + ",".join(problems))
    intent = {"schema": "moriarty.loan-prover-prestart-intent/1", "allocationId": session.plan["allocationId"],
              "containerId": prover["containerId"], "imageDigest": prover["imageDigest"],
              "configSha256": _sha(result.stdout.encode("utf-8")), "wrapperPath": wrapper_rel,
              "expectationSha256": session.current["proverContainerSha256"],
              "wrapperSha256": lifetime["wrapper"]["sha256"], "controlSha256": control_sha,
              "invocationSha256": session.invocation_sha256, "chargeId": session.invocation["chargeId"],
              "reservationId": session.invocation["reservationId"], "observedAtUtc": _now_utc()}
    outcome = session.persist("prover-prestart-intent.json", intent, "prover-prestart-intent",
                              bound_cut=session.cuts["setup"])
    if not outcome.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "pre-start intent")


def _arm_timer(session):
    plan = session.plan
    intent = {
        "schema": loan_cleanup.INTENT_SCHEMA, "allocationId": plan["allocationId"], "unit": plan["unit"],
        "timerUnit": plan["timerUnit"], "containerId": plan["prover"]["containerId"],
        "invocationSha256": session.invocation_sha256, "chargeId": session.invocation["chargeId"],
        "reservationId": session.invocation["reservationId"], "evidenceDirectory": str(plan.evidence_dir),
        "proverReceiptPath": str(plan.evidence_dir / "prover-ownership.json"),
        "unitReceiptPath": str(plan.evidence_dir / "unit-ownership.json"),
    }
    outcome = session.persist("cleanup-intent.json", intent, "cleanup-intent", bound_cut=session.cuts["setup"])
    if not outcome.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "cleanup intent")
    session.cleanup_intent = intent
    session.cleanup_intent_path = Path(outcome.path)
    delay = int((session.cuts["timer"] - session.now_ns()) / NS)
    if delay <= 0:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "timer cutoff already passed")
    argv = loan_cleanup.timer_arm_argv(plan["timerUnit"], delay, session.deps["python"],
                                       session.deps["cleanup_script"], outcome.path, outcome.sha256)
    bound = session.remaining_seconds(session.cuts["setup"], MAX_STOP_SECONDS)
    if bound is None:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "timer arming")
    result, record = session.run(timeout_prefix(bound) + argv, bound)
    session.timer_armed = result is not None and result.returncode == 0 and not getattr(result, "timed_out", False)
    show_bound = session.remaining_seconds(session.cuts["setup"], MAX_SHOW_SECONDS)
    if show_bound is None:
        if session.timer_armed:
            _cancel_timer(session, setup_only=True)
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "timer observation")
    shown, shown_record = session.run(loan_cleanup.timer_show_argv(plan["timerUnit"], show_bound), show_bound)
    fields = parse_systemctl_show(shown.stdout) if shown is not None and shown.returncode == 0 and shown.stdout else None
    # The actual timer target must be the absolute +1620 cutoff on the same
    # monotonic clock (AccuracySec=1s plus one second of arming slack).
    problems = []
    if fields is None or fields.get("LoadState") != "loaded" or fields.get("ActiveState") != "active":
        problems.append("not-active")
    else:
        if fields.get("Unit") != plan["timerUnit"] + ".service":
            problems.append("unit-target")
        next_elapse = fields.get("NextElapseUSecMonotonic")
        if type(next_elapse) is not str or not re.fullmatch(r"[1-9][0-9]*", next_elapse):
            problems.append("next-elapse-malformed")
        elif abs(int(next_elapse) * 1000 - session.cuts["timer"]) > 2 * NS:
            problems.append("next-elapse-off-schedule")
    active = session.timer_armed and not problems
    armed = session.persist("timer-armed.json", {"arm": record, "observation": shown_record, "delaySeconds": delay,
                                                 "cutoffNs": session.cuts["timer"], "active": active,
                                                 "problems": problems},
                            "timer-armed", bound_cut=session.cuts["setup"])
    if not active or not armed.persisted:
        if session.timer_armed:
            _cancel_timer(session, setup_only=True)
        session.timer_armed = False
        if not armed.persisted:
            raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "timer arming evidence")
        raise ExecutorRefusal("CONTAINMENT_UNSUPPORTED", "independent timer could not be armed and observed: "
                              + ",".join(problems))


def _cancel_timer(session, setup_only=False):
    bound = session.remaining_seconds(session.cuts["cleanup"], MAX_STOP_SECONDS)
    if bound is None:
        session.timer_cancel_rc = None
        session.timer_cancel_receipt = False
        session.owners.append({"owner": "timer:" + session.plan["timerUnit"], "resource": "timer",
                               "reason": "cleanup-budget-exhausted"})
        return False
    argv = loan_cleanup.timer_cancel_argv(session.plan["timerUnit"], bound)
    result, record = session.run(argv, bound)
    rc = None if result is None or getattr(result, "timed_out", False) else result.returncode
    session.timer_cancel_rc = rc
    outcome = session.persist("timer-cancel.json" if not setup_only else "timer-cancel-setup.json",
                              {"cancel": record, "setupOnly": setup_only, "observedAtUtc": _now_utc()},
                              "timer-cancel")
    session.timer_cancel_receipt = outcome.persisted
    if rc != 0 or not outcome.persisted:
        session.owners.append({"owner": "timer:" + session.plan["timerUnit"], "resource": "timer",
                               "reason": "TIMER_CANCEL_FAILED"})
        return False
    session.timer_armed = False
    return True


def _require_fresh_observation(session, stage):
    """Immediately before financial startup: the same observation, aged by monotonic elapsed time."""
    current = session.current
    age_ms = current["observationAgeMs"] + int((session.now_ns() - session.validated_ns) / 1_000_000)
    observation_path = current.get("observationPath")
    if observation_path is not None:
        try:
            if _sha(Path(observation_path).read_bytes()) != current["observationSha256"]:
                raise ExecutorRefusal("OBSERVATION_INVALID", "observation changed since validation (no refresh admitted)")
        except OSError:
            raise ExecutorRefusal("OBSERVATION_INVALID", "observation unavailable at " + stage)
    if age_ms > accounting.MAX_OBSERVATION_AGE_SECONDS * 1000:
        raise ExecutorRefusal("OBSERVATION_INVALID", "observation older than %d seconds at %s" % (
            accounting.MAX_OBSERVATION_AGE_SECONDS, stage))


def _start_prover(session):
    prover = session.plan["prover"]
    bound = session.remaining_seconds(session.cuts["setup"], MAX_STOP_SECONDS)
    if bound is None:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "docker start")
    session.resources_started = True
    argv = timeout_prefix(bound) + [DOCKER, "start", prover["containerId"]]
    result, record = session.run(argv, bound)
    if result is None or result.returncode != 0 or getattr(result, "timed_out", False):
        session.limitations.append({"dockerStart": record})
        raise ExecutorRefusal("STARTUP_INVALID", "docker start failed")
    inspect_bound = session.remaining_seconds(session.cuts["setup"], MAX_SHOW_SECONDS)
    if inspect_bound is None:
        raise ExecutorRefusal("STARTUP_INVALID", "no time to observe the prover start")
    inspect, inspect_record = session.run(loan_cleanup.docker_inspect_argv(prover["containerId"], inspect_bound), inspect_bound)
    state = loan_cleanup.parse_container_inspect(inspect.stdout) if inspect is not None and inspect.returncode == 0 else None
    if state is None or state["Id"] != prover["containerId"] or not state["Running"] \
            or loan_cleanup._STARTED_AT.fullmatch(state["StartedAt"]) is None:
        session.limitations.append({"dockerPostStart": inspect_record})
        raise ExecutorRefusal("STARTUP_INVALID", "prover start not observed")
    session.prover_started = state
    receipt = {"schema": loan_cleanup.PROVER_RECEIPT_SCHEMA, "allocationId": session.plan["allocationId"],
               "invocationSha256": session.invocation_sha256, "chargeId": session.invocation["chargeId"],
               "reservationId": session.invocation["reservationId"], "containerId": state["Id"],
               "containerStartedAt": state["StartedAt"], "startArgv": record["argv"],
               "startReturnCode": record["returnCode"], "observedAtUtc": _now_utc()}
    outcome = session.persist("prover-ownership.json", receipt, "prover-ownership", bound_cut=session.cuts["setup"])
    if not outcome.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "prover ownership receipt")
    session.prover_receipt = receipt


def _launch_unit(session, runtime_plan_path):
    plan = session.plan
    command = [str(runtime_plan_path) if part == RUNTIME_PLACEHOLDER else part for part in plan["command"]]
    runtime_max = int((session.cuts["bootstrap"] - session.now_ns()) / NS)
    if runtime_max <= 0:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "unit launch")
    bound = session.remaining_seconds(session.cuts["setup"], MAX_STOP_SECONDS)
    if bound is None:
        raise ExecutorRefusal("DEADLINE_EXCEEDED", "unit launch")
    argv = timeout_prefix(bound) + [SYSTEMD_RUN, "--user", "--unit=" + plan["unit"], "--service-type=exec",
                                    "-p", "RemainAfterExit=yes", "-p", "KillMode=control-group",
                                    "-p", "RuntimeMaxSec=%d" % runtime_max, "--collect", "--", *command]
    result, record = session.run(argv, bound)
    if result is None or result.returncode != 0 or getattr(result, "timed_out", False):
        session.limitations.append({"unitLaunch": record})
        raise ExecutorRefusal("STARTUP_INVALID", "financial unit launch failed")
    launched_ns = session.now_ns()
    show_bound = session.remaining_seconds(session.cuts["setup"], MAX_SHOW_SECONDS)
    if show_bound is None:
        raise ExecutorRefusal("STARTUP_INVALID", "no time to observe the unit startup")
    shown, shown_record = session.run(startup_show_argv(plan["unit"], show_bound), show_bound)
    fields = parse_systemctl_show(shown.stdout) if shown is not None and shown.returncode == 0 and shown.stdout else None
    startup = {"schema": "moriarty.loan-unit-startup/1", "launch": record, "observation": shown_record,
               "observed": fields, "observedAtUtc": _now_utc(), "runtimeMaxSec": runtime_max,
               "launchMonotonicNs": launched_ns, "bootstrapCutNs": session.cuts["bootstrap"], "command": command}
    try:
        invocation_id = require_startup_identity(fields)
        # Activation is anchored to the same monotonic clock. A delayed
        # activation cannot move the +1606 bootstrap exit: the relative
        # RuntimeMaxSec must still end at or before the absolute cutoff.
        activated_us = int(fields["ActiveEnterTimestampMonotonic"])
        now_us = session.now_ns() // 1000
        if activated_us > now_us + 1_000_000 or activated_us < launched_ns // 1000 - 5_000_000:
            raise ValueError("STARTUP_ACTIVATION_TIME")
        if activated_us + runtime_max * 1_000_000 > session.cuts["bootstrap"] // 1000:
            raise ValueError("STARTUP_ACTIVATION_LATE")
    except ValueError as error:
        startup["problem"] = str(error)
        session.persist("unit-startup.json", startup, "unit-startup", bound_cut=session.cuts["setup"])
        raise ExecutorRefusal("STARTUP_INVALID", "loaded startup identity not established: " + str(error))
    outcome = session.persist("unit-startup.json", startup, "unit-startup", bound_cut=session.cuts["setup"])
    if not outcome.persisted:
        session.unit_started = fields
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "startup observation")
    session.unit_started = fields
    session.unit_invocation_id = invocation_id
    receipt = {"schema": loan_cleanup.UNIT_RECEIPT_SCHEMA, "allocationId": plan["allocationId"],
               "invocationSha256": session.invocation_sha256, "chargeId": session.invocation["chargeId"],
               "reservationId": session.invocation["reservationId"], "unit": plan["unit"],
               "unitInvocationId": invocation_id, "unitControlGroup": fields["ControlGroup"],
               "unitMainPid": fields["MainPID"], "observedAtUtc": _now_utc()}
    outcome = session.persist("unit-ownership.json", receipt, "unit-ownership", bound_cut=session.cuts["setup"])
    if not outcome.persisted:
        raise ExecutorRefusal("EVIDENCE_WRITE_FAILED", "unit ownership receipt")
    session.unit_receipt = receipt


def _collect(session):
    plan = session.plan
    collector_dir = plan.evidence_dir / "collector"
    try:
        collector_dir.mkdir(mode=0o700)
    except OSError:
        raise ExecutorRefusal("EVIDENCE_PREEXISTS", "collector directory")
    run_command = session.deps["run_command"]
    transport = UserSystemctlTransport(lambda argv: run_command(argv, _argv_bound(argv)))
    sleep = session.deps.get("sleep", time.sleep)
    writer = session.deps["write_exclusive"]

    def save(path, value, timeout_seconds):
        path = Path(path)
        if path.exists() or path.is_symlink():
            raise FileExistsError("PERSISTENCE_FAILED")
        raw = (json.dumps(value, indent=2) + "\n").encode("utf-8")
        if len(raw) > 65536:
            raise ValueError("RECORD_TOO_LARGE")
        if not timeout_seconds:
            raise PersistenceTimeout("PERSISTENCE_TIMEOUT")
        outcome = writer(path, raw, 0o600, timeout_seconds)
        if not outcome.persisted:
            if outcome.error_class == "PersistenceTimeout":
                raise PersistenceTimeout("PERSISTENCE_TIMEOUT")
            raise OSError("PERSISTENCE_FAILED:" + str(outcome.detail))
    while True:
        session.require_parent("collection")
        show_bound = session.remaining_seconds(session.cuts["collection"], MAX_SHOW_SECONDS)
        if show_bound is None:
            return {"status": "COLLECTION_CUTOFF"}
        stop_bound = session.remaining_seconds(session.cuts["cleanup"], MAX_STOP_SECONDS)
        persist_bound = session.remaining_seconds(session.cuts["cleanup"], MAX_STOP_SECONDS)
        if stop_bound is None or persist_bound is None:
            return {"status": "COLLECTION_CUTOFF"}
        result = retain_loan_main_exit(plan["unit"], session.unit_started, transport, collector_dir,
                                       show_seconds=show_bound, stop_seconds=stop_bound,
                                       persist_timeout_seconds=persist_bound, save=save)
        if result["status"] == "NOT_TERMINAL":
            sleep(1.0)
            continue
        return result


def _record_collector(session, result):
    observation = result.get("terminalObservationPath")
    if observation and result.get("terminalObservationSha256"):
        session.evidence.append({"kind": "terminal-observation",
                                 "path": str(Path(observation).relative_to(session.root)),
                                 "sha256": result["terminalObservationSha256"]})
    stop_path = result.get("explicitStopPath")
    if stop_path and Path(stop_path).is_file():
        session.evidence.append({"kind": "explicit-stop", "path": str(Path(stop_path).relative_to(session.root)),
                                 "sha256": _sha(Path(stop_path).read_bytes())})


def _dispose_collector(session, result):
    """Map the installed collector result onto the closed disposition table."""
    status = result.get("status")
    classification = result.get("classification") or {}
    if status == "COLLECTION_CUTOFF":
        _unknown(session, "MAIN_EXIT_UNAVAILABLE", "collection cutoff with only running observations")
        return
    _record_collector(session, result)
    session.terminal_persisted = bool(result.get("terminalObservationSha256")) and result.get("rejected") not in (
        "PERSISTENCE_FAILED", "PERSISTENCE_TIMEOUT")
    if status == "EXIT_RETAINED_THEN_STOP_COMPLETED":
        session.status = "PROCESS_SUCCESS"
        session.failure_code = None
        session.raw_main_exit = {"kind": "exit", "code": 0}
        session.stop_return_code = result.get("stopReturnCode")
        session.stop_error_class = None
        session.stop_receipt_persisted = True
        return
    if status in ("STOP_RECEIPT_PERSISTENCE_FAILED", "STOP_FAILED_AFTER_RETENTION"):
        session.raw_main_exit = {"kind": "exit", "code": 0}
        session.stop_return_code = result.get("stopReturnCode")
        session.stop_error_class = result.get("stopErrorClass")
        session.stop_receipt_persisted = bool(result.get("stopReceiptPersisted"))
        _unknown(session, "STOP_RECEIPT_FAILED" if status == "STOP_RECEIPT_PERSISTENCE_FAILED" else "STOP_FAILED",
                 result.get("stopError") or result.get("stopReceiptError"))
        return
    # RETENTION_FAILED
    rejected = result.get("rejected")
    if classification.get("status") == "PROCESS_FAILED":
        _known_failure(session, classification)
        if result.get("persistFailed"):
            session.limitations.append({"terminalPersistence": rejected})
        return
    if rejected in ("PERSISTENCE_FAILED", "PERSISTENCE_TIMEOUT") or result.get("persistFailed"):
        code = "EVIDENCE_PREEXISTS" if rejected == "PERSISTENCE_FAILED" and not session.terminal_persisted \
            and (session.plan.evidence_dir / "collector" / "terminal-observation.json").exists() \
            and classification.get("status") is None else "EVIDENCE_WRITE_FAILED"
        _unknown(session, code, rejected)
        return
    if rejected == "STOP_EVIDENCE_PRESENT":
        _unknown(session, "EVIDENCE_PREEXISTS", rejected)
        return
    if classification.get("status") == "PROCESS_UNKNOWN":
        _unknown(session, classification.get("failureCode") or "MAIN_OBSERVATION_INVALID", rejected)
        return
    _unknown(session, "MAIN_OBSERVATION_INVALID", rejected)


def _contain(session, purpose):
    """S or normal cleanup: identity-checked, bounded, independently observed, durably retained."""
    if not session.resources_started:
        session.containment_complete = True
        return
    ownership = {"intent": session.cleanup_intent, "proverReceipt": session.prover_receipt,
                 "unitReceipt": session.unit_receipt, "unitStarted": session.unit_started,
                 "proverStarted": session.prover_started}
    if session.cleanup_intent is None:
        session.owners.append({"owner": "parent", "resource": "all", "reason": "CLEANUP_INTENT_UNAVAILABLE"})
        session.containment_complete = False
        return
    if session.remaining_seconds(session.cuts["cleanup"], 1) is None:
        session.owners.append({"owner": "parent", "resource": "all", "reason": "cleanup-budget-exhausted"})
        session.containment_complete = False
        _unknown(session, "CONTAINMENT_UNRESOLVED", "cleanup cutoff passed before containment")
        return
    facts = session.deps["observe_containment"](ownership, session.cuts["cleanup"], purpose)
    outcome = session.persist("containment-%s.json" % purpose, facts, "containment")
    complete = bool(facts.get("complete")) and outcome.persisted
    session.containment_complete = complete
    for owner in facts.get("outstandingOwners", []):
        session.owners.append(owner)
    if not outcome.persisted:
        session.owners.append({"owner": "parent", "resource": "containment-evidence", "reason": "EVIDENCE_WRITE_FAILED"})
    if not complete:
        _unknown(session, "CONTAINMENT_UNRESOLVED", facts.get("outstandingOwners"))


def _compose_result(session):
    invocation = session.invocation or {}
    return {
        "schema": RESULT_SCHEMA,
        "allocationId": session.plan["allocationId"],
        "actionId": session.plan["actionId"],
        "candidateHash": session.plan["candidateHash"],
        "runnerDigest": invocation.get("runnerDigest"),
        "chargeId": invocation.get("chargeId"),
        "reservationId": invocation.get("reservationId"),
        "invocationSha256": session.invocation_sha256,
        "unit": session.plan["unit"],
        "invocationId": session.unit_invocation_id,
        "status": session.status,
        "rawMainExit": session.raw_main_exit,
        "terminalEvidencePersisted": session.terminal_persisted,
        "stopReturnCode": session.stop_return_code,
        "stopErrorClass": session.stop_error_class,
        "stopReceiptPersisted": session.stop_receipt_persisted,
        "containmentComplete": session.containment_complete,
        "timerCancelReturnCode": session.timer_cancel_rc,
        "timerCancelReceiptPersisted": session.timer_cancel_receipt,
        "failureCode": session.failure_code,
        "evidence": session.evidence,
        "outstandingOwners": [{"owner": str(o["owner"]), "resource": str(o["resource"]), "reason": str(o["reason"])}
                              for o in session.owners],
        "retryAllowed": False,
        "financialAcceptance": "pending",
    }


def _write_result(session):
    document = _compose_result(session)
    data = (json.dumps(document, indent=2, sort_keys=True) + "\n").encode("utf-8")
    if session.invocation is not None:
        # The result is due by +1740; the outer deadline keeps the last margin.
        outer = session.invocation["outerDeadlineMonotonic"]
        bound = session.remaining_seconds(outer, MAX_SHOW_SECONDS)
        if bound is None:
            return document, DurableWriteResult(False, session.plan.result_path, error_class="DeadlineExceeded",
                                                detail="DEADLINE_EXCEEDED")
    else:
        bound = MAX_SHOW_SECONDS
    outcome = session.deps["write_exclusive"](session.plan.result_path, data, 0o600, bound)
    return document, outcome


def execute_once(plan, admission, dependencies):
    """One authenticated invocation. Returns {exitCode, result, resultPersisted, resultSha256}."""
    if not __debug__:
        raise RuntimeError("ASSERTIONS_REQUIRED")
    deps = dependencies
    session = Session(plan, admission, deps)
    channel = None
    try:
        exchanged = deps["exchange_invocation"](admission)
        if isinstance(exchanged, Refusal):
            raise ExecutorRefusal(exchanged.code, exchanged.detail)
        channel = exchanged
        session.channel = channel
        session.invocation = channel.invocation
        session.invocation_sha256 = channel.invocation_sha256
        start = channel.invocation["outerStartMonotonic"]
        limits = plan["limits"]
        session.cuts = {
            "setup": start + limits["setupSeconds"] * NS,
            "operation": start + limits["operationDeadlineSeconds"] * NS,
            "bootstrap": start + limits["bootstrapExitSeconds"] * NS,
            "collection": start + limits["collectionDeadlineSeconds"] * NS,
            "timer": start + limits["timerSeconds"] * NS,
            "cleanup": start + limits["cleanupSeconds"] * NS,
        }
        if channel.invocation["outerDeadlineMonotonic"] < start + limits["outerSeconds"] * NS:
            raise ExecutorRefusal("DEADLINE_EXCEEDED", "block deadline shortens the outer window below the fixed schedule")
        if deps["boot_id"]() != channel.invocation["bootId"]:
            raise ExecutorRefusal("AUTHORITY_INVALID", "boot mismatch")
        session.require_setup_window("entry")
        session.require_parent("entry")
        current = deps["validate_current"](plan, admission, channel.invocation,
                                           wall_time_ms=deps["wall_time_ms"](), financial_plan=plan.financial)
        if isinstance(current, Refusal):
            raise ExecutorRefusal(current.code, current.detail)
        session.current = current
        session.validated_ns = session.now_ns()
        if current["invocationSha256"] != channel.invocation_sha256:
            raise ExecutorRefusal("CLAIM_INVALID", "store event digest differs from the exchanged payload")
        session.require_setup_window("validated")
        session.require_parent("validated")
        channel.send("validated", {"invocationSha256": channel.invocation_sha256})
        runtime_plan_path, runtime_sha, deadline_ms = _derive_runtime_plan(session)
        session.require_setup_window("runtime-plan")
        _check_unoccupied(session)
        session.require_setup_window("occupancy")
        session.require_parent("occupancy")
        control_sha = _write_prover_control(session)
        _validate_container(session, control_sha)
        session.require_setup_window("prover-config")
        session.require_parent("prover-config")
        _arm_timer(session)
        session.require_setup_window("timer")
        session.require_parent("timer")
        _require_fresh_observation(session, "resource-start")
        channel.send("startup", {"timerArmed": True})
        # Resource start: from here on cleanup owns every started resource.
        try:
            _start_prover(session)
            session.require_setup_window("prover-start")
            session.require_parent("prover-start")
            _require_fresh_observation(session, "financial-startup")
            _launch_unit(session, runtime_plan_path)
            session.require_setup_window("unit-start")
            channel.send("launched", {"unit": plan["unit"], "invocationId": session.unit_invocation_id})
            collected = _collect(session)
            _dispose_collector(session, collected)
        except ExecutorRefusal as error:
            if error.code == "PARENT_LOST":
                session.limitations.append({"parentLost": error.detail})
            _unknown(session, error.code, error.detail)
        purpose = "normal-cleanup" if session.status == "PROCESS_SUCCESS" else "safety-cleanup"
        _contain(session, purpose)
        if session.containment_complete:
            if session.timer_armed and not _cancel_timer(session):
                _unknown(session, "TIMER_CANCEL_FAILED", "timer remains outstanding")
        else:
            session.owners.append({"owner": "timer:" + plan["timerUnit"], "resource": "timer",
                                   "reason": "retained until containment"})
        if session.status == "PROCESS_SUCCESS" and (session.owners or not session.containment_complete
                                                    or session.timer_cancel_rc != 0 or not session.timer_cancel_receipt):
            session.status = "PROCESS_UNKNOWN"
            session.failure_code = session.failure_code or "CONTAINMENT_UNRESOLVED"
    except ExecutorRefusal as error:
        if session.resources_started:
            _unknown(session, error.code, error.detail)
            _contain(session, "safety-cleanup")
        else:
            _refused(session, error.code, error.detail)
            if session.timer_armed:
                _cancel_timer(session, setup_only=True)
            session.containment_complete = True
    document, outcome = _write_result(session)
    exit_code = STATUS_EXIT.get(session.status, 3)
    if not outcome.persisted:
        exit_code = 3
        if channel is not None:
            channel.send("result-write-failed", {"error": outcome.error_class, "detail": outcome.detail})
    else:
        if channel is not None:
            channel.send("contained" if session.containment_complete else "unresolved",
                         {"status": session.status, "failureCode": session.failure_code})
            channel.send("result", {"path": str(plan.result_path), "sha256": outcome.sha256})
    if channel is not None:
        channel.close()
    return {"exitCode": exit_code, "result": document, "resultPersisted": outcome.persisted,
            "resultSha256": outcome.sha256, "resultPath": str(plan.result_path),
            "limitations": session.limitations}


def build_parser():
    parser = argparse.ArgumentParser(prog="loan_executor", description="One authenticated loan invocation")
    parser.add_argument("--plan", required=True, help="contained executor plan path")
    parser.add_argument("--sha256", required=True, help="plan digest")
    parser.add_argument("--admission", required=True, help="contained immutable resource amendment path")
    return parser


def main(argv=None, dependencies=None):
    if not __debug__:
        sys.stderr.write("ASSERTIONS_REQUIRED\n")
        return 2
    args = build_parser().parse_args(argv)
    root = Path(dependencies["root"]).resolve() if dependencies else Path.cwd().resolve()
    try:
        plan = load_plan(root, args.plan, args.sha256)
        admission = load_admission(root, args.admission, plan)
    except ExecutorRefusal as error:
        sys.stderr.write("refused before invocation: %s\n" % error)
        return 2
    deps = dependencies or production_dependencies(root)
    outcome = execute_once(plan, admission, deps)
    sys.stdout.write(json.dumps({"status": outcome["result"]["status"], "failureCode": outcome["result"]["failureCode"],
                                 "resultPath": outcome["resultPath"], "resultPersisted": outcome["resultPersisted"]}) + "\n")
    return outcome["exitCode"]


if __name__ == "__main__":
    raise SystemExit(main())
