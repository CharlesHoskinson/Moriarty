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
