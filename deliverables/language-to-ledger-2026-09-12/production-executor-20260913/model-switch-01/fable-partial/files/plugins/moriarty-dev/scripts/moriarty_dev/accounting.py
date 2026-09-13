"""Authenticated executor context verification for the loan executor.

This module is the production implementation of the executor's
validate_current dependency. It authenticates the immutable resource
amendment -> projection/context/votes -> canonical store token/history ->
current observation/correspondence -> the claimed runtime debit, reservation
and continuously held wallet ownership, entirely through the shared
records/store validators. It never debits, adopts a store, resets an
attempt, creates a wallet or refreshes an observation.

The full accounting producer (predebit gate, funding preflight, master
debit/projection replacement, atomic preflight-to-runtime handover) is not
implemented here. A live cmd_run cannot create current authority until that
producer exists; this verifier accepts only records that such a producer, or
an isolated fixture of it, has already written.
"""
import datetime
from decimal import Decimal, ROUND_CEILING
import fcntl
import hashlib
import json
import os
from pathlib import Path
import sqlite3
import time
from contextlib import closing

from moriarty_dev import records, store
from moriarty_dev.loan_exit_retention.exit_retention import run_bounded

PROJECTION_SCHEMA = "moriarty.mc02-successor-projection/1"
VOTE_SCHEMA = "moriarty.i2-resource-vote/1"
CONTEXT_SCHEMA = "moriarty.i2-execution-context/1"
HISTORY_SCHEMA = "moriarty.i2-retained-history/1"
OBSERVATION_SCHEMA = "moriarty.i2-funding-observation/1"
CORRESPONDENCE_SCHEMA = "moriarty.i2-current-correspondence/1"
OWNERSHIP_SCHEMA = "moriarty.i2-wallet-ownership/1"
INVOCATION_SCHEMA = "moriarty.loan-invocation/1"

PROJECTION_KEYS = (
    "schema", "predecessor", "ledgerOwner", "selectedOwner", "envelopeId", "grantSeconds",
    "grantDispatches", "closureSeconds", "closureAdjustmentSeconds", "campaignId", "actionId",
    "historicalDispositions", "executionContextSha256",
)
VOTE_KEYS = ("schema", "reviewer", "model", "role", "scope", "candidateSha256", "verdict", "findings")
VOTE_ROLE = "independent-resource-reviewer"
VOTE_SCOPE = "MC02 active-envelope grant and execution context"
# Substantive resource votes come from two fresh independent reviewers of
# distinct vendors. The historical pair (claude-opus-5, gpt-6-astra) remains
# accepted with its original identities; the current user routing (Fable 5.1
# implementation with GPT-6 high checking) is admitted through the same closed
# table without relabelling old votes. Model routing alone grants nothing.
VOTE_VENDORS = records.REVIEWER_VENDORS
VOTE_IDENTITIES = frozenset(VOTE_VENDORS)
CONTEXT_KEYS = (
    "schema", "repository", "gitCommonDir", "store", "history", "observer",
    "correspondencePath", "expected", "observerExecution",
)
STORE_KEYS = ("path", "identityToken", "inventory")
HISTORY_KEYS = ("schema", "repositoryKeys", "requirement", "capability", "receipts", "outcomes", "eventBindings")
OUTCOME_KEYS = ("receiptSha256", "defectClass", "outcome", "resolvedBy")
OUTCOME_VALUES = frozenset(("failed", "unknown", "verified-success"))
EVENT_BINDING_KEYS = ("repository", "requirement", "capability", "eventIds")
OBSERVER_KEYS = (
    "entryPath", "entrySha256", "receiptPath", "maxAgeSeconds", "preflightSeconds",
    "preflightAttempts", "lockPath",
)
EXPECTED_KEYS = (
    "allocationId", "network", "roleIdentity", "selectedToken", "walletStateDirectory",
    "seedPath", "pendingUseDisposition",
)
NETWORK_KEYS = ("networkId", "genesisHash", "protocolVersion")
ROLE_KEYS = ("publicKey", "address")
TOKEN_KEYS = ("id", "unit")
OBSERVER_EXECUTION_KEYS = (
    "interpreter", "entry", "files", "sdkClosure", "launcher", "argv", "environment", "parentEvidencePath",
)
OBSERVATION_KEYS = (
    "schema", "allocationId", "preflightChargeId", "sourceSha256", "observedAt", "syncTime", "network",
    "roleIdentity", "amounts", "unitNames", "finalityRef", "pendingUseDispositionSha256",
    "canonicalInputsUnchanged", "childrenStopped",
)
CORRESPONDENCE_KEYS = (
    "schema", "allocationId", "executionContextSha256", "masterSha256", "projectionSha256",
    "observationSha256", "preflightChargeId", "runtimeChargeId", "reservationId",
)
DEBIT_KEYS = (
    "id", "seconds", "actionId", "candidateHash", "runnerDigest", "package", "envelope",
    "storeIdentity", "reservationId", "launchClaim", "purpose",
)
ENVELOPE_KEYS = (
    "owner", "projectionSha256", "amendmentSha256", "remainingReservedSeconds",
    "workerDispatches", "workerDispatchLimit",
)
OWNERSHIP_KEYS = (
    "schema", "allocationId", "executionContextSha256", "projectionSha256", "storeIdentity",
    "preflightReservationId", "preflightChargeId", "runtimeChargeId", "runtimeReservationId",
    "ownerPid", "ownerStartTicks", "state",
)
INVOCATION_KEYS = (
    "schema", "allocationId", "actionId", "candidateHash", "runnerDigest", "chargeId", "reservationId",
    "storeIdentity", "executionContextSha256", "projectionSha256", "correspondenceSha256",
    "authoritySha256", "bootId", "outerStartMonotonic", "outerDeadlineMonotonic", "blockDeadlineUtc",
    "parentPid", "parentStartTicks", "launcherPid", "launcherStartTicks", "nonceSha256",
)
PARENT_EVIDENCE_SCHEMA = "moriarty.i2-observer-parent-evidence/1"
PARENT_EVIDENCE_KEYS = (
    "schema", "allocationId", "preflightChargeId", "contextSha256", "launchSha256", "sourceSha256",
    "outputSha256", "processExit", "containment", "canonicalInputsBefore", "canonicalInputsAfter",
    "startedAt", "endedAt",
)
CANONICAL_INPUT_KEYS = ("seedSha256", "snapshotSha256")
CONTAINMENT_KEYS = ("capability", "childrenStopped")
PROVER_CONTAINER_SCHEMA = "moriarty.prover-container-expectation/1"
PROVER_CONTAINER_KEYS = (
    "schema", "containerId", "imageDigest", "memoryLimitBytes", "entrypoint", "args", "mounts",
    "restartPolicy", "pidMode", "privileged", "capAdd",
)
MOUNT_KEYS = ("source", "destination", "readOnly")
FOREMAN_LAUNCHER_PATH = "/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js"
OBSERVER_LAUNCH_ARGS = ["--timeout", "90", "--grace", "5", "--require-containment", "strong", "--"]
FORBIDDEN_ENVIRONMENT_KEYS = ("NODE_OPTIONS", "LD_PRELOAD", "LD_LIBRARY_PATH", "PYTHONPATH", "NODE_PATH")
CLOSURE_BASIS = "historical-credit-closure-not-paid-cost"
RESERVE_BASIS = "mc02-successor-reserve-not-paid-cost"
RESERVE_ROW_KEYS = ("id", "seconds", "basis", "sourceSha256", "amendmentSha256")
PRESERVED_PREDECESSOR_FIELDS = (
    "package", "charges", "planning_charge_seconds", "planning_overhead_reserved_seconds",
    "package_limit_seconds", "worker_dispatches", "worker_dispatch_limit",
)
MAX_OBSERVATION_AGE_SECONDS = 120
OBSERVER_MAX_AGE = 120
OBSERVER_PREFLIGHT_SECONDS = 125
OBSERVER_PREFLIGHT_ATTEMPTS = 1
MIN_RUNTIME_DEBIT_SECONDS = 1835
OWNERSHIP_STATE_RUNTIME = "runtime-held"
LOCK_WAIT_SECONDS = 5.0   # bounded acquisition of the existing budget lock
WRITE_BOUND_SECONDS = 25  # supervised master/correspondence/ownership replacement


class Authenticated(object):
    """A parsed record together with the digest and path of its exact bytes."""

    def __init__(self, record, sha256, path):
        self.record = record
        self.sha256 = sha256
        self.path = str(path)

    def __getitem__(self, key):
        return self.record[key]

    def get(self, key, default=None):
        return self.record.get(key, default)


class Refusal(object):
    __slots__ = ("code", "detail", "missing")

    def __init__(self, code, detail, missing=None):
        self.code = code
        self.detail = detail
        self.missing = list(missing or [])

    def as_dict(self):
        return {"refused": True, "code": self.code, "detail": self.detail, "missing": self.missing}

    def __repr__(self):
        return "Refusal(%s: %s)" % (self.code, self.detail)


class CurrentExecution(dict):
    """Validated current execution state; a plain closed mapping."""

    refused = False


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _iso_ms(value):
    if type(value) is not str:
        return None
    try:
        parsed = datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None
    if parsed.tzinfo is None or parsed.utcoffset() != datetime.timedelta(0):
        return None
    return int(parsed.timestamp() * 1000)


def _posint(value):
    return type(value) is int and type(value) is not bool and value > 0


def _nonneg(value):
    return type(value) is int and type(value) is not bool and value >= 0


def _nonempty(value):
    return type(value) is str and value.strip() != ""


def _canonical_root(root):
    return Path(root).resolve()


# --- authority chain ------------------------------------------------------

def authenticate_authority(root, plan, admission):
    """Amendment -> projection/votes/context -> canonical store token and lineage.

    Returns a dict describing the authenticated chain, or a Refusal with
    AUTHORITY_INVALID. Nothing here reads mutable evidence.
    """
    root = _canonical_root(root)
    missing = []
    record = admission.record if isinstance(admission, Authenticated) else None
    if type(record) is not dict:
        return Refusal("AUTHORITY_INVALID", "admission is not an authenticated amendment record")
    plan_record = plan.record if isinstance(plan, Authenticated) else plan
    if type(plan_record) is not dict or type(plan_record.get("authority")) is not dict \
            or plan_record["authority"].get("sha256") != admission.sha256:
        return Refusal("AUTHORITY_INVALID", "plan.authority digest does not match admission bytes")
    extra = [k for k in record if k not in records.RESOURCE_AMENDMENT_KEYS]
    if extra:
        return Refusal("AUTHORITY_INVALID", "unknown amendment fields: " + ",".join(sorted(extra)))
    resources = record.get("resources")
    if type(resources) is not dict:
        return Refusal("AUTHORITY_INVALID", "amendment resources object missing")
    for key in ("projection", "votes", "executionContext", "blockDeadlineUtc", "proverContainer"):
        if key not in resources:
            return Refusal("AUTHORITY_INVALID", "amendment resources." + key + " missing")
    block_deadline_ms = _iso_ms(resources["blockDeadlineUtc"])
    if block_deadline_ms is None:
        return Refusal("AUTHORITY_INVALID", "blockDeadlineUtc is not a UTC ISO-8601 timestamp")

    projection_loaded = records._reference_json(root, resources["projection"], missing, "projection")
    if projection_loaded is None:
        return Refusal("AUTHORITY_INVALID", "projection reference", missing)
    projection_path, projection_bytes, projection = projection_loaded
    projection_sha = resources["projection"]["sha256"]
    if not records._exact_keys(projection, PROJECTION_KEYS, missing, "projection"):
        return Refusal("AUTHORITY_INVALID", "projection fields", missing)
    if projection["schema"] != PROJECTION_SCHEMA:
        return Refusal("AUTHORITY_INVALID", "projection schema")
    if projection["ledgerOwner"] != "MC01" or projection["selectedOwner"] != "MC02":
        return Refusal("AUTHORITY_INVALID", "projection owners")
    if not (_posint(projection["grantSeconds"]) and _posint(projection["grantDispatches"])
            and _nonneg(projection["closureSeconds"]) and _nonneg(projection["closureAdjustmentSeconds"])):
        return Refusal("AUTHORITY_INVALID", "projection grant arithmetic fields")
    if not _nonempty(projection["envelopeId"]) or not _nonempty(projection["campaignId"]):
        return Refusal("AUTHORITY_INVALID", "projection identity fields")
    if projection["actionId"] != plan.get("actionId"):
        return Refusal("AUTHORITY_INVALID", "projection actionId differs from plan")
    predecessor = projection["predecessor"]
    if not records._exact_keys(predecessor, records.REFERENCE_KEYS, missing, "predecessor") \
            or not records._hex64(predecessor["sha256"]) or not _nonempty(predecessor["path"]):
        return Refusal("AUTHORITY_INVALID", "projection predecessor reference", missing)
    dispositions = projection["historicalDispositions"]
    if type(dispositions) is not list:
        return Refusal("AUTHORITY_INVALID", "historicalDispositions is not a list")
    seen = set()
    predecessor_bytes = None
    for item in dispositions:
        loaded = records._reference(root, item, missing, "historical-disposition")
        if loaded is None:
            return Refusal("AUTHORITY_INVALID", "historical disposition reference", missing)
        if item["sha256"] in seen:
            return Refusal("AUTHORITY_INVALID", "duplicate historical disposition")
        seen.add(item["sha256"])
        if item["sha256"] == predecessor["sha256"]:
            predecessor_bytes = loaded[1]
    if predecessor_bytes is None:
        return Refusal("AUTHORITY_INVALID", "retained predecessor master bytes are not among the historical dispositions")
    try:
        predecessor_master = json.loads(predecessor_bytes.decode("utf-8"), parse_float=Decimal)
    except (UnicodeDecodeError, ValueError):
        return Refusal("AUTHORITY_INVALID", "retained predecessor is not JSON")
    if type(predecessor_master) is not dict:
        return Refusal("AUTHORITY_INVALID", "retained predecessor is not an object")
    # The amendment's grant must equal G exactly; no duplicate allocation.
    if record.get("allocationSeconds") is not None or record.get("additionalSeconds") != projection["grantSeconds"]:
        return Refusal("AUTHORITY_INVALID", "amendment grant differs from the projection grant")
    prover_loaded = records._reference_json(root, resources["proverContainer"], missing, "prover-container")
    if prover_loaded is None:
        return Refusal("AUTHORITY_INVALID", "prover container expectation reference", missing)
    prover_container = prover_loaded[2]
    if not records._exact_keys(prover_container, PROVER_CONTAINER_KEYS, missing, "prover-container") \
            or prover_container["schema"] != PROVER_CONTAINER_SCHEMA:
        return Refusal("AUTHORITY_INVALID", "prover container expectation fields", missing)
    if type(prover_container["args"]) is not list or not prover_container["args"] \
            or any(type(a) is not str for a in prover_container["args"]) \
            or type(prover_container["mounts"]) is not list \
            or any(not records._exact_keys(m, MOUNT_KEYS, [], "mount") for m in prover_container["mounts"]) \
            or prover_container["restartPolicy"] != "no" or prover_container["pidMode"] != "" \
            or prover_container["privileged"] is not False or prover_container["capAdd"] != []:
        return Refusal("AUTHORITY_INVALID", "prover container expectation is not an admissible configuration")

    context_ref = resources["executionContext"]
    context_loaded = records._reference_json(root, context_ref, missing, "execution-context")
    if context_loaded is None:
        return Refusal("AUTHORITY_INVALID", "execution context reference", missing)
    context_path, context_bytes, context = context_loaded
    context_sha = context_ref["sha256"]
    if projection["executionContextSha256"] != context_sha:
        return Refusal("AUTHORITY_INVALID", "projection does not commit this execution context")

    votes = resources["votes"]
    if type(votes) is not list or len(votes) != 2:
        return Refusal("AUTHORITY_INVALID", "exactly two vote references are required")
    identities = []
    vote_digests = set()
    for item in votes:
        loaded = records._reference_json(root, item, missing, "vote")
        if loaded is None:
            return Refusal("AUTHORITY_INVALID", "vote reference", missing)
        _, _, vote = loaded
        if item["sha256"] in vote_digests:
            return Refusal("AUTHORITY_INVALID", "duplicate vote reference")
        vote_digests.add(item["sha256"])
        if not records._exact_keys(vote, VOTE_KEYS, missing, "vote"):
            return Refusal("AUTHORITY_INVALID", "vote fields", missing)
        if vote["schema"] != VOTE_SCHEMA or vote["role"] != VOTE_ROLE or vote["scope"] != VOTE_SCOPE:
            return Refusal("AUTHORITY_INVALID", "vote schema/role/scope")
        if vote["candidateSha256"] != projection_sha:
            return Refusal("AUTHORITY_INVALID", "vote binds a different projection")
        if vote["verdict"] != "APPROVED" or type(vote["findings"]) is not list \
                or any(type(f) is not str for f in vote["findings"]):
            return Refusal("AUTHORITY_INVALID", "vote verdict/findings")
        if vote["model"] not in VOTE_IDENTITIES or not _nonempty(vote["reviewer"]):
            return Refusal("AUTHORITY_INVALID", "vote identity")
        identities.append(vote["model"])
    if len(set(identities)) != 2 or len({VOTE_VENDORS[m] for m in identities}) != 2:
        return Refusal("AUTHORITY_INVALID", "votes must come from two distinct independent vendor identities")

    if not records._exact_keys(context, CONTEXT_KEYS, missing, "execution-context"):
        return Refusal("AUTHORITY_INVALID", "execution context fields", missing)
    if context["schema"] != CONTEXT_SCHEMA:
        return Refusal("AUTHORITY_INVALID", "execution context schema")
    if context["repository"] != str(root):
        return Refusal("AUTHORITY_INVALID", "execution context repository is not this canonical checkout")
    identity_missing = []
    common = records._repository_identity(root, identity_missing)
    if identity_missing or context["gitCommonDir"] != common:
        return Refusal("AUTHORITY_INVALID", "execution context gitCommonDir mismatch")
    store_spec = context["store"]
    if not records._exact_keys(store_spec, STORE_KEYS, missing, "context-store"):
        return Refusal("AUTHORITY_INVALID", "context store fields", missing)
    canonical_db = store.get_db_path(root)
    if store_spec["path"] != str(canonical_db):
        return Refusal("AUTHORITY_INVALID", "context store path is not the canonical resolver path")
    if records._reference(root, store_spec["inventory"], missing, "store-inventory") is None:
        return Refusal("AUTHORITY_INVALID", "store inventory reference", missing)
    if not _nonempty(store_spec["identityToken"]):
        return Refusal("AUTHORITY_INVALID", "store identity token missing")
    persisted = store.read_identity_token(canonical_db)
    if persisted is None or persisted != store_spec["identityToken"]:
        return Refusal("AUTHORITY_INVALID", "persisted store token unavailable or different")
    history_loaded = records._reference_json(root, context["history"], missing, "history")
    if history_loaded is None:
        return Refusal("AUTHORITY_INVALID", "history reference", missing)
    observer = context["observer"]
    if not records._exact_keys(observer, OBSERVER_KEYS, missing, "observer"):
        return Refusal("AUTHORITY_INVALID", "observer fields", missing)
    if observer["maxAgeSeconds"] != OBSERVER_MAX_AGE or observer["preflightSeconds"] != OBSERVER_PREFLIGHT_SECONDS \
            or observer["preflightAttempts"] != OBSERVER_PREFLIGHT_ATTEMPTS:
        return Refusal("AUTHORITY_INVALID", "observer bounds differ from the reviewed constants")
    if records._reference(root, {"path": observer["entryPath"], "sha256": observer["entrySha256"]},
                          missing, "observer-entry") is None:
        return Refusal("AUTHORITY_INVALID", "observer entry", missing)
    for key in ("receiptPath", "lockPath"):
        if not _nonempty(observer[key]):
            return Refusal("AUTHORITY_INVALID", "observer " + key)
    if not _nonempty(context["correspondencePath"]):
        return Refusal("AUTHORITY_INVALID", "correspondencePath")
    expected = context["expected"]
    if not records._exact_keys(expected, EXPECTED_KEYS, missing, "expected") \
            or not records._exact_keys(expected["network"], NETWORK_KEYS, missing, "expected-network") \
            or not records._exact_keys(expected["roleIdentity"], ROLE_KEYS, missing, "expected-role") \
            or not records._exact_keys(expected["selectedToken"], TOKEN_KEYS, missing, "expected-token"):
        return Refusal("AUTHORITY_INVALID", "expected record fields", missing)
    if expected["allocationId"] != plan.get("allocationId"):
        return Refusal("AUTHORITY_INVALID", "expected allocationId differs from plan")
    if records._reference(root, expected["pendingUseDisposition"], missing, "pending-use-disposition") is None:
        return Refusal("AUTHORITY_INVALID", "pending-use disposition reference", missing)
    execution = context["observerExecution"]
    if not records._exact_keys(execution, OBSERVER_EXECUTION_KEYS, missing, "observer-execution"):
        return Refusal("AUTHORITY_INVALID", "observerExecution fields", missing)
    return {
        "root": root,
        "admissionSha256": admission.sha256,
        "admissionPath": admission.path,
        "projection": projection,
        "projectionSha256": projection_sha,
        "context": context,
        "contextSha256": context_sha,
        "history": history_loaded[2],
        "historySha256": context["history"]["sha256"],
        "dbPath": canonical_db,
        "storeIdentity": store_spec["identityToken"],
        "blockDeadlineUtc": resources["blockDeadlineUtc"],
        "blockDeadlineMs": block_deadline_ms,
        "voteDigests": sorted(vote_digests),
        "masterPath": predecessor["path"],
        "predecessorSha256": predecessor["sha256"],
        "predecessorMaster": predecessor_master,
        "proverContainer": prover_container,
        "proverContainerSha256": resources["proverContainer"]["sha256"],
    }


# --- history ---------------------------------------------------------------

def verify_history(chain, db_path=None):
    root = chain["root"]
    history = chain["history"]
    missing = []
    if not records._exact_keys(history, HISTORY_KEYS, missing, "history"):
        return Refusal("HISTORY_UNRESOLVED", "history fields", missing)
    if history["schema"] != HISTORY_SCHEMA:
        return Refusal("HISTORY_UNRESOLVED", "history schema")
    if not _nonempty(history["requirement"]) or not _nonempty(history["capability"]):
        return Refusal("HISTORY_UNRESOLVED", "history lineage")
    keys = history["repositoryKeys"]
    if type(keys) is not list or not keys or any(not _nonempty(k) for k in keys):
        return Refusal("HISTORY_UNRESOLVED", "repositoryKeys empty")
    receipts = history["receipts"]
    if type(receipts) is not list or not receipts:
        return Refusal("HISTORY_UNRESOLVED", "receipts empty")
    receipt_digests = set()
    receipt_contents = {}
    for item in receipts:
        loaded = records._reference(root, item, missing, "history-receipt")
        if loaded is None:
            return Refusal("HISTORY_UNRESOLVED", "receipt reference", missing)
        receipt_digests.add(item["sha256"])
        content = records._parse_json(loaded[1], [], "history-receipt")
        if type(content) is not dict:
            return Refusal("HISTORY_UNRESOLVED", "receipt is not a JSON object: " + item["path"])
        receipt_contents[item["sha256"]] = content
    outcomes = history["outcomes"]
    if type(outcomes) is not list or not outcomes:
        return Refusal("HISTORY_UNRESOLVED", "outcomes empty")
    for entry in outcomes:
        if not records._exact_keys(entry, OUTCOME_KEYS, missing, "history-outcome"):
            return Refusal("HISTORY_UNRESOLVED", "outcome fields", missing)
        if entry["receiptSha256"] not in receipt_digests:
            return Refusal("HISTORY_UNRESOLVED", "outcome references an unlisted receipt")
        if entry["outcome"] not in OUTCOME_VALUES or not _nonempty(entry["defectClass"]):
            return Refusal("HISTORY_UNRESOLVED", "outcome value")
        # The receipt's own retained content must state the same outcome class.
        content = receipt_contents[entry["receiptSha256"]]
        if content.get("outcome") != entry["outcome"] or content.get("defectClass") != entry["defectClass"]:
            return Refusal("HISTORY_UNRESOLVED", "outcome entry disagrees with the retained receipt content")
        resolved = entry["resolvedBy"]
        if type(resolved) is not list or any(r not in receipt_digests for r in resolved):
            return Refusal("HISTORY_UNRESOLVED", "resolution references an unlisted receipt")
        for digest in resolved:
            resolution = receipt_contents[digest]
            if resolution.get("outcome") != "verified-success" or entry["receiptSha256"] not in (resolution.get("resolves") or []):
                return Refusal("HISTORY_UNRESOLVED", "resolution receipt does not verify this same-defect outcome")
    bindings = history["eventBindings"]
    if type(bindings) is not list or not bindings:
        return Refusal("HISTORY_UNRESOLVED", "eventBindings empty")
    db_path = Path(db_path or chain["dbPath"])
    if not db_path.is_file():
        return Refusal("HISTORY_UNRESOLVED", "canonical store unavailable")
    try:
        with closing(sqlite3.connect(str(db_path), timeout=5.0)) as conn:
            for binding in bindings:
                if not records._exact_keys(binding, EVENT_BINDING_KEYS, missing, "event-binding"):
                    return Refusal("HISTORY_UNRESOLVED", "event binding fields", missing)
                if binding["repository"] not in keys:
                    return Refusal("HISTORY_UNRESOLVED", "event binding repository outside repositoryKeys")
                if binding["requirement"] != history["requirement"] or binding["capability"] != history["capability"]:
                    return Refusal("HISTORY_UNRESOLVED", "event binding lineage mismatch")
                ids = binding["eventIds"]
                if type(ids) is not list or not ids or any(not _posint(i) for i in ids):
                    return Refusal("HISTORY_UNRESOLVED", "event ids")
                for event_id in ids:
                    row = conn.execute(
                        "SELECT repository, requirement, capability, payload_json FROM events WHERE id = ?", (event_id,)
                    ).fetchone()
                    if row is None or list(row[:3]) != [binding["repository"], binding["requirement"], binding["capability"]]:
                        return Refusal("HISTORY_UNRESOLVED", "canonical event row %d missing or different" % event_id)
                    try:
                        payload = json.loads(row[3])
                    except ValueError:
                        payload = None
                    if type(payload) is not dict or payload.get("receiptSha256") not in receipt_digests:
                        return Refusal("HISTORY_UNRESOLVED", "canonical event row %d is not bound to a reviewed receipt" % event_id)
    except sqlite3.Error as exc:
        return Refusal("HISTORY_UNRESOLVED", "store read failed: " + type(exc).__name__)
    return {"receiptDigests": receipt_digests, "requirement": history["requirement"],
            "capability": history["capability"]}


# --- observation and correspondence -----------------------------------------

def _mutable_json(root, relative, kind, missing):
    path = records._contained_file(root, relative, missing, kind)
    if path is None:
        return None
    data = records._read_bytes(path, missing, kind)
    if data is None:
        return None
    payload = records._parse_json(data, missing, kind)
    if type(payload) is not dict:
        missing.append(kind + "-not-object")
        return None
    return path, data, payload


def _observer_execution_ok(context, chain_sha, missing):
    execution = context["observerExecution"]
    observer = context["observer"]
    for key in ("interpreter", "entry", "launcher"):
        if not records._exact_keys(execution[key], records.REFERENCE_KEYS, missing, "observer-" + key) \
                or not records._hex64(execution[key]["sha256"]) or not _nonempty(execution[key]["path"]):
            return "observerExecution." + key
    if execution["entry"]["path"] != observer["entryPath"] or execution["entry"]["sha256"] != observer["entrySha256"]:
        return "observerExecution.entry differs from the pinned observer entry"
    if execution["launcher"]["path"] != FOREMAN_LAUNCHER_PATH:
        return "observerExecution.launcher is not the existing Foreman launcher"
    for key in ("interpreter", "launcher"):
        path = Path(execution[key]["path"])
        if not path.is_absolute() or path.is_symlink() or not path.is_file():
            return "observerExecution." + key + " file unavailable"
        if _sha(path.read_bytes()) != execution[key]["sha256"]:
            return "observerExecution." + key + " bytes differ from the pinned digest"
    expected_argv = [execution["interpreter"]["path"], execution["launcher"]["path"], *OBSERVER_LAUNCH_ARGS,
                     execution["interpreter"]["path"], execution["entry"]["path"]]
    if execution["argv"] != expected_argv:
        return "observerExecution.argv is not the fixed strong-containment observer invocation"
    environment = execution["environment"]
    if type(environment) is not dict or any(type(k) is not str or type(v) is not str for k, v in environment.items()) \
            or any(k in FORBIDDEN_ENVIRONMENT_KEYS for k in environment):
        return "observerExecution.environment"
    files = execution["files"]
    if type(files) is not dict or any(type(k) is not str or not records._hex64(v) for k, v in files.items()):
        return "observerExecution.files"
    for rel, digest in files.items():
        path = Path(rel) if rel.startswith("/") else None
        if path is None or path.is_symlink() or not path.is_file() or _sha(path.read_bytes()) != digest:
            return "observerExecution.files entry unavailable or changed: " + rel
    if records._exact_keys(execution["sdkClosure"], records.REFERENCE_KEYS, [], "sdk") is False \
            or not records._hex64(execution["sdkClosure"]["sha256"]):
        return "observerExecution.sdkClosure"
    if not _nonempty(execution["parentEvidencePath"]):
        return "observerExecution.parentEvidencePath"
    return None


def verify_observation(chain, history_facts, wall_time_ms, financial_plan=None):
    """Current funding observation authenticated through parent evidence, not child flags."""
    root = chain["root"]
    context = chain["context"]
    missing = []
    problem = _observer_execution_ok(context, chain["contextSha256"], missing)
    if problem is not None:
        return Refusal("OBSERVATION_INVALID", problem, missing)
    loaded = _mutable_json(root, context["observer"]["receiptPath"], "observation", missing)
    if loaded is None:
        return Refusal("OBSERVATION_INVALID", "observation receipt", missing)
    path, data, observation = loaded
    observation_sha = _sha(data)
    if not records._exact_keys(observation, OBSERVATION_KEYS, missing, "observation"):
        return Refusal("OBSERVATION_INVALID", "observation fields", missing)
    if observation["schema"] != OBSERVATION_SCHEMA:
        return Refusal("OBSERVATION_INVALID", "observation schema")
    expected = context["expected"]
    if observation["allocationId"] != expected["allocationId"]:
        return Refusal("OBSERVATION_INVALID", "observation allocation differs")
    if not _nonempty(observation["preflightChargeId"]):
        return Refusal("OBSERVATION_INVALID", "observation preflight charge identity")
    if observation["sourceSha256"] != context["observer"]["entrySha256"]:
        return Refusal("OBSERVATION_INVALID", "observation source is not the pinned observer entry")
    if observation["network"] != expected["network"] or observation["roleIdentity"] != expected["roleIdentity"]:
        return Refusal("OBSERVATION_INVALID", "observation identity differs from reviewed expectation")
    if type(wall_time_ms) is not int:
        return Refusal("OBSERVATION_INVALID", "wall clock unavailable")
    ages = {}
    for key in ("observedAt", "syncTime"):
        stamp = _iso_ms(observation[key])
        if stamp is None:
            return Refusal("OBSERVATION_INVALID", key + " is not a UTC timestamp")
        if stamp > wall_time_ms:
            return Refusal("OBSERVATION_INVALID", key + " is in the future")
        if wall_time_ms - stamp > MAX_OBSERVATION_AGE_SECONDS * 1000:
            return Refusal("OBSERVATION_INVALID", key + " is older than %d seconds" % MAX_OBSERVATION_AGE_SECONDS)
        ages[key] = wall_time_ms - stamp
    amounts = observation["amounts"]
    units = observation["unitNames"]
    if type(amounts) is not dict or type(units) is not dict or not amounts or set(amounts) != set(units):
        return Refusal("OBSERVATION_INVALID", "amounts/unitNames keys")
    for value in amounts.values():
        if type(value) is not str or not value.isdigit():
            return Refusal("OBSERVATION_INVALID", "amounts must be nonnegative integer strings")
    if any(not _nonempty(u) for u in units.values()):
        return Refusal("OBSERVATION_INVALID", "unit names")
    token = expected["selectedToken"]
    if token["id"] not in amounts or units[token["id"]] != token["unit"]:
        return Refusal("OBSERVATION_INVALID", "selected funding asset or unit is not observed")
    if "DUST" not in amounts or units["DUST"] != "speck":
        return Refusal("OBSERVATION_INVALID", "DUST speck funding is not observed")
    if records._reference(root, observation["finalityRef"], missing, "finality") is None:
        return Refusal("OBSERVATION_INVALID", "finality reference", missing)
    if observation["pendingUseDispositionSha256"] != expected["pendingUseDisposition"]["sha256"] \
            or observation["pendingUseDispositionSha256"] not in history_facts["receiptDigests"]:
        return Refusal("OBSERVATION_INVALID", "pending-use disposition is not the reviewed applicable disposition")
    if type(financial_plan) is dict:
        wallet = financial_plan.get("wallet", {})
        network = financial_plan.get("networkConfig", {})
        limits = financial_plan.get("limits", {})
        if wallet.get("expectedAddress") != expected["roleIdentity"]["address"]:
            return Refusal("OBSERVATION_INVALID", "financial plan wallet address differs from reviewed identity")
        if network.get("networkId") != expected["network"]["networkId"]:
            return Refusal("OBSERVATION_INVALID", "financial plan network differs from reviewed identity")
        if limits.get("allocationId") != expected["allocationId"]:
            return Refusal("OBSERVATION_INVALID", "financial plan allocation differs from reviewed expectation")
        gross = limits.get("grossByLogicalAsset", {})
        required = gross.get(token["id"]) if type(gross) is dict else None
        if type(required) is not str or not required.isdigit():
            return Refusal("OBSERVATION_INVALID", "financial plan gross ceiling for the selected asset unavailable")
        if int(amounts[token["id"]]) < int(required):
            return Refusal("OBSERVATION_INVALID", "observed selected-asset funding below the plan gross ceiling")
        fee = limits.get("dustFee")
        if type(fee) is not str or not fee.isdigit() or int(amounts["DUST"]) < int(fee):
            return Refusal("OBSERVATION_INVALID", "observed DUST below the plan fee ceiling")
    # Parent-authenticated observer execution evidence: the booleans in the
    # observation are derived from it, never taken from the child.
    execution = context["observerExecution"]
    parent_loaded = _mutable_json(root, execution["parentEvidencePath"], "observer-parent-evidence", missing)
    if parent_loaded is None:
        return Refusal("OBSERVATION_INVALID", "observer parent evidence unavailable", missing)
    _, parent_data, parent = parent_loaded
    if not records._exact_keys(parent, PARENT_EVIDENCE_KEYS, missing, "observer-parent-evidence") \
            or parent["schema"] != PARENT_EVIDENCE_SCHEMA:
        return Refusal("OBSERVATION_INVALID", "observer parent evidence fields", missing)
    launch_sha = store.canonical_digest(execution)
    if parent["allocationId"] != expected["allocationId"] or parent["preflightChargeId"] != observation["preflightChargeId"] \
            or parent["contextSha256"] != chain["contextSha256"] or parent["launchSha256"] != launch_sha \
            or parent["sourceSha256"] != context["observer"]["entrySha256"] or parent["outputSha256"] != observation_sha:
        return Refusal("OBSERVATION_INVALID", "observer parent evidence does not bind this context, launch, source and output")
    if type(parent["processExit"]) is bool or parent["processExit"] != 0:
        return Refusal("OBSERVATION_INVALID", "observer process exit was not zero")
    containment = parent["containment"]
    if not records._exact_keys(containment, CONTAINMENT_KEYS, missing, "observer-containment") \
            or containment["capability"] != "posix_pidns_userns_strong" or containment["childrenStopped"] is not True:
        return Refusal("OBSERVATION_INVALID", "observer containment was not strong with stopped children")
    before, after = parent["canonicalInputsBefore"], parent["canonicalInputsAfter"]
    for record in (before, after):
        if not records._exact_keys(record, CANONICAL_INPUT_KEYS, missing, "canonical-inputs") \
                or any(not records._hex64(record[k]) for k in CANONICAL_INPUT_KEYS):
            return Refusal("OBSERVATION_INVALID", "canonical input observations", missing)
    unchanged = before == after
    if observation["canonicalInputsUnchanged"] is not unchanged or observation["childrenStopped"] is not containment["childrenStopped"]:
        return Refusal("OBSERVATION_INVALID", "observation booleans disagree with parent-derived evidence")
    if not unchanged:
        return Refusal("OBSERVATION_INVALID", "canonical wallet inputs changed during observation")
    started, ended = _iso_ms(parent["startedAt"]), _iso_ms(parent["endedAt"])
    if started is None or ended is None or ended < started or ended > wall_time_ms:
        return Refusal("OBSERVATION_INVALID", "observer execution timestamps")
    return {"observation": observation, "observationSha256": observation_sha, "path": path,
            "agesMs": ages, "parentEvidenceSha256": _sha(parent_data)}


def verify_correspondence(chain, observation_facts):
    root = chain["root"]
    missing = []
    loaded = _mutable_json(root, chain["context"]["correspondencePath"], "correspondence", missing)
    if loaded is None:
        return Refusal("CLAIM_INVALID", "correspondence", missing)
    path, data, correspondence = loaded
    if not records._exact_keys(correspondence, CORRESPONDENCE_KEYS, missing, "correspondence"):
        return Refusal("CLAIM_INVALID", "correspondence fields", missing)
    if correspondence["schema"] != CORRESPONDENCE_SCHEMA:
        return Refusal("CLAIM_INVALID", "correspondence schema")
    if correspondence["allocationId"] != chain["context"]["expected"]["allocationId"]:
        return Refusal("CLAIM_INVALID", "correspondence allocation")
    if correspondence["executionContextSha256"] != chain["contextSha256"] \
            or correspondence["projectionSha256"] != chain["projectionSha256"]:
        return Refusal("CLAIM_INVALID", "correspondence binds a different context/projection")
    if observation_facts is not None and correspondence["observationSha256"] != observation_facts["observationSha256"]:
        return Refusal("CLAIM_INVALID", "correspondence observation digest is stale")
    if observation_facts is not None and correspondence["preflightChargeId"] != observation_facts["observation"]["preflightChargeId"]:
        return Refusal("CLAIM_INVALID", "correspondence preflight charge differs from observation")
    if not _nonempty(correspondence["runtimeChargeId"]) or not _nonempty(correspondence["reservationId"]):
        return Refusal("CLAIM_INVALID", "runtime charge/reservation not yet committed")
    return {"correspondence": correspondence, "correspondenceSha256": _sha(data)}


# --- master claim and store claim -------------------------------------------

def _master_bytes(chain):
    path = Path(chain["masterPath"])
    if path.is_symlink() or not path.is_file():
        return None
    try:
        return path.read_bytes()
    except OSError:
        return None


def _seconds_sum(rows):
    total = Decimal(0)
    if type(rows) is not list:
        return None
    for row in rows:
        value = row.get("seconds") if type(row) is dict else None
        if type(value) is bool or not isinstance(value, (int, Decimal)) or value < 0:
            return None
        total += Decimal(value)
    return total


def _decimal_field(payload, key):
    value = payload.get(key)
    if type(value) is bool or not isinstance(value, (int, Decimal)) or value < 0:
        return None
    return Decimal(value)


def _ceil(value):
    return int(value.to_integral_value(rounding=ROUND_CEILING))


def verify_grant_invariants(chain, master):
    """Predecessor M/U -> C/D -> current limit, preserved rows and reserve transfer."""
    projection = chain["projection"]
    predecessor = chain["predecessorMaster"]
    grant = projection["grantSeconds"]
    dispatches = projection["grantDispatches"]
    limit = _decimal_field(predecessor, "master_limit_seconds")
    parts = [_decimal_field(predecessor, "planning_charge_seconds"),
             _decimal_field(predecessor, "planning_overhead_reserved_seconds"),
             _seconds_sum(predecessor.get("charges")), _seconds_sum(predecessor.get("reserved")),
             _seconds_sum(predecessor.get("externalPackageCharges"))]
    if limit is None or any(part is None for part in parts):
        return "predecessor master arithmetic fields"
    used = sum(parts, Decimal(0))
    closure = _ceil(max(Decimal(0), limit - used))
    adjustment = max(0, _ceil(used + Decimal(closure)) - int(limit))
    if projection["closureSeconds"] != closure or projection["closureAdjustmentSeconds"] != adjustment:
        return "projection C/D differ from the exact predecessor arithmetic"
    if master.get("master_limit_seconds") != int(limit) + adjustment + grant:
        return "current master limit is not M + D + G"
    for key in PRESERVED_PREDECESSOR_FIELDS:
        if master.get(key) != predecessor.get(key):
            return "predecessor field changed: " + key
    if master.get("package") != "MC01":
        return "ledger owner is not MC01"
    reserved = master.get("reserved")
    old_reserved = predecessor.get("reserved") or []
    if type(reserved) is not list or reserved[:len(old_reserved)] != old_reserved:
        return "historical reserved rows changed"
    new_rows = reserved[len(old_reserved):]
    for row in new_rows:
        if not records._exact_keys(row, RESERVE_ROW_KEYS, [], "reserve-row") or not _posint(row["seconds"]) and row["seconds"] != 0:
            return "new reserve row fields"
        if row["amendmentSha256"] != chain["admissionSha256"] or not records._hex64(row["sourceSha256"]):
            return "new reserve row provenance"
    closure_rows = [row for row in new_rows if row["basis"] == CLOSURE_BASIS]
    envelope_rows = [row for row in new_rows if row["basis"] == RESERVE_BASIS]
    if len(closure_rows) != (1 if closure > 0 else 0) or (closure_rows and closure_rows[0]["seconds"] != closure):
        return "closure reserve row differs from C"
    if len(envelope_rows) != 1 or len(new_rows) != len(closure_rows) + 1:
        return "exactly one envelope reserve row is required"
    return {"reserveRow": envelope_rows[0], "grant": grant, "dispatches": dispatches,
            "predecessorExternalRows": predecessor.get("externalPackageCharges") or [],
            "predecessorExternalDispatches": predecessor.get("externalWorkerDispatches", 0),
            "predecessorDispatchLimit": predecessor.get("master_worker_dispatch_limit", 0)}


def verify_master_claim(chain, correspondence, charge_id, reservation_id, invocation):
    projection = chain["projection"]
    data = _master_bytes(chain)
    if data is None:
        return Refusal("CLAIM_INVALID", "master unavailable")
    if _sha(data) != correspondence["masterSha256"]:
        return Refusal("CLAIM_INVALID", "master generation differs from correspondence; reconciliation required")
    missing = []
    try:
        master = json.loads(data.decode("utf-8"), parse_float=Decimal)
    except (UnicodeDecodeError, ValueError):
        master = None
    if type(master) is not dict:
        return Refusal("CLAIM_INVALID", "master is not an object", missing)
    invariants = verify_grant_invariants(chain, master)
    if isinstance(invariants, str):
        return Refusal("CLAIM_INVALID", "grant invariants: " + invariants)
    envelope_id = projection["envelopeId"]
    grant = projection["grantSeconds"]
    dispatch_limit = projection["grantDispatches"]
    rows = master.get("externalPackageCharges")
    if type(rows) is not list:
        return Refusal("CLAIM_INVALID", "externalPackageCharges missing")
    spent = 0
    count = 0
    own = []
    for row in rows:
        if type(row) is not dict:
            return Refusal("CLAIM_INVALID", "malformed charge row")
        if row.get("package") != "MC02" or row.get("envelope") != envelope_id:
            continue
        if not records._exact_keys(row, DEBIT_KEYS, missing, "debit"):
            return Refusal("CLAIM_INVALID", "debit row fields", missing)
        if not _posint(row["seconds"]):
            return Refusal("CLAIM_INVALID", "debit seconds")
        spent += row["seconds"]
        count += 1
        if row["id"] == charge_id:
            own.append(row)
    if len(own) != 1:
        return Refusal("CLAIM_INVALID", "exactly one master debit row for this charge is required")
    row = own[0]
    expected = {
        "actionId": invocation["actionId"], "candidateHash": invocation["candidateHash"],
        "runnerDigest": invocation["runnerDigest"], "storeIdentity": chain["storeIdentity"],
        "reservationId": reservation_id, "launchClaim": "claimed", "purpose": "financial-runtime",
    }
    for key, value in expected.items():
        if row[key] != value:
            return Refusal("CLAIM_INVALID", "master debit " + key + " differs from current invocation")
    if row["seconds"] < MIN_RUNTIME_DEBIT_SECONDS:
        return Refusal("CLAIM_INVALID", "runtime debit below the minimum prepaid bound")
    if spent > grant or count > dispatch_limit:
        return Refusal("CLAIM_INVALID", "envelope over-committed")
    old_rows = invariants["predecessorExternalRows"]
    if rows[:len(old_rows)] != old_rows or any(r.get("package") != "MC02" or r.get("envelope") != envelope_id
                                              for r in rows[len(old_rows):]):
        return Refusal("CLAIM_INVALID", "predecessor external rows changed or foreign rows appended")
    if invariants["reserveRow"]["seconds"] != grant - spent:
        return Refusal("CLAIM_INVALID", "envelope reserve row does not equal G - S (reserve transfer)")
    if master.get("externalWorkerDispatches") != invariants["predecessorExternalDispatches"] + count \
            or master.get("master_worker_dispatch_limit") != invariants["predecessorDispatchLimit"] + dispatch_limit:
        return Refusal("CLAIM_INVALID", "aggregate dispatch counters do not recompute")
    envelopes = master.get("successorEnvelopes")
    if type(envelopes) is not dict or type(envelopes.get(envelope_id)) is not dict:
        return Refusal("CLAIM_INVALID", "current envelope missing")
    envelope = envelopes[envelope_id]
    if not records._exact_keys(envelope, ENVELOPE_KEYS, missing, "envelope"):
        return Refusal("CLAIM_INVALID", "envelope fields", missing)
    if envelope["owner"] != "MC02" or envelope["projectionSha256"] != chain["projectionSha256"] \
            or envelope["amendmentSha256"] != chain["admissionSha256"]:
        return Refusal("CLAIM_INVALID", "envelope provenance")
    # Own prepaid equality: A = G - S + c >= c is exactly S <= G. The own charge
    # is already inside S and is never subtracted a second time.
    if envelope["remainingReservedSeconds"] != grant - spent or envelope["workerDispatches"] != count \
            or envelope["workerDispatchLimit"] != dispatch_limit:
        return Refusal("CLAIM_INVALID", "envelope counters do not recompute from committed rows")
    return {"masterSha256": _sha(data), "debit": row, "envelopeRemainingSeconds": grant - spent,
            "prepaidAvailableSeconds": grant - spent + row["seconds"]}


def verify_store_claim(chain, plan, charge_id, reservation_id, invocation, db_path=None):
    db_path = Path(db_path or chain["dbPath"])
    if not db_path.is_file():
        return Refusal("CLAIM_INVALID", "canonical store unavailable")
    claim = store.claim_for_charge(db_path, charge_id)
    if claim is None:
        return Refusal("CLAIM_INVALID", "no SQLite runner claim for this charge")
    if claim["reservationId"] != reservation_id or claim["status"] != "active":
        return Refusal("CLAIM_INVALID", "runner claim belongs to another or inactive reservation")
    if claim["actionId"] != plan.get("actionId") or claim["candidate"] != plan.get("candidateHash"):
        return Refusal("CLAIM_INVALID", "reservation lineage differs from plan")
    digest = store.canonical_digest(invocation)
    if store.find_invocation_event(db_path, digest) != invocation:
        return Refusal("CLAIM_INVALID", "durable invocation event is absent or different in the canonical store")
    return {"invocationSha256": digest, "claim": claim}


# --- ownership --------------------------------------------------------------

def ownership_path(chain):
    return Path(chain["context"]["observer"]["lockPath"] + ".ownership.json")


def verify_ownership(chain, charge_id, reservation_id, invocation):
    path = ownership_path(chain)
    if path.is_symlink() or not path.is_file():
        return Refusal("OWNERSHIP_UNRESOLVED", "wallet ownership record unavailable")
    missing = []
    try:
        data = path.read_bytes()
    except OSError:
        return Refusal("OWNERSHIP_UNRESOLVED", "wallet ownership record unreadable")
    record = records._parse_json(data, missing, "ownership")
    if not records._exact_keys(record, OWNERSHIP_KEYS, missing, "ownership"):
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership fields", missing)
    if record["schema"] != OWNERSHIP_SCHEMA or record["state"] != OWNERSHIP_STATE_RUNTIME:
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership schema/state")
    expected = {
        "allocationId": chain["context"]["expected"]["allocationId"],
        "executionContextSha256": chain["contextSha256"],
        "projectionSha256": chain["projectionSha256"],
        "storeIdentity": chain["storeIdentity"],
        "runtimeChargeId": charge_id,
        "runtimeReservationId": reservation_id,
    }
    for key, value in expected.items():
        if record[key] != value:
            return Refusal("OWNERSHIP_UNRESOLVED", "ownership " + key + " differs")
    if not _nonempty(record["preflightReservationId"]) or not _nonempty(record["preflightChargeId"]):
        return Refusal("OWNERSHIP_UNRESOLVED", "preflight ownership lineage missing")
    if invocation is not None and (record["ownerPid"] != invocation["parentPid"]
                                   or record["ownerStartTicks"] != invocation["parentStartTicks"]):
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership is not held by the invoking parent")
    return {"ownership": record, "ownershipSha256": _sha(data), "path": str(path)}


# --- executor entry (validate_current) ----------------------------------------

def validate_invocation_shape(invocation):
    missing = []
    if not records._exact_keys(invocation, INVOCATION_KEYS, missing, "invocation"):
        return Refusal("AUTHORITY_INVALID", "invocation fields", missing)
    if invocation["schema"] != INVOCATION_SCHEMA:
        return Refusal("AUTHORITY_INVALID", "invocation schema")
    for key in ("outerStartMonotonic", "outerDeadlineMonotonic", "parentPid", "parentStartTicks",
                "launcherPid", "launcherStartTicks"):
        if not _nonneg(invocation[key]):
            return Refusal("AUTHORITY_INVALID", "invocation " + key)
    for key in ("runnerDigest", "executionContextSha256", "projectionSha256", "correspondenceSha256",
                "authoritySha256", "nonceSha256"):
        if not records._hex64(invocation[key]):
            return Refusal("AUTHORITY_INVALID", "invocation " + key)
    return None


def validate_executor_context(plan, admission, invocation, *, root, wall_time_ms=None, financial_plan=None):
    """Production validate_current: authenticate the complete current chain.

    Returns CurrentExecution or a Refusal whose code is one of AUTHORITY_INVALID,
    HISTORY_UNRESOLVED, OBSERVATION_INVALID, OWNERSHIP_UNRESOLVED, CLAIM_INVALID.
    """
    shape = validate_invocation_shape(invocation)
    if shape is not None:
        return shape
    chain = authenticate_authority(root, plan, admission)
    if isinstance(chain, Refusal):
        return chain
    if invocation["authoritySha256"] != chain["admissionSha256"] \
            or invocation["executionContextSha256"] != chain["contextSha256"] \
            or invocation["projectionSha256"] != chain["projectionSha256"] \
            or invocation["storeIdentity"] != chain["storeIdentity"] \
            or invocation["blockDeadlineUtc"] != chain["blockDeadlineUtc"] \
            or invocation["allocationId"] != plan.get("allocationId") \
            or invocation["actionId"] != plan.get("actionId") \
            or invocation["candidateHash"] != plan.get("candidateHash"):
        return Refusal("AUTHORITY_INVALID", "invocation identities differ from authenticated authority")
    history_facts = verify_history(chain)
    if isinstance(history_facts, Refusal):
        return history_facts
    if wall_time_ms is None:
        wall_time_ms = int(datetime.datetime.now(datetime.timezone.utc).timestamp() * 1000)
    observation_facts = verify_observation(chain, history_facts, wall_time_ms, financial_plan)
    if isinstance(observation_facts, Refusal):
        return observation_facts
    charge_id = invocation["chargeId"]
    reservation_id = invocation["reservationId"]
    ownership = verify_ownership(chain, charge_id, reservation_id, invocation)
    if isinstance(ownership, Refusal):
        return ownership
    correspondence = verify_correspondence(chain, observation_facts)
    if isinstance(correspondence, Refusal):
        return correspondence
    if correspondence["correspondenceSha256"] != invocation["correspondenceSha256"]:
        return Refusal("CLAIM_INVALID", "correspondence changed since the invocation was recorded")
    current = correspondence["correspondence"]
    if current["runtimeChargeId"] != charge_id or current["reservationId"] != reservation_id:
        return Refusal("CLAIM_INVALID", "correspondence runtime identities differ from the claimed debit")
    master = verify_master_claim(chain, current, charge_id, reservation_id, invocation)
    if isinstance(master, Refusal):
        return master
    store_claim = verify_store_claim(chain, plan, charge_id, reservation_id, invocation)
    if isinstance(store_claim, Refusal):
        return store_claim
    return CurrentExecution({
        "actionId": plan["actionId"], "candidateHash": plan["candidateHash"], "allocationId": plan["allocationId"],
        "storePath": str(chain["dbPath"]), "storeIdentity": chain["storeIdentity"],
        "masterPath": chain["masterPath"], "masterSha256": master["masterSha256"],
        "chargeId": charge_id, "reservationId": reservation_id,
        "debitSeconds": master["debit"]["seconds"],
        "envelopeRemainingSeconds": master["envelopeRemainingSeconds"],
        "prepaidAvailableSeconds": master["prepaidAvailableSeconds"],
        "executionContextSha256": chain["contextSha256"], "projectionSha256": chain["projectionSha256"],
        "observationSha256": observation_facts["observationSha256"],
        "correspondenceSha256": correspondence["correspondenceSha256"],
        "historySha256": chain["historySha256"], "authoritySha256": chain["admissionSha256"],
        "invocationSha256": store_claim["invocationSha256"],
        "ownership": ownership["ownership"], "ownershipSha256": ownership["ownershipSha256"],
        "ownershipPath": ownership["path"],
        "blockDeadlineUtc": chain["blockDeadlineUtc"], "blockDeadlineMs": chain["blockDeadlineMs"],
        "observationAgeMs": max(observation_facts["agesMs"].values()),
        "observationPath": str(observation_facts["path"]),
        "proverContainer": chain["proverContainer"], "proverContainerSha256": chain["proverContainerSha256"],
    })


# --- parent (runner) side ------------------------------------------------------

def parent_current_state(root, plan, admission, wall_time_ms=None):
    """Runner-side producer state: authenticated digests and current claim identities.

    Used before the invocation event is appended and again before the
    handshake answer. It reads the same records the child later verifies.
    """
    chain = authenticate_authority(root, plan, admission)
    if isinstance(chain, Refusal):
        return chain
    history_facts = verify_history(chain)
    if isinstance(history_facts, Refusal):
        return history_facts
    if wall_time_ms is None:
        wall_time_ms = int(datetime.datetime.now(datetime.timezone.utc).timestamp() * 1000)
    observation_facts = verify_observation(chain, history_facts, wall_time_ms)
    if isinstance(observation_facts, Refusal):
        return observation_facts
    correspondence = verify_correspondence(chain, observation_facts)
    if isinstance(correspondence, Refusal):
        return correspondence
    current = correspondence["correspondence"]
    return {
        "storePath": str(chain["dbPath"]), "storeIdentity": chain["storeIdentity"],
        "executionContextSha256": chain["contextSha256"], "projectionSha256": chain["projectionSha256"],
        "correspondenceSha256": correspondence["correspondenceSha256"],
        "observationSha256": observation_facts["observationSha256"],
        "authoritySha256": chain["admissionSha256"],
        "chargeId": current["runtimeChargeId"], "reservationId": current["reservationId"],
        "blockDeadlineUtc": chain["blockDeadlineUtc"], "blockDeadlineMs": chain["blockDeadlineMs"],
        "chain": chain, "correspondence": current,
    }


def parent_claim_check(state, plan, invocation):
    """Before answering the handshake: store event, reservation, master claim, ownership."""
    chain = state["chain"]
    ownership = verify_ownership(chain, invocation["chargeId"], invocation["reservationId"], invocation)
    if isinstance(ownership, Refusal):
        return ownership
    correspondence = verify_correspondence(chain, None)
    if isinstance(correspondence, Refusal):
        return correspondence
    if correspondence["correspondenceSha256"] != invocation["correspondenceSha256"]:
        return Refusal("CLAIM_INVALID", "correspondence changed after invocation append")
    master = verify_master_claim(chain, correspondence["correspondence"], invocation["chargeId"],
                                 invocation["reservationId"], invocation)
    if isinstance(master, Refusal):
        return master
    claim = verify_store_claim(chain, plan, invocation["chargeId"], invocation["reservationId"], invocation)
    if isinstance(claim, Refusal):
        return claim
    return {"invocationSha256": claim["invocationSha256"], "masterSha256": master["masterSha256"]}


# --- runtime handover (tail of the producer sequence) ----------------------------
# The full producer (predebit gate, preflight reservation and observer, debit
# commit, grant application) is pending. This tail performs only the specified
# one-shot runtime claim on records that already exist: an unclaimed runtime
# debit row bound to this exact action/candidate/runner, a preflight-held
# wallet ownership record and a correspondence without a runtime reservation.
OWNERSHIP_STATE_PREFLIGHT = "preflight-held"
OWNERSHIP_STATE_RELEASED = "released"


def _write_replacement(temp, data):
    fd = os.open(str(temp), os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
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


def _finish_replacement(temp, path):
    os.rename(temp, path)
    dirfd = os.open(str(Path(path).parent), os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(dirfd)
    finally:
        os.close(dirfd)


class MutationFailed(Exception):
    """A supervised master/correspondence/ownership replacement did not complete."""


def _replace_file(path, data, bound_seconds=None):
    """Supervised replacement: temp write+fsync then rename+dir fsync in bounded workers.

    A worker that misses its bound is killed; the target keeps its previous
    bytes and the temp file stays as evidence. Nothing here is atomic across
    files: interrupted generations deny until reconciliation.
    """
    path = Path(path)
    temp = path.with_name(path.name + ".tmp-" + str(os.getpid()))
    bound = WRITE_BOUND_SECONDS if bound_seconds is None else bound_seconds
    outcome = run_bounded(lambda: _write_replacement(temp, data), bound)
    if not outcome.completed:
        raise MutationFailed("replacement write %s: %s" % (outcome.status, outcome.detail))
    outcome = run_bounded(lambda: _finish_replacement(temp, path), bound)
    if not outcome.completed:
        raise MutationFailed("replacement rename %s: %s" % (outcome.status, outcome.detail))


class BudgetLock(object):
    """Bounded acquisition of the existing predecessor-derived budget lock."""

    def __init__(self, chain):
        self.path = Path(chain["masterPath"]).with_suffix(".lock")
        self.fd = None

    def __enter__(self):
        self.fd = os.open(str(self.path), os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
        deadline = time.monotonic() + LOCK_WAIT_SECONDS
        while True:
            try:
                fcntl.flock(self.fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                return self
            except OSError:
                if time.monotonic() >= deadline:
                    os.close(self.fd)
                    self.fd = None
                    raise MutationFailed("budget lock held by another owner")
                time.sleep(0.05)

    def __exit__(self, *exc):
        if self.fd is not None:
            os.close(self.fd)
            self.fd = None
        return False


def _dump(payload):
    return (json.dumps(payload, indent=2, sort_keys=True) + "\n").encode("utf-8")


def parent_pre_handover_state(root, plan, admission, wall_time_ms=None):
    """Authenticated chain, history and observation before the runtime claim exists."""
    chain = authenticate_authority(root, plan, admission)
    if isinstance(chain, Refusal):
        return chain
    history_facts = verify_history(chain)
    if isinstance(history_facts, Refusal):
        return history_facts
    if wall_time_ms is None:
        wall_time_ms = int(datetime.datetime.now(datetime.timezone.utc).timestamp() * 1000)
    observation_facts = verify_observation(chain, history_facts, wall_time_ms)
    if isinstance(observation_facts, Refusal):
        return observation_facts
    return {"chain": chain, "observation": observation_facts}


def runtime_handover(root, plan, admission, charge_id, reservation_id, runner_digest, owner_pid, owner_ticks,
                     wall_time_ms=None):
    """One-shot runtime claim: master row unclaimed -> claimed, correspondence and ownership updated.

    Returns parent_current_state on success or a Refusal. Never creates a
    debit, never refunds, never repeats a claimed transition.
    """
    pre = parent_pre_handover_state(root, plan, admission, wall_time_ms)
    if isinstance(pre, Refusal):
        return pre
    chain = pre["chain"]
    try:
        with BudgetLock(chain):
            return _runtime_handover_locked(root, plan, admission, chain, pre["observation"], charge_id,
                                            reservation_id, runner_digest, owner_pid, owner_ticks, wall_time_ms)
    except MutationFailed as error:
        return Refusal("CLAIM_INVALID", str(error) + "; consumed/unavailable until reconciliation")


def _runtime_handover_locked(root, plan, admission, chain, observation_facts, charge_id, reservation_id,
                             runner_digest, owner_pid, owner_ticks, wall_time_ms):
    missing = []
    loaded = _mutable_json(root, chain["context"]["correspondencePath"], "correspondence", missing)
    if loaded is None:
        return Refusal("CLAIM_INVALID", "correspondence", missing)
    corr_path, corr_bytes, correspondence = loaded
    if not records._exact_keys(correspondence, CORRESPONDENCE_KEYS, missing, "correspondence") \
            or correspondence["schema"] != CORRESPONDENCE_SCHEMA:
        return Refusal("CLAIM_INVALID", "correspondence fields", missing)
    if correspondence["executionContextSha256"] != chain["contextSha256"] \
            or correspondence["projectionSha256"] != chain["projectionSha256"] \
            or correspondence["observationSha256"] != observation_facts["observationSha256"]:
        return Refusal("CLAIM_INVALID", "correspondence binds different context/projection/observation")
    if correspondence["runtimeChargeId"] != charge_id:
        return Refusal("CLAIM_INVALID", "correspondence runtime charge differs from the bound runner charge")
    if correspondence["reservationId"] is not None:
        return Refusal("CLAIM_INVALID", "runtime reservation already handed over; no second claim")
    master_data = _master_bytes(chain)
    if master_data is None or _sha(master_data) != correspondence["masterSha256"]:
        return Refusal("CLAIM_INVALID", "master generation differs from correspondence; reconciliation required")
    master = records._parse_json(master_data, missing, "master")
    if type(master) is not dict or type(master.get("externalPackageCharges")) is not list:
        return Refusal("CLAIM_INVALID", "master rows", missing)
    envelope_id = chain["projection"]["envelopeId"]
    own = [row for row in master["externalPackageCharges"]
           if type(row) is dict and row.get("id") == charge_id and row.get("package") == "MC02"
           and row.get("envelope") == envelope_id]
    if len(own) != 1:
        return Refusal("CLAIM_INVALID", "exactly one master debit row for this charge is required")
    row = own[0]
    if not records._exact_keys(row, DEBIT_KEYS, missing, "debit"):
        return Refusal("CLAIM_INVALID", "debit row fields", missing)
    if row["launchClaim"] != "unclaimed":
        return Refusal("CLAIM_INVALID", "master launch claim already consumed")
    if row["purpose"] != "financial-runtime" or row["actionId"] != plan["actionId"] \
            or row["candidateHash"] != plan["candidateHash"] or row["runnerDigest"] != runner_digest \
            or row["storeIdentity"] != chain["storeIdentity"]:
        return Refusal("CLAIM_INVALID", "master debit is bound to another action/candidate/runner/store")
    if not _posint(row["seconds"]) or row["seconds"] < MIN_RUNTIME_DEBIT_SECONDS:
        return Refusal("CLAIM_INVALID", "runtime debit below the minimum prepaid bound")
    ownership_file = ownership_path(chain)
    if ownership_file.is_symlink() or not ownership_file.is_file():
        return Refusal("OWNERSHIP_UNRESOLVED", "wallet ownership record unavailable")
    ownership = records._parse_json(ownership_file.read_bytes(), missing, "ownership")
    if not records._exact_keys(ownership, OWNERSHIP_KEYS, missing, "ownership") or ownership["schema"] != OWNERSHIP_SCHEMA:
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership fields", missing)
    if ownership["state"] != OWNERSHIP_STATE_PREFLIGHT or ownership["runtimeReservationId"] is not None:
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership is not preflight-held awaiting runtime handover")
    if ownership["executionContextSha256"] != chain["contextSha256"] or ownership["projectionSha256"] != chain["projectionSha256"] \
            or ownership["storeIdentity"] != chain["storeIdentity"] \
            or ownership["allocationId"] != chain["context"]["expected"]["allocationId"]:
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership binds a different context/projection/store")
    if ownership["preflightReservationId"] != row["reservationId"] or ownership["preflightChargeId"] != correspondence["preflightChargeId"]:
        return Refusal("OWNERSHIP_UNRESOLVED", "preflight ownership lineage differs from the master row")
    # Master first, then correspondence, then ownership. Interruption leaves a
    # correspondence/master disagreement that denies until reconciliation.
    row["reservationId"] = reservation_id
    row["launchClaim"] = "claimed"
    _replace_file(chain["masterPath"], _dump(master))
    new_master_sha = _sha(Path(chain["masterPath"]).read_bytes())
    correspondence["masterSha256"] = new_master_sha
    correspondence["reservationId"] = reservation_id
    _replace_file(corr_path, _dump(correspondence))
    ownership["runtimeChargeId"] = charge_id
    ownership["runtimeReservationId"] = reservation_id
    ownership["ownerPid"] = owner_pid
    ownership["ownerStartTicks"] = owner_ticks
    ownership["state"] = OWNERSHIP_STATE_RUNTIME
    _replace_file(ownership_file, _dump(ownership))
    return parent_current_state(root, plan, admission, wall_time_ms)


def release_ownership(root, plan, admission, reservation_id):
    """After validated durable containment: mark the runtime ownership released."""
    chain = authenticate_authority(root, plan, admission)
    if isinstance(chain, Refusal):
        return chain
    ownership_file = ownership_path(chain)
    missing = []
    if ownership_file.is_symlink() or not ownership_file.is_file():
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership record unavailable")
    ownership = records._parse_json(ownership_file.read_bytes(), missing, "ownership")
    if not records._exact_keys(ownership, OWNERSHIP_KEYS, missing, "ownership") \
            or ownership["runtimeReservationId"] != reservation_id or ownership["state"] != OWNERSHIP_STATE_RUNTIME:
        return Refusal("OWNERSHIP_UNRESOLVED", "ownership is not held by this runtime reservation")
    ownership["state"] = OWNERSHIP_STATE_RELEASED
    try:
        with BudgetLock(chain):
            _replace_file(ownership_file, _dump(ownership))
    except MutationFailed as error:
        return Refusal("OWNERSHIP_UNRESOLVED", "release not durable: " + str(error))
    return {"released": True}
