"""Runner for a campaign-bound, already charged command.

Campaign binding bytes authorize the command. The existing budget runner must
record its debit before calling the CLI; this runner never changes that budget.
Child stdout is data, never authority for charges or completion metadata.

The admitted argv is launched directly as a child process in its own process
group, with stdin closed. The runner enforces the timeout itself: SIGTERM to
the group at timeoutSeconds, SIGKILL after graceSeconds. A platform CLI lane
(codex, grok, agy or argon-run, muse) is admitted like any other command: its
absolute executable path and its full argv, including the model pin, are
hash-bound in the campaign binding.
"""
import hashlib
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import time

from moriarty_dev import records

SCHEMA = "moriarty.bound-runner/2"
TIMEOUT_EXIT = 124


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
            # The only external input is the admitted executable itself.
            if name != executable:
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
    required = {"schema", "action", "argv", "files",
                "timeoutSeconds", "graceSeconds", "outputLimitBytes", "chargeId", "chargedSeconds", "accountingPath"}
    if set(plan) - (required | {"assertion"}) or not required.issubset(plan):
        raise ValueError("unsupported runner binding fields")
    if plan["schema"] != SCHEMA or plan["action"] != action or campaign["candidateHash"] != action["candidate"]:
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


def _signal_group(pgid, sig):
    try:
        os.killpg(pgid, sig)
    except ProcessLookupError:
        pass


def execute(root, plan, digest, reservation_id):
    """Observe the actual process; retain bounded output while hashing all bytes."""
    command = list(plan["argv"])
    started = time.monotonic()
    proc = subprocess.Popen(command, cwd=root, stdin=subprocess.DEVNULL,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    pgid = proc.pid
    selector = selectors.DefaultSelector()
    streams = {"stdout": bytearray(), "stderr": bytearray()}
    hashes = {name: hashlib.sha256() for name in streams}
    sizes = {name: 0 for name in streams}
    for pipe, name in ((proc.stdout, "stdout"), (proc.stderr, "stderr")):
        selector.register(pipe, selectors.EVENT_READ, name)
    limit = plan["outputLimitBytes"]
    timeout, grace = plan["timeoutSeconds"], plan["graceSeconds"]
    terminated_at = None
    timed_out = False
    ambiguous = False
    try:
        while selector.get_map() or proc.poll() is None:
            elapsed = time.monotonic() - started
            if terminated_at is None and elapsed > timeout:
                timed_out = True
                terminated_at = elapsed
                _signal_group(pgid, signal.SIGTERM)
            if terminated_at is not None and elapsed > terminated_at + grace:
                _signal_group(pgid, signal.SIGKILL)
            if elapsed > timeout + grace + 30:
                # Output pipes held open by an escaped descendant.
                ambiguous = True
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
        if timed_out and not ambiguous:
            # The runner itself stopped and reaped the child: a confirmed
            # timeout, reported as exit 124.
            ret = TIMEOUT_EXIT
        else:
            # A signal or an exit code at or above 125 does not establish
            # completion.
            ambiguous = ambiguous or ret < 0 or ret >= 125
    finally:
        # Remove any descendant left in the child's process group.
        _signal_group(pgid, signal.SIGKILL)
        if proc.poll() is None:
            proc.kill()
            proc.wait()
        selector.close()
        proc.stdout.close()
        proc.stderr.close()
    overflow = any(size > limit for size in sizes.values())
    assertion = plan.get("assertion")
    verified = bool(not ambiguous and not timed_out and not overflow and assertion and ret == assertion["exitCode"]
                    and hashes["stdout"].hexdigest() == assertion["stdoutSha256"])
    observed = {"actionId": plan["action"]["id"], "candidateHash": plan["action"]["candidate"],
                "runnerDigest": digest, "chargeId": plan["chargeId"], "reservationId": reservation_id,
                "argv": command, "exitCode": ret, "elapsedSeconds": time.monotonic() - started,
                "outputSha256": {name: h.hexdigest() for name, h in hashes.items()}, "outputBytes": sizes,
                "timedOut": timed_out,
                "completionAmbiguous": ambiguous, "outputLimitExceeded": overflow,
                "behavioralAssertions": [{"assertionId": assertion["id"], "candidateHash": plan["action"]["candidate"],
                                          "outcome": "defect-observed"}] if verified else []}
    return {"exitCode": ret, "stdout": streams["stdout"].decode("utf-8", errors="replace"),
            "stderr": streams["stderr"].decode("utf-8", errors="replace"), "runnerReceipt": observed}
