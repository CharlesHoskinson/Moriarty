"""Synthetic field-port checks only; no K evaluation or owner admission assertion."""
import copy
import sys
import unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'source'))
import envget_ports as p


def fixture():
    active = dict(data=0, data_fields=0, env=1, value_slots=0)
    alloc = dict(data_nodes=0, data_fields=0, environment_cells=1)
    return dict(admission=dict(identity={'test': [False, 0, '0', None]},
                               bounds={}, view={'unused': []}, layout={}),
                image=dict(run_id={'test_run': ['immutable', 3]},
                    control={'Eval': {'expr': 3}}, control_origin=3,
                    env_root={'Some': 0}, stack_limit=8, outcome='Running',
                    initial_fuel=128, remaining_work=120, consumed_work=8,
                    helper_work=5, helper_visits=2, reference_cursor=6,
                    alloc=alloc, active=active,
                    helper={'Some': {'EnvGet': {
                        'common': {'site': 'EvalVar', 'origin_reference_cursor': 6,
                                   'parent_resume': {'Some': 0}},
                        'binder': 0, 'cursor': {'Some': 0}, 'found': None,
                        'status': 'Pending'}}}, result=None,
                    seg_base=dict(active=dict(active), alloc=dict(alloc), consumed_work=4),
                    data=[], data_fields=[],
                    env=[{'binder': 0, 'value': {'Const': 0}, 'previous': None}],
                    frames=[{'ScopeF': {'saved_env': None, 'origin': 0}},
                            {'LetF': {'binder': 1, 'body': 4,
                                      'saved_env': {'Some': 0}, 'origin': 2}}],
                    tasks=[], value_slots=[]))


class FieldPorts(unittest.TestCase):
    def test_complete_roundtrip_and_no_input_mutation(self):
        x = fixture(); saved = copy.deepcopy(x)
        self.assertEqual(p.to_snapshot(p.from_snapshot(x)), x)
        self.assertEqual(x, saved)

    def test_all_23_image_fields_retained(self):
        x = fixture()
        self.assertEqual(set(p.to_snapshot(p.from_snapshot(x))['image']), set(x['image']))
        self.assertEqual(len(x['image']), 23)

    def test_opaque_admission_and_run_id(self):
        x = fixture(); y = copy.deepcopy(x)
        y['admission']['view']['new_empty_table'] = []
        y['image']['run_id']['extra'] = {'Some': 0}
        self.assertNotEqual(p.from_snapshot(x), p.from_snapshot(y))
        self.assertEqual(p.to_snapshot(p.from_snapshot(y)), y)

    def test_outer_helper_result_and_nested_options(self):
        x = fixture()
        x['image']['result'] = {'Some': {'EnvFound': {'value': {'Data': 0}}}}
        x['image']['helper']['Some']['EnvGet']['found'] = {'Some': {'Const': 0}}
        self.assertEqual(p.to_snapshot(p.from_snapshot(x)), x)
        x['image']['helper'] = None
        self.assertEqual(p.to_snapshot(p.from_snapshot(x)), x)

    def test_none_vs_some_zero(self):
        x = fixture(); y = copy.deepcopy(x); y['image']['env_root'] = None
        self.assertNotEqual(p.from_snapshot(x), p.from_snapshot(y))
        self.assertEqual(p.to_snapshot(p.from_snapshot(y)), y)

    def test_raw_frames_and_saved_environment(self):
        x = fixture(); y = copy.deepcopy(x); y['image']['frames'].reverse()
        self.assertNotEqual(p.from_snapshot(x), p.from_snapshot(y))
        self.assertEqual(p.to_snapshot(p.from_snapshot(y)), y)

    def test_seg_base_and_inactive_rows(self):
        x = fixture(); x['image']['seg_base']['consumed_work'] = 13
        x['image']['data'] = [{'U128': {'n': (1 << 128) - 1}}]
        x['image']['data_fields'] = [{'Data': 0}, {'Const': 0}]
        x['image']['value_slots'] = [{'Const': 0}]
        self.assertEqual(p.to_snapshot(p.from_snapshot(x)), x)

    def test_generic_native_kinds_and_order(self):
        x = {'values': [False, 0, True, 1, '1', None, [], {'Some': 0}]}
        self.assertEqual(p.to_json(p.from_json(x)), x)
        self.assertNotEqual(p.from_json(False), p.from_json(0))

    def test_query_is_unreduced_and_before_only(self):
        x = fixture(); q = p.query(x)
        self.assertEqual(q['constructor'], 'egTwoEventWindow')
        self.assertEqual(q['sort'], 'EGWindow')
        self.assertEqual(len(q['arguments']), 1)
        self.assertEqual(p.to_snapshot(q['arguments'][0]), x)


if __name__ == '__main__':
    unittest.main()
