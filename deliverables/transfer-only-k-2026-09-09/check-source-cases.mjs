import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { prepareSuccessor } from '../../experiments/moriarty-language/src/successor/evaluate.ts';

// Exercise the real source parser/elaborator as a separate path to every K fixture.
// Expected results were independently authored from the financial contract; never generated here.
const cases = JSON.parse(readFileSync(new URL('../../experiments/moriarty-language/formal/k/fixtures/transfer-only.json', import.meta.url), 'utf8'));
assert.equal(cases.length, 16);

const observations = [];
for (const c of cases) {
  const { state, actions } = c.input;
  const [transfer, repay] = actions;
  assert.equal(transfer.kind, 'Transfer');
  if (repay) assert.equal(repay.kind, 'Repay');
  const obligation = state.obligations[0];
  const parties = new Set([
    ...state.balances.map(b => b.party), ...state.allowances.map(a => a.party),
    obligation.debtor, obligation.creditor, transfer.from, transfer.to, ...(repay ? [repay.payer] : []),
  ]);
  const assets = new Set([...state.balances.map(b => b.asset), ...state.allowances.map(a => a.asset), obligation.settlementAsset, transfer.asset]);
  const source = `profile "moriarty-successor-syntax/0";
agreement KComparison {
  unit ${obligation.denomination};
  ${[...parties].map(p => `party ${p};`).join('\n  ')}
  ${[...assets].map(a => `asset ${a}: Asset<${obligation.denomination}>;`).join('\n  ')}
  action pay() {
    emit Transfer { id: TransferId(${JSON.stringify(transfer.id)}), from: ${transfer.from}, to: ${transfer.to}, settlementAsset: ${transfer.asset}, amount: amount(${transfer.amount}, ${transfer.asset}) };
    ${repay ? `emit Repay { allocationId: AllocationId(${JSON.stringify(repay.allocationId)}), transferId: TransferId(${JSON.stringify(repay.transferId)}), obligationId: ObligationId(${JSON.stringify(repay.obligationId)}), payer: ${repay.payer}, nominalAmount: debt(${repay.nominalAmount}, ${obligation.denomination}) };` : ''}
  }
}`;
  const invocation = { schemaVersion: 'moriarty-funded-source/0', action: 'pay', arguments: {}, state };
  const result = prepareSuccessor(source, JSON.stringify(invocation));
  assert.deepEqual(result, c.expected, c.id);
  observations.push({ id: c.id, source, invocation, result });
}
if (process.argv[2]) writeFileSync(process.argv[2], JSON.stringify({ observations }, null, 2) + '\n');
console.log(`${observations.length} full source observations match the independent K fixture expectations; no K execution performed by this command.`);
