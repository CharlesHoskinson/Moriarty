// Source-only candidate. Root must select execution after fresh independent review.
// All data is synthetic/public. Proof envelopes below are NOT cryptographic proofs.
import {
  ContractOperation, ContractCallPrototype, Intent, Transaction, CostModel, Proof,
} from '/home/charl/projectfenrir/experiments/midnight-intents-2026-10-12/validation/testnet/node_modules/@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js';

class SmokeFailure extends Error {
  constructor(code) { super(code); this.code = code; }
}
class CallbackStop extends Error {
  constructor() { super('OWNED_CALLBACK_STOP'); }
}
const owners = [];
const handles = new Set();
let firstFailure;
let failureStage;
let stage = 'SETUP';
let inFlight = false;
let freed = 0;
let callbackStop;
const calls = [];
const KEY = 'synthetic/callback';
const nonclaims = {
  cryptographicValidity: false, deployedKey: false, fees: false, dust: false,
  nodeFeatures: false, memoryFit: false, financialAcceptance: false,
  targetAcceptance: false, fullSherlockAcceptance: false,
};
function fail(code) {
  firstFailure ??= new SmokeFailure(code);
  failureStage ??= stage;
  throw firstFailure;
}
function requireThat(ok, code) { if (!ok) fail(code); }
function own(handle) {
  requireThat(handle && typeof handle.free === 'function', 'OWN_HANDLE_SHAPE');
  requireThat(!handles.has(handle), 'OWN_DUPLICATE_ALIAS');
  handles.add(handle);
  owners.push({ handle, attempted: false });
  return handle;
}
function sameBytes(a, b) {
  return a instanceof Uint8Array && b instanceof Uint8Array
    && a.length === b.length && a.every((byte, i) => byte === b[i]);
}
function rejected(thunk, code) {
  let refused = false;
  try { own(thunk()); } catch (error) {
    if (error instanceof SmokeFailure) throw error;
    refused = true;
  }
  requireThat(refused, code);
}
function snapshot(tx) {
  const map = tx.intents;
  if (map === undefined) return []; // Installed getter uses undefined for zero intents.
  requireThat(map instanceof Map && map.size > 0, 'SEGMENT_MAP_SHAPE');
  // Each getter-produced Intent is a fresh owned wrapper. Register ALL before use.
  for (const intent of map.values()) own(intent);
  return [...map].sort(([a], [b]) => a - b)
    .map(([segment, intent]) => [segment, intent.serialize()]);
}
function segmentsEqual(snapshotValue, expected) {
  return snapshotValue.map(([key]) => key).join(',') === expected;
}
const hex = (s) => Uint8Array.from(s.match(/../g) ?? [], (pair) => Number.parseInt(pair, 16));
const tagged = (tag, body) => Uint8Array.from([
  ...new TextEncoder().encode(`midnight:${tag}:`), ...hex(body),
]);

let observations;
try {
  stage = 'CONSTRUCT';
  const op = own(new ContractOperation()); // Never set verifierKey.
  const proto = own(new ContractCallPrototype(
    '00'.repeat(32), 'entrysmoke', op, undefined, undefined, [],
    { value: [], alignment: [] }, { value: [], alignment: [] }, '00', KEY,
  ));
  const base = own(Intent.new(new Date('2030-01-01T00:00:00.000Z')));
  const withCall = own(base.addCall(proto));
  const tx = own(Transaction.fromParts('stagenet', undefined, undefined, withCall));
  const model = own(CostModel.initialCostModel());
  let checkedRaw;
  const provider = {
    async check(...args) {
      requireThat(inFlight && this === provider && calls.length === 0, 'CHECK_ORDER');
      requireThat(args.length === 2 && args[0] instanceof Uint8Array
        && args[0].length > 0 && args[1] === KEY, 'CHECK_ABI');
      checkedRaw = args[0].slice(); // Synthetic only; never printed or persisted.
      calls.push({ callback: 'check', arity: 2, bytes: true, key: true });
      return [];
    },
    async lookupKey(...args) {
      requireThat(inFlight && this === provider && calls.length === 1, 'LOOKUP_ORDER');
      requireThat(args.length === 1 && args[0] === KEY, 'LOOKUP_ABI');
      calls.push({ callback: 'lookupKey', arity: 1, key: true });
      return { proverKey: new Uint8Array(), verifierKey: new Uint8Array(), ir: new Uint8Array() };
    },
    async prove(...args) {
      requireThat(inFlight && this === provider && calls.length === 2, 'PROVE_ORDER');
      requireThat(args.length === 3 && args[0] instanceof Uint8Array
        && sameBytes(args[0], checkedRaw) && args[1] === KEY
        && typeof args[2] === 'bigint' && args[2] > 0n
        && args[2] < (1n << 248n), 'PROVE_ABI');
      calls.push({ callback: 'prove', arity: 3, bytes: true, key: true,
        bindingBigint: true, bindingNonzero: true, bindingBelow2Pow248: true });
      callbackStop = new CallbackStop(); // Latch before vendor wrapping.
      throw callbackStop; // No proof bytes returned; tag parsing is unreachable.
    },
  };
  let proveRejected = false;
  stage = 'CALLBACK';
  inFlight = true;
  try { own(await tx.prove(provider, model)); } catch (error) {
    if (error instanceof SmokeFailure) throw error;
    proveRejected = true; // Never classify vendor error text.
  } finally { inFlight = false; checkedRaw?.fill(0); checkedRaw = undefined; }
  requireThat(firstFailure === undefined, 'CALLBACK_FIRST_FAILURE');
  requireThat(proveRejected && callbackStop instanceof CallbackStop
    && calls.map(({ callback }) => callback).join(',') === 'check,lookupKey,prove', 'SENTINEL_NOT_OBSERVED');

  stage = 'BIND_MERGE';
  const before = tx.serialize();
  const boundA = own(tx.bind());
  const boundB = own(tx.bind());
  const boundClone = own(boundA.bind());
  requireThat(!sameBytes(boundA.serialize(), boundB.serialize()), 'SEALS_NOT_DISTINCT');
  requireThat(sameBytes(boundA.serialize(), boundClone.serialize()), 'BOUND_CLONE_BYTES');
  requireThat(sameBytes(before, tx.serialize()), 'BIND_MUTATED_INPUT');
  requireThat(segmentsEqual(snapshot(boundA), '1')
    && segmentsEqual(snapshot(boundClone), '1'), 'BOUND_SEGMENT_CHANGED');

  const empty = own(Transaction.fromParts('stagenet'));
  requireThat(empty.intents === undefined, 'ZERO_INTENTS_GETTER_SHAPE');
  const second = own(empty.addIntent({ tag: 'specific', value: 2 }, base));
  const merged = own(tx.merge(second));
  const mergedSnapshot = snapshot(merged);
  requireThat(segmentsEqual(mergedSnapshot, '1,2'), 'MERGED_SEGMENTS');
  requireThat(sameBytes(mergedSnapshot[0][1], withCall.serialize())
    && sameBytes(mergedSnapshot[1][1], base.serialize()), 'MERGED_INTENT_BYTES');
  rejected(() => merged.merge(second), 'COLLISION_ACCEPTED');
  rejected(() => second.merge(boundA), 'MIXED_VARIANT_ACCEPTED');
  rejected(() => empty.addIntent({ tag: 'specific', value: 0 }, base), 'ZERO_SEGMENT_ACCEPTED');
  const once = own(empty.addIntent({ tag: 'specific', value: 1 }, base));
  const replaced = own(once.addIntent({ tag: 'specific', value: 1 }, withCall));
  const replacedSnapshot = snapshot(replaced);
  const onceSnapshot = snapshot(once);
  requireThat(segmentsEqual(replacedSnapshot, '1')
    && sameBytes(replacedSnapshot[0][1], withCall.serialize()), 'REPLACE_INTENT_BYTES');
  requireThat(segmentsEqual(onceSnapshot, '1')
    && sameBytes(onceSnapshot[0][1], base.serialize()), 'ADD_INTENT_MUTATED_INPUT');
  const removed = own(replaced.addIntent({ tag: 'specific', value: 1 }, undefined));
  requireThat(snapshot(removed).length === 0, 'REMOVE_INTENT_FAILED');

  stage = 'CODEC';
  // Untagged stored ProofVersioned: discriminant + SCALE Vec length + raw body.
  const v2 = own(new Proof('010caabbcc'));
  const v3 = own(new Proof('020caabbcc'));
  const storedV2 = tagged('proof-versioned', '010caabbcc');
  const storedV3 = tagged('proof-versioned', '020caabbcc');
  requireThat(v2.instance === 'proof' && v3.instance === 'proof', 'PROOF_INSTANCE');
  requireThat(sameBytes(v2.serialize(), storedV2)
    && sameBytes(v3.serialize(), storedV3), 'STORED_CODEC_BYTES');
  const reopenedV2 = own(Proof.deserialize(storedV2));
  const reopenedV3 = own(Proof.deserialize(storedV3));
  requireThat(sameBytes(reopenedV2.serialize(), storedV2)
    && sameBytes(reopenedV3.serialize(), storedV3), 'STORED_CODEC_ROUNDTRIP');
  // These negative cases distinguish the public stored wrapper from base transport.
  rejected(() => Proof.deserialize(tagged('proof[v5]', '0caabbcc')), 'BARE_TRANSPORT_ACCEPTED_AS_STORED');
  rejected(() => new Proof('000caabbcc'), 'OLD_DISCRIMINANT_ACCEPTED');
  rejected(() => new Proof('030caabbcc'), 'UNKNOWN_DISCRIMINANT_ACCEPTED');
  rejected(() => new Proof('010caabb'), 'TRUNCATED_BODY_ACCEPTED');
  rejected(() => new Proof('010caabbcc00'), 'TRAILING_BODY_ACCEPTED');
  rejected(() => Proof.deserialize(tagged('proof-versioned', '020caabb')), 'TRUNCATED_ENVELOPE_ACCEPTED');
  rejected(() => Proof.deserialize(tagged('proof-versioned', '020caabbcc00')), 'TRAILING_ENVELOPE_ACCEPTED');
  requireThat(handles.size === owners.length, 'OWNER_IDENTITY_COUNT');
  observations = { schema: 'public-installed-callback-codec-smoke/1', status: 'PASS',
    callbacks: calls, bind: 'TWO_DISTINCT_SEALS_AND_BOUND_CLONE',
    merge: 'DISJOINT_PRESERVED_COLLISION_AND_VARIANT_REFUSED',
    segment: 'INSERT_REPLACE_REMOVE_ZERO_REFUSAL',
    codec: { storedVersions: [2, 3], roundtrips: 2, refusedVectors: 7,
      bareTransport: 'REFUSED_BY_STORED_WRAPPER' },
    blocked: ['O3_IMPORTED_HELPER_CLOSURE_UNINSPECTED', 'NOOP_OVERRIDE_CHANGE_UNOBSERVED',
      'POST_SENTINEL_TAG_MAPPING_UNOBSERVED', 'BARE_TRANSPORT_POSITIVE_NO_PUBLIC_CODEC'],
    scope: { freedHandles: 'CALLER_VISIBLE_WRAPPERS_ONLY',
      internalGetterWrappers: 'SDK_FINALIZATION_REGISTRY_UNOBSERVED',
      timeout: 'EXTERNAL_CAPTURE_REQUIRED_UNSETTLED_AWAIT_FAILS' },
    nonclaims };
} catch (error) {
  failureStage ??= stage;
  if (!firstFailure) firstFailure = error instanceof SmokeFailure
    ? error : new SmokeFailure(`SDK_${stage}_FAILURE`);
} finally {
  // All awaited SDK use has settled; aliases never enter this owner stack twice.
  if (inFlight) {
    firstFailure ??= new SmokeFailure('FREE_WHILE_IN_FLIGHT');
    failureStage ??= 'CLEANUP';
  }
  if (!inFlight) for (const record of owners.slice().reverse()) {
    try {
      if (record.attempted) fail('FREE_DUPLICATE_ATTEMPT');
      record.attempted = true;
      record.handle.free();
      freed++;
    } catch {
      firstFailure ??= new SmokeFailure('FREE_FAILED');
      failureStage ??= 'CLEANUP';
    }
  }
}
if (!firstFailure && freed !== owners.length) {
  firstFailure = new SmokeFailure('FREE_COUNT_MISMATCH');
  failureStage = 'CLEANUP';
}
if (firstFailure) {
  process.stderr.write(JSON.stringify({ schema: 'public-installed-callback-codec-smoke/1',
    status: 'FAIL', stage: failureStage, code: firstFailure.code, callbacks: calls,
    ownership: { owned: owners.length, freed, inFlight }, nonclaims }) + '\n');
  process.exitCode = 1;
} else {
  process.stdout.write(JSON.stringify({ ...observations,
    ownership: { owned: owners.length, freed, distinctHandles: handles.size,
      inFlight } }) + '\n');
}
