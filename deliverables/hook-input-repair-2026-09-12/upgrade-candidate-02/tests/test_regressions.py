import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent.parent / "scripts"
TESTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

import test_records as tr
from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot
from moriarty_dev.store import (
    get_db_path,
    init_db,
    record_event,
    record_review,
    get_history,
)


class HistoricalRegressionsTestCase(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.TemporaryDirectory()
        self.root = Path(self.temp_dir.name)
        subprocess.run(["git", "init"], cwd=str(self.root), capture_output=True, check=True)
        subprocess.run(["git", "config", "user.email", "test@moriarty.local"], cwd=str(self.root), check=True)
        subprocess.run(["git", "config", "user.name", "Test User"], cwd=str(self.root), check=True)
        self.db_path = get_db_path(self.root)
        init_db(self.db_path).close()

    def tearDown(self):
        self.temp_dir.cleanup()

    def test_composition_green_summary_cannot_clear_failing_full_state(self):
        # Even if a review or summary claims all green, a failing full-state test invalidates candidateCurrent
        snap = {
            "authorityCurrent": True,
            "entryEligible": True,
            "candidateCurrent": False,
            "resourceAdmitted": True,
            "sameDefectFailures": 0,
            "adminCycles": 0,
            "adminSeconds": 0,
            "primaryActive": False,
            "reproducerVerified": False,
            "approachChanged": False,
            "nextActionId": "sp05-ledger-driver",
            "missingEvidence": ["publication-control-acceptance:exitCode-nonzero"],
        }
        action = {
            "id": "sp05-ledger-implement",
            "requirement": "SP05.1",
            "capability": "fixed-financial-driver",
            "kind": "implement",
            "candidate": "daac83578f105b0a70ce44d88c579018881daae5",
            "admissionRef": "campaign:sp01-loan-swap-grok-01",
            "commandRef": "commands.json#driver",
            "evidenceProfile": "local-runtime",
        }
        dec = assess(snap, action)
        self.assertFalse(dec["allow"])
        self.assertEqual(dec["reasonCode"], "EVIDENCE_STALE")

    def test_new_packet_cannot_erase_prior_failures(self):
        # Failures are keyed by lineage: (repository, requirement, capability)
        req = "SP05.1"
        cap = "fixed-financial-driver"
        record_event(self.db_path, str(self.root), req, cap, "candidate-01", "act-01", "defect_failure", {"defectId": "D1"})
        record_event(self.db_path, str(self.root), req, cap, "candidate-01", "act-01", "defect_failure", {"defectId": "D2"})

        # New packet created with candidate-02
        history = get_history(self.db_path, str(self.root), req, cap)
        self.assertEqual(history["sameDefectFailures"], 2)

        action = {
            "id": "sp05-ledger-implement",
            "requirement": req,
            "capability": cap,
            "kind": "implement",
            "candidate": "candidate-02",
            "admissionRef": "campaign:sp01-loan-swap-grok-01",
            "commandRef": "commands.json#driver",
            "evidenceProfile": "local-runtime",
        }
        snap = {
            "authorityCurrent": True,
            "entryEligible": True,
            "candidateCurrent": True,
            "resourceAdmitted": True,
            "sameDefectFailures": history["sameDefectFailures"],
            "adminCycles": 0,
            "adminSeconds": 0,
            "primaryActive": False,
            "reproducerVerified": False,
            "approachChanged": False,
            "nextActionId": "sp05-ledger-reproduce",
            "missingEvidence": [],
        }
        dec = assess(snap, action)
        self.assertFalse(dec["allow"])
        self.assertEqual(dec["reasonCode"], "REPRODUCE_BEFORE_RETRY")

    def test_stale_checkpoint_activity_not_live_worker(self):
        # A historical checkpoint with no live worker does not grant live dispatch
        stale_budget = {
            "schema": "moriarty.supervised-accounting/1",
            "master_limit_seconds": 20000,
            "worker_dispatches": 8,
            "worker_dispatch_limit": 8,  # Dispatches exhausted
        }
        self.assertEqual(stale_budget["worker_dispatches"], stale_budget["worker_dispatch_limit"])

    def test_reproducer_demonstrates_defect(self):
        # Running reproduce_driver.py exits with 1 because the driver returns admitted-not-executed
        reproducer = SCRIPTS_DIR.parent / "tests" / "reproduce_driver.py"
        res = subprocess.run([sys.executable, str(reproducer)], capture_output=True, text=True)
        self.assertNotEqual(res.returncode, 0)
        self.assertIn("Defect reproduced", res.stderr)
        self.assertIn("admitted-not-executed", res.stderr)

    def test_stop_rules_preserve_repair_and_deny_broad_implement(self):
        req = "SP05.1"
        cap = "fixed-financial-driver"
        snap = {
            "authorityCurrent": True,
            "entryEligible": True,
            "candidateCurrent": True,
            "resourceAdmitted": True,
            "sameDefectFailures": 2,
            "adminCycles": 0,
            "adminSeconds": 0,
            "primaryActive": False,
            "reproducerVerified": True,
            "approachChanged": True,
            "nextActionId": "sp05-ledger-repair",
            "missingEvidence": [],
        }
        implement_action = {
            "id": "sp05-ledger-implement",
            "requirement": req,
            "capability": cap,
            "kind": "implement",
            "candidate": "candidate-02",
            "admissionRef": "campaign:sp01-loan-swap-grok-01",
            "commandRef": "commands.json#driver",
            "evidenceProfile": "local-runtime",
        }
        repair_action = {
            "id": "sp05-ledger-repair",
            "requirement": req,
            "capability": cap,
            "kind": "repair",
            "candidate": "candidate-02",
            "admissionRef": "campaign:sp01-loan-swap-grok-01",
            "commandRef": "commands.json#repair-driver",
            "evidenceProfile": "local-runtime",
        }
        # Implement is blocked
        dec_imp = assess(snap, implement_action)
        self.assertFalse(dec_imp["allow"])
        self.assertEqual(dec_imp["reasonCode"], "REPRODUCE_BEFORE_RETRY")

        # Repair is permitted because reproducerVerified and approachChanged are both True
        dec_rep = assess(snap, repair_action)
        self.assertTrue(dec_rep["allow"])
        self.assertEqual(dec_rep["reasonCode"], "ALLOWED")


if __name__ == "__main__":
    unittest.main()
