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
        init_db(self.db_path)
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
        cmd_data = {
            "schema": "moriarty-dev.commands/1",
            "commands": {
                "driver": {
                    "argv": [
                        sys.executable,
                        "-c",
                        f"from pathlib import Path; Path({repr(str(self.child_marker))}).write_text('one launch\\n')",
                    ]
                }
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

    def test_allowed_reproducer_runs_once(self):
        self.make_case(failures=2, action_kind="reproduce")
        res1 = self.run_cli("run", "--action", "sp01-loan-reproduce")
        self.assertEqual(res1.returncode, 0)
        self.assertTrue(self.child_marker.exists())
        self.assertEqual(self.child_marker.read_text(), "one launch\n")

        # After reproducer verified, reproducer result is recorded
        history = get_history(self.db_path, str(self.root), "SP01.6", "loan-swap-subset")
        self.assertTrue(history["reproducerVerified"])

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

        # Reserving again should clean up crashed reservation and succeed
        new_res = reserve(self.db_path, str(self.root), action, snap)
        self.assertTrue(new_res)
        finish(self.db_path, new_res, {"exitCode": 0})

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


if __name__ == "__main__":
    unittest.main()
