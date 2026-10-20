// Ordinary selected-operation lifecycle only; Root execution pending.
// Source-integrated dependency readiness is modeled by Merged. This is not a
// model of observed/accepted native dependency stages or M1-M4 target validity.
module doctorwatson_ext_test {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).* from "./doctorwatson"

  action prepare(id: int): bool = claim(id, if (id == 3) 4 else id)
    .then(freeze(id, 3, true)).then(edit(id, if (id == 3) 4 else id, Source)).then(submit(id))
  action passLocal(id: int): bool = startLocal(id)
    .then(checker(id, jobToken(state, id), CompleteMatch))
    .then(regressions(id, jobToken(state, id), true))
  action candidate(id: int): bool = prepare(id).then(passLocal(id))

  // Positive: one dispatch, token-bound pass, cleanup clears the debt.
  run extPassAndCleanupTest = init.then(candidate(1)).then(dispatch(1))
    .expect(state.tasks.get(1).ext == Dispatched and state.tasks.get(1).cleanupOwed)
    .expect(not(canCleanup(state, 1)))
    .then(extObserve(1, state.tasks.get(1).extAttempt, true))
    .expect(not(extUsable(state, 1)) and noEscalation)
    .then(cleanup(1)).expect(not(state.tasks.get(1).cleanupOwed) and cleaned and extUsable(state, 1))
  // Negative: no dispatch before local pass / from Queued.
  run extNoEarlyDispatchTest = init.expect(not(canDispatch(state, 1)))
    .then(prepare(1)).expect(not(canDispatch(state, 1)))
  // Negative: late/stale attempt token cannot complete the attempt.
  run extLateTokenRefusedTest = init.then(candidate(1)).then(dispatch(1))
    .expect(not(canExtObserve(state, 1, state.tasks.get(1).extAttempt + 1)))
    .expect(not(canExtObserve(state, 1, 0)))
  // Negative: missing receipt after dispatch is unknown, consumed, owed.
  run extUnknownConsumedTest = init.then(candidate(1)).then(dispatch(1))
    .then(extUnknown(1, state.tasks.get(1).extAttempt))
    .expect(state.tasks.get(1).ext == ExtUnknown and not(extUsable(state, 1)))
    .expect(not(canDispatch(state, 1)) and state.tasks.get(1).cleanupOwed)
    .then(cleanup(1)).expect(not(state.tasks.get(1).cleanupOwed))
  // Negative: an edit after an imported pass stales usability, keeps history.
  run extEditStalesPassTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, state.tasks.get(1).extAttempt, true)).then(cleanup(1))
    .expect(extUsable(state, 1))
    .then(edit(1, 1, Source))
    .expect(state.tasks.get(1).ext == ExtPassed and not(extUsable(state, 1)))
  // Negative: failure import is terminal for the attempt, never a pass.
  run extFailTest = init.then(candidate(2)).then(dispatch(2))
    .then(extObserve(2, state.tasks.get(2).extAttempt, false))
    .expect(state.tasks.get(2).ext == ExtFailed and not(extUsable(state, 2)))
    .expect(not(canDispatch(state, 2)))
  // Invariant spot-checks preserved from the ordinary model.
  run extInvariantsTest = init.then(candidate(1)).then(dispatch(1))
    .expect(workflowInvariant and admittedLayers == Set(K, Quint, LeanClean, Handwritten))

  action unknownCandidate(id: int): bool = candidate(id).then(dispatch(id)).then(extUnknown(id, 1))
  action resolveUnknown(id: int, kind: ResolutionKind): bool = reconcile(id,
    externalResult(state, id), terminalEvidence(externalToken(state, id), kind))

  run correctOriginalResolutionTest = init.then(unknownCandidate(1))
    .then(resolveUnknown(1, ProcessExited))
    .expect(reconciledExited and state.extResolutions.size() == 1)
    .expect(state.extResults.contains(externalResult(state, 1)))
    .expect(externalDischarged(state, 1) and not(extUsable(state, 1)) and workflowInvariant)

  run wrongAttemptResolutionRefusedTest = init.then(unknownCandidate(1))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), attempt: 2 }, ProcessExited))))
    .expect(state.extResolutions == Set() and state.tasks.get(1).cleanupOwed)

  run wrongFingerprintResolutionRefusedTest = init.then(unknownCandidate(1))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), fingerprint: emptyFingerprint }, ProcessExited))))

  run wrongOriginalResultRefusedTest = init.then(unknownCandidate(1))
    .expect(not(canReconcile(state, 1, { ...externalResult(state, 1), outcome: ExtFailed },
      terminalEvidence(externalToken(state, 1), ProcessExited))))

  run nonterminalEvidenceRefusedTest = init.then(unknownCandidate(1))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      { ...terminalEvidence(externalToken(state, 1), ProcessExited), terminal: false })))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      { ...terminalEvidence(externalToken(state, 1), ProcessExited), cleanupComplete: false })))

  run cleanupAloneCannotResolveUnknownTest = init.then(unknownCandidate(1)).then(cleanup(1))
    .expect(state.extCleanups.size() == 1 and state.extResolutions == Set())
    .expect(not(externalDischarged(state, 1)) and not(canDispatch(state, 1)))
    .expect(workflowInvariant)

  run duplicateResolutionRefusedTest = init.then(unknownCandidate(1))
    .then(resolveUnknown(1, ProcessNotStarted))
    .expect(reconciledNotStarted)
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      terminalEvidence(externalToken(state, 1), TxIncluded))))
    .expect(state.extResolutions.size() == 1 and state.extResults.size() == 1)

  run includedResolutionIsNotResendOrNativePassTest = init.then(unknownCandidate(1))
    .then(resolveUnknown(1, TxIncluded))
    .expect(reconciledIncluded and externalDischarged(state, 1))
    .expect(not(canDispatch(state, 1)) and not(extUsable(state, 1)))
    .expect(state.tasks.get(1).ext == ExtUnknown and noEscalation)
    .expect(state.tasks.get(1).observed == Set() and state.fullCompletion == Set())

  run changedUnknownStillNeedsOriginalResolutionTest = init.then(unknownCandidate(1)).then(cleanup(1))
    .then(edit(1, 1, Source)).then(submit(1)).then(passLocal(1))
    .expect(not(canDispatch(state, 1)))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), fingerprint: fingerprint(state, 1) }, ProcessExited))))
    .then(resolveUnknown(1, ProcessExited)).expect(canDispatch(state, 1))
    .then(dispatch(1)).expect(state.tasks.get(1).extAttempt == 2 and workflowInvariant)
    .expect(state.extResults.exists(r => r.token.attempt == 1 and r.outcome == ExtUnknown))
    .expect(state.extResolutions.exists(x => x.original.token.attempt == 1))

  run staleLatePassRetainedTest = init.then(candidate(1)).then(dispatch(1)).then(edit(1, 1, Source))
    .expect(not(canExtObserveToken(state, 1,
      { ...externalToken(state, 1), fingerprint: fingerprint(state, 1) })))
    .then(extObserveToken(1, externalToken(state, 1), true)).then(cleanup(1))
    .expect(not(extUsable(state, 1)) and state.extResults.size() == 1 and workflowInvariant)

  run failedSameFingerprintCannotRetryTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, false)).then(cleanup(1))
    .expect(not(canDispatch(state, 1)) and state.extResults.size() == 1)

  run cancelledUnknownDischargesWithOriginalEvidenceTest = init.then(unknownCandidate(1))
    .then(cancel(1, 1)).then(resolveUnknown(1, ProcessNotStarted))
    .expect(externalDischarged(state, 1) and not(extUsable(state, 1)))
    .expect(not(canDispatch(state, 1)) and workflowInvariant)

  run blockedPendingCanDischargeTest = init.then(candidate(1)).then(dispatch(1))
    .then(process(1, 30)).then(nameBlocker(1, 1))
    .then(extUnknown(1, 1)).then(resolveUnknown(1, ProcessExited))
    .expect(state.tasks.get(1).phase == Blocked and externalDischarged(state, 1))
    .expect(not(extUsable(state, 1)) and workflowInvariant)

  run staleAttemptCannotFinishNewOperationTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, false)).then(cleanup(1))
    .then(edit(1, 1, Source)).then(submit(1)).then(passLocal(1)).then(dispatch(1))
    .expect(not(canExtObserve(state, 1, 1)))
    .then(extObserve(1, 2, true)).then(cleanup(1))
    .expect(extUsable(state, 1) and state.extResults.size() == 2 and workflowInvariant)

  run finalizationRejectsKnownResultCleanupDebtTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, false)).then(cancel(1, 1)).then(cancel(2, 2)).then(cancel(3, 4))
    .expect(not(canFinalize(state)))
    .then(cleanup(1)).then(finalize).expect(terminalDischarged and workflowInvariant)

  run otherTaskResolutionRefusedTest = init.then(unknownCandidate(1))
    .expect(not(canReconcile(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), task: 2 }, ProcessExited))))

  run originalHistorySurvivesSharedInputResetTest = init.then(candidate(1)).then(dispatch(1))
    .then(sharedInput(4, Tool)).then(extUnknown(1, 1)).then(resolveUnknown(1, ProcessExited))
    .expect(state.tasks.get(1).phase == Queued and not(extUsable(state, 1)))
    .expect(state.extDispatches.size() == 1 and state.extResults.size() == 1 and state.extResolutions.size() == 1)
    .expect(workflowInvariant)
}
