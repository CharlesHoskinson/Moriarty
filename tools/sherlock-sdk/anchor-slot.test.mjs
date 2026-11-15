// T12-SLOT15: the SherlockLedger deploy state with the anchor at top-level slot 15 and the
// prologue rewrite, on the stock onchain runtime of the ledger 9.1.0.1 stack (no proofs).
// usage: SHERLOCK_SDK_ROOT=<sdk with node_modules> node --test anchor-slot.test.mjs
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import test from 'node:test';
import {ANCHOR_SLOT, codec, ledgerDeployState, ledgerDeployStateValue, prologueToSlot15} from './anchor-slot.mjs';

const sdk = process.env.SHERLOCK_SDK_ROOT;
assert.ok(sdk, 'SHERLOCK_SDK_ROOT');
const require = createRequire(resolve(sdk, 'package.json'));
const rt = await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')).href);
const ocrt = await import(pathToFileURL(require.resolve('@midnightntwrk/onchain-runtime-v4')).href);
const c = codec(rt);
const C_L = 0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994n;
const REG = 0x1234n;
// The compactc 0.36.0 prologue of every SherlockLedger entry (compiled ops, read of field 0).
const prologue0 = () => [{dup: {n: 0}}, {idx: {cached: false, pushPath: false, path: [c.key(0)]}}, {popeq: {cached: false, result: null}}];

const run = (state, ops) => {
  const st = new ocrt.VmStack();
  st.push(state, true);
  const r = ocrt.runProgram(st, ops, ocrt.CostModel.initialCostModel());
  return {reads: r.events.filter(e => e.tag === 'read').map(e => c.field(e.content.value)), state: r.stack.get(0)};
};
const readAt = k => [{dup: {n: 0}}, {idx: {cached: false, pushPath: false, path: [c.key(k)]}}, {popeq: {cached: false, result: null}}];

test('deploy state: 16 children, compactc fields at 0 to 3, anchor at 15', () => {
  const enc = ledgerDeployState(rt, C_L, REG);
  assert.equal(enc.content.length, 16);
  assert.deepEqual(enc.content.slice(4, 15).map(x => x.tag), Array(11).fill('null'));
  const s = ledgerDeployStateValue(rt, ocrt, C_L, REG);
  assert.deepEqual(run(s, readAt(ANCHOR_SLOT)).reads, [C_L]);
  assert.deepEqual(run(s, readAt(3)).reads, [REG]);
  assert.throws(() => ledgerDeployState(rt, 0n, REG), /zero anchor/);
});

test('prologue rewrite: idx [0] becomes idx [15], reads the anchor; other shapes refused', () => {
  const s = ledgerDeployStateValue(rt, ocrt, C_L, REG);
  const ops = prologueToSlot15(rt, prologue0());
  assert.deepEqual(ops[1].idx.path, [c.key(ANCHOR_SLOT)]);
  assert.deepEqual(ops[0], prologue0()[0]);
  assert.deepEqual(ops[2], prologue0()[2]);
  assert.deepEqual(run(s, ops).reads, [C_L]);
  assert.throws(() => prologueToSlot15(rt, readAt(3)), /not the SherlockLedger prologue/);
  assert.throws(() => prologueToSlot15(rt, [{dup: {n: 1}}, ...prologue0().slice(1)]), /not the SherlockLedger prologue/);
  assert.throws(() => prologueToSlot15(rt, [prologue0()[0], {idx: {cached: true, pushPath: false, path: [c.key(0)]}}, prologue0()[2]]), /not the SherlockLedger prologue/);
});

test('a chunk anchor write at slot 15 (push key 15; push value; ins 1) is accepted, then read back', () => {
  const s = ledgerDeployStateValue(rt, ocrt, C_L, REG);
  const a1 = 0x77n;
  const w = run(s, [...readAt(ANCHOR_SLOT), {push: {storage: false, value: c.keyCell(ANCHOR_SLOT)}}, {push: {storage: true, value: c.cell(a1)}}, {ins: {cached: false, n: 1}}]);
  assert.deepEqual(w.reads, [C_L]);
  assert.deepEqual(run(w.state, readAt(ANCHOR_SLOT)).reads, [a1]);
  assert.deepEqual(run(w.state, readAt(0)).reads, [C_L], 'field 0 is untouched');
});
