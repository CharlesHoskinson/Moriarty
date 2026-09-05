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
  pure def canVerify(state: ExecutionState[s,c,a,p], id: AttemptId, evidence: EvidenceBundle[s,c,a,p]): bool = false
  pure def applyVerify(state: ExecutionState[s,c,a,p], id: AttemptId, evidence: EvidenceBundle[s,c,a,p]): ExecutionState[s,c,a,p] = state
  pure def canCommit(state: ExecutionState[s,c,a,p], id: AttemptId): bool = false
  pure def applyCommit(state: ExecutionState[s,c,a,p], id: AttemptId): ExecutionState[s,c,a,p] = state
}
