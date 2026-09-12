import test from 'node:test';
import assert from 'node:assert/strict';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { createFundedFinancialExpressionSourceV1 } from '../src/successor/funded-expression-source-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = 'moriarty-financial-expression-source/1';
const UINT128_MAX = '340282366920938463463374607431768211455';
const SIGNED128_MAX_PLUS_ONE = (1n << 127n).toString();

const transferFields = {
  from: ['Text'],
  id: ['Text'],
  settlementAsset: ['Text'],
  to: ['Text'],
  transferAmount: ['Amount', 'Cash'],
};
const repayFields = {
  allocationId: ['Text'],
  nominalAmount: ['Quantity', [['Cash', '1']], '0'],
  obligationId: ['Text'],
  payer: ['Text'],
  transferId: ['Text'],
};

function schema(patch = {}) {
  return canonical({
    args: {},
    assets: ['Cash'],
    enumTypes: {},
    fields: {
      due: { type: ['UInt128'], writeClass: 'ordinary' },
      paid: { type: ['UInt128'], writeClass: 'ordinary' },
    },
    observations: {},
    operations: { Repay: 'RepayFields', Transfer: 'TransferFields' },
    parties: [],
    recordTypes: { RepayFields: repayFields, TransferFields: transferFields },
    units: ['Cash'],
    variantTypes: {},
    vaults: [],
    ...patch,
  });
}

function source(body) {
  return `profile "${P}"; agreement Loan { action pay() { ${body} } }`;
}

function transferEmit(id, amountExpr, settlement = 'Cash') {
  return `emit Transfer { id: "${id}", from: "Payer", to: "Lender", settlementAsset: "${settlement}", transferAmount: ${amountExpr} };`;
}

function repayEmit(allocationId, transferId, obligationId, nominal) {
  return `emit Repay { allocationId: "${allocationId}", transferId: "${transferId}", obligationId: "${obligationId}", payer: "Payer", nominalAmount: quantity<Units<Cash,1>,0>(${nominal}) };`;
}

function computedPay(numerator, denominator, literal, transferId = 'T1', allocationId = 'Alloc1') {
  return [
    'requires pre.due > 0;',
    `let payment = floor_div(pre.due * ${numerator}, ${denominator});`,
    `requires payment == ${literal};`,
    'next.paid = pre.paid + payment;',
    transferEmit(transferId, 'amount<Cash>(payment)'),
    repayEmit(allocationId, transferId, 'Due100', literal),
    'ensures post.paid == pre.paid + payment;',
  ].join('');
}

function snapshot(workInitial, Pre = { due: '100', paid: '0' }) {
  return canonical({ Args: {}, Obs: {}, Pre, workInitial });
}

function obligation(id, extra = {}) {
  return {
    id,
    debtor: 'Payer',
    creditor: 'Lender',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    principal: '100',
    accrued: '0',
    outstanding: '100',
    allocationRule: 'AccrualFirst',
    conversion: { mantissa: '1', scale: '0', rounding: 'none' },
    status: 'Outstanding',
    ...extra,
  };
}

function state(extra = {}) {
  const work = { remaining: '100', spent: '0', closureReserve: '16', ...(extra.work ?? {}) };
  const { work: _ignored, ...rest } = extra;
  return {
    balances: [
      { party: 'Payer', asset: 'Cash', amount: '100' },
      { party: 'Lender', asset: 'Cash', amount: '0' },
      { party: 'Unrelated', asset: 'Cash', amount: '5' },
    ],
    allowances: [{ party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' }],
    obligations: [
      obligation('Due100'),
      obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }),
    ],
    usedTransferIds: ['Earlier'],
    usedAllocationIds: ['Prior'],
    work,
    ...rest,
  };
}

function stateJSON(extra = {}) {
  return JSON.stringify(state(extra));
}

function language(sch = schema()) {
  return createFundedFinancialExpressionSourceV1(sch);
}

function evaluate(s, snap, st, sch = schema()) {
  return language(sch).evaluate(s, snap, st);
}

function expressionUsed(s, workInitial, sch = schema()) {
  const result = createFinancialExpressionSourceV1(sch).evaluate(s, snapshot(workInitial));
  assert.equal(result.status, 'ExpressionPrepared', JSON.stringify(result));
  return BigInt(workInitial) - BigInt(result.workRemaining);
}

function assertRejected(result) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal('post' in result, false, JSON.stringify(result));
  assert.equal('financialPost' in result, false, JSON.stringify(result));
  assert.equal('descriptors' in result, false, JSON.stringify(result));
  assert.equal('effects' in result, false, JSON.stringify(result));
  return result;
}

function assertWork(result, remainingBefore, spentBefore, reserveBefore, E, N) {
  const remainingAfter = remainingBefore - E - N;
  const spentAfter = spentBefore + E + N;
  assert.equal(result.workRemaining, remainingAfter.toString());
  assert.equal(result.financialPost.work.remaining, remainingAfter.toString());
  assert.equal(result.financialPost.work.spent, spentAfter.toString());
  assert.equal(result.financialPost.work.closureReserve, reserveBefore);
}

test('computed due100/pay30 publishes ordinary post, cash, principal and allowance together', () => {
  const s = source(computedPay(3, 10, 30));
  const remaining = 100n, spent = 0n, reserve = '16', N = 2n;
  const E = expressionUsed(s, remaining.toString());
  const result = evaluate(s, snapshot(remaining.toString()), stateJSON());
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.deepEqual(result.post, { due: '100', paid: '30' });
  assert.equal(result.financialPost.balances[0].amount, '70');
  assert.equal(result.financialPost.balances[1].amount, '30');
  assert.equal(result.financialPost.balances[2].amount, '5');
  assert.equal(result.financialPost.obligations[0].principal, '70');
  assert.equal(result.financialPost.obligations[0].outstanding, '70');
  assert.equal(result.financialPost.allowances[0].remaining, '70');
  assert.equal(result.financialPost.allowances[0].spent, '30');
  assertWork(result, remaining, spent, reserve, E, N);
  assert.equal(result.effects[0].kind, 'Transfer');
  assert.equal(result.effects[0].amount, '30');
  assert.equal(result.effects[1].kind, 'Repayment');
  assert.equal(result.effects[1].principalDischarged, '30');
});

test('AccrualFirst P100/I10/pay7 discharges accrued first and leaves residual duty', () => {
  const s = source(computedPay(7, 100, 7));
  const remaining = 100n, spent = 0n, reserve = '16', N = 2n;
  const E = expressionUsed(s, remaining.toString());
  const st = stateJSON({
    obligations: [
      obligation('Due100', { principal: '100', accrued: '10', outstanding: '110' }),
      obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }),
    ],
  });
  const result = evaluate(s, snapshot(remaining.toString()), st);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.obligations[0].principal, '100');
  assert.equal(result.financialPost.obligations[0].accrued, '3');
  assert.equal(result.financialPost.obligations[0].outstanding, '103');
  assert.equal(result.financialPost.balances[0].amount, '93');
  assert.equal(result.financialPost.balances[1].amount, '7');
  assert.equal(result.effects[1].principalDischarged, '0');
  assert.equal(result.effects[1].accruedDischarged, '7');
  assertWork(result, remaining, spent, reserve, E, N);
});

test('arithmetic changes the Transfer amount and a failing equality guard rejects Repay', () => {
  const half = source([
    'requires pre.due > 0;',
    'let payment = floor_div(pre.due, 2);',
    'next.paid = pre.paid + payment;',
    transferEmit('T1', 'amount<Cash>(payment)'),
    repayEmit('Alloc1', 'T1', 'Due100', 30),
    'ensures post.paid == payment;',
  ].join(''));
  const halfResult = evaluate(half, snapshot('100'), stateJSON());
  assert.equal(halfResult.status, 'FundedExpressionPrepared', JSON.stringify(halfResult));
  assert.equal(halfResult.effects[0].amount, '50');
  assert.equal(halfResult.effects[1].nominalAmount, '30');
  assert.equal(halfResult.financialPost.balances[0].amount, '50');
  assert.equal(halfResult.financialPost.obligations[0].principal, '70');

  const guarded = source(computedPay(1, 2, 30));
  const rejected = assertRejected(evaluate(guarded, snapshot('100'), stateJSON()));
  assert.equal(rejected.code, 'GUARD_FAILED');
});

test('failed guard or ensure after staged write or emission returns rejection only', () => {
  const afterWrite = source('next.paid = 1; requires false;');
  const guard = assertRejected(evaluate(afterWrite, snapshot('100'), stateJSON()));
  assert.equal(guard.code, 'GUARD_FAILED');

  const afterEmit = source([
    'next.paid = 1;',
    transferEmit('T1', 'amount<Cash>(30)'),
    repayEmit('Alloc1', 'T1', 'Due100', 30),
    'ensures false;',
  ].join(''));
  const ensure = assertRejected(evaluate(afterEmit, snapshot('100'), stateJSON()));
  assert.equal(ensure.code, 'ENSURES_FAILED');
});

test('overflow, negative nominal, wrong units, unsupported operations, missing and repeated funding reject atomically', () => {
  const negative = source(transferEmit('T1', 'amount<Cash>(30)') + repayEmit('Alloc1', 'T1', 'Due100', '-1'));
  const negativeResult = assertRejected(evaluate(negative, snapshot('100'), stateJSON()));
  assert.equal(negativeResult.code, 'NOMINAL_RANGE');

  const overflowSource = source(`let x = quantity<Units<Cash,1>,0>(${SIGNED128_MAX_PLUS_ONE});`);
  const overflow = assertRejected(evaluate(overflowSource, snapshot('100'), stateJSON()));
  assert.ok(['VALUE_BOUND', 'TYPE_LITERAL', 'INPUT_VALUE', 'ARITH_RANGE'].includes(overflow.code), overflow.code);

  const wrongSettlement = source(
    transferEmit('T1', 'amount<Cash>(30)', 'USD') + repayEmit('Alloc1', 'T1', 'Due100', 30),
  );
  const settlement = assertRejected(evaluate(wrongSettlement, snapshot('100'), stateJSON()));
  assert.equal(settlement.code, 'SETTLEMENT_UNIT');

  const wrongUnit = stateJSON({
    obligations: [
      obligation('Due100', { denomination: 'USD' }),
      obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }),
    ],
  });
  const unit = assertRejected(evaluate(source(computedPay(3, 10, 30)), snapshot('100'), wrongUnit));
  assert.equal(unit.code, 'NOMINAL_UNIT');

  const noticeSchema = schema({
    operations: { Notice: 'NoticeFields', Repay: 'RepayFields', Transfer: 'TransferFields' },
    recordTypes: {
      NoticeFields: { n: ['UInt128'] },
      RepayFields: repayFields,
      TransferFields: transferFields,
    },
  });
  const unsupported = assertRejected(evaluate(source('emit Notice { n: 1 };'), snapshot('100'), stateJSON(), noticeSchema));
  assert.equal(unsupported.code, 'OPERATION_BINDING');

  const missing = source(transferEmit('T1', 'amount<Cash>(20)') + repayEmit('Alloc1', 'T1', 'Due100', 30));
  const unfunded = assertRejected(evaluate(missing, snapshot('100'), stateJSON()));
  assert.equal(unfunded.code, 'INSUFFICIENT_UNALLOCATED');

  const reused = assertRejected(evaluate(source(computedPay(3, 10, 30)), snapshot('100'), stateJSON({
    usedTransferIds: ['Earlier', 'T1'],
  })));
  assert.equal(reused.code, 'DUPLICATE');
});

test('work is exact at E+N, one unit short rejects, prior spent is kept and reserve is unchanged', () => {
  const s = source(computedPay(3, 10, 30));
  const E = expressionUsed(s, '100');
  const N = 2n;
  const exactRemaining = (E + N).toString();
  const exact = evaluate(s, snapshot(exactRemaining), stateJSON({
    work: { remaining: exactRemaining, spent: '7', closureReserve: '16' },
  }));
  assert.equal(exact.status, 'FundedExpressionPrepared', JSON.stringify(exact));
  assertWork(exact, E + N, 7n, '16', E, N);

  const shortRemaining = (E + N - 1n).toString();
  const short = assertRejected(evaluate(s, snapshot(shortRemaining), stateJSON({
    work: { remaining: shortRemaining, spent: '7', closureReserve: '16' },
  })));
  assert.equal(short.code, 'INSUFFICIENT_WORK');

  const carried = evaluate(s, snapshot('100'), stateJSON({
    work: { remaining: '100', spent: '11', closureReserve: '16' },
  }));
  assert.equal(carried.status, 'FundedExpressionPrepared', JSON.stringify(carried));
  assertWork(carried, 100n, 11n, '16', E, N);
});

test('multiple allocations share one transfer decreasing funding', () => {
  const s = source([
    'let payment = floor_div(pre.due, 2);',
    'requires payment == 50;',
    'next.paid = payment;',
    transferEmit('T1', 'amount<Cash>(payment)'),
    repayEmit('Alloc1', 'T1', 'Due100', 20),
    repayEmit('Alloc2', 'T1', 'Due100', 30),
    'ensures post.paid == 50;',
  ].join(''));
  const E = expressionUsed(s, '100');
  const result = evaluate(s, snapshot('100'), stateJSON());
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.balances[0].amount, '50');
  assert.equal(result.financialPost.obligations[0].principal, '50');
  assert.equal(result.effects.length, 3);
  assert.equal(result.effects[1].nominalAmount, '20');
  assert.equal(result.effects[2].nominalAmount, '30');
  assertWork(result, 100n, 0n, '16', E, 3n);
});

test('second invocation preserves prior spent, IDs, unrelated state and residual duties', () => {
  const firstSource = source(computedPay(3, 10, 30));
  const first = evaluate(firstSource, snapshot('100'), stateJSON({
    work: { remaining: '100', spent: '4', closureReserve: '16' },
  }));
  assert.equal(first.status, 'FundedExpressionPrepared', JSON.stringify(first));
  const E1 = expressionUsed(firstSource, '100');
  assertWork(first, 100n, 4n, '16', E1, 2n);
  assert.deepEqual(first.financialPost.obligations[1], obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }));
  assert.deepEqual(first.financialPost.usedTransferIds, ['Earlier', 'T1']);
  assert.deepEqual(first.financialPost.usedAllocationIds, ['Prior', 'Alloc1']);

  const secondSource = source(computedPay(1, 5, 20, 'T2', 'Alloc2'));
  const remaining = first.financialPost.work.remaining;
  const secondSnap = snapshot(remaining, { due: '100', paid: '30' });
  const second = evaluate(secondSource, secondSnap, JSON.stringify(first.financialPost));
  assert.equal(second.status, 'FundedExpressionPrepared', JSON.stringify(second));
  const E2 = expressionUsed(secondSource, remaining);
  assert.equal(second.post.paid, '50');
  assert.equal(second.financialPost.obligations[0].principal, '50');
  assert.deepEqual(second.financialPost.obligations[1], obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }));
  assert.deepEqual(second.financialPost.usedTransferIds, ['Earlier', 'T1', 'T2']);
  assert.deepEqual(second.financialPost.usedAllocationIds, ['Prior', 'Alloc1', 'Alloc2']);
  assertWork(second, BigInt(remaining), BigInt(first.financialPost.work.spent), '16', E2, 2n);
});

test('workInitial must equal remaining, financial schema fields are rejected, and empty batches never reach the kernel', () => {
  const s = source(computedPay(3, 10, 30));
  const mismatch = assertRejected(evaluate(s, snapshot('1000'), stateJSON()));
  assert.equal(mismatch.code, 'WORK_MISMATCH');

  const financialSchema = schema({
    fields: {
      due: { type: ['UInt128'], writeClass: 'ordinary' },
      paid: { type: ['UInt128'], writeClass: 'ordinary' },
      funds: { type: ['UInt128'], writeClass: 'financial' },
    },
  });
  const financial = assertRejected(evaluate(source('next.paid = 1;'), snapshot('100', { due: '100', paid: '0', funds: '8' }), stateJSON(), financialSchema));
  assert.equal(financial.code, 'TYPE_FINANCIAL_WRITE');

  const empty = assertRejected(evaluate(source('next.paid = 1;'), snapshot('100'), stateJSON()));
  assert.equal(empty.code, 'EMPTY_BATCH');
});

test('adapter owns parsed JSON only and does not publish on malformed state or hostile accessors', () => {
  const s = source(computedPay(3, 10, 30));
  assertRejected(evaluate(s, snapshot('100'), '{'));
  let touched = false;
  const hostile = { get remaining() { touched = true; throw new Error('read'); } };
  const result = language().evaluate(s, snapshot('100'), hostile);
  assertRejected(result);
  assert.equal(touched, false);
  assert.equal(result.code, 'INPUT_SCHEMA');
});
