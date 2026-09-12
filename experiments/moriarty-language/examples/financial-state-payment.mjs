import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { createFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const read = (name) => readFileSync(new URL('../spec/successor/examples/' + name, import.meta.url), 'utf8');
const source = read('financial-state-payment.mori');
const snapshots = read('financial-state-payment.snapshots.json');
const state = read('financial-state-payment.state.json');
const language = createFinancialAgreementSourceV3();
const checked = language.check(source);
assert.equal(checked.judgmentResult, 'SourceChecked');
assert.deepEqual(checked.actions.map((item) => item.action), ['repay', 'repay_installment', 'repay_remaining']);

const first = language.evaluate(source, 'repay', snapshots, state);
assert.equal(first.status, 'FundedExpressionPrepared');
assert.equal(first.post.paid, '30');
assert.equal(first.financialPost.obligations[0].outstanding, '70');

const second = language.evaluate(
  source,
  'repay_installment',
  canonical({
    Args: { allocationId: 'Alloc2', transferId: 'T2' },
    Obs: {},
    Pre: first.post,
    workInitial: first.financialPost.work.remaining,
  }),
  JSON.stringify(first.financialPost),
);
assert.equal(second.status, 'FundedExpressionPrepared');
assert.equal(second.post.paid, '50');
assert.equal(second.financialPost.obligations[0].outstanding, '50');

const third = language.evaluate(
  source,
  'repay_remaining',
  canonical({
    Args: { allocationId: 'Alloc3', transferId: 'T3' },
    Obs: {},
    Pre: second.post,
    workInitial: second.financialPost.work.remaining,
  }),
  JSON.stringify(second.financialPost),
);
assert.equal(third.status, 'FundedExpressionPrepared');
assert.equal(third.post.paid, '100');
assert.equal(third.financialPost.obligations[0].outstanding, '0');
assert.equal(third.financialPost.obligations[0].status, 'Settled');
assert.equal(third.financialPost.balances[0].amount, '0');
assert.equal(third.financialPost.balances[1].amount, '100');
assert.equal(third.financialPost.allowances[0].remaining, '0');
assert.equal(third.financialPost.allowances[0].spent, '100');

const fourth = language.evaluate(
  source,
  'repay_remaining',
  canonical({
    Args: { allocationId: 'Alloc4', transferId: 'T4' },
    Obs: {},
    Pre: third.post,
    workInitial: third.financialPost.work.remaining,
  }),
  JSON.stringify(third.financialPost),
);
assert.equal(fourth.status, 'Rejected');
assert.equal(fourth.code, 'GUARD_FAILED');
assert.equal('post' in fourth, false);

console.log(JSON.stringify({
  sourceCheck: checked,
  repay30: first,
  installment20: second,
  remaining50: third,
  settledGuard: fourth,
}, null, 2));
