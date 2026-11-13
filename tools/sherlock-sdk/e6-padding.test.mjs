// node --test e6-padding.test.mjs  (no SDK, no devnet). The SDK-level check is `e6-local-run.mjs dry-run`.
import test from 'node:test';
import assert from 'node:assert/strict';
import {scaleEncode, Reader, decodePreimage, encodePreimage, callVector, frameVerifierKey, frameProof, padTranscript,
  assertNormalized, paddedCheckResult, bigToLeHex, FR_MODULUS, VK_TAG} from './e6-padding.mjs';

test('SCALE integers round-trip at every mode boundary', () => {
  for (const v of [0n, 1n, 63n, 64n, 16383n, 16384n, (1n << 30n) - 1n, 1n << 30n, (1n << 32n) - 1n, 1n << 32n, FR_MODULUS - 1n]) {
    const r = new Reader(scaleEncode(v)); assert.equal(r.scale(), v); assert.ok(r.done());
  }
  // XC-9's tagged key: header 0x35 0x22 = 2,189 bytes of MidnightVK; E5's 1,325 bytes = 0xb5 0x14
  assert.deepEqual([...scaleEncode(2189n)], [0x35, 0x22]);
  assert.deepEqual([...scaleEncode(1325n)], [0xb5, 0x14]);
});

test('verifier key and proof framing (raw or tagged in, tagged out)', () => {
  const body = Buffer.alloc(1325, 7);
  const f = frameVerifierKey(body);
  assert.equal(f.tagged.length, 26 + 2 + 1325); assert.equal(f.tagged.toString('latin1', 0, 26), VK_TAG);
  const g = frameVerifierKey(f.tagged);
  assert.equal(g.input, 'tagged'); assert.ok(g.body.equals(body)); assert.ok(g.tagged.equals(f.tagged));
  assert.throws(() => frameVerifierKey(Buffer.concat([f.tagged, Buffer.from([0])])));
  assert.equal(frameProof(Buffer.alloc(2912)).tagged.length, 19 + 2 + 2912);
});

const preimage = words => ({tag: 'proof-preimage', inputs: [5n, FR_MODULUS - 1n], privateTranscript: [],
  publicTranscriptInputs: words, publicTranscriptOutputs: [9n], bindingInput: 0n,
  communicationsCommitment: {comm: 123456789n << 200n, rand: 42n}, keyLocation: 'sherlock/acceptRun'});

test('preimage decode round-trips and yields the ledger call vector', () => {
  const p = preimage([...Array(68).keys()].map(BigInt).concat(Array(956).fill(0n)));
  const d = decodePreimage(encodePreimage(p));
  assert.deepEqual(d, p);
  const v = callVector(d, bigToLeHex(77n));
  assert.equal(v.length, 1026); assert.equal(v[0], 77n); assert.equal(v[1], p.communicationsCommitment.comm);
  assert.throws(() => decodePreimage(Buffer.concat([encodePreimage(p), Buffer.from([0])])), /trailing/);
});

const T = program => ({gas: {readTime: 1n, computeTime: 2n, bytesWritten: 3n, bytesDeleted: 4n}, effects: {}, program});

test('padding appends one Noop with n = target - words and keeps gas', () => {
  const t = T(['pop', {push: {storage: false, value: {tag: 'null'}}}, {ins: {cached: false, n: 1}}]);
  const r = padTranscript(t, 68, 1024);
  assert.equal(r.mode, 'appended'); assert.equal(r.noopN, 956); assert.equal(r.opsAdded, 1);
  assert.deepEqual(r.transcript.program.at(-1), {noop: {n: 956}});
  assert.equal(r.transcript.program.length, 4); assert.equal(t.program.length, 3, 'input untouched');
  assert.deepEqual(r.transcript.gas, t.gas, 'gas is left to Transaction.prove');
  assert.deepEqual(paddedCheckResult([undefined, undefined, undefined], r.opsAdded), [undefined, undefined, undefined, undefined]);
});

test('padding merges into a trailing Noop', () => {
  const r = padTranscript(T(['pop', {noop: {n: 3}}]), 68, 1024);
  assert.equal(r.mode, 'merged'); assert.equal(r.noopN, 959); assert.equal(r.opsAdded, 0);
  assert.equal(r.transcript.program.length, 2);
});

test('no padding at the target; refusal above it', () => {
  const r = padTranscript(T(['pop']), 1024, 1024);
  assert.equal(r.mode, 'none'); assert.equal(r.transcript.program.length, 1);
  assert.throws(() => padTranscript(T(['pop']), 1025, 1024), /above the target/);
});

test('normalization rules of ledger verify.rs', () => {
  assert.throws(() => assertNormalized([{noop: {n: 0}}]), /noop 0/);
  assert.throws(() => assertNormalized(['pop', {noop: {n: 1}}, {noop: {n: 2}}]), /adjacent/);
  assertNormalized([{noop: {n: 1}}, 'pop', {noop: {n: 2}}]);
  assert.throws(() => paddedCheckResult([undefined, 4n], 1), /skips/);
});
