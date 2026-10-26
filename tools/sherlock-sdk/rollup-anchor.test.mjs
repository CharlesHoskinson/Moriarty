// Offline S0 anchor semantics only. No wallet, proving, or network calls.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import test from 'node:test';
import {anchorView, bindBatch, assertPredecessor, assertSettled} from './rollup-statement.mjs';

const sdk = process.env.SHERLOCK_SDK_ROOT;
assert.ok(sdk, 'SHERLOCK_SDK_ROOT must identify the pinned SDK package directory');
const require = createRequire(resolve(sdk, 'package.json'));
const rt = await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')).href);
const native = await import(pathToFileURL(require.resolve('@midnight-ntwrk/midnight-js-protocol/ledger')).href);
const build = resolve(process.env.ROLLUP_ANCHOR_BUILD ?? 'target/sherlock-rollup-anchor/build');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
assert.ok(process.env.SHERLOCK_B3_DIR, 'SHERLOCK_B3_DIR must identify a verified B3 run');
const proof = resolve(process.env.SHERLOCK_B3_DIR);
const fromLe = value => BigInt('0x' + Buffer.from(value, 'hex').reverse().toString('hex'));
const fields = JSON.parse(readFileSync(join(proof, 'leaf.instance.json'))).map(fromLe);
const projection = [0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
const domain = fromLe(Buffer.from('DOM_STMT').toString('hex').padEnd(64, '0'));
const statement = (f, id = 1n, batch = 1n) => rt.transientHash(
  new rt.CompactTypeVector(17, rt.CompactTypeField), [domain, id, batch, ...projection.map(i => f[i])]);

async function initial() {
  const contract = new mod.Contract({});
  const state = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)),
    1n, fields[0], 0n, 0n, fields[5], fields[7]);
  return {contract, state: state.currentContractState};
}
async function call(session, entry, ...args) {
  const context = rt.createCircuitContext(entry, '01'.repeat(32), '00'.repeat(32), session.state, {});
  const result = await session.contract.provableCircuits[entry](context, ...args);
  session.state.data = result.context.callContext.currentQueryContext.state;
}

test('terminal projection matches the retained stock D statement', () => {
  const verified = JSON.parse(readFileSync(join(proof, 'verified.json')));
  assert.equal(statement(fields, 0n), fromLe(verified.terminal_statement));
});

test('anchor readback crosses the SDK ledger and compact runtime WASM modules', async () => {
  const session = await initial();
  const state = native.ContractState.deserialize(session.state.serialize());
  const view = anchorView(rt, mod, state);
  assert.equal(view.root, fields[0]);
  assert.equal(view.instance_id, 1n);
});

test('deployment binds the proven leaf, parent, statement and complete predecessor', async () => {
  const leaf = JSON.parse(readFileSync(join(proof, 'leaf.instance.json')));
  const parent = JSON.parse(readFileSync(join(proof, 'parent.instance.json')));
  const le = n => Buffer.from(n.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
  const manifest = {instance_id: le(1n), batch_no: le(1n), statement: le(statement(fields))};
  const bound = bindBatch(leaf, parent, manifest, rt);
  const session = await initial();
  const view = mod.ledger(session.state.data);
  assertPredecessor(view, bound);
  const changed = leaf.slice(); changed[0] = le(fields[0] + 1n);
  assert.throws(() => bindBatch(changed, parent, manifest, rt), /leaf\/parent/);
  assert.throws(() => bindBatch(leaf, parent, {...manifest, statement: le(1n)}, rt), /statement mismatch/);
  assert.throws(() => assertPredecessor({...view, root: view.root + 1n}, bound), /predecessor root/);
});

test('accept then settle advances the complete synthetic anchor observation', async () => {
  const session = await initial();
  const stmt = statement(fields);
  await call(session, 'acceptValidity', stmt);
  await call(session, 'settleValidity', 1n, fields);
  const state = mod.ledger(session.state.data);
  assert.equal(state.root, fields[1]);
  assert.equal(state.height, fields[16]);
  assert.equal(state.batch_no, 1n);
  assert.equal(state.dep_consumed, fields[6]);
  assert.equal(state.dep_acc, fields[8]);
  assert.equal(state.last_in_root, fields[4]);
  assert.equal(state.last_fee_sum, fields[9]);
  assert.equal(state.last_wd_sum_usd, fields[10]);
  assert.equal(state.last_wd_sum_night, fields[11]);
  assert.equal(state.last_dep_sum_usd, fields[12]);
  assert.equal(state.last_dep_sum_night, fields[13]);
  assert.ok(state.settled.member(stmt));
  assert.ok(!state.validity_ok.member(stmt));
  assertSettled(state, {fields, id: 1n, batch: 1n, statement: stmt});
  assert.throws(() => assertSettled({...state, dep_acc: state.dep_acc + 1n},
    {fields, id: 1n, batch: 1n, statement: stmt}), /settled dep_acc/);
  await assert.rejects(call(session, 'acceptValidity', stmt), /replay/);
  await assert.rejects(call(session, 'settleValidity', 1n, fields), /batch/);
});

test('settlement requires proof acceptance and the exact current predecessor', async () => {
  await assert.rejects(call(await initial(), 'settleValidity', 1n, fields), /not accepted/);
  for (const [index, reason] of [[0, /predecessor root/], [15, /predecessor height/],
    [1, /not accepted/], [4, /not accepted/], [7, /deposit predecessor/]]) {
    const session = await initial();
    await call(session, 'acceptValidity', statement(fields));
    const changed = fields.slice(); changed[index] += 1n;
    await assert.rejects(call(session, 'settleValidity', 1n, changed), reason);
  }
});
