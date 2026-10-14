import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash, generateKeyPairSync, randomBytes, sign } from 'node:crypto';
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { bindHelperReceipt, initialHeadDatum, runNativeCaller } from '../src/native-caller.mjs';
import { judgeEconomicAction } from '../src/native-financial.mjs';
import { encodeUnitWitnessCborHex, readConwayEnvelope, selectNeededScriptLanguage, verifyExactBodyWitness } from '../src/native-integrate.mjs';
import { produceOwnedRuntime } from '../src/native-runtime.mjs';
import {
  bindApprovedNodeConfig,
  materializeApprovedBootstrapSpecs,
  noteByronReplaced,
  parseQueryUtxoText,
  planApprovedBootstrap,
  producerRoleFrame,
} from '../src/native-bootstrap.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const OWN = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-f207';
const PACKET = join(OWN, 'checks', 'consumer');
const CHECKS = join(OWN, 'checks', 'consumer-tmp');
const PROTOCOL = join(TASK, 'tooling/runtime/actual-protocol-parameters.json');
const SPEND_SCRIPT = join(TASK, 't1-native-feasibility-23/revision02/design-review-b/logs/financial-head-applied.script.json');
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');
const AIKEN = '/home/charl/.aiken/bin/aiken';
const OWNED_RECEIPTS = join(TASK, 'tooling/transaction-receipts/t1-f207-own-222');
const PAYMENT_VKEY = `5820${'0123456789abcdeffedcba9876543210'.repeat(2)}`;
const PAYOUT_VKEY = `5820${'ffeeddccbbaa99887766554433221100'.repeat(2)}`;
const BORROWER_VKEY = `5820${'102030405060708090a0b0c0d0e0f001'.repeat(2)}`;

mkdirSync(CHECKS, { recursive: true });
mkdirSync(join(PACKET, 'checks'), { recursive: true });
mkdirSync(join(PACKET, 'logs'), { recursive: true });

function nativeReviews(dir) {
  mkdirSync(dir, { recursive: true });
  const sourcePath = join(dir, 'source.txt');
  const sourceText = 'financial-consumer-129\n';
  writeFileSync(sourcePath, sourceText);
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const body = JSON.stringify({
    sourceHash,
    files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }],
  });
  writeFileSync(join(dir, 'manifest.json'), body);
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
    manifest: join(dir, 'manifest.json'),
    reviewAuthority: join(dir, 'authority.json'),
  };
}

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (statSync(path, { throwIfNoEntry: false })) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function writeStub(path, units = { memory: 800000, steps: 400000 }) {
  const text = `#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { appendFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs';
import { encodeUnitConwayEnvelope, encodeUnitWitnessCborHex } from ${JSON.stringify(fileURLToPath(new URL('../src/native-integrate.mjs', import.meta.url)))};
const argv = process.argv.slice(2);
const log = process.env.M129_CLI_LOG;
if (log) appendFileSync(log, JSON.stringify(argv) + '\\n');
const flag = (name) => {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : '';
};
const protocolPath = flag('--protocol-params-file') || process.env.M129_PROTOCOL;
const protocol = protocolPath ? JSON.parse(readFileSync(protocolPath, 'utf8')) : {};
if (argv[0] === 'address' && argv[1] === 'build') {
  const file = flag('--payment-verification-key-file') || flag('--payment-script-file') || 'none';
  process.stdout.write('addr_test1' + createHash('sha256').update(file).digest('hex').slice(0, 20) + '\\n');
  process.exit(0);
}
if (argv[0] === 'query' && argv[1] === 'tip') {
  process.stdout.write('slotLengthMs=1000\\nlucidPosix=1700000001000\\ncliPosix=1700000001000\\nscriptContextLower=1700000001000\\ntipSlot=100\\nevidenceClass=unit-fixture\\n');
  process.exit(0);
}
if (argv[0] === 'query' && argv[1] === 'era-history') {
  process.stdout.write('era-history-text\\n');
  process.exit(0);
}
if (argv[0] === 'query' && argv[1] === 'utxo') {
  const ref = flag('--tx-in');
  const effectRoot = process.env.M170_EFFECT_ROOT;
  if (effectRoot && ref) {
    const effectFile = effectRoot.replace(/\\/$/, '') + '/effect-utxo.json';
    if (existsSync(effectFile)) {
      const map = JSON.parse(readFileSync(effectFile, 'utf8'));
      if (map[ref]) {
        process.stdout.write(JSON.stringify({ [ref]: map[ref] }) + '\\n');
        process.exit(0);
      }
    }
  }
  const datum = JSON.parse(readFileSync(process.env.M129_DATUM, 'utf8'));
  const body = {
    address: 'addr_test1readback',
    value: {
      lovelace: '900000000000002',
      [${JSON.stringify('aa'.repeat(28))}]: { '43617368': '100' },
      [${JSON.stringify('bb'.repeat(28))}]: { '48656164': '1' },
    },
    inlineDatum: datum,
    datum: null,
  };
  process.stdout.write(JSON.stringify({ [ref]: body }) + '\\n');
  process.exit(0);
}
if (argv.includes('calculate-min-required-utxo')) {
  process.stdout.write('Coin ' + String(protocol.utxoCostPerByte) + '\\n');
  process.exit(0);
}
if (argv.includes('calculate-min-fee')) {
  const bodyFile = flag('--tx-body-file');
  const marker = process.env.M129_DIVERGE_MARKER || '';
  const diverge = process.env.M129_DIVERGE_RECOUNT === '1' && String(bodyFile).includes('head-open.body.json');
  if (diverge && marker && existsSync(marker)) {
    process.stdout.write(JSON.stringify({ fee: Number(protocol.txFeeFixed) + 44 }) + '\\n');
    process.exit(0);
  }
  if (diverge && marker) writeFileSync(marker, 'seen\\n');
  process.stdout.write(JSON.stringify({ fee: protocol.txFeeFixed }) + '\\n');
  process.exit(0);
}
if (argv.includes('calculate-plutus-script-cost')) {
  process.stdout.write(JSON.stringify([{
    scriptHash: '11'.repeat(28),
    executionUnits: { memory: ${Number(units.memory)}, steps: ${Number(units.steps)} },
    lovelaceCost: protocol.txFeePerByte,
  }]) + '\\n');
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
  const hash = createHash('sha256').update(body.cborHex).digest('hex');
  if (argv.includes('--output-json')) process.stdout.write(JSON.stringify({ txid: hash }) + '\\n');
  else process.stdout.write(hash + '\\n');
  process.exit(0);
}
if (argv[0] === 'debug' && argv.includes('view')) {
  const file = flag('--tx-body-file');
  const saved = JSON.parse(readFileSync(file + '.argv.json', 'utf8'));
  const refs = [];
  for (let index = 0; index < saved.length; index += 1) {
    if (saved[index] === '--tx-in' || saved[index] === '--tx-in-collateral') refs.push(saved[index + 1]);
  }
  const shown = { era: 'Conway', inputs: refs, fee: 'unit', outputs: [] };
  process.stdout.write(JSON.stringify(shown) + '\\n' + refs.join('\\n') + '\\n');
  process.exit(0);
}
if (argv.includes('witness') || argv.includes('assemble')) {
  const out = flag('--out-file');
  writeFileSync(out, JSON.stringify({ type: 'TxWitness ConwayEra', description: 'unit witness structure, not a signature check', cborHex: encodeUnitWitnessCborHex() }));
  process.exit(0);
}
if (argv.includes('submit')) process.exit(0);
process.exit(0);
`;
  writeFileSync(path, text);
  chmodSync(path, 0o755);
}

function writeBin(path, script) {
  writeFileSync(path, script);
  chmodSync(path, 0o755);
}

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

function producerSpawn(paths) {
  return (argv, step) => {
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
      writeFileSync(paths.shelleyWork, JSON.stringify({
        slotLength: 1,
        epochLength: 3600,
        networkMagic: 764,
        systemStart: '2026-01-01T00:00:00Z',
        minFeeA: 44,
        minFeeB: 155381,
        maxLovelaceSupply: 45000000000000000,
        genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } },
      }));
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
    if (step.id === 'query-tip') return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ce'.repeat(32), block: 3 }) };
    if (step.id === 'query-era-history') return { status: 0, stdout: JSON.stringify({ type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' }) };
    if (step.id === 'query-funder-utxo' || step.id.startsWith('funding-confirm')) {
      return { status: 0, stdout: `${'0123456789abcdef'.repeat(4)} 0 9000000\n` };
    }
    const slot = FIN.find((item) => item[0] === step.id);
    if (slot) return { status: 0, stdout: `${slot[1].repeat(32)} 0 ${slot[2]}\n` };
    if (step.id === 'calculate-min-fee') return { status: 0, stdout: '{"fee":123456}\n' };
    if (step.id === 'calculate-plutus-script-cost') {
      return { status: 0, stdout: JSON.stringify([{ scriptHash: '11'.repeat(28), executionUnits: { memory: 500, steps: 64100 }, lovelaceCost: 44 }]) };
    }
    return { status: 0, stdout: '' };
  };
}

function owned(home, reviews, txFile, referenceScriptPath) {
  return {
    executeNow: false,
    financialFrame: true,
    authority: reviews,
    ownedHome: home,
    runId: 't1129',
    nodePort: 14041,
    submitApiPort: 14042,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    supplyLovelace: 9000000,
    txFile,
    unitSpawn: null,
    referenceScriptPath,
  };
}

const PRODUCER_HOME = {
  consumer: 'c0',
  'none-ref': 'c1',
  'with-ref': 'c2',
  'online-refused': 'c3',
  'online-unit': 'c4',
  'units-1-1': 'c5',
  'fee-diverge': 'c7',
};

async function runConsumer({ referenceScriptPath = null, label = 'consumer', costMode = null, costMagicSource = null, costMemory = 800000, costSteps = 400000 } = {}) {
  const root = mkdtempSync(join(CHECKS, `${label}-`));
  const home = join(OWN, PRODUCER_HOME[label] || 'c9');
  mkdirSync(home, { recursive: true });
  const reviews = nativeReviews(join(root, 'reviews'));
  const txFile = join(home, 'unit-body.tx');
  writeFileSync(txFile, 'unit-body-not-a-ledger-tx\n');
  const prepared = produceOwnedRuntime({ ...owned(home, reviews, txFile), financialFrame: true });
  assert.equal(prepared.schema71.inputsReady, true, JSON.stringify(prepared.schema71.blockers));
  const stub = join(root, 'cli-stub.mjs');
  writeStub(stub, { memory: costMemory, steps: costSteps });
  const datum = initialHeadDatum({
    programHash: '11'.repeat(28),
    boundsHash: '22'.repeat(28),
    sourceHash: '33'.repeat(28),
    schemaHash: '44'.repeat(28),
    seedHash: '55'.repeat(28),
  }, '66'.repeat(28)).plutus;
  const datumFile = join(root, 'datum.json');
  writeFileSync(datumFile, JSON.stringify(datum));
  const genesis = join(root, 'shelley-genesis.json');
  writeFileSync(genesis, JSON.stringify({ slotLength: 1, systemStart: '2026-01-01T00:00:00Z' }));
  const socket = join(root, 'time.sock');
  writeFileSync(socket, '');
  const keyPath = join(root, 'role-payment.skey');
  writeFileSync(keyPath, '{"type":"PaymentSigningKeyShelley_ed25519","description":"unit fixture","cborHex":"58200123456789abcdeffedcba98765432100123456789abcdeffedcba9876543210"}\n');
  chmodSync(keyPath, 0o600);
  const signBin = join(root, 'sign.sh');
  const submitBin = join(root, 'submit.sh');
  const pollBin = join(root, 'poll.sh');
  writeBin(signBin, '#!/bin/bash\nexit 0\n');
  writeBin(submitBin, '#!/bin/bash\nexit 0\n');
  writeBin(pollBin, '#!/bin/bash\nexit 0\n');
  const cliLog = join(PACKET, 'checks', `${label}-cli.log`);
  writeFileSync(cliLog, '');
  process.env.M129_CLI_LOG = cliLog;
  process.env.M129_DATUM = datumFile;
  process.env.M129_PROTOCOL = PROTOCOL;
  process.env.M170_EFFECT_ROOT = join(PACKET, 'checks', label);
  const report = await runNativeCaller({
    authority: reviews,
    cliPath: stub,
    evidenceRoot: join(PACKET, 'checks', label),
    networkMagic: 764,
    paymentVkeyCbor: PAYMENT_VKEY,
    payoutVkeyCbor: PAYOUT_VKEY,
    lenderVkeyCbor: PAYOUT_VKEY,
    borrowerVkeyCbor: BORROWER_VKEY,
    spendScriptPath: SPEND_SCRIPT,
    detailedData: datum,
    witnessKeyPath: keyPath,
    referenceScriptPath,
    costMode,
    costMagicSource,
    costSocket: socket,
    financialBody: true,
    buildBody: true,
    receiptRoot: OWNED_RECEIPTS,
    keepReceipt: true,
    receiptLabel: `c177-${label}-${randomBytes(4).toString('hex')}`,
    ownedProducer: {
      ...owned(home, reviews, txFile),
      unitSpawn: producerSpawn(prepared.schema71.paths),
    },
    time: {
      cliPath: stub,
      socket,
      execute: true,
      genesisPath: genesis,
      magic: 764,
    },
    sibling: {
      compilerBin: AIKEN,
      sourcePath: SOURCE,
      previousProgramHash: '00'.repeat(28),
    },
    race: {
      signBin,
      submitBin,
      pollBin,
      logPath: join(PACKET, 'checks', `${label}-race.log`),
      socket,
      signingKeyPath: keyPath,
      leftActor: '11'.repeat(28),
      rightActor: '22'.repeat(28),
      leftResource: 'release',
      rightResource: 'refund',
    },
    readback: {
      cliPath: stub,
      socket,
      magic: 764,
    },
  });
  delete process.env.M129_CLI_LOG;
  delete process.env.M129_DATUM;
  delete process.env.M129_PROTOCOL;
  delete process.env.M170_EFFECT_ROOT;
  rmSync(home, { recursive: true, force: true });
  return { report, cliLog, root };
}

test('producer role utxos drive cash, head, fee, rebuild, race, and readback', async () => {
  const { report, cliLog, root } = await runConsumer({ label: 'none-ref' });
  const stages = report.stages;
  assert.equal(stages.dispatch.code, 'PRODUCER_DISPATCHED', JSON.stringify(stages.dispatch.results?.map((item) => [item.id, item.code])));
  assert.equal(stages.financial.twelveInputFrameSatisfied, true);
  assert.equal(stages.inputs.roleSource, 'producer-dispatch');
  assert.equal(stages.inputs.twelveInputFrameSatisfied, true);
  assert.equal(stages.mint.code, 'UNSIGNED_BODY', JSON.stringify({ code: stages.mint.code, detail: stages.mint.detail }));
  assert.equal(stages.mint.kernelCash, '100');
  assert.equal(stages.mint.markerNotNativeFunding, true);
  assert.equal(stages.mint.ledgerFunding, false);
  assert.equal(stages.body.code, 'UNSIGNED_BODY', JSON.stringify({ code: stages.body.code, detail: stages.body.detail }));
  assert.equal(stages.body.datumFieldCount, 13);
  assert.equal(stages.body.detailedConstructor, true);
  assert.equal(stages.body.holding, `${stages.mint.txid}#${stages.mint.headOutputIndex}`);
  assert.equal(stages.body.headOutputIndex, 0);
  assert.equal(stages.body.changeOutputIndex, 1);
  assert.equal(stages.body.outputCount, 2);
  assert.match(String(stages.body.changeLovelace), /^[1-9][0-9]*$/);
  assert.equal(stages.rebuild.applied, true, JSON.stringify({ code: stages.rebuild.code, hash: stages.rebuild.scriptDataHash }));
  assert.equal(stages.rebuild.scriptDataHashSource, 'conway-body-key-11');
  assert.equal(stages.rebuild.scriptDataHashInvented, false);
  assert.equal(stages.rebuild.matchedRedeemer.tag, 1);
  assert.equal(stages.rebuild.matchedRedeemer.index, 0);
  assert.equal(stages.rebuild.matchedRedeemer.memory, 800000);
  assert.equal(stages.rebuild.matchedRedeemer.steps, 400000);
  assert.match(stages.rebuild.scriptDataHash, /^[0-9a-f]{64}$/);
  assert.equal(stages.rebuild.nativeBudgetMeasured, false);
  assert.equal(stages.rebuild.observedMemory, 800000);
  assert.equal(stages.rebuild.observedSteps, 400000);
  assert.equal(stages.outputFee.referenceScriptOmitted, true);
  assert.equal(stages.outputFee.nativeBudgetMeasured, false);
  assert.ok(stages.outputFee.omittedFlags.includes('--reference-script-size'));
  assert.equal(stages.spend.code, 'SPEND_BODIES_ENCODED', stages.spend.code);
  assert.equal(stages.witnessCount.code, 'WITNESS_SET_COUNTED_UNIT');
  assert.equal(stages.witnessCount.witnessCountVerified, true);
  assert.equal(stages.witnessCount.signatureVerified, false);
  assert.equal(stages.witnessCount.signatureCheckRan, true);
  assert.equal(stages.witnessCount.bodyBound, true);
  assert.equal(stages.witnessCount.ledgerExecution, false);
  assert.equal(stages.witnessCount.nativeBudgetMeasured, false);
  assert.equal(stages.receipt.failureClass, 'confirmation-timeout');
  assert.equal(stages.receipt.confirmed, false);
  assert.equal(stages.receipt.helperConfirmed, false);
  assert.ok(stages.receipt.calls.poll >= 1);
  assert.equal(stages.receipt.helperSha256, 'f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18');
  assert.equal(stages.receipt.receiptRoot, OWNED_RECEIPTS);
  assert.equal(stages.receipt.providerQueried, false);
  assert.equal(stages.receipt.kept, true);
  assert.match(stages.receipt.label, /^c177-none-ref-[0-9a-f]{8}$/);
  assert.equal(existsSync(join(OWNED_RECEIPTS, stages.receipt.label, 'signed.cbor')), true);
  assert.equal(existsSync(join(OWNED_RECEIPTS, stages.receipt.label, 'receipt.json')), true);
  assert.equal(existsSync(join(TASK, 'tooling/transaction-receipts/revision02', stages.receipt.label)), false);
  assert.equal(stages.race.code, 'RACE_SUBMITTED_BEFORE_POLL', JSON.stringify({ code: stages.race.code, detail: stages.race.detail }));
  assert.equal(stages.race.submitted, false);
  assert.equal(stages.race.ledgerSubmitted, false);
  assert.equal(stages.race.submitsBeforeConfirms, true);
  assert.equal(stages.race.submitCount, 2);
  const submits = stages.race.events.map((item, index) => (item.event === 'submit' ? index : -1)).filter((index) => index >= 0);
  const confirms = stages.race.events.map((item, index) => (item.event === 'confirm' ? index : -1)).filter((index) => index >= 0);
  assert.equal(confirms.every((index) => index > Math.max(...submits)), true);
  assert.equal(stages.clockParity.code, 'POSIX_PARITY_MATCH');
  assert.equal(stages.clockParity.parityExecuted, true);
  assert.equal(stages.sibling.ok, true, JSON.stringify({ code: stages.sibling.code, detail: stages.sibling.detail }));
  assert.equal(stages.sibling.siblingEmitted, true);
  assert.equal(stages.readback.code, 'HEAD_READBACK', stages.readback.code);
  assert.equal(stages.readback.fieldCount, 13);
  assert.equal(stages.readback.txid, stages.body.txid);
  assert.equal(stages.readback.unitOnly, true);
  assert.equal(stages.readback.financialMatch, true, JSON.stringify({
    code: stages.readback.financialCode,
    mismatches: stages.readback.mismatches,
    effect: stages.readback.effectQueryCode || null,
    posts: stages.posts?.code || null,
    spend: stages.spend?.code || null,
  }));
  assert.equal(stages.readback.financialCode, 'FINANCIAL_FRAME_MATCHED');
  assert.equal(stages.readback.confirmed, false);
  assert.equal(stages.confirmation.code, 'READBACK_UNIT_ONLY');
  assert.equal(stages.confirmation.confirmed, false);
  assert.equal(report.confirmed, false);
  assert.equal(stages.readback.nativePins, false);
  const frame = stages.spend.release.frame;
  const decoded = stages.readback.decoded;
  for (const name of [
    'program_hash', 'bounds_hash', 'source_hash', 'schema_hash', 'seed_hash',
    'domain_network', 'domain_deployment', 'instanceId', 'sourceHash',
    'transport_revision', 'transport_remaining', 'ordinary_locked', 'ordinary_revision',
    'headQuantity', 'continuingCash', 'payoutCash',
  ]) {
    assert.equal(decoded?.[name], frame?.[name], name);
  }
  assert.deepEqual(decoded.balances, frame.balances);
  assert.deepEqual(decoded.allowances, frame.allowances);
  assert.deepEqual(decoded.obligations, frame.obligations);
  assert.deepEqual(decoded.work, frame.work);
  assert.deepEqual(decoded.usedTransferIds, frame.usedTransferIds);
  assert.equal(decoded.work.spent, 49);
  assert.equal(decoded.continuingCash, 0);
  assert.equal(decoded.payoutCash, 100);
  assert.equal(decoded.headQuantity, 1);
  assert.equal(decoded.transport_remaining, 0);
  assert.notEqual(decoded.transport_remaining, 1);
  const openDatum = JSON.parse(readFileSync(join(root, 'datum.json'), 'utf8'));
  assert.equal(openDatum.constructor, 0);
  assert.equal(openDatum.fields.length, 13);
  assert.equal(openDatum.fields[9].int, 1);
  const effectMap = JSON.parse(readFileSync(join(PACKET, 'checks', 'none-ref', 'effect-utxo.json'), 'utf8'));
  const effectHead = effectMap[`${stages.spend.release.txid}#${stages.spend.release.builtHeadOutputIndex}`];
  assert.equal(effectHead.inlineDatum.constructor, 0);
  assert.equal(effectHead.inlineDatum.fields.length, 13);
  assert.equal(effectHead.evidenceClass, 'unit-fixture');
  assert.equal(Object.hasOwn(effectHead.inlineDatum, 'balances'), false);
  assert.equal(stages.readback.effectTxid, stages.spend.release.txid);
  assert.notEqual(stages.readback.effectTxid, stages.body.txid);
  assert.equal(stages.readback.effectUnitOnly, true);
  const byAction = stages.readback.byAction;
  for (const kind of ['release', 'refund', 'cancel']) {
    assert.equal(byAction?.[kind]?.financialMatch, true, JSON.stringify(byAction?.[kind] || null));
    assert.equal(byAction[kind].judgeCode, 'ECONOMIC_ACTION_MATCHED');
    assert.equal(byAction[kind].confirmed, false);
    assert.equal(byAction[kind].providerQueried, false);
    assert.equal(byAction[kind].spendMintEmpty, true);
    assert.equal(byAction[kind].unitOnly, true);
    assert.equal(stages.spend[kind].economicAction.code, 'ECONOMIC_ACTION_MATCHED');
    assert.equal(stages.spend[kind].economicAction.confirmed, false);
    assert.equal(stages.spend[kind].economicAction.nativeBudgetMeasured, false);
    assert.equal(stages.spend[kind].spendMintEmpty, true);
  }
  assert.equal(byAction.release.recipient, 'Lender');
  assert.equal(byAction.refund.recipient, 'Borrower');
  assert.equal(byAction.cancel.recipient, 'Borrower');
  assert.equal(stages.readback.economicAction.code, 'ECONOMIC_ACTION_MATCHED');
  assert.equal(stages.readback.economicAction.recipient, 'Lender');
  assert.equal(stages.readback.economicAction.confirmed, false);
  assert.equal(stages.readback.economicAction.providerQueried, false);
  assert.notEqual(byAction.release.payoutPaymentHash, byAction.refund.payoutPaymentHash);
  assert.equal(byAction.refund.payoutPaymentHash, byAction.cancel.payoutPaymentHash);
  assert.match(byAction.release.payoutPaymentHash, /^[0-9a-f]{56}$/);
  for (const kind of ['refund', 'cancel']) {
    const row = effectMap[`${stages.spend[kind].txid}#${stages.spend[kind].builtHeadOutputIndex ?? 0}`];
    assert.equal(row.inlineDatum.constructor, 0);
    assert.equal(row.inlineDatum.fields.length, 13);
    assert.equal(row.evidenceClass, 'unit-fixture');
  }
  assert.equal(report.ledgerExecution, false);
  assert.equal(report.ledgerSubmitted, false);
  assert.equal(report.nativePins, false);
  const log = readFileSync(cliLog, 'utf8');
  assert.match(log, /"\(800000,400000\)"/);
  assert.match(log, new RegExp(`${stages.body.txid}#${stages.body.headOutputIndex}`));
  assert.match(log, /"calculate-plutus-script-cost","offline"/);
  assert.equal(log.includes('"online"'), false);
  assert.equal(log.includes('--reference-script-size'), false);
  assert.equal(log.includes('script data hash'), false);
  const headFeeLines = log.split('\n').filter((line) => line.includes('"calculate-min-fee"') && line.includes('head-open.body.json'));
  assert.equal(headFeeLines.length, 2);
  const recountedArgv = JSON.parse(headFeeLines[1]);
  const witnessAt = recountedArgv.indexOf('--witness-count');
  assert.equal(recountedArgv[witnessAt + 1], String(stages.witnessCount.counted));
  assert.equal(stages.outputFee.witnessCountProvenance.witnessCountProvenanceClass, 'counted-from-signing-key-paths');
  assert.equal(stages.outputFee.feeMatchesSignedWitnessCount, true);
  assert.equal(stages.outputFee.nativeBudgetMeasured, false);
  assert.equal(String(stages.outputFee.parsed.fee), String(stages.outputFee.recountedFee));
  const headFile = join(PACKET, 'checks', 'none-ref', 'assemble', 'head-open.body.json');
  const storedHead = JSON.parse(readFileSync(headFile, 'utf8'));
  assert.equal(storedHead.cborHex, stages.body.cborHex);
  rmSync(root, { recursive: true, force: true });
});

test('reference script size is priced on the connected consumer', async () => {
  const prep = mkdtempSync(join(CHECKS, 'ref-file-'));
  const referenceScriptPath = join(prep, 'ref.bytes');
  writeFileSync(referenceScriptPath, Buffer.alloc(32, 0x7a));
  const { report, cliLog, root } = await runConsumer({ referenceScriptPath, label: 'with-ref' });
  const stages = report.stages;
  assert.equal(stages.mint.code, 'UNSIGNED_BODY', JSON.stringify({ code: stages.mint.code, detail: stages.mint.detail }));
  assert.equal(stages.body.code, 'UNSIGNED_BODY', JSON.stringify({ code: stages.body.code, detail: stages.body.detail }));
  assert.equal(stages.body.headOutputIndex, 0);
  assert.equal(stages.body.datumFieldCount, 13);
  assert.equal(stages.rebuild.applied, true, JSON.stringify({ code: stages.rebuild.code }));
  assert.equal(stages.rebuild.nativeBudgetMeasured, false);
  assert.equal(stages.rebuild.scriptDataHashSource, 'conway-body-key-11');
  assert.equal(stages.spend.code, 'SPEND_BODIES_ENCODED', stages.spend.code);
  assert.equal(stages.witnessCount.code, 'WITNESS_SET_COUNTED_UNIT');
  assert.equal(stages.witnessCount.signatureVerified, false);
  assert.equal(stages.witnessCount.ledgerExecution, false);
  assert.equal(stages.race.code, 'RACE_SUBMITTED_BEFORE_POLL', JSON.stringify({ code: stages.race.code, detail: stages.race.detail }));
  assert.equal(stages.race.submitted, false);
  assert.equal(stages.race.submitsBeforeConfirms, true);
  assert.equal(stages.race.submitCount, 2);
  assert.equal(stages.readback.code, 'HEAD_READBACK', stages.readback.code);
  assert.equal(stages.readback.fieldCount, 13);
  assert.equal(stages.readback.txid, stages.body.txid);
  assert.equal(stages.outputFee.referenceScriptOmitted, false);
  assert.equal(stages.outputFee.omittedFlags.includes('--reference-script-size'), false);
  assert.equal(stages.outputFee.referenceScriptSize, 32);
  assert.equal(stages.outputFee.nativeBudgetMeasured, false);
  const log = readFileSync(cliLog, 'utf8');
  assert.match(log, /"--reference-script-size","32"/);
  assert.match(log, /"calculate-plutus-script-cost","offline"/);
  assert.equal(log.includes('script data hash'), false);
  rmSync(root, { recursive: true, force: true });
  rmSync(prep, { recursive: true, force: true });
});

test('online cost refuses a caller-chosen magic before the cost command spawns', async () => {
  const { report, cliLog, root } = await runConsumer({ label: 'online-refused', costMode: 'online' });
  const stages = report.stages;
  assert.equal(stages.mint.code, 'MAGIC_SOURCE_REFUSED', JSON.stringify({ mint: stages.mint.code, rebuild: stages.rebuild.code }));
  assert.equal(stages.rebuild.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(stages.rebuild.applied, false);
  assert.equal(stages.rebuild.nativeBudgetMeasured, false);
  assert.equal(stages.body.code, 'HEAD_OPEN_BLOCKED');
  assert.equal(stages.spend, null);
  assert.equal(report.ledgerExecution, false);
  const log = readFileSync(cliLog, 'utf8');
  assert.equal(log.includes('"calculate-plutus-script-cost","online"'), false);
  assert.equal(log.includes('"calculate-plutus-script-cost","offline"'), false);
  rmSync(root, { recursive: true, force: true });
});

test('online unit cost binds shelley magic and does not measure a native budget', async () => {
  const { report, cliLog, root } = await runConsumer({
    label: 'online-unit',
    costMode: 'online',
    costMagicSource: 'shelley-networkMagic',
  });
  const stages = report.stages;
  assert.equal(stages.rebuild.applied, true, JSON.stringify({ code: stages.rebuild.code, hash: stages.rebuild.scriptDataHash }));
  assert.equal(stages.rebuild.nativeBudgetMeasured, false);
  assert.equal(stages.rebuild.scriptDataHashSource, 'conway-body-key-11');
  assert.equal(stages.rebuild.scriptDataHashInvented, false);
  assert.equal(stages.spend.code, 'SPEND_BODIES_ENCODED', stages.spend.code);
  assert.equal(stages.witnessCount.code, 'WITNESS_SET_COUNTED_UNIT');
  assert.equal(stages.witnessCount.signatureVerified, false);
  assert.equal(stages.race.code, 'RACE_SUBMITTED_BEFORE_POLL');
  assert.equal(stages.race.submitted, false);
  assert.equal(stages.race.ledgerSubmitted, false);
  assert.equal(stages.race.submitCount, 2);
  assert.equal(stages.readback.fieldCount, 13);
  assert.equal(stages.readback.unitOnly, true);
  const log = readFileSync(cliLog, 'utf8');
  assert.match(log, /"calculate-plutus-script-cost","online","--socket-path"/);
  assert.match(log, /"--testnet-magic","764"/);
  assert.equal(log.includes('"offline"'), false);
  assert.equal(log.includes('script data hash'), false);
  rmSync(root, { recursive: true, force: true });
});

test('supplied 1/1 execution units are refused before a body is adopted', async () => {
  const { report, root } = await runConsumer({ label: 'units-1-1', costMemory: 1, costSteps: 1 });
  const stages = report.stages;
  assert.equal(stages.rebuild.code, 'SUPPLIED_UNITS_NOT_A_MEASUREMENT', JSON.stringify({ rebuild: stages.rebuild.code, mint: stages.mint.code }));
  assert.equal(stages.rebuild.applied, false);
  assert.equal(stages.rebuild.nativeBudgetMeasured, false);
  assert.equal(stages.mint.code, 'SUPPLIED_UNITS_NOT_A_MEASUREMENT');
  assert.equal(stages.spend, null);
  assert.equal(report.ledgerExecution, false);
  rmSync(root, { recursive: true, force: true });
});

test('captured Conway bodies expose key 11 and redeemer pointer, not view fields', () => {
  const cash = JSON.parse(readFileSync(join(TASK, 't1-native-consumer-repair-70/artifacts/cash-mint.body.json'), 'utf8'));
  const head = JSON.parse(readFileSync(join(TASK, 't1-native-consumer-repair-70/artifacts/head-open.body.json'), 'utf8'));
  for (const envelope of [cash, head]) {
    assert.equal(Object.hasOwn(envelope, 'scriptDataHash'), false);
    assert.equal(Object.hasOwn(envelope, 'witnesses'), false);
  }
  const cashTx = readConwayEnvelope(cash.cborHex);
  const headTx = readConwayEnvelope(head.cborHex);
  assert.equal(cashTx.ok, true, 'cash envelope did not decode');
  assert.equal(headTx.ok, true, 'head envelope did not decode');
  assert.equal(cashTx.scriptDataHash, '20a2dfdea72bbd2ce02ea7ec14b1bc6be89cc1f337614f35013b74c6a0d6aeff');
  assert.equal(headTx.scriptDataHash, cashTx.scriptDataHash);
  assert.deepEqual(cashTx.redeemers, [{ tag: 1, index: 0, memory: 1, steps: 1 }]);
  assert.deepEqual(headTx.redeemers, [{ tag: 1, index: 0, memory: 1, steps: 1 }]);
  assert.equal(cashTx.vkeyWitnessCount, 0);
  assert.equal(headTx.vkeyWitnessCount, 0);
  assert.equal(headTx.outputs[0].datumConstructor, 0);
  assert.equal(headTx.outputs[0].datumFieldCount, 13);
  assert.equal(headTx.headOutputIndex, 0);
  assert.equal(cashTx.outputCount, 1);
  assert.equal(headTx.outputCount, 1);
});

function cborUintHex(value) {
  const amount = BigInt(value);
  if (amount < 24n) return amount.toString(16).padStart(2, '0');
  if (amount < 256n) return `18${amount.toString(16).padStart(2, '0')}`;
  if (amount < 65536n) return `19${amount.toString(16).padStart(4, '0')}`;
  return `1a${amount.toString(16).padStart(8, '0')}`;
}

function unitEd25519(cml) {
  const { publicKey, privateKey } = generateKeyPairSync('ed25519');
  const der = publicKey.export({ type: 'spki', format: 'der' });
  const pk = cml.PublicKey.from_bytes(der.subarray(der.length - 32));
  return { privateKey, pk, hash: pk.hash().to_hex() };
}

function witnessHexFor(cml, rows) {
  const list = cml.VkeywitnessList.new();
  for (const row of rows) list.add(cml.Vkeywitness.new(row.pk, row.signature));
  const set = cml.TransactionWitnessSet.new();
  set.set_vkeywitnesses(list);
  return Buffer.from(set.to_cbor_bytes()).toString('hex');
}

test('owned receipt directory is rejected before the provider poll', async () => {
  const owned = join(PACKET, 'receipts-owned');
  rmSync(owned, { recursive: true, force: true });
  const result = await bindHelperReceipt('ab'.repeat(32), {
    receiptRoot: owned,
    cliPath: join(PACKET, 'missing-cli'),
    confirmationBudgetMs: 80,
    label: 'owned-164-namespace',
  });
  assert.equal(result.failureClass, 'namespace-rejected');
  assert.equal(result.calls.poll, 0);
  assert.equal(result.calls.submit, 0);
  assert.equal(result.confirmed, false);
  assert.equal(result.helperSha256, 'f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18');
  assert.equal(existsSync(owned), false);
});

test('exact body hash and required signer reject a placeholder and a foreign key', () => {
  const fixturePath = join(TASK, 't1-native-consumer-repair-70/artifacts/cash-mint.body.json');
  const before = createHash('sha256').update(readFileSync(fixturePath)).digest('hex');
  const cash = JSON.parse(readFileSync(fixturePath, 'utf8'));
  const placeholder = verifyExactBodyWitness(cash.cborHex, encodeUnitWitnessCborHex());
  assert.equal(placeholder.signatureVerified, false);
  assert.equal(placeholder.signatureCheckRan, true);
  assert.equal(placeholder.bodyBound, true);
  assert.equal(placeholder.bodyHashHex, '4daa8d22b9a57a2549f9d5cba91eb5955721e6dbd8a2d28119f827ac2aa6ac83');
  assert.equal(placeholder.requiredSignerCount, 1);
  assert.equal(placeholder.requiredSignersBound, false);
  assert.equal(placeholder.nativeBudgetMeasured, false);
  assert.equal(placeholder.ledgerExecution, false);
  assert.equal(placeholder.confirmed, false);
  const cmlPath = join(TASK, 'integrity140-verified-freeze148/installed-cml/cardano_multiplatform_lib.js');
  const cml = createRequire(cmlPath)(cmlPath);
  const tx = cml.Transaction.from_cbor_bytes(Buffer.from(cash.cborHex, 'hex'));
  const bodyHash = Buffer.from(cml.hash_transaction(tx.body()).to_raw_bytes());
  const foreignKey = unitEd25519(cml);
  const foreign = witnessHexFor(cml, [{
    pk: foreignKey.pk,
    signature: cml.Ed25519Signature.from_raw_bytes(sign(null, bodyHash, foreignKey.privateKey)),
  }]);
  const mismatched = verifyExactBodyWitness(cash.cborHex, foreign);
  assert.equal(mismatched.code, 'WITNESS_SIGNER_MISMATCH');
  assert.equal(mismatched.signatureVerified, false);
  assert.equal(mismatched.verifiedCount, 1);
  assert.equal(mismatched.requiredSignersBound, false);
  assert.equal(mismatched.nativeBudgetMeasured, false);
  const requiredKey = unitEd25519(cml);
  const feeKey = unitEd25519(cml);
  const collateralKey = unitEd25519(cml);
  const signers = cml.Ed25519KeyHashList.new();
  signers.add(requiredKey.pk.hash());
  const body = tx.body();
  body.set_required_signers(signers);
  const auxiliary = tx.auxiliary_data() || null;
  const rebound = cml.Transaction.new(body, tx.witness_set(), tx.is_valid(), auxiliary);
  const reboundHex = Buffer.from(rebound.to_cbor_bytes()).toString('hex');
  const reboundHash = Buffer.from(cml.hash_transaction(cml.Transaction.from_cbor_bytes(Buffer.from(reboundHex, 'hex')).body()).to_raw_bytes());
  const signRow = (key) => ({
    pk: key.pk,
    signature: cml.Ed25519Signature.from_raw_bytes(sign(null, reboundHash, key.privateKey)),
  });
  const requiredOnly = witnessHexFor(cml, [signRow(requiredKey)]);
  const missingCredential = verifyExactBodyWitness(reboundHex, requiredOnly, {
    ordinaryKeyHashes: [feeKey.hash],
    collateralKeyHashes: [collateralKey.hash],
  });
  assert.equal(missingCredential.signatureVerified, false);
  assert.equal(missingCredential.requiredSignersBound, true);
  assert.equal(missingCredential.collateralOmitted, false);
  assert.equal(missingCredential.code, 'WITNESS_KEYS_INCOMPLETE');
  const omittedCredential = verifyExactBodyWitness(reboundHex, requiredOnly);
  assert.equal(omittedCredential.signatureVerified, false);
  assert.equal(omittedCredential.collateralOmitted, true);
  assert.equal(omittedCredential.code, 'WITNESS_KEYS_INCOMPLETE');
  const coveredWitness = witnessHexFor(cml, [signRow(requiredKey), signRow(feeKey), signRow(collateralKey)]);
  const matched = verifyExactBodyWitness(reboundHex, coveredWitness, {
    ordinaryKeyHashes: [feeKey.hash],
    collateralKeyHashes: [collateralKey.hash],
  });
  assert.equal(matched.signatureVerified, true);
  assert.equal(matched.requiredSignersBound, true);
  assert.equal(matched.collateralOmitted, false);
  assert.equal(matched.code, 'WITNESS_SIGNATURE_VERIFIED');
  assert.equal(matched.nativeBudgetMeasured, false);
  assert.equal(matched.ledgerExecution, false);
  assert.equal(matched.ledgerSubmitted, false);
  assert.equal(matched.confirmed, false);
  assert.notEqual(matched.bodyHashHex, placeholder.bodyHashHex);
  const after = createHash('sha256').update(readFileSync(fixturePath)).digest('hex');
  assert.equal(after, before);
});

test('a disagreed fee recount keeps the preserved head body', async () => {
  const marker = join(CHECKS, 'fee-diverge-marker');
  rmSync(marker, { force: true });
  process.env.M129_DIVERGE_RECOUNT = '1';
  process.env.M129_DIVERGE_MARKER = marker;
  try {
    const { report, cliLog, root } = await runConsumer({ label: 'fee-diverge' });
    const stages = report.stages;
    const protocol = JSON.parse(readFileSync(PROTOCOL, 'utf8'));
    const originalFee = `02${cborUintHex(protocol.txFeeFixed)}`;
    const divergedFee = `02${cborUintHex(Number(protocol.txFeeFixed) + 44)}`;
    assert.equal(stages.outputFee.code, 'FEE_WITNESS_COUNT_DIVERGED');
    assert.equal(stages.outputFee.feeMatchesSignedWitnessCount, false);
    assert.equal(stages.outputFee.nativeBudgetMeasured, false);
    assert.equal(stages.witnessCount.signatureVerified, false);
    assert.equal(stages.body.cborHex.includes(originalFee), true);
    assert.equal(stages.body.cborHex.includes(divergedFee), false);
    const stored = JSON.parse(readFileSync(join(PACKET, 'checks', 'fee-diverge', 'assemble', 'head-open.body.json'), 'utf8'));
    assert.equal(stored.cborHex, stages.body.cborHex);
    const log = readFileSync(cliLog, 'utf8');
    const headFeeLines = log.split('\n').filter((line) => line.includes('"calculate-min-fee"') && line.includes('head-open.body.json'));
    assert.equal(headFeeLines.length, 2);
    rmSync(root, { recursive: true, force: true });
  } finally {
    delete process.env.M129_DIVERGE_RECOUNT;
    delete process.env.M129_DIVERGE_MARKER;
    rmSync(marker, { force: true });
  }
});

function actionFrame(action, patch = {}) {
  const recipient = action === 'release' ? 'Lender' : 'Borrower';
  const transferId = action === 'release' ? 'Release1' : action === 'refund' ? 'Refund1' : 'Cancel1';
  return {
    program_hash: '11'.repeat(28),
    bounds_hash: '22'.repeat(28),
    source_hash: '33'.repeat(28),
    schema_hash: '44'.repeat(28),
    seed_hash: '55'.repeat(28),
    domain_network: 'yaci-isolated-t1',
    domain_deployment: 't1-custody-revision03',
    instanceId: '66'.repeat(28),
    balances: {
      Escrow: 0,
      Borrower: recipient === 'Borrower' ? 100 : 0,
      Lender: recipient === 'Lender' ? 100 : 0,
    },
    allowances: {
      Escrow: { remaining: 0, spent: 100 },
      Borrower: { remaining: 0, spent: 0 },
      Lender: { remaining: 0, spent: 0 },
    },
    obligations: [],
    work: { remaining: 975, spent: 49, closureReserve: 16 },
    usedTransferIds: [transferId],
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

test('economic action judge accepts the three posts and refuses adverse frames', () => {
  for (const action of ['release', 'refund', 'cancel']) {
    const matched = judgeEconomicAction(actionFrame(action), action);
    assert.equal(matched.code, 'ECONOMIC_ACTION_MATCHED');
    assert.equal(matched.confirmed, false);
    assert.equal(matched.ledgerExecution, false);
    assert.equal(matched.nativeBudgetMeasured, false);
    assert.equal(matched.providerQueried, false);
    assert.equal(matched.spendMint, 'empty');
    assert.equal(matched.continuationHead, 1);
  }
  assert.equal(judgeEconomicAction(actionFrame('release'), 'refund').code, 'ECONOMIC_ACTION_REFUSED');
  const amount = actionFrame('release', { balances: { Escrow: 0, Borrower: 0, Lender: 99 } });
  assert.ok(judgeEconomicAction(amount, 'release').mismatches.includes('balances'));
  const duty = actionFrame('cancel', { obligations: [{ party: 'Borrower' }] });
  assert.ok(judgeEconomicAction(duty, 'cancel').mismatches.includes('obligations'));
  const work = actionFrame('refund', { work: { remaining: 975, spent: 48, closureReserve: 16 } });
  assert.ok(judgeEconomicAction(work, 'refund').mismatches.includes('work'));
  const ids = actionFrame('release', { usedTransferIds: ['Release1', 'Release1'] });
  assert.ok(judgeEconomicAction(ids, 'release').mismatches.includes('transfer'));
  const stale = actionFrame('release', { transport_remaining: 1 });
  assert.ok(judgeEconomicAction(stale, 'release').mismatches.includes('transport'));
  const fake = actionFrame('release', { confirmed: true });
  assert.ok(judgeEconomicAction(fake, 'release').mismatches.includes('fakeConfirmed'));
  const missing = actionFrame('release', { payoutCash: null });
  assert.ok(judgeEconomicAction(missing, 'release').mismatches.includes('payoutCash'));
  const zeros = actionFrame('release', { program_hash: '00'.repeat(32), instanceId: '00'.repeat(32) });
  assert.ok(judgeEconomicAction(zeros, 'release').mismatches.includes('identity'));
  const shape = actionFrame('release', {
    instanceId: 'f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d4',
  });
  assert.ok(judgeEconomicAction(shape, 'release').mismatches.includes('identity'));
  assert.equal(judgeEconomicAction(null, 'release').code, 'ECONOMIC_ACTION_REFUSED');
  assert.equal(judgeEconomicAction(actionFrame('release'), 'phase').code, 'ECONOMIC_ACTION_REFUSED');
});

test('needed script language follows purpose and ignores unused witness languages', () => {
  const cashPolicy = 'c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555';
  const cash = JSON.parse(readFileSync(join(TASK, 't1-native-consumer-repair-70/artifacts/cash-mint.body.json'), 'utf8'));
  const inline = selectNeededScriptLanguage({ cborHex: cash.cborHex });
  assert.equal(inline.ok, true, JSON.stringify({ code: inline.code, unresolved: inline.unresolved, detail: inline.detail }));
  assert.equal(inline.code, 'NEEDED_SCRIPT_SELECTED');
  assert.equal(inline.needed.length, 1);
  assert.equal(inline.needed[0].purpose, 'mint');
  assert.equal(inline.needed[0].scriptHash, cashPolicy);
  assert.equal(inline.needed[0].language, 'PlutusV3');
  assert.equal(inline.needed[0].source, 'inline-script');
  assert.deepEqual(inline.languages, ['PlutusV3']);
  assert.equal(inline.witnessFieldUsed, false);
  assert.equal(inline.referenceUnionUsed, false);
  assert.equal(inline.nativeBudgetMeasured, false);
  assert.equal(inline.nativeReferenceFixture, false);
  assert.equal(inline.ledgerExecution, false);
  assert.equal(inline.confirmed, false);
  assert.equal(inline.inlineDatumInIntegrity, false);
  assert.equal(inline.linkedBinaryAbi, 'UNKNOWN');
  assert.equal(inline.ledgerCallee, 'UNKNOWN');
  const foreign = selectNeededScriptLanguage({
    cborHex: cash.cborHex,
    referenceInputs: [`${'ab'.repeat(32)}#0`],
    referenceUtxos: [{
      scriptHash: 'cd'.repeat(28),
      language: 'PlutusV1',
      ref: `${'ab'.repeat(32)}#0`,
    }],
    witnessLanguages: ['PlutusV1', 'PlutusV2', 'PlutusV3'],
  });
  assert.equal(foreign.ok, true, JSON.stringify(foreign));
  assert.deepEqual(foreign.languages, ['PlutusV3']);
  assert.equal(foreign.needed[0].scriptHash, cashPolicy);
  assert.equal(foreign.witnessFieldUsed, false);
  const spendRef = `${'ef'.repeat(32)}#1`;
  const covered = selectNeededScriptLanguage({
    redeemers: [{ tag: 1, index: 0 }],
    mintPolicies: [cashPolicy],
    inlineScripts: [],
    witnessLanguages: [],
    spendingInputs: [spendRef],
    referenceUtxos: [
      { scriptHash: cashPolicy, language: 'PlutusV3', ref: spendRef },
      { scriptHash: 'cd'.repeat(28), language: 'PlutusV1', ref: `${'ab'.repeat(32)}#0` },
    ],
  });
  assert.equal(covered.code, 'NEEDED_SCRIPT_SELECTED', JSON.stringify(covered));
  assert.equal(covered.needed[0].source, 'reference-utxo');
  assert.equal(covered.needed[0].language, 'PlutusV3');
  assert.deepEqual(covered.languages, ['PlutusV3']);
  assert.equal(covered.nativeReferenceFixture, false);
  const paymentHash = '9d0b9e4c9c125f34f35d77ce38faf1b72ee3e54d884c1aa07af233dd';
  const spent = selectNeededScriptLanguage({
    redeemers: [{ tag: 0, index: 0 }],
    spentOutputs: [{ pointer: 0, paymentScriptHash: paymentHash, ref: spendRef }],
    spendingInputs: [{ ref: spendRef }],
    inlineScripts: [],
    witnessLanguages: ['PlutusV1'],
    referenceUtxos: [{ scriptHash: paymentHash, language: 'PlutusV2', ref: spendRef }],
  });
  assert.equal(spent.code, 'NEEDED_SCRIPT_SELECTED', JSON.stringify(spent));
  assert.equal(spent.needed[0].purpose, 'spend');
  assert.equal(spent.needed[0].scriptHash, paymentHash);
  assert.deepEqual(spent.languages, ['PlutusV2']);
  assert.notDeepEqual(spent.languages, spent.witnessLanguages);
  assert.equal(spent.witnessFieldUsed, false);
  const diverged = selectNeededScriptLanguage({
    redeemers: [{ tag: 0, index: 0 }],
    spentOutputs: [{ pointer: 0, paymentScriptHash: paymentHash }],
    spendScripts: [{ pointer: 0, scriptHash: cashPolicy }],
    inlineScripts: [{ hash: paymentHash, language: 'PlutusV3' }],
  });
  assert.equal(diverged.code, 'SPEND_HASH_DIVERGED');
  assert.equal(diverged.languages, null);
  const empty = selectNeededScriptLanguage({
    cborHex: cash.cborHex,
    emptyScriptLists: [{ decoderVersion: 9 }],
  });
  assert.equal(empty.code, 'EMPTY_SCRIPT_LIST_REJECTED');
  assert.equal(empty.ok, false);
  assert.equal(empty.languages, null);
  assert.equal(empty.decoderVersionAtLinkedNode, 'UNKNOWN');
  assert.equal(empty.nativeBudgetMeasured, false);
  const kept = selectNeededScriptLanguage({
    cborHex: cash.cborHex,
    emptyScriptLists: [{ decoderVersion: 8 }],
  });
  assert.equal(kept.code, 'NEEDED_SCRIPT_SELECTED');
  const missingModel = selectNeededScriptLanguage({
    cborHex: cash.cborHex,
    costModels: { PlutusV1: [1], PlutusV2: [1] },
  });
  assert.equal(missingModel.code, 'COST_MODEL_MISSING');
  assert.equal(missingModel.languages, null);
  assert.deepEqual(missingModel.missingCostModels, ['PlutusV3']);
  assert.equal(missingModel.nativeBudgetMeasured, false);
  const other = selectNeededScriptLanguage({
    redeemers: [{ tag: 4, index: 0 }],
    witnessLanguages: ['PlutusV3'],
  });
  assert.equal(other.code, 'NEEDED_SCRIPT_UNDECIDED');
  assert.equal(other.languages, null);
});

test('approved bootstrap plan stays unexecuted and keeps query quantities exact', () => {
  const refused = planApprovedBootstrap({ magic: 42, nowMs: 1_700_000_000_000, leadSeconds: 30, runDir: CHECKS });
  assert.equal(refused.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(refused.spawned, false);
  assert.equal(refused.nodeStarted, false);
  const runDir = mkdtempSync(join(CHECKS, 'boot-'));
  const plan = planApprovedBootstrap({
    magic: 764,
    nowMs: 1_700_000_000_000,
    leadSeconds: 30,
    runDir,
  });
  assert.equal(plan.code, 'APPROVED_BOOTSTRAP_PLANNED', plan.code);
  assert.equal(plan.executed, false);
  assert.equal(plan.spawned, false);
  assert.equal(plan.nodeStarted, false);
  assert.equal(plan.byronReplaced, false);
  assert.equal(plan.ledgerExecution, false);
  assert.equal(plan.nativeBudgetMeasured, false);
  const s1 = plan.steps.find((step) => step.id === 'S1-create-testnet-data');
  const s2 = plan.steps.find((step) => step.id === 'S2-create-cardano-byron-zero');
  const s3 = plan.steps.findIndex((step) => step.id === 'S3-replace-byron-genesis');
  const s4 = plan.steps.findIndex((step) => step.id === 'S4-hash-after-replacement');
  const s7 = plan.steps.find((step) => step.id === 'S7-node-run');
  assert.equal(s1.argv.includes('--pools'), true);
  assert.equal(s1.argv[s1.argv.indexOf('--pools') + 1], '1');
  assert.equal(s1.argv[s1.argv.indexOf('--stake-delegators') + 1], '1');
  assert.equal(s1.argv[s1.argv.indexOf('--utxo-keys') + 1], '1');
  assert.equal(s1.argv.includes('--supply'), false);
  assert.equal(s2.supplyOmitted, true);
  assert.equal(s2.argv.includes('--supply'), false);
  assert.equal(s2.argv[s2.argv.indexOf('--gen-utxo-keys') + 1], '0');
  assert.equal(s1.argv[s1.argv.indexOf('--start-time') + 1], s2.argv[s2.argv.indexOf('--start-time') + 1]);
  assert.match(plan.start, /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/);
  assert.equal(s4 > s3, true);
  assert.equal(s7.executed, false);
  assert.equal(s7.spawn, false);
  assert.equal(s7.argv.includes(join(runDir, 'node.socket')), true);
  assert.equal(plan.home, join(runDir, 'home'));
  assert.equal(plan.pidPath, join(runDir, 'node.pid'));
  assert.equal(plan.specPins.alonzo.sha256, '104bdbd36634fc48c0221dc481bb37f2556a3c0265aa26ce0820e28e3830cbd3');
  assert.equal(plan.specPins.conway.edit, null);
  const early = bindApprovedNodeConfig(plan, {
    byron: 'ab'.repeat(32),
    shelley: 'cd'.repeat(32),
    alonzo: 'ef'.repeat(32),
    conway: '12'.repeat(32),
    byronMethod: 'print-genesis-hash',
  });
  assert.equal(early.code, 'HASH_BEFORE_BYRON_REPLACEMENT');
  const bound = bindApprovedNodeConfig(noteByronReplaced(plan), {
    byron: 'ab'.repeat(32),
    shelley: 'cd'.repeat(32),
    alonzo: 'ef'.repeat(32),
    conway: '12'.repeat(32),
    byronMethod: 'print-genesis-hash',
  });
  assert.equal(bound.code, 'NODE_CONFIG_BOUND');
  assert.equal(bound.executed, false);
  assert.equal(bound.configPath, join(runDir, 'node-config.json'));
  assert.equal(bound.config.RequiresNetworkMagic, 'RequiresMagic');
  assert.equal(bound.config.ByronGenesisFile, 'genesis/byron-genesis.json');
  assert.equal(bound.config.ConwayGenesisFile, 'genesis/conway-genesis.json');
  assert.equal(Object.hasOwn(bound.config, 'DijkstraGenesisFile'), false);
  const specs = materializeApprovedBootstrapSpecs(plan, runDir);
  assert.equal(specs.code, 'BOOTSTRAP_SPECS_MATERIALIZED');
  assert.equal(specs.spawned, false);
  assert.equal(specs.writesGenesis, false);
  assert.equal(existsSync(join(runDir, 'genesis')), false);
  const edited = readFileSync(join(runDir, 'spec/alonzo.spec.json'), 'utf8');
  assert.equal(edited.includes('"exUnitsMem": 16500000'), true);
  assert.equal(edited.includes('"exUnitsMem": 14000000'), false);
  const source = readFileSync(plan.specPins.alonzo.path);
  assert.equal(createHash('sha256').update(source).digest('hex'), plan.specPins.alonzo.sha256);
  const copied = readFileSync(join(runDir, 'spec/conway.spec.json'));
  assert.equal(createHash('sha256').update(copied).digest('hex'), plan.specPins.conway.sha256);
  const funderRef = `${'ab'.repeat(32)}#0`;
  const parsed = parseQueryUtxoText(`{"${funderRef}":{"address":"addr_test1funder","value":{"lovelace":13500000000000000}}}`);
  const frame = producerRoleFrame(parsed);
  assert.equal(frame.code, 'PRODUCER_ROLE_FRAME');
  assert.equal(frame.lovelaceText, '13500000000000000');
  assert.equal(BigInt(frame.lovelaceText), 13500000000000000n);
  assert.equal(frame.numberUsed, false);
  assert.equal(frame.nodeUtxoQueried, false);
  assert.equal(frame.confirmed, false);
  assert.equal(frame.ledgerExecution, false);
  const lossy = producerRoleFrame({ ok: true, lovelace: 9007199254740993, lovelaceText: '9007199254740993' });
  assert.equal(lossy.code, 'LOSSY_LOVELACE');
  rmSync(runDir, { recursive: true, force: true });
});
