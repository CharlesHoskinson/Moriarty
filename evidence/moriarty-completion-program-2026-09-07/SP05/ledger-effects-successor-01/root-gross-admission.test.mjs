import {test} from 'node:test';
import assert from 'node:assert/strict';
import {importPinned, validateSignedRecipe, createFinancialProviders} from './candidate/experiments/moriarty-midnight-financial/ledger/providers.mjs';

// Native signing with synthetic UTXOs. All facade and transport functions below
// are inert local stubs. No wallet, native finalization, proof or network call.
const ledger = await importPinned('@midnight-ntwrk/ledger-v8');
async function exercise({cap, declared, balancing = false}) {
  const sk = ledger.sampleSigningKey();
  const vk = ledger.signatureVerifyingKey(sk);
  const owner = ledger.addressFromKey(vk);
  const token = ledger.sampleRawTokenType();
  const ttl = new Date('2030-01-01T00:00:00Z');
  function signedTx(segment, value) {
    const intent = ledger.Intent.new(ttl);
    intent.guaranteedUnshieldedOffer = ledger.UnshieldedOffer.new(
      [{owner: vk, type: token, value, intentHash: ledger.sampleIntentHash(), outputNo: 0}],
      [{owner, type: token, value: value - 1n}], [],
    );
    const tx = ledger.Transaction.fromParts('undeployed').addIntent({tag: 'specific', value: segment}, intent);
    const intents = tx.intents;
    const actual = intents.get(segment);
    actual.guaranteedUnshieldedOffer = actual.guaranteedUnshieldedOffer.addSignatures([ledger.signData(sk, actual.signatureData(segment))]);
    intents.set(segment, actual); tx.intents = intents;
    return tx;
  }
  const recipe = {type: 'UNBOUND_TRANSACTION', baseTransaction: signedTx(1, 100n)};
  if (balancing) recipe.balancingTransaction = signedTx(2, 200n);
  assert.equal(validateSignedRecipe(recipe, {vk}, {ledger, tokenType: token}).ok, true);
  const counters = {submission: 1n, grossSpend: cap, reservedSubmission: 0n, reservedGross: 0n};
  const calls = [];
  const providers = await createFinancialProviders({
    walletContext: {unshieldedKeystore: {signData: p => ledger.signData(sk, p)}, facade: {
      balanceUnboundTransaction: async () => recipe,
      signRecipe: async () => recipe,
      finalizeRecipe: async () => {calls.push('inert-finalize-stub'); throw new Error('LOCAL_FINALIZE_BOUNDARY');},
      submitTransaction: async () => {throw new Error('UNREACHABLE_SUBMIT');},
    }},
    networkConfig: {indexer: 'inert:', prover: 'inert:'},
    privateStateLocation: '/unused-inert-state',
    provenAssetManifest: {proven: false}, resourceCounters: counters,
    onSubmission: () => {throw new Error('UNREACHABLE_SUBMISSION');}, eventSink: () => {},
    adapters: {
      CompiledContract: {}, NodeZkConfigProvider: {}, httpClientProofProvider: {},
      indexerPublicDataProvider: {}, levelPrivateStateProvider: {},
      deployContract: {}, submitCallTx: {}, findDeployedContract: {},
    },
  });
  let error;
  try {
    await providers.fundUnshielded({tx: recipe.baseTransaction, payer: {vk}, ttl,
      allowance: {submission: 1n, ...(declared === undefined ? {} : {grossSpend: declared})}, tokenType: token});
  } catch (e) {error = e;}
  return {error, counters, calls};
}
test('control: permitted signed input reaches only the inert finalization boundary', async () => {
  const r = await exercise({cap: 500n});
  assert.equal(r.error?.message, 'LOCAL_FINALIZE_BOUNDARY');
  assert.equal(r.counters.reservedGross, 100n);
  assert.deepEqual(r.calls, ['inert-finalize-stub']);
});
test('gross admission rejects a caller declaration below the signed 100-unit input', async () => {
  const r = await exercise({cap: 50n, declared: 0n});
  assert.match(r.error?.message ?? '', /gross-spend allowance exceeded/);
  assert.deepEqual(r.calls, []);
});
test('gross admission includes 200-unit balancing input with the 100-unit base', async () => {
  const r = await exercise({cap: 200n, balancing: true});
  assert.match(r.error?.message ?? '', /gross-spend allowance exceeded/);
  assert.deepEqual(r.calls, []);
});
