// PROVISIONAL wire experiment. No source/Core, wallet, proof or ledger consumer.
import { createHash } from 'node:crypto';

export const CAPS = Object.freeze({ recordBytes: 4096, identifierBytes: 64, scale: 38, u64: 2n ** 64n - 1n, nominal: 2n ** 127n - 1n });
export const HEADER = 'moriarty-intent/3\0';
const fields = [
  ['profile', 'literal', 's0-provisional/1', 1],
  ['domain', 'id'], ['agreementId', 'id'], ['stageId', 'id'], ['episodeId', 'id'], ['actionId', 'id'],
  ['sourceVersion', 'literal', 6, 6], ['sourceHash', 'hex'], ['coreVersion', 'literal', 5, 5],
  ['coreProgramId', 'id'], ['coreHash', 'hex'], ['policyHash', 'hex'], ['signer', 'id'],
  ['keyScheme', 'literal', 'schnorr_bip340', 1], ['signerKey', 'hex'], ['asset', 'id'], ['scale', 'scale'],
  ['preHead', 'hex'], ['predecessor', 'hex'], ['nonce', 'hex'], ['validFrom', 'u64'], ['validUntil', 'u64'],
  ['grossCap', 'nominal'], ['feeCap', 'nominal'], ['netFloor', 'nominal'], ['effectCommitment', 'hex'],
  ['failurePolicy', 'literal', 'atomic-reject-terminal-success', 1],
  ['supplyChanges', 'empty'], ['observations', 'empty'], ['disclosures', 'empty'], ['retainedEffects', 'empty'], ['retainedDuties', 'empty'],
  ['delegation', 'literal', 'none', 0], ['recovery', 'literal', 'none', 0], ['operation', 'operation'],
];
const transfer = [['owner', 'id'], ['recipient', 'id'], ['feeRecipient', 'id'], ['amount', 'nominal'], ['fee', 'nominal']];
const repayment = [['obligationId', 'id'], ['payer', 'id'], ['debtor', 'id'], ['creditor', 'id'], ['amount', 'nominal'], ['allocation', 'literal', 'AccrualFirst', 1], ['conversion', 'literal', 'identity', 1]];
const typedArrayPrototype = Object.getPrototypeOf(Uint8Array.prototype);
const viewType = Object.getOwnPropertyDescriptor(typedArrayPrototype, Symbol.toStringTag).get;
const viewByteLength = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteLength').get;
const viewBuffer = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'buffer').get;
const viewByteOffset = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteOffset').get;
function fail(code, field) { const error = new Error(`${code}: ${field}`); error.code = code; throw error; }
function shape(value, names, field, kindDescriptor) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE', field);
  const keys = Reflect.ownKeys(value);
  if (keys.length !== names.length || keys.some(key => typeof key !== 'string' || !names.includes(key))) fail('SHAPE', field);
  const snapshot = Object.create(null);
  for (const name of names) {
    const descriptor = name === 'kind' && kindDescriptor ? kindDescriptor : Object.getOwnPropertyDescriptor(value, name);
    if (!descriptor || !('value' in descriptor) || !descriptor.enumerable) fail('SHAPE', `${field}.${name}`);
    snapshot[name] = descriptor.value;
  }
  return snapshot;
}
function uintBytes(value, width) {
  const result = Buffer.alloc(width);
  for (let i = width - 1; i >= 0; i--) { result[i] = Number(value & 255n); value >>= 8n; }
  return result;
}
function readUInt(bytes) { let value = 0n; for (const byte of bytes) value = (value << 8n) | BigInt(byte); return value; }
function encodePrimitive(value, type, name, literal, byte) {
  switch (type) {
    case 'literal': if (value !== literal) fail('LITERAL', name); return Buffer.from([byte]);
    case 'id': {
      if (typeof value !== 'string') fail('ID', name);
      // Accepted IDs are ASCII, so a code-unit limit bounds validation and allocation.
      if (value.length > CAPS.identifierBytes) fail('LENGTH', name);
      if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
      const data = Buffer.from(value, 'ascii');
      return Buffer.concat([uintBytes(BigInt(data.length), 2), data]);
    }
    case 'hex': if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) fail('HEX', name); return Buffer.from(value, 'hex');
    case 'u64': case 'nominal': {
      if (typeof value !== 'string') fail('INTEGER', name);
      // Bound text before regex scanning and BigInt conversion, even if malformed.
      if (value.length > (type === 'u64' ? 20 : 39)) fail('RANGE', name);
      if (!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER', name);
      const number = BigInt(value);
      if (number > CAPS[type]) fail('RANGE', name);
      return uintBytes(number, type === 'u64' ? 8 : 16);
    }
    case 'scale': if (!Number.isInteger(value) || Object.is(value, -0) || value < 0 || value > CAPS.scale) fail('RANGE', name); return Buffer.from([value]);
    case 'empty': if (!Array.isArray(value) || Reflect.ownKeys(value).length !== 1 || value.length !== 0) fail('EMPTY', name); return Buffer.alloc(2);
    default: fail('SHAPE', name);
  }
}
function operationFields(value) {
  const descriptor = value && Object.getOwnPropertyDescriptor(value, 'kind');
  if (!descriptor || !('value' in descriptor)) fail('SHAPE', 'operation.kind');
  if (descriptor.value === 'transfer') return [1, transfer, descriptor];
  if (descriptor.value === 'repayment') return [2, repayment, descriptor];
  fail('OPERATION_TAG', 'operation.kind');
}
function encodeOperation(value) {
  const [tag, schema, kindDescriptor] = operationFields(value);
  value = shape(value, ['kind', ...schema.map(([name]) => name)], 'operation', kindDescriptor);
  const parts = [Buffer.from([tag])];
  for (const [name, type, literal, byte] of schema) parts.push(encodePrimitive(value[name], type, `operation.${name}`, literal, byte));
  if (value.amount === '0') fail('RANGE', 'operation.amount');
  return Buffer.concat(parts);
}
export function encodeAuthorization(value) {
  value = shape(value, ['schemaVersion', ...fields.map(([name]) => name)], 'authorization');
  if (value.schemaVersion !== 'moriarty-intent/3') fail('LITERAL', 'schemaVersion');
  const parts = [Buffer.from(HEADER, 'ascii')];
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    parts.push(Buffer.from([i + 1]));
    parts.push(type === 'operation' ? encodeOperation(value[name]) : encodePrimitive(value[name], type, name, literal, byte));
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  const result = Buffer.concat(parts);
  if (result.length > CAPS.recordBytes) fail('LENGTH', 'authorization');
  return result;
}
class Reader {
  constructor(bytes) { this.bytes = bytes; this.offset = 0; }
  take(size, field) {
    if (this.offset + size > this.bytes.length) fail('TRUNCATED', field);
    const result = this.bytes.subarray(this.offset, this.offset + size); this.offset += size; return result;
  }
  byte(field) { return this.take(1, field)[0]; }
  primitive(type, name, literal, byte) {
    switch (type) {
      case 'literal': if (this.byte(name) !== byte) fail('LITERAL', name); return literal;
      case 'id': {
        const length = Number(readUInt(this.take(2, name)));
        if (length === 0) fail('ID', name);
        if (length > CAPS.identifierBytes) fail('LENGTH', name);
        const data = this.take(length, name);
        if (data.some(value => value > 127)) fail('ID', name);
        const value = data.toString('ascii');
        if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
        return value;
      }
      case 'hex': return this.take(32, name).toString('hex');
      case 'u64': case 'nominal': {
        const number = readUInt(this.take(type === 'u64' ? 8 : 16, name));
        if (number > CAPS[type]) fail('RANGE', name);
        return number.toString();
      }
      case 'scale': { const value = this.byte(name); if (value > CAPS.scale) fail('RANGE', name); return value; }
      case 'empty': if (readUInt(this.take(2, name)) !== 0n) fail('EMPTY', name); return [];
      default: fail('SHAPE', name);
    }
  }
  operation() {
    const tag = this.byte('operation.kind');
    const schema = tag === 1 ? transfer : tag === 2 ? repayment : null;
    if (!schema) fail('OPERATION_TAG', 'operation.kind');
    const result = { kind: tag === 1 ? 'transfer' : 'repayment' };
    for (const [name, type, literal, byte] of schema) result[name] = this.primitive(type, `operation.${name}`, literal, byte);
    if (result.amount === '0') fail('RANGE', 'operation.amount');
    return result;
  }
}
export function decodeAuthorization(bytes) {
  // Brand check rejects proxies/forged prototypes before any shadowable property read.
  if (!ArrayBuffer.isView(bytes)) fail('SHAPE', 'bytes');
  let length, buffer, offset;
  try {
    if (viewType.call(bytes) !== 'Uint8Array') fail('SHAPE', 'bytes');
    length = viewByteLength.call(bytes);
    buffer = viewBuffer.call(bytes);
    offset = viewByteOffset.call(bytes);
  } catch { fail('SHAPE', 'bytes'); }
  if (length > CAPS.recordBytes) fail('LENGTH', 'bytes');
  let copy;
  try {
    // Construct a trusted view from intrinsic metadata; copy at most the checked cap.
    // Construction also rejects detached or incompatible backing storage with SHAPE.
    const view = new Uint8Array(buffer, offset, length);
    copy = Buffer.alloc(length);
    Uint8Array.prototype.set.call(copy, view);
  } catch { fail('SHAPE', 'bytes'); }
  const reader = new Reader(copy);
  if (!reader.take(Buffer.byteLength(HEADER), 'header').equals(Buffer.from(HEADER, 'ascii'))) fail('HEADER', 'header');
  const value = { schemaVersion: 'moriarty-intent/3' };
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    if (reader.byte(name) !== i + 1) fail('FIELD_TAG', name);
    value[name] = type === 'operation' ? reader.operation() : reader.primitive(type, name, literal, byte);
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  if (reader.offset !== reader.bytes.length) fail('TRAILING', 'bytes');
  return value;
}
export function authorizationDigest(value) { return createHash('sha256').update(encodeAuthorization(value)).digest('hex'); }
