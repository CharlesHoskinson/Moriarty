"""Two ordinary coherent reporting fixtures; no subject or tool execution.

The selected checker is the production sibling check.py. There is no
environment override or candidate fallback.
Independent complete expected records/input graphs predate implementation.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
CHECKER_PATH = HERE / 'check.py'
spec = importlib.util.spec_from_file_location('selected_production_sherlock_checker', CHECKER_PATH)
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


class CompleteCompileEvidenceRecordPositive(unittest.TestCase):
    def complete_record(self, case):
        frozen = json.loads((HERE / 'fixtures/compile-evidence-record/FREEZE.json').read_text())
        entry = next(row for row in frozen['cases'] if row['case'] == case)
        root = HERE / 'fixtures/compile-evidence-record' / case
        expected_bytes = (root / 'EXPECTED.json').read_bytes()
        self.assertEqual(hashlib.sha256(expected_bytes).hexdigest(), entry['expected']['sha256'])
        for name, identity in entry['inputGraph'].items():
            raw = (root / 'fixture' / name).read_bytes()
            self.assertEqual(len(raw), identity['bytes'])
            self.assertEqual(hashlib.sha256(raw).hexdigest(), identity['sha256'])
        expected = json.loads(expected_bytes)
        with tempfile.TemporaryDirectory(prefix='sherlock-evidence-positive-') as temporary:
            repo = Path(temporary)
            shutil.copytree(root / 'fixture', repo / 'fixture')
            # The same explicit production API is used by optional CLI reporting.
            actual = checker.compile_evidence_record(repo, Path('fixture'), case)
        self.assertEqual(actual, expected, 'complete independent seven-field record, not selected fields')
        self.assertEqual(actual['acceptance'], 'none')
        self.assertEqual(actual['source_tree']['current_source_equality'], 'not-checked')
        self.assertEqual(actual['consumer']['invocation_authenticity'], 'not-established')
        return actual

    def test_compile_only_complete_record(self):
        actual = self.complete_record('quote')
        self.assertEqual(actual['predicate']['id'], 'sherlock.compile_host.prepare_admit/1')
        self.assertEqual(actual['evidence_class']['kind'], 'recorded-compile-only-artifact')
        self.assertEqual(actual['evidence_class']['artifact_evidence']['host_observation'], 'unrun')

    def test_reported_observation_is_a_distinct_complete_class(self):
        actual = self.complete_record('eligibility')
        self.assertEqual(actual['predicate']['id'], 'sherlock.compile_host.prepare_admit_observe/1')
        self.assertEqual(actual['evidence_class']['kind'], 'recorded-host-observation-artifact')
        self.assertEqual(actual['evidence_class']['artifact_evidence']['host_observation'], 'observed')


if __name__ == '__main__':
    unittest.main()
