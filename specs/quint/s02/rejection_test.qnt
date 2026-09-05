module rejection_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import execution_harness.* from "./execution_harness"

  pure val attempt = settlementAttempt(signedAfter)
  pure val evidence = settlementEvidence(signedAfter, aliceAfter, bobAfter)
  pure val proposed = applyPropose(signedAfter, DispositionAttempt, OpSettle, settleObservation, Bob)
  pure val verified = applyVerify(proposed, DispositionAttempt, evidence)
  pure val missing = {...evidence, signatures: Map()}
  pure val staleContext = {...attempt.context, environment: {...environment, anchor: 1}}
  pure val staleVerified = {...verified, authority: {...verified.authority, context: staleContext}}
  pure val rejectedProposal = applyRejectProposed(proposed, DispositionAttempt, missing)
  pure val rejectedCommit = applyRejectVerified(staleVerified, DispositionAttempt)
  run missingProofRejectionEnabledTest = assert(canRejectProposed(proposed, DispositionAttempt, missing))
  run validProposalCannotBeRejectedTest = assert(not(canRejectProposed(proposed, DispositionAttempt, evidence)))
  run validVerificationCannotBeRejectedTest = assert(not(canRejectVerified(verified, DispositionAttempt)))
  run staleCommitRejectionEnabledTest = assert(canRejectVerified(staleVerified, DispositionAttempt))
  run missingProofRecordTest = assert(rejectedProposal.attempts.get(DispositionAttempt) == RejectedOperation({
    attempt: attempt, evidence: missing, observedContext: attempt.context,
    reason: EvidenceMissing, stage: VerificationBoundary}))
  run staleCommitRecordTest = assert(rejectedCommit.attempts.get(DispositionAttempt) == RejectedOperation({
    attempt: attempt, evidence: evidence, observedContext: staleContext,
    reason: StaleBindings, stage: CommitBoundary}))
  run rejectionPreservesAuthorityTest = assert(rejectedProposal.authority == proposed.authority
    and rejectedCommit.authority == staleVerified.authority)
  run rejectionPreservesOtherAttemptsTest = assert(ATTEMPT_IDS.exclude(Set(DispositionAttempt)).forall(id =>
    rejectedProposal.attempts.get(id) == proposed.attempts.get(id)
      and rejectedCommit.attempts.get(id) == staleVerified.attempts.get(id)))
  run cannotReuseRejectedAttemptTest = assert(not(canPropose(rejectedProposal, DispositionAttempt, OpSettle, settleObservation, Bob))
    and not(canVerify(rejectedProposal, DispositionAttempt, evidence))
    and not(canCommit(rejectedProposal, DispositionAttempt))
    and not(canRejectProposed(rejectedProposal, DispositionAttempt, missing))
    and not(canRejectVerified(rejectedCommit, DispositionAttempt)))
  run cannotRejectAbsentOrExecutedTest = {
    val executed = applyCommit(verified, DispositionAttempt)
    assert(not(canRejectProposed(signedAfter, DispositionAttempt, missing))
      and not(canRejectVerified(signedAfter, DispositionAttempt))
      and not(canRejectProposed(executed, DispositionAttempt, missing))
      and not(canRejectVerified(executed, DispositionAttempt)))
  }
  run rejectionStageMustMatchTest = assert(not(canRejectVerified(proposed, DispositionAttempt))
    and not(canRejectProposed(verified, DispositionAttempt, missing)))
  run missingClassificationTest = assert(rejectionReason(attempt.context, attempt, missing) == EvidenceMissing)
  run stalePrecedesMissingTest = assert(rejectionReason(staleContext, attempt, missing) == StaleBindings)
  run substitutionClassificationTest = assert(rejectionReason(attempt.context, attempt,
    {...evidence, effect: {...evidence.effect, attempt: {...attempt, actor: Alice}}}) == StaleBindings)
}
