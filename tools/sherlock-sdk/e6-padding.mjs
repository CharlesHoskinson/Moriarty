// E6 helpers (no SDK, no network): midnight-serialize SCALE integers, the tagged ProofPreimage decoder, the
// verifier-key and proof framings, and the trailing-Noop padding of a guaranteed transcript.
// Sources (midnight-ledger a2b30128 = ledger-9.1.0.1):
//   serialize/src/util.rs:227-300      ScaleBigInt (u32, u64 and Fr all serialize through it)
//   serialize/src/serializable.rs:47   Vec<T> = u32 length, then the elements; str = u64 length, then bytes
//   transient-crypto/src/proofs.rs:720 ProofPreimage field order
//   onchain-vm/src/ops.rs:509-519      Noop { n } = n zero field words
//   ledger/src/verify.rs:2001-2011     refuses Noop { n: 0 } and two adjacent Noops (NotNormalized)
//   ledger/src/prove.rs:253-370        the builder merges adjacent Noops and adds
//                                      noop_constant + noop_coeff_arg * n to the declared compute_time
import assert from 'node:assert/strict';

export const FR_MODULUS = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;
export const VK_TAG = 'midnight:verifier-key[v7]:';
export const PROOF_TAG = 'midnight:proof[v5]:';

// ---- SCALE integers (ScaleBigInt), little-endian, up to 32 bytes for Fr.
export function scaleEncode(v) {
  assert.ok(typeof v === 'bigint' && v >= 0n);
  if (v < 1n << 6n) return Buffer.from([Number(v << 2n)]);
  if (v < 1n << 14n) { const x = Number((v << 2n) | 1n); return Buffer.from([x & 0xff, x >> 8]); }
  if (v < 1n << 30n) { const b = Buffer.alloc(4); b.writeUInt32LE(Number((v << 2n) | 2n)); return b; }
  const bytes = [];
  for (let x = v; x > 0n; x >>= 8n) bytes.push(Number(x & 0xffn));
  assert.ok(bytes.length >= 4 && bytes.length <= 67);
  return Buffer.from([((bytes.length - 4) << 2) | 3, ...bytes]);
}
export class Reader {
  constructor(buf) { this.b = Buffer.from(buf); this.o = 0; }
  byte() { assert.ok(this.o < this.b.length, 'truncated'); return this.b[this.o++]; }
  take(n) { assert.ok(this.o + n <= this.b.length, 'truncated'); const s = this.b.subarray(this.o, this.o + n); this.o += n; return s; }
  scale() {
    const first = this.byte();
    switch (first & 3) {
      case 0: return BigInt(first >> 2);
      case 1: { const s = this.byte(); return BigInt(((s << 8) | first) >> 2); }
      case 2: { const r = this.take(3); return BigInt(((r[2] << 24) >>> 0) + (r[1] << 16) + (r[0] << 8) + first) >> 2n; }
      default: { const r = this.take((first >> 2) + 4); let v = 0n; for (let i = r.length - 1; i >= 0; i--) v = (v << 8n) | BigInt(r[i]); return v; }
    }
  }
  fr() { const v = this.scale(); assert.ok(v < FR_MODULUS, 'non-canonical Fr'); return v; }
  frVec() { const n = Number(this.scale()); const out = []; for (let i = 0; i < n; i++) out.push(this.fr()); return out; }
  done() { return this.o === this.b.length; }
}
const encFrVec = v => Buffer.concat([scaleEncode(BigInt(v.length)), ...v.map(scaleEncode)]);

// ---- tagged ProofPreimage (what the ledger hands the proving provider's `prove` and `check`).
export function decodePreimage(raw) {
  const buf = Buffer.from(raw), m = buf.toString('latin1').match(/^midnight:([^:]+):/);
  assert.ok(m && m[1].startsWith('proof-preimage'), `not a tagged proof preimage (tag ${m?.[1]})`);
  const r = new Reader(buf.subarray(m[0].length));
  const p = {tag: m[1], inputs: r.frVec(), privateTranscript: r.frVec(), publicTranscriptInputs: r.frVec(),
    publicTranscriptOutputs: r.frVec(), bindingInput: r.fr()};
  const opt = r.byte();
  assert.ok(opt === 0 || opt === 1, 'Option marker');
  p.communicationsCommitment = opt ? {comm: r.fr(), rand: r.fr()} : null;
  p.keyLocation = r.take(Number(r.scale())).toString('utf8');
  assert.ok(r.done(), 'trailing bytes after the proof preimage');
  assert.deepEqual(encodePreimage(p), buf, 'preimage re-encodes to the same bytes');
  return p;
}
export function encodePreimage(p) {
  const kl = Buffer.from(p.keyLocation, 'utf8');
  return Buffer.concat([Buffer.from(`midnight:${p.tag}:`, 'latin1'), encFrVec(p.inputs), encFrVec(p.privateTranscript),
    encFrVec(p.publicTranscriptInputs), encFrVec(p.publicTranscriptOutputs), scaleEncode(p.bindingInput),
    p.communicationsCommitment ? Buffer.concat([Buffer.from([1]), scaleEncode(p.communicationsCommitment.comm),
      scaleEncode(p.communicationsCommitment.rand)]) : Buffer.from([0]),
    scaleEncode(BigInt(kl.length)), kl]);
}
export const leHexToBig = h => { assert.match(h, /^[0-9a-f]{64}$/); return BigInt('0x' + Buffer.from(h, 'hex').reverse().toString('hex')); };
export const bigToLeHex = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');

// The ledger's statement for one ContractCall: [binding_input, comm] ++ public_transcript_inputs
// (ledger/src/verify.rs public_inputs; sherlock_gp.rs gp_call_prove).
export function callVector(preimage, bindingLeHex) {
  assert.ok(preimage.communicationsCommitment, 'call commitment present');
  return [leHexToBig(bindingLeHex), preimage.communicationsCommitment.comm, ...preimage.publicTranscriptInputs];
}

// ---- framings. The ledger's VerifierKey and Proof are Vec<u8> newtypes: tag, SCALE length, bytes.
function frame(tag, bytes) {
  const b = Buffer.from(bytes);
  if (b.toString('latin1', 0, tag.length) === tag) {
    const r = new Reader(b.subarray(tag.length)), n = Number(r.scale());
    assert.equal(r.o + n, b.length - tag.length, `${tag} length prefix`);
    return {tagged: b, body: b.subarray(tag.length + r.o), input: 'tagged'};
  }
  assert.ok(!b.toString('latin1', 0, 9).startsWith('midnight:'), `unexpected tag ${b.toString('latin1', 0, 40)}`);
  return {tagged: Buffer.concat([Buffer.from(tag, 'latin1'), scaleEncode(BigInt(b.length)), b]), body: b, input: 'raw'};
}
export const frameVerifierKey = bytes => frame(VK_TAG, bytes);
export const frameProof = bytes => frame(PROOF_TAG, bytes);

// ---- padding. `words` = public transcript words of the unpadded program (from its decoded preimage),
// `target` = wanted transcript words (1,024 for a 1,026-input wrap key). Appends one Noop, or grows the
// trailing Noop, so the program stays normalized. Gas is NOT changed here: Transaction.prove (ledger
// prove.rs) adds the Noop's cost to compute_time as it does for every Noop in the program.
export function padTranscript(transcript, words, target) {
  assert.ok(transcript && Array.isArray(transcript.program), 'guaranteed transcript');
  const pad = target - words;
  assert.ok(Number.isInteger(pad) && pad >= 0, `transcript has ${words} words, above the target ${target}`);
  const program = [...transcript.program];
  const last = program.at(-1);
  let mode = 'none';
  if (pad > 0 && last && typeof last === 'object' && 'noop' in last) {
    program[program.length - 1] = {noop: {n: last.noop.n + pad}}; mode = 'merged';
  } else if (pad > 0) { program.push({noop: {n: pad}}); mode = 'appended'; }
  const out = {...transcript, program};
  assertNormalized(out.program);
  return {transcript: out, pad, mode, noopN: mode === 'none' ? 0 : out.program.at(-1).noop.n, opsAdded: mode === 'appended' ? 1 : 0};
}
export function assertNormalized(program) {
  const isNoop = op => op && typeof op === 'object' && 'noop' in op;
  program.forEach((op, i) => {
    if (isNoop(op)) assert.ok(op.noop.n > 0, `noop 0 at ${i}`);
    if (i > 0 && isNoop(op) && isNoop(program[i - 1])) assert.fail(`adjacent noops at ${i - 1}, ${i}`);
  });
}
// The check result the builder needs for the padded program: one `undefined` (no skip) per op it reads.
export function paddedCheckResult(unpaddedSkips, opsAdded) {
  assert.ok(unpaddedSkips.every(s => s === undefined), `the unpadded check asks for skips: ${unpaddedSkips}`);
  return [...unpaddedSkips, ...Array(opsAdded).fill(undefined)];
}
