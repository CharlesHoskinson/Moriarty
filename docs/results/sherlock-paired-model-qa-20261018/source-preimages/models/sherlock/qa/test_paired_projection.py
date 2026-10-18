"""Ordinary supplied-data comparator regressions; no model execution evidence.

SOURCE_ONLY / UNRUN until Root selects this exact source after fresh review.
N01 literals below transcribe the independent normal oracle; they are not
produced by the checker or either semantic machine.
"""
import copy
import json
from pathlib import Path
import tempfile
import unittest

import check as qa

REPO = Path(__file__).resolve().parents[3]


def fixture_family():
    program = dict(case_id='N01', code_k='uiNoFuns()',
                   entry_k='uiLiteral(uiU(7))', input_sha256='tiny-data-fixture-only',
                   stack_limit=8, exact_completion_fuel=1, expected_value_k='uiU(7)')
    states = [dict(charged_steps=0, remaining_fuel=1, depth=0,
                   control_k='uiEval(uiLiteral(uiU(7)))', environment_k='.Map',
                   frames_top_first_k=[], via_rule_start_line=None),
              dict(charged_steps=1, remaining_fuel=0, depth=0,
                   control_k='uiReturn(uiU(7))', environment_k='.Map',
                   frames_top_first_k=[], via_rule_start_line=102)]
    trace = dict(case_id='N01', fixed_state_fields=dict(stack_limit=8, output_k='uiPending()'),
                 states=states, ordinary_fuel_prefixes=[[1, 1, 0, 'uiDone(uiU(7))', 80]])
    return [(dict(profile=qa.PROFILE, source=qa.SOURCE, programs=[program]),
             dict(profile=qa.PROFILE, source=qa.SOURCE, traces=[trace]))]


def supplied_fixture(layer):
    # Full11-field states, with an independently supplied free publication.
    initial = dict(code_k='uiNoFuns()', entry_k='uiLiteral(uiU(7))',
                   control_k='uiEval(uiLiteral(uiU(7)))', environment_k='.Map',
                   frames_top_first_k=[], depth=0, stack_limit=8,
                   remaining_fuel=1, initial_fuel=1, output_k='uiPending()', charged_steps=0)
    returned = dict(initial, control_k='uiReturn(uiU(7))', remaining_fuel=0, charged_steps=1)
    terminal = dict(returned, control_k='.K', output_k='uiDone(uiU(7))')
    run = dict(case_id='N01', initial=initial, records=[
        dict(rule_start_line=102, charge=1, before=copy.deepcopy(initial), after=copy.deepcopy(returned)),
        dict(rule_start_line=80, charge=0, before=copy.deepcopy(returned), after=copy.deepcopy(terminal))],
        terminal=terminal, terminal_has_next=False)
    fmt, producer, _ = qa.LAYER_ARTIFACTS[layer]
    observation = dict(producer=producer, profile=qa.PROFILE, source_sha256=qa.SOURCE['sha256'],
                       boundary='post-startup', programs=[dict(case_id='N01', code_k='uiNoFuns()',
                       entry_k='uiLiteral(uiU(7))')], runs=[run])
    artifact = dict(format=fmt, observations=observation, projection=dict(
                    origin='hand-supplied-comparator-fixture-not-execution'),
                    evidence=dict(execution_receipt='not-supplied', fixture_only=True))
    if layer == 'k':
        artifact.update(raw_runs=[dict(observation=copy.deepcopy(run),
                        raw_captures=['raw supplied initial', 'raw supplied return', 'raw supplied publication'],
                        raw_startup_probe='supplied startup', raw_terminal_probe='supplied terminal probe')],
                        comparison_ready=True)
    else:
        artifact.update(raw_traces=[], raw_trace_references=[])
    return artifact


class PairedProjectionTests(unittest.TestCase):
    def compare(self, k=None, quint=None):
        return qa.compare_paired_observations(
            k if k is not None else supplied_fixture('k'),
            quint if quint is not None else supplied_fixture('quint'), fixture_family(),
            case_id='N01', initial_fuel=1)

    def test_complete_supplied_pair_matches_both_independent_oracle_and_each_other(self):
        report = self.compare()
        self.assertEqual(report['status'], 'match')
        self.assertEqual(report['paired']['status'], 'match')
        self.assertTrue(all(row['status'] == 'match' for row in report['layers'].values()))
        self.assertEqual(report['numeric_bounds'], dict(UInt128_min=0,
            UInt128_max=340282366920938463463374607431768211455, initial_fuel=1,
            stack_limit=8, observed_stack_limit=dict(k=8, quint=8),
            observed_logical_frame_peak=dict(k=0, quint=0)))
        self.assertEqual(report['refinement'], 'none')
        self.assertEqual(report['actual_execution'], 'not-established-by-comparator')
        self.assertEqual(report['selection']['compared_programs'], 1)
        self.assertEqual(report['selection']['compared_runs'], 1)

    def test_intermediate_saved_environment_difference_is_not_hidden_by_equal_terminal(self):
        quint = supplied_fixture('quint')
        quint['observations']['runs'][0]['records'][1]['before']['environment_k'] = '0 |-> uiU(9)'
        report = self.compare(quint=quint)
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['paired']['first_difference']['path'],
                         '/runs/0/records/1/before/environment_k')
        self.assertEqual(report['layers']['quint']['status'], 'mismatch')
        self.assertIn('raw_context', report['layers']['quint']['first_difference'])
        self.assertEqual(report['layers']['k']['status'], 'match')

    def test_agreeing_pair_cannot_replace_independent_literal_expectation(self):
        k, quint = supplied_fixture('k'), supplied_fixture('quint')
        for artifact in (k, quint):
            artifact['observations']['runs'][0]['records'][0]['rule_start_line'] = 103
        report = self.compare(k, quint)
        self.assertEqual(report['paired']['status'], 'match')
        self.assertEqual(report['status'], 'mismatch')
        self.assertTrue(all(row['status'] == 'mismatch' for row in report['layers'].values()))

    def test_all_boundary_fields_rule_charge_and_terminal_enabledness_are_compared(self):
        for member, replacement in [('charge', 0), ('rule_start_line', 103)]:
            quint = supplied_fixture('quint')
            quint['observations']['runs'][0]['records'][0][member] = replacement
            self.assertEqual(self.compare(quint=quint)['status'], 'mismatch')
        for member, replacement in [('terminal_has_next', True), ('extra_state', 'retained')]:
            quint = supplied_fixture('quint')
            quint['observations']['runs'][0][member] = replacement
            self.assertEqual(self.compare(quint=quint)['status'], 'mismatch')

    def test_program_identity_is_compared_before_run_values(self):
        quint = supplied_fixture('quint')
        quint['observations']['programs'][0]['code_k'] = 'different complete code'
        self.assertEqual(self.compare(quint=quint)['paired']['first_difference']['path'],
                         '/programs/0/code_k')

    def test_explicit_selected_mode_reports_omitted_rows_and_keeps_raw_provenance(self):
        k, quint = supplied_fixture('k'), supplied_fixture('quint')
        for artifact in (k, quint):
            artifact['observations']['programs'].append(dict(case_id='UNSELECTED', code_k='raw', entry_k='raw'))
            other = copy.deepcopy(artifact['observations']['runs'][0])
            other['case_id'] = 'UNSELECTED'
            artifact['observations']['runs'].append(other)
        report = self.compare(k, quint)
        self.assertEqual(report['status'], 'match')
        for layer, artifact in [('k', k), ('quint', quint)]:
            row = report['layers'][layer]
            self.assertEqual(row['omitted_programs'], 1)
            self.assertEqual(row['omitted_runs'], 1)
            self.assertEqual(row['evidence'], artifact['evidence'])
            self.assertEqual(row['projection'], artifact['projection'])
            self.assertEqual(row['producer_authenticity'], 'not-established')
        self.assertEqual(report['layers']['k']['raw_context']['raw_terminal_probe'], 'supplied terminal probe')

    def test_absent_selected_run_is_explicit_input_error_not_empty_match(self):
        quint = supplied_fixture('quint')
        quint['observations']['runs'] = []
        with self.assertRaisesRegex(ValueError, 'exactly one selected run'):
            self.compare(quint=quint)

    def test_projection_contract_contains_every_field_and_declares_hidden_steps_and_unmatched_state(self):
        report = self.compare()
        contract = report['projection_contract']
        self.assertEqual(len(contract['fieldMaps']), 9)
        self.assertEqual(len(contract['hiddenSteps']), 4)
        self.assertIn('lastRule', contract['ruleToAction'])
        self.assertIn('not0', contract['hiddenSteps'][0])
        self.assertIn('await/resume', contract['unmatchedFinanceWait'])
        self.assertIn('F03', report['catalogue']['unobserved_by_this_report'])

    def test_selected_cli_api_keeps_default_full_corpus_counts(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = []
            for layer in ('k', 'quint'):
                path = Path(tmp) / (layer + '.json')
                path.write_text(json.dumps(supplied_fixture(layer)))
                paths.append(path)
            report = qa.run(REPO, paired_k_observations=paths[0],
                paired_quint_observations=paths[1], selected_case='N01', selected_initial_fuel=1)
        self.assertEqual(report['counts'], dict(programs=8, state_rows=208, charged_edges=200,
                                               publications=8, prefix_rows=216))
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(report['selected_paired_projection']['status'], 'match')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))

    def test_f03_driver_is_only_literal_input_to_unchanged_generic_machine(self):
        text = (REPO / 'models/sherlock/quint/observations/f03ListsumObservationTest.qnt').read_text()
        self.assertIn('initProgram(f03Program, 93, 64)', text)
        self.assertIn('.then(94.reps(i => step))', text)
        self.assertIn('Call({name: 3, arguments: List(9)})', text)
        self.assertNotIn('.expect(', text)
        self.assertNotIn('pure def ', text)


if __name__ == '__main__':
    unittest.main()
