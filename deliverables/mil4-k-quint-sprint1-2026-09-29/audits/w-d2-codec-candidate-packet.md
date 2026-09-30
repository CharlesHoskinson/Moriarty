Independent read-only review of Moriarty PROVISIONAL W-D2 `/3` signed authorization codec. Review exact embedded bytes only. This is one review seat; do not use tools, skills, delegation, external pages or workspace files. Find concrete high/medium defects in canonical encoding/decoding, field/tag closure, limits, signed scope completeness, digest fixtures and evidence claims. Give separate verdicts for a useful bounded prototype and for W-D2 normative closure. Do not infer wallet signature interoperability, source/Core/K/Quint correspondence, native proof or ledger acceptance from codec tests.

## Manifest

```json
[
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/WIRE-DECISION-RESEARCH.md",
    "bytes": 4880,
    "sha256": "df59ccfbbfc582b44f0d7b07f9248f1db05a14c8d74a69cb04bab68b17a4bf6f"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/SPEC.md",
    "bytes": 7872,
    "sha256": "b95b77085d8eb8998bf7a48837f40d36b7c1575d89450b4803ee4b9e4cd8e08e"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.mjs",
    "bytes": 8451,
    "sha256": "23075dfb5a6dc63fa700af1416ad8d5ec9d7fb5756ca4b18fa024ccdc7d3a811"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.test.mjs",
    "bytes": 8249,
    "sha256": "92382f2bb4315577d9f6f944e0b1d8f80344ab1f59089bf0c4906b28efa030b3"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/fixtures.json",
    "bytes": 13286,
    "sha256": "9525457d8bcec97dc133b05b9799a11b90a3be87bfe7525809febe688708bd55"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/record-results.mjs",
    "bytes": 2990,
    "sha256": "afa2c778ba6dc53b818494e68b1ab1ee59d7e21cfe9caba8f236b184bfe3ec8f"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/results.json",
    "bytes": 2346,
    "sha256": "c08648cd9d54163925b7eda8c6d9a7ee3878dd72da8738ffc2515a3d7125f2cb"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/RESULT.md",
    "bytes": 3364,
    "sha256": "64fa06cfa00ff793287c9eb34e14e4eec8eb3b28c2c5e5a2bbdccbab5e4542d4"
  }
]
```

## FILE deliverables/mil4-k-quint-sprint1-2026-09-29/WIRE-DECISION-RESEARCH.md (SHA-256 df59ccfbbfc582b44f0d7b07f9248f1db05a14c8d74a69cb04bab68b17a4bf6f)

````text
# W-D1/W-D2 recommendation from current primary sources

**Status:** research recommendation; no wire freeze, wallet interoperability result, verifier certificate or ledger acceptance result. W-D1 and W-D2 remain open.

## Verified interface facts

The pinned [Midnight DApp connector specification](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L371) requires `signData` to prepend `midnight_signed_message:<data_size>:` to the decoded byte payload. It describes Schnorr BIP340 and ECDSA signatures, with different key encodings, and treats an omitted scheme as Schnorr. It does not specify a maximum payload size. Its [encoding options](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L254) decode hex or base64 before signing.

The [ledger WASM API](https://docs.midnight.network/api-reference/ledger/functions/verifySignature) exposes `verifySignature` for arbitrary bytes. The [pinned implementation](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/onchain-runtime-wasm/src/primitives.rs#L93) deserializes Schnorr keys and signatures. The ledger [Schnorr wrapper](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/base-crypto/src/schnorr.rs#L142) calls k256 verification; pinned k256 0.13.4 [SHA-256 hashes the supplied message](https://github.com/RustCrypto/elliptic-curves/blob/5ac8f5d77f11399ff48d87b0554935f6eddda342/k256/src/schnorr/verifying.rs#L111) before raw BIP340 verification. The [BIP340 specification](https://github.com/bitcoin/bips/blob/3a10b5b5f0a7586df8928d580a3009744ebb2079/bip-0340.mediawiki#messages-of-arbitrary-size) does not itself require that extra application prehash.

These facts establish a host API and a possible preprocessing rule. They do not establish that a current wallet returns a compatible signature or that a consensus contract can enforce this arbitrary-message check as part of Moriarty acceptance.

## Recommended candidate for review

Use one S0 Schnorr profile. Let `C` be the eventual canonical `/3` authorization bytes; let `D = SHA256(C)` be exactly 32 bytes. For a connector `signData` call with hex-encoded `D`, define `M = ASCII("midnight_signed_message:32:") || D`. The **candidate** verification predicate is raw `BIP340.Verify(key32, SHA256(M), signature64)`, equivalent to the inspected k256 host wrapper when called with `M`. Reject explicit ECDSA or unknown schemes in this profile; normalize an omitted scheme to Schnorr only where the connector version requires it.

This candidate needs an actual wallet signature vector and host-verifier result before adoption. The proof public statement and ledger consumer must bind the same `D`, signer key, selected program, predecessor, replay key and complete effect commitment. The proof must constrain `D` as the digest of the exact signed authorization bytes. A caller supplied authentication Boolean is insufficient. If no consensus enforced host hook exists, retain an external premise or choose and certify an in-circuit verifier.

For `/3`, recommend a closed typed binary encoding with a literal domain header, numeric tags, fixed field order, fixed-width UInt128 atomic amounts, length-prefixed identifiers, explicit absent/empty values and canonical ordering of set-valued collections. Preserve ordered effect lines. Reject unknown tags, duplicate or unordered sets, omitted required leaves, trailing bytes and out-of-cap fields. Source/6, Core/5 and signed `/3` identities remain independent version axes; `/4` migration requires a separate authenticated relation. The complete tag table, finite caps and codec are still to be written and reviewed.

## Required experiments before a decision vote

1. Obtain a wallet `signData` signature over the candidate 32-byte `D`. Independently verify against `M`, `SHA256(M)`, `D` and `SHA256(D)` to identify the actual preprocessing. Check decoded 32-byte length against 64 hex characters and reject wrong or duplicated prefixes.
2. Hold the signature fixed while changing one signed recipient byte, fee cap, proof-public digest, signer key, predecessor, replay value or effect commitment. Every substitution must reject with no accepted effects.
3. Run codec round trips and independent noncanonical controls: unknown tag, omitted empty, duplicate collection item, alternate order, trailing bytes, cap+1 and wrong scheme/key length.
4. Demonstrate an acceptance consumer that fails when the host signature check is bypassed, or retain authentication as a named external premise. Pin the exact wallet, ledger and k256 versions used in that experiment.

No experiment in this note has yet run. This recommendation does not close W-D1 or W-D2.

````

## FILE experiments/moriarty-language/formal/mil4/wire/SPEC.md (SHA-256 b95b77085d8eb8998bf7a48837f40d36b7c1575d89450b4803ee4b9e4cd8e08e)

````text
# PROVISIONAL `/3` S0 signed authorization codec

**Candidate only.** This isolated wire experiment does not freeze W-D1/W-D2, migrate `/3` to `/4`, or implement a source/Core/ledger consumer. The caps below are experimental admission choices, not measured wallet or network limits.

## Bytes and primitive types

Header is 18 bytes: the 17 ASCII bytes `moriarty-intent/3` followed by one NUL. Every subsequent field starts with its one-byte tag, in the exact order below. There are no optional fields. No generic extension tag, padding or trailing byte is admitted. Binary input must be a Uint8Array of at most 4096 bytes; tag order is part of canonicality.

`id` is a UInt16 big-endian byte count followed by 1–64 ASCII bytes matching `[A-Za-z0-9][A-Za-z0-9._:/-]*`. There is no Unicode normalization, NUL, case folding or whitespace trimming. IDs are nominal and distinct fields preserve distinct sorts; the codec does not authenticate identifier ownership.

`hash32` and `key32` are exactly 32 raw bytes. Their JSON presentation is exactly 64 lowercase hex characters, without `0x`. A key's length is checked; membership on secp256k1 is an external cryptographic check.

`u64` and `nominal` are respectively 8 and 16 big-endian bytes. Their JSON presentation is a canonical nonnegative decimal string: `0` or a nonzero digit followed by digits. u64 cap is `2^64−1`; nominal cap is `2^127−1`. Negative numbers, numeric JS values, leading zeroes, decimal points, exponent notation and cap+1 reject. Fixed width prohibits redundant integer encodings. An operation amount must be positive. This codec does not check fee/gross/net arithmetic or balances.

`scale` is one unsigned byte, cap 38. `empty` is an explicit UInt16 count equal to zero and presents as `[]`; any element/count rejects. Fixed literal fields present as the exact strings/numbers in the table. Unknown object keys, missing fields, inherited required fields and unknown operation keys reject; property insertion order does not affect encoded bytes.

## Ordered common fields

| Tag | JSON field | Type or exact encoded value |
| --- | --- | --- |
| 1 | profile | `s0-provisional/1`, byte `01` |
| 2 | domain | id |
| 3 | agreementId | id |
| 4 | stageId | id |
| 5 | episodeId | id |
| 6 | actionId | id |
| 7 | sourceVersion | number 6, byte `06` |
| 8 | sourceHash | hash32 |
| 9 | coreVersion | number 5, byte `05` |
| 10 | coreProgramId | id |
| 11 | coreHash | hash32 |
| 12 | policyHash | hash32 |
| 13 | signer | id |
| 14 | keyScheme | `schnorr_bip340`, byte `01` |
| 15 | signerKey | key32 |
| 16 | asset | id |
| 17 | scale | scale |
| 18 | preHead | hash32 |
| 19 | predecessor | hash32 |
| 20 | nonce | hash32 |
| 21 | validFrom | u64 |
| 22 | validUntil | u64; must be ≥validFrom |
| 23 | grossCap | nominal |
| 24 | feeCap | nominal |
| 25 | netFloor | nominal |
| 26 | effectCommitment | hash32, supplied commitment |
| 27 | failurePolicy | `atomic-reject-terminal-success`, byte `01` |
| 28 | supplyChanges | empty |
| 29 | observations | empty |
| 30 | disclosures | empty |
| 31 | retainedEffects | empty |
| 32 | retainedDuties | empty |
| 33 | delegation | `none`, byte `00` |
| 34 | recovery | `none`, byte `00` |
| 35 | operation | variant below |

The JSON field `schemaVersion` is mandatory and exactly `moriarty-intent/3`; the header encodes it. Domain is explicitly signed. Exactly one signer key, one domain and one settlement asset exist in this record. Validity is a closed interval of rounds in that domain; conversion to clock time is not implemented. Replay identity is the tuple `(domain, signer, nonce)`; nonce is independent of the intent digest. Authenticating predecessor/head or consuming replay is outside this codec.

## Operation variants

After tag 35, operation kind byte `01` means `{kind:'transfer', owner:id, recipient:id, feeRecipient:id, amount:nominal, fee:nominal}` in that order. All endpoints, fee and principal are signed, including feeRecipient when fee=0. Alias rejection and zero-fee effect policy belong to the semantic consumer.

Kind byte `02` means `{kind:'repayment', obligationId:id, payer:id, debtor:id, creditor:id, amount:nominal, allocation:'AccrualFirst', conversion:'identity'}` in that order. The last two literals encode as bytes `01`, `01`. This admits a reference to an existing obligation only: obligation opening, status, debtor/creditor/asset correspondence and funded reduction require an authenticated consumer. Identity means mantissa=1, scale=0, rounding=none; the asset's signed atomic-unit scale remains the common scale field.

## Digest and effect commitment

The candidate content digest is SHA-256 of the complete canonical record bytes, including the `/3` header. `authorizationDigest` returns its lowercase 64-character hex presentation. No wallet prefix, prehash wrapper, signature, signature-valid flag, proof or transaction is part of this codec.

`effectCommitment` is an opaque 32-byte commitment supplied before encoding. The intended producer commits the complete prepared effect vector and required consumption/footprint information under a separately selected effect encoding. That vector must not include this authorization digest, this commitment, a signature, or a post-head defined from this authorization digest. Thus the dependency is `authenticated pre-state + operation → prepared effects → supplied effectCommitment → authorization bytes → digest`; no codec operation feeds the digest back into effects. The fixture commitments are illustrative byte strings, not authenticated effects. Selecting the effect encoding and proving commitment equality are explicitly unimplemented.

The consumer must prove sourceHash/Core identity/policy correspondence, derive effects from these signed terms at preHead, bind effectCommitment to those effects and bind the same digest across proof and acceptance. No such binding follows from a round trip.

## Stable prototype rejection codes

`SHAPE` rejects missing/extra/inherited fields or wrong object/byte-input types; `LITERAL` fixed-version/profile/scheme/failure/allocation/conversion mismatch; `ID` noncanonical alphabet/empty ID; `LENGTH` ID or record cap; `HEX` noncanonical key/hash presentation; `INTEGER` noncanonical decimal presentation; `RANGE` numeric/scale cap or zero operation amount; `VALIDITY` reversed interval; `EMPTY` nonempty deferred field; `HEADER` wrong header; `FIELD_TAG` missing/out-of-order/unknown field tag; `OPERATION_TAG` unknown variant; `TRUNCATED` incomplete payload; `TRAILING` excess bytes. Object shape precedes primitive validation; primitive fields follow the fixed order. Validity is checked after validUntil; positive amount is checked after the entire operation. Decoder record type/size/header checks precede field decoding. These are codec errors, not W-D3 six-judgment codes.

## Evidence and unverified loci

The [current connector specification](https://github.com/midnightntwrk/midnight-dapp-connector-api/blob/612db2b62dbd78c079da62e57e7b8585a204b858/docs/api/_media/SPECIFICATION.md#L371) requires a signed-message prefix and advertises Schnorr/ECDSA. The [ledger host verifier](https://github.com/midnightntwrk/midnight-ledger/blob/9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8/onchain-runtime-wasm/src/primitives.rs#L93) uses Schnorr, whose [k256 preprocessing](https://github.com/RustCrypto/elliptic-curves/blob/5ac8f5d77f11399ff48d87b0554935f6eddda342/k256/src/schnorr/verifying.rs#L111) needs an explicit interoperability discriminator. This prototype does not implement those paths. Its raw key bytes are not the ledger's tagged serialization.

Canonical Source/6 lowering, Core/5 round trips, semantic execution, SHA-256 circuit cost, native certificate, wallet/signature interoperability, on-chain verification, effect correspondence, cap feasibility and final W-D2 votes all remain open.

````

## FILE experiments/moriarty-language/formal/mil4/wire/codec.mjs (SHA-256 23075dfb5a6dc63fa700af1416ad8d5ec9d7fb5756ca4b18fa024ccdc7d3a811)

````text
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
function fail(code, field) { const error = new Error(`${code}: ${field}`); error.code = code; throw error; }
function shape(value, names, field) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE', field);
  const keys = Reflect.ownKeys(value);
  if (keys.length !== names.length || keys.some(key => typeof key !== 'string' || !names.includes(key))) fail('SHAPE', field);
  for (const name of names) {
    const descriptor = Object.getOwnPropertyDescriptor(value, name);
    if (!descriptor || !('value' in descriptor) || !descriptor.enumerable) fail('SHAPE', `${field}.${name}`);
  }
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
      if (typeof value !== 'string' || !/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
      const data = Buffer.from(value, 'ascii');
      if (data.length > CAPS.identifierBytes) fail('LENGTH', name);
      return Buffer.concat([uintBytes(BigInt(data.length), 2), data]);
    }
    case 'hex': if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) fail('HEX', name); return Buffer.from(value, 'hex');
    case 'u64': case 'nominal': {
      if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER', name);
      // Bound decimal length before BigInt conversion, including adversarial text.
      if (value.length > (type === 'u64' ? 20 : 39)) fail('RANGE', name);
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
  if (descriptor.value === 'transfer') return [1, transfer];
  if (descriptor.value === 'repayment') return [2, repayment];
  fail('OPERATION_TAG', 'operation.kind');
}
function encodeOperation(value) {
  const [tag, schema] = operationFields(value);
  shape(value, ['kind', ...schema.map(([name]) => name)], 'operation');
  const parts = [Buffer.from([tag])];
  for (const [name, type, literal, byte] of schema) parts.push(encodePrimitive(value[name], type, `operation.${name}`, literal, byte));
  if (value.amount === '0') fail('RANGE', 'operation.amount');
  return Buffer.concat(parts);
}
export function encodeAuthorization(value) {
  shape(value, ['schemaVersion', ...fields.map(([name]) => name)], 'authorization');
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
  if (!(bytes instanceof Uint8Array)) fail('SHAPE', 'bytes');
  if (bytes.byteLength > CAPS.recordBytes) fail('LENGTH', 'bytes');
  const reader = new Reader(Buffer.from(bytes));
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

````

## FILE experiments/moriarty-language/formal/mil4/wire/codec.test.mjs (SHA-256 92382f2bb4315577d9f6f944e0b1d8f80344ab1f59089bf0c4906b28efa030b3)

````text
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const { fixtures } = JSON.parse(await readFile(new URL('./fixtures.json', import.meta.url)));
let codec;
try { codec = await import('./codec.mjs'); }
catch (error) { if (error.code !== 'ERR_MODULE_NOT_FOUND') throw error; codec = {}; }
const encode = (...args) => { assert.equal(typeof codec.encodeAuthorization, 'function', 'candidate encoder is not implemented'); return codec.encodeAuthorization(...args); };
const decode = (...args) => { assert.equal(typeof codec.decodeAuthorization, 'function', 'candidate decoder is not implemented'); return codec.decodeAuthorization(...args); };
const digest = (...args) => { assert.equal(typeof codec.authorizationDigest, 'function', 'candidate digest is not implemented'); return codec.authorizationDigest(...args); };
const transfer = fixtures[0];
const fresh = () => structuredClone(transfer.authorization);
const wire = () => Buffer.from(transfer.expected.wireHex, 'hex');
const at = (name, part = 'value') => transfer.expected.fieldOffsets[name][part];
const rejects = (fn, code) => assert.throws(fn, error => error.code === code, `expected ${code}`);
const U64 = (2n ** 64n - 1n).toString();
const S = (2n ** 127n - 1n).toString();

for (const fixture of fixtures) {
  test(`${fixture.id}: encoder matches independently assembled exact bytes`, () => assert.equal(encode(fixture.authorization).toString('hex'), fixture.expected.wireHex));
  test(`${fixture.id}: decoder matches independently specified typed value`, () => assert.deepEqual(decode(Buffer.from(fixture.expected.wireHex, 'hex')), fixture.authorization));
  test(`${fixture.id}: canonical round trip and independent digest`, () => {
    assert.deepEqual(decode(encode(fixture.authorization)), fixture.authorization);
    assert.equal(digest(fixture.authorization), fixture.expected.digestHex);
  });
}
test('object insertion order cannot change signed bytes', () => assert.deepEqual(encode(Object.fromEntries(Object.entries(fresh()).reverse())), encode(fresh())));
test('explicit zero fee preserves its bound fee recipient', () => { const a = fresh(); a.operation.fee = '0'; assert.deepEqual(decode(encode(a)), a); });
test('maximum 64-byte nominal ID is admitted', () => { const a = fresh(); a.agreementId = 'a'.repeat(64); assert.deepEqual(decode(encode(a)), a); });
test('maximum numeric widths and scale are admitted', () => {
  const a = fresh(); a.validFrom = U64; a.validUntil = U64; a.grossCap = S; a.feeCap = S; a.netFloor = S; a.operation.amount = S; a.operation.fee = S; a.scale = 38;
  assert.deepEqual(decode(encode(a)), a); // formation only; semantic amount/cap relationships are external
});
for (const [name, mutate, code] of [
  ['unknown root property', a => { a.extra = true; }, 'SHAPE'],
  ['missing explicit empty', a => { delete a.observations; }, 'SHAPE'],
  ['unknown operation property', a => { a.operation.extra = 0; }, 'SHAPE'],
  ['wrong version', a => { a.schemaVersion = 'moriarty-intent/4'; }, 'LITERAL'],
  ['unknown profile', a => { a.profile = 'future'; }, 'LITERAL'],
  ['unknown scheme', a => { a.keyScheme = 'ecdsa_secp256k1_sha256'; }, 'LITERAL'],
  ['wrong source version', a => { a.sourceVersion = 5; }, 'LITERAL'],
  ['wrong core version', a => { a.coreVersion = 4; }, 'LITERAL'],
  ['unknown operation', a => { a.operation.kind = 'mint'; }, 'OPERATION_TAG'],
  ['ID cap+1', a => { a.domain = 'a'.repeat(65); }, 'LENGTH'],
  ['non-ASCII identifier', a => { a.signer = 'alïce'; }, 'ID'],
  ['empty identifier', a => { a.asset = ''; }, 'ID'],
  ['upper-case hex', a => { a.preHead = 'AA'.repeat(32); }, 'HEX'],
  ['wrong hash length', a => { a.effectCommitment = '88'.repeat(33); }, 'HEX'],
  ['wrong key length', a => { a.signerKey = '11'.repeat(31); }, 'HEX'],
  ['leading-zero decimal', a => { a.grossCap = '0110'; }, 'INTEGER'],
  ['numeric JS amount', a => { a.operation.amount = 100; }, 'INTEGER'],
  ['negative decimal', a => { a.feeCap = '-1'; }, 'INTEGER'],
  ['nominal cap+1', a => { a.grossCap = (2n ** 127n).toString(); }, 'RANGE'],
  ['u64 cap+1', a => { a.validUntil = (2n ** 64n).toString(); }, 'RANGE'],
  ['scale cap+1', a => { a.scale = 39; }, 'RANGE'],
  ['zero operation amount', a => { a.operation.amount = '0'; }, 'RANGE'],
  ['reversed validity', a => { a.validFrom = '201'; }, 'VALIDITY'],
  ['nonempty failure branch', a => { a.retainedEffects = ['fee']; }, 'EMPTY'],
  ['nonempty supply field', a => { a.supplyChanges = ['mint']; }, 'EMPTY'],
  ['nonempty observation field', a => { a.observations = ['price']; }, 'EMPTY'],
  ['nonempty disclosure field', a => { a.disclosures = ['secret']; }, 'EMPTY'],
  ['nonempty duty field', a => { a.retainedDuties = ['refund']; }, 'EMPTY'],
  ['delegation', a => { a.delegation = 'delegated'; }, 'LITERAL'],
  ['recovery', a => { a.recovery = 'recover'; }, 'LITERAL'],
  ['unselected failure policy', a => { a.failurePolicy = 'retain-fee'; }, 'LITERAL'],
]) test(`typed rejection: ${name}`, () => { const a = fresh(); mutate(a); rejects(() => encode(a), code); });
test('inherited required fields are rejected', () => { const a = fresh(); delete a.domain; Object.setPrototypeOf(a, { domain: 'midnight:preview' }); rejects(() => encode(a), 'SHAPE'); });
test('repayment allocation and conversion are closed', () => {
  for (const field of ['allocation', 'conversion']) { const a = structuredClone(fixtures[1].authorization); a.operation[field] = 'other'; rejects(() => encode(a), 'LITERAL'); }
});
for (const [name, mutate, code] of [
  ['unknown field tag', b => { b[at('domain', 'tag')] = 255; return b; }, 'FIELD_TAG'],
  ['duplicated field tag', b => { b[at('stageId', 'tag')] = 3; return b; }, 'FIELD_TAG'],
  ['wrong header', b => { b[0] = 0; return b; }, 'HEADER'],
  ['unknown profile byte', b => { b[at('profile')] = 2; return b; }, 'LITERAL'],
  ['unknown operation tag', b => { b[at('operation')] = 255; return b; }, 'OPERATION_TAG'],
  ['unknown scheme byte', b => { b[at('keyScheme')] = 2; return b; }, 'LITERAL'],
  ['nominal high bit', b => { b[at('grossCap')] = 128; return b; }, 'RANGE'],
  ['nonzero deferred count', b => { b.writeUInt16BE(1, at('observations')); return b; }, 'EMPTY'],
  ['ID cap+1 bytes', b => { b.writeUInt16BE(65, at('domain')); return b; }, 'LENGTH'],
  ['empty ID bytes', b => { b.writeUInt16BE(0, at('domain')); return b; }, 'ID'],
  ['non-ASCII ID bytes', b => { b[at('domain') + 2] = 255; return b; }, 'ID'],
  ['integer extra padding', b => Buffer.concat([b.subarray(0, at('grossCap')), Buffer.from([0]), b.subarray(at('grossCap'))]), 'FIELD_TAG'],
  ['trailing bytes', b => Buffer.concat([b, Buffer.from([0])]), 'TRAILING'],
  ['record cap+1', () => Buffer.alloc(4097), 'LENGTH'],
]) test(`wire rejection: ${name}`, () => rejects(() => decode(mutate(wire())), code));
test('every truncation of a valid transfer is rejected', () => { const b = wire(); for (let n = 0; n < b.length; n++) assert.throws(() => decode(b.subarray(0, n)), `accepted prefix length ${n}`); });
test('decoder rejects ordinary arrays as bytes', () => rejects(() => decode([...wire()]), 'SHAPE'));
for (const [name, mutate] of [
  ['recipient', a => { a.operation.recipient = 'eve'; }],
  ['fee recipient', a => { a.operation.feeRecipient = 'other'; }],
  ['fee cap one unit', a => { a.feeCap = '6'; }],
  ['domain', a => { a.domain = 'midnight:preprod'; }],
  ['source identity', a => { a.sourceHash = '01'.repeat(32); }],
  ['Core identity', a => { a.coreHash = '02'.repeat(32); }],
  ['head', a => { a.preHead = '03'.repeat(32); }],
  ['predecessor', a => { a.predecessor = '04'.repeat(32); }],
  ['replay nonce', a => { a.nonce = '05'.repeat(32); }],
  ['validity', a => { a.validUntil = '201'; }],
  ['signer key', a => { a.signerKey = '06'.repeat(32); }],
  ['effect commitment', a => { a.effectCommitment = '07'.repeat(32); }],
]) test(`content binding discriminator: ${name}`, () => { const a = fresh(); mutate(a); assert.notEqual(digest(a), transfer.expected.digestHex); });
test('repayment creditor substitution changes digest', () => { const a = structuredClone(fixtures[1].authorization); a.operation.creditor = 'eve'; assert.notEqual(digest(a), fixtures[1].expected.digestHex); });

````

## FILE experiments/moriarty-language/formal/mil4/wire/fixtures.json (SHA-256 9525457d8bcec97dc133b05b9799a11b90a3be87bfe7525809febe688708bd55)

````text
{
  "status": "PROVISIONAL; expected bytes assembled from SPEC.md before codec implementation; effect commitments illustrative",
  "fixtures": [
    {
      "id": "transfer",
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "midnight:preview",
        "agreementId": "agreement-1",
        "stageId": "stage-1",
        "episodeId": "episode-1",
        "actionId": "transfer-1",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "core-transfer-1",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "alice",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "asset:USD",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "110",
        "feeCap": "5",
        "netFloor": "100",
        "effectCommitment": "8888888888888888888888888888888888888888888888888888888888888888",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "transfer",
          "owner": "alice",
          "recipient": "bob",
          "feeRecipient": "fees",
          "amount": "100",
          "fee": "5"
        }
      },
      "expected": {
        "wireHex": "6d6f7269617274792d696e74656e742f330001010200106d69646e696768743a7072657669657703000b61677265656d656e742d3104000773746167652d31050009657069736f64652d3106000a7472616e736665722d31070608222222222222222222222222222222222222222222222222222222222222222209050a000f636f72652d7472616e736665722d310b33333333333333333333333333333333333333333333333333333333333333330c44444444444444444444444444444444444444444444444444444444444444440d0005616c6963650e010f111111111111111111111111111111111111111111111111111111111111111110000961737365743a55534411021255555555555555555555555555555555555555555555555555555555555555551366666666666666666666666666666666666666666666666666666666666666661477777777777777777777777777777777777777777777777777777777777777771500000000000000641600000000000000c8170000000000000000000000000000006e180000000000000000000000000000000519000000000000000000000000000000641a88888888888888888888888888888888888888888888888888888888888888881b011c00001d00001e00001f00002000002100220023010005616c6963650003626f620004666565730000000000000000000000000000006400000000000000000000000000000005",
        "digestHex": "722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22",
        "length": 540,
        "fieldOffsets": {
          "profile": {
            "tag": 18,
            "value": 19
          },
          "domain": {
            "tag": 20,
            "value": 21
          },
          "agreementId": {
            "tag": 39,
            "value": 40
          },
          "stageId": {
            "tag": 53,
            "value": 54
          },
          "episodeId": {
            "tag": 63,
            "value": 64
          },
          "actionId": {
            "tag": 75,
            "value": 76
          },
          "sourceVersion": {
            "tag": 88,
            "value": 89
          },
          "sourceHash": {
            "tag": 90,
            "value": 91
          },
          "coreVersion": {
            "tag": 123,
            "value": 124
          },
          "coreProgramId": {
            "tag": 125,
            "value": 126
          },
          "coreHash": {
            "tag": 143,
            "value": 144
          },
          "policyHash": {
            "tag": 176,
            "value": 177
          },
          "signer": {
            "tag": 209,
            "value": 210
          },
          "keyScheme": {
            "tag": 217,
            "value": 218
          },
          "signerKey": {
            "tag": 219,
            "value": 220
          },
          "asset": {
            "tag": 252,
            "value": 253
          },
          "scale": {
            "tag": 264,
            "value": 265
          },
          "preHead": {
            "tag": 266,
            "value": 267
          },
          "predecessor": {
            "tag": 299,
            "value": 300
          },
          "nonce": {
            "tag": 332,
            "value": 333
          },
          "validFrom": {
            "tag": 365,
            "value": 366
          },
          "validUntil": {
            "tag": 374,
            "value": 375
          },
          "grossCap": {
            "tag": 383,
            "value": 384
          },
          "feeCap": {
            "tag": 400,
            "value": 401
          },
          "netFloor": {
            "tag": 417,
            "value": 418
          },
          "effectCommitment": {
            "tag": 434,
            "value": 435
          },
          "failurePolicy": {
            "tag": 467,
            "value": 468
          },
          "supplyChanges": {
            "tag": 469,
            "value": 470
          },
          "observations": {
            "tag": 472,
            "value": 473
          },
          "disclosures": {
            "tag": 475,
            "value": 476
          },
          "retainedEffects": {
            "tag": 478,
            "value": 479
          },
          "retainedDuties": {
            "tag": 481,
            "value": 482
          },
          "delegation": {
            "tag": 484,
            "value": 485
          },
          "recovery": {
            "tag": 486,
            "value": 487
          },
          "operation": {
            "tag": 488,
            "value": 489
          },
          "operation.owner": {
            "value": 490
          },
          "operation.recipient": {
            "value": 497
          },
          "operation.feeRecipient": {
            "value": 502
          },
          "operation.amount": {
            "value": 508
          },
          "operation.fee": {
            "value": 524
          }
        }
      }
    },
    {
      "id": "repayment",
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "midnight:preview",
        "agreementId": "agreement-1",
        "stageId": "stage-1",
        "episodeId": "episode-1",
        "actionId": "repay-1",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "core-repay-1",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "alice",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "asset:USD",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "30",
        "effectCommitment": "9999999999999999999999999999999999999999999999999999999999999999",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "loan-1",
          "payer": "alice",
          "debtor": "alice",
          "creditor": "bank",
          "amount": "30",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "expected": {
        "wireHex": "6d6f7269617274792d696e74656e742f330001010200106d69646e696768743a7072657669657703000b61677265656d656e742d3104000773746167652d31050009657069736f64652d3106000772657061792d31070608222222222222222222222222222222222222222222222222222222222222222209050a000c636f72652d72657061792d310b33333333333333333333333333333333333333333333333333333333333333330c44444444444444444444444444444444444444444444444444444444444444440d0005616c6963650e010f111111111111111111111111111111111111111111111111111111111111111110000961737365743a55534411021255555555555555555555555555555555555555555555555555555555555555551366666666666666666666666666666666666666666666666666666666666666661477777777777777777777777777777777777777777777777777777777777777771500000000000000641600000000000000c8170000000000000000000000000000001e1800000000000000000000000000000000190000000000000000000000000000001e1a99999999999999999999999999999999999999999999999999999999999999991b011c00001d00001e00001f000020000021002200230200066c6f616e2d310005616c6963650005616c696365000462616e6b0000000000000000000000000000001e0101",
        "digestHex": "7de5d64b195c0f509e26622cd8f68ce993ae95a741cc82043cf387de344da673",
        "length": 530,
        "fieldOffsets": {
          "profile": {
            "tag": 18,
            "value": 19
          },
          "domain": {
            "tag": 20,
            "value": 21
          },
          "agreementId": {
            "tag": 39,
            "value": 40
          },
          "stageId": {
            "tag": 53,
            "value": 54
          },
          "episodeId": {
            "tag": 63,
            "value": 64
          },
          "actionId": {
            "tag": 75,
            "value": 76
          },
          "sourceVersion": {
            "tag": 85,
            "value": 86
          },
          "sourceHash": {
            "tag": 87,
            "value": 88
          },
          "coreVersion": {
            "tag": 120,
            "value": 121
          },
          "coreProgramId": {
            "tag": 122,
            "value": 123
          },
          "coreHash": {
            "tag": 137,
            "value": 138
          },
          "policyHash": {
            "tag": 170,
            "value": 171
          },
          "signer": {
            "tag": 203,
            "value": 204
          },
          "keyScheme": {
            "tag": 211,
            "value": 212
          },
          "signerKey": {
            "tag": 213,
            "value": 214
          },
          "asset": {
            "tag": 246,
            "value": 247
          },
          "scale": {
            "tag": 258,
            "value": 259
          },
          "preHead": {
            "tag": 260,
            "value": 261
          },
          "predecessor": {
            "tag": 293,
            "value": 294
          },
          "nonce": {
            "tag": 326,
            "value": 327
          },
          "validFrom": {
            "tag": 359,
            "value": 360
          },
          "validUntil": {
            "tag": 368,
            "value": 369
          },
          "grossCap": {
            "tag": 377,
            "value": 378
          },
          "feeCap": {
            "tag": 394,
            "value": 395
          },
          "netFloor": {
            "tag": 411,
            "value": 412
          },
          "effectCommitment": {
            "tag": 428,
            "value": 429
          },
          "failurePolicy": {
            "tag": 461,
            "value": 462
          },
          "supplyChanges": {
            "tag": 463,
            "value": 464
          },
          "observations": {
            "tag": 466,
            "value": 467
          },
          "disclosures": {
            "tag": 469,
            "value": 470
          },
          "retainedEffects": {
            "tag": 472,
            "value": 473
          },
          "retainedDuties": {
            "tag": 475,
            "value": 476
          },
          "delegation": {
            "tag": 478,
            "value": 479
          },
          "recovery": {
            "tag": 480,
            "value": 481
          },
          "operation": {
            "tag": 482,
            "value": 483
          },
          "operation.obligationId": {
            "value": 484
          },
          "operation.payer": {
            "value": 492
          },
          "operation.debtor": {
            "value": 499
          },
          "operation.creditor": {
            "value": 506
          },
          "operation.amount": {
            "value": 512
          }
        }
      }
    }
  ]
}

````

## FILE experiments/moriarty-language/formal/mil4/wire/record-results.mjs (SHA-256 afa2c778ba6dc53b818494e68b1ab1ee59d7e21cfe9caba8f236b184bfe3ec8f)

````text
// Reproduce only this directory's candidate tests; no financial/native claims.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { encodeAuthorization, authorizationDigest } from './codec.mjs';

const dir = new URL('./', import.meta.url);
const cwd = fileURLToPath(new URL('../../../../../', import.meta.url));
const testPath = 'experiments/moriarty-language/formal/mil4/wire/codec.test.mjs';
const args = ['--test', '--test-reporter=tap', testPath];
const started = new Date().toISOString();
const run = spawnSync(process.execPath, args, { cwd, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
const output = (run.stdout ?? '') + (run.stderr ?? '');
await writeFile(new URL('test-output.tap', dir), output);
const counts = {};
for (const name of ['tests', 'pass', 'fail', 'cancelled', 'skipped', 'todo']) {
  const match = output.match(new RegExp(`^# ${name} (\\d+)$`, 'm'));
  counts[name] = match ? Number(match[1]) : null;
}
const { fixtures } = JSON.parse(await readFile(new URL('fixtures.json', dir)));
const comparisons = fixtures.map(fixture => ({
  id: fixture.id,
  encodedBytes: encodeAuthorization(fixture.authorization).length,
  exactBytesMatch: encodeAuthorization(fixture.authorization).toString('hex') === fixture.expected.wireHex,
  digest: authorizationDigest(fixture.authorization),
  independentDigestMatch: authorizationDigest(fixture.authorization) === fixture.expected.digestHex,
}));
const names = ['SPEC.md', 'PLAN.md', 'codec.mjs', 'codec.test.mjs', 'fixtures.json', 'record-results.mjs', 'RESULT.md', 'red-output.tap', 'test-output.tap'];
const artifacts = {};
for (const name of names) artifacts[name] = createHash('sha256').update(await readFile(new URL(name, dir))).digest('hex');
const result = {
  status: 'PROVISIONAL_W_D2_CODEC_EXPERIMENT', startedAt: started, finishedAt: new Date().toISOString(),
  cwd, command: [process.execPath, ...args], runtime: process.version, platform: process.platform,
  exitCode: run.status, signal: run.signal, spawnError: run.error?.message ?? null,
  counts, fixtureComparisons: comparisons, artifacts,
  acceptedClaims: ['targeted canonical byte/digest fixture agreement', 'targeted round-trip and hostile codec outcomes'],
  unverified: ['wallet/signature interoperability', 'effect commitment correspondence', 'Source/Core/K/Quint correspondence', 'native proof/hash binding', 'ledger verification and atomic acceptance', 'cap feasibility', 'W-D1/W-D2 normative closure'],
};
await writeFile(new URL('results.json', dir), JSON.stringify(result, null, 2) + '\n');
const passed = run.status === 0 && counts.fail === 0 && counts.tests === 72 && comparisons.every(value => value.exactBytesMatch && value.independentDigestMatch);
console.log(JSON.stringify({ passed, exitCode: run.status, counts, fixtureComparisons: comparisons }, null, 2));
if (!passed) process.exitCode = 1;

````

## FILE experiments/moriarty-language/formal/mil4/wire/results.json (SHA-256 c08648cd9d54163925b7eda8c6d9a7ee3878dd72da8738ffc2515a3d7125f2cb)

````text
{
  "status": "PROVISIONAL_W_D2_CODEC_EXPERIMENT",
  "startedAt": "2026-09-30T05:47:54.965Z",
  "finishedAt": "2026-09-30T05:47:55.267Z",
  "cwd": "/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/",
  "command": [
    "/home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node",
    "--test",
    "--test-reporter=tap",
    "experiments/moriarty-language/formal/mil4/wire/codec.test.mjs"
  ],
  "runtime": "v24.21.0",
  "platform": "linux",
  "exitCode": 0,
  "signal": null,
  "spawnError": null,
  "counts": {
    "tests": 72,
    "pass": 72,
    "fail": 0,
    "cancelled": 0,
    "skipped": 0,
    "todo": 0
  },
  "fixtureComparisons": [
    {
      "id": "transfer",
      "encodedBytes": 540,
      "exactBytesMatch": true,
      "digest": "722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22",
      "independentDigestMatch": true
    },
    {
      "id": "repayment",
      "encodedBytes": 530,
      "exactBytesMatch": true,
      "digest": "7de5d64b195c0f509e26622cd8f68ce993ae95a741cc82043cf387de344da673",
      "independentDigestMatch": true
    }
  ],
  "artifacts": {
    "SPEC.md": "b95b77085d8eb8998bf7a48837f40d36b7c1575d89450b4803ee4b9e4cd8e08e",
    "PLAN.md": "0d79005cb69e841bef630e8f8da33c3f61ae94ea530593339ea305fee944833e",
    "codec.mjs": "23075dfb5a6dc63fa700af1416ad8d5ec9d7fb5756ca4b18fa024ccdc7d3a811",
    "codec.test.mjs": "92382f2bb4315577d9f6f944e0b1d8f80344ab1f59089bf0c4906b28efa030b3",
    "fixtures.json": "9525457d8bcec97dc133b05b9799a11b90a3be87bfe7525809febe688708bd55",
    "record-results.mjs": "afa2c778ba6dc53b818494e68b1ab1ee59d7e21cfe9caba8f236b184bfe3ec8f",
    "RESULT.md": "64fa06cfa00ff793287c9eb34e14e4eec8eb3b28c2c5e5a2bbdccbab5e4542d4",
    "red-output.tap": "235d33174018a6cf8bac764104baaf82f0cef9a9e8f776929c9fc97c28c68d39",
    "test-output.tap": "d0e3a934570019efa833c9c93fac5d084805a5dd9320917ddcabeb20fd05acf9"
  },
  "acceptedClaims": [
    "targeted canonical byte/digest fixture agreement",
    "targeted round-trip and hostile codec outcomes"
  ],
  "unverified": [
    "wallet/signature interoperability",
    "effect commitment correspondence",
    "Source/Core/K/Quint correspondence",
    "native proof/hash binding",
    "ledger verification and atomic acceptance",
    "cap feasibility",
    "W-D1/W-D2 normative closure"
  ]
}

````

## FILE experiments/moriarty-language/formal/mil4/wire/RESULT.md (SHA-256 64fa06cfa00ff793287c9eb34e14e4eec8eb3b28c2c5e5a2bbdccbab5e4542d4)

````text
# PROVISIONAL W-D2 codec experiment

This candidate implements a separate `/3` binary authorization codec for one-signer S0 transfer and existing-obligation AccrualFirst repayment. All prototype files are new and confined to this directory. It does not adopt a wire profile or close W-D2.

Observed targeted result: 72 tests passed, zero failed. The [machine result](results.json) records the exact runtime, invocation, exit code, fixture comparisons and SHA-256 artifact hashes. [Complete current TAP](test-output.tap) and the [first failing TAP](red-output.tap) preserve the red/green observations. First red run: 71 failures and one passing rejection-only test because the codec functions were absent; missing APIs failed assertions rather than a module-loader error. This does not assert that every later internal helper had its own separate red cycle.

The two expected byte vectors were assembled from the spec with a separate Python implementation before the Node codec existed. Transfer is 540 bytes with digest `722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22`; repayment is 530 bytes with digest `7de5d64b195c0f509e26622cd8f68ce993ae95a741cc82043cf387de344da673`. The independent fixture digest uses Python hashlib.sha256; the candidate uses built-in Node crypto. This is bounded experimental agreement, not a formal uniqueness or correspondence proof.

Controls cover exact fixture bytes, typed decode, round trips, object key order, zero fee, maximum admitted ID/numeric widths/scale, unknown/missing properties, unknown versions/tags/schemes, duplicate field tags, unknown repayment literals, noncanonical ASCII/hex/decimal representations, cap+1 values, nonempty deferred fields, extra integer padding, trailing bytes, record cap+1 and every truncated prefix of the valid transfer. Content mutations change digests for recipients, fee cap, domain, source/Core hashes, head/predecessor/nonce/validity, signer key and effect commitment. No test verifies a signature or financial acceptance.

Exact commands from the checkout root:

```bash
node --test experiments/moriarty-language/formal/mil4/wire/codec.test.mjs
node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs > experiments/moriarty-language/formal/mil4/wire/red-output.tap
node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs
node experiments/moriarty-language/formal/mil4/wire/record-results.mjs
```

The first command was run while the codec was absent. The second preserved the initial failing corpus as TAP. Subsequent commands ran after implementation. The recorder reruns the complete targeted corpus and records its actual executable path.

Remaining loci: the effect commitment is illustrative and opaque; no canonical effect encoder/equality consumer exists here. Source/6 lowering, Core/5/K/Quint correspondence, key curve validity, signer ownership, obligation/head authentication, replay consumption, financial range/cap predicates, native proof binding, SHA-256 circuit cost, wallet message preprocessing, tagged ledger key conversion, consensus verification, atomic effect application and cap feasibility remain unverified. No wallet, proof, compiler campaign or public transaction was invoked. Final W-D1/W-D2 decisions require independent review and the named consumer evidence.

````
