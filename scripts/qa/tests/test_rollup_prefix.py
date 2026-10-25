"""Incomplete and reused prefix runs cannot become completed benchmark evidence."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('prefix', Path(__file__).parents[1] / 'check-prefix-evidence.py')
prefix = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(prefix)


class PrefixEvidenceTests(unittest.TestCase):
    def test_missing_workload_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(OSError):
                prefix.check(tmp)

    def test_censored_or_reused_workload_cannot_pass(self):
        complete = {'schema': 'sherlock-b5-workload/1', 'prefixes': [2, 4, 6],
                    'status': 'completed', 'transactions_per_batch': 16,
                    'headers_per_batch': 50, 'samples_per_prefix': 1,
                    'proof_reuse_between_prefixes': False}
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'workload.json'
            for changes in ({'status': 'running'}, {'status': 'censored'},
                            {'proof_reuse_between_prefixes': True}, {'prefixes': [1, 2, 3]}):
                path.write_text(json.dumps({**complete, **changes}))
                with self.assertRaisesRegex(ValueError, 'completed independent exact prefix'):
                    prefix.check(tmp)
