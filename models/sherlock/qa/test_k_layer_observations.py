"""Supplied K comparator fixtures; no example in this file ran the K engine."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import check as qa
import host_observations as host

REPO = Path(__file__).resolve().parents[3]


def fixture():
    families = []
    for family in ('normal', 'extra-normal'):
        families.append(tuple(json.loads((REPO / 'models/sherlock/oracles' /
                                         (family + '-' + kind + '.json')).read_text())
                              for kind in ('programs', 'prefixes')))
    observed = json.loads(json.dumps(
        host.expected_observations(families, producer='sherlock-k-reference')))
    raw = []
    # Retain the actual nested decoder envelope shape. These strings are
    # deliberately labeled fixtures, not serialized observations from K.
    for run in observed['runs']:
        case, fuel = run['case_id'], run['initial']['initial_fuel']
        captures = [dict(configuration=f'FIXTURE {case}/{fuel}/state/{i}',
                         incoming_log=f'FIXTURE {case}/{fuel}/log/{i}')
                    for i in range(len(run['records']) + 1)]
        raw.append(dict(observation=run, raw_captures=captures,
                        decoded_incoming_logs=[dict(raw_log=row['incoming_log'])
                                               for row in captures[1:]],
                        raw_startup_probe=dict(log=f'FIXTURE {case}/{fuel}/startup'),
                        raw_terminal_probe=dict(log=f'FIXTURE {case}/{fuel}/terminal'),
                        comparison_ready=True, gaps=[]))
    # Decouple diagnostic declarations from the projected observation fixture.
    raw = json.loads(json.dumps(raw))
    return dict(format='sherlock-k-normal-observations/1', observations=observed,
                raw_runs=raw, comparison_ready=True,
                projection=dict(boundary='post-startup', gaps=[],
                                startup='separate raw startup probes when supplied',
                                rule_identity='fixture declarations only'),
                evidence=dict(execution_receipt='not-supplied',
                              producer_authenticity='not-established',
                              adapter_execution='supplied-capture-text-only'))


class KLayerObservationTests(unittest.TestCase):
    def supplied(self, artifact):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / 'supplied-k.json'
            p.write_text(json.dumps(artifact))
            try:
                return qa.run(REPO, k_observations=p)
            except TypeError as error:
                self.fail('Supplied K comparison could not run: ' + str(error))

    def test_complete_supplied_projection_preserves_nonexecution_scope(self):
        artifact = fixture()
        report = self.supplied(artifact)
        result = report['supplied_k_observation']
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(report['acceptance'], 'none')
        self.assertEqual(result['status'], 'match')
        self.assertEqual(result['expected_programs'], 8)
        self.assertEqual(result['expected_prefix_runs'], 216)
        self.assertEqual(result['producer_authenticity'], 'not-established')
        self.assertEqual(result['execution_receipt'], 'not-supplied')
        self.assertEqual(result['artifact_metadata']['evidence'], artifact['evidence'])
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))

    def test_record_difference_selects_exact_nested_case_and_fuel_log(self):
        artifact = fixture()
        artifact['observations']['runs'][1]['records'][0]['after']['control_k'] = 'different control'
        report = self.supplied(artifact)
        delta = report['first_difference']
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(delta['path'], '/runs/1/records/0/after/control_k')
        self.assertEqual(delta['case_id'], 'N01')
        self.assertEqual(delta['initial_fuel'], 1)
        context = delta['raw_context']
        self.assertEqual(context['raw_run_index'], 1)
        self.assertEqual(context['raw_run'], artifact['raw_runs'][1])
        self.assertEqual(context['raw_before_capture'], artifact['raw_runs'][1]['raw_captures'][0])
        self.assertEqual(context['raw_after_capture'], artifact['raw_runs'][1]['raw_captures'][1])
        self.assertEqual(context['incoming_rule_log'], artifact['raw_runs'][1]['decoded_incoming_logs'][0])

    def test_terminal_difference_retains_raw_terminal_and_startup_probes(self):
        artifact = fixture()
        artifact['observations']['runs'][214]['terminal']['remaining_fuel'] = 9
        delta = self.supplied(artifact)['first_difference']
        context = delta['raw_context']
        self.assertEqual(context['raw_run_index'], 214)
        self.assertEqual(context['raw_capture'], artifact['raw_runs'][214]['raw_captures'][-1])
        self.assertEqual(context['raw_terminal_probe'], artifact['raw_runs'][214]['raw_terminal_probe'])
        self.assertEqual(context['raw_startup_probe'], artifact['raw_runs'][214]['raw_startup_probe'])

    def test_isolated_cli_selects_k_mode_without_claiming_execution(self):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / 'supplied-k.json'
            p.write_text(json.dumps(fixture()))
            result = subprocess.run([sys.executable, '-I', '-B', str(Path(qa.__file__)),
                                     '--k-observations', str(p)],
                                    capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report['supplied_k_observation']['status'], 'match')
        self.assertEqual(report['acceptance'], 'none')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))


class KLayerHostileTests(unittest.TestCase):
    """Security-owner hostile cases; synthetic supplied text, never K output."""

    @classmethod
    def setUpClass(cls):
        cls.text = json.dumps(fixture())

    def fresh(self):
        return json.loads(self.text)

    def supplied(self, artifact):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / 'supplied-k.json'
            p.write_text(json.dumps(artifact))
            return qa.run(REPO, k_observations=p)

    def cli(self, *flags, raw=None):
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / 'supplied-k.json'
            p.write_text(self.text if raw is None else raw)
            argv = [str(p) if flag == '@k' else flag for flag in flags]
            return subprocess.run([sys.executable, '-I', '-B', str(Path(qa.__file__)), *argv],
                                  capture_output=True, text=True, check=False)

    def test_non_int_fuel_never_selects_raw_run(self):
        for index, fuel, decoy in ((0, 0, False), (0, 0, 0.0), (1, 1, True), (1, 1, 1.0)):
            with self.subTest(fuel=fuel, decoy=decoy):
                artifact = self.fresh()
                artifact['raw_runs'][index]['observation']['initial']['initial_fuel'] = decoy
                context = qa.layer_raw_context(artifact, dict(case_id='N01', initial_fuel=fuel,
                                                              path=f'/runs/{index}/terminal/control_k'))
                self.assertEqual(context['raw_context_status'], 'unavailable')
                self.assertEqual(context['raw_run_candidate_indices'], [])
                self.assertNotIn('raw_run', context)

    def test_non_int_decoy_does_not_create_multiple(self):
        artifact = self.fresh()
        decoy = json.loads(json.dumps(artifact['raw_runs'][1]))
        decoy['observation']['initial']['initial_fuel'] = True
        artifact['raw_runs'].append(decoy)
        context = qa.layer_raw_context(artifact, dict(case_id='N01', initial_fuel=1,
                                                      path='/runs/1/terminal/control_k'))
        self.assertEqual(context['raw_context_status'], 'available')
        self.assertEqual(context['raw_run_index'], 1)

    def test_exact_duplicate_raw_runs_are_multiple(self):
        artifact = self.fresh()
        artifact['raw_runs'].append(artifact['raw_runs'][214])
        context = qa.layer_raw_context(artifact, dict(case_id='N08', initial_fuel=82,
                                                      path='/runs/214/terminal/control_k'))
        self.assertEqual(context['raw_context_status'], 'multiple')
        self.assertEqual(context['raw_run_candidate_indices'], [214, 216])
        self.assertNotIn('raw_run', context)

    def test_absent_raw_run_is_unavailable(self):
        artifact = self.fresh()
        artifact['raw_runs'].pop(214)
        context = qa.layer_raw_context(artifact, dict(case_id='N08', initial_fuel=82,
                                                      path='/runs/214/terminal/control_k'))
        self.assertEqual(context['raw_context_status'], 'unavailable')
        self.assertNotIn('raw_run', context)

    def test_hostile_raw_shapes_and_record_indices_fail_closed(self):
        difference = dict(case_id='N01', initial_fuel=1, path='/runs/1/terminal/control_k')
        artifact = self.fresh()
        artifact['raw_runs'] = {'N01': artifact['raw_runs'][1]}
        self.assertEqual(qa.layer_raw_context(artifact, difference)['raw_context_status'], 'unavailable')
        artifact = self.fresh()
        artifact['raw_runs'][1]['observation'] = 'N01'
        self.assertEqual(qa.layer_raw_context(artifact, difference)['raw_context_status'], 'unavailable')
        for record in (True, -1, 5):
            with self.subTest(record_index=record):
                context = qa.layer_raw_context(self.fresh(), dict(
                    case_id='N01', initial_fuel=1, record_index=record,
                    path='/runs/1/records/0/after/control_k'))
                self.assertEqual(context['raw_context_status'], 'available')
                self.assertEqual(context['raw_state_status'], 'unavailable')
                self.assertNotIn('raw_before_capture', context)

    def test_record_context_states_missing_incoming_log(self):
        artifact = self.fresh()
        del artifact['raw_runs'][1]['decoded_incoming_logs']
        context = qa.layer_raw_context(artifact, dict(case_id='N01', initial_fuel=1, record_index=0,
                                                      path='/runs/1/records/0/after/control_k'))
        self.assertEqual(context['raw_before_capture'], artifact['raw_runs'][1]['raw_captures'][0])
        self.assertEqual(context['incoming_rule_log_status'], 'unavailable')
        self.assertNotIn('incoming_rule_log', context)

    def test_self_declared_receipt_and_readiness_never_upgrade_scope(self):
        artifact = self.fresh()
        artifact['evidence'] = dict(execution_receipt='verified', producer_authenticity='established',
                                    adapter_execution='krun')
        artifact['comparison_ready'] = True
        artifact['acceptance'] = 'accepted'
        report = self.supplied(artifact)
        result = report['supplied_k_observation']
        self.assertEqual(report['acceptance'], 'none')
        self.assertEqual(result['status'], 'match')
        self.assertEqual(result['producer_authenticity'], 'not-established')
        self.assertEqual(result['execution_receipt'], 'not-supplied')
        self.assertEqual(result['constructor_startup'], 'unsupported')
        self.assertEqual(result['artifact_metadata']['evidence'], artifact['evidence'])
        self.assertEqual(result['artifact_metadata']['acceptance'], 'accepted')
        self.assertEqual(report['supplied_comparison_gaps']['K']['status'], 'supplied-comparison-only')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))

    def test_match_is_projection_only_without_raw_runs(self):
        # Pins scope: a match concerns the supplied projection, not raw K cells.
        artifact = self.fresh()
        artifact['raw_runs'] = []
        artifact['comparison_ready'] = False
        artifact['projection']['gaps'] = ['supplied-declared-gap']
        report = self.supplied(artifact)
        result = report['supplied_k_observation']
        self.assertEqual(report['acceptance'], 'none')
        self.assertEqual(result['status'], 'match')
        self.assertEqual(result['raw_diagnostic_counts'], {'raw_runs': 0})
        self.assertIs(result['artifact_metadata']['comparison_ready'], False)
        self.assertEqual(result['projection']['gaps'], ['supplied-declared-gap'])

    def test_envelope_members_and_format_are_required(self):
        for name in ('observations', 'projection', 'raw_runs', 'evidence', 'comparison_ready'):
            with self.subTest(missing=name):
                artifact = self.fresh()
                del artifact[name]
                with self.assertRaisesRegex(ValueError, 'k artifact: missing ' + name):
                    self.supplied(artifact)
        artifact = self.fresh()
        artifact['format'] = 'sherlock-quint-itf-observations/1'
        with self.assertRaisesRegex(ValueError, 'k artifact: format mismatch'):
            self.supplied(artifact)
        artifact = self.fresh()
        artifact['projection'] = []
        with self.assertRaisesRegex(ValueError, 'k artifact: projection must be object'):
            self.supplied(artifact)

    def test_foreign_producer_is_a_mismatch(self):
        artifact = self.fresh()
        artifact['observations']['producer'] = 'sherlock-host-reference'
        report = self.supplied(artifact)
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['first_difference']['path'], '/producer')
        self.assertEqual(report['first_difference']['raw_context']['raw_context_status'], 'unavailable')

    def test_strict_json_rejections_exit_two(self):
        duplicate = self.text[:-1] + ', "format": "sherlock-k-normal-observations/1"}'
        nonfinite = json.dumps(dict(self.fresh(), comparison_ready=float('nan')))
        for label, raw, message in (('duplicate', duplicate, 'duplicate JSON member'),
                                    ('nonfinite', nonfinite, 'nonfinite JSON number')):
            with self.subTest(label=label):
                result = self.cli('--k-observations', '@k', raw=raw)
                self.assertEqual(result.returncode, 2)
                self.assertIn(message, result.stderr)
                self.assertEqual(result.stdout, '')

    def test_k_mode_excludes_other_modes(self):
        for other in ('--lean-observations', '--host-observations'):
            with self.subTest(other=other):
                result = self.cli('--k-observations', '@k', other, 'unread.json')
                self.assertEqual(result.returncode, 2)
                self.assertIn('not allowed with argument', result.stderr)
                self.assertEqual(result.stdout, '')
        result = self.cli('--k-observations', '@k', '--observed-programs', 'unread-a.json',
                          '--observed-prefixes', 'unread-b.json')
        self.assertEqual(result.returncode, 2)
        self.assertIn('separate comparison modes', result.stderr)


if __name__ == '__main__':
    unittest.main()
