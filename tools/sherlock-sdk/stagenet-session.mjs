// Maintainer SDK loading and the existing stagenet wallet. No wallet access at import time.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync, statSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
export function sdkLoader(sdkRoot, scratch) {
  for (const [name, version] of Object.entries({
    '@midnight-ntwrk/midnight-js-contracts': '5.0.0-beta.7',
    '@midnightntwrk/wallet-sdk': '2.0.0-beta.2',
    '@midnight-ntwrk/compact-runtime': '0.19.0',
  })) assert.equal(JSON.parse(readFileSync(resolve(sdkRoot, 'node_modules', name, 'package.json'))).version, version);
  mkdirSync(scratch, {recursive: true});
  if (!existsSync(join(scratch, 'node_modules')))
    symlinkSync(resolve(sdkRoot, 'node_modules'), join(scratch, 'node_modules'));
  return async name => {
    const file = join(scratch, createHash('sha256').update(name).digest('hex') + '.mjs');
    writeFileSync(file, `import * as sdk from ${JSON.stringify(name)}; export default sdk;\n`);
    return (await import(pathToFileURL(file).href)).default;
  };
}
export const endpoints = Object.freeze({
  indexer: 'https://indexer.stagenet.shielded.tools/api/v4/graphql',
  indexerWs: 'wss://indexer.stagenet.shielded.tools/api/v4/graphql/ws',
  rpc: 'wss://rpc.stagenet.shielded.tools', proof: 'http://localhost:6300',
});
export async function openStagenetSession(load, storage, {proofUrl = endpoints.proof,
  finalize = operation => operation()} = {}) {
  const {getNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
  assert.equal(getNetworkId(), 'stagenet');
  const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
  const Rx = await load('rxjs');
  const {WebSocket} = await load('ws');
  assert.equal(typeof WebSocket, 'function'); globalThis.WebSocket = WebSocket;
  const {HDWallet, Roles, createKeystore, validateMnemonic, WalletFacade, ShieldedWallet, DustWallet,
    UnshieldedWallet, PublicKey, NoOpTransactionHistoryStorage} = await load('@midnightntwrk/wallet-sdk');
  const {mnemonicToSeedSync} = await load('@scure/bip39');
  const {ttlOneHour} = await load('@midnight-ntwrk/midnight-js-utils');
  const {levelPrivateStateProvider} = await load('@midnight-ntwrk/midnight-js-level-private-state-provider');
  const walletDir = join(homedir(), '.midnight/stagenet-wallet');
  const mnemonic = join(walletDir, 'stagenet-wallet.mnemonic');
  assert.equal(statSync(mnemonic).mode & 0o777, 0o600);
  const phrase = readFileSync(mnemonic, 'utf8').trim().toLowerCase().split(/\s+/).join(' ');
  assert.ok(validateMnemonic(phrase), 'invalid mnemonic');
  const hd = HDWallet.fromSeed(mnemonicToSeedSync(phrase)); assert.equal(hd.type, 'seedOk');
  const derived = hd.hdWallet.selectAccount(0).selectRoles([Roles.Zswap, Roles.NightExternal, Roles.Dust]).deriveKeysAt(0);
  assert.equal(derived.type, 'keysDerived'); hd.hdWallet.clear();
  const shieldedSecretKeys = ledger.ZswapSecretKeys.fromSeed(derived.keys[Roles.Zswap]);
  const dustSecretKey = ledger.DustSecretKey.fromSeed(derived.keys[Roles.Dust]);
  const keystore = createKeystore({kind: 'schnorr', secret: derived.keys[Roles.NightExternal]}, getNetworkId());
  const config = {networkId: getNetworkId(),
    indexerClientConnection: {indexerHttpUrl: endpoints.indexer, indexerWsUrl: endpoints.indexerWs},
    provingServerUrl: new URL(proofUrl), relayURL: new URL(endpoints.rpc)};
  const wallet = await WalletFacade.init({configuration: {...config,
    txHistoryStorage: new NoOpTransactionHistoryStorage(),
    costParameters: {additionalFeeOverhead: 300_000_000_000_000n, feeBlocksMargin: 5}},
    shielded: cfg => ShieldedWallet(cfg).startWithSecretKeys(shieldedSecretKeys),
    unshielded: cfg => UnshieldedWallet(cfg).startWithPublicKey(PublicKey.fromKeyStore(keystore)),
    dust: cfg => DustWallet(cfg).startWithSecretKey(dustSecretKey, ledger.LedgerParameters.initialParameters().dust)});
  try {
    await wallet.start(shieldedSecretKeys, dustSecretKey);
    let last = 0;
    const progress = s => [s.shielded.state.progress, s.unshielded.progress, s.dust.state.progress]
      .map(p => p?.isCompleteWithin?.(50n) ?? false);
    await Rx.firstValueFrom(wallet.state().pipe(Rx.tap(s => {
      if (Date.now() - last >= 30_000) { last = Date.now(); console.log('Wallet sync components', progress(s)); }
    }), Rx.filter(s => progress(s).every(Boolean)), Rx.timeout({first: 20 * 60_000})));
    const walletProvider = {
      getCoinPublicKey: () => shieldedSecretKeys.coinPublicKey,
      getEncryptionPublicKey: () => shieldedSecretKeys.encryptionPublicKey,
      balanceTx: async (tx, ttl = ttlOneHour()) => {
        const recipe = await wallet.balanceUnboundTransaction(tx, {shieldedSecretKeys, dustSecretKey}, {ttl});
        return finalize(() => wallet.finalizeRecipe(recipe));
      },
      submitTx: tx => wallet.submitTransaction(tx),
    };
    return {walletProvider, privateStateProvider: levelPrivateStateProvider({
      privateStateStoreName: join(storage, 'private-state'), signingKeyStoreName: join(storage, 'signing-keys'),
      privateStoragePasswordProvider: () => readFileSync(join(walletDir, 'private-state.password'), 'utf8').trim(),
      accountId: String(keystore.getBech32Address())}), close: () => wallet.stop()};
  } catch (error) { await wallet.stop(); throw error; }
}
