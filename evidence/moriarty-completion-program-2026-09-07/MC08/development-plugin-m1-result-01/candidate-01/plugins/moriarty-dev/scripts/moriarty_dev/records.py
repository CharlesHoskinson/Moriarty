"""Exact-source readers for Moriarty development plugin snapshots.

``load_snapshot(repo, action_id, history_reader=None)`` and ``read_actions(repo)``
read current bytes. They never write, never launch ``commandRef`` argv, never
import referenced code, and never open secret paths. Missing or conflicting
sources become named ``missingEvidence`` strings. Missing operational history
is ``operational-history`` and is not a verified zero-failure count.

This module does not grant admission. It reports whether referenced program,
sprint, campaign, binding, resource and command records currently resolve.

On-disk reader contract (paths are relative to the supplied repository root):

- ``.moriarty-dev/actions.json``
  Closed object. Required keys: ``schema``, ``actions``.
  ``schema`` must be ``moriarty-dev.actions/1``.
  ``actions`` is a list of closed action objects with exactly:
  ``id``, ``requirement``, ``capability``, ``kind``, ``candidate``,
  ``admissionRef``, ``commandRef``, ``evidenceProfile`` (all ``str``).
  Duplicate JSON keys, unknown keys and non-string values are unresolved.
  The catalog maps references. It is not an acceptance register.

- ``openspec/moriarty-completion-program.json``
  Existing program register. ``schemaVersion`` must be ``int`` (not ``bool``,
  not ``str``). Task eligibility uses ``reportReconciliation.stageAdmission``
  stages and sprint ``entryGates``. ``dispatchEnabled`` must be ``bool`` and
  never admits work by itself. Whole-sprint ``status`` never decides entry.

- ``openspec/sprints/sprints.json``
  Existing sprint register. ``schemaVersion`` must be ``int``.
  ``dispatchEnabled`` and ``resourceAllocationGranted`` must be ``bool``.
  Neither flag admits resources. Entry uses the gate that lists the action
  ``requirement`` task id. Required stage ids must have stage ``status``
  ``complete``. A blocked ``f0`` stage does not make an SP05 task ineligible
  unless that task's own gate lists ``f0``.

- Campaign store from ``stageAdmission.campaignRecordStore``, defaulting to
  ``evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json``.
  Supported schema: ``moriarty.campaign-admission/1``.
  Supported ``admissionRef`` form: ``campaign:<id>`` where ``<id>`` is a key
  under ``campaigns``. The record is not permission. Resource admission needs
  that record plus a current binding digest and an existing resource-amendment
  file. A stage ``complete`` flag or stage ``candidateHash`` without a campaign
  record does not admit resources.

- Binding files referenced by ``campaigns.*.binding``
  Bytes are hashed with SHA-256 and compared to ``bindingSha256``. A mismatch
  is stale. Supported binding schemas when present:
  ``moriarty.sp01-execution-binding/1`` (existing program bindings) and
  ``moriarty.source-ledger-integration/1`` (fixture bindings used by tests).
  Fixture schema is not production authority. Hash comparison is on raw bytes.

- Resource files referenced by ``campaigns.*.resourceAmendment``
  Must exist, stay inside the repository and parse as JSON. Existing program
  amendments may omit ``schema``. The fixture schema
  ``moriarty.delegated-resource-amendment/1`` is accepted as a file shape only.

- ``commandRef``
  Empty string: no command (valid for ``report``).
  Supported form: ``<contained-json>#<name>``.
  File schema must be ``moriarty-dev.commands/1`` with ``commands.<name>.argv``
  a list of strings. The argv list is never executed. Absolute paths, path
  escape and missing names are unresolved as ``commandRef-*``.

Unsupported ``admissionRef`` values:

- A contained JSON file is opened only to name its ``schema``. Unknown schemas
  are recorded exactly, for example the schema string itself. They do not admit.
- A digest manifest such as ``moriarty-dev.unauthoritative-digest/1`` is not
  authority even when hashes match.
- ``..`` segments, resolved symlink escape and absolute paths are rejected
  before read when detected, with ``path``, ``escape``, ``containment`` or
  ``symlink`` in the evidence name.
- Paths whose parts include secret names (``wallet``, ``seed``, ``credential``,
  ``witness``, ``token``, ``.env``, ``private-key``) are refused without opening.

Reads are bounded at 8 MiB per file. JSON object duplicate keys are rejected.
``history_reader``, when supplied, is an injected internal function
``reader({"repository", "requirement", "capability"}) -> dict``. Production
code does not install an always-deny or always-zero history stub. Absence of a
reader is ``operational-history``.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

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
BINDING_SCHEMAS = frozenset(
    (
        "moriarty.sp01-execution-binding/1",
        "moriarty.source-ledger-integration/1",
    )
)
MAX_BYTES = 8 * 1024 * 1024
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
ADMITTED_STATUS_PREFIXES = ("admitted-", "complete")


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

    entry_eligible = False
    if action is not None and program_ok and sprints_ok:
        entry_eligible = _entry_eligible(
            sprints, program, action["requirement"], missing
        )

    authority = False
    candidate_current = False
    resource_admitted = False
    if action is not None:
        command_ok = _resolve_command(root, action, missing)
        admitted, candidate_current, resource_admitted = _resolve_admission(
            root, action, campaigns if campaign_ok else None, missing
        )
        authority = bool(
            program_ok
            and sprints_ok
            and campaign_ok
            and command_ok
            and admitted
            and _no_blocking_input_defects(missing)
        )

    snap = {
        "authorityCurrent": authority,
        "entryEligible": entry_eligible,
        "candidateCurrent": candidate_current,
        "resourceAdmitted": resource_admitted,
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
        _apply_history(history_reader, root, action, snap, missing)
    else:
        if history_reader is None:
            missing.append("operational-history")
    snap["missingEvidence"] = list(missing)
    return _snapshot(**snap)


def read_actions(repo):
    missing = []
    root = Path(repo).resolve()
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
    path = root / relative
    if not path.is_file():
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
    if payload.get("schema") != ACTIONS_SCHEMA:
        missing.append(
            "actions-schema-unresolved:" + str(payload.get("schema"))
        )
        return []
    rows = payload.get("actions")
    if type(rows) is not list:
        missing.append("actions-not-list")
        return []
    actions = []
    for index, row in enumerate(rows):
        parsed = _closed_action(row, missing, index)
        if parsed is not None:
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
    if "schemaVersion" not in program:
        missing.append("schemaVersion-missing")
        return False
    if type(program["schemaVersion"]) is not int:
        missing.append("schemaVersion-not-int")
        return False
    reconciliation = program.get("reportReconciliation")
    if type(reconciliation) is not dict:
        missing.append("reportReconciliation-unresolved")
        return False
    admission = reconciliation.get("stageAdmission")
    if type(admission) is not dict:
        missing.append("stageAdmission-unresolved")
        return False
    if type(admission.get("dispatchEnabled")) is not bool:
        missing.append("dispatchEnabled-not-bool")
        return False
    if type(admission.get("campaignRecordStore")) is not str:
        missing.append("campaignRecordStore-not-str")
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
        requires = stage.get("requires", [])
        if type(requires) is not list or any(type(item) is not str for item in requires):
            missing.append("stage-requires-unresolved:" + stage_id)
            valid = False
        candidate = stage.get("candidateHash", None)
        if candidate is not None and type(candidate) is not str:
            missing.append("stage-candidateHash-type:" + stage_id)
            valid = False
        record_id = stage.get("campaignRecordId", None)
        if record_id is not None and type(record_id) is not str:
            missing.append("stage-campaignRecordId-type:" + stage_id)
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
            tasks = gate.get("tasks", [])
            requires = gate.get("requires", [])
            if type(tasks) is not list or any(type(item) is not str for item in tasks):
                missing.append("entryGate-tasks-unresolved")
                valid = False
            if type(requires) is not list or any(
                type(item) is not str for item in requires
            ):
                missing.append("entryGate-requires-unresolved")
                valid = False
    return valid


def _validate_campaigns(campaigns, missing):
    if campaigns is None:
        return False
    if type(campaigns) is not dict:
        missing.append("campaign-store-not-object")
        return False
    schema = campaigns.get("schema")
    if schema != CAMPAIGN_SCHEMA:
        missing.append("campaign-schema-unresolved:" + str(schema))
        return False
    table = campaigns.get("campaigns")
    if type(table) is not dict:
        missing.append("campaigns-not-object")
        return False
    return True


def _entry_eligible(sprints, program, requirement, missing):
    gate = None
    for sprint in sprints.get("sprints", []):
        if type(sprint) is not dict:
            continue
        for candidate in sprint.get("entryGates", []):
            if type(candidate) is not dict:
                continue
            tasks = candidate.get("tasks", [])
            if type(tasks) is list and requirement in tasks:
                gate = candidate
                break
        if gate is not None:
            break
    if gate is None:
        missing.append("entryGate-unresolved:" + requirement)
        return False
    requires = gate.get("requires", [])
    stages = {}
    for stage in program["reportReconciliation"]["stageAdmission"]["stages"]:
        if type(stage) is dict and type(stage.get("id")) is str:
            stages[stage["id"]] = stage
    for required in requires:
        stage = stages.get(required)
        if stage is None:
            missing.append("required-stage-missing:" + required)
            return False
        if stage.get("status") != "complete":
            missing.append(
                "required-stage-incomplete:"
                + required
                + ":"
                + str(stage.get("status"))
            )
            return False
    return True


def _resolve_command(root, action, missing):
    ref = action["commandRef"]
    if ref == "":
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
    if payload.get("schema") != COMMANDS_SCHEMA:
        missing.append(
            "commandRef-unsupported-schema:" + str(payload.get("schema"))
        )
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
    argv = spec.get("argv")
    if type(argv) is not list or any(type(item) is not str for item in argv):
        missing.append("commandRef-unresolved:" + ref)
        return False
    return True


def _resolve_admission(root, action, campaigns, missing):
    ref = action["admissionRef"]
    if ref.startswith("campaign:"):
        return _resolve_campaign_admission(root, action, campaigns, missing)
    return _resolve_file_admission(root, ref, missing)


def _resolve_campaign_admission(root, action, campaigns, missing):
    ref = action["admissionRef"]
    campaign_id = ref[len("campaign:") :]
    if campaign_id == "":
        missing.append("admissionRef-unresolved:campaign:")
        return False, False, False
    if campaigns is None:
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
    binding_ok, binding_obj = _verify_binding(root, campaign_id, record, missing)
    resource_ok = _verify_resource(root, campaign_id, record, missing)
    record_hash = record.get("candidateHash")
    binding_hash = None
    if binding_obj is not None and type(binding_obj.get("candidateHash")) is str:
        binding_hash = binding_obj["candidateHash"]
    if (
        type(record_hash) is str
        and binding_hash is not None
        and record_hash != binding_hash
    ):
        missing.append("candidateHash-conflict:" + campaign_id)
        binding_ok = False
    # Lineage is (repository, requirement, capability). The action candidate
    # string may be renamed. Currentness is the binding byte digest, not the
    # label.
    candidate_current = binding_ok
    status = record.get("status")
    status_ok = type(status) is str and status.startswith(ADMITTED_STATUS_PREFIXES)
    resource_admitted = binding_ok and resource_ok and status_ok
    return binding_ok, candidate_current, resource_admitted


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
    schema = payload.get("schema")
    if schema is not None and schema not in BINDING_SCHEMAS:
        missing.append("binding-schema-unresolved:" + str(schema))
    return True, payload


def _verify_resource(root, campaign_id, record, missing):
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
    return True


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


def _apply_history(history_reader, root, action, snap, missing):
    if history_reader is None:
        missing.append("operational-history")
        return
    if not callable(history_reader):
        missing.append("operational-history-not-callable")
        return
    lineage = {
        "repository": str(root),
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
    next_id = payload.get("nextActionId", None)
    if next_id is not None and type(next_id) is not str:
        missing.append("operational-history-type:nextActionId")
        return
    snap["nextActionId"] = next_id


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
        if candidate.is_symlink():
            resolved = candidate.resolve()
            if not resolved.is_relative_to(root):
                missing.append("path-containment-rejected-symlink:" + relative)
                return None
        resolved = candidate.resolve()
    except OSError:
        missing.append(kind + "-path-unresolved:" + relative)
        return None
    if not resolved.is_relative_to(root):
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
            if lowered == secret or lowered.startswith(secret + ".") or secret in lowered.split("."):
                return True
            if secret in lowered and secret in ("wallet", "seed", "witness"):
                return True
    return False


def _has_drive(path):
    return len(path) >= 2 and path[1] == ":" and path[0].isalpha()


def _no_blocking_input_defects(missing):
    markers = (
        "duplicate-key:",
        "unknown-action-field:",
        "schemaVersion-",
        "dispatchEnabled-not-bool",
        "secret-path-refused:",
        "path-escape:",
        "path-containment-",
    )
    return not any(item.startswith(markers) for item in missing)
