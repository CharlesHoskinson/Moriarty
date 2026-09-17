/** Production fee-admission check with a synthetic native container.
 * All wallet/prover/indexer/private-store transports are inert. No original
 * private inputs, proof verification or ledger submission is performed.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync, mkdtempSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {createHash} from 'node:crypto';
import * as ledger from '/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_fs.js';
import {createFinancialProviders, initializeFinancialReservations} from './providers.mjs';
const ttl = new Date('2030-01-01T00:00:00Z');
async function proofFree(tx) {
  const forbidden = () => {throw Error('PROVER_FORBIDDEN');};
  return tx.prove({check: forbidden, prove: forbidden}, ledger.CostModel.initialCostModel());
}
test('native positive DUST fee is rejected above the allowance before wallet access', async t => {
  // Synthetic unbound container with a retained public native spend. Its
  // transplanted proof is never verified or submitted; this checks admission.
  const raw = readFileSync(new URL('./fixtures/historical-dust/transaction.bin', import.meta.url));
  assert.equal(createHash('sha256').update(raw).digest('hex'), 'f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a');
  const historical = ledger.Transaction.deserialize('signature', 'proof', 'binding', raw);
  const actions = [...historical.intents.values()].find(intent => intent.dustActions?.spends.length).dustActions;
  const fee = actions.spends.reduce((sum, spend) => sum + spend.vFee, 0n);
  assert.equal(fee, 300000000000001n);
  for (const cap of [fee - 1n, fee]) {
    const fixture = await proofFree(ledger.Transaction.fromParts('undeployed').addIntent({tag: 'specific', value: 1}, ledger.Intent.new(ttl)));
    const intents = fixture.intents, intent = intents.get(1);
    intent.dustActions = actions; intents.set(1, intent); fixture.intents = intents;
    ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', fixture.serialize());
    let walletCalls = 0;
    const dir = mkdtempSync(join(tmpdir(), 'moriarty-fee-control-'));
    t.after(() => rmSync(dir, {recursive: true, force: true}));
    const publicKey = ledger.signatureVerifyingKey(ledger.sampleSigningKey());
    const forbidden = () => {throw Error('UNEXPECTED_PROVIDER_CALL');};
    const options = {
      ledger,
      walletContext: {
        wallet: {async balanceUnboundTransaction() {walletCalls++; throw Error('INERT_WALLET_BOUNDARY');},
          signRecipe: forbidden, finalizeRecipe: forbidden, submitTransaction: forbidden, async stop() {}},
        shieldedSecretKeys: {coinPublicKey: 'synthetic', encryptionPublicKey: 'synthetic'}, dustSecretKey: {},
        unshieldedKeystore: {getPublicKey: () => publicKey, getBech32Address: () => ({toString: () => 'synthetic-account'}), signData: forbidden},
      },
      networkConfig: {networkId: 'undeployed', node: 'http://127.0.0.1:9944', indexer: 'http://127.0.0.1:8088/graphql', indexerWS: 'ws://127.0.0.1:8088/graphql', proofServer: 'http://127.0.0.1:6300'},
      zkConfigPath: '/synthetic/no-assets',
      privateStateConfig: {midnightDbName: '/synthetic/no-store', privateStateStoreName: 'inert', privateStoragePasswordProvider: forbidden},
      limits: {allocationId: 'native-fee-control', reservationStatePath: join(dir, 'reservation.json'), deadlineMs: Date.now() + 10000, submissions: 1, grossByAsset: {}, dustFee: cap},
      sdk: {getNetworkId: () => 'undeployed', NodeZkConfigProvider: class {}, levelPrivateStateProvider: () => ({}), indexerPublicDataProvider: () => ({}), httpClientProofProvider: () => ({})},
    };
    initializeFinancialReservations(options);
    const p = await createFinancialProviders(options);
    await assert.rejects(p.walletProvider.balanceTx(fixture, ttl), {message: cap < fee ? 'DUST fee allowance exceeded' : 'INERT_WALLET_BOUNDARY'});
    assert.equal(walletCalls, cap < fee ? 0 : 1);
    assert.equal(p.getState().reservedSubmissions, 0);
    assert.equal(p.getState().reservedDustFee, 0n);
    await p.cleanup();
  }
});
