#!/usr/bin/env python3
"""Independent empty-case arithmetic and conditional public-format projection.

No Moriarty imports, subprocess calls, parser/evaluator/compiler/provider/native
execution. Schemas come from disclosed baseline format-only reads; semantic/work
predictions come from source and explicit public normative premises, not code.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BASE = Path('/home/charl/Moriarty/.worktrees/utxo-core3-validator-local-20261019')
WORKTREE = Path('/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018')
FIXTURES = ROOT / 'core3-public-consumer-source-preparation-20261019/AFTER/crates/fixtures/core3-public-consumers'
CONTRACT = ROOT / 'core3-public-consumer-contract-preparation-20261019/CONTRACT.md'


def clone(value):
    return json.loads(json.dumps(value))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def encode(value):
    # Chosen independently from the adopted ASCII pretty-rendering premise:
    # lexical keys, array order, two-space indentation, one final LF.
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True) + '\n').encode()


def write(name, value):
    (HERE / name).write_bytes(encode(value))


source_bytes = (FIXTURES / 'empty.mori').read_bytes()
scenario_bytes = (FIXTURES / 'empty-scenario.json').read_bytes()
assert source_bytes == b'language "moriarty/3";\nagreement Empty {\n  state flag: Bool = true;\n  action run() {}\n}\n'
scenario = json.loads(scenario_bytes)
assert scenario == {'state': {}, 'inputs': {}, 'balances': [], 'liabilities': []}
(HERE / 'empty.mori').write_bytes(source_bytes)
(HERE / 'empty-scenario.json').write_bytes(scenario_bytes)
(HERE / 'DECLARED-PUBLIC-CONTRACT.md').write_bytes(CONTRACT.read_bytes())

# Source-only finite deductions. The initializer is metadata, not an executed
# action expression. There are zero statements, hence zero dispatches/nodes.
typed_flag = {'type': 'Bool', 'value': True}
opening = {'state': {'flag': typed_flag}, 'balances': [], 'liabilities': []}
closing = clone(opening)
statements = []
expressions = []
statement_charges = sum(1 for _ in statements)
expression_charges = sum(1 for _ in expressions)
work = statement_charges + expression_charges
assert work == 0
actions = [{'name': 'run', 'inputs': [], 'statements': []}]
states = [{'name': 'flag', 'ty': 'Bool', 'initial': typed_flag}]
program = {'language': 'moriarty/3', 'semantics': 'moriarty-core/3', 'name': 'Empty',
           'states': states, 'assets': [], 'expressions': [], 'actions': actions}
# Baseline Program serde omits empty duties. D2 inspect deliberately exposes it.
canonical_program = json.dumps(program, sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode()
program_hash = hashlib.sha256(canonical_program).hexdigest()
(HERE / 'CONDITIONAL-PROGRAM-CANONICAL-BYTES.json').write_bytes(canonical_program)
action_plan = {'action': 'run', 'statement_count': 0, 'sites': [],
               'segment_cost': ['0'], 'committed_before': ['0'], 'plan_total': '0'}
stage_plan = {'format': 'moriarty-stage-plan/1', 'program_hash': program_hash, 'actions': [action_plan]}
canonical_plan = json.dumps(stage_plan, sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode()
plan_hash = hashlib.sha256(canonical_plan).hexdigest()
(HERE / 'CONDITIONAL-PLAN-CANONICAL-BYTES.json').write_bytes(canonical_plan)
labels = {'qualification': 'local-reference-stage', 'authentication': 'none', 'consumption': 'untracked'}
idle = {'site': 0, 'captures': []}
execution = {'semantics': 'moriarty-core/3', 'action': 'run', 'pre': opening,
             'post': closing, 'steps': [], 'effects': []}
start_report = dict(labels, status='complete', attempted='0', committed='0', residual=[],
                    segment={'action': 'run', 'frame_pre': idle, 'frame_post': clone(idle), 'execution': execution})
write('CONDITIONAL-PROGRAM.json', program)
write('CONDITIONAL-START-REPORT.json', start_report)
(HERE / 'CONDITIONAL-START-STDOUT.txt').write_bytes(encode(start_report))

conditions = [
    'D1 source selection is adopted exactly as proposed and this supported /3 source reaches the real load_v3/start consumers.',
    'D2 is adopted with the exact /3 source/arena/report projection and preserved declared financial/stage rules; the proposal currently explicitly says DRAFT, not adopted.',
    'The public Core/2 empty-action, initializer normalization, segment DTO, zero-work and ASCII stage-rendering premises are explicitly selected for /3, rather than inferred as already adopted.',
]
write('NORMAL-SEMANTIC-PROJECTION.json', {
    'schema': 'moriarty-independent-empty-public-normal-projection/1',
    'source': {'language': 'moriarty/3', 'agreement': 'Empty', 'action': 'run'},
    'status': 'conditional public proposal prediction, not observed result or adopted acceptance',
    'conditions': conditions,
    'scenario_input': scenario, 'normalized_opening_frame': opening, 'closing_frame': closing,
    'state_initializer_rule': 'Empty scenario state selects all declared initial values; flag Bool true is preserved by the empty action.',
    'statement_count': 0, 'expression_arena_count': 0, 'steps': [], 'effects': [],
    'history': {'opening': opening, 'executed_statement_prefixes': [], 'closing': closing,
                'effect_prefix_before_and_after': [], 'no_synthetic_step': True},
    'financial': {'balance_rows_before': [], 'balance_rows_after': [],
                  'liability_rows_before': [], 'liability_rows_after': [],
                  'all_balance_and_liability_rows_absent_before_and_after': True,
                  'present_zero_liability_rows': [], 'positive_residual': [],
                  'source_explicit_fee': '0', 'gross_debit': '0', 'gross_credit': '0',
                  'signed_net_cash_flow': '0', 'liability_created': '0', 'liability_discharged': '0',
                  'native_dust_service_fee_proof_cost': 'unmeasured; not asserted zero'},
    'work_under_preserved_schedule': {'profile': 'moriarty-reference-logical-cost/2',
        'statement_dispatches': 0, 'expression_visits': 0, 'await_entry_charges': 0,
        'attempted': '0', 'committed': '0', 'segment_cost': ['0'], 'committed_before': ['0'], 'plan_total': '0',
        'source_cap': None, 'complete_report_has_no_continuation_or_cap_member': True},
    'core3_inheritance_is_not_currently_adopted': True,
    'full_financial_goldens': 'Missing separately. Empty action does not exercise transfers, fees, liabilities, discharge, zero-duty preservation, awaits, or general financial coverage.',
    'compiler_proof_target_acceptance': False,
})

check_known = dict(labels, status='accepted', language='moriarty/3', semantics='moriarty-core/3',
                   agreement='Empty', actions=actions, diagnostics=[])
check_report = dict(check_known, file='empty.mori', source_hash=hashlib.sha256(source_bytes).hexdigest())
write('CONDITIONAL-CHECK-REPORT.json', check_report)
write('CHECK-KNOWN-PROJECTION.json', {
    'schema': 'moriarty-independent-partial-check-projection/1', 'conditions': conditions,
    'complete_conditional_report_available': True, 'known_members': check_report,
    'selected_invocation': {'cwd': str(HERE), 'arguments': ['check', 'empty.mori', '--json']},
    'file_is_invoked_argument_not_absolute_checkout': True,
    'source_hash_rule': 'lowercase SHA256 of the exact raw authorized source bytes',
    'normal_json_check_process_under_baseline_format': {'exit': 0, 'stderr': ''},
    'missing_wire_rules': ['core_io::print exact stdout encoding was not opened; JSON artifact pretty bytes are not claimed as check stdout'],
})
inspect_known = dict(labels, status='accepted', language='moriarty/3', semantics='moriarty-core/3',
                     agreement='Empty', states=states, assets=[], duties=[], actions=actions,
                     expressions=[], enforcement_report=None,
                     stage_plan=stage_plan)
write('CONDITIONAL-INSPECT-REPORT.json', inspect_known)
write('INSPECT-KNOWN-PROJECTION.json', {
    'schema': 'moriarty-independent-partial-inspect-projection/1', 'conditions': conditions,
    'complete_conditional_report_available': True, 'known_members': inspect_known,
    'hash_format_observed_from_permitted_baseline': 'SHA256(intent::canonical(Program)), lowercase 64-character hex.',
    'canonical_rule_format_exposure': 'intent::canonical recursive lexical object keys, array order retained, compact serde JSON bytes; all selected strings are ASCII, and Python stdlib independently encodes this finite subset.',
    'program_hash': program_hash, 'plan_hash_diagnostic_not_public_inspect_member': plan_hash,
    'missing_wire_rules': ['inspect exact stdout renderer/process implementation was not opened; JSON artifact pretty bytes are not claimed as inspect stdout'],
})
write('CONDITIONAL-HASH-DERIVATION.json', {
    'schema': 'moriarty-independent-conditional-public-metadata-hashes/1',
    'conditions': conditions,
    'raw_source_sha256': hashlib.sha256(source_bytes).hexdigest(),
    'raw_source_bytes': len(source_bytes),
    'program_canonical_bytes': len(canonical_program), 'program_hash': program_hash,
    'plan_canonical_bytes': len(canonical_plan), 'plan_hash': plan_hash,
    'canonical_files_have_no_final_newline': True,
    'empty_duties_serialization': 'omitted in Program canonical value; mandatory [] in proposed public inspect object',
    'security_or_hash_soundness_judgment': False,
})

manifest = {'profile': 'moriarty-tests/3', 'cases': [{
    'name': 'empty-normal-independent', 'source': 'empty.mori', 'scenario': 'empty-scenario.json',
    'action': 'run', 'logical_cap': None, 'expected': 'CONDITIONAL-START-REPORT.json', 'events': []}]}
write('CONDITIONAL-TEST-MANIFEST.json', manifest)
aggregate = {'status': 'passed', 'cases': [{'name': 'empty-normal-independent', 'passed': True,
    'stages': [{'index': 0, 'passed': True, 'actual': start_report}]}]}
write('CONDITIONAL-MANIFEST-AGGREGATE.json', aggregate)
(HERE / 'CONDITIONAL-MANIFEST-STDOUT.txt').write_bytes(encode(aggregate))
write('MANIFEST-KNOWN-PROJECTION.json', {
    'schema': 'moriarty-independent-partial-test-manifest-projection/1',
    'conditions': conditions + ['D3 is adopted exactly, relative paths resolve from this manifest and the complete actual start report equals this independently derived conditional report.'],
    'complete_conditional_aggregate_available': True,
    'selected_manifest': manifest,
    'known_observations_on_equality': {'case_name': 'empty-normal-independent', 'passed': True,
        'number_of_cases': 1, 'number_of_stages': 1, 'stage_passed': True, 'stage_actual': start_report},
    'complete_conditional_aggregate': aggregate,
    'normal_process_under_preserved_format': {'exit': 0, 'shell_status_label': 'Tested', 'shell_status_message': '1 passed; 0 failed'},
    'missing_process_rules': ['exact Shell.status stderr bytes, spacing, color/terminal/quiet settings'],
    'no_actual_continuation_or_resume_expected': True,
    'manifest_whole_equality_acceptance': False,
})

unknowns = [
    {'id': 'U-ADOPTION', 'paths': ['all public predictions'], 'missing': 'D1/D2/D3 and explicit /3 arena/report/work preservation remain declared DRAFT premises; no authoritative adopted /3 ordinary contract supplied.',
     'minimum_resolution': 'Freeze exact adopted /3 source/Program/report/work and public consumer clauses, with actual votes/provenance; then compare this frozen conditional prediction without replacing it with candidate output.'},
    {'id': 'U-CHECK-INSPECT-WIRE', 'paths': ['check stdout', 'inspect stdout'], 'missing': 'Exact proposed /3 public rendering/process contract beyond object keys.', 'minimum_resolution': 'Specify independent key/escape/indent/final-LF and exit/stderr rules.'},
    {'id': 'U-MANIFEST-STDERR', 'paths': ['test stderr'], 'missing': 'Exact Shell.status stderr raw formatting and selected terminal/color/quiet settings.', 'minimum_resolution': 'Specify this normal process format independently; semantic status label/message and exit0 are already frozen conditionally.'},
    {'id': 'U-FINANCE-GENERALITY', 'paths': ['full financial goldens', 'general language/compiler/proof/target claims'], 'missing': 'Empty program exercises none of the financial or native obligations.', 'minimum_resolution': 'Separate independent complete financial/cross-layer cases and authorized real comparisons; this packet cannot supply those exits.'},
]
write('UNKNOWNS.json', {'schema': 'moriarty-independent-public-normal-unknowns/1', 'unknowns': unknowns})

baseline_exposure = [
    ('crates/mori/src/commands/check_source.rs', 'whole file read; core_report keys only used; baseline dispatch visible but not a semantic oracle'),
    ('crates/mori-core/src/stage/report.rs', 'whole output-format constructor read; no evaluator code'),
    ('crates/mori-core/src/stage/digest.rs', 'whole digest/preimage-format file read; SHA256 canonical reference only, no helper execution'),
    ('crates/mori-core/src/stage/plan.rs', 'lines 1..59, types only; build body not read'),
    ('crates/mori-core/src/stage/mod.rs', 'metadata/type/function-name search and lines 113..201 DTOs only; load/load_v3 bodies not read'),
    ('crates/mori-core/src/ir.rs', 'schema-name search and lines 33..44,92..133,184..237,272..290 only; no evaluator'),
    ('crates/mori-core/src/stage/profile.rs', 'names/metadata rg search only; no predicate bodies used'),
    ('crates/mori/src/json.rs', 'names/metadata rg search only; tests/goldens not opened'),
    ('crates/mori-core/src/intent.rs', 'function-name search and lines 69..91, hash_bytes/canonical/hex only; no signature/authorization/verification logic read'),
    ('crates/mori/src/commands/check.rs', 'whole file, caller file display and JSON stream/process format only; no parser/evaluator internals'),
    ('crates/mori/src/commands/test.rs', 'function/printing-name search and lines 107..138 aggregate formatter only; test module not read'),
    ('crates/mori/src/commands/test/stage_test.rs', 'function/printing-name search and lines 43..140 for existing aggregate literals/index/status format only; calls to load/start/resume visible but no outcome was taken from these calls or executed'),
]
provenance = {
    'schema': 'moriarty-independent-public-empty-oracle-provenance/1',
    'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'requested_identity': 'gpt-6.1-sol high; parent-inherited agent, no separate provider model receipt',
    'input_pins': [{'path': str(FIXTURES / name), 'bytes': (FIXTURES / name).stat().st_size, 'sha256': sha(FIXTURES / name)} for name in ('empty.mori', 'empty-scenario.json')],
    'proposed_contract': {'path': str(CONTRACT), 'sha256': sha(CONTRACT), 'status': 'DRAFT; declared premises, not adopted'},
    'baseline': {'declared_commit_by_parent': '214b377daf372f7df1506c49d3e15148f9931012',
        'checkout': str(BASE), 'git_not_invoked': True,
        'format_only_exposure': [{'path': str(BASE / name), 'sha256': sha(BASE / name), 'read_scope': scope} for name, scope in baseline_exposure]},
    'normative_reuse': {'core2_stage': str(WORKTREE / 'docs/CORE-STAGED-TESTS.md'),
        'core2_stage_sha256_at_freeze': sha(WORKTREE / 'docs/CORE-STAGED-TESTS.md'),
        'core1_reference': 'Prior authorized frozen public CORE-REFERENCE-CONTRACT.md for state{} initializer selection and finite statement interpretation.',
        'previous_own_core3_freeze': str(ROOT / 'core3-positive-finance-independent-freeze-20261019/FREEZE-MANIFEST.json'),
        'previous_own_core3_freeze_sha256': sha(ROOT / 'core3-positive-finance-independent-freeze-20261019/FREEZE-MANIFEST.json'),
        'reuse_scope': 'Normative type/report/stage and prior conditionality only; no previous financial numbers borrowed as new oracles.'},
    'independence': {'new_publicconsumer_candidate_code_tests_expected_files_opened': False,
        'only_two_authorized_candidate_input_files_read': True,
        'baseline_source_format_exposure_disclosed': True,
        'semantic_work_results_from_product_code': False, 'candidate_or_evaluator_generated_golden': False,
        'parent_selected_existing_fixture_exposure': 'Intentional input selection; parent has implementation exposure; author receives no new actual outputs or selected expected values.',
        'prior_exposure': 'Prior Core1 public normatives and own arithmetic; prior /3 named requirements and format-independent proposed-duty clauses; no new evaluator/admit/validate/cost implementation.',
        'product_runtime_compiler_sdk_native_provider_ledger_execution': False,
        'only_standalone_derivation_executed_for_outcomes': True,
        'mandatory_read_only_plugin_status': 'Refreshed; historical SP01.6 blocked/missing evidence, pendingTransactions []; no campaign or product run.',
        'git_or_shared_writes': False, 'web_research': False,
        'conditional_only_no_test_or_acceptance_claim': True},
    'wire_notes': {'conditional_start_stdout_encoding': 'Independent JSON lexical-key ASCII pretty rule from adopted Core2 normative clause, explicitly conditional for Core3 D2.',
        'conditional_start_stdout_bytes': len(encode(start_report)), 'conditional_start_stdout_sha256': sha(HERE / 'CONDITIONAL-START-STDOUT.txt'),
        'source_hash_field_resolved_only_after_permitted_baseline_hash_bytes_format_read': True,
        'conditional_manifest_stdout_bytes': len(encode(aggregate)), 'conditional_manifest_stdout_sha256': sha(HERE / 'CONDITIONAL-MANIFEST-STDOUT.txt')},
    'chronology': 'Authored and frozen before any consumer execution in this task; parent will compare later. No actual results used.',
    'derivation_script_sha256': sha(HERE / 'derive.py'),
}
write('PROVENANCE.json', provenance)
(HERE / 'README.md').write_text('''# Independent conditional empty Core3 public oracle

Only the two authorized fixture inputs were read from the new publicconsumer
candidate. Their bytes are preserved here. The proposed public CONTRACT is
copied and pinned as DRAFT, not adopted. Baseline C214b format-only exposure,
normative reuse, all hashes and limitations are recorded in PROVENANCE.json.

From source and declared rules, the empty action retains flag Bool true and all
balance/liability rows absent. Steps/effects/residuals are empty; attempted,
committed and total logical work are zero. The initializer allocates no action
expression. No synthetic Step or continuation exists. These are conditional
on the proposed Core3 ordinary semantics/report/work preservation being selected.

CONDITIONAL-START-REPORT.json is a complete independently assembled proposed
normal start report. CONDITIONAL-START-STDOUT.txt freezes its proposed ASCII
pretty bytes with one LF. This is a conditional artifact, not an adopted full
ordinary oracle or an observed successful run. CONDITIONAL-PROGRAM.json and
CONDITIONAL-TEST-MANIFEST.json are similarly explicit proposed projections.

Complete conditional check, inspect and manifest objects are frozen separately.
The explicit check invocation uses literal relative argument empty.mori from
this oracle directory; its file member therefore does not invent a checkout.
Program/plan hash preimages and lowercase SHA256 are independently computed
from the source-only Program using permitted baseline format definitions, never
by importing the producer. Complete manifest stdout bytes are independently
assembled. Exact check/inspect stdout and manifest raw stderr remain unknown.
Wrapper projections and UNKNOWNS.json retain these gaps and the adoption gap.

No evaluator/compiler/provider/SDK/native/ledger ran and no new candidate test or
expected-output file was opened. Only standalone stdlib derivation ran. Parent
must compare later and retain the conditional scope. This empty case does not
supply full financial goldens, general language/compiler/proof/target acceptance.
FREEZE-MANIFEST.json hashes this packet except itself. Preserve the original
before reproduction: rerunning derive.py changes provenance timestamp.
''')
names = sorted(path.name for path in HERE.iterdir() if path.is_file() and path.name != 'FREEZE-MANIFEST.json')
write('FREEZE-MANIFEST.json', {'schema': 'moriarty-independent-public-empty-freeze/1',
    'frozen_at_utc': provenance['frozen_at_utc'], 'before_consumer_execution_in_author_task': True,
    'conditional_scope_preserved': True, 'artifacts': [{'path': name, 'bytes': (HERE / name).stat().st_size, 'sha256': sha(HERE / name)} for name in names],
    'manifest_self_hash_excluded': True})
print(json.dumps({'source_sha256': sha(HERE / 'empty.mori'), 'scenario_sha256': sha(HERE / 'empty-scenario.json'),
    'contract_sha256': sha(CONTRACT), 'conditional_start_bytes': len(encode(start_report)),
    'conditional_start_sha256': sha(HERE / 'CONDITIONAL-START-STDOUT.txt'),
    'freeze_manifest_sha256': sha(HERE / 'FREEZE-MANIFEST.json')}, sort_keys=True))
