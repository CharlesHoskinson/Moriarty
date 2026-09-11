import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

# Ensure scripts and tests directories are importable
SCRIPTS_DIR = Path(__file__).resolve().parent.parent / "scripts"
TESTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

import test_records as tr
from moriarty_dev.store import (
    get_db_path,
    init_db,
    reserve,
    finish,
    record_event,
    record_review,
    record_admin_interval,
    enqueue_tx,
    get_undelivered_txs,
    mark_delivered,
    get_history,
    ReservationConflictError,
    StoreError,
)
from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot


class ExecutionTestCase(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.TemporaryDirectory()
        self.root = Path(self.temp_dir.name)
        # git init
        subprocess.run(["git", "init"], cwd=str(self.root), capture_output=True, check=True)
        subprocess.run(["git", "config", "user.email", "test@moriarty.local"], cwd=str(self.root), check=True)
        subprocess.run(["git", "config", "user.name", "Test User"], cwd=str(self.root), check=True)
        
        self.db_path = get_db_path(self.root)
        init_db(self.db_path).close()
        self.child_marker = self.root / "child-marker.txt"

    def tearDown(self):
        self.temp_dir.cleanup()

    def make_case(self, failures=0, action_kind="implement", candidate=None):
        # Set up genuine loan review family in self.root
        tr.copy_genuine_registers(self.root)
        campaigns = tr.load_json(self.root / "evidence" / "moriarty-completion-program-2026-09-07" / "report-reconciliation" / "campaign-admission.json")
        tr.write_current_accounting(self.root)
        tr.attach_current_accounting(self.root, campaigns)

        # Write commands.json
        cmd_file = self.root / "commands.json"
        candidate_value = candidate or tr.LOAN_CANDIDATE
        action_id = "sp01-loan-" + action_kind
        runner_program = (
            "import json; from pathlib import Path; "
            f"Path({str(self.child_marker)!r}).write_text('one launch\\n'); "
            f"print(json.dumps({{'candidateHash': {candidate_value!r}, 'actionId': {action_id!r}, "
            "'charged': True, 'chargeId': 'test-admitted-charge-01', "
            f"'behavioralAssertions': [{{'candidateHash': {candidate_value!r}, 'assertionId': 'test-defect', 'outcome': 'defect-observed'}}]}})); "
            "raise SystemExit(1)"
        )
        cmd_data = {
            "schema": "moriarty-dev.commands/1",
            "commands": {
                "driver": {"admittedRunner": {
                    "argv": [
                        sys.executable,
                        "-c",
                        runner_program,
                    ],
                    "actionId": action_id,
                    "candidateHash": candidate_value,
                    "timeoutSeconds": 30,
                }}
            }
        }
        cmd_file.write_text(json.dumps(cmd_data))

        action_def = tr.loan_action(action_kind, "commands.json#driver")
        if candidate:
            action_def["candidate"] = candidate

        tr.write_actions(self.root, [action_def, tr.loan_action("report")])

        for i in range(failures):
            record_event(
                self.db_path,
                str(self.root),
                action_def["requirement"],
                action_def["capability"],
                action_def["candidate"],
                action_def["id"],
                "defect_failure",
                {"defectId": f"DEFECT-{i+1}", "details": "failing test"},
            )

        return action_def

    def run_cli(self, *args):
        cli_py = SCRIPTS_DIR / "moriarty_dev" / "cli.py"
        cmd = [sys.executable, str(cli_py), "--repo", str(self.root)] + list(args)
        return subprocess.run(cmd, capture_output=True, text=True)

    def test_denied_launch_has_no_child_effect(self):
        self.make_case(failures=2, action_kind="implement")
        res = self.run_cli("run", "--action", "sp01-loan-implement")
        self.assertEqual(res.returncode, 2)
        self.assertFalse(self.child_marker.exists())

    def test_restart_preserves_failure(self):
        self.make_case(failures=2, action_kind="implement", candidate=tr.LOAN_CANDIDATE)
        res1 = self.run_cli("run", "--action", "sp01-loan-implement")
        self.assertEqual(res1.returncode, 2)

        # Rename candidate in action: failure remains because lineage (repo, req, cap) is unchanged
        self.make_case(failures=0, action_kind="implement", candidate="other-candidate")
        res2 = self.run_cli("run", "--action", "sp01-loan-implement")
        self.assertEqual(res2.returncode, 2)
        self.assertFalse(self.child_marker.exists())

    def test_untrusted_reproducer_mapping_does_not_launch(self):
        self.make_case(failures=2, action_kind="reproduce")
        res1 = self.run_cli("run", "--action", "sp01-loan-reproduce")
        self.assertEqual(res1.returncode, 2, res1.stderr)
        self.assertIn("no authenticated admitted-runner authority", res1.stderr)
        self.assertFalse(self.child_marker.exists())

        # A mutable command mapping and self-reported charge cannot verify a defect.
        history = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertFalse(history["reproducerVerified"])

    def test_concurrent_reservation_denial(self):
        action = self.make_case(failures=0, action_kind="implement")
        snap = {"primaryActive": False}
        res_id1 = reserve(self.db_path, str(self.root), action, snap)
        self.assertTrue(res_id1)

        # Second reservation for primary implementation must fail
        with self.assertRaises(ReservationConflictError):
            reserve(self.db_path, str(self.root), action, snap)

        # Finish first reservation
        finish(self.db_path, res_id1, {"exitCode": 0})

        # Now reservation succeeds
        res_id2 = reserve(self.db_path, str(self.root), action, snap)
        self.assertTrue(res_id2)
        finish(self.db_path, res_id2, {"exitCode": 0})

    def test_linked_worktrees_share_db(self):
        readme = self.root / "README.md"
        readme.write_text("initial")
        subprocess.run(["git", "add", "README.md"], cwd=str(self.root), check=True, capture_output=True)
        subprocess.run(["git", "commit", "-m", "initial commit"], cwd=str(self.root), check=True, capture_output=True)

        wt_dir = Path(tempfile.mkdtemp(prefix="wt-test-"))
        try:
            subprocess.run(["git", "worktree", "add", "-b", "branch-b", str(wt_dir)], cwd=str(self.root), check=True, capture_output=True)
            wt_db_path = get_db_path(wt_dir)
            self.assertEqual(wt_db_path.resolve(), self.db_path.resolve())

            action = {"id": "worktree-primary", "kind": "implement"}
            first = reserve(self.db_path, str(self.root), action, {})
            with self.assertRaises(ReservationConflictError):
                reserve(wt_db_path, str(wt_dir), action, {})
            finish(self.db_path, first, {"exitCode": 0})
            successor = reserve(wt_db_path, str(wt_dir), action, {})
            finish(wt_db_path, successor, {"exitCode": 0})

            record_event(wt_db_path, str(self.root), "SP05", "fixed-financial-driver", "cand-x", "act-x", "defect_failure", {})
            history = get_history(self.db_path, str(self.root), "SP05", "fixed-financial-driver")
            self.assertEqual(history["sameDefectFailures"], 1)
        finally:
            subprocess.run(["git", "worktree", "remove", "--force", str(wt_dir)], cwd=str(self.root), check=False, capture_output=True)
            shutil.rmtree(wt_dir, ignore_errors=True)

    def test_crashed_reservation_recovery(self):
        action = self.make_case(failures=0, action_kind="implement")
        snap = {"primaryActive": False}
        res_id = reserve(self.db_path, str(self.root), action, snap)

        # Simulate dead process by setting PID to a non-existent PID (e.g. 999999)
        conn = init_db(self.db_path)
        with conn:
            conn.execute("UPDATE reservations SET pid = 999999 WHERE id = ?", (res_id,))
        conn.close()

        # A dead parent says nothing about its child or durable charge. Keep
        # this reservation blocking until actual completion is reconciled.
        with self.assertRaises(ReservationConflictError):
            reserve(self.db_path, str(self.root), action, snap)
        history = get_history(self.db_path, str(self.root), action["requirement"], action["capability"])
        self.assertTrue(history["primaryActive"])
        conn = init_db(self.db_path)
        self.assertEqual(conn.execute("SELECT status FROM reservations WHERE id = ?", (res_id,)).fetchone()[0], "active")
        conn.close()

    def test_dead_parent_remains_active_in_read_only_history(self):
        action = self.make_case(action_kind="implement")
        res_id = reserve(self.db_path, str(self.root), action, {})
        conn = init_db(self.db_path)
        conn.execute("UPDATE reservations SET pid = 999999 WHERE id = ?", (res_id,))
        conn.close()
        history = get_history(self.db_path, str(self.root), action["requirement"], action["capability"])
        self.assertTrue(history["primaryActive"])

    def test_outbox_rejects_private_fields(self):
        # Good public notification
        enqueue_tx(self.db_path, str(self.root), "tx-12345", "submitted", {"blockNumber": 42})
        undelivered = get_undelivered_txs(self.db_path, str(self.root))
        self.assertEqual(len(undelivered), 1)
        self.assertEqual(undelivered[0]["txId"], "tx-12345")

        # Forbidden secret keys
        with self.assertRaises(ValueError):
            enqueue_tx(self.db_path, str(self.root), "tx-secret", "submitted", {"seed": "supersecret"})

        with self.assertRaises(ValueError):
            enqueue_tx(self.db_path, str(self.root), "tx-witness", "submitted", {"witness": "privatedata"})

        with self.assertRaises(ValueError):
            enqueue_tx(self.db_path, str(self.root), "tx-sk", "submitted", {"spending_key": "key"})

    def test_admin_intervals_and_stop_rule(self):
        record_admin_interval(self.db_path, str(self.root), 1000.0, 2800.0, "report_formatting")
        history = get_history(self.db_path, str(self.root), "SP05", "fixed-financial-driver")
        self.assertEqual(history["adminSeconds"], 1800)
        self.assertEqual(history["adminCycles"], 1)

        admin_action = tr.loan_action("admin")
        dec = assess({
            "authorityCurrent": True,
            "entryEligible": True,
            "candidateCurrent": True,
            "resourceAdmitted": True,
            "sameDefectFailures": 0,
            "adminCycles": 1,
            "adminSeconds": 1800,
            "primaryActive": False,
            "reproducerVerified": False,
            "approachChanged": False,
            "nextActionId": None,
            "missingEvidence": [],
        }, admin_action)
        self.assertFalse(dec["allow"])
        self.assertEqual(dec["reasonCode"], "ADMIN_LIMIT")

    def test_public_cli_rejects_unrelated_candidate_review(self):
        self.make_case(failures=0, action_kind="review")
        receipt_file = self.root / "receipt-unrelated-candidate.json"
        receipt_file.write_text(json.dumps({
            "requirement": "SP01.6",
            "capability": "loan-swap-subset",
            "candidateHash": "f" * 64,
            "scope": "RP01-MC02 specified design consistency only",
            "author": "grok-4-6",
            "reviewer": "gpt-6-astra",
            "verdict": "APPROVED",
        }))
        res = self.run_cli("review", str(receipt_file))
        self.assertEqual(res.returncode, 3)
        self.assertIn("candidateHash", res.stderr)
        history = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(history)

    def test_public_cli_rejects_unrelated_scope_review(self):
        self.make_case(failures=0, action_kind="review")
        receipt_file = self.root / "receipt-unrelated-scope.json"
        receipt_file.write_text(json.dumps({
            "requirement": "SP01.6",
            "capability": "loan-swap-subset",
            "candidateHash": tr.LOAN_CANDIDATE,
            "scope": "unrelated-review-scope",
            "author": "grok-4-6",
            "reviewer": "gpt-6-astra",
            "verdict": "APPROVED",
        }))
        res = self.run_cli("review", str(receipt_file))
        self.assertEqual(res.returncode, 3)
        self.assertIn("does not match campaign scope", res.stderr)
        history = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(history)

    def test_public_cli_rejects_author_self_review(self):
        self.make_case(failures=0, action_kind="review")
        receipt_file = self.root / "receipt-self-review.json"
        receipt_file.write_text(json.dumps({
            "requirement": "SP01.6",
            "capability": "loan-swap-subset",
            "candidateHash": tr.LOAN_CANDIDATE,
            "scope": "RP01-MC02 specified design consistency only",
            "author": "grok-4-6",
            "reviewer": "grok-4-6",
            "verdict": "APPROVED",
        }))
        res = self.run_cli("review", str(receipt_file))
        self.assertEqual(res.returncode, 3)
        self.assertIn("author self-review is forbidden", res.stderr)
        history = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(history)

    def test_public_cli_accepts_genuine_current_non_author_review(self):
        self.make_case(failures=1, action_kind="review")
        hist_before = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist_before["sameDefectFailures"], 1)

        receipt_file = self.root / "receipt-genuine.json"
        receipt_file.write_text(json.dumps({
            "requirement": "SP01.6",
            "capability": "loan-swap-subset",
            "candidateHash": tr.LOAN_CANDIDATE,
            "scope": "RP01-MC02 specified design consistency only",
            "author": "grok-4-6",
            "reviewer": "gpt-6-astra",
            "verdict": "APPROVED",
        }))
        res = self.run_cli("review", str(receipt_file))
        self.assertEqual(res.returncode, 0, f"CLI review failed: {res.stderr}")
        hist_after = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist_after["sameDefectFailures"], 0)

    def test_corrupt_history_and_safe_writes(self):
        self.db_path.write_bytes(b"corrupt sqlite header not valid database")
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(hist)
        with self.assertRaises(StoreError):
            init_db(self.db_path)
        self.assertEqual(self.db_path.read_bytes(), b"corrupt sqlite header not valid database")

    def test_schema_only_history_returns_none(self):
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(hist)

    def test_invalid_row_handled_conservatively(self):
        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            tr.LOAN_CANDIDATE,
            "act1",
            "defect_failure",
            {"defectId": "D1"},
        )
        conn = init_db(self.db_path)
        with conn:
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (str(self.root), "SP01.6", "loan-swap-subset", tr.LOAN_CANDIDATE, "act1", "defect_failure", "2026-09-08T00:00:00Z", "not-a-json{"),
            )
        conn.close()
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNotNone(hist)
        self.assertGreaterEqual(hist["sameDefectFailures"], 1)

    def test_unauthorized_finding_resolution_cannot_erase_blocker(self):
        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            tr.LOAN_CANDIDATE,
            "act1",
            "defect_failure",
            {"defectId": "DEFECT-BLOCKER-01"},
        )
        hist1 = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist1["sameDefectFailures"], 1)

        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            tr.LOAN_CANDIDATE,
            "act1",
            "defect_resolved",
            {"defectId": "DEFECT-BLOCKER-01", "details": "author claimed fixed"},
        )
        hist2 = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist2["sameDefectFailures"], 1)

    def test_authentic_review_clears_stable_finding(self):
        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            tr.LOAN_CANDIDATE,
            "act1",
            "defect_failure",
            {"defectId": "DEFECT-BLOCKER-02"},
        )
        hist1 = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist1["sameDefectFailures"], 1)

        receipt = {
            "requirement": "SP01.6",
            "capability": "loan-swap-subset",
            "candidateHash": tr.LOAN_CANDIDATE,
            "scope": "RP01-MC02 specified design consistency only",
            "author": "grok-4-6",
            "reviewer": "gpt-6-astra",
            "verdict": "APPROVED",
        }
        record_review(self.db_path, str(self.root), receipt)

        hist2 = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist2["sameDefectFailures"], 0)

    def test_lineage_and_finding_identity_survives_candidate_rename(self):
        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            "01" * 32,
            "act1",
            "defect_failure",
            {"findingId": "FINDING-STABLE-01", "defectClass": "driver-loop"},
        )
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist["sameDefectFailures"], 1)

        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            "02" * 32,
            "act2",
            "defect_failure",
            {"findingId": "FINDING-STABLE-01", "defectClass": "driver-loop"},
        )
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist["sameDefectFailures"], 1)

    def test_admin_timing_unknown_and_union_and_exclusion(self):
        record_event(
            self.db_path,
            str(self.root),
            "SP01.6",
            "loan-swap-subset",
            tr.LOAN_CANDIDATE,
            "act1",
            "defect_failure",
            {"findingId": "F1"},
        )
        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertIsNone(hist["adminSeconds"])

        record_admin_interval(self.db_path, str(self.root), 100.0, 200.0, "status")
        record_admin_interval(self.db_path, str(self.root), 150.0, 250.0, "review")
        record_admin_interval(self.db_path, str(self.root), 300.0, 400.0, "test")
        record_admin_interval(self.db_path, str(self.root), 400.0, 500.0, "afk")
        record_admin_interval(self.db_path, str(self.root), 500.0, 600.0, "idle")
        record_admin_interval(self.db_path, str(self.root), 600.0, 700.0, "testing")

        hist = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertEqual(hist["adminSeconds"], 150)

    def test_status_cli_names_stale_source_alongside_unknown_history(self):
        self.make_case(failures=0, action_kind="report")
        source = self.root / "openspec/sprints/sp01-financial-contract-and-execution-admission.md"
        source.write_text(source.read_text() + "\nUnreviewed drift.\n")
        result = self.run_cli("status", "--json")
        self.assertEqual(result.returncode, 0)
        data = json.loads(result.stdout)
        self.assertIn("operational history is unresolved or unverified", data["reason"])
        self.assertIn("binding-input-stale:", data["reason"])
        self.assertIn("candidate-input-stale:", data["reason"])

    def test_status_cli_reports_unresolved_history_honestly(self):
        self.make_case(failures=0, action_kind="implement")
        res = self.run_cli("status", "--json")
        self.assertEqual(res.returncode, 0)
        data = json.loads(res.stdout)
        self.assertIn("operational-history", data["missingEvidence"])
        self.assertIn("operational history is unresolved or unverified", data["reason"])
        self.assertIn("implementation/repair of", data["blockedAction"])


if __name__ == "__main__":
    unittest.main()
