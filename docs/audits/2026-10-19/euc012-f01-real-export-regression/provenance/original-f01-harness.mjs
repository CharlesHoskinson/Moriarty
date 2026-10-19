/**
 * EUC-012 F01: required Aiken test identity in compileCustody.
 * An absent required test must not be satisfied by another generated title
 * that only ends with the same text. No Aiken process runs: check and build
 * are injected through the aikenRun argument of compileCustody.
 * Runtime prerequisites are the designer, oracle, and pinned stdlib inputs
 * that compileCustody reads before it calls aikenRun. Nothing is read here at
 * module top level.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { compileCustody } from '../src/compile-custody.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const DESIGNER = join(TASK, 'designers/02-escrow-time');
const ORACLE = join(TASK, 't1-independent-oracles-19/RESULTS.json');
const OMITTED = 'release_missing_lender_row';
const SHADOW = 'oracle_release_missing_lender_row';
const SUCCESS_NAMES = ['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json'];

function unitManifest() {
  return {
    schema: 'moriarty-t1-custody-manifest/1',
    nativePins: false,
    unitFixture: true,
    production: false,
    domainNetwork: 'yaci-isolated-t1',
    domainDeployment: 't1-custody-revision03',
    threadPolicyId: 'e5'.repeat(28),
    cashPolicyId: 'b2'.repeat(28),
    lenderVkh: 'c3'.repeat(28),
    borrowerVkh: 'd4'.repeat(28),
    scriptHash: 'e5'.repeat(28),
    threadAssetNameHex: '48656164',
    cashAssetNameHex: '43617368',
    quantum: '1',
  };
}

// Fake check: every generated test title except `omit` is reported as passing,
// optionally module-qualified. Fake build: a parseable blueprint without the
// four purpose titles, refused by applyUnitParameters before any spawn.
function fakeAiken({ omit = null, qualify = false } = {}) {
  const seen = { check: 0, build: 0, generated: [] };
  const aikenRun = (args, cwd) => {
    if (args[0] === 'check') {
      seen.check += 1;
      const validator = readFileSync(join(cwd, 'validators/financial_head.ak'), 'utf8');
      seen.generated = [...validator.matchAll(/^test ([A-Za-z0-9_]+)\(/gm)].map((match) => match[1]);
      const tests = seen.generated
        .filter((title) => title !== omit)
        .map((title) => ({ title: qualify ? `financial_head.${title}` : title, status: 'success', kind: 'unit' }));
      return { status: 0, stdout: JSON.stringify({ summary: { fake: true }, modules: [{ name: 'financial_head', test: tests }] }), stderr: '' };
    }
    seen.build += 1;
    writeFileSync(join(cwd, 'plutus.json'), `${JSON.stringify({ validators: [] })}\n`);
    return { status: 0, stdout: '', stderr: '' };
  };
  return { seen, aikenRun };
}

function compileWith(t, runner) {
  const outDir = mkdtempSync(join(tmpdir(), 'cu-f01-'));
  t.after(() => rmSync(outDir, { recursive: true, force: true }));
  for (const name of SUCCESS_NAMES) writeFileSync(join(outDir, name), 'STALE\n');
  const result = compileCustody({
    sourcePath: join(DESIGNER, 'escrow-custody-v5.mori'),
    seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
    casesPath: join(DESIGNER, 'cases.json'),
    expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
    expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
    residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
    manifest: unitManifest(),
    outDir,
    oracleResultsPath: ORACLE,
    aikenRun: runner.aikenRun,
  });
  return { outDir, result };
}

// The required-test gate was reached: an earlier refusal never calls aikenRun.
function assertGateReached(runner, result) {
  assert.equal(runner.seen.check, 1, `check not reached: ${result?.code} ${result?.detail}`);
  assert.equal(runner.seen.build, 1);
  assert.ok(runner.seen.generated.includes(OMITTED), `${OMITTED} was not generated`);
  assert.ok(runner.seen.generated.includes(SHADOW), `${SHADOW} was not generated`);
}

function assertNoSuccessArtifact(outDir, result) {
  assert.notEqual(result?.ok, true);
  assert.equal(result?.blueprintPath ?? null, null);
  for (const name of SUCCESS_NAMES) {
    assert.equal(existsSync(join(outDir, name)), false, `${name} survived a failed compile`);
  }
}

test('EUC-012 F01 an absent required test is not satisfied by a longer title with the same suffix', (t) => {
  const runner = fakeAiken({ omit: OMITTED });
  const { outDir, result } = compileWith(t, runner);
  assertGateReached(runner, result);
  const reported = result.check.tests.map((item) => item.title);
  assert.equal(reported.includes(OMITTED), false);
  assert.equal(reported.includes(SHADOW), true);
  assert.equal(result.ok, false);
  assert.equal(result.code, 'AIKEN_CHECK');
  assert.deepEqual(result.missingTests, [OMITTED]);
  assert.equal(result.detail, OMITTED);
  assert.equal(result.checkStatus, 1);
  assert.equal(result.buildStatus, 0);
  assert.equal(result.applied, null);
  assertNoSuccessArtifact(outDir, result);
  const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
  assert.equal(admission.ok, false);
  assert.equal(admission.code, 'AIKEN_CHECK');
  assert.deepEqual(admission.missingTests, [OMITTED]);
  assert.equal(admission.checkStatus, 1);
});

test('EUC-012 F01 module-qualified titles satisfy required tests and the same omission is still caught', (t) => {
  const all = fakeAiken({ qualify: true });
  const present = compileWith(t, all);
  assertGateReached(all, present.result);
  assert.deepEqual(present.result.missingTests, []);
  assert.equal(present.result.checkStatus, 0);
  assert.equal(present.result.applied?.ok, false);
  assert.equal(present.result.applied?.detail, 'blueprint titles');
  assertNoSuccessArtifact(present.outDir, present.result);

  const omitted = fakeAiken({ omit: OMITTED, qualify: true });
  const absent = compileWith(t, omitted);
  assertGateReached(omitted, absent.result);
  assert.equal(absent.result.code, 'AIKEN_CHECK');
  assert.deepEqual(absent.result.missingTests, [OMITTED]);
  assert.equal(absent.result.checkStatus, 1);
  assert.equal(absent.result.applied, null);
  assertNoSuccessArtifact(absent.outDir, absent.result);
});
