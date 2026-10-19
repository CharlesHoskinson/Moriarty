#!/usr/bin/env python3
"""Finite source/input rederivation; no product imports or execution.

This context has prior mathematical/format exposure, explicitly disclosed. New
whole reports and bytes are derived before any NEW whole-wire observation from
the selected literals and conditional normative premises, not old oracle files.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
ORIGINAL = ROOT / 'core3-positive-finance-independent-freeze-20261019'
BASE = Path('/home/charl/Moriarty/.worktrees/utxo-core3-validator-local-20261019')
CHECKOUT = Path('/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018')
REQ = ROOT / 'core3-runtime-admission-muse-source-owner-preparation-20261019/packet/requirements'
CONTRACT = ROOT / 'core3-public-consumer-contract-preparation-20261019/CONTRACT.md'
for dirname in ('inputs', 'programs', 'plans', 'reports', 'continuations', 'canonical', 'stdout'):
    (HERE / dirname).mkdir(exist_ok=True)


def clone(value):
    return json.loads(json.dumps(value))


def sha_bytes(value):
    return hashlib.sha256(value).hexdigest()


def sha_file(path):
    return sha_bytes(path.read_bytes())


def pretty(value):
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=True) + '\n').encode()


def canonical(value):
    # Independent finite ASCII implementation of the permitted baseline format:
    # recursively lexical object keys, retained arrays, compact JSON, no LF.
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode()


def write(path, value):
    (HERE / path).write_bytes(pretty(value))


inputs = json.loads((ORIGINAL / 'SOURCE-INPUTS.json').read_text())
assert inputs['language'] == 'moriarty/3' and inputs['agreement'] == 'P' and inputs['cap'] is None
assert inputs['reconstructed_source_bytes_are_original'] is False
assert len(inputs['programs']) == 2
p1_decl, p2_decl = inputs['programs']
assert p1_decl['id'] == 'P1' and p2_decl['id'] == 'P2'
assert p1_decl['statements'] == ['discharge("loan", 3);']
assert p2_decl['statements'] == ['await ready(go: Bool) when go;', 'discharge("loan", 3);']
asset_decl = clone(inputs['asset'])
asset = clone(asset_decl['identity'])
duty_decl = clone(inputs['source_duty'])
assert asset_decl == {'name': 'A', 'identity': {'network': 'local', 'issuer': 'i', 'reference': 'A', 'scale': 0}}
assert duty_decl == {'name': 'loan', 'asset': 'A', 'debtor': 'alice', 'creditor': 'bob'}
assert p1_decl['action'] == p2_decl['action'] == 'pay'
for name in ('P1-RECONSTRUCTED-INPUT.mori', 'P2-RECONSTRUCTED-INPUT.mori', 'SOURCE-INPUTS.json'):
    (HERE / 'inputs' / name).write_bytes((ORIGINAL / name).read_bytes())
source_prefix = '''language "moriarty/3";
agreement P {
  asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
  duty "loan" = { asset: A, debtor: "alice", creditor: "bob" };
  action pay() {
'''
assert (ORIGINAL / 'P1-RECONSTRUCTED-INPUT.mori').read_text() == source_prefix + '    discharge("loan", 3);\n  }\n}\n'
assert (ORIGINAL / 'P2-RECONSTRUCTED-INPUT.mori').read_text() == source_prefix + '    await ready(go: Bool) when go;\n    discharge("loan", 3);\n  }\n}\n'

# Explicit finite M01-preservation premise: metadata allocates no arena nodes;
# written source occurrences allocate fresh nodes in lexical/postorder sequence.
uint3 = {'type': 'UInt', 'value': '3'}
literal3 = {'kind': 'Literal', 'value': uint3}
go_name = {'kind': 'Name', 'name': 'go'}
await_statement = {'kind': 'Await', 'label': 'ready', 'binders': [{'name': 'go', 'ty': 'Bool'}], 'guard': 0}
discharge_p1 = {'kind': 'Discharge', 'name': 'loan', 'amount': 0}
discharge_p2 = {'kind': 'Discharge', 'name': 'loan', 'amount': 1}


def program(expressions, statements):
    return {'language': 'moriarty/3', 'semantics': 'moriarty-core/3', 'name': 'P',
            'states': [], 'assets': [clone(asset_decl)], 'duties': [clone(duty_decl)],
            'expressions': clone(expressions),
            'actions': [{'name': 'pay', 'inputs': [], 'statements': clone(statements)}]}


programs = {'P1': program([literal3], [discharge_p1]),
            'P2': program([go_name, literal3], [await_statement, discharge_p2])}
hash_records = []


def freeze_canonical(name, value, role):
    raw = canonical(value)
    relative = 'canonical/' + name + '.json'
    (HERE / relative).write_bytes(raw)
    record = {'path': relative, 'bytes': len(raw), 'sha256': sha_bytes(raw), 'role': role,
              'encoding': 'compact lexical-key JSON, array order retained, no final LF; all selected strings ASCII'}
    hash_records.append(record)
    return record['sha256']


program_hashes = {}
plan_hashes = {}
plans = {}
for key, value in programs.items():
    write('programs/' + key + '.json', value)
    program_hashes[key] = freeze_canonical(key + '-program', value, 'product program_hash under declared baseline-format/Core3-preservation premises')

dispatch = 1
literal_visit = 1
guard_visit = 1
discharge_work = dispatch + literal_visit
await_entry_work = dispatch
resume_guard_work = dispatch + guard_visit
resume_suffix_work = resume_guard_work + discharge_work
assert discharge_work == 2 and await_entry_work == 1 and resume_guard_work == 2 and resume_suffix_work == 4
action_plans = {
    'P1': {'action': 'pay', 'statement_count': len(p1_decl['statements']), 'sites': [],
           'segment_cost': [str(discharge_work)], 'committed_before': ['0'], 'plan_total': str(discharge_work)},
    'P2': {'action': 'pay', 'statement_count': len(p2_decl['statements']),
           'sites': [{'site': 1, 'ordinal': 0, 'label': 'ready', 'binders': [{'name': 'go', 'ty': 'Bool'}], 'captures': []}],
           'segment_cost': [str(await_entry_work), str(resume_suffix_work)],
           'committed_before': ['0', str(await_entry_work)],
           'plan_total': str(await_entry_work + resume_suffix_work)},
}
for key, action_plan in action_plans.items():
    plans[key] = {'format': 'moriarty-stage-plan/1', 'program_hash': program_hashes[key], 'actions': [action_plan]}
    write('plans/' + key + '.json', plans[key])
    plan_hashes[key] = freeze_canonical(key + '-plan', plans[key], 'product plan_hash under declared baseline-format/Core3-preservation premises')


def snapshot(scenario):
    assert scenario['inputs'] == {} and scenario['state'] == {} and scenario['balances'] == []
    assert len(scenario['liabilities']) == 1
    row = scenario['liabilities'][0]
    assert row['name'] == 'loan' and row['asset'] == asset and row['debtor'] == 'alice' and row['creditor'] == 'bob'
    return {key: clone(scenario[key]) for key in ('state', 'balances', 'liabilities')}


def discharge(opening):
    closing = clone(opening)
    before = int(opening['liabilities'][0]['amount'])
    after = before - int(uint3['value'])
    assert 0 <= before <= 2**128 - 1 and 0 <= after <= 2**128 - 1
    closing['liabilities'][0]['amount'] = str(after)
    effect = {'kind': 'Liability', 'name': 'loan', 'asset': clone(asset),
              'debtor': 'alice', 'creditor': 'bob', 'before': str(before), 'after': str(after)}
    return closing, effect


def residual(boundary):
    return [clone(row) for row in boundary['liabilities'] if int(row['amount']) > 0]


def execution(opening, closing, steps, effects):
    return {'semantics': 'moriarty-core/3', 'action': 'pay', 'pre': clone(opening), 'post': clone(closing),
            'steps': clone(steps), 'effects': clone(effects)}


idle = {'site': 0, 'captures': []}
pending = {'site': 1, 'captures': []}
labels = {'qualification': 'local-reference-stage', 'authentication': 'none', 'consumption': 'untracked'}


def segment(opening, closing, steps, effects, frame_pre, frame_post):
    return {'action': 'pay', 'frame_pre': clone(frame_pre), 'frame_post': clone(frame_post),
            'execution': execution(opening, closing, steps, effects)}


def completed(seg, attempted, committed):
    return dict(labels, status='complete', segment=seg, attempted=str(attempted), committed=str(committed),
                residual=residual(seg['execution']['post']))


def discharge_step(index, statement, closing, effect):
    return {'index': index, 'statement': clone(statement), 'value': clone(uint3),
            'snapshot': clone(closing), 'effects': [clone(effect)]}


reports = {}
operation_records = []
input_scenarios = []
for scenario_index, supplied_scenario in enumerate(p1_decl['initial_scenarios']):
    opening = snapshot(supplied_scenario)
    closing, effect = discharge(opening)
    amount = opening['liabilities'][0]['amount']
    assert amount in ('5', '3')
    case = 'P1-initial' + amount
    input_path = 'inputs/' + case + '-scenario.json'
    write(input_path, supplied_scenario)
    input_scenarios.append(input_path)
    name = case + '-start'
    step = discharge_step(0, discharge_p1, closing, effect)
    reports[name] = completed(segment(opening, closing, [step], [effect], idle, idle), discharge_work, discharge_work)
    operation_records.append({'name': name, 'case': case, 'operation': 'start', 'program': 'P1',
        'scenario': input_path, 'source': 'inputs/P1-RECONSTRUCTED-INPUT.mori',
        'opening': opening, 'closing': closing, 'frame_before': clone(idle), 'frame_after': clone(idle),
        'statement_financial_prefixes': [clone(opening), clone(closing)],
        'statement_effect_deltas': [[clone(effect)]], 'statement_effect_prefixes': [[], [clone(effect)]],
        'history_steps_after': [clone(step)], 'history_effects_before': [], 'history_effects_after': [clone(effect)],
        'attempted': str(discharge_work), 'committed_before': '0', 'committed_after': str(discharge_work),
        'cumulative_attempted': str(discharge_work), 'cap': None})

p2_scenario = p2_decl['initial_scenario']
write('inputs/P2-initial5-scenario.json', p2_scenario)
opening = snapshot(p2_scenario)
assert opening['liabilities'][0]['amount'] == '5'
closing, effect = discharge(opening)
assert len(p2_decl['events']) == 2
event_inputs = []
for index, event_input in enumerate(p2_decl['events']):
    assert event_input['action'] == 'pay' and event_input['label'] == 'ready'
    assert event_input['values'] == [{'name': 'go', 'value': {'type': 'Bool', 'value': bool(index)}}]
    event = dict(event_input, format='moriarty-stage-event/1')
    name = 'P2-event-' + ('false' if index == 0 else 'true')
    write('inputs/' + name + '.json', event)
    freeze_canonical(name, event, 'diagnostic StageEvent byte identity, not a product event-hash field')
    event_inputs.append(event)

continuation = {'format': 'moriarty-continuation/1', 'program_hash': program_hashes['P2'], 'plan_hash': plan_hashes['P2'],
    'action': 'pay', 'site': 1, 'captures': [],
    'logical': {'profile': 'moriarty-reference-logical-cost/2', 'cap': None, 'committed': str(await_entry_work)},
    'boundary': clone(opening)}
write('continuations/P2-after-start.json', continuation)
write('continuations/P2-after-false.json', continuation)
freeze_canonical('P2-continuation-after-start', continuation, 'diagnostic continuation byte identity; no product continuation-hash field')
freeze_canonical('P2-continuation-after-false', continuation, 'diagnostic continuation byte identity; unchanged object and bytes')
assert canonical(continuation) == canonical(clone(continuation))

reports['P2-start'] = dict(labels, status='waiting', advanced=True,
    segment=segment(opening, opening, [], [], idle, pending), continuation=clone(continuation),
    attempted=str(await_entry_work), committed=str(await_entry_work), residual=residual(opening))
reports['P2-resume-false'] = dict(labels, status='waiting', advanced=False, segment=None,
    continuation=clone(continuation), attempted=str(resume_guard_work),
    committed=str(await_entry_work), residual=residual(opening))
await_step = {'index': 0, 'statement': clone(await_statement), 'value': {'type': 'Bool', 'value': True},
              'snapshot': clone(opening), 'effects': []}
last_step = discharge_step(1, discharge_p2, closing, effect)
reports['P2-resume-true'] = completed(segment(opening, closing, [await_step, last_step], [effect], pending, idle),
    resume_suffix_work, await_entry_work + resume_suffix_work)

for operation_index, name in enumerate(('P2-start', 'P2-resume-false', 'P2-resume-true')):
    final = operation_index == 2
    operation_records.append({'name': name, 'case': 'P2-initial5-false-true',
        'operation': 'start' if operation_index == 0 else 'resume', 'program': 'P2',
        'scenario': 'inputs/P2-initial5-scenario.json' if operation_index == 0 else None,
        'event': None if operation_index == 0 else ('inputs/P2-event-false.json' if operation_index == 1 else 'inputs/P2-event-true.json'),
        'source': 'inputs/P2-RECONSTRUCTED-INPUT.mori',
        'opening': clone(opening), 'closing': clone(closing if final else opening),
        'frame_before': clone(idle if operation_index == 0 else pending), 'frame_after': clone(idle if final else pending),
        'statement_financial_prefixes': [clone(opening), clone(opening), clone(closing)] if final else [],
        'statement_effect_deltas': [[], [clone(effect)]] if final else [],
        'statement_effect_prefixes': [[], [], [clone(effect)]] if final else [],
        'history_steps_after': [clone(await_step), clone(last_step)] if final else [],
        'history_effects_before': [], 'history_effects_after': [clone(effect)] if final else [],
        'retained_operation_financial_history': [clone(opening)] * (operation_index + 1) + [clone(closing if final else opening)],
        'attempted': str((await_entry_work, resume_guard_work, resume_suffix_work)[operation_index]),
        'committed_before': '0' if operation_index == 0 else str(await_entry_work),
        'committed_after': str(await_entry_work + resume_suffix_work if final else await_entry_work),
        'cumulative_attempted': str(sum((await_entry_work, resume_guard_work, resume_suffix_work)[:operation_index + 1])),
        'cap': None, 'false_guard_has_public_step_or_segment': False if operation_index == 1 else None})

assert len(reports) == len(operation_records) == 5
assert reports['P1-initial3-start']['segment']['execution']['post']['liabilities'][0]['amount'] == '0'
assert reports['P1-initial3-start']['residual'] == []
assert reports['P2-start']['continuation'] == reports['P2-resume-false']['continuation']
assert reports['P2-resume-false']['segment'] is None
assert [reports[name]['attempted'] for name in ('P2-start', 'P2-resume-false', 'P2-resume-true')] == ['1', '2', '4']
assert [reports[name]['committed'] for name in ('P2-start', 'P2-resume-false', 'P2-resume-true')] == ['1', '1', '5']

for record in operation_records:
    before = int(record['opening']['liabilities'][0]['amount'])
    after = int(record['closing']['liabilities'][0]['amount'])
    record['complete_financial_summary'] = {
        'opening_liability_present': True, 'closing_liability_present': True,
        'opening_loan_amount': str(before), 'closing_loan_amount': str(after),
        'creation': '0', 'discharge': str(before - after),
        'unchanged_duty_identity': {'name': 'loan', 'asset': asset, 'debtor': 'alice', 'creditor': 'bob'},
        'all_other_liabilities_absent': True, 'alice_and_bob_balance_rows_remain_absent': True,
        'complete_opening_state': {}, 'complete_closing_state': {},
        'complete_opening_balances': [], 'complete_closing_balances': [],
        'gross_debit': '0', 'gross_credit': '0', 'explicit_fee': '0', 'net_cash_flow': '0', 'net_receipts': '0',
        'payment_by_discharge': False, 'liability_change': str(after - before), 'positive_residual': residual(record['closing']),
        'native_dust_service_fee_proof_cost': 'unmeasured, not asserted zero'}
    name = record['name']
    write('reports/' + name + '.json', reports[name])
    (HERE / 'stdout' / (name + '.txt')).write_bytes(pretty(reports[name]))
    record['report_path'] = 'reports/' + name + '.json'
    record['stdout_path'] = 'stdout/' + name + '.txt'
    record['stdout_bytes'] = len(pretty(reports[name]))
    record['stdout_sha256'] = sha_bytes(pretty(reports[name]))
    record['expected_normal_process_under_selected_public_premise'] = {'exit': 0, 'stderr': ''}
    freeze_canonical(name + '-report', reports[name], 'diagnostic complete report canonical byte identity, not a product field')
    freeze_canonical(name + '-opening-snapshot', record['opening'], 'diagnostic complete Snapshot byte identity, not a product frame_hash')
    freeze_canonical(name + '-closing-snapshot', record['closing'], 'diagnostic complete Snapshot byte identity, not a product frame_hash')
freeze_canonical('idle-pending-frame', idle, 'diagnostic PendingFrame byte identity, not a product frame_hash')
freeze_canonical('site1-pending-frame', pending, 'diagnostic PendingFrame byte identity, not a product frame_hash')

# A separately identified proposed whole-report manifest. Expected report files
# never provide an actual continuation; the real harness must chain actuals.
manifest_cases = []
for initial in ('5', '3'):
    manifest_cases.append({'name': 'independent-P1-loan' + initial,
        'source': 'inputs/P1-RECONSTRUCTED-INPUT.mori', 'scenario': 'inputs/P1-initial' + initial + '-scenario.json',
        'action': 'pay', 'logical_cap': None, 'expected': 'reports/P1-initial' + initial + '-start.json', 'events': []})
manifest_cases.append({'name': 'independent-P2-false-true', 'source': 'inputs/P2-RECONSTRUCTED-INPUT.mori',
    'scenario': 'inputs/P2-initial5-scenario.json', 'action': 'pay', 'logical_cap': None,
    'expected': 'reports/P2-start.json', 'events': [
        {'event': 'inputs/P2-event-false.json', 'expected': 'reports/P2-resume-false.json'},
        {'event': 'inputs/P2-event-true.json', 'expected': 'reports/P2-resume-true.json'}]})
write('CONDITIONAL-TEST-MANIFEST.json', {'profile': 'moriarty-tests/3', 'cases': manifest_cases})
aggregate = {'status': 'passed', 'cases': []}
for case, report_names in zip(manifest_cases, (['P1-initial5-start'], ['P1-initial3-start'], ['P2-start', 'P2-resume-false', 'P2-resume-true'])):
    aggregate['cases'].append({'name': case['name'], 'passed': True,
        'stages': [{'index': index, 'passed': True, 'actual': clone(reports[name])} for index, name in enumerate(report_names)]})
write('CONDITIONAL-MANIFEST-AGGREGATE.json', aggregate)
(HERE / 'stdout' / 'manifest-aggregate.txt').write_bytes(pretty(aggregate))

conditions = [
    {'id': 'C-REG3', 'status': 'declared conditional premise, not this author security/adoption decision',
     'premise': 'The proposed /3 DutyDecl/REG3 extension selects exactly the declared loan AssetId/parties and permits these supplied positive financial/staged cases. No synthetic Liability statement or expression is inserted.'},
    {'id': 'C-D1', 'status': 'proposed public contract DRAFT', 'premise': 'Explicit /3 source selection calls load_v3 and the real staged consumer, without profile substitution.'},
    {'id': 'C-D2', 'status': 'proposed public contract DRAFT; Core2 adoption does not automatically adopt Core3',
     'premise': 'For this finite source grammar, /3 explicitly preserves M01 fresh-occurrence arena metadata rules, exact DTO/serde projection, financial subtraction and present-zero retention, staged status/Step/effect/capture/cap rules, primitive work schedule and public ASCII stage bytes, with program Execution.semantics moriarty-core/3.'},
    {'id': 'C-D3', 'status': 'proposed only; applies to optional manifest artifacts',
     'premise': 'moriarty-tests/3 uses complete report equality and actual continuation chaining, retained aggregate format/index ordering, and the selected independent case names/files.'},
    {'id': 'C-FORMAT', 'status': 'disclosed source-exposed baseline format definitions, not producer execution',
     'premise': 'C214b DTO/serde and SHA256(canonical Program/Plan) metadata definitions are used for ordinary byte-format prediction only; raw canonical ASCII compact JSON has lexical keys, array order and no LF.'},
]
write('COMPLETE-OPERATION-INDEX-AND-PREFIXES.json', {'schema': 'moriarty-independent-conditional-public-financial-history/1',
    'conditions': conditions, 'source_program_count': 2, 'scenario_history_count': 3, 'operation_count': 5,
    'reports_are_complete_objects_not_projection_substitutes': True,
    'operations': operation_records,
    'work_equations': {'P1': 'dispatch1 + literal1 = 2', 'P2_segment0': 'await_entry1 = 1',
        'P2_false_attempt': 'resume_dispatch1 + guard_Name1 = 2; committed remains 1',
        'P2_true_attempt': 'resume_dispatch1 + guard_Name1 + discharge_dispatch1 + literal1 = 4',
        'P2_plan_total': 'segment0(1) + successful_segment1(4) = 5',
        'P2_total_attempts_across_supplied_events': '1 + 2 + 4 = 7; not a product committed or lifetime-work field'},
    'original_partial_packet_modified': False,
    'financial_scope': 'Complete for these supplied single-duty/no-balance discharge-only cases; full finance, other language forms, compiler/proof/native/ledger/authority/general acceptance remain open.'})
write('CANONICAL-HASH-INDEX.json', {'schema': 'moriarty-independent-public-financial-canonical-identities/1',
    'conditions': conditions, 'program_hashes': program_hashes, 'plan_hashes': plan_hashes,
    'identities': hash_records,
    'no_product_frame_hash_invented': True,
    'diagnostic_hashes': 'Continuation, report, Snapshot, event and PendingFrame SHA256 values identify derived bytes only; none adds a product wire field, security judgment or authenticated history claim.'})
write('UNKNOWNS-AND-BOUNDARIES.json', {'schema': 'moriarty-independent-public-financial-boundaries/1',
    'conditions': conditions,
    'unfilled_fields_within_conditional_five_report_objects': [],
    'open_normative_premises': ['REG3/Core3 extension and D1/D2/D3 adoption/authority are not established by this arithmetic author.',
        'Exact /3 arena/report/work selection must be explicit; Core2 normative clauses alone do not adopt it.'],
    'wire_unknowns_outside_five_normal_simulate_reports': ['Nonquiet optional manifest raw stderr spacing/color/terminal settings remain unpredicted; semantic status Tested and message 3 passed; 0 failed are format-only conditional metadata.'],
    'source_custody_limit': 'The original independent SOURCE-INPUTS explicitly says these literal .mori files reconstruct the selected parent inputs and do not claim original security-author source-byte identity. Exact bytes of these selected source files are frozen unchanged.',
    'no_product_frame_hash': 'Only actual public program_hash/plan_hash fields are predicted; all other hashes are labelled diagnostic byte identities.',
    'no_new_admission_security_policy': True, 'no_test_run_or_acceptance_claim': True,
    'all_parent_financial_compiler_proof_target_obligations_retained': True})

# Provenance pins require reading/hash only the specifically permitted files.
# No prior EXPECTED, derive.py, goldens, candidate or actual results are opened.
baseline_scopes = [
    ('crates/mori-core/src/ir.rs', 'Current read lines68..90,134..183,239..270 for Expr/Statement/Effect serde shape; earlier authorized format spans33..44,92..133,184..237,272..290 for DTOs.'),
    ('crates/mori-core/src/stage/mod.rs', 'Prior current-context format exposure: constants/name metadata and DTO lines113..201; load/load_v3 bodies not opened.'),
    ('crates/mori-core/src/stage/plan.rs', 'Prior current-context format exposure lines1..59, types only; no build body.'),
    ('crates/mori-core/src/stage/report.rs', 'Prior current-context whole format-only output mapping file.'),
    ('crates/mori-core/src/stage/digest.rs', 'Prior current-context whole ordinary metadata digest/preimage-format file.'),
    ('crates/mori-core/src/intent.rs', 'Prior current-context lines69..91 hash_bytes/canonical/hex only; no authentication/verification logic.'),
]
normative_pins = []
stage_path = CHECKOUT / 'docs/CORE-STAGED-TESTS.md'
for first, last in ((163, 213), (237, 294)):
    selected = ''.join(stage_path.read_text().splitlines(keepends=True)[first - 1:last])
    normative_pins.append({'path': str(stage_path), 'first_line': first, 'last_line': last,
        'selected_utf8_sha256': sha_bytes(selected.encode()), 'selected_normative_text': selected})
for name in ('COMPLETE-ZERO-AWAIT-2.4.md', 'COMPLETE-CAPS-2.5.md', 'COMPLETE-FINAL-A5.md', 'COMPLETE-PROPOSED-D1-D1A.txt'):
    normative_pins.append({'path': str(REQ / name), 'sha256': sha_file(REQ / name),
        'selected_scope': 'Named normative clause only; /3 D1A previously read in this context and retains PROPOSED-UNADOPTED status.'})
write('NORMATIVE-PINS.json', {'schema': 'moriarty-independent-financial-normative-pins/1', 'clauses': normative_pins,
    'public_contract': {'path': str(CONTRACT), 'sha256': sha_file(CONTRACT), 'status': 'DRAFT, declared premises; prior authorized context read only'}})
provenance = {
    'schema': 'moriarty-independent-complete-public-financial-provenance/1',
    'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'requested_identity': 'gpt-6.1-sol high; inherited agent routing, no separate returned provider identity receipt',
    'independence_limit': 'Source-exposed mathematical rederivation with prior own arithmetic memory, not fresh independence from the old oracle and not retroactive preimplementation independence.',
    'prior_arithmetic_exposure': 'The earlier task derived these financial values in this same persistent agent context. They remain in memory. This new script rederives from original source/input literals and explicit normative equations without opening prior EXPECTED/derive/golden files.',
    'chronology': 'New complete whole objects, canonical preimages and bytes frozen before any NEW whole-wire standalone financial consumer observation in this task. Parent will execute and compare separately after freeze.',
    'original_input_pins': [{'path': str(ORIGINAL / name), 'bytes': (ORIGINAL / name).stat().st_size, 'sha256': sha_file(ORIGINAL / name)}
        for name in ('P1-RECONSTRUCTED-INPUT.mori', 'P2-RECONSTRUCTED-INPUT.mori', 'SOURCE-INPUTS.json')],
    'baseline': {'declared_C214b_by_parent': '214b377daf372f7df1506c49d3e15148f9931012', 'git_not_invoked': True,
        'format_only_files': [{'path': str(BASE / name), 'sha256': sha_file(BASE / name), 'actual_exposure_scope': scope} for name, scope in baseline_scopes]},
    'additional_prior_context_format_exposure': ['Baseline check_source.rs core_report, check.rs caller/stream envelope; test.rs aggregate_report; stage_test.rs lines43..140 format including visible product callsites but no executing outcomes; json.rs/profile.rs symbol metadata only. These are disclosed prior empty-format exposure, not new financial oracles.'],
    'current_task_disallowed_reads_not_performed': {'prior_EXPECTED_FINANCIAL_AND_WORK': True, 'prior_derive_scripts_or_goldens': True,
        'new_combined_or_publicconsumer_candidate_code_tests': True, 'runtime_actuals_or_product_outputs': True,
        'runtime_admit_validate_cost_parser_algorithms': True},
    'execution': {'standalone_stdlib_arithmetic_only_for_outcomes': True, 'product_evaluator_compiler_sdk_native_proof_provider_network_ledger_execution': False,
        'mandatory_plugin_status': 'Read-only refreshed; historical SP01.6 blocked/missing evidence, pendingTransactions []; no dispatch.', 'git_or_shared_writes': False},
    'graph_first': {'skill_read': '/home/charl/.agents/skills/graphify/SKILL.md',
        'metadata_check': str(CHECKOUT / 'graphify-out/graph.json'), 'metadata_check_result': 'not present (test -f exit1)',
        'restricted_graph_decision': 'No broad graph query/extraction: could expose forbidden implementation/results; input filename discovery only. Parent expressly accepted this blind-preserving scope.'},
    'startup': {'develop_skill_reread': '/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.1/skills/develop/SKILL.md',
        'AGENTS_product_contract': 'Same active checkout instructions loaded in this context; no new production code read.'},
    'event_format_completion': 'Original SOURCE-INPUTS event declarations give action/label/values. Raw input files add the normative StageEvent.format literal moriarty-stage-event/1; original source declaration is preserved separately.',
    'security_target_scope': 'No source-consent/private/target/hash-soundness or new admission judgment; no test/acceptance/proof/ledger claim.',
    'original_partial_oracle_preserved': True,
    'derivation_script_sha256': sha_file(HERE / 'derive.py'),
}
write('PROVENANCE.json', provenance)
(HERE / 'README.md').write_text('''# Complete conditional public Core3 financial oracle

Two exact source files and their original source/input declarations were read
from the immutable first independent packet. Its expected results, derivation,
goldens and all new candidate code/tests/actual output were not read. Prior own
arithmetic and format exposure remains disclosed: this is a mathematical
rederivation before NEW whole-wire observations, not a fresh or retroactive
preimplementation independence claim. The original partial packet is unchanged.

Five complete public report objects and exact conditional ASCII stdout bytes
cover P1 loan5 start, P1 loan3 start, and P2 loan5 start/false-go/true-go.
P1 closes loan2 or present-zero loan0 with one Liability reduction effect and
work2. P2 waits at loan5, retains it on false, then closes loan2. Attempted
work1/2/4 contrasts with committed1/1/5; total attempted7 is diagnostic history,
not a product work field. No discharge moves cash or charges an explicit fee.

Programs, plans, numeric statement ExprIds, continuation, event wire inputs,
all statement snapshots/deltas/prefixes/residuals and complete financial frames
are independently assembled from literals and named normative rules. Program
and plan product hash fields have exact compact canonical preimages. Other
SHA256 records identify derived report/continuation/Snapshot/PendingFrame bytes
only; no product frame_hash is invented. The false continuation is unchanged.

REG3 and public D1/D2/D3 are explicitly conditional/unadopted premises. Core2
normatives do not automatically adopt Core3. In particular, complete numeric
arena/report/work preservation for these finite sources must be selected
explicitly. This packet supplies no source/admission/security adoption or test
pass. It is complete within those declared premises, with no report-field
placeholder or projection substituted for a whole report.

Use COMPLETE-OPERATION-INDEX-AND-PREFIXES.json for paths, financial/work prefixes
and byte digests; CANONICAL-HASH-INDEX.json for exact preimage identities;
NORMATIVE-PINS.json and PROVENANCE.json for exposed format sources and chronology.
UNKNOWNS-AND-BOUNDARIES.json preserves adoption and broader financial/native gaps.
Optional moriarty-tests/3 manifest/aggregate bytes retain D3 conditionality;
nonquiet aggregate stderr formatting remains unknown. A real harness must chain
its actual continuation, never this expectation, between events.

Only standalone Python stdlib arithmetic/JSON/hash formatting ran. No evaluator,
compiler, provider, network, SDK, native, proof or ledger execution occurred.
Root integrates and compares separately after freeze. Full finance/general
language/compiler/proof/target/authority parent obligations remain open.
FREEZE-MANIFEST.json hashes every artifact except itself. Preserve this packet
before reproduction because rerunning derive.py changes its provenance time.
''')
artifacts = sorted(path for path in HERE.rglob('*') if path.is_file() and path.name != 'FREEZE-MANIFEST.json')
write('FREEZE-MANIFEST.json', {'schema': 'moriarty-independent-complete-public-financial-freeze/1',
    'frozen_at_utc': provenance['frozen_at_utc'], 'before_new_whole_wire_consumer_observation_in_author_task': True,
    'source_exposed_mathematical_rederivation_not_retroactive_independence': True,
    'conditional_scope_preserved': True,
    'artifacts': [{'path': str(path.relative_to(HERE)), 'bytes': path.stat().st_size, 'sha256': sha_file(path)} for path in artifacts],
    'manifest_self_hash_excluded': True})
print(json.dumps({'operations': len(reports), 'program_hashes': program_hashes, 'plan_hashes': plan_hashes,
    'report_stdout_hashes': {record['name']: record['stdout_sha256'] for record in operation_records},
    'freeze_manifest_sha256': sha_file(HERE / 'FREEZE-MANIFEST.json')}, sort_keys=True))
