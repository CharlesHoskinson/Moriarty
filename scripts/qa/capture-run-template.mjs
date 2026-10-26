// Offline acceptRun transcript capture for the general-run decider. No wallet, proving or network.
// Usage: node capture-run-template.mjs BUILD RUNTIME_INDEX_JS LEDGER_FS_JS OUT_TEMPLATE_JSON
// Two captures with pairwise-distinct inputs: equal fields are template constants, and every
// differing field must equal exactly one named input in both captures (it becomes a stmt or value slot).
import assert from 'node:assert/strict';
import {writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const [build, runtimeEntry, ledgerEntry, out] = process.argv.slice(2);
assert.ok(build && runtimeEntry && ledgerEntry && out,
  'usage: node capture-run-template.mjs BUILD RUNTIME_ENTRY LEDGER_ENTRY OUT');
const rt = await import(pathToFileURL(resolve(runtimeEntry)).href);
const ledger = await import(pathToFileURL(resolve(ledgerEntry)).href);
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const modulus = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;

function transcript(raw) {
  const bytes = Buffer.from(raw), tag = Buffer.from('midnight:proof-preimage:');
  assert.ok(bytes.subarray(0, tag.length).equals(tag));
  let at = tag.length;
  const integer = () => {
    const first = bytes[at++], mode = first & 3;
    if (mode === 0) return BigInt(first >>> 2);
    const count = mode === 3 ? (first >>> 2) + 4 : [0, 2, 4][mode];
    let value = mode === 3 ? 0n : BigInt(first);
    for (let i = Number(mode !== 3); i < count; i++) value |= BigInt(bytes[at++]) << BigInt(8 * i);
    return mode === 3 ? value : value >> 2n;
  };
  const vector = () => {
    const count = Number(integer());
    assert.ok(count <= 4096);
    return Array.from({length: count}, () => { const f = integer(); assert.ok(f < modulus); return le(f); });
  };
  vector(); vector();
  return vector();
}

// value index: prog 0, init 1, fin 2, steps 3, result 4; stmt is its own slot.
const runs = [
  {stmt: 4242n, prog: 1111n, init: 1212n, n: 7n, fin: 9999n, r: 6n},
  {stmt: 4343n, prog: 2121n, init: 2222n, n: 8n, fin: 9898n, r: 120n},
];
// The result is Bytes<16> big-endian; its field representation is the little-endian integer of those bytes.
const be16 = v => Buffer.from(v.toString(16).padStart(32, '0'), 'hex');
const leInt = b => BigInt('0x' + (Buffer.from(b).reverse().toString('hex') || '0'));
const named = run => ({stmt: run.stmt, 'value:0': run.prog, 'value:1': run.init, 'value:2': run.fin,
  'value:3': run.n, 'value:4': leInt(be16(run.r))});
const captures = [];
for (const run of runs) {
  const contract = new mod.Contract({});
  const initial = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)), run.prog, run.init,
    new Uint8Array(be16(1n)));
  const context = rt.createCircuitContext('acceptRun', '01'.repeat(32), '00'.repeat(32),
    initial.currentContractState, {});
  const result = await contract.provableCircuits.acceptRun(context, run.stmt, run.prog, run.init, run.n, run.fin,
    new Uint8Array(be16(run.r)));
  const data = result.context.callProofDataTrace[0];
  captures.push(transcript(ledger.proofDataIntoSerializedPreimage(data.input, data.output,
    data.publicTranscript, data.privateTranscriptOutputs, 'sherlock/acceptRun')));
}
assert.equal(captures[0].length, captures[1].length);
const slots = [{kind: 'binding_input'}, {kind: 'communication_commitment'}];
const free = {};
for (let i = 0; i < captures[0].length; i++) {
  const [a, b] = [captures[0][i], captures[1][i]];
  if (a === b) { slots.push({kind: 'const', value: a}); continue; }
  const hits = Object.entries(named(runs[0])).filter(([k, v]) => le(v) === a
    && le(named(runs[1])[k]) === b).map(([k]) => k);
  assert.equal(hits.length, 1, `field ${i} is free but matches ${hits}`);
  const [k] = hits;
  free[k] = (free[k] || 0) + 1;
  slots.push(k === 'stmt' ? {kind: 'stmt'} : {kind: 'value', value: k.split(':')[1]});
}
assert.ok(free.stmt >= 1, 'a stmt slot');
assert.ok(free['value:0'] && free['value:1'] && free['value:4'], 'program, init and result must reach the transcript');
writeFileSync(out, JSON.stringify({slots, source: 'tools/sherlock-sdk/sherlock-run.compact acceptRun',
  provisional: false}, null, 1) + '\n');
console.log(`PASS: ${slots.length} slots; free fields ${JSON.stringify(free)}`);
