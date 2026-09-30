Independent MIL/4 Source/6 to purpose-1 image adapter audit. Requested GPT-6.1 Sol high and Grok 4.7 xhigh. Review exact embedded bytes; GPT may recompute manifest hashes and independently run the frozen tests. Check actual parseSource6 text admission, all nine projected fields, suite metadata distinction for sourceVersion=6/wireProfile=1, content hash bytes for transfer/repayment, malicious source/presentation invariance, Core-invalid-but-formed limit, 50-test/result record and no arbitrary caller AST path. Check embedded codec/source dependency hashes. Find high/medium defects or bounded acceptance only. No B01/B05 authentication, full consumer, ledger admission or Sprint1 closure. Do not edit packet or source.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/PLAN.md",
    "bytes": 1949,
    "sha256": "f65dfd37b1773c5b2b57f856f9fa13098128611ce535dd9ed372a300743d4a34"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/RESULT.md",
    "bytes": 6956,
    "sha256": "ba628d10dbc7acc19b5c3cc71c25d151e4f109636992151636d60a3bef86c37c"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.d.mts",
    "bytes": 755,
    "sha256": "cad8766e7e23c7d94ecdf3950c606f4f630729f31784cbb11ee8a7c967188f47"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.mjs",
    "bytes": 2095,
    "sha256": "62569c89ad6f91573fe68429935bb0d28fa56de08faa30679a8d396e6c742de2"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.test.mjs",
    "bytes": 9220,
    "sha256": "26d9286a2db51acc98caeceb4bc8b3f89b03746c0cab116898f6b3eb7521a8e6"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/fixtures.mjs",
    "bytes": 3247,
    "sha256": "7d0b29eb92b2093ea7608e21198cd59ca90664438b50c5019c0ee5495e026e8c"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/protected-inputs.json",
    "bytes": 1188,
    "sha256": "9766c0718ac7f3ea0b9b0e9b334d8c7a648ffbb67a05824593047aa3dc9468e2"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/reference-vectors.py",
    "bytes": 1404,
    "sha256": "e39d53490a010f12cd875e2ec0be06323a143fef3ef67361b3d1186242cb9483"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/repay.source.mori",
    "bytes": 1151,
    "sha256": "6db584de67fd6df4ec61ecec0bc7f3be14b20875763903913c20608bc98628ce"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/result-data.json",
    "bytes": 3944,
    "sha256": "dcf5b1162ed06db9f5a6a9a0357c9be366497399b94cb51884eccf9163b960ab"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/run-experiment.mjs",
    "bytes": 2834,
    "sha256": "89e01ef61abd9dff8ceff991bb163ce427d4d7f996249c8871ecd472d2c9fc41"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/run-results.json",
    "bytes": 395,
    "sha256": "c0e781e7c0e0198e21fd981afb079610ac0086a015d0fcf0184eda9501c1962d"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/test-results.tap",
    "bytes": 9791,
    "sha256": "f2e7748f9d32642bfda605504ba831b79c09b4edc628766c304c8392ab07520b"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/transfer.source.mori",
    "bytes": 957,
    "sha256": "80015b54c0e3ef22b5227dd5a2fc553ab78b017822e9fb2af908e9b98f62686f"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/vectors.json",
    "bytes": 1479,
    "sha256": "59d673c6f6743ab56bfcd0391911c530b8cc0e5d3d028357f0352e61421412c1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/source-image-adapter/artifact-manifest.json",
    "bytes": 3068,
    "sha256": "d16505eb5de29622921ad9924a4a212dc5ef75929a63a95e5330e5b191be93d0"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-image-codec/codec.mjs",
    "bytes": 23537,
    "sha256": "a94edacb2f1e81e03bc930d3f7921358e6b3c59b1b8ff1e25532f810874b4d62"
  },
  {
    "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
    "bytes": 20746,
    "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
  }
]
```

## experiments/moriarty-language/formal/mil4/source-image-adapter/PLAN.md

sha256: `f65dfd37b1773c5b2b57f856f9fa13098128611ce535dd9ed372a300743d4a34`

```text
# Source/6 selected-definition image projection

Authorized bounded local implementation; no commit or merge. All writes stay
in this directory. Source/Core, codec, H proposals and existing fixtures are
read-only inputs. The loaded Moriarty develop status reports unrelated SP01
admission gaps and no pending public transaction notifications.

Implement a source-text-only boundary: actual `parseSource6` formation, private
AST projection, closed frozen `SourceDefinition`, then frozen local codec
purpose 1. Export no arbitrary-AST entry point. Callers cannot manufacture AST
fields, override versions, or provide a digest to fill a source claim.

Map `programId` to agreement identity; domain, settlement asset/scale and
selected action directly from the parsed AST; operation kind from its signed
action. Source version 6 and wire profile 1 are explicit local suite constants.
The parser checks the exact Source/6 profile but has no wire profile member.
These constants are serialization metadata, not newly parsed facts.

Acceptance: independently specified Python byte vectors for actual parsed
transfer and repayment fixtures; committed field changes alter images;
presentation, claim, intent, snapshot and effect changes preserve the selected
definition image when formation succeeds; malformed Source produces no image.
Comparisons return content-only/authenticated=false. No B01/B05, provider,
full consumer, policy/Core image, signing, proof or ledger acceptance claim.

Write targeted tests before implementation and preserve their initial failure.
Use one local targeted test run after repair and a runnable hash-pinned results
demonstration; repeat only for an actual failure or added case. Expected Python
serialization is a separate implementation, not independent provider review.
Pin all imported executable inputs and grammar/API in protected-inputs.json;
the runner fails if any input changes. Retain input/output hashes and TAP.

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/RESULT.md

sha256: `ba628d10dbc7acc19b5c3cc71c25d151e4f109636992151636d60a3bef86c37c`

```text
# Source/6 selected-definition image result

Local executable experiment, 2026-09-30. Developers can parse a complete
Source/6 S0 document and produce or compare its purpose-1 selected-definition
image using the local hash-image-codec Proxy repair-01 API. Both
`TransferLiteralFee` and `RepayAccrualFirst` are exercised through the actual
parser. This supplies content projection only. No commit or merge was made.

## Callable boundary and field origin

`adapter.mjs` exports `projectSource6Definition(source)`,
`encodeSource6Definition(source)` and
`compareSource6DefinitionContent(source, claimedDigest)`. Each accepts primitive
source text and calls the real `parseSource6` before privately projecting its
owned AST. There is no public arbitrary-AST input, version override, coerced
object input, caller-owned nested field or claim-fill operation. Parser errors
propagate with their existing code and byte offset. The projected definition
and nominal identity wrappers are frozen. Image byte buffers are ordinary
owned mutable output buffers; callers must retain their exact bytes for use.

| Closed image field | Origin |
| --- | --- |
| `sourceVersion=6` | Explicit `SOURCE_IMAGE_SUITE` metadata; exact Source/6 profile is checked by the parser |
| `profile` | `ast.profile` |
| `wireProfile=1` | Explicit local codec-suite metadata; **absent from Source6Ast and grammar** |
| `agreementInstanceId:agreement` | `ast.programId`, the identifier after `agreement`; this is not the selected builtin |
| `domain:domain` | `ast.domain` |
| `asset:asset` | `ast.settlement.asset` |
| `scale` | `ast.settlement.scale`, canonical decimal text |
| `selectedActionId:action` | `ast.selected.actionId` |
| `operationKind` | `ast.intent.signedAction.kind`, not the submitted action |

Nominal wrappers assign the destination record's explicit identifier roles;
they do not authenticate those identities. The parser enforces the supported
selector/kind pairs. No required purpose-1 content field is missing from this
parser. The version and wire-profile distinction above is the exact API gap:
neither is a separate AST member, and wire profile is not declared in source.
The adapter does not pretend either constant was such a parsed member.

## Executed observations

Node `v24.21.0`: **50 tests passed, 0 failed, 0 skipped** with
`node --test --test-reporter=tap adapter.test.mjs`. Complete output is retained
in `test-results.tap`. The initial pre-implementation run retains 0/50 passing
in `initial-test-results.tap`. The first implementation run retained 49/50:
one test expected `NON_CANONICAL_INTEGER` where the existing parser actually
returns `INVALID_INTEGER`. `expected-code-failure.tap` preserves that run; only
the test expectation changed. No parser error code or formation rule changed.

`reference-vectors.py` specifies the expected fields directly and uses Python
`struct`, integer bytes and `hashlib`, without importing the JS parser, adapter
or codec. `vectors.json` freezes complete expected payload, envelope and digest
for one transfer and one repayment. This is a separate serialization
implementation by the same author, not an independent person/provider audit.
Both actual parsed fixtures match every expected byte and the existing codec.

| Parsed fixture | Payload bytes | Purpose-1 SHA256 |
| --- | --- | --- |
| Transfer | 81 | `01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a` |
| Repay | 80 | `c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d` |

Envelope overhead is 27 bytes. `run-experiment.mjs` exited 0, verified all six
frozen read-only inputs before importing executable modules, and retained
complete projected records, bytes and content comparisons in `result-data.json`.
It also saved the actual parsed texts as `transfer.source.mori` and
`repay.source.mori`, with their SHA256 values in that result. `run-results.json`
retains the command summary. `protected-inputs.json` pins the codec/API,
parser, Core module, Source preparation wrapper and grammar. The wrapper is
used only for test boundary controls; the adapter itself never calls it.

Positive controls include formed and locally prepared transfer, repayment,
zero-fee transfer and full repayment. Changing agreement identity, domain,
asset or scale changes the image for both kinds; changing between the two
valid selector/kind pairs changes it. The projected record has exactly the
nine codec fields, with distinct agreement/domain/asset/action nominal sorts.

Presentation-invariance controls cover whitespace, CRLF, leading/trailing and
block comments, a decoded claim-string escape, source and policy claim changes,
intent key/nonce changes, authenticated round/work changes and submitted-effect
amount changes. All compared sources successfully form. Neither digest claim
is filled, authenticated, validated as a hash or copied into the image.

Hostile controls reject old or escaped profiles, unsupported/mismatched
selectors, duplicate/extra fields, a source wire-profile field, noncanonical
integers, out-of-range scale, non-ASCII identifiers, mismatched authenticated
cells, inconsistent repayment totals, trailing source, unterminated comments,
source-size excess and lone surrogates. Caller AST objects, boxed strings,
Proxy objects and coercion objects reject without property reads. Formation
rejection precedes malformed content comparison claims.

## Limits and reproduction

Purpose 1 intentionally excludes intent, state and submitted effects. A source
with an incorrect submitted debit can successfully form and receive the same
definition image while `prepareSource6S0Unqualified` returns `CoreRejected`;
the test suite exercises that boundary. Parsed `authenticated` syntax supplies
claims and is not evidence of external authentication. Image equality is not
complete source/policy equality or a statement about valid execution.

Content comparisons retain `scope:"content-only"` and `authenticated:false`.
The experiment does not implement or close B01/B05, selected-suite/provider
authentication, policy/Core image derivation, a full consumer, signatures,
compilation correspondence, native proof, PCD or ledger acceptance. No public
transaction was submitted. Source/Core/codec/H proposal documents and modules
were not edited. Independent current executable reviews remain pending; the
codec's existing repair-01 review status is not advanced here. The declaration
file has not received a separate TypeScript compile check.

From this directory:

```bash
node --test --test-reporter=tap adapter.test.mjs
node run-experiment.mjs
```

`python3 reference-vectors.py` regenerates the standalone expected vectors
from its literal expectations. Preserve `artifact-manifest.json` before
regeneration or edits. That manifest hashes all delivered files except itself
and records the measured test scope; input drift makes the runner fail.
All writes and retained failures are confined to `source-image-adapter/`.

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.d.mts

sha256: `cad8766e7e23c7d94ecdf3950c606f4f630729f31784cbb11ee8a7c967188f47`

```text
/// <reference types="node" />
import type {SourceDefinition, Image, ContentComparison} from '../hash-image-codec/codec.mjs';
export const SOURCE_IMAGE_SUITE: Readonly<{sourceVersion: 6; wireProfile: 1;
  profile: 'moriarty-financial-agreement-source/6'; purpose: 1}>;
export class SourceImageAdapterError extends Error {
  code: string;
  scope: 'local-source-image-projection';
  constructor(code: string, message: string);
}
/** Forms Source/6 itself; never accepts an unverified caller AST. */
export function projectSource6Definition(source: string): Readonly<SourceDefinition>;
export function encodeSource6Definition(source: string): Image;
export function compareSource6DefinitionContent(source: string, claimedDigest: string): ContentComparison;

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.mjs

sha256: `62569c89ad6f91573fe68429935bb0d28fa56de08faa30679a8d396e6c742de2`

```text
/** Selected-definition content projection only; formation is not authentication. */
import { parseSource6 } from '../../../src/successor/financial-agreement-source-v6-frontend.ts';
import { SOURCE_PROFILE, id, encodeImage, compareContent } from '../hash-image-codec/codec.mjs';

/** Local serialization-suite metadata; wireProfile is absent from Source6Ast. */
export const SOURCE_IMAGE_SUITE = Object.freeze({sourceVersion: 6, wireProfile: 1,
  profile: SOURCE_PROFILE, purpose: 1});

export class SourceImageAdapterError extends Error {
  constructor(code, message) {
    super(message);
    this.name = 'SourceImageAdapterError';
    this.code = code;
    this.scope = 'local-source-image-projection';
  }
}

/** Parse the entire Source/6 document, then privately project its owned AST.
 * No externally supplied AST, coercion, version override or claim fill is used.
 * Parser formation errors are propagated unchanged, including byte offset.
 */
export function projectSource6Definition(source) {
  if (typeof source !== 'string')
    throw new SourceImageAdapterError('SOURCE_TEXT_REQUIRED', 'Primitive Source/6 text required');
  const ast = parseSource6(source);
  return Object.freeze({
    sourceVersion: SOURCE_IMAGE_SUITE.sourceVersion,
    profile: ast.profile,
    wireProfile: SOURCE_IMAGE_SUITE.wireProfile,
    agreementInstanceId: id('agreement', ast.programId),
    domain: id('domain', ast.domain),
    asset: id('asset', ast.settlement.asset),
    scale: ast.settlement.scale,
    selectedActionId: id('action', ast.selected.actionId),
    operationKind: ast.intent.signedAction.kind,
  });
}

/** Purpose 1 only. Does not encode or inspect embedded source/policy claims. */
export function encodeSource6Definition(source) {
  return encodeImage(SOURCE_IMAGE_SUITE.purpose, projectSource6Definition(source));
}

/** Local content comparison; no registry/provider/signature authority supplied. */
export function compareSource6DefinitionContent(source, claimedDigest) {
  return compareContent(SOURCE_IMAGE_SUITE.purpose, projectSource6Definition(source), claimedDigest);
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/adapter.test.mjs

sha256: `26d9286a2db51acc98caeceb4bc8b3f89b03746c0cab116898f6b3eb7521a8e6`

```text
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { parseSource6, Source6Error } from '../../../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../../../src/successor/mil4-s0-source-v6.ts';
import { encodeImage, ImageCodecError } from '../hash-image-codec/codec.mjs';
import { transfer, repay } from './fixtures.mjs';

const api = await import('./adapter.mjs').catch(() => ({}));
const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url), 'utf8'));
const expectedDefinition = (kind) => ({ sourceVersion: 6,
  profile: 'moriarty-financial-agreement-source/6', wireProfile: 1,
  agreementInstanceId: {sort: 'agreement', value: 'AgreementA'},
  domain: {sort: 'domain', value: 'D'}, asset: {sort: 'asset', value: 'A'}, scale: '0',
  selectedActionId: {sort: 'action', value: kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst'},
  operationKind: kind });

test('source-text-only API exists without arbitrary AST projection entry', () => {
  assert.equal(typeof api.projectSource6Definition, 'function');
  assert.equal(typeof api.encodeSource6Definition, 'function');
  assert.equal(typeof api.compareSource6DefinitionContent, 'function');
  assert.deepEqual(Object.keys(api).sort(), ['SOURCE_IMAGE_SUITE', 'SourceImageAdapterError',
    'compareSource6DefinitionContent', 'encodeSource6Definition', 'projectSource6Definition'].sort());
});
for (const [kind, fixture] of [['Transfer', transfer], ['Repay', repay]]) {
  test(`${kind}: real parser and private AST projection retain exact closed fields`, () => {
    const source = fixture();
    const ast = parseSource6(source);
    assert.equal(ast.intent.signedAction.kind, kind);
    assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
    const definition = api.projectSource6Definition(source);
    assert.deepEqual(definition, expectedDefinition(kind));
    assert.equal(definition.agreementInstanceId.value, ast.programId);
    assert.equal(definition.selectedActionId.value, ast.selected.actionId);
    assert.equal(definition.operationKind, ast.intent.signedAction.kind);
    assert.equal(Object.isFrozen(definition), true);
    for (const name of ['agreementInstanceId', 'domain', 'asset', 'selectedActionId'])
      assert.equal(Object.isFrozen(definition[name]), true);
    assert.throws(() => { definition.asset.value = 'Forged'; }, TypeError);
  });
  test(`${kind}: independent complete expected payload, preimage and digest`, () => {
    const vector = vectors.find(v => v.expectedFields.operationKind === kind);
    const image = api.encodeSource6Definition(fixture());
    assert.equal(image.purpose, 1);
    assert.equal(image.payload.toString('hex'), vector.payloadHex);
    assert.equal(image.preimage.toString('hex'), vector.preimageHex);
    assert.equal(image.digest, vector.sha256);
    assert.deepEqual(image, encodeImage(1, expectedDefinition(kind)));
  });
  for (const [field, value] of [['agreement', 'OtherAgreement'], ['domain', 'OtherDomain'],
    ['asset', 'OtherAsset'], ['scale', '18']]) {
    test(`${kind}: ${field} is committed`, () => {
      assert.notEqual(api.encodeSource6Definition(fixture({[field]: value})).digest,
        api.encodeSource6Definition(fixture()).digest);
    });
  }
  for (const [name, changed] of [
    ['comments and whitespace', s => '// leading\n' + s.replaceAll(';', '; /* inert */\r\n') + '\n// trailing'],
    ['source claim', s => s.replace('claim-source', 'unverified-changed-source-claim')],
    ['policy claim', s => s.replace('claim-policy', 'unverified-changed-policy-claim')],
    ['decoded claim escape', s => s.replace('claim-source', 'claim-\\u0073ource')],
    ['intent key and nonce', s => s.replace('key1', 'other-key').replaceAll('n1', 'n2')],
    ['authenticated counters and round', s => s.replace('round 1;', 'round 2;').replace('work_remaining 10;', 'work_remaining 9;')],
    ['submitted effects', s => s.replace('debit Owner 11;', 'debit Owner 12;').replace('debit Payer 30;', 'debit Payer 31;')],
  ]) {
    test(`${kind}: ${name} is excluded after successful formation`, () => {
      const original = fixture(), changedSource = changed(original);
      assert.notEqual(changedSource, original);
      parseSource6(changedSource);
      assert.deepEqual(api.encodeSource6Definition(changedSource), api.encodeSource6Definition(original));
    });
  }
}

test('suite metadata explicitly identifies version and wire-profile constants', () => {
  assert.deepEqual(api.SOURCE_IMAGE_SUITE, {sourceVersion: 6, wireProfile: 1,
    profile: 'moriarty-financial-agreement-source/6', purpose: 1});
  assert.equal(Object.isFrozen(api.SOURCE_IMAGE_SUITE), true);
  assert.equal(Object.hasOwn(parseSource6(transfer()), 'wireProfile'), false);
});
test('selector and operation kind change together across valid builtins', () => {
  assert.notEqual(api.encodeSource6Definition(transfer()).digest, api.encodeSource6Definition(repay()).digest);
});
test('zero-fee transfer is formed and prepared with the same selected-definition image', () => {
  const source = transfer({fee: '0'});
  assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(transfer()));
});
test('full repayment is formed and prepared with the same selected-definition image', () => {
  const source = repay({amount: '1010', postPrincipal: '0', postOutstanding: '0', postStatus: 'settled'});
  assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(repay()));
});
test('stage/effect rejection is outside successful source formation', () => {
  const source = transfer().replace('debit Owner 11;', 'debit Owner 12;');
  assert.equal(prepareSource6S0Unqualified(source).status, 'CoreRejected');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(transfer()));
});

for (const [name, change, code] of [
  ['old version', s => s.replace('source/6', 'source/5'), 'SOURCE6_VERSION'],
  ['escaped profile', s => s.replace('source/6', 'source/\\u0036'), 'SOURCE6_VERSION'],
  ['unknown selector', s => s.replace('TransferLiteralFee', 'Arbitrary'), 'SOURCE6_PROFILE_UNSUPPORTED'],
  ['wrong supported selector', s => s.replace('TransferLiteralFee', 'RepayAccrualFirst'), 'SOURCE6_PROFILE_UNSUPPORTED'],
  ['duplicate field', s => s.replace('domain D;', 'domain D; domain Other;'), 'SOURCE6_SHAPE'],
  ['extra wire profile field', s => s.replace('domain D;', 'domain D; wire_profile 1;'), 'SOURCE6_SHAPE'],
  ['noncanonical integer', s => s.replace('scale 0;', 'scale 00;'), 'INVALID_INTEGER'],
  ['scale bound', s => s.replace('scale 0;', 'scale 19;'), 'SOURCE6_RANGE'],
  ['non-ASCII identifier', s => s.replace('AgreementA', 'AgreementÁ'), 'NON_ASCII_IDENTIFIER'],
  ['wrong authenticated owner', s => s.replace('balance Owner', 'balance Other'), 'SOURCE6_CELL_SHAPE'],
  ['trailing source', s => s + ' extra', 'SOURCE6_SHAPE'],
  ['unterminated comment', s => s + ' /*', 'UNTERMINATED_COMMENT'],
  ['source size bound', s => s + ' '.repeat(65537), 'SOURCE_BOUND'],
  ['lone surrogate', s => s + '\ud800', 'INVALID_SURROGATE'],
]) {
  test(`malformed ${name} rejects before projection and encoding`, () => {
    for (const fn of [api.projectSource6Definition, api.encodeSource6Definition])
      assert.throws(() => fn(change(transfer())), e => e instanceof Source6Error && e.code === code);
  });
}
test('repayment inconsistent obligation rejects formation', () => {
  assert.throws(() => api.encodeSource6Definition(repay({outstanding: '1009'})),
    e => e instanceof Source6Error && e.code === 'SOURCE6_RANGE');
});
test('caller objects and Proxy/coercion inputs are not parsed AST authority', () => {
  let gets = 0;
  const proxy = new Proxy(parseSource6(transfer()), {get() { gets++; throw Error('must not read'); }});
  for (const input of [proxy, parseSource6(transfer()), new String(transfer()), null, 6, undefined,
    {[Symbol.toPrimitive]() { gets++; return transfer(); }}]) {
    assert.throws(() => api.encodeSource6Definition(input),
      e => e instanceof api.SourceImageAdapterError && e.code === 'SOURCE_TEXT_REQUIRED');
  }
  assert.equal(gets, 0);
});
test('content comparison never authenticates or fills embedded claims', () => {
  const source = transfer(), digest = api.encodeSource6Definition(source).digest;
  const comparison = api.compareSource6DefinitionContent(source, digest);
  assert.deepEqual(comparison, {matches: true, digest, claimedDigest: digest,
    scope: 'content-only', authenticated: false});
  assert.equal(api.compareSource6DefinitionContent(source, '0'.repeat(64)).matches, false);
  assert.equal(parseSource6(source).selected.sourceHash, 'claim-source');
  assert.throws(() => api.compareSource6DefinitionContent(source, 'not-a-hash'), ImageCodecError);
});
test('malformed Source takes precedence over malformed comparison claim', () => {
  assert.throws(() => api.compareSource6DefinitionContent('garbage', 'not-a-hash'), Source6Error);
});

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/fixtures.mjs

sha256: `7d0b29eb92b2093ea7608e21198cd59ca90664438b50c5019c0ee5495e026e8c`

```text
/** Local source fixtures. Claims and authenticated blocks are presentation only. */
export function transfer(overrides = {}) {
  const p = { agreement: 'AgreementA', domain: 'D', asset: 'A', scale: '0',
    sourceHash: 'claim-source', policyDigest: 'claim-policy', key: 'key1',
    value: '10', fee: '1', balance: '100', postHead: 'h1', ...overrides };
  const gross = (BigInt(p.value) + BigInt(p.fee)).toString();
  return `profile "moriarty-financial-agreement-source/6";
agreement ${p.agreement} {
 domain ${p.domain}; settlement ${p.asset} scale ${p.scale};
 selected TransferLiteralFee source_hash "${p.sourceHash}" digest "${p.policyDigest}";
 intent {
  signer Owner key "${p.key}"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap ${gross}; fee_cap ${p.fee}; net_floor ${p.value}; failure success_only;
  signed_action transfer from Owner to Recipient fee_to Fee value ${p.value} fee ${p.fee};
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Owner ${p.balance}; balance Recipient 0; balance Fee 0;
  allowance Owner remaining 100 spent 0; replay unused; work_remaining 10; work_spent 0;
 }
 submit transfer from Owner to Recipient fee_to Fee value ${p.value} fee ${p.fee};
 effects { debit Owner ${gross}; credit Recipient ${p.value};
  ${p.fee === '0' ? '' : `credit Fee ${p.fee};`}
  use_allowance Owner ${gross}; use_replay "n1"; advance_head "h0" "${p.postHead}";
 }
 post_head "${p.postHead}";
}`;
}

export function repay(overrides = {}) {
  const p = { agreement: 'AgreementA', domain: 'D', asset: 'A', scale: '0',
    sourceHash: 'claim-source', policyDigest: 'claim-policy', amount: '30',
    principal: '1000', accrued: '10', outstanding: '1010', postPrincipal: '980',
    postOutstanding: '980', postStatus: 'outstanding', ...overrides };
  return `profile "moriarty-financial-agreement-source/6";
agreement ${p.agreement} {
 domain ${p.domain}; settlement ${p.asset} scale ${p.scale};
 selected RepayAccrualFirst source_hash "${p.sourceHash}" digest "${p.policyDigest}";
 intent {
  signer Payer key "key1"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap ${p.amount}; fee_cap 0; net_floor 0; failure success_only;
  signed_action repay obligation Loan payer Payer amount ${p.amount} conversion identity;
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Payer 2000; balance Creditor 0;
  allowance Payer remaining 2000 spent 0;
  obligation Loan { debtor Payer; creditor Creditor; asset ${p.asset};
   principal ${p.principal}; accrued ${p.accrued}; outstanding ${p.outstanding}; status outstanding; }
  replay unused; work_remaining 10; work_spent 0;
 }
 submit repay obligation Loan payer Payer amount ${p.amount} conversion identity;
 effects { debit Payer ${p.amount}; credit Creditor ${p.amount};
  set_obligation Loan principal ${p.postPrincipal} accrued 0 outstanding ${p.postOutstanding} status ${p.postStatus};
  use_allowance Payer ${p.amount}; use_replay "n1"; advance_head "h0" "h1";
 }
 post_head "h1";
}`;
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/protected-inputs.json

sha256: `9766c0718ac7f3ea0b9b0e9b334d8c7a648ffbb67a05824593047aa3dc9468e2`

```text
{
  "scope": "read-only local proxy-repair-01 API and Source/6 formation inputs",
  "inputs": [
    {
      "path": "../hash-image-codec/codec.mjs",
      "bytes": 23537,
      "sha256": "a94edacb2f1e81e03bc930d3f7921358e6b3c59b1b8ff1e25532f810874b4d62"
    },
    {
      "path": "../hash-image-codec/codec.d.mts",
      "bytes": 3397,
      "sha256": "5159460be78fd8c1c122bb1b5e02e63b2ec53c8787214f73021f943bd757595a"
    },
    {
      "path": "../../../src/successor/financial-agreement-source-v6-frontend.ts",
      "bytes": 20746,
      "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
    },
    {
      "path": "../../../src/successor/mil4-s0-core-v5.ts",
      "bytes": 17917,
      "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
    },
    {
      "path": "../../../src/successor/mil4-s0-source-v6.ts",
      "bytes": 2640,
      "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
    },
    {
      "path": "../../../spec/successor/financial-agreement-source-v6-grammar.ebnf",
      "bytes": 3753,
      "sha256": "5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/reference-vectors.py

sha256: `e39d53490a010f12cd875e2ec0be06323a143fef3ef67361b3d1186242cb9483`

```text
"""Purpose-1 expected bytes; imports neither JS parser, adapter nor codec.

Inputs are declared expectations for the local fixtures, not extracted from
producer output. This separate serializer is not a separate-person audit.
"""
import hashlib
import json
from pathlib import Path
import struct

BASE = Path(__file__).resolve().parent
PROFILE = "moriarty-financial-agreement-source/6"

def sized(text):
    raw = text.encode("ascii")
    return struct.pack(">H", len(raw)) + raw

vectors = []
for kind, selector, tag in [("Transfer", "TransferLiteralFee", 1),
                            ("Repay", "RepayAccrualFirst", 2)]:
    fields = {"agreement": "AgreementA", "domain": "D", "asset": "A",
              "scale": "0", "selectedAction": selector, "operationKind": kind}
    payload = (bytes([6]) + sized(PROFILE) + bytes([1])
               + sized(fields["agreement"]) + sized(fields["domain"])
               + sized(fields["asset"]) + bytes([0]) + sized(selector) + bytes([tag]))
    preimage = (b"moriarty-mil4-image/1\x00" + bytes([1])
                + struct.pack(">I", len(payload)) + payload)
    vectors.append({"expectedFields": fields, "payloadHex": payload.hex(),
                    "preimageHex": preimage.hex(), "payloadBytes": len(payload),
                    "sha256": hashlib.sha256(preimage).hexdigest()})
(BASE / "vectors.json").write_text(json.dumps(vectors, indent=2) + "\n")

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/repay.source.mori

sha256: `6db584de67fd6df4ec61ecec0bc7f3be14b20875763903913c20608bc98628ce`

```text
profile "moriarty-financial-agreement-source/6";
agreement AgreementA {
 domain D; settlement A scale 0;
 selected RepayAccrualFirst source_hash "claim-source" digest "claim-policy";
 intent {
  signer Payer key "key1"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap 30; fee_cap 0; net_floor 0; failure success_only;
  signed_action repay obligation Loan payer Payer amount 30 conversion identity;
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Payer 2000; balance Creditor 0;
  allowance Payer remaining 2000 spent 0;
  obligation Loan { debtor Payer; creditor Creditor; asset A;
   principal 1000; accrued 10; outstanding 1010; status outstanding; }
  replay unused; work_remaining 10; work_spent 0;
 }
 submit repay obligation Loan payer Payer amount 30 conversion identity;
 effects { debit Payer 30; credit Creditor 30;
  set_obligation Loan principal 980 accrued 0 outstanding 980 status outstanding;
  use_allowance Payer 30; use_replay "n1"; advance_head "h0" "h1";
 }
 post_head "h1";
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/result-data.json

sha256: `dcf5b1162ed06db9f5a6a9a0357c9be366497399b94cb51884eccf9163b960ab`

```text
{
  "status": "local-source-image-experiment-not-adopted",
  "codecRepair": "proxy-repair-01",
  "suite": {
    "sourceVersion": 6,
    "wireProfile": 1,
    "profile": "moriarty-financial-agreement-source/6",
    "purpose": 1
  },
  "claim": "parsed-selected-definition-content-only",
  "authenticated": false,
  "protectedInputsManifestSha256": "9766c0718ac7f3ea0b9b0e9b334d8c7a648ffbb67a05824593047aa3dc9468e2",
  "verifiedReadOnlyInputs": 6,
  "results": [
    {
      "kind": "Transfer",
      "sourceFile": "transfer.source.mori",
      "sourceTextSha256": "80015b54c0e3ef22b5227dd5a2fc553ab78b017822e9fb2af908e9b98f62686f",
      "definition": {
        "sourceVersion": 6,
        "profile": "moriarty-financial-agreement-source/6",
        "wireProfile": 1,
        "agreementInstanceId": {
          "sort": "agreement",
          "value": "AgreementA"
        },
        "domain": {
          "sort": "domain",
          "value": "D"
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
      "purpose": 1,
      "payloadBytes": 81,
      "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74410001440001410000125472616e736665724c69746572616c46656501",
      "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000510600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74410001440001410000125472616e736665724c69746572616c46656501",
      "digest": "01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a",
      "comparison": {
        "matches": true,
        "digest": "01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a",
        "claimedDigest": "01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a",
        "scope": "content-only",
        "authenticated": false
      },
      "unchangedAfterClaimMutation": true,
      "embeddedSourceClaim": "claim-source",
      "embeddedPolicyClaim": "claim-policy"
    },
    {
      "kind": "Repay",
      "sourceFile": "repay.source.mori",
      "sourceTextSha256": "6db584de67fd6df4ec61ecec0bc7f3be14b20875763903913c20608bc98628ce",
      "definition": {
        "sourceVersion": 6,
        "profile": "moriarty-financial-agreement-source/6",
        "wireProfile": 1,
        "agreementInstanceId": {
          "sort": "agreement",
          "value": "AgreementA"
        },
        "domain": {
          "sort": "domain",
          "value": "D"
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
      "purpose": 1,
      "payloadBytes": 80,
      "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e744100014400014100001152657061794163637275616c466972737402",
      "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000500600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e744100014400014100001152657061794163637275616c466972737402",
      "digest": "c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d",
      "comparison": {
        "matches": true,
        "digest": "c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d",
        "claimedDigest": "c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d",
        "scope": "content-only",
        "authenticated": false
      },
      "unchangedAfterClaimMutation": true,
      "embeddedSourceClaim": "claim-source",
      "embeddedPolicyClaim": "claim-policy"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/run-experiment.mjs

sha256: `89e01ef61abd9dff8ceff991bb163ce427d4d7f996249c8871ecd472d2c9fc41`

```text
/** Bounded Source/6 formation → purpose-1 image demonstration; content only. */
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const base = new URL('./', import.meta.url);
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const protectedInputs = JSON.parse(readFileSync(new URL('protected-inputs.json', base), 'utf8'));
// Verify read-only dependencies before importing and executing those bytes.
for (const input of protectedInputs.inputs) {
  const bytes = readFileSync(new URL(input.path, base));
  if (bytes.length !== input.bytes || sha256(bytes) !== input.sha256)
    throw Error(`Frozen read-only input changed: ${input.path}`);
}
const {SOURCE_IMAGE_SUITE, projectSource6Definition, encodeSource6Definition,
  compareSource6DefinitionContent} = await import('./adapter.mjs');
const {transfer, repay} = await import('./fixtures.mjs');
const vectors = JSON.parse(readFileSync(new URL('vectors.json', base), 'utf8'));
const results = [];
for (const [kind, fixture] of [['Transfer', transfer], ['Repay', repay]]) {
  const source = fixture();
  const vector = vectors.find(v => v.expectedFields.operationKind === kind);
  const image = encodeSource6Definition(source);
  if (image.payload.toString('hex') !== vector.payloadHex
    || image.preimage.toString('hex') !== vector.preimageHex || image.digest !== vector.sha256)
    throw Error(`Independent expected bytes mismatch: ${kind}`);
  const file = `${kind.toLowerCase()}.source.mori`;
  writeFileSync(new URL(file, base), source + '\n');
  results.push({kind, sourceFile: file, sourceTextSha256: sha256(Buffer.from(source + '\n')),
    definition: projectSource6Definition(source), purpose: image.purpose,
    payloadBytes: image.payload.length, payloadHex: image.payload.toString('hex'),
    preimageHex: image.preimage.toString('hex'), digest: image.digest,
    comparison: compareSource6DefinitionContent(source, vector.sha256),
    unchangedAfterClaimMutation: encodeSource6Definition(fixture({sourceHash: 'other-source-claim'})).digest === image.digest,
    embeddedSourceClaim: 'claim-source', embeddedPolicyClaim: 'claim-policy'});
}
const record = {status: 'local-source-image-experiment-not-adopted',
  codecRepair: 'proxy-repair-01', suite: SOURCE_IMAGE_SUITE,
  claim: 'parsed-selected-definition-content-only', authenticated: false,
  protectedInputsManifestSha256: sha256(readFileSync(new URL('protected-inputs.json', base))),
  verifiedReadOnlyInputs: protectedInputs.inputs.length, results};
writeFileSync(new URL('result-data.json', base), JSON.stringify(record, null, 2) + '\n');
console.log(JSON.stringify({status: record.status, inputsVerified: record.verifiedReadOnlyInputs,
  outputs: results.map(({kind, digest, payloadBytes}) => ({kind, digest, payloadBytes}))}, null, 2));

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/run-results.json

sha256: `c0e781e7c0e0198e21fd981afb079610ac0086a015d0fcf0184eda9501c1962d`

```text
{
  "status": "local-source-image-experiment-not-adopted",
  "inputsVerified": 6,
  "outputs": [
    {
      "kind": "Transfer",
      "digest": "01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a",
      "payloadBytes": 81
    },
    {
      "kind": "Repay",
      "digest": "c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d",
      "payloadBytes": 80
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/test-results.tap

sha256: `f2e7748f9d32642bfda605504ba831b79c09b4edc628766c304c8392ab07520b`

```text
TAP version 13
# Subtest: source-text-only API exists without arbitrary AST projection entry
ok 1 - source-text-only API exists without arbitrary AST projection entry
  ---
  duration_ms: 1.665138
  type: 'test'
  ...
# Subtest: Transfer: real parser and private AST projection retain exact closed fields
ok 2 - Transfer: real parser and private AST projection retain exact closed fields
  ---
  duration_ms: 11.871391
  type: 'test'
  ...
# Subtest: Transfer: independent complete expected payload, preimage and digest
ok 3 - Transfer: independent complete expected payload, preimage and digest
  ---
  duration_ms: 3.613369
  type: 'test'
  ...
# Subtest: Transfer: agreement is committed
ok 4 - Transfer: agreement is committed
  ---
  duration_ms: 2.254293
  type: 'test'
  ...
# Subtest: Transfer: domain is committed
ok 5 - Transfer: domain is committed
  ---
  duration_ms: 1.01771
  type: 'test'
  ...
# Subtest: Transfer: asset is committed
ok 6 - Transfer: asset is committed
  ---
  duration_ms: 0.840376
  type: 'test'
  ...
# Subtest: Transfer: scale is committed
ok 7 - Transfer: scale is committed
  ---
  duration_ms: 0.837057
  type: 'test'
  ...
# Subtest: Transfer: comments and whitespace is excluded after successful formation
ok 8 - Transfer: comments and whitespace is excluded after successful formation
  ---
  duration_ms: 4.258115
  type: 'test'
  ...
# Subtest: Transfer: source claim is excluded after successful formation
ok 9 - Transfer: source claim is excluded after successful formation
  ---
  duration_ms: 1.107048
  type: 'test'
  ...
# Subtest: Transfer: policy claim is excluded after successful formation
ok 10 - Transfer: policy claim is excluded after successful formation
  ---
  duration_ms: 0.779428
  type: 'test'
  ...
# Subtest: Transfer: decoded claim escape is excluded after successful formation
ok 11 - Transfer: decoded claim escape is excluded after successful formation
  ---
  duration_ms: 1.258976
  type: 'test'
  ...
# Subtest: Transfer: intent key and nonce is excluded after successful formation
ok 12 - Transfer: intent key and nonce is excluded after successful formation
  ---
  duration_ms: 3.153487
  type: 'test'
  ...
# Subtest: Transfer: authenticated counters and round is excluded after successful formation
ok 13 - Transfer: authenticated counters and round is excluded after successful formation
  ---
  duration_ms: 0.618549
  type: 'test'
  ...
# Subtest: Transfer: submitted effects is excluded after successful formation
ok 14 - Transfer: submitted effects is excluded after successful formation
  ---
  duration_ms: 1.711314
  type: 'test'
  ...
# Subtest: Repay: real parser and private AST projection retain exact closed fields
ok 15 - Repay: real parser and private AST projection retain exact closed fields
  ---
  duration_ms: 2.110536
  type: 'test'
  ...
# Subtest: Repay: independent complete expected payload, preimage and digest
ok 16 - Repay: independent complete expected payload, preimage and digest
  ---
  duration_ms: 0.622492
  type: 'test'
  ...
# Subtest: Repay: agreement is committed
ok 17 - Repay: agreement is committed
  ---
  duration_ms: 0.478498
  type: 'test'
  ...
# Subtest: Repay: domain is committed
ok 18 - Repay: domain is committed
  ---
  duration_ms: 3.678852
  type: 'test'
  ...
# Subtest: Repay: asset is committed
ok 19 - Repay: asset is committed
  ---
  duration_ms: 1.100891
  type: 'test'
  ...
# Subtest: Repay: scale is committed
ok 20 - Repay: scale is committed
  ---
  duration_ms: 0.866006
  type: 'test'
  ...
# Subtest: Repay: comments and whitespace is excluded after successful formation
ok 21 - Repay: comments and whitespace is excluded after successful formation
  ---
  duration_ms: 1.964968
  type: 'test'
  ...
# Subtest: Repay: source claim is excluded after successful formation
ok 22 - Repay: source claim is excluded after successful formation
  ---
  duration_ms: 2.580244
  type: 'test'
  ...
# Subtest: Repay: policy claim is excluded after successful formation
ok 23 - Repay: policy claim is excluded after successful formation
  ---
  duration_ms: 0.721585
  type: 'test'
  ...
# Subtest: Repay: decoded claim escape is excluded after successful formation
ok 24 - Repay: decoded claim escape is excluded after successful formation
  ---
  duration_ms: 1.198431
  type: 'test'
  ...
# Subtest: Repay: intent key and nonce is excluded after successful formation
ok 25 - Repay: intent key and nonce is excluded after successful formation
  ---
  duration_ms: 1.995857
  type: 'test'
  ...
# Subtest: Repay: authenticated counters and round is excluded after successful formation
ok 26 - Repay: authenticated counters and round is excluded after successful formation
  ---
  duration_ms: 2.527683
  type: 'test'
  ...
# Subtest: Repay: submitted effects is excluded after successful formation
ok 27 - Repay: submitted effects is excluded after successful formation
  ---
  duration_ms: 0.734187
  type: 'test'
  ...
# Subtest: suite metadata explicitly identifies version and wire-profile constants
ok 28 - suite metadata explicitly identifies version and wire-profile constants
  ---
  duration_ms: 0.874295
  type: 'test'
  ...
# Subtest: selector and operation kind change together across valid builtins
ok 29 - selector and operation kind change together across valid builtins
  ---
  duration_ms: 1.064785
  type: 'test'
  ...
# Subtest: zero-fee transfer is formed and prepared with the same selected-definition image
ok 30 - zero-fee transfer is formed and prepared with the same selected-definition image
  ---
  duration_ms: 0.716202
  type: 'test'
  ...
# Subtest: full repayment is formed and prepared with the same selected-definition image
ok 31 - full repayment is formed and prepared with the same selected-definition image
  ---
  duration_ms: 0.874218
  type: 'test'
  ...
# Subtest: stage/effect rejection is outside successful source formation
ok 32 - stage/effect rejection is outside successful source formation
  ---
  duration_ms: 1.376082
  type: 'test'
  ...
# Subtest: malformed old version rejects before projection and encoding
ok 33 - malformed old version rejects before projection and encoding
  ---
  duration_ms: 0.485613
  type: 'test'
  ...
# Subtest: malformed escaped profile rejects before projection and encoding
ok 34 - malformed escaped profile rejects before projection and encoding
  ---
  duration_ms: 0.603701
  type: 'test'
  ...
# Subtest: malformed unknown selector rejects before projection and encoding
ok 35 - malformed unknown selector rejects before projection and encoding
  ---
  duration_ms: 1.066161
  type: 'test'
  ...
# Subtest: malformed wrong supported selector rejects before projection and encoding
ok 36 - malformed wrong supported selector rejects before projection and encoding
  ---
  duration_ms: 0.958177
  type: 'test'
  ...
# Subtest: malformed duplicate field rejects before projection and encoding
ok 37 - malformed duplicate field rejects before projection and encoding
  ---
  duration_ms: 1.636652
  type: 'test'
  ...
# Subtest: malformed extra wire profile field rejects before projection and encoding
ok 38 - malformed extra wire profile field rejects before projection and encoding
  ---
  duration_ms: 0.647665
  type: 'test'
  ...
# Subtest: malformed noncanonical integer rejects before projection and encoding
ok 39 - malformed noncanonical integer rejects before projection and encoding
  ---
  duration_ms: 0.669448
  type: 'test'
  ...
# Subtest: malformed scale bound rejects before projection and encoding
ok 40 - malformed scale bound rejects before projection and encoding
  ---
  duration_ms: 0.48906
  type: 'test'
  ...
# Subtest: malformed non-ASCII identifier rejects before projection and encoding
ok 41 - malformed non-ASCII identifier rejects before projection and encoding
  ---
  duration_ms: 0.094076
  type: 'test'
  ...
# Subtest: malformed wrong authenticated owner rejects before projection and encoding
ok 42 - malformed wrong authenticated owner rejects before projection and encoding
  ---
  duration_ms: 1.869742
  type: 'test'
  ...
# Subtest: malformed trailing source rejects before projection and encoding
ok 43 - malformed trailing source rejects before projection and encoding
  ---
  duration_ms: 2.522141
  type: 'test'
  ...
# Subtest: malformed unterminated comment rejects before projection and encoding
ok 44 - malformed unterminated comment rejects before projection and encoding
  ---
  duration_ms: 0.428178
  type: 'test'
  ...
# Subtest: malformed source size bound rejects before projection and encoding
ok 45 - malformed source size bound rejects before projection and encoding
  ---
  duration_ms: 0.565234
  type: 'test'
  ...
# Subtest: malformed lone surrogate rejects before projection and encoding
ok 46 - malformed lone surrogate rejects before projection and encoding
  ---
  duration_ms: 0.136762
  type: 'test'
  ...
# Subtest: repayment inconsistent obligation rejects formation
ok 47 - repayment inconsistent obligation rejects formation
  ---
  duration_ms: 0.38008
  type: 'test'
  ...
# Subtest: caller objects and Proxy/coercion inputs are not parsed AST authority
ok 48 - caller objects and Proxy/coercion inputs are not parsed AST authority
  ---
  duration_ms: 0.677153
  type: 'test'
  ...
# Subtest: content comparison never authenticates or fills embedded claims
ok 49 - content comparison never authenticates or fills embedded claims
  ---
  duration_ms: 4.051115
  type: 'test'
  ...
# Subtest: malformed Source takes precedence over malformed comparison claim
ok 50 - malformed Source takes precedence over malformed comparison claim
  ---
  duration_ms: 0.098244
  type: 'test'
  ...
1..50
# tests 50
# suites 0
# pass 50
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 289.845271

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/transfer.source.mori

sha256: `80015b54c0e3ef22b5227dd5a2fc553ab78b017822e9fb2af908e9b98f62686f`

```text
profile "moriarty-financial-agreement-source/6";
agreement AgreementA {
 domain D; settlement A scale 0;
 selected TransferLiteralFee source_hash "claim-source" digest "claim-policy";
 intent {
  signer Owner key "key1"; nonce "n1"; pre_head "h0"; valid 0..10;
  gross_cap 11; fee_cap 1; net_floor 10; failure success_only;
  signed_action transfer from Owner to Recipient fee_to Fee value 10 fee 1;
  observations empty; disclosures empty; retained_effects empty; retained_duties empty;
  delegation none; recovery none;
 }
 authenticated {
  head "h0"; predecessor "genesis"; round 1;
  balance Owner 100; balance Recipient 0; balance Fee 0;
  allowance Owner remaining 100 spent 0; replay unused; work_remaining 10; work_spent 0;
 }
 submit transfer from Owner to Recipient fee_to Fee value 10 fee 1;
 effects { debit Owner 11; credit Recipient 10;
  credit Fee 1;
  use_allowance Owner 11; use_replay "n1"; advance_head "h0" "h1";
 }
 post_head "h1";
}

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/vectors.json

sha256: `59d673c6f6743ab56bfcd0391911c530b8cc0e5d3d028357f0352e61421412c1`

```text
[
  {
    "expectedFields": {
      "agreement": "AgreementA",
      "domain": "D",
      "asset": "A",
      "scale": "0",
      "selectedAction": "TransferLiteralFee",
      "operationKind": "Transfer"
    },
    "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74410001440001410000125472616e736665724c69746572616c46656501",
    "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000510600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e74410001440001410000125472616e736665724c69746572616c46656501",
    "payloadBytes": 81,
    "sha256": "01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a"
  },
  {
    "expectedFields": {
      "agreement": "AgreementA",
      "domain": "D",
      "asset": "A",
      "scale": "0",
      "selectedAction": "RepayAccrualFirst",
      "operationKind": "Repay"
    },
    "payloadHex": "0600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e744100014400014100001152657061794163637275616c466972737402",
    "preimageHex": "6d6f7269617274792d6d696c342d696d6167652f310001000000500600256d6f7269617274792d66696e616e6369616c2d61677265656d656e742d736f757263652f3601000a41677265656d656e744100014400014100001152657061794163637275616c466972737402",
    "payloadBytes": 80,
    "sha256": "c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d"
  }
]

```

## experiments/moriarty-language/formal/mil4/source-image-adapter/artifact-manifest.json

sha256: `d16505eb5de29622921ad9924a4a212dc5ef75929a63a95e5330e5b191be93d0`

```text
{
  "status": "local-source-image-experiment-not-adopted",
  "codecRepair": "proxy-repair-01",
  "independentExecutableReview": "pending",
  "tests": {
    "passed": 50,
    "failed": 0,
    "skipped": 0,
    "node": "v24.21.0",
    "command": "node --test --test-reporter=tap adapter.test.mjs",
    "scope": [
      "adapter.mjs",
      "adapter.test.mjs",
      "fixtures.mjs",
      "vectors.json",
      "protected-inputs.json"
    ]
  },
  "protectedInputsUnchanged": true,
  "artifacts": [
    {
      "path": "PLAN.md",
      "bytes": 1949,
      "sha256": "f65dfd37b1773c5b2b57f856f9fa13098128611ce535dd9ed372a300743d4a34"
    },
    {
      "path": "RESULT.md",
      "bytes": 6956,
      "sha256": "ba628d10dbc7acc19b5c3cc71c25d151e4f109636992151636d60a3bef86c37c"
    },
    {
      "path": "adapter.d.mts",
      "bytes": 755,
      "sha256": "cad8766e7e23c7d94ecdf3950c606f4f630729f31784cbb11ee8a7c967188f47"
    },
    {
      "path": "adapter.mjs",
      "bytes": 2095,
      "sha256": "62569c89ad6f91573fe68429935bb0d28fa56de08faa30679a8d396e6c742de2"
    },
    {
      "path": "adapter.test.mjs",
      "bytes": 9220,
      "sha256": "26d9286a2db51acc98caeceb4bc8b3f89b03746c0cab116898f6b3eb7521a8e6"
    },
    {
      "path": "expected-code-failure.tap",
      "bytes": 10756,
      "sha256": "e509dfe1f50178ae0a8ac895f3d8f38ef1407f07e0e562dd2533f3e7c88154ed"
    },
    {
      "path": "fixtures.mjs",
      "bytes": 3247,
      "sha256": "7d0b29eb92b2093ea7608e21198cd59ca90664438b50c5019c0ee5495e026e8c"
    },
    {
      "path": "initial-test-results.tap",
      "bytes": 55325,
      "sha256": "22d4eef875fb917cb629361cc10315ba7195d8735186665da29e403a2858471d"
    },
    {
      "path": "protected-inputs.json",
      "bytes": 1188,
      "sha256": "9766c0718ac7f3ea0b9b0e9b334d8c7a648ffbb67a05824593047aa3dc9468e2"
    },
    {
      "path": "reference-vectors.py",
      "bytes": 1404,
      "sha256": "e39d53490a010f12cd875e2ec0be06323a143fef3ef67361b3d1186242cb9483"
    },
    {
      "path": "repay.source.mori",
      "bytes": 1151,
      "sha256": "6db584de67fd6df4ec61ecec0bc7f3be14b20875763903913c20608bc98628ce"
    },
    {
      "path": "result-data.json",
      "bytes": 3944,
      "sha256": "dcf5b1162ed06db9f5a6a9a0357c9be366497399b94cb51884eccf9163b960ab"
    },
    {
      "path": "run-experiment.mjs",
      "bytes": 2834,
      "sha256": "89e01ef61abd9dff8ceff991bb163ce427d4d7f996249c8871ecd472d2c9fc41"
    },
    {
      "path": "run-results.json",
      "bytes": 395,
      "sha256": "c0e781e7c0e0198e21fd981afb079610ac0086a015d0fcf0184eda9501c1962d"
    },
    {
      "path": "test-results.tap",
      "bytes": 9791,
      "sha256": "f2e7748f9d32642bfda605504ba831b79c09b4edc628766c304c8392ab07520b"
    },
    {
      "path": "transfer.source.mori",
      "bytes": 957,
      "sha256": "80015b54c0e3ef22b5227dd5a2fc553ab78b017822e9fb2af908e9b98f62686f"
    },
    {
      "path": "vectors.json",
      "bytes": 1479,
      "sha256": "59d673c6f6743ab56bfcd0391911c530b8cc0e5d3d028357f0352e61421412c1"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/hash-image-codec/codec.mjs

sha256: `a94edacb2f1e81e03bc930d3f7921358e6b3c59b1b8ff1e25532f810874b4d62`

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

```

## experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts

sha256: `f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7`

```text
/** Provisional, closed Source/6 S0 presentation parser. No authentication occurs here. */
import {
  MIL4_S0_CORE, MIL4_S0_INTENT, MIL4_S0_SOURCE,
  type S0Effect, type S0Intent, type S0State,
} from './mil4-s0-core-v5.ts';

const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const encoder = new TextEncoder();
const RESERVED = new Set((
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head'
).split(' '));

type TokenKind = 'word' | 'integer' | 'string' | 'punctuation' | 'eof';
interface Token { kind: TokenKind; text: string; value: string; start: number; end: number }
export class Source6Error extends Error {
  readonly code: string;
  readonly offset: number;
  constructor(code: string, offset: number, message: string) {
    super(message);
    this.name = 'Source6Error';
    this.code = code;
    this.offset = offset;
  }
}
function fail(code: string, offset: number, message: string): never {
  throw new Source6Error(code, offset, message);
}
function isSurrogate(value: number): boolean { return value >= 0xd800 && value <= 0xdfff; }
function scalarString(value: string, offset: number): void {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && i + 1 < value.length) {
      const low = value.charCodeAt(i + 1);
      if (low >= 0xdc00 && low <= 0xdfff) { i++; continue; }
    }
    if (isSurrogate(c)) fail('INVALID_SURROGATE', offset + encoder.encode(value.slice(0, i)).length, 'Lone UTF-16 surrogate');
  }
}
function lexical(source: string): Token[] {
  scalarString(source, 0);
  if (encoder.encode(source).length > 65536) fail('SOURCE_BOUND', 0, 'Source exceeds 65536 UTF-8 bytes');
  const tokens: Token[] = [];
  let i = 0;
  let byte = 0;
  const advance = (end: number): void => { byte += encoder.encode(source.slice(i, end)).length; i = end; };
  const emit = (kind: TokenKind, end: number, value = source.slice(i, end)): void => {
    if (tokens.length >= 8191) fail('TOKEN_BOUND', byte, 'Too many tokens');
    const start = byte;
    const raw = source.slice(i, end);
    advance(end);
    tokens.push({ kind, text: raw, value, start, end: byte });
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { advance(i + 1); continue; }
    if (source.startsWith('//', i)) {
      const next = source.indexOf('\n', i + 2);
      advance(next < 0 ? source.length : next); continue;
    }
    if (source.startsWith('/*', i)) {
      const next = source.indexOf('*/', i + 2);
      if (next < 0) fail('UNTERMINATED_COMMENT', byte, 'Unclosed block comment');
      advance(next + 2); continue;
    }
    if (c === '"') {
      let end = i + 1;
      let escaped = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; break; }
        if (x === '\\' && !escaped) escaped = true;
        else escaped = false;
      }
      const raw = source.slice(i, end);
      if (!raw.endsWith('"') || raw.length < 2) fail('INVALID_STRING', byte, 'Unclosed string');
      let value: string;
      try { value = JSON.parse(raw) as string; }
      catch { fail('INVALID_STRING', byte, 'Invalid JSON string'); }
      scalarString(value, byte);
      if (encoder.encode(value).length > 1024) fail('STRING_BOUND', byte, 'Decoded string exceeds 1024 UTF-8 bytes');
      emit('string', end, value); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9]/.test(source[end])) end++;
      const raw = source.slice(i, end);
      if (raw.length > 78) fail('INTEGER_BOUND', byte, 'Integer exceeds 78 digits');
      if (!/^(0|[1-9][0-9]*)$/.test(raw)) fail('INVALID_INTEGER', byte, 'Noncanonical integer');
      emit('integer', end); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      const next = source.codePointAt(end);
      if (next !== undefined && /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(next)))
        fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
      if (end - i > 64) fail('IDENTIFIER_BOUND', byte, 'Identifier exceeds 64 characters');
      emit('word', end); continue;
    }
    const point = source.codePointAt(i)!;
    if (/[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(point)))
      fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
    if (source.startsWith('..', i)) { emit('punctuation', i + 2); continue; }
    if ('{};'.includes(c)) { emit('punctuation', i + 1); continue; }
    fail('UNEXPECTED_CHAR', byte, 'Unexpected source character');
  }
  tokens.push({ kind: 'eof', text: '', value: '', start: byte, end: byte });
  return tokens;
}

export type Source6Action =
  | { kind: 'Transfer'; from: string; to: string; feeTo: string; value: string; fee: string }
  | { kind: 'Repay'; obligation: string; payer: string; amount: string; conversion: 'identity' };
export interface Source6Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string; status: 'Outstanding';
}
export interface Source6Ast {
  profile: typeof MIL4_S0_SOURCE; programId: string; domain: string;
  settlement: { asset: string; scale: string };
  selected: { actionId: string; sourceHash: string; policyDigest: string };
  intent: {
    signer: string; keyRef: string; nonce: string; preHead: string;
    notBefore: string; notAfter: string; grossCap: string; feeCap: string; netFloor: string;
    signedAction: Source6Action; failure: 'success_only';
    observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty'; retainedDuties: 'empty';
    delegation: 'none'; recovery: 'none';
  };
  authenticated: {
    head: string; predecessor: string; round: string;
    balances: { account: string; amount: string }[];
    allowance: { owner: string; remaining: string; spent: string };
    obligation?: Source6Obligation; replay: 'unused' | 'consumed';
    workRemaining: string; workSpent: string;
  };
  submitted: { action: Source6Action; effects: S0Effect[]; postHead: string };
}

class Parser {
  private index = 0;
  private nodes = 0;
  private depth = 0;
  private readonly tokens: Token[];
  constructor(tokens: Token[]) { this.tokens = tokens; }
  private get here(): Token { return this.tokens[this.index]; }
  private node(): void {
    if (++this.nodes > 8192) fail('AST_BOUND', this.here.start, 'Too many AST nodes');
  }
  private enter(): void { if (++this.depth > 64) fail('DEPTH_BOUND', this.here.start, 'Nesting exceeds 64'); }
  private leave(): void { this.depth--; }
  private take(word: string, code = 'SOURCE6_SHAPE'): void {
    if (this.here.text !== word) fail(code, this.here.start, `Expected ${word}`);
    this.index++;
  }
  private effectTag(word: string): void {
    if (this.here.text !== word && this.here.kind === 'word'
        && !new Set(['debit', 'credit', 'set_obligation', 'use_allowance', 'use_replay', 'advance_head']).has(this.here.text))
      fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown effect tag');
    this.take(word);
  }
  private id(): string {
    const token = this.here;
    if (token.kind !== 'word' || RESERVED.has(token.text)) fail('SOURCE6_SHAPE', token.start, 'Expected identifier');
    this.index++; return token.value;
  }
  private string(): string {
    const token = this.here;
    if (token.kind !== 'string') fail('SOURCE6_SHAPE', token.start, 'Expected string');
    if (token.value.length === 0) fail('SOURCE6_SHAPE', token.start, 'Empty opaque string');
    this.index++; return token.value;
  }
  private uint(max: bigint = U128): string {
    const token = this.here;
    if (token.kind !== 'integer') fail('SOURCE6_SHAPE', token.start, 'Expected integer');
    this.index++;
    if (BigInt(token.value) > max) fail('SOURCE6_RANGE', token.start, 'Integer exceeds nominal bound');
    return token.value;
  }
  private action(): Source6Action {
    this.node();
    if (this.here.text === 'transfer') {
      this.take('transfer'); this.take('from'); const from = this.id();
      this.take('to'); const to = this.id(); this.take('fee_to'); const feeTo = this.id();
      this.take('value'); const value = this.uint(S128); this.take('fee'); const fee = this.uint(S128);
      this.take(';'); return { kind: 'Transfer', from, to, feeTo, value, fee };
    }
    if (this.here.text === 'repay') {
      this.take('repay'); this.take('obligation'); const obligation = this.id();
      this.take('payer'); const payer = this.id(); this.take('amount'); const amount = this.uint(S128);
      this.take('conversion'); this.take('identity'); this.take(';');
      return { kind: 'Repay', obligation, payer, amount, conversion: 'identity' };
    }
    fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown action');
  }
  private effects(action: Source6Action, asset: string): S0Effect[] {
    this.node(); this.enter(); this.take('effects'); this.take('{');
    const debit = (): S0Effect => { this.effectTag('debit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Debit', account, asset, amount }; };
    const credit = (): S0Effect => { this.effectTag('credit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Credit', account, asset, amount }; };
    const result: S0Effect[] = [debit(), credit()];
    if (action.kind === 'Transfer' && this.here.text === 'credit') result.push(credit());
    if (action.kind === 'Repay') {
      this.effectTag('set_obligation'); const id = this.id(); this.take('principal'); const principal = this.uint(S128);
      this.take('accrued'); const accrued = this.uint(S128); this.take('outstanding'); const outstanding = this.uint(S128);
      this.take('status');
      if (this.here.text !== 'outstanding' && this.here.text !== 'settled')
        fail('SOURCE6_SHAPE', this.here.start, 'Expected obligation status');
      const status = this.here.text === 'settled' ? 'Settled' : 'Outstanding'; this.index++; this.take(';');
      result.push({ kind: 'SetObligation', id, principal, accrued, outstanding, status });
    }
    this.effectTag('use_allowance'); const owner = this.id(); const amount = this.uint(); this.take(';');
    result.push({ kind: 'UseAllowance', owner, amount });
    this.effectTag('use_replay'); const key = this.string(); this.take(';'); result.push({ kind: 'UseReplay', key });
    this.effectTag('advance_head'); const predecessor = this.string(); const successor = this.string(); this.take(';');
    result.push({ kind: 'AdvanceHead', predecessor, successor }); this.take('}'); this.leave();
    return result;
  }
  parse(): Source6Ast {
    this.node(); this.take('profile', 'SOURCE6_VERSION');
    const profileToken = this.here;
    if (profileToken.kind !== 'string' || profileToken.text !== '"moriarty-financial-agreement-source/6"')
      fail('SOURCE6_VERSION', profileToken.start, 'Unsupported source profile');
    this.index++;
    this.take(';'); this.take('agreement'); const programId = this.id(); this.take('{'); this.enter();
    this.take('domain'); const domain = this.id(); this.take(';');
    this.take('settlement'); const asset = this.id(); this.take('scale'); const scale = this.uint(18n); this.take(';');
    this.take('selected'); const actionId = this.id(); this.take('source_hash'); const sourceHash = this.string();
    this.take('digest'); const policyDigest = this.string(); this.take(';');
    this.take('intent'); this.take('{'); this.enter(); this.node();
    this.take('signer'); const signer = this.id(); this.take('key'); const keyRef = this.string(); this.take(';');
    this.take('nonce'); const nonce = this.string(); this.take(';');
    this.take('pre_head'); const preHead = this.string(); this.take(';');
    this.take('valid'); const notBefore = this.uint(); this.take('..'); const notAfter = this.uint(); this.take(';');
    if (BigInt(notBefore) > BigInt(notAfter))
      fail('SOURCE6_RANGE', this.here.start, 'Validity lower bound exceeds upper bound');
    this.take('gross_cap'); const grossCap = this.uint(S128); this.take(';');
    this.take('fee_cap'); const feeCap = this.uint(S128); this.take(';');
    this.take('net_floor'); const netFloor = this.uint(S128); this.take(';');
    this.take('failure'); this.take('success_only'); this.take(';'); this.take('signed_action');
    const signedAction = this.action();
    for (const field of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) {
      this.take(field); this.take('empty'); this.take(';');
    }
    this.take('delegation'); this.take('none'); this.take(';');
    this.take('recovery'); this.take('none'); this.take(';'); this.take('}'); this.leave();
    this.take('authenticated'); this.take('{'); this.enter(); this.node();
    this.take('head'); const head = this.string(); this.take(';');
    this.take('predecessor'); const predecessor = this.string(); this.take(';');
    this.take('round'); const round = this.uint(); this.take(';');
    const balances: { account: string; amount: string }[] = [];
    const balanceCount = signedAction.kind === 'Transfer' ? 3 : 2;
    for (let i = 0; i < balanceCount; i++) {
      this.take('balance', 'SOURCE6_CELL_SHAPE');
      balances.push({ account: this.id(), amount: this.uint() }); this.take(';');
    }
    if (this.here.text === 'balance')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Too many balance rows');
    this.take('allowance'); const owner = this.id(); this.take('remaining'); const remaining = this.uint();
    this.take('spent'); const spent = this.uint(); this.take(';');
    let obligation: Source6Obligation | undefined;
    if (signedAction.kind === 'Repay') {
      this.take('obligation', 'SOURCE6_CELL_SHAPE'); const id = this.id(); this.take('{'); this.enter(); this.node();
      this.take('debtor'); const debtor = this.id(); this.take(';');
      this.take('creditor'); const creditor = this.id(); this.take(';');
      this.take('asset'); const obligationAsset = this.id(); this.take(';');
      this.take('principal'); const principal = this.uint(S128); this.take(';');
      this.take('accrued'); const accrued = this.uint(S128); this.take(';');
      this.take('outstanding'); const outstanding = this.uint(S128); this.take(';');
      this.take('status'); this.take('outstanding'); this.take(';'); this.take('}'); this.leave();
      obligation = { id, debtor, creditor, asset: obligationAsset, principal, accrued, outstanding, status: 'Outstanding' };
    }
    if (this.here.text === 'obligation')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Unexpected obligation row');
    this.take('replay');
    if (this.here.text !== 'unused' && this.here.text !== 'consumed')
      fail('SOURCE6_SHAPE', this.here.start, 'Expected replay status');
    const replay = this.here.text as 'unused' | 'consumed'; this.index++; this.take(';');
    this.take('work_remaining'); const workRemaining = this.uint(); this.take(';');
    this.take('work_spent'); const workSpent = this.uint(); this.take(';'); this.take('}'); this.leave();
    this.take('submit'); const action = this.action(); const effects = this.effects(action, asset);
    this.take('post_head'); const postHead = this.string(); this.take(';'); this.take('}'); this.leave();
    if (this.tokens[this.index].kind !== 'eof') fail('SOURCE6_SHAPE', this.here.start, 'Trailing source');
    const expectedActionId = signedAction.kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst';
    if (actionId !== expectedActionId)
      fail('SOURCE6_PROFILE_UNSUPPORTED', this.here.start, 'Selected action is outside the S0 profile');
    const accounts = balances.map((row) => row.account);
    const expectedAccounts = signedAction.kind === 'Transfer'
      ? [signedAction.from, signedAction.to, signedAction.feeTo]
      : [signedAction.payer, obligation?.creditor];
    if (accounts.some((id, i) => id !== expectedAccounts[i]) || new Set(accounts).size !== accounts.length
        || owner !== signer || (signedAction.kind === 'Transfer' &&
          (signedAction.from !== signer || new Set([signedAction.from, signedAction.to, signedAction.feeTo]).size !== 3))
        || (signedAction.kind === 'Repay' && (!obligation || obligation.id !== signedAction.obligation
          || signedAction.payer !== signer || obligation.debtor !== signer || obligation.asset !== asset
          || obligation.creditor === signer)))
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Authenticated cells do not match action');
    if (BigInt(workRemaining) + BigInt(workSpent) > U128
        || BigInt(remaining) + BigInt(spent) > U128)
      fail('SOURCE6_RANGE', this.here.start, 'Counter total exceeds UInt128');
    if (obligation && BigInt(obligation.principal) + BigInt(obligation.accrued) !== BigInt(obligation.outstanding))
      fail('SOURCE6_RANGE', this.here.start, 'Obligation outstanding must equal principal plus accrued');
    return {
      profile: MIL4_S0_SOURCE, programId, domain, settlement: { asset, scale },
      selected: { actionId, sourceHash, policyDigest },
      intent: { signer, keyRef, nonce, preHead, notBefore, notAfter, grossCap, feeCap, netFloor,
        signedAction, failure: 'success_only', observations: 'empty', disclosures: 'empty',
        retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none' },
      authenticated: { head, predecessor, round, balances, allowance: { owner, remaining, spent },
        obligation, replay, workRemaining, workSpent },
      submitted: { action, effects, postHead },
    };
  }
}

/** Parse one exact Source/6 S0 document. Throws Source6Error on formation failure. */
export function parseSource6(source: string): Source6Ast { return new Parser(lexical(source)).parse(); }

export interface Source6Lowered {
  ast: Source6Ast; state: S0State; intent: S0Intent;
  submittedEffects: S0Effect[]; proposedPostHead: string;
}

/** Lower claims for local Core/5 preparation; no signed digest or external premise is manufactured. */
export function lowerSource6(ast: Source6Ast): Source6Lowered {
  const { authenticated: auth, intent: signed, submitted, settlement, selected } = ast;
  const replayKey = JSON.stringify([ast.domain, signed.signer, signed.nonce]);
  const state: S0State = {
    core: MIL4_S0_CORE, domain: ast.domain, asset: settlement.asset, head: auth.head,
    round: auth.round, workRemaining: auth.workRemaining, workSpent: auth.workSpent,
    balances: auth.balances.map((row) => ({ ...row })), allowances: [{ ...auth.allowance }],
    obligations: auth.obligation ? [{ ...auth.obligation }] : [],
    consumedReplay: auth.replay === 'consumed' ? [replayKey] : [],
  };
  const base = {
    version: MIL4_S0_INTENT, core: MIL4_S0_CORE, sourceProfile: MIL4_S0_SOURCE,
    programId: selected.actionId, sourceHash: selected.sourceHash, policyDigest: selected.policyDigest,
    keyRef: signed.keyRef, domain: ast.domain, asset: settlement.asset,
    signer: signed.signer, nonce: signed.nonce, preHead: signed.preHead,
    notBefore: signed.notBefore, notAfter: signed.notAfter,
    grossCap: signed.grossCap, feeCap: signed.feeCap, netFloor: signed.netFloor,
  };
  const action = signed.signedAction;
  const intent: S0Intent = action.kind === 'Transfer'
    ? { ...base, kind: 'Transfer', recipient: action.to, feeRecipient: action.feeTo,
        amount: action.value, fee: action.fee }
    : { ...base, kind: 'Repay', obligationId: action.obligation, amount: action.amount };
  // Source `use_replay` names a nonce. Core/5 compares its domain/signer/nonce tuple.
  const submittedEffects = submitted.effects.map((effect): S0Effect =>
    effect.kind === 'UseReplay'
      ? { kind: 'UseReplay', key: JSON.stringify([ast.domain, signed.signer, effect.key]) }
      : { ...effect });
  return { ast, state, intent, submittedEffects, proposedPostHead: submitted.postHead };
}

export function parseAndLowerSource6(source: string): Source6Lowered {
  return lowerSource6(parseSource6(source));
}

```

