"""Append-only reconciliation for unknown/cleanup-debt original attempts.

A reconciliation record never edits or deletes history. It references
exactly one original (task, attempt, fingerprint, adapter) plus the
canonical digest of that original consumer Result, and carries fixed
actual evidence for that same operation:

- ``process-evidence`` (local adapters): the outer operator's
  doctorwatson.process-reconciliation-evidence/2 JSON, binding the
  canonical original-result digest (the job token over the whole
  original Result record) and stating the observed process outcome
  (not-started, or exited with exit_code/signal) and
  cleanup/restoration states. The stated outcome, exit_code and signal
  must be consistent with the original record's own recorded
  terminal/exit_code/signal/submission: a known-ran original cannot be
  discharged as not-started, and recorded exit/signal cannot be
  contradicted. The evidence's ``preexec_sha256`` field is only an
  operator assertion: it is format-checked, but no PREEXEC file is
  opened and it is bound to no FileRef, so it establishes nothing
  beyond the existing ``producer_authenticity: not-established``
  basis. A supplied clearDebt-style boolean is not a field anywhere.
- ``source-result`` (sherlock-anchor-settle-readback): the raw harness
  settle result for the original transaction, decoded only via
  settle_observation.decode_sherlock_settle_result, plus a
  cleanup-observation receipt for the original attempt.

Missing executor receipt stays unknown-reconcile. A genuine applicable
resolution lets a later plan attempt become a selection candidate again
without erasing the old history; a record for a different attempt,
fingerprint or original-result digest cannot clear anything.

assess_reconciled folds validated records around delivery.assess,
deriving each original attempt's unknown/debt obligations from the
consumer's own delivery.attempt_obligations (the same derivation
assess uses, including actual resource-receipt artifact validity), so
lifting one attempt's obligation can never erase a sibling's.
``load_readiness`` is the ONE common readiness path: each readiness
input is read ONCE through the pinning reader and the pinned bytes
themselves are strictly parsed (delivery.parse_plan_bytes /
parse_history_bytes / parse_reconciliations_bytes), so the parsed
objects are provably the hashed bytes; a post-read pin recheck is
retained as a change check. The shell ``--inspect`` route and the
executor prelaunch recheck in run_selected both consume it, so
inspection and launch cannot disagree on reconciled readiness for
the same file identities. This pins readiness inputs only; it does
not make pathname-based guard execution or config reads at dispatch
time atomic. No producer authentication is claimed beyond delivery's
existing basis.

Retry rule after a genuine original discharge: any discharged original
(process-not-started, process-exited, cleanup-complete of an exited
process, tx-included) leaves the task a candidate. A same-fingerprint
retry of a consumed outcome carries an advisory warning and is logged;
it is never suppressed. An unresolved current plan attempt, a non-active
(cancelled/blocked) disposition, or a possible-launch current attempt
never becomes a candidate through reconciliation.
"""
from __future__ import annotations

import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat

import delivery  # sibling ordinary consumer in models/doctorwatson

import settle_observation

HEX = re.compile(r"[0-9a-f]{64}\Z")
MAX_JSON_BYTES = 4 * 1024 * 1024
RECORD_KEYS = ("schema", "task", "attempt", "fingerprint", "adapter",
               "original_result_sha256", "resolution", "evidence")
EVIDENCE_ROLES = {
    "process-not-started": ("process-evidence",),
    "process-exited": ("process-evidence",),
    "cleanup-complete": ("process-evidence",),
    "tx-included": ("source-result", "post-state", "resource-receipt"),
}


class ReconciliationError(ValueError):
    pass


def _pairs(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise ReconciliationError(f"duplicate JSON member: {key}")
        out[key] = value
    return out


def _float(text):
    value = float(text)
    if not math.isfinite(value):
        raise ReconciliationError("nonfinite JSON number")
    return value


def _constant(text):
    raise ReconciliationError(f"non-JSON numeric constant: {text}")


def _json(raw: bytes):
    if len(raw) > MAX_JSON_BYTES:
        raise ReconciliationError("JSON size limit exceeded")
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=_pairs,
                          parse_constant=_constant, parse_float=_float)
    except (UnicodeError, json.JSONDecodeError, RecursionError,
            ValueError) as exc:
        raise ReconciliationError(f"invalid JSON: {exc}") from exc


def _canonical(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      allow_nan=False)


def result_digest(result) -> str:
    """Canonical digest of an original consumer Result record."""
    if isinstance(result, delivery.Result):
        value = {"task": result.task, "attempt": result.attempt,
                 "fingerprint": result.fingerprint,
                 "adapter": result.adapter, "terminal": result.terminal,
                 "exit_code": result.exit_code, "signal": result.signal,
                 "cleanup": result.cleanup,
                 "submission": result.submission,
                 "artifacts": [a.record() for a in result.artifacts],
                 "review": result.review.record() if result.review else None,
                 "adoption": result.adoption.record()
                 if result.adoption else None}
    else:
        value = result
    return hashlib.sha256(_canonical(value).encode()).hexdigest()


def _read_ref(ref, cap=None) -> bytes:
    if cap is not None and ref.bytes > cap:
        raise ReconciliationError(f"{ref.path}: JSON size limit exceeded")
    fd = os.open(ref.path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size != ref.bytes:
            raise ReconciliationError(f"{ref.path}: pinned size/type mismatch")
        digest, chunks = hashlib.sha256(), []
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            digest.update(chunk)
            chunks.append(chunk)
        if digest.hexdigest() != ref.sha256:
            raise ReconciliationError(f"{ref.path}: pinned bytes changed")
        return b"".join(chunks)
    finally:
        os.close(fd)


def _exact(value, keys, at):
    if type(value) is not dict or set(value) != set(keys):
        raise ReconciliationError(f"{at}: expected exactly {keys}")
    return value


def parse_reconciliations_bytes(raw: bytes) -> tuple[dict, ...]:
    """Strictly parse reconciliations from bytes read once by the caller.

    Byte entrypoint matching delivery.parse_plan_bytes: the returned
    records are derived from ``raw`` itself, never from a second
    pathname read.
    """
    doc = _json(raw)
    _exact(doc, ("schema", "records"), "reconciliations")
    if doc["schema"] != "doctorwatson.reconciliations/1":
        raise ReconciliationError("reconciliations schema mismatch")
    records = doc["records"]
    if type(records) is not list or len(records) > 4096:
        raise ReconciliationError("records: bounded array required")
    seen = set()
    for rec in records:
        _exact(rec, RECORD_KEYS, "reconciliation")
        for field in ("task", "attempt", "adapter"):
            if type(rec[field]) is not str or not rec[field]:
                raise ReconciliationError(f"{field}: nonempty string required")
        if rec["adapter"] not in delivery.ROLES:
            raise ReconciliationError("unknown adapter")
        if type(rec["fingerprint"]) is not str \
                or not HEX.fullmatch(rec["fingerprint"]):
            raise ReconciliationError("fingerprint malformed")
        if rec["schema"] != "doctorwatson.reconciliation/1":
            raise ReconciliationError("reconciliation schema mismatch")
        if rec["resolution"] not in EVIDENCE_ROLES:
            raise ReconciliationError("unknown resolution")
        if type(rec["original_result_sha256"]) is not str \
                or not HEX.fullmatch(rec["original_result_sha256"]):
            raise ReconciliationError("original_result_sha256 malformed")
        key = (rec["task"], rec["attempt"], rec["original_result_sha256"])
        if key in seen:
            raise ReconciliationError("duplicate reconciliation for attempt")
        seen.add(key)
        evidence = rec["evidence"]
        if type(evidence) is not list:
            raise ReconciliationError("evidence: array required")
        roles = []
        for item in evidence:
            _exact(item, ("role", "path", "bytes", "sha256"), "evidence")
            try:
                delivery._ref(item, "evidence", role=True)
            except delivery.SchemaError as exc:
                raise ReconciliationError(str(exc)) from exc
            roles.append(item["role"])
        if tuple(sorted(roles)) != tuple(sorted(
                EVIDENCE_ROLES[rec["resolution"]])):
            raise ReconciliationError(
                "evidence roles != fixed roles for resolution")
    return tuple(records)


def load_reconciliations(path) -> tuple[dict, ...]:
    with open(path, "rb") as stream:
        return parse_reconciliations_bytes(
            stream.read(MAX_JSON_BYTES + 1))


def _evidence_refs(rec) -> dict:
    out = {}
    for item in rec["evidence"]:
        try:
            ref = delivery._ref(item, "evidence", role=True)
        except delivery.SchemaError as exc:
            raise ReconciliationError(str(exc)) from exc
        out[item["role"]] = ref
    return out


def _process_consistency(orig, value):
    """Bind process evidence to the original's recorded process facts.

    The whole-Result digest already binds the subject; this binds the
    *claim* to that subject. ``not-started`` is consistent only with
    actual no-start subjects (unknown-submission or refused terminal,
    no recorded exit/signal, no recorded tx inclusion); a terminal
    that proves execution (succeeded/failed/timed-out/cancelled) or
    any recorded exit_code/signal forbids it -- a known-ran original
    cannot be relabelled not-started to free a same-fingerprint
    retry. ``exited`` evidence must equal any recorded exit_code and
    signal exactly, cannot discharge a refused (never dispatched)
    original, and a succeeded original requires exit 0 without
    signal. Inconsistency raises, so it is visible as invalid.
    """
    outcome = value["process_outcome"]
    if outcome == "not-started":
        if orig.terminal not in ("unknown-submission", "refused") \
                or orig.exit_code is not None \
                or orig.signal is not None \
                or orig.submission in ("included", "rejected"):
            raise ReconciliationError(
                "not-started evidence contradicts recorded original "
                f"terminal/exit/signal/submission (terminal="
                f"{orig.terminal}, exit_code={orig.exit_code}, "
                f"signal={orig.signal}, submission={orig.submission})")
    else:  # exited
        if orig.terminal == "refused":
            raise ReconciliationError(
                "exited evidence contradicts refused original "
                "(never dispatched)")
        if orig.exit_code is not None \
                and value["exit_code"] != orig.exit_code:
            raise ReconciliationError(
                "exited evidence exit_code contradicts recorded "
                f"original exit_code {orig.exit_code}")
        if orig.signal is not None \
                and value["signal"] != orig.signal:
            raise ReconciliationError(
                "exited evidence signal contradicts recorded "
                f"original signal {orig.signal}")
        if orig.terminal == "succeeded" \
                and (value["exit_code"] != 0
                     or value["signal"] is not None):
            raise ReconciliationError(
                "exited evidence contradicts succeeded original "
                "(requires exit 0 without signal)")


def _original_unresolved(plan, orig, obligations):
    """Whether one original is unknown/debt, by the consumer derivation.

    ``obligations`` (from assess_reconciled) maps (task, attempt) to
    the delivery.attempt_obligations pair computed with the exact
    currentness assess used. Standalone callers get the same
    derivation with a proxy currentness (current only when attempt,
    fingerprint and adapter all match the current plan task); the
    fold always passes the exact map.
    """
    if obligations is not None:
        pair = obligations.get((orig.task, orig.attempt))
        if pair is not None:
            return pair[0] or pair[1]
    task = plan.by_id.get(orig.task)
    if task is None:
        return orig.submission == "unknown" \
            or orig.terminal == "unknown-submission" \
            or orig.cleanup != "complete"
    currentness = "current" \
        if orig.attempt == task.attempt \
        and orig.fingerprint == delivery.fingerprint(plan, orig.task) \
        and orig.adapter == task.adapter else "stale-attempt"
    unknown, debt = delivery.attempt_obligations(
        task, orig, currentness)
    return unknown or debt


def validate_reconciliation(rec, plan, results,
                            obligations=None) -> dict | None:
    """Return an applicable resolution summary, or None if not applicable.

    Applicable means: an original result exists with the exact
    task/attempt/fingerprint/adapter and canonical digest; that original
    is unknown or cleanup-debt under the consumer's own
    delivery.attempt_obligations derivation (record cleanup string or
    actual resource-receipt artifact debt); and the fixed evidence
    decodes for that same operation and is consistent with the
    original's recorded terminal/exit/signal/submission. Invalid
    evidence raises ReconciliationError so a malformed record is
    visible, never silently treated as a pass.
    """
    originals = [r for r in results if r.task == rec["task"]
                 and r.attempt == rec["attempt"]]
    if len(originals) != 1:
        return None
    orig = originals[0]
    if orig.fingerprint != rec["fingerprint"] \
            or orig.adapter != rec["adapter"] \
            or result_digest(orig) != rec["original_result_sha256"]:
        return None
    if not _original_unresolved(plan, orig, obligations):
        return None
    unknown_outcome = orig.submission == "unknown" \
        or orig.terminal == "unknown-submission"
    if unknown_outcome and orig.adapter == "sherlock-anchor-settle-readback" \
            and rec["resolution"] in ("process-exited", "cleanup-complete"):
        return None  # local process exit does not establish tx outcome
    refs = _evidence_refs(rec)
    if rec["resolution"] in ("process-not-started", "process-exited",
                             "cleanup-complete"):
        value = _json(_read_ref(refs["process-evidence"],
                                cap=MAX_JSON_BYTES))
        _exact(value, ("schema", "task", "attempt", "fingerprint",
                       "preexec_sha256", "original_result_sha256",
                       "process_outcome", "exit_code",
                       "signal", "cleanup", "restoration"),
               "process-evidence")
        if value["schema"] != \
                "doctorwatson.process-reconciliation-evidence/2" \
                or value["task"] != orig.task \
                or value["attempt"] != orig.attempt \
                or value["fingerprint"] != orig.fingerprint \
                or type(value["preexec_sha256"]) is not str \
                or not HEX.fullmatch(value["preexec_sha256"]):
            raise ReconciliationError("process evidence binding mismatch")
        if value["original_result_sha256"] != result_digest(orig) \
                or value["original_result_sha256"] != \
                rec["original_result_sha256"]:
            raise ReconciliationError(
                "process evidence original-result token mismatch "
                "(evidence does not validate the whole original subject)")
        if value["cleanup"] != "complete" \
                or value["restoration"] != "complete":
            raise ReconciliationError("process evidence retains cleanup debt")
        want = {"process-not-started": "not-started",
                "process-exited": "exited",
                "cleanup-complete": "exited"}[rec["resolution"]]
        if value["process_outcome"] != want:
            raise ReconciliationError("process outcome != resolution")
        if want == "not-started" and (value["exit_code"] is not None
                                      or value["signal"] is not None):
            raise ReconciliationError("not-started with exit/signal")
        if want == "exited":
            ec, sg = value["exit_code"], value["signal"]
            if not ((type(ec) is int and 0 <= ec <= 255 and sg is None)
                    or (ec is None and type(sg) is int and 1 <= sg <= 128)):
                raise ReconciliationError("exited outcome needs exit/signal")
        _process_consistency(orig, value)
    else:  # tx-included
        if orig.adapter != "sherlock-anchor-settle-readback":
            return None
        task = plan.by_id.get(orig.task)
        if task is None:
            return None
        post_raw = _read_ref(refs["post-state"])
        settle_observation.decode_sherlock_settle_result(
            _read_ref(refs["source-result"]), task.inputs,
            post_state_sha256=hashlib.sha256(post_raw).hexdigest())
        cleanup = _json(_read_ref(refs["resource-receipt"],
                                  cap=MAX_JSON_BYTES))
        _exact(cleanup, ("schema", "task", "attempt", "fingerprint",
                         "cleanup", "restoration"), "cleanup receipt")
        if cleanup["schema"] != "doctorwatson.cleanup-observation/1" \
                or cleanup["task"] != orig.task \
                or cleanup["attempt"] != orig.attempt \
                or cleanup["fingerprint"] != orig.fingerprint \
                or cleanup["cleanup"] != "complete" \
                or cleanup["restoration"] != "complete":
            raise ReconciliationError("tx reconciliation cleanup invalid")
    return {"task": orig.task, "attempt": orig.attempt,
            "fingerprint": orig.fingerprint, "adapter": orig.adapter,
            "resolution": rec["resolution"],
            "original_result_sha256": rec["original_result_sha256"]}


UNKNOWN_BLOCKER = (
    "historical submission outcome unresolved; "
    "reconcile original attempt")
DEBT_BLOCKER = (
    "historical cleanup/restoration debt remains unresolved")


def assess_reconciled(plan, results, reconciliations):
    """delivery.assess with applicable reconciliations folded in.

    History rows are preserved verbatim; folded resolutions are added
    under ``reconciliations``. Per-original obligations come from the
    consumer's own delivery.attempt_obligations (record cleanup string
    and actual resource-receipt artifact validity/status/source/
    membership), keyed by attempt with the currentness assess itself
    recorded. A historical blocker kind is rewritten only from the
    obligations that remain after crediting valid per-attempt
    records: one attempt's discharge never lifts a sibling attempt's
    unknown or debt, and fully discharging every contributing attempt
    is required before promotion. Current-attempt unknowns are not
    rewritten into successes.
    """
    reads = delivery._AssessmentReads()
    report = delivery._assess(plan, results, reads)
    obligations = {}
    for row in report["tasks"]:
        task = plan.by_id[row["id"]]
        currentness = {h["attempt"]: h["currentness"]
                       for h in row["history"]}
        for result in results:
            if result.task == row["id"]:
                obligations[(result.task, result.attempt)] = \
                    delivery.attempt_obligations(
                        task, result,
                        currentness.get(result.attempt, "stale-attempt"),
                        _reader=reads.read)
    applicable = {}
    for rec in reconciliations:
        try:
            summary = validate_reconciliation(
                rec, plan, results, obligations=obligations)
        except (OSError, ReconciliationError,
                settle_observation.SettleDecodeError) as exc:
            summary = {"task": rec["task"], "attempt": rec["attempt"],
                       "resolution": rec["resolution"],
                       "invalid": str(exc)}
        if summary is not None:
            applicable.setdefault(rec["task"], []).append(summary)
    ready = []
    for row in report["tasks"]:
        entries = applicable.get(row["id"], [])
        row["reconciliations"] = entries
        valid = [e for e in entries if "invalid" not in e]
        if row["state"] in ("unknown-reconcile", "cleanup-debt") and valid:
            task = plan.by_id[row["id"]]
            per_attempt = {
                attempt: pair for (name, attempt), pair
                in obligations.items() if name == row["id"]}
            unresolved_attempts = {
                a for a, (u, d) in per_attempt.items() if u or d}
            resolved_attempts = {e["attempt"] for e in valid}
            remaining = {
                a: per_attempt[a] for a in unresolved_attempts
                if a not in resolved_attempts}
            remaining_unknown = any(u for u, _d in remaining.values())
            remaining_debt = any(d for _u, d in remaining.values())
            current_unresolved = row["attempt"] in unresolved_attempts
            base_blockers = [
                b for b in row["blockers"]
                if b not in (UNKNOWN_BLOCKER, DEBT_BLOCKER)]
            if remaining:
                # Credit only the discharged attempts; restate exactly
                # the obligation kinds still owed by named siblings.
                row["blockers"] = base_blockers + (
                    [UNKNOWN_BLOCKER] if remaining_unknown else []) + (
                    [DEBT_BLOCKER] if remaining_debt else [])
                row["state"] = "unknown-reconcile" \
                    if remaining_unknown else "cleanup-debt"
                row["selection_candidate"] = False
            elif not current_unresolved \
                    and task.disposition == "active" \
                    and task.launch_state == "not-started":
                consumed_same_fp = [
                    e for e in valid
                    if e.get("fingerprint") == row["fingerprint"]
                    and e["resolution"] != "process-not-started"]
                if consumed_same_fp:
                    # Advisory only: an unchanged retry after a discharged
                    # original is allowed and logged, never suppressed.
                    row.setdefault("warnings", []).append(
                        "unchanged retry of a discharged original "
                        "(advisory; original attempt discharged as "
                        + ",".join(sorted({e["resolution"]
                                            for e in consumed_same_fp}))
                        + ")")
                if not base_blockers:
                    row["blockers"] = base_blockers
                    row["state"] = "ready"
                    row["selection_candidate"] = True
                else:
                    row["blockers"] = base_blockers
        if row["selection_candidate"]:
            ready.append(row["id"])
    report["ready_for_root_selection"] = ready
    return delivery.apply_capabilities(report)


def _read_pinned(path, what):
    """Read one readiness input ONCE; return (pin, exact raw bytes).

    The pin's sha256 is over the returned bytes, so a caller that
    parses only those bytes assesses exactly what it hashed. The
    recorded path is the canonical absolute path of the input as
    opened, so identity provenance is exact across equivalent
    spellings (relative vs absolute) of the same file.
    """
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size > MAX_JSON_BYTES:
            raise ReconciliationError(f"{what}: not a bounded regular file")
        chunks, total = [], 0
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            total += len(chunk)
            if total > MAX_JSON_BYTES:
                raise ReconciliationError(f"{what}: size limit exceeded")
            chunks.append(chunk)
        after = os.fstat(fd)
        stamp = lambda st: (st.st_dev, st.st_ino, st.st_size,
                             st.st_mtime_ns, st.st_ctime_ns)
        if stamp(before) != stamp(after):
            raise ReconciliationError(f"{what}: changed while pinning")
        raw = b"".join(chunks)
        return {"path": str(Path(path).resolve()), "bytes": total,
                "sha256": hashlib.sha256(raw).hexdigest()}, raw
    finally:
        os.close(fd)


def _pin_raw(path, what) -> dict:
    """Digest-pin one readiness input file's exact raw bytes."""
    return _read_pinned(path, what)[0]


def load_readiness(plan_path, history_path=None, reconciliations_path=None):
    """ONE common readiness path for inspection and executor prelaunch.

    Each readiness input is read ONCE through the pinning reader and
    the pinned bytes themselves are strictly parsed
    (delivery.parse_plan_bytes / parse_history_bytes /
    parse_reconciliations_bytes), so the report, plan and fingerprints
    are derived from exactly the hashed bytes stamped in
    ``readiness_identity`` -- never from transient bytes swapped in
    and restored during a separate pathname parse. A post-read pin
    recheck is retained and refuses on any change while loading. The
    executor re-runs this at actual use and compares identities;
    reconciliations without the raw history they reference are
    refused. With no reconciliations file this is the ordinary assess
    over the same parsed plan/results. History is an optional input:
    with none supplied, readiness is assessed over zero results, so
    no durable replay barrier is claimed without a supplied complete
    raw history.
    """
    if reconciliations_path is not None and history_path is None:
        raise ReconciliationError(
            "reconciliations require the raw history they reference")
    plan_pin, plan_raw = _read_pinned(plan_path, "plan")
    history_pin, history_raw = _read_pinned(history_path, "history") \
        if history_path is not None else (None, None)
    recs_pin, recs_raw = _read_pinned(reconciliations_path,
                                      "reconciliations") \
        if reconciliations_path is not None else (None, None)
    pins = {"plan": plan_pin, "history": history_pin,
            "reconciliations": recs_pin}
    plan = delivery.parse_plan_bytes(plan_raw)
    results = delivery.parse_history_bytes(history_raw) \
        if history_path is not None else ()
    recs = parse_reconciliations_bytes(recs_raw) \
        if reconciliations_path is not None else ()
    after = {"plan": _pin_raw(plan_path, "plan"),
             "history": _pin_raw(history_path, "history")
             if history_path is not None else None,
             "reconciliations": _pin_raw(reconciliations_path,
                                          "reconciliations")
             if reconciliations_path is not None else None}
    if after != pins:
        raise ReconciliationError(
            "readiness input bytes changed while loading; refusing")
    report = assess_reconciled(plan, results, recs)
    report["readiness_identity"] = pins
    return {"plan": plan, "results": results, "reconciliations": recs,
            "report": report, "identity": pins}


def inspect_reconciled(plan_path, history_path, reconciliations_path):
    return load_readiness(plan_path, history_path,
                          reconciliations_path)["report"]


def main(argv=None):
    """Read-only CLI: --plan P --results H [--reconciliations R] [--next].

    This is the shell --inspect route; it consumes load_readiness, the
    same path the executor rechecks before launch.
    """
    import argparse
    import sys
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plan", required=True)
    parser.add_argument("--results", required=True)
    parser.add_argument("--reconciliations", required=False, default=None)
    parser.add_argument("--next", action="store_true")
    args = parser.parse_args(argv)
    try:
        report = load_readiness(args.plan, args.results,
                                args.reconciliations)["report"]
    except (OSError, ReconciliationError, delivery.SchemaError) as exc:
        print(json.dumps({"schema": "doctorwatson.delivery-error/1",
                          "records_valid": False,
                          "execution_authority": False,
                          "error": str(exc)}), file=sys.stderr)
        return 2
    if args.next:
        report = {k: report[k] for k in
                  ("schema", "records_valid", "execution_authority",
                   "producer_authenticity", "ready_for_root_selection",
                   "ready_for_dispatch", "dispatch_preflight",
                   "dispatch_readiness_scope", "readiness_identity")}
    print(json.dumps(report, sort_keys=True, indent=2, allow_nan=False))
    return 0  # inspectable records only; never execution approval


if __name__ == "__main__":
    raise SystemExit(main())
