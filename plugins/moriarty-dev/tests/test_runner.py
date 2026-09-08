"""Public CLI tests: campaign authority and precharged execution, not stdout flags."""
import hashlib
import json
import os
import shutil
import signal
import sqlite3
import subprocess
import sys
import time
import unittest
from pathlib import Path

sys.path[:0] = [str(Path(__file__).resolve().parent), str(Path(__file__).resolve().parents[1] / "scripts")]
import test_execution as execution
import test_records as tr


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


class BoundRunnerTests(unittest.TestCase):
    setUp = execution.ExecutionTestCase.setUp
    tearDown = execution.ExecutionTestCase.tearDown
    make_case = execution.ExecutionTestCase.make_case
    run_cli = execution.ExecutionTestCase.run_cli

    def admit(self, program=None, kind="verify"):
        self.action = self.make_case(action_kind=kind)
        script = self.root / "approved-check.py"
        script.write_text(program or "from pathlib import Path\nPath('child-marker.txt').write_text('checked')\nprint('checked')\n")
        node = str(Path(shutil.which("node")).resolve())
        launcher = "/home/charl/foreman/skills/foreman/runtime/dist/foreman-launch.js"
        python = str(Path(sys.executable).resolve())
        self.plan = {
            "schema": "moriarty.bound-runner/1", "action": self.action,
            "argv": [python, "approved-check.py"],
            "files": {python: digest(python), "approved-check.py": digest(script)},
            "launcher": [node, launcher],
            "launcherFiles": {node: digest(node), launcher: digest(launcher)},
            "timeoutSeconds": 5, "graceSeconds": 2, "outputLimitBytes": 4096,
            "chargeId": "local-verification-01", "chargedSeconds": 120,
            "accountingPath": tr.CURRENT_ACCOUNTING_REL,
        }
        if kind == "reproduce":
            self.plan["assertion"] = {
                "id": "known-behavior", "exitCode": 42,
                "stdoutSha256": hashlib.sha256(b"defect observed\n").hexdigest(),
            }
        mapping = {"schema": "moriarty-dev.commands/1", "commands": {"driver": {"argv": self.plan["argv"]}}}
        tr.dump_json(self.root / "commands.json", mapping)
        campaign_path = self.root / "evidence/moriarty-completion-program-2026-09-07/report-reconciliation/campaign-admission.json"
        self.campaign_path = campaign_path
        campaigns = tr.load_json(campaign_path)
        campaign = campaigns["campaigns"]["sp01-loan-swap-grok-01"]
        binding = tr.load_json(self.root / campaign["binding"])
        # A new current binding preserves the original accepted historical file.
        binding["runners"] = {self.action["id"]: self.plan}
        tr.dump_json(self.root / "current-binding.json", binding)
        campaign["binding"] = "current-binding.json"
        campaign["bindingSha256"] = digest(self.root / "current-binding.json")
        tr.dump_json(campaign_path, campaigns)
        self.accounting = self.root / tr.CURRENT_ACCOUNTING_REL
        budget = tr.load_json(self.accounting)
        self.runner_digest = hashlib.sha256(json.dumps(self.plan, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        # The caller's existing budget runner debits before invoking the plugin.
        budget["charges"].append({"id": self.plan["chargeId"], "seconds": self.plan["chargedSeconds"],
                                  "candidateHash": self.action["candidate"], "actionId": self.action["id"],
                                  "runnerDigest": self.runner_digest})
        tr.dump_json(self.accounting, budget)
        return self.action

    def test_bound_precharged_verify_executes_once(self):
        action = self.admit()
        budget_before = self.accounting.read_bytes()
        result = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(self.child_marker.read_text(), "checked")
        self.assertEqual(self.accounting.read_bytes(), budget_before, "plugin must not charge twice")
        self.child_marker.unlink()
        again = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(again.returncode, 2, again.stdout + again.stderr)
        self.assertFalse(self.child_marker.exists(), "one charge cannot authorize a second execution")
        with sqlite3.connect(self.db_path) as conn:
            receipt = json.loads(conn.execute("SELECT receipt_json FROM reservations WHERE status='finished'").fetchone()[0])
        self.assertEqual(receipt["runnerReceipt"]["runnerDigest"], self.runner_digest)
        self.assertEqual(receipt["runnerReceipt"]["chargeId"], self.plan["chargeId"])

    def test_changed_executable_source_does_not_launch(self):
        action = self.admit()
        (self.root / "approved-check.py").write_text("raise RuntimeError('replaced')")
        result = self.run_cli("run", "--action", action["id"])
        self.assertEqual(result.returncode, 2)
        self.assertFalse(self.child_marker.exists())

    def test_missing_charge_does_not_launch(self):
        action = self.admit()
        budget = tr.load_json(self.accounting)
        budget["charges"] = budget["charges"][:-1]
        tr.dump_json(self.accounting, budget)
        result = self.run_cli("run", "--action", action["id"])
        self.assertEqual(result.returncode, 2)
        self.assertFalse(self.child_marker.exists())

    def test_precharged_action_does_not_require_a_second_budget(self):
        action = self.admit()
        budget = tr.load_json(self.accounting)
        budget["planning_charge_seconds"] = 0
        budget["planning_overhead_reserved_seconds"] = 0
        used = sum(row["seconds"] for row in budget["charges"])
        budget["package_limit_seconds"] = used
        budget["master_limit_seconds"] = used + sum(row["seconds"] for row in budget["reserved"])
        tr.dump_json(self.accounting, budget)
        result = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue(self.child_marker.exists())

    def test_wrong_candidate_debit_does_not_launch(self):
        action = self.admit()
        budget = tr.load_json(self.accounting)
        budget["charges"][-1]["candidateHash"] = "f" * 64
        tr.dump_json(self.accounting, budget)
        result = self.run_cli("run", "--action", action["id"])
        self.assertEqual(result.returncode, 2)
        self.assertFalse(self.child_marker.exists())

    def test_output_is_bounded_and_overflow_is_failure(self):
        action = self.admit("print('x' * 20000)\n")
        result = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        with sqlite3.connect(self.db_path) as conn:
            receipt = json.loads(conn.execute("SELECT receipt_json FROM reservations").fetchone()[0])
        self.assertLessEqual(len(receipt["stdout"]), 4096)
        self.assertTrue(receipt["runnerReceipt"]["outputLimitExceeded"])
        self.assertEqual(receipt["runnerReceipt"]["outputBytes"]["stdout"], 20001)

    def test_supervisor_death_keeps_reservation_unresolved(self):
        action = self.admit("from pathlib import Path\nimport time\nPath('child-marker.txt').write_text('started')\ntime.sleep(2)\n")
        cli = Path(__file__).resolve().parents[1] / "scripts/moriarty_dev/cli.py"
        proc = subprocess.Popen([sys.executable, str(cli), "--repo", str(self.root), "run", "--action", action["id"]],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            deadline = time.monotonic() + 5
            while not self.child_marker.exists() and proc.poll() is None and time.monotonic() < deadline:
                time.sleep(0.01)
            self.assertTrue(self.child_marker.exists(), "actual child must have started")
            children = Path(f"/proc/{proc.pid}/task/{proc.pid}/children").read_text().split()
            self.assertEqual(len(children), 1)
            os.kill(int(children[0]), signal.SIGKILL)
            proc.communicate(timeout=10)
            with sqlite3.connect(self.db_path) as conn:
                self.assertEqual(conn.execute("SELECT status FROM reservations").fetchone()[0], "active")
        finally:
            if proc.poll() is None:
                proc.kill()
            proc.communicate(timeout=10)

    def test_launcher_error_keeps_reservation_unresolved(self):
        action = self.admit("raise SystemExit(125)\n")
        result = self.run_cli("run", "--action", action["id"])
        self.assertEqual(result.returncode, 4)
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT status FROM reservations").fetchone()[0], "active")

    def test_encoded_signal_keeps_reservation_unresolved(self):
        action = self.admit("raise SystemExit(137)\n")
        result = self.run_cli("run", "--action", action["id"])
        self.assertEqual(result.returncode, 4)
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT status FROM reservations").fetchone()[0], "active")

    def test_approved_behavioral_verifier_records_observed_defect(self):
        action = self.admit("print('defect observed')\nraise SystemExit(42)\n", "reproduce")
        result = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertTrue(json.loads(result.stdout)["reproducerVerified"])

    def test_forged_stdout_and_initialization_error_are_not_a_defect(self):
        action = self.admit("print('{\"charged\":true,\"behavioralAssertions\":[{\"outcome\":\"defect-observed\"}]}')\nraise RuntimeError('initialization failed')\n", "reproduce")
        result = self.run_cli("run", "--action", action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        with sqlite3.connect(self.db_path) as conn:
            self.assertEqual(conn.execute("SELECT count(*) FROM events WHERE event_kind='reproducer_verified'").fetchone()[0], 0)


if __name__ == "__main__":
    unittest.main()
