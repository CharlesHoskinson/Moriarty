// File: experiments/moriarty-eutxo/tests/custody-publication-isolation.test.mjs
// Also integrable into experiments/moriarty-eutxo/tests/custody.test.mjs
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { compileCustody } from '../src/compile-custody.mjs';
import { ROOT } from '../src/compile-public.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const DESIGNER = join(TASK, 'designers/02-escrow-time');
const SOURCE = join(DESIGNER, 'escrow-custody-v5.mori');
const SEED = join(DESIGNER, 'financial-seed-custody-v5.json');
const CASES = join(DESIGNER, 'cases.json');
const E13 = join(DESIGNER, 'E13-release-real-custody.expected.json');
const E11 = join(DESIGNER, 'E11-refund-real-custody.expected.json');
const RESIDUAL = join(DESIGNER, 'financial-residual-custody-v5.json');

function unitManifest() {
  return JSON.parse(readFileSync(join(ROOT, 'examples/custody-unit.manifest.json'), 'utf8'));
}

test('failed aiken check with succeeding aiken build does not publish blueprint, advertise blueprintPath, or apply parameters', () => {
  const outDir = mkdtempSync(join(tmpdir(), 'moriarty-q1-source1-check-exit-'));
  try {
    mkdirSync(outDir, { recursive: true });
    writeFileSync(join(outDir, 'plutus.json'), 'STALE\n');
    const calls = [];
    const result = compileCustody({
      sourcePath: SOURCE,
      seedPath: SEED,
      casesPath: CASES,
      expectedReleasePath: E13,
      expectedRefundPath: E11,
      residualPath: RESIDUAL,
      manifest: unitManifest(),
      outDir,
      aikenRun(args, cwd) {
        calls.push(args);
        if (args[0] === 'check') {
          return { status: 7, stdout: '', stderr: 'forced-check-exit-7' };
        }
        if (args[0] === 'build') {
          writeFileSync(
            join(cwd, 'plutus.json'),
            JSON.stringify({ preamble: { title: 'unverified-build' }, validators: [] }) + '\n',
          );
          return { status: 0, stdout: 'build-ok', stderr: '' };
        }
        return { status: 1, stdout: '', stderr: 'unexpected-command' };
      },
    });

    assert.equal(calls.length, 2);
    assert.equal(result.ok, false);
    assert.equal(result.code, 'AIKEN_CHECK');
    assert.equal(result.check.status, 7);
    assert.equal(result.build.status, 0);
    assert.equal(result.blueprintPath, null);
    assert.equal(result.applied, null);
    assert.equal(existsSync(join(outDir, 'plutus.json')), false);

    const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
    assert.equal(admission.ok, false);
    assert.equal(admission.code, 'AIKEN_CHECK');
    assert.equal(admission.checkStatus, 7);
    assert.equal(admission.buildStatus, 0);
    assert.equal(admission.applied, undefined);
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
});

test('missing required named tests on status-0 aiken check with succeeding build does not publish blueprint or apply parameters', () => {
  const outDir = mkdtempSync(join(tmpdir(), 'moriarty-q1-source1-missing-tests-'));
  try {
    const result = compileCustody({
      sourcePath: SOURCE,
      seedPath: SEED,
      casesPath: CASES,
      expectedReleasePath: E13,
      expectedRefundPath: E11,
      residualPath: RESIDUAL,
      manifest: unitManifest(),
      outDir,
      aikenRun(args, cwd) {
        if (args[0] === 'check') {
          return { status: 0, stdout: '{"tests":[]}', stderr: '' };
        }
        if (args[0] === 'build') {
          writeFileSync(
            join(cwd, 'plutus.json'),
            JSON.stringify({ preamble: { title: 'unverified-build' }, validators: [] }) + '\n',
          );
          return { status: 0, stdout: 'build-ok', stderr: '' };
        }
        return { status: 1, stdout: '', stderr: 'unexpected-command' };
      },
    });

    assert.equal(result.ok, false);
    assert.equal(result.code, 'AIKEN_CHECK');
    assert.equal(result.checkStatus, 1);
    assert.equal(result.buildStatus, 0);
    assert.equal(result.blueprintPath, null);
    assert.equal(result.applied, null);
    assert.ok(Array.isArray(result.missingTests) && result.missingTests.length > 0);
    assert.equal(existsSync(join(outDir, 'plutus.json')), false);

    const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
    assert.equal(admission.ok, false);
    assert.equal(admission.code, 'AIKEN_CHECK');
    assert.equal(admission.checkStatus, 1);
    assert.equal(admission.buildStatus, 0);
    assert.equal(admission.applied, undefined);
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
});
