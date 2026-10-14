/**
 * Direct calls of the exported funding and consumed-financial functions.
 * Layer S 1000/1099, 1100/1100, and 1050/1051 must stop before encodeSpendBody.
 * A representable 40000/99999 release reaches CLI half-open slots. The redeemer keeps POSIX.
 * No node, genesis, keygen, fund, mint, sign, submit, query, or latch.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { chmodSync, existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assembleConsumedFinancial, buildApprovedFunding } from '../src/native-caller.mjs';
import { datumFromPost } from '../src/native-financial.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');
const CASH = join(TASK, 't1-cash-repair-53/prototype/validators/cash_fixture.ak');
const HERE = fileURLToPath(new URL('.', import.meta.url));
const INTEGRATE = join(HERE, '../src/native-integrate.mjs');
const FROZEN_CACHE = join(TASK, 't1-native-repair-59/checks/source-cache');
const FRESH_CACHE = join(TASK, 't1-consumer-repair-119/owned-output/checks/source-cache');
const SEED = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1';
const PAYMENT = `5820${'0123456789abcdeffedcba9876543210'.repeat(2)}`;
const PAYOUT = `5820${'fedcba98765432100123456789abcdef'.repeat(2)}`;

function ref(byte, index) {
  return { txId: byte.repeat(32), index };
}

function sourceRecord() {
  const sha = createHash('sha256').update(readFileSync(SOURCE)).digest('hex');
  for (const dir of [FRESH_CACHE, FROZEN_CACHE]) {
    const path = join(dir, `${sha}.json`);
    if (existsSync(path)) return { sha, ...JSON.parse(readFileSync(path, 'utf8')) };
  }
  return { sha, blueprintPath: null };
}

function writeStub(dir) {
  const logPath = join(dir, 'cli-argv.jsonl');
  const stub = join(dir, 'cli.mjs');
  writeFileSync(stub, `#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { encodeUnitConwayEnvelope } from ${JSON.stringify(INTEGRATE)};
const argv = process.argv.slice(2);
const log = process.env.MORIARTY_BODY_LOG;
function flag(name) {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : '';
}
let datumText = null;
let redeemerText = null;
const datumPath = flag('--tx-out-inline-datum-file');
const redeemerPath = flag('--tx-in-redeemer-file');
if (datumPath) { try { datumText = readFileSync(datumPath, 'utf8'); } catch { datumText = null; } }
if (redeemerPath) { try { redeemerText = readFileSync(redeemerPath, 'utf8'); } catch { redeemerText = null; } }
if (log) writeFileSync(log, JSON.stringify({ argv, datumText, redeemerText }) + '\\n', { flag: 'a' });
if (argv[0] === 'address' && argv[1] === 'build') {
  const file = flag('--payment-verification-key-file') || flag('--payment-script-file') || 'none';
  process.stdout.write('addr_test1' + createHash('sha256').update(file).digest('hex').slice(0, 20) + '\\n');
  process.exit(0);
}
if (argv.includes('build-raw')) {
  const out = flag('--out-file');
  writeFileSync(out, encodeUnitConwayEnvelope(argv));
  writeFileSync(out + '.argv.json', JSON.stringify(argv));
  process.exit(0);
}
if (argv.includes('txid')) {
  const body = JSON.parse(readFileSync(flag('--tx-body-file'), 'utf8'));
  process.stdout.write(createHash('sha256').update(body.cborHex).digest('hex') + '\\n');
  process.exit(0);
}
if (argv[0] === 'debug') {
  const saved = JSON.parse(readFileSync(flag('--tx-body-file') + '.argv.json', 'utf8'));
  const refs = [];
  for (let i = 0; i < saved.length; i += 1) {
    if (saved[i] === '--tx-in' || saved[i] === '--tx-in-collateral') refs.push(saved[i + 1]);
  }
  process.stdout.write(refs.join('\\n') + '\\n');
  process.exit(0);
}
if (argv.includes('calculate-min-required-utxo')) {
  process.stdout.write('Coin 2000000\\n');
  process.exit(0);
}
if (argv.includes('calculate-min-fee')) {
  process.stdout.write('{"fee":180000}\\n');
  process.exit(0);
}
if (argv.includes('calculate-plutus-script-cost')) {
  process.stdout.write(JSON.stringify([{ scriptHash: '11'.repeat(28), executionUnits: { memory: 800000, steps: 400000 }, lovelaceCost: 44 }]) + '\\n');
  process.exit(0);
}
process.exit(0);
`);
  chmodSync(stub, 0o755);
  return { stub, logPath };
}

function stagesFor(record) {
  const head = ref('12', 0);
  return {
    inputs: {
      ok: true,
      cashSeed: { txId: SEED, index: 1 },
      mintFee: ref('13', 0),
      mintCollateral: ref('14', 0),
      head,
      headFee: ref('15', 0),
      headCollateral: ref('16', 0),
      left: { fee: ref('17', 0), collateral: ref('18', 0) },
      right: { fee: ref('19', 0), collateral: ref('1a', 0) },
      cancel: { fee: ref('1b', 0), collateral: ref('1c', 0) },
    },
    source: {
      path: SOURCE,
      blueprintPath: record.blueprintPath,
      hashes: {
        programHash: '11'.repeat(32),
        boundsHash: '22'.repeat(32),
        sourceHash: record.sha,
        schemaHash: '44'.repeat(32),
        seedHash: '55'.repeat(32),
      },
    },
    time: { windows: { releaseSlot: '40', partitionSlot: '100' } },
  };
}

function spec() {
  return {
    outputLovelace: '2000000',
    feeLovelace: '200000',
    invalidBefore: '40',
    invalidHereafter: '100',
    paymentVkeyCbor: PAYMENT,
    networkMagic: 764,
  };
}

function requestFor(dir, bin, extra = {}) {
  return {
    evidenceRoot: dir,
    sourcePath: SOURCE,
    cashDesign: { sourcePath: CASH },
    cliPath: bin.stub,
    magic: 764,
    networkMagic: 764,
    payoutVkeyHex: PAYOUT,
    paymentVkeyCbor: PAYMENT,
    ...extra,
  };
}

function argvLog(logPath) {
  return readFileSync(logPath, 'utf8').trim().split('\n').filter(Boolean).map((line) => JSON.parse(line));
}

test('exported buildApprovedFunding refuses Layer S windows before encodeSpendBody', { timeout: 180000 }, () => {
  const record = sourceRecord();
  assert.equal(typeof record.blueprintPath, 'string', `source cache missing for ${record.sha}`);
  assert.equal(existsSync(record.blueprintPath), true, record.blueprintPath);
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-fund-343-'));
  const bin = writeStub(dir);
  process.env.MORIARTY_BODY_LOG = bin.logPath;
  try {
    const funded = buildApprovedFunding(stagesFor(record), requestFor(dir, bin), spec());
    assert.equal(funded.body?.code, 'UNSIGNED_BODY', JSON.stringify({
      body: funded.body?.code,
      mint: funded.mint?.code,
      cash: funded.cashCompile?.code,
      head: funded.headApply?.code,
      posts: funded.posts?.code,
      detail: funded.body?.detail || funded.cashCompile?.detail || funded.headApply?.detail || funded.posts?.detail || null,
    }));
    assert.equal(Object.keys(funded.body.datumFields).length, 13);
    assert.equal(funded.body.ledgerFunding, false);
    assert.equal(funded.mint.cashCanMint, false);
    const posts = funded.posts;
    assert.equal(posts?.code, 'SOURCE_POSTS_EVALUATED', JSON.stringify({ code: posts?.code, detail: posts?.detail }));
    const expectBound = {
      release: ['1000', '1099'],
      refund: ['1100', '1100'],
      cancel: ['1050', '1051'],
    };
    for (const kind of ['release', 'refund', 'cancel']) {
      const spent = funded.spend[kind];
      const terminal = posts.terminals[kind];
      assert.equal(spent.code, 'FIXTURE_WINDOW_UNREPRESENTABLE', JSON.stringify({ kind, code: spent.code, detail: spent.detail }));
      assert.equal(spent.encodeSpendBodyCalled, false);
      assert.equal(spent.cliIntervalEmitted, false);
      assert.equal(spent.bodyBuilt, false);
      assert.equal(spent.bodyFile, null);
      assert.equal(spent.invalidBefore, null);
      assert.deepEqual([spent.redeemerLower, spent.redeemerUpper], expectBound[kind]);
      assert.equal(terminal.constructorWorkCount, 49);
      assert.equal(terminal.collectionCapacity, 128);
      assert.equal(terminal.payoutCash, 100);
      assert.equal(spent.frame?.payoutCash, 100);
      assert.equal(spent.frame?.headQuantity, 1);
      assert.equal(spent.frame?.work?.spent, 49);
      assert.equal(spent.redeemer.amount, 100);
      assert.equal(typeof spent.redeemer.recipient, 'string');
      assert.equal(spent.economicAction?.ledgerExecution, false);
    }
    const calls = argvLog(bin.logPath);
    const rawCalls = calls.filter((row) => row.argv.includes('build-raw'));
    assert.equal(rawCalls.some((row) => row.argv.includes('--tx-in-redeemer-file')), false);
    const head = rawCalls.find((row) => row.argv.includes('--mint') && row.argv.some((item) => String(item).includes('48656164')));
    assert.ok(head, 'head open did not reach the CLI');
    assert.equal(head.argv[head.argv.indexOf('--invalid-before') + 1], '40');
    assert.equal(head.argv[head.argv.indexOf('--invalid-hereafter') + 1], '100');
    const datum = JSON.parse(head.datumText);
    assert.equal(datum.fields.length, 13);
    assert.equal(datum.fields[12].fields.length, 9);
  } finally {
    delete process.env.MORIARTY_BODY_LOG;
  }
});

test('aligned release reaches CLI slots and the redeemer stays POSIX', { timeout: 180000 }, () => {
  const record = sourceRecord();
  assert.equal(existsSync(record.blueprintPath), true);
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-aligned-343-'));
  const bin = writeStub(dir);
  process.env.MORIARTY_BODY_LOG = bin.logPath;
  try {
    const funded = buildApprovedFunding(stagesFor(record), requestFor(dir, bin, {
      representableSpendBounds: { release: { lower: '40000', upper: '99999' } },
    }), spec());
    const release = funded.spend?.release;
    assert.equal(release?.code, 'UNSIGNED_BODY', JSON.stringify({
      code: release?.code,
      detail: release?.detail,
      posts: funded.posts?.code,
    }));
    assert.equal(release.encodeSpendBodyCalled, true);
    assert.equal(typeof release.invalidBefore, 'number');
    assert.equal(release.invalidBefore, 40);
    assert.equal(typeof release.invalidHereafter, 'number');
    assert.equal(release.invalidHereafter, 100);
    assert.equal(release.timeSeparation.redeemerLower, '40000');
    assert.equal(release.timeSeparation.redeemerUpper, '99999');
    assert.equal(funded.spend.refund.encodeSpendBodyCalled, false);
    assert.equal(funded.spend.cancel.encodeSpendBodyCalled, false);
    const calls = argvLog(bin.logPath);
    const spend = calls.find((row) => row.argv.includes('--tx-in-redeemer-file'));
    assert.ok(spend, 'aligned spend did not reach the CLI');
    assert.equal(spend.argv[spend.argv.indexOf('--invalid-before') + 1], '40');
    assert.equal(spend.argv[spend.argv.indexOf('--invalid-hereafter') + 1], '100');
    const redeemer = JSON.parse(spend.redeemerText);
    assert.equal(redeemer.fields[3].int, 40000);
    assert.equal(redeemer.fields[4].int, 99999);
    assert.notEqual(redeemer.fields[3].int, 40);
    assert.equal(release.frame?.work?.spent, 49);
    assert.equal(release.frame?.payoutCash, 100);
  } finally {
    delete process.env.MORIARTY_BODY_LOG;
  }
});

test('exported assembleConsumedFinancial stops the same Layer S windows before encodeSpendBody', { timeout: 180000 }, async () => {
  const record = sourceRecord();
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-assemble-343-'));
  const bin = writeStub(dir);
  const open = datumFromPost({
    programHash: '11'.repeat(32),
    boundsHash: '22'.repeat(32),
    sourceHash: record.sha,
    schemaHash: '44'.repeat(32),
    seedHash: '55'.repeat(32),
  }, '66'.repeat(32), {
    financialPost: {
      schemaVersion: 'moriarty-financial-lifecycle-state/1',
      balances: [
        { party: 'Escrow', asset: 'Cash', amount: '100' },
        { party: 'Borrower', asset: 'Cash', amount: '0' },
        { party: 'Lender', asset: 'Cash', amount: '0' },
      ],
      allowances: [
        { party: 'Escrow', asset: 'Cash', remaining: '100', spent: '0' },
        { party: 'Borrower', asset: 'Cash', remaining: '0', spent: '0' },
        { party: 'Lender', asset: 'Cash', remaining: '0', spent: '0' },
      ],
      obligations: [],
      usedTransferIds: [],
      usedAllocationIds: [],
      usedOriginationIds: [],
      usedAccrualIds: [],
      work: { remaining: '1024', spent: '0', closureReserve: '16' },
    },
    transportRevision: '0',
    transportRemaining: '128',
    post: { locked: '1', revision: '0' },
  });
  const inputs = stagesFor(record).inputs;
  inputs.inputValues = ['cashSeed', 'head', 'mintFee', 'mintCollateral', 'headFee', 'headCollateral'].map((name) => ({
    ref: `${inputs[name].txId}#${inputs[name].index}`,
    lovelaceText: '9000000',
  })).concat(['left', 'right', 'cancel'].flatMap((side) => [
    { ref: `${inputs[side].fee.txId}#${inputs[side].fee.index}`, lovelaceText: '9000000' },
    { ref: `${inputs[side].collateral.txId}#${inputs[side].collateral.index}`, lovelaceText: '9000000' },
  ]));
  writeFileSync(join(dir, 'unit-offline-start.json'), `${JSON.stringify({
    systemStart: '2020-01-01T00:00:00Z',
    unitOnly: true,
  })}\n`);
  process.env.MORIARTY_BODY_LOG = bin.logPath;
  try {
    const assembled = await assembleConsumedFinancial({
      inputs,
      time: { windows: { releaseSlot: '40', partitionSlot: '100' } },
    }, {
      evidenceRoot: dir,
      sourcePath: SOURCE,
      cliPath: bin.stub,
      magic: 764,
      networkMagic: 764,
      paymentVkeyCbor: PAYMENT,
      payoutVkeyCbor: PAYOUT,
      lenderVkeyCbor: PAYOUT,
      borrowerVkeyCbor: `5820${'00112233445566778899aabbccddeeff'.repeat(2)}`,
      detailedData: open.plutus,
      costMagicSource: 'shelley-networkMagic',
      time: { genesisPath: join(dir, 'unit-offline-start.json') },
      ownedProducer: { unitSpawn() { return { status: 0, stdout: '' }; } },
    });
    const spend = assembled.spend;
    if (!spend?.release) {
      assert.fail(JSON.stringify({
        outputFee: assembled.outputFee?.code,
        mint: assembled.mint?.code,
        body: assembled.body?.code,
        rebuild: assembled.rebuild?.code,
        spend: spend?.code || null,
        detail: assembled.mint?.detail || assembled.body?.detail || assembled.rebuild?.detail || assembled.outputFee?.detail || null,
      }));
    }
    for (const kind of ['release', 'refund', 'cancel']) {
      assert.equal(spend[kind].encodeSpendBodyCalled, false);
      assert.equal(spend[kind].code, 'FIXTURE_WINDOW_UNREPRESENTABLE');
      assert.equal(spend[kind].cliIntervalEmitted, false);
      assert.equal(spend[kind].bodyBuilt, false);
      assert.equal(spend[kind].bodyFile ?? null, null);
    }
    assert.equal(spend.release.redeemerLower, '1000');
    assert.equal(spend.release.redeemerUpper, '1099');
    assert.equal(spend.refund.redeemerLower, '1100');
    assert.equal(spend.refund.redeemerUpper, '1100');
    assert.equal(spend.cancel.redeemerLower, '1050');
    assert.equal(spend.cancel.redeemerUpper, '1051');
    const calls = argvLog(bin.logPath);
    assert.equal(calls.some((row) => row.argv.includes('--tx-in-redeemer-file')), false);
  } finally {
    delete process.env.MORIARTY_BODY_LOG;
  }
});
