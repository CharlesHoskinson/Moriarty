"""Independent normal diagnostic fixtures; no interpreter execution or oracle imports."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

MODULE = Path(__file__).with_name('work_summary.py')
SPEC = importlib.util.spec_from_file_location('work_summary', MODULE)
work = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(work)


def snapshot(case, fuel, step, control, frames=()):
    return dict(code_k='uiNoFuns()', entry_k='literal7' if case == 'N01' else 'add2and3',
                control_k=control, environment_k='.Map', frames_top_first_k=list(frames),
                depth=len(frames), stack_limit=8, remaining_fuel=fuel-step,
                initial_fuel=fuel, output_k='uiPending()', charged_steps=step)


def run(case, fuel):
    # N02's seven incoming rules and saved-frame depths are specified here,
    # independently of the consumer and committed oracle reconstruction code.
    rows = ([('literal7', (), None), ('return7', (), 102)] if case == 'N01' else
            [('add2and3', (), None), ('pushLeft', (), 117),
             ('eval2', ('left',), 91), ('return2', ('left',), 102),
             ('eval3', ('right2',), 123), ('return3', ('right2',), 102),
             ('apply2and3', (), 126), ('return5', (), 141)])
    boundary = min(fuel, len(rows)-1)
    states = [snapshot(case, fuel, i, control, frames)
              for i, (control, frames, _) in enumerate(rows[:boundary+1])]
    records = [dict(rule_start_line=rows[i][2], charge=1,
                    before=states[i-1], after=states[i]) for i in range(1, len(states))]
    terminal = dict(states[-1], control_k='.K', output_k=(
        'uiDone(uiU(7))' if case == 'N01' else 'uiDone(uiU(5))')
        if boundary == len(rows)-1 else 'uiExhausted()')
    records.append(dict(rule_start_line=80 if boundary == len(rows)-1 else 84,
                        charge=0, before=states[-1], after=terminal))
    return dict(case_id=case, initial=states[0], records=records,
                terminal=terminal, terminal_has_next=False)


def artifact(selection=(('N01', (0, 1, 2)), ('N02', (0, 3, 7, 8)))):
    return dict(producer='independent-normal-fixture', profile='sherlock-k-a/1',
                source_sha256='a'*64, boundary='post-startup',
                programs=[dict(case_id=case, code_k='uiNoFuns()',
                               entry_k='literal7' if case == 'N01' else 'add2and3')
                          for case, _ in selection],
                runs=[run(case, fuel) for case, fuels in selection for fuel in fuels])


class WorkSummaryTests(unittest.TestCase):
    def test_zero_middle_and_full_run_columns(self):
        report = work.summarize(artifact())
        zero, full, surplus, _, middle, n02full, _ = report['runs']
        self.assertEqual(zero['key'], dict(case_id='N01', initial_fuel=0))
        self.assertEqual((zero['record_count'], zero['charged_sum'], zero['publication_count']), (1, 0, 1))
        self.assertEqual(zero['per_source_rule'], [dict(rule_start_line=84, event_count=1, charged_sum=0)])
        self.assertEqual((full['record_count'], full['charged_sum'], full['publication_count']), (2, 1, 1))
        self.assertEqual((surplus['terminal_work']['charged_steps'], surplus['terminal_work']['remaining_fuel']), (1, 1))
        self.assertEqual((middle['record_count'], middle['charged_sum'], middle['max_frames'], middle['max_depth']), (4, 3, 1, 1))
        self.assertEqual(middle['per_source_rule'], [
            dict(rule_start_line=117, event_count=1, charged_sum=1),
            dict(rule_start_line=91, event_count=1, charged_sum=1),
            dict(rule_start_line=102, event_count=1, charged_sum=1),
            dict(rule_start_line=84, event_count=1, charged_sum=0)])
        self.assertEqual((n02full['record_count'], n02full['charged_sum'], n02full['publication_count']), (8, 7, 1))
        self.assertEqual(n02full['per_source_rule'][2], dict(rule_start_line=102, event_count=2, charged_sum=2))
        self.assertEqual(n02full['initial_work'], dict(initial_fuel=7, remaining_fuel=7, charged_steps=0))
        self.assertFalse(n02full['terminal_has_next'])

    def test_cross_run_prefix_work_and_unique_complete_work_are_separate(self):
        total = work.summarize(artifact())['cross_run']
        self.assertEqual(total['prefix_runs'], dict(run_count=7, record_count=26, charged_sum=19,
                                                  charged_event_count=19, publication_count=7))
        self.assertEqual(total['charged_prefix_events'], dict(observed=19, distinct=8, repeated=11))
        self.assertEqual(total['unique_program_full_traces']['record_count'], 10)
        self.assertEqual(total['unique_program_full_traces']['charged_sum'], 8)
        self.assertEqual(total['unique_program_full_traces']['selected_run_indices'], [1, 5])

    def test_publications_are_counted_separately_in_all_event_fingerprints(self):
        total = work.summarize(artifact((('N01', (0, 1, 2)),)))['cross_run']
        # Exhaustion publication differs from successful publication. Only the
        # literal edge and its successful publication repeat at surplus fuel.
        self.assertEqual(total['prefix_event_fingerprints'], dict(observed=5, distinct=3, repeated=2))
        self.assertEqual(total['charged_prefix_events'], dict(observed=2, distinct=1, repeated=1))

    def test_zero_only_and_middle_only_have_unknown_complete_work(self):
        for fuels in ((0,), (3,)):
            with self.subTest(fuels=fuels):
                total = work.summarize(artifact((('N02', fuels),)))['cross_run']
                self.assertIsNone(total['unique_program_full_traces']['charged_sum'])
                self.assertIsNone(total['unique_program_full_traces']['record_count'])
                self.assertEqual(total['unique_program_full_traces']['not_observed_case_ids'], ['N02'])
                self.assertEqual(total['charged_prefix_events']['repeated'], 0)

    def test_preserves_order_membership_and_declared_identity_without_mutation(self):
        value = artifact((('N02', (7, 0)), ('N01', (1,))))
        before = copy.deepcopy(value)
        report = work.summarize(value)
        self.assertEqual(value, before)
        self.assertEqual(report['declarations'], {key: value[key] for key in (
            'producer', 'profile', 'source_sha256', 'boundary')})
        self.assertEqual(report['programs'], value['programs'])
        self.assertEqual([r['key'] for r in report['runs']], [
            dict(case_id='N02', initial_fuel=7), dict(case_id='N02', initial_fuel=0),
            dict(case_id='N01', initial_fuel=1)])
        expected = [hashlib.sha256(json.dumps(row, sort_keys=True, separators=(',', ':'),
                                             ensure_ascii=False, allow_nan=False).encode()).hexdigest()
                    for row in value['runs'][0]['records']]
        self.assertEqual(report['runs'][0]['record_sha256_in_trace_order'], expected)

    def test_null_work_counter_and_next_declaration_are_retained(self):
        value = artifact((('N01', (1,)),))
        value['runs'][0]['terminal']['charged_steps'] = None
        value['runs'][0]['terminal_has_next'] = True
        row = work.summarize(value)['runs'][0]
        self.assertIsNone(row['terminal_work']['charged_steps'])
        self.assertTrue(row['terminal_has_next'])

    def test_existing_envelopes_keep_projection_declarations(self):
        baseline = artifact()
        for fmt in ('sherlock-lean-normal-observations/1', 'sherlock-quint-itf-observations/1',
                    'sherlock-image-observations/1'):
            with self.subTest(format=fmt):
                envelope = dict(format=fmt, observations=baseline,
                                projection=dict(terminal_has_next='declared-normal-projection'))
                report = work.summarize(envelope)
                self.assertEqual(report['input_format'], fmt)
                self.assertEqual(report['projection'], envelope['projection'])
                self.assertEqual(report['runs'], work.summarize(baseline)['runs'])

    def test_cli_reads_only_explicit_artifact_and_reports_exact_byte_hash(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)/'supplied.json'
            raw = (json.dumps(artifact())+'\n').encode()
            path.write_bytes(raw)
            completed = subprocess.run([sys.executable, '-I', '-B', str(MODULE), str(path)],
                                       capture_output=True, text=True, check=False)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        report = json.loads(completed.stdout)
        self.assertEqual(report['input']['sha256'], hashlib.sha256(raw).hexdigest())
        self.assertEqual(report['input']['bytes'], len(raw))
        self.assertEqual(report['evidence']['execution_authenticity'], 'not-established')
        self.assertEqual(report['physical_cost']['status'], 'unmeasured')


if __name__ == '__main__':
    unittest.main()
