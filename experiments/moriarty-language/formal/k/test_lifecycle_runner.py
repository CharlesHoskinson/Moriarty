"""Offline runner suite isolation for financial-lifecycle-v1. No K subprocess."""
import json
import unittest
from pathlib import Path
from unittest.mock import patch
import run

HERE = Path(__file__).resolve().parent


class LifecycleRunnerTests(unittest.TestCase):
    def test_lifecycle_suite_is_registered_without_replacing_old_suites(self):
        self.assertEqual(run.SUITE_LIMITS['initial'], (16, 512))
        self.assertEqual(run.SUITE_LIMITS['branches'], (16, 512))
        self.assertEqual(run.SUITE_LIMITS['transfer-only'], (16, 512))
        self.assertEqual(run.SUITE_LIMITS['numeric'], (64, 1512))
        self.assertIn('financial-lifecycle-v1', run.SUITES)
        self.assertIn('financial-lifecycle-v1', run.SUITE_LIMITS)

    def test_lifecycle_evaluate_does_not_dispatch_k_without_admission(self):
        if not hasattr(run, 'evaluate_lifecycle'):
            self.fail('run.evaluate_lifecycle must exist')
        with patch.object(run, 'command') as command:
            with self.assertRaisesRegex(run.RunnerError, 'K_NOT_ADMITTED'):
                run.evaluate_lifecycle({'schema': '{}', 'request': '{}', 'financialPreState': '{}'}, 'offline')
            command.assert_not_called()

    def test_lifecycle_request_limit_is_not_old_funded_packet_limit(self):
        import lifecycle_codec as lc
        self.assertEqual(lc.REQUEST_LIMIT, 2_000_000)
        self.assertEqual(lc.SCHEMA_LIMIT, 65536)
        self.assertEqual(lc.FINANCIAL_LIMIT, 65536)


if __name__ == '__main__':
    unittest.main()
