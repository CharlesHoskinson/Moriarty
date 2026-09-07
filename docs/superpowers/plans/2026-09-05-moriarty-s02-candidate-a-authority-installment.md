# Candidate A Installment Authority Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Execute A2-I01–I11 using actual installment semantics and the frozen A authority boundary under both signing profiles.

**Architecture:** Independent literal fixtures provide full expected plans/results. Pure guarded helpers and a finite shared-state harness drive actual adapter observations through prepare/sign/propose/verify/commit or retained rejection. Original nonce0 signs four fill/cancel alternatives; FreshCancelAttempt reuses it and recovery signs nonce1 against cancelled history.

**Tech Stack:** Quint 0.32.0/Rust; frozen source at `d14cfea98a1c9213e5ef5f12c1a088f4e966083d`.

## Global Constraints

- Specified-only corrected draft; independent A1 adoption follows A0. Embedded code has not been run by the planner. Compiling RED/GREEN gates are mandatory during implementation.
- Authority: approved `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`, semantic contract, A1-R01–R05 and A2-I01–I11.
- Alice, Bob and Mallory coordinate through shared state, not messages.
- Signature validity remains an explicit finite symbolic external premise, not cryptography.
- Only commit atomically changes money, candidate, parent consumption, registry and the retained attempt.
- Cancellation is NoInput, unchanged A predecessor, empty effects and NoCoreProjection.
- Plans contain one to four operations; after-resolution outer operations equal identity.operations.
- Do not equate remainingAllowance with escrow after recovery: escrow is zero while cancelled parent history remains intact.
- No common, adapter/boundary, Python Core/swap, DB, B–D, generic installment or evidence edits in this planning task. No implementation/commit now.
- A2 implementer owns exactly the four new files below. Root owns OpenSpec/EARS acceptance, evidence capture, integration and later Council. Every task requires fresh nonauthor source/spec review plus exact-source runtime receipts.
- Sampled paths are experiment observations, never proofs. Separate XML model checking, exports and correspondence remain later obligations.

## File ownership and dependencies

Create only:

- `specs/quint/s02/candidate_a_authority_installment_fixtures.qnt`: independent literal program, states, facts, plans and policies.
- `specs/quint/s02/candidate_a_authority_installment.qnt`: pure guard/update helpers, route definitions, invariants.
- `specs/quint/s02/candidate_a_authority_installment_harness.qnt`: state, guarded actions, step and action witnesses.
- `specs/quint/s02/candidate_a_authority_installment_test.qnt`: deterministic tests and negative controls.

Read-only imports are effects, consumption, observations, policies, authorization, execution, candidate_a_types/programs/core/projection, candidate_a_authority_adapter/boundary. Generic installment is reference only. All module blocks below require these explicit imports; never assume imports are transitive. Use the following exact preamble in each module, then its own definitions and final closing brace. In lifecycle add fixture import; in harness add fixture and lifecycle imports; in tests add fixture and lifecycle imports.

```quint
import effects.* from "./effects"
import consumption.* from "./consumption"
import observations.* from "./observations"
import policies.* from "./policies"
import authorization.* from "./authorization"
import execution.* from "./execution"
import candidate_a_types.* from "./candidate_a_types"
import candidate_a_programs.* from "./candidate_a_programs"
import candidate_a_core.* from "./candidate_a_core"
import candidate_a_projection.* from "./candidate_a_projection"
import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
```

Repository observations: validEntry requires revision=used slots+cancellation; signing requires AuthorityUnused; authorityAllows permits consumed parent keys for fills/cancel. Recovery checks cancelled allowance=escrow before commitment but preserves parent history afterward. FreshCancelAttempt is admitted. RecoveryAttempt is single-use. After-resolution policy validity checks every planned body/effect pair, including faithful rejected results.

## Requirement inventory

Every row is deterministic under SignAfterResolve and SignBeforeResolve.

| Requirement | Route/check | Exact result |
|---|---|---|
| I01 | registrationTest | N4/minTime2, escrow10; actual prepare/sign registers 0/10/rev0 without money/candidate change |
| I02 | fill-first prefix7 | N2, fill1=1, Bob5/escrow5, slots{1}, paid5/allowance5/rev1 |
| I03 | TwoFillsI | N0, Bob10/escrow0, slots{1,2}, paid10/allowance0/rev2; replay refused |
| I04 | initial cancellation prefix8 | both initial attempts verified from same context; cancel wins, exact fill loser rejected stale; escrow10, paid0/allowance10/rev1 |
| I05 | residual prefix8 | fill wins, exact cancel loser rejected stale; escrow5; fill2 separately enabled |
| I06 | residual prefix11 | FreshCancelAttempt, existing nonce0 signature, N2/escrow5 unchanged; paid5/allowance5/rev2 |
| I07 | unfilled Choice2I | new nonce1 prepare/sign, recover1 Alice refund10; N0/escrow0; nonce0/parent unchanged |
| I08 | residual Choice2I | refund5; Alice5/Bob5; cancelled paid5/allowance5/rev2 unchanged |
| I09 | initial/residual Timeout100I and Timeout101I | explicit environment advance; separately authorized NoInput refunds10/5, reductions2 |
| I10 | initial/residual Refuse100I and Refuse101I | supplied recovery contract_closed; original minTime2/state rollback, zero payments/warnings/effects/reductions; retained CoreRejected |
| I11 | negativeAuthorityTest/duplicateTest/freshDuplicateCancellationTest | no-cancel, unsigned/old nonce and duplicate fill/cancel/recovery rejected at exact boundary |

## Task 1: literal fixture and compiling RED

**Files:** fixture and test. **Produces:** all definitions below. Module name is `candidate_a_authority_installment_fixtures`.

- [ ] Add the full fixture below with exactly one RED defect: parentPlanI.operations initially contains only fill1PlannedI, fill2PlannedI and cancel0PlannedI. Add fourOperationParentTest. Typecheck first; capture actual assertion failure and complete source/import closure before editing. Type/import errors are not behavioral RED.
- [ ] Restore the fourth operation as shown; run the focused test GREEN and literalFidelityTest. Expected values never call adaptAuthorityA or projectResult.

```quint
type RecoveryModeI = Choice2I | Timeout100I | Timeout101I | Refuse100I | Refuse101I
type ScenarioI = TwoFillsI | RecoverI({residual: bool, mode: RecoveryModeI})
pure val MODES_I = Set(Choice2I, Timeout100I, Timeout101I, Refuse100I, Refuse101I)
pure val SCENARIOS_I = Set(TwoFillsI).union(Set(false, true).map(residual =>
  MODES_I.map(mode => RecoverI({residual: residual, mode: mode}))).flatten())
pure val PROFILES_I = Set(SignAfterResolve, SignBeforeResolve)
pure val parentKeyI: AuthorityKey = {domain: InstallmentDomain, principal: Alice, nonce: 0}
pure val recoveryKeyI: AuthorityKey = {domain: InstallmentDomain, principal: Alice, nonce: 1}
pure val fill1I: Operation = OpFillSlot({parent: parentKeyI, slot: 1})
pure val fill2I: Operation = OpFillSlot({parent: parentKeyI, slot: 2})
pure val cancelI: Operation = OpCancelParent(parentKeyI)
pure val recoverI: Operation = OpRecover({parent: parentKeyI, recovery: recoveryKeyI})
pure val programI: AProgram = {root: N4, nodes: Map(
  N0 -> CloseA,
  N1 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}),
  N2 -> WhenA({cases: List(
    {caseAction: ChoiceA({id: SecondFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N1},
    {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
    timeout: Time100, timeoutNode: N0}),
  N3 -> PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N2}),
  N4 -> WhenA({cases: List(
    {caseAction: ChoiceA({id: FirstFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N3},
    {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
    timeout: Time100, timeoutNode: N0}),
  N5 -> CloseA, N6 -> CloseA, N7 -> CloseA, N8 -> CloseA, N9 -> CloseA,
  N10 -> CloseA, N11 -> CloseA, N12 -> CloseA, N13 -> CloseA, N14 -> CloseA, N15 -> CloseA)}
pure def accountsI(q: int): CoreAccount -> int = Map(
  {owner: Alice, asset: TokenA} -> q, {owner: Alice, asset: TokenB} -> 0,
  {owner: Bob, asset: TokenA} -> 0, {owner: Bob, asset: TokenB} -> 0,
  {owner: Mallory, asset: TokenA} -> 0, {owner: Mallory, asset: TokenB} -> 0)
pure def choicesI(first: OptionalInt, second: OptionalInt, recovery: OptionalInt): AChoiceId -> OptionalInt =
  Map(SettleId -> NoInt, FirstFillId -> first, SecondFillId -> second, RecoveryId -> recovery, OtherId -> NoInt)
pure def neutralChoicesI(first: OptionalInt, second: OptionalInt, recovery: OptionalInt): str -> OptionalInt =
  Map("settle" -> NoInt, "fill1" -> first, "fill2" -> second, "recover" -> recovery, "other" -> NoInt)
pure val initialBeforeI: APredecessor = {program: programI, state: {
  continuation: N4, accounts: accountsI(10), choices: choicesI(NoInt, NoInt, NoInt), minimumTime: Time2}}
pure val residualBeforeI: APredecessor = {program: programI, state: {
  continuation: N2, accounts: accountsI(5), choices: choicesI(IntValue(1), NoInt, NoInt), minimumTime: Time2}}
pure val filledBeforeI: APredecessor = {program: programI, state: {
  continuation: N0, accounts: accountsI(0), choices: choicesI(IntValue(1), IntValue(1), NoInt), minimumTime: Time2}}
pure def ledgerI(escrow: int, alice: int, bob: int): Ledger = LEDGER_KEYS.mapBy(_ => 0)
  .put((Escrow(aliceA), TokenA), escrow).put((Wallet(Alice), TokenA), alice).put((Wallet(Bob), TokenA), bob)
pure def environmentI(t: int): Environment = {physicalTime: t, anchor: 0, implementationVersion: 0, enforcementMechanism: 0}
pure def parentFactsI(residual: bool, cancelled: bool): ParentFacts = ParentPresent({
  cancelled: cancelled, usedSlots: if (residual) Set(1) else Set(), paid: if (residual) 5 else 0,
  remainingAllowance: if (residual) 5 else 10, revision: (if (residual) 1 else 0) + (if (cancelled) 1 else 0)})
pure def factsI(residual: bool, cancelled: bool, t: int): PolicyFacts = {
  ledger: if (residual) ledgerI(5, 0, 5) else ledgerI(10, 0, 0), environment: environmentI(t),
  parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent).put(parentKeyI, parentFactsI(residual, cancelled))}
pure def inputI(id: AChoiceId, chooser: Principal): AInput = PresentAInput(ChoiceInputA({id: id, chooser: chooser, chosen: 1}))
pure def transferI(recipient: Principal, q: int): Transfer =
  {source: Escrow(aliceA), destination: Wallet(recipient), asset: TokenA, quantity: q}
pure def projectedI(node: NodeId, q: int, first: OptionalInt, second: OptionalInt, recovery: OptionalInt,
  t: int, accepted: bool, error: CoreError, payments: List[CorePayment], reductions: int): CoreProjection[AContinuation] =
  CoreProjected({accepted: accepted, error: error, warnings: List(), payments: payments, reductions: reductions,
    state: {accounts: accountsI(q), choices: neutralChoicesI(first, second, recovery),
      continuation: {program: programI, node: node}, minimumTime: t}})
pure val fill1PlannedI: AAuthorityPlanned = {predecessor: initialBeforeI, proposedSuccessor: residualBeforeI,
  artifactAndCall: AgreementCallA({before: initialBeforeI, input: inputI(FirstFillId, Bob), now: Time2}),
  input: ChoiceLike({id: "fill1", chooser: Bob, chosen: 1}), operation: fill1I, transactionTime: 2,
  effects: List(transferI(Bob, 5)), predecessorFacts: factsI(false, false, 2),
  coreProjection: projectedI(N2, 5, IntValue(1), NoInt, NoInt, 2, true, NoCoreError,
    List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), 1)}
pure val fill2PlannedI: AAuthorityPlanned = {predecessor: residualBeforeI, proposedSuccessor: filledBeforeI,
  artifactAndCall: AgreementCallA({before: residualBeforeI, input: inputI(SecondFillId, Bob), now: Time2}),
  input: ChoiceLike({id: "fill2", chooser: Bob, chosen: 1}), operation: fill2I, transactionTime: 2,
  effects: List(transferI(Bob, 5)), predecessorFacts: factsI(true, false, 2),
  coreProjection: projectedI(N0, 0, IntValue(1), IntValue(1), NoInt, 2, true, NoCoreError,
    List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), 1)}
pure def cancelPlannedI(residual: bool): AAuthorityPlanned = {
  val before = if (residual) residualBeforeI else initialBeforeI
  {predecessor: before, proposedSuccessor: before, artifactAndCall: CancellationCallA({before: before, now: Time2}),
    input: NoInput, operation: cancelI, transactionTime: 2, effects: List(), coreProjection: NoCoreProjection,
    predecessorFacts: factsI(residual, false, 2)}
}
pure val cancel0PlannedI = cancelPlannedI(false)
pure val cancel1PlannedI = cancelPlannedI(true)
pure val parentPlanI: AResolvedPlan = {identity: InstallmentPlanA,
  operations: List(fill1PlannedI, fill2PlannedI, cancel0PlannedI, cancel1PlannedI)}
pure def residualI(scenario: ScenarioI): bool = match scenario { | TwoFillsI => true | RecoverI(r) => r.residual }
pure def modeI(scenario: ScenarioI): RecoveryModeI = match scenario { | TwoFillsI => Choice2I | RecoverI(r) => r.mode }
pure def timeI(mode: RecoveryModeI): ATime = match mode {
  | Choice2I => Time2 | Timeout100I => Time100 | Timeout101I => Time101 | Refuse100I => Time100 | Refuse101I => Time101 }
pure def refusedI(mode: RecoveryModeI): bool = Set(Refuse100I, Refuse101I).contains(mode)
pure def suppliedI(mode: RecoveryModeI): bool = mode == Choice2I or refusedI(mode)
pure def recoveryPlannedI(residual: bool, mode: RecoveryModeI): AAuthorityPlanned = {
  val before = if (residual) residualBeforeI else initialBeforeI
  val q = if (residual) 5 else 10
  val first = if (residual) IntValue(1) else NoInt
  val now = timeI(mode)
  val rejected = refusedI(mode)
  val after = if (rejected) before else {program: programI, state: {continuation: N0,
    accounts: accountsI(0), choices: choicesI(first, NoInt, if (suppliedI(mode)) IntValue(1) else NoInt), minimumTime: now}}
  {predecessor: before, proposedSuccessor: after,
    artifactAndCall: AgreementCallA({before: before, input: if (suppliedI(mode)) inputI(RecoveryId, Alice) else NoAInput, now: now}),
    input: if (suppliedI(mode)) ChoiceLike({id: "recover", chooser: Alice, chosen: 1}) else NoInput,
    operation: recoverI, transactionTime: timeValue(now), effects: if (rejected) List() else List(transferI(Alice, q)),
    predecessorFacts: factsI(residual, true, timeValue(now)),
    coreProjection: if (rejected) projectedI(if (residual) N2 else N4, q, first, NoInt, NoInt, 2,
      false, CoreErrorCode("contract_closed"), List(), 0)
    else projectedI(N0, 0, first, NoInt, if (suppliedI(mode)) IntValue(1) else NoInt, timeValue(now), true, NoCoreError,
      List({source: aliceA, recipient: Alice, asset: TokenA, quantity: q}), if (suppliedI(mode)) 1 else 2)}
}
pure def recoveryPlanI(scenario: ScenarioI): AResolvedPlan = {identity: RecoveryPlanA,
  operations: List(recoveryPlannedI(residualI(scenario), modeI(scenario)))}
pure def clauseI(planned: AAuthorityPlanned): Clause = {operation: planned.operation,
  conditions: List(InputIs(planned.input), ParentMatches({key: parentKeyI,
    expected: planned.predecessorFacts.parents.get(parentKeyI)}), SourceBalanceIs({
    location: Escrow(aliceA), asset: TokenA, quantity: planned.predecessorFacts.ledger.get((Escrow(aliceA), TokenA))})),
  requiredEffects: planned.effects, allowedEffects: planned.effects, effectOrder: ExactOrder}
pure val parentBodyI: PolicyBody = {key: parentKeyI, debitLocations: Set(Escrow(aliceA)),
  capabilities: Set(FirstFillCapability, SecondFillCapability, CancelCapability), disclosures: Set(),
  validFrom: 2, validUntil: 99, implementationVersion: 0, enforcementMechanism: 0,
  clauses: List(clauseI(fill1PlannedI), clauseI(fill2PlannedI), clauseI(cancel0PlannedI), clauseI(cancel1PlannedI))}
pure def policyI(body: PolicyBody, plan: AResolvedPlan, profile: SigningProfile): AAuthorityPolicy = {
  body: body, profile: profile, binding: if (profile == SignAfterResolve)
    AfterResolution({identity: plan, operations: plan.operations}) else BeforeResolution(AnyArtifactUnderMechanism)}
pure def parentPolicyI(profile: SigningProfile): AAuthorityPolicy = policyI(parentBodyI, parentPlanI, profile)
pure def recoveryPolicyI(scenario: ScenarioI, profile: SigningProfile): AAuthorityPolicy = {
  val planned = recoveryPlannedI(residualI(scenario), modeI(scenario))
  policyI({key: recoveryKeyI, debitLocations: Set(Escrow(aliceA)), capabilities: Set(RecoveryCapability), disclosures: Set(),
    validFrom: planned.transactionTime, validUntil: planned.transactionTime, implementationVersion: 0, enforcementMechanism: 0,
    clauses: List(clauseI(planned))}, recoveryPlanI(scenario), profile)
}
pure val unsignedI: AAuthorityExecution = {authority: {context: {candidate: initialBeforeI,
  ledger: ledgerI(10, 0, 0), environment: environmentI(2), registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
  parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}, signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)},
  attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
pure def plannedForI(id: AttemptId, scenario: ScenarioI): AAuthorityPlanned = match id {
  | FirstFillAttempt => fill1PlannedI | SecondFillAttempt => fill2PlannedI
  | CancelAttempt => cancel0PlannedI | FreshCancelAttempt => cancel1PlannedI
  | _ => recoveryPlannedI(residualI(scenario), modeI(scenario)) }
pure def planForI(id: AttemptId, scenario: ScenarioI): AResolvedPlan =
  if (id == RecoveryAttempt) recoveryPlanI(scenario) else parentPlanI
pure def expectedObservationI(id: AttemptId, scenario: ScenarioI): AAuthorityObservation = {
  val p = plannedForI(id, scenario)
  {predecessor: p.predecessor, proposedSuccessor: p.proposedSuccessor, artifactAndCall: p.artifactAndCall,
    resolvedPlan: planForI(id, scenario), input: p.input, transactionTime: p.transactionTime,
    effects: p.effects, coreProjection: p.coreProjection, effectEvidence: EvidenceValid, display: PublicDisplay,
    outcome: if (id == FirstFillAttempt) FirstInstallment else if (id == SecondFillAttempt) SecondInstallment
      else if (id == RecoveryAttempt) (if (refusedI(modeI(scenario))) Rejected(CoreRejected(CoreErrorCode("contract_closed"))) else Recovery)
      else Cancellation}
}
pure def actualI(id: AttemptId, scenario: ScenarioI): AAuthorityAdaptation = {
  val p = plannedForI(id, scenario)
  val request = match p.artifactAndCall {
    | AgreementCallA(r) => r | CancellationCallA(r) => {before: r.before, input: NoAInput, now: r.now} }
  adaptAuthorityA(request, p.operation, planForI(id, scenario), PublicDisplay)
}
```

Module `candidate_a_authority_installment_test` initially contains the preamble, fixture import and:

```quint
run fourOperationParentTest = assert(parentPlanI.operations ==
  List(fill1PlannedI, fill2PlannedI, cancel0PlannedI, cancel1PlannedI)
  and PROFILES_I.forall(p => validAuthorityPolicyA(parentPolicyI(p))))
run literalFidelityTest = assert(programI == installmentProgram and SCENARIOS_I.forall(s =>
  Set(FirstFillAttempt, SecondFillAttempt, CancelAttempt, FreshCancelAttempt, RecoveryAttempt).forall(id =>
    actualI(id, s) == AuthorityAdaptedA(expectedObservationI(id, s))))
  and SCENARIOS_I.forall(s => authorityPlanMatchesA(recoveryPlanI(s))))
```

Deadline rejection policies deliberately permit their exact empty-effect planned result for signing; the unchanged projection/financial execution guards refuse commitment. This is separate authorization of a rejected request, not a refund.

- [ ] Gate1: reviewer validates all literal results against frozen semantics, full maps/node table, policy facts and four-operation binding. Root captures RED/GREEN source closure and outcomes before accepting this task.

## Task 2: lifecycle helpers and deterministic authority routes

**Files:** lifecycle and test. **Interfaces:** CommandI, routeI, canCommandI, applyCommandI, runPrefixI, safetyI, financialTerminalI. Lifecycle module is `candidate_a_authority_installment`, imports preamble plus fixtures.

- [ ] Add code below with the compiling RED variant: in ProposeI branch add `(id != FreshCancelAttempt or state.authority.context.registry.get(parentKeyI) == AuthorityUnused)`. Add freshCancellationTest. Typecheck, capture failure after valid first fill and stale loser rejection, then remove that extra guard. Do not repair common authorization.
- [ ] Implement the final helper bodies below.

```quint
type CommandI = PrepareParentI | SignParentI | ProposeI(AttemptId) | VerifyI(AttemptId)
  | CommitI(AttemptId) | RejectStaleI(AttemptId) | AdvanceI | PrepareRecoveryI | SignRecoveryI | RejectRecoveryI
pure val racePrefixI: List[CommandI] = List(PrepareParentI, SignParentI,
  ProposeI(FirstFillAttempt), ProposeI(CancelAttempt), VerifyI(FirstFillAttempt), VerifyI(CancelAttempt))
pure def routeI(scenario: ScenarioI): List[CommandI] = {
  val residual = residualI(scenario)
  val race = if (residual) List(CommitI(FirstFillAttempt), RejectStaleI(CancelAttempt))
    else List(CommitI(CancelAttempt), RejectStaleI(FirstFillAttempt))
  val second = if (scenario == TwoFillsI) List(ProposeI(SecondFillAttempt), VerifyI(SecondFillAttempt), CommitI(SecondFillAttempt))
    else if (residual) List(ProposeI(FreshCancelAttempt), VerifyI(FreshCancelAttempt), CommitI(FreshCancelAttempt)) else List()
  val clock = if (modeI(scenario) == Choice2I) List() else List(AdvanceI)
  val recovery = if (scenario == TwoFillsI) List() else clock.concat(List(PrepareRecoveryI, SignRecoveryI,
    ProposeI(RecoveryAttempt), if (refusedI(modeI(scenario))) RejectRecoveryI else VerifyI(RecoveryAttempt)))
    .concat(if (refusedI(modeI(scenario))) List() else List(CommitI(RecoveryAttempt)))
  racePrefixI.concat(race).concat(second).concat(recovery)
}
pure def selectedPolicyI(id: AttemptId, scenario: ScenarioI, profile: SigningProfile): AAuthorityPolicy =
  if (id == RecoveryAttempt) recoveryPolicyI(scenario, profile) else parentPolicyI(profile)
pure def evidenceI(attempt: AAuthorityAttempt, policy: AAuthorityPolicy): AAuthorityEvidence = {
  effect: {attempt: attempt, disposition: EvidenceValid}, signatures: Map(policy.body.key -> {
    attempt: attempt, disposition: EvidenceValid, signed: {policy: policy, signer: Alice, token: 0}})}
pure def executedI(state: AAuthorityExecution, id: AttemptId): bool = match state.attempts.get(id) {
  | ExecutedOperation(_) => true | _ => false }
pure def cancelledI(state: AAuthorityExecution): bool = match state.authority.context.parents.get(parentKeyI) {
  | ParentLive(live) => live.entry.cancelled | _ => false }
pure def raceReadyI(state: AAuthorityExecution): bool = match state.attempts.get(FirstFillAttempt) {
  | VerifiedOperation(f) => match state.attempts.get(CancelAttempt) {
      | VerifiedOperation(c) => f.attempt.context == c.attempt.context and c.attempt.context == state.authority.context
      | _ => false }
  | _ => false }
pure def canCommandI(state: AAuthorityExecution, scenario: ScenarioI, profile: SigningProfile, cmd: CommandI): bool = match cmd {
  | PrepareParentI => canPrepareSigningAuthorityA(state.authority, parentPolicyI(profile), Alice)
  | SignParentI => canSignAuthorityA(state.authority, parentPolicyI(profile), Alice, 0)
  | PrepareRecoveryI => cancelledI(state) and canPrepareSigningAuthorityA(state.authority, recoveryPolicyI(scenario, profile), Alice)
  | SignRecoveryI => cancelledI(state) and canSignAuthorityA(state.authority, recoveryPolicyI(scenario, profile), Alice, 0)
  | AdvanceI => cancelledI(state) and state.authority.context.environment.physicalTime == 2 and modeI(scenario) != Choice2I
  | ProposeI(id) => match actualI(id, scenario) {
      | AuthorityAdaptedA(obs) => validAuthorityExecutionA(state) and
          canPropose(state, id, plannedForI(id, scenario).operation, obs,
            if (Set(FirstFillAttempt, SecondFillAttempt).contains(id)) Bob else Alice)
      | _ => false }
  | VerifyI(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => canVerifyAuthorityA(state, id, evidenceI(a, selectedPolicyI(id, scenario, profile)))
      | _ => false }
  | CommitI(id) => canCommitAuthorityA(state, id)
      and (not(Set(FirstFillAttempt, CancelAttempt).contains(id)) or raceReadyI(state))
  | RejectStaleI(id) => canRejectVerifiedAuthorityA(state, id)
  | RejectRecoveryI => match state.attempts.get(RecoveryAttempt) {
      | ProposedAttempt(a) => canRejectProposedAuthorityA(state, RecoveryAttempt, evidenceI(a, recoveryPolicyI(scenario, profile)))
      | _ => false }
}
pure def applyCommandI(state: AAuthorityExecution, scenario: ScenarioI, profile: SigningProfile, cmd: CommandI): AAuthorityExecution = match cmd {
  | PrepareParentI => {...state, authority: applyPrepareSigning(state.authority, parentPolicyI(profile), Alice)}
  | SignParentI => {...state, authority: applySign(state.authority, parentPolicyI(profile), Alice, 0)}
  | PrepareRecoveryI => {...state, authority: applyPrepareSigning(state.authority, recoveryPolicyI(scenario, profile), Alice)}
  | SignRecoveryI => {...state, authority: applySign(state.authority, recoveryPolicyI(scenario, profile), Alice, 0)}
  | AdvanceI => {...state, authority: {...state.authority, context: {...state.authority.context,
      environment: environmentI(timeValue(timeI(modeI(scenario))))}}}
  | ProposeI(id) => match actualI(id, scenario) {
      | AuthorityAdaptedA(obs) => applyPropose(state, id, plannedForI(id, scenario).operation, obs,
          if (Set(FirstFillAttempt, SecondFillAttempt).contains(id)) Bob else Alice)
      | _ => state }
  | VerifyI(id) => match state.attempts.get(id) {
      | ProposedAttempt(a) => applyVerify(state, id, evidenceI(a, selectedPolicyI(id, scenario, profile)))
      | _ => state }
  | CommitI(id) => applyCommit(state, id)
  | RejectStaleI(id) => applyRejectVerifiedAuthorityA(state, id)
  | RejectRecoveryI => match state.attempts.get(RecoveryAttempt) {
      | ProposedAttempt(a) => applyRejectProposedAuthorityA(state, RecoveryAttempt, evidenceI(a, recoveryPolicyI(scenario, profile)))
      | _ => state }
}
pure def noPendingI(state: AAuthorityExecution): bool = ATTEMPT_IDS.forall(id => match state.attempts.get(id) {
  | ProposedAttempt(_) => false | VerifiedOperation(_) => false | _ => true })
  and AUTHORITY_KEYS.forall(key => match state.authority.signing.get(key) { | PreparedSigning(_) => false | _ => true })
pure def financialTerminalI(state: AAuthorityExecution): bool = state.authority.context.candidate.state.continuation == N0
  and balance(state.authority.context.ledger, Escrow(aliceA), TokenA) == 0 and noPendingI(state)
pure def retainedRaceI(state: AAuthorityExecution): bool = Set(FirstFillAttempt, CancelAttempt).forall(id =>
  match state.attempts.get(id) {
    | RejectedOperation(r) => r.reason == StaleBindings and r.stage == CommitBoundary and r.attempt.id == id
        and r.attempt.context != r.observedContext and authorityAttemptMatchesA(r.attempt)
        and verificationValid(r.attempt.context, r.attempt, r.evidence)
        and r.evidence.effect.attempt == r.attempt
    | _ => true })
pure def recoveryPreservesI(state: AAuthorityExecution): bool = match state.attempts.get(RecoveryAttempt) {
  | ExecutedOperation(done) => state.authority.context.parents == done.attempt.context.parents
      and state.authority.context.registry.get(parentKeyI) == done.attempt.context.registry.get(parentKeyI)
      and state.authority.context.registry.get(recoveryKeyI) == AuthorityConsumed({
        signed: done.evidence.signatures.get(recoveryKeyI).signed, revision: 1})
  | _ => true }
pure def safetyI(state: AAuthorityExecution): bool = validAuthorityExecutionA(state)
  and totalAsset(state.authority.context.ledger, TokenA) == 10 and totalAsset(state.authority.context.ledger, TokenB) == 0
  and retainedRaceI(state) and recoveryPreservesI(state)
  and match state.authority.context.parents.get(parentKeyI) {
    | ParentVacant => state.authority.context.candidate == initialBeforeI and state.authority.context.ledger == ledgerI(10, 0, 0)
    | ParentLive(live) => live.entry.paid + live.entry.remainingAllowance == 10
        and balance(state.authority.context.ledger, Wallet(Bob), TokenA) == live.entry.paid
        and (if (executedI(state, RecoveryAttempt)) live.entry.cancelled
          and balance(state.authority.context.ledger, Escrow(aliceA), TokenA) == 0
          and balance(state.authority.context.ledger, Wallet(Alice), TokenA) == live.entry.remainingAllowance
        else balance(state.authority.context.ledger, Escrow(aliceA), TokenA) == live.entry.remainingAllowance
          and balance(state.authority.context.ledger, Wallet(Alice), TokenA) == 0) }
pure def transitionExactI(before: AAuthorityExecution, after: AAuthorityExecution, cmd: CommandI): bool = match cmd {
  | CommitI(id) => after == applyCommit(before, id)
  | AdvanceI => {...after.authority.context, environment: before.authority.context.environment} == before.authority.context
      and after.attempts == before.attempts and after.authority.signing == before.authority.signing
  | SignParentI => after.authority.context.candidate == before.authority.context.candidate
      and after.authority.context.ledger == before.authority.context.ledger
  | SignRecoveryI => after.authority.context.candidate == before.authority.context.candidate
      and after.authority.context.ledger == before.authority.context.ledger
      and after.authority.context.parents == before.authority.context.parents
      and after.authority.context.registry.get(parentKeyI) == before.authority.context.registry.get(parentKeyI)
  | _ => after.authority.context == before.authority.context }
type PrefixI = {state: AAuthorityExecution, ok: bool, visited: List[CommandI]}
pure def runPrefixI(scenario: ScenarioI, profile: SigningProfile, count: int): PrefixI =
  routeI(scenario).slice(0, count).foldl({state: unsignedI, ok: safetyI(unsignedI), visited: List()}, (acc, cmd) => {
    val commandReady = canCommandI(acc.state, scenario, profile, cmd)
    val after = if (commandReady) applyCommandI(acc.state, scenario, profile, cmd) else acc.state
    {state: after, ok: acc.ok and commandReady and safetyI(after) and transitionExactI(acc.state, after, cmd),
      visited: if (commandReady) acc.visited.append(cmd) else acc.visited}
  })
```

runPrefixI is a pure test interpreter whose ok flag must be asserted. It is not an enabled no-op action. The scenario scheduler fixes initial race preparation before either commit; this finite workload excludes unrestricted interleavings. canCommandI still uses the actual boundary, and common proposal remains permissive for adversarial tests.

Add lifecycle import and these definitions to tests:

```quint
pure def expectedParentI(profile: SigningProfile, residual: bool, cancelled: bool): ParentCell[APredecessor,AContinuation,ACall,AResolvedPlan] = {
  val parent = {key: parentKeyI, policy: {policy: parentPolicyI(profile), signer: Alice, token: 0},
    source: Escrow(aliceA), recipient: Bob, asset: TokenA, budget: 10, slots: Map(1 -> 5, 2 -> 5)}
  ParentLive({parent: parent, entry: {claim: if (residual or cancelled) Claimed(parent) else Unclaimed,
    usedSlots: if (residual) Set(1) else Set(), paid: if (residual) 5 else 0, remainingAllowance: if (residual) 5 else 10,
    cancelled: cancelled, revision: (if (residual) 1 else 0) + (if (cancelled) 1 else 0)}})
}
run registrationTest = assert(PROFILES_I.forall(p => {
  val result = runPrefixI(TwoFillsI, p, 2)
  result.ok and result.state.authority.context.candidate == initialBeforeI
    and result.state.authority.context.ledger == ledgerI(10, 0, 0) and result.state.attempts == unsignedI.attempts
    and result.state.authority.context.parents.get(parentKeyI) == expectedParentI(p, false, false)
    and result.state.authority.context.registry.get(parentKeyI) == AuthorityRegistered({policy: parentPolicyI(p), signer: Alice, token: 0})
}))
run bothRaceOrdersTest = assert(PROFILES_I.forall(p => Set(false, true).forall(residual => {
  val s = RecoverI({residual: residual, mode: Choice2I})
  val ready = runPrefixI(s, p, 6)
  val winner = runPrefixI(s, p, 7)
  val rejected = runPrefixI(s, p, 8)
  val loser = if (residual) CancelAttempt else FirstFillAttempt
  ready.ok and winner.ok and rejected.ok and raceReadyI(ready.state)
    and not(canCommitAuthorityA(winner.state, loser))
    and winner.state.attempts.get(loser) == ready.state.attempts.get(loser)
    and rejected.state.authority == winner.state.authority
    and rejected.state.authority.context.parents.get(parentKeyI) == expectedParentI(p, residual, not(residual))
    and rejected.state.authority.context.candidate == (if (residual) residualBeforeI else initialBeforeI)
    and match ready.state.attempts.get(loser) {
      | VerifiedOperation(v) => rejected.state.attempts.get(loser) == RejectedOperation({attempt: v.attempt, evidence: v.evidence,
          observedContext: winner.state.authority.context, reason: StaleBindings, stage: CommitBoundary})
      | _ => false }
})))
run twoFillsTest = assert(PROFILES_I.forall(p => {
  val result = runPrefixI(TwoFillsI, p, 11)
  result.ok and financialTerminalI(result.state) and result.state.authority.context.candidate == filledBeforeI
    and result.state.authority.context.ledger == ledgerI(0, 0, 10)
    and match result.state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => live.entry.usedSlots == Set(1, 2) and live.entry.paid == 10
          and live.entry.remainingAllowance == 0 and live.entry.revision == 2 and not(live.entry.cancelled)
          and result.state.authority.context.registry.get(parentKeyI) == AuthorityConsumed({signed: live.parent.policy, revision: 2})
      | _ => false }
}))
run freshCancellationTest = assert(PROFILES_I.forall(p => {
  val s = RecoverI({residual: true, mode: Choice2I})
  val before = runPrefixI(s, p, 8)
  val after = runPrefixI(s, p, 11)
  before.ok and after.ok and after.state.authority.context.candidate == residualBeforeI
    and after.state.authority.context.ledger == ledgerI(5, 0, 5)
    and after.state.authority.signing == before.state.authority.signing
    and after.state.authority.context.parents.get(parentKeyI) == expectedParentI(p, true, true)
    and not(canPrepareSigningAuthorityA(before.state.authority, parentPolicyI(p), Alice))
    and canCommandI(before.state, s, p, ProposeI(SecondFillAttempt))
    and match after.state.attempts.get(FreshCancelAttempt) {
      | ExecutedOperation(done) => done.attempt.context == before.state.authority.context
          and done.evidence.signatures.get(parentKeyI).signed == {policy: parentPolicyI(p), signer: Alice, token: 0}
          and done.attempt.observation == expectedObservationI(FreshCancelAttempt, s)
      | _ => false }
}))
run recoveryMatrixTest = assert(PROFILES_I.forall(p => SCENARIOS_I.exclude(Set(TwoFillsI)).forall(s => {
  val result = runPrefixI(s, p, routeI(s).length())
  val cancelled = runPrefixI(s, p, if (residualI(s)) 11 else 8)
  val registered = runPrefixI(s, p, routeI(s).length() - (if (refusedI(modeI(s))) 2 else 3))
  val done = result.state
  result.ok and cancelled.ok and registered.ok and done.authority.context.parents == cancelled.state.authority.context.parents
    and done.authority.context.registry.get(parentKeyI) == cancelled.state.authority.context.registry.get(parentKeyI)
    and (if (refusedI(modeI(s))) not(financialTerminalI(done)) and noPendingI(done)
      and done.authority.context.candidate == (if (residualI(s)) residualBeforeI else initialBeforeI)
      and done.authority.context.ledger == (if (residualI(s)) ledgerI(5, 0, 5) else ledgerI(10, 0, 0))
      and done.authority == registered.state.authority
      and done.authority.context.registry.get(recoveryKeyI) == AuthorityRegistered({
        policy: recoveryPolicyI(s, p), signer: Alice, token: 0})
      and done.authority.signing.get(recoveryKeyI) == CompletedSigning({
        policy: recoveryPolicyI(s, p), signer: Alice, token: 0})
      and match done.attempts.get(RecoveryAttempt) {
        | RejectedOperation(r) => {
            val attempt: AAuthorityAttempt = {id: RecoveryAttempt, actor: Alice, operation: recoverI,
              observation: expectedObservationI(RecoveryAttempt, s), context: registered.state.authority.context}
            done == {...registered.state, attempts: registered.state.attempts.put(RecoveryAttempt, RejectedOperation({
              attempt: attempt, evidence: evidenceI(attempt, recoveryPolicyI(s, p)),
              observedContext: registered.state.authority.context,
              reason: CoreRejected(CoreErrorCode("contract_closed")), stage: VerificationBoundary}))}
          }
        | _ => false }
    else financialTerminalI(done)
      and done.authority.context.candidate == recoveryPlannedI(residualI(s), modeI(s)).proposedSuccessor
      and done.authority.context.ledger == (if (residualI(s)) ledgerI(0, 5, 5) else ledgerI(0, 10, 0))
      and recoveryPreservesI(done))
})))
```

- [ ] Gate2: exact-source nonauthor review of original nonce0 signature, fresh cancellation, two-fill revision2, both verified race losers, genuine Core rollback and nonce1 recovery. Root captures actual RED/GREEN and does not mark acceptance from code presence.

## Task 3: negatives, guarded harness and action witnesses

**Files:** test and harness. Harness module is `candidate_a_authority_installment_harness`, with full preamble and fixture/lifecycle imports.

- [ ] Add the tests below. To capture a compiling behavioral RED for the A boundary wrapper, temporarily replace canVerifyAuthorityA by canVerify in VerifyI. changedUnusedNodeTest must fail. Capture failure before restoring the A guard; if this mutant fails to trigger, record it as ineffective and investigate, never invent RED.
- [ ] Add these exact tests:

```quint
pure def rejectedUnchangedI(before: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence,
  expectedReason: RejectionReason): bool = {
  val after = applyRejectProposedAuthorityA(before, id, evidence)
  canRejectProposedAuthorityA(before, id, evidence) and match before.attempts.get(id) {
    | ProposedAttempt(attempt) => after == {...before, attempts: before.attempts.put(id, RejectedOperation({
        attempt: attempt, evidence: evidence, observedContext: before.authority.context,
        reason: expectedReason, stage: VerificationBoundary}))}
    | _ => false }
}
run negativeAuthorityTest = assert(PROFILES_I.forall(p => {
  val s = RecoverI({residual: false, mode: Choice2I})
  val registered = runPrefixI(s, p, 2)
  val cancelled = runPrefixI(s, p, 8)
  val obs = expectedObservationI(RecoveryAttempt, s)
  val noCancel = applyPropose(registered.state, RecoveryAttempt, recoverI, obs, Alice)
  val unsigned = applyPropose(cancelled.state, RecoveryAttempt, recoverI, obs, Alice)
  registered.ok and cancelled.ok
    and not(operationFinancialGuard(registered.state.authority.context, recoverI, obs.effects))
    and not(canCommandI(registered.state, s, p, PrepareRecoveryI))
    and canPropose(registered.state, RecoveryAttempt, recoverI, obs, Alice)
    and canPropose(cancelled.state, RecoveryAttempt, recoverI, obs, Alice)
    and match noCancel.attempts.get(RecoveryAttempt) {
      | ProposedAttempt(a) => {
          val e = evidenceI(a, recoveryPolicyI(s, p))
          not(canVerifyAuthorityA(noCancel, RecoveryAttempt, e))
            and authorityRejectionReasonA(noCancel, a, e) == ConsumptionConflict
            and rejectedUnchangedI(noCancel, RecoveryAttempt, e, ConsumptionConflict)
        } | _ => false }
    and match unsigned.attempts.get(RecoveryAttempt) {
      | ProposedAttempt(a) => {
          val e = evidenceI(a, recoveryPolicyI(s, p))
          val old = evidenceI(a, parentPolicyI(p))
          not(canVerifyAuthorityA(unsigned, RecoveryAttempt, e))
            and authorityRejectionReasonA(unsigned, a, e) == UnauthorizedEffect
            and not(canVerifyAuthorityA(unsigned, RecoveryAttempt, old))
            and authorityRejectionReasonA(unsigned, a, old) == EvidenceMissing
            and rejectedUnchangedI(unsigned, RecoveryAttempt, e, UnauthorizedEffect)
            and rejectedUnchangedI(unsigned, RecoveryAttempt, old, EvidenceMissing)
        } | _ => false }
    and not(canPrepareSigningAuthorityA(cancelled.state.authority, parentPolicyI(p), Alice))
}))
run duplicateTest = assert(PROFILES_I.forall(p => SCENARIOS_I.forall(s => {
  val result = runPrefixI(s, p, routeI(s).length())
  result.ok and ATTEMPT_IDS.forall(id => match result.state.attempts.get(id) {
    | ExecutedOperation(done) => not(canCommitAuthorityA(result.state, id))
        and not(canPropose(result.state, id, done.attempt.operation, done.attempt.observation, done.attempt.actor))
    | _ => true })
    and (if (cancelledI(result.state)) match result.state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => not(canCancelParent(live.parent, live.entry, live.entry))
          and not(canConsumeSlot(live.parent, live.entry, 1, live.entry))
          and not(canConsumeSlot(live.parent, live.entry, 2, live.entry))
      | _ => false } else true)
})))
run changedUnusedNodeTest = assert({
  val s = TwoFillsI
  val p = SignBeforeResolve
  val ready = runPrefixI(s, p, 4)
  match ready.state.attempts.get(FirstFillAttempt) {
    | ProposedAttempt(a) => {
        val changed = {...a, observation: {...a.observation, proposedSuccessor: {...a.observation.proposedSuccessor,
          program: {...programI, nodes: programI.nodes.put(N15, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}}}}
        val forged = {...ready.state, attempts: ready.state.attempts.put(FirstFillAttempt, ProposedAttempt(changed))}
        val e = evidenceI(changed, parentPolicyI(p))
        ready.ok and not(canCommandI(forged, s, p, VerifyI(FirstFillAttempt)))
          and authorityRejectionReasonA(forged, changed, e) == UnauthorizedEffect
          and rejectedUnchangedI(forged, FirstFillAttempt, e, UnauthorizedEffect)
      } | _ => false }
})
run freshDuplicateCancellationTest = assert(PROFILES_I.forall(p => {
  val s = RecoverI({residual: false, mode: Choice2I})
  val base = runPrefixI(s, p, 8)
  val proposed = applyPropose(base.state, FreshCancelAttempt, cancelI, expectedObservationI(CancelAttempt, s), Alice)
  base.ok and canPropose(base.state, FreshCancelAttempt, cancelI, expectedObservationI(CancelAttempt, s), Alice)
    and match proposed.attempts.get(FreshCancelAttempt) {
      | ProposedAttempt(a) => {
          val e = evidenceI(a, parentPolicyI(p))
          not(canVerifyAuthorityA(proposed, FreshCancelAttempt, e))
            and authorityRejectionReasonA(proposed, a, e) == ConsumptionConflict
            and rejectedUnchangedI(proposed, FreshCancelAttempt, e, ConsumptionConflict)
        } | _ => false }
}))
```

- [ ] Add the harness. The scenario/profile are independent environment state; cursor/history describe the finite workload. False terminal fallback assigns all variables under a false guard. Cancellation/refusal can end the selected workload while financialTerminalI remains false.

The no-cancellation proposal also lacks a registered nonce1 signature: its end-to-end rejection is a composite negative. The separate `operationFinancialGuard` and `PrepareRecoveryI` assertions isolate the cancellation precondition; do not claim cancellation is the only failing end-to-end condition.

```quint
var state: AAuthorityExecution
var scenario: ScenarioI
var profile: SigningProfile
var cursor: int
var history: List[CommandI]
var lastAtomicI: bool
action init = {
  nondet chosenScenario = SCENARIOS_I.oneOf()
  nondet chosenProfile = PROFILES_I.oneOf()
  all { state' = unsignedI, scenario' = chosenScenario, profile' = chosenProfile, cursor' = 0, history' = List(), lastAtomicI' = true }
}
action performI(cmd: CommandI): bool = all {
  cursor < routeI(scenario).length(), routeI(scenario).nth(cursor) == cmd,
  canCommandI(state, scenario, profile, cmd),
  state' = applyCommandI(state, scenario, profile, cmd), scenario' = scenario, profile' = profile,
  cursor' = cursor + 1, history' = history.append(cmd),
  lastAtomicI' = transitionExactI(state, applyCommandI(state, scenario, profile, cmd), cmd),
}
action step = if (cursor < routeI(scenario).length()) performI(routeI(scenario).nth(cursor))
  else all { false, state' = state, scenario' = scenario, profile' = profile, cursor' = cursor, history' = history, lastAtomicI' = lastAtomicI }
def seenI(cmd: CommandI): bool = history.foldl(false, (found, actual) => found or actual == cmd)
val installmentSafetyI = safetyI(state) and lastAtomicI and cursor == history.length() and cursor <= routeI(scenario).length()
val actionWitnessI = history.length() > 0
val allCasesWitnessI = cursor == routeI(scenario).length() and cursor > 0
val parentPreparedI = seenI(PrepareParentI)
val parentSignedI = seenI(SignParentI)
val firstProposedI = seenI(ProposeI(FirstFillAttempt))
val cancelProposedI = seenI(ProposeI(CancelAttempt))
val firstVerifiedI = seenI(VerifyI(FirstFillAttempt))
val cancelVerifiedI = seenI(VerifyI(CancelAttempt))
val firstCommittedI = seenI(CommitI(FirstFillAttempt))
val cancelCommittedI = seenI(CommitI(CancelAttempt))
val firstRejectedI = seenI(RejectStaleI(FirstFillAttempt))
val cancelRejectedI = seenI(RejectStaleI(CancelAttempt))
val secondProposedI = seenI(ProposeI(SecondFillAttempt))
val secondVerifiedI = seenI(VerifyI(SecondFillAttempt))
val secondCommittedI = seenI(CommitI(SecondFillAttempt))
val freshProposedI = seenI(ProposeI(FreshCancelAttempt))
val freshVerifiedI = seenI(VerifyI(FreshCancelAttempt))
val freshCommittedI = seenI(CommitI(FreshCancelAttempt))
val clockAdvancedI = seenI(AdvanceI)
val recoveryPreparedI = seenI(PrepareRecoveryI)
val recoverySignedI = seenI(SignRecoveryI)
val recoveryProposedI = seenI(ProposeI(RecoveryAttempt))
val recoveryVerifiedI = seenI(VerifyI(RecoveryAttempt))
val recoveryCommittedI = seenI(CommitI(RecoveryAttempt))
val recoveryRejectedI = seenI(RejectRecoveryI)
val afterProfileCompletedI = allCasesWitnessI and profile == SignAfterResolve
val beforeProfileCompletedI = allCasesWitnessI and profile == SignBeforeResolve
```

Add namespaced harness import and stateful test initialization to tests. Every pure scenario/profile pair is already deterministically tested; action tests additionally check the actual assignment route.

```quint
import candidate_a_authority_installment_harness as H from "./candidate_a_authority_installment_harness"
action initCaseI(s: ScenarioI, p: SigningProfile): bool = all {
  H::state' = unsignedI, H::scenario' = s, H::profile' = p, H::cursor' = 0, H::history' = List(), H::lastAtomicI' = true }
run initialWitnessesFalseTest = H::init.expect(not(H::actionWitnessI or H::allCasesWitnessI))
run residualRecoveryActionTest = initCaseI(RecoverI({residual: true, mode: Choice2I}), SignAfterResolve)
  .then(16.reps(_ => H::step)).expect(H::allCasesWitnessI and H::installmentSafetyI and financialTerminalI(H::state))
run beforeRecoveryActionTest = initCaseI(RecoverI({residual: true, mode: Choice2I}), SignBeforeResolve)
  .then(16.reps(_ => H::step)).expect(H::allCasesWitnessI and H::installmentSafetyI and financialTerminalI(H::state))
run timeout101ActionTest = initCaseI(RecoverI({residual: true, mode: Timeout101I}), SignAfterResolve)
  .then(17.reps(_ => H::step)).expect(H::allCasesWitnessI and H::clockAdvancedI and H::recoveryCommittedI)
run refused100ActionTest = initCaseI(RecoverI({residual: false, mode: Refuse100I}), SignBeforeResolve)
  .then(13.reps(_ => H::step)).expect(H::allCasesWitnessI and H::recoveryRejectedI and not(financialTerminalI(H::state)))
run routeBoundsTest = assert(SCENARIOS_I.forall(s => routeI(s).length() <= 17)
  and routeI(RecoverI({residual: true, mode: Timeout101I})).length() == 17
  and routeI(RecoverI({residual: true, mode: Choice2I})).length() == 16
  and routeI(RecoverI({residual: false, mode: Refuse100I})).length() == 13
  and routeI(TwoFillsI).length() == 11)
```

### Bound derivation and verification

Longest actual route: parent prepare/sign2 + two proposal/verification pairs4 + winner commit/stale-loser rejection2 + fresh cancel proposal/verify/commit3 + clock advance1 + recovery prepare/sign2 + recovery propose/verify/commit3 =17 transitions after init (18 states). Residual time2 recovery is16; initial timeout14; initial refusal13; residual refusal16; two fills11. Use max-steps20, margin3. Core's internal22 reduction-iteration bound is unrelated and unchanged.

- [ ] Run every new module's typecheck, all deterministic tests (names end Test), then samples and regressions. Root captures raw output and source closure before subsequent changes.

```bash
quint --version
quint typecheck specs/quint/s02/candidate_a_authority_installment_fixtures.qnt
quint typecheck specs/quint/s02/candidate_a_authority_installment.qnt
quint typecheck specs/quint/s02/candidate_a_authority_installment_harness.qnt
quint typecheck specs/quint/s02/candidate_a_authority_installment_test.qnt
quint test specs/quint/s02/candidate_a_authority_installment_test.qnt --backend=rust --seed=42 --match '.*Test'
quint run specs/quint/s02/candidate_a_authority_installment_harness.qnt --backend=rust --seed=42 --max-samples=100 --max-steps=20 --invariant=installmentSafetyI --witnesses parentPreparedI parentSignedI firstProposedI cancelProposedI firstVerifiedI cancelVerifiedI firstCommittedI cancelCommittedI firstRejectedI cancelRejectedI secondProposedI secondVerifiedI secondCommittedI freshProposedI freshVerifiedI freshCommittedI clockAdvancedI recoveryPreparedI recoverySignedI recoveryProposedI recoveryVerifiedI recoveryCommittedI recoveryRejectedI afterProfileCompletedI beforeProfileCompletedI --verbosity=1
quint test specs/quint/s02/candidate_a_authority_boundary_test.qnt --backend=rust --seed=42
quint test specs/quint/s02/candidate_a_authority_adapter_test.qnt --backend=rust --seed=42
/home/charl/Moriarty/.venv/bin/python -m pytest -q
```

Start with100 samples. Only if a named witness is absent, inspect its deterministic path and rerun the same command with --max-samples=1000; retain both receipts and the missing witness name. Do not escalate after all counts are nonzero. The22 deterministic scenario/profile combinations supply finite inventory coverage.

Expected: typechecks exit0; deterministic tests all pass; sample invariant has no violation and each of23 action witnesses plus2 profile witnesses has a nonzero count. Witness arguments are space-separated. Zero is an unmet gate requiring trace inspection, not evidence of safety. Never call samples proofs.

- [ ] Gate3: independent review of exact final four-file source closure, all22 scenario/profile combinations, behavioral RED/GREEN, complete per-action counts, financial terminal distinction, and limitations. Root maps outcomes to A2 EARS/OpenSpec and commits only after review acceptance and fresh verification. Preserve old artifacts; rollback is a scoped follow-up commit, never a reset.

## Evidence output contract and source-compatibility risks

Root owns receipt production. Each immutable receipt records:
`{requirementId,scenario,profile,command,cwd,sourceCommit,sourceClosure:[{path,sha256}],tool:{path,version,sha256},exitCode,stdoutPath,stderrPath,tracePaths,expectedPredicate,observedResult,classification}`.
Classification is behavioral-red, green, type-error, diagnostic, sampled-no-violation or witness-count. Trace inventory must preserve original requests, full observations/results/effects, policies/evidence, contexts and attempt cells. No invented counts/hashes/results.

Compatibility risks: generic variant/map inference may need the exact frozen type aliases; namespace assignments must be validated against installed Quint before runtime capture. The imported frozen APIs/signatures above were inspected, but plan blocks are specified-only. Single RecoveryAttempt forbids overwriting a rejected deadline request with cleanup; refusal and NoInput refunds are separate traces. Raw Time0 remains outside authority adapter admission; installment's legitimate initial minimumTime is2. No permitted fix may widen attempt IDs, weaken common rules, re-sign nonce0, or substitute generic string outcomes. This scheduled finite workload does not establish arbitrary-program correspondence, unrestricted interleavings, cryptography or exhaustive model checking.
