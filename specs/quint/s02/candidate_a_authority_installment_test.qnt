module candidate_a_authority_installment_test {
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
import candidate_a_authority_installment_fixtures.* from "./candidate_a_authority_installment_fixtures"
import candidate_a_authority_installment.* from "./candidate_a_authority_installment"

run fourOperationParentTest = assert(parentPlanI.operations ==
  List(fill1PlannedI, fill2PlannedI, cancel0PlannedI, cancel1PlannedI)
  and PROFILES_I.forall(p => validAuthorityPolicyA(parentPolicyI(p))))
run literalFidelityTest = assert(programI == installmentProgram and SCENARIOS_I.forall(s =>
  Set(FirstFillAttempt, SecondFillAttempt, CancelAttempt, FreshCancelAttempt, RecoveryAttempt).forall(id =>
    actualI(id, s) == AuthorityAdaptedA(expectedObservationI(id, s))))
  and SCENARIOS_I.forall(s => authorityPlanMatchesA(recoveryPlanI(s))))

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
}
