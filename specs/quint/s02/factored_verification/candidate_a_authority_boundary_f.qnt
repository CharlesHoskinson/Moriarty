module candidate_a_authority_boundary_f {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter_f.* from "./candidate_a_authority_adapter_f"
  type AAuthorityContext = AuthorityContext[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthoritySigning = SigningState[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityAttempt = PreparedAttempt[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityVerified = VerifiedAttempt[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityPlanned = PlannedOperation[APredecessor,AContinuation,ACall]

  // Validate complete finite maps before account/escrow lookups.
  pure def authorityPredecessorLedgerMatchesA(before: APredecessor, ledger: Ledger): bool = {
    val shared_before = before
    val shared_ledger = ledger
    a5_body_authorityPredecessorLedgerMatchesA(shared_before, shared_ledger)
  }
  pure def a5_body_authorityPredecessorLedgerMatchesA(before: APredecessor, ledger: Ledger): bool =
    if (validState(before.program, before.state) and validLedger(ledger))
      CORE_ACCOUNTS.forall(account =>
        before.state.accounts.get(account) == ledger.get((Escrow(account), account.asset)))
    else false
  pure def authorityPlannedOperationMatchesA(planned: AAuthorityPlanned, plan: AResolvedPlan): bool = {
    val shared_planned = planned
    val shared_plan = plan
    a5_body_authorityPlannedOperationMatchesA(shared_planned, shared_plan)
  }
  pure def a5_body_authorityPlannedOperationMatchesA(planned: AAuthorityPlanned, plan: AResolvedPlan): bool =
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
  pure def authorityPlanMatchesA(plan: AResolvedPlan): bool = {
    val shared_plan = plan
    a5_body_authorityPlanMatchesA(shared_plan)
  }
  pure def a5_body_authorityPlanMatchesA(plan: AResolvedPlan): bool =
    plan.operations.length() >= 1 and plan.operations.length() <= 4
      and plan.operations.foldl(true, (ok, planned) => ok and authorityPlannedOperationMatchesA(planned, plan))
  pure def validAuthorityPolicyA(policy: AAuthorityPolicy): bool = {
    val shared_policy = policy
    a5_body_validAuthorityPolicyA(shared_policy)
  }
  pure def a5_body_validAuthorityPolicyA(policy: AAuthorityPolicy): bool =
    validPolicy(policy) and match policy.binding {
      | BeforeResolution(_) => true
      | AfterResolution(view) => view.operations == view.identity.operations and authorityPlanMatchesA(view.identity)
    }
  pure def validAuthoritySigningA(state: AAuthoritySigning): bool = {
    val shared_state = state
    a5_body_validAuthoritySigningA(shared_state)
  }
  pure def a5_body_validAuthoritySigningA(state: AAuthoritySigning): bool =
    validSigningState(state) and authorityPredecessorLedgerMatchesA(state.context.candidate, state.context.ledger)
  pure def canPrepareSigningAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal): bool = {
    val shared_state = state
    val shared_policy = policy
    val shared_signer = signer
    a5_body_canPrepareSigningAuthorityA(shared_state, shared_policy, shared_signer)
  }
  pure def a5_body_canPrepareSigningAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal): bool =
    validAuthoritySigningA(state) and validAuthorityPolicyA(policy) and canPrepareSigning(state, policy, signer)
  pure def canSignAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal, token: int): bool = {
    val shared_state = state
    val shared_policy = policy
    val shared_signer = signer
    val shared_token = token
    a5_body_canSignAuthorityA(shared_state, shared_policy, shared_signer, shared_token)
  }
  pure def a5_body_canSignAuthorityA(state: AAuthoritySigning, policy: AAuthorityPolicy, signer: Principal, token: int): bool =
    validAuthoritySigningA(state) and validAuthorityPolicyA(policy) and canSign(state, policy, signer, token)

  // Retained stale attempts remain representable. Freshness belongs to the gate.
  pure def validAuthorityExecutionA(state: AAuthorityExecution): bool = {
    val shared_state = state
    a5_body_validAuthorityExecutionA(shared_state)
  }
  pure def a5_body_validAuthorityExecutionA(state: AAuthorityExecution): bool =
    validExecutionState(state) and authorityPredecessorLedgerMatchesA(
      state.authority.context.candidate, state.authority.context.ledger)
  pure def authorityAttemptMatchesA(attempt: AAuthorityAttempt): bool = {
    val shared_attempt = attempt
    a5_body_authorityAttemptMatchesA(shared_attempt)
  }
  pure def a5_body_authorityAttemptMatchesA(attempt: AAuthorityAttempt): bool =
    authorityObservationMatchesA(attempt.operation, attempt.observation)
      and authorityPlanMatchesA(attempt.observation.resolvedPlan)
  pure def authorityEvidencePoliciesMatchA(evidence: AAuthorityEvidence): bool = {
    val shared_evidence = evidence
    a5_body_authorityEvidencePoliciesMatchA(shared_evidence)
  }
  pure def a5_body_authorityEvidencePoliciesMatchA(evidence: AAuthorityEvidence): bool =
    evidence.signatures.keys().forall(key => validAuthorityPolicyA(evidence.signatures.get(key).signed.policy))
  pure def canVerifyAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool = {
    val shared_state = state
    val shared_id = id
    val shared_evidence = evidence
    a5_body_canVerifyAuthorityA(shared_state, shared_id, shared_evidence)
  }
  pure def a5_body_canVerifyAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool =
    if (validAuthorityExecutionA(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id and authorityAttemptMatchesA(attempt)
          and authorityEvidencePoliciesMatchA(evidence) and canVerify(state, id, evidence)
      | _ => false
    } else false
  pure def canCommitAuthorityA(state: AAuthorityExecution, id: AttemptId): bool = {
    val shared_state = state
    val shared_id = id
    a5_body_canCommitAuthorityA(shared_state, shared_id)
  }
  pure def a5_body_canCommitAuthorityA(state: AAuthorityExecution, id: AttemptId): bool =
    if (validAuthorityExecutionA(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id and authorityAttemptMatchesA(verified.attempt)
          and authorityEvidencePoliciesMatchA(verified.evidence) and canCommit(state, id)
      | _ => false
    } else false
  // Structural envelope only: the semantic predicate that failed cannot strand
  // an otherwise representable proposal/verified record.
  pure def authorityRejectionDomainA(state: AAuthorityExecution): bool = {
    val shared_state = state
    a5_body_authorityRejectionDomainA(shared_state)
  }
  pure def a5_body_authorityRejectionDomainA(state: AAuthorityExecution): bool =
    state.attempts.keys() == ATTEMPT_IDS and state.authority.signing.keys() == AUTHORITY_KEYS
      and state.authority.context.registry.keys() == AUTHORITY_KEYS
      and state.authority.context.parents.keys() == AUTHORITY_KEYS
  pure def authorityRejectionReasonA(state: AAuthorityExecution, attempt: AAuthorityAttempt,
    evidence: AAuthorityEvidence): RejectionReason = {
    val shared_state = state
    val shared_attempt = attempt
    val shared_evidence = evidence
    a5_body_authorityRejectionReasonA(shared_state, shared_attempt, shared_evidence)
  }
  pure def a5_body_authorityRejectionReasonA(state: AAuthorityExecution, attempt: AAuthorityAttempt,
    evidence: AAuthorityEvidence): RejectionReason =
    if (not(authorityPredecessorLedgerMatchesA(state.authority.context.candidate, state.authority.context.ledger))
        or not(authorityAttemptMatchesA(attempt)) or not(authorityEvidencePoliciesMatchA(evidence)))
      UnauthorizedEffect
    else rejectionReason(state.authority.context, attempt, evidence)
  pure def canRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool = {
    val shared_state = state
    val shared_id = id
    val shared_evidence = evidence
    a5_body_canRejectProposedAuthorityA(shared_state, shared_id, shared_evidence)
  }
  pure def a5_body_canRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId, evidence: AAuthorityEvidence): bool =
    if (authorityRejectionDomainA(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id and not(canVerifyAuthorityA(state, id, evidence))
      | _ => false
    } else false
  pure def canRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): bool = {
    val shared_state = state
    val shared_id = id
    a5_body_canRejectVerifiedAuthorityA(shared_state, shared_id)
  }
  pure def a5_body_canRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): bool =
    if (authorityRejectionDomainA(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id and not(canCommitAuthorityA(state, id))
      | _ => false
    } else false
  pure def applyRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId,
    evidence: AAuthorityEvidence): AAuthorityExecution = {
    val shared_state = state
    val shared_id = id
    val shared_evidence = evidence
    a5_body_applyRejectProposedAuthorityA(shared_state, shared_id, shared_evidence)
  }
  pure def a5_body_applyRejectProposedAuthorityA(state: AAuthorityExecution, id: AttemptId,
    evidence: AAuthorityEvidence): AAuthorityExecution =
    if (canRejectProposedAuthorityA(state, id, evidence)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => {...state, attempts: state.attempts.put(id, RejectedOperation({
          attempt: attempt, evidence: evidence, observedContext: state.authority.context,
          reason: authorityRejectionReasonA(state, attempt, evidence), stage: VerificationBoundary}))}
      | _ => state
    } else state
  pure def applyRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): AAuthorityExecution = {
    val shared_state = state
    val shared_id = id
    a5_body_applyRejectVerifiedAuthorityA(shared_state, shared_id)
  }
  pure def a5_body_applyRejectVerifiedAuthorityA(state: AAuthorityExecution, id: AttemptId): AAuthorityExecution =
    if (canRejectVerifiedAuthorityA(state, id)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => {...state, attempts: state.attempts.put(id, RejectedOperation({
          attempt: verified.attempt, evidence: verified.evidence, observedContext: state.authority.context,
          reason: authorityRejectionReasonA(state, verified.attempt, verified.evidence), stage: CommitBoundary}))}
      | _ => state
    } else state
}
