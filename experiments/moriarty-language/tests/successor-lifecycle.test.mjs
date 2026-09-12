import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  prepareFinancialLifecycle,
  admitFinancialLifecycleStateJSON,
  LIFECYCLE_VERSION,
  LIFECYCLE_STATE_VERSION,
  LIFECYCLE_BOUNDS,
} from '../src/successor/financial-lifecycle.ts';
import { prepareRepayment, REPAYMENT_VERSION } from '../src/successor/repayment.ts';

const root = join(dirname(fileURLToPath(import.meta.url)), '../../..');
const oracle = JSON.parse(readFileSync(join(
  root,
  'deliverables/language-to-ledger-2026-09-12/design-review/origination-accrual-expectations.json',
), 'utf8'));
const fixtures = JSON.parse(readFileSync(join(
  root,
  'deliverables/language-to-ledger-2026-09-12/origination/result-bound-fixtures.json',
), 'utf8'));

const M = '170141183460469231731687303715884105727';
const U64 = '18446744073709551615';
const U128 = '340282366920938463463374607431768211455';

function conversion(over = {}) {
  return { mantissa: '1', scale: '0', rounding: 'none', ...over };
}
function terms(over = {}) {
  return {
    numerator: '1',
    denominator: '10',
    rounding: 'floor',
    periodSeconds: '60',
    firstPeriodStart: '1000',
    ...over,
  };
}
function loan(over = {}) {
  return {
    id: 'Loan1',
    debtor: 'Borrower',
    creditor: 'Lender',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    principal: '100',
    accrued: '0',
    outstanding: '100',
    allocationRule: 'AccrualFirst',
    conversion: conversion(),
    status: 'Outstanding',
    originationId: 'O1',
    originationTransferId: 'D1',
    initialPrincipal: '100',
    nominalLiabilityCap: '110',
    liabilityIncurred: '100',
    accrualTerms: terms(),
    lastAccruedPeriod: '0',
    nextAccrualAt: '1060',
    ...over,
  };
}
function seed(over = {}) {
  return {
    schemaVersion: LIFECYCLE_STATE_VERSION,
    balances: [
      { party: 'Lender', asset: 'Cash', amount: '100' },
      { party: 'Borrower', asset: 'Cash', amount: '10' },
      { party: 'Other', asset: 'Token', amount: '7' },
    ],
    allowances: [
      { party: 'Lender', asset: 'Cash', remaining: '100', spent: '0' },
      { party: 'Borrower', asset: 'Cash', remaining: '110', spent: '0' },
      { party: 'Other', asset: 'Token', remaining: '3', spent: '1' },
    ],
    obligations: [],
    usedTransferIds: [],
    usedAllocationIds: [],
    usedOriginationIds: [],
    usedAccrualIds: [],
    work: { remaining: '256', spent: '17', closureReserve: '16' },
    ...over,
  };
}
function transfer(over = {}) {
  return { kind: 'Transfer', id: 'D1', from: 'Lender', to: 'Borrower', asset: 'Cash', amount: '100', ...over };
}
function originate(over = {}) {
  return {
    kind: 'Originate',
    obligationId: 'Loan1',
    transferId: 'D1',
    originationId: 'O1',
    debtor: 'Borrower',
    creditor: 'Lender',
    nominalAmount: '100',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    conversion: conversion(),
    allocationRule: 'AccrualFirst',
    accrualTerms: terms(),
    nominalLiabilityCap: '110',
    ...over,
  };
}
function accrue(over = {}) {
  return {
    kind: 'Accrue',
    accrualId: 'A1',
    obligationId: 'Loan1',
    periodIndex: '1',
    observedTime: '1060',
    ...over,
  };
}
function repay(over = {}) {
  return {
    kind: 'Repay',
    allocationId: 'R1',
    transferId: 'P1',
    obligationId: 'Loan1',
    payer: 'Borrower',
    nominalAmount: '30',
    ...over,
  };
}
function payTransfer(over = {}) {
  return { kind: 'Transfer', id: 'P1', from: 'Borrower', to: 'Lender', asset: 'Cash', amount: '30', ...over };
}
function input(state, actions, extra = {}) {
  return JSON.stringify({
    schemaVersion: extra.schemaVersion ?? LIFECYCLE_VERSION,
    state,
    actions,
    ...extra.extraRoot,
  });
}
function run(state, actions, extra = {}) {
  return prepareFinancialLifecycle(input(state, actions, extra));
}
function assertRejected(result, actionIndex) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.actionIndex, actionIndex);
  assert.equal(typeof result.code, 'string');
  assert.ok(result.code.length > 0);
  assert.deepEqual(Object.keys(result).sort(), ['actionIndex', 'code', 'status']);
  assert.equal('post' in result, false);
  assert.equal('effects' in result, false);
  assert.equal('financialPost' in result, false);
}
function assertPrepared(result) {
  assert.equal(result.status, 'Prepared', JSON.stringify(result));
  assert.equal(result.schemaVersion, LIFECYCLE_VERSION);
  assert.deepEqual(Object.keys(result).sort(), ['effects', 'post', 'schemaVersion', 'status']);
  const admitted = admitFinancialLifecycleStateJSON(JSON.stringify(result.post));
  assert.equal(admitted.ok, true, JSON.stringify(admitted));
  assert.deepEqual(admitted.value, result.post);
}

describe('lifecycle exports', () => {
  test('versions, bounds and factory arity', () => {
    assert.equal(LIFECYCLE_VERSION, 'moriarty-financial-lifecycle/1');
    assert.equal(LIFECYCLE_STATE_VERSION, 'moriarty-financial-lifecycle-state/1');
    assert.equal(LIFECYCLE_BOUNDS.sourceUtf8Bytes, 65536);
    assert.equal(LIFECYCLE_BOUNDS.collectionCapacity, 128);
    assert.equal(LIFECYCLE_BOUNDS.signed128Max, M);
    assert.equal(typeof prepareFinancialLifecycle, 'function');
    assert.equal(prepareFinancialLifecycle.length, 1);
    assert.equal(typeof admitFinancialLifecycleStateJSON, 'function');
  });
});

describe('independent kernel oracle', () => {
  test('main trace states, effects, work and unrelated Token rows', () => {
    const remaining = [];
    for (const stage of oracle.kernelTrace) {
      const compact = JSON.stringify({
        schemaVersion: LIFECYCLE_VERSION,
        state: stage.pre,
        actions: stage.actions,
      });
      const result = prepareFinancialLifecycle(compact);
      assertPrepared(result);
      assert.deepEqual(result.post, stage.post);
      assert.deepEqual(result.effects, stage.effects);
      remaining.push(result.post.work);
    }
    assert.deepEqual(remaining.map((w) => [w.remaining, w.spent, w.closureReserve]), [
      ['254', '19', '16'],
      ['253', '20', '16'],
      ['251', '22', '16'],
      ['249', '24', '16'],
    ]);
    const settled = remaining[3];
    const last = oracle.kernelTrace[3].post;
    assert.equal(last.obligations[0].status, 'Settled');
    assert.equal(last.obligations[0].liabilityIncurred, '110');
    assert.equal(last.balances.find((b) => b.asset === 'Token').amount, '7');
    assert.equal(settled.closureReserve, '16');
  });

  test('negative kernel cases reject without fragments', () => {
    for (const item of oracle.negativeKernelCases) {
      const result = prepareFinancialLifecycle(JSON.stringify({
        schemaVersion: LIFECYCLE_VERSION,
        state: item.pre,
        actions: item.actions,
      }));
      assertRejected(result, item.expected.actionIndex);
      assert.equal(result.code, item.expected.code, item.name);
    }
  });
});

describe('result-bound fixtures', () => {
  test('compact oversized input rejects RESULT_BOUND; adjacent exact bound succeeds', () => {
    const oversized = fixtures.cases.find((c) => c.name === 'oversized-result');
    const exact = fixtures.cases.find((c) => c.name === 'exact-state-result-bound');
    const overCompact = JSON.stringify(oversized.input);
    const exactCompact = JSON.stringify(exact.input);
    assert.equal(Buffer.byteLength(overCompact, 'utf8'), oversized.sizes.input);
    assert.equal(Buffer.byteLength(exactCompact, 'utf8'), exact.sizes.input);
    const rejected = prepareFinancialLifecycle(overCompact);
    assertRejected(rejected, null);
    assert.equal(rejected.code, 'RESULT_BOUND');
    const prepared = prepareFinancialLifecycle(exactCompact);
    assertPrepared(prepared);
    assert.equal(Buffer.byteLength(JSON.stringify(prepared.post), 'utf8'), exact.sizes.mathematicalPost);
  });
});

describe('admission', () => {
  test('pretty-printed input and extra keys reject before execution', () => {
    const compact = input(seed(), [transfer(), originate()]);
    assertRejected(prepareFinancialLifecycle(JSON.stringify(JSON.parse(compact), null, 2)), null);
    assertRejected(run(seed(), [transfer(), originate()], { extraRoot: { callback: true } }), null);
    assertRejected(run(seed({ extra: 1 }), [transfer()]), null);
    assertRejected(run(seed(), []), null);
    assertRejected(run(seed(), [transfer()], { schemaVersion: REPAYMENT_VERSION }), null);
  });

  test('closed-record and state invariant faults', () => {
    const funded = seed({
      obligations: [loan()],
      usedTransferIds: ['D1'],
      usedOriginationIds: ['O1'],
    });
    const badSum = admitFinancialLifecycleStateJSON(JSON.stringify({
      ...funded,
      obligations: [loan({ outstanding: '99' })],
    }));
    assert.equal(badSum.ok, false);
    assertRejected(badSum.result, null);
    assert.equal(badSum.result.code, 'INVARIANT');

    const badCursor = admitFinancialLifecycleStateJSON(JSON.stringify({
      ...funded,
      obligations: [loan({ nextAccrualAt: '1061' })],
    }));
    assert.equal(badCursor.ok, false);
    assert.equal(badCursor.result.code, 'INVARIANT');

    const extraOrigin = admitFinancialLifecycleStateJSON(JSON.stringify({
      ...funded,
      usedOriginationIds: ['O1', 'O2'],
    }));
    assert.equal(extraOrigin.ok, false);
    assert.equal(extraOrigin.result.code, 'INVARIANT');

    const unknown = run(seed(), [{ ...originate(), extra: 'x' }]);
    assertRejected(unknown, null);
    assert.equal(unknown.code, 'UNKNOWN_FIELD');
  });

  test('owned trees do not share mutable arrays', () => {
    const text = JSON.stringify(seed({
      obligations: [loan()],
      usedTransferIds: ['D1'],
      usedOriginationIds: ['O1'],
    }));
    const first = admitFinancialLifecycleStateJSON(text);
    assert.equal(first.ok, true);
    first.value.balances.push({ party: 'X', asset: 'Cash', amount: '1' });
    first.value.usedOriginationIds.push('O9');
    const second = admitFinancialLifecycleStateJSON(text);
    assert.equal(second.ok, true);
    assert.equal(second.value.balances.length, 3);
    assert.deepEqual(second.value.usedOriginationIds, ['O1']);
    assert.notEqual(first.value.balances, second.value.balances);
  });
});

describe('originate', () => {
  test('consumes the entire same-step transfer once', () => {
    const result = run(seed(), [transfer(), originate()]);
    assertPrepared(result);
    assert.equal(result.post.obligations[0].principal, '100');
    assert.equal(result.post.obligations[0].initialPrincipal, '100');
    assert.equal(result.post.obligations[0].liabilityIncurred, '100');
    assert.equal(result.post.work.remaining, '254');
    const again = run(result.post, [originate({ originationId: 'O2', obligationId: 'Loan2' })]);
    assertRejected(again, 0);
    assert.equal(again.code, 'TRANSFER_NOT_IN_STEP');
  });

  test('partial reverse-role repayment witness cannot originate', () => {
    const after = run(seed(), [transfer(), originate()]);
    assertPrepared(after);
    const partial = run(after.post, [
      payTransfer(),
      repay({ nominalAmount: '10' }),
      originate({
        obligationId: 'Loan2',
        originationId: 'O2',
        transferId: 'P1',
        debtor: 'Lender',
        creditor: 'Borrower',
        nominalAmount: '20',
        nominalLiabilityCap: '20',
      }),
    ]);
    assertRejected(partial, 2);
    assert.equal(partial.code, 'TRANSFER_ALREADY_ALLOCATED');
  });

  test('wrong direction, amount, zero and duplicate', () => {
    assert.equal(run(seed(), [transfer(), originate({ debtor: 'Lender', creditor: 'Borrower' })]).code, 'TRANSFER_MISMATCH');
    assert.equal(run(seed(), [transfer({ amount: '99' }), originate()]).code, 'TRANSFER_AMOUNT_MISMATCH');
    const rich = seed({
      balances: [
        { party: 'Lender', asset: 'Cash', amount: '200' },
        { party: 'Borrower', asset: 'Cash', amount: '10' },
        { party: 'Other', asset: 'Token', amount: '7' },
      ],
      allowances: [
        { party: 'Lender', asset: 'Cash', remaining: '200', spent: '0' },
        { party: 'Borrower', asset: 'Cash', remaining: '110', spent: '0' },
        { party: 'Other', asset: 'Token', remaining: '3', spent: '1' },
      ],
    });
    assert.equal(run(rich, [transfer({ amount: '101' }), originate()]).code, 'TRANSFER_AMOUNT_MISMATCH');
    assert.equal(run(seed(), [transfer(), originate({ nominalAmount: '0' })]).code, 'ZERO_AMOUNT');
    assert.equal(run(seed(), [transfer(), originate({ nominalLiabilityCap: '0' })]).code, 'ZERO_AMOUNT');
    const once = run(seed(), [transfer(), originate()]);
    assertPrepared(once);
    const funded = {
      ...once.post,
      balances: once.post.balances.map((row) => (
        row.party === 'Lender' && row.asset === 'Cash' ? { ...row, amount: '100' } : row
      )),
      allowances: once.post.allowances.map((row) => (
        row.party === 'Lender' && row.asset === 'Cash' ? { ...row, remaining: '100', spent: '100' } : row
      )),
    };
    const dup = run(funded, [
      transfer({ id: 'D2' }),
      originate({ originationId: 'O1', obligationId: 'Loan2', transferId: 'D2' }),
    ]);
    assertRejected(dup, 1);
    assert.equal(dup.code, 'DUPLICATE');
  });

  test('malformed later action beats earlier runtime failure', () => {
    const result = run(seed(), [
      transfer({ amount: '0' }),
      { kind: 'Originate' },
    ]);
    assertRejected(result, null);
    assert.equal(result.code, 'SCHEMA');
  });

  test('missing funding plus overflowing conversion is TRANSFER_NOT_IN_STEP', () => {
    const result = run(seed(), [originate({
      conversion: conversion({ mantissa: U128, scale: '0' }),
    })]);
    assertRejected(result, 0);
    assert.equal(result.code, 'TRANSFER_NOT_IN_STEP');
  });

  test('UInt64 start overflow on Originate', () => {
    const result = run(seed(), [transfer(), originate({
      accrualTerms: terms({ firstPeriodStart: U64, periodSeconds: '1' }),
    })]);
    assertRejected(result, 1);
    assert.equal(result.code, 'OVERFLOW');
  });
});

describe('accrue', () => {
  function originated() {
    const result = run(seed(), [transfer(), originate()]);
    assertPrepared(result);
    return result.post;
  }

  test('floor, ceil, zero interest and numeric overflow', () => {
    const floor = run(originated(), [accrue()]);
    assertPrepared(floor);
    assert.equal(floor.post.obligations[0].accrued, '10');
    assert.equal(floor.effects[0].interestAmount, '10');

    const ceilState = run(seed(), [transfer(), originate({
      accrualTerms: terms({ rounding: 'ceil' }),
      nominalLiabilityCap: '112',
    })]);
    const ceilP = { ...ceilState.post, obligations: [{
      ...ceilState.post.obligations[0],
      principal: '101',
      outstanding: '101',
      initialPrincipal: '101',
      liabilityIncurred: '101',
    }] };
    const ceil = run(ceilP, [accrue()]);
    assertPrepared(ceil);
    assert.equal(ceil.post.obligations[0].accrued, '11');

    const zero = run(seed(), [transfer(), originate({
      accrualTerms: terms({ numerator: '0' }),
    })]);
    const z = run(zero.post, [accrue()]);
    assertPrepared(z);
    assert.equal(z.post.obligations[0].accrued, '0');
    assert.deepEqual(z.post.usedAccrualIds, ['A1']);
    assert.equal(z.post.obligations[0].lastAccruedPeriod, '1');
    const repeat = run(z.post, [accrue({ accrualId: 'A2', periodIndex: '1', observedTime: '1120' })]);
    assert.equal(repeat.code, 'PERIOD_SEQUENCE');

    const born = originated();
    const hot = {
      ...born,
      obligations: [{
        ...born.obligations[0],
        principal: '2',
        outstanding: '2',
        accrued: '0',
        initialPrincipal: '2',
        liabilityIncurred: '2',
        nominalLiabilityCap: M,
        accrualTerms: terms({
          numerator: U128,
          denominator: U128,
        }),
        lastAccruedPeriod: '0',
        nextAccrualAt: '1060',
      }],
      usedAccrualIds: [],
    };
    const product = run(hot, [accrue({ accrualId: 'A9' })]);
    assertRejected(product, 0);
    assert.equal(product.code, 'OVERFLOW');
  });

  test('combined error order', () => {
    const start = originated();
    const overflowBeforeCap = run({
      ...start,
      obligations: [{
        ...start.obligations[0],
        accrualTerms: terms({ firstPeriodStart: (BigInt(U64) - 1n).toString(), periodSeconds: '1' }),
        lastAccruedPeriod: '0',
        nextAccrualAt: U64,
        nominalLiabilityCap: '100',
        liabilityIncurred: '100',
      }],
    }, [accrue({ observedTime: U64 })]);
    assertRejected(overflowBeforeCap, 0);
    assert.equal(overflowBeforeCap.code, 'OVERFLOW');

    const dup = run({
      ...start,
      usedAccrualIds: ['A1'],
      obligations: [{ ...start.obligations[0], lastAccruedPeriod: '1', nextAccrualAt: '1120' }],
    }, [accrue({ periodIndex: '3', observedTime: '1' })]);
    assertRejected(dup, 0);
    assert.equal(dup.code, 'DUPLICATE');

    const seq = run(start, [accrue({ accrualId: 'A2', periodIndex: '2', observedTime: '1' })]);
    assertRejected(seq, 0);
    assert.equal(seq.code, 'PERIOD_SEQUENCE');
  });

  test('cap is lifetime incurred; repayment does not restore it', () => {
    const afterPay = oracle.kernelTrace.find((s) => s.name === 'repay30').post;
    const refused = run(afterPay, [accrue({ accrualId: 'A2', periodIndex: '2', observedTime: '1120' })]);
    assertRejected(refused, 0);
    assert.equal(refused.code, 'LIABILITY_CAP_EXCEEDED');
  });

  test('settled cannot accrue; zero rate still fails', () => {
    const settled = oracle.kernelTrace.find((s) => s.name === 'repay80').post;
    const result = run(settled, [accrue({ accrualId: 'A2', periodIndex: '2', observedTime: '1120' })]);
    assertRejected(result, 0);
    assert.equal(result.code, 'NOT_OUTSTANDING');
    assert.equal(settled.obligations[0].lastAccruedPeriod, '1');
    assert.equal(settled.usedAccrualIds[0], 'A1');
  });
});

describe('legacy repayment kernel stays closed', () => {
  test('old prepareRepayment still rejects Originate', () => {
    const source = JSON.stringify({
      schemaVersion: REPAYMENT_VERSION,
      state: {
        balances: [{ party: 'Lender', asset: 'Cash', amount: '100' }],
        allowances: [{ party: 'Lender', asset: 'Cash', remaining: '100', spent: '0' }],
        obligations: [],
        usedTransferIds: [],
        usedAllocationIds: [],
        work: { remaining: '10', spent: '0', closureReserve: '0' },
      },
      actions: [transfer()],
    });
    const result = prepareRepayment(source);
    assert.equal(result.status, 'Prepared');
    const withOriginate = JSON.parse(source);
    withOriginate.actions.push(originate());
    const rejected = prepareRepayment(JSON.stringify(withOriginate));
    assert.equal(rejected.status, 'Rejected');
    assert.equal(rejected.code, 'UNKNOWN_ACTION');
  });
});
