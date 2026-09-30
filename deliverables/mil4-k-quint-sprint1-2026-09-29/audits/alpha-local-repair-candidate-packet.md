Independent read-only audit of executable finite alpha_local comparator. Review ONLY exact embedded bytes. Requested reviewer Grok 4.7 xhigh, independent from GPT-6.1 Sol high audit. Do not use tools, skills, delegation, external pages or workspace files. The initial GPT audit found namespace-collision and numeric-type-erasure malformed-input acceptance; the repaired GPT audit confirmed those three reproducers reject, 12/12 tests pass and three recorded positives still match. A residual P3 noted ordinary ITF record and #tup wrappers can project to the same financial value; RESULT now narrows its claim to rejected mismatch, unknown case, namespace collision, and malformed numeric field. Check source/evidence consistency, exact K observedOut/stdout (not expectedOut), Quint ITF pre/effects/post, TypeScript direct Core, alias/round mapping, mutation assertions and remaining high/medium defects. Give finite three-positive adequacy and Sprint1/general-alpha verdicts separately. This is LOCAL unqualified evidence, not ledger correspondence.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/RESULT.md",
    "bytes": 10849,
    "sha256": "85e07467fa412eb86f3569dd94c0edd5c6b9883939aab415f3d963b7f750e497"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/compare.py",
    "bytes": 17496,
    "sha256": "1c7337b705ce929e3e6c1401638b2e721d084138a6b756bf9ffde7f01630365b"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs",
    "bytes": 4277,
    "sha256": "df6fdc938eeb0c36b349eccee26efa9159cd8f6a0901145522774be3176f7063"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py",
    "bytes": 6485,
    "sha256": "bc137c428fbd3b191ac5b48df0d9dfb1c108335ee35d71b6f6cbdd8c4505fed9"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/repair-red-command-results.json",
    "bytes": 2782,
    "sha256": "702688a519e41ff07d2d912c26e58ca7388ca3be83003fa049b6e69c784a62d6"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/repair-final-command-results.json",
    "bytes": 38748,
    "sha256": "6d2b79585634736da85d53daabf02c8c7cf43579ec95e1a72b968542bd612201"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/repair-final-artifact-sha256.json",
    "bytes": 1292,
    "sha256": "15fa98c0f7cd0c1b7cd1f25687961347e3b1c7cc18f90d7cb59ef112ce9cebc9"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/quint_transferCommonTest_0.itf.json",
    "bytes": 6952,
    "sha256": "499c09eefbdc8609c12e36695b2a7cbc4a8781593c6b81dc93321ba1a8658ade"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/quint_repayThirtyCommonTest_1.itf.json",
    "bytes": 7601,
    "sha256": "0b376280a750730724c8a4615669fd8d11a2e90680b69087d045d90fdc5f94b3"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/alpha-local/quint_repayNearBoundCommonTest_2.itf.json",
    "bytes": 8262,
    "sha256": "3f7809719aa636d08ddabeb4c3bc6120ebafce62292d1cdd702b6192ede03589"
  },
  {
    "path": "experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json",
    "bytes": 23079,
    "sha256": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/s0.qnt",
    "bytes": 21207,
    "sha256": "9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt",
    "bytes": 8604,
    "sha256": "b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
    "bytes": 17917,
    "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
  }
]
```

## experiments/moriarty-language/formal/mil4/alpha-local/RESULT.md

```text
# Executable finite α_local comparison

**Experiment observation, 2026-09-30:** `T-10-1`, `R-30`, and `R-near-bound`
have equal exact projected pre-states, ordered effect vectors, and post-states
across the archived K observations, newly captured Quint ITF traces, and fresh
TypeScript direct Core preparations. This is a three-case local observation
comparison. It does not close a sprint or establish general refinement.

From the checkout root, run the comparator without writing any files:

```sh
python3 experiments/moriarty-language/formal/mil4/alpha-local/compare.py
```

It exits 0 and prints JSON containing `scope: "three-fixed-local-positives"`,
`typescriptStatus: "PreparedUnqualified"`, the three `equal: true` case records,
their full exact projections, and SHA256 values for the inputs. A mismatch,
unknown case, variable-namespace collision, or malformed numeric field exits
nonzero. The parser does not require every ITF collection wrapper to use its
original encoding if the decoded projected value is the same. The exact stdout,
stderr, argv, and exit codes are in [command-results.json](command-results.json).
The current comparator after independent review and repair is recorded in
[repair-final-command-results.json](repair-final-command-results.json).

| Case | Projected resulting balances | Resulting allowance | Resulting debt |
| --- | --- | --- | --- |
| T-10-1 | O89, R10, F1 | Only O: remaining0, spent11 | Absent |
| R-30 | O70, C30 | Only O: remaining70, spent30 | L: principal980, accrued0, outstanding980, Outstanding |
| R-near-bound | O0, C340282366920938463463374607431768211455 | Only O: remaining0, spent340282366920938463463374607431768211455 | L: principal170141183460469231731687303715884105726, accrued0, outstanding170141183460469231731687303715884105726, Outstanding |

All resulting heads map to 1, work remaining/spent are 0/1,
and exactly the corresponding `(D,O,case)` replay key is consumed. Full
pre-state cells and ordered effect lines are retained in the JSON output.
Quint and TypeScript observe round 0 in both state fields. K's `state`
constructor has no round field: its projected `round` is the contextual round
0 from the recorded submission request, carried into both projections. This
does not establish that K independently published or preserved a post-state
round. The comparator output explicitly identifies this distinction.

## Inputs and projection

- K: read-only [s1b-common-results.json](../../k/mil4/corpus/s1b-common-results.json).
  The parser decodes each `observedOut` constructor tree and checks that it
  equals the captured stdout `<out>` cell. It does not use `expectedOut`,
  `matched`, or `externalPreserved` as an oracle. The submitted pre-state and
  effect vector must equal the observed pre-state and effects. Success,
  terminal phase, no duty, diagnostic work zero, source/Core version, domain,
  settlement asset, nonce and round are checked. No K execution is rerun here.
- Quint: the three new `quint_*_*.itf.json` files, generated by the command
  below. The last two trace states supply the signed pre-state and committed
  post-state. The adapter requires a passing trace, commit count 0→1,
  initially empty effects, and the same singleton signed-intent map before
  and after the commit. Exact ITF `#bigint`, map, set, tuple and variant values
  are decoded directly.
- TypeScript: [direct-core.mjs](direct-core.mjs) calls `prepareMil4S0` from the
  existing Core/5 module with separately written literal inputs and submitted
  effects. It supplies the optional local stipulation tuple and requested
  terminal outcome. It checks input immutability and requires every result
  to remain `PreparedUnqualified` with its original four required premises.
  It does not invoke Source/6 lowering.

K repayment account P maps to O everywhere in financial cells, effects and
replay keys. Other account identifiers are unchanged. K and TypeScript heads
h0/h1 map to exact decimal strings 0/1; other heads are outside this finite
map. Numeric values remain exact decimal strings, including UInt128 endpoints.
K integer tokens and ITF `#bigint` values keep distinct numeric type markers
until numeric fields are validated. Quoted K numbers and plain Quint numeric
strings reject. Quint states must contain exactly the twelve names under
`s0_common_witnesses::s0::`; a foreign namespace or extra variable rejects
before projection. Only validated numeric values are compared as decimal
strings in the emitted JSON.
Balances, allowances and obligations retain complete key sets, distinguishing
an absent row from a present zero row. Replay sets are sorted for comparison;
effect order is preserved. Quint's single settlement asset A annotates its
assetless debit/credit lines. Allowance lines compare owner and gross amount,
with K's asset checked separately. SetObligation effects compare the shared
write fields; full debtor, creditor and asset identities remain in pre/post
obligation cells. These are explicit representation choices.

## Executed checks

The command/output excerpt below records the initial candidate. Its receipt
and [artifact-sha256.json](artifact-sha256.json) are historical captures of
those initial bytes; current hashes are in
[repair-final-artifact-sha256.json](repair-final-artifact-sha256.json).

```text
$ quint --version
0.32.0
$ node --version
v24.21.0
$ python3 --version
Python 3.14.4

$ quint test experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt --main s0_common_witnesses --match '.*CommonTest' --seed 0x5 --backend typescript --out-itf 'experiments/moriarty-language/formal/mil4/alpha-local/quint_{test}_{seq}.itf.json'

  s0_common_witnesses
    ok transferCommonTest passed 1 test(s)
    ok repayThirtyCommonTest passed 1 test(s)
    ok repayNearBoundCommonTest passed 1 test(s)

  3 passing (42ms)

$ python3 experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py
.........
----------------------------------------------------------------------
Ran 9 tests in 0.369s

OK
```

Every recorded command exited 0. The recorded check environment sets
`PYTHONDONTWRITEBYTECODE=1`. The comparator invokes Node internally, with its
output checked before constructing the result.

The independently specified literal checks in [test_compare.py](test_compare.py)
check the transfer post-state, repayment creditor/debt cells, and exact
UInt128 endpoints. Deliberate mutations establish that changing a coherent K
stdout/observed balance, fabricating a zero Quint allowance row, reordering
effects, or changing a TypeScript UInt128 endpoint by one causes comparison
failure. Poisoning the K expected-output and match flags leaves the result
unchanged, establishing that those fields are not the comparison oracle.
Trailing K terms and floating-point ITF values reject. These checks were
written by the implementation author; they are not an independent provider
audit. The initial executable contract failed because `compare.py` did not
yet exist, before implementation.

The run recorded before/after SHA256 values for 86 existing K, Quint, wire,
signature and TypeScript files. They were unchanged. All new files for this
work are under this directory. Existing S1B and W-D2 receipts were not
regenerated or rewritten.

## Independent review and repair

The parent reported an independent GPT-6.1 Sol high audit that passed the
three recorded positives but found two malformed-input acceptance paths.
Suffix-only Quint state names allowed a foreign `balances` variable to mask
a changed balance. Erasing numeric tags allowed a quoted K numeric token or
plain Quint numeric string to compare equal to a valid integer. This note
records the parent's audit report; the provider receipt is managed separately
by the parent. It does not assert approval of the repaired candidate.

Three targeted tests reproduced those failures before the decoder changes.
[repair-red-command-results.json](repair-red-command-results.json) captures
the exact command, candidate hashes, and `12 tests / 3 failures` output.
After repair, the same command passes all 12 tests, and `compare.py` still
reports the three original equal projections. Both commands, exact outputs,
source hashes, and unchanged protected-input checks are recorded in
[repair-final-command-results.json](repair-final-command-results.json).

```sh
PYTHONDONTWRITEBYTECODE=1 python3 experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py
python3 experiments/moriarty-language/formal/mil4/alpha-local/compare.py
```

The repaired adapter validates full Quint variable names and numeric types
before normalization. The original receipts and three ITF captures remain
unchanged by the repair. Fresh review of the repaired bytes remains pending.

**Repository observation:** before capturing repaired execution, the current
Core/5 module, its Source/6 test file, and Quint's MIL/4 README had different
SHA256 values from the initial capture. This adapter task did not edit those
files. The new receipt
records both initial and current hashes, and reruns direct Core preparation
against the current module. Existing K files, executable Quint inputs and
captured receipts, and wire/signature files match their original hashes.
Stability during the repaired run is recorded
separately from changes between the two captures.
The intermediate [repair-green-command-results.json](repair-green-command-results.json)
and [repair-artifact-sha256.json](repair-artifact-sha256.json) remain historical
captures; the final repair receipt and manifest include this corrected note.

## Limits

This relation covers exactly the shared financial pre/effect/post projection
of these three positives. It excludes full cross-language signed-intent
equality, policy/evidence/source metadata, signature bytes, and external
authentication truth. It does not compare hostile judgments or codes. K's
terminal/no-duty/diagnostic observation is checked for domain membership;
Quint's ordinary ITF states do not carry a full Decision or retained-outcome
record, so no cross-language equality for those fields is asserted.

K outputs are historical captured local executions, rather than fresh K
runs against current sources. Quint traces and TypeScript results are fresh
executions. The existing Quint fixture supplies literal effects to its
commit action; it checks those effects against preparation, but this
comparison is not a separate implementation of that action. Signature,
snapshot, successor, native qualification and ledger atomicity are stipulated
premises in the local experiments. TypeScript explicitly remains unqualified.
No universal refinement proof, exhaustive model check, canonical signed-wire
freeze, native proof, financial ledger admission or ledger-committed α domain
is established. General α remains outside this experiment.

```

## experiments/moriarty-language/formal/mil4/alpha-local/compare.py

```text
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


def head(value):
    require(value in ('h0', 'h1', '0', '1'), f'Head outside finite map: {value}')
    return {'h0': '0', 'h1': '1', '0': '0', '1': '1'}[value]


def account(value, name):
    return 'O' if name != 'T-10-1' and value == 'P' else value


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
                obligations=obligations, head=head(args(a[5], 'head', 1)[0]),
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
            lines.append(dict(kind='AdvanceHead', predecessor=head(args(before, 'head', 1)[0]),
                              successor=head(args(after, 'head', 1)[0])))
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
    if '#bigint' in value:
        require(set(value) == {'#bigint'} and re.fullmatch(r'0|[1-9][0-9]*', value['#bigint']), 'ITF bigint')
        return QuintInteger(value['#bigint'])
    if '#map' in value:
        return unique_map((decode_itf(k), decode_itf(v)) for k, v in value['#map'])
    if '#set' in value or '#tup' in value:
        return decode_itf(value.get('#set', value.get('#tup')))
    return {k: decode_itf(v) for k, v in value.items() if k != '#meta'}


def q_state(raw):
    require(set(raw) - {'#meta'} == {Q_PREFIX + field for field in Q_FIELDS},
            'Quint state namespace/footprint')
    values = {field: decode_itf(raw[Q_PREFIX + field]) for field in Q_FIELDS}
    def numeric(value, field):
        require(isinstance(value, QuintInteger), f'Quint integer type: {field}')
    for field in ['round', 'currentHead', 'committedCount', 'workRemaining', 'workSpent']:
        numeric(values[field], field)
    for field in ['balances', 'allowanceRemaining', 'allowanceSpent']:
        require(isinstance(values[field], dict), f'Quint map type: {field}')
        for key, value in values[field].items():
            require(type(key) is str, f'Quint map key type: {field}')
            numeric(value, field + '/' + key)
    for key, intent in values['signed'].items():
        for field in ['preHead', 'validFrom', 'validThrough', 'grossCap', 'feeCap',
                      'netFloor', 'actionAmount', 'actionFee', 'conversionMantissa', 'conversionScale']:
            numeric(intent[field], 'signed/' + key + '/' + field)
    for line in values['lastEffects']:
        kind, value = line['tag'], line['value']
        if kind in ('Debit', 'Credit', 'UseAllowance'):
            numeric(value['amount'], 'effect/' + kind + '/amount')
        elif kind == 'AdvanceHead':
            numeric(value['before'], 'effect/head/before')
            numeric(value['after'], 'effect/head/after')
        elif kind == 'SetObligation':
            for field in ['principal', 'accrued', 'outstanding']:
                numeric(value['value'][field], 'effect/debt/' + field)
    require(values['allowanceRemaining'].keys() == values['allowanceSpent'].keys(), 'Quint allowance footprint')
    debts = {}
    for oid, debt in values['obligations'].items():
        for field in ['principal', 'accrued', 'outstanding']:
            numeric(debt[field], 'obligation/' + oid + '/' + field)
        status = debt['status']
        require(status['tag'] in ('Outstanding', 'Settled') and status['value'] == [], 'Quint debt status')
        debts[oid] = {**debt, 'status': status['tag']}
    result = dict(balances=values['balances'],
                  allowances={owner: dict(remaining=n, spent=values['allowanceSpent'][owner])
                              for owner, n in values['allowanceRemaining'].items()},
                  obligations=debts, head=head(values['currentHead']), round=values['round'],
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
            lines.append(dict(kind=kind, predecessor=head(value['before']), successor=head(value['after'])))
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
                head=head(state['head']), round=state['round'],
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
            line['predecessor'], line['successor'] = head(line['predecessor']), head(line['successor'])
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
                         roundSource='K: recorded submission context; Quint and TypeScript: observed state fields',
                         cases=cases, inputSha256={str(p.relative_to(ROOT)): sha(p) for p in inputs},
                         directCoreCommand=['node', 'experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs']),
                     indent=2, sort_keys=True))


if __name__ == '__main__':
    main()

```

## experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs

```text
/** Literal independent inputs for the three finite local comparisons. */
import { prepareMil4S0 } from '../../../src/successor/mil4-s0-core-v5.ts';

const U = '340282366920938463463374607431768211455';
const UminusOne = '340282366920938463463374607431768211454';
const S = '170141183460469231731687303715884105727';
const SminusOne = '170141183460469231731687303715884105726';
const base = {
  core: 'moriarty-core/5', domain: 'D', asset: 'A', head: 'h0', round: '0',
  workRemaining: '1', workSpent: '0', consumedReplay: [],
};
const commonIntent = {
  version: 'moriarty-intent/3', core: 'moriarty-core/5',
  sourceProfile: 'moriarty-financial-agreement-source/6',
  sourceHash: 'local-source-label', policyDigest: 'p', keyRef: 'local-key-label',
  domain: 'D', asset: 'A', signer: 'O', preHead: 'h0',
  notBefore: '0', notAfter: '10', netFloor: '0',
};
const obligation = (principal, accrued, outstanding) => ({
  id: 'L', debtor: 'O', creditor: 'C', asset: 'A', principal, accrued, outstanding,
  status: 'Outstanding',
});
const tail = (nonce, amount) => [
  { kind: 'UseAllowance', owner: 'O', amount },
  { kind: 'UseReplay', key: JSON.stringify(['D', 'O', nonce]) },
  { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
];
const fixtures = [
  {
    case: 'T-10-1', state: { ...base,
      balances: [{ account: 'O', amount: '100' }, { account: 'R', amount: '0' },
        { account: 'F', amount: '0' }],
      allowances: [{ owner: 'O', remaining: '11', spent: '0' }], obligations: [],
    }, intent: { ...commonIntent, kind: 'Transfer', programId: 'TransferLiteralFee',
      nonce: 'T-10-1', recipient: 'R', feeRecipient: 'F', grossCap: '11',
      feeCap: '1', amount: '10', fee: '1',
    }, effects: [
      { kind: 'Debit', account: 'O', asset: 'A', amount: '11' },
      { kind: 'Credit', account: 'R', asset: 'A', amount: '10' },
      { kind: 'Credit', account: 'F', asset: 'A', amount: '1' }, ...tail('T-10-1', '11'),
    ],
  },
  {
    case: 'R-30', state: { ...base,
      balances: [{ account: 'O', amount: '100' }, { account: 'C', amount: '0' }],
      allowances: [{ owner: 'O', remaining: '100', spent: '0' }],
      obligations: [obligation('1000', '10', '1010')],
    }, intent: { ...commonIntent, kind: 'Repay', programId: 'RepayAccrualFirst',
      nonce: 'R-30', obligationId: 'L', grossCap: '30', feeCap: '0', amount: '30',
    }, effects: [
      { kind: 'Debit', account: 'O', asset: 'A', amount: '30' },
      { kind: 'Credit', account: 'C', asset: 'A', amount: '30' },
      { kind: 'SetObligation', id: 'L', principal: '980', accrued: '0',
        outstanding: '980', status: 'Outstanding' }, ...tail('R-30', '30'),
    ],
  },
  {
    case: 'R-near-bound', state: { ...base,
      balances: [{ account: 'O', amount: '1' }, { account: 'C', amount: UminusOne }],
      allowances: [{ owner: 'O', remaining: '1', spent: UminusOne }],
      obligations: [obligation(SminusOne, '1', S)],
    }, intent: { ...commonIntent, kind: 'Repay', programId: 'RepayAccrualFirst',
      nonce: 'R-near-bound', obligationId: 'L', grossCap: '1', feeCap: '0', amount: '1',
    }, effects: [
      { kind: 'Debit', account: 'O', asset: 'A', amount: '1' },
      { kind: 'Credit', account: 'C', asset: 'A', amount: '1' },
      { kind: 'SetObligation', id: 'L', principal: SminusOne, accrued: '0',
        outstanding: SminusOne, status: 'Outstanding' }, ...tail('R-near-bound', '1'),
    ],
  },
];
const outcome = { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [] };
const observations = fixtures.map(({ case: name, state, intent, effects }) => {
  const before = JSON.stringify({ state, intent, effects });
  const result = prepareMil4S0(state, intent, effects, 'h1', outcome,
    { state, intent, round: '0', expectedSuccessor: 'h1', requestedOutcome: outcome });
  if (JSON.stringify({ state, intent, effects }) !== before) throw Error('Input mutation');
  if (result.status !== 'PreparedUnqualified') throw Error(`${name}: ${JSON.stringify(result)}`);
  return { case: name, pre: state, intent, result };
});
// U is a literal independent expected endpoint, not an arithmetic oracle.
if (observations[2].result.candidatePost.balances[1].amount !== U) throw Error('UInt128 endpoint');
console.log(JSON.stringify(observations));

```

## experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py

```text
"""The subprocess contract is tested before the comparator exists."""
import json
import copy
from pathlib import Path
import subprocess
import sys
import unittest
import compare as adapter

HERE = Path(__file__).resolve().parent


class ExecutableContract(unittest.TestCase):
    def test_three_observations_match(self):
        run = subprocess.run([sys.executable, str(HERE / 'compare.py')],
                             capture_output=True, text=True)
        self.assertEqual(run.returncode, 0, run.stderr)
        result = json.loads(run.stdout)
        self.assertEqual(result['scope'], 'three-fixed-local-positives')
        self.assertEqual([r['case'] for r in result['cases']],
                         ['T-10-1', 'R-30', 'R-near-bound'])
        self.assertTrue(all(r['equal'] for r in result['cases']))
        self.assertEqual(result['typescriptStatus'], 'PreparedUnqualified')


class IndependentChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.k = json.loads(adapter.K_RECEIPT.read_text())
        cls.q = {name: json.loads((HERE / f'quint_{test}_{index}.itf.json').read_text())
                 for index, (name, test) in enumerate(adapter.CASES.items())}
        run = subprocess.run(['node', str(HERE / 'direct-core.mjs')], check=True,
                             capture_output=True, text=True)
        cls.ts = json.loads(run.stdout)

    def test_independent_literal_transfer_post(self):
        p = adapter.project_quint(self.q['T-10-1'], 'T-10-1')
        self.assertEqual(p['post'], {
            'balances': {'O': '89', 'R': '10', 'F': '1'},
            'allowances': {'O': {'remaining': '0', 'spent': '11'}},
            'obligations': {}, 'head': '1', 'round': '0',
            'consumedReplay': [['D', 'O', 'T-10-1']],
            'workRemaining': '0', 'workSpent': '1',
        })
        self.assertEqual([line['kind'] for line in p['effects']],
                         ['Debit', 'Credit', 'Credit', 'UseAllowance', 'UseReplay', 'AdvanceHead'])

    def test_independent_literal_repayment_and_bound(self):
        thirty = adapter.project_k(self.k[1])
        self.assertEqual(thirty['post']['balances'], {'O': '70', 'C': '30'})
        self.assertEqual(thirty['post']['obligations'], {'L': {
            'debtor': 'O', 'creditor': 'C', 'asset': 'A', 'principal': '980',
            'accrued': '0', 'outstanding': '980', 'status': 'Outstanding',
        }})
        near = adapter.project_k(self.k[2])
        self.assertEqual(near['post']['balances']['C'], '340282366920938463463374607431768211455')
        self.assertEqual(near['post']['allowances']['O']['spent'], '340282366920938463463374607431768211455')
        self.assertEqual(near['post']['obligations']['L']['principal'], '170141183460469231731687303715884105726')

    def test_expected_receipt_and_match_flags_are_not_oracles(self):
        rows = copy.deepcopy(self.k)
        for row in rows:
            row['expectedOut'] = 'rejected()'
            row['matched'] = False
            row['externalPreserved'] = False
        self.assertEqual(adapter.compare(rows, self.q, self.ts),
                         adapter.compare(self.k, self.q, self.ts))

    def test_coherent_k_output_mutation_fails_comparison(self):
        rows = copy.deepcopy(self.k)
        for field in ['observedOut', 'stdout']:
            rows[0][field] = rows[0][field].replace('"A" , 89', '"A" , 88')
        with self.assertRaisesRegex(ValueError, 'K/Quint post mismatch'):
            adapter.compare(rows, self.q, self.ts)

    def test_quint_fabricated_zero_allowance_fails(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        for field in ['allowanceRemaining', 'allowanceSpent']:
            key = next(k for k in post if k.endswith('::' + field))
            post[key]['#map'].append(['R', {'#bigint': '0'}])
        with self.assertRaisesRegex(ValueError, 'K/Quint post mismatch'):
            adapter.compare(self.k, traces, self.ts)

    def test_effect_order_mutation_fails(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::lastEffects'))
        post[key][0], post[key][1] = post[key][1], post[key][0]
        with self.assertRaisesRegex(ValueError, 'K/Quint effects mismatch'):
            adapter.compare(self.k, traces, self.ts)

    def test_one_unit_near_bound_mutation_fails(self):
        rows = copy.deepcopy(self.ts)
        rows[2]['result']['candidatePost']['balances'][1]['amount'] = '340282366920938463463374607431768211454'
        with self.assertRaisesRegex(ValueError, 'K/TypeScript post mismatch'):
            adapter.compare(self.k, self.q, rows)

    def test_trailing_k_term_and_float_itf_fail_closed(self):
        with self.assertRaisesRegex(ValueError, 'Trailing K term'):
            adapter.parse_term('noEffects()noEffects()')
        with self.assertRaisesRegex(ValueError, 'exact #bigint'):
            adapter.decode_itf(340282366920938463463374607431768211455.0)

    def test_quint_foreign_suffix_cannot_mask_changed_balance(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::balances'))
        original = copy.deepcopy(post[key])
        for owner, amount in post[key]['#map']:
            if owner == 'O':
                amount['#bigint'] = '88'
        post['foreign::balances'] = original
        with self.assertRaisesRegex(ValueError, 'Quint state namespace/footprint'):
            adapter.compare(self.k, traces, self.ts)

    def test_k_quoted_integer_is_not_numeric_observation(self):
        rows = copy.deepcopy(self.k)
        for field in ['observedOut', 'stdout']:
            rows[0][field] = rows[0][field].replace('"A" , 89', '"A" , "89"')
        with self.assertRaisesRegex(ValueError, 'K integer type'):
            adapter.compare(rows, self.q, self.ts)

    def test_quint_plain_numeric_string_is_not_bigint(self):
        traces = copy.deepcopy(self.q)
        post = traces['T-10-1']['states'][-1]
        key = next(k for k in post if k.endswith('::balances'))
        post[key]['#map'] = [[owner, '89' if owner == 'O' else amount]
                            for owner, amount in post[key]['#map']]
        with self.assertRaisesRegex(ValueError, 'Quint integer type'):
            adapter.compare(self.k, traces, self.ts)


if __name__ == '__main__':
    unittest.main()

```

## experiments/moriarty-language/formal/mil4/alpha-local/repair-red-command-results.json

```text
{
  "recordedAt": "2026-09-30T07:37:28.195001+00:00",
  "argv": [
    "python3",
    "experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py"
  ],
  "exitCode": 1,
  "stdout": "",
  "stderr": "......F..FF.\n======================================================================\nFAIL: test_k_quoted_integer_is_not_numeric_observation (__main__.IndependentChecks.test_k_quoted_integer_is_not_numeric_observation)\n----------------------------------------------------------------------\nTraceback (most recent call last):\n  File \"/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py\", line 121, in test_k_quoted_integer_is_not_numeric_observation\n    with self.assertRaisesRegex(ValueError, 'K integer type'):\n         ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nAssertionError: ValueError not raised\n\n======================================================================\nFAIL: test_quint_foreign_suffix_cannot_mask_changed_balance (__main__.IndependentChecks.test_quint_foreign_suffix_cannot_mask_changed_balance)\n----------------------------------------------------------------------\nTraceback (most recent call last):\n  File \"/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py\", line 114, in test_quint_foreign_suffix_cannot_mask_changed_balance\n    with self.assertRaisesRegex(ValueError, 'Quint state namespace/footprint'):\n         ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nAssertionError: ValueError not raised\n\n======================================================================\nFAIL: test_quint_plain_numeric_string_is_not_bigint (__main__.IndependentChecks.test_quint_plain_numeric_string_is_not_bigint)\n----------------------------------------------------------------------\nTraceback (most recent call last):\n  File \"/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py\", line 130, in test_quint_plain_numeric_string_is_not_bigint\n    with self.assertRaisesRegex(ValueError, 'Quint integer type'):\n         ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nAssertionError: ValueError not raised\n\n----------------------------------------------------------------------\nRan 12 tests in 0.269s\n\nFAILED (failures=3)\n",
  "candidateSha256": {
    "compare.py": "aafbe605c28988493fbb05b5baf1b99b7959d408457e1d2c344e62e7ef1af244",
    "test_compare.py": "bc137c428fbd3b191ac5b48df0d9dfb1c108335ee35d71b6f6cbdd8c4505fed9"
  },
  "purpose": "Reproduce independent audit malformed-input findings before repair; 12 tests, 3 expected failures."
}

```

## experiments/moriarty-language/formal/mil4/alpha-local/repair-final-command-results.json

```text
{
  "recordedAt": "2026-09-30T07:43:13.594554+00:00",
  "cwd": "/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929",
  "commands": [
    {
      "argv": [
        "python3",
        "experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": "............\n----------------------------------------------------------------------\nRan 12 tests in 0.558s\n\nOK\n"
    },
    {
      "argv": [
        "python3",
        "experiments/moriarty-language/formal/mil4/alpha-local/compare.py"
      ],
      "exitCode": 0,
      "stdout": "{\n  \"cases\": [\n    {\n      \"case\": \"T-10-1\",\n      \"equal\": true,\n      \"projection\": {\n        \"effects\": [\n          {\n            \"account\": \"O\",\n            \"amount\": \"11\",\n            \"asset\": \"A\",\n            \"kind\": \"Debit\"\n          },\n          {\n            \"account\": \"R\",\n            \"amount\": \"10\",\n            \"asset\": \"A\",\n            \"kind\": \"Credit\"\n          },\n          {\n            \"account\": \"F\",\n            \"amount\": \"1\",\n            \"asset\": \"A\",\n            \"kind\": \"Credit\"\n          },\n          {\n            \"amount\": \"11\",\n            \"kind\": \"UseAllowance\",\n            \"owner\": \"O\"\n          },\n          {\n            \"key\": [\n              \"D\",\n              \"O\",\n              \"T-10-1\"\n            ],\n            \"kind\": \"UseReplay\"\n          },\n          {\n            \"kind\": \"AdvanceHead\",\n            \"predecessor\": \"0\",\n            \"successor\": \"1\"\n          }\n        ],\n        \"post\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"0\",\n              \"spent\": \"11\"\n            }\n          },\n          \"balances\": {\n            \"F\": \"1\",\n            \"O\": \"89\",\n            \"R\": \"10\"\n          },\n          \"consumedReplay\": [\n            [\n              \"D\",\n              \"O\",\n              \"T-10-1\"\n            ]\n          ],\n          \"head\": \"1\",\n          \"obligations\": {},\n          \"round\": \"0\",\n          \"workRemaining\": \"0\",\n          \"workSpent\": \"1\"\n        },\n        \"pre\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"11\",\n              \"spent\": \"0\"\n            }\n          },\n          \"balances\": {\n            \"F\": \"0\",\n            \"O\": \"100\",\n            \"R\": \"0\"\n          },\n          \"consumedReplay\": [],\n          \"head\": \"0\",\n          \"obligations\": {},\n          \"round\": \"0\",\n          \"workRemaining\": \"1\",\n          \"workSpent\": \"0\"\n        }\n      }\n    },\n    {\n      \"case\": \"R-30\",\n      \"equal\": true,\n      \"projection\": {\n        \"effects\": [\n          {\n            \"account\": \"O\",\n            \"amount\": \"30\",\n            \"asset\": \"A\",\n            \"kind\": \"Debit\"\n          },\n          {\n            \"account\": \"C\",\n            \"amount\": \"30\",\n            \"asset\": \"A\",\n            \"kind\": \"Credit\"\n          },\n          {\n            \"accrued\": \"0\",\n            \"id\": \"L\",\n            \"kind\": \"SetObligation\",\n            \"outstanding\": \"980\",\n            \"principal\": \"980\",\n            \"status\": \"Outstanding\"\n          },\n          {\n            \"amount\": \"30\",\n            \"kind\": \"UseAllowance\",\n            \"owner\": \"O\"\n          },\n          {\n            \"key\": [\n              \"D\",\n              \"O\",\n              \"R-30\"\n            ],\n            \"kind\": \"UseReplay\"\n          },\n          {\n            \"kind\": \"AdvanceHead\",\n            \"predecessor\": \"0\",\n            \"successor\": \"1\"\n          }\n        ],\n        \"post\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"70\",\n              \"spent\": \"30\"\n            }\n          },\n          \"balances\": {\n            \"C\": \"30\",\n            \"O\": \"70\"\n          },\n          \"consumedReplay\": [\n            [\n              \"D\",\n              \"O\",\n              \"R-30\"\n            ]\n          ],\n          \"head\": \"1\",\n          \"obligations\": {\n            \"L\": {\n              \"accrued\": \"0\",\n              \"asset\": \"A\",\n              \"creditor\": \"C\",\n              \"debtor\": \"O\",\n              \"outstanding\": \"980\",\n              \"principal\": \"980\",\n              \"status\": \"Outstanding\"\n            }\n          },\n          \"round\": \"0\",\n          \"workRemaining\": \"0\",\n          \"workSpent\": \"1\"\n        },\n        \"pre\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"100\",\n              \"spent\": \"0\"\n            }\n          },\n          \"balances\": {\n            \"C\": \"0\",\n            \"O\": \"100\"\n          },\n          \"consumedReplay\": [],\n          \"head\": \"0\",\n          \"obligations\": {\n            \"L\": {\n              \"accrued\": \"10\",\n              \"asset\": \"A\",\n              \"creditor\": \"C\",\n              \"debtor\": \"O\",\n              \"outstanding\": \"1010\",\n              \"principal\": \"1000\",\n              \"status\": \"Outstanding\"\n            }\n          },\n          \"round\": \"0\",\n          \"workRemaining\": \"1\",\n          \"workSpent\": \"0\"\n        }\n      }\n    },\n    {\n      \"case\": \"R-near-bound\",\n      \"equal\": true,\n      \"projection\": {\n        \"effects\": [\n          {\n            \"account\": \"O\",\n            \"amount\": \"1\",\n            \"asset\": \"A\",\n            \"kind\": \"Debit\"\n          },\n          {\n            \"account\": \"C\",\n            \"amount\": \"1\",\n            \"asset\": \"A\",\n            \"kind\": \"Credit\"\n          },\n          {\n            \"accrued\": \"0\",\n            \"id\": \"L\",\n            \"kind\": \"SetObligation\",\n            \"outstanding\": \"170141183460469231731687303715884105726\",\n            \"principal\": \"170141183460469231731687303715884105726\",\n            \"status\": \"Outstanding\"\n          },\n          {\n            \"amount\": \"1\",\n            \"kind\": \"UseAllowance\",\n            \"owner\": \"O\"\n          },\n          {\n            \"key\": [\n              \"D\",\n              \"O\",\n              \"R-near-bound\"\n            ],\n            \"kind\": \"UseReplay\"\n          },\n          {\n            \"kind\": \"AdvanceHead\",\n            \"predecessor\": \"0\",\n            \"successor\": \"1\"\n          }\n        ],\n        \"post\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"0\",\n              \"spent\": \"340282366920938463463374607431768211455\"\n            }\n          },\n          \"balances\": {\n            \"C\": \"340282366920938463463374607431768211455\",\n            \"O\": \"0\"\n          },\n          \"consumedReplay\": [\n            [\n              \"D\",\n              \"O\",\n              \"R-near-bound\"\n            ]\n          ],\n          \"head\": \"1\",\n          \"obligations\": {\n            \"L\": {\n              \"accrued\": \"0\",\n              \"asset\": \"A\",\n              \"creditor\": \"C\",\n              \"debtor\": \"O\",\n              \"outstanding\": \"170141183460469231731687303715884105726\",\n              \"principal\": \"170141183460469231731687303715884105726\",\n              \"status\": \"Outstanding\"\n            }\n          },\n          \"round\": \"0\",\n          \"workRemaining\": \"0\",\n          \"workSpent\": \"1\"\n        },\n        \"pre\": {\n          \"allowances\": {\n            \"O\": {\n              \"remaining\": \"1\",\n              \"spent\": \"340282366920938463463374607431768211454\"\n            }\n          },\n          \"balances\": {\n            \"C\": \"340282366920938463463374607431768211454\",\n            \"O\": \"1\"\n          },\n          \"consumedReplay\": [],\n          \"head\": \"0\",\n          \"obligations\": {\n            \"L\": {\n              \"accrued\": \"1\",\n              \"asset\": \"A\",\n              \"creditor\": \"C\",\n              \"debtor\": \"O\",\n              \"outstanding\": \"170141183460469231731687303715884105727\",\n              \"principal\": \"170141183460469231731687303715884105726\",\n              \"status\": \"Outstanding\"\n            }\n          },\n          \"round\": \"0\",\n          \"workRemaining\": \"1\",\n          \"workSpent\": \"0\"\n        }\n      }\n    }\n  ],\n  \"directCoreCommand\": [\n    \"node\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs\"\n  ],\n  \"inputSha256\": {\n    \"experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json\": \"c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/compare.py\": \"1c7337b705ce929e3e6c1401638b2e721d084138a6b756bf9ffde7f01630365b\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs\": \"df6fdc938eeb0c36b349eccee26efa9159cd8f6a0901145522774be3176f7063\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/quint_repayNearBoundCommonTest_2.itf.json\": \"3f7809719aa636d08ddabeb4c3bc6120ebafce62292d1cdd702b6192ede03589\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/quint_repayThirtyCommonTest_1.itf.json\": \"0b376280a750730724c8a4615669fd8d11a2e90680b69087d045d90fdc5f94b3\",\n    \"experiments/moriarty-language/formal/mil4/alpha-local/quint_transferCommonTest_0.itf.json\": \"499c09eefbdc8609c12e36695b2a7cbc4a8781593c6b81dc93321ba1a8658ade\",\n    \"experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt\": \"b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e\",\n    \"experiments/moriarty-language/formal/quint/mil4/s0.qnt\": \"9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53\",\n    \"experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts\": \"855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda\"\n  },\n  \"roundSource\": \"K: recorded submission context; Quint and TypeScript: observed state fields\",\n  \"scope\": \"three-fixed-local-positives\",\n  \"typescriptStatus\": \"PreparedUnqualified\"\n}\n",
      "stderr": ""
    }
  ],
  "environmentOverride": {
    "PYTHONDONTWRITEBYTECODE": "1"
  },
  "candidateSha256": {
    "experiments/moriarty-language/formal/mil4/alpha-local/compare.py": "1c7337b705ce929e3e6c1401638b2e721d084138a6b756bf9ffde7f01630365b",
    "experiments/moriarty-language/formal/mil4/alpha-local/direct-core.mjs": "df6fdc938eeb0c36b349eccee26efa9159cd8f6a0901145522774be3176f7063",
    "experiments/moriarty-language/formal/mil4/alpha-local/test_compare.py": "bc137c428fbd3b191ac5b48df0d9dfb1c108335ee35d71b6f6cbdd8c4505fed9",
    "experiments/moriarty-language/formal/mil4/alpha-local/RESULT.md": "85e07467fa412eb86f3569dd94c0edd5c6b9883939aab415f3d963b7f750e497",
    "experiments/moriarty-language/formal/mil4/alpha-local/quint_transferCommonTest_0.itf.json": "499c09eefbdc8609c12e36695b2a7cbc4a8781593c6b81dc93321ba1a8658ade",
    "experiments/moriarty-language/formal/mil4/alpha-local/quint_repayThirtyCommonTest_1.itf.json": "0b376280a750730724c8a4615669fd8d11a2e90680b69087d045d90fdc5f94b3",
    "experiments/moriarty-language/formal/mil4/alpha-local/quint_repayNearBoundCommonTest_2.itf.json": "3f7809719aa636d08ddabeb4c3bc6120ebafce62292d1cdd702b6192ede03589"
  },
  "protectedInputsBeforeSha256": {
    "experiments/moriarty-language/formal/k/mil4/README.md": "91ceb57507c6fb2f804b0d2b9c16537ec8d3c053f28902b43ec48ac7eab36c7e",
    "experiments/moriarty-language/formal/k/mil4/s0.k": "e76c168bc6826e13308f19a34f229e5b07b64196a41892a6ac3122890d4c0cc0",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-parser-checked-results.json": "dae74490604f8c7d69f76a5d73f45df1e90685cca541acd278ca1fe71007529b",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-verification.json": "71dd8b6b682c33ff4cf452dbb11456bc78a9c735bd7035fc809d6d9c8c47bad5",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-results.json": "490621eb02e315a29c0f0498ca08bc70985b9e74fba689456c2489331f74ee6f",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-results.json": "0c5e8ede144a0e7aa157dec8981278c868cf1c62a3e3fb84b0f9a10d5c6dc34e",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-checked-results.json": "c6e433f3abbb197a90c2891694aa7c3e4268ea8f5142eb3ab06e09d9422326e7",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-results.json": "97b4a9668737e2852a8570518da6a62f36df996f39ddce619b83d7265ac2dd3a",
    "experiments/moriarty-language/formal/k/mil4/corpus/run_checked.py": "e2a2ac64a3438d8445cadf7e943a010da20d318b3367fe53608468d04389e463",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-semantic-compile.json": "01016057cda1f8ee549321c6520a53118fb1a3044e0cb7e6489f7a1b059dad2f",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-checked-results.json": "f565bc4bfa0308c0a8ff2bc6be43eb79a5617a604f2532b03c9db66e9ff020d7",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-compile.json": "ad135daf6c72894f62ead495294509143371c0106993cb3d2ffce46855d71c84",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-s0.k.txt": "f174aec77085bc4e351931ad52ccc32f360c1cdab6ac0c5271b6863054523551",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-parser-failure.json": "ff59dfae31cfb49b8b1a540775999a1ea3131f8b0a6b57fde8caa4a086489720",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd",
    "experiments/moriarty-language/formal/k/mil4/corpus/common_run.py": "7321573967ab0701a4395379a06381827d42f0f7f2496927a335ef36053d41b6",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-regression-results.json": "9ba861ec1ab85979b2ef7278687914496b845ae90c433ed9afaffc18eb668c24",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-verification.json": "d799ea72d6e1f93be0d23f15e4d22d24a5104724ed909dafa8fe711a5b49f816",
    "experiments/moriarty-language/formal/k/mil4/corpus/common-results.json": "f6c7a024611824320413b01f0960ffeb892b1ba7b7a2d61e224bb7e9e8428694",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-results.json": "7c676c28652cc578a23390aedc7ef0c396057ee3ed608da1c175744185a051c3",
    "experiments/moriarty-language/formal/k/mil4/corpus/checked-results.json": "4ecdddf2a515fd89bc588db4e99a6818f51f0079788a595ef41e06dcf5a08da3",
    "experiments/moriarty-language/formal/k/mil4/corpus/run.py": "ecb9c9d66be9ebb8b4d494fc3aa672555eedc78f341c37999657378d191bd8c7",
    "experiments/moriarty-language/formal/k/mil4/corpus/results.json": "35eb658f4710920d95b2f52cf18668abe2ffd5ae993c7f97b9a36fb58288a379",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-compile.json": "732e3848f239e0dafbcb5a306b9ee81c83d1037c96a419ed202f3d237d45d194",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-positional-final-compile.json": "3b9901d2acdd467330d4dbb6672281c5c6e9d0d3ee814aaff2e6a0c1fb9acb64",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b_run.py": "3649ecfee714b1c94ee1cbf8fcffafc7b745368dac77d15cafe8aa3e9f8702a2",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-compile.json": "80d4f4055801d5f8f164dbfa98c3e4d118a7e43adedaa51f234cdb2bc2d1323e",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-results.json": "e60e3a9ecb0044a2ded9f43320863d4773c22ba5f4e95785054f644b734dd87f",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-compile.json": "2d60c56a4b3a2b5bcdcbf992f3582325f3ef6f307635ed137f39126675ac95ed",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-red-result.json": "02bf27d5db669584924e9896959af00ec12c9d7ae2346799cee2dab62e99c2f7",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-verification.json": "e3f72bba4f552793f8d3e4c0dbea78932d0f3461c0a5cdf43ed1795f86c84f70",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-semantic-red-results.json": "4299c3f78302c28aaabef9895338064c49d48eb56815608970bb4d870742664f",
    "experiments/moriarty-language/formal/k/mil4/corpus/corpus.k": "949dec467bc62379bce1d876d233c0261ba1ec7e8e5e4917436b9f2aa797dbdf",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-regression-results.json": "481cc4f5d806c02e5778e6a412fe8d43f89e9899d5177752efc421e3d19ba49a",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-regression-results.json": "9ba861ec1ab85979b2ef7278687914496b845ae90c433ed9afaffc18eb668c24",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-common-results.json": "544c050206ebb4efdcd5d2deaa329626af4a17bf4578a016533209c42906c2d9",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-final-compile.json": "f527beaf9d1c7c4d35c1fb6f0710809a38365c520cb92df794b998abb42d458c",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-common-results.json": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-compile.json": "9d80b068ddf1f355750fdb3a300e754f721b926c7069ca72a7ddfab91249c293",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/run_checked.cpython-314.pyc": "c5607c4115bd391fcd706b7163e606523172deaafe305391bd69995ed83b9f1a",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/run.cpython-314.pyc": "d39b132b39337a3261b0cb3c1c4a232ef361d1838188559f32e5f537ff048116",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/s1b_run.cpython-314.pyc": "44870de6aca654d4a9bfa62aafcb22c579f0d9be853b093570169ca6f05c2bf7",
    "experiments/moriarty-language/formal/quint/mil4/common-state.md": "5266f82e675228009c98d4cf6ceb092a0922f220b469f1b5a8c9651de866b00e",
    "experiments/moriarty-language/formal/quint/mil4/README.md": "9173e643401047be23dea1f8d502307e439e5e4cc90b115a985583d29f6ffe72",
    "experiments/moriarty-language/formal/quint/mil4/s0.qnt": "9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS-BEFORE-REPAIR.md": "0ab7eaf67f7ebd5e07b8acdbfb3c532707bb1e5755ff1f4a9f7bf17adff6aa83",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-BEFORE-REPAIR.md": "da901bfb7463c2645d76a5100ba97749bde823fbeb21dca9b54036c71bfa8880",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt": "358aa6e5c3d7e19c5ca8750e03597fcf17d21d855b085a18f0c6c456807c7ce1",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-pre-audit-command-results.json": "04d31bf725a9fc17278e5f34880e499041f2c771632fe8cd05d1fa0432c51ef1",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS-AFTER-ORDERING-REPAIR.md": "d192d5a8b59e69fcfd0993f7b17b806d91122d1237d361f2dd0526e1d17f5727",
    "experiments/moriarty-language/formal/quint/mil4/corpus/common-command-results.json": "f26e9747cf8d8cd2e9e0e6ca48176bafc6c3a2e3f1f32562025839f69e873477",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt": "a46335f66e003af41f4200448a8790f85175a4b78a5942bbefa5240aecb6f885",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt": "007b66094455b21467ee27fb744b55706e5a91c7a745523253ecb82cdf297591",
    "experiments/moriarty-language/formal/quint/mil4/corpus/diagnostic-command-results.json": "4134f0fc407acc405ad2075d615d33c0bbddaeab4cf9c923705dcb4c6035e3ca",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-AFTER-ORDERING-REPAIR.md": "5581682f997a1085c18000d32ed16cddf4f478106e4d6aebeccc4765e6fd0cfb",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-command-results.json": "ed69d4edb9a5c293ac24a1968f12a421f7f84414e837c809a720d5625e1c2a1c",
    "experiments/moriarty-language/formal/quint/mil4/corpus/repair-command-results.json": "407427617146acba92bdee536c2ab7edc4868c222a50bdb7edff1fc5257a2403",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-command-results.json": "a9905dc2de48a690c540111e1678878327cfafa7859e6186360e55f8e69dcffe",
    "experiments/moriarty-language/formal/quint/mil4/corpus/S1B-RESULTS.md": "2eb298dcb40cf650a95d11c140191f06d2c6d0808d5271943e5358bfc95db680",
    "experiments/moriarty-language/formal/quint/mil4/corpus/common-before-command-results.json": "82be849fb4ce5a5d558dc7ae114cef4e58bf1984011c67aaf007fbf2d45e059c",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-pre-repair-command-results.json": "8af96f3b19a53cbcbe770f2a70fbbc978fd0af6b5fab6782638a0e95c969ea49",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt": "b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS.md": "68e3609c264516e566f4f333dfca6f22ef4548b09a7ee2da128f800a64ba9322",
    "experiments/moriarty-language/formal/quint/mil4/corpus/COMMON-RESULTS.md": "d339a13c4a4f4d6658abf30f9e1b12c24542108cf18887ab169c073550c97110",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIAGNOSTIC-RESULTS.md": "8a32f5ee9f1b1abe36cd4fcfb2bce465722842fb18a6711edfca65e53262383f",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-RESULTS.md": "fd9b0b558a5e5cac4c56d450139bfc227fc0481704edd1de8f5c138f37f4173b",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt": "badb83cb42fdf5e0e5801762602ffbe5e8301a9b0139242aece1829172974304",
    "experiments/moriarty-language/formal/quint/mil4/corpus/diagnostic-before-command-results.json": "9e530e6920379f7b8b4effff7f541853908b36230969b5b759c7750cb6964df3",
    "experiments/moriarty-language/formal/mil4/wire/PLAN.md": "4d151e0f63de4283dafce5a8b7f12c6bb030b7d15fc853e18ccd865dc4564a90",
    "experiments/moriarty-language/formal/mil4/wire/RESULT.md": "e79365271a104685912c165fa485ca230ce9ca30b3c957ca1478fc90d803e602",
    "experiments/moriarty-language/formal/mil4/wire/fixtures.json": "b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d",
    "experiments/moriarty-language/formal/mil4/wire/test-output.tap": "22ff49b67099fe8cdca6c10fd8d84e8f26e464dcc2e41ff725e2e1386fc907d1",
    "experiments/moriarty-language/formal/mil4/wire/metadata-snapshot-red-output.tap": "67cfd7621da0a5d5a6e29fff4f99d3812340200a426772eebd6eb83b9c625f40",
    "experiments/moriarty-language/formal/mil4/wire/post-audit-red-output.tap": "81c60c3cd90feacb3a7f456d806980fa99aa10dde5d2753e334556344d36705c",
    "experiments/moriarty-language/formal/mil4/wire/REFERENCE-VECTORS-RECEIPT.md": "913c4f51b1711331917a6ef1f020ebacd2fc2ea803e4adca45a1e3fa0c9f7c94",
    "experiments/moriarty-language/formal/mil4/wire/results.json": "9ebb42d0d98e1dbbba20a277cf722fa998b1123e660b33e7b87327fc842ec7cb",
    "experiments/moriarty-language/formal/mil4/wire/reference-vectors.py": "d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1",
    "experiments/moriarty-language/formal/mil4/wire/red-output.tap": "235d33174018a6cf8bac764104baaf82f0cef9a9e8f776929c9fc97c28c68d39",
    "experiments/moriarty-language/formal/mil4/wire/codec.test.mjs": "34c6dc78eb895e5966647d279390f9d4ca0cd4e1dc4a45f4aa34f04c6917a5e8",
    "experiments/moriarty-language/formal/mil4/wire/codec.mjs": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe",
    "experiments/moriarty-language/formal/mil4/wire/record-results.mjs": "baef324656586eff31824a772921365d0dcd65d37b8f9fd0eaf4502947a6d425",
    "experiments/moriarty-language/formal/mil4/wire/SPEC.md": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1",
    "experiments/moriarty-language/formal/mil4/signature/RESULT.md": "937a44916cd05535764338f496e7856fa4b52a54d197d78413d5709bcc931aa5",
    "experiments/moriarty-language/formal/mil4/signature/host-interoperability.test.mjs": "2465746efb832a8581d24334bac306eb6dcc04c71f9397ee8e7e54ab583fd132",
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda",
    "experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs": "0cc1c7857ee14ad1525fbd81e30b72bfec15932c5cd1f468360e2ccbde3efe67"
  },
  "protectedInputsAfterSha256": {
    "experiments/moriarty-language/formal/k/mil4/README.md": "91ceb57507c6fb2f804b0d2b9c16537ec8d3c053f28902b43ec48ac7eab36c7e",
    "experiments/moriarty-language/formal/k/mil4/s0.k": "e76c168bc6826e13308f19a34f229e5b07b64196a41892a6ac3122890d4c0cc0",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-parser-checked-results.json": "dae74490604f8c7d69f76a5d73f45df1e90685cca541acd278ca1fe71007529b",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-verification.json": "71dd8b6b682c33ff4cf452dbb11456bc78a9c735bd7035fc809d6d9c8c47bad5",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-results.json": "490621eb02e315a29c0f0498ca08bc70985b9e74fba689456c2489331f74ee6f",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-results.json": "0c5e8ede144a0e7aa157dec8981278c868cf1c62a3e3fb84b0f9a10d5c6dc34e",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-checked-results.json": "c6e433f3abbb197a90c2891694aa7c3e4268ea8f5142eb3ab06e09d9422326e7",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-results.json": "97b4a9668737e2852a8570518da6a62f36df996f39ddce619b83d7265ac2dd3a",
    "experiments/moriarty-language/formal/k/mil4/corpus/run_checked.py": "e2a2ac64a3438d8445cadf7e943a010da20d318b3367fe53608468d04389e463",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-semantic-compile.json": "01016057cda1f8ee549321c6520a53118fb1a3044e0cb7e6489f7a1b059dad2f",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-checked-results.json": "f565bc4bfa0308c0a8ff2bc6be43eb79a5617a604f2532b03c9db66e9ff020d7",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-compile.json": "ad135daf6c72894f62ead495294509143371c0106993cb3d2ffce46855d71c84",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-s0.k.txt": "f174aec77085bc4e351931ad52ccc32f360c1cdab6ac0c5271b6863054523551",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-parser-failure.json": "ff59dfae31cfb49b8b1a540775999a1ea3131f8b0a6b57fde8caa4a086489720",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd",
    "experiments/moriarty-language/formal/k/mil4/corpus/common_run.py": "7321573967ab0701a4395379a06381827d42f0f7f2496927a335ef36053d41b6",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-regression-results.json": "9ba861ec1ab85979b2ef7278687914496b845ae90c433ed9afaffc18eb668c24",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-verification.json": "d799ea72d6e1f93be0d23f15e4d22d24a5104724ed909dafa8fe711a5b49f816",
    "experiments/moriarty-language/formal/k/mil4/corpus/common-results.json": "f6c7a024611824320413b01f0960ffeb892b1ba7b7a2d61e224bb7e9e8428694",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-results.json": "7c676c28652cc578a23390aedc7ef0c396057ee3ed608da1c175744185a051c3",
    "experiments/moriarty-language/formal/k/mil4/corpus/checked-results.json": "4ecdddf2a515fd89bc588db4e99a6818f51f0079788a595ef41e06dcf5a08da3",
    "experiments/moriarty-language/formal/k/mil4/corpus/run.py": "ecb9c9d66be9ebb8b4d494fc3aa672555eedc78f341c37999657378d191bd8c7",
    "experiments/moriarty-language/formal/k/mil4/corpus/results.json": "35eb658f4710920d95b2f52cf18668abe2ffd5ae993c7f97b9a36fb58288a379",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-compile.json": "732e3848f239e0dafbcb5a306b9ee81c83d1037c96a419ed202f3d237d45d194",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-positional-final-compile.json": "3b9901d2acdd467330d4dbb6672281c5c6e9d0d3ee814aaff2e6a0c1fb9acb64",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b_run.py": "3649ecfee714b1c94ee1cbf8fcffafc7b745368dac77d15cafe8aa3e9f8702a2",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-compile.json": "80d4f4055801d5f8f164dbfa98c3e4d118a7e43adedaa51f234cdb2bc2d1323e",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-results.json": "e60e3a9ecb0044a2ded9f43320863d4773c22ba5f4e95785054f644b734dd87f",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-compile.json": "2d60c56a4b3a2b5bcdcbf992f3582325f3ef6f307635ed137f39126675ac95ed",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-positional-repair-red-result.json": "02bf27d5db669584924e9896959af00ec12c9d7ae2346799cee2dab62e99c2f7",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-verification.json": "e3f72bba4f552793f8d3e4c0dbea78932d0f3461c0a5cdf43ed1795f86c84f70",
    "experiments/moriarty-language/formal/k/mil4/corpus/pre-repair-semantic-red-results.json": "4299c3f78302c28aaabef9895338064c49d48eb56815608970bb4d870742664f",
    "experiments/moriarty-language/formal/k/mil4/corpus/corpus.k": "949dec467bc62379bce1d876d233c0261ba1ec7e8e5e4917436b9f2aa797dbdf",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-regression-results.json": "481cc4f5d806c02e5778e6a412fe8d43f89e9899d5177752efc421e3d19ba49a",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-regression-results.json": "9ba861ec1ab85979b2ef7278687914496b845ae90c433ed9afaffc18eb668c24",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-common-results.json": "544c050206ebb4efdcd5d2deaa329626af4a17bf4578a016533209c42906c2d9",
    "experiments/moriarty-language/formal/k/mil4/corpus/repair-final-compile.json": "f527beaf9d1c7c4d35c1fb6f0710809a38365c520cb92df794b998abb42d458c",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-pre-audit-common-results.json": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd",
    "experiments/moriarty-language/formal/k/mil4/corpus/s1b-single-duty-compile.json": "9d80b068ddf1f355750fdb3a300e754f721b926c7069ca72a7ddfab91249c293",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/run_checked.cpython-314.pyc": "c5607c4115bd391fcd706b7163e606523172deaafe305391bd69995ed83b9f1a",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/run.cpython-314.pyc": "d39b132b39337a3261b0cb3c1c4a232ef361d1838188559f32e5f537ff048116",
    "experiments/moriarty-language/formal/k/mil4/corpus/__pycache__/s1b_run.cpython-314.pyc": "44870de6aca654d4a9bfa62aafcb22c579f0d9be853b093570169ca6f05c2bf7",
    "experiments/moriarty-language/formal/quint/mil4/common-state.md": "5266f82e675228009c98d4cf6ceb092a0922f220b469f1b5a8c9651de866b00e",
    "experiments/moriarty-language/formal/quint/mil4/README.md": "9173e643401047be23dea1f8d502307e439e5e4cc90b115a985583d29f6ffe72",
    "experiments/moriarty-language/formal/quint/mil4/s0.qnt": "9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS-BEFORE-REPAIR.md": "0ab7eaf67f7ebd5e07b8acdbfb3c532707bb1e5755ff1f4a9f7bf17adff6aa83",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-BEFORE-REPAIR.md": "da901bfb7463c2645d76a5100ba97749bde823fbeb21dca9b54036c71bfa8880",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt": "358aa6e5c3d7e19c5ca8750e03597fcf17d21d855b085a18f0c6c456807c7ce1",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-pre-audit-command-results.json": "04d31bf725a9fc17278e5f34880e499041f2c771632fe8cd05d1fa0432c51ef1",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS-AFTER-ORDERING-REPAIR.md": "d192d5a8b59e69fcfd0993f7b17b806d91122d1237d361f2dd0526e1d17f5727",
    "experiments/moriarty-language/formal/quint/mil4/corpus/common-command-results.json": "f26e9747cf8d8cd2e9e0e6ca48176bafc6c3a2e3f1f32562025839f69e873477",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt": "a46335f66e003af41f4200448a8790f85175a4b78a5942bbefa5240aecb6f885",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt": "007b66094455b21467ee27fb744b55706e5a91c7a745523253ecb82cdf297591",
    "experiments/moriarty-language/formal/quint/mil4/corpus/diagnostic-command-results.json": "4134f0fc407acc405ad2075d615d33c0bbddaeab4cf9c923705dcb4c6035e3ca",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-AFTER-ORDERING-REPAIR.md": "5581682f997a1085c18000d32ed16cddf4f478106e4d6aebeccc4765e6fd0cfb",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-command-results.json": "ed69d4edb9a5c293ac24a1968f12a421f7f84414e837c809a720d5625e1c2a1c",
    "experiments/moriarty-language/formal/quint/mil4/corpus/repair-command-results.json": "407427617146acba92bdee536c2ab7edc4868c222a50bdb7edff1fc5257a2403",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-command-results.json": "a9905dc2de48a690c540111e1678878327cfafa7859e6186360e55f8e69dcffe",
    "experiments/moriarty-language/formal/quint/mil4/corpus/S1B-RESULTS.md": "2eb298dcb40cf650a95d11c140191f06d2c6d0808d5271943e5358bfc95db680",
    "experiments/moriarty-language/formal/quint/mil4/corpus/common-before-command-results.json": "82be849fb4ce5a5d558dc7ae114cef4e58bf1984011c67aaf007fbf2d45e059c",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-pre-repair-command-results.json": "8af96f3b19a53cbcbe770f2a70fbbc978fd0af6b5fab6782638a0e95c969ea49",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt": "b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e",
    "experiments/moriarty-language/formal/quint/mil4/corpus/RESULTS.md": "68e3609c264516e566f4f333dfca6f22ef4548b09a7ee2da128f800a64ba9322",
    "experiments/moriarty-language/formal/quint/mil4/corpus/COMMON-RESULTS.md": "d339a13c4a4f4d6658abf30f9e1b12c24542108cf18887ab169c073550c97110",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIAGNOSTIC-RESULTS.md": "8a32f5ee9f1b1abe36cd4fcfb2bce465722842fb18a6711edfca65e53262383f",
    "experiments/moriarty-language/formal/quint/mil4/corpus/DIVERGENCE-RESULTS.md": "fd9b0b558a5e5cac4c56d450139bfc227fc0481704edd1de8f5c138f37f4173b",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt": "badb83cb42fdf5e0e5801762602ffbe5e8301a9b0139242aece1829172974304",
    "experiments/moriarty-language/formal/quint/mil4/corpus/diagnostic-before-command-results.json": "9e530e6920379f7b8b4effff7f541853908b36230969b5b759c7750cb6964df3",
    "experiments/moriarty-language/formal/mil4/wire/PLAN.md": "4d151e0f63de4283dafce5a8b7f12c6bb030b7d15fc853e18ccd865dc4564a90",
    "experiments/moriarty-language/formal/mil4/wire/RESULT.md": "e79365271a104685912c165fa485ca230ce9ca30b3c957ca1478fc90d803e602",
    "experiments/moriarty-language/formal/mil4/wire/fixtures.json": "b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d",
    "experiments/moriarty-language/formal/mil4/wire/test-output.tap": "22ff49b67099fe8cdca6c10fd8d84e8f26e464dcc2e41ff725e2e1386fc907d1",
    "experiments/moriarty-language/formal/mil4/wire/metadata-snapshot-red-output.tap": "67cfd7621da0a5d5a6e29fff4f99d3812340200a426772eebd6eb83b9c625f40",
    "experiments/moriarty-language/formal/mil4/wire/post-audit-red-output.tap": "81c60c3cd90feacb3a7f456d806980fa99aa10dde5d2753e334556344d36705c",
    "experiments/moriarty-language/formal/mil4/wire/REFERENCE-VECTORS-RECEIPT.md": "913c4f51b1711331917a6ef1f020ebacd2fc2ea803e4adca45a1e3fa0c9f7c94",
    "experiments/moriarty-language/formal/mil4/wire/results.json": "9ebb42d0d98e1dbbba20a277cf722fa998b1123e660b33e7b87327fc842ec7cb",
    "experiments/moriarty-language/formal/mil4/wire/reference-vectors.py": "d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1",
    "experiments/moriarty-language/formal/mil4/wire/red-output.tap": "235d33174018a6cf8bac764104baaf82f0cef9a9e8f776929c9fc97c28c68d39",
    "experiments/moriarty-language/formal/mil4/wire/codec.test.mjs": "34c6dc78eb895e5966647d279390f9d4ca0cd4e1dc4a45f4aa34f04c6917a5e8",
    "experiments/moriarty-language/formal/mil4/wire/codec.mjs": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe",
    "experiments/moriarty-language/formal/mil4/wire/record-results.mjs": "baef324656586eff31824a772921365d0dcd65d37b8f9fd0eaf4502947a6d425",
    "experiments/moriarty-language/formal/mil4/wire/SPEC.md": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1",
    "experiments/moriarty-language/formal/mil4/signature/RESULT.md": "937a44916cd05535764338f496e7856fa4b52a54d197d78413d5709bcc931aa5",
    "experiments/moriarty-language/formal/mil4/signature/host-interoperability.test.mjs": "2465746efb832a8581d24334bac306eb6dcc04c71f9397ee8e7e54ab583fd132",
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda",
    "experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs": "0cc1c7857ee14ad1525fbd81e30b72bfec15932c5cd1f468360e2ccbde3efe67"
  },
  "protectedInputsUnchangedDuringRepairRun": true,
  "protectedInputsUnchangedSinceInitialCapture": false,
  "inputsChangedSinceInitialCapture": {
    "experiments/moriarty-language/formal/quint/mil4/README.md": {
      "initialSha256": "87e7ed03d686bfbec14451f99dcb25152c7945f7c80570a93b739c4f27565a83",
      "currentSha256": "9173e643401047be23dea1f8d502307e439e5e4cc90b115a985583d29f6ffe72"
    },
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": {
      "initialSha256": "e0b6c203eb29045ef6e4357033f6cb519fc1783a9c8e1fc3087227c7795d6aa3",
      "currentSha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
    },
    "experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs": {
      "initialSha256": "6cdce3bf80bed4f2ce6b32596169e98fe7ee7cdad618406506fdeab17b687971",
      "currentSha256": "0cc1c7857ee14ad1525fbd81e30b72bfec15932c5cd1f468360e2ccbde3efe67"
    }
  },
  "initialITFTracesUnchanged": true,
  "scope": "Repaired finite local comparator; 12 author checks; fresh provider review pending."
}

```

## experiments/moriarty-language/formal/mil4/alpha-local/repair-final-artifact-sha256.json

```text
{
  "RESULT.md": "85e07467fa412eb86f3569dd94c0edd5c6b9883939aab415f3d963b7f750e497",
  "artifact-sha256.json": "adb36ec45acaaa38801f13b8e2015e398056208fc6cdea9b46dd68038598317c",
  "command-results.json": "6592210c0feb3b8a0d7b89d87dd03a20e06cef59b279a91b72ff13c31f24d547",
  "compare.py": "1c7337b705ce929e3e6c1401638b2e721d084138a6b756bf9ffde7f01630365b",
  "direct-core.mjs": "df6fdc938eeb0c36b349eccee26efa9159cd8f6a0901145522774be3176f7063",
  "quint_repayNearBoundCommonTest_2.itf.json": "3f7809719aa636d08ddabeb4c3bc6120ebafce62292d1cdd702b6192ede03589",
  "quint_repayThirtyCommonTest_1.itf.json": "0b376280a750730724c8a4615669fd8d11a2e90680b69087d045d90fdc5f94b3",
  "quint_transferCommonTest_0.itf.json": "499c09eefbdc8609c12e36695b2a7cbc4a8781593c6b81dc93321ba1a8658ade",
  "repair-artifact-sha256.json": "f2fbff4f85f0fc0490e14b06a060cb1f835f65fee63945e37292931ad84042ae",
  "repair-final-command-results.json": "6d2b79585634736da85d53daabf02c8c7cf43579ec95e1a72b968542bd612201",
  "repair-green-command-results.json": "5fcd9182d4b314c5ed774e73a8e0386041b38d43dbd66302677a1ed0b90f0078",
  "repair-red-command-results.json": "702688a519e41ff07d2d912c26e58ca7388ca3be83003fa049b6e69c784a62d6",
  "test_compare.py": "bc137c428fbd3b191ac5b48df0d9dfb1c108335ee35d71b6f6cbdd8c4505fed9"
}

```

## experiments/moriarty-language/formal/mil4/alpha-local/quint_transferCommonTest_0.itf.json

```text
{"#meta":{"format":"ITF","format-description":"https://apalache-mc.org/docs/adr/015adr-trace.html","source":"experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt","status":"passed","description":"Created by Quint on Wed Sep 30 2026 01:25:21 GMT-0600 (Mountain Daylight Time)","timestamp":1790753121838},"vars":["s0_common_witnesses::s0::round","s0_common_witnesses::s0::currentHead","s0_common_witnesses::s0::committedCount","s0_common_witnesses::s0::obligations","s0_common_witnesses::s0::lastEffects","s0_common_witnesses::s0::consumed","s0_common_witnesses::s0::workSpent","s0_common_witnesses::s0::balances","s0_common_witnesses::s0::workRemaining","s0_common_witnesses::s0::allowanceSpent","s0_common_witnesses::s0::signed","s0_common_witnesses::s0::allowanceRemaining"],"states":[{"#meta":{"index":0},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[]},"s0_common_witnesses::s0::balances":{"#map":[]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":1},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"11"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":2},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"11"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["O",{"#bigint":"100"}],["R",{"#bigint":"0"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":3},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"11"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["F",{"#bigint":"0"}],["O",{"#bigint":"100"}],["R",{"#bigint":"0"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":4},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"11"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["F",{"#bigint":"0"}],["O",{"#bigint":"100"}],["R",{"#bigint":"0"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-T-10-1",{"actionAmount":{"#bigint":"10"},"actionFee":{"#bigint":"1"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"","debtor":"","digest":"digest-T-10-1","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"1"},"feeRecipient":"F","grossCap":{"#bigint":"11"},"netFloor":{"#bigint":"0"},"nonce":"T-10-1","obligationId":"","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"TransferProgram","value":{"#tup":[]}},"recipient":"R","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":5},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"11"}]]},"s0_common_witnesses::s0::balances":{"#map":[["F",{"#bigint":"1"}],["O",{"#bigint":"89"}],["R",{"#bigint":"10"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"1"},"s0_common_witnesses::s0::consumed":{"#set":[{"#tup":["D","O","T-10-1"]}]},"s0_common_witnesses::s0::currentHead":{"#bigint":"1"},"s0_common_witnesses::s0::lastEffects":[{"tag":"Debit","value":{"account":"O","amount":{"#bigint":"11"}}},{"tag":"Credit","value":{"account":"R","amount":{"#bigint":"10"}}},{"tag":"Credit","value":{"account":"F","amount":{"#bigint":"1"}}},{"tag":"UseAllowance","value":{"amount":{"#bigint":"11"},"owner":"O"}},{"tag":"UseReplay","value":{"#tup":["D","O","T-10-1"]}},{"tag":"AdvanceHead","value":{"after":{"#bigint":"1"},"before":{"#bigint":"0"}}}],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-T-10-1",{"actionAmount":{"#bigint":"10"},"actionFee":{"#bigint":"1"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"","debtor":"","digest":"digest-T-10-1","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"1"},"feeRecipient":"F","grossCap":{"#bigint":"11"},"netFloor":{"#bigint":"0"},"nonce":"T-10-1","obligationId":"","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"TransferProgram","value":{"#tup":[]}},"recipient":"R","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"0"},"s0_common_witnesses::s0::workSpent":{"#bigint":"1"}}]}
```

## experiments/moriarty-language/formal/mil4/alpha-local/quint_repayThirtyCommonTest_1.itf.json

```text
{"#meta":{"format":"ITF","format-description":"https://apalache-mc.org/docs/adr/015adr-trace.html","source":"experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt","status":"passed","description":"Created by Quint on Wed Sep 30 2026 01:25:21 GMT-0600 (Mountain Daylight Time)","timestamp":1790753121846},"vars":["s0_common_witnesses::s0::round","s0_common_witnesses::s0::currentHead","s0_common_witnesses::s0::committedCount","s0_common_witnesses::s0::obligations","s0_common_witnesses::s0::lastEffects","s0_common_witnesses::s0::consumed","s0_common_witnesses::s0::workSpent","s0_common_witnesses::s0::balances","s0_common_witnesses::s0::workRemaining","s0_common_witnesses::s0::allowanceSpent","s0_common_witnesses::s0::signed","s0_common_witnesses::s0::allowanceRemaining"],"states":[{"#meta":{"index":0},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[]},"s0_common_witnesses::s0::balances":{"#map":[]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":1},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":2},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"0"}],["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":3},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"0"}],["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"10"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"1010"},"principal":{"#bigint":"1000"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":4},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"0"}],["O",{"#bigint":"100"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"10"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"1010"},"principal":{"#bigint":"1000"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-R-30",{"actionAmount":{"#bigint":"30"},"actionFee":{"#bigint":"0"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"C","debtor":"O","digest":"digest-R-30","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"0"},"feeRecipient":"","grossCap":{"#bigint":"30"},"netFloor":{"#bigint":"0"},"nonce":"R-30","obligationId":"L","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"RepayProgram","value":{"#tup":[]}},"recipient":"","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":5},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"70"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"30"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"30"}],["O",{"#bigint":"70"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"1"},"s0_common_witnesses::s0::consumed":{"#set":[{"#tup":["D","O","R-30"]}]},"s0_common_witnesses::s0::currentHead":{"#bigint":"1"},"s0_common_witnesses::s0::lastEffects":[{"tag":"Debit","value":{"account":"O","amount":{"#bigint":"30"}}},{"tag":"Credit","value":{"account":"C","amount":{"#bigint":"30"}}},{"tag":"SetObligation","value":{"id":"L","value":{"accrued":{"#bigint":"0"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"980"},"principal":{"#bigint":"980"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}}},{"tag":"UseAllowance","value":{"amount":{"#bigint":"30"},"owner":"O"}},{"tag":"UseReplay","value":{"#tup":["D","O","R-30"]}},{"tag":"AdvanceHead","value":{"after":{"#bigint":"1"},"before":{"#bigint":"0"}}}],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"0"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"980"},"principal":{"#bigint":"980"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-R-30",{"actionAmount":{"#bigint":"30"},"actionFee":{"#bigint":"0"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"C","debtor":"O","digest":"digest-R-30","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"0"},"feeRecipient":"","grossCap":{"#bigint":"30"},"netFloor":{"#bigint":"0"},"nonce":"R-30","obligationId":"L","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"RepayProgram","value":{"#tup":[]}},"recipient":"","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"0"},"s0_common_witnesses::s0::workSpent":{"#bigint":"1"}}]}
```

## experiments/moriarty-language/formal/mil4/alpha-local/quint_repayNearBoundCommonTest_2.itf.json

```text
{"#meta":{"format":"ITF","format-description":"https://apalache-mc.org/docs/adr/015adr-trace.html","source":"experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt","status":"passed","description":"Created by Quint on Wed Sep 30 2026 01:25:21 GMT-0600 (Mountain Daylight Time)","timestamp":1790753121850},"vars":["s0_common_witnesses::s0::round","s0_common_witnesses::s0::currentHead","s0_common_witnesses::s0::committedCount","s0_common_witnesses::s0::obligations","s0_common_witnesses::s0::lastEffects","s0_common_witnesses::s0::consumed","s0_common_witnesses::s0::workSpent","s0_common_witnesses::s0::balances","s0_common_witnesses::s0::workRemaining","s0_common_witnesses::s0::allowanceSpent","s0_common_witnesses::s0::signed","s0_common_witnesses::s0::allowanceRemaining"],"states":[{"#meta":{"index":0},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[]},"s0_common_witnesses::s0::balances":{"#map":[]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":1},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"340282366920938463463374607431768211454"}]]},"s0_common_witnesses::s0::balances":{"#map":[["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":2},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"340282366920938463463374607431768211454"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"340282366920938463463374607431768211454"}],["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":3},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"340282366920938463463374607431768211454"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"340282366920938463463374607431768211454"}],["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"1"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"170141183460469231731687303715884105727"},"principal":{"#bigint":"170141183460469231731687303715884105726"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":4},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"340282366920938463463374607431768211454"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"340282366920938463463374607431768211454"}],["O",{"#bigint":"1"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"0"},"s0_common_witnesses::s0::consumed":{"#set":[]},"s0_common_witnesses::s0::currentHead":{"#bigint":"0"},"s0_common_witnesses::s0::lastEffects":[],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"1"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"170141183460469231731687303715884105727"},"principal":{"#bigint":"170141183460469231731687303715884105726"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-R-near-bound",{"actionAmount":{"#bigint":"1"},"actionFee":{"#bigint":"0"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"C","debtor":"O","digest":"digest-R-near-bound","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"0"},"feeRecipient":"","grossCap":{"#bigint":"1"},"netFloor":{"#bigint":"0"},"nonce":"R-near-bound","obligationId":"L","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"RepayProgram","value":{"#tup":[]}},"recipient":"","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"1"},"s0_common_witnesses::s0::workSpent":{"#bigint":"0"}},{"#meta":{"index":5},"s0_common_witnesses::s0::allowanceRemaining":{"#map":[["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::allowanceSpent":{"#map":[["O",{"#bigint":"340282366920938463463374607431768211455"}]]},"s0_common_witnesses::s0::balances":{"#map":[["C",{"#bigint":"340282366920938463463374607431768211455"}],["O",{"#bigint":"0"}]]},"s0_common_witnesses::s0::committedCount":{"#bigint":"1"},"s0_common_witnesses::s0::consumed":{"#set":[{"#tup":["D","O","R-near-bound"]}]},"s0_common_witnesses::s0::currentHead":{"#bigint":"1"},"s0_common_witnesses::s0::lastEffects":[{"tag":"Debit","value":{"account":"O","amount":{"#bigint":"1"}}},{"tag":"Credit","value":{"account":"C","amount":{"#bigint":"1"}}},{"tag":"SetObligation","value":{"id":"L","value":{"accrued":{"#bigint":"0"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"170141183460469231731687303715884105726"},"principal":{"#bigint":"170141183460469231731687303715884105726"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}}},{"tag":"UseAllowance","value":{"amount":{"#bigint":"1"},"owner":"O"}},{"tag":"UseReplay","value":{"#tup":["D","O","R-near-bound"]}},{"tag":"AdvanceHead","value":{"after":{"#bigint":"1"},"before":{"#bigint":"0"}}}],"s0_common_witnesses::s0::obligations":{"#map":[["L",{"accrued":{"#bigint":"0"},"asset":"A","creditor":"C","debtor":"O","outstanding":{"#bigint":"170141183460469231731687303715884105726"},"principal":{"#bigint":"170141183460469231731687303715884105726"},"status":{"tag":"Outstanding","value":{"#tup":[]}}}]]},"s0_common_witnesses::s0::round":{"#bigint":"0"},"s0_common_witnesses::s0::signed":{"#map":[["digest-R-near-bound",{"actionAmount":{"#bigint":"1"},"actionFee":{"#bigint":"0"},"asset":"A","conversionMantissa":{"#bigint":"1"},"conversionScale":{"#bigint":"0"},"coreVersion":"Core/5","creditor":"C","debtor":"O","digest":"digest-R-near-bound","domain":"D","evidenceChoice":"e","feeCap":{"#bigint":"0"},"feeRecipient":"","grossCap":{"#bigint":"1"},"netFloor":{"#bigint":"0"},"nonce":"R-near-bound","obligationId":"L","payer":"O","policyDigest":"p","preHead":{"#bigint":"0"},"profile":"S0","program":{"tag":"RepayProgram","value":{"#tup":[]}},"recipient":"","roundingNone":true,"signer":"O","sourceVersion":"Source/6","terminalOnly":true,"validFrom":{"#bigint":"0"},"validThrough":{"#bigint":"10"}}]]},"s0_common_witnesses::s0::workRemaining":{"#bigint":"0"},"s0_common_witnesses::s0::workSpent":{"#bigint":"1"}}]}
```

## experiments/moriarty-language/formal/k/mil4/corpus/s1b-common-results.json

```text
[
  {
    "case": "T-10-1",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),\n            allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),\n        effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),\n          effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),\n            effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"O\",\"A\",89),balance(\"R\",\"A\",10),balance(\"F\",\"A\",1),\n            allowance(\"O\",\"A\",0,11),noObligation(),head(\"h1\"),\n            used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"O\" , \"A\" , 11 ) , effect ( credit ( \"R\" , \"A\" , 10 ) , effect ( credit ( \"F\" , \"A\" , 1 ) , effect ( useAllowance ( \"O\" , \"A\" , 11 ) , effect ( useReplay ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"O\" , \"A\" , 89 ) , balance ( \"R\" , \"A\" , 10 ) , balance ( \"F\" , \"A\" , 1 ) , allowance ( \"O\" , \"A\" , 0 , 11 ) , noObligation ( ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "observedExternal": "authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"O\" , \"A\" , \"T-10-1\" , head ( \"h0\" ) , 0 , 10 , \"R\" , \"F\" , 11 , 1 , 0 , transfer ( 10 , 1 ) , \"digest-T-10-1\" ) , state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )",
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "command": [
      "krun",
      "--definition",
      "/tmp/mil4-s1b-corpus-llvm-kompiled",
      "-o",
      "pretty",
      "/dev/stdin"
    ],
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "program": "withAuthenticated(submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0),authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties())))",
    "stdout": "<s0>\n  <k>\n    .K\n  </k>\n  <out>\n    accepted ( state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"O\" , \"A\" , 11 ) , effect ( credit ( \"R\" , \"A\" , 10 ) , effect ( credit ( \"F\" , \"A\" , 1 ) , effect ( useAllowance ( \"O\" , \"A\" , 11 ) , effect ( useReplay ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"O\" , \"A\" , 89 ) , balance ( \"R\" , \"A\" , 10 ) , balance ( \"F\" , \"A\" , 1 ) , allowance ( \"O\" , \"A\" , 0 , 11 ) , noObligation ( ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )\n  </out>\n  <external>\n    authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"O\" , \"A\" , \"T-10-1\" , head ( \"h0\" ) , 0 , 10 , \"R\" , \"F\" , 11 , 1 , 0 , transfer ( 10 , 1 ) , \"digest-T-10-1\" ) , state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )\n  </external>\n</s0>\n",
    "stderr": ""
  },
  {
    "case": "R-30",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),\n            allowance(\"P\",\"A\",100,0),\n            obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),\n            head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"P\",\"A\",30),effect(credit(\"C\",\"A\",30),\n        effect(setObligation(\"L\",980,0,980,\"Outstanding\"),\n          effect(useAllowance(\"P\",\"A\",30),\n            effect(useReplay(replayKey(\"D\",\"P\",\"R-30\")),\n              effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"P\",\"A\",70),balance(\"C\",\"A\",30),noBalance(),\n            allowance(\"P\",\"A\",70,30),\n            obligation(\"L\",\"P\",\"C\",\"A\",980,0,980,\"Outstanding\"),\n            head(\"h1\"),used(replayKey(\"D\",\"P\",\"R-30\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"P\" , \"A\" , 100 ) , balance ( \"C\" , \"A\" , 0 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 100 , 0 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 1000 , 10 , 1010 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 30 ) , effect ( credit ( \"C\" , \"A\" , 30 ) , effect ( setObligation ( \"L\" , 980 , 0 , 980 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 30 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-30\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 70 ) , balance ( \"C\" , \"A\" , 30 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 70 , 30 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 980 , 0 , 980 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-30\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "observedExternal": "authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"P\" , \"A\" , \"R-30\" , head ( \"h0\" ) , 0 , 10 , \"\" , \"\" , 30 , 0 , 0 , repay ( \"L\" , \"P\" , 30 , 1 , 0 , \"none\" ) , \"digest-R-30\" ) , state ( balance ( \"P\" , \"A\" , 100 ) , balance ( \"C\" , \"A\" , 0 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 100 , 0 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 1000 , 10 , 1010 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )",
    "preState": "state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),allowance(\"P\",\"A\",100,0),obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "command": [
      "krun",
      "--definition",
      "/tmp/mil4-s1b-corpus-llvm-kompiled",
      "-o",
      "pretty",
      "/dev/stdin"
    ],
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),0,10,\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),\"digest-R-30\"),fill(\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),head(\"h1\")),state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),allowance(\"P\",\"A\",100,0),obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),effect(debit(\"P\",\"A\",30),effect(credit(\"C\",\"A\",30),effect(setObligation(\"L\",980,0,980,\"Outstanding\"),effect(useAllowance(\"P\",\"A\",30),effect(useReplay(replayKey(\"D\",\"P\",\"R-30\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),0,10,\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),\"digest-R-30\"),state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),allowance(\"P\",\"A\",100,0),obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "program": "withAuthenticated(submit(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),0,10,\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),\"digest-R-30\"),fill(\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),head(\"h1\")),state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),allowance(\"P\",\"A\",100,0),obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),effect(debit(\"P\",\"A\",30),effect(credit(\"C\",\"A\",30),effect(setObligation(\"L\",980,0,980,\"Outstanding\"),effect(useAllowance(\"P\",\"A\",30),effect(useReplay(replayKey(\"D\",\"P\",\"R-30\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0),authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-30\",head(\"h0\"),0,10,\"\",\"\",30,0,0,repay(\"L\",\"P\",30,1,0,\"none\"),\"digest-R-30\"),state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),allowance(\"P\",\"A\",100,0),obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties())))",
    "stdout": "<s0>\n  <k>\n    .K\n  </k>\n  <out>\n    accepted ( state ( balance ( \"P\" , \"A\" , 100 ) , balance ( \"C\" , \"A\" , 0 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 100 , 0 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 1000 , 10 , 1010 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 30 ) , effect ( credit ( \"C\" , \"A\" , 30 ) , effect ( setObligation ( \"L\" , 980 , 0 , 980 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 30 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-30\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 70 ) , balance ( \"C\" , \"A\" , 30 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 70 , 30 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 980 , 0 , 980 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-30\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )\n  </out>\n  <external>\n    authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"P\" , \"A\" , \"R-30\" , head ( \"h0\" ) , 0 , 10 , \"\" , \"\" , 30 , 0 , 0 , repay ( \"L\" , \"P\" , 30 , 1 , 0 , \"none\" ) , \"digest-R-30\" ) , state ( balance ( \"P\" , \"A\" , 100 ) , balance ( \"C\" , \"A\" , 0 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 100 , 0 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 1000 , 10 , 1010 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )\n  </external>\n</s0>\n",
    "stderr": ""
  },
  {
    "case": "R-near-bound",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"P\",\"A\",1),\n            balance(\"C\",\"A\",340282366920938463463374607431768211454),\n            noBalance(),\n            allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),\n            obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,\n                       1,170141183460469231731687303715884105727,\"Outstanding\"),\n            head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"P\",\"A\",1),effect(credit(\"C\",\"A\",1),\n        effect(setObligation(\"L\",170141183460469231731687303715884105726,\n                             0,170141183460469231731687303715884105726,\n                             \"Outstanding\"),\n          effect(useAllowance(\"P\",\"A\",1),\n            effect(useReplay(replayKey(\"D\",\"P\",\"R-near-bound\")),\n              effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"P\",\"A\",0),\n            balance(\"C\",\"A\",340282366920938463463374607431768211455),\n            noBalance(),\n            allowance(\"P\",\"A\",0,340282366920938463463374607431768211455),\n            obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,\n                       0,170141183460469231731687303715884105726,\"Outstanding\"),\n            head(\"h1\"),used(replayKey(\"D\",\"P\",\"R-near-bound\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"P\" , \"A\" , 1 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211454 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 1 , 340282366920938463463374607431768211454 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 1 , 170141183460469231731687303715884105727 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 1 ) , effect ( credit ( \"C\" , \"A\" , 1 ) , effect ( setObligation ( \"L\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 1 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 0 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211455 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 0 , 340282366920938463463374607431768211455 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "observedExternal": "authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"P\" , \"A\" , \"R-near-bound\" , head ( \"h0\" ) , 0 , 10 , \"\" , \"\" , 1 , 0 , 0 , repay ( \"L\" , \"P\" , 1 , 1 , 0 , \"none\" ) , \"digest-R-near-bound\" ) , state ( balance ( \"P\" , \"A\" , 1 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211454 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 1 , 340282366920938463463374607431768211454 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 1 , 170141183460469231731687303715884105727 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )",
    "preState": "state(balance(\"P\",\"A\",1),balance(\"C\",\"A\",340282366920938463463374607431768211454),noBalance(),allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,1,170141183460469231731687303715884105727,\"Outstanding\"),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "command": [
      "krun",
      "--definition",
      "/tmp/mil4-s1b-corpus-llvm-kompiled",
      "-o",
      "pretty",
      "/dev/stdin"
    ],
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),0,10,\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),\"digest-R-near-bound\"),fill(\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),head(\"h1\")),state(balance(\"P\",\"A\",1),balance(\"C\",\"A\",340282366920938463463374607431768211454),noBalance(),allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,1,170141183460469231731687303715884105727,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),effect(debit(\"P\",\"A\",1),effect(credit(\"C\",\"A\",1),effect(setObligation(\"L\",170141183460469231731687303715884105726,0,170141183460469231731687303715884105726,\"Outstanding\"),effect(useAllowance(\"P\",\"A\",1),effect(useReplay(replayKey(\"D\",\"P\",\"R-near-bound\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),0,10,\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),\"digest-R-near-bound\"),state(balance(\"P\",\"A\",1),balance(\"C\",\"A\",340282366920938463463374607431768211454),noBalance(),allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,1,170141183460469231731687303715884105727,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "program": "withAuthenticated(submit(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),0,10,\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),\"digest-R-near-bound\"),fill(\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),head(\"h1\")),state(balance(\"P\",\"A\",1),balance(\"C\",\"A\",340282366920938463463374607431768211454),noBalance(),allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,1,170141183460469231731687303715884105727,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),effect(debit(\"P\",\"A\",1),effect(credit(\"C\",\"A\",1),effect(setObligation(\"L\",170141183460469231731687303715884105726,0,170141183460469231731687303715884105726,\"Outstanding\"),effect(useAllowance(\"P\",\"A\",1),effect(useReplay(replayKey(\"D\",\"P\",\"R-near-bound\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0),authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"P\",\"A\",\"R-near-bound\",head(\"h0\"),0,10,\"\",\"\",1,0,0,repay(\"L\",\"P\",1,1,0,\"none\"),\"digest-R-near-bound\"),state(balance(\"P\",\"A\",1),balance(\"C\",\"A\",340282366920938463463374607431768211454),noBalance(),allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,1,170141183460469231731687303715884105727,\"Outstanding\"),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties())))",
    "stdout": "<s0>\n  <k>\n    .K\n  </k>\n  <out>\n    accepted ( state ( balance ( \"P\" , \"A\" , 1 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211454 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 1 , 340282366920938463463374607431768211454 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 1 , 170141183460469231731687303715884105727 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 1 ) , effect ( credit ( \"C\" , \"A\" , 1 ) , effect ( setObligation ( \"L\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 1 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 0 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211455 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 0 , 340282366920938463463374607431768211455 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )\n  </out>\n  <external>\n    authenticated ( intent ( \"Source/6\" , \"Core/5\" , \"D\" , \"P\" , \"A\" , \"R-near-bound\" , head ( \"h0\" ) , 0 , 10 , \"\" , \"\" , 1 , 0 , 0 , repay ( \"L\" , \"P\" , 1 , 1 , 0 , \"none\" ) , \"digest-R-near-bound\" ) , state ( balance ( \"P\" , \"A\" , 1 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211454 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 1 , 340282366920938463463374607431768211454 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 1 , 170141183460469231731687303715884105727 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , 0 , head ( \"h1\" ) , outcome ( terminalSuccess ( ) , noEffects ( ) , noDuties ( ) ) )\n  </external>\n</s0>\n",
    "stderr": ""
  }
]

```

## experiments/moriarty-language/formal/quint/mil4/s0.qnt

```text
// Provisional MIL/4 S0 transition model. See README.md for its evidence limits.
module s0 {
  type ReplayKey = (str, str, str)
  type Program = TransferProgram | RepayProgram
  type DebtStatus = Outstanding | Settled
  type Judgment = Accepted | Stage | Intent | Effect | Authority | History | Failure
  type Decision = { accepted: bool, judgment: Judgment, code: str,
                    diagnosticWork: int }

  type SignedIntent = {
    sourceVersion: str,
    coreVersion: str,
    profile: str,
    program: Program,
    domain: str,
    asset: str,
    digest: str,
    policyDigest: str,
    evidenceChoice: str,
    signer: str,
    payer: str,
    recipient: str,
    feeRecipient: str,
    obligationId: str,
    debtor: str,
    creditor: str,
    nonce: str,
    preHead: int,
    validFrom: int,
    validThrough: int,
    grossCap: int,
    feeCap: int,
    netFloor: int,
    actionAmount: int,
    actionFee: int,
    conversionMantissa: int,
    conversionScale: int,
    roundingNone: bool,
    terminalOnly: bool,
  }

  type Obligation = {
    debtor: str,
    creditor: str,
    asset: str,
    principal: int,
    accrued: int,
    outstanding: int,
    status: DebtStatus,
  }

  type EffectLine =
    | Debit({ account: str, amount: int })
    | Credit({ account: str, amount: int })
    | SetObligation({ id: str, value: Obligation })
    | UseAllowance({ owner: str, amount: int })
    | UseReplay(ReplayKey)
    | AdvanceHead({ before: int, after: int })

  type RequestedPhase = TerminalSuccess | RequestedFailure | RequestedPending
  type RetainedDuty = { id: str, owner: str, amount: int }
  type RequestedOutcome = { phase: RequestedPhase,
    retainedEffects: List[EffectLine], retainedDuties: List[RetainedDuty] }
  type FinancialSnapshot = {
    balances: str -> int, allowanceRemaining: str -> int,
    allowanceSpent: str -> int, workRemaining: int, workSpent: int,
    obligations: str -> Obligation, head: int, consumed: Set[ReplayKey],
  }
  type StipulatedTuple = { available: bool, intent: SignedIntent,
    preState: FinancialSnapshot, round: int, expectedSuccessor: int,
    requestedOutcome: RequestedOutcome }
  type ComparisonRequest = { submittedSuccessor: int,
    outcome: RequestedOutcome, premise: StipulatedTuple }
  pure val terminalOutcome: RequestedOutcome = {
    phase: TerminalSuccess, retainedEffects: List(), retainedDuties: List(),
  }

  // These are environmental premises, not checks implemented by Quint. The
  // selected external interfaces must establish their truth before admission.
  const signatureVerified: Set[str]
  const authenticatedSnapshots: Set[int]
  const nativeQualified: Set[str]
  const ledgerAtomicReady: bool
  const executingDomain: str
  const settlementAsset: str
  const initialWorkBudget: int

  pure val UINT128_MAX = 340282366920938463463374607431768211455
  pure val NOMINAL_MAX = 170141183460469231731687303715884105727

  var signed: str -> SignedIntent
  var balances: str -> int
  var allowanceRemaining: str -> int
  var allowanceSpent: str -> int
  var workRemaining: int
  var workSpent: int
  var obligations: str -> Obligation
  var currentHead: int
  var round: int
  var consumed: Set[ReplayKey]
  var lastEffects: List[EffectLine]
  var committedCount: int

  val financialSnapshot: FinancialSnapshot = {
    balances: balances, allowanceRemaining: allowanceRemaining,
    allowanceSpent: allowanceSpent, workRemaining: workRemaining,
    workSpent: workSpent, obligations: obligations, head: currentHead,
    consumed: consumed,
  }
  // Snapshot authentication remains an explicit external premise at every head.
  // A local commit never adds a head to this stipulated set.
  val currentSnapshotAuthenticated = authenticatedSnapshots.contains(currentHead)

  pure def replayKey(i: SignedIntent): ReplayKey = (i.domain, i.signer, i.nonce)
  pure def ok: Decision = { accepted: true, judgment: Accepted, code: "",
                            diagnosticWork: 0 }
  // Rejection reports one abstract diagnostic unit; it does not debit work.
  pure def reject(j: Judgment, c: str): Decision =
    { accepted: false, judgment: j, code: c, diagnosticWork: 1 }
  pure def withinUInt(n: int): bool = n >= 0 and n <= UINT128_MAX
  pure def withinNominal(n: int): bool = n >= 0 and n <= NOMINAL_MAX
  pure def minInt(a: int, b: int): int = if (a < b) a else b

  // Stage validates the complete work pair before premise or Authority checks.
  val stageWorkWellFormed = withinUInt(workRemaining) and withinUInt(workSpent)
    and workRemaining + workSpent <= UINT128_MAX

  pure def transferLines(i: SignedIntent, v: int, f: int, successor: int): List[EffectLine] = {
    val gross = v + f
    val money = if (f == 0)
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }))
    else
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }),
           Credit({ account: i.feeRecipient, amount: f }))
    money.concat(List(
      UseAllowance({ owner: i.payer, amount: gross }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    ))
  }

  pure def repaid(o: Obligation, n: int): Obligation = {
    val da = minInt(n, o.accrued)
    val dp = n - da
    val p = o.principal - dp
    val a = o.accrued - da
    { ...o, principal: p, accrued: a, outstanding: p + a,
      status: if (p + a == 0) Settled else Outstanding }
  }

  pure def repayLines(i: SignedIntent, o: Obligation, n: int, successor: int): List[EffectLine] =
    List(
      Debit({ account: i.payer, amount: n }),
      Credit({ account: o.creditor, amount: n }),
      SetObligation({ id: i.obligationId, value: repaid(o, n) }),
      UseAllowance({ owner: i.payer, amount: n }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    )

  // A rejected proposal is inspected through this value; it is never a stage.
  // Code spelling and within-judgment precedence are provisional W-D3 leaves.
  // Signed scope and caps precede alias diagnosis. Numeric effect guards in
  // each observation precede submitted-vector comparison.
  def beforeEffect(id: str, supplied: SignedIntent,
                     claimedHead: int, amount: int, fee: int,
                     recipient: str, debtor: str, isRepay: bool, request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)) or
        supplied.sourceVersion != "Source/6" or
        supplied.coreVersion != "Core/5" or
        supplied.profile != "S0" or
        supplied.domain != executingDomain or
        supplied.asset != settlementAsset)
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else if (not(signatureVerified.contains(signed.get(id).digest)) or
             not(currentSnapshotAuthenticated) or
             not(nativeQualified.contains(signed.get(id).digest)) or
             not(ledgerAtomicReady) or
             not(request.premise.available) or
             request.premise.intent != signed.get(id) or
             request.premise.preState != financialSnapshot or
             request.premise.round != round or
             request.premise.requestedOutcome != request.outcome)
      reject(Stage, "S0_STAGE_PREMISE")
    else if (signed.get(id) != supplied or
             amount != supplied.actionAmount or
             fee != supplied.actionFee or
             amount <= 0 or
             not(withinNominal(supplied.actionAmount)) or
             not(withinNominal(supplied.actionFee)) or
             not(withinNominal(supplied.grossCap)) or
             not(withinNominal(supplied.feeCap)) or
             not(withinNominal(supplied.netFloor)) or
             not(withinUInt(supplied.validFrom)) or
             not(withinUInt(supplied.validThrough)) or
             round < supplied.validFrom or round > supplied.validThrough or
             supplied.validFrom > supplied.validThrough or
             claimedHead != supplied.preHead or
             (not(isRepay) and
              (fee > supplied.feeCap or
               amount + fee > supplied.grossCap or
               amount < supplied.netFloor)) or
             (isRepay and (amount > supplied.grossCap or
                           supplied.feeCap != 0 or
                           supplied.netFloor != 0 or
                           supplied.recipient != "" or
                           supplied.feeRecipient != "" or
                           supplied.debtor != debtor or
                           supplied.creditor != recipient or
                           supplied.conversionMantissa != 1 or
                           supplied.conversionScale != 0 or
                           not(supplied.roundingNone))))
      reject(Intent, "S0_INTENT_SCOPE")
    else if (supplied.payer == recipient or
             (not(isRepay) and
              (supplied.payer == supplied.feeRecipient or
               supplied.recipient == supplied.feeRecipient)))
      reject(Intent, "S0_INTENT_ALIAS")
    else if (not(withinNominal(amount)) or
             not(withinNominal(amount + fee)) or fee < 0)
      reject(Effect, "S0_EFFECT_RANGE")
    else ok
  }

  def afterEffect(supplied: SignedIntent, claimedHead: int,
                  amount: int, fee: int, request: ComparisonRequest): Decision = {
    if (supplied.signer != supplied.payer or
             allowanceRemaining.get(supplied.payer) < amount + fee or
             allowanceSpent.get(supplied.payer) + amount + fee > UINT128_MAX or
             workRemaining < 1 or workSpent + 1 > UINT128_MAX)
      reject(Authority, "S0_AUTH_SCOPE")
    else if (claimedHead != currentHead)
      reject(History, "S0_HISTORY_STALE")
    else if (consumed.contains(replayKey(supplied)))
      reject(History, "S0_HISTORY_REPLAY")
    else if (request.submittedSuccessor == claimedHead or
             request.submittedSuccessor != request.premise.expectedSuccessor)
      reject(History, "S0_HISTORY_SUCCESSOR")
    else if (not(supplied.terminalOnly) or
             request.outcome.phase != TerminalSuccess or
             request.outcome.retainedEffects != List() or
             request.outcome.retainedDuties != List())
      reject(Failure, "S0_FAILURE_UNSUPPORTED")
    else ok
  }

  def transferObservation(id: str, supplied: SignedIntent,
                          claimedHead: int, v: int, f: int,
                          lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != TransferProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set() or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          // Preserve Intent diagnosis for signed endpoint aliases. Distinct
          // endpoints select the exact three-balance S0 footprint.
          (bound.payer != bound.recipient and bound.payer != bound.feeRecipient and
           bound.recipient != bound.feeRecipient and
           balances.keys() != Set(bound.payer, bound.recipient, bound.feeRecipient)) or
          not(balances.keys().contains(bound.payer)) or
          not(balances.keys().contains(bound.recipient)) or
          not(balances.keys().contains(bound.feeRecipient)) or
          not(withinUInt(balances.get(bound.recipient))) or
          not(withinUInt(balances.get(bound.feeRecipient))) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val base = beforeEffect(id, supplied, claimedHead, v, f,
                                supplied.recipient, "", false, request)
      if (not(base.accepted)) base
      else if (balances.get(supplied.payer) < v + f or
               balances.get(supplied.recipient) + v > UINT128_MAX or
               balances.get(supplied.feeRecipient) + f > UINT128_MAX or
               not(withinUInt(balances.get(supplied.payer))) or
               not(withinUInt(balances.get(supplied.recipient))) or
               not(withinUInt(balances.get(supplied.feeRecipient))))
        reject(Effect, "S0_EFFECT_RANGE")
      else if (lines != transferLines(supplied, v, f, request.submittedSuccessor))
        reject(Effect, "S0_EFFECT_MISMATCH")
      else afterEffect(supplied, claimedHead, v, f, request)
      }
    }
  }

  def repayObservation(id: str, supplied: SignedIntent,
                       claimedHead: int, n: int,
                       lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != RepayProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set(bound.obligationId) or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          not(obligations.keys().contains(bound.obligationId)) or
          not(balances.keys().contains(bound.payer)) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val o = obligations.get(bound.obligationId)
        // Use the immutable bound intent here: a supplied payer substitution
        // belongs to Intent after the existing typed state has passed Stage.
        if (balances.keys() != Set(bound.payer, o.creditor) or
            not(balances.keys().contains(o.creditor)) or
            not(withinUInt(balances.get(o.creditor))) or
            not(withinNominal(o.principal)) or
            not(withinNominal(o.accrued)) or
            not(withinNominal(o.outstanding)) or
            o.outstanding != o.principal + o.accrued or
            o.status != Outstanding or o.asset != settlementAsset or
            o.debtor != bound.signer or o.debtor != bound.payer or
            o.creditor == bound.payer)
          reject(Stage, "S0_STAGE_UNSUPPORTED")
        else {
        val base = beforeEffect(id, supplied, claimedHead, n, 0,
                                  o.creditor, o.debtor, true, request)
        if (not(base.accepted)) base
        else if (n > o.outstanding or
                 balances.get(supplied.payer) < n or
                 balances.get(o.creditor) + n > UINT128_MAX or
                 not(withinUInt(balances.get(supplied.payer))) or
                 not(withinUInt(balances.get(o.creditor))) or
                 not(withinUInt(repaid(o, n).outstanding)))
          reject(Effect, "S0_EFFECT_RANGE")
        else if (lines != repayLines(supplied, o, n, request.submittedSuccessor))
          reject(Effect, "S0_EFFECT_MISMATCH")
        else afterEffect(supplied, claimedHead, n, 0, request)
        }
      }
    }
  }

  action init: bool = all {
    withinUInt(initialWorkBudget),
    signed' = Map(),
    balances' = Map(),
    allowanceRemaining' = Map(),
    allowanceSpent' = Map(),
    workRemaining' = initialWorkBudget,
    workSpent' = 0,
    obligations' = Map(),
    currentHead' = 0,
    round' = 0,
    consumed' = Set(),
    lastEffects' = List(),
    committedCount' = 0,
  }

  // Environment setup represents authenticated cells; it is not a ledger proof.
  action seedAccount(account: str, balance: int,
                     remaining: int, spent: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance) and withinUInt(remaining) and withinUInt(spent),
    remaining + spent <= UINT128_MAX,
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining.put(account, remaining),
    allowanceSpent' = allowanceSpent.put(account, spent),
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  // A receiving balance does not imply authority to spend from that account.
  action seedBalanceOnly(account: str, balance: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance),
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action seedObligation(id: str, o: Obligation): bool = all {
    committedCount == 0,
    not(obligations.keys().contains(id)),
    o.asset == settlementAsset,
    withinUInt(o.principal) and withinUInt(o.accrued) and
      withinUInt(o.outstanding),
    o.outstanding == o.principal + o.accrued,
    obligations' = obligations.put(id, o),
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action advanceRound: bool = all {
    round' = round + 1,
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    obligations' = obligations, currentHead' = currentHead,
    consumed' = consumed, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  action sign(i: SignedIntent): bool = all {
    not(signed.keys().contains(i.digest)),
    i.domain == executingDomain and i.asset == settlementAsset,
    i.sourceVersion == "Source/6" and i.coreVersion == "Core/5",
    i.profile == "S0",
    signed' = signed.put(i.digest, i),
    balances' = balances, allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent, obligations' = obligations,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action submitTransfer(id: str, supplied: SignedIntent, claimedHead: int,
                        v: int, f: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    transferObservation(id, supplied, claimedHead, v, f, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - v - f)
      .put(supplied.recipient, balances.get(supplied.recipient) + v)
      .put(supplied.feeRecipient, balances.get(supplied.feeRecipient) + f),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - v - f),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + v + f),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, obligations' = obligations, round' = round,
  }

  action submitRepay(id: str, supplied: SignedIntent, claimedHead: int,
                     n: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    repayObservation(id, supplied, claimedHead, n, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - n)
      .put(obligations.get(supplied.obligationId).creditor,
           balances.get(obligations.get(supplied.obligationId).creditor) + n),
    obligations' = obligations.put(supplied.obligationId,
      repaid(obligations.get(supplied.obligationId), n)),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - n),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + n),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, round' = round,
  }
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt

```text
// Three common-state positive witnesses with independent literal expectations.
// K h0/h1 map to 0/1 and repayment account P maps to O in this instance.
module s0_common_witnesses {
  import s0(
    signatureVerified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "digest-T-10-1", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "T-10-1",
    preHead: 0, validFrom: 0, validThrough: 10,
    grossCap: 11, feeCap: 1, netFloor: 0, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }
  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "digest-R-30", nonce: "R-30",
    recipient: "", feeRecipient: "", obligationId: "L",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    actionAmount: 30, actionFee: 0,
  }
  pure val near: SignedIntent = {
    ...r, digest: "digest-R-near-bound", nonce: "R-near-bound",
    grossCap: 1, actionAmount: 1,
  }
  pure val debt: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val debtAfterThirty: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 980,
    accrued: 0, outstanding: 980, status: Outstanding,
  }
  pure val debtNear: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 1, outstanding: 170141183460469231731687303715884105727,
    status: Outstanding,
  }
  pure val debtAfterOne: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 0, outstanding: 170141183460469231731687303715884105726,
    status: Outstanding,
  }

  // Expected lines and debt values are literal; no model preparation helper
  // computes the oracle supplied to submit or checked in the successor.
  pure val transferExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    Credit({ account: "F", amount: 1 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(("D", "O", "T-10-1")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val thirtyExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    Credit({ account: "C", amount: 30 }),
    SetObligation({ id: "L", value: debtAfterThirty }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(("D", "O", "R-30")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val oneExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 1 }),
    Credit({ account: "C", amount: 1 }),
    SetObligation({ id: "L", value: debtAfterOne }),
    UseAllowance({ owner: "O", amount: 1 }),
    UseReplay(("D", "O", "R-near-bound")),
    AdvanceHead({ before: 0, after: 1 })
  )

  run transferCommonTest =
    init.then(seedAccount("O", 100, 11, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 100, "R" -> 0, "F" -> 0),
        allowanceRemaining == Map("O" -> 11),
        allowanceSpent == Map("O" -> 0),
        obligations == Map(),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        transferObservation("digest-T-10-1", t, 0, 10, 1,
          transferExpected, fixtureRequest("digest-T-10-1", t, 1)) == {
            accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
          },
      })
      .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, transferExpected, fixtureRequest("digest-T-10-1", t, 1)))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 89, "R" -> 10, "F" -> 1),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 11),
        not(allowanceRemaining.keys().contains("R")) and
          not(allowanceRemaining.keys().contains("F")),
        not(allowanceSpent.keys().contains("R")) and
          not(allowanceSpent.keys().contains("F")),
        obligations == Map(),
        consumed == Set(("D", "O", "T-10-1")),
        lastEffects == transferExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayThirtyCommonTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("L", debt)).then(sign(r))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 100, "C" -> 0),
        allowanceRemaining == Map("O" -> 100),
        allowanceSpent == Map("O" -> 0),
        obligations == Map("L" -> debt),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 70, "C" -> 30),
        allowanceRemaining == Map("O" -> 70),
        allowanceSpent == Map("O" -> 30),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterThirty),
        consumed == Set(("D", "O", "R-30")),
        lastEffects == thirtyExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayNearBoundCommonTest =
    init.then(seedAccount("O", 1, 1, 340282366920938463463374607431768211454))
      .then(seedBalanceOnly("C", 340282366920938463463374607431768211454))
      .then(seedObligation("L", debtNear)).then(sign(near))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 1,
          "C" -> 340282366920938463463374607431768211454),
        allowanceRemaining == Map("O" -> 1),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211454),
        obligations == Map("L" -> debtNear),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 0,
          "C" -> 340282366920938463463374607431768211455),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211455),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterOne),
        consumed == Set(("D", "O", "R-near-bound")),
        lastEffects == oneExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })
}

```

## experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts

```text
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

