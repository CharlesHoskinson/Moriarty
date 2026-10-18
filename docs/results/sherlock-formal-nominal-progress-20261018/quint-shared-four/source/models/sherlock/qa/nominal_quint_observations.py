"""Normalize supplied nominal Quint ITF to the declaration-only full-state wire.

No model imports, evaluator, transition relation or expected-cell dependency.
Raw ITF and ordinal provenance stay in the envelope. Only exact repeated terminal
states leave the comparison prefix; pending repeats and changed states remain.
"""
import argparse
import hashlib
import json
from pathlib import Path

from quint_observations import canonical_bytes, decode_itf

PROFILE = 'sherlock-quint-nominal-construction/1'
WIRE_SCHEMA = 'independent-nominal-full-state/1'
FORMAT = 'sherlock-nominal-quint-itf-wire/1'
SNAPSHOT_KEYS = frozenset({
    'program', 'control', 'environment', 'frames', 'initialFuel', 'remaining',
    'consumed', 'helperWork', 'constructorNodes', 'constructorFieldSlots',
    'outcome', 'source', 'event', 'builds', 'selectedArms', 'enteredArguments',
    'maxDepth',
})
NULLARY = frozenset({
    'Pending', 'Exhausted', 'OutsideDomain', 'Stopped', 'Add', 'Sub', 'Equal',
    'Start', 'LiteralEvent', 'VariableEvent', 'LetEvent', 'BinaryEvent', 'IfEvent',
    'ConstructorIntroduce', 'ConstructorFieldReturn', 'ConstructorBuild',
    'CallEvent', 'ArgumentReturn', 'CallEnter', 'CallReturn', 'ScopeReturn',
    'MatchEvent', 'MatchReturn', 'MatchMiss', 'MatchSelect', 'PrimitiveReturn',
    'BranchSelect', 'TerminalPublication', 'FuelStop', 'DomainStop',
})


def environment_wire(environment):
    """Preserve all integer-name bindings, in the declared numeric name order."""
    return [dict(name=name, value=typed_wire(environment[name]))
            for name in sorted(environment)]


def typed_wire(value):
    """Pure structural representation change; no expression expansion/evaluation."""
    if type(value) is list:
        return [typed_wire(item) for item in value]
    if type(value) is not dict:
        return value
    if set(value) == {'tag', 'value'}:
        tag, payload = value['tag'], value['value']
        if tag in NULLARY:
            return tag
        if tag == 'ScopeFrame':
            return {tag: environment_wire(payload)}
        return {tag: typed_wire(payload)}
    # All non-program maps in the declared nominal Snapshot are environments.
    # Program maps are handled by program_wire before reaching this function.
    if all(type(key) is int for key in value):
        return environment_wire(value)
    return {key: typed_wire(item) for key, item in value.items()}


def program_wire(program):
    """Retain every AST/function/layout row; numeric IDs are never expanded."""
    if set(program) != {'expressions', 'functions', 'layouts', 'entry'}:
        raise ValueError('nominal Program field set differs from declared profile')
    layouts = []
    for key in sorted(program['layouts']):
        row = program['layouts'][key]
        if key != (row['typeId'], row['ctorId']):
            raise ValueError('nominal layout key/row identity disagreement')
        layouts.append(dict(typeId=row['typeId'], ctorId=row['ctorId'],
                            fields=typed_wire(row['fields'])))
    return dict(
        expressions=[dict(exprId=identity, expression=typed_wire(expression))
                     for identity, expression in sorted(program['expressions'].items())],
        functions=[dict(name=name, function=typed_wire(function))
                   for name, function in sorted(program['functions'].items())],
        layouts=layouts,
        entry=program['entry'],
    )


def snapshot_wire(snapshot):
    """Compare the entire declared 17-field Snapshot; never select a subset."""
    if set(snapshot) != SNAPSHOT_KEYS:
        raise ValueError('nominal Snapshot field set differs from declared profile')
    normalized = {key: typed_wire(value) for key, value in snapshot.items()
                  if key not in {'program', 'environment'}}
    normalized['program'] = program_wire(snapshot['program'])
    normalized['environment'] = environment_wire(snapshot['environment'])
    return normalized


def machine_variable(state):
    names = [name for name in state if name == 'machine' or name.endswith('::machine')]
    if len(names) != 1:
        raise ValueError('expected one complete nominal machine variable')
    return decode_itf(state[names[0]])


def input_metadata(case):
    """Only immutable input declarations produce source occurrence/group metadata."""
    occurrences = [dict(exprId=row['exprId'], occurrence=row['occurrence'])
                   for row in sorted(case['expressions'], key=lambda row: row['exprId'])]
    groups = [dict(typeId=row['typeId'], recursiveGroup=row['recursiveGroup'])
              for row in sorted(case['declarations'], key=lambda row: row['typeId'])]
    return occurrences, groups


def normalize_trace(case, trace, *, terminal_step_contract):
    """Return actual normalized states plus raw capture/provenance, never a golden."""
    raw_states = trace['states']
    if not raw_states:
        raise ValueError('supplied ITF has no states')
    normalized_states = [snapshot_wire(machine_variable(state)) for state in raw_states]
    states, raw_ordinals, stutters = [], [], []
    for ordinal, snapshot in enumerate(normalized_states):
        if states and states[-1]['outcome'] != 'Pending' and snapshot == states[-1]:
            stutters.append(ordinal)
        else:
            states.append(snapshot)
            raw_ordinals.append(ordinal)
    occurrences, groups = input_metadata(case)
    observation = dict(schema=WIRE_SCHEMA, profile=PROFILE, caseId=case['caseId'],
                       states=states, terminalStepContract=terminal_step_contract,
                       sourceOccurrences=occurrences, recursiveGroups=groups)
    return dict(
        format=FORMAT,
        observation=observation,
        raw_itf=trace,
        raw_itf_canonical_sha256=hashlib.sha256(canonical_bytes(trace)).hexdigest(),
        projection=dict(raw_state_count=len(raw_states), comparison_state_count=len(states),
                        comparison_to_raw_ordinals=raw_ordinals,
                        exact_terminal_stutter_ordinals=stutters,
                        pending_stutters='retained', changed_terminal_states='retained',
                        numeric_names='ascending', node_identity='structural source profile',
                        transitions='not evaluated', source_profile=PROFILE),
        evidence=dict(source_execution_receipt='not-supplied',
                      producer_authenticity='not-established',
                      independent_expected_cells='not-read'),
    )


def _source_case(document, case_id):
    cases = document.get('cases', [document.get('case', document)])
    selected = [case for case in cases if case['caseId'] == case_id]
    if len(selected) != 1:
        raise ValueError('expected exactly one selected input-only source case')
    return selected[0]


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('manifest', type=Path,
                        help='{terminalStepContract,runs:[{caseId,itf,safeInput}]}')
    args = parser.parse_args(argv)
    manifest = json.loads(args.manifest.read_text())
    bundles = []
    for row in manifest['runs']:
        itf_path = args.manifest.parent / row['itf']
        input_path = args.manifest.parent / row['safeInput']
        raw = itf_path.read_bytes()
        source = input_path.read_bytes()
        case = _source_case(json.loads(source), row['caseId'])
        bundle = normalize_trace(case, json.loads(raw),
                                 terminal_step_contract=manifest['terminalStepContract'])
        bundle['supplied_files'] = dict(
            itf=dict(path=str(itf_path), sha256=hashlib.sha256(raw).hexdigest()),
            safe_input=dict(path=str(input_path), sha256=hashlib.sha256(source).hexdigest()))
        bundles.append(bundle)
    print(canonical_bytes(dict(format=FORMAT, runs=bundles)).decode())


if __name__ == '__main__':
    main()
