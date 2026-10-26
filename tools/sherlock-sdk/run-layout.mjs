// Byte-level E7/E9 analysis of SherlockRun contract states, computed from serialized states only
// (the indexer's `state` bytes for G11; the local ledger states for G9). Nothing is predicted.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

const sha = b => createHash('sha256').update(b).digest('hex');
const all = (hay, needle) => {
  const at = [];
  for (let i = hay.indexOf(needle); i >= 0; i = hay.indexOf(needle, i + 1)) at.push(i);
  return at;
};
export const be16 = v => Buffer.from(v.toString(16).padStart(32, '0'), 'hex');

// E7: the verifier body (after the `midnight:verifier-key[vN]:` header) occurs exactly once in the state,
// starting at most 64 bytes after the end of the entry name.
export function installedKey(state, vkRaw, entry = 'acceptRun') {
  const raw = Buffer.from(vkRaw), header = raw.toString('latin1').match(/^midnight:verifier-key\[v\d+\]:/);
  assert.ok(header, 'tagged verifier key');
  const body = raw.subarray(header[0].length);
  const bodyAt = all(Buffer.from(state), body);
  const nameAt = all(Buffer.from(state), Buffer.from(entry));
  assert.equal(bodyAt.length, 1, `verifier body occurs ${bodyAt.length} times in the state`);
  const gaps = nameAt.map(n => bodyAt[0] - (n + entry.length)).filter(g => g >= 0 && g <= 64);
  assert.equal(gaps.length, 1, 'verifier body within 64 bytes after the entry name');
  return {header: header[0], bodyBytes: body.length, bodySHA256: sha(body), fileSHA256: sha(raw),
    bodyOffset: bodyAt[0], entryNameOffset: bodyAt[0] - gaps[0] - entry.length, gapBytes: gaps[0]};
}

// E9 layout: the unique offset where the deploy state holds be16(initial) and the post state be16(result).
export function resultWindow(deployState, postState, initial, result) {
  const d = Buffer.from(deployState), p = Buffer.from(postState);
  assert.notEqual(initial, result, 'the post result must differ from the initial value');
  const hits = all(d, be16(initial)).filter(o => p.subarray(o, o + 16).equals(be16(result)));
  assert.equal(hits.length, 1, `result window candidates: ${hits.length}`);
  return {encoding: 'u128-be', offset: hits[0], length: 16, initial: initial.toString(),
    deployCandidates: all(d, be16(initial)).length, deployStateSHA256: sha(d), postStateSHA256: sha(p),
    postValue: result.toString()};
}
export const readWindow = (state, layout) =>
  BigInt('0x' + Buffer.from(state).subarray(layout.offset, layout.offset + layout.length).toString('hex'));

// effects == []: the call's guaranteed transcript claims, mints and unshielded flows are all empty.
export function callEffects(action) {
  assert.ok(!action.fallibleTranscript, 'acceptRun is entirely guaranteed');
  const fx = action.guaranteedTranscript.effects;
  const required = ['claimedNullifiers', 'claimedShieldedReceives', 'claimedShieldedSpends', 'claimedContractCalls',
    'shieldedMints', 'unshieldedMints', 'unshieldedInputs', 'unshieldedOutputs'];
  for (const k of required) assert.ok(k in fx, `effect field ${k}`);
  const sizes = {};
  for (const [k, v] of Object.entries(fx))
    sizes[k] = v instanceof Map || v instanceof Set ? v.size : Array.isArray(v) ? v.length
      : v && typeof v === 'object' ? Object.keys(v).length : v ? 1 : 0;
  return {sizes, empty: Object.values(sizes).every(n => n === 0)};
}
