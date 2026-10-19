#!/usr/bin/env python3
"""Wrapper-only construction from previously frozen independent preimages.

No old derivation execution, parser/product/CLI/compiler/provider imports or
execution. All writes target this NEW packet. Old artifacts are read-only hashes
except the explicitly authorized literal Program/Plan/source inputs.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
OLD = ROOT / 'core3-public-financial-complete-independent-oracle-20261019'
CC = Path('/home/charl/Moriarty/.worktrees/doctorwatson-core3-mcp-combined-20261019')
CONTRACT = ROOT / 'core3-public-consumer-contract-preparation-20261019/CONTRACT.md'
OLD_MANIFEST_SHA = '1887b8397a7650f8378029804fe8ebcfaa1b700f390a52729de635b25f45f2bc'


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def clone(value):
    return json.loads(json.dumps(value))


def encode(value):
    # Every selected source/report string is ASCII. Recursive lexical keys,
    # retained arrays, two-space JSON indent, one final LF match the explicitly
    # inspected bounded wrapper export format for this finite value subset.
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True) + '\n').encode()


def write(name, value):
    (HERE / name).write_bytes(encode(value))


old_manifest_raw = (OLD / 'FREEZE-MANIFEST.json').read_bytes()
assert sha(old_manifest_raw) == OLD_MANIFEST_SHA
old_manifest = json.loads(old_manifest_raw)
expected_inventory = {row['path']: row for row in old_manifest['artifacts']}
expected_inventory['FREEZE-MANIFEST.json'] = {'path': 'FREEZE-MANIFEST.json',
    'bytes': len(old_manifest_raw), 'sha256': OLD_MANIFEST_SHA}
assert len(old_manifest['artifacts']) == 59
assert len(expected_inventory) == 60  # all59 artifact entries plus the manifest


def preservation_snapshot():
    actual_names = {str(path.relative_to(OLD)) for path in OLD.rglob('*') if path.is_file()}
    assert actual_names == set(expected_inventory)
    observed = []
    for name in sorted(actual_names):
        # Hashing all59 for preservation is not semantic inspection or JSON
        # loading of prior results, derives, simulations, or expected files.
        raw = (OLD / name).read_bytes()
        pin = expected_inventory[name]
        assert len(raw) == pin['bytes'] and sha(raw) == pin['sha256']
        observed.append({'path': name, 'bytes': len(raw), 'sha256': sha(raw)})
    return observed


before = preservation_snapshot()
value_inputs = []
programs = {}
plans = {}
sources = {}
for name in ('P1', 'P2'):
    for kind, values in (('program', programs), ('plan', plans)):
        path = OLD / 'canonical' / (name + '-' + kind + '.json')
        raw = path.read_bytes()
        values[name] = json.loads(raw)
        # Confirm these are the exact old canonical byte literals, never
        # regenerating or executing the old script.
        assert json.dumps(values[name], sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode() == raw
        value_inputs.append({'path': str(path), 'bytes': len(raw), 'sha256': sha(raw),
            'role': 'only independently chosen whole ' + kind + ' value source'})
    source_path = OLD / 'inputs' / (name + '-RECONSTRUCTED-INPUT.mori')
    raw_source = source_path.read_bytes()
    sources[name] = raw_source
    (HERE / (name + '.mori')).write_bytes(raw_source)
    value_inputs.append({'path': str(source_path), 'bytes': len(raw_source), 'sha256': sha(raw_source),
        'role': 'original selected literal source bytes, renamed only to chosen CLI argument ' + name + '.mori'})
    assert plans[name]['program_hash'] == sha((OLD / 'canonical' / (name + '-program.json')).read_bytes())
    assert programs[name]['language'] == 'moriarty/3' and programs[name]['semantics'] == 'moriarty-core/3'
    assert programs[name]['name'] == 'P'

labels = {'qualification': 'local-reference-stage', 'authentication': 'none', 'consumption': 'untracked'}
reports = {}
index = []
field_provenance = []
for name in ('P1', 'P2'):
    program = programs[name]
    check = dict(labels, status='accepted', file=name + '.mori', source_hash=sha(sources[name]),
        language=program['language'], semantics=program['semantics'], agreement=program['name'],
        actions=clone(program['actions']), diagnostics=[])
    inspect = dict(labels, status='accepted', language=program['language'], semantics=program['semantics'],
        agreement=program['name'], states=clone(program['states']), assets=clone(program['assets']),
        duties=clone(program['duties']), actions=clone(program['actions']), expressions=clone(program['expressions']),
        stage_plan=clone(plans[name]), enforcement_report=None)
    for operation, report in (('check', check), ('inspect', inspect)):
        report_name = name + '-' + operation
        reports[report_name] = report
        write(report_name + '.json', report)
        stdout = encode(report)
        (HERE / (report_name + '.stdout.txt')).write_bytes(stdout)
        (HERE / (report_name + '.stderr.txt')).write_bytes(b'')
        arguments = ['mori', '--color', 'never', operation, name + '.mori']
        if operation == 'check':
            arguments.append('--json')
        index.append({'name': report_name, 'argv': arguments, 'cwd': str(HERE),
            'expected_json': report_name + '.json', 'stdout': report_name + '.stdout.txt',
            'stdout_bytes': len(stdout), 'stdout_sha256': sha(stdout), 'final_LF_count': 1,
            'stderr': report_name + '.stderr.txt', 'stderr_bytes': 0, 'expected_exit': 0,
            'complete_object_no_unfilled_fields': True,
            'scope': 'conditional accepted local check/inspect wrapper; not actual execution or adoption'})
        source_fields = {'language': '/language', 'semantics': '/semantics', 'agreement': '/name',
            'actions': '/actions', 'states': '/states', 'assets': '/assets', 'duties': '/duties', 'expressions': '/expressions'}

        def visit(value, pointer, top, suffix):
            if top in source_fields:
                origin = {'kind': 'old independently frozen whole Program literal',
                    'input_path': str(OLD / 'canonical' / (name + '-program.json')),
                    'input_pointer': source_fields[top] + suffix}
            elif top == 'stage_plan':
                origin = {'kind': 'old independently frozen whole StagePlan literal',
                    'input_path': str(OLD / 'canonical' / (name + '-plan.json')), 'input_pointer': suffix or '/'}
            elif top == 'source_hash':
                origin = {'kind': 'independent SHA256 of exact original selected source bytes',
                    'source_file': name + '.mori', 'source_sha256': sha(sources[name]),
                    'format': 'prior permitted intent::hash_bytes definition; lowercase64 hex, local metadata only'}
            elif top == 'file':
                origin = {'kind': 'chosen literal CLI path argument', 'value': name + '.mori',
                    'wrapper': 'CC check.rs args.file.display().to_string(), no absolute canonicalization'}
            elif top in labels:
                origin = {'kind': 'declared local stage label, projected by current wrapper', 'value': labels[top]}
            else:
                origin = {'kind': 'conditional accepted wrapper constant', 'value': value,
                    'wrapper': 'CC check_source.rs accepted summary or inspect.rs /3 accepted source-only constructor'}
            field_provenance.append({'report': report_name, 'pointer': pointer, 'origin': origin,
                'node_type': 'object' if isinstance(value, dict) else 'array' if isinstance(value, list) else 'scalar'})
            if isinstance(value, dict):
                for key, child in value.items():
                    escaped = key.replace('~', '~0').replace('/', '~1')
                    visit(child, pointer + '/' + escaped, top, suffix + '/' + escaped)
            elif isinstance(value, list):
                for ordinal, child in enumerate(value):
                    visit(child, pointer + '/' + str(ordinal), top, suffix + '/' + str(ordinal))
        for top, value in report.items():
            visit(value, '/' + top, top, '')

assert len(reports) == 4
for name in ('P1', 'P2'):
    assert reports[name + '-check']['actions'] == programs[name]['actions']
    assert reports[name + '-inspect']['duties'] == programs[name]['duties']
    assert reports[name + '-inspect']['expressions'] == programs[name]['expressions']
    assert reports[name + '-inspect']['stage_plan'] == plans[name]

write('EXPECTED-INVOCATIONS.json', {'schema': 'moriarty-independent-nonempty-check-inspect-invocations/1',
    'invocations': index, 'actual_execution_performed': False,
    'normal_stream_rule': 'complete bounded ASCII pretty JSON, lexical keys, array order retained, two-space indent, one final LF; selected strings contain ASCII only'})
write('PER-FIELD-PROVENANCE.json', {'schema': 'moriarty-independent-check-inspect-per-field/1',
    'coverage': 'Every object member, array container/element and scalar in all four expected reports has an origin. No filtering to selected action.',
    'fields': field_provenance})
conditions = [
    {'id': 'D1', 'status': 'proposed public contract not adopted by this packet',
     'premise': 'Exactly recognized /3 sources use the real /3 source check/inspect route and complete normal admission under existing owner-selected conditions.'},
    {'id': 'D2', 'status': 'conditional source-format premise; no preimplementation independence',
     'premise': 'Current wrapper fields/format match inspected CC constructors. Their contents use only previously independently frozen Program/Plan values, and explicit /3 representation/report assumptions remain conditional.'},
    {'id': 'LOCAL-DIGEST', 'status': 'local metadata projection only',
     'premise': 'source_hash is SHA256(raw selected source bytes); inspect program_hash is copied from the exact old independently frozen StagePlan, with the original local canonical-hash qualification. No hash soundness, identity authority, proof or native claim.'},
    {'id': 'NORMAL-PROCESS', 'status': 'conditional accepted normal wrapper/process definition',
     'premise': 'Chosen args include --color never; check --json emits no shell status, source-only inspect uses no shell output, accepted returns exit0. No I/O/export/validation failure is asserted from a product run.'},
]
write('CONDITIONS-AND-UNKNOWNS.json', {'schema': 'moriarty-independent-wrapper-conditions/1',
    'conditions': conditions, 'unfilled_fields_in_four_conditional_objects': [],
    'adoption_not_established': True, 'source_format_exposure_not_semantic_product_proof': True,
    'whole_finance_general_language_compiler_proof_native_ledger_acceptance': False,
    'source_custody': 'Original literal sources were reconstructed in the first source-owner-input packet; byte identity here is to those selected original independent source files, not an invented original security-author source identity.'})

wrapper_spans = {
    'crates/mori/src/commands/check_source.rs': [(93, 123)],
    'crates/mori/src/commands/check.rs': [(23, 55), (96, 116)],
    'crates/mori/src/commands/inspect.rs': [(1, 435)],
    'crates/mori/src/commands/core_io.rs': [(247, 252)],
    'crates/mori/src/commands/simulate.rs': [(156, 170)],
    'crates/mori-core/src/json.rs': [(117, 155)],
}
source_exposure = []
for relative, spans in wrapper_spans.items():
    path = CC / relative
    raw = path.read_bytes()
    lines = raw.decode().splitlines(keepends=True)
    source_exposure.append({'path': str(path), 'whole_file_sha256': sha(raw),
        'actual_read_spans': [{'first': first, 'last': last,
            'span_utf8_sha256': sha(''.join(lines[first - 1:last]).encode())} for first, last in spans],
        'use': 'wrapper constructor/label/stream serialization only; no Program/Plan/financial values derived from current code'})
write('CURRENT-WRAPPER-PINS-AND-EXPOSURE.json', {'schema': 'moriarty-current-wrapper-format-pins/1',
    'root_declared_base': '0ad8a7e7e9cbd63b6e5d002139de26535abf3025',
    'root_declared_base_tree': '63226695eea932993bbf3f9569cca65899f08a92',
    'candidate_state': 'current uncommitted combined overlay; no git invoked by this author',
    'files': source_exposure,
    'extra_exposure_correction': 'Whole inspect.rs was opened instead of only its /3 wrapper. Unused /1/native enforcement-map helper source was visible; no helper-derived semantic/security values or conclusions used. The root was informed and authorized continuing.',
    'incidental_json_exposure': 'json.rs117..155 includes following small parser duplicate/trailing-JSON test assertions unrelated to selected P1/P2 expected fixtures; no selected source-author fixture/golden/result read or used.',
    'metadata_searches': 'Function/name/constructor/print rg searches on these wrapper paths; no broad runtime/evaluator search.'})

# Verify full old59 again after construction. This does not load old results,
# run any old script, or change a single old file.
after = preservation_snapshot()
assert before == after
write('OLD59-PRESERVATION.json', {'schema': 'moriarty-old59-byte-preservation/1',
    'old_packet': str(OLD), 'old_manifest_sha256_before': OLD_MANIFEST_SHA,
    'old_manifest_sha256_after': sha((OLD / 'FREEZE-MANIFEST.json').read_bytes()),
    'expected_and_observed_artifact_count_excluding_manifest': 59,
    'expected_and_observed_total_file_count_including_manifest': 60,
    'no_extra_missing_or_changed_paths': True,
    'before_equals_after': before == after, 'every_file_matches_original_manifest_size_and_sha256': True,
    'all_file_observations_before': before, 'all_file_observations_after': after,
    'old_derive_executed': False, 'old_packet_writes': False})
provenance = {'schema': 'moriarty-independent-nonempty-wrapper-provenance/1',
    'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'scope': 'Four chosen complete ordinary check/inspect report objects and exact stdout/process expectations, from old whole independent Program/Plan literals plus disclosed current-wrapper format.',
    'value_inputs': value_inputs,
    'public_contract': {'path': str(CONTRACT), 'sha256': sha(CONTRACT.read_bytes()), 'status': 'proposed D1/D2, not adopted'},
    'identity': 'Requested gpt-6.1-sol high, inherited agent; no separate provider/model receipt or network call.',
    'prior_exposure': 'Same persistent context authored prior independent financial/Program/Plan predictions and saw its own generated packet. Earlier empty-format task exposed baseline DTO/canonical/hash/caller/aggregate formats. Current task uses only authorized old Program/Plan preimages/source bytes as semantic values.',
    'independence_limit': 'Current-source-exposed wrapper prediction, not preimplementation independence or whole financial-independent implementation proof. Frozen before the next four chosen check/inspect comparisons; parent reports earlier simulations occurred, their outputs were not read.',
    'excluded_reads': 'No actual check/inspect/simulation result packet, selected candidate P1/P2 tests/goldens, source-author expected fixtures, runtime/parser/admit/validate/cost evaluator implementation or provider output opened. Old59 hashing reads bytes for preservation only.',
    'excess_exposure': 'Precisely disclosed in CURRENT-WRAPPER-PINS-AND-EXPOSURE.json, including whole inspect.rs and incidental unrelated parser assertions. No conclusions from native/authorization helper bodies.',
    'execution': {'new_standalone_stdlib_script_only': True, 'old_script_rerun': False,
        'product_parser_cli_backend_compiler_provider_sdk_native_ledger_network_execution': False,
        'git_or_shared_writes': False,
        'startup': 'develop skill and active AGENTS reread; read-only plugin status refreshed, historical SP01.6 missing evidence and no pending transactions.'},
    'conditions': conditions, 'derivation_script_sha256': sha((HERE / 'derive.py').read_bytes()),
    'no_test_or_acceptance_claim': True}
write('PROVENANCE.json', provenance)
(HERE / 'README.md').write_text('''# Nonempty Core3 check/inspect conditional oracle

Four complete JSON objects and exact pretty stdout bytes cover these chosen
commands from this directory, with --color never: check P1.mori --json,
inspect P1.mori, check P2.mori --json, inspect P2.mori. Expected normal exit is0
and stderr is empty. Literal source bytes are copied unchanged from the original
independent packet and renamed only to P1.mori/P2.mori for these invocations.

Every semantic value comes from the exact previously independently frozen whole
Program/StagePlan canonical literals. No action, declaration, duty, expression,
site or cost vector is filtered or recomputed by product logic. Wrapper constants,
fields and stream format use explicitly disclosed current source. D1/D2 and local
canonical-digest status remain conditional. This is not preimplementation
independence, a financial implementation proof or observed output.

EXPECTED-INVOCATIONS.json lists commands, file paths, stdout hashes and process
expectations. PER-FIELD-PROVENANCE.json maps every report container/scalar to its
old literal input or wrapper/command constant. CURRENT-WRAPPER-PINS-AND-EXPOSURE
records exact source spans and excess exposure: whole inspect.rs and incidental
unrelated parser assertions were visible and not used for semantic/security values.
Actual result packets, selected tests/goldens and runtime algorithms were not read.

OLD59-PRESERVATION.json verifies all59 original listed artifacts plus their
manifest (60 files total) before and after: same exact paths, lengths and SHA256
values. No old script ran or old packet byte was written. Only this
new standalone stdlib construction ran; no Moriarty CLI/parser/backend/compiler,
provider, SDK, native/proof, network or ledger execution occurred. Root compares
actuals after freeze. All financial/general/compiler/proof/target exits remain.
FREEZE-MANIFEST.json hashes this new packet except itself. Preserve it before
reproduction, which updates timestamps. No test-pass or acceptance is claimed.
''')
artifact_paths = sorted(path for path in HERE.iterdir() if path.is_file() and path.name != 'FREEZE-MANIFEST.json')
write('FREEZE-MANIFEST.json', {'schema': 'moriarty-independent-nonempty-check-inspect-freeze/1',
    'frozen_at_utc': provenance['frozen_at_utc'], 'before_next_four_chosen_consumer_comparisons_in_author_task': True,
    'conditional_wrapper_format_scope': True, 'old59_preserved': True,
    'artifacts': [{'path': path.name, 'bytes': path.stat().st_size, 'sha256': sha(path.read_bytes())} for path in artifact_paths],
    'manifest_self_hash_excluded': True})
print(json.dumps({'old59_preserved': True, 'original_artifact_count': 59, 'total_old_files_including_manifest': 60,
    'expected_stdout': {row['name']: {'bytes': row['stdout_bytes'],
    'sha256': row['stdout_sha256']} for row in index}, 'freeze_manifest_sha256': sha((HERE / 'FREEZE-MANIFEST.json').read_bytes())}, sort_keys=True))
