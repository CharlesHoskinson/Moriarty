// Prepare public deployment state offline. No wallet, network, or submission.
// Usage: node rollup-prepare.mjs SDK_ROOT BUILD B3_DIR KEY_DIR SETTLE_KEY OUTPUT
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {anchorView, bindBatch} from './rollup-statement.mjs';

const [sdk, build, b3, key, settle, output] = process.argv.slice(2);
assert.ok(output, 'SDK_ROOT BUILD B3_DIR KEY_DIR SETTLE_KEY OUTPUT required');
const require = createRequire(resolve(sdk, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {ZKConfigProvider, createVerifierKey} = await load('@midnight-ntwrk/midnight-js-types');
const {createUnprovenDeployTx} = await load('@midnight-ntwrk/midnight-js-contracts');
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
const runtime = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const manifest = JSON.parse(readFileSync(join(key, 'manifest.json')));
assert.equal(manifest.schema, 'sherlock-b4-preparation/1');
assert.equal(manifest.parent_vk_sha256, sha(readFileSync(join(b3, 'parent.vk'))));
const {fields, id, batch} = bindBatch(JSON.parse(readFileSync(join(b3, 'leaf.instance.json'))),
  JSON.parse(readFileSync(join(b3, 'parent.instance.json'))), manifest, runtime);
// This deployment starts at the public synthetic genesis, before batch one.
assert.equal(fields[15], 1n); assert.equal(batch, 1n);
const constructor = [id, fields[0], 0n, 0n, fields[5], fields[7]];
const paths = {acceptValidity: resolve(key, 'acceptValidity.verifier'), settleValidity: resolve(settle)};
const keys = Object.fromEntries(Object.entries(paths).map(([entry, path]) => {
  const raw = readFileSync(path);
  return [entry, {path, bytes: raw.length, sha256: sha(raw)}];
}));
assert.equal(keys.acceptValidity.sha256, manifest.tagged_vk_sha256);
class ExactKeys extends ZKConfigProvider {
  async getVerifierKey(name) {
    assert.ok(keys[name], 'unexpected entry point');
    const raw = readFileSync(keys[name].path);
    assert.equal(sha(raw), keys[name].sha256);
    return createVerifierKey(raw);
  }
  async getProverKey() { throw Error('offline deployment must not prove a call'); }
  async getZKIR() { throw Error('offline deployment must not prove a call'); }
}
setNetworkId('stagenet');
const compiledContract = CompiledContract.make('SherlockRollupAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(resolve(build)));
const dry = await createUnprovenDeployTx({zkConfigProvider: new ExactKeys(), walletProvider: {
  getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32),
}}, {compiledContract, args: constructor});
const state = dry.public.initialContractState, view = anchorView(runtime, mod, state);
for (const [i, name] of ['instance_id', 'root', 'height', 'batch_no', 'dep_consumed', 'dep_acc'].entries())
  assert.equal(view[name], constructor[i], name);
for (const name of ['last_in_root', 'last_fee_sum', 'last_wd_sum_usd', 'last_wd_sum_night',
  'last_dep_sum_usd', 'last_dep_sum_night']) assert.equal(view[name], 0n, name);
assert.ok(view.validity_ok.isEmpty() && view.settled.isEmpty());
for (const [entry, pin] of Object.entries(keys))
  assert.equal(sha(state.operation(entry).verifierKey), pin.sha256);
mkdirSync(output, {recursive: true});
writeFileSync(join(output, 'contract-state.bin'), state.serialize(), {flag: 'wx'});
writeFileSync(join(output, 'preparation.json'), JSON.stringify({
  schema: 'sherlock-rollup-deployment-preparation/1',
  sourceSHA256: sha(readFileSync(new URL('./rollup-anchor.compact', import.meta.url))),
  constructor: constructor.map(String), keys, all12FieldsEqual: true, setsEmpty: true,
  stateSHA256: sha(state.serialize()), submitted: false,
}, null, 2) + '\n', {flag: 'wx'});
console.log('PASS offline rollup deployment: twelve fields, both empty sets and exact verifier keys');
