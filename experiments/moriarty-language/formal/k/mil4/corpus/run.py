#!/usr/bin/env python3
"""Finite, explicit S0 requests with independently fixed verdicts."""
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
DEFINITION = Path('/tmp/mil4-s1b-corpus-llvm-kompiled')
U = 2**128 - 1
S = 2**127 - 1


def q(value):
    return json.dumps(value)


def call(name, *args):
    return f'{name}({",".join(map(str, args))})'


def balance(who, amount, asset="A"):
    return call('balance', q(who), q(asset), amount)


def transfer_case(name, *, value=10, fee=1, cap=1, gross=11,
                  receiver='R', fill_receiver=None, fee_effect=True,
                  allowance=11, replay='noReplays()', head='h0',
                  state_head=None, receiver_balance=0, round=7,
                  owner_cell="O", recipient_cell="R", fee_cell="F",
                  cell_asset="A", allowance_owner="O", allowance_asset="A",
                  fee_receiver="F", post_head='h1', effect_head=None,
                  outcome=None, premise_post='h1'):
    post = call('head', q(post_head))
    effect_post = call('head', q(effect_head if effect_head is not None else post_head))
    signed_head = call('head', q(head))
    pre = call('state',
               balance(owner_cell, 100, cell_asset) if owner_cell is not None else 'noBalance()',
               balance(recipient_cell, receiver_balance, cell_asset) if recipient_cell is not None else 'noBalance()',
               balance(fee_cell, 0, cell_asset) if fee_cell is not None else 'noBalance()',
               call('allowance', q(allowance_owner), q(allowance_asset), allowance, 0),
               'noObligation()', call('head', q(state_head or head)), replay, 1, 0)
    action = call('transfer', value, fee)
    intent = call('intent', q('Source/6'), q('Core/5'), q('D'), q('O'), q('A'),
                  q(name), signed_head, 0, 10, q(receiver), q(fee_receiver), gross, cap,
                  0, action, q('digest-'+name))
    fill = call('fill', q('D'), q('O'), q('A'), q(name), signed_head,
                q(fill_receiver or receiver), q(fee_receiver), gross, cap, 0, action, post)
    effects = [call('debit', q('O'), q('A'), value+fee),
               call('credit', q(receiver), q('A'), value)]
    if fee and fee_effect:
        effects.append(call('credit', q(fee_receiver), q('A'), fee))
    effects += [call('useAllowance', q('O'), q('A'), value+fee),
                call('useReplay', call('replayKey', q('D'), q('O'), q(name))),
                call('advanceHead', signed_head, effect_post)]
    return make(name, intent, fill, pre, effects, round, call('head', q(premise_post)), outcome)


def repay_case(name, *, principal=1000, accrued=10, amount=30, payer_balance=100,
               creditor_balance=0, allowance=100, spent=0, credit=True,
               credit_to='C', round=7, debtor='P', signer='P',
               multiplier=1, scale=0, rounding='none', allowance_owner='P'):
    prehead, post = call('head', q('h0')), call('head', q('h1'))
    total = principal + accrued
    debt = call('obligation', q('L'), q(debtor), q('C'), q('A'), principal,
                accrued, total, q('Outstanding'))
    pre = call('state', balance('P', payer_balance), balance('C', creditor_balance),
               'noBalance()', call('allowance', q(allowance_owner), q('A'), allowance, spent),
               debt, prehead, 'noReplays()', 1, 0)
    action = call('repay', q('L'), q('P'), amount, multiplier, scale, q(rounding))
    intent = call('intent', q('Source/6'), q('Core/5'), q('D'), q(signer), q('A'),
                  q(name), prehead, 0, 10, q(''), q(''), amount, 0, 0, action,
                  q('digest-'+name))
    fill = call('fill', q('D'), q(signer), q('A'), q(name), prehead, q(''), q(''),
                amount, 0, 0, action, post)
    paid_accrued = min(amount, accrued)
    remaining = total - amount
    effects = [call('debit', q('P'), q('A'), amount)]
    if credit:
        effects.append(call('credit', q(credit_to), q('A'), amount))
    effects += [call('setObligation', q('L'), principal-(amount-paid_accrued),
                     accrued-paid_accrued, remaining,
                     q('Settled' if remaining == 0 else 'Outstanding')),
                call('useAllowance', q('P'), q('A'), amount),
                call('useReplay', call('replayKey', q('D'), q('P'), q(name))),
                call('advanceHead', prehead, post)]
    return make(name, intent, fill, pre, effects, round, post)


def effects_term(items):
    tail = 'noEffects()'
    for item in reversed(items):
        tail = call('effect', item, tail)
    return tail


def make(name, intent, fill, pre, effects, round, post, outcome=None):
    requested = outcome or call('outcome', 'terminalSuccess()', 'noEffects()', 'noDuties()')
    request = call('submit', intent, fill, pre, effects_term(effects), requested, round)
    premise = call('authenticated', intent, pre, round, post, requested)
    return {'name': name, 'request': request, 'premise': premise,
            'intent': intent, 'preState': pre, 'round': round, 'outcome': requested}


def corpus():
    cases = [
        (transfer_case('T-10-1'), 'accepted'),
        (repay_case('R-30'), 'accepted'),
        (repay_case('R-near-bound', principal=S-1, accrued=1, amount=1,
                    payer_balance=1, creditor_balance=U-1, allowance=1,
                    spent=U-1), 'accepted'),
        (transfer_case('H-recipient', fill_receiver='R-prime'),
         ('intent', 'S0_INTENT_SCOPE')),
        (transfer_case('H-fee-cap', fee=2, cap=1, gross=12, allowance=12),
         ('intent', 'S0_INTENT_SCOPE')),
        (transfer_case('H-missing-fee', fee_effect=False),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (repay_case('H-missing-credit', credit=False),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (repay_case('H-wrong-creditor', credit_to='X'),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (transfer_case('H-allowance', allowance=10),
         ('authority', 'S0_AUTH_SCOPE')),
        (transfer_case('H-stale-head', state_head='h-current'),
         ('history', 'S0_HISTORY_STALE')),
        (transfer_case('H-replay', replay=call('used',
             call('replayKey', q('D'), q('O'), q('H-replay')), 'noReplays()')),
         ('history', 'S0_HISTORY_REPLAY')),
        (transfer_case('H-overflow', receiver_balance=U),
         ('effect', 'S0_EFFECT_RANGE')),
        (repay_case('H-overpay', amount=1011, payer_balance=1011,
                    allowance=1011), ('effect', 'S0_EFFECT_RANGE')),
        (transfer_case('H-both', fee_effect=False, state_head='h-current'),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (transfer_case('H-missing-owner-cell', owner_cell=None), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-missing-recipient-cell', recipient_cell=None), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-missing-fee-cell', fee_cell=None), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-owner-cell', owner_cell='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-recipient-cell', recipient_cell='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-fee-cell', fee_cell='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-swapped-transfer-cells', recipient_cell='F', fee_cell='R'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-cell-asset', cell_asset='B', allowance_asset='B'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-allowance-owner', allowance_owner='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-wrong-allowance-asset', allowance_asset='B'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (repay_case('H-debtor-mismatch', debtor='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (repay_case('H-payer-signer-mismatch', signer='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (repay_case('H-repay-allowance-owner', allowance_owner='X'), ('stage', 'S0_STAGE_UNSUPPORTED')),
        (transfer_case('H-alias-owner-recipient', receiver='O'), ('intent', 'S0_INTENT_ALIAS')),
        (transfer_case('H-alias-owner-fee', fee_receiver='O'), ('intent', 'S0_INTENT_ALIAS')),
        (transfer_case('H-alias-recipient-fee', fee_receiver='R'), ('intent', 'S0_INTENT_ALIAS')),
        (transfer_case('H-invalid-round-alias', receiver='O', round=11), ('intent', 'S0_INTENT_SCOPE')),
        (transfer_case('H-cap-alias', receiver='O', fee=2, cap=1, gross=12, allowance=12), ('intent', 'S0_INTENT_SCOPE')),
        (transfer_case('H-overflow-missing-fee', receiver_balance=U, fee_effect=False), ('effect', 'S0_EFFECT_RANGE')),
        (repay_case('H-overpay-missing-credit', amount=1011, payer_balance=1011, allowance=1011, credit=False), ('effect', 'S0_EFFECT_RANGE')),
        (repay_case('H-nonidentity-multiplier', multiplier=2), ('intent', 'S0_INTENT_SCOPE')),
        (repay_case('H-nonidentity-scale', scale=1), ('intent', 'S0_INTENT_SCOPE')),
        (repay_case('H-nonidentity-rounding', rounding='floor'), ('intent', 'S0_INTENT_SCOPE')),

    ]
    return cases


def run():
    results = []
    for case, expected in corpus():
        cmd = ['krun', '--definition', str(DEFINITION), '-o', 'pretty', '/dev/stdin']
        program = call('withAuthenticated', case['request'], case['premise'])
        proc = subprocess.run(cmd, input=program, text=True,
                              capture_output=True, timeout=120)
        output = proc.stdout.strip()
        compact = ''.join(output.split())
        if isinstance(expected, tuple):
            verdict = f'rejected({q(expected[0])},{q(expected[1])},1)'
            matched = verdict in compact
        else:
            matched = 'accepted(' in compact and 'rejected(' not in compact
        results.append({'case': case['name'], 'expected': expected,
                        'matched': matched and proc.returncode == 0,
                        'exitCode': proc.returncode, 'command': cmd,
                        'request': case['request'], 'premise': case['premise'],
                        'program': program,
                        'stdout': proc.stdout, 'stderr': proc.stderr})
        print(f'{case["name"]}: {"MATCH" if results[-1]["matched"] else "MISMATCH"} '
              f'(exit={proc.returncode})', flush=True)
    (HERE / 's1b-smoke-results.json').write_text(json.dumps(results, indent=2) + '\n')
    return 0 if all(x['matched'] for x in results) else 1


if __name__ == '__main__':
    raise SystemExit(run())
