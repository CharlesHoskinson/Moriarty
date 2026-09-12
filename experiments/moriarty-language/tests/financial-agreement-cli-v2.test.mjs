import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2.ts';
import { formatFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2-frontend.ts';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import { createFundedFinancialExpressionSourceV1 } from '../src/successor/funded-expression-source-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-financial-agreement-cli-v2-'));
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
const agreement = 'moriarty-financial-agreement-source/2';
const agreementV1 = 'moriarty-financial-agreement-source/1';
const financial = 'moriarty-financial-expression-source/1';
const original = 'moriarty-expression-source/1';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const v2Args = (sourcePath, action, snapshotsPath, statePath) => [
  'simulate', '--profile', agreement, '--action', action, '--snapshots', snapshotsPath,
  '--repayment-state', statePath, sourcePath,
];
const v1Args = (sourcePath, snapshotsPath, statePath) => [
  'simulate', '--profile', agreementV1, '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
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

test('agreement /2 check and format use no schema or action and match the API', () => {
  const sourcePath = example('multiple-action-payment.mori');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const checked = invoke(['check', '--profile', agreement, sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV2().check(sourceText)) + '\n');

  const formatted = invoke(['format', '--profile', agreement, sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSourceV2(sourceText));
  assert.equal(invoke(['format', '--profile', agreement, file(formatted.stdout)]).stdout, formatted.stdout);
});

test('agreement /2 simulate requires --action and equals the API including the action field', () => {
  const sourcePath = example('multiple-action-payment.mori');
  const snapshotsPath = example('multiple-action-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const snapshotsText = readFileSync(snapshotsPath, 'utf8');
  const stateText = readFileSync(statePath, 'utf8');
  const expected = createFinancialAgreementSourceV2().evaluate(sourceText, 'repay', snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));

  const result = invoke(v2Args(sourcePath, 'repay', snapshotsPath, statePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.deepEqual(JSON.parse(result.stdout), {
    judgmentResult: 'SourceSimulated',
    sourceProfile: agreement,
    action: 'repay',
    pre: { due: '100', paid: '0' },
    financialPre: JSON.parse(stateText),
    initialWork: '100',
    result: expected,
  });
});

test('agreement /2 continuation simulate uses first post, financialPost and remaining work', () => {
  const sourcePath = example('multiple-action-payment.mori');
  const snapshotsPath = example('multiple-action-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const first = invoke(v2Args(sourcePath, 'repay', snapshotsPath, statePath));
  assert.equal(first.status, 0, first.stderr);
  const firstEnvelope = JSON.parse(first.stdout);
  const secondSnapshots = file(canonical({
    Args: { allocationId: 'Alloc2', transferId: 'T2' },
    Obs: {},
    Pre: firstEnvelope.result.post,
    workInitial: firstEnvelope.result.financialPost.work.remaining,
  }));
  const secondState = file(JSON.stringify(firstEnvelope.result.financialPost));
  const second = invoke(v2Args(sourcePath, 'repay_installment', secondSnapshots, secondState));
  assert.equal(second.status, 0, second.stderr);
  const envelope = JSON.parse(second.stdout);
  assert.equal(envelope.action, 'repay_installment');
  assert.equal(envelope.result.post.paid, '50');
  assert.equal(envelope.result.financialPost.obligations[0].outstanding, '50');
  assert.deepEqual(envelope.result.financialPost.usedTransferIds, ['T1', 'T2']);
  assert.deepEqual(envelope.result.financialPost.usedAllocationIds, ['Alloc1', 'Alloc2']);
});

test('agreement /2 CLI writes semantic rejection to stderr with empty stdout', () => {
  const snapshotsPath = example('multiple-action-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourceText = readFileSync(example('multiple-action-payment.mori'), 'utf8');
  const failing = sourceText.replace('requires payment > 0;', 'requires payment == 1;');
  const expected = createFinancialAgreementSourceV2().evaluate(
    failing,
    'repay',
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  );
  assert.equal(expected.status, 'Rejected');
  assert.equal('post' in expected, false);
  failure(v2Args(file(failing), 'repay', snapshotsPath, statePath), expected);
});

test('/2 rejects --schema and --action on check/format; /1 and older profiles reject --action', () => {
  const sourcePath = example('multiple-action-payment.mori');
  const v1Source = example('source-defined-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('multiple-action-payment.snapshots.json');
  const v1Snapshots = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  cliFailure(['check', '--profile', agreement, '--schema', schemaPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['format', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', agreement, '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
  ], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', agreement, '--schema', schemaPath, '--action', 'repay',
    '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
  ], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', agreementV1, '--action', 'pay', '--snapshots', v1Snapshots,
    '--repayment-state', statePath, v1Source,
  ], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', financial, '--action', 'pay', '--schema', schemaPath,
    '--snapshots', v1Snapshots, v1Source,
  ], 'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', 'unknown', '--action', 'repay', '--snapshots', 'missing',
    '--repayment-state', 'missing', 'missing'], 'CLI_PROFILE', 'arguments');
});

test('agreement /2 simulate retains snapshot and repayment-state transport limits', () => {
  const sourcePath = example('multiple-action-payment.mori');
  const snapshotsPath = example('multiple-action-payment.snapshots.json');
  const expected = createFinancialAgreementSourceV2().evaluate(
    readFileSync(sourcePath, 'utf8'),
    'repay',
    readFileSync(snapshotsPath, 'utf8'),
    '{',
  );
  assert.equal(expected.status, 'Rejected');
  failure(v2Args(sourcePath, 'repay', snapshotsPath, file('{')), expected);
  cliFailure(v2Args(sourcePath, 'repay', snapshotsPath, file(' '.repeat(65537))), 'INPUT_BOUND', 'repayment-state', 1);
  cliFailure(v2Args(sourcePath, 'repay', snapshotsPath, file(new Uint8Array([0xff]))), 'INVALID_UTF8', 'repayment-state', 1);
  cliFailure(v2Args(sourcePath, 'repay', snapshotsPath, directory), 'CLI_IO', 'repayment-state');
  cliFailure(v2Args(file(' '.repeat(65537)), 'repay', snapshotsPath, file('{}')), 'SOURCE_BOUND', 'source', 1);
});

test('/1 simulate and older profiles stay unchanged and omit the /2 action field', () => {
  const sourcePath = example('source-defined-payment.mori');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const v1 = invoke(v1Args(sourcePath, snapshotsPath, statePath));
  assert.equal(v1.status, 0, v1.stderr);
  const v1Envelope = JSON.parse(v1.stdout);
  assert.equal(v1Envelope.sourceProfile, agreementV1);
  assert.equal('action' in v1Envelope, false);
  assert.deepEqual(v1Envelope.result, createFinancialAgreementSourceV1().evaluate(
    readFileSync(sourcePath, 'utf8'),
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  ));

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
  const funded = invoke([
    'simulate', '--profile', financial, '--schema', schemaPath,
    '--snapshots', snapshotsPath, '--repayment-state', statePath, example('expression-funded-payment.mori'),
  ]);
  assert.equal(funded.status, 0, funded.stderr);
  assert.equal(JSON.parse(funded.stdout).sourceProfile, financial);
  assert.deepEqual(
    JSON.parse(funded.stdout).result,
    createFundedFinancialExpressionSourceV1(readFileSync(schemaPath, 'utf8')).evaluate(
      readFileSync(example('expression-funded-payment.mori'), 'utf8'),
      readFileSync(snapshotsPath, 'utf8'),
      readFileSync(statePath, 'utf8'),
    ),
  );
});
