import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import { formatFinancialAgreementSource } from '../src/successor/financial-agreement-source-frontend.ts';
import { createFundedFinancialExpressionSourceV1 } from '../src/successor/funded-expression-source-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-financial-agreement-cli-'));
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
const agreement = 'moriarty-financial-agreement-source/1';
const financial = 'moriarty-financial-expression-source/1';
const original = 'moriarty-expression-source/1';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const agreementArgs = (sourcePath, snapshotsPath, statePath) => [
  'simulate', '--profile', agreement, '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
];
const fundedArgs = (schemaPath, snapshotsPath, statePath, sourcePath) => [
  'simulate', '--profile', financial, '--schema', schemaPath,
  '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
];
const simulateArgs = (profile, schemaPath, snapshotsPath, sourcePath) => [
  'simulate', '--profile', profile, '--schema', schemaPath, '--snapshots', snapshotsPath, sourcePath,
];
const failure = (args, expected, exit = 1) => {
  const result = invoke(args);
  assert.equal(result.status, exit, result.stderr);
  assert.equal(result.stdout, '');
  assert.equal(result.stderr, JSON.stringify(expected) + '\n');
};
const cliFailure = (args, code, input, exit = 2) => failure(args, { status: 'CliRejected', code, input }, exit);

test('agreement check and format use no schema and match the API', () => {
  const sourcePath = example('source-defined-payment.mori');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const checked = invoke(['check', '--profile', agreement, sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV1().check(sourceText)) + '\n');

  const formatted = invoke(['format', '--profile', agreement, sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSource(sourceText));
  assert.equal(invoke(['format', '--profile', agreement, file(formatted.stdout)]).stdout, formatted.stdout);
});

test('agreement simulate CLI equals the API and the schema-based funded example', () => {
  const sourcePath = example('source-defined-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const schemaText = readFileSync(schemaPath, 'utf8');
  const snapshotsText = readFileSync(snapshotsPath, 'utf8');
  const stateText = readFileSync(statePath, 'utf8');
  const expected = createFinancialAgreementSourceV1().evaluate(sourceText, snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  const baseline = createFundedFinancialExpressionSourceV1(schemaText)
    .evaluate(readFileSync(example('expression-funded-payment.mori'), 'utf8'), snapshotsText, stateText);
  assert.deepEqual(expected, baseline);

  const result = invoke(agreementArgs(sourcePath, snapshotsPath, statePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.deepEqual(JSON.parse(result.stdout), {
    judgmentResult: 'SourceSimulated',
    sourceProfile: agreement,
    pre: { due: '100', paid: '0' },
    financialPre: JSON.parse(stateText),
    initialWork: '100',
    result: expected,
  });
});

test('agreement CLI writes semantic rejection to stderr with empty stdout', () => {
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourceText = readFileSync(example('source-defined-payment.mori'), 'utf8');
  const failing = sourceText.replace('requires payment > 0;', 'requires payment == 1;');
  const expected = createFinancialAgreementSourceV1().evaluate(
    failing,
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  );
  assert.equal(expected.status, 'Rejected');
  assert.equal('post' in expected, false);
  failure(agreementArgs(file(failing), snapshotsPath, statePath), expected);
});

test('new profile rejects --schema and old profiles reject the schema-free forms', () => {
  const sourcePath = example('source-defined-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  cliFailure(['check', '--profile', agreement, '--schema', schemaPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', agreement, '--schema', schemaPath,
    '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', agreement, '--snapshots', snapshotsPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', financial, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', original, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(agreementArgs(sourcePath, snapshotsPath, statePath).map((part) => part === agreement ? financial : part),
    'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', 'unknown', '--snapshots', 'missing', '--repayment-state', 'missing', 'missing'],
    'CLI_PROFILE', 'arguments');
});

test('agreement simulate retains snapshot and repayment-state transport limits', () => {
  const sourcePath = example('source-defined-payment.mori');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const expected = createFinancialAgreementSourceV1().evaluate(
    readFileSync(sourcePath, 'utf8'),
    readFileSync(snapshotsPath, 'utf8'),
    '{',
  );
  assert.equal(expected.status, 'Rejected');
  failure(agreementArgs(sourcePath, snapshotsPath, file('{')), expected);
  cliFailure(agreementArgs(sourcePath, snapshotsPath, file(' '.repeat(65537))), 'INPUT_BOUND', 'repayment-state', 1);
  cliFailure(agreementArgs(sourcePath, snapshotsPath, file(new Uint8Array([0xff]))), 'INVALID_UTF8', 'repayment-state', 1);
  cliFailure(agreementArgs(sourcePath, snapshotsPath, directory), 'CLI_IO', 'repayment-state');
  cliFailure(agreementArgs(file(' '.repeat(65537)), snapshotsPath, file('{}')), 'SOURCE_BOUND', 'source', 1);
});

test('old-profile simulate, check and format stay unchanged', () => {
  const financialSource = example('financial-vault-quote.mori');
  const financialSchema = example('financial-vault-quote.schema.json');
  const financialSnapshots = example('financial-vault-quote.snapshots.json');
  const financialResult = invoke(simulateArgs(financial, financialSchema, financialSnapshots, financialSource));
  assert.equal(financialResult.status, 0, financialResult.stderr);
  const financialApi = createFinancialExpressionSourceV1(readFileSync(financialSchema, 'utf8'))
    .evaluate(readFileSync(financialSource, 'utf8'), readFileSync(financialSnapshots, 'utf8'));
  assert.deepEqual(JSON.parse(financialResult.stdout), {
    judgmentResult: 'SourceSimulated', sourceProfile: financial, pre: { last: '0' }, initialWork: '1000',
    result: financialApi,
  });

  const originalSource = example('expression-counter.mori');
  const originalSchema = example('expression-counter.schema.json');
  const originalSnapshots = example('expression-counter.snapshots.json');
  const originalResult = invoke(simulateArgs(original, originalSchema, originalSnapshots, originalSource));
  assert.equal(originalResult.status, 0, originalResult.stderr);
  const originalApi = createExpressionSourceV1(readFileSync(originalSchema, 'utf8'))
    .evaluate(readFileSync(originalSource, 'utf8'), readFileSync(originalSnapshots, 'utf8'));
  assert.deepEqual(JSON.parse(originalResult.stdout), {
    judgmentResult: 'SourceSimulated', sourceProfile: original,
    pre: { counter: '10', funds: '100' }, initialWork: '1000', result: originalApi,
  });

  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourcePath = example('expression-funded-payment.mori');
  const funded = invoke(fundedArgs(schemaPath, snapshotsPath, statePath, sourcePath));
  assert.equal(funded.status, 0, funded.stderr);
  assert.equal(JSON.parse(funded.stdout).sourceProfile, financial);
});
