import assert from 'node:assert/strict';

export function checkO1Accounts(frozen,scenario,before,after) {
  for (const balances of [before,after]) {
    assert.equal(balances.length,frozen.accountOrder.length);
    for (const pair of balances) {
      assert.equal(pair.length,frozen.assetOrder.length);
      assert.ok(pair.every(value=>Number.isSafeInteger(value)&&value>=0));
    }
    const totals = frozen.assetOrder.map((_,i)=>balances.reduce((sum,pair)=>sum+pair[i],0));
    assert.deepEqual(totals,frozen.supply,'A/B supply');
  }
  if (scenario !== 'rejected') assert.deepEqual(before,frozen.funded,'pre-fill balances');
  const changes = before.map((pair,i)=>pair.map((value,j)=>after[i][j]-value));
  assert.deepEqual(changes,frozen[scenario].change,'complete '+scenario+' A/B changes');
}
