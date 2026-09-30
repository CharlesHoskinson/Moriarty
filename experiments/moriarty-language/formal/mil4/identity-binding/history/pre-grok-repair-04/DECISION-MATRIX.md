# W-D2G candidate mappings and hostile identity cases

**Status: proposed / specified-only.** No case below was executed. The numbered
positive control means a proposed isolated identity predicate holds, not authenticated
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
    "selected": {
      "actionId": "TransferLiteralFee",
      "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
      "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444"
    },
    "intent": {"preHead": "5555555555555555555555555555555555555555555555555555555555555555"},
    "authenticated": {"head": "5555555555555555555555555555555555555555555555555555555555555555"},
    "settlementAsset": "A",
    "signedActionKind": "Transfer"
  },
  "core": {"domain": "D", "programId": "TransferLiteralFee", "kind": "Transfer"},
  "context": {"stageId": "Stage1", "episodeId": "Episode1"},
  "wire": {
    "domain": "D", "agreementId": "AgreementA", "stageId": "Stage1",
    "episodeId": "Episode1", "actionId": "TransferLiteralFee",
    "coreProgramId": "TransferLiteralFee", "operationKind": "transfer",
    "asset": "A",
    "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
    "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
    "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
    "preHead": "5555555555555555555555555555555555555555555555555555555555555555"
  },
  "proposedRelations": {
    "agreementKey": ["D", "AgreementA"],
    "episodeKey": ["D", "AgreementA", "Episode1"],
    "stageKey": ["D", "AgreementA", "Episode1", "Stage1"],
    "stageAssociationRef": "StageAnchor1",
    "registryViewRef": "RegistryView1",
    "agreementAssociations": {
      "stageIds": ["Stage1"], "episodeIds": ["Episode1"],
      "actionIds": ["TransferLiteralFee"],
      "coreProgramIds": ["TransferLiteralFee"], "assetIds": ["A"]
    },
    "episodeStageKeys": [["D", "AgreementA", "Episode1", "Stage1"]],
    "actionKey": ["D", "AgreementA", "Episode1", "Stage1", "TransferLiteralFee"],
    "selectedTarget": ["TransferLiteralFee", "TransferLiteralFee", "transfer"],
    "imageReferences": {"source": "SourceImage1", "core": "CoreImage1", "policy": "PolicyImage1"},
    "expectedArtifactHashes": {
      "source": "2222222222222222222222222222222222222222222222222222222222222222",
      "core": "3333333333333333333333333333333333333333333333333333333333333333",
      "policy": "4444444444444444444444444444444444444444444444444444444444444444"
    },
    "expectedSnapshotFacts": {
      "core": "moriarty-core/5", "domain": "D", "asset": "A",
      "head": "5555555555555555555555555555555555555555555555555555555555555555"
    }
  }
}
```

The complete parser/wire envelope is a future dependency. The baseline is an
exact projection for relation design; it is not parseable Source text or a
complete wire encoder input. All unspecified nonidentity fields must be
independently fixed before running full-path tests. Registry view/proofs are
absent. StageAnchor1 and RegistryView1 are association references, never
independent head authorities or trustworthy strings. B11 must authenticate
their linkage to the selected snapshot. Isolated expectations below assume an adopted real verifier and all
earlier fields/gates pass unless a row states otherwise.
The explicit expected artifact hashes are conditional provider-fact values,
not hashes computed from supplied images. Exact Source/Core/policy image bytes,
hash definitions and proof formats remain unavailable. Source hash/policy
claims are preserved separately from these expected artifact values; Core has
no invented coreHash slot. Both Source head paths are explicit claims and must
match wire.preHead before any B11 outcome below is reachable.

The positive G21 control stops after tag6. It covers the B01–B04 reachable
fragment and B01 membership checks at4/5/6 only. Later comparisons are separate
conditional predicates: tag10 requires B05 at8 and every earlier gate to pass;
tag16 additionally requires B06/B07/B08/B09 and B10 at16. Only under those
explicit preconditions may the baseline's later B01 Core-program/asset
memberships and B04 program equality be checked. This packet supplies none of
those later providers and reports no reachable positive at10/16.

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
| G03 | Change Source agreement and wire agreement both to `TransferLiteralFee`; supply only a CoreProgramId record with that spelling as B01 evidence | `EvidenceInvalid/B01`. Matching strings across sorts are not an agreement registration. This row specifies only rejection; no extra unnumbered positive control is implied. |
| G04 | Wire/Source remain D,AgreementA; agreement witness key=`["OtherDomain","AgreementA"]` | `EvidenceInvalid/B01`; cross-domain same-label witness cannot satisfy D. |
| G05 | Agreement row key stays `["D","AgreementA"]`; claimed cell namespace becomes AgreementB's while unchanged proof authenticates AgreementA's namespace | `EvidenceInvalid/B01`; proof does not establish the claimed instance record. Actual cells and namespace-to-snapshot linkage remain B11. |
| G06 | Supply two distinct agreement records for key `["D","AgreementA"]`, one permitting TransferLiteralFee and one permitting RepayAccrualFirst | `EvidenceInvalid/B01`; never choose first/last matching record. |
| G07 | Remove context.stageId | `Unavailable/B02`, field stageId, after B01; no default Stage1. Missing claim domain check is deferred to its availability gate. |
| G08 | `context.stageId="Stage2"`; signed wire remains Stage1 | `FieldMismatch/stageId` after B01; wrapper claim cannot rewrite signed scope. |
| G09 | Wire/context remain Stage1; stage witness key=`["D","AgreementB","Episode1","Stage1"]` | `EvidenceInvalid/B02`, after B01; cross-agreement stage collision. |
| G10 | Wire/context remain Episode1; stage witness key=`["D","AgreementA","Episode2","Stage1"]` | `EvidenceInvalid/B02`, after B01; stage label scoped to another episode. |
| G11 | Duplicate stage key `["D","AgreementA","Episode1","Stage1"]` with stageAssociationRef respectively StageAnchor1 and StageAnchor2 | `EvidenceInvalid/B02`; conflicting occurrence registration, without B02 head comparison. |
| G12 | Unique authentic stage record declares stageAssociationRef=StageAnchor2. wire.preHead, source.intent.preHead and source.authenticated.head all remain 64 copies of `5`; registry/stage-to-snapshot linkage provider is absent | B02 succeeds at tag4 and retains its reference. After earlier gates and both tag18 Source literals pass, `Unavailable/B11`, field preHead, because linkage cannot be verified. B02 has no head veto. G46–G48 pin invalid proof, complete valid other-head fact and invalid linkage separately. |
| G13 | Remove context.episodeId; remaining stage evidence otherwise available | `Unavailable/B03`, field episodeId, after valid B01/B02. B02 uses the signed episode ID; a Source episode slot is not fabricated. Missing wrapper episode is not guessed. |
| G14 | Context/wire both Episode1 and B02 valid; episode witness key=`["D","AgreementB","Episode1"]` | `EvidenceInvalid/B03`; duplicate episode label in another agreement is not the same episode. |
| G15 | B02 valid; two conflicting episode rows under `["D","AgreementA","Episode1"]` with different initial history anchors | `EvidenceInvalid/B03`; continuing episode cannot have ambiguous genesis identity. History authenticity is still separate. |
| G16 | `wire.actionId="Action"`; Source remains TransferLiteralFee | `FieldMismatch/actionId` at tag6 after B01–B03; exact current W-D2E label is not an implicit alias. |
| G17 | Set Source.selected.actionId to `Action` in an otherwise valid Transfer Source document | Existing `SourceRejected/SOURCE6_PROFILE_UNSUPPORTED` before D/M, from parser's builtin constraint. No B04 alias lookup occurs. |
| G18 | `wire.coreProgramId="RepayAccrualFirst"`; action, Source/core and the valid registry target remain TransferLiteralFee | `FieldMismatch/coreProgramId` at tag10 after valid B04 at tag6 and every intervening gate, including B05. B04 retains expected TransferLiteralFee without comparing this changed signed field early. If B04 is absent, `Unavailable/B04` wins at tag6. |
| G19 | Wire tuple unchanged; action witness key=`["D","AgreementA","Episode2","Stage1","TransferLiteralFee"]` | `EvidenceInvalid/B04`; contextual action collision, after B01–B03. |
| G20 | Two targets under exact actionKey: `["TransferLiteralFee","TransferLiteralFee","transfer"]` and `["RepayAccrualFirst","RepayAccrualFirst","repayment"]` | `EvidenceInvalid/B04`; ambiguous selected relation. |
| G21 | Exact baseline identity projection; under hypothetical valid F/D inputs, four genuine unique B01–B04 same-view proofs establish its records, association sets, prior-stage membership, target and references. Core selector comes only from actual `lowerSource6` on a corresponding valid Source document | Specified positive fragment through tag6 only: B01–B04 authentication and B01 stage/episode/action membership at4/5/6 do not reject. Stop before B05 at8; no tag10/16 comparison or non-rejection is claimed. Later checks have separate conditional preconditions above. No proof bytes or observed positive exists; B05–B16/B17 remain open. |
| G22 | `wire.agreementId="agreementa"`, Source remains AgreementA | `FieldMismatch/agreementId`; no case folding. |
| G23 | `wire.stageId="Stage1 "`, context Stage1 | Wire codec rejects `ID` at F-wire, before adapter checks; no trimming. |
| G24 | `wire.stageId="Stage/1"`, context Stage1 | Existing wire alphabet admits it; D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field stageId, sourcePath=null, inputPath=wire.stageId, before any B01 lookup. |
| G25 | `wire.stageId="St\u0430ge1"` (Cyrillic a), context Stage1 | Wire codec rejects `ID` at F-wire; no Unicode confusable alias. |
| G26 | `wire.stageId="action"` | Wire codec admits it; D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field stageId, sourcePath=null, inputPath=wire.stageId, because action belongs to Source's reserved set. |
| G27 | Keep all baseline literals; agreement/stage witnesses use one registry view and episode/action witnesses use another unbound view | First relation unable to establish same-view provenance rejects at its own anchor; if B01 and B02 pass, differing episode witness gives `EvidenceInvalid/B03`. Hash/view equality is not trust. |
| G28 | `context.episodeId="Episode2"`; signed wire remains Episode1; B01/B02 valid under signed context | `FieldMismatch/episodeId` at tag5. Stage binding at tag4 uses signed episode scope; wrapper episode claim cannot rewrite it. |
| G29 | Wire/source/core unchanged; a unique authenticated registry relation at the same actionKey has selectedTarget `["RepayAccrualFirst","RepayAccrualFirst","repayment"]`, and its genuine provider proof establishes that target | B04 succeeds at tag6 and retains expected RepayAccrualFirst. After intervening gates pass, `FieldMismatch/coreProgramId` at tag10 against that expected ID. The authenticated different target is not invalid evidence. The later constructor difference is not reached. |
| G30 | Claimed stageAssociationRef becomes StageAnchor2, but unchanged provider proof authenticates baseline reference StageAnchor1 | `EvidenceInvalid/B02` at tag4: claimed stage association differs from authenticated proof content. Unlike G12, proof does not establish the supplied fact. No head comparison occurs. Actual proof bytes require the future selected format. |
| G31 | Claimed selectedTarget becomes `["RepayAccrualFirst","RepayAccrualFirst","repayment"]`, but the unchanged provider proof authenticates baseline target `["TransferLiteralFee","TransferLiteralFee","transfer"]` | `EvidenceInvalid/B04` at tag6: claimed relation differs from authenticated proof content. Unlike G29, the proof does not establish the supplied target. Actual proof bytes require the future selected format. |
| G32 | Genuine B01 fact retains stageIds=["Stage2"]; baseline wire/context/B02 remain Stage1 | `FieldMismatch/stageId` at tag4 after direct wrapper comparison and valid B02 authentication; B01 future-stage association is checked here, not tag3. |
| G33 | Genuine B01 fact retains episodeIds=["Episode2"]; baseline wire/context/B02/B03 remain Episode1 | `FieldMismatch/episodeId` at tag5 after direct wrapper comparison and valid B03 authentication; B01 episode association precedes B02's retained episode comparison. |
| G34 | Genuine B01 fact retains actionIds=["RepayAccrualFirst"]; baseline Source/wire/B04 remain TransferLiteralFee | `FieldMismatch/actionId` at tag6 after direct Source comparison and valid B04 authentication; B01 action association is not checked early. |
| G35 | Genuine B01 fact retains coreProgramIds=["RepayAccrualFirst"]; baseline Source/wire/Core/B04 remain TransferLiteralFee | `FieldMismatch/coreProgramId` at tag10 after Source/lowered literals; compare B01 association before B04's retained program target. Earlier B05 must pass. |
| G36 | Genuine B01 fact retains assetIds=["B"]; baseline wire/Source asset remains A and B10 authenticates registered A | `FieldMismatch/asset` at tag16 after direct Source literal and B10 authentication. This forward association does not itself implement B10. |
| G37 | Genuine B03 episode row at `["D","AgreementA","Episode1"]` declares episodeStageKeys=[]; B02 has authenticated baseline Stage1 occurrence | `EvidenceInvalid/B03` at tag5: episode row alone is genuine, but B03 verifier cannot establish required membership of the already authenticated stage occurrence. Do not silently accept an unrelated episode proof. |
| G38 | B04 retains source image reference SourceImage1; B05 provides a genuine artifact proof for SourceImage2 with no selected-context link to SourceImage1 | `EvidenceInvalid/B05` at tag8: unrelated artifact authentication does not establish the prior selected image linkage. This row has one mutation and one expected outcome. |
| G39 | B04 retains Core image reference CoreImage1; B06 provides a genuine artifact proof for CoreImage2 with no selected-context link to CoreImage1 | `EvidenceInvalid/B06` at tag11 after tag10 passes: unrelated Core artifact cannot establish required prior selected linkage. |
| G40 | B04 retains policy image reference PolicyImage1; B07 provides a genuine artifact proof for PolicyImage2 with no selected-context link to PolicyImage1 | `EvidenceInvalid/B07` at tag12: unrelated policy artifact cannot establish required prior selected linkage. |
| G41 | Genuine B04 target is `["TransferLiteralFee","TransferLiteralFee","repayment"]`; Source/wire/lowered selector and transfer constructor otherwise remain baseline | B04 fact authenticates at tag6 and program IDs match at tag10. With earlier gates passed, `FieldMismatch/operation.kind` at tag35 against retained constructor. Semantic allowed-triple rule is not an early proof-validity rule. |
| G42 | Genuine B04 target is `["TransferLiteralFee","RepayAccrualFirst","transfer"]`; remaining baseline unchanged | Fact authentication succeeds at tag6; `FieldMismatch/coreProgramId` at tag10 against expected RepayAccrualFirst. Incompatible authenticated target is not invalid proof. |
| G43 | context.stageId="Stage/1"; wire remains Stage1 | D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field stageId, sourcePath=context.stageId, before any B01. Present wrapper value is checked before wire stageId. |
| G44 | context.episodeId="action"; wire remains Episode1 | D rejects `W_D2F_DOMAIN_UNSUPPORTED`, field episodeId, sourcePath=context.episodeId; action is reserved. No parser offset is fabricated. |
| G45 | context.stageId="action" and wire.stageId="Stage/1"; both wire-legal strings | D rejects reserved wrapper value first: field stageId, sourcePath=context.stageId, before wire counterpart and B01. The wrapper path is a proposed extension. Wire-only variant with context Stage1 is the separate G24 case, using sourcePath=null and inputPath=wire.stageId. |
| G46 | G12 stage reference; wire.preHead, source.intent.preHead and source.authenticated.head all equal 64 copies of `5`. B11 declares head `5`×64, but supplied proof authenticates head `6`×64; core/domain/asset stay baseline and linkage provider otherwise available | `EvidenceInvalid/B11` at tag18 after both Source literals pass: proof does not establish its declared snapshot fact. No B02 reference preempts B11 authentication. |
| G47 | G12 stage reference; wire.preHead, source.intent.preHead and source.authenticated.head all equal `5`×64. Complete genuine B11 fact/linkage establish head `6`×64 with core=Core/5, domain=D, asset=A | Both Source literals pass, then B11 integrity and core/domain/asset checks pass; `FieldMismatch/preHead` at18, binding=B11, factPath=B11.snapshot.head, sourcePath=null. Other-head fact is genuine, not invalid proof. |
| G48 | G12 stage reference; wire.preHead, source.intent.preHead and source.authenticated.head all equal `5`×64. B11 snapshot head is `5`×64 with baseline core/domain/asset; claimed StageAnchor2-to-this-snapshot proof establishes only linkage to the `6`×64 snapshot | Both Source literals pass; `EvidenceInvalid/B11` at18 for invalid declared association linkage. B02 remains valid. With absent linkage provider instead, G12's Unavailable/B11 applies. |
| G49 | Genuine B02 record at the baseline scoped key retains payload episodeAssociation=Episode2; wire/context/B01 episode membership and B03 record remain Episode1. B03 genuinely proves membership of the exact previously authenticated stage occurrence | `FieldMismatch/episodeId` at tag5: wrapper literal passes, B03 authenticates, B01 Episode1 membership passes, then B02's retained Episode2 is the first mismatching fact. This is not G28's earlier wrapper mismatch or a proof-content failure. |
| G50 | G21 fragment, but one genuine B01 record has actionIds=["TransferLiteralFee","RepayAccrualFirst"] and coreProgramIds=["TransferLiteralFee","RepayAccrualFirst"]; current selection remains TransferLiteralFee | Specified positive action-membership control at tag6 after earlier fragment gates: TransferLiteralFee is a member despite the second declared action. Do not demand whole-set equality. Core-program membership at10 remains a separate conditional check requiring B05 and every earlier gate; no tag10 positive is claimed here. |
| G51 | Set wire.sourceHash and source.selected.sourceHash both to `9`×64. B04 still selects SourceImage1; a genuine linked B05 artifact fact yields baseline expected source hash `2`×64 | Earlier gates pass and direct wire/Source hash claims agree at8. Selected artifact integrity/linkage succeeds, then `FieldMismatch/sourceHash` at8, binding=B05, against its authentic `2`×64 hash. This is separate from G38's wrong-image evidence failure. |
| G52 | Set only wire.coreHash to `9`×64. B04 still selects CoreImage1; genuine B06 selected artifact/lowering evidence yields expected Core hash `3`×64 | All earlier gates including B05/tag10 pass. B06 authenticates selected CoreImage1 and correspondence, then `FieldMismatch/coreHash` at11, binding=B06, against its authentic `3`×64 hash. Source/Core have no fabricated coreHash claim slot. |
| G53 | Set wire.policyHash and source.selected.policyDigest both to `9`×64. B04 still selects PolicyImage1; genuine linked B07 fact yields expected policy hash `4`×64 | Earlier gates through11 pass and direct policy claims agree at12. B07 selected artifact integrity/linkage succeeds, then `FieldMismatch/policyHash` at12, binding=B07, against its authentic `4`×64 hash. This is separate from G40's wrong-image evidence failure. |

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
- A repayment positive identity control is outside this bounded packet. No
  unnumbered positive or coherent-label replacement is claimed. A later control
  requires its own complete identity projection, corresponding valid repayment
  Source document, actual lowerer output and explicit genuine relation-provider
  hypotheses; changing labels alone does not supply those inputs. The current
  Repay selector's observed role and the proposed compatible triple remain in
  the identity map without positive test coverage.
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
at tag6. With G12 plus unavailable B04, missing B04 at tag6 precedes unavailable
B11 at18. With G12 plus G30, invalid B02 proof content wins at tag4. At tag18 a
direct wire/Source head difference precedes B11; when these literals agree,
invalid declared snapshot proof (G46) is EvidenceInvalid/B11, unavailable
proposed linkage (G12) is Unavailable/B11, and a complete valid different-head
B11 fact (G47) is FieldMismatch/preHead. G48 cannot authenticate its complete
declared linkage and fails B11 rather than inventing a B02 head comparison.
These expectations preserve W-D2F head custody; no normative adoption is
asserted. No downstream isolated result should be relabeled as a full-path
outcome.
