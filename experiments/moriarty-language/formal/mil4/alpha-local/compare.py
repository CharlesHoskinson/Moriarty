#!/usr/bin/env python3
"""Finite alpha_local observation equality; no ledger refinement claim.

Read archived K observations, captured Quint ITF states and fresh direct Core
results. Amounts remain decimal strings throughout. Default execution is read
only. No receipt's expectedOut, matched, or externalPreserved flag is trusted.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
K_RECEIPT = ROOT / 'experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json'
CASES = {'T-10-1': 'transferCommonTest', 'R-30': 'repayThirtyCommonTest',
         'R-near-bound': 'repayNearBoundCommonTest'}
K_ACCOUNT_MAPS = {
    'T-10-1': {'O': 'O', 'R': 'R', 'F': 'F'},
    'R-30': {'P': 'O', 'C': 'C'},
    'R-near-bound': {'P': 'O', 'C': 'C'},
}
HEAD_MAPS = {
    'K': {'h0': '0', 'h1': '1'},
    'TypeScript': {'h0': '0', 'h1': '1'},
    'Quint': {'0': '0', '1': '1'},
}
Q_PREFIX = 's0_common_witnesses::s0::'
Q_FIELDS = {'round', 'currentHead', 'committedCount', 'obligations', 'lastEffects',
            'consumed', 'workSpent', 'balances', 'workRemaining', 'allowanceSpent',
            'signed', 'allowanceRemaining'}
K_NUMERIC_POSITIONS = {
    'balance': (2,), 'allowance': (2, 3), 'obligation': (4, 5, 6),
    'state': (7, 8), 'accepted': (5,), 'debit': (2,), 'credit': (2,),
    'useAllowance': (2,), 'setObligation': (1, 2, 3),
    'intent': (7, 8, 11, 12, 13), 'fill': (7, 8, 9),
    'transfer': (0, 1), 'repay': (2, 3, 4), 'submit': (5,),
}


class KInteger(str):
    """An unquoted integer token; distinguish it from a K String until validation."""


class QuintInteger(str):
    """An exact ITF #bigint; plain strings are not numeric observations."""


class QuintMap(dict):
    """An ITF #map, rather than an ordinary record."""


class QuintSet(list):
    """An ITF #set, rather than a tuple or list."""


class QuintTuple(list):
    """An ITF #tup, rather than a set or list."""


def require(condition, message):
    if not condition:
        raise ValueError(message)


def parse_term(source):
    """Decode the closed K constructor grammar, without eval or integer floats."""
    token_re = re.compile(r'\s*("(?:[^"\\]|\\.)*"|-?[0-9]+|[A-Za-z][A-Za-z0-9_]*|[(),])')
    tokens, pos = [], 0
    while pos < len(source):
        match = token_re.match(source, pos)
        if not match:
            require(not source[pos:].strip(), 'Invalid K token')
            break
        tokens.append(match.group(1))
        pos = match.end()
    cursor = 0

    def take():
        nonlocal cursor
        require(cursor < len(tokens), 'Truncated K term')
        token = tokens[cursor]
        cursor += 1
        return token

    def term():
        token = take()
        if token.startswith('"'):
            return json.loads(token)
        if re.fullmatch(r'-?[0-9]+', token):
            return KInteger(token)
        require(re.fullmatch(r'[A-Za-z][A-Za-z0-9_]*', token), 'Expected constructor')
        require(take() == '(', 'Expected opening parenthesis')
        args = []
        if cursor < len(tokens) and tokens[cursor] != ')':
            args.append(term())
            while cursor < len(tokens) and tokens[cursor] == ',':
                take()
                args.append(term())
        require(take() == ')', 'Expected closing parenthesis')
        for index in K_NUMERIC_POSITIONS.get(token, ()):
            require(index < len(args) and isinstance(args[index], KInteger),
                    f'K integer type: {token}[{index}]')
        return (token, args)

    result = term()
    require(cursor == len(tokens), 'Trailing K term')
    return result


def args(term, constructor, arity):
    require(isinstance(term, tuple) and term[0] == constructor
            and len(term[1]) == arity, f'Expected {constructor}/{arity}')
    return term[1]


def head(value, source):
    if source in ('K', 'TypeScript'):
        require(type(value) is str and value in ('h0', 'h1'),
                f'{source} head representation: expected opaque h0/h1')
        return HEAD_MAPS[source][value]
    require(source == 'Quint' and isinstance(value, QuintInteger) and value in ('0', '1'),
            'Quint head representation: expected integer 0/1')
    return HEAD_MAPS[source][value]


def account(value, name):
    require(name in K_ACCOUNT_MAPS and type(value) is str and value in K_ACCOUNT_MAPS[name],
            f'K account outside case-specific source domain: {name}/{value}')
    return K_ACCOUNT_MAPS[name][value]


def unique_map(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, f'Duplicate cell: {key}')
        result[key] = value
    return result


def k_state(term, name, round_value):
    # K's state constructor has no round field. This is submission context,
    # propagated only as an explicitly limited comparison annotation.
    a = args(term, 'state', 9)
    balances = []
    for cell in a[:3]:
        if cell == ('noBalance', []):
            continue
        owner, asset, amount = args(cell, 'balance', 3)
        require(asset == 'A', 'K balance asset')
        balances.append((account(owner, name), amount))
    owner, asset, remaining, spent = args(a[3], 'allowance', 4)
    require(asset == 'A', 'K allowance asset')
    obligations = {}
    if a[4] != ('noObligation', []):
        oid, debtor, creditor, asset, principal, accrued, outstanding, status = args(a[4], 'obligation', 8)
        obligations[oid] = dict(debtor=account(debtor, name), creditor=account(creditor, name),
                                asset=asset, principal=principal, accrued=accrued,
                                outstanding=outstanding, status=status)
    replays, rest = [], a[6]
    while rest != ('noReplays', []):
        replay, rest = args(rest, 'used', 2)
        domain, signer, nonce = args(replay, 'replayKey', 3)
        replays.append([domain, account(signer, name), nonce])
    require(len(replays) == len({tuple(x) for x in replays}), 'Duplicate K replay')
    return dict(balances=unique_map(balances),
                allowances={account(owner, name): dict(remaining=remaining, spent=spent)},
                obligations=obligations, head=head(args(a[5], 'head', 1)[0], 'K'),
                round=round_value, consumedReplay=sorted(replays),
                workRemaining=a[7], workSpent=a[8])


def k_effects(term, name):
    lines = []
    while term != ('noEffects', []):
        line, term = args(term, 'effect', 2)
        kind, values = line
        if kind in ('debit', 'credit'):
            owner, asset, amount = args(line, kind, 3)
            lines.append(dict(kind=kind.capitalize(), account=account(owner, name), asset=asset, amount=amount))
        elif kind == 'setObligation':
            oid, principal, accrued, outstanding, status = args(line, kind, 5)
            lines.append(dict(kind='SetObligation', id=oid, principal=principal, accrued=accrued,
                              outstanding=outstanding, status=status))
        elif kind == 'useAllowance':
            owner, asset, amount = args(line, kind, 3)
            require(asset == 'A', 'K effect allowance asset')
            lines.append(dict(kind='UseAllowance', owner=account(owner, name), amount=amount))
        elif kind == 'useReplay':
            domain, signer, nonce = args(args(line, kind, 1)[0], 'replayKey', 3)
            lines.append(dict(kind='UseReplay', key=[domain, account(signer, name), nonce]))
        elif kind == 'advanceHead':
            before, after = args(line, kind, 2)
            lines.append(dict(kind='AdvanceHead', predecessor=head(args(before, 'head', 1)[0], 'K'),
                              successor=head(args(after, 'head', 1)[0], 'K')))
        else:
            raise ValueError(f'Unsupported K effect: {kind}')
    return lines


def project_k(row):
    name = row['case']
    require(name in CASES, 'K case outside finite domain')
    require(row['exitCode'] == 0 and row['parseError'] is None, 'Failed K execution')
    captured = re.search(r'<out>\s*(.*?)\s*</out>', row['stdout'], re.S)
    require(captured is not None, 'Missing captured K out cell')
    observed = parse_term(row['observedOut'])
    require(observed == parse_term(captured.group(1)), 'K stdout/observedOut disagreement')
    pre, effects, post, phase, duty, diagnostic = args(observed, 'accepted', 6)
    require(phase == ('terminalSuccess', []) and duty == ('noDuty', []) and diagnostic == '0',
            'K observation outside successful finite domain')
    request = args(parse_term(row['request']), 'submit', 6)
    intent = args(request[0], 'intent', 16)
    require(intent[:3] == ['Source/6', 'Core/5', 'D'] and intent[4] == 'A', 'K version/domain/asset')
    require(intent[5] == name and request[-1] == str(row['round']), 'K nonce/round')
    require(pre == request[2], 'K observed pre differs from submitted pre')
    require(effects == request[3], 'K observed effects differ from submitted effects')
    return dict(pre=k_state(pre, name, request[-1]), effects=k_effects(effects, name),
                post=k_state(post, name, request[-1]))


def decode_itf(value):
    if isinstance(value, list):
        return [decode_itf(v) for v in value]
    if not isinstance(value, dict):
        require(isinstance(value, bool) or not isinstance(value, (float, int)), 'ITF number must use exact #bigint')
        return value
    wrappers = set(value) & {'#bigint', '#map', '#set', '#tup'}
    require(not wrappers or len(value) == 1, 'ITF wrapper exclusivity')
    if '#bigint' in value:
        require(set(value) == {'#bigint'} and re.fullmatch(r'0|[1-9][0-9]*', value['#bigint']), 'ITF bigint')
        return QuintInteger(value['#bigint'])
    if '#map' in value:
        require(type(value['#map']) is list and all(type(pair) is list and len(pair) == 2
                                                  for pair in value['#map']), 'ITF map payload type')
        return QuintMap(unique_map((decode_itf(k), decode_itf(v)) for k, v in value['#map']))
    if '#set' in value or '#tup' in value:
        key = '#set' if '#set' in value else '#tup'
        require(type(value[key]) is list, 'ITF collection payload type')
        container = QuintSet if key == '#set' else QuintTuple
        return container(decode_itf(v) for v in value[key])
    return {k: decode_itf(v) for k, v in value.items() if k != '#meta'}


def q_state(raw):
    require(set(raw) - {'#meta'} == {Q_PREFIX + field for field in Q_FIELDS},
            'Quint state namespace/footprint')
    values = {field: decode_itf(raw[Q_PREFIX + field]) for field in Q_FIELDS}
    for field in ['balances', 'allowanceRemaining', 'allowanceSpent', 'signed', 'obligations']:
        require(isinstance(values[field], QuintMap), f'Quint map type: {field}')
    require(isinstance(values['consumed'], QuintSet), 'Quint consumed set type')
    def replay_tuple(value):
        require(isinstance(value, QuintTuple) and len(value) == 3
                and all(type(part) is str for part in value), 'Quint replay tuple type')
    for replay in values['consumed']:
        replay_tuple(replay)
    require(type(values['lastEffects']) is list, 'Quint effect list type')
    def numeric(value, field):
        require(isinstance(value, QuintInteger), f'Quint integer type: {field}')
    def debt_record(debt):
        require(type(debt) is dict, 'Quint debt record type')
        for field in ['principal', 'accrued', 'outstanding']:
            numeric(debt[field], 'debt/' + field)
        status = debt['status']
        require(type(status) is dict and set(status) == {'tag', 'value'}
                and status['tag'] in ('Outstanding', 'Settled')
                and isinstance(status['value'], QuintTuple) and status['value'] == [], 'Quint debt status')
    for field in ['round', 'currentHead', 'committedCount', 'workRemaining', 'workSpent']:
        numeric(values[field], field)
    for field in ['balances', 'allowanceRemaining', 'allowanceSpent']:
        require(isinstance(values[field], dict), f'Quint map type: {field}')
        for key, value in values[field].items():
            require(type(key) is str, f'Quint map key type: {field}')
            numeric(value, field + '/' + key)
    for key, intent in values['signed'].items():
        require(type(intent) is dict, 'Quint intent record type')
        for field in ['preHead', 'validFrom', 'validThrough', 'grossCap', 'feeCap',
                      'netFloor', 'actionAmount', 'actionFee', 'conversionMantissa', 'conversionScale']:
            numeric(intent[field], 'signed/' + key + '/' + field)
    for line in values['lastEffects']:
        require(type(line) is dict, 'Quint effect record type')
        kind, value = line['tag'], line['value']
        if kind == 'UseReplay':
            replay_tuple(value)
        else:
            require(type(value) is dict, 'Quint effect payload record type')
        if kind in ('Debit', 'Credit', 'UseAllowance'):
            numeric(value['amount'], 'effect/' + kind + '/amount')
        elif kind == 'AdvanceHead':
            numeric(value['before'], 'effect/head/before')
            numeric(value['after'], 'effect/head/after')
        elif kind == 'SetObligation':
            debt_record(value['value'])
    require(values['allowanceRemaining'].keys() == values['allowanceSpent'].keys(), 'Quint allowance footprint')
    debts = {}
    for oid, debt in values['obligations'].items():
        debt_record(debt)
        status = debt['status']
        debts[oid] = {**debt, 'status': status['tag']}
    result = dict(balances=values['balances'],
                  allowances={owner: dict(remaining=n, spent=values['allowanceSpent'][owner])
                              for owner, n in values['allowanceRemaining'].items()},
                  obligations=debts, head=head(values['currentHead'], 'Quint'), round=values['round'],
                  consumedReplay=sorted(values['consumed']),
                  workRemaining=values['workRemaining'], workSpent=values['workSpent'])
    return result, values


def project_quint(trace, name):
    require(trace['#meta']['status'] == 'passed', 'Failed Quint trace')
    require(len(trace['states']) >= 2, 'Truncated Quint trace')
    pre, before = q_state(trace['states'][-2])
    post, after = q_state(trace['states'][-1])
    require(before['committedCount'] == '0' and after['committedCount'] == '1', 'Quint commit count')
    require(before['lastEffects'] == [] and before['signed'] == after['signed'], 'Quint signing/effect precondition')
    signed = before['signed']
    require(set(signed) == {'digest-' + name}, 'Quint signed footprint')
    intent = signed['digest-' + name]
    require((intent['sourceVersion'], intent['coreVersion'], intent['domain'], intent['asset'], intent['nonce'])
            == ('Source/6', 'Core/5', 'D', 'A', name), 'Quint version/domain/asset/nonce')
    lines = []
    for line in after['lastEffects']:
        kind, value = line['tag'], line['value']
        if kind in ('Debit', 'Credit'):
            lines.append(dict(kind=kind, asset=intent['asset'], **value))
        elif kind == 'SetObligation':
            debt = value['value']
            require(debt == after['obligations'][value['id']], 'Quint debt effect/post disagreement')
            lines.append(dict(kind=kind, id=value['id'], principal=debt['principal'], accrued=debt['accrued'],
                              outstanding=debt['outstanding'], status=debt['status']['tag']))
        elif kind == 'UseAllowance':
            lines.append(dict(kind=kind, **value))
        elif kind == 'UseReplay':
            lines.append(dict(kind=kind, key=value))
        elif kind == 'AdvanceHead':
            lines.append(dict(kind=kind, predecessor=head(value['before'], 'Quint'), successor=head(value['after'], 'Quint')))
        else:
            raise ValueError(f'Unsupported Quint effect: {kind}')
    return dict(pre=pre, effects=lines, post=post)


def ts_state(state):
    require((state['core'], state['domain'], state['asset']) == ('moriarty-core/5', 'D', 'A'), 'TS core/domain/asset')
    return dict(balances=unique_map((x['account'], x['amount']) for x in state['balances']),
                allowances=unique_map((x['owner'], dict(remaining=x['remaining'], spent=x['spent']))
                                      for x in state['allowances']),
                obligations=unique_map((x['id'], {k: v for k, v in x.items() if k != 'id'})
                                       for x in state['obligations']),
                head=head(state['head'], 'TypeScript'), round=state['round'],
                consumedReplay=sorted(json.loads(x) for x in state['consumedReplay']),
                workRemaining=state['workRemaining'], workSpent=state['workSpent'])


def project_ts(row):
    result = row['result']
    require(result['status'] == 'PreparedUnqualified', 'TS must remain unqualified')
    require(result['requiredPremises'] == ['canonical-intent-signature', 'snapshot-to-head',
            'head-extension', 'atomic-ledger-compare-and-consume'], 'TS premise list')
    require(result['preHead'] == row['pre']['head'], 'TS pre-head')
    effects = []
    for line in result['effects']:
        line = dict(line)
        if line['kind'] == 'UseReplay':
            line['key'] = json.loads(line['key'])
        elif line['kind'] == 'AdvanceHead':
            line['predecessor'], line['successor'] = head(line['predecessor'], 'TypeScript'), head(line['successor'], 'TypeScript')
        effects.append(line)
    return dict(pre=ts_state(row['pre']), effects=effects, post=ts_state(result['candidatePost']))


def compare(k_rows, traces, ts_rows):
    k = unique_map((x['case'], x) for x in k_rows)
    ts = unique_map((x['case'], x) for x in ts_rows)
    require(k.keys() == ts.keys() == CASES.keys() == traces.keys(), 'Exact three-case footprint required')
    results = []
    for name in CASES:
        kp, qp, tp = project_k(k[name]), project_quint(traces[name], name), project_ts(ts[name])
        for part in ('pre', 'effects', 'post'):
            require(kp[part] == qp[part], f'{name}: K/Quint {part} mismatch')
            require(kp[part] == tp[part], f'{name}: K/TypeScript {part} mismatch')
        results.append(dict(case=name, equal=True, projection=kp))
    return results


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--k-receipt', type=Path, default=K_RECEIPT)
    parser.add_argument('--trace-dir', type=Path, default=HERE)
    options = parser.parse_args()
    paths = {name: options.trace_dir / f'quint_{test}_{index}.itf.json'
             for index, (name, test) in enumerate(CASES.items())}
    ts_run = subprocess.run(['node', str(HERE / 'direct-core.mjs')], cwd=ROOT,
                            capture_output=True, text=True, check=True)
    cases = compare(json.loads(options.k_receipt.read_text()),
                    {name: json.loads(path.read_text()) for name, path in paths.items()},
                    json.loads(ts_run.stdout))
    inputs = [options.k_receipt, *paths.values(), HERE / 'direct-core.mjs', HERE / 'compare.py',
              ROOT / 'experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts',
              ROOT / 'experiments/moriarty-language/formal/quint/mil4/s0.qnt',
              ROOT / 'experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt']
    print(json.dumps(dict(scope='three-fixed-local-positives', typescriptStatus='PreparedUnqualified',
                         normalization=dict(kAccountsByCase=K_ACCOUNT_MAPS, headsBySource=HEAD_MAPS,
                                            headSourceRepresentations={'K': 'quoted opaque strings',
                                                                       'TypeScript': 'plain opaque strings',
                                                                       'Quint': 'ITF #bigint integers'}),
                         roundSource='K: recorded submission context; Quint and TypeScript: observed state fields',
                         cases=cases, inputSha256={str(p.relative_to(ROOT)): sha(p) for p in inputs},
                         directCoreCommand=['node', 'experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs']),
                     indent=2, sort_keys=True))


if __name__ == '__main__':
    main()
