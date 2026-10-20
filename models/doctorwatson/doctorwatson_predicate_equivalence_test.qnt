// Direct old/new predicate comparisons, not another transition system.
module doctorwatson_predicate_equivalence_test {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).* from "./doctorwatson"

  pure def originalResultResolved(s: State, r: ExtResult): bool =
    r.outcome != ExtUnknown or s.extResolutions.exists(x => x.original == r)
  pure def originalExternalDischarged(s: State, id: int): bool =
    s.extDispatches.filter(token => token.task == id).forall(token =>
      s.extCleanups.contains(token) and s.extResults.exists(r =>
        r.token == token and originalResultResolved(s, r)))
  pure def originalCanDispatch(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate,
    s.tasks.get(id).test == Passed, fresh(s, id), readyDependencies(s, id),
    originalExternalDischarged(s, id), not(s.tasks.get(id).cleanupOwed),
    // Explicit next selection must consume genuinely changed inputs. No same
    // fingerprint resend follows a failure, unknown resolution or tx inclusion.
    not(s.extDispatches.exists(token => token.task == id and token.fingerprint == fingerprint(s, id))),
  }
  pure def originalCanReconcile(s: State, id: int, original: ExtResult, evidence: TerminalEvidence): bool = and {
    not(s.terminal), original == externalResult(s, id), original.outcome == ExtUnknown,
    s.extResults.contains(original), evidence.token == original.token,
    evidence.terminal, evidence.cleanupComplete,
    not(s.extResolutions.exists(x => x.original.token == original.token)),
  }
  pure def originalExternalHistorySound(s: State): bool = and {
    s.extResults.forall(r => s.extDispatches.contains(r.token) and
      Set(ExtPassed, ExtFailed, ExtUnknown).contains(r.outcome)),
    s.extResults.forall(r => s.extResults.forall(other =>
      r.token != other.token or r == other)),
    s.extCleanups.forall(token => s.extResults.exists(r => r.token == token)),
    s.extResolutions.forall(x => s.extResults.contains(x.original) and
      x.original.outcome == ExtUnknown and x.evidence.token == x.original.token and
      x.evidence.terminal and x.evidence.cleanupComplete and s.extCleanups.contains(x.original.token)),
    s.extResolutions.forall(x => s.extResolutions.forall(other =>
      x.original.token != other.original.token or x == other)),
    TASKS.forall(id => s.extDispatches.filter(token => token.task == id).size() == s.tasks.get(id).extAttempt),
    s.extDispatches.forall(token => token.attempt == s.tasks.get(token.task).extAttempt or
      (s.extCleanups.contains(token) and s.extResults.exists(r => r.token == token and originalResultResolved(s, r)))),
    s.extDispatches.forall(token => token.task == token.fingerprint.task and token.attempt >= 1 and
      token.attempt <= s.tasks.get(token.task).extAttempt),
    s.extDispatches.forall(token => s.extDispatches.forall(other =>
      token.task != other.task or token == other or
      (token.attempt != other.attempt and token.fingerprint != other.fingerprint))),
    TASKS.forall(id => s.tasks.get(id).extAttempt == 0 or
      (s.extDispatches.contains(externalToken(s, id)) and
       (s.tasks.get(id).ext == Dispatched or s.extResults.contains(externalResult(s, id))))),
  }

  pure def currentOriginalReconcile(s: State, id: int): bool = originalCanReconcile(s, id,
    externalResult(s, id), terminalEvidence(externalToken(s, id), ProcessExited))
  pure def predicateAgreement(s: State): bool = and {
    TASKS.forall(id => and {
      originalExternalDischarged(s, id) == externalDischarged(s, id),
      originalCanDispatch(s, id) == canDispatch(s, id),
      currentOriginalReconcile(s, id) == canReconcileCurrent(s, id),
    }),
    s.extResults.forall(r => originalResultResolved(s, r) == resultResolved(s, r)),
    originalExternalHistorySound(s) == externalHistorySoundFor(s),
  }
  pure def identityAgreement(s: State, id: int, original: ExtResult, evidence: TerminalEvidence): bool =
    originalCanReconcile(s, id, original, evidence) == canReconcile(s, id, original, evidence)

  action prepare(id: int): bool = claim(id, if (id == 3) 4 else id)
    .then(freeze(id, 3, true)).then(edit(id, if (id == 3) 4 else id, Source)).then(submit(id))
  action passLocal(id: int): bool = startLocal(id)
    .then(checker(id, jobToken(state, id), CompleteMatch))
    .then(regressions(id, jobToken(state, id), true))
  action candidate(id: int): bool = prepare(id).then(passLocal(id))
  action unknownCandidate(id: int): bool = candidate(id).then(dispatch(id)).then(extUnknown(id, 1))
  action resolve(id: int, kind: ResolutionKind): bool = reconcile(id, externalResult(state, id),
    terminalEvidence(externalToken(state, id), kind))

  run emptyAndOrdinaryStateEquivalenceTest = init.expect(predicateAgreement(state))
    .then(prepare(1)).expect(predicateAgreement(state))
    .then(passLocal(1)).expect(predicateAgreement(state))
  run pendingPassCleanupEquivalenceTest = init.then(candidate(1)).then(dispatch(1))
    .expect(predicateAgreement(state)).then(extObserve(1, 1, true)).expect(predicateAgreement(state))
    .then(cleanup(1)).expect(predicateAgreement(state))
  run changedFailureAndSecondAttemptEquivalenceTest = init.then(candidate(1)).then(dispatch(1))
    .then(extObserve(1, 1, false)).then(cleanup(1)).expect(predicateAgreement(state))
    .then(edit(1, 1, Source)).then(submit(1)).then(passLocal(1)).expect(predicateAgreement(state))
    .then(dispatch(1)).expect(predicateAgreement(state))
    .then(extObserve(1, 2, true)).then(cleanup(1)).expect(predicateAgreement(state))
  run unknownCleanupAndNotStartedEquivalenceTest = init.then(unknownCandidate(1))
    .expect(predicateAgreement(state)).then(cleanup(1)).expect(predicateAgreement(state))
    .then(resolve(1, ProcessNotStarted)).expect(predicateAgreement(state))
  run exitedAndIncludedEquivalenceTest = init.then(unknownCandidate(1))
    .then(resolve(1, ProcessExited)).expect(predicateAgreement(state))
    .then(edit(1, 1, Source)).then(submit(1)).then(passLocal(1)).then(dispatch(1))
    .then(extUnknown(1, 2)).then(resolve(1, TxIncluded)).expect(predicateAgreement(state))
  run cancelledBlockedAndFinalizedEquivalenceTest = init.then(unknownCandidate(1))
    .then(cancel(1, 1)).expect(predicateAgreement(state))
    .then(resolve(1, ProcessExited)).then(cancel(2, 2)).then(nameBlocker(3, 4))
    .expect(predicateAgreement(state)).then(finalize).expect(predicateAgreement(state))
  run sourceDriftLateResultEquivalenceTest = init.then(candidate(1)).then(dispatch(1))
    .then(edit(1, 1, Source)).expect(predicateAgreement(state))
    .then(extObserve(1, 1, true)).then(cleanup(1)).expect(predicateAgreement(state))
  run sharedInputAndTwoTaskHistoryEquivalenceTest = init.then(unknownCandidate(1))
    .then(resolve(1, ProcessExited)).then(unknownCandidate(2))
    .then(sharedInput(4, Tool)).expect(predicateAgreement(state))
    .then(resolve(2, TxIncluded)).expect(predicateAgreement(state))
  run adversarialTokenIdentityEquivalenceTest = init.then(unknownCandidate(1))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), attempt: 2 }, ProcessExited)))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), task: 2 }, ProcessExited)))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      terminalEvidence({ ...externalToken(state, 1), fingerprint: emptyFingerprint }, ProcessExited)))
    .expect(identityAgreement(state, 1, { ...externalResult(state, 1), outcome: ExtFailed },
      terminalEvidence(externalToken(state, 1), ProcessExited)))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      { ...terminalEvidence(externalToken(state, 1), ProcessExited), terminal: false }))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      { ...terminalEvidence(externalToken(state, 1), ProcessExited), cleanupComplete: false }))
  run duplicateResolutionIdentityEquivalenceTest = init.then(unknownCandidate(1))
    .then(resolve(1, ProcessExited)).expect(predicateAgreement(state))
    .expect(identityAgreement(state, 1, externalResult(state, 1),
      terminalEvidence(externalToken(state, 1), TxIncluded)))
}
