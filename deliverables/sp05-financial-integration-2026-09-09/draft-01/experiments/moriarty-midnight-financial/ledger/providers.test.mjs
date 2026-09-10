import test from 'node:test';
import assert from 'node:assert/strict';
import * as ledger from '/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_fs.js';
import {createFinancialProviders, loadFinancialSdk} from './providers.mjs';

// Native ledger data; all wallet, prover, indexer and submission transports below are inert.
// Proof-free fixture conversion uses native .prove with callbacks that throw if any actual proof is requested.
// There are no contract calls, proof bodies, prover requests or financial ledger acceptance.
const assetA = ledger.sampleRawTokenType(), assetB = ledger.sampleRawTokenType();
const sk = ledger.sampleSigningKey(), vk = ledger.signatureVerifyingKey(sk);
const ttl = new Date('2030-01-01T00:00:00Z');
function transaction(asset = assetA, amount = 10n, fallible = false, segment = 1) {
  let intent = ledger.Intent.new(ttl);
  intent[fallible ? 'fallibleUnshieldedOffer' : 'guaranteedUnshieldedOffer'] = ledger.UnshieldedOffer.new(
    [{owner: vk, type: asset, value: amount, intentHash: ledger.sampleIntentHash(), outputNo: 0}],
    [{owner: ledger.addressFromKey(vk), type: asset, value: amount}], []);
  return ledger.Transaction.fromParts('undeployed').addIntent({tag: 'specific', value: segment}, intent);
}
async function proofFree(tx) {
  const forbidden = () => { throw Error('actual prover invocation forbidden'); };
  return tx.prove({check: forbidden, prove: forbidden}, ledger.CostModel.initialCostModel());
}
async function unbound(...args) { return proofFree(transaction(...args)); }
function sign(tx) {
  const intents = tx.intents;
  for (const [segment, intent] of intents) {
    for (const key of ['guaranteedUnshieldedOffer', 'fallibleUnshieldedOffer']) {
      if (intent[key]) intent[key] = intent[key].addSignatures(intent[key].inputs.map(() => ledger.signData(sk, intent.signatureData(segment))));
    }
    intents.set(segment, intent);
  }
  tx.intents = intents;
  return tx;
}
function harness(overrides = {}) {
  const calls = [], events = [], config = {};
  const wallet = {
    async balanceUnboundTransaction(tx) { calls.push('balance'); return {type: 'UNBOUND_TRANSACTION', baseTransaction: tx}; },
    async signRecipe(recipe) { calls.push('sign'); return {...recipe, baseTransaction: sign(recipe.baseTransaction)}; },
    async finalizeRecipe(recipe) { calls.push('inert-finalize'); return recipe.baseTransaction.bind(); },
    async submitTransaction(tx) { calls.push('inert-submit'); return tx.identifiers().at(-1); },
    async stop() { calls.push('stop'); },
    ...overrides.wallet,
  };
  const sdk = {
    NodeZkConfigProvider: class { constructor(path) { config.zk = path; } },
    levelPrivateStateProvider(c) { config.privateState = c; return {setContractAddress() {}, async get() { return {}; }}; },
    indexerPublicDataProvider(...args) { config.indexer = args; return {async queryContractState() { return {}; }}; },
    httpClientProofProvider(...args) { config.proof = args; return {async proveTx() { return {}; }}; },
  };
  const options = {
    walletContext: {wallet, shieldedSecretKeys: {coinPublicKey: 'coin', encryptionPublicKey: 'enc'}, dustSecretKey: {},
      unshieldedKeystore: {getPublicKey: () => vk, getBech32Address: () => ({toString: () => 'existing-wallet-account'}), signData: data => ledger.signData(sk, data)}},
    networkConfig: {indexer: 'http://127.0.0.1:8088/api/v3/graphql', indexerWS: 'ws://127.0.0.1:8088/api/v3/graphql/ws', proofServer: 'http://127.0.0.1:6300'},
    zkConfigPath: '/tmp/sp05-proven-assets',
    privateStateConfig: {midnightDbName: '/tmp/sp05-isolated-state', privateStateStoreName: 'financial', privateStoragePasswordProvider: () => 'Inert-test-password-37!'},
    limits: {deadlineMs: Date.now() + 10000, submissions: 8, grossByAsset: {[assetA]: 20n, [assetB]: 30n}, dustFee: 1000n},
    ledger, sdk, onEvent: event => events.push(event), ...overrides.options,
  };
  return {calls, events, config, options, wallet, create: () => createFinancialProviders(options)};
}
test('constructs the actual SDK provider interfaces with account-scoped storage and proven asset path', async () => {
  const h = harness(), p = await h.create();
  assert.equal(h.config.zk, h.options.zkConfigPath);
  assert.equal(h.config.privateState.midnightDbName, h.options.privateStateConfig.midnightDbName);
  assert.equal(h.config.privateState.accountId, 'existing-wallet-account');
  assert.equal(h.config.privateState.privateStoragePasswordProvider, h.options.privateStateConfig.privateStoragePasswordProvider);
  assert.equal(h.config.proof[0], h.options.networkConfig.proofServer);
  assert.equal(h.config.indexer[1], h.options.networkConfig.indexerWS);
  assert.equal(p.walletProvider.getCoinPublicKey(), 'coin');
  assert.equal(p.walletProvider.getEncryptionPublicKey(), 'enc');
  assert.equal(typeof p.midnightProvider.submitTx, 'function');
  await p.cleanup();
});
test('real pinned SDK constructors accept production options without storage or network operations', async () => {
  const sdk = await loadFinancialSdk();
  const h = harness({options: {sdk}}), p = await h.create();
  assert.equal(typeof p.privateStateProvider.get, 'function');
  assert.equal(typeof p.publicDataProvider.watchForTxData, 'function');
  assert.equal(typeof p.zkConfigProvider.getProverKey, 'function');
  await p.cleanup();
});
test('production balance/sign/finalize/submit retains per-asset gross without subtracting refunds', async () => {
  const h = harness(), p = await h.create();
  for (const [asset, amount] of [[assetA, 10n], [assetB, 30n], [assetA, 10n]]) {
    const tx = await p.walletProvider.balanceTx(await unbound(asset, amount), ttl);
    await p.midnightProvider.submitTx(tx);
  }
  assert.deepEqual(p.getState().reservedGrossByAsset, {[assetA]: 20n, [assetB]: 30n});
  assert.equal(p.getState().reservedSubmissions, 3);
  await assert.rejects(p.walletProvider.balanceTx(await unbound(assetA, 1n), ttl), /gross.*allowance/);
  assert.equal(h.calls.filter(c => c === 'inert-finalize').length, 3);
  assert.equal(h.events.filter(e => e.kind === 'submitted').length, 3);
});
test('rejects malformed counters before wallet balance or signing', async () => {
  for (const limits of [{submissions: NaN}, {submissions: 1.5}, {deadlineMs: Infinity}, {dustFee: -1n}, {grossByAsset: {[assetA]: -1n}}]) {
    const h = harness(); Object.assign(h.options.limits, limits);
    await assert.rejects(h.create()); assert.deepEqual(h.calls, []);
  }
});
test('unknown recipes and diagnostic transaction maps stop before signing or finalizing', async () => {
  for (const recipe of [{type: 'UNKNOWN', baseTransaction: transaction()}, {type: 'UNBOUND_TRANSACTION', baseTransaction: {intents: transaction().intents}}]) {
    const h = harness({wallet: {async balanceUnboundTransaction() { return recipe; }}}), p = await h.create();
    await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl));
    assert.deepEqual(h.calls, []);
    await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl), /stopped/);
  }
});
test('DUST registration is excluded before signing regardless of signature presence or fee authorization', async () => {
  const tx = transaction(), intents = tx.intents, intent = intents.get(1);
  intent.dustActions = new ledger.DustActions('signature', 'pre-proof', new Date(), [], [new ledger.DustRegistration('signature', vk, undefined, 900n, ledger.signData(sk, new Uint8Array([1])))]);
  intents.set(1, intent); tx.intents = intents;
  const h = harness(), p = await h.create();
  await assert.rejects(p.walletProvider.balanceTx(tx, ttl), /registration.*unsupported/);
  assert.deepEqual(h.calls, []);
});
test('wrong signatures reject before finalize; presence is insufficient', async () => {
  const h = harness({wallet: {async signRecipe(r) { return r; }}}), p = await h.create();
  await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl), /signature/);
  assert.equal(h.calls.includes('inert-finalize'), false);
});
test('changed fallible output and extra finalized intent stop submission and retain reservation', async () => {
  for (const mode of ['output', 'intent']) {
    const h = harness({wallet: {async finalizeRecipe(r) {
      const tx = r.baseTransaction;
      if (mode === 'output') { const ints = tx.intents, i = ints.get(1); const offer = i.fallibleUnshieldedOffer;
        i.fallibleUnshieldedOffer = ledger.UnshieldedOffer.new(offer.inputs, [{...offer.outputs[0], value: 1n}], offer.signatures); ints.set(1, i); tx.intents = ints;
      } else { return (await proofFree(transaction().addIntent({tag: 'specific', value: 7}, ledger.Intent.new(ttl)))).bind(); }
      return tx.bind();
    }}}), p = await h.create();
    await assert.rejects(p.walletProvider.balanceTx(await unbound(assetA, 10n, true), ttl), /semantic/);
    assert.equal(p.getState().reservedGrossByAsset[assetA], 10n);
    assert.equal(p.getState().stopped, true);
    assert.equal(h.calls.includes('inert-submit'), false);
  }
});
test('submission only accepts issued unchanged transaction once', async () => {
  const h = harness(), p = await h.create(), tx = await p.walletProvider.balanceTx(await unbound(), ttl);
  await p.midnightProvider.submitTx(tx);
  await assert.rejects(p.midnightProvider.submitTx(tx), /issued|submitted/);
  assert.equal(h.calls.filter(c => c === 'inert-submit').length, 1);
});
test('deadline during finalization stops all later effects and reports uncontained operation', async () => {
  let finish;
  const h = harness({wallet: {finalizeRecipe() { return new Promise(resolve => { finish = resolve; }); }}});
  h.options.limits.deadlineMs = Date.now() + 80;
  const p = await h.create();
  await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl), /deadline/);
  assert.equal(p.getState().reservedSubmissions, 1);
  assert.equal(p.getState().pendingOperations, 1);
  await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl), /stopped/);
  finish(transaction().bind());
  await new Promise(resolve => setImmediate(resolve));
  await p.cleanup();
  assert.equal(h.calls.includes('inert-submit'), false);
});
test('inert ambiguous submit failure permanently stops provider and retains IDs', async () => {
  const h = harness({wallet: {async submitTransaction() { throw Error('inert disconnect'); }}}), p = await h.create();
  const tx = await p.walletProvider.balanceTx(await unbound(), ttl);
  await assert.rejects(p.midnightProvider.submitTx(tx), /inert disconnect/);
  assert.equal(p.getState().stopped, true);
  assert.deepEqual(p.getState().identifiers, tx.identifiers());
  assert.equal(p.getState().reservedSubmissions, 1);
});
test('native balancing transaction contributes separate assets and fallible gross to the same reservation', async () => {
  const h = harness({wallet: {
    async balanceUnboundTransaction(tx) { return {type: 'UNBOUND_TRANSACTION', baseTransaction: tx, balancingTransaction: transaction(assetB, 30n, true, 7)}; },
    async signRecipe(recipe) { return {...recipe, baseTransaction: sign(recipe.baseTransaction), balancingTransaction: sign(recipe.balancingTransaction)}; },
    async finalizeRecipe(recipe) { return recipe.baseTransaction.bind().merge((await proofFree(recipe.balancingTransaction)).bind()); },
  }}), p = await h.create();
  const tx = await p.walletProvider.balanceTx(await unbound(assetA, 10n), ttl);
  await p.midnightProvider.submitTx(tx);
  assert.deepEqual(p.getState().reservedGrossByAsset, {[assetA]: 10n, [assetB]: 30n});
});
test('unknown asset and exhausted submission cap reject before further wallet operations', async () => {
  const h = harness(), p = await h.create();
  await assert.rejects(p.walletProvider.balanceTx(await unbound(ledger.sampleRawTokenType()), ttl), /gross.*allowance/);
  assert.deepEqual(h.calls, []);
  const other = harness(); other.options.limits.submissions = 1;
  const q = await other.create(); await q.walletProvider.balanceTx(await unbound(), ttl);
  const count = other.calls.length;
  await assert.rejects(q.walletProvider.balanceTx(await unbound(), ttl), /submission allowance/);
  assert.equal(other.calls.length, count);
});
test('query deadline and explicit cleanup share the wallet stop latch', async () => {
  const h = harness(), p = await h.create();
  p.stop('caller diagnosis');
  assert.throws(() => p.publicDataProvider.queryContractState('ignored'), /stopped/);
  assert.throws(() => p.walletProvider.getCoinPublicKey(), /stopped/);
  assert.equal((await p.cleanup()).walletStopped, true);
  await p.cleanup(); assert.equal(h.calls.filter(c => c === 'stop').length, 1);
});
test('actual wallet balance flags exclude shielded funding; pre-proof callers reject before balance', async () => {
  const h = harness({wallet: {async balanceUnboundTransaction(tx, keys, options) {
    assert.deepEqual(options.tokenKindsToBalance, ['unshielded', 'dust']);
    return {type: 'UNBOUND_TRANSACTION', baseTransaction: tx};
  }}}), p = await h.create();
  await p.walletProvider.balanceTx(await unbound(), ttl);
  const other = harness(), q = await other.create();
  await assert.rejects(q.walletProvider.balanceTx(transaction(), ttl), /deserialize/);
  assert.deepEqual(other.calls, []);
});
test('balancing recipe with unadmitted gross rejects before sign', async () => {
  const h = harness({wallet: {async balanceUnboundTransaction(tx) {
    return {type: 'UNBOUND_TRANSACTION', baseTransaction: tx, balancingTransaction: transaction(assetB, 31n, false, 7)};
  }}}), p = await h.create();
  await assert.rejects(p.walletProvider.balanceTx(await unbound(), ttl), /gross.*allowance/);
  assert.deepEqual(h.calls, []);
});
