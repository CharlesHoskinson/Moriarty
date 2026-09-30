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
