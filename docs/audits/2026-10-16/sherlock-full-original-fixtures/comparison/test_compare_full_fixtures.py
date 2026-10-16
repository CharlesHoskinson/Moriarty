"""Positive literal construction tests; synthetic rows are never host evidence."""
import copy
import importlib.util
import pathlib
import unittest

HERE = pathlib.Path(__file__).resolve().parent
REPO = pathlib.Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
RUNTIME = HERE.parent
spec = importlib.util.spec_from_file_location('full_fixtures', HERE / 'compare_full_fixtures.py')
consumer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(consumer)


class CompleteFrozenConstruction(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.qa, cls.host, cls.expected, cls.membership = consumer.prepare(REPO, RUNTIME)
        cls.freeze, cls.families, cls.arithmetic = consumer.load_frozen(RUNTIME, cls.qa)

    def test_complete_independently_frozen_membership(self):
        self.assertEqual([row['host_case_id'] for row in self.membership],
                         ['F01', 'F02', 'F03', 'F04', 'F05', 'F06', 'F07', 'F08'])
        self.assertEqual([len(row['ordinary_prefix_fuels']) for row in self.membership],
                         [115, 115, 95, 62, 15, 22, 23, 122])
        self.assertEqual(len(self.expected['observations']['runs']), 569)
        self.assertEqual(len(self.expected['original_fixture_runs']), 8)
        self.assertEqual([row['original_case_id'] for row in self.membership],
                         [row['case_id'] for row in self.freeze['cases']])
        self.assertEqual([row['input_sha256'] for row in self.membership],
                         [row['input_sha256'] for row in self.freeze['cases']])

    def test_every_prefix_uses_complete_existing_helper_projection(self):
        index = 0
        for programs, prefixes in self.families:
            for program, trace in zip(programs['programs'], prefixes['traces'], strict=True):
                for fuel, boundary, remaining, output, rule in trace['ordinary_fuel_prefixes']:
                    run = self.expected['observations']['runs'][index]
                    self.assertEqual(run['initial'], self.host.literal_snapshot(program, trace, trace['states'][0], fuel))
                    self.assertEqual(len(run['records']), boundary + 1)
                    self.assertEqual(run['terminal'], self.host.literal_snapshot(program, trace, trace['states'][boundary], fuel, published_output=output))
                    self.assertEqual(run['terminal']['remaining_fuel'], remaining)
                    self.assertEqual(run['records'][-1]['rule_start_line'], rule)
                    self.assertEqual(run['records'][-1]['charge'], 0)
                    self.assertIs(run['terminal_has_next'], False)
                    snapshots = [run['initial'], run['terminal']]
                    for record in run['records']:
                        snapshots.extend((record['before'], record['after']))
                    for snapshot in snapshots:
                        self.assertEqual(set(snapshot), set(consumer.SNAPSHOT_FIELDS))
                    index += 1
        self.assertEqual(index, 569)

    def test_original_runs_preserve_independent_budget_and_terminal_literals(self):
        for index, run in enumerate(self.expected['original_fixture_runs']):
            case = self.freeze['cases'][index]
            self.assertEqual(run['initial']['initial_fuel'], 1000)
            self.assertEqual(run['initial']['remaining_fuel'], 1000)
            self.assertEqual(run['initial']['stack_limit'], 64)
            self.assertEqual(len(run['records']), case['charged_edges_specified'] + 1)
            self.assertEqual(run['terminal']['remaining_fuel'], 1000 - case['charged_edges_specified'])
            self.assertIs(run['terminal_has_next'], False)
        for literal, run in zip(self.arithmetic['traces'], self.expected['original_fixture_runs'][:2], strict=True):
            for key, value in literal['expected_terminal_summary'].items():
                self.assertEqual(run['terminal']['charged_steps' if key == 'charged_work' else key], value)
        for trace, run in zip(self.families[1][1]['traces'], self.expected['original_fixture_runs'][2:5], strict=True):
            for key, value in trace['original_fixture_run']['terminal'].items():
                if key != 'code_ref':
                    self.assertEqual(run['terminal'][key], value)
            overlays = trace['original_fixture_run']['state_fuel_overlays']
            for (index, remaining), record in zip(overlays[1:], run['records'][:-1], strict=True):
                self.assertEqual(record['after']['charged_steps'], index)
                self.assertEqual(record['after']['remaining_fuel'], remaining)

    def test_complete_synthetic_artifact_roundtrip_and_diagnostic_retention(self):
        diagnostic = {'format': 'synthetic-test-diagnostics-only',
                      'inputs': copy.deepcopy(self.expected['fixture_identities']),
                      'prefix_selection': [{'synthetic': True}],
                      'original_runs': ['uninterpreted original debug'],
                      'runs': ['uninterpreted prefix debug']}
        artifact = dict(format=consumer.FORMAT,
                        observations=copy.deepcopy(self.expected['observations']),
                        original_fixture_runs=copy.deepcopy(self.expected['original_fixture_runs']),
                        projection=diagnostic, extra_diagnostic='retained')
        before = copy.deepcopy(artifact)
        report = consumer.compare_prepared(self.expected, artifact, self.qa, self.host, self.membership)
        self.assertEqual(report['status'], 'match')
        self.assertEqual(report['acceptance'], 'none')
        self.assertEqual(report['retained_diagnostics']['projection'], diagnostic)
        self.assertEqual(report['retained_diagnostics']['extra_diagnostic'], 'retained')
        self.assertEqual(artifact, before)
        self.assertIs(report['subject_execution'], False)
        self.assertEqual(report['execution_evidence'], 'not-established-by-this-consumer')


if __name__ == '__main__':
    unittest.main(verbosity=2)
