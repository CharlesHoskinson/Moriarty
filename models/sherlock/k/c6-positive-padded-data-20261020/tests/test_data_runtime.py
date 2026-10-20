"""Combined DATA runtime and preserved field-port tests. UNRUN."""
import copy
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'source'))
import data_ports as p
import data_runtime as r


def lanes(values):
    result = p.term('CTLanes', 'ctNoLanes')
    for value in reversed(values):
        result = p.term('CTLanes', 'ctLane', p.atom('Int', value), result)
    return result


def synthetic_tables():
    return {
        'data': [
            {'U128': {'n': 340282366920938463463374607431768211455}},
            {'Bool': {'b': True}},
            {'Bool': {'b': False}},
            {'Nil': None},
            {'Cons': {'fields': {'start': 0, 'len': 2}}},
            {'Leaf': {'fields': {'start': 2, 'len': 1}}},
            {'Branch': {'fields': {'start': 4, 'len': 0}}},
            {'Constructor': {'type_id': 9, 'ctor_id': 10,
                             'fields': {'start': 1, 'len': 2}}},
        ],
        'data_fields': [{'Const': 5}, {'Data': 7}, {'Const': 0}, {'Data': 1}],
        'env': [
            {'binder': 8, 'value': {'Data': 6}, 'previous': {'None': None}},
            {'binder': 0, 'value': {'Const': 5}, 'previous': {'Some': 0}},
            {'binder': 3, 'value': {'Data': 7}, 'previous': {'Some': 3}},
        ],
        'value_slots': [{'Data': 7}, {'Const': 5}],
    }


def synthetic_observation_tree():
    return p.term(
        'DPObservation', 'dpObservation',
        p.term('DPGeometry', 'dpGeometry',
               p.atom('Int', 7), p.atom('Int', 47), p.atom('Int', 57),
               p.atom('Int', 77), p.atom('Int', 83)),
        p.term('CTActive', 'ctActive',
               p.atom('Int', 8), p.atom('Int', 4), p.atom('Int', 3),
               p.atom('Int', 2)),
        lanes([18918541878711089, 1, 1145132097, 8, 4, 3, 2,
               0, 5, 0, 0, 0, 0, 0]),
        p.tables(synthetic_tables()))


class DataRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.input = json.loads(
            (ROOT / 'SAFE/SAFE-INPUT-ONLY.proposed.json').read_bytes())

    def test_full_typed_observation_projects_exact_four_keys(self):
        tree = synthetic_observation_tree()
        actual = p.observation_document(tree)
        self.assertEqual(list(actual),
                         ['geometry', 'counts', 'padded_lanes', 'decoded_tables'])
        self.assertEqual(actual['geometry'], {
            'data_base': 7, 'fields_base': 47, 'env_base': 57,
            'values_base': 77, 'total': 83})
        self.assertEqual(actual['counts'], [8, 4, 3, 2])
        self.assertEqual(actual['padded_lanes'],
                         [18918541878711089, 1, 1145132097, 8, 4, 3, 2,
                          0, 5, 0, 0, 0, 0, 0])
        self.assertEqual(actual['decoded_tables'], synthetic_tables())
        self.assertNotIn('profile', actual)
        self.assertNotIn('caseId', actual)
        self.assertNotIn('source_input', actual)

    def test_native_bool_and_int_remain_distinct(self):
        tree = synthetic_observation_tree()
        decoded = p.observation_document(tree)['decoded_tables']['data']
        self.assertIs(decoded[1]['Bool']['b'], True)
        self.assertIs(type(decoded[1]['Bool']['b']), bool)
        self.assertIs(decoded[2]['Bool']['b'], False)
        self.assertIs(type(decoded[0]['U128']['n']), int)
        self.assertFalse(r.same_whole(True, 1))
        self.assertFalse(r.same_whole({'value': True}, {'value': 1}))
        self.assertTrue(r.same_whole({'value': True}, {'value': True}))

    def test_some_zero_none_const_data_empty_slice_and_full128(self):
        decoded = p.observation_document(
            synthetic_observation_tree())['decoded_tables']
        self.assertEqual(decoded['data'][0]['U128']['n'].bit_length(), 128)
        self.assertEqual(decoded['data'][6],
                         {'Branch': {'fields': {'start': 4, 'len': 0}}})
        self.assertEqual(decoded['env'][0]['previous'], {'None': None})
        self.assertEqual(decoded['env'][1]['previous'], {'Some': 0})
        self.assertNotEqual(decoded['env'][0]['previous'],
                            decoded['env'][1]['previous'])
        self.assertEqual(decoded['data_fields'][0], {'Const': 5})
        self.assertEqual(decoded['data_fields'][1], {'Data': 7})
        self.assertNotEqual(decoded['data_fields'][0],
                            decoded['data_fields'][1])

    def test_original_input_query_inverse_and_metadata_separation(self):
        term, wire, ported = r.carrier(copy.deepcopy(self.input))
        self.assertEqual(ported['profile'],
                         'sherlock-c6-data-positive-padded/1')
        self.assertEqual(ported['case_id'],
                         'seven-variants-partial-unequal-capacities-01')
        self.assertEqual(ported['source_document'], self.input)
        self.assertEqual(wire['format'], 'KAST')
        self.assertEqual(wire['version'], 4)
        self.assertNotIn('dpObserve', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 26)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        saved_registry = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            self.assertEqual(len(r.base.codec.REG), 27)
            self.assertIn('dpObserve', r.base.codec.REG)
            query_tree = r.base.tree(term)
            decoded_tree = r.base.tree(r.decode_document(wire, 'DPObservation'))
        finally:
            r.base.codec.REG = saved_registry
        self.assertIs(r.base.codec.REG, saved_registry)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        self.assertNotIn('dpObserve', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 26)
        self.assertTrue(r.base.same(query_tree, decoded_tree))
        self.assertEqual(term.sort, 'DPObservation')
        self.assertNotIn('dpObserve', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 26)
        self.assertEqual(len(r.DATA_REG), 26)
        self.assertEqual(len(r.LABELS), 27)

    def test_residual_functions_are_not_output_constructors(self):
        self.assertIn('dpObserve', r.LABELS)
        self.assertNotIn('dpObserve', r.DATA_REG)
        self.assertNotIn('dpObservePacked', r.DATA_REG)
        self.assertNotIn('dpObservePacked', r.LABELS)
        with self.assertRaises(ValueError):
            r.from_tree({'sort': 'DPObservation',
                         'constructor': 'dpObservePacked', 'arguments': []})
        with self.assertRaises(ValueError):
            r.from_tree({'sort': 'DPObservation',
                         'constructor': 'dpObserve', 'arguments': []})

    def test_padded_lanes_retain_inactive_zeroes_without_compaction(self):
        actual = p.observation_document(synthetic_observation_tree())
        self.assertEqual(len(actual['padded_lanes']), 14)
        self.assertEqual(actual['padded_lanes'][7:], [0, 5, 0, 0, 0, 0, 0])
        self.assertEqual(actual['counts'], [8, 4, 3, 2])

    def test_field_and_case_metadata_bind_only_outside_result(self):
        carrier = p.port_input(copy.deepcopy(self.input))
        self.assertEqual(carrier['profile'], self.input['profile'])
        self.assertEqual(carrier['case_id'], self.input['caseId'])
        result = p.observation_document(synthetic_observation_tree())
        self.assertEqual(set(result),
                         {'geometry', 'counts', 'padded_lanes', 'decoded_tables'})


class DataPortTests(unittest.TestCase):
    def setUp(self):
        self.input = json.loads((ROOT / 'SAFE/SAFE-INPUT-ONLY.proposed.json').read_bytes())

    def test_exact_shared_input_retained_and_all_seven_variants(self):
        original = copy.deepcopy(self.input)
        carrier = p.port_input(self.input)
        self.assertEqual(carrier['source_document'], original)
        self.assertEqual(self.input, original)
        tables = carrier['query']['arguments'][1]
        self.assertEqual(p.tables_document(tables), original['tables'])
        self.assertEqual([next(iter(x)) for x in p.tables_document(tables)['data']],
                         ['U128', 'Bool', 'Nil', 'Cons', 'Leaf', 'Branch', 'Constructor'])

    def test_full_uint128_and_native_bool_retained(self):
        rows = self.input['tables']['data']
        uint = p.data_node(rows[0])
        boolean = p.data_node(rows[1])
        self.assertEqual(uint['arguments'][0], {'sort': 'Int', 'value': rows[0]['U128']['n']})
        self.assertEqual(rows[0]['U128']['n'].bit_length(), 128)
        self.assertEqual(boolean['arguments'][0], {'sort': 'Bool', 'value': True})
        self.assertIs(type(boolean['arguments'][0]['value']), bool)

    def test_raw_empty_slice_start_retained(self):
        row = self.input['tables']['data'][5]
        self.assertEqual(p.node_document(p.data_node(row)), row)
        self.assertEqual(p.data_node(row)['arguments'][0]['arguments'],
                         [{'sort': 'Int', 'value': 4}, {'sort': 'Int', 'value': 0}])

    def test_option_zero_and_reference_kinds_stay_distinct(self):
        absent = p.environment({'None': None})
        present = p.environment({'Some': 0})
        self.assertNotEqual(absent, present)
        self.assertEqual(p.environment_document(absent), {'None': None})
        self.assertEqual(p.environment_document(present), {'Some': 0})
        self.assertNotEqual(p.reference({'Const': 0}), p.reference({'Data': 0}))

    def test_capacity_valid_references_beyond_active_are_not_looked_up(self):
        document = p.tables_document(p.tables(self.input['tables']))
        self.assertEqual(document['data_fields'][1], {'Data': 7})
        self.assertEqual(document['env'][2]['previous'], {'Some': 3})
        self.assertEqual(document['value_slots'][0], {'Data': 7})
        self.assertEqual(len(document['data']), 7)
        self.assertEqual(len(document['env']), 3)

    def test_caps_order_and_exact_four_table_fields(self):
        query = p.port_input(self.input)['query']
        self.assertEqual(query['constructor'], 'dpObserve')
        self.assertEqual([x['value'] for x in query['arguments'][0]['arguments']],
                         [self.input['caps'][key] for key in p.CAP_FIELDS])
        self.assertEqual(list(p.tables_document(query['arguments'][1])),
                         ['data', 'data_fields', 'env', 'value_slots'])


if __name__ == '__main__':
    unittest.main()
