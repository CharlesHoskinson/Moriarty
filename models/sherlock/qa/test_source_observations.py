"""Ordinary supplied source diagnostics; copied literals are test fixtures only."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

PATH = Path(__file__).with_name('check_source_observations.py')
SPEC = importlib.util.spec_from_file_location('source_observation_checker', PATH)
source = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(source)
REPO = Path(__file__).resolve().parents[3]


class SourceObservationTests(unittest.TestCase):
    def fixture(self):
        return source.expected_observations(REPO)

    def compare(self, artifact):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'copied-literal-test-fixture.json'
            path.write_text(json.dumps(artifact), encoding='utf-8')
            return source.run(REPO, observations=path)

    def test_frozen_source_projection_has_two_cases_and_every_complete_prefix(self):
        value = self.fixture()
        self.assertEqual([row['case_id'] for row in value['programs']], ['S01', 'S02'])
        self.assertEqual(len(value['runs']), 70)
        self.assertEqual(sum(len(row['records']) for row in value['runs']), 1259)
        self.assertEqual([len(row['preparation']['occurrences']) for row in value['programs']], [9, 8])
        self.assertEqual(value['runs'][34]['terminal']['output_k'], 'uiDone(uiU(39))')
        self.assertEqual(value['runs'][68]['terminal']['output_k'], 'uiDone(uiB(false))')

    def test_matching_copied_literals_remain_supplied_artifact_comparison_only(self):
        report = self.compare(self.fixture())
        self.assertEqual(report['status'], 'pass')
        supplied = report['supplied_source_observation']
        self.assertEqual(supplied['status'], 'match')
        self.assertEqual(supplied['producer_authenticity'], 'not-established')
        self.assertEqual(supplied['execution_receipt'], 'not-supplied')
        self.assertEqual(supplied['expected_records'], 1259)
        self.assertEqual(report['acceptance'], 'none')
        self.assertTrue(all(row['status'] == 'not-run' for row in report['execution_layers']))
        self.assertEqual(report['first_difference'], None)

    def test_complete_preparation_report_is_compared_before_run_states(self):
        artifact = self.fixture()
        artifact['programs'][1]['preparation']['occurrences'][3]['body_path'] = [1]
        artifact['runs'][0]['terminal']['remaining_fuel'] = 1
        report = self.compare(artifact)
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['first_difference']['path'],
                         '/programs/1/preparation/occurrences/3/body_path/0')
        self.assertEqual(report['source_preparation_comparison']['status'], 'mismatch')

    def test_source_text_profiles_and_all_program_identities_are_compared(self):
        for field in ('source_text', 'source_path', 'source_language', 'source_semantics',
                      'code_k', 'entry_k'):
            with self.subTest(field=field):
                artifact = self.fixture()
                artifact['programs'][1][field] += 'different supplied text'
                report = self.compare(artifact)
                self.assertEqual(report['first_difference']['path'], '/programs/1/' + field)

    def test_full_saved_frames_and_work_have_record_context(self):
        artifact = self.fixture()
        artifact['runs'][34]['records'][30]['after']['frames_top_first_k'][0] = 'different saved frame'
        delta = self.compare(artifact)['first_difference']
        self.assertEqual(delta['path'], '/runs/34/records/30/after/frames_top_first_k/0')
        self.assertEqual(delta['case_id'], 'S01')
        self.assertEqual(delta['initial_fuel'], 34)
        self.assertEqual(delta['record_index'], 30)
        self.assertEqual(delta['observed_state']['frames_top_first_k'][0], 'different saved frame')

    def test_all_prefixes_and_final_free_publication_are_required(self):
        for target in ('prefix', 'publication'):
            with self.subTest(target=target):
                artifact = self.fixture()
                if target == 'prefix':
                    artifact['runs'].pop()
                else:
                    artifact['runs'][-1]['records'].pop()
                self.assertEqual(self.compare(artifact)['status'], 'mismatch')

    def test_source_mapping_unexpected_field_is_retained_as_difference(self):
        artifact = self.fixture()
        artifact['programs'][0]['preparation']['extra_diagnostic'] = 'supplied field'
        delta = self.compare(artifact)['first_difference']
        self.assertEqual(delta['path'], '/programs/0/preparation/extra_diagnostic')
        self.assertEqual(delta['observed'], 'supplied field')

    def test_matching_program_reports_do_not_hide_different_terminal(self):
        artifact = self.fixture()
        artifact['runs'][68]['terminal']['output_k'] = 'uiDone(uiB(true))'
        report = self.compare(artifact)
        self.assertEqual(report['source_preparation_comparison']['status'], 'match')
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['first_difference']['case_id'], 'S02')

    def test_profile_declarations_are_compared(self):
        artifact = self.fixture()
        artifact['profile'] = 'different-profile'
        self.assertEqual(self.compare(artifact)['first_difference']['path'], '/profile')

    def test_isolated_cli_reports_only_the_supplied_fixture_comparison(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'copied-literal-test-fixture.json'
            path.write_text(json.dumps(self.fixture()), encoding='utf-8')
            process = subprocess.run([sys.executable, '-I', '-B', str(PATH),
                                      '--observations', str(path)],
                                     capture_output=True, text=True, check=False)
        self.assertEqual(process.returncode, 0, process.stderr)
        report = json.loads(process.stdout)
        self.assertEqual(report['supplied_source_observation']['status'], 'match')
        self.assertEqual(report['supplied_source_observation']['execution_receipt'], 'not-supplied')
        self.assertEqual(report['counts']['prefix_runs'], 70)


if __name__ == '__main__':
    unittest.main()
