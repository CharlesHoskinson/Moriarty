module candidate_a_authority_installment {
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
}
