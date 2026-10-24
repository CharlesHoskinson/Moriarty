"""Field-only positive JSON <-> K DATA carrier; never computes encoded lanes.

This is a model input/output port over externally positive typed carriers. It is
not a parser, admission/security/refusal gate, K evaluator or lane encoder. The
later guarded JSONv4/raw-print runtime and comparator remain designated-owner
work after actual compiled-label qualification. No private oracle imports.
"""
from pathlib import Path
import json

HERE = Path(__file__).resolve().parent
INPUT_PATH = HERE.parent.parent / 'declaration-transport01' / 'SAFE' / 'SAFE-INPUTS.json'
PUBLIC = json.loads((HERE / 'PUBLIC-COMBINED-CONSTRUCTOR-REGISTRY.json').read_text())
POSITION = json.loads((HERE / 'PUBLIC-COMBINED-JSON-POSITION-MAP.json').read_text())['constructors']
REG = {x['name']: (x['sort'], tuple(x['argument_sorts'])) for x in PUBLIC['constructors']}
BY_SORT = {}
for _name, (_sort, _args) in REG.items():
    BY_SORT.setdefault(_sort, []).append(_name)
CASE_KEYS = ('helper_record_cases', 'result_record_cases', 'task_record_cases',
             'helper_section_cases', 'state_cases')
PRIMITIVES = {'Int': int, 'Bool': bool, 'String': str}
ENTRY = dict(name='csObserveAll', sort='CSObservations', argument_sorts=['CSCohort'],
             declaring_module='SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE')


def node(name, *arguments):
    """Ordinary construction of a declared positive DATA term, not K evaluation."""
    return {'sort': REG[name][0], 'constructor': name, 'arguments': list(arguments)}


def from_json(sort, value):
    """Map complete positive source fields; no normalization or numerical codec."""
    if sort in PRIMITIVES:
        if type(value) is not PRIMITIVES[sort]:
            raise TypeError('positive carrier primitive kind differs: ' + sort)
        return {'sort': sort, 'literal': value}
    names = BY_SORT[sort]
    empty = next((n for n in names if POSITION[n]['kind'] == 'empty_array'), None)
    if empty is not None:
        row = next(n for n in names if POSITION[n]['kind'] == 'ordered_list_cons')
        head_sort = REG[row][1][0]
        tail = node(empty)
        for head in reversed(value):
            tail = node(row, from_json(head_sort, head), tail)
        return tail
    if sort == 'CSOwnedState':
        image = dict(value['root'])
        image['frames'] = value['frames']
        return node('csOwnedState', from_json('CTRoot', image),
                    from_json('DPTables', value['data']),
                    from_json('CSHelperTables', value['helper']))
    for name in names:
        m = POSITION[name]
        kind = m['kind']
        arg_sorts = REG[name][1]
        fields = m.get('fields', [])
        if kind == 'null' and value is None:
            return node(name)
        if kind == 'transparent_outer_presence' and value is not None:
            return node(name, from_json(arg_sorts[0], value))
        if kind == 'some' and isinstance(value, dict) and set(value) == {'Some'}:
            return node(name, from_json(arg_sorts[0], value['Some']))
        if kind == 'nullary_variant' and value == m['variant']:
            return node(name)
        if kind == 'singleton_scalar_variant' and isinstance(value, dict) and set(value) == {m['variant']}:
            return node(name, from_json(arg_sorts[0], value[m['variant']]))
        if kind == 'payload_variant' and isinstance(value, dict) and set(value) == {m['variant']}:
            payload = value[m['variant']]
            return node(name, *(from_json(s, payload[f]) for s, f in zip(arg_sorts, fields)))
        if kind in ('object', 'image_root_with_frames'):
            return node(name, *(from_json(s, value[f]) for s, f in zip(arg_sorts, fields)))
    raise ValueError('no declared positive carrier variant: ' + sort)


def to_json(term):
    """Decode the whole DATA shape, never manufacture a missing observation."""
    sort = term['sort']
    if sort in PRIMITIVES:
        return term['literal']
    name, args = term['constructor'], term['arguments']
    m = POSITION[name]
    kind = m['kind']
    if kind in ('empty_array', 'ordered_list_cons'):
        out = []
        current = term
        while POSITION[current['constructor']]['kind'] == 'ordered_list_cons':
            head, current = current['arguments']
            out.append(to_json(head))
        return out
    if kind == 'split_image_object':
        image = to_json(args[0])
        frames = image.pop('frames')
        return dict(root=image, frames=frames, data=to_json(args[1]), helper=to_json(args[2]))
    if kind == 'null':
        return None
    if kind == 'transparent_outer_presence':
        return to_json(args[0])
    if kind == 'some':
        return {'Some': to_json(args[0])}
    if kind == 'nullary_variant':
        return m['variant']
    if kind == 'singleton_scalar_variant':
        return {m['variant']: to_json(args[0])}
    payload = {f: to_json(a) for f, a in zip(m['fields'], args)}
    if kind == 'payload_variant':
        return {m['variant']: payload}
    return payload


def document_to_cohort(document):
    """Retain every original case, source schema/profile/caps and exact ID order."""
    return from_json('CSCohort', document)


def cohort_to_document(term):
    return to_json(term)


def query(document):
    """Unreduced function query; may never be accepted as an actual DATA answer."""
    return {'sort': 'CSObservations', 'constructor': 'csObserveAll',
            'arguments': [document_to_cohort(document)]}
