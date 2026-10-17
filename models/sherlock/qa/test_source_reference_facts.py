"""Computed read membership is distinct from reviews and execution evidence."""
import hashlib
import importlib.util
from pathlib import Path
import unittest

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]


def load(name):
    path = HERE / (name + '.py')
    spec = importlib.util.spec_from_file_location('dw_' + name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class SourceReferenceFactsTests(unittest.TestCase):
    def inventory(self):
        return load('source_reference_facts').ReadInventory()

    def test_empty_inventory_is_not_an_execution_or_review_claim(self):
        report = self.inventory().report()
        self.assertEqual(report['files'], [])
        self.assertEqual(report['read_events'], 0)
        self.assertEqual(report['unique_paths'], 0)
        self.assertEqual(report['bytes_read'], 0)
        self.assertEqual(report['scope'], 'explicit-checker-input-byte-reads')
        self.assertEqual(report['reviewer_read_extent'], 'not-observed')
        self.assertEqual(report['execution_evidence'], 'none')
        self.assertEqual(report['dependency_closure'], 'not-established')

    def test_hashes_exact_binary_bytes_without_text_normalization(self):
        inventory = self.inventory()
        raw = b'line\r\n\x00\xfftail'
        inventory.record('source/data.bin', raw)
        report = inventory.report()
        self.assertEqual(report['files'], [{
            'path': 'source/data.bin', 'read_count': 1,
            'changed_between_reads': False,
            'snapshots': [{'sha256': hashlib.sha256(raw).hexdigest(),
                           'bytes': len(raw), 'read_count': 1}],
        }])
        self.assertEqual(report['bytes_read'], len(raw))

    def test_repeat_reads_count_bytes_without_duplicate_membership(self):
        inventory = self.inventory()
        inventory.record('serializer.py', b'pass\n')
        inventory.record('serializer.py', b'pass\n')
        report = inventory.report()
        self.assertEqual(report['unique_paths'], 1)
        self.assertEqual(report['read_events'], 2)
        self.assertEqual(report['bytes_read'], 10)
        self.assertEqual(report['files'][0]['snapshots'][0]['read_count'], 2)
        self.assertFalse(report['files'][0]['changed_between_reads'])

    def test_changed_reads_retain_both_snapshots(self):
        inventory = self.inventory()
        for raw in (b'old', b'new', b'old'):
            inventory.record('changing.rs', raw)
        row = inventory.report()['files'][0]
        self.assertTrue(row['changed_between_reads'])
        self.assertEqual(row['read_count'], 3)
        self.assertEqual({s['sha256']: s['read_count'] for s in row['snapshots']}, {
            hashlib.sha256(b'old').hexdigest(): 2,
            hashlib.sha256(b'new').hexdigest(): 1,
        })

    def test_report_is_stable_sorted_and_cannot_mutate_inventory(self):
        first, second = self.inventory(), self.inventory()
        for path, raw in [('b', b'2'), ('a', b'1')]:
            first.record(path, raw)
        for path, raw in [('a', b'1'), ('b', b'2')]:
            second.record(path, raw)
        self.assertEqual(first.report(), second.report())
        changed = first.report()
        changed['files'][0]['snapshots'][0]['bytes'] = 999
        self.assertEqual(first.report(), second.report())

    def test_real_checker_reports_exact_existing_input_membership(self):
        checker = load('check')
        report = checker.run(REPO)
        facts = report['source_reference_facts']
        expected = {
            checker.SOURCE['path'], checker.SERIALIZER['path'],
            *(f'models/sherlock/oracles/{name}' for name in checker.FROZEN),
            'crates/mori-core/src/sherlock/normal_oracles.rs',
            'crates/mori-core/src/sherlock/extra_normal_oracles.rs',
        }
        self.assertEqual([row['path'] for row in facts['files']], sorted(expected))
        self.assertEqual(facts['unique_paths'], 8)
        self.assertEqual(facts['read_events'], 9)
        total = 0
        for row in facts['files']:
            raw = (REPO / row['path']).read_bytes()
            count = 2 if row['path'] == checker.SERIALIZER['path'] else 1
            self.assertEqual(row['read_count'], count)
            self.assertFalse(row['changed_between_reads'])
            self.assertEqual(row['snapshots'], [{
                'sha256': hashlib.sha256(raw).hexdigest(),
                'bytes': len(raw), 'read_count': count,
            }])
            total += count * len(raw)
        self.assertEqual(facts['bytes_read'], total)
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(report['acceptance'], 'none')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))


if __name__ == '__main__':
    unittest.main()
