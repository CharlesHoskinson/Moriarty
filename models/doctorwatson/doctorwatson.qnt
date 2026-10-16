// Authoritative local factory state, not a scheduler or a message protocol.
module doctorwatson {
  type Phase = Queued | Working | Candidate | Merged | Cancelled | Blocked
  type Stage = Idle | Checker | Regressions | Stopped | Complete
  type Observation = CompleteMatch | Incomplete | IdentityMismatch | StateMismatch
  type Result = NotRun | Running | Passed | Failed | TimedOut
  type Tier = Ordinary | Correspondence | Native | Target
  type Layer = K | Quint | LeanClean | Handwritten | NativeLedger
  type Mutation = Source | Profile | Oracle | Bounds | Tool | Projection | Dependency
  type Event = Initial | Claimed | Frozen | Edited | Submitted | Started | CheckerAdvanced | Finished |
    Reviewed | Rebased | Integrated | Availability | LayerRecorded | ProcessOnly |
    Reproduced | ApproachChanged | CancelledEvent | BlockerNamed | Finalized | DependenciesChanged | Completed
  type Fingerprint = { task: int, version: int, base: int, dependencies: int -> int }
  type JobToken = { fingerprint: Fingerprint, attempt: int }
  type Receipt = { task: int, fingerprint: Fingerprint, tier: Tier, result: Result, actor: int, local: bool, attempt: int, layers: Set[Layer] }
  type Review = { fingerprint: Fingerprint, reviewer: int, approved: bool }
  type Task = {
    phase: Phase, owner: int, oracleOwner: int, writes: Set[int], shared: bool,
    dependencies: Set[int], required: Set[Layer], version: int, base: int,
    frozen: bool, oracleIndependent: bool, test: Result, stage: Stage, attempt: int, tested: Fingerprint,
    reviewed: Review, observed: Set[Layer], failures: int, reproduced: bool,
    changedApproach: bool, processCycles: int, adminMinutes: int,
  }
  type State = {
    tasks: int -> Task, integrationBase: int, available: Set[Layer],
    receipts: Set[Receipt], reviews: Set[Review], event: Event, lastTask: int, terminal: bool,
    sourceMerges: Set[int], fullCompletion: Set[int],
  }
  // The finite instance bounds participants, never concurrent work-in-progress.
  const TASKS: Set[int]
  const ACTORS: Set[int]
  const INTEGRATOR: int
  pure def admittedLayers: Set[Layer] = Set(K, Quint, LeanClean, Handwritten)
  // External applicable-owner predicate; its security policy is not derived here.
  const ELIGIBLE_REVIEWERS: int -> Set[int]
  var state: State

  pure def emptyFingerprint: Fingerprint = { task: 0, version: -1, base: -1, dependencies: Map() }
  pure def emptyReview: Review = { fingerprint: emptyFingerprint, reviewer: 0, approved: false }
  pure def initialTask(id: int): Task = {
    phase: Queued, owner: if (id == 3) INTEGRATOR else id, oracleOwner: 3,
    writes: Set(id), shared: id == 3,
    dependencies: if (id == 3) Set(1, 2) else Set(),
    required: Set(K, Quint, LeanClean, Handwritten), version: 0, base: 0,
    frozen: false, oracleIndependent: false, test: NotRun, stage: Idle, attempt: 0,
    tested: emptyFingerprint, reviewed: emptyReview, observed: Set(),
    failures: 0, reproduced: false, changedApproach: false, processCycles: 0, adminMinutes: 0,
  }
  pure def initialState: State = {
    tasks: TASKS.mapBy(id => initialTask(id)), integrationBase: 0, available: Set(),
    receipts: Set(), reviews: Set(), event: Initial, lastTask: 0, terminal: false,
    sourceMerges: Set(), fullCompletion: Set(),
  }
  pure def active(t: Task): bool = t.phase == Working or t.phase == Candidate
  pure def readyDependencies(s: State, id: int): bool =
    s.tasks.get(id).dependencies.forall(dep => s.tasks.get(dep).phase == Merged)
  pure def disjoint(s: State, id: int): bool = TASKS.forall(other =>
    other == id or not(active(s.tasks.get(other))) or
    s.tasks.get(id).writes.intersect(s.tasks.get(other).writes) == Set())
  pure def canClaim(s: State, id: int, actor: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Queued, actor == s.tasks.get(id).owner,
    ACTORS.contains(actor), readyDependencies(s, id), disjoint(s, id),
    not(s.tasks.get(id).shared) or actor == INTEGRATOR,
  }
  pure def applyClaim(s: State, id: int): State = {
    ...s, tasks: s.tasks.setBy(id, t => { ...t, phase: Working, base: s.integrationBase }),
    event: Claimed, lastTask: id,
  }
  pure def fingerprint(s: State, id: int): Fingerprint = {
    task: id, version: s.tasks.get(id).version, base: s.tasks.get(id).base,
    dependencies: s.tasks.get(id).dependencies.mapBy(dep => s.tasks.get(dep).version),
  }
  pure def reset(t: Task): Task = {
    ...t, test: NotRun, stage: Idle, tested: emptyFingerprint,
    reviewed: emptyReview, observed: Set(),
  }
  pure def changeTask(s: State, id: int, t: Task, event: Event): State =
    { ...s, tasks: s.tasks.set(id, t), event: event, lastTask: id }
  pure def canFreeze(s: State, id: int, actor: int, independent: bool): bool = and {
    not(s.terminal), active(s.tasks.get(id)), not(s.tasks.get(id).frozen),
    actor == s.tasks.get(id).oracleOwner, actor != s.tasks.get(id).owner, independent,
  }
  pure def applyFreeze(s: State, id: int): State = changeTask(s, id,
    { ...s.tasks.get(id), frozen: true, oracleIndependent: true }, Frozen)
  pure def extendAffected(s: State, affected: Set[int]): Set[int] = affected.union(
    TASKS.filter(id => s.tasks.get(id).dependencies.intersect(affected) != Set()))
  pure def affected(s: State, id: int): Set[int] =
    TASKS.fold(Set(id), (acc, unused) => extendAffected(s, acc))
  pure def processStopped(t: Task): bool = t.processCycles >= 2 or t.adminMinutes >= 30
  pure def canEdit(s: State, id: int, actor: int): bool =
    if (s.terminal or actor != s.tasks.get(id).owner or not(s.tasks.get(id).frozen) or
      not(active(s.tasks.get(id)) or s.tasks.get(id).phase == Merged) or
      (s.tasks.get(id).failures >= 2 and not(s.tasks.get(id).changedApproach))) false
    else TASKS.exclude(affected(s, id)).forall(other => not(active(s.tasks.get(other))) or
      s.tasks.get(id).writes.intersect(s.tasks.get(other).writes) == Set())
  pure def applyEdit(s: State, id: int, kind: Mutation): State = {
    val impacted = affected(s, id)
    val updated = s.tasks.keys().mapBy(other => {
      val t = s.tasks.get(other)
      val disposed = t.phase == Cancelled or t.phase == Blocked
      if (impacted.contains(other)) {
        ...reset(t), version: t.version + 1,
        phase: if (disposed) t.phase else if (other == id) Working else Queued,
        frozen: t.frozen and other == id and kind != Oracle and kind != Profile and kind != Bounds and kind != Projection,
        processCycles: if (disposed) t.processCycles else 0,
        adminMinutes: if (disposed) t.adminMinutes else 0,
        failures: if (disposed) t.failures else if (t.changedApproach) 0 else t.failures,
        reproduced: if (disposed) t.reproduced else false,
        changedApproach: if (disposed) t.changedApproach else false,
      } else t
    })
    { ...s, tasks: updated, sourceMerges: s.sourceMerges.exclude(impacted),
      fullCompletion: s.fullCompletion.exclude(impacted), event: Edited, lastTask: id }
  }
  pure def fresh(s: State, id: int): bool =
    s.tasks.get(id).base == s.integrationBase and s.tasks.get(id).tested == fingerprint(s, id)
  // Shared profile/oracle/bounds/tool/projection input is consumed by every task.
  pure def canSharedInput(s: State, actor: int, kind: Mutation): bool = and {
    not(s.terminal), actor == INTEGRATOR, Set(Profile, Oracle, Bounds, Tool, Projection).contains(kind),
    TASKS.exists(id => active(s.tasks.get(id)) or s.tasks.get(id).phase == Merged),
  }
  pure def applySharedInput(s: State): State = {
    val tasks = TASKS.mapBy(id => {
      val t = s.tasks.get(id)
      if (t.phase == Cancelled or t.phase == Blocked) t else
        { ...reset(t), phase: Queued, version: t.version + 1, frozen: false, processCycles: 0, adminMinutes: 0 }
    })
    { ...s, tasks: tasks, sourceMerges: Set(), fullCompletion: Set(), event: Edited, lastTask: 0 }
  }
  pure def canSubmit(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Working, s.tasks.get(id).version > 0,
    s.tasks.get(id).frozen, s.tasks.get(id).oracleIndependent,
    s.tasks.get(id).base == s.integrationBase, readyDependencies(s, id),
  }
  pure def applySubmit(s: State, id: int): State =
    changeTask(s, id, { ...s.tasks.get(id), phase: Candidate }, Submitted)
  pure def canStart(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate, s.tasks.get(id).frozen,
    s.tasks.get(id).test != Running, s.tasks.get(id).test != Passed,
    s.tasks.get(id).base == s.integrationBase, s.tasks.get(id).failures < 2,
    not(processStopped(s.tasks.get(id))), readyDependencies(s, id),
  }
  pure def applyStart(s: State, id: int): State = changeTask(s, id,
    { ...s.tasks.get(id), test: Running, stage: Checker, attempt: s.tasks.get(id).attempt + 1, tested: fingerprint(s, id) }, Started)
  pure def jobToken(s: State, id: int): JobToken = {
    fingerprint: s.tasks.get(id).tested, attempt: s.tasks.get(id).attempt,
  }
  pure def canFinish(s: State, id: int, stamp: JobToken): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate, s.tasks.get(id).test == Running,
    s.tasks.get(id).attempt == stamp.attempt, s.tasks.get(id).tested == stamp.fingerprint,
    s.tasks.get(id).base == s.integrationBase, stamp.fingerprint == fingerprint(s, id),
  }
  pure def applyResult(s: State, id: int, result: Result): State = {
    val t = s.tasks.get(id)
    val receipt = { task: id, fingerprint: t.tested, tier: Ordinary, result: result, actor: t.owner, local: true, attempt: t.attempt, layers: Set() }
    val updatedState = changeTask(s, id, { ...t, test: result, stage: if (result == Passed) Complete else Stopped,
      failures: if (result == Passed) 0 else t.failures + 1,
      reproduced: false, changedApproach: false }, Finished)
    { ...updatedState, receipts: s.receipts.union(Set(receipt)) }
  }
  pure def applyChecker(s: State, id: int, observation: Observation): State =
    if (observation == CompleteMatch)
      changeTask(s, id, { ...s.tasks.get(id), stage: Regressions }, CheckerAdvanced)
    else applyResult(s, id, Failed)

  pure def canReview(s: State, id: int, actor: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate, s.tasks.get(id).test == Passed,
    fresh(s, id), actor != s.tasks.get(id).owner, ELIGIBLE_REVIEWERS.get(id).contains(actor),
    s.tasks.get(id).reviewed.fingerprint != fingerprint(s, id),
  }
  pure def applyReview(s: State, id: int, actor: int, approved: bool): State = {
    val review = { fingerprint: fingerprint(s, id), reviewer: actor, approved: approved }
    val updatedState = changeTask(s, id, { ...s.tasks.get(id), reviewed: review }, Reviewed)
    { ...updatedState, reviews: s.reviews.union(Set(review)) }
  }
  pure def canRebase(s: State, id: int): bool = and {
    not(s.terminal), active(s.tasks.get(id)), s.tasks.get(id).base != s.integrationBase,
  }
  pure def applyRebase(s: State, id: int): State = changeTask(s, id,
    { ...reset(s.tasks.get(id)), phase: Working, base: s.integrationBase,
      version: s.tasks.get(id).version + 1 }, Rebased)
  pure def canMerge(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate, readyDependencies(s, id),
    s.tasks.get(id).test == Passed, fresh(s, id), s.tasks.get(id).reviewed.approved,
    s.tasks.get(id).reviewed.fingerprint == fingerprint(s, id),
    s.tasks.get(id).reviewed.reviewer != s.tasks.get(id).owner,
    ELIGIBLE_REVIEWERS.get(id).contains(s.tasks.get(id).reviewed.reviewer),
    not(processStopped(s.tasks.get(id))),
  }
  pure def applyMerge(s: State, id: int): State = {
    val tasks = TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (other == id) { ...t, phase: Merged }
      else if (active(t)) { ...reset(t), phase: Working }
      else t
    })
    { ...s, tasks: tasks, integrationBase: s.integrationBase + 1,
      sourceMerges: s.sourceMerges.union(Set(id)), event: Integrated, lastTask: id }
  }
  pure def canChangeDependencies(s: State, id: int, dep: int, actor: int): bool =
    if (s.terminal or actor != s.tasks.get(id).owner or id == dep or
      s.tasks.get(id).dependencies.contains(dep) or s.tasks.get(dep).phase != Merged) false
    else canEdit(s, id, actor) and not(affected(s, id).contains(dep))
  pure def applyDependencies(s: State, id: int, dep: int): State = {
    val editedState = applyEdit(s, id, Dependency)
    val t = editedState.tasks.get(id)
    changeTask(editedState, id, { ...t, dependencies: t.dependencies.union(Set(dep)) }, DependenciesChanged)
  }
  pure def canAvailability(s: State, layer: Layer): bool =
    not(s.terminal) and not(s.available.contains(layer))
  pure def applyAvailability(s: State, layer: Layer): State =
    { ...s, available: s.available.union(Set(layer)), event: Availability, lastTask: 0 }
  pure def canObserve(s: State, id: int, layer: Layer): bool = and {
    not(s.terminal), s.available.contains(layer), admittedLayers.contains(layer),
    s.tasks.get(id).phase == Candidate or s.tasks.get(id).phase == Merged,
    s.tasks.get(id).test == Passed, s.tasks.get(id).tested == fingerprint(s, id),
    not(s.tasks.get(id).observed.contains(layer)),
  }
  pure def applyObserve(s: State, id: int, layer: Layer): State = {
    val t = s.tasks.get(id)
    val receipt = { task: id, fingerprint: fingerprint(s, id), tier: Correspondence,
      result: Passed, actor: INTEGRATOR, local: true, attempt: t.attempt, layers: Set(layer) }
    val updatedState = changeTask(s, id, { ...t, observed: t.observed.union(Set(layer)) }, LayerRecorded)
    { ...updatedState, receipts: s.receipts.union(Set(receipt)) }
  }
  pure def canComplete(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Merged,
    s.tasks.get(id).required.subseteq(s.tasks.get(id).observed), not(s.fullCompletion.contains(id)),
  }
  pure def applyComplete(s: State, id: int): State =
    { ...s, fullCompletion: s.fullCompletion.union(Set(id)), event: Completed, lastTask: id }
  pure def canProcess(s: State, id: int): bool =
    not(s.terminal) and active(s.tasks.get(id)) and not(processStopped(s.tasks.get(id))) and s.tasks.get(id).test != Running
  pure def applyProcess(s: State, id: int, minutes: int): State = changeTask(s, id,
    { ...s.tasks.get(id), processCycles: s.tasks.get(id).processCycles + 1,
      adminMinutes: s.tasks.get(id).adminMinutes + minutes }, ProcessOnly)
  pure def canReproduce(s: State, id: int): bool = and {
    not(s.terminal), active(s.tasks.get(id)), s.tasks.get(id).failures >= 2, not(s.tasks.get(id).reproduced),
  }
  pure def applyReproduce(s: State, id: int): State =
    changeTask(s, id, { ...s.tasks.get(id), reproduced: true }, Reproduced)
  pure def canApproach(s: State, id: int): bool = and {
    not(s.terminal), active(s.tasks.get(id)), s.tasks.get(id).reproduced, not(s.tasks.get(id).changedApproach),
  }
  pure def applyApproach(s: State, id: int): State =
    changeTask(s, id, { ...s.tasks.get(id), changedApproach: true }, ApproachChanged)
  pure def canCancel(s: State, id: int, actor: int): bool = and {
    not(s.terminal), actor == s.tasks.get(id).owner,
    active(s.tasks.get(id)) or s.tasks.get(id).phase == Queued,
  }
  pure def applyCancel(s: State, id: int): State =
    changeTask(s, id, { ...reset(s.tasks.get(id)), phase: Cancelled }, CancelledEvent)
  pure def canBlock(s: State, id: int, actor: int): bool = and {
    not(s.terminal), actor == s.tasks.get(id).owner,
    (active(s.tasks.get(id)) and (processStopped(s.tasks.get(id)) or s.tasks.get(id).failures >= 2)) or
    (s.tasks.get(id).phase == Queued and s.tasks.get(id).dependencies.exists(dep =>
      s.tasks.get(dep).phase == Cancelled or s.tasks.get(dep).phase == Blocked)),
  }
  pure def applyBlock(s: State, id: int): State =
    changeTask(s, id, { ...reset(s.tasks.get(id)), phase: Blocked }, BlockerNamed)
  pure def canFinalize(s: State): bool = not(s.terminal) and TASKS.forall(id =>
    Set(Merged, Cancelled, Blocked).contains(s.tasks.get(id).phase))
  pure def applyFinalize(s: State): State = { ...s, terminal: true, event: Finalized, lastTask: 0 }

  action init = state' = initialState
  action claim(id: int, actor: int): bool = all { canClaim(state, id, actor), state' = applyClaim(state, id) }
  action freeze(id: int, actor: int, independent: bool): bool = all {
    canFreeze(state, id, actor, independent), state' = applyFreeze(state, id),
  }
  action edit(id: int, actor: int, kind: Mutation): bool = all {
    canEdit(state, id, actor), state' = applyEdit(state, id, kind),
  }
  action sharedInput(actor: int, kind: Mutation): bool = all {
    canSharedInput(state, actor, kind), state' = applySharedInput(state),
  }
  action submit(id: int): bool = all { canSubmit(state, id), state' = applySubmit(state, id) }
  action startLocal(id: int): bool = all { canStart(state, id), state' = applyStart(state, id) }
  action checker(id: int, stamp: JobToken, observation: Observation): bool = all {
    canFinish(state, id, stamp), state.tasks.get(id).stage == Checker,
    state' = applyChecker(state, id, observation),
  }
  action regressions(id: int, stamp: JobToken, passed: bool): bool = all {
    canFinish(state, id, stamp), state.tasks.get(id).stage == Regressions,
    state' = applyResult(state, id, if (passed) Passed else Failed),
  }
  action timeout(id: int, stamp: JobToken): bool = all {
    canFinish(state, id, stamp), state' = applyResult(state, id, TimedOut),
  }
  action review(id: int, actor: int, approved: bool): bool = all {
    canReview(state, id, actor), state' = applyReview(state, id, actor, approved),
  }
  action rebase(id: int): bool = all { canRebase(state, id), state' = applyRebase(state, id) }
  action merge(id: int): bool = all { canMerge(state, id), state' = applyMerge(state, id) }
  action changeDependencies(id: int, dep: int, actor: int): bool = all {
    canChangeDependencies(state, id, dep, actor), state' = applyDependencies(state, id, dep),
  }
  action makeAvailable(layer: Layer): bool = all { canAvailability(state, layer), state' = applyAvailability(state, layer) }
  action observe(id: int, layer: Layer): bool = all { canObserve(state, id, layer), state' = applyObserve(state, id, layer) }
  action complete(id: int): bool = all { canComplete(state, id), state' = applyComplete(state, id) }
  action process(id: int, minutes: int): bool = all {
    Set(15, 30).contains(minutes), canProcess(state, id), state' = applyProcess(state, id, minutes),
  }
  action reproduce(id: int): bool = all { canReproduce(state, id), state' = applyReproduce(state, id) }
  action changeApproach(id: int): bool = all { canApproach(state, id), state' = applyApproach(state, id) }
  action cancel(id: int, actor: int): bool = all { canCancel(state, id, actor), state' = applyCancel(state, id) }
  action nameBlocker(id: int, actor: int): bool = all { canBlock(state, id, actor), state' = applyBlock(state, id) }
  action finalize = all { canFinalize(state), state' = applyFinalize(state) }
  // Choose within enabled operations: an unlucky task/actor cannot stop a trace.
  action step = all { not(state.terminal), {
    val editable = TASKS.filter(id => canEdit(state, id, state.tasks.get(id).owner))
    any {
    { nondet id = TASKS.filter(id => canClaim(state, id, state.tasks.get(id).owner)).oneOf()
      claim(id, state.tasks.get(id).owner) },
    { nondet id = TASKS.filter(id => canFreeze(state, id, state.tasks.get(id).oracleOwner, true)).oneOf()
      freeze(id, state.tasks.get(id).oracleOwner, true) },
    { nondet id = editable.oneOf()
      nondet kind = Set(Source, Profile, Oracle, Bounds, Tool, Projection, Dependency).oneOf()
      edit(id, state.tasks.get(id).owner, kind) },
    { nondet kind = Set(Profile, Oracle, Bounds, Tool, Projection).filter(kind => canSharedInput(state, INTEGRATOR, kind)).oneOf()
      sharedInput(INTEGRATOR, kind) },
    { nondet id = TASKS.filter(id => canSubmit(state, id)).oneOf()
      submit(id) },
    { nondet id = TASKS.filter(id => canStart(state, id)).oneOf()
      startLocal(id) },
    { nondet id = TASKS.filter(id => canFinish(state, id, jobToken(state, id)) and state.tasks.get(id).stage == Checker).oneOf()
      nondet observation = Set(CompleteMatch, Incomplete, IdentityMismatch, StateMismatch).oneOf()
      checker(id, jobToken(state, id), observation) },
    { nondet id = TASKS.filter(id => canFinish(state, id, jobToken(state, id)) and state.tasks.get(id).stage == Regressions).oneOf()
      nondet passed = Set(true, false).oneOf()
      regressions(id, jobToken(state, id), passed) },
    { nondet id = TASKS.filter(id => canFinish(state, id, jobToken(state, id))).oneOf()
      timeout(id, jobToken(state, id)) },
    { nondet id = TASKS.filter(id => ELIGIBLE_REVIEWERS.get(id).exists(actor => canReview(state, id, actor))).oneOf()
      nondet actor = ELIGIBLE_REVIEWERS.get(id).filter(actor => canReview(state, id, actor)).oneOf()
      nondet approved = Set(true, false).oneOf()
      review(id, actor, approved) },
    { nondet id = TASKS.filter(id => canRebase(state, id)).oneOf()
      rebase(id) },
    { nondet id = TASKS.filter(id => canMerge(state, id)).oneOf()
      merge(id) },
    { nondet id = editable.filter(id => TASKS.exists(dep => canChangeDependencies(state, id, dep, state.tasks.get(id).owner))).oneOf()
      nondet dep = TASKS.filter(dep => canChangeDependencies(state, id, dep, state.tasks.get(id).owner)).oneOf()
      changeDependencies(id, dep, state.tasks.get(id).owner) },
    { nondet layer = Set(K, Quint, LeanClean, Handwritten, NativeLedger).filter(layer => canAvailability(state, layer)).oneOf()
      makeAvailable(layer) },
    { nondet id = TASKS.filter(id => state.available.exists(layer => canObserve(state, id, layer))).oneOf()
      nondet layer = state.available.filter(layer => canObserve(state, id, layer)).oneOf()
      observe(id, layer) },
    { nondet id = TASKS.filter(id => canComplete(state, id)).oneOf()
      complete(id) },
    { nondet id = TASKS.filter(id => canProcess(state, id)).oneOf()
      nondet minutes = Set(15, 30).oneOf()
      process(id, minutes) },
    { nondet id = TASKS.filter(id => canReproduce(state, id)).oneOf()
      reproduce(id) },
    { nondet id = TASKS.filter(id => canApproach(state, id)).oneOf()
      changeApproach(id) },
    { nondet id = TASKS.filter(id => canCancel(state, id, state.tasks.get(id).owner)).oneOf()
      cancel(id, state.tasks.get(id).owner) },
    { nondet id = TASKS.filter(id => canBlock(state, id, state.tasks.get(id).owner)).oneOf()
      nameBlocker(id, state.tasks.get(id).owner) },
    finalize,
    }
  } }
  val claimed = state.event == Claimed
  val frozen = state.event == Frozen
  val edited = state.event == Edited
  val sharedChanged = state.event == Edited and state.lastTask == 0
  val submitted = state.event == Submitted
  val started = state.event == Started
  val checkerAdvanced = state.event == CheckerAdvanced
  val checkerFailed = state.event == Finished and state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).test == Failed
  val testPassed = state.event == Finished and state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).test == Passed
  val timedOut = state.event == Finished and state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).test == TimedOut
  val reviewed = state.event == Reviewed
  val rebased = state.event == Rebased
  val integrated = state.event == Integrated
  val dependenciesChanged = state.event == DependenciesChanged
  val availabilityChanged = state.event == Availability
  val layerRecorded = state.event == LayerRecorded
  val completed = state.event == Completed
  val processStoppedWitness = state.event == ProcessOnly and processStopped(state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask))
  val reproduced = state.event == Reproduced
  val approachChanged = state.event == ApproachChanged
  val cancelled = state.event == CancelledEvent
  val blockerNamed = state.event == BlockerNamed
  val finalized = state.event == Finalized
  val parallel = active(state.tasks.get(1)) and active(state.tasks.get(2))
  val sourceOnly = state.sourceMerges != Set() and state.fullCompletion == Set() and state.available == Set()
  val ownership = TASKS.forall(id => not(active(state.tasks.get(id))) or disjoint(state, id))
  val independentOracle = TASKS.forall(id => state.tasks.get(id).frozen implies
    (state.tasks.get(id).oracleIndependent and state.tasks.get(id).oracleOwner != state.tasks.get(id).owner))
  val localOnly = state.receipts.forall(r => r.local)
  val noEscalation = state.receipts.forall(r => r.tier == Ordinary or r.tier == Correspondence)
  val exactReviews = TASKS.forall(id => state.tasks.get(id).reviewed.approved implies and {
    state.tasks.get(id).reviewed.fingerprint == fingerprint(state, id),
    state.tasks.get(id).reviewed.reviewer != state.tasks.get(id).owner,
    ELIGIBLE_REVIEWERS.get(id).contains(state.tasks.get(id).reviewed.reviewer),
  })
  val exactMerges = state.sourceMerges.forall(id => and {
    state.tasks.get(id).phase == Merged, state.tasks.get(id).test == Passed,
    state.tasks.get(id).tested == fingerprint(state, id), state.tasks.get(id).reviewed.approved,
  })
  val observedCompletion = state.fullCompletion.forall(id =>
    state.sourceMerges.contains(id) and state.tasks.get(id).required.subseteq(state.tasks.get(id).observed))
  val noPrematureStop = state.terminal or canFinalize(state) or TASKS.exists(id => canCancel(state, id, state.tasks.get(id).owner))
  val workflowInvariant = noPrematureStop and ownership and independentOracle and localOnly and noEscalation and exactReviews and exactMerges and observedCompletion

}

module doctorwatson_small {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).*
}
