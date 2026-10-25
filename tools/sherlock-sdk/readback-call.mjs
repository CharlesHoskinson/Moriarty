// Independent public readback of the latest acceptValidity call on the pinned contract. Read-only:
// no wallet, no submitter receipt; inputs are the pinned address, the public A4 statement and the indexer.
// The installed SDK JS WASM does not verify proofs cryptographically, so the call proof is re-verified by
// the native proof-verifying validate_call example against the indexed pre-call state.
// Usage: node readback-call.mjs SDK_ROOT BUILD STATEMENT_PUBLIC_JSON OUTPUT
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
const [sdkRoot, build, statementPublic, outputArg] = process.argv.slice(2);
assert.ok(outputArg, 'usage: readback-call.mjs SDK_ROOT BUILD STATEMENT_PUBLIC_JSON OUTPUT');
const output = resolve(outputArg); mkdirSync(output, {recursive: true});
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const URL = 'https://indexer.stagenet.shielded.tools/api/v4/graphql';
const sha = b => createHash('sha256').update(b).digest('hex');
const pins = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/stagenet-verifier.json'));
const stmtLe = JSON.parse(readFileSync(statementPublic))[10];
const stmt = BigInt('0x' + Buffer.from(stmtLe, 'hex').reverse().toString('hex'));
const graphql = async query => {
  const r = await fetch(URL, {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
  assert.ok(r.ok, 'indexer HTTP ' + r.status); const body = await r.json();
  assert.ok(!body.errors, 'indexer error ' + JSON.stringify(body.errors)); return body.data;
};
const actionAt = async height => (await graphql(`{contractAction(address:"${pins.address}"${height === undefined ? ''
  : `,offset:{blockOffset:{height:${height}}}`}){__typename state transaction{hash block{height}} ... on ContractCall{entryPoint}}}`)).contractAction;
// Complete public action history, newest first; one action per block is checked below.
const history = [];
for (let a = await actionAt(); a; ) {
  history.push(a);
  if (a.__typename === 'ContractDeploy') break;
  assert.ok(history.length < 500, 'history too long');
  a = await actionAt(a.transaction.block.height - 1);
}
assert.equal(history.at(-1).__typename, 'ContractDeploy', 'history must reach the deployment');
// The height walk sees one action per block; enumerate each visited block and refuse any block
// holding more than one action on this contract, so no action is skipped silently.
for (const a of history) {
  const {block} = await graphql(`{block(offset:{height:${a.transaction.block.height}}){transactions{hash contractActions{__typename address}}}}`);
  const here = block.transactions.flatMap(t => t.contractActions.filter(c => c.address === pins.address).map(c => ({tx: t.hash, kind: c.__typename})));
  assert.deepEqual(here, [{tx: a.transaction.hash, kind: a.__typename}], `block ${a.transaction.block.height} must hold exactly this one action`);
}
const index = history.findIndex(a => a.__typename === 'ContractCall' && a.entryPoint === 'acceptValidity');
assert.ok(index >= 0, 'no acceptValidity ContractCall on the pinned contract');
const call = history[index], pre = history[index + 1], deploy = history.at(-1);
const txq = h => graphql(`{transactions(offset:{hash:"${h}"}){hash raw block{height hash timestamp}
  ... on RegularTransaction{fee transactionResult{status}}}}`);
const [tx] = (await txq(call.transaction.hash)).transactions;
const [deployTx] = (await txq(deploy.transaction.hash)).transactions;
assert.equal(tx.transactionResult.status, 'SUCCESS', 'call transaction result');
const raw = Buffer.from(tx.raw, 'hex');
const decoded = ledger.Transaction.deserialize('signature', 'proof', 'binding', raw);
const actions = [...decoded.intents.values()].flatMap(i => i.actions);
assert.equal(actions.length, 1, 'exactly one contract action');
assert.ok(actions[0] instanceof ledger.ContractCall); assert.equal(actions[0].address, pins.address);
const entry = actions[0].entryPoint;
assert.equal(typeof entry === 'string' ? entry : Buffer.from(entry).toString('utf8'), 'acceptValidity');
// Bootstrap an isolated ledger holding the indexed deployment, then the indexed pre-call state.
const deployTxDecoded = ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(deployTx.raw, 'hex'));
const deployments = [...deployTxDecoded.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
assert.equal(deployments.length, 1); assert.equal(deployments[0].address, pins.address);
const preState = ledger.ContractState.deserialize(Buffer.from(pre.state, 'hex'));
for (const name of ['acceptValidity', 'settleValidity'])
  assert.equal(sha(preState.operation(name).verifierKey), sha(deployments[0].initialState.operation(name).verifierKey), name + ' key unchanged since deployment');
const secs = BigInt(Math.floor(new Date(tx.block.timestamp).getTime() / 1000));
const block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
  verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
let local = ledger.LedgerState.blank('stagenet');
const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined,
  ledger.Intent.new(new Date((Number(secs) + 3600) * 1000)).addDeploy(deployments[0])).eraseProofs();
const [boot, bootResult] = local.apply(setup.wellFormed(local, lax, new Date(Number(secs) * 1000)), new ledger.TransactionContext(local, block));
assert.equal(bootResult.type, 'success');
local = boot.updateIndex(pins.address, preState.data, preState.balance);
const native = await new Promise((ok, reject) => {
  const p = spawn(resolve('target/release/examples/validate_call'), [], {stdio: ['pipe', 'pipe', 'pipe']});
  let out = '', err = ''; p.stdout.on('data', b => { out += b; }); p.stderr.on('data', b => { err += b; });
  p.on('error', reject);
  p.on('close', code => { writeFileSync(join(output, 'native-stderr.log'), err); code === 0 ? ok(JSON.parse(out)) : reject(Error('validate_call failed')); });
  p.stdin.end(JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'), transaction: raw.toString('hex'), block_seconds: Number(secs)}));
});
assert.equal(native.proofVerification, 'native/proof-verifying');
assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
assert.equal(native.contractCalls.length, 1);
assert.equal(native.contractCalls[0].publicInputs[10], stmtLe, 'call proof is bound to the public A4 statement');
// Complete effects: deterministic replay of the verified call over the indexed pre-state equals the indexed post-state.
const [replayed, replayResult] = local.apply(decoded.wellFormed(local, lax, new Date(Number(secs) * 1000)), new ledger.TransactionContext(local, block));
assert.equal(replayResult.type, 'success');
const norm = bytes => Buffer.from(rt.ContractState.deserialize(bytes).serialize());
const observedPost = norm(Buffer.from(call.state, 'hex'));
assert.deepEqual(norm(replayed.index(pins.address).serialize()), observedPost, 'complete contract effects');
const view = bytes => mod.ledger(rt.ContractState.deserialize(bytes).data);
const before = view(Buffer.from(pre.state, 'hex')), after = view(observedPost);
assert.ok(!before.validity_ok.member(stmt)); assert.ok(after.validity_ok.member(stmt)); assert.ok(after.settled.isEmpty());
for (const f of ['instance_id', 'keys_id', 'a102_ctx_lo', 'a102_ctx_hi', 'a80_ctx_lo', 'a80_ctx_hi', 'program_c', 'root', 'height', 'batch_no'])
  assert.equal(after[f], before[f], f + ' unchanged');
writeFileSync(join(output, 'post-state.bin'), observedPost);
const receipt = {schema: 'sherlock-a10-public-readback/1', network: 'stagenet', address: pins.address,
  deciderVkSHA256: pins.vk_sha256, deployTx: deploy.transaction.hash, deployHeight: deploy.transaction.block.height,
  callTx: tx.hash, callHeight: tx.block.height, callBlockHash: tx.block.hash, callTimestamp: tx.block.timestamp,
  status: tx.transactionResult.status, fee: tx.fee, transactionSHA256: sha(raw), proofSHA256: native.contractCalls[0].proofSHA256,
  statement: stmtLe, publicInputs: native.contractCalls[0].publicInputs, nativeProofVerified: true,
  preStateSHA256: sha(Buffer.from(pre.state, 'hex')), postStateSHA256: sha(observedPost), completeEffectsMatch: true,
  validityStatementPresent: true, settledEmpty: true, rootHeightBatchUnchanged: true,
  historyNewestFirst: history.map(a => ({kind: a.__typename, entryPoint: a.entryPoint ?? null, tx: a.transaction.hash, height: a.transaction.block.height}))};
writeFileSync(join(output, 'receipt.json'), JSON.stringify(receipt, null, 2) + '\n');
console.log('PASS A10: independent readback of', tx.hash, 'block', tx.block.height, '; native proof verified; complete effects match');
