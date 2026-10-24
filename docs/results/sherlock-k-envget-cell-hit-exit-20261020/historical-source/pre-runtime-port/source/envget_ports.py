"""Positive native snapshot <-> complete K DATA fields, without evaluation.

Only the released actual BEFORE belongs in query(). This module opens public
declarations/ports, never owner outputs or any expected operand. It does not
project OwnerWire, implement a guard/parser, or compute transition outcomes.
The guarded runtime is separate designated-owner source.
"""
import importlib.util
import json
from pathlib import Path

N = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01')
STATE_SOURCE = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-C6-complete-positive-HELPER-TASK-STATE-20261020/producer-source01/source/state_ports.py')
spec = importlib.util.spec_from_file_location('envget_existing_state_ports', STATE_SOURCE)
state = importlib.util.module_from_spec(spec)
spec.loader.exec_module(state)
PUBLIC = json.loads((N / 'PUBLIC-SOURCE-DECLARATION-REGISTRY.json').read_text())
# Registry shape belongs to the frozen public declaration cut.
ROWS = PUBLIC['constructors_and_functions']
REG = {name: (row['sort'], tuple(row['argument_sorts'])) for name, row in ROWS.items()}

ROOT_FIELDS = ('control', 'control_origin', 'env_root', 'stack_limit', 'outcome',
               'initial_fuel', 'remaining_work', 'consumed_work', 'helper_work',
               'helper_visits', 'reference_cursor', 'alloc', 'active')
EVENT_FIELDS = ('step_id', 'rule_id', 'event_kind', 'site', 'helper_kind',
                'disposition', 'work_delta', 'helper_work_delta',
                'helper_visits_delta', 'reference_cursor_delta', 'alloc_deltas',
                'value_slot_writes', 'logical_depth_before', 'logical_depth_after',
                'task_cursor_before', 'task_cursor_after', 'helper_phase_before',
                'helper_phase_after', 'helper_cursor_before', 'helper_cursor_after',
                'result_tag', 'origin_reference_cursor', 'source_expr_id',
                'source_occurrence', 'exhausted_kind', 'outcome_tag',
                'outcome_payload', 'boundary')


def node(name, *arguments):
    return {'sort': REG[name][0], 'constructor': name, 'arguments': list(arguments)}


def literal(sort, value):
    return {'sort': sort, 'literal': value}


def from_json(value):
    """Complete immutable EGJSON; no admission/RunId reconstruction."""
    if value is None:
        return node('egNull')
    if type(value) is bool:
        return node('egBoolean', literal('Bool', value))
    if type(value) is int:
        return node('egInteger', literal('Int', value))
    if type(value) is str:
        return node('egText', literal('String', value))
    if type(value) is list:
        out = node('egNoValues')
        for item in reversed(value):
            out = node('egValue', from_json(item), out)
        return node('egArray', out)
    if type(value) is dict:
        out = node('egNoFields')
        for key, item in reversed(list(value.items())):
            out = node('egField', literal('String', key), from_json(item), out)
        return node('egObject', out)
    raise TypeError('outside declared positive native JSON carrier')


def values_to_json(term):
    out = []
    while term['constructor'] == 'egValue':
        value, term = term['arguments']
        out.append(to_json(value))
    return out


def to_json(term):
    name, args = term['constructor'], term['arguments']
    if name == 'egNull':
        return None
    if name in ('egInteger', 'egBoolean', 'egText'):
        return args[0]['literal']
    if name == 'egArray':
        return values_to_json(args[0])
    if name == 'egObject':
        fields, out = args[0], {}
        while fields['constructor'] == 'egField':
            key, value, fields = fields['arguments']
            out[key['literal']] = to_json(value)
        return out
    raise ValueError('not an EGJSON DATA carrier')


def from_snapshot(snapshot):
    """Field-only split/repack of all23 image fields and whole admission."""
    image = snapshot['image']
    root = {key: image[key] for key in ROOT_FIELDS}
    seg = image['seg_base']
    root.update(seg_active=seg['active'], seg_alloc=seg['alloc'],
                seg_consumed_work=seg['consumed_work'])
    helper = {key: (None if image[key] is None else image[key]['Some'])
              for key in ('helper', 'result')}
    helper['tasks'] = image['tasks']
    data = {key: image[key] for key in ('data', 'data_fields', 'env', 'value_slots')}
    owned = state.from_json('CSOwnedState',
                            dict(root=root, frames=image['frames'], data=data, helper=helper))
    return node('egSnapshot', from_json(snapshot['admission']),
                from_json(image['run_id']), owned)


def to_snapshot(term):
    admission, run_id, owned = term['arguments']
    split = state.to_json(owned)
    root = split['root']
    image = {key: root[key] for key in ROOT_FIELDS}
    image['run_id'] = to_json(run_id)
    image['seg_base'] = dict(active=root['seg_active'], alloc=root['seg_alloc'],
                             consumed_work=root['seg_consumed_work'])
    image['frames'] = split['frames']
    image.update(split['data'])
    for key in ('helper', 'result'):
        value = split['helper'][key]
        image[key] = None if value is None else {'Some': value}
    image['tasks'] = split['helper']['tasks']
    return dict(admission=to_json(admission), image=image)


def query(actual_before):
    """One unreduced call; no Rust afterstate/event/expected argument."""
    return {'sort': 'EGWindow', 'constructor': 'egTwoEventWindow',
            'arguments': [from_snapshot(actual_before)]}


def event_field(term):
    """Declared EGEvent payloads -> original native ABI, without updates."""
    if term['sort'] in ('Int', 'Bool', 'String'):
        return term['literal']
    name, args = term['constructor'], term['arguments']
    if name == 'egStepId':
        return dict(run_id=to_json(args[0]), ordinal=args[1]['literal'])
    names = {'egEnvGetCellRule': 'BHelperEnvGetCell',
             'egEnvGetExitRule': 'BHelperEnvGetExit',
             'egHelperVisit': 'HelperVisit', 'egHelperExit': 'HelperExit',
             'egApplied': 'Applied', 'egRunningTag': 'Running'}
    if name in names:
        return names[name]
    if name in ('egNoPhase', 'egNoResultTag', 'egNatNone',
                'egNoExhaustion', 'egNoBoundary'):
        return None
    if name == 'egSiteSome':
        return {'Some': state.to_json(args[0])}
    if name == 'egSomeEnvGet':
        return {'Some': 'EnvGet'}
    if name == 'egTriPhase':
        return {'Some': {'Tri': state.to_json(args[0])}}
    if name == 'egEnvFoundTag':
        return {'Some': 'EnvFound'}
    if name == 'egNatSome':
        return {'Some': args[0]['literal']}
    if name == 'egOccurrenceLocator':
        return {'Occurrence': dict(owner_is_function=args[0]['literal'],
                                   function=args[1]['literal'],
                                   ast_path=values_to_json(args[2]))}
    return state.to_json(term)


def to_event(term):
    return {key: event_field(value) for key, value in zip(EVENT_FIELDS, term['arguments'])}


def to_window(term):
    before, middle, after, cell_event, exit_event = term['arguments']
    return dict(before=to_snapshot(before), after_cell=to_snapshot(middle),
                after_exit=to_snapshot(after), events=[to_event(cell_event), to_event(exit_event)])
