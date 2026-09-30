"""Independent literal expectations; never imports or invokes the JS codec/Core."""
import copy
import hashlib
import json
import struct
from pathlib import Path

HERE = Path(__file__).parent
HEAD, NEXT = '55' * 32, 'aa' * 32
PREMISES = ['canonical-intent-signature', 'snapshot-to-head', 'head-extension',
            'atomic-ledger-compare-and-consume']
U = '340282366920938463463374607431768211455'
UM = '340282366920938463463374607431768211454'
S = '170141183460469231731687303715884105727'
SM = '170141183460469231731687303715884105726'


def replay(nonce, domain='D'):
    return json.dumps([domain, 'O', nonce], separators=(',', ':'))


def debt(p, a, o, status='Outstanding'):
    return dict(id='L', debtor='O', creditor='C', asset='A', principal=p,
                accrued=a, outstanding=o, status=status)


def make_case(name, nonce_byte, op, money_before, money_after, allowance_before,
              allowance_after, debt_before, debt_after, lines):
    nonce = nonce_byte * 32
    key = replay(nonce)
    state = dict(core='moriarty-core/5', domain='D', asset='A', head=HEAD,
                 round='100', workRemaining='2', workSpent='7',
                 balances=[dict(account=k, amount=v) for k, v in money_before],
                 allowances=[dict(owner='O', remaining=allowance_before[0],
                                  spent=allowance_before[1])],
                 obligations=debt_before, consumedReplay=[])
    post = copy.deepcopy(state)
    post.update(head=NEXT, workRemaining='1', workSpent='8',
                balances=[dict(account=k, amount=v) for k, v in money_after],
                allowances=[dict(owner='O', remaining=allowance_after[0],
                                 spent=allowance_after[1])],
                obligations=debt_after, consumedReplay=[key])
    amount = '11' if name == 'transfer-fee' else op['amount']
    effects = copy.deepcopy(lines) + [dict(kind='UseAllowance', owner='O', amount=amount),
               dict(kind='UseReplay', key=key),
               dict(kind='AdvanceHead', predecessor=HEAD, successor=NEXT)]
    obligation_rows = []
    for before, after in zip(debt_before, debt_after, strict=True):
        row = {k: before[k] for k in ['id', 'debtor', 'creditor', 'asset']}
        for suffix, cell in [('Before', before), ('After', after)]:
            row.update({k + suffix: cell[k] for k in ['principal', 'accrued', 'outstanding', 'status']})
        obligation_rows.append(row)
    prepared = dict(schemaVersion='moriarty-s0-effects/1', core='moriarty-core/5',
        domain='D', asset='A', scale=2, operationKind=op['kind'], round='100',
        preHead=HEAD, successor=NEXT, effects=effects,
        footprint=dict(balances=[dict(account=b[0], before=b[1], after=a[1])
                                for b, a in zip(money_before, money_after, strict=True)],
            allowances=[dict(owner='O', remainingBefore=allowance_before[0],
                spentBefore=allowance_before[1], remainingAfter=allowance_after[0],
                spentAfter=allowance_after[1])], obligations=obligation_rows),
        consumption=dict(workRemainingBefore='2', workSpentBefore='7',
            workRemainingAfter='1', workSpentAfter='8', replayBefore=[], replayAfter=[key]),
        requiredPremises=PREMISES)
    auth = dict(schemaVersion='moriarty-intent/3', profile='s0-provisional/1',
        domain='D', agreementId='Agreement', stageId='Stage', episodeId='Episode',
        actionId='Action', sourceVersion=6, sourceHash='22'*32, coreVersion=5,
        coreProgramId='TransferLiteralFee' if op['kind']=='transfer' else 'RepayAccrualFirst',
        coreHash='33'*32, policyHash='44'*32, signer='O', keyScheme='schnorr_bip340',
        signerKey='11'*32, asset='A', scale=2, preHead=HEAD, predecessor='66'*32,
        nonce=nonce, validFrom='100', validUntil='200', grossCap=amount,
        feeCap=op.get('fee', '0'), netFloor=op['amount'] if op['kind']=='transfer' else '0',
        effectCommitment='00'*32, failurePolicy='atomic-reject-terminal-success',
        supplyChanges=[], observations=[], disclosures=[], retainedEffects=[],
        retainedDuties=[], delegation='none', recovery='none', operation=op)
    intent = dict(version='moriarty-intent/3', core='moriarty-core/5',
        sourceProfile='moriarty-financial-agreement-source/6',
        programId=auth['coreProgramId'], sourceHash=auth['sourceHash'],
        policyDigest=auth['policyHash'], keyRef=auth['signerKey'], domain='D', asset='A',
        signer='O', nonce=nonce, preHead=HEAD, notBefore='100', notAfter='200',
        grossCap=amount, feeCap=auth['feeCap'], netFloor=auth['netFloor'], amount=op['amount'])
    if op['kind'] == 'transfer':
        intent.update(kind='Transfer', recipient='R', feeRecipient='F', fee=op['fee'])
    else:
        intent.update(kind='Repay', obligationId='L')
    return dict(id=name, prepared=prepared, authorization=auth, state=state,
                coreIntent=intent, expectedPost=post)


def debit(n):
    return dict(kind='Debit', account='O', asset='A', amount=n)


def credit(who, n):
    return dict(kind='Credit', account=who, asset='A', amount=n)


def set_debt(p, a, o, status='Outstanding'):
    return dict(kind='SetObligation', id='L', principal=p, accrued=a,
                outstanding=o, status=status)


cases = [
 make_case('transfer-fee', '77', dict(kind='transfer', owner='O', recipient='R',
    feeRecipient='F', amount='10', fee='1'), [('O','100'),('R','0'),('F','0')],
    [('O','89'),('R','10'),('F','1')], ('11','0'), ('0','11'), [], [],
    [debit('11'), credit('R','10'), credit('F','1')]),
 make_case('transfer-zero-fee', '78', dict(kind='transfer', owner='O', recipient='R',
    feeRecipient='F', amount='10', fee='0'), [('O','100'),('R','0'),('F','4')],
    [('O','90'),('R','10'),('F','4')], ('20','3'), ('10','13'), [], [],
    [debit('10'), credit('R','10')]),
]
for name, nonce, amount, before, after, bal_pre, bal_post, allowance_pre, allowance_post in [
 ('repay-30','79','30',debt('1000','10','1010'),debt('980','0','980'),
  [('O','100'),('C','0')],[('O','70'),('C','30')],('100','0'),('70','30')),
 ('repay-accrued-only','7a','5',debt('1000','10','1010'),debt('1000','5','1005'),
  [('O','100'),('C','0')],[('O','95'),('C','5')],('100','0'),('95','5')),
 ('repay-settled','7b','1010',debt('1000','10','1010'),debt('0','0','0','Settled'),
  [('O','1010'),('C','0')],[('O','0'),('C','1010')],('1010','0'),('0','1010')),
 ('repay-near-bound','7c','1',debt(SM,'1',S),debt(SM,'0',SM),
  [('O','1'),('C',UM)],[('O','0'),('C',U)],('1',UM),('0',U)),
]:
    op = dict(kind='repayment', obligationId='L', payer='O', debtor='O', creditor='C',
              amount=amount, allocation='AccrualFirst', conversion='identity')
    cases.append(make_case(name, nonce, op, bal_pre, bal_post, allowance_pre,
        allowance_post, [before], [after], [debit(amount), credit('C',amount),
        set_debt(after['principal'],after['accrued'],after['outstanding'],after['status'])]))


# Separate reference byte construction. Inputs above are independent literals.
def count(n): return struct.pack('>H', n)
def ident(s):
    b = s.encode('ascii')
    return count(len(b)) + b
def number(n, width=16): return int(n).to_bytes(width, 'big')
def raw(s): return bytes.fromhex(s)
def status(s): return bytes([{'Outstanding':1, 'Settled':2}[s]])
def replay_bytes(s):
    d, owner, nonce = json.loads(s)
    return ident(d) + ident(owner) + raw(nonce)


def reference_bytes(p):
    b = bytearray(b'moriarty-s0-effects/1\0')
    b += b'\x05' + ident(p['domain']) + ident(p['asset'])
    b += bytes([p['scale'], {'transfer':1,'repayment':2}[p['operationKind']]])
    b += number(p['round'],8) + raw(p['preHead']) + raw(p['successor'])
    b += count(len(p['effects']))
    for line in p['effects']:
        kind = line['kind']
        b += bytes([{'Debit':1,'Credit':2,'SetObligation':3,'UseAllowance':4,
                     'UseReplay':5,'AdvanceHead':6}[kind]])
        if kind in ['Debit','Credit']:
            b += ident(line['account']) + ident(line['asset']) + number(line['amount'])
        elif kind=='SetObligation':
            b += ident(line['id'])
            for k in ['principal','accrued','outstanding']: b += number(line[k])
            b += status(line['status'])
        elif kind=='UseAllowance': b += ident(line['owner']) + number(line['amount'])
        elif kind=='UseReplay': b += replay_bytes(line['key'])
        else: b += raw(line['predecessor']) + raw(line['successor'])
    f = p['footprint']
    b += count(len(f['balances']))
    for row in f['balances']:
        b += ident(row['account']) + number(row['before']) + number(row['after'])
    b += count(len(f['allowances']))
    for row in f['allowances']:
        b += ident(row['owner'])
        for k in ['remainingBefore','spentBefore','remainingAfter','spentAfter']: b += number(row[k])
    b += count(len(f['obligations']))
    for row in f['obligations']:
        for k in ['id','debtor','creditor','asset']: b += ident(row[k])
        for suffix in ['Before','After']:
            for k in ['principal','accrued','outstanding']: b += number(row[k+suffix])
            b += status(row['status'+suffix])
    c = p['consumption']
    for k in ['workRemainingBefore','workSpentBefore','workRemainingAfter','workSpentAfter']: b += number(c[k])
    for k in ['replayBefore','replayAfter']:
        b += count(len(c[k]))
        for key in c[k]: b += replay_bytes(key)
    if p['requiredPremises'] != PREMISES: raise ValueError('LITERAL')
    b += b'\x0f'
    return bytes(b)


def expected(p):
    wire = reference_bytes(p)
    return dict(length=len(wire), wireHex=wire.hex(), commitment=hashlib.sha256(wire).hexdigest())


for case in cases:
    case['expected'] = expected(case['prepared'])
    case['authorization']['effectCommitment'] = case['expected']['commitment']

hostile = []
def mutation(name, base, change, code='EFFECT_COMMITMENT_MISMATCH'):
    p = copy.deepcopy(cases[base]['prepared'])
    change(p)
    record = dict(id=name, base=cases[base]['id'], prepared=p, expectedCode=code)
    if code == 'EFFECT_COMMITMENT_MISMATCH': record['expected'] = expected(p)
    hostile.append(record)

mutation('changed-debit',0,lambda p:p['effects'][0].update(amount='12'))
mutation('dropped-line',0,lambda p:p['effects'].pop(2))
mutation('appended-line',1,lambda p:p['effects'].append(credit('F','1')))
mutation('line-order',0,lambda p:p['effects'].reverse())
mutation('recipient',0,lambda p:p['effects'][1].update(account='X'))
mutation('allowance-line',0,lambda p:p['effects'][3].update(amount='10'))
mutation('work-counter',0,lambda p:p['consumption'].update(workSpentAfter='9'))
mutation('omitted-footprint',0,lambda p:p['footprint']['balances'].pop(2))
mutation('balance-footprint',0,lambda p:p['footprint']['balances'][0].update(after='88'))
mutation('allowance-prestate',0,lambda p:p['footprint']['allowances'][0].update(spentBefore='1'))
mutation('obligation-debtor',2,lambda p:p['footprint']['obligations'][0].update(debtor='X'))
mutation('principal-allocation',2,lambda p:p['effects'][2].update(principal='990',accrued='0',outstanding='990'))
mutation('replay-line',0,lambda p:p['effects'][4].update(key=replay('ee'*32)))
mutation('replay-history',0,lambda p:p['consumption']['replayBefore'].append(replay('ee'*32)))
mutation('replay-domain',0,lambda p:p['effects'][4].update(key=replay('77'*32,'X')))
mutation('root-head',0,lambda p:p.update(preHead='bb'*32))
mutation('successor-line',0,lambda p:p['effects'][5].update(successor='bb'*32))
mutation('footprint-order',0,lambda p:p['footprint']['balances'].reverse())
mutation('premise-order',0,lambda p:p['requiredPremises'].reverse(),'LITERAL')

output = dict(status='W-D2E independent expectations frozen before JS codec; no acceptance',
              positive=cases, hostile=hostile)
if __name__ == '__main__':
    target = HERE / 'fixtures.json'
    text = json.dumps(output, indent=2) + '\n'
    if target.exists():
        if target.read_text() != text: raise SystemExit('Frozen fixtures differ; refusing overwrite')
        print('Frozen fixtures independently reproduced without modification')
    else:
        target.write_text(text)
        print('Frozen 6 positives and 19 hostile expected vectors')
