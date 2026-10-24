// Focused integrate-first design projection, not an executor or runtime refinement.
// Actual59cbb API descriptors below name real functions; inspected source/API and
// actual result/milestone observations are explicit external premises, never
// inferred from an identity hash or provider terminal SUCCESS.
module doctorwatson_delivery_flow {
  type API = LoadReadiness | ResolveTask | InspectDelivery
  type Predicate = ReadinessFromCurrentInputs | ConsumerUsesCurrentReadiness
  type Phase = Queued | Ready | Running | Checked | Integrated
  type Provider = ProviderSucceeded | ProviderInterrupted | ProviderFailed
  type Verdict = Approve | Revise
  type Failure = NoFailure | CheckMismatch | CheckTimeout
  type Milestone = RecursiveProof | StagenetReadback | DegreeAndBenchmark |
    KLane | QuintLane | CleanLane | HandwrittenCircuit
  type Cut = { source: int, api: API }
  // Presence and actual signature shape, not a native invocation or invented API.
  type Call = { api: API, plan: bool, history: bool, reconciliations: bool, taskArgument: int }
  type Stamp = { cut: Cut, approach: int, attempt: int, base: int }
  type Key = { task: int, cut: Cut, call: Call, dependency: Stamp, approach: int, attempt: int, base: int }
  type ActualResult = { key: Key, observed: bool, passed: bool, failure: Failure }
  type Review = { key: Key, inspectedSource: Cut, inspectedAPI: API,
    sourceRead: bool, apiRead: bool, reviewer: int, provider: Provider, verdict: Verdict }
  type Merge = { key: Key, integrator: int }
  type ProjectCut = { base: int, one: Key, two: Key }
  type MilestoneResult = { cut: ProjectCut, kind: Milestone, actual: bool, passed: bool }
  type Task = { owner: int, predicate: Predicate, cut: Cut, call: Call,
    phase: Phase, approach: int, attempt: int, base: int, pending: Key,
    failure: Failure, sameFailures: int }
  type State = { tasks: int -> Task, integrationBase: int, complete: bool,
    results: Set[ActualResult], reviews: Set[Review], merges: Set[Merge], milestones: Set[MilestoneResult] }
  pure val TASKS: Set[int] = Set(1, 2)
  pure val ROOT: int = 3
  pure val REVIEWER: int = 4
  pure val REQUIRED: Set[Milestone] = Set(RecursiveProof, StagenetReadback, DegreeAndBenchmark,
    KLane, QuintLane, CleanLane, HandwrittenCircuit)
  pure val emptyCut: Cut = { source: -1, api: InspectDelivery }
  pure val emptyCall: Call = { api: InspectDelivery, plan: false, history: false, reconciliations: false, taskArgument: 0 }
  pure val emptyStamp: Stamp = { cut: emptyCut, approach: -1, attempt: 0, base: -1 }
  pure val emptyKey: Key = { task: 0, cut: emptyCut, call: emptyCall, dependency: emptyStamp,
    approach: -1, attempt: 0, base: -1 }
  pure def actualAPI(id: int): API = if (id == 1) LoadReadiness else ResolveTask
  pure def expectedPredicate(id: int): Predicate =
    if (id == 1) ReadinessFromCurrentInputs else ConsumerUsesCurrentReadiness
  pure def actualCall(id: int): Call = { api: actualAPI(id), plan: true,
    history: true, reconciliations: true, taskArgument: if (id == 1) 0 else id }
  pure def initialTask(id: int): Task = { owner: id, predicate: expectedPredicate(id),
    cut: { source: 0, api: actualAPI(id) }, call: actualCall(id), phase: Queued,
    approach: 0, attempt: 0, base: 0, pending: emptyKey, failure: NoFailure, sameFailures: 0 }
  pure val initialState: State = { tasks: TASKS.mapBy(id => initialTask(id)), integrationBase: 0,
    complete: false, results: Set(), reviews: Set(), merges: Set(), milestones: Set() }
  var state: State
  pure def stamp(s: State, id: int): Stamp = { cut: s.tasks.get(id).cut,
    approach: s.tasks.get(id).approach, attempt: s.tasks.get(id).attempt, base: s.tasks.get(id).base }
  pure def keyAt(s: State, id: int, attempt: int): Key = {
    task: id, cut: s.tasks.get(id).cut, call: s.tasks.get(id).call,
    dependency: if (id == 1) emptyStamp else stamp(s, 1),
    approach: s.tasks.get(id).approach, attempt: attempt, base: s.tasks.get(id).base,
  }
  pure def readyKey(s: State, id: int): Key = {
    ...keyAt(s, id, s.tasks.get(id).attempt + 1), base: s.integrationBase,
  }
  // load_readiness(plan, history=None, reconciliations=None) rejects recs without
  // raw history; resolve_task additionally requires its actual task argument.
  pure def validCall(id: int, call: Call): bool = and {
    call.api == actualAPI(id), call.plan,
    not(call.reconciliations) or call.history,
    call.taskArgument == (if (id == 1) 0 else id),
  }
  pure def currentPass(s: State, id: int): bool = and {
    Set(Checked, Integrated).contains(s.tasks.get(id).phase),
    s.tasks.get(id).pending == keyAt(s, id, s.tasks.get(id).attempt),
    s.results.contains({ key: s.tasks.get(id).pending, observed: true, passed: true, failure: NoFailure }),
  }
  pure def examined(review: Review, key: Key): bool = and {
    review.key == key, review.inspectedSource == key.cut, review.inspectedAPI == key.cut.api,
    review.sourceRead, review.apiRead, review.reviewer == REVIEWER,
    review.provider == ProviderSucceeded,
  }
  pure def applicable(review: Review, key: Key): bool = examined(review, key) and review.verdict == Approve
  pure def reviewedForMerge(s: State, key: Key): bool =
    s.reviews.exists(r => applicable(r, key)) and
    not(s.reviews.exists(r => examined(r, key) and r.verdict == Revise))
  pure def currentMerge(s: State, id: int): bool = and {
    s.tasks.get(id).phase == Integrated, currentPass(s, id),
    reviewedForMerge(s, s.tasks.get(id).pending),
    s.merges.contains({ key: s.tasks.get(id).pending, integrator: ROOT }),
  }
  pure def dependenciesReady(s: State, id: int): bool = id == 1 or currentMerge(s, 1)
  pure def canReady(s: State, id: int, actor: int): bool = and {
    not(s.complete), actor == s.tasks.get(id).owner, s.tasks.get(id).phase == Queued,
    s.tasks.get(id).predicate == expectedPredicate(id), s.tasks.get(id).cut.api == actualAPI(id),
    validCall(id, s.tasks.get(id).call), dependenciesReady(s, id),
    s.tasks.get(id).sameFailures < 2, s.tasks.get(id).attempt < 3,
  }
  pure def applyReady(s: State, id: int): State = { ...s, tasks: s.tasks.setBy(id, t =>
    { ...t, phase: Ready, base: s.integrationBase, pending: readyKey(s, id) }) }
  pure def canStart(s: State, id: int): bool = and {
    not(s.complete), s.tasks.get(id).phase == Ready, dependenciesReady(s, id),
    s.tasks.get(id).base == s.integrationBase, s.tasks.get(id).sameFailures < 2,
    s.tasks.get(id).pending == keyAt(s, id, s.tasks.get(id).attempt + 1),
  }
  pure def applyStart(s: State, id: int): State = { ...s, tasks: s.tasks.setBy(id, t =>
    { ...t, phase: Running, attempt: t.attempt + 1 }) }
  pure def resultFor(s: State, id: int, passed: bool, failure: Failure): ActualResult = {
    key: s.tasks.get(id).pending, observed: true, passed: passed, failure: failure,
  }
  pure def canRecordResult(s: State, result: ActualResult): bool = and {
    not(s.complete), TASKS.contains(result.key.task), result.observed,
    result.passed iff result.failure == NoFailure,
    s.tasks.get(result.key.task).phase == Running,
    result.key == s.tasks.get(result.key.task).pending,
    result.key == keyAt(s, result.key.task, s.tasks.get(result.key.task).attempt),
  }
  pure def applyResult(s: State, result: ActualResult): State = { ...s,
    results: s.results.union(Set(result)), tasks: s.tasks.setBy(result.key.task, t => { ...t,
      phase: if (result.passed) Checked else Queued,
      failure: result.failure,
      sameFailures: if (result.passed) 0 else if (t.failure == result.failure) t.sameFailures + 1 else 1,
    }) }
  pure def goodReview(s: State, id: int): Review = { key: s.tasks.get(id).pending,
    inspectedSource: s.tasks.get(id).cut, inspectedAPI: s.tasks.get(id).cut.api,
    sourceRead: true, apiRead: true, reviewer: REVIEWER, provider: ProviderSucceeded, verdict: Approve }
  // Retain wrong-API/interrupted/revise records, but never apply them as approval.
  pure def canRecordReview(s: State, review: Review): bool = and {
    not(s.complete), TASKS.contains(review.key.task), currentPass(s, review.key.task),
    s.tasks.get(review.key.task).phase == Checked,
    review.key == s.tasks.get(review.key.task).pending,
  }
  pure def applyReview(s: State, review: Review): State = { ...s, reviews: s.reviews.union(Set(review)) }
  pure def canMerge(s: State, id: int, actor: int): bool = and {
    not(s.complete), actor == ROOT, s.integrationBase < 5,
    s.tasks.get(id).phase == Checked, s.tasks.get(id).base == s.integrationBase,
    currentPass(s, id), dependenciesReady(s, id),
    reviewedForMerge(s, s.tasks.get(id).pending),
  }
  pure def applyMerge(s: State, id: int): State = { ...s, integrationBase: s.integrationBase + 1,
    tasks: s.tasks.setBy(id, t => { ...t, phase: Integrated }),
    merges: s.merges.union(Set({ key: s.tasks.get(id).pending, integrator: ROOT })) }
  pure def affected(id: int): Set[int] = if (id == 1) TASKS else Set(2)
  pure def canChangeCut(s: State, id: int, actor: int): bool =
    actor == s.tasks.get(id).owner and s.tasks.get(id).cut.source == 0
  pure def applyCut(s: State, id: int): State = { ...s, complete: false,
    tasks: TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (affected(id).contains(other)) { ...t, phase: Queued, pending: emptyKey,
        cut: if (other == id) { ...t.cut, source: 1 } else t.cut }
      else t
    }) }
  pure def canChangeApproach(s: State, id: int, actor: int): bool = and {
    not(s.complete), actor == s.tasks.get(id).owner,
    s.tasks.get(id).sameFailures >= 2, s.tasks.get(id).approach == 0,
  }
  pure def applyApproach(s: State, id: int): State = { ...s,
    tasks: TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (affected(id).contains(other)) { ...t, phase: Queued, pending: emptyKey,
        approach: if (other == id) 1 else t.approach,
        failure: if (other == id) NoFailure else t.failure,
        sameFailures: if (other == id) 0 else t.sameFailures }
      else t
    }) }
  pure def canConfigureCall(s: State, id: int, actor: int): bool =
    not(s.complete) and actor == s.tasks.get(id).owner and s.tasks.get(id).phase == Queued
  pure def applyCall(s: State, id: int, call: Call): State =
    { ...s, tasks: s.tasks.setBy(id, t => { ...t, call: call }) }
  pure def projectCut(s: State): ProjectCut = {
    base: s.integrationBase, one: s.tasks.get(1).pending, two: s.tasks.get(2).pending,
  }
  pure def canObserveMilestone(s: State, observation: MilestoneResult): bool = and {
    not(s.complete), TASKS.forall(id => currentMerge(s, id)),
    observation.cut == projectCut(s), observation.actual, observation.passed,
  }
  pure def applyMilestone(s: State, observation: MilestoneResult): State =
    { ...s, milestones: s.milestones.union(Set(observation)) }
  pure def canComplete(s: State): bool = and {
    not(s.complete), TASKS.forall(id => currentMerge(s, id)),
    REQUIRED.forall(kind => s.milestones.contains({ cut: projectCut(s), kind: kind, actual: true, passed: true })),
  }
  action init = state' = initialState
  action ready(id: int, actor: int): bool = all { canReady(state, id, actor), state' = applyReady(state, id) }
  action start(id: int): bool = all { canStart(state, id), state' = applyStart(state, id) }
  action recordResult(result: ActualResult): bool = all { canRecordResult(state, result), state' = applyResult(state, result) }
  action recordReview(review: Review): bool = all { canRecordReview(state, review), state' = applyReview(state, review) }
  action merge(id: int, actor: int): bool = all { canMerge(state, id, actor), state' = applyMerge(state, id) }
  action changeCut(id: int, actor: int): bool = all { canChangeCut(state, id, actor), state' = applyCut(state, id) }
  action changeApproach(id: int, actor: int): bool = all {
    canChangeApproach(state, id, actor), state' = applyApproach(state, id),
  }
  action configureCall(id: int, actor: int, call: Call): bool = all {
    canConfigureCall(state, id, actor), state' = applyCall(state, id, call),
  }
  action observeMilestone(observation: MilestoneResult): bool = all {
    canObserveMilestone(state, observation), state' = applyMilestone(state, observation),
  }
  action complete = all { canComplete(state), state' = { ...state, complete: true } }
  action step = any {
    { nondet id = TASKS.filter(id => canReady(state, id, id)).oneOf() ready(id, id) },
    { nondet id = TASKS.filter(id => canStart(state, id)).oneOf() start(id) },
    { nondet id = TASKS.filter(id => state.tasks.get(id).phase == Running).oneOf()
      nondet failure = Set(NoFailure, CheckMismatch, CheckTimeout).oneOf()
      recordResult(resultFor(state, id, failure == NoFailure, failure)) },
    { nondet id = TASKS.filter(id => state.tasks.get(id).phase == Checked and not(state.complete)).oneOf()
      nondet api = Set(LoadReadiness, ResolveTask, InspectDelivery).oneOf()
      nondet terminal = Set(ProviderSucceeded, ProviderInterrupted, ProviderFailed).oneOf()
      nondet verdict = Set(Approve, Revise).oneOf()
      recordReview({ ...goodReview(state, id), inspectedAPI: api, provider: terminal, verdict: verdict }) },
    { nondet id = TASKS.filter(id => canMerge(state, id, ROOT)).oneOf() merge(id, ROOT) },
    { nondet id = TASKS.filter(id => canChangeCut(state, id, id)).oneOf() changeCut(id, id) },
    { nondet id = TASKS.filter(id => canChangeApproach(state, id, id)).oneOf() changeApproach(id, id) },
    { nondet id = TASKS.filter(id => canConfigureCall(state, id, id)).oneOf()
      nondet history = Set(true, false).oneOf()
      configureCall(id, id, { ...actualCall(id), history: history }) },
    { nondet kind = REQUIRED.oneOf()
      observeMilestone({ cut: projectCut(state), kind: kind, actual: true, passed: true }) },
    complete,
  }
  val integrated = TASKS.forall(id => currentMerge(state, id))
  val singleIntegrator = state.merges.forall(m => m.integrator == ROOT)
  val reviewedCurrentMerge = TASKS.forall(id => state.tasks.get(id).phase == Integrated implies currentMerge(state, id))
  val dependentCurrent = currentMerge(state, 2) implies currentMerge(state, 1)
  val checkedActual = TASKS.forall(id => Set(Checked, Integrated).contains(state.tasks.get(id).phase) implies currentPass(state, id))
  val failureStop = TASKS.forall(id => state.tasks.get(id).sameFailures >= 2 implies
    (not(canReady(state, id, id)) and not(canStart(state, id))))
  val noPassingRequeue = TASKS.forall(id => Set(Checked, Integrated).contains(state.tasks.get(id).phase) implies not(canReady(state, id, id)))
  val completionSound = state.complete implies (integrated and REQUIRED.forall(kind =>
    state.milestones.contains({ cut: projectCut(state), kind: kind, actual: true, passed: true })))
  val bounds = state.tasks.keys() == TASKS and state.integrationBase <= 5 and TASKS.forall(id => and {
    state.tasks.get(id).owner == id, state.tasks.get(id).cut.source >= 0, state.tasks.get(id).cut.source <= 1,
    state.tasks.get(id).attempt <= 3, state.tasks.get(id).approach <= 1, state.tasks.get(id).sameFailures <= 2,
  })
  val workflowInvariant = singleIntegrator and reviewedCurrentMerge and dependentCurrent and checkedActual and
    failureStop and noPassingRequeue and completionSound and bounds

  // Directed tests stay here to keep this ONE new file self-contained. They
  // execute only modeled transitions/imported premises, never native commands.
  action passed(id: int): bool = ready(id, id).then(start(id))
    .then(recordResult(resultFor(state, id, true, NoFailure))).expect(workflowInvariant)
  action reviewed(id: int): bool = passed(id).then(recordReview(goodReview(state, id)))
    .expect(canMerge(state, id, ROOT) and workflowInvariant)
  action integrate(id: int): bool = reviewed(id).then(merge(id, ROOT)).expect(currentMerge(state, id) and workflowInvariant)
  action failOnce(id: int, kind: Failure): bool = ready(id, id).then(start(id))
    .then(recordResult(resultFor(state, id, false, kind))).expect(workflowInvariant)
  action observed(kind: Milestone): bool = observeMilestone({ cut: projectCut(state), kind: kind, actual: true, passed: true })
  action allMilestones: bool = observed(RecursiveProof).then(observed(StagenetReadback)).then(observed(DegreeAndBenchmark))
    .then(observed(KLane)).then(observed(QuintLane)).then(observed(CleanLane)).then(observed(HandwrittenCircuit))

  run initialPendingTest = init.expect(workflowInvariant and not(integrated) and not(canComplete(state)))
    .expect(state.milestones == Set() and not(canReady(state, 2, 2)))
  run readyIsNotActualResultTest = init.then(ready(1, 1))
    .expect(not(currentPass(state, 1)) and not(canMerge(state, 1, ROOT)) and state.results == Set())
  run observedResultMustMatchCurrentAttemptTest = init.then(ready(1, 1)).then(start(1))
    .expect(not(canRecordResult(state, { ...resultFor(state, 1, true, NoFailure), key: { ...state.tasks.get(1).pending, attempt: 2 } })))
    .expect(not(canRecordResult(state, { ...resultFor(state, 1, true, NoFailure), observed: false })))
    .then(recordResult(resultFor(state, 1, true, NoFailure))).expect(currentPass(state, 1) and workflowInvariant)
  run providerSuccessIsNotApplicableReviewTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), sourceRead: false, apiRead: false }))
    .expect(state.reviews.size() == 1 and not(canMerge(state, 1, ROOT)))
  run wrongActualAPIReviewCannotApproveTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), inspectedAPI: InspectDelivery }))
    .expect(state.reviews.size() == 1 and not(canMerge(state, 1, ROOT)))
    .then(recordReview(goodReview(state, 1))).expect(canMerge(state, 1, ROOT))
  run wrongInspectedSourceCannotApproveTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), inspectedSource: { source: 1, api: LoadReadiness } }))
    .expect(not(canMerge(state, 1, ROOT)))
  run interruptedAndReviseAreNotApprovalTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), provider: ProviderInterrupted }))
    .then(recordReview({ ...goodReview(state, 1), verdict: Revise }))
    .expect(state.reviews.size() == 2 and not(canMerge(state, 1, ROOT)))
  run applicableReviseNeedsChangedCutTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), verdict: Revise }))
    .then(recordReview(goodReview(state, 1))).expect(not(canMerge(state, 1, ROOT)))
    .then(changeCut(1, 1)).then(passed(1)).then(recordReview(goodReview(state, 1)))
    .then(merge(1, ROOT)).expect(currentMerge(state, 1) and state.reviews.size() == 3 and workflowInvariant)
  run sourceAuthorCannotSupplyIndependentReviewTest = init.then(passed(1))
    .then(recordReview({ ...goodReview(state, 1), reviewer: 1 })).expect(not(canMerge(state, 1, ROOT)))
  run rootAloneIntegratesTest = init.then(reviewed(1))
    .expect(not(canMerge(state, 1, 1)) and not(canMerge(state, 1, REVIEWER)))
    .then(merge(1, ROOT)).expect(currentMerge(state, 1) and workflowInvariant)
  run reachableIntegrateFirstTest = init.expect(not(canReady(state, 2, 2)))
    .then(integrate(1)).expect(canReady(state, 2, 2)).then(integrate(2))
    .expect(integrated and state.integrationBase == 2 and state.results.size() == 2 and state.merges.size() == 2)
    .expect(not(canComplete(state)) and state.milestones == Set() and workflowInvariant)
  run passingChecksAreNotRequeuedTest = init.then(passed(1)).expect(not(canReady(state, 1, 1)))
    .then(recordReview(goodReview(state, 1))).then(merge(1, ROOT)).expect(not(canReady(state, 1, 1)))
  run staleReviewCannotApproveChangedCutTest = init.then(reviewed(1)).then(changeCut(1, 1))
    .expect(state.reviews.size() == 1 and state.results.size() == 1 and not(canMerge(state, 1, ROOT)))
    .then(passed(1)).expect(not(canMerge(state, 1, ROOT)))
    .then(recordReview(goodReview(state, 1))).expect(canMerge(state, 1, ROOT) and workflowInvariant)
  run changedUpstreamInvalidatesDependentEvidenceTest = init.then(integrate(1)).then(integrate(2)).then(changeCut(1, 1))
    .expect(not(integrated) and state.tasks.get(2).phase == Queued and state.tasks.get(2).cut.source == 0)
    .expect(state.results.size() == 2 and state.reviews.size() == 2 and state.merges.size() == 2)
    .expect(not(canReady(state, 2, 2)) and not(canComplete(state)) and workflowInvariant)
  run bothSourceChangesRemainIntegrableTest = init.then(integrate(1)).then(integrate(2))
    .then(changeCut(2, 2)).then(integrate(2)).then(changeCut(1, 1))
    .then(integrate(1)).then(integrate(2))
    .expect(integrated and state.integrationBase == 5 and state.tasks.get(2).attempt == 3 and workflowInvariant)
  run staleRunningResultCannotFinishNewCutTest = init.then(ready(1, 1)).then(start(1))
    .then(changeCut(1, 1))
    .expect(not(canRecordResult(state, { key: keyAt({ ...state, tasks: state.tasks.setBy(1, t => { ...t, cut: { source: 0, api: LoadReadiness } }) }, 1, 1), observed: true, passed: true, failure: NoFailure })))
    .expect(state.results == Set() and workflowInvariant)
  run twoSameFailuresRequireNewApproachTest = init.then(failOnce(1, CheckMismatch)).then(failOnce(1, CheckMismatch))
    .expect(state.tasks.get(1).sameFailures == 2 and not(canReady(state, 1, 1)))
    .then(changeApproach(1, 1)).expect(canReady(state, 1, 1)).then(integrate(1))
    .expect(state.tasks.get(1).attempt == 3 and state.tasks.get(1).approach == 1 and workflowInvariant)
  run changedCutDoesNotEraseFailureThresholdTest = init.then(failOnce(1, CheckMismatch)).then(failOnce(1, CheckMismatch))
    .then(changeCut(1, 1)).expect(state.tasks.get(1).sameFailures == 2 and not(canReady(state, 1, 1)))
    .then(configureCall(1, 1, actualCall(1))).expect(not(canReady(state, 1, 1)) and workflowInvariant)
  run differentFailureClassStartsNewStreakTest = init.then(failOnce(1, CheckMismatch)).then(failOnce(1, CheckTimeout))
    .expect(state.tasks.get(1).sameFailures == 1 and canReady(state, 1, 1) and workflowInvariant)
  run actualCallShapeAndHistoryRequiredTest = init
    .then(configureCall(1, 1, { ...actualCall(1), history: false }))
    .expect(not(canReady(state, 1, 1)))
    .then(configureCall(1, 1, { ...actualCall(1), api: InspectDelivery })).expect(not(canReady(state, 1, 1)))
    .then(configureCall(1, 1, actualCall(1))).then(integrate(1))
    .then(configureCall(2, 2, { ...actualCall(2), taskArgument: 1 })).expect(not(canReady(state, 2, 2)))
    .then(configureCall(2, 2, actualCall(2))).then(integrate(2)).expect(integrated and workflowInvariant)
  run sourceMergeDoesNotCompleteFormalOrNativeTest = init.then(integrate(1)).then(integrate(2))
    .expect(not(canComplete(state)) and state.milestones == Set())
    .expect(not(canObserveMilestone(state, { cut: projectCut(state), kind: RecursiveProof, actual: false, passed: true })))
  run allActualMilestonesAreNecessaryTest = init.then(integrate(1)).then(integrate(2))
    .then(observed(RecursiveProof)).then(observed(StagenetReadback)).then(observed(DegreeAndBenchmark))
    .expect(not(canComplete(state)))
    .then(observed(KLane)).then(observed(QuintLane)).then(observed(CleanLane)).expect(not(canComplete(state)))
    .then(observed(HandwrittenCircuit)).expect(canComplete(state)).then(complete)
    .expect(state.complete and workflowInvariant)
  run completionRearmedByChangedCutTest = init.then(integrate(1)).then(integrate(2)).then(allMilestones).then(complete)
    .then(changeCut(1, 1)).expect(not(state.complete) and not(canComplete(state)) and state.milestones.size() == 7)
    .then(integrate(1)).then(integrate(2)).expect(integrated and not(canComplete(state)))
    .then(allMilestones).then(complete).expect(state.complete and state.milestones.size() == 14 and workflowInvariant)
}
