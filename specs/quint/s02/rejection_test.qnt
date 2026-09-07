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
  run unavailableAndInvalidEvidenceDifferTest = assert(
    rejectionReason(attempt.context, attempt, {...evidence, effect: {...evidence.effect, disposition: EvidenceUnavailable}})
      == EvidenceMissing
    and rejectionReason(attempt.context, attempt, {...evidence, effect: {...evidence.effect, disposition: EvidenceInvalid}})
      == UnauthorizedEffect)
  run unavailableAndInvalidSignatureDifferTest = assert(Set(aliceKey, bobKey).forall(key => {
    val proof = evidence.signatures.get(key)
    rejectionReason(attempt.context, attempt, {...evidence,
      signatures: evidence.signatures.put(key, {...proof, disposition: EvidenceUnavailable})}) == EvidenceMissing
    and rejectionReason(attempt.context, attempt, {...evidence,
      signatures: evidence.signatures.put(key, {...proof, disposition: EvidenceInvalid})}) == UnauthorizedEffect
  }))
  pure val parentInitialSigning = {...signingInitial,
    context: {...signingInitial.context, candidate: "unfilled", ledger: installmentFacts.ledger}}
  pure val parentSigned = applySign(applyPrepareSigning(parentInitialSigning, parentBefore, Alice), parentBefore, Alice, 0)
  pure val signedParent = {policy: parentBefore, signer: Alice, token: 0}
  pure val parent = fixtureParent(signedParent)
  pure val residualContext = {...parentSigned.context, candidate: "first-filled", ledger: afterFillFacts.ledger,
    registry: parentSigned.context.registry.put(parentKey, AuthorityConsumed({signed: signedParent, revision: 1})),
    parents: parentSigned.context.parents.put(parentKey, ParentLive({parent: parent,
      entry: applyConsumeSlot(parent, initialEntry(parent), 1)}))}
  pure val acceptedCore: CoreResultObservation[str] = {accepted: true, error: NoCoreError,
    warnings: List(), payments: List(), reductions: 0,
    state: {accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: Map(), continuation: "closed", minimumTime: 2}}
  pure def repeatAttempt(projection: CoreProjection[str]): PreparedAttempt[str,str,str,str] = {
    id: FirstFillAttempt, actor: Bob, operation: fillOne, context: residualContext,
    observation: {...fillObservation, predecessor: "first-filled", coreProjection: projection}}
  pure def repeatEvidence(other: PreparedAttempt[str,str,str,str]): EvidenceBundle[str,str,str,str] = {
    effect: {attempt: other, disposition: EvidenceValid}, signatures: Map(parentKey -> {
      attempt: other, signed: signedParent, disposition: EvidenceValid})}
  // Constructed conflicting context checks diagnostic precedence only. The
  // reported Core carrier is not execution/correspondence evidence.
  run acceptedCoreDoesNotHideConsumptionConflictTest = assert(Set(NoCoreProjection, CoreProjected(acceptedCore)).forall(projection => {
    val other = repeatAttempt(projection)
    validAuthorityContext(residualContext) and not(verificationValid(residualContext, other, repeatEvidence(other)))
      and rejectionReason(residualContext, other, repeatEvidence(other)) == ConsumptionConflict
  }))
  run reportedCoreRejectionPrecedesConsumptionTest = {
    val other = repeatAttempt(CoreProjected({...acceptedCore, accepted: false, error: CoreErrorCode("input_required")}))
    assert(rejectionReason(residualContext, other, repeatEvidence(other)) == CoreRejected(CoreErrorCode("input_required")))
  }
  run unknownCoreErrorIsNotReportedAsCoreRejectionTest = {
    val other = repeatAttempt(CoreProjected({...acceptedCore, accepted: false, error: CoreErrorCode("invented-error")}))
    assert(rejectionReason(residualContext, other, repeatEvidence(other)) == UnauthorizedEffect)
  }
  run cancellationCannotBecomeCoreRejectionTest = {
    val other = {...repeatAttempt(CoreProjected({...acceptedCore, accepted: false,
      error: CoreErrorCode("input_required")})), id: CancelAttempt, operation: cancel}
    assert(not(verificationValid(residualContext, other, repeatEvidence(other)))
      and rejectionReason(residualContext, other, repeatEvidence(other)) == UnauthorizedEffect)
  }
  run signatureSubstitutionPreservesOriginalAttemptTest = {
    val substituted = {...attempt, actor: Alice}
    val wrong = {...evidence, signatures: evidence.signatures.put(aliceKey,
      {...evidence.signatures.get(aliceKey), attempt: substituted})}
    val rejected = applyRejectProposed(proposed, DispositionAttempt, wrong)
    assert(canRejectProposed(proposed, DispositionAttempt, wrong)
      and rejected.attempts.get(DispositionAttempt) == RejectedOperation({attempt: attempt,
        evidence: wrong, observedContext: attempt.context, reason: StaleBindings, stage: VerificationBoundary}))
  }
}
