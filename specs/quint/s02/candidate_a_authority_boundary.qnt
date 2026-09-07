module candidate_a_authority_boundary {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
  type AAuthorityContext = AuthorityContext[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthoritySigning = SigningState[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityAttempt = PreparedAttempt[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityVerified = VerifiedAttempt[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityPlanned = PlannedOperation[APredecessor,AContinuation,ACall]

  // Validate complete finite maps before account/escrow lookups.
  pure def authorityPredecessorLedgerMatchesA(before: APredecessor, ledger: Ledger): bool =
    if (validState(before.program, before.state) and validLedger(ledger))
      CORE_ACCOUNTS.forall(account =>
        before.state.accounts.get(account) == ledger.get((Escrow(account), account.asset)))
    else false
  pure def authorityPlannedOperationMatchesA(planned: AAuthorityPlanned, plan: AResolvedPlan): bool =
    if (validPolicyFacts(planned.predecessorFacts)
        and authorityPredecessorLedgerMatchesA(planned.predecessor, planned.predecessorFacts.ledger)
        and planned.transactionTime == planned.predecessorFacts.environment.physicalTime) {
      val request = match planned.artifactAndCall {
        | AgreementCallA(call) => call
        | CancellationCallA(call) => {before: call.before, input: NoAInput, now: call.now}
      }
      match adaptAuthorityA(request, planned.operation, plan, PublicDisplay) {
        | AuthorityAdaptedA(obs) => plannedMatches(planned, planned.operation, obs, planned.predecessorFacts)
        | _ => false
      }
    } else false
  // Fidelity is not permission, uniqueness or future-state reachability.
  pure def authorityPlanMatchesA(plan: AResolvedPlan): bool =
    plan.operations.length() >= 1 and plan.operations.length() <= 4
      and plan.operations.foldl(true, (ok, planned) => ok and authorityPlannedOperationMatchesA(planned, plan))
  pure def validAuthorityPolicyA(policy: AAuthorityPolicy): bool =
    validPolicy(policy) and match policy.binding {
      | BeforeResolution(_) => true
      | AfterResolution(view) => view.operations == view.identity.operations and authorityPlanMatchesA(view.identity)
    }
  pure def validAuthoritySigningA(state: AAuthoritySigning): bool =
    validSigningState(state) and authorityPredecessorLedgerMatchesA(state.context.candidate, state.context.ledger)
  pure def canPrepareSigningAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal): bool =
    validAuthoritySigningA(state) and validAuthorityPolicyA(policy) and canPrepareSigning(state, policy, signer)
  pure def canSignAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal, token: int): bool =
    validAuthoritySigningA(state) and validAuthorityPolicyA(policy) and canSign(state, policy, signer, token)

  // Retained stale attempts remain representable. Freshness belongs to the gate.
  pure def validAuthorityExecutionA(state: AAuthorityExecution): bool =
    validExecutionState(state) and authorityPredecessorLedgerMatchesA(
      state.authority.context.candidate, state.authority.context.ledger)
  pure def authorityAttemptMatchesA(attempt: AAuthorityAttempt): bool =
    authorityObservationMatchesA(attempt.operation, attempt.observation)
      and authorityPlanMatchesA(attempt.observation.resolvedPlan)
  pure def authorityEvidencePoliciesMatchA(evidence: AAuthorityEvidence): bool =
    evidence.signatures.keys().forall(key => validAuthorityPolicyA(evidence.signatures.get(key).signed.policy))
  pure def canVerifyAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool =
    if (validAuthorityExecutionA(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id and authorityAttemptMatchesA(attempt)
          and authorityEvidencePoliciesMatchA(evidence) and canVerify(state, id, evidence)
      | _ => false
    } else false
  pure def canCommitAuthorityA(state: AAuthorityExecution, id: AttemptId): bool =
    if (validAuthorityExecutionA(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id and authorityAttemptMatchesA(verified.attempt)
          and authorityEvidencePoliciesMatchA(verified.evidence) and canCommit(state, id)
      | _ => false
    } else false
  // Structural envelope only: the semantic predicate that failed cannot strand
  // an otherwise representable proposal/verified record.
  pure def authorityRejectionDomainA(state: AAuthorityExecution): bool =
    state.attempts.keys() == ATTEMPT_IDS and state.authority.signing.keys() == AUTHORITY_KEYS
      and state.authority.context.registry.keys() == AUTHORITY_KEYS
      and state.authority.context.parents.keys() == AUTHORITY_KEYS
  pure def authorityRejectionReasonA(state: AAuthorityExecution, attempt: AAuthorityAttempt,
    evidence: AAuthorityEvidence): RejectionReason =
    if (not(authorityPredecessorLedgerMatchesA(state.authority.context.candidate, state.authority.context.ledger))
        or not(authorityAttemptMatchesA(attempt)) or not(authorityEvidencePoliciesMatchA(evidence)))
      UnauthorizedEffect
    else rejectionReason(state.authority.context, attempt, evidence)
  pure def canRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool =
    if (authorityRejectionDomainA(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id and not(canVerifyAuthorityA(state, id, evidence))
      | _ => false
    } else false
  pure def canRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): bool =
    if (authorityRejectionDomainA(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id and not(canCommitAuthorityA(state, id))
      | _ => false
    } else false
  pure def applyRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId,
    evidence: AAuthorityEvidence): AAuthorityExecution =
    if (canRejectProposedAuthorityA(state, id, evidence)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => {...state, attempts: state.attempts.put(id, RejectedOperation({
          attempt: attempt, evidence: evidence, observedContext: state.authority.context,
          reason: authorityRejectionReasonA(state, attempt, evidence), stage: VerificationBoundary}))}
      | _ => state
    } else state
  pure def applyRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): AAuthorityExecution =
    if (canRejectVerifiedAuthorityA(state, id)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => {...state, attempts: state.attempts.put(id, RejectedOperation({
          attempt: verified.attempt, evidence: verified.evidence, observedContext: state.authority.context,
          reason: authorityRejectionReasonA(state, verified.attempt, verified.evidence), stage: CommitBoundary}))}
      | _ => state
    } else state
}
