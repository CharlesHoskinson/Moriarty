// Maintainer-only stagenet deployment. Default mode checks the SDK state without a wallet.
// Usage: node rollup-deploy.mjs SDK_ROOT BUILD PREPARATION OUTPUT [--submit]
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, statSync, cpSync, symlinkSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {anchorView} from './rollup-statement.mjs';
import {ownedWalletProver} from './owned-wallet-prover.mjs';
import {recordSubmissionRefusal, quietSubmission} from './rollup-refusal.mjs';
const [sdkRoot, buildArg, preparation, outputArg, mode] = process.argv.slice(2);
assert.ok(sdkRoot && buildArg && preparation && outputArg);
assert.ok(mode === undefined || mode === '--submit');
// Resolve packages through normal ESM imports, including import-only barrel packages.
const load = async name => {
  const imports = join(output, 'sdk-imports');
  mkdirSync(imports, {recursive: true});
  if (!existsSync(join(imports, 'node_modules')))
    symlinkSync(resolve(sdkRoot, 'node_modules'), join(imports, 'node_modules'));
  const file = join(imports, sha(Buffer.from(name)) + '.mjs');
  writeFileSync(file, `import * as sdk from ${JSON.stringify(name)}; export default sdk;\n`);
  return (await import(pathToFileURL(file).href)).default;
};
for (const [name, version] of Object.entries({
  '@midnight-ntwrk/midnight-js-contracts': '5.0.0-beta.7',
  '@midnightntwrk/wallet-sdk': '2.0.0-beta.2',
  '@midnight-ntwrk/compact-runtime': '0.19.0',
})) assert.equal(JSON.parse(readFileSync(resolve(sdkRoot, 'node_modules', name, 'package.json'))).version, version);
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const build = resolve(buildArg), output = resolve(outputArg);
const expected = JSON.parse(readFileSync(preparation));
assert.equal(expected.schema, 'sherlock-rollup-deployment-preparation/1');
assert.equal(expected.sourceSHA256, sha(readFileSync(new URL('./rollup-anchor.compact', import.meta.url))));
assert.ok(expected.all12FieldsEqual && expected.setsEmpty && !expected.submitted);
mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'refuse a second deployment from this output directory');
const {setNetworkId, getNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
const ledgerLib = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {ZKConfigProvider, createVerifierKey} = await load('@midnight-ntwrk/midnight-js-types');
const {createUnprovenDeployTx, deployContract} = await load('@midnight-ntwrk/midnight-js-contracts');
const rt = await load('@midnight-ntwrk/compact-runtime');
// WASM classes are tied to a module instance; use the wallet SDK runtime for this copy.
const deployBuild = join(output, 'build');
mkdirSync(deployBuild, {recursive: true});
cpSync(join(build, 'contract'), join(deployBuild, 'contract'), {recursive: true});
writeFileSync(join(deployBuild, 'package.json'), '{"type":"module"}\n');
if (!existsSync(join(deployBuild, 'node_modules')))
  symlinkSync(resolve(sdkRoot, 'node_modules'), join(deployBuild, 'node_modules'));
const mod = await import(pathToFileURL(join(deployBuild, 'contract/index.js')).href);
setNetworkId('stagenet');
class ExactKeys extends ZKConfigProvider {
  async getVerifierKey(name) {
    const pin = expected.keys[name];
    assert.ok(pin, 'unexpected entry point');
    const raw = readFileSync(pin.path);
    assert.equal(raw.length, pin.bytes);
    assert.equal(sha(raw), pin.sha256);
    return createVerifierKey(raw);
  }
  async getProverKey() { throw Error('deployment must not prove a contract call'); }
  async getZKIR() { throw Error('deployment must not prove a contract call'); }
}
const zkConfigProvider = new ExactKeys();
const compiledContract = CompiledContract.make('SherlockRollupAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(deployBuild));
const options = {compiledContract, args: expected.constructor.map(BigInt)};
// Exercise the same SDK constructor/key installation used by deployContract before wallet access.
const dry = await createUnprovenDeployTx({zkConfigProvider, walletProvider: {
  getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
}}, options);
const checkState = state => {
  const view = anchorView(rt, mod, state), c = expected.constructor.map(BigInt);
  const fields = ['instance_id', 'root', 'height', 'batch_no', 'dep_consumed', 'dep_acc'];
  for (let i = 0; i < fields.length; i++) assert.equal(view[fields[i]], c[i], fields[i]);
  for (const field of ['last_in_root', 'last_fee_sum', 'last_wd_sum_usd', 'last_wd_sum_night',
    'last_dep_sum_usd', 'last_dep_sum_night']) assert.equal(view[field], 0n, field);
  assert.ok(view.validity_ok.isEmpty() && view.settled.isEmpty());
  for (const [entry, pin] of Object.entries(expected.keys)) {
    assert.equal(sha(state.operation(entry).verifierKey), pin.sha256);
  }
};
checkState(dry.public.initialContractState);
console.log('PASS SDK constructor: all 12 fields, empty sets and exact verifier keys');
const Rx = await load('rxjs');
const {WebSocket} = await load('ws');
assert.equal(typeof WebSocket, 'function', 'ESM WebSocket constructor');
assert.equal(typeof Rx.firstValueFrom, 'function');
assert.equal(typeof Rx.filter, 'function');
globalThis.WebSocket = WebSocket;
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const {httpClientProofProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const {levelPrivateStateProvider} = await load('@midnight-ntwrk/midnight-js-level-private-state-provider');
const {ttlOneHour} = await load('@midnight-ntwrk/midnight-js-utils');
const {HDWallet, Roles, createKeystore, validateMnemonic, WalletFacade, ShieldedWallet, DustWallet,
  UnshieldedWallet, PublicKey, NoOpTransactionHistoryStorage} = await load('@midnightntwrk/wallet-sdk');
const {mnemonicToSeedSync} = await load('@scure/bip39');
assert.equal(typeof HDWallet.fromSeed, 'function');
assert.equal(typeof WalletFacade.init, 'function');
if (!mode) process.exit(0);
const guard = await ownedWalletProver({binary: process.env.MORIARTY_WALLET_PROOF_SERVER,
  parameters: process.env.MORIARTY_WALLET_PROOF_PARAMS, output,
  sha256: process.env.MORIARTY_WALLET_PROOF_SERVER_SHA256, purpose: 'deploy'});
const walletDir = join(homedir(), '.midnight/stagenet-wallet');
const mnemonicPath = join(walletDir, 'stagenet-wallet.mnemonic');
assert.equal(statSync(mnemonicPath).mode & 0o777, 0o600);
const phrase = readFileSync(mnemonicPath, 'utf8').trim().toLowerCase().split(/\s+/).join(' ');
assert.ok(validateMnemonic(phrase), 'invalid mnemonic');
const hd = HDWallet.fromSeed(mnemonicToSeedSync(phrase));
assert.equal(hd.type, 'seedOk');
const derived = hd.hdWallet.selectAccount(0).selectRoles([Roles.Zswap, Roles.NightExternal, Roles.Dust]).deriveKeysAt(0);
assert.equal(derived.type, 'keysDerived'); hd.hdWallet.clear();
const shieldedSecretKeys = ledgerLib.ZswapSecretKeys.fromSeed(derived.keys[Roles.Zswap]);
const dustSecretKey = ledgerLib.DustSecretKey.fromSeed(derived.keys[Roles.Dust]);
const keystore = createKeystore({kind: 'schnorr', secret: derived.keys[Roles.NightExternal]}, getNetworkId());
const indexerHttpUrl = 'https://indexer.stagenet.shielded.tools/api/v4/graphql';
const indexerWsUrl = 'wss://indexer.stagenet.shielded.tools/api/v4/graphql/ws';
const config = {networkId: getNetworkId(), indexerClientConnection: {indexerHttpUrl, indexerWsUrl},
  provingServerUrl: new URL(guard.url), relayURL: new URL('wss://rpc.stagenet.shielded.tools')};
const wallet = await WalletFacade.init({configuration: {...config,
  txHistoryStorage: new NoOpTransactionHistoryStorage(),
  costParameters: {additionalFeeOverhead: 300_000_000_000_000n, feeBlocksMargin: 5}},
  shielded: cfg => ShieldedWallet(cfg).startWithSecretKeys(shieldedSecretKeys),
  unshielded: cfg => UnshieldedWallet(cfg).startWithPublicKey(PublicKey.fromKeyStore(keystore)),
  dust: cfg => DustWallet(cfg).startWithSecretKey(dustSecretKey, ledgerLib.LedgerParameters.initialParameters().dust)});
const started = performance.now();
let submissionStarted, submissionAcknowledged;
try {
  await wallet.start(shieldedSecretKeys, dustSecretKey);
  console.log('Syncing stagenet wallet');
  await Rx.firstValueFrom(wallet.state().pipe(Rx.filter(s =>
    [s.shielded.state.progress, s.unshielded.progress, s.dust.state.progress]
      .every(p => p?.isCompleteWithin?.(50n) ?? false)), Rx.timeout({first: 20 * 60_000})));
  const walletProvider = {
    getCoinPublicKey: () => shieldedSecretKeys.coinPublicKey,
    getEncryptionPublicKey: () => shieldedSecretKeys.encryptionPublicKey,
    balanceTx: async (tx, ttl = ttlOneHour()) => {
      const recipe = await wallet.balanceUnboundTransaction(tx, {shieldedSecretKeys, dustSecretKey}, {ttl});
      return guard.run(() => wallet.finalizeRecipe(recipe));
    },
    submitTx: async tx => {
      writeFileSync(join(output, 'submitted.json'), JSON.stringify({at: new Date().toISOString(),
        network: 'stagenet', purpose: 'deploy synthetic Sherlock rollup verifier',
        transactionSHA256: sha(tx.serialize())}) + '\n', {flag: 'wx', mode: 0o600});
      submissionStarted = performance.now();
      try {
        const submitted = await quietSubmission(() => wallet.submitTransaction(tx));
        submissionAcknowledged = performance.now();
        return submitted;
      } catch (error) {
        recordSubmissionRefusal(error, output, 'deploy');
        throw Error('stagenet deployment submission failed; private details suppressed');
      }
    },
  };
  const providers = {zkConfigProvider, walletProvider, midnightProvider: walletProvider,
    publicDataProvider: indexerPublicDataProvider(indexerHttpUrl, indexerWsUrl),
    proofProvider: httpClientProofProvider(guard.url, zkConfigProvider),
    privateStateProvider: levelPrivateStateProvider({
      privateStateStoreName: join(output, 'private-state'), signingKeyStoreName: join(output, 'signing-keys'),
      privateStoragePasswordProvider: () => readFileSync(join(walletDir, 'private-state.password'), 'utf8').trim(),
      accountId: String(keystore.getBech32Address())})};
  console.log('Deploying synthetic Sherlock rollup verifier');
  const deployed = await deployContract(providers, options);
  const finalizationObserved = performance.now();
  const p = deployed.deployTxData.public;
  // Save only explicit public fields. Never serialize the SDK deployment result.
  const receipt = {schema: 'sherlock-rollup-stagenet-deployment/1', network: 'stagenet', address: p.contractAddress,
    txId: p.txId, txHash: p.txHash, blockHeight: p.blockHeight, blockHash: p.blockHash,
    fees: p.fees, status: p.status, deployedAt: new Date().toISOString(),
    wallSeconds: (performance.now() - started) / 1000, sourceSHA256: expected.sourceSHA256, keys: expected.keys,
    provingArtifacts: expected.provingArtifacts,
    timing: {submissionAckSeconds: (submissionAcknowledged - submissionStarted) / 1000,
      finalizationObservedSeconds: (finalizationObserved - submissionStarted) / 1000,
      finalizationScope: 'host RPC submission start through SDK finalized deployment observation',
      validatorCryptographicVerificationSeconds: null, validatorTimingEvidence: 'unavailable'}};
  writeFileSync(join(output, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
  const readbackStarted = performance.now();
  const state = await providers.publicDataProvider.queryContractState(p.contractAddress);
  assert.ok(state); checkState(state);
  writeFileSync(join(output, 'contract-state.bin'), state.serialize());
  receipt.timing.readbackSeconds = (performance.now() - readbackStarted) / 1000;
  receipt.timing.submissionToReadbackSeconds = (performance.now() - submissionStarted) / 1000;
  receipt.completeEffectsMatch = true;
  receipt.postStateSHA256 = sha(state.serialize());
  writeFileSync(join(output, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
  console.log(JSON.stringify({...receipt, readback: 'all fields and both keys match'}));
} catch { throw Error('rollup deployment failed; private wallet/provider details suppressed'); }
finally { try { await wallet.stop(); } catch { throw Error('wallet shutdown failed; private details suppressed'); } }
process.exit(0);
