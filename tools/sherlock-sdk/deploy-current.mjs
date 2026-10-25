// Maintainer-only stagenet deployment. Default mode checks the SDK state without a wallet.
// Usage: node deploy-current.mjs SDK_ROOT BUILD PREPARATION OUTPUT [--instance-id N] [--submit]
// --instance-id deploys the same source and keys under another instance_id (constructor argument 0 only).
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, statSync, cpSync, symlinkSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {parseDeployArgs, constructorArgs} from './deploy-args.mjs';
const {sdkRoot, build: buildArg, preparation, output: outputArg, submit, instanceId} = parseDeployArgs(process.argv.slice(2));
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
assert.equal(expected.schema, 'sherlock-offline-deployment-preparation/1');
assert.equal(expected.sourceSHA256, sha(readFileSync(new URL('./sherlock-anchor.compact', import.meta.url))));
assert.ok(expected.all13FieldsEqual && expected.setsEmpty && !expected.submitted);
mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'refuse a second deployment from this output directory');
const {setNetworkId, getNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
const ledgerLib = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {ZKConfigProvider, createVerifierKey} = await load('@midnight-ntwrk/midnight-js-types');
const {createUnprovenDeployTx, deployContract} = await load('@midnight-ntwrk/midnight-js-contracts');
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
const compiledContract = CompiledContract.make('SherlockAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(deployBuild));
const ctorArgs = constructorArgs(expected, instanceId);
if (instanceId !== undefined) console.log(`instance_id override: ${instanceId}`);
const options = {compiledContract, args: ctorArgs};
// Exercise the same SDK constructor/key installation used by deployContract before wallet access.
const dry = await createUnprovenDeployTx({zkConfigProvider, walletProvider: {
  getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
}}, options);
const checkState = state => {
  const view = mod.ledger(state.data), c = ctorArgs;
  const fields = ['instance_id','keys_id','a102_ctx_lo','a102_ctx_hi','a80_ctx_lo','a80_ctx_hi',
    'program_c','genesis_root','genesis_height','genesis_batch'];
  for (let i = 0; i < fields.length; i++) assert.equal(view[fields[i]], c[i], fields[i]);
  assert.equal(view.root, c[7]); assert.equal(view.height, c[8]); assert.equal(view.batch_no, c[9]);
  assert.ok(view.validity_ok.isEmpty() && view.settled.isEmpty());
  for (const [entry, pin] of Object.entries(expected.keys)) {
    assert.equal(sha(state.operation(entry).verifierKey), pin.sha256);
  }
};
checkState(dry.public.initialContractState);
console.log('PASS SDK constructor: all 13 fields, empty sets and exact verifier keys');
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
if (!submit) process.exit(0);
assert.equal((await (await fetch('http://localhost:6300/version')).text()).trim(), '9.0.0-rc.5');
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
  provingServerUrl: new URL('http://localhost:6300'), relayURL: new URL('wss://rpc.stagenet.shielded.tools')};
const wallet = await WalletFacade.init({configuration: {...config,
  txHistoryStorage: new NoOpTransactionHistoryStorage(),
  costParameters: {additionalFeeOverhead: 300_000_000_000_000n, feeBlocksMargin: 5}},
  shielded: cfg => ShieldedWallet(cfg).startWithSecretKeys(shieldedSecretKeys),
  unshielded: cfg => UnshieldedWallet(cfg).startWithPublicKey(PublicKey.fromKeyStore(keystore)),
  dust: cfg => DustWallet(cfg).startWithSecretKey(dustSecretKey, ledgerLib.LedgerParameters.initialParameters().dust)});
const started = performance.now();
try {
  await wallet.start(shieldedSecretKeys, dustSecretKey);
  console.log('Syncing stagenet wallet');
  await Rx.firstValueFrom(wallet.state().pipe(Rx.filter(s =>
    [s.shielded.state.progress, s.unshielded.progress, s.dust.state.progress]
      .every(p => p?.isCompleteWithin?.(50n) ?? false)), Rx.timeout({first: 20 * 60_000})));
  const walletProvider = {
    getCoinPublicKey: () => shieldedSecretKeys.coinPublicKey,
    getEncryptionPublicKey: () => shieldedSecretKeys.encryptionPublicKey,
    balanceTx: async (tx, ttl = ttlOneHour()) => wallet.finalizeRecipe(
      await wallet.balanceUnboundTransaction(tx, {shieldedSecretKeys, dustSecretKey}, {ttl})),
    submitTx: tx => {
      writeFileSync(join(output, 'submitted.json'), JSON.stringify({at: new Date().toISOString(),
        network: 'stagenet', purpose: 'deploy current Sherlock verifier'}) + '\n', {flag: 'wx', mode: 0o600});
      return wallet.submitTransaction(tx);
    },
  };
  const providers = {zkConfigProvider, walletProvider, midnightProvider: walletProvider,
    publicDataProvider: indexerPublicDataProvider(indexerHttpUrl, indexerWsUrl),
    proofProvider: httpClientProofProvider('http://localhost:6300', zkConfigProvider),
    privateStateProvider: levelPrivateStateProvider({
      privateStateStoreName: join(output, 'private-state'), signingKeyStoreName: join(output, 'signing-keys'),
      privateStoragePasswordProvider: () => readFileSync(join(walletDir, 'private-state.password'), 'utf8').trim(),
      accountId: String(keystore.getBech32Address())})};
  console.log('Deploying current Sherlock verifier');
  const deployed = await deployContract(providers, options);
  const p = deployed.deployTxData.public;
  // Save only explicit public fields. Never serialize the SDK deployment result.
  const receipt = {schema: 'sherlock-stagenet-deployment/2', network: 'stagenet', address: p.contractAddress,
    txId: p.txId, blockHeight: p.blockHeight, deployedAt: new Date().toISOString(),
    wallSeconds: (performance.now() - started) / 1000, sourceSHA256: expected.sourceSHA256, keys: expected.keys,
    preparationSHA256: sha(readFileSync(preparation)), instanceId: String(ctorArgs[0]),
    instanceIdOverridden: instanceId !== undefined, constructor: ctorArgs.map(String)};
  writeFileSync(join(output, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
  const state = await providers.publicDataProvider.queryContractState(p.contractAddress);
  assert.ok(state); checkState(state);
  writeFileSync(join(output, 'contract-state.bin'), state.serialize());
  console.log(JSON.stringify({...receipt, readback: 'all fields and both keys match'}));
} finally { await wallet.stop(); }
process.exit(0);
