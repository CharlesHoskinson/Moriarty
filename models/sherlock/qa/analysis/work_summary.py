"""Summarize supplied complete Sherlock observations without executing a machine.

Only Python's standard library and the explicitly supplied JSON are read. This
is logical diagnostic accounting, separate from circuit costs and execution
authentication. It neither expands expected prefixes nor constructs states.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import sys

FORMAT = 'sherlock-logical-work-summary/1'
ENVELOPES = ('sherlock-lean-normal-observations/1', 'sherlock-quint-itf-observations/1',
             'sherlock-image-observations/1')
SNAPSHOT_FIELDS = ('code_k', 'entry_k', 'control_k', 'environment_k',
                   'frames_top_first_k', 'depth', 'stack_limit', 'remaining_fuel',
                   'initial_fuel', 'output_k', 'charged_steps')
WORK_FIELDS = ('initial_fuel', 'remaining_fuel', 'charged_steps')


def digest(value):
    """Digest normalized JSON bytes; array order and complete fields are retained."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                    ensure_ascii=False, allow_nan=False).encode('utf-8')).hexdigest()


def fields(value, names, label):
    if type(value) is not dict:
        raise ValueError(f'{label}: expected an object')
    for name in names:
        if name not in value:
            raise ValueError(f'{label}: missing {name}')


def natural(value, label):
    if type(value) is not int or value < 0:
        raise ValueError(f'{label}: expected a nonnegative integer')


def check_snapshot(value, label):
    fields(value, SNAPSHOT_FIELDS, label)
    for name in ('code_k', 'entry_k', 'control_k', 'environment_k', 'output_k'):
        if type(value[name]) is not str:
            raise ValueError(f'{label}.{name}: expected text')
    if type(value['frames_top_first_k']) is not list:
        raise ValueError(f'{label}.frames_top_first_k: expected an array')
    for name in ('depth', 'stack_limit', 'remaining_fuel', 'initial_fuel'):
        natural(value[name], f'{label}.{name}')
    if value['charged_steps'] is not None:
        natural(value['charged_steps'], f'{label}.charged_steps')


def normalized_record(record):
    """Ignore only run-budget fields for cross-run charged-event comparisons.

    This is an observed-event fingerprint, not a semantic equivalence proof.
    Charged work, controls, environments, frame order and all other fields stay.
    The original record and its exact digest remain untouched.
    """
    result = dict(record)
    for side in ('before', 'after'):
        result[side] = {key: value for key, value in record[side].items()
                        if key not in ('initial_fuel', 'remaining_fuel')}
    return result


def publication(record):
    """Count supplied free Pending-to-published output edges in the common shape."""
    return (record['charge'] == 0 and record['before']['output_k'] == 'uiPending()'
            and record['after']['output_k'] != 'uiPending()')


def summarize_run(run, index):
    fields(run, ('case_id', 'initial', 'records', 'terminal', 'terminal_has_next'), f'run {index}')
    check_snapshot(run['initial'], f'run {index}.initial')
    check_snapshot(run['terminal'], f'run {index}.terminal')
    if type(run['terminal_has_next']) is not bool or type(run['records']) is not list:
        raise ValueError(f'run {index}: expected records array and Boolean terminal_has_next')
    states, rules = [run['initial'], run['terminal']], {}
    for offset, record in enumerate(run['records']):
        label = f'run {index}.records[{offset}]'
        fields(record, ('rule_start_line', 'charge', 'before', 'after'), label)
        natural(record['rule_start_line'], label+'.rule_start_line')
        natural(record['charge'], label+'.charge')
        check_snapshot(record['before'], label+'.before')
        check_snapshot(record['after'], label+'.after')
        states.extend((record['before'], record['after']))
        rule = rules.setdefault(record['rule_start_line'], dict(
            rule_start_line=record['rule_start_line'], event_count=0, charged_sum=0))
        rule['event_count'] += 1
        rule['charged_sum'] += record['charge']
    initial_work = {name: run['initial'][name] for name in WORK_FIELDS}
    terminal_work = {name: run['terminal'][name] for name in WORK_FIELDS}
    delta = (None if initial_work['charged_steps'] is None or terminal_work['charged_steps'] is None
             else terminal_work['charged_steps'] - initial_work['charged_steps'])
    charged = sum(record['charge'] for record in run['records'])
    return dict(run_index=index, key=dict(case_id=run['case_id'], initial_fuel=initial_work['initial_fuel']),
                record_count=len(run['records']), charged_sum=charged,
                charged_event_count=sum(record['charge'] > 0 for record in run['records']),
                publication_count=sum(publication(record) for record in run['records']),
                per_source_rule=list(rules.values()), max_frames=max(len(s['frames_top_first_k']) for s in states),
                max_depth=max(s['depth'] for s in states), initial_work=initial_work, terminal_work=terminal_work,
                work_counter_delta=delta, charge_matches_counter_delta=None if delta is None else delta == charged,
                terminal_output_k=run['terminal']['output_k'], terminal_has_next=run['terminal_has_next'],
                run_sha256=digest(run), record_sha256_in_trace_order=[digest(r) for r in run['records']])


def summarize(artifact):
    """Return ordered accounting of supplied observations; no expected data is read."""
    if type(artifact) is not dict:
        raise ValueError('artifact: expected an object')
    envelope = 'observations' in artifact
    if envelope:
        fields(artifact, ('format', 'observations', 'projection'), 'envelope')
        if artifact['format'] not in ENVELOPES:
            raise ValueError('unsupported observation envelope format')
        observations = artifact['observations']
    else:
        observations = artifact
    fields(observations, ('producer', 'profile', 'source_sha256', 'boundary', 'programs', 'runs'), 'observations')
    if type(observations['programs']) is not list or type(observations['runs']) is not list:
        raise ValueError('observations: programs and runs must be arrays')
    programs = observations['programs']
    case_ids = []
    for program in programs:
        fields(program, ('case_id', 'code_k', 'entry_k'), 'program')
        if type(program['case_id']) is not str or program['case_id'] in case_ids:
            raise ValueError('program: case_id must be distinct text')
        case_ids.append(program['case_id'])
    runs, seen_keys, distinct_events, distinct_charged, completed = [], set(), set(), set(), {}
    for index, run in enumerate(observations['runs']):
        row = summarize_run(run, index)
        key = (row['key']['case_id'], row['key']['initial_fuel'])
        if key[0] not in case_ids or key in seen_keys:
            raise ValueError(f'run {index}: case membership or repeated (case_id, initial_fuel) key')
        seen_keys.add(key)
        runs.append(row)
        for record in run['records']:
            event_key = (key[0], digest(normalized_record(record)))
            distinct_events.add(event_key)
            if record['charge'] > 0:
                distinct_charged.add(event_key)
        if (not run['terminal_has_next'] and run['terminal']['output_k'].startswith('uiDone(')
                and row['publication_count'] > 0):
            completed.setdefault(key[0], row)
    prefix = dict(run_count=len(runs), **{name: sum(row[name] for row in runs) for name in (
        'record_count', 'charged_sum', 'charged_event_count', 'publication_count')})
    selected = [completed[case] for case in case_ids if case in completed]
    missing = [case for case in case_ids if case not in completed]
    full = dict(selection='first supplied no-next uiDone run with observed free publication per program',
                selected_run_indices=[row['run_index'] for row in selected],
                not_observed_case_ids=missing, observed_program_count=len(selected),
                record_count=None if missing else sum(row['record_count'] for row in selected),
                charged_sum=None if missing else sum(row['charged_sum'] for row in selected),
                observed_record_subtotal=sum(row['record_count'] for row in selected),
                observed_charged_subtotal=sum(row['charged_sum'] for row in selected))
    return dict(format=FORMAT, input_format=artifact['format'] if envelope else 'HOST-OBSERVATIONS',
                input_document_sha256=digest(artifact), observations_sha256=digest(observations),
                declarations={name: copy.deepcopy(observations[name]) for name in (
                    'producer', 'profile', 'source_sha256', 'boundary')},
                projection=copy.deepcopy(artifact['projection']) if envelope else None,
                programs=copy.deepcopy(programs), runs=runs,
                cross_run=dict(prefix_runs=prefix,
                    prefix_event_fingerprints=dict(observed=prefix['record_count'], distinct=len(distinct_events),
                                                  repeated=prefix['record_count']-len(distinct_events)),
                    charged_prefix_events=dict(observed=prefix['charged_event_count'], distinct=len(distinct_charged),
                                               repeated=prefix['charged_event_count']-len(distinct_charged)),
                    unique_program_full_traces=full),
                evidence=dict(scope='supplied-diagnostic-logical-accounting-only',
                              execution_authenticity='not-established', oracle_comparison='not-performed',
                              acceptance='none', terminal_has_next='preserved declaration; not evaluated'),
                physical_cost=dict(status='unmeasured', reason='no physical accounting evidence consumed'),
                economics=dict(status='unmeasured', reason='no DUST, fee or target evidence consumed'),
                helper_work=dict(status='unknown', reason='not a counter in the supplied A-profile snapshots'))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('input', type=Path, help='complete supplied HOST-OBSERVATIONS JSON or known envelope')
    args = parser.parse_args(argv)
    try:
        raw = args.input.read_bytes()
        report = summarize(json.loads(raw))
        report['input'] = dict(path=str(args.input), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
        print(json.dumps(report, indent=2, ensure_ascii=False, allow_nan=False))
    except (OSError, ValueError, TypeError, KeyError) as error:
        print(f'work-summary input error: {error}', file=sys.stderr)
        return 2
    return 0


if __name__ == '__main__':
    sys.exit(main())
