Independent read-only W-D2G B01-B04 identity design audit. Review ONLY exact embedded bytes; no tools, skills, delegation, external pages or workspace files. Requested GPT-6.1 Sol high and Grok 4.7 xhigh independently. This is a proposed design, not an adopted registry or verified consumer. Check Source agreement versus selected action versus Core program distinction; five nominal roles, contextual stage/episode wrapper, scoped registry keys and common identifier domains; recommended strict mapping A against alias mapping B; 31 proposed hostile/control cases and first-failure schedule; compatibility with frozen W-D2F two-step design and existing Source/Core/wire/effect contracts. Find high/medium contradictions, invalid evidence-versus-valid-different-fact confusion, false authentication or migration claims. Separate usefulness of design from normative W-D2/W-D3/Sprint1 closure. No runtime identity tests or authenticating providers exist.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/identity-binding/SPEC.md",
    "bytes": 16151,
    "sha256": "2e8447bda7de825400ee91234b00b3e898d7faa0d8444ced6814a7efc1b79995"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/identity-binding/DECISION-MATRIX.md",
    "bytes": 15911,
    "sha256": "731397d98130565e4a4c535d9b5d64f6faeca12156c5a041d9bd50e29c17bb4e"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/identity-binding/RESULT.md",
    "bytes": 4671,
    "sha256": "e7d3cabb524c6380daa9f4d1e31fc66be1f63726e0b3e163c788e45658f27b3c"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md",
    "bytes": 32338,
    "sha256": "bbd5a2378303b455650fad0aef4c819b8e974ef7eb5dfe75c03c926406504e58"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md",
    "bytes": 12914,
    "sha256": "bc9320f5f35feed677739d5c9d2ffd3c4853d39dd801990e37d9aff009e5a6f0"
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
    "path": "experiments/moriarty-language/formal/mil4/wire/SPEC.md",
    "bytes": 9579,
    "sha256": "644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md",
    "bytes": 8671,
    "sha256": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json",
    "bytes": 131232,
    "sha256": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93"
  }
]
```

## experiments/moriarty-language/formal/mil4/identity-binding/SPEC.md

sha256: `2e8447bda7de825400ee91234b00b3e898d7faa0d8444ced6814a7efc1b79995`

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

```

## experiments/moriarty-language/formal/mil4/identity-binding/DECISION-MATRIX.md

sha256: `731397d98130565e4a4c535d9b5d64f6faeca12156c5a041d9bd50e29c17bb4e`

```text
# W-D2G candidate mappings and hostile identity cases

**Status: proposed / specified-only.** No case below was executed. Positive
controls mean a proposed isolated identity predicate holds, not authenticated
consumption. No real registry proof or valid full Source-to-wire fixture is
being supplied. Recommendation is awaiting independent consequential review.

## Candidate comparison

| Criterion | A: strict selectors plus stage/episode wrapper (recommended) | B: separate logical action with explicit alias registry |
| --- | --- | --- |
| Agreement | Signed agreement equals Source `programId`; domain-qualified instance registration | Same literal mapping; optionally requires a separate future definition/instance design |
| Action/program | Two nominal roles with equal builtin spelling in current S0; explicit allowed triple still required | Signed action names a logical action such as `Action`; registry maps it to Source builtin and Core program |
| Stage/episode | Required wrapper claims with scoped registry relations | Same wrapper and scoped relations |
| Source/6 impact | Closed selector parser and lowerer remain compatible | Source parser still uses builtin; signed action differs, so W-D2F direct action comparison must be revised by explicit versioned contract |
| Existing W-D2E `Action` labels | Preserve as equality fixtures; require new full-path identity vectors | Could fit alias shape, but old fixtures still have no alias proof, Source image, signature or snapshot evidence |
| Additional types | Five nominal sorts and one two-field wrapper | Same five sorts; explicit alias relation. A Source logical-action carrier would require a future source version |
| Binding strength | No implicit aliases; smaller equivalence relation | Needs unique forward target for each fully scoped alias and immutable revision binding; target sharing does not imply alias equality |
| Compatibility risk | Rejects historical labels as full-path positives; source templates cannot silently become distinct instances | Changes an existing literal check and adds contextual alias semantics; unsupported by the current W-D2F contract |
| Extensibility | Bounded two-builtin S0 only | Supports many logical actions selecting one builtin, once separately designed and adopted |

**Recommendation (inference from repository observations): A.** It resolves
the nominal-role collision with the smallest new carrier and preserves the
actual Source parser, Core builtin checks and W-D2F literal rules. Stage and
episode receive explicit claims and relation obligations. No old equality
fixture gains authentication. B is a legitimate later design if logical
action aliases are required, but merely copying `Action` from a fixture cannot
adopt it. No vote or external audit is claimed.

For B, the proposed alternative would replace only the action literal rule:
`(domain,agreement,episode,stage,wire.actionId)` resolves at tag6 to exactly one
`(Source.selected.actionId,expectedCoreProgramId,kind)` target. Retain the
authenticated expected ID, then compare signed coreProgramId and actual lowered
Core.intent.programId at tag10; a submitted Core ID is not an early registry
verification input. It must not resolve an unqualified global string. The current W-D2F
tag6 direct literal would reject before that alternative relation could run;
therefore B requires an explicit successor contract and new expectations.

## Exact proposed isolated baseline

Use this symbolic identity input; the hex string is an anchor claim, not proof.
There is no synthetic authenticated fixture or runtime verifier here.

```json
{
  "source": {
    "domain": "D",
    "programId": "AgreementA",
    "selected": {"actionId": "TransferLiteralFee"},
    "signedActionKind": "Transfer"
  },
  "core": {"domain": "D", "programId": "TransferLiteralFee", "kind": "Transfer"},
  "context": {"stageId": "Stage1", "episodeId": "Episode1"},
  "wire": {
    "domain": "D", "agreementId": "AgreementA", "stageId": "Stage1",
    "episodeId": "Episode1", "actionId": "TransferLiteralFee",
    "coreProgramId": "TransferLiteralFee", "operationKind": "transfer",
    "preHead": "5555555555555555555555555555555555555555555555555555555555555555"
  },
  "proposedRelations": {
    "agreementKey": ["D", "AgreementA"],
    "episodeKey": ["D", "AgreementA", "Episode1"],
    "stageKey": ["D", "AgreementA", "Episode1", "Stage1"],
    "stagePreHead": "5555555555555555555555555555555555555555555555555555555555555555",
    "actionKey": ["D", "AgreementA", "Episode1", "Stage1", "TransferLiteralFee"],
    "selectedTarget": ["TransferLiteralFee", "TransferLiteralFee", "transfer"]
  }
}
```

The complete parser/wire envelope is a future dependency. The baseline is an
exact projection for relation design; it is not parseable Source text or a
complete wire encoder input. All unspecified nonidentity fields must be
independently fixed before running full-path tests. Registry view/proofs are
absent. Isolated expectations below assume an adopted real verifier and all
earlier fields/gates pass unless a row states otherwise.

## Exact hostile vectors and expected predicate outcomes

Mutate only the named baseline values. `EvidenceInvalid/Bnn` abbreviates the
specified `EvidenceRejected/W_D2F_EVIDENCE_INVALID` at that binding;
`FieldMismatch/f` abbreviates `BindingRejected/W_D2F_FIELD_MISMATCH` with field f.
`Unavailable/Bnn` abbreviates `BindingRejected/W_D2F_BINDING_UNAVAILABLE`.
All outcomes are specified-only, with null published post/effects.

| ID | Exact mutation | Isolated expected outcome under A; prerequisites |
| --- | --- | --- |
| G01 | No mutations; all actual providers absent | `Unavailable/B01`, field agreementId, after compatible parsing/D/literal agreement comparison. Equal text supplies no registry. |
| G02 | `wire.agreementId="TransferLiteralFee"`; Source remains AgreementA | `FieldMismatch/agreementId`, before B01. Core selector must not replace Source agreement. |
| G03 | Change Source agreement and wire agreement both to `TransferLiteralFee`; supply only a CoreProgramId record with that spelling as B01 evidence | `EvidenceInvalid/B01`. Matching strings across sorts are not an agreement registration. A correctly registered agreement with that spelling is an isolated valid control. |
| G04 | Wire/Source remain D,AgreementA; agreement witness key=`["OtherDomain","AgreementA"]` | `EvidenceInvalid/B01`; cross-domain same-label witness cannot satisfy D. |
| G05 | Agreement row key stays `["D","AgreementA"]` but cell namespace is AgreementB's namespace | `EvidenceInvalid/B01`; same agreement label does not permit another instance's cells. Authenticating actual cells remains B11. |
| G06 | Supply two distinct agreement records for key `["D","AgreementA"]`, one permitting TransferLiteralFee and one permitting RepayAccrualFirst | `EvidenceInvalid/B01`; never choose first/last matching record. |
| G07 | Remove context.stageId | `Unavailable/B02`, field stageId, after B01; no default Stage1. Missing claim domain check is deferred to its availability gate. |
| G08 | `context.stageId="Stage2"`; signed wire remains Stage1 | `FieldMismatch/stageId` after B01; wrapper claim cannot rewrite signed scope. |
| G09 | Wire/context remain Stage1; stage witness key=`["D","AgreementB","Episode1","Stage1"]` | `EvidenceInvalid/B02`, after B01; cross-agreement stage collision. |
| G10 | Wire/context remain Episode1; stage witness key=`["D","AgreementA","Episode2","Stage1"]` | `EvidenceInvalid/B02`, after B01; stage label scoped to another episode. |
| G11 | Duplicate stage key `["D","AgreementA","Episode1","Stage1"]` with anchors respectively 64 copies of `5` and 64 copies of `6` | `EvidenceInvalid/B02`; conflicting occurrence registration. |
| G12 | Unique authenticated stage record at the same scoped key has stagePreHead equal to 64 copies of `6`; its genuine provider proof establishes that value. Signed/Source preHead remains 64 copies of `5` | B02 succeeds at tag4 and retains expected preHead. After all intervening gates pass, `FieldMismatch/preHead` at tag18 against retained value; a valid different anchor is not invalid evidence. |
| G13 | Remove context.episodeId; remaining stage evidence otherwise available | `Unavailable/B03`, field episodeId, after valid B01/B02. B02 uses the signed episode ID; a Source episode slot is not fabricated. Missing wrapper episode is not guessed. |
| G14 | Context/wire both Episode1 and B02 valid; episode witness key=`["D","AgreementB","Episode1"]` | `EvidenceInvalid/B03`; duplicate episode label in another agreement is not the same episode. |
| G15 | B02 valid; two conflicting episode rows under `["D","AgreementA","Episode1"]` with different initial history anchors | `EvidenceInvalid/B03`; continuing episode cannot have ambiguous genesis identity. History authenticity is still separate. |
| G16 | `wire.actionId="Action"`; Source remains TransferLiteralFee | `FieldMismatch/actionId` at tag6 after B01–B03; exact current W-D2E label is not an implicit alias. |
| G17 | Set Source.selected.actionId to `Action` in an otherwise valid Transfer Source document | Existing `SourceRejected/SOURCE6_PROFILE_UNSUPPORTED` before D/M, from parser's builtin constraint. No B04 alias lookup occurs. |
| G18 | `wire.coreProgramId="RepayAccrualFirst"`; action, Source/core and the valid registry target remain TransferLiteralFee | `FieldMismatch/coreProgramId` at tag10 after valid B04 at tag6 and every intervening gate, including B05. B04 retains expected TransferLiteralFee without comparing this changed signed field early. If B04 is absent, `Unavailable/B04` wins at tag6. |
| G19 | Wire tuple unchanged; action witness key=`["D","AgreementA","Episode2","Stage1","TransferLiteralFee"]` | `EvidenceInvalid/B04`; contextual action collision, after B01–B03. |
| G20 | Two targets under exact actionKey: `["TransferLiteralFee","TransferLiteralFee","transfer"]` and `["RepayAccrualFirst","RepayAccrualFirst","repayment"]` | `EvidenceInvalid/B04`; ambiguous selected relation. |
| G21 | Wire/source and valid registry target unchanged, claimed lowered Core.programId=`RepayAccrualFirst` and Core.kind=Transfer | `FieldMismatch/coreProgramId` at tag10 against the actual lowered selector, after valid B04 and all intervening gates. B04 retains the registry expected ID at tag6. Existing Core isolated prepare rejects `S0_STAGE_UNSUPPORTED`; the real current lowerer cannot produce this output from the baseline. |
| G22 | `wire.agreementId="agreementa"`, Source remains AgreementA | `FieldMismatch/agreementId`; no case folding. |
| G23 | `wire.stageId="Stage1 "`, context Stage1 | Wire codec rejects `ID` at F-wire, before adapter checks; no trimming. |
| G24 | `wire.stageId="Stage/1"`, context Stage1 | Existing wire alphabet admits it; W-D2F D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field stageId, before any B01 lookup. |
| G25 | `wire.stageId="St\u0430ge1"` (Cyrillic a), context Stage1 | Wire codec rejects `ID` at F-wire; no Unicode confusable alias. |
| G26 | `wire.stageId="action"` | Wire codec admits it; D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field stageId, because `action` belongs to Source's exact reserved set. |
| G27 | Keep all baseline literals; agreement/stage witnesses use one registry view and episode/action witnesses use another unbound view | First relation unable to establish same-view provenance rejects at its own anchor; if B01 and B02 pass, differing episode witness gives `EvidenceInvalid/B03`. Hash/view equality is not trust. |
| G28 | `context.episodeId="Episode2"`; signed wire remains Episode1; B01/B02 valid under signed context | `FieldMismatch/episodeId` at tag5. Stage binding at tag4 uses signed episode scope; wrapper episode claim cannot rewrite it. |
| G29 | Wire/source/core unchanged; a unique authenticated registry relation at the same actionKey has selectedTarget `["RepayAccrualFirst","RepayAccrualFirst","repayment"]`, and its genuine provider proof establishes that target | B04 succeeds at tag6 and retains expected RepayAccrualFirst. After intervening gates pass, `FieldMismatch/coreProgramId` at tag10 against that expected ID. The authenticated different target is not invalid evidence. The later constructor difference is not reached. |
| G30 | Claimed stagePreHead becomes 64 copies of `6`, but the unchanged provider proof authenticates the baseline record whose stagePreHead is 64 copies of `5` | `EvidenceInvalid/B02` at tag4: claimed record differs from authenticated proof content. Unlike G12, the proof does not establish the supplied fact. Actual proof bytes require the future selected format. |
| G31 | Claimed selectedTarget becomes `["RepayAccrualFirst","RepayAccrualFirst","repayment"]`, but the unchanged provider proof authenticates baseline target `["TransferLiteralFee","TransferLiteralFee","transfer"]` | `EvidenceInvalid/B04` at tag6: claimed relation differs from authenticated proof content. Unlike G29, the proof does not establish the supplied target. Actual proof bytes require the future selected format. |

## Structural collision and alias controls

These compare registry-key structure only, before any chosen byte/hash format.

- `["D","AB","C","Stage1"]` and `["D","A","BC","Stage1"]`
  are different stage keys despite equal `AB+C` and `A+BC` concatenations.
  Do not construct either key by concatenation. Both may be valid independently;
  a witness for the second cannot discharge the first.
- Stage key `["D","AgreementA","Episode1","Stage1"]` and episode key
  `["D","AgreementA","Stage1"]` have different product sorts. A stage
  proof cannot discharge B03 even when a field happens to share the label.
- Under alternative B only, exact alias key
  `["D","AgreementA","Episode1","Stage1","Action"]` targets
  `["TransferLiteralFee","TransferLiteralFee","transfer"]`. It is an
  isolated valid relation control in B and rejects the tag6 literal in A.
  A global alias `Action -> TransferLiteralFee`, a target from AgreementB,
  or two targets for that exact key must reject B04 under B too.
- Under A, coherent replacement of Source selected action, signed action,
  signed program, actual Core kind/program, and operation constructor with
  `(RepayAccrualFirst,RepayAccrualFirst,repayment)` is an isolated compatible
  identity control only if the same contextual registration explicitly permits
  it. It requires an independently valid repayment Source/state/effect fixture;
  changing labels alone cannot turn a transfer fixture into a repayment.
- Equal labels across distinct scoped stage/episode keys are permitted.
  Reusing an ID at two anchors within the same episode is prohibited by the
  proposed unique stage-occurrence rule. This does not demonstrate concurrency,
  replay exclusion or actual history validity.

## Multiple-defect controls

Keep W-D2F's domain-before-binding and anchored-literal-before-binding rules.
With B01 absent, G24's domain defect wins. With B01 absent and the compatible
G16 action mismatch, unavailable B01 wins at tag3. With B01 absent and G02,
agreement literal mismatch wins at tag3. With B04 absent and G18, unavailable
B04 wins at tag6 before tag10. With valid B04 but unavailable B05, G18 reports
unavailable B05 at tag8 before its tag10 mismatch. With valid B04 and all
intervening gates passed, G18 reports the signed field mismatch at tag10.
With G18 plus G29, B04 still establishes the genuine Repay registry target;
the signed Repay ID first differs from Source Transfer at tag10, producing
FieldMismatch/coreProgramId. With G18 plus G31, invalid B04 proof content wins
at tag6. With G12 plus unavailable B04, the missing B04 at tag6 precedes G12's
tag18 field mismatch. With G12 plus G30, invalid B02 proof content wins at tag4.
These two-step expectations align with a pending W-D2F proposal; no final
review or adoption is asserted. No downstream isolated result should be
relabeled as a full-path outcome.

```

## experiments/moriarty-language/formal/mil4/identity-binding/RESULT.md

sha256: `e7d3cabb524c6380daa9f4d1e31fc66be1f63726e0b3e163c788e45658f27b3c`

```text
# W-D2G identity design sprint result

**Status: proposed / specified-only.** Completed deliverable is a bounded
design draft for B01–B04. No authenticated implementation, empirical identity
test result, independent audit, adopted decision or acceptance-gate closure is
claimed.

## Findings and output

Repository observation: Source agreement `programId` and Core selected-builtin
`programId` have different roles. Source's selected action lowers to Core's
program selector; stage/episode have no Source/Core slots. W-D2E commits a
supplied image with neither those identity roles nor a Source registry relation.
Existing W-D2E `actionId=Action` fixtures cannot satisfy the existing W-D2F
direct action comparison with closed Source/6 selectors.

- [SPEC.md](SPEC.md) inventories present identities, proposes five nominal
  sorts and a required stage/episode wrapper, defines contextual registry
  obligations and names open authentication/migration questions.
- [DECISION-MATRIX.md](DECISION-MATRIX.md) compares strict selectors and explicit
  alias mapping, recommends strict selectors provisionally, and specifies
  exact collision, namespace, stale-anchor, duplicate-registration, alias and
  multiple-defect controls. Every test is specified-only.

Recommendation: preserve Source/6's current builtin selectors and introduce
explicit contextual stage/episode claims. Register each nominal role under a
component-wise scoped key. Same spelling across nominal sorts can be valid,
but evidence for one sort cannot prove another. An alias design would require
an explicit successor to the current W-D2F literal contract.

## Evidence and limits

Inspected repository HEAD:
`983a4bb49e3ccae399ee2514da2f6fa3f03593fd`.
This shared worktree includes uncommitted and untracked inputs; HEAD does not
commit the inspected Source/6/Core/5 or MIL/4 experiment files. No immutable
review freeze or source hash receipt is asserted for this proposal.

Primary repository inputs: effect-consumer FIELD-MAP.md and PLAN.md;
financial-agreement-source-v6-frontend.ts (AST, selected builtin constraint,
lowerer); mil4-s0-core-v5.ts (types, builtin/kind checks); mil4-s0-source-v6.ts
(unverified wrapper result); wire SPEC.md; effect-wire SPEC.md and fixtures.json.
No external sources were acquired. No runtime tests or experiments were run.
Document inspection checked the proposed expectations against parser diagnostics
and W-D2F anchor order. The actual wrong-selector parser code is
`SOURCE6_PROFILE_UNSUPPORTED`; stage/episode claims remain absent from Source.
The proposal separates B02 stage-fact authentication at tag4 from signed
preHead comparison at tag18, and B04 registry-fact authentication at tag6 from
signed/actual Core-program comparison at tag10. G12/G29 specify valid different
provider facts followed by field mismatches at tag18/tag10. G30/G31 separately
specify claimed facts that disagree with their unchanged proof content, which
reject as invalid evidence at tag4/tag6. G18/G21 retain tag10 mismatches with
valid earlier evidence. This aligns with a pending W-D2F repair and claims
neither its final review nor adoption. All 31 cases remain specified-only;
concrete proof bytes await the selected provider format.

The development CLI `status --json` was inspected in this checkout. It reported
SP01.6 loan-swap-subset blocked by stale admission inputs, missing current
accounting and unavailable resource live state, with no pending transactions.
That operational state does not establish or deny any identity relation; this
authorized document task dispatches no registered campaign.

The graphify skill was inspected; this checkout has no graphify-out graph.
A new graph build would write outside the delegated directory and was not
performed. Primary source inspection provides the observations above.
Brainstorming was applied as context inspection and candidate comparison; the
AFK authority and delegated output scope select a draft, without user questions,
implementation, commits or a claim of adopted approval.

## Remaining work

Independent consequential review is required before adopting A or B. Real
registry authority/view selection, snapshot association and verifier behavior
are missing. B05–B07 hash images/lowering, B08–B16 authentication and B17 ledger
consumption remain separate obligations. Source template/instance reuse,
episode genesis, stage allocation, identity fields in the effect image and
cross-instance replay policy remain explicitly open.

All created files are inside identity-binding/. No effect-consumer inputs,
Source/Core modules, wire fixtures, historical evidence or gate records were
modified by this task.

```

## experiments/moriarty-language/formal/mil4/effect-consumer/FIELD-MAP.md

sha256: `bbd5a2378303b455650fad0aef4c819b8e974ef7eb5dfe75c03c926406504e58`

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
| header | schemaVersion=/3 | Source profile is /6; lowerer chooses /3 discriminator | intent.version=/3 | independent effects/1 header | Existing decoder fixes /3; no /4 fallback; version labels do not authenticate lowering |
| 1 | profile=s0-provisional/1 | /6 closed S0 grammar | Core/5 S0 selection | operationKind transfer/repayment | Explicit profile compatibility required; only this intersection is proposed |
| 2 | domain | ast.domain; state.domain; intent.domain | checks state/intent equality | root.domain and replay components | Direct equality; domain membership and ledger network mapping unverified |
| 3 | agreementId | ast.programId retained | absent | absent | Direct literal agreementId==AST.programId comparison available; B01 authenticated agreement-instance binding unavailable; never map to Core.intent.programId |
| 4 | stageId | absent | absent | absent | B02: authenticated stage identity and uniqueness unavailable |
| 5 | episodeId | absent | absent | absent | B03: authenticated episode/history identity unavailable |
| 6 | actionId | Direct equality with ast.selected.actionId available; distinct wire/coreProgramId relation unselected | programId contains Source selected action | absent | Direct literal action comparison available; B04 authenticated selected action/program relation unselected; W-D2E Action differs from Source's supported selected IDs |
| 7 | sourceVersion=6 | exact profile /6 | sourceProfile=/6 | absent | Existing literals correspond by version; no hash/lowering authenticity follows |
| 8 | sourceHash | ast.selected.sourceHash, opaque string; copied to intent | only validates opaque string | absent | Direct literal claim equality available; B05 canonical source hash image and source-to-selection verification undefined |
| 9 | coreVersion=5 | lowerer chooses Core/5 | state.core, intent.core | core=/5 | Version equality only; B06 additionally required for exact selected code |
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
| 20 | nonce | ast.intent.nonce opaque; lowerer composite replay tuple | replay key=(domain, signer, nonce) | typed replay tuples and histories | Direct equality on exact 64 lowercase hex text; B13: unused-key authentication/durable consumption unavailable |
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
| state.consumedReplay / candidatePost.consumedReplay | replayBefore/replayAfter | Preserve complete histories in order, append exact signed tuple; B11/B13 |
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
| B13 | Replay-unused evidence for full domain/signer/nonce tuple and durable uniqueness policy |
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

Scan tags 1–35. At each tag, check direct Source literals first, authenticate
any provider facts anchored there, then compare **only this tag's signed value**
to the retained authenticated facts. An anchor authenticates provider integrity,
scope and availability now; it does not compare later signed fields early.
Authentication is never deferred. Later signed equality is performed at that
field's tag, and every equality must pass before C can execute.
Binding anchors are tag3 B01; tag4 B02; tag5 B03; tag6 B04;
tag8 B05; tag11 B06; tag12 B07; tag13 B08; tag14 B09; tag16 B10; tag18 B11;
tag19 B12; tag20 B13; tag23 B14. After tag35 and its nested fields, check B15 at
successor, then B16 at replayBefore. Retained contexts are immutable verifier
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
| B13 nonce/20 | Authenticate unused replay-key fact, scoped to prior domain/signer/head | Compare current nonce; retain unused tuple/history facts for tail/B16 |
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

Direct literal mappings use the table above, with version/policy fixed literals
already canonical and corresponding Source forms checked. Source agreement and
selected action names are literal comparisons, independent of registry trust.
At coreProgramId, compare Source selected.actionId then lowered.intent.programId;
equal strings do not establish B04/B06. sourceHash/policyHash compare claims,
not a computed image; keyRef-to-key comparison requires B08's adopted reference
resolution and is never a guess based on matching opaque text. Missing Source
stage/episode slots require adopted B02/B03 wrapper evidence, not invented IDs.

Head precedence at tag18: wire.preHead versus AST.intent.preHead first, then
AST.authenticated.head; then authenticate B11's declared snapshot fact and compare
its retained authentic head to the current signed preHead. A valid proof of a
different head yields FIELD_MISMATCH/preHead at18; a proof that does not authenticate
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

```

## experiments/moriarty-language/formal/mil4/effect-consumer/PLAN.md

sha256: `bc9320f5f35feed677739d5c9d2ffd3c4853d39dd801990e37d9aff009e5a6f0`

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
authenticatedSnapshot, signatureEvidence, selectionEvidence,
headExtensionEvidence, submittedEffectImage})`.

Evidence parameters name future verifier outputs. Their constructors, trust
roots and verification are absent. A JSON object or a caller Boolean is not a
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
4. **M:** signed tags in order. At each tag, perform declared direct literal
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
   reuse is a B13 evidence failure in this verified path; isolated Core reuse
   tests retain their own history rejection.
6. **C:** existing read-only Core prepare with the exact verified snapshot,
   bound signed intent, ordered submitted Source effects, independent successor
   and terminal-success outcome. Core derives internally before sameEffects.
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

## experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json

sha256: `23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93`

```json
{
  "status": "W-D2E independent expectations frozen before JS codec; no acceptance",
  "positive": [
    {
      "id": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "effectCommitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
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
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "1"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "11",
            "spent": "0"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "1"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "89"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "1"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "11"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
        ]
      },
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e"
      }
    },
    {
      "id": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "effectCommitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d",
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
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "0"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "20",
            "spent": "3"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "0"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "90"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "10",
            "spent": "13"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
        ]
      },
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
      }
    },
    {
      "id": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
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
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "30",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "30",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "70"
          },
          {
            "account": "C",
            "amount": "30"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "70",
            "spent": "30"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888"
      }
    },
    {
      "id": "repay-accrued-only",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "5"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "95"
            },
            {
              "account": "C",
              "before": "0",
              "after": "5"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "95",
              "spentAfter": "5"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "1000",
              "accruedAfter": "5",
              "outstandingAfter": "1005",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
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
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "5",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "5",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "95"
          },
          {
            "account": "C",
            "amount": "5"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "95",
            "spent": "5"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000502000143000141000000000000000000000000000000050300014c000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed010400014f000000000000000000000000000000050500014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000000640000000000000000000000000000005f0001430000000000000000000000000000000000000000000000000000000000000005000100014f00000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000005f00000000000000000000000000000005000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a0f",
        "commitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa"
      }
    },
    {
      "id": "repay-settled",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1010"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1010",
              "after": "0"
            },
            {
              "account": "C",
              "before": "0",
              "after": "1010"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1010",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "1010"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "0",
              "accruedAfter": "0",
              "outstandingAfter": "0",
              "statusAfter": "Settled"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
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
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1010",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1010"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1010",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1010",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "1010"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "1010"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f000141000000000000000000000000000003f202000143000141000000000000000000000000000003f20300014c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020400014f000000000000000000000000000003f20500014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000003f20000000000000000000000000000000000014300000000000000000000000000000000000000000000000000000000000003f2000100014f000000000000000000000000000003f20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f2000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f20100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b0f",
        "commitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90"
      }
    },
    {
      "id": "repay-near-bound",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1",
              "after": "0"
            },
            {
              "account": "C",
              "before": "340282366920938463463374607431768211454",
              "after": "340282366920938463463374607431768211455"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1",
              "spentBefore": "340282366920938463463374607431768211454",
              "remainingAfter": "0",
              "spentAfter": "340282366920938463463374607431768211455"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "170141183460469231731687303715884105726",
              "accruedBefore": "1",
              "outstandingBefore": "170141183460469231731687303715884105727",
              "statusBefore": "Outstanding",
              "principalAfter": "170141183460469231731687303715884105726",
              "accruedAfter": "0",
              "outstandingAfter": "170141183460469231731687303715884105726",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
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
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211454"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1",
            "spent": "340282366920938463463374607431768211454"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "1",
            "outstanding": "170141183460469231731687303715884105727",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211455"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "340282366920938463463374607431768211455"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000102000143000141000000000000000000000000000000010300014c7ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe010400014f000000000000000000000000000000010500014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000000100000000000000000000000000000000000143fffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffff000100014f00000000000000000000000000000001fffffffffffffffffffffffffffffffe00000000000000000000000000000000ffffffffffffffffffffffffffffffff000100014c00014f0001430001417ffffffffffffffffffffffffffffffe000000000000000000000000000000017fffffffffffffffffffffffffffffff017ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c0f",
        "commitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43"
      }
    }
  ],
  "hostile": [
    {
      "id": "changed-debit",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "12"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000c020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a413899541c60a5118f898f971f782acf3ea55dbd826ab5489d66753d32ba173"
      }
    },
    {
      "id": "dropped-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "053cd21be0ff7e2b84fcec4d3722c887d333bfe53f8b6e9d5a2cf6458f2bcd86"
      }
    },
    {
      "id": "appended-line",
      "base": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0200014600014100000000000000000000000000000001000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "b497a80a6ee91da5db6828700fc8f68f47a478855678fa3e0dd6b569a76a52d9"
      }
    },
    {
      "id": "line-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0006065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0500014400014f77777777777777777777777777777777777777777777777777777777777777770400014f0000000000000000000000000000000b0200014600014100000000000000000000000000000001020001520001410000000000000000000000000000000a0100014f0001410000000000000000000000000000000b000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8b7bd1c1aafd1aca2952f85e4ddbd8bbb5276d0f798ad29e7f97e3b2ba24fba8"
      }
    },
    {
      "id": "recipient",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "X",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001580001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e5ef93825658e880d7502fac2467d6a05766cfbf439c9733a40c28700f6f8b72"
      }
    },
    {
      "id": "allowance-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000a0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "af174da5ed1d16d89f089c62b7e89837ddff931300052780a3b9db1349689a59"
      }
    },
    {
      "id": "work-counter",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "9",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000090000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "fb76a96c769d0581d2d1c4956a0b2b9beec45e1c6994eb95d70ee6ce16c31809"
      }
    },
    {
      "id": "omitted-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 548,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a04cf590396a7e6124787589f58cba97971900b730715fe57fe9e04f9395dbcb"
      }
    },
    {
      "id": "balance-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "88"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000058000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "f7880298148d0bd4a5a866577c0bcf1804f7f4bacb91a45f05e2d8ef0f7c8090"
      }
    },
    {
      "id": "allowance-prestate",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "1",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "afd61c0a33c4253f0445f35107ad928073de793635845383bd773434dab14b4f"
      }
    },
    {
      "id": "obligation-debtor",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "X",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c000158000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e0597fa8d561e1b7895475c3078885309024eda2bce0285e3377e4bb6c2e23b4"
      }
    },
    {
      "id": "principal-allocation",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "990",
            "accrued": "0",
            "outstanding": "990",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003de00000000000000000000000000000000000000000000000000000000000003de010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "c90dc47483729707d96a5cd666fa6248ceb9c45ed35c7846b33d301b78aecb6b"
      }
    },
    {
      "id": "replay-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "77fd5da6b511b9b3b97f8c130ac51a5606c733cf7d09c6157abe241e18bce8fb"
      }
    },
    {
      "id": "replay-history",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [
            "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          ],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 621,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b000000000000000000000000000000000002000000000000000000000000000000070000000000000000000000000000000100000000000000000000000000000008000100014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8c5e38faa8bae29fdaae2d64cbb6380d2e6f258e30508f288c22ef20eebda227"
      }
    },
    {
      "id": "replay-domain",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"X\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500015800014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "60a2d27b93beb5eed67299ccb97a3121cc90420569e55a693f889da88df14a46"
      }
    },
    {
      "id": "root-head",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f31000500014400014102010000000000000064bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "1d8175071b704f8af1efecb1c355c07ab04939aa7a114e98070c587294e50fa3"
      }
    },
    {
      "id": "successor-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e782c133039f5d75b7dc2f6eea415234bab5f02d3e2293dab95987552dc423b2"
      }
    },
    {
      "id": "footprint-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "F",
              "before": "0",
              "after": "1"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "O",
              "before": "100",
              "after": "89"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00030001460000000000000000000000000000000000000000000000000000000000000001000152000000000000000000000000000000000000000000000000000000000000000a00014f0000000000000000000000000000006400000000000000000000000000000059000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "66de3c01954550edf4d0f78af534a9a6ca358bd803afc4a7c130f8b01d14812b"
      }
    },
    {
      "id": "premise-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "atomic-ledger-compare-and-consume",
          "head-extension",
          "snapshot-to-head",
          "canonical-intent-signature"
        ]
      },
      "expectedCode": "LITERAL"
    }
  ]
}

```

