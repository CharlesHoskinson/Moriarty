#!/usr/bin/env python3
"""Pure full original Sherlock fixture comparison; never launch the subject.

Frozen prefix literals are expanded by the existing host_observations helper.
The supplied envelope is retained in the report; only its explicitly named
common semantic projections and fixture identity declarations are compared.
Text comparison is exact: the frozen check.py host convention has no normalizer.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

FREEZE = 'ROOT-FULL-FIXTURE-EXPECTATION-FREEZE-01.json'
FREEZE_SHA = '5646c018cfae26be384f0ea6f84576879bf5c93ae51870b4e4f73385c9f6bacc'
HELPERS = {
    'check.py': '7a6d1bbca881f5c2b012278989dac600834d610986694e640da51a40b5a412e0',
    'host_observations.py': '61b8258595a298231679362cb3974ae176b6970e7e4475388252d59da545fcdc',
}
FORMAT = 'sherlock-original-fixture-host-observations/1'
PRODUCER = 'sherlock-original-fixture-host-reference'
FAMILIES = (
    ('fixture-oracle-arithmetic-01', 'newprograms.json', 'prefixes.json'),
    ('fixture-oracle-data-01', 'fixture-normal-programs.json', 'fixture-normal-prefixes.json'),
    ('fixture-oracle-environment-01', 'fixture-normal-programs.json', 'fixture-normal-prefixes.json'),
)
PATH_TO_CASE = {
    'models/sherlock/k/fixtures/uiFactorial3.ui': 'F01',
    'models/sherlock/k/fixtures/uiSum3.ui': 'F02',
    'models/sherlock/k/fixtures/uiListsum123.ui': 'F03',
    'models/sherlock/k/fixtures/uiTreesum45.ui': 'F04',
    'models/sherlock/k/fixtures/uiConstructList.ui': 'F05',
    'models/sherlock/k/fixtures/uiBoolLetOrder.ui': 'F06',
    'models/sherlock/k/fixtures/uiCallerLocal.ui': 'F07',
    'models/sherlock/k/fixtures/uiMutualEven4.ui': 'F08',
}
SNAPSHOT_FIELDS = ('code_k', 'entry_k', 'control_k', 'environment_k',
                   'frames_top_first_k', 'depth', 'stack_limit', 'remaining_fuel',
                   'initial_fuel', 'output_k', 'charged_steps')


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def load_helpers(repo):
    modules = []
    for name, expected_sha in HELPERS.items():
        path = Path(repo) / 'models/sherlock/qa' / name
        raw = path.read_bytes()
        require(sha(raw) == expected_sha, f'pinned comparison helper changed: {name}')
        spec = importlib.util.spec_from_file_location('full_fixture_' + path.stem, path)
        module = importlib.util.module_from_spec(spec)
        exec(compile(raw, str(path), 'exec', dont_inherit=True), module.__dict__)
        modules.append(module)
    return tuple(modules)


def load_frozen(runtime, qa):
    """Rehash all 27 original pins before parsing any frozen source packet."""
    runtime = Path(runtime)
    raw = (runtime / FREEZE).read_bytes()
    require(sha(raw) == FREEZE_SHA, 'independent freeze manifest changed')
    # The pinned check.py strict reader supplies the existing JSON convention.
    _, freeze = qa.document(runtime / FREEZE, strict=True)
    for ref in freeze['files']:
        raw = (runtime / ref['path']).read_bytes()
        require(len(raw) == ref['bytes'] and sha(raw) == ref['sha256'],
                'frozen original changed: ' + ref['path'])
    def read(relative):
        return qa.document(runtime / relative, strict=True)[1]
    families = [(read(group + '/' + programs), read(group + '/' + prefixes))
                for group, programs, prefixes in FAMILIES]
    original_arithmetic = read('fixture-oracle-arithmetic-01/original-fixture-runs.json')
    return freeze, families, original_arithmetic


def original_trace(program, trace, arithmetic):
    """Resolve only each independent packet's explicit fuel1000 reconstruction."""
    case = program['case_id']
    if case in arithmetic:
        original = copy.deepcopy(arithmetic[case])
        summary = original['expected_terminal_summary']
        fuel, boundary, remaining, output, rule = (
            original['fixed_state_fields']['initial_fuel'],
            original['publication']['pre_state_index'], summary['remaining_fuel'],
            summary['output_k'], original['publication']['rule_start_line'])
        require(original['input_sha256'] == program['input_sha256'], 'original input reference changed')
        require(original['original_fixture'] == program['original_fixture'], 'original fixture reference changed')
    else:
        original = copy.deepcopy(trace)
        overlay = trace['original_fixture_run']
        if 'state_fuel_overlays' in overlay:
            fuel = overlay['fixed_state_overrides']['initial_fuel']
            original['fixed_state_fields'].update(overlay['fixed_state_overrides'])
            original['initial_configuration'] = copy.deepcopy(overlay['initial_configuration'])
            require(overlay['source_path'] == program['original_fixture']['path'] and
                    overlay['source_sha256'] == program['original_fixture']['sha256'],
                    'original overlay fixture identity changed')
            require(len(overlay['state_fuel_overlays']) == len(original['states']), 'original overlay state membership')
            for index, (state, row) in enumerate(zip(original['states'], overlay['state_fuel_overlays'], strict=True)):
                require(row[0] == index, 'original overlay state order')
                state['remaining_fuel'] = row[1]
            original['publication'] = copy.deepcopy(overlay['publication'])
            boundary = overlay['publication']['pre_state_index']
            remaining, output = overlay['terminal']['remaining_fuel'], overlay['terminal']['output_k']
            rule = overlay['publication']['rule_start_line']
        else:
            fuel, boundary, remaining, output, rule = (
                overlay['initial_fuel'], overlay['boundary_state_index'], overlay['remaining_fuel'],
                overlay['published_output_k'], overlay['publication_rule_start_line'])
            require(overlay['consumed_work'] == boundary, 'original boundary work changed')
            original['fixed_state_fields'].update(initial_fuel=fuel, stack_limit=overlay['stack_limit'])
            for state in original['states']:
                state['remaining_fuel'] = fuel - state['charged_steps']
    require(fuel == 1000 and original['fixed_state_fields']['stack_limit'] == 64,
            'original source budget must remain fuel1000 stack64')
    require(boundary == len(original['states']) - 1, 'original complete state membership changed')
    for state in original['states']:
        require(state['remaining_fuel'] == fuel - state['charged_steps'], 'original literal fuel inconsistent')
    original['ordinary_fuel_prefixes'] = [[fuel, boundary, remaining, output, rule]]
    return original


def prepare(repo, runtime):
    qa, host = load_helpers(repo)
    freeze, families, arithmetic_doc = load_frozen(runtime, qa)
    arithmetic = {row['case_id']: row for row in arithmetic_doc['traces']}
    programs = [row for inputs, _ in families for row in inputs['programs']]
    traces = [row for _, prefixes in families for row in prefixes['traces']]
    require(len(programs) == len(traces) == 8, 'full eight-fixture membership required')
    mapped, original_families, identities = [], [], []
    frozen_cases = {row['case_id']: row for row in freeze['cases']}
    for program, trace in zip(programs, traces, strict=True):
        case, fixture = program['case_id'], program['original_fixture']
        require(trace['case_id'] == case and trace['input_sha256'] == program['input_sha256'], 'prefix input reference changed')
        frozen_case = frozen_cases[case]
        require(frozen_case['fixture'] == fixture['path'] and frozen_case['input_sha256'] == program['input_sha256'], 'frozen case identity changed')
        require(program['exact_completion_fuel'] == frozen_case['charged_edges_specified'] and
                len(trace['ordinary_fuel_prefixes']) == frozen_case['ordinary_prefixes_specified'], 'frozen prefix membership changed')
        require(sha((Path(repo) / fixture['path']).read_bytes()) == fixture['sha256'], 'original fixture bytes changed')
        mapped_case = PATH_TO_CASE[fixture['path']]
        mapped.append(dict(original_case_id=case, host_case_id=mapped_case,
                           fixture_path=fixture['path'], fixture_sha256=fixture['sha256'],
                           input_sha256=program['input_sha256'],
                           ordinary_prefix_fuels=[row[0] for row in trace['ordinary_fuel_prefixes']]))
        original = original_trace(program, trace, arithmetic)
        original_families.append((dict(profile=families[0][0]['profile'], source=families[0][0]['source'], programs=[program]),
                                  dict(traces=[original])))
        identities.append(dict(case_id=mapped_case, fixture_path=fixture['path'],
                               declared_fixture_sha256=fixture['sha256'], original_fuel=1000, stack_limit=64))
    require([row['fixture_path'] for row in mapped] == list(PATH_TO_CASE), 'fixture path membership/order changed')
    require(sha((Path(repo) / qa.SOURCE['path']).read_bytes()) == qa.SOURCE['sha256'], 'pinned K source changed')
    ordinary = host.expected_observations(families, producer=PRODUCER)
    originals = host.expected_observations(original_families, producer=PRODUCER)['runs']
    aliases = {row['original_case_id']: row['host_case_id'] for row in mapped}
    for row in ordinary['programs'] + ordinary['runs'] + originals:
        row['case_id'] = aliases[row['case_id']]
    require(len(ordinary['runs']) == 569 and len(originals) == 8, 'complete comparison membership changed')
    return qa, host, dict(observations=ordinary, original_fixture_runs=originals,
                         fixture_identities=identities), mapped


def compare_prepared(expected, artifact, qa, host, membership):
    """Exact complete arrays; never select a common subset or observed fuel extent."""
    qa.fields(artifact, ('format', 'observations', 'original_fixture_runs', 'projection'), 'fixture artifact')
    require(artifact['format'] == FORMAT, 'fixture artifact: format mismatch')
    qa.fields(artifact['projection'], ('inputs',), 'fixture projection')
    qa.require(type(artifact['projection']['inputs']) is list, 'fixture inputs must be array')
    identity_keys = ('case_id', 'fixture_path', 'declared_fixture_sha256', 'original_fuel', 'stack_limit')
    identities = []
    for row in artifact['projection']['inputs']:
        qa.fields(row, identity_keys, 'fixture input')
        identities.append({key: row[key] for key in identity_keys})
    difference = qa.first_difference(expected['fixture_identities'], identities, '/fixture_identities')
    if difference is None:
        difference = host.compare_observations(expected['observations'], artifact['observations'], qa.first_difference)
        if difference is not None:
            difference['comparison_group'] = 'ordinary-prefixes'
    original_delta = host.compare_observations(dict(runs=expected['original_fixture_runs']),
                                              dict(runs=artifact['original_fixture_runs']), qa.first_difference)
    if difference is None and original_delta is not None:
        difference = original_delta
        difference['comparison_group'] = 'original-fixture-runs'
    if difference is not None:
        case = difference.get('case_id')
        difference['fixture_membership'] = next((row for row in membership if row['host_case_id'] == case), None)
    return dict(status='mismatch' if difference else 'match', acceptance='none',
                scope='complete-supplied-full-original-fixture-host-diagnostic-comparison-only',
                first_difference=difference, expected_ordinary_prefix_runs=569,
                expected_original_fixture_runs=8, membership=membership,
                snapshot_fields=list(SNAPSHOT_FIELDS), normalization='none; exact frozen check.py host text convention',
                semantic_selection=['observations (entire object)', 'original_fixture_runs (entire array)',
                                    'projection.inputs: case_id, fixture_path, declared_fixture_sha256, original_fuel, stack_limit'],
                retained_diagnostics={key: value for key, value in artifact.items()
                                      if key not in ('observations', 'original_fixture_runs')},
                subject_execution=False, producer_authenticity='not-established',
                constructor_startup='outside-post-startup-comparison',
                execution_evidence='not-established-by-this-consumer')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, required=True)
    parser.add_argument('--observations', type=Path, help='supplied capture; never launched')
    args = parser.parse_args(argv)
    try:
        qa, host, expected, membership = prepare(args.repo, args.runtime)
        if args.observations is None:
            report = dict(status='prepared-only', acceptance='none', subject_execution=False,
                          actual_capture_read=False, expected_ordinary_prefix_runs=569,
                          expected_original_fixture_runs=8, membership=membership,
                          helper_sha256=HELPERS, freeze_sha256=FREEZE_SHA,
                          normalization='none; exact frozen check.py host text convention')
        else:
            ref, artifact = qa.document(args.observations, strict=True)
            report = compare_prepared(expected, artifact, qa, host, membership)
            report.update(supplied_file=ref, helper_sha256=HELPERS, freeze_sha256=FREEZE_SHA)
        print(json.dumps(report, indent=2, allow_nan=False))
        return 1 if report['status'] == 'mismatch' else 0
    except (OSError, ValueError, KeyError, TypeError, IndexError, RecursionError) as error:
        parser.exit(2, f'Full fixture input error: {error}\n')


if __name__ == '__main__':
    sys.exit(main())
