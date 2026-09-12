import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { formatFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5-frontend.ts';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-financial-agreement-cli-v5-'));
after(() => rmSync(directory, { recursive: true, force: true }));
let serial = 0;
const file = (content) => {
  const name = join(directory, `input ${serial++}`);
  writeFileSync(name, content);
  return name;
};
const invoke = (args) => {
  const result = spawnSync(process.execPath, [cli, ...args], { encoding: 'utf8', timeout: 5000 });
  assert.equal(result.error, undefined);
  assert.equal(result.signal, null);
  return result;
};
const agreement = 'moriarty-financial-agreement-source/5';
const agreementV4 = 'moriarty-financial-agreement-source/4';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const v5Args = (sourcePath, action, snapshotsPath, statePath) => [
  'simulate', '--profile', agreement, '--action', action, '--snapshots', snapshotsPath,
  '--repayment-state', statePath, sourcePath,
];
const failure = (args, expected, exit = 1) => {
  const result = invoke(args);
  assert.equal(result.status, exit, result.stderr);
  assert.equal(result.stdout, '');
  assert.equal(result.stderr, JSON.stringify(expected) + '\n');
};
const cliFailure = (args, code, input, exit = 2) => failure(args, { status: 'CliRejected', code, input }, exit);

test('agreement /5 check and format use no schema or action and match the API', () => {
  const sourcePath = example('financial-lifecycle-payment.mori');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const checked = invoke(['check', '--profile', agreement, sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV5().check(sourceText)) + '\n');

  const formatted = invoke(['format', '--profile', agreement, sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSourceV5(sourceText));
});

test('agreement /5 simulate requires --action and equals the API', () => {
  const sourcePath = example('financial-lifecycle-payment.mori');
  const statePath = example('financial-lifecycle-payment.state.json');
  const snapshotsPath = example('financial-lifecycle-payment.snapshots.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const expected = createFinancialAgreementSourceV5().evaluate(
    sourceText,
    'originate',
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  );
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  const result = invoke(v5Args(sourcePath, 'originate', snapshotsPath, statePath));
  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(JSON.parse(result.stdout).result, expected);
});

test('/5 rejects --schema; /4 stays on its form', () => {
  const sourcePath = example('financial-lifecycle-payment.mori');
  const v4Source = example('financial-postconditions-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  cliFailure(['check', '--profile', agreement, '--schema', schemaPath, sourcePath], 'CLI_USAGE', 'arguments');
  const v4 = invoke(['check', '--profile', agreementV4, v4Source]);
  assert.equal(v4.status, 0, v4.stderr);
  assert.equal(JSON.parse(v4.stdout).sourceProfile, agreementV4);
  assert.equal(typeof createFinancialAgreementSourceV4().check, 'function');
});
