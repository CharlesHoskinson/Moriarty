/** Local authenticated plan format. No function here verifies a contract theorem or PCD proof. */
import type { Action, CoreState, Effect, Evaluation } from './core.ts';
import { checkIntentEffects, type IntentPolicy } from './policy.ts';

export type Domain = { network: string; deployment: string; semantics: 'moriarty-r2/1'; verifierProfile: 'midnight-native-pcd/1' };
export type ClaimType = 'ContractInvariant' | 'IntentEffects' | 'TransitionValidity' | 'HistoryCompliance';
export type ClaimSpec = { type: ClaimType; version: '1'; mode: 'mandatory'; programHash: string;
  specificationHash: string; verifierProfile: string; txCoreHash: string; effectHash: string; dependencies: string[] };
export type PreparedPlan = { kind: 'PreparedLocalPlan'; domain: Domain;
  txCore: { programHash: string; before: CoreState; action: Action; after: CoreState; effects: Effect[] };
  policy: IntentPolicy; claims: { id: string; spec: ClaimSpec }[]; manifestRoot: string; intentDigest: string };
export type SignedPlan = { kind: 'SignedLocalPlan'; plan: PreparedPlan; publicKey: string; signature: string;
  boundClaims: { claimId: string; intentDigest: string; evidence: null }[] };
type Rejected = { outcome: 'rejected'; code: string; message: string };
type Checked = { outcome: 'checked'; predicate: 'signature-and-bindings'; assurance: 'local-check-only' };
const TYPES: ClaimType[] = ['ContractInvariant', 'IntentEffects', 'TransitionValidity', 'HistoryCompliance'];
const encoder = new TextEncoder();
const MAX_BYTES = 65536;

/** Canonical v1 domain: JSON null/bool/string/array/plain record; no numeric tokens. */
export function canonical(value: unknown): string {
  const ancestors = new Set<object>(); let nodes = 0;
  const visit = (x: unknown, depth: number): string => {
    if (depth > 16 || ++nodes > 8192) throw new Error('EncodingBounds: depth or node count exceeded.');
    if (x === null) return 'null';
    if (typeof x === 'boolean') return x ? 'true' : 'false';
    if (typeof x === 'string') {
      if (x.length > 4096 || encoder.encode(x).length > 4096 || /[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/u.test(x)) throw new Error('EncodingBounds: invalid or oversized text.');
      return JSON.stringify(x);
    }
    if (typeof x !== 'object') throw new Error('InvalidEncoding: numeric tokens, undefined and executable values are forbidden.');
    if (ancestors.has(x)) throw new Error('InvalidEncoding: cyclic object.');
    ancestors.add(x);
    let encoded: string;
    if (Array.isArray(x)) {
      if (Object.getPrototypeOf(x) !== Array.prototype || x.length > 128) throw new Error('EncodingBounds: nonstandard or oversized array.');
      const descriptors = Object.getOwnPropertyDescriptors(x);
      const keys = Reflect.ownKeys(descriptors);
      if (keys.some(key => typeof key === 'symbol') || keys.length !== x.length + 1) throw new Error('InvalidEncoding: sparse or extended array.');
      const values: unknown[] = [];
      for (let n = 0; n < x.length; n++) {
        const descriptor = descriptors[String(n)];
        if (!descriptor || !descriptor.enumerable || !('value' in descriptor)) throw new Error('InvalidEncoding: sparse, hidden or accessor array item.');
        values.push(descriptor.value);
      }
      encoded = '[' + values.map(v => visit(v, depth + 1)).join(',') + ']';
    } else {
      if (Object.getPrototypeOf(x) !== Object.prototype && Object.getPrototypeOf(x) !== null) throw new Error('InvalidEncoding: non-plain object.');
      const keys = Object.keys(x).sort();
      if (keys.length > 64 || Reflect.ownKeys(x).length !== keys.length) throw new Error('EncodingBounds: too many, symbolic or non-enumerable keys.');
      encoded = '{' + keys.map(k => {
        if (!/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(k) || ['__proto__', 'constructor', 'prototype'].includes(k)) throw new Error('InvalidEncoding: unsupported key.');
        const descriptor = Object.getOwnPropertyDescriptor(x, k)!;
        if (!('value' in descriptor)) throw new Error('InvalidEncoding: accessor field.');
        return JSON.stringify(k) + ':' + visit(descriptor.value, depth + 1);
      }).join(',') + '}';
    }
    ancestors.delete(x);
    if (encoded.length > MAX_BYTES) throw new Error('EncodingBounds: object exceeds byte budget.');
    return encoded;
  };
  const bytes = visit(value, 0);
  if (encoder.encode(bytes).length > MAX_BYTES) throw new Error('EncodingBounds: envelope exceeds byte budget.');
  return bytes;
}

export function decodeCanonical(text: string): unknown {
  if (typeof text !== 'string' || text.length > MAX_BYTES || encoder.encode(text).length > MAX_BYTES) throw new Error('EncodingBounds: input exceeds byte budget.');
  const value: unknown = JSON.parse(text);
  // Re-encoding rejects duplicate keys, alternate ordering/escapes, whitespace and numbers.
  if (canonical(value) !== text) throw new Error('NonCanonicalEncoding: bytes do not match canonical encoding.');
  return value;
}

function hex(bytes: ArrayBuffer): string { return Array.from(new Uint8Array(bytes), b => b.toString(16).padStart(2, '0')).join(''); }
function fromHex(text: string, size: number): Uint8Array<ArrayBuffer> {
  if (typeof text !== 'string' || text.length !== size * 2 || !/^[0-9a-f]+$/.test(text)) throw new Error('InvalidEncoding: wrong hex size or alphabet.');
  return Uint8Array.from(text.match(/../g)!, b => parseInt(b, 16));
}
export async function hashCanonical(value: unknown): Promise<string> { return hex(await crypto.subtle.digest('SHA-256', encoder.encode(canonical(value)))); }
function reject(error: unknown): Rejected { return { outcome: 'rejected', code: 'InvalidPlan', message: error instanceof Error ? error.message : 'Invalid local plan.' }; }
function exactKeys(object: unknown, keys: string[]): void {
  if (!object || typeof object !== 'object' || Array.isArray(object) || Object.keys(object).sort().join('|') !== [...keys].sort().join('|')) throw new Error('InvalidEncoding: unknown or missing fields.');
}
function validateDomain(domain: Domain): void {
  exactKeys(domain, ['network','deployment','semantics','verifierProfile']);
  for (const value of Object.values(domain)) if (typeof value !== 'string' || !/^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,63}$/.test(value)) throw new Error('WrongDomain: invalid identifier.');
  if (domain.semantics !== 'moriarty-r2/1' || domain.verifierProfile !== 'midnight-native-pcd/1') throw new Error('UnsupportedProfile: semantics or verifier profile is not permitted.');
}

async function makePlan(txCore: PreparedPlan['txCore'], policy: IntentPolicy, domain: Domain): Promise<PreparedPlan> {
  validateDomain(domain);
  exactKeys(txCore, ['programHash','before','action','after','effects']);
  fromHex(txCore.programHash, 32);
  exactKeys(txCore.action, ['name','args']);
  const policyCheck = checkIntentEffects(txCore.before, txCore.after, txCore.effects, policy);
  if (policyCheck.outcome === 'rejected') throw new Error(policyCheck.code + ': ' + policyCheck.message);
  const txCoreHash = await hashCanonical({ domain:'MORIARTY-TX-CORE-v1', txCore });
  const effectHash = await hashCanonical({ domain:'MORIARTY-EFFECTS-v1', effects:txCore.effects });
  const claims: PreparedPlan['claims'] = [];
  for (const type of TYPES) {
    const dependencies = type === 'TransitionValidity' ? claims.map(c => c.id) : type === 'HistoryCompliance' ? [claims[2]!.id] : [];
    const spec: ClaimSpec = { type, version:'1', mode:'mandatory', programHash:txCore.programHash,
      specificationHash:await hashCanonical({ domain:'MORIARTY-SPEC-v1', type, semantics:domain.semantics }),
      verifierProfile:domain.verifierProfile, txCoreHash, effectHash, dependencies };
    claims.push({ id:await hashCanonical({ domain:'MORIARTY-CLAIM-v1', spec }), spec });
  }
  const manifestRoot = await hashCanonical({ domain:'MORIARTY-MANIFEST-v1', claims });
  const intentDigest = await hashCanonical({ domain:'MORIARTY-INTENT-v1', deploymentDomain:domain,
    intentCore: { policy }, txCoreHash, manifestRoot });
  return { kind:'PreparedLocalPlan', domain, txCore, policy, claims, manifestRoot, intentDigest };
}

/** The result is a plan for local signing, not evidence of program or history correctness. */
export async function prepareLocalPlan(program: unknown, before: CoreState, action: Action, result: Evaluation,
  policy: IntentPolicy, domain: Domain): Promise<{outcome:'prepared'; plan:PreparedPlan} | Rejected> {
  try {
    // Capture every caller-owned input synchronously. No reference crosses the first digest await.
    const snapshot = decodeCanonical(canonical({ program, before, action, result, policy, domain })) as {
      program: unknown; before: CoreState; action: Action; result: Evaluation; policy: IntentPolicy; domain: Domain;
    };
    if (snapshot.result.outcome !== 'evaluated') throw new Error('EvaluationRejected: no plan can be prepared.');
    if (canonical(snapshot.before) !== canonical(snapshot.result.before)) throw new Error('WrongState: evaluation predecessor differs.');
    const txCore = decodeCanonical(canonical({programHash:await hashCanonical(snapshot.program), before:snapshot.before, action:snapshot.action,
      after:snapshot.result.after, effects:snapshot.result.effects})) as PreparedPlan['txCore'];
    const p = snapshot.policy;
    const d = snapshot.domain;
    const plan = await makePlan(txCore, p, d);
    canonical(plan);
    return { outcome:'prepared', plan };
  } catch (error) { return reject(error); }
}

async function validatePrepared(value: unknown): Promise<PreparedPlan> {
  const plan = decodeCanonical(canonical(value)) as PreparedPlan;
  exactKeys(plan, ['kind','domain','txCore','policy','claims','manifestRoot','intentDigest']);
  if (plan.kind !== 'PreparedLocalPlan') throw new Error('InvalidPlan: wrong kind.');
  const rebuilt = await makePlan(plan.txCore, plan.policy, plan.domain);
  if (canonical(rebuilt) !== canonical(plan)) throw new Error('ClaimBindingMismatch: required types, modes, predicates, dependencies or commitments differ.');
  return plan;
}

export async function generateLocalKey(): Promise<CryptoKeyPair> {
  // Nonextractable private key; public key is exported only for explicit trust comparison.
  return await crypto.subtle.generateKey({ name:'Ed25519' }, false, ['sign','verify']) as CryptoKeyPair;
}
export async function signLocalPlan(value: PreparedPlan, keys: CryptoKeyPair): Promise<SignedPlan> {
  const plan = await validatePrepared(value);
  const publicKey = hex(await crypto.subtle.exportKey('raw', keys.publicKey));
  const signature = hex(await crypto.subtle.sign('Ed25519', keys.privateKey,
    encoder.encode(canonical({ domain:'MORIARTY-SIGN-v1', intentDigest:plan.intentDigest }))));
  return { kind:'SignedLocalPlan', plan, publicKey, signature,
    boundClaims:plan.claims.map(c => ({claimId:c.id, intentDigest:plan.intentDigest, evidence:null})) };
}

/** expected comes from the relying party, and trustedPublicKey from its key policy. */
export async function verifySignedPlan(value: unknown, expected: PreparedPlan, trustedPublicKey: string): Promise<Checked | Rejected> {
  try {
    const envelope = decodeCanonical(canonical(value)) as SignedPlan;
    exactKeys(envelope, ['kind','plan','publicKey','signature','boundClaims']);
    if (envelope.kind !== 'SignedLocalPlan') throw new Error('InvalidPlan: wrong envelope kind.');
    const trusted = await validatePrepared(expected);
    const plan = await validatePrepared(envelope.plan);
    if (canonical(plan) !== canonical(trusted)) throw new Error('WrongContext: signed plan differs from relying party program/state/domain/policy.');
    const required = plan.claims.map(c => ({claimId:c.id,intentDigest:plan.intentDigest,evidence:null}));
    if (canonical(envelope.boundClaims) !== canonical(required)) throw new Error('MissingRequiredClaim: removed, altered or unsupported evidence descriptor.');
    if (envelope.publicKey !== trustedPublicKey) throw new Error('WrongSigner: public key is not trusted for this local request.');
    const publicBytes = fromHex(trustedPublicKey, 32); const signature = fromHex(envelope.signature, 64);
    const key = await crypto.subtle.importKey('raw', publicBytes, 'Ed25519', false, ['verify']);
    const valid = await crypto.subtle.verify('Ed25519', key, signature,
      encoder.encode(canonical({domain:'MORIARTY-SIGN-v1',intentDigest:plan.intentDigest})));
    if (!valid) throw new Error('InvalidSignature: Ed25519 verification failed.');
    return { outcome:'checked', predicate:'signature-and-bindings', assurance:'local-check-only' };
  } catch (error) { return reject(error); }
}

export async function verifyRequiredClaims(value: unknown, expected: PreparedPlan, trustedPublicKey: string): Promise<Rejected | {outcome:'unavailable'; missing:ClaimType[]; reason:string}> {
  const checked = await verifySignedPlan(value, expected, trustedPublicKey);
  if (checked.outcome === 'rejected') return checked;
  return { outcome:'unavailable', missing:[...TYPES], reason:'Signature and local intent/effect checks succeeded. Contract certificates, transition proofs and recursive history verification are not connected. Real Moriarty acceptance is unavailable.' };
}
