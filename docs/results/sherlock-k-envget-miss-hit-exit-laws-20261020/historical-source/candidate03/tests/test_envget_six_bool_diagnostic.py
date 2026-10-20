"""Synthetic transport tests for envget_six_bool_diagnostic.

These tests check only query transport, fixture shape, and Bool result
classification. They never execute K and never claim any of the six
K laws passed. No genuine actual-before, admitted image, expected
afterstate, or golden is used.
"""
import copy
import sys
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve()
for _candidate in (_HERE.parent, _HERE.parents[1] / 'source', _HERE.parent / 'source'):
    if (_candidate / 'envget_six_bool_diagnostic.py').exists():
        sys.path.insert(0, str(_candidate))
        break
for _candidate in (_HERE.parent, _HERE.parents[1] / 'source', _HERE.parent / 'source'):
    if (_candidate / 'envget_runtime.py').exists():
        sys.path.insert(0, str(_candidate))
        break

import envget_ports as p
import envget_runtime as r
import envget_six_bool_diagnostic as d


EXPECTED_LABELS = {
    'egMissPreservesWholeCarrier':
        'egMissPreservesWholeCarrier(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
    'egMissPreviousNoneRemainsPending':
        'egMissPreviousNoneRemainsPending(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
    'egMissNonzeroToSomeZero':
        'egMissNonzeroToSomeZero(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
    'egMissEventCursorAndAccounting':
        'egMissEventCursorAndAccounting(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
    'egTraversalLinked':
        'egTraversalLinked(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGTraversalWindow',
    'egMissOffChainPreviousDiscriminator':
        'egMissOffChainPreviousDiscriminator(_)_SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS_Bool_EGSnapshot',
}


class SixBoolDiagnosticSeeds(unittest.TestCase):
    def test_S_shape_offchain_rows_and_inverse_23(self):
        x = d.synthetic_S()
        self.assertTrue(d.DIAGNOSTIC_IDENTITY.startswith('PUBLIC UNADMITTED'))
        self.assertEqual(len(x['image']), 23)
        helper = x['image']['helper']['Some']['EnvGet']
        self.assertEqual(helper['binder'], 31)
        self.assertEqual(helper['cursor'], {'Some': 2})
        self.assertIsNone(helper['found'])
        self.assertEqual(helper['status'], 'Pending')
        self.assertEqual(x['image']['env'], [
            {'binder': 31, 'value': {'Const': 0}, 'previous': None},
            {'binder': 31, 'value': {'Data': 1}, 'previous': None},
            {'binder': 47, 'value': {'Const': 2}, 'previous': {'Some': 0}}])
        self.assertEqual(p.to_snapshot(p.from_snapshot(x)), x)

    def test_S_none_is_distinct_previous_None_variant(self):
        s = d.synthetic_S()
        none = d.synthetic_S_none()
        self.assertEqual(s['image']['env'][2]['previous'], {'Some': 0})
        self.assertIsNone(none['image']['env'][2]['previous'])
        expected = copy.deepcopy(s)
        expected['image']['env'][2]['previous'] = None
        self.assertEqual(none, expected)
        # Some(0) remains a present cursor, distinct from both Nones.
        self.assertIsNotNone(s['image']['env'][2]['previous'])
        self.assertEqual(p.to_snapshot(p.from_snapshot(none)), none)

    def test_S_event_occurrence_slice_complete_and_context_retained(self):
        s = d.synthetic_S()
        event = d.synthetic_S_event()
        view = event['admission']['view']
        self.assertEqual(view['unused'], [])
        self.assertEqual(view['occurrence_paths'], [0, 3])
        self.assertEqual(len(view['occurrences']), 4)
        origin = event['image']['control_origin']
        self.assertEqual(origin, 3)
        row = view['occurrences'][origin]
        self.assertIs(type(row['owner_is_function']), bool)
        self.assertIs(type(row['function']), int)
        self.assertIs(type(row['path']['start']), int)
        self.assertIs(type(row['path']['len']), int)
        start, length = row['path']['start'], row['path']['len']
        self.assertGreaterEqual(start, 0)
        self.assertGreaterEqual(length, 0)
        self.assertLessEqual(start + length, len(view['occurrence_paths']))
        self.assertEqual(view['occurrence_paths'][start:start + length], [0, 3])
        stripped = copy.deepcopy(event)
        stripped['admission']['view'] = {'unused': []}
        self.assertEqual(stripped, s)
        self.assertEqual(p.to_snapshot(p.from_snapshot(event)), event)

    def test_builders_return_fresh_and_do_not_mutate_original_seed(self):
        s = d.synthetic_S()
        saved = copy.deepcopy(s)
        none = d.synthetic_S_none()
        event = d.synthetic_S_event()
        none['image']['env'][0]['binder'] = 99
        event['admission']['view']['occurrences'][3]['function'] = 99
        self.assertEqual(s, saved)
        self.assertEqual(d.synthetic_S(), saved)


class SixBoolDiagnosticQueries(unittest.TestCase):
    def test_closed_order_variants_and_exact_labels(self):
        self.assertEqual([name for name, _ in d.QUERY_ORDER], [
            'egMissPreservesWholeCarrier',
            'egMissPreviousNoneRemainsPending',
            'egMissNonzeroToSomeZero',
            'egMissEventCursorAndAccounting',
            'egTraversalLinked',
            'egMissOffChainPreviousDiscriminator'])
        self.assertEqual([variant for _, variant in d.QUERY_ORDER],
                         ['S', 'S_none', 'S', 'S_event', 'S_event', 'S'])
        for name, label in EXPECTED_LABELS.items():
            self.assertEqual(r.LABELS[name], label)
            self.assertEqual(d.LAW_BY_NAME[name]['compiled_klabel'], label)
            self.assertEqual(r.REG_ALL[name][0], 'Bool')
            self.assertEqual(len(r.REG_ALL[name][1]), 1)

    def test_all_six_typed_roots_and_single_nested_position(self):
        for law_name, variant in d.QUERY_ORDER:
            native = d.input_for_variant(variant)
            term, envelope, wire = d.build_query_term(law_name, native)
            self.assertEqual(term.name, law_name)
            self.assertEqual(term.sort, 'Bool')
            self.assertEqual(envelope['format'], 'KAST')
            self.assertEqual(envelope['version'], 4)
            self.assertEqual(
                envelope['term']['label']['name'],
                EXPECTED_LABELS[law_name])
            self.assertIn(law_name + '(', r.LABELS[law_name])
            if law_name == 'egTraversalLinked':
                self.assertEqual(term.args[0].name, 'egThreeEventTraversal')
                self.assertEqual(term.args[0].sort, 'EGTraversalWindow')
                self.assertEqual(term.args[0].args[0].sort, 'EGSnapshot')
            else:
                self.assertEqual(term.args[0].sort, 'EGSnapshot')

    def test_nested_traversal_refused_in_wrong_positions(self):
        native = d.synthetic_S_event()
        bad = p.node('egMissPreservesWholeCarrier',
                     p.node('egThreeEventTraversal', p.from_snapshot(native)))
        with self.assertRaises(ValueError):
            d.validate_query_wire(bad)
        bad_root = p.node('egThreeEventTraversal', p.from_snapshot(native))
        with self.assertRaises(ValueError):
            d.validate_query_wire(bad_root)
        bad_function = p.node('egCellMiss', p.from_snapshot(native))
        with self.assertRaises(ValueError):
            d.validate_query_wire(bad_function)
        double_nested = p.node(
            'egTraversalLinked',
            p.node('egThreeEventTraversal',
                   p.node('egThreeEventTraversal', p.from_snapshot(native))))
        with self.assertRaises(ValueError):
            d.validate_query_wire(double_nested)

    def test_rejected_construction_restores_registry_and_before_pin(self):
        saved_reg = dict(r.base.codec.REG)
        saved_before = r.BEFORE_PIN
        with self.assertRaises(ValueError):
            d.build_query_term('egCellMiss', d.synthetic_S())
        with self.assertRaises(ValueError):
            d.validate_query_wire(
                p.node('egMissPreservesWholeCarrier',
                       p.node('egThreeEventTraversal',
                              p.from_snapshot(d.synthetic_S()))))
        # ports.from_snapshot first reads snapshot['image']; the supplied
        # {'not': 'a snapshot'} therefore raises the real KeyError for the
        # missing snapshot shape, before any finite validator is reached.
        with self.assertRaises(KeyError):
            d.build_query_wire('egMissPreservesWholeCarrier',
                               {'not': 'a snapshot'})
        self.assertEqual(r.base.codec.REG, saved_reg)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        self.assertNotIn('egThreeEventTraversal', r.base.codec.REG)
        self.assertIs(r.BEFORE_PIN, saved_before)
        self.assertIsNone(r.BEFORE_PIN)


class SixBoolDiagnosticRootCarrier(unittest.TestCase):
    def test_wire_law_roots_are_bool_constructors_not_literals(self):
        for law_name, variant in d.QUERY_ORDER:
            native = d.input_for_variant(variant)
            wire = d.build_query_wire(law_name, native)
            self.assertEqual(set(wire), {'sort', 'constructor', 'arguments'})
            self.assertEqual(wire['sort'], 'Bool')
            self.assertEqual(wire['constructor'], law_name)
            self.assertNotIn('literal', wire)
            internal = d._wire_to_internal_diag(wire)
            self.assertEqual(set(internal), {'sort', 'constructor', 'arguments'})
            self.assertEqual(internal['sort'], 'Bool')
            self.assertEqual(internal['constructor'], law_name)
            self.assertNotIn('value', internal)
            self.assertEqual(d._internal_to_wire_diag(internal), wire)
            if law_name == 'egTraversalLinked':
                nested = internal['arguments'][0]
                self.assertEqual(
                    set(nested), {'sort', 'constructor', 'arguments'})
                self.assertEqual(nested['constructor'], 'egThreeEventTraversal')
                self.assertEqual(nested['sort'], 'EGTraversalWindow')
                self.assertEqual(nested['arguments'][0]['sort'], 'EGSnapshot')
            else:
                self.assertEqual(internal['arguments'][0]['sort'], 'EGSnapshot')

    def test_internal_bool_law_root_is_codec_constructor_not_bool_leaf(self):
        for law_name, variant in d.QUERY_ORDER:
            native = d.input_for_variant(variant)
            wire = d.build_query_wire(law_name, native)
            internal = d._wire_to_internal_diag(wire)
            with r._codec_reg(dict(r.ALL_REG)):
                term = d._from_tree_diag(internal, law_name, True)
                leaf = d._from_tree_diag(
                    {'sort': 'Bool', 'value': True}, law_name, False)
            self.assertEqual(term.name, law_name)
            self.assertEqual(term.sort, 'Bool')
            self.assertNotEqual(term.name, '@Bool')
            self.assertEqual(leaf.name, '@Bool')
            self.assertEqual(leaf.sort, 'Bool')
            self.assertIs(leaf.args[0], True)
            built_term, envelope, built_wire = d.build_query_term(
                law_name, native)
            self.assertEqual(built_term.name, law_name)
            self.assertEqual(built_term.sort, 'Bool')
            self.assertEqual(built_wire, wire)
            self.assertEqual(envelope['term']['node'], 'KApply')

    def test_bool_literal_leaf_crosses_all_carriers_and_kind_confusion_refused(self):
        law_name = 'egMissPreservesWholeCarrier'
        for value, token in ((True, 'true'), (False, 'false')):
            wire = {'sort': 'Bool', 'literal': value}
            internal = d._wire_to_internal_diag(wire)
            self.assertEqual(internal, {'sort': 'Bool', 'value': value})
            self.assertEqual(d._internal_to_wire_diag(internal), wire)
            with r._codec_reg(dict(r.ALL_REG)):
                term = d._from_tree_diag(internal, law_name, False)
            self.assertEqual(term.name, '@Bool')
            self.assertEqual(term.sort, 'Bool')
            self.assertIs(term.args[0], value)
            node = d._encode_node_diag(term, law_name, False)
            self.assertEqual(node, {
                'node': 'KToken',
                'sort': {'node': 'KSort', 'name': 'Bool', 'params': []},
                'token': token})
            with r._codec_reg(dict(r.ALL_REG)):
                decoded = d._decode_node_diag(node, 'Bool', law_name, False)
            self.assertEqual(decoded.name, '@Bool')
            self.assertIs(decoded.args[0], value)
        for bad_wire in (
                {'sort': 'Bool', 'literal': 1},
                {'sort': 'Bool', 'literal': 0},
                {'sort': 'Int', 'literal': True},
                {'sort': 'Bool', 'value': True},
                {'sort': 'Bool', 'literal': True,
                 'constructor': law_name, 'arguments': []}):
            with self.assertRaises(ValueError):
                d._wire_to_internal_diag(bad_wire)
        for bad_internal in (
                {'sort': 'Bool', 'value': 1},
                {'sort': 'Int', 'value': True}):
            with self.assertRaises(ValueError):
                with r._codec_reg(dict(r.ALL_REG)):
                    d._from_tree_diag(bad_internal, law_name, False)

    def test_unknown_bool_constructor_and_misplaced_traversal_still_refused(self):
        saved_reg = dict(r.base.codec.REG)
        saved_before = r.BEFORE_PIN
        native = d.synthetic_S_event()
        snapshot = p.from_snapshot(native)
        unknown_wire = {
            'sort': 'Bool',
            'constructor': 'egUndeclaredBoolLaw',
            'arguments': [snapshot]}
        with self.assertRaises(ValueError):
            d.validate_query_wire(unknown_wire)
        unknown_internal = d._wire_to_internal_diag(unknown_wire)
        self.assertEqual(unknown_internal['constructor'], 'egUndeclaredBoolLaw')
        with self.assertRaises(ValueError):
            with r._codec_reg(dict(r.ALL_REG)):
                d._from_tree_diag(
                    unknown_internal, 'egUndeclaredBoolLaw', True)
        bad_wire = p.node(
            'egMissPreservesWholeCarrier',
            p.node('egThreeEventTraversal', snapshot))
        with self.assertRaises(ValueError):
            d.validate_query_wire(bad_wire)
        bad_internal = d._wire_to_internal_diag(bad_wire)
        with self.assertRaises(ValueError):
            with r._codec_reg(dict(r.ALL_REG)):
                d._from_tree_diag(
                    bad_internal, 'egMissPreservesWholeCarrier', True)
        self.assertEqual(r.base.codec.REG, saved_reg)
        self.assertEqual(r.base.codec.REG, r.DATA_REG)
        self.assertIs(r.BEFORE_PIN, saved_before)
        self.assertIsNone(r.BEFORE_PIN)


class SixBoolDiagnosticBoolDecode(unittest.TestCase):
    def test_true_pass_and_false_semantic_failure_are_distinct(self):
        passed = d.classify_bool_print('egMissPreservesWholeCarrier', 'true')
        failed = d.classify_bool_print('egMissPreservesWholeCarrier', 'false')
        self.assertEqual(passed['status'], 'LAW_PASS')
        self.assertIs(passed['bool_value'], True)
        self.assertEqual(failed['status'], 'LAW_FAIL')
        self.assertIs(failed['bool_value'], False)
        wrapped = d.classify_bool_print(
            'egTraversalLinked', '<generatedTop><k> true ~> .K </k></generatedTop>')
        self.assertEqual(wrapped['status'], 'LAW_PASS')

    def test_bool_vs_int_residual_trailing_and_multiple_rejected(self):
        law = 'egMissNonzeroToSomeZero'
        for text in ('1', '0', 'egMissNonzeroToSomeZero(true)',
                     'true false', 'true true', '"true"', ''):
            result = d.classify_bool_print(law, text)
            self.assertEqual(result['status'], 'FAIL_BOOL_PRINT_PARSE', text)
            self.assertIsNone(result['bool_value'], text)
        with self.assertRaises(ValueError):
            d.classify_bool_print('egNotAClosedLaw', 'true')


if __name__ == '__main__':
    unittest.main()
