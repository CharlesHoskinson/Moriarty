"""Exact-source readers for Moriarty development plugin snapshots.

``load_snapshot(repo, action_id, history_reader=None)`` and ``read_actions(repo)``
read current bytes. They never write, never launch ``commandRef`` argv, never
import referenced code, and never open secret paths. Missing or conflicting
sources become named ``missingEvidence`` strings. Missing operational history
is ``operational-history`` and is not a verified zero-failure count.

This module does not grant admission. New action mappings cannot create
acceptance or resource permission. Existing program, sprint and campaign
records remain authoritative.

On-disk reader contract (paths are relative to the supplied repository root):

- ``.moriarty-dev/actions.json``
  Closed object. Required keys: ``schema``, ``actions``.
  ``schema`` must be ``moriarty-dev.actions/1``.
  ``actions`` is a list of closed action objects with exactly:
  ``id``, ``requirement``, ``capability``, ``kind``, ``candidate``,
  ``admissionRef``, ``commandRef``, ``evidenceProfile`` (all ``str``).
  Duplicate ids, duplicate JSON keys, unknown keys and non-string values
  are unresolved. The catalog maps references. It is not an acceptance
  register.

- ``openspec/moriarty-completion-program.json``
  ``schemaVersion`` must be the integer ``1``. Task eligibility uses
  ``reportReconciliation.stageAdmission`` stages and sprint ``entryGates``.
  ``stageAdmission`` and each stage are closed shapes. ``dispatchEnabled``
  never admits work. Whole-sprint ``status`` never decides entry.

- ``openspec/sprints/sprints.json``
  ``schemaVersion`` must be the integer ``1``. Entry uses the gate that
  lists the action ``requirement``. ``requires`` and ``campaignOwner`` are
  required. A blocked ``f0`` stage does not make an SP05 task ineligible
  unless that task's own gate lists ``f0``.

- Campaign store from ``stageAdmission.campaignRecordStore``.
  Schema ``moriarty.campaign-admission/1``. ``admissionRef`` form
  ``campaign:<id>``. The record is not permission. The action gate stage
  and campaignOwner must match the current campaign, the program stage
  ``campaignRecordId``, and the accepted profile. Complete historical
  design records do not admit source implementation.

- Binding files: schema ``moriarty.sp01-execution-binding/1`` only.
  Bytes must match ``bindingSha256``. Binding inputs/ownedFiles and
  authority are references, not self-authenticating permission.

- Candidate currentness uses ``candidateManifest`` (candidateSha256,
  digestAlgorithm, ownedFiles, inputs) plus acceptance/review when
  present. Frozen owned-file digests are the candidate commitment.
  ``unchangedPayloadHashes`` must equal those freeze digests; they
  cannot rewrite them. A publication bridge authenticates original
  freeze bytes, then permits only ``subsets.RP01-MC02`` acceptance,
  control metadata and closure wording. A matching mutable digest
  cannot replace frozen control. Missing original bytes are
  unavailable. Prerequisites reuse binding, candidate, review and
  acceptance checks. Action, stage, campaign, profile, review and
  candidate identities must agree. Review verdict ``APPROVED`` is the
  supported pass; ``NOT_REVIEWED`` is unresolved.

- Resource files: authority path, matching scope and a positive integer
  allocation or additionalSeconds. Live remaining uses an explicit
  ``currentAccounting`` current-runner record of family
  ``moriarty.supervised-accounting/1``, never
  ``resourceRuntimeSnapshot``. Planning charges, reservations,
  amendments, dispatch limits and the action's bounded cost apply. A
  historical integer snapshot is not credit. The existing runner owns
  charging.

- ``commandRef``: empty only for ``report``. Actionful kinds need
  ``<contained-json>#<name>`` with schema ``moriarty-dev.commands/1`` and
  a nonempty argv list of strings. The argv list is never executed.

Reads are bounded at 8 MiB. Duplicate JSON keys are rejected. Contained
reads apply to fixed program/sprint files and references. Resolved path
components are checked for secret names before opening.

``history_reader`` is ``reader({"repository", "requirement", "capability"})``.
``repository`` is the resolved git-common-dir. Source reads use the
selected worktree. Absence of a reader is ``operational-history``.
``nextActionId`` is selected from currently admitted catalog actions in
order reproduce, repair, review, verify, implement, admin, report.
Named absence is ``next-action-none``.
"""
from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path

from moriarty_dev.policy import assess

ACTION_KEYS = (
    "id",
    "requirement",
    "capability",
    "kind",
    "candidate",
    "admissionRef",
    "commandRef",
    "evidenceProfile",
)
SNAPSHOT_KEYS = (
    "authorityCurrent",
    "entryEligible",
    "candidateCurrent",
    "resourceAdmitted",
    "sameDefectFailures",
    "adminCycles",
    "adminSeconds",
    "primaryActive",
    "reproducerVerified",
    "approachChanged",
    "nextActionId",
    "missingEvidence",
)
HISTORY_INT_KEYS = ("sameDefectFailures", "adminCycles", "adminSeconds")
HISTORY_BOOL_KEYS = ("primaryActive", "reproducerVerified", "approachChanged")
HISTORY_KEYS = HISTORY_INT_KEYS + HISTORY_BOOL_KEYS
STAGE_KEYS = (
    "id",
    "owners",
    "requires",
    "status",
    "acceptedProfile",
    "candidateHash",
    "campaignRecordId",
    "purpose",
)
STAGE_ADMISSION_KEYS = (
    "schema",
    "dispatchEnabled",
    "campaignRecordStore",
    "rule",
    "stages",
)
GATE_KEYS = ("stage", "requires", "campaignOwner", "tasks")
ACTIONS_REL = ".moriarty-dev/actions.json"
PROGRAM_REL = "openspec/moriarty-completion-program.json"
SPRINTS_REL = "openspec/sprints/sprints.json"
DEFAULT_CAMPAIGN_REL = (
    "evidence/moriarty-completion-program-2026-09-07/"
    "report-reconciliation/campaign-admission.json"
)
ACTIONS_SCHEMA = "moriarty-dev.actions/1"
COMMANDS_SCHEMA = "moriarty-dev.commands/1"
CAMPAIGN_SCHEMA = "moriarty.campaign-admission/1"
BINDING_SCHEMA = "moriarty.sp01-execution-binding/1"
ACCOUNTING_SCHEMA = "moriarty.supervised-accounting/1"
STAGE_ADMISSION_SCHEMA = "openspec/report-stage-admission.schema.json"
PROGRAM_VERSION = 1
SPRINTS_VERSION = 1
MAX_BYTES = 8 * 1024 * 1024
PROGRAM_KEYS = (
    "acceptanceLineage",
    "auditEfforts",
    "auditModels",
    "authority",
    "charter",
    "currentAdmission",
    "currentAtomicFixturePreparation",
    "executionAuthority",
    "extensionOwnership",
    "latestAfkExecutionAuthority",
    "midnightTransactionReportingAuthority",
    "networkMilestoneTestingAuthority",
    "packages",
    "program",
    "publicationPolicy",
    "reportReconciliation",
    "resourceAdmission",
    "resourceAmendments",
    "resourceReservations",
    "resourceRuntimeSnapshot",
    "reviewAuthority",
    "runtimeGoalId",
    "schemaVersion",
    "sprintPlan",
    "status",
)
CAMPAIGN_RECORD_KEYS = (
    "acceptance",
    "acceptedProfile",
    "binding",
    "bindingSha256",
    "candidateHash",
    "candidateManifest",
    "candidateSha256",
    "checks",
    "currentAccounting",
    "currentFindingReview",
    "dispatchCondition",
    "executor",
    "owner",
    "publication",
    "reconciliation",
    "recordedAt",
    "records",
    "resourceAmendment",
    "review",
    "reviewRequirement",
    "scope",
    "sourceReview",
    "stage",
    "status",
    "successorAmendments",
    "successorReviewAmendment",
    "timingDisclosure",
)
MANIFEST_KEYS = (
    "baseCommit",
    "candidateSha256",
    "diffSha256",
    "digestAlgorithm",
    "inputs",
    "ownedFiles",
    "priorCandidate",
    "schema",
    "scope",
    "status",
    "timeoutPartial",
    "verification",
)
ACCEPTANCE_KEYS = (
    "candidateHash",
    "derivation",
    "publishedControlPath",
    "publishedControlSha256",
    "review",
    "sourceCandidate",
    "status",
    "transactionIds",
    "unchangedPayloadHashes",
)
ACCEPTANCE_REQUIRED = (
    "candidateHash",
    "derivation",
    "publishedControlPath",
    "publishedControlSha256",
    "review",
    "sourceCandidate",
    "status",
)
PUBLICATION_SUBSET = "RP01-MC02"
PUBLICATION_SCHEMA = "moriarty.semantic-challenges/1"
PUBLICATION_MUTABLE = (
    "acceptance",
    "candidateHash",
    "candidateHashStatus",
    "reviews",
    "status",
)
REVIEW_VERDICT_PASS = "APPROVED"
REVIEW_VERDICT_FAIL = frozenset(("BLOCKED", "REJECTED"))
REVIEW_REQUIRED = (
    "candidateSha256",
    "recordedAt",
    "reviewer",
    "scope",
    "verdict",
)
REVIEW_KEYS = (
    "agent",
    "candidateSha256",
    "effort",
    "excluded",
    "findings",
    "integrity",
    "priorSatisfiedFindings",
    "publicTransactions",
    "recordedAt",
    "resolved",
    "resourceAllowanceSeconds",
    "resourceEnforcement",
    "reviewer",
    "scope",
    "verdict",
)
ACCOUNTING_KEYS = (
    "accounting",
    "amendment_sha256",
    "authority_sha256",
    "charges",
    "delegated_authority",
    "dispatch_amendment_sha256",
    "externalPackageCharges",
    "externalWorkerDispatchLimits",
    "externalWorkerDispatches",
    "gross_tnight_debit",
    "master_limit_seconds",
    "master_worker_dispatch_limit",
    "native_campaigns",
    "package",
    "package_limit_seconds",
    "planning_charge_seconds",
    "planning_overhead_reserved_seconds",
    "planning_source_sha256",
    "preview_submissions",
    "reserved",
    "schema",
    "successorEnvelopes",
    "worker_dispatch_limit",
    "worker_dispatches",
)
CURRENT_ACCOUNTING_KEYS = ("schema", "path", "passBinding", "passLedger")
PASS_LEDGER_KEYS = ("charges", "priorStop", "reason", "stopped", "workerDispatches")
RESOURCE_SCHEMAS = frozenset(
    (
        "moriarty.delegated-resource-amendment/1",
        "moriarty.packet-preparation/1",
        "moriarty.delegated-successor/1",
    )
)
EVIDENCE_PROFILES = (
    "syntax",
    "local-runtime",
    "k-execution",
    "theorem-discharge",
    "proven-build",
    "finalized-financial-settlement",
)
STOP_WORDS = frozenset(("only", "the", "and", "or", "a", "an", "of", "to"))
SECRET_PARTS = (
    "wallet",
    "seed",
    "credential",
    "credentials",
    "witness",
    "token",
    "tokens",
    ".env",
    "private-key",
    "private_key",
)
ACTIONFUL_KINDS = frozenset(
    ("implement", "reproduce", "repair", "verify", "review", "admin")
)
NEXT_KIND_ORDER = (
    "reproduce",
    "repair",
    "review",
    "verify",
    "implement",
    "admin",
    "report",
)
IMPLEMENT_KINDS = frozenset(("implement", "repair", "admin"))


class DuplicateKeyError(ValueError):
    """JSON object contained a repeated key."""


def load_snapshot(repo, action_id, history_reader=None):
    missing = []
    root = Path(repo)
    try:
        root = root.resolve()
    except OSError:
        missing.append("repository-unresolved:" + str(repo))
        return _snapshot(missing=missing)
    if not root.is_dir():
        missing.append("repository-unresolved:" + str(repo))
        return _snapshot(missing=missing)

    actions = _load_actions_catalog(root, missing)
    action = _find_action(actions, action_id, missing)
    program = _load_register(root, PROGRAM_REL, missing)
    sprints = _load_register(root, SPRINTS_REL, missing)
    program_ok = _validate_program(program, missing)
    sprints_ok = _validate_sprints(sprints, missing)
    campaign_rel = DEFAULT_CAMPAIGN_REL
    if program_ok:
        campaign_rel = program["reportReconciliation"]["stageAdmission"][
            "campaignRecordStore"
        ]
        if type(campaign_rel) is not str or campaign_rel == "":
            missing.append("campaignRecordStore-unresolved")
            campaign_rel = DEFAULT_CAMPAIGN_REL
            program_ok = False
    campaigns = None
    campaign_ok = False
    campaign_path = _contained_file(root, campaign_rel, missing, "campaign-store")
    if campaign_path is not None:
        campaigns = _load_json_path(campaign_path, missing, campaign_rel)
        campaign_ok = _validate_campaigns(campaigns, missing)

    snap = {
        "authorityCurrent": False,
        "entryEligible": False,
        "candidateCurrent": False,
        "resourceAdmitted": False,
        "sameDefectFailures": 0,
        "adminCycles": 0,
        "adminSeconds": 0,
        "primaryActive": False,
        "reproducerVerified": False,
        "approachChanged": False,
        "nextActionId": None,
        "missingEvidence": missing,
    }
    if action is not None:
        flags = _action_flags(
            root,
            action,
            program,
            sprints,
            campaigns,
            program_ok,
            sprints_ok,
            campaign_ok,
            missing,
        )
        snap.update(flags)
        _apply_history(history_reader, root, action, snap, missing)
        history_fields = {
            "sameDefectFailures": snap["sameDefectFailures"],
            "adminCycles": snap["adminCycles"],
            "adminSeconds": snap["adminSeconds"],
            "primaryActive": snap["primaryActive"],
            "reproducerVerified": snap["reproducerVerified"],
            "approachChanged": snap["approachChanged"],
        }
        snap["nextActionId"] = _select_next(
            root,
            actions,
            action,
            history_fields,
            history_reader,
            program,
            sprints,
            campaigns,
            program_ok,
            sprints_ok,
            campaign_ok,
            missing,
        )
    else:
        if history_reader is None:
            missing.append("operational-history")
    snap["missingEvidence"] = list(missing)
    return _snapshot(**snap)


def read_actions(repo):
    missing = []
    try:
        root = Path(repo).resolve()
    except OSError:
        raise ValueError("actions-unresolved:repository")
    actions = _load_actions_catalog(root, missing)
    if missing and not actions:
        raise ValueError("actions-unresolved:" + ",".join(missing))
    return actions


def _snapshot(**fields):
    missing = list(fields.get("missingEvidence", fields.get("missing", [])))
    values = {
        "authorityCurrent": fields.get("authorityCurrent", False),
        "entryEligible": fields.get("entryEligible", False),
        "candidateCurrent": fields.get("candidateCurrent", False),
        "resourceAdmitted": fields.get("resourceAdmitted", False),
        "sameDefectFailures": fields.get("sameDefectFailures", 0),
        "adminCycles": fields.get("adminCycles", 0),
        "adminSeconds": fields.get("adminSeconds", 0),
        "primaryActive": fields.get("primaryActive", False),
        "reproducerVerified": fields.get("reproducerVerified", False),
        "approachChanged": fields.get("approachChanged", False),
        "nextActionId": fields.get("nextActionId", None),
        "missingEvidence": missing,
    }
    return {key: values[key] for key in SNAPSHOT_KEYS}


def _object_pairs(pairs):
    obj = {}
    for key, value in pairs:
        if key in obj:
            raise DuplicateKeyError(key)
        obj[key] = value
    return obj


def _load_register(root, relative, missing):
    path = _contained_file(root, relative, missing, relative)
    if path is None:
        if not any(relative in item for item in missing):
            missing.append(relative)
        return None
    return _load_json_path(path, missing, relative)


def _load_json_path(path, missing, label):
    data = _read_bytes(path, missing, label)
    if data is None:
        return None
    return _parse_json(data, missing, label)


def _read_bytes(path, missing, label):
    try:
        with open(path, "rb") as handle:
            data = handle.read(MAX_BYTES + 1)
    except OSError:
        missing.append("read-failed:" + label)
        return None
    if len(data) > MAX_BYTES:
        missing.append("bounded-read-exceeded:" + label)
        return None
    return data


def _parse_json(data, missing, label):
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        missing.append("encoding:" + label)
        return None
    try:
        return json.loads(text, object_pairs_hook=_object_pairs)
    except DuplicateKeyError as exc:
        missing.append("duplicate-key:" + label + ":" + str(exc))
        return None
    except json.JSONDecodeError:
        missing.append("json:" + label)
        return None


def _load_actions_catalog(root, missing):
    path = _contained_file(root, ACTIONS_REL, missing, "actions")
    if path is None:
        return []
    payload = _load_json_path(path, missing, ACTIONS_REL)
    if type(payload) is not dict:
        if payload is not None:
            missing.append("actions-not-object")
        return []
    extra = [key for key in payload if key not in ("schema", "actions")]
    for key in extra:
        missing.append("unknown-actions-field:" + key)
    schema = payload.get("schema")
    if schema != ACTIONS_SCHEMA:
        missing.append("actions-schema-unresolved:" + str(schema))
        return []
    rows = payload.get("actions")
    if type(rows) is not list:
        missing.append("actions-not-list")
        return []
    actions = []
    seen = set()
    for index, row in enumerate(rows):
        parsed = _closed_action(row, missing, index)
        if parsed is None:
            continue
        if parsed["id"] in seen:
            missing.append("duplicate-action-id:" + parsed["id"])
            continue
        seen.add(parsed["id"])
        actions.append(parsed)
    return actions


def _closed_action(row, missing, index):
    if type(row) is not dict:
        missing.append("action-not-object:" + str(index))
        return None
    extra = [key for key in row if key not in ACTION_KEYS]
    absent = [key for key in ACTION_KEYS if key not in row]
    for key in extra:
        missing.append("unknown-action-field:" + key)
    for key in absent:
        missing.append("missing-action-field:" + key)
    if extra or absent:
        return None
    for key in ACTION_KEYS:
        if type(row[key]) is not str:
            missing.append("action-field-type:" + key)
            return None
    return {key: row[key] for key in ACTION_KEYS}


def _find_action(actions, action_id, missing):
    if type(action_id) is not str:
        missing.append("action-id-not-str")
        return None
    for row in actions:
        if row["id"] == action_id:
            return row
    missing.append("action-unresolved:" + action_id)
    return None


def _validate_program(program, missing):
    if program is None:
        return False
    if type(program) is not dict:
        missing.append("program-not-object")
        return False
    extra_program = [key for key in program if key not in PROGRAM_KEYS]
    for key in extra_program:
        missing.append("unknown-program-field:" + key)
    if extra_program:
        return False
    if "schemaVersion" not in program:
        missing.append("schemaVersion-missing")
        return False
    if type(program["schemaVersion"]) is not int:
        missing.append("schemaVersion-not-int")
        return False
    if program["schemaVersion"] != PROGRAM_VERSION:
        missing.append("schemaVersion-unsupported:" + str(program["schemaVersion"]))
        return False
    reconciliation = program.get("reportReconciliation")
    if type(reconciliation) is not dict:
        missing.append("reportReconciliation-unresolved")
        return False
    admission = reconciliation.get("stageAdmission")
    if type(admission) is not dict:
        missing.append("stageAdmission-unresolved")
        return False
    extra = [key for key in admission if key not in STAGE_ADMISSION_KEYS]
    for key in extra:
        missing.append("unknown-stageAdmission-field:" + key)
    if extra:
        return False
    if type(admission.get("dispatchEnabled")) is not bool:
        missing.append("dispatchEnabled-not-bool")
        return False
    if type(admission.get("campaignRecordStore")) is not str:
        missing.append("campaignRecordStore-not-str")
        return False
    if type(admission.get("rule")) is not str:
        missing.append("stageAdmission-unresolved")
        return False
    if admission.get("schema") != STAGE_ADMISSION_SCHEMA:
        missing.append(
            "stageAdmission-schema-unresolved:" + str(admission.get("schema"))
        )
        return False
    stages = admission.get("stages")
    if type(stages) is not list:
        missing.append("stages-not-list")
        return False
    valid = True
    seen = set()
    for stage in stages:
        if type(stage) is not dict:
            missing.append("stage-not-object")
            valid = False
            continue
        extra_stage = [key for key in stage if key not in STAGE_KEYS]
        for key in extra_stage:
            missing.append("unknown-nested-stage-field:" + key)
            valid = False
        absent = [key for key in STAGE_KEYS if key not in stage]
        for key in absent:
            missing.append("missing-stage-field:" + key)
            valid = False
        if extra_stage or absent:
            continue
        stage_id = stage.get("id")
        if type(stage_id) is not str:
            missing.append("stage-id-not-str")
            valid = False
            continue
        if stage_id in seen:
            missing.append("duplicate-stage:" + stage_id)
            valid = False
        seen.add(stage_id)
        if type(stage.get("status")) is not str:
            missing.append("stage-status-not-str:" + stage_id)
            valid = False
        owners = stage.get("owners")
        if type(owners) is not list or any(type(item) is not str for item in owners):
            missing.append("stage-owners-unresolved:" + stage_id)
            valid = False
        requires = stage.get("requires")
        if type(requires) is not list or any(type(item) is not str for item in requires):
            missing.append("stage-requires-unresolved:" + stage_id)
            valid = False
        if type(stage.get("purpose")) is not str:
            missing.append("stage-purpose-unresolved:" + stage_id)
            valid = False
        for field in ("acceptedProfile", "candidateHash", "campaignRecordId"):
            value = stage.get(field)
            if value is not None and type(value) is not str:
                missing.append("stage-" + field + "-type:" + stage_id)
                valid = False
    return valid


def _validate_sprints(sprints, missing):
    if sprints is None:
        return False
    if type(sprints) is not dict:
        missing.append("sprints-not-object")
        return False
    if type(sprints.get("schemaVersion")) is not int:
        missing.append("sprints-schemaVersion-not-int")
        return False
    if sprints.get("schemaVersion") != SPRINTS_VERSION:
        missing.append("sprints-schemaVersion-unsupported:" + str(sprints.get("schemaVersion")))
        return False
    if type(sprints.get("dispatchEnabled")) is not bool:
        missing.append("sprints-dispatchEnabled-not-bool")
        return False
    if type(sprints.get("resourceAllocationGranted")) is not bool:
        missing.append("sprints-resourceAllocationGranted-not-bool")
        return False
    rows = sprints.get("sprints")
    if type(rows) is not list:
        missing.append("sprints-list-unresolved")
        return False
    valid = True
    for sprint in rows:
        if type(sprint) is not dict:
            missing.append("sprint-not-object")
            valid = False
            continue
        gates = sprint.get("entryGates", [])
        if type(gates) is not list:
            missing.append("entryGates-not-list:" + str(sprint.get("id")))
            valid = False
            continue
        for gate in gates:
            if type(gate) is not dict:
                missing.append("entryGate-not-object")
                valid = False
                continue
            extra = [key for key in gate if key not in GATE_KEYS]
            for key in extra:
                missing.append("unknown-entryGate-field:" + key)
                valid = False
            if "requires" not in gate:
                missing.append("entryGate-requires-unresolved")
                valid = False
            if "campaignOwner" not in gate:
                missing.append("entryGate-campaignOwner-unresolved")
                valid = False
            if "stage" not in gate or type(gate.get("stage")) is not str:
                missing.append("entryGate-stage-unresolved")
                valid = False
            tasks = gate.get("tasks", [])
            requires = gate.get("requires")
            owners = gate.get("campaignOwner")
            if type(tasks) is not list or any(type(item) is not str for item in tasks):
                missing.append("entryGate-tasks-unresolved")
                valid = False
            if requires is not None and (
                type(requires) is not list
                or any(type(item) is not str for item in requires)
            ):
                missing.append("entryGate-requires-unresolved")
                valid = False
            if owners is not None and (
                type(owners) is not list
                or any(type(item) is not str for item in owners)
            ):
                missing.append("entryGate-campaignOwner-unresolved")
                valid = False
    return valid


def _validate_campaigns(campaigns, missing):
    if campaigns is None:
        return False
    if type(campaigns) is not dict:
        missing.append("campaign-store-not-object")
        return False
    extra = [key for key in campaigns if key not in ("schema", "campaigns")]
    for key in extra:
        missing.append("unknown-campaign-store-field:" + key)
    schema = campaigns.get("schema")
    if schema != CAMPAIGN_SCHEMA:
        missing.append("campaign-schema-unresolved:" + str(schema))
        return False
    table = campaigns.get("campaigns")
    if type(table) is not dict:
        missing.append("campaigns-not-object")
        return False
    valid = extra == []
    for campaign_id, record in table.items():
        if type(campaign_id) is not str or campaign_id == "":
            missing.append("campaign-id-unresolved")
            valid = False
            continue
        if type(record) is not dict:
            missing.append("campaign-not-object:" + campaign_id)
            valid = False
            continue
        extra_record = [key for key in record if key not in CAMPAIGN_RECORD_KEYS]
        for key in extra_record:
            missing.append("unknown-campaign-field:" + key)
            valid = False
    return valid


def _program_stages(program):
    stages = {}
    rows = program["reportReconciliation"]["stageAdmission"]["stages"]
    for stage in rows:
        if type(stage) is dict and type(stage.get("id")) is str:
            stages[stage["id"]] = stage
    return stages


def _find_gate(sprints, requirement):
    for sprint in sprints.get("sprints", []):
        if type(sprint) is not dict:
            continue
        for gate in sprint.get("entryGates", []):
            if type(gate) is not dict:
                continue
            tasks = gate.get("tasks", [])
            if type(tasks) is list and requirement in tasks:
                return gate
    return None


def _entry_eligible(sprints, program, campaigns, root, requirement, missing):
    gate = _find_gate(sprints, requirement)
    if gate is None:
        missing.append("entryGate-unresolved:" + requirement)
        return False
    if "requires" not in gate:
        missing.append("entryGate-requires-unresolved")
        return False
    requires = gate.get("requires")
    if type(requires) is not list or any(type(item) is not str for item in requires):
        missing.append("entryGate-requires-unresolved")
        return False
    stages = _program_stages(program)
    visiting = set()
    done = set()
    for required in requires:
        if not _stage_complete(
            required, stages, campaigns, root, missing, visiting, done
        ):
            return False
    return True


def _stage_complete(stage_id, stages, campaigns, root, missing, visiting, done):
    if stage_id in done:
        return True
    if stage_id in visiting:
        missing.append("stage-cycle:" + stage_id)
        return False
    stage = stages.get(stage_id)
    if stage is None:
        missing.append("required-stage-missing:" + stage_id)
        return False
    if stage.get("status") != "complete":
        missing.append(
            "required-stage-incomplete:" + stage_id + ":" + str(stage.get("status"))
        )
        return False
    for field in ("acceptedProfile", "candidateHash", "campaignRecordId"):
        value = stage.get(field)
        if type(value) is not str or value == "":
            missing.append("required-stage-evidence-missing:" + stage_id + ":" + field)
            return False
    if not _prerequisite_campaign(stage, campaigns, root, missing):
        return False
    requires = stage.get("requires")
    if type(requires) is not list:
        missing.append("stage-requires-unresolved:" + stage_id)
        return False
    visiting.add(stage_id)
    for req in requires:
        if not _stage_complete(
            req, stages, campaigns, root, missing, visiting, done
        ):
            visiting.discard(stage_id)
            return False
    visiting.discard(stage_id)
    done.add(stage_id)
    return True


def _complete_status(status):
    return (
        type(status) is str
        and (status == "complete" or status.startswith("complete-"))
        and "revoked" not in status
    )


def _prerequisite_campaign(stage, campaigns, root, missing):
    stage_id = stage["id"]
    campaign_id = stage.get("campaignRecordId")
    if type(campaign_id) is not str or campaign_id == "":
        missing.append("required-stage-evidence-missing:" + stage_id + ":campaignRecordId")
        return False
    if campaigns is None or type(campaigns.get("campaigns")) is not dict:
        missing.append("required-stage-campaign-unresolved:" + campaign_id)
        return False
    record = campaigns["campaigns"].get(campaign_id)
    if type(record) is not dict:
        missing.append("required-stage-campaign-unresolved:" + campaign_id)
        return False
    rec_hash = record.get("candidateHash")
    if rec_hash is None:
        rec_hash = record.get("candidateSha256")
    stage_hash = stage.get("candidateHash")
    if type(rec_hash) is not str or rec_hash == "":
        missing.append("required-stage-candidate-unresolved:" + stage_id)
        return False
    if type(stage_hash) is not str or stage_hash == "":
        missing.append("required-stage-evidence-missing:" + stage_id + ":candidateHash")
        return False
    if rec_hash.lower() != stage_hash.lower():
        missing.append("required-stage-candidate-conflict:" + stage_id)
        return False
    rec_profile = record.get("acceptedProfile")
    stage_profile = stage.get("acceptedProfile")
    if type(rec_profile) is not str or rec_profile == "":
        missing.append("required-stage-profile-unresolved:" + stage_id)
        return False
    if type(stage_profile) is not str or stage_profile == "" or rec_profile != stage_profile:
        missing.append("required-stage-profile-conflict:" + stage_id)
        return False
    rec_stage = record.get("stage")
    if rec_stage != stage_id:
        missing.append("required-stage-mismatch:" + stage_id)
        return False
    rec_status = record.get("status")
    if rec_status in ("blocked", "complete-but-revoked") or not _complete_status(rec_status):
        missing.append("required-stage-campaign-status:" + stage_id + ":" + str(rec_status))
        return False
    rec_owners = _owners_of(record.get("owner"))
    stage_owners = stage.get("owners")
    if type(stage_owners) is not list or not rec_owners.intersection(set(stage_owners)):
        missing.append("required-stage-owner-mismatch:" + stage_id)
        return False
    binding_ok, _binding = _verify_binding(root, campaign_id, record, missing)
    if not binding_ok:
        return False
    return _verify_candidate(root, campaign_id, record, missing)


def _resolve_command(root, action, missing):
    ref = action["commandRef"]
    if ref == "":
        if action["kind"] in ACTIONFUL_KINDS:
            missing.append("commandRef-unresolved:" + action["id"])
            return False
        return True
    if ref.startswith("/") or ref.startswith("\\") or _has_drive(ref):
        missing.append("commandRef-absolute:" + ref)
        return False
    if "#" not in ref:
        missing.append("commandRef-unresolved:" + ref)
        return False
    file_part, fragment = ref.split("#", 1)
    if file_part == "" or fragment == "":
        missing.append("commandRef-unresolved:" + ref)
        return False
    path = _contained_file(root, file_part, missing, "commandRef")
    if path is None:
        missing.append("commandRef-unresolved:" + ref)
        return False
    payload = _load_json_path(path, missing, file_part)
    if type(payload) is not dict:
        missing.append("commandRef-unresolved:" + ref)
        return False
    extra = [key for key in payload if key not in ("schema", "commands")]
    for key in extra:
        missing.append("unknown-command-field:" + key)
    if payload.get("schema") != COMMANDS_SCHEMA:
        missing.append("commandRef-unsupported-schema:" + str(payload.get("schema")))
        missing.append("commandRef-unresolved:" + ref)
        return False
    commands = payload.get("commands")
    if type(commands) is not dict or fragment not in commands:
        missing.append("commandRef-unresolved:" + ref)
        return False
    spec = commands[fragment]
    if type(spec) is not dict:
        missing.append("commandRef-unresolved:" + ref)
        return False
    extra_spec = [key for key in spec if key != "argv"]
    for key in extra_spec:
        missing.append("unknown-command-field:" + key)
    argv = spec.get("argv")
    if type(argv) is not list or argv == [] or any(type(item) is not str for item in argv):
        missing.append("commandRef-unresolved:" + ref)
        return False
    return extra == [] and extra_spec == []


def _action_flags(
    root,
    action,
    program,
    sprints,
    campaigns,
    program_ok,
    sprints_ok,
    campaign_ok,
    missing,
):
    entry_eligible = False
    if program_ok and sprints_ok:
        entry_eligible = _entry_eligible(
            sprints,
            program,
            campaigns if campaign_ok else None,
            root,
            action["requirement"],
            missing,
        )
    command_ok = _resolve_command(root, action, missing)
    admitted, candidate_current, resource_admitted = _resolve_admission(
        root,
        action,
        program if program_ok else None,
        sprints if sprints_ok else None,
        campaigns if campaign_ok else None,
        missing,
    )
    authority = bool(
        program_ok
        and sprints_ok
        and campaign_ok
        and command_ok
        and admitted
        and not _structural_block(missing)
    )
    return {
        "authorityCurrent": authority,
        "entryEligible": entry_eligible,
        "candidateCurrent": candidate_current,
        "resourceAdmitted": resource_admitted,
    }


def _resolve_admission(root, action, program, sprints, campaigns, missing):
    ref = action["admissionRef"]
    if ref.startswith("campaign:"):
        return _resolve_campaign_admission(
            root, action, program, sprints, campaigns, missing
        )
    return _resolve_file_admission(root, ref, missing)


def _resolve_campaign_admission(root, action, program, sprints, campaigns, missing):
    ref = action["admissionRef"]
    campaign_id = ref[len("campaign:") :]
    if campaign_id == "":
        missing.append("admissionRef-unresolved:campaign:")
        return False, False, False
    if campaigns is None or program is None or sprints is None:
        missing.append("admissionRef-unresolved:" + ref)
        return False, False, False
    table = campaigns.get("campaigns")
    if type(table) is not dict or campaign_id not in table:
        missing.append("admissionRef-unresolved:" + ref)
        return False, False, False
    record = table[campaign_id]
    if type(record) is not dict:
        missing.append("admissionRef-unresolved:" + ref)
        return False, False, False
    gate = _find_gate(sprints, action["requirement"])
    stages = _program_stages(program)
    matched = _campaign_matches_gate(
        record, gate, action, stages, campaign_id, missing
    )
    binding_ok, binding_obj = _verify_binding(root, campaign_id, record, missing)
    candidate_ok = _verify_candidate(root, campaign_id, record, missing)
    resource_ok = _verify_resource(
        root, campaign_id, record, program, binding_obj, action, missing
    )
    return matched and binding_ok, candidate_ok, resource_ok


def _campaign_matches_gate(record, gate, action, stages, campaign_id, missing):
    if gate is None:
        missing.append("entryGate-unresolved:" + action["requirement"])
        return False
    gate_stage = gate.get("stage")
    if type(gate_stage) is not str or gate_stage == "":
        missing.append("entryGate-stage-unresolved:" + action["requirement"])
        return False
    rec_stage = record.get("stage")
    if type(rec_stage) is not str or rec_stage != gate_stage:
        missing.append("campaign-stage-mismatch:" + campaign_id)
        return False
    owners = gate.get("campaignOwner")
    if type(owners) is not list or any(type(item) is not str for item in owners):
        missing.append("entryGate-campaignOwner-unresolved:" + action["requirement"])
        return False
    rec_owners = _owners_of(record.get("owner"))
    if not rec_owners.intersection(set(owners)):
        missing.append("campaign-owner-mismatch:" + campaign_id)
        return False
    stage = stages.get(gate_stage)
    if stage is None:
        missing.append("action-stage-missing:" + gate_stage)
        return False
    if stage.get("status") == "blocked":
        missing.append("action-stage-blocked:" + gate_stage)
        return False
    record_id = stage.get("campaignRecordId")
    if type(record_id) is not str or record_id == "":
        missing.append("stage-campaignRecordId-null:" + gate_stage)
        return False
    if record_id != campaign_id:
        missing.append("stage-campaignRecordId-mismatch:" + gate_stage)
        return False
    rec_profile = record.get("acceptedProfile")
    stage_profile = stage.get("acceptedProfile")
    status = record.get("status")
    complete = type(status) is str and (
        status == "complete" or status.startswith("complete-")
    )
    if complete:
        if stage.get("status") != "complete":
            missing.append("action-stage-incomplete:" + gate_stage)
            return False
        if type(rec_profile) is not str or rec_profile == "":
            missing.append("acceptedProfile-unresolved:" + campaign_id)
            return False
        if type(stage_profile) is not str or stage_profile == "":
            missing.append("stage-acceptedProfile-unresolved:" + gate_stage)
            return False
        if rec_profile != stage_profile:
            missing.append("acceptedProfile-conflict:" + campaign_id)
            return False
    elif (
        type(rec_profile) is str
        and type(stage_profile) is str
        and rec_profile != stage_profile
    ):
        missing.append("acceptedProfile-conflict:" + campaign_id)
        return False
    if action["evidenceProfile"] not in EVIDENCE_PROFILES:
        missing.append("evidenceProfile-unsupported:" + action["evidenceProfile"])
        return False
    if not _evidence_applicable(action["evidenceProfile"], status):
        missing.append("evidenceProfile-inapplicable:" + action["evidenceProfile"])
        return False
    rec_hash = record.get("candidateHash")
    if rec_hash is None:
        rec_hash = record.get("candidateSha256")
    stage_hash = stage.get("candidateHash")
    action_hash = action["candidate"]
    if complete:
        if type(rec_hash) is not str or rec_hash == "":
            missing.append("candidateHash-unresolved:" + campaign_id)
            return False
        if type(stage_hash) is not str or stage_hash == "":
            missing.append("stage-candidateHash-null:" + gate_stage)
            return False
    if type(rec_hash) is str and action_hash.lower() != rec_hash.lower():
        missing.append("action-candidate-conflict:" + campaign_id)
        return False
    if type(stage_hash) is str and action_hash.lower() != stage_hash.lower():
        missing.append("stage-candidate-conflict:" + gate_stage)
        return False
    texts = [
        str(stage.get("purpose") or ""),
        str(record.get("scope") or ""),
        str(action["requirement"]),
    ]
    if not _capability_agrees(action["capability"], texts):
        missing.append("action-capability-unresolved:" + action["capability"])
        return False
    if not _status_allows(status, action["kind"]):
        missing.append("campaign-status-unsupported:" + str(status))
        return False
    return True


def _evidence_applicable(profile, status):
    if profile not in EVIDENCE_PROFILES:
        return False
    rank = EVIDENCE_PROFILES.index(profile)
    if type(status) is not str or status == "":
        return False
    if status == "complete" or status.startswith("complete-"):
        return rank <= EVIDENCE_PROFILES.index("syntax")
    if status.startswith("admitted-"):
        return rank <= EVIDENCE_PROFILES.index("local-runtime")
    return False


def _capability_agrees(capability, texts):
    if type(capability) is not str or capability == "":
        return False
    blob = " ".join(texts).lower().replace("/", " ").replace("_", "-")
    for stem in capability.lower().replace("_", "-").split("-"):
        if stem == "" or stem in STOP_WORDS:
            continue
        if stem not in blob:
            return False
    return True


def _owners_of(value):
    if type(value) is not str:
        return set()
    parts = set()
    for chunk in value.replace(",", "/").split("/"):
        item = chunk.strip()
        if item:
            parts.add(item)
    return parts


def _status_allows(status, kind):
    if type(status) is not str or status == "":
        return False
    if status == "complete-but-revoked":
        return False
    if kind == "report":
        return True
    if kind in ("review", "reproduce", "verify"):
        return status == "complete" or status.startswith("admitted-")
    if kind in IMPLEMENT_KINDS:
        return status.startswith("admitted-")
    return False


def _verify_binding(root, campaign_id, record, missing):
    binding_rel = record.get("binding")
    expected = record.get("bindingSha256")
    if binding_rel is None and expected is None:
        missing.append("binding-unresolved:" + campaign_id)
        return False, None
    if type(binding_rel) is not str or type(expected) is not str:
        missing.append("binding-unresolved:" + campaign_id)
        return False, None
    path = _contained_file(root, binding_rel, missing, "binding")
    if path is None:
        missing.append("binding-missing:" + campaign_id)
        return False, None
    data = _read_bytes(path, missing, binding_rel)
    if data is None:
        return False, None
    digest = hashlib.sha256(data).hexdigest()
    if digest != expected.lower():
        missing.append("bindingSha256-stale:" + campaign_id)
        return False, None
    payload = _parse_json(data, missing, binding_rel)
    if type(payload) is not dict:
        missing.append("binding-not-object:" + campaign_id)
        return False, None
    if payload == {}:
        missing.append("binding-schema-unresolved:empty")
        return False, None
    schema = payload.get("schema")
    if type(schema) is not str:
        missing.append("binding-schema-unresolved:" + type(schema).__name__)
        return False, None
    if schema != BINDING_SCHEMA:
        missing.append("binding-schema-unresolved:" + schema)
        return False, None
    for key in ("status", "authority", "scope", "resources"):
        if key not in payload:
            missing.append("binding-field-missing:" + key)
            return False, None
        if key != "resources" and type(payload[key]) is not str:
            missing.append("binding-field-type:" + key)
            return False, None
    if _contained_file(root, payload["authority"], missing, "binding-authority") is None:
        return False, None
    resources = payload["resources"]
    if type(resources) is not dict:
        missing.append("binding-resources-unresolved:" + campaign_id)
        return False, None
    seconds = resources.get("allocationSeconds")
    if seconds is not None and type(seconds) is not int:
        missing.append("binding-allocation-invalid:" + campaign_id)
        return False, None
    if not _verify_path_map(root, payload.get("inputs"), missing, "binding-input"):
        return False, None
    owned = payload.get("ownedFiles")
    if owned is None:
        pass
    elif type(owned) is list:
        for item in owned:
            if type(item) is not str:
                missing.append("binding-ownedFiles-unresolved:" + campaign_id)
                return False, None
            if _contained_file(root, item, missing, "binding-owned") is None:
                return False, None
    elif type(owned) is dict:
        if not _verify_path_map(root, owned, missing, "binding-owned"):
            return False, None
    else:
        missing.append("binding-ownedFiles-unresolved:" + campaign_id)
        return False, None
    return True, payload


def _verify_path_map(root, mapping, missing, kind):
    if mapping is None:
        return True
    if type(mapping) is not dict:
        missing.append(kind + "-unresolved")
        return False
    for rel, digest in mapping.items():
        if type(rel) is not str or type(digest) is not str:
            missing.append(kind + "-unresolved")
            return False
        path = _contained_file(root, rel, missing, kind)
        if path is None:
            return False
        data = _read_bytes(path, missing, rel)
        if data is None:
            return False
        if hashlib.sha256(data).hexdigest() != digest.lower():
            missing.append(kind + "-stale:" + rel)
            return False
    return True


def _verify_candidate(root, campaign_id, record, missing):
    manifest_rel = record.get("candidateManifest")
    if type(manifest_rel) is not str or manifest_rel == "":
        missing.append("candidateManifest-unresolved:" + campaign_id)
        return False
    man_path = _contained_file(root, manifest_rel, missing, "candidateManifest")
    if man_path is None:
        return False
    manifest = _load_json_path(man_path, missing, manifest_rel)
    if type(manifest) is not dict:
        missing.append("candidateManifest-not-object:" + campaign_id)
        return False
    extra = [key for key in manifest if key not in MANIFEST_KEYS]
    for key in extra:
        missing.append("unknown-manifest-field:" + key)
        return False
    if "schema" in manifest:
        missing.append("candidateManifest-schema-unresolved:" + str(manifest.get("schema")))
        return False
    owned = manifest.get("ownedFiles")
    if type(owned) is not dict or not owned:
        missing.append("candidate-ownedFiles-unresolved:" + campaign_id)
        return False
    for rel, digest in owned.items():
        if type(rel) is not str or type(digest) is not str:
            missing.append("candidate-ownedFiles-unresolved:" + campaign_id)
            return False
    algo = manifest.get("digestAlgorithm")
    candidate_sha = manifest.get("candidateSha256")
    if type(candidate_sha) is not str or candidate_sha == "":
        missing.append("candidateSha256-unresolved:" + campaign_id)
        return False
    if type(algo) is not str or "SHA256" not in algo:
        missing.append("digestAlgorithm-unsupported:" + campaign_id)
        return False
    compact = json.dumps(owned, separators=(",", ":"), sort_keys=True)
    if hashlib.sha256(compact.encode("utf-8")).hexdigest() != candidate_sha.lower():
        missing.append("candidateSha256-stale:" + campaign_id)
        return False
    record_hash = record.get("candidateHash")
    if record_hash is not None and type(record_hash) is not str:
        missing.append("candidateHash-type:" + campaign_id)
        return False
    if type(record_hash) is str and record_hash.lower() != candidate_sha.lower():
        missing.append("candidateHash-conflict:" + campaign_id)
        return False
    inputs = manifest.get("inputs")
    if type(inputs) is not dict:
        missing.append("candidate-inputs-unresolved:" + campaign_id)
        return False
    review_rel = record.get("review")
    if type(review_rel) is str and review_rel != "":
        if not _apply_review(root, review_rel, record, candidate_sha, campaign_id, missing):
            return False
    elif _complete_status(record.get("status")):
        missing.append("review-unresolved:" + campaign_id)
        return False
    published_skip = None
    acc_rel = record.get("acceptance")
    if type(acc_rel) is str and acc_rel != "":
        published_skip = _apply_acceptance(
            root, acc_rel, man_path, candidate_sha, owned, campaign_id, missing
        )
        if published_skip is False:
            return False
    elif _complete_status(record.get("status")):
        missing.append("acceptance-unresolved:" + campaign_id)
        return False
    profile = record.get("acceptedProfile")
    if profile is not None and type(profile) is not str:
        missing.append("acceptedProfile-type:" + campaign_id)
        return False
    freeze_bytes = {
        rel: digest
        for rel, digest in owned.items()
        if rel != published_skip
    }
    if not _verify_path_map(root, freeze_bytes, missing, "candidate-source"):
        return False
    return _verify_path_map(root, inputs, missing, "candidate-input")


def _freeze_archive_rel(manifest_rel, published_rel):
    posix = manifest_rel.replace("\\", "/")
    published = published_rel.replace("\\", "/")
    if not posix.endswith("-freeze.json"):
        return None
    parts = [part for part in posix.split("/") if part not in ("", ".")]
    if not parts:
        return None
    stem = parts[-1][: -len("-freeze.json")]
    if stem == "":
        return None
    archived = parts[:-1] + [stem] + [part for part in published.split("/") if part not in ("", ".")]
    if any(part == ".." for part in archived):
        return None
    return "/".join(archived)


def _publication_core(payload):
    core = json.loads(json.dumps(payload))
    subsets = core.get("subsets")
    if type(subsets) is not dict:
        return core
    subset = subsets.get(PUBLICATION_SUBSET)
    if type(subset) is not dict:
        return core
    for key in PUBLICATION_MUTABLE:
        subset.pop(key, None)
    outputs = subset.get("outputs")
    if type(outputs) is list:
        for item in outputs:
            if type(item) is dict:
                item.pop("sha256", None)
                item.pop("status", None)
    summaries = subset.get("rowSummaries")
    if type(summaries) is dict:
        for row in summaries.values():
            if type(row) is dict:
                row.pop("closureTask", None)
    return core


def _publication_transform_ok(
    root,
    original_bytes,
    current_bytes,
    owned,
    candidate_sha,
    acceptance,
    published_path,
    missing,
):
    original = _parse_json(original_bytes, missing, "publication-original")
    current = _parse_json(current_bytes, missing, published_path)
    if type(original) is not dict or type(current) is not dict:
        missing.append("publication-control-substitution:" + published_path)
        return False
    if original.get("schemaVersion") != PUBLICATION_SCHEMA:
        missing.append("publication-control-substitution:" + published_path)
        return False
    if current.get("schemaVersion") != original.get("schemaVersion"):
        missing.append("publication-control-substitution:" + published_path)
        return False
    if _publication_core(original) != _publication_core(current):
        missing.append("publication-control-substitution:" + published_path)
        return False
    subsets = current.get("subsets")
    subset = subsets.get(PUBLICATION_SUBSET) if type(subsets) is dict else None
    if type(subset) is not dict:
        missing.append("publication-control-substitution:" + published_path)
        return False
    if subset.get("status") != "complete":
        missing.append("publication-control-status:" + published_path)
        return False
    digest = subset.get("candidateHash")
    if type(digest) is not str or digest.lower() != candidate_sha.lower():
        missing.append("publication-control-candidateHash:" + published_path)
        return False
    bound = subset.get("candidateHashStatus")
    if type(bound) is not str or "bound" not in bound:
        missing.append("publication-control-candidateHashStatus:" + published_path)
        return False
    pub_acc = subset.get("acceptance")
    if type(pub_acc) is not dict:
        missing.append("publication-control-acceptance:" + published_path)
        return False
    if pub_acc.get("candidateManifest") != acceptance.get("sourceCandidate"):
        missing.append("publication-control-acceptance:" + published_path)
        return False
    if pub_acc.get("review") != acceptance.get("review"):
        missing.append("publication-control-acceptance:" + published_path)
        return False
    reviews = subset.get("reviews")
    if type(reviews) is not list or reviews == []:
        missing.append("publication-control-reviews:" + published_path)
        return False
    matched = False
    for item in reviews:
        if type(item) is not dict:
            missing.append("publication-control-reviews:" + published_path)
            return False
        path = item.get("path")
        digest = item.get("sha256")
        verdict = item.get("verdict")
        if type(path) is not str or type(digest) is not str:
            missing.append("publication-control-reviews:" + published_path)
            return False
        if type(verdict) is not str or verdict.upper() != REVIEW_VERDICT_PASS:
            missing.append("publication-control-reviews:" + published_path)
            return False
        file_path = _contained_file(root, path, missing, "publication-review")
        if file_path is None:
            return False
        body = _read_bytes(file_path, missing, path)
        if body is None:
            return False
        if hashlib.sha256(body).hexdigest() != digest.lower():
            missing.append("publication-control-reviews:" + published_path)
            return False
        if path == acceptance.get("review"):
            matched = True
    if not matched:
        missing.append("publication-control-reviews:" + published_path)
        return False
    outputs = subset.get("outputs")
    if type(outputs) is not list:
        missing.append("publication-control-outputs:" + published_path)
        return False
    for item in outputs:
        if type(item) is not dict:
            missing.append("publication-control-outputs:" + published_path)
            return False
        path = item.get("path")
        digest = item.get("sha256")
        if type(path) is not str or type(digest) is not str:
            missing.append("publication-control-outputs:" + published_path)
            return False
        frozen = owned.get(path)
        if type(frozen) is not str or frozen.lower() != digest.lower():
            missing.append("publication-control-outputs:" + published_path)
            return False
    summaries = subset.get("rowSummaries")
    if type(summaries) is dict:
        for row in summaries.values():
            if type(row) is dict and row.get("closureTask") is not None:
                if type(row.get("closureTask")) is not str:
                    missing.append("publication-control-closure:" + published_path)
                    return False
    return True


def _apply_acceptance(root, acc_rel, man_path, candidate_sha, owned, campaign_id, missing):
    acc_path = _contained_file(root, acc_rel, missing, "acceptance")
    if acc_path is None:
        return False
    acceptance = _load_json_path(acc_path, missing, acc_rel)
    if type(acceptance) is not dict:
        missing.append("acceptance-not-object:" + campaign_id)
        return False
    extra = [key for key in acceptance if key not in ACCEPTANCE_KEYS]
    for key in extra:
        missing.append("unknown-acceptance-field:" + key)
        return False
    for field in ACCEPTANCE_REQUIRED:
        value = acceptance.get(field)
        if type(value) is not str or value == "":
            missing.append("acceptance-field-unresolved:" + field)
            return False
    status = acceptance.get("status")
    if not _complete_status(status):
        missing.append("acceptance-status-unsupported:" + str(status))
        return False
    acc_hash = acceptance.get("candidateHash")
    if acc_hash.lower() != candidate_sha.lower():
        missing.append("acceptance-candidateHash-conflict:" + campaign_id)
        return False
    source = acceptance.get("sourceCandidate")
    src_path = _contained_file(root, source, missing, "sourceCandidate")
    if src_path is None or src_path != man_path:
        missing.append("acceptance-sourceCandidate-conflict:" + campaign_id)
        return False
    unchanged = acceptance.get("unchangedPayloadHashes")
    if unchanged is None:
        unchanged = {}
    if type(unchanged) is not dict:
        missing.append("acceptance-unchanged-unresolved:" + campaign_id)
        return False
    for rel, digest in unchanged.items():
        if type(rel) is not str or type(digest) is not str:
            missing.append("acceptance-unchanged-unresolved:" + campaign_id)
            return False
        frozen = owned.get(rel)
        if type(frozen) is not str or frozen.lower() != digest.lower():
            missing.append("acceptance-unchanged-conflict:" + rel)
            return False
    published_path = acceptance.get("publishedControlPath")
    published_sha = acceptance.get("publishedControlSha256")
    frozen_digest = owned.get(published_path)
    if type(frozen_digest) is not str or frozen_digest == "":
        missing.append("publication-bridge-unavailable:" + campaign_id)
        return False
    pub_path = _contained_file(root, published_path, missing, "published-control")
    if pub_path is None:
        missing.append("publication-bridge-unavailable:" + campaign_id)
        return False
    data = _read_bytes(pub_path, missing, published_path)
    if data is None:
        return False
    current_digest = hashlib.sha256(data).hexdigest()
    if current_digest != published_sha.lower():
        missing.append("publication-bridge-stale:" + published_path)
        return False
    if current_digest == frozen_digest.lower():
        return None
    archive_rel = _freeze_archive_rel(source, published_path)
    if archive_rel is None:
        missing.append("publication-original-unavailable:" + published_path)
        return False
    archive_path = _contained_file(root, archive_rel, missing, "publication-original")
    if archive_path is None:
        missing.append("publication-original-unavailable:" + published_path)
        return False
    original = _read_bytes(archive_path, missing, archive_rel)
    if original is None:
        return False
    if hashlib.sha256(original).hexdigest() != frozen_digest.lower():
        missing.append("publication-original-stale:" + archive_rel)
        return False
    if not _publication_transform_ok(
        root,
        original,
        data,
        owned,
        candidate_sha,
        acceptance,
        published_path,
        missing,
    ):
        return False
    return published_path


def _apply_review(root, review_rel, record, candidate_sha, campaign_id, missing):
    rev_path = _contained_file(root, review_rel, missing, "review")
    if rev_path is None:
        return False
    review = _load_json_path(rev_path, missing, review_rel)
    if type(review) is not dict:
        missing.append("review-not-object:" + campaign_id)
        return False
    extra = [key for key in review if key not in REVIEW_KEYS]
    for key in extra:
        missing.append("unknown-review-field:" + key)
        return False
    for field in REVIEW_REQUIRED:
        value = review.get(field)
        if type(value) is not str or value == "":
            missing.append("review-field-unresolved:" + field)
            return False
    findings = review.get("findings")
    if type(findings) is not list:
        missing.append("review-findings-unresolved:" + campaign_id)
        return False
    rev_hash = review.get("candidateSha256")
    if rev_hash.lower() != candidate_sha.lower():
        missing.append("review-candidateSha256-conflict:" + campaign_id)
        return False
    rev_scope = review.get("scope")
    camp_scope = record.get("scope")
    if type(camp_scope) is str and rev_scope != camp_scope:
        missing.append("review-scope-conflict:" + campaign_id)
        return False
    verdict = review.get("verdict")
    upper = verdict.upper()
    if upper in REVIEW_VERDICT_FAIL:
        missing.append("review-verdict-blocked:" + campaign_id)
        return False
    if upper != REVIEW_VERDICT_PASS:
        missing.append("review-verdict-unresolved:" + campaign_id)
        return False
    reviewer = review.get("reviewer")
    lowered = reviewer.lower()
    if lowered.startswith("author") or "author-" in lowered:
        missing.append("review-reviewer-not-independent:" + reviewer)
        return False
    return True


def _verify_resource(root, campaign_id, record, program, binding_obj, action, missing):
    relative = record.get("resourceAmendment")
    if type(relative) is not str or relative == "":
        missing.append("resourceAmendment-unresolved:" + campaign_id)
        return False
    path = _contained_file(root, relative, missing, "resource")
    if path is None:
        missing.append("resourceAmendment-missing:" + campaign_id)
        return False
    payload = _load_json_path(path, missing, relative)
    if type(payload) is not dict:
        missing.append("resourceAmendment-not-object:" + campaign_id)
        return False
    if payload == {}:
        missing.append("resource-empty:" + campaign_id)
        return False
    schema = payload.get("schema")
    if schema is not None and schema not in RESOURCE_SCHEMAS:
        missing.append("resource-schema-unresolved:" + str(schema))
        return False
    authority = payload.get("authority")
    if type(authority) is not str or authority == "":
        missing.append("resource-authority-unresolved:" + campaign_id)
        return False
    if _contained_file(root, authority, missing, "resource-authority") is None:
        return False
    alloc = _amendment_seconds(payload, campaign_id, missing)
    if alloc is None:
        return False
    res_scope = payload.get("scope")
    camp_scope = record.get("scope")
    if res_scope is not None:
        if type(res_scope) is not str or res_scope == "":
            missing.append("resource-scope-unresolved:" + campaign_id)
            return False
        if type(camp_scope) is str and res_scope != camp_scope:
            missing.append("resource-scope-mismatch:" + campaign_id)
            return False
    successors = record.get("successorAmendments")
    if successors is None:
        successors = []
    if type(successors) is not list:
        missing.append("successorAmendments-unresolved:" + campaign_id)
        return False
    for item in successors:
        if type(item) is not str:
            missing.append("successorAmendments-unresolved:" + campaign_id)
            return False
        extra = _load_amendment_file(root, item, campaign_id, missing)
        if extra is None:
            return False
        alloc += extra
    if type(binding_obj) is dict:
        resources = binding_obj.get("resources")
        if type(resources) is dict:
            bound = resources.get("allocationSeconds")
            if bound is not None:
                if type(bound) is not int:
                    missing.append("binding-allocation-invalid:" + campaign_id)
                    return False
                alloc += bound
    if alloc <= 0:
        missing.append("resource-allocation-invalid:" + campaign_id)
        return False
    historical = None
    if type(program) is dict:
        runtime = program.get("resourceRuntimeSnapshot")
        if type(runtime) is dict:
            historical = runtime.get("path")
    spec = record.get("currentAccounting")
    if spec is None:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return False
    remaining = _remaining_from_current(
        root, spec, record, binding_obj, action, alloc, historical, campaign_id, missing
    )
    if remaining is None:
        return False
    if remaining <= 0:
        missing.append("resource-exhausted:" + campaign_id)
        return False
    return True


def _amendment_seconds(payload, campaign_id, missing):
    if "allocationSeconds" in payload:
        value = payload["allocationSeconds"]
        if type(value) is not int or value <= 0:
            missing.append("resource-allocation-invalid:" + campaign_id)
            return None
        additional = payload.get("additionalSeconds")
        if additional is None:
            return value
        if type(additional) is not int or additional < 0:
            missing.append("resource-allocation-invalid:" + campaign_id)
            return None
        return value + additional
    additional = payload.get("additionalSeconds")
    if type(additional) is not int or additional <= 0:
        missing.append("resource-allocation-invalid:" + campaign_id)
        return None
    return additional


def _load_amendment_file(root, relative, campaign_id, missing):
    path = _contained_file(root, relative, missing, "successor-amendment")
    if path is None:
        return None
    payload = _load_json_path(path, missing, relative)
    if type(payload) is not dict:
        missing.append("successorAmendments-unresolved:" + campaign_id)
        return None
    authority = payload.get("authority")
    if type(authority) is str and authority != "":
        if _contained_file(root, authority, missing, "successor-authority") is None:
            return None
    additional = payload.get("additionalSeconds")
    if type(additional) is not int or additional < 0:
        missing.append("successorAmendments-unresolved:" + campaign_id)
        return None
    return additional


def _remaining_from_current(
    root, spec, record, binding_obj, action, alloc, historical, campaign_id, missing
):
    if type(spec) is not dict:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    extra = [key for key in spec if key not in CURRENT_ACCOUNTING_KEYS]
    for key in extra:
        missing.append("unknown-currentAccounting-field:" + key)
        return None
    if spec.get("schema") != ACCOUNTING_SCHEMA:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    relative = spec.get("path")
    if type(relative) is not str or relative == "":
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    if type(historical) is str and relative == historical:
        missing.append("resource-historical-snapshot:" + campaign_id)
        return None
    path = _contained_file(root, relative, missing, "current-accounting")
    if path is None:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    payload = _load_json_path(path, missing, relative)
    if type(payload) is not dict:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    extra_acc = [key for key in payload if key not in ACCOUNTING_KEYS]
    for key in extra_acc:
        missing.append("unknown-accounting-field:" + key)
        return None
    if payload.get("schema") != ACCOUNTING_SCHEMA:
        missing.append("resource-snapshot-schema-unresolved:" + str(payload.get("schema")))
        return None
    planning = _nonneg_number(payload.get("planning_charge_seconds"), True)
    overhead = _nonneg_number(payload.get("planning_overhead_reserved_seconds"), True)
    if planning is None or overhead is None:
        missing.append("resource-planning-unresolved:" + campaign_id)
        return None
    master_limit = payload.get("master_limit_seconds")
    package_limit = payload.get("package_limit_seconds")
    if type(master_limit) is not int or master_limit <= 0:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    if type(package_limit) is not int or package_limit <= 0:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    package = payload.get("package")
    rec_owners = _owners_of(record.get("owner"))
    if type(package) is not str or package == "":
        missing.append("resource-scope-mismatch:" + campaign_id)
        return None
    if rec_owners and package not in rec_owners:
        missing.append("resource-scope-mismatch:" + campaign_id)
        return None
    charges = _seconds_list(payload.get("charges"), missing, campaign_id)
    reserved = _seconds_list(payload.get("reserved"), missing, campaign_id)
    external = _seconds_list(payload.get("externalPackageCharges"), missing, campaign_id)
    if charges is None or reserved is None or external is None:
        return None
    master_used = planning + overhead + charges + reserved + external
    package_used = planning + overhead + charges
    pass_binding = _optional_pass_record(
        root, spec.get("passBinding"), PASS_BINDING_OPTIONAL, missing, "pass-binding"
    )
    if pass_binding is False:
        return None
    pass_ledger = _optional_pass_record(
        root, spec.get("passLedger"), PASS_LEDGER_KEYS, missing, "pass-ledger"
    )
    if pass_ledger is False:
        return None
    if type(pass_ledger) is dict and pass_ledger.get("stopped") is True:
        missing.append("resource-pass-stopped:" + campaign_id)
        return None
    envelope_remain = None
    if type(pass_binding) is dict:
        envelope_id = pass_binding.get("envelope")
        envelopes = payload.get("successorEnvelopes")
        if type(envelope_id) is str and envelope_id != "":
            if type(envelopes) is not dict or envelope_id not in envelopes:
                missing.append("resource-envelope-unresolved:" + envelope_id)
                return None
            env = envelopes[envelope_id]
            if type(env) is not dict:
                missing.append("resource-envelope-unresolved:" + envelope_id)
                return None
            envelope_remain = env.get("remainingReservedSeconds")
            if type(envelope_remain) is not int:
                missing.append("resource-envelope-unresolved:" + envelope_id)
                return None
            env_dispatches = env.get("workerDispatches")
            env_limit = env.get("workerDispatchLimit")
            if action["kind"] in IMPLEMENT_KINDS:
                if type(env_dispatches) is not int or type(env_limit) is not int:
                    missing.append("resource-dispatch-unresolved:" + campaign_id)
                    return None
                if env_dispatches >= env_limit:
                    missing.append("resource-dispatch-exhausted:" + campaign_id)
                    return None
    if action["kind"] in IMPLEMENT_KINDS:
        dispatches = payload.get("worker_dispatches")
        dispatch_limit = payload.get("worker_dispatch_limit")
        if type(dispatches) is not int or type(dispatch_limit) is not int:
            missing.append("resource-dispatch-unresolved:" + campaign_id)
            return None
        if dispatches >= dispatch_limit:
            missing.append("resource-dispatch-exhausted:" + campaign_id)
            return None
    cost = _action_cost(action, binding_obj, pass_binding, campaign_id, missing)
    if cost is None:
        return None
    remainders = [
        master_limit - master_used,
        package_limit - package_used,
        alloc,
    ]
    if type(envelope_remain) is int:
        remainders.append(envelope_remain)
    remaining = min(remainders) - cost
    return remaining


def _nonneg_number(value, required):
    if value is None:
        return None if required else 0
    if type(value) is bool:
        return None
    if type(value) is int:
        return value if value >= 0 else None
    if type(value) is float:
        if value != value or value in (float("inf"), float("-inf")):
            return None
        return value if value >= 0 else None
    return None


def _seconds_list(rows, missing, campaign_id):
    if rows is None:
        return 0
    if type(rows) is not list:
        missing.append("resource-live-state-unavailable:" + campaign_id)
        return None
    total = 0
    for row in rows:
        if type(row) is not dict:
            missing.append("resource-live-state-unavailable:" + campaign_id)
            return None
        seconds = row.get("seconds")
        if type(seconds) is bool or type(seconds) is not int or seconds < 0:
            missing.append("resource-live-state-unavailable:" + campaign_id)
            return None
        total += seconds
    return total


def _optional_pass_record(root, relative, allowed, missing, kind):
    if relative is None:
        return None
    if type(relative) is not str or relative == "":
        missing.append(kind + "-unresolved")
        return False
    path = _contained_file(root, relative, missing, kind)
    if path is None:
        return False
    payload = _load_json_path(path, missing, relative)
    if type(payload) is not dict:
        missing.append(kind + "-not-object")
        return False
    extra = [key for key in payload if key not in allowed]
    for key in extra:
        missing.append("unknown-" + kind + "-field:" + key)
        return False
    return payload


PASS_BINDING_OPTIONAL = (
    "authority",
    "base",
    "envelope",
    "owned",
    "parentCandidate",
    "parentFreezeSha256",
    "resources",
    "runtimeNodeModules",
    "schema",
    "scope",
    "sources",
    "worktree",
    "allocationSeconds",
    "workerDispatches",
    "retainedBytes",
    "ownedBytesLimit",
    "verificationCommands",
)


def _action_cost(action, binding_obj, pass_binding, campaign_id, missing):
    kind = action["kind"]
    if kind == "report":
        return 0
    resources = {}
    if type(binding_obj) is dict and type(binding_obj.get("resources")) is dict:
        resources = binding_obj["resources"]
    if type(pass_binding) is dict and type(pass_binding.get("resources")) is dict:
        merged = dict(resources)
        merged.update(pass_binding["resources"])
        resources = merged
    commands = resources.get("verificationCommands")
    if type(commands) is not dict:
        commands = {}
    if kind == "review":
        value = commands.get("review")
        if type(value) is int and value > 0:
            return value
        gpt = resources.get("gpt6ReviewSeconds")
        if type(gpt) is int and gpt > 0:
            return gpt
        missing.append("resource-action-cost-unresolved:" + campaign_id)
        return None
    if kind == "verify":
        value = commands.get("verify")
        if type(value) is int and value > 0:
            return value
        verify = resources.get("verificationSeconds")
        if type(verify) is int and verify > 0:
            return verify
        missing.append("resource-action-cost-unresolved:" + campaign_id)
        return None
    if kind == "reproduce":
        value = commands.get("reproduce")
        if type(value) is int and value > 0:
            return value
        verify = resources.get("verificationSeconds")
        if type(verify) is int and verify > 0:
            return verify
        missing.append("resource-action-cost-unresolved:" + campaign_id)
        return None
    if kind in IMPLEMENT_KINDS or kind == "admin":
        workers = resources.get("workerSeconds")
        if type(workers) is list and workers and type(workers[0]) is int and workers[0] > 0:
            return workers[0]
        bound = resources.get("allocationSeconds")
        if type(bound) is int and bound > 0:
            return bound
        missing.append("resource-action-cost-unresolved:" + campaign_id)
        return None
    missing.append("resource-action-cost-unresolved:" + campaign_id)
    return None


def _resolve_file_admission(root, ref, missing):
    if ref == "":
        missing.append("admissionRef-unresolved:")
        return False, False, False
    path = _contained_file(root, ref, missing, "admissionRef")
    if path is None:
        return False, False, False
    payload = _load_json_path(path, missing, ref)
    if type(payload) is not dict:
        missing.append("admissionRef-unresolved-shape:" + ref)
        return False, False, False
    schema = payload.get("schema")
    missing.append("admissionRef-unsupported-schema:" + str(schema))
    return False, False, False


def _select_next(
    root,
    actions,
    current,
    history_fields,
    history_reader,
    program,
    sprints,
    campaigns,
    program_ok,
    sprints_ok,
    campaign_ok,
    missing,
):
    ranked = sorted(
        actions,
        key=lambda row: (
            NEXT_KIND_ORDER.index(row["kind"])
            if row["kind"] in NEXT_KIND_ORDER
            else 99,
            row["id"],
        ),
    )
    history_missing = [
        item
        for item in missing
        if item == "operational-history" or item.startswith("operational-history")
    ]
    for other in ranked:
        local = []
        flags = _action_flags(
            root,
            other,
            program,
            sprints,
            campaigns,
            program_ok,
            sprints_ok,
            campaign_ok,
            local,
        )
        snap = _snapshot(**flags, missingEvidence=list(local))
        same_lineage = (
            other["requirement"] == current["requirement"]
            and other["capability"] == current["capability"]
        )
        if same_lineage:
            snap.update(history_fields)
            for item in history_missing:
                if item not in local:
                    local.append(item)
        else:
            _apply_history(history_reader, root, other, snap, local)
        snap["missingEvidence"] = list(local)
        snap["nextActionId"] = None
        decision = assess(snap, other)
        if decision.get("allow") is True:
            return other["id"]
    missing.append("next-action-none")
    return None


def _apply_history(history_reader, root, action, snap, missing):
    if history_reader is None:
        missing.append("operational-history")
        return
    if not callable(history_reader):
        missing.append("operational-history-not-callable")
        return
    lineage = {
        "repository": _repository_identity(root, missing),
        "requirement": action["requirement"],
        "capability": action["capability"],
    }
    try:
        payload = history_reader(lineage)
    except Exception as exc:
        missing.append("operational-history-error:" + type(exc).__name__)
        return
    if type(payload) is not dict:
        missing.append("operational-history-invalid")
        return
    extra = [key for key in payload if key not in HISTORY_KEYS]
    if extra:
        missing.append("operational-history-invalid")
        return
    for key in HISTORY_INT_KEYS:
        value = payload.get(key)
        if type(value) is not int or value < 0:
            missing.append("operational-history-type:" + key)
            return
        snap[key] = value
    for key in HISTORY_BOOL_KEYS:
        value = payload.get(key)
        if type(value) is not bool:
            missing.append("operational-history-type:" + key)
            return
        snap[key] = value


def _repository_identity(root, missing):
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--git-common-dir"],
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
            timeout=5,
        )
    except (OSError, subprocess.TimeoutExpired):
        missing.append("git-common-dir-unresolved")
        return str(root)
    if result.returncode != 0:
        missing.append("git-common-dir-unresolved")
        return str(root)
    raw = result.stdout.strip()
    if raw == "":
        missing.append("git-common-dir-unresolved")
        return str(root)
    path = Path(raw)
    if not path.is_absolute():
        path = root / path
    try:
        return str(path.resolve())
    except OSError:
        missing.append("git-common-dir-unresolved")
        return str(root)


def _contained_file(root, relative, missing, kind):
    if type(relative) is not str or relative == "":
        missing.append(kind + "-path-empty")
        return None
    posix = relative.replace("\\", "/")
    if posix.startswith("/") or _has_drive(posix):
        missing.append(kind + "-path-not-contained:" + relative)
        return None
    parts = [part for part in posix.split("/") if part not in ("", ".")]
    if any(part == ".." for part in parts):
        missing.append("path-escape:" + relative)
        return None
    if _secret_parts(parts):
        missing.append("secret-path-refused:" + relative)
        return None
    candidate = root.joinpath(*parts)
    try:
        resolved = candidate.resolve()
    except OSError:
        missing.append(kind + "-path-unresolved:" + relative)
        return None
    if not resolved.is_relative_to(root):
        missing.append("path-containment-rejected-symlink:" + relative)
        return None
    try:
        resolved_rel = resolved.relative_to(root)
        if _secret_parts(resolved_rel.parts):
            missing.append("secret-path-refused:" + relative)
            return None
    except ValueError:
        missing.append("path-containment-rejected:" + relative)
        return None
    if not resolved.is_file():
        missing.append(kind + "-missing:" + relative)
        return None
    return resolved


def _secret_parts(parts):
    for part in parts:
        lowered = part.lower()
        for secret in SECRET_PARTS:
            if lowered == secret or lowered.startswith(secret + "."):
                return True
            if secret in lowered.split("."):
                return True
            if secret in lowered and secret in ("wallet", "seed", "witness"):
                return True
    return False


def _has_drive(path):
    return len(path) >= 2 and path[1] == ":" and path[0].isalpha()


def _structural_block(missing):
    markers = (
        "duplicate-",
        "unknown-",
        "schemaVersion-",
        "sprints-schemaVersion",
        "actions-schema-",
        "secret-path-refused:",
        "path-escape:",
        "path-containment-",
        "binding-schema-",
        "campaign-schema-",
        "entryGate-requires-",
        "commandRef-",
        "unsupported-",
        "unknown-nested-",
        "unknown-actions-field:",
        "unknown-command-field:",
        "unknown-stageAdmission-field:",
        "unknown-entryGate-field:",
        "unknown-program-field:",
        "unknown-campaign-field:",
        "unknown-manifest-field:",
        "unknown-acceptance-field:",
        "unknown-review-field:",
        "unknown-accounting-field:",
        "stageAdmission-schema-",
        "resource-schema-unresolved:",
        "candidateManifest-schema-",
    )
    return any(item.startswith(markers) for item in missing)
