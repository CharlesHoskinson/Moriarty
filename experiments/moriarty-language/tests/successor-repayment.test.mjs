import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  prepareRepayment,
  REPAYMENT_VERSION,
  REPAYMENT_BOUNDS,
} from '../src/successor/repayment.ts';

const examplePath = join(
  dirname(fileURLToPath(import.meta.url)),
  '../spec/successor/examples/funded-repayment.json',
);

const U128 = REPAYMENT_BOUNDS.uint128Max;
const U128_PLUS_1 = (BigInt(U128) + 1n).toString();

function due100(over = {}) {
  const conversion = {
    mantissa: '1',
    scale: '0',
    rounding: 'none',
    ...(over.conversion || {}),
  };
  const rest = { ...over };
  delete rest.conversion;
  return {
    id: 'Due100',
    debtor: 'Payer',
    creditor: 'Lender',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    principal: '100',
    accrued: '0',
    outstanding: '100',
    allocationRule: 'AccrualFirst',
    conversion,
    status: 'Outstanding',
    ...rest,
  };
}

function obligationWith(principal, accrued, extra = {}) {
  const outstanding = (BigInt(principal) + BigInt(accrued)).toString();
  return due100({
    principal,
    accrued,
    outstanding,
    status: outstanding === '0' ? 'Settled' : 'Outstanding',
    ...extra,
  });
}

function transfer(over = {}) {
  return {
    kind: 'Transfer',
    id: 'T1',
    from: 'Payer',
    to: 'Lender',
    asset: 'Cash',
    amount: '30',
    ...over,
  };
}

function repay(over = {}) {
  return {
    kind: 'Repay',
    allocationId: 'Alloc1',
    transferId: 'T1',
    obligationId: 'Due100',
    payer: 'Payer',
    nominalAmount: '30',
    ...over,
  };
}

function defaultState(over = {}) {
  return {
    balances: [
      { party: 'Payer', asset: 'Cash', amount: '100' },
      { party: 'Lender', asset: 'Cash', amount: '0' },
    ],
    allowances: [
      { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
    ],
    obligations: [due100()],
    usedTransferIds: [],
    usedAllocationIds: [],
    work: { remaining: '100', spent: '0', closureReserve: '16' },
    ...over,
  };
}

function src(over = {}) {
  const body = {
    schemaVersion: over.schemaVersion ?? REPAYMENT_VERSION,
    state: defaultState(over.state || {}),
    actions: over.actions ?? [transfer(), repay()],
  };
  if (over.extraRoot) Object.assign(body, over.extraRoot);
  return JSON.stringify(body);
}

function ids(n, prefix) {
  return Array.from({ length: n }, (_, i) => prefix + String(i));
}

function assertNoBigInt(value) {
  assert.notEqual(typeof value, 'bigint');
  if (Array.isArray(value)) {
    for (const item of value) assertNoBigInt(item);
    return;
  }
  if (value && typeof value === 'object') {
    for (const inner of Object.values(value)) assertNoBigInt(inner);
  }
}

function assertRejected(result, actionIndex) {
  assert.equal(result.status, 'Rejected');
  assert.equal(result.actionIndex, actionIndex);
  assert.equal(typeof result.code, 'string');
  assert.ok(result.code.length > 0);
  assert.deepEqual(Object.keys(result).sort(), ['actionIndex', 'code', 'status']);
  assertNoBigInt(result);
}

function assertPrepared(result) {
  assert.equal(result.status, 'Prepared');
  assert.equal(result.schemaVersion, REPAYMENT_VERSION);
  assert.equal(Array.isArray(result.effects), true);
  assert.equal(result.post && typeof result.post, 'object');
  assert.deepEqual(Object.keys(result).sort(), ['effects', 'post', 'schemaVersion', 'status']);
  assert.deepEqual(
    Object.keys(result.post).sort(),
    ['allowances', 'balances', 'obligations', 'usedAllocationIds', 'usedTransferIds', 'work'],
  );
  assertNoBigInt(result);
}

function bal(post, party, asset) {
  const row = post.balances.find((item) => item.party === party && item.asset === asset);
  assert.ok(row);
  return row.amount;
}

function allowanceOf(post, party, asset) {
  const row = post.allowances.find((item) => item.party === party && item.asset === asset);
  assert.ok(row);
  return row;
}

function obl(post, id) {
  const row = post.obligations.find((item) => item.id === id);
  assert.ok(row);
  return row;
}

function tokenState(over = {}) {
  return {
    balances: [
      { party: 'Payer', asset: 'USD_TOKEN', amount: '100' },
      { party: 'Lender', asset: 'USD_TOKEN', amount: '0' },
    ],
    allowances: [
      { party: 'Payer', asset: 'USD_TOKEN', remaining: '100', spent: '0' },
    ],
    ...over,
  };
}

function tokenObligation(over = {}) {
  return due100({
    denomination: 'USD_micro',
    settlementAsset: 'USD_TOKEN',
    ...over,
  });
}

function tokenTransfer(over = {}) {
  return transfer({ asset: 'USD_TOKEN', ...over });
}

describe('exports',
  () => {
    test('REPAYMENT_VERSION and immutable REPAYMENT_BOUNDS',
      () => {
        assert.equal(REPAYMENT_VERSION, 'moriarty-funded-repayment/0');
        assert.equal(REPAYMENT_BOUNDS.sourceUtf8Bytes, 65536);
        assert.equal(REPAYMENT_BOUNDS.collectionCapacity, 128);
        assert.equal(REPAYMENT_BOUNDS.identifierCharacters, 64);
        assert.equal(REPAYMENT_BOUNDS.maxScale, 18);
        assert.equal(REPAYMENT_BOUNDS.uint128Max, '340282366920938463463374607431768211455');
        const before = REPAYMENT_BOUNDS.sourceUtf8Bytes;
        try {
          REPAYMENT_BOUNDS.sourceUtf8Bytes = 1;
        } catch {
          /* frozen or accessor */
        }
        assert.equal(REPAYMENT_BOUNDS.sourceUtf8Bytes, before);
        assert.equal(typeof prepareRepayment, 'function');
      });
  });

describe('example fixture file',
  () => {
    const example = readFileSync(examplePath, 'utf8');

    test('exact compact JSON without trailing newline and accepted by prepareRepayment',
      () => {
        assert.equal(example.endsWith('\n'), false);
        assert.equal(example.endsWith('\r'), false);
        assert.equal(JSON.stringify(JSON.parse(example)), example);
        assert.equal(example, src());
        const result = prepareRepayment(example);
        assertPrepared(result);
        assert.equal(bal(result.post, 'Payer', 'Cash'), '70');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '30');
        assert.equal(obl(result.post, 'Due100').principal, '70');
        assert.equal(obl(result.post, 'Due100').outstanding, '70');
        assert.equal(result.post.work.remaining, '98');
        assert.equal(result.post.work.spent, '2');
        assert.equal(result.post.work.closureReserve, '16');
      });

    test('trailing newline is not a valid source',
      () => {
        assertRejected(prepareRepayment(example + '\n'), null);
      });
  });

describe('positive funded repayment',
  () => {
    test('due100 principal 100 / accrued 0, Transfer 30 then Repay 30',
      () => {
        const source = src();
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects.length, 2);
        assert.equal(bal(result.post, 'Payer', 'Cash'), '70');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '30');
        assert.deepEqual(obl(result.post, 'Due100'), {
          id: 'Due100',
          debtor: 'Payer',
          creditor: 'Lender',
          denomination: 'Cash',
          settlementAsset: 'Cash',
          principal: '70',
          accrued: '0',
          outstanding: '70',
          allocationRule: 'AccrualFirst',
          conversion: { mantissa: '1', scale: '0', rounding: 'none' },
          status: 'Outstanding',
        });
        assert.deepEqual(allowanceOf(result.post, 'Payer', 'Cash'), {
          party: 'Payer',
          asset: 'Cash',
          remaining: '70',
          spent: '30',
        });
        assert.deepEqual(result.post.work, {
          remaining: '98',
          spent: '2',
          closureReserve: '16',
        });
        assert.deepEqual(result.post.usedTransferIds, ['T1']);
        assert.deepEqual(result.post.usedAllocationIds, ['Alloc1']);
        assert.deepEqual(result.effects[0], transfer());
        assert.deepEqual(result.effects[1], {
          kind: 'Repayment',
          allocationId: 'Alloc1',
          transferId: 'T1',
          obligationId: 'Due100',
          payer: 'Payer',
          creditor: 'Lender',
          denomination: 'Cash',
          settlementAsset: 'Cash',
          nominalAmount: '30',
          settlementAmount: '30',
          principalDischarged: '30',
          accruedDischarged: '0',
          remainingOutstanding: '70',
        });
        assert.deepEqual(prepareRepayment(source), result);
        assert.equal(source, src());
      });

    test('TX02 AccrualFirst P100/A10 Transfer7/Repay7 leaves P100/A3/outstanding 103',
      () => {
        const source = src({
          state: { obligations: [obligationWith('100', '10')] },
          actions: [transfer({ amount: '7' }), repay({ nominalAmount: '7' })],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        const debt = obl(result.post, 'Due100');
        assert.equal(debt.principal, '100');
        assert.equal(debt.accrued, '3');
        assert.equal(debt.outstanding, '103');
        assert.equal(debt.status, 'Outstanding');
        assert.equal(bal(result.post, 'Payer', 'Cash'), '93');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '7');
        assert.equal(result.effects[1].principalDischarged, '0');
        assert.equal(result.effects[1].accruedDischarged, '7');
        assert.equal(result.effects[1].remainingOutstanding, '103');
        assert.equal(result.effects[1].settlementAmount, '7');
      });

    test('PrincipalFirst P100/A10 N7 discharges 93/10',
      () => {
        const source = src({
          state: {
            obligations: [obligationWith('100', '10', { allocationRule: 'PrincipalFirst' })],
          },
          actions: [transfer({ amount: '7' }), repay({ nominalAmount: '7' })],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        const debt = obl(result.post, 'Due100');
        assert.equal(debt.principal, '93');
        assert.equal(debt.accrued, '10');
        assert.equal(debt.outstanding, '103');
        assert.equal(result.effects[1].principalDischarged, '7');
        assert.equal(result.effects[1].accruedDischarged, '0');
      });

    test('ProRata P100/A10 N7 is floor(7*100/110)=6 so 94/9',
      () => {
        const source = src({
          state: {
            obligations: [obligationWith('100', '10', { allocationRule: 'ProRata' })],
          },
          actions: [transfer({ amount: '7' }), repay({ nominalAmount: '7' })],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        const debt = obl(result.post, 'Due100');
        assert.equal(debt.principal, '94');
        assert.equal(debt.accrued, '9');
        assert.equal(debt.outstanding, '103');
        assert.equal(result.effects[1].principalDischarged, '6');
        assert.equal(result.effects[1].accruedDischarged, '1');
      });

    test('two allocations 10+20 of transfer 30 to distinct obligations succeed',
      () => {
        const source = src({
          state: {
            obligations: [
              obligationWith('50', '0', { id: 'D1' }),
              obligationWith('50', '0', { id: 'D2' }),
            ],
          },
          actions: [
            transfer({ amount: '30' }),
            repay({ allocationId: 'Alloc1', obligationId: 'D1', nominalAmount: '10' }),
            repay({ allocationId: 'Alloc2', obligationId: 'D2', nominalAmount: '20' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects.length, 3);
        assert.equal(obl(result.post, 'D1').outstanding, '40');
        assert.equal(obl(result.post, 'D2').outstanding, '30');
        assert.equal(bal(result.post, 'Payer', 'Cash'), '70');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '30');
        assert.deepEqual(result.post.usedAllocationIds, ['Alloc1', 'Alloc2']);
        assert.equal(result.post.work.remaining, '97');
        assert.equal(result.post.work.spent, '3');
        assert.equal(result.post.work.closureReserve, '16');
        assert.equal(result.effects[1].settlementAmount, '10');
        assert.equal(result.effects[2].settlementAmount, '20');
      });

    test('same allocation id reused across two Repay actions rejects',
      () => {
        const source = src({
          state: {
            obligations: [
              obligationWith('50', '0', { id: 'D1' }),
              obligationWith('50', '0', { id: 'D2' }),
            ],
          },
          actions: [
            transfer({ amount: '30' }),
            repay({ allocationId: 'Alloc1', obligationId: 'D1', nominalAmount: '10' }),
            repay({ allocationId: 'Alloc1', obligationId: 'D2', nominalAmount: '20' }),
          ],
        });
        const result = prepareRepayment(source);
        assertRejected(result, 2);
      });

    test('20+20 against a transfer of 30 rejects the over-allocation',
      () => {
        const source = src({
          state: {
            obligations: [
              obligationWith('50', '0', { id: 'D1' }),
              obligationWith('50', '0', { id: 'D2' }),
            ],
          },
          actions: [
            transfer({ amount: '30' }),
            repay({ allocationId: 'Alloc1', obligationId: 'D1', nominalAmount: '20' }),
            repay({ allocationId: 'Alloc2', obligationId: 'D2', nominalAmount: '20' }),
          ],
        });
        const result = prepareRepayment(source);
        assertRejected(result, 2);
      });

    test('exact full repayment keeps Settled tombstone; reuse of post rejects; independent second discharge works',
      () => {
        const source = src({
          state: {
            obligations: [
              obligationWith('30', '0', { id: 'D1' }),
              obligationWith('50', '0', { id: 'D2' }),
            ],
          },
          actions: [
            transfer({ id: 'T1', amount: '30' }),
            repay({
              allocationId: 'Alloc1',
              transferId: 'T1',
              obligationId: 'D1',
              nominalAmount: '30',
            }),
          ],
        });
        const first = prepareRepayment(source);
        assertPrepared(first);
        const settled = obl(first.post, 'D1');
        assert.equal(settled.status, 'Settled');
        assert.equal(settled.principal, '0');
        assert.equal(settled.accrued, '0');
        assert.equal(settled.outstanding, '0');
        assert.equal(settled.id, 'D1');
        assert.equal(settled.debtor, 'Payer');
        assert.equal(settled.creditor, 'Lender');
        assert.equal(settled.allocationRule, 'AccrualFirst');
        assert.equal(obl(first.post, 'D2').outstanding, '50');
        assert.equal(first.post.obligations.length, 2);

        const reuse = JSON.stringify({
          schemaVersion: REPAYMENT_VERSION,
          state: first.post,
          actions: JSON.parse(source).actions,
        });
        assertRejected(prepareRepayment(reuse), 0);

        const secondSource = JSON.stringify({
          schemaVersion: REPAYMENT_VERSION,
          state: first.post,
          actions: [
            transfer({ id: 'T2', amount: '10' }),
            repay({
              allocationId: 'Alloc2',
              transferId: 'T2',
              obligationId: 'D2',
              nominalAmount: '10',
            }),
          ],
        });
        const second = prepareRepayment(secondSource);
        assertPrepared(second);
        assert.equal(obl(second.post, 'D1').status, 'Settled');
        assert.equal(obl(second.post, 'D1').outstanding, '0');
        assert.equal(obl(second.post, 'D2').outstanding, '40');
        assert.equal(obl(second.post, 'D2').status, 'Outstanding');
        assert.equal(bal(second.post, 'Payer', 'Cash'), '60');
        assert.equal(bal(second.post, 'Lender', 'Cash'), '40');
        assert.deepEqual(second.post.usedTransferIds, ['T1', 'T2']);
        assert.deepEqual(second.post.usedAllocationIds, ['Alloc1', 'Alloc2']);
        assert.equal(second.post.work.remaining, '96');
        assert.equal(second.post.work.spent, '4');
        assert.equal(second.post.work.closureReserve, '16');
      });

    test('nominal USD_micro settled via USD_TOKEN mantissa 2 scale 0: N30 -> cash 60',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '2', scale: '0', rounding: 'none' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '60' }),
            repay({ nominalAmount: '30' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(bal(result.post, 'Payer', 'USD_TOKEN'), '40');
        assert.equal(bal(result.post, 'Lender', 'USD_TOKEN'), '60');
        assert.equal(obl(result.post, 'Due100').denomination, 'USD_micro');
        assert.equal(obl(result.post, 'Due100').settlementAsset, 'USD_TOKEN');
        assert.equal(obl(result.post, 'Due100').principal, '70');
        assert.equal(obl(result.post, 'Due100').outstanding, '70');
        assert.equal(result.effects[1].settlementAmount, '60');
        assert.equal(result.effects[1].nominalAmount, '30');
        assert.equal(result.effects[1].denomination, 'USD_micro');
        assert.equal(result.effects[1].settlementAsset, 'USD_TOKEN');
      });

    test('floor conversion 4 * 3 / 10^1 = 1 remainder 2 => settlement 1',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '3', scale: '1', rounding: 'floor' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '1' }),
            repay({ nominalAmount: '4' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects[1].settlementAmount, '1');
        assert.equal(result.effects[1].principalDischarged, '4');
        assert.equal(bal(result.post, 'Payer', 'USD_TOKEN'), '99');
        assert.equal(bal(result.post, 'Lender', 'USD_TOKEN'), '1');
        assert.equal(obl(result.post, 'Due100').principal, '96');
      });

    test('ceil conversion 4 * 3 / 10^1 = 1 remainder 2 => settlement 2',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '3', scale: '1', rounding: 'ceil' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '2' }),
            repay({ nominalAmount: '4' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects[1].settlementAmount, '2');
        assert.equal(bal(result.post, 'Lender', 'USD_TOKEN'), '2');
      });

    test('none conversion rejects 4 * 3 / 10^1 because remainder is 2',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '3', scale: '1', rounding: 'none' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '10' }),
            repay({ nominalAmount: '4' }),
          ],
        });
        assertRejected(prepareRepayment(source), 1);
      });

    test('exact division 10 * 3 / 10^1 remainder 0 yields settlement 3 for none, floor, and ceil',
      () => {
        for (const rounding of ['none', 'floor', 'ceil']) {
          const source = src({
            state: tokenState({
              obligations: [
                tokenObligation({
                  conversion: { mantissa: '3', scale: '1', rounding },
                }),
              ],
            }),
            actions: [
              tokenTransfer({ amount: '3' }),
              repay({ nominalAmount: '10' }),
            ],
          });
          const result = prepareRepayment(source);
          assertPrepared(result);
          assert.equal(result.effects[1].settlementAmount, '3');
          assert.equal(obl(result.post, 'Due100').principal, '90');
        }
      });

    test('floor conversion 1 * 1 / 10^1 = 0 is dust and rejects',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '1', scale: '1', rounding: 'floor' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '1' }),
            repay({ nominalAmount: '1' }),
          ],
        });
        assertRejected(prepareRepayment(source), 1);
      });

    test('none conversion 1 * 1 / 10^1 is inexact and rejects',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '1', scale: '1', rounding: 'none' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '1' }),
            repay({ nominalAmount: '1' }),
          ],
        });
        assertRejected(prepareRepayment(source), 1);
      });

    test('ceil conversion 1 * 1 / 10^1 remainder 1 yields settlement 1',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: { mantissa: '1', scale: '1', rounding: 'ceil' },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '1' }),
            repay({ nominalAmount: '1' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects[1].settlementAmount, '1');
      });

    test('scale 18 exact: mantissa 10^18, N 1, settlement 1',
      () => {
        const source = src({
          state: tokenState({
            obligations: [
              tokenObligation({
                conversion: {
                  mantissa: '1000000000000000000',
                  scale: '18',
                  rounding: 'none',
                },
              }),
            ],
          }),
          actions: [
            tokenTransfer({ amount: '1' }),
            repay({ nominalAmount: '1' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects[1].settlementAmount, '1');
      });

    test('third-party funded repayment does not require debtor == payer',
      () => {
        const source = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '100' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
              { party: 'Borrower', asset: 'Cash', amount: '5' },
            ],
            allowances: [
              { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
              { party: 'Borrower', asset: 'Cash', remaining: '5', spent: '0' },
            ],
            obligations: [due100({ debtor: 'Borrower' })],
          },
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(bal(result.post, 'Payer', 'Cash'), '70');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '30');
        assert.equal(bal(result.post, 'Borrower', 'Cash'), '5');
        assert.deepEqual(allowanceOf(result.post, 'Borrower', 'Cash'), {
          party: 'Borrower',
          asset: 'Cash',
          remaining: '5',
          spent: '0',
        });
        assert.equal(obl(result.post, 'Due100').debtor, 'Borrower');
        assert.equal(result.effects[1].payer, 'Payer');
        assert.equal(result.effects[1].creditor, 'Lender');
      });

    test('untouched unrelated records and histories are preserved',
      () => {
        const frozen = obligationWith('0', '0', {
          id: 'Frozen',
          debtor: 'Borrower',
          creditor: 'Lender',
          denomination: 'Bond',
          settlementAsset: 'Bond',
        });
        const source = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '100' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
              { party: 'Watcher', asset: 'Bond', amount: '9' },
            ],
            allowances: [
              { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
              { party: 'Watcher', asset: 'Bond', remaining: '3', spent: '1' },
            ],
            obligations: [due100(), frozen],
            usedTransferIds: ['OldT'],
            usedAllocationIds: ['OldA'],
          },
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(bal(result.post, 'Watcher', 'Bond'), '9');
        assert.deepEqual(allowanceOf(result.post, 'Watcher', 'Bond'), {
          party: 'Watcher',
          asset: 'Bond',
          remaining: '3',
          spent: '1',
        });
        assert.deepEqual(obl(result.post, 'Frozen'), frozen);
        assert.deepEqual(result.post.usedTransferIds, ['OldT', 'T1']);
        assert.deepEqual(result.post.usedAllocationIds, ['OldA', 'Alloc1']);
      });

    test('refund transfer does not restore the earlier sender allowance; remaining+spent is conserved',
      () => {
        const source = src({
          state: {
            allowances: [
              { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
              { party: 'Lender', asset: 'Cash', remaining: '50', spent: '0' },
            ],
          },
          actions: [
            transfer({ id: 'T1', from: 'Payer', to: 'Lender', amount: '30' }),
            transfer({ id: 'T2', from: 'Lender', to: 'Payer', amount: '30' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(bal(result.post, 'Payer', 'Cash'), '100');
        assert.equal(bal(result.post, 'Lender', 'Cash'), '0');
        assert.deepEqual(allowanceOf(result.post, 'Payer', 'Cash'), {
          party: 'Payer',
          asset: 'Cash',
          remaining: '70',
          spent: '30',
        });
        assert.deepEqual(allowanceOf(result.post, 'Lender', 'Cash'), {
          party: 'Lender',
          asset: 'Cash',
          remaining: '20',
          spent: '30',
        });
        assert.equal(
          (BigInt(allowanceOf(result.post, 'Payer', 'Cash').remaining) +
            BigInt(allowanceOf(result.post, 'Payer', 'Cash').spent)).toString(),
          '100',
        );
        assert.equal(
          (BigInt(allowanceOf(result.post, 'Lender', 'Cash').remaining) +
            BigInt(allowanceOf(result.post, 'Lender', 'Cash').spent)).toString(),
          '50',
        );
        assert.equal(result.post.work.remaining, '98');
        assert.equal(result.post.work.spent, '2');
        assert.equal(result.post.work.closureReserve, '16');
      });

    test('receiver balance is appended when absent',
      () => {
        const source = src({
          state: {
            balances: [{ party: 'Payer', asset: 'Cash', amount: '100' }],
          },
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.deepEqual(result.post.balances, [
          { party: 'Payer', asset: 'Cash', amount: '70' },
          { party: 'Lender', asset: 'Cash', amount: '30' },
        ]);
      });

    test('transfer-only candidate moves cash and spends work without touching debt',
      () => {
        const source = src({
          actions: [transfer({ amount: '30' })],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.effects.length, 1);
        assert.equal(bal(result.post, 'Payer', 'Cash'), '70');
        assert.equal(obl(result.post, 'Due100').outstanding, '100');
        assert.deepEqual(result.post.usedAllocationIds, []);
        assert.equal(result.post.work.remaining, '99');
        assert.equal(result.post.work.spent, '1');
      });

    test('transfer id and allocation id may share a string because namespaces are separate',
      () => {
        const source = src({
          actions: [
            transfer({ id: 'Shared' }),
            repay({ allocationId: 'Shared', transferId: 'Shared' }),
          ],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.deepEqual(result.post.usedTransferIds, ['Shared']);
        assert.deepEqual(result.post.usedAllocationIds, ['Shared']);
      });

    test('work remaining equals action count is accepted; closureReserve stays put',
      () => {
        const source = src({
          state: { work: { remaining: '2', spent: '8', closureReserve: '16' } },
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.deepEqual(result.post.work, {
          remaining: '0',
          spent: '10',
          closureReserve: '16',
        });
      });
  });

describe('input admission',
  () => {
    test('non-string hostile values are rejected without calling coercion traps',
      () => {
        const hostile = {
          toString() {
            throw new Error('toString-called');
          },
          valueOf() {
            throw new Error('valueOf-called');
          },
          [Symbol.toPrimitive]() {
            throw new Error('toPrimitive-called');
          },
        };
        const values = [
          hostile,
          Object.create(hostile),
          new String(src()),
          0,
          false,
          null,
          undefined,
          1n,
          { source: src() },
          ['[]'],
        ];
        for (const value of values) {
          const result = prepareRepayment(value);
          assertRejected(result, null);
        }
      });

    test('empty string, non-object JSON, and invalid JSON reject with actionIndex null',
      () => {
        for (const source of ['', 'null', '[]', 'true', '1', '{', '{]', '"x"']) {
          assertRejected(prepareRepayment(source), null);
        }
      });

    test('lone surrogates reject before shape execution',
      () => {
        assertRejected(prepareRepayment('\uD800'), null);
        assertRejected(prepareRepayment('\uDEAD'), null);
        assertRejected(prepareRepayment('{"x":"\uD800"}'), null);
        assertRejected(prepareRepayment(src().slice(0, 8) + '\uDFFF' + src().slice(8)), null);
      });

    test('UTF16 length above 65536 rejects',
      () => {
        assertRejected(prepareRepayment('a'.repeat(65537)), null);
      });

    test('UTF16 length 65536 of non-JSON still admits encoding then rejects parse',
      () => {
        assertRejected(prepareRepayment('a'.repeat(65536)), null);
      });

    test('UTF8 byte length above 65536 with UTF16 length under the bound rejects',
      () => {
        assertRejected(prepareRepayment('\u00e9'.repeat(32769)), null);
      });

    test('whitespace, duplicate keys, and nonminimal number forms fail the stringify roundtrip',
      () => {
        const compact = src();
        assertRejected(prepareRepayment(JSON.stringify(JSON.parse(compact), null, 2)), null);
        assertRejected(prepareRepayment(compact + ' '), null);
        assertRejected(prepareRepayment(' ' + compact), null);
        const duplicateParty = compact.replace('"party":"Payer"', '"party":"Payer","party":"Payer"');
        assertRejected(prepareRepayment(duplicateParty), null);
        const scientific = compact.replace('"amount":"100"', '"amount":1e2');
        assertRejected(prepareRepayment(scientific), null);
        const unicodeEscape = compact.replace('"Due100"', '"\\u0044ue100"');
        assertRejected(prepareRepayment(unicodeEscape), null);
      });

    test('JSON numeric amounts that survive stringify still fail schema',
      () => {
        const numeric = src().replace('"amount":"100"', '"amount":100');
        assert.equal(JSON.stringify(JSON.parse(numeric)), numeric);
        assertRejected(prepareRepayment(numeric), null);
      });

    test('wrong schemaVersion rejects',
      () => {
        assertRejected(prepareRepayment(src({ schemaVersion: 'moriarty-funded-repayment/1' })), null);
        assertRejected(prepareRepayment(src({ schemaVersion: 'moriarty-funded-repayment/0 ' })), null);
      });

    test('unknown root, nested, action, and profile labels reject schema and do not steer the predicate',
      () => {
        assertRejected(prepareRepayment(src({ extraRoot: { label: 'must-accept' } })), null);
        assertRejected(prepareRepayment(src({ extraRoot: { label: 'must-reject' } })), null);
        assertRejected(prepareRepayment(src({ extraRoot: { proofs: [] } })), null);
        const proto = '{"__proto__":{"polluted":"yes"},' + src().slice(1);
        assertRejected(prepareRepayment(proto), null);

        const nested = JSON.parse(src());
        nested.state.obligations[0].conversion.mode = 'bankers';
        assertRejected(prepareRepayment(JSON.stringify(nested)), null);

        const obligationLabel = JSON.parse(src());
        obligationLabel.state.obligations[0].fixture = 'ForcePrepared';
        assertRejected(prepareRepayment(JSON.stringify(obligationLabel)), null);

        const workLimit = JSON.parse(src());
        workLimit.state.work.limit = '0';
        assertRejected(prepareRepayment(JSON.stringify(workLimit)), null);

        const memo = JSON.parse(src());
        memo.actions[0].memo = 'hi';
        assertRejected(prepareRepayment(JSON.stringify(memo)), null);

        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ allocationRule: 'ShouldSucceed' })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ allocationRule: 'ForcePrepared' })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(
            src({
              actions: [{ kind: 'Mint', id: 'M1', from: 'Payer', to: 'Lender', asset: 'Cash', amount: '1' }],
            }),
          ),
          null,
        );
        assertRejected(
          prepareRepayment(
            src({
              actions: [{ kind: 'Repayment', allocationId: 'Alloc1', transferId: 'T1', obligationId: 'Due100', payer: 'Payer', nominalAmount: '30' }],
            }),
          ),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              obligations: [due100({ conversion: { rounding: 'halfup' } })],
            },
          })),
          null,
        );
      });

    test('missing closed-shape fields reject with actionIndex null',
      () => {
        for (const key of ['schemaVersion', 'state', 'actions']) {
          const parsed = JSON.parse(src());
          delete parsed[key];
          assertRejected(prepareRepayment(JSON.stringify(parsed)), null);
        }
        for (const key of ['balances', 'allowances', 'obligations', 'usedTransferIds', 'usedAllocationIds', 'work']) {
          const parsed = JSON.parse(src());
          delete parsed.state[key];
          assertRejected(prepareRepayment(JSON.stringify(parsed)), null);
        }
        for (const key of ['kind', 'id', 'from', 'to', 'asset', 'amount']) {
          const action = transfer();
          delete action[key];
          assertRejected(prepareRepayment(src({ actions: [action] })), null);
        }
        for (const key of ['kind', 'allocationId', 'transferId', 'obligationId', 'payer', 'nominalAmount']) {
          const action = repay();
          delete action[key];
          assertRejected(prepareRepayment(src({ actions: [action] })), null);
        }
        for (const key of [
          'id',
          'debtor',
          'creditor',
          'denomination',
          'settlementAsset',
          'principal',
          'accrued',
          'outstanding',
          'allocationRule',
          'conversion',
          'status',
        ]) {
          const obligation = due100();
          delete obligation[key];
          assertRejected(prepareRepayment(src({ state: { obligations: [obligation] } })), null);
        }
        for (const key of ['mantissa', 'scale', 'rounding']) {
          const obligation = due100();
          delete obligation.conversion[key];
          assertRejected(prepareRepayment(src({ state: { obligations: [obligation] } })), null);
        }
      });

    test('duplicate collection identities reject with actionIndex null',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: 'Payer', asset: 'Cash', amount: '100' },
                { party: 'Payer', asset: 'Cash', amount: '50' },
              ],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: [
                { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
                { party: 'Payer', asset: 'Cash', remaining: '10', spent: '0' },
              ],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              obligations: [due100(), due100({ principal: '40', outstanding: '40' })],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedTransferIds: ['T9', 'T9'] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedAllocationIds: ['A9', 'A9'] },
          })),
          null,
        );
      });

    test('non-canonical decimals, bad identifiers, and invariant mismatches reject',
      () => {
        for (const amount of ['00', '01', '100.0', '1e2', '+100', '-1', '', '0x1', ' 100']) {
          assertRejected(
            prepareRepayment(src({
              state: {
                balances: [
                  { party: 'Payer', asset: 'Cash', amount },
                  { party: 'Lender', asset: 'Cash', amount: '0' },
                ],
              },
            })),
            null,
          );
        }
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: '1Payer', asset: 'Cash', amount: '100' },
                { party: 'Lender', asset: 'Cash', amount: '0' },
              ],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: 'Payer!', asset: 'Cash', amount: '100' },
                { party: 'Lender', asset: 'Cash', amount: '0' },
              ],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: 'PayerX'.slice(0, 5) + ' ', asset: 'Cash', amount: '100' },
                { party: 'Lender', asset: 'Cash', amount: '0' },
              ],
            },
          })),
          null,
        );
        const tooLong = 'A' + 'b'.repeat(64);
        assert.equal(tooLong.length, 65);
        assertRejected(
          prepareRepayment(src({
            actions: [transfer({ id: tooLong })],
          })),
          null,
        );
        const maxId = 'A' + 'b'.repeat(63);
        assert.equal(maxId.length, 64);
        const maxOk = prepareRepayment(src({
          actions: [transfer({ id: maxId }), repay({ transferId: maxId })],
        }));
        assertPrepared(maxOk);

        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ principal: '100', accrued: '10', outstanding: '100' })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ outstanding: '0', status: 'Outstanding' })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              obligations: [due100({ principal: '0', accrued: '0', outstanding: '0', status: 'Outstanding' })],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ status: 'Settled' })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ conversion: { mantissa: '0' } })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ conversion: { scale: '19' } })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { obligations: [due100({ conversion: { scale: '00' } })] },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              work: { remaining: '100', spent: U128, closureReserve: '1' },
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: [{ party: 'Payer', asset: 'Cash', remaining: '1', spent: U128 }],
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: 'Payer', asset: 'Cash', amount: U128_PLUS_1 },
                { party: 'Lender', asset: 'Cash', amount: '0' },
              ],
            },
          })),
          null,
        );
      });

    test('empty actions and over-capacity collections reject with actionIndex null',
      () => {
        assertRejected(prepareRepayment(src({ actions: [] })), null);
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: ids(129, 'P').map((party) => ({ party, asset: 'Cash', amount: '0' })),
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: ids(129, 'P').map((party) => ({
                party,
                asset: 'Cash',
                remaining: '0',
                spent: '0',
              })),
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              obligations: ids(129, 'D').map((id) => obligationWith('1', '0', { id })),
            },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedTransferIds: ids(129, 'U') },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedAllocationIds: ids(129, 'A') },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { work: { remaining: '200', spent: '0', closureReserve: '0' } },
            actions: ids(129, 'T').map((id) => transfer({ id, amount: '1' })),
          })),
          null,
        );
      });
  });

describe('work and capacity',
  () => {
    test('insufficient ordinary work rejects before commit; closureReserve cannot pay ordinary work',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: { work: { remaining: '1', spent: '0', closureReserve: '16' } },
          })),
          null,
        );
        assertRejected(
          prepareRepayment(src({
            state: { work: { remaining: '0', spent: '0', closureReserve: '50' } },
            actions: [transfer({ amount: '1' })],
          })),
          null,
        );
      });

    test('post-collection usedTransferIds capacity 128 is enforced',
      () => {
        const full = src({
          state: { usedTransferIds: ids(128, 'U') },
          actions: [transfer({ id: 'TNew', amount: '1' })],
        });
        assertRejected(prepareRepayment(full), 0);
        const almost = src({
          state: { usedTransferIds: ids(127, 'U') },
          actions: [transfer({ id: 'TNew', amount: '1' })],
        });
        const result = prepareRepayment(almost);
        assertPrepared(result);
        assert.equal(result.post.usedTransferIds.length, 128);
      });

    test('appending a new receiver balance beyond 128 rejects the transfer',
      () => {
        const balances = [
          { party: 'Payer', asset: 'Cash', amount: '100' },
          ...ids(127, 'Q').map((party) => ({ party, asset: 'Cash', amount: '0' })),
        ];
        assert.equal(balances.length, 128);
        assertRejected(
          prepareRepayment(src({
            state: { balances },
            actions: [transfer({ amount: '1' })],
          })),
          0,
        );
      });

    test('usedAllocationIds at capacity 128 rejects a new allocation',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: { usedAllocationIds: ids(128, 'A') },
          })),
          1,
        );
      });
  });

describe('transfer and repay negatives',
  () => {
    test('zero amount and self-transfer reject at the transfer index',
      () => {
        assertRejected(prepareRepayment(src({ actions: [transfer({ amount: '0' })] })), 0);
        assertRejected(
          prepareRepayment(src({
            actions: [transfer({ from: 'Payer', to: 'Payer' })],
          })),
          0,
        );
      });

    test('insufficient cash, missing sender balance, missing allowance, and insufficient cap',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [
                { party: 'Payer', asset: 'Cash', amount: '10' },
                { party: 'Lender', asset: 'Cash', amount: '0' },
              ],
            },
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              balances: [{ party: 'Lender', asset: 'Cash', amount: '0' }],
            },
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: { allowances: [] },
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: [{ party: 'Payer', asset: 'Cash', remaining: '10', spent: '0' }],
            },
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: [{ party: 'Payer', asset: 'Bond', remaining: '100', spent: '0' }],
            },
          })),
          0,
        );
      });

    test('missing funding, future funding, and prior-step transfer cannot fund a Repay',
      () => {
        assertRejected(prepareRepayment(src({ actions: [repay()] })), 0);
        assertRejected(
          prepareRepayment(src({
            actions: [repay(), transfer()],
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedTransferIds: ['T1'] },
            actions: [repay()],
          })),
          0,
        );
        assertRejected(
          prepareRepayment(src({
            state: { usedTransferIds: ['T1'] },
          })),
          0,
        );
      });

    test('wrong sender, receiver, or settlement asset rejects the Repay after a successful Transfer',
      () => {
        const wrongPayer = src({
          actions: [transfer(), repay({ payer: 'Watcher' })],
        });
        assertRejected(prepareRepayment(wrongPayer), 1);

        const wrongReceiver = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '100' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
              { party: 'Watcher', asset: 'Cash', amount: '0' },
            ],
          },
          actions: [transfer({ to: 'Watcher' }), repay()],
        });
        assertRejected(prepareRepayment(wrongReceiver), 1);

        const wrongAsset = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '100' },
              { party: 'Payer', asset: 'Bond', amount: '100' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
              { party: 'Lender', asset: 'Bond', amount: '0' },
            ],
            allowances: [
              { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
              { party: 'Payer', asset: 'Bond', remaining: '100', spent: '0' },
            ],
          },
          actions: [transfer({ asset: 'Bond', amount: '30' }), repay()],
        });
        assertRejected(prepareRepayment(wrongAsset), 1);
      });

    test('transfer 1 cannot fund nominal 20 at 1:1',
      () => {
        const source = src({
          actions: [transfer({ amount: '1' }), repay({ nominalAmount: '20' })],
        });
        assertRejected(prepareRepayment(source), 1);
      });

    test('zero nominalAmount rejects the Repay',
      () => {
        assertRejected(
          prepareRepayment(src({
            actions: [transfer(), repay({ nominalAmount: '0' })],
          })),
          1,
        );
      });

    test('nominal above outstanding, Settled target, and unknown obligation reject',
      () => {
        assertRejected(
          prepareRepayment(src({
            actions: [transfer({ amount: '100' }), repay({ nominalAmount: '101' })],
          })),
          1,
        );
        assertRejected(
          prepareRepayment(src({
            state: {
              obligations: [obligationWith('0', '0')],
              work: { remaining: '100', spent: '0', closureReserve: '16' },
            },
            actions: [transfer({ amount: '1' }), repay({ nominalAmount: '1' })],
          })),
          1,
        );
        assertRejected(
          prepareRepayment(src({
            actions: [transfer(), repay({ obligationId: 'Missing' })],
          })),
          1,
        );
      });

    test('allocation id already in history rejects',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: { usedAllocationIds: ['Alloc1'] },
          })),
          1,
        );
      });

    test('duplicate transfer id in this step rejects the later transfer',
      () => {
        assertRejected(
          prepareRepayment(src({
            state: {
              allowances: [
                { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
                { party: 'Lender', asset: 'Cash', remaining: '50', spent: '0' },
              ],
            },
            actions: [
              transfer({ id: 'T1', amount: '10' }),
              transfer({ id: 'T1', from: 'Lender', to: 'Payer', amount: '1' }),
            ],
          })),
          1,
        );
      });

    test('late failure after a successful Transfer returns no post or effects and leaves the source unchanged',
      () => {
        const source = src({
          actions: [transfer({ amount: '30' }), repay({ nominalAmount: '0' })],
        });
        const frozen = source;
        const result = prepareRepayment(source);
        assertRejected(result, 1);
        assert.equal(source, frozen);
        assert.equal(source, src({
          actions: [transfer({ amount: '30' }), repay({ nominalAmount: '0' })],
        }));
        const again = prepareRepayment(source);
        assert.deepEqual(again, result);
      });

    test('rejected codes are stable nonempty strings for the same source',
      () => {
        const source = src({ actions: [transfer({ amount: '0' })] });
        const a = prepareRepayment(source);
        const b = prepareRepayment(source);
        assertRejected(a, 0);
        assertRejected(b, 0);
        assert.equal(a.code, b.code);
      });
  });

describe('uint128 and intermediate overflow',
  () => {
    test('uint128Max is a legal amount; uint128Max+1 is not',
      () => {
        const atBound = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: U128 },
              { party: 'Lender', asset: 'Cash', amount: '0' },
            ],
            allowances: [{ party: 'Payer', asset: 'Cash', remaining: U128, spent: '0' }],
            obligations: [obligationWith('0', '0', { id: 'Frozen' })],
          },
          actions: [transfer({ amount: U128 })],
        });
        const prepared = prepareRepayment(atBound);
        assertPrepared(prepared);
        assert.equal(bal(prepared.post, 'Payer', 'Cash'), '0');
        assert.equal(bal(prepared.post, 'Lender', 'Cash'), U128);

        const over = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: U128_PLUS_1 },
              { party: 'Lender', asset: 'Cash', amount: '0' },
            ],
          },
        });
        assertRejected(prepareRepayment(over), null);
      });

    test('crediting a receiver already at uint128Max overflows the Transfer',
      () => {
        const source = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '1' },
              { party: 'Lender', asset: 'Cash', amount: U128 },
            ],
            allowances: [{ party: 'Payer', asset: 'Cash', remaining: '1', spent: '0' }],
          },
          actions: [transfer({ amount: '1' })],
        });
        assertRejected(prepareRepayment(source), 0);
      });

    test('ProRata intermediate N*P must fit UInt128 before division',
      () => {
        const source = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '2' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
            ],
            allowances: [{ party: 'Payer', asset: 'Cash', remaining: '2', spent: '0' }],
            obligations: [obligationWith(U128, '0', { allocationRule: 'ProRata' })],
          },
          actions: [transfer({ amount: '2' }), repay({ nominalAmount: '2' })],
        });
        assertRejected(prepareRepayment(source), 1);
      });

    test('conversion product nominal*mantissa overflow rejects the Repay after Transfer',
      () => {
        const source = src({
          state: {
            balances: [
              { party: 'Payer', asset: 'Cash', amount: '1' },
              { party: 'Lender', asset: 'Cash', amount: '0' },
            ],
            allowances: [{ party: 'Payer', asset: 'Cash', remaining: '1', spent: '0' }],
            obligations: [
              obligationWith(U128, '0', {
                conversion: { mantissa: '2', scale: '0', rounding: 'none' },
              }),
            ],
          },
          actions: [transfer({ amount: '1' }), repay({ nominalAmount: U128 })],
        });
        const result = prepareRepayment(source);
        assertRejected(result, 1);
      });

    test('work spent at uint128Max-1 with one action succeeds and writes uint128Max',
      () => {
        const spent = (BigInt(U128) - 1n).toString();
        const source = src({
          state: {
            work: { remaining: '1', spent, closureReserve: '0' },
          },
          actions: [transfer({ amount: '1' })],
        });
        const result = prepareRepayment(source);
        assertPrepared(result);
        assert.equal(result.post.work.remaining, '0');
        assert.equal(result.post.work.spent, U128);
        assert.equal(result.post.work.closureReserve, '0');
      });
  });
