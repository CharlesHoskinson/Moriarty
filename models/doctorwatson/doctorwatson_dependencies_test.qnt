// Consumer selectors, currentness and exact artifact acceptance; Root execution pending.
module doctorwatson_dependencies_test {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).* from "./doctorwatson"
  action prepare(id: int): bool = claim(id, if (id == 3) 4 else id)
    .then(freeze(id, 3, true)).then(edit(id, if (id == 3) 4 else id, Source)).then(submit(id))
  action passLocal(id: int): bool = startLocal(id).then(checker(id, jobToken(state, id), CompleteMatch))
    .then(regressions(id, jobToken(state, id), true))
  action candidate(id: int): bool = prepare(id).then(passLocal(id))
  action integrate(id: int): bool = candidate(id).then(review(id, 3, true)).then(merge(id))
  action acceptCurrent(id: int): bool = {
    val observation = currentDeliveryObservation(state, id)
    val reviewed = deliveryReview(observation, 3)
    acceptDelivery(id, observation, reviewed, deliveryAdoption(observation, reviewed))
  }
  action nativeCandidate(stage: DependencyStage): bool = configureStage(3, 1, stage, 4)
    .then(configureAdapter(1, NativeAdapter, 1)).then(integrate(2)).then(candidate(1))
  action nativeObserved(stage: DependencyStage): bool = nativeCandidate(stage).then(dispatch(1))
    .then(extObserve(1, 1, true)).then(cleanup(1))
  action chainCandidate: bool = integrate(1).then(prepare(2)).then(changeDependencies(2, 1, 2))
    .then(configureStage(2, 1, NATIVE_OBSERVED, 2)).then(removeDependency(3, 1, 4))
    .then(configureAdapter(1, NativeAdapter, 1)).then(candidate(1))
  action chainReady: bool = chainCandidate.then(dispatch(1)).then(extObserve(1, 1, true)).then(cleanup(1))
    .then(candidate(2)).then(review(2, 3, true)).then(merge(2))

  run exactSelectorGrammarTest = init.expect(and {
    dependencyStageName(SOURCE_INTEGRATED) == "source-integrated",
    dependencyStageName(NATIVE_OBSERVED) == "observed",
    dependencyStageName(NATIVE_ACCEPTED) == "accepted",
  })
  run sourceDoesNotSatisfyObservedTest = init.then(configureStage(3, 1, NATIVE_OBSERVED, 4))
    .then(integrate(1)).then(integrate(2))
    .expect(deliverySourceRecorded(state, 1) and not(deliveryObservedRecorded(state, 1)))
    .expect(not(readyDependencies(state, 3)) and not(canClaim(state, 3, 4)) and workflowInvariant)
  run nativeDoesNotSatisfySourceTest = init.then(nativeObserved(NATIVE_OBSERVED))
    .expect(deliveryObservedRecorded(state, 1) and not(deliverySourceRecorded(state, 1)))
    .expect(not(canConfigureStage(state, 3, 1, SOURCE_INTEGRATED, 4)))
    .expect(readyDependencies(state, 3) and workflowInvariant)
  run availabilityIsNeitherObservationNorAcceptanceTest = init
    .then(configureStage(3, 1, NATIVE_ACCEPTED, 4)).then(configureAdapter(1, NativeAdapter, 1))
    .then(makeAvailable(NativeLedger))
    .expect(not(deliveryObservedRecorded(state, 1)) and not(deliveryAcceptedRecorded(state, 1)))
    .expect(not(readyDependencies(state, 3)) and noEscalation)
  run observedNeedsExactAcceptanceTest = init.then(nativeObserved(NATIVE_ACCEPTED))
    .expect(not(deliveryAcceptedRecorded(state, 1)) and not(readyDependencies(state, 3)))
    .then(acceptCurrent(1)).expect(deliveryAcceptedRecorded(state, 1) and readyDependencies(state, 3))
    .expect(nativeAcceptedWitness and workflowInvariant)
  run acceptedSourceAdapterUnlocksAcceptedEdgeTest = init.then(configureStage(3, 1, NATIVE_ACCEPTED, 4))
    .then(integrate(2)).then(integrate(1))
    .expect(deliverySourceRecorded(state, 1) and not(deliveryObservedRecorded(state, 1)))
    .expect(not(deliveryAcceptedRecorded(state, 1)) and not(readyDependencies(state, 3)))
    .then(acceptCurrent(1)).expect(sourceAcceptedWitness and deliveryAcceptedRecorded(state, 1))
    .expect(readyDependencies(state, 3) and workflowInvariant)
  run cleanupDebtBlocksTypedRowsTest = init.then(nativeCandidate(NATIVE_ACCEPTED)).then(dispatch(1))
    .then(extObserve(1, 1, true))
    .expect(not(deliveryObservedRecorded(state, 1)) and not(sCanAcceptDelivery(state, 1)))
    .then(cleanup(1)).then(acceptCurrent(1)).expect(deliveryAcceptedRecorded(state, 1) and workflowInvariant)
  run cancellationKeepsHistoryButBlocksRowsTest = init.then(nativeObserved(NATIVE_ACCEPTED)).then(acceptCurrent(1))
    .then(cancel(1, 1)).expect(not(deliveryObservedRecorded(state, 1)) and not(deliveryAcceptedRecorded(state, 1)))
    .expect(state.deliveryObservations.size() == 2 and state.deliveryAcceptances.size() == 1)
    .expect(not(readyDependencies(state, 3)) and workflowInvariant)
  run resolvedIncludedIsNotTypedObservationTest = init.then(nativeCandidate(NATIVE_ACCEPTED)).then(dispatch(1))
    .then(extUnknown(1, 1)).then(reconcile(1, externalResult(state, 1), terminalEvidence(externalToken(state, 1), TxIncluded)))
    .expect(not(deliveryObservedRecorded(state, 1)) and not(deliveryAcceptedRecorded(state, 1)))
    .expect(not(canDispatch(state, 1)) and not(readyDependencies(state, 3)) and workflowInvariant)
  run wrongAttemptAndFingerprintAcceptanceRefusedTest = init.then(nativeObserved(NATIVE_ACCEPTED))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val wrong = { ...observed, token: { ...observed.token, attempt: 2 } }
      val reviewed = deliveryReview(wrong, 3)
      not(canAcceptDelivery(state, 1, wrong, reviewed, deliveryAdoption(wrong, reviewed))))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val wrong = { ...observed, token: { ...observed.token, fingerprint: emptyFingerprint } }
      val reviewed = deliveryReview(wrong, 3)
      not(canAcceptDelivery(state, 1, wrong, reviewed, deliveryAdoption(wrong, reviewed))))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val wrong = { ...observed, token: { ...observed.token, operationAttempt: observed.token.operationAttempt + 1 } }
      val reviewed = deliveryReview(wrong, 3)
      not(canAcceptDelivery(state, 1, wrong, reviewed, deliveryAdoption(wrong, reviewed))))
  run wrongManifestAndAdoptionRefusedTest = init.then(nativeObserved(NATIVE_ACCEPTED))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val wrong = { ...observed, manifest: observed.manifest + 1 }
      val reviewed = deliveryReview(wrong, 3)
      not(canAcceptDelivery(state, 1, wrong, reviewed, deliveryAdoption(wrong, reviewed))))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val reviewed = deliveryReview(observed, 3)
      not(canAcceptDelivery(state, 1, observed, reviewed,
        { ...deliveryAdoption(observed, reviewed), integrator: 1 })))
  run missingOrInapplicableReviewRefusedTest = init.then(nativeObserved(NATIVE_ACCEPTED))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val reviewed = deliveryReview(observed, 1)
      not(canAcceptDelivery(state, 1, observed, reviewed, deliveryAdoption(observed, reviewed))))
    .expect(
      val observed = currentDeliveryObservation(state, 1)
      val reviewed = { ...deliveryReview(observed, 3), approved: false }
      not(canAcceptDelivery(state, 1, observed, reviewed, deliveryAdoption(observed, reviewed))))
  run nativeArtifactDriftStalesAcceptanceTest = init.then(nativeObserved(NATIVE_ACCEPTED)).then(acceptCurrent(1))
    .then(artifactDrift(1)).expect(not(deliveryObservedRecorded(state, 1)) and not(deliveryAcceptedRecorded(state, 1)))
    .expect(state.deliveryAcceptances.size() == 1 and not(readyDependencies(state, 3)) and workflowInvariant)
  run sourceArtifactDriftStalesAcceptanceTest = init.then(integrate(1)).then(acceptCurrent(1))
    .then(artifactDrift(1)).expect(not(deliverySourceRecorded(state, 1)) and not(deliveryAcceptedRecorded(state, 1)))
    .expect(state.deliveryAcceptances.size() == 1 and workflowInvariant)
  run stageChangeInvalidatesConsumedFingerprintTest = init.then(integrate(1)).then(integrate(2)).then(candidate(3))
    .expect(fingerprint(state, 3) != fingerprint(applyConfigureStage(state, 3, 1, NATIVE_ACCEPTED), 3))
    .then(configureStage(3, 1, NATIVE_ACCEPTED, 4))
    .expect(not(readyDependencies(state, 3)) and state.tasks.get(3).test == NotRun and workflowInvariant)
  run transitiveInputDriftWithoutIntermediateSourceEditTest = init.then(chainReady)
    .expect(readyDependencies(state, 3))
    .expect(
      val changed = applyInputDrift(state, 1)
      and {
        state.tasks.get(2).version == changed.tasks.get(2).version,
        fingerprint(state, 3).dependencies == fingerprint(changed, 3).dependencies,
        fingerprint(state, 3).dependencyAttempts == fingerprint(changed, 3).dependencyAttempts,
        fingerprint(state, 3).transitiveInputs != fingerprint(changed, 3).transitiveInputs,
        fingerprint(state, 3) != fingerprint(changed, 3),
      })
    .then(inputDrift(1)).expect(not(readyDependencies(state, 3)) and workflowInvariant)
  run transitiveAttemptBindingBeforeObservationTest = init.then(chainCandidate)
    .expect(canDispatch(state, 1))
    .expect(
      val launched = applyDispatch(state, 1)
      and {
        state.tasks.get(2).version == launched.tasks.get(2).version,
        fingerprint(state, 3).dependencies == fingerprint(launched, 3).dependencies,
        fingerprint(state, 3).dependencyAttempts == fingerprint(launched, 3).dependencyAttempts,
        fingerprint(state, 3).transitiveAttempts != fingerprint(launched, 3).transitiveAttempts,
      })
    .then(dispatch(1)).expect(not(readyDependencies(state, 3)) and workflowInvariant)
  run nativeObservationSurvivesUnrelatedSourceMergeTest = init.then(chainReady)
    .expect(state.tasks.get(1).phase == Working and deliveryObservedRecorded(state, 1))
    .expect(readyDependencies(state, 3) and extUsable(state, 1) and workflowInvariant)

  run sourceHistoricalDebtBlocksTypedIntegrationTest = init.then(candidate(1)).then(dispatch(1))
    .then(review(1, 3, true)).then(merge(1))
    .expect(not(deliverySourceRecorded(state, 1)) and not(sCanAcceptDelivery(state, 1)))
    .then(extUnknown(1, 1)).then(cleanup(1))
    .expect(not(deliverySourceRecorded(state, 1)))
    .then(reconcile(1, externalResult(state, 1), terminalEvidence(externalToken(state, 1), ProcessExited)))
    .expect(not(deliverySourceRecorded(state, 1)) and not(canDispatch(state, 1)))
    .then(edit(1, 1, Source)).then(rebase(1)).then(submit(1)).then(passLocal(1)).then(review(1, 3, true)).then(merge(1))
    .expect(deliverySourceRecorded(state, 1) and workflowInvariant)
  run changedSourceManifestNeedsFreshAcceptanceTest = init.then(integrate(1)).then(acceptCurrent(1))
    .then(artifactDrift(1)).then(edit(1, 1, Source)).then(rebase(1))
    .then(submit(1)).then(passLocal(1)).then(review(1, 3, true)).then(merge(1))
    .expect(deliverySourceRecorded(state, 1) and not(deliveryAcceptedRecorded(state, 1)))
    .then(acceptCurrent(1)).expect(deliveryAcceptedRecorded(state, 1) and state.deliveryAcceptances.size() == 2)
    .expect(workflowInvariant)
}
