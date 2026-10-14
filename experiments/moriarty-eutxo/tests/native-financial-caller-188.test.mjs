import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { datumFromPost, judgeEconomicAction } from '../src/native-financial.mjs';
import {
  encodeUnitConwayEnvelope,
  readConwayEnvelope,
  recomputeScriptDataHash,
  redeemerPointerAccepts,
  selectNeededScriptLanguage,
  sortedOrdinaryRefs,
  verifyExactBodyWitness,
} from '../src/native-integrate.mjs';
import { initialHeadDatum, runNativeCaller, runOwnedRace } from '../src/native-caller.mjs';
import { produceOwnedRuntime } from '../src/native-runtime.mjs';

const OWN = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-f207';
const ROOT = OWN;
const RUN_DIR = join(OWN, 'a');
const SOCKET_BASENAME = 'node.sock';
const PROTOCOL = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/actual-protocol-parameters.json';
const CML_PATH = '/home/charl/moriarty-eutxo-validation-2026-10-12/integrity140-verified-freeze148/installed-cml/cardano_multiplatform_lib.js';
const GOLDEN = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-68/sources/cardano-cli/cardano-cli/test/cardano-cli-golden/files/input/conway/witnessed.tx';
const requireCml = createRequire(CML_PATH);

mkdirSync(join(ROOT, 'logs'), { recursive: true });
mkdirSync(join(ROOT, 'checks'), { recursive: true });

function utf8(node) {
  return Buffer.from(node.bytes, 'hex').toString('utf8');
}

function releasePost(extra = {}) {
  return {
    schemaVersion: 'moriarty-financial-lifecycle-state/1',
    balances: [
      { party: 'Escrow', asset: 'Cash', amount: '0' },
      { party: 'Borrower', asset: 'Cash', amount: '0' },
      { party: 'Lender', asset: 'Cash', amount: '100' },
    ],
    allowances: [
      { party: 'Escrow', asset: 'Cash', remaining: '0', spent: '100' },
      { party: 'Borrower', asset: 'Cash', remaining: '0', spent: '0' },
      { party: 'Lender', asset: 'Cash', remaining: '0', spent: '0' },
    ],
    obligations: [{
      id: 'duty',
      debtor: 'Borrower',
      creditor: 'Lender',
      denomination: 'Cash',
      settlementAsset: 'Cash',
      principal: '1',
      accrued: '0',
      outstanding: '1',
      allocationRule: 'AccrualFirst',
      status: 'Outstanding',
    }],
    usedTransferIds: ['Release1'],
    usedAllocationIds: [],
    usedOriginationIds: [],
    usedAccrualIds: [],
    work: { remaining: '975', spent: '49', closureReserve: '16' },
    ...extra,
  };
}

function datum() {
  const hashes = {
    programHash: '11'.repeat(32),
    boundsHash: '22'.repeat(32),
    sourceHash: '33'.repeat(32),
    schemaHash: '44'.repeat(32),
    seedHash: '55'.repeat(32),
  };
  return datumFromPost(hashes, '66'.repeat(32), {
    financialPost: releasePost(),
    transportRevision: '1',
    transportRemaining: '0',
    post: { locked: '0', revision: '1' },
  });
}

test('F1 lifecycle plutus keeps obligations and transfer ids in packet order', () => {
  const built = datum();
  const life = built.plutus.fields[12];
  assert.equal(life.fields.length, 9);
  assert.equal(life.fields[3].list.length, 1);
  assert.equal(utf8(life.fields[3].list[0].fields[0]), 'duty');
  assert.equal(life.fields[4].list.length, 1);
  assert.equal(utf8(life.fields[4].list[0]), 'Release1');
  assert.equal(life.fields[5].list.length, 0);
  assert.equal(life.fields[6].list.length, 0);
  assert.equal(life.fields[7].list.length, 0);
  assert.deepEqual(built.fields.lifecycle.used_transfer_ids, ['Release1']);
  assert.deepEqual(built.fields.lifecycle.used_allocation_ids, []);
  assert.deepEqual(built.fields.lifecycle.used_origination_ids, []);
  assert.deepEqual(built.fields.lifecycle.used_accrual_ids, []);
  assert.equal(built.fields.lifecycle.obligations.length, 1);
});

function judgeFrame(patch = {}) {
  return {
    program_hash: '11'.repeat(32),
    bounds_hash: '22'.repeat(32),
    source_hash: '33'.repeat(32),
    schema_hash: '44'.repeat(32),
    seed_hash: '55'.repeat(32),
    domain_network: 'yaci-isolated-t1',
    domain_deployment: 't1-custody-revision03',
    instanceId: '66'.repeat(32),
    balances: { Escrow: 0, Borrower: 0, Lender: 100 },
    allowances: {
      Escrow: { remaining: 0, spent: 100 },
      Borrower: { remaining: 0, spent: 0 },
      Lender: { remaining: 0, spent: 0 },
    },
    obligations: [],
    work: { remaining: 975, spent: 49, closureReserve: 16 },
    usedTransferIds: ['Release1'],
    usedAllocationIds: [],
    usedOriginationIds: [],
    usedAccrualIds: [],
    transport_revision: 1,
    transport_remaining: 0,
    ordinary_locked: 0,
    ordinary_revision: 1,
    headQuantity: 1,
    continuingCash: 0,
    payoutCash: 100,
    phase: null,
    nonce: null,
    confirmed: false,
    ...patch,
  };
}

test('F2 judge binds the loaded 173 post and the redeemer transfer id', () => {
  const matched = judgeEconomicAction(judgeFrame(), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'Release1', amount: 100 },
  });
  assert.equal(matched.code, 'ECONOMIC_ACTION_MATCHED');
  assert.equal(matched.recipient, 'Lender');
  assert.equal(matched.transferId, 'Release1');
  assert.equal(matched.packetLoaded, true);
  assert.equal(matched.confirmed, false);
  const otherLegal = judgeEconomicAction(judgeFrame({ usedTransferIds: ['ReleaseLive'] }), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'ReleaseLive', amount: 100 },
  });
  assert.equal(otherLegal.code, 'ECONOMIC_ACTION_MATCHED');
  assert.equal(otherLegal.transferId, 'ReleaseLive');
  const mutated = judgeEconomicAction(judgeFrame({ usedTransferIds: ['ReleaseLive'] }), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'Release1', amount: 100 },
  });
  assert.equal(mutated.code, 'ECONOMIC_ACTION_REFUSED');
  const foreign = judgeEconomicAction(judgeFrame(), 'release', {
    redeemer: { recipient: 'Stranger', transferId: 'Release1', amount: 100 },
  });
  assert.ok(foreign.mismatches.includes('recipient'));
  const amount = judgeEconomicAction(judgeFrame({ payoutCash: 99 }), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'Release1', amount: 99 },
  });
  assert.ok(amount.mismatches.includes('amount'));
  const accrual = judgeEconomicAction(judgeFrame({ usedAccrualIds: ['A'] }), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'Release1', amount: 100 },
  });
  assert.ok(accrual.mismatches.includes('usedAccrualIds'));
  const duty = judgeEconomicAction(judgeFrame({ obligations: [{ id: 'duty' }] }), 'release', {
    redeemer: { recipient: 'Lender', transferId: 'Release1', amount: 100 },
  });
  assert.ok(duty.mismatches.includes('obligations'));
});

function sameTxArgv(mintUnits = true) {
  const tx = '11'.repeat(32);
  const argv = [
    'conway', 'transaction', 'build-raw',
    '--tx-in', `${tx}#1`,
    '--tx-in-script-file', 'unit-head.script',
    '--tx-in-redeemer-file', 'unit-redeemer.json',
    '--tx-in-execution-units', '(800000,400000)',
    '--tx-in', `${tx}#0`,
    '--tx-in-collateral', `${'22'.repeat(32)}#4`,
    '--read-only-tx-in-reference', `${'33'.repeat(32)}#0`,
    '--fee', '200000',
    '--protocol-params-file', PROTOCOL,
  ];
  if (mintUnits) argv.push('--mint-execution-units', '(100,200)');
  return {
    tx,
    fee: `${tx}#0`,
    head: `${tx}#1`,
    collateral: `${'22'.repeat(32)}#4`,
    foreign: `${'33'.repeat(32)}#0`,
    argv,
  };
}

test('F4 same txid fee index 0 and head index 1 bind spend redeemer 1', () => {
  const spec = sameTxArgv();
  const envelope = JSON.parse(encodeUnitConwayEnvelope(spec.argv));
  const read = readConwayEnvelope(envelope.cborHex);
  const spend = read.redeemers.find((row) => row.tag === 0);
  const mint = read.redeemers.find((row) => row.tag === 1);
  assert.equal(spend.index, 1);
  assert.equal(mint.index, 0);
  assert.equal(spend.index === 0, false);
});

test('F3 needed reference language enters calc_script_data_hash for that cbor', () => {
  const spec = sameTxArgv(false);
  const envelope = JSON.parse(encodeUnitConwayEnvelope(spec.argv));
  const read = readConwayEnvelope(envelope.cborHex);
  assert.equal(read.redeemers.some((row) => row.tag === 1), false);
  assert.equal(read.redeemers.find((row) => row.tag === 0).index, 1);
  const scriptHash = 'ab'.repeat(28);
  const context = {
    spentOutputs: [{ pointer: 1, paymentScriptHash: scriptHash, ref: spec.head }],
    spendingInputs: [{ ref: spec.head }],
    referenceUtxos: [
      { scriptHash, language: 'PlutusV3', ref: spec.head },
      { scriptHash: 'cd'.repeat(28), language: 'PlutusV1', ref: spec.foreign },
    ],
    referenceInputs: [spec.foreign],
  };
  const selected = selectNeededScriptLanguage({ cborHex: envelope.cborHex, ...context });
  assert.equal(selected.ok, true, JSON.stringify(selected));
  assert.deepEqual(selected.languages, ['PlutusV3']);
  const hashed = recomputeScriptDataHash(envelope.cborHex, { protocolPath: PROTOCOL, ...context });
  assert.equal(hashed.neededLanguageSource, 'selectNeededScriptLanguage');
  assert.deepEqual(hashed.neededLanguages, ['PlutusV3']);
  assert.equal(hashed.code, 'SCRIPT_DATA_HASH_RECOMPUTED');
  const cml = requireCml(CML_PATH);
  const tx = cml.Transaction.from_cbor_bytes(Buffer.from(envelope.cborHex, 'hex'));
  const witness = tx.witness_set();
  assert.equal(witness.plutus_v1_scripts(), undefined);
  assert.equal(witness.plutus_v2_scripts(), undefined);
  assert.equal(witness.plutus_v3_scripts(), undefined);
  const langs = cml.LanguageList.new();
  langs.add(cml.Language.PlutusV3);
  const models = JSON.parse(readFileSync(PROTOCOL, 'utf8')).costModels;
  const numeric = {};
  if (Array.isArray(models.PlutusV1)) numeric['0'] = models.PlutusV1;
  if (Array.isArray(models.PlutusV2)) numeric['1'] = models.PlutusV2;
  if (Array.isArray(models.PlutusV3)) numeric['2'] = models.PlutusV3;
  const expected = cml.calc_script_data_hash(
    tx.witness_set().redeemers(),
    tx.witness_set().plutus_datums() || cml.PlutusDataList.new(),
    cml.CostModels.from_json(JSON.stringify(numeric)),
    langs,
  ).to_hex();
  assert.equal(hashed.recomputedHex, expected);
  const mismatched = selectNeededScriptLanguage({
    cborHex: envelope.cborHex,
    spentOutputs: [{ pointer: 1, paymentScriptHash: scriptHash, ref: spec.head }],
    spendingInputs: [{ ref: spec.head }],
    referenceUtxos: [{ scriptHash, language: 'PlutusV3', ref: spec.foreign }],
    referenceInputs: [],
  });
  assert.equal(mismatched.ok, false);
  assert.equal(mismatched.code, 'NEEDED_SCRIPT_UNRESOLVED');
  const refused = recomputeScriptDataHash(envelope.cborHex, {
    protocolPath: PROTOCOL,
    spentOutputs: mismatched.needed,
    spendingInputs: [{ ref: spec.head }],
    referenceUtxos: [{ scriptHash, language: 'PlutusV3', ref: spec.foreign }],
  });
  assert.equal(refused.recomputedHex, null);
  assert.notEqual(refused.neededLanguageSource, 'witness.languages');
  const unused = cml.LanguageList.new();
  unused.add(cml.Language.PlutusV1);
  unused.add(cml.Language.PlutusV3);
  const foreignHash = cml.calc_script_data_hash(
    tx.witness_set().redeemers(),
    tx.witness_set().plutus_datums() || cml.PlutusDataList.new(),
    cml.CostModels.from_json(JSON.stringify(numeric)),
    unused,
  ).to_hex();
  assert.notEqual(hashed.recomputedHex, foreignHash);
});

function unitKeys() {
  const cml = requireCml(CML_PATH);
  const ordinary = cml.PrivateKey.generate_ed25519();
  const collateral = cml.PrivateKey.generate_ed25519();
  return { cml, ordinary, collateral };
}

function witnessHex(cml, bodyHex, keys) {
  const tx = cml.Transaction.from_cbor_bytes(Buffer.from(bodyHex, 'hex'));
  const hash = cml.hash_transaction(tx.body());
  const list = cml.VkeywitnessList.new();
  for (const key of keys) list.add(cml.make_vkey_witness(hash, key));
  const set = cml.TransactionWitnessSet.new();
  set.set_vkeywitnesses(list);
  return Buffer.from(set.to_cbor_bytes()).toString('hex');
}

test('F5 empty required signers do not cover a missing collateral key', () => {
  const spec = sameTxArgv();
  const envelope = JSON.parse(encodeUnitConwayEnvelope(spec.argv));
  const { cml, ordinary, collateral } = unitKeys();
  const ordinaryHash = ordinary.to_public().hash().to_hex();
  const collateralHash = collateral.to_public().hash().to_hex();
  const partial = witnessHex(cml, envelope.cborHex, [ordinary]);
  const missing = verifyExactBodyWitness(envelope.cborHex, partial, {
    ordinaryKeyHashes: [ordinaryHash],
    collateralKeyHashes: [collateralHash],
    inputsResolved: true,
    unitLabel: 'UNIT',
  });
  assert.equal(missing.signatureCheckRan, true);
  assert.equal(missing.verifiedCount, 1);
  assert.equal(missing.signatureVerified, false);
  assert.equal(missing.unitFlagPromoted, false);
  const full = witnessHex(cml, envelope.cborHex, [ordinary, collateral]);
  const covered = verifyExactBodyWitness(envelope.cborHex, full, {
    ordinaryKeyHashes: [ordinaryHash],
    collateralKeyHashes: [collateralHash],
    inputsResolved: true,
    unitLabel: 'UNIT',
  });
  assert.equal(covered.signatureVerified, true);
  assert.equal(covered.confirmed, false);
  assert.equal(covered.ledgerExecution, false);
  const golden = JSON.parse(readFileSync(GOLDEN, 'utf8'));
  const goldenTx = cml.Transaction.from_cbor_bytes(Buffer.from(golden.cborHex, 'hex'));
  const goldenWitness = Buffer.from(goldenTx.witness_set().to_cbor_bytes()).toString('hex');
  const checked = verifyExactBodyWitness(golden.cborHex, goldenWitness);
  assert.equal(checked.signatureCheckRan, true);
  assert.equal(checked.verifiedCount, 2);
  assert.equal(checked.bodyBound, true);
  assert.match(checked.bodyHashHex, /^[0-9a-f]{64}$/);
  assert.equal(checked.ledgerExecution, false);
});

test('F6 race submits release and cancel before poll and keeps refund separate', async () => {
  const dir = join(ROOT, 'checks', 'race-201');
  mkdirSync(dir, { recursive: true });
  const logPath = unusedPath(join(dir, 'order.log'));
  writeFileSync(logPath, '');
  const releaseBody = join(dir, 'release.body.json');
  const cancelBody = join(dir, 'cancel.body.json');
  const refundBody = join(dir, 'refund.body.json');
  writeFileSync(releaseBody, '{"cborHex":"01"}\n');
  writeFileSync(cancelBody, '{"cborHex":"02"}\n');
  writeFileSync(refundBody, '{"cborHex":"03"}\n');
  const keyPath = join(dir, 'unit.skey');
  writeFileSync(keyPath, '{"type":"PaymentSigningKeyShelley_ed25519","description":"UNIT key file, not a native witness","cborHex":"5820aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}\n');
  chmodSync(keyPath, 0o600);
  const bin = join(dir, 'stub-cli.mjs');
  writeFileSync(bin, `#!/usr/bin/env node
import { appendFileSync } from 'node:fs';
const argv = process.argv.slice(2);
const log = ${JSON.stringify(logPath)};
const kind = argv.includes('witness') ? 'sign' : argv.includes('assemble') ? 'assemble' : argv.includes('submit') ? 'submit' : argv.includes('tip') ? 'poll' : 'other';
const { readFileSync } = await import('node:fs');
const priorSubmits = readFileSync(log, 'utf8').split('\\n').filter((line) => line === 'submit').length;
if (kind === 'submit' && priorSubmits >= 2) process.stdout.write('BadInputsUTxO\\n');
appendFileSync(log, kind + '\\n');
if (argv.includes('--out-file')) {
  const out = argv[argv.indexOf('--out-file') + 1];
  const { writeFileSync } = await import('node:fs');
  writeFileSync(out, '{"type":"unit"}\\n');
}
process.exit(0);
`);
  chmodSync(bin, 0o755);
  const tx = '44'.repeat(32);
  const head = { txId: tx, index: 1 };
  const raced = await runOwnedRace({
    signBin: bin,
    submitBin: bin,
    pollBin: bin,
    logPath,
    socket: 'unit-socket',
    networkMagic: 764,
    head,
    left: {
      actor: 'Lender',
      resource: 'release-fee-root',
      fee: { txId: `${'55'.repeat(32)}`, index: 0 },
      collateral: { txId: `${'66'.repeat(32)}`, index: 0 },
    },
    right: {
      actor: 'Borrower',
      resource: 'cancel-fee-root',
      fee: { txId: `${'77'.repeat(32)}`, index: 0 },
      collateral: { txId: `${'88'.repeat(32)}`, index: 0 },
    },
    leftBodyFile: releaseBody,
    rightBodyFile: cancelBody,
    signingKeyPath: keyPath,
    overlap: 'release-cancel',
    cashQuantity: 100,
  });
  assert.equal(raced.confirmed, false);
  assert.equal(raced.submitsBeforeConfirms, true);
  assert.equal(raced.submitCount, 2);
  assert.equal(raced.replay.expectedClass, 'BadInputsUTxO');
  assert.equal(raced.ledgerSubmitted, false);
  const order = readFileSync(logPath, 'utf8').trim().split('\n');
  const firstPoll = order.indexOf('poll');
  const submits = order.map((line, index) => (line === 'submit' ? index : -1)).filter((index) => index >= 0);
  assert.equal(submits.length >= 2, true);
  assert.equal(firstPoll > submits[1], true);
  assert.equal(order.includes('BadInputsUTxO') || readFileSync(logPath, 'utf8').includes('BadInputsUTxO') || raced.replay.observed === true, true);
  assert.equal(createHash('sha256').update(readFileSync(refundBody)).digest('hex') !== createHash('sha256').update(readFileSync(cancelBody)).digest('hex'), true);
});

test('body pointer uses sorted ordinary inputs and excludes collateral', () => {
  const tx = '11'.repeat(32);
  const collateral = `${'22'.repeat(32)}#4`;
  const context = {
    ordinaryInputs: [`${tx}#1`, `${tx}#0`],
    scriptRef: `${tx}#1`,
    mintPolicyCount: 0,
    collateralRef: collateral,
  };
  const sorted = sortedOrdinaryRefs(context.ordinaryInputs);
  assert.deepEqual(sorted, [`${tx}#0`, `${tx}#1`]);
  assert.equal(sorted.includes(collateral), false);
  assert.equal(redeemerPointerAccepts({ tag: 0, index: 1 }, context), true);
  assert.equal(redeemerPointerAccepts({ tag: 0, index: 0 }, context), false);
  assert.equal(redeemerPointerAccepts({ tag: 1, index: 0 }, { mintPolicyCount: 1 }), true);
  assert.equal(redeemerPointerAccepts({ tag: 1, index: 1 }, { mintPolicyCount: 1 }), false);
});

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');
const SPEND_SCRIPT = join(TASK, 't1-native-feasibility-23/revision02/design-review-b/logs/financial-head-applied.script.json');
const PAYMENT_VKEY = `5820${'0123456789abcdeffedcba9876543210'.repeat(2)}`;
const PAYOUT_VKEY = `5820${'ffeeddccbbaa99887766554433221100'.repeat(2)}`;
const BORROWER_VKEY = `5820${'102030405060708090a0b0c0d0e0f001'.repeat(2)}`;
const FIN = [
  ['fin-cash-seed', 'a1', '45000000000000000'],
  ['fin-head', 'a2', '900000000000002'],
  ['fin-mint-fee', 'a3', '900000000000003'],
  ['fin-mint-collateral', 'a4', '900000000000004'],
  ['fin-head-fee', 'a5', '900000000000005'],
  ['fin-head-collateral', 'a6', '900000000000006'],
  ['fin-release-fee', 'a7', '900000000000007'],
  ['fin-release-collateral', 'a8', '900000000000008'],
  ['fin-refund-fee', 'a9', '900000000000009'],
  ['fin-refund-collateral', 'aa', '900000000000010'],
  ['fin-cancel-fee', 'ab', '900000000000011'],
  ['fin-cancel-collateral', 'ac', '900000000000012'],
];

function unusedPath(path) {
  if (!statSync(path, { throwIfNoEntry: false })) return path;
  const slash = path.lastIndexOf('/');
  const dirName = slash >= 0 ? path.slice(0, slash) : '.';
  const base = slash >= 0 ? path.slice(slash + 1) : path;
  const dot = base.lastIndexOf('.');
  const stem = dot > 0 ? base.slice(0, dot) : base;
  const ext = dot > 0 ? base.slice(dot) : '';
  let n = 1;
  let next = join(dirName, `${stem}-${n}${ext}`);
  while (statSync(next, { throwIfNoEntry: false })) {
    n += 1;
    next = join(dirName, `${stem}-${n}${ext}`);
  }
  return next;
}

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (statSync(path, { throwIfNoEntry: false })) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function nativeReviews(dir) {
  mkdirSync(dir, { recursive: true });
  const sourcePath = join(dir, 'source.txt');
  const sourceText = 'financial-caller-201\n';
  writeFileSync(sourcePath, sourceText);
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const body = JSON.stringify({ sourceHash, files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }] });
  writeFileSync(join(dir, 'candidate.json'), body);
  const manifestHash = createHash('sha256').update(body).digest('hex');
  writeFileSync(join(dir, 'authority.json'), JSON.stringify({ manifestSha256: manifestHash }));
  const review = (reviewer) => ({
    verdict: 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION',
    evidenceClass: 'reviewed-implementation',
    reviewClass: 'native-implementation',
    reviewer,
    candidate: manifestHash,
    sourceHash,
    selfApproval: false,
    unitGateProbe: true,
  });
  writeFileSync(join(dir, 'a.json'), JSON.stringify(review('probe-a')));
  writeFileSync(join(dir, 'b.json'), JSON.stringify(review('probe-b')));
  return {
    reviewA: join(dir, 'a.json'),
    reviewB: join(dir, 'b.json'),
    candidate: join(dir, 'candidate.json'),
    reviewAuthority: join(dir, 'authority.json'),
  };
}

test('F6 caller binds release and cancel intervals through runNativeCaller', async () => {
  const ownIntegrate = new URL('../src/native-integrate.mjs', import.meta.url).pathname;
  const dir = join(ROOT, 'checks', 'caller-201');
  mkdirSync(dir, { recursive: true });
  const home = RUN_DIR;
  mkdirSync(home, { recursive: true });
  const reviews = nativeReviews(join(dir, 'reviews'));
  const txFile = join(home, 'unit-body.tx');
  writeFileSync(txFile, 'unit-body-not-a-ledger-tx\n');
  const prepared = produceOwnedRuntime({
    executeNow: false,
    financialFrame: true,
    authority: reviews,
    ownedHome: home,
    runDir: RUN_DIR,
    runId: 't1201',
    nodePort: 14141,
    submitApiPort: 14142,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    supplyLovelace: 9000000,
    txFile,
    unitSpawn: null,
    referenceScriptPath: null,
  });
  const socketPath = prepared.schema71.paths.socketPath;
  assert.equal(socketPath.endsWith(`/${SOCKET_BASENAME}`), true);
  assert.equal(socketPath.startsWith(`${RUN_DIR}/`), true);
  assert.equal(Buffer.byteLength(socketPath) < 107, true, socketPath);
  assert.equal(prepared.schema71.inputsReady, true, JSON.stringify(prepared.schema71.blockers));
  const paths = prepared.schema71.paths;
  const stub = join(dir, 'cli-stub.mjs');
  const cliLog = unusedPath(join(ROOT, 'logs', 'caller-201-cli.log'));
  writeFileSync(cliLog, '');
  const datum = initialHeadDatum({
    programHash: '11'.repeat(28),
    boundsHash: '22'.repeat(28),
    sourceHash: '33'.repeat(28),
    schemaHash: '44'.repeat(28),
    seedHash: '55'.repeat(28),
  }, '66'.repeat(28)).plutus;
  const datumFile = join(dir, 'datum.json');
  writeFileSync(datumFile, JSON.stringify(datum));
  writeFileSync(stub, `#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { appendFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs';
import { encodeUnitConwayEnvelope, encodeUnitWitnessCborHex } from ${JSON.stringify(ownIntegrate)};
const argv = process.argv.slice(2);
appendFileSync(${JSON.stringify(cliLog)}, JSON.stringify(argv) + '\\n');
const flag = (name) => { const index = argv.indexOf(name); return index >= 0 ? argv[index + 1] : ''; };
const protocolPath = flag('--protocol-params-file') || ${JSON.stringify(PROTOCOL)};
const protocol = JSON.parse(readFileSync(protocolPath, 'utf8'));
if (argv[0] === 'address' && argv[1] === 'build') {
  const file = flag('--payment-verification-key-file') || flag('--payment-script-file') || 'none';
  process.stdout.write('addr_test1' + createHash('sha256').update(file).digest('hex').slice(0, 20) + '\\n');
  process.exit(0);
}
if (argv[0] === 'query' && argv[1] === 'tip') {
  process.stdout.write('slotLengthMs=1000\\nlucidPosix=1700000001000\\ncliPosix=1700000001000\\nscriptContextLower=1700000001000\\ntipSlot=100\\nevidenceClass=unit-fixture\\n');
  process.exit(0);
}
if (argv[0] === 'query' && argv[1] === 'utxo') {
  const ref = flag('--tx-in');
  const effectFile = ${JSON.stringify(join(dir, 'effect-utxo.json'))};
  if (existsSync(effectFile)) {
    const map = JSON.parse(readFileSync(effectFile, 'utf8'));
    if (map[ref]) { process.stdout.write(JSON.stringify({ [ref]: map[ref] }) + '\\n'); process.exit(0); }
  }
  const posted = JSON.parse(readFileSync(${JSON.stringify(datumFile)}, 'utf8'));
  process.stdout.write(JSON.stringify({ [ref]: { address: 'addr_test1readback', value: { lovelace: '900000000000002' }, inlineDatum: posted } }) + '\\n');
  process.exit(0);
}
if (argv.includes('calculate-min-required-utxo')) { process.stdout.write('Coin ' + String(protocol.utxoCostPerByte) + '\\n'); process.exit(0); }
if (argv.includes('calculate-min-fee')) { process.stdout.write(JSON.stringify({ fee: protocol.txFeeFixed }) + '\\n'); process.exit(0); }
if (argv.includes('calculate-plutus-script-cost')) {
  process.stdout.write(JSON.stringify([{ scriptHash: '11'.repeat(28), executionUnits: { memory: 800000, steps: 400000 }, lovelaceCost: protocol.txFeePerByte }]) + '\\n');
  process.exit(0);
}
if (argv.includes('build-raw')) {
  const out = flag('--out-file');
  writeFileSync(out, encodeUnitConwayEnvelope(argv));
  writeFileSync(out + '.argv.json', JSON.stringify(argv));
  process.exit(0);
}
if (argv.includes('txid')) {
  const file = flag('--tx-body-file') || flag('--tx-file');
  const body = JSON.parse(readFileSync(file, 'utf8'));
  process.stdout.write(createHash('sha256').update(body.cborHex).digest('hex') + '\\n');
  process.exit(0);
}
if (argv[0] === 'debug' && argv.includes('view')) {
  const saved = JSON.parse(readFileSync(flag('--tx-body-file') + '.argv.json', 'utf8'));
  const refs = [];
  for (let index = 0; index < saved.length; index += 1) {
    if (saved[index] === '--tx-in' || saved[index] === '--tx-in-collateral') refs.push(saved[index + 1]);
  }
  process.stdout.write(refs.join('\\n') + '\\n');
  process.exit(0);
}
if (argv.includes('witness') || argv.includes('assemble')) {
  writeFileSync(flag('--out-file'), JSON.stringify({ type: 'TxWitness ConwayEra', description: 'unit witness', cborHex: encodeUnitWitnessCborHex() }));
  process.exit(0);
}
if (argv.includes('submit')) process.exit(0);
process.exit(0);
`);
  chmodSync(stub, 0o755);
  const keyPath = join(dir, 'role-payment.skey');
  writeFileSync(keyPath, '{"type":"PaymentSigningKeyShelley_ed25519","description":"UNIT key file, not a native witness","cborHex":"58200123456789abcdeffedcba98765432100123456789abcdeffedcba9876543210"}\n');
  chmodSync(keyPath, 0o600);
  const raceLog = join(dir, 'race-order.log');
  writeFileSync(raceLog, '');
  const genesis = join(dir, 'shelley-genesis.json');
  writeFileSync(genesis, JSON.stringify({ slotLength: 1, systemStart: '2026-01-01T00:00:00Z' }));
  const socket = join(dir, 'time.sock');
  writeFileSync(socket, '');
  const report = await runNativeCaller({
    authority: reviews,
    cliPath: stub,
    evidenceRoot: dir,
    sourcePath: SOURCE,
    networkMagic: 764,
    paymentVkeyCbor: PAYMENT_VKEY,
    payoutVkeyCbor: PAYOUT_VKEY,
    lenderVkeyCbor: PAYOUT_VKEY,
    borrowerVkeyCbor: BORROWER_VKEY,
    spendScriptPath: SPEND_SCRIPT,
    detailedData: datum,
    witnessKeyPath: keyPath,
    financialBody: true,
    buildBody: true,
    receiptRoot: join(ROOT, 'receipts'),
    keepReceipt: false,
    ownedProducer: {
      executeNow: false,
      financialFrame: true,
      authority: reviews,
      ownedHome: home,
      runDir: RUN_DIR,
      runId: 't1201',
      nodePort: 14141,
      submitApiPort: 14142,
      magic: 764,
      magicSource: 'caller-chosen-for-genesis',
      supplyLovelace: 9000000,
      txFile,
      unitSpawn: (argv, step) => {
        const writeKey = (path, body) => {
          mkdirSync(dirname(path), { recursive: true });
          writeFileSync(path, JSON.stringify({ type: 'PaymentVerificationKeyShelley_ed25519', cborHex: `5820${body}` }));
        };
        if (step.id.startsWith('role-keygen')) {
          const mark = step.id.endsWith('issuer') ? 'a1' : step.id.endsWith('lender') ? 'b2' : 'c4';
          writeKey(argv[argv.indexOf('--verification-key-file') + 1], mark.repeat(32));
          writeKey(argv[argv.indexOf('--signing-key-file') + 1], mark.repeat(32));
        }
        if (step.id === 'legacy-genesis') {
          const bodies = ['0123456789abcdef'.repeat(4), '00112233445566778899aabbccddeeff'.repeat(2), 'fedcba98765432100123456789abcdef'.repeat(2)];
          paths.utxoVkeys.forEach((path, index) => writeKey(path, bodies[index]));
          paths.utxoSkeys.forEach((path, index) => writeKey(path, bodies[index]));
          writeFileSync(paths.shelleyWork, JSON.stringify({ slotLength: 1, epochLength: 3600, networkMagic: 764, systemStart: '2026-01-01T00:00:00Z', minFeeA: 44, minFeeB: 155381, maxLovelaceSupply: 45000000000000000, genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } } }));
          writeFileSync(paths.byronWork, JSON.stringify({ startTime: 0 }));
          writeFileSync(paths.alonzoWork, JSON.stringify({ seed: 1 }));
          writeFileSync(paths.conwayWork, JSON.stringify({ seed: 1 }));
        }
        if (step.id.startsWith('initial-addr') || step.id.startsWith('role-address')) {
          const out = argv[argv.indexOf('--out-file') + 1];
          mkdirSync(dirname(out), { recursive: true });
          writeFileSync(out, `addr_test1fixture${step.id}\n`);
        }
        if (step.id === 'start-node') bindUnixSocket(paths.socketPath);
        if (step.id === 'calculate-min-fee') return { status: 0, stdout: '{"fee":123456}\n' };
        if (step.id === 'calculate-plutus-script-cost') {
          return { status: 0, stdout: `${JSON.stringify([{ scriptHash: '11'.repeat(28), executionUnits: { memory: 800000, steps: 400000 }, lovelaceCost: 44 }])}\n` };
        }
        if (step.id === 'query-tip') return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ce'.repeat(32), block: 3 }) };
        if (step.id === 'query-era-history') return { status: 0, stdout: JSON.stringify({ type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' }) };
        if (step.id === 'query-funder-utxo' || step.id.startsWith('funding-confirm')) return { status: 0, stdout: `${'0123456789abcdef'.repeat(4)} 0 9000000\n` };
        const slot = FIN.find((item) => item[0] === step.id);
        if (slot) return { status: 0, stdout: `${slot[1].repeat(32)} 0 ${slot[2]}\n` };
        return { status: 0, stdout: '' };
      },
    },
    time: { cliPath: stub, socket, execute: true, genesisPath: genesis, magic: 764 },
    race: {
      signBin: stub,
      submitBin: stub,
      pollBin: stub,
      logPath: raceLog,
      socket,
      signingKeyPath: keyPath,
      leftActor: 'Lender',
      rightActor: 'Borrower',
      leftResource: 'release-fee-root',
      rightResource: 'cancel-fee-root',
    },
    readback: { cliPath: stub, socket, magic: 764 },
  });
  const stages = report.stages;
  assert.equal(stages.spend?.code, 'SPEND_BODIES_ENCODED', JSON.stringify({
    spend: stages.spend?.code,
    release: stages.spend?.release?.code,
    posts: stages.posts?.code,
    body: stages.body?.code,
    detail: stages.spend?.release?.detail || stages.body?.detail || null,
  }));
  assert.equal(stages.spend.release.invalidBefore, 1000);
  assert.equal(stages.spend.release.invalidHereafter, 1099);
  assert.equal(stages.spend.cancel.invalidBefore, 1050);
  assert.equal(stages.spend.cancel.invalidHereafter, 1051);
  assert.equal(stages.spend.refund.invalidBefore, 1100);
  assert.equal(stages.spend.refund.invalidHereafter, 1100);
  assert.equal(stages.spend.release.economicAction.code, 'ECONOMIC_ACTION_MATCHED');
  assert.equal(stages.spend.release.economicAction.recipient, 'Lender');
  assert.equal(stages.spend.release.economicAction.transferId, 'Release1');
  assert.equal(stages.spend.release.redeemer.transferId, 'Release1');
  assert.equal(stages.spend.release.mintPolicyCount, 0);
  assert.equal(stages.spend.release.ordinaryInputs.includes(stages.spend.release.collateralRef), false);
  assert.equal(stages.mint.mintPolicyCount, 1);
  assert.equal(stages.rebuild.matchedRedeemer.tag, 1);
  assert.equal(stages.rebuild.matchedRedeemer.index, 0);
  assert.equal(stages.race.partnerCode, 'RACE_RELEASE_CANCEL');
  assert.equal(stages.race.rightKind, 'cancel');
  assert.equal(stages.race.refundSeparate, true);
  assert.equal(stages.race.submitCount, 2);
  assert.equal(stages.race.submitsBeforeConfirms, true);
  assert.equal(stages.race.confirmed, false);
  assert.equal(stages.race.ledgerSubmitted, false);
  assert.notEqual(stages.spend.release.bodyFile, stages.spend.cancel.bodyFile);
  assert.notEqual(stages.spend.refund.bodyFile, stages.spend.cancel.bodyFile);
  assert.equal(stages.witnessCount.contextResolved, false);
  assert.equal(stages.witnessCount.signatureVerified, false);
  assert.equal(stages.witnessCount.unitOnly, true);
  assert.equal(stages.readback.byAction.release.judgeCode, 'ECONOMIC_ACTION_MATCHED');
  assert.equal(stages.readback.byAction.release.confirmed, false);
  assert.equal(stages.readback.byAction.release.providerQueried, false);
  assert.equal(stages.readback.economicAction.confirmed, false);
  assert.equal(report.confirmed, false);
  assert.equal(report.ledgerExecution, false);
  assert.equal(report.nativePins, false);
  assert.equal(report.unitOnly, true);
  rmSync(home, { recursive: true, force: true });
});
