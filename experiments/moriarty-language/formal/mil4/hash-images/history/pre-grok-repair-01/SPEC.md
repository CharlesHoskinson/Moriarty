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
