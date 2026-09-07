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
  type RejectionStage = VerificationBoundary | CommitBoundary
  type RejectedAttempt[s,c,a,p] = {attempt: PreparedAttempt[s,c,a,p], evidence: EvidenceBundle[s,c,a,p],
    observedContext: AuthorityContext[s,c,a,p], reason: RejectionReason, stage: RejectionStage}
  type AttemptRecord[s,c,a,p] = NoAttempt | ProposedAttempt(PreparedAttempt[s,c,a,p])
    | VerifiedOperation(VerifiedAttempt[s,c,a,p]) | ExecutedOperation(VerifiedAttempt[s,c,a,p])
    | RejectedOperation(RejectedAttempt[s,c,a,p])
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
  pure def projectionAdmitsExecution(op: Operation, projection: CoreProjection[c]): bool = match op {
    // Lifecycle cancellation is not a Core transaction and cannot invent one.
    | OpCancelParent(_) => projection == NoCoreProjection
    | _ => match projection {
        | NoCoreProjection => true
        | CoreProjected(result) => result.accepted and result.error == NoCoreError
      }
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
        and projectionAdmitsExecution(attempt.operation, attempt.observation.coreProjection)
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
  pure def canCommit(state: ExecutionState[s,c,a,p], id: AttemptId): bool =
    if (validExecutionState(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id
          and verificationValid(state.authority.context, verified.attempt, verified.evidence)
      | _ => false
    } else false
  // This classifier is called only for a failed boundary. It never changes
  // the candidate's observation or fabricates a Core error for an envelope
  // rejection. CoreRejected retains a reported, known error under the same
  // external-verifier abstraction; it is not an independent Core derivation.
  pure def consumptionConflict(context: AuthorityContext[s,c,a,p], op: Operation): bool = match op {
    | OpFillSlot(fill) => match context.parents.get(fill.parent) {
        | ParentVacant => true
        | ParentLive(live) => not(canConsumeSlot(live.parent, live.entry, fill.slot, live.entry))
      }
    | OpCancelParent(parent) => match context.parents.get(parent) {
        | ParentVacant => true
        | ParentLive(live) => not(canCancelParent(live.parent, live.entry, live.entry))
      }
    | OpRecover(recovery) => match context.parents.get(recovery.parent) {
        | ParentVacant => true
        | ParentLive(live) => not(live.entry.cancelled) or live.entry.remainingAllowance <= 0
      }
    | _ => false
  }
  pure def rejectionReason(context: AuthorityContext[s,c,a,p], attempt: PreparedAttempt[s,c,a,p],
                            evidence: EvidenceBundle[s,c,a,p]): RejectionReason = {
    val obs = attempt.observation
    val supplied = evidence.signatures.keys()
    val stale = attempt.context != context or obs.predecessor != context.candidate
      or obs.transactionTime != context.environment.physicalTime or evidence.effect.attempt != attempt
      or supplied.exists(key => evidence.signatures.get(key).attempt != attempt)
    if (stale) StaleBindings
    else if (not(validAuthorityContext(context)) or not(validOperation(attempt.operation))) UnauthorizedEffect
    else {
      val needed = requiredKeys(attempt.operation, obs.effects)
      val missing = not(needed.subseteq(supplied)) or obs.effectEvidence == EvidenceUnavailable
        or evidence.effect.disposition == EvidenceUnavailable
        or supplied.exists(key => evidence.signatures.get(key).disposition == EvidenceUnavailable)
      val invalid = supplied != needed or obs.effectEvidence != EvidenceValid
        or evidence.effect.disposition != EvidenceValid
        or supplied.exists(key => evidence.signatures.get(key).disposition != EvidenceValid)
      if (missing) EvidenceMissing
      else if (invalid) UnauthorizedEffect
      else match obs.coreProjection {
        | CoreProjected(result) =>
            if (not(result.accepted) and result.error != NoCoreError and validFrozenCoreError(result.error)
              and not(isCancellation(attempt.operation))) CoreRejected(result.error)
            else if (projectionAdmitsExecution(attempt.operation, obs.coreProjection)
              and consumptionConflict(context, attempt.operation)) ConsumptionConflict
            else UnauthorizedEffect
        | NoCoreProjection => if (consumptionConflict(context, attempt.operation)) ConsumptionConflict
            else UnauthorizedEffect
      }
    }
  }
  pure def isCancellation(op: Operation): bool = match op {
    | OpCancelParent(_) => true
    | _ => false
  }
  pure def canRejectProposed(state: ExecutionState[s,c,a,p], id: AttemptId,
                             evidence: EvidenceBundle[s,c,a,p]): bool =
    if (validExecutionState(state)) match state.attempts.get(id) {
      | ProposedAttempt(attempt) => attempt.id == id
          and not(verificationValid(state.authority.context, attempt, evidence))
      | _ => false
    } else false
  pure def applyRejectProposed(state: ExecutionState[s,c,a,p], id: AttemptId,
                               evidence: EvidenceBundle[s,c,a,p]): ExecutionState[s,c,a,p] = match state.attempts.get(id) {
    | ProposedAttempt(attempt) => {...state, attempts: state.attempts.put(id, RejectedOperation({
        attempt: attempt, evidence: evidence, observedContext: state.authority.context,
        reason: rejectionReason(state.authority.context, attempt, evidence), stage: VerificationBoundary}))}
    | _ => state
  }
  pure def canRejectVerified(state: ExecutionState[s,c,a,p], id: AttemptId): bool =
    if (validExecutionState(state)) match state.attempts.get(id) {
      | VerifiedOperation(verified) => verified.attempt.id == id
          and not(verificationValid(state.authority.context, verified.attempt, verified.evidence))
      | _ => false
    } else false
  pure def applyRejectVerified(state: ExecutionState[s,c,a,p], id: AttemptId): ExecutionState[s,c,a,p] = match state.attempts.get(id) {
    | VerifiedOperation(verified) => {...state, attempts: state.attempts.put(id, RejectedOperation({
        attempt: verified.attempt, evidence: verified.evidence, observedContext: state.authority.context,
        reason: rejectionReason(state.authority.context, verified.attempt, verified.evidence), stage: CommitBoundary}))}
    | _ => state
  }
  // Guards establish live exact parent entries before these updates are used.
  pure def committedParents(context: AuthorityContext[s,c,a,p], op: Operation): AuthorityKey -> ParentCell[s,c,a,p] = match op {
    | OpFillSlot(fill) => match context.parents.get(fill.parent) {
        | ParentVacant => context.parents
        | ParentLive(live) => context.parents.put(fill.parent, ParentLive({...live,
            entry: applyConsumeSlot(live.parent, live.entry, fill.slot)}))
      }
    | OpCancelParent(parent) => match context.parents.get(parent) {
        | ParentVacant => context.parents
        | ParentLive(live) => context.parents.put(parent, ParentLive({...live,
            entry: applyCancelParent(live.parent, live.entry)}))
      }
    | _ => context.parents
  }
  pure def committedRevision(context: AuthorityContext[s,c,a,p], key: AuthorityKey, op: Operation): int =
    if (operationParents(op).contains(key) and isParentKey(key)) match context.parents.get(key) {
      | ParentVacant => 0
      | ParentLive(live) => live.entry.revision + 1
    } else 1
  pure def committedContext(verified: VerifiedAttempt[s,c,a,p]): AuthorityContext[s,c,a,p] = {
    val attempt = verified.attempt
    val before = attempt.context
    val needed = requiredKeys(attempt.operation, attempt.observation.effects)
    val registry = needed.fold(before.registry, (acc, key) => acc.put(key,
      AuthorityConsumed({signed: verified.evidence.signatures.get(key).signed,
        revision: committedRevision(before, key, attempt.operation)})))
    {...before, candidate: attempt.observation.proposedSuccessor,
      ledger: applyTransfers(before.ledger, attempt.observation.effects), registry: registry,
      parents: committedParents(before, attempt.operation)}
  }
  // The guarded action performs this single state assignment: money, parent
  // accounting, authority consumption, and attempt status change atomically.
  pure def applyCommit(state: ExecutionState[s,c,a,p], id: AttemptId): ExecutionState[s,c,a,p] = match state.attempts.get(id) {
    | VerifiedOperation(verified) => {...state, authority: {...state.authority, context: committedContext(verified)},
        attempts: state.attempts.put(id, ExecutedOperation(verified))}
    | _ => state
  }
}
