#!/usr/bin/env python3
"""Strict H/F/P experiment under explicit stipulated tuples, never authentication.

Every case uses T-10-1's nonce/intent and round zero unless a named mutation
changes it. Expected accepted output is the independent literal T-10-1 term.
"""
import json
from pathlib import Path

from run import call, effects_term, q, transfer_case
from run_checked import EXPECTED_ACCEPTED, check_case

HERE = Path(__file__).resolve().parent


def case(name, **changes):
    result = transfer_case('T-10-1', round=0, **changes)
    result['name'] = name
    return result


def unavailable(name, **changes):
    result = case(name, **changes)
    result['premise'] = 'unavailable()'
    return result


def mismatched_premise(name, field):
    result = case(name)
    intent, pre, round = result['intent'], result['preState'], result['round']
    outcome = result['outcome']
    if field == 'intent':
        intent = intent.replace(q('digest-T-10-1'), q('digest-other'))
    elif field == 'preState':
        pre = pre.replace('balance("O","A",100)', 'balance("O","A",99)')
    elif field == 'round':
        round = 1
    elif field == 'outcome':
        outcome = call('outcome', call('requestedFailure', q('failure')), 'noEffects()', 'noDuties()')
    else:
        raise ValueError(field)
    result['premise'] = call('authenticated', intent, pre, round, 'head("h1")', outcome)
    return result


def s1b_cases():
    failed = call('outcome', call('requestedFailure', q('failure')), 'noEffects()', 'noDuties()')
    retained_effect = call('outcome', 'terminalSuccess()',
                           effects_term([call('credit', q('R'), q('A'), 1)]), 'noDuties()')
    retained_duty = call('outcome', 'terminalSuccess()', 'noEffects()',
                         call('duty', q('duty'), 'noDuties()'))
    consumed = call('used', call('replayKey', q('D'), q('O'), q('T-10-1')), 'noReplays()')
    return [
        (case('H1'), 'accepted'),
        (case('H2', post_head='h9'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H3', post_head='h0'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H3-self-premise', post_head='h0', premise_post='h0'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H4', effect_head='h9'), ('effect', 'S0_EFFECT_MISMATCH')),
        (case('H5', head='h9', state_head='h0', post_head='h9'), ('history', 'S0_HISTORY_STALE')),
        (case('H6', replay=consumed, post_head='h9'), ('history', 'S0_HISTORY_REPLAY')),
        (case('F1', outcome=failed), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (case('F2', outcome=retained_effect), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (case('F3', outcome=retained_duty), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (unavailable('P1'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-intent', 'intent'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-preState', 'preState'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-round', 'round'), ('stage', 'S0_STAGE_PREMISE')),
        (unavailable('P3', owner_cell=None), ('stage', 'S0_STAGE_UNSUPPORTED')),
        # Extra outcome binding control beyond the 14 required H/F/P instances.
        (mismatched_premise('P2-outcome', 'outcome'), ('stage', 'S0_STAGE_PREMISE')),
        # Combined controls pin the first observation, not just source rule order.
        (case('H7-stale-replay', head='h9', state_head='h0', replay=consumed),
         ('history', 'S0_HISTORY_STALE')),
        (case('H8-effect-replay', effect_head='h9', replay=consumed),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (case('H9-effect-successor', post_head='h9', effect_head='h1'),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (case('F4-effect-failure', effect_head='h9', outcome=failed),
         ('effect', 'S0_EFFECT_MISMATCH')),
    ]


def main():
    cases = s1b_cases()
    required = {'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 'F1', 'F2', 'F3',
                'P1', 'P2-intent', 'P2-preState', 'P2-round', 'P3', 'P2-outcome', 'H3-self-premise',
                'H7-stale-replay', 'H8-effect-replay', 'H9-effect-successor', 'F4-effect-failure'}
    names = [entry['name'] for entry, _ in cases]
    if len(names) != len(required) or set(names) != required:
        raise ValueError('missing, extra, or duplicate experiment case')
    results = []
    for entry, verdict in cases:
        expected = (EXPECTED_ACCEPTED['T-10-1'] if verdict == 'accepted'
                    else call('rejected', q(verdict[0]), q(verdict[1]), 1))
        result = check_case(entry, expected)
        results.append(result)
        print(f'{entry["name"]}: {"MATCH" if result["matched"] else "MISMATCH"}', flush=True)
    (HERE / 's1b-results.json').write_text(json.dumps(results, indent=2) + '\n')
    return 0 if all(entry['matched'] for entry in results) else 1


if __name__ == '__main__':
    raise SystemExit(main())
