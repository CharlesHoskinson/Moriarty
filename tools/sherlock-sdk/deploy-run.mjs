// Maintainer-only stagenet deployment of SherlockRun (general-purpose recursion settlement).
// Usage: node deploy-run.mjs SDK_ROOT BUILD RUN_DECIDER_VERIFIER PROG_D INIT_D OUTPUT [--submit]
// BUILD is the compiled sherlock-run.compact (compact 0.34.0, --feature-zkir-v3). The acceptRun
// verifier installed is RUN_DECIDER_VERIFIER (tagged ledger VerifierKey bytes of the hand-written
// run decider), never the compiled acceptRun key. Without --submit only the SDK constructor and key
// installation are checked, with no wallet access. Wallet material is never printed or written.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, cpSync, symlinkSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {createHash} from 'node:crypto';
import {sdkLoader, openStagenetSession, endpoints} from './stagenet-session.mjs';

const argv = process.argv.slice(2);
const submit = argv.includes('--submit');
const pos = argv.filter(a => a !== '--submit');
assert.equal(pos.length, 6, 'usage: deploy-run.mjs SDK_ROOT BUILD VERIFIER PROG_D INIT_D OUTPUT [--submit]');
const [sdkRoot, buildArg, vkPath, progHex, initHex, outputArg] = pos;
const FR = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;
const sha = b => createHash('sha256').update(b).digest('hex');
// 32-byte little-endian field hex (mori_recursion::util::f_to_hex) to bigint.
const fromLe = h => { assert.match(h, /^[0-9a-f]{64}$/); const v = BigInt('0x' + Buffer.from(h, 'hex').reverse().toString('hex')); assert.ok(v > 0n && v < FR); return v; };
const prog = fromLe(progHex), init = fromLe(initHex);
const source = readFileSync(new URL('./sherlock-run.compact', import.meta.url));
const vkRaw = readFileSync(vkPath);
assert.ok(vkRaw.subarray(0, 22).toString().startsWith('midnight:verifier-key['), 'tagged ledger verifier key');
const output = resolve(outputArg), build = resolve(buildArg);
mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'refuse a second deployment from this output directory');
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {ZKConfigProvider, createVerifierKey} = await load('@midnight-ntwrk/midnight-js-types');
const {createUnprovenDeployTx, deployContract} = await load('@midnight-ntwrk/midnight-js-contracts');
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const {httpClientProofProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const deployBuild = join(output, 'build');
mkdirSync(deployBuild, {recursive: true});
cpSync(join(build, 'contract'), join(deployBuild, 'contract'), {recursive: true});
writeFileSync(join(deployBuild, 'package.json'), '{"type":"module"}\n');
if (!existsSync(join(deployBuild, 'node_modules'))) symlinkSync(resolve(sdkRoot, 'node_modules'), join(deployBuild, 'node_modules'));
const mod = await import(new URL('file://' + join(deployBuild, 'contract/index.js')).href);
setNetworkId('stagenet');
class ExactKeys extends ZKConfigProvider {
  async getVerifierKey(name) { assert.equal(name, 'acceptRun', 'unexpected entry point'); return createVerifierKey(vkRaw); }
  async getProverKey() { throw Error('deployment must not prove a contract call'); }
  async getZKIR() { throw Error('deployment must not prove a contract call'); }
}
const zkConfigProvider = new ExactKeys();
const compiledContract = CompiledContract.make('SherlockRun', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(deployBuild));
const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
const INITIAL_RESULT = 1n; // big-endian u128 1: a nonzero low byte keeps the 16-byte window
const options = {compiledContract, args: [prog, init, be16(INITIAL_RESULT)]};
const checkState = state => {
  const v = mod.ledger(state.data);
  assert.equal(v.program_d, prog); assert.equal(v.init_d, init);
  assert.deepEqual(Buffer.from(v.result), Buffer.from(be16(INITIAL_RESULT))); assert.equal(v.steps, 0n); assert.ok(v.runs.isEmpty());
  assert.equal(sha(state.operation('acceptRun').verifierKey), sha(vkRaw), 'installed acceptRun key');
};
const dry = await createUnprovenDeployTx({zkConfigProvider, walletProvider: {
  getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32)}}, options);
checkState(dry.public.initialContractState);
const dryState = Buffer.from(dry.public.initialContractState.serialize());
writeFileSync(join(output, 'dry-contract-state.bin'), dryState);
console.log(`PASS SDK constructor: program_d, init_d, result 1 (BE), empty runs, run-decider key installed (${vkRaw.length} bytes)`);
if (!submit) process.exit(0);
assert.equal((await (await fetch(endpoints.proof + '/version')).text()).trim(), '9.0.0-rc.5');
const started = performance.now();
const session = await openStagenetSession(load, output);
try {
  const walletProvider = {...session.walletProvider, submitTx: tx => {
    writeFileSync(join(output, 'submitted.json'), JSON.stringify({at: new Date().toISOString(),
      network: 'stagenet', purpose: 'deploy SherlockRun'}) + '\n', {flag: 'wx', mode: 0o600});
    return session.walletProvider.submitTx(tx);
  }};
  const providers = {zkConfigProvider, walletProvider, midnightProvider: walletProvider,
    publicDataProvider: indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs),
    proofProvider: httpClientProofProvider(endpoints.proof, zkConfigProvider),
    privateStateProvider: session.privateStateProvider};
  const deployed = await deployContract(providers, options);
  const p = deployed.deployTxData.public;
  const receipt = {schema: 'sherlock-run-stagenet-deployment/1', network: 'stagenet', address: p.contractAddress,
    txId: p.txId, txHash: p.txHash, blockHeight: p.blockHeight, deployedAt: new Date().toISOString(),
    wallSeconds: (performance.now() - started) / 1000, sourceSHA256: sha(source),
    verifier: {path: vkPath, bytes: vkRaw.length, sha256: sha(vkRaw)}, constructor: [progHex, initHex]};
  writeFileSync(join(output, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
  const state = await providers.publicDataProvider.queryContractState(p.contractAddress);
  assert.ok(state); checkState(state);
  writeFileSync(join(output, 'contract-state.bin'), state.serialize());
  console.log(JSON.stringify({...receipt, readback: 'program_d, init_d, result, runs and key match'}));
} finally { await session.close(); }
process.exit(0);
