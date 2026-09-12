import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const read = (name) => readFileSync(new URL('../spec/successor/examples/' + name, import.meta.url), 'utf8');
const source = read('financial-lifecycle-payment.mori');
const state = read('financial-lifecycle-payment.state.json');
const language = createFinancialAgreementSourceV5();
const checked = language.check(source);
assert.equal(checked.judgmentResult, 'SourceChecked');
assert.deepEqual(checked.actions.map((item) => item.action), ['originate', 'accrue', 'repay']);

const originated = language.evaluate(
  source,
  'originate',
  read('financial-lifecycle-payment.snapshots.json'),
  state,
);
assert.equal(originated.status, 'FundedExpressionPrepared');
assert.equal(originated.financialPost.obligations[0].principal, '100');
assert.equal(originated.financialPost.obligations[0].status, 'Outstanding');
assert.equal(originated.financialPost.work.closureReserve, '16');

const accrued = language.evaluate(
  source,
  'accrue',
  canonical({
    Args: { accrualId: 'A1', observedTime: '1060', periodIndex: '1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: originated.financialPost.work.remaining,
  }),
  JSON.stringify(originated.financialPost),
);
assert.equal(accrued.status, 'FundedExpressionPrepared');
assert.equal(accrued.financialPost.obligations[0].accrued, '10');
assert.equal(accrued.financialPost.obligations[0].liabilityIncurred, '110');

const repaid = language.evaluate(
  source,
  'repay',
  canonical({
    Args: { allocationId: 'R1', nominal: '30', transferId: 'P1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: accrued.financialPost.work.remaining,
  }),
  JSON.stringify(accrued.financialPost),
);
assert.equal(repaid.status, 'FundedExpressionPrepared');
assert.equal(repaid.financialPost.obligations[0].outstanding, '80');
assert.equal(repaid.financialPost.obligations[0].liabilityIncurred, '110');
assert.equal(repaid.effects[1].principalDischarged, '20');
assert.equal(repaid.effects[1].accruedDischarged, '10');

console.log(JSON.stringify({
  sourceCheck: checked,
  originatedWork: originated.financialPost.work,
  accrued: {
    accrued: accrued.financialPost.obligations[0].accrued,
    incurred: accrued.financialPost.obligations[0].liabilityIncurred,
    work: accrued.financialPost.work,
  },
  repaid: {
    outstanding: repaid.financialPost.obligations[0].outstanding,
    incurred: repaid.financialPost.obligations[0].liabilityIncurred,
    work: repaid.financialPost.work,
  },
}, null, 2));
