"""Synthetic format expectations, authored before adapter implementation.

No private oracle values or captured actual values; not a semantic oracle.
"""
import copy
import unittest

from program_snapshot_normalizer import FormatError, canonical_json, loads_strict
from expected_snapshot_adapter import normalize_expected_snapshot


EMPTY = {
    'entry': 0, 'expressions': [], 'occurrences': [], 'occurrence_paths': [],
    'children': [], 'functions': [], 'parameters': [], 'arms': [], 'patterns': [],
    'pattern_children': [], 'types': [], 'type_args': [], 'constructors': [],
    'ctor_fields': [], 'decl_param_counts': [], 'decl_ctor_counts': [],
    'decl_templates': [], 'plan_nodes': [], 'plan_cases': [], 'field_paths': [],
    'constants': [], 'constant_fields': [], 'names': [],
}
MAX128 = 340282366920938463463374607431768211455


class ExpectedRepresentationTests(unittest.TestCase):
    def test_zero_and_max_exact_integer_to_decimal_string_only(self):
        before = copy.deepcopy(EMPTY)
        before['constants'] = [
            {'U128': {'marker_free': True, 'n': 0}},
            {'U128': {'marker_free': False, 'n': MAX128}},
            {'U128': {'marker_free': True, 'n': 0}},
        ]
        expected = copy.deepcopy(EMPTY)
        expected['constants'] = [
            {'U128': {'marker_free': True, 'n': '0'}},
            {'U128': {'marker_free': False, 'n': '340282366920938463463374607431768211455'}},
            {'U128': {'marker_free': True, 'n': '0'}},
        ]
        original = copy.deepcopy(before)
        result = normalize_expected_snapshot(before)
        self.assertEqual(canonical_json(result), canonical_json(expected))
        self.assertEqual(canonical_json(before), canonical_json(original))
        self.assertEqual(len(result), 23)
        self.assertIsNot(result['constants'], before['constants'])
        self.assertIs(type(result['constants'][1]['U128']['n']), str)

    def test_other_numeric_fields_names_options_padding_and_boolean_kinds_unchanged(self):
        before = copy.deepcopy(EMPTY)
        before.update({
            'entry': 7, 'children': [2, 0, 2], 'names': ['007', 'U128', 'n', '007'],
            'type_args': [0, 1, 2, 3, 7, 0], 'plan_cases': [None, 0, None, 2],
            'functions': [{'function_name': 7, 'params': {'start': 2, 'len': 0}, 'return_type': 2, 'body': 0}],
            'constants': [{'Bool': {'marker_free': False, 'b': True}},
                          {'Nil': {'marker_free': True}},
                          {'U128OutOfRange': {'marker_free': False}},
                          {'Constructor': {'marker_free': True, 'type_id': 7, 'ctor_id': 2,
                                           'fields': {'start': 2, 'len': 0}}}],
            'decl_templates': [{'name': 'U128', 'parameters': ['007'], 'ctors': [
                {'name': 'C', 'fields': [{'name': 'n', 'ty': {'Named': {'name': '007'}}}]}]}],
        })
        result = normalize_expected_snapshot(before)
        self.assertEqual(canonical_json(result), canonical_json(before))
        self.assertIs(type(result['entry']), int)
        self.assertIs(type(result['constants'][0]['Bool']['b']), bool)
        self.assertIs(type(result['functions'][0]['return_type']), int)

    def test_u128_wrong_kinds_and_out_of_range_refused(self):
        for bad in (True, False, -1, MAX128 + 1, '0', '7', None, 0.0, 1.0, [], {}):
            obj = copy.deepcopy(EMPTY)
            obj['constants'] = [{'U128': {'marker_free': True, 'n': bad}}]
            with self.subTest(bad=bad), self.assertRaises(FormatError):
                normalize_expected_snapshot(obj)

    def test_strict_complete_schema_not_global_number_or_key_coercion(self):
        for field, bad in (('entry', True), ('entry', '7'), ('entry', -1),
                           ('type_args', ['UInt']), ('type_args', [True]),
                           ('children', ['2']), ('names', [7]),
                           ('expressions', [{'U128': {'marker_free': True, 'n': 7}}]),
                           ('constants', [{'U128': {'marker_free': 1, 'n': 7}}]),
                           ('constants', [{'U128': {'marker_free': True, 'n': 7, 'extra': 0}}]),
                           ('constants', [{'U128': {'n': 7}}]),
                           ('constants', ['U128']),
                           ('patterns', [{'PNil': None}]),
                           ('plan_cases', [{'Some': 0}])):
            obj = copy.deepcopy(EMPTY)
            obj[field] = bad
            with self.subTest(field=field, bad=bad), self.assertRaises(FormatError):
                normalize_expected_snapshot(obj)
        for key in EMPTY:
            obj = copy.deepcopy(EMPTY)
            del obj[key]
            with self.subTest(missing=key), self.assertRaises(FormatError):
                normalize_expected_snapshot(obj)
        obj = copy.deepcopy(EMPTY)
        obj['extra'] = 0
        with self.assertRaises(FormatError):
            normalize_expected_snapshot(obj)

    def test_strict_decode_precedes_adapter(self):
        for text in ('{"n":1,"n":2}', '{"n":1e3}', '{"n":NaN}'):
            with self.subTest(text=text), self.assertRaises(FormatError):
                loads_strict(text)
        exact = loads_strict('{"n":340282366920938463463374607431768211455}')
        self.assertIs(type(exact['n']), int)
        self.assertEqual(exact['n'], MAX128)


if __name__ == '__main__':
    unittest.main()
