"""Pure next-action assessment for the Moriarty development plugin.

``assess(snapshot, action)`` is deterministic and has no I/O. It does not
grant admission. Booleans and integers are distinct: ``bool`` is not ``int``.
Unknown keys, missing keys and wrong types yield ``INPUT_INVALID``.

Check order for a well-typed input:

1. unknown action kind
2. read-only ``report`` (always allowed)
3. current authority
4. current candidate bytes/hash
5. task entry eligibility
6. resource admission
7. named admission/command gaps
8. unresolved operational history
9. primary implementation reservation
10. repeated same-defect failures
11. administrative cycle/time limits
12. allow

A focused repair after two same-defect failures requires both
``reproducerVerified`` and ``approachChanged``. Repair occupies the primary
implementation slot. Report remains allowed when dispatch is denied.
Reproduce remains allowed for repeated-failure, history, admin and
concurrency limits, but not for admission/command gaps.
"""
from __future__ import annotations

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
DECISION_KEYS = ("allow", "reasonCode", "nextActionId", "missingEvidence")
KNOWN_KINDS = frozenset(
    ("implement", "reproduce", "repair", "verify", "review", "admin", "report")
)
BOOL_SNAPSHOT_KEYS = (
    "authorityCurrent",
    "entryEligible",
    "candidateCurrent",
    "resourceAdmitted",
    "primaryActive",
    "reproducerVerified",
    "approachChanged",
)
INT_SNAPSHOT_KEYS = ("sameDefectFailures", "adminCycles", "adminSeconds")
HISTORY_EXEMPT_KINDS = frozenset(("report", "reproduce", "review", "verify"))
PRIMARY_BLOCKED_KINDS = frozenset(("implement", "repair", "admin"))
RETRY_BLOCKED_KINDS = frozenset(("implement", "admin"))
ADMIN_SECONDS_LIMIT = 1800
ADMIN_CYCLE_LIMIT = 2
FAILURE_RETRY_LIMIT = 2


def assess(snapshot, action):
    problems = []
    snap_ok = _validate_snapshot(snapshot, problems)
    act_ok = _validate_action(action, problems)
    if not snap_ok or not act_ok:
        return _decision(False, "INPUT_INVALID", None, problems)

    kind = action["kind"]
    missing = list(snapshot["missingEvidence"])
    next_id = snapshot["nextActionId"]
    if kind not in KNOWN_KINDS:
        return _decision(False, "UNKNOWN_KIND", next_id, missing)
    if kind == "report":
        return _decision(True, "ALLOWED", next_id, missing)
    if snapshot["authorityCurrent"] is False:
        return _decision(False, "AUTHORITY_UNAVAILABLE", next_id, missing)
    if snapshot["candidateCurrent"] is False:
        return _decision(False, "EVIDENCE_STALE", next_id, missing)
    if snapshot["entryEligible"] is False:
        return _decision(False, "ENTRY_INELIGIBLE", next_id, missing)
    if snapshot["resourceAdmitted"] is False:
        return _decision(False, "RESOURCE_EXHAUSTED", next_id, missing)
    if _admission_gap(missing):
        return _decision(False, "ADMISSION_UNRESOLVED", next_id, missing)
    if _history_gap(missing) and kind not in HISTORY_EXEMPT_KINDS:
        return _decision(False, "HISTORY_UNRESOLVED", next_id, missing)
    if snapshot["primaryActive"] is True and kind in PRIMARY_BLOCKED_KINDS:
        return _decision(False, "PRIMARY_ACTIVE", next_id, missing)
    if snapshot["sameDefectFailures"] >= FAILURE_RETRY_LIMIT:
        if kind in RETRY_BLOCKED_KINDS:
            return _decision(False, "REPRODUCE_BEFORE_RETRY", next_id, missing)
        if kind == "repair":
            ready = (
                snapshot["reproducerVerified"] is True
                and snapshot["approachChanged"] is True
            )
            if not ready:
                return _decision(False, "REPAIR_UNREADY", next_id, missing)
    if kind == "admin" and _admin_limit(snapshot):
        return _decision(False, "ADMIN_LIMIT", next_id, missing)
    return _decision(True, "ALLOWED", next_id, missing)


def _decision(allow, reason_code, next_action_id, missing_evidence):
    return {
        "allow": allow,
        "reasonCode": reason_code,
        "nextActionId": next_action_id,
        "missingEvidence": list(missing_evidence),
    }


def _validate_snapshot(snapshot, problems):
    if type(snapshot) is not dict:
        problems.append("snapshot-not-object")
        return False
    extra = [key for key in snapshot if key not in SNAPSHOT_KEYS]
    absent = [key for key in SNAPSHOT_KEYS if key not in snapshot]
    for key in extra:
        problems.append("unknown-snapshot-field:" + key)
    for key in absent:
        problems.append("missing-snapshot-field:" + key)
    if extra or absent:
        return False
    valid = True
    for key in BOOL_SNAPSHOT_KEYS:
        if type(snapshot[key]) is not bool:
            problems.append("snapshot-type:" + key)
            valid = False
    for key in INT_SNAPSHOT_KEYS:
        value = snapshot[key]
        if type(value) is not int or value < 0:
            problems.append("snapshot-type:" + key)
            valid = False
    next_id = snapshot["nextActionId"]
    if next_id is not None and type(next_id) is not str:
        problems.append("snapshot-type:nextActionId")
        valid = False
    evidence = snapshot["missingEvidence"]
    if type(evidence) is not list:
        problems.append("snapshot-type:missingEvidence")
        valid = False
    elif any(type(item) is not str for item in evidence):
        problems.append("snapshot-type:missingEvidence")
        valid = False
    return valid


def _validate_action(action, problems):
    if type(action) is not dict:
        problems.append("action-not-object")
        return False
    extra = [key for key in action if key not in ACTION_KEYS]
    absent = [key for key in ACTION_KEYS if key not in action]
    for key in extra:
        problems.append("unknown-action-field:" + key)
    for key in absent:
        problems.append("missing-action-field:" + key)
    if extra or absent:
        return False
    valid = True
    for key in ACTION_KEYS:
        if type(action[key]) is not str:
            problems.append("action-type:" + key)
            valid = False
    return valid


def _admission_gap(missing):
    markers = (
        "commandRef-",
        "admissionRef-",
        "duplicate-action-id:",
        "unknown-actions-field:",
        "actions-schema-unresolved:",
        "binding-schema-",
        "campaign-schema-unresolved:",
        "unknown-command-field:",
        "unknown-nested-",
        "entryGate-requires-unresolved",
    )
    for item in missing:
        if item.startswith(markers):
            return True
    return False


def _history_gap(missing):
    return any(
        item == "operational-history" or item.startswith("operational-history")
        for item in missing
    )


def _admin_limit(snapshot):
    return (
        snapshot["adminCycles"] >= ADMIN_CYCLE_LIMIT
        or snapshot["adminSeconds"] >= ADMIN_SECONDS_LIMIT
    )
