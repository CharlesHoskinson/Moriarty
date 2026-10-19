"""Pure independent-snapshot representation adapter, not a semantic oracle.

The oracle declared structural notation rather than the production wire. Its
schema's U128.n is a u128; Root observed exact JSON integer notation there.
The public target writer emits that same mathematical value as decimal text.
Only schema-typed /constants/*/U128/n changes JSON kind, integer -> string.
Every other schema leaf retains its JSON kind and value. No files are read here.

The pinned existing normalizer supplies only its fixed declarations and error
type. Its source converter is neither changed nor invoked by this adapter.
"""
from program_snapshot_normalizer import FormatError, _PROGRAM, _TYPES


def _fail(path, reason):
    raise FormatError(f'{path}: {reason}')


def _integer(value, path):
    # bool subclasses int in Python, but is a different JSON kind.
    if type(value) is not int or value < 0:
        _fail(path, 'expected exact nonnegative JSON integer, not bool/string/float')
    return value


def _visit(value, declaration, path):
    if type(declaration) is str:
        if declaration in ('usize_decimal_to_exact_integer', 'TypeIdC_to_exact_integer'):
            # Expected structural notation already uses integer indices/TypeIds.
            return _integer(value, path)
        if declaration == 'identity_u128_decimal_string':
            number = _integer(value, path)
            if number > (1 << 128) - 1:
                _fail(path, 'u128 integer out of range')
            # Sole representation change; no actual values are consulted.
            return str(number)
        if declaration == 'identity_boolean':
            if type(value) is not bool:
                _fail(path, 'expected JSON boolean')
            return value
        if declaration == 'identity_string':
            if type(value) is not str:
                _fail(path, 'expected JSON string')
            return value
        return _visit(value, _TYPES[declaration], path)
    if 'option' in declaration:
        return None if value is None else _visit(value, declaration['option'], path)
    if 'array' in declaration:
        if type(value) is not list:
            _fail(path, 'expected JSON array')
        return [_visit(item, declaration['array'], f'{path}[{i}]')
                for i, item in enumerate(value)]
    if 'fields' in declaration:
        fields = declaration['fields']
        if type(value) is not dict or set(value) != set(fields):
            _fail(path, 'expected object with exactly ' + ','.join(fields))
        return {key: _visit(value[key], sub, path + '.' + key)
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
            return {tag: _visit(value[tag], variants[tag], path + '.' + tag)}
    _fail(path, 'expected declared unit string or single-key payload variant')


def normalize_expected_snapshot(value):
    """Validate all 23 fields; translate only ConstantSnapshot.U128.n integers.

    Input MUST be the complete expected snapshot decoded by the pinned
    loads_strict, not a prefiltered object or production source owner-wire.
    No omitted/defaulted keys, reordered/removed rows or semantic repair.
    Compare output using pinned canonical_json so bool/int kinds stay distinct.
    """
    return _visit(value, {'fields': _PROGRAM}, '$.expected_snapshot')
