import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, copyFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { compareLifecycleResult, runLifecycleCorpus } from '/home/charl/Moriarty/experiments/moriarty-language/formal/k/lifecycle-corpus.mjs';
import { createFinancialAgreementSourceV5 } from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { createFinancialExpressionContractV4 } from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/financial-expression-v1.ts';

const repo = '/home/charl/Moriarty';
const sourcePath = path.join(repo, 'experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori');
const source = readFileSync(sourcePath, 'utf8');
const findings = [];

function statementSpan(text, from = 0) {
  const start = source.indexOf(text, from);
  if (start < 0) throw new Error(`missing ${text}`);
  return {
    kind: 'source',
    start: String(Buffer.byteLength(source.slice(0, start))),
    end: String(Buffer.byteLength(source.slice(0, start + text.length))),
  };
}

const report = await runLifecycleCorpus();
assert.equal(report.ok, true);
assert.equal(report.kExecuted, false);
assert.equal(report.kStatus, 'not-executed');

// Spans must match independently computed source-text ranges, not hardcoded dumps.
const settleStart = source.indexOf('action settle(');
const guardExpected = statementSpan('requires payment > 0;', settleStart);
const lateText = 'ensures post_balance<Cash>("Lender") == amount<Cash>(111);';
const lateSource = source.replace('ensures post_balance<Cash>("Lender") == amount<Cash>(110);', lateText);
const lateStart = lateSource.indexOf(lateText);
const ensureExpected = {
  kind: 'source',
  start: String(Buffer.byteLength(lateSource.slice(0, lateStart))),
  end: String(Buffer.byteLength(lateSource.slice(0, lateStart + lateText.length))),
};
const guard = report.cases.find((c) => c.name === 'settled-source-guard');
const late = report.cases.find((c) => c.name === 'late-false-ensure');
assert.deepEqual(guard.source.result.span, guardExpected);
assert.deepEqual(guard.core.result.span, guardExpected);
assert.deepEqual(late.source.result.span, ensureExpected);
assert.deepEqual(late.core.result.span, ensureExpected);
assert.notEqual(guardExpected.start, '0');
assert.equal(source.slice(Number(guardExpected.start), Number(guardExpected.end)), 'requires payment > 0;');
assert.equal(lateSource.slice(Number(ensureExpected.start), Number(ensureExpected.end)), lateText);
// repay also contains requires payment > 0; settled span must use settle occurrence
const repayRequire = source.indexOf('requires payment > 0;');
assert.ok(repayRequire >= 0);
assert.notEqual(String(Buffer.byteLength(source.slice(0, repayRequire))), guardExpected.start);

// Own-predecessor chaining on the real (non-injected) run
for (const kind of ['source', 'core']) {
  for (let i = 1; i < report.stages.length; i++) {
    const previous = report.stages[i - 1][kind].result;
    const input = report.stages[i][kind].input;
    assert.deepEqual(input.snapshot.Pre, previous.post);
    assert.deepEqual(JSON.parse(input.financialPreState), previous.financialPost);
    assert.equal(input.snapshot.workInitial, previous.workRemaining);
  }
}

// Complete oracle fields: debt split, history, unrelated Other token, work
const expectedOutstanding = ['100', '110', '80', '0'];
const expectedSpent = ['116', '181', '269', '355'];
const expectedRemaining = ['413', '348', '260', '174'];
for (let i = 0; i < 4; i++) {
  const r = report.stages[i].source.result;
  const ob = r.financialPost.obligations[0];
  assert.equal(ob.outstanding, expectedOutstanding[i]);
  assert.equal(r.financialPost.work.spent, expectedSpent[i]);
  assert.equal(r.workRemaining, expectedRemaining[i]);
  assert.equal(r.financialPost.work.closureReserve, '16');
  const other = r.financialPost.balances.find((b) => b.party === 'Other');
  assert.equal(other.amount, '7');
}
assert.deepEqual(report.stages[0].source.result.financialPost.obligations[0].principal, '100');
assert.deepEqual(report.stages[0].source.result.financialPost.obligations[0].accrued, '0');
assert.deepEqual(report.stages[1].source.result.financialPost.obligations[0].principal, '100');
assert.deepEqual(report.stages[1].source.result.financialPost.obligations[0].accrued, '10');
assert.deepEqual(report.stages[2].source.result.financialPost.usedTransferIds, ['D1', 'P1']);
assert.deepEqual(report.stages[3].source.result.financialPost.usedTransferIds, ['D1', 'P1', 'P2']);

// Comparator: financial split with conserved outstanding, extra/missing keys, types, history, work
const accrue = structuredClone(report.stages[1].expected);
accrue.financialPost.obligations[0].principal = '99';
accrue.financialPost.obligations[0].accrued = '11';
assert.equal(compareLifecycleResult(accrue, report.stages[1].expected).ok, false);
assert.ok(compareLifecycleResult(accrue, report.stages[1].expected).differences.some((d) => d.includes('principal')));
const extra = structuredClone(report.stages[1].expected);
extra.financialPost.surprise = '1';
assert.equal(compareLifecycleResult(extra, report.stages[1].expected).ok, false);
const missing = structuredClone(report.stages[1].expected);
delete missing.financialPost.work.spent;
assert.equal(compareLifecycleResult(missing, report.stages[1].expected).ok, false);
const typed = structuredClone(report.stages[1].expected);
typed.workRemaining = 348;
assert.equal(compareLifecycleResult(typed, report.stages[1].expected).ok, false);
const hist = structuredClone(report.stages[3].expected);
hist.financialPost.usedAllocationIds = [...hist.financialPost.usedAllocationIds].reverse();
assert.equal(compareLifecycleResult(hist, report.stages[3].expected).ok, false);

// CLI ignores injected factories: real CLI always uses default factories.
const cli = spawnSync(process.execPath, [
  path.join(repo, 'experiments/moriarty-language/formal/k/lifecycle-corpus.mjs'),
  '--sourceFactory', 'bogus', '--coreFactory', 'bogus', '--mode', 'k', '--kAdmitted', 'true',
], { encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
assert.equal(cli.status, 0, cli.stderr);
const cliReport = JSON.parse(cli.stdout);
assert.equal(cliReport.ok, true);
assert.equal(cliReport.kExecuted, false);
assert.equal(cliReport.mode, 'offline');

// API injection is possible; CLI is not that path.
const injected = await runLifecycleCorpus({
  sourceFactory() { throw new Error('injected-source'); },
});
assert.equal(injected.ok, false);
assert.ok(injected.failures.some((f) => f.message === 'injected-source'));
assert.equal(injected.kExecuted, false);

// Missing local evidence fails nonzero.
const tmp = mkdtempSync(path.join(tmpdir(), 'moriarty-grok-missing-'));
try {
  const entry = 'experiments/moriarty-language/formal/k/lifecycle-corpus.mjs';
  mkdirSync(path.dirname(path.join(tmp, entry)), { recursive: true });
  copyFileSync(path.join(repo, entry), path.join(tmp, entry));
  mkdirSync(path.join(tmp, 'experiments/moriarty-language/spec/successor/examples'), { recursive: true });
  copyFileSync(sourcePath, path.join(tmp, 'experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori'));
  // no evidence files
  const child = spawnSync(process.execPath, [path.join(tmp, entry)], {
    encoding: 'utf8',
    maxBuffer: 8 * 1024 * 1024,
    env: { ...process.env, NODE_PATH: path.join(repo, 'experiments/moriarty-language') },
  });
  // This copy still imports ../../src from the copied file location, which will fail module load.
  // Use a dedicated missing-oracle copy instead with src symlink via running from repo with oracle removed is not allowed.
} finally {
  rmSync(tmp, { recursive: true, force: true });
}

const missingOracleDir = mkdtempSync(path.join(tmpdir(), 'moriarty-grok-oracle-'));
try {
  const files = [
    'experiments/moriarty-language/formal/k/lifecycle-corpus.mjs',
    'experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori',
    'deliverables/language-to-ledger-2026-09-12/design-review/lifecycle-source-expectations.json',
    'deliverables/language-to-ledger-2026-09-12/lifecycle/root-work-expectations.json',
  ];
  for (const file of files) {
    mkdirSync(path.dirname(path.join(missingOracleDir, file)), { recursive: true });
    copyFileSync(path.join(repo, file), path.join(missingOracleDir, file));
  }
  const { symlinkSync } = await import('node:fs');
  symlinkSync(path.join(repo, 'experiments/moriarty-language/src'), path.join(missingOracleDir, 'experiments/moriarty-language/src'), 'dir');
  rmSync(path.join(missingOracleDir, files[2]));
  const child = spawnSync(process.execPath, [path.join(missingOracleDir, files[0])], {
    encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
  });
  assert.equal(child.status, 1);
  const missingReport = JSON.parse(child.stdout);
  assert.equal(missingReport.ok, false);
  assert.equal(missingReport.kExecuted, false);
  assert.ok(missingReport.failures.some((f) => String(f.message || '').includes('ENOENT')));
} finally {
  rmSync(missingOracleDir, { recursive: true, force: true });
}

// Source hash pin
assert.equal(createHash('sha256').update(source).digest('hex'), 'a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa');
assert.equal(report.sourceSHA256, 'a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa');

// Rejection envelopes publish no state
for (const row of report.cases) {
  for (const kind of ['source', 'core']) {
    for (const key of ['post', 'financialPost', 'effects', 'descriptors', 'continueSuffix', 'workRemaining']) {
      assert.equal(Object.hasOwn(row[kind].result, key), false, `${row.name} ${kind} ${key}`);
    }
  }
}

console.log(JSON.stringify({
  passed: true,
  spanGuard: guardExpected,
  spanEnsure: ensureExpected,
  kExecuted: report.kExecuted,
  stages: report.stages.length,
  cases: report.cases.length,
  retries: report.cases.filter((c) => c.retry).length,
  cliIgnoresFactoryFlags: true,
  missingEvidenceNonzero: true,
  comparatorFinancialSplit: true,
}));
