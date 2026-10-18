#!/usr/bin/env python3
"""Independent finite arithmetic; no Moriarty imports, parsing, or execution.

The hand-transcribed statements below are checked against the complete public
source. Their expression trees are explicit oracle premises, not extracted IR.
This writes an oracle in its own schema, never a raw Program or Execution.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
INPUT_DIR = HERE.parent / 'guarded-normal-independent-inputs-20261018'
MAX_U128 = 2**128 - 1


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def clone(value):
    return json.loads(json.dumps(value))


def uint(value):
    result = int(value)
    assert 0 <= result <= MAX_U128
    return result


def checked(value):
    assert 0 <= value <= MAX_U128
    return value


def write(name, value):
    (HERE / name).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


source = (INPUT_DIR / 'quantity.mori').read_text()
supplied = json.loads((INPUT_DIR / 'INPUT.json').read_text())
manifest = json.loads((INPUT_DIR / 'INPUT-MANIFEST.json').read_text())
reference = (INPUT_DIR / 'CORE-REFERENCE-CONTRACT.md').read_text()
for row in manifest['files']:
    path = INPUT_DIR / row['path']
    assert path.stat().st_size == row['bytes']
    assert digest(path) == row['sha256']
assert 'moriarty-reference-logical-cost/1' in reference
assert supplied['action'] == 'first_ge'

# Hand-transcribed complete body. Expression node labels specify the source-tree
# premise, not concrete parser allocation, opcode spellings, or ExprId values.
statements = [
    ('requires first >= second;', ['ge(first,second)', 'name:first', 'name:second']),
    ('requires active;', ['name:active']),
    ('let maximum = first;', ['name:first']),
    ('let minimum = second;', ['name:second']),
    ('let quantity = maximum - minimum;', ['sub(maximum,minimum)', 'name:maximum', 'name:minimum']),
    ('let quote = quantity * price;', ['mul(quantity,price)', 'name:quantity', 'name:price']),
    ('let feeCharge = baseFee + minimum;', ['add(baseFee,minimum)', 'name:baseFee', 'name:minimum']),
    ('let gross = quote + feeCharge;', ['add(quote,feeCharge)', 'name:quote', 'name:feeCharge']),
    ('transfer(Cash, "buyer", "seller", quote);', ['name:quote']),
    ('fee(Cash, "buyer", "treasury", feeCharge);', ['name:feeCharge']),
    ('liability("delivery", Cash, "seller", "buyer", quote);', ['name:quote']),
    ('discharge("delivery", settleAmount);', ['name:settleAmount']),
    ('next filled = filled + quantity;', ['add(filled,quantity)', 'name:filled', 'name:quantity']),
    ('next lastGross = gross;', ['name:gross']),
    ('emit "max"(maximum);', ['name:maximum']),
    ('emit "min"(minimum);', ['name:minimum']),
    ('emit "quantity"(quantity);', ['name:quantity']),
    ('ensures lastGross == gross;', ['eq(lastGross,gross)', 'name:lastGross', 'name:gross']),
]
body = source.split('action first_ge(', 1)[1].split('{', 1)[1]
source_body = [line.strip() for line in body.splitlines() if line.strip() not in ('', '}')]
assert source_body == [row[0] for row in statements]

scenario = clone(supplied['scenario'])
frame = {key: clone(scenario[key]) for key in ('state', 'balances', 'liabilities')}
opening = clone(frame)
inputs = {key: uint(value['value']) for key, value in scenario['inputs'].items()}
bindings = {}
economic = []
emissions = []
state_writes = []
source_order_effects = []
prefixes = []
schedule = []
used = 0


def balance(account, reference_name):
    matches = [row for row in frame['balances'] if row['account'] == account and row['asset']['reference'] == reference_name]
    assert len(matches) == 1
    return matches[0]


def duty(name):
    matches = [row for row in frame['liabilities'] if row['name'] == name]
    assert len(matches) == 1
    return matches[0]


def payment(kind, recipient, amount):
    payer_row = balance('buyer', 'Cash')
    receiver_row = balance(recipient, 'Cash')
    assert payer_row['asset'] == receiver_row['asset']
    payer_before, receiver_before = uint(payer_row['amount']), uint(receiver_row['amount'])
    payer_row['amount'] = str(checked(payer_before - amount))
    receiver_row['amount'] = str(checked(receiver_before + amount))
    effect = {
        'kind': kind, 'asset': clone(payer_row['asset']), 'from': 'buyer', 'to': recipient,
        'amount': str(amount), 'payer_before': str(payer_before), 'payer_after': payer_row['amount'],
        'recipient_before': str(receiver_before), 'recipient_after': receiver_row['amount'],
    }
    economic.append(effect)
    source_order_effects.append(effect)


for index, (statement, nodes) in enumerate(statements):
    before = clone(frame)
    effects_before = len(source_order_effects)
    expression_result = None
    if index == 0:
        expression_result = inputs['first'] >= inputs['second']
        assert expression_result
    elif index == 1:
        expression_result = frame['state']['active']['value']
        assert expression_result is True
    elif index == 2:
        bindings['maximum'] = inputs['first']
        expression_result = str(bindings['maximum'])
    elif index == 3:
        bindings['minimum'] = inputs['second']
        expression_result = str(bindings['minimum'])
    elif index == 4:
        bindings['quantity'] = checked(bindings['maximum'] - bindings['minimum'])
        expression_result = str(bindings['quantity'])
    elif index == 5:
        bindings['quote'] = checked(bindings['quantity'] * inputs['price'])
        expression_result = str(bindings['quote'])
    elif index == 6:
        bindings['feeCharge'] = checked(inputs['baseFee'] + bindings['minimum'])
        expression_result = str(bindings['feeCharge'])
    elif index == 7:
        bindings['gross'] = checked(bindings['quote'] + bindings['feeCharge'])
        expression_result = str(bindings['gross'])
    elif index == 8:
        expression_result = str(bindings['quote'])
        payment('transfer', 'seller', bindings['quote'])
    elif index == 9:
        expression_result = str(bindings['feeCharge'])
        payment('fee', 'treasury', bindings['feeCharge'])
    elif index == 10:
        expression_result = str(bindings['quote'])
        row = duty('delivery')
        assert row['asset'] == balance('buyer', 'Cash')['asset']
        assert row['debtor'] == 'seller' and row['creditor'] == 'buyer'
        prior = uint(row['amount'])
        row['amount'] = str(checked(prior + bindings['quote']))
        effect = {'kind': 'liability_add', 'name': 'delivery', 'asset': clone(row['asset']),
                  'debtor': row['debtor'], 'creditor': row['creditor'], 'amount': expression_result,
                  'opening_present': True, 'opening_amount': str(prior),
                  'closing_present': True, 'closing_amount': row['amount']}
        economic.append(effect)
        source_order_effects.append(effect)
    elif index == 11:
        expression_result = str(inputs['settleAmount'])
        row = duty('delivery')
        prior = uint(row['amount'])
        row['amount'] = str(checked(prior - inputs['settleAmount']))
        effect = {'kind': 'discharge', 'name': 'delivery', 'asset': clone(row['asset']),
                  'debtor': row['debtor'], 'creditor': row['creditor'], 'amount': expression_result,
                  'opening_present': True, 'opening_amount': str(prior),
                  'closing_present': True, 'closing_amount': row['amount'],
                  'balance_change': False}
        economic.append(effect)
        source_order_effects.append(effect)
    elif index == 12:
        old = uint(frame['state']['filled']['value'])
        expression_result = str(checked(old + bindings['quantity']))
        frame['state']['filled']['value'] = expression_result
        effect = {'kind': 'state_write', 'name': 'filled', 'before': str(old), 'after': expression_result}
        state_writes.append(effect)
        source_order_effects.append(effect)
    elif index == 13:
        old = uint(frame['state']['lastGross']['value'])
        expression_result = str(bindings['gross'])
        frame['state']['lastGross']['value'] = expression_result
        effect = {'kind': 'state_write', 'name': 'lastGross', 'before': str(old), 'after': expression_result}
        state_writes.append(effect)
        source_order_effects.append(effect)
    elif index in (14, 15, 16):
        label, binding = [('max', 'maximum'), ('min', 'minimum'), ('quantity', 'quantity')][index - 14]
        expression_result = str(bindings[binding])
        effect = {'kind': 'emit', 'name': label, 'value': {'type': 'UInt', 'value': expression_result}}
        emissions.append(effect)
        source_order_effects.append(effect)
    elif index == 17:
        expression_result = uint(frame['state']['lastGross']['value']) == bindings['gross']
        assert expression_result
    else:
        raise AssertionError(index)
    charge = 1 + len(nodes)
    schedule.append({
        'statement_ordinal': index + 1, 'source_statement': statement,
        'statement_units': 1, 'uncached_expression_nodes': nodes,
        'expression_units': len(nodes), 'units': charge,
        'used_before': str(used), 'used_after': str(used + charge),
    })
    used += charge
    prefixes.append({
        'statement_ordinal': index + 1, 'source_statement': statement,
        'evaluated_expression_result': expression_result,
        'bindings_after': {key: {'type': 'UInt', 'value': str(value)} for key, value in bindings.items()},
        'opening_frame': before, 'closing_frame': clone(frame),
        'statement_semantic_operations': clone(source_order_effects[effects_before:]),
        'cumulative_economic_effects': clone(economic),
        'cumulative_state_writes': clone(state_writes),
        'cumulative_emissions': clone(emissions),
        'cumulative_semantic_operations': clone(source_order_effects),
        'logical_used_under_stated_mapping_premise': str(used),
    })

closing = clone(frame)
flows = []
for opening_row, closing_row in zip(opening['balances'], closing['balances']):
    assert (opening_row['account'], opening_row['asset']) == (closing_row['account'], closing_row['asset'])
    relevant = [effect for effect in economic if effect['kind'] in ('transfer', 'fee') and effect['asset'] == opening_row['asset']]
    gross_debit = sum(uint(effect['amount']) for effect in relevant if effect['from'] == opening_row['account'])
    gross_credit = sum(uint(effect['amount']) for effect in relevant if effect['to'] == opening_row['account'])
    fee_debit = sum(uint(effect['amount']) for effect in relevant if effect['kind'] == 'fee' and effect['from'] == opening_row['account'])
    fee_credit = sum(uint(effect['amount']) for effect in relevant if effect['kind'] == 'fee' and effect['to'] == opening_row['account'])
    net = gross_credit - gross_debit
    assert uint(opening_row['amount']) + net == uint(closing_row['amount'])
    flows.append({'account': opening_row['account'], 'asset': opening_row['asset'],
                  'opening_present': True, 'closing_present': True,
                  'opening_amount': opening_row['amount'], 'closing_amount': closing_row['amount'],
                  'gross_debit_including_fees': str(gross_debit),
                  'nonfee_debit': str(gross_debit - fee_debit), 'fee_debit': str(fee_debit),
                  'gross_credit': str(gross_credit), 'fee_credit': str(fee_credit),
                  'signed_net_flow': str(net)})

liability_evolution = []
for prior, after in zip(opening['liabilities'], closing['liabilities']):
    assert prior['name'] == after['name']
    adds = sum(uint(effect['amount']) for effect in economic if effect['kind'] == 'liability_add' and effect['name'] == prior['name'])
    discharges = sum(uint(effect['amount']) for effect in economic if effect['kind'] == 'discharge' and effect['name'] == prior['name'])
    assert uint(prior['amount']) + adds - discharges == uint(after['amount'])
    liability_evolution.append({'name': prior['name'], 'asset': prior['asset'],
                                'debtor': prior['debtor'], 'creditor': prior['creditor'],
                                'opening_present': True, 'opening_amount': prior['amount'],
                                'addition': str(adds), 'discharge': str(discharges),
                                'closing_present': True, 'closing_amount': after['amount']})

asset_totals = []
assets = {json.dumps(row['asset'], sort_keys=True): row['asset'] for row in opening['balances']}
for identity, asset in assets.items():
    prior = sum(uint(row['amount']) for row in opening['balances'] if row['asset'] == asset)
    after = sum(uint(row['amount']) for row in closing['balances'] if row['asset'] == asset)
    assert prior == after
    asset_totals.append({'asset': asset, 'opening_total': str(prior), 'closing_total': str(after),
                         'minted': '0', 'burned': '0', 'net_external_flow': '0'})

unknowns = [
    {'id': 'U1', 'topic': 'Surface-to-IR expression graph',
     'unknown': 'The reference contract defines per-ExprId charging but does not specify exact lowering/allocation of every surface name, arithmetic/comparison, asset/party/name argument, or wrapper into ExprIds.',
     'impact': '50 is conditional on the explicit ordinary expression-tree mapping; no implementation inspection was used to establish that mapping.'},
    {'id': 'U2', 'topic': 'Concrete expression IDs and cache sharing',
     'unknown': 'Concrete ExprIds and parser interning are unobserved. The supplied program has no repeated source name within any one statement expression; the model assumes each listed node is evaluated once. Name references read bound values rather than reevaluating let right-hand sides.',
     'impact': 'No raw Program or expression-ID schedule is predicted. Caches reset between statements as the reference specifies.'},
    {'id': 'U3', 'topic': 'Raw statement/step/effect encoding',
     'unknown': 'The input contract does not give raw serde discriminants, enum payload layout, Step.value representation, or whether next writes appear in the raw effects array.',
     'impact': 'This oracle uses its own explicit semantic operations and evaluated-expression fields; it does not invent raw Program/Execution fields or claim byte equality.'},
    {'id': 'U4', 'topic': 'Canonical row ordering',
     'unknown': 'The reference promises canonical balance/liability ordering without specifying its comparator.',
     'impact': 'Oracle frame rows retain input order. Compare complete keyed asset/account and liability identities semantically; raw ordering needs separately supplied public schema evidence.'},
    {'id': 'U5', 'topic': 'Consumer configuration and product observation',
     'unknown': 'No logical cap, sponsor configuration, trusted bootstrap, consumer result, parser result, or runtime output was supplied or executed.',
     'impact': 'No sponsor acceptance, actual execution, test pass, proof, ledger, or product acceptance claim is made.'},
    {'id': 'U6', 'topic': 'Runtime provenance outside finite arithmetic',
     'unknown': 'Authentication/authorization of balances, duties, assets, source, and scenario is outside these public inputs.',
     'impact': 'This is a normal functional finance prediction for the supplied scenario, not a security judgment or native fee/proof result.'},
]

expectation = {
    'schema': 'moriarty-independent-financial-semantic-expectation/1',
    'input_id': supplied['id'], 'action': supplied['action'],
    'purpose': 'Frozen independent normal functional arithmetic prediction before consumer comparison; own semantic schema, not product output.',
    'semantic_prediction': 'All written guards and ensures hold; checked arithmetic and explicit balance/duty arithmetic are within u128.',
    'scope': {'opening_and_closing_frames_complete_for_supplied_scenario': True,
              'all_18_statement_prefixes_included': True, 'raw_product_representation_predicted': False,
              'product_executed': False, 'acceptance_claim': False},
    'frame_conventions': {'row_order': 'input order, without a raw canonical-order claim',
                          'balance_presence': 'every listed row is present; no other balance row is introduced',
                          'liability_presence': 'every listed liability remains present, including settledDuty at zero; unlisted liabilities remain absent',
                          'state': 'complete typed scenario state; changes apply immediately at next',
                          'source_order_effects': 'own semantic operations includes explicit state writes, without asserting raw effect schema'},
    'inputs': clone(scenario['inputs']),
    'bindings': {key: {'type': 'UInt', 'value': str(value)} for key, value in bindings.items()},
    'opening_frame': opening, 'closing_frame': closing,
    'statement_prefixes': prefixes,
    'ordered_economic_effects': economic, 'ordered_state_writes': state_writes,
    'ordered_emissions': emissions, 'ordered_semantic_operations': source_order_effects,
    'per_account_asset_flows': flows, 'liability_evolution': liability_evolution,
    'asset_conservation': asset_totals,
    'explicit_absence_and_preservation': {
        'balance_rows_before': 8, 'balance_rows_after': 8,
        'liability_rows_before': 3, 'liability_rows_after': 3,
        'zero_liability': {'name': 'settledDuty', 'present_before': True, 'present_after': True, 'amount_before': '0', 'amount_after': '0'},
        'all_unlisted_balance_and_liability_rows': 'remain absent; no implicit account creation',
        'reserve_balances': {'buyer': '3', 'seller': '5', 'treasury': '7', 'witness': '11'},
        'witness_cash': '41', 'reserveDuty': '17',
        'preserved_state': {'active': {'type': 'Bool', 'value': True}, 'marker': {'type': 'UInt', 'value': '91'}},
        'discharge_cash_flow': '0', 'token_supply_liability_conflation': False,
    },
    'unknowns_and_unestablished_premises': unknowns,
}

logical = {
    'schema': 'moriarty-independent-logical-work-expectation/1',
    'profile': 'moriarty-reference-logical-cost/1',
    'status': 'conditional on explicit surface-to-expression-node mapping; not observed runtime cost',
    'mapping_premises': [
        'Every source statement shown contributes exactly one statement charge, including let, requires, ensures, next, emit, and financial statements.',
        'Each name reference is one expression node; each binary operator/comparison is one additional expression node.',
        'Transfer/fee/liability/discharge evaluate only their amount expression; asset identifiers and literal account/liability/event names are statement metadata rather than separately charged expression nodes.',
        'No hidden wrapper, injected, or desugared expression node is evaluated; no constant folding removes the listed nodes.',
        'Let references return captured values and state references return current state, without re-evaluating earlier expressions.',
        'There are no repeated nodes within an expression tree for this program; per-statement caches reset, so equal values or repeated names in different statements do not share charges.',
        'The listed node order describes a root-before-left-before-right charging premise only; successful totals do not depend on the within-expression traversal order.',
    ],
    'schedule': schedule,
    'statement_units': len(statements), 'expression_units': sum(len(nodes) for _, nodes in statements),
    'total_units': str(used),
    'sufficient_cap_under_mapping': str(used),
    'one_unit_short_prediction_under_mapping': {
        'cap': str(used - 1), 'refusal_statement_ordinal': 18,
        'refusal_context': 'final equality has one pending charge after statement and two expression charges',
        'used_on_denied_charge': str(used - 1),
        'partial_execution_or_committed_effects_on_refusal': False,
        'caveat': 'exact denied expression node depends on unobserved traversal and lowering; no consumer run performed',
    },
    'excluded_costs': ['parsing', 'validation', 'normalization', 'snapshot allocation', 'lookup complexity', 'replay comparison', 'wall-clock', 'allocation bound', 'DUST', 'native proving', 'ledger settlement'],
    'unknowns': ['U1', 'U2', 'U3', 'U5'],
}

write('EXPECTED-SEMANTICS.json', expectation)
write('EXPECTED-LOGICAL-WORK.json', logical)
read_paths = [
    '/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.1/skills/develop/SKILL.md',
    '/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018/AGENTS.md',
    '/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018/docs/MORIARTY-PRODUCT-CONTRACT.md',
    '/home/charl/.agents/skills/superpowers/skills/using-superpowers/SKILL.md',
]
provenance = {
    'schema': 'moriarty-independent-oracle-provenance/1',
    'frozen_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'requested_author_identity': 'gpt-6.1-sol high',
    'task_identity': '/root/guarded_normal_independent_finance_sol',
    'actual_model_identity': 'Model routing was inherited from the parent agent; no separate provider invocation or returned model-identity receipt was available to this arithmetic task.',
    'independence': {
        'fresh_context_received_prior_outputs_or_expected_values': False,
        'integrator_selected_existing_inputs': True,
        'integrator_has_implementation_exposure': True,
        'inputs_expected_and_financial_derivation_excluded_per_manifest': manifest['expectedAndFinancialDerivationExcluded'],
        'implementation_parser_runtime_logical_cost_tests_author_goldens_prior_reports_inspected': False,
        'product_provider_compiler_sdk_crypto_key_wallet_ledger_execution': False,
        'web_research': False, 'git_or_shared_writes': False,
        'future_product_comparison_responsibility': 'parent integrator; separate from this frozen authoring task',
        'no_test_or_acceptance_claim': True,
    },
    'program_reference_data': [{'path': str(INPUT_DIR / name), 'bytes': (INPUT_DIR / name).stat().st_size,
                                'sha256': digest(INPUT_DIR / name)}
                               for name in ('quantity.mori', 'INPUT.json', 'CORE-REFERENCE-CONTRACT.md', 'INPUT-MANIFEST.json')],
    'administrative_read_paths': read_paths,
    'administrative_read_only_command': {
        'cwd': '/home/charl/Moriarty/.worktrees/utxo-actus-prefix-main-20261018',
        'command': 'python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json',
        'observed_summary': 'Historical SP01.6 loan-swap-subset record; dependent dispatch blocked by stale binding/candidate, missing current-accounting and unavailable resource live state; pendingTransactions empty.',
        'effect_on_task': 'No consumer acceptance inference or campaign dispatch; independent standalone arithmetic remains the authorized task.',
    },
    'derivation_script_sha256': digest(HERE / 'derive.py'),
    'unknown_ids': [row['id'] for row in unknowns],
}
write('PROVENANCE.json', provenance)
readme = '''# Frozen independent functional expectation

This packet predicts normal functional finance from the four public frozen inputs.
It is a standalone arithmetic derivation in an independent semantic schema. No
Moriarty program, parser, runtime, compiler, sponsor, SDK, wallet, or ledger was run.
No security test or acceptance judgment is supplied.

The integrator selected these existing inputs and has implementation exposure.
This fresh author context received no product outputs, author expected values,
fixtures, goldens, or prior reports. Required administrative startup documents and
the read-only plugin status were read; they supplied no case-specific results.

Run `python3 derive.py` only to reproduce this standalone arithmetic packet. A rerun
updates provenance time and the manifest. Preserve this frozen original before any
rerun. The parent must compare the frozen packet separately after an actual consumer
check; do not adapt this expectation to observed implementation behavior.

The six computed bindings are maximum 21, minimum 6, quantity 15, quote 45,
feeCharge 8, and gross 53. Buyer Cash closes at 147, seller at 65, treasury at 11,
and witness at 41. Reserve balances remain buyer 3, seller 5, treasury 7, witness 11.
Cash totals 264 and Reserve totals 26 remain conserved. Delivery evolves
14 + 45 - 11 = 48, with no payment implied by discharge. reserveDuty remains 17;
settledDuty remains present at zero. State closes filled 19, lastGross 53,
marker 91, active true. The 18 complete statement prefixes include full pre/post
financial frames, captured bindings, economic effects, state writes and emissions.

The logical schedule predicts 50 units: 18 statement charges and 32 expression-node
charges. This is conditional on the explicit mapping premises in
EXPECTED-LOGICAL-WORK.json. The reference defines costs per ExprId but does not pin
surface lowering or concrete IDs. No raw Program or Execution representation is
invented. Raw serde fields, statement values, effects encoding, row ordering,
expression-ID allocation and actual consumer results remain unobserved.

FREEZE-MANIFEST.json hashes the script and all authored artifacts. The manifest does
not hash itself. PROVENANCE.json records all four input hashes and limitations.
'''
(HERE / 'README.md').write_text(readme)
artifacts = ('derive.py', 'EXPECTED-SEMANTICS.json', 'EXPECTED-LOGICAL-WORK.json', 'PROVENANCE.json', 'README.md')
write('FREEZE-MANIFEST.json', {
    'schema': 'moriarty-independent-oracle-freeze/1',
    'frozen_at_utc': provenance['frozen_at_utc'],
    'before_any_consumer_execution_in_this_task': True,
    'artifacts': [{'path': name, 'bytes': (HERE / name).stat().st_size, 'sha256': digest(HERE / name)} for name in artifacts],
    'manifest_self_hash_excluded': True,
})
print(json.dumps({'logical_units_conditional': str(used), 'prefixes': len(prefixes),
                  'closing_cash': {row['account']: row['amount'] for row in closing['balances'] if row['asset']['reference'] == 'Cash'},
                  'closing_liabilities': {row['name']: row['amount'] for row in closing['liabilities']},
                  'freeze_manifest_sha256': digest(HERE / 'FREEZE-MANIFEST.json')}, sort_keys=True))
