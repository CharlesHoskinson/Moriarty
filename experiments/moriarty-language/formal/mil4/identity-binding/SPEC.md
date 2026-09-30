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

Under mapping A, the matrix's `selectedTarget` tuple records B04's retained
current-action/builtin-selector fact, expected CoreProgramId and constructor,
in that order. A genuine difference in its first component rejects at tag6
after B01 action membership, before any tag10 or tag35 comparison. G29 keeps
the baseline TransferLiteralFee first component and changes only the expected
Core program and constructor; its first mismatch is therefore at tag10 once
all earlier gates pass.

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
then authenticate integrity of B11's declared same-instance snapshot fact,
including the proposed registry-view/stage-association-to-declared-snapshot
linkage. This reference linkage does not compare core/domain/asset metadata
early. After integrity succeeds, compare genuine retained facts in current
W-D2F order: snapshot.core against V-S0-01 Core/5, snapshot.domain against prior
bound wire/AST domain, snapshot.asset against prior wire/AST/B10 asset, then
snapshot.head against current signed preHead. Genuine wrong core preserves
W_D2F_PROFILE_UNSUPPORTED/coreVersion; genuine wrong domain/asset/head gives
FIELD_MISMATCH/domain/asset/preHead at18, with binding B11 and corresponding
factPath B11.snapshot.core/domain/asset/head, sourcePath=null. Invalid declared
proof rejects before these fact comparisons. No B02 reference independently
vetoes the B11 head comparison. Missing
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
The matrix explicitly includes wire sourceHash/coreHash/policyHash and the
Source selected.sourceHash/policyDigest claims. G51–G53 isolate genuine selected
artifacts with signed hash mismatches, separately from G38–G40's invalid
wrong-image linkage. Both Source preHead claim paths are also explicit in the
baseline; G12/G46–G48 require those two literal comparisons to pass before B11.

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
