Correct the existing two native-statement sourcecrosswalk files and rootFOREMAN_REPORT.md/.json now,600-second bound. No code/spec/register/wiki edits, shell commands, tests, builds, proofs, network, gitwrites, orsubagents. Gate remainspendingreview andmissingcurrentatomicfixture/export remainsBLOCKED. Preserve financialrows/oldhashes/oldfailureevidence. Sourcepreparationpublicationonly; no newbackenddecision. Read targeted existing sections andperform small edits, avoid wholefilerewrite. Two files under evidence/moriarty-completion-program-2026-09-07/SP01/native-statement-subset-01: statement.json and design.md. A40KiB combinedreviewbound isnowadmitted forcompletefields; do notclaimold24KiBcapmet. Finishwithbriefresult/providertransportwillrecordidentity.

Requiredcorrections:
N1 completeTerminalObservationMap: Remove stateHash from currentStateBody; addseparate currentStateEnvelope withschemaVersion'moriarty-state/1',stateHash=hash(STATE,StateBody),bodyrefcurrentStateBody. Remove traceHash from currentCompleteBodyUnbound; addseparate currentComplete mapping schemaVersion'moriarty-result/1',traceHash=hash(TRACE,CompleteBody),bodyref. ProofContext DOEScontaintraceHash but NOTproofContextHash. RemoveproofContextHashfrom currentProofContextUnbound; recordseparately derivedhash ofProofContext carried byProofAcceptanceVerdict per typed-schemas.md lines599/645/658. Hashpreimages NEVERcontaintheirdigest. Includeeachstructural/memberfield fromexactruntime-typesbelow in a documentary fieldmap; existingknownnumericprojectionstays; missingfixturevalues explicitlyunboundwithowner. Notfakewireobjects.
N2 twoStepEffects: Explicitlylabeloldflattenedeffects/actionargs as DOCUMENTARYPROJECTIONS, old wire effects are{kind,fields}. Currenteffectrecords mustcomplete typedrecords, copyexact reviewedsource-derived loan effects embeddedbelow. AllEffectRecordoperands required: DueCreated debtorcreditorandtypedAmount, DueSettled debtorcreditorassetandallsix SettlementResolutionfields asset,binding,ledgerAmount,nominalAmount,quantum,unit. Transferalso complete. Do notaddsettlement:null toDueCreated. Currentaction args mapsareprojections ofActionCall.arguments NamedStoredValue[], label accordingly. authority fieldmaps includeSIGNEDschemaVersion forbothExactPlanStatement='moriarty-exact-plan/1' andOutcomeStatement='moriarty-outcome-intent/1'. Distinguishsignatureenvelopevsstatement. No directObservationSetsigning.
N3 controls.hostSourceChecks: replace 'every financial bit' withprecise preflight_native mutationcoverage: bits0and64 for EACH11financialvalue perstate; each of8x4digestlimbs XOR1. Thesearespecified/unexecutedhere. Do notclaimexhaustive128bitmutations. Inspectalready-pinnedharness lines359–380 ifneeded.
Claims: actualfourClaimRequirement kinds ContractProperty,IntentRefinement,TransitionValidity,PredecessorHistory arecorrect. FourKINDS !=fourrequirements. Currentloanrequires sevenIDs: bounded_profile_safety_v1,loan_first_period_interest_floor_v1,loan_first_period_principal_v1,loan_first_period_settlement_exact_v1,atomic_intent_refinement_v1,bounded_atomic_transition_v1,bounded_history_compliance_v1. Nameallseven currentmanifestrequirements andpreservethehistoricalconceptualnamecrosswalkonlyasdeprecatednaming, nevercurrentIDs. No claimsdischargedbyfixedhashes.
Gateownership: Replace 'RP01-MC03 then F0/F0a'. Currentatomicbindingfixturepreparation mayproceedalongsideF0 underitsownadmission. Nativeexportauthorshipbelongs separatelyadmittedF0a afterF0go/nativepathfreeze. F2 stillrequiresacceptedatomic,reviewedRP01-MC03,allF1P1/P2/P3 andRP03. F1fixtures independentofF2financialproof; no newcycle. Aligndesignparagraph andclosureTasks atomic-context-export ownership accordingly. Missingcurrentatomic fixture/export remainsblocked.
Reports: initialGPT6candidate01BLOCKED forthese sourcefidelityfindings; parentstaticverificationpassed pinnedrows/arithmetic only; workerhasrun no commands; successor parentverify2/GPT6reviewpending; no selfapproval. Candidatehash remainsnull inauthordoc, reviews[].

Exact reviewed source-derived current effects (notnewexecution), from acceptedRP01-MC02candidate982062069f2146b98598e5a17ef214c4a0745b7adef4d00c0470575bd2fd748f:
{
  "loan-accrue": [
    {
      "kind": "DueCreated",
      "ordinal": "0",
      "dueId": "lam01:period1:PR",
      "debtor": "borrower",
      "creditor": "lender",
      "denomination": "USD_micro",
      "amount": {
        "tag": "Amount",
        "unit": "USD_micro",
        "value": "500000000"
      }
    },
    {
      "kind": "DueCreated",
      "ordinal": "1",
      "dueId": "lam01:period1:IP",
      "debtor": "borrower",
      "creditor": "lender",
      "denomination": "USD_micro",
      "amount": {
        "tag": "Amount",
        "unit": "USD_micro",
        "value": "33972602"
      }
    }
  ],
  "loan-settle": [
    {
      "kind": "Transfer",
      "ordinal": "0",
      "asset": "USD_TEST_ASSET",
      "from": "borrower",
      "to": "lender",
      "amount": {
        "tag": "Amount",
        "unit": "USD_micro",
        "value": "533972602"
      },
      "settlement": {
        "asset": "USD_TEST_ASSET",
        "binding": "USD_micro_asset",
        "ledgerAmount": "533972602",
        "nominalAmount": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "533972602"
        },
        "quantum": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "1"
        },
        "unit": "USD_micro"
      }
    },
    {
      "kind": "DueSettled",
      "ordinal": "1",
      "dueId": "lam01:period1:PR",
      "debtor": "borrower",
      "creditor": "lender",
      "denomination": "USD_micro",
      "asset": "USD_TEST_ASSET",
      "amount": {
        "tag": "Amount",
        "unit": "USD_micro",
        "value": "500000000"
      },
      "settlement": {
        "asset": "USD_TEST_ASSET",
        "binding": "USD_micro_asset",
        "ledgerAmount": "500000000",
        "nominalAmount": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "500000000"
        },
        "quantum": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "1"
        },
        "unit": "USD_micro"
      }
    },
    {
      "kind": "DueSettled",
      "ordinal": "2",
      "dueId": "lam01:period1:IP",
      "debtor": "borrower",
      "creditor": "lender",
      "denomination": "USD_micro",
      "asset": "USD_TEST_ASSET",
      "amount": {
        "tag": "Amount",
        "unit": "USD_micro",
        "value": "33972602"
      },
      "settlement": {
        "asset": "USD_TEST_ASSET",
        "binding": "USD_micro_asset",
        "ledgerAmount": "33972602",
        "nominalAmount": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "33972602"
        },
        "quantum": {
          "tag": "Amount",
          "unit": "USD_micro",
          "value": "1"
        },
        "unit": "USD_micro"
      }
    }
  ]
}
Exact runtime-types.ts pinned input:
import type {BoundsRef, BoundProgram, ClaimRequirement, NamedStoredValue, PolicyUse, Span, StoredValue} from './types.ts';
export type AmountValue=Extract<StoredValue,{tag:'Amount'}>;
export type ProgramRef={bounds:BoundsRef;coreVersion:'moriarty-core/1';profile:'moriarty-bounded-atomic/1';programHash:string;schemaVersion:'moriarty-program-ref/1';sourceHash:string};
export type ExecutionDomain={deployment:string;network:string};
export type PrincipalBinding={actor:string;principal:string};
export type ObservationBinding={authenticationPolicy:string;name:string;provider:string};
export type GenesisBody={bounds:BoundsRef;domain:ExecutionDomain;horizon:string;initialState:NamedStoredValue[];instanceId:string;lifetime:string;observationBindings:ObservationBinding[];principalBindings:PrincipalBinding[];profile:'moriarty-bounded-atomic/1';program:ProgramRef;requiredClaimRoot:string;schemaVersion:'moriarty-genesis-body/1'};
export type Genesis={body:GenesisBody;genesisHash:string;schemaVersion:'moriarty-genesis/1'};
export type ObligationRecord={amount:AmountValue;creditor:string;debtor:string;denomination:string;dueId:string;status:'Outstanding'|'Settled'};
export type Status={agreementStatus:'Outstanding'|'NoOutstanding';episodeStatus:'Open'|'Closed';remainingNotional:{amount:AmountValue;tag:'Amount'}|{tag:'NotApplicable'}};
export type StateBody=Status&{genesisHash:string;instanceId:string;obligations:ObligationRecord[];profile:'moriarty-bounded-atomic/1';programHash:string;remaining:string;revision:string;schemaVersion:'moriarty-state-body/1';values:NamedStoredValue[]};
export type StateEnvelope={body:StateBody;schemaVersion:'moriarty-state/1';stateHash:string};
export type ObservationSet={observations:{evidenceDigest:string;name:string;provider:string;value:StoredValue}[];schemaVersion:'moriarty-observations/1'};
export type ActionCall={arguments:NamedStoredValue[];name:string;schemaVersion:'moriarty-action/1'};
export type SettlementResolution={asset:string;binding:string;ledgerAmount:string;nominalAmount:AmountValue;quantum:AmountValue;unit:string};
export type EffectRecord=({amount:AmountValue;asset:string;from:string;kind:'Transfer'|'Fee';ordinal:string;settlement:SettlementResolution;to:string})|({amount:AmountValue;creditor:string;debtor:string;denomination:string;dueId:string;kind:'DueCreated';ordinal:string})|({amount:AmountValue;asset:string;creditor:string;debtor:string;denomination:string;dueId:string;kind:'DueSettled';ordinal:string;settlement:SettlementResolution});
export type WriteRecord={field:string;policy:PolicyUse;value:StoredValue};
export type AuthorityCommon={beforeStateHash:string;domain:ExecutionDomain;genesisHash:string;instanceId:string;nonce:string;predecessors:string[];principal:string;program:ProgramRef;requiredClaimRoot:string;requiredClaims:ClaimRequirement[];validity:{notBefore:string;notAfterExclusive:string}};
export type ExactPlanStatement=AuthorityCommon&{action:ActionCall;exactEffects:{effect:EffectRecord}[];exactWrites:{field:string;value:StoredValue}[];mode:'ExactPlan';schemaVersion:'moriarty-exact-plan/1'};
export type OutcomeStatement=AuthorityCommon&{allowedActions:string[];grossDebitCaps:{actor:string;asset:string;maximumLedgerAmount:string}[];minimumNetCredits:{actor:string;asset:string;minimumLedgerAmount:string}[];mode:'IntentRefinement';permittedCalls:{callee:string;selector:string}[];permittedRecipients:string[];schemaVersion:'moriarty-outcome-intent/1'};
export type Authority={domain:string;schemaVersion:'moriarty-authority/1';signature:{algorithm:string;bytes:string;keyId:string}}&({tag:'ExactPlan';statement:ExactPlanStatement}|{tag:'IntentRefinement';statement:OutcomeStatement});
export type ExternalChecks={authenticatedPrincipal:string;genesisValid:boolean;nonceFresh:boolean;observationsAuthentic:boolean;predecessorSetValid:boolean;signatureValid:boolean;stateCurrentAndUnconsumed:boolean};
export type EvaluationInput={action:ActionCall;authority:Authority;checks:ExternalChecks;genesis:Genesis;observations:ObservationSet;program:ProgramRef;schemaVersion:'moriarty-evaluation/1';state:StateEnvelope};
export type ResourceCounts={canonicalDepth:string;canonicalNodes:string;canonicalUtf8Bytes:string;effects:string;executedInstructions:string;expressionNodes:string;maximumExpressionDepth:string;unitComponents:string};
export type CompleteBody={actionHash:string;after:StateEnvelope;authorityConsumption:{mode:'ExactPlan'|'IntentRefinement';nonce:string;principal:string;statementDigest:string};beforeStateHash:string;effects:EffectRecord[];obligationDelta:{created:ObligationRecord[];settled:ObligationRecord[]};observationsHash:string;outcome:'Complete';predecessors:string[];profile:'moriarty-bounded-atomic/1';programHash:string;resourceCounts:ResourceCounts;schemaVersion:'moriarty-complete-body/1';writes:WriteRecord[]};
export type Complete={body:CompleteBody;schemaVersion:'moriarty-result/1';traceHash:string};
export type Diagnostic={code:string;message:string;primarySpan:Span;relatedSpans:Span[];stage:string};
export type Rejected={diagnostics:Diagnostic[];outcome:'Rejected';profile:'moriarty-bounded-atomic/1';programHash:string;schemaVersion:'moriarty-result/1'};
export type ProofContext={actionHash:string;authorityDigest:string;beforeStateHash:string;domain:ExecutionDomain;genesisHash:string;predecessors:string[];program:ProgramRef;requiredClaimRoot:string;schemaVersion:'moriarty-proof-context/1';traceHash:string};
/** Internal, explicitly unaccepted and non-consuming; never a public Result. */
export type Simulation={kind:'Simulation';candidate:Complete;context:ProofContext};
export type SourceBinding={source:string|Uint8Array;bounds:string|Uint8Array};
/** Deployment-owned trust boundary, never deserialize or accept this from a client.
 * MC05 must provide actual cryptographic/claim verification. MC04 must provide
 * durable atomic current-state+nonce consumption. This package supplies neither.
 * verifyAndCommit must authenticate ALL claims, assets/movements and history,
 * check exact context, then atomically consume both state and nonce. It must
 * throw on unavailable verification, invalid proof, stale state or stale nonce.
 * No callback returning boolean verdicts is an implementation of this contract.
 */
export interface TrustedAcceptanceBackend extends SourceBinding {
 authenticate(bound:BoundProgram,input:Omit<EvaluationInput,'checks'>):Promise<ExternalChecks>;
 verifyAndCommit(bound:BoundProgram,input:EvaluationInput,simulation:Simulation):Promise<{tag:'Committed';traceHash:string;proofContextHash:string;afterStateHash:string}>;
}
