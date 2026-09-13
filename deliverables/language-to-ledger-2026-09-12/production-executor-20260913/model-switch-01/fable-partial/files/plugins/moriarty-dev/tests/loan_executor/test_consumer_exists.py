"""Reproduce the missing production consumer before any repair.

These tests name the installed modules, the bound-runner executor profile and
the CLI result mapping that the afk-loan-executor specification requires. They
must fail while the consumer is absent and pass only through installed source.
"""
import importlib
import sys
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[2] / "scripts"
if str(SCRIPTS) not in sys.path:
    sys.path.insert(0, str(SCRIPTS))


class ConsumerPresence(unittest.TestCase):
    def test_installed_modules_import_without_side_effects(self):
        for name in ("moriarty_dev.loan_executor", "moriarty_dev.accounting",
                     "moriarty_dev.loan_cleanup", "moriarty_dev.loan_exit_retention",
                     "moriarty_dev.loan_exit_retention.loan_exit_operator"):
            with self.subTest(module=name):
                importlib.import_module(name)

    def test_runner_supports_executor_profile(self):
        from moriarty_dev import runner
        self.assertTrue(hasattr(runner, "EXECUTOR_SCHEMA"), "runner has no loan executor invocation profile")
        self.assertTrue(callable(getattr(runner, "execute_loan_invocation", None)))

    def test_accounting_exposes_real_validator(self):
        from moriarty_dev import accounting
        self.assertTrue(callable(getattr(accounting, "validate_executor_context", None)))
        self.assertTrue(hasattr(accounting, "Refusal") and hasattr(accounting, "CurrentExecution"))

    def test_cli_maps_loan_process_result(self):
        from moriarty_dev import cli
        self.assertTrue(callable(getattr(cli, "_loan_disposition", None)),
                        "cmd_run has no loan-process-result disposition mapping")


if __name__ == "__main__":
    unittest.main()
