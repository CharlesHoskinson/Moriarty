import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {makeCallProviderLink, LIMITATION, TRANSPORT_LIMITATION, MAX_PROOF_BYTES} from './B4-provider-link.mjs';

const KEY = 'sherlock/acceptValidity', H = c => c.repeat(64), COMM = H('c');
const enc = s => new TextEncoder().encode(s), sha = b => createHash('sha256').update(b).digest('hex'), hex = b => Buffer.from(b).toString('hex');
// Verifier keys as prove.rs peek_tag sees them: 'midnight:verifier-key[vN]:' then an opaque body.
const VK = {v5: enc('midnight:verifier-key[v5]:vk'), v6: enc('midnight:verifier-key[v6]:vk'), v7: enc('midnight:verifier-key[v7]:vk')};
const EXPECT = {stmt: H('1'), instance_id: H('2'), batch_no: H('3'), verifier_sha256: H('4'), verifier_key_sha256: sha(VK.v6)};
const BRANDS = new WeakSet();
const CTX = Object.freeze({schema: 'sherlock-sealed-call-context/1', network: 'stagenet', address: 'abcd', entry_point: 'acceptValidity', deployment_record_sha256: H('5')});
BRANDS.add(CTX);
const normAddr = s => { if (typeof s !== 'string') throw Error('address'); return s.toLowerCase().replace(/^0x/, ''); };
const le = n => Buffer.from(n.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
let n = 0;
const cat = (...p) => Uint8Array.from(p.flatMap(x => [...x]));
// SCALE compact u32 (util.rs ScaleBigInt layout), pinned by literal vectors in the tests below.
const scale = k => k < 64 ? [k << 2] : k < 1 << 14 ? [(k & 63) << 2 | 1, k >> 6] : [(k & 63) << 2 | 2, k >> 6 & 255, k >> 14 & 255, k >> 22 & 255];
// Rust-layout length reader for the fake codec only (1/2/4-byte modes).
const unscale = (b, i) => { const w = [1, 2, 4][b[i] & 3]; if (!w) throw Error('n-byte length'); let v = 0; for (let j = w - 1; j >= 0; j--) v = v * 256 + b[i + j]; return [Math.floor(v / 4), i + w]; };
const BARE = 'midnight:proof[v5]:', VERS = 'midnight:proof-versioned:';
const body = raw => cat(scale(raw.length), raw), bare = raw => cat(enc(BARE), body(raw)), versioned = (raw, d) => cat(enc(VERS), [d], body(raw));
const bodyOf = t => t.subarray(Buffer.from(t.subarray(0, BARE.length)).toString('latin1') === BARE ? BARE.length : VERS.length + 1);
const stored = (t, d = 1) => cat(enc(VERS), [d], bodyOf(t));
const rawOf = (k = 8) => { const r = new Uint8Array(k).fill(0xab); new DataView(r.buffer).setUint32(0, ++n); return r; };
const fresh = () => bare(rawOf());
const code = c => e => e?.code === c;
// Controlled read port: the first read of path p parks on a deferred until release(); reached settles when that read is issued.
const parkRead = p => {
  let release, reach, parked = false;
  const gate = new Promise(r => { release = r; }), reached = new Promise(r => { reach = r; });
  return {reached, release: () => release(), read: (q, files) => q !== p || parked ? files.get(q) : (parked = true, reach(), gate.then(() => files.get(q)))};
};
const RAW = new Uint8Array([1, 2, 3]), BIND = 77n;

// Stand-in for installed ledger.Proof (crypto.rs/structure.rs): it parses only tagged 'proof-versioned' with discriminant
// 1|2 and an exactly consumed Vec<u8> body, keeps {d, raw} and re-serializes from that structure, never echoing input.
function codec(mode = {}) {
  const counts = {made: 0, freed: 0};
  class Proof {
    constructor(d, raw) { this.d = d; this.raw = raw; this.instance = 'proof'; counts.made++; }
    static deserialize(raw) {
      const h = VERS.length, [k, i] = Buffer.from(raw.subarray(0, h)).toString('latin1') === VERS && [1, 2].includes(raw[h]) ? unscale(raw, h + 1) : [];
      if (mode.deserialize || i === undefined || i + k !== raw.length) throw Error('deserialize');
      const p = new Proof(raw[h], raw.slice(i));
      if (mode.mutate) raw.fill(0);
      return p;
    }
    serialize() { if (mode.serialize) throw Error('serialize'); return versioned(this.raw, mode.alter ? 3 - this.d : this.d); }
    free() { counts.freed++; if (mode.free) throw Error('free'); }
  }
  return {ledger: {Proof}, counts};
}

// Borrowed ContractCall stand-in. communicationCommitment is to_hex_ser(Fr): '73' is the ScaleBigInt n-byte header for 32
// occupied bytes. Each proof getter read is a fresh owned wrapper over the stored ProofVersioned with discriminant d
// (1 = V2 for verifier-key[v6], 2 = V3 for [v7]); d = 0 models the refused claim that the getter echoes the transport.
function sealed(t, over = {}, counts = {got: 0, freed: 0}, d = 1) {
  return {address: '0xABCD', entryPoint: new TextEncoder().encode('acceptValidity'), communicationCommitment: '73' + COMM, ...over,
    get proof() { counts.got++; const b = d ? stored(t, d) : t; return {instance: 'proof', serialize: () => new Uint8Array(b), free: () => { counts.freed++; }}; }};
}

// The fake base mirrors the makeProvider hand branch: it writes the preimage, awaits route.prove, then rereads proofFile.
function setup(o = {}) {
  const files = new Map(), {ledger, counts} = codec(o.codec), proof = o.proof ?? fresh(), vk = o.vk ?? VK.v6;
  const link = makeCallProviderLink({callKey: KEY, ledger, read: p => o.read ? o.read(p, files) : files.get(p), callContext: CTX, isCallContext: v => BRANDS.has(v), normalizeContractAddress: normAddr, expect: {...EXPECT, verifier_key_sha256: sha(o.pinVk ?? vk)}});
  const binder = o.binder ?? link, routeLink = o.routeLink ?? link;
  const inner = {material: 'm', calls: 0, prove: async args => {
    inner.calls++;
    const comm = o.comm ?? COMM;
    try { binder.bindCallback({public_inputs: [args.bindingLe, comm, H('e')], binding_input: args.bindingLe, communication_commitment: comm, ...EXPECT, submission_intent: {address: '0xabcd'}, ...o.frozen}); }
    catch (e) { if (o.swallow !== 'quiet') throw o.swallow ? Error('bound-D rethrows without cause') : e; }
    files.set('/p/proof.tagged', new Uint8Array(proof));
    return {proofFile: '/p/proof.tagged', stats: {ok: true, verifies_adapter_vector_under_pinned_key: true, ...o.stats}};
  }};
  const handRoutes = {[o.routeKey ?? KEY]: routeLink.route(inner)}, calls = [];
  const base = {events: [], calls,
    check: async (raw, k) => { calls.push(['check', k]); return []; },
    lookupKey: async k => { calls.push(['lookupKey', k]); return k !== KEY ? {k} : 'material' in o ? o.material : {material: 'm', verifierKey: new Uint8Array(vk)}; },
    prove: async (raw, key, binding) => {
      calls.push(['prove', key]);
      const h = handRoutes[key];
      if (!h) return new Uint8Array([4, 4]);
      files.set('/p/preimage.bin', o.preimage ?? new Uint8Array(raw));
      const r = await h.prove({preimage: '/p/preimage.bin', bindingLe: o.bindingLe ?? le(binding), dir: '/p'});
      if (o.replace) files.set(r.proofFile, o.replace);
      return new Uint8Array(files.get(r.proofFile));
    }};
  return {files, counts, link, base, provider: link.compose(base), proof, vk, inner, handRoutes};
}
// prove.rs order: the resolver looks the call key up through the same provider before prove.
const run = async (s, lookups = 1) => { for (let i = 0; i < lookups; i++) await s.provider.lookupKey(KEY); return s.provider.prove(RAW, KEY, BIND); };

test('genuine call proof links provider return, route capture and sealed/finalized call', async () => {
  const s = setup(), counts = {got: 0, freed: 0};
  const out = await run(s);
  assert.deepEqual([...out], [...s.proof]);
  assert.deepEqual(s.counts, {made: 1, freed: 1});
  out.fill(0); s.files.get('/p/proof.tagged').fill(0);
  const r = s.link.observeSealed({sealedCall: sealed(s.proof, {}, counts), finalizedCall: sealed(s.proof, {}, counts)});
  assert.deepEqual(counts, {got: 2, freed: 2});
  assert.equal(r.transport_form, 'bare-proof[v5]'); assert.equal(r.transport_sha256, sha(s.proof));
  assert.equal(r.verifier_key_tag, 'verifier-key[v6]'); assert.equal(r.stored_variant, 'V2'); assert.equal(r.stored_proof_hex, hex(stored(s.proof, 1)));
  assert.equal(r.raw_proof_hex, hex(bodyOf(s.proof).subarray(1))); assert.equal(r.transport_limitation, TRANSPORT_LIMITATION);
  assert.equal(r.binding_le, le(BIND)); assert.equal(r.address, 'abcd'); assert.equal(r.communication_commitment, COMM);
  assert.equal(r.limitation, LIMITATION); assert.ok(Object.isFrozen(r)); assert.equal(s.link.status(), 'linked'); assert.equal(s.link.firstFailure(), null);
  await assert.rejects(run(s), code('ERR_B4_LINK_REUSED'));
  assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof), finalizedCall: sealed(s.proof)}), code('ERR_B4_SEALED_OUT_OF_ORDER'));
  assert.deepEqual([s.link.status(), s.link.firstFailure()], ['failed', 'ERR_B4_LINK_REUSED']);
});

test('DUST proving, check and lookupKey delegate to the existing provider exactly once', async () => {
  const s = setup();
  assert.deepEqual([...await s.provider.prove(RAW, 'dust', 9n)], [4, 4]);
  await s.provider.check(RAW, 'dust'); await s.provider.lookupKey('dust');
  assert.deepEqual(s.base.calls, [['prove', 'dust'], ['check', 'dust'], ['lookupKey', 'dust']]);
  assert.equal(s.provider.events, s.base.events); assert.equal(s.link.status(), 'idle'); assert.equal(s.counts.made, 0);
});

test('proofFile replaced after the route returned is refused', async () => {
  const s = setup({replace: fresh()});
  await assert.rejects(run(s), code('ERR_B4_RETURNED_NOT_ROUTE_PROOF'));
  assert.equal(s.counts.made, 0); assert.equal(s.link.status(), 'failed');
});

test('sealed call with the proof marker but different bytes is refused and its wrapper still freed', async () => {
  const s = setup(), counts = {got: 0, freed: 0};
  await run(s);
  assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof, {}, counts), finalizedCall: sealed(fresh(), {}, counts)}), code('ERR_B4_SEALED_PROOF_BYTES'));
  assert.deepEqual(counts, {got: 2, freed: 2});
});

test('cross-wired hook A under provider B, and call B under link A, are refused', async () => {
  const a = setup(), b = setup({routeLink: a.link, binder: a.link});
  await assert.rejects(run(b), code('ERR_B4_ROUTE_WITHOUT_PROVIDER'));
  const c = setup(), d = setup();
  await run(c); await run(d);
  assert.throws(() => c.link.observeSealed({sealedCall: sealed(d.proof), finalizedCall: sealed(d.proof)}), code('ERR_B4_SEALED_PROOF_BYTES'));
});

test('changed binding, key, preimage, callback vector, communication and context are refused', async () => {
  await assert.rejects(run(setup({bindingLe: le(78n)})), code('ERR_B4_ROUTE_BINDING'));
  await assert.rejects(run(setup({preimage: new Uint8Array([9])})), code('ERR_B4_ROUTE_PREIMAGE'));
  await assert.rejects(run(setup({routeKey: 'other'})), code('ERR_B4_NO_ROUTE_CAPTURE'));
  await assert.rejects(setup({routeKey: 'other'}).provider.prove(RAW, 'other', BIND), code('ERR_B4_ROUTE_WITHOUT_PROVIDER'));
  await assert.rejects(run(setup({binder: {bindCallback() {}}})), code('ERR_B4_CALLBACK_UNBOUND'));
  for (const frozen of [{communication_commitment: H('d')}, {binding_input: le(78n)}, {stmt: H('9')}, {submission_intent: {address: 'beef'}}])
    await assert.rejects(run(setup({frozen})), code('ERR_B4_CALLBACK_CONTEXT'));
  await assert.rejects(run(setup({stats: {verifies_adapter_vector_under_pinned_key: false}})), code('ERR_B4_ROUTE_REFUSED'));
  for (const [over, c] of [[{communicationCommitment: '73' + H('d')}, 'ERR_B4_SEALED_COMMUNICATION'], [{address: 'beef'}, 'ERR_B4_SEALED_ADDRESS'], [{entryPoint: 'other'}, 'ERR_B4_SEALED_ENTRY']]) {
    const s = setup(); await run(s);
    assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof), finalizedCall: sealed(s.proof, over)}), code(c));
  }
});

test('a proof already returned through another link is refused as reused', async () => {
  const s = setup(); await run(s);
  await assert.rejects(run(setup({proof: s.proof})), code('ERR_B4_PROOF_REUSED'));
});

test('installed codec failures on the derived stored form drain the owned wrapper once; borrowed bytes are unchanged', async () => {
  for (const [mode, c, made, proof] of [[{deserialize: true}, 'ERR_B4_CODEC_ACQUIRE', 0], [{serialize: true}, 'ERR_B4_CODEC_SERIALIZE', 1], [{free: true}, 'ERR_B4_CODEC_FREE', 1], [{alter: true}, 'ERR_B4_PROOF_NOT_CANONICAL', 1]]) {
    const s = setup({codec: mode, proof});
    await assert.rejects(run(s), code(c));
    assert.deepEqual(s.counts, {made, freed: made});
  }
  const m = setup({codec: {mutate: true}}), out = await run(m);
  assert.deepEqual([...out], [...m.proof]);
  assert.doesNotThrow(() => m.link.observeSealed({sealedCall: sealed(m.proof), finalizedCall: sealed(m.proof)}));
});

test('callback outside the route, a second compose and an unbranded context are refused', () => {
  const s = setup();
  assert.throws(() => s.link.compose(s.base), code('ERR_B4_COMPOSE'));
  assert.throws(() => s.link.bindCallback({}), code('ERR_B4_CALLBACK_OUTSIDE_ROUTE'));
  assert.throws(() => makeCallProviderLink({callKey: KEY, ledger: codec().ledger, read: () => {}, callContext: {...CTX}, isCallContext: v => BRANDS.has(v), normalizeContractAddress: normAddr, expect: EXPECT}), code('ERR_B4_LINK_ARGS'));
});

test('literal tagged/SCALE vectors; a genuine bare transport under verifier-key[v7] is stored and sealed as V3', async () => {
  const abc = Uint8Array.of(0xaa, 0xbb, 0xcc);
  assert.equal(hex(bare(abc)), hex(enc('midnight:proof[v5]:')) + '0caabbcc');
  assert.equal(hex(versioned(abc, 2)), hex(enc('midnight:proof-versioned:')) + '020caabbcc');
  assert.deepEqual([scale(3), scale(64), scale(16384)], [[0x0c], [1, 1], [2, 0, 1, 0]]);
  const raw = rawOf(64), s = setup({vk: VK.v7, proof: bare(raw)}), counts = {got: 0, freed: 0};
  assert.deepEqual([...await run(s, 2)], [...bare(raw)]);
  assert.deepEqual(s.base.calls, [['lookupKey', KEY], ['lookupKey', KEY], ['prove', KEY]]);
  const r = s.link.observeSealed({sealedCall: sealed(s.proof, {}, counts, 2), finalizedCall: sealed(s.proof, {}, counts, 2)});
  assert.deepEqual(counts, {got: 2, freed: 2});
  assert.deepEqual([r.verifier_key_tag, r.stored_variant, r.stored_proof_hex, r.raw_proof_hex, r.verifier_key_sha256], ['verifier-key[v7]', 'V3', hex(versioned(raw, 2)), hex(raw), sha(VK.v7)]);
});

test('a ProofVersioned::V2 wire is accepted as tx.rs falls back and stored per the resolved key; V0/V3/unknown wires are refused', async () => {
  for (const [vk, d, k] of [[VK.v6, 1, 16384], [VK.v7, 2, 8]]) {
    const raw = rawOf(k), s = setup({vk, proof: versioned(raw, 1)});
    assert.deepEqual([...await run(s)], [...versioned(raw, 1)]);
    const r = s.link.observeSealed({sealedCall: sealed(s.proof, {}, undefined, d), finalizedCall: sealed(s.proof, {}, undefined, d)});
    assert.deepEqual([r.transport_form, r.stored_proof_hex, r.raw_proof_hex], ['proof-versioned-V2', hex(versioned(raw, d)), hex(raw)]);
  }
  for (const d of [0, 2, 3]) {
    const s = setup({proof: versioned(rawOf(), d)});
    await assert.rejects(run(s), code('ERR_B4_TRANSPORT_VARIANT')); assert.equal(s.counts.made, 0);
  }
});

test('wrong tag, non-canonical, out-of-range or out-of-bounds length, truncated and trailing transports are refused before the codec', async () => {
  const raw = rawOf(), B = enc(BARE);
  for (const [proof, c] of [
    [cat(enc('midnight:proof[v4]:'), body(raw)), 'ERR_B4_TRANSPORT_TAG'], [cat(enc('midnight:proof-preimage:'), body(raw)), 'ERR_B4_TRANSPORT_TAG'], [body(raw), 'ERR_B4_TRANSPORT_TAG'],
    [cat(B, [33, 0], raw), 'ERR_B4_TRANSPORT_LENGTH_NONCANONICAL'], [cat(B, [34, 0, 0, 0], raw), 'ERR_B4_TRANSPORT_LENGTH_NONCANONICAL'],
    [cat(B, [3, 8, 0, 0, 1], raw), 'ERR_B4_TRANSPORT_LENGTH_NONCANONICAL'], [cat(B, [7, 0, 0, 0, 0, 1]), 'ERR_B4_TRANSPORT_LENGTH_RANGE'],
    [cat(B, [0]), 'ERR_B4_TRANSPORT_LENGTH_BOUNDS'], [cat(B, scale(MAX_PROOF_BYTES + 1)), 'ERR_B4_TRANSPORT_LENGTH_BOUNDS'],
    [B, 'ERR_B4_TRANSPORT_LENGTH_TRUNCATED'], [cat(B, [1]), 'ERR_B4_TRANSPORT_LENGTH_TRUNCATED'],
    [bare(raw).subarray(0, -1), 'ERR_B4_TRANSPORT_TRUNCATED'], [cat(bare(raw), [0]), 'ERR_B4_TRANSPORT_TRAILING']]) {
    const s = setup({proof});
    await assert.rejects(run(s), code(c)); assert.equal(s.counts.made, 0); assert.equal(s.link.status(), 'failed');
  }
});

test('the verifier key resolved through this provider is authenticated before the call is proved', async () => {
  for (const [o, c] of [[{vk: VK.v5}, 'ERR_B4_VK_TAG'], [{vk: enc('verifier-key[v6]:vk')}, 'ERR_B4_VK_TAG'], [{vk: VK.v7, pinVk: VK.v6}, 'ERR_B4_VK_IDENTITY']]) {
    const s = setup(o);
    await assert.rejects(s.provider.lookupKey(KEY), code(c));
    await assert.rejects(s.provider.prove(RAW, KEY, BIND), code('ERR_B4_LINK_REUSED'));
    assert.deepEqual(s.base.calls, [['lookupKey', KEY]]);
  }
  const s = setup();
  await assert.rejects(s.provider.prove(RAW, KEY, BIND), code('ERR_B4_VK_UNRESOLVED'));
  assert.deepEqual(s.base.calls, []);
});

test('a sealed proof in the other variant, or echoing the transport, is refused', async () => {
  for (const d of [2, 0]) {
    const s = setup(); await run(s);
    assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof, {}, undefined, d), finalizedCall: sealed(s.proof)}), code('ERR_B4_SEALED_PROOF_BYTES'));
  }
});

test('sealed communicationCommitment is decoded as to_hex_ser ScaleBigInt and matched numerically to the 32-byte LE callback value', async () => {
  const small = '05' + '0'.repeat(62), s = setup({comm: small}); await run(s);
  assert.equal(s.link.observeSealed({sealedCall: sealed(s.proof, {communicationCommitment: '14'}), finalizedCall: sealed(s.proof, {communicationCommitment: '14'})}).communication_commitment, small);
  for (const [cc, c] of [['0x73' + COMM, '_FORM'], ['73' + COMM.toUpperCase(), '_FORM'], ['', '_FORM'], [5, '_FORM'], [COMM, '_TRAILING'], ['73' + COMM + '00', '_TRAILING'],
    ['77' + COMM + '00', '_NONCANONICAL'], ['77' + COMM + '01', '_RANGE'], ['73' + COMM.slice(2), '_TRUNCATED'], ['14', '']]) {
    const t = setup(); await run(t);
    assert.throws(() => t.link.observeSealed({sealedCall: sealed(t.proof), finalizedCall: sealed(t.proof, {communicationCommitment: cc})}), code('ERR_B4_SEALED_COMMUNICATION' + c));
  }
});

test('callback pis[0] must be the proved binding and communication the strict 32-byte LE hex; the first refusal code survives a causeless rethrow', async () => {
  for (const frozen of [{public_inputs: [le(78n), COMM, H('e')]}, {communication_commitment: '0x' + COMM}, {public_inputs: [le(BIND), COMM.toUpperCase(), H('e')], communication_commitment: COMM.toUpperCase()}])
    await assert.rejects(run(setup({frozen})), code('ERR_B4_CALLBACK_CONTEXT'));
  const s = setup({frozen: {stmt: H('9')}, swallow: true});
  await assert.rejects(run(s), code('ERR_B4_UNEXPECTED'));
  assert.deepEqual([s.link.firstFailure(), s.link.status()], ['ERR_B4_CALLBACK_CONTEXT', 'failed']);
});

test('the same raw proof is refused as reused across bare and V2-fallback transports', async () => {
  const raw = rawOf(); await run(setup({proof: bare(raw)}));
  await assert.rejects(run(setup({proof: versioned(raw, 1)})), code('ERR_B4_PROOF_REUSED'));
});

test('borrowed call.proof getter acquire/serialize/free errors keep the first code and free each owned wrapper once', async () => {
  for (const [make, c, freed] of [[() => { throw Error('getter'); }, 'ERR_B4_SEALED_PROOF_ACQUIRE', 0],
    [k => ({instance: 'pre-proof', serialize: () => new Uint8Array(1), free: k}), 'ERR_B4_SEALED_PROOF_SERIALIZE', 1],
    [k => ({instance: 'proof', serialize: () => { throw Error('serialize'); }, free: k}), 'ERR_B4_SEALED_PROOF_SERIALIZE', 1],
    [k => ({instance: 'proof', serialize: () => new Uint8Array(1), free: () => { k(); throw Error('free'); }}), 'ERR_B4_SEALED_PROOF_FREE', 1]]) {
    const s = setup(); await run(s); let f = 0;
    const call = {...sealed(s.proof), get proof() { return make(() => f++); }};
    assert.throws(() => s.link.observeSealed({sealedCall: call, finalizedCall: sealed(s.proof)}), code(c));
    assert.equal(f, freed); assert.equal(s.link.firstFailure(), c);
  }
});

test('call-key lookupKey delegates once and returns the existing material; a link without the verifier-key pin is refused', async () => {
  const s = setup(), m = await s.provider.lookupKey(KEY);
  assert.deepEqual([m.material, [...m.verifierKey]], ['m', [...VK.v6]]);
  assert.deepEqual(s.base.calls, [['lookupKey', KEY]]); assert.equal(s.link.status(), 'idle');
  const {verifier_key_sha256, ...pins} = EXPECT;
  assert.throws(() => makeCallProviderLink({callKey: KEY, ledger: codec().ledger, read: () => {}, callContext: CTX, isCallContext: v => BRANDS.has(v), normalizeContractAddress: normAddr, expect: pins}), code('ERR_B4_LINK_ARGS'));
});

test('a refusal injected while the route awaits its proofFile read is absorbing: no returned or linked receipt and the first code survives', async () => {
  const g = parkRead('/p/proof.tagged'), ok = setup({read: g.read});
  await ok.provider.lookupKey(KEY); const q = ok.provider.prove(RAW, KEY, BIND);
  await g.reached; g.release();
  assert.deepEqual([...await q], [...ok.proof]);
  ok.link.observeSealed({sealedCall: sealed(ok.proof), finalizedCall: sealed(ok.proof)});
  assert.deepEqual([ok.link.status(), ok.link.firstFailure()], ['linked', null]);
  for (const [c, inject] of [
    ['ERR_B4_SEALED_OUT_OF_ORDER', s => assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof), finalizedCall: sealed(s.proof)}), code('ERR_B4_SEALED_OUT_OF_ORDER'))],
    ['ERR_B4_CALLBACK_OUTSIDE_ROUTE', s => assert.throws(() => s.link.bindCallback({}), code('ERR_B4_CALLBACK_OUTSIDE_ROUTE'))],
    ['ERR_B4_LINK_REUSED', s => assert.rejects(s.provider.prove(RAW, KEY, BIND), code('ERR_B4_LINK_REUSED'))]]) {
    const gate = parkRead('/p/proof.tagged'), s = setup({read: gate.read}), counts = {got: 0, freed: 0};
    await s.provider.lookupKey(KEY);
    const p = s.provider.prove(RAW, KEY, BIND);
    await gate.reached; await inject(s); gate.release();
    await assert.rejects(p, code('ERR_B4_STATE_CHANGED'));
    assert.deepEqual([s.inner.calls, s.link.status(), s.link.firstFailure(), s.counts], [1, 'failed', c, {made: 0, freed: 0}]);
    assert.throws(() => s.link.observeSealed({sealedCall: sealed(s.proof, {}, counts), finalizedCall: sealed(s.proof, {}, counts)}), code('ERR_B4_SEALED_OUT_OF_ORDER'));
    assert.deepEqual([counts, s.link.firstFailure()], [{got: 0, freed: 0}, c]);
  }
});

test('a second route call while the first awaits its preimage read is refused and inner is never proved twice', async () => {
  const gate = parkRead('/p/preimage.bin'), s = setup({read: gate.read});
  await s.provider.lookupKey(KEY);
  const p = s.provider.prove(RAW, KEY, BIND);
  await gate.reached;
  await assert.rejects(s.handRoutes[KEY].prove({preimage: '/p/preimage.bin', bindingLe: le(BIND), dir: '/p'}), code('ERR_B4_ROUTE_WITHOUT_PROVIDER'));
  gate.release();
  await assert.rejects(p, code('ERR_B4_STATE_CHANGED'));
  assert.deepEqual([s.inner.calls, s.link.status(), s.link.firstFailure(), s.counts.made], [0, 'failed', 'ERR_B4_ROUTE_WITHOUT_PROVIDER', 0]);
  assert.deepEqual(s.base.calls, [['lookupKey', KEY], ['prove', KEY]]);
});

test('an inner that swallows a bindCallback refusal and returns success still fails closed with the first CALLBACK_CONTEXT', async () => {
  const s = setup({frozen: {stmt: H('9')}, swallow: 'quiet'});
  await assert.rejects(run(s), code('ERR_B4_CALLBACK_UNBOUND'));
  assert.deepEqual([s.inner.calls, s.link.status(), s.link.firstFailure(), s.counts.made], [1, 'failed', 'ERR_B4_CALLBACK_CONTEXT', 0]);
});

test('missing or identical sealed calls, bad call arguments and call-key material without verifierKey are refused with exact codes', async () => {
  for (const args of [() => undefined, () => ({}), c => ({sealedCall: c}), c => ({finalizedCall: c}), c => ({sealedCall: c, finalizedCall: c})]) {
    const s = setup(), counts = {got: 0, freed: 0}; await run(s);
    assert.throws(() => s.link.observeSealed(args(sealed(s.proof, {}, counts))), code('ERR_B4_SEALED_ARGS'));
    assert.deepEqual([counts, s.link.status(), s.link.firstFailure()], [{got: 0, freed: 0}, 'failed', 'ERR_B4_SEALED_ARGS']);
  }
  for (const [raw, binding] of [[[1, 2, 3], BIND], [RAW, 77], [RAW, 0n], [RAW, -1n]]) {
    const s = setup(); await s.provider.lookupKey(KEY);
    await assert.rejects(s.provider.prove(raw, KEY, binding), code('ERR_B4_CALL_ARGS'));
    assert.deepEqual([s.base.calls, s.link.status(), s.link.firstFailure()], [[['lookupKey', KEY]], 'failed', 'ERR_B4_CALL_ARGS']);
  }
  for (const material of [{material: 'm'}, {material: 'm', verifierKey: [...VK.v6]}, null]) {
    const s = setup({material});
    await assert.rejects(s.provider.lookupKey(KEY), code('ERR_B4_NOT_BYTES'));
    await assert.rejects(s.provider.prove(RAW, KEY, BIND), code('ERR_B4_LINK_REUSED'));
    assert.deepEqual([s.base.calls, s.link.firstFailure()], [[['lookupKey', KEY]], 'ERR_B4_NOT_BYTES']);
  }
});
