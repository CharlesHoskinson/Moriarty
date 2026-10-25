"""Measurement failures must remain failures and still leave resource evidence."""
import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('rollup_measure', Path(__file__).parents[1] / 'measure-rollup.py')
measure = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(measure)


class MeasureTests(unittest.TestCase):
    def test_terminal_failure_keeps_exit_and_resource_scope(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = measure.run([sys.executable, '-c', 'raise SystemExit(7)'], Path(tmp) / 'measurement.json')
            self.assertEqual(result['exit'], 7)
            self.assertGreater(result['wall_seconds'], 0)
            self.assertGreater(result['max_rss_bytes'], 0)
            self.assertEqual(result['scope'], 'one child process including all threads; build excluded')

    def test_success_records_swap_sampling_and_cpu(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = measure.run([sys.executable, '-c', 'x=bytearray(1000000)'], Path(tmp) / 'measurement.json')
            self.assertEqual(result['exit'], 0)
            self.assertGreaterEqual(result['swap_samples'], 1)
            self.assertGreaterEqual(result['swap_peak_bytes'], 0)
            self.assertGreaterEqual(result['cpu_user_seconds'], 0)
