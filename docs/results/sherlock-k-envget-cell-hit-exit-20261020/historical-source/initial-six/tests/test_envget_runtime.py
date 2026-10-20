"""Synthetic EnvGet runtime checks; no K evaluation, no actual operand.

Includes the nine frozen field-port tests unchanged in substance, plus
renderer/decoder tests over independently constructed trees. No real
observed afterstate is used as an expected value; afterstates are never
computed in Python.
"""
import copy
import sys
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve()
_CANDIDATES = [_HERE.parents[1] / 'source', _HERE.parent, _HERE.parents[1],
               Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01/actual-before-field-port-source01/source')]
for _candidate in _CANDIDATES:
    if (_candidate / 'envget_runtime.py').exists() or (_candidate / 'envget_ports.py').exists():
        sys.path.insert(0, str(_candidate))
import envget_ports as p
import envget_runtime as r


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


def _int(value):
    return p.literal('Int', value)


def synthetic_event():
    step = p.node('egStepId', p.from_json({'run': ['x"y', 1, True, None]}), _int(0))
    alloc = p.node('ctAlloc', _int(0), _int(0), _int(0))
    occurrence = p.node('egOccurrenceLocator', p.literal('Bool', False), _int(0),
                        p.node('egValue', p.node('egInteger', _int(0)),
                               p.node('egNoValues')))
    args = [step, p.node('egEnvGetCellRule'), p.node('egHelperVisit'),
            p.node('egSiteSome', p.node('chSiteEvalVar')),
            p.node('egSomeEnvGet'), p.node('egApplied'),
            _int(1), _int(1), _int(1), _int(0), alloc, _int(0),
            _int(2), _int(2), _int(0), _int(0),
            p.node('egTriPhase', p.node('chTriPending')),
            p.node('egTriPhase', p.node('chTriPositive')),
            p.node('chIndexSome', _int(0)), p.node('chIndexSome', _int(0)),
            p.node('egNoResultTag'), p.node('egNatSome', _int(6)), _int(3),
            occurrence, p.node('egNoExhaustion'), p.node('egRunningTag'),
            p.node('ctRunning'), p.node('egNoBoundary')]
    assert len(args) == 28
    return p.node('egEvent', *args)


def synthetic_window():
    x = fixture()
    snap = p.from_snapshot(x)
    event = synthetic_event()
    return p.node('egWindow', snap, snap, snap, event, event), x


class EnvGetRuntimeTests(unittest.TestCase):
    def test_generic_strings_preserve_native_kinds(self):
        value = {'s': 'a"b\\c\nd€\U0001F600', 'empty': '',
                 'list': ['x', 0, '0', False, True, None, []]}
        wire = p.from_json(value)
        self.assertEqual(p.to_json(wire), value)
        internal = r.wire_to_internal(wire)
        self.assertEqual(r.internal_to_wire(internal), wire)
        term = r.from_tree(internal)
        parsed = r.parse_printed_term(r.base.codec.render(term), 'EGJSON')
        self.assertEqual(r.base.tree(parsed), internal)
        self.assertEqual(p.to_json(r.internal_to_wire(r.base.tree(parsed))), value)
        self.assertFalse(r.same_whole(r.wire_to_internal(p.from_json('1')),
                                      r.wire_to_internal(p.from_json(1))))

    def test_actual_label_query_inverse_and_entry_lifetime(self):
        self.assertEqual(len(r.LABELS), 306)
        self.assertEqual(len(r.DATA_REG), 279)
        self.assertEqual(len(r.FUNCTION_NAMES), 27)
        self.assertEqual(r.LABELS['egTwoEventWindow'],
                         'egTwoEventWindow(_)_SHERLOCK-ENVGET-POSITIVE-STEP_EGWindow_EGSnapshot')
        self.assertEqual(r.REVERSE_LABELS[r.LABELS['egTwoEventWindow']],
                         'egTwoEventWindow')
        x = fixture()
        term, wire_env, _wire_query = r.build_query_term(x)
        self.assertEqual(term.name, 'egTwoEventWindow')
        self.assertEqual(term.sort, 'EGWindow')
        decoded = r.decode_document(wire_env, 'EGWindow', allow_entry=True)
        self.assertTrue(r.base.same(r.query_tree(term), r.query_tree(decoded)))
        self.assertNotIn('egTwoEventWindow', r.base.codec.REG)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        with self.assertRaises(ValueError):
            r.query_tree(decoded.args[0])
        self.assertNotIn('egTwoEventWindow', r.base.codec.REG)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        with self.assertRaises(ValueError):
            r.from_tree(r.wire_to_internal(p.query(x)))
        with self.assertRaises(ValueError):
            r.decode_document(wire_env, 'EGWindow')
        self.assertEqual(r.base.codec.REG, r.DATA_REG)

    def test_window_output_field_mapping_28_keys(self):
        window, x = synthetic_window()
        native = p.to_window(window)
        self.assertEqual(list(native), ['before', 'after_cell', 'after_exit', 'events'])
        self.assertEqual(len(native['events']), 2)
        for event in native['events']:
            self.assertEqual(list(event), list(p.EVENT_FIELDS))
            self.assertEqual(len(event), 28)
        for field in ('before', 'after_cell', 'after_exit'):
            self.assertEqual(set(native[field]), {'admission', 'image'})
            self.assertEqual(len(native[field]['image']), 23)

    def test_printed_window_roundtrip_and_before_equality(self):
        window, x = synthetic_window()
        term = r.from_tree(r.wire_to_internal(window))
        printed = r.base.codec.render(term)
        parsed = r.parse_printed_window(printed)
        native, internal, wire, before_equal = r.decode_window_term(parsed, x)
        self.assertTrue(before_equal)
        self.assertEqual(native['before'], x)
        self.assertEqual(r.wire_to_internal(wire), internal)
        wrapped = '<generatedTop><k> ' + printed + ' ~> .K </k></generatedTop>'
        self.assertTrue(r.base.same(r.parse_printed_window(wrapped), parsed))

    def test_whole_input_equality_vs_mutated_source_field(self):
        window, x = synthetic_window()
        term = r.from_tree(r.wire_to_internal(window))
        parsed = r.parse_printed_window(r.base.codec.render(term))
        mutated = copy.deepcopy(x)
        mutated['image']['consumed_work'] += 1
        native, _i, _w, before_equal = r.decode_window_term(parsed, mutated)
        self.assertFalse(before_equal)
        self.assertNotEqual(native['before'], mutated)
        self.assertEqual(native['before'], x)

    def test_residual_function_unknown_label_and_trailing_refused(self):
        window, x = synthetic_window()
        snap_term = r.from_tree(r.wire_to_internal(window['arguments'][0]))
        snap_printed = r.base.codec.render(snap_term)
        with self.assertRaises(ValueError):
            r.parse_printed_window('egTwoEventWindow(' + snap_printed + ')')
        with self.assertRaises(ValueError):
            r.parse_printed_window('egCellHit(' + snap_printed + ')')
        term = r.from_tree(r.wire_to_internal(window))
        printed = r.base.codec.render(term)
        with self.assertRaises(ValueError):
            r.parse_printed_window(printed.replace('egWindow(', 'egMissing(', 1))
        with self.assertRaises(ValueError):
            r.parse_printed_window(printed + ' ' + printed)
        with self.assertRaises(ValueError):
            r.decode_document({'format': 'KAST', 'version': 4, 'term': {
                'node': 'KApply',
                'label': {'node': 'KLabel', 'name': 'egMissing()_X_Y', 'params': []},
                'arity': 0, 'args': []}}, 'EGWindow')

    def test_scalar_kind_and_sort_confusion_refused(self):
        with self.assertRaises(ValueError):
            r.from_tree({'sort': 'Int', 'value': True})
        with self.assertRaises(ValueError):
            r.from_tree({'sort': 'Bool', 'value': 1})
        with self.assertRaises(ValueError):
            r.wire_to_internal({'sort': 'Int', 'value': 1})
        with self.assertRaises(ValueError):
            r.internal_to_wire({'sort': 'String', 'literal': 'x'})
        bad = copy.deepcopy(synthetic_window()[0])
        bad['arguments'][3] = bad['arguments'][0]
        with self.assertRaises(ValueError):
            r.from_tree(r.wire_to_internal(bad))


if __name__ == '__main__':
    unittest.main()
