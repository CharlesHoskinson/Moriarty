module execution_adversarial_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import execution_harness.* from "./execution_harness"

  // Test authorship, not an independent review or a candidate interpreter.
  // EvidenceValid is an abstract external verifier result, not generated proof.
  pure val proposed = applyPropose(signedAfter, DispositionAttempt, OpSettle, settleObservation, Bob)
  pure val attempt = settlementAttempt(signedAfter)
  pure val evidence = settlementEvidence(signedAfter, aliceAfter, bobAfter)
  pure val verified = applyVerify(proposed, DispositionAttempt, evidence)
  pure val proposedBefore = applyPropose(signedBefore, DispositionAttempt, OpSettle, settleObservation, Bob)
  pure val beforeEvidence = settlementEvidence(signedBefore, aliceBefore, bobBefore)

  pure def bindEvidence(bundle: EvidenceBundle[str,str,str,str], other: PreparedAttempt[str,str,str,str]): EvidenceBundle[str,str,str,str] = {
    effect: {...bundle.effect, attempt: other},
    signatures: bundle.signatures.keys().mapBy(key => {...bundle.signatures.get(key), attempt: other}),
  }
  pure def withContext(state: FixtureExecution, context: AuthorityContext[str,str,str,str]): FixtureExecution =
    {...state, authority: {...state.authority, context: context}}
  pure def withAttempt(state: FixtureExecution, other: PreparedAttempt[str,str,str,str]): FixtureExecution =
    {...state, attempts: state.attempts.put(other.id, ProposedAttempt(other))}

  run positiveEvidenceFixturesTest = assert(canVerify(proposed, DispositionAttempt, evidence)
    and canVerify(proposedBefore, DispositionAttempt, beforeEvidence))
  run positiveCommitFixtureTest = assert(canCommit(verified, DispositionAttempt))
  run unavailableEffectProofTest = assert(not(canVerify(proposed, DispositionAttempt,
    {...evidence, effect: {...evidence.effect, disposition: EvidenceUnavailable}})))
  run invalidEffectProofTest = assert(not(canVerify(proposed, DispositionAttempt,
    {...evidence, effect: {...evidence.effect, disposition: EvidenceInvalid}})))
  run omittedDebitOwnerTest = assert(Set(aliceKey, bobKey).forall(key =>
    not(canVerify(proposed, DispositionAttempt,
      {...evidence, signatures: Map(key -> evidence.signatures.get(key))}))))
  run noSignatureProofsTest = assert(not(canVerify(proposed, DispositionAttempt,
    {...evidence, signatures: Map()})))
  run unavailableSignatureProofTest = assert(Set(aliceKey, bobKey).forall(key =>
    not(canVerify(proposed, DispositionAttempt, {...evidence,
      signatures: evidence.signatures.put(key, {...evidence.signatures.get(key), disposition: EvidenceUnavailable})}))))
  run invalidSignatureProofTest = assert(Set(aliceKey, bobKey).forall(key =>
    not(canVerify(proposed, DispositionAttempt, {...evidence,
      signatures: evidence.signatures.put(key, {...evidence.signatures.get(key), disposition: EvidenceInvalid})}))))
  run swappedSignatureTokenTest = assert(Set(aliceKey, bobKey).forall(key => {
    val proof = evidence.signatures.get(key)
    not(canVerify(proposed, DispositionAttempt, {...evidence,
      signatures: evidence.signatures.put(key, {...proof, signed: {...proof.signed, token: 1}})}))
  }))
  run swappedSignedPolicyTest = assert(not(canVerify(proposed, DispositionAttempt, {...evidence,
    signatures: evidence.signatures.put(aliceKey, {...evidence.signatures.get(aliceKey),
      signed: {...evidence.signatures.get(aliceKey).signed, policy: aliceBefore}})})))
  run swappedSignerTest = assert(not(canVerify(proposed, DispositionAttempt, {...evidence,
    signatures: evidence.signatures.put(aliceKey, {...evidence.signatures.get(aliceKey),
      signed: {...evidence.signatures.get(aliceKey).signed, signer: Bob}})})))

  // Every mutation is applied separately to the effect proof and each required
  // signature proof. The stored proposal remains fixed at the positive fixture.
  pure def rejectsEvidenceAttempt(other: PreparedAttempt[str,str,str,str]): bool =
    not(canVerify(proposed, DispositionAttempt, {...evidence, effect: {...evidence.effect, attempt: other}}))
      and Set(aliceKey, bobKey).forall(key => not(canVerify(proposed, DispositionAttempt,
        {...evidence, signatures: evidence.signatures.put(key, {...evidence.signatures.get(key), attempt: other})})))
  run actorSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt, actor: Alice}))
  run attemptIdSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt, id: CancelAttempt}))
  run operationSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt, operation: OpVoluntaryRefund}))
  run inputSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, input: refundInput}}))
  run planSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, resolvedPlan: "other-plan"}}))
  run artifactSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, artifactAndCall: "other-call"}}))
  run wholeObservationSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt, observation: cancelObservation}))
  run predecessorSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, predecessor: "other-state"}}))
  run successorSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, proposedSuccessor: "other-successor"}}))
  run effectsSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, effects: List(aliceLeg)}}))
  run displaySubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    observation: {...attempt.observation, display: AlternateDisplay}}))
  run stateContextSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    context: {...attempt.context, candidate: "other-state"}}))
  run ledgerContextSubstitutionTest = assert(rejectsEvidenceAttempt({...attempt,
    context: {...attempt.context, ledger: initialLedger(0, 0)}}))
  pure val alteredEnvironments = Set(
    {...environment, physicalTime: 1}, {...environment, physicalTime: 100},
    {...environment, anchor: 1}, {...environment, implementationVersion: 1},
    {...environment, enforcementMechanism: 1})
  run environmentContextSubstitutionTest = assert(alteredEnvironments.forall(env =>
    rejectsEvidenceAttempt({...attempt, context: {...attempt.context, environment: env}})))

  run observationEffectUnavailableTest = {
    val other = {...attempt, observation: {...attempt.observation, effectEvidence: EvidenceUnavailable}}
    assert(not(canVerify(withAttempt(proposed, other), DispositionAttempt, bindEvidence(evidence, other))))
  }
  run observationEffectInvalidTest = {
    val other = {...attempt, observation: {...attempt.observation, effectEvidence: EvidenceInvalid}}
    assert(not(canVerify(withAttempt(proposed, other), DispositionAttempt, bindEvidence(evidence, other))))
  }
  // Fresh evidence about substituted observations still must satisfy signed bounds.
  run freshEvidenceCannotExpandSignedAfterPlanTest = assert(Set(
    {...attempt.observation, resolvedPlan: "other-plan"},
    {...attempt.observation, artifactAndCall: "other-call"},
    {...attempt.observation, input: refundInput},
    {...attempt.observation, effects: List(aliceLeg)}).forall(obs => {
      val other = {...attempt, observation: obs}
      not(canVerify(withAttempt(proposed, other), DispositionAttempt, bindEvidence(evidence, other)))
    }))
  pure val coreState: CoreStateObservation[str] = {accounts: CORE_ACCOUNTS.mapBy(_ => 0),
    choices: Map(), continuation: "closed", minimumTime: 2}
  pure val rejectedCore: CoreResultObservation[str] = {accepted: false,
    error: CoreErrorCode("time_before_state"), warnings: List(), payments: List(),
    state: coreState, reductions: 0}
  run coreRejectedProjectionTest = {
    val base = settlementAttempt(signedBefore)
    val other = {...base, observation: {...base.observation, coreProjection: CoreProjected(rejectedCore)}}
    assert(not(canVerify(withAttempt(proposedBefore, other), DispositionAttempt,
      bindEvidence(beforeEvidence, other))))
  }
  run consumedGenericSwapAuthorityTest = assert(Set(aliceKey, bobKey).forall(key => {
    val context = {...attempt.context, registry: attempt.context.registry.put(key,
      AuthorityConsumed({signed: evidence.signatures.get(key).signed, revision: 1}))}
    val other = {...attempt, context: context}
    val state = withAttempt(withContext(proposed, context), other)
    validExecutionState(state) and not(canVerify(state, DispositionAttempt, bindEvidence(evidence, other)))
  }))
  run staleCandidateAfterVerificationTest = assert(not(canCommit(withContext(verified,
    {...attempt.context, candidate: "other-state"}), DispositionAttempt)))
  run staleLedgerAfterVerificationTest = assert(not(canCommit(withContext(verified,
    {...attempt.context, ledger: initialLedger(0, 0)}), DispositionAttempt)))
  run staleEnvironmentAfterVerificationTest = assert(alteredEnvironments.forall(env =>
    not(canCommit(withContext(verified, {...attempt.context, environment: env}), DispositionAttempt))))
  run staleRegistryAfterVerificationTest = assert(Set(aliceKey, bobKey).forall(key =>
    not(canCommit(withContext(verified, {...attempt.context,
      registry: attempt.context.registry.put(key, AuthorityUnused)}), DispositionAttempt))))

  // Constructed parent contexts: these do NOT witness executed cancellation,
  // first fill, recovery, or their resolve/check/sign/verify/commit lifecycles.
  pure val parentSigningInitial = {...signingInitial,
    context: {...signingInitial.context, candidate: "unfilled", ledger: installmentFacts.ledger}}
  pure val checkedParent = applyPrepareSigning(parentSigningInitial, parentBefore, Alice)
  pure val parentSigning = applySign(checkedParent, parentBefore, Alice, 0)
  pure val parentState: FixtureExecution = {...executionInitial, authority: parentSigning}
  pure val signedParent = {policy: parentBefore, signer: Alice, token: 0}
  pure val parent = fixtureParent(signedParent)
  pure def parentEvidence(other: PreparedAttempt[str,str,str,str]): EvidenceBundle[str,str,str,str] = {
    effect: {attempt: other, disposition: EvidenceValid},
    signatures: Map(parentKey -> {attempt: other, signed: signedParent, disposition: EvidenceValid}),
  }
  pure val cancelAttempt: PreparedAttempt[str,str,str,str] = {id: CancelAttempt, actor: Bob,
    operation: cancel, observation: cancelObservation, context: parentSigning.context}
  pure val cancelProposed = applyPropose(parentState, CancelAttempt, cancel, cancelObservation, Bob)
  run constructedParentCancellationPreconditionsTest = assert(
    canPrepareSigning(parentSigningInitial, parentBefore, Alice)
      and canSign(checkedParent, parentBefore, Alice, 0)
      and canVerify(cancelProposed, CancelAttempt, parentEvidence(cancelAttempt)))
  run unauthorizedNoEffectCancellationTest = assert(not(canVerify(cancelProposed, CancelAttempt,
    {...parentEvidence(cancelAttempt), signatures: Map()})))
  run cancellationInvalidParentSignatureTest = assert(not(canVerify(cancelProposed, CancelAttempt,
    {...parentEvidence(cancelAttempt), signatures: Map(parentKey -> {
      attempt: cancelAttempt, signed: signedParent, disposition: EvidenceInvalid})})))
  run cancellationCannotInventCoreProjectionTest = {
    val acceptedCore = {...rejectedCore, accepted: true, error: NoCoreError}
    val other = {...cancelAttempt, observation: {...cancelObservation, coreProjection: CoreProjected(acceptedCore)}}
    assert(not(canVerify(withAttempt(cancelProposed, other), CancelAttempt, parentEvidence(other))))
  }
  pure val residualEntry = applyConsumeSlot(parent, initialEntry(parent), 1)
  pure val residualContext = {...parentSigning.context, candidate: "first-filled", ledger: afterFillFacts.ledger,
    registry: parentSigning.context.registry.put(parentKey, AuthorityConsumed({signed: signedParent, revision: 1})),
    parents: parentSigning.context.parents.put(parentKey, ParentLive({parent: parent, entry: residualEntry}))}
  pure val secondObservation = {...fillObservation, predecessor: "first-filled", proposedSuccessor: "completed",
    artifactAndCall: "fill-two-call", outcome: SecondInstallment}
  pure val secondAttempt: PreparedAttempt[str,str,str,str] = {id: SecondFillAttempt, actor: Bob,
    operation: fillTwo, observation: secondObservation, context: residualContext}
  pure val secondProposed = withAttempt(withContext(parentState, residualContext), secondAttempt)
  run constructedCurrentParentResidualPreconditionsTest = assert(validExecutionState(secondProposed)
    and canVerify(secondProposed, SecondFillAttempt, parentEvidence(secondAttempt)))
  run staleParentNonceRevisionTest = {
    val context = {...residualContext, registry: residualContext.registry.put(parentKey,
      AuthorityConsumed({signed: signedParent, revision: 2}))}
    val other = {...secondAttempt, context: context}
    assert(not(canVerify(withAttempt(withContext(secondProposed, context), other), SecondFillAttempt,
      parentEvidence(other))))
  }
  run reusedFirstSlotWithCurrentResidualTest = {
    val other = {...secondAttempt, id: FirstFillAttempt, operation: fillOne}
    assert(not(canVerify(withAttempt(secondProposed, other), FirstFillAttempt, parentEvidence(other))))
  }
  run substitutedRetainedParentTokenTest = {
    val bundle = parentEvidence(secondAttempt)
    assert(not(canVerify(secondProposed, SecondFillAttempt, {...bundle,
      signatures: Map(parentKey -> {...bundle.signatures.get(parentKey), signed: {...signedParent, token: 1}})})))
  }
}
