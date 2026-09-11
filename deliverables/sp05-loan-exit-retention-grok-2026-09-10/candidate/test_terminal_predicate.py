import json
import pathlib
import sys
import unittest

_CANDIDATE = pathlib.Path(__file__).resolve().parent
if str(_CANDIDATE) not in sys.path:
    sys.path.insert(0, str(_CANDIDATE))

from terminal_predicate import require_exit_zero

DELIVERABLES = pathlib.Path(__file__).resolve().parents[2]
LOAN_TERMINAL = (
    DELIVERABLES
    / "sp05-financial-integration-2026-09-09"
    / "preview-loan-exit-01"
    / "terminal-observation-01.json"
)
SWAP_TERMINAL = (
    DELIVERABLES
    / "sp05-financial-integration-2026-09-09"
    / "preview-swap-exit-01"
    / "terminal-observation-01.json"
)

LOADED = {
    "LoadState": "loaded",
    "ActiveState": "active",
    "SubState": "exited",
    "Type": "exec",
    "RemainAfterExit": "yes",
    "Transient": "yes",
    "MainPID": "0",
    "Result": "success",
    "ExecMainCode": "1",
    "ExecMainStatus": "0",
    "InvocationID": "103af569b7b74c81ac2147fe3a263348",
}


class TerminalPredicate(unittest.TestCase):
    def test_retained_same_invocation(self):
        result = require_exit_zero(LOADED, LOADED["InvocationID"])
        self.assertEqual(result["exitCode"], 0)
        self.assertEqual(result["status"], "EXIT_ZERO_OBSERVED")
        self.assertFalse(result["externalContainmentEstablished"])
        for key, value in [
            ("LoadState", "not-found"),
            ("ExecMainCode", "0"),
            ("ExecMainStatus", "1"),
            ("ActiveState", "inactive"),
            ("MainPID", "12"),
            ("InvocationID", "a" * 32),
            ("SubState", "running"),
            ("RemainAfterExit", "no"),
            ("Transient", "no"),
            ("Result", "exit-code"),
        ]:
            with self.subTest(key=key, value=value):
                with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
                    require_exit_zero(dict(LOADED, **{key: value}), LOADED["InvocationID"])

    def test_missing_fields(self):
        for key in LOADED:
            absent = dict(LOADED)
            del absent[key]
            with self.subTest(missing=key):
                with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
                    require_exit_zero(absent, LOADED["InvocationID"])

    def test_invalid_expected_invocation(self):
        for value in [None, "", "0" * 32, "abc", "A" * 32, "1" * 31, "1" * 33, 1]:
            with self.subTest(value=value):
                with self.assertRaisesRegex(ValueError, "INVOCATION_REQUIRED"):
                    require_exit_zero(LOADED, value)

    def test_authentic_swap_observation_establishes_exit_zero(self):
        payload = json.loads(SWAP_TERMINAL.read_text())
        fields = payload["observed"]
        result = require_exit_zero(fields, fields["InvocationID"])
        self.assertEqual(result["exitCode"], 0)
        self.assertEqual(result["invocationId"], "b58efe5700db4dd3a63fd097fc5a1652")
        self.assertFalse(result["externalContainmentEstablished"])

    def test_authentic_loan_unloaded_observation_is_not_an_exit(self):
        raw = json.loads(LOAN_TERMINAL.read_text())["systemdShow"]
        fields = dict(line.split("=", 1) for line in raw.splitlines() if line)
        with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
            require_exit_zero(fields, "1" * 32)
        self.assertEqual(fields.get("LoadState"), "not-found")
        self.assertEqual(fields.get("ExecMainCode"), "0")
        self.assertEqual(fields.get("ExecMainStatus"), "0")

    def test_financial_complete_mapping_is_not_exit_zero(self):
        with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
            require_exit_zero({"FINANCIAL_COMPLETE": "yes"}, LOADED["InvocationID"])
        mixed = dict(LOADED)
        mixed["FINANCIAL_COMPLETE"] = "yes"
        # extra keys do not satisfy missing required identity; required keys still must match
        result = require_exit_zero(mixed, LOADED["InvocationID"])
        self.assertEqual(result["exitCode"], 0)
        failed = dict(LOADED)
        failed["LoadState"] = "not-found"
        failed["FINANCIAL_COMPLETE"] = "yes"
        with self.assertRaisesRegex(ValueError, "EXIT_ZERO_NOT_ESTABLISHED"):
            require_exit_zero(failed, LOADED["InvocationID"])


if __name__ == "__main__":
    unittest.main()
