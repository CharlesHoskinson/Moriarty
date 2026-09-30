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
| B01 | `(domain, agreementInstanceId)` | Exactly one agreement-instance record binds the Source agreement claim, permitted selected-code references and cell namespace. The instance cannot be silently rebound to a different definition or another namespace. Code references require B05–B07; cell authenticity still requires B11. |
| B02 | `(domain, agreementInstanceId, episodeId, stageId)` | Exactly one authenticated stage occurrence supplies an expected preHead and relevant agreement cell namespace under the signed episode's scoped key. Tag4 verifies this provider fact and retains expected preHead; comparison with signed preHead occurs at tag18. Within an episode a stage label cannot refer to two occurrences or two pre-heads. Action/program selection is checked by B04. |
| B03 | `(domain, agreementInstanceId, episodeId)` | Exactly one continuing episode record binds the same agreement, its initial history anchor and declared stage membership. Membership is not proof of valid predecessor linkage (B12), head extension (B15) or durable consumption (B17). |
| B04 | `(domain, agreementInstanceId, episodeId, stageId, actionId)` | Exactly one authenticated selected-program relation supplies a registered builtin selector, expected CoreProgramId and expected operation constructor under the signed action's scoped key. Tag6 verifies this provider fact and retains its values. Program/selector comparisons occur at tag10 and constructor comparison at tag35. The relation references source/Core/policy commitments without defining their images or establishing lowering correspondence (B05–B07). |

All tables participating in one proposed check must be verified under one
immutable registry view associated with the same domain/agreement and snapshot
anchor. A caller-selected root or `verified:true` is insufficient. Selection
of that trusted view, its proof format and authority/rotation semantics remain
open; the wrapper contains claims, not a trusted view. Authentication cannot
be inferred from an immutable local object or from equal hashes.

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
Wrong Source selected builtin remains a parser error (`SOURCE6_PROFILE_UNSUPPORTED`) before
adapter mapping. D covers the wrapper claims at their corresponding stage/
episode positions as an explicitly proposed additional input-domain check;
Source still has no such slots. Present wrapper claims are checked before
their corresponding wire value; a missing wrapper claim is reported at its M
availability anchor, rather than treated as a malformed present identifier.
No adapter error receives a fabricated Source offset. A compatible wrong
literal is handled at M, not D.

At tag3 compare wire agreement to the AST, then verify B01. At tag4 compare
wire stage to the wrapper, then verify B02 using the signed episode ID in its
key and retain the provider's expected preHead. B02 authenticates the stage
fact; it does not compare a later signed preHead or reinterpret a valid
different stage anchor as an invalid proof. B02 does not require a Source
episode slot or resolve the wrapper episode claim early. At tag5 compare
episode to the wrapper, then verify B03. A missing
stage/episode claim reports unavailable B02/B03 at its respective anchor.
At tag6 compare action to Source selection, then verify B04's registry/provider
fact for the complete contextual key, then retain the registered builtin
selector, expected CoreProgramId and expected constructor. Valid provider
evidence can establish a different fact from the submitted Source/program/
operation claims. Such a difference is a field mismatch at that field's
comparison point, not invalid evidence. Do not compare submitted signed
coreProgramId or operation kind at tag6. At tag10 compare signed coreProgramId
to Source selection, then actual lowered Core `programId`, then the retained
expected CoreProgramId and registered builtin selector under mapping A. The first
difference is `W_D2F_FIELD_MISMATCH`, field coreProgramId. Earlier tag8 B05 and
all other preceding gates must have passed before a tag10 result is reachable.
At tag18 compare wire preHead to AST.intent.preHead, then AST.authenticated.head,
then B02's retained expected preHead; a difference is the same field-mismatch
family with field preHead, before B11 verification at that tag. At tag35 compare
operation.kind with B04's retained expected constructor after the existing
Source constructor comparison; report field operation.kind on a difference.

Missing or invalid B04 registry/provider evidence still rejects at tag6. A
well-formed changed signed coreProgramId with valid B04 evidence reaches the
tag10 literal check. This two-step sequence is proposed alignment with a
pending W-D2F repair, not a claim that W-D2F has completed final review or
adopted the rule.
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
- A real implemented verifier finds invalid evidence, wrong scoped relation,
  duplicate key, stale view or conflicting relation ->
  `EvidenceRejected/W_D2F_EVIDENCE_INVALID`, exact anchored binding.

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
