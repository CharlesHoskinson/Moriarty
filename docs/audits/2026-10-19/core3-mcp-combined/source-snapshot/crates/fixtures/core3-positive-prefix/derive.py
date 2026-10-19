#!/usr/bin/env python3
"""Independent finite duty arithmetic and proposed-Core3 staged predictions.

Uses only Python stdlib. No product imports, parse/load/evaluator calls, model,
provider, compiler, SDK, cryptographic-key, native, or ledger execution.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CHECKOUT = Path('/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018')
REQ = ROOT / 'core3-runtime-admission-muse-source-owner-preparation-20261019/packet/requirements'


def clone(value):
    return json.loads(json.dumps(value))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(name, value):
    (HERE / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def uint(value):
    result = int(value)
    assert 0 <= result <= 2**128 - 1
    return result


asset = {'network': 'local', 'issuer': 'i', 'reference': 'A', 'scale': 0}
declaration = {'name': 'loan', 'asset': 'A', 'debtor': 'alice', 'creditor': 'bob'}
source_base = '''language "moriarty/3";
agreement P {
  asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
  duty "loan" = { asset: A, debtor: "alice", creditor: "bob" };
  action pay() {
BODY
  }
}
'''
p1_source = source_base.replace('BODY', '    discharge("loan", 3);')
p2_source = source_base.replace('BODY', '    await ready(go: Bool) when go;\n    discharge("loan", 3);')
(HERE / 'P1-RECONSTRUCTED-INPUT.mori').write_text(p1_source)
(HERE / 'P2-RECONSTRUCTED-INPUT.mori').write_text(p2_source)


def frame(amount):
    return {'state': {}, 'balances': [], 'liabilities': [
        {'name': 'loan', 'asset': clone(asset), 'debtor': 'alice', 'creditor': 'bob', 'amount': str(amount)}]}


def scenario(amount):
    result = frame(amount)
    result['inputs'] = {}
    return result


def residual(snapshot):
    return [clone(row) for row in snapshot['liabilities'] if uint(row['amount']) > 0]


def reduction(before_amount):
    remaining = uint(before_amount) - 3
    assert 0 <= remaining <= 2**128 - 1
    return remaining, {'kind': 'Liability', 'name': 'loan', 'asset': clone(asset),
                       'debtor': 'alice', 'creditor': 'bob', 'before': str(before_amount), 'after': str(remaining)}


idle = {'site': 0, 'captures': []}
pending = {'site': 1, 'captures': []}
qualifications = {'qualification': 'local-reference-stage', 'authentication': 'none', 'consumption': 'untracked'}


def financial_summary(before_amount, after_amount, effects):
    return {
        'exact_declared_assets': [{'name': 'A', 'identity': clone(asset)}],
        'source_duty_declaration': clone(declaration),
        'opening_frame': frame(before_amount), 'closing_frame': frame(after_amount),
        'balance_row_count_before': 0, 'balance_row_count_after': 0,
        'state_keys_before': [], 'state_keys_after': [],
        'loan_presence_before': True, 'loan_presence_after': True,
        'other_liabilities': 'absent before and after; no other rows introduced',
        'alice_and_bob_balance_rows': 'absent before and after; duty parties do not create balance rows',
        'liability_creation_amount': '0', 'liability_discharge_amount': str(before_amount - after_amount),
        'remaining_loan_amount': str(after_amount),
        'positive_residual_before': residual(frame(before_amount)),
        'positive_residual_after': residual(frame(after_amount)),
        'ordered_economic_effects': clone(effects),
        'cash_gross_debit': '0', 'cash_gross_credit': '0', 'explicit_fee': '0',
        'signed_net_cash_flow': '0', 'cash_net_receipts': '0',
        'alice_cash_net': '0', 'bob_cash_net': '0',
        'alice_liability_change': str(after_amount - before_amount),
        'bob_receivable_change': str(after_amount - before_amount),
        'token_supply_change': '0',
        'native_dust_or_service_fees': 'unmeasured and outside this local arithmetic; not asserted zero',
        'payment_implied_by_discharge': False,
    }


def discharge_step(index, before_amount):
    after_amount, effect = reduction(before_amount)
    return {
        'action_statement_ordinal': index,
        'source_statement': 'discharge("loan", 3);',
        'exact_ir_statement_and_amount_exprid': None,
        'typed_result': {'type': 'UInt', 'value': '3'},
        'pre_statement_frame': frame(before_amount), 'complete_post_statement_snapshot': frame(after_amount),
        'new_effect_delta': [effect], 'post_statement_effect_prefix': [effect],
        'work': {'statement_dispatch': '1', 'amount_literal_node': '1', 'total': '2'},
        'representation_status': 'semantic observation only; exact /3 ExprIds and raw wire layout not asserted',
    }


def p1_case(before_amount):
    after_amount, effect = reduction(before_amount)
    step = discharge_step(0, before_amount)
    return {
        'id': 'P1-initial-' + str(before_amount), 'source_input': 'P1-RECONSTRUCTED-INPUT.mori',
        'source_profile': 'moriarty/3', 'proposed_program_semantics': 'moriarty-core/3', 'action': 'pay',
        'initial_scenario': scenario(before_amount), 'start_logical_cap': None,
        'financial_summary': financial_summary(before_amount, after_amount, [effect]),
        'plan_observation_under_preserved_staged_schedule': {
            'statement_count': 1, 'await_sites': [], 'segment_cost': ['2'],
            'committed_before': ['0'], 'plan_total': '2'},
        'operation_history': [{
            'operation_ordinal': 0, 'operation': 'start', 'event': None,
            'semantic_outcome': 'complete', 'advanced_member': 'absent', 'continuation_member': 'absent',
            'qualification_projection_under_preserved_stage_contract': clone(qualifications),
            'frame_pre': clone(idle), 'frame_post': clone(idle),
            'segment_exists': True,
            'segment_pre': frame(before_amount), 'segment_post': frame(after_amount),
            'step_observations': [step], 'segment_effects': [effect],
            'history_steps_after': [step], 'history_effects_before': [], 'history_effects_after': [effect],
            'history_financial_prefixes': [frame(before_amount), frame(after_amount)],
            'positive_residual': residual(frame(after_amount)),
            'logical': {'profile_under_preservation_premise': 'moriarty-reference-logical-cost/2',
                        'stored_cap': None, 'attempted': '2', 'committed_before': '0',
                        'committed_after': '2', 'cumulative_attempted': '2'},
        }],
        'expectation_status': 'finite arithmetic plus preserved-Core2 functional/work premise for the proposed /3 source',
    }


after, discharge_effect = reduction(5)
accepted_await_step = {
    'action_statement_ordinal': 0,
    'source_statement': 'await ready(go: Bool) when go;',
    'exact_ir_statement_and_guard_exprid': None,
    'typed_result': {'type': 'Bool', 'value': True},
    'pre_statement_frame': frame(5), 'complete_post_statement_snapshot': frame(5),
    'new_effect_delta': [], 'post_statement_effect_prefix': [],
    'accepted_event': {'action': 'pay', 'label': 'ready', 'values': [{'name': 'go', 'value': {'type': 'Bool', 'value': True}}]},
    'event_binding_is_state': False,
    'work': {'resume_dispatch': '1', 'guard_name_node': '1', 'total': '2'},
    'representation_status': 'semantic observation only; exact /3 ExprIds and raw wire layout not asserted',
}
last_step = discharge_step(1, 5)
event_false = {'action': 'pay', 'label': 'ready', 'values': [{'name': 'go', 'value': {'type': 'Bool', 'value': False}}]}
event_true = clone(accepted_await_step['accepted_event'])
p2_history = [
    {
        'operation_ordinal': 0, 'operation': 'start', 'event': None,
        'semantic_outcome': 'waiting', 'advanced': True,
        'frame_pre': clone(idle), 'frame_post': clone(pending),
        'segment_exists': True, 'segment_pre': frame(5), 'segment_post': frame(5),
        'step_observations': [], 'segment_effects': [],
        'await_entry': {'source_ordinal': 0, 'entry_charge': '1', 'guard_evaluated': False, 'public_step_emitted': False},
        'continuation_semantic_projection': {'action': 'pay', 'site': 1, 'captures': [], 'stored_cap': None, 'committed': '1', 'boundary': frame(5)},
        'history_steps_after': [], 'history_effects_before': [], 'history_effects_after': [],
        'positive_residual': residual(frame(5)),
        'logical': {'profile_under_preservation_premise': 'moriarty-reference-logical-cost/2', 'stored_cap': None,
                    'attempted': '1', 'committed_before': '0', 'committed_after': '1', 'cumulative_attempted': '1'},
    },
    {
        'operation_ordinal': 1, 'operation': 'resume', 'event': event_false,
        'semantic_outcome': 'waiting', 'advanced': False,
        'frame_pre': clone(pending), 'frame_post': clone(pending),
        'segment_exists': False, 'public_segment': None,
        'retained_boundary_before': frame(5), 'retained_boundary_after': frame(5),
        'step_observations': [], 'public_effects_emitted': False,
        'guard_semantic_result': False, 'guard_public_step_emitted': False,
        'continuation_unchanged_from_previous_operation': True,
        'continuation_semantic_projection': {'action': 'pay', 'site': 1, 'captures': [], 'stored_cap': None, 'committed': '1', 'boundary': frame(5)},
        'history_steps_after': [], 'history_effects_before': [], 'history_effects_after': [],
        'positive_residual': residual(frame(5)),
        'logical': {'profile_under_preservation_premise': 'moriarty-reference-logical-cost/2', 'stored_cap': None,
                    'resume_dispatch': '1', 'guard_name_node': '1', 'attempted': '2',
                    'committed_before': '1', 'committed_after': '1', 'cumulative_attempted': '3'},
    },
    {
        'operation_ordinal': 2, 'operation': 'resume', 'event': event_true,
        'semantic_outcome': 'complete', 'advanced_member': 'absent', 'continuation_member': 'absent',
        'frame_pre': clone(pending), 'frame_post': clone(idle),
        'segment_exists': True, 'segment_pre': frame(5), 'segment_post': frame(after),
        'step_observations': [accepted_await_step, last_step], 'segment_effects': [discharge_effect],
        'history_steps_after': [accepted_await_step, last_step],
        'history_effects_before': [], 'history_effects_after': [discharge_effect],
        'positive_residual': residual(frame(after)),
        'logical': {'profile_under_preservation_premise': 'moriarty-reference-logical-cost/2', 'stored_cap': None,
                    'resume_dispatch': '1', 'guard_name_node': '1', 'discharge_dispatch': '1', 'literal_node': '1',
                    'attempted': '4', 'committed_before': '1', 'committed_after': '5', 'cumulative_attempted': '7'},
    },
]
for operation in p2_history:
    operation['qualification_projection_under_preserved_stage_contract'] = clone(qualifications)
    operation['history_financial_prefixes'] = [frame(5), frame(5), frame(5), frame(after)][:operation['operation_ordinal'] + 2]
    operation['financial_summary_for_operation'] = financial_summary(5, after if operation['operation_ordinal'] == 2 else 5,
                                                                    [discharge_effect] if operation['operation_ordinal'] == 2 else [])
p2 = {
    'id': 'P2-initial-5-false-then-true', 'source_input': 'P2-RECONSTRUCTED-INPUT.mori',
    'source_profile': 'moriarty/3', 'proposed_program_semantics': 'moriarty-core/3', 'action': 'pay',
    'initial_scenario': scenario(5), 'start_logical_cap': None, 'resume_logical_caps': [None, None],
    'event_source_assumption': 'source-owner selected two ordinary well-typed event tuples, false then true, naming action pay and label ready; exact wire bytes were not supplied',
    'financial_summary': financial_summary(5, after, [discharge_effect]),
    'plan_observation_under_preserved_staged_schedule': {
        'statement_count': 2,
        'await_sites': [{'site': 1, 'ordinal': 0, 'label': 'ready', 'binders': [{'name': 'go', 'ty': 'Bool'}], 'captures': []}],
        'segment_cost': ['1', '4'], 'committed_before': ['0', '1'], 'plan_total': '5',
        'equations': ['segment0 = entry(await) = 1', 'segment1 = resume_dispatch1 + guard_name1 + discharge_dispatch1 + literal1 = 4',
                      'plan_total = 1 + 4 = 5', 'all_observed_attempts = start1 + false2 + true4 = 7',
                      'successful_committed = start1 + true4 = 5; false2 is attempted only']},
    'operation_history': p2_history,
    'cross_operation_step_ordinals': [0, 1],
    'retained_false_event_binding': False,
    'expectation_status': 'finite arithmetic plus preserved-Core2 functional/work premise for the proposed /3 source',
}
cases = [p1_case(5), p1_case(3), p2]
assert uint(cases[0]['financial_summary']['remaining_loan_amount']) == 2
assert cases[1]['financial_summary']['loan_presence_after'] is True
assert cases[1]['financial_summary']['positive_residual_after'] == []
assert cases[1]['financial_summary']['closing_frame']['liabilities'][0]['amount'] == '0'
assert [uint(operation['logical']['attempted']) for operation in p2_history] == [1, 2, 4]
assert sum(uint(operation['logical']['attempted']) for operation in p2_history) == 7
assert [uint(operation['logical']['committed_after']) for operation in p2_history] == [1, 1, 5]

unknowns = [
    {'id': 'C3-PRESERVATION', 'unknown': 'The supplied COMPLETE-PROPOSED-D1-D1A.txt explicitly labels the /3 extension PROPOSED-UNADOPTED. Core/2 adopted observation/cost clauses explicitly scope their exact wire strings to /2.',
     'treatment': 'Arithmetic and source cases are frozen independently. Core/3 functional/staged/work outcomes are conditional on the intended extension preserving these Core/2 financial/stage rules, including cost::fill; no /3 adoption or execution is asserted.'},
    {'id': 'EXPRIDS', 'unknown': 'No exact /3 source-to-expression-arena adoption was supplied; no parser result was inspected.',
     'treatment': 'Amount/guard raw ExprIds and complete raw Program/Statement fields remain null/unpredicted. One literal-node/one guard-name-node are explicit semantic work premises, not concrete IDs.'},
    {'id': 'WIRE', 'unknown': 'Original source bytes, event wire text, program/plan hashes, concrete continuations, consumer/executable and output wire strings were not supplied.',
     'treatment': 'Source files here are explicitly reconstructed from the parent-selected semantic input. No raw Program, Execution, continuation, public stdout or aggregate-byte golden is invented. Preserved-Core2 envelope observations are conditional semantic projections only.'},
    {'id': 'IDENTITY-SECURITY', 'unknown': 'Source identity, consent/authority, private/target rules and their security acceptance are outside this assignment.',
     'treatment': 'No such judgment, rejection policy, hostile test, negative-admission assertion or general acceptance claim is authored.'},
    {'id': 'EXTERNAL-COST', 'unknown': 'DUST, service fees, proving/native/ledger costs, wall time and memory are not defined by the supplied logical schedule.',
     'treatment': 'Explicit source fee and cash movement are zero; unmeasured external costs are not asserted zero.'},
]
inputs = {'schema': 'moriarty-independent-source-owner-inputs/1',
          'origin': 'Parent task message; parent says designated security-owner selected existing runtime cases; no case expected result or assertion read.',
          'reconstructed_source_bytes_are_original': False,
          'agreement': 'P', 'language': 'moriarty/3', 'asset': {'name': 'A', 'identity': asset},
          'source_duty': declaration,
          'programs': [{'id': 'P1', 'action': 'pay', 'statements': ['discharge("loan", 3);'], 'initial_scenarios': [scenario(5), scenario(3)]},
                       {'id': 'P2', 'action': 'pay', 'statements': ['await ready(go: Bool) when go;', 'discharge("loan", 3);'],
                        'initial_scenario': scenario(5), 'events': [event_false, event_true]}],
          'cap': None}
write('SOURCE-INPUTS.json', inputs)
write('EXPECTED-FINANCIAL-AND-WORK.json', {'schema': 'moriarty-independent-positive-core3-finance-work/1',
      'case_count': 3, 'source_program_count': 2,
      'purpose': 'Bounded normal functional predictions only; no product test or acceptance assertion.',
      'cases': cases, 'unknowns_and_conditional_premises': unknowns})

# Only explicitly normative selected sections are semantically inspected; hashing
# whole files records freshness without using prior example outputs as oracles.
normative_specs = [
    (CHECKOUT / 'docs/CORE-STAGED-TESTS.md', 163, 213),
    (CHECKOUT / 'docs/CORE-STAGED-TESTS.md', 237, 299),
    (CHECKOUT / 'docs/CORE-EXECUTION-REPORTS.md', 93, 127),
]
normative_pins = []
for path, first, last in normative_specs:
    lines = path.read_text().splitlines(keepends=True)
    selected = ''.join(lines[first - 1:last])
    normative_pins.append({'path': str(path), 'whole_file_sha256': sha(path), 'selected_first_line': first,
                           'selected_last_line': last, 'selected_utf8_sha256': hashlib.sha256(selected.encode()).hexdigest(),
                           'selected_normative_text': selected})
for name in ('COMPLETE-FINAL-A3.md', 'COMPLETE-FINAL-A4.md', 'COMPLETE-FINAL-A5.md',
             'COMPLETE-ZERO-AWAIT-2.4.md', 'COMPLETE-CAPS-2.5.md', 'COMPLETE-RESUME-PREDICATES-R1-R7.md',
             'COMPLETE-PROPOSED-D1-D1A.txt'):
    path = REQ / name
    normative_pins.append({'path': str(path), 'whole_file_sha256': sha(path), 'normative_text': path.read_text()})
write('NORMATIVE-PINS-AND-CLAUSES.json', {'schema': 'moriarty-independent-normative-pins/1', 'pins': normative_pins})

provenance = {
    'schema': 'moriarty-independent-core3-positive-oracle-provenance/1',
    'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'task': '/root/guarded_normal_independent_finance_sol',
    'requested_model': 'gpt-6.1-sol high',
    'actual_model_receipt': 'Parent-inherited agent model; no separate provider invocation or returned model identity receipt in this task.',
    'independence': {
        'parent_selected_source_owner_cases': True,
        'parent_implementation_exposure': 'Intentional source selection disclosed by parent; parent compares separately.',
        'prior_core3_production_source_exposure': False,
        'prior_new_core3_test_results_assertions_or_model_opinions_received': False,
        'new_production_source_or_test_files_or_source_array_opened': False,
        'runtime_admit_validate_cost_parser_ir_implementations_read': False,
        'prior_context_reuse': 'Administrative workflow plus prior frozen Core/1 public reference semantics and own Core/1 arithmetic; no prior case output or numbers reused as an oracle.',
        'current_normative_freshness': 'Normative named requirements and adopted staged/report/M01 sections reread in current task; exact pins attached.',
        'normative_author_exposure': 'Adopted Core/2 docs disclose implementation-source-exposed design; no implementation code was used by this independent arithmetic author.',
        'requirement_corrections_receipt': 'Read COMPLETE-REVIEW-CORRECTIONS-1-3-2-5.json because it was in the allowed requirements directory; its source/grammar and compatibility corrections contained no selected-case expected results. It is not used as an arithmetic oracle or as approval.',
        'only_arithmetic_program_executed': 'derive.py; read-only mandatory plugin status executed separately',
        'product_execution': False, 'web_or_provider_execution': False,
        'sdk_native_ledger_crypto_key_wallet_execution': False, 'git_or_shared_writes': False,
        'financial_test_or_acceptance_claim': False,
    },
    'administrative_startup': {
        'skill_path': '/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.1/skills/develop/SKILL.md',
        'agents_path': str(CHECKOUT / 'AGENTS.md'),
        'product_contract': 'Read in prior context for same checkout; no financial case expectations from it.',
        'status_command': 'python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json',
        'status_cwd': str(CHECKOUT),
        'status_observation': 'Historical SP01.6 missing binding/candidate/current-accounting/resource evidence; pendingTransactions []; no dependent dispatch.'},
    'additional_requirement_filenames_read': ['COMPLETE-COMPATIBILITY-SECTION-1.md', 'COMPLETE-UNI-018.md', 'COMPLETE-UNI-020.md', 'COMPLETE-UNI-021.md', 'COMPLETE-CSQ-016.md', 'COMPLETE-PROPOSED-4.3-REGISTRY.txt', 'COMPLETE-ORIGINAL-OD1-ROW.md', 'COMPLETE-REVIEW-CORRECTIONS-1-3-2-5.json'],
    'derivation_script_sha256': sha(HERE / 'derive.py'),
    'unknown_ids': [item['id'] for item in unknowns],
    'future_comparison': 'Integrator must compare separately after this freeze; actual financial consumer results were not available to the author.',
}
write('PROVENANCE.json', provenance)
(HERE / 'README.md').write_text('''# Independent positive Core3 financial/work freeze

Two parent-selected source programs; three bounded cases including P1's full
discharge variant. The sources here reconstruct the parent message and do not
claim original source-byte identity. No product implementation or selected-case
test assertions/results were used. Only standalone stdlib arithmetic ran.

P1 discharges 3 from an existing loan5, leaving present loan2; the separate loan3
variant leaves present loan0. Both have one Liability effect, no balance rows,
no cash movement and no explicit fee. The zero row remains in the snapshot but
is absent from the positive residual. Expected local work is 2.

P2 starts waiting with loan5; a false go event retains that boundary and emits no
segment; a true go event has Await Bool(true), then Discharge UInt3, leaving loan2.
Expected attempted work is 1,2,4, with committed successful-prefix work 1,1,5.
The plan costs are [1,4], committed_before [0,1], plan_total5. Total attempted is7.
False-guard attempted work2 does not enter committed work. Cap stays absent/null.

These staged and work predictions explicitly assume that the proposed Core3
extension preserves the normative Core2 financial/stage schedule. The provided
/3 clause is labeled PROPOSED-UNADOPTED. No Core3 adoption, source admission,
security identity, actual test, proof, native result or general acceptance is
claimed. Concrete ExprIds, raw wire bytes, program/plan identities and consumer
results remain unpredicted. External DUST and service costs remain unmeasured.

EXPECTED-FINANCIAL-AND-WORK.json contains complete case/operation/statement
frames, typed values, effect deltas, prefixes, residuals and arithmetic schedules.
NORMATIVE-PINS-AND-CLAUSES.json retains exact selected public normative sections.
PROVENANCE.json discloses prior context, administrative reads and independence.
FREEZE-MANIFEST.json hashes all packet artifacts except itself. Preserve this
frozen packet before any reproduction; rerunning derive.py changes its timestamp.
The parent must compare actual results separately without changing this oracle.
''')
artifacts = ['derive.py', 'P1-RECONSTRUCTED-INPUT.mori', 'P2-RECONSTRUCTED-INPUT.mori',
             'SOURCE-INPUTS.json', 'EXPECTED-FINANCIAL-AND-WORK.json',
             'NORMATIVE-PINS-AND-CLAUSES.json', 'PROVENANCE.json', 'README.md']
write('FREEZE-MANIFEST.json', {'schema': 'moriarty-independent-positive-core3-freeze/1',
      'frozen_at_utc': provenance['frozen_at_utc'], 'before_any_product_execution_in_author_task': True,
      'artifacts': [{'path': name, 'bytes': (HERE / name).stat().st_size, 'sha256': sha(HERE / name)} for name in artifacts],
      'manifest_self_hash_excluded': True})
print(json.dumps({'case_ids': [case['id'] for case in cases],
                  'closing_loan_amounts': [case['financial_summary']['remaining_loan_amount'] for case in cases],
                  'P2_attempted': [operation['logical']['attempted'] for operation in p2_history],
                  'P2_committed': [operation['logical']['committed_after'] for operation in p2_history],
                  'expectation_sha256': sha(HERE / 'EXPECTED-FINANCIAL-AND-WORK.json'),
                  'manifest_sha256': sha(HERE / 'FREEZE-MANIFEST.json')}, sort_keys=True))
