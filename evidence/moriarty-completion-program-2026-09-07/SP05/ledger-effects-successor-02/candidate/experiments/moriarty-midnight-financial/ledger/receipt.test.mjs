#!/usr/bin/env node
/** Offline ledger-source tests. Synthetic keys only. No network, wallet, proof, or env reads. */
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash, randomBytes} from 'node:crypto';
import {existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname, join, resolve} from 'node:path';
import test from 'node:test';
import {fileURLToPath, pathToFileURL} from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const worktree = resolve(here, '../../..');
const custody = join(worktree, 'experiments/moriarty-midnight-financial/custody');
const bindings = JSON.parse(readFileSync(join(custody, 'bindings.json'), 'utf8'));
const loanFixture = JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-midnight-financial/fixtures/loan.json'), 'utf8'));
const swapFixture = JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-midnight-financial/fixtures/swap.json'), 'utf8'));
const generateUrl = pathToFileURL(join(custody, 'generate.mjs')).href;
const nm = bindings.toolchain.runtimeNodeModules;
const SYNTH = ['b001010101010101010101010101010101010101010101010101010101010101', 'b002020202020202020202020202020202020202020202020202020202020202', 'b003030303030303030303030303030303030303030303030303030303030303', 'b004040404040404040404040404040404040404040404040404040404040404'];
const FORBIDDEN_SRC = ['deploy.ts', 'process.env', 'MORIARTY_', 'hello-world/src/deploy'];

function sha(buf) { return createHash('sha256').update(buf).digest('hex'); }
function src(name) { return readFileSync(join(here, name), 'utf8'); }
async function load(name) { return import(pathToFileURL(join(here, name)).href + '?t=' + Date.now()); }
function mustThrow(p) { return p.then(() => { throw new Error('expected rejection'); }, (e) => e); }
function clone(v) { return JSON.parse(JSON.stringify(v)); }
function setPath(obj, path, value) {
  const parts = path.split('.');
  const last = parts.pop();
  let cur = obj;
  for (const p of parts) {
    if (Array.isArray(cur)) cur = cur[Number(p)];
    else cur = cur[p];
  }
  if (Array.isArray(cur)) cur[Number(last)] = value;
  else cur[last] = value;
}
function gcd(a, b) {
  a = a < 0n ? -a : a; b = b < 0n ? -b : b;
  while (b !== 0n) { const t = a % b; a = b; b = t; }
  return a === 0n ? 1n : a;
}

test('accepted generate.mjs exports frozen names validatePinnedInputs and generateWrappers', async () => {
  const g = await import(generateUrl);
  assert.equal(typeof g.validatePinnedInputs, 'function');
  assert.equal(typeof g.generateWrappers, 'function');
  const pinned = g.validatePinnedInputs({worktree, bindings, readFileSync});
  const generated = g.generateWrappers({worktree, bindings, readFileSync, pinned});
  assert.equal(sha(generated.loanSource), sha(readFileSync(join(custody, 'loan.compact'))));
  assert.equal(sha(generated.swapSource), sha(readFileSync(join(custody, 'swap.compact'))));
});

test('owned sources do not import deploy.ts or read environment values', () => {
  for (const name of ['build-proven.mjs', 'providers.mjs', 'decode-receipt.mjs', 'run-local.mjs']) {
    const text = src(name);
    assert.equal(text.includes('hello-world/src/deploy'), false, name);
    assert.equal(/\bprocess\.env\b/.test(text), false, name + ' env');
  }
});

test('clean imports are inert: no subprocess, wallet, storage, or transport', async () => {
  const before = {exit: process.listenerCount('exit'), uncaught: process.listenerCount('uncaughtException')};
  const mods = await Promise.all(['providers.mjs', 'decode-receipt.mjs', 'build-proven.mjs', 'run-local.mjs'].map(load));
  for (const m of mods) {
    assert.equal(typeof m, 'object');
  }
  assert.equal(process.listenerCount('exit'), before.exit);
  assert.equal(process.listenerCount('uncaughtException'), before.uncaught);
});

test('loader admits only listed present exports and rejects absent compact-js CJS', async () => {
  const p = await load('providers.mjs');
  await p.validatePinnedRuntime({nodeModules: nm, readFileSync, createHash});
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  assert.equal(typeof ledger.sampleSigningKey, 'function');
  assert.equal(typeof ledger.verifySignature, 'function');
  assert.equal(typeof ledger.addressFromKey, 'function');
  assert.equal(typeof ledger.rawTokenType, 'function');
  await assert.rejects(() => p.importPinned('@midnight-ntwrk/compact-js', {
    nodeModules: nm,
    entry: 'dist/cjs/index.js',
  }));
  await assert.rejects(() => p.importPinned('@midnight-ntwrk/not-a-package', {nodeModules: nm}));
});

test('wrong pin hash fails closed before import', async () => {
  const p = await load('providers.mjs');
  const broken = join(mkdtempSync(join(tmpdir(), 'pin-')), 'node_modules');
  mkdirSync(join(broken, '@midnight-ntwrk/ledger-v8'), {recursive: true});
  writeFileSync(join(broken, '@midnight-ntwrk/ledger-v8/package.json'), '{"name":"x"}');
  await assert.rejects(() => p.validatePinnedRuntime({nodeModules: broken, readFileSync, createHash}));
});

test('real pinned signing, address, and token-color APIs with in-memory keys', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const sk = ledger.sampleSigningKey();
  const vk = ledger.signatureVerifyingKey(sk);
  const data = randomBytes(32);
  const sig = ledger.signData(sk, data);
  assert.equal(ledger.verifySignature(vk, data, sig), true);
  assert.equal(ledger.verifySignature(vk, randomBytes(32), sig), false);
  const addr = ledger.addressFromKey(vk);
  assert.equal(typeof addr, 'string');
  assert.equal(SYNTH.includes(String(addr).replace(/^0x/i, '')), false);
  const contract = ledger.sampleContractAddress();
  const color = ledger.rawTokenType(Buffer.from(bindings.loan.usdDomain, 'hex'), contract);
  const other = ledger.rawTokenType(Buffer.from(bindings.swap.assetADomain, 'hex'), contract);
  assert.notEqual(String(color), String(other));
});

test('buildProvenCustody reuses frozen generate names and rejects stale wrappers', async () => {
  const b = await load('build-proven.mjs');
  const tmp = mkdtempSync(join(tmpdir(), 'bp-'));
  const out = join(tmp, 'out');
  await assert.rejects(() => b.buildProvenCustody({
    worktree, bindings, custodyDir: custody, outputDir: out, buildRoot: tmp,
    readFileSync: (f) => {
      if (String(f).endsWith('loan.compact')) return Buffer.from('stale');
      return readFileSync(f);
    },
  }));
});

test('missing full-build admission cannot label proven output', async () => {
  const b = await load('build-proven.mjs');
  const tmp = mkdtempSync(join(tmpdir(), 'bp2-'));
  const r = await b.buildProvenCustody({
    worktree, bindings, custodyDir: custody, outputDir: join(tmp, 'out'), buildRoot: tmp,
    readFileSync, writeFileSync, mkdirSync,
  });
  assert.equal(r.proven, false);
  assert.notEqual(r.status, 'proven');
  assert.equal(r.compilerInvoked, false);
});

test('injected compiler adapter can only emit an unproven test manifest', async () => {
  const b = await load('build-proven.mjs');
  const tmp = mkdtempSync(join(tmpdir(), 'bp3-'));
  const calls = [];
  const r = await b.buildProvenCustody({
    worktree, bindings, custodyDir: custody, outputDir: join(tmp, 'fresh'), buildRoot: tmp,
    fullBuildAdmission: {kind: 'test-adapter-only', allowCompiler: false},
    processAdapter: (argv) => {
      calls.push(argv);
      assert.equal(argv.includes('--skip-zk'), false);
      return {status: 0, stdout: 'adapter', stderr: '', files: {}};
    },
    readFileSync, writeFileSync, mkdirSync, rmSync, existsSync, lstatSync,
  });
  assert.ok(calls.length >= 1);
  assert.equal(r.proven, false);
  assert.equal(r.manifestKind, 'unproven-test-manifest');
  assert.notEqual(r.label, 'proven');
});

test('path traversal and symlink output escape are rejected', async () => {
  const b = await load('build-proven.mjs');
  const tmp = mkdtempSync(join(tmpdir(), 'bp4-'));
  const root = join(tmp, 'root');
  mkdirSync(root);
  await assert.rejects(() => b.buildProvenCustody({
    worktree, bindings, custodyDir: custody, outputDir: join(root, '..', 'escape'), buildRoot: root,
    readFileSync, writeFileSync, mkdirSync,
  }));
  const link = join(root, 'link');
  symlinkSync(tmp, link);
  await assert.rejects(() => b.buildProvenCustody({
    worktree, bindings, custodyDir: custody, outputDir: join(link, 'x'), buildRoot: root,
    readFileSync, writeFileSync, mkdirSync, lstatSync, existsSync,
  }));
});

test('false proven-asset manifest is rejected before wallet calls', async () => {
  const r = await load('run-local.mjs');
  const walletCalls = [];
  const err = await mustThrow(r.runLocalFinancialCase({
    case: 'loan',
    networkAdmission: {logicalTag: 'local', observedProtocol: 'undetermined', bound: false},
    sourceManifest: {bindings},
    provenAssetManifest: {proven: true, files: {}},
    walletContext: {facade: {balanceUnboundTransaction: async () => walletCalls.push('bal')}},
    roleCapabilities: {},
    recipientAddresses: {borrower: 'aa'.repeat(32), lender: 'bb'.repeat(32)},
    callHints: {},
    blockTime: 1700000000,
    eventSink: () => {},
    limits: {attempts: 1, deadlineMs: 1000, spend: 1n},
  }));
  assert.equal(walletCalls.length, 0);
  assert.ok(err);
});

test('unknown network, malformed budget, missing admission, missing roles, invalid time, synthetic recipients fail before wallet', async () => {
  const r = await load('run-local.mjs');
  const wallet = {facade: {balanceUnboundTransaction: async () => { throw new Error('wallet'); }}};
  const base = {
    case: 'loan', walletContext: wallet, sourceManifest: {bindings},
    provenAssetManifest: {proven: false, status: 'unproven'},
    callHints: {accrue: {}, settle: {}}, eventSink: () => {},
    operationalAdmission: {id: 'not-admitted'},
  };
  const cases = [
    {...base, networkAdmission: {logicalTag: 'preview', observedProtocol: 'x', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 1700000000, limits: {attempts: 1, deadlineMs: 10, spend: 1n}},
    {...base, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 1700000000, limits: {attempts: -1, deadlineMs: 10, spend: 1n}},
    {...base, operationalAdmission: null, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 1700000000, limits: {attempts: 1, deadlineMs: 10, spend: 1n}},
    {...base, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 1700000000, limits: {attempts: 1, deadlineMs: 10, spend: 1n}},
    {...base, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 2000000001, limits: {attempts: 1, deadlineMs: 10, spend: 1n}},
    {...base, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: SYNTH[0], lender: SYNTH[1]}, blockTime: 1700000000, limits: {attempts: 1, deadlineMs: 10, spend: 1n}},
  ];
  for (const c of cases) {
    const err = await mustThrow(r.runLocalFinancialCase(c));
    assert.ok(err instanceof Error);
  }
  await assert.rejects(() => r.runLocalFinancialCase({...base, unknownOption: true, networkAdmission: {logicalTag: 'local', observedProtocol: 'observed-local', bound: true}, roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)}, recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)}, blockTime: 1700000000, limits: {attempts: 1, deadlineMs: 10, spend: 1n}}));
});

test('constructor text alone does not bind network observation', async () => {
  const r = await load('run-local.mjs');
  const err = await mustThrow(r.runLocalFinancialCase({
    case: 'loan',
    networkAdmission: {logicalTag: bindings.networkTagAscii, observedProtocol: null, bound: false},
    sourceManifest: {bindings},
    provenAssetManifest: {proven: false},
    walletContext: {facade: {}},
    roleCapabilities: {borrower: randomBytes(32), lender: randomBytes(32)},
    recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)},
    callHints: {},
    blockTime: 1700000000,
    eventSink: () => {},
    limits: {attempts: 1, deadlineMs: 10, spend: 1n},
    operationalAdmission: {id: 'x'},
  }));
  assert.ok(err);
});

function mockPayer(ledger) {
  const sk = ledger.sampleSigningKey();
  const vk = ledger.signatureVerifyingKey(sk);
  const address = ledger.addressFromKey(vk);
  return {sk, vk, address, signData: (payload) => ledger.signData(sk, payload)};
}

function recipeFixture(payer, token, ttl) {
  const input = {intentHash: 'aa'.repeat(32), outputNo: 0, type: String(token), value: 100n, owner: payer.vk};
  const output = {intent: 'aa'.repeat(32), segment: 1, outputNo: 0, type: String(token), value: 90n, owner: payer.address};
  const intent = {
    signatureData: (segment) => Buffer.from(`seg:${segment}:${input.intentHash}:${input.outputNo}`),
    guaranteedUnshieldedOffer: {inputs: [input], outputs: [output], signatures: []},
    fallibleUnshieldedOffer: undefined,
    ttl,
  };
  const tx = {
    intents: new Map([[1, intent]]),
    identifiers: () => ['id-base'],
    transactionHash: () => 'hash-not-an-identifier',
  };
  return {
    type: 'UNBOUND_TRANSACTION',
    baseTransaction: tx,
    balancingTransaction: {
      intents: new Map(),
      identifiers: () => ['id-balance'],
    },
    ttl,
  };
}

test('signed recipe inputs are validated and reserved before finalizeRecipe', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const payer = mockPayer(ledger);
  const token = ledger.rawTokenType(Buffer.from(bindings.loan.usdDomain, 'hex'), ledger.sampleContractAddress());
  const ttl = new Date(Date.now() + 60000);
  const order = [];
  const events = [];
  let pending = false;
  const recipe = recipeFixture(payer, token, ttl);
  const wallet = {
    shieldedSecretKeys: {},
    dustSecretKey: {},
    unshieldedKeystore: {signData: (payload) => { order.push('signData'); return payer.signData(payload); }},
    facade: {
      balanceUnboundTransaction: async () => { order.push('balance'); return recipe; },
      signRecipe: async (rec, cb) => {
        order.push('sign');
        const data = rec.baseTransaction.intents.get(1).signatureData(1);
        rec.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [cb(data)];
        return rec;
      },
      finalizeRecipe: async () => {
        assert.equal(counters.reservedGross > 0n, true, 'finalize before reserve');
        assert.equal(counters.reservedSubmission > 0n, true, 'submission reserved before finalize');
        order.push('finalize');
        pending = true;
        return {
          serialize: () => Uint8Array.from([1, 2, 3]),
          identifiers: () => ['id-final'],
          intents: recipe.baseTransaction.intents,
          transactionHash: () => 'th',
        };
      },
      submitTransaction: async () => { order.push('submit'); return 'submitted-id'; },
    },
  };
  const counters = {submission: 1n, grossSpend: 1000n, reservedSubmission: 0n, reservedGross: 0n};
  const providers = await p.createFinancialProviders({
    walletContext: wallet,
    networkConfig: {indexer: 'mock://indexer', prover: 'mock://prover', node: 'mock://node'},
    provenAssetManifest: {proven: false},
    privateStateLocation: join(mkdtempSync(join(tmpdir(), 'ps-')), 'state'),
    resourceCounters: counters,
    onSubmission: (id) => { order.push('onSubmission:' + id); },
    eventSink: (e) => events.push(e),
    deadlineMs: Date.now() + 5000,
    adapters: {
      CompiledContract: {make: () => ({pipe() { return this; }})},
      NodeZkConfigProvider: class { constructor() {} },
      httpClientProofProvider: () => ({proveTx: async () => { throw new Error('no network'); }}),
      indexerPublicDataProvider: () => ({
        queryContractState: async (_a, cfg) => {
          assert.equal(cfg.type, 'blockHash');
          assert.equal(typeof cfg.blockHash, 'string');
          return null;
        },
        queryUnshieldedBalances: async (_a, cfg) => {
          assert.equal(cfg.type, 'blockHash');
          return [];
        },
      }),
      levelPrivateStateProvider: () => ({set: async () => {}, get: async () => null}),
      deployContract: async () => { throw new Error('no deploy in this test'); },
      submitCallTx: async () => { throw new Error('no call in this test'); },
    },
  });
  recipe.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [];
  const funded = await providers.fundUnshielded({
    tx: recipe.baseTransaction,
    payer,
    ttl,
    allowance: {submission: 1n, grossSpend: 100n},
    tokenType: String(token),
  });
  assert.deepEqual(order.slice(0, 3), ['balance', 'sign', 'signData']);
  assert.ok(order.indexOf('finalize') < order.indexOf('submit'));
  assert.equal(funded.submittedId, 'submitted-id');
  assert.ok(order.includes('onSubmission:submitted-id'));
  assert.equal(pending, true);
  assert.equal(counters.reservedGross > 0n, true);
});

test('wrong payer, duplicate UTXO, extra input, missing signature, changed association fail the validator', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const payer = mockPayer(ledger);
  const other = mockPayer(ledger);
  const token = ledger.rawTokenType(Buffer.from(bindings.loan.usdDomain, 'hex'), ledger.sampleContractAddress());
  const ttl = new Date(Date.now() + 1000);
  const good = recipeFixture(payer, token, ttl);
  const data = good.baseTransaction.intents.get(1).signatureData(1);
  good.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [payer.signData(data)];
  p.validateSignedRecipe(good, payer, {tokenType: String(token), ledger});
  const wrongPayer = recipeFixture(other, token, ttl);
  wrongPayer.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [other.signData(wrongPayer.baseTransaction.intents.get(1).signatureData(1))];
  assert.throws(() => p.validateSignedRecipe(wrongPayer, payer, {tokenType: String(token), ledger}));
  const dup = recipeFixture(payer, token, ttl);
  const inp = dup.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.inputs[0];
  dup.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.inputs = [inp, {...inp}];
  dup.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [payer.signData(dup.baseTransaction.intents.get(1).signatureData(1)), payer.signData(dup.baseTransaction.intents.get(1).signatureData(1))];
  assert.throws(() => p.validateSignedRecipe(dup, payer, {tokenType: String(token), ledger}));
  const missing = recipeFixture(payer, token, ttl);
  missing.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [];
  assert.throws(() => p.validateSignedRecipe(missing, payer, {tokenType: String(token), ledger}));
  const swapped = recipeFixture(payer, token, ttl);
  swapped.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [other.signData(data)];
  assert.throws(() => p.validateSignedRecipe(swapped, payer, {tokenType: String(token), ledger}));
});

test('finalization failure records known identifiers as unsubmitted-but-pending and stops', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const payer = mockPayer(ledger);
  const token = ledger.rawTokenType(Buffer.from(bindings.loan.usdDomain, 'hex'), ledger.sampleContractAddress());
  const ttl = new Date(Date.now() + 1000);
  const recipe = recipeFixture(payer, token, ttl);
  const events = [];
  const wallet = {
    unshieldedKeystore: {signData: (payload) => payer.signData(payload)},
    facade: {
      balanceUnboundTransaction: async () => recipe,
      signRecipe: async (rec, cb) => {
        rec.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [cb(rec.baseTransaction.intents.get(1).signatureData(1))];
        return rec;
      },
      finalizeRecipe: async () => { throw new Error('finalize failed after pending register'); },
      submitTransaction: async () => { throw new Error('must not retry'); },
    },
  };
  const providers = await p.createFinancialProviders({
    walletContext: wallet,
    networkConfig: {indexer: 'mock://i', prover: 'mock://p', node: 'mock://n'},
    provenAssetManifest: {proven: false},
    privateStateLocation: join(mkdtempSync(join(tmpdir(), 'ps2-')), 's'),
    resourceCounters: {submission: 2n, grossSpend: 1000n, reservedSubmission: 0n, reservedGross: 0n},
    onSubmission: () => { throw new Error('no submit'); },
    eventSink: (e) => events.push(e),
    deadlineMs: Date.now() + 2000,
    adapters: {
      CompiledContract: {make: () => ({pipe() { return this; }})},
      NodeZkConfigProvider: class {},
      httpClientProofProvider: () => ({}),
      indexerPublicDataProvider: () => ({queryContractState: async () => null, queryUnshieldedBalances: async () => []}),
      levelPrivateStateProvider: () => ({}),
    },
  });
  const err = await mustThrow(providers.fundUnshielded({
    tx: recipe.baseTransaction, payer, ttl, allowance: {submission: 1n, grossSpend: 100n}, tokenType: String(token),
  }));
  assert.ok(err);
  const rec = events.find((e) => e && e.kind === 'unsubmitted-but-pending');
  assert.ok(rec, 'lifecycle record missing');
  assert.ok(rec.identifiers.includes('id-base') || rec.identifiers.includes('id-balance') || rec.pendingState === 'pending-state-unknown');
  assert.equal(rec.reservationRetained, true);
  assert.equal(rec.retried, false);
});

test('submit ID is published before finality wait; timeout and indexer rejection retain identifiers', async () => {
  const p = await load('providers.mjs');
  const events = [];
  const order = [];
  const wrapped = p.wrapMidnightSubmit({
    midnightProvider: {
      submitTx: async () => { order.push('submitTx'); return 'net-id'; },
    },
    eventSink: (e) => { events.push(e); order.push('sink:' + e.kind); },
    onSubmission: (id) => { order.push('onSubmission:' + id); },
    waitFinality: async () => { order.push('finality'); throw new Error('timeout'); },
    identifiers: () => ['net-id', 'local-id'],
    counters: {reservedSubmission: 1n},
  });
  const err = await mustThrow(wrapped.submitTx({}));
  assert.ok(err);
  assert.ok(order.indexOf('onSubmission:net-id') < order.indexOf('finality'));
  assert.ok(events.some((e) => e.identifiers && e.identifiers.includes('net-id')));
});

test('FailFallible retains guaranteed effects and does not continue a success sequence', async () => {
  const d = await load('decode-receipt.mjs');
  const rec = await d.decodePublicFinancialReceipt({
    provenance: 'synthetic-test',
    finalizedTxData: {
      status: 'FailFallible',
      txId: 't1',
      identifiers: ['t1'],
      txHash: 'h',
      blockHash: 'bb'.repeat(32),
      blockHeight: 1,
      fees: {paidFees: '1', estimatedFees: '1'},
      segmentStatusMap: new Map([[1, 'SegmentSuccess'], [2, 'SegmentFail']]),
      unshielded: {created: [], spent: []},
      tx: null,
    },
    serializedTx: null,
    contractAddress: 'cc'.repeat(32),
    publicState: {kernelState: {notional: '5000000000'}, remaining: '1', revision: '1', lastResult: {}, residualDuty: [], work: '1'},
  });
  assert.equal(rec.acceptedStage, false);
  assert.equal(rec.txStatus, 'FailFallible');
  assert.ok(rec.guaranteedEffectsRetained);
  const v = d.validatePublicFinancialReceipt(rec);
  assert.equal(v.ok, false);
});

test('mocked finalize object fails the production finalized-byte decoder', async () => {
  const d = await load('decode-receipt.mjs');
  const fake = {status: 'SucceedEntirely', tx: {serialize: () => new Uint8Array([0]), identifiers: () => ['x']}, identifiers: ['x'], txId: 'x', txHash: 'h', blockHash: 'b', fees: {paidFees: '0', estimatedFees: '0'}};
  await assert.rejects(() => Promise.resolve(d.decodePublicFinancialReceipt({
    provenance: 'finalized-bytes',
    finalizedTxData: fake,
    serializedTx: fake.tx.serialize(),
    contractAddress: 'cc'.repeat(32),
  })));
});

test('CallTxFailedError public extractor does not serialize private fields', async () => {
  const d = await load('decode-receipt.mjs');
  const err = {
    name: 'CallTxFailedError',
    finalizedTxData: {status: 'FailEntirely', identifiers: ['z'], txId: 'z', txHash: 'h', blockHash: 'b', fees: {paidFees: '0', estimatedFees: '0'}, tx: null},
    private: {witness: 'SECRET'},
    message: 'failed',
  };
  const pub = d.extractPublicFailedTxData(err);
  const dumped = JSON.stringify(pub);
  assert.equal(dumped.includes('SECRET'), false);
  assert.equal(pub.finalizedTxData.status, 'FailEntirely');
});

test('BlockHashConfig uses type blockHash and blockHash field', async () => {
  const p = await load('providers.mjs');
  const cfg = p.blockHashConfig('ab'.repeat(32));
  assert.deepEqual(cfg, {type: 'blockHash', blockHash: 'ab'.repeat(32)});
  assert.equal('blockHeight' in cfg, false);
});

function observedLoanReceipt() {
  return {
    provenance: 'synthetic-test',
    contractAddress: '11'.repeat(32),
    blockHash: '22'.repeat(32),
    txStatus: 'SucceedEntirely',
    acceptedStage: true,
    identifiers: ['i1'],
    fees: {paidFees: '0', estimatedFees: '0', knownAggregate: '0', unobservedPrivateDust: true},
    programDigest: bindings.loan.programDigest,
    kernelState: {
      notional: '4500000000',
      principal_due: '0',
      interest_due: '0',
      principal_paid: '500000000',
      interest_paid: '33972602',
      borrower_cash: '19466027398',
      lender_cash: '533972602',
      cursor: '2',
      episode_closed: '1',
    },
    remaining: '0',
    revision: '2',
    work: '0',
    lastResult: {},
    residualDuty: [{id: 'remaining-notional', kind: 'nominal-agreement-debt', amount: '4500000000', unit: 'USD_micro', status: 'open'}],
    minted: [{color: 'USD_TEST_ASSET', value: '20000000000'}],
    balances: {
      borrower: '19466027398',
      lender: '533972602',
    },
    transfers: [{from: 'borrower', to: 'lender', amount: '533972602', color: 'USD_TEST_ASSET'}],
    stagesObserved: {
      setup: {notional: '5000000000', borrower_cash: '20000000000', lender_cash: '0', remaining: '2', revision: '0', work: '2', cursor: '0', episode_closed: '0', principal_due: '0', interest_due: '0', principal_paid: '0', interest_paid: '0'},
      accrue: {notional: '4500000000', principal_due: '500000000', interest_due: '33972602', remaining: '1', revision: '1', work: '1', cursor: '1', episode_closed: '0', borrower_cash: '20000000000', lender_cash: '0', principal_paid: '0', interest_paid: '0'},
      settle: {notional: '4500000000', principal_due: '0', interest_due: '0', principal_paid: '500000000', interest_paid: '33972602', borrower_cash: '19466027398', lender_cash: '533972602', remaining: '0', revision: '2', work: '0', cursor: '2', episode_closed: '1'},
    },
  };
}

test('oracle projection uses .mori constants, explicit provenance, and comparator always returns networkAcceptance false', async () => {
  const d = await load('decode-receipt.mjs');
  const {compareFinancialEffects} = await import(pathToFileURL(join(worktree, 'experiments/moriarty-midnight-financial/src/differential.mjs')).href);
  const receipt = observedLoanReceipt();
  const proj = d.projectFinancialOracle({
    case: 'loan',
    receipt,
    expectedFixture: loanFixture,
    sourceText: readFileSync(join(worktree, 'experiments/moriarty-language/spec/examples/loan.mori'), 'utf8'),
    sourceSha256: bindings.digests.loanSource,
    metadata: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/loan/metadata.json'), 'utf8')),
    metadataSha256: bindings.digests.loanMetadata,
    boundProgram: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/loan/bound-program.json'), 'utf8')),
    deployedProgramDigest: bindings.loan.programDigest,
  });
  assert.equal(proj.oracleRecord.networkAcceptance, false);
  assert.equal(proj.oracleRecord.observationKind, 'synthetic-local');
  assert.equal(proj.oracleRecord.networkEvidence, 'incompleteNetworkEvidence');
  assert.equal(proj.oracleRecord.sourcePins.metadataUsedForExpectations, false);
  assert.equal(proj.oracleRecord.schemaVersion, 'moriarty-financial-record/1');
  const n = 5000000000n;
  const remainder = (n * 8n * 31n) % (100n * 365n);
  const den = 100n * 365n;
  const g = gcd(remainder, den);
  assert.equal(remainder, 27000n);
  assert.equal(remainder / g, 54n);
  assert.equal(den / g, 73n);
  const leaf = proj.provenanceMap['stages.accrue.liabilities.accrual.floorRemainderNumerator'];
  assert.equal(leaf.kind, 'arithmetic-derived');
  const rate = proj.provenanceMap['stages.accrue.liabilities.accrual.rateNumerator'];
  assert.equal(rate.kind, 'source-derived');
  assert.equal(rate.path.includes('loan.mori'), true);
  const paid = proj.provenanceMap['stages.settle.liabilities.principal.paid'];
  assert.equal(paid.kind, 'observed');
  const cmp = compareFinancialEffects(loanFixture, proj.oracleRecord);
  assert.equal(cmp.networkAcceptance, false);
  assert.equal(cmp.ok, true, JSON.stringify(cmp.errors.slice(0, 8)));
  assert.notEqual(proj.realReceipt, proj.oracleRecord);
});

test('financial leaf mutations fail projection or independent receipt predicate, not mutationId', async () => {
  const d = await load('decode-receipt.mjs');
  const {compareFinancialEffects} = await import(pathToFileURL(join(worktree, 'experiments/moriarty-midnight-financial/src/differential.mjs')).href);
  const mutations = [
    ['stagesObserved.settle.lender_cash', '1'],
    ['stagesObserved.accrue.interest_due', '1'],
    ['stagesObserved.setup.borrower_cash', '1'],
    ['kernelState.notional', '1'],
    ['remaining', '9'],
    ['revision', '9'],
    ['work', '9'],
    ['txStatus', 'FailEntirely'],
    ['blockHash', '00'.repeat(32)],
    ['contractAddress', '00'.repeat(32)],
    ['programDigest', '00'.repeat(32)],
    ['fees.paidFees', '99'],
    ['residualDuty.0.amount', '1'],
    ['balances.borrower', '1'],
    ['transfers.0.amount', '1'],
    ['stagesObserved.accrue.principal_due', '1'],
    ['kernelState.episode_closed', '0'],
  ];
  for (const [path, value] of mutations) {
    const mutated = clone(observedLoanReceipt());
    setPath(mutated, path, value);
    let failed = false;
    try {
      const proj = d.projectFinancialOracle({
        case: 'loan',
        receipt: mutated,
        expectedFixture: loanFixture,
        sourceText: readFileSync(join(worktree, 'experiments/moriarty-language/spec/examples/loan.mori'), 'utf8'),
        sourceSha256: bindings.digests.loanSource,
        metadata: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/loan/metadata.json'), 'utf8')),
        metadataSha256: bindings.digests.loanMetadata,
        boundProgram: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/loan/bound-program.json'), 'utf8')),
        deployedProgramDigest: bindings.loan.programDigest,
      });
      const indep = d.validatePublicFinancialReceipt(mutated);
      const cmp = compareFinancialEffects(loanFixture, proj.oracleRecord);
      failed = cmp.ok === false || indep.ok === false || proj.ok === false;
    } catch {
      failed = true;
    }
    assert.equal(failed, true, 'mutation at ' + path + ' was not detected by validator');
  }
});

test('swap projection arithmetic and close reserves come from observed state plus swap.mori constants', async () => {
  const d = await load('decode-receipt.mjs');
  const {compareFinancialEffects} = await import(pathToFileURL(join(worktree, 'experiments/moriarty-midnight-financial/src/differential.mjs')).href);
  const receipt = {
    provenance: 'synthetic-test',
    contractAddress: '33'.repeat(32),
    blockHash: '44'.repeat(32),
    txStatus: 'SucceedEntirely',
    acceptedStage: true,
    identifiers: ['s1'],
    fees: {paidFees: '0', estimatedFees: '0', knownAggregate: '0', unobservedPrivateDust: true},
    programDigest: bindings.swap.programDigest,
    kernelState: {
      reserve_a: '0', reserve_b: '0', trader_a: '90000', trader_b: '19743',
      provider_a: '1010000', provider_b: '1980257', epoch_closed: '1',
    },
    remaining: '6', revision: '2', work: '6', lastResult: {}, residualDuty: [{id: 'reserve-for-close', kind: 'action-allowance-reservation', amount: '1', unit: 'action', status: 'discharged'}],
    minted: [],
    balances: {traderA: '90000', traderB: '19743', providerA: '1010000', providerB: '1980257', poolA: '0', poolB: '0'},
    transfers: [],
    stagesObserved: {
      setup: {reserve_a: '1000000', reserve_b: '2000000', trader_a: '100000', trader_b: '0', provider_a: '0', provider_b: '0', epoch_closed: '0', remaining: '8', revision: '0', work: '8'},
      swap: {reserve_a: '1010000', reserve_b: '1980257', trader_a: '90000', trader_b: '19743', provider_a: '0', provider_b: '0', epoch_closed: '0', remaining: '7', revision: '1', work: '7'},
      close: {reserve_a: '0', reserve_b: '0', trader_a: '90000', trader_b: '19743', provider_a: '1010000', provider_b: '1980257', epoch_closed: '1', remaining: '6', revision: '2', work: '6'},
    },
  };
  const proj = d.projectFinancialOracle({
    case: 'swap',
    receipt,
    expectedFixture: swapFixture,
    sourceText: readFileSync(join(worktree, 'experiments/moriarty-language/spec/examples/swap.mori'), 'utf8'),
    sourceSha256: bindings.digests.swapSource,
    metadata: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/swap/metadata.json'), 'utf8')),
    metadataSha256: bindings.digests.swapMetadata,
    boundProgram: JSON.parse(readFileSync(join(worktree, 'experiments/moriarty-language/compact/generated/swap/bound-program.json'), 'utf8')),
    deployedProgramDigest: bindings.swap.programDigest,
  });
  const cmp = compareFinancialEffects(swapFixture, proj.oracleRecord);
  assert.equal(cmp.networkAcceptance, false);
  assert.equal(cmp.ok, true, JSON.stringify(cmp.errors.slice(0, 8)));
  const feeN = proj.provenanceMap['source.fee_numerator'] || proj.provenanceMap['fee_numerator'];
  assert.ok(feeN === undefined || feeN.kind === 'source-derived');
});

test('query helpers pass BlockHashConfig to PublicDataProvider signatures', async () => {
  const p = await load('providers.mjs');
  const seen = [];
  const adapters = {
    CompiledContract: {make: () => ({pipe() { return this; }})},
    NodeZkConfigProvider: class {},
    httpClientProofProvider: () => ({}),
    indexerPublicDataProvider: () => ({
      queryContractState: async (addr, cfg) => { seen.push(['state', addr, cfg]); return {data: {}}; },
      queryUnshieldedBalances: async (addr, cfg) => { seen.push(['bal', addr, cfg]); return []; },
    }),
    levelPrivateStateProvider: () => ({}),
  };
  const providers = await p.createFinancialProviders({
    walletContext: {facade: {}, unshieldedKeystore: {signData: () => {}}},
    networkConfig: {indexer: 'mock://i', prover: 'mock://p', node: 'mock://n'},
    provenAssetManifest: {proven: false},
    privateStateLocation: join(mkdtempSync(join(tmpdir(), 'ps3-')), 's'),
    resourceCounters: {submission: 1n, grossSpend: 1n, reservedSubmission: 0n, reservedGross: 0n},
    onSubmission: () => {},
    eventSink: () => {},
    deadlineMs: Date.now() + 1000,
    adapters,
  });
  await providers.queryPublic('abcd', 'ee'.repeat(32));
  assert.equal(seen.length, 2);
  assert.deepEqual(seen[0][2], {type: 'blockHash', blockHash: 'ee'.repeat(32)});
  assert.deepEqual(seen[1][2], {type: 'blockHash', blockHash: 'ee'.repeat(32)});
});

test('CLI explains missing operational admission and does not invent identities', async () => {
  const r = spawnSync(process.execPath, [join(here, 'run-local.mjs'), '--explain'], {encoding: 'utf8', env: {PATH: process.env.PATH}, timeout: 10000});
  assert.equal(r.status, 0);
  assert.match(r.stdout, /operational admission/i);
  assert.equal(r.stdout.includes(SYNTH[0]), false);
});

test('zero implicit retries after submission failure', async () => {
  const p = await load('providers.mjs');
  let submits = 0;
  const wrapped = p.wrapMidnightSubmit({
    midnightProvider: {submitTx: async () => { submits += 1; throw new Error('ambiguous'); }},
    eventSink: () => {},
    onSubmission: () => {},
    waitFinality: async () => {},
    identifiers: () => ['a'],
    counters: {reservedSubmission: 1n},
  });
  await mustThrow(wrapped.submitTx({}));
  assert.equal(submits, 1);
});

// Native ledger-v8 objects, synthetic UTXO references only. Pre-proof and
// pre-binding. These do not establish valid finalized bytes or ledger acceptance.
function nativeUtxo(ledger, vk, token, value, outputNo = 0) {
  return {value, owner: vk, type: token, intentHash: ledger.sampleIntentHash(), outputNo};
}
function signUnshieldedTx(ledger, tx, sk) {
  const intents = tx.intents;
  for (const [segment, intent] of intents) {
    const offer = intent.guaranteedUnshieldedOffer;
    if (offer && offer.inputs.length) {
      intent.guaranteedUnshieldedOffer = offer.addSignatures([ledger.signData(sk, intent.signatureData(segment))]);
      intents.set(segment, intent);
    }
  }
  tx.intents = intents;
  return tx;
}
function nativeIntent(ledger, {ttl, vk, owner, token, valueIn, valueOut, fallibleOut, extraGuaranteed}) {
  const intent = ledger.Intent.new(ttl);
  const outputs = [{value: valueOut, owner, type: token}];
  if (extraGuaranteed) outputs.push(extraGuaranteed);
  intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new([nativeUtxo(ledger, vk, token, valueIn)], outputs, []);
  if (fallibleOut != null) {
    intent.fallibleUnshieldedOffer = ledger.UnshieldedOffer.new([], [{value: fallibleOut, owner, type: token}], []);
  }
  return intent;
}
function nativeBaseTx(ledger, keys, token, ttl, opts = {}) {
  const intent = nativeIntent(ledger, {
    ttl, vk: keys.vk, owner: keys.address, token,
    valueIn: opts.valueIn ?? 100n, valueOut: opts.valueOut ?? 90n,
    fallibleOut: opts.fallibleOut ?? 7n, extraGuaranteed: opts.extraGuaranteed,
  });
  return signUnshieldedTx(ledger, ledger.Transaction.fromParts('undeployed', undefined, undefined, intent), keys.sk);
}
function nativeBalancingTx(ledger, keys, token, ttl, segment = 2) {
  const intent = nativeIntent(ledger, {
    ttl, vk: keys.vk, owner: keys.address, token,
    valueIn: 50n, valueOut: 40n, fallibleOut: 3n,
  });
  const tx = ledger.Transaction.fromParts('undeployed').addIntent({tag: 'specific', value: segment}, intent);
  return signUnshieldedTx(ledger, tx, keys.sk);
}

test('native signed recipe validates and unchanged pre-proof copy compares', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const tx = nativeBaseTx(ledger, keys, token, ttl);
  const recipe = {type: 'UNPROVEN_TRANSACTION', transaction: tx, baseTransaction: tx, ttl};
  assert.deepEqual(p.validateSignedRecipe(recipe, keys, {ledger, tokenType: token}), {ok: true, inputCount: 1, signed: 1});
  const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', tx.serialize());
  assert.deepEqual(copy.serialize(), tx.serialize());
  assert.equal(p.recheckFinalizedAgainstRecipe(copy, recipe).ok, true);
  assert.equal(p.recheckFinalizedAgainstRecipe(copy, {type: 'UNPROVEN_TRANSACTION', transaction: tx, ttl}).ok, true);
});

test('native comparison rejects changed fallible recipient and amount; signatures independently fail', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  for (const change of ['recipient', 'amount']) {
    const tx = nativeBaseTx(ledger, keys, token, ttl);
    const recipe = {type: 'UNPROVEN_TRANSACTION', transaction: tx, baseTransaction: tx, ttl};
    assert.equal(p.validateSignedRecipe(recipe, keys, {ledger, tokenType: token}).ok, true);
    const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', tx.serialize());
    const intents = copy.intents;
    const [segment, intent] = [...intents][0];
    const outputs = intent.fallibleUnshieldedOffer.outputs;
    if (change === 'recipient') outputs[0].owner = ledger.sampleUserAddress();
    else outputs[0].value += 1n;
    intent.fallibleUnshieldedOffer = ledger.UnshieldedOffer.new([], outputs, []);
    intents.set(segment, intent);
    copy.intents = intents;
    assert.throws(() => p.validateSignedRecipe({baseTransaction: copy, ttl}, keys, {ledger, tokenType: token}), /signature verify failed/);
    assert.throws(() => p.recheckFinalizedAgainstRecipe(copy, recipe), /fallible output identity diverged/);
  }
});

test('native merge of disjoint base and balancing intents is the accepted combination', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const base = nativeBaseTx(ledger, keys, token, ttl);
  const balancing = nativeBalancingTx(ledger, keys, token, ttl, 2);
  const recipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: base, balancingTransaction: balancing, ttl};
  assert.equal(p.validateSignedRecipe(recipe, keys, {ledger, tokenType: token}).ok, true);
  const merged = base.merge(balancing);
  assert.deepEqual([...merged.intents.keys()], [1, 2]);
  assert.equal(p.recheckFinalizedAgainstRecipe(merged, recipe).ok, true);
  const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', merged.serialize());
  assert.equal(p.recheckFinalizedAgainstRecipe(copy, recipe).ok, true);
});

test('native comparison rejects changed balancing recipient, omitted balancing, and extra intent', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const recipeOf = () => {
    const base = nativeBaseTx(ledger, keys, token, ttl);
    const balancing = nativeBalancingTx(ledger, keys, token, ttl, 2);
    return {recipe: {type: 'UNBOUND_TRANSACTION', baseTransaction: base, balancingTransaction: balancing, ttl}, base, balancing};
  };
  {
    const {recipe, base, balancing} = recipeOf();
    const merged = base.merge(balancing);
    const intents = merged.intents;
    const intent = intents.get(2);
    const outputs = intent.guaranteedUnshieldedOffer.outputs;
    outputs[0].owner = ledger.sampleUserAddress();
    intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new(intent.guaranteedUnshieldedOffer.inputs, outputs, intent.guaranteedUnshieldedOffer.signatures);
    intents.set(2, intent);
    merged.intents = intents;
    assert.throws(() => p.validateSignedRecipe({baseTransaction: merged}, keys, {ledger, tokenType: token}), /signature verify failed/);
    assert.throws(() => p.recheckFinalizedAgainstRecipe(merged, recipe), /guaranteed output identity diverged/);
  }
  {
    const {recipe, base} = recipeOf();
    assert.throws(() => p.recheckFinalizedAgainstRecipe(base, recipe), /finalized missing intent segment 2|finalized intent set diverged/);
  }
  {
    const {recipe, base, balancing} = recipeOf();
    const extra = nativeBalancingTx(ledger, keys, token, ttl, 3);
    const merged = base.merge(balancing).merge(extra);
    assert.throws(() => p.recheckFinalizedAgainstRecipe(merged, recipe), /extra intent|intent set diverged/);
  }
});

test('colliding recipe segment mapping is rejected; output order is not sorted away', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const a = nativeBaseTx(ledger, keys, token, ttl);
  const b = nativeBaseTx(ledger, keys, token, ttl);
  assert.throws(() => p.recheckFinalizedAgainstRecipe(a, {type: 'UNBOUND_TRANSACTION', baseTransaction: a, balancingTransaction: b, ttl}), /colliding recipe intent segment/);
  const extra = {value: 5n, owner: keys.address, type: token};
  const twoOut = nativeBaseTx(ledger, keys, token, ttl, {extraGuaranteed: extra, fallibleOut: null});
  const recipe = {type: 'UNPROVEN_TRANSACTION', transaction: twoOut, ttl};
  assert.equal(p.recheckFinalizedAgainstRecipe(twoOut, recipe).ok, true);
  const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', twoOut.serialize());
  const intents = copy.intents;
  const [segment, intent] = [...intents][0];
  const outputs = intent.guaranteedUnshieldedOffer.outputs;
  assert.equal(outputs.length, 2);
  outputs[0].owner = ledger.sampleUserAddress();
  intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new(
    intent.guaranteedUnshieldedOffer.inputs, outputs, intent.guaranteedUnshieldedOffer.signatures,
  );
  intents.set(segment, intent);
  copy.intents = intents;
  assert.throws(() => p.validateSignedRecipe({baseTransaction: copy}, keys, {ledger, tokenType: token}), /signature verify failed/);
  assert.throws(() => p.recheckFinalizedAgainstRecipe(copy, recipe), /guaranteed output identity diverged/);
  const syn = {
    ttl: 1700000100,
    guaranteedUnshieldedOffer: {
      inputs: [],
      outputs: [{outputNo: 0, type: 'A', value: 1n, owner: 'x'}, {outputNo: 1, type: 'B', value: 2n, owner: 'y'}],
      signatures: [],
    },
  };
  const rec = {baseTransaction: {intents: new Map([[1, syn]])}};
  assert.equal(p.recheckFinalizedAgainstRecipe({intents: new Map([[1, structuredClone(syn)]])}, rec).ok, true);
  const swapped = structuredClone(syn);
  swapped.guaranteedUnshieldedOffer.outputs = [swapped.guaranteedUnshieldedOffer.outputs[1], swapped.guaranteedUnshieldedOffer.outputs[0]];
  assert.throws(() => p.recheckFinalizedAgainstRecipe({intents: new Map([[1, swapped]])}, rec), /guaranteed output identity diverged/);
});

test('synthetic complete-effects helper rejects fallible, balancing, extra, omitted, and missing ttl', async () => {
  const p = await load('providers.mjs');
  function intent(segment, owner = 'approved-recipient') {
    return {
      ttl: 1700000100,
      guaranteedUnshieldedOffer: {
        inputs: [], outputs: [{segment, outputNo: 0, type: 'asset-A', value: 12n, owner}], signatures: [],
      },
      fallibleUnshieldedOffer: {
        inputs: [], outputs: [{segment, outputNo: 1, type: 'asset-B', value: 7n, owner}], signatures: [],
      },
    };
  }
  function fixture() {
    const baseTransaction = {intents: new Map([[1, intent(1)]])};
    const balancingTransaction = {intents: new Map([[2, intent(2)]])};
    const recipe = {baseTransaction, balancingTransaction};
    const finalized = {intents: new Map([...structuredClone(baseTransaction.intents), ...structuredClone(balancingTransaction.intents)])};
    return {recipe, finalized};
  }
  const control = fixture();
  assert.equal(p.recheckFinalizedAgainstRecipe(control.finalized, control.recipe).ok, true);
  const cases = [
    ['fallible recipient', f => { f.intents.get(1).fallibleUnshieldedOffer.outputs[0].owner = 'unapproved-recipient'; }, /fallible output identity diverged/],
    ['fallible amount', f => { f.intents.get(1).fallibleUnshieldedOffer.outputs[0].value = 700n; }, /fallible output identity diverged/],
    ['balancing recipient', f => { f.intents.get(2).guaranteedUnshieldedOffer.outputs[0].owner = 'unapproved-recipient'; }, /guaranteed output identity diverged/],
    ['additional intent', f => { f.intents.set(3, intent(3, 'unapproved-recipient')); }, /extra intent|intent set diverged/],
    ['missing balancing intent', f => { f.intents.delete(2); }, /missing intent segment 2|intent set diverged/],
    ['missing expiration', f => { delete f.intents.get(1).ttl; }, /ttl missing/],
  ];
  for (const [, mutate, re] of cases) {
    const {recipe, finalized} = fixture();
    mutate(finalized);
    assert.throws(() => p.recheckFinalizedAgainstRecipe(finalized, recipe), re);
  }
});

function zswapContractOffer(ledger, token, value, segment, contract) {
  return ledger.ZswapOffer.fromOutput(
    ledger.ZswapOutput.newContractOwned(ledger.createShieldedCoinInfo(token, value), segment, contract),
  );
}

async function inertFundUnshielded(p, {recipe, payer, token, ttl, cap, declared}) {
  const counters = {submission: 1n, grossSpend: cap, reservedSubmission: 0n, reservedGross: 0n};
  const calls = [];
  const events = [];
  const providers = await p.createFinancialProviders({
    walletContext: {
      unshieldedKeystore: {signData: (payload) => payer.signData(payload)},
      facade: {
        balanceUnboundTransaction: async () => recipe,
        signRecipe: async () => recipe,
        finalizeRecipe: async () => {
          calls.push('inert-finalize-stub');
          throw new Error('LOCAL_FINALIZE_BOUNDARY');
        },
        submitTransaction: async () => { throw new Error('UNREACHABLE_SUBMIT'); },
      },
    },
    networkConfig: {indexer: 'inert:', prover: 'inert:'},
    privateStateLocation: join(mkdtempSync(join(tmpdir(), 'psg-')), 's'),
    provenAssetManifest: {proven: false},
    resourceCounters: counters,
    onSubmission: () => { throw new Error('UNREACHABLE_SUBMISSION'); },
    eventSink: (e) => events.push(e),
    adapters: {
      CompiledContract: {}, NodeZkConfigProvider: {}, httpClientProofProvider: {},
      indexerPublicDataProvider: {}, levelPrivateStateProvider: {},
      deployContract: {}, submitCallTx: {}, findDeployedContract: {},
    },
  });
  let error;
  try {
    await providers.fundUnshielded({
      tx: recipe.baseTransaction || recipe.transaction,
      payer,
      ttl,
      allowance: {submission: 1n, ...(declared === undefined ? {} : {grossSpend: declared})},
      tokenType: token,
    });
  } catch (e) { error = e; }
  return {error, counters, calls, events};
}

test('native unchanged Zswap and bind/merge stay accepted; altered zswap and network reject', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const contract = ledger.sampleContractAddress();
  const tx = nativeBaseTx(ledger, keys, token, ttl);
  tx.guaranteedOffer = zswapContractOffer(ledger, token, 5n, undefined, contract);
  const recipe = {type: 'UNPROVEN_TRANSACTION', transaction: tx, ttl};
  assert.equal(p.validateSignedRecipe({baseTransaction: tx, ttl}, keys, {ledger, tokenType: token}).ok, true);
  const copy = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', tx.serialize());
  assert.equal(p.recheckFinalizedAgainstRecipe(copy, recipe).ok, true);
  const bound = tx.bind();
  assert.equal(Buffer.from(bound.serialize()).equals(Buffer.from(tx.serialize())), false);
  assert.equal(p.recheckFinalizedAgainstRecipe(bound, recipe).ok, true);

  const preview = ledger.Transaction.fromParts('preview');
  preview.intents = tx.intents;
  preview.guaranteedOffer = tx.guaranteedOffer;
  assert.equal(Buffer.from(preview.serialize()).equals(Buffer.from(tx.serialize())), false);
  assert.equal(Buffer.from(preview.intents.get(1).signatureData(1)).equals(Buffer.from(tx.intents.get(1).signatureData(1))), true);
  assert.throws(() => p.recheckFinalizedAgainstRecipe(preview, recipe));

  for (const change of ['remove', 'amount', 'recipient', 'add-fallible']) {
    const source = nativeBaseTx(ledger, keys, token, ttl);
    source.guaranteedOffer = zswapContractOffer(ledger, token, 5n, undefined, contract);
    const rec = {type: 'UNPROVEN_TRANSACTION', transaction: source, ttl};
    const mutated = ledger.Transaction.deserialize('signature', 'pre-proof', 'pre-binding', source.serialize());
    if (change === 'remove') mutated.guaranteedOffer = undefined;
    if (change === 'amount') mutated.guaranteedOffer = zswapContractOffer(ledger, token, 6n, undefined, contract);
    if (change === 'recipient') mutated.guaranteedOffer = zswapContractOffer(ledger, token, 5n, undefined, ledger.sampleContractAddress());
    if (change === 'add-fallible') mutated.fallibleOffer = new Map([[1, zswapContractOffer(ledger, token, 5n, 1, contract)]]);
    assert.equal(Buffer.from(mutated.serialize()).equals(Buffer.from(source.serialize())), false);
    assert.equal(Buffer.from(mutated.intents.get(1).signatureData(1)).equals(Buffer.from(source.intents.get(1).signatureData(1))), true);
    assert.throws(() => p.recheckFinalizedAgainstRecipe(mutated, rec));
  }

  const base = nativeBaseTx(ledger, keys, token, ttl);
  const balancing = nativeBalancingTx(ledger, keys, token, ttl, 2);
  base.guaranteedOffer = zswapContractOffer(ledger, token, 5n, undefined, contract);
  balancing.fallibleOffer = new Map([[2, zswapContractOffer(ledger, token, 7n, 2, contract)]]);
  const mergedRecipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: base, balancingTransaction: balancing, ttl};
  const merged = base.merge(balancing);
  assert.deepEqual([...merged.intents.keys()], [1, 2]);
  assert.equal(merged.guaranteedOffer.outputs.length, 1);
  assert.equal(merged.fallibleOffer.get(2).outputs.length, 1);
  assert.equal(p.recheckFinalizedAgainstRecipe(merged, mergedRecipe).ok, true);
  const boundMerged = merged.bind();
  assert.equal(p.recheckFinalizedAgainstRecipe(boundMerged, mergedRecipe).ok, true);
});

test('native fundUnshielded reserves complete signed gross before finalize and rejects under-declared caps', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const signed = (segment, value) => {
    const intent = ledger.Intent.new(ttl);
    intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new(
      [nativeUtxo(ledger, keys.vk, token, value)],
      [{owner: keys.address, type: token, value: value - 1n}],
      [],
    );
    const tx = ledger.Transaction.fromParts('undeployed').addIntent({tag: 'specific', value: segment}, intent);
    return signUnshieldedTx(ledger, tx, keys.sk);
  };
  const controlRecipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: signed(1, 100n), ttl};
  const control = await inertFundUnshielded(p, {recipe: controlRecipe, payer: keys, token, ttl, cap: 500n});
  assert.equal(control.error?.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.equal(control.counters.reservedGross, 100n);
  assert.equal(control.counters.reservedSubmission, 1n);
  assert.deepEqual(control.calls, ['inert-finalize-stub']);

  const refundRecipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: signed(1, 100n), ttl};
  const refund = await inertFundUnshielded(p, {recipe: refundRecipe, payer: keys, token, ttl, cap: 1000n});
  assert.equal(refund.error?.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.equal(refund.counters.reservedGross, 100n);

  const over = await inertFundUnshielded(p, {
    recipe: {type: 'UNBOUND_TRANSACTION', baseTransaction: signed(1, 100n), ttl},
    payer: keys, token, ttl, cap: 50n, declared: 0n,
  });
  assert.match(over.error?.message ?? '', /gross-spend allowance exceeded/);
  assert.deepEqual(over.calls, []);
  assert.equal(over.counters.reservedGross, 0n);
  assert.equal(over.counters.reservedSubmission, 0n);

  const under = await inertFundUnshielded(p, {
    recipe: {type: 'UNBOUND_TRANSACTION', baseTransaction: signed(1, 100n), ttl},
    payer: keys, token, ttl, cap: 1000n, declared: 50n,
  });
  assert.match(under.error?.message ?? '', /gross-spend allowance exceeded/);
  assert.deepEqual(under.calls, []);
  assert.equal(under.counters.reservedGross, 0n);

  const parts = await inertFundUnshielded(p, {
    recipe: {
      type: 'UNBOUND_TRANSACTION',
      baseTransaction: signed(1, 100n),
      balancingTransaction: signed(2, 200n),
      ttl,
    },
    payer: keys, token, ttl, cap: 200n,
  });
  assert.match(parts.error?.message ?? '', /gross-spend allowance exceeded/);
  assert.deepEqual(parts.calls, []);
  assert.equal(parts.counters.reservedGross, 0n);

  const failKeep = await inertFundUnshielded(p, {
    recipe: {type: 'UNBOUND_TRANSACTION', baseTransaction: signed(1, 100n), ttl},
    payer: keys, token, ttl, cap: 500n,
  });
  assert.equal(failKeep.error?.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.equal(failKeep.counters.reservedGross, 100n);
  assert.equal(failKeep.events.some((e) => e && e.reservationRetained === true), true);

  const zswapOut = signed(1, 100n);
  zswapOut.guaranteedOffer = zswapContractOffer(ledger, token, 5n, undefined, ledger.sampleContractAddress());
  const shieldedOut = await inertFundUnshielded(p, {
    recipe: {type: 'UNBOUND_TRANSACTION', baseTransaction: zswapOut, ttl},
    payer: keys, token, ttl, cap: 500n,
  });
  assert.equal(shieldedOut.error?.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.equal(shieldedOut.counters.reservedGross, 100n);
});

test('malformed input amounts and mixed assets reject before reservation', async () => {
  const p = await load('providers.mjs');
  const ledger = await p.importPinned('@midnight-ntwrk/ledger-v8', {nodeModules: nm});
  const keys = mockPayer(ledger);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  const syn = recipeFixture(keys, token, ttl);
  syn.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.signatures = [
    keys.signData(syn.baseTransaction.intents.get(1).signatureData(1)),
  ];
  syn.baseTransaction.intents.get(1).guaranteedUnshieldedOffer.inputs[0].value = 100;
  const bad = await inertFundUnshielded(p, {recipe: syn, payer: keys, token, ttl, cap: 1000n});
  assert.ok(bad.error);
  assert.notEqual(bad.error.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.deepEqual(bad.calls, []);
  assert.equal(bad.counters.reservedGross, 0n);

  const mixed = nativeBaseTx(ledger, keys, token, ttl);
  const other = ledger.sampleRawTokenType();
  const intents = mixed.intents;
  const intent = intents.get(1);
  const inputs = [
    nativeUtxo(ledger, keys.vk, token, 40n, 0),
    nativeUtxo(ledger, keys.vk, other, 60n, 1),
  ];
  intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new(
    inputs,
    intent.guaranteedUnshieldedOffer.outputs,
    [ledger.signData(keys.sk, intent.signatureData(1)), ledger.signData(keys.sk, intent.signatureData(1))],
  );
  intents.set(1, intent);
  mixed.intents = intents;
  const mixedRecipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: mixed, ttl};
  const mix = await inertFundUnshielded(p, {recipe: mixedRecipe, payer: keys, token, ttl, cap: 1000n});
  assert.ok(mix.error);
  assert.notEqual(mix.error.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.deepEqual(mix.calls, []);
  assert.equal(mix.counters.reservedGross, 0n);
});

