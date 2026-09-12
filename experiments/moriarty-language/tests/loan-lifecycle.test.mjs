import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { formatFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5-frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';
import {
  admitFinancialLifecycleStateJSON,
  LIFECYCLE_BOUNDS,
} from '../src/successor/financial-lifecycle.ts';
import {
  evaluateLoanLifecycleAction,
  loadLoanLifecycle,
  runLoanLifecycleDemo,
} from '../examples/loan-lifecycle.mjs';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const oracle = JSON.parse(readFileSync(new URL(
  '../../../deliverables/language-to-ledger-2026-09-12/design-review/lifecycle-source-expectations.json',
  import.meta.url,
), 'utf8'));

// Hand-counted dynamic constructors: Eprefix + N + Esuffix. N is kernel actions 2/1/2/2.
// Counts are from the frozen source body, not evaluator remaining.
const WORK = Object.freeze({
  originate: Object.freeze({ prefix: 36, N: 2, suffix: 61, W: 99, staticBound: 97 }),
  accrue: Object.freeze({ prefix: 10, N: 1, suffix: 54, W: 65, staticBound: 64 }),
  repay: Object.freeze({ prefix: 35, N: 2, suffix: 51, W: 88, staticBound: 86 }),
  settle: Object.freeze({ prefix: 33, N: 2, suffix: 51, W: 86, staticBound: 84 }),
});
const SEED_REMAINING = 512n;
const SEED_SPENT = 17n;
const RESERVE = '16';
const C = {
  originate: WORK.originate.W,
  accrue: WORK.originate.W + WORK.accrue.W,
  repay: WORK.originate.W + WORK.accrue.W + WORK.repay.W,
  settle: WORK.originate.W + WORK.accrue.W + WORK.repay.W + WORK.settle.W,
};

function remainingAfter(stage) {
  return String(SEED_REMAINING - BigInt(C[stage]));
}
function spentAfter(stage) {
  return String(SEED_SPENT + BigInt(C[stage]));
}

const FORBIDDEN = ['post', 'financialPost', 'effects', 'descriptors', 'continueSuffix', 'workRemaining'];

const ORIGINATE_ARGS = { originationId: 'O1', transferId: 'D1' };
const ACCRUE_ARGS = { accrualId: 'A1', observedTime: '1060', periodIndex: '1' };
const REPAY_ARGS = { allocationId: 'R1', nominal: '30', transferId: 'P1' };
const SETTLE_ARGS = { allocationId: 'R2', transferId: 'P2' };

function stageTemplate(name) {
  return oracle.stages.find((item) => item.name === name).expectedResultTemplate;
}

function expectedStage(name) {
  const template = structuredClone(stageTemplate(name));
  const remaining = remainingAfter(name === 'originate100' ? 'originate'
    : name === 'accrue10' ? 'accrue'
      : name === 'repay30' ? 'repay' : 'settle');
  const spent = spentAfter(name === 'originate100' ? 'originate'
    : name === 'accrue10' ? 'accrue'
      : name === 'repay30' ? 'repay' : 'settle');
  template.financialPost.work = { remaining, spent, closureReserve: RESERVE };
  template.workRemaining = remaining;
  return template;
}

function assertRejected(result, code) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.code, code, JSON.stringify(result));
  for (const key of FORBIDDEN) assert.equal(Object.hasOwn(result, key), false, key);
  return result;
}

function countConstructors(node) {
  let count = 0;
  const pending = [node];
  while (pending.length) {
    const value = pending.pop();
    if (value === null || typeof value !== 'object') continue;
    if (Object.hasOwn(value, 'constructor') && typeof value.constructor === 'string') count++;
    pending.push(...Object.values(value));
  }
  return count;
}

function splitActionWork(elaborated, action) {
  const artifact = elaborated.actions.find((item) => item.action === action);
  const statements = artifact.core.statements;
  const suffixIndex = statements.findIndex((node) => node.constructor === 'Ensure');
  return {
    prefix: countConstructors(statements.slice(0, suffixIndex)),
    suffix: countConstructors(statements.slice(suffixIndex)),
    staticBound: artifact.staticWorkBound,
  };
}

function seedState(over = {}) {
  return {
    schemaVersion: 'moriarty-financial-lifecycle-state/1',
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
    work: { remaining: '512', spent: '17', closureReserve: '16' },
    ...over,
  };
}

const unrelatedObligation = {
  id: 'OtherLoan',
  debtor: 'Other',
  creditor: 'Lender',
  denomination: 'Token',
  settlementAsset: 'Token',
  principal: '0',
  accrued: '0',
  outstanding: '0',
  allocationRule: 'AccrualFirst',
  conversion: { mantissa: '1', scale: '0', rounding: 'none' },
  status: 'Settled',
  originationId: 'OX',
  originationTransferId: 'DX',
  initialPrincipal: '5',
  nominalLiabilityCap: '5',
  liabilityIncurred: '5',
  accrualTerms: {
    numerator: '1',
    denominator: '10',
    rounding: 'floor',
    periodSeconds: '60',
    firstPeriodStart: '1000',
  },
  lastAccruedPeriod: '0',
  nextAccrualAt: '1060',
};

function withWork(state, remaining, spent = '17') {
  const copy = structuredClone(state);
  copy.work = { remaining: String(remaining), spent: String(spent), closureReserve: RESERVE };
  return copy;
}

test('loan-lifecycle source checks four actions and frozen constructor counts', () => {
  const { source, language } = loadLoanLifecycle();
  const checked = language.check(source);
  assert.equal(checked.judgmentResult, 'SourceChecked', JSON.stringify(checked));
  assert.deepEqual(checked.actions.map((item) => item.action), ['originate', 'accrue', 'repay', 'settle']);
  assert.deepEqual(
    checked.actions.map((item) => item.staticWorkBound),
    [String(WORK.originate.staticBound), String(WORK.accrue.staticBound), String(WORK.repay.staticBound), String(WORK.settle.staticBound)],
  );
  const elaborated = language.elaborate(source);
  for (const [action, expected] of Object.entries(WORK)) {
    const split = splitActionWork(elaborated, action);
    assert.equal(split.prefix, expected.prefix, `${action} prefix`);
    assert.equal(split.suffix, expected.suffix, `${action} suffix`);
    assert.equal(Number(split.staticBound), expected.prefix + expected.suffix, `${action} static`);
    assert.equal(expected.W, expected.prefix + expected.N + expected.suffix, `${action} W`);
  }
});

test('complete source lifecycle matches the independent oracle with actual predecessor posts', () => {
  const loaded = loadLoanLifecycle();
  const { source, language, stateText, snapshotsText } = loaded;
  assert.equal(JSON.parse(stateText).work.remaining, '512');
  assert.equal(JSON.parse(stateText).work.spent, '17');
  assert.equal(JSON.parse(stateText).work.closureReserve, '16');
  assert.equal(snapshotsText, canonical({
    Args: ORIGINATE_ARGS,
    Obs: {},
    Pre: { paid: '0', phase: '0' },
    workInitial: '512',
  }));

  const originated = language.evaluate(source, 'originate', snapshotsText, stateText);
  assert.deepEqual(originated, expectedStage('originate100'));
  assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(originated.financialPost)).ok, true);
  assert.equal(BigInt(originated.financialPost.work.remaining) + BigInt(originated.financialPost.work.spent), 529n);
  assert.equal(
    BigInt(originated.financialPost.work.remaining)
      + BigInt(originated.financialPost.work.spent)
      + BigInt(originated.financialPost.work.closureReserve),
    545n,
  );

  const accruedCall = evaluateLoanLifecycleAction(language, source, 'accrue', originated, ACCRUE_ARGS);
  assert.deepEqual(accruedCall.snapshot.Pre, originated.post);
  assert.equal(accruedCall.snapshot.workInitial, originated.financialPost.work.remaining);
  assert.deepEqual(accruedCall.financialPre, originated.financialPost);
  assert.deepEqual(accruedCall.result, expectedStage('accrue10'));
  assert.equal(accruedCall.result.financialPost.obligations[0].principal, '100');
  assert.equal(accruedCall.result.financialPost.obligations[0].accrued, '10');
  assert.equal(accruedCall.result.financialPost.obligations[0].outstanding, '110');

  const repaidCall = evaluateLoanLifecycleAction(language, source, 'repay', accruedCall.result, REPAY_ARGS);
  assert.deepEqual(repaidCall.snapshot.Pre, accruedCall.result.post);
  assert.equal(repaidCall.snapshot.workInitial, accruedCall.result.financialPost.work.remaining);
  assert.deepEqual(repaidCall.result, expectedStage('repay30'));
  assert.equal(repaidCall.result.effects[1].principalDischarged, '20');
  assert.equal(repaidCall.result.effects[1].accruedDischarged, '10');

  const settledCall = evaluateLoanLifecycleAction(language, source, 'settle', repaidCall.result, SETTLE_ARGS);
  assert.deepEqual(settledCall.snapshot.Pre, repaidCall.result.post);
  assert.equal(settledCall.snapshot.workInitial, repaidCall.result.financialPost.work.remaining);
  assert.equal(Object.keys(settledCall.snapshot.Args).sort().join(','), 'allocationId,transferId');
  assert.equal('nominal' in settledCall.snapshot.Args, false);
  assert.deepEqual(settledCall.result, expectedStage('settle80'));
  assert.equal(settledCall.result.financialPost.obligations[0].status, 'Settled');
  assert.equal(settledCall.result.financialPost.obligations[0].outstanding, '0');
  assert.equal(settledCall.result.financialPost.balances.find((row) => row.party === 'Lender').amount, '110');
  assert.equal(settledCall.result.financialPost.balances.find((row) => row.party === 'Borrower').amount, '0');
  assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(settledCall.result.financialPost)).ok, true);

  const wrongSplit = structuredClone(settledCall.result.financialPost);
  wrongSplit.obligations[0].outstanding = '1';
  wrongSplit.obligations[0].principal = '1';
  assert.notDeepEqual(wrongSplit, expectedStage('settle80').financialPost);
  const missingOther = structuredClone(settledCall.result.financialPost);
  missingOther.balances = missingOther.balances.filter((row) => row.party !== 'Other');
  assert.notDeepEqual(missingOther, expectedStage('settle80').financialPost);
});

test('named kernel and source failures publish no candidate and preserve the predecessor', () => {
  const { source, language, stateText, snapshotsText } = loadLoanLifecycle();
  const originated = language.evaluate(source, 'originate', snapshotsText, stateText);
  const accrued = evaluateLoanLifecycleAction(language, source, 'accrue', originated, ACCRUE_ARGS).result;
  const repaid = evaluateLoanLifecycleAction(language, source, 'repay', accrued, REPAY_ARGS).result;
  const settled = evaluateLoanLifecycleAction(language, source, 'settle', repaid, SETTLE_ARGS).result;

  const duplicate = evaluateLoanLifecycleAction(language, source, 'accrue', accrued, ACCRUE_ARGS).result;
  assertRejected(duplicate, 'DUPLICATE');
  const duplicateAgain = evaluateLoanLifecycleAction(language, source, 'accrue', accrued, ACCRUE_ARGS).result;
  assert.deepEqual(duplicateAgain, duplicate);
  const afterDuplicate = evaluateLoanLifecycleAction(language, source, 'repay', accrued, REPAY_ARGS).result;
  assert.deepEqual(afterDuplicate, expectedStage('repay30'));

  const sequence = evaluateLoanLifecycleAction(language, source, 'accrue', accrued, {
    accrualId: 'A2', observedTime: '1120', periodIndex: '1',
  }).result;
  assertRejected(sequence, 'PERIOD_SEQUENCE');

  const early = evaluateLoanLifecycleAction(language, source, 'accrue', originated, {
    accrualId: 'A1', observedTime: '1059', periodIndex: '1',
  }).result;
  assertRejected(early, 'PERIOD_NOT_ELIGIBLE');
  const retryEarly = evaluateLoanLifecycleAction(language, source, 'accrue', originated, ACCRUE_ARGS).result;
  assert.deepEqual(retryEarly, expectedStage('accrue10'));

  const cap = evaluateLoanLifecycleAction(language, source, 'accrue', repaid, {
    accrualId: 'A2', observedTime: '1120', periodIndex: '2',
  }).result;
  assertRejected(cap, 'LIABILITY_CAP_EXCEEDED');
  const settleAfterCap = evaluateLoanLifecycleAction(language, source, 'settle', repaid, SETTLE_ARGS).result;
  assert.deepEqual(settleAfterCap, expectedStage('settle80'));

  const funding = evaluateLoanLifecycleAction(language, source, 'repay', accrued, {
    allocationId: 'RF', nominal: '111', transferId: 'PF',
  }).result;
  assertRejected(funding, 'INSUFFICIENT_BALANCE');
  const retryFunding = evaluateLoanLifecycleAction(language, source, 'repay', accrued, {
    allocationId: 'RF', nominal: '30', transferId: 'PF',
  }).result;
  assert.equal(retryFunding.status, 'FundedExpressionPrepared');
  assert.equal(retryFunding.financialPost.usedTransferIds.includes('PF'), true);
  assert.equal(retryFunding.financialPost.usedAllocationIds.includes('RF'), true);
  assert.equal(retryFunding.financialPost.obligations[0].outstanding, '80');
  assert.equal(retryFunding.post.paid, '30');

  const missingTransfer = source.replace(
    `emit Transfer {
      id: transferId,
      from: "Lender",
      to: "Borrower",
      settlementAsset: "Cash",
      transferAmount: amount<Cash>(100)
    };
    emit Originate`,
    'emit Originate',
  );
  assertRejected(language.evaluate(missingTransfer, 'originate', snapshotsText, stateText), 'TRANSFER_NOT_IN_STEP');

  const falseEnsure = source.replace(
    'ensures post_balance<Cash>("Lender") == amount<Cash>(110);',
    'ensures post_balance<Cash>("Lender") == amount<Cash>(111);',
  );
  const late = evaluateLoanLifecycleAction(
    createFinancialAgreementSourceV5(),
    falseEnsure,
    'settle',
    repaid,
    SETTLE_ARGS,
  ).result;
  assertRejected(late, 'ENSURES_FAILED');
  assert.equal(late.workUsed, String(WORK.settle.W));
  const retryEnsure = evaluateLoanLifecycleAction(language, source, 'settle', repaid, SETTLE_ARGS).result;
  assert.deepEqual(retryEnsure, expectedStage('settle80'));

  const afterSettle = evaluateLoanLifecycleAction(language, source, 'settle', settled, {
    allocationId: 'R3', transferId: 'P3',
  }).result;
  assertRejected(afterSettle, 'GUARD_FAILED');
  assert.equal(afterSettle.code === 'NOT_OUTSTANDING', false);
});

test('exact work W succeeds remaining 0; W-1, prefix+N-1 and prefix+N reject without output', () => {
  const { source, language, stateText, snapshotsText } = loadLoanLifecycle();
  const originated = language.evaluate(source, 'originate', snapshotsText, stateText);
  const accrued = evaluateLoanLifecycleAction(language, source, 'accrue', originated, ACCRUE_ARGS).result;
  const repaid = evaluateLoanLifecycleAction(language, source, 'repay', accrued, REPAY_ARGS).result;

  const cases = [
    {
      action: 'originate',
      predecessor: {
        post: { phase: '0', paid: '0' },
        financialPost: JSON.parse(stateText),
        status: 'FundedExpressionPrepared',
      },
      args: ORIGINATE_ARGS,
      work: WORK.originate,
      expected: expectedStage('originate100'),
    },
    {
      action: 'accrue',
      predecessor: originated,
      args: ACCRUE_ARGS,
      work: WORK.accrue,
      expected: expectedStage('accrue10'),
    },
    {
      action: 'repay',
      predecessor: accrued,
      args: REPAY_ARGS,
      work: WORK.repay,
      expected: expectedStage('repay30'),
    },
    {
      action: 'settle',
      predecessor: repaid,
      args: SETTLE_ARGS,
      work: WORK.settle,
      expected: expectedStage('settle80'),
    },
  ];

  for (const item of cases) {
    const exactState = withWork(item.predecessor.financialPost, item.work.W, '17');
    const exactPred = {
      ...item.predecessor,
      financialPost: exactState,
    };
    const exact = evaluateLoanLifecycleAction(language, source, item.action, exactPred, item.args).result;
    assert.equal(exact.status, 'FundedExpressionPrepared', item.action);
    assert.equal(exact.financialPost.work.remaining, '0');
    assert.equal(exact.financialPost.work.spent, String(17 + item.work.W));
    assert.equal(exact.financialPost.work.closureReserve, RESERVE);
    assert.deepEqual(exact.post, item.expected.post);
    assert.deepEqual(exact.effects, item.expected.effects);

    const minus = evaluateLoanLifecycleAction(
      language,
      source,
      item.action,
      { ...item.predecessor, financialPost: withWork(item.predecessor.financialPost, item.work.W - 1, '17') },
      item.args,
    ).result;
    assertRejected(minus, 'WORK_EXHAUSTED');

    const kernelShort = evaluateLoanLifecycleAction(
      language,
      source,
      item.action,
      {
        ...item.predecessor,
        financialPost: withWork(item.predecessor.financialPost, item.work.prefix + item.work.N - 1, '17'),
      },
      item.args,
    ).result;
    assertRejected(kernelShort, 'INSUFFICIENT_WORK');

    const suffixStart = evaluateLoanLifecycleAction(
      language,
      source,
      item.action,
      {
        ...item.predecessor,
        financialPost: withWork(item.predecessor.financialPost, item.work.prefix + item.work.N, '17'),
      },
      item.args,
    ).result;
    assertRejected(suffixStart, 'WORK_EXHAUSTED');
  }
});

test('ordinary phase offset survives every stage and unrelated obligation histories append', () => {
  const { source, language } = loadLoanLifecycle();
  const offsetSeed = seedState();
  const offsetOrigin = language.evaluate(
    source,
    'originate',
    canonical({ Args: ORIGINATE_ARGS, Obs: {}, Pre: { phase: '7', paid: '0' }, workInitial: '512' }),
    JSON.stringify(offsetSeed),
  );
  assert.equal(offsetOrigin.status, 'FundedExpressionPrepared');
  assert.deepEqual(offsetOrigin.post, { phase: '8', paid: '0' });
  const offsetAccrue = evaluateLoanLifecycleAction(language, source, 'accrue', offsetOrigin, ACCRUE_ARGS).result;
  assert.deepEqual(offsetAccrue.post, { phase: '9', paid: '0' });
  const offsetRepay = evaluateLoanLifecycleAction(language, source, 'repay', offsetAccrue, REPAY_ARGS).result;
  assert.deepEqual(offsetRepay.post, { phase: '10', paid: '30' });
  const offsetSettle = evaluateLoanLifecycleAction(language, source, 'settle', offsetRepay, SETTLE_ARGS).result;
  assert.deepEqual(offsetSettle.post, { phase: '11', paid: '110' });

  const related = seedState({
    obligations: [unrelatedObligation],
    usedTransferIds: ['DX'],
    usedOriginationIds: ['OX'],
  });
  assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(related)).ok, true);
  const kept = language.evaluate(
    source,
    'originate',
    canonical({ Args: ORIGINATE_ARGS, Obs: {}, Pre: { phase: '0', paid: '0' }, workInitial: '512' }),
    JSON.stringify(related),
  );
  assert.equal(kept.status, 'FundedExpressionPrepared', JSON.stringify(kept));
  assert.equal(kept.financialPost.obligations[0].id, 'OtherLoan');
  assert.deepEqual(kept.financialPost.obligations[0], unrelatedObligation);
  assert.equal(kept.financialPost.obligations[1].id, 'Loan1');
  assert.deepEqual(kept.financialPost.usedOriginationIds, ['OX', 'O1']);
  assert.deepEqual(kept.financialPost.usedTransferIds, ['DX', 'D1']);
  const keptAccrue = evaluateLoanLifecycleAction(language, source, 'accrue', kept, ACCRUE_ARGS).result;
  assert.deepEqual(keptAccrue.financialPost.obligations[0], unrelatedObligation);
  assert.deepEqual(keptAccrue.financialPost.usedAccrualIds, ['A1']);
  assert.equal(keptAccrue.financialPost.obligations[1].lastAccruedPeriod, '1');
});

test('malformed unrelated allowance is admission rejection, not a lifecycle continuation', () => {
  const { source, language } = loadLoanLifecycle();
  const malformed = seedState();
  malformed.allowances[2] = {
    party: 'Other',
    asset: 'Token',
    remaining: LIFECYCLE_BOUNDS.uint128Max,
    spent: '1',
  };
  const rejected = language.evaluate(
    source,
    'originate',
    canonical({ Args: ORIGINATE_ARGS, Obs: {}, Pre: { phase: '0', paid: '0' }, workInitial: '512' }),
    JSON.stringify(malformed),
  );
  assertRejected(rejected, 'INVARIANT');
});

test('demo captures actual inputs, duplicate accrual and failed settled payment', () => {
  const trace = runLoanLifecycleDemo();
  assert.equal(trace.checked.judgmentResult, 'SourceChecked');
  assert.equal(trace.calls.length >= 6, true);
  const names = trace.calls.map((item) => item.action + ':' + item.result.status);
  assert.equal(names.includes('originate:FundedExpressionPrepared'), true);
  assert.equal(names.includes('accrue:FundedExpressionPrepared'), true);
  assert.equal(names.includes('accrue:Rejected'), true);
  assert.equal(names.includes('repay:FundedExpressionPrepared'), true);
  assert.equal(names.includes('settle:FundedExpressionPrepared'), true);
  const duplicate = trace.calls.find((item) => item.action === 'accrue' && item.result.status === 'Rejected');
  assert.equal(duplicate.result.code, 'DUPLICATE');
  const failedSettle = trace.calls.find((item) => item.action === 'settle' && item.result.status === 'Rejected');
  assert.equal(failedSettle.result.code, 'GUARD_FAILED');
  let predecessor = null;
  for (const call of trace.calls) {
    if (predecessor !== null && call.result.status === 'FundedExpressionPrepared') {
      assert.deepEqual(call.snapshot.Pre, predecessor.post);
      assert.equal(call.snapshot.workInitial, predecessor.financialPost.work.remaining);
      assert.deepEqual(JSON.parse(call.financialJSON), predecessor.financialPost);
    }
    if (call.result.status === 'FundedExpressionPrepared') predecessor = call.result;
  }
  assert.deepEqual(trace.settled, expectedStage('settle80'));
});

test('example import has no command or stream side effects', () => {
  const result = spawnSync(
    process.execPath,
    ['--input-type=module', '-e', 'await import(process.argv[1]);', new URL('../examples/loan-lifecycle.mjs', import.meta.url).href],
    { encoding: 'utf8', timeout: 5000 },
  );
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, '');
  assert.equal(result.stderr, '');
});

test('CLI check, format and originate simulate match the public API', () => {
  const sourcePath = example('loan-lifecycle.mori');
  const statePath = example('loan-lifecycle.state.json');
  const snapshotsPath = example('loan-lifecycle.snapshots.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const invoke = (args) => spawnSync(process.execPath, [cli, ...args], { encoding: 'utf8', timeout: 5000 });
  const checked = invoke(['check', '--profile', 'moriarty-financial-agreement-source/5', sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV5().check(sourceText)) + '\n');
  const formatted = invoke(['format', '--profile', 'moriarty-financial-agreement-source/5', sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSourceV5(sourceText));
  const simulated = invoke([
    'simulate', '--profile', 'moriarty-financial-agreement-source/5', '--action', 'originate',
    '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
  ]);
  assert.equal(simulated.status, 0, simulated.stderr);
  assert.deepEqual(JSON.parse(simulated.stdout).result, expectedStage('originate100'));
  const formattedSource = formatFinancialAgreementSourceV5(sourceText);
  const formattedResult = createFinancialAgreementSourceV5().evaluate(
    formattedSource,
    'originate',
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  );
  assert.deepEqual(formattedResult, expectedStage('originate100'));
});
