"""Pure next-action assessment for the Moriarty development plugin.

``assess(snapshot, action)`` is deterministic and has no I/O. It does not
grant admission. Booleans and integers are distinct: ``bool`` is not ``int``.
Unknown keys, missing keys and wrong types yield ``INPUT_INVALID``.

Blocking checks, in order, for a well-typed input:

1. unknown action kind
2. read-only ``report`` (always allowed)
3. primary implementation reservation (one writer at a time)
4. administrative cycle/time limits
5. allow

Everything else is advisory. Missing or stale campaign authority, stale
candidate evidence, task entry eligibility, resource admission,
admission/command gaps, unresolved operational history and repeated
same-defect failures (unchanged retries, missing review receipts, missing
independent approval, consumed attempts) never deny an action. The guarded
CLI still cannot launch a command it cannot resolve; that is structural. Each one is recorded in ``warnings`` as the reason
code it used to produce, and the action stays ``ALLOWED``. Report remains
allowed whenever dispatch is denied.
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
DECISION_KEYS = ("allow", "reasonCode", "nextActionId", "missingEvidence", "warnings")
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
    warnings = _advisories(snapshot, kind, missing)
    if kind == "report":
        return _decision(True, "ALLOWED", next_id, missing, warnings)
    if snapshot["primaryActive"] is True and kind in PRIMARY_BLOCKED_KINDS:
        return _decision(False, "PRIMARY_ACTIVE", next_id, missing, warnings)
    if kind == "admin" and _admin_limit(snapshot):
        return _decision(False, "ADMIN_LIMIT", next_id, missing, warnings)
    return _decision(True, "ALLOWED", next_id, missing, warnings)


def _advisories(snapshot, kind, missing):
    """Former gates, now warnings: recorded, never blocking."""
    warnings = []
    if snapshot["authorityCurrent"] is False:
        warnings.append("AUTHORITY_UNAVAILABLE")
    if snapshot["candidateCurrent"] is False:
        warnings.append("EVIDENCE_STALE")
    if snapshot["entryEligible"] is False:
        warnings.append("ENTRY_INELIGIBLE")
    if snapshot["resourceAdmitted"] is False:
        warnings.append("RESOURCE_EXHAUSTED")
    if _admission_gap(missing):
        warnings.append("ADMISSION_UNRESOLVED")
    if _history_gap(missing) and kind not in HISTORY_EXEMPT_KINDS:
        warnings.append("HISTORY_UNRESOLVED")
    same_failures = snapshot.get("sameDefectFailures")
    if same_failures is not None and same_failures >= FAILURE_RETRY_LIMIT:
        if kind in RETRY_BLOCKED_KINDS:
            warnings.append("REPRODUCE_BEFORE_RETRY")
        elif kind == "repair" and not (
            snapshot["reproducerVerified"] is True
            and snapshot["approachChanged"] is True
        ):
            warnings.append("REPAIR_UNREADY")
    return warnings


def _decision(allow, reason_code, next_action_id, missing_evidence, warnings=()):
    return {
        "allow": allow,
        "reasonCode": reason_code,
        "nextActionId": next_action_id,
        "missingEvidence": list(missing_evidence),
        "warnings": list(warnings),
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
    has_hist_gap = _history_gap(snapshot.get("missingEvidence", [])) if isinstance(snapshot.get("missingEvidence"), list) else False
    for key in INT_SNAPSHOT_KEYS:
        value = snapshot[key]
        if has_hist_gap and value is None:
            continue
        if value is None and key in ("adminCycles", "adminSeconds"):
            continue
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


def _history_gap(missing: list) -> bool:
    return any(
        isinstance(item, str) and (item == "operational-history" or item.startswith("operational-history"))
        for item in missing
    )


def _admin_limit(snapshot):
    cycles = snapshot.get("adminCycles")
    seconds = snapshot.get("adminSeconds")
    return (
        (cycles is not None and cycles >= ADMIN_CYCLE_LIMIT)
        or (seconds is not None and seconds >= ADMIN_SECONDS_LIMIT)
    )
