# Candidate A authority boundary implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Complete this bounded unit with behavioral tests and independent review before lifecycle implementation.

**Goal:** enforce the adopted Candidate A semantics and complete plan fidelity at
the unchanged common signing, verification and atomic commitment boundaries.

**Architecture:** add A-specific pure guards and rejection constructors around
the existing common implementation. Do not change agreement semantics, common
policy/signature/ledger rules, or the completed adapter. Actual signature validity
remains the common finite symbolic premise, not a cryptographic implementation.

**Contract:** `docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-a-authority-design.md`.
This plan implements its boundary unit, not both complete lifecycle workloads.
Quint 0.32.0, Rust execution backend, existing Python environment.

## Task 1: full-plan fidelity and signing guards

Create `specs/quint/s02/candidate_a_authority_boundary.qnt`,
`candidate_a_authority_boundary_fixtures.qnt`, and
`candidate_a_authority_boundary_test.qnt`. These three names share the same directory.
Add a harness in Task 3. Import actual A/common modules, never the string-based
generic test harnesses. Preserve protected inputs and unrelated files.

- [ ] Read the adopted design, adapter and the full policies, authorization,
  execution and effects modules. Declare aliases for A-specialized
  AuthorityContext, SigningState, PreparedAttempt and VerifiedAttempt as needed.
- [ ] Implement `authorityPredecessorLedgerMatchesA(before, ledger)` with explicit
  validation of `validState(before.program,before.state)` and `validLedger` before
  reading any map entry. Then require equality for all six Core accounts and
  their corresponding `(Escrow(account),account.asset)` ledger entries. Missing
  keys must return false, never panic or become implicit zero-account evidence.
- [ ] Implement `authorityPlannedOperationMatchesA(planned, plan)` by reconstructing
  the request from the complete ACall and invoking adaptAuthorityA with the
  planned operation, supplied plan and PublicDisplay. On AuthorityAdaptedA,
  compare every PlannedOperation field to the corresponding observation field
  using `plannedMatches(planned, planned.operation, observation,
  planned.predecessorFacts)`. Also require validPolicyFacts, predecessor/ledger
  coupling, and transactionTime equal to facts.environment.physicalTime.
  Diagnostic results return false. Call-kind equality is enforced by this full
  comparison. PlannedOperation has no outcome/display/evidence field: do not
  fabricate one from a label. The adapter supplies the actual complete result.
- [ ] Implement `authorityPlanMatchesA(plan)` for one through four operations,
  checking **every** operation with the preceding helper. This is deterministic
  view fidelity, not permission, solvency, or reachability of future facts.
  Financial authorization remains in the common gates. Empty adapter metadata
  must fail here. Core-rejected operations may be faithful views; they remain
  unexecutable because the common gate rejects their actual result.
- [ ] Implement `validAuthorityPolicyA(policy)` as common validPolicy plus:
  BeforeResolution needs no concrete plan; AfterResolution requires
  `view.operations == view.identity.operations` and authorityPlanMatchesA of the
  full identity payload. Do not check only the current operation or the plan ID.
- [ ] Implement `validAuthoritySigningA(state)` as validSigningState and candidate
  escrow/account coupling. Implement `canPrepareSigningAuthorityA` and
  `canSignAuthorityA` with this state validity, validAuthorityPolicyA and the
  corresponding common guards. Use existing applyPrepareSigning and applySign
  only after these guards. Do not add replacement signature/parent transitions.

### Literal fixtures and behavioral tests

Start with actual canonicalSwap at N6, emptyAState minimumTime Time1,
wallet Alice/TokenA10, all other ledger entries zero, physicalTime2, anchor0,
implementationVersion0, enforcementMechanism0, all authority cells unused,
parent cells vacant, signing checks absent, and attempts absent. This is a
funding-boundary fixture, not the complete swap lifecycle.

Use actual deposit10 into aliceA by Alice at Time2, OpFund and FundingOneAttempt.
The independent expected successor is the same full program with state N5,
aliceA10, minimumTime2 and all choices unchanged. The one expected effect is
Wallet(Alice) -> Escrow(aliceA), TokenA10. The actual Core projection has accepted
true, NoCoreError, no payments/warnings and zero reductions, full projected state
and continuation. Declare these expected fields literally rather than creating
the expected plan by copying an unvalidated adapter result.

Construct a one-operation SwapPlanA from those literal fields and exact initial
PolicyFacts. Alice's key is SwapDomain/Alice/nonce0. Body: FundCapability,
debitLocations={Wallet(Alice)}, exact-order deposit effect required and allowed,
InputIs(the actual DepositLike), BeforeTime100, SourceBalanceIs(wallet10),
validFrom1/validUntil99, version0/mechanism0. Instantiate both profiles over this
body: the exact AfterResolution view, and BeforeResolution AnyArtifactUnderMechanism.
Produce actual observations by matching the adapter's result, not an unchecked
default-on-diagnostic extraction helper. Test initialization must assert the
actual successful adaptation and equality to the independent expected fields.

- [ ] Before implementing each new guard, retain executable behavioral RED
  using a compiling permissive stub/common delegation and the relevant negative
  assertion. Capture full source/import closure and command/terminal output.
- [ ] Plan cases: honest one-operation plan and both policies pass; zero/five
  operations fail; a two-operation plan with two faithful identical operations
  passes fidelity (no uniqueness/reachability claim); mutate only the second
  operation's successor, input, full call/program including unused node, time,
  projection or effects and require false. Mutate only its ledger coupling or
  facts clock and require false. Remove account/ledger/facts keys and require
  false without evaluator errors. Use missing parent-map keys as a facts control.
- [ ] After-resolution mismatch control: keep a faithful identity payload but
  put a different operation list in the outer view; generic validPolicy may
  still hold but validAuthorityPolicyA and A signing preparation must fail.
  Before-resolution policy preparation passes without a concrete plan.
- [ ] Actual preparation then signing under both profiles must use and assert
  the A guards. Signing cannot transfer money/advance candidate/consume nonce.
  Invalid plan and coupled-state mutation must fail again at signing, even if
  a prepared record was constructed earlier under the common abstraction.

## Task 2: verification, commit and reachable rejection

- [ ] Implement `validAuthorityExecutionA(state)` as validExecutionState plus
  candidate escrow/account coupling. Do not require freshness of every retained
  attempt: stale records are deliberate model states.
- [ ] Implement `authorityAttemptMatchesA(attempt)` as full
  authorityObservationMatchesA plus authorityPlanMatchesA of its resolved plan.
  Implement evidence-policy fidelity by checking validAuthorityPolicyA for each
  supplied signature proof's policy; the common verifier still establishes exact
  signature inventory, full attempt equality and actual registration.
- [ ] `canVerifyAuthorityA(state,id,evidence)` requires validAuthorityExecutionA,
  a retained ProposedAttempt with matching id, authorityAttemptMatchesA,
  evidence-policy fidelity, and common canVerify. `canCommitAuthorityA` does the
  same on a retained VerifiedOperation and its retained evidence, then canCommit.
  Use explicit conditional guards before map lookups. No adapter or trusted tag
  alone grants permission. Use unchanged applyVerify/applyCommit after success.
- [ ] Define a structural rejection-domain guard: all attempt, signing,
  registry and parent maps have their expected complete key sets. Do not require
  semantic A validity, account/escrow coupling, fresh context or common policy
  validity to permit rejection. These may be the failed boundary being recorded.
- [ ] `canRejectProposedAuthorityA` requires that structural domain, exactly a
  ProposedAttempt at the queried id with matching retained id, and failure of
  canVerifyAuthorityA for the supplied evidence. Verified counterpart requires
  exactly VerifiedOperation and failure of canCommitAuthorityA. Terminal records
  cannot be rejected again. An absent/mismatched stage/id must return false.
- [ ] The A rejection reason is UnauthorizedEffect for failed candidate/state
  coupling, observation/plan fidelity or evidence-policy fidelity. Otherwise use
  existing rejectionReason on the original attempt/evidence/current context.
  This ordering prevents a forged Core error from acquiring genuine CoreRejected
  classification. Common stale/missing-evidence/consumption classifications are
  preserved when A-specific validation passes.
- [ ] `applyRejectProposedAuthorityA` and `applyRejectVerifiedAuthorityA` construct
  RejectedOperation with the exact original attempt/evidence, current observed
  context, appropriate VerificationBoundary/CommitBoundary and A reason. Change
  only the queried attempt. Safely return state if their rejection guard fails.
  Never edit proof dispositions, signing records or financial/authority context.

### Adversarial and positive assertions

- [ ] Full honest funding pipeline for both profiles: prepare/sign/propose/
  verify/commit with every guard asserted; final candidate N5/aliceA10, ledger
  walletAlice0/escrowAlice10, registry exact consumed signature revision1,
  ExecutedOperation exact retained evidence and unchanged unrelated keys/parents.
  Conservation and validAuthorityExecutionA hold; second commit is forbidden.
- [ ] Construct a before-resolution attempted observation with the honest
  deposit effect but an unchanged proposed successor; rebind **all** proof
  attempt references exactly to it. Show common canVerify=true and A=false.
  Show A rejection is enabled while common canRejectProposed=false. Rejection
  preserves original evidence bytes/records and context and classifies
  UnauthorizedEffect at VerificationBoundary. No terminal second rejection.
- [ ] Simulate tampering of a verified payload and rebind all its proof attempt
  references. Show common canCommit=true but A=false and A commit rejection
  retains that exact tampered payload at CommitBoundary with no money movement.
- [ ] Mutate an observation's raw error/accepted bit into a forged known
  Core rejection; show A rejection says UnauthorizedEffect, not CoreRejected.
  A genuine adapter-produced deadline-input rollback is a separate test which
  remains classifiable as CoreRejected when A plan/facts/signatures are faithful;
  construct a dedicated actual deadline request and matching permissive unsigned
  body/view as necessary. Never reuse the pre-deadline clause as positive evidence.
- [ ] Missing effect/signature evidence with otherwise faithful candidate data
  retains EvidenceMissing; change only context anchor after verification and
  retain StaleBindings. Both are rejectable without changing context.
- [ ] Change one escrow balance after proposal while keeping complete valid
  ledger maps: A state validity/verify fail but rejection remains possible. Change
  candidate accounts similarly. Missing structural attempt/signing/registry/
  parent maps refuse guards safely; malformed candidate/account maps remain
  rejectable within the complete structural envelope.
- [ ] Empty/mutated resolved plan must fail execution even under a valid signed
  before-resolution policy. A tampered second planned operation and after-view
  mismatch must fail even if current-operation fields/effect tags are authentic.

## Task 3: actual action witness, verification and handoff

Create `candidate_a_authority_boundary_harness.qnt`. Reuse only the non-test
fixtures module. One execution-state variable plus a finite chosen profile;
init starts unsigned actual funding state. Guarded actions perform one prepare,
sign, propose, verify and commit in order using the A guards and unchanged common
updates. No stutter. The safety invariant requires A state validity, conservation,
and no ledger/candidate transfer until ExecutedOperation. The witness requires
the exact actual committed candidate, ledger, consumed key and resolved attempt.
Run each profile explicitly through deterministic action tests; a mixed-profile
sample alone does not prove both paths occurred.

- [ ] Typecheck all four new modules; run all boundary tests with Rust seed42.
- [ ] Run the harness with Rust seed42, max-samples100, max-steps6, its safety
  invariant and actual-commit witness; retain actual observed counts and terminal
  outputs. No statistical/completeness claim follows from repeated finite paths.
- [ ] Run existing adapter regression tests and the full Python suite once after
  source completion; retain complete tested source closures, tool identity and
  actual terminal receipts, including genuine RED before implementation.
- [ ] Self-review; commit only the four owned files; write a unique task report.
  Request independent source/spec/evidence review, fix demonstrated findings and
  run affected checks. Root archives immutable receipts and updates wiki/checkpoint.

## Exclusions and plan review

This adds one real funding commitment boundary, not the complete swap/installment
workload, adversarial exhaustive verification, A selection or S02 passage.
Both-profile parent cancellation/recovery races, final correspondence exports,
semantics-preserving Apalache factoring, candidates B–D and exact-provider Council
remain open. Empty timeout semantics and generic no-effect rejection remain as
recorded in the adopted design; do not weaken the common contract to hide them.

The parent contract was adopted under delegated authority. Native nonauthor
`s02_recovery_review` inspected this plan at SHA-256
`dbc4ecccf04368c1f21f106ad832d1cd04d48ea5078b3958a256555579e569c3`
and found no blocking contract/fixture defect. Its retained cautions are the
explicit structural rejection domain and A-specific failure classification
before delegating to the common classifier. Root's source review agrees and
adopts this bounded plan. This is not Council or implementation acceptance.
