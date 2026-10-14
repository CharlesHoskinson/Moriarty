/**
 * SameScriptBoundHead/2 identity.
 * Instance and genesis preimages are the measured unescaped JSON records.
 * A quote, backslash, or non-ASCII field is ENCODER_DISAGREEMENT.
 * This module does not create keys, start a node, or submit.
 */
import { createHash } from 'node:crypto';
import { canonicalEncode, hashDomain } from '../../moriarty-language/src/codec.ts';

export const INSTANCE_DOMAIN = 'MORIARTY-INSTANCE-financial-agreement/5';
export const GENESIS_DOMAIN = 'MORIARTY-GENESIS-financial-agreement/5';
export const GENESIS_VERSION = 'GenesisCommitment/1';
export const THREAD_NAME = '48656164';
export const CASH_NAME = '43617368';
export const NETWORK = 'yaci-isolated-t1';
export const DEPLOYMENT = 't1-custody-revision03';
export const ZERO28 = '00'.repeat(28);
export const ZERO32 = '00'.repeat(32);
export const SHAPE28 = [
  '24ba87f0a8b4f617d991e86e7a84fc3d4d90855dae7b2ce94e451a79',
  '1d038a1069d649adf83745f02ff5c84575e32839194593263b0a2855',
  'b39aa2a57e9c3730b8234ada7592743d512edca5c23f60ba03c9dd4b',
  '72f72192639eb173fce5342d2df730b73a0d838c48d2422ebf91e48c',
  '64b8d588ca2e8207014edd458014a7b49007f8c01fab78ff809925fe',
];
export const SHAPE_INSTANCE = 'f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d4';
const HEX28 = /^[0-9a-f]{56}$/;
const HEX32 = /^[0-9a-f]{64}$/;
const DECIMAL = /^(0|[1-9][0-9]*)$/;

function refuse(code, detail) {
  return { ok: false, code, detail };
}

function rawField(value, label) {
  if (typeof value !== 'string' || value.length === 0) return refuse('ENCODER_DISAGREEMENT', label);
  if (!/^[\x20-\x7e]+$/.test(value) || value.includes('"') || value.includes('\\')) {
    return refuse('ENCODER_DISAGREEMENT', label);
  }
  return { ok: true, value };
}

export function unescapedJson(record) {
  if (record === null || typeof record !== 'object' || Array.isArray(record)) return refuse('ENCODER_DISAGREEMENT', 'record');
  const keys = Object.keys(record).sort();
  const parts = [];
  for (const key of keys) {
    const keyOk = rawField(key, `key:${key}`);
    if (!keyOk.ok) return keyOk;
    const valueOk = rawField(record[key], key);
    if (!valueOk.ok) return valueOk;
    parts.push(`${JSON.stringify(key)}:${JSON.stringify(valueOk.value)}`);
  }
  const json = `{${parts.join(',')}}`;
  let canonical;
  try {
    canonical = canonicalEncode(record);
  } catch (error) {
    return refuse(error.code || 'ENCODER_DISAGREEMENT', error.message);
  }
  if (canonical !== json) return refuse('ENCODER_DISAGREEMENT', 'canonical json differs from the unescaped record');
  return { ok: true, json };
}

function domainDigest(domain, record) {
  const encoded = unescapedJson(record);
  if (!encoded.ok) return encoded;
  const manual = createHash('sha256')
    .update(Buffer.from(domain, 'utf8'))
    .update(Buffer.from([0]))
    .update(Buffer.from(encoded.json, 'utf8'))
    .digest('hex');
  let viaHashDomain;
  try {
    viaHashDomain = hashDomain(domain, record);
  } catch (error) {
    return refuse(error.code || 'ENCODER_DISAGREEMENT', error.message);
  }
  if (viaHashDomain !== manual) return refuse('ENCODER_DISAGREEMENT', 'hashDomain differs from the unescaped preimage');
  const preimage = Buffer.concat([
    Buffer.from(domain, 'utf8'),
    Buffer.from([0]),
    Buffer.from(encoded.json, 'utf8'),
  ]);
  return { ok: true, digest: manual, preimage, json: encoded.json };
}

export function roleClean(id) {
  return typeof id === 'string' && HEX28.test(id) && id !== ZERO28 && !SHAPE28.includes(id);
}

export function rolesOk(own, cash, lender, borrower) {
  return roleClean(own) && roleClean(cash) && roleClean(lender) && roleClean(borrower)
    && cash !== own && lender !== own && borrower !== own
    && lender !== borrower && lender !== cash && borrower !== cash;
}

export function instanceRecord(threadPolicyId, cashPolicyId) {
  return {
    cashAssetNameHex: CASH_NAME,
    cashPolicyId,
    domainDeployment: DEPLOYMENT,
    domainNetwork: NETWORK,
    threadAssetNameHex: THREAD_NAME,
    threadPolicyId,
  };
}

export function instancePreimage(threadPolicyId, cashPolicyId) {
  if (!HEX28.test(String(threadPolicyId || '')) || !HEX28.test(String(cashPolicyId || ''))) {
    return refuse('MALFORMED_ID', 'instance policy');
  }
  const built = domainDigest(INSTANCE_DOMAIN, instanceRecord(threadPolicyId, cashPolicyId));
  if (!built.ok) return built;
  return built;
}

export function instanceDigest(threadPolicyId, cashPolicyId) {
  const built = instancePreimage(threadPolicyId, cashPolicyId);
  if (!built.ok) return built;
  if (!HEX32.test(built.digest) || built.digest === ZERO32 || built.digest === SHAPE_INSTANCE) {
    return refuse('SHAPE_SAMPLE_FORBIDDEN', 'instanceId');
  }
  return { ok: true, instanceId: built.digest, preimage: built.preimage, json: built.json };
}

export function genesisRecord({ hashes, cashPolicyId, lender, borrower, seedTxId, seedIndex }) {
  return {
    borrowerKeyHash: borrower,
    boundsHash: hashes.boundsHash,
    cashAssetNameHex: CASH_NAME,
    cashPolicyId,
    domainDeployment: DEPLOYMENT,
    domainNetwork: NETWORK,
    lenderKeyHash: lender,
    programHash: hashes.programHash,
    schemaHash: hashes.schemaHash,
    seedHash: hashes.seedHash,
    seedOutRefIndex: String(seedIndex),
    seedOutRefTxId: seedTxId,
    sourceHash: hashes.sourceHash,
    threadAssetNameHex: THREAD_NAME,
    version: GENESIS_VERSION,
  };
}

export function genesisDigest({ hashes, cashPolicyId, lender, borrower, seedTxId, seedIndex }) {
  if (!hashes || typeof hashes !== 'object') return refuse('MALFORMED_DATA', 'hashes');
  for (const key of ['boundsHash', 'programHash', 'schemaHash', 'seedHash', 'sourceHash']) {
    if (!HEX32.test(String(hashes[key] || ''))) return refuse('MALFORMED_ID', key);
  }
  if (!HEX28.test(String(cashPolicyId || '')) || !roleClean(lender) || !roleClean(borrower)) {
    return refuse('MALFORMED_ID', 'genesis role');
  }
  if (!HEX32.test(String(seedTxId || '')) || seedTxId === ZERO32) return refuse('ALL_ZERO_FORBIDDEN', 'seedOutRefTxId');
  const index = String(seedIndex);
  if (!DECIMAL.test(index)) return refuse('NON_CANONICAL_VALUE', 'seedOutRefIndex');
  const built = domainDigest(GENESIS_DOMAIN, genesisRecord({ hashes, cashPolicyId, lender, borrower, seedTxId, seedIndex: index }));
  if (!built.ok) return built;
  return { ok: true, genesis: built.digest, preimage: built.preimage, json: built.json };
}

function definiteInt(value) {
  const n = BigInt(value);
  if (n < 0n || n > 0xffffffffn) return null;
  if (n < 24n) return Buffer.from([Number(n)]);
  if (n < 256n) return Buffer.from([0x18, Number(n)]);
  if (n < 65536n) return Buffer.from([0x19, Number(n) >> 8, Number(n) & 0xff]);
  return Buffer.from([0x1a, Number((n >> 24n) & 0xffn), Number((n >> 16n) & 0xffn), Number((n >> 8n) & 0xffn), Number(n & 0xffn)]);
}

function definiteBytes(hex) {
  const raw = Buffer.from(hex, 'hex');
  if (raw.length !== hex.length / 2) return null;
  if (raw.length < 24) return Buffer.concat([Buffer.from([0x40 + raw.length]), raw]);
  if (raw.length < 256) return Buffer.concat([Buffer.from([0x58, raw.length]), raw]);
  return null;
}

/** Definite canonical Data. OutputReference is constr 0. Byte arrays are definite. */
export function definiteDataHex(kind, value) {
  if (kind === 'bytes') {
    const encoded = definiteBytes(value);
    if (!encoded) return refuse('NON_CANONICAL_VALUE', 'bytes');
    return { ok: true, hex: encoded.toString('hex') };
  }
  if (kind === 'output-reference') {
    const tx = definiteBytes(value.transactionId);
    const index = definiteInt(value.index);
    if (!tx || !index || !HEX32.test(value.transactionId)) return refuse('NON_CANONICAL_VALUE', 'output-reference');
    const body = Buffer.concat([Buffer.from([0x82]), tx, index]);
    const encoded = Buffer.concat([Buffer.from([0xd8, 0x79]), body]);
    return { ok: true, hex: encoded.toString('hex') };
  }
  return refuse('NON_CANONICAL_VALUE', kind);
}
