"""Existing cmd_run -> runner -> pinned Foreman launcher -> real main -> durable result -> history.

Each case runs the actual plugin CLI as a subprocess against an isolated
fixture repository, through the real strong-containment launcher, with the
real installed executor whose service transports are routed to fake
executables. Store integrity is checked from a fresh process afterwards.
"""
import json
import os
import pathlib
import sqlite3
import subprocess
import sys
import unittest
from contextlib import closing

HERE = pathlib.Path(__file__).resolve().parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import harness  # noqa: E402
from harness import LoanFixture  # noqa: E402

CLI = harness.SCRIPTS / "moriarty_dev" / "cli.py"
INTEGRITY = ("import sqlite3, sys; c = sqlite3.connect(sys.argv[1]); "
             "print(c.execute('pragma integrity_check').fetchall()[0][0]); c.close()")


class CliCase(unittest.TestCase):
    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)

    def run_cli(self, *args):
        return subprocess.run([sys.executable, str(CLI), "--repo", str(self.fx.root), *args],
                              capture_output=True, text=True, timeout=180)

    def rows(self, query, *params):
        with closing(sqlite3.connect(self.fx.db_path)) as conn:
            return conn.execute(query, params).fetchall()

    def events(self):
        return [row[0] for row in self.rows("SELECT event_kind FROM events ORDER BY id")]

    def event_payload(self, kind):
        return json.loads(self.rows("SELECT payload_json FROM events WHERE event_kind = ?", kind)[-1][0])

    def integrity(self):
        fresh = subprocess.run([sys.executable, "-c", INTEGRITY, str(self.fx.db_path)], capture_output=True, text=True)
        self.assertEqual(fresh.stdout.strip(), "ok", fresh.stderr)

    def master_row(self):
        return json.loads(self.fx.master_path.read_text())["externalPackageCharges"][0]


class Consumer(CliCase):
    def test_p09_child_zero_with_valid_success_gives_cli_zero_and_finished_reservation(self):
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["success"], out["disposition"], out["status"], out["reservationStatus"]),
                         (True, "success", "PROCESS_SUCCESS", "finished"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("finished",)])
        kinds = self.events()
        self.assertIn("loan_invocation", kinds)
        self.assertIn("loan_process_success", kinds)
        self.assertLess(kinds.index("loan_invocation"), kinds.index("loan_process_success"))
        payload = self.event_payload("loan_process_success")
        self.assertEqual(payload["resultSha256"], harness.sha256_file(self.fx.root / payload["resultPath"]))
        self.assertEqual(payload["invocationSha256"], out["invocationSha256"])
        self.assertIsInstance(payload["invocationEventId"], int)
        document = json.loads((self.fx.root / payload["resultPath"]).read_text())
        self.assertEqual((document["financialAcceptance"], document["retryAllowed"]), ("pending", False))
        self.assertEqual(json.loads(self.fx.ownership_path.read_text())["state"], "released")
        self.integrity()
        # One claim: the same charge cannot launch again.
        again = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(again.returncode, 2, again.stdout + again.stderr)
        self.assertIn("already claimed", again.stderr)
        self.assertEqual(len([r for r in self.fx.services.calls() if "start" in r]), 1)

    def test_p10_child_one_with_verified_containment_gives_cli_four_and_failed_reservation(self):
        self.fx.services._save("script.json", {"terminal": {"kind": "signal", "code": 9}})
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["disposition"], out["status"], out["failureCode"], out["reservationStatus"]),
                         (1, "failed", "PROCESS_FAILED", "MAIN_SIGNAL", "failed"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("failed",)])
        self.assertIn("loan_process_failed", self.events())
        self.assertEqual(self.master_row()["launchClaim"], "claimed", "failure keeps the debit consumed")
        self.integrity()

    def test_p10_child_two_with_valid_refused_gives_cli_four_and_a_distinct_event(self):
        self.fx.services.set_unit(harness.UNIT, dict(harness.fake_services.NOT_FOUND, LoadState="loaded",
                                                     ActiveState="inactive"))
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["disposition"], out["status"], out["failureCode"]),
                         (2, "refused", "REFUSED", "UNIT_OCCUPIED"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("failed",)])
        self.assertIn("refused_before_start", self.events())
        self.assertNotIn("runner_unresolved", self.events())
        self.assertEqual([r for r in self.fx.services.calls() if "start" in r], [])
        self.integrity()

    def test_policy_denial_before_invocation_remains_exit_two(self):
        # An exhausted legacy resource envelope denies the action before any child.
        import test_records as tr
        accounting_path = self.fx.root / tr.CURRENT_ACCOUNTING_REL
        budget = json.loads(accounting_path.read_text())
        budget["master_limit_seconds"] = 100
        accounting_path.write_text(json.dumps(budget))
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("Policy denied", result.stdout)
        self.assertIn("RESOURCE_EXHAUSTED", result.stdout)
        self.assertEqual(self.rows("SELECT count(*) FROM reservations"), [(0,)])
        self.assertNotIn("loan_invocation", self.events())

    def test_p11_child_three_keeps_reservation_active_as_runner_unresolved(self):
        self.fx.services._save("script.json", {"terminal": {"kind": "unloaded"}})
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["disposition"], out["status"], out["reservationStatus"]),
                         (3, "unresolved", "PROCESS_UNKNOWN", "active"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("active",)])
        self.assertIn("runner_unresolved", self.events())
        self.assertEqual(json.loads(self.fx.ownership_path.read_text())["state"], "runtime-held")
        self.integrity()

    def test_p11_child_zero_with_missing_or_forged_result_is_unresolved(self):
        forged = self.fx.root / "forged-driver.py"
        forged.write_text("""import json, sys
from pathlib import Path
plan = json.loads(Path(sys.argv[sys.argv.index('--plan') + 1]).read_text())
document = {k: None for k in %r}
document.update({"schema": "moriarty.loan-process-result/1", "allocationId": plan["allocationId"],
                 "actionId": plan["actionId"], "candidateHash": plan["candidateHash"], "unit": plan["unit"],
                 "status": "PROCESS_SUCCESS", "rawMainExit": {"kind": "exit", "code": 0},
                 "terminalEvidencePersisted": True, "stopReturnCode": 0, "stopReceiptPersisted": True,
                 "containmentComplete": True, "timerCancelReturnCode": 0, "timerCancelReceiptPersisted": True,
                 "evidence": [], "outstandingOwners": [], "retryAllowed": False, "financialAcceptance": "pending",
                 "runnerDigest": "0" * 64, "chargeId": "runtime-charge-01", "reservationId": "forged",
                 "invocationSha256": "0" * 64})
Path(plan["resultPath"]).write_text(json.dumps(document))
print(json.dumps({"status": "PROCESS_SUCCESS"}))
raise SystemExit(0)
""" % (list(__import__("moriarty_dev.loan_executor", fromlist=["RESULT_KEYS"]).RESULT_KEYS),))
        argv = [self.fx.python, "forged-driver.py", "--plan", self.fx.plan_rel, "--sha256", self.fx.plan_sha,
                "--admission", self.fx.admission_rel]
        self.fx.write_runner(argv=argv)
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["disposition"], out["reservationStatus"]), (0, "unresolved", "active"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("active",)])
        payload = self.event_payload("runner_unresolved")
        self.assertIn("differs from the current invocation", payload["problem"])
        # Child zero with no result at all.
        missing = LoanFixture().build()
        self.addCleanup(missing.cleanup)
        (missing.root / "silent-driver.py").write_text("print('{\"status\": \"PROCESS_SUCCESS\"}')\nraise SystemExit(0)\n")
        missing.write_runner(argv=[missing.python, "silent-driver.py", "--plan", missing.plan_rel, "--sha256",
                                   missing.plan_sha, "--admission", missing.admission_rel])
        result = subprocess.run([sys.executable, str(CLI), "--repo", str(missing.root), "run", "--action",
                                 missing.action["id"], "--json"], capture_output=True, text=True, timeout=180)
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        self.assertEqual(json.loads(result.stdout)["disposition"], "unresolved")
        with closing(sqlite3.connect(missing.db_path)) as conn:
            self.assertEqual(conn.execute("SELECT status FROM reservations").fetchall(), [("active",)])

    def test_p11_exit_status_mismatch_is_unresolved(self):
        mismatch = self.fx.root / "mismatch-driver.py"
        mismatch.write_text("""import sys
sys.path.insert(0, %r)
from pathlib import Path
from moriarty_dev import loan_executor
deps = loan_executor.production_dependencies(Path.cwd(), service_table=%r)
code = loan_executor.main(sys.argv[1:], dependencies=deps)
raise SystemExit(0)  # child zero despite a PROCESS_FAILED result
""" % (str(harness.SCRIPTS), self.fx.service_table))
        self.fx.services._save("script.json", {"terminal": {"kind": "exit", "code": 4}})
        self.fx.write_runner(argv=[self.fx.python, "mismatch-driver.py", "--plan", self.fx.plan_rel, "--sha256",
                                   self.fx.plan_sha, "--admission", self.fx.admission_rel])
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["status"], out["disposition"], out["reservationStatus"]),
                         (0, "PROCESS_FAILED", "unresolved", "active"))

    def test_p11_known_main_failure_with_unresolved_containment_retains_both(self):
        self.fx.services._save("script.json", {"terminal": {"kind": "exit", "code": 5}, "replacementAfterStart": True})
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["exitCode"], out["status"], out["failureCode"], out["disposition"], out["reservationStatus"]),
                         (1, "PROCESS_FAILED", "MAIN_EXIT_NONZERO", "failed_unresolved", "active"))
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("active",)])
        payload = self.event_payload("loan_failure_with_unresolved_ownership")
        self.assertEqual(payload["failureCode"], "MAIN_EXIT_NONZERO")
        self.assertTrue(any(o["resource"].startswith("container:") for o in payload["outstandingOwners"]))
        self.assertEqual(json.loads(self.fx.ownership_path.read_text())["state"], "runtime-held")
        self.assertEqual(self.master_row()["launchClaim"], "claimed")
        self.integrity()

    def test_authority_refusal_before_launch_is_exit_two_without_child(self):
        self.fx.write_observation(age_seconds=400)
        self.fx.write_correspondence()
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("OBSERVATION_INVALID", result.stderr)
        self.assertEqual(self.rows("SELECT status FROM reservations"), [("failed",)])
        self.assertEqual(self.fx.services.calls(), [])
        self.assertEqual(self.master_row()["launchClaim"], "unclaimed", "no launch claim without a launch")
        self.assertNotIn("loan_invocation", self.events())

    def test_wallet_lock_held_by_another_owner_refuses_before_launch(self):
        import fcntl
        holder = os.open(str(self.fx.lock_path), os.O_RDWR | os.O_CREAT, 0o600)
        fcntl.flock(holder, fcntl.LOCK_EX)
        self.addCleanup(os.close, holder)
        result = self.run_cli("run", "--action", self.fx.action["id"], "--json")
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertIn("OWNERSHIP_UNRESOLVED", result.stderr)
        self.assertEqual(self.fx.services.calls(), [])
        self.assertEqual(self.master_row()["launchClaim"], "unclaimed")


if __name__ == "__main__":
    unittest.main()
