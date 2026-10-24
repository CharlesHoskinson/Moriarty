"""Positive field transport/extraction only. UNRUN; no K semantics oracle."""
import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'source'))
import context_ports as p
import context_runtime as r


def lanes(values):
    result = p.term('CTLanes', 'ctNoLanes')
    for value in reversed(values):
        result = p.term('CTLanes', 'ctLane', p.atom('Int', value), result)
    return result


def synthetic_observation(context):
    # Arbitrary typed extraction fixture, NOT canonical cxPack/cap derivation.
    return p.term('CXObservation', 'cxObservation', lanes(list(range(22))),
                  p.payload(context),
                  p.record('CXExtFields', 'cxExt', p.EXT_FIELDS,
                           {name: index for index, name in enumerate(p.EXT_FIELDS)}),
                  p.term('CXStateView', 'cxStateView',
                         p.term('CXStateCaps', 'cxStateCaps',
                                p.record('CXPhysCaps', 'cxPhysCaps', p.PHYS_FIELDS,
                                         {name: index for index, name in enumerate(p.PHYS_FIELDS)}),
                                p.atom('Int', 71), p.atom('Int', 73)),
                         p.atom('Bool', False), p.term('CXStateLimits', 'cxStateLimits', p.atom('Int', 79))),
                  lanes(list(range(29))))


class ContextPortTests(unittest.TestCase):
    def setUp(self):
        self.source = json.loads((ROOT / 'SAFE/SAFE-INPUT-ONLY.json').read_text())

    def test_complete_original_cases_inverse(self):
        ported = p.port_input(self.source)
        self.assertEqual(p.query_cases_document(ported['query']), self.source['cases'])
        self.assertEqual(ported['source_document'], self.source)
        self.assertEqual(ported['case_ids'], [case['case_id'] for case in self.source['cases']])

    def test_source_sidecar_is_deep_copy(self):
        ported = p.port_input(self.source)
        ported['source_document']['cases'][0]['input']['bounds']['step_cap'] = 0
        self.assertNotEqual(ported['source_document'], self.source)
        self.assertEqual(self.source['cases'][0]['input']['bounds']['step_cap'], 127)

    def test_original_absent_and_present_zero_distinct(self):
        absent, present = [case['input']['context'] for case in self.source['cases'][:2]]
        self.assertNotEqual(p.payload(absent), p.payload(present))
        self.assertEqual(p.payload_document(p.payload(absent)), absent)
        self.assertEqual(p.payload_document(p.payload(present)), present)
        self.assertEqual(p.payload(absent)['arguments'][0]['constructor'], 'cxAbsent')
        self.assertEqual(p.payload(present)['arguments'][0]['constructor'], 'cxPresent')

    def test_raw_lifetime_not_narrowed(self):
        for case in self.source['cases'][2:]:
            value = case['input']['context']
            result = p.payload_document(p.payload(value))
            self.assertEqual(result['ext']['C6CtxPresent']['max_lifetime_work'],
                             value['ext']['C6CtxPresent']['max_lifetime_work'])
            self.assertGreater(result['ext']['C6CtxPresent']['max_lifetime_work'], 2**64 - 1)

    def test_full_input_unobserved_fields_are_retained(self):
        for case in self.source['cases']:
            original = case['input']
            restored = p.input_document(p.input_value(original))
            self.assertEqual(restored, original)
            self.assertEqual(len(restored['bounds']), 22)
            self.assertEqual(len(restored['limits']), 5)
            self.assertEqual(restored['bounds']['max_helper_work'], original['bounds']['max_helper_work'])
            self.assertEqual(restored['limits']['max_total_lanes'], original['limits']['max_total_lanes'])

    def test_complete_synthetic_projection_no_lane_computation(self):
        context = self.source['cases'][3]['input']['context']
        result = p.observation_document(synthetic_observation(context))
        self.assertEqual(list(result), ['encoded_lanes', 'decoded_context', 'effective_ext',
                                        'state_context', 'state_descriptor'])
        self.assertEqual(result['encoded_lanes'], list(range(22)))
        self.assertEqual(result['decoded_context'], context)
        self.assertEqual(list(result['effective_ext'].values()), list(range(16)))
        self.assertEqual(list(result['state_context']['caps']['phys'].values()), list(range(27)))
        self.assertEqual(result['state_context']['caps']['max_data_field_slots'], 71)
        self.assertEqual(result['state_context']['caps']['max_helper_tasks'], 73)
        self.assertIs(result['state_context']['ext_present'], False)
        self.assertIs(type(result['state_context']['caps']['phys']['max_frames']), int)
        self.assertEqual(result['state_context']['limits'], {'max_total_lanes': 79})
        self.assertEqual(result['state_descriptor'], list(range(29)))

    def test_whole_ordered_wire_wrapper_from_public_identity(self):
        observations = p.term('CXObservations', 'cxNoObservations')
        for case in reversed(self.source['cases']):
            observations = p.term('CXObservations', 'cxObservationCons',
                                  synthetic_observation(case['input']['context']), observations)
        result = p.observations_document(observations, self.source)
        self.assertEqual(list(result), ['schema', 'profile', 'case_ids', 'observations'])
        self.assertEqual(result['schema'], 'independent-c6-context-positive/1')
        self.assertEqual(result['profile'], self.source['profile'])
        self.assertEqual(result['case_ids'], [case['case_id'] for case in self.source['cases']])
        self.assertEqual([value['decoded_context'] for value in result['observations']],
                         [case['input']['context'] for case in self.source['cases']])


def full_synthetic_tree(source):
    observations = p.term('CXObservations', 'cxNoObservations')
    for case in reversed(source['cases']):
        observations = p.term('CXObservations', 'cxObservationCons',
                              synthetic_observation(case['input']['context']), observations)
    return observations


class ContextRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.source = json.loads((ROOT / 'SAFE/SAFE-INPUT-ONLY.json').read_text())

    def test_full_carrier_fields_order_lanes_caps_descriptor(self):
        tree = full_synthetic_tree(self.source)
        doc = p.observations_document(tree, self.source)
        self.assertEqual(list(doc), ['schema', 'profile', 'case_ids', 'observations'])
        self.assertEqual(doc['case_ids'], ['absent', 'present-all-zero',
                                           'present-contrasting-wide-lifetime', 'present-u128-lifetime-limit'])
        self.assertEqual(len(doc['observations']), 4)
        for obs in doc['observations']:
            self.assertEqual(list(obs), ['encoded_lanes', 'decoded_context', 'effective_ext',
                                         'state_context', 'state_descriptor'])
            self.assertEqual(len(obs['encoded_lanes']), 22)
            self.assertEqual(len(obs['effective_ext']), 16)
            self.assertEqual(len(obs['state_context']['caps']['phys']), 27)
            self.assertEqual(len(obs['state_descriptor']), 29)
        self.assertEqual(doc['observations'][2]['decoded_context']['ext']['C6CtxPresent']['max_lifetime_work'],
                         18446744073709551616)
        self.assertEqual(doc['observations'][3]['decoded_context']['ext']['C6CtxPresent']['max_lifetime_work'],
                         340282366920938463463374607431768211455)

    def test_absent_present_zero_and_native_bool_int(self):
        tree = full_synthetic_tree(self.source)
        doc = p.observations_document(tree, self.source)
        self.assertEqual(doc['observations'][0]['decoded_context']['ext'], 'C6CtxAbsent')
        self.assertNotEqual(doc['observations'][0]['decoded_context'],
                            doc['observations'][1]['decoded_context'])
        wire = r.tree_to_wire(tree)
        self.assertTrue(r.same_whole(r.wire_to_tree(wire), tree))
        found = []
        stack = [wire]
        while stack:
            node = stack.pop()
            if type(node) is dict and set(node) == {'sort', 'literal'}:
                found.append(node)
            elif type(node) is dict:
                stack.extend(node['arguments'])
        self.assertTrue(any(n['sort'] == 'Bool' and n['literal'] is False for n in found))
        self.assertTrue(any(n['sort'] == 'Int' and type(n['literal']) is int for n in found))
        self.assertFalse(r.same_whole({'sort': 'Bool', 'literal': True}, {'sort': 'Int', 'literal': 1}))

    def test_ast_wire_value_literal_whole_lossless_identity(self):
        tree = p.port_input(copy.deepcopy(self.source))['query']
        wire = r.tree_to_wire(tree)
        self.assertIn({'sort': 'String', 'literal': 'absent'}, self._prims(wire))
        self.assertEqual(r.wire_to_tree(wire), tree)
        self.assertEqual(r.tree_to_wire(r.wire_to_tree(wire)), wire)
        with self.assertRaises(ValueError):
            r.tree_to_wire({'sort': 'Int', 'value': 1, 'literal': 1})
        with self.assertRaises(ValueError):
            r.wire_to_tree({'sort': 'Int', 'value': 1})

    def _prims(self, node):
        out, stack = [], [node]
        while stack:
            x = stack.pop()
            if type(x) is dict and set(x) == {'sort', 'literal'}:
                out.append(x)
            elif type(x) is dict:
                stack.extend(x['arguments'])
        return out

    def test_query_inverse_registry_restored_after_carrier(self):
        term, wire, ported = r.carrier(copy.deepcopy(self.source))
        self.assertEqual(ported['profile'], 'sherlock-c6-context-positive/1')
        self.assertEqual(ported['case_ids'], [c['case_id'] for c in self.source['cases']])
        self.assertNotIn('cxObserveAll', r.base.codec.REG)
        self.assertEqual(len(r.base.codec.REG), 20)
        saved = r.base.codec.REG
        try:
            r.base.codec.REG = {**r.DATA_REG, **r.ENTRY_REG}
            self.assertEqual(len(r.base.codec.REG), 21)
            query_tree = r.base.tree(term)
            decoded_tree = r.base.tree(r.decode_document(wire, 'CXObservations', allow_string=True))
        finally:
            r.base.codec.REG = saved
        self.assertIs(r.base.codec.REG, saved)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        self.assertTrue(r.base.same(query_tree, decoded_tree))
        self.assertEqual(term.sort, 'CXObservations')

    def test_residual_functions_refuse_as_output(self):
        for name in ('cxObserve', 'cxObserveAll', 'cxPack', 'cxUnpack', 'cxDerive'):
            self.assertNotIn(name, r.DATA_REG)
            with self.assertRaises(ValueError):
                r.from_tree({'sort': 'CXObservations', 'constructor': name, 'arguments': []})
        self.assertIn('cxObserveAll', r.LABELS)
        self.assertEqual(len(r.LABELS), 21)
        self.assertEqual(len(r.DATA_REG), 20)
        string_token = {'node': 'KToken', 'sort': {'node': 'KSort', 'name': 'String', 'params': []}, 'token': '"x"'}
        with self.assertRaises(ValueError):
            r.decode_node(string_token, 'String', strict_output=True)

    def test_compare_fake_operands_both_roles_and_barrier(self):
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            def put(name, value):
                path = root / name
                path.write_text(json.dumps(value) + '\n')
                return r.base.pin(path)
            common_a, ast_a = put('ca.json', {'x': 1}), put('aa.json', {'sort': 'CXObservations'})
            common_e, ast_e = put('ce.json', {'x': 1}), put('ae.json', {'sort': 'CXObservations'})
            barrier = put('barrier.json', {'expected_body_reads': 0,
                                           'expected_body_read_scope': r.COMPARE_SCOPE,
                                           'members': [common_a, ast_a]})
            manifest = put('manifest.json', {'commonJSON': {'actual': common_a, 'expected': common_e},
                                             'typedAST': {'actual': ast_a, 'expected': ast_e},
                                             'durable_freeze': barrier})
            with mock.patch.object(r, 'EXPECTED_COMMON_PIN', common_e), mock.patch.object(r, 'EXPECTED_AST_PIN', ast_e):
                result = r.compare_manifest(r.base.load(manifest['path']))
                self.assertTrue(result['whole_typed_equal'])
                self.assertEqual(result['expected_body_reads'], 2)
                self.assertTrue(result['commonJSON']['whole_typed_equal'])
                self.assertTrue(result['typedAST']['whole_typed_equal'])
            common_e_bad = put('ce_bad.json', {'x': 2})
            manifest_bad = put('manifest_bad.json', {'commonJSON': {'actual': common_a, 'expected': common_e_bad},
                                                     'typedAST': {'actual': ast_a, 'expected': ast_e},
                                                     'durable_freeze': barrier})
            with mock.patch.object(r, 'EXPECTED_COMMON_PIN', common_e_bad), mock.patch.object(r, 'EXPECTED_AST_PIN', ast_e):
                result2 = r.compare_manifest(r.base.load(manifest_bad['path']))
                self.assertFalse(result2['whole_typed_equal'])
                self.assertEqual(result2['expected_body_reads'], 2)
                self.assertTrue(result2['commonJSON']['whole_typed_equal'] is False)
                self.assertTrue(result2['typedAST']['whole_typed_equal'])
            barrier2 = put('barrier_missing.json', {'expected_body_reads': 0,
                                                    'expected_body_read_scope': r.COMPARE_SCOPE,
                                                    'members': [common_a]})
            manifest2 = put('manifest_missing_barrier.json', {'commonJSON': {'actual': common_a, 'expected': common_e},
                                                              'typedAST': {'actual': ast_a, 'expected': ast_e},
                                                              'durable_freeze': barrier2})
            with mock.patch.object(r, 'EXPECTED_COMMON_PIN', common_e), mock.patch.object(r, 'EXPECTED_AST_PIN', ast_e):
                with self.assertRaises(ValueError):
                    r.compare_manifest(r.base.load(manifest2['path']))


if __name__ == '__main__':
    unittest.main()
