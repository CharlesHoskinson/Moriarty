"""Retained collector tests against the installed loan_exit_retention package.

Adapted from deliverables/sp05-loan-exit-retention-grok-2026-09-10/candidate.
Fixtures resolve from an explicitly established repository root and the
original deliverables/sp05-financial-integration-2026-09-09 paths. New tests
cover the closed nonzero/signal/unknown classification, the explicit
remaining-time argv bounds and the bounded persistence worker.
"""
import importlib
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import time
import unittest
from contextlib import redirect_stderr, redirect_stdout
from io import StringIO

SCRIPTS = pathlib.Path(__file__).resolve().parents[2] / "scripts"
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))

from moriarty_dev.loan_exit_retention import cli, exit_retention  # noqa: E402
from moriarty_dev.loan_exit_retention.exit_retention import (  # noqa: E402
    CommandResult,
    PersistenceTimeout,
    UserSystemctlTransport,
    durable_exclusive_save,
    parse_systemctl_show,
    retain_exit_evidence,
    run_bounded,
)
from moriarty_dev.loan_exit_retention.launch_contract import (  # noqa: E402
    describe_contract,
    explicit_stop_argv,
    require_startup_identity,
    terminal_show_argv,
)
from moriarty_dev.loan_exit_retention.loan_exit_operator import retain_loan_main_exit  # noqa: E402
from moriarty_dev.loan_exit_retention.terminal_predicate import classify_main_exit, require_exit_zero  # noqa: E402

REPO_ROOT = pathlib.Path(__file__).resolve().parents[4]
_FIN = REPO_ROOT / "deliverables" / "sp05-financial-integration-2026-09-09"
LOAN_TERMINAL = _FIN / "preview-loan-exit-01" / "terminal-observation-01.json"
LOAN_STARTUP = _FIN / "preview-loan-exit-01" / "launcher-active.json"
SWAP_TERMINAL = _FIN / "preview-swap-exit-01" / "terminal-observation-01.json"
SWAP_STARTUP = _FIN / "preview-swap-exit-01" / "launcher-active.json"
SWAP_UNIT = "moriarty-sp05-preview-swap-exit-01.service"
LOAN_UNIT = "moriarty-sp05-preview-loan-exit-01.service"
TEST_UNIT = "moriarty-sp05-loan-exit-next.service"
FIXED_CLOCK = lambda: "2026-09-10T00:00:00+00:00"  # noqa: E731


def _swap_stdout():
    return json.loads(SWAP_TERMINAL.read_text())["stdout"]


def _loan_stdout():
    return json.loads(LOAN_TERMINAL.read_text())["systemdShow"]


def _swap_startup():
    return json.loads(SWAP_STARTUP.read_text())["observed"]


def _loan_startup():
    return json.loads(LOAN_STARTUP.read_text())["observed"]


class ScriptedTransport(object):
    def __init__(self, show_stdout, show_code=0, stop_code=0, evidence_dir=None, stop_raises=None):
        self.show_stdout = show_stdout
        self.show_code = show_code
        self.stop_code = stop_code
        self.stop_raises = stop_raises
        self.evidence_dir = pathlib.Path(evidence_dir) if evidence_dir is not None else None
        self.calls = []
        self.observation_present_at_stop = None
        self.bounds = []

    def show(self, unit, properties, show_seconds=None):
        present = False
        if self.evidence_dir is not None:
            present = (self.evidence_dir / "terminal-observation.json").is_file()
        self.calls.append(("show", unit, properties, present))
        self.bounds.append(("show", show_seconds))
        argv = terminal_show_argv(unit, show_seconds) if show_seconds else \
            ["/usr/bin/systemctl", "--user", "show", unit, "--property=" + properties]
        return CommandResult(argv, self.show_code, self.show_stdout, "")

    def stop(self, unit, stop_seconds=25):
        present = False
        if self.evidence_dir is not None:
            present = (self.evidence_dir / "terminal-observation.json").is_file()
        self.observation_present_at_stop = present
        self.calls.append(("stop", unit, present))
        self.bounds.append(("stop", stop_seconds))
        if self.stop_raises is not None:
            raise self.stop_raises
        return CommandResult(explicit_stop_argv(unit, stop_seconds), self.stop_code, "", "")


class StatefulTransport(object):
    def __init__(self, loaded_stdout, unloaded_stdout):
        self.loaded_stdout = loaded_stdout
        self.unloaded_stdout = unloaded_stdout
        self.stopped = False

    def show(self, unit, properties, show_seconds=None):
        stdout = self.unloaded_stdout if self.stopped else self.loaded_stdout
        return CommandResult(["show", unit], 0, stdout, "")

    def stop(self, unit, stop_seconds=25):
        self.stopped = True
        return CommandResult(["stop", unit], 0, "", "")


class ExitRetention(unittest.TestCase):
    def _retain(self, transport, startup, unit, evidence_dir, **extra):
        return retain_loan_main_exit(unit=unit, startup_observed=startup, transport=transport,
                                     evidence_dir=evidence_dir, now_utc=FIXED_CLOCK, **extra)

    def test_fixtures_resolve_from_repository_root(self):
        for path in (LOAN_TERMINAL, LOAN_STARTUP, SWAP_TERMINAL, SWAP_STARTUP):
            self.assertTrue(path.is_file(), str(path))

    def test_happy_path_save_before_stop(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertEqual(result["sequence"], ["show", "persist", "stop"])
            self.assertEqual(transport.calls[0][0], "show")
            self.assertFalse(transport.calls[0][3])
            self.assertEqual(transport.calls[1][0], "stop")
            self.assertTrue(transport.calls[1][2])
            self.assertTrue(transport.observation_present_at_stop)
            self.assertTrue(result["exitRetained"])
            self.assertTrue(result["explicitStopCompleted"])
            self.assertEqual(result["status"], "EXIT_RETAINED_THEN_STOP_COMPLETED")
            self.assertEqual(result["classification"]["status"], "PROCESS_SUCCESS")
            self.assertEqual(result["classification"]["rawMainExit"], {"kind": "exit", "code": 0})
            self.assertFalse(result["financialAcceptance"])
            self.assertFalse(result["containmentAcceptance"])
            self.assertFalse(result["sp05Complete"])
            self.assertIsNone(result["admission"])
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertEqual(saved["predicate"]["status"], "EXIT_ZERO_OBSERVED")
            self.assertEqual(saved["startupInvocationId"], "b58efe5700db4dd3a63fd097fc5a1652")
            stop = json.loads((evidence / "explicit-stop.json").read_text())
            self.assertEqual(stop["terminalBeforeStopSha256"], result["terminalObservationSha256"])
            self.assertIn("InvocationID", transport.calls[0][2])
            self.assertIn("RemainAfterExit", transport.calls[0][2])

    def test_authentic_swap_sequence_through_operator(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = retain_exit_evidence(SWAP_UNIT, _swap_startup(), transport, evidence, now_utc=FIXED_CLOCK)
            self.assertTrue(result["exitRetained"])
            self.assertEqual(result["predicate"]["invocationId"], _swap_startup()["InvocationID"])
            self.assertEqual(result["startupInvocationId"], require_startup_identity(_swap_startup()))

    def test_authentic_loan_terminal_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_loan_stdout(), evidence_dir=evidence)
            result = self._retain(transport, dict(_swap_startup()), LOAN_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertEqual(result["rejected"], "EXIT_ZERO_NOT_ESTABLISHED")
            self.assertEqual(result["classification"]["status"], "PROCESS_UNKNOWN")
            self.assertEqual(result["classification"]["failureCode"], "MAIN_OBSERVATION_INVALID")
            self.assertIsNone(result["predicate"])
            self.assertFalse(result["stopAttempted"])
            self.assertFalse((evidence / "explicit-stop.json").exists())
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertEqual(saved["stdout"], _loan_stdout())
            self.assertFalse(saved["financialAcceptance"])

    def test_loan_startup_cannot_bind(self):
        with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
            require_startup_identity(_loan_startup())
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _loan_startup(), LOAN_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "STARTUP_IDENTITY_REQUIRED")
            self.assertEqual(transport.calls, [])
            self.assertFalse((evidence / "terminal-observation.json").exists())

    def test_invalid_startup_ids(self):
        base = dict(_swap_startup())
        for value in [None, "", "0" * 32, "ABC" * 10 + "abcd", "1" * 31, "g" * 32]:
            observed = dict(base)
            observed["InvocationID"] = value
            with self.subTest(value=value):
                with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
                    require_startup_identity(observed)

    def test_identity_mismatch(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            startup = dict(_swap_startup())
            startup["InvocationID"] = "c" * 32
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, startup, SWAP_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "EXIT_ZERO_NOT_ESTABLISHED")
            self.assertEqual(result["classification"]["failureCode"], "MAIN_OBSERVATION_INVALID")
            self.assertFalse(result["stopAttempted"])

    def test_duplicate_and_malformed_show(self):
        with self.assertRaisesRegex(ValueError, "DUPLICATE_FIELD"):
            parse_systemctl_show("LoadState=loaded\nLoadState=loaded\n")
        with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
            parse_systemctl_show("FINANCIAL_COMPLETE\nLoadState=loaded\n")
        with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
            parse_systemctl_show("")
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport("LoadState=loaded\nLoadState=not-found\n", evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), TEST_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "DUPLICATE_FIELD")

    def test_failed_persistence_does_not_report_success(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            (evidence / "terminal-observation.json").write_text("preexisting\n")
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "PERSISTENCE_FAILED")
            self.assertTrue(result["persistFailed"])
            self.assertEqual((evidence / "terminal-observation.json").read_text(), "preexisting\n")
            self.assertFalse(result["stopAttempted"])
            self.assertEqual([c[0] for c in transport.calls], ["show"])

    def test_failed_stop_after_retention(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), stop_code=5, evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(result["exitRetained"])
            self.assertFalse(result["explicitStopCompleted"])
            self.assertEqual(result["status"], "STOP_FAILED_AFTER_RETENTION")
            self.assertEqual(result["stopReturnCode"], 5)
            self.assertTrue(result["stopReceiptPersisted"])

    def test_stop_exception_after_retention_is_not_command_success(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence, stop_raises=OSError("stop"))
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(result["exitRetained"])
            self.assertEqual(result["status"], "STOP_FAILED_AFTER_RETENTION")
            self.assertIsNone(result["stopReturnCode"])
            self.assertEqual(result["stopErrorClass"], "OSError")
            recorded = json.loads((evidence / "explicit-stop.json").read_text())
            self.assertEqual(recorded["stopErrorClass"], "OSError")

    def test_post_stop_observation_rejected(self):
        transport = StatefulTransport(_swap_stdout(), _loan_stdout())
        with tempfile.TemporaryDirectory() as first:
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, pathlib.Path(first))
            self.assertTrue(result["exitRetained"])
        with tempfile.TemporaryDirectory() as second:
            later = self._retain(transport, _swap_startup(), SWAP_UNIT, pathlib.Path(second))
            self.assertFalse(later["exitRetained"])
            self.assertEqual(later["rejected"], "EXIT_ZERO_NOT_ESTABLISHED")

    def test_financial_text_cannot_bypass_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            result = self._retain(ScriptedTransport("FINANCIAL_COMPLETE=yes\n", evidence_dir=evidence),
                                  _swap_startup(), TEST_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "MALFORMED_SHOW")
            self.assertEqual(result["classification"]["failureCode"], "MAIN_OBSERVATION_INVALID")

    def test_show_failed_and_bad_inputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            result = self._retain(ScriptedTransport("", show_code=1, evidence_dir=evidence), _swap_startup(),
                                  TEST_UNIT, evidence)
            self.assertEqual(result["rejected"], "SHOW_FAILED")
            self.assertEqual(result["classification"]["failureCode"], "MAIN_OBSERVATION_UNAVAILABLE")
        missing = retain_loan_main_exit(SWAP_UNIT, _swap_startup(), ScriptedTransport(_swap_stdout()),
                                        "/no/such/loan-exit-dir", now_utc=FIXED_CLOCK)
        self.assertEqual(missing["rejected"], "EVIDENCE_DIR_REQUIRED")
        with tempfile.TemporaryDirectory() as tmp:
            bad = retain_loan_main_exit("../evil.service", _swap_startup(), ScriptedTransport(_swap_stdout()),
                                        pathlib.Path(tmp), now_utc=FIXED_CLOCK)
            self.assertEqual(bad["rejected"], "UNIT_REQUIRED")

    def test_cli_import_safe_and_collect_once_refuses(self):
        importlib.reload(cli)
        with redirect_stderr(StringIO()) as err:
            self.assertEqual(cli.main([]), 2)
            self.assertEqual(cli.main(["collect-once", "--admission", "nope"]), 2)
        self.assertIn("no admission", err.getvalue())
        with redirect_stdout(StringIO()) as out:
            self.assertEqual(cli.main(["describe"]), 0)
        payload = json.loads(out.getvalue())
        self.assertIsNone(payload["admission"])
        self.assertFalse(payload["liveCollectionAuthorized"])
        self.assertEqual(payload["caller"], "loan_exit_operator.retain_loan_main_exit")
        script = SCRIPTS / "moriarty_dev" / "loan_exit_retention" / "cli.py"
        direct = subprocess.run([sys.executable, str(script), "describe"], capture_output=True, text=True)
        self.assertEqual(direct.returncode, 0, direct.stderr)
        self.assertEqual(json.loads(direct.stdout)["caller"], payload["caller"])

    def test_controlled_subprocess_standin_not_live_service(self):
        with self.assertRaisesRegex(TypeError, "EXPLICIT_RUNNER_REQUIRED"):
            UserSystemctlTransport(None)
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            shown = evidence / "show.txt"
            shown.write_text(_swap_stdout())

            def runner(argv):
                kind = "show" if "show" in argv else "stop"
                return subprocess.run([sys.executable, "-c",
                                       "import pathlib,sys; p=pathlib.Path(sys.argv[1]); "
                                       "sys.stdout.write(p.read_text() if sys.argv[2]=='show' else '')",
                                       str(shown), kind], capture_output=True, text=True, timeout=5)

            result = self._retain(UserSystemctlTransport(runner), _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(result["exitRetained"])
            self.assertEqual(result["sequence"], ["show", "persist", "stop"])

    def test_preexisting_stop_evidence_is_not_fresh_retention(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            stop = evidence / "explicit-stop.json"
            stop.write_text('{"preserved":true}\n')
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertEqual(result["rejected"], "STOP_EVIDENCE_PRESENT")
            self.assertEqual(transport.calls, [])
            self.assertFalse((evidence / "terminal-observation.json").exists())

    def test_running_then_exited_same_evidence_dir(self):
        invocation = _swap_startup()["InvocationID"]
        running = ("LoadState=loaded\nActiveState=active\nSubState=running\nType=exec\nRemainAfterExit=yes\n"
                   "Transient=yes\nMainPID=123\nResult=\nExecMainCode=0\nExecMainStatus=0\nInvocationID="
                   + invocation + "\nControlGroup=/user.slice/unit.service\n")
        transport = StatefulTransport(running, running)
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            first = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertEqual(first["status"], "NOT_TERMINAL")
            self.assertTrue(first["pending"])
            self.assertEqual(first["classification"]["status"], "pending")
            self.assertFalse((evidence / "terminal-observation.json").exists())
            transport.loaded_stdout = _swap_stdout()
            second = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(second["exitRetained"])

    def test_stop_rc0_receipt_persistence_failed(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            real_save = exit_retention.durable_exclusive_save

            def wrapped(path, value, timeout_seconds=None):
                if pathlib.Path(path).name == "explicit-stop.json":
                    raise OSError(28, "No space left on device")
                return real_save(path, value, timeout_seconds)

            exit_retention.durable_exclusive_save = wrapped
            try:
                result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            finally:
                exit_retention.durable_exclusive_save = real_save
            self.assertEqual(result["status"], "STOP_RECEIPT_PERSISTENCE_FAILED")
            self.assertEqual(result["stopReturnCode"], 0)
            self.assertFalse(result["stopReceiptPersisted"])

    # --- new: explicit remaining-time bounds -----------------------------------
    def test_explicit_bounds_are_the_retained_argv(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence, show_seconds=3, stop_seconds=7)
            self.assertEqual(transport.bounds, [("show", 3), ("stop", 7)])
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertEqual(saved["argv"][:3], ["/usr/bin/timeout", "--signal=KILL", "3s"])
            self.assertEqual(saved["showBoundSeconds"], 3)
            stop = json.loads((evidence / "explicit-stop.json").read_text())
            self.assertEqual(stop["argv"][:3], ["/usr/bin/timeout", "--signal=KILL", "7s"])
            self.assertEqual(result["stopArgv"][2], "7s")
        self.assertEqual(terminal_show_argv("u.service", 50)[2], "5s")
        self.assertEqual(explicit_stop_argv("u.service", 90)[2], "25s")
        with self.assertRaisesRegex(ValueError, "BOUND_REQUIRED"):
            terminal_show_argv("u.service", 0)
        with self.assertRaisesRegex(ValueError, "BOUND_REQUIRED"):
            explicit_stop_argv("u.service", -1)
        with tempfile.TemporaryDirectory() as tmp:
            bad = self._retain(ScriptedTransport(_swap_stdout()), _swap_startup(), SWAP_UNIT, pathlib.Path(tmp),
                               show_seconds=0)
            self.assertEqual(bad["rejected"], "BOUND_REQUIRED")
        self.assertEqual(describe_contract()["explicitStopArgvTemplate"][2], "25s")

    def test_nonzero_and_signal_terminals_are_retained_and_classified(self):
        invocation = _swap_startup()["InvocationID"]
        base = ("LoadState=loaded\nActiveState=%s\nSubState=%s\nType=exec\nRemainAfterExit=yes\nTransient=yes\n"
                "MainPID=0\nResult=%s\nExecMainCode=%s\nExecMainStatus=%s\nInvocationID=" + invocation + "\n"
                "ControlGroup=\n")
        cases = [
            (base % ("failed", "failed", "exit-code", "1", "1"), "PROCESS_FAILED", "MAIN_EXIT_NONZERO", {"kind": "exit", "code": 1}),
            (base % ("active", "exited", "exit-code", "1", "255"), "PROCESS_FAILED", "MAIN_EXIT_NONZERO", {"kind": "exit", "code": 255}),
            (base % ("failed", "failed", "signal", "2", "9"), "PROCESS_FAILED", "MAIN_SIGNAL", {"kind": "signal", "code": 9}),
            (base % ("failed", "failed", "core-dump", "3", "64"), "PROCESS_FAILED", "MAIN_SIGNAL", {"kind": "signal", "code": 64}),
            (base % ("failed", "failed", "signal", "2", "65"), "PROCESS_UNKNOWN", "MAIN_OBSERVATION_INVALID", {"kind": "unknown", "code": None}),
        ]
        for stdout, status, code, raw in cases:
            with self.subTest(status=status, code=code, raw=raw):
                with tempfile.TemporaryDirectory() as tmp:
                    evidence = pathlib.Path(tmp)
                    transport = ScriptedTransport(stdout, evidence_dir=evidence)
                    result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
                    self.assertFalse(result["exitRetained"])
                    self.assertFalse(result["stopAttempted"], "a nonzero exit never triggers the collector stop")
                    self.assertEqual(result["classification"]["status"], status)
                    self.assertEqual(result["classification"]["failureCode"], code)
                    self.assertEqual(result["classification"]["rawMainExit"], raw)
                    saved = json.loads((evidence / "terminal-observation.json").read_text())
                    self.assertEqual(saved["classification"]["rawMainExit"], raw)

    def test_bounded_persistence_worker_kills_a_stalled_write(self):
        started = time.monotonic()
        outcome = run_bounded(lambda: time.sleep(5), 0.2)
        self.assertEqual(outcome.status, "timeout")
        self.assertLess(time.monotonic() - started, 2.0)
        with tempfile.TemporaryDirectory() as tmp:
            target = pathlib.Path(tmp) / "record.json"
            durable_exclusive_save(target, {"ok": 1}, timeout_seconds=2)
            self.assertEqual(json.loads(target.read_text()), {"ok": 1})
            with self.assertRaises(FileExistsError):
                durable_exclusive_save(target, {"ok": 2}, timeout_seconds=2)
            real = exit_retention._write_exclusive_sync
            exit_retention._write_exclusive_sync = lambda path, raw: time.sleep(5)
            try:
                with self.assertRaises(PersistenceTimeout):
                    durable_exclusive_save(pathlib.Path(tmp) / "stalled.json", {"x": 1}, timeout_seconds=0.2)
            finally:
                exit_retention._write_exclusive_sync = real
        value = run_bounded(lambda: {"digest": "abc"}, 2)
        self.assertEqual(value.value, {"digest": "abc"})


class TerminalPredicate(unittest.TestCase):
    LOADED = {
        "LoadState": "loaded", "ActiveState": "active", "SubState": "exited", "Type": "exec",
        "RemainAfterExit": "yes", "Transient": "yes", "MainPID": "0", "Result": "success",
        "ExecMainCode": "1", "ExecMainStatus": "0", "InvocationID": "103af569b7b74c81ac2147fe3a263348",
    }

    def test_retained_same_invocation(self):
        result = require_exit_zero(self.LOADED, self.LOADED["InvocationID"])
        self.assertEqual(result["exitCode"], 0)
        for key, value in [("LoadState", "not-found"), ("ExecMainCode", "0"), ("ExecMainStatus", "1"),
                           ("ActiveState", "inactive"), ("MainPID", "12"), ("InvocationID", "a" * 32),
                           ("SubState", "running"), ("RemainAfterExit", "no"), ("Transient", "no"),
                           ("Result", "exit-code")]:
            with self.subTest(key=key, value=value):
                with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
                    require_exit_zero(dict(self.LOADED, **{key: value}), self.LOADED["InvocationID"])

    def test_missing_fields(self):
        for key in self.LOADED:
            absent = dict(self.LOADED)
            del absent[key]
            with self.subTest(missing=key):
                with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
                    require_exit_zero(absent, self.LOADED["InvocationID"])

    def test_authentic_swap_observation_establishes_exit_zero(self):
        fields = json.loads(SWAP_TERMINAL.read_text())["observed"]
        result = require_exit_zero(fields, fields["InvocationID"])
        self.assertEqual(result["invocationId"], "b58efe5700db4dd3a63fd097fc5a1652")
        self.assertEqual(classify_main_exit(fields, fields["InvocationID"])["status"], "PROCESS_SUCCESS")

    def test_authentic_loan_unloaded_observation_is_not_an_exit(self):
        raw = json.loads(LOAN_TERMINAL.read_text())["systemdShow"]
        fields = dict(line.split("=", 1) for line in raw.splitlines() if line)
        with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
            require_exit_zero(fields, "1" * 32)
        self.assertEqual(classify_main_exit(fields, "1" * 32)["failureCode"], "MAIN_OBSERVATION_INVALID")

    def test_closed_classification_table(self):
        inv = self.LOADED["InvocationID"]

        def fields(**patch):
            return dict(self.LOADED, **patch)
        self.assertEqual(classify_main_exit(fields(SubState="running", MainPID="77", ExecMainCode="0"), inv)["status"], "pending")
        self.assertEqual(classify_main_exit(fields(SubState="running", MainPID="77"), inv)["status"], "PROCESS_UNKNOWN",
                         "a running row carrying a terminal main record is contradictory")
        for status_code in range(1, 256):
            row = classify_main_exit(fields(ActiveState="failed", SubState="failed", Result="exit-code",
                                            ExecMainStatus=str(status_code)), inv)
            self.assertEqual((row["status"], row["failureCode"], row["rawMainExit"]),
                             ("PROCESS_FAILED", "MAIN_EXIT_NONZERO", {"kind": "exit", "code": status_code}))
        for sig in range(1, 65):
            row = classify_main_exit(fields(ActiveState="failed", SubState="failed", Result="signal",
                                            ExecMainCode="2", ExecMainStatus=str(sig)), inv)
            self.assertEqual((row["status"], row["failureCode"], row["rawMainExit"]),
                             ("PROCESS_FAILED", "MAIN_SIGNAL", {"kind": "signal", "code": sig}))
        unknown = {"kind": "unknown", "code": None}
        for patch in ({"LoadState": "not-found"}, {"ExecMainCode": "0"}, {"ExecMainStatus": "zero"},
                      {"InvocationID": "b" * 32}, {"ActiveState": "deactivating", "SubState": "stop-sigterm"},
                      {"Type": "simple"}, {"RemainAfterExit": "no"}, {"Transient": "no"},
                      {"MainPID": "5"}, {"Result": "exit-code"}, {"ExecMainStatus": "256", "Result": "exit-code"},
                      {"ExecMainCode": "2", "ExecMainStatus": "0", "Result": "signal"},
                      {"SubState": "running", "MainPID": "0"}):
            with self.subTest(patch=patch):
                row = classify_main_exit(fields(**patch), inv)
                self.assertEqual(row["status"], "PROCESS_UNKNOWN")
                self.assertEqual(row["failureCode"], "MAIN_OBSERVATION_INVALID")
                self.assertEqual(row["rawMainExit"], unknown)
        zero = classify_main_exit(self.LOADED, inv)
        self.assertEqual(zero["rawMainExit"], {"kind": "exit", "code": 0})
        with self.assertRaisesRegex(ValueError, "INVOCATION_REQUIRED"):
            classify_main_exit(self.LOADED, "0" * 32)


if __name__ == "__main__":
    unittest.main()
