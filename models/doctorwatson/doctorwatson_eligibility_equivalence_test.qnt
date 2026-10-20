// Frozen06 guards are independent source copies; these are bounded equivalence
// scenarios, not runtime measurements or a replacement sampled transition system.
module doctorwatson_eligibility_equivalence_test {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).* from "./doctorwatson"
  pure def frozen06ExtendAffected(s: State, frozen06Affected: Set[int]): Set[int] = frozen06Affected.union(
    TASKS.filter(id => s.tasks.get(id).dependencies.intersect(frozen06Affected) != Set()))
  pure def frozen06Affected(s: State, id: int): Set[int] =
    TASKS.fold(Set(id), (acc, unused) => frozen06ExtendAffected(s, acc))
  pure def frozen06CanEdit(s: State, id: int, actor: int): bool =
    if (s.terminal or actor != s.tasks.get(id).owner or not(s.tasks.get(id).frozen) or
      not(active(s.tasks.get(id)) or s.tasks.get(id).phase == Merged) or
      (s.tasks.get(id).failures >= 2 and not(s.tasks.get(id).changedApproach))) false
    else TASKS.exclude(frozen06Affected(s, id)).forall(other => not(active(s.tasks.get(other))) or
      s.tasks.get(id).writes.intersect(s.tasks.get(other).writes) == Set())
  pure def frozen06CanConfigure(s: State, id: int, actor: int): bool = and {
    not(s.terminal), actor == s.tasks.get(id).owner,
    s.tasks.get(id).phase == Queued or frozen06CanEdit(s, id, actor),
  }
  pure def frozen06CanConfigureStage(s: State, id: int, dep: int, stage: DependencyStage, actor: int): bool = and {
    frozen06CanConfigure(s, id, actor), s.tasks.get(id).dependencies.contains(dep),
    s.tasks.get(id).dependencyStages.get(dep) != stage,
    stage != SOURCE_INTEGRATED or s.tasks.get(dep).adapter == SourceIntegration,
  }
  pure def frozen06CanRemoveDependency(s: State, id: int, dep: int, actor: int): bool =
    frozen06CanConfigure(s, id, actor) and s.tasks.get(id).dependencies.contains(dep)
  pure def frozen06CanConfigureAdapter(s: State, id: int, adapter: Adapter, actor: int): bool = and {
    frozen06CanConfigure(s, id, actor), s.tasks.get(id).adapter != adapter,
    adapter == SourceIntegration or TASKS.forall(other =>
      not(s.tasks.get(other).dependencies.contains(id)) or s.tasks.get(other).dependencyStages.get(id) != SOURCE_INTEGRATED),
  }
  pure def frozen06SCanConfigureStage(s: State, id: int): bool = frozen06CanConfigure(s, id, s.tasks.get(id).owner) and
    s.tasks.get(id).dependencies.exists(dep => Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).exists(stage =>
      frozen06CanConfigureStage(s, id, dep, stage, s.tasks.get(id).owner)))
  pure def arbitraryGuardAgreement(s: State): bool = TASKS.forall(id =>
    ACTORS.union(Set(0, 99)).forall(actor => and {
      frozen06CanEdit(s, id, actor) == canEdit(s, id, actor),
      frozen06CanConfigure(s, id, actor) == canConfigure(s, id, actor),
      TASKS.forall(dep => and {
        frozen06CanRemoveDependency(s, id, dep, actor) == canRemoveDependency(s, id, dep, actor),
        Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).forall(stage =>
          frozen06CanConfigureStage(s, id, dep, stage, actor) == canConfigureStage(s, id, dep, stage, actor)),
      }),
      Set(SourceIntegration, NativeAdapter).forall(adapter =>
        frozen06CanConfigureAdapter(s, id, adapter, actor) == canConfigureAdapter(s, id, adapter, actor)),
    }))
  val eligibilityAgreement = and {
    currentEditable == TASKS.filter(id => frozen06CanEdit(state, id, state.tasks.get(id).owner)),
    currentConfigurable == TASKS.filter(id => frozen06CanConfigure(state, id, state.tasks.get(id).owner)),
    TASKS.forall(id => and {
      sCanConfigureStageCurrent(id) == frozen06SCanConfigureStage(state, id),
      ACTORS.union(Set(0, 99)).forall(actor => and {
        canEditCurrent(id, actor) == frozen06CanEdit(state, id, actor),
        canConfigureCurrent(id, actor) == frozen06CanConfigure(state, id, actor),
        TASKS.forall(dep => and {
          canRemoveDependencyCurrent(id, dep, actor) == frozen06CanRemoveDependency(state, id, dep, actor),
          Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).forall(stage =>
            canConfigureStageCurrent(id, dep, stage, actor) == frozen06CanConfigureStage(state, id, dep, stage, actor)),
        }),
        Set(SourceIntegration, NativeAdapter).forall(adapter =>
          canConfigureAdapterCurrent(id, adapter, actor) == frozen06CanConfigureAdapter(state, id, adapter, actor)),
      }),
    }),
    arbitraryGuardAgreement(state),
  }
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
  run initialEligibilityEquivalenceTest = init.expect(eligibilityAgreement)
    .expect(currentEditable == Set() and currentConfigurable == TASKS)
  run claimedUnfrozenEligibilityEquivalenceTest = init.expect(eligibilityAgreement).then(claim(1, 1))
    .expect(eligibilityAgreement and not(currentEditable.contains(1)) and not(currentConfigurable.contains(1)))
  run frozenEligibilityEquivalenceTest = init.then(claim(1, 1)).expect(eligibilityAgreement)
    .then(freeze(1, 3, true)).expect(eligibilityAgreement)
    .expect(currentEditable == Set(1) and currentConfigurable == TASKS)
  run candidateEligibilityEquivalenceTest = init.then(prepare(1)).expect(eligibilityAgreement)
    .then(passLocal(1)).expect(eligibilityAgreement and currentEditable.contains(1))
  run mergedSourceEligibilityEquivalenceTest = init.then(integrate(1))
    .expect(eligibilityAgreement and deliverySourceRecorded(state, 1) and currentEditable.contains(1))
  run nativeAcceptedEligibilityEquivalenceTest = init.then(configureStage(3, 1, NATIVE_ACCEPTED, 4))
    .then(configureAdapter(1, NativeAdapter, 1)).then(integrate(2)).then(candidate(1))
    .expect(eligibilityAgreement).then(dispatch(1)).then(extObserve(1, 1, true)).then(cleanup(1))
    .expect(eligibilityAgreement).then(acceptCurrent(1))
    .expect(eligibilityAgreement and deliveryAcceptedRecorded(state, 1) and readyDependencies(state, 3))
  run cancelledEligibilityEquivalenceTest = init.then(prepare(1)).expect(eligibilityAgreement)
    .then(cancel(1, 1)).expect(eligibilityAgreement)
    .expect(not(currentEditable.contains(1)) and not(currentConfigurable.contains(1)))
  run blockedEligibilityEquivalenceTest = init.then(prepare(1)).then(process(1, 30))
    .expect(eligibilityAgreement).then(nameBlocker(1, 1)).expect(eligibilityAgreement)
    .expect(not(currentEditable.contains(1)) and not(currentConfigurable.contains(1)))
  run terminalEligibilityEquivalenceTest = init.then(cancel(1, 1)).then(cancel(2, 2))
    .then(nameBlocker(3, 4)).expect(eligibilityAgreement).then(finalize)
    .expect(eligibilityAgreement and currentEditable == Set() and currentConfigurable == Set())
  run processStopResumeEligibilityEquivalenceTest = init.then(prepare(1))
    .then(process(1, 15)).then(process(1, 15))
    .expect(eligibilityAgreement and processStopped(state.tasks.get(1)) and currentEditable.contains(1))
    .then(edit(1, 1, Source)).then(submit(1)).expect(eligibilityAgreement)
    .expect(state.tasks.get(1).processCycles == 0 and state.tasks.get(1).adminMinutes == 0)
  run repeatFailureRefusalEligibilityEquivalenceTest = init.then(prepare(1)).then(startLocal(1))
    .then(checker(1, jobToken(state, 1), StateMismatch)).then(startLocal(1))
    .then(checker(1, jobToken(state, 1), StateMismatch))
    .expect(eligibilityAgreement and not(currentEditable.contains(1)) and not(currentConfigurable.contains(1)))
    .then(reproduce(1)).then(changeApproach(1)).expect(eligibilityAgreement and currentEditable.contains(1))
  run transitiveStageConfigurationEquivalenceTest = init.then(integrate(1)).then(prepare(2))
    .then(changeDependencies(2, 1, 2)).expect(eligibilityAgreement)
    .then(configureStage(2, 1, NATIVE_OBSERVED, 2)).expect(eligibilityAgreement)
    .then(removeDependency(3, 1, 4)).then(configureAdapter(1, NativeAdapter, 1))
    .expect(eligibilityAgreement and state.tasks.get(2).phase == Queued and not(currentEditable.contains(2)))
  run removeDependencyCacheInvalidationEquivalenceTest = init.then(integrate(1)).then(integrate(2))
    .then(prepare(3)).expect(eligibilityAgreement and currentEditable.contains(3))
    .then(removeDependency(3, 1, 4)).expect(eligibilityAgreement)
    .expect(not(currentEditable.contains(3)) and currentConfigurable.contains(3))
  run adapterRoundTripCacheInvalidationEquivalenceTest = init.then(configureStage(3, 1, NATIVE_OBSERVED, 4))
    .expect(eligibilityAgreement).then(configureAdapter(1, NativeAdapter, 1)).expect(eligibilityAgreement)
    .then(configureAdapter(1, SourceIntegration, 1)).expect(eligibilityAgreement)
    .expect(state.tasks.get(1).adapter == SourceIntegration and currentConfigurable.contains(1))
  run wrongActorAndMissingEdgeEquivalenceTest = init.then(prepare(1)).expect(eligibilityAgreement)
    .expect(not(canEditCurrent(1, 99)) and not(canConfigureCurrent(1, 2)))
    .expect(not(canConfigureStageCurrent(1, 2, NATIVE_ACCEPTED, 1)))
    .expect(not(canRemoveDependencyCurrent(1, 2, 1)))
    .expect(not(canConfigureAdapterCurrent(1, NativeAdapter, 1)))
  run overlapEligibilityRefusalEquivalenceTest = init.then(prepare(1)).expect(eligibilityAgreement)
    .expect(
      val overlapped = { ...state, tasks: state.tasks.setBy(2, t => { ...t, phase: Working, writes: Set(1) }) }
      not(frozen06CanEdit(overlapped, 1, 1)) and not(canEdit(overlapped, 1, 1)) and arbitraryGuardAgreement(overlapped))
    .expect(arbitraryGuardAgreement({ ...state, terminal: true }))
}
