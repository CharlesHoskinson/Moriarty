#!/usr/bin/env python3
"""Validate frozen Sherlock normal literals and compare existing literal documents.

File/JSON operations and source transcription only. No interpreter, model,
compiler, SDK, network, subprocess, proof or ledger execution occurs here.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import re
from pathlib import Path
import sys

PROFILE = 'sherlock-k-a/1'
SOURCE = {'path': 'models/sherlock/k/main.k',
          'sha256': 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'}
FROZEN = {
    'normal-programs.json': 'ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f',
    'normal-prefixes.json': 'dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d',
    'extra-normal-programs.json': 'dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153',
    'extra-normal-prefixes.json': 'acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16',
}
SERIALIZER = {'path': 'models/sherlock/reference/check-reference.py',
              'sha256': '1b22a0c4dc14ac0dabd5fe4e8c5ca5ff85485e9b22dd2673a4645f5145cfa94b'}
FAMILIES = {'normal': ('N01', 'N02', 'N03', 'N04', 'N05', 'N06'),
            'extra-normal': ('N07', 'N08')}
STATE_FIELDS = ('charged_steps', 'remaining_fuel', 'depth', 'control_k',
                'environment_k', 'frames_top_first_k', 'via_rule_start_line')
INITIAL_FIELDS = ('control_k', 'environment_k', 'frames_top_first_k', 'depth',
                  'stack_limit', 'code_k', 'remaining_fuel', 'initial_fuel',
                  'output_k', 'charged_steps')
LAYER_ARTIFACTS = {
    'k': ('sherlock-k-normal-observations/1', 'sherlock-k-reference',
          ('raw_runs', 'evidence', 'comparison_ready')),
    'lean': ('sherlock-lean-normal-observations/1', 'sherlock-lean-reference', ('raw_runs',)),
    'quint': ('sherlock-quint-itf-observations/1', 'sherlock-quint-k-profile',
              ('raw_traces', 'raw_trace_references', 'evidence')),
    'image': ('sherlock-image-observations/1', 'sherlock-image-reference', ()),
}


# Additive selected-run metadata. This is a declared projection, not execution
# evidence or an additional model. Full-corpus constants and modes stay unchanged.
PAIRED_PROJECTION_CONTRACT = {'comparisonProfile': 'sherlock-k-a/1',
 'referenceSource': {'path': 'models/sherlock/k/main.k',
                     'sha256': 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41',
                     'bytes': 16249},
 'commonDomain': {'input': 'Finite closed immutable K-compatible syntax/function tables, acyclic '
                           'expression references, named recursive functions, canonical signed-decimal '
                           'names, UInt128/Bool/List[UInt128]/structuralTree, finite fuel/stack. K-A '
                           'counterpart rather than older finite-algorithm Quint-A.',
                  'numericBounds': {'UInt128min': '0',
                                    'UInt128max': '340282366920938463463374607431768211455',
                                    'F03initialFuel': 93,
                                    'F03stackLimit': 64,
                                    'F03expectedLogicalFramePeak': 11,
                                    'F03literalListLength': 3,
                                    'programIdentity': 'F03 '
                                                       'input_sha256832f1c6521477d052b1021cb7cff25c2a7da3b807e6374243479e7b3126e4100'},
                  'notClaims': ['Fullnumericspace exhaustively checked',
                                'Wholegrammar/allprogram refinement',
                                'General arbitrary Snapshot admission',
                                'Physical heap/native-stack/time guarantee']},
 'fieldMaps': [{'commonField': 'code_k / entry_k',
                'K': 'observed <code> plus retained input entry',
                'Quint': 'finite immutable expression/function tables expanded by _Renderer; named '
                         'calls remain named, never unrolled',
                'difference': 'ExprId sharing and unused rows retained in rawITF; Ktext is structural '
                              'diagnostic, not pointer/codec identity'},
               {'commonField': 'control_k',
                'K': 'observed <k> after startup',
                'Quint': 'all supported Control variants render through _Renderer.control',
                'difference': 'No typed K prestartup↔Quint initialized constructor correspondence '
                              'claimed'},
               {'commonField': 'environment_k',
                'K': 'observed Int-key map',
                'Quint': 'raw int-key map rendered in canonical decimal-string lexical key order',
                'difference': 'Raw map encoding/order retained; iteration order not semantic'},
               {'commonField': 'frames_top_first_k / depth',
                'K': '<stack>/<depth> retained, including all saved environments and pending '
                     'return/multiplication/args/scope',
                'Quint': 'topfirst Frame list, full payload rendering; depth derived from listlength',
                'difference': 'No frame payload or caller environment dropped'},
               {'commonField': 'stack_limit',
                'K': '<stackLimit>',
                'Quint': 'machine.stackLimit',
                'difference': 'Exact selected finiteinput64 for F03; no physical/native stack claim'},
               {'commonField': 'initial_fuel / remaining_fuel / charged_steps',
                'K': '<initialFuel>/<fuel>; work initial-minus-remaining',
                'Quint': 'machine.initial/remaining; derivedwork',
                'difference': 'K-A schedule selected on both; helpers unmetered; no transfer to frozen '
                              'Quint-A terminaltick profile'},
               {'commonField': 'output_k',
                'K': '<out> Pending/Done/Rejected/Exhausted',
                'Quint': 'Outcome rendering retains same K-A diagnostic forms',
                'difference': 'No financial segment commit/duty result added'},
               {'commonField': 'records.rule_start_line / charge / before / after',
                'K': 'actual DebugAppliedRewriteRules span matched to compiled inventory +observed fuel '
                     'delta +complete cells',
                'Quint': 'raw lastRule/lastCharge on actual step; complete adjacent snapshots',
                'difference': 'No rule inferred from expected states or borrowed host execution'},
               {'commonField': 'terminal_has_next',
                'K': 'actual separate terminal no-rule/unchanged-state probe under recorded capture '
                     'convention',
                'Quint': 'no further K-A semantic edge after publication; actual total action may '
                         'remain enabled for rule0/charge0stutter',
                'difference': 'Distinct enabledness interpretations declared; rawstutters retained and '
                              'counted, not deleted without explanation'}],
 'ruleToAction': 'For each actually observed K semantic rule span/startline, compare the Quint step '
                 'transition raw lastRule with that same pinned rule line and lastCharge with observed '
                 'charge. step dispatches through generic '
                 'advance/evaluate/return/invoke/bind/cases/primitive branches; no case-specific '
                 'interpreter. Startup74 is model initialization metadata, not an observed Quint '
                 'startup action.',
 'hiddenSteps': ['K pure helper/function normalization is inside its A charged semantic rule; '
                 'visits/work remain unmeasured, not0. Raw engine simplifications/rewrites are retained '
                 'separately from selected semantic boundaries.',
                 'Quint pure helper folds/rendering do not add A charges. ExprId table sharing expands '
                 'only for diagnostic text.',
                 'Quint postpublication rule0/charge0 stutters remain raw and are explicitly excluded '
                 'from K-A semantic records; no disabledness probe fabricated.',
                 'KgeneratedCounter and prestartupKore are retained raw but not in nine-cell typed '
                 'common projection.'],
 'unmatchedFinanceWait': 'Chosen pure K-A grammar has no financial working '
                         'store/effects/duties/await/resume cells. Older universalMachine localwaiting '
                         'does not establish a Kfinancial counterpart; no mapping is claimed for it. '
                         'Bfinancial/helper state and newer interpreter-chunk models are separate '
                         'retained obligations.',
 'reportDisposition': 'FINITE_ACTUAL_TRACE_AGREEMENT_ONLY when actual pair matches; semantic '
                      'refinement/proof/native/target NONE. Producer origin comes from separately '
                      'retained actual receipts, not adapter hashes alone.'}
F03_ORACLE_ROOT = 'docs/audits/2026-10-16/sherlock-full-original-fixtures/frozen/fixture-oracle-data-01'
F03_ORACLE_PINS = {
    'fixture-normal-programs.json': '71e780f207c862e6b7061b9b24cb51c3f5243874e256442d46d009e9fc937113',
    'fixture-normal-prefixes.json': '13d00607c8eac53d6d607502e00395b50f6a2f2a9be386f2ba8bbc5bb167a49b',
}
SELECTED_CASES = tuple(case for members in FAMILIES.values() for case in members) + ('F03',)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def document(path, *, strict=False, inventory=None, reference_path=None):
    raw = Path(path).read_bytes()
    if inventory is not None:
        inventory.record(reference_path if reference_path is not None else str(path), raw)
    value = (json.loads(raw, object_pairs_hook=unique_members, parse_constant=nonfinite,
                        parse_float=finite) if strict else json.loads(raw))
    return {'path': str(path), 'sha256': digest(raw)}, value


def require(condition, message):
    if not condition:
        raise ValueError(message)


def unique_members(pairs):
    """Supplied observations may not hide a member behind a later duplicate."""
    value = {}
    for key, member in pairs:
        require(key not in value, f'duplicate JSON member: {key!r}')
        value[key] = member
    return value


def nonfinite(token):
    raise ValueError(f'nonfinite JSON number: {token}')


def finite(token):
    value = float(token)
    require(math.isfinite(value), f'nonfinite JSON number: {token}')
    return value


def fields(value, names, location):
    require(type(value) is dict, f'{location}: expected object')
    for name in names:
        require(name in value, f'{location}: missing {name}')


def integer(value, location):
    require(type(value) is int and value >= 0, f'{location}: expected nonnegative integer')


def text(value, location):
    require(type(value) is str and bool(value), f'{location}: expected nonempty literal text')


def validate_pair(programs, prefixes, *, family):
    """Check container consistency, never evaluate a program or a K rule."""
    require(family in FAMILIES, 'unknown corpus family')
    for label, doc, member, fmt in (
            ('programs', programs, 'programs', 'sherlock-literal-normal-inputs/1'),
            ('prefixes', prefixes, 'traces', 'sherlock-literal-normal-prefixes/1')):
        fields(doc, ('format', 'profile', 'source', member), label)
        require(doc['format'] == fmt, f'{label}: format mismatch')
        require(doc['profile'] == PROFILE, f'{label}: profile mismatch')
        require(doc['source'] == SOURCE, f'{label}: source identity mismatch')
        require(type(doc[member]) is list, f'{label}: membership must be an array')
        require(all(type(row) is dict for row in doc[member]), f'{label}: membership row must be object')
        require([row.get('case_id') for row in doc[member]] == list(FAMILIES[family]),
                f'{label}: case membership/order mismatch')
    counts = dict(programs=0, state_rows=0, charged_edges=0, publications=0, prefix_rows=0)
    for program, trace in zip(programs['programs'], prefixes['traces']):
        case = program['case_id']
        fields(program, ('case_id', 'code_k', 'entry_k', 'initial_arguments', 'stack_limit',
                         'exact_completion_fuel', 'input_sha256', 'expected_value_k'), case)
        source_keys = ('case_id', 'code_k', 'entry_k', 'initial_arguments', 'stack_limit', 'exact_completion_fuel')
        source_digest = digest(json.dumps({key: program[key] for key in source_keys},
                                         sort_keys=True, separators=(',', ':')).encode())
        require(source_digest == program['input_sha256'], f'{case}: input digest mismatch')
        for key in ('code_k', 'entry_k', 'initial_arguments', 'expected_value_k'):
            text(program[key], f'{case}.{key}')
        exact = program['exact_completion_fuel']
        integer(exact, f'{case}.exact_completion_fuel')
        integer(program['stack_limit'], f'{case}.stack_limit')
        fields(trace, ('case_id', 'input_sha256', 'fixed_state_fields', 'initial_configuration',
                       'startup', 'states', 'publication', 'ordinary_fuel_prefixes'), case)
        fixed = trace['fixed_state_fields']
        fields(fixed, ('code_ref', 'initial_fuel', 'stack_limit', 'output_k'), case + '.fixed')
        require(trace['input_sha256'] == program['input_sha256'], f'{case}: input reference mismatch')
        require(fixed['code_ref'] == dict(file=family + '-programs.json', case_id=case,
                                        input_sha256=program['input_sha256']), f'{case}: code reference mismatch')
        require(fixed['initial_fuel'] == exact and type(fixed['initial_fuel']) is int,
                f'{case}: initial fuel mismatch')
        require(fixed['stack_limit'] == program['stack_limit'] and type(fixed['stack_limit']) is int,
                f'{case}: stack limit mismatch')
        require(fixed['output_k'] == 'uiPending()', f'{case}: pending output mismatch')
        initial = trace['initial_configuration']
        fields(initial, INITIAL_FIELDS, case + '.initial_configuration')
        require(initial == dict(control_k=f"uiRun({program['code_k']}, {program['entry_k']}, {exact}, {program['stack_limit']})",
                                environment_k='.Map', frames_top_first_k=[], depth=0, stack_limit=0,
                                code_k='uiNoFuns()', remaining_fuel=0, initial_fuel=0,
                                output_k='uiPending()', charged_steps=0), f'{case}: initial configuration mismatch')
        require(trace['startup'] == dict(rule_start_line=74, charge=0, post_state_index=0),
                f'{case}: startup mismatch')
        states = trace['states']
        require(type(states) is list and len(states) == exact + 1, f'{case}: state count/work mismatch')
        for index, state in enumerate(states):
            location = f'{case}.states[{index}]'
            fields(state, STATE_FIELDS, location)
            for name in ('charged_steps', 'remaining_fuel', 'depth'):
                integer(state[name], location + '.' + name)
            require(state['charged_steps'] == index, location + ': charged state order mismatch')
            require(state['remaining_fuel'] == exact - index, location + ': remaining fuel/work mismatch')
            require(type(state['frames_top_first_k']) is list, location + ': frames must be array')
            require(state['depth'] == len(state['frames_top_first_k']), location + ': depth mismatch')
            require(state['depth'] <= program['stack_limit'], location + ': depth exceeds stack limit')
            for frame in state['frames_top_first_k']:
                text(frame, location + '.frame')
            text(state['control_k'], location + '.control_k')
            text(state['environment_k'], location + '.environment_k')
            if index == 0:
                require(state['via_rule_start_line'] is None, location + ': initial incoming rule must be absent')
                require(state['control_k'] == f"uiEval({program['entry_k']})", location + ': initial control mismatch')
            else:
                integer(state['via_rule_start_line'], location + '.via_rule_start_line')
                require(state['via_rule_start_line'] > 0, location + ': missing charged rule')
        publication = trace['publication']
        fields(publication, ('pre_state_index', 'rule_start_line', 'charge', 'post_overrides'), case + '.publication')
        require(publication == dict(pre_state_index=exact, rule_start_line=80, charge=0,
                                    post_overrides=dict(control_k='.K', output_k=f"uiDone({program['expected_value_k']})")),
                f'{case}: publication mismatch')
        require(states[-1]['control_k'] == f"uiRet({program['expected_value_k']})" and states[-1]['depth'] == 0,
                f'{case}: publication predecessor mismatch')
        rows = trace['ordinary_fuel_prefixes']
        require(type(rows) is list and len(rows) == exact + 2, f'{case}: prefix coverage mismatch')
        for fuel, row in enumerate(rows):
            require(type(row) is list and len(row) == 5, f'{case}: prefix columns mismatch')
            for column in (0, 1, 2, 4):
                integer(row[column], f'{case}.prefix[{fuel}][{column}]')
            boundary = min(fuel, exact)
            outcome = f"uiDone({program['expected_value_k']})" if fuel >= exact else 'uiExhausted()'
            require(row == [fuel, boundary, fuel - boundary, outcome, 80 if fuel >= exact else 84],
                    f'{case}: prefix work/publication mismatch at fuel {fuel}')
        counts['programs'] += 1
        counts['state_rows'] += len(states)
        counts['charged_edges'] += len(states) - 1
        counts['publications'] += 1
        counts['prefix_rows'] += len(rows)
    return counts


def pointer(path, key):
    return path + '/' + str(key).replace('~', '~0').replace('/', '~1')


def first_difference(expected, observed, path=''):
    """Structural equality with exact types, all fields and ordered arrays."""
    def delta(kind, left=expected, right=observed, at=path, **extra):
        return dict(path=at, kind=kind, expected=left, observed=right, **extra)
    if type(expected) is not type(observed):
        return delta('type')
    if type(expected) is dict:
        for key in list(expected) + [key for key in observed if key not in expected]:
            at = pointer(path, key)
            if key not in expected or key not in observed:
                return delta('member', expected.get(key), observed.get(key), at,
                             expected_present=key in expected, observed_present=key in observed)
            found = first_difference(expected[key], observed[key], at)
            if found is not None:
                return found
    elif type(expected) is list:
        for index, (left, right) in enumerate(zip(expected, observed)):
            found = first_difference(left, right, pointer(path, index))
            if found is not None:
                return found
        if len(expected) != len(observed):
            return delta('array-length', common_entries=min(len(expected), len(observed)))
    elif expected != observed:
        return delta('value')
    return None


def state_at(trace, index):
    """Only an in-range nonnegative int selects a state; bool or negative never does."""
    states = trace['states']
    if type(states) is not list or type(index) is not int or not 0 <= index < len(states):
        raise IndexError('unusable supplied state index')
    return states[index]


def observation_state(programs, trace, parts):
    """Overlay only the corpus's documented literal fields for diagnostics.

    No rule is evaluated. Missing or malformed supplied components leave the
    reconstructed diagnostic unavailable; the original raw trace is retained.
    """
    if type(trace) is not dict or type(programs) is not dict:
        return None
    program = next((p for p in programs.get('programs', [])
                    if type(p) is dict and p.get('case_id') == trace.get('case_id')), None)
    if program is None:
        return None
    try:
        field = parts[4] if len(parts) > 4 else None
        if field == 'initial_configuration':
            return trace['initial_configuration']
        if field == 'startup':
            state = state_at(trace, 0)
        elif field == 'states' and len(parts) > 5 and parts[5].isdecimal():
            state = state_at(trace, int(parts[5]))
        elif field == 'publication':
            state = state_at(trace, trace['publication']['pre_state_index'])
        elif field == 'ordinary_fuel_prefixes' and len(parts) > 5 and parts[5].isdecimal():
            row = trace['ordinary_fuel_prefixes'][int(parts[5])]
            state = state_at(trace, row[1])
        else:
            return None
        result = dict(trace['fixed_state_fields'], **state)
        result.update(code_k=program['code_k'], entry_k=program['entry_k'])
        if field == 'publication':
            result.update(trace['publication']['post_overrides'])
        elif field == 'ordinary_fuel_prefixes':
            result.update(initial_fuel=row[0], remaining_fuel=row[2], control_k='.K', output_k=row[3])
        return result
    except (KeyError, IndexError, TypeError, ValueError):
        return None


def compare_documents(expected_programs, expected_prefixes, observed_programs, observed_prefixes):
    """Compare the existing literal documents, including every supplied member.

    This is literal readback comparison, not an executable-layer adapter. No
    formatting string is interpreted as an authenticated machine snapshot.
    """
    require(observed_programs is not None and observed_prefixes is not None,
            'absent observation: both existing program/prefix documents are required')
    pairs = (('programs', expected_programs, observed_programs),
             ('prefixes', expected_prefixes, observed_prefixes))
    # Contract order: identity precedes state inspection (no Rust comparator claim).
    for name in ('profile', 'source'):
        for label, expected, observed in pairs:
            if type(observed) is not dict or name not in observed:
                return first_difference({name: expected[name]}, observed, '/' + label)
            found = first_difference(expected[name], observed[name], f'/{label}/{name}')
            if found is not None:
                return found
    for label, expected, observed in pairs:
        found = first_difference(expected, observed, '/' + label)
        if found is None:
            continue
        parts = found['path'].split('/')
        if label == 'prefixes' and len(parts) > 3 and parts[2] == 'traces' and parts[3].isdecimal():
            index = int(parts[3])
            left = expected['traces'][index] if index < len(expected['traces']) else None
            right = observed.get('traces', [])
            right = right[index] if type(right) is list and index < len(right) else None
            found.update(case_id=left.get('case_id') if left else None,
                         expected_trace=left, observed_trace=right)
            if len(parts) > 5 and parts[4] in ('states', 'ordinary_fuel_prefixes') and parts[5].isdecimal():
                found['state_index' if parts[4] == 'states' else 'prefix_index'] = int(parts[5])
            expected_state = observation_state(expected_programs, left, parts)
            observed_state = observation_state(observed_programs, right, parts)
            found.update(expected_state=expected_state, observed_state=observed_state)
            if left is not None and len(parts) > 4:
                if parts[4] == 'states' and 'state_index' in found:
                    found['rule_start_line'] = left['states'][found['state_index']]['via_rule_start_line']
                elif parts[4] == 'publication':
                    found['rule_start_line'] = left['publication']['rule_start_line']
                elif parts[4] == 'startup':
                    found['rule_start_line'] = left['startup']['rule_start_line']
                elif parts[4] == 'ordinary_fuel_prefixes' and 'prefix_index' in found:
                    found['rule_start_line'] = left['ordinary_fuel_prefixes'][found['prefix_index']][4]
        return found
    return None


def check_rust_transcription(repo, programs, prefixes, family, *, read_text=None, inventory=None):
    """Call the pinned existing serializer only, never its rustc/test entry point.

    Its bytes are hashed before any execution and exactly those verified bytes
    are compiled, so neither a later file swap nor cached bytecode runs. Its
    imports and the interpreter remain unpinned: the checkout must be trusted.
    """
    helper = repo / SERIALIZER['path']
    helper_raw = helper.read_bytes()
    if inventory is not None:
        inventory.record(SERIALIZER['path'], helper_raw)
    require(digest(helper_raw) == SERIALIZER['sha256'], f'{helper}: serializer digest mismatch; not executed')
    spec = importlib.util.spec_from_file_location('sherlock_literal_serializer', helper)
    require(spec is not None and spec.loader is not None, 'serializer unavailable')
    module = importlib.util.module_from_spec(spec)
    exec(compile(helper_raw, str(helper), 'exec', dont_inherit=True), module.__dict__)
    extra = family == 'extra-normal'
    expected = module.serialize_oracles(programs, prefixes, case_ids=FAMILIES[family],
                                        programs_file=family + '-programs.json',
                                        constant_prefix='EXTRA_' if extra else '')
    path = repo / 'crates/mori-core/src/sherlock' / ('extra_normal_oracles.rs' if extra else 'normal_oracles.rs')
    actual = read_text(path).encode('utf-8') if read_text is not None else path.read_bytes()
    if inventory is not None:
        inventory.record(str(path.relative_to(repo)), actual)
    require(actual == expected.encode('utf-8'), f'{path}: committed Rust literal transcription differs from frozen JSON')
    return dict(path=str(path.relative_to(repo)), sha256=digest(actual),
                serializer_sha256=digest(helper_raw), status='literal-transcription-match',
                scope='source-literals-only; Rust implementation not executed')


def host_adapter():
    """Load the known sibling source even when Python -I excludes script paths."""
    path = Path(__file__).resolve().with_name('host_observations.py')
    raw = path.read_bytes()
    spec = importlib.util.spec_from_file_location('sherlock_host_observations', path)
    require(spec is not None and spec.loader is not None, 'host observation adapter unavailable')
    module = importlib.util.module_from_spec(spec)
    exec(compile(raw, str(path), 'exec', dont_inherit=True), module.__dict__)
    return module


def layer_raw_context(artifact, difference):
    """Retain supplied diagnostics for the differing expected case and fuel.

    These fields are diagnostic declarations, never another source of expected
    states. A reference or raw Repr/ITF value is not an execution receipt.
    """
    case, fuel = difference.get('case_id'), difference.get('initial_fuel')
    if artifact.get('format') == 'sherlock-k-normal-observations/1':
        # Select by the expected run's case and exact int fuel only: a supplied
        # bool or float fuel never aliases an int, as in state_at. Cell and log
        # text is retained as supplied, never decoded or treated as a receipt.
        rows = artifact.get('raw_runs')
        candidates = []
        if type(rows) is list and type(case) is str and type(fuel) is int:
            for index, row in enumerate(rows):
                if type(row) is not dict or type(row.get('observation')) is not dict:
                    continue
                observed = row['observation']
                initial = observed.get('initial')
                if (observed.get('case_id') == case and type(initial) is dict
                        and type(initial.get('initial_fuel')) is int
                        and initial['initial_fuel'] == fuel):
                    candidates.append(index)
        if len(candidates) != 1:
            return dict(raw_context_status='multiple' if candidates else 'unavailable',
                        raw_run_candidate_indices=candidates, case_id=case, initial_fuel=fuel)
        index = candidates[0]
        row = rows[index]
        context = dict(raw_context_status='available', raw_run_index=index,
                       case_id=case, initial_fuel=fuel, raw_run=row)
        for key in ('raw_startup_probe', 'raw_terminal_probe'):
            if key in row:
                context[key] = row[key]
        captures = row.get('raw_captures')
        parts = difference.get('path', '').split('/')
        field = parts[3] if len(parts) > 3 and parts[1] == 'runs' else None
        record = difference.get('record_index')
        if (field == 'records' and type(record) is int and type(captures) is list
                and 0 <= record < len(captures) - 1):
            context.update(raw_before_capture=captures[record],
                           raw_after_capture=captures[record + 1])
            logs = row.get('decoded_incoming_logs')
            if type(logs) is list and record < len(logs):
                context.update(incoming_rule_log=logs[record], incoming_rule_log_status='available')
            else:
                context['incoming_rule_log_status'] = 'unavailable'
        elif field in ('initial', 'terminal') and type(captures) is list and captures:
            context['raw_capture'] = captures[0 if field == 'initial' else -1]
        else:
            context['raw_state_status'] = 'unavailable'
        return context
    if artifact.get('format') == 'sherlock-quint-itf-observations/1':
        # Inspect only the existing ITF Snapshot fields. No model execution or
        # projection recomputation participates in this diagnostic lookup.
        def machine(state):
            if type(state) is not dict:
                return None
            keys = [key for key in state if key == 'machine' or key.endswith('::machine')]
            return state[keys[0]] if len(keys) == 1 and type(state[keys[0]]) is dict else None

        rows = artifact.get('raw_traces')
        candidates = []
        if type(rows) is list and case is not None and type(fuel) is int:
            for index, row in enumerate(rows):
                if type(row) is not dict or row.get('case_id') != case:
                    continue
                try:
                    initial = machine(row['itf']['states'][0])['initial']
                    if type(initial) is dict and int(initial['#bigint']) == fuel:
                        candidates.append(index)
                except (KeyError, IndexError, TypeError, ValueError):
                    continue
        if len(candidates) != 1:
            return dict(raw_context_status='multiple' if candidates else 'unavailable',
                        raw_trace_candidate_indices=candidates,
                        case_id=case, initial_fuel=fuel)

        index = candidates[0]
        row = rows[index]
        states = row['itf']['states']
        context = dict(raw_context_status='available', raw_trace_index=index,
                       case_id=case, initial_fuel=fuel, raw_trace=row)
        references = artifact.get('raw_trace_references')
        if (type(references) is list and index < len(references)
                and type(references[index]) is dict and references[index].get('case_id') == case):
            context['raw_trace_reference'] = references[index]
            context['raw_trace_reference_status'] = 'available'
        else:
            context['raw_trace_reference_status'] = 'unavailable'

        # Only post-publication rule-0 stutters are omitted from K-A records.
        # The actual publication is the first raw non-Pending outcome, so a
        # terminal diagnostic never accidentally substitutes its later stutter.
        publication = next((i for i, state in enumerate(states)
                            if type(machine(state)) is dict
                            and type(machine(state).get('outcome')) is dict
                            and machine(state)['outcome'].get('tag') in ('Done', 'Rejected', 'Exhausted')), None)
        parts = difference.get('path', '').split('/')
        field = parts[3] if len(parts) > 3 and parts[1] == 'runs' else None
        record = difference.get('record_index')
        if field == 'records' and type(record) is int and publication is not None and 0 <= record < publication:
            context.update(raw_before_state=states[record], raw_after_state=states[record + 1],
                           raw_state_indices=dict(before=record, after=record + 1))
        elif field in ('initial', 'terminal'):
            state_index = 0 if field == 'initial' else publication
            if state_index is not None:
                context.update(raw_state=states[state_index], raw_state_indices={field: state_index})
            else:
                context['raw_state_status'] = 'unavailable'
        elif field == 'records':
            context['raw_state_status'] = 'unavailable'
        return context
    context = {}
    projection = artifact['projection']
    def matching(rows, *, prefix=False):
        if type(rows) is not list or case is None:
            return None
        return next((row for row in rows if type(row) is dict and row.get('case_id') == case
                     and (not prefix or row.get('initial_fuel') == fuel)), None)
    raw_runs = artifact.get('raw_runs')
    if raw_runs is None and type(projection) is dict:
        raw_runs = projection.get('runs')
    for label, rows, prefix in (
            ('raw_run', raw_runs, True),
            ('raw_trace', artifact.get('raw_traces'), False),
            ('raw_trace_reference', artifact.get('raw_trace_references'), False),
            ('program_image', projection.get('program_images') if type(projection) is dict else None, False)):
        row = matching(rows, prefix=prefix)
        if row is not None:
            context[label] = row
    return context


def compare_supplied_observations(artifact, families, *, layer, selection=None):
    """Compare a declared adapter envelope with independently frozen literals.

    Only the existing common observation projection is compared. Raw machine
    diagnostics and projection limitations are retained without interpreting
    them or authenticating the producing process.
    """
    require(layer in LAYER_ARTIFACTS, 'unsupported supplied observation layer')
    fmt, producer, extra = LAYER_ARTIFACTS[layer]
    fields(artifact, ('format', 'observations', 'projection') + extra, layer + ' artifact')
    require(artifact['format'] == fmt, layer + ' artifact: format mismatch')
    require(type(artifact['projection']) is dict, layer + ' artifact: projection must be object')
    adapter = host_adapter()
    expected = adapter.expected_observations(families, producer=producer)
    observed = artifact['observations']
    omissions = None
    if selection is not None:
        expected, _ = _selected_observations(expected, **selection)
        observed, omissions = _selected_observations(observed, **selection)
    difference = adapter.compare_observations(expected, observed, first_difference)
    if difference is not None:
        difference.update(projection=artifact['projection'],
                          raw_context=layer_raw_context(artifact, difference))
    declaration = observed if type(observed) is dict else {}
    summary = dict(status='mismatch' if difference else 'match',
                   scope='complete-supplied-' + layer + '-diagnostic-artifact-comparison-only',
                   format=fmt, declared_producer=declaration.get('producer'),
                   declared_profile=declaration.get('profile'),
                   declared_source_sha256=declaration.get('source_sha256'),
                   producer_authenticity='not-established', execution_receipt='not-supplied',
                   constructor_startup='unsupported', projection=artifact['projection'],
                   artifact_metadata={key: value for key, value in artifact.items()
                                      if key not in ('observations', 'projection', 'raw_runs', 'raw_traces')},
                   raw_diagnostic_counts={key: len(artifact[key]) for key in ('raw_runs', 'raw_traces')
                                          if type(artifact.get(key)) is list},
                   expected_programs=len(expected['programs']), expected_prefix_runs=len(expected['runs']))
    if omissions is not None:
        summary.update(omissions)
        summary['scope'] = 'selected-supplied-' + layer + '-diagnostic-artifact-comparison-only'
    return difference, summary


def _selected_observations(observations, *, case_id, initial_fuel):
    """Select complete literal objects; do not trim fields within either object."""
    require(type(observations) is dict, 'selected observations must be object')
    programs, runs = observations.get('programs'), observations.get('runs')
    require(type(programs) is list and type(runs) is list,
            'selected observations need program and run arrays')
    selected_programs = [row for row in programs
                         if type(row) is dict and row.get('case_id') == case_id]
    selected_runs = [row for row in runs if type(row) is dict
                     and row.get('case_id') == case_id and type(row.get('initial')) is dict
                     and type(row['initial'].get('initial_fuel')) is int
                     and row['initial']['initial_fuel'] == initial_fuel]
    require(len(selected_programs) == 1, 'exactly one selected program identity required')
    require(len(selected_runs) == 1, 'exactly one selected run required')
    # Preserve all envelope members as well, so undeclared extra members do not
    # disappear from complete structural comparison. Only unrelated rows are omitted.
    selected = dict(observations, programs=selected_programs, runs=selected_runs)
    return selected, dict(omitted_programs=len(programs) - 1, omitted_runs=len(runs) - 1)


def compare_paired_observations(k_artifact, quint_artifact, families, *, case_id, initial_fuel):
    """Compare one explicitly selected supplied pair and independent literals.

    No producer is executed or authenticated. Each selected run retains all11
    snapshot fields, ordered records, rule/charge, before/after and terminal probe.
    Literal overlay expansion uses the existing adapter, never a new evaluator.
    """
    require(type(case_id) is str and type(initial_fuel) is int,
            'selected case and exact integer initial fuel required')
    adapter = host_adapter()
    expected_all = adapter.expected_observations(families)
    expected, omitted = _selected_observations(expected_all, case_id=case_id, initial_fuel=initial_fuel)
    artifacts = dict(k=k_artifact, quint=quint_artifact)
    selected, layers = {}, {}
    for layer, artifact in artifacts.items():
        observed, _ = _selected_observations(
            artifact['observations'], case_id=case_id, initial_fuel=initial_fuel)
        selected[layer] = observed
        difference, summary = compare_supplied_observations(artifact, families, layer=layer,
            selection=dict(case_id=case_id, initial_fuel=initial_fuel))
        context_key = dict(case_id=case_id, initial_fuel=initial_fuel, path='/runs/0/terminal')
        summary.update(first_difference=difference, evidence=artifact.get('evidence'),
                       raw_context=layer_raw_context(artifact, context_key))
        layers[layer] = summary
    # Producer names intentionally differ. Their exact per-layer identity was
    # compared above; every other selected observation member is compared here.
    left = {key: value for key, value in selected['k'].items() if key != 'producer'}
    right = {key: value for key, value in selected['quint'].items() if key != 'producer'}
    paired_difference = adapter.compare_observations(left, right, first_difference)
    if paired_difference is not None:
        paired_difference['raw_context'] = {layer: layer_raw_context(artifact, paired_difference)
                                            for layer, artifact in artifacts.items()}
    failed = paired_difference is not None or any(row['first_difference'] is not None
                                                 for row in layers.values())
    expected_program = expected['programs'][0]
    oracle_program = next(program for programs, _ in families for program in programs['programs']
                          if program['case_id'] == case_id)
    peaks, stack_limits, record_projection = {}, {}, {}
    for layer, observations in selected.items():
        run = observations['runs'][0]
        stack_limits[layer] = run['initial'].get('stack_limit')
        records = run.get('records', [])
        snapshots = [run.get('initial'), run.get('terminal')]
        if type(records) is list:
            snapshots.extend(record.get(field) for record in records if type(record) is dict
                             for field in ('before', 'after'))
            record_projection[layer] = [dict(record_index=index,
                rule_start_line=record.get('rule_start_line'), charge=record.get('charge'))
                for index, record in enumerate(records) if type(record) is dict]
        else:
            record_projection[layer] = None
        peaks[layer] = max((state['depth'] for state in snapshots if type(state) is dict
                           and type(state.get('depth')) is int), default=None)
    known_cases = list(dict.fromkeys(SELECTED_CASES + tuple(
        row['case_id'] for row in expected_all['programs'])))
    return dict(status='mismatch' if failed else 'match',
        scope='explicit-selected-supplied-k-quint-complete-state-comparison-only',
        disposition='mismatch' if failed else 'supplied-finite-complete-trace-agreement-only',
        actual_execution='not-established-by-comparator', refinement='none', acceptance='none',
        selection=dict(case_id=case_id, initial_fuel=initial_fuel, compared_programs=1,
                       compared_runs=1, oracle_omissions=omitted),
        oracle=dict(profile=expected['profile'], source_sha256=expected['source_sha256'],
                    program=expected_program, input_sha256=oracle_program.get('input_sha256'),
                    exact_completion_fuel=oracle_program.get('exact_completion_fuel'),
                    expected_value_k=oracle_program.get('expected_value_k'),
                    expected_logical_frame_peak=oracle_program.get('expected_maximum_frame_depth')),
        numeric_bounds=dict(UInt128_min=0, UInt128_max=340282366920938463463374607431768211455,
            initial_fuel=initial_fuel, stack_limit=expected['runs'][0]['initial']['stack_limit'],
            observed_stack_limit=stack_limits, observed_logical_frame_peak=peaks),
        projection_contract=PAIRED_PROJECTION_CONTRACT, selected_rule_action_projection=record_projection,
        paired=dict(status='mismatch' if paired_difference else 'match', first_difference=paired_difference),
        layers=layers, catalogue=dict(retained_default_cases=list(SELECTED_CASES[:-1]),
            unobserved_by_this_report=[case for case in known_cases if case != case_id],
            other_catalogue_status='unchanged; omitted cases are not observed by this report'),
        gaps=['Supplied declarations and raw diagnostics do not authenticate model execution.',
              'Finite selected trace agreement is not semantic refinement, proof, native or target evidence.',
              'The pure profile supplies no finance/wait correspondence; other retained models remain separate.'])


def source_read_inventory():
    """Load the sibling facts helper without relying on Python's script path."""
    path = Path(__file__).resolve().with_name('source_reference_facts.py')
    raw = path.read_bytes()
    spec = importlib.util.spec_from_file_location('sherlock_source_reference_facts', path)
    require(spec is not None and spec.loader is not None, 'source facts helper unavailable')
    module = importlib.util.module_from_spec(spec)
    exec(compile(raw, str(path), 'exec', dont_inherit=True), module.__dict__)
    return module.ReadInventory()


def compile_evidence_record(repo, package, case):
    """Identify seven evidence fields from recorded package files only.

    All optional reporting logic lives inside this already snapshotted module.
    Source/binary identities remain historical receipt declarations; current
    equality, client route, invocation authenticity and semantics are not proved.
    No compiler, interpreter, model, SDK or native operation is selected.
    """
    def require(condition, message):
        if not condition:
            raise ValueError('compile evidence record: ' + message)

    def read_file(path, *, json_document):
        raw = Path(path).read_bytes()
        identity = dict(path=str(path), sha256=digest(raw), bytes=len(raw))
        value = (json.loads(raw, object_pairs_hook=unique_members, parse_constant=nonfinite,
                            parse_float=finite) if json_document else raw)
        return identity, value

    require(case in ('quote', 'eligibility'), 'supported case is quote or eligibility')
    repo, package = Path(repo), Path(package)
    if not package.is_absolute():
        package = repo / package

    def load(relative, *, json_document=True):
        path = package / relative
        identity, value = read_file(path, json_document=json_document)
        try:
            public_path = str(path.relative_to(repo))
        except ValueError:
            public_path = str(path)
        return dict(path=public_path, sha256=identity['sha256']), identity['bytes'], value

    inventory_ref, _, inventory = load('core-metadata-diagnostic-01/PREEXEC.json')
    build_input_ref, _, build_input = load('core-rlib-01/PREEXEC.json')
    build_result_ref, _, build_result = load('core-rlib-01/RESULT.json')
    capture_input_ref, _, capture_input = load('actual-capture/PREEXEC.json')
    capture_result_ref, _, capture_result = load('actual-capture/RESULT.json')
    client_ref, client_bytes, _ = load('actual-capture/capture.rs', json_document=False)
    artifact_ref, artifact_bytes, artifact = load('actual-capture/' + case + '.stdout')

    require(build_input['input_snapshot'] == '../core-metadata-diagnostic-01/PREEXEC.json',
            'build input snapshot must identify the published source inventory')
    members = inventory['source_pins']
    require(type(members) is list and bool(members), 'recorded source inventory is required')
    require(all(type(member) is dict and all(key in member for key in ('path', 'sha256', 'bytes'))
                for member in members), 'recorded source members require their identities')
    require(build_result['exit_code'] == 0 and build_result['source_changed'] == [],
            'recorded library build must have a zero exit and unchanged sources')
    require(capture_result['status'] == 'CAPTURED' and capture_result['input_changed'] == [],
            'capture must record captured output and unchanged inputs')
    for phase in ('compile', case):
        selected = [step for step in capture_result['steps'] if step['name'] == phase]
        require(len(selected) == 1 and selected[0]['exit_code'] == 0,
                'selected recorded compile/case exit must be zero')
    output = capture_result['outputs'][case + '.stdout']
    require(output['sha256'] == artifact_ref['sha256'] and output['bytes'] == artifact_bytes,
            'captured artifact bytes must match the recorded output identity')
    clients = [entry for entry in capture_input['inputs']
               if entry['sha256'] == client_ref['sha256'] and entry['bytes'] == client_bytes]
    require(len(clients) == 1, 'capture client bytes must match one recorded input identity')
    library = capture_input['core_library']
    require(type(library) is dict and all(key in library for key in ('path', 'sha256', 'bytes')),
            'recorded core library identity is required')
    require(sum(entry == library for entry in build_result['artifacts']) == 1,
            'capture core library must match one successful recorded build artifact')
    require(artifact['format'] == 'sherlock-host-artifact/1', 'unsupported captured artifact format')
    require(type(artifact['config']) is dict and type(artifact['admission']['bounds']) is dict,
            'complete supplied config and admission bounds must remain objects')
    observation, evidence = artifact['observation'], artifact['evidence']
    observed = observation['status'] == 'observed'
    require(observation['status'] in ('not-requested', 'observed'), 'unsupported observation disposition')
    require(observation['evaluation_performed'] is observed,
            'observation disposition must preserve its reported evaluation flag')
    require(evidence['host_observation'] == ('observed' if observed else 'unrun'),
            'reported host evidence must agree with the observation disposition')

    return {
        'format': 'sherlock-compiler-evidence-record/1',
        'status': 'linked-historical-records',
        'record_boundary': 'captured artifact plus recorded capture/build/source receipts',
        'predicate': {
            'id': ('sherlock.compile_host.prepare_admit_observe/1' if observed
                   else 'sherlock.compile_host.prepare_admit/1'),
            'claim': ('receipt-declared host observation' if observed
                      else 'receipt-declared source preparation and admission only'),
        },
        'source_tree': {
            'kind': 'historical-recorded-source-inventory',
            'inventory': inventory_ref,
            'members': members,
            'current_source_equality': 'not-checked',
        },
        'profile': artifact['execution_profile'],
        'consumer': {
            'id': 'mori_core::sherlock_compile_artifact::compile_host',
            'capture_client': client_ref,
            'argument_mode': 'observe=true' if observed else 'observe=false',
            'invocation_authenticity': 'not-established',
            'identity_basis': 'supported-capture-family-convention',
            'argument_mode_basis': 'inferred-from-artifact-observation',
            'client_call_route': 'not-checked',
        },
        'bounds': {'config': artifact['config'], 'admission': artifact['admission']['bounds']},
        'target': {
            'declared': artifact['target'],
            'qualification': artifact['qualification'],
            'native': evidence['native'],
            'target': evidence['target'],
        },
        'evidence_class': {
            'kind': ('recorded-host-observation-artifact' if observed
                     else 'recorded-compile-only-artifact'),
            'receipt_status': capture_result['status'],
            'artifact_evidence': evidence,
            'semantic_validity': 'not-established-by-this-reader',
        },
        'identities': {
            'artifact': artifact_ref,
            'core_library': library,
            'capture_inputs': capture_input_ref,
            'capture_result': capture_result_ref,
            'build_inputs': build_input_ref,
            'build_result': build_result_ref,
        },
        'bindings_checked': [
            'captured artifact bytes equal recorded output identity',
            'capture client bytes equal one recorded capture input identity',
            'capture core-library identity equals recorded successful build artifact',
            'recorded build input snapshot names the published source inventory',
            'recorded selected compile/case exits are zero; source/input changed arrays are empty',
        ],
        'gaps': [
            'Historical source inventory entries are recorded identities; source bytes are not reopened or claimed current.',
            'Library identity is matched between receipts; binary bytes and source-to-binary authenticity are not established.',
            'Capture executable identity at invocation time is unrecorded in this receipt format.',
            'Invocation authenticity and independent semantic correctness are not established by a file reader.',
            'No compiler, interpreter, model, SDK, proof or ledger is executed; native/target qualification and acceptance remain separate.',
            'Consumer and argument mode are the supported-family convention and artifact-status inference; client call-route source is not checked.',
        ],
        'acceptance': 'none',
    }



def requirement_evidence_report(repo, path):
    """Validate declared row membership/currentness, never decide semantic acceptance.

    The optional sidecar carries owner assessments. Reading matching files does
    not authenticate an audit or execute a scenario. Bare corpus/snapshot checks
    never call this function or read its catalogue/evidence dependencies.
    """
    path = Path(path)
    if not path.is_absolute():
        path = Path(repo) / path
    reference, ledger = document(path, strict=True)
    require(type(ledger) is dict and ledger.get('schema') == 'sherlock.current-requirement-evidence/1',
            'unknown requirement evidence schema')
    require(type(ledger.get('retainedInventory')) is list and ledger['retainedInventory'],
            'retained requirement-family inventory required')
    issues, cache = [], {}

    def issue(findings, kind, location, message, **detail):
        findings.append(dict(kind=kind, location=location, message=message, **detail))

    def pinned(ref, location, findings):
        if (type(ref) is not dict or type(ref.get('path')) is not str or not ref['path']
                or type(ref.get('sha256')) is not str
                or re.fullmatch('[0-9a-f]{64}', ref['sha256']) is None):
            issue(findings, 'invalid', location, 'reference requires path and SHA-256')
            return None
        target = Path(ref['path'])
        if not target.is_absolute():
            target = Path(repo) / target
        key = str(target)
        if key not in cache:
            try:
                cache[key] = target.read_bytes()
            except OSError as error:
                cache[key] = error
        raw = cache[key]
        if isinstance(raw, OSError):
            issue(findings, 'missing', location, 'declared file cannot be read', path=ref['path'])
            return None
        if any(key in ref for key in ('startLine', 'endLine')):
            start = ref.get('startLine', 1)
            end = ref.get('endLine', start)
            if (type(start) is not int or type(end) is not int or start < 1
                    or end < start or end > len(raw.splitlines())):
                issue(findings, 'invalid', location, 'declared line span is outside the file', path=ref['path'])
        if 'claim' in ref and type(ref['claim']) is not str:
            issue(findings, 'invalid', location, 'reference claim must be a string', path=ref['path'])
        actual = digest(raw)
        if actual != ref['sha256']:
            issue(findings, 'stale', location, 'declared file bytes changed', path=ref['path'],
                  expected_sha256=ref['sha256'], actual_sha256=actual)
        return raw

    def integrity(findings):
        return next((kind for kind in ('invalid', 'missing', 'stale')
                     if any(row['kind'] == kind for row in findings)), 'current')

    def declared_path(ref):
        candidate = Path(ref['path'])
        return candidate if candidate.is_absolute() else Path(repo) / candidate

    catalogue_raw = pinned(ledger.get('requirementCatalogue'), '/requirementCatalogue', issues)
    inventory = ledger.get('refinementInventory')
    inventory_raw = pinned(inventory, '/refinementInventory', issues)
    for ref, expected_path in (
            (ledger.get('requirementCatalogue'), 'openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md'),
            (inventory, 'openspec/changes/sherlock-completion-convergence/specs/sherlock-completion/spec.md')):
        if type(ref) is dict and type(ref.get('path')) is str and declared_path(ref) != Path(repo) / expected_path:
            issue(issues, 'invalid', '/inventory', 'catalogue must name the retained canonical specification', path=ref['path'])
    require(catalogue_raw is not None and inventory_raw is not None,
            'requirement catalogue or refinement inventory unavailable')
    def sections(raw):
        result = {}
        for part in re.split(r'(?m)^### Requirement: ', raw.decode('utf-8'))[1:]:
            heading = part.splitlines()[0]
            identifier = heading.split(' ', 1)[0]
            require(identifier not in result, 'duplicate requirement heading in catalogue')
            result[identifier] = dict(heading=heading,
                scenarios=re.findall(r'(?m)^#### Scenario: (.+)$', part))
        return result

    originals, refinements = sections(catalogue_raw), sections(inventory_raw)
    required_ids, feature_ids = list(originals), list(refinements)
    original_scenario_count = sum(len(row['scenarios']) for row in originals.values())
    scenario_count = sum(len(row['scenarios']) for row in refinements.values())
    if (required_ids != [f'UIREQ-{number:03}' for number in range(1, 36)]
            or feature_ids != [f'SCON-{number:03}' for number in range(1, 26)]
            or scenario_count != 68):
        issue(issues, 'invalid', '/inventory', 'retained 35/25/68 named membership differs')
    if inventory.get('featureIds') != feature_ids or inventory.get('scenarioCount') != scenario_count:
        issue(issues, 'invalid', '/refinementInventory', 'declared refinement inventory differs')
    assessment = ledger.get('assessment')
    if (type(assessment) is not dict or type(assessment.get('status')) is not str
            or not assessment['status'] or assessment.get('globalV1') not in ('open', 'closed')
            or assessment.get('adoption') not in ('pending-root', 'adopted')):
        issue(issues, 'invalid', '/assessment', 'explicit assessment disposition required')
        assessment = assessment if type(assessment) is dict else {}

    def rows(member, expected):
        declared = ledger.get(member)
        if type(declared) is not list:
            issue(issues, 'invalid', '/' + member, 'complete row array required')
            declared = []
        ids = [row.get('id') if type(row) is dict else None for row in declared]
        if ids != expected:
            issue(issues, 'invalid', '/' + member, 'row membership/order differs', expected=expected, observed=ids)
        results = []
        for index, row in enumerate(declared):
            location, findings = f'/{member}/{index}', []
            try:
                require(type(row) is dict, 'row must be an object')
                fields(row, ('id', 'requirementSource', 'predicate', 'implementation', 'validation',
                             'acceptance', 'currentness', 'nextDiscriminatingCheck', 'citations'), location)
                source = row['requirementSource']
                fields(source, ('path', 'sha256', 'heading', 'scenarioNames'), location + '/requirementSource')
                raw = pinned(source, location + '/requirementSource', findings)
                catalogue_ref = ledger['requirementCatalogue'] if member == 'requirements' else inventory
                expected_section = (originals if member == 'requirements' else refinements).get(row['id'])
                def source_path(ref):
                    candidate = Path(ref['path'])
                    return candidate if candidate.is_absolute() else Path(repo) / candidate
                require(source_path(source) == source_path(catalogue_ref)
                        and source['sha256'] == catalogue_ref['sha256'],
                        'row source must bind its declared requirement catalogue')
                if (expected_section is None or source['heading'] != expected_section['heading']
                        or source['scenarioNames'] != expected_section['scenarios']):
                    issue(findings, 'invalid', location + '/requirementSource',
                          'row heading or named scenario membership differs from its ID',
                          expected=expected_section, observed=source)
                predicate = row['predicate']
                fields(predicate, ('id', 'description', 'domain', 'applicableEvidenceClasses'), location + '/predicate')
                require(all(type(predicate[key]) is str and predicate[key] for key in ('id', 'description', 'domain'))
                        and type(predicate['applicableEvidenceClasses']) is list,
                        'explicit predicate/domain/evidence classes required')
                require(predicate['applicableEvidenceClasses']
                        and all(type(item) is str and item for item in predicate['applicableEvidenceClasses']),
                        'evidence classes must be nonempty named strings')
                for field, allowed, extra in (
                        ('implementation', ('complete', 'partial', 'absent', 'unknown'), ('supportedScope', 'missing')),
                        ('validation', ('observed', 'partial', 'unrun', 'unknown'), ('evidenceRefs', 'countercaseRefs', 'reviewRefs')),
                        ('acceptance', ('closed', 'open'), ('scope', 'reason'))):
                    fields(row[field], ('status',) + extra, location + '/' + field)
                    require(row[field]['status'] in allowed, f'unknown {field} status')
                require(type(row['implementation']['supportedScope']) is str
                        and type(row['implementation']['missing']) is str
                        and all(type(row['acceptance'][key]) is str and row['acceptance'][key]
                                for key in ('scope', 'reason')), 'explicit scope/gap/reason strings required')
                fields(row['currentness'], ('sourcePins', 'comparedToCurrent', 'limitations'), location + '/currentness')
                require(row['currentness']['comparedToCurrent'] in ('identical', 'changed', 'partial', 'unknown'),
                        'unknown declared currentness')
                require(type(row['currentness']['limitations']) is list
                        and all(type(item) is str for item in row['currentness']['limitations']),
                        'currentness limitations must be strings')
                fields(row['nextDiscriminatingCheck'], ('consumer', 'input', 'expected', 'missingAuthorityOrOwner'),
                       location + '/nextDiscriminatingCheck')
                require(all(type(value) is str for value in row['nextDiscriminatingCheck'].values()),
                        'next check fields must be explicit strings')
                for field, refs in (
                        ('sourcePins', row['currentness']['sourcePins']),
                        ('evidenceRefs', row['validation']['evidenceRefs']),
                        ('countercaseRefs', row['validation']['countercaseRefs']),
                        ('reviewRefs', row['validation']['reviewRefs']), ('citations', row['citations']),
                        ('sourceRefs', row['implementation'].get('sourceRefs', [])),
                        ('recordedConsumerClosure', row['currentness'].get('recordedConsumerClosure', []))):
                    require(type(refs) is list, field + ' must be an array')
                    for ref_index, ref in enumerate(refs):
                        pinned(ref, f'{location}/{field}/{ref_index}', findings)
                if row['acceptance']['status'] == 'closed':
                    require(row['validation']['evidenceRefs'] and row['validation']['reviewRefs']
                            and row['currentness']['sourcePins'] and row['acceptance']['scope']
                            and row['acceptance']['reason'], 'closed declaration needs scoped evidence, review and source references')
                    require(row['implementation']['status'] == 'complete' and row['validation']['status'] == 'observed'
                            and row['currentness']['comparedToCurrent'] == 'identical',
                            'closed declaration requires complete/observed/current declarations')
            except (ValueError, KeyError, TypeError, UnicodeError) as error:
                issue(findings, 'invalid', location, str(error))
            state = integrity(findings + issues)
            results.append(dict(id=row.get('id') if type(row) is dict else None, declared=row,
                                integrity=state, current_reportability=('current-owner-declaration' if state == 'current' else 'not-current'),
                                findings=findings))
        return results

    original_rows = rows('requirements', required_ids)
    feature_rows = rows('refinementRequirements', feature_ids)
    # Later membership failures also invalidate earlier rows; no stale/invalid
    # declaration enters a current subtotal merely because it appeared first.
    if issues:
        for row in original_rows + feature_rows:
            row['integrity'] = integrity(row['findings'] + issues)
            row['current_reportability'] = 'not-current'
    combined = original_rows + feature_rows
    def summarize(group):
        def eligible(row):
            declared = row['declared']
            return (row['integrity'] == 'current' and type(declared) is dict
                    and type(declared.get('acceptance')) is dict
                    and declared['acceptance'].get('status') == 'closed'
                    and type(declared.get('currentness')) is dict
                    and declared['currentness'].get('comparedToCurrent') == 'identical')
        result = dict(rows=len(group), current_owner_declared_closed=sum(eligible(row) for row in group),
                      current_adopted_closed=0)
        if assessment.get('adoption') == 'adopted':
            result['current_adopted_closed'] = sum(eligible(row)
                and row['declared']['acceptance'].get('adoption') == 'adopted' for row in group)
        for field in ('implementation', 'validation', 'acceptance'):
            states = [row['declared'][field].get('status') for row in group
                      if type(row['declared']) is dict and type(row['declared'].get(field)) is dict]
            result['declared_' + field] = {status: states.count(status) for status in sorted(
                {state for state in states if type(state) is str})}
        return result
    retained = []
    retained_issues = []
    for index, item in enumerate(ledger.get('retainedInventory', [])):
        findings = []
        if (type(item) is dict and type(item.get('family')) is str and type(item.get('status')) is str
                and type(item.get('disposition')) is str):
            pinned(item.get('catalogue'), f'/retainedInventory/{index}/catalogue', findings)
        else:
            issue(findings, 'invalid', f'/retainedInventory/{index}', 'retained family/status/catalogue required')
        retained.append(dict(declared=item, integrity=integrity(findings), findings=findings))
        retained_issues.extend(findings)
    summary = dict(original35=summarize(original_rows), refinement25=summarize(feature_rows))
    return dict(status=integrity(issues + retained_issues + [finding for row in combined for finding in row['findings']]),
                scope='declared-requirement-evidence-completeness-and-currentness-only',
                semantic_assessment='owner-declared; file checks do not authenticate audits or decide acceptance',
                dependency_closure='declared-only; undeclared dependencies are not established',
                ledger=reference, assessment=assessment,
                inventory=dict(original_requirement_ids=required_ids, refinement_feature_ids=feature_ids,
                               original_scenario_count=original_scenario_count, scenario_count=scenario_count,
                               scenario_execution='not-established-by-file-check'),
                requirements=original_rows, refinement_requirements=feature_rows, retained_inventory=retained,
                summary=summary, findings=issues)


def run(repo, *, observed_programs=None, observed_prefixes=None, family='normal', host_observations=None,
        lean_observations=None, quint_observations=None, image_observations=None, k_observations=None,
        requirement_evidence=None, compile_evidence_package=None, compile_evidence_case=None,
        paired_k_observations=None, paired_quint_observations=None, selected_case=None, selected_initial_fuel=None):
    repo = Path(repo)
    require((compile_evidence_package is None) == (compile_evidence_case is None),
            'compile evidence package and case must be supplied together')
    inventory = source_read_inventory()
    files = []
    totals = dict(programs=0, state_rows=0, charged_edges=0, publications=0, prefix_rows=0)
    transcriptions = []
    frozen = {}
    source_raw = (repo / SOURCE['path']).read_bytes()
    inventory.record(SOURCE['path'], source_raw)
    require(digest(source_raw) == SOURCE['sha256'], 'frozen K source changed')
    files.append(dict(path=SOURCE['path'], sha256=digest(source_raw)))
    for member in FAMILIES:
        docs = []
        for suffix in ('programs', 'prefixes'):
            name = member + '-' + suffix + '.json'
            path = repo / 'models/sherlock/oracles' / name
            ref, value = document(path, inventory=inventory, reference_path=str(path.relative_to(repo)))
            require(ref['sha256'] == FROZEN[name], f'frozen oracle changed: {name}')
            ref['path'] = str(path.relative_to(repo))
            files.append(ref)
            docs.append(value)
        counts = validate_pair(*docs, family=member)
        for key, count in counts.items():
            totals[key] += count
        transcriptions.append(check_rust_transcription(repo, *docs, member, inventory=inventory))
        frozen[member] = docs
    require((observed_programs is None) == (observed_prefixes is None),
            'observed-programs and observed-prefixes must be supplied together')
    supplied_paths = dict(host=host_observations, lean=lean_observations,
                          quint=quint_observations, image=image_observations, k=k_observations)
    require(sum(path is not None for path in supplied_paths.values()) <= 1,
            'supplied layer observations are mutually exclusive comparison modes')
    require(not any(path is not None for path in supplied_paths.values()) or observed_programs is None,
            'layer observations and literal document observations are separate comparison modes')
    paired_options = (paired_k_observations, paired_quint_observations, selected_case, selected_initial_fuel)
    paired_mode = any(option is not None for option in paired_options)
    require(not paired_mode or all(option is not None for option in paired_options),
            'paired K/Quint paths, selected case and initial fuel must be supplied together')
    require(not paired_mode or (observed_programs is None
            and not any(path is not None for path in supplied_paths.values())),
            'selected paired and existing observation comparisons are separate modes')
    require(not paired_mode or selected_case in SELECTED_CASES, 'unsupported selected paired case')
    difference = None
    selected_pair = None
    supplied_host = None
    supplied_layers = {}
    observed_files = []
    if observed_programs is not None:
        observations = []
        for path in (observed_programs, observed_prefixes):
            ref, value = document(path, strict=True)
            observed_files.append(ref)
            observations.append(value)
        difference = compare_documents(*frozen[family], *observations)
    if host_observations is not None:
        adapter = host_adapter()
        host_ref, observed = document(host_observations, strict=True)
        expected = adapter.expected_observations(list(frozen.values()))
        difference = adapter.compare_observations(expected, observed, first_difference)
        declaration = observed if type(observed) is dict else {}
        supplied_host = dict(status='mismatch' if difference else 'match',
                             scope='complete-supplied-host-diagnostic-artifact-comparison-only',
                             file=host_ref, declared_producer=declaration.get('producer'),
                             declared_profile=declaration.get('profile'),
                             declared_source_sha256=declaration.get('source_sha256'),
                             producer_authenticity='not-established', execution_receipt='not-supplied',
                             constructor_startup='unsupported',
                             expected_programs=len(expected['programs']), expected_prefix_runs=len(expected['runs']))
    for layer, path in supplied_paths.items():
        if layer == 'host' or path is None:
            continue
        ref, artifact = document(path, strict=True)
        difference, summary = compare_supplied_observations(artifact, list(frozen.values()), layer=layer)
        summary['file'] = ref
        supplied_layers['supplied_' + layer + '_observation'] = summary
    if paired_mode:
        # Keep the full default source/corpus validation above. Only this explicit
        # selected comparison may use a single run instead of216 corpus runs.
        oracle_families = list(frozen.values())
        oracle_files = []
        if selected_case == 'F03':
            fixture_documents = []
            for name, pin in F03_ORACLE_PINS.items():
                path = repo / F03_ORACLE_ROOT / name
                ref, value = document(path, strict=True, inventory=inventory,
                                      reference_path=str(path.relative_to(repo)))
                require(ref['sha256'] == pin, 'frozen F03 oracle changed: ' + name)
                ref['path'] = str(path.relative_to(repo))
                oracle_files.append(ref)
                fixture_documents.append(value)
            oracle_families = [tuple(fixture_documents)]
        pair_documents = []
        for path in (paired_k_observations, paired_quint_observations):
            ref, value = document(path, strict=True)
            observed_files.append(ref)
            pair_documents.append(value)
        selected_pair = compare_paired_observations(*pair_documents, oracle_families,
            case_id=selected_case, initial_fuel=selected_initial_fuel)
        selected_pair['oracle_files'] = oracle_files if oracle_files else files[1:]
        selected_pair['supplied_files'] = dict(zip(('k', 'quint'), observed_files, strict=True))
        difference = selected_pair['paired']['first_difference']
        for row in selected_pair['layers'].values():
            if difference is None:
                difference = row['first_difference']
    report = dict(status='mismatch' if difference else 'pass',
                scope='frozen-normal-corpus-and-source-literal-QA', acceptance='none',
                profile=PROFILE, counts=totals, frozen_files=files,
                rust_literal_transcriptions=transcriptions, observed_files=observed_files,
                source_reference_facts=inventory.report(),
                supplied_observation_status=('mismatch' if difference else 'match') if observed_files else 'not-supplied',
                supplied_observation_family=family if observed_files else None,
                supplied_comparison_gaps={'K': dict(
                    status='supplied-comparison-only' if k_observations is not None else 'unsupported',
                    reason='Supplied K cells, rule labels and terminal probes are diagnostic declarations; '
                           'no K executor or receipt authentication is invoked by this consumer.')},
                first_difference=difference,
                execution_layers=[dict(layer=layer, status='not-run', reason='No executable adapter invoked by this SDK-free QA consumer')
                                  for layer in ('K', 'Quint', 'Lean', 'Clean', 'handwritten-circuit', 'Rust-host', 'native-ledger')],
                gaps=['Complete literal agreement supplies no semantic execution, refinement, constraint, proof or target evidence.',
                      'The supplied observation documents use the existing corpus format, not a runtime state codec.',
                      'No B helper microsteps, hostile cases, general grammar or financial continuation coverage.'],
                **({'supplied_host_observation': supplied_host} if supplied_host is not None else {}),
                **supplied_layers)
    if selected_pair is not None:
        report['selected_paired_projection'] = selected_pair
        report['supplied_observation_family'] = None
        report['supplied_comparison_gaps']['K']['status'] = 'selected-supplied-comparison-only'
    if compile_evidence_package is not None:
        report['compile_evidence_record'] = compile_evidence_record(
            repo, compile_evidence_package, compile_evidence_case)
    if requirement_evidence is not None:
        current = requirement_evidence_report(repo, requirement_evidence)
        report['requirement_evidence'] = current
        if current['status'] != 'current':
            report['status'] = 'mismatch'
            if report['first_difference'] is None:
                report['first_difference'] = dict(path='/requirement_evidence',
                    expected='complete current declared evidence', observed=current['status'])
    return report


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument('--observed-programs', type=Path)
    parser.add_argument('--compile-evidence-record', type=Path,
                        help='identify evidence fields from an existing compiler capture package; no producer execution')
    parser.add_argument('--compile-evidence-case', choices=('quote', 'eligibility'),
                        help='exact captured artifact for the optional companion record')
    parser.add_argument('--requirement-evidence', type=Path,
                        help='report explicitly selected owner-declared requirement evidence/currentness; no semantic acceptance')
    supplied = parser.add_mutually_exclusive_group()
    supplied.add_argument('--host-observations', type=Path,
                        help='compare a supplied complete normal host diagnostic artifact; does not execute it')
    for layer in LAYER_ARTIFACTS:
        supplied.add_argument('--' + layer + '-observations', type=Path,
                              help='compare a supplied complete normal ' + layer + ' adapter artifact; does not execute it')
    parser.add_argument('--paired-k-observations', type=Path,
                        help='explicit selected-run K artifact; requires paired Quint, case and fuel; no execution')
    parser.add_argument('--paired-quint-observations', type=Path,
                        help='explicit selected-run Quint artifact; does not require all216 runs')
    parser.add_argument('--selected-case', choices=SELECTED_CASES)
    parser.add_argument('--selected-initial-fuel', type=int)
    parser.add_argument('--observed-prefixes', type=Path)
    parser.add_argument('--family', choices=FAMILIES, default='normal')
    args = parser.parse_args(argv)
    if (args.compile_evidence_record is None) != (args.compile_evidence_case is None):
        parser.error('--compile-evidence-record and --compile-evidence-case must be supplied together')
    if any(getattr(args, layer + '_observations') is not None for layer in ('host', *LAYER_ARTIFACTS)) and (args.observed_programs is not None or args.observed_prefixes is not None):
        parser.error('layer observations and literal document observations are separate comparison modes')
    if (args.observed_programs is None) != (args.observed_prefixes is None):
        parser.error('--observed-programs and --observed-prefixes must be supplied together')
    try:
        report = run(args.repo, observed_programs=args.observed_programs,
                     observed_prefixes=args.observed_prefixes, family=args.family,
                     host_observations=args.host_observations, lean_observations=args.lean_observations,
                     quint_observations=args.quint_observations, image_observations=args.image_observations,
                     k_observations=args.k_observations, requirement_evidence=args.requirement_evidence,
                     compile_evidence_package=args.compile_evidence_record,
                     compile_evidence_case=args.compile_evidence_case,
                     paired_k_observations=args.paired_k_observations,
                     paired_quint_observations=args.paired_quint_observations,
                     selected_case=args.selected_case, selected_initial_fuel=args.selected_initial_fuel)
        output = json.dumps(report, indent=2, allow_nan=False)
    except (OSError, ValueError, KeyError, TypeError, RecursionError) as error:
        parser.exit(2, f'QA input error: {error}\n')
    print(output)
    return 1 if report['status'] == 'mismatch' else 0


if __name__ == '__main__':
    sys.exit(main())
