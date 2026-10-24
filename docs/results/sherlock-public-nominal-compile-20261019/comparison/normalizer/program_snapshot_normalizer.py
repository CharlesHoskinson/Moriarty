"""Pure ProgramView owner-wire -> public ProgramSnapshot JSON representation.

Source-derived format conversion only: no evaluator, admission, identity or semantic
oracle. All 23 fields and all rows (including padding) are retained in order.
The fixed declarations below are transcribed from frozen map SHA256
0dde99df859412eb669c48200e69bab48caa07f7ba432ef160b878d2bee7d24d.
No file I/O, CLI, expected data, captured actuals or import-time calls.
"""
import json
import re


class FormatError(ValueError):
    """Input is not the exact declared JSON wire representation."""


def _fail(path, reason):
    raise FormatError(f"{path}: {reason}")


def _pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            _fail('$', f'duplicate JSON object key {key!r}')
        result[key] = value
    return result


def _noninteger(token):
    _fail('$', f'noninteger JSON numeric token {token!r}')


def loads_strict(text):
    """Decode entire JSON text before extracting program_view; reject duplicates.

    Floats/exponents and non-JSON NaN/Infinity are not part of either wire.
    Standard JSON integer parsing preserves precision. No object projection here.
    """
    if type(text) is not str:
        _fail('$', 'expected JSON text string')
    try:
        return json.loads(text, object_pairs_hook=_pairs,
                          parse_float=_noninteger, parse_constant=_noninteger)
    except ValueError as exc:
        raise FormatError(str(exc)) from exc


def canonical_json(value):
    """For exact comparison AFTER strict decoding/normalization.

    Only object keys are sorted; arrays, strings and JSON scalar kinds persist.
    Compare these strings, not Python objects (where True == 1).
    This function does not validate target snapshot schema or grant acceptance.
    """
    return json.dumps(value, sort_keys=True, separators=(',', ':'),
                      ensure_ascii=True, allow_nan=False)


def _decimal(value, path):
    if type(value) is not str or re.fullmatch(r'0|[1-9][0-9]*', value) is None:
        _fail(path, 'expected canonical nonnegative decimal string')
    return int(value)


def _convert(value, declaration, path):
    if type(declaration) is str:
        if declaration == 'usize_decimal_to_exact_integer':
            return _decimal(value, path)
        if declaration == 'identity_u128_decimal_string':
            if _decimal(value, path) > (1 << 128) - 1:
                _fail(path, 'u128 decimal string out of range')
            return value
        if declaration == 'identity_boolean':
            if type(value) is not bool:
                _fail(path, 'expected JSON boolean')
            return value
        if declaration == 'identity_string':
            if type(value) is not str:
                _fail(path, 'expected JSON string')
            return value
        if declaration == 'TypeIdC_to_exact_integer':
            if type(value) is str and value in ('UInt', 'Bool', 'List', 'Tree'):
                return ('UInt', 'Bool', 'List', 'Tree').index(value)
            if type(value) is dict and set(value) == {'TypeId'}:
                type_id = _decimal(value['TypeId'], path + '.TypeId')
                # The source writer reserves IDs 0..3 for the unit strings above.
                if type_id >= 4:
                    return type_id
            _fail(path, 'expected builtin TypeIdC name or custom {TypeId: decimal}')
        return _convert(value, _TYPES[declaration], path)
    if 'option' in declaration:
        return None if value is None else _convert(value, declaration['option'], path)
    if 'array' in declaration:
        if type(value) is not list:
            _fail(path, 'expected JSON array')
        return [_convert(item, declaration['array'], f'{path}[{i}]')
                for i, item in enumerate(value)]
    if 'fields' in declaration:
        fields = declaration['fields']
        if type(value) is not dict or set(value) != set(fields):
            _fail(path, 'expected object with exactly ' + ','.join(fields))
        return {key: _convert(value[key], sub, path + '.' + key)
                for key, sub in fields.items()}
    if 'unitVariants' in declaration:
        if type(value) is not str or value not in declaration['unitVariants']:
            _fail(path, 'unknown unit variant or payload on unit variant')
        return value
    variants = declaration['enum']
    if type(value) is str and value in variants and variants[value] is None:
        return value
    if type(value) is dict and len(value) == 1:
        tag = next(iter(value))
        if type(tag) is str and tag in variants and variants[tag] is not None:
            return {tag: _convert(value[tag], variants[tag], path + '.' + tag)}
    _fail(path, 'expected declared unit string or single-key payload variant')


def normalize_program_view(value):
    """Normalize exactly the selected artifact's /admission/program_view object.

    Caller must use loads_strict on the FULL artifact first so duplicate keys in
    discarded outer metadata cannot disappear. Outer identity/genesis/layout and
    source/config/preparation metadata are outside this table comparison.
    No fields are inferred, defaulted, filtered, sorted or semantically checked.
    """
    return _convert(value, {'fields': _PROGRAM}, '$.program_view')


# Fixed public declarations, not a caller-configurable schema framework.
_PROGRAM = {'entry': 'usize_decimal_to_exact_integer',
 'expressions': {'array': 'ExpressionSnapshot'},
 'occurrences': {'array': 'OccurrenceRowC'},
 'occurrence_paths': {'array': 'usize_decimal_to_exact_integer'},
 'children': {'array': 'usize_decimal_to_exact_integer'},
 'functions': {'array': 'FunctionRowC'},
 'parameters': {'array': 'ParameterRowC'},
 'arms': {'array': 'ArmRowC'},
 'patterns': {'array': 'PatternRowC'},
 'pattern_children': {'array': 'usize_decimal_to_exact_integer'},
 'types': {'array': 'TypeRowC'},
 'type_args': {'array': 'TypeIdC_to_exact_integer'},
 'constructors': {'array': 'CtorRowExt'},
 'ctor_fields': {'array': 'CtorFieldRowExt'},
 'decl_param_counts': {'array': 'usize_decimal_to_exact_integer'},
 'decl_ctor_counts': {'array': 'usize_decimal_to_exact_integer'},
 'decl_templates': {'array': 'TypeDeclSrc'},
 'plan_nodes': {'array': 'PlanNodeC'},
 'plan_cases': {'array': {'option': 'usize_decimal_to_exact_integer'}},
 'field_paths': {'array': 'usize_decimal_to_exact_integer'},
 'constants': {'array': 'ConstantSnapshot'},
 'constant_fields': {'array': 'usize_decimal_to_exact_integer'},
 'names': {'array': 'identity_string'}}

_TYPES = {'SliceC': {'fields': {'start': 'usize_decimal_to_exact_integer',
                       'len': 'usize_decimal_to_exact_integer'}},
 'OccurrenceRowC': {'fields': {'owner_is_function': 'identity_boolean',
                               'function': 'usize_decimal_to_exact_integer',
                               'path': 'SliceC'}},
 'FunctionRowC': {'fields': {'function_name': 'usize_decimal_to_exact_integer',
                             'params': 'SliceC',
                             'return_type': 'TypeIdC_to_exact_integer',
                             'body': 'usize_decimal_to_exact_integer'}},
 'ParameterRowC': {'fields': {'binder': 'usize_decimal_to_exact_integer',
                              'sort': 'TypeIdC_to_exact_integer'}},
 'ArmRowC': {'fields': {'pattern': 'usize_decimal_to_exact_integer',
                        'body': 'usize_decimal_to_exact_integer'}},
 'TypeRowC': {'fields': {'name_id': 'usize_decimal_to_exact_integer',
                         'decl': {'option': 'usize_decimal_to_exact_integer'},
                         'args': 'SliceC',
                         'ctor_start': 'usize_decimal_to_exact_integer',
                         'ctor_len': 'usize_decimal_to_exact_integer'}},
 'CtorRowExt': {'fields': {'owning_type': 'TypeIdC_to_exact_integer',
                           'name_id': 'usize_decimal_to_exact_integer',
                           'source_order': 'usize_decimal_to_exact_integer',
                           'field_start': 'usize_decimal_to_exact_integer',
                           'field_len': 'usize_decimal_to_exact_integer'}},
 'CtorFieldRowExt': {'fields': {'name_id': 'usize_decimal_to_exact_integer',
                                'type_id': 'TypeIdC_to_exact_integer'}},
 'TypeDeclSrc': {'fields': {'name': 'identity_string',
                            'parameters': {'array': 'identity_string'},
                            'ctors': {'array': 'CtorDeclSrc'}}},
 'CtorDeclSrc': {'fields': {'name': 'identity_string', 'fields': {'array': 'CtorFieldSrc'}}},
 'CtorFieldSrc': {'fields': {'name': 'identity_string', 'ty': 'TypeExprSrc'}},
 'TypeExprSrc': {'enum': {'UInt': None,
                          'Bool': None,
                          'List': None,
                          'Tree': None,
                          'Named': {'fields': {'name': 'identity_string'}},
                          'Param': {'fields': {'name': 'identity_string'}},
                          'Apply': {'fields': {'name': 'identity_string',
                                               'args': {'array': 'TypeExprSrc'}}}}},
 'ExpressionSnapshot': {'enum': {'Literal': {'fields': {'constant': 'usize_decimal_to_exact_integer'}},
                                 'Var': {'fields': {'binder': 'usize_decimal_to_exact_integer'}},
                                 'Let': {'fields': {'binder': 'usize_decimal_to_exact_integer',
                                                    'value': 'usize_decimal_to_exact_integer',
                                                    'body': 'usize_decimal_to_exact_integer'}},
                                 'Binary': {'fields': {'op': 'BinaryOp',
                                                       'left': 'usize_decimal_to_exact_integer',
                                                       'right': 'usize_decimal_to_exact_integer'}},
                                 'Unary': {'fields': {'op': 'UnaryOp',
                                                      'operand': 'usize_decimal_to_exact_integer'}},
                                 'If': {'fields': {'cond': 'usize_decimal_to_exact_integer',
                                                   'then_expr': 'usize_decimal_to_exact_integer',
                                                   'else_expr': 'usize_decimal_to_exact_integer'}},
                                 'Call': {'fields': {'function_name': 'usize_decimal_to_exact_integer',
                                                     'args': 'SliceC'}},
                                 'Match': {'fields': {'scrutinee': 'usize_decimal_to_exact_integer',
                                                      'arms': 'SliceC',
                                                      'plan': {'option': 'usize_decimal_to_exact_integer'}}},
                                 'Construct': {'fields': {'type_id': 'TypeIdC_to_exact_integer',
                                                          'ctor_id': 'usize_decimal_to_exact_integer',
                                                          'fields': 'SliceC',
                                                          'constant': {'option': 'usize_decimal_to_exact_integer'}}}}},
 'PatternRowC': {'enum': {'PVar': {'fields': {'x': 'usize_decimal_to_exact_integer'}},
                          'PNil': None,
                          'PCons': {'fields': {'x': 'usize_decimal_to_exact_integer',
                                               'y': 'usize_decimal_to_exact_integer'}},
                          'PLeaf': {'fields': {'x': 'usize_decimal_to_exact_integer'}},
                          'PBranch': {'fields': {'x': 'usize_decimal_to_exact_integer',
                                                 'y': 'usize_decimal_to_exact_integer'}},
                          'Wildcard': None,
                          'Constructor': {'fields': {'type_id': 'TypeIdC_to_exact_integer',
                                                     'ctor_id': 'usize_decimal_to_exact_integer',
                                                     'children': 'SliceC'}}}},
 'PlanNodeC': {'enum': {'Switch': {'fields': {'source_match': 'usize_decimal_to_exact_integer',
                                              'path': 'SliceC',
                                              'type_id': 'TypeIdC_to_exact_integer',
                                              'cases': 'SliceC',
                                              'default': 'usize_decimal_to_exact_integer'}},
                        'Bind': {'fields': {'source_match': 'usize_decimal_to_exact_integer',
                                            'path': 'SliceC',
                                            'binder': 'usize_decimal_to_exact_integer',
                                            'next': 'usize_decimal_to_exact_integer'}},
                        'Select': {'fields': {'source_match': 'usize_decimal_to_exact_integer',
                                              'arm': 'usize_decimal_to_exact_integer'}},
                        'NoMatch': {'fields': {'source_match': 'usize_decimal_to_exact_integer'}}}},
 'ConstantSnapshot': {'enum': {'U128': {'fields': {'marker_free': 'identity_boolean',
                                                   'n': 'identity_u128_decimal_string'}},
                               'Bool': {'fields': {'marker_free': 'identity_boolean',
                                                   'b': 'identity_boolean'}},
                               'Nil': {'fields': {'marker_free': 'identity_boolean'}},
                               'Cons': {'fields': {'marker_free': 'identity_boolean',
                                                   'fields': 'SliceC'}},
                               'Leaf': {'fields': {'marker_free': 'identity_boolean',
                                                   'fields': 'SliceC'}},
                               'Branch': {'fields': {'marker_free': 'identity_boolean',
                                                     'fields': 'SliceC'}},
                               'Constructor': {'fields': {'marker_free': 'identity_boolean',
                                                          'type_id': 'TypeIdC_to_exact_integer',
                                                          'ctor_id': 'usize_decimal_to_exact_integer',
                                                          'fields': 'SliceC'}},
                               'U128OutOfRange': {'fields': {'marker_free': 'identity_boolean'}}}},
 'BinaryOp': {'unitVariants': ['Add', 'Sub', 'Mul', 'Eq', 'Lt', 'Cons', 'Branch'],
              'normalization': 'identity string'},
 'UnaryOp': {'unitVariants': ['Leaf', 'Not'], 'normalization': 'identity string'}}
