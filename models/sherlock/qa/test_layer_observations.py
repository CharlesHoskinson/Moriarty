"""Ordinary supplied-artifact integration; fixture data is not executed evidence."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import check as qa
import host_observations as host

REPO = Path(__file__).resolve().parents[3]
LAYERS = {
    'lean': ('sherlock-lean-normal-observations/1', 'sherlock-lean-reference'),
    'quint': ('sherlock-quint-itf-observations/1', 'sherlock-quint-k-profile'),
    'image': ('sherlock-image-observations/1', 'sherlock-image-reference'),
}


def documents():
    return [(json.loads((REPO / 'models/sherlock/oracles' / (family + '-programs.json')).read_text()),
             json.loads((REPO / 'models/sherlock/oracles' / (family + '-prefixes.json')).read_text()))
            for family in ('normal', 'extra-normal')]


def quint_trace_fixture(case_id, fuel, *, stutter=False, namespace=''):
    """Small raw diagnostic fixture, never a claimed Quint execution."""
    states = []
    for index in range(3 + int(stutter)):
        published = index >= 2
        state = {'#meta': {'index': index},
                 namespace + 'machine': {
                     'initial': {'#bigint': str(fuel)},
                     'outcome': {'tag': 'Done' if published else 'Pending',
                                 'value': {'#tup': []}},
                     'control': {'tag': 'EmptyControl' if published else 'Eval',
                                 'value': {'#bigint': '0'}}},
                 namespace + 'lastRule': {'#bigint': str(0 if index == 3 else 80 if published else 74 if index == 0 else 102)},
                 namespace + 'lastCharge': {'#bigint': str(int(index == 1))}}
        states.append(state)
    return dict(case_id=case_id, itf={'states': states})


def supplied_fixture(layer):
    """Copy frozen expected literals solely as a labeled comparator test fixture."""
    fmt, producer = LAYERS[layer]
    observations = host.expected_observations(documents())
    observations['producer'] = producer
    projection = dict(terminal_has_next='inferred-test-fixture-only',
                      rule_start_line='inferred-test-fixture-only')
    value = dict(format=fmt, observations=observations, projection=projection)
    raw_run = dict(case_id='N01', initial_fuel=1, initial='raw initial',
                   records=[dict(before='raw before', after='raw actual terminal control')],
                   terminal='raw actual terminal control')
    if layer == 'lean':
        value['raw_runs'] = [dict(case_id='N01', initial_fuel=0), raw_run]
    elif layer == 'quint':
        value.update(raw_traces=[quint_trace_fixture(run['case_id'], run['initial']['initial_fuel'])
                                 for run in observations['runs']],
                     raw_trace_references=[dict(case_id=run['case_id'], canonical_itf_sha256='test-fixture-' + str(index))
                                           for index, run in enumerate(observations['runs'])],
                     evidence=dict(execution_receipt='not-supplied'))
    else:
        projection.update(format='sherlock-host-flat-id-diagnostics/1',
                          program_images=[dict(case_id='N01', image={'nodes': ['raw node']})],
                          runs=[dict(case_id='N01', initial_fuel=0), raw_run])
    return value


class LayerObservationTests(unittest.TestCase):
    def run_supplied(self, layer, artifact):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'supplied.json'
            path.write_text(json.dumps(artifact))
            return qa.run(REPO, **{layer + '_observations': path})

    def test_expected_producer_parameter_preserves_complete_frozen_shape(self):
        baseline = host.expected_observations(documents())
        for _, producer in LAYERS.values():
            with self.subTest(producer=producer):
                expected = host.expected_observations(documents(), producer=producer)
                self.assertEqual(expected['producer'], producer)
                expected['producer'] = host.PRODUCER
                self.assertEqual(expected, baseline)

    def test_all_concrete_layers_compare_complete_literals_without_execution_claim(self):
        for layer in LAYERS:
            with self.subTest(layer=layer):
                artifact = supplied_fixture(layer)
                report = self.run_supplied(layer, artifact)
                supplied = report['supplied_' + layer + '_observation']
                self.assertEqual(report['status'], 'pass')
                self.assertEqual(supplied['status'], 'match')
                self.assertEqual(supplied['producer_authenticity'], 'not-established')
                self.assertEqual(supplied['execution_receipt'], 'not-supplied')
                self.assertEqual(supplied['constructor_startup'], 'unsupported')
                self.assertEqual(supplied['expected_programs'], 8)
                self.assertEqual(supplied['expected_prefix_runs'], 216)
                self.assertEqual(supplied['projection'], artifact['projection'])
                self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))

    def test_lean_mismatch_keeps_raw_run_and_inference_metadata(self):
        artifact = supplied_fixture('lean')
        artifact['observations']['runs'][1]['records'][0]['after']['control_k'] = 'different control'
        report = self.run_supplied('lean', artifact)
        delta = report['first_difference']
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(delta['path'], '/runs/1/records/0/after/control_k')
        self.assertEqual(delta['case_id'], 'N01')
        self.assertEqual(delta['initial_fuel'], 1)
        self.assertEqual(delta['observed_record']['after']['control_k'], 'different control')
        self.assertEqual(delta['raw_context']['raw_run'], artifact['raw_runs'][1])
        self.assertEqual(delta['projection'], artifact['projection'])

    def test_quint_mismatch_keeps_itf_and_reference(self):
        artifact = supplied_fixture('quint')
        artifact['observations']['runs'][1]['terminal']['remaining_fuel'] = 9
        delta = self.run_supplied('quint', artifact)['first_difference']
        self.assertEqual(delta['raw_context']['raw_trace'], artifact['raw_traces'][1])
        self.assertEqual(delta['raw_context']['raw_trace_reference'], artifact['raw_trace_references'][1])
        self.assertEqual(delta['observed_state']['remaining_fuel'], 9)

    def test_quint_n08_fuel82_selects_raw214_and_exact_record_boundaries(self):
        artifact = supplied_fixture('quint')
        difference = dict(case_id='N08', initial_fuel=82, record_index=1,
                          path='/runs/214/records/1/after/control_k')
        context = qa.layer_raw_context(artifact, difference)
        self.assertEqual(context.get('raw_trace'), artifact['raw_traces'][214])
        self.assertEqual(context.get('raw_trace_index'), 214)
        self.assertEqual(context.get('raw_trace_reference'), artifact['raw_trace_references'][214])
        states = artifact['raw_traces'][214]['itf']['states']
        self.assertEqual(context.get('raw_before_state'), states[1])
        self.assertEqual(context.get('raw_after_state'), states[2])
        self.assertEqual(context.get('raw_state_indices'), dict(before=1, after=2))

    def test_quint_initial_and_terminal_use_publication_before_stutter(self):
        artifact = supplied_fixture('quint')
        artifact['raw_traces'][2] = quint_trace_fixture('N01', 2, stutter=True, namespace='normal::')
        states = artifact['raw_traces'][2]['itf']['states']
        for field, index in (('initial', 0), ('terminal', 2)):
            difference = dict(case_id='N01', initial_fuel=2, path=f'/runs/2/{field}/control_k')
            context = qa.layer_raw_context(artifact, difference)
            self.assertEqual(context.get('raw_trace_index'), 2)
            self.assertEqual(context.get('raw_state'), states[index])
            self.assertEqual(context.get('raw_state_indices'), {field: index})

    def test_quint_context_is_explicit_when_selected_capture_is_unavailable(self):
        artifact = supplied_fixture('quint')
        artifact['raw_traces'].pop(214)
        context = qa.layer_raw_context(artifact, dict(case_id='N08', initial_fuel=82,
                                                    path='/runs/214/terminal/control_k'))
        self.assertEqual(context.get('raw_context_status'), 'unavailable')
        self.assertNotIn('raw_trace', context)
        self.assertNotIn('raw_trace_reference', context)

    def test_quint_context_is_explicit_for_multiple_selected_captures(self):
        artifact = supplied_fixture('quint')
        artifact['raw_traces'].append(artifact['raw_traces'][214])
        context = qa.layer_raw_context(artifact, dict(case_id='N08', initial_fuel=82,
                                                    path='/runs/214/terminal/control_k'))
        self.assertEqual(context.get('raw_context_status'), 'multiple')
        self.assertEqual(context.get('raw_trace_candidate_indices'), [214, 216])
        self.assertNotIn('raw_trace', context)

    def test_quint_reference_uses_selected_index_and_same_case(self):
        artifact = supplied_fixture('quint')
        artifact['raw_trace_references'][214] = dict(case_id='N07', canonical_itf_sha256='other normal case')
        context = qa.layer_raw_context(artifact, dict(case_id='N08', initial_fuel=82,
                                                    path='/runs/214/terminal/control_k'))
        self.assertEqual(context.get('raw_trace_index'), 214)
        self.assertEqual(context.get('raw_trace_reference_status'), 'unavailable')
        self.assertNotIn('raw_trace_reference', context)

    def test_image_mismatch_keeps_flat_ids_and_program_tables(self):
        artifact = supplied_fixture('image')
        artifact['observations']['runs'][1]['records'][0]['charge'] = 0
        delta = self.run_supplied('image', artifact)['first_difference']
        self.assertEqual(delta['raw_context']['raw_run'], artifact['projection']['runs'][1])
        self.assertEqual(delta['raw_context']['program_image'], artifact['projection']['program_images'][0])

    def test_ordinary_cli_flags_read_supplied_files_under_isolated_python(self):
        for layer in LAYERS:
            with self.subTest(layer=layer), tempfile.TemporaryDirectory() as tmp:
                path = Path(tmp) / 'supplied.json'
                path.write_text(json.dumps(supplied_fixture(layer)))
                result = subprocess.run([sys.executable, '-I', '-B', str(Path(qa.__file__)),
                                         '--' + layer + '-observations', str(path)],
                                        capture_output=True, text=True, check=False)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(json.loads(result.stdout)['supplied_' + layer + '_observation']['status'], 'match')

    def test_cli_mismatch_exits_one_with_full_diagnostics(self):
        for layer in LAYERS:
            with self.subTest(layer=layer), tempfile.TemporaryDirectory() as tmp:
                artifact = supplied_fixture(layer)
                artifact['observations']['runs'][1]['records'][0]['charge'] = 0
                path = Path(tmp) / 'supplied.json'
                path.write_text(json.dumps(artifact))
                result = subprocess.run([sys.executable, '-I', '-B', str(Path(qa.__file__)),
                                         '--' + layer + '-observations', str(path)],
                                        capture_output=True, text=True, check=False)
                self.assertEqual(result.returncode, 1, result.stderr)
                delta = json.loads(result.stdout)['first_difference']
                self.assertEqual(delta['rule_start_line'], 102)
                self.assertIn('expected_record', delta)
                self.assertIn('observed_record', delta)
                self.assertIn('raw_context', delta)

    def test_layer_flags_select_separate_comparison_modes(self):
        for flags in (('--host-observations', '--lean-observations'),
                      ('--lean-observations', '--quint-observations'),
                      ('--quint-observations', '--image-observations')):
            with self.subTest(flags=flags):
                result = subprocess.run([sys.executable, '-I', '-B', str(Path(qa.__file__)),
                                         flags[0], 'unread-a.json', flags[1], 'unread-b.json'],
                                        capture_output=True, text=True, check=False)
                self.assertEqual(result.returncode, 2)
                self.assertIn('not allowed with argument', result.stderr)
                self.assertEqual(result.stdout, '')

    def test_k_supplied_comparison_remains_explicitly_unsupported(self):
        report = qa.run(REPO)
        gaps = report['supplied_comparison_gaps']
        self.assertEqual(gaps['K']['status'], 'unsupported')
        self.assertIn('rule', gaps['K']['reason'])


if __name__ == '__main__':
    unittest.main()
