// E9: independent public readback of a SherlockRun deployment and its acceptRun call (G11 readback half,
// G12 input). Read-only: no wallet and no submitter receipt. Inputs are the address, the pinned run-decider
// key file, the `mori simulate` reference result and the public indexer. The installed SDK JS WASM does not
// verify proofs, so the call proof is re-verified by the native proof-verifying validator against the
// indexed pre-call state.
// Usage: node readback-run.mjs SDK_ROOT BUILD ADDRESS VERIFIER REFERENCE_RESULT OUTPUT
//   SHERLOCK_VALIDATE_CALL  native validator (default target/release/examples/validate_call)
// Writes OUTPUT/readback.json, OUTPUT/state-layout.json (computed from the indexed deploy and post bytes),
// OUTPUT/deploy-state.bin and OUTPUT/post-state.bin (the indexer's bytes, unmodified).
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {installedKey, resultWindow, readWindow, be16, callEffects} from './run-layout.mjs';

const [sdkRoot, build, address, vkPath, referenceArg, outputArg] = process.argv.slice(2);
assert.ok(outputArg, 'usage: readback-run.mjs SDK_ROOT BUILD ADDRESS VERIFIER REFERENCE_RESULT OUTPUT');
assert.match(address, /^[0-9a-f]{64}$/);
const reference = BigInt(referenceArg);
const output = resolve(outputArg); mkdirSync(output, {recursive: true});
const validateCall = resolve(process.env.SHERLOCK_VALIDATE_CALL ?? 'target/release/examples/validate_call');
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const INDEXER = 'https://indexer.stagenet.shielded.tools/api/v4/graphql';
const sha = b => createHash('sha256').update(b).digest('hex');
const vkRaw = readFileSync(vkPath);
const graphql = async query => {
  const r = await fetch(INDEXER, {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
  assert.ok(r.ok, 'indexer HTTP ' + r.status); const body = await r.json();
  assert.ok(!body.errors, 'indexer error ' + JSON.stringify(body.errors)); return body.data;
};
const txq = async h => (await graphql(`{transactions(offset:{hash:"${h}"}){hash raw block{height hash timestamp}
  ... on RegularTransaction{fee transactionResult{status}}}}`)).transactions;
// The newest action must be the acceptRun call; its deployment comes from the call itself.
const {contractAction: call} = await graphql(`{contractAction(address:"${address}"){__typename state
  transaction{hash block{height}} ... on ContractCall{entryPoint deploy{state transaction{hash block{height}}}}}}`);
assert.ok(call, 'no contract action');
assert.equal(call.__typename, 'ContractCall'); assert.equal(call.entryPoint, 'acceptRun');
const deploy = call.deploy, callHeight = call.transaction.block.height;
// E7: the deploy transaction is indexed exactly once with SUCCESS.
const deployTxs = await txq(deploy.transaction.hash);
assert.equal(deployTxs.length, 1, 'deploy transaction indexed exactly once');
assert.equal(deployTxs[0].transactionResult.status, 'SUCCESS', 'deploy transaction result');
const callTxs = await txq(call.transaction.hash);
assert.equal(callTxs.length, 1, 'call transaction indexed exactly once');
const [tx] = callTxs;
assert.equal(tx.transactionResult.status, 'SUCCESS', 'call transaction result');
// Exactly one action on this contract in the call block, so the previous-height state is the exact pre-state.
{
  const {block} = await graphql(`{block(offset:{height:${callHeight}}){transactions{hash contractActions{__typename address}}}}`);
  const here = block.transactions.flatMap(t => t.contractActions.filter(c => c.address === address).map(c => ({tx: t.hash, kind: c.__typename})));
  assert.deepEqual(here, [{tx: call.transaction.hash, kind: 'ContractCall'}], `block ${callHeight} must hold exactly this one action`);
}
const {contract: pre} = await graphql(`{contract(address:"${address}",offset:{height:${callHeight - 1}}){state}}`);
assert.ok(pre, 'no indexed state before the call');
const deployBytes = Buffer.from(deploy.state, 'hex'), preBytes = Buffer.from(pre.state, 'hex'), postBytes = Buffer.from(call.state, 'hex');
writeFileSync(join(output, 'deploy-state.bin'), deployBytes); writeFileSync(join(output, 'post-state.bin'), postBytes);
// E7/E9: the pinned key is installed in the deploy and post states (operation and byte checks).
const op = bytes => ledger.ContractState.deserialize(bytes).operation('acceptRun').verifierKey;
for (const [name, bytes] of [['deploy', deployBytes], ['pre', preBytes], ['post', postBytes]])
  assert.equal(sha(op(bytes)), sha(vkRaw), `${name} state acceptRun key is the pinned run-decider key`);
const deployKey = installedKey(deployBytes, vkRaw), postKey = installedKey(postBytes, vkRaw);
// E9: results, decoded through the contract and through the byte window computed from the indexed bytes.
const view = bytes => mod.ledger(rt.ContractState.deserialize(bytes).data);
const d = view(deployBytes), p = view(postBytes);
const initial = BigInt('0x' + Buffer.from(d.result).toString('hex'));
const posted = BigInt('0x' + Buffer.from(p.result).toString('hex'));
assert.equal(posted, reference, 'post-call result equals the mori simulate reference');
assert.notEqual(posted, initial, 'post-call result differs from the initial value');
assert.equal(sha(deployBytes), sha(preBytes), 'no other action between deploy and call');
const layout = resultWindow(deployBytes, postBytes, initial, posted);
assert.equal(readWindow(deployBytes, layout), initial); assert.equal(readWindow(postBytes, layout), posted);
assert.deepEqual(Buffer.from(d.result), be16(initial));
assert.equal(p.program_d, d.program_d); assert.equal(p.init_d, d.init_d);
// Transaction: one acceptRun call and no other effects.
const raw = Buffer.from(tx.raw, 'hex');
const decoded = ledger.Transaction.deserialize('signature', 'proof', 'binding', raw);
const actions = [...decoded.intents.values()].flatMap(i => i.actions);
assert.equal(actions.length, 1); assert.ok(actions[0] instanceof ledger.ContractCall); assert.equal(actions[0].address, address);
const effects = callEffects(actions[0]);
assert.ok(effects.empty, 'effects == []: ' + JSON.stringify(effects.sizes));
// Native proof re-verification over an isolated ledger with the indexed deployment and pre-call state.
const [deployTx] = deployTxs;
const deployments = [...ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(deployTx.raw, 'hex'))
  .intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
assert.equal(deployments.length, 1); assert.equal(deployments[0].address, address);
const secs = BigInt(Math.floor(new Date(tx.block.timestamp).getTime() / 1000));
const block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
  verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
let local = ledger.LedgerState.blank('stagenet');
const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined,
  ledger.Intent.new(new Date((Number(secs) + 3600) * 1000)).addDeploy(deployments[0])).eraseProofs();
const [boot, bootResult] = local.apply(setup.wellFormed(local, lax, new Date(Number(secs) * 1000)), new ledger.TransactionContext(local, block));
assert.equal(bootResult.type, 'success');
const preState = ledger.ContractState.deserialize(preBytes);
local = boot.updateIndex(address, preState.data, preState.balance);
const native = await new Promise((ok, reject) => {
  const c = spawn(validateCall, [], {stdio: ['pipe', 'pipe', 'pipe']});
  let out = '', err = ''; c.stdout.on('data', b => { out += b; }); c.stderr.on('data', b => { err += b; });
  c.on('error', reject);
  c.on('close', code => { writeFileSync(join(output, 'native-stderr.log'), err); code === 0 ? ok(JSON.parse(out)) : reject(Error('validate_call failed')); });
  c.stdin.end(JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'), transaction: raw.toString('hex'), block_seconds: Number(secs)}));
});
assert.equal(native.proofVerification, 'native/proof-verifying');
assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
// Complete effects: replay of the verified call over the indexed pre-state equals the indexed post-state.
const [replayed, replayResult] = local.apply(decoded.wellFormed(local, lax, new Date(Number(secs) * 1000)), new ledger.TransactionContext(local, block));
assert.equal(replayResult.type, 'success');
const norm = bytes => Buffer.from(rt.ContractState.deserialize(bytes).serialize());
assert.deepEqual(norm(replayed.index(address).serialize()), norm(postBytes), 'complete contract effects');
const stateLayout = {schema: 'sherlock-run-state-layout/1', address, field: 'result', ...layout,
  derivedFrom: 'indexed deploy and post contract state bytes'};
writeFileSync(join(output, 'state-layout.json'), JSON.stringify(stateLayout, null, 2) + '\n');
const record = {schema: 'sherlock-run-public-readback/1', network: 'stagenet', address, indexer: INDEXER,
  deployTx: deploy.transaction.hash, deployHeight: deploy.transaction.block.height, deployStatus: 'SUCCESS', deployIndexedOnce: true,
  callTx: tx.hash, callHeight: tx.block.height, callBlockHash: tx.block.hash, callTimestamp: tx.block.timestamp,
  callStatus: tx.transactionResult.status, fee: tx.fee, entryPoint: 'acceptRun', transactionSHA256: sha(raw),
  proofSHA256: native.contractCalls[0].proofSHA256, publicInputs: native.contractCalls[0].publicInputs, nativeProofVerified: true,
  verifier: {path: vkPath, ...deployKey}, keyInstalledInPostState: postKey.bodySHA256 === deployKey.bodySHA256,
  deployResult: initial.toString(), postResult: posted.toString(), referenceResult: reference.toString(),
  programDigest: d.program_d.toString(16), initDigest: d.init_d.toString(16), steps: p.steps.toString(),
  deployStateSHA256: sha(deployBytes), postStateSHA256: sha(postBytes), stateLayout: {path: join(output, 'state-layout.json'),
    sha256: sha(readFileSync(join(output, 'state-layout.json')))}, effects: [], effectFieldSizes: effects.sizes, completeEffectsMatch: true};
writeFileSync(join(output, 'readback.json'), JSON.stringify(record, null, 2) + '\n');
console.log(`PASS E9 readback ${address}: deploy result ${initial} -> ${posted} (reference ${reference}); key pinned in both states; effects []`);
process.exit(0);
