// Public CLI argument transport only; no compiler, oracle or native operation.
import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const cli = fileURLToPath(new URL('../src/cli.mjs', import.meta.url));
const captureHook = fileURLToPath(new URL('./support/compile-custody-cli-capture.mjs', import.meta.url));

function captureOptions(oracleResultsPath) {
  const dir = mkdtempSync(join(tmpdir(), 'mori-custody-cli-args-'));
  try {
    const manifest = { purpose: 'CLI argument transport only' };
    const manifestPath = join(dir, 'manifest.json');
    const capturePath = join(dir, 'captured-options.json');
    writeFileSync(manifestPath, JSON.stringify(manifest));
    const inputs = {
      sourcePath: 'inputs/source.mori',
      seedPath: 'inputs/seed.json',
      casesPath: 'inputs/cases.json',
      expectedReleasePath: 'inputs/release.json',
      expectedRefundPath: 'inputs/refund.json',
      residualPath: 'inputs/residual.json',
      manifest,
      outDir: join(dir, 'output artifacts'),
    };
    const args = [
      '--import', captureHook, cli, 'compile-custody',
      '--source', inputs.sourcePath,
      '--seed', inputs.seedPath,
      '--cases', inputs.casesPath,
      '--expected-release', inputs.expectedReleasePath,
      '--expected-refund', inputs.expectedRefundPath,
      '--residual', inputs.residualPath,
      '--manifest', manifestPath,
      '--out', inputs.outDir,
    ];
    if (oracleResultsPath !== undefined) args.push('--oracle-results', oracleResultsPath);
    const child = spawnSync(process.execPath, args, {
      cwd: dir,
      encoding: 'utf8',
      env: { ...process.env, MORI_CLI_ARGUMENT_CAPTURE: capturePath },
    });
    assert.equal(child.error, undefined);
    assert.equal(child.status, 1, child.stderr);
    assert.equal(child.stderr, '');
    // The operation probe deliberately returns no compile-success result.
    assert.equal(JSON.parse(child.stdout).code, 'CLI_ARGUMENT_MAPPING_CAPTURE');
    const captured = JSON.parse(readFileSync(capturePath, 'utf8'));
    const expected = oracleResultsPath === undefined
      ? inputs
      : { ...inputs, oracleResultsPath };
    assert.deepEqual(captured.options, expected);
    assert.equal(captured.oracleResultsPathType,
      oracleResultsPath === undefined ? 'undefined' : 'string');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

for (const path of ['fixtures/public/oracle-a.json', 'fixtures/public/oracle-b.json']) {
  test(`compile-custody forwards the exact oracle path ${path}`, () => {
    captureOptions(path);
  });
}

test('compile-custody forwards an explicit oracle results path with spaces', () => {
  captureOptions(join(tmpdir(), 'independent oracle directory', 'RESULTS.json'));
});

test('compile-custody preserves a relative oracle results path for its existing API', () => {
  captureOptions('independent oracle directory/RESULTS.json');
});

test('compile-custody omission leaves the existing API default selected', () => {
  captureOptions(undefined);
});
