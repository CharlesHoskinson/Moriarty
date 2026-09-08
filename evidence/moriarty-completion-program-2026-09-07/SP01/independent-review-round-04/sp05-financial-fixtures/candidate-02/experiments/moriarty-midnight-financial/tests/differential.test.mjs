import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { compareFinancialEffects } from '../src/differential.mjs';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const loan = JSON.parse(readFileSync(join(root, 'fixtures/loan.json'), 'utf8'));
const swap = JSON.parse(readFileSync(join(root, 'fixtures/swap.json'), 'utf8'));

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function freezeDeep(value) {
  if (value && typeof value === 'object') {
    Object.freeze(value);
    for (const key of Object.keys(value)) freezeDeep(value[key]);
  }
  return value;
}

function stageOf(record, name) {
  return record.stages.find((item) => item.stage === name);
}

function bal(list, id) {
  return list.find((item) => item.id === id);
}

function effectOf(stage, id) {
  return stage.actorEffects.find((item) => item.id === id);
}

function dueOf(stage, id) {
  return stage.liabilities.dues.find((item) => item.id === id);
}

function codes(result) {
  return result.errors.map((item) => item.code);
}

function assertNetworkIncomplete(result) {
  assert.equal(result.networkAcceptance, false);
  assert.equal(result.networkEvidence, 'incompleteNetworkEvidence');
}

function assertFails(expected, observed, requiredCodes) {
  const first = compareFinancialEffects(expected, observed);
  const second = compareFinancialEffects(expected, observed);
  assert.equal(first.ok, false);
  assertNetworkIncomplete(first);
  assert.equal(JSON.stringify(first.errors), JSON.stringify(second.errors));
  const got = new Set(codes(first));
  for (const code of requiredCodes) {
    assert.ok(got.has(code), `missing ${code} in ${JSON.stringify(first.errors)}`);
  }
  return first;
}

test('independent loan floor interest and cash totals match fixture', () => {
  const notional = 5000000000n;
  const interest = (notional * 8n * 31n) / (100n * 365n);
  const remainder = (notional * 8n * 31n) % (100n * 365n);
  assert.equal(interest, 33972602n);
  assert.equal(remainder, 27000n);
  assert.equal(remainder * 73n, 54n * 36500n);
  const total = 500000000n + interest;
  assert.equal(total, 533972602n);
  assert.equal(20000000000n - total, 19466027398n);
  assert.equal(notional - 500000000n, 4500000000n);
  const accrue = stageOf(loan, 'accrue');
  const settle = stageOf(loan, 'settle');
  assert.equal(loan.observationKind, 'synthetic-local');
  assert.equal(dueOf(accrue, 'lam01:period1:IP').outstanding, '33972602');
  assert.equal(dueOf(accrue, 'lam01:period1:PR').outstanding, '500000000');
  assert.equal(settle.liabilities.principal.paid, '500000000');
  assert.equal(settle.liabilities.accrual.interestPaid, '33972602');
  assert.equal(settle.liabilities.nominalRemaining, '4500000000');
  assert.equal(bal(settle.balances.post, 'borrower|USD_TEST_ASSET|USD_micro').amount, '19466027398');
  assert.equal(bal(settle.balances.post, 'lender|USD_TEST_ASSET|USD_micro').amount, '533972602');
  assert.equal(settle.revision, '2');
  assert.equal(settle.remaining, '0');
  assert.equal(settle.work, '0');
  assert.equal(loan.sourcePins.metadataUsedForExpectations, false);
});

test('independent swap floor output fee and reserves match fixture', () => {
  const amountIn = 10000n;
  const reserveA = 1000000n;
  const reserveB = 2000000n;
  const fee = amountIn - (amountIn * 997n) / 1000n;
  assert.equal(fee, 30n);
  const effective = amountIn * 997n;
  const output = (effective * reserveB) / (reserveA * 1000n + effective);
  assert.equal(output, 19743n);
  const noFee = (amountIn * reserveB) / (reserveA + amountIn);
  assert.equal(noFee, 19801n);
  assert.ok(noFee > output);
  const trade = stageOf(swap, 'swap');
  const close = stageOf(swap, 'close');
  const setup = stageOf(swap, 'setup');
  assert.equal(swap.observationKind, 'synthetic-local');
  assert.equal(trade.economicFee.amount, '30');
  assert.equal(trade.transfers[1].amount, '19743');
  assert.equal(bal(trade.balances.post, 'pool|ASSET_A|AssetA_quantum').amount, '1010000');
  assert.equal(bal(trade.balances.post, 'pool|ASSET_B|AssetB_quantum').amount, '1980257');
  assert.equal(bal(trade.balances.post, 'trader|ASSET_A|AssetA_quantum').amount, '90000');
  assert.equal(bal(close.balances.post, 'provider|ASSET_A|AssetA_quantum').amount, '1010000');
  assert.equal(bal(close.balances.post, 'provider|ASSET_B|AssetB_quantum').amount, '1980257');
  assert.equal(setup.transfers.length, 3);
  assert.equal(swap.agreement.lifetime, '8');
  assert.equal(trade.revision, '1');
  assert.equal(trade.remaining, '7');
  assert.equal(close.revision, '2');
  assert.equal(close.remaining, '6');
  assert.equal(close.residualDuties[0].status, 'discharged');
  assert.equal(swap.sourcePins.metadataUsedForExpectations, false);
});

test('identical valid records compare ok and still report incomplete network evidence', () => {
  const loanResult = compareFinancialEffects(loan, clone(loan));
  const swapResult = compareFinancialEffects(swap, clone(swap));
  assert.equal(loanResult.ok, true);
  assert.equal(swapResult.ok, true);
  assert.deepEqual(loanResult.errors, []);
  assert.deepEqual(swapResult.errors, []);
  assertNetworkIncomplete(loanResult);
  assertNetworkIncomplete(swapResult);
  const claimed = clone(loan);
  claimed.networkAcceptance = true;
  assertFails(loan, claimed, ['NETWORK_CLAIM_FORBIDDEN']);
});

test('wrong payee fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').transfers[0].to = 'borrower';
  assertFails(loan, observed, ['TRANSFER_MISMATCH']);
});

test('wrong color fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').transfers[0].color = 'USDC';
  assertFails(loan, observed, ['COLOR_MISMATCH']);
});

test('wrong quantum fails', () => {
  const observed = clone(loan);
  observed.deploymentBinding.assets[0].quantum = '1000000';
  stageOf(observed, 'settle').bindings.assets[0].quantum = '1000000';
  assertFails(loan, observed, ['QUANTUM_MISMATCH']);
});

test('wrong denomination fails', () => {
  const observed = clone(loan);
  observed.deploymentBinding.assets[0].denomination = 'USD';
  stageOf(observed, 'accrue').liabilities.denomination = 'USD';
  dueOf(stageOf(observed, 'accrue'), 'lam01:period1:PR').denomination = 'USD';
  assertFails(loan, observed, ['DENOMINATION_MISMATCH']);
});

test('omitted transfer fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').transfers = [];
  assertFails(loan, observed, ['OMITTED_TRANSFER']);
});

test('extra output fails', () => {
  const observed = clone(swap);
  const trade = stageOf(observed, 'swap');
  trade.transfers.push({
    id: 'swap:trade:extra-dust',
    ordinal: '2',
    from: 'pool',
    to: 'trader',
    color: 'ASSET_B',
    unit: 'AssetB_quantum',
    amount: '1'
  });
  assertFails(swap, observed, ['EXTRA_TRANSFER']);
});

test('duplicate identity fails', () => {
  const observed = clone(loan);
  const settle = stageOf(observed, 'settle');
  settle.transfers.push({ ...settle.transfers[0] });
  assertFails(loan, observed, ['DUPLICATE_IDENTITY']);
});

test('missing remaining 4500000000 debt fails', () => {
  const observed = clone(loan);
  const settle = stageOf(observed, 'settle');
  settle.liabilities.nominalRemaining = '0';
  settle.liabilities.principal.outstandingNotional = '0';
  settle.residualDuties = [];
  assertFails(loan, observed, ['LIABILITY_MISMATCH', 'RESIDUAL_DUTY_MISMATCH']);
});

test('wrong due allocation fails', () => {
  const observed = clone(loan);
  const accrue = stageOf(observed, 'accrue');
  dueOf(accrue, 'lam01:period1:PR').outstanding = '33972602';
  dueOf(accrue, 'lam01:period1:PR').created = '33972602';
  dueOf(accrue, 'lam01:period1:IP').outstanding = '500000000';
  dueOf(accrue, 'lam01:period1:IP').created = '500000000';
  accrue.liabilities.principal.due = '33972602';
  accrue.liabilities.accrual.interestDue = '500000000';
  assertFails(loan, observed, ['DUE_MISMATCH']);
});

test('interest off by one fails', () => {
  const observed = clone(loan);
  const accrue = stageOf(observed, 'accrue');
  accrue.liabilities.accrual.interestCalculated = '33972601';
  accrue.liabilities.accrual.interestDue = '33972601';
  dueOf(accrue, 'lam01:period1:IP').created = '33972601';
  dueOf(accrue, 'lam01:period1:IP').outstanding = '33972601';
  assertFails(loan, observed, ['DUE_MISMATCH']);
});

test('floor remainder dust fails', () => {
  const loanObs = clone(loan);
  const accrue = stageOf(loanObs, 'accrue');
  accrue.liabilities.accrual.interestCalculated = '33972603';
  accrue.liabilities.accrual.interestDue = '33972603';
  dueOf(accrue, 'lam01:period1:IP').created = '33972603';
  dueOf(accrue, 'lam01:period1:IP').outstanding = '33972603';
  assertFails(loan, loanObs, ['DUE_MISMATCH']);
  const swapObs = clone(swap);
  const trade = stageOf(swapObs, 'swap');
  trade.transfers[1].amount = '19744';
  bal(trade.balances.post, 'trader|ASSET_B|AssetB_quantum').amount = '19744';
  bal(trade.balances.post, 'pool|ASSET_B|AssetB_quantum').amount = '1980256';
  effectOf(trade, 'trader|ASSET_B|AssetB_quantum').netCredit = '19744';
  effectOf(trade, 'pool|ASSET_B|AssetB_quantum').grossDebit = '19744';
  assertFails(swap, swapObs, ['TRANSFER_MISMATCH']);
});

test('swapped asset reserve fails', () => {
  const observed = clone(swap);
  const trade = stageOf(observed, 'swap');
  bal(trade.balances.post, 'pool|ASSET_A|AssetA_quantum').amount = '1980257';
  bal(trade.balances.post, 'pool|ASSET_B|AssetB_quantum').amount = '1010000';
  assertFails(swap, observed, ['BALANCE_INCONSISTENT']);
});

test('swap fee omitted fails', () => {
  const observed = clone(swap);
  const trade = stageOf(observed, 'swap');
  trade.economicFee.amount = '0';
  effectOf(trade, 'trader|ASSET_A|AssetA_quantum').fee = '0';
  trade.transfers[1].amount = '19801';
  bal(trade.balances.post, 'trader|ASSET_B|AssetB_quantum').amount = '19801';
  bal(trade.balances.post, 'pool|ASSET_B|AssetB_quantum').amount = '1980199';
  effectOf(trade, 'trader|ASSET_B|AssetB_quantum').netCredit = '19801';
  effectOf(trade, 'pool|ASSET_B|AssetB_quantum').grossDebit = '19801';
  assertFails(swap, observed, ['FEE_MISMATCH', 'TRANSFER_MISMATCH']);
});

test('swap fee credited twice fails', () => {
  const observed = clone(swap);
  const trade = stageOf(observed, 'swap');
  trade.economicFee.amount = '60';
  effectOf(trade, 'trader|ASSET_A|AssetA_quantum').fee = '60';
  trade.transfers.push({
    id: 'swap:trade:fee-again',
    ordinal: '2',
    from: 'trader',
    to: 'pool',
    color: 'ASSET_A',
    unit: 'AssetA_quantum',
    amount: '30'
  });
  assertFails(swap, observed, ['FEE_MISMATCH', 'EXTRA_TRANSFER']);
});

test('close reserve shortfall fails', () => {
  const observed = clone(swap);
  const close = stageOf(observed, 'close');
  close.transfers[0].amount = '1009999';
  bal(close.balances.post, 'provider|ASSET_A|AssetA_quantum').amount = '1009999';
  bal(close.balances.post, 'pool|ASSET_A|AssetA_quantum').amount = '1';
  effectOf(close, 'pool|ASSET_A|AssetA_quantum').grossDebit = '1009999';
  effectOf(close, 'provider|ASSET_A|AssetA_quantum').netCredit = '1009999';
  assertFails(swap, observed, ['TRANSFER_MISMATCH']);
});

test('wrong revision fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').revision = '1';
  assertFails(loan, observed, ['REVISION_MISMATCH']);
});

test('work refresh fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').work = '2';
  stageOf(observed, 'settle').remaining = '2';
  assertFails(loan, observed, ['WORK_MISMATCH', 'REMAINING_MISMATCH']);
});

test('gross refund net substitution fails', () => {
  const observed = clone(loan);
  const settle = stageOf(observed, 'settle');
  const borrower = effectOf(settle, 'borrower|USD_TEST_ASSET|USD_micro');
  borrower.grossDebit = '0';
  borrower.refund = '533972602';
  assertFails(loan, observed, ['GROSS_DEBIT_MISMATCH', 'REFUND_MISMATCH']);
});

test('net after fees shortfall fails', () => {
  const observed = clone(swap);
  const trade = stageOf(observed, 'swap');
  effectOf(trade, 'trader|ASSET_A|AssetA_quantum').grossDebit = '9970';
  effectOf(trade, 'trader|ASSET_B|AssetB_quantum').netCredit = '19801';
  trade.transfers[0].amount = '9970';
  trade.transfers[1].amount = '19801';
  trade.economicFee.amount = '0';
  effectOf(trade, 'trader|ASSET_A|AssetA_quantum').fee = '0';
  bal(trade.balances.post, 'trader|ASSET_A|AssetA_quantum').amount = '90030';
  bal(trade.balances.post, 'pool|ASSET_A|AssetA_quantum').amount = '1009970';
  bal(trade.balances.post, 'trader|ASSET_B|AssetB_quantum').amount = '19801';
  bal(trade.balances.post, 'pool|ASSET_B|AssetB_quantum').amount = '1980199';
  effectOf(trade, 'pool|ASSET_A|AssetA_quantum').netCredit = '9970';
  effectOf(trade, 'pool|ASSET_B|AssetB_quantum').grossDebit = '19801';
  assertFails(swap, observed, ['FEE_MISMATCH', 'TRANSFER_MISMATCH']);
});

test('unknown field fails', () => {
  const observed = clone(loan);
  observed.hostSuccess = true;
  assertFails(loan, observed, ['UNKNOWN_FIELD']);
});

test('overflow numeric fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').transfers[0].amount = '340282366920938463463374607431768211456';
  assertFails(loan, observed, ['INTEGER_OVERFLOW']);
});

test('newline numeric fails', () => {
  const observed = clone(loan);
  stageOf(observed, 'settle').transfers[0].amount = '533972602\n';
  assertFails(loan, observed, ['NONCANONICAL_INTEGER']);
});

test('malformed structure fails', () => {
  assertFails(loan, null, ['MALFORMED_STRUCTURE']);
  assertFails(loan, [], ['MALFORMED_STRUCTURE']);
  const missing = clone(loan);
  delete missing.stages;
  assertFails(loan, missing, ['MISSING_FIELD']);
});

test('inputs stay immutable', () => {
  const expected = freezeDeep(clone(loan));
  const observed = freezeDeep(clone(loan));
  const beforeExpected = JSON.stringify(expected);
  const beforeObserved = JSON.stringify(observed);
  const ok = compareFinancialEffects(expected, observed);
  assert.equal(ok.ok, true);
  const bad = clone(loan);
  stageOf(bad, 'settle').transfers[0].to = 'pool';
  const frozenBad = freezeDeep(bad);
  const beforeBad = JSON.stringify(frozenBad);
  const fail = compareFinancialEffects(expected, frozenBad);
  assert.equal(fail.ok, false);
  assert.equal(JSON.stringify(expected), beforeExpected);
  assert.equal(JSON.stringify(observed), beforeObserved);
  assert.equal(JSON.stringify(frozenBad), beforeBad);
});

test('error ordering is deterministic', () => {
  const observed = clone(loan);
  observed.extraFlag = 1;
  stageOf(observed, 'settle').transfers[0].to = 'pool';
  stageOf(observed, 'settle').revision = '9';
  const first = compareFinancialEffects(loan, observed);
  const second = compareFinancialEffects(loan, observed);
  assert.equal(first.ok, false);
  assert.equal(JSON.stringify(first.errors), JSON.stringify(second.errors));
  const sorted = [...first.errors].sort((a, b) => {
    if (a.path < b.path) return -1;
    if (a.path > b.path) return 1;
    if (a.code < b.code) return -1;
    if (a.code > b.code) return 1;
    return 0;
  });
  assert.deepEqual(first.errors, sorted);
  assert.ok(first.errors.length >= 2);
});

test('equal internally inconsistent records fail conservation', () => {
  const broken = clone(loan);
  bal(stageOf(broken, 'settle').balances.post, 'lender|USD_TEST_ASSET|USD_micro').amount = '1';
  const result = compareFinancialEffects(broken, clone(broken));
  assert.equal(result.ok, false);
  assertNetworkIncomplete(result);
  assert.ok(codes(result).includes('BALANCE_INCONSISTENT'));
});

function hasError(result, code, pathFragment) {
  return result.errors.some((item) => item.code === code && item.path.includes(pathFragment));
}

function assertNoThrowCompare(expected, observed) {
  const beforeExpected = JSON.stringify(expected);
  const beforeObserved = JSON.stringify(observed);
  let result;
  assert.doesNotThrow(() => {
    result = compareFinancialEffects(expected, observed);
  });
  assert.equal(typeof result, 'object');
  assert.equal(result.ok, false);
  assertNetworkIncomplete(result);
  assert.ok(Array.isArray(result.errors) && result.errors.length > 0);
  assert.equal(JSON.stringify(expected), beforeExpected);
  assert.equal(JSON.stringify(observed), beforeObserved);
  return result;
}

function assertEqualCloneFails(record, required) {
  const expected = clone(record);
  const observed = clone(record);
  const first = compareFinancialEffects(expected, observed);
  const second = compareFinancialEffects(expected, observed);
  assert.equal(first.ok, false);
  assertNetworkIncomplete(first);
  assert.equal(JSON.stringify(first.errors), JSON.stringify(second.errors));
  for (const item of required) {
    assert.ok(
      hasError(first, item.code, item.path),
      `missing ${item.code} at ${item.path} in ${JSON.stringify(first.errors)}`
    );
  }
  return first;
}

test('changed lender logicalId fails role binding comparison', () => {
  const observed = clone(loan);
  observed.deploymentBinding.roles.lender.logicalId = 'attacker';
  const result = assertFails(loan, observed, ['BINDING_MISMATCH']);
  assert.ok(hasError(result, 'BINDING_MISMATCH', 'deploymentBinding.roles.lender.logicalId'));
  assert.equal(result.networkAcceptance, false);
});

test('changed setup-mint status fails role binding comparison', () => {
  const observed = clone(loan);
  observed.deploymentBinding.roles['setup-mint'].status = 'symbolic-not-address';
  const result = assertFails(loan, observed, ['BINDING_MISMATCH']);
  assert.ok(hasError(result, 'BINDING_MISMATCH', 'deploymentBinding.roles.setup-mint.status'));
});

test('changed source pin fails admitted pin comparison', () => {
  const observed = clone(loan);
  observed.sourcePins.acceptedSourceSha256 = '0'.repeat(64);
  const result = assertFails(loan, observed, ['VALUE_MISMATCH']);
  assert.ok(hasError(result, 'VALUE_MISMATCH', 'sourcePins.acceptedSourceSha256'));
});

test('nested malformed containers return structured failures and do not throw', () => {
  const cases = [
    {
      title: 'observed missing settle economicFee',
      mutate(record) {
        delete stageOf(record, 'settle').economicFee;
      },
      asExpected: false,
      code: 'MISSING_FIELD',
      path: 'economicFee'
    },
    {
      title: 'observed null settle stage',
      mutate(record) {
        record.stages[2] = null;
      },
      asExpected: false,
      code: 'MALFORMED_STRUCTURE',
      path: 'stages'
    },
    {
      title: 'observed null role map',
      mutate(record) {
        record.deploymentBinding.roles = null;
      },
      asExpected: false,
      code: 'MALFORMED_STRUCTURE',
      path: 'deploymentBinding.roles'
    },
    {
      title: 'expected missing settle economicFee',
      mutate(record) {
        delete stageOf(record, 'settle').economicFee;
      },
      asExpected: true,
      code: 'MISSING_FIELD',
      path: 'economicFee'
    },
    {
      title: 'expected null settle stage',
      mutate(record) {
        record.stages[2] = null;
      },
      asExpected: true,
      code: 'MALFORMED_STRUCTURE',
      path: 'stages'
    },
    {
      title: 'expected null role map',
      mutate(record) {
        record.deploymentBinding.roles = null;
      },
      asExpected: true,
      code: 'MALFORMED_STRUCTURE',
      path: 'deploymentBinding.roles'
    }
  ];
  for (const item of cases) {
    const mutated = clone(loan);
    item.mutate(mutated);
    const expected = item.asExpected ? mutated : loan;
    const observed = item.asExpected ? loan : mutated;
    const result = assertNoThrowCompare(expected, observed);
    assert.ok(
      hasError(result, item.code, item.path),
      `${item.title}: missing ${item.code} at ${item.path} in ${JSON.stringify(result.errors)}`
    );
  }
});

test('equal erased remaining debts fail internal liability validation', () => {
  const broken = clone(loan);
  const settle = stageOf(broken, 'settle');
  settle.liabilities.nominalRemaining = '0';
  settle.liabilities.principal.outstandingNotional = '0';
  settle.residualDuties = [];
  assertEqualCloneFails(broken, [
    { code: 'LIABILITY_MISMATCH', path: 'nominalRemaining' },
    { code: 'LIABILITY_MISMATCH', path: 'outstandingNotional' },
    { code: 'RESIDUAL_DUTY_MISMATCH', path: 'residualDuties' }
  ]);
});

test('equal contradictory interestCalculated fails due validation', () => {
  const broken = clone(loan);
  stageOf(broken, 'accrue').liabilities.accrual.interestCalculated = '1';
  const notional = 5000000000n;
  const interest = (notional * 8n * 31n) / (100n * 365n);
  assert.equal(interest, 33972602n);
  assert.equal(dueOf(stageOf(broken, 'accrue'), 'lam01:period1:IP').outstanding, '33972602');
  assertEqualCloneFails(broken, [
    { code: 'DUE_MISMATCH', path: 'interestCalculated' }
  ]);
});

test('equal contradictory principal due outstanding fails due validation', () => {
  const broken = clone(loan);
  dueOf(stageOf(broken, 'accrue'), 'lam01:period1:PR').outstanding = '1';
  assert.equal(dueOf(stageOf(broken, 'accrue'), 'lam01:period1:PR').created, '500000000');
  assert.equal(stageOf(broken, 'accrue').liabilities.principal.due, '500000000');
  assertEqualCloneFails(broken, [
    { code: 'DUE_MISMATCH', path: 'lam01:period1:PR' }
  ]);
});

test('equal discontinuous accrue cash fails stage continuity', () => {
  const broken = clone(loan);
  const accrue = stageOf(broken, 'accrue');
  bal(accrue.balances.pre, 'borrower|USD_TEST_ASSET|USD_micro').amount = '1';
  bal(accrue.balances.post, 'borrower|USD_TEST_ASSET|USD_micro').amount = '1';
  assert.equal(
    bal(stageOf(broken, 'setup').balances.post, 'borrower|USD_TEST_ASSET|USD_micro').amount,
    '20000000000'
  );
  assert.equal(
    bal(stageOf(broken, 'settle').balances.pre, 'borrower|USD_TEST_ASSET|USD_micro').amount,
    '20000000000'
  );
  assertEqualCloneFails(broken, [
    { code: 'VALUE_MISMATCH', path: 'stages[accrue].balances' }
  ]);
});

test('equal contradictory swap economicFee fails fee reconciliation', () => {
  const broken = clone(swap);
  const trade = stageOf(broken, 'swap');
  trade.economicFee.amount = '31';
  assert.equal(effectOf(trade, 'trader|ASSET_A|AssetA_quantum').fee, '30');
  const amountIn = 10000n;
  const fee = amountIn - (amountIn * 997n) / 1000n;
  assert.equal(fee, 30n);
  assertEqualCloneFails(broken, [
    { code: 'FEE_MISMATCH', path: 'economicFee.amount' }
  ]);
});

test('equal contradictory trader fee effect fails fee reconciliation', () => {
  const broken = clone(swap);
  const trade = stageOf(broken, 'swap');
  effectOf(trade, 'trader|ASSET_A|AssetA_quantum').fee = '0';
  assert.equal(trade.economicFee.amount, '30');
  assertEqualCloneFails(broken, [
    { code: 'FEE_MISMATCH', path: 'trader|ASSET_A|AssetA_quantum' }
  ]);
});

test('equal refreshed close revision and work fail lifecycle', () => {
  const broken = clone(swap);
  const close = stageOf(broken, 'close');
  close.work = '100';
  close.revision = '77';
  assert.equal(close.remaining, '6');
  assert.equal(broken.agreement.lifetime, '8');
  assertEqualCloneFails(broken, [
    { code: 'REVISION_MISMATCH', path: 'stages[close].revision' },
    { code: 'WORK_MISMATCH', path: 'stages[close].work' }
  ]);
});

test('equal missing settle actorEffects fail required coverage', () => {
  const broken = clone(loan);
  stageOf(broken, 'settle').actorEffects = [];
  assertEqualCloneFails(broken, [
    { code: 'MISSING_FIELD', path: 'actorEffects' }
  ]);
});

test('equal missing settle balances and effects fail required coverage', () => {
  const broken = clone(loan);
  const settle = stageOf(broken, 'settle');
  settle.balances = { pre: [], post: [] };
  settle.actorEffects = [];
  assert.equal(settle.transfers.length, 1);
  assertEqualCloneFails(broken, [
    { code: 'MISSING_FIELD', path: 'balances' },
    { code: 'MISSING_FIELD', path: 'actorEffects' }
  ]);
});

test('equal duplicate close transfer ordinal fails identity', () => {
  const broken = clone(swap);
  const close = stageOf(broken, 'close');
  close.transfers[1].ordinal = '0';
  assert.equal(close.transfers[0].ordinal, '0');
  assertEqualCloneFails(broken, [
    { code: 'DUPLICATE_IDENTITY', path: 'ordinal' }
  ]);
});

test('equal duplicate borrower role binding fails identity', () => {
  const broken = clone(loan);
  stageOf(broken, 'setup').bindings.roles.push('borrower');
  assertEqualCloneFails(broken, [
    { code: 'DUPLICATE_IDENTITY', path: 'bindings.roles' }
  ]);
});

test('equal quantum zero fails positive quantum', () => {
  const broken = clone(loan);
  broken.deploymentBinding.assets[0].quantum = '0';
  for (const stage of broken.stages) {
    stage.bindings.assets[0].quantum = '0';
  }
  assertEqualCloneFails(broken, [
    { code: 'QUANTUM_MISMATCH', path: 'quantum' }
  ]);
});
