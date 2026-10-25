"""Evidence checks must refuse missing stages and altered public artifacts."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('rollup_evidence', Path(__file__).parents[1] / 'check-rollup-evidence.py')
evidence = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


class EvidenceTests(unittest.TestCase):
    def test_missing_result_cannot_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises((ValueError, OSError)):
                evidence.check('B3', Path(tmp))

    def test_hash_check_rejects_altered_artifact(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'leaf.proof'
            path.write_bytes(b'public proof')
            digest = evidence.digest(path)
            evidence.require_digest(path, digest)
            path.write_bytes(b'changed proof')
            with self.assertRaisesRegex(ValueError, 'digest mismatch'):
                evidence.require_digest(path, digest)
