"""Isolated live-profile policy and review-history regression tests.

Exercises public ``assess`` / store / CLI APIs against tempdir SQLite only.
Does not write a live git common store, budget.lock, manifests, wallets,
or grant live admission. ``assess`` never grants admission.
"""
from __future__ import annotations

import contextlib
import io
import json
import sys
import tempfile
import threading
import unittest
import uuid
from pathlib import Path
import os
import shutil
import subprocess

SCRIPTS_DIR = Path(__file__).resolve().parent.parent / "scripts"
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))

from moriarty_dev.cli import catalog_live_lineage, cmd_review, cmd_run, cmd_status
from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot
from moriarty_dev.store import (
    ReservationConflictError,
    get_db_path,
    get_history,
    get_live_history,
    has_other_primary,
    init_db,
    make_guarded_history_reader,
    record_event,
    record_review,
    read_store_identity,
    reserve,
    finish,
    SCHEMA_SQL,
)

LIVE_PROFILE = "finalized-financial-settlement"
NONFINANCIAL_PROFILE = "local-runtime"
LIVE_HISTORY_KINDS = (
    "verify",
    "review",
    "reproduce",
    "implement",
    "repair",
    "admin",
    "report",
)
LIVE_PRIMARY_KINDS = LIVE_HISTORY_KINDS
LIVE_RETRY_DENIED_KINDS = (
    "verify",
    "review",
    "reproduce",
    "implement",
    "admin",
    "report",
)
REQ = "SP05"
CAP = "loan-executor"
CANDIDATE_HASH = "a" * 64


def live_action(kind, *, profile=LIVE_PROFILE, **overrides):
    body = {
        "id": "live-loan-" + kind,
        "requirement": REQ,
        "capability": CAP,
        "kind": kind,
        "candidate": "diagnosis-only",
        "admissionRef": "campaign:diagnosis-only",
        "commandRef": "diagnosis-only#no-command",
        "evidenceProfile": profile,
    }
    body.update(overrides)
    return body


def snapshot(**overrides):
    body = {
        "authorityCurrent": True,
        "entryEligible": True,
        "candidateCurrent": True,
        "resourceAdmitted": True,
        "sameDefectFailures": 0,
        "adminCycles": 0,
        "adminSeconds": 0,
        "primaryActive": False,
        "reproducerVerified": False,
        "approachChanged": False,
        "nextActionId": None,
        "missingEvidence": [],
    }
    body.update(overrides)
    return body


def reproducer_snapshot():
    """Exact pure-assess loophole inputs from the retained live-kind reproducer."""
    return snapshot(
        sameDefectFailures=2,
        primaryActive=True,
        reproducerVerified=False,
        approachChanged=False,
        nextActionId=None,
        missingEvidence=["operational-history"],
    )


def assert_allowed(test, result, *, context=""):
    test.assertTrue(result["allow"], context or result)
    test.assertEqual(result["reasonCode"], "ALLOWED", context or result)


def assert_denied(test, result, reason, *, context=""):
    test.assertFalse(result["allow"], context or result)
    test.assertEqual(result["reasonCode"], reason, context or result)


class LiveFinancialAssessGates(unittest.TestCase):
    def test_fresh_live_actions_remain_permitted_when_gates_are_clear(self):
        s = snapshot()
        for kind in (
            "verify",
            "review",
            "reproduce",
            "implement",
            "repair",
            "admin",
            "report",
        ):
            assert_allowed(self, assess(s, live_action(kind)), context=kind)

    def test_live_report_obeys_history_retry_and_primary(self):
        s = reproducer_snapshot()
        result = assess(s, live_action("report"))
        assert_denied(self, result, "HISTORY_UNRESOLVED")
        assert_denied(
            self, assess(snapshot(primaryActive=True), live_action("report")), "PRIMARY_ACTIVE"
        )
        assert_denied(
            self,
            assess(snapshot(sameDefectFailures=2), live_action("report")),
            "REPRODUCE_BEFORE_RETRY",
        )

    def test_history_gate_denies_each_live_dispatch_kind_independently(self):
        s = snapshot(missingEvidence=["operational-history"])
        for kind in LIVE_HISTORY_KINDS:
            result = assess(s, live_action(kind))
            assert_denied(self, result, "HISTORY_UNRESOLVED", context=kind)
            self.assertEqual(result["missingEvidence"], ["operational-history"])

    def test_history_prefix_marker_is_also_a_live_history_gap(self):
        s = snapshot(missingEvidence=["operational-history:unverified"])
        result = assess(s, live_action("verify"))
        assert_denied(self, result, "HISTORY_UNRESOLVED")

    def test_primary_gate_denies_each_live_dispatch_kind_independently(self):
        s = snapshot(primaryActive=True)
        for kind in LIVE_PRIMARY_KINDS:
            assert_denied(
                self,
                assess(s, live_action(kind)),
                "PRIMARY_ACTIVE",
                context=kind,
            )

    def test_retry_gate_denies_live_nonrepair_kinds_like_implement(self):
        s = snapshot(sameDefectFailures=2, nextActionId="live-loan-reproduce")
        for kind in LIVE_RETRY_DENIED_KINDS:
            result = assess(s, live_action(kind))
            assert_denied(self, result, "REPRODUCE_BEFORE_RETRY", context=kind)
            self.assertEqual(result["nextActionId"], "live-loan-reproduce")

    def test_live_reproduce_denied_at_two_failures_even_when_ready(self):
        s = snapshot(
            sameDefectFailures=2,
            reproducerVerified=True,
            approachChanged=True,
        )
        assert_denied(
            self, assess(s, live_action("reproduce")), "REPRODUCE_BEFORE_RETRY"
        )
        assert_denied(self, assess(s, live_action("verify")), "REPRODUCE_BEFORE_RETRY")
        assert_denied(self, assess(s, live_action("report")), "REPRODUCE_BEFORE_RETRY")
        assert_allowed(self, assess(s, live_action("repair")))

    def test_live_repair_still_requires_reproducer_and_changed_approach(self):
        s = snapshot(sameDefectFailures=2)
        assert_denied(self, assess(s, live_action("repair")), "REPAIR_UNREADY")
        s["reproducerVerified"] = True
        s["approachChanged"] = True
        assert_allowed(self, assess(s, live_action("repair")))
        assert_denied(
            self, assess(s, live_action("implement")), "REPRODUCE_BEFORE_RETRY"
        )
        assert_denied(
            self, assess(s, live_action("verify")), "REPRODUCE_BEFORE_RETRY"
        )
        assert_denied(
            self, assess(s, live_action("reproduce")), "REPRODUCE_BEFORE_RETRY"
        )

    def test_combined_reproducer_snapshot_denies_live_kinds_like_implement(self):
        s = reproducer_snapshot()
        implement = assess(s, live_action("implement"))
        assert_denied(self, implement, "HISTORY_UNRESOLVED")
        for kind in ("verify", "review", "reproduce", "report"):
            result = assess(s, live_action(kind))
            assert_denied(self, result, "HISTORY_UNRESOLVED", context=kind)

    def test_profile_discriminates_history_exemption(self):
        s = snapshot(missingEvidence=["operational-history"])
        assert_allowed(
            self, assess(s, live_action("verify", profile=NONFINANCIAL_PROFILE))
        )
        assert_denied(
            self, assess(s, live_action("verify")), "HISTORY_UNRESOLVED"
        )

    def test_profile_discriminates_primary_exemption(self):
        s = snapshot(primaryActive=True)
        assert_allowed(
            self, assess(s, live_action("review", profile=NONFINANCIAL_PROFILE))
        )
        assert_denied(self, assess(s, live_action("review")), "PRIMARY_ACTIVE")

    def test_profile_discriminates_retry_exemption_for_verify(self):
        s = snapshot(sameDefectFailures=2)
        assert_allowed(
            self, assess(s, live_action("verify", profile=NONFINANCIAL_PROFILE))
        )
        assert_denied(
            self, assess(s, live_action("verify")), "REPRODUCE_BEFORE_RETRY"
        )

    def test_live_verify_still_fail_closed_on_earlier_gates(self):
        resource = snapshot(
            resourceAdmitted=False, missingEvidence=["operational-history"]
        )
        assert_denied(
            self,
            assess(resource, live_action("verify")),
            "RESOURCE_EXHAUSTED",
        )
        admission = snapshot(
            missingEvidence=["admissionRef-unresolved:campaign:missing"]
        )
        assert_denied(
            self,
            assess(admission, live_action("reproduce")),
            "ADMISSION_UNRESOLVED",
        )
        stale = snapshot(candidateCurrent=False, primaryActive=True)
        assert_denied(
            self, assess(stale, live_action("review")), "EVIDENCE_STALE"
        )


class NonfinancialAssessControls(unittest.TestCase):
    def test_nonfinancial_verify_review_reproduce_keep_historical_exemptions(self):
        blocked = snapshot(
            sameDefectFailures=2,
            primaryActive=True,
            missingEvidence=["operational-history"],
        )
        for kind in ("verify", "review", "reproduce"):
            assert_allowed(
                self,
                assess(
                    blocked, live_action(kind, profile=NONFINANCIAL_PROFILE)
                ),
                context=kind,
            )
        assert_allowed(
            self,
            assess(blocked, live_action("report", profile=NONFINANCIAL_PROFILE)),
        )

    def test_nonfinancial_reproduce_keeps_retry_exemption(self):
        s = snapshot(sameDefectFailures=2)
        assert_allowed(
            self, assess(s, live_action("reproduce", profile=NONFINANCIAL_PROFILE))
        )

    def test_nonfinancial_implement_still_hits_history_first(self):
        s = snapshot(missingEvidence=["operational-history"])
        assert_denied(
            self,
            assess(s, live_action("implement", profile=NONFINANCIAL_PROFILE)),
            "HISTORY_UNRESOLVED",
        )

    def test_syntax_profile_verify_keeps_nonfinancial_history_exemption(self):
        s = snapshot(missingEvidence=["operational-history"], primaryActive=True)
        assert_allowed(
            self, assess(s, live_action("verify", profile="syntax"))
        )


class IsolatedTempStore(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name) / "isolated-repo"
        self.root.mkdir()
        # No git metadata: get_db_path must use the in-repo fallback, never a
        # live worktree common directory.
        self.db_path = get_db_path(self.root)
        self.assertEqual(
            self.db_path, self.root / ".moriarty-dev" / "state.sqlite3"
        )
        self.assertFalse((self.root / ".git").exists())
        init_db(self.db_path).close()

    def tearDown(self):
        self.temp.cleanup()

    def history(self):
        return get_history(self.db_path, str(self.root), REQ, CAP)

    def seed_findings(self, *finding_ids):
        for index, finding_id in enumerate(finding_ids):
            record_event(
                self.db_path,
                str(self.root),
                REQ,
                CAP,
                CANDIDATE_HASH,
                "seed-" + str(index),
                "defect_failure",
                {
                    "findingId": finding_id,
                    "defectClass": "retained-loan-executor-defect",
                },
            )

    def approved(self, scope, resolved=None):
        receipt = {
            "author": "grok-4.6",
            "reviewer": "claude-opus-5",
            "candidateHash": CANDIDATE_HASH,
            "scope": scope,
            "verdict": "APPROVED",
            "requirement": REQ,
            "capability": CAP,
        }
        if resolved is not None:
            receipt["resolvedFindings"] = list(resolved)
        return receipt

    def reserve_kind(self, kind, *, profile=LIVE_PROFILE, action_id=None):
        action = live_action(kind, profile=profile)
        if action_id is not None:
            action["id"] = action_id
        return reserve(self.db_path, str(self.root), action, snapshot())


class LivePrimaryReservation(IsolatedTempStore):
    def test_store_path_is_isolated_and_creates_no_budget_lock_or_claims(self):
        self.assertTrue(str(self.db_path).startswith(str(self.root)))
        self.assertFalse(any(self.root.rglob("budget.lock")))
        conn = init_db(self.db_path)
        try:
            claims = conn.execute("SELECT count(*) FROM runner_claims").fetchone()[0]
        finally:
            conn.close()
        self.assertEqual(claims, 0)

    def test_active_live_verify_occupies_primary_for_readers(self):
        res_id = self.reserve_kind("verify")
        history = self.history()
        self.assertIsNotNone(history)
        self.assertTrue(history["primaryActive"])
        self.assertTrue(has_other_primary(self.db_path, "not-" + res_id))
        self.assertFalse(has_other_primary(self.db_path, res_id))
        finish(self.db_path, res_id, {"exitCode": 0})
        history = self.history()
        self.assertIsNotNone(history)
        self.assertFalse(history["primaryActive"])

    def test_live_verify_conflicts_with_active_implement(self):
        impl_id = self.reserve_kind("implement")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("verify")
        finally:
            finish(self.db_path, impl_id, {"exitCode": 0})

    def test_implement_conflicts_with_active_live_verify(self):
        verify_id = self.reserve_kind("verify")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("implement")
        finally:
            finish(self.db_path, verify_id, {"exitCode": 0})

    def test_live_review_conflicts_with_active_implement(self):
        impl_id = self.reserve_kind("implement", action_id="live-loan-implement-review")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("review")
        finally:
            finish(self.db_path, impl_id, {"exitCode": 0})

    def test_live_reproduce_conflicts_with_active_implement(self):
        impl_id = self.reserve_kind(
            "implement", action_id="live-loan-implement-reproduce"
        )
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("reproduce")
        finally:
            finish(self.db_path, impl_id, {"exitCode": 0})

    def test_live_report_conflicts_with_active_implement(self):
        impl_id = self.reserve_kind("implement", action_id="live-loan-implement-report")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("report")
        finally:
            finish(self.db_path, impl_id, {"exitCode": 0})

    def test_implement_conflicts_with_active_live_report(self):
        report_id = self.reserve_kind("report")
        try:
            history = self.history()
            self.assertTrue(history["primaryActive"])
            self.assertTrue(has_other_primary(self.db_path, "not-" + report_id))
            self.assertFalse(has_other_primary(self.db_path, report_id))
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("implement")
        finally:
            finish(self.db_path, report_id, {"exitCode": 0})

    def test_second_live_verify_conflicts_like_primary(self):
        first = self.reserve_kind("verify")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("verify", action_id="live-loan-verify-other")
        finally:
            finish(self.db_path, first, {"exitCode": 0})

    def test_live_verify_conflicts_with_active_repair(self):
        repair_id = self.reserve_kind("repair")
        try:
            with self.assertRaises(ReservationConflictError):
                self.reserve_kind("verify")
        finally:
            finish(self.db_path, repair_id, {"exitCode": 0})

    def test_nonfinancial_verify_does_not_occupy_or_conflict_with_primary(self):
        verify_id = self.reserve_kind("verify", profile=NONFINANCIAL_PROFILE)
        history = self.history()
        self.assertIsNotNone(history)
        self.assertFalse(history["primaryActive"])
        self.assertFalse(has_other_primary(self.db_path, "unrelated"))
        impl_id = self.reserve_kind(
            "implement",
            profile=NONFINANCIAL_PROFILE,
            action_id="local-implement",
        )
        history = self.history()
        self.assertTrue(history["primaryActive"])
        finish(self.db_path, impl_id, {"exitCode": 0})
        finish(self.db_path, verify_id, {"exitCode": 0})

    def test_nonfinancial_report_reservation_does_not_occupy_primary(self):
        report_id = self.reserve_kind("report", profile=NONFINANCIAL_PROFILE)
        history = self.history()
        self.assertIsNotNone(history)
        self.assertFalse(history["primaryActive"])
        impl_id = self.reserve_kind("implement")
        finish(self.db_path, impl_id, {"exitCode": 0})
        finish(self.db_path, report_id, {"exitCode": 0})

    def test_reserve_without_charge_id_does_not_write_runner_claims(self):
        res_id = self.reserve_kind("verify")
        conn = init_db(self.db_path)
        try:
            claims = conn.execute("SELECT count(*) FROM runner_claims").fetchone()[0]
        finally:
            conn.close()
        self.assertEqual(claims, 0)
        finish(self.db_path, res_id, {"exitCode": 0})

    def test_unknown_active_profile_occupies_primary(self):
        conn = init_db(self.db_path)
        now = "2026-09-11T00:00:00+00:00"
        try:
            with conn:
                conn.execute(
                    """
                    INSERT INTO reservations (
                        id, repository, requirement, capability, candidate,
                        action_id, action_kind, pid, status, reserved_at
                    ) VALUES (?, ?, ?, ?, ?, ?, 'verify', 1, 'active', ?)
                    """,
                    (
                        "unknown-res",
                        str(self.root),
                        REQ,
                        CAP,
                        CANDIDATE_HASH,
                        "unknown-verify",
                        now,
                    ),
                )
                conn.execute(
                    """
                    INSERT INTO events (
                        repository, requirement, capability, candidate,
                        action_id, event_kind, observed_at, payload_json
                    ) VALUES (?, ?, ?, ?, ?, 'action_start', ?, ?)
                    """,
                    (
                        str(self.root),
                        REQ,
                        CAP,
                        CANDIDATE_HASH,
                        "unknown-verify",
                        now,
                        "{}",
                    ),
                )
        finally:
            conn.close()
        history = self.history()
        self.assertIsNotNone(history)
        self.assertTrue(history["primaryActive"])
        self.assertTrue(has_other_primary(self.db_path, "other"))

    def test_concurrent_live_verify_only_one_primary(self):
        winners = []
        conflicts = []

        def worker(action_id):
            try:
                winners.append(self.reserve_kind("verify", action_id=action_id))
            except ReservationConflictError:
                conflicts.append(action_id)

        t1 = threading.Thread(target=worker, args=("live-loan-verify-a",))
        t2 = threading.Thread(target=worker, args=("live-loan-verify-b",))
        t1.start()
        t2.start()
        t1.join()
        t2.join()
        self.assertEqual(len(winners), 1)
        self.assertEqual(len(conflicts), 1)
        self.assertTrue(self.history()["primaryActive"])
        self.assertFalse(has_other_primary(self.db_path, winners[0]))
        finish(self.db_path, winners[0], {"exitCode": 0})

    def test_malformed_profiles_occupy_all_three_consumers(self):
        for profile in (False, 7, "", "unknown-profile"):
            action = live_action("verify", profile=profile)
            action["id"] = "malformed-" + repr(profile)
            res_id = reserve(self.db_path, str(self.root), action, snapshot())
            try:
                history = self.history()
                self.assertTrue(history["primaryActive"], profile)
                self.assertTrue(has_other_primary(self.db_path, "other"), profile)
                with self.assertRaises(ReservationConflictError):
                    self.reserve_kind(
                        "verify", action_id="second-after-" + repr(profile)
                    )
            finally:
                finish(self.db_path, res_id, {"exitCode": 0})

    def test_syntax_and_local_runtime_remain_non_occupying_controls(self):
        for profile in ("syntax", NONFINANCIAL_PROFILE):
            res_id = self.reserve_kind(
                "verify", profile=profile, action_id="nonlive-" + profile
            )
            try:
                history = self.history()
                self.assertFalse(history["primaryActive"], profile)
                self.assertFalse(has_other_primary(self.db_path, "other"), profile)
                live_id = self.reserve_kind(
                    "verify", action_id="live-after-" + profile
                )
                finish(self.db_path, live_id, {"exitCode": 0})
            finally:
                finish(self.db_path, res_id, {"exitCode": 0})

    def test_live_kind_then_implement_and_repair_conflict(self):
        for kind in LIVE_HISTORY_KINDS:
            first = self.reserve_kind(kind, action_id="first-" + kind)
            try:
                with self.assertRaises(ReservationConflictError):
                    self.reserve_kind("implement", action_id="impl-after-" + kind)
                with self.assertRaises(ReservationConflictError):
                    self.reserve_kind("repair", action_id="repair-after-" + kind)
            finally:
                finish(self.db_path, first, {"exitCode": 0})

    def test_concurrent_live_verify_versus_implement(self):
        winners = []
        conflicts = []

        def worker(kind, action_id):
            try:
                winners.append(self.reserve_kind(kind, action_id=action_id))
            except ReservationConflictError:
                conflicts.append(kind)

        t1 = threading.Thread(target=worker, args=("verify", "race-verify"))
        t2 = threading.Thread(target=worker, args=("implement", "race-implement"))
        t1.start()
        t2.start()
        t1.join()
        t2.join()
        self.assertEqual(len(winners), 1)
        self.assertEqual(len(conflicts), 1)
        finish(self.db_path, winners[0], {"exitCode": 0})

    def test_absent_action_start_occupies_primary(self):
        conn = init_db(self.db_path)
        now = "2026-09-11T00:00:00+00:00"
        try:
            with conn:
                conn.execute(
                    """
                    INSERT INTO reservations (
                        id, repository, requirement, capability, candidate,
                        action_id, action_kind, pid, status, reserved_at
                    ) VALUES (?, ?, ?, ?, ?, ?, 'verify', 1, 'active', ?)
                    """,
                    (
                        "no-start-res",
                        str(self.root),
                        REQ,
                        CAP,
                        CANDIDATE_HASH,
                        "no-start-verify",
                        now,
                    ),
                )
                conn.execute(
                    """
                    INSERT INTO events (
                        repository, requirement, capability, candidate,
                        action_id, event_kind, observed_at, payload_json
                    ) VALUES (?, ?, ?, ?, ?, 'defect_failure', ?, ?)
                    """,
                    (
                        str(self.root),
                        REQ,
                        CAP,
                        CANDIDATE_HASH,
                        "no-start-verify",
                        now,
                        json.dumps({"findingId": "ABSENT-START"}),
                    ),
                )
        finally:
            conn.close()
        history = self.history()
        self.assertTrue(history["primaryActive"])
        self.assertTrue(has_other_primary(self.db_path, "other"))
        with self.assertRaises(ReservationConflictError):
            self.reserve_kind("implement", action_id="after-absent-start")

    def test_mixed_action_start_profiles_occupy_primary(self):
        conn = init_db(self.db_path)
        now = "2026-09-11T00:00:00+00:00"
        try:
            with conn:
                conn.execute(
                    """
                    INSERT INTO reservations (
                        id, repository, requirement, capability, candidate,
                        action_id, action_kind, pid, status, reserved_at
                    ) VALUES (?, ?, ?, ?, ?, ?, 'verify', 1, 'active', ?)
                    """,
                    (
                        "mixed-res",
                        str(self.root),
                        REQ,
                        CAP,
                        CANDIDATE_HASH,
                        "mixed-verify",
                        now,
                    ),
                )
                for profile in (NONFINANCIAL_PROFILE, LIVE_PROFILE):
                    conn.execute(
                        """
                        INSERT INTO events (
                            repository, requirement, capability, candidate,
                            action_id, event_kind, observed_at, payload_json
                        ) VALUES (?, ?, ?, ?, ?, 'action_start', ?, ?)
                        """,
                        (
                            str(self.root),
                            REQ,
                            CAP,
                            CANDIDATE_HASH,
                            "mixed-verify",
                            now,
                            json.dumps(
                                {
                                    "action": live_action(
                                        "verify", profile=profile
                                    ),
                                    "snapshot": {},
                                }
                            ),
                        ),
                    )
        finally:
            conn.close()
        history = self.history()
        self.assertTrue(history["primaryActive"])
        self.assertTrue(has_other_primary(self.db_path, "other"))
        with self.assertRaises(ReservationConflictError):
            self.reserve_kind("repair", action_id="after-mixed-start")


class ScopedReviewHistory(IsolatedTempStore):
    def test_source_specification_resource_approvals_leave_finding_ids(self):
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        self.assertEqual(self.history()["sameDefectFailures"], 2)
        for scope in ("source", "specification", "resource"):
            record_review(self.db_path, str(self.root), self.approved(scope))
            history = self.history()
            self.assertIsNotNone(history)
            self.assertEqual(history["sameDefectFailures"], 2, scope)

    def test_design_approval_without_named_ids_leaves_findings(self):
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        record_review(
            self.db_path,
            str(self.root),
            self.approved("RP01-MC02 specified design consistency only"),
        )
        history = self.history()
        self.assertEqual(history["sameDefectFailures"], 2)

    def test_other_lineage_named_resolution_does_not_clear_this_lineage(self):
        self.seed_findings("LIVE-FINDING-A")
        receipt = self.approved("actual-result", resolved=["LIVE-FINDING-A"])
        receipt["requirement"] = "SP01.6"
        receipt["capability"] = "loan-swap-subset"
        record_review(self.db_path, str(self.root), receipt)
        history = self.history()
        self.assertEqual(history["sameDefectFailures"], 1)

    def test_actual_result_explicit_resolution_removes_only_named_findings(self):
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        record_review(
            self.db_path,
            str(self.root),
            self.approved("actual-result", resolved=["LIVE-FINDING-A"]),
        )
        history = self.history()
        self.assertIsNotNone(history)
        self.assertEqual(history["sameDefectFailures"], 1)
        record_review(
            self.db_path,
            str(self.root),
            self.approved("actual-result", resolved=["LIVE-FINDING-B"]),
        )
        history = self.history()
        self.assertEqual(history["sameDefectFailures"], 0)

    def test_explicit_resolution_does_not_invent_missing_ids(self):
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        record_review(
            self.db_path,
            str(self.root),
            self.approved("actual-result", resolved=["LIVE-FINDING-MISSING"]),
        )
        history = self.history()
        self.assertEqual(history["sameDefectFailures"], 2)

    def test_author_defect_resolved_event_cannot_erase_findings(self):
        self.seed_findings("LIVE-FINDING-A")
        record_event(
            self.db_path,
            str(self.root),
            REQ,
            CAP,
            CANDIDATE_HASH,
            "author-claim",
            "defect_resolved",
            {"findingId": "LIVE-FINDING-A"},
        )
        history = self.history()
        self.assertIsNotNone(history)
        self.assertEqual(history["sameDefectFailures"], 1)

    def test_lineage_keeps_findings_after_candidate_rename_events(self):
        self.seed_findings("LIVE-FINDING-STABLE")
        record_event(
            self.db_path,
            str(self.root),
            REQ,
            CAP,
            "b" * 64,
            "renamed-packet",
            "defect_failure",
            {
                "findingId": "LIVE-FINDING-STABLE",
                "defectClass": "retained-loan-executor-defect",
            },
        )
        history = self.history()
        self.assertEqual(history["sameDefectFailures"], 1)


class IsolationControls(IsolatedTempStore):
    def test_no_git_dir_and_no_lock_after_policy_and_store_use(self):
        self.seed_findings("LIVE-FINDING-A")
        record_review(
            self.db_path, str(self.root), self.approved("source")
        )
        res_id = self.reserve_kind("report")
        finish(self.db_path, res_id, {"exitCode": 0})
        self.assertFalse((self.root / ".git").exists())
        self.assertFalse((self.root / "budget.lock").exists())
        self.assertFalse(any(self.root.rglob("budget.lock")))
        self.assertEqual(
            self.db_path.resolve().parent, (self.root / ".moriarty-dev").resolve()
        )
        self.assertNotIn("store_identity", SCHEMA_SQL)


class CanonicalHistoryAndCli(IsolatedTempStore):
    def write_live_action(self, **overrides):
        action = live_action("verify", **overrides)
        action["candidate"] = CANDIDATE_HASH
        (self.root / ".moriarty-dev").mkdir(exist_ok=True)
        (self.root / ".moriarty-dev" / "actions.json").write_text(
            json.dumps({"schema": "moriarty-dev.actions/1", "actions": [action]})
        )
        return action

    def guarded_reader(self, **kwargs):
        return make_guarded_history_reader(
            self.db_path,
            live_lineage=catalog_live_lineage(self.root),
            **kwargs,
        )

    def write_fixture_identity(self, token):
        conn = init_db(self.db_path)
        try:
            with conn:
                conn.execute("CREATE TABLE store_identity (token TEXT NOT NULL)")
                conn.execute(
                    "INSERT INTO store_identity (token) VALUES (?)", (token,)
                )
        finally:
            conn.close()

    def test_schema_sql_does_not_create_identity(self):
        self.assertNotIn("store_identity", SCHEMA_SQL)
        self.assertIsNone(read_store_identity(self.db_path))

    def test_missing_identity_yields_unavailable_live_history(self):
        self.seed_findings("LIVE-FINDING-A")
        self.assertIsNone(
            get_live_history(self.db_path, str(self.root), REQ, CAP)
        )
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor="fixture-anchor-1"
            )
        )

    def test_unrelated_events_do_not_create_verified_zero_live_history(self):
        record_event(
            self.db_path, str(self.root), "", "", "", "seed", "admin_cycle", {}
        )
        self.write_fixture_identity("fixture-anchor-1")
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor="fixture-anchor-1"
            )
        )

    def test_anchored_fixture_counts_exact_lineage_through_actual_reader(self):
        self.write_live_action()
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        self.write_fixture_identity("fixture-anchor-1")
        hist = get_live_history(
            self.db_path,
            str(self.root),
            REQ,
            CAP,
            identity_anchor="fixture-anchor-1",
        )
        self.assertIsNotNone(hist)
        self.assertEqual(hist["sameDefectFailures"], 2)
        reader = self.guarded_reader(identity_anchor="fixture-anchor-1")
        snap = load_snapshot(str(self.root), "live-loan-verify", history_reader=reader)
        self.assertEqual(snap["sameDefectFailures"], 2)
        self.assertNotIn("operational-history", snap["missingEvidence"])

    def test_cli_seeded_unrelated_event_is_unavailable_not_zero(self):
        self.write_live_action()
        record_event(
            self.db_path, str(self.root), "", "", "", "seed", "admin_cycle", {}
        )
        reader = self.guarded_reader()
        snap = load_snapshot(str(self.root), "live-loan-verify", history_reader=reader)
        self.assertIsNone(snap["sameDefectFailures"])
        self.assertIn("operational-history", snap["missingEvidence"])
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            cmd_status(self.root, self.db_path, reader, True)
        data = json.loads(buf.getvalue())
        self.assertIn("operational-history", data["missingEvidence"])

    def test_cli_renamed_capability_is_unavailable_not_zero(self):
        self.write_live_action()
        self.seed_findings("LIVE-FINDING-A")
        action = live_action("verify")
        action["candidate"] = CANDIDATE_HASH
        action["capability"] = "renamed-loan-executor"
        (self.root / ".moriarty-dev" / "actions.json").write_text(
            json.dumps({"schema": "moriarty-dev.actions/1", "actions": [action]})
        )
        reader = self.guarded_reader()
        snap = load_snapshot(str(self.root), "live-loan-verify", history_reader=reader)
        self.assertIsNone(snap["sameDefectFailures"])
        self.assertIn("operational-history", snap["missingEvidence"])

    def test_cli_run_does_not_launch_live_action(self):
        self.write_live_action()
        marker = self.root / "child-marker.txt"
        reader = self.guarded_reader()
        with self.assertRaises(SystemExit) as caught:
            cmd_run(self.root, self.db_path, reader, "live-loan-verify", False)
        self.assertIn(caught.exception.code, (2, 3))
        self.assertFalse(marker.exists())
        self.assertTrue(str(self.db_path).startswith(str(self.root)))

    def test_cli_review_authority_rejection_is_preexisting(self):
        self.write_live_action()
        (self.root / "review.json").write_text(json.dumps(self.approved("source")))
        err = io.StringIO()
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(err):
            with self.assertRaises(SystemExit) as caught:
                cmd_review(
                    self.root, self.db_path, "review.json", True, "live-loan-verify"
                )
        self.assertEqual(caught.exception.code, 3)
        self.assertIn("authority unavailable", err.getvalue())

    def test_status_does_not_create_identity_table(self):
        self.write_live_action()
        reader = self.guarded_reader()
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            cmd_status(self.root, self.db_path, reader, True)
        conn = init_db(self.db_path)
        try:
            names = [
                row[0]
                for row in conn.execute(
                    "SELECT name FROM sqlite_master WHERE type='table'"
                ).fetchall()
            ]
        finally:
            conn.close()
        self.assertNotIn("store_identity", names)
        self.assertIsNone(read_store_identity(self.db_path))

    def _cli(self, *args, env=None):
        cli_py = SCRIPTS_DIR / "moriarty_dev" / "cli.py"
        merged = os.environ.copy()
        if env:
            merged.update(env)
        return subprocess.run(
            [sys.executable, str(cli_py), "--repo", str(self.root), *args],
            capture_output=True,
            text=True,
            env=merged,
        )

    def test_main_subprocess_ignores_populated_identity_and_env_anchor(self):
        self.write_live_action()
        self.seed_findings("LIVE-FINDING-A", "LIVE-FINDING-B")
        self.write_fixture_identity("fixture-anchor-1")
        env = {
            "MORIARTY_IDENTITY_ANCHOR": "fixture-anchor-1",
            "IDENTITY_ANCHOR": "fixture-anchor-1",
            "identity_anchor": "fixture-anchor-1",
        }
        status = self._cli("status", "--json", env=env)
        self.assertEqual(status.returncode, 0, status.stderr)
        status_data = json.loads(status.stdout)
        self.assertIn("operational-history", status_data["missingEvidence"])
        nxt = self._cli("next", "--json", env=env)
        self.assertEqual(nxt.returncode, 0, nxt.stderr)
        next_data = json.loads(nxt.stdout)
        self.assertFalse(next_data["allow"])
        self.assertIn("operational-history", next_data["missingEvidence"])
        (self.root / "review.json").write_text(json.dumps(self.approved("source")))
        review = self._cli(
            "review", "--receipt", "review.json", "--action", "live-loan-verify", env=env
        )
        self.assertEqual(review.returncode, 3)
        self.assertIn("authority unavailable", review.stderr)
        reader = self.guarded_reader()
        snap = load_snapshot(str(self.root), "live-loan-verify", history_reader=reader)
        self.assertIsNone(snap["sameDefectFailures"])
        self.assertIn("operational-history", snap["missingEvidence"])

    def test_mismatched_and_invalid_identity_tokens_are_unavailable(self):
        self.write_live_action()
        self.seed_findings("LIVE-FINDING-A")
        self.write_fixture_identity("fixture-anchor-1")
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor="fixture-anchor-2"
            )
        )
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor=" "
            )
        )
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor=7
            )
        )
        conn = init_db(self.db_path)
        try:
            with conn:
                conn.execute("DELETE FROM store_identity")
                conn.execute("INSERT INTO store_identity (token) VALUES (?)", ("   ",))
        finally:
            conn.close()
        self.assertIsNone(read_store_identity(self.db_path))
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor="fixture-anchor-1"
            )
        )
        conn = init_db(self.db_path)
        try:
            with conn:
                conn.execute("DELETE FROM store_identity")
                conn.execute("DROP TABLE store_identity")
                conn.execute("CREATE TABLE store_identity (token BLOB NOT NULL)")
                conn.execute(
                    "INSERT INTO store_identity (token) VALUES (?)",
                    (b"\x00\x01nonstring",),
                )
        finally:
            conn.close()
        self.assertIsNone(read_store_identity(self.db_path))
        conn = init_db(self.db_path)
        try:
            with conn:
                conn.execute("DELETE FROM store_identity")
                conn.execute(
                    "INSERT INTO store_identity (token) VALUES (?), (?)",
                    ("fixture-anchor-1", "fixture-anchor-2"),
                )
        finally:
            conn.close()
        self.assertIsNone(read_store_identity(self.db_path))
        self.assertIsNone(
            get_live_history(
                self.db_path, str(self.root), REQ, CAP, identity_anchor="fixture-anchor-1"
            )
        )
        reader = make_guarded_history_reader(
            self.db_path,
            live_lineage=catalog_live_lineage(self.root),
            identity_anchor="fixture-anchor-1",
        )
        snap = load_snapshot(str(self.root), "live-loan-verify", history_reader=reader)
        self.assertIsNone(snap["sameDefectFailures"])
        self.assertIn("operational-history", snap["missingEvidence"])

    def test_catalog_callback_exceptions_are_unavailable(self):
        self.seed_findings("LIVE-FINDING-A")

        def boom(_lineage):
            raise RuntimeError("catalog")

        reader = make_guarded_history_reader(self.db_path, live_lineage=boom)
        self.assertIsNone(
            reader(
                {
                    "repository": str(self.root),
                    "requirement": REQ,
                    "capability": CAP,
                }
            )
        )
        check = catalog_live_lineage(self.root)
        self.assertTrue(
            check(
                {
                    "repository": str(self.root),
                    "requirement": REQ,
                    "capability": CAP,
                }
            )
        )

    def test_git_init_and_linked_worktree_live_history_never_zero(self):
        # Writers key events by checkout path; records supplies the git common
        # dir. That mismatch is preexisting. Live history counts related keys
        # or stays unavailable, never 0 while checkout findings remain.
        main = self.root / "git-main"
        main.mkdir()
        subprocess.run(["git", "init"], cwd=str(main), check=True, capture_output=True)
        subprocess.run(
            ["git", "config", "user.email", "test@moriarty.local"],
            cwd=str(main),
            check=True,
            capture_output=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "Test User"],
            cwd=str(main),
            check=True,
            capture_output=True,
        )
        (main / "README.md").write_text("init\n")
        subprocess.run(
            ["git", "add", "README.md"], cwd=str(main), check=True, capture_output=True
        )
        subprocess.run(
            ["git", "commit", "-m", "init"],
            cwd=str(main),
            check=True,
            capture_output=True,
        )
        git_db = get_db_path(main)
        live_common = Path("/home/charl/Moriarty/.git/moriarty-dev").resolve()
        self.assertFalse(str(git_db.resolve()).startswith(str(live_common)))
        self.assertTrue(str(git_db.resolve()).startswith(str(self.root.resolve())))
        init_db(git_db).close()
        action = live_action("verify")
        action["candidate"] = CANDIDATE_HASH
        (main / ".moriarty-dev").mkdir(exist_ok=True)
        (main / ".moriarty-dev" / "actions.json").write_text(
            json.dumps({"schema": "moriarty-dev.actions/1", "actions": [action]})
        )
        conn = init_db(git_db)
        try:
            with conn:
                conn.execute("CREATE TABLE store_identity (token TEXT NOT NULL)")
                conn.execute(
                    "INSERT INTO store_identity (token) VALUES (?)",
                    ("fixture-anchor-1",),
                )
        finally:
            conn.close()
        record_event(
            git_db, str(main), REQ, CAP, CANDIDATE_HASH, "old", "defect_failure",
            {"findingId": "GIT-F1"},
        )
        record_event(
            git_db, str(main), REQ, CAP, CANDIDATE_HASH, "old", "defect_failure",
            {"findingId": "GIT-F2"},
        )
        reader = make_guarded_history_reader(
            git_db,
            live_lineage=catalog_live_lineage(main),
            identity_anchor="fixture-anchor-1",
        )
        snap = load_snapshot(str(main), "live-loan-verify", history_reader=reader)
        self.assertNotEqual(snap["sameDefectFailures"], 0)
        wt = Path(tempfile.mkdtemp(prefix="wt-live-hist-", dir=str(self.root)))
        try:
            subprocess.run(
                ["git", "worktree", "add", str(wt)],
                cwd=str(main),
                check=True,
                capture_output=True,
            )
            wt_db = get_db_path(wt)
            self.assertEqual(wt_db.resolve(), git_db.resolve())
            self.assertFalse(str(wt_db.resolve()).startswith(str(live_common)))
            (wt / ".moriarty-dev").mkdir(exist_ok=True)
            (wt / ".moriarty-dev" / "actions.json").write_text(
                json.dumps({"schema": "moriarty-dev.actions/1", "actions": [action]})
            )
            record_event(
                wt_db, str(wt), REQ, CAP, CANDIDATE_HASH, "old", "defect_failure",
                {"findingId": "WT-F1"},
            )
            record_event(
                wt_db, str(wt), REQ, CAP, CANDIDATE_HASH, "old", "defect_failure",
                {"findingId": "WT-F2"},
            )
            common = (main / ".git").resolve()
            direct = get_history(wt_db, str(wt), REQ, CAP)
            self.assertGreaterEqual(direct["sameDefectFailures"], 2)
            wt_reader = make_guarded_history_reader(
                wt_db,
                live_lineage=catalog_live_lineage(wt),
                identity_anchor="fixture-anchor-1",
            )
            before = load_snapshot(str(wt), "live-loan-verify", history_reader=wt_reader)
            self.assertNotEqual(before["sameDefectFailures"], 0)
            record_event(
                wt_db, str(common), REQ, CAP, CANDIDATE_HASH, "seed", "observation", {}
            )
            after = load_snapshot(str(wt), "live-loan-verify", history_reader=wt_reader)
            self.assertNotEqual(after["sameDefectFailures"], 0)
            live_hist = get_live_history(
                wt_db, str(common), REQ, CAP, identity_anchor="fixture-anchor-1"
            )
            self.assertIsNotNone(live_hist)
            self.assertNotEqual(live_hist["sameDefectFailures"], 0)
            legacy_common = get_history(wt_db, str(common), REQ, CAP)
            self.assertEqual(legacy_common["sameDefectFailures"], 0)
        finally:
            subprocess.run(
                ["git", "worktree", "remove", "--force", str(wt)],
                cwd=str(main),
                check=False,
                capture_output=True,
            )
            shutil.rmtree(wt, ignore_errors=True)

    def test_copied_inventoried_schema_does_not_create_identity(self):
        self.write_live_action()
        conn = init_db(self.db_path)
        now = "2026-09-11T00:00:00+00:00"
        try:
            with conn:
                conn.execute(
                    """
                    INSERT INTO events (
                        repository, requirement, capability, candidate,
                        action_id, event_kind, observed_at, payload_json
                    ) VALUES (?, '', '', '', 'admin', 'admin_cycle', ?, ?)
                    """,
                    (str(self.root), now, json.dumps({"type": "status"})),
                )
                conn.execute(
                    """
                    INSERT INTO reservations (
                        id, repository, requirement, capability, candidate,
                        action_id, action_kind, pid, status, reserved_at, finished_at
                    ) VALUES (?, ?, ?, ?, ?, ?, 'report', 1, 'finished', ?, ?)
                    """,
                    (
                        "inventoried-res",
                        str(self.root),
                        "SP01.6",
                        "loan-swap-subset",
                        CANDIDATE_HASH,
                        "sp01-loan-report",
                        now,
                        now,
                    ),
                )
                conn.execute(
                    "INSERT INTO runner_claims (charge_id, reservation_id) VALUES (?, ?)",
                    ("inventoried-charge", "inventoried-res"),
                )
                conn.execute(
                    """
                    INSERT INTO admin_intervals (repository, start_time, end_time, activity_type)
                    VALUES (?, 1.0, 2.0, 'status')
                    """,
                    (str(self.root),),
                )
                conn.execute(
                    """
                    INSERT INTO outbox (repository, tx_id, status, enqueued_at, details_json)
                    VALUES (?, ?, 'submitted', ?, ?)
                    """,
                    (
                        str(self.root),
                        "tx-inventoried",
                        now,
                        json.dumps({"network": "preview"}),
                    ),
                )
        finally:
            conn.close()
        reader = self.guarded_reader()
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            cmd_status(self.root, self.db_path, reader, True)
        (self.root / "review.json").write_text(json.dumps(self.approved("source")))
        err = io.StringIO()
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(err):
            with self.assertRaises(SystemExit) as caught:
                cmd_review(
                    self.root, self.db_path, "review.json", True, "live-loan-verify"
                )
        self.assertEqual(caught.exception.code, 3)
        self.assertIn("authority unavailable", err.getvalue())
        with self.assertRaises(SystemExit) as run_caught:
            cmd_run(self.root, self.db_path, reader, "live-loan-verify", False)
        self.assertIn(run_caught.exception.code, (2, 3))
        conn = init_db(self.db_path)
        try:
            names = [
                row[0]
                for row in conn.execute(
                    "SELECT name FROM sqlite_master WHERE type='table'"
                ).fetchall()
            ]
        finally:
            conn.close()
        self.assertNotIn("store_identity", names)
        self.assertIsNone(read_store_identity(self.db_path))


if __name__ == "__main__":
    unittest.main()
