"""Regressions for the R2 independent audit findings (R2-01 .. R2-13).

Each test was observed failing against the frozen R2 source before repair
(see FOREMAN_REPORT.md evidence). They drive the real reader, runner, CLI,
executor, collector, cleanup, accounting and records validators on isolated
fixtures with fake service transports.
"""
import copy
import fcntl
import json
import os
import pathlib
import sqlite3
import subprocess
import sys
import time
import unittest
from contextlib import closing

HERE = pathlib.Path(__file__).resolve().parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import harness  # noqa: E402
import fake_services  # noqa: E402
from harness import ExecutorRun, FaultyWriter, LoanFixture  # noqa: E402
from moriarty_dev import accounting, cli, loan_cleanup, loan_executor, records, runner, store  # noqa: E402
from moriarty_dev.loan_exit_retention import exit_retention  # noqa: E402
from moriarty_dev.loan_exit_retention.terminal_predicate import classify_main_exit  # noqa: E402

NS = 1_000_000_000
CLI = harness.SCRIPTS / "moriarty_dev" / "cli.py"


def rows(db_path, query, *params):
    with closing(sqlite3.connect(db_path)) as conn:
        return conn.execute(query, params).fetchall()


class ResultReader(unittest.TestCase):
    """R2-01, R2-02, R2-09: the parent result reader and disposition."""

    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)
        self.plan = loan_executor.load_plan(self.fx.root, self.fx.plan_rel, self.fx.plan_sha)

    def success_document(self):
        document = {key: None for key in loan_executor.RESULT_KEYS}
        document.update(schema=loan_executor.RESULT_SCHEMA, allocationId=self.plan["allocationId"],
                        actionId=self.plan["actionId"], candidateHash=self.plan["candidateHash"],
                        runnerDigest=self.fx.runner_digest, chargeId=self.fx.runner["chargeId"],
                        reservationId="isolated-reservation", invocationSha256="1" * 64, unit=self.plan["unit"],
                        invocationId="2" * 32, status="PROCESS_SUCCESS", rawMainExit={"kind": "exit", "code": 0},
                        terminalEvidencePersisted=True, stopReturnCode=0, stopErrorClass=None,
                        stopReceiptPersisted=True, containmentComplete=True, timerCancelReturnCode=0,
                        timerCancelReceiptPersisted=True, failureCode=None, evidence=[], outstandingOwners=[],
                        retryAllowed=False, financialAcceptance="pending")
        return document

    def read(self, document):
        self.plan.result_path.write_text(json.dumps(document))
        loaded, problem = runner._read_result_document(self.fx.root, self.plan, self.fx.runner_digest,
                                                       "isolated-reservation", "1" * 64, self.fx.runner)
        self.plan.result_path.unlink()
        return loaded, problem

    def test_r2_01_success_requires_real_evidence_and_strict_scalars(self):
        loaded, problem = self.read(self.success_document())
        self.assertIsNone(loaded, "success with evidence=[] must not be accepted: %s" % problem)
        for patch in ({"terminalEvidencePersisted": "false"}, {"stopReturnCode": False},
                      {"timerCancelReturnCode": False}, {"stopErrorClass": "TimeoutError"},
                      {"invocationId": None}, {"rawMainExit": {"kind": "exit", "code": 0}, "failureCode": "X"},
                      {"stopReturnCode": 256}, {"rawMainExit": {"kind": "signal", "code": 0}},
                      {"status": "PROCESS_FAILED", "rawMainExit": {"kind": "exit", "code": 0}, "failureCode": "MAIN_EXIT_NONZERO"}):
            with self.subTest(patch=patch):
                document = self.success_document()
                document.update(patch)
                loaded, problem = self.read(document)
                self.assertIsNone(loaded, patch)

    def test_r2_02_null_identity_refusal_stays_unresolved(self):
        document = self.success_document()
        document.update(status="REFUSED", runnerDigest=None, chargeId=None, reservationId=None, invocationSha256=None,
                        rawMainExit={"kind": "unknown", "code": None}, containmentComplete=False,
                        outstandingOwners=[{"owner": "timer", "resource": "fixture", "reason": "unresolved"}],
                        failureCode="AUTHORITY_INVALID", invocationId=None)
        loaded, problem = self.read(document)
        disposition = runner._disposition_for(2, loaded, False, False, False, True, progress=[])
        self.assertEqual(disposition, "unresolved")
        document.update(containmentComplete=True, outstandingOwners=[])
        loaded, problem = self.read(document)
        self.assertEqual(runner._disposition_for(2, loaded, False, False, False, True, progress=[]), "unresolved",
                         "a report without a completed handshake is diagnostic, not a close")

    def test_r2_09_nonregular_result_path_is_bounded_unresolved(self):
        os.mkfifo(self.plan.result_path)
        started = time.monotonic()
        loaded, problem = runner._read_result_document(self.fx.root, self.plan, self.fx.runner_digest,
                                                       "isolated-reservation", "1" * 64, self.fx.runner)
        self.assertLess(time.monotonic() - started, 5)
        self.assertIsNone(loaded)
        self.assertIn("regular", problem)
        self.plan.result_path.unlink()
        big = self.plan.result_path
        big.write_bytes(b"{" + b" " * (8 * 1024 * 1024 + 2))
        loaded, problem = runner._read_result_document(self.fx.root, self.plan, self.fx.runner_digest,
                                                       "isolated-reservation", "1" * 64, self.fx.runner)
        self.assertIsNone(loaded)


class OwnershipRelease(unittest.TestCase):
    """R2-03: a refused or failed durable release prevents operational completion."""

    def test_release_refusal_keeps_reservation_active_and_cli_four(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        original = accounting.release_ownership
        # Make the release fail after the executor completes: the ownership
        # record is changed underneath the parent before release.
        driver = fx.root / "release-driver.py"
        driver.write_text("""import sys, json
sys.path.insert(0, %r)
from pathlib import Path
from moriarty_dev import loan_executor, accounting
deps = loan_executor.production_dependencies(Path.cwd(), service_table=%r)
code = loan_executor.main(sys.argv[1:], dependencies=deps)
record = json.loads(Path(%r).read_text())
record["runtimeReservationId"] = "someone-else"
Path(%r).write_text(json.dumps(record))
raise SystemExit(code)
""" % (str(harness.SCRIPTS), fx.service_table, str(fx.ownership_path), str(fx.ownership_path)))
        fx.write_runner(argv=[fx.python, "release-driver.py", "--plan", fx.plan_rel, "--sha256", fx.plan_sha,
                              "--admission", fx.admission_rel])
        result = subprocess.run([sys.executable, str(CLI), "--repo", str(fx.root), "run", "--action",
                                 fx.action["id"], "--json"], capture_output=True, text=True, timeout=180)
        self.assertEqual(result.returncode, 4, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["status"], out["disposition"], out["reservationStatus"]),
                         ("PROCESS_SUCCESS", "unresolved", "active"))
        self.assertEqual(rows(fx.db_path, "SELECT status FROM reservations"), [("active",)])
        self.assertEqual(json.loads(fx.ownership_path.read_text())["state"], "runtime-held")
        del original


class RequiredWrites(unittest.TestCase):
    """R2-04: runtime-plan comparison and timer-armed evidence are required."""

    def test_missing_required_setup_evidence_refuses_before_start(self):
        for name in ("runtime-plan-comparison.json", "timer-armed.json"):
            with self.subTest(artifact=name):
                fx = LoanFixture().build()
                self.addCleanup(fx.cleanup)
                run = ExecutorRun(fx, writer=FaultyWriter(fail=[name]))
                run.run()
                self.assertNotEqual(run.result["status"], "PROCESS_SUCCESS")
                self.assertEqual([r for r in run.calls() if "start" in r], [], "no start after a failed required write")
                self.assertEqual(run.result["failureCode"], "EVIDENCE_WRITE_FAILED")
                self.assertEqual(run.outcome["exitCode"], 2)
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        run = ExecutorRun(fx, writer=FaultyWriter(timeout=["timer-armed.json"]))
        run.run()
        self.assertNotEqual(run.result["status"], "PROCESS_SUCCESS")
        self.assertEqual([r for r in run.calls() if "start" in r], [])


class AccountingEvidence(unittest.TestCase):
    """R2-05, R2-06, R2-12: verifier semantics, freshness at startup, master serialization."""

    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)

    def verify(self):
        run = ExecutorRun(self.fx) if not hasattr(self, "_run") else self._run
        self._run = run
        return accounting.validate_executor_context(self.fx.loaded_plan, self.fx.loaded_admission, run.payload,
                                                    root=self.fx.root, financial_plan=self.fx.loaded_plan.financial)

    def test_r2_05_observation_semantics(self):
        self.assertIsInstance(self.verify(), accounting.CurrentExecution)
        cases = {
            "wrong token and units": dict(amounts={"OTHER": "0"}, unitNames={"OTHER": "wrong"}),
            "insufficient selected asset": dict(amounts={"USD_TEST_ASSET": "1", "DUST": "5000000000000000"},
                                                unitNames={"USD_TEST_ASSET": "units", "DUST": "speck"}),
            "unrelated observer source": dict(sourceSha256="9" * 64),
            "substituted pending-use receipt": dict(pendingUseDispositionSha256=harness.sha256_file(
                self.fx.root / "deliverables/loan-fixture/immutable/loan-attempt-1.json")),
            "child boolean without parent evidence": dict(canonicalInputsUnchanged=True, childrenStopped=True,
                                                          preflightChargeId="other-preflight"),
        }
        for label, patch in cases.items():
            with self.subTest(case=label):
                self.fx.write_observation(**patch)
                refusal = self.verify()
                self.assertIsInstance(refusal, accounting.Refusal, label)
                self.assertEqual(refusal.code, "OBSERVATION_INVALID")
        self.fx.write_observation()
        self.fx.parent_evidence_path.unlink()
        refusal = self.verify()
        self.assertEqual(refusal.code, "OBSERVATION_INVALID", "missing parent observer evidence must be unavailable")

    def test_r2_05_history_contents_and_grant_invariants(self):
        self.verify()
        fx = self.fx
        chain = accounting.authenticate_authority(fx.root, fx.loaded_plan, fx.loaded_admission)
        # Receipt content disagrees with the outcome entry.
        chain2 = copy.deepcopy(chain)
        chain2["history"]["outcomes"][0]["outcome"] = "verified-success"
        self.assertEqual(accounting.verify_history(chain2).code, "HISTORY_UNRESOLVED")
        # Event payload not bound to a listed receipt.
        store.record_event(fx.db_path, str(fx.root), fx.action["requirement"], fx.action["capability"],
                           fx.action["candidate"], "x", "defect_failure", {"defectId": "UNBOUND"})
        unbound = rows(fx.db_path, "SELECT id FROM events WHERE action_id = 'x'")[0][0]
        chain3 = copy.deepcopy(chain)
        chain3["history"]["eventBindings"][0]["eventIds"].append(unbound)
        self.assertEqual(accounting.verify_history(chain3).code, "HISTORY_UNRESOLVED")
        # Master reserve transfer and grant arithmetic.
        run = self._run
        master = json.loads(fx.master_path.read_text())
        good = json.dumps(master)

        def apply(mutation, payload_patch=None):
            current = json.loads(good)
            mutation(current)
            fx.master_path.write_text(json.dumps(current, indent=2, sort_keys=True) + "\n")
            corr = json.loads((fx.root / fx.correspondence_path).read_text())
            corr["masterSha256"] = harness.sha256_file(fx.master_path)
            (fx.root / fx.correspondence_path).write_text(json.dumps(corr, indent=2, sort_keys=True) + "\n")
            payload = dict(run.payload, correspondenceSha256=harness.sha256_file(fx.root / fx.correspondence_path))
            store.record_invocation(fx.db_path, str(fx.root), fx.action, payload)
            return accounting.validate_executor_context(fx.loaded_plan, fx.loaded_admission, payload, root=fx.root,
                                                        financial_plan=fx.loaded_plan.financial)

        def reserve_lies(m):
            m["reserved"][0]["seconds"] = 1
        self.assertEqual(apply(reserve_lies).code, "CLAIM_INVALID")

        def limit_lies(m):
            m["master_limit_seconds"] += 5
        self.assertEqual(apply(limit_lies).code, "CLAIM_INVALID")

        def predecessor_row_changed(m):
            m["charges"].append({"id": "new", "seconds": 1})
        self.assertEqual(apply(predecessor_row_changed).code, "CLAIM_INVALID")
        # Amendment grant must equal G exactly.
        admission = json.loads((fx.root / fx.admission_rel).read_text())
        admission["additionalSeconds"] = harness.GRANT_SECONDS + 1
        other = accounting.Authenticated(admission, fx.admission_sha, fx.admission_rel)
        self.assertEqual(accounting.authenticate_authority(fx.root, fx.loaded_plan, other).code, "AUTHORITY_INVALID")

    def test_r2_06_freshness_rechecked_at_financial_startup(self):
        fx = self.fx
        fx.write_observation(age_seconds=20)
        fx.write_correspondence()
        run = ExecutorRun(fx, setup_delay_seconds=110)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "OBSERVATION_INVALID"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])
        self.assertEqual(harness.sha256_file(fx.root / fx.receipt_path), fx.observation_sha)

    def test_r2_12_budget_lock_and_supervised_master_writes(self):
        fx = self.fx
        lock_path = fx.master_path.with_suffix(".lock")
        holder = os.open(str(lock_path), os.O_RDWR | os.O_CREAT, 0o600)
        fcntl.flock(holder, fcntl.LOCK_EX)
        self.addCleanup(os.close, holder)
        before = fx.master_path.read_bytes()
        bound = accounting.LOCK_WAIT_SECONDS
        accounting.LOCK_WAIT_SECONDS = 0.5
        self.addCleanup(setattr, accounting, "LOCK_WAIT_SECONDS", bound)
        with self.assertRaisesRegex(AssertionError, "CLAIM_INVALID"):
            fx.reserve_and_handover()
        self.assertEqual(fx.master_path.read_bytes(), before, "held budget lock: master untouched")
        fcntl.flock(holder, fcntl.LOCK_UN)
        # Stalled replacement write is bounded and leaves the master unchanged.
        real = accounting._write_replacement
        accounting._write_replacement = lambda temp, data: time.sleep(30)
        self.addCleanup(setattr, accounting, "_write_replacement", real)
        fx2 = LoanFixture().build()
        self.addCleanup(fx2.cleanup)
        write_bound = accounting.WRITE_BOUND_SECONDS
        accounting.WRITE_BOUND_SECONDS = 0.5
        self.addCleanup(setattr, accounting, "WRITE_BOUND_SECONDS", write_bound)
        started = time.monotonic()
        with self.assertRaisesRegex(AssertionError, "CLAIM_INVALID"):
            fx2.reserve_and_handover()
        self.assertLess(time.monotonic() - started, 10)
        self.assertEqual(json.loads(fx2.master_path.read_text())["externalPackageCharges"][0]["launchClaim"], "unclaimed")


class CleanupIdentity(unittest.TestCase):
    """R2-07, R2-08: exact identities per destructive command and one absolute budget."""

    def setUp(self):
        self.tmp = pathlib.Path(__import__("tempfile").mkdtemp(prefix="moriarty-r2-cleanup-"))
        self.services = fake_services.FakeServices(self.tmp / "state")
        self.run = fake_services.run_command_for(self.services)
        self.container = "d" * 64
        self.intent = {
            "schema": loan_cleanup.INTENT_SCHEMA, "allocationId": "alloc-01", "unit": "fixture.service",
            "timerUnit": "fixture-cleanup", "containerId": self.container, "invocationSha256": "e" * 64,
            "chargeId": "c", "reservationId": "r",
            "evidenceDirectory": str(self.tmp), "proverReceiptPath": str(self.tmp / "prover-ownership.json"),
            "unitReceiptPath": str(self.tmp / "unit-ownership.json"),
        }
        self.services.seed_container(fake_services.container_config(self.container, "sha256:" + "0" * 64, 1, "/w",
                                                                     "/c", "/w", "/c", "/c/x"))
        self.services._save("script.json", {"dockerStartedAt": "2026-09-13T20:00:00Z", "invocationId": "f" * 32})
        self.run(["/usr/bin/docker", "start", self.container])
        self.run(["/usr/bin/systemd-run", "--user", "--unit=fixture.service", "--", "/bin/true"])
        self.unit_fields = self.services.unit("fixture.service")

    def receipts(self, container_id=None, unit=None, cgroup=None, charge="c", reservation="r"):
        prover = {"schema": loan_cleanup.PROVER_RECEIPT_SCHEMA, "allocationId": "alloc-01", "invocationSha256": "e" * 64,
                  "chargeId": charge, "reservationId": reservation, "containerId": container_id or self.container,
                  "containerStartedAt": "2026-09-13T20:00:00Z", "startArgv": [], "startReturnCode": 0,
                  "observedAtUtc": "2026-09-13T00:00:00+00:00"}
        unit_receipt = {"schema": loan_cleanup.UNIT_RECEIPT_SCHEMA, "allocationId": "alloc-01", "invocationSha256": "e" * 64,
                        "chargeId": charge, "reservationId": reservation, "unit": unit or "fixture.service",
                        "unitInvocationId": "f" * 32, "unitControlGroup": cgroup or self.unit_fields["ControlGroup"],
                        "unitMainPid": "4242", "observedAtUtc": "2026-09-13T00:00:00+00:00"}
        return prover, unit_receipt

    def contain(self, prover, unit, deadline_ns=None, clock=None):
        clock = clock or time.monotonic_ns
        deadline_ns = deadline_ns if deadline_ns is not None else clock() + 25 * NS
        return loan_cleanup.contain_owned(self.intent, prover, unit, self.run, deadline_ns, "safety-cleanup",
                                          monotonic_ns=clock)

    def kills(self):
        return [row for row in self.services.calls() if "kill" in row]

    def test_r2_07_receipt_identities_must_match_intent(self):
        cases = (("container", dict(container_id="a" * 64), "docker"),
                 ("unit", dict(unit="other.service"), "systemctl"),
                 ("cgroup", dict(cgroup="/user.slice/other.service"), "systemctl"),
                 ("claim", dict(charge="other"), None))
        for label, patch, untouched in cases:
            with self.subTest(defect=label):
                self.setUp()  # fresh fake state per case
                prover, unit = self.receipts(**patch)
                facts = self.contain(prover, unit)
                self.assertFalse(facts["complete"], label)
                kills = self.kills()
                if untouched is None:
                    self.assertEqual(kills, [], label)
                else:
                    self.assertFalse(any(untouched in row[3] for row in kills), "%s: mismatched resource was killed" % label)
                    self.assertTrue(any(o["reason"] in ("receipt-identity-mismatch", "invocation-mismatch-not-killed")
                                        for o in facts["outstandingOwners"]))

    def test_r2_07_replacement_between_kill_and_stop_is_not_stopped(self):
        self.services._save("script.json", {"replaceUnitAfterKill": True, "dockerStartedAt": "2026-09-13T20:00:00Z"})
        facts = self.contain(*self.receipts())
        self.assertFalse(facts["complete"])
        self.assertEqual(facts["unit"]["disposition"], "invocation-changed-after-kill")
        self.assertEqual([r for r in self.services.calls() if "stop" in r and "fixture.service" in r], [])

    def test_r2_08_single_absolute_budget_across_commands(self):
        virtual = {"now": time.monotonic_ns()}

        def clock():
            return virtual["now"]
        deadline = clock() + 1 * NS
        original = self.run

        def slow(argv, timeout_seconds=30):
            virtual["now"] += int(0.9 * NS)
            return original(argv, timeout_seconds)
        self.run = slow
        facts = self.contain(*self.receipts(), deadline_ns=deadline, clock=clock)
        self.assertFalse(facts["complete"])
        self.assertLessEqual(len(facts["commands"]), 2, "no command after the shared cutoff")
        self.assertTrue(any(o["reason"].startswith("cleanup-budget") for o in facts["outstandingOwners"]))


class ExecutorSchedule(unittest.TestCase):
    """R2-08, R2-10, R2-11: timer target, prover argv, startup timestamp, running predicate."""

    def test_r2_08_timer_next_elapse_must_match_the_absolute_cutoff(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.services._save("script.json", {"timerNextElapseOffsetSeconds": 300})
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "CONTAINMENT_UNSUPPORTED"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])
        fx2 = LoanFixture().build()
        self.addCleanup(fx2.cleanup)
        fx2.services._save("script.json", {"timerUnitName": "other.service"})
        run2 = ExecutorRun(fx2)
        run2.run()
        self.assertEqual(run2.result["failureCode"], "CONTAINMENT_UNSUPPORTED")

    def test_r2_08_delayed_activation_cannot_extend_the_bootstrap_cutoff(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.services._save("script.json", {"activationDelaySeconds": 200})
        run = ExecutorRun(fx)
        run.run()
        self.assertNotEqual(run.result["status"], "PROCESS_SUCCESS")
        self.assertEqual(run.result["failureCode"], "STARTUP_INVALID")

    def test_r2_10_prover_command_is_pinned_by_reviewed_authority(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        config = fx.services._load("container.json", None)
        config["Args"] = [config["Args"][0], "--", "/bin/sh", "-c", "anything"]
        fx.services.seed_container(config)
        run = ExecutorRun(fx)
        run.run()
        self.assertEqual((run.result["status"], run.result["failureCode"]), ("REFUSED", "CONTAINMENT_UNSUPPORTED"))
        self.assertEqual([r for r in run.calls() if "start" in r], [])
        fx2 = LoanFixture().build()
        self.addCleanup(fx2.cleanup)
        cfg = fx2.services._load("container.json", None)
        cfg["Mounts"][1]["Destination"] = "/elsewhere"
        cfg["Args"][0] = "/elsewhere/prover-control"
        fx2.services.seed_container(cfg)
        run2 = ExecutorRun(fx2)
        run2.run()
        self.assertEqual(run2.result["failureCode"], "CONTAINMENT_UNSUPPORTED")

    def test_r2_11_startup_requires_actual_start_timestamp(self):
        fx = LoanFixture().build()
        self.addCleanup(fx.cleanup)
        fx.services._save("script.json", {"startupMissingTimestamp": True})
        run = ExecutorRun(fx)
        run.run()
        self.assertNotEqual(run.result["status"], "PROCESS_SUCCESS")
        self.assertEqual(run.result["failureCode"], "STARTUP_INVALID")

    def test_r2_11_contradictory_running_rows_are_terminal_unknown(self):
        inv = "a" * 32
        base = {"LoadState": "loaded", "ActiveState": "active", "SubState": "running", "Type": "exec",
                "RemainAfterExit": "yes", "Transient": "yes", "MainPID": "77", "Result": "success",
                "ExecMainCode": "0", "ExecMainStatus": "0", "InvocationID": inv, "ControlGroup": "/u"}
        self.assertEqual(classify_main_exit(base, inv)["status"], "pending")
        for patch in ({"Type": "oneshot"}, {"RemainAfterExit": "no"}, {"Transient": "no"}, {"ExecMainCode": "1"},
                      {"ExecMainStatus": "3"}, {"Result": "exit-code"}):
            with self.subTest(patch=patch):
                row = classify_main_exit(dict(base, **patch), inv)
                self.assertEqual((row["status"], row["failureCode"]), ("PROCESS_UNKNOWN", "MAIN_OBSERVATION_INVALID"))
        # The installed collector must not turn such rows into NOT_TERMINAL.
        import tempfile
        from moriarty_dev.loan_exit_retention.exit_retention import CommandResult, retain_exit_evidence

        class Transport(object):
            def show(self, unit, properties, show_seconds=None):
                fields = dict(base, ExecMainCode="1")
                return CommandResult(["show"], 0, "".join("%s=%s\n" % kv for kv in fields.items()), "")

            def stop(self, unit, stop_seconds=25):
                raise AssertionError("no stop for an invalid observation")
        startup = {"LoadState": "loaded", "ActiveState": "active", "MainPID": "77", "InvocationID": inv,
                   "ControlGroup": "/u", "ActiveEnterTimestampMonotonic": "5"}
        with tempfile.TemporaryDirectory() as tmp:
            result = retain_exit_evidence("fixture.service", startup, Transport(), tmp)
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertFalse(result.get("pending"))
            self.assertEqual(result["classification"]["failureCode"], "MAIN_OBSERVATION_INVALID")
            self.assertTrue((pathlib.Path(tmp) / "terminal-observation.json").is_file())


class LiveAdmission(unittest.TestCase):
    """R2-13: the exact item 1.2 live labels reach the shared validators."""

    def test_live_profile_fixture_is_admitted_and_near_misses_are_rejected(self):
        fx = LoanFixture().build(profile="live")
        self.addCleanup(fx.cleanup)
        from moriarty_dev.records import load_snapshot
        from moriarty_dev.policy import assess
        from moriarty_dev.store import make_history_reader
        snap = load_snapshot(str(fx.root), fx.action["id"], history_reader=make_history_reader(fx.db_path))
        self.assertTrue(snap["authorityCurrent"] and snap["candidateCurrent"] and snap["resourceAdmitted"]
                        and snap["entryEligible"], snap["missingEvidence"])
        self.assertTrue(assess(snap, fx.action)["allow"], snap["missingEvidence"])
        self.assertEqual((fx.action["requirement"], fx.action["evidenceProfile"]), ("SP05.3", "finalized-financial-settlement"))
        # Near misses individually reject.
        for label, mutate in (
            ("binding status suffix", lambda b, c: b.update(status="admitted-i2-source-only-x")),
            ("digest algorithm substring", lambda b, c: c.update(digestAlgorithm="my SHA256 thing")),
            ("review scope case", lambda b, c: fx.live_review.update(scope=records.LIVE_SCOPE.upper())),
            ("acceptance status", lambda b, c: fx.live_acceptance.update(status="complete-i2-source-only-x")),
            ("same-vendor reviews", lambda b, c: fx.live_source_review.update(agent="claude-opus-5")),
            ("bridge diff", lambda b, c: fx.live_bridge.update(diffSha256="0" * 64)),
        ):
            with self.subTest(defect=label):
                fx.mutate_live(mutate)
                snap = load_snapshot(str(fx.root), fx.action["id"], history_reader=make_history_reader(fx.db_path))
                self.assertFalse(snap["authorityCurrent"] and snap["candidateCurrent"] and snap["entryEligible"], label)
                fx.restore_live()

    def test_live_profile_cmd_run_completes_through_the_real_path(self):
        fx = LoanFixture().build(profile="live")
        self.addCleanup(fx.cleanup)
        result = subprocess.run([sys.executable, str(CLI), "--repo", str(fx.root), "run", "--action",
                                 fx.action["id"], "--json"], capture_output=True, text=True, timeout=180)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        out = json.loads(result.stdout)
        self.assertEqual((out["action"], out["disposition"]), ("sp05-loan-execute-once", "success"))
        self.assertEqual(rows(fx.db_path, "SELECT status, action_kind FROM reservations"), [("finished", "implement")])

    def test_historical_design_verification_is_unchanged(self):
        fx = LoanFixture().build(profile="sp01-verify")
        self.addCleanup(fx.cleanup)
        from moriarty_dev.records import load_snapshot
        from moriarty_dev.store import make_history_reader
        snap = load_snapshot(str(fx.root), fx.action["id"], history_reader=make_history_reader(fx.db_path))
        self.assertTrue(snap["authorityCurrent"] and snap["candidateCurrent"], snap["missingEvidence"])
        stale = load_snapshot(str(harness.REPO_ROOT), "sp01-loan-verify-current", history_reader=lambda l: None)
        self.assertTrue(any("stale" in item for item in stale["missingEvidence"]))


if __name__ == "__main__":
    unittest.main()
