import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';

const corpusUrl = new URL('../formal/k/lifecycle-corpus.mjs', import.meta.url);
const coverageUrl = new URL('../formal/k/lifecycle-coverage.json', import.meta.url);
const sourcePath = fileURLToPath(new URL('../spec/successor/examples/loan-lifecycle.mori', import.meta.url));
const SOURCE_SHA = 'a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa';

test('lifecycle corpus consumer exists', () => {
  assert.equal(existsSync(fileURLToPath(corpusUrl)), true, 'formal/k/lifecycle-corpus.mjs must exist');
});

test('frozen source sha is unchanged', () => {
  const source = readFileSync(sourcePath);
  assert.equal(createHash('sha256').update(source).digest('hex'), SOURCE_SHA);
});

test('offline corpus elaborates whole source, chains Core, and does not execute K', async () => {
  const { runLifecycleCorpus } = await import(corpusUrl.href);
  const report = await runLifecycleCorpus({ mode: 'offline' });
  assert.equal(report.sourceSHA256, SOURCE_SHA);
  assert.equal(report.wholeSourceChecked, true);
  assert.equal(report.kExecuted, false);
  assert.equal(report.kStatus, 'not-executed');
  assert.deepEqual(report.actions, ['originate', 'accrue', 'repay', 'settle']);
  assert.equal(report.sourceResults.length, 4);
  assert.equal(report.coreResults.length, 4);
  for (const [i, name] of ['originate', 'accrue', 'repay', 'settle'].entries()) {
    assert.equal(report.sourceResults[i].status, 'FundedExpressionPrepared');
    assert.equal(report.coreResults[i].status, 'FundedExpressionPrepared');
    assert.deepEqual(report.sourceResults[i], report.coreResults[i]);
    assert.deepEqual(report.sourceResults[i], report.independent[name]);
  }
  assert.deepEqual(report.independent.originate.financialPost.work, { remaining: '413', spent: '116', closureReserve: '16' });
  assert.deepEqual(report.independent.accrue.financialPost.work, { remaining: '348', spent: '181', closureReserve: '16' });
  assert.deepEqual(report.independent.repay.financialPost.work, { remaining: '260', spent: '269', closureReserve: '16' });
  assert.deepEqual(report.independent.settle.financialPost.work, { remaining: '174', spent: '355', closureReserve: '16' });
});

test('comparator rejects debt-split and history mutations', async () => {
  const { compareLifecycleResult, runLifecycleCorpus } = await import(corpusUrl.href);
  const report = await runLifecycleCorpus({ mode: 'offline' });
  const actual = report.sourceResults[0];
  const split = structuredClone(actual);
  split.financialPost.obligations[0].principal = '70';
  split.financialPost.obligations[0].accrued = '30';
  assert.equal(compareLifecycleResult(actual, split).ok, false);
  const spent = structuredClone(actual);
  spent.financialPost.allowances[0].spent = '99';
  assert.equal(compareLifecycleResult(actual, spent).ok, false);
  const work = structuredClone(actual);
  work.financialPost.work.spent = '115';
  assert.equal(compareLifecycleResult(actual, work).ok, false);
  const cursor = structuredClone(report.sourceResults[1]);
  cursor.financialPost.obligations[0].lastAccruedPeriod = '0';
  assert.equal(compareLifecycleResult(report.sourceResults[1], cursor).ok, false);
  const effect = structuredClone(actual);
  effect.effects[0].amount = '99';
  assert.equal(compareLifecycleResult(actual, effect).ok, false);
  const history = structuredClone(report.sourceResults[2]);
  history.financialPost.usedTransferIds = [...history.financialPost.usedTransferIds].reverse();
  assert.equal(compareLifecycleResult(report.sourceResults[2], history).ok, false);
});

test('coverage inventory lists unsupported Core/4 constructors as open boundaries', () => {
  assert.equal(existsSync(fileURLToPath(coverageUrl)), true);
  const coverage = JSON.parse(readFileSync(coverageUrl, 'utf8'));
  assert.equal(coverage.allCoreEquivalenceClaimed, false);
  const names = new Set(coverage.core4Constructors.map((item) => item.name));
  for (const required of [
    'ConstructAmount', 'ScalarValue', 'Emit', 'ReadPostOutstanding', 'ReadBalance',
    'ConstructShares', 'ConstructVariant', 'Select', 'ConvertUInt',
  ]) assert.equal(names.has(required), true, required);
  const unsupported = coverage.core4Constructors.filter((item) => item.kStatus === 'unsupported-open-coverage');
  assert.ok(unsupported.length > 0);
  assert.equal(unsupported.every((item) => item.failureClass === 'execution-boundary'), true);
});

test('offline npm command reports K not executed', () => {
  const pkg = fileURLToPath(new URL('..', import.meta.url));
  const result = spawnSync('npm', ['run', 'lifecycle-k-corpus', '--prefix', pkg], { encoding: 'utf8' });
  assert.notEqual(result.status, 1, result.stderr);
  assert.match(result.stdout, /not-executed|kExecuted": false/);
});
