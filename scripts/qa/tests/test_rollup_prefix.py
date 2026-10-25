"""Incomplete and reused prefix runs cannot become completed benchmark evidence."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import shutil

SPEC = importlib.util.spec_from_file_location('prefix', Path(__file__).parents[1] / 'check-prefix-evidence.py')
prefix = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(prefix)


class PrefixEvidenceTests(unittest.TestCase):
    def test_copied_public_proof_is_not_an_independent_prefix_sample(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for count in (2, 4, 6):
                for batch in range(1, count + 1):
                    directory = root / f'prefix-{count}/batch-{batch:02}'
                    directory.mkdir(parents=True)
                    for layer in ('leaf', 'parent', 'decider'):
                        (directory / f'{layer}.proof').write_bytes(f'{count}/{batch}/{layer}'.encode())
            prefix.reject_reused_proofs(root)
            shutil.copyfile(root / 'prefix-6/batch-01/leaf.proof', root / 'prefix-2/batch-01/leaf.proof')
            with self.assertRaisesRegex(ValueError, 'proof reused across independent prefixes'):
                prefix.reject_reused_proofs(root)

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
