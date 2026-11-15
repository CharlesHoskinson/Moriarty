// FAL-LEDGER-VERIFICATION-1 (MID-002, SLC-001, SLC-004): the deploy key check of t10a-ledger-run.mjs on the
// four-entry SherlockLedger. The wrap key goes on the write entries `acceptRun` and `acceptCollection` only; the
// read entries `rootAccepted` and `runAccepted` keep the compiled native key of the entry (compactc keys/*.verifier).
// Each key is read back from the serialized contract state (the bytes the indexer returns). Control: a read entry
// with the wrap key fails the readback. The three-entry T10/T17 contract keeps the wrap key on every entry.
// Usage: [SHERLOCK_SDK_ROOT=~/.cache/xc9/sdk] [T20_BUILD=~/.cache/t20/build] [T17_BUILD=~/.cache/t17/contract/build]
//        [T17_WRAP_VK=~/.cache/t17/in/wrap.vk] node --test ledger-keys.test.mjs
import assert from 'node:assert/strict';
import {readFileSync, existsSync} from 'node:fs';
import {createRequire} from 'node:module';
import {join, resolve} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import test from 'node:test';
import {compiledEntries, keyPlan, checkInstalled, T10_ENTRIES} from './ledger-keys.mjs';
import {frameVerifierKey} from './e6-padding.mjs';

const H = homedir();
const sdk = resolve(process.env.SHERLOCK_SDK_ROOT ?? join(H, '.cache/xc9/sdk'));
const ledger = await import(pathToFileURL(createRequire(join(sdk, 'package.json')).resolve('@midnightntwrk/ledger-v9')).href);
const t20 = resolve(process.env.T20_BUILD ?? join(H, '.cache/t20/build'), 'sherlock-ledger');
const t17 = resolve(process.env.T17_BUILD ?? join(H, '.cache/t17/contract/build'), 'sherlock-ledger');
const wrapVk = new Uint8Array(frameVerifierKey(readFileSync(process.env.T17_WRAP_VK ?? join(H, '.cache/t17/in/wrap.vk'))).tagged);
const native = build => e => new Uint8Array(readFileSync(join(build, 'keys', `${e}.verifier`)));
const SLC_001 = ['acceptCollection', 'acceptRun', 'rootAccepted', 'runAccepted'];

// Deploy-state stand-in: one ContractOperation per plan entry, serialized and read back as the indexer returns it.
const deployReadback = keys => {
  const s = new ledger.ContractState();
  for (const [e, bytes] of Object.entries(keys)) { const op = new ledger.ContractOperation(); op.verifierKey = bytes; s.setOperation(e, op); }
  const back = ledger.ContractState.deserialize(s.serialize());
  return e => back.operation(e).verifierKey;
};
const bytesOf = plan => Object.fromEntries(Object.entries(plan).map(([e, p]) => [e, p.bytes]));

test('FAL-LEDGER-VERIFICATION-1: four-entry SherlockLedger, wrap key on write entries, native keys on read entries', () => {
  assert.ok(existsSync(join(t20, 'keys')), `T20 build with keys at ${t20} (t20-batched tools/sherlock-sdk/t20-build.sh)`);
  const entries = compiledEntries(join(t20, 'keys'));
  assert.deepEqual(entries, SLC_001, 'SLC-001 entries of the compiled contract');
  const plan = keyPlan(entries, wrapVk, native(t20));
  assert.deepEqual(Object.keys(plan).sort(), SLC_001, 'the plan covers exactly the compiled entries');
  for (const e of ['acceptRun', 'acceptCollection']) assert.equal(plan[e].kind, 'wrap', `MID-002: ${e} holds the wrap key`);
  for (const e of ['rootAccepted', 'runAccepted']) {
    assert.equal(plan[e].kind, 'native', `SLC-004: ${e} holds its compiled native key`);
    assert.ok(Buffer.from(plan[e].bytes).equals(Buffer.from(native(t20)(e))), `${e}: compactc keys/${e}.verifier bytes`);
  }
  const res = checkInstalled(plan, deployReadback(bytesOf(plan)));
  assert.equal(res.ok, true, JSON.stringify(res.rows));
});

test('FAL-LEDGER-VERIFICATION-1 control: a read entry with the wrap key fails the readback', () => {
  const plan = keyPlan(compiledEntries(join(t20, 'keys')), wrapVk, native(t20));
  const wrong = {...bytesOf(plan), rootAccepted: wrapVk};
  const res = checkInstalled(plan, deployReadback(wrong));
  assert.equal(res.ok, false);
  assert.equal(res.rows.rootAccepted.equals, false);
  assert.equal(res.rows.runAccepted.equals, true);
});

test('FAL-LEDGER-VERIFICATION-1: the three-entry T10/T17 contract keeps the wrap key on every entry', () => {
  assert.ok(existsSync(join(t17, 'keys')), `T17 build with keys at ${t17}`);
  const entries = compiledEntries(join(t17, 'keys'));
  assert.deepEqual(entries, T10_ENTRIES);
  const plan = keyPlan(entries, wrapVk, native(t17));
  for (const e of T10_ENTRIES) assert.equal(plan[e].kind, 'wrap', e);
  assert.equal(checkInstalled(plan, deployReadback(bytesOf(plan))).ok, true);
});
