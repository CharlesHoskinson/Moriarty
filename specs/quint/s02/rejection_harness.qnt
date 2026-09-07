module rejection_harness {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import execution_harness.* from "./execution_harness"

  // Constructed signed swap base isolates rejection. Full signing-to-settlement
  // traces live in execution_harness. No candidate semantics or verifier here.
  action rejectionInit = initSignedAfter
  val missingEvidence = {...settlementEvidence(executionState, aliceAfter, bobAfter), signatures: Map()}
  action rejectMissing = all {
    canRejectProposed(executionState, DispositionAttempt, missingEvidence),
    executionState' = applyRejectProposed(executionState, DispositionAttempt, missingEvidence),
    selectedProfile' = selectedProfile,
  }
  val canAdvanceAnchor = settlementVerified and executionState.authority.context.environment.anchor == 0
  action advanceAnchor = all {
    canAdvanceAnchor,
    executionState' = {...executionState, authority: {...executionState.authority,
      context: {...executionState.authority.context,
        environment: {...executionState.authority.context.environment, anchor: 1}}}},
    selectedProfile' = selectedProfile,
  }
  action rejectStale = all {
    canRejectVerified(executionState, DispositionAttempt),
    executionState' = applyRejectVerified(executionState, DispositionAttempt),
    selectedProfile' = selectedProfile,
  }
  action rejectionStep = any {proposeSettlement, verifySettlement, rejectMissing, advanceAnchor, rejectStale}
  val missingRejected = match executionState.attempts.get(DispositionAttempt) {
    | RejectedOperation(rejected) => rejected.reason == EvidenceMissing and rejected.stage == VerificationBoundary
    | _ => false
  }
  val anchorAdvanced = executionState.authority.context.environment.anchor == 1
  val staleRejected = match executionState.attempts.get(DispositionAttempt) {
    | RejectedOperation(rejected) => rejected.reason == StaleBindings and rejected.stage == CommitBoundary
    | _ => false
  }
  val rejectionTerminal = missingRejected or staleRejected
  val rejectionEnabled = (signaturesReady and canPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob))
    or canVerify(executionState, DispositionAttempt, settlementEvidence(executionState, aliceAfter, bobAfter))
    or canRejectProposed(executionState, DispositionAttempt, missingEvidence)
    or canAdvanceAnchor or canRejectVerified(executionState, DispositionAttempt)
  val rejectionSafety = validExecutionState(executionState)
    and executionState.authority.context.ledger == signedAfter.authority.context.ledger
    and executionState.authority.context.registry == signedAfter.authority.context.registry
    and executionState.authority.context.parents == signedAfter.authority.context.parents
    and executionState.authority.context.candidate == "funded"
    and (rejectionTerminal iff not(rejectionEnabled))
    and match executionState.attempts.get(DispositionAttempt) {
      | RejectedOperation(rejected) => rejected.attempt == settlementAttempt(signedAfter)
          and rejected.observedContext == executionState.authority.context
          and rejected.evidence.effect.attempt == rejected.attempt
      | _ => true
    }
}
