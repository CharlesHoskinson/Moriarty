"""Behavioral checks for the U021/U026 comparison package entrypoint."""
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


REPO = Path(__file__).resolve().parents[3]
CLI = REPO / 'models/sherlock/qa/model_package.py'
ARCHIVE = REPO / 'docs/results/sherlock-k-quint-f03-actual-20261018/actual/outputs'


class ModelPackageTests(unittest.TestCase):
    def run_cli(self, *args):
        result = subprocess.run([sys.executable, str(CLI), *map(str, args)], cwd=REPO,
                                capture_output=True, text=True, timeout=60)
        self.assertIn(result.returncode, (0, 1), result.stderr)
        return result.returncode, json.loads(result.stdout)

    def test_archived_pair_is_compared_afresh_with_complete_projection_and_no_refinement(self):
        code, report = self.run_cli('projection')
        self.assertEqual(code, 0)
        self.assertEqual(report['comparison']['status'], 'match')
        self.assertEqual(report['comparison']['selection']['case_id'], 'F03')
        self.assertEqual(report['comparison']['numeric_bounds']['initial_fuel'], 93)
        self.assertEqual(report['comparison']['numeric_bounds']['observed_logical_frame_peak'],
                         dict(k=11, quint=11))
        contract = report['comparison']['projection_contract']
        self.assertEqual(len(contract['fieldMaps']), 9)
        self.assertTrue(contract['ruleToAction'])
        self.assertTrue(contract['hiddenSteps'])
        self.assertIn('await/resume', contract['unmatchedFinanceWait'])
        self.assertEqual(report['refinement'], 'none')
        self.assertEqual(report['execution'], 'not-run')
        self.assertEqual(report['comparison']['layers']['k']['status'], 'match')
        self.assertEqual(report['comparison']['layers']['quint']['status'], 'match')
        self.assertEqual(report['compared_edges'], dict(k=94, quint=94))
        self.assertTrue(report['oracle_files'])

    def test_supplied_intermediate_environment_mutation_has_first_divergence_despite_equal_result(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = {}
            for layer in ('k', 'quint'):
                raw = gzip.decompress((ARCHIVE / (layer + '-observations.json.gz')).read_bytes())
                value = json.loads(raw)
                if layer == 'quint':
                    value['observations']['runs'][0]['records'][1]['after']['environment_k'] = '0 |-> uiU(99)'
                paths[layer] = Path(tmp) / (layer + '.json')
                paths[layer].write_text(json.dumps(value))
            code, report = self.run_cli('projection', '--k-observations', paths['k'],
                '--quint-observations', paths['quint'], '--case', 'F03', '--fuel', '93')
        self.assertEqual(code, 1)
        difference = report['comparison']['paired']['first_difference']
        self.assertEqual(difference['path'], '/runs/0/records/1/after/environment_k')
        self.assertEqual(report['comparison']['layers']['k']['status'], 'match')
        self.assertEqual(report['comparison']['layers']['quint']['status'], 'mismatch')

    def test_partial_selection_is_a_json_input_error(self):
        result = subprocess.run([sys.executable, str(CLI), 'projection', '--fuel', '93'],
                                cwd=REPO, capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 2)
        self.assertTrue(result.stdout, result.stderr)
        self.assertEqual(json.loads(result.stdout)['status'], 'input-error')

    def test_supplied_provenance_and_mismatching_values_do_not_leak_private_content(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = {}
            sentinel = '/private/integration-sentinel/private-code-sentinel.lean'
            for layer in ('k', 'quint'):
                value = json.loads(gzip.decompress((ARCHIVE / (layer + '-observations.json.gz')).read_bytes()))
                value['evidence']['private_path'] = sentinel
                value['projection']['private_source'] = sentinel
                if layer == 'quint':
                    value['observations']['runs'][0]['records'][1]['after']['environment_k'] = sentinel
                paths[layer] = Path(tmp) / (layer + '.json')
                paths[layer].write_text(json.dumps(value))
            code, report = self.run_cli('projection', '--k-observations', paths['k'],
                '--quint-observations', paths['quint'], '--case', 'F03', '--fuel', '93')
        self.assertEqual(code, 1)
        self.assertFalse(sentinel in json.dumps(report), 'private content leaked in projection')
        difference = report['comparison']['paired']['first_difference']
        self.assertEqual(difference['path'], '/runs/0/records/1/after/environment_k')
        self.assertEqual(len(difference['observed_sha256']), 64)

    def test_stage_does_not_publish_arbitrary_supplied_comparison_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            _, projection = self.run_cli('projection')
            sentinel = '/private/integration-sentinel/private-source-sentinel.lean'
            projection['oracle_files'].append(dict(path=sentinel, private_code=sentinel))
            projection['comparison']['numeric_bounds']['private_path'] = sentinel
            path = Path(tmp) / 'comparison.json'
            path.write_text(json.dumps(projection))
            _, report = self.run_cli('stage', '--comparison', path)
        self.assertFalse(sentinel in json.dumps(report), 'private content leaked in stage')

    def test_successful_supplied_comparison_keeps_provenance_private(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = {}
            sentinel = '/private/success-provenance-sentinel.lean'
            for layer in ('k', 'quint'):
                value = json.loads(gzip.decompress((ARCHIVE / (layer + '-observations.json.gz')).read_bytes()))
                value['evidence']['private_path'] = sentinel
                paths[layer] = Path(tmp) / (layer + '.json')
                paths[layer].write_text(json.dumps(value))
            code, report = self.run_cli('projection', '--k-observations', paths['k'],
                '--quint-observations', paths['quint'], '--case', 'F03', '--fuel', '93')
        self.assertEqual(code, 0)
        self.assertFalse(sentinel in json.dumps(report), 'private provenance leaked on match')

    def test_truncated_gzip_is_a_json_input_error(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'broken.json.gz'
            path.write_bytes(gzip.compress(b'{}')[:10])
            result = subprocess.run([sys.executable, str(CLI), 'stage', '--comparison', str(path)],
                cwd=REPO, capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertTrue(result.stdout, result.stderr)
        self.assertEqual(json.loads(result.stdout)['status'], 'input-error')

    def test_malformed_stage_inputs_are_json_errors(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'malformed.json'
            for value in ([], None, 'not-an-object', {'format': 'sherlock-model-projection/1', 'comparison': []}):
                path.write_text(json.dumps(value))
                for flag in ('--private-record', '--comparison'):
                    with self.subTest(value=value, flag=flag):
                        result = subprocess.run([sys.executable, str(CLI), 'stage', flag, str(path)],
                            cwd=REPO, capture_output=True, text=True, timeout=10)
                        self.assertEqual(result.returncode, 2, result.stderr)
                        self.assertTrue(result.stdout, result.stderr)
                        self.assertEqual(json.loads(result.stdout)['status'], 'input-error')

    def test_stage_lists_separate_required_tasks_and_does_not_promote_historical_checks(self):
        code, report = self.run_cli('stage')
        self.assertEqual(code, 0)
        tasks = {row['id']: row for row in report['tasks']}
        self.assertIn('recursive-lean-semantics', tasks)
        self.assertIn('fuel-step-run', tasks)
        self.assertIn('bounded-clean-encoding', tasks)
        self.assertIn('bounded-clean-step-circuit', tasks)
        self.assertEqual(tasks['bounded-clean-step-circuit']['status'], 'not-implemented')
        self.assertEqual(report['historical_lean']['normal_cases'], 8)
        self.assertEqual(report['historical_lean']['zero_remaining_publications'], 1)
        self.assertEqual(report['historical_lean']['current_execution'], 'not-run')
        self.assertEqual(len(report['public_reuse']['files']), 6)
        self.assertEqual(report['public_reuse']['commit'], '1a6c28426372bb99af79e533aa7b9a0788eff0d7')
        self.assertEqual(report['midnight_exporter'], 'not-implemented')
        self.assertEqual(report['execution_authority'], 'none')
        self.assertEqual(report['acceptance'], 'none')
        self.assertEqual(report['private_integration']['status'], 'not-supplied')

    def test_stage_comparison_names_every_layer_and_independent_complete_prefix_expectations(self):
        _, report = self.run_cli('stage')
        comparison = report['correctness_loop']
        self.assertEqual({row['layer'] for row in comparison['layers']},
            {'K', 'Quint', 'Lean', 'Clean', 'handwritten-circuit', 'native-ledger'})
        self.assertEqual(comparison['first_divergence'], None)
        self.assertEqual(comparison['comparison_status'], 'not-run')
        self.assertEqual(len(comparison['complete_state_fields']), 11)
        self.assertTrue(comparison['independent_expectations'])
        self.assertTrue(comparison['projections']['K/Quint']['ruleToAction'])
        self.assertTrue(all(row['source_membership'] is not None for row in comparison['layers']))
        self.assertTrue(report['unproved_gaps'])

    def test_stage_recompares_retained_pair_without_external_inputs(self):
        code, report = self.run_cli('stage', '--retained-comparison')
        self.assertEqual(code, 0)
        loop = report['correctness_loop']
        self.assertEqual(loop['comparison_status'], 'match')
        self.assertEqual(loop['first_divergence'], None)
        self.assertEqual(loop['selection'], dict(case_id='F03', initial_fuel=93))
        self.assertEqual(loop['execution'], 'compared-locally; producers-not-rerun')
        self.assertIn('retained', loop['first_divergence_disposition'])
        self.assertEqual(loop['comparison_input']['origin'], 'retained-actual-F03')
        self.assertEqual({row['layer']: row.get('comparison_status') for row in loop['layers'][:2]},
                         dict(K='match', Quint='match'))
        self.assertEqual(report['status'], 'package-ready')

    def test_stage_refuses_both_retained_and_supplied_comparisons(self):
        result = subprocess.run([sys.executable, str(CLI), 'stage', '--retained-comparison',
                                 '--comparison', str(ARCHIVE / 'k-observations.json.gz')],
                                cwd=REPO, capture_output=True, text=True, timeout=60)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(json.loads(result.stdout)['status'], 'input-error')

    def test_supplied_mismatch_propagates_to_stage_package(self):
        with tempfile.TemporaryDirectory() as tmp:
            _, projection = self.run_cli('projection')
            # A supplied comparison is a declaration, never authenticated execution.
            projection['comparison']['status'] = 'mismatch'
            projection['comparison']['paired']['first_difference'] = dict(
                path='/runs/0/terminal/output_k', expected='uiDone(uiU(6))', observed='uiDone(uiU(9))')
            path = Path(tmp) / 'comparison.json'
            path.write_text(json.dumps(projection))
            code, report = self.run_cli('stage', '--comparison', path)
        self.assertEqual(code, 1)
        self.assertEqual(report['correctness_loop']['comparison_status'], 'mismatch')
        self.assertEqual(report['correctness_loop']['first_divergence']['path'],
                         '/runs/0/terminal/output_k')
        self.assertEqual(report['correctness_loop']['execution'], 'not-established-by-supplied-report')

    def test_stage_conservatively_propagates_a_divergence_despite_a_claimed_aggregate_match(self):
        with tempfile.TemporaryDirectory() as tmp:
            _, original = self.run_cli('projection')
            path = Path(tmp) / 'comparison.json'
            for location in ('paired', 'quint'):
                with self.subTest(location=location):
                    projection = json.loads(json.dumps(original))
                    comparison = projection['comparison']
                    row = comparison['paired'] if location == 'paired' else comparison['layers']['quint']
                    row['first_difference'] = dict(path='/runs/0/terminal/output_k',
                        expected='uiDone(uiU(6))', observed='uiDone(uiU(9))')
                    row['status'] = 'mismatch'
                    path.write_text(json.dumps(projection))
                    code, report = self.run_cli('stage', '--comparison', path)
                    self.assertEqual(code, 1)
                    self.assertEqual(report['correctness_loop']['comparison_status'], 'mismatch')

    def test_private_binding_pins_source_bytes_without_disclosing_paths_or_code(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / 'private-integration-sentinel'
            source = root / 'Clean/Sherlock/Semantics.lean'
            source.parent.mkdir(parents=True)
            source.write_text('-- private-code-sentinel\n')
            (root / 'Clean/Sherlock/SemanticsExamples.lean').write_text('-- fixture only\n')
            record = Path(tmp) / 'private-record.json'
            code, binding = self.run_cli('bind-private', '--integration-root', root, '--record', record)
            self.assertEqual(code, 0)
            retained = json.loads(record.read_text())
            self.assertEqual(retained['source_files'][0]['sha256'], hashlib.sha256(source.read_bytes()).hexdigest())
            self.assertEqual(binding['private_integration']['record_sha256'], hashlib.sha256(record.read_bytes()).hexdigest())
            _, stage = self.run_cli('stage', '--private-record', record)
        public = json.dumps([binding, stage])
        self.assertNotIn(str(root), public)
        self.assertNotIn('private-code-sentinel', public)
        self.assertEqual(stage['private_integration']['status'], 'retained-private-source-record')
        self.assertEqual(stage['private_integration']['proof_membership'], 'not-established')

    def test_private_binding_refuses_a_tracked_public_destination(self):
        result = subprocess.run([sys.executable, str(CLI), 'bind-private',
            '--integration-root', str(REPO), '--record', str(REPO / 'docs/private-binding.json')],
            cwd=REPO, capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 2)
        self.assertTrue(result.stdout, result.stderr)
        self.assertEqual(json.loads(result.stdout)['status'], 'input-error')
        self.assertFalse((REPO / 'docs/private-binding.json').exists())


if __name__ == '__main__':
    unittest.main()
