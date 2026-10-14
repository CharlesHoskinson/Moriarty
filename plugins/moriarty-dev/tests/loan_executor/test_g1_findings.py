"""Isolated regressions for G1-01..G1-13 and TD-01..TD-06.

Each case drives the actual production reader, runner, CLI, executor,
accounting or store consumer on a fresh fixture path. Independent probe
roots under /tmp/moriarty-production-grok-r1-independent-probes/ are not
reused. Fixture failure artifacts stay local; real wallet/budget/DB are
never inspected.
"""
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

HERE = Path(__file__).resolve().parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import harness  # noqa: E402
from harness import ExecutorRun, LoanFixture  # noqa: E402
from moriarty_dev import accounting, cli, loan_executor, records, runner, store  # noqa: E402
from moriarty_dev.loan_exit_retention.exit_retention import run_bounded  # noqa: E402
from moriarty_dev.store import make_history_reader  # noqa: E402

NS = 1_000_000_000
CLI = harness.SCRIPTS / "moriarty_dev" / "cli.py"


def _sha(data):
    return hashlib.sha256(data).hexdigest()


class ResultReaderG1(unittest.TestCase):
    """G1-01 / G1-02 / G1-09: parent result reader and disposition."""

    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)
        self.run = ExecutorRun(self.fx)
        self.run.run()
        self.assertEqual(self.run.result["status"], "PROCESS_SUCCESS", self.run.result.get("failureCode"))
        self.plan = self.fx.loaded_plan
        self.document = json.loads((self.fx.root / self.fx.plan["resultPath"]).read_text())

    def reread(self, document=None):
        payload = document if document is not None else json.loads((self.fx.root / self.fx.plan["resultPath"]).read_text())
        (self.fx.root / self.fx.plan["resultPath"]).write_text(json.dumps(payload, indent=2) + "\n")
        return runner._read_result_document(self.fx.root, self.plan, self.fx.runner_digest,
                                            self.fx.reservation_id, self.run.digest, self.fx.runner)

    def mutate_evidence(self, kind, mutate):
        document = copy.deepcopy(self.document)
        item = next(row for row in document["evidence"] if row["kind"] == kind)
        path = self.fx.root / item["path"]
        payload = json.loads(path.read_text())
        mutate(payload)
        raw = (json.dumps(payload, indent=2, sort_keys=True) + "\n").encode("utf-8")
        path.write_bytes(raw)
        item["sha256"] = _sha(raw)
        return document

    def test_g1_01_one_field_negatives_are_rejected(self):
        cases = [
            ("terminal-observation", lambda p: p.update({"observation": dict(p.get("observation") or {}, timedOut=True)})),
            ("containment", lambda p: p.update({"complete": True, "outstandingOwners": [],
                                                "unit": {"contained": False}, "prover": {"contained": False},
                                                "commands": []})),
            ("unit-startup", lambda p: (p.get("observed") or p.setdefault("observed", {})).update(
                {"LoadState": "not-found", "MainPID": "0", "Type": "forking"})),
            ("runtime-plan-comparison", lambda p: p.update({"changedFields": ["wallet.seedFile"],
                                                            "runtimeSha256": "0" * 64,
                                                            "originalSeedFile": "/tmp/other.seed"})),
            ("timer-armed", lambda p: p.update({"active": False, "problems": ["not-active"]})),
            ("explicit-stop", lambda p: p.update({"returnCode": False})),
        ]
        for kind, mutate in cases:
            with self.subTest(kind=kind):
                document = self.mutate_evidence(kind, mutate)
                loaded, problem = self.reread(document)
                self.assertIsNone(loaded, "contradictory %s must be rejected: %s" % (kind, problem))
                self.assertIsNotNone(problem)

    def test_g1_02_refused_without_contained_progress_stays_unresolved(self):
        loaded = {"document": {"status": "REFUSED", "containmentComplete": True, "outstandingOwners": []},
                  "unbound": False}
        handshake = True
        self.assertEqual(runner._disposition_for(2, loaded, handshake, False, False, True, [], True),
                         "unresolved")
        self.assertEqual(runner._disposition_for(2, loaded, handshake, True, False, True,
                                                 [{"phase": "contained"}], True),
                         "refused")

    def test_g1_09_known_main_with_stop_failure_is_a_validated_unknown(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"stopReturnCode": 1})
        run.run()
        self.assertEqual(run.result["status"], "PROCESS_UNKNOWN")
        self.assertEqual(run.result["failureCode"], "STOP_FAILED")
        self.assertEqual(run.result["rawMainExit"], {"kind": "exit", "code": 0})
        loaded, problem = runner._read_result_document(
            fx.root, fx.loaded_plan, fx.runner_digest, fx.reservation_id, run.digest, fx.runner)
        self.assertIsNotNone(loaded, problem)
        self.assertEqual(loaded["document"]["status"], "PROCESS_UNKNOWN")


class ProgressFraming(unittest.TestCase):
    """G1-10 / TD-04: bounded AF_UNIX progress framing."""

    def ingest(self, chunks):
        buf = bytearray()
        progress = []
        handshake = {}
        acknowledged = False
        for chunk in chunks:
            acknowledged = runner.ingest_progress_chunk(buf, chunk, progress, handshake)
        return acknowledged, progress, handshake, bytes(buf)

    def test_g1_10_split_coalesced_utf8_oversize_and_foreign_records(self):
        contained = (json.dumps({"schema": loan_executor.PROGRESS_SCHEMA, "phase": "contained",
                                 "detail": {"ok": True}}, separators=(",", ":")) + "\n").encode()
        validated = (json.dumps({"schema": loan_executor.PROGRESS_SCHEMA, "phase": "validated",
                                 "detail": {}}, separators=(",", ":")) + "\n").encode()
        ack, progress, handshake, leftover = self.ingest([validated[:8], validated[8:], contained[:10], contained[10:]])
        self.assertTrue(ack)
        self.assertEqual([p["phase"] for p in progress], ["validated", "contained"])
        self.assertFalse(handshake.get("denied"))
        self.assertEqual(leftover, b"")
        coalesced = validated + contained
        ack, progress, handshake, leftover = self.ingest([coalesced])
        self.assertTrue(ack)
        self.assertEqual([p["phase"] for p in progress], ["validated", "contained"])
        snowman = (json.dumps({"schema": loan_executor.PROGRESS_SCHEMA, "phase": "validated",
                               "detail": {"note": "雪"}}, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")
        mid = snowman.index("雪".encode("utf-8")) + 1
        ack, progress, handshake, leftover = self.ingest([snowman[:mid], snowman[mid:]])
        self.assertFalse(handshake.get("denied"), handshake)
        self.assertEqual([p["phase"] for p in progress], ["validated"])
        ack, progress, handshake, leftover = self.ingest([b"{" + b"x" * (runner.PROGRESS_MAX_BYTES)])
        self.assertEqual(handshake.get("denied"), "PROGRESS_OVERSIZE")
        ack, progress, handshake, leftover = self.ingest(
            [b'{"schema":"moriarty.loan-progress/1","phase":"validated"}\n',
             b'{"schema":"other","phase":"contained"}\n'])
        self.assertEqual(handshake.get("denied"), "PROGRESS_MALFORMED")
        ack, progress, handshake, leftover = self.ingest(
            [b'{"schema":"moriarty.loan-progress/1","phase":"result"}\n'])
        self.assertEqual(handshake.get("denied"), "PROGRESS_PHASE")


class AbsoluteSupervisor(unittest.TestCase):
    """G1-03 / TD-02: CLOCK_MONOTONIC supervisor is the absolute bound."""

    def identity(self):
        boot = Path("/proc/sys/kernel/random/boot_id").read_text().strip()
        inode = os.stat("/proc/self/ns/time").st_ino
        return boot, str(inode)

    def test_g1_03_formula_counterexample_is_not_a_bound_without_the_supervisor(self):
        now = 100.950 * NS
        cut = 1606 * NS
        activation = 102.050 * NS
        runtime = max(1, int((cut - now) / NS))
        self.assertEqual(runtime, 1505)
        self.assertGreater(activation + runtime * NS, cut)

    def test_g1_03_supervisor_kills_term_resistant_child_at_the_absolute_cutoff(self):
        boot, inode = self.identity()
        cutoff = int(time.clock_gettime(time.CLOCK_MONOTONIC) * NS) + int(0.4 * NS)
        child = [sys.executable, "-c",
                 "import signal, time, os\nsignal.signal(signal.SIGTERM, signal.SIG_IGN)\n"
                 "signal.signal(signal.SIGINT, signal.SIG_IGN)\ntime.sleep(8)\n"]
        started = time.monotonic()
        code = loan_executor.supervise_absolute(cutoff, boot, inode, child)
        elapsed = time.monotonic() - started
        self.assertEqual(code, 137)
        self.assertLess(elapsed, 2.0)

    def test_g1_03_late_entry_refuses_without_starting_the_payload(self):
        boot, inode = self.identity()
        marker = Path(tempfile.mkdtemp(prefix="moriarty-supervise-")) / "started"
        cutoff = int(time.clock_gettime(time.CLOCK_MONOTONIC) * NS) - NS
        child = [sys.executable, "-c", "open(%r,'w').write('started')" % str(marker)]
        code = loan_executor.supervise_absolute(cutoff, boot, inode, child)
        self.assertEqual(code, 64)
        self.assertFalse(marker.exists())

    def test_g1_03_timer_next_elapse_offset_of_four_seconds_is_rejected(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.services._save("script.json", {"timerNextElapseOffsetSeconds": 4})
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual(run.result["failureCode"], "CONTAINMENT_UNSUPPORTED")
        self.assertEqual(run.result["status"], "REFUSED")


class PersistenceBudget(unittest.TestCase):
    """G1-04: remaining time is computed immediately before each write."""

    def test_g1_04_past_deadline_grants_no_new_write_or_lock_budget(self):
        with self.assertRaises(accounting.MutationFailed):
            accounting.remaining_bound_seconds(time.monotonic_ns() - NS, 25)
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        from moriarty_dev.records import load_snapshot
        snap = load_snapshot(str(fx.root), fx.action["id"], history_reader=make_history_reader(fx.db_path))
        rid = store.reserve(fx.db_path, str(fx.root), fx.action, snap, charge_id=harness.RUNTIME_CHARGE)
        plan = loan_executor.load_plan(fx.root, fx.plan_rel, fx.plan_sha)
        admission = loan_executor.load_admission(fx.root, fx.admission_rel, plan)
        state = accounting.runtime_handover(fx.root, plan, admission, harness.RUNTIME_CHARGE, rid,
                                            fx.runner_digest, os.getpid(), 7, deadline_ns=time.monotonic_ns() - NS)
        self.assertIsInstance(state, accounting.Refusal)
        self.assertEqual(json.loads(fx.master_path.read_text())["externalPackageCharges"][0]["launchClaim"],
                         "unclaimed")


class ObserverAndMint(unittest.TestCase):
    """G1-05 / G1-06: observer evidence and native-aligned financial admission."""

    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)

    def test_g1_05_missing_sdk_closure_file_is_invalid(self):
        (self.fx.root / self.fx.sdk_closure_rel).unlink()
        plan = loan_executor.load_plan(self.fx.root, self.fx.plan_rel, self.fx.plan_sha)
        admission = loan_executor.load_admission(self.fx.root, self.fx.admission_rel, plan)
        chain = accounting.authenticate_authority(self.fx.root, plan, admission)
        history = accounting.verify_history(chain)
        wall = int(time.time() * 1000)
        observed = accounting.verify_observation(chain, history, wall, plan.financial)
        self.assertIsInstance(observed, accounting.Refusal)
        self.assertIn("sdkClosure", observed.detail)

    def test_g1_06_python_rejects_the_native_malformed_plans(self):
        financial = json.loads((self.fx.root / self.fx.financial_rel).read_text())
        expected = self.fx.expected
        wall = int(time.time() * 1000)
        wrong_role = copy.deepcopy(financial)
        wrong_role["roles"]["firstAddress"] = "0" * 64
        self.assertIn("firstAddress", accounting.financial_plan_shape(wrong_role))
        overlap = copy.deepcopy(financial)
        overlap["outputDirectory"] = overlap["wallet"]["stateDirectory"]
        self.assertIn("overlap", accounting.financial_plan_shape(overlap))
        overflow = copy.deepcopy(financial)
        overflow["limits"]["dustFee"] = str(1 << 128)
        self.assertIn("dustFee", accounting.financial_plan_shape(overflow))
        missing_receipt = copy.deepcopy(financial)
        missing_receipt["build"]["receiptPath"] = str(self.fx.root / "missing-receipt.json")
        problem = accounting.validate_financial_plan(missing_receipt, expected, wall)
        self.assertIsInstance(problem, str)
        self.assertIn("mint derivation unavailable", problem)


class ClaimAndHistory(unittest.TestCase):
    """G1-07 / G1-08 / G1-12 / G1-13."""

    def test_g1_07_missing_preflight_counterpart_or_marker_is_claim_invalid(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        master = json.loads(fx.master_path.read_text())
        master["externalPackageCharges"] = [row for row in master["externalPackageCharges"]
                                            if row["id"] != harness.PREFLIGHT_CHARGE]
        master["successorEnvelopes"][harness.ENVELOPE]["workerDispatches"] = 1
        master["successorEnvelopes"][harness.ENVELOPE]["remainingReservedSeconds"] = (
            harness.GRANT_SECONDS - harness.RUNTIME_SECONDS)
        master["externalWorkerDispatches"] = fx.predecessor["externalWorkerDispatches"] + 1
        fx.master_path.write_text(json.dumps(master, indent=2) + "\n")
        fx.write_correspondence()
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "CLAIM_INVALID"))

    def test_g1_08_live_cmd_run_rejects_caller_history_and_alternate_db(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        for idx in range(2):
            store.record_event(fx.db_path, str(fx.root), fx.action["requirement"], fx.action["capability"],
                               fx.action["candidate"], "new-failure-%d" % idx, "defect_failure",
                               {"defectId": "LIVE-FAIL-%d" % idx, "details": "unresolved live failure"})
        silent = lambda lineage: {"sameDefectFailures": 0, "adminCycles": 0, "adminSeconds": 0,
                                  "primaryActive": False, "reproducerVerified": False, "approachChanged": False}
        with self.assertRaises(SystemExit) as exited:
            cli.cmd_run(fx.root, fx.db_path, silent, fx.action["id"], True)
        self.assertEqual(exited.exception.code, 2)
        other = Path(tempfile.mkdtemp(prefix="moriarty-other-db-")) / "other.sqlite"
        store.init_db(other).close()
        with self.assertRaises(SystemExit) as exited:
            cli.cmd_run(fx.root, other, make_history_reader(fx.db_path), fx.action["id"], True)
        self.assertEqual(exited.exception.code, 2)

    def test_g1_12_live_acceptance_rejects_historical_design_status(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.live["acceptance"]["status"] = "complete-design-subset-only"
        fx.write_live_records()
        from moriarty_dev.records import load_snapshot
        snap = load_snapshot(str(fx.root), fx.action["id"], history_reader=make_history_reader(fx.db_path))
        self.assertTrue(any("acceptance-status-unsupported" in item for item in snap.get("missingEvidence") or []))

    def test_g1_13_source_review_does_not_clear_runtime_failures(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        store.record_event(fx.db_path, str(fx.root), fx.action["requirement"], fx.action["capability"],
                           fx.action["candidate"], "runtime-fail", "defect_failure",
                           {"defectId": "RUNTIME-1", "details": "operational"})
        before = store.get_history(fx.db_path, str(fx.root), fx.action["requirement"], fx.action["capability"])
        self.assertGreaterEqual(before["sameDefectFailures"], 1)
        store.record_review(fx.db_path, str(fx.root), {
            "reviewer": "gpt-6-astra independent", "candidateHash": fx.action["candidate"],
            "scope": records.LIVE_SCOPE, "verdict": "APPROVED", "findings": ["RUNTIME-1"],
            "resolvedFindings": ["RUNTIME-1"],
        }, fx.action)
        after = store.get_history(fx.db_path, str(fx.root), fx.action["requirement"], fx.action["capability"])
        self.assertGreaterEqual(after["sameDefectFailures"], 1)
        kinds = [row[0] for row in __import__("sqlite3").connect(fx.db_path).execute(
            "SELECT event_kind FROM events ORDER BY id").fetchall()]
        self.assertIn("source_review", kinds)
        self.assertNotEqual(kinds[-1], "result_review")


class StartupDiagnostics(unittest.TestCase):
    """G1-11 / TD-03: malformed startup and authentic unknown identity."""

    def test_g1_11_malformed_startup_show_reaches_durable_result(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, script={"startupMalformedShow": True})
        run.run()
        self.assertTrue(run.outcome["resultPersisted"])
        self.assertEqual(run.result["status"], "PROCESS_UNKNOWN")
        self.assertEqual(run.result["failureCode"], "STARTUP_INVALID")
        self.assertTrue((fx.root / fx.plan["resultPath"]).is_file())

    def test_td_03_unknown_startup_retains_observed_invocation_id(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        observed = "ab" * 16
        run = ExecutorRun(fx, script={"activationDelaySeconds": 200, "invocationId": observed})
        run.run()
        self.assertEqual(run.result["failureCode"], "STARTUP_INVALID")
        startup = json.loads((fx.evidence_dir / "unit-startup.json").read_text())
        self.assertEqual(startup["observed"]["InvocationID"], observed)
        self.assertEqual(run.result["invocationId"], observed)
        loaded, problem = runner._read_result_document(
            fx.root, fx.loaded_plan, fx.runner_digest, fx.reservation_id, run.digest, fx.runner)
        self.assertIsNotNone(loaded, problem)


class PipeAndParentLoss(unittest.TestCase):
    """TD-01 / TD-05 / TD-06."""

    def test_td_05_payload_beyond_pipe_capacity_drains_and_stalled_worker_is_killed(self):
        outcome = run_bounded(lambda: "x" * 200000, 5)
        self.assertEqual(outcome.status, "completed", outcome.detail)
        started = time.monotonic()
        stalled = run_bounded(lambda: time.sleep(30), 0.4)
        self.assertEqual(stalled.status, "timeout")
        self.assertLess(time.monotonic() - started, 3)

    def test_td_06_cli_json_retains_the_reader_problem(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.services._save("script.json", {"startupMalformedShow": True})
        result = subprocess.run([sys.executable, str(CLI), "--repo", str(fx.root), "run",
                                 "--action", fx.action["id"], "--json"],
                                capture_output=True, text=True, timeout=180)
        self.assertNotEqual(result.returncode, 0)
        out = json.loads(result.stdout)
        self.assertIn("problem", out)
        self.assertTrue(out.get("status") or out.get("disposition") or out.get("problem"))


if __name__ == "__main__":
    unittest.main()
