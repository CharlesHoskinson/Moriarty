import assert from 'node:assert/strict';
import test from 'node:test';

import { evaluate } from '../dist/language/core.js';
import { defaultAction, elaborate, exampleSource } from '../dist/language/packages.js';

const mustElaborate = (kind) => {
  const result = elaborate(JSON.parse(exampleSource(kind)));
  assert.equal(result.outcome, 'elaborated', result.message);
  return result;
};

test('editing one elaborated program cannot corrupt later packages or reset examples', () => {
  const edited = mustElaborate('loan');
  edited.program.effectSchemas.Transfer.amount = 'String';
  delete edited.program.effectSchemas.DueCreated;
  for (const kind of ['loan', 'swap']) {
    const fresh = mustElaborate(kind);
    assert.equal(fresh.program.effectSchemas.Transfer.amount, 'UInt128');
    assert.ok(fresh.program.effectSchemas.DueCreated);
    assert.equal(evaluate(fresh.program, fresh.initialState, defaultAction(fresh, fresh.initialState)).outcome, 'evaluated');
  }
});

test('default constant-product swap emits 19743 and commits its balances', () => {
  const bundle = mustElaborate('swap');
  const result = evaluate(bundle.program, bundle.initialState, defaultAction(bundle, bundle.initialState));
  assert.equal(result.outcome, 'evaluated');
  assert.deepEqual(result.after.values, {
    reserveA: '1010000', reserveB: '1980257', traderA: '90000', traderB: '19743',
    providerA: '0', providerB: '0', closed: '0',
  });
  assert.deepEqual(result.effects, [
    { kind: 'Transfer', fields: { asset: 'demo:A', from: 'trader', to: 'pool:demo', amount: '10000' } },
    { kind: 'Transfer', fields: { asset: 'demo:B', from: 'pool:demo', to: 'trader', amount: '19743' } },
  ]);
  assert.match(bundle.description, /fee is retained inside the full input/i);
  assert.equal(result.after.revision, '1');
  assert.equal(result.after.remaining, '7');
});

test('minimum output above the quote rejects atomically', () => {
  const bundle = mustElaborate('swap');
  const action = { ...defaultAction(bundle, bundle.initialState), args: { ...defaultAction(bundle, bundle.initialState).args, minOut: '19744' } };
  const before = structuredClone(bundle.initialState);
  const result = evaluate(bundle.program, bundle.initialState, action);
  assert.equal(result.outcome, 'rejected');
  assert.equal(result.code, 'GUARD_FAILED');
  assert.deepEqual(bundle.initialState, before);
});

test('loan accrues on old notional and creates separate dues', () => {
  const bundle = mustElaborate('loan');
  const result = evaluate(bundle.program, bundle.initialState, defaultAction(bundle, bundle.initialState));
  assert.equal(result.outcome, 'evaluated');
  assert.equal(result.after.values.notional, '4500000000');
  assert.equal(result.after.values.principalDue, '500000000');
  assert.equal(result.after.values.interestDue, '33972602');
  assert.equal(result.after.values.cursor, '1');
  assert.deepEqual(result.effects.map((effect) => effect.fields.amount), ['500000000', '33972602']);
  assert.match(bundle.description, /33\.972602739726/);
  assert.match(bundle.description, /33\.972602 USD/);
});

test('loan full settlement consumes dues and transfers 533972602', () => {
  const bundle = mustElaborate('loan');
  const accrued = evaluate(bundle.program, bundle.initialState, defaultAction(bundle, bundle.initialState));
  assert.equal(accrued.outcome, 'evaluated');
  const settled = evaluate(bundle.program, accrued.after, defaultAction(bundle, accrued.after));
  assert.equal(settled.outcome, 'evaluated');
  assert.equal(settled.after.values.borrowerCash, '19466027398');
  assert.equal(settled.after.values.lenderCash, '533972602');
  assert.equal(settled.after.values.principalDue, '0');
  assert.equal(settled.after.values.interestDue, '0');
  assert.equal(settled.after.values.principalPaid, '500000000');
  assert.equal(settled.after.values.interestPaid, '33972602');
  assert.equal(settled.after.values.closed, '1');
  assert.equal(settled.effects[0].fields.amount, '533972602');
});

test('wrong order, actor, asset, and insufficient balance reject', () => {
  const loan = mustElaborate('loan');
  const settleFirst = { name: 'settle', args: { actor: 'borrower', asset: 'demo:USD6', amount: '0' } };
  assert.equal(evaluate(loan.program, loan.initialState, settleFirst).outcome, 'rejected');
  const badActor = { ...defaultAction(loan, loan.initialState), args: { ...defaultAction(loan, loan.initialState).args, actor: 'mallory' } };
  assert.equal(evaluate(loan.program, loan.initialState, badActor).outcome, 'rejected');
  const accrued = evaluate(loan.program, loan.initialState, defaultAction(loan, loan.initialState));
  assert.equal(accrued.outcome, 'evaluated');
  const settle = defaultAction(loan, accrued.after);
  assert.equal(evaluate(loan.program, accrued.after, { ...settle, args: { ...settle.args, asset: 'demo:EUR6' } }).outcome, 'rejected');
  const poor = { ...accrued.after, values: { ...accrued.after.values, borrowerCash: '1' } };
  assert.equal(evaluate(loan.program, poor, settle).outcome, 'rejected');
});

const genericProgram = {
  version: 'moriarty-core/1',
  stateSchema: { count: 'UInt128', owner: 'String' },
  effectSchemas: { Transfer: { asset: 'String', from: 'String', to: 'String', amount: 'UInt128' } },
  entrypoints: {
    increment: {
      argSchema: { actor: 'String', amount: 'UInt128' },
      instructions: [
        { op: 'guard', condition: { op: 'eq', left: { op: 'arg', name: 'actor' }, right: { op: 'state', field: 'owner' } }, message: 'owner required' },
        { op: 'let', name: 'next', value: { op: 'add', left: { op: 'state', field: 'count' }, right: { op: 'arg', name: 'amount' } } },
        { op: 'set', field: 'count', value: { op: 'local', name: 'next' } },
      ],
    },
  },
};

test('generic Core evaluates a nonfinancial program', () => {
  const state = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '4', owner: 'alice' } };
  const result = evaluate(genericProgram, state, { name: 'increment', args: { actor: 'alice', amount: '3' } });
  assert.equal(result.outcome, 'evaluated');
  assert.equal(result.after.values.count, '7');
});

test('zero division and intermediate overflow reject with rollback', () => {
  const state = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '4', owner: 'alice' } };
  for (const value of [
    { op: 'div', left: { op: 'state', field: 'count' }, right: { op: 'literal', value: '0' } },
    { op: 'add', left: { op: 'literal', value: '340282366920938463463374607431768211455' }, right: { op: 'literal', value: '1' } },
  ]) {
    const program = structuredClone(genericProgram);
    program.entrypoints.increment.instructions[1].value = value;
    const before = structuredClone(state);
    assert.equal(evaluate(program, state, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
    assert.deepEqual(state, before);
  }
});

test('unknown syntax, exhausted budget, malformed source, and cycles reject', () => {
  const state = { instance: 'counter:1', revision: '0', remaining: '0', values: { count: '4', owner: 'alice' } };
  assert.equal(evaluate(genericProgram, state, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
  const unknown = structuredClone(genericProgram);
  unknown.entrypoints.increment.instructions[0] = { op: 'launch' };
  assert.equal(evaluate(unknown, { ...state, remaining: '2' }, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
  assert.equal(elaborate({ language: 'moriarty-r2/1', package: 'unknown', terms: {} }).outcome, 'rejected');
  const cyclic = structuredClone(genericProgram);
  cyclic.entrypoints.increment.instructions[0].condition = cyclic;
  assert.equal(evaluate(cyclic, { ...state, remaining: '2' }, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
});

test('pool reserves its final allowance for closure and closes to provider', () => {
  const bundle = mustElaborate('swap');
  const almostDone = { ...bundle.initialState, remaining: '1' };
  assert.equal(evaluate(bundle.program, almostDone, defaultAction(bundle, almostDone)).outcome, 'rejected');
  const close = { name: 'close', args: { actor: 'provider' } };
  const result = evaluate(bundle.program, almostDone, close);
  assert.equal(result.outcome, 'evaluated');
  assert.equal(result.after.values.reserveA, '0');
  assert.equal(result.after.values.reserveB, '0');
  assert.equal(result.after.values.providerA, '1000000');
  assert.equal(result.after.values.providerB, '2000000');
  assert.equal(result.after.values.closed, '1');
});

test('validation rejects hidden unknown reads and cumulative expression excess', () => {
  const state = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '4', owner: 'alice' } };
  const hidden = structuredClone(genericProgram);
  hidden.entrypoints.increment.instructions[0].condition = {
    op: 'or', left: { op: 'literal', value: true },
    right: { op: 'eq', left: { op: 'state', field: 'missing' }, right: { op: 'literal', value: '0' } },
  };
  assert.equal(evaluate(hidden, state, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');

  const excessive = structuredClone(genericProgram);
  excessive.entrypoints.increment.instructions = Array.from({ length: 64 }, (_, index) => ({
    op: 'let', name: `v${index}`,
    value: { op: 'add', left: { op: 'add', left: { op: 'literal', value: '1' }, right: { op: 'literal', value: '1' } }, right: { op: 'literal', value: '1' } },
  }));
  assert.equal(evaluate(excessive, state, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
});

test('Core statically rejects operator, guard, set, and emitted-field type mismatches', () => {
  const state = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '4', owner: '123owner' } };
  const action = { name: 'increment', args: { actor: '123owner', amount: '3' } };
  const mismatches = [];
  const numericString = structuredClone(genericProgram);
  numericString.entrypoints.increment.instructions[1].value = { op: 'add', left: { op: 'state', field: 'owner' }, right: { op: 'literal', value: '1' } };
  mismatches.push(numericString);
  const nonBooleanGuard = structuredClone(genericProgram);
  nonBooleanGuard.entrypoints.increment.instructions[0].condition = { op: 'state', field: 'count' };
  mismatches.push(nonBooleanGuard);
  const wrongSet = structuredClone(genericProgram);
  wrongSet.entrypoints.increment.instructions[2].value = { op: 'literal', value: true };
  mismatches.push(wrongSet);
  const wrongEffect = structuredClone(genericProgram);
  wrongEffect.entrypoints.increment.instructions.push({ op: 'emit', kind: 'Transfer', fields: {
    asset: { op: 'literal', value: 'asset' }, from: { op: 'literal', value: 'from' },
    to: { op: 'literal', value: 'to' }, amount: { op: 'literal', value: true },
  } });
  mismatches.push(wrongEffect);
  const hiddenMismatch = structuredClone(genericProgram);
  hiddenMismatch.entrypoints.increment.instructions[0].condition = {
    op: 'or', left: { op: 'literal', value: true },
    right: { op: 'add', left: { op: 'state', field: 'owner' }, right: { op: 'literal', value: '1' } },
  };
  mismatches.push(hiddenMismatch);
  for (const program of mismatches) assert.equal(evaluate(program, state, action).outcome, 'rejected');
});

test('Core and package intake reject active objects without invoking host callbacks', () => {
  let calls = 0;
  const program = structuredClone(genericProgram);
  Object.defineProperty(program, 'surprise', { enumerable: true, get() { calls += 1; return 'boom'; } });
  const state = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '4', owner: 'alice' } };
  assert.equal(evaluate(program, state, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');
  assert.equal(calls, 0);

  const source = JSON.parse(exampleSource('loan'));
  Object.defineProperty(source, 'toJSON', { enumerable: true, value() { calls += 1; return {}; } });
  assert.equal(elaborate(source).outcome, 'rejected');
  assert.equal(calls, 0);
});

test('bounded intake and package profile constraints reject malformed domains', () => {
  const counterState = { instance: 'counter:1', revision: '0', remaining: '2', values: { count: '9'.repeat(1000), owner: 'alice' } };
  assert.equal(evaluate(genericProgram, counterState, { name: 'increment', args: { actor: 'alice', amount: '3' } }).outcome, 'rejected');

  const invalidSources = [];
  const aliasedPool = JSON.parse(exampleSource('swap')); aliasedPool.terms.provider = aliasedPool.terms.trader; invalidSources.push(aliasedPool);
  const sameAssets = JSON.parse(exampleSource('swap')); sameAssets.terms.assetB = sameAssets.terms.assetA; invalidSources.push(sameAssets);
  const badFee = JSON.parse(exampleSource('swap')); badFee.terms.feeNumerator = '1001'; invalidSources.push(badFee);
  const zeroReserve = JSON.parse(exampleSource('swap')); zeroReserve.terms.reserveA = '0'; invalidSources.push(zeroReserve);
  const aliasedLoan = JSON.parse(exampleSource('loan')); aliasedLoan.terms.lender = aliasedLoan.terms.borrower; invalidSources.push(aliasedLoan);
  const wrongCurrency = JSON.parse(exampleSource('loan')); wrongCurrency.terms.denomination = 'EUR'; invalidSources.push(wrongCurrency);
  const badId = JSON.parse(exampleSource('loan')); badId.terms.borrower = 'bad\nactor'; invalidSources.push(badId);
  for (const source of invalidSources) assert.equal(elaborate(source).outcome, 'rejected');
});

test('pool rejects a positive input whose quantized output is zero', () => {
  const source = JSON.parse(exampleSource('swap'));
  source.terms.reserveA = '1000000000000'; source.terms.reserveB = '1'; source.terms.traderA = '1';
  const bundle = elaborate(source); assert.equal(bundle.outcome, 'elaborated');
  const action = { name: 'swap', args: { actor: 'trader', amountIn: '1', minOut: '0', recipient: 'trader', assetIn: 'demo:A', assetOut: 'demo:B' } };
  assert.equal(evaluate(bundle.program, bundle.initialState, action).outcome, 'rejected');
});

test('edited loan terms receive a dynamic quantization description', () => {
  const source = JSON.parse(exampleSource('loan'));
  source.terms.notional = '1000000'; source.terms.principalPayment = '100000'; source.terms.rateNumerator = '1'; source.terms.dayCountNumerator = '1';
  const bundle = elaborate(source); assert.equal(bundle.outcome, 'elaborated');
  assert.doesNotMatch(bundle.description, /33\.972602/);
  assert.match(bundle.description, /27 micro-USD/);
});
