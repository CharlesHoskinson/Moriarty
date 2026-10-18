"""Adapter for a campaign-bound, already charged Foreman invocation.

Campaign binding bytes authorize the command. The existing budget runner must
record its debit before calling the CLI; this adapter never changes that budget.
Child stdout is data, never authority for charges or completion metadata.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import signal
import socket
import subprocess
import time

from moriarty_dev import records

FOREMAN_LAUNCHER = Path("/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js")


def _contained(root, relative):
    missing = []
    path = records._contained_file(root, relative, missing, "runner")
    if path is None:
        raise ValueError("runner path unavailable: " + ", ".join(missing))
    return path


def _json(path):
    missing = []
    result = records._load_json_path(path, missing, "runner")
    if type(result) is not dict or missing:
        raise ValueError("runner record unavailable: " + ", ".join(missing))
    return result


def _hash(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def _check_files(root, mapping, executable):
    if type(mapping) is not dict or executable not in mapping:
        raise ValueError("runner executable is not hash-bound")
    for name, expected in mapping.items():
        if type(name) is not str or type(expected) is not str or len(expected) != 64:
            raise ValueError("invalid runner file commitment")
        path = Path(name)
        if path.is_absolute():
            # External inputs are limited to the interpreter and Foreman itself.
            if name != executable and path != FOREMAN_LAUNCHER:
                raise ValueError("unapproved external runner input")
        else:
            path = _contained(root, name)
        if not path.is_file() or _hash(path) != expected:
            raise ValueError("runner file changed: " + name)


def load_runner(root, action):
    """Resolve through the same campaign and hashed binding as source admission."""
    root = Path(root).resolve()
    program = _json(_contained(root, "openspec/moriarty-completion-program.json"))
    store = program["reportReconciliation"]["stageAdmission"]["campaignRecordStore"]
    campaigns = _json(_contained(root, store))["campaigns"]
    if not action["admissionRef"].startswith("campaign:"):
        raise ValueError("runner requires existing campaign authority")
    campaign_id = action["admissionRef"][len("campaign:"):]
    campaign = campaigns[campaign_id]
    missing = []
    valid, binding = records._verify_binding(root, campaign_id, campaign, missing)
    if not valid or not records._verify_candidate(root, campaign_id, campaign, missing):
        raise ValueError("runner admission changed: " + ", ".join(missing))
    plan = binding.get("runners", {}).get(action["id"])
    if type(plan) is not dict:
        raise ValueError("no authenticated admitted-runner authority is registered")
    required = {"schema", "action", "argv", "files", "launcher", "launcherFiles",
                "timeoutSeconds", "graceSeconds", "outputLimitBytes", "chargeId", "chargedSeconds", "accountingPath"}
    if set(plan) - (required | {"assertion", "executor"}) or not required.issubset(plan):
        raise ValueError("unsupported runner binding fields")
    if plan["schema"] != "moriarty.bound-runner/1" or plan["action"] != action or campaign["candidateHash"] != action["candidate"]:
        raise ValueError("runner action/candidate binding mismatch")
    ref, fragment = action["commandRef"].split("#", 1)
    entry = _json(_contained(root, ref))["commands"][fragment]
    argv = plan["argv"]
    if type(argv) is not list or not argv or any(type(x) is not str or not x for x in argv):
        raise ValueError("invalid bound argv")
    if entry != {"argv": argv}:
        raise ValueError("command mapping differs from admitted runner")
    if not Path(argv[0]).is_absolute() or not os.access(argv[0], os.X_OK):
        raise ValueError("runner executable must be an absolute executable path")
    _check_files(root, plan["files"], argv[0])
    for arg in argv[1:]:
        if not arg.startswith("-") and (root / arg).is_file() and arg not in plan["files"]:
            raise ValueError("argv file is not hash-bound: " + arg)
    launcher = plan["launcher"]
    if type(launcher) is not list or len(launcher) != 2 or launcher[1] != str(FOREMAN_LAUNCHER):
        raise ValueError("runner must use the existing Foreman launcher")
    if not Path(launcher[0]).is_absolute() or not os.access(launcher[0], os.X_OK):
        raise ValueError("launcher executable must be absolute")
    if str(FOREMAN_LAUNCHER) not in plan["launcherFiles"]:
        raise ValueError("Foreman bytes are not hash-bound")
    _check_files(root, plan["launcherFiles"], launcher[0])
    _validate_executor_entry(root, plan, argv, campaign)
    for key, low, high in (("timeoutSeconds", 1, 1800), ("graceSeconds", 1, 10), ("outputLimitBytes", 256, 1048576)):
        if type(plan[key]) is not int or not low <= plan[key] <= high:
            raise ValueError("invalid runner bound: " + key)
    if type(plan["chargedSeconds"]) is not int or plan["chargedSeconds"] < plan["timeoutSeconds"] + plan["graceSeconds"] + 30:
        raise ValueError("charge does not cover startup, execution and cleanup")
    if type(plan["chargeId"]) is not str or not plan["chargeId"].strip():
        raise ValueError("runner charge identity unavailable")
    assertion = plan.get("assertion")
    if action["kind"] == "reproduce":
        if type(assertion) is not dict or set(assertion) != {"id", "exitCode", "stdoutSha256"}:
            raise ValueError("reproducer requires an admitted behavioral assertion")
        if type(assertion["id"]) is not str or not assertion["id"].strip():
            raise ValueError("empty behavioral assertion ID")
        if type(assertion["exitCode"]) is not int or not 2 <= assertion["exitCode"] <= 123:
            raise ValueError("behavioral exit must differ from ordinary failure and timeout")
        if type(assertion["stdoutSha256"]) is not str or len(assertion["stdoutSha256"]) != 64:
            raise ValueError("behavioral output commitment unavailable")
    elif assertion is not None:
        raise ValueError("assertion supplied for a non-reproducer")
    digest = hashlib.sha256(json.dumps(plan, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    if campaign["currentAccounting"]["path"] != plan["accountingPath"]:
        raise ValueError("charge source differs from current campaign accounting")
    budget = _json(_contained(root, plan["accountingPath"]))
    charges = budget["charges"] + budget["externalPackageCharges"]
    matching = [row for row in charges if row.get("id") == plan["chargeId"]]
    if len(matching) != 1:
        raise ValueError("exactly one existing debit is required")
    charge = matching[0]
    expected = {"seconds": plan["chargedSeconds"], "candidateHash": action["candidate"],
                "actionId": action["id"], "runnerDigest": digest}
    if any(charge.get(k) != v for k, v in expected.items()):
        raise ValueError("existing debit is not bound to this runner/action/candidate")
    return plan, digest


def execute(root, plan, digest, reservation_id):
    """Observe the actual process; retain bounded output while hashing all bytes."""
    if plan.get("executor") is not None:
        return execute_loan_invocation(root, plan, digest, reservation_id)
    command = [*plan["launcher"], "--timeout", str(plan["timeoutSeconds"]),
               "--grace", str(plan["graceSeconds"]), "--require-containment", "strong", "--", *plan["argv"]]
    started = time.monotonic()
    proc = subprocess.Popen(command, cwd=root, stdin=subprocess.DEVNULL,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    selector = selectors.DefaultSelector()
    streams = {"stdout": bytearray(), "stderr": bytearray()}
    hashes = {name: hashlib.sha256() for name in streams}
    sizes = {name: 0 for name in streams}
    for pipe, name in ((proc.stdout, "stdout"), (proc.stderr, "stderr")):
        selector.register(pipe, selectors.EVENT_READ, name)
    limit = plan["outputLimitBytes"]
    ambiguous = False
    try:
        while selector.get_map() or proc.poll() is None:
            if time.monotonic() - started > plan["timeoutSeconds"] + plan["graceSeconds"] + 30:
                ambiguous = True
                os.killpg(proc.pid, signal.SIGKILL)
                break
            for key, _ in selector.select(0.1):
                chunk = os.read(key.fileobj.fileno(), 16384)
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                name = key.data
                hashes[name].update(chunk)
                sizes[name] += len(chunk)
                streams[name].extend(chunk[:max(0, limit - len(streams[name]))])
        ret = proc.wait(timeout=5)
        # A signal, launcher failure, or namespace signal encoding does not
        # establish completion. Foreman's confirmed timeout uses exit 124.
        ambiguous = ambiguous or ret < 0 or ret >= 125
    finally:
        selector.close()
        proc.stdout.close()
        proc.stderr.close()
    overflow = any(size > limit for size in sizes.values())
    assertion = plan.get("assertion")
    verified = bool(not ambiguous and not overflow and assertion and ret == assertion["exitCode"]
                    and hashes["stdout"].hexdigest() == assertion["stdoutSha256"])
    observed = {"actionId": plan["action"]["id"], "candidateHash": plan["action"]["candidate"],
                "runnerDigest": digest, "chargeId": plan["chargeId"], "reservationId": reservation_id,
                "argv": command, "exitCode": ret, "elapsedSeconds": time.monotonic() - started,
                "outputSha256": {name: h.hexdigest() for name, h in hashes.items()}, "outputBytes": sizes,
                "completionAmbiguous": ambiguous, "outputLimitExceeded": overflow,
                "behavioralAssertions": [{"assertionId": assertion["id"], "candidateHash": plan["action"]["candidate"],
                                          "outcome": "defect-observed"}] if verified else []}
    return {"exitCode": ret, "stdout": streams["stdout"].decode("utf-8", errors="replace"),
            "stderr": streams["stderr"].decode("utf-8", errors="replace"), "runnerReceipt": observed}


# --- loan executor invocation profile ------------------------------------------
# A bound runner may carry an optional closed ``executor`` entry naming the
# immutable executor plan and its resource-amendment admission. Its argv tail
# is fixed to ``--plan P --sha256 D --admission A``; both files are hash-bound
# in ``files``. Legacy runners without this entry keep the original behavior.
EXECUTOR_SCHEMA = "moriarty.loan-executor-launch/1"
EXECUTOR_KEYS = frozenset(("schema", "plan", "sha256", "admission"))
EXECUTOR_TIMEOUT_SECONDS = 1800
EXECUTOR_GRACE_SECONDS = 5
ENV_SOCKET = "MORIARTY_INVOCATION_SOCKET"
ENV_NONCE = "MORIARTY_INVOCATION_NONCE"
ANCESTRY_LIMIT = 64
SETUP_SECONDS = 120
APPEND_BOUND_SECONDS = 30  # bounded worker for the durable invocation append


class LaunchRefused(ValueError):
    """Authority or transport refusal before any child was started."""


def _validate_executor_entry(root, plan, argv, campaign):
    executor = plan.get("executor")
    if executor is None:
        return
    if type(executor) is not dict or set(executor) != EXECUTOR_KEYS or executor["schema"] != EXECUTOR_SCHEMA:
        raise ValueError("unsupported executor launch fields")
    for key in ("plan", "sha256", "admission"):
        if type(executor[key]) is not str or not executor[key]:
            raise ValueError("executor launch " + key + " unavailable")
    if len(executor["sha256"]) != 64 or executor["sha256"] != executor["sha256"].lower():
        raise ValueError("executor plan digest must be lowercase SHA-256")
    tail = ["--plan", executor["plan"], "--sha256", executor["sha256"], "--admission", executor["admission"]]
    if argv[-6:] != tail:
        raise ValueError("executor argv tail differs from the bound launch entry")
    plan_path = _contained(root, executor["plan"])
    if _hash(plan_path) != executor["sha256"]:
        raise ValueError("executor plan bytes changed")
    _contained(root, executor["admission"])
    if campaign.get("resourceAmendment") != executor["admission"]:
        raise ValueError("executor admission is not the campaign's bound resource amendment")
    if plan["timeoutSeconds"] != EXECUTOR_TIMEOUT_SECONDS or plan["graceSeconds"] != EXECUTOR_GRACE_SECONDS:
        raise ValueError("executor runner must use the fixed 1800/5 outer bounds")
    if "assertion" in plan:
        raise ValueError("executor launch cannot carry a behavioral assertion")


def _proc_stat(pid):
    try:
        text = Path("/proc/%d/stat" % pid).read_text()
    except OSError:
        return None
    end = text.rfind(")")
    if end < 0:
        return None
    fields = text[end + 2:].split()
    try:
        return {"ppid": int(fields[1]), "startTicks": int(fields[19])}
    except (IndexError, ValueError):
        return None


def _start_ticks(pid):
    info = _proc_stat(pid)
    return None if info is None else info["startTicks"]


def _descends_from(peer_pid, launcher_pid, launcher_ticks):
    """Bounded /proc ancestry: peer must be the launcher or one of its descendants."""
    if launcher_ticks is None or _start_ticks(launcher_pid) != launcher_ticks:
        return False, "launcher start ticks changed or unavailable"
    pid = peer_pid
    for _ in range(ANCESTRY_LIMIT):
        if pid == launcher_pid:
            return True, "ancestry depth ok"
        info = _proc_stat(pid)
        if info is None or info["ppid"] <= 0:
            return False, "peer ancestry does not reach the launcher"
        pid = info["ppid"]
    return False, "ancestry deeper than the bound"


def _peer_credentials(conn):
    import struct
    raw = conn.getsockopt(socket.SOL_SOCKET, socket.SO_PEERCRED, struct.calcsize("3i"))
    pid, uid, gid = struct.unpack("3i", raw)
    return pid, uid, gid


def _recv_line(conn, timeout, limit=65536):
    conn.settimeout(timeout)
    chunks = []
    total = 0
    while True:
        piece = conn.recv(4096)
        if piece == b"":
            return None
        chunks.append(piece)
        total += len(piece)
        if total > limit:
            return None
        if b"\n" in piece:
            return b"".join(chunks).split(b"\n", 1)[0]


def _send_json(conn, payload):
    conn.sendall((json.dumps(payload, sort_keys=True, separators=(",", ":")) + "\n").encode("utf-8"))


RESULT_MAX_BYTES = 8 * 1024 * 1024
EVIDENCE_MAX_BYTES = 8 * 1024 * 1024
FAILURE_CODES_FAILED = frozenset(("MAIN_EXIT_NONZERO", "MAIN_SIGNAL"))
FAILURE_CODES_UNKNOWN = frozenset((
    "STARTUP_INVALID", "EVIDENCE_WRITE_FAILED", "MAIN_OBSERVATION_INVALID", "EVIDENCE_PREEXISTS",
    "MAIN_OBSERVATION_UNAVAILABLE", "MAIN_EXIT_UNAVAILABLE", "STOP_FAILED", "STOP_RECEIPT_FAILED",
    "DEADLINE_EXCEEDED", "PARENT_LOST", "CONTAINMENT_UNRESOLVED", "TIMER_CANCEL_FAILED", "RESULT_WRITE_FAILED",
))
FAILURE_CODES_REFUSED = frozenset((
    "AUTHORITY_INVALID", "HISTORY_UNRESOLVED", "OBSERVATION_INVALID", "OWNERSHIP_UNRESOLVED", "CLAIM_INVALID",
    "CONTAINMENT_UNSUPPORTED", "DEADLINE_EXCEEDED", "PLAN_INVALID", "UNIT_OCCUPIED", "PROVER_CONTROL_INVALID",
    "EVIDENCE_WRITE_FAILED", "HANDSHAKE_DENIED", "PARENT_LOST", "EVIDENCE_PREEXISTS",
))
SETUP_EVIDENCE_KINDS = ("runtime-financial-plan", "runtime-plan-comparison", "prover-control",
                        "prover-prestart-intent", "cleanup-intent", "timer-armed")
STARTED_EVIDENCE_KINDS = ("prover-ownership", "unit-startup", "unit-ownership")
SUCCESS_EVIDENCE_KINDS = SETUP_EVIDENCE_KINDS + STARTED_EVIDENCE_KINDS + (
    "terminal-observation", "explicit-stop", "containment", "timer-cancel")
FAILED_EVIDENCE_KINDS = SETUP_EVIDENCE_KINDS + STARTED_EVIDENCE_KINDS + ("terminal-observation",)


def _read_regular_bounded(path, limit):
    """No-follow, nonblocking open; the descriptor must be a regular file within limit."""
    import stat as _stat
    try:
        fd = os.open(str(path), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC)
    except OSError as error:
        return None, "unavailable: " + type(error).__name__
    try:
        details = os.fstat(fd)
        if not _stat.S_ISREG(details.st_mode):
            return None, "not a regular file"
        if details.st_size > limit:
            return None, "too large"
        chunks = []
        remaining = limit + 1
        while remaining > 0:
            chunk = os.read(fd, min(65536, remaining))
            if not chunk:
                break
            chunks.append(chunk)
            remaining -= len(chunk)
    except OSError as error:
        return None, "read failed: " + type(error).__name__
    finally:
        os.close(fd)
    data = b"".join(chunks)
    if len(data) > limit:
        return None, "too large"
    return data, None


def _is_bool(value):
    return type(value) is bool


def _is_return_code(value):
    return value is None or (type(value) is int and 0 <= value <= 255)


def _read_evidence(root, exec_plan, item, missing):
    if type(item) is not dict or set(item) != {"kind", "path", "sha256"} \
            or any(type(v) is not str for v in item.values()) or not records._hex64(item["sha256"]):
        return None, "evidence entry malformed"
    rel = item["path"]
    if rel.startswith("/") or ".." in rel.split("/"):
        return None, "evidence path not contained"
    path = root / rel
    resolved = path.resolve()
    allowed = (exec_plan.evidence_dir, exec_plan.control_dir)
    if not any(resolved == base or resolved.is_relative_to(base) for base in allowed):
        return None, "evidence outside the evidence/control directories: " + rel
    data, problem = _read_regular_bounded(path, EVIDENCE_MAX_BYTES)
    if data is None:
        return None, "evidence " + rel + " " + problem
    if hashlib.sha256(data).hexdigest() != item["sha256"]:
        return None, "evidence digest stale: " + rel
    if item["kind"] == "prover-control":
        return {"raw": data}, None
    try:
        payload = json.loads(data.decode("utf-8"))
    except (UnicodeDecodeError, ValueError):
        return None, "evidence is not JSON: " + rel
    if type(payload) is not dict:
        return None, "evidence is not an object: " + rel
    return payload, None


def _validate_evidence_contents(document, evidence, identities, plan_container):
    """Cross-check the retained artifacts against the document's claims."""
    status = document["status"]
    invocation_id = document["invocationId"]
    sha = document["invocationSha256"]
    charge = document["chargeId"]
    reservation = document["reservationId"]
    required = SUCCESS_EVIDENCE_KINDS if status == "PROCESS_SUCCESS" else FAILED_EVIDENCE_KINDS if status == "PROCESS_FAILED" else ()
    for kind in required:
        if kind not in evidence:
            return "required evidence missing: " + kind
    if status == "PROCESS_FAILED" and document["containmentComplete"] and "containment" not in evidence:
        return "containment claimed without containment evidence"
    startup = evidence.get("unit-startup")
    if startup is not None:
        observed = startup.get("observed")
        if type(observed) is not dict or observed.get("InvocationID") != invocation_id \
                or not str(observed.get("ControlGroup", "")).startswith("/") \
                or not re.fullmatch(r"[1-9][0-9]*", str(observed.get("ActiveEnterTimestampMonotonic", ""))):
            return "unit-startup evidence does not establish the loaded startup identity"
    for kind in ("unit-ownership", "prover-ownership", "cleanup-intent", "prover-prestart-intent"):
        record = evidence.get(kind)
        if record is None:
            continue
        if record.get("invocationSha256") != sha or record.get("chargeId") != charge or record.get("reservationId") != reservation:
            return kind + " evidence binds other invocation identities"
    unit_receipt = evidence.get("unit-ownership")
    if unit_receipt is not None and unit_receipt.get("unitInvocationId") != invocation_id:
        return "unit-ownership evidence names another invocation"
    prover_receipt = evidence.get("prover-ownership")
    if prover_receipt is not None and prover_receipt.get("containerId") != plan_container:
        return "prover-ownership evidence names another container"
    terminal = evidence.get("terminal-observation")
    if terminal is not None:
        classification = terminal.get("classification") or {}
        observed = terminal.get("observed") or {}
        if terminal.get("startupInvocationId") != invocation_id or observed.get("InvocationID") != invocation_id:
            return "terminal-observation evidence names another invocation"
        if status == "PROCESS_SUCCESS":
            predicate = terminal.get("predicate") or {}
            if classification.get("status") != "PROCESS_SUCCESS" or predicate.get("status") != "EXIT_ZERO_OBSERVED" \
                    or classification.get("rawMainExit") != {"kind": "exit", "code": 0}:
                return "terminal-observation evidence does not establish the exact zero main exit"
        if status == "PROCESS_FAILED":
            if classification.get("status") != "PROCESS_FAILED" or classification.get("rawMainExit") != document["rawMainExit"] \
                    or classification.get("failureCode") != document["failureCode"]:
                return "terminal-observation evidence disagrees with the claimed main failure"
    stop = evidence.get("explicit-stop")
    if status == "PROCESS_SUCCESS":
        if stop is None or stop.get("returnCode") != 0 or stop.get("stopErrorClass") is not None \
                or stop.get("terminalBeforeStopSha256") != identities["terminalSha256"]:
            return "explicit-stop evidence does not establish a clean stop after the durable terminal"
        containment = evidence.get("containment")
        if containment is None or containment.get("complete") is not True or containment.get("outstandingOwners"):
            return "containment evidence is not complete"
        cancel = evidence.get("timer-cancel")
        if cancel is None or (cancel.get("cancel") or {}).get("returnCode") != 0 or cancel.get("setupOnly") is not False:
            return "timer-cancel evidence does not establish cancellation"
    if status == "PROCESS_FAILED" and document["containmentComplete"]:
        containment = evidence.get("containment")
        if containment is None or containment.get("complete") is not True or containment.get("outstandingOwners"):
            return "containment evidence is not complete"
    return None


def _read_result_document(root, exec_plan, digest, reservation_id, invocation_sha256, plan):
    """Bounded no-follow read and strict validation of the durable result; stdout is never consulted."""
    from moriarty_dev import loan_executor
    missing = []
    path = exec_plan.result_path
    data, problem = _read_regular_bounded(path, RESULT_MAX_BYTES)
    if data is None:
        return None, "result " + problem
    resolved = path.resolve()
    if not resolved.is_relative_to(root) or resolved.parent != exec_plan.evidence_dir:
        return None, "result escaped the evidence directory"
    document = records._parse_json(data, missing, "loan-result")
    if type(document) is not dict or not records._exact_keys(document, loan_executor.RESULT_KEYS, missing, "loan-result"):
        return None, "result fields: " + ", ".join(missing)
    if document["schema"] != loan_executor.RESULT_SCHEMA:
        return None, "result schema"
    status = document["status"]
    if status not in loan_executor.STATUS_EXIT:
        return None, "result status"
    expected = {"allocationId": exec_plan["allocationId"], "actionId": exec_plan["actionId"],
                "candidateHash": exec_plan["candidateHash"], "runnerDigest": digest,
                "chargeId": plan["chargeId"], "reservationId": reservation_id,
                "invocationSha256": invocation_sha256, "unit": exec_plan["unit"]}
    # A refusal before the exchange completed may carry null invocation-bound
    # identities; such a report is diagnostic only (see _disposition_for).
    identity_keys = ("runnerDigest", "chargeId", "reservationId", "invocationSha256")
    unbound = status == "REFUSED" and all(document[key] is None for key in identity_keys)
    for key, value in expected.items():
        if unbound and key in identity_keys:
            continue
        if document[key] != value:
            return None, "result " + key + " differs from the current invocation"
    if document["retryAllowed"] is not False or document["financialAcceptance"] != "pending":
        return None, "result retry/acceptance fields"
    # Strict scalar types: bool is not int, strings are not flags.
    for key in ("terminalEvidencePersisted", "stopReceiptPersisted", "containmentComplete", "timerCancelReceiptPersisted"):
        if not _is_bool(document[key]):
            return None, "result " + key + " is not a boolean"
    for key in ("stopReturnCode", "timerCancelReturnCode"):
        if not _is_return_code(document[key]):
            return None, "result " + key + " is not a return code"
    if document["stopErrorClass"] is not None and (type(document["stopErrorClass"]) is not str or not document["stopErrorClass"]):
        return None, "result stopErrorClass"
    invocation_id = document["invocationId"]
    if invocation_id is not None and (type(invocation_id) is not str or not re.fullmatch(r"[0-9a-f]{32}", invocation_id)
                                      or invocation_id == "0" * 32):
        return None, "result invocationId"
    raw = document["rawMainExit"]
    if type(raw) is not dict or set(raw) != {"kind", "code"} or raw["kind"] not in ("exit", "signal", "unknown") \
            or (raw["code"] is not None and (type(raw["code"]) is not int or type(raw["code"]) is bool)):
        return None, "result rawMainExit"
    code = document["failureCode"]
    if code is not None and type(code) is not str:
        return None, "result failureCode type"
    # Closed status / raw exit / failure code combinations.
    if status == "PROCESS_SUCCESS":
        if raw != {"kind": "exit", "code": 0} or code is not None or invocation_id is None:
            return None, "success must carry exit 0, no failure code and a loaded invocation"
        if not (document["terminalEvidencePersisted"] and document["stopReturnCode"] == 0
                and document["stopErrorClass"] is None and document["stopReceiptPersisted"]
                and document["containmentComplete"] and document["timerCancelReturnCode"] == 0
                and document["timerCancelReceiptPersisted"] and document["outstandingOwners"] == []):
            return None, "success flags are not all established"
    elif status == "PROCESS_FAILED":
        if invocation_id is None or not document["terminalEvidencePersisted"]:
            return None, "failure without a loaded invocation or durable terminal evidence"
        if raw["kind"] == "exit" and not (code == "MAIN_EXIT_NONZERO" and 1 <= raw["code"] <= 255):
            return None, "nonzero exit failure combination"
        if raw["kind"] == "signal" and not (code == "MAIN_SIGNAL" and 1 <= raw["code"] <= 64):
            return None, "signal failure combination"
        if raw["kind"] == "unknown":
            return None, "failure without a raw main exit"
    elif status == "PROCESS_UNKNOWN":
        if raw != {"kind": "unknown", "code": None} or code not in FAILURE_CODES_UNKNOWN:
            return None, "unknown status combination"
    else:  # REFUSED
        if raw != {"kind": "unknown", "code": None} or code not in FAILURE_CODES_REFUSED or invocation_id is not None:
            return None, "refused status combination"
    if type(document["evidence"]) is not list or type(document["outstandingOwners"]) is not list:
        return None, "result evidence lists"
    for item in document["outstandingOwners"]:
        if type(item) is not dict or set(item) != {"owner", "resource", "reason"} \
                or any(type(v) is not str or not v for v in item.values()):
            return None, "result outstandingOwners entry"
    evidence = {}
    paths = set()
    for item in document["evidence"]:
        payload, problem = _read_evidence(root, exec_plan, item, missing)
        if payload is None:
            return None, "result " + problem
        if item["kind"] in evidence or item["path"] in paths:
            return None, "duplicate evidence kind or path: " + item["kind"]
        evidence[item["kind"]] = payload
        paths.add(item["path"])
    terminal_sha = next((item["sha256"] for item in document["evidence"] if item["kind"] == "terminal-observation"), None)
    problem = _validate_evidence_contents(document, evidence, {"terminalSha256": terminal_sha},
                                          exec_plan["prover"]["containerId"])
    if problem is not None:
        return None, "result evidence: " + problem
    return {"document": document, "sha256": hashlib.sha256(data).hexdigest(), "unbound": unbound}, None


def _disposition_for(exit_code, loaded, handshake, acknowledged, ambiguous, appended=True, progress=None,
                     released=None):
    """Map the validated result, the parent's own observations and the release outcome.

    - A failed durable invocation append keeps everything unresolved.
    - A report without a completed handshake or with null identities is diagnostic only.
    - REFUSED closes only when the parent saw no resource start (no startup
      progress) and the child reports complete containment with no owners.
    - Success and contained failure additionally require durable ownership release.
    """
    if not appended or ambiguous or loaded is None or not handshake:
        return "unresolved"
    document = loaded["document"]
    status = document["status"]
    from moriarty_dev import loan_executor
    if loan_executor.STATUS_EXIT.get(status) != exit_code or loaded.get("unbound"):
        return "unresolved"
    phases = [p.get("phase") for p in (progress or [])]
    if status == "REFUSED":
        no_start = "startup" not in phases and "launched" not in phases
        if no_start and document["containmentComplete"] and not document["outstandingOwners"] and released is not False:
            return "refused"
        return "unresolved"
    if status == "PROCESS_SUCCESS":
        return "success" if acknowledged and released is True else "unresolved"
    if status == "PROCESS_FAILED":
        contained = document["containmentComplete"] and not document["outstandingOwners"] and acknowledged
        return "failed" if contained and released is True else "failed_unresolved"
    return "unresolved"


def execute_loan_invocation(root, plan, digest, reservation_id):
    """Actual one-shot runner handoff for the loan executor profile."""
    import secrets
    import struct  # noqa: F401 - peer credentials
    from moriarty_dev import accounting, loan_executor, store
    from moriarty_dev.loan_exit_retention.exit_retention import run_bounded

    import fcntl
    root = Path(root).resolve()
    executor = plan["executor"]
    exec_plan = loan_executor.load_plan(root, executor["plan"], executor["sha256"])
    admission = loan_executor.load_admission(root, executor["admission"], exec_plan)
    parent_pid = os.getpid()
    parent_ticks = _start_ticks(parent_pid)
    pre = accounting.parent_pre_handover_state(root, exec_plan, admission)
    if isinstance(pre, accounting.Refusal):
        raise LaunchRefused("%s: %s" % (pre.code, pre.detail))
    # The accounting parent holds the exclusive wallet-use OS lock from here
    # through validated containment. An active holder denies; no stealing.
    lock_path = pre["chain"]["context"]["observer"]["lockPath"]
    lock_fd = os.open(lock_path, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        os.close(lock_fd)
        raise LaunchRefused("OWNERSHIP_UNRESOLVED: wallet-use lock is held by another owner")
    try:
        state = accounting.runtime_handover(root, exec_plan, admission, plan["chargeId"], reservation_id, digest,
                                            parent_pid, parent_ticks if parent_ticks is not None else 0)
        if isinstance(state, accounting.Refusal):
            raise LaunchRefused("%s: %s" % (state.code, state.detail))
        if state["chargeId"] != plan["chargeId"] or state["reservationId"] != reservation_id:
            raise LaunchRefused("CLAIM_INVALID: current correspondence names another charge/reservation")
        receipt = _run_loan_child(root, plan, digest, reservation_id, exec_plan, admission, state, parent_pid,
                                  parent_ticks)
        loan = receipt["runnerReceipt"]["loanResult"]
        released = None
        if loan["disposition"] in ("success", "failed", "refused"):
            # Durable release is part of operational completion: a refused or
            # failed release keeps occupancy and downgrades the disposition.
            outcome = accounting.release_ownership(root, exec_plan, admission, reservation_id)
            released = not isinstance(outcome, accounting.Refusal)
            loan["releaseProblem"] = None if released else "%s: %s" % (outcome.code, outcome.detail)
        loan["ownershipReleased"] = bool(released)
        loan["disposition"] = _disposition_for(loan["exitCode"], loan["loaded"], loan["handshake"]["completed"],
                                               loan["containmentAcknowledged"], loan["ambiguous"],
                                               loan["invocationAppended"], loan["progress"], released)
        del loan["loaded"]
        return receipt
    finally:
        os.close(lock_fd)


def _run_loan_child(root, plan, digest, reservation_id, exec_plan, admission, state, parent_pid, parent_ticks):
    import secrets
    from moriarty_dev import accounting, loan_executor, store
    from moriarty_dev.loan_exit_retention.exit_retention import run_bounded
    db_path = Path(state["storePath"])
    action = plan["action"]

    runtime_dir = exec_plan.evidence_dir / ("channel-" + secrets.token_hex(8))
    os.mkdir(runtime_dir, 0o700)
    socket_path = runtime_dir / "channel.sock"
    # AF_UNIX addresses are limited to 108 bytes; bind through the directory
    # descriptor so a deep evidence directory cannot break the private channel.
    dir_fd = os.open(str(runtime_dir), os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
    listener = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    try:
        listener.bind("/proc/self/fd/%d/%s" % (dir_fd, socket_path.name))
        os.chmod(socket_path, 0o600)
        listener.listen(1)
    except OSError:
        listener.close()
        os.close(dir_fd)
        os.rmdir(runtime_dir)
        raise
    finally:
        pass
    nonce = secrets.token_bytes(32)
    nonce_hex = nonce.hex()
    env = dict(os.environ)
    env[ENV_SOCKET] = str(socket_path)
    env[ENV_NONCE] = nonce_hex

    command = [*plan["launcher"], "--timeout", str(plan["timeoutSeconds"]),
               "--grace", str(plan["graceSeconds"]), "--require-containment", "strong", "--", *plan["argv"]]
    boot_id = Path("/proc/sys/kernel/random/boot_id").read_text().strip()
    outer_start_ns = time.monotonic_ns()
    wall_start_ms = int(time.time() * 1000)
    started = time.monotonic()
    proc = subprocess.Popen(command, cwd=root, stdin=subprocess.DEVNULL, env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    launcher_ticks = _start_ticks(proc.pid)
    outer_deadline_ns = outer_start_ns + plan["timeoutSeconds"] * 1_000_000_000
    block_remaining_ms = state["blockDeadlineMs"] - wall_start_ms
    if block_remaining_ms * 1_000_000 < plan["timeoutSeconds"] * 1_000_000_000:
        outer_deadline_ns = outer_start_ns + max(0, block_remaining_ms) * 1_000_000
    payload = {
        "schema": accounting.INVOCATION_SCHEMA, "allocationId": exec_plan["allocationId"],
        "actionId": exec_plan["actionId"], "candidateHash": exec_plan["candidateHash"], "runnerDigest": digest,
        "chargeId": plan["chargeId"], "reservationId": reservation_id, "storeIdentity": state["storeIdentity"],
        "executionContextSha256": state["executionContextSha256"], "projectionSha256": state["projectionSha256"],
        "correspondenceSha256": state["correspondenceSha256"], "authoritySha256": state["authoritySha256"],
        "bootId": boot_id, "outerStartMonotonic": outer_start_ns, "outerDeadlineMonotonic": outer_deadline_ns,
        "blockDeadlineUtc": state["blockDeadlineUtc"], "parentPid": parent_pid,
        "parentStartTicks": parent_ticks if parent_ticks is not None else 0,
        "launcherPid": proc.pid, "launcherStartTicks": launcher_ticks if launcher_ticks is not None else 0,
        "nonceSha256": hashlib.sha256(nonce).hexdigest(),
    }
    append = run_bounded(lambda: list(store.record_invocation(db_path, str(root), action, payload)), APPEND_BOUND_SECONDS)
    invocation_event = append.value if append.completed else None
    invocation_sha256 = store.canonical_digest(payload)
    append_ok = append.completed and invocation_event is not None and invocation_event[1] == invocation_sha256

    selector = selectors.DefaultSelector()
    streams = {"stdout": bytearray(), "stderr": bytearray()}
    hashes = {name: hashlib.sha256() for name in streams}
    sizes = {name: 0 for name in streams}
    for pipe, name in ((proc.stdout, "stdout"), (proc.stderr, "stderr")):
        selector.register(pipe, selectors.EVENT_READ, name)
    limit = plan["outputLimitBytes"]
    ambiguous = False
    handshake = {"attempted": False, "completed": False, "denied": None, "peerPid": None}
    progress = []
    acknowledged = False
    conn = None
    if append_ok:
        selector.register(listener, selectors.EVENT_READ, "listener")
    else:
        listener.close()
        listener = None

    def deny(connection, reason):
        handshake["denied"] = reason
        try:
            _send_json(connection, {"schema": loan_executor.ANSWER_SCHEMA, "denied": reason})
        except OSError:
            pass
        connection.close()

    try:
        while selector.get_map() or proc.poll() is None:
            if time.monotonic() - started > plan["timeoutSeconds"] + plan["graceSeconds"] + 30:
                ambiguous = True
                os.killpg(proc.pid, signal.SIGKILL)
                break
            if proc.poll() is not None and listener is not None and listener.fileno() in selector.get_map() \
                    and all(key.data in ("listener", "channel") for key in selector.get_map().values()):
                # The child is gone without ever connecting: no handshake will follow.
                selector.unregister(listener)
                listener.close()
                listener = None
                continue
            for key, _ in selector.select(0.1):
                name = key.data
                if name == "listener":
                    selector.unregister(listener)
                    connection, _ = listener.accept()
                    listener.close()  # exactly one exchange: no further listener exists
                    listener = None
                    handshake["attempted"] = True
                    if time.monotonic_ns() > outer_start_ns + SETUP_SECONDS * 1_000_000_000:
                        deny(connection, "SETUP_WINDOW_PASSED")
                        continue
                    peer_pid, peer_uid, _ = _peer_credentials(connection)
                    handshake["peerPid"] = peer_pid
                    if peer_uid != os.getuid():
                        deny(connection, "PEER_UID")
                        continue
                    ok, why = _descends_from(peer_pid, proc.pid, launcher_ticks)
                    if not ok:
                        deny(connection, "PEER_ANCESTRY:" + why)
                        continue
                    line = _recv_line(connection, 5.0)
                    try:
                        request = json.loads(line.decode("utf-8")) if line else None
                    except ValueError:
                        request = None
                    if type(request) is not dict or request.get("schema") != loan_executor.HANDSHAKE_SCHEMA \
                            or set(request) != {"schema", "nonce", "authoritySha256"} \
                            or type(request.get("nonce")) is not str or type(request.get("authoritySha256")) is not str:
                        deny(connection, "HANDSHAKE_MALFORMED")
                        continue
                    import hmac
                    if not hmac.compare_digest(request["nonce"], nonce_hex):
                        deny(connection, "NONCE")
                        continue
                    if not hmac.compare_digest(request["authoritySha256"], admission.sha256):
                        deny(connection, "AUTHORITY")
                        continue
                    check = accounting.parent_claim_check(state, exec_plan, payload)
                    if isinstance(check, accounting.Refusal):
                        deny(connection, check.code + ":" + check.detail)
                        continue
                    if store.find_invocation_event(db_path, invocation_sha256) != payload:
                        deny(connection, "STORE_EVENT")
                        continue
                    _send_json(connection, {"schema": loan_executor.ANSWER_SCHEMA, "payload": payload,
                                            "sha256": invocation_sha256})
                    handshake["completed"] = True
                    conn = connection
                    conn.settimeout(None)
                    selector.register(conn, selectors.EVENT_READ, "channel")
                    continue
                if name == "channel":
                    try:
                        chunk = conn.recv(65536)
                    except OSError:
                        chunk = b""
                    if not chunk:
                        selector.unregister(conn)
                        continue
                    for raw_line in chunk.split(b"\n"):
                        if not raw_line.strip():
                            continue
                        try:
                            message = json.loads(raw_line.decode("utf-8"))
                        except ValueError:
                            continue
                        if type(message) is dict and message.get("schema") == loan_executor.PROGRESS_SCHEMA:
                            progress.append({"phase": message.get("phase"), "detail": message.get("detail")})
                            if message.get("phase") == "contained":
                                acknowledged = True
                    continue
                chunk = os.read(key.fileobj.fileno(), 16384)
                if not chunk:
                    selector.unregister(key.fileobj)
                    continue
                hashes[name].update(chunk)
                sizes[name] += len(chunk)
                streams[name].extend(chunk[:max(0, limit - len(streams[name]))])
        ret = proc.wait(timeout=5)
        ambiguous = ambiguous or ret < 0 or ret >= 125
    finally:
        selector.close()
        proc.stdout.close()
        proc.stderr.close()
        if conn is not None:
            conn.close()
        if listener is not None:
            listener.close()
        try:
            socket_path.unlink()
            runtime_dir.rmdir()
        except OSError:
            pass
        os.close(dir_fd)
    overflow = any(size > limit for size in sizes.values())
    loaded, problem = _read_result_document(root, exec_plan, digest, reservation_id, invocation_sha256, plan)
    # Preliminary disposition assuming a durable release succeeds; the caller
    # performs the release and recomputes with the actual outcome.
    disposition = _disposition_for(ret, loaded, handshake["completed"], acknowledged, ambiguous, append_ok,
                                   progress, released=True)
    loan_result = {
        "disposition": disposition, "problem": problem, "loaded": loaded, "exitCode": ret, "ambiguous": ambiguous,
        "resultPath": str(exec_plan.result_path.relative_to(root)),
        "resultSha256": loaded["sha256"] if loaded else None,
        "status": loaded["document"]["status"] if loaded else None,
        "failureCode": loaded["document"]["failureCode"] if loaded else None,
        "containmentComplete": loaded["document"]["containmentComplete"] if loaded else None,
        "outstandingOwners": loaded["document"]["outstandingOwners"] if loaded else None,
        "invocationSha256": invocation_sha256, "invocationEventId": invocation_event[0] if invocation_event else None,
        "invocationAppended": append_ok, "handshake": handshake, "containmentAcknowledged": acknowledged,
        "progress": progress, "bootId": boot_id, "outerStartMonotonic": outer_start_ns,
        "outerDeadlineMonotonic": outer_deadline_ns,
    }
    observed = {"actionId": plan["action"]["id"], "candidateHash": plan["action"]["candidate"],
                "runnerDigest": digest, "chargeId": plan["chargeId"], "reservationId": reservation_id,
                "argv": command, "exitCode": ret, "elapsedSeconds": time.monotonic() - started,
                "outputSha256": {name: h.hexdigest() for name, h in hashes.items()}, "outputBytes": sizes,
                "completionAmbiguous": ambiguous, "outputLimitExceeded": overflow,
                "behavioralAssertions": [], "loanResult": loan_result}
    return {"exitCode": ret, "stdout": streams["stdout"].decode("utf-8", errors="replace"),
            "stderr": streams["stderr"].decode("utf-8", errors="replace"), "runnerReceipt": observed}
