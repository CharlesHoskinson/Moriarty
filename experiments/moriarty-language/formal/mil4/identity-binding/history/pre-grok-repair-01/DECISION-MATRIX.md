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
