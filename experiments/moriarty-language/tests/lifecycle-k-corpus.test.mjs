import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, copyFileSync, symlinkSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { compareLifecycleResult, runLifecycleCorpus } from '../formal/k/lifecycle-corpus.mjs';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { createFinancialExpressionContractV4 } from '../src/successor/financial-expression-v1.ts';

const packageRoot = fileURLToPath(new URL('..', import.meta.url));
const repoRoot = path.resolve(packageRoot, '../..');
const evidence = 'deliverables/language-to-ledger-2026-09-12';
const entry = 'experiments/moriarty-language/formal/k/lifecycle-corpus.mjs';
const report = await runLifecycleCorpus();

test('offline corpus agrees on complete four-stage independent states/effects/work', () => {
  assert.equal(report.ok, true, JSON.stringify(report.failures));
  assert.equal(report.wholeSourceChecked, true);
  assert.equal(report.kExecuted, false);
  assert.equal(report.kStatus, 'not-executed');
  assert.equal(report.stages.length, 4);
  assert.deepEqual(report.stages.map(x => x.source.result.workRemaining), ['413', '348', '260', '174']);
  assert.deepEqual(report.stages.map(x => x.core.result.financialPost.work.spent), ['116', '181', '269', '355']);
  for (const stage of report.stages) {
    assert.deepEqual(stage.source.result, stage.expected);
    assert.deepEqual(stage.core.result, stage.expected);
    assert.equal(stage.expected.financialPost.work.closureReserve, '16');
    assert.equal(BigInt(stage.expected.financialPost.work.remaining) + BigInt(stage.expected.financialPost.work.spent), 529n);
  }
});

test('nine actual negative cases compare complete diagnostic envelopes and valid continuations', () => {
  assert.deepEqual(report.cases.map(x => [x.name, x.expected.code]), [
    ['duplicate-accrual', 'DUPLICATE'], ['repeated-period', 'PERIOD_SEQUENCE'],
    ['early-period', 'PERIOD_NOT_ELIGIBLE'], ['incurred-cap-after-repay', 'LIABILITY_CAP_EXCEEDED'],
    ['insufficient-funding', 'INSUFFICIENT_BALANCE'], ['settled-source-guard', 'GUARD_FAILED'],
    ['late-false-ensure', 'ENSURES_FAILED'], ['malformed-unrelated-allowance', 'INVARIANT'],
    ['work-mismatch', 'WORK_MISMATCH'],
  ]);
  for (const row of report.cases) {
    for (const kind of ['source', 'core']) {
      assert.deepEqual(row[kind].result, row.expected);
      for (const key of ['post', 'financialPost', 'effects', 'descriptors', 'continueSuffix', 'workRemaining']) {
        assert.equal(Object.hasOwn(row[kind].result, key), false);
      }
      if (row.retry) assert.equal(row[kind].retry.result.status, 'FundedExpressionPrepared');
    }
  }
  const early = report.cases.find(x => x.name === 'early-period');
  assert.equal(early.source.input.snapshot.Args.observedTime, '1059');
  assert.equal(early.source.retry.input.snapshot.Args.observedTime, '1060');
  assert.equal(early.source.input.financialPreState, early.source.retry.input.financialPreState);
  const funding = report.cases.find(x => x.name === 'insufficient-funding');
  assert.deepEqual(funding.source.retry.input.snapshot.Args, { transferId: 'PF', allocationId: 'RF', nominal: '30' });
  assert.equal(funding.source.input.financialPreState, funding.source.retry.input.financialPreState);
  assert.equal(report.cases.find(x => x.name === 'late-false-ensure').source.result.workUsed, '86');
});

test('comparator detects debt split, allowance, cursor, effect, work and ordered history mutations', () => {
  const expected = report.stages[1].expected;
  const mutations = [
    x => { x.financialPost.obligations[0].principal = '99'; x.financialPost.obligations[0].accrued = '11'; },
    x => { x.financialPost.allowances[2].remaining = '2'; },
    x => { x.financialPost.obligations[0].nextAccrualAt = '1121'; },
    x => { x.effects[0].kind = 'Transfer'; },
    x => { x.financialPost.work.spent = '180'; },
    x => { x.workRemaining = 348; },
    x => { delete x.financialPost.balances[2]; },
    x => { x.extra = undefined; },
  ];
  for (const mutate of mutations) {
    const actual = structuredClone(expected);
    mutate(actual);
    assert.equal(compareLifecycleResult(actual, expected).ok, false);
  }
  const history = structuredClone(report.stages[3].expected);
  history.financialPost.usedTransferIds.reverse();
  assert.equal(compareLifecycleResult(history, report.stages[3].expected).ok, false);
  assert.equal(compareLifecycleResult({}, { missing: undefined }).ok, false);
  assert.equal(compareLifecycleResult({ extra: undefined }, {}).ok, false);
  assert.equal(compareLifecycleResult([undefined], Array(1)).ok, false);
  assert.equal(compareLifecycleResult(Object.assign([], { extra: undefined }), []).ok, false);
  assert.equal(compareLifecycleResult({ x: null }, { x: {} }).ok, false);
  assert.equal(compareLifecycleResult({ a: 1, b: 2 }, { b: 2, a: 1 }).ok, true);
});

test('Source and Core each receive their own actual predecessor, including divergence', async () => {
  let coreCalls = 0;
  const divergent = await runLifecycleCorpus({
    coreFactory(schema, financial) {
      const core = createFinancialExpressionContractV4(schema, financial);
      return { evaluate(request) {
        const result = core.evaluate(request);
        if (coreCalls++ === 0) {
          result.post.phase = '8';
          result.financialPost.balances[2].amount = '8';
        }
        return result;
      } };
    },
  });
  assert.equal(divergent.ok, false);
  assert.equal(divergent.stages.length, 4);
  for (const kind of ['source', 'core']) {
    for (let i = 1; i < divergent.stages.length; i++) {
      const previous = divergent.stages[i - 1][kind].result;
      const input = divergent.stages[i][kind].input;
      assert.deepEqual(input.snapshot.Pre, previous.post);
      assert.deepEqual(JSON.parse(input.financialPreState), previous.financialPost);
      assert.equal(input.snapshot.workInitial, previous.workRemaining);
    }
  }
  assert.equal(divergent.stages[1].core.input.snapshot.Pre.phase, '8');
  assert.equal(divergent.stages[1].source.input.snapshot.Pre.phase, '1');
  assert.equal(divergent.stages[3].core.result.financialPost.balances[2].amount, '8');
  assert.equal(divergent.stages[3].source.result.financialPost.balances[2].amount, '7');
});

test('check failure, elaboration failure and incomplete evaluation cannot pass', async () => {
  for (const method of ['check', 'elaborate', 'evaluate']) {
    const failed = await runLifecycleCorpus({ sourceFactory() {
      return { ...createFinancialAgreementSourceV5(), [method]() { return { status: 'Rejected', code: 'TEST_FAILURE' }; } };
    } });
    assert.equal(failed.ok, false, method);
    assert.ok(failed.failures.length > 0);
    assert.equal(failed.kExecuted, false);
    assert.equal(failed.kStatus, 'not-executed');
  }
});

test('caller admission flags cannot turn offline comparison into K execution', async () => {
  const attempted = await runLifecycleCorpus({ mode: 'k', kAdmitted: true });
  assert.equal(attempted.ok, true);
  assert.equal(attempted.mode, 'offline');
  assert.equal(attempted.kExecuted, false);
  assert.equal(attempted.kStatus, 'not-executed');
});

test('real CLI succeeds and fails nonzero for changed source, oracle mismatch or missing local evidence', () => {
  const run = root => {
    const child = spawnSync(process.execPath, [path.join(root, entry)], { encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
    assert.equal(child.error, undefined);
    assert.equal(child.signal, null);
    const result = JSON.parse(child.stdout);
    assert.equal(result.kExecuted, false);
    assert.equal(result.kStatus, 'not-executed');
    return { child, result };
  };
  const valid = run(repoRoot);
  assert.equal(valid.child.status, 0, valid.child.stderr);
  assert.equal(valid.result.ok, true);
  const temporary = mkdtempSync(path.join(tmpdir(), 'moriarty-lifecycle-corpus-'));
  try {
    const files = [entry, 'experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori',
      `${evidence}/design-review/lifecycle-source-expectations.json`, `${evidence}/lifecycle/root-work-expectations.json`];
    for (const file of files) {
      mkdirSync(path.dirname(path.join(temporary, file)), { recursive: true });
      copyFileSync(path.join(repoRoot, file), path.join(temporary, file));
    }
    symlinkSync(path.join(packageRoot, 'src'), path.join(temporary, 'experiments/moriarty-language/src'), 'dir');
    const sourcePath = path.join(temporary, files[1]);
    const original = readFileSync(sourcePath, 'utf8');
    writeFileSync(sourcePath, `${original}\n`);
    let bad = run(temporary);
    assert.equal(bad.child.status, 1);
    assert.equal(bad.result.wholeSourceChecked, false);
    writeFileSync(sourcePath, original);
    const oraclePath = path.join(temporary, files[2]);
    const oracle = JSON.parse(readFileSync(oraclePath, 'utf8'));
    oracle.stages[0].expectedResultTemplate.post.phase = '999';
    writeFileSync(oraclePath, JSON.stringify(oracle));
    bad = run(temporary);
    assert.equal(bad.child.status, 1);
    assert.equal(bad.result.wholeSourceChecked, true);
    assert.ok(bad.result.failures.some(x => x.label === 'originate source independent'));
    rmSync(oraclePath);
    bad = run(temporary);
    assert.equal(bad.child.status, 1);
    assert.ok(bad.result.failures.some(x => x.message?.includes('ENOENT')));
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
});
