// Required behavior replacing frozen-R2 counterexamples; Root execution pending.
module doctorwatson_review_witness_test {
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

  // Requirements replacing the exact five passing defect witnesses in frozen R2.
  run cleanupDebtSuppressesUsabilityTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, true))
    .expect(not(extUsable(state, 1)) and state.tasks.get(1).cleanupOwed)
    .then(cleanup(1)).expect(extUsable(state, 1) and workflowInvariant)

  run cancelledPassRetainedButUnusableTest = init.then(candidate(1)).then(dispatch(1))
    .then(cancel(1, 1)).then(extObserve(1, 1, true)).then(cleanup(1))
    .expect(state.tasks.get(1).phase == Cancelled and not(extUsable(state, 1)))
    .expect(state.extResults.size() == 1 and workflowInvariant)

  run finalizationWaitsForExternalDischargeTest = init.then(candidate(1)).then(dispatch(1))
    .then(cancel(1, 1)).then(cancel(2, 2)).then(cancel(3, 4))
    .expect(not(canFinalize(state)) and canExtUnknown(state, 1, 1))
    .then(extUnknown(1, 1)).then(cleanup(1))
    .expect(not(canFinalize(state)))
    .then(reconcile(1, externalResult(state, 1), terminalEvidence(externalToken(state, 1), ProcessExited)))
    .expect(reconciled and not(cleaned)).then(finalize)
    .expect(state.terminal and terminalDischarged and workflowInvariant)

  run reconciliationDoesNotResendConsumedFingerprintTest = init.then(candidate(1)).then(dispatch(1))
    .then(extUnknown(1, 1)).then(cleanup(1))
    .then(reconcile(1, externalResult(state, 1), terminalEvidence(externalToken(state, 1), ProcessNotStarted)))
    .expect(not(canDispatch(state, 1)) and state.tasks.get(1).ext == ExtUnknown)
    .expect(state.extDispatches.size() == 1 and state.extResults.size() == 1)
    .expect(state.extResolutions.size() == 1 and reconciled and workflowInvariant)

  run changedFailedOperationCanDispatchNewAttemptTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, false)).then(cleanup(1))
    .then(edit(1, 1, Source)).then(submit(1)).then(passLocal(1))
    .expect(fresh(state, 1) and state.tasks.get(1).test == Passed and canDispatch(state, 1))
    .then(dispatch(1)).expect(state.tasks.get(1).extAttempt == 2)
    .expect(state.extDispatches.size() == 2 and state.extResults.size() == 1)
    .expect(state.extResults.exists(r => r.token.attempt == 1 and r.outcome == ExtFailed))
    .expect(workflowInvariant)
}
