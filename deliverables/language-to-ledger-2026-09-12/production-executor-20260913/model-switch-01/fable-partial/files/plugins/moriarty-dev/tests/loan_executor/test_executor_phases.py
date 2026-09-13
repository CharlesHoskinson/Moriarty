"""execute_once through installed source: phases, deadlines, fault table, authority chain.

Every test drives the real loan_executor.execute_once with the real accounting
verifier, installed collector, cleanup helper and durable writer. Only the
invocation channel, service commands, clock and write worker are injected.
"""
import json
import os
import pathlib
import shutil
import subprocess
import sys
import time
import unittest

HERE = pathlib.Path(__file__).resolve().parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import harness  # noqa: E402
from harness import ExecutorRun, FakeChannel, FaultyWriter, LoanFixture  # noqa: E402
from moriarty_dev import accounting, loan_executor, store  # noqa: E402

NS = 1_000_000_000


class ExecutorCase(unittest.TestCase):
    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)

    def run_once(self, **kwargs):
        run = ExecutorRun(self.fx, **kwargs)
        run.run()
        return run

    def result_on_disk(self, run):
        doc = json.loads(self.fx.root.joinpath(self.fx.plan["resultPath"]).read_text())
        self.assertEqual(doc, run.result)
        return doc

    def master_rows(self):
        return json.loads(self.fx.master_path.read_text())["externalPackageCharges"]


class CompletePath(ExecutorCase):
    def test_success_runs_every_stage_in_order_once(self):
        run = self.run_once(script={"runningPolls": 2})
        self.assertEqual(run.outcome["exitCode"], 0)
        doc = self.result_on_disk(run)
        self.assertEqual(doc["status"], "PROCESS_SUCCESS")
        self.assertEqual(set(doc), set(loan_executor.RESULT_KEYS))
        self.assertEqual(doc["rawMainExit"], {"kind": "exit", "code": 0})
        self.assertTrue(doc["containmentComplete"] and doc["terminalEvidencePersisted"] and doc["stopReceiptPersisted"])
        self.assertEqual((doc["timerCancelReturnCode"], doc["timerCancelReceiptPersisted"]), (0, True))
        self.assertEqual(doc["outstandingOwners"], [])
        self.assertEqual((doc["retryAllowed"], doc["financialAcceptance"]), (False, "pending"))
        self.assertEqual(doc["invocationSha256"], run.digest)
        self.assertEqual(run.evidence_kinds(), [
            "runtime-financial-plan", "runtime-plan-comparison", "prover-control", "prover-prestart-intent",
            "cleanup-intent", "timer-armed", "prover-ownership", "unit-startup", "unit-ownership",
            "terminal-observation", "explicit-stop", "containment", "timer-cancel"])
        for item in doc["evidence"]:
            path = self.fx.root / item["path"]
            self.assertEqual(harness.sha256_file(path), item["sha256"])
        rows = run.calls()
        first = lambda predicate: next(i for i, row in enumerate(rows) if predicate(row))  # noqa: E731
        inspect_index = first(lambda row: "inspect" in row and "{{json .}}" in row)
        timer_index = first(lambda row: any(p.startswith("--on-active=") for p in row))
        start_index = first(lambda row: "start" in row)
        launch_index = first(lambda row: any(p.startswith("--service-type") for p in row))
        show_index = first(lambda row: "show" in row and harness.UNIT in row and "Result" in row[-1])
        stop_index = first(lambda row: "stop" in row and harness.UNIT in row)
        cancel_index = first(lambda row: "stop" in row and harness.TIMER + ".timer" in row)
        self.assertLess(inspect_index, timer_index)
        self.assertLess(timer_index, start_index)
        self.assertLess(start_index, launch_index)
        self.assertLess(launch_index, show_index)
        self.assertLess(show_index, stop_index)
        self.assertLess(stop_index, cancel_index)
        starts = [row for row in rows if "start" in row]
        self.assertEqual(len(starts), 1, "exactly one docker start")
        launches = [row for row in run.calls() if any(p.startswith("--service-type") for p in row)]
        self.assertEqual(len(launches), 1, "exactly one financial unit launch")
        self.assertEqual(run.slept, [1.0, 1.0], "one-second polls only while the exact invocation runs")
        self.assertEqual([p[0] for p in run.channel.sent], ["validated", "startup", "launched", "contained", "result"])
        self.assertTrue(run.channel.closed)

    def test_timer_is_armed_before_any_resource_start(self):
        run = self.run_once()
        rows = run.calls()
        timer_index = next(i for i, row in enumerate(rows) if any(p.startswith("--on-active=") for p in row))
        start_index = next(i for i, row in enumerate(rows) if "start" in row)
        launch_index = next(i for i, row in enumerate(rows) if any(p.startswith("--service-type") for p in row))
        self.assertLess(timer_index, start_index)
        self.assertLess(start_index, launch_index)
        armed = json.loads((self.fx.evidence_dir / "timer-armed.json").read_text())
        self.assertEqual(armed["cutoffNs"], run.start_ns + 1620 * NS)
        self.assertGreaterEqual(armed["delaySeconds"], 1618)
        self.assertLessEqual(armed["delaySeconds"], 1620)

    def test_runtime_plan_substitutes_only_the_derived_deadline(self):
        run = self.run_once()
        comparison = json.loads((self.fx.evidence_dir / "runtime-plan-comparison.json").read_text())
        self.assertEqual(comparison["changedFields"], ["limits.deadlineMs"])
        runtime = json.loads((self.fx.evidence_dir / "runtime-financial-plan.json").read_text())
        original = json.loads((self.fx.root / self.fx.financial_rel).read_text())
        original["limits"]["deadlineMs"] = runtime["limits"]["deadlineMs"]
        self.assertEqual(runtime, original)
        self.assertLessEqual(runtime["limits"]["deadlineMs"],
                             comparison["wallNowMs"] + (comparison["operationCutNs"] - comparison["monotonicNowNs"]) // 1_000_000)
        launch = json.loads((self.fx.state_dir / "launch-command.json").read_text())
        self.assertEqual(launch[-1], str(self.fx.evidence_dir / "runtime-financial-plan.json"))
        self.assertEqual(launch[-3:-1], [self.fx.python, "fake-financial.py"])
        self.assertNotIn(loan_executor.RUNTIME_PLACEHOLDER, launch)
        self.assertIn("RemainAfterExit=yes", launch)
        self.assertTrue(any(p.startswith("RuntimeMaxSec=") and int(p.split("=")[1]) <= 1606 for p in launch))

    def test_prover_control_binds_fixed_cutoffs_and_is_honoured_by_the_real_wrapper(self):
        run = self.run_once()
        control = self.fx.control_dir / "prover-control"
        lines = control.read_text().split("\n")
        self.assertEqual(lines[0], "moriarty.prover-lifetime/1")
        self.assertEqual(int(lines[3]), run.start_ns + 120 * NS)
        self.assertEqual(int(lines[4]), run.start_ns + 1620 * NS)
        self.assertEqual(lines[5], run.digest)
        self.assertEqual(oct(control.stat().st_mode & 0o777), oct(0o400))
        if not self.fx.wrapper_compiled:
            self.skipTest("gcc unavailable: static wrapper not compiled")
        wrapper = self.fx.root / self.fx.wrapper_rel
        # The written control admits an immediate start (entry < latest start).
        ok = subprocess.run([str(wrapper), str(control), "--", "/bin/true"], capture_output=True, text=True, timeout=10)
        self.assertEqual(ok.returncode, 0, ok.stderr)
        # A delayed wrapper entry at or after outer+120 refuses inside PID 1.
        late = self.fx.control_dir / "late"
        late.write_text("moriarty.prover-lifetime/1\n%s\n%d\n%d\n%d\n%s\n" % (
            lines[1], int(lines[2]), time.monotonic_ns() - 1, time.monotonic_ns() + 1500 * NS, run.digest))
        refused = subprocess.run([str(wrapper), str(late), "--", "/bin/true"], capture_output=True, text=True, timeout=10)
        self.assertEqual(refused.returncode, 64)
        self.assertIn("late entry", refused.stderr)


class InputRejection(ExecutorCase):
    """P01: invalid inputs reject before any wallet/service/start/store mutation."""

    def assert_inert(self):
        self.assertEqual(self.fx.services.calls(), [])
        self.assertEqual(sorted(os.listdir(self.fx.evidence_dir)), [])

    def test_import_and_help_are_inert(self):
        script = harness.SCRIPTS / "moriarty_dev" / "loan_executor.py"
        helped = subprocess.run([sys.executable, str(script), "--help"], capture_output=True, text=True)
        self.assertEqual(helped.returncode, 0)
        self.assertIn("--admission", helped.stdout)
        extra = subprocess.run([sys.executable, str(script), "--plan", "x", "--sha256", "y", "--admission", "z",
                                "--extra"], capture_output=True, text=True)
        self.assertEqual(extra.returncode, 2)
        self.assert_inert()

    def test_plan_rejections(self):
        good = json.loads(json.dumps(self.fx.plan))
        cases = {
            "extra-field": dict(good, extra=1),
            "missing-field": {k: v for k, v in good.items() if k != "limits"},
            "two-placeholders": dict(good, command=[self.fx.python, "{runtimeFinancialPlan}", "{runtimeFinancialPlan}"]),
            "no-placeholder": dict(good, command=[self.fx.python, "fake-financial.py"]),
            "relaxed-limit": dict(good, limits=dict(good["limits"], outerSeconds=2500)),
            "escaping-evidence": dict(good, evidenceDirectory="../outside"),
            "absolute-result": dict(good, resultPath="/tmp/result.json"),
            "unknown-prover-field": dict(good, prover=dict(good["prover"], extra=1)),
            "bad-container": dict(good, prover=dict(good["prover"], containerId="short")),
            "timer-collides": dict(good, timerUnit=good["unit"][:-len(".service")]),
        }
        for name, plan in cases.items():
            with self.subTest(case=name):
                sha = self.fx.write_plan(plan)
                with self.assertRaises(loan_executor.ExecutorRefusal) as ctx:
                    loan_executor.load_plan(self.fx.root, self.fx.plan_rel, sha)
                self.assertEqual(ctx.exception.code, "PLAN_INVALID")
        self.fx.write_plan(good)
        with self.assertRaises(loan_executor.ExecutorRefusal):
            loan_executor.load_plan(self.fx.root, self.fx.plan_rel, "0" * 64)
        link = self.fx.root / "deliverables/loan-fixture/immutable/link-plan.json"
        link.symlink_to(self.fx.root / self.fx.plan_rel)
        with self.assertRaisesRegex(loan_executor.ExecutorRefusal, "symlink"):
            loan_executor.load_plan(self.fx.root, "deliverables/loan-fixture/immutable/link-plan.json", self.fx.plan_sha)
        (self.fx.evidence_dir / "stale.json").write_text("{}")
        with self.assertRaisesRegex(loan_executor.ExecutorRefusal, "already used"):
            loan_executor.load_plan(self.fx.root, self.fx.plan_rel, self.fx.plan_sha)
        (self.fx.evidence_dir / "stale.json").unlink()
        wrong_admission = self.fx.root / "deliverables/loan-fixture/immutable/other.json"
        wrong_admission.write_text("{}")
        plan = loan_executor.load_plan(self.fx.root, self.fx.plan_rel, self.fx.plan_sha)
        with self.assertRaises(loan_executor.ExecutorRefusal):
            loan_executor.load_admission(self.fx.root, "deliverables/loan-fixture/immutable/other.json", plan)
        self.assertEqual(self.fx.services.calls(), [])

    def test_occupied_unit_or_timer_refuses_before_any_start(self):
        for name in (harness.UNIT, harness.TIMER + ".timer"):
            with self.subTest(occupied=name):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                fx.services.set_unit(name, dict(harness.fake_services.NOT_FOUND, LoadState="loaded", ActiveState="inactive"))
                run = ExecutorRun(fx)
                run.run()
                self.assertEqual(run.outcome["exitCode"], 2)
                self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "UNIT_OCCUPIED"))
                self.assertEqual([r for r in run.calls() if "start" in r or "systemd-run" in r[3:4]], [])
                self.assertFalse((fx.control_dir / "prover-control").exists(), "no control write before occupancy check")


class DeadlineSchedule(ExecutorCase):
    """P05: monotonic absolute cutoffs anchored to the outer start."""

    def test_entry_at_plus_fifty_keeps_absolute_cutoffs(self):
        run = self.run_once(entry_offset_ns=50 * NS)
        self.assertEqual(run.result["status"], "PROCESS_SUCCESS")
        control = (self.fx.control_dir / "prover-control").read_text().split("\n")
        self.assertEqual(int(control[3]), run.start_ns + 120 * NS)
        self.assertEqual(int(control[4]), run.start_ns + 1620 * NS)
        armed = json.loads((self.fx.evidence_dir / "timer-armed.json").read_text())
        self.assertLessEqual(armed["delaySeconds"], 1570)
        launch = json.loads((self.fx.state_dir / "launch-command.json").read_text())
        runtime_max = next(int(p.split("=")[1]) for p in launch if p.startswith("RuntimeMaxSec="))
        self.assertLessEqual(runtime_max, 1556)

    def test_entry_at_plus_120_refuses_without_start(self):
        run = self.run_once(entry_offset_ns=120 * NS)
        self.assertEqual(run.outcome["exitCode"], 2)
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "DEADLINE_EXCEEDED"))
        self.assertEqual(run.calls(), [])
        self.assertEqual(self.master_rows()[0]["launchClaim"], "claimed", "no refund or reset on refusal")

    def test_short_block_deadline_refuses_the_complete_schedule(self):
        run = self.run_once(outer_deadline_ns=time.monotonic_ns() + 1700 * NS)
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "DEADLINE_EXCEEDED"))
        self.assertEqual(run.calls(), [])

    def test_stale_observation_denies_without_refresh_or_refund(self):
        # Age 119 at handover and at financial startup: accepted.
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.write_observation(age_seconds=119)
        fx.write_correspondence()
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual(run.result["status"], "PROCESS_SUCCESS")
        # Age 121 at the parent handover: the runner-side verifier denies first.
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.write_observation(age_seconds=121)
        fx.write_correspondence()
        with self.assertRaisesRegex(AssertionError, "OBSERVATION_INVALID"):
            fx.reserve_and_handover()
        self.assertEqual(json.loads(fx.master_path.read_text())["externalPackageCharges"][0]["launchClaim"], "unclaimed")
        # Fresh at handover, but setup consumed the window: the child denies at
        # financial startup without refreshing the observation or refunding.
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, wall_offset_ms=121_000)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "OBSERVATION_INVALID"))
        self.assertEqual(run.calls(), [])
        self.assertEqual(harness.sha256_file(fx.root / fx.receipt_path), fx.observation_sha, "no implicit refresh")
        rows = json.loads(fx.master_path.read_text())["externalPackageCharges"]
        self.assertEqual((rows[0]["seconds"], rows[0]["launchClaim"]), (harness.RUNTIME_SECONDS, "claimed"))


class ParentLoss(ExecutorCase):
    """P04: parent loss before versus after resource startup."""

    def test_parent_lost_before_startup_refuses(self):
        run = self.run_once(channel=None)
        self.assertEqual(run.result["status"], "PROCESS_SUCCESS")  # control
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx)
        run.channel.alive = False
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "PARENT_LOST"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])

    def test_parent_lost_after_startup_cleans_owned_resources_and_stays_unknown(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"runningPolls": 5})
        run.channel.lose_after = "launched"
        run.run()
        self.assertEqual(run.outcome["exitCode"], 3)
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "PARENT_LOST"))
        self.assertTrue(run.result["containmentComplete"])
        self.assertEqual(run.result["rawMainExit"], {"kind": "unknown", "code": None})
        kills = [r for r in run.calls() if "kill" in r]
        self.assertEqual(len(kills), 2, "identity-checked safety cleanup of unit and prover")
        containment = json.loads((fx.evidence_dir / "containment-safety-cleanup.json").read_text())
        self.assertEqual(containment["purpose"], "safety-cleanup")
        self.assertFalse((fx.evidence_dir / "collector" / "explicit-stop.json").exists(),
                         "cleanup is never the collector stop")

    def test_hung_writer_is_killed_at_its_bound_and_never_counts_as_durable(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        writer = FaultyWriter(timeout=["unit-ownership.json"])
        started = time.monotonic()
        run = ExecutorRun(fx, writer=writer)
        run.run()
        self.assertLess(time.monotonic() - started, 10)
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "EVIDENCE_WRITE_FAILED"))
        self.assertNotIn("unit-ownership", run.evidence_kinds())
        self.assertTrue(any(l.get("artifact") == "unit-ownership" for l in run.outcome["limitations"]))


class FaultTable(ExecutorCase):
    """P06/P07/P08 rows of the deterministic fault disposition table."""

    def test_nonzero_and_signal_main_exits_stay_failed_despite_zero_cleanup(self):
        for terminal, code, raw in (({"kind": "exit", "code": 7}, "MAIN_EXIT_NONZERO", {"kind": "exit", "code": 7}),
                                    ({"kind": "signal", "code": 11}, "MAIN_SIGNAL", {"kind": "signal", "code": 11})):
            with self.subTest(terminal=terminal):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, script={"terminal": terminal})
                run.run()
                self.assertEqual(run.outcome["exitCode"], 1)
                self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_FAILED", code))
                self.assertEqual(run.result["rawMainExit"], raw)
                self.assertTrue(run.result["containmentComplete"])
                self.assertEqual(run.result["timerCancelReturnCode"], 0)
                self.assertIsNone(run.result["stopReturnCode"], "no collector stop after a failed main exit")
                self.assertTrue(run.result["terminalEvidencePersisted"])

    def test_unknown_observations_are_unknown_never_zero(self):
        for kind, code in (("unloaded", "MAIN_OBSERVATION_INVALID"), ("transitional", "MAIN_OBSERVATION_INVALID"),
                           ("malformed", "MAIN_OBSERVATION_INVALID"), ("default-identity", "MAIN_OBSERVATION_INVALID"),
                           ("changed-invocation", "MAIN_OBSERVATION_INVALID"), ("exit-wrong-result", "MAIN_OBSERVATION_INVALID")):
            with self.subTest(kind=kind):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, script={"terminal": {"kind": kind}})
                run.run()
                self.assertEqual(run.outcome["exitCode"], 3)
                self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", code))
                self.assertEqual(run.result["rawMainExit"], {"kind": "unknown", "code": None})
                self.assertIsNone(run.result["stopReturnCode"])

    def test_show_failure_or_timeout_is_terminal_without_retry(self):
        for script, code in (({"showReturnCode": 3}, "MAIN_OBSERVATION_UNAVAILABLE"),
                             ({"showTimeout": True}, "MAIN_OBSERVATION_UNAVAILABLE")):
            with self.subTest(script=script):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, script=script)
                run.run()
                self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", code))
                terminal_shows = [r for r in run.calls() if "show" in r and harness.UNIT in r and "Result" in r[-1]]
                self.assertEqual(len(terminal_shows), 1, "no retry of the terminal observation")

    def test_collection_cutoff_with_only_running_observations(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"runningPolls": 999999}, jump_after_launch_ns=1615 * NS)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "MAIN_EXIT_UNAVAILABLE"))
        self.assertTrue(run.result["containmentComplete"])
        self.assertLessEqual(len(run.slept), 4, "polling stops at the collection cutoff")
        self.assertGreaterEqual(len(run.slept), 1)
        self.assertEqual(run.result["rawMainExit"], {"kind": "unknown", "code": None})
        # A clock jump past the cleanup cutoff as well: bounded, no hang, unknown.
        fx2 = LoanFixture().build()
        self.addCleanup(fx2.cleanup)
        run2 = ExecutorRun(fx2, script={"runningPolls": 999999}, jump_after_launch_ns=1750 * NS)
        started = time.monotonic()
        run2.run()
        self.assertLess(time.monotonic() - started, 10)
        self.assertEqual(run2.result["status"], "PROCESS_UNKNOWN")
        self.assertEqual(run2.outcome["exitCode"], 3)

    def test_terminal_write_failures_and_preexisting_artifacts(self):
        cases = {
            "terminal-observation.json": ("EVIDENCE_WRITE_FAILED", 3),
            "explicit-stop.json": ("STOP_RECEIPT_FAILED", 3),
            "containment-normal-cleanup.json": ("CONTAINMENT_UNRESOLVED", 3),
            "timer-cancel.json": ("TIMER_CANCEL_FAILED", 3),
            "prover-control": ("PROVER_CONTROL_INVALID", 2),
            "prover-prestart-intent.json": ("EVIDENCE_WRITE_FAILED", 2),
            "cleanup-intent.json": ("EVIDENCE_WRITE_FAILED", 2),
            "prover-ownership.json": ("EVIDENCE_WRITE_FAILED", 3),
            "unit-startup.json": ("EVIDENCE_WRITE_FAILED", 3),
            "runtime-financial-plan.json": ("EVIDENCE_WRITE_FAILED", 2),
        }
        for name, (code, exit_code) in cases.items():
            with self.subTest(artifact=name):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, writer=FaultyWriter(fail=[name]))
                run.run()
                self.assertEqual(run.outcome["exitCode"], exit_code, run.result)
                self.assertNotEqual(run.result["status"], "PROCESS_SUCCESS")
                self.assertEqual(run.result["failureCode"], code)
                if exit_code == 2:
                    self.assertEqual([r for r in run.calls() if "start" in r], [], "no start after failed prevalidation")
                    self.assertEqual(run.result["status"], "REFUSED")
                if name == "terminal-observation.json":
                    self.assertIsNone(run.result["stopReturnCode"], "no collector stop without durable terminal evidence")
                    self.assertEqual(len([r for r in run.calls() if "kill" in r]), 2, "immediate safety cleanup")
                if name == "explicit-stop.json":
                    self.assertEqual(run.result["stopReturnCode"], 0)
                    self.assertFalse(run.result["stopReceiptPersisted"])
                    stops = [r for r in run.calls() if "stop" in r and harness.UNIT in r]
                    self.assertEqual(len(stops), 1, "stop is not repeated for evidence")
                if name == "timer-cancel.json":
                    self.assertTrue(run.result["containmentComplete"])
                    self.assertTrue(any(o["resource"] == "timer" for o in run.result["outstandingOwners"]))
        # Preexisting collector artifacts in the evidence directory are a reused
        # directory: refused by plan validation before any invocation.
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        (fx.evidence_dir / "collector").mkdir()
        (fx.evidence_dir / "collector" / "explicit-stop.json").write_text("{}")
        with self.assertRaisesRegex(loan_executor.ExecutorRefusal, "already used"):
            loan_executor.load_plan(fx.root, fx.plan_rel, fx.plan_sha)
        self.assertEqual(fx.services.calls(), [])

    def test_stop_failure_after_durable_zero_terminal(self):
        for script, code in (({"stopReturnCode": 5}, "STOP_FAILED"), ({"stopTimeout": True}, "STOP_FAILED")):
            with self.subTest(script=script):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, script=script)
                run.run()
                self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", code))
                self.assertTrue(run.result["terminalEvidencePersisted"])
                self.assertEqual(run.result["rawMainExit"], {"kind": "exit", "code": 0})
                stop = json.loads((fx.evidence_dir / "collector" / "explicit-stop.json").read_text())
                self.assertEqual(stop["argv"][:2], ["/usr/bin/timeout", "--signal=KILL"])
                self.assertLessEqual(int(stop["argv"][2][:-1]), 25)

    def test_prover_prevalidation_failures_never_start(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        config = fx.services._load("container.json", None)
        config["HostConfig"]["RestartPolicy"]["Name"] = "always"
        fx.services.seed_container(config)
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "CONTAINMENT_UNSUPPORTED"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])
        for patch in ({"State": {"Status": "running", "Running": True, "StartedAt": "2026-09-13T00:00:00Z"}},
                      {"HostConfig": dict(config["HostConfig"], PidMode="host", RestartPolicy={"Name": "no"})},
                      {"Mounts": config["Mounts"][:1]}, {"Image": "sha256:" + "9" * 64}):
            with self.subTest(patch=list(patch)):
                fx2 = LoanFixture().build()
                self.addCleanup(fx2.cleanup)
                cfg = fx2.services._load("container.json", None)
                cfg.update(patch)
                fx2.services.seed_container(cfg)
                run2 = ExecutorRun(fx2)
                run2.run()
                self.assertEqual(run2.result["failureCode"], "CONTAINMENT_UNSUPPORTED")
                self.assertEqual([r for r in run2.calls() if "start" in r], [])

    def test_timer_arm_failure_refuses_without_resources(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"timerArmReturnCode": 1})
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "CONTAINMENT_UNSUPPORTED"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])

    def test_docker_start_failure_is_unknown_with_bounded_control(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"dockerStartReturnCode": 1})
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "STARTUP_INVALID"))
        self.assertTrue((fx.control_dir / "prover-control").exists())
        self.assertEqual([r for r in run.calls() if any(p.startswith("--service-type") for p in r)], [])

    def test_ownership_receipt_loss_after_start_keeps_pid1_bound_and_cleans_current_identity(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, writer=FaultyWriter(fail=["prover-ownership.json"]))
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "EVIDENCE_WRITE_FAILED"))
        control = (fx.control_dir / "prover-control").read_text().split("\n")
        self.assertEqual(int(control[4]), run.start_ns + 1620 * NS, "PID 1 bound exists independent of the receipt")
        self.assertTrue(run.result["containmentComplete"], "in-process start observation permits identity-checked S")
        self.assertEqual(len([r for r in run.calls() if "docker" in r[3] and "kill" in r]), 1)

    def test_replacement_generation_is_not_killed_and_ownership_stays_unresolved(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"replacementAfterStart": True})
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_UNKNOWN", "CONTAINMENT_UNRESOLVED"))
        self.assertFalse(run.result["containmentComplete"])
        self.assertEqual([r for r in run.calls() if "docker" in r[3] and "kill" in r], [])
        owners = {o["resource"] for o in run.result["outstandingOwners"]}
        self.assertIn("container:" + harness.CONTAINER_ID, owners)
        self.assertIn("timer", owners)
        self.assertIsNone(run.result["timerCancelReturnCode"], "timer retained until containment")

    def test_known_failure_survives_incomplete_containment(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"terminal": {"kind": "exit", "code": 3}, "replacementAfterStart": True})
        run.run()
        self.assertEqual(run.outcome["exitCode"], 1)
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("PROCESS_FAILED", "MAIN_EXIT_NONZERO"))
        self.assertFalse(run.result["containmentComplete"])
        self.assertTrue(run.result["outstandingOwners"])


class AuthorityChain(ExecutorCase):
    """P03/P12: forged, stale, replayed and defective current-state inputs."""

    def prepared(self):
        if not hasattr(self, "_run"):
            self._run = ExecutorRun(self.fx)
        return self._run

    def refusal(self, patch_payload=None):
        payload = dict(self.prepared().payload)
        if patch_payload:
            payload.update(patch_payload)
        return accounting.validate_executor_context(self.fx.loaded_plan, self.fx.loaded_admission, payload,
                                                    root=self.fx.root)

    def test_authentic_fixture_is_accepted_by_the_shared_verifier(self):
        run = self.prepared()
        current = accounting.validate_executor_context(self.fx.loaded_plan, self.fx.loaded_admission, run.payload,
                                                       root=self.fx.root)
        self.assertIsInstance(current, accounting.CurrentExecution, current)
        self.assertEqual(current["prepaidAvailableSeconds"], harness.GRANT_SECONDS)
        self.assertEqual(current["envelopeRemainingSeconds"], harness.GRANT_SECONDS - harness.RUNTIME_SECONDS)

    def test_defects_are_named_before_startup(self):
        self.assertEqual(self.refusal({"authoritySha256": "0" * 64}).code, "AUTHORITY_INVALID")
        self.assertEqual(self.refusal({"storeIdentity": "other-token"}).code, "AUTHORITY_INVALID")
        self.assertEqual(self.refusal({"reservationId": "another"}).code, "OWNERSHIP_UNRESOLVED")
        self.assertEqual(self.refusal({"parentPid": os.getpid() + 1}).code, "OWNERSHIP_UNRESOLVED")
        self.assertEqual(self.refusal({"chargeId": "runtime-charge-99"}).code, "OWNERSHIP_UNRESOLVED")
        stale = self.refusal({"correspondenceSha256": "1" * 64})
        self.assertEqual(stale.code, "CLAIM_INVALID")
        unrecorded = self.refusal({"nonceSha256": "2" * 64})
        self.assertEqual((unrecorded.code, "durable invocation event" in unrecorded.detail), ("CLAIM_INVALID", True))

    def test_history_observation_and_claim_defects(self):
        fx = self.fx
        self.prepared()
        history_path = fx.root / "deliverables/loan-fixture/immutable/history.json"
        original = history_path.read_text()
        history = json.loads(original)
        history["eventBindings"][0]["eventIds"] = [9999]
        history_path.write_text(json.dumps(history))
        # Changing history bytes breaks the immutable reference first (AUTHORITY_INVALID);
        # a matching digest with missing rows is HISTORY_UNRESOLVED.
        self.assertEqual(self.refusal().code, "AUTHORITY_INVALID")
        history_path.write_text(original)
        context_path = fx.root / "deliverables/loan-fixture/immutable/execution-context.json"
        chain = accounting.authenticate_authority(fx.root, fx.loaded_plan, fx.loaded_admission)
        chain["history"]["eventBindings"][0]["eventIds"] = [9999]
        self.assertEqual(accounting.verify_history(chain).code, "HISTORY_UNRESOLVED")
        chain = accounting.authenticate_authority(fx.root, fx.loaded_plan, fx.loaded_admission)
        chain["history"]["outcomes"] = []
        self.assertEqual(accounting.verify_history(chain).code, "HISTORY_UNRESOLVED")
        fx.write_observation(roleIdentity={"publicKey": "b" * 64, "address": "mn_addr_other"})
        self.assertEqual(self.refusal().code, "OBSERVATION_INVALID")

    def test_alternate_or_restored_sqlite_cannot_reuse_the_claim(self):
        fx = self.fx
        run = self.prepared()
        backup = fx.db_path.read_bytes()
        self.assertIsInstance(accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, run.payload,
                                                                   root=fx.root), accounting.CurrentExecution)
        # A restored pre-invocation backup lacks the durable event: CLAIM_INVALID.
        import sqlite3
        with sqlite3.connect(fx.db_path) as conn:
            conn.execute("DELETE FROM events WHERE event_kind = 'loan_invocation'")
        conn.close()
        self.assertEqual(accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, run.payload,
                                                              root=fx.root).code, "CLAIM_INVALID")
        # A store without the adopted token is not the canonical store.
        with sqlite3.connect(fx.db_path) as conn:
            conn.execute("DROP TABLE store_identity")
        conn.close()
        self.assertEqual(accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, run.payload,
                                                              root=fx.root).code, "AUTHORITY_INVALID")
        del backup

    def test_master_claim_defects_and_own_prepaid_equality(self):
        fx = self.fx
        run = self.prepared()
        master = json.loads(fx.master_path.read_text())
        good = json.dumps(master)

        def apply(mutation):
            current = json.loads(good)
            mutation(current)
            fx.master_path.write_text(json.dumps(current, indent=2, sort_keys=True) + "\n")
            corr = json.loads((fx.root / fx.correspondence_path).read_text())
            corr["masterSha256"] = harness.sha256_file(fx.master_path)
            (fx.root / fx.correspondence_path).write_text(json.dumps(corr, indent=2, sort_keys=True) + "\n")
            payload = dict(run.payload, correspondenceSha256=harness.sha256_file(fx.root / fx.correspondence_path))
            store.record_invocation(fx.db_path, str(fx.root), fx.action, payload)
            return accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, payload, root=fx.root)

        def duplicate(m):
            m["externalPackageCharges"].append(dict(m["externalPackageCharges"][0]))
        self.assertEqual(apply(duplicate).code, "CLAIM_INVALID")

        def unclaimed(m):
            m["externalPackageCharges"][0]["launchClaim"] = "unclaimed"
        self.assertEqual(apply(unclaimed).code, "CLAIM_INVALID")

        def overcommit(m):
            m["externalPackageCharges"].append(dict(m["externalPackageCharges"][0], id="other", seconds=9000))
        self.assertEqual(apply(overcommit).code, "CLAIM_INVALID")

        def counters_lie(m):
            m["successorEnvelopes"]["mc02-envelope-01"]["remainingReservedSeconds"] = 0
        self.assertEqual(apply(counters_lie).code, "CLAIM_INVALID")

        def own_equality(m):
            m["externalPackageCharges"][-1]["seconds"] = harness.GRANT_SECONDS
            m["successorEnvelopes"]["mc02-envelope-01"]["remainingReservedSeconds"] = 0
            m["reserved"][-1]["seconds"] = 0
        current = apply(own_equality)
        self.assertIsInstance(current, accounting.CurrentExecution, "S=G is valid for the own prepaid invocation")
        self.assertEqual(current["envelopeRemainingSeconds"], 0)
        self.assertEqual(current["prepaidAvailableSeconds"], harness.GRANT_SECONDS, "own charge is not subtracted twice")

    def test_interrupted_generation_denies_until_reconciliation(self):
        fx = self.fx
        run = self.prepared()
        master = json.loads(fx.master_path.read_text())
        master["reserved"].append({"id": "later", "seconds": 1, "basis": "x", "sourceSha256": "0" * 64, "amendmentSha256": "0" * 64})
        fx.master_path.write_text(json.dumps(master))
        refusal = accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, run.payload, root=fx.root)
        self.assertEqual(refusal.code, "CLAIM_INVALID")
        self.assertIn("reconciliation", refusal.detail)

    def test_runtime_handover_is_one_shot_and_denies_competing_reservations(self):
        fx = self.fx
        fx.reserve_and_handover()
        again = accounting.runtime_handover(fx.root, fx.loaded_plan, fx.loaded_admission, harness.RUNTIME_CHARGE,
                                            "second-reservation", fx.runner_digest, os.getpid(), 7)
        self.assertEqual(again.code, "CLAIM_INVALID")
        self.assertIn("no second claim", again.detail)
        self.assertEqual(json.loads(fx.master_path.read_text())["externalPackageCharges"][0]["reservationId"],
                         fx.reservation_id)
        payload = fx.invocation_payload(b"\x00" * 32, reservationId="second-reservation")
        self.assertEqual(accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, payload,
                                                              root=fx.root).code, "OWNERSHIP_UNRESOLVED")

    def test_vote_and_context_defects_are_authority_invalid(self):
        fx = self.fx
        self.prepared()
        vote_path = fx.root / "deliverables/loan-fixture/immutable/vote-gpt-6-astra.json"
        original = vote_path.read_text()
        vote = json.loads(original)
        vote["model"] = "claude-opus-5"
        vote_path.write_text(json.dumps(vote))
        self.assertEqual(self.refusal().code, "AUTHORITY_INVALID")  # digest mismatch
        vote_path.write_text(original)
        chain = accounting.authenticate_authority(fx.root, fx.loaded_plan, fx.loaded_admission)
        self.assertNotIsInstance(chain, accounting.Refusal)
        # Two votes from one vendor (or one duplicated identity) are not two independent votes.
        for models in (("claude-opus-5", "claude-fable-5-1"), ("gpt-6-astra", "gpt-6-astra")):
            def two(mods=models):
                admission = json.loads((fx.root / fx.admission_rel).read_text())
                votes = []
                for index, model in enumerate(mods):
                    path = "deliverables/loan-fixture/immutable/vote-alt-%d.json" % index
                    digest = harness.write_json(fx.root / path, {
                        "schema": accounting.VOTE_SCHEMA, "reviewer": model, "model": model,
                        "role": accounting.VOTE_ROLE, "scope": accounting.VOTE_SCOPE,
                        "candidateSha256": fx.projection_sha, "verdict": "APPROVED", "findings": []})
                    votes.append({"path": path, "sha256": digest})
                admission["resources"]["votes"] = votes
                return accounting.Authenticated(admission, fx.admission_sha, fx.admission_rel)
            self.assertEqual(accounting.authenticate_authority(fx.root, fx.loaded_plan, two()).code, "AUTHORITY_INVALID")
        # The current routing pair (claude-fable-5-1 + gpt-6) is an admissible distinct-vendor pair.
        admission = json.loads((fx.root / fx.admission_rel).read_text())
        votes = []
        for index, model in enumerate(("claude-fable-5-1", "gpt-6")):
            path = "deliverables/loan-fixture/immutable/vote-route-%d.json" % index
            digest = harness.write_json(fx.root / path, {
                "schema": accounting.VOTE_SCHEMA, "reviewer": model + " high", "model": model,
                "role": accounting.VOTE_ROLE, "scope": accounting.VOTE_SCOPE,
                "candidateSha256": fx.projection_sha, "verdict": "APPROVED", "findings": []})
            votes.append({"path": path, "sha256": digest})
        admission["resources"]["votes"] = votes
        routed = accounting.Authenticated(admission, fx.admission_sha, fx.admission_rel)
        self.assertNotIsInstance(accounting.authenticate_authority(fx.root, fx.loaded_plan, routed), accounting.Refusal)
        # Swapped context: a different context digest committed by the projection.
        admission = json.loads((fx.root / fx.admission_rel).read_text())
        admission["resources"]["executionContext"]["sha256"] = "3" * 64
        other = accounting.Authenticated(admission, fx.admission_sha, fx.admission_rel)
        self.assertEqual(accounting.authenticate_authority(fx.root, fx.loaded_plan, other).code, "AUTHORITY_INVALID")


if __name__ == "__main__":
    unittest.main()
