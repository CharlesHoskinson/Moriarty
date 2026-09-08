Continue the existing SP01.6 correction. Your prior 900-second attempt timed out AFTER saving traces.json; the parent preserved those partial bytes. Do NOT repeat that large rewrite. You have 600 seconds including termination. Start by making targeted edits to design.md and semantic-challenges.json, then reports. Do not spend the pass rewriting traces or rereading all sources. Existing traces already contains D1-D5 corrections; align prose to it, with small trace fixes only if a concrete inconsistency is found. Read only the relevant sections. No shell commands, code/spec/test/register/wiki changes, network, proofs, git writes, or self-approval. Owned files remain the three paths below and root FOREMAN_REPORT.md/.json.

Primary remaining work: design.md still has incomplete four read-footprint paragraphs, overly broad shared authority binding prose, under-specified negative controls, missing shared Complete/State observation map references, and stale atomic acceptance prose. semantic-challenges.json and reports also need current acceptance and network metadata. Preserve original authored base as history. Keep gate pending-review and fullRP01/native incomplete. Current traces contains sharedEvaluationDependencies, sharedObservationMap, negativeFixtureConvention, updated per-row maps/counts, and captureMetadata/networkMilestone. Reuse by explicit reference rather than copying huge JSON into prose. Correct the exact differences specified below. Prior static verifier failed on DueCreated settlement:null; current partial traces removed it. No commands or tests run by worker; parent checks and final GPT6 review pending. Finish with a brief final JSON-compatible result so model identity is captured.

You are Grok4.6 high, implementation author of the existing Moriarty loan/swap SP01.6 design. Apply the independent GPT6 corrections now with Read/Edit/Write. No new permission/design cycle; correcting fidelity to existing schema only. Worktree /home/charl/Moriarty/.worktrees/sp01-loan-swap-grok. Own only the same3 files (semantic-challenges.json, SP01/loan-swap-subset-01/design.md and traces.json under evidence/moriarty-completion-program-2026-09-07), plus FOREMAN_REPORT.md/.json. No code/spec/test/register/wiki edits, shell, Git writes, agents, installs, web, wallet/proof/public activity. Parent froze original candidate and failed verifier. Finish within890sec. Preserve correct financial values and other content. Make targeted edits; do not rewrite the entire80KiB trace file unless necessary. Parent runs unchanged verify.py afterward; then independent GPT6 checks correction.

CONSOLIDATED GPT6 BLOCKERS (all must close):
{
  "reviewer": "gpt-6-astra",
  "effort": "high",
  "agent": "/root/loan_swap_design_review",
  "candidateSha256": "6714fec0e1171236a784dabcc5784a571b574f0dbb8cf0dd858b3fa0e60c40cb",
  "verdict": "BLOCKED",
  "integrity": "Three owned and ten input hashes matched before/after",
  "scope": "Static existing loan/swap design only; no tests/compilers/network/actions",
  "blockingFindings": [
    {
      "id": "D1",
      "finding": "loan-accrue orderedEffects[0/1].settlement:null illegal for closed DueCreatedEffect; field must be absent"
    },
    {
      "id": "D2",
      "finding": "Negative fixtures omit reseal/rebind/authentication conventions; actual rejection may occur before declared predicate. loan-replayed-due must mutate after-accrue status, not actual exhausted after-settle. Reserve control revision7/remaining1; wrong role needs bound wrong-role principal; ExactPlan mutations need simulated/new signature; net-goal fixture sufficient gross cap; false checks vs omitted record fields; client booleans belong to third evaluate argument."
    },
    {
      "id": "D3",
      "finding": "Authority prose conflates signed statement fields with derived action/observation context. OutcomeStatement does not contain ActionCall or min_out. Neither statement directly signs current ObservationSet."
    },
    {
      "id": "D4",
      "finding": "Read footprints mix pre-state/staged/global inputs and omit source and envelope dependencies. Need explicit convention and complete bounded shared dependency reference."
    },
    {
      "id": "D5",
      "finding": "Complete/State observation map misses authorityConsumption mode/principal/nonce, obligationDelta, resource counts, structural envelope fields. Deferred counts inconsistent across rows; every field needs mapping/derivation/owner."
    }
  ],
  "confirmedFinancialValues": [
    "Loan interest33972602 remainder54/73 total533972602 cash19466027398 residual4500000000",
    "Swap output19743 input10000 reserves1010000/1980257 provider close transfers, order/lifecycle correct",
    "Min-receive inequality, absent Fee effect, retained settled tombstones and closed episode vs outstanding debt correct",
    "Fixed-source instruction counts and accrue FloorDiv node correct",
    "Nominal-debt and pool/provider custody limitations correctly retained"
  ],
  "promotionRequirements": [
    "Preserve authoredf173 history and bind current atomicc30aaf5/b296ce9 separately",
    "Explicitly retain Preview loan AND swap transactions/finality/full comparisons and rejection controls after fixtures/custody/contracts/campaign review",
    "No fullRP01/native/financial corpus/mandatory-judgment closure"
  ],
  "resourceNote": "First600 seconds of900-second native allowance; no process-cgroup enforcement"
}

PRECISE IMPLEMENTATION GUIDANCE:
D1: Remove ONLY settlement:null from the two loan-accrue orderedEffects DueCreated wire records. Other conceptual not-applicable metadata may use null; distinguish design metadata from exact wire fragments.
D2: Add top-level negativeFixtureConvention and per-negative explicit construction references/exceptions, NOT an extra entry in negativeTraces (that map contains trace IDs only). Convention: start from fresh independent valid fixture for each control; preserve prior admission/binding checks unless deliberately targeted. Reseal mutated StateBody with stateHash=hash(STATE,body), then rebind authority.statement.beforeStateHash and predecessors to that mutated hash when targeting obligation/lifetime predicates. Signature is explicitly simulation-only/assumed valid in simulator controls; never imply a valid real signature survives changing signed bytes or that malicious state is historically reachable. unchangedInput means supplied mutated fixture immediately before vs after evaluation.
- loan-replayed-due: after-accrue state with one status changed to Settled. Real after-settle state remaining0 fails LIFETIME_EXHAUSTED first and is a separate non-equivalent case.
- swap-no-reserve: revision7 remaining1 consistent with lifetime8; reseal/rebind synthetic state.
- wrong-role source guard: action.actor=wrong role, authority principal that wrong actor, trusted authenticatedPrincipal same, matching genesis principal binding; source guard then fails. Distinguish actor/principal mismatch PRINCIPAL_BINDING. For ExactPlan, signed statement.action must match mutated action.
- ExactPlan extra/reorder controls explicitly use new simulated statements with signature-valid assumption; never assert unchanged real signature.
- net goal includes every debit fixture has sufficient gross cap (maxUInt128 as fresh cited test), not preceding101 cap.
- stale nonce/state: present false Boolean gives named rejection; omitted required checks field is INPUT_SCHEMA.
- client booleans: malformed third evaluate argument used as fake backend, as tests/semantics.test.mjs shows; not extra fields in EvaluationInput.
Align all relevant negative records and design.md Negatives section.
D3: Replace broad both-bind-action-and-observations prose with exact separation from runtime-types.ts below. Common signed program/profile/bounds, domain/genesis/instance/state/predecessors/principal/nonce/validity/requiredclaims. Genesis indirectly commits observation/provider and principal bindings. ExactPlan signs ActionCall (includingargs), ordered exactwrites/effects. Outcome signs allowedactions and protected outcomes/recipients/calls, not ActionCall/min_out. ObservationSet is authenticated separately; actual actionHash/observationsHash and trace/ProofContext bind evaluated invocation afterward. Neither authority directly signs current ObservationSet. Preserve source-authorized constants and unresolved debt/custody limitations.
D4: Define consistent bounded footprints. Recommended row readFootprint object with sourcePreStateReads, sourceStagedReads, sourceConstantsArgumentsObservations, sharedDependencyRef; top-level sharedEvaluationDependencies covers entire source-bound program/manifest, canonical whole input/state hash, all typed/state/authority/genesis/observation fields, obligations and status derivation (finite registered bounds). Include every actual source state read, including staged principal_due/interest_due accrual emits; role and expected-value constants; swap trader_b; close source notreadingtrader balances but fullstatehash stillcommits them. Use source file as ground truth; distinguish staged reads and prestate dependencies rather than dropping them. Align four design footprint paragraphs. Do not imply only listed source reads account for entire evaluator work.
D5: Add sharedObservationMap with field-by-field explicit Complete wrapper, CompleteBody, after StateEnvelope/StateBody and ProofContext fields. Cross-reference row-specific financial values/effects/writes/obligations and structural schema/profile constants; all hash fields may be symbolic but exact domain/preimage references required. Include authorityConsumption.mode/principal/nonce/statementDigest, obligationDelta.created/settled, all8 ResourceCounts members, after schema/stateHash/body structuralfields. Every row should reference shared map and retain specialized economic observations; don't silently drop existing maps. For all4 resourceCountsSpecifiedFromSource records include all8 fields. Known instruction/effect counts retain values; uncomputed counts explicitly state pending derivation, method (source traversal or CountBody canonical encoding excludes entire resourceCounts), owner and closure task. Do not invent counts or hash values. Main semantics state resource/result checks still required before any actual Complete; these are design expectations only.

PROMOTION HISTORY / NETWORK: Preserve authored basef173 in capture metadata. Add current independent localatomic acceptance code c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51 / evidence b296ce95afae722177b5bba482d1d195fa334d1e; current SP01.8 no longer pending (historical authored state may stay explicitly dated). User explicitly required Midnight network milestone tests. Reference raw/assignments/moriarty-midnight-milestone-testing-2026-09-07.md (exists in main after your base) and state SP05 needs meaningful loan AND swap Preview txs, finalizedreceipts, fullcomparisons and rejectioncontrols AFTER separately reviewed fixtures, custody, contracts and campaign. Source/spec/local checks do not discharge them. Mandatory-proof SP09 and laternetwork gates unchanged. FullRP01/native subset stillincomplete; this record remains pending-review. Keep candidateHash/outputsha null for parent freeze, reviews empty; do not selfapprove.

REPORT: list concrete corrections, prior static schema check failed, no commands run by you, parent verification/GPT6 final review pending.

EXACT CURRENT runtime-types.ts (read-only correction source):
```ts
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

```
