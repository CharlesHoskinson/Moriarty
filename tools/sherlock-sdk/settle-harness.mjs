// Sherlock settle-path harness (interface: qa/holdout/cases/rejections/s-c-wrong-predecessor/case.json
// harnessInterface). Maintainer tooling for the A9 stagenet rejection cases; never a public prerequisite.
//
//   prepareSettle({load, session, snapshot, batch, fields, address?, ledgerParameters?, ttl?, prover?}) -> {tx, report}
//   settleTrace(snapshot, fields) -> bigint       f15 as settleValidity computes it
//   settleStatement(snapshot, batch, fields)      stmt as settleValidity computes it
//   editSnapshot(snapshot, edits), ledgerView(snapshot), predictSettle(...)   edited snapshots, decoding, VM post-state
//
// prepareSettle executes and proves settleValidity against exactly the given snapshot. It never re-fetches,
// compares or refuses on chain contract state, never submits, and never writes or logs a transaction body.
// The only client-side refusal is the circuit itself being unsatisfiable over the snapshot: an Error named
// SettleUnsatisfiable, thrown before the session is touched.
//
// Inputs beyond the interface (optional, with defaults):
//   address           the contract the call targets (a ContractState does not carry it). Default: the A8 pin
//                     in crates/mori-recursion/tests/fixtures/stagenet-verifier.json. Pass it for D2/D3.
//   ledgerParameters  network parameters for the transaction cost model. Default: block.ledgerParameters of
//                     the indexer's latest block (network configuration only; no contract state is read).
//   ttl               a Date, at least 30 minutes ahead. Default: one hour from balancing.
//   prover            {check, lookupKey, prove} for the one settleValidity proof. Default: the local proof
//                     server with the installed compiled key from MORIARTY_ANCHOR_KEYED. Tests stop here.
//   MORIARTY_ANCHOR_BUILD / MORIARTY_ANCHOR_KEYED select the compiled anchor and its keys, with the same
//   defaults as qa/holdout/lib/stagenet/produce-a9.sh.
// The statement uses the snapshot's batch_no (the contract's "current B"); a batch argument that differs is
// unsatisfiable (assert "batch"), and settleStatement throws on it rather than return a statement no call has.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {createRequire} from 'node:module';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {endpoints} from './stagenet-session.mjs';

const REPO = fileURLToPath(new URL('../../', import.meta.url));
export const BUILD = resolve(process.env.MORIARTY_ANCHOR_BUILD ?? join(REPO, 'target/a4-anchor/deploy-run3/build'));
export const KEYED = resolve(process.env.MORIARTY_ANCHOR_KEYED ?? join(REPO, 'target/a4-anchor/keyed'));
export const SETTLE_VK_SHA256 = 'be8b0943d8e2acadeb2fd914e4bcc100bdcce72ffe2a8da4e34f9417141c0b5c';
const PIN = JSON.parse(readFileSync(join(REPO, 'crates/mori-recursion/tests/fixtures/stagenet-verifier.json')));
const entry = join(BUILD, 'contract/index.js');
// The runtime the compiled contract itself resolves, so cells and hashes use one module instance.
const rt = await import(pathToFileURL(createRequire(entry).resolve('@midnight-ntwrk/compact-runtime')).href);
const mod = await import(pathToFileURL(entry).href);
const sha = b => createHash('sha256').update(b).digest('hex');

// Constants of sherlock-anchor.compact (SHK_STMT, SHK_TRACE, envelopes, n_steps, work).
const SHK_STMT = 0x544d54535f4b4853n, SHK_TRACE = 0x45434152545f4b4853n;
const A102_TAG = 3n, A102_CHARGE = 1n, A80_TAG = 0n, A80_CHARGE = 0n, N_STEPS = 2n, WORK = 1n;
const H_MAX_PRE = 2n ** 64n - 3n, B_MAX = 2n ** 64n - 1n;
const F21 = new rt.CompactTypeVector(21, rt.CompactTypeField), F19 = new rt.CompactTypeVector(19, rt.CompactTypeField);
const U64 = new rt.CompactTypeUnsignedInteger(B_MAX, 8);
// Ledger cell order of SherlockAnchor (declaration order; one flat array).
const CELLS = ['validity_ok', 'settled', 'root', 'height', 'batch_no', 'instance_id', 'keys_id', 'a102_ctx_lo',
  'a102_ctx_hi', 'a80_ctx_lo', 'a80_ctx_hi', 'program_c', 'genesis_root', 'genesis_height', 'genesis_batch'];
const U64_CELLS = new Set(['height', 'batch_no', 'genesis_height', 'genesis_batch']);

const bytesOf = snapshot => {
  assert.equal(typeof snapshot?.serialize, 'function', 'snapshot must be a ContractState');
  return Buffer.from(snapshot.serialize());
};
const toState = snapshot => {
  const raw = bytesOf(snapshot), state = rt.ContractState.deserialize(raw);
  assert.deepEqual(Buffer.from(state.serialize()), raw, 'snapshot does not round-trip');
  return state;
};
export const ledgerView = snapshot => mod.ledger(toState(snapshot).data);
const checkFields = fields => {
  assert.ok(Array.isArray(fields) && fields.length === 16, 'fields must be 16 bigints f0..f15');
  assert.ok(fields.every(f => typeof f === 'bigint'), 'fields must be 16 bigints f0..f15');
};
const trace = (v, f) => rt.transientHash(F21, [SHK_TRACE, v.keys_id, v.a102_ctx_lo, v.a102_ctx_hi, v.a80_ctx_lo,
  v.a80_ctx_hi, v.program_c, v.root, v.height, f[11], A102_TAG, A102_CHARGE, f[8], v.height + 1n, f[12], A80_TAG,
  A80_CHARGE, f[9], v.height + 2n, N_STEPS, WORK]);

export function settleTrace(snapshot, fields) {
  checkFields(fields);
  return trace(ledgerView(snapshot), fields);
}

export function settleStatement(snapshot, batch, fields) {
  checkFields(fields);
  assert.equal(typeof batch, 'bigint', 'batch must be a bigint');
  const v = ledgerView(snapshot);
  assert.equal(batch, v.batch_no, 'batch differs from the snapshot batch_no; settleValidity asserts "batch"');
  return rt.transientHash(F19, [SHK_STMT, v.instance_id, v.batch_no, v.keys_id, v.a102_ctx_lo, v.a102_ctx_hi,
    v.a80_ctx_lo, v.a80_ctx_hi, v.program_c, v.root, v.height, fields[8], fields[9], v.height + 2n, fields[11],
    fields[12], N_STEPS, WORK, trace(v, fields)]);
}

// A copy of snapshot with named cells replaced and set members added: {root, height, ..., validity_ok_add: [],
// settled_add: []}. Verifier keys, maintenance authority and balance are kept.
export function editSnapshot(snapshot, edits) {
  const state = toState(snapshot);
  const enc = state.data.state.encode();
  assert.equal(enc.tag, 'array'); assert.equal(enc.content.length, CELLS.length);
  for (const [name, value] of Object.entries(edits)) {
    const set = name.match(/^(validity_ok|settled)_add$/)?.[1];
    if (set) {
      const map = enc.content[CELLS.indexOf(set)];
      assert.equal(map.tag, 'map');
      for (const m of value) {
        assert.equal(typeof m, 'bigint');
        map.content.set({value: rt.CompactTypeField.toValue(m), alignment: rt.CompactTypeField.alignment()}, {tag: 'null'});
      }
      continue;
    }
    const at = CELLS.indexOf(name);
    assert.ok(at >= 2 && typeof value === 'bigint', `unknown or non-bigint cell edit: ${name}`);
    const type = U64_CELLS.has(name) ? U64 : rt.CompactTypeField;
    enc.content[at] = {tag: 'cell', content: {value: type.toValue(value), alignment: type.alignment()}};
  }
  state.data = new rt.ChargedState(rt.StateValue.decode(enc));
  return state;
}

class SettleUnsatisfiable extends Error {
  constructor(message, cause) { super(message, {cause}); this.name = 'SettleUnsatisfiable'; }
}

// The complete contract state after settleValidity(batch, fields) over snapshot, by the compact-runtime VM.
// Throws SettleUnsatisfiable when a circuit assertion fails over the snapshot.
export async function predictSettle(snapshot, batch, fields, address = PIN.address) {
  const clone = toState(snapshot);
  let predicted;
  try {
    const context = rt.createCircuitContext('settleValidity', address, '00'.repeat(32), clone, {});
    predicted = await new mod.Contract({}).provableCircuits.settleValidity(context, batch, fields);
  } catch (error) {
    if (error instanceof rt.CompactError || error?.isCompactError)
      throw new SettleUnsatisfiable(`settleValidity is unsatisfiable over the snapshot: ${error.message}`, error);
    throw error;
  }
  clone.data = predicted.context.callContext.currentQueryContext.state;
  return clone;
}

export async function prepareSettle({load, session, snapshot, batch, fields, address = PIN.address, ledgerParameters, ttl, prover}) {
  // (1) Inputs. No wallet use until the circuit is known to be satisfiable over the snapshot.
  assert.equal(typeof load, 'function', 'load must be the stagenet-session sdkLoader');
  assert.equal(typeof batch, 'bigint', 'batch must be a bigint');
  checkFields(fields);
  assert.match(address, /^[0-9a-f]{64}$/, 'address must be 64 lowercase hex');
  const raw = bytesOf(snapshot), state = toState(snapshot);
  const vk = state.operation('settleValidity')?.verifierKey;
  assert.ok(vk, 'snapshot has no settleValidity operation');
  assert.equal(sha(vk), SETTLE_VK_SHA256, 'snapshot settleValidity key is not the installed compiled key');
  assert.equal(sha(readFileSync(join(KEYED, 'keys/settleValidity.verifier'))), SETTLE_VK_SHA256, 'keyed settleValidity verifier');
  // (2) Independent VM execution over the snapshot: satisfiability and the predicted complete post-state.
  const predictedPostState = Buffer.from((await predictSettle(state, batch, fields, address)).serialize());
  const report = {address, snapshotSHA256: sha(raw), predictedPostStateSHA256: sha(predictedPostState),
    stmt: settleStatement(state, batch, fields), trace: settleTrace(state, fields)};
  // (3) SDK call over the snapshot (no state provider, so no chain read), proof, wallet balancing.
  const {getNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id');
  assert.equal(getNetworkId(), 'stagenet');
  const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
  const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
  const {createUnprovenCallTxFromInitialStates} = await load('@midnight-ntwrk/midnight-js-contracts');
  const {parseCoinPublicKeyToHex, deserializeCompactContractState} = await load('@midnight-ntwrk/midnight-js-utils');
  const {NodeZkConfigProvider} = await load('@midnight-ntwrk/midnight-js-node-zk-config-provider');
  const {httpClientProvingProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
  if (!ledgerParameters) ledgerParameters = await latestLedgerParameters(load);
  const keys = new NodeZkConfigProvider(KEYED);
  const compiledContract = CompiledContract.make('SherlockAnchor', mod.Contract).pipe(
    CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(BUILD));
  const wallet = session.walletProvider;
  const call = await createUnprovenCallTxFromInitialStates(keys, {compiledContract, contractAddress: address,
    circuitId: 'settleValidity', args: [batch, fields],
    coinPublicKey: parseCoinPublicKeyToHex(wallet.getCoinPublicKey(), getNetworkId()),
    initialContractState: deserializeCompactContractState(raw, {caller: 'settle-harness'}),
    initialZswapChainState: new ledger.ZswapChainState(), ledgerParameters}, wallet.getEncryptionPublicKey());
  // The SDK's ChargedState comes from another WASM module instance; cross over through the plain encoding.
  const sdkPost = rt.ContractState.deserialize(raw);
  sdkPost.data = new rt.ChargedState(rt.StateValue.decode(call.public.nextContractState.encode()));   // a StateValue
  assert.equal(sha(sdkPost.serialize()), report.predictedPostStateSHA256, 'SDK and VM post-states differ');
  const base = prover ?? httpClientProvingProvider(endpoints.proof, keys, {timeout: 15 * 60_000});
  const location = `contract:${address}/settleValidity?vk=${SETTLE_VK_SHA256}`;
  let proofs = 0;
  const guarded = {check: (r, loc) => base.check(r, loc), lookupKey: loc => base.lookupKey(loc),
    prove: (r, loc, binding) => {
      assert.equal(++proofs, 1, 'one contract proof per transaction');
      assert.equal(loc, location, 'proof key location');
      return base.prove(r, loc, binding);
    }};
  const proven = await call.private.unprovenTx.prove(guarded, ledgerParameters.transactionCostModel.runtimeCostModel);
  assert.equal(proofs, 1);
  const expires = ttl ?? new Date(Date.now() + 60 * 60_000);
  assert.ok(expires instanceof Date && expires.getTime() - Date.now() >= 30 * 60_000, 'ttl must be at least 30 minutes ahead');
  const tx = await wallet.balanceTx(proven, expires);
  return {tx, report: {...report, ttl: expires.toISOString()}};
}

async function latestLedgerParameters(load) {
  const {parseHexLedgerParameters} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
  const r = await fetch(endpoints.indexer, {method: 'POST', headers: {'content-type': 'application/json'},
    body: JSON.stringify({query: '{block{ledgerParameters}}'})});
  assert.ok(r.ok, 'indexer ledgerParameters query');
  const body = await r.json();
  assert.ok(!body.errors && body.data?.block?.ledgerParameters, 'indexer ledgerParameters query');
  return parseHexLedgerParameters(body.data.block.ledgerParameters);
}
