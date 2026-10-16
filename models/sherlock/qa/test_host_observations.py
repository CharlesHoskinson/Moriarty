"""Normal diagnostic-projection tests; fixture-shaped data is not executed evidence."""
import copy
import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest

import check as qa
import host_observations as host

REPO = Path(__file__).resolve().parents[3]


def documents():
    return [(json.loads((REPO / 'models/sherlock/oracles' / (family + '-programs.json')).read_text()),
             json.loads((REPO / 'models/sherlock/oracles' / (family + '-prefixes.json')).read_text()))
            for family in ('normal', 'extra-normal')]


def literal_snapshot(control, fuel, remaining, outcome, work):
    return dict(code_k='uiNoFuns()', entry_k='uiLiteral(uiU(7))', control_k=control,
                environment_k='.Map', frames_top_first_k=[], depth=0, stack_limit=8,
                remaining_fuel=remaining, initial_fuel=fuel, output_k=outcome, charged_steps=work)


class HostObservationTests(unittest.TestCase):
    def expected(self):
        value = host.expected_observations(documents())
        self.assertEqual(len(value.get('programs', [])), 8, 'complete independent input projection missing')
        self.assertEqual(len(value.get('runs', [])), 216, 'complete literal prefix projection missing')
        return value

    def changed(self, mutation):
        expected = self.expected()
        observed = copy.deepcopy(expected)
        mutation(observed)
        delta = host.compare_observations(expected, observed, qa.first_difference)
        self.assertIsNotNone(delta)
        return delta

    def test_n01_zero_and_exact_and_surplus_frozen_projection(self):
        payload = self.expected()
        self.assertEqual(len(payload.get('programs', [])), 8)
        self.assertEqual(len(payload.get('runs', [])), 216)
        zero = literal_snapshot('uiEval(uiLiteral(uiU(7)))', 0, 0, 'uiPending()', 0)
        exhausted = literal_snapshot('.K', 0, 0, 'uiExhausted()', 0)
        self.assertEqual(payload['runs'][0], dict(case_id='N01', initial=zero,
            records=[dict(rule_start_line=84, charge=0, before=zero, after=exhausted)],
            terminal=exhausted, terminal_has_next=False))
        run = payload['runs'][1]
        self.assertEqual([r['rule_start_line'] for r in run['records']], [102, 80])
        self.assertEqual([r['charge'] for r in run['records']], [1, 0])
        self.assertEqual(run['terminal'], literal_snapshot('.K', 1, 0, 'uiDone(uiU(7))', 1))
        self.assertEqual(payload['runs'][2]['terminal'], literal_snapshot('.K', 2, 1, 'uiDone(uiU(7))', 1))

    def test_complete_projection_has_every_prefix_and_publication(self):
        payload = self.expected()
        # N02 follows three N01 rows; its fuel7 row is run index10.
        self.assertEqual(payload['runs'][10]['case_id'], 'N02')
        self.assertEqual(len(payload['runs'][10]['records']), 8)
        self.assertEqual(payload['runs'][10]['records'][-1]['charge'], 0)
        self.assertEqual(payload['runs'][-1]['case_id'], 'N08')
        self.assertEqual(payload['runs'][-1]['terminal']['output_k'], 'uiDone(uiU(2))')
        self.assertIsNone(host.compare_observations(payload, copy.deepcopy(payload), qa.first_difference))

    def test_all_program_identities_checked_before_any_run(self):
        def mutation(value):
            value['programs'][-1]['entry_k'] = 'wrong'
            value['runs'][0]['terminal']['remaining_fuel'] = 7
        delta = self.changed(mutation)
        self.assertEqual(delta['path'], '/programs/7/entry_k')

    def test_missing_prefix_never_matches(self):
        delta = self.changed(lambda value: value['runs'].pop())
        self.assertEqual(delta['path'], '/runs')
        self.assertEqual(delta['kind'], 'array-length')

    def test_full_before_after_field_and_context(self):
        delta = self.changed(lambda value: value['runs'][1]['records'][0]['after'].pop('environment_k'))
        self.assertIn('/records/0/after/environment_k', delta['path'])
        self.assertEqual(delta.get('case_id'), 'N01')
        self.assertEqual(delta.get('initial_fuel'), 1)
        self.assertEqual(delta.get('rule_start_line'), 102)
        self.assertFalse(delta['observed_present'])
        self.assertIn('expected_record', delta)
        self.assertIn('observed_record', delta)

    def test_record_order_and_work_and_charge(self):
        self.changed(lambda value: value['runs'][1]['records'].reverse())
        self.changed(lambda value: value['runs'][1]['records'][0].update(charge=0))
        self.changed(lambda value: value['runs'][1]['records'][0]['after'].update(charged_steps=0))

    def test_outcome_and_publication_not_projected_away(self):
        self.changed(lambda value: value['runs'][1]['records'].pop())
        self.changed(lambda value: value['runs'][1]['terminal'].update(output_k='uiExhausted()'))
        self.changed(lambda value: value['runs'][1].update(terminal_has_next=True))

    def test_saved_frames_and_whole_program_retained(self):
        self.changed(lambda value: value['runs'][10]['records'][1]['after']['frames_top_first_k'].__setitem__(0, 'wrong saved environment'))
        self.changed(lambda value: value['runs'][10]['records'][0]['before'].update(code_k='wrong ordered functions'))

    def test_profile_and_source_declarations_compared(self):
        self.assertEqual(self.changed(lambda value: value.update(profile='other'))['path'], '/profile')
        self.assertEqual(self.changed(lambda value: value.update(source_sha256='0' * 64))['path'], '/source_sha256')

    def test_supplied_host_artifact_is_not_an_execution_claim(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'supplied.json'
            path.write_text(json.dumps(self.expected()))
            result = subprocess.run([sys.executable, str(REPO / 'models/sherlock/qa/check.py'),
                                     '--host-observations', str(path)],
                                    capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        supplied = report.get('supplied_host_observation', {})
        self.assertEqual(supplied.get('status'), 'match')
        self.assertEqual(supplied.get('execution_receipt'), 'not-supplied')
        self.assertEqual(supplied.get('producer_authenticity'), 'not-established')
        self.assertEqual(supplied.get('constructor_startup'), 'unsupported')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))

    def test_host_cli_runs_in_isolated_python_without_sibling_sys_path(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'supplied.json'
            path.write_text(json.dumps(self.expected()))
            result = subprocess.run([sys.executable, '-I', '-B',
                                     str(REPO / 'models/sherlock/qa/check.py'),
                                     '--host-observations', str(path)],
                                    capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['supplied_host_observation']['status'], 'match')

    def test_host_and_corpus_modes_are_rejected_before_any_comparison(self):
        result = subprocess.run([sys.executable, '-I', '-B',
                                 str(REPO / 'models/sherlock/qa/check.py'),
                                 '--host-observations', 'not-read-host.json',
                                 '--observed-programs', 'not-read-programs.json',
                                 '--observed-prefixes', 'not-read-prefixes.json'],
                                capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn('separate comparison modes', result.stderr)
        self.assertEqual(result.stdout, '')

    def test_host_cli_difference_exits_one_and_keeps_raw_record(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'supplied.json'
            payload = self.expected()
            payload['runs'][1]['records'][0]['charge'] = 0
            path.write_text(json.dumps(payload))
            result = subprocess.run([sys.executable, str(REPO / 'models/sherlock/qa/check.py'),
                                     '--host-observations', str(path)],
                                    capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 1, result.stderr)
        delta = json.loads(result.stdout)['first_difference']
        self.assertEqual(delta.get('rule_start_line'), 102)
        self.assertIn('observed_record', delta)

    def test_absent_observation_never_matches(self):
        with self.assertRaisesRegex(ValueError, 'absent'):
            host.compare_observations(self.expected(), None, qa.first_difference)


if __name__ == '__main__':
    unittest.main()
