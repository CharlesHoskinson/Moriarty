import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createFundedFinancialExpressionSourceV1 } from '../src/successor/funded-expression-source-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-funded-expression-cli-'));
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
const financial = 'moriarty-financial-expression-source/1';
const original = 'moriarty-expression-source/1';
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
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

const transferFields = {
  from: ['Text'],
  id: ['Text'],
  settlementAsset: ['Text'],
  to: ['Text'],
  transferAmount: ['Amount', 'Cash'],
};
const repayFields = {
  allocationId: ['Text'],
  nominalAmount: ['Quantity', [['Cash', '1']], '0'],
  obligationId: ['Text'],
  payer: ['Text'],
  transferId: ['Text'],
};
const schemaText = canonical({
  args: {},
  assets: ['Cash'],
  enumTypes: {},
  fields: {
    due: { type: ['UInt128'], writeClass: 'ordinary' },
    paid: { type: ['UInt128'], writeClass: 'ordinary' },
  },
  observations: {},
  operations: { Repay: 'RepayFields', Transfer: 'TransferFields' },
  parties: [],
  recordTypes: { RepayFields: repayFields, TransferFields: transferFields },
  units: ['Cash'],
  variantTypes: {},
  vaults: [],
});
const sourceText = `profile "${financial}"; agreement Loan { action pay() {
  requires pre.due > 0;
  let payment = floor_div(pre.due * 3, 10);
  requires payment == 30;
  next.paid = pre.paid + payment;
  emit Transfer { id: "T1", from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };
  emit Repay { allocationId: "Alloc1", transferId: "T1", obligationId: "Due100", payer: "Payer", nominalAmount: quantity<Units<Cash,1>,0>(30) };
  ensures post.paid == pre.paid + payment;
} }`;
const snapshotsText = canonical({ Args: {}, Obs: {}, Pre: { due: '100', paid: '0' }, workInitial: '100' });
const stateObject = {
  balances: [
    { party: 'Payer', asset: 'Cash', amount: '100' },
    { party: 'Lender', asset: 'Cash', amount: '0' },
  ],
  allowances: [{ party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' }],
  obligations: [{
    id: 'Due100',
    debtor: 'Payer',
    creditor: 'Lender',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    principal: '100',
    accrued: '0',
    outstanding: '100',
    allocationRule: 'AccrualFirst',
    conversion: { mantissa: '1', scale: '0', rounding: 'none' },
    status: 'Outstanding',
  }],
  usedTransferIds: [],
  usedAllocationIds: [],
  work: { remaining: '100', spent: '0', closureReserve: '16' },
};
const stateText = JSON.stringify(stateObject);

test('funded simulate CLI equals the adapter and includes initial ordinary and financial state', () => {
  const schemaPath = file(schemaText);
  const snapshotsPath = file(snapshotsText);
  const statePath = file(stateText);
  const sourcePath = file(sourceText);
  const expected = createFundedFinancialExpressionSourceV1(schemaText).evaluate(sourceText, snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  const result = invoke(fundedArgs(schemaPath, snapshotsPath, statePath, sourcePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.deepEqual(JSON.parse(result.stdout), {
    judgmentResult: 'SourceSimulated',
    sourceProfile: financial,
    pre: { due: '100', paid: '0' },
    financialPre: JSON.parse(stateText),
    initialWork: '100',
    result: expected,
  });
});

test('funded simulate CLI writes semantic rejection to stderr with empty stdout', () => {
  const schemaPath = file(schemaText);
  const snapshotsPath = file(snapshotsText);
  const statePath = file(stateText);
  const failing = sourceText.replace('requires payment == 30;', 'requires payment == 1;');
  const expected = createFundedFinancialExpressionSourceV1(schemaText).evaluate(failing, snapshotsText, stateText);
  assert.equal(expected.status, 'Rejected');
  assert.equal('post' in expected, false);
  assert.equal('financialPost' in expected, false);
  assert.equal('effects' in expected, false);
  failure(fundedArgs(schemaPath, snapshotsPath, statePath, file(failing)), expected);
});

test('repayment-state is rejected for the original profile, check, format and reordered flags', () => {
  const schemaPath = file(schemaText);
  const snapshotsPath = file(snapshotsText);
  const statePath = file(stateText);
  const sourcePath = file(sourceText);
  cliFailure(fundedArgs(schemaPath, snapshotsPath, statePath, sourcePath).map((part) => part === financial ? original : part),
    'CLI_USAGE', 'arguments');
  cliFailure(['check', '--profile', financial, '--schema', schemaPath, '--repayment-state', statePath, sourcePath],
    'CLI_USAGE', 'arguments');
  cliFailure(['format', '--profile', financial, '--repayment-state', statePath, sourcePath],
    'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', financial, '--schema', schemaPath, '--repayment-state', statePath,
    '--snapshots', snapshotsPath, sourcePath], 'CLI_USAGE', 'arguments');
  cliFailure(['simulate', '--profile', 'unknown', '--schema', 'missing', '--snapshots', 'missing',
    '--repayment-state', 'missing', 'missing'], 'CLI_PROFILE', 'arguments');
});

test('funded simulate CLI retains repayment-state transport limits and malformed JSON', () => {
  const schemaPath = file(schemaText);
  const snapshotsPath = file(snapshotsText);
  const sourcePath = file(sourceText);
  const expected = createFundedFinancialExpressionSourceV1(schemaText).evaluate(sourceText, snapshotsText, '{');
  assert.equal(expected.status, 'Rejected');
  failure(fundedArgs(schemaPath, snapshotsPath, file('{'), sourcePath), expected);
  cliFailure(fundedArgs(schemaPath, snapshotsPath, file(' '.repeat(65537)), sourcePath), 'INPUT_BOUND', 'repayment-state', 1);
  cliFailure(fundedArgs(schemaPath, snapshotsPath, file(new Uint8Array([0xff])), sourcePath), 'INVALID_UTF8', 'repayment-state', 1);
  cliFailure(fundedArgs(schemaPath, snapshotsPath, directory, sourcePath), 'CLI_IO', 'repayment-state');
});

test('pure simulate, check and format stay unchanged without repayment-state', () => {
  const financialSource = example('financial-vault-quote.mori');
  const financialSchema = example('financial-vault-quote.schema.json');
  const financialSnapshots = example('financial-vault-quote.snapshots.json');
  const financialResult = invoke(simulateArgs(financial, financialSchema, financialSnapshots, financialSource));
  assert.equal(financialResult.status, 0, financialResult.stderr);
  assert.equal(financialResult.stderr, '');
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

  const checked = invoke(['check', '--profile', financial, '--schema', financialSchema, financialSource]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stdout, JSON.stringify(createFinancialExpressionSourceV1(readFileSync(financialSchema, 'utf8'))
    .check(readFileSync(financialSource, 'utf8'))) + '\n');
});

test('published expression-funded-payment example runs through the normal CLI', () => {
  const sourcePath = example('expression-funded-payment.mori');
  const schemaPath = example('expression-funded-payment.schema.json');
  const snapshotsPath = example('expression-funded-payment.snapshots.json');
  const statePath = example('expression-funded-payment.state.json');
  const sourceText = readFileSync(sourcePath, 'utf8');
  const schemaText = readFileSync(schemaPath, 'utf8');
  const snapshotsText = readFileSync(snapshotsPath, 'utf8');
  const stateText = readFileSync(statePath, 'utf8');
  const expected = createFundedFinancialExpressionSourceV1(schemaText).evaluate(sourceText, snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  const result = invoke(fundedArgs(schemaPath, snapshotsPath, statePath, sourcePath));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  const body = JSON.parse(result.stdout);
  assert.equal(body.judgmentResult, 'SourceSimulated');
  assert.equal(body.sourceProfile, financial);
  assert.deepEqual(body.result, expected);
  assert.equal(body.result.post.paid, '30');
  assert.equal(body.result.financialPost.balances[0].amount, '70');
  assert.equal(body.result.financialPost.obligations[0].principal, '70');
  assert.equal(body.result.effects[1].nominalAmount, '30');

  const secondSnapshots = canonical({
    Args: { allocationId: 'Alloc2', first: '10', second: '10', transferId: 'T2' },
    Obs: {},
    Pre: { due: '100', paid: '30' },
    workInitial: expected.workRemaining,
  });
  const secondState = JSON.stringify(expected.financialPost);
  const secondExpected = createFundedFinancialExpressionSourceV1(schemaText)
    .evaluate(sourceText, secondSnapshots, secondState);
  assert.equal(secondExpected.status, 'FundedExpressionPrepared', JSON.stringify(secondExpected));
  const second = invoke(fundedArgs(schemaPath, file(secondSnapshots), file(secondState), sourcePath));
  assert.equal(second.status, 0, second.stderr);
  const secondBody = JSON.parse(second.stdout);
  assert.deepEqual(secondBody.result, secondExpected);
  assert.equal(secondBody.result.post.paid, '50');
  assert.equal(secondBody.result.financialPost.balances[0].amount, '50');
  assert.equal(secondBody.result.financialPost.obligations[0].principal, '50');
  assert.equal(secondBody.result.financialPost.work.closureReserve, '16');
  assert.deepEqual(secondBody.result.financialPost.usedTransferIds, ['T1', 'T2']);
});
