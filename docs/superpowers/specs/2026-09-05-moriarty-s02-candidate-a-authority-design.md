# Candidate A authority integration contract

Date: 2026-09-05. Classification: local experimental design under the user's
delegated architect authority. Status: adapter/type unit reviewed and adopted; no implementation or
Council acceptance follows from this document. Source boundary:
`d7384d2c8b2c86ac73d3c9f881a1ba66775f10ab` on `s02-model-comparison`.

## Requirement and design choice

XML v1.3 S02 and the adopted common-design decision require actual candidate
semantics at the shared authority boundary. Candidate A's semantic-unit plan
Task 8 requires full predecessor/program, input/time, artifact/plan, result and
effect binding. The existing common execution fixture is not this integration.

Keep the agreement interpreter and shared signing/policy/commit implementation
unchanged. Add a candidate-specific adapter and guarded verification/rejection
wrappers, then instantiate real swap and installment workloads under both signing
profiles. This preserves one common financial/nonce implementation while removing
trusted effect tags as a substitute for executing Candidate A.

Alternatives considered: replacing the common verifier with an A-specific
interpreter would entangle all four candidate boundaries; using existing string
fixtures would not execute Candidate A. Neither is adopted. The wrapper approach
retains the current generic carriers and atomic update function.

Participants remain Alice, Bob and Mallory, acting on shared state; there is no
message protocol or consensus model. Use plain Quint, not Choreo. Signature
authenticity remains a finite symbolic external premise. Effect derivation is an
actual local computation. Neither is a cryptographic proof or universal semantic
correspondence theorem. Preserve frozen Core/swap and scope bytes.

## Concrete carrier sketch

Reuse all existing types; do not add agreement constructors or string lifecycle
states. These aliases are the exact intended integration interfaces:

```quint
type AAuthorityRequest = {before: APredecessor, input: AInput, now: ATime}
type ACall = AgreementCallA(AAuthorityRequest)
  | CancellationCallA({before: APredecessor, now: ATime})
type APlanId = SwapPlanA | InstallmentPlanA | RecoveryPlanA
type AResolvedPlan = {identity: APlanId,
  operations: List[PlannedOperation[APredecessor,AContinuation,ACall]]}
type AAuthorityObservation = CandidateObservation[APredecessor,AContinuation,ACall,AResolvedPlan]
type AAuthorityPolicy = Policy[APredecessor,AContinuation,ACall,AResolvedPlan]
type AAuthorityExecution = ExecutionState[APredecessor,AContinuation,ACall,AResolvedPlan]
type AAuthorityEvidence = EvidenceBundle[APredecessor,AContinuation,ACall,AResolvedPlan]
type AAuthorityAdaptation = AuthorityAdaptedA(AAuthorityObservation)
  | AuthorityComputationDiagnosticA(ATransactionEvaluation)
  | AuthorityExtractionDiagnosticA(AEffectExtraction)
  | AuthorityInvalidRequestA
```

The effect layer's complete finite maps, twelve authority keys and eight attempt
IDs are unchanged. AProgram remains the complete sixteen-node table, including
unused nodes. APredecessor contains all six account and five optional choice
entries and the actual minimum time. AContinuation contains program and node.
ACall contains the full original request, not an unverified opaque call label.
Plan IDs are finite opaque identities inside complete plan payloads of one to
four planned operations. The after-resolution view must equal the payload's
operations, and every operation must independently satisfy candidate semantic
binding and its own predecessor ledger/account coupling. A plan ID alone proves
no fidelity. The adapter carries the supplied plan verbatim; the execution and
signing integration validates it before treating it as a faithful plan view.

## Adapter contract

`adaptAuthorityA(request: AAuthorityRequest, op: Operation, plan: AResolvedPlan,
display: DisplayProjection): AAuthorityAdaptation` executes
`computeTransaction(request.before.program, request.before.state, request.input,
request.now)` for every non-cancellation operation. On an actual computed result,
call `extractCommittedEffects` with that exact result and project the entire
result using `projectResult`. Populate predecessor from request.before, successor
from unchanged program plus actual result.state, input from projectInput, time
from timeValue, call from AgreementCallA(request), and the exact extracted effects.
Only this successful extraction produces effectEvidence=EvidenceValid.

Actual Core rejection is still AuthorityAdaptedA: retain its exact error, original
state, empty payments/warnings/effects and zero reductions. Its outcome is
Rejected(CoreRejected(actual.error)). Evaluation/extraction diagnostics remain
the separate diagnostic variants; they never become Core rejection or success.

Cancellation is the sole lifecycle-only adapter path. Require validOperation(op),
an admitted predecessor, NoAInput, and a non-Time0 request clock. Return the
unchanged predecessor as successor, NoInput, empty effects, NoCoreProjection,
Cancellation outcome and CancellationCallA({before,now}). It does not call the
Core interpreter. The common parent/signature gates decide whether it is allowed.

For non-cancellation, require validOperation(op) and a non-Time0 request clock.
Do not prefilter malformed agreement requests or errors out of the evaluator.
Outcome on an accepted result is the operation's corresponding neutral label:
OpFund/FundingAccepted, OpSettle/Settlement, OpVoluntaryRefund/VoluntaryRefund,
OpDeadlineRefund/DeadlineRefund, slot1/FirstInstallment, slot2/SecondInstallment,
OpRecover/Recovery. Labels do not authorize an operation; signed clauses bind
actual inputs/time/effects and the common operation-specific financial guard.

`authorityObservationMatchesA(op: Operation, obs: AAuthorityObservation): bool`
reconstructs the request from obs.artifactAndCall, reruns the adapter using the
same op, plan and display, and requires AuthorityAdaptedA(obs) by full equality.
Call kind must match cancellation versus agreement. NoCoreProjection is invalid
for every agreement call. Mutating the program, continuation, choices, time,
accepted bit, error, warnings, reductions, payments, effects, or neutral input
without the corresponding actual computation must fail. Plan/display changes
alone need not change semantics, but exact signed-plan and proof/context binding
still apply at the common boundary.

## Execution and rejection contract

`validAuthorityExecutionA(state)` first requires validExecutionState and
validState of context.candidate, then checks every Core account equals its
corresponding escrow balance in context.ledger. Maps must be validated before
lookup. No financial transfer or candidate advancement occurs during adaptation,
proposal, signing or verification.

At signing and execution, `authorityPlanMatchesA(plan)` requires one to four
operations and validates each planned request/result/effect using the adapter,
plus valid predecessor facts and escrow/account coupling. This is a deterministic
view-fidelity check, not evidence that a future predecessor will occur. The
after-resolution policy check additionally requires its view.operations to equal
view.identity.operations. Before-resolution policies have no concrete plan to
validate at signing; validate the later supplied plan at execution. Tests mutate
only the second planned operation to ensure full-plan checking is not reduced to
the current operation. No cyclic type or recursive evaluation is necessary:
PlannedOperation has no plan field, and adaptAuthorityA does not call the plan
validator.

`canVerifyAuthorityA(state,id,evidence)` requires this valid state, a proposed
attempt with full authorityObservationMatchesA, and existing canVerify.
`canCommitAuthorityA(state,id)` requires the same candidate checks on the retained
verified attempt and existing canCommit. Both boundaries recompute semantics.
Use existing applyVerify/applyCommit after these guards: the latter updates the
candidate, ledger, parent accounting, registry and attempt atomically.

Candidate-specific rejection guards are necessary. A forged semantic observation
can pass the generic trusted-tag abstraction while failing the adapter. It must
not become stuck between canVerify=false and common canReject=false. For a
proposed or verified attempt, reject when the corresponding combined A boundary
fails, retaining the original attempt and evidence verbatim in RejectedOperation.
Use UnauthorizedEffect when semantic binding fails; otherwise use existing
rejectionReason with the observed current context. Preserve VerificationBoundary
versus CommitBoundary. Never change a proof's disposition to manufacture failure,
and never label a forged Core error as genuine CoreRejected. Rejection changes
only the attempt record, not signing records or financial/authority context.

Rejection guards are exact to the retained stage and ID and permit only one
terminal rejection per attempt. Do not require the failing A-specific semantic
or escrow-coupling predicate to enable rejection. Validate structural map/record
preconditions before inspecting them; invalid binding is not a reason to strand
an otherwise representable attempt.

## Candidate-specific workloads and policy fidelity

Start installment at the existing literal installmentProgram N4, escrow Alice/A10,
all other accounts/wallets zero and minimum/physical time2. The parent nonce0
policy binds actual ChoiceLike(fill1,Bob,1), ChoiceLike(fill2,Bob,1), and NoInput
cancellation as separate clauses, exact five-unit transfers, exact current
parent facts and appropriate source balances. A cancellation updates only the
common parent entry; do not replace the candidate with a "cancelled" label.

SignAfterResolve binds complete independently constructed expected operation
views for initial fill/cancel and the residual fill/cancel alternatives, including
the actual Core projections. Do not create an expected plan by simply copying
an unvalidated solver proposal. SignBeforeResolve binds the same unsigned bounds
and mechanism before a concrete proposal. Each uses the unchanged common signing
prepare/sign transitions, per-operation proposals, evidence verification and
atomic commit. Permit both first-fill/cancel race orders and retain the loser.

After cancellation, recovery nonce1 is prepared and signed against the actual
remaining state. Recovery uses ChoiceLike(recover,Alice,1) at time2, refunds exactly
ten or five as computed by Core, and leaves the cancelled parent accounting and
nonce0 record unchanged. Separate deadline scenarios use NoAInput at time100 and
an explicitly matching signed recovery clause; supplied recovery input at the
deadline is an actual Core rejection, not a refund. No early financial recovery
is authorized merely because the agreement would accept its input.

Swap starts with actual wallet funding and empty agreement N6. Register and consume
each depositor's nonce0 separately; register disposition nonce1 policies under
both profiles. Actual deposits, settle=1, settle=0, funded/Alice-only timeout and
deadline-input rejection traverse the same A verification/commit boundary.
Time advancement is a separate finite guarded environment action, with stale
prepared/verified records retained and rejected or explicitly refreshed.

Known empty-timeout boundary: Core accepts an empty N6 timeout with no transfers.
The current common verifier requires nonempty requiredKeys and a positive-length
effect list for nonparent operations, so that observation is envelope-rejected.
Retain both facts in a deterministic control. Do not invent a transfer, signer,
or accepted commitment. The S02 coverage/selection decision must explicitly
assess this limitation; this document does not silently change the common contract.

## Acceptance tests and staged implementation

1. Adapter/type unit: actual deposit and first/second fill projections, recovery
   ten/five, exact timeout rollback, cancellation unchanged/no Core result;
   malformed finite state yields a diagnostic; call/neutral-input/result/effect
   substitutions fail full binding. Pure tests, then a small action witness.
2. Boundary unit: an honest prepared attempt verifies and commits; forged but
   generic-tag-valid observation fails and remains rejectable; mutated verified
   payload fails commit; stale context uses shared classification; rejection
   preserves exact original records and context; escrow/account mismatch fails.
3. Both-profile installment lifecycle: real choices, parent fill/cancel race,
   stale loser, separate second fill, cancellation then newly signed nonce1
   recovery ten/five, positive action witnesses, conservation and residual/nonce
   invariants. Terminal means actual Close/zero escrow plus resolved attempts,
   not cancellation alone.
4. Both-profile swap and deadline lifecycle: actual funding/disposition signatures,
   strict request clocks, explicit empty-timeout rejection, and all required
   positive financial outcomes. No Core-unavailable flags authorize A transitions.
5. Export actual integrated records; extend correspondence with an explicitly
   pinned producer schema, preserving raw requests/results and checking complete
   inventories. Retain adversarial controls and independent review. Final bounded
   model checking, A–D comparison and Council acceptance remain required S02 work.

Use behavioral RED/GREEN tests before implementation and preserve exact source
closures/commands/receipts. No passing typecheck or local semantic predicate is
an independent checker result. This contract does not declare any stage complete.

## Local adapter/type review disposition

The nonauthor native reviewer `quint_policy_review` inspected design SHA-256
`790a299479f0f66e8ad76a4e8e1d8540f54fcb9ccf35ac01dd3848795f0b0fed`
and reported no blocking carrier/adapter finding. Its substantive cautions are
retained above: reachable rejection despite failed A validation; full-plan
validation at signing and execution; per-operation fidelity does not prove
future predecessor reachability; integrated exports must retain raw results;
empty-timeout rejection remains explicit. The reviewer authored projection
dependencies, not this contract. This is native design review, not Council.

Root adopts the concrete type/adapter unit under existing delegated authority.
Only that unit is ready for implementation; later boundary/lifecycle units need
their own concrete test plan and review, without repeating the common-design vote.
