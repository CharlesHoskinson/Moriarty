import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { prepareSuccessor } from '../../experiments/moriarty-language/src/successor/evaluate.ts';
const source = readFileSync(new URL('../../experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori', import.meta.url), 'utf8');
const { cases } = JSON.parse(readFileSync(new URL('./independent-financial-cases.json', import.meta.url), 'utf8'));
function invocation(c) {
  return {
    schemaVersion: 'moriarty-funded-source/0', action: 'pay',
    arguments: {
      cash: { type: { kind: 'Amount', name: 'Cash' }, value: c.input.actions[0].amount },
      nominal: { type: { kind: 'Debt', name: 'Cash' }, value: c.input.actions[1].nominalAmount },
    }, state: structuredClone(c.input.state),
  };
}
const call = (s, v) => prepareSuccessor(s, JSON.stringify(v));
let count = 0;
for (const c of cases) { assert.deepEqual(call(source, invocation(c)), c.expected, c.id); count++; }
function reject(label, s, v) {
  const result = call(s, v);
  assert.equal(result.status, 'Rejected', label);
  assert.equal(Object.hasOwn(result, 'post'), false, label + ' must not expose post');
  assert.equal(Object.hasOwn(result, 'effects'), false, label + ' must not expose effects');
  count++;
}
const base = () => invocation(cases[0]);
let v = base(); v.arguments.nominal.type.name = 'Other'; reject('wrong argument unit', source, v);
v = base(); v.state.obligations[0].denomination = 'Other'; reject('runtime obligation denomination differs', source, v);
v = base(); v.state.obligations[0].settlementAsset = 'Other'; reject('runtime settlement asset differs', source, v);
v = base(); v.state.allowances[0].remaining = '29'; reject('gross allowance insufficient', source, v);
v = base(); v.state.work.remaining = '1'; reject('closure reserve is not ordinary work', source, v);
v = base(); v.state.balances[0].amount = '29'; reject('payer cash insufficient', source, v);
v = base(); v.arguments.nominal.value = '31'; reject('late repayment lacks funding', source, v);
v = base(); v.state.usedTransferIds = ['T1']; reject('transfer replay', source, v);
v = base(); v.state.usedAllocationIds = ['Alloc1']; reject('allocation replay', source, v);
v = base(); v.arguments.nominal.value = '030'; reject('noncanonical amount', source, v);
v = base(); v.arguments.nominal.value = '340282366920938463463374607431768211456'; reject('UInt128 overflow', source, v);
v = base(); v.arguments.extra = v.arguments.nominal; reject('unexpected argument', source, v);
v = base(); v.state.obligations[0].extra = 'unmodeled'; reject('unknown financial field', source, v);
v = base(); v.core = {}; reject('supplied Core cannot bypass source', source, v);
// A changed conversion must be honored, never replaced by identity conversion.
v = base(); v.state.obligations[0].conversion.mantissa = '2'; v.arguments.cash.value = '60';
let got = call(source, v); assert.equal(got.status, 'Prepared'); assert.equal(got.post.balances[0].amount, '40'); assert.equal(got.post.obligations[0].principal, '70'); assert.equal(got.effects[1].settlementAmount, '60'); count++;
// The full supplied projection must survive, including unrelated duties and tombstones.
v = base(); const other = { ...structuredClone(v.state.obligations[0]), id: 'Unrelated', principal: '17', outstanding: '17' };
v.state.obligations.push(other); v.state.usedTransferIds.push('EarlierTransfer'); v.state.usedAllocationIds.push('EarlierAllocation');
got = call(source, v); assert.equal(got.status, 'Prepared'); assert.deepEqual(got.post.obligations[1], other); assert.deepEqual(got.post.usedTransferIds, ['EarlierTransfer','T1']); assert.deepEqual(got.post.usedAllocationIds, ['EarlierAllocation','Alloc1']); count++;
// Source selection is executable: modifying the target recipient cannot leave a canned result.
const modified = source.replace('to: Lender', 'to: Payer');
assert.notEqual(modified, source, 'probe must modify source'); reject('source recipient mutation changes execution', modified, base());
let coerced = false; const hostile = { toString() { coerced = true; throw Error('coercion'); } };
assert.equal(prepareSuccessor(hostile, JSON.stringify(base())).status, 'Rejected'); assert.equal(coerced, false); count++;
console.log(`${count} independent source-path probes passed`);
