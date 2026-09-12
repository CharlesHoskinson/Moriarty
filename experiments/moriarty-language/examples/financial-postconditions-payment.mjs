import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const read = (name) => readFileSync(new URL('../spec/successor/examples/' + name, import.meta.url), 'utf8');
const source = read('financial-postconditions-payment.mori');
const state = read('financial-state-payment.state.json');
const language = createFinancialAgreementSourceV4();
const checked = language.check(source);
assert.equal(checked.judgmentResult, 'SourceChecked');
assert.deepEqual(checked.actions.map((item) => item.action), ['repay', 'repay_installment', 'repay_remaining']);
assert.equal(checked.actions.find((item) => item.action === 'repay_remaining').staticWorkBound, '80');

const remaining = language.evaluate(
  source,
  'repay_remaining',
  canonical({
    Args: { allocationId: 'Alloc1', transferId: 'T1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: '256',
  }),
  state,
);
assert.equal(remaining.status, 'FundedExpressionPrepared');
assert.equal(remaining.post.paid, '100');
assert.equal(remaining.financialPost.obligations[0].principal, '0');
assert.equal(remaining.financialPost.obligations[0].accrued, '0');
assert.equal(remaining.financialPost.obligations[0].outstanding, '0');
assert.equal(remaining.financialPost.obligations[0].status, 'Settled');
assert.equal(remaining.financialPost.balances[0].amount, '0');
assert.equal(remaining.financialPost.balances[1].amount, '100');
assert.equal(remaining.financialPost.allowances[0].remaining, '0');
assert.equal(remaining.financialPost.allowances[0].spent, '100');
assert.equal(remaining.financialPost.work.remaining, '174');
assert.equal(remaining.financialPost.work.spent, '82');
assert.equal(remaining.financialPost.work.closureReserve, '16');

const exact = language.evaluate(
  source,
  'repay_remaining',
  canonical({
    Args: { allocationId: 'Alloc1', transferId: 'T1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: '82',
  }),
  JSON.stringify({ ...JSON.parse(state), work: { remaining: '82', spent: '0', closureReserve: '16' } }),
);
assert.equal(exact.status, 'FundedExpressionPrepared');
assert.equal(exact.financialPost.work.remaining, '0');
assert.equal(exact.financialPost.work.spent, '82');
assert.equal(exact.financialPost.work.closureReserve, '16');

console.log(JSON.stringify({
  sourceCheck: checked,
  remaining100: remaining,
  exact82: {
    remaining: exact.financialPost.work.remaining,
    spent: exact.financialPost.work.spent,
    reserve: exact.financialPost.work.closureReserve,
  },
}, null, 2));
