Independent read-only W-D2H B05-B07 hash-image design audit. Review ONLY exact embedded bytes; no tools, skills, delegation, external pages or workspace files. Requested GPT-6.1 Sol high and Grok 4.7 xhigh independently. This is a proposed design, not an adopted image codec or authenticated producer. Check Source/6 claim self-reference avoidance; canonical nine-field definition versus parsed-location document projection; purpose-separated SHA256 envelope and exact field serialization; raw three-module Core package closure and loaded-artifact correspondence limit; policy image acyclicity, signed scope, tag8/11/12 first-failure and deferred later terms; 3 specified positive plus 18 hostile/invariance oracles; compatibility with embedded W-D2F/G contracts and upgrade/version consequences. Find high/medium contradictions, omitted signed terms, false execution or hash proof claims. Zero image implementation, computed vectors, tests, signature or ledger acceptance. W-D2/Sprint1 remain open.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-images/SPEC.md",
    "bytes": 24839,
    "sha256": "d21a28bc6610d22fe3c964d864adc1f5ae7799d9f4fa0df69f317532ac6d1338"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-images/DECISION-MATRIX.md",
    "bytes": 13850,
    "sha256": "d6b9c64611387ba7ca664c8d9f9e6dc6c41d43bc63170a000c8e2486b65263cc"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/hash-images/RESULT.md",
    "bytes": 4190,
    "sha256": "d9d8da24598d363c87b4eef7d8c7dc77ec4cab3c7db3dca527db1bb79cc6b8fe"
  },
  {
    "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
    "bytes": 11648,
    "sha256": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec"
  },
  {
    "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf",
    "bytes": 3753,
    "sha256": "5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e"
  },
  {
    "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
    "bytes": 20746,
    "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
    "bytes": 17917,
    "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts",
    "bytes": 2640,
    "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/s0-implementation-contract.md",
    "bytes": 6518,
    "sha256": "a7f564e3c0a72f549b09849fdd7d3de5e0b0ff1b74ec34261bd9a229acf9a788"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/SPEC.md",
    "bytes": 9579,
    "sha256": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md",
    "bytes": 34536,
    "sha256": "e51cd539699bd0afd21f0c81aecd0af39bf12e24e8c0a50fba88f8929e9c1129"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md",
    "bytes": 17505,
    "sha256": "b7cabbe427cb9ea9d05010b498baec30275c0a939070b4379bb42a34a99981e0"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/identity-binding/SPEC.md",
    "bytes": 22103,
    "sha256": "33b4fcb274af1d4dd4a053b6019a09e03565250e53669af6f304c9d9488745c0"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md",
    "bytes": 8671,
    "sha256": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8"
  }
]
```

## experiments/moriarty-language/formal/mil4/hash-images/SPEC.md

sha256: `d21a28bc6610d22fe3c964d864adc1f5ae7799d9f4fa0df69f317532ac6d1338`

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

**A — parsed-location document projection.** Take the exact UTF-8 source
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
the claims still require independent checks.

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
| `text` | UInt16 big-endian UTF-8 byte count then decoded Unicode scalar values; 1–1024 bytes; no normalization, trimming, case folding or original JSON escape text |
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

### Policy payload, purpose4

The policy is an exact bounded authorization configuration, scoped to the
selected definition and implementation. This proposal fixes action arguments
and validity bounds in the policy. It does not infer a reusable general policy
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
producing a candidate artifact; exact Source failure/empty/none literals map
through F-S0-01. `supplyChanges=[]` is the closed S0 wire constant and is
encoded explicitly despite Source having no supply-change slot. Both
operation discriminators must agree. Source actionId and CoreProgramId follow
W-D2G mapping A; equal text does not merge their nominal sorts.

**Excluded:** the embedded policyDigest itself, sourceHash's embedded claim,
nonce, intent.preHead, snapshot/prior-head/round/cells/replay/work, submitted
action/effects/successor, stage/episode, raw signerKey, signature, effect
commitment, authorization digest, proof and registry evidence. Computed Source
and Core digests are included instead of trusting claim strings. keyRef is
an exact decoded reference, not a key encoding; B08 still authenticates its
relation to the signed key. Fresh nonce/current head permit a repeated proposal
under the same configuration, subject to all existing replay/snapshot/authority
checks. Changing amount, zero-fee recipient, interval, keyRef or any cap changes
the policy image. No term is inferred from a net balance delta.

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
| tag8/B05 | First compare wire.sourceHash to AST.selected.sourceHash. Then authenticate the selected Source artifact/definition under the prior B01–B04 context, encode its purpose1 image and retain the computed Source digest and declared fields. Compare current sourceHash to that digest. Earlier domain/agreement/action roles must agree with the already bound context. Retain asset/scale/constructor for comparisons at16/17/35. |
| tag10 | Preserve Source selected.actionId == lowered intent.programId == signed coreProgramId and B04's retained selector/Core ID. Agreement ID is never Core programId. |
| tag11/B06 | Authenticate exact selected package and independently verified correspondence/loaded-artifact binding; compute purpose3 hash. Compare signed coreHash to the computed hash. Package Core ID/kind is tied to B04's registered pair, with signed operation-kind equality deferred to35. |
| tag12/B07 | First compare wire.policyHash to AST.selected.policyDigest. Then authenticate the independent policy artifact and its computed Source/Core links; compute purpose4 hash. Compare current policyHash to the artifact digest. Retain policy terms; compare submitted later terms at their own anchors, not early at12. |

For tag ordering, image producers may derive a policy/definition from a complete
candidate AST, but consumers recompute authenticated artifact images, then
retain their fields. They must not hash the candidate's later fields at8/12
and thereby diagnose an asset, scale, cap or endpoint mismatch early. Compare
retained Source/policy asset at16, scale at17, signer/keyRef relation at13,
validFrom/validUntil at21/22, caps/floor at23/24/25, failure at27, explicit
empty/none fields at28–34, and operation terms at35. Source keyRef equality
uses `sourcePath=intent.keyRef` under the signer binding, with no fictional
wire keyRef slot. B08's key authorization is still required there.
At12 compare the policy artifact's sourceHash/coreHash links to already
retained computed digests; wrong links are invalid selected-context evidence
if they do not prove the required B07 relation. A valid bound policy whose
terms differ from later submitted fields yields a later field mismatch.

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
whether their definition/evidence is absent/invalid. Never try purposes1 and2
until one hash matches. Concurrent suites need an explicit authenticated version
binding or a separately reviewed wire/profile version; no wire/4 is chosen here.

- Definition image changes to field inclusion, constructor meaning, bounds,
  serialization or domain prefix require a new image/profile version and
  fresh independent vectors. A future Source grammar with real action bodies
  needs a new committed body/lowering relation, not reuse of this projection.
- Exact package byte changes, even comments/line endings or an unused branch,
  change coreHash. Compatible behavior may retain Core/5 only after an explicit
  compatibility/correspondence decision; changed semantics require versioned
  contract review. A hash change alone never certifies compatibility.
- New imports, runtime/backend mappings or altered package roles require a
  reviewed package-image profile/correspondence. No compiler/toolchain change
  inherits prior correspondence evidence automatically.
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

## experiments/moriarty-language/formal/mil4/hash-images/DECISION-MATRIX.md

sha256: `d6b9c64611387ba7ca664c8d9f9e6dc6c41d43bc63170a000c8e2486b65263cc`

```text
# W-D2H decision matrix and design oracles

**Status: proposed / specified-only, 2026-09-30.** No vote, implementation,
test run, hash vector, authentication or acceptance result is recorded here.
The [SPEC](SPEC.md) supplies exact recommended payloads and ordering.

## Source strategy comparison

| Criterion | A: parsed-location document projection | B: canonical selected-definition projection |
| --- | --- | --- |
| Hash purpose | 02 | 01 |
| Included bytes | Exact UTF-8 document except two replaced selected claim tokens; all remaining comments, whitespace, runtime state and proposal terms | Binary nine-field tuple: Source version/profile, wire profile, agreement, domain, settlement asset/scale, selected builtin and constructor |
| Excluded claims | Only selected sourceHash/policyDigest string tokens are replaced with exact distinct fixed markers | All digest claims; policy/runtime/submission fields are outside this definition image |
| Self-reference | None: replacing either claim's whole token fixes the input independently of that claim's value/length | None: both claim leaves absent; separate policy includes computed Source/Core hashes and excludes itself |
| Whitespace/comments | Any remaining byte change changes source image; offsets must be grammar-derived | Source presentation is ignored after formation; oversized/malformed source still rejects before projection |
| Runtime change | Snapshot, nonce, effects and successor change sourceHash | These do not change definition hash; their own signed/authenticated checks remain mandatory |
| Policy change | Policy terms change sourceHash as well as policyHash | Policy terms change policyHash; definition hash stays stable unless scoped definition fields change |
| Builtin identity | Selector bytes included, exact code still needs separate Core commitment | Selector/kind pair included; actual code/lowerer/wrapper committed by Core package |
| Stable reusable meaning | Exact stage presentation with two exclusions; no reusable code identity | Instance-scoped builtin selection/settlement definition; no user-defined action body or template reuse claim |
| Failure risk | Naive mask can miss escaped tokens, mask comments or create unexpected omission; new token-span extractor needed | Overbroad reuse can omit future action-body semantics; reject future profiles until new body image is adopted |
| Consequence of adoption | Every snapshot/presentation update needs fresh Source commitment/signature; digest-independent successor remains required | Explicitly separate definition, exact implementation and bounded policy; later fields checked at their own tags |
| Recommendation | Keep as the fully specified alternative, not a fallback hash mode | Recommend for this closed S0 proposal; still requires independent adoption |

Hashing the entire unmodified Source document is rejected because it contains
its own `source_hash`. Hashing a builtin name alone is rejected as B06 because
that name cannot identify changed code. Hashing only an intent or effect vector
cannot identify the parser/lowerer/wrapper that produced it. No fixed-point
search, caller-selected mask, arbitrary JSON serialization or legacy opaque
claim becomes an accepted strategy by matching a value.

## Core and policy identity choices

| Choice | Benefit | Cost / required evidence | Recommendation |
| --- | --- | --- | --- |
| Closed raw three-module Core package | Exact implementation-source bytes and selected builtin are unambiguous; includes shared lowering/wrapper behavior | Comments/unused branch changes invalidate hash; actual loaded execution and correspondence still need independent evidence | Proposed purpose3 for current host prototype only |
| New canonical abstract Core program image | Could survive implementation presentation changes and identify semantics directly | Core/5 has no canonical program serializer or proved compiler relation; defining an IR is a separate consequential design | Defer; do not invent a name-only stand-in |
| Exact scoped policy with Source/Core links and fixed signed terms | A policy cannot migrate to another definition/package by preserving a digest claim; endpoints/caps/interval/zero-fee recipient are explicit | New amount/interval/key/cap needs a new policy; nonce/head/history remain separate; real policy authority is unavailable | Proposed purpose4, without adopting a general policy language |

## Exact design-oracle conventions

These are independently specified predicates for later implementation and
review, not observed results. Let `S(x)`, `C(p)` and `P(q,S,C)` denote the
complete recommended preimage bytes from SPEC, and `H` SHA256. Construct two
new Source positives with strict W-D2G action IDs, one fee transfer and one
accrual-first repayment. Parseable documents must supply compatible ordered
cells, canonical hash-shaped claims, exact submitted actions/effects and an
independent successor. Existing W-D2E fixtures are not these positives.

Standalone image predicates below need no synthetic authentication. Any
full-path expectation additionally requires independent real passing evidence
for every preceding F/D/M gate, including B01–B04. For an anchor8/11/12 oracle,
all earlier anchors pass. Later Core/image/ledger outcomes are never inferred.
A positive image comparison would establish bytes/content only, with null
published effects/post; it would not establish a full consumer or B17.
Expected digest equality follows identical bytes. Unequal preimages must be
asserted directly; expected unequal SHA256 values additionally rely on the
hash's collision-resistance assumption and are not a proof of injectivity.

| ID | Exact positive or hostile construction | Proposed image / path oracle |
| --- | --- | --- |
| H-P01 | Strict transfer Source selectedActionId/coreProgramId=TransferLiteralFee, constructorTransfer; compute three images from independent selected artifacts, then put their hashes into AST/wire claims | Replacing claims leaves S/P unchanged; artifact digest comparisons at8/11/12 pass as isolated predicates. Full path still requires all other bindings. |
| H-P02 | Strict repayment selectedActionId/coreProgramId=RepayAccrualFirst, constructorRepay, allocationAccrualFirst, conversionidentity, one matching obligation | Image selection/constructor and policy fields pass; repayment C differs from transfer C even for identical package files because ID/kind bytes differ. This is not repayment execution evidence. |
| H-P03 | Add ASCII whitespace/CRLF and comments between tokens, below all Source bounds; preserve parsed values and the exact required raw profile token | Recommended S/P and their digests are byte-identical; A's projected document bytes differ. No broad semantic-equivalence claim follows. |
| H-H01 | Change only Source selected.sourceHash to another canonical 64-hex value and change wire.sourceHash to the same value, leaving authenticated artifact unchanged | S/P producer bytes do not change; tag8 fails sourceHash/computed-artifact equality, W_D2F_FIELD_MISMATCH. Claim equality alone cannot pass B05. |
| H-H02 | Change only Source selected.policyDigest and wire.policyHash together, leaving the selected policy artifact unchanged | No feedback into S/C/P; tag12 policyHash mismatch. An authorizationDigest presented as `digest` is the same hostile case unless it independently equals the correct policy commitment; equal content still cannot supply policy authority. |
| H-H03 | Try to hash the complete original Source bytes with source_hash included, or include policyHash in its own policy payload | Rejected profile/definition; only purpose1 or reviewed purpose2 exclusions are defined. A caller's fixed-point/equal-claim assertion supplies no accepted image. Missing adopted image/provider uses B05/B07 unavailable; invalid evidence under an actual selected verifier uses invalid at the owning anchor. No encoder for such a mode is implemented. |
| H-H04 | Change Transfer's selected.actionId to RepayAccrualFirst without changing the Transfer constructor | Existing Source formation rejects SOURCE6_PROFILE_UNSUPPORTED before image or adapter checks. No tag8/10 oracle is reachable. |
| H-H05 | Keep strict transfer Source and valid B04 provider, change only signed coreProgramId to RepayAccrualFirst | Tag10 coreProgramId mismatch after tag8 passes; never diagnose it as invalid B04 at6. Source definition stays fixed; wrong package is not substituted. |
| H-H06 | Provide authentic alternative package bytes with one Core byte changed, retain the original signed coreHash; otherwise valid selected-program/correspondence evidence for the alternative package | Purpose3 preimage differs; tag11 coreHash mismatch. A real verifier's inability to establish the changed package correspondence instead produces B06 invalid/unavailable before that equality; these are separate prerequisites. No success from unchanged builtin name. |
| H-H07 | Keep the original signed coreHash and exact package files, but run a different lowerer, an uncommitted imported helper or another runtime/compiled artifact | B06 correspondence/loaded-artifact obligation fails (invalid with real verifier, unavailable without it). Matching source package hash alone cannot reach positive execution correspondence. New module closure requires a new package profile. |
| H-H08 | Change only a comment or LF/CRLF in one committed package file, with old coreHash | Core preimage differs and needs a fresh hash/correspondence decision; tag11 coreHash mismatch given independent evidence for new bytes. A comment change in a Source proposal instead follows H-P03. |
| H-H09 | Change only Source/wire grossCap from100 to101 while retaining a valid selected policy with grossCap100, identical nonce/head/definition/package/hash claims | Source definition and selected artifact policy preimages unchanged; tag12 hash checks pass, then tag23 grossCap field mismatch. Do not recompute submitted later terms at12 and reorder the diagnosis. An independent candidate policy producer would encode different P bytes. |
| H-H10 | Change policy artifact's feeRecipient with fee=0, recompute its policy hash, but retain original Source/wire feeRecipient; independently authenticate the new policy for the same required context | New P bytes differ despite no positive fee Credit; hash equality at12 can pass after updating claims. Exact operation.feeRecipient mismatch at35. Earlier signer/interval/caps/empty terms must match. No netted effect or fee=0 exemption. |
| H-H11 | Policy artifact links to another computed Source definition or Core package under a different instance/selector, with a freshly computed policy digest and internally equal claim strings | Required B07 selected-context relation fails at12; equal digest claims are insufficient. With no actual verifier this remains unavailable, not an observed evidence rejection. |
| H-H12 | Same selected policy, vary only fresh nonce/preHead and matching snapshot/successor/effects for a legitimate next proposal | S/C/P preimages remain identical; canonical authorization bytes change. Snapshot/history/replay/head/signature gates must be freshly verified; old signature/replay consumption does not carry over. This is an image invariance oracle, not a full-path positive. |
| H-H13 | Change snapshot balance/work spent, predecessor, unrelated replay history or submitted effect while retaining S/C/P | Their images remain identical. B11/B12/B16 claim checks or Core/effect image comparison must detect the hostile content at its existing anchor. Matching hashes cannot authorize it; no [] history shortcut. |
| H-H14 | Change Source agreement/domain/asset/scale/selected definition, preserve old Source claim and authenticate the changed definition independently | S bytes differ. Earlier domain/agreement/action literal/provider differences win if present; otherwise old computed-source claim rejects at8. A valid provider with later asset/scale differences retains them for16/17, without early tag8 comparison to those submitted fields. |
| H-H15 | Use identical payload bytes x with purpose01 and purpose03, or purpose03 and04; try to relabel a digest/parse payload from the other role | Complete preimages differ at the purpose byte. Typed encoder/decoder rejects a payload of the wrong purpose/shape; raw digest equality cannot authenticate the missing scoped relation. Even a deliberately assumed SHA256 collision must not enable cross-sort coercion. Do not assert a mathematical proof that SHA256 cannot collide. |
| H-H16 | Change only raw JSON escape spelling of keyRef while preserving its decoded scalar value; keep legal Source profile's raw header unchanged | P text bytes/digest unchanged; A document bytes change. Different decoded keyRef changes P and is checked under the policy/signer relation; neither spelling converts keyRef to signerKey. |
| H-H17 | Change purpose/image version, add an unknown slot, use a noncanonical ID/round/scale, omit a package role, reorder roles or append a byte | Proposed image format rejects; no alternate-suite retry. Source formation/common-domain failures retain their earlier diagnostics. No new image-code execution is claimed. |
| H-H18 | Use unchanged image/claims but omit every authentication provider | Current planned full path stops at agreementId/B01 after F/D/direct checks; standalone content predicates remain possible. Do not fabricate providers to report that8/11/12 passed end-to-end. |

No binary/vector files or tests are created. A later implementation sprint must
freeze independently generated exact bytes, claimed-versus-computed hashes and
expected outcomes before its encoder/consumer is written, then exercise these
cases through the actual callable path. Real signature/snapshot/native/ledger
controls are additional evidence, not implied by this table.

```

## experiments/moriarty-language/formal/mil4/hash-images/RESULT.md

sha256: `d9d8da24598d363c87b4eef7d8c7dc77ec4cab3c7db3dca527db1bb79cc6b8fe`

```text
# W-D2H bounded design result

**Status: proposed / specified-only, 2026-09-30.** This is an author design
result, not an approval, adoption, implementation or authentication result.
W-D2/W-D3 and B01–B17 remain open.

## Delivered scope

Only [SPEC](SPEC.md), [DECISION-MATRIX](DECISION-MATRIX.md) and this RESULT are
created in `formal/mil4/hash-images/`. W-D2F/W-D2G, source modules, fixtures,
protected records and historical evidence are unchanged by this sprint.

Repository inspection covers current W-D2F FIELD-MAP/PLAN, W-D2G SPEC,
Source/6 frontend/grammar/contract, Core/5 preparer/S0 contract, the Source
wrapper and wire/3 specification. The development skill was loaded and guarded
status inspected. Status still reports unresolved product execution/admission
gaps; no registered execution was dispatched. AGENTS' historical September10
ACTIVE-ROUTING/assignment paths are absent in this worktree; no execution or
model-routing substitution is inferred from that absence.

## Findings and recommendation

Repository observation: Source/6 is a builtin-selection stage proposal, not a
user-defined program body. Its embedded Source/policy claims are opaque and
currently copied unchanged. Whole-document hashing without exclusions would
self-reference `source_hash`; no current producer resolves that issue.

Recommendation: select the canonical instance-scoped Source definition
projection, an exact raw three-module Core implementation package, and an
exact scoped policy that commits computed Source/Core digests. SPEC gives
included/excluded bytes, fixed purposes, primitive widths, package roles,
static literals, action variants and acyclic generation order. A grammar-located
two-token document projection is the independently specified alternative.

Inference: this split makes Source presentation changes harmless to the
definition while preserving exact implementation identity and explicit
policy terms. It does not authenticate any artifact or prove that committed
source bytes were executed. B06 still needs independently verified loaded
artifact/toolchain/lowering correspondence, and B16 must replace the local
lowerer's incomplete replay projection with an adopted authenticated transport.

SPEC preserves W-D2F's tag order:8/11/12 compare authenticated artifact hashes,
while retained asset/scale/policy terms compare at their own later fields.
This avoids using an early candidate hash to move a cap/endpoint mismatch
ahead of its specified anchor. Upgrade consequences include fresh
signature evidence for changed wire claims, conservative Core hash changes
even on package comments, explicit suite/version binding and no implicit
state/replay/registry reset. Wire/3's existing content-digest/signature-message
boundary is unchanged.

The decision matrix defines2 positive construction oracles, presentation
invariance and18 hostile/invariance design cases, including self-reference,
selected-action mismatch, exact-code mismatch, policy/context mismatch,
zero-fee recipient, lowerer substitution, incomplete history and cross-purpose
digest collision. These are expected predicates, not frozen digest vectors
or executed outcomes.

## Evidence limits and next bounded work

Implementation changes: **0**. New encoders/consumers/verifiers: **0**.
Tests added/run: **0**. Computed Source/Core/policy digest vectors: **0**.
Authentication/correspondence proofs/native evidence/ledger submissions: **0**.
Independent audits/decision votes: **0**. No gate is closed.

The author performed a text consistency review of field inclusion, dependency
cycles, tag ordering, selector roles and claim-versus-computation language.
This is not an independent audit or an executed design-oracle result.

Next bounded work: independently review the proposed suite versus the document
alternative and the exact-code-versus-abstract-Core choice. Only after adoption
freeze independent image bytes/digests and hostile expectations, then implement
the smallest image encoder/comparator with its honest content-only scope.
Full authenticated consumption additionally needs actual B01–B17 definitions
and evidence at their existing gates; B17 remains a separate ledger result.

```

## experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md

sha256: `725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec`

```text
# Financial agreement Source/6 and Core/5 S0 contract

**Status:** provisional Sprint 0 specification, 2026-09-29. The grammar is a closed presentation syntax for one S0 stage proposal, and an isolated Source/6 parser and local Core/5 preparer implement this first slice. It is not a signed `/3` encoding or an adopted MIL/4 profile. W-D0–W-D4 remain open. See the [MIL/4 semantic contract](../../formal/mil4/semantics-contract.md) and [S0 implementation contract](../../formal/mil4/s0-implementation-contract.md).

## Formation and version gate

The [EBNF](financial-agreement-source-v6-grammar.ebnf) parses exactly one `profile`, one `agreement`, and one stage proposal. Its header must contain the raw token `"moriarty-financial-agreement-source/6"` before parsing the agreement body; a differently escaped spelling is outside this provisional presentation. The fixed declaration and field order is presentation syntax, not a claim about signed bytes. Unknown, repeated, omitted or out-of-order fields reject at Source/6 formation with `SOURCE6_SHAPE`; an unknown action or effect tag rejects with `SOURCE6_UNKNOWN_TAG`. A different header rejects with `SOURCE6_VERSION`. The closed source grammar also rejects a nonempty failure, observation, disclosure, retained-effect or duty form at formation with `SOURCE6_SHAPE`; `S0_FAILURE_UNSUPPORTED` applies only when a typed Core/5 stage reaches the failure judgment. These source rejections have no Core/5 term or K transition. A local Source/5 parser test confirms that a `/6` header rejects with `PROFILE_MISMATCH`; Core/4-to-Core/5 compatibility has not been implemented or verified here.

The grammar borrows only the token definitions from `lexical.md`; all literal words in the Source/6 EBNF are profile-local reserved words and cannot be identifiers. This does not alter Source/5 keywords. Limits are simultaneous: source UTF-8 bytes ≤65536, tokens ≤8192, AST nodes ≤8192, nesting depth ≤64, identifiers ≤64 ASCII characters, decoded strings ≤1024 UTF-8 bytes, and one stage/effect vector per document. Opaque string fields in this S0 presentation must be nonempty. `scale` is 0..18; each nominal amount (`value`, `fee`, `amount`, caps and floor) is 0..`2^127−1`; each balance, allowance counter and work count is 0..`2^128−1`. S0 uses checked UInt128 intermediates, including `value+fee`, `principal+accrued`, credits and spent counters. The current source/kernel nominal bound does not by itself impose the same bound on every lifecycle state field. This S0 proposal additionally caps principal, accrued and outstanding at `2^127−1` and requires `outstanding=principal+accrued`. W-D4 must review that additional narrowing. Invalid bounds, including an inverted validity interval, reject before K admission with `SOURCE6_RANGE`.

Source/6 formation applies an action-dependent shape check after the closed EBNF parses: transfer requires three balance rows in owner, recipient, fee-recipient order and no obligation; repay requires two rows in payer, bound-creditor order and exactly one obligation. All endpoint identifiers are pairwise distinct for transfer; payer and creditor differ for repay. Duplicate or missing authenticated cells reject `SOURCE6_CELL_SHAPE`; an absent receiver balance is not silently initialized. The allowance owner equals signer. The replay row states whether the selected signed key is unused or consumed; a consumed row rejects at history. The authenticated block is a *claim* until a snapshot-to-head premise establishes each cell and the current head. The strings used for key, nonce, digest and heads are opaque typed identifiers in this syntax, not hash byte definitions.

## Source fields to Core/5

One elaboration proposes `Core5Stage(pre, action, signedScope, submittedEffects, premises)`. The current direct parser preserves agreement ID, asset scale and authenticated predecessor on its AST, but the local Core preparer does not bind those three fields or an exact `/3` digest. It reports the candidate as `PreparedUnqualified`. The Core/5 tags below are proposed typed constructors. No caller Boolean can establish authentication or signature validity.

The local intent object carries the proposed version label `moriarty-intent/3`. That label is a type discriminator only. Source/6 lowering does not encode canonical `/3` bytes or calculate their digest, and no verifier consumes this object as a signed authorization.

| Source/6 field or form | Core/5 field or constructor | Rule |
| --- | --- | --- |
| `profile`, `agreement`, `domain`, `settlement` | Proposed `Version(Source6,Core5)`, `AgreementId`, `DomainId`, `AssetId(scale)` | The parser retains all four. The current local Core preparer does not bind `AgreementId` or scale; these remain named unverified bindings. |
| `selected … source_hash … digest` | Proposed `SelectedProgram(actionId,sourceHash,policyDigest)` | The current Core field named `programId` contains the selected **action ID**, not the agreement ID. Binding all three to a signed statement remains unverified. S0 uses exact action IDs `TransferLiteralFee` and `RepayAccrualFirst`, matching the signed and submitted constructor. Any other action ID rejects `SOURCE6_PROFILE_UNSUPPORTED`. |
| `signer … key`, `nonce`, `pre_head`, `valid` | `SignedScope(signer,keyRef,replayKey,preHead,roundLo,roundHi)` | `replayKey=(domain,signer,nonce)`; the exact digest and verifier are external typed premises. |
| `signed_action` | `SignedAction(TransferLiteralFee | RepayAccrualFirst)` | Fix owner/payer, recipients, fee recipient, obligation ID and quantities under the signature. The submitted action must match this signed action exactly. |
| `gross_cap`, `fee_cap`, `net_floor` | `Bounds(grossCap,feeCap,netFloor)` | Bind all three to the signed scope; authority uses gross debit. |
| `failure success_only` and six explicit empty/none fields | `FailurePolicy(SuccessOnly)`, empty observation, disclosure, retained effect and duty, no delegation or recovery | The current parser rejects a nonempty source variant at formation with `SOURCE6_SHAPE`. A separately constructed typed Core/5 requested outcome reaches `S0_FAILURE_UNSUPPORTED` at Failure. The Source/6 parser cannot express that typed hostile input. No accepted fee-bearing failure exists. |
| `authenticated head`, `predecessor`, `round` | `PreHead`, `Predecessor`, `CurrentRound` | Must be authenticated against the same snapshot; current head comparison is atomic with replay consumption. |
| `balance`, `allowance`, `obligation`, `replay`, `work_remaining`, `work_spent` | `BalanceCell`, `AllowanceCell`, `ObligationCell`, `ReplayCell`, `WorkCell` | These are read cells. Obligation binds debtor, creditor, asset, principal, accrued, outstanding and status. Both work counters come from the authenticated snapshot; lowering never resets spent work. |
| `transfer` | `TransferLiteralFee(owner,recipient,feeRecipient,value,fee)` | Owner=signer. Prepare debit gross, recipient credit value, and fee credit only when fee>0. |
| `repay` | `RepayAccrualFirst(obligationId,payer,amount,IdentityConversion)` | Payer=debtor=signer. Read bound creditor and asset from obligation; do not accept caller supplied substitutes. |
| `effects` | `PreparedEffects` comparison candidate | Order and values must equal the internally derived vector below. The supplied vector does not define the effect. In this provisional text profile, `use_replay` supplies the signed nonce; Core derives the domain/signer/nonce replay key. |
| `post_head` | `PostHead` candidate | Must be a valid authenticated successor under the external head-extension premise. |

For transfer, prepare `Debit(owner,v+f)`, `Credit(recipient,v)`, optional `Credit(feeRecipient,f)` when `f>0`, `UseAllowance(owner,v+f)`, `UseReplay(key)`, `AdvanceHead(pre,post)` in exactly that order. The optional line is present iff `f>0`; a zero-valued fee line rejects. Require `v>0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, sufficient owner balance and remaining allowance, and no overflow in any receiver or spent counter. Debit and credits conserve the same nominal asset. Preserve gross effects even though balance changes could be netted. The local wrapper checks Stage first, then requires the transfer in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

For repay, require one Outstanding obligation, `0<n≤outstanding`, matching asset, identity conversion `(mantissa=1,scale=0,rounding=none)`, `feeCap=0`, `netFloor=0`, sufficient payer balance and allowance, and no overflow in the creditor or spent counter. Let `da=min(n,accrued)` and `dp=n−da`. Prepare `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(p−dp,a−da,p+a−n,status')`, `UseAllowance(payer,n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. `status'` is Settled iff outstanding becomes zero. Every unlisted authenticated cell is unchanged. A debt reduction without the creditor credit rejects. The local wrapper checks Stage first, then requires the repayment in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

## Admission and observation

Core/5 applies `stage → intent → effect → authority → history → failure`. `stage` checks version, selected program, typed cells and authentication premises. `intent` checks exact signed scope, validity, endpoints, alias policy and all signed nominal caps and floors. `effect` computes and compares the complete ordered vector and post cells. `authority` checks signer, allowance and work budget. `history` compares the current head and unused replay key and verifies successor binding. `failure` accepts only terminal success with empty retained effects and duties. The first failing judgment returns `(judgment,code,diagnosticWork)` without a published post-state or effects. Within-judgment code spelling and precedence remain W-D3 choices; provisional codes are in `s0-implementation-contract.md`.

The result shape is `Core5Observation(pre,action,signedScope,preparedEffects,post,remainingDuty,remainingWork,replay,preHead,postHead,phase,judgment,code)`. Accepted S0 success contains complete effects, consumption, writes, one post-head and empty duty. Atomic rejection contains the first judgment/code and diagnostic work, with `post`, published effects and post-head absent. Snapshot authentication, exact signature verification, head extension and ledger compare-and-consume are typed external premises; failure or absence rejects. Source/6 grammar acceptance alone does not imply admission.

## Boundary with earlier and later forms

Source/5 `profile`, declarations, `action`, expression and `emit` forms have no automatic injection into this S0 stage. A future migration must map every legacy field, selected Core/4 program, authenticated state, signed digest and complete effect obligation, then prove the old and new observations equivalent on the stated domain. `Repay` in Core/4 consumes a separate Transfer; Core/5's `RepayAccrualFirst` is one funded stage. All old Source/5 and Core/4 behavior remains historical until such a mapping is demonstrated.

The eight MIL/4 first families and all later profiles reject from this S0 grammar with `SOURCE6_PROFILE_UNSUPPORTED` or `SOURCE6_UNKNOWN_TAG` at formation. A later full Source/6 grammar must add typed productions and Core/5 constructors per family. General Φ₁, uncertified Ω, division, rounding, mint, reserve, foreign evidence, accepted failures and recovery are outside S0. This document does not turn those forms into generic records or strings.

```

## experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf

sha256: `5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e`

```ebnf
(* ISO 14977 EBNF; provisional Source/6 S0 presentation syntax only.
   This is not the /3 signed-intent byte codec. The exact profile string is
   moriarty-financial-agreement-source/6. Whitespace, comments, identifiers,
   integers and strings use lexical.md. New words and the .. interval token
   below are profile-local and do not extend Source/5.
   All productions are closed; unknown fields and duplicate fields reject.
   The companion contract defines nominal typing, caps and Core/5 mapping. *)

source = header, agreement, ? end of file ? ;
header = "profile", '"moriarty-financial-agreement-source/6"', ";" ;
agreement = "agreement", identifier, "{", domain, settlement,
            selected, intent, authenticated, submitted, "}" ;
domain = "domain", identifier, ";" ;
settlement = "settlement", identifier, "scale", integerToken, ";" ;
selected = "selected", identifier, "source_hash", stringToken,
           "digest", stringToken, ";" ;

intent = "intent", "{", "signer", identifier, "key", stringToken, ";",
         "nonce", stringToken, ";", "pre_head", stringToken, ";",
         "valid", integerToken, "..", integerToken, ";",
         "gross_cap", integerToken, ";", "fee_cap", integerToken, ";",
         "net_floor", integerToken, ";", "failure", "success_only", ";",
         "signed_action", ( transfer | repay ),
         "observations", "empty", ";", "disclosures", "empty", ";",
         "retained_effects", "empty", ";", "retained_duties", "empty", ";",
         "delegation", "none", ";", "recovery", "none", ";", "}" ;

authenticated = "authenticated", "{", "head", stringToken, ";",
                "predecessor", stringToken, ";", "round", integerToken, ";",
                "balance", identifier, integerToken, ";",
                "balance", identifier, integerToken, ";",
                [ "balance", identifier, integerToken, ";" ],
                "allowance", identifier, "remaining", integerToken,
                "spent", integerToken, ";",
                [ obligation ], "replay", ( "unused" | "consumed" ), ";",
                "work_remaining", integerToken, ";",
                "work_spent", integerToken, ";", "}" ;
obligation = "obligation", identifier, "{", "debtor", identifier, ";",
             "creditor", identifier, ";", "asset", identifier, ";",
             "principal", integerToken, ";", "accrued", integerToken, ";",
             "outstanding", integerToken, ";", "status", "outstanding", ";",
             "}" ;

submitted = "submit", ( transfer | repay ), "effects", "{", effectVector, "}",
            "post_head", stringToken, ";" ;
transfer = "transfer", "from", identifier, "to", identifier,
           "fee_to", identifier, "value", integerToken,
           "fee", integerToken, ";" ;
repay = "repay", "obligation", identifier, "payer", identifier,
        "amount", integerToken, "conversion", "identity", ";" ;
effectVector = transferEffects | repayEffects ;
transferEffects = debit, credit, [ credit ], useAllowance, useReplay, advanceHead ;
repayEffects = debit, credit, setObligation, useAllowance, useReplay, advanceHead ;
debit = "debit", identifier, integerToken, ";" ;
credit = "credit", identifier, integerToken, ";" ;
setObligation = "set_obligation", identifier, "principal", integerToken,
                "accrued", integerToken, "outstanding", integerToken,
                "status", ( "outstanding" | "settled" ), ";" ;
useAllowance = "use_allowance", identifier, integerToken, ";" ;
useReplay = "use_replay", stringToken, ";" ;
advanceHead = "advance_head", stringToken, stringToken, ";" ;

identifier = ? ASCII identifier token in lexical.md ? ;
integerToken = ? canonical unsigned decimal token in lexical.md ? ;
stringToken = ? JSON string token in lexical.md ? ;

```

## experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts

sha256: `f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7`

```typescript
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

## experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts

sha256: `855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda`

```typescript
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

## experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts

sha256: `1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8`

```typescript
/** Provisional Source/6 to Core/5 local preparation. This module cannot admit a ledger stage. */
import { prepareMil4S0, type S0PreparedUnqualified, type S0Rejected } from './mil4-s0-core-v5.ts';
import {
  parseAndLowerSource6, Source6Error,
  type Source6Ast, type Source6Lowered,
} from './financial-agreement-source-v6-frontend.ts';

export type Source6S0Outcome =
  | { status: 'SourceRejected'; code: string; offset: number; publishedPost: null; publishedEffects: null }
  | { status: 'CoreRejected'; ast: Source6Ast; rejection: S0Rejected }
  | {
      status: 'PreparedUnqualified'; ast: Source6Ast; candidate: S0PreparedUnqualified;
      unverifiedBindings: readonly ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'];
    };

function sameAction(a: Source6Ast['intent']['signedAction'], b: Source6Ast['submitted']['action']): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === 'Transfer' && b.kind === 'Transfer') {
    return a.from === b.from && a.to === b.to && a.feeTo === b.feeTo
      && a.value === b.value && a.fee === b.fee;
  }
  return a.kind === 'Repay' && b.kind === 'Repay'
    && a.obligation === b.obligation && a.payer === b.payer && a.amount === b.amount
    && a.conversion === b.conversion;
}

/** Parse and prepare one S0 stage without asserting source authentication or ledger acceptance. */
export function prepareSource6S0Unqualified(source: string): Source6S0Outcome {
  let lowered: Source6Lowered;
  try {
    lowered = parseAndLowerSource6(source);
  } catch (error) {
    if (!(error instanceof Source6Error)) throw error;
    return {
      status: 'SourceRejected', code: error.code, offset: error.offset,
      publishedPost: null, publishedEffects: null,
    };
  }
  const { ast, state, intent, submittedEffects, proposedPostHead } = lowered;
  const result = prepareMil4S0(state, intent, submittedEffects, proposedPostHead);
  if (result.status === 'Rejected' && result.judgment === 'stage') {
    return { status: 'CoreRejected', ast, rejection: result };
  }
  if (!sameAction(ast.intent.signedAction, ast.submitted.action)) {
    return {
      status: 'CoreRejected', ast,
      rejection: {
        status: 'Rejected', judgment: 'intent', code: 'S0_INTENT_SCOPE',
        diagnosticWork: 1, publishedPost: null, publishedEffects: null,
      },
    };
  }
  return result.status === 'Rejected'
    ? { status: 'CoreRejected', ast, rejection: result }
    : {
        status: 'PreparedUnqualified', ast, candidate: result,
        unverifiedBindings: ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'],
      };
}

```

## experiments/moriarty-language/formal/mil4/s0-implementation-contract.md

sha256: `a7f564e3c0a72f549b09849fdd7d3de5e0b0ff1b74ec34261bd9a229acf9a788`

```text
# S0 implementation contract, provisional

**Status:** executable prototype contract. It fixes choices for isolated K and Quint work, but does not adopt MIL/4, close W-D0–W-D4, or qualify a native/ledger path. The [semantic contract](semantics-contract.md), [projection](projection.md) and [independent cases](s0-discriminators.md) control the intended behavior.

## Version and input

Use a new `Source/6` and `Core/5` identity. S0's signed intent envelope remains a separate proposed `/3` encoding. A stage carries one selected program, domain, signer, settlement asset, signed nonce, signed pre-head, validity round interval, fixed recipient and fee recipient, gross and fee caps, net floor, and a complete submitted effect vector. Repayment also carries an authenticated obligation with debtor, creditor, settlement asset, principal, accrued, outstanding and status. A K input may use symbolic terms rather than JSON, but it must retain these fields as typed values or explicit premises.

The signer is the debited owner/payer in S0. Delegation is absent. The replay key is `(domain, signer, signed nonce)`. A second intent may repay the same obligation with a distinct nonce and current head. Snapshot-to-head authentication, signature verification of the exact intent digest, and atomic ledger compare-and-consume are named premises. An unavailable premise rejects. A caller Boolean does not establish a premise.

## Provisional Source/6 lowering

The Source/6 parser preserves the agreement ID, selected action ID, settlement scale, authenticated predecessor, signed action, submitted action, and explicit empty failure/observation/duty fields in a typed stage wrapper. The selected `digest` is the policy digest; the `/3` signed-intent digest is not defined by Source/6 and must not be fabricated during lowering. The wrapper checks local Stage shape before reporting a changed `signed_action`/`submit` pair at Intent; it withholds any candidate produced during that local check. `work_remaining` and `work_spent` are both authenticated cells; neither counter may be synthesized as zero. The Source/6 `use_replay` string names the signed nonce in this provisional presentation. Core/5 derives the composite replay key from domain, signer and nonce and compares that complete line. This textual convention remains a W-D3 candidate, not final signed bytes.

## Canonical effects and arithmetic

Transfer requires `v>0`, `f≥0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, and balances/allowance sufficient for gross `v+f`. The candidate first profile requires owner, recipient and fee recipient to be pairwise distinct. This explicit narrow-profile rejection avoids ambiguous alias writes until a later alias policy is selected. Ordered effects are `Debit(owner,v+f)`, `Credit(recipient,v)`, then `Credit(feeRecipient,f)` if `f>0`; omit the zero-fee line. Preserve gross debit before deriving net cell deltas. Consume allowance remaining and increase spent by `v+f`.

Repayment requires `0<n≤outstanding`, `outstanding=principal+accrued`, status Outstanding, payer=debtor=signer, creditor from the authenticated obligation, matching settlement asset, and identity conversion. Set `da=min(n,accrued)`, `dp=n−da`, `accrued'=accrued−da`, `principal'=principal−dp`, `outstanding'=principal'+accrued'`. Ordered effects are `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(...)`, `UseAllowance(n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. Transfer has the corresponding allowance, replay and head lines. No impairment substitutes for payment.

Use checked UInt128 for balances, allowances and each intermediate sum. Apply the current source/kernel `2^127−1` bound to S0 nominal amounts and liability caps; do not mislabel every lifecycle state field as signed-width. Subtraction underflow and credit overflow reject. S0 has no division, rounding, reserve, mint or accepted retained-effect failure.

## Judgment order and result

Apply `stage → intent → effect → authority → history → failure` in that order. A candidate Core rejection uses `(firstJudgment, stableCode, diagnosticWork=1)` and publishes no post-state or effects. The one diagnostic unit is an abstract observation, not runtime gas; Source/6 formation errors are outside this count. Exact wire code spelling remains provisional under W-D3. `stage` binds the authenticated balance, allowance and obligation cells required by the signed action, including debtor and creditor identity for repayment. `intent` checks fixed signed scope, validity, positive nominal action and caps/floor before diagnosing a supported direct-Core endpoint alias. `effect` checks numeric range and obligation arithmetic before comparing the complete ordered submitted vector with preparation. `authority` checks signer, allowance and work budget. `history` checks pre-head and replay key. `failure` accepts only terminal success with empty retained effects and duties.

| Condition | First judgment | Provisional code |
| --- | --- | --- |
| Unsupported source/Core/profile or malformed typed state | stage | `S0_STAGE_UNSUPPORTED` |
| Required typed premise unavailable or does not bind the exact intent, state, round and requested outcome | stage | `S0_STAGE_PREMISE` in K and Quint; TypeScript reports it only in the optional local-stipulation comparison and remains unqualified on success |
| Changed signed endpoint, nonce, bounds or invalid validity round | intent | `S0_INTENT_SCOPE` |
| Endpoint alias in this first profile | intent | `S0_INTENT_ALIAS` |
| Submitted vector or derived post-state differs from preparation | effect | `S0_EFFECT_MISMATCH` |
| Insufficient balance, numeric overflow or invalid obligation arithmetic | effect | `S0_EFFECT_RANGE` |
| Authenticated obligation debtor differs from signed signer/payer | stage | `S0_STAGE_UNSUPPORTED` |
| Submitted payer differs from the signed payer | intent | `S0_INTENT_SCOPE` |
| Allowance or work budget exceeded after typed payer binding | authority | `S0_AUTH_SCOPE` |
| Stale pre-head, consumed replay key, or wrong stipulated successor | history | `S0_HISTORY_STALE`, `S0_HISTORY_REPLAY`, or `S0_HISTORY_SUCCESSOR` |
| Nonempty retained effect/duty or unselected failure branch | failure | `S0_FAILURE_UNSUPPORTED` |

This table is an implementation discriminator, not an accepted canonical diagnostic schedule. Compile and typecheck results may establish syntax only. Semantic traces, proofs, native qualification and ledger readback remain separate evidence.

```

## experiments/moriarty-language/formal/mil4/wire/SPEC.md

sha256: `644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1`

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

## experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md

sha256: `e51cd539699bd0afd21f0c81aecd0af39bf12e24e8c0a50fba88f8929e9c1129`

```text
# W-D2F observed field map and missing bindings

Read-only repository observations, 2026-09-30. This is a design sprint, with no
consumer implementation, signature verifier, authenticated registry, or ledger
acceptance. Every proposed check below is specified-only unless explicitly
identified as an existing module behavior. No normative W-D2 choice is made.

## Observed interfaces

The existing [wire schema](../wire/SPEC.md) signs fields 1–35. Its decoder checks
canonical byte shape, not ownership, selected program, state or effects.
[Source/6 AST](../../../src/successor/financial-agreement-source-v6-frontend.ts:119)
preserves additional presentation claims. Its
[lowerer](../../../src/successor/financial-agreement-source-v6-frontend.ts:324)
constructs the existing Core intent/state, and deliberately does not compute an
authorization digest. [Core/5](../../../src/successor/mil4-s0-core-v5.ts:131)
derives effects and post-cells internally before comparing supplied effects,
but accepts caller state without authentication. Its optional local stipulation
is a matching caller tuple, not evidence of an external premise.
The [Source wrapper](../../../src/successor/mil4-s0-source-v6.ts:13) names four
unverified bindings: agreement ID, selected program, scale and predecessor.
[W-D2E](../effect-wire/SPEC.md) adds a supplied effect-image encoder and field 26
equality check, but does not authenticate or semantically derive that image.

`AST.programId` is the identifier after `agreement`; `AST.selected.actionId`
is the selected action. `Core.intent.programId` equals the latter. They are
different nominal roles despite the overloaded property name. The existing
W-D2E fixture uses `agreementId=Agreement`, `actionId=Action`, and
`coreProgramId=TransferLiteralFee` or `RepayAccrualFirst`. It contains no
Source/6 document or source-to-wire binding. It must not be promoted to a
positive full-consumer fixture by treating `actionId` as ignorable.

There are two distinct layers. Literal `wire.agreementId == AST.programId`
is available as an inspection comparison; B01 authenticated agreement-instance
binding remains unavailable. Literal `wire.actionId == AST.selected.actionId`
is also available; B04's authenticated action/Core-program relationship remains
unselected. Comparing these strings does not require a registry or prove one.
Scale, predecessor, claimed sourceHash and claimed policyHash similarly permit
literal comparisons at the AST boundary while their meanings/authentication
remain unverified. Core's loss of a field does not erase the AST comparison.

## Complete signed field map

“Direct” means an existing scalar/constructor can be compared for literal
equality. It does not mean that scalar has been authenticated. “Absent” means
the downstream type has no slot. “Unselected” means the correspondence or hash
image has not been adopted and must not be supplied by a guessed constant.

| Tag | Signed field | Source/6 AST / lowering | Core/5 | W-D2E image | Missing binding / proposed behavior |
| --- | --- | --- | --- | --- | --- |
| header | schemaVersion=/3 | Source profile is /6; lowerer chooses /3 discriminator | intent.version=/3 | independent effects/1 header | V-S0-01 relates this fixed /3 header to Source/6, Core/5 and effects/1; no raw cross-vocabulary equality or /4 fallback |
| 1 | profile=s0-provisional/1 | /6 closed S0 grammar | Core/5 S0 selection | operationKind transfer/repayment | Selected local V-S0-01 tuple, checked at M/tag1; wire.profile != AST.profile is expected vocabulary difference |
| 2 | domain | ast.domain; state.domain; intent.domain | checks state/intent equality | root.domain and replay components | Direct equality; domain membership and ledger network mapping unverified |
| 3 | agreementId | ast.programId retained | absent | absent | Direct literal agreementId==AST.programId comparison available; B01 authenticated agreement-instance binding unavailable; never map to Core.intent.programId |
| 4 | stageId | absent | absent | absent | B02: authenticated stage identity and uniqueness unavailable |
| 5 | episodeId | absent | absent | absent | B03: authenticated episode/history identity unavailable |
| 6 | actionId | Direct equality with ast.selected.actionId available; distinct wire/coreProgramId relation unselected | programId contains Source selected action | absent | Direct literal action comparison available; B04 authenticated selected action/program relation unselected; W-D2E Action differs from Source's supported selected IDs |
| 7 | sourceVersion=6 | exact profile /6 | sourceProfile=/6 | absent | V-S0-01 explicitly relates integer6 to Source profile /6; no hash/lowering authenticity follows |
| 8 | sourceHash | ast.selected.sourceHash, opaque string; copied to intent | only validates opaque string | absent | Direct literal claim equality available; B05 canonical source hash image and source-to-selection verification undefined |
| 9 | coreVersion=5 | lowerer chooses Core/5 | state.core, intent.core | core=/5 | V-S0-01 relates integer5 to Core /5; B06 additionally required for exact selected code |
| 10 | coreProgramId | Literal coreProgramId==selected.actionId comparison available | programId accepts only TransferLiteralFee / RepayAccrualFirst for corresponding kind | operationKind only | Equal literal still leaves authenticated B04 action/program and B06 exact-code correspondence unavailable |
| 11 | coreHash | absent | absent | absent | B06: canonical Core program bytes/hash and selected program verifier undefined |
| 12 | policyHash | ast.selected.policyDigest, opaque; copied to intent.policyDigest | only opaque validation | absent | Direct literal claim equality available; B07 exact policy image and authenticated digest relationship undefined |
| 13 | signer | ast.intent.signer; Source owner/payer constraint | signer to debited owner/debtor | Debit account, allowance owner, replay signer | Direct equality; B08: nominal signer/key ownership and key rotation unavailable |
| 14 | keyScheme=schnorr_bip340 | no scheme field | no scheme field | absent | Exact wire literal is schnorr_bip340; B09 preprocessing/verifier remains unselected |
| 15 | signerKey | ast.intent.keyRef opaque string; copied to intent.keyRef | only opaque validation | absent | B08/B09: keyRef to exact x-only key mapping, curve validity, and authorized ownership unavailable |
| 16 | asset | ast.settlement.asset; state/intent asset | obligation asset equality; Debit/Credit asset | root.asset; obligation footprint asset; effect assets | Direct equality; B10 anchors here at asset/tag16, before scale/tag17; asset identity/unit registration and settlement binding unavailable |
| 17 | scale | ast.settlement.scale decimal string 0..18, retained only | absent | root.scale integer 0..38 | Direct integer equality available in 0..18 intersection; B10 meaning/authentication unverified; never rescale |
| 18 | preHead | ast.intent.preHead and ast.authenticated.head; lowerer state.head/intent.preHead | checks equality at history | root.preHead and AdvanceHead predecessor | Direct equality; B11: snapshot-to-this-head authentication unavailable |
| 19 | predecessor | ast.authenticated.predecessor retained; direct literal equality available | dropped; no predecessor slot | absent; AdvanceHead.predecessor is current preHead | Direct AST claim equality available; B12 actual authenticated prior-head relation unavailable; never equate field19 with AdvanceHead.predecessor |
| 20 | nonce | ast.intent.nonce opaque; lowerer composite replay tuple | replay key=(domain, signer, nonce) | typed replay tuples and histories | Direct equality on exact 64 lowercase hex text; B13: authenticated replay-status fact/unused predicate and durable uniqueness unavailable |
| 21 | validFrom | ast.intent.notBefore uint128 | intent.notBefore, compares state.round | round only | Direct exact integer equality in UInt64 intersection; B11: round authentication unavailable |
| 22 | validUntil | ast.intent.notAfter uint128 | intent.notAfter, closed round interval | round only | Same as validFrom; no clock-time conversion |
| 23 | grossCap | ast.intent.grossCap | intent.grossCap; gross Debit limit | amounts; pre/post allowance | Direct exact equality; B11/B14 authenticate grant/counters; never derive authority from net delta |
| 24 | feeCap | ast.intent.feeCap | intent.feeCap; repay requires zero | fee Credit when positive | Direct exact equality; zero-fee recipient still bound in action and footprint |
| 25 | netFloor | ast.intent.netFloor | transfer value floor; repay requires zero | effect amounts | Direct exact equality; no fee/net reinterpretation |
| 26 | effectCommitment | absent | absent; prepare returns internally derived vector/post | SHA256 of complete image | Rederive Core effects/post, derive complete footprint from same pre/post, then hash; supplied image is never the preparation oracle |
| 27 | failurePolicy=atomic-reject-terminal-success | ast.intent.failure=success_only | TerminalSuccess requested outcome in wrapper | fixed premise requirement; no failure variant | Selected local relation F-S0-01 maps these different literals for closed S0 only; not direct string equality or normative W-D2 adoption |
| 28 | supplyChanges=[] | no supply-change syntax | no mint/burn S0 operation | no supply-change field | Closed S0 supports empty only; existing wire decoder rejects nonempty |
| 29 | observations=[] | ast.intent.observations=empty | dropped; no observation slot | absent | Existing closed syntax rejects nonempty; no external observation claim |
| 30 | disclosures=[] | ast.intent.disclosures=empty | dropped | absent | Existing closed syntax rejects nonempty; no new disclosure mechanism |
| 31 | retainedEffects=[] | ast.intent.retainedEffects=empty | requestedOutcome.retainedEffects empty | absent | Closed terminal success; never fabricate failure effects |
| 32 | retainedDuties=[] | ast.intent.retainedDuties=empty | requestedOutcome.retainedDuties empty | absent | Closed terminal success; no retained duty execution |
| 33 | delegation=none | ast.intent.delegation=none | absent | absent | No delegated signer; fail closed on extension |
| 34 | recovery=none | ast.intent.recovery=none | absent | absent | No recovery rule; fail closed on extension |
| 35 | operation | signedAction; submitted.action must match | Transfer or Repay derived from signedAction | operationKind plus derived lines | All nested fields below must be bound before preparation |

## Complete nested operation map

| Signed operation field | Source / Core relationship | Required check |
| --- | --- | --- |
| transfer.kind | TransferLiteralFee / Transfer | Exact constructor; no generic effect record substitutes |
| transfer.owner | signedAction.from, submitted.from, intent.signer | All equal; Core does not retain from independently |
| transfer.recipient | signedAction.to, submitted.to, intent.recipient | Exact account equality including zero-value aliases prohibition |
| transfer.feeRecipient | signedAction.feeTo, submitted.feeTo, intent.feeRecipient | Exact equality even fee=0; required third balance remains |
| transfer.amount | signedAction.value, submitted.value, intent.amount | Exact nominal integer; >0 |
| transfer.fee | signedAction.fee, submitted.fee, intent.fee | Exact integer; >=0; Core checks fee cap and gross |
| repayment.kind | RepayAccrualFirst / Repay | Exact supported constructor |
| repayment.obligationId | signedAction.obligation, submitted.obligation, intent.obligationId, state obligation.id | All equal; no replacement obligation |
| repayment.payer | signedAction.payer, submitted.payer, intent.signer, debt.debtor | All equal; funded Debit |
| repayment.debtor | authenticated obligation.debtor; no separate signedAction debtor field | Exact payer/debtor/signer equality plus B11 snapshot authentication |
| repayment.creditor | authenticated obligation.creditor; Core derives bound creditor | Exact signed creditor to authenticated creditor equality; never overwrite snapshot creditor from wire |
| repayment.amount | signedAction.amount, submitted.amount, intent.amount | Exact nominal integer; >0 and <=outstanding |
| repayment.allocation=AccrualFirst | no explicit Source allocation; selected builtin semantics | B04/B06 must bind selected builtin; Core applies da=min(n, accrued), dp=n-da |
| repayment.conversion=identity | signedAction.conversion, submitted.conversion; no Core conversion slot | Exact selected Source literal and corresponding closed builtin; no invented rate or scale conversion |

## State, effect-image and footprint completeness

| Input / result | Image field | Required producer derivation and missing evidence |
| --- | --- | --- |
| state.core/domain/asset | root.core/domain/asset | Copy from bound Core input; compare signed compatible versions/domain/asset |
| state.round | root.round | Copy exact authenticated round, not validFrom; B11 |
| state.head, result.candidatePost.head | root.preHead/successor and AdvanceHead | Copy exact values; B11/B12/B15 |
| intent.kind | root.operationKind | Transfer->transfer or Repay->repayment, only existing closed constructor relation |
| asset metadata | root.scale | No Core slot; requires adopted B10 binding. Cannot infer from amounts or assume 0/2 |
| result.effects | effects ordered list | Use internally derived Core result, not caller image; preserve gross Debit and zero-fee omission |
| ordered state balances and post balances | footprint.balances | Exact three transfer rows or two repay rows; IDs and before/after values from matching positions; no missing receiver initialization |
| ordered allowances | footprint.allowances | Exactly signer allowance remaining/spent before and after; B11/B14 |
| ordered obligation(s) | footprint.obligations | Repay exactly one full id/debtor/creditor/asset/status/value row; transfer none; pre/post identity unchanged |
| workRemaining/workSpent pre/post | consumption work counters | Copy both authenticated pre-values and Core post-values; no spent reset; one unit debit is existing Core behavior |
| state.consumedReplay / candidatePost.consumedReplay | replayBefore/replayAfter | Preserve complete histories in order, append exact signed tuple; B11/B13/B16 |
| result.requiredPremises | requiredPremises | Fixed required list; requirements are not verified evidence |

Source/6 replay syntax only claims `unused` or `consumed` for the selected key.
Lowering fabricates the corresponding local representation [] or [selected
key] from that claim; it does not read complete ledger replay history. W-D2E
commits complete supplied arrays (cap16). A positive Source-based consumer with
nonempty unrelated history requires an adopted projection or a richer snapshot
transport. Do not discard authenticated history to make [] match. This is B16.

## Missing authenticated bindings

| ID | Required material / unavailable implementation |
| --- | --- |
| B01 | Agreement instance registry binds wire agreementId to Source agreement identity, selected code and authenticated cells |
| B02 | Stage identity and uniqueness, bound to same domain/agreement/episode/snapshot |
| B03 | Episode identity and continuing history, bound to same agreement |
| B04 | Adopted relation among signed actionId, signed coreProgramId, Source selected action and operation constructor |
| B05 | Selected Source code image, exact hash algorithm/domain separation, and authenticated sourceHash correspondence |
| B06 | Canonical selected Core code image, exact hash, source-to-Core lowering correspondence and builtin identity |
| B07 | Exact policy bytes/hash and policy-to-selected-source/intent relationship |
| B08 | Nominal signer/keyRef to x-only signerKey authorization, validity and key-rotation/replay policy |
| B09 | Exact signature-message preprocessing and verifier for selected scheme remain unselected; canonical-wire SHA256 content digest is not an adopted raw signature message |
| B10 | Asset identity, atomic-unit scale and settlement registration; approved scale relation |
| B11 | One authenticated snapshot binds all required balance/allowance/obligation/work/replay cells, head, domain, instance and round |
| B12 | Signed predecessor equals authenticated prior-head linkage of current preHead; field19 is not the current head |
| B13 | Authenticated replay-status evidence for full domain/signer/nonce tuple, require unused, and durable uniqueness policy |
| B14 | Grant/allowance/work authority whose remaining and spent cells bind to same signer, agreement and snapshot |
| B15 | Valid independently selected successor head and head-extension evidence; successor must not depend on this authorization digest |
| B16 | Source replay-claim to complete authenticated replay-history projection without omission |
| B17 | Atomic ledger compare-and-consume/apply binding the exact digest, snapshot, effects, allowances, replay and successor |

Core only names B09/B11/B15/B17 premises; its success does not discharge them.
The Source wrapper additionally names B01/B04/B10/B12 as unverified. The rest
are gaps exposed by complete field mapping. Neither an effect hash nor a caller
Boolean, matching tuple, callback promise, or opaque receipt authenticates them.

The current Source document embeds `source_hash` and `digest` claims. Defining
sourceHash as SHA256 of the entire document that contains sourceHash introduces
a self-reference. There is no selected canonical code-only image or exclusion
rule here. Defining a new masked-document hash, canonical AST serialization,
policy image, Core image, or registry would be a separate reviewed design.

## Compatibility boundaries and exact planned rejections

Wire IDs allow colons, slashes, hyphens and initial digits. Source identifiers
are ASCII initial letters followed by alphanumerics/underscore, with reserved
words excluded. Core IDs allow initial letters and alphanumerics/dot/hyphen/
underscore. No encoding, escaping, renaming or case folding has been adopted.
Source-valid IDs form the proposed local intersection. Wire/effect scale38
versus Source scale18 and UInt64 wire rounds versus UInt128 Source/Core rounds
must be checked before projection; no clamping or modulo arithmetic.

Reading B is the single repaired schedule: F-wire, F-source, D domain sweep,
M tag-ordered literal/binding checks, H complete-history bound, C Core, I image,
E field26, and optional separate L ledger consumption. B01–B16 are prerequisites
for local semantic comparison; B17 is explicitly pending after successful E.
It is never an earlier local-comparison rejection. These are adapter phases,
not an adoption of W-D3's canonical judgment precedence.

### Exact D domain sweep

Visit fields in tag order: domain, agreementId, stageId, episodeId, actionId,
sourceHash, coreProgramId, policyHash, signer, asset, scale, preHead,
predecessor, nonce, validFrom, validUntil; then nested operation fields in the
wire variant's order. Mapped Source values are checked before the corresponding
decoded wire value; Source's signed action before its submitted action.
For stageId/episodeId no Source slot exists, so check only the decoded identifier
against the proposed common identifier subtype. Do not synthesize a Source slot.

Identifier subtype is `[A-Za-z][A-Za-z0-9_]{0,63}` with Source reserved words
excluded. Hash32 claim subtype is exactly 64 lowercase hex characters for
Source selected.sourceHash, selected.policyDigest, intent.preHead,
authenticated.head, authenticated.predecessor and intent.nonce. At preHead,
check intent.preHead before authenticated.head. Source keyRef remains an opaque
reference; do not require it to equal raw key hex or invent a conversion.
Scale comparison uses canonical Source decimal value and wire byte in 0..18;
validity values must be <=2^64-1. The wire decoder already enforces its own
primitive formats. After all signed/operation domains, visit authenticated.round
(UInt64), submitted.postHead (hash32), and AdvanceHead line predecessor then
successor in submitted line order (hash32). Thus symbolic h0/h1 Source claims
fail domain before any authentication check; valid but different heads proceed
to the literal/Core checks below. No Source offset is fabricated for adapter
errors: they carry field/sourcePath; SourceRejected alone preserves parser's
original UTF-8 offset.

D finishes completely before M starts. A malformed late hash or owner subtype
can precede missing B01. A domain-valid late value mismatch cannot precede B01.
This distinction is frozen in the expectation matrix's multi-defect controls.

### Exact M literal and binding order

At tag1 apply selected local V-S0-01 (the exact tuple in PLAN.md), not literal
wire.profile==AST.profile. Scan tags 1–35. At each tag, check direct Source literals first, authenticate
any provider facts anchored there, then compare **only this tag's signed value**
to the retained authenticated facts. An anchor authenticates provider integrity,
scope and availability now; it does not compare later signed fields early.
Authentication is never deferred. Later signed equality is performed at that
field's tag, and every equality must pass before C can execute.
Binding anchors are tag3 B01; tag4 B02; tag5 B03; tag6 B04;
tag8 B05; tag11 B06; tag12 B07; tag13 B08; tag14 B09; tag16 B10; tag18 B11;
tag19 B12; tag20 B13; tag23 B14. After tag35 and its nested fields, perform
the unsigned Source snapshot tail, then B15 at successor, then B16 at replayBefore. Retained contexts are immutable verifier
outputs, not caller claims or booleans. Facts associated with already processed
fields must agree with that verified context at the anchor; equality with future
signed fields waits for those fields. A repeated literal does not waive any
provider authentication or field comparison.
Missing B10 reports field asset, not scale. B17 is not an M anchor.

| Anchor | Authenticate and retain at anchor | Signed comparisons at own tag / unsigned tail |
| --- | --- | --- |
| B01 agreementId/3 | Authentic agreement-instance record, scope to domain; compare current agreement identity | Retain declared stage/episode/action/code/asset associations; compare future fields at4/5/6/10/16 |
| B02 stageId/4 | Authentic stage record and link to already verified agreement; compare stageId | Retain episode association; compare at5 |
| B03 episodeId/5 | Authentic episode/history record and prior agreement/stage links; compare episodeId | Retain relevant later head/history facts; check at their own loci |
| B04 actionId/6 | Authentic selected action manifest; current actionId and prior instance links | Retain Core ID, source/Core/policy image identities and operation constructor; compare at10/8/11/12/35 |
| B05 sourceHash/8 | Authentic selected Source artifact/image and compute retained sourceHash | Compare current sourceHash after authentication; do not treat wire mismatch as invalid artifact evidence |
| B06 coreHash/11 | Authentic selected Core artifact/image and source-lowering correspondence; retain computed Core hash | Compare current coreHash after authentication; Core ID was compared at10 |
| B07 policyHash/12 | Authentic policy artifact and prior selected-context links; retain policy hash and selected policy predicates | Compare current policyHash; test later caps/floor under adopted predicates at23/24/25 |
| B08 signer/13 | Authentic signer ownership/keyRef resolution and prior instance/domain links; compare current signer | Retain authorized x-only key; compare wire.signerKey at15, never at13 |
| B09 keyScheme/14 | Real verifier succeeds under later adopted preprocessing, using retained B08 authorized key and exact canonical record binding | Compare current exact scheme schnorr_bip340; wire.signerKey equality remains tag15. Passing B09 alone cannot establish it |
| B10 asset/16 | Authentic registered asset metadata and prior instance association; compare current asset | Retain registered scale; compare wire.scale at17, never at16 |
| B11 preHead/18 | Authenticate complete same-instance/domain/asset snapshot proof; retain head, predecessor, round, cells/work/history | Compare current preHead; signed predecessor19 and debtor/creditor35 later; unsigned Source round/cells/work at explicit tail |
| B12 predecessor/19 | Authenticate prior-head linkage fact and its already verified current-head scope | Compare current signed predecessor after verification |
| B13 nonce/20 | Authenticate replay-status fact, scoped to prior domain/signer/head; integrity accepts genuine unused or consumed status | Compare current nonce, then require unused; consumed -> W_D2F_REPLAY_CONSUMED at20, not EVIDENCE_INVALID; retain facts for B16 |
| B14 grossCap/23 | Authenticate grant/work authority fact and prior signer/instance/snapshot links | Test current grossCap and later feeCap/netFloor under adopted authority relation at23/24/25; Source allowance/work claims at unsigned tail; never equate cap with remaining counter |
| B15 successor tail | Authenticate head-extension fact and digest-independent selection under adopted protocol | Compare current Source proposed successor to retained authentic successor |
| B16 replay tail | Authenticate adopted complete-history projection and selected-key relation to retained snapshot | Compare Source selected replay claim and preserve complete retained history; never synthesize [] |

For B08/B10/B04, a valid provider fact with a different future signed key/scale/
Core ID is not an early evidence rejection. It produces FIELD_MISMATCH at15/17/10.
B09 must use the authenticated B08 key, because tag15 equality has not run yet.
A correctly authorized signature can bind a record containing inconsistent key
metadata; tag15 still rejects that metadata. No unselected message convention
is inferred by this scheduling rule. Invalid evidence that fails to authenticate
its declared provider fact remains EVIDENCE_INVALID at the anchor.

Direct literal mappings use the table above. Version/profile vocabularies use
V-S0-01; failure-policy vocabularies use F-S0-01, never raw cross-string equality. Source agreement and
selected action names are literal comparisons, independent of registry trust.
At coreProgramId, compare Source selected.actionId then lowered.intent.programId;
equal strings do not establish B04/B06. sourceHash/policyHash compare claims,
not a computed image; keyRef-to-key comparison requires B08's adopted reference
resolution and is never a guess based on matching opaque text. Missing Source
stage/episode slots require adopted B02/B03 wrapper evidence, not invented IDs.

Head precedence at tag18: wire.preHead versus AST.intent.preHead first, then
AST.authenticated.head; then authenticate B11's declared snapshot fact and compare
its retained authentic head to the current signed preHead. A valid proof of a
different head yields FIELD_MISMATCH/preHead at18, binding B11,
factPath B11.snapshot.head and sourcePath null; the Source literal checks already
passed, so authenticated.head is not the mismatching locus. A proof that does not authenticate
its declared snapshot fact yields EVIDENCE_INVALID/B11 at18. Keep these separate.
At tag19 compare the AST predecessor then verified snapshot prior-head linkage
under B12. AdvanceHead.predecessor always denotes current preHead, never field19.
Submitted successor-line differences are Core effect mismatches at C. An image
root/line discrepancy that survived C is an image mismatch at I. For nested
operations, compare Source signed action then submitted action; repayment
debtor/creditor additionally compare the authenticated obligation. Preserve
owner/payer/recipient/feeRecipient/amount/fee field names exactly as the wire
schema, rather than using Source aliases in reported field names.

After tag35/subfields and before B15/B16, compare **unsigned Source snapshot
claims** against retained authenticated facts in this exact order: round,
balances in Source row order (account then amount), allowance (owner, remaining,
spent), obligation (id, debtor, creditor, asset, principal, accrued, outstanding,
status), workRemaining, workSpent. Counts and order are exact. Report field and
sourcePath as `authenticated.<path>` for these unsigned claims, not a fictional
wire tag. These checks cannot be skipped because signed head/commitment matches.
Source's selected replay claim is compared at B16; Core receives complete verified
history under the adopted projection, not Source's unauthenticated [] shortcut.
B16 authenticates complete history and compares the selected replay-status claim:
Source consumed versus verified absent rejects FIELD_MISMATCH at the tail, field
and sourcePath authenticated.replay, factPath B16.selectedReplayStatus. A genuine
B13 consumed proof rejects W_D2F_REPLAY_CONSUMED at tag20 first; invalid B13 proof
rejects EVIDENCE_INVALID. Invalid B12/B13/B14 evidence needs only earlier tags
through18/19/22, respectively; the unsigned tail has not run at those anchors.
Invalid B16 evidence requires all35 fields, unsigned tail and B15 to pass.

F-S0-01 is explicitly selected for this local adapter: Source success_only
corresponds to wire atomic-reject-terminal-success only for closed S0 terminal
success with empty retained effects/duties and atomic unpublished rejection.
These literals differ and are never tested by string equality. Existing grammar/
codec rejects unsupported literals at formation; new failure branches require a
separate adopted relation. This local selection does not close normative W-D2.

Adapter D diagnostics use exact slots: intent.preHead -> field preHead,
sourcePath intent.preHead; authenticated.head -> field preHead, sourcePath
authenticated.head; authenticated.predecessor -> field predecessor, sourcePath
authenticated.predecessor; submitted.postHead -> field successor, sourcePath
submitted.postHead; submitted AdvanceHead line -> field effects[i].predecessor
or effects[i].successor and sourcePath submitted.effects[i].predecessor/successor.
The zero-based AdvanceHead index i is5 for fee-positive transfer and repayment,
and4 for zero-fee transfer. Each concrete expectation names its constructor;
there is no universal effects[5] slot.
For a wire-only subtype failure, sourcePath is null and inputPath names wire.field.
This names current/prior/proposed heads distinctly without inventing offsets.

### H history and I image bounds

After B01–B16 verify, check complete authenticated replayBefore count, then
the full count after appending the unused selected tuple, before C. If either
exceeds16, return `EffectDomainRejected/W_D2F_REPLAY_HISTORY_BOUND` with exact
field (`consumption.replayBefore` or `consumption.replayAfter`), count, cap16,
null published effects/post. At I, an otherwise shaped supplied replay array
exceeding16 gets the same explicit result, before generic W-D2E encoding.
Do not truncate, deduplicate, discard unrelated keys, or classify these as
authorization WireRejected. Example: pre16 unused keys -> post17 rejects at
replayAfter; pre17 -> replayBefore wins. B17 remains pending in both cases and
does not alter this local phase ordering.

At I validate/encode the derived side first, then supplied root/replay array
shape and replayBefore/replayAfter counts, then supplied codec shape/bounds,
then complete equality. EffectImageRejected carries imageSide=derived or
supplied; shaped supplied history overflow carries imageSide=supplied, with
replayBefore winning if both counts overflow. H authoritative history overflow
carries imageSide=derived. These are effect image errors, never WireRejected.

The proposed adapter diagnostics are distinct from W-D3's six judgments:

- wire decode error -> `WireRejected`, preserve existing codec `code`, null
  published effects/post;
- Source parse error -> `SourceRejected`, preserve existing Source6Error code
  and offset, null published effects/post;
- out-of-intersection signed identifier, round or scale -> `BindingRejected`,
  `W_D2F_DOMAIN_UNSUPPORTED`, name exact field; no translation;
- missing adopted relation or authentication evidence -> `BindingRejected`,
  `W_D2F_BINDING_UNAVAILABLE`, exact field and binding ID;
- mismatch under an available adopted relation -> `BindingRejected`,
  `W_D2F_FIELD_MISMATCH`, exact field;
- invalid evidence under an implemented verifier -> `EvidenceRejected`,
  `W_D2F_EVIDENCE_INVALID`, exact binding ID; this is specified-only because
  those verifier implementations are unavailable;
- Core rejection -> `CoreRejected`, preserve existing judgment/code/
  diagnosticWork=1, null published effects/post;
- supplied image not exactly equal to complete internally derived image ->
  `EffectRejected/W_D2F_PREPARED_IMAGE_MISMATCH`, null published effects/post;
- matching derived image but field26 differs ->
  `EffectRejected/EFFECT_COMMITMENT_MISMATCH`, null published effects/post.

Image schema errors preserve W-D2E code under EffectImageRejected; replay-cap
overflow uses the explicit EffectDomainRejected result above. E equality yields
SemanticComparedUnqualified with verifiedBindings B01–B16 and pendingBindings
[B17], null published effects/post, and no ledger change. Only separately
requested L mode can report LedgerRejected/W_D2F_BINDING_UNAVAILABLE for B17.

After D passes and earlier direct literals match, the first unavailable
local-path mapping today is agreementId/B01. This is a planned reject, not an
observed consumer execution. At any downstream predicate tested in isolation,
earlier gates remain explicit preconditions; no synthetic evidence may make a
full-path positive. Even after future verified gates, a local comparison must
remain unqualified while atomic ledger acceptance B17 is unavailable.

### Exact direct Core construction boundary

The specified constructor in PLAN.md copies every S0State field from verified
B11.snapshot except consumedReplay, which comes from verified B16.completeReplayHistory
for that same snapshot. Bound lowerSource6(ast).intent supplies the exact S0Intent;
lowerSource6(ast).submittedEffects supplies ordered effects with transformed
composite UseReplay keys. Verified B15.successor supplies proposedPostHead.
TerminalSuccess has empty retained arrays; optional localStipulation is omitted.
Never pass lowerer.state, Source-fabricated []/[selectedKey], raw AST replay
lines, or prepareSource6S0Unqualified. The direct Core diagnostic remains intact.
The external interface names snapshotProofInput; only the adopted proof verifier
can create the internal verified B11 snapshot. No caller snapshot is trusted.

```

## experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md

sha256: `b7cabbe427cb9ea9d05010b498baec30275c0a939070b4379bb42a34a99981e0`

```text
# W-D2F local semantic-consumer design plan

**Status:** design only, frozen before implementation. No consumer module is
created by this sprint. Full positive consumption is specified-only because
named mappings and authentication providers are unavailable.

**Goal:** map each signed field, identify exact missing evidence, and select a
bounded next implementation that cannot silently discard agreement, selected
program, scale, hash, predecessor or history obligations.

**Inputs read-only:** wire/SPEC.md and codec.mjs; Source/6 frontend and local
wrapper; Core/5 S0 preparer; W-D2E SPEC.md, codec.mjs and frozen fixtures.
**Outputs this sprint:** FIELD-MAP.md, EXPECTATIONS.json, this PLAN.md,
freeze-receipt.json and RESULT.md; all inside effect-consumer/.
**Acceptance:** exhaustive header+35-field and nested-operation map; complete
state-to-image derivation map; independent intended expectations for all six
W-D2E positives and hostile gates; exact diagnostics with gate preconditions;
no invented mapping and no claim of an observed end-to-end consumer success.

## Proposed full-path interface, specified-only

`consumeSource6Authorization({canonicalAuthorizationBytes, sourceDocument,
snapshotProofInput, signatureEvidence, selectionEvidence,
headExtensionEvidence, submittedEffectImage})`.

Evidence parameters name untrusted proof objects or provider requests, never
trusted caller snapshots. Only successful adopted verifiers create internal
immutable verified context. Their constructors, trust roots and verification
are absent. A JSON object or a caller Boolean is not a
substitute. Implementation must not add an option that admits empty evidence.
This repair selects **Reading B**: B01–B16 must be adopted and verified for a
local semantic image/field26 comparison. B17 is recorded as pending after that
comparison; it is not a rejection prerequisite for local comparison. Actual
ledger consumption additionally requires B17. Every provider remains absent
today, and this sprint implements neither local comparison nor consumption.

One total sequence controls first failure; an isolated predicate observation
must not be presented as this sequence's result:

1. **F-wire:** existing canonical decoder, 4096-byte bound. On error preserve
   WireRejected/code; do not parse Source. A typed effect-image domain error
   later is never WireRejected.
2. **F-source:** existing parser, 65536 UTF-8-byte bound. Preserve SourceRejected,
   original Source6Error code and UTF-8 byte offset. Parser formation checks
   (including selected action, aliases and scale18) precede every adapter check.
3. **D:** complete compatibility-domain sweep before any binding check. Visit
   signed fields in tag order, each mapped Source counterpart before the wire
   value, then operation fields in wire variant order. Check identifier subtype,
   mapped Source hash32 claims, scale and validity widths; then Source snapshot
   round and proposed successor. Exact lists are in FIELD-MAP.md. First failure
   -> BindingRejected/W_D2F_DOMAIN_UNSUPPORTED with field and sourcePath. Thus a
   late out-of-domain operation.owner beats unavailable agreementId/B01; a
   well-formed late literal mismatch does not. Never translate, clamp or mask.
4. **M:** first apply local V-S0-01 at tag1 to the exact version/profile
   tuple below, then signed tags in order. At each tag, perform declared direct literal
   checks, authenticate the provider facts anchored there and retain immutable
   verified context, then compare this tag's signed value to authenticated facts.
   Anchors authenticate integrity/scope/availability immediately, without
   comparing future signed fields early. Future equality is checked at that
   field's own tag. Literal/fact difference -> W_D2F_FIELD_MISMATCH; unavailable
   definition/provider -> W_D2F_BINDING_UNAVAILABLE; invalid implemented evidence
   -> W_D2F_EVIDENCE_INVALID. Then operation subfields in wire variant order,
   followed by unsigned Source snapshot claims in exact FIELD-MAP order, then
   B15 successor and B16 replay-history projection. Each B01–B16
   binding is verified once at its anchor, not silently skipped because a
   repeated literal matches. B10 anchors at asset/tag16, before scale/tag17.
   B04/tag6 retains Core ID for comparison at10; B08/tag13 retains authorized
   key for comparison at15; B10/tag16 retains registered scale for comparison
   at17. Complete provider/field comparisons, including unsigned snapshot
   claims, are mandatory before C. B09/tag14 uses retained B08 authorized key;
   a passing signature never substitutes for the later signerKey comparison.
   B09 anchors at exact keyScheme=schnorr_bip340/tag14: its message preprocessing and verifier
   are unselected; raw SHA256 as the signature message is not adopted here.
   SHA256 of canonical authorization bytes is only the current wire content
   digest. Default local scope never resolves or demands B17.
   At tag27 use selected local F-S0-01 correspondence between Source success_only
   and wire atomic-reject-terminal-success for closed S0 success/atomic reject.
   Their different strings are not a direct literal equality. This local rule
   does not adopt normative W-D2. Provider authentication and current-tag fact
   checks are detailed for every anchor in FIELD-MAP.md.
5. **H:** complete verified replay history must fit W-D2E's cap16 before Core
   preparation. Check replayBefore count first, then count after appending the
   selected tuple (if absent). >16 -> EffectDomainRejected/
   W_D2F_REPLAY_HISTORY_BOUND with field consumption.replayBefore or replayAfter,
   exact count and cap16, and null published effects/post. Do not truncate,
   deduplicate, or call this an authorization decoder rejection. Selected-key
   reuse is a valid B13 status rejection (W_D2F_REPLAY_CONSUMED), not an
   invalid-proof error; isolated Core reuse
   tests retain their own history rejection.
6. **C:** call direct read-only prepareMil4S0 with the exact constructor below:
   verified B11 snapshot cells/work, complete B16 history, bound lowered intent,
   ordered lowerer-transformed submitted effects, verified B15 successor and
   terminal-success outcome. Core derives internally before sameEffects.
   Preserve first existing judgment/code/diagnosticWork on rejection. Effect
   mismatches precede authority/history/failure; late Core predicate expectations
   explicitly require sameEffects to have passed. The outer B01–B16 verified
   path may reject stale-head or signature evidence earlier than an isolated
   Core predicate. Never turn those isolated tests into full-path assertions.
7. **I:** only after PreparedUnqualified, derive the entire W-D2E image from the
   same verified pre-state and Core post, including registered B10 scale, every
   required ordered cell, both work counters and complete replay histories.
   I sub-order is: validate/encode derived image; check supplied root/replay-array
   shape and replayBefore count then replayAfter count; validate/encode supplied
   image; compare complete images. Codec-shaped errors -> EffectImageRejected
   with original W-D2E code and imageSide=derived or supplied. Schema-valid
   unequal complete image -> EffectRejected/W_D2F_PREPARED_IMAGE_MISMATCH. Shaped
   supplied replay arrays over16 use explicit W_D2F_REPLAY_HISTORY_BOUND with
   imageSide=supplied before generic supplied codec checks; when both overflow,
   replayBefore wins. Validate both sides with the W-D2E encoder;
   any other effect codec bound (including record4096) preserves its original
   code under EffectImageRejected, with imageSide=derived or supplied. Neither
   side's encoder errors are authorization WireRejected. Do not hash the submitted
   image as the preparation oracle.
8. **E:** hash the derived complete image and compare decoded field26. Mismatch
   -> EffectRejected/EFFECT_COMMITMENT_MISMATCH. Equality ->
   `{status:'SemanticComparedUnqualified', commitment, verifiedBindings:[B01..B16],
   pendingBindings:[B17], publishedPost:null, publishedEffects:null}`. Candidate
   diagnostics may be returned separately; no effect is applied or published.
9. **L, separate requested ledger mode:** resolving B17 and performing actual
   atomic compare-and-consume is required for a ledger outcome. Missing B17
   fails only this later mode; it cannot retroactively block phases C/I/E or
   convert their unqualified result into ledger acceptance.

## Exact local profile relation and C handoff (specified-only)

V-S0-01 is a selected local compatibility relation over this single tuple:
wire header `moriarty-intent/3`, profile `s0-provisional/1`, sourceVersion `6`,
coreVersion `5`; AST.profile and lowered.intent.sourceProfile
`moriarty-financial-agreement-source/6`; lowered.intent.version
`moriarty-intent/3`; lowered.intent.core and verified snapshot.core
`moriarty-core/5`; derived image schema `moriarty-s0-effects/1` and core
`moriarty-core/5`. Different vocabularies are related, never compared as if
wire.profile must equal AST.profile. At M/tag1 check the formation-produced
wire/AST/lowered tuple; B11 later authenticates snapshot.core, and I checks the
image tuple. A formation-valid unsupported tuple would reject
BindingRejected/W_D2F_PROFILE_UNSUPPORTED at field profile, relation V-S0-01.
The current closed decoder/parser/lowerer cannot produce that unsupported
branch; it is a specified future compatibility branch, not fabricated wire
bytes or an executable hostile fixture. Current wrong-version inputs keep
their earlier F-wire/F-source formation errors. V-S0-01 establishes local
compatibility only, and neither source selection authenticity nor normative W-D2.

At C all M predicates, including unsigned snapshot claims and B16, and H have
passed. The following is the exact proposed construction, not executable
consumer code. `verified` denotes immutable verifier outputs; none of its
fields is a caller assertion. All arrays retain order, counts and values;
copying permits neither sorting nor omission. B16 verifies that its complete
history is the adopted projection of this same B11 snapshot, with each tuple
serialized exactly as Core's canonical JSON string `[domain,signer,nonce]`.

```ts
const lowered = lowerSource6(ast); // read-only existing lowering, bound by B04/B06
const s = verified.B11.snapshot;
const coreState: S0State = {
  core: s.core, domain: s.domain, asset: s.asset, head: s.head, round: s.round,
  workRemaining: s.workRemaining, workSpent: s.workSpent,
  balances: s.balances.map(({ account, amount }) => ({ account, amount })),
  allowances: s.allowances.map(({ owner, remaining, spent }) => ({ owner, remaining, spent })),
  obligations: s.obligations.map(({ id, debtor, creditor, asset, principal, accrued, outstanding, status }) =>
    ({ id, debtor, creditor, asset, principal, accrued, outstanding, status })),
  consumedReplay: [...verified.B16.completeReplayHistory],
};
const coreIntent: S0Intent = { ...lowered.intent }; // every mapped field already bound in M
const coreSubmittedEffects: S0Effect[] = lowered.submittedEffects.map(line => ({ ...line }));
const outcome: S0RequestedOutcome = { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [] };
const result = prepareMil4S0(coreState, coreIntent, coreSubmittedEffects,
  verified.B15.successor, outcome); // omit optional localStipulation
```

The lowerer transforms each submitted UseReplay nonce into
`JSON.stringify([ast.domain, ast.intent.signer, effect.key])`. Pass those
transformed effects, never raw AST UseReplay lines. Do not use lowered.state,
its Source-claim-derived `[]`/`[selectedKey]`, or Source wrapper preparation.
Do not manufacture signedDigest or a local stipulation. The Source wrapper's
unverified field republication and later overridden diagnostic are not the C
handoff. M has already checked signed/submitted action correspondence. Preserve
the direct Core result's judgment/code/diagnosticWork; build I from coreState
and its actual candidatePost/effects, not from republished Source state.

B13 first authenticates the replay-status proof, compares its tuple's current
nonce at tag20 (prior domain/signer/head must already agree), then requires
status unused. A genuine consumed fact rejects BindingRejected/
W_D2F_REPLAY_CONSUMED, field nonce, binding B13, factPath B13.replay.status,
comparisonTag20. An invalid proof remains EvidenceRejected/
W_D2F_EVIDENCE_INVALID. If B13 verifies unused and B16 verifies the selected key
absent but Source claims consumed, the B16 tail rejects FIELD_MISMATCH,
field/sourcePath authenticated.replay, factPath B16.selectedReplayStatus.
B13 rejection occurs before that tail even when Source also claims consumed.

## Honest bounded runnable subset

An **inspection-only adapter** can be implemented without inventing binding
rules. Proposed API:

`inspectSource6Wire({canonicalAuthorizationBytes, sourceDocument,
submittedEffectImage}) -> {status:'InspectionOnly', directComparisons,
unresolvedBindings, localCoreObservation, imageObservation,
publishedPost:null, publishedEffects:null}`.

It may parse existing types, show direct scalar differences, retain every
unresolved signed field, run existing Source/Core local preparation, and show
W-D2E equality separately. It must not combine these observations into
SemanticAccepted, Authenticated, Accepted or Qualified. Mappings marked
unselected stay unresolved even when strings happen to match. Their equality
is not an implementation of the missing registry or hash relation.

The second honest subset is testing existing Core/effect-image predicates in
isolation against already frozen literal vectors, explicitly without a Source
to signed-wire identity claim. This is useful regression evidence but largely
overlaps W-D2E. A fail-closed gate can reject current full-path requests with
the named unavailable binding; that negative behavior is runnable. No full
positive semantic consumer is currently runnable honestly.

## Recommended decision order and first meaningful implementation

Do not build the inspection API now. Existing W-D2E already covers isolated
Core/equality observations. The executable gap is authenticated correspondence,
so the next work should resolve the definitions needed to test that gap.

1. Adopt an explicit nominal identity map for agreement instance, stage, episode,
   action and selected Core program (B01–B04). Decide which missing identities
   need a new typed wrapper, and reject ambiguous historical labels. Do not
   reuse Core.intent.programId for the agreement.
2. Select exact Source, Core and policy hash images and lowerer correspondence
   (B05–B07). Resolve Source's embedded source_hash/digest self-reference by a
   reviewed image definition, without silently masking fields. Pin exact bytes
   and independently expected hashes before coding.
3. Adopt signer/keyRef ownership and asset/scale mappings (B08/B10). Reconcile
   Source18 and wire38 scale bounds explicitly. Select common identifier and
   round domains without translation. Do not assume current literal key hashes
   or scale2 supply metadata authentication.
4. Specify and implement the authenticated snapshot transport, predecessor,
   replay policy, grant/work cells and complete replay-history projection
   (B11–B14/B16). Preserve same-head provenance and every required cell.
5. Adopt digest-independent head extension and exact signature message/verifier
   semantics (B15/B09). Use real selected implementations with negative evidence
   controls. These definitions must coexist with field26 without feedback into
   the successor or selected source image.
6. Freeze new independent full local positive/hostile vectors under those
   definitions. Existing W-D2E authorizations remain equality fixtures; they
   are not automatically upgraded to authenticated Source fixtures.
7. Implement the **bound semantic derivation adapter** as the first useful new
   consumer: decode exact authorization bytes, check every adopted mapping and
   available verified premise, prepare effects with read-only Core/5, derive all
   image rows from that same pre/post, compare complete submitted image, then
   recompute field26 from the derived image. Tests must reject a re-committed
   hostile vector even though W-D2E equality alone passes. Missing providers
   reject; no successful full authentication is simulated.
8. Preserve commands, exact outputs, scope hashes and dependency hashes in a
   new implementation packet. Obtain separately routed review. Only after this
   local result should B17 atomic ledger acceptance/consumption be implemented
   and empirically demonstrated; local success must expose B17 as still open.

## Implementation gate and missing decisions

Do not implement or assert a positive full consumer until B01 agreement-instance,
B02 stage, B03 episode, B04 action/program and B05–B07 source/Core/policy images
have adopted definitions. B08 keyRef, B10 asset/scale, B12 predecessor and B16
replay-history transport require adopted definitions too. B09/B11/B13–B15
require real verification implementations for local comparison; B17 requires
separate actual ledger consumption. Independent positive
fixtures must then be frozen from those definitions before full-path code.

This design does not expand Source/Core types, choose a masked source hash,
drop a signed field, create a synthetic authenticated registry, bind opaque
heads by a caller flag, or adopt new normative W-D2/W-D3 semantics.

```

## experiments/moriarty-language/formal/mil4/identity-binding/SPEC.md

sha256: `33b4fcb274af1d4dd4a053b6019a09e03565250e53669af6f304c9d9488745c0`

```text
# W-D2G B01–B04 nominal identity proposal

**Status: proposed / specified-only, 2026-09-30.** This document proposes an
identity relation. No registry, verifier, consumer, authentication, proof or
ledger acceptance is implemented. The recommendation is not an adopted vote.
W-D2/W-D3 and B01–B17 remain open. Scope is B01–B04 only.

## Repository observations

Primary inputs are [W-D2F FIELD-MAP](../effect-consumer/FIELD-MAP.md) and
[W-D2F PLAN](../effect-consumer/PLAN.md), inspected without modification.
The following are observations of the current modules, not proposed behavior.

| Nominal role | Source/6 | Core/5 | Authorization wire /3 | W-D2E effects/1 |
| --- | --- | --- | --- | --- |
| Domain | `ast.domain` | `state.domain`, `intent.domain` | tag2 `domain` | root domain and replay tuples |
| Agreement instance | `ast.programId`, parsed after `agreement`; plain identifier with no instance registration | No slot | tag3 `agreementId` | No slot |
| Stage | No slot | No slot | tag4 `stageId` | No slot |
| Episode | No slot | No slot | tag5 `episodeId` | No slot |
| Selected action | `ast.selected.actionId` | Copied into `intent.programId` | tag6 `actionId` | No action-ID slot |
| Selected Core program | No distinct slot; builtin selector is `selected.actionId` | `intent.programId`, checked against the supported builtin for the operation kind | tag10 `coreProgramId`; tag11 `coreHash` | `operationKind` and fixed Core/5 version; no program-ID/hash slot |

Evidence: [AST and parser](../../../src/successor/financial-agreement-source-v6-frontend.ts),
[lowerer](../../../src/successor/financial-agreement-source-v6-frontend.ts),
[Core preparation](../../../src/successor/mil4-s0-core-v5.ts),
[Source wrapper](../../../src/successor/mil4-s0-source-v6.ts),
[wire schema](../wire/SPEC.md), [effect schema](../effect-wire/SPEC.md).

Source parsing requires `TransferLiteralFee` for a Transfer and
`RepayAccrualFirst` for a Repay (`expectedActionId` in the parser). Core checks
the same builtin/kind pair. Lowering sets Core `programId=selected.actionId`;
it does not copy Source's agreement `programId` into Core. Source retains the
agreement label in the returned AST. Neither type has stage or episode fields.
The wrapper returns `PreparedUnqualified` with `agreement-id` and
`selected-program` among its four unverified bindings.

Wire IDs are ASCII strings of length1–64 with alphabet
`[A-Za-z0-9][A-Za-z0-9._:/-]*`. Source IDs begin with a letter, then letters,
digits or underscore, maximum64, excluding the parser's exact reserved words.
Core IDs begin with a letter and additionally admit dot/hyphen after it.
Wire fields have nominal roles in the specification; their string codec does
not authenticate the role or ownership. A common-domain check is additional
adapter behavior proposed in W-D2F.

The six existing [W-D2E positives](../effect-wire/fixtures.json) use
`agreementId=Agreement`, `stageId=Stage`, `episodeId=Episode`, `actionId=Action`
and the appropriate builtin `coreProgramId`. They contain no Source document
or registry binding. W-D2E can compare a supplied effect image and field26;
that comparison provides no identity correspondence. Its image intentionally
has no agreement, stage, episode, action or Core-program identity fields.

## Recommended mapping A: strict names with a contextual wrapper

**Recommendation, specified-only.** Preserve the existing closed Source/6
builtin selectors and the W-D2F literal comparisons. Give each nominal role
its own type, even where two role values have the same ASCII spelling.

For a compatible parsed Source document `s`, lowered Core intent `c` and wire
authorization `w`, the proposed relation is:

```text
w.domain        = s.domain = c.domain
w.agreementId   = s.programId                     (AgreementInstanceId)
w.actionId      = s.selected.actionId              (ActionId)
w.coreProgramId = s.selected.actionId = c.programId (CoreProgramId)
(w.actionId, w.coreProgramId, operationKind) is a registered allowed triple
```

The only proposed S0 triples are
`(TransferLiteralFee,TransferLiteralFee,transfer)` and
`(RepayAccrualFirst,RepayAccrualFirst,repayment)`. Equality of their two names
does not identify their nominal sorts and does not prove B04 or B06.
`w.agreementId` is never obtained from Core `programId`.
These triples constrain semantic compatibility. Authenticating a manifest's
declared target is a separate predicate: a genuine record declaring another
target is a valid provider fact until field comparisons reject its target.
Core-program differences reject at tag10; a constructor-only difference rejects
at tag35. An authentic incompatible target is not an invalid proof merely
because it falls outside the compatible triples.

Add five nominal sorts at the adapter boundary:
`AgreementInstanceId`, `StageId`, `EpisodeId`, `ActionId`, `CoreProgramId`.
Domain remains the existing domain carrier; qualified product keys below
always include it. A future implementation may brand the domain too, but this
proposal does not require a new domain format. Each constructor accepts the
W-D2F common subtype `[A-Za-z][A-Za-z0-9_]{0,63}` excluding Source reserved
words. Constructors establish shape and sort only, never authentication.

Stage and episode claims require one new wrapper record surrounding the
unchanged Source AST/Core input:

```text
Source6IdentityContext = {
  stageId: StageId,
  episodeId: EpisodeId
}
```

Each wrapper value must equal its signed wire field at that field's M anchor.
They are caller claims until verified. Do not synthesize `Stage`, `Episode`,
an array index, action name or nonce when a wrapper slot is missing. Derive
agreement/action/program claims from the retained AST and actual lowerer result,
rather than admitting a second freely editable copy. This wrapper is not a
Source/6 grammar extension or a change to Core/5 preparation.

## Minimum registry obligations

**Proposed relation shapes only.** These are logical tables; no database,
evidence constructor, root encoding, authority scheme or verifier is supplied.
Registry authentication and its connection to the snapshot are unavailable.

| Binding | Scoped identity key | Required relation |
| --- | --- | --- |
| B01 | `(domain, agreementInstanceId)` | Exactly one agreement-instance record binds the Source agreement claim and cell namespace, retaining declared stage/episode/action/Core-program/asset associations. Compare these later at tags4/5/6/10/16. The instance cannot be silently rebound to another definition or namespace. Code-image authenticity remains B05–B07; cell authenticity remains B11. |
| B02 | `(domain, agreementInstanceId, episodeId, stageId)` | Authenticate one stage occurrence and its prior agreement linkage at tag4. Retain its episode association for tag5 and a stage-association/anchor reference for the B11 stage-to-snapshot linkage obligation. B02 does not retain or compare a second authoritative preHead. A stage label cannot identify two occurrences within an episode. |
| B03 | `(domain, agreementInstanceId, episodeId)` | Authenticate one continuing episode/history record, with prior agreement linkage and membership of the exact stage occurrence already authenticated by B02. An authentic episode record alone does not establish this membership. Initial-history references do not prove predecessor linkage (B12), head extension (B15) or durable consumption (B17). |
| B04 | `(domain, agreementInstanceId, episodeId, stageId, actionId)` | Authenticate one selected-action manifest and prior instance linkage at tag6. Retain its builtin selector, expected CoreProgramId, constructor and source/Core/policy image references. Compare program IDs at10, source/Core/policy images through B05–B07 at8/11/12, and constructor at35. No exact image definition or lowering proof is invented here. |

B01's five declared associations are authenticated finite sets of the
corresponding nominal role, presented as arrays in the symbolic projection.
The current signed ID must be a member of the respective set; never compare
the entire set to a singleton or to the caller's selected IDs. Multiple stages,
episodes or actions may be declared. An agreement may declare both S0 actions
and both Core program IDs; selecting TransferLiteralFee then requires that
name's membership plus the exact selected B04 relation. Membership in separate
sets does not authorize arbitrary action/program pairings or constructors.
Array order is irrelevant to this logical membership predicate; this is not a
choice of registry bytes, storage representation or list-size cap.

B02 authenticates its declared record at the scoped lookup key and its already
processed agreement/stage linkage. Its payload's declared episode association
is retained, then compared at tag5. Using the signed episode as a lookup-key
component does not waive that payload comparison or compare the future field
early. A genuine record can declare a different future episode association;
that is a tag5 FIELD_MISMATCH, not failed record authentication. B03's prior-stage
membership is a separate proof of membership of the exact stage occurrence
already authenticated, not an early comparison of that future association.

All identity tables must use one authenticated immutable registry view with
the same domain/agreement. Its association with the snapshot is a B11
verification obligation, not a second head equality performed by B01–B04.
At the identity anchors verify provider integrity and links to already verified
identity context; retain a registry-view association reference for B11. A
caller-selected root or `verified:true` is insufficient. Trusted-view selection,
its proof format and authority/rotation semantics remain open. Missing adopted
registry-view/stage-to-snapshot linkage is unavailable B11 at tag18; a proof
that cannot establish its declared linkage is invalid B11 evidence. Neither
equal local roots nor an immutable object establish that linkage.

Product keys are compared component by component. No concatenation, Unicode
normalization, case folding, whitespace trimming, punctuation replacement,
implicit namespace inheritance or string-to-sort coercion is permitted. This
proposal does not choose canonical registry bytes or a registry hash; those
need a later reviewed image definition. The same stage/episode label may occur
in different scoped keys. Two agreement instances within one domain require
different agreement IDs. Reusing an action/program spelling across agreements
is permitted only through each agreement's explicit relation.

Cross-sort equality of text is permitted. For example an agreement named
`TransferLiteralFee` is not automatically forbidden, but it must resolve as an
AgreementInstanceId with its own B01 record. A CoreProgramId record with that
spelling cannot satisfy B01. Template-to-instance reuse under one Source
agreement label is outside mapping A; distinct instances must supply distinct
Source agreement IDs and later satisfy B05's chosen source-image relation.

## Proposed check order and diagnostics

Preserve W-D2F F-wire, F-source, then the complete D-domain sweep before M.
Tag1 follows current W-D2F V-S0-01's exact version/profile relation, never raw
cross-profile string equality. This identity proposal changes no tag1 rule.
Wrong Source selected builtin remains a parser error (`SOURCE6_PROFILE_UNSUPPORTED`) before
adapter mapping. D covers the wrapper claims at their corresponding stage/
episode positions as an explicitly proposed additional input-domain check;
Source still has no such slots. Present wrapper claims are checked before
their corresponding wire value; a missing wrapper claim is reported at its M
availability anchor, rather than treated as a malformed present identifier.
No adapter error receives a fabricated Source offset. A compatible wrong
literal is handled at M, not D.
For present wrapper claims, report `sourcePath=context.stageId` or
`context.episodeId`; this is a separately proposed wrapper-path extension,
not a Source AST path or an adopted W-D2F rule. For a decoded wire-only subtype
failure preserve W-D2F: `sourcePath=null`, with `inputPath=wire.stageId` or
`wire.episodeId` (and `wire.field` for other wire-only fields). Adapter
results never fabricate a parser offset. At each tag the present wrapper claim
is swept before its wire counterpart; missing claims still wait for M.

At tag3 compare wire agreement to the AST, then verify B01. At tag4 compare
wire stage to the wrapper, then verify B02 using the signed episode ID in its
key and retain its episode association and stage-association reference. B02
does not authenticate a snapshot head or compare preHead. It does not require a Source
episode slot or resolve the wrapper episode claim early. At tag5 compare
episode to the wrapper, then verify B03. A missing
stage/episode claim reports unavailable B02/B03 at its respective anchor.
At tag6 perform these steps in this exact order: (1) compare wire actionId to
Source selected.actionId; (2) authenticate B04's declared manifest and prior
identity links, retaining its builtin selector, expected CoreProgramId,
constructor and image references; (3) test current actionId membership in B01's
retained actionIds; (4) compare current actionId to B04's retained current-action
fact. Reject the first failure. Valid provider
evidence can establish a different fact from the submitted Source/program/
operation claims. Such a difference is a field mismatch at that field's
comparison point, not invalid evidence. Do not compare submitted signed
coreProgramId or operation kind at tag6. At tag10 perform these comparisons in
this exact order: (1) signed coreProgramId versus Source selected.actionId;
(2) versus actual lowered Core `programId`; (3) membership of current
coreProgramId in B01's retained coreProgramIds; (4) versus B04's retained expected
CoreProgramId; (5) versus B04's registered builtin selector under mapping A. The first
difference is `W_D2F_FIELD_MISMATCH`, field coreProgramId. Earlier tag8 B05 and
all other preceding gates must have passed before a tag10 result is reachable.
At tag18 compare wire preHead to AST.intent.preHead, then AST.authenticated.head,
then authenticate B11's complete declared same-instance/domain/asset snapshot
fact, including the proposed registry-view/stage-association-to-snapshot
linkage. After complete B11 verification, compare its authentic head with signed
preHead. No B02 reference may independently veto this head comparison. Missing
linkage definition/provider reports `W_D2F_BINDING_UNAVAILABLE/B11`; invalid
snapshot or linkage evidence reports `W_D2F_EVIDENCE_INVALID/B11`. A complete
valid B11 fact for another head reports `W_D2F_FIELD_MISMATCH/preHead`.
That authenticated-head mismatch carries binding B11, factPath
B11.snapshot.head and sourcePath=null; the Source head literals already passed.
This specifies B11's additional identity linkage obligation; its implementation
and reviewed definition remain unavailable. At tag35 compare operation.kind
with B04's retained expected constructor after the existing Source constructor
comparison; report field operation.kind on a difference.

For tag4 the order is direct wrapper-stage equality, B02 record authentication,
then current stageId membership in B01's stageIds. For tag5 the order is direct
wrapper-episode equality, B03 prior-stage membership/record authentication,
then current episodeId membership in B01's episodeIds, then equality with B02's
retained episode association, then the current B03 episode fact. At tag16 the
order is direct Source asset equality, B10 provider authentication, current
asset membership in B01's assetIds, then equality with B10's current asset fact.
The tag6/tag10 lists above include both B01 and B04 explicitly. B03 authentication must establish membership of the
already verified stage occurrence, not merely an unrelated episode record.

At tag8/11/12, B05/B06/B07 must establish their artifact/image identity's link
to B04's retained source/Core/policy image reference respectively. This is a
link to already verified selected context, verified as part of the respective
provider authentication. A genuine artifact proof for another image does not
prove that link and reports invalid evidence at B05/B06/B07. Once the selected
artifact is authenticated and its hash computed, compare the current signed
hash to the retained authenticated hash at its own tag. A valid selected
artifact with a different signed hash is FIELD_MISMATCH, never invalid evidence
solely because the signed hash differs. Exact image bytes remain B05–B07's open
definitions; symbolic references here are not hash algorithms.

Missing or invalid B04 registry/provider evidence still rejects at tag6. A
well-formed changed signed coreProgramId with valid B04 evidence reaches the
tag10 literal check. This two-step sequence is proposed alignment with a
W-D2F provider-fact schedule, not a claim of normative W-D2/W-D3 adoption.
Invalid evidence means the selected verifier cannot establish the claimed
provider fact for the required scoped key/view, including proof-content,
provenance, uniqueness or authentication failure. A valid proof of a different
retained value does not fail authentication solely because that value differs
from a later signed field.
B01 records selection/cell references for later verification; it does not
silently run B04's action/program checks, B05–B07 image checks or B11 snapshot
authentication. B02 stage membership similarly does not run B03's continuing
episode authentication early. Each relation's owned fields determine its
diagnostic anchor; earlier relation availability remains a prerequisite.

Use the already proposed W-D2F result families:

- Unsupported identifier -> `BindingRejected/W_D2F_DOMAIN_UNSUPPORTED`, exact
  field and input path.
- Literal/wrapper difference -> `BindingRejected/W_D2F_FIELD_MISMATCH`, exact
  field; no invented registry result.
- Missing wrapper relation or provider ->
  `BindingRejected/W_D2F_BINDING_UNAVAILABLE`, field and B01/B02/B03/B04.
- A real implemented verifier cannot establish its declared provider fact or
  required prior-context linkage because of tampered proof content, wrong
  scoped lookup, failed provenance/view binding or nonfunctional registration
  (two different records for one exact identity key) ->
  `EvidenceRejected/W_D2F_EVIDENCE_INVALID`, exact anchored binding.

A genuine unique B01 record whose authenticated association set omits a later
current signed ID is disjoint from these invalid-evidence cases: provider
authentication succeeds, then the membership check yields FIELD_MISMATCH at
that ID's tag, after that tag's anchored provider authentication. Two permitted
action names in one authenticated finite set are not conflicting registration.
An authentic different future value likewise mismatches at its own tag; an
invalid proof that fails its declared fact rejects at the authentication anchor.

All rejection results have null published post/effects. The invalid-evidence
branch is specified-only because no verifier exists. Equal identity claims
with all providers absent would stop at B01, after successful earlier checks.
Even verified B01–B04 alone would not yield W-D2F local semantic comparison:
B05–B16 remain required, and B17 remains separate ledger acceptance.

## Compatibility and bounded next sprint

Mapping A changes no wire/3 or effect/1 bytes, Source/6 grammar or Core/5 module.
It adds a proposed wrapper requirement to a future authenticated consumer.
Existing Source/Core preparation retains its unqualified interpretation.
Existing wire and W-D2E fixtures remain valid for their existing codec/equality
purposes. They are incompatible as authenticated Source-to-wire positives
because `Action` differs from the required Source selected builtin.

Do not rewrite their action field or archived bytes. New independent identity
vectors must choose the strict builtin action name; changing actionId changes
canonical authorization bytes/content digest and requires new external
signature evidence. It need not change the existing effect image because that
schema omits these identities, but unchanged effect equality still cannot
establish the new relation. Source agreement renaming also needs fresh source
and policy image decisions once B05–B07 are adopted.

The bounded follow-on sprint should seek independent design review of mapping
A versus B in [DECISION-MATRIX](DECISION-MATRIX.md), then freeze independent
identity-only inputs/expected outcomes using the hostile cases there. A future
shape/literal checker may demonstrate only those comparisons and named
unavailable bindings. A real positive identity verifier additionally needs an
adopted registry authority, view/snapshot binding and implementation. Neither
step can claim full authenticated Source/Core correspondence or gate closure.

## Explicit open questions

1. Registry authority, bootstrap, update/rotation rules, proof format and
   trusted view selection are unspecified; B11 must bind the same snapshot.
2. Episode genesis and continuing-history semantics, stage allocation and
   concurrency are not chosen. Scoped identity uniqueness alone proves none.
3. Exact Source/Core/policy images and lowerer correspondence remain B05–B07;
   Source's embedded hash self-reference is not resolved here.
4. Reusing one source template for multiple instances may require a distinct
   future AgreementDefinitionId or a versioned instance wrapper. Mapping A
   deliberately does not adopt that extension.
5. Existing replay keys remain `(domain,signer,nonce)` and omit agreement,
   episode and stage. This proposal neither changes replay policy nor proves
   cross-instance replay safety; that remains B13/B16/B17 work.
6. Effect/1 omits these identity fields. Whether a later effect schema should
   commit them explicitly remains open; field26 equality cannot replace B01–B04.
7. A reviewed compatibility decision and real verifier evidence are absent.
   This proposal is not decision-grade authentication evidence.

```

## experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md

sha256: `53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8`

```text
# W-D2E finite effect commitment candidate

Experiment specification; separate from the frozen S1B and W-D2 audit packets.
This proposal does not select normative W-D2 bytes or close wallet, proof,
source/Core correspondence, snapshot authentication, or ledger consumption.

## Construction and dependency

Select a successor head independently of the authorization digest. Local S0
preparation supplies complete ordered effects and required state cells. Encode
the effects and complete pre/post footprint under the scheme below, compute
SHA-256 of these bytes, and insert the lowercase hexadecimal hash into signed
authorization field 26. Only then encode and digest the authorization.

`state + operation + independently selected successor -> effects + footprint ->
effect bytes -> effect commitment -> authorization bytes -> authorization digest
-> external signature`.

The effect schema admits no authorization digest, effect commitment, signature,
proof, signature-valid flag, or head derived from this authorization digest.
Absence of a field cannot prove an external caller selected an independent head;
that dependency remains a producer/protocol premise. The consumer decodes the
existing canonical authorization bytes and compares field 26 with a recomputed
hash. A successful comparison returns `CommitmentEqualUnqualified`. It neither
verifies a signature nor prepares, authenticates, or consumes a state transition.
An attacker who can replace both bytes and commitment can obtain equality.

## Canonical encoding

The header is ASCII `moriarty-s0-effects/1` followed by NUL (22 bytes).
All integers and counts use big endian. No field tags, optional fields, trailing
metadata, or generic extensions exist. Property insertion order has no effect;
array order is preserved and committed. The implementation is an encoder only.

Primitive `id`: UInt16 byte length, then 1–64 ASCII bytes matching
`[A-Za-z0-9][A-Za-z0-9._:/-]*`. `hash`: 32 raw bytes from exactly 64 lowercase
hex digits. `u64`: eight bytes, cap 2^64-1. `u128`: sixteen bytes, cap 2^128-1.
`nominal`: sixteen bytes, cap 2^127-1. JSON integers are canonical unsigned
decimal strings; JSON numbers, signs, leading zeros, or exponents reject.
`status`: byte 01 Outstanding, 02 Settled. `kind`: byte 01 transfer, 02 repayment.
Scale is one integer byte, cap 38. Core is fixed `moriarty-core/5` byte 05.
Arrays have UInt16 counts. Maximum record 4096 bytes, effects 6, balances 3,
allowances 1, obligations 1, replay history 16 entries per array. These are
experimental finite bounds, with no claim about network or wallet feasibility.

Root object keys and their exact byte order:

1. `schemaVersion`: exact `moriarty-s0-effects/1`, represented by the header.
2. `core`: fixed Core/5 byte; `domain`: id; `asset`: id; `scale`: byte;
   `operationKind`: kind; `round`: u64; `preHead`: hash; `successor`: hash.
3. `effects`: count then each line in supplied order, using variants below.
4. `footprint`: object with `balances`, `allowances`, `obligations`, in that order.
5. `consumption`: object with work counters followed by replay arrays below.
6. `requiredPremises`: exact ordered four-string array below, encoded byte 0f.

Effect variants (one kind byte followed by the fields in this order):

| Kind byte | Kind | Fields |
| --- | --- | --- |
| 01 | Debit | account:id, asset:id, amount:nominal |
| 02 | Credit | account:id, asset:id, amount:nominal |
| 03 | SetObligation | id:id, principal:nominal, accrued:nominal, outstanding:nominal, status:status |
| 04 | UseAllowance | owner:id, amount:nominal |
| 05 | UseReplay | key:replay |
| 06 | AdvanceHead | predecessor:hash, successor:hash |

`replay` JSON presentation is exactly `JSON.stringify([domain, signer, nonce])`
with domain and signer satisfying id and nonce satisfying hash. Encode three
typed components (id,id,hash), without the JSON quotes or punctuation. This is
the Core/5 composite replay key presentation for these finite fixtures. A
whitespace-modified or otherwise noncanonical JSON spelling rejects.

Each footprint array has a count then rows:

| Array | Fields in each row |
| --- | --- |
| balances | account:id, before:u128, after:u128 |
| allowances | owner:id, remainingBefore:u128, spentBefore:u128, remainingAfter:u128, spentAfter:u128 |
| obligations | id:id, debtor:id, creditor:id, asset:id, principalBefore:nominal, accruedBefore:nominal, outstandingBefore:nominal, statusBefore:status, principalAfter:nominal, accruedAfter:nominal, outstandingAfter:nominal, statusAfter:status |

Consumption fields in exact order are `workRemainingBefore`, `workSpentBefore`,
`workRemainingAfter`, `workSpentAfter` (all u128), then `replayBefore` and
`replayAfter` (each count then replay entries). Head consumption appears both
in the root pre/successor fields and the ordered AdvanceHead line. Allowance
consumption appears both in UseAllowance and its pre/post footprint. These
redundant views deliberately commit disagreements instead of silently erasing
them. Semantic consistency and exact required row selection are producer duties.
The footprint includes all cells admitted by these S0 Core/5 fixtures, including
the unchanged zero-fee recipient balance. No netting, sorting, or deduplication
occurs. Work debit is explicit through both pre/post counters.

`requiredPremises` is exactly `canonical-intent-signature`, `snapshot-to-head`,
`head-extension`, `atomic-ledger-compare-and-consume`, in that order. Byte 0f
records requirements, not claims that they were established. Failure, retained
effects and retained duties remain the existing authorization's terminal-success
empty fields; this experiment does not introduce a failure path.

Only exact own enumerable data properties are admitted; inherited, accessor,
missing and extra keys reject. Array indexes must be dense own enumerable data
properties. Reflection on arbitrary proxies is outside the experiment's host
sandbox claims. Primitive bounds precede allocation or integer parsing.

## Independent expected cases, frozen before JS implementation

`reference-vectors.py` supplies literal pre-state, signed terms, ordered effects,
and literal post-state; arithmetic is not delegated to Core/5. Its separate
Python struct/hashlib construction freezes exact bytes and SHA-256 in
`fixtures.json`. JS must not generate or update these expected values.

Positive cases: transfer 10 fee 1 (gross 11); transfer 10 fee 0 (omit fee Credit,
retain fee recipient footprint); AccrualFirst repayment 30 against 1000+10
(980+0); accrued-only repayment 5 (1000+5); full repayment 1010 (0+0 Settled);
near-bound repayment 1 with UInt128 credit/allowance-spent endpoint and signed
nominal principal endpoint. Every case consumes one work unit, appends its exact
replay key, and advances a preselected head.

Hostile cases retain the original authorization commitment: changed Debit
amount, dropped line, appended line, reordered lines, changed recipient,
changed allowance line, changed work counter, omitted balance footprint,
changed balance value, changed allowance pre-state, changed obligation debtor,
changed principal allocation, replay line nonce, replay history, replay domain,
root head, successor line, footprint order, and required premise order.
All encodable mutations must produce different bytes and commitment, and return
`Rejected/EFFECT_COMMITMENT_MISMATCH`; an invalid schema mutation returns a codec
error. Exact per-case expected codes and hashes are frozen in fixtures.

Additional schema controls: extra digest/commitment/signature fields, inherited
fields, accessor fields, numeric values, cap+1, noncanonical decimals, uppercase
hash, malformed replay key, excessive row count, and unknown line kind reject.
Core/5 comparison reads the existing preparation module and compares its result
to every independent pre/post/effect vector; status must remain
`PreparedUnqualified`. This does not prove cross-layer correspondence.

## Stable local diagnostics and limits

Encoder codes: `SHAPE`, `LITERAL`, `ID`, `LENGTH`, `HEX`, `INTEGER`, `RANGE`,
`REPLAY`, `VARIANT`. Equality mismatch is `EFFECT_COMMITMENT_MISMATCH`.
These are experiment codes; W-D3 diagnostics remain open.

Equality demonstrates binding only for the supplied canonical data and frozen
finite cases, subject to SHA-256 assumptions. It does not establish semantic
preparation for arbitrary input, collision resistance empirically, a circuit,
proof soundness, source identity, state/head authentication, replay prevention,
signature or wallet interoperability, native verification, atomic acceptance,
or financial ledger settlement. W-D2 and all these acceptance gates remain open.

```

