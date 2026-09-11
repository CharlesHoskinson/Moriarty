import importlib
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from io import StringIO

_CANDIDATE = pathlib.Path(__file__).resolve().parent
if str(_CANDIDATE) not in sys.path:
    sys.path.insert(0, str(_CANDIDATE))

import cli
from exit_retention import (
    CommandResult,
    UserSystemctlTransport,
    parse_systemctl_show,
    retain_exit_evidence,
)
from launch_contract import require_startup_identity
from loan_exit_operator import retain_loan_main_exit

_FIN = pathlib.Path(__file__).resolve().parents[2] / "sp05-financial-integration-2026-09-09"
LOAN_TERMINAL = _FIN / "preview-loan-exit-01" / "terminal-observation-01.json"
LOAN_STARTUP = _FIN / "preview-loan-exit-01" / "launcher-active.json"
SWAP_TERMINAL = _FIN / "preview-swap-exit-01" / "terminal-observation-01.json"
SWAP_STARTUP = _FIN / "preview-swap-exit-01" / "launcher-active.json"
SWAP_UNIT = "moriarty-sp05-preview-swap-exit-01.service"
LOAN_UNIT = "moriarty-sp05-preview-loan-exit-01.service"
TEST_UNIT = "moriarty-sp05-loan-exit-next.service"
FIXED_CLOCK = lambda: "2026-09-10T00:00:00+00:00"


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

    def show(self, unit, properties):
        present = False
        if self.evidence_dir is not None:
            present = (self.evidence_dir / "terminal-observation.json").is_file()
        self.calls.append(("show", unit, properties, present))
        return CommandResult(
            ["/usr/bin/systemctl", "--user", "show", unit, "--property=" + properties],
            self.show_code,
            self.show_stdout,
            "",
        )

    def stop(self, unit):
        present = False
        if self.evidence_dir is not None:
            present = (self.evidence_dir / "terminal-observation.json").is_file()
        self.observation_present_at_stop = present
        self.calls.append(("stop", unit, present))
        if self.stop_raises is not None:
            raise self.stop_raises
        return CommandResult(
            ["/usr/bin/timeout", "--signal=KILL", "25s", "/usr/bin/systemctl", "--user", "stop", unit],
            self.stop_code,
            "",
            "",
        )


class StatefulTransport(object):
    def __init__(self, loaded_stdout, unloaded_stdout):
        self.loaded_stdout = loaded_stdout
        self.unloaded_stdout = unloaded_stdout
        self.stopped = False

    def show(self, unit, properties):
        stdout = self.unloaded_stdout if self.stopped else self.loaded_stdout
        return CommandResult(["show", unit], 0, stdout, "")

    def stop(self, unit):
        self.stopped = True
        return CommandResult(["stop", unit], 0, "", "")


class ExitRetention(unittest.TestCase):
    def _retain(self, transport, startup, unit, evidence_dir):
        return retain_loan_main_exit(
            unit=unit,
            startup_observed=startup,
            transport=transport,
            evidence_dir=evidence_dir,
            now_utc=FIXED_CLOCK,
        )

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
            result = retain_exit_evidence(
                SWAP_UNIT, _swap_startup(), transport, evidence, now_utc=FIXED_CLOCK
            )
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
            self.assertIsNone(result["predicate"])
            self.assertFalse(result["stopAttempted"])
            self.assertFalse((evidence / "explicit-stop.json").exists())
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertEqual(saved["stdout"], _loan_stdout())
            self.assertNotEqual(saved.get("predicate"), {"status": "EXIT_ZERO_OBSERVED"})
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
            self.assertIsNone(result["startupInvocationId"])
            self.assertFalse(result["stopAttempted"])
            self.assertEqual(transport.calls, [])
            self.assertFalse((evidence / "terminal-observation.json").exists())
            self.assertFalse((evidence / "explicit-stop.json").exists())

    def test_invalid_startup_ids(self):
        base = dict(_swap_startup())
        for value in [None, "", "0" * 32, "ABC" * 10 + "abcd", "1" * 31, "g" * 32]:
            observed = dict(base)
            observed["InvocationID"] = value
            with self.subTest(value=value):
                with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
                    require_startup_identity(observed)
        unloaded = dict(base)
        unloaded["LoadState"] = "not-found"
        with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
            require_startup_identity(unloaded)
        dead = dict(base)
        dead["MainPID"] = "0"
        with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
            require_startup_identity(dead)

    def test_identity_mismatch(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            startup = dict(_swap_startup())
            startup["InvocationID"] = "c" * 32
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, startup, SWAP_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "EXIT_ZERO_NOT_ESTABLISHED")
            self.assertEqual(result["startupInvocationId"], "c" * 32)
            self.assertFalse(result["stopAttempted"])
            self.assertFalse((evidence / "explicit-stop.json").exists())

    def test_duplicate_and_malformed_show(self):
        with self.assertRaisesRegex(ValueError, "DUPLICATE_FIELD"):
            parse_systemctl_show("LoadState=loaded\nLoadState=loaded\n")
        with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
            parse_systemctl_show("FINANCIAL_COMPLETE\nLoadState=loaded\n")
        with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
            parse_systemctl_show("")
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(
                "LoadState=loaded\nLoadState=not-found\n", evidence_dir=evidence
            )
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
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertEqual(result["rejected"], "PERSISTENCE_FAILED")
            self.assertIsNone(result["predicate"])
            self.assertEqual((evidence / "terminal-observation.json").read_text(), "preexisting\n")
            self.assertFalse(result["stopAttempted"])
            self.assertEqual([c[0] for c in transport.calls], ["show"])
            self.assertFalse((evidence / "explicit-stop.json").exists())

    def test_failed_stop_after_retention(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(_swap_stdout(), stop_code=5, evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(result["exitRetained"])
            self.assertFalse(result["explicitStopCompleted"])
            self.assertEqual(result["status"], "STOP_FAILED_AFTER_RETENTION")
            self.assertFalse(result["financialAcceptance"])
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertEqual(saved["predicate"]["status"], "EXIT_ZERO_OBSERVED")

    def test_stop_exception_after_retention_is_not_command_success(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = ScriptedTransport(
                _swap_stdout(), evidence_dir=evidence, stop_raises=OSError("stop")
            )
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(result["exitRetained"])
            self.assertFalse(result["explicitStopCompleted"])
            self.assertEqual(result["status"], "STOP_FAILED_AFTER_RETENTION")

    def test_post_stop_observation_rejected(self):
        transport = StatefulTransport(_swap_stdout(), _loan_stdout())
        with tempfile.TemporaryDirectory() as first:
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, pathlib.Path(first))
            self.assertTrue(result["exitRetained"])
            self.assertTrue(transport.stopped)
        with tempfile.TemporaryDirectory() as second:
            later = self._retain(transport, _swap_startup(), SWAP_UNIT, pathlib.Path(second))
            self.assertFalse(later["exitRetained"])
            self.assertFalse(later["stopAttempted"])
            self.assertEqual(later["rejected"], "EXIT_ZERO_NOT_ESTABLISHED")
            saved = json.loads((pathlib.Path(second) / "terminal-observation.json").read_text())
            self.assertEqual(saved["observed"]["LoadState"], "not-found")

    def test_financial_text_cannot_bypass_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            result = self._retain(
                ScriptedTransport("FINANCIAL_COMPLETE=yes\n", evidence_dir=evidence),
                _swap_startup(),
                TEST_UNIT,
                evidence,
            )
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "MALFORMED_SHOW")
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertIsNone(result["predicate"])
            saved = json.loads((evidence / "terminal-observation.json").read_text())
            self.assertIn("FINANCIAL_COMPLETE", saved["stdout"])
            with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
                parse_systemctl_show(saved["stdout"])
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            rescued = _loan_stdout() + "FINANCIAL_COMPLETE=yes\n"
            result = self._retain(
                ScriptedTransport(rescued, evidence_dir=evidence),
                _swap_startup(),
                LOAN_UNIT,
                evidence,
            )
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertEqual(result["rejected"], "MALFORMED_SHOW")
            self.assertNotEqual(result["status"], "EXIT_RETAINED_THEN_STOP_COMPLETED")
            with self.assertRaisesRegex(ValueError, "MALFORMED_SHOW"):
                parse_systemctl_show(rescued)

    def test_show_failed_and_bad_inputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            result = self._retain(
                ScriptedTransport("", show_code=1, evidence_dir=evidence),
                _swap_startup(),
                TEST_UNIT,
                evidence,
            )
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "SHOW_FAILED")
        missing = retain_loan_main_exit(
            SWAP_UNIT, _swap_startup(), ScriptedTransport(_swap_stdout()), "/no/such/loan-exit-dir", now_utc=FIXED_CLOCK
        )
        self.assertEqual(missing["rejected"], "EVIDENCE_DIR_REQUIRED")
        self.assertFalse(missing["stopAttempted"])
        with tempfile.TemporaryDirectory() as tmp:
            bad = retain_loan_main_exit(
                "../evil.service", _swap_startup(), ScriptedTransport(_swap_stdout()), pathlib.Path(tmp), now_utc=FIXED_CLOCK
            )
            self.assertEqual(bad["rejected"], "UNIT_REQUIRED")
            self.assertFalse(bad["stopAttempted"])

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
        self.assertFalse(payload["financialAcceptance"])
        self.assertEqual(payload["caller"], "loan_exit_operator.retain_loan_main_exit")

    def test_controlled_subprocess_standin_not_live_service(self):
        with self.assertRaisesRegex(TypeError, "EXPLICIT_RUNNER_REQUIRED"):
            UserSystemctlTransport(None)
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            shown = evidence / "show.txt"
            shown.write_text(_swap_stdout())

            def runner(argv):
                kind = "show" if "show" in argv else "stop"
                return subprocess.run(
                    [
                        sys.executable,
                        "-c",
                        "import pathlib,sys; p=pathlib.Path(sys.argv[1]); sys.stdout.write(p.read_text() if sys.argv[2]=='show' else '')",
                        str(shown),
                        kind,
                    ],
                    capture_output=True,
                    text=True,
                    timeout=5,
                )

            result = self._retain(
                UserSystemctlTransport(runner), _swap_startup(), SWAP_UNIT, evidence
            )
            self.assertTrue(result["exitRetained"])
            self.assertEqual(result["sequence"], ["show", "persist", "stop"])
            self.assertFalse(result["financialAcceptance"])
            self.assertIsNone(result["admission"])

    def test_startup_mainpid_must_be_canonical_positive_decimal_string(self):
        base = dict(_swap_startup())
        self.assertEqual(require_startup_identity(base), base["InvocationID"])
        self.assertRegex(base["MainPID"], r"^[1-9][0-9]*$")
        for value in [True, False, 2145499, 1, 1.5, "+1", "-1", " 1", "1 ", "0", "00", "01", "1e2", "1.0", "", None]:
            observed = dict(base)
            observed["MainPID"] = value
            with self.subTest(value=repr(value)):
                with self.assertRaisesRegex(ValueError, "STARTUP_IDENTITY_REQUIRED"):
                    require_startup_identity(observed)

    def test_preexisting_stop_evidence_is_not_fresh_retention(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            stop = evidence / "explicit-stop.json"
            preserved = '{"preserved":true}\n'
            stop.write_text(preserved)
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["status"], "RETENTION_FAILED")
            self.assertEqual(result["rejected"], "STOP_EVIDENCE_PRESENT")
            self.assertFalse(result["stopAttempted"])
            self.assertEqual(result["sequence"], [])
            self.assertEqual(transport.calls, [])
            self.assertEqual(stop.read_text(), preserved)
            self.assertFalse((evidence / "terminal-observation.json").exists())
        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            marker = evidence / "explicit-stop.json"
            marker.symlink_to("absent-explicit-stop.json")
            transport = ScriptedTransport(_swap_stdout(), evidence_dir=evidence)
            result = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertFalse(result["exitRetained"])
            self.assertEqual(result["rejected"], "STOP_EVIDENCE_PRESENT")
            self.assertFalse(result["stopAttempted"])
            self.assertEqual(transport.calls, [])
            self.assertTrue(marker.is_symlink())
            self.assertFalse((evidence / "terminal-observation.json").exists())

    def test_running_then_exited_same_evidence_dir(self):
        invocation = _swap_startup()["InvocationID"]
        running = (
            "LoadState=loaded\nActiveState=active\nSubState=running\n"
            "Type=exec\nRemainAfterExit=yes\nTransient=yes\nMainPID=123\n"
            "Result=\nExecMainCode=0\nExecMainStatus=0\n"
            "InvocationID=" + invocation + "\nControlGroup=/user.slice/unit.service\n"
        )

        class TwoStep(object):
            def __init__(self):
                self.calls = []
                self.stdout = running

            def show(self, unit, properties):
                self.calls.append(("show", unit))
                return CommandResult(["show", unit], 0, self.stdout, "")

            def stop(self, unit):
                self.calls.append(("stop", unit))
                return CommandResult(["stop", unit], 0, "", "")

        with tempfile.TemporaryDirectory() as tmp:
            evidence = pathlib.Path(tmp)
            transport = TwoStep()
            first = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertEqual(first["status"], "NOT_TERMINAL")
            self.assertTrue(first["pending"])
            self.assertFalse(first["exitRetained"])
            self.assertFalse(first["stopAttempted"])
            self.assertEqual(first["sequence"], ["show"])
            self.assertEqual(transport.calls, [("show", SWAP_UNIT)])
            self.assertFalse((evidence / "terminal-observation.json").exists())
            self.assertFalse((evidence / "explicit-stop.json").exists())
            transport.stdout = _swap_stdout()
            second = self._retain(transport, _swap_startup(), SWAP_UNIT, evidence)
            self.assertTrue(second["exitRetained"])
            self.assertFalse(second.get("pending"))
            self.assertEqual(second["sequence"], ["show", "persist", "stop"])
            self.assertEqual([c[0] for c in transport.calls], ["show", "show", "stop"])
            self.assertTrue((evidence / "terminal-observation.json").is_file())
            self.assertTrue((evidence / "explicit-stop.json").is_file())


if __name__ == "__main__":
    unittest.main()
