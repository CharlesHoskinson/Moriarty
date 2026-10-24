"""Author's source-derived format cases, not an independent semantic oracle.

Authored before normalizer implementation; all execution remains UNRUN.
No actual capture, private expected snapshot, evaluator or filesystem fixtures.
"""
import copy
import json
import unittest

from program_snapshot_normalizer import FormatError, loads_strict, normalize_program_view


EMPTY = {
    'entry': '0', 'expressions': [], 'occurrences': [], 'occurrence_paths': [],
    'children': [], 'functions': [], 'parameters': [], 'arms': [], 'patterns': [],
    'pattern_children': [], 'types': [], 'type_args': [], 'constructors': [],
    'ctor_fields': [], 'decl_param_counts': [], 'decl_ctor_counts': [],
    'decl_templates': [], 'plan_nodes': [], 'plan_cases': [], 'field_paths': [],
    'constants': [], 'constant_fields': [], 'names': [],
}
S = {'start': '2', 'len': '0'}
T = {'start': 2, 'len': 0}
MAX128 = '340282366920938463463374607431768211455'

# Each expected row is stated explicitly from the two public writer conventions.
# Indices need not refer to valid rows: these test formats, not admitted semantics.
ROWS = [
    ('expressions', {'Literal': {'constant': '7'}}, {'Literal': {'constant': 7}}),
    ('expressions', {'Var': {'binder': '7'}}, {'Var': {'binder': 7}}),
    ('expressions', {'Let': {'binder': '7', 'value': '2', 'body': '0'}}, {'Let': {'binder': 7, 'value': 2, 'body': 0}}),
    ('expressions', {'Binary': {'op': 'Add', 'left': '2', 'right': '0'}}, {'Binary': {'op': 'Add', 'left': 2, 'right': 0}}),
    ('expressions', {'Unary': {'op': 'Not', 'operand': '2'}}, {'Unary': {'op': 'Not', 'operand': 2}}),
    ('expressions', {'If': {'cond': '2', 'then_expr': '0', 'else_expr': '7'}}, {'If': {'cond': 2, 'then_expr': 0, 'else_expr': 7}}),
    ('expressions', {'Call': {'function_name': '7', 'args': S}}, {'Call': {'function_name': 7, 'args': T}}),
    ('expressions', {'Match': {'scrutinee': '7', 'arms': S, 'plan': None}}, {'Match': {'scrutinee': 7, 'arms': T, 'plan': None}}),
    ('expressions', {'Match': {'scrutinee': '7', 'arms': S, 'plan': '0'}}, {'Match': {'scrutinee': 7, 'arms': T, 'plan': 0}}),
    ('expressions', {'Construct': {'type_id': {'TypeId': '7'}, 'ctor_id': '2', 'fields': S, 'constant': '0'}}, {'Construct': {'type_id': 7, 'ctor_id': 2, 'fields': T, 'constant': 0}}),
    ('occurrences', {'owner_is_function': False, 'function': '7', 'path': S}, {'owner_is_function': False, 'function': 7, 'path': T}),
    ('functions', {'function_name': '7', 'params': S, 'return_type': 'Tree', 'body': '2'}, {'function_name': 7, 'params': T, 'return_type': 3, 'body': 2}),
    ('parameters', {'binder': '7', 'sort': 'Bool'}, {'binder': 7, 'sort': 1}),
    ('arms', {'pattern': '7', 'body': '2'}, {'pattern': 7, 'body': 2}),
    ('patterns', {'PVar': {'x': '7'}}, {'PVar': {'x': 7}}),
    ('patterns', 'PNil', 'PNil'),
    ('patterns', {'PCons': {'x': '7', 'y': '2'}}, {'PCons': {'x': 7, 'y': 2}}),
    ('patterns', {'PLeaf': {'x': '7'}}, {'PLeaf': {'x': 7}}),
    ('patterns', {'PBranch': {'x': '7', 'y': '2'}}, {'PBranch': {'x': 7, 'y': 2}}),
    ('patterns', 'Wildcard', 'Wildcard'),
    ('patterns', {'Constructor': {'type_id': 'List', 'ctor_id': '7', 'children': S}}, {'Constructor': {'type_id': 2, 'ctor_id': 7, 'children': T}}),
    ('types', {'name_id': '7', 'decl': None, 'args': S, 'ctor_start': '2', 'ctor_len': '0'}, {'name_id': 7, 'decl': None, 'args': T, 'ctor_start': 2, 'ctor_len': 0}),
    ('types', {'name_id': '7', 'decl': '0', 'args': S, 'ctor_start': '2', 'ctor_len': '0'}, {'name_id': 7, 'decl': 0, 'args': T, 'ctor_start': 2, 'ctor_len': 0}),
    ('constructors', {'owning_type': 'UInt', 'name_id': '7', 'source_order': '2', 'field_start': '0', 'field_len': '2'}, {'owning_type': 0, 'name_id': 7, 'source_order': 2, 'field_start': 0, 'field_len': 2}),
    ('ctor_fields', {'name_id': '7', 'type_id': {'TypeId': '4'}}, {'name_id': 7, 'type_id': 4}),
    ('plan_nodes', {'Switch': {'source_match': '7', 'path': S, 'type_id': 'Bool', 'cases': S, 'default': '2'}}, {'Switch': {'source_match': 7, 'path': T, 'type_id': 1, 'cases': T, 'default': 2}}),
    ('plan_nodes', {'Bind': {'source_match': '7', 'path': S, 'binder': '2', 'next': '0'}}, {'Bind': {'source_match': 7, 'path': T, 'binder': 2, 'next': 0}}),
    ('plan_nodes', {'Select': {'source_match': '7', 'arm': '2'}}, {'Select': {'source_match': 7, 'arm': 2}}),
    ('plan_nodes', {'NoMatch': {'source_match': '7'}}, {'NoMatch': {'source_match': 7}}),
    ('constants', {'U128': {'marker_free': True, 'n': MAX128}}, {'U128': {'marker_free': True, 'n': MAX128}}),
    ('constants', {'Bool': {'marker_free': False, 'b': True}}, {'Bool': {'marker_free': False, 'b': True}}),
    ('constants', {'Nil': {'marker_free': True}}, {'Nil': {'marker_free': True}}),
    ('constants', {'Cons': {'marker_free': True, 'fields': S}}, {'Cons': {'marker_free': True, 'fields': T}}),
    ('constants', {'Leaf': {'marker_free': False, 'fields': S}}, {'Leaf': {'marker_free': False, 'fields': T}}),
    ('constants', {'Branch': {'marker_free': True, 'fields': S}}, {'Branch': {'marker_free': True, 'fields': T}}),
    ('constants', {'Constructor': {'marker_free': False, 'type_id': {'TypeId': '7'}, 'ctor_id': '2', 'fields': S}}, {'Constructor': {'marker_free': False, 'type_id': 7, 'ctor_id': 2, 'fields': T}}),
    ('constants', {'U128OutOfRange': {'marker_free': False}}, {'U128OutOfRange': {'marker_free': False}}),
]


class FormatTests(unittest.TestCase):
    def refuse(self, field, value):
        obj = copy.deepcopy(EMPTY)
        obj[field] = value
        with self.assertRaises(FormatError):
            normalize_program_view(obj)

    def test_complete_23_fields_all_rows_and_padding(self):
        source, target = copy.deepcopy(EMPTY), copy.deepcopy(EMPTY)
        target['entry'] = 0
        for field, before, after in ROWS:
            source[field].append(copy.deepcopy(before))
            target[field].append(copy.deepcopy(after))
        for field in ('occurrence_paths', 'children', 'pattern_children',
                      'decl_param_counts', 'decl_ctor_counts', 'field_paths', 'constant_fields'):
            source[field], target[field] = ['2', '0', '2'], [2, 0, 2]
        source['type_args'] = ['UInt', 'Bool', 'List', 'Tree', {'TypeId': '7'}, 'UInt']
        target['type_args'] = [0, 1, 2, 3, 7, 0]
        source['plan_cases'], target['plan_cases'] = [None, '2', None, '0'], [None, 2, None, 0]
        source['names'] = target['names'] = ['007', 'UInt', '', '007']
        # The TypeExprSrc units must stay strings despite TypeIdC conversion.
        template = {'name': '007', 'parameters': ['T', '007'], 'ctors': [
            {'name': 'C', 'fields': [{'name': 'x', 'ty': x} for x in [
                'UInt', 'Bool', 'List', 'Tree', {'Named': {'name': '007'}},
                {'Param': {'name': 'T'}}, {'Apply': {'name': 'Box', 'args': [
                    {'Param': {'name': 'T'}}, {'Apply': {'name': 'Box', 'args': ['UInt']}}]}}]]}]} 
        source['decl_templates'] = [copy.deepcopy(template)]
        target['decl_templates'] = [copy.deepcopy(template)]
        unchanged = copy.deepcopy(source)
        result = normalize_program_view(source)
        self.assertEqual(len(result), 23)
        self.assertEqual(result, target)
        self.assertEqual(source, unchanged)
        self.assertIsNot(result['expressions'], source['expressions'])
        self.assertEqual(normalize_program_view(loads_strict(json.dumps(source))), target)

    def test_all_unit_operators(self):
        for op in ('Add', 'Sub', 'Mul', 'Eq', 'Lt', 'Cons', 'Branch'):
            obj = copy.deepcopy(EMPTY)
            obj['expressions'] = [{'Binary': {'op': op, 'left': '0', 'right': '2'}}]
            self.assertEqual(normalize_program_view(obj)['expressions'],
                             [{'Binary': {'op': op, 'left': 0, 'right': 2}}])
        for op in ('Leaf', 'Not'):
            obj = copy.deepcopy(EMPTY)
            obj['expressions'] = [{'Unary': {'op': op, 'operand': '2'}}]
            self.assertEqual(normalize_program_view(obj)['expressions'],
                             [{'Unary': {'op': op, 'operand': 2}}])

    def test_missing_extra_and_wrong_containers(self):
        for key in EMPTY:
            obj = copy.deepcopy(EMPTY)
            del obj[key]
            with self.subTest(missing=key), self.assertRaises(FormatError):
                normalize_program_view(obj)
        obj = copy.deepcopy(EMPTY)
        obj['extra'] = []
        with self.assertRaises(FormatError):
            normalize_program_view(obj)
        for field in EMPTY.keys() - {'entry'}:
            for invalid in (None, {}, '[]', (), False):
                with self.subTest(field=field, invalid=invalid):
                    self.refuse(field, invalid)
        for invalid in (None, [], False, 0):
            with self.assertRaises(FormatError):
                normalize_program_view(invalid)
        for row in ({'binder': '0'}, {'binder': '0', 'sort': 'UInt', 'extra': 0}):
            self.refuse('parameters', [row])
        self.refuse('functions', [{'function_name': '0', 'params': {'start': '0'}, 'return_type': 'UInt', 'body': '0'}])

    def test_decimal_and_scalar_contexts(self):
        for invalid in ('', '00', '01', '+1', '-1', ' 1', '1 ', '1.0', '1e3', '١', 1, True, 1.0, None):
            with self.subTest(decimal=invalid):
                self.refuse('entry', invalid)
                self.refuse('constants', [{'U128': {'marker_free': True, 'n': invalid}}])
        self.refuse('constants', [{'U128': {'marker_free': True, 'n': '340282366920938463463374607431768211456'}}])
        for invalid in (1, 0, 'true', None):
            self.refuse('constants', [{'Bool': {'marker_free': invalid, 'b': True}}])
            self.refuse('constants', [{'Bool': {'marker_free': True, 'b': invalid}}])
        self.refuse('names', [7])
        for invalid in ('0', 0, True, 'Unknown', {'TypeId': 7}, {'TypeId': '0'},
                        {'TypeId': '07'}, {'TypeId': '7', 'x': 0}):
            self.refuse('type_args', [invalid])

    def test_enum_units_payloads_options_and_recursion(self):
        for invalid in ('Var', 'Unknown', {'PNil': None}, {'PNil': {}},
                        {'PVar': None}, {'PVar': {'x': '0'}, 'PNil': None},
                        {'PVar': {'x': '0', 'extra': '2'}}):
            self.refuse('patterns', [invalid])
        for invalid in ('Nil', {'Nil': None}, {'Nil': {}}, {'Unknown': {}}):
            self.refuse('constants', [invalid])
        for invalid in ({'Some': '0'}, {'None': None}, 'None', False, 0):
            self.refuse('plan_cases', [invalid])
        for invalid in ('Unknown', {'Add': None}, True):
            self.refuse('expressions', [{'Binary': {'op': invalid, 'left': '0', 'right': '0'}}])
        for invalid in ('Apply', {'UInt': None}, {'TypeId': '7'}, {'Apply': {'name': 'B', 'args': ['Unknown']}}):
            self.refuse('decl_templates', [{'name': 'B', 'parameters': [], 'ctors': [
                {'name': 'C', 'fields': [{'name': 'f', 'ty': invalid}]}]}])

    def test_duplicate_keys_and_non_json_numbers_are_rejected_before_projection(self):
        for text in ('{"entry":"0","entry":"1"}',
                     '{"x":1,"\\u0078":2}', '{"outer":{"x":1,"x":2}}',
                     '{"x":NaN}', '{"x":Infinity}', '{"x":-Infinity}',
                     '{"x":1.0}', '{"x":1e3}', '{} {}', '{"x":01}'):
            with self.subTest(text=text), self.assertRaises(FormatError):
                loads_strict(text)
        self.assertEqual(loads_strict('{"n":340282366920938463463374607431768211455,"b":true}'),
                         {'n': 340282366920938463463374607431768211455, 'b': True})


if __name__ == '__main__':
    unittest.main()
