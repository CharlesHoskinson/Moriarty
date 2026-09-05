module execution {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import authorization.* from "./authorization"

  type AttemptId = FundingOneAttempt | FundingTwoAttempt | DispositionAttempt
    | FirstFillAttempt | SecondFillAttempt | CancelAttempt | FreshCancelAttempt | RecoveryAttempt
  type PreparedAttempt[s,c,a,p] = {id: AttemptId, actor: Principal, operation: Operation,
    observation: CandidateObservation[s,c,a,p], context: AuthorityContext[s,c,a,p]}
  // Evidence disposition abstracts the external verifier result, not its
  // implementation. Full bindings are checked here; adapters must establish
  // actual effect derivation and cryptographic verification later.
  type EffectProof[s,c,a,p] = {attempt: PreparedAttempt[s,c,a,p], disposition: EvidenceDisposition}
  type SignatureProof[s,c,a,p] = {attempt: PreparedAttempt[s,c,a,p],
    signed: SignedPolicy[s,c,a,p], disposition: EvidenceDisposition}
  type EvidenceBundle[s,c,a,p] = {effect: EffectProof[s,c,a,p],
    signatures: AuthorityKey -> SignatureProof[s,c,a,p]}
  type VerifiedAttempt[s,c,a,p] = {attempt: PreparedAttempt[s,c,a,p], evidence: EvidenceBundle[s,c,a,p]}
  type AttemptRecord[s,c,a,p] = NoAttempt | ProposedAttempt(PreparedAttempt[s,c,a,p])
    | VerifiedOperation(VerifiedAttempt[s,c,a,p]) | ExecutedOperation(VerifiedAttempt[s,c,a,p])
  type ExecutionState[s,c,a,p] = {authority: SigningState[s,c,a,p],
    attempts: AttemptId -> AttemptRecord[s,c,a,p]}
  pure val ATTEMPT_IDS = Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt,
    FirstFillAttempt, SecondFillAttempt, CancelAttempt, FreshCancelAttempt, RecoveryAttempt)

  pure def validExecutionState(state: ExecutionState[s,c,a,p]): bool =
    validSigningState(state.authority) and state.attempts.keys() == ATTEMPT_IDS
  pure def attemptIdMatches(id: AttemptId, op: Operation): bool = match op {
    | OpFund => Set(FundingOneAttempt, FundingTwoAttempt).contains(id)
    | OpFillSlot(fill) => id == (if (fill.slot == 1) FirstFillAttempt else SecondFillAttempt)
    | OpCancelParent(_) => Set(CancelAttempt, FreshCancelAttempt).contains(id)
    | OpRecover(_) => id == RecoveryAttempt
    | _ => id == DispositionAttempt
  }
  // Stale/malicious observations may be proposed; verification must reject them.
  pure def canPropose(state: ExecutionState[s,c,a,p], id: AttemptId, op: Operation,
                      obs: CandidateObservation[s,c,a,p], actor: Principal): bool =
    if (validExecutionState(state)) validOperation(op) and attemptIdMatches(id, op)
      and state.attempts.get(id) == NoAttempt else false
  pure def applyPropose(state: ExecutionState[s,c,a,p], id: AttemptId, op: Operation,
                        obs: CandidateObservation[s,c,a,p], actor: Principal): ExecutionState[s,c,a,p] = {
    ...state, attempts: state.attempts.put(id, ProposedAttempt({id: id, actor: actor, operation: op,
      observation: obs, context: state.authority.context})),
  }
  pure def parentTransfer(parent: Parent[SignedPolicy[s,c,a,p]], recipient: Principal, quantity: int): Transfer = {
    source: parent.source, destination: Wallet(recipient), asset: parent.asset, quantity: quantity,
  }
  pure def operationFinancialGuard(context: AuthorityContext[s,c,a,p], op: Operation, actual: List[Transfer]): bool = match op {
    | OpFillSlot(fill) => match context.parents.get(fill.parent) {
        | ParentVacant => false
        | ParentLive(live) => canConsumeSlot(live.parent, live.entry, fill.slot, live.entry)
            and actual == List(parentTransfer(live.parent, live.parent.recipient, slotQuantity(live.parent, fill.slot)))
      }
    | OpCancelParent(parent) => match context.parents.get(parent) {
        | ParentVacant => false
        | ParentLive(live) => canCancelParent(live.parent, live.entry, live.entry) and actual == List()
      }
    | OpRecover(recovery) => match context.parents.get(recovery.parent) {
        | ParentVacant => false
        | ParentLive(live) => live.entry.cancelled and live.entry.remainingAllowance > 0
            and balance(context.ledger, live.parent.source, live.parent.asset) == live.entry.remainingAllowance
            and actual == List(parentTransfer(live.parent, live.parent.key.principal, live.entry.remainingAllowance))
      }
    | _ => actual.length() > 0
  }
  pure def authorityAllows(context: AuthorityContext[s,c,a,p], key: AuthorityKey,
                            attempt: PreparedAttempt[s,c,a,p], proof: SignatureProof[s,c,a,p]): bool = {
    val policyAllowsActual = policyAllowsObservation(proof.signed.policy, attempt.operation,
      attempt.observation, contextPolicyFacts(context))
    proof.disposition == EvidenceValid and proof.attempt == attempt
      and proof.signed.policy.body.key == key and policyAllowsActual
      and match context.registry.get(key) {
        | AuthorityUnused => false
        | AuthorityRegistered(signed) => signed == proof.signed
        | AuthorityConsumed(consumed) => consumed.signed == proof.signed
            and match attempt.operation {
              | OpFillSlot(fill) => fill.parent == key
              | OpCancelParent(parent) => parent == key
              | _ => false
            }
      }
  }
  pure def projectionAdmitsExecution(projection: CoreProjection[c]): bool = match projection {
    | NoCoreProjection => true
    | CoreProjected(result) => result.accepted and result.error == NoCoreError
  }
  pure def verificationValid(context: AuthorityContext[s,c,a,p], attempt: PreparedAttempt[s,c,a,p],
                             evidence: EvidenceBundle[s,c,a,p]): bool =
    if (validAuthorityContext(context) and validOperation(attempt.operation)) {
      val needed = requiredKeys(attempt.operation, attempt.observation.effects)
      val completeProofs = evidence.signatures.keys() == needed and needed.subseteq(AUTHORITY_KEYS)
      if (completeProofs) needed.size() > 0 and attemptIdMatches(attempt.id, attempt.operation)
        and attempt.context == context and attempt.observation.predecessor == context.candidate
        and attempt.observation.transactionTime == context.environment.physicalTime
        and attempt.observation.effectEvidence == EvidenceValid
        and projectionAdmitsExecution(attempt.observation.coreProjection)
        and evidence.effect.disposition == EvidenceValid and evidence.effect.attempt == attempt
        and canApply(context.ledger, attempt.observation.effects)
        and operationFinancialGuard(context, attempt.operation, attempt.observation.effects)
        and needed.forall(key => authorityAllows(context, key, attempt, evidence.signatures.get(key)))
      else false
    } else false
  pure def canVerify(state: ExecutionState[s,c,a,p], id: AttemptId, evidence: EvidenceBundle[s,c,a,p]): bool =
    if (validExecutionState(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id and verificationValid(state.authority.context, attempt, evidence)
      | _ => false
    } else false
  pure def applyVerify(state: ExecutionState[s,c,a,p], id: AttemptId, evidence: EvidenceBundle[s,c,a,p]): ExecutionState[s,c,a,p] = {
    ...state, attempts: state.attempts.put(id, VerifiedOperation({attempt: evidence.effect.attempt, evidence: evidence})),
  }
  pure def canCommit(state: ExecutionState[s,c,a,p], id: AttemptId): bool = false
  pure def applyCommit(state: ExecutionState[s,c,a,p], id: AttemptId): ExecutionState[s,c,a,p] = state
}
