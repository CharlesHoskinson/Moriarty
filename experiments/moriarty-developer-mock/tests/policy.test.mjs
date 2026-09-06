import test from 'node:test';
import assert from 'node:assert/strict';
import { checkIntentEffects } from '../src/language/policy.ts';

const checked = { outcome: 'checked', predicate: 'intent-effects', assurance: 'local-check-only' };
const reject = (result, code) => {
  assert.equal(result.outcome, 'rejected');
  assert.equal(result.code, code);
  assert.notEqual(result.outcome, 'PCDverified');
};

const swapAccounting = {
  reserveA: { asset: 'A', account: 'pool' }, traderA: { asset: 'A', account: 'trader' }, providerA: { asset: 'A', account: 'provider' },
  reserveB: { asset: 'B', account: 'pool' }, traderB: { asset: 'B', account: 'trader' }, providerB: { asset: 'B', account: 'provider' },
};
const swapPolicy = {
  profile: 'swap', accounting: swapAccounting,
  transfers: [{ asset: 'A', from: 'trader', to: 'pool', maxAmount: '20' }, { asset: 'B', from: 'pool', to: 'trader', maxAmount: '30' }],
  minimumCredits: [{ asset: 'B', to: 'trader', minAmount: '18' }], fees: [], dues: [],
  allowedWrites: ['reserveA', 'reserveB', 'traderA', 'traderB'],
};
const state = (values, revision = '0', remaining = '4') => ({ instance: 'swap-1', revision, remaining, values });
const beforeSwap = state({ reserveA: '100', reserveB: '200', traderA: '50', traderB: '0', providerA: '0', providerB: '0', closed: '0' });
const afterSwap = state({ reserveA: '110', reserveB: '181', traderA: '40', traderB: '19', providerA: '0', providerB: '0', closed: '0' }, '1', '3');
const swapEffects = [
  { kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '10' } },
  { kind: 'Transfer', fields: { asset: 'B', from: 'pool', to: 'trader', amount: '19' } },
];

test('accepts a fully projected swap under restrictive intent', () => {
  assert.deepEqual(checkIntentEffects(beforeSwap, afterSwap, swapEffects, swapPolicy), checked);
});

test('rejects wrong recipient, wrong asset, cumulative allowance and cumulative fee evasions', () => {
  const wrongRecipient = [{ kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'provider', amount: '10' } }];
  reject(checkIntentEffects(beforeSwap, afterSwap, wrongRecipient, swapPolicy), 'EffectNotAllowed');
  const wrongAsset = [{ kind: 'Transfer', fields: { asset: 'B', from: 'trader', to: 'pool', amount: '10' } }];
  reject(checkIntentEffects(beforeSwap, afterSwap, wrongAsset, swapPolicy), 'EffectNotAllowed');

  const duplicatedTransfers = [
    { kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '11' } },
    { kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '9' } },
  ];
  reject(checkIntentEffects(beforeSwap, afterSwap, duplicatedTransfers, { ...swapPolicy, transfers: [{ asset: 'A', from: 'trader', to: 'pool', maxAmount: '19' }], minimumCredits: [] }), 'EffectLimitExceeded');

  const feeBefore = beforeSwap;
  const feeAfter = state({ ...beforeSwap.values, traderA: '38', reserveA: '112' }, '1', '3');
  const fees = [
    { kind: 'Fee', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '6' } },
    { kind: 'Fee', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '6' } },
  ];
  reject(checkIntentEffects(feeBefore, feeAfter, fees, { ...swapPolicy, transfers: [], minimumCredits: [], fees: [{ asset: 'A', from: 'trader', to: 'pool', maxAmount: '10' }], allowedWrites: ['traderA', 'reserveA'] }), 'FeeLimitExceeded');
});

test('rejects duplicate policy allowances instead of multiplying caps', () => {
  const allowance = { asset: 'A', from: 'trader', to: 'pool', maxAmount: '10' };
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, transfers: [allowance, allowance] }), 'MalformedPolicy');
});

test('rejects transfer allowances whose accounts cannot be projected', () => {
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, {
    ...swapPolicy,
    transfers: [{ asset: 'A', from: 'ghost', to: 'pool', maxAmount: '20' }],
    minimumCredits: [],
  }), 'UnsupportedMapping');
});

test('rejects approval, unlisted and unknown writes, missing effects, and envelope tampering', () => {
  reject(checkIntentEffects(beforeSwap, afterSwap, [...swapEffects, { kind: 'Approval', fields: {} }], swapPolicy), 'MalformedEffect');
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, allowedWrites: ['reserveA', 'reserveB', 'traderA'] }), 'WriteNotAllowed');
  reject(checkIntentEffects(beforeSwap, state({ ...afterSwap.values, secretBalance: '1' }, '1', '3'), swapEffects, swapPolicy), 'MalformedState');
  reject(checkIntentEffects(beforeSwap, afterSwap, [swapEffects[0]], { ...swapPolicy, minimumCredits: [] }), 'AccountingMismatch');
  reject(checkIntentEffects(beforeSwap, { ...afterSwap, revision: '2' }, swapEffects, swapPolicy), 'InvalidTransition');
  reject(checkIntentEffects(beforeSwap, { ...afterSwap, instance: 'other' }, swapEffects, swapPolicy), 'InvalidTransition');
});

const loanAccounting = { borrowerCash: { asset: 'USD6', account: 'borrower' }, lenderCash: { asset: 'USD6', account: 'lender' } };
const loanBase = { notional: '1000', principalDue: '0', interestDue: '0', principalPaid: '0', interestPaid: '0', borrowerCash: '600', lenderCash: '0', cursor: '0', closed: '0' };
const loanPolicy = {
  profile: 'loan', accounting: loanAccounting, transfers: [], minimumCredits: [], fees: [], allowedWrites: [],
  dues: [
    { kind: 'DueCreated', bucket: 'principal', dueId: 'loan-1:principal', debtor: 'borrower', creditor: 'lender', denomination: 'USD', maxAmount: '100' },
    { kind: 'DueCreated', bucket: 'interest', dueId: 'loan-1:interest', debtor: 'borrower', creditor: 'lender', denomination: 'USD', maxAmount: '8' },
    { kind: 'DueSettled', bucket: 'principal', dueId: 'loan-1:principal', debtor: 'borrower', creditor: 'lender', denomination: 'USD', asset: 'USD6', maxAmount: '100' },
    { kind: 'DueSettled', bucket: 'interest', dueId: 'loan-1:interest', debtor: 'borrower', creditor: 'lender', denomination: 'USD', asset: 'USD6', maxAmount: '8' },
  ],
};
const loanState = (values, revision = '0', remaining = '4') => ({ instance: 'loan-1', revision, remaining, values });

test('accepts separate due creation and verifies principal reduction', () => {
  const after = loanState({ ...loanBase, notional: '900', principalDue: '100', interestDue: '8', cursor: '1' }, '1', '3');
  const effects = [
    { kind: 'DueCreated', fields: { dueId: 'loan-1:principal', debtor: 'borrower', creditor: 'lender', denomination: 'USD', amount: '100' } },
    { kind: 'DueCreated', fields: { dueId: 'loan-1:interest', debtor: 'borrower', creditor: 'lender', denomination: 'USD', amount: '8' } },
  ];
  const policy = { ...loanPolicy, allowedWrites: ['notional', 'principalDue', 'interestDue', 'cursor'] };
  assert.deepEqual(checkIntentEffects(loanState(loanBase), after, effects, policy), checked);
  reject(checkIntentEffects(loanState(loanBase), { ...after, values: { ...after.values, notional: '901' } }, effects, policy), 'LoanAccountingMismatch');
});

test('accepts backed settlement and rejects an unbacked paid accounting update', () => {
  const before = loanState({ ...loanBase, notional: '900', principalDue: '100', interestDue: '8' });
  const after = loanState({ ...before.values, principalDue: '0', interestDue: '0', principalPaid: '100', interestPaid: '8', borrowerCash: '492', lenderCash: '108' }, '1', '3');
  const effects = [
    { kind: 'Transfer', fields: { asset: 'USD6', from: 'borrower', to: 'lender', amount: '108' } },
    { kind: 'DueSettled', fields: { dueId: 'loan-1:principal', debtor: 'borrower', creditor: 'lender', denomination: 'USD', asset: 'USD6', amount: '100' } },
    { kind: 'DueSettled', fields: { dueId: 'loan-1:interest', debtor: 'borrower', creditor: 'lender', denomination: 'USD', asset: 'USD6', amount: '8' } },
  ];
  const policy = { ...loanPolicy, transfers: [{ asset: 'USD6', from: 'borrower', to: 'lender', maxAmount: '108' }], minimumCredits: [{ asset: 'USD6', to: 'lender', minAmount: '108' }], allowedWrites: ['principalDue', 'interestDue', 'principalPaid', 'interestPaid', 'borrowerCash', 'lenderCash'] };
  assert.deepEqual(checkIntentEffects(before, after, effects, policy), checked);
  reject(checkIntentEffects(before, after, effects.slice(0, 2), policy), 'LoanAccountingMismatch');
  reject(checkIntentEffects(before, after, effects.slice(1), { ...policy, transfers: [], minimumCredits: [] }), 'AccountingMismatch');
});

test('rejects malformed input, aliases, unknown fields and UInt128 overflow', () => {
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, surprise: true }), 'MalformedPolicy');
  reject(checkIntentEffects(beforeSwap, afterSwap, [{ kind: 'Transfer', fields: { ...swapEffects[0].fields, extra: 'x' } }, swapEffects[1]], swapPolicy), 'MalformedEffect');
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, accounting: { ...swapAccounting, providerA: { asset: 'A', account: 'trader' }, providerB: { asset: 'B', account: 'trader' } } }), 'MalformedPolicy');
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, transfers: [{ ...swapPolicy.transfers[0], asset: 'A\0shadow' }] }), 'MalformedPolicy');
  reject(checkIntentEffects({ ...beforeSwap, remaining: '01' }, afterSwap, swapEffects, swapPolicy), 'MalformedState');
  reject(checkIntentEffects(beforeSwap, afterSwap, [{ kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'pool', amount: (1n << 128n).toString() } }], swapPolicy), 'MalformedEffect');
  reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, { ...swapPolicy, transfers: new Array(17).fill(swapPolicy.transfers[0]) }), 'MalformedPolicy');
});

test('rejects active or oversized inputs without invoking callbacks or throwing', () => {
  let calls = 0;
  const activePolicy = { ...swapPolicy };
  Object.defineProperty(activePolicy, 'toJSON', { enumerable: true, value() { calls += 1; return swapPolicy; } });
  assert.doesNotThrow(() => reject(checkIntentEffects(beforeSwap, afterSwap, swapEffects, activePolicy), 'MalformedPolicy'));
  assert.equal(calls, 0);
  const activeState = { ...beforeSwap };
  Object.defineProperty(activeState, 'values', { enumerable: true, get() { calls += 1; return beforeSwap.values; } });
  assert.doesNotThrow(() => reject(checkIntentEffects(activeState, afterSwap, swapEffects, swapPolicy), 'MalformedState'));
  assert.equal(calls, 0);
  const huge = { ...beforeSwap, instance: 'x'.repeat(70_000) };
  assert.doesNotThrow(() => reject(checkIntentEffects(huge, afterSwap, swapEffects, swapPolicy), 'MalformedState'));
});

test('loan due allowances are bound to instance, roles, denomination, bucket, and settlement asset', () => {
  const mutations = [
    due => ({ ...due, debtor: 'lender', creditor: 'borrower' }),
    due => ({ ...due, denomination: 'USD6' }),
    due => ({ ...due, dueId: 'other:principal' }),
    due => ({ ...due, bucket: due.bucket === 'principal' ? 'interest' : 'principal' }),
    due => due.kind === 'DueSettled' ? ({ ...due, asset: 'other' }) : due,
  ];
  for (const mutate of mutations) {
    const dues = loanPolicy.dues.map((due, index) => index === (mutations.indexOf(mutate) === 4 ? 2 : 0) ? mutate(due) : due);
    reject(checkIntentEffects(loanState(loanBase), loanState(loanBase, '1', '3'), [], { ...loanPolicy, dues }), 'MalformedPolicy');
  }
});

test('minimum credit is net of outgoing transfers and fees', () => {
  const after = state({ ...beforeSwap.values, reserveB: '197', traderB: '2', providerB: '1' }, '1', '3');
  const effects = [
    { kind: 'Transfer', fields: { asset: 'B', from: 'pool', to: 'trader', amount: '3' } },
    { kind: 'Fee', fields: { asset: 'B', from: 'trader', to: 'provider', amount: '1' } },
  ];
  const policy = { ...swapPolicy,
    transfers: [{ asset: 'B', from: 'pool', to: 'trader', maxAmount: '3' }],
    fees: [{ asset: 'B', from: 'trader', to: 'provider', maxAmount: '1' }],
    minimumCredits: [{ asset: 'B', to: 'trader', minAmount: '3' }],
    allowedWrites: ['reserveB', 'traderB', 'providerB'],
  };
  reject(checkIntentEffects(beforeSwap, after, effects, policy), 'MinimumCreditNotMet');
});

test('gross edge authority caps cannot be hidden by refunds or intermediate recipients', () => {
  const after = state({ ...beforeSwap.values, reserveA: '110', traderA: '40' }, '1', '3');
  const roundTrip = [
    { kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'pool', amount: '21' } },
    { kind: 'Transfer', fields: { asset: 'A', from: 'pool', to: 'trader', amount: '11' } },
  ];
  const policy = { ...swapPolicy,
    transfers: [{ asset: 'A', from: 'trader', to: 'pool', maxAmount: '20' }, { asset: 'A', from: 'pool', to: 'trader', maxAmount: '11' }],
    minimumCredits: [], allowedWrites: ['reserveA', 'traderA'],
  };
  reject(checkIntentEffects(beforeSwap, after, roundTrip, policy), 'EffectLimitExceeded');
  const intermediate = [
    { kind: 'Transfer', fields: { asset: 'A', from: 'trader', to: 'provider', amount: '10' } },
    { kind: 'Transfer', fields: { asset: 'A', from: 'provider', to: 'pool', amount: '10' } },
  ];
  reject(checkIntentEffects(beforeSwap, after, intermediate, policy), 'EffectNotAllowed');
});

test('aggregate effect arithmetic is bounded to signed UInt128 magnitude', () => {
  const max = (1n << 128n) - 1n;
  const effects = [
    { kind: 'Transfer', fields: { asset: 'B', from: 'pool', to: 'trader', amount: max.toString() } },
    { kind: 'Transfer', fields: { asset: 'B', from: 'provider', to: 'trader', amount: max.toString() } },
  ];
  const policy = { ...swapPolicy,
    transfers: [
      { asset: 'B', from: 'pool', to: 'trader', maxAmount: max.toString() },
      { asset: 'B', from: 'provider', to: 'trader', maxAmount: max.toString() },
    ], minimumCredits: [], allowedWrites: [],
  };
  reject(checkIntentEffects(beforeSwap, state(beforeSwap.values, '1', '3'), effects, policy), 'AggregateOverflow');
});
