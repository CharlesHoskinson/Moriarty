// Word scanner of the public surfaces of a Compact contract call (T10a, ZK-002 / FP-6 privacy scan).
// No SDK import: the walkers are duck-typed over compact-runtime and ledger-v9 objects alike.
// Every surface is reduced to a list of field words (BigInt) and a forbidden value is matched by word,
// never by raw bytes (a 2-byte n_steps would match random key or hash bytes).
//   callVector     the ledger's statement [binding_input, comm] ++ public transcript inputs (or the transcript
//                  inputs alone when no transaction exists yet)
//   transcriptOps  every aligned-value atom inside the public transcript ops (as compact-runtime records them)
//   callInputs     the circuit's input and output atoms (the call data the proof binds)
//   state          every cell atom, map key and map value of the contract state, walked as a StateValue tree
import assert from 'node:assert/strict';

const leToBig = bytes => {
  const b = Buffer.from(bytes);
  let v = 0n;
  for (let i = b.length - 1; i >= 0; i--) v = (v << 8n) | BigInt(b[i]);
  return v;
};
const atomBytes = x => x instanceof Uint8Array ? x : typeof x === 'string' && /^([0-9a-f]{2})*$/i.test(x) ? Buffer.from(x, 'hex') : null;

// Atoms of an AlignedValue {value: [bytes...], alignment: [...]}: one word per atom (empty atom = 0).
export function alignedWords(av) {
  assert.ok(av && Array.isArray(av.value), 'aligned value');
  return av.value.map(atomBytes).filter(b => b !== null).map(leToBig);
}

// Every AlignedValue nested anywhere in a JS value (transcript ops, proof data).
export function nestedWords(v, out = []) {
  if (Array.isArray(v)) { v.forEach(x => nestedWords(x, out)); return out; }
  if (v && typeof v === 'object' && !(v instanceof Uint8Array)) {
    if (Array.isArray(v.value) && Array.isArray(v.alignment)) { out.push(...alignedWords(v)); return out; }
    for (const x of Object.values(v)) nestedWords(x, out);
    if (typeof v.n === 'number') out.push(BigInt(v.n)); // dup/swap/noop counts are words too
  }
  if (typeof v === 'bigint') out.push(v);
  return out;
}

// Every word of a StateValue tree (ChargedState: pass .state). Map keys and values both count.
export function stateWords(sv, out = []) {
  const t = sv.type();
  if (t === 'null') return out;
  if (t === 'cell') { out.push(...alignedWords(sv.asCell())); return out; }
  if (t === 'array') { for (const x of sv.asArray()) stateWords(x, out); return out; }
  if (t === 'map') {
    const m = sv.asMap();
    for (const k of m.keys()) { out.push(...alignedWords(k)); stateWords(m.get(k), out); }
    return out;
  }
  if (t === 'boundedMerkleTree') { out.push(...nestedWords(JSON.parse(JSON.stringify(sv.asBoundedMerkleTree?.() ?? {})))); return out; }
  assert.fail(`state value type ${t}`);
}
export const contractStateWords = cs => stateWords(cs.data?.state ?? cs.data ?? cs);

// forbid: [{label, value: BigInt}], surfaces: {name: BigInt[]}. Returns one row per forbidden value.
export function scan(surfaces, forbid) {
  const sets = Object.fromEntries(Object.entries(surfaces).map(([k, ws]) => [k, new Set(ws.map(BigInt))]));
  return forbid.map(({label, value}) => {
    const v = BigInt(value);
    const foundIn = Object.keys(sets).filter(k => sets[k].has(v));
    return {label, value: v.toString(), found: foundIn.length > 0, foundIn};
  });
}
export const surfaceSizes = surfaces => Object.fromEntries(Object.entries(surfaces).map(([k, v]) => [k, v.length]));

// base 2^31 packing of up to 8 BabyBear words, sum_i b_i 2^(31 i) (baby_bear_bls12381_poseidon.rs pack_words)
export const packWords = ws => { assert.ok(ws.length <= 8); return ws.reduceRight((acc, w) => acc * (1n << 31n) + BigInt(w), 0n); };
