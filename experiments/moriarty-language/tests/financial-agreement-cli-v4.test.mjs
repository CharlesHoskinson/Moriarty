import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';
import { formatFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4-frontend.ts';
import { createFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-financial-agreement-cli-v4-'));
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
const agreement = 'moriarty-financial-agreement-source/4';
const agreementV3 = 'moriarty-financial-agreement-source/3';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const v4Args = (sourcePath, action, snapshotsPath, statePath) => [
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

test('agreement /4 check and format use no schema or action and match the API', () => {
  const sourcePath = example('financial-postconditions-payment.mori');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const checked = invoke(['check', '--profile', agreement, sourcePath]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(createFinancialAgreementSourceV4().check(sourceText)) + '\n');

  const formatted = invoke(['format', '--profile', agreement, sourcePath]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stdout, formatFinancialAgreementSourceV4(sourceText));
  assert.equal(invoke(['format', '--profile', agreement, file(formatted.stdout)]).stdout, formatted.stdout);
});

test('agreement /4 simulate requires --action and equals the API including the action field', () => {
  const sourcePath = example('financial-postconditions-payment.mori');
  const statePath = example('financial-state-payment.state.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const stateText = readFileSync(statePath, 'utf8');
  const snapshotsText = canonical({
    Args: { allocationId: 'Alloc1', transferId: 'T1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: '256',
  });
  const snapshotsPath = file(snapshotsText);
  const expected = createFinancialAgreementSourceV4().evaluate(sourceText, 'repay_remaining', snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  assert.equal(expected.financialPost.work.spent, '82');

  const result = invoke(v4Args(sourcePath, 'repay_remaining', snapshotsPath, statePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.deepEqual(JSON.parse(result.stdout), {
    judgmentResult: 'SourceSimulated',
    sourceProfile: agreement,
    action: 'repay_remaining',
    pre: { due: '100', paid: '0' },
    financialPre: JSON.parse(stateText),
    initialWork: '256',
    result: expected,
  });
});

test('agreement /4 CLI writes semantic rejection to stderr with empty stdout', () => {
  const sourceText = readFileSync(example('financial-postconditions-payment.mori'), 'utf8');
  const failing = sourceText.replace(
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(1);',
  );
  const snapshotsText = canonical({
    Args: { allocationId: 'Alloc1', transferId: 'T1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: '256',
  });
  const expected = createFinancialAgreementSourceV4().evaluate(
    failing,
    'repay_remaining',
    snapshotsText,
    readFileSync(example('financial-state-payment.state.json'), 'utf8'),
  );
  assert.equal(expected.status, 'Rejected');
  assert.equal(expected.code, 'ENSURES_FAILED');
  failure(
    v4Args(file(failing), 'repay_remaining', file(snapshotsText), example('financial-state-payment.state.json')),
    expected,
  );
});

test('/4 rejects --schema and --action on check/format; /3 stays on its form', () => {
  const sourcePath = example('financial-postconditions-payment.mori');
  const v3Source = example('financial-state-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('financial-state-payment.snapshots.json');
  const statePath = example('financial-state-payment.state.json');
  cliFailure(['check', '--profile', agreement, '--schema', schemaPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['format', '--profile', agreement, '--action', 'repay', sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure([
    'simulate', '--profile', agreement, '--snapshots', snapshotsPath, '--repayment-state', statePath, sourcePath,
  ], 'CLI_USAGE', 'arguments');
  const v3 = invoke([
    'simulate', '--profile', agreementV3, '--action', 'repay', '--snapshots', snapshotsPath,
    '--repayment-state', statePath, v3Source,
  ]);
  assert.equal(v3.status, 0, v3.stderr);
  assert.equal(JSON.parse(v3.stdout).sourceProfile, agreementV3);
  assert.deepEqual(JSON.parse(v3.stdout).result, createFinancialAgreementSourceV3().evaluate(
    readFileSync(v3Source, 'utf8'),
    'repay',
    readFileSync(snapshotsPath, 'utf8'),
    readFileSync(statePath, 'utf8'),
  ));
});
