"""Positive carrier tests only; no K equation, lane encoder or oracle execution."""
import copy
import json
from pathlib import Path
import sys
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'source'))
import state_ports as p

class PositiveStatePorts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.doc = json.loads(p.INPUT_PATH.read_text())

    def test_whole_source_cohort_inverse(self):
        self.assertEqual(p.cohort_to_document(p.document_to_cohort(self.doc)), self.doc)

    def test_all_helper_result_and_staged_carriers(self):
        for case in self.doc['helper_record_cases']:
            with self.subTest(case=case['case_id']):
                term = p.from_json('CHHelperOption', case['helper'])
                self.assertEqual(p.to_json(term), case['helper'])
        for case in self.doc['result_record_cases']:
            with self.subTest(case=case['case_id']):
                self.assertEqual(p.to_json(p.from_json('CHResultOption', case['result'])), case['result'])

    def test_three_tasks_and_ordered_duplicate_task_stack(self):
        for case in self.doc['task_record_cases']:
            self.assertEqual(p.to_json(p.from_json('CSTask', case['task'])), case['task'])
        tasks = self.doc['state_cases'][0]['state']['helper']['tasks']
        self.assertEqual(p.to_json(p.from_json('CSTasks', tasks)), tasks)
        self.assertEqual(len(p.to_json(p.from_json('CSTasks', tasks + tasks))), 2 * len(tasks))

    def test_root16_frame17_split_preserves_all_fields(self):
        for case in self.doc['state_cases']:
            state = case['state']
            t = p.from_json('CSOwnedState', state)
            self.assertEqual(len(t['arguments'][0]['arguments']), 17)
            self.assertEqual(p.to_json(t), state)
            changed = copy.deepcopy(state)
            changed['frames'] = list(reversed(changed['frames']))
            self.assertNotEqual(p.from_json('CSOwnedState', changed), t)
            self.assertEqual(p.to_json(p.from_json('CSOwnedState', changed)), changed)

    def test_owned_vectors_not_active_prefix_or_capacity_padding(self):
        state = self.doc['state_cases'][0]['state']
        before = copy.deepcopy(state['data'])
        changed = copy.deepcopy(state)
        changed['root']['active'] = dict(data=0, data_fields=0, env=0, value_slots=0)
        self.assertEqual(p.to_json(p.from_json('CSOwnedState', changed))['data'], before)

    def test_none_somezero_scalar_none_and_ref_kinds_distinct(self):
        self.assertNotEqual(p.from_json('CTEnv', None), p.from_json('CTEnv', {'Some': 0}))
        self.assertNotEqual(p.from_json('CTFunction', None), p.from_json('CTFunction', {'Some': 0}))
        self.assertNotEqual(p.from_json('CTRef', {'Const': 0}), p.from_json('CTRef', {'Data': 0}))
        self.assertEqual(p.to_json(p.from_json('CHStaged', 'None')), 'None')
        self.assertEqual(p.to_json(p.from_json('CHHelperOption', None)), None)

    def test_u128_bool_empty_slice_and_context29_retained(self):
        n = (1 << 128) - 1
        self.assertEqual(p.to_json(p.from_json('DPData', {'U128': {'n': n}})), {'U128': {'n': n}})
        self.assertIs(p.to_json(p.from_json('DPData', {'Bool': {'b': True}}))['Bool']['b'], True)
        self.assertEqual(p.to_json(p.from_json('CTSlice', dict(start=17, len=0))), dict(start=17, len=0))
        for case in self.doc['state_cases']:
            t = p.from_json('CSContext', case['context'])
            self.assertEqual(len(t['arguments'][0]['arguments']), 29)
            self.assertEqual(p.to_json(t), case['context'])

    def test_caseids_and_query_hold_entire_original_cohort(self):
        t = p.query(self.doc)
        self.assertEqual(t['constructor'], 'csObserveAll')
        self.assertEqual(p.cohort_to_document(t['arguments'][0]), self.doc)
        self.assertEqual(set(t), {'sort', 'constructor', 'arguments'})

    def test_observation_shape_is_full_eightfield_root(self):
        # Synthetic carrier, no predicted codec output. All five lists empty.
        empty = dict(schema='independent-c6-positive-helper-task-state/1', profile=self.doc['profile'],
                     case_ids={key: [] for key in p.CASE_KEYS},
                     helper_record_observations=[], result_record_observations=[],
                     task_record_observations=[], helper_section_observations=[], state_observations=[])
        self.assertEqual(p.to_json(p.from_json('CSObservations', empty)), empty)

if __name__ == '__main__':
    unittest.main()
