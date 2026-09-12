import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3.ts';
import { formatFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3-frontend.ts';
import { createFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-financial-agreement-cli-v3-'));
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
const agreement = 'moriarty-financial-agreement-source/3';
const agreementV2 = 'moriarty-financial-agreement-source/2';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const v3Args = (sourcePath, action, snapshotsPath, statePath) => [
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

test('agreement /3 check and format use no schema or action and match the API', () => {
  const sourcePath = example('financial-state-payment.mori');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const checked = invoke(['check', '--profile', agreement, sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV3().check(sourceText)) + '\n');

  const formatted = invoke(['format', '--profile', agreement, sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSourceV3(sourceText));
  assert.equal(invoke(['format', '--profile', agreement, file(formatted.stdout)]).stdout, formatted.stdout);
});

test('agreement /3 simulate requires --action and equals the API including the action field', () => {
  const sourcePath = example('financial-state-payment.mori');
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const statePath = example('financial-state-payment.state.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const snapshotsText = readFileSync(snapshotsPath, 'utf8');
  const stateText = readFileSync(statePath, 'utf8');
  const expected = createFinancialAgreementSourceV3().evaluate(sourceText, 'repay', snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));

  const result = invoke(v3Args(sourcePath, 'repay', snapshotsPath, statePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.deepEqual(JSON.parse(result.stdout), {
    judgmentResult: 'SourceSimulated',
    sourceProfile: agreement,
    action: 'repay',
    pre: { due: '100', paid: '0' },
    financialPre: JSON.parse(stateText),
    initialWork: '256',
    result: expected,
  });
});

test('agreement /3 continuation simulate uses remaining work and complete financialPost', () => {
  const sourcePath = example('financial-state-payment.mori');
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const statePath = example('financial-state-payment.state.json');
  const first = invoke(v3Args(sourcePath, 'repay', snapshotsPath, statePath));
  assert.equal(first.status, 0, first.stderr);
  const firstEnvelope = JSON.parse(first.stdout);
  const secondSnapshots = file(canonical({
    Args: { allocationId: 'Alloc2', transferId: 'T2' },
    Obs: {},
    Pre: firstEnvelope.result.post,
    workInitial: firstEnvelope.result.financialPost.work.remaining,
  }));
  const secondState = file(JSON.stringify(firstEnvelope.result.financialPost));
  const second = invoke(v3Args(sourcePath, 'repay_installment', secondSnapshots, secondState));
  assert.equal(second.status, 0, second.stderr);
  const envelope = JSON.parse(second.stdout);
  assert.equal(envelope.action, 'repay_installment');
  assert.equal(envelope.result.post.paid, '50');
  assert.equal(envelope.result.financialPost.obligations[0].outstanding, '50');
});

test('agreement /3 CLI writes semantic rejection to stderr with empty stdout', () => {
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const statePath = example('financial-state-payment.state.json');
  const sourceText = readFileSync(example('financial-state-payment.mori'), 'utf8');
  const failing = sourceText.replace('requires payment > 0;', 'requires payment == 1;');
  const expected = createFinancialAgreementSourceV3().evaluate(
    failing,
    'repay',
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  );
  assert.equal(expected.status, 'Rejected');
  failure(v3Args(file(failing), 'repay', snapshotsPath, statePath), expected);
});

test('/3 rejects --schema and --action on check/format; /2 stays on its form', () => {
  const sourcePath = example('financial-state-payment.mori');
  const v2Source = example('multiple-action-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const v2Snapshots = example('multiple-action-payment.snapshots.json');
  const statePath = example('financial-state-payment.state.json');
  const v2State = example('expression-funded-payment.state.json');
  cliFailure(['check', '--profile', agreement, '--schema', schemaPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['format', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', agreement, '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
  ], 'CLI_USAGE', 'arguments');
  const v2 = invoke([
    'simulate', '--profile', agreementV2, '--action', 'repay', '--snapshots', v2Snapshots,
    '--repayment-state', v2State, v2Source,
  ]);
  assert.equal(v2.status, 0, v2.stderr);
  assert.equal(JSON.parse(v2.stdout).sourceProfile, agreementV2);
  assert.deepEqual(JSON.parse(v2.stdout).result, createFinancialAgreementSourceV2().evaluate(
    readFileSync(v2Source, 'utf8'),
    'repay',
    readFileSync(v2Snapshots, 'utf8'),
    readFileSync(v2State, 'utf8'),
  ));
});

test('state admission rejection is kernel-shaped on the CLI', () => {
  const sourcePath = example('financial-state-payment.mori');
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const bad = JSON.parse(readFileSync(example('financial-state-payment.state.json'), 'utf8'));
  bad.obligations.push(bad.obligations[0]);
  const expected = createFinancialAgreementSourceV3().evaluate(
    readFileSync(sourcePath, 'utf8'),
    'repay',
    readFileSync(snapshotsPath, 'utf8'),
    JSON.stringify(bad),
  );
  assert.equal(expected.code, 'DUPLICATE');
  failure(v3Args(sourcePath, 'repay', snapshotsPath, file(JSON.stringify(bad))), expected);
});
