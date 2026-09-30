Independent read-only W-D2 post-review codec audit. Review only the exact embedded bytes; do not use tools, skills, delegation, external pages or workspace files. Requested reviewers are Grok 4.7 xhigh and GPT-6.1 Sol high, independently. Prior repaired packet SHA256 00006be0a79efb0a523f1a2ee4a959697b536e3f968f064975d8d0cc482de5bd had two medium JavaScript API defects: shadowable typed-array bounds and changing Proxy property reads. This candidate snapshots validated descriptor values and uses intrinsic typed-array metadata. Inspect canonicality, accepted encode/decode round trips, bounds before copy, exact vectors and test strength. Give separate verdicts: useful provisional codec, and W-D2 normative closure. Do not infer wallet signature, proof, effect commitment equality, Source/Core binding or ledger admission.

## Manifest

```json
[
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/WIRE-DECISION-RESEARCH.md",
    "bytes": 6370,
    "sha256": "98e13c548c99ee7a25f4b8b9c89d43bf29f3efa2ca876c55dc44b3bc496b7c03"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/SPEC.md",
    "bytes": 9579,
    "sha256": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.mjs",
    "bytes": 9975,
    "sha256": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.test.mjs",
    "bytes": 15639,
    "sha256": "34c6dc78eb895e5966647d279390f9d4ca0cd4e1dc4a45f4aa34f04c6917a5e8"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/fixtures.json",
    "bytes": 13320,
    "sha256": "b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/reference-vectors.py",
    "bytes": 8405,
    "sha256": "d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/REFERENCE-VECTORS-RECEIPT.md",
    "bytes": 3126,
    "sha256": "913c4f51b1711331917a6ef1f020ebacd2fc2ea803e4adca45a1e3fa0c9f7c94"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/record-results.mjs",
    "bytes": 4448,
    "sha256": "baef324656586eff31824a772921365d0dcd65d37b8f9fd0eaf4502947a6d425"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/results.json",
    "bytes": 13462,
    "sha256": "9ebb42d0d98e1dbbba20a277cf722fa998b1123e660b33e7b87327fc842ec7cb"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/RESULT.md",
    "bytes": 6725,
    "sha256": "e79365271a104685912c165fa485ca230ce9ca30b3c957ca1478fc90d803e602"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/PLAN.md",
    "bytes": 4717,
    "sha256": "4d151e0f63de4283dafce5a8b7f12c6bb030b7d15fc853e18ccd865dc4564a90"
  }
]
```

## deliverables/mil4-k-quint-sprint1-2026-09-29/WIRE-DECISION-RESEARCH.md

```text
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

## 2026-09-30 experiment update

The paragraph above records the state when this research note was first written. A separate [provisional `/3` codec experiment](../../experiments/moriarty-language/formal/mil4/wire/RESULT.md) now implements closed S0 tags and executes finite byte tests. Its Python reference builder was added after the first codec audit. Neither the codec nor its tests establish signature, effect-commitment, proof or ledger correspondence. W-D1 and W-D2 remain open.

The codec uses 16-byte unsigned fields but admits S0 nominal values only through `2^127−1`; balances and allowance counters in the semantic S0 prototype use checked UInt128. This is narrower than the full-UInt128 atomic-amount recommendation above. No proof-target feasibility result or final W-D4 decision justifies that difference yet. A normative wire decision must either adopt the narrower nominal rule with its exact source/Core boundary or widen the codec and re-review every bound and hostile vector. The Source/6 settlement scale cap is 18 while the provisional wire scale cap is 38; this also requires an explicit cross-layer rule before closure.

A local [pinned ledger primitive experiment](../../experiments/moriarty-language/formal/mil4/signature/RESULT.md) shows that a synthetic signature over the connector-style prefixed digest verifies on the exact supplied message and rejects several changed messages. It does not use a connector wallet or establish a consensus acceptance hook.

```

## experiments/moriarty-language/formal/mil4/wire/SPEC.md

```text
# PROVISIONAL `/3` S0 signed authorization codec

**Candidate only.** This isolated wire experiment does not freeze W-D1/W-D2, migrate `/3` to `/4`, or implement a source/Core/ledger consumer. The caps below are experimental admission choices, not measured wallet or network limits.

## Bytes and primitive types

Header is 18 bytes: the 17 ASCII bytes `moriarty-intent/3` followed by one NUL. Every subsequent field starts with its one-byte tag, in the exact order below. There are no optional fields. No generic extension tag, padding or trailing byte is admitted. Binary input must be a Uint8Array of at most 4096 bytes; tag order is part of canonicality. Decoder checks the intrinsic view brand/type, byte length, backing buffer and offset; own metadata properties and accessors cannot change the bound or copied bytes. It copies only the checked view into a fresh bounded buffer. Proxied or forged views, detached storage and incompatible view types return `SHAPE`. Oversized actual views return `LENGTH` before allocation, even when their own byteLength reports zero.

`id` is a UInt16 big-endian byte count followed by 1–64 ASCII bytes matching `[A-Za-z0-9][A-Za-z0-9._:/-]*`. There is no Unicode normalization, NUL, case folding or whitespace trimming. IDs are nominal and distinct fields preserve distinct sorts; the codec does not authenticate identifier ownership. Encoder string type and the 64-code-unit length limit precede alphabet validation and byte allocation; any longer string returns `LENGTH`, including a malformed string. Accepted ASCII strings have equal code-unit and byte lengths.

`hash32` and `key32` are exactly 32 raw bytes. Their JSON presentation is exactly 64 lowercase hex characters, without `0x`. A key's length is checked; membership on secp256k1 is an external cryptographic check.

`u64` and `nominal` are respectively 8 and 16 big-endian bytes. Their JSON presentation is a canonical nonnegative decimal string: `0` or a nonzero digit followed by digits. u64 cap is `2^64−1`; nominal cap is `2^127−1`. Negative numbers, numeric JS values, leading zeroes, decimal points, exponent notation and cap+1 reject. Encoder string type and the 20/39-code-unit limits precede decimal validation and BigInt conversion; any longer string returns `RANGE`, including a malformed string. Fixed width prohibits redundant integer encodings. An operation amount must be positive. This codec does not check fee/gross/net arithmetic or balances.

`scale` is one unsigned byte, cap 38. `empty` is an explicit UInt16 count equal to zero and presents as `[]`; any element/count rejects. Fixed literal fields present as the exact strings/numbers in the table. Unknown object keys, missing fields, inherited required fields and unknown operation keys reject; property insertion order does not affect encoded bytes. Each required root and operation property is read once through its own enumerable data descriptor and retained in a local snapshot. Encoding and validity/positive-amount checks use that same snapshot. Property get traps are not used for these fields; operation kind also uses its single captured descriptor. For a Proxy, the validated descriptor snapshot is the accepted input value. Ordinary parsed JSON has the same bytes as before. This is not a general sandbox for arbitrary reflection traps or concurrent backing-storage mutation.

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

The JSON field `schemaVersion` is mandatory and exactly `moriarty-intent/3`; the header encodes it. Domain is explicitly signed. Exactly one signer key, one domain and one settlement asset exist in this record. Validity is a closed interval of rounds in that domain; conversion to clock time is not implemented. Provisional replay identity is the tuple `(domain, signer, nonce)`. The nonce is a separately supplied replay field, selected before digest construction; field 20 is included in the signed record, so changing it changes the intent digest. The two fixtures use distinct nonces and replay identities. The tuple uses nominal signer identity; authenticating its relationship to signerKey, predecessor/head, and consuming replay requires a consumer outside this codec. This tuple does not settle the normative key-rotation/replay policy.

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

```

## experiments/moriarty-language/formal/mil4/wire/codec.mjs

```text
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

```

## experiments/moriarty-language/formal/mil4/wire/codec.test.mjs

```text
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';

const { fixtures } = JSON.parse(await readFile(new URL('./fixtures.json', import.meta.url)));
let codec;
try { codec = await import('./codec.mjs'); }
catch (error) { if (error.code !== 'ERR_MODULE_NOT_FOUND') throw error; codec = {}; }
const encode = (...args) => { assert.equal(typeof codec.encodeAuthorization, 'function', 'candidate encoder is not implemented'); return codec.encodeAuthorization(...args); };
const decode = (...args) => { assert.equal(typeof codec.decodeAuthorization, 'function', 'candidate decoder is not implemented'); return codec.decodeAuthorization(...args); };
const digest = (...args) => { assert.equal(typeof codec.authorizationDigest, 'function', 'candidate digest is not implemented'); return codec.authorizationDigest(...args); };
const transfer = fixtures[0];
const repayment = fixtures[1];
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
    assert.equal(createHash('sha256').update(Buffer.from(fixture.expected.wireHex, 'hex')).digest('hex'), fixture.expected.digestHex);
    assert.equal(digest(fixture.authorization), fixture.expected.digestHex);
  });
  test(`${fixture.id}: nested operation key order cannot change signed bytes`, () => {
    const a = structuredClone(fixture.authorization);
    a.operation = Object.fromEntries(Object.entries(a.operation).reverse());
    assert.equal(encode(a).toString('hex'), fixture.expected.wireHex);
  });
}
test('fixtures have distinct provisional replay identities', () => {
  const replay = a => JSON.stringify([a.domain, a.signer, a.nonce]);
  assert.equal(new Set(fixtures.map(fixture => replay(fixture.authorization))).size, fixtures.length);
});
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
for (const field of ['domain', 'operation.owner']) {
  test(`huge identifier ${field} is length-rejected before alphabet validation`, () => {
    for (const text of ['a'.repeat(1024 * 1024), ' '.repeat(1024 * 1024)]) {
      const a = fresh();
      if (field === 'domain') a.domain = text; else a.operation.owner = text;
      rejects(() => encode(a), 'LENGTH');
    }
  });
}
for (const field of ['validUntil', 'grossCap', 'operation.amount']) {
  test(`huge decimal ${field} is range-rejected before canonicality validation`, () => {
    for (const text of ['9'.repeat(1024 * 1024), 'x'.repeat(1024 * 1024)]) {
      const a = fresh();
      if (field === 'operation.amount') a.operation.amount = text; else a[field] = text;
      rejects(() => encode(a), 'RANGE');
    }
  });
}
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
for (const fixture of fixtures) {
  test(`every truncation of a valid ${fixture.id} reports TRUNCATED`, () => {
    const b = Buffer.from(fixture.expected.wireHex, 'hex');
    for (let n = 0; n < b.length; n++) rejects(() => decode(b.subarray(0, n)), 'TRUNCATED');
  });
  test(`${fixture.id}: reversed wire validity reports VALIDITY`, () => {
    const b = Buffer.from(fixture.expected.wireHex, 'hex');
    b.writeBigUInt64BE(201n, fixture.expected.fieldOffsets.validFrom.value);
    rejects(() => decode(b), 'VALIDITY');
  });
}
for (const [name, mutate, code] of [
  ['unknown kind', (b, at) => { b[at('operation')] = 3; }, 'OPERATION_TAG'],
  ['obligation length cap+1', (b, at) => { b.writeUInt16BE(65, at('operation.obligationId')); }, 'LENGTH'],
  ['empty payer', (b, at) => { b.writeUInt16BE(0, at('operation.payer')); }, 'ID'],
  ['non-ASCII debtor', (b, at) => { b[at('operation.debtor') + 2] = 255; }, 'ID'],
  ['invalid creditor alphabet', (b, at) => { b[at('operation.creditor') + 2] = 32; }, 'ID'],
  ['amount high bit', (b, at) => { b[at('operation.amount')] = 128; }, 'RANGE'],
  ['zero amount', (b, at) => { b.fill(0, at('operation.amount'), at('operation.amount') + 16); }, 'RANGE'],
  ['unknown allocation', b => { b[b.length - 2] = 2; }, 'LITERAL'],
  ['unknown conversion', b => { b[b.length - 1] = 2; }, 'LITERAL'],
]) test(`repayment wire rejection: ${name}`, () => {
  const b = Buffer.from(repayment.expected.wireHex, 'hex');
  const offset = name => repayment.expected.fieldOffsets[name].value;
  mutate(b, offset); rejects(() => decode(b), code);
});
test('decoder rejects ordinary arrays as bytes', () => rejects(() => decode([...wire()]), 'SHAPE'));
test('decoder uses actual size for a huge view with shadowed byteLength', () => {
  const bytes = new Uint8Array(1024 * 1024); bytes.set(wire());
  Object.defineProperty(bytes, 'byteLength', { value: 0 });
  rejects(() => decode(bytes), 'LENGTH');
});
test('decoder does not read shadowed view metadata getters', () => {
  const bytes = new Uint8Array(wire());
  for (const name of ['byteLength', 'length', 'buffer', 'byteOffset']) {
    Object.defineProperty(bytes, name, { get() { throw new Error(`untrusted ${name} read`); } });
  }
  assert.deepEqual(decode(bytes), transfer.authorization);
});
test('decoder ignores a shorter shadowed view length', () => {
  const bytes = new Uint8Array(wire());
  Object.defineProperty(bytes, 'length', { value: 1 });
  assert.deepEqual(decode(bytes), transfer.authorization);
});
test('decoder rejects a Proxy over a byte view with SHAPE', () => rejects(() => decode(new Proxy(new Uint8Array(wire()), {})), 'SHAPE'));
test('decoder rejects a forged typed-array prototype with SHAPE', () => rejects(() => decode(Object.create(Uint8Array.prototype)), 'SHAPE'));
test('decoder rejects a detached view with SHAPE', () => {
  const bytes = new Uint8Array(wire());
  structuredClone(bytes.buffer, { transfer: [bytes.buffer] });
  rejects(() => decode(bytes), 'SHAPE');
});
test('decoder preserves a byte subview with nonzero offset', () => {
  const expected = wire(); const padded = new Uint8Array(expected.length + 16);
  padded.set(expected, 7);
  assert.deepEqual(decode(padded.subarray(7, 7 + expected.length)), transfer.authorization);
});
test('decoder rejects other typed-array element types with SHAPE', () => rejects(() => decode(new Uint16Array(270)), 'SHAPE'));
test('root Proxy cannot hide reversed validity through changing property reads', () => {
  const a = fresh(); a.validFrom = '201'; let reads = 0;
  const proxy = new Proxy(a, { get(target, name) { if (name === 'validFrom') return ++reads === 1 ? '201' : '100'; return Reflect.get(target, name); } });
  rejects(() => encode(proxy), 'VALIDITY');
});
test('operation Proxy cannot hide zero amount through changing property reads', () => {
  const a = fresh(); a.operation.amount = '0'; let reads = 0;
  a.operation = new Proxy(a.operation, { get(target, name) { if (name === 'amount') return ++reads === 1 ? '0' : '1'; return Reflect.get(target, name); } });
  rejects(() => encode(a), 'RANGE');
});
test('root and nested descriptor snapshots encode without property get traps', () => {
  for (const fixture of fixtures) {
    const a = structuredClone(fixture.authorization); const counts = new Map();
    const handler = field => ({
      get() { throw new Error('property get trap must not run'); },
      getOwnPropertyDescriptor(target, name) {
        const key = `${field}.${name}`; counts.set(key, (counts.get(key) ?? 0) + 1);
        return Reflect.getOwnPropertyDescriptor(target, name);
      },
    });
    a.operation = new Proxy(a.operation, handler('operation'));
    const encoded = encode(new Proxy(a, handler('authorization')));
    assert.equal(encoded.toString('hex'), fixture.expected.wireHex);
    assert.deepEqual(decode(encoded), fixture.authorization);
    assert.equal(counts.size, Object.keys(fixture.authorization).length + Object.keys(fixture.authorization.operation).length);
    for (const count of counts.values()) assert.equal(count, 1, 'each descriptor must be read once');
  }
});
test('operation kind descriptor is snapshotted once', () => {
  const a = fresh(); let reads = 0;
  a.operation = new Proxy(a.operation, {
    getOwnPropertyDescriptor(target, name) {
      const descriptor = Reflect.getOwnPropertyDescriptor(target, name);
      if (name === 'kind' && ++reads > 1) descriptor.value = 'repayment';
      return descriptor;
    },
  });
  const encoded = encode(a);
  assert.equal(encoded.toString('hex'), transfer.expected.wireHex);
  assert.deepEqual(decode(encoded), transfer.authorization);
  assert.equal(reads, 1);
});
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
  ['signer identity', a => { a.signer = 'other'; }],
  ['asset', a => { a.asset = 'asset:EUR'; }],
  ['scale', a => { a.scale = 3; }],
  ['policy hash', a => { a.policyHash = '08'.repeat(32); }],
  ['gross cap', a => { a.grossCap = '111'; }],
  ['net floor', a => { a.netFloor = '101'; }],
  ['amount', a => { a.operation.amount = '101'; }],
  ['fee', a => { a.operation.fee = '6'; }],
  ['owner', a => { a.operation.owner = 'other'; }],
  ['program id', a => { a.coreProgramId = 'other-program'; }],
]) test(`content binding discriminator: ${name}`, () => { const a = fresh(); mutate(a); assert.notEqual(digest(a), transfer.expected.digestHex); });
test('repayment creditor substitution changes digest', () => { const a = structuredClone(fixtures[1].authorization); a.operation.creditor = 'eve'; assert.notEqual(digest(a), fixtures[1].expected.digestHex); });

```

## experiments/moriarty-language/formal/mil4/wire/fixtures.json

```text
{
  "status": "PROVISIONAL; current expected vectors independently reproduced post-audit from SPEC.md; repayment nonce repaired; effect commitments illustrative",
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
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
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
        "wireHex": "6d6f7269617274792d696e74656e742f330001010200106d69646e696768743a7072657669657703000b61677265656d656e742d3104000773746167652d31050009657069736f64652d3106000772657061792d31070608222222222222222222222222222222222222222222222222222222222222222209050a000c636f72652d72657061792d310b33333333333333333333333333333333333333333333333333333333333333330c44444444444444444444444444444444444444444444444444444444444444440d0005616c6963650e010f111111111111111111111111111111111111111111111111111111111111111110000961737365743a55534411021255555555555555555555555555555555555555555555555555555555555555551366666666666666666666666666666666666666666666666666666666666666661478787878787878787878787878787878787878787878787878787878787878781500000000000000641600000000000000c8170000000000000000000000000000001e1800000000000000000000000000000000190000000000000000000000000000001e1a99999999999999999999999999999999999999999999999999999999999999991b011c00001d00001e00001f000020000021002200230200066c6f616e2d310005616c6963650005616c696365000462616e6b0000000000000000000000000000001e0101",
        "digestHex": "4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4",
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

```

## experiments/moriarty-language/formal/mil4/wire/reference-vectors.py

```text
#!/usr/bin/env python3
"""Post-audit independent reproduction of SPEC.md's provisional /3 vectors.

Python standard library only. Construction uses authorization data and the
ordered specification below; expected fixture values are read only afterward
for comparison. This is neither a production codec nor provenance evidence
that the original fixtures preceded the Node implementation.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys


# SPEC.md's common field table, in tag order. Tags are 1 through 35.
COMMON = (
    ("profile", "literal", ("s0-provisional/1", 1)),
    ("domain", "id", None),
    ("agreementId", "id", None),
    ("stageId", "id", None),
    ("episodeId", "id", None),
    ("actionId", "id", None),
    ("sourceVersion", "literal", (6, 6)),
    ("sourceHash", "hash32", None),
    ("coreVersion", "literal", (5, 5)),
    ("coreProgramId", "id", None),
    ("coreHash", "hash32", None),
    ("policyHash", "hash32", None),
    ("signer", "id", None),
    ("keyScheme", "literal", ("schnorr_bip340", 1)),
    ("signerKey", "hash32", None),
    ("asset", "id", None),
    ("scale", "scale", None),
    ("preHead", "hash32", None),
    ("predecessor", "hash32", None),
    ("nonce", "hash32", None),
    ("validFrom", "u64", None),
    ("validUntil", "u64", None),
    ("grossCap", "nominal", None),
    ("feeCap", "nominal", None),
    ("netFloor", "nominal", None),
    ("effectCommitment", "hash32", None),
    ("failurePolicy", "literal", ("atomic-reject-terminal-success", 1)),
    ("supplyChanges", "empty", None),
    ("observations", "empty", None),
    ("disclosures", "empty", None),
    ("retainedEffects", "empty", None),
    ("retainedDuties", "empty", None),
    ("delegation", "literal", ("none", 0)),
    ("recovery", "literal", ("none", 0)),
    ("operation", "operation", None),
)
OPERATIONS = {
    "transfer": (1, (
        ("owner", "id", None),
        ("recipient", "id", None),
        ("feeRecipient", "id", None),
        ("amount", "nominal", None),
        ("fee", "nominal", None),
    )),
    "repayment": (2, (
        ("obligationId", "id", None),
        ("payer", "id", None),
        ("debtor", "id", None),
        ("creditor", "id", None),
        ("amount", "nominal", None),
        ("allocation", "literal", ("AccrualFirst", 1)),
        ("conversion", "literal", ("identity", 1)),
    )),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def primitive(value, kind, literal=None):
    if kind == "literal":
        presentation, byte = literal
        require(type(value) is type(presentation) and value == presentation,
                "incorrect literal")
        return bytes([byte])
    if kind == "id":
        require(isinstance(value, str), "noncanonical identifier")
        require(1 <= len(value) <= 64, "identifier exceeds cap")
        require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:/-]*", value),
                "noncanonical identifier")
        payload = value.encode("ascii")
        require(1 <= len(payload) <= 64, "identifier exceeds cap")
        return len(payload).to_bytes(2, "big") + payload
    if kind == "hash32":
        require(isinstance(value, str) and
                re.fullmatch(r"[0-9a-f]{64}", value),
                "noncanonical 32-byte hex string")
        return bytes.fromhex(value)
    if kind in ("u64", "nominal"):
        require(isinstance(value, str), "noncanonical decimal string")
        width, cap = (8, 2**64 - 1) if kind == "u64" else (16, 2**127 - 1)
        require(len(value) <= len(str(cap)), "integer exceeds cap")
        require(re.fullmatch(r"0|[1-9][0-9]*", value),
                "noncanonical decimal string")
        number = int(value)
        require(number <= cap, "integer exceeds cap")
        return number.to_bytes(width, "big")
    if kind == "scale":
        require(type(value) is int and 0 <= value <= 38, "invalid scale")
        return bytes([value])
    if kind == "empty":
        require(type(value) is list and not value, "nonempty deferred field")
        return (0).to_bytes(2, "big")
    raise ValueError("unknown primitive type: " + kind)


def reference_vector(authorization):
    """Build bytes and offsets without accepting or consulting expected data."""
    require(type(authorization) is dict and set(authorization) ==
            {"schemaVersion", *(name for name, _, _ in COMMON)},
            "incorrect authorization shape")
    operation = authorization["operation"]
    require(type(operation) is dict and
            isinstance(operation.get("kind"), str) and
            operation["kind"] in OPERATIONS, "incorrect operation shape/kind")
    operation_tag, operation_fields = OPERATIONS[operation["kind"]]
    require(set(operation) == {"kind", *(name for name, _, _ in operation_fields)},
            "incorrect operation keys")
    require(authorization["schemaVersion"] == "moriarty-intent/3",
            "incorrect schema version")

    wire = bytearray(authorization["schemaVersion"].encode("ascii") + b"\x00")
    offsets = {}
    for tag, (name, kind, literal) in enumerate(COMMON, start=1):
        offsets[name] = {"tag": len(wire), "value": len(wire) + 1}
        wire.append(tag)
        if kind == "operation":
            wire.append(operation_tag)
            for leaf, leaf_kind, leaf_literal in operation_fields:
                # fixtures.json indexes variable operation leaves. Fixed
                # repayment allocation/conversion suffixes have no entries.
                if leaf_kind != "literal":
                    offsets["operation." + leaf] = {"value": len(wire)}
                wire.extend(primitive(operation[leaf], leaf_kind, leaf_literal))
            require(int(operation["amount"]) > 0, "zero operation amount")
        else:
            wire.extend(primitive(authorization[name], kind, literal))
        if name == "validUntil":
            require(int(authorization["validUntil"]) >=
                    int(authorization["validFrom"]), "reversed validity interval")
    require(len(wire) <= 4096, "record exceeds cap")
    return {
        "wireHex": wire.hex(),
        "digestHex": hashlib.sha256(wire).hexdigest(),
        "length": len(wire),
        "fieldOffsets": offsets,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fixtures", nargs="?", type=Path,
                        default=Path(__file__).with_name("fixtures.json"))
    parser.add_argument("--json", action="store_true",
                        help="emit all calculated vectors and comparison results")
    args = parser.parse_args()
    try:
        document = json.loads(args.fixtures.read_text(encoding="utf-8"))
        fixtures = document["fixtures"]
        require(type(fixtures) is list and fixtures, "fixtures must be nonempty")
        results = []
        for fixture in fixtures:
            calculated = reference_vector(fixture["authorization"])
            expected = fixture["expected"]
            require(type(expected) is dict and set(expected) == set(calculated),
                    "incorrect expected vector keys")
            mismatches = [key for key in calculated if calculated[key] != expected[key]]
            results.append({"id": fixture["id"], "calculated": calculated,
                            "mismatches": mismatches})
        passed = all(not result["mismatches"] for result in results)
        if args.json:
            print(json.dumps({"passed": passed, "vectors": results}, indent=2))
        else:
            for result in results:
                vector = result["calculated"]
                verdict = "PASS" if not result["mismatches"] else "FAIL"
                print(f"{verdict} {result['id']}: length={vector['length']} "
                      f"digestHex={vector['digestHex']} "
                      f"fieldOffsets={len(vector['fieldOffsets'])}")
                if result["mismatches"]:
                    print("  mismatches: " + ", ".join(result["mismatches"]))
            print(f"{'PASS' if passed else 'FAIL'}: {len(results)} vectors; "
                  "compared wireHex, digestHex, length, fieldOffsets")
        return 0 if passed else 1
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())

```

## experiments/moriarty-language/formal/mil4/wire/REFERENCE-VECTORS-RECEIPT.md

```text
# Independent Python vector reproduction

Experiment observation, 2026-09-30 07:02:34 UTC. This current post-audit
reproduction follows the intrinsic metadata and descriptor snapshot repair.
The two golden vectors remain unchanged from the 06:21:14 UTC length-guard
and replay fixture repair reproduction. It is
derived from `SPEC.md`, using only the Python standard library, and does not
establish that the original fixtures preceded the Node codec.
No Node codec or expected byte strings supply construction data: the generator
receives only each fixture's `authorization`, then compares the computed result
with `expected` afterward. Current expected vectors were regenerated through
that same independent Python `reference_vector` function after repayment's
nonce changed to `78` repeated 32 times. Transfer retains `77` repeated 32 times.

Command, from the checkout root:

```text
python3 experiments/moriarty-language/formal/mil4/wire/reference-vectors.py
```

Python 3.14.4; exit code 0. Actual output:

```text
PASS transfer: length=540 digestHex=722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22 fieldOffsets=40
PASS repayment: length=530 digestHex=4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4 fieldOffsets=40
PASS: 2 vectors; compared wireHex, digestHex, length, fieldOffsets
```

Four separate temporary fixture copies changed one expected `wireHex`,
`digestHex`, `length` or `fieldOffsets` value. Each run named the altered field
as a mismatch and returned exit code 1. The originals were not changed by these
controls. `--json` emits the complete calculated vectors. Offsets count from
byte zero; identifier value offsets point to their UInt16 length prefixes.
The fixture offset presentation omits repayment's fixed allocation/conversion
suffix entries; their bytes are independently constructed and compared, and
the hostile decoder checks mutate the final two bytes directly.

The earlier 06:04:54 UTC reproduction checked the prior repayment vector
with nonce `77` repeated 32 times and digest
`7de5d64b195c0f509e26622cd8f68ce993ae95a741cc82043cf387de344da673`.
The frozen packet and manifests in the deliverable's audits directory retain
that prior candidate. They were not edited during this repair; current wire
source files intentionally differ from that historical candidate.

Current inputs and generator SHA-256:

```text
SPEC.md              644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1
fixtures.json        b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d
reference-vectors.py d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1
```

The current recorder runs the generator itself and records its command,
runtime, exit status and complete calculated vectors in `results.json`.
Node tests also hash each stored golden wire directly with Node SHA-256.
This result covers the two valid provisional codec vectors and mismatch
detection. W-D2 normative closure, semantic correspondence, authenticated
effects, signatures, wallet interoperability, proofs and ledger acceptance
remain unestablished by this reproduction.

```

## experiments/moriarty-language/formal/mil4/wire/record-results.mjs

```text
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
const referencePath = 'experiments/moriarty-language/formal/mil4/wire/reference-vectors.py';
const referenceArgs = [referencePath, '--json'];
const reference = spawnSync('python3', referenceArgs, { cwd, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
let referenceResult = null;
if (reference.status === 0) referenceResult = JSON.parse(reference.stdout);
const pythonVersion = spawnSync('python3', ['--version'], { cwd, encoding: 'utf8' });
const comparisons = fixtures.map(fixture => ({
  id: fixture.id,
  encodedBytes: encodeAuthorization(fixture.authorization).length,
  exactBytesMatch: encodeAuthorization(fixture.authorization).toString('hex') === fixture.expected.wireHex,
  digest: authorizationDigest(fixture.authorization),
  expectedDigestMatch: authorizationDigest(fixture.authorization) === fixture.expected.digestHex,
  goldenWireDigestMatch: createHash('sha256').update(Buffer.from(fixture.expected.wireHex, 'hex')).digest('hex') === fixture.expected.digestHex,
  independentDigestMatch: referenceResult?.vectors.find(value => value.id === fixture.id)?.calculated.digestHex === authorizationDigest(fixture.authorization),
}));
const names = ['SPEC.md', 'PLAN.md', 'codec.mjs', 'codec.test.mjs', 'fixtures.json', 'record-results.mjs', 'reference-vectors.py', 'REFERENCE-VECTORS-RECEIPT.md', 'RESULT.md', 'red-output.tap', 'post-audit-red-output.tap', 'metadata-snapshot-red-output.tap', 'test-output.tap'];
const artifacts = {};
for (const name of names) artifacts[name] = createHash('sha256').update(await readFile(new URL(name, dir))).digest('hex');
const result = {
  status: 'PROVISIONAL_W_D2_CODEC_EXPERIMENT', startedAt: started, finishedAt: new Date().toISOString(),
  cwd, command: [process.execPath, ...args], runtime: process.version, platform: process.platform,
  exitCode: run.status, signal: run.signal, spawnError: run.error?.message ?? null,
  counts, fixtureComparisons: comparisons, artifacts,
  independentReference: {
    command: ['python3', ...referenceArgs], runtime: pythonVersion.stdout.trim(),
    exitCode: reference.status, signal: reference.signal, spawnError: reference.error?.message ?? null,
    stderr: reference.stderr, passed: referenceResult?.passed === true,
    vectors: referenceResult?.vectors ?? null,
    provenance: 'post-audit independent reproduction; no claim that original vectors preceded Node codec',
  },
  acceptedClaims: ['targeted canonical byte/digest fixture agreement', 'targeted round-trip and hostile codec outcomes'],
  unverified: ['wallet/signature interoperability', 'effect commitment correspondence', 'Source/Core/K/Quint correspondence', 'native proof/hash binding', 'ledger verification and atomic acceptance', 'cap feasibility', 'W-D1/W-D2 normative closure'],
};
await writeFile(new URL('results.json', dir), JSON.stringify(result, null, 2) + '\n');
const passed = run.status === 0 && counts.fail === 0 && counts.tests === 114 && counts.pass === 114
  && reference.status === 0 && referenceResult?.passed === true
  && comparisons.every(value => value.exactBytesMatch && value.expectedDigestMatch && value.goldenWireDigestMatch && value.independentDigestMatch);
console.log(JSON.stringify({ passed, exitCode: run.status, counts, fixtureComparisons: comparisons }, null, 2));
if (!passed) process.exitCode = 1;

```

## experiments/moriarty-language/formal/mil4/wire/results.json

```text
{
  "status": "PROVISIONAL_W_D2_CODEC_EXPERIMENT",
  "startedAt": "2026-09-30T07:03:58.130Z",
  "finishedAt": "2026-09-30T07:03:58.365Z",
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
    "tests": 114,
    "pass": 114,
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
      "expectedDigestMatch": true,
      "goldenWireDigestMatch": true,
      "independentDigestMatch": true
    },
    {
      "id": "repayment",
      "encodedBytes": 530,
      "exactBytesMatch": true,
      "digest": "4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4",
      "expectedDigestMatch": true,
      "goldenWireDigestMatch": true,
      "independentDigestMatch": true
    }
  ],
  "artifacts": {
    "SPEC.md": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1",
    "PLAN.md": "4d151e0f63de4283dafce5a8b7f12c6bb030b7d15fc853e18ccd865dc4564a90",
    "codec.mjs": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe",
    "codec.test.mjs": "34c6dc78eb895e5966647d279390f9d4ca0cd4e1dc4a45f4aa34f04c6917a5e8",
    "fixtures.json": "b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d",
    "record-results.mjs": "baef324656586eff31824a772921365d0dcd65d37b8f9fd0eaf4502947a6d425",
    "reference-vectors.py": "d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1",
    "REFERENCE-VECTORS-RECEIPT.md": "913c4f51b1711331917a6ef1f020ebacd2fc2ea803e4adca45a1e3fa0c9f7c94",
    "RESULT.md": "e79365271a104685912c165fa485ca230ce9ca30b3c957ca1478fc90d803e602",
    "red-output.tap": "235d33174018a6cf8bac764104baaf82f0cef9a9e8f776929c9fc97c28c68d39",
    "post-audit-red-output.tap": "81c60c3cd90feacb3a7f456d806980fa99aa10dde5d2753e334556344d36705c",
    "metadata-snapshot-red-output.tap": "67cfd7621da0a5d5a6e29fff4f99d3812340200a426772eebd6eb83b9c625f40",
    "test-output.tap": "22ff49b67099fe8cdca6c10fd8d84e8f26e464dcc2e41ff725e2e1386fc907d1"
  },
  "independentReference": {
    "command": [
      "python3",
      "experiments/moriarty-language/formal/mil4/wire/reference-vectors.py",
      "--json"
    ],
    "runtime": "Python 3.14.4",
    "exitCode": 0,
    "signal": null,
    "spawnError": null,
    "stderr": "",
    "passed": true,
    "vectors": [
      {
        "id": "transfer",
        "calculated": {
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
        },
        "mismatches": []
      },
      {
        "id": "repayment",
        "calculated": {
          "wireHex": "6d6f7269617274792d696e74656e742f330001010200106d69646e696768743a7072657669657703000b61677265656d656e742d3104000773746167652d31050009657069736f64652d3106000772657061792d31070608222222222222222222222222222222222222222222222222222222222222222209050a000c636f72652d72657061792d310b33333333333333333333333333333333333333333333333333333333333333330c44444444444444444444444444444444444444444444444444444444444444440d0005616c6963650e010f111111111111111111111111111111111111111111111111111111111111111110000961737365743a55534411021255555555555555555555555555555555555555555555555555555555555555551366666666666666666666666666666666666666666666666666666666666666661478787878787878787878787878787878787878787878787878787878787878781500000000000000641600000000000000c8170000000000000000000000000000001e1800000000000000000000000000000000190000000000000000000000000000001e1a99999999999999999999999999999999999999999999999999999999999999991b011c00001d00001e00001f000020000021002200230200066c6f616e2d310005616c6963650005616c696365000462616e6b0000000000000000000000000000001e0101",
          "digestHex": "4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4",
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
        },
        "mismatches": []
      }
    ],
    "provenance": "post-audit independent reproduction; no claim that original vectors preceded Node codec"
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

```

## experiments/moriarty-language/formal/mil4/wire/RESULT.md

```text
# PROVISIONAL W-D2 codec experiment

This candidate implements a separate `/3` binary authorization codec for one-signer S0 transfer and existing-obligation AccrualFirst repayment. All prototype files are new and confined to this directory. It does not adopt a wire profile or close W-D2.

Observed current targeted result: 114 tests passed, zero failed. The [machine result](results.json) records the exact runtime, invocation, exit code, fixture comparisons, independently executed Python reproduction and SHA-256 artifact hashes. [Complete current TAP](test-output.tap) and [metadata/snapshot failing TAP](metadata-snapshot-red-output.tap) preserve the current repair observations: 104 passed and ten failed before the intrinsic metadata and descriptor snapshots were repaired. The twelve added controls include hostile byte-view metadata, Proxy/forged/detached/incompatible views, a nonzero-offset subview, reversed-validity and zero-amount property-read exploits, no-get-trap snapshots, and single-read operation kind.

The earlier length-guard/nonce repair passed all 102 then-current controls, retained in this expanded corpus. Its [post-audit failing TAP](post-audit-red-output.tap) records 96 passed and six failed before that repair. Those six failures were the shared replay identity, two oversized malformed ID cases and three oversized malformed decimal cases. The original [first failing TAP](red-output.tap) remains historical: 71 failures and one passing rejection-only test because the codec functions were absent; that one test accepted any thrown exception. Current truncation checks require the codec's `TRUNCATED` code for every proper prefix of both fixtures. This does not assert that every later internal helper had its own separate red cycle.

The current expected vectors were generated and reproduced post-audit from authorization values and the ordered specification using [reference-vectors.py](reference-vectors.py), with Python hashlib.sha256. No Node codec output or stored expected bytes supply construction data. This establishes post-audit independent reproduction; it does not establish that the original vectors preceded the Node codec. Transfer remains 540 bytes with digest `722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22`. Repayment now uses nonce `78` repeated 32 times, distinct from transfer's `77` repeated 32 times, and remains 530 bytes with digest `4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4`. Its expected bytes, digest, length and offsets were regenerated by the Python reference generator. [The receipt](REFERENCE-VECTORS-RECEIPT.md) records current reproduction and deliberate expected-value mismatch controls. The original frozen packet and manifests in the deliverable's audits directory remain immutable historical evidence of the prior candidate, including its old repayment digest. This is bounded agreement, not a formal uniqueness or correspondence proof.

Controls cover exact fixture bytes, direct SHA-256 of each stored golden wire, typed decode, round trips, root and nested operation key order, distinct fixture replay identities, zero fee, maximum admitted ID/numeric widths/scale, unknown/missing properties, unknown versions/tags/schemes, duplicate field tags, unknown repayment literals, noncanonical ASCII/hex/decimal representations, cap+1 values, nonempty deferred fields, extra integer padding, trailing bytes, record cap+1 and every truncated prefix of both valid records. Valid and malformed one-megabyte ID/decimal strings exercise the guards through root and operation fields. Encoder length guards now precede regex scans and allocations/BigInt conversion. Wire validity reversal is checked for both records. Repayment mutations cover kind, obligation length, payer/debtor/creditor encodings, amount high-bit/zero and allocation/conversion bytes. Content mutations also cover signer identity, asset, scale, policy hash, gross cap, net floor, amount, fee, owner and program ID. No test verifies a signature or financial acceptance.

Decoder now rejects an actual one-megabyte Uint8Array with own byteLength=0 using `LENGTH`; it ignores shadowed metadata on valid views and preserves the actual view's offset and bytes. Proxied, forged, detached and incompatible views use `SHAPE`. Encoder snapshots every required root/operation data descriptor once, including kind, and uses those values for encoding and validity/amount predicates. A Proxy with reversed validity or zero amount cannot hide that value with a later property get. Tests confirm that valid descriptor snapshots require no root/operation property get traps. Both accepted root/nested proxy variants and the changing-kind-descriptor input explicitly satisfy `decode(encode(input))` equal to the snapshot's typed value. These controls establish the named outcomes; they are not a general sandbox for arbitrary reflection traps or concurrent backing-storage changes.

Exact commands from the checkout root:

```bash
node --test experiments/moriarty-language/formal/mil4/wire/codec.test.mjs
node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs > experiments/moriarty-language/formal/mil4/wire/red-output.tap
node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs
node experiments/moriarty-language/formal/mil4/wire/record-results.mjs
python3 experiments/moriarty-language/formal/mil4/wire/reference-vectors.py
```

The original first command was run while the codec was absent. The second preserved the initial failing corpus as TAP and must not be used to overwrite that history. The length/nonce red run redirected its expanded corpus to `post-audit-red-output.tap`; the metadata/snapshot red run redirected the current expanded corpus to `metadata-snapshot-red-output.tap`. The recorder reruns the complete current targeted corpus and the Python generator, records actual command/runtime/status, and requires both independent vector comparisons to pass. No separate guard timing or peak-memory benchmark was performed.

Remaining loci: the effect commitment is illustrative and opaque; no canonical effect encoder/equality consumer exists here. Source/6 lowering, Core/5/K/Quint correspondence, key curve validity, signer ownership, obligation/head authentication, replay consumption, financial range/cap predicates, native proof binding, SHA-256 circuit cost, wallet message preprocessing, tagged ledger key conversion, consensus verification, atomic effect application and cap feasibility remain unverified. No wallet, proof, compiler campaign or public transaction was invoked. Final W-D1/W-D2 decisions require independent review and the named consumer evidence.

```

## experiments/moriarty-language/formal/mil4/wire/PLAN.md

```text
# Provisional S0 wire codec implementation plan

**Goal:** implement an isolated candidate `/3` authorization codec and targeted discriminators. This is authorized prototype work; no normative vote, migration or common-document change occurs.

**Architecture:** standalone Node ESM, fixed ordered tagged binary records, explicit transfer/repayment variants and a SHA-256 content digest. JSON fixtures use canonical decimal strings for integers. Only new files in this directory are in scope.

**Alternatives:** canonical JSON would need duplicate-key and whitespace rules; CBOR would require a dependency or additional parser. Fixed tags and widths provide a smaller dependency-free experiment and make hostile byte offsets explicit. This choice is provisional.

**Scope:** built-in Node crypto/assert/test/fs only; no wallet invocation, signing, signature verification, proof generation, ledger integration, or Source/Core/K/Quint consumer changes. The effect commitment is supplied and signed; equality to actual effects is an external consumer obligation. Caps are experimental choices, not observed network limits.

## Task 1: independently specified fixtures and failing tests

- [x] Write `SPEC.md` with exact header, tag/type order, operation variants, numeric/size caps, rejection codes and unknown external premises.
- [x] Write `fixtures.json` with transfer and existing-obligation repayment values independent of codec implementation.
- [x] Write `codec.test.mjs` to compare a separately assembled transfer byte vector and round trips, then mutate header/tags/lengths/numbers/empties and signed bindings.
- [x] Run `node --test --test-reporter=tap experiments/moriarty-language/formal/mil4/wire/codec.test.mjs`; record expected missing-codec failure. Dynamic import fallback lets the missing API fail as an assertion rather than hiding it as a loader error.

## Task 2: minimal codec and hostile decoding

- [x] Implement `codec.mjs` exports `encodeAuthorization(value): Buffer`, `decodeAuthorization(bytes): object`, `authorizationDigest(value): string`, and immutable public caps/constants. Errors expose stable `code`.
- [x] Re-run the same test command; observed 72/72 passing.
- [x] Check for acceptance defects; none found by this targeted corpus. Add a rule-reaching hostile case before any later repair.

## Task 3: reproducible candidate evidence

- [x] Write `record-results.mjs` to run the targeted test command, save complete TAP output, runtime/command/status/counts and SHA-256 artifact digests to `results.json` and `test-output.tap`.
- [x] Run `node experiments/moriarty-language/formal/mil4/wire/record-results.mjs` and inspect its output and recorded results; result status is recorded independently in results.json.
- [x] Write `RESULT.md` stating exact observed scope and unverified wallet/hash/proof/ledger/cap/consumer loci. Do not close W-D2 or alter other files.

No commit is made from this shared working tree: the parent will review and integrate the isolated artifact.

## Post-audit repair, 2026-09-30

- [x] Reproduce long-string guard and shared replay identity failures before repair: 102 tests, 96 passed, six failed; preserve `post-audit-red-output.tap`.
- [x] Apply string length guards before ID/decimal regex validation and byte allocation/BigInt conversion.
- [x] Require `TRUNCATED` for every transfer and repayment prefix; directly hash stored golden bytes; add nested key-order, wire validity, repayment hostile-byte and content-binding controls.
- [x] Give repayment a distinct nonce; regenerate expected bytes, SHA-256, length and offsets using the independent Python generator alone.
- [x] Correct original-vector provenance claims to post-audit reproduction; retain the frozen audit candidate as historical evidence.
- [x] Observe 102/102 current checks and both independent reference vectors passing; record actual runs. W-D2 and all named consumer loci remain open.

## Metadata and descriptor snapshot repair, 2026-09-30

- [x] Add twelve controls; reproduce ten failures before repair with 104/114 passing; preserve `metadata-snapshot-red-output.tap`.
- [x] Snapshot required root and operation descriptor values once; reuse the captured kind descriptor; encode and check validity/positive amount from those same values.
- [x] Check decoder intrinsic byte-view brand/type and metadata, guard actual size before bounded copy, and map incompatible/forged/proxied/detached views to `SHAPE`.
- [x] Preserve all 102 prior controls and both independently constructed golden vectors; observe 114/114 passing.
- [x] Refresh current result, independent reproduction receipt and artifact hashes. Frozen audits stay historical; W-D2 and consumer acceptance stay open.

```

