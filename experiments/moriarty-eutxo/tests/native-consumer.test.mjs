/**
 * Native consumer controls. No node, no key creation, no submission.
 * Receipt controls call the re-hashed helper with historical signed CBOR.
 * Those CBOR bytes are transport fixtures. They are not a financial-head proof.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { chmodSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { tmpdir } from 'node:os';
import {
  assertPinsAbsent,
  bindReceipt,
  bindWitness,
  buildConcurrentPair,
  classifyFailure,
  encodeUnsignedBody,
  inspectTransportEnvelope,
  loadReceiptHelper,
  observePosix,
  offlineWindows,
  prepareGuardSibling,
  queryProtocolParameters,
  refuseCashMint,
  resolveOutputPath,
  runNativeCase,
  separateQuantities,
  submitConcurrentPair,
  compareBudget,
  inspectLauncher,
} from '../src/native-consumer.mjs';
import { instanceDigest, genesisDigest } from '../src/same-script-identity.mjs';

const CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli';
const SNAP = '/home/charl/moriarty-eutxo-validation-2026-10-12/stage00-reviewed-snapshot/content/TASK/tooling/final-runtime';
const RECEIPTS = '/home/charl/moriarty-eutxo-validation-2026-10-12/transaction-receipts-10';
const T3B = '1697c0c373a86eea95b67763ac2e3a1c08150edfeb7030c6fae042954a8aa72b';
const t3bHex = readFileSync(join(SNAP, 'moriarty-final-wrong-datum-t3b.signed.cbor'), 'utf8').trim();
const step1Hex = readFileSync(join(SNAP, 'moriarty-final-step-1.signed.cbor'), 'utf8').trim();

test('known instance and genesis digests match the approved vectors', () => {
  const own = '44'.repeat(28);
  const cash = '11'.repeat(28);
  const instance = instanceDigest(own, cash);
  assert.equal(instance.ok, true);
  assert.equal(instance.preimage.length, 331);
  assert.equal(instance.instanceId, '2ca07ddef53c21617d50aec99b0cd9f9544c1f56d784746a2fb0702cad7b38fe');
  const review = instanceDigest('2f7ba283053de5d8aa69215d75e9651dc53ca28f30d18d6f0f68f715', '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9');
  assert.equal(review.instanceId, '08c11cf173cce63a8ef03638bf10e95b21f0a7d5cfca36e63096262db4622f6e');
  const genesis = genesisDigest({
    hashes: {
      programHash: '93ed829602f6a554d3f47879ee220c7382140045250bf53b21470ee6091297b0',
      boundsHash: 'b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d',
      sourceHash: 'facb73a98d78c4a8839e6aa05af30a7702e15798d3d7ebb15e4eb1a4b78bc736',
      schemaHash: '2254f3ccbe5494a83bef188de808473f64b220b4f2f1178976916baa7d335b4c',
      seedHash: 'fc93bf4a17f0fb0a7dd70761e36a3b903bf66843d7beea29a210329d5d1c327f',
    },
    cashPolicyId: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9',
    lender: '22'.repeat(28),
    borrower: '33'.repeat(28),
    seedTxId: '88'.repeat(32),
    seedIndex: 0,
  });
  assert.equal(genesis.ok, true, JSON.stringify(genesis));
  assert.equal(genesis.preimage.length, 943);
  assert.equal(genesis.genesis, '4eae8b9c420d77e9c18f35ee63e2fccbfd4bb7644ce3175d59bb76adf73298cf');
});

test('quoted domain text is refused instead of treated as generic JSON', () => {
  const bad = instanceDigest(`${'ab'.repeat(27)}"c`, '11'.repeat(28));
  assert.equal(bad.ok, false);
});

test('absent and synthetic pins do not become RuntimePins', async () => {
  assert.equal(assertPinsAbsent(null).code, 'NOT_EXECUTED');
  assert.equal(assertPinsAbsent({ nativePins: false }).code, 'NOT_EXECUTED');
  assert.equal(assertPinsAbsent({ unitOnly: true, nativePins: true, scriptHash: '11'.repeat(28), cashPolicyId: '22'.repeat(28) }).code, 'UNIT_FIXTURE_NOT_PINS');
  const fabricated = {
    nativePins: true,
    runtimePinsCreated: true,
    scriptHash: 'b87b493284c84a4eef169922f3f6376ecccba384f15078237cefd36e',
    cashPolicyId: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9',
    lenderVkh: 'ab'.repeat(28),
    borrowerVkh: 'cd'.repeat(28),
    seedTxId: '12'.repeat(32),
    keyPaths: ['/tmp/not-a-real-key.skey'],
    keyMode: '0600',
    utxoOwnedByRun: true,
  };
  const copied = assertPinsAbsent(fabricated);
  assert.equal(copied.nativePins, false);
  assert.equal(copied.code, 'WITNESS_KEYS_ABSENT');
  assert.equal(assertPinsAbsent({
    nativePins: true,
    scriptHash: 'e5'.repeat(28),
    cashPolicyId: 'not-hex',
    keyPaths: ['/tmp/not-a-real-key.skey'],
    keyMode: '0600',
    utxoOwnedByRun: true,
  }).code, 'MALFORMED_ID');
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-pin-'));
  try {
    const keyPath = join(dir, 'pay.skey');
    writeFileSync(keyPath, 'not-a-key\n');
    chmodSync(keyPath, 0o600);
    const blueprint = join(dir, 'plutus.applied.json');
    writeFileSync(blueprint, JSON.stringify({
      validators: [
        { title: 'financial_head.financial_head.mint', hash: 'b87b493284c84a4eef169922f3f6376ecccba384f15078237cefd36e', compiledCode: 'aa' },
        { title: 'cash.cash.mint', hash: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9', compiledCode: 'bb' },
      ],
    }));
    const mismatched = assertPinsAbsent({
      nativePins: true,
      scriptHash: 'b87b493284c84a4eef169922f3f6376ecccba384f15078237cefd36e',
      cashPolicyId: '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9',
      keyPath,
      appliedBlueprintPath: blueprint,
      utxoOwnedByRun: true,
    });
    assert.equal(mismatched.nativePins, false);
    assert.equal(mismatched.code, 'PIN_RELATION_MISMATCH');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
  const ran = await runNativeCase({
    pins: fabricated,
    posix: {
      probeExecuted: true,
      evidenceClass: 'node-genesis-v3-probe',
      slotLengthMs: '1000',
      lucidPosix: '1000',
      cliPosix: '1000',
      scriptContextLower: '1000',
      measuredByThisProcess: true,
    },
    authorizeLedger: true,
  });
  assert.equal(ran.code, 'WITNESS_KEYS_ABSENT');
  assert.notEqual(ran.code, 'CALIBRATED_SOURCE_NOT_THIS_PHASE');
  assert.notEqual(ran.code, 'AUTHORING_STAGE_REFUSES_EXECUTION');
  assert.equal(ran.ledgerExecution, false);
  assert.equal(ran.confirmed, false);
  assert.equal(ran.nativePins, false);
});

test('protocol min-ADA, kernel Cash, and the unit payout stay three quantities', () => {
  const ok = separateQuantities({ protocolMinAda: '5000000', kernelCash: 100, unitPayout: 2000000 });
  assert.equal(ok.ok, true);
  assert.notEqual(ok.protocolMinAda, ok.unitPayout);
  assert.equal(separateQuantities({ protocolMinAda: '2000000', kernelCash: 100, unitPayout: 2000000 }).code, 'QUANTITY_COLLAPSED');
  assert.equal(separateQuantities({ kernelCash: 100, unitPayout: 2000000 }).code, 'MIN_ADA_UNMEASURED');
});

test('slot length other than 1000 and unmeasured parity do not emit a sibling', () => {
  assert.equal(observePosix(null).code, 'PARITY_ABSENT');
  assert.equal(observePosix({ slotLengthMs: '500' }).code, 'SLOT_LENGTH_REFUSED');
  assert.equal(prepareGuardSibling({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1001',
  }).code, 'IMPOSSIBLE_BOUNDARY');
  const forged = prepareGuardSibling({
    probeExecuted: true,
    evidenceClass: 'node-genesis-v3-probe',
    slotLengthMs: '1000',
    lucidPosix: '1000',
    cliPosix: '1000',
    scriptContextLower: '1000',
    measuredByThisProcess: true,
  });
  assert.equal(forged.siblingEmitted, false);
  assert.equal(forged.code, 'PARITY_NOT_MEASURED');
  assert.equal(forged.flagIgnored, true);
  const windows = offlineWindows({ zeroTime: '0', zeroSlot: '0', slotLengthMs: '1000', tipSlot: '10' });
  assert.equal(windows.ok, true);
  assert.equal(windows.ledgerExecution, false);
  assert.equal(windows.siblingEmitted, false);
});

test('concurrent bodies share only the head and are not submitted', () => {
  const pair = buildConcurrentPair(
    { outRef: 'head#0' },
    { fee: { outRef: 'a-fee#0' }, collateral: { outRef: 'a-col#0' }, requiredSigners: ['lender'] },
    { fee: { outRef: 'b-fee#0' }, collateral: { outRef: 'b-col#0' }, requiredSigners: ['borrower'] },
  );
  assert.equal(pair.ok, true);
  assert.deepEqual(pair.shared, ['head']);
  assert.equal(pair.submitted, false);
  assert.equal(pair.confirmed, false);
  assert.equal(pair.replay.executed, false);
  assert.equal(pair.members[0].head, pair.members[1].head);
  assert.notEqual(pair.members[0].fee, pair.members[1].fee);
  assert.equal(buildConcurrentPair(
    { outRef: 'head#0' },
    { fee: { outRef: 'same#0' }, collateral: { outRef: 'a-col#0' } },
    { fee: { outRef: 'same#0' }, collateral: { outRef: 'b-col#0' } },
  ).ok, false);
});

test('failure text keeps phase-2 PlutusFailure apart from phase-1 and collateral', () => {
  const plutus = classifyFailure('ShelleyTxValidationError PlutusFailure budget');
  assert.equal(plutus.failureClass, 'PlutusFailure');
  assert.equal(plutus.predicateReached, false);
  assert.equal(classifyFailure('InvalidWitnessesUTXOW').phase, 1);
  assert.equal(classifyFailure('BadInputsUTxO').failureClass, 'BadInputsUTxO');
  assert.equal(classifyFailure('OutsideValidityIntervalUTxO').failureClass, 'OutsideValidityInterval');
  assert.equal(classifyFailure('{"isValid": false}').predicateReached, false);
  assert.equal(classifyFailure('ogmios preflight rejected').failureClass, 'provider-preflight');
  const mixedPhase = classifyFailure('ShelleyTxValidationError InvalidWitnessesUTXOW before PlutusFailure');
  assert.equal(mixedPhase.failureClass, 'MIXED_FAILURE');
  assert.equal(mixedPhase.predicateReached, false);
  assert.equal(mixedPhase.phase, 'mixed');
  const mixedProvider = classifyFailure('ogmios preflight rejected before PlutusFailure');
  assert.equal(mixedProvider.failureClass, 'MIXED_FAILURE');
  assert.equal(mixedProvider.predicateReached, false);
  const corroborated = classifyFailure('PlutusFailure', { isValid: true, signed: true });
  assert.equal(corroborated.failureClass, 'PlutusFailure');
  assert.equal(corroborated.predicateReached, true);
  assert.equal(corroborated.ledgerExecution, false);
});

test('receipt helper hash, namespace, mismatch, timeout, collision, late poll, and hang', async () => {
  const helper = loadReceiptHelper();
  assert.equal(helper.ok, true);
  const root = mkdtempSync(join(RECEIPTS, 't1-native-46-'));
  try {
  const outside = await bindReceipt({
    helperPath: RECEIPT_HELPER_PATH_MISSING(),
    label: 'nope',
    signedCborHex: t3bHex,
    cliPath: CLI,
    receiptRoot: join(tmpdir(), 'not-allowed'),
  });
  assert.equal(outside.ok, false);
  const mismatch = await bindReceipt({
    label: 'id-mismatch',
    signedCborHex: t3bHex,
    cliPath: CLI,
    receiptRoot: root,
    submit: async () => 'ff'.repeat(32),
    pollConfirmation: async () => { throw new Error('poll must not run'); },
  });
  assert.equal(mismatch.failureClass, 'transport-binding-failure');
  assert.equal(mismatch.confirmed, false);
  const timeout = await bindReceipt({
    label: 'timeout',
    signedCborHex: t3bHex,
    cliPath: CLI,
    receiptRoot: root,
    confirmationBudgetMs: 20,
    submit: async () => T3B,
    pollConfirmation: async () => 'unconfirmed',
  });
  assert.equal(timeout.failureClass, 'confirmation-timeout');
  assert.equal(timeout.confirmed, false);
  assert.equal(timeout.timeoutIsConfirmation, false);
  const collision = await bindReceipt({
    label: 'collision',
    signedCborHex: t3bHex,
    cliPath: CLI,
    receiptRoot: root,
    submit: async () => T3B,
  });
  assert.equal(collision.receipt.status, 'submitted');
  assert.equal(collision.confirmed, false);
  const collided = await bindReceipt({
    label: 'collision',
    signedCborHex: step1Hex,
    cliPath: CLI,
    receiptRoot: root,
  });
  assert.equal(collided.failureClass, 'label-collision');
  let release;
  const hang = await bindReceipt({
    label: 'hang',
    signedCborHex: step1Hex,
    cliPath: CLI,
    receiptRoot: root,
    confirmationBudgetMs: 40,
    submit: async () => '59c11a0edd665cb2c5aa9f77106323a7d6b695752f82344b4614e91ab0695c4e',
    pollConfirmation: () => new Promise((resolve) => { release = resolve; }),
  });
  assert.equal(hang.failureClass, 'confirmation-timeout');
  release({ status: 'confirmed' });
  assert.equal(hang.confirmed, false);
  const late = await bindReceipt({
    label: 'late',
    signedCborHex: t3bHex,
    cliPath: CLI,
    receiptRoot: root,
    confirmationBudgetMs: 30,
    submit: async () => T3B,
    pollConfirmation: () => new Promise((resolve) => setTimeout(() => resolve({ status: 'confirmed' }), 80)),
  });
  assert.equal(late.confirmed, false);
  assert.notEqual(late.receipt?.status, 'confirmed');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('fail-closed Cash is not a mint and an unsigned body is not acceptance', () => {
  const refused = refuseCashMint();
  assert.equal(refused.code, 'CASH_POLICY_FAIL_CLOSED');
  assert.equal(refused.cashCanMint, false);
  assert.equal(refused.financialValidatorProof, false);
  assert.equal(refused.submitted, false);
  const body = encodeUnsignedBody({
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 1 },
    collateral: { txId: '33'.repeat(32), index: 2 },
    outputLovelace: '2000000',
    feeLovelace: '200000',
    invalidBefore: 40,
    invalidHereafter: 100,
  });
  assert.equal(body.ok, true, JSON.stringify({ code: body.code, detail: body.detail }));
  assert.match(body.txid, /^[0-9a-f]{64}$/);
  const again = encodeUnsignedBody({
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 1 },
    collateral: { txId: '33'.repeat(32), index: 2 },
    outputLovelace: '2000000',
    feeLovelace: '200000',
    invalidBefore: 40,
    invalidHereafter: 100,
  });
  assert.equal(again.txid, body.txid);
  assert.equal(body.ledgerExecution, false);
  assert.equal(body.signed, false);
  assert.equal(body.nativeAcceptance, false);
  assert.equal(body.collateralInLedgerBody, true);
  assert.equal(body.scriptContextHasCollateralField, false);
  assert.equal(encodeUnsignedBody({
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '11'.repeat(32), index: 0 },
    collateral: { txId: '33'.repeat(32), index: 2 },
    requiredSignerHash: 'cd'.repeat(28),
  }).code, 'INPUTS_NOT_DISJOINT');
  assert.equal(bindWitness({}).code, 'WITNESS_KEYS_ABSENT');
  assert.equal(bindWitness({ signingKeyPath: '/tmp/not-created.skey' }).code, 'WITNESS_KEYS_ABSENT');
  const keyDir = mkdtempSync(join(tmpdir(), 'moriarty-key-'));
  try {
    const loose = join(keyDir, 'loose.skey');
    const owned = join(keyDir, 'owned.skey');
    writeFileSync(loose, 'not-a-key\n');
    chmodSync(loose, 0o644);
    writeFileSync(owned, 'not-a-key\n');
    chmodSync(owned, 0o600);
    assert.equal(bindWitness({ signingKeyPath: loose, txBodyCborHex: body.cborHex, txBodyType: body.type }).code, 'KEY_MODE_REFUSED');
    const reached = bindWitness({ signingKeyPath: owned, txBodyCborHex: body.cborHex, txBodyType: body.type });
    assert.equal(reached.constructorReached, true);
    assert.equal(reached.signed, false);
    assert.equal(reached.code, 'WITNESS_CONSTRUCTOR_REJECTED');
    assert.equal(reached.ledgerExecution, false);
    assert.equal(Object.hasOwn(reached, 'detail'), false);
  } finally {
    rmSync(keyDir, { recursive: true, force: true });
  }
  const transport = inspectTransportEnvelope({ cborHex: t3bHex, expectedTxId: T3B });
  assert.equal(transport.ok, true, JSON.stringify(transport));
  assert.equal(transport.nativeAcceptance, false);
  assert.equal(transport.layer, 'transport-binding');
});

test('budget, launcher, output path, and race order stay unit-only', async () => {
  assert.equal(queryProtocolParameters({ socket: '/tmp/moriarty-no-such-socket.sock' }).code, 'PROTOCOL_PARAMS_ABSENT');
  const over = compareBudget({
    queried: true,
    unitOnly: true,
    maxTxExecutionUnits: { memory: '1000', steps: '1000' },
    cost: { memory: '111620000', steps: '32820000000' },
  });
  assert.equal(over.code, 'BUDGET_EXCEEDED');
  assert.equal(over.submitBlocked, true);
  assert.equal(over.nativeBudgetMeasured, false);
  assert.equal(over.quantities.kernelCash, '100');
  assert.equal(over.quantities.unitPayout, '2000000');
  assert.equal(over.quantities.protocolMinAda, 'unmeasured');
  const blocked = await runNativeCase({
    submit: true,
    budget: {
      queried: true,
      unitOnly: true,
      maxTxExecutionUnits: { memory: '1000', steps: '1000' },
      cost: { memory: '111620000', steps: '32820000000' },
    },
  });
  assert.equal(blocked.code, 'BUDGET_EXCEEDED');
  assert.equal(blocked.submitted, false);
  assert.equal(blocked.stages.submit.submitBlocked, true);
  const absent = await runNativeCase({ submit: true });
  assert.equal(absent.submitted, false);
  assert.equal(absent.stages.budget.code, 'PROTOCOL_PARAMS_ABSENT');
  const launcher = await inspectLauncher({ startNode: false });
  assert.equal(launcher.started, false);
  assert.equal(launcher.setsid, true);
  assert.equal(launcher.port.ok, true);
  assert.notEqual(launcher.code, 'AUTHORING_STAGE_REFUSES_EXECUTION');
  const missing = await inspectLauncher({ nodeBin: '/tmp/moriarty-no-such-cardano-node', startNode: true });
  assert.equal(missing.code, 'NODE_BINARY_ABSENT');
  assert.equal(missing.spawnAttempted, true);
  assert.equal(missing.started, false);
  assert.equal(inspectLauncher({ wrapper: 'yaci-npm' }) instanceof Promise, true);
  const yaci = await inspectLauncher({ wrapper: 'yaci-npm' });
  assert.equal(yaci.code, 'YACI_WRAPPER_REFUSED');
  const historical = resolveOutputPath('/home/charl/moriarty-eutxo-validation-2026-10-12/t1-capacity-repair-25/checks/cli-custody');
  assert.equal(historical.code, 'OUTPUT_PATH_FORBIDDEN');
  const fresh = resolveOutputPath(null);
  assert.equal(fresh.ok, true);
  assert.match(fresh.path, /t1-native-adapters-57\/checks\/cli-custody$/);
  const pair = buildConcurrentPair(
    { outRef: 'head#0' },
    { fee: { outRef: 'a-fee#0' }, collateral: { outRef: 'a-col#0' } },
    { fee: { outRef: 'b-fee#0' }, collateral: { outRef: 'b-col#0' } },
  );
  const order = [];
  const raced = await submitConcurrentPair(pair, {
    unitOnly: true,
    submit: async (member) => {
      order.push(`submit:${member.fee}`);
      return 'unit-submit';
    },
    poll: async () => {
      order.push('poll');
    },
  });
  assert.equal(raced.submitsBeforeConfirms, true);
  assert.equal(raced.submitCount, 2);
  assert.equal(raced.ledgerSubmitted, false);
  assert.equal(raced.confirmed, false);
  assert.equal(raced.replay.executed, false);
  assert.equal(raced.replay.expectedClass, 'BadInputsUTxO');
  assert.deepEqual(order.slice(0, 2), ['submit:a-fee#0', 'submit:b-fee#0']);
  assert.equal(order.filter((item) => item === 'poll').length, 2);
  assert.ok(order.indexOf('poll') > order.lastIndexOf('submit:b-fee#0') || order[2] === 'poll');
});

test('cli native passes a missing key to the consumer and refuses a historical out path', () => {
  const root = fileURLToPath(new URL('..', import.meta.url));
  const task = '/home/charl/moriarty-eutxo-validation-2026-10-12';
  const empty = spawnSync(process.execPath, [
    '--experimental-strip-types', 'src/cli.mjs', 'native',
    '--out', join(task, 't1-consumer-repair-119/checks/native-consumer/native-cli-empty'),
  ], { cwd: root, encoding: 'utf8' });
  assert.equal(empty.status, 0, empty.stderr);
  const emptyReport = JSON.parse(empty.stdout);
  assert.equal(emptyReport.code, 'NOT_EXECUTED');
  assert.equal(emptyReport.nativePins, false);
  assert.equal(emptyReport.startedNode, false);
  assert.equal(emptyReport.submitted, false);
  assert.equal(emptyReport.shellExitMeansLedger, false);
  const missing = spawnSync(process.execPath, [
    '--experimental-strip-types', 'src/cli.mjs', 'native',
    '--script-hash', 'ab'.repeat(28),
    '--cash-policy', 'cd'.repeat(28),
    '--signing-key', '/tmp/not-created.skey',
    '--out', join(task, 't1-consumer-repair-119/checks/native-consumer/native-cli-missing-key'),
  ], { cwd: root, encoding: 'utf8' });
  assert.equal(missing.status, 0, missing.stderr);
  const report = JSON.parse(missing.stdout);
  assert.equal(report.code, 'WITNESS_KEYS_ABSENT');
  assert.equal(report.stages.pins, 'WITNESS_KEYS_ABSENT');
  assert.equal(report.stages.witness, 'WITNESS_KEYS_ABSENT');
  assert.equal(report.nativePins, false);
  assert.equal(report.startedNode, false);
  assert.equal(report.submitted, false);
  assert.equal(report.confirmed, false);
  assert.equal(report.ledgerExecution, false);
  const forbidden = spawnSync(process.execPath, [
    '--experimental-strip-types', 'src/cli.mjs', 'native',
    '--out', join(task, 't1-capacity-repair-25/checks/cli-custody'),
  ], { cwd: root, encoding: 'utf8' });
  assert.equal(forbidden.status, 1);
  assert.match(`${forbidden.stdout}${forbidden.stderr}`, /OUTPUT_PATH_FORBIDDEN/);
});

function RECEIPT_HELPER_PATH_MISSING() {
  return '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/transaction-receipts/revision02/submit-with-receipt.mjs';
}
