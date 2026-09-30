/** Local content-only W-D2H experiment. No provider or consumer is implemented. */
import { createHash } from 'node:crypto';

export const SOURCE_PROFILE = 'moriarty-financial-agreement-source/6';
const PREFIX = Buffer.from('moriarty-mil4-image/1\0', 'ascii');
const SOURCE_KEYS = ['sourceVersion', 'profile', 'wireProfile', 'agreementInstanceId', 'domain', 'asset',
  'scale', 'selectedActionId', 'operationKind'];
const CORE_KEYS = ['coreVersion', 'coreProfile', 'sourceVersion', 'intentSchema', 'wireProfile',
  'coreProgramId', 'operationKind', 'lowerEntry', 'prepareEntry', 'wrapperEntry', 'fileCount', 'files'];
const TERM_KEYS = ['signer', 'keyRef', 'validFrom', 'validUntil', 'grossCap', 'feeCap', 'netFloor',
  'failureRelation', 'supplyChanges', 'observations', 'disclosures', 'retainedEffects', 'retainedDuties',
  'delegation', 'recovery', 'operation'];
const POLICY_KEYS = ['sourceVersion', 'coreVersion', 'wireProfile', 'agreementInstanceId', 'domain', 'asset',
  'scale', 'actionId', 'coreProgramId', 'operationKind', 'sourceHash', 'coreHash', ...TERM_KEYS];
const SORTS = new Set(['agreement', 'domain', 'asset', 'action', 'coreProgram', 'signer', 'account', 'obligation']);
// Exact Source/6 reserved-word set frozen with the proposed package, not an ambient import.
const RESERVED = new Set(('profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head').split(' '));
const NOMINAL_MAX = (1n << 127n) - 1n;
const ROUND_MAX = (1n << 64n) - 1n;
const TYPED_ARRAY = Object.getPrototypeOf(Uint8Array.prototype);
const intrinsicLength = Object.getOwnPropertyDescriptor(TYPED_ARRAY, 'byteLength').get;
const intrinsicOffset = Object.getOwnPropertyDescriptor(TYPED_ARRAY, 'byteOffset').get;
const intrinsicBuffer = Object.getOwnPropertyDescriptor(TYPED_ARRAY, 'buffer').get;

export class ImageCodecError extends Error {
  constructor(field, message) {
    super(`${field}: ${message}`);
    this.name = 'ImageCodecError';
    this.field = field;
    this.scope = 'local-image-domain';
  }
}
const reject = (field, message) => { throw new ImageCodecError(field, message); };

function record(value, keys, path) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) reject(path, 'closed record required');
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) reject(path, 'plain data record required');
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const actual = Reflect.ownKeys(descriptors);
  if (actual.length !== keys.length || actual.some(key => typeof key !== 'string' || !keys.includes(key)))
    reject(path, 'unexpected or missing field');
  for (const key of keys) {
    const descriptor = descriptors[key];
    if (!descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable)
      reject(`${path}.${key}`, 'enumerable data field required');
  }
}

// All caller records cross this boundary before validation or hashing. Descriptor
// values are captured once; ordinary Proxy get traps are never authoritative.
// The memo preserves shared nested inputs across Source/Core/policy phases.
function snapshotRecord(value, keys, path, memo, children = {}) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) reject(path, 'closed record required');
  const prior = memo.get(value);
  if (prior) {
    if (prior.building || !prior.descriptors) reject(path, 'cyclic or incompatible shared records forbidden');
    const expected = typeof keys === 'function' ? keys(prior.descriptors) : keys;
    if (prior.shape !== expected.join('|')) reject(path, 'shared input has incompatible record roles');
    return prior.snapshot;
  }
  let prototype, descriptors;
  try { prototype = Object.getPrototypeOf(value); descriptors = Object.getOwnPropertyDescriptors(value); }
  catch { reject(path, 'record descriptor capture failed'); }
  if (prototype !== Object.prototype && prototype !== null) reject(path, 'plain data record required');
  const expected = typeof keys === 'function' ? keys(descriptors) : keys;
  const actual = Reflect.ownKeys(descriptors);
  if (actual.length !== expected.length || actual.some(key => typeof key !== 'string' || !expected.includes(key)))
    reject(path, 'unexpected or missing field');
  const output = {};
  for (const key of expected) {
    const descriptor = descriptors[key];
    if (!descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable)
      reject(`${path}.${key}`, 'enumerable data field required');
    output[key] = descriptor.value;
  }
  const entry = {shape: expected.join('|'), descriptors, building: true};
  memo.set(value, entry);
  for (const [key, child] of Object.entries(children))
    if (Object.hasOwn(output, key)) output[key] = child(output[key], `${path}.${key}`, memo);
  entry.snapshot = Object.freeze(output);
  entry.building = false;
  return entry.snapshot;
}

const snapshotId = (value, path, memo) => snapshotRecord(value, ['sort', 'value'], path, memo);

function snapshotBytes(value, path, memo) {
  const prior = memo.get(value);
  if (prior) {
    if (prior.shape !== 'bytes') reject(path, 'incompatible shared byte input');
    return prior.snapshot;
  }
  if (!(value instanceof Uint8Array)) reject(path, 'explicit Uint8Array bytes required');
  let byteLength, byteOffset, backing;
  try {
    byteLength = intrinsicLength.call(value);
    byteOffset = intrinsicOffset.call(value);
    backing = intrinsicBuffer.call(value);
  } catch { reject(path, 'genuine Uint8Array view required'); }
  if (backing instanceof SharedArrayBuffer) reject(path, 'shared byte memory forbidden');
  if (byteLength > 1048576) reject(path, 'file exceeds 1MiB');
  let snapshot;
  try { snapshot = Buffer.from(new Uint8Array(backing, byteOffset, byteLength)); }
  catch { reject(path, 'detached or invalid byte memory'); }
  // Buffers are owned copies. They remain private until copied into the payload.
  memo.set(value, {shape: 'bytes', snapshot});
  return snapshot;
}

function snapshotFiles(value, path, memo) {
  const prior = memo.get(value);
  if (prior) {
    if (prior.shape !== 'files-array' || prior.building) reject(path, 'incompatible shared file input');
    return prior.snapshot;
  }
  if (!Array.isArray(value)) reject(path, 'exact ordered array of three file records required');
  let prototype, descriptors;
  try { prototype = Object.getPrototypeOf(value); descriptors = Object.getOwnPropertyDescriptors(value); }
  catch { reject(path, 'file array descriptor capture failed'); }
  const actual = Reflect.ownKeys(descriptors);
  if (prototype !== Array.prototype || actual.length !== 4
      || actual.some(key => !['0', '1', '2', 'length'].includes(key)) || descriptors.length?.value !== 3)
    reject(path, 'exact ordered array of three file records required');
  const entry = {shape: 'files-array', building: true}; memo.set(value, entry);
  const output = [];
  for (let i = 0; i < 3; i++) {
    const descriptor = descriptors[i];
    if (!descriptor || !Object.hasOwn(descriptor, 'value') || !descriptor.enumerable)
      reject(`${path}.${i}`, 'enumerable data element required');
    output.push(snapshotRecord(descriptor.value, ['role', 'bytes'], `${path}.${i}`, memo, {bytes: snapshotBytes}));
  }
  entry.snapshot = Object.freeze(output); entry.building = false;
  return entry.snapshot;
}

function snapshotOperation(value, path, memo) {
  const keys = descriptors => {
    const kind = descriptors.kind?.value;
    if (kind === 'Transfer') return ['kind', 'owner', 'recipient', 'feeRecipient', 'amount', 'fee'];
    if (kind === 'Repay') return ['kind', 'obligationId', 'payer', 'amount', 'allocation', 'conversion'];
    reject(`${path}.kind`, 'unsupported operation constructor');
  };
  // Only children present in the selected closed variant are traversed.
  const children = {owner: snapshotId, recipient: snapshotId, feeRecipient: snapshotId,
    obligationId: snapshotId, payer: snapshotId};
  return snapshotRecord(value, keys, path, memo, children);
}

function snapshotSource(value, memo) {
  return snapshotRecord(value, SOURCE_KEYS, 'sourceDefinition', memo,
    {agreementInstanceId: snapshotId, domain: snapshotId, asset: snapshotId, selectedActionId: snapshotId});
}
function snapshotCore(value, memo) {
  return snapshotRecord(value, CORE_KEYS, 'package', memo, {coreProgramId: snapshotId, files: snapshotFiles});
}
function snapshotTerms(value, memo) {
  return snapshotRecord(value, TERM_KEYS, 'intent', memo, {signer: snapshotId, operation: snapshotOperation});
}
function snapshotPolicy(value, memo) {
  return snapshotRecord(value, POLICY_KEYS, 'policy', memo, {agreementInstanceId: snapshotId, domain: snapshotId,
    asset: snapshotId, actionId: snapshotId, coreProgramId: snapshotId, signer: snapshotId, operation: snapshotOperation});
}

function constant(value, expected, field) {
  if (value !== expected) reject(field, `expected exact ${expected}`);
}

function idText(value, sort, field) {
  record(value, ['sort', 'value'], field);
  constant(value.sort, sort, `${field}.sort`);
  if (typeof value.value !== 'string' || value.value.length > 64
      || !/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(value.value) || RESERVED.has(value.value))
    reject(field, 'Source common identifier required (1–64 ASCII bytes; reserved words excluded)');
  return value.value;
}

/** Explicit nominal sorts prevent equal text from merging agreement/action/Core roles. */
export function id(sort, value) {
  if (!SORTS.has(sort)) reject('id.sort', 'unknown nominal identifier role');
  const result = {sort, value};
  idText(result, sort, 'id');
  return Object.freeze(result);
}

function identifier(value, sort, field) {
  const content = idText(value, sort, field);
  const count = Buffer.alloc(2);
  count.writeUInt16BE(content.length);
  return Buffer.concat([count, Buffer.from(content, 'ascii')]);
}

function text(value, field, opaque = false) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 1024)
    reject(field, 'text must be nonempty and at most 1024 UTF-16 code units');
  for (let i = 0; i < value.length; i++) {
    const code = value.charCodeAt(i);
    if (code >= 0xd800 && code <= 0xdbff) {
      const low = value.charCodeAt(++i);
      if (!(low >= 0xdc00 && low <= 0xdfff)) reject(field, 'lone high surrogate');
    } else if (code >= 0xdc00 && code <= 0xdfff) reject(field, 'lone low surrogate');
    if (opaque && (code <= 0x1f || code === 0x7f)) reject(field, 'Core opaque controls forbidden');
  }
  const length = Buffer.byteLength(value, 'utf8');
  if (length > 1024) reject(field, 'text exceeds 1024 UTF-8 bytes');
  const count = Buffer.alloc(2);
  count.writeUInt16BE(length);
  // Surrogates were checked before Buffer encoding, which otherwise repairs them.
  return Buffer.concat([count, Buffer.from(value, 'utf8')]);
}

function decimal(value, max, field) {
  // Bound string length before regex/BigInt; no number, coercion, zeros or repair.
  if (typeof value !== 'string' || value.length > max.toString().length
      || !/^(0|[1-9][0-9]*)$/.test(value)) reject(field, 'canonical unsigned decimal string required');
  const result = BigInt(value);
  if (result > max) reject(field, 'integer out of domain');
  return result;
}

function unsigned(value, width, max, field) {
  let integer = decimal(value, max, field);
  const output = Buffer.alloc(width);
  for (let i = width - 1; i >= 0; i--) { output[i] = Number(integer & 255n); integer >>= 8n; }
  return output;
}

function hash32(value, field) {
  if (typeof value !== 'string' || value.length !== 64 || !/^[0-9a-f]{64}$/.test(value))
    reject(field, 'exact lowercase 64-character hash presentation required');
  return Buffer.from(value, 'hex');
}

function operationKind(kind, selected, field) {
  const expected = kind === 'Transfer' ? 'TransferLiteralFee' : kind === 'Repay' ? 'RepayAccrualFirst' : null;
  if (expected === null || selected !== expected) reject(field, 'unsupported selector/constructor pair');
  return kind === 'Transfer' ? 1 : 2;
}

function capped(parts, cap, field) {
  const length = parts.reduce((sum, part) => sum + part.length, 0);
  if (length > cap) reject(field, `payload exceeds ${cap} bytes`);
  return Buffer.concat(parts, length);
}

function sourcePayload(value) {
  record(value, SOURCE_KEYS, 'sourceDefinition');
  constant(value.sourceVersion, 6, 'sourceDefinition.sourceVersion');
  constant(value.profile, SOURCE_PROFILE, 'sourceDefinition.profile');
  constant(value.wireProfile, 1, 'sourceDefinition.wireProfile');
  const selected = idText(value.selectedActionId, 'action', 'sourceDefinition.selectedActionId');
  const kind = operationKind(value.operationKind, selected, 'sourceDefinition.operationKind');
  return capped([Buffer.from([6]), text(value.profile, 'sourceDefinition.profile'), Buffer.from([1]),
    identifier(value.agreementInstanceId, 'agreement', 'sourceDefinition.agreementInstanceId'),
    identifier(value.domain, 'domain', 'sourceDefinition.domain'), identifier(value.asset, 'asset', 'sourceDefinition.asset'),
    unsigned(value.scale, 1, 18n, 'sourceDefinition.scale'), identifier(value.selectedActionId, 'action', 'sourceDefinition.selectedActionId'),
    Buffer.from([kind])], 4096, 'sourceDefinition');
}

function packageFiles(value) {
  if (!Array.isArray(value) || Object.getPrototypeOf(value) !== Array.prototype || value.length !== 3)
    reject('package.files', 'exact ordered array of three file records required');
  const descriptors = Object.getOwnPropertyDescriptors(value);
  if (Reflect.ownKeys(descriptors).length !== 4) reject('package.files', 'extra array fields forbidden');
  const parts = [];
  for (let i = 0; i < 3; i++) {
    if (!Object.hasOwn(descriptors[i] ?? {}, 'value')) reject(`package.files.${i}`, 'data element required');
    const file = descriptors[i].value;
    const path = `package.files.${i}`;
    record(file, ['role', 'bytes'], path);
    constant(file.role, i + 1, `${path}.role`);
    if (!(file.bytes instanceof Uint8Array)) reject(`${path}.bytes`, 'explicit Uint8Array bytes required');
    let byteLength, byteOffset, backing;
    try {
      byteLength = intrinsicLength.call(file.bytes);
      byteOffset = intrinsicOffset.call(file.bytes);
      backing = intrinsicBuffer.call(file.bytes);
    } catch { reject(`${path}.bytes`, 'genuine Uint8Array view required'); }
    if (backing instanceof SharedArrayBuffer) reject(`${path}.bytes`, 'shared byte memory forbidden');
    if (byteLength > 1048576) reject(`${path}.bytes`, 'file exceeds 1MiB');
    // Read intrinsic slots, never caller-shadowable length/buffer properties.
    let content;
    try { content = Buffer.from(new Uint8Array(backing, byteOffset, byteLength)); }
    catch { reject(`${path}.bytes`, 'detached or invalid byte memory'); }
    const length = Buffer.alloc(4); length.writeUInt32BE(byteLength);
    parts.push(Buffer.from([i + 1]), length, content);
  }
  return parts;
}

function corePayload(value) {
  record(value, CORE_KEYS, 'package');
  for (const [key, expected] of Object.entries({coreVersion: 5, coreProfile: 'moriarty-core/5', sourceVersion: 6,
    intentSchema: 'moriarty-intent/3', wireProfile: 1, lowerEntry: 'parseAndLowerSource6',
    prepareEntry: 'prepareMil4S0', wrapperEntry: 'prepareSource6S0Unqualified', fileCount: 3}))
    constant(value[key], expected, `package.${key}`);
  const selected = idText(value.coreProgramId, 'coreProgram', 'package.coreProgramId');
  const kind = operationKind(value.operationKind, selected, 'package.operationKind');
  const files = packageFiles(value.files);
  return capped([Buffer.from([5]), text(value.coreProfile, 'package.coreProfile'), Buffer.from([6]),
    text(value.intentSchema, 'package.intentSchema'), Buffer.from([1]),
    identifier(value.coreProgramId, 'coreProgram', 'package.coreProgramId'), Buffer.from([kind]),
    text(value.lowerEntry, 'package.lowerEntry'), text(value.prepareEntry, 'package.prepareEntry'),
    text(value.wrapperEntry, 'package.wrapperEntry'), Buffer.from([0, 3]), ...files], 4194304, 'package');
}

function policyOperation(value, kind) {
  const path = 'policy.operation';
  if (kind === 'Transfer') {
    record(value, ['kind', 'owner', 'recipient', 'feeRecipient', 'amount', 'fee'], path);
    constant(value.kind, kind, `${path}.kind`);
    return [Buffer.from([1]), identifier(value.owner, 'account', `${path}.owner`),
      identifier(value.recipient, 'account', `${path}.recipient`), identifier(value.feeRecipient, 'account', `${path}.feeRecipient`),
      unsigned(value.amount, 16, NOMINAL_MAX, `${path}.amount`), unsigned(value.fee, 16, NOMINAL_MAX, `${path}.fee`)];
  }
  record(value, ['kind', 'obligationId', 'payer', 'amount', 'allocation', 'conversion'], path);
  constant(value.kind, 'Repay', `${path}.kind`);
  constant(value.allocation, 'AccrualFirst', `${path}.allocation`);
  constant(value.conversion, 'identity', `${path}.conversion`);
  return [Buffer.from([2]), identifier(value.obligationId, 'obligation', `${path}.obligationId`),
    identifier(value.payer, 'account', `${path}.payer`), unsigned(value.amount, 16, NOMINAL_MAX, `${path}.amount`), Buffer.from([1, 1])];
}

function policyPayload(value) {
  record(value, POLICY_KEYS, 'policy');
  for (const [key, expected] of Object.entries({sourceVersion: 6, coreVersion: 5, wireProfile: 1,
    failureRelation: 'success_only', supplyChanges: 'empty', observations: 'empty', disclosures: 'empty',
    retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none'}))
    constant(value[key], expected, `policy.${key}`);
  // KeyRef is validated as decoded scalar text before encoding any body bytes.
  const keyRef = text(value.keyRef, 'policy.keyRef', true);
  const action = idText(value.actionId, 'action', 'policy.actionId');
  idText(value.coreProgramId, 'coreProgram', 'policy.coreProgramId');
  const kind = operationKind(value.operationKind, action, 'policy.operationKind');
  // H-H20 requires a genuine differing policy Core ID to survive image formation
  // and reach a later body/context comparison. Content encoding performs no such
  // consumer comparison. Candidate composition below establishes its own links.
  if (decimal(value.validFrom, ROUND_MAX, 'policy.validFrom') > decimal(value.validUntil, ROUND_MAX, 'policy.validUntil'))
    reject('policy.validFrom', 'validity interval is reversed');
  return capped([Buffer.from([6, 5, 1]), identifier(value.agreementInstanceId, 'agreement', 'policy.agreementInstanceId'),
    identifier(value.domain, 'domain', 'policy.domain'), identifier(value.asset, 'asset', 'policy.asset'),
    unsigned(value.scale, 1, 18n, 'policy.scale'), identifier(value.actionId, 'action', 'policy.actionId'),
    identifier(value.coreProgramId, 'coreProgram', 'policy.coreProgramId'), Buffer.from([kind]),
    hash32(value.sourceHash, 'policy.sourceHash'), hash32(value.coreHash, 'policy.coreHash'),
    identifier(value.signer, 'signer', 'policy.signer'), keyRef,
    unsigned(value.validFrom, 8, ROUND_MAX, 'policy.validFrom'), unsigned(value.validUntil, 8, ROUND_MAX, 'policy.validUntil'),
    unsigned(value.grossCap, 16, NOMINAL_MAX, 'policy.grossCap'), unsigned(value.feeCap, 16, NOMINAL_MAX, 'policy.feeCap'),
    unsigned(value.netFloor, 16, NOMINAL_MAX, 'policy.netFloor'), Buffer.from([1]),
    Buffer.from([0, 0]), Buffer.from([0, 0]), Buffer.from([0, 0]), Buffer.from([0, 0]), Buffer.from([0, 0]),
    Buffer.from([0]), Buffer.from([0]), ...policyOperation(value.operation, value.operationKind)], 4096, 'policy');
}

/** Producers accept only closed typed records; purpose 2 is unavailable. */
function encodeOwned(purpose, value) {
  const payload = purpose === 1 ? sourcePayload(value) : purpose === 3 ? corePayload(value) : purpose === 4 ? policyPayload(value) :
    reject('purpose', 'only purposes 1, 3 and 4 are available in this experiment');
  const length = Buffer.alloc(4); length.writeUInt32BE(payload.length);
  const preimage = Buffer.concat([PREFIX, Buffer.from([purpose]), length, payload]);
  return {purpose, payload, preimage, digest: createHash('sha256').update(preimage).digest('hex')};
}

export function encodeImage(purpose, value) {
  const memo = new WeakMap();
  const owned = purpose === 1 ? snapshotSource(value, memo) : purpose === 3 ? snapshotCore(value, memo) :
    purpose === 4 ? snapshotPolicy(value, memo) : reject('purpose', 'only purposes 1, 3 and 4 are available in this experiment');
  return encodeOwned(purpose, owned);
}

/** Link policy to computed content hashes, never to supplied Source claim slots. */
export function produceImages(definition, corePackage, terms) {
  const memo = new WeakMap();
  definition = snapshotSource(definition, memo);
  corePackage = snapshotCore(corePackage, memo);
  terms = snapshotTerms(terms, memo);
  const source = encodeOwned(1, definition);
  const core = encodeOwned(3, corePackage);
  if (definition.selectedActionId.value !== corePackage.coreProgramId.value || definition.operationKind !== corePackage.operationKind)
    reject('package.coreProgramId', 'selected Source/package identity differs');
  const policyValue = Object.freeze({sourceVersion: 6, coreVersion: 5, wireProfile: 1,
    agreementInstanceId: definition.agreementInstanceId, domain: definition.domain, asset: definition.asset, scale: definition.scale,
    actionId: definition.selectedActionId, coreProgramId: corePackage.coreProgramId, operationKind: definition.operationKind,
    sourceHash: source.digest, coreHash: core.digest,
    signer: terms.signer, keyRef: terms.keyRef, validFrom: terms.validFrom, validUntil: terms.validUntil,
    grossCap: terms.grossCap, feeCap: terms.feeCap, netFloor: terms.netFloor, failureRelation: terms.failureRelation,
    supplyChanges: terms.supplyChanges, observations: terms.observations, disclosures: terms.disclosures,
    retainedEffects: terms.retainedEffects, retainedDuties: terms.retainedDuties, delegation: terms.delegation,
    recovery: terms.recovery, operation: terms.operation});
  let policy;
  try { policy = encodeOwned(4, policyValue); } catch (error) {
    if (error instanceof ImageCodecError && error.field === 'policy.keyRef') {
      Object.assign(error, {classification: 'BindingRejected', code: 'W_D2F_DOMAIN_UNSUPPORTED', binding: 'B07',
        inputPath: 'intent.keyRef', sourcePath: 'intent.keyRef', comparisonTag: null, factPath: null,
        publishedPost: null, publishedEffects: null});
    }
    throw error;
  }
  return {source, core, policy, policyValue};
}

/** Matching content is explicitly unauthenticated; no B05/B06/B07 result is issued. */
export function compareContent(purpose, value, claimedDigest) {
  hash32(claimedDigest, 'claimedDigest');
  const {digest} = encodeImage(purpose, value);
  return {matches: digest === claimedDigest, digest, claimedDigest, scope: 'content-only', authenticated: false};
}
