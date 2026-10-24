// B4 call-only provider link. It wraps the existing makeProvider result. Only the call key is
// intercepted, so DUST proving, the queue, check and lookupKey all stay with the existing provider; the call key's
// lookupKey is still delegated once and only observed for its verifier key.
// rc3 tx.rs:496-509 consumes the Uint8Array this prove resolves: first as a bare tagged transient Proof
// (proof[v5], what handkey wrap writes), else only as tagged ProofVersioned::V2. prove.rs:357-376 then stores
// ProofVersioned V2 or V3 by the peek_tag of the verifier key resolved through this provider's lookupKey.
// This module returns the transport unchanged and ties its proof body to that stored form and the sealed call.
import {createHash} from 'node:crypto';

export const LIMITATION = 'ADAPTER_VECTOR_ONLY: installed ContractCall exports no public_inputs/binding_input and the ledger vector depends on the parent binding_commitment; only real validateTransaction and on-chain validation check it';
export const TRANSPORT_LIMITATION = 'installed ledger.Proof is the tagged ProofVersioned codec (proof-versioned, u8 variant, Proof body); the handkey transport is bare tagged proof[v5], which tx.rs accepts first and no public JS codec decodes. B4 parses it from pinned 9a8777c serialize/proofs.rs source, derives the stored V2/V3 form from the resolved verifier-key tag and round-trips that through installed ledger.Proof. Installed v9 binary correspondence to the rc3/9a8777c sources is unproven until a separately reviewed installed-SDK codec smoke runs';
export class LinkError extends Error {
  constructor(code, cause, cleanup = null) { super(code, cause === undefined ? undefined : {cause}); this.name = 'LinkError'; this.code = code; this.cleanup = cleanup; }
}
const fail = (code, cause) => { throw new LinkError(code, cause); };
const wrap = e => e instanceof LinkError ? e : new LinkError('ERR_B4_UNEXPECTED', e);
const sha = b => createHash('sha256').update(b).digest('hex');
const copy = b => b instanceof Uint8Array ? new Uint8Array(b) : fail('ERR_B4_NOT_BYTES');
const same = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);
const deep = o => { if (o && typeof o === 'object') { Object.values(o).forEach(deep); Object.freeze(o); } return o; };
const HEX64 = /^[0-9a-f]{64}$/;
const hex = b => Buffer.from(b).toString('hex');
const starts = (b, h) => b.length >= h.length && h.every((v, i) => b[i] === v);
const cat = (...p) => { const o = new Uint8Array(p.reduce((k, x) => k + x.length, 0)); p.reduce((i, x) => (o.set(x, i), i + x.length), 0); return o; };
const PINS = ['stmt', 'instance_id', 'batch_no', 'verifier_sha256'], KEYS = [...PINS, 'verifier_key_sha256'];
// serializable.rs:26-33 tagged_serialize writes 'midnight:' + T::tag() + ':' then the payload. proofs.rs:135 tags the
// bare transient Proof 'proof[v5]'; structure.rs:292-310 tags ProofVersioned 'proof-versioned' + u8 1 (V2) | 2 (V3).
const enc = s => new TextEncoder().encode(s), GLOBAL = enc('midnight:'), BARE = enc('midnight:proof[v5]:'), VERSIONED = enc('midnight:proof-versioned:');
// prove.rs:372-375: resolved verifier-key tag -> stored variant and its discriminant.
const VARIANT = new Map([['verifier-key[v6]', ['V2', 1]], ['verifier-key[v7]', ['V3', 2]]]);
export const MAX_PROOF_BYTES = 1 << 20;

// util.rs ScaleBigInt::serialize of a little-endian magnitude.
function scaleEncode(v) {
  let n = v.length; while (n && !v[n - 1]) n--;
  const lo = i => (v[i] & 63) << 2, hi = i => v[i] >> 6, sq = v[Math.max(n - 1, 0)] < 64;
  if (!n || (n === 1 && sq)) return [lo(0)];
  if (n === 1 || (n === 2 && sq)) return [lo(0) | 1, hi(0) | lo(1)];
  if (n <= 3 || (n === 4 && sq)) return [lo(0) | 2, hi(0) | lo(1), hi(1) | lo(2), hi(2) | lo(3)];
  return [(n - 4) << 2 | 3, ...v.subarray(0, n)];
}

// Reads one ScaleBigInt at b[i] in the util.rs layout and refuses anything that does not re-encode to the same bytes,
// so every non-canonical form is refused (util.rs alone accepts an n-byte form of a value that fits 4 bytes).
function scaleRead(b, i, maxBytes, code) {
  const f = b[i], v = new Uint8Array(67);
  if (f === undefined) fail(code + '_TRUNCATED');
  const end = i + ((f & 3) === 3 ? (f >> 2) + 5 : [1, 2, 4][f & 3]);
  if (end > b.length) fail(code + '_TRUNCATED');
  if ((f & 3) === 3) v.set(b.subarray(i + 1, end));
  else for (let k = 0; k < end - i; k++) v[k] = b[i + k] >> 2 | (k + 1 < end - i ? b[i + k + 1] & 3 : 0) << 6;
  if (!same(scaleEncode(v), b.subarray(i, end))) fail(code + '_NONCANONICAL');
  if (v.subarray(maxBytes).some(x => x)) fail(code + '_RANGE');
  return {v: v.slice(0, maxBytes), end};
}

// tx.rs:501-509: bare proof[v5], else proof-versioned with discriminant 1 (V2) only. Both carry the same Proof body,
// a Vec<u8> with a u32 SCALE length (serializable.rs:47-52, util.rs:211), which must be consumed exactly.
function parseTransport(t) {
  let form, i;
  if (starts(t, BARE)) [form, i] = ['bare-proof[v5]', BARE.length];
  else if (starts(t, VERSIONED)) {
    if (t[VERSIONED.length] !== 1) fail('ERR_B4_TRANSPORT_VARIANT');
    [form, i] = ['proof-versioned-V2', VERSIONED.length + 1];
  } else fail('ERR_B4_TRANSPORT_TAG');
  const {v, end} = scaleRead(t, i, 4, 'ERR_B4_TRANSPORT_LENGTH'), len = v.reduceRight((a, x) => a * 256 + x, 0);
  if (!len || len > MAX_PROOF_BYTES) fail('ERR_B4_TRANSPORT_LENGTH_BOUNDS');
  if (end + len !== t.length) fail(end + len > t.length ? 'ERR_B4_TRANSPORT_TRUNCATED' : 'ERR_B4_TRANSPORT_TRAILING');
  return {form, body: t.slice(i), raw: t.slice(end)};
}

// deserializable.rs peek_tag: the bytes after 'midnight:' up to the next ':' within the first 512.
function peekTag(b) {
  const head = b.subarray(0, 512), end = head.indexOf(58, GLOBAL.length);
  if (!starts(head, GLOBAL) || end < 0) fail('ERR_B4_VK_TAG');
  return new TextDecoder().decode(head.subarray(GLOBAL.length, end));
}

// contract.rs:212 getter is to_hex_ser(Fr): lowercase hex of ScaleBigInt(Fr repr) (curve.rs:105-150). Returns the
// 32-byte LE hex that handkey fr_hex writes for the same field element, or refuses.
function commitmentLe(s) {
  if (typeof s !== 'string' || !/^(?:[0-9a-f]{2})+$/.test(s)) fail('ERR_B4_SEALED_COMMUNICATION_FORM');
  const b = Buffer.from(s, 'hex'), {v, end} = scaleRead(b, 0, 32, 'ERR_B4_SEALED_COMMUNICATION');
  if (end !== b.length) fail('ERR_B4_SEALED_COMMUNICATION_TRAILING');
  return hex(v);
}
// Same conversion as the makeProvider hand branch (lib.mjs:146).
export const bindingLe = n => Buffer.from(n.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const seen = new Set();

// Owned ledger.Proof wrapper: acquired once, serialized once, freed exactly once (a non-object acquisition holds nothing
// to free). A free failure is kept as its own LinkError(code_FREE, free error) in .cleanup, either beside a primary SERIALIZE
// that keeps its own cause or under a _FREE primary, so neither cause is dropped (audit R3-06).
function ownedProofBytes(acquire, code) {
  let p, out, err, gone;
  try { p = acquire(); } catch (e) { fail(code + '_ACQUIRE', e); }
  try { if (p?.instance !== 'proof') throw Error('not a proof'); out = copy(p.serialize()); } catch (e) { err = [code + '_SERIALIZE', e]; }
  try { if (Object(p) === p) p.free(); } catch (e) { gone = [code + '_FREE', e]; }
  if (err || gone) throw new LinkError(...(err ?? gone), gone ? new LinkError(...gone) : null);
  return out;
}

export function makeCallProviderLink({callKey, ledger, read, callContext, isCallContext, normalizeContractAddress, expect} = {}) {
  if (typeof callKey !== 'string' || !callKey || typeof ledger?.Proof?.deserialize !== 'function' || typeof read !== 'function' || typeof isCallContext !== 'function' || typeof normalizeContractAddress !== 'function' || !isCallContext(callContext) || !KEYS.every(k => HEX64.test(expect?.[k]))) fail('ERR_B4_LINK_ARGS');
  const want = Object.freeze(Object.fromEntries(KEYS.map(k => [k, expect[k]])));
  let state = 'idle', base = null, pending = null, active = null, captured = null, callback = null, callbackSha = null, returned = null, resolved = null, first = null, claimed = false, cleanup = null;
  // An owned-wrapper free failure is latched as its fixed code, apart from the first code (audit R3-06). ownedProofBytes only
  // throws B4-made LinkErrors, so .cleanup is read from B4's own error and never from a foreign one.
  const owned = (acquire, c) => { try { return ownedProofBytes(acquire, c); } catch (e) { cleanup = cleanup ?? e.cleanup?.code ?? null; throw e; } };
  // The first refusal code is kept even when a caller rethrows it without cause (audit S1).
  const failed = e => { const w = wrap(e); state = 'failed'; first = first ?? w.code; return w; };
  const guard = fn => async (...a) => { try { return await fn(...a); } catch (e) { throw failed(e); } };
  const sync = fn => (...a) => { try { return fn(...a); } catch (e) { throw failed(e); } };
  // 'failed' is absorbing (audit SC-1): each state write after an await or a foreign call first rechecks the state it leaves.
  const at = s => { if (state !== s || first !== null) fail('ERR_B4_STATE_CHANGED'); };

  // C installs this as handRoutes[callKey] = link.route(boundDWrapped) before makeProvider.
  const route = inner => {
    if (typeof inner?.prove !== 'function') fail('ERR_B4_ROUTE_ARGS');
    return {...inner, prove: guard(async args => {
      // Claimed before the first await, so a concurrent call is refused without reaching inner (audit SC-1).
      if (state !== 'proving' || claimed) fail('ERR_B4_ROUTE_WITHOUT_PROVIDER');
      claimed = true;
      if (args?.bindingLe !== pending.binding_le) fail('ERR_B4_ROUTE_BINDING');
      if (sha(copy(await read(args.preimage))) !== pending.preimage_sha256) fail('ERR_B4_ROUTE_PREIMAGE');
      at('proving'); active = Object.freeze({bindingLe: args.bindingLe});
      let result;
      try { result = await inner.prove(args); } finally { active = null; }
      if (state !== 'proving' || !callback) fail('ERR_B4_CALLBACK_UNBOUND');
      if (result?.stats?.ok !== true || result.stats.verifies_adapter_vector_under_pinned_key !== true) fail('ERR_B4_ROUTE_REFUSED');
      const proof = copy(await read(result.proofFile));
      at('proving'); captured = proof; state = 'routed';
      return result;
    })};
  };

  // C calls this inside the bound-D callback, after frozen is built and before route.prove.
  const bindCallback = sync(frozen => {
    if (!active || callback || state !== 'proving') fail('ERR_B4_CALLBACK_OUTSIDE_ROUTE');
    if (!frozen || typeof frozen !== 'object') fail('ERR_B4_CALLBACK_CONTEXT');
    const f = JSON.parse(JSON.stringify(frozen)), pis = f.public_inputs;
    // pis[0] is the proved binding (audit S2); pis[1] is the handkey fr_hex form, 32-byte LE lowercase hex.
    if (!Array.isArray(pis) || pis[0] !== active.bindingLe || !HEX64.test(pis[1]) || f.communication_commitment !== pis[1] || f.binding_input !== active.bindingLe || PINS.some(k => f[k] !== want[k]) || normalizeContractAddress(f.submission_intent?.address) !== callContext.address) fail('ERR_B4_CALLBACK_CONTEXT');
    callbackSha = sha(JSON.stringify(f)); callback = deep(f);
  });

  const prove = guard(async (raw, key, binding) => {
    if (state !== 'idle') fail('ERR_B4_LINK_REUSED');
    if (!(raw instanceof Uint8Array) || typeof binding !== 'bigint' || binding <= 0n) fail('ERR_B4_CALL_ARGS');
    if (!resolved) fail('ERR_B4_VK_UNRESOLVED');
    state = 'proving';
    pending = Object.freeze({preimage_sha256: sha(raw), binding_le: bindingLe(binding)});
    const got = copy(await base.prove(raw, key, binding));
    if (state !== 'routed') fail('ERR_B4_NO_ROUTE_CAPTURE');
    if (!same(got, captured)) fail('ERR_B4_RETURNED_NOT_ROUTE_PROOF');
    // The ledger stores ProofVersioned(variant of the resolved key) over this same body; installed ledger.Proof must round-trip it.
    const t = parseTransport(got), [variant, d] = VARIANT.get(resolved.tag), stored = cat(VERSIONED, [d], t.body);
    if (!same(owned(() => ledger.Proof.deserialize(copy(stored)), 'ERR_B4_CODEC'), stored)) fail('ERR_B4_PROOF_NOT_CANONICAL');
    const id = sha(t.raw);
    if (seen.has(id)) fail('ERR_B4_PROOF_REUSED');
    seen.add(id); returned = Object.freeze({...t, variant, tag: resolved.tag, stored, transport: got, raw_sha256: id}); at('routed'); state = 'returned';
    return new Uint8Array(got);
  });

  // prove.rs resolves preimage.key_location through this provider's lookupKey (tx.rs JsProvingProvider resolver) and
  // peeks the verifier key's tag before proving. The material is delegated once and returned as is.
  const lookup = guard(async key => {
    const m = await base.lookupKey(key), vk = copy(m?.verifierKey), tag = peekTag(vk);
    if (!VARIANT.has(tag)) fail('ERR_B4_VK_TAG');
    if (sha(vk) !== want.verifier_key_sha256) fail('ERR_B4_VK_IDENTITY');
    resolved = Object.freeze({tag, sha256: sha(vk)});
    return m;
  });

  // The returned object is the provider C passes to A proveWithBoundModel for the call-only transaction.prove.
  const compose = b => {
    if (base || !b || ['prove', 'check', 'lookupKey'].some(k => typeof b[k] !== 'function')) fail('ERR_B4_COMPOSE');
    base = b;
    return Object.freeze({
      events: b.events,
      check: (raw, key) => b.check(raw, key),
      lookupKey: key => key === callKey ? lookup(key) : b.lookupKey(key),
      prove: (raw, key, binding) => key === callKey ? prove(raw, key, binding) : b.prove(raw, key, binding),
    });
  };

  // C calls this in verifyAfterMerge before assertSealedCallPreserved, B3 native verify, publication or send.
  // The ContractCall objects are borrowed. Each call.proof getter result is owned and freed once.
  const observeSealed = sync(({sealedCall, finalizedCall} = {}) => {
    if (state !== 'returned' || first !== null) fail('ERR_B4_SEALED_OUT_OF_ORDER');
    state = 'observing';
    if (!sealedCall || !finalizedCall || sealedCall === finalizedCall) fail('ERR_B4_SEALED_ARGS');
    for (const call of [sealedCall, finalizedCall]) {
      if (normalizeContractAddress(call.address) !== callContext.address) fail('ERR_B4_SEALED_ADDRESS');
      const e = call.entryPoint;
      if ((e instanceof Uint8Array ? new TextDecoder('utf-8', {fatal: true}).decode(e) : e) !== callContext.entry_point) fail('ERR_B4_SEALED_ENTRY');
      if (commitmentLe(call.communicationCommitment) !== callback.communication_commitment) fail('ERR_B4_SEALED_COMMUNICATION');
      if (!same(owned(() => call.proof, 'ERR_B4_SEALED_PROOF'), returned.stored)) fail('ERR_B4_SEALED_PROOF_BYTES');
    }
    at('observing'); state = 'linked';
    return deep({schema: 'b4-call-provider-link/2', key_location: callKey, preimage_sha256: pending.preimage_sha256, binding_le: pending.binding_le,
      callback_sha256: callbackSha, communication_commitment: callback.communication_commitment, ...want,
      address: callContext.address, entry_point: callContext.entry_point, deployment_record_sha256: callContext.deployment_record_sha256,
      verifier_key_tag: returned.tag, stored_variant: returned.variant, transport_form: returned.form, transport_sha256: sha(returned.transport),
      stored_proof_sha256: sha(returned.stored), stored_proof_hex: hex(returned.stored), raw_proof_sha256: returned.raw_sha256, raw_proof_hex: hex(returned.raw),
      linked: 'PROVIDER_RETURN_EQ_ROUTE_CAPTURE; TRANSPORT_BODY_EQ_STORED_BODY; STORED_VARIANT_FROM_RESOLVED_VERIFIER_KEY_TAG; INSTALLED_CODEC_ROUNDTRIPS_STORED; SEALED_AND_FINALIZED_GETTERS_EQ_STORED',
      limitation: LIMITATION, transport_limitation: TRANSPORT_LIMITATION,
      requires: ['B3 native verify of raw_proof_hex (raw form, distinct from transport and stored forms) with the C-held native vector', 'real ledger validateTransaction', 'on-chain validation']});
  });

  return Object.freeze({route, bindCallback, compose, observeSealed, status: () => state, firstFailure: () => first, cleanupFailure: () => cleanup});
}
