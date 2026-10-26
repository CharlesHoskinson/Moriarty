#!/usr/bin/env python3
"""Report the U021 projection and U026 required correspondence stage.

Comparisons consume supplied or retained observations; no K, Quint, Lean,
constraint, proof or native executor is invoked. Private integration records
are opt-in local evidence and never copied into the public report.
"""
import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

import check as qa


REPO = Path(__file__).resolve().parents[3]
ARCHIVE = 'docs/results/sherlock-k-quint-f03-actual-20261018'
ARCHIVED_OUTPUTS = {
    'k': '780dfb6b3a4908f46269bb605c99b72f5074058a06b3c1a733e677fae2814468',
    'quint': '85f5b9e20a491cf12114f330dc863167e89a270b7cb9045caa1afd419c16736a',
}
STAGE = 'openspec/changes/sherlock-universal-interpreter-design'
PRIVATE_FORMAT = 'sherlock-private-integration/1'
SEMANTIC_MEMBERS = ('Clean/Sherlock/Semantics.lean', 'Clean/Sherlock/SemanticsExamples.lean')
COMPLETE_STATE_FIELDS = ('code_k', 'entry_k', 'control_k', 'environment_k',
    'frames_top_first_k', 'depth', 'stack_limit', 'initial_fuel',
    'remaining_fuel', 'charged_steps', 'output_k')
PUBLIC_PATH_FIELDS = frozenset(COMPLETE_STATE_FIELDS + ('programs', 'runs', 'initial', 'terminal',
    'records', 'rule_start_line', 'charge', 'before', 'after', 'terminal_has_next', 'case_id',
    'producer', 'profile', 'source_sha256', 'boundary'))


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def read_json(path, *, expected=None):
    raw = Path(path).read_bytes()
    decoded = gzip.decompress(raw) if str(path).endswith('.gz') else raw
    qa.require(expected is None or sha256(decoded) == expected, 'archived input pin mismatch')
    value = json.loads(decoded, object_pairs_hook=qa.unique_members,
                       parse_constant=qa.nonfinite, parse_float=qa.finite)
    return value, dict(sha256=sha256(decoded), bytes=len(decoded),
                       stored_sha256=sha256(raw), stored_bytes=len(raw))


def source_member(path):
    raw = (REPO / path).read_bytes()
    return dict(path=path, sha256=sha256(raw), bytes=len(raw))


def value_digest(value):
    return sha256(json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False).encode())


def public_difference(difference):
    """Expose the first field and value identities, never arbitrary supplied data.

    Full raw states remain in the input artifacts identified by digest. Unknown
    JSON pointer segments may themselves carry private text, so expose their
    position and the digest of the original pointer instead.
    """
    if difference is None:
        return None
    qa.fields(difference, ('path',), 'first difference')
    path = difference['path']
    qa.require(type(path) is str, 'difference path must be a string')
    safe = '/'.join(part if part in PUBLIC_PATH_FIELDS or re.fullmatch(r'[0-9]+', part)
                    else '<unmapped>' for part in path.split('/')[1:])
    result = dict(path='/' + safe if path else '', path_sha256=sha256(path.encode()))
    for field in ('expected', 'observed'):
        if field in difference:
            result[field + '_sha256'] = value_digest(difference[field])
        elif field + '_sha256' in difference:
            pin = difference[field + '_sha256']
            qa.require(type(pin) is str and re.fullmatch(r'[0-9a-f]{64}', pin), 'invalid value digest')
            result[field + '_sha256'] = pin
    return result


def public_comparison(comparison, artifacts):
    result = {key: comparison[key] for key in ('status', 'scope', 'disposition', 'actual_execution',
        'refinement', 'acceptance', 'selection', 'oracle', 'catalogue', 'gaps')}
    result['projection_contract'] = copy.deepcopy(qa.PAIRED_PROJECTION_CONTRACT)
    bounds = copy.deepcopy(comparison['numeric_bounds'])
    bounds['observed_stack_limit'] = {layer: value if type(value) is int else None
                                     for layer, value in bounds['observed_stack_limit'].items()}
    result['numeric_bounds'] = bounds
    result['paired'] = dict(status=comparison['paired']['status'],
                           first_difference=public_difference(comparison['paired']['first_difference']))
    result['layers'] = {}
    for layer, artifact in zip(('k', 'quint'), artifacts):
        row = comparison['layers'][layer]
        result['layers'][layer] = dict(status=row['status'],
            first_difference=public_difference(row['first_difference']),
            producer_authenticity='not-established', constructor_startup='unsupported',
            expected_programs=row['expected_programs'], expected_prefix_runs=row['expected_prefix_runs'],
            omitted_programs=row['omitted_programs'], omitted_runs=row['omitted_runs'],
            provenance_sha256=value_digest({key: value for key, value in artifact.items()
                                             if key != 'observations'}))
    result['selected_rule_action_projection'] = {
        layer: [dict(record_index=row['record_index'],
                     rule_start_line=row['rule_start_line'] if type(row['rule_start_line']) is int else None,
                     charge=row['charge'] if type(row['charge']) is int else None)
                for row in records]
        for layer, records in comparison['selected_rule_action_projection'].items()}
    return result


def oracle_families(case):
    if case == 'F03':
        members = [(qa.F03_ORACLE_ROOT + '/' + name, pin)
                   for name, pin in qa.F03_ORACLE_PINS.items()]
    else:
        members = [('models/sherlock/oracles/' + name, pin) for name, pin in qa.FROZEN.items()]
    documents, refs = [], []
    for path, pin in members:
        value, ref = read_json(REPO / path, expected=pin)
        documents.append(value)
        refs.append(dict(path=path, **ref))
    return list(zip(documents[::2], documents[1::2])), refs


def projection(k_path=None, quint_path=None, case=None, fuel=None):
    options = (k_path, quint_path, case, fuel)
    supplied = any(value is not None for value in options)
    qa.require(not supplied or all(value is not None for value in options),
               'supply K, Quint, case and fuel together')
    qa.require(not supplied or (case in qa.SELECTED_CASES and type(fuel) is int and fuel >= 0),
               'unsupported case or fuel')
    source = source_member(qa.SOURCE['path'])
    qa.require(source['sha256'] == qa.SOURCE['sha256'], 'frozen K source changed')
    if not supplied:
        case, fuel = 'F03', 93
        k_path, quint_path = (REPO / ARCHIVE / 'actual/outputs' / (name + '-observations.json.gz')
                              for name in ('k', 'quint'))
    artifacts, inputs = [], {}
    for layer, path in (('k', k_path), ('quint', quint_path)):
        artifact, ref = read_json(path, expected=None if supplied else ARCHIVED_OUTPUTS[layer])
        artifacts.append(artifact)
        # Supplied filenames may themselves contain private integration details.
        inputs[layer] = dict(origin='supplied' if supplied else 'retained-actual-F03', **ref)
    families, refs = oracle_families(case)
    comparison = qa.compare_paired_observations(*artifacts, families, case_id=case, initial_fuel=fuel)
    edges = {layer: len(qa._selected_observations(artifact['observations'],
        case_id=case, initial_fuel=fuel)[0]['runs'][0]['records'])
        for layer, artifact in zip(('k', 'quint'), artifacts)}
    return dict(format='sherlock-model-projection/1', requirement='UIREQ-021',
        status=comparison['status'], profile=qa.PROFILE, comparison=public_comparison(comparison, artifacts),
        compared_edges=edges, inputs=inputs, oracle_files=refs, reference_source=source,
        execution='not-run', producer_evidence='supplied-declarations' if supplied else
            'retained-historical-actual-receipts; producers-not-rerun',
        historical_receipts=[] if supplied else [
            ARCHIVE + '/actual/k/root/RESULT.json', ARCHIVE + '/actual/quint/root/RESULT.json'],
        refinement='none', constraint_evidence='none', native_evidence='none', acceptance='none')


def private_binding(record_path):
    if record_path is None:
        return dict(status='not-supplied', proof_membership='not-established',
                    obligation='Retain actual integration source/profile/configuration and evidence privately')
    value, ref = read_json(record_path)
    qa.require(type(value) is dict and value.get('format') == PRIVATE_FORMAT
               and type(value.get('source_files')) is list and value['source_files'],
               'invalid private integration record')
    for row in value['source_files']:
        qa.fields(row, ('path', 'sha256', 'bytes'), 'private source member')
        qa.require(type(row['sha256']) is str and re.fullmatch(r'[0-9a-f]{64}', row['sha256'])
                   and type(row['bytes']) is int and row['bytes'] >= 0, 'invalid private source pin')
    return dict(status='retained-private-source-record', record_sha256=ref['sha256'],
        record_bytes=ref['bytes'], source_file_count=len(value['source_files']),
        proof_membership='not-established', native_membership='not-established', currentness='not-rechecked',
        qualification='Source membership only; retained private record is not correspondence evidence')


def bind_private(root, record_path):
    """Read a selected integration tree; write metadata only to an ignored location.

    No private source is copied. The private record retains paths/HEAD/dirty
    membership, while public stdout contains only its digest and scope.
    """
    root, destination = Path(root).resolve(), Path(record_path).resolve()
    try:
        relative = destination.relative_to(REPO)
    except ValueError:
        relative = None
    if relative is not None:
        qa.require(relative.parts[:2] == ('target', 'sherlock-kc'),
                   'private record inside checkout must be under target/sherlock-kc')
        ignored = subprocess.run(['git', 'check-ignore', '--quiet', str(relative)],
                                 cwd=REPO, capture_output=True)
        qa.require(ignored.returncode == 0, 'private destination must be gitignored')
    qa.require(not destination.is_relative_to(root), 'private source tree is read-only')
    members = list(SEMANTIC_MEMBERS) + [
        'Clean/Circuit/Provable.lean', 'Clean/Circuit/Basic.lean',
        'Clean/Table/Inductive.lean', 'Clean/Circuit/WitnessIR.lean',
        'Clean/Circuit/WitnessExport.lean', 'Clean/Backends/Circom/R1CS.lean',
        'lean-toolchain', 'lakefile.lean', 'lake-manifest.json']
    sources, missing = [], []
    for member in members:
        path = root / member
        if path.is_file():
            raw = path.read_bytes()
            sources.append(dict(path=member, sha256=sha256(raw), bytes=len(raw)))
        else:
            missing.append(member)
    qa.require(all(any(row['path'] == member for row in sources) for member in SEMANTIC_MEMBERS),
               'integration is missing required semantic sources')
    def git(*args):
        result = subprocess.run(['git', '-C', str(root), *args], capture_output=True, text=True)
        return result.stdout.strip() if result.returncode == 0 else None
    record = dict(format=PRIVATE_FORMAT, integration_root=str(root), head=git('rev-parse', 'HEAD'),
        dirty_membership=git('status', '--porcelain', '--untracked-files=all', '--', '.'),
        source_files=sources, missing_members=missing, profile=qa.PROFILE,
        representation='pure recursive values; bounded Clean MachineImage not established',
        configuration='finite fuel and stack arguments; no B capacities admitted',
        proof_membership='not-established', native_membership='not-established',
        execution='not-run', scope='private source membership, not proof or correspondence')
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(record, indent=2) + '\n')
    return dict(format='sherlock-private-binding/1', private_integration=private_binding(destination))


def stage(comparison_path=None, private_record=None, retained_comparison=False):
    qa.require(not (retained_comparison and comparison_path is not None),
               'choose a retained or a supplied comparison, not both')
    inventory, _ = read_json(REPO / STAGE / 'public-clean-api-inventory.json')
    public = dict(upstream=inventory['upstream'], commit=inventory['public_upstream_commit'],
        files=inventory['files'], receipt=source_member(STAGE + '/public-clean-api-inventory.json'),
        scope='Pinned public API reuse inventory; current network/build verification not run',
        limits=['WitnessIR U64 operations wrap; checked UInt128 requires its own mapping',
                'Circom lookup/interact are refused; no Midnight exporter follows'])
    task_specs = [
        ('recursive-lean-semantics', 'historical-narrow-foundation',
         'Recursive UIntList/UIntTree/Value, syntax, ordered program tables, environments and saved frames; '
         'generic recursive ADTs and B helper controls remain required', []),
        ('fuel-step-run', 'historical-narrow-foundation',
         'Finite-fuel step/run with explicit attempted work, outcomes and free terminal publication; '
         'B helper-metered schedule remains required', ['recursive-lean-semantics']),
        ('bounded-clean-encoding', 'not-implemented',
         'Freeze Bounds/LayoutSchema/MachineImage, partial codecs, StepInv, canonical boundary projection '
         'and representation relation for complete environments, frames, data and counters', ['recursive-lean-semantics']),
        ('bounded-clean-step-circuit', 'not-implemented',
         'Implement fixed-capacity StepInput/StepOutput components and separate constraint/witness plans; '
         'host materializeStep is outside the circuit', ['fuel-step-run', 'bounded-clean-encoding']),
        ('semantic-representation-correspondence', 'unproved',
         'Admitted round trips and step/run correspondence under named profiles, bounds and helper clocks',
         ['bounded-clean-encoding', 'bounded-clean-step-circuit']),
        ('backend-native-integration', 'not-implemented',
         'Capability-checked witness/constraint lowering and stock-compatible ledger-9 native consumer',
         ['bounded-clean-step-circuit'])]
    tasks = [dict(id=name, status=status, required=True, deliverable=deliverable, depends=depends)
             for name, status, deliverable, depends in task_specs]
    _, oracle_refs = oracle_families('F03')
    layer_members = {
        'K': [source_member('models/sherlock/k/main.k')],
        'Quint': [source_member('models/sherlock/quint/kProfileMachine.qnt'),
                  source_member('models/sherlock/quint/observations/f03ListsumObservationTest.qnt')],
        'Lean': [], 'Clean': [], 'handwritten-circuit': [], 'native-ledger': []}
    layers = [dict(layer=layer, status='not-run' if layer in ('K', 'Quint', 'Lean') else 'not-implemented',
        source_membership=members, witness_membership=[], constraint_membership=[],
        reason='Private source binding required' if layer in ('Lean', 'Clean') else
               'No layer executor invoked; component/native correspondence remains unestablished')
        for layer, members in layer_members.items()]
    loop = dict(method='docs/SHERLOCK-CORRECTNESS-LOOP.md',
        comparison_status='not-run', first_divergence=None,
        first_divergence_disposition='No comparison supplied; null is not evidence of agreement',
        projections={'K/Quint': copy.deepcopy(qa.PAIRED_PROJECTION_CONTRACT)},
        complete_state_fields=list(COMPLETE_STATE_FIELDS), independent_expectations=oracle_refs,
        prefix_policy='Compare every complete pre/post state, ordered rule/charge, fuel prefix and publication; '
                      'matching final values alone is insufficient',
        layers=layers, shared_dependencies=['Existing diagnostic renderer/comparator is shared; no independence theorem'],
        execution='not-run', evidence_classes=dict(theorem='none', constraint='none', native='none', target='none'))
    if retained_comparison:
        supplied = projection()
        ref = dict(origin='retained-actual-F03',
                   sha256=value_digest(supplied), inputs=supplied['inputs'])
    elif comparison_path is not None:
        supplied, ref = read_json(comparison_path)
    if retained_comparison or comparison_path is not None:
        qa.require(type(supplied) is dict and supplied.get('format') == 'sherlock-model-projection/1',
                   'unknown comparison report format')
        comparison = supplied['comparison']
        qa.fields(comparison, ('status', 'paired', 'layers', 'selection', 'numeric_bounds'), 'comparison')
        qa.fields(comparison['paired'], ('status', 'first_difference'), 'paired comparison')
        qa.require(comparison['paired']['status'] in ('match', 'mismatch'), 'invalid paired status')
        qa.fields(comparison['layers'], ('k', 'quint'), 'comparison layers')
        for layer in ('k', 'quint'):
            qa.fields(comparison['layers'][layer], ('status', 'first_difference'), 'comparison layer')
            qa.require(comparison['layers'][layer]['status'] in ('match', 'mismatch'), 'invalid layer status')
        selection = comparison['selection']
        qa.fields(selection, ('case_id', 'initial_fuel'), 'comparison selection')
        qa.require(selection['case_id'] in qa.SELECTED_CASES and type(selection['initial_fuel']) is int
                   and selection['initial_fuel'] >= 0, 'invalid comparison selection')
        _, selected_oracles = oracle_families(selection['case_id'])
        qa.require(comparison['status'] in ('match', 'mismatch'), 'invalid comparison status')
        difference = comparison['paired']['first_difference']
        if difference is None:
            difference = next((row['first_difference'] for row in comparison['layers'].values()
                               if row['first_difference'] is not None), None)
        failed = (comparison['status'] == 'mismatch' or comparison['paired']['status'] == 'mismatch'
                  or any(comparison['layers'][layer]['status'] == 'mismatch' for layer in ('k', 'quint'))
                  or difference is not None)
        loop.update(comparison_status='mismatch' if failed else 'match', first_divergence=public_difference(difference),
            first_divergence_disposition=(
                'Computed now from retained pinned K/Quint observations against the independent oracle; '
                'producers not rerun' if retained_comparison else
                'Supplied report diagnostic, not authenticated execution'),
            comparison_input=ref, execution=('compared-locally; producers-not-rerun' if retained_comparison
                                             else 'not-established-by-supplied-report'),
            independent_expectations=selected_oracles,
            selection=dict(case_id=selection['case_id'], initial_fuel=selection['initial_fuel']))
        for row in layers[:2]:
            row['comparison_status'] = comparison['layers'][row['layer'].lower()]['status']
    return dict(format='sherlock-clean-stage/1', requirement='UIREQ-026',
        status='mismatch' if loop['comparison_status'] == 'mismatch' else 'package-ready',
        stage_status='required-development-stage; correspondence-open', profile=qa.PROFILE,
        design_sources=[source_member(STAGE + '/clean-stage.md'),
                        source_member('docs/SHERLOCK-CORRECTNESS-LOOP.md')],
        tasks=tasks, public_reuse=public, private_integration=private_binding(private_record),
        historical_lean=dict(status='historical-narrow-evidence', normal_cases=8,
            zero_remaining_publications=1, current_execution='not-run',
            establishes='Pure first-order Lean reference and finite publication examples only',
            excludes=['B bounded images', 'Clean transition circuits', 'correspondence', 'exporters']),
        correctness_loop=loop,
        unproved_gaps=['K/Lean construct, rule, outcome and work correspondence',
            'ReferenceState/MachineImage representation and admitted codec round trips',
            'B helper-metered step/run and cap-boundary/charge correspondence',
            'Lean meaning/bounded Clean step and actual constraint soundness/completeness',
            'Handwritten circuit component and shared dependency membership',
            'ConstraintPlan/WitnessPlan capability-checked exporter',
            'Clean/native/ZKIR/ledger-9 stock-verifier correspondence',
            'Finance, effects, duties, await/resume and target-phase qualification'],
        midnight_exporter='not-implemented', execution_authority='none', acceptance='none',
        public_developer_requirements='Lean/Clean maintainer tooling is not a compile/prove/deploy prerequisite')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    compare = commands.add_parser('projection')
    compare.add_argument('--k-observations', type=Path)
    compare.add_argument('--quint-observations', type=Path)
    compare.add_argument('--case')
    compare.add_argument('--fuel', type=int)
    package = commands.add_parser('stage')
    package.add_argument('--comparison', type=Path)
    package.add_argument('--private-record', type=Path)
    package.add_argument('--retained-comparison', action='store_true',
                         help='re-compare the retained pinned F03 K/Quint pair in-process')
    binding = commands.add_parser('bind-private')
    binding.add_argument('--integration-root', required=True, type=Path)
    binding.add_argument('--record', required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        if args.command == 'projection':
            result = projection(args.k_observations, args.quint_observations, args.case, args.fuel)
        elif args.command == 'stage':
            result = stage(args.comparison, args.private_record, args.retained_comparison)
        else:
            result = bind_private(args.integration_root, args.record)
    except (OSError, ValueError, KeyError, TypeError, RecursionError, EOFError) as error:
        # Private exception paths/content must not appear in public stdout.
        result = dict(status='input-error', error_type=type(error).__name__,
                      reason='Input missing, malformed, unsupported or pin mismatch; check the declared interface')
        print(json.dumps(result))
        return 2
    print(json.dumps(result, indent=2, allow_nan=False))
    return 1 if result.get('status') == 'mismatch' else 0


if __name__ == '__main__':
    raise SystemExit(main())
