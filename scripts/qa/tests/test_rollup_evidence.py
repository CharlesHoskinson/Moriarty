"""Evidence checks must refuse missing stages and altered public artifacts."""
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('rollup_evidence', Path(__file__).parents[1] / 'check-rollup-evidence.py')
evidence = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(evidence)


class EvidenceTests(unittest.TestCase):
    def test_profile_cannot_override_a_newer_b3_relation(self):
        with tempfile.TemporaryDirectory() as tmp:
            target = Path(tmp)
            old = target / 'sherlock-b3/run-old'
            new = target / 'sherlock-b3/run-new'
            profile = target / 'sherlock-b7/run-profile'
            for root in (old, new, profile):
                root.mkdir(parents=True)
            (old / 'verified.json').write_text('{}')
            (new / 'verified.json').write_text('{}')
            os.utime(old / 'verified.json', ns=(1, 1))
            os.utime(new / 'verified.json', ns=(2, 2))
            (profile / 'profile.json').write_text(json.dumps({'reference': str(old.resolve())}))
            self.assertEqual(evidence.latest_run('B7', target), new)
            (profile / 'profile.json').write_text(json.dumps({'reference': str(new.resolve())}))
            self.assertEqual(evidence.latest_run('B7', target), profile)

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
