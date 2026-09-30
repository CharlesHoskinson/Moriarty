Independent W-D2H local image-codec result audit. Requested GPT-6.1 Sol high and Grok 4.7 xhigh. Review exact embedded bytes only if no filesystem access; GPT may independently run the frozen tests and recompute hashes. Check purpose1/3/4 byte order/width/caps, nominal/text/opaque admission, domain separation, vector independence limits, actual-package claims, content comparator limits and 72-test/result record. H repair-02 SPEC is embedded under history/pre-grok-repair-03 and is the exact codec source; current H repair-03 SPEC is embedded for parity comparison, and is still under independent design review. Find high/medium defects or bounded acceptance only. No provider, Source AST adapter, full consumer, loaded-artifact proof, B05-B07 adoption or W-D2/Sprint1 closure. Do not edit packet or source.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/PLAN.md",
    "bytes": 1834,
    "sha256": "9a6aff6dbe8b0b19c02b2e0fab20901bf60ac836e17051f1ab8f537bb3894110"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/RESULT.md",
    "bytes": 6979,
    "sha256": "0b8ad63f50b6e110a02a575b500567a5d94b8f6ea09047741834392b323c6d83"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/ISSUES.md",
    "bytes": 3091,
    "sha256": "9fb33c3fdb512aa91629621d056661f76cf01c43e7048cba9944665a7ef26db5"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/codec.d.mts",
    "bytes": 3329,
    "sha256": "eb7dd60101474bdfe2be645e800991820f8f0e10ac0f0ff0b628c250ec99cc72"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/codec.mjs",
    "bytes": 17108,
    "sha256": "c7e9479889d4168f858c147fc28bd8d2bb2be0ac24ec4b0554b61a4bb87041a6"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/codec.test.mjs",
    "bytes": 15221,
    "sha256": "7c6a55e80226be4abe8c618aaba9fe3145123560cd22b0570967bd1e55297090"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/reference-vectors.py",
    "bytes": 7823,
    "sha256": "1acb9190ba5bba016e3241e16a0c4adb67f0bcb40aa6259aeef882f1e32a3af9"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/vectors.json",
    "bytes": 22577,
    "sha256": "ef780924877c2822cdb72ee071fb9f3d8cbb7cb0bbf793a650389571f5cfb3c6"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/protected-inputs.json",
    "bytes": 2276,
    "sha256": "b90ed12bf887687f6f03e10381046bb70d37292d6a8ea7fc23151fb605a5e282"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/result-data.json",
    "bytes": 4471,
    "sha256": "35baac51bb1f3f95dd0e824e37874c03f03d44fa4250c6d2f9af645d3849e188"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/run-experiment.mjs",
    "bytes": 4163,
    "sha256": "e5101bdf66473bb4ee182ae429f924ec4ca8f5ce94f2c61f8bdb86f981684e1a"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/test-results.tap",
    "bytes": 11608,
    "sha256": "c084410b00abc3880fa57d727e77c2daa30c2ffa40b5266475f815c540d72cb4"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/artifact-manifest.json",
    "bytes": 2293,
    "sha256": "e31306c37ba09414dec37fc3bbc794cb112309e064075074f3c3eaf017e0c1a7"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-images/history/pre-grok-repair-03/SPEC.md",
    "bytes": 35762,
    "sha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-images/SPEC.md",
    "bytes": 39237,
    "sha256": "7f6ebf64002d8cbae45de22ed759eb71dce919735c4160deb014bc0673e7232d"
  }
]
```

## experiments/moriarty-language/formal/mil4/hash-image-codec/PLAN.md

sha256: `9a6aff6dbe8b0b19c02b2e0fab20901bf60ac836e17051f1ab8f537bb3894110`

```text
# Local W-D2H image-codec experiment

Status: local executable experiment; no adopted image suite or B05–B07 gate claim.

Source: `../hash-images/SPEC.md`, repair-02, SHA256
`52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc`.
The repair-02 frozen packet records the same digest. Historical
`pre-grok-repair-02` contains the earlier candidate and is not this input.
The parent reported that repair-03 is being prepared for two consumer-order
review findings. This content experiment stays pinned to repair-02 and needs
comparison to repair-03 before any design claim. The executable runner reads
the frozen repair-02 packet, so an advancing live SPEC is not silently selected.

Implement purposes 1, 3 and 4 in a standalone Node module. Closed records,
nominal identifier wrappers, canonical decimal strings, exact constants and
explicit ordered file-byte inputs define admission. Produce payload, envelope
and SHA256; compare content claims without authentication. Compose a policy
with freshly computed Source and Core digests to demonstrate acyclic production.
Do not supply purpose 2, an artifact decoder, AST parser, provider, registry,
full consumer, execution correspondence or ledger behavior.

Write targeted tests first; freeze separate Python serializer byte/digest
vectors, run Node tests against them, then run an actual three-module package
comparison. Record bounds, limitations, measured results and input/output hashes
here. Synthetic package vectors deliberately test arbitrary raw bytes and do
not assert that those bytes export the declared entry names. Actual export,
dependency closure and loaded-artifact evidence remain separate B06 obligations.

All writes stay in this directory. No commit or merge. Existing reviewed
documents, audit packets, source modules and fixtures remain input-only.

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/RESULT.md

sha256: `0b8ad63f50b6e110a02a575b500567a5d94b8f6ea09047741834392b323c6d83`

```text
# W-D2H local image-codec experiment result

Status: **local executable experiment, 2026-09-30**. The proposed suite is
not adopted. B05–B07, W-D2/W-D3 and Sprint 1 remain open. No authenticated
consumer, signature, native proof or ledger acceptance result is supplied.
No commit or merge was performed.

## Frozen proposal and scope

The codec is pinned to **hash-images repair-02 SPEC**, SHA256
`52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc`.
Its exact bytes are embedded in the immutable repair-02 candidate packet,
SHA256 `daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9`.
The runner verifies the packet and embedded SPEC bytes, rather than silently
selecting a newer live proposal.

The parent reported that repair-03 is being prepared for two medium static
consumer-order findings at tag35 debtor/creditor and tag16/17 B05/B07 fact
precedence. **Comparison to repair-03 is required before any design claim.**
Those consumer anchors are outside this experiment. Prior design review does
not approve these executable bytes.

All authored files are under this `hash-image-codec/` directory. The existing
hash-images documents, audit packets and source modules were read only.
[Protected input hashes](protected-inputs.json) record six matching frozen
inputs at the measurement point: three repair-02 documents and three modules.
Another agent's subsequent design revision must not be mistaken for a codec
write or adoption.

## Delivered behavior

[codec.mjs](codec.mjs) produces canonical payload, envelope and SHA256 for
purposes 1 (Source definition), 3 (explicit raw implementation package) and
4 (policy). [codec.d.mts](codec.d.mts) declares the closed records and distinct
nominal identifier roles. Runtime admission rejects extra fields, accessors,
symbols, unsupported constants/selector pairs, malformed/collapsed ID roles,
noncanonical decimal strings and out-of-range amounts/rounds/scales.

Text is exact decoded Unicode scalar content, without repair or normalization.
Policy keyRef enforces nonempty text, simultaneous 1024-byte/1024-code-unit
bounds and the Core opaque control exclusions. Hashes present as lowercase
64-character hex and encode as exactly 32 raw bytes. Every closed empty/none
constant is explicit in the policy payload.

Package files are three caller-supplied role-ordered byte arrays, each capped
at 1MiB, with the full payload capped at 4MiB. Paths and filesystem metadata
are absent. Raw bytes, line endings, BOM and invalid UTF-8 bytes are preserved.
Intrinsic typed-array slots enforce byte caps and exact subview contents, so
shadowed JavaScript length/buffer properties cannot bypass bounds. Source and
policy payloads enforce the specified 4096-byte caps before final allocation.

`produceImages` computes Source and Core first, constructs the policy with those
computed links, and requires coherent selected Source/package identities.
`compareContent` reports matching content with `authenticated:false` and
`scope:"content-only"`; it issues no B05/B06/B07 or tag8/11/12 result.
See [issues and boundaries](ISSUES.md) for policy-body admission/context
separation and the unimplemented Source formation/artifact transport obligations.

## Executed observations

Node `v24.21.0`: **72 tests passed, 0 failed, 0 skipped** with
`node --test --test-reporter=tap codec.test.mjs`. The complete retained output
is [test-results.tap](test-results.tap). The initial API assertion failed before
the codec existed. Subsequent negative controls reproduced a shadowed-byte-cap
bypass and a shadowed-buffer getter before the intrinsic-slot repair; both now
pass. Node syntax checks exited 0 for codec and runner. The declaration file
has not received a separate TypeScript compile check.

[reference-vectors.py](reference-vectors.py) generated expected bytes before
the Node producer was implemented. [vectors.json](vectors.json) freezes nine
complete payload/envelope/digest vectors (transfer, repayment, Unicode policy,
each with purposes 1/3/4), plus two actual-module package digest/length vectors.
Python `struct`/`to_bytes` and Node `Buffer` are separate serializers by the
same author, **not an independent person/provider audit**.

Tests cover nominal/round boundaries, reserved/common IDs, version/constructor
selection, decoded controls/surrogates, exact text caps, Unicode distinctions,
hash shape, retention constants, package role/count/size/raw bytes, acyclic
links, identity content differences, zero-fee recipient commitment, explicit
purpose rejection and content comparison. The actual modules total 41303
raw bytes and agree with both frozen Python package digests.

[run-experiment.mjs](run-experiment.mjs) exited 0 and retained
[result-data.json](result-data.json). These are projected typed input examples,
not newly parsed Source/full-consumer positives.

| Typed example | Source digest | Actual package digest | Linked policy digest |
| --- | --- | --- | --- |
| Transfer | `07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3` | `21a5a5c01858be797ee49894c02dceb777a1ad31d70d196cddc93d423804e585` | `6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e` |
| Repayment | `dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78` | `6e153b337f696bcb9f7ec0bcd84e3dcf5a3bac70b604cd417836e65178e2b425` | `09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc` |

For both examples exact policy content matches; increasing the action amount
by one fails content equality. Transfer Source/Core/policy payload sizes are
87/41446/274 bytes; repayment sizes are 86/41445/254 bytes. Purpose-separated
envelopes add exactly 27 bytes. These are local observations, not empirical
wallet/proof/ledger limits or an injectivity theorem for SHA256.

## Reproduction and handoff

From this directory:

```bash
python3 reference-vectors.py
node --test --test-reporter=tap codec.test.mjs
node run-experiment.mjs
```

Regenerating vectors reads the current three module inputs and changes their
freeze if those files change; retain the recorded manifest before doing so.
The runner refuses changed packet or module inputs. Test measurements become
stale after changes to codec/tests/vectors or the actual package files.
[artifact-manifest.json](artifact-manifest.json) pins the local handoff bytes.
The task's directory restriction takes precedence over the checkpoint skill's
repository `.foreman/session.db` destination; local machine-readable results,
commands and input/output hash scopes provide the bounded handoff record.

Remaining work includes repair-03 comparison, fresh independent executable
reviews, successful Source formation/full D sweep, an artifact decoder and
authenticated suite selection, provider provenance/selection, actual export/
closure/loaded-artifact/toolchain/lowering correspondence, a real B16 handoff,
signature verification, native proof and ledger evidence. None is discharged
by this experiment's content digest agreement.

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/ISSUES.md

sha256: `9fb33c3fdb512aa91629621d056661f76cf01c43e7048cba9944665a7ef26db5`

```text
# Experiment boundaries and unresolved specification issues

Status: local experiment, not an adopted codec/consumer contract.
The candidate is pinned to repair-02. The parent reported a newer repair-03
for two static consumer-order findings (tag35 debtor/creditor and tag16/17
B05/B07 fact precedence). These are not codec/provider results. Comparison
with repair-03 is required before any design claim; this experiment does not
implement those consumer anchors.

1. **Policy identity domain versus context.** The policy prose references mapping A,
   while H-H20 requires a genuine policy Core ID differing from earlier context
   to survive image formation and fail a later body comparison. The local policy
   serializer checks the Core ID's nominal sort and common identifier subtype.
   It does not assert its equality to external Source/Core selection. Candidate
   composition separately derives both IDs from the checked Source/package
   inputs and requires those inputs to agree. The test for a differing standalone
   policy Core ID establishes different encoded content only. A full artifact
   admission or tag12 judgment remains unresolved and is not implemented.

2. **Actual package correspondence.** Fixed labels and role bytes can be serialized
   for arbitrary raw bytes. No lexical export scan can establish that those bytes
   implement the selected functions, have the same declared semantics, form a
   complete module closure or were loaded/executed. Synthetic file vectors test
   bytes only. Actual current-file vectors identify the exact files read; they
   do not discharge the spec's separate B06 correspondence obligations.

3. **Artifact decoder and diagnostics.** The proposal does not select general
   image-parser codes or artifact transport. This experiment has a producer and
   a content comparator over closed projected records, with a local exception
   class and field paths. It does not invent a decoder, consumer anchors or
   tag8/11/12 diagnostic schedule. The explicitly specified standalone candidate
   keyRef domain error is retained with null consumer anchor/fact path and null
   published post/effects. These local errors cannot be reclassified as an
   authenticated consumer result.

4. **Typed projection boundary.** The producer accepts already projected records.
   Source version/profile, selector/constructor, included identifier domains,
   scales, policy rounds/nominals and decoded keyRef are checked. It does not
   establish successful Source formation, validate excluded AST fields or run
   W-D2F's complete D sweep. A future Source adapter must parse/check the whole
   candidate before projecting and must derive the kind from the signed action.
   No caller claim of successful formation supplies that missing adapter.

5. **Vector independence.** Python struct/to_bytes serialization and Node Buffer
   serialization are separate implementations written by this experiment's
   author. Their agreement detects implementation differences; it is not a
   separate-person/provider audit or a proof of unique SHA256 images.

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/codec.d.mts

sha256: `eb7dd60101474bdfe2be645e800991820f8f0e10ac0f0ff0b628c250ec99cc72`

```text
/// <reference types="node" />
/** Closed proposed image records. Runtime admission also rejects extra fields. */
export type IdSort = 'agreement' | 'domain' | 'asset' | 'action' | 'coreProgram' | 'signer' | 'account' | 'obligation';
export type Id<S extends IdSort> = Readonly<{sort: S; value: string}>;
/** Canonical unsigned decimal text; exact domain checked at runtime. */
export type Decimal = string;
export type Kind = 'Transfer' | 'Repay';
export interface SourceDefinition {
  sourceVersion: 6; profile: 'moriarty-financial-agreement-source/6'; wireProfile: 1;
  agreementInstanceId: Id<'agreement'>; domain: Id<'domain'>; asset: Id<'asset'>;
  scale: Decimal; selectedActionId: Id<'action'>; operationKind: Kind;
}
export interface CorePackage {
  coreVersion: 5; coreProfile: 'moriarty-core/5'; sourceVersion: 6; intentSchema: 'moriarty-intent/3'; wireProfile: 1;
  coreProgramId: Id<'coreProgram'>; operationKind: Kind;
  lowerEntry: 'parseAndLowerSource6'; prepareEntry: 'prepareMil4S0'; wrapperEntry: 'prepareSource6S0Unqualified';
  fileCount: 3; files: [{role: 1; bytes: Uint8Array}, {role: 2; bytes: Uint8Array}, {role: 3; bytes: Uint8Array}];
}
export type Operation = {kind: 'Transfer'; owner: Id<'account'>; recipient: Id<'account'>;
  feeRecipient: Id<'account'>; amount: Decimal; fee: Decimal} |
  {kind: 'Repay'; obligationId: Id<'obligation'>; payer: Id<'account'>; amount: Decimal;
    allocation: 'AccrualFirst'; conversion: 'identity'};
export interface PolicyTerms {
  signer: Id<'signer'>; keyRef: string; validFrom: Decimal; validUntil: Decimal;
  grossCap: Decimal; feeCap: Decimal; netFloor: Decimal; failureRelation: 'success_only';
  supplyChanges: 'empty'; observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty';
  retainedDuties: 'empty'; delegation: 'none'; recovery: 'none'; operation: Operation;
}
export interface Policy extends PolicyTerms {
  sourceVersion: 6; coreVersion: 5; wireProfile: 1; agreementInstanceId: Id<'agreement'>;
  domain: Id<'domain'>; asset: Id<'asset'>; scale: Decimal; actionId: Id<'action'>;
  coreProgramId: Id<'coreProgram'>; operationKind: Kind; sourceHash: string; coreHash: string;
}
export interface Image {purpose: 1 | 3 | 4; payload: Buffer; preimage: Buffer; digest: string}
export const SOURCE_PROFILE: 'moriarty-financial-agreement-source/6';
export class ImageCodecError extends Error {field: string; scope: 'local-image-domain'}
export function id<S extends IdSort>(sort: S, value: string): Id<S>;
export function encodeImage(purpose: 1, value: SourceDefinition): Image;
export function encodeImage(purpose: 3, value: CorePackage): Image;
export function encodeImage(purpose: 4, value: Policy): Image;
export function produceImages(definition: SourceDefinition, corePackage: CorePackage, terms: PolicyTerms):
  {source: Image; core: Image; policy: Image; policyValue: Policy};
export interface ContentComparison {matches: boolean; digest: string; claimedDigest: string;
  scope: 'content-only'; authenticated: false}
export function compareContent(purpose: 1, value: SourceDefinition, claimedDigest: string): ContentComparison;
export function compareContent(purpose: 3, value: CorePackage, claimedDigest: string): ContentComparison;
export function compareContent(purpose: 4, value: Policy, claimedDigest: string): ContentComparison;

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/codec.mjs

sha256: `c7e9479889d4168f858c147fc28bd8d2bb2be0ac24ec4b0554b61a4bb87041a6`

```text
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
export function encodeImage(purpose, value) {
  const payload = purpose === 1 ? sourcePayload(value) : purpose === 3 ? corePayload(value) : purpose === 4 ? policyPayload(value) :
    reject('purpose', 'only purposes 1, 3 and 4 are available in this experiment');
  const length = Buffer.alloc(4); length.writeUInt32BE(payload.length);
  const preimage = Buffer.concat([PREFIX, Buffer.from([purpose]), length, payload]);
  return {purpose, payload, preimage, digest: createHash('sha256').update(preimage).digest('hex')};
}

/** Link policy to computed content hashes, never to supplied Source claim slots. */
export function produceImages(definition, corePackage, terms) {
  const source = encodeImage(1, definition);
  const core = encodeImage(3, corePackage);
  record(terms, TERM_KEYS, 'intent');
  if (definition.selectedActionId.value !== corePackage.coreProgramId.value || definition.operationKind !== corePackage.operationKind)
    reject('package.coreProgramId', 'selected Source/package identity differs');
  const policyValue = {sourceVersion: 6, coreVersion: 5, wireProfile: 1,
    agreementInstanceId: definition.agreementInstanceId, domain: definition.domain, asset: definition.asset, scale: definition.scale,
    actionId: definition.selectedActionId, coreProgramId: corePackage.coreProgramId, operationKind: definition.operationKind,
    sourceHash: source.digest, coreHash: core.digest,
    signer: terms.signer, keyRef: terms.keyRef, validFrom: terms.validFrom, validUntil: terms.validUntil,
    grossCap: terms.grossCap, feeCap: terms.feeCap, netFloor: terms.netFloor, failureRelation: terms.failureRelation,
    supplyChanges: terms.supplyChanges, observations: terms.observations, disclosures: terms.disclosures,
    retainedEffects: terms.retainedEffects, retainedDuties: terms.retainedDuties, delegation: terms.delegation,
    recovery: terms.recovery, operation: terms.operation};
  let policy;
  try { policy = encodeImage(4, policyValue); } catch (error) {
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

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/codec.test.mjs

sha256: `7c6a55e80226be4abe8c618aaba9fe3145123560cd22b0570967bd1e55297090`

```text
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

let codec;
try { codec = await import('./codec.mjs'); } catch (error) {
  if (error.code !== 'ERR_MODULE_NOT_FOUND') throw error;
}
const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url)));
const fixture = (index = 0) => {
  const value = structuredClone(vectors.vectors[index].inputs);
  value.package.files = value.package.filesHex.map((hex, i) => ({role: i + 1, bytes: Buffer.from(hex, 'hex')}));
  delete value.package.filesHex;
  return value;
};

test('codec implements the proposed closed purpose 1/3/4 API', () => {
  assert.ok(codec, 'the scoped codec API must exist');
  for (const name of ['id', 'encodeImage', 'produceImages', 'compareContent']) assert.equal(typeof codec[name], 'function');
});

for (const [index, vector] of vectors.vectors.entries()) {
  test(`${vector.name}: payload, envelope and digest equal frozen Python vectors`, () => {
    const input = fixture(index);
    const images = codec.produceImages(input.definition, input.package, input.terms);
    for (const name of ['source', 'core', 'policy']) {
      assert.equal(images[name].payload.toString('hex'), vector.images[name].payloadHex);
      assert.equal(images[name].preimage.toString('hex'), vector.images[name].preimageHex);
      assert.equal(images[name].digest, vector.images[name].sha256);
      assert.equal(images[name].payload.length, vector.images[name].payloadBytes);
    }
    assert.deepEqual(images.policyValue, vector.policyInput);
  });
}

const sourceRejection = (label, mutate) => test(label, () => {
  const { definition } = fixture(); mutate(definition);
  assert.throws(() => codec.encodeImage(1, definition), codec.ImageCodecError);
});
sourceRejection('extra fields cannot enter a Source image', x => { x.sourceHash = '00'.repeat(32); });
sourceRejection('agreement and action nominal sorts stay distinct', x => { x.agreementInstanceId.sort = 'action'; });
sourceRejection('reserved Source IDs reject', x => { x.domain.value = 'domain'; });
sourceRejection('wire-only identifier punctuation rejects', x => { x.domain.value = 'Preview:a'; });
sourceRejection('initial digits reject', x => { x.domain.value = '1Preview'; });
sourceRejection('non-ASCII IDs reject', x => { x.domain.value = 'Pr\u00e9view'; });
sourceRejection('ID length over 64 rejects', x => { x.domain.value = 'A'.repeat(65); });
sourceRejection('scale canonical leading zeros reject', x => { x.scale = '00'; });
sourceRejection('scale 19 rejects', x => { x.scale = '19'; });
sourceRejection('numeric coercion rejects', x => { x.scale = 0; });
sourceRejection('old Source version rejects', x => { x.sourceVersion = 5; });
sourceRejection('old Source profile rejects', x => { x.profile = 'moriarty-financial-agreement-source/5'; });
sourceRejection('selector and signed constructor disagreement rejects', x => { x.operationKind = 'Repay'; });
sourceRejection('closed records reject getters', x => { Object.defineProperty(x, 'domain', {get() { throw Error('must not run'); }, enumerable: true}); });
sourceRejection('closed records reject symbols', x => { x[Symbol('hidden')] = 1; });

const policyRejection = (label, mutate) => test(label, () => {
  const policy = structuredClone(vectors.vectors[0].policyInput); mutate(policy);
  assert.throws(() => codec.encodeImage(4, policy), codec.ImageCodecError);
});
policyRejection('policy rejects empty keyRef', x => { x.keyRef = ''; });
policyRejection('policy rejects lone high surrogate', x => { x.keyRef = '\ud800'; });
policyRejection('policy rejects lone low surrogate', x => { x.keyRef = '\udfff'; });
policyRejection('decoded NUL control rejects', x => { x.keyRef = JSON.parse('"\\u0000"'); });
policyRejection('decoded DEL control rejects', x => { x.keyRef = '\x7f'; });
policyRejection('UTF-8 byte cap applies to multibyte keyRef', x => { x.keyRef = '\u00e9'.repeat(513); });
policyRejection('UTF-16 keyRef cap rejects before encoding', x => { x.keyRef = 'k'.repeat(1025); });
policyRejection('round 2^64 rejects instead of truncating', x => { x.validUntil = '18446744073709551616'; });
policyRejection('round leading zeros reject', x => { x.validFrom = '01'; });
policyRejection('reversed validity interval rejects', x => { x.validFrom = '2'; x.validUntil = '1'; });
policyRejection('nominal 2^127 rejects', x => { x.grossCap = '170141183460469231731687303715884105728'; });
policyRejection('negative nominal rejects', x => { x.feeCap = '-1'; });
policyRejection('unbounded integer text rejects', x => { x.grossCap = '1'.repeat(10000); });
policyRejection('float nominal rejects', x => { x.netFloor = '1.0'; });
policyRejection('uppercase hash text rejects', x => { x.sourceHash = 'AA'.repeat(32); });
policyRejection('short hash text rejects', x => { x.coreHash = 'ab'; });
policyRejection('nominal action/core interchange rejects', x => { x.actionId.sort = 'coreProgram'; });
policyRejection('accepted failure variant rejects', x => { x.failureRelation = 'accepted_failure'; });
policyRejection('nonempty retention rejects', x => { x.retainedDuties = []; });
policyRejection('operation variant disagreement rejects', x => { x.operation = structuredClone(vectors.vectors[1].policyInput.operation); });
policyRejection('arbitrary dynamic policy fields reject', x => { x.operation.creditor = {sort: 'account', value: 'Bob'}; });

test('repayment requires the explicit allocation and identity conversion constants', () => {
  for (const key of ['allocation', 'conversion']) {
    const value = structuredClone(vectors.vectors[1].policyInput);
    value.operation[key] = 'other';
    assert.throws(() => codec.encodeImage(4, value), codec.ImageCodecError);
  }
});

test('keyRef 1024 ASCII bytes and 512 two-byte scalars admit exactly', () => {
  for (const keyRef of ['k'.repeat(1024), '\u00e9'.repeat(512)]) {
    const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = keyRef;
    assert.ok(codec.encodeImage(4, value).payload.length < 4096);
  }
});

test('keyRef preserves Unicode composition, whitespace, case and literal escape content', () => {
  const hashes = ['\u00e9', 'e\u0301', ' key ', 'key', 'Key', '\\u0000'].map(keyRef => {
    const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = keyRef;
    return codec.encodeImage(4, value).digest;
  });
  assert.equal(new Set(hashes).size, hashes.length);
});

test('equal nominal text is admitted separately in agreement/domain roles', () => {
  const { definition } = fixture();
  definition.agreementInstanceId = codec.id('agreement', 'Same');
  definition.domain = codec.id('domain', 'Same');
  assert.ok(codec.encodeImage(1, definition));
});

const coreRejection = (label, mutate) => test(label, () => {
  const { package: value } = fixture(); mutate(value);
  assert.throws(() => codec.encodeImage(3, value), codec.ImageCodecError);
});
coreRejection('file count must be exactly three', x => { x.fileCount = 2; });
coreRejection('file role order must be 1/2/3', x => { x.files.reverse(); });
coreRejection('extra package files reject', x => { x.files.push({role: 4, bytes: Buffer.alloc(0)}); });
coreRejection('file over 1MiB rejects', x => { x.files[0].bytes = Buffer.alloc(1048577); });
coreRejection('file path cannot replace explicit bytes', x => { x.files[0].bytes = './front.ts'; });
coreRejection('package metadata is outside the image', x => { x.files[0].path = './front.ts'; });
coreRejection('stale package entry labels reject', x => { x.lowerEntry = 'renamed'; });
coreRejection('wrong Core profile rejects', x => { x.coreProfile = 'moriarty-core/4'; });
coreRejection('wrong intent schema rejects', x => { x.intentSchema = 'moriarty-intent/4'; });
coreRejection('shared file backing memory rejects', x => { x.files[0].bytes = new Uint8Array(new SharedArrayBuffer(4)); });

test('all three files at exactly 1MiB admit below total 4MiB cap', () => {
  const { package: value } = fixture();
  value.files = [1, 2, 3].map(role => ({role, bytes: Buffer.alloc(1048576, role)}));
  assert.ok(codec.encodeImage(3, value).payload.length < 4194304);
});

test('every file byte, line ending, BOM and unused branch changes the package digest', () => {
  const { package: value } = fixture(); const original = codec.encodeImage(3, value).digest;
  for (const index of [0, 1, 2]) {
    const changed = fixture().package;
    changed.files[index].bytes = Buffer.concat([changed.files[index].bytes, Buffer.from('// unused repayment branch\r\n\ufeff')]);
    assert.notEqual(codec.encodeImage(3, changed).digest, original);
  }
  value.files[0].bytes = Buffer.from(value.files[0].bytes.toString().replace('\r\n', '\n'));
  assert.notEqual(codec.encodeImage(3, value).digest, original);
});

test('producer recomputes links and rejects incoherent selected package identity', () => {
  const input = fixture();
  input.package.coreProgramId = codec.id('coreProgram', 'RepayAccrualFirst');
  input.package.operationKind = 'Repay';
  assert.throws(() => codec.produceImages(input.definition, input.package, input.terms), codec.ImageCodecError);
});

test('content comparator states its content-only scope for equal and unequal claims', () => {
  const { definition } = fixture(); const digest = vectors.vectors[0].images.source.sha256;
  assert.deepEqual(codec.compareContent(1, definition, digest), {matches: true, digest, claimedDigest: digest,
    scope: 'content-only', authenticated: false});
  assert.equal(codec.compareContent(1, definition, '00'.repeat(32)).matches, false);
  assert.throws(() => codec.compareContent(1, definition, digest.toUpperCase()), codec.ImageCodecError);
});

test('no purpose 2, unknown purpose or implicit suite search is available', () => {
  for (const purpose of [0, 2, 5, '1']) assert.throws(() => codec.encodeImage(purpose, fixture().definition), codec.ImageCodecError);
});

test('object enumeration order does not determine bytes', () => {
  const { definition } = fixture();
  const reverse = Object.fromEntries(Object.entries(definition).reverse());
  assert.deepEqual(codec.encodeImage(1, reverse).preimage, codec.encodeImage(1, definition).preimage);
});

test('policy amount and zero-fee recipient have independent content effects', () => {
  const original = structuredClone(vectors.vectors[0].policyInput);
  const digest = codec.encodeImage(4, original).digest;
  for (const edit of [x => {x.operation.amount = '12';}, x => {x.operation.feeRecipient.value = 'Other';}]) {
    const value = structuredClone(original); edit(value); assert.notEqual(codec.encodeImage(4, value).digest, digest);
  }
});

test('actual current package bytes match independently frozen transfer/repayment digests', () => {
  for (const expected of vectors.actualPackages) {
    const value = fixture(expected.operationKind === 'Transfer' ? 0 : 1).package;
    value.files = vectors.actualModuleInputs.map(input => ({role: input.role,
      bytes: readFileSync(new URL(`../../../src/successor/${input.name}`, import.meta.url))}));
    const result = codec.encodeImage(3, value);
    assert.equal(result.payload.length, expected.payloadBytes);
    assert.equal(result.digest, expected.sha256);
  }
});

test('standalone policy body identity differs without a consumer context assertion', () => {
  const value = structuredClone(vectors.vectors[0].policyInput);
  value.coreProgramId = codec.id('coreProgram', 'RepayAccrualFirst');
  const encoded = codec.encodeImage(4, value);
  assert.notEqual(encoded.digest, vectors.vectors[0].images.policy.sha256);
  assert.equal(codec.compareContent(4, value, encoded.digest).authenticated, false);
});

test('standalone producer keyRef domain error has no consumer anchor', () => {
  const input = fixture(); input.terms.keyRef = '\x1f';
  assert.throws(() => codec.produceImages(input.definition, input.package, input.terms), error => {
    assert.equal(error.code, 'W_D2F_DOMAIN_UNSUPPORTED');
    assert.equal(error.classification, 'BindingRejected');
    assert.equal(error.field, 'policy.keyRef');
    assert.equal(error.inputPath, 'intent.keyRef');
    assert.equal(error.sourcePath, 'intent.keyRef');
    assert.equal(error.comparisonTag, null);
    assert.equal(error.factPath, null);
    assert.equal(error.publishedPost, null);
    assert.equal(error.publishedEffects, null);
    return true;
  });
});

test('maximum 64-byte IDs and scale 18 encode without repair', () => {
  const {definition} = fixture();
  definition.domain = codec.id('domain', 'A'.repeat(64)); definition.scale = '18';
  const encoded = codec.encodeImage(1, definition);
  assert.ok(encoded.payload.length < 4096);
  assert.ok(encoded.payload.includes(Buffer.from('A'.repeat(64), 'ascii')));
});

test('file array extra properties and accessors reject without executing accessors', () => {
  const input = fixture().package; input.files.extra = true;
  assert.throws(() => codec.encodeImage(3, input), codec.ImageCodecError);
  const next = fixture().package;
  Object.defineProperty(next.files, '0', {get() { throw new Error('must not execute'); }});
  assert.throws(() => codec.encodeImage(3, next), codec.ImageCodecError);
});

test('package uses exactly the passed Uint8Array subview bytes', () => {
  const value = fixture().package;
  const view = new Uint8Array([99, 0, 255, 239, 187, 191, 99]).subarray(1, 6);
  value.files[1].bytes = view;
  assert.equal(codec.encodeImage(3, value).digest, vectors.vectors[0].images.core.sha256);
});

test('decoded astral scalar length observes UTF-8 cap simultaneously', () => {
  const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = '\ud83d\ude00'.repeat(256);
  assert.ok(codec.encodeImage(4, value));
  value.keyRef += '\ud83d\ude00';
  assert.throws(() => codec.encodeImage(4, value), codec.ImageCodecError);
});

test('cross-purpose values cannot be coerced into the other closed records', () => {
  const input = fixture(); const policy = structuredClone(vectors.vectors[0].policyInput);
  for (const [purpose, value] of [[1, input.package], [3, input.definition], [4, input.definition], [1, policy]])
    assert.throws(() => codec.encodeImage(purpose, value), codec.ImageCodecError);
});

test('returned buffers do not alias caller package bytes', () => {
  const value = fixture().package; const image = codec.encodeImage(3, value);
  const expected = Buffer.from(image.preimage); value.files[0].bytes.fill(0);
  assert.deepEqual(image.preimage, expected);
});

test('shadowed byte lengths cannot bypass the exact file cap', () => {
  const input = fixture().package;
  const bytes = new Uint8Array(1048577);
  Object.defineProperty(bytes, 'byteLength', {value: 0});
  input.files[0].bytes = bytes;
  assert.throws(() => codec.encodeImage(3, input), codec.ImageCodecError);
});

test('byte buffer shadow properties do not change the intrinsic view content', () => {
  const input = fixture().package;
  const bytes = new Uint8Array([0, 255, 239, 187, 191]);
  Object.defineProperties(bytes, {length: {value: 0}, byteOffset: {value: 12345}, buffer: {get() { throw Error('must not execute'); }}});
  input.files[1].bytes = bytes;
  assert.equal(codec.encodeImage(3, input).digest, vectors.vectors[0].images.core.sha256);
});

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/reference-vectors.py

sha256: `1acb9190ba5bba016e3241e16a0c4adb67f0bcb40aa6259aeef882f1e32a3af9`

```text
"""Separate struct-based expected-byte generator; never imports the Node codec.

Synthetic package files only test serialization. The actual package vectors
read the three explicit reviewed module paths without changing them.
This is a second implementation by the same author, not an independent audit.
"""
import hashlib
import json
from pathlib import Path
import struct

BASE = Path(__file__).resolve().parent
SOURCE_PROFILE = "moriarty-financial-agreement-source/6"
PREFIX = b"moriarty-mil4-image/1\x00"


def nominal(sort, value):
    return {"sort": sort, "value": value}


def identifier(value):
    raw = value["value"].encode("ascii")
    return struct.pack(">H", len(raw)) + raw


def text(value):
    raw = value.encode("utf-8", errors="strict")
    return struct.pack(">H", len(raw)) + raw


def number(value, width):
    return int(value).to_bytes(width, "big", signed=False)


def definition(value):
    return (b"\x06" + text(SOURCE_PROFILE) + b"\x01"
            + identifier(value["agreementInstanceId"]) + identifier(value["domain"])
            + identifier(value["asset"]) + number(value["scale"], 1)
            + identifier(value["selectedActionId"])
            + bytes([1 if value["operationKind"] == "Transfer" else 2]))


def package(value):
    raw = (b"\x05" + text("moriarty-core/5") + b"\x06"
           + text("moriarty-intent/3") + b"\x01"
           + identifier(value["coreProgramId"])
           + bytes([1 if value["operationKind"] == "Transfer" else 2])
           + text("parseAndLowerSource6") + text("prepareMil4S0")
           + text("prepareSource6S0Unqualified") + b"\x00\x03")
    for role, file_hex in enumerate(value["filesHex"], start=1):
        content = bytes.fromhex(file_hex)
        raw += bytes([role]) + struct.pack(">I", len(content)) + content
    return raw


def policy(value):
    raw = (b"\x06\x05\x01" + identifier(value["agreementInstanceId"])
           + identifier(value["domain"]) + identifier(value["asset"])
           + number(value["scale"], 1) + identifier(value["actionId"])
           + identifier(value["coreProgramId"])
           + bytes([1 if value["operationKind"] == "Transfer" else 2])
           + bytes.fromhex(value["sourceHash"]) + bytes.fromhex(value["coreHash"])
           + identifier(value["signer"]) + text(value["keyRef"])
           + number(value["validFrom"], 8) + number(value["validUntil"], 8)
           + number(value["grossCap"], 16) + number(value["feeCap"], 16)
           + number(value["netFloor"], 16) + b"\x01"
           + b"\x00\x00" * 5 + b"\x00\x00")
    op = value["operation"]
    if op["kind"] == "Transfer":
        raw += (b"\x01" + identifier(op["owner"]) + identifier(op["recipient"])
                + identifier(op["feeRecipient"]) + number(op["amount"], 16)
                + number(op["fee"], 16))
    else:
        raw += (b"\x02" + identifier(op["obligationId"]) + identifier(op["payer"])
                + number(op["amount"], 16) + b"\x01\x01")
    return raw


def image(purpose, payload):
    envelope = PREFIX + bytes([purpose]) + struct.pack(">I", len(payload)) + payload
    return {"purpose": purpose, "payloadHex": payload.hex(),
            "preimageHex": envelope.hex(), "payloadBytes": len(payload),
            "sha256": hashlib.sha256(envelope).hexdigest()}


def inputs(kind, unicode_key=False):
    selected = "TransferLiteralFee" if kind == "Transfer" else "RepayAccrualFirst"
    source = {"sourceVersion": 6, "profile": SOURCE_PROFILE, "wireProfile": 1,
              "agreementInstanceId": nominal("agreement", "Agreement1"),
              "domain": nominal("domain", "Preview"), "asset": nominal("asset", "A"),
              "scale": "0", "selectedActionId": nominal("action", selected),
              "operationKind": kind}
    core = {"coreVersion": 5, "coreProfile": "moriarty-core/5", "sourceVersion": 6,
            "intentSchema": "moriarty-intent/3", "wireProfile": 1,
            "coreProgramId": nominal("coreProgram", selected), "operationKind": kind,
            "lowerEntry": "parseAndLowerSource6", "prepareEntry": "prepareMil4S0",
            "wrapperEntry": "prepareSource6S0Unqualified", "fileCount": 3,
            "filesHex": ["2f2f2066726f6e740d0a", "00ffefbbbf", ""]}
    terms = {"signer": nominal("signer", "Alice"), "keyRef": "key:e\u0301\U0001f600" if unicode_key else "key1",
             "validFrom": "0", "validUntil": "18446744073709551615",
             "grossCap": "170141183460469231731687303715884105727", "feeCap": "0",
             "netFloor": "1", "failureRelation": "success_only", "supplyChanges": "empty",
             "observations": "empty", "disclosures": "empty", "retainedEffects": "empty",
             "retainedDuties": "empty", "delegation": "none", "recovery": "none"}
    terms["operation"] = ({"kind": "Transfer", "owner": nominal("account", "Alice"),
                           "recipient": nominal("account", "Bob"),
                           "feeRecipient": nominal("account", "Fees"), "amount": "11", "fee": "0"}
                          if kind == "Transfer" else
                          {"kind": "Repay", "obligationId": nominal("obligation", "Loan1"),
                           "payer": nominal("account", "Alice"), "amount": "7",
                           "allocation": "AccrualFirst", "conversion": "identity"})
    return {"definition": source, "package": core, "terms": terms}


def make_vector(name, data):
    source_image = image(1, definition(data["definition"]))
    core_image = image(3, package(data["package"]))
    s = data["definition"]
    policy_input = {"sourceVersion": 6, "coreVersion": 5, "wireProfile": 1,
                    **{k: s[k] for k in ("agreementInstanceId", "domain", "asset", "scale", "operationKind")},
                    "actionId": s["selectedActionId"], "coreProgramId": data["package"]["coreProgramId"],
                    "sourceHash": source_image["sha256"], "coreHash": core_image["sha256"], **data["terms"]}
    return {"name": name, "inputs": data, "policyInput": policy_input,
            "images": {"source": source_image, "core": core_image,
                       "policy": image(4, policy(policy_input))}}


def main():
    vectors = [make_vector("synthetic-transfer", inputs("Transfer")),
               make_vector("synthetic-repayment", inputs("Repay")),
               make_vector("unicode-transfer", inputs("Transfer", True))]
    module_base = (BASE / "../../../src/successor").resolve()
    names = ["financial-agreement-source-v6-frontend.ts", "mil4-s0-core-v5.ts", "mil4-s0-source-v6.ts"]
    files = [(module_base / name).read_bytes() for name in names]
    actual = []
    for kind in ("Transfer", "Repay"):
        data = inputs(kind)
        data["package"]["filesHex"] = [raw.hex() for raw in files]
        actual.append({"operationKind": kind, **image(3, package(data["package"]))})
    # Full raw module bytes already live in the input tree. Freeze only length/digest
    # for actual package vectors, avoiding a second copy of the reviewed modules.
    for value in actual:
        del value["payloadHex"]
        del value["preimageHex"]
    result = {"status": "local-experiment-not-adopted", "vectorMethod": "separate Python struct serializer; same author; no provider audit",
              "specSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
              "vectors": vectors, "actualPackages": actual,
              "actualModuleInputs": [{"role": i + 1, "name": name, "bytes": len(raw),
                                      "sha256": hashlib.sha256(raw).hexdigest()}
                                     for i, (name, raw) in enumerate(zip(names, files))]}
    (BASE / "vectors.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/vectors.json

sha256: `ef780924877c2822cdb72ee071fb9f3d8cbb7cb0bbf793a650389571f5cfb3c6`

```text
{
  "status": "local-experiment-not-adopted",
  "vectorMethod": "separate Python struct serializer; same author; no provider audit",
  "specSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
  "vectors": [
    {
      "name": "synthetic-transfer",
      "inputs": {
        "definition": {
          "sourceVersion": 6,
          "profile": "moriarty-financial-agreement-source/6",
          "wireProfile": 1,
          "agreementInstanceId": {
            "sort": "agreement",
            "value": "Agreement1"
          },
          "domain": {
            "sort": "domain",
            "value": "Preview"
          },
          "asset": {
            "sort": "asset",
            "value": "A"
          },
          "scale": "0",
          "selectedActionId": {
            "sort": "action",
            "value": "TransferLiteralFee"
          },
          "operationKind": "Transfer"
        },
        "package": {
          "coreVersion": 5,
          "coreProfile": "moriarty-core/5",
          "sourceVersion": 6,
          "intentSchema": "moriarty-intent/3",
          "wireProfile": 1,
          "coreProgramId": {
            "sort": "coreProgram",
            "value": "TransferLiteralFee"
          },
          "operationKind": "Transfer",
          "lowerEntry": "parseAndLowerSource6",
          "prepareEntry": "prepareMil4S0",
          "wrapperEntry": "prepareSource6S0Unqualified",
          "fileCount": 3,
          "filesHex": [
            "2f2f2066726f6e740d0a",
            "00ffefbbbf",
            ""
          ]
        },
        "terms": {
          "signer": {
            "sort": "signer",
            "value": "Alice"
          },
          "keyRef": "key1",
          "validFrom": "0",
          "validUntil": "18446744073709551615",
          "grossCap": "170141183460469231731687303715884105727",
          "feeCap": "0",
          "netFloor": "1",
          "failureRelation": "success_only",
          "supplyChanges": "empty",
          "observations": "empty",
          "disclosures": "empty",
          "retainedEffects": "empty",
          "retainedDuties": "empty",
          "delegation": "none",
          "recovery": "none",
          "operation": {
            "kind": "Transfer",
            "owner": {
              "sort": "account",
              "value": "Alice"
            },
            "recipient": {
              "sort": "account",
              "value": "Bob"
            },
            "feeRecipient": {
              "sort": "account",
              "value": "Fees"
            },
            "amount": "11",
            "fee": "0"
          }
        }
      },
      "policyInput": {
        "sourceVersion": 6,
        "coreVersion": 5,
        "wireProfile": 1,
        "agreementInstanceId": {
          "sort": "agreement",
          "value": "Agreement1"
        },
        "domain": {
          "sort": "domain",
          "value": "Preview"
        },
        "asset": {
          "sort": "asset",
          "value": "A"
        },
        "scale": "0",
        "operationKind": "Transfer",
        "actionId": {
          "sort": "action",
          "value": "TransferLiteralFee"
        },
        "coreProgramId": {
          "sort": "coreProgram",
          "value": "TransferLiteralFee"
        },
        "sourceHash": "07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3",
        "coreHash": "e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e",
        "signer": {
          "sort": "signer",
          "value": "Alice"
        },
        "keyRef": "key1",
        "validFrom": "0",
        "validUntil": "18446744073709551615",
        "grossCap": "170141183460469231731687303715884105727",
        "feeCap": "0",
        "netFloor": "1",
        "failureRelation": "success_only",
        "supplyChanges": "empty",
        "observations": "empty",
        "disclosures": "empty",
        "retainedEffects": "empty",
        "retainedDuties": "empty",
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "Transfer",
          "owner": {
            "sort": "account",
            "value": "Alice"
          },
          "recipient": {
            "sort": "account",
            "value": "Bob"
          },
          "feeRecipient": {
            "sort": "account",
            "value": "Fees"
          },
          "amount": "11",
          "fee": "0"
        }
      },
      "images": {
        "source": {
          "purpose": 1,
          "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656501",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000570600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656501",
          "payloadBytes": 87,
          "sha256": "07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3"
        },
        "core": {
          "purpose": 3,
          "payloadHex": "05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f330100125472616e736665724c69746572616c4665650100147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f3100030000009e05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f330100125472616e736665724c69746572616c4665650100147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "payloadBytes": 158,
          "sha256": "e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e"
        },
        "policy": {
          "purpose": 4,
          "payloadHex": "060501000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656500125472616e736665724c69746572616c4665650107ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e0005416c69636500046b6579310000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000000000000000000101000000000000000000000000010005416c6963650003426f620004466565730000000000000000000000000000000b00000000000000000000000000000000",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f31000400000112060501000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656500125472616e736665724c69746572616c4665650107ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e0005416c69636500046b6579310000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000000000000000000101000000000000000000000000010005416c6963650003426f620004466565730000000000000000000000000000000b00000000000000000000000000000000",
          "payloadBytes": 274,
          "sha256": "990dded61608683fdba39b2b4e6711a0174f0494958e4fe864d22aeef68b0867"
        }
      }
    },
    {
      "name": "synthetic-repayment",
      "inputs": {
        "definition": {
          "sourceVersion": 6,
          "profile": "moriarty-financial-agreement-source/6",
          "wireProfile": 1,
          "agreementInstanceId": {
            "sort": "agreement",
            "value": "Agreement1"
          },
          "domain": {
            "sort": "domain",
            "value": "Preview"
          },
          "asset": {
            "sort": "asset",
            "value": "A"
          },
          "scale": "0",
          "selectedActionId": {
            "sort": "action",
            "value": "RepayAccrualFirst"
          },
          "operationKind": "Repay"
        },
        "package": {
          "coreVersion": 5,
          "coreProfile": "moriarty-core/5",
          "sourceVersion": 6,
          "intentSchema": "moriarty-intent/3",
          "wireProfile": 1,
          "coreProgramId": {
            "sort": "coreProgram",
            "value": "RepayAccrualFirst"
          },
          "operationKind": "Repay",
          "lowerEntry": "parseAndLowerSource6",
          "prepareEntry": "prepareMil4S0",
          "wrapperEntry": "prepareSource6S0Unqualified",
          "fileCount": 3,
          "filesHex": [
            "2f2f2066726f6e740d0a",
            "00ffefbbbf",
            ""
          ]
        },
        "terms": {
          "signer": {
            "sort": "signer",
            "value": "Alice"
          },
          "keyRef": "key1",
          "validFrom": "0",
          "validUntil": "18446744073709551615",
          "grossCap": "170141183460469231731687303715884105727",
          "feeCap": "0",
          "netFloor": "1",
          "failureRelation": "success_only",
          "supplyChanges": "empty",
          "observations": "empty",
          "disclosures": "empty",
          "retainedEffects": "empty",
          "retainedDuties": "empty",
          "delegation": "none",
          "recovery": "none",
          "operation": {
            "kind": "Repay",
            "obligationId": {
              "sort": "obligation",
              "value": "Loan1"
            },
            "payer": {
              "sort": "account",
              "value": "Alice"
            },
            "amount": "7",
            "allocation": "AccrualFirst",
            "conversion": "identity"
          }
        }
      },
      "policyInput": {
        "sourceVersion": 6,
        "coreVersion": 5,
        "wireProfile": 1,
        "agreementInstanceId": {
          "sort": "agreement",
          "value": "Agreement1"
        },
        "domain": {
          "sort": "domain",
          "value": "Preview"
        },
        "asset": {
          "sort": "asset",
          "value": "A"
        },
        "scale": "0",
        "operationKind": "Repay",
        "actionId": {
          "sort": "action",
          "value": "RepayAccrualFirst"
        },
        "coreProgramId": {
          "sort": "coreProgram",
          "value": "RepayAccrualFirst"
        },
        "sourceHash": "dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78",
        "coreHash": "179ebb40f3836951dee93a5b6ebfde2c00d624f4041419dbfdd71c2780c60bb4",
        "signer": {
          "sort": "signer",
          "value": "Alice"
        },
        "keyRef": "key1",
        "validFrom": "0",
        "validUntil": "18446744073709551615",
        "grossCap": "170141183460469231731687303715884105727",
        "feeCap": "0",
        "netFloor": "1",
        "failureRelation": "success_only",
        "supplyChanges": "empty",
        "observations": "empty",
        "disclosures": "empty",
        "retainedEffects": "empty",
        "retainedDuties": "empty",
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "Repay",
          "obligationId": {
            "sort": "obligation",
            "value": "Loan1"
          },
          "payer": {
            "sort": "account",
            "value": "Alice"
          },
          "amount": "7",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "images": {
        "source": {
          "purpose": 1,
          "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e743100075072657669657700014100001152657061794163637275616c466972737402",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000560600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e743100075072657669657700014100001152657061794163637275616c466972737402",
          "payloadBytes": 86,
          "sha256": "dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78"
        },
        "core": {
          "purpose": 3,
          "payloadHex": "05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f3301001152657061794163637275616c46697273740200147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f3100030000009d05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f3301001152657061794163637275616c46697273740200147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "payloadBytes": 157,
          "sha256": "179ebb40f3836951dee93a5b6ebfde2c00d624f4041419dbfdd71c2780c60bb4"
        },
        "policy": {
          "purpose": 4,
          "payloadHex": "060501000a41677265656d656e743100075072657669657700014100001152657061794163637275616c4669727374001152657061794163637275616c466972737402dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78179ebb40f3836951dee93a5b6ebfde2c00d624f4041419dbfdd71c2780c60bb40005416c69636500046b6579310000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000001010000000000000000000000000200054c6f616e310005416c696365000000000000000000000000000000070101",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310004000000fe060501000a41677265656d656e743100075072657669657700014100001152657061794163637275616c4669727374001152657061794163637275616c466972737402dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78179ebb40f3836951dee93a5b6ebfde2c00d624f4041419dbfdd71c2780c60bb40005416c69636500046b6579310000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000001010000000000000000000000000200054c6f616e310005416c696365000000000000000000000000000000070101",
          "payloadBytes": 254,
          "sha256": "a7d35ad928dcc6e784ce24167ad447bbd37d44a59ac87258a5b22751d062b43e"
        }
      }
    },
    {
      "name": "unicode-transfer",
      "inputs": {
        "definition": {
          "sourceVersion": 6,
          "profile": "moriarty-financial-agreement-source/6",
          "wireProfile": 1,
          "agreementInstanceId": {
            "sort": "agreement",
            "value": "Agreement1"
          },
          "domain": {
            "sort": "domain",
            "value": "Preview"
          },
          "asset": {
            "sort": "asset",
            "value": "A"
          },
          "scale": "0",
          "selectedActionId": {
            "sort": "action",
            "value": "TransferLiteralFee"
          },
          "operationKind": "Transfer"
        },
        "package": {
          "coreVersion": 5,
          "coreProfile": "moriarty-core/5",
          "sourceVersion": 6,
          "intentSchema": "moriarty-intent/3",
          "wireProfile": 1,
          "coreProgramId": {
            "sort": "coreProgram",
            "value": "TransferLiteralFee"
          },
          "operationKind": "Transfer",
          "lowerEntry": "parseAndLowerSource6",
          "prepareEntry": "prepareMil4S0",
          "wrapperEntry": "prepareSource6S0Unqualified",
          "fileCount": 3,
          "filesHex": [
            "2f2f2066726f6e740d0a",
            "00ffefbbbf",
            ""
          ]
        },
        "terms": {
          "signer": {
            "sort": "signer",
            "value": "Alice"
          },
          "keyRef": "key:é😀",
          "validFrom": "0",
          "validUntil": "18446744073709551615",
          "grossCap": "170141183460469231731687303715884105727",
          "feeCap": "0",
          "netFloor": "1",
          "failureRelation": "success_only",
          "supplyChanges": "empty",
          "observations": "empty",
          "disclosures": "empty",
          "retainedEffects": "empty",
          "retainedDuties": "empty",
          "delegation": "none",
          "recovery": "none",
          "operation": {
            "kind": "Transfer",
            "owner": {
              "sort": "account",
              "value": "Alice"
            },
            "recipient": {
              "sort": "account",
              "value": "Bob"
            },
            "feeRecipient": {
              "sort": "account",
              "value": "Fees"
            },
            "amount": "11",
            "fee": "0"
          }
        }
      },
      "policyInput": {
        "sourceVersion": 6,
        "coreVersion": 5,
        "wireProfile": 1,
        "agreementInstanceId": {
          "sort": "agreement",
          "value": "Agreement1"
        },
        "domain": {
          "sort": "domain",
          "value": "Preview"
        },
        "asset": {
          "sort": "asset",
          "value": "A"
        },
        "scale": "0",
        "operationKind": "Transfer",
        "actionId": {
          "sort": "action",
          "value": "TransferLiteralFee"
        },
        "coreProgramId": {
          "sort": "coreProgram",
          "value": "TransferLiteralFee"
        },
        "sourceHash": "07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3",
        "coreHash": "e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e",
        "signer": {
          "sort": "signer",
          "value": "Alice"
        },
        "keyRef": "key:é😀",
        "validFrom": "0",
        "validUntil": "18446744073709551615",
        "grossCap": "170141183460469231731687303715884105727",
        "feeCap": "0",
        "netFloor": "1",
        "failureRelation": "success_only",
        "supplyChanges": "empty",
        "observations": "empty",
        "disclosures": "empty",
        "retainedEffects": "empty",
        "retainedDuties": "empty",
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "Transfer",
          "owner": {
            "sort": "account",
            "value": "Alice"
          },
          "recipient": {
            "sort": "account",
            "value": "Bob"
          },
          "feeRecipient": {
            "sort": "account",
            "value": "Fees"
          },
          "amount": "11",
          "fee": "0"
        }
      },
      "images": {
        "source": {
          "purpose": 1,
          "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656501",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000570600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656501",
          "payloadBytes": 87,
          "sha256": "07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3"
        },
        "core": {
          "purpose": 3,
          "payloadHex": "05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f330100125472616e736665724c69746572616c4665650100147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f3100030000009e05000f6d6f7269617274792d636f72652f350600116d6f7269617274792d696e74656e742f330100125472616e736665724c69746572616c4665650100147061727365416e644c6f776572536f7572636536000d707265706172654d696c345330001b70726570617265536f75726365365330556e7175616c69666965640003010000000a2f2f2066726f6e740d0a020000000500ffefbbbf0300000000",
          "payloadBytes": 158,
          "sha256": "e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e"
        },
        "policy": {
          "purpose": 4,
          "payloadHex": "060501000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656500125472616e736665724c69746572616c4665650107ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e0005416c696365000b6b65793a65cc81f09f98800000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000000000000000000101000000000000000000000000010005416c6963650003426f620004466565730000000000000000000000000000000b00000000000000000000000000000000",
          "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f31000400000119060501000a41677265656d656e74310007507265766965770001410000125472616e736665724c69746572616c46656500125472616e736665724c69746572616c4665650107ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3e2b032a58e0992ed71e2fd1c29498235e00a9ea42a42710acc5df5ce5737037e0005416c696365000b6b65793a65cc81f09f98800000000000000000ffffffffffffffff7fffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000000000000000000101000000000000000000000000010005416c6963650003426f620004466565730000000000000000000000000000000b00000000000000000000000000000000",
          "payloadBytes": 281,
          "sha256": "3ead0aa2cc6455eef1828dd2f3103765f1cd6714c4f7e6fe27166bda04a09aa6"
        }
      }
    }
  ],
  "actualPackages": [
    {
      "operationKind": "Transfer",
      "purpose": 3,
      "payloadBytes": 41446,
      "sha256": "21a5a5c01858be797ee49894c02dceb777a1ad31d70d196cddc93d423804e585"
    },
    {
      "operationKind": "Repay",
      "purpose": 3,
      "payloadBytes": 41445,
      "sha256": "6e153b337f696bcb9f7ec0bcd84e3dcf5a3bac70b604cd417836e65178e2b425"
    }
  ],
  "actualModuleInputs": [
    {
      "role": 1,
      "name": "financial-agreement-source-v6-frontend.ts",
      "bytes": 20746,
      "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
    },
    {
      "role": 2,
      "name": "mil4-s0-core-v5.ts",
      "bytes": 17917,
      "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
    },
    {
      "role": 3,
      "name": "mil4-s0-source-v6.ts",
      "bytes": 2640,
      "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/protected-inputs.json

sha256: `b90ed12bf887687f6f03e10381046bb70d37292d6a8ea7fc23151fb605a5e282`

```text
{
  "packet": "deliverables/mil4-k-quint-sprint1-2026-09-29/audits/w-d2h-grok-repair-02-candidate-packet.md",
  "packetSha256": "daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9",
  "inputs": [
    {
      "path": "experiments/moriarty-language/formal/mil4/hash-images/SPEC.md",
      "expectedSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
      "currentSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
      "bytes": 35762,
      "unchangedFromFrozenPacket": true
    },
    {
      "path": "experiments/moriarty-language/formal/mil4/hash-images/DECISION-MATRIX.md",
      "expectedSha256": "620605c0ad98539736b4ebce6452bcf10b651a6f781a484d448961035e3042db",
      "currentSha256": "620605c0ad98539736b4ebce6452bcf10b651a6f781a484d448961035e3042db",
      "bytes": 20698,
      "unchangedFromFrozenPacket": true
    },
    {
      "path": "experiments/moriarty-language/formal/mil4/hash-images/RESULT.md",
      "expectedSha256": "86940c7a15ff1cf8b5c3aeb966f958911a6efa1a2e5ff5a73483d8023d3fbcd9",
      "currentSha256": "86940c7a15ff1cf8b5c3aeb966f958911a6efa1a2e5ff5a73483d8023d3fbcd9",
      "bytes": 6756,
      "unchangedFromFrozenPacket": true
    },
    {
      "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
      "expectedSha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7",
      "currentSha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7",
      "bytes": 20746,
      "unchangedFromFrozenPacket": true
    },
    {
      "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
      "expectedSha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda",
      "currentSha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda",
      "bytes": 17917,
      "unchangedFromFrozenPacket": true
    },
    {
      "path": "experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts",
      "expectedSha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8",
      "currentSha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8",
      "bytes": 2640,
      "unchangedFromFrozenPacket": true
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/result-data.json

sha256: `35baac51bb1f3f95dd0e824e37874c03f03d44fa4250c6d2f9af645d3849e188`

```text
{
  "status": "local-executable-experiment-not-adopted",
  "pinnedCandidate": "hash-images-repair-02",
  "specSha256": "52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc",
  "packetSha256": "daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9",
  "comparisonToRepair03RequiredBeforeDesignClaim": true,
  "node": "v24.21.0",
  "moduleInputs": [
    {
      "role": 1,
      "name": "financial-agreement-source-v6-frontend.ts",
      "bytes": 20746,
      "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
    },
    {
      "role": 2,
      "name": "mil4-s0-core-v5.ts",
      "bytes": 17917,
      "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
    },
    {
      "role": 3,
      "name": "mil4-s0-source-v6.ts",
      "bytes": 2640,
      "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
    }
  ],
  "observations": [
    {
      "operationKind": "Transfer",
      "images": {
        "source": {
          "purpose": 1,
          "payloadBytes": 87,
          "envelopeBytes": 114,
          "sha256": "07ef99c28fad13e9ba80f83fdfd36de87fdb92f86e66bee4a7b887f99ac4a8e3"
        },
        "core": {
          "purpose": 3,
          "payloadBytes": 41446,
          "envelopeBytes": 41473,
          "sha256": "21a5a5c01858be797ee49894c02dceb777a1ad31d70d196cddc93d423804e585"
        },
        "policy": {
          "purpose": 4,
          "payloadBytes": 274,
          "envelopeBytes": 301,
          "sha256": "6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e"
        }
      },
      "exactPolicyContentComparison": {
        "matches": true,
        "digest": "6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e",
        "claimedDigest": "6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e",
        "scope": "content-only",
        "authenticated": false
      },
      "changedAmountContentComparison": {
        "matches": false,
        "digest": "0ae1e8dd6a1ba5580a40cbd46aab93d55b5f2e00108a64f366df098a4ebdb8e1",
        "claimedDigest": "6aa3c9cfbe90002a12aa1e6b6a43c8c10faad78b89ec2e554d5ce7363919102e",
        "scope": "content-only",
        "authenticated": false
      },
      "changedAmountSha256": "0ae1e8dd6a1ba5580a40cbd46aab93d55b5f2e00108a64f366df098a4ebdb8e1"
    },
    {
      "operationKind": "Repay",
      "images": {
        "source": {
          "purpose": 1,
          "payloadBytes": 86,
          "envelopeBytes": 113,
          "sha256": "dca58b38c41070228a5f6625bd0ec2dd4aa8f183e2b4809aa56b81a3e595ae78"
        },
        "core": {
          "purpose": 3,
          "payloadBytes": 41445,
          "envelopeBytes": 41472,
          "sha256": "6e153b337f696bcb9f7ec0bcd84e3dcf5a3bac70b604cd417836e65178e2b425"
        },
        "policy": {
          "purpose": 4,
          "payloadBytes": 254,
          "envelopeBytes": 281,
          "sha256": "09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc"
        }
      },
      "exactPolicyContentComparison": {
        "matches": true,
        "digest": "09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc",
        "claimedDigest": "09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc",
        "scope": "content-only",
        "authenticated": false
      },
      "changedAmountContentComparison": {
        "matches": false,
        "digest": "a1c2b6e8cb5b7dce30ebf79db943dccd98a9b51c70e5180515d58d95b524ac30",
        "claimedDigest": "09af673fac67829bde03788c074c8fe4e48cf0e47f8ac43ed5d1cecc4123c3cc",
        "scope": "content-only",
        "authenticated": false
      },
      "changedAmountSha256": "a1c2b6e8cb5b7dce30ebf79db943dccd98a9b51c70e5180515d58d95b524ac30"
    }
  ],
  "frozenSyntheticCompleteVectors": 9,
  "frozenActualPackageDigestVectors": 2,
  "vectorIndependence": "different language and serialization implementation; same author; no independent provider review",
  "authentication": false,
  "b05b06b07Adopted": false,
  "wd2GateClosed": false,
  "limitations": [
    "closed projected typed values, not a Source parser or full common-domain AST sweep",
    "no artifact decoder or consumer schedule",
    "no provider or registry authenticity",
    "no actual-export, dependency-closure, loaded-artifact, toolchain or lowering correspondence proof",
    "no B16 complete-history adapter",
    "no signature, native proof or ledger submission"
  ]
}

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/run-experiment.mjs

sha256: `e5101bdf66473bb4ee182ae429f924ec4ca8f5ce94f2c61f8bdb86f981684e1a`

```text
/** Reproduce local content results using explicit current package bytes. */
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {encodeImage, produceImages, compareContent} from './codec.mjs';

const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url)));
const digest = raw => createHash('sha256').update(raw).digest('hex');
// The live design can advance while this experiment stays pinned to repair-02.
const packetPath = '../../../../../deliverables/mil4-k-quint-sprint1-2026-09-29/audits/w-d2h-grok-repair-02-candidate-packet.md';
const packetBytes = readFileSync(new URL(packetPath, import.meta.url));
const packetSha256 = 'daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9';
assert.equal(digest(packetBytes), packetSha256, 'frozen repair-02 packet changed');
const prefix = `## experiments/moriarty-language/formal/mil4/hash-images/SPEC.md\n\nsha256: \`${vectors.specSha256}\`\n\n\`\`\`text\n`;
const suffix = '\n```\n\n## experiments/moriarty-language/formal/mil4/hash-images/DECISION-MATRIX.md';
const sourceBytes = Buffer.from(packetBytes.toString('utf8').split(prefix)[1].split(suffix)[0], 'utf8');
assert.equal(digest(sourceBytes), vectors.specSha256, 'embedded repair-02 SPEC bytes changed');
const modules = vectors.actualModuleInputs.map(input => {
  const bytes = readFileSync(new URL(`../../../src/successor/${input.name}`, import.meta.url));
  assert.equal(bytes.length, input.bytes, 'current package input length changed');
  assert.equal(digest(bytes), input.sha256, 'current package input bytes changed');
  return {role: input.role, bytes};
});
const observations = [];
for (const index of [0, 1]) {
  const input = structuredClone(vectors.vectors[index].inputs);
  input.package.files = modules;
  delete input.package.filesHex;
  const result = produceImages(input.definition, input.package, input.terms);
  const expected = vectors.actualPackages[index];
  assert.equal(result.core.digest, expected.sha256, 'separate Python actual-package digest differs');
  assert.equal(result.core.payload.length, expected.payloadBytes);
  const images = {};
  for (const name of ['source', 'core', 'policy']) images[name] = {
    purpose: result[name].purpose, payloadBytes: result[name].payload.length,
    envelopeBytes: result[name].preimage.length, sha256: result[name].digest,
  };
  const same = compareContent(4, result.policyValue, result.policy.digest);
  const changed = structuredClone(result.policyValue);
  changed.operation.amount = (BigInt(changed.operation.amount) + 1n).toString();
  const different = compareContent(4, changed, result.policy.digest);
  assert.equal(same.matches, true); assert.equal(different.matches, false);
  observations.push({operationKind: input.definition.operationKind, images,
    exactPolicyContentComparison: same, changedAmountContentComparison: different,
    changedAmountSha256: encodeImage(4, changed).digest});
}
const report = {
  status: 'local-executable-experiment-not-adopted', pinnedCandidate: 'hash-images-repair-02',
  specSha256: vectors.specSha256, packetSha256,
  comparisonToRepair03RequiredBeforeDesignClaim: true,
  node: process.version, moduleInputs: vectors.actualModuleInputs, observations,
  frozenSyntheticCompleteVectors: vectors.vectors.length * 3,
  frozenActualPackageDigestVectors: vectors.actualPackages.length,
  vectorIndependence: 'different language and serialization implementation; same author; no independent provider review',
  authentication: false, b05b06b07Adopted: false, wd2GateClosed: false,
  limitations: ['closed projected typed values, not a Source parser or full common-domain AST sweep',
    'no artifact decoder or consumer schedule', 'no provider or registry authenticity',
    'no actual-export, dependency-closure, loaded-artifact, toolchain or lowering correspondence proof',
    'no B16 complete-history adapter', 'no signature, native proof or ledger submission'],
};
writeFileSync(new URL('./result-data.json', import.meta.url), JSON.stringify(report, null, 2) + '\n');
console.log(JSON.stringify(report, null, 2));

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/test-results.tap

sha256: `c084410b00abc3880fa57d727e77c2daa30c2ffa40b5266475f815c540d72cb4`

```text
TAP version 13
# Subtest: codec implements the proposed closed purpose 1/3/4 API
ok 1 - codec implements the proposed closed purpose 1/3/4 API
  ---
  duration_ms: 3.050533
  type: 'test'
  ...
# Subtest: synthetic-transfer: payload, envelope and digest equal frozen Python vectors
ok 2 - synthetic-transfer: payload, envelope and digest equal frozen Python vectors
  ---
  duration_ms: 5.672795
  type: 'test'
  ...
# Subtest: synthetic-repayment: payload, envelope and digest equal frozen Python vectors
ok 3 - synthetic-repayment: payload, envelope and digest equal frozen Python vectors
  ---
  duration_ms: 2.323685
  type: 'test'
  ...
# Subtest: unicode-transfer: payload, envelope and digest equal frozen Python vectors
ok 4 - unicode-transfer: payload, envelope and digest equal frozen Python vectors
  ---
  duration_ms: 0.809257
  type: 'test'
  ...
# Subtest: extra fields cannot enter a Source image
ok 5 - extra fields cannot enter a Source image
  ---
  duration_ms: 0.603912
  type: 'test'
  ...
# Subtest: agreement and action nominal sorts stay distinct
ok 6 - agreement and action nominal sorts stay distinct
  ---
  duration_ms: 0.166798
  type: 'test'
  ...
# Subtest: reserved Source IDs reject
ok 7 - reserved Source IDs reject
  ---
  duration_ms: 0.224205
  type: 'test'
  ...
# Subtest: wire-only identifier punctuation rejects
ok 8 - wire-only identifier punctuation rejects
  ---
  duration_ms: 0.132305
  type: 'test'
  ...
# Subtest: initial digits reject
ok 9 - initial digits reject
  ---
  duration_ms: 0.51409
  type: 'test'
  ...
# Subtest: non-ASCII IDs reject
ok 10 - non-ASCII IDs reject
  ---
  duration_ms: 0.432984
  type: 'test'
  ...
# Subtest: ID length over 64 rejects
ok 11 - ID length over 64 rejects
  ---
  duration_ms: 0.354853
  type: 'test'
  ...
# Subtest: scale canonical leading zeros reject
ok 12 - scale canonical leading zeros reject
  ---
  duration_ms: 0.125882
  type: 'test'
  ...
# Subtest: scale 19 rejects
ok 13 - scale 19 rejects
  ---
  duration_ms: 0.16616
  type: 'test'
  ...
# Subtest: numeric coercion rejects
ok 14 - numeric coercion rejects
  ---
  duration_ms: 0.147916
  type: 'test'
  ...
# Subtest: old Source version rejects
ok 15 - old Source version rejects
  ---
  duration_ms: 0.0846
  type: 'test'
  ...
# Subtest: old Source profile rejects
ok 16 - old Source profile rejects
  ---
  duration_ms: 0.113874
  type: 'test'
  ...
# Subtest: selector and signed constructor disagreement rejects
ok 17 - selector and signed constructor disagreement rejects
  ---
  duration_ms: 0.081206
  type: 'test'
  ...
# Subtest: closed records reject getters
ok 18 - closed records reject getters
  ---
  duration_ms: 0.299289
  type: 'test'
  ...
# Subtest: closed records reject symbols
ok 19 - closed records reject symbols
  ---
  duration_ms: 0.181712
  type: 'test'
  ...
# Subtest: policy rejects empty keyRef
ok 20 - policy rejects empty keyRef
  ---
  duration_ms: 0.244956
  type: 'test'
  ...
# Subtest: policy rejects lone high surrogate
ok 21 - policy rejects lone high surrogate
  ---
  duration_ms: 0.0894
  type: 'test'
  ...
# Subtest: policy rejects lone low surrogate
ok 22 - policy rejects lone low surrogate
  ---
  duration_ms: 0.08777
  type: 'test'
  ...
# Subtest: decoded NUL control rejects
ok 23 - decoded NUL control rejects
  ---
  duration_ms: 0.076042
  type: 'test'
  ...
# Subtest: decoded DEL control rejects
ok 24 - decoded DEL control rejects
  ---
  duration_ms: 0.075774
  type: 'test'
  ...
# Subtest: UTF-8 byte cap applies to multibyte keyRef
ok 25 - UTF-8 byte cap applies to multibyte keyRef
  ---
  duration_ms: 0.27108
  type: 'test'
  ...
# Subtest: UTF-16 keyRef cap rejects before encoding
ok 26 - UTF-16 keyRef cap rejects before encoding
  ---
  duration_ms: 0.109362
  type: 'test'
  ...
# Subtest: round 2^64 rejects instead of truncating
ok 27 - round 2^64 rejects instead of truncating
  ---
  duration_ms: 0.167897
  type: 'test'
  ...
# Subtest: round leading zeros reject
ok 28 - round leading zeros reject
  ---
  duration_ms: 0.086463
  type: 'test'
  ...
# Subtest: reversed validity interval rejects
ok 29 - reversed validity interval rejects
  ---
  duration_ms: 0.254884
  type: 'test'
  ...
# Subtest: nominal 2^127 rejects
ok 30 - nominal 2^127 rejects
  ---
  duration_ms: 1.224464
  type: 'test'
  ...
# Subtest: negative nominal rejects
ok 31 - negative nominal rejects
  ---
  duration_ms: 0.225453
  type: 'test'
  ...
# Subtest: unbounded integer text rejects
ok 32 - unbounded integer text rejects
  ---
  duration_ms: 0.138328
  type: 'test'
  ...
# Subtest: float nominal rejects
ok 33 - float nominal rejects
  ---
  duration_ms: 0.277187
  type: 'test'
  ...
# Subtest: uppercase hash text rejects
ok 34 - uppercase hash text rejects
  ---
  duration_ms: 0.121187
  type: 'test'
  ...
# Subtest: short hash text rejects
ok 35 - short hash text rejects
  ---
  duration_ms: 0.221135
  type: 'test'
  ...
# Subtest: nominal action/core interchange rejects
ok 36 - nominal action/core interchange rejects
  ---
  duration_ms: 0.094027
  type: 'test'
  ...
# Subtest: accepted failure variant rejects
ok 37 - accepted failure variant rejects
  ---
  duration_ms: 0.135642
  type: 'test'
  ...
# Subtest: nonempty retention rejects
ok 38 - nonempty retention rejects
  ---
  duration_ms: 0.068508
  type: 'test'
  ...
# Subtest: operation variant disagreement rejects
ok 39 - operation variant disagreement rejects
  ---
  duration_ms: 0.207701
  type: 'test'
  ...
# Subtest: arbitrary dynamic policy fields reject
ok 40 - arbitrary dynamic policy fields reject
  ---
  duration_ms: 0.534952
  type: 'test'
  ...
# Subtest: repayment requires the explicit allocation and identity conversion constants
ok 41 - repayment requires the explicit allocation and identity conversion constants
  ---
  duration_ms: 0.461842
  type: 'test'
  ...
# Subtest: keyRef 1024 ASCII bytes and 512 two-byte scalars admit exactly
ok 42 - keyRef 1024 ASCII bytes and 512 two-byte scalars admit exactly
  ---
  duration_ms: 5.402414
  type: 'test'
  ...
# Subtest: keyRef preserves Unicode composition, whitespace, case and literal escape content
ok 43 - keyRef preserves Unicode composition, whitespace, case and literal escape content
  ---
  duration_ms: 6.542376
  type: 'test'
  ...
# Subtest: equal nominal text is admitted separately in agreement/domain roles
ok 44 - equal nominal text is admitted separately in agreement/domain roles
  ---
  duration_ms: 0.65897
  type: 'test'
  ...
# Subtest: file count must be exactly three
ok 45 - file count must be exactly three
  ---
  duration_ms: 0.273646
  type: 'test'
  ...
# Subtest: file role order must be 1/2/3
ok 46 - file role order must be 1/2/3
  ---
  duration_ms: 0.147106
  type: 'test'
  ...
# Subtest: extra package files reject
ok 47 - extra package files reject
  ---
  duration_ms: 0.154439
  type: 'test'
  ...
# Subtest: file over 1MiB rejects
ok 48 - file over 1MiB rejects
  ---
  duration_ms: 0.140968
  type: 'test'
  ...
# Subtest: file path cannot replace explicit bytes
ok 49 - file path cannot replace explicit bytes
  ---
  duration_ms: 0.144906
  type: 'test'
  ...
# Subtest: package metadata is outside the image
ok 50 - package metadata is outside the image
  ---
  duration_ms: 0.140161
  type: 'test'
  ...
# Subtest: stale package entry labels reject
ok 51 - stale package entry labels reject
  ---
  duration_ms: 0.147977
  type: 'test'
  ...
# Subtest: wrong Core profile rejects
ok 52 - wrong Core profile rejects
  ---
  duration_ms: 0.117929
  type: 'test'
  ...
# Subtest: wrong intent schema rejects
ok 53 - wrong intent schema rejects
  ---
  duration_ms: 1.955371
  type: 'test'
  ...
# Subtest: shared file backing memory rejects
ok 54 - shared file backing memory rejects
  ---
  duration_ms: 1.20697
  type: 'test'
  ...
# Subtest: all three files at exactly 1MiB admit below total 4MiB cap
ok 55 - all three files at exactly 1MiB admit below total 4MiB cap
  ---
  duration_ms: 17.147236
  type: 'test'
  ...
# Subtest: every file byte, line ending, BOM and unused branch changes the package digest
ok 56 - every file byte, line ending, BOM and unused branch changes the package digest
  ---
  duration_ms: 1.501424
  type: 'test'
  ...
# Subtest: producer recomputes links and rejects incoherent selected package identity
ok 57 - producer recomputes links and rejects incoherent selected package identity
  ---
  duration_ms: 0.259471
  type: 'test'
  ...
# Subtest: content comparator states its content-only scope for equal and unequal claims
ok 58 - content comparator states its content-only scope for equal and unequal claims
  ---
  duration_ms: 0.2571
  type: 'test'
  ...
# Subtest: no purpose 2, unknown purpose or implicit suite search is available
ok 59 - no purpose 2, unknown purpose or implicit suite search is available
  ---
  duration_ms: 2.31825
  type: 'test'
  ...
# Subtest: object enumeration order does not determine bytes
ok 60 - object enumeration order does not determine bytes
  ---
  duration_ms: 0.881649
  type: 'test'
  ...
# Subtest: policy amount and zero-fee recipient have independent content effects
ok 61 - policy amount and zero-fee recipient have independent content effects
  ---
  duration_ms: 3.936474
  type: 'test'
  ...
# Subtest: actual current package bytes match independently frozen transfer/repayment digests
ok 62 - actual current package bytes match independently frozen transfer/repayment digests
  ---
  duration_ms: 2.922751
  type: 'test'
  ...
# Subtest: standalone policy body identity differs without a consumer context assertion
ok 63 - standalone policy body identity differs without a consumer context assertion
  ---
  duration_ms: 0.273292
  type: 'test'
  ...
# Subtest: standalone producer keyRef domain error has no consumer anchor
ok 64 - standalone producer keyRef domain error has no consumer anchor
  ---
  duration_ms: 0.706025
  type: 'test'
  ...
# Subtest: maximum 64-byte IDs and scale 18 encode without repair
ok 65 - maximum 64-byte IDs and scale 18 encode without repair
  ---
  duration_ms: 0.168349
  type: 'test'
  ...
# Subtest: file array extra properties and accessors reject without executing accessors
ok 66 - file array extra properties and accessors reject without executing accessors
  ---
  duration_ms: 1.021159
  type: 'test'
  ...
# Subtest: package uses exactly the passed Uint8Array subview bytes
ok 67 - package uses exactly the passed Uint8Array subview bytes
  ---
  duration_ms: 0.17538
  type: 'test'
  ...
# Subtest: decoded astral scalar length observes UTF-8 cap simultaneously
ok 68 - decoded astral scalar length observes UTF-8 cap simultaneously
  ---
  duration_ms: 0.246638
  type: 'test'
  ...
# Subtest: cross-purpose values cannot be coerced into the other closed records
ok 69 - cross-purpose values cannot be coerced into the other closed records
  ---
  duration_ms: 0.361655
  type: 'test'
  ...
# Subtest: returned buffers do not alias caller package bytes
ok 70 - returned buffers do not alias caller package bytes
  ---
  duration_ms: 0.195056
  type: 'test'
  ...
# Subtest: shadowed byte lengths cannot bypass the exact file cap
ok 71 - shadowed byte lengths cannot bypass the exact file cap
  ---
  duration_ms: 0.185391
  type: 'test'
  ...
# Subtest: byte buffer shadow properties do not change the intrinsic view content
ok 72 - byte buffer shadow properties do not change the intrinsic view content
  ---
  duration_ms: 0.368178
  type: 'test'
  ...
1..72
# tests 72
# suites 0
# pass 72
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 255.202856

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/artifact-manifest.json

sha256: `e31306c37ba09414dec37fc3bbc794cb112309e064075074f3c3eaf017e0c1a7`

```text
{
  "status": "local-experiment-not-adopted",
  "candidate": "hash-images-repair-02",
  "tests": {
    "passed": 72,
    "failed": 0,
    "skipped": 0,
    "command": "node --test --test-reporter=tap codec.test.mjs",
    "scope": [
      "codec.mjs",
      "codec.test.mjs",
      "vectors.json",
      "../../../src/successor/financial-agreement-source-v6-frontend.ts",
      "../../../src/successor/mil4-s0-core-v5.ts",
      "../../../src/successor/mil4-s0-source-v6.ts"
    ]
  },
  "artifacts": [
    {
      "path": "ISSUES.md",
      "bytes": 3091,
      "sha256": "9fb33c3fdb512aa91629621d056661f76cf01c43e7048cba9944665a7ef26db5"
    },
    {
      "path": "PLAN.md",
      "bytes": 1834,
      "sha256": "9a6aff6dbe8b0b19c02b2e0fab20901bf60ac836e17051f1ab8f537bb3894110"
    },
    {
      "path": "RESULT.md",
      "bytes": 6979,
      "sha256": "0b8ad63f50b6e110a02a575b500567a5d94b8f6ea09047741834392b323c6d83"
    },
    {
      "path": "codec.d.mts",
      "bytes": 3329,
      "sha256": "eb7dd60101474bdfe2be645e800991820f8f0e10ac0f0ff0b628c250ec99cc72"
    },
    {
      "path": "codec.mjs",
      "bytes": 17108,
      "sha256": "c7e9479889d4168f858c147fc28bd8d2bb2be0ac24ec4b0554b61a4bb87041a6"
    },
    {
      "path": "codec.test.mjs",
      "bytes": 15221,
      "sha256": "7c6a55e80226be4abe8c618aaba9fe3145123560cd22b0570967bd1e55297090"
    },
    {
      "path": "protected-inputs.json",
      "bytes": 2276,
      "sha256": "b90ed12bf887687f6f03e10381046bb70d37292d6a8ea7fc23151fb605a5e282"
    },
    {
      "path": "reference-vectors.py",
      "bytes": 7823,
      "sha256": "1acb9190ba5bba016e3241e16a0c4adb67f0bcb40aa6259aeef882f1e32a3af9"
    },
    {
      "path": "result-data.json",
      "bytes": 4471,
      "sha256": "35baac51bb1f3f95dd0e824e37874c03f03d44fa4250c6d2f9af645d3849e188"
    },
    {
      "path": "run-experiment.mjs",
      "bytes": 4163,
      "sha256": "e5101bdf66473bb4ee182ae429f924ec4ca8f5ce94f2c61f8bdb86f981684e1a"
    },
    {
      "path": "test-results.tap",
      "bytes": 11608,
      "sha256": "c084410b00abc3880fa57d727e77c2daa30c2ffa40b5266475f815c540d72cb4"
    },
    {
      "path": "vectors.json",
      "bytes": 22577,
      "sha256": "ef780924877c2822cdb72ee071fb9f3d8cbb7cb0bbf793a650389571f5cfb3c6"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/hash-images/history/pre-grok-repair-03/SPEC.md

sha256: `52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc`

```text
# W-D2H B05–B07 hash-image and lowering proposal

**Status: proposed / specified-only, 2026-09-30.** Scope is the closed
Source/6 → Core/5 S0 proposal and authorization wire/3. This sprint supplies
design definitions and design oracles only. It implements no encoder, hash
producer, consumer, registry, authentication, correspondence proof or test.
The recommendation is not an adopted decision. W-D2/W-D3 and B01–B17 remain
open. No existing `source_hash`, `digest`, `sourceHash`, `coreHash` or
`policyHash` claim is promoted to a computed commitment.

## Inspected repository facts

The [Source contract](../../../spec/successor/financial-agreement-source-v6.md),
[grammar](../../../spec/successor/financial-agreement-source-v6-grammar.ebnf),
[frontend/lowerer](../../../src/successor/financial-agreement-source-v6-frontend.ts),
[Core preparer](../../../src/successor/mil4-s0-core-v5.ts),
[Source wrapper](../../../src/successor/mil4-s0-source-v6.ts),
[S0 contract](../s0-implementation-contract.md),
[wire/3 specification](../wire/SPEC.md),
[W-D2F field map](../effect-consumer/FIELD-MAP.md),
[W-D2F plan](../effect-consumer/PLAN.md) and
[W-D2G identity proposal](../identity-binding/SPEC.md) are inputs, unchanged.

Repository observations:

- Source/6 contains one stage proposal selecting a builtin. It has no
  user-defined action body. The only selectors are `TransferLiteralFee` and
  `RepayAccrualFirst`, checked against the signed action constructor.
- The identifier after `agreement` is `ast.programId`. Lowering instead sets
  `Core.intent.programId=ast.selected.actionId`. These are different roles.
- `selected source_hash` and `selected digest` parse as nonempty opaque
  strings. The latter is the policy claim, not an authorization digest.
  The lowerer copies both strings; Core checks opaque shape only.
- Lowering constructs local state/intent/effects, preserves both work
  counters, and converts a replay nonce to `JSON.stringify([domain,signer,nonce])`.
  It projects a selected replay claim to `[]` or `[selectedTuple]`; that is
  not complete authenticated replay history.
- Agreement, scale and authenticated predecessor survive only in the AST.
  The wrapper also reports selected-program binding as unverified. It compares
  signed/submitted actions after Stage and withholds a prepared candidate on
  disagreement. No exact signed digest is calculated.
- Wire/3 tags8,11,12 are supplied 32-byte Source/Core/policy claims. Its
  current authorization content digest is SHA256 of all canonical wire bytes.
  Neither a wire round trip nor equal digest claims proves their provenance.

## Two non-self-referential Source strategies

Both strategies require successful existing Source formation and W-D2F's
common-domain check before a candidate comparison. Neither hashes a document
including its own `source_hash` value. Neither recursively hashes the policy
claim. [DECISION-MATRIX](DECISION-MATRIX.md) compares consequences.

**A — parsed-location document projection, preimage alternative only.** Take the exact UTF-8 source
document, including comments, presentation whitespace, every other token,
snapshot, submitted effects and successor. Replace exactly the entire JSON
string token at `selected.sourceHash` with the ASCII token
`"<excluded:source_hash>"`, and exactly the token at `selected.policyDigest`
with `"<excluded:digest>"`. Preserve all remaining bytes, including whitespace
around the replaced tokens. Locate these two unique fields by the successfully
parsed grammar, never by regex, global string replacement or a caller's byte
offsets. This token-span extraction would be new work: the current returned
AST does not expose these spans. No other occurrence in comments, keys or
strings is masked. Purpose2 below hashes this projected document. The projected
payload is bounded at65573 bytes: Source's65536-byte bound plus at most37 bytes
of growth when its two nonempty quoted tokens are replaced by the24-byte and
19-byte markers. The encoder must count that actual projected length.

This is a commitment to a particular stage presentation with two excluded
claims. It changes on whitespace, comment, snapshot and independent successor
changes. It cannot be called a code-only image or reused as a stable action
definition. Changing either excluded claim cannot change its own source image;
the claims still require independent checks. This proposal defines A's preimage
only. It does not define an A-specific authenticated consumer, policy linkage
or diagnostic schedule; purpose4 and the consumer sections below select B.
A cannot substitute into that path or be tried as a fallback.

**B — canonical selected-definition projection, recommended.** From a parsed
Source artifact extract only the ordered definition tuple below. Purpose1
hashes its binary encoding. Presentation changes do not change the tuple.
The definition identifies an instance-scoped builtin selection and settlement
declaration; the exact builtin/lowerer implementation is separately committed
by the Core package. No new user-defined code or template-to-instance relation
is inferred. This limited interpretation is necessary because Source/6 has no
action body. A later grammar with action bodies must commit those bodies under
a new image profile; dropping them is forbidden.

## Exact proposed byte rules

The following are new definitions, not existing codec behavior.

For purposes1–4 define the preimage:

```text
ASCII("moriarty-mil4-image/1") || 00 || purpose:u8 || payloadLength:u32be || payload
digest = SHA256(preimage)
```

`purpose` is exactly `01` Source definition B, `02` projected document A,
`03` Core implementation package, or `04` policy. No other purpose, extension,
padding or trailing byte is accepted. `payloadLength` counts payload bytes,
not the envelope. Digests are 32 raw bytes inside another image and exactly
64 lowercase hex characters in a Source/wire presentation. These domain
prefixes do not change wire/3's existing authorization content-digest rule or
select B09's signature-message preprocessing.

Use these primitives, all with exact length bounds before allocation:

| Primitive | Proposed bytes and domain |
| --- | --- |
| `id` | UInt16 big-endian byte count then exact ASCII; W-D2F common Source identifier subtype and reserved-word exclusion; 1–64 bytes |
| `text` | UInt16 big-endian byte count then exactly UTF-8 encoding of the decoded Unicode scalar sequence; 1–1024 bytes; no normalization, trimming, case folding or original JSON escape bytes |
| `nominal` | UInt128 big-endian, value0..2^127−1; Source canonical decimal parsed exactly |
| `round` | UInt64 big-endian, value0..2^64−1; explicit Source/wire intersection, no truncation |
| `scale` | One byte0..18; no rescaling |
| `hash32` | Exactly32 raw bytes; no prefix or text encoding |
| `blob` | UInt32 big-endian byte count then exact bytes |

Integer presentation leading zeros and malformed strings already reject
formation/domain checking; encoders must not repair them. The binary tuple
has no field names, implicit defaults or optional slots. Each fixed byte
below must be encoded even when derivable from another field. An image
encoder accepts a closed typed value, not arbitrary object enumeration.
Purpose1 and purpose4 payloads each have a4096-byte proposed admission cap.
All displayed integer/role constants with leading zeros are hexadecimal
encoded-byte spellings, not Source decimal presentation. Thus `0003:u16be`
means exactly the two bytes00 03; `0000:u16be` means00 00.
UTF-8 uses the shortest valid encoding of each scalar, with no BOM or NUL
terminator. Lone surrogates, invalid UTF-8 and replacement-decoder repair
reject. Purpose4 keyRef additionally satisfies Core/5's opaque admission:
nonempty, at most1024 UTF-16 code units, and no U+0000..U+001F or U+007F.
The1024-byte text cap is simultaneous, not a replacement for this restriction.
Escaped JSON control characters decode before that check; `"\u0000"` cannot
be admitted by retaining escape text. Source formation may accept a decoded
control in an opaque string; the proposed policy image rejects such a keyRef
and does not invent a Source parser error or expand W-D2F's D sweep.

### Source definition payload B, purpose1

Order is exact:

```text
sourceVersion=06:u8
profile=text("moriarty-financial-agreement-source/6")
wireProfile=01:u8                         # s0-provisional/1
agreementInstanceId=id(ast.programId)
domain=id(ast.domain)
asset=id(ast.settlement.asset)
scale=u8(ast.settlement.scale)
selectedActionId=id(ast.selected.actionId)
operationKind=01:u8 | 02:u8               # transfer | repayment
```

The only permitted selector/kind pairs are
`(TransferLiteralFee,01)` and `(RepayAccrualFirst,02)`. The kind is checked
against `ast.intent.signedAction.kind`; it is not chosen by a caller independently.
Source version and source profile both participate; no `/5` fallback exists.

**Included:** the nine fields above only. **Excluded:** original token spellings,
comments, whitespace, both embedded digest claims, all `intent` fields beyond
the constructor discriminator, the entire authenticated block, submitted
action/effects/post-head, signature/proof/receipt bytes, wire stage/episode,
authorization digest and effect commitment. Stage/episode are absent from
Source and belong to W-D2G's separately checked wrapper. Settlement is included
as a declaration, without claiming asset/scale authenticity (B10).

Agreement/domain/asset/scale/selector renaming changes this image. Nonce,
pre-head, action arguments, policy terms and snapshot updates do not. They
remain bound elsewhere as described below; exclusion never authorizes a
consumer to ignore them.

### Core implementation package payload, purpose3

Core/5 currently has no canonical code AST serializer. Recommend a
conservative implementation-source commitment with an explicit closed module
closure, rather than inventing a hash of a builtin name or an intent object.
It is a package identity for the current host prototype, not a ZKIR/native
executable identity or evidence of semantic equivalence.

Payload order:

```text
coreVersion=05:u8
coreProfile=text("moriarty-core/5")
sourceVersion=06:u8
intentSchema=text("moriarty-intent/3")
wireProfile=01:u8
coreProgramId=id(selected builtin)
operationKind=01:u8 | 02:u8
lowerEntry=text("parseAndLowerSource6")
prepareEntry=text("prepareMil4S0")
wrapperEntry=text("prepareSource6S0Unqualified")
fileCount=0003:u16be
file1Role=01:u8 || file1=blob(raw frontend/lowerer file bytes)
file2Role=02:u8 || file2=blob(raw Core file bytes)
file3Role=03:u8 || file3=blob(raw Source wrapper file bytes)
```

The exact file roles currently designate, relative to `experiments/moriarty-language/`:

1. `src/successor/financial-agreement-source-v6-frontend.ts`
2. `src/successor/mil4-s0-core-v5.ts`
3. `src/successor/mil4-s0-source-v6.ts`

Role bytes identify files; local checkout paths, filesystem metadata and Git
commit IDs are excluded. Raw complete file bytes are included: comments,
line endings, reserved-word set, bounds, helper functions, both operation
branches, lowering and wrapper comparison/precedence. No minification, import
stripping, AST pretty printing or selected-branch extraction is permitted.
No line ending, BOM or encoding repair is permitted. A freeze must inspect
this closure: the current local imports resolve within these three modules;
an added implementation dependency needs a new package-image profile, not an
uncommitted ambient helper. Each file is bounded at1MiB; total payload is
bounded at4MiB. These are proposed off-chain design limits, not measured
wallet, proof or ledger limits.

The builtin/kind pairs are the same two above. Hashing all branches binds
shared code; changing the repayment branch changes both builtin package
hashes even if transfer observations happen to remain equal. The selector
and constructor in the package distinguish the two identities despite their
shared files. Source/runtime instances, policy claims, snapshot/effects,
successor, compiled output, runtime binary, environment and authentication
receipts are excluded. SourceHash/policyDigest property names in implementation
text are ordinary code bytes, not embedded values from a stage proposal.

Exact code equality does not prove that those bytes were executed. B06 must
add independently verified lowering/implementation correspondence, actual
loaded-artifact binding and the execution/toolchain identity. A different
runtime, compiler, transformed output or native backend cannot be admitted by
presenting the same three source files. Their artifact/proof formats are
unselected here and remain unavailable; no default runtime or caller attestation
discharges this requirement.
The purpose3 labels must describe the actual role bytes: role1 must export
`parseAndLowerSource6`, role2 `prepareMil4S0`, and role3
`prepareSource6S0Unqualified`; their declared profiles and supported builtin/kind
must agree with the package labels. Valid fixed entry-name strings cannot
stand in for those actual exports or implementation checks. A renamed export
with stale labels, or fresh hashes over code that changes a declared version,
must fail B06's selected implementation correspondence.

Those entries describe only the existing local preparation path. None accepts
an authenticated complete-history handoff. A future semantic consumer's actual
B16 projection/handoff and its relation to this local lowerer must be independently
verified and bound as additional implementation evidence; this three-module
package does not commit an absent adapter. It cannot qualify full-history
execution by invoking the named wrapper unchanged or asserting an unused
`B16 verified` flag.

### Policy payload, purpose4

The policy is an exact bounded authorization configuration, scoped to the
selected definition and implementation. This proposal fixes only the action
arguments listed in its two variants and the validity bounds. Repayment's
snapshot-derived debtor/creditor remain outside the policy. It does not infer a reusable general policy
language, range permissions or dynamic counterparty selection.

Payload order:

```text
sourceVersion=06:u8 || coreVersion=05:u8 || wireProfile=01:u8
agreementInstanceId=id || domain=id || asset=id || scale:u8
actionId=id || coreProgramId=id || operationKind:u8
sourceHash=hash32(computed Source B digest)
coreHash=hash32(computed package digest)
signer=id(ast.intent.signer)
keyRef=text(ast.intent.keyRef)
validFrom=round(ast.intent.notBefore) || validUntil=round(ast.intent.notAfter)
grossCap=nominal || feeCap=nominal || netFloor=nominal
failureRelation=01:u8                    # W-D2F F-S0-01 only
supplyChanges=0000:u16be
observations=0000:u16be || disclosures=0000:u16be
retainedEffects=0000:u16be || retainedDuties=0000:u16be
delegation=00:u8 || recovery=00:u8
operation variant below
```

Transfer variant is byte`01`, then `owner:id, recipient:id, feeRecipient:id,
amount:nominal, fee:nominal`, from signedAction's `from,to,feeTo,value,fee`.
Repayment variant is byte`02`, then `obligationId:id,payer:id,amount:nominal,
allocation=01:u8,conversion=01:u8`. `allocation=01` means the selected
AccrualFirst builtin; `conversion=01` means the exact identity conversion.
Repayment debtor/creditor are not additional policy slots: Source signedAction
has no such fields. B11's authenticated obligation and tag35's exact signed
debtor/creditor checks still bind them. Do not add a caller creditor copy here.

The policy fields are copied from a successfully parsed Source proposal when
producing a candidate artifact. Only Source failure `success_only` maps through
F-S0-01 to wire `atomic-reject-terminal-success`, for closed S0 TerminalSuccess
with empty retained effects/duties and atomic unpublished rejection. It does
not adopt any accepted-failure branch or normative W-D2 failure semantics.
The empty/none fields are their own exact closed constructor comparisons,
not additional translations supplied by F-S0-01. `supplyChanges=[]` is the closed S0 wire constant and is
encoded explicitly despite Source having no supply-change slot. Both
operation discriminators must agree. Source actionId and CoreProgramId follow
W-D2G mapping A; equal text does not merge their nominal sorts.

**Excluded:** the embedded policyDigest itself, sourceHash's embedded claim,
nonce, intent.preHead, snapshot/prior-head/round/cells/replay/work, submitted
action/effects/successor, stage/episode, repayment debtor/creditor, keyScheme,
raw signerKey, signature, effect
commitment, authorization digest, proof and registry evidence. Computed Source
and Core digests are included instead of trusting claim strings. keyRef is
an exact decoded reference, not a key encoding; B08 still authenticates its
relation to the signed key. Fresh nonce/current head permit a repeated proposal
under the same configuration, subject to all existing replay/snapshot/authority
checks. Changing amount, zero-fee recipient, interval, keyRef or any cap changes
the policy image. No term is inferred from a net balance delta.
Excluded keyScheme remains signed at wire tag14 and verified under B09's
separately selected message/verifier rule. Excluded repayment debtor/creditor
remain signed in wire.operation and compared with retained B11 obligation
facts at their tag35 subfields, after B11 authenticates at18. No policy hash
or allocation literal can replace those checks.

## Acyclic producer dependencies and claim comparisons

Proposed production order is:

```text
parsed definition + selected package bytes -> sourceHash, coreHash
selected policy terms + those computed hashes -> policyHash
independent pre-state + bound operation + independent successor -> prepared effects
complete derived effect image -> effectCommitment
claims + all signed fields + effectCommitment -> canonical wire/3 -> content digest
```

The Source artifact can contain provisional placeholders in its two claim
slots while computing B's projection; fill the computed claims afterwards.
That is a generation procedure only. Consumer checks never accept placeholders.
Changing claims does not feed back into the Source or policy image. No policy
image contains its own digest; neither Source nor Core image contains policyHash.
No image includes authorizationDigest or signature. The independently selected
successor must remain digest-independent under B15; this proposal does not
define its generation or authenticate it.

The B05/B06/B07 provider relations must use the same trusted immutable view,
domain/agreement/context as B01–B04 and B11. Their authority/proof format and
trusted-root selection remain open. A locally recomputed digest establishes
content equality only. A registry hash, path, matching tuple, caller flag,
callback or author receipt is not an authenticated selected artifact.

## Proposed consumer anchors and exact lowering obligation

Preserve W-D2F F-wire → F-source → complete D sweep → tag-ordered M → H →
C → I → E. W-D2G mapping A is an input recommendation, not an adopted fact.
Its B01–B04 design is undergoing separate review; this sprint takes the
cross-layer binding anchors from W-D2F's current FIELD-MAP/PLAN and adopts no
W-D2G stage/episode/head authority rule.
Every downstream oracle requires all earlier gates to pass independently.
Missing providers today would stop at B01, not reach a hash-image positive.

| Anchor | Proposed authenticated fact and current-tag comparison |
| --- | --- |
| tag8/B05 | First compare wire.sourceHash to AST.selected.sourceHash. Then authenticate integrity/provenance of the declared Source artifact/definition and its selection, validate its exact purpose1 image, and retain its declared fields and computed digest. Perform the exact prior-body comparisons below before comparing current sourceHash to that digest. Retain asset/scale/constructor for16/17/35. |
| tag10 | Preserve Source selected.actionId == lowered intent.programId == signed coreProgramId and B04's retained selector/Core ID. Agreement ID is never Core programId. |
| tag11/B06 | Authenticate integrity of the declared exact package and independently verified correspondence/loaded-artifact binding, including actual export/label correspondence but no invented B16 handoff; compute purpose3 hash. Compare package profile/version/intent-schema labels with the already selected compatibility tuple and package CoreProgramId with retained B04/tag10 context, then current coreHash with the computed hash. Retain constructor for signed operation-kind equality at35. |
| tag12/B07 | First compare wire.policyHash to AST.selected.policyDigest. Then authenticate integrity/provenance of the declared policy artifact and its selected relation, decode its exact purpose4 payload and compute its hash. Perform the exact prior-context comparisons below immediately at12, then compare current policyHash with the artifact digest. Retain only later fields for their own anchors. |

At8, after the direct Source/wire hash comparison and B05 authentication/image
domain checks, compare the genuine Source definition body in this exact order:
wireProfile against retained V-S0-01 selection; domain against tag2;
agreementInstanceId against tag3/B01; selectedActionId against tag6/B04;
sourceVersion against tag7; profile against the formation-selected Source/6
profile. Only after all six pass compare signed sourceHash with the computed
purpose1 digest. Genuine differing body values return
`BindingRejected/W_D2F_FIELD_MISMATCH`, binding B05, comparisonTag8,
`field=sourceDefinition.<name>`, `inputPath=sourceDefinition.<name>`,
`factPath=B05.sourceDefinition.<name>`, `sourcePath=null`. For example a
genuine wrong domain names sourceDefinition.domain and wins before a
simultaneously stale computed-source hash. These are artifact body diagnostics
at8, not a rewind to an earlier signed tag or an invalid-proof diagnosis.
Absent/invalid B05 evidence wins before body comparisons; the direct Source
hash literal difference wins before authentication. Fixed profile/version
bytes still require image formation/domain validity before these equality
checks. Asset/scale/constructor stay deferred; changing one with old hash
claims therefore still produces the computed-hash mismatch at8.

Purpose4 keyRef validation is part of policy image-domain checking at12,
after the direct Source/wire policyHash comparison and B07 provider
authentication, but before all nine prior-body comparisons and before
computed-policyHash equality. Reject a malformed decoded scalar/text value,
empty value, either length-cap failure or a forbidden Core opaque control
with `BindingRejected/W_D2F_DOMAIN_UNSUPPORTED`, binding B07,
comparisonTag12, `field=policy.keyRef`, `inputPath=policy.keyRef`,
`factPath=B07.policy.keyRef`, `sourcePath=null`. This is a proposed image-domain
diagnostic, not an executed encoder result or a Source byte-offset error.
A validly shaped but wrong prior policy identity and an invalid policy.keyRef
therefore report the keyRef domain rejection first. Absent/invalid provider
evidence precedes that domain check. A Source-accepted candidate keyRef with
a decoded control is not substituted into the independent selected artifact:
if that artifact also contains the invalid value, it fails at12 as above;
if its keyRef is valid, tag12 may pass and the candidate's differing keyRef
fails the retained policy comparison at13, after B08 authentication, using
`W_D2F_FIELD_MISMATCH`, field signer, sourcePath intent.keyRef. A standalone
candidate policy producer encountering that invalid keyRef returns the same
BindingRejected/domain code with field policy.keyRef, inputPath intent.keyRef,
sourcePath intent.keyRef, and null comparisonTag/factPath (no consumer anchor).

At12, compare the authenticated policy body's earlier identities in this
exact wire-anchor order: wireProfile against the retained V-S0-01 selection;
domain against verified tag2 context; agreementInstanceId against tag3/B01;
actionId against tag6/B04; sourceVersion against tag7; sourceHash against
tag8's computed B05 digest; coreVersion against tag9; coreProgramId against
tag10/B04 and the actual lowered programId; coreHash against tag11's computed
B06 digest. Each role comparison uses its nominal sort. Passing the policy
provider's integrity/provenance check or matching external record links cannot
replace any of these body-field comparisons. Only after all nine pass compare
wire.policyHash with the computed policy artifact digest.

An authentic policy body with a wrong already bound identity is a
`BindingRejected/W_D2F_FIELD_MISMATCH` at anchor12/B07, `field=policy.<name>`,
`inputPath=policy.<name>`, `sourcePath=null`; for example `policy.domain`.
The owning diagnostic anchor remains12; do not rewind to tag2 or label a
genuine differing body value invalid cryptographic evidence. Wrong computed
hash links similarly use `policy.sourceHash` or `policy.coreHash`. If provider
evidence cannot authenticate its declared record/view/provenance, the result
instead is B07 invalid (with a real verifier) or unavailable (without one),
before any body comparisons. A provider selected under another scoped key
does not establish the required relation; it cannot be repaired by merely
matching the body. This distinguishes wrong provider selection from an
authentic correctly selected artifact whose body disagrees with prior facts.

For tag ordering, image producers may derive a policy/definition from a complete
candidate AST, but consumers recompute authenticated artifact images, then
retain their fields. They must not hash the candidate's later fields at8/12
and thereby diagnose an asset, scale, cap or endpoint mismatch early. Compare
retained Source/policy asset at16, scale at17, signer/keyRef relation at13,
validFrom/validUntil at21/22, caps/floor at23/24/25, failure at27, explicit
empty/none fields at28–34, and operation terms at35. Source keyRef equality
uses `sourcePath=intent.keyRef` under the signer binding, with no fictional
wire keyRef slot. B08's key authorization is still required there.
A valid bound policy whose later terms differ from submitted fields yields a
field mismatch at those fields. Asset/scale and operationKind are later terms;
tag12 does not compare them against submitted signed fields. Their typed
image-domain validation is still performed when decoding the policy artifact.

For a precise multi-provider order at23: first direct Source/wire grossCap
equality, then B14 provider authentication and already-bound grant scope
checks, then exact retained policy.grossCap equality, then B14's adopted
gross-cap authority predicate. Thus a direct mismatch wins first; missing/
invalid B14 evidence or a conflicting prior grant scope wins before a policy
cap mismatch; with genuine compatible grant scope, a policy cap mismatch wins
before a simultaneously false B14 gross-cap authority predicate. Repeat the
retained-policy-before-authority predicate order at24/25, without re-verifying
B14 or comparing either later field at23. This proposes a bounded refinement
of W-D2F's per-tag schedule, not a new canonical W-D3 order.
For the prior B14 grant-scope control, use the proposed retained scope tuple
`(domain,agreementInstanceId,signer,asset,preHead)`. After B14 authentication,
compare those components in that exact prior-wire-anchor order to retained
tags2/3/13/16/18, before retained policy.grossCap or the grant-cap predicate.
A genuine difference returns `BindingRejected/W_D2F_FIELD_MISMATCH`, binding
B14, comparisonTag23, sourcePath null, inputPath/factPath
`B14.grant.<component>`. The corresponding field is domain/agreementId/signer/
asset/preHead respectively. A wrong genuine grant domain thus names field
domain, factPath/inputPath B14.grant.domain and wins before a simultaneous
policy cap mismatch. These tuple names specify this proposed control, not an
implemented grant transport. Missing/invalid B14 evidence still precedes scope
equality, and a direct Source/wire grossCap mismatch precedes authentication.

At35, visit operation subfields in wire variant order. For each subfield,
compare Source signed action first, submitted action second where a Source
slot exists, then the applicable retained definition/B04 constructor,
retained policy term, and retained B11 obligation fact, in that order.
Repayment order is kind, obligationId, payer, debtor, creditor, amount,
allocation, conversion. B11 was authenticated at18: absent/invalid/conflicting
earlier snapshot evidence prevents35 from being reached. With compatible
earlier evidence, a policy payer mismatch wins before a genuine B11 debtor
or creditor value mismatch because payer occurs first; a debtor/creditor
mismatch wins before a policy amount mismatch because those fields precede
amount. At obligationId, applicable policy equality precedes retained B11 ID
equality after the direct Source checks. No debtor/creditor policy slot exists.

**Lowering correspondence must establish all of the following, specified-only:**

1. The actual parser/lowerer/preparer/wrapper are the selected exact package,
   with a verified execution/artifact mapping. A hash of source text alone is
   insufficient. Finite matching examples alone are not a universal proof.
2. Existing formation and bounds precede lowering. Source profile/selected
   constructor determines the same two supported Core/kind pairs, and every
   common-domain amount/round/identifier is copied exactly without coercion.
3. Lowered intent fields equal the frontend mapping: versions, selected action
   as Core programId, Source/policy claims, keyRef, domain/asset, signer/nonce,
   preHead/validity/caps/floor; transfer recipient/feeRecipient/amount/fee or
   repayment obligationId/amount. Transfer owner and repayment payer are
   bound through signer plus the Source signed action, never erased.
4. Local lowered state copies ordered cells, both work counters and claims;
   agreement/scale/predecessor stay in the retained AST/context and are checked
   at their W-D2F anchors. Dropped Core slots are not evidence of irrelevance.
5. Submitted effects preserve order and values; only UseReplay transforms
   its nonce into the exact composite tuple. Source signedAction/submitted
   action equality is checked by the selected wrapper; bare lowerSource6
   does not establish this equality. Stage-first wrapper diagnostics are
   accounted for separately from the outer adapter's proposed ordering.
6. Source replay `unused/consumed` is only a selected-key claim. The full
   authenticated history enters the consumer through an adopted B16 projection,
   and is never replaced by the local lowerer's synthesized []/[key]. That
   projection/transport is unselected, not supplied by this hash design.
7. Read-only Core derives the complete ordered effects and post cells before
   supplied-vector comparison, including funded repayment/creditor binding,
   gross consumption, checked arithmetic, work/replay/head changes and no
   published post/effects on rejection. Neither an image nor policy chooses
   those effects. Signatures, snapshots, successors and atomic ledger consumption
   remain separately verified B09/B11/B15/B17 obligations.

No lowering proof, runtime evidence or new transport is supplied here. A
future bounded regression suite may demonstrate its specific cases; it must
state its domain and not rename them a full compiler-correctness theorem.

## Diagnostics and compatibility consequences

Use W-D2F's existing proposed families. Literal/computed commitment difference
is `BindingRejected/W_D2F_FIELD_MISMATCH` at sourceHash8, coreHash11 or policyHash12.
Missing adopted definition/artifact/correspondence/provider is
`BindingRejected/W_D2F_BINDING_UNAVAILABLE`, identifying B05/B06/B07 at its
anchor. An implemented verifier finding invalid scoped selection, provenance,
closure or lowering evidence gives `EvidenceRejected/W_D2F_EVIDENCE_INVALID`
at that binding. A valid provider of a later differing field remains a
field mismatch at that field. No adapter diagnosis receives a fabricated
Source byte offset. Every rejection has null published post/effects.
Image-parser codes and artifact/proof transports are not implemented or
silently borrowed from the authorization decoder.

Wire/3 can hold these three 32-byte values without changing field layout.
That fact does not adopt their meaning. Because /3 has no separate image-version
slot, this exact image suite must be selected by an authenticated profile/view;
unknown, ambiguous or mixed suites reject as unavailable/invalid according to
whether their definition/evidence is absent/invalid. Select one authenticated
suite before any image comparison; never try another purpose or suite to find
a matching hash. Purpose2 is only a preimage alternative and is unavailable
to this B-specific consumer path. Concurrent suites need an explicit authenticated version
binding or a separately reviewed wire/profile version; no wire/4 is chosen here.

- Definition image changes to field inclusion, constructor meaning, bounds,
  serialization or domain prefix require a new image/profile version and
  fresh independent vectors. A future Source grammar with real action bodies
  needs a new committed body/lowering relation, not reuse of this projection.
- Exact package byte changes, even comments/line endings or an unused branch,
  change coreHash. Compatible behavior may retain Core/5 only after an explicit
  compatibility/correspondence decision; changed semantics require versioned
  contract review. A hash change alone never certifies compatibility.
- New imports or altered package roles/encoding require a reviewed package-image
  profile. A different execution/toolchain/backend mapping requires fresh
  correspondence and loaded-artifact evidence; it changes coreHash only when
  committed package bytes/labels change, since executable/runtime bytes are
  excluded from purpose3. Same source hash never carries execution evidence
  across that change. No compiler/toolchain change inherits prior correspondence.
- Changing policy field inclusion, interpretation, bounds, failure relation,
  purpose or encoding requires a new policy-image/suite version and independent
  vectors. Ordinary changed policy values under the same schema need fresh
  policyHash/authentication, without automatically changing Source/6 or Core/5.
  An extension beyond closed F-S0-01 additionally needs its own reviewed
  Source/Core/failure relation; purpose4 byte01 cannot authorize that extension.
- Policy terms, settlement/instance selection or package changes require fresh
  policyHash and authentication under the selected view. Definition renaming
  changes sourceHash and policyHash. Presentation changes to the Source proposal
  alone leave B's images unchanged; editing committed package text does not.
- Any changed wire hash changes canonical wire bytes/content digest and needs
  fresh signature evidence under the eventual B09 scheme. Hash migration does
  not reset allowances, spent work, nonce consumption, heads, continuing duties
  or stage/episode identity. No implicit registry rebinding is permitted.
- Historical wire/W-D2E positives remain evidence for their original purposes.
  They have no authenticated Source/Core/policy images and use actionId=Action;
  they cannot be reclassified as full-consumer positives. No fixture is rewritten.

Independent review must adopt the image suite and its compatibility before
implementation. Runtime correspondence, registry authority, B16 transport,
independent expected byte/digest vectors, real authentication and native/ledger
evidence remain open. This proposal makes those requirements concrete without
claiming any gate is closed.

```

## experiments/moriarty-language/formal/mil4/hash-images/SPEC.md

sha256: `7f6ebf64002d8cbae45de22ed759eb71dce919735c4160deb014bc0673e7232d`

```text
# W-D2H B05–B07 hash-image and lowering proposal

**Status: proposed / specified-only, 2026-09-30.** Scope is the closed
Source/6 → Core/5 S0 proposal and authorization wire/3. This sprint supplies
design definitions and design oracles only. It implements no encoder, hash
producer, consumer, registry, authentication, correspondence proof or test.
The recommendation is not an adopted decision. W-D2/W-D3 and B01–B17 remain
open. No existing `source_hash`, `digest`, `sourceHash`, `coreHash` or
`policyHash` claim is promoted to a computed commitment.

## Inspected repository facts

The [Source contract](../../../spec/successor/financial-agreement-source-v6.md),
[grammar](../../../spec/successor/financial-agreement-source-v6-grammar.ebnf),
[frontend/lowerer](../../../src/successor/financial-agreement-source-v6-frontend.ts),
[Core preparer](../../../src/successor/mil4-s0-core-v5.ts),
[Source wrapper](../../../src/successor/mil4-s0-source-v6.ts),
[S0 contract](../s0-implementation-contract.md),
[wire/3 specification](../wire/SPEC.md),
[W-D2F field map](../effect-consumer/FIELD-MAP.md),
[W-D2F plan](../effect-consumer/PLAN.md) and
[W-D2G identity proposal](../identity-binding/SPEC.md) are inputs, unchanged.

Repository observations:

- Source/6 contains one stage proposal selecting a builtin. It has no
  user-defined action body. The only selectors are `TransferLiteralFee` and
  `RepayAccrualFirst`, checked against the signed action constructor.
- The identifier after `agreement` is `ast.programId`. Lowering instead sets
  `Core.intent.programId=ast.selected.actionId`. These are different roles.
- `selected source_hash` and `selected digest` parse as nonempty opaque
  strings. The latter is the policy claim, not an authorization digest.
  The lowerer copies both strings; Core checks opaque shape only.
- Lowering constructs local state/intent/effects, preserves both work
  counters, and converts a replay nonce to `JSON.stringify([domain,signer,nonce])`.
  It projects a selected replay claim to `[]` or `[selectedTuple]`; that is
  not complete authenticated replay history.
- Agreement, scale and authenticated predecessor survive only in the AST.
  The wrapper also reports selected-program binding as unverified. It compares
  signed/submitted actions after Stage and withholds a prepared candidate on
  disagreement. No exact signed digest is calculated.
- Wire/3 tags8,11,12 are supplied 32-byte Source/Core/policy claims. Its
  current authorization content digest is SHA256 of all canonical wire bytes.
  Neither a wire round trip nor equal digest claims proves their provenance.

## Two non-self-referential Source strategies

Both strategies require successful existing Source formation and W-D2F's
common-domain check before a candidate comparison. Neither hashes a document
including its own `source_hash` value. Neither recursively hashes the policy
claim. [DECISION-MATRIX](DECISION-MATRIX.md) compares consequences.

**A — parsed-location document projection, preimage alternative only.** Take the exact UTF-8 source
document, including comments, presentation whitespace, every other token,
snapshot, submitted effects and successor. Replace exactly the entire JSON
string token at `selected.sourceHash` with the ASCII token
`"<excluded:source_hash>"`, and exactly the token at `selected.policyDigest`
with `"<excluded:digest>"`. Preserve all remaining bytes, including whitespace
around the replaced tokens. Locate these two unique fields by the successfully
parsed grammar, never by regex, global string replacement or a caller's byte
offsets. This token-span extraction would be new work: the current returned
AST does not expose these spans. No other occurrence in comments, keys or
strings is masked. Purpose2 below hashes this projected document. The projected
payload is bounded at65573 bytes: Source's65536-byte bound plus at most37 bytes
of growth when its two nonempty quoted tokens are replaced by the24-byte and
19-byte markers. The encoder must count that actual projected length.

This is a commitment to a particular stage presentation with two excluded
claims. It changes on whitespace, comment, snapshot and independent successor
changes. It cannot be called a code-only image or reused as a stable action
definition. Changing either excluded claim cannot change its own source image;
the claims still require independent checks. This proposal defines A's preimage
only. It does not define an A-specific authenticated consumer, policy linkage
or diagnostic schedule; purpose4 and the consumer sections below select B.
A cannot substitute into that path or be tried as a fallback.

**B — canonical selected-definition projection, recommended.** From a parsed
Source artifact extract only the ordered definition tuple below. Purpose1
hashes its binary encoding. Presentation changes do not change the tuple.
The definition identifies an instance-scoped builtin selection and settlement
declaration; the exact builtin/lowerer implementation is separately committed
by the Core package. No new user-defined code or template-to-instance relation
is inferred. This limited interpretation is necessary because Source/6 has no
action body. A later grammar with action bodies must commit those bodies under
a new image profile; dropping them is forbidden.

## Exact proposed byte rules

The following are new definitions, not existing codec behavior.

For purposes1–4 define the preimage:

```text
ASCII("moriarty-mil4-image/1") || 00 || purpose:u8 || payloadLength:u32be || payload
digest = SHA256(preimage)
```

`purpose` is exactly `01` Source definition B, `02` projected document A,
`03` Core implementation package, or `04` policy. No other purpose, extension,
padding or trailing byte is accepted. `payloadLength` counts payload bytes,
not the envelope. Digests are 32 raw bytes inside another image and exactly
64 lowercase hex characters in a Source/wire presentation. These domain
prefixes do not change wire/3's existing authorization content-digest rule or
select B09's signature-message preprocessing.

Use these primitives, all with exact length bounds before allocation:

| Primitive | Proposed bytes and domain |
| --- | --- |
| `id` | UInt16 big-endian byte count then exact ASCII; W-D2F common Source identifier subtype and reserved-word exclusion; 1–64 bytes |
| `text` | UInt16 big-endian byte count then exactly UTF-8 encoding of the decoded Unicode scalar sequence; 1–1024 bytes; no normalization, trimming, case folding or original JSON escape bytes |
| `nominal` | UInt128 big-endian, value0..2^127−1; Source canonical decimal parsed exactly |
| `round` | UInt64 big-endian, value0..2^64−1; explicit Source/wire intersection, no truncation |
| `scale` | One byte0..18; no rescaling |
| `hash32` | Exactly32 raw bytes; no prefix or text encoding |
| `blob` | UInt32 big-endian byte count then exact bytes |

Integer presentation leading zeros and malformed strings already reject
formation/domain checking; encoders must not repair them. The binary tuple
has no field names, implicit defaults or optional slots. Each fixed byte
below must be encoded even when derivable from another field. An image
encoder accepts a closed typed value, not arbitrary object enumeration.
Purpose1 and purpose4 payloads each have a4096-byte proposed admission cap.
All displayed integer/role constants with leading zeros are hexadecimal
encoded-byte spellings, not Source decimal presentation. Thus `0003:u16be`
means exactly the two bytes00 03; `0000:u16be` means00 00.
UTF-8 uses the shortest valid encoding of each scalar, with no BOM or NUL
terminator. Lone surrogates, invalid UTF-8 and replacement-decoder repair
reject. Purpose4 keyRef additionally satisfies Core/5's opaque admission:
nonempty, at most1024 UTF-16 code units, and no U+0000..U+001F or U+007F.
The1024-byte text cap is simultaneous, not a replacement for this restriction.
Escaped JSON control characters decode before that check; `"\u0000"` cannot
be admitted by retaining escape text. Source formation may accept a decoded
control in an opaque string; the proposed policy image rejects such a keyRef
and does not invent a Source parser error or expand W-D2F's D sweep.

### Source definition payload B, purpose1

Order is exact:

```text
sourceVersion=06:u8
profile=text("moriarty-financial-agreement-source/6")
wireProfile=01:u8                         # s0-provisional/1
agreementInstanceId=id(ast.programId)
domain=id(ast.domain)
asset=id(ast.settlement.asset)
scale=u8(ast.settlement.scale)
selectedActionId=id(ast.selected.actionId)
operationKind=01:u8 | 02:u8               # transfer | repayment
```

The only permitted selector/kind pairs are
`(TransferLiteralFee,01)` and `(RepayAccrualFirst,02)`. The kind is checked
against `ast.intent.signedAction.kind`; it is not chosen by a caller independently.
Source version and source profile both participate; no `/5` fallback exists.

**Included:** the nine fields above only. **Excluded:** original token spellings,
comments, whitespace, both embedded digest claims, all `intent` fields beyond
the constructor discriminator, the entire authenticated block, submitted
action/effects/post-head, signature/proof/receipt bytes, wire stage/episode,
authorization digest and effect commitment. Stage/episode are absent from
Source and belong to W-D2G's separately checked wrapper. Settlement is included
as a declaration, without claiming asset/scale authenticity (B10).

Agreement/domain/asset/scale/selector renaming changes this image. Nonce,
pre-head, action arguments, policy terms and snapshot updates do not. They
remain bound elsewhere as described below; exclusion never authorizes a
consumer to ignore them.

### Core implementation package payload, purpose3

Core/5 currently has no canonical code AST serializer. Recommend a
conservative implementation-source commitment with an explicit closed module
closure, rather than inventing a hash of a builtin name or an intent object.
It is a package identity for the current host prototype, not a ZKIR/native
executable identity or evidence of semantic equivalence.

Payload order:

```text
coreVersion=05:u8
coreProfile=text("moriarty-core/5")
sourceVersion=06:u8
intentSchema=text("moriarty-intent/3")
wireProfile=01:u8
coreProgramId=id(selected builtin)
operationKind=01:u8 | 02:u8
lowerEntry=text("parseAndLowerSource6")
prepareEntry=text("prepareMil4S0")
wrapperEntry=text("prepareSource6S0Unqualified")
fileCount=0003:u16be
file1Role=01:u8 || file1=blob(raw frontend/lowerer file bytes)
file2Role=02:u8 || file2=blob(raw Core file bytes)
file3Role=03:u8 || file3=blob(raw Source wrapper file bytes)
```

The exact file roles currently designate, relative to `experiments/moriarty-language/`:

1. `src/successor/financial-agreement-source-v6-frontend.ts`
2. `src/successor/mil4-s0-core-v5.ts`
3. `src/successor/mil4-s0-source-v6.ts`

Role bytes identify files; local checkout paths, filesystem metadata and Git
commit IDs are excluded. Raw complete file bytes are included: comments,
line endings, reserved-word set, bounds, helper functions, both operation
branches, lowering and wrapper comparison/precedence. No minification, import
stripping, AST pretty printing or selected-branch extraction is permitted.
No line ending, BOM or encoding repair is permitted. A freeze must inspect
this closure: the current local imports resolve within these three modules;
an added implementation dependency needs a new package-image profile, not an
uncommitted ambient helper. Each file is bounded at1MiB; total payload is
bounded at4MiB. These are proposed off-chain design limits, not measured
wallet, proof or ledger limits.

The builtin/kind pairs are the same two above. Hashing all branches binds
shared code; changing the repayment branch changes both builtin package
hashes even if transfer observations happen to remain equal. The selector
and constructor in the package distinguish the two identities despite their
shared files. Source/runtime instances, policy claims, snapshot/effects,
successor, compiled output, runtime binary, environment and authentication
receipts are excluded. SourceHash/policyDigest property names in implementation
text are ordinary code bytes, not embedded values from a stage proposal.

Exact code equality does not prove that those bytes were executed. B06 must
add independently verified lowering/implementation correspondence, actual
loaded-artifact binding and the execution/toolchain identity. A different
runtime, compiler, transformed output or native backend cannot be admitted by
presenting the same three source files. Their artifact/proof formats are
unselected here and remain unavailable; no default runtime or caller attestation
discharges this requirement.
The purpose3 labels must describe the actual role bytes: role1 must export
`parseAndLowerSource6`, role2 `prepareMil4S0`, and role3
`prepareSource6S0Unqualified`; their declared profiles and supported builtin/kind
must agree with the package labels. Valid fixed entry-name strings cannot
stand in for those actual exports or implementation checks. A renamed export
with stale labels, or fresh hashes over code that changes a declared version,
must fail B06's selected implementation correspondence.

Those entries describe only the existing local preparation path. None accepts
an authenticated complete-history handoff. A future semantic consumer's actual
B16 projection/handoff and its relation to this local lowerer must be independently
verified and bound as additional implementation evidence; this three-module
package does not commit an absent adapter. It cannot qualify full-history
execution by invoking the named wrapper unchanged or asserting an unused
`B16 verified` flag.

### Policy payload, purpose4

The policy is an exact bounded authorization configuration, scoped to the
selected definition and implementation. This proposal fixes only the action
arguments listed in its two variants and the validity bounds. Repayment's
snapshot-derived debtor/creditor remain outside the policy. It does not infer a reusable general policy
language, range permissions or dynamic counterparty selection.

Payload order:

```text
sourceVersion=06:u8 || coreVersion=05:u8 || wireProfile=01:u8
agreementInstanceId=id || domain=id || asset=id || scale:u8
actionId=id || coreProgramId=id || operationKind:u8
sourceHash=hash32(computed Source B digest)
coreHash=hash32(computed package digest)
signer=id(ast.intent.signer)
keyRef=text(ast.intent.keyRef)
validFrom=round(ast.intent.notBefore) || validUntil=round(ast.intent.notAfter)
grossCap=nominal || feeCap=nominal || netFloor=nominal
failureRelation=01:u8                    # W-D2F F-S0-01 only
supplyChanges=0000:u16be
observations=0000:u16be || disclosures=0000:u16be
retainedEffects=0000:u16be || retainedDuties=0000:u16be
delegation=00:u8 || recovery=00:u8
operation variant below
```

Transfer variant is byte`01`, then `owner:id, recipient:id, feeRecipient:id,
amount:nominal, fee:nominal`, from signedAction's `from,to,feeTo,value,fee`.
Repayment variant is byte`02`, then `obligationId:id,payer:id,amount:nominal,
allocation=01:u8,conversion=01:u8`. `allocation=01` means the selected
AccrualFirst builtin; `conversion=01` means the exact identity conversion.
Repayment debtor/creditor are not additional policy slots: Source signedAction
has no such fields. B11's authenticated obligation and tag35's exact signed
debtor/creditor checks still bind them. Do not add a caller creditor copy here.

The policy fields are copied from a successfully parsed Source proposal when
producing a candidate artifact. Only Source failure `success_only` maps through
F-S0-01 to wire `atomic-reject-terminal-success`, for closed S0 TerminalSuccess
with empty retained effects/duties and atomic unpublished rejection. It does
not adopt any accepted-failure branch or normative W-D2 failure semantics.
The empty/none fields are their own exact closed constructor comparisons,
not additional translations supplied by F-S0-01. `supplyChanges=[]` is the closed S0 wire constant and is
encoded explicitly despite Source having no supply-change slot. Both
operation discriminators must agree. Source actionId and CoreProgramId follow
W-D2G mapping A; equal text does not merge their nominal sorts.

**Excluded:** the embedded policyDigest itself, sourceHash's embedded claim,
nonce, intent.preHead, snapshot/prior-head/round/cells/replay/work, submitted
action/effects/successor, stage/episode, repayment debtor/creditor, keyScheme,
raw signerKey, signature, effect
commitment, authorization digest, proof and registry evidence. Computed Source
and Core digests are included instead of trusting claim strings. keyRef is
an exact decoded reference, not a key encoding; B08 still authenticates its
relation to the signed key. Fresh nonce/current head permit a repeated proposal
under the same configuration, subject to all existing replay/snapshot/authority
checks. Changing amount, zero-fee recipient, interval, keyRef or any cap changes
the policy image. No term is inferred from a net balance delta.
Excluded keyScheme remains signed at wire tag14 and verified under B09's
separately selected message/verifier rule. Excluded repayment debtor/creditor
remain signed in wire.operation and compared with retained B11 obligation
facts at their tag35 subfields, after B11 authenticates at18. No policy hash
or allocation literal can replace those checks.

## Acyclic producer dependencies and claim comparisons

Proposed production order is:

```text
parsed definition + selected package bytes -> sourceHash, coreHash
selected policy terms + those computed hashes -> policyHash
independent pre-state + bound operation + independent successor -> prepared effects
complete derived effect image -> effectCommitment
claims + all signed fields + effectCommitment -> canonical wire/3 -> content digest
```

The Source artifact can contain provisional placeholders in its two claim
slots while computing B's projection; fill the computed claims afterwards.
That is a generation procedure only. Consumer checks never accept placeholders.
Changing claims does not feed back into the Source or policy image. No policy
image contains its own digest; neither Source nor Core image contains policyHash.
No image includes authorizationDigest or signature. The independently selected
successor must remain digest-independent under B15; this proposal does not
define its generation or authenticate it.

The B05/B06/B07 provider relations must use the same trusted immutable view,
domain/agreement/context as B01–B04 and B11. Their authority/proof format and
trusted-root selection remain open. A locally recomputed digest establishes
content equality only. A registry hash, path, matching tuple, caller flag,
callback or author receipt is not an authenticated selected artifact.

## Proposed consumer anchors and exact lowering obligation

Preserve W-D2F F-wire → F-source → complete D sweep → tag-ordered M → H →
C → I → E. W-D2G mapping A is an input recommendation, not an adopted fact.
Its B01–B04 design is undergoing separate review; this sprint takes the
cross-layer binding anchors from W-D2F's current FIELD-MAP/PLAN and adopts no
W-D2G stage/episode/head authority rule.
Every downstream oracle requires all earlier gates to pass independently.
Missing providers today would stop at B01, not reach a hash-image positive.

| Anchor | Proposed authenticated fact and current-tag comparison |
| --- | --- |
| tag8/B05 | First compare wire.sourceHash to AST.selected.sourceHash. Then authenticate integrity/provenance of the declared Source artifact/definition and its selection, validate its exact purpose1 image, and retain its declared fields and computed digest. Perform the exact prior-body comparisons below before comparing current sourceHash to that digest. Retain asset/scale/constructor for16/17/35. |
| tag10 | Preserve Source selected.actionId == lowered intent.programId == signed coreProgramId and B04's retained selector/Core ID. Agreement ID is never Core programId. |
| tag11/B06 | Authenticate integrity of the declared exact package and independently verified correspondence/loaded-artifact binding, including actual export/label correspondence but no invented B16 handoff; compute purpose3 hash. Compare package profile/version/intent-schema labels with the already selected compatibility tuple and package CoreProgramId with retained B04/tag10 context, then current coreHash with the computed hash. Retain constructor for signed operation-kind equality at35. |
| tag12/B07 | First compare wire.policyHash to AST.selected.policyDigest. Then authenticate integrity/provenance of the declared policy artifact and its selected relation, decode its exact purpose4 payload and compute its hash. Perform the exact prior-context comparisons below immediately at12, then compare current policyHash with the artifact digest. Retain only later fields for their own anchors. |

At8, after the direct Source/wire hash comparison and B05 authentication/image
domain checks, compare the genuine Source definition body in this exact order:
wireProfile against retained V-S0-01 selection; domain against tag2;
agreementInstanceId against tag3/B01; selectedActionId against tag6/B04;
sourceVersion against tag7; profile against the formation-selected Source/6
profile. Only after all six pass compare signed sourceHash with the computed
purpose1 digest. Genuine differing body values return
`BindingRejected/W_D2F_FIELD_MISMATCH`, binding B05, comparisonTag8,
`field=sourceDefinition.<name>`, `inputPath=sourceDefinition.<name>`,
`factPath=B05.sourceDefinition.<name>`, `sourcePath=null`. For example a
genuine wrong domain names sourceDefinition.domain and wins before a
simultaneously stale computed-source hash. These are artifact body diagnostics
at8, not a rewind to an earlier signed tag or an invalid-proof diagnosis.
Absent/invalid B05 evidence wins before body comparisons; the direct Source
hash literal difference wins before authentication. Fixed profile/version
bytes still require image formation/domain validity before these equality
checks. Asset/scale/constructor stay deferred; changing one with old hash
claims therefore still produces the computed-hash mismatch at8.

Purpose4 keyRef validation is part of policy image-domain checking at12,
after the direct Source/wire policyHash comparison and B07 provider
authentication, but before all nine prior-body comparisons and before
computed-policyHash equality. Reject a malformed decoded scalar/text value,
empty value, either length-cap failure or a forbidden Core opaque control
with `BindingRejected/W_D2F_DOMAIN_UNSUPPORTED`, binding B07,
comparisonTag12, `field=policy.keyRef`, `inputPath=policy.keyRef`,
`factPath=B07.policy.keyRef`, `sourcePath=null`. This is a proposed image-domain
diagnostic, not an executed encoder result or a Source byte-offset error.
A validly shaped but wrong prior policy identity and an invalid policy.keyRef
therefore report the keyRef domain rejection first. Absent/invalid provider
evidence precedes that domain check. A Source-accepted candidate keyRef with
a decoded control is not substituted into the independent selected artifact:
if that artifact also contains the invalid value, it fails at12 as above;
if its keyRef is valid, tag12 may pass and the candidate's differing keyRef
fails the retained policy comparison at13, after B08 authentication, using
`W_D2F_FIELD_MISMATCH`, field signer, sourcePath intent.keyRef. A standalone
candidate policy producer encountering that invalid keyRef returns the same
BindingRejected/domain code with field policy.keyRef, inputPath intent.keyRef,
sourcePath intent.keyRef, and null comparisonTag/factPath (no consumer anchor).

At12, compare the authenticated policy body's earlier identities in this
exact wire-anchor order: wireProfile against the retained V-S0-01 selection;
domain against verified tag2 context; agreementInstanceId against tag3/B01;
actionId against tag6/B04; sourceVersion against tag7; sourceHash against
tag8's computed B05 digest; coreVersion against tag9; coreProgramId against
tag10/B04 and the actual lowered programId; coreHash against tag11's computed
B06 digest. Each role comparison uses its nominal sort. Passing the policy
provider's integrity/provenance check or matching external record links cannot
replace any of these body-field comparisons. Only after all nine pass compare
wire.policyHash with the computed policy artifact digest.

An authentic policy body with a wrong already bound identity is a
`BindingRejected/W_D2F_FIELD_MISMATCH` at anchor12/B07, `field=policy.<name>`,
`inputPath=policy.<name>`, `sourcePath=null`; for example `policy.domain`.
The owning diagnostic anchor remains12; do not rewind to tag2 or label a
genuine differing body value invalid cryptographic evidence. Wrong computed
hash links similarly use `policy.sourceHash` or `policy.coreHash`. If provider
evidence cannot authenticate its declared record/view/provenance, the result
instead is B07 invalid (with a real verifier) or unavailable (without one),
before any body comparisons. A provider selected under another scoped key
does not establish the required relation; it cannot be repaired by merely
matching the body. This distinguishes wrong provider selection from an
authentic correctly selected artifact whose body disagrees with prior facts.

For tag ordering, image producers may derive a policy/definition from a complete
candidate AST, but consumers recompute authenticated artifact images, then
retain their fields. They must not hash the candidate's later fields at8/12
and thereby diagnose an asset, scale, cap or endpoint mismatch early. Compare
retained Source/policy asset at16, scale at17, signer/keyRef relation at13,
validFrom/validUntil at21/22, caps/floor at23/24/25, failure at27, explicit
empty/none fields at28–34, and operation terms at35. Source keyRef equality
uses `sourcePath=intent.keyRef` under the signer binding, with no fictional
wire keyRef slot. B08's key authorization is still required there.
A valid bound policy whose later terms differ from submitted fields yields a
field mismatch at those fields. Asset/scale and operationKind are later terms;
tag12 does not compare them against submitted signed fields. Their typed
image-domain validation is still performed when decoding the policy artifact.

For asset/tag16 use this total order: (1) direct wire.asset versus Source
settlement.asset; (2) B10 provider integrity/selection authentication, then
its genuine prior domain and agreementInstanceId scope facts against retained
tags2/3 in that order; (3) wire.asset membership in retained B01.assetIds;
(4) wire.asset versus retained B10.asset; (5) versus retained B05.definition.asset;
(6) versus retained B07.policy.asset. Missing/invalid B10 evidence is evaluated
at step2 and cannot be skipped because a later asset fact also differs.
B01 associations are membership sets, not singleton asset identities.

At scale/tag17 the total order is direct wire.scale versus Source settlement.scale,
then versus retained B10.scale, then B05.definition.scale, then B07.policy.scale.
No provider is reauthenticated and no scale equality is moved into16. The
existing F/D formation/intersection checks remain prerequisites.

All genuine asset/scale differences use `BindingRejected/W_D2F_FIELD_MISMATCH`
with comparisonTag16/17 and field asset/scale. Direct Source differences name
inputPath wire.asset/scale, sourcePath settlement.asset/scale, and null binding/
factPath. For a retained fact or B01 membership difference, inputPath is
wire.asset/scale, sourcePath null, and binding/factPath are respectively
B01/B01.assetIds, B10/B10.asset or B10.scale, B05/B05.definition.asset or
B05.definition.scale, B07/B07.policy.asset or B07.policy.scale. The B05
definition paths name the retained purpose1 declaration slots, distinct from
tag8's prior-identity diagnostic paths. B10 genuine prior domain/agreement
scope differences at16 use field domain/agreementId, inputPath wire.domain/
agreementId, binding B10, factPath B10.domain/agreementInstanceId and sourcePath
null. Missing B10 gives BindingRejected/W_D2F_BINDING_UNAVAILABLE; failed actual
authentication gives EvidenceRejected/W_D2F_EVIDENCE_INVALID, both field asset,
binding B10, comparisonTag16, inputPath wire.asset, sourcePath/factPath null.
No fabricated parser offset or future-field comparison is added.

For a precise multi-provider order at23: first direct Source/wire grossCap
equality, then B14 provider authentication and already-bound grant scope
checks, then exact retained policy.grossCap equality, then B14's adopted
gross-cap authority predicate. Thus a direct mismatch wins first; missing/
invalid B14 evidence or a conflicting prior grant scope wins before a policy
cap mismatch; with genuine compatible grant scope, a policy cap mismatch wins
before a simultaneously false B14 gross-cap authority predicate. Repeat the
retained-policy-before-authority predicate order at24/25, without re-verifying
B14 or comparing either later field at23. This proposes a bounded refinement
of W-D2F's per-tag schedule, not a new canonical W-D3 order.
For the prior B14 grant-scope control, use the proposed retained scope tuple
`(domain,agreementInstanceId,signer,asset,preHead)`. After B14 authentication,
compare those components in that exact prior-wire-anchor order to retained
tags2/3/13/16/18, before retained policy.grossCap or the grant-cap predicate.
A genuine difference returns `BindingRejected/W_D2F_FIELD_MISMATCH`, binding
B14, comparisonTag23, sourcePath null, inputPath/factPath
`B14.grant.<component>`. The corresponding field is domain/agreementId/signer/
asset/preHead respectively. A wrong genuine grant domain thus names field
domain, factPath/inputPath B14.grant.domain and wins before a simultaneous
policy cap mismatch. These tuple names specify this proposed control, not an
implemented grant transport. Missing/invalid B14 evidence still precedes scope
equality, and a direct Source/wire grossCap mismatch precedes authentication.

At35, visit operation subfields in wire variant order. For each existing Source
action slot compare signed Source then submitted Source before retained facts.
For debtor/creditor, compare the Source authenticated obligation cell before
the retained B11 fact, as explicitly ordered below. Otherwise compare the
applicable retained definition/B04 constructor, retained policy term, and
retained B11 obligation fact after direct Source checks.
Repayment order is kind, obligationId, payer, debtor, creditor, amount,
allocation, conversion. B11 was authenticated at18: absent/invalid/conflicting
earlier snapshot evidence prevents35 from being reached. With compatible
earlier evidence, a policy payer mismatch wins before a genuine B11 debtor
or creditor value mismatch because payer occurs first; a debtor/creditor
mismatch wins before a policy amount mismatch because those fields precede
amount. At obligationId, applicable policy equality precedes retained B11 ID
equality after the direct Source checks. No debtor/creditor policy slot exists.

At operation.debtor, first compare wire.operation.debtor with
AST.authenticated.obligation.debtor; only if equal compare the retained
B11.snapshot.obligations[0].debtor and its relation to the already bound payer/
signer. At operation.creditor, first compare wire.operation.creditor with
AST.authenticated.obligation.creditor, then the retained
B11.snapshot.obligations[0].creditor. Each Source-cell difference is
BindingRejected/W_D2F_FIELD_MISMATCH, comparisonTag35, field operation.debtor/
creditor, inputPath wire.operation.debtor/creditor, sourcePath
authenticated.obligation.debtor/creditor, with null binding/factPath. Only equal
wire/Source claims can reach a genuine B11 fact difference; that uses the same
status/code/field/comparisonTag/inputPath, binding B11, sourcePath null and
factPath B11.snapshot.obligations[0].debtor/creditor. Distinct X/Y controls must
keep Source formation valid: signer/payer/Source debtor agree, obligation/cell
ordering and endpoint aliases satisfy the parser. Source mismatch wins for
both wireX/SourceY/B11Y and wireX/SourceY/B11X. WireY/SourceY/B11X instead reaches
the genuine B11 mismatch. A matching B11 fact never skips the earlier Source
cell check, and the later unsigned snapshot tail cannot replace this check.

**Lowering correspondence must establish all of the following, specified-only:**

1. The actual parser/lowerer/preparer/wrapper are the selected exact package,
   with a verified execution/artifact mapping. A hash of source text alone is
   insufficient. Finite matching examples alone are not a universal proof.
2. Existing formation and bounds precede lowering. Source profile/selected
   constructor determines the same two supported Core/kind pairs, and every
   common-domain amount/round/identifier is copied exactly without coercion.
3. Lowered intent fields equal the frontend mapping: versions, selected action
   as Core programId, Source/policy claims, keyRef, domain/asset, signer/nonce,
   preHead/validity/caps/floor; transfer recipient/feeRecipient/amount/fee or
   repayment obligationId/amount. Transfer owner and repayment payer are
   bound through signer plus the Source signed action, never erased.
4. Local lowered state copies ordered cells, both work counters and claims;
   agreement/scale/predecessor stay in the retained AST/context and are checked
   at their W-D2F anchors. Dropped Core slots are not evidence of irrelevance.
5. Submitted effects preserve order and values; only UseReplay transforms
   its nonce into the exact composite tuple. Source signedAction/submitted
   action equality is checked by the selected wrapper; bare lowerSource6
   does not establish this equality. Stage-first wrapper diagnostics are
   accounted for separately from the outer adapter's proposed ordering.
6. Source replay `unused/consumed` is only a selected-key claim. The full
   authenticated history enters the consumer through an adopted B16 projection,
   and is never replaced by the local lowerer's synthesized []/[key]. That
   projection/transport is unselected, not supplied by this hash design.
7. Read-only Core derives the complete ordered effects and post cells before
   supplied-vector comparison, including funded repayment/creditor binding,
   gross consumption, checked arithmetic, work/replay/head changes and no
   published post/effects on rejection. Neither an image nor policy chooses
   those effects. Signatures, snapshots, successors and atomic ledger consumption
   remain separately verified B09/B11/B15/B17 obligations.

No lowering proof, runtime evidence or new transport is supplied here. A
future bounded regression suite may demonstrate its specific cases; it must
state its domain and not rename them a full compiler-correctness theorem.

## Diagnostics and compatibility consequences

Use W-D2F's existing proposed families. Literal/computed commitment difference
is `BindingRejected/W_D2F_FIELD_MISMATCH` at sourceHash8, coreHash11 or policyHash12.
Missing adopted definition/artifact/correspondence/provider is
`BindingRejected/W_D2F_BINDING_UNAVAILABLE`, identifying B05/B06/B07 at its
anchor. An implemented verifier finding invalid scoped selection, provenance,
closure or lowering evidence gives `EvidenceRejected/W_D2F_EVIDENCE_INVALID`
at that binding. A valid provider of a later differing field remains a
field mismatch at that field. No adapter diagnosis receives a fabricated
Source byte offset. Every rejection has null published post/effects.
Image-parser codes and artifact/proof transports are not implemented or
silently borrowed from the authorization decoder.

Wire/3 can hold these three 32-byte values without changing field layout.
That fact does not adopt their meaning. Because /3 has no separate image-version
slot, this exact image suite must be selected by an authenticated profile/view;
unknown, ambiguous or mixed suites reject as unavailable/invalid according to
whether their definition/evidence is absent/invalid. Select one authenticated
suite before any image comparison; never try another purpose or suite to find
a matching hash. Purpose2 is only a preimage alternative and is unavailable
to this B-specific consumer path. Concurrent suites need an explicit authenticated version
binding or a separately reviewed wire/profile version; no wire/4 is chosen here.

- Definition image changes to field inclusion, constructor meaning, bounds,
  serialization or domain prefix require a new image/profile version and
  fresh independent vectors. A future Source grammar with real action bodies
  needs a new committed body/lowering relation, not reuse of this projection.
- Exact package byte changes, even comments/line endings or an unused branch,
  change coreHash. Compatible behavior may retain Core/5 only after an explicit
  compatibility/correspondence decision; changed semantics require versioned
  contract review. A hash change alone never certifies compatibility.
- New imports or altered package roles/encoding require a reviewed package-image
  profile. A different execution/toolchain/backend mapping requires fresh
  correspondence and loaded-artifact evidence; it changes coreHash only when
  committed package bytes/labels change, since executable/runtime bytes are
  excluded from purpose3. Same source hash never carries execution evidence
  across that change. No compiler/toolchain change inherits prior correspondence.
- Changing policy field inclusion, interpretation, bounds, failure relation,
  purpose or encoding requires a new policy-image/suite version and independent
  vectors. Ordinary changed policy values under the same schema need fresh
  policyHash/authentication, without automatically changing Source/6 or Core/5.
  An extension beyond closed F-S0-01 additionally needs its own reviewed
  Source/Core/failure relation; purpose4 byte01 cannot authorize that extension.
- Policy terms, settlement/instance selection or package changes require fresh
  policyHash and authentication under the selected view. Definition renaming
  changes sourceHash and policyHash. Presentation changes to the Source proposal
  alone leave B's images unchanged; editing committed package text does not.
- Any changed wire hash changes canonical wire bytes/content digest and needs
  fresh signature evidence under the eventual B09 scheme. Hash migration does
  not reset allowances, spent work, nonce consumption, heads, continuing duties
  or stage/episode identity. No implicit registry rebinding is permitted.
- Historical wire/W-D2E positives remain evidence for their original purposes.
  They have no authenticated Source/Core/policy images and use actionId=Action;
  they cannot be reclassified as full-consumer positives. No fixture is rewritten.

Independent review must adopt the image suite and its compatibility before
implementation. Runtime correspondence, registry authority, B16 transport,
independent expected byte/digest vectors, real authentication and native/ledger
evidence remain open. This proposal makes those requirements concrete without
claiming any gate is closed.

```

