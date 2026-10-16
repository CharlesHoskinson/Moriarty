"""Normal-data regression tests for the SDK-free QA consumer."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]


class ConsumerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location('sherlock_qa', HERE / 'check.py')
        if spec is None or spec.loader is None:
            raise AssertionError('QA consumer must exist')
        cls.qa = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.qa)
        cls.programs = json.loads((REPO / 'models/sherlock/oracles/normal-programs.json').read_text())
        cls.prefixes = json.loads((REPO / 'models/sherlock/oracles/normal-prefixes.json').read_text())

    def changed(self, mutate):
        programs, prefixes = copy.deepcopy(self.programs), copy.deepcopy(self.prefixes)
        mutate(programs, prefixes)
        return programs, prefixes

    def difference(self, mutate):
        programs, prefixes = self.changed(mutate)
        result = self.qa.compare_documents(self.programs, self.prefixes, programs, prefixes)
        self.assertIsNotNone(result)
        return result

    def invalid(self, mutate, message):
        with self.assertRaisesRegex(ValueError, message):
            self.qa.validate_pair(*self.changed(mutate), family='normal')

    def test_complete_corpus_and_real_rust_transcription(self):
        report = self.qa.run(REPO)
        self.assertEqual(report['counts'], dict(programs=8, state_rows=208, charged_edges=200,
                                              publications=8, prefix_rows=216))
        self.assertEqual(report['status'], 'pass')
        self.assertEqual(len(report['rust_literal_transcriptions']), 2)
        self.assertTrue(all(item['status'] == 'not-run' for item in report['execution_layers']))

    def test_all_fields_match_only_when_present(self):
        self.assertIsNone(self.qa.compare_documents(self.programs, self.prefixes,
                                                    copy.deepcopy(self.programs), copy.deepcopy(self.prefixes)))
        delta = self.difference(lambda p, t: t['traces'][0]['states'][0].pop('environment_k'))
        self.assertEqual(delta['path'], '/prefixes/traces/0/states/0/environment_k')
        self.assertFalse(delta['observed_present'])
        self.assertEqual(delta['case_id'], 'N01')
        self.assertIn('expected_trace', delta)

    def test_missing_state(self):
        self.invalid(lambda p, t: t['traces'][1]['states'].pop(1), 'state|charged')
        self.difference(lambda p, t: t['traces'][1]['states'].pop(1))

    def test_state_order(self):
        self.invalid(lambda p, t: t['traces'][1]['states'].reverse(), 'state|charged')

    def test_case_order(self):
        self.invalid(lambda p, t: t['traces'].reverse(), 'membership')

    def test_case_missing(self):
        self.invalid(lambda p, t: t['traces'].pop(), 'membership')

    def test_work_counters(self):
        self.invalid(lambda p, t: t['traces'][1]['states'][1].update(remaining_fuel=999), 'fuel|work')
        self.difference(lambda p, t: t['traces'][1]['states'][1].update(remaining_fuel=999))

    def test_work_type(self):
        self.invalid(lambda p, t: t['traces'][0]['states'][0].update(charged_steps=False), 'integer')

    def test_wrong_outcome(self):
        delta = self.difference(lambda p, t: t['traces'][0]['publication']['post_overrides'].update(output_k='uiDone(uiU(8))'))
        self.assertIn('output_k', delta['path'])
        self.invalid(lambda p, t: t['traces'][0]['publication']['post_overrides'].update(output_k='uiDone(uiU(8))'), 'publication')

    def test_identity_precedes_state(self):
        def mutate(p, t):
            p['profile'] = 'other/1'
            t['traces'][0]['states'][0]['control_k'] = 'wrong'
        self.assertEqual(self.difference(mutate)['path'], '/programs/profile')
        self.invalid(mutate, 'profile')

    def test_source_identity(self):
        self.assertEqual(self.difference(lambda p, t: t['source'].update(sha256='0' * 64))['path'], '/prefixes/source/sha256')

    def test_program_identity(self):
        self.invalid(lambda p, t: p['programs'][0].update(entry_k='uiLiteral(uiU(8))'), 'input.*digest')
        self.difference(lambda p, t: p['programs'][0].update(code_k='wrong'))

    def test_code_reference(self):
        self.invalid(lambda p, t: t['traces'][0]['fixed_state_fields']['code_ref'].update(case_id='N02'), 'reference')

    def test_initial_configuration_and_startup_not_ignored(self):
        self.difference(lambda p, t: t['traces'][0]['initial_configuration'].pop('code_k'))
        self.invalid(lambda p, t: t['traces'][0]['startup'].update(charge=1), 'startup')

    def test_frame_depth_and_order(self):
        self.invalid(lambda p, t: t['traces'][1]['states'][2].update(depth=0), 'depth')
        delta = self.difference(lambda p, t: t['traces'][3]['states'][8]['frames_top_first_k'].reverse())
        self.assertIn('frames_top_first_k', delta['path'])

    def test_rule_and_terminal_publication_present(self):
        self.invalid(lambda p, t: t['traces'][0]['states'][1].pop('via_rule_start_line'), 'via_rule')
        self.invalid(lambda p, t: t['traces'][0].pop('publication'), 'publication')
        self.difference(lambda p, t: t['traces'][0]['publication'].update(charge=1))

    def test_all_prefixes_required(self):
        self.invalid(lambda p, t: t['traces'][0]['ordinary_fuel_prefixes'].pop(0), 'prefix')
        self.difference(lambda p, t: t['traces'][0]['ordinary_fuel_prefixes'].pop(0))

    def test_prefix_terminal_precedence_and_surplus(self):
        self.invalid(lambda p, t: t['traces'][0]['ordinary_fuel_prefixes'][1].__setitem__(3, 'uiExhausted()'), 'prefix')
        self.invalid(lambda p, t: t['traces'][0]['ordinary_fuel_prefixes'][2].__setitem__(2, 0), 'prefix')

    def test_state_difference_reports_complete_raw_state_and_rule(self):
        delta = self.difference(lambda p, t: t['traces'][1]['states'][2].update(environment_k='0 |-> uiU(9)'))
        self.assertEqual(delta.get('rule_start_line'), 135)
        self.assertEqual(delta.get('state_index'), 2)
        expected = delta.get('expected_state', {})
        self.assertEqual(expected.get('code_k'), 'uiNoFuns()')
        self.assertEqual(expected.get('charged_steps'), 2)
        self.assertEqual(expected.get('remaining_fuel'), 5)
        self.assertEqual(expected.get('initial_fuel'), 7)
        self.assertEqual(expected.get('output_k'), 'uiPending()')
        self.assertEqual(delta.get('observed_state', {}).get('environment_k'), '0 |-> uiU(9)')

    def test_prefix_difference_reports_publication_state_and_rule(self):
        delta = self.difference(lambda p, t: t['traces'][0]['ordinary_fuel_prefixes'][2].__setitem__(3, 'uiExhausted()'))
        self.assertEqual(delta.get('prefix_index'), 2)
        self.assertEqual(delta.get('rule_start_line'), 80)
        self.assertEqual(delta.get('expected_state', {}).get('initial_fuel'), 2)
        self.assertEqual(delta.get('expected_state', {}).get('remaining_fuel'), 1)
        self.assertEqual(delta.get('expected_state', {}).get('control_k'), '.K')
        self.assertEqual(delta.get('expected_state', {}).get('output_k'), 'uiDone(uiU(7))')
        self.assertEqual(delta.get('observed_state', {}).get('output_k'), 'uiExhausted()')

    def test_additional_field_is_compared(self):
        delta = self.difference(lambda p, t: t['traces'][0]['states'][0].update(extra='retained'))
        self.assertFalse(delta['expected_present'])
        self.assertTrue(delta['observed_present'])

    def test_absent_observation_is_not_empty_success(self):
        with self.assertRaisesRegex(ValueError, 'absent'):
            self.qa.compare_documents(self.programs, self.prefixes, None, None)

    def test_real_literal_adapter_change_is_detected(self):
        def reader(path):
            text = path.read_text()
            return text.replace('remaining_fuel: 1,', 'remaining_fuel: 99,', 1)
        with self.assertRaisesRegex(ValueError, 'transcription'):
            self.qa.check_rust_transcription(REPO, self.programs, self.prefixes, 'normal', read_text=reader)

    def test_cli_outputs_report_and_rejects_partial_observation(self):
        result = subprocess.run([sys.executable, str(HERE / 'check.py')], cwd=REPO,
                                capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout).get('status'), 'pass')
        result = subprocess.run([sys.executable, str(HERE / 'check.py'), '--observed-programs', 'absent'],
                                cwd=REPO, capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn('together', result.stderr)

    def test_cli_actual_difference_exit_and_raw_context(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'observed.json'
            prefixes = copy.deepcopy(self.prefixes)
            prefixes['traces'][0]['states'][1]['control_k'] = 'uiRet(uiU(8))'
            path.write_text(json.dumps(prefixes))
            result = subprocess.run([sys.executable, str(HERE / 'check.py'),
                                     '--observed-programs', str(REPO / 'models/sherlock/oracles/normal-programs.json'),
                                     '--observed-prefixes', str(path)], cwd=REPO,
                                    capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 1, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report['status'], 'mismatch')
        self.assertEqual(report['first_difference']['case_id'], 'N01')

    # Security-owned hostile-input regressions, Opus 5.5 high, 2026-10-16.
    def observed_run(self, programs_raw=None, prefixes_raw=None):
        oracles = REPO / 'models/sherlock/oracles'
        with tempfile.TemporaryDirectory() as tmp:
            paths = []
            for name, raw in (('normal-programs.json', programs_raw), ('normal-prefixes.json', prefixes_raw)):
                path = Path(tmp) / name
                path.write_bytes((oracles / name).read_bytes() if raw is None else raw)
                paths.append(path)
            return self.qa.run(REPO, observed_programs=paths[0], observed_prefixes=paths[1])

    def test_strict_reader_accepts_frozen_bytes(self):
        report = self.observed_run()
        self.assertEqual(report['supplied_observation_status'], 'match')
        self.assertEqual(report['supplied_observation_family'], 'normal')

    def test_duplicate_member_cannot_hide_observed_value(self):
        text = json.dumps(self.prefixes)
        self.assertIn('"states": [{', text)
        raw = text.replace('"states": [{', '"states": [{"remaining_fuel": 999, ', 1).encode()
        with self.assertRaisesRegex(ValueError, 'duplicate JSON member'):
            self.observed_run(prefixes_raw=raw)

    def test_nonfinite_observed_number_is_rejected(self):
        text = json.dumps(self.programs)
        for token in ('NaN', 'Infinity', '-Infinity', '1e999'):
            with self.subTest(token=token), self.assertRaisesRegex(ValueError, 'nonfinite'):
                self.observed_run(programs_raw=(text[:-1] + ', "extra": ' + token + '}').encode())

    def test_cli_hostile_observation_is_input_error(self):
        frozen = str(REPO / 'models/sherlock/oracles/normal-prefixes.json')
        cases = (('nonfinite', json.dumps(self.programs)[:-1] + ', "extra": NaN}'),
                 ('depth', '[' * 100000 + ']' * 100000))
        with tempfile.TemporaryDirectory() as tmp:
            for name, text in cases:
                path = Path(tmp) / (name + '.json')
                path.write_text(text)
                with self.subTest(case=name):
                    result = subprocess.run([sys.executable, str(HERE / 'check.py'),
                                             '--observed-programs', str(path), '--observed-prefixes', frozen],
                                            cwd=REPO, capture_output=True, text=True, check=False)
                    self.assertEqual(result.returncode, 2, result.stderr[-2000:])
                    self.assertEqual(result.stdout, '')
                    self.assertIn('QA input error', result.stderr)

    def test_malformed_publication_override_is_reported(self):
        delta = self.difference(lambda p, t: t['traces'][0]['publication'].update(post_overrides='xy'))
        self.assertEqual(delta['path'], '/prefixes/traces/0/publication/post_overrides')
        self.assertIsNone(delta['observed_state'])
        self.assertEqual(delta['rule_start_line'], 80)

    def test_rust_transcription_compares_committed_bytes(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            for relative in ('models/sherlock/reference/check-reference.py',
                             'crates/mori-core/src/sherlock/normal_oracles.rs'):
                raw = (REPO / relative).read_bytes()
                if relative.endswith('.rs'):
                    raw = raw.replace(b'\n', b'\r\n')
                (repo / relative).parent.mkdir(parents=True)
                (repo / relative).write_bytes(raw)
            with self.assertRaisesRegex(ValueError, 'transcription'):
                self.qa.check_rust_transcription(repo, self.programs, self.prefixes, 'normal')

    def test_unpinned_serializer_is_not_executed(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            helper = repo / 'models/sherlock/reference/check-reference.py'
            marker = repo / 'executed'
            helper.parent.mkdir(parents=True)
            body = f"\nopen({str(marker)!r}, 'w').close()\nraise RuntimeError('serializer executed')\n"
            helper.write_bytes((REPO / 'models/sherlock/reference/check-reference.py').read_bytes() + body.encode())
            with self.assertRaisesRegex(ValueError, 'serializer digest'):
                self.qa.check_rust_transcription(repo, self.programs, self.prefixes, 'normal')
            self.assertFalse(marker.exists())

    def test_unusable_observed_state_index_has_no_reconstruction(self):
        cases = (('/prefixes/traces/0/publication/pre_state_index',
                  lambda t, v: t['traces'][0]['publication'].update(pre_state_index=v)),
                 ('/prefixes/traces/0/ordinary_fuel_prefixes/2/1',
                  lambda t, v: t['traces'][0]['ordinary_fuel_prefixes'][2].__setitem__(1, v)))
        for path, mutate in cases:
            for value in (-1, True):
                with self.subTest(path=path, index=value):
                    prefixes = copy.deepcopy(self.prefixes)
                    mutate(prefixes, value)
                    report = self.observed_run(prefixes_raw=json.dumps(prefixes).encode())
                    self.assertEqual(report['status'], 'mismatch')
                    delta = report['first_difference']
                    self.assertEqual(delta['path'], path)
                    self.assertIsNone(delta['observed_state'])
                    self.assertIsNotNone(delta['expected_state'])


if __name__ == '__main__':
    unittest.main()
