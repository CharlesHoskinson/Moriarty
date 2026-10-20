// Authoritative local factory state, not a scheduler or a message protocol.
module doctorwatson {
  // Exact public-consumer dependency selector grammar. The accepted selector
  // applies to SourceIntegration AND NativeAdapter results; it proves no native
  // validity or authenticity. Constructor names retain the requested projection.
  type DependencyStage = SOURCE_INTEGRATED | NATIVE_OBSERVED | NATIVE_ACCEPTED
  type Adapter = SourceIntegration | NativeAdapter
  type Phase = Queued | Working | Candidate | Merged | Cancelled | Blocked
  type Stage = Idle | Checker | Regressions | Stopped | Complete
  type Observation = CompleteMatch | Incomplete | IdentityMismatch | StateMismatch
  type Result = NotRun | Running | Passed | Failed | TimedOut
  type Tier = Ordinary | Correspondence | Native | Target
  // External selected-operation observation, imported as a typed result.
  // It is not a Layer: admittedLayers and noEscalation are unchanged, and
  // no ExtObs value certifies cryptographic or ledger validity.
  type ExtObs = NotDispatched | Dispatched | ExtPassed | ExtFailed | ExtUnknown
  type Layer = K | Quint | LeanClean | Handwritten | NativeLedger
  type Mutation = Source | Profile | Oracle | Bounds | Tool | Projection | Dependency
  type Event = Initial | Claimed | Frozen | Edited | Submitted | Started | CheckerAdvanced | Finished |
    Reviewed | Rebased | Integrated | Availability | LayerRecorded | ProcessOnly |
    DispatchedEvent | ExtObserved | ExtUnknownEvent | CleanedEvent | ReconciledEvent | DeliveryAcceptedEvent |
    Reproduced | ApproachChanged | CancelledEvent | BlockerNamed | Finalized | DependenciesChanged | Completed
  type InputStamp = { source: int, inputs: int, base: int, adapter: Adapter,
    dependencies: Set[int], stages: int -> DependencyStage }
  type EdgeAttempt = { parent: int, dependency: int, stage: DependencyStage, attempt: int }
  type Fingerprint = { task: int, version: int, inputs: int, base: int, adapter: Adapter,
    dependencies: int -> int, dependencyStages: int -> DependencyStage,
    dependencyAttempts: int -> int, transitiveInputs: int -> InputStamp,
    transitiveAttempts: Set[EdgeAttempt] }
  type JobToken = { fingerprint: Fingerprint, attempt: int }
  type Receipt = { task: int, fingerprint: Fingerprint, tier: Tier, result: Result, actor: int, local: bool, attempt: int, layers: Set[Layer] }
  // The operation token pins the SAME original operation, not the current source.
  // attempt retains the existing local/external effort ordinal; operationAttempt
  // is the unique selected-delivery attempt across adapter changes/source merges.
  type ExtToken = { task: int, fingerprint: Fingerprint, attempt: int, operationAttempt: int }
  type ExtResult = { token: ExtToken, outcome: ExtObs, manifest: int }
  type ResolutionKind = ProcessNotStarted | ProcessExited | TxIncluded
  // A terminal-evidence input abstracts validated external observations. A cleanup
  // observation alone never implies nonexecution, process exit or tx inclusion.
  type TerminalEvidence = { token: ExtToken, kind: ResolutionKind, terminal: bool, cleanupComplete: bool }
  type ExtResolution = { original: ExtResult, evidence: TerminalEvidence }
  type DeliveryObservation = { token: ExtToken, adapter: Adapter, manifest: int }
  type DeliveryReview = { observation: DeliveryObservation, reviewer: int, approved: bool }
  type DeliveryAdoption = { observation: DeliveryObservation, review: DeliveryReview, integrator: int, adopted: bool }
  type DeliveryAcceptance = { observation: DeliveryObservation, review: DeliveryReview, adoption: DeliveryAdoption }
  type Review = { fingerprint: Fingerprint, reviewer: int, approved: bool }
  type Task = {
    phase: Phase, owner: int, oracleOwner: int, writes: Set[int], shared: bool,
    dependencies: Set[int], dependencyStages: int -> DependencyStage, adapter: Adapter,
    required: Set[Layer], version: int, inputVersion: int, artifactVersion: int, sourceManifest: int,
    deliveryAttempt: int, sourceAttempt: int, extOperationAttempt: int, base: int,
    frozen: bool, oracleIndependent: bool, test: Result, stage: Stage, attempt: int, tested: Fingerprint,
    reviewed: Review, observed: Set[Layer], failures: int, reproduced: bool,
    changedApproach: bool, processCycles: int, adminMinutes: int,
    ext: ExtObs, extAttempt: int, extPrint: Fingerprint, extManifest: int, cleanupOwed: bool,
  }
  type State = {
    tasks: int -> Task, integrationBase: int, available: Set[Layer],
    receipts: Set[Receipt], reviews: Set[Review], event: Event, lastTask: int, terminal: bool,
    sourceMerges: Set[int], fullCompletion: Set[int],
    extDispatches: Set[ExtToken], extResults: Set[ExtResult],
    extCleanups: Set[ExtToken], extResolutions: Set[ExtResolution],
    deliveryObservations: Set[DeliveryObservation], deliveryAcceptances: Set[DeliveryAcceptance],
  }
  // The finite instance bounds participants, never concurrent work-in-progress.
  const TASKS: Set[int]
  const ACTORS: Set[int]
  const INTEGRATOR: int
  pure val admittedLayers: Set[Layer] = Set(K, Quint, LeanClean, Handwritten)
  // External applicable-owner predicate; its security policy is not derived here.
  const ELIGIBLE_REVIEWERS: int -> Set[int]
  var state: State

  pure val emptyFingerprint: Fingerprint = { task: 0, version: -1, inputs: -1, base: -1, adapter: SourceIntegration,
    dependencies: Map(), dependencyStages: Map(), dependencyAttempts: Map(), transitiveInputs: Map(), transitiveAttempts: Set() }
  pure val emptyReview: Review = { fingerprint: emptyFingerprint, reviewer: 0, approved: false }
  pure def initialTask(id: int): Task = {
    phase: Queued, owner: if (id == 3) INTEGRATOR else id, oracleOwner: 3,
    writes: Set(id), shared: id == 3,
    dependencies: if (id == 3) Set(1, 2) else Set(),
    dependencyStages: if (id == 3) Map(1 -> SOURCE_INTEGRATED, 2 -> SOURCE_INTEGRATED) else Map(),
    adapter: SourceIntegration,
    required: Set(K, Quint, LeanClean, Handwritten), version: 0, inputVersion: 0,
    artifactVersion: 0, sourceManifest: -1, deliveryAttempt: 0, sourceAttempt: 0, extOperationAttempt: 0, base: 0,
    frozen: false, oracleIndependent: false, test: NotRun, stage: Idle, attempt: 0,
    tested: emptyFingerprint, reviewed: emptyReview, observed: Set(),
    failures: 0, reproduced: false, changedApproach: false, processCycles: 0, adminMinutes: 0,
    ext: NotDispatched, extAttempt: 0, extPrint: emptyFingerprint, extManifest: -1, cleanupOwed: false,
  }
  pure val initialState: State = {
    tasks: TASKS.mapBy(id => initialTask(id)), integrationBase: 0, available: Set(),
    receipts: Set(), reviews: Set(), event: Initial, lastTask: 0, terminal: false,
    sourceMerges: Set(), fullCompletion: Set(),
    extDispatches: Set(), extResults: Set(), extCleanups: Set(), extResolutions: Set(),
    deliveryObservations: Set(), deliveryAcceptances: Set(),
  }
  pure def active(t: Task): bool = t.phase == Working or t.phase == Candidate
  pure def dependencyStageName(stage: DependencyStage): str =
    if (stage == SOURCE_INTEGRATED) "source-integrated" else if (stage == NATIVE_OBSERVED) "observed" else "accepted"
  pure def dependencyAncestors(s: State, id: int): Set[int] =
    if (s.tasks.get(id).dependencies == Set()) Set() else
      TASKS.fold(s.tasks.get(id).dependencies, (known, unused) =>
        known.fold(known, (expanded, dep) => expanded.union(s.tasks.get(dep).dependencies)))
  pure def selectedDependencyAttempt(s: State, id: int): int =
    if (s.tasks.get(id).adapter == SourceIntegration) s.tasks.get(id).sourceAttempt else s.tasks.get(id).extOperationAttempt
  pure def sourceObservation(s: State, id: int): DeliveryObservation = {
    token: { task: id, fingerprint: s.tasks.get(id).tested, attempt: s.tasks.get(id).attempt,
      operationAttempt: s.tasks.get(id).sourceAttempt },
    adapter: SourceIntegration, manifest: s.tasks.get(id).sourceManifest,
  }
  pure def externalObservation(s: State, id: int): DeliveryObservation = {
    token: externalToken(s, id), adapter: NativeAdapter, manifest: s.tasks.get(id).extManifest,
  }
  pure def currentDeliveryObservation(s: State, id: int): DeliveryObservation =
    if (s.tasks.get(id).adapter == SourceIntegration) sourceObservation(s, id) else externalObservation(s, id)
  pure def sourceRecordedLocal(s: State, id: int): bool = and {
    s.tasks.get(id).adapter == SourceIntegration, s.tasks.get(id).phase == Merged,
    s.sourceMerges.contains(id), s.tasks.get(id).test == Passed,
    s.tasks.get(id).tested == fingerprint(s, id),
    s.tasks.get(id).sourceManifest == s.tasks.get(id).artifactVersion,
    not(s.tasks.get(id).cleanupOwed), externalDischarged(s, id),
    s.deliveryObservations.contains(sourceObservation(s, id)),
  }
  pure def nativeRecordedLocal(s: State, id: int): bool = and {
    s.tasks.get(id).adapter == NativeAdapter, s.tasks.get(id).ext == ExtPassed,
    active(s.tasks.get(id)) or s.tasks.get(id).phase == Merged,
    s.tasks.get(id).extPrint == fingerprint(s, id),
    s.tasks.get(id).extManifest == s.tasks.get(id).artifactVersion,
    not(s.tasks.get(id).cleanupOwed), externalDischarged(s, id),
    s.deliveryObservations.contains(externalObservation(s, id)),
  }
  pure def observationRecordedLocal(s: State, id: int): bool =
    sourceRecordedLocal(s, id) or nativeRecordedLocal(s, id)
  pure def acceptedRecordedLocal(s: State, id: int): bool =
    observationRecordedLocal(s, id) and s.deliveryAcceptances.exists(a =>
      a.observation == currentDeliveryObservation(s, id))
  pure def stageRecordedLocal(s: State, id: int, stage: DependencyStage): bool =
    if (stage == SOURCE_INTEGRATED) sourceRecordedLocal(s, id)
    else if (stage == NATIVE_OBSERVED) nativeRecordedLocal(s, id) else acceptedRecordedLocal(s, id)
  // Evaluate the DAG's local row predicates for every transitive edge. This is
  // the nonrecursive finite equivalent of consumer topological row assessment.
  pure def readyDependencies(s: State, id: int): bool =
    if (s.tasks.get(id).dependencies == Set()) true else
      dependencyAncestors(s, id).union(Set(id)).forall(parent =>
        s.tasks.get(parent).dependencies.forall(dep =>
          stageRecordedLocal(s, dep, s.tasks.get(parent).dependencyStages.get(dep))))
  pure def deliverySourceRecorded(s: State, id: int): bool = sourceRecordedLocal(s, id) and readyDependencies(s, id)
  pure def deliveryObservedRecorded(s: State, id: int): bool = nativeRecordedLocal(s, id) and readyDependencies(s, id)
  pure def deliveryAcceptedRecorded(s: State, id: int): bool = acceptedRecordedLocal(s, id) and readyDependencies(s, id)
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
  pure def inputStamp(s: State, id: int): InputStamp = {
    source: s.tasks.get(id).version, inputs: s.tasks.get(id).inputVersion,
    base: s.tasks.get(id).base, adapter: s.tasks.get(id).adapter,
    dependencies: s.tasks.get(id).dependencies, stages: s.tasks.get(id).dependencyStages,
  }
  pure def fingerprint(s: State, id: int): Fingerprint = {
    val t = s.tasks.get(id)
    val ancestors = dependencyAncestors(s, id)
    { task: id, version: t.version, inputs: t.inputVersion, base: t.base, adapter: t.adapter,
      dependencies: t.dependencies.mapBy(dep => s.tasks.get(dep).version),
      dependencyStages: t.dependencyStages,
      dependencyAttempts: t.dependencies.mapBy(dep => selectedDependencyAttempt(s, dep)),
      transitiveInputs: ancestors.mapBy(dep => inputStamp(s, dep)),
      transitiveAttempts: ancestors.union(Set(id)).map(parent =>
        s.tasks.get(parent).dependencies.map(dep => {
          parent: parent, dependency: dep, stage: s.tasks.get(parent).dependencyStages.get(dep),
          attempt: selectedDependencyAttempt(s, dep),
        })).flatten(),
    }
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
        inputVersion: if (kind == Source) t.inputVersion else t.inputVersion + 1,
        phase: if (disposed) t.phase else if (other == id) Working else Queued,
        frozen: t.frozen and other == id and kind != Oracle and kind != Profile and kind != Bounds and kind != Projection,
        // Only this task owner's implementation resumes its administration budget.
        processCycles: if (disposed or other != id) t.processCycles else 0,
        adminMinutes: if (disposed or other != id) t.adminMinutes else 0,
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
        { ...reset(t), phase: Queued, version: t.version + 1, inputVersion: t.inputVersion + 1, frozen: false }
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
  // Equivalent to canFinish(s, id, jobToken(s, id)), without allocating a token
  // for every task considered by the sampler before its cheap phase guards.
  pure def canFinishCurrent(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).phase == Candidate, s.tasks.get(id).test == Running,
    s.tasks.get(id).base == s.integrationBase, s.tasks.get(id).tested == fingerprint(s, id),
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
  // Source delivery is the ordinary integration artifact, published atomically
  // only when no prior selected external operation/debt consumes these inputs.
  // Ordinary source integration itself remains possible while independent
  // external work is pending; that cannot mint a usable delivery result.
  pure def canPublishSourceDelivery(s: State, id: int): bool = {
    val currentPrint = fingerprint(s, id)
    and {
      s.tasks.get(id).adapter == SourceIntegration, externalDischarged(s, id),
      not(s.extDispatches.exists(token => token.task == id and token.fingerprint == currentPrint)),
      not(s.deliveryObservations.exists(o => o.token.task == id and o.token.fingerprint == currentPrint)),
    }
  }
  pure def applyMerge(s: State, id: int): State = {
    val publishingSource = canPublishSourceDelivery(s, id)
    val tasks = TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (other == id) { ...t, phase: Merged, sourceManifest: t.artifactVersion,
        deliveryAttempt: if (publishingSource) t.deliveryAttempt + 1 else t.deliveryAttempt,
        sourceAttempt: if (publishingSource) t.deliveryAttempt + 1 else t.sourceAttempt }
      else if (active(t)) { ...reset(t), phase: Working }
      else t
    })
    { ...s, tasks: tasks, integrationBase: s.integrationBase + 1,
      sourceMerges: s.sourceMerges.union(Set(id)),
      deliveryObservations: if (publishingSource)
        s.deliveryObservations.union(Set({ token: { task: id, fingerprint: s.tasks.get(id).tested,
          attempt: s.tasks.get(id).attempt, operationAttempt: s.tasks.get(id).deliveryAttempt + 1 },
          adapter: SourceIntegration, manifest: s.tasks.get(id).artifactVersion }))
        else s.deliveryObservations,
      event: Integrated, lastTask: id }
  }
  pure def canChangeDependencies(s: State, id: int, dep: int, actor: int): bool =
    if (s.terminal or actor != s.tasks.get(id).owner or id == dep or
      s.tasks.get(id).dependencies.contains(dep) or s.tasks.get(dep).phase != Merged or
      s.tasks.get(dep).adapter != SourceIntegration) false
    else canEdit(s, id, actor) and not(affected(s, id).contains(dep))
  pure def applyDependencies(s: State, id: int, dep: int): State = {
    val editedState = applyEdit(s, id, Dependency)
    val t = editedState.tasks.get(id)
    changeTask(editedState, id, { ...t, dependencies: t.dependencies.union(Set(dep)),
      dependencyStages: t.dependencyStages.put(dep, SOURCE_INTEGRATED) }, DependenciesChanged)
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
  // --- Selected external operation in the existing finite workflow ---
  // Typed dependency rows distinguish source integration, native observation,
  // and explicit acceptance for either adapter. No row is native validity.
  pure def externalToken(s: State, id: int): ExtToken = {
    task: id, fingerprint: s.tasks.get(id).extPrint, attempt: s.tasks.get(id).extAttempt,
    operationAttempt: s.tasks.get(id).extOperationAttempt,
  }
  pure def externalResult(s: State, id: int): ExtResult = {
    token: externalToken(s, id), outcome: s.tasks.get(id).ext, manifest: s.tasks.get(id).extManifest,
  }
  pure def terminalEvidence(token: ExtToken, kind: ResolutionKind): TerminalEvidence = {
    token: token, kind: kind, terminal: true, cleanupComplete: true,
  }
  pure def resultResolved(s: State, r: ExtResult): bool =
    r.outcome != ExtUnknown or s.extResolutions.fold(false, (found, x) => found or x.original == r)
  pure def externalDischarged(s: State, id: int): bool =
    s.extDispatches.fold(true, (discharged, token) => discharged and
      (token.task != id or (s.extCleanups.contains(token) and
        s.extResults.fold(false, (found, r) => found or
          (r.token == token and resultResolved(s, r))))))
  pure def extCurrent(s: State, id: int): bool =
    s.tasks.get(id).extPrint == fingerprint(s, id) and readyDependencies(s, id)
  pure def canDispatch(s: State, id: int): bool = {
    // Scoped values are reused within this call; no persistent cached state.
    val t = s.tasks.get(id)
    val currentPrint = fingerprint(s, id)
    and {
      not(s.terminal), t.phase == Candidate, t.test == Passed,
      not(t.cleanupOwed), t.base == s.integrationBase, t.tested == currentPrint,
      readyDependencies(s, id),
      externalDischarged(s, id),
      not(s.extDispatches.fold(false, (consumed, token) => consumed or
        (token.task == id and token.fingerprint == currentPrint))),
    }
  }
  pure def applyDispatch(s: State, id: int): State = {
    val updated = changeTask(s, id,
      { ...s.tasks.get(id), ext: Dispatched, extAttempt: s.tasks.get(id).extAttempt + 1,
        deliveryAttempt: s.tasks.get(id).deliveryAttempt + 1, extOperationAttempt: s.tasks.get(id).deliveryAttempt + 1,
        extPrint: fingerprint(s, id), extManifest: s.tasks.get(id).artifactVersion, cleanupOwed: true }, DispatchedEvent)
    { ...updated, extDispatches: s.extDispatches.union(Set(externalToken(updated, id))) }
  }
  // Late observations bind the original token even after edit/cancel/block;
  // retention is distinct from usability. A different fingerprint/attempt fails.
  pure def canExtObserveToken(s: State, id: int, token: ExtToken): bool = and {
    not(s.terminal), s.tasks.get(id).ext == Dispatched,
    token == externalToken(s, id), s.extDispatches.contains(token),
    not(s.extResults.exists(r => r.token == token)),
  }
  pure def canExtObserve(s: State, id: int, attempt: int): bool =
    attempt == s.tasks.get(id).extAttempt and canExtObserveToken(s, id, externalToken(s, id))
  pure def applyExtObserve(s: State, id: int, passed: bool): State = {
    val outcome = if (passed) ExtPassed else ExtFailed
    val result = { token: externalToken(s, id), outcome: outcome, manifest: s.tasks.get(id).extManifest }
    val updated = changeTask(s, id, { ...s.tasks.get(id), ext: outcome }, ExtObserved)
    { ...updated, extResults: s.extResults.union(Set(result)),
      deliveryObservations: if (passed and result.token.fingerprint.adapter == NativeAdapter)
        s.deliveryObservations.union(Set({ token: result.token, adapter: NativeAdapter, manifest: result.manifest }))
        else s.deliveryObservations }
  }
  pure def canExtUnknown(s: State, id: int, attempt: int): bool = canExtObserve(s, id, attempt)
  pure def applyExtUnknown(s: State, id: int): State = {
    val result = { token: externalToken(s, id), outcome: ExtUnknown, manifest: s.tasks.get(id).extManifest }
    val updated = changeTask(s, id, { ...s.tasks.get(id), ext: ExtUnknown }, ExtUnknownEvent)
    { ...updated, extResults: s.extResults.union(Set(result)) }
  }
  // Cleanup remains operable after task disposition or source drift. It records
  // only the original operation's resource discharge, never terminal outcome.
  pure def canCleanup(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).cleanupOwed,
    s.tasks.get(id).ext != Dispatched, s.tasks.get(id).ext != NotDispatched,
    s.extResults.contains(externalResult(s, id)),
  }
  pure def applyCleanup(s: State, id: int): State = {
    val updated = changeTask(s, id, { ...s.tasks.get(id), cleanupOwed: false }, CleanedEvent)
    { ...updated, extCleanups: s.extCleanups.union(Set(externalToken(s, id))) }
  }
  // Resolution appends evidence for the exact UNKNOWN ORIGINAL RESULT. It does
  // not rewrite the original, mint a new attempt, reset to NotDispatched or
  // manufacture ExtPassed. TxIncluded remains an externally supplied terminal
  // disposition; typed artifact/native/ledger validity is outside this model.
  pure def canReconcile(s: State, id: int, original: ExtResult, evidence: TerminalEvidence): bool = and {
    not(s.terminal), original == externalResult(s, id), original.outcome == ExtUnknown,
    s.extResults.contains(original), evidence.token == original.token,
    evidence.terminal, evidence.cleanupComplete,
    not(s.extResolutions.exists(x => x.original.token == original.token)),
  }
  // Equivalent to the full public guard with current result/evidence. Keep
  // that guard unchanged for supplied external identities and negative tests.
  pure def canReconcileCurrent(s: State, id: int): bool = and {
    not(s.terminal), s.tasks.get(id).ext == ExtUnknown,
    canReconcile(s, id, externalResult(s, id), terminalEvidence(externalToken(s, id), ProcessExited)),
  }
  pure def applyReconcile(s: State, id: int, original: ExtResult, evidence: TerminalEvidence): State = {
    val updated = changeTask(s, id, { ...s.tasks.get(id), cleanupOwed: false }, ReconciledEvent)
    { ...updated, extCleanups: s.extCleanups.union(Set(original.token)),
      extResolutions: s.extResolutions.union(Set({ original: original, evidence: evidence })) }
  }
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
    Set(Merged, Cancelled, Blocked).contains(s.tasks.get(id).phase) and externalDischarged(s, id))
  pure def applyFinalize(s: State): State = { ...s, terminal: true, event: Finalized, lastTask: 0 }

  pure def canConfigure(s: State, id: int, actor: int): bool = and {
    not(s.terminal), actor == s.tasks.get(id).owner,
    s.tasks.get(id).phase == Queued or canEdit(s, id, actor),
  }
  pure def invalidateConfiguration(s: State, id: int): State = {
    val impacted = affected(s, id)
    val tasks = TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (impacted.contains(other) and not(Set(Cancelled, Blocked).contains(t.phase)))
        { ...reset(t), phase: Queued, version: t.version + 1, inputVersion: t.inputVersion + 1, frozen: false }
      else t
    })
    { ...s, tasks: tasks, sourceMerges: s.sourceMerges.exclude(impacted),
      fullCompletion: s.fullCompletion.exclude(impacted), event: DependenciesChanged, lastTask: id }
  }
  // A pure tail keeps the arbitrary-State API and the cached current-State
  // path on the same clauses, with membership preceding the stage-map lookup.
  pure def configurationStageAllowed(s: State, id: int, dep: int, stage: DependencyStage): bool = and {
    s.tasks.get(id).dependencies.contains(dep),
    s.tasks.get(id).dependencyStages.get(dep) != stage,
    stage != SOURCE_INTEGRATED or s.tasks.get(dep).adapter == SourceIntegration,
  }
  pure def canConfigureStage(s: State, id: int, dep: int, stage: DependencyStage, actor: int): bool =
    canConfigure(s, id, actor) and configurationStageAllowed(s, id, dep, stage)
  pure def applyConfigureStage(s: State, id: int, dep: int, stage: DependencyStage): State = {
    val updated = invalidateConfiguration(s, id)
    { ...updated, tasks: updated.tasks.setBy(id, t =>
      { ...t, dependencyStages: t.dependencyStages.set(dep, stage) }) }
  }
  pure def canRemoveDependency(s: State, id: int, dep: int, actor: int): bool =
    canConfigure(s, id, actor) and s.tasks.get(id).dependencies.contains(dep)
  pure def applyRemoveDependency(s: State, id: int, dep: int): State = {
    val updated = invalidateConfiguration(s, id)
    val t = updated.tasks.get(id)
    val remaining = t.dependencies.exclude(Set(dep))
    { ...updated, tasks: updated.tasks.set(id, { ...t, dependencies: remaining,
      dependencyStages: remaining.mapBy(other => t.dependencyStages.get(other)) }) }
  }
  pure def configurationAdapterAllowed(s: State, id: int, adapter: Adapter): bool = and {
    s.tasks.get(id).adapter != adapter,
    adapter == SourceIntegration or TASKS.forall(other =>
      not(s.tasks.get(other).dependencies.contains(id)) or s.tasks.get(other).dependencyStages.get(id) != SOURCE_INTEGRATED),
  }
  pure def canConfigureAdapter(s: State, id: int, adapter: Adapter, actor: int): bool =
    canConfigure(s, id, actor) and configurationAdapterAllowed(s, id, adapter)
  pure def applyConfigureAdapter(s: State, id: int, adapter: Adapter): State = {
    val updated = invalidateConfiguration(s, id)
    { ...updated, tasks: updated.tasks.setBy(id, t => { ...t, adapter: adapter }) }
  }
  // External input drift invalidates current source evidence but does not fake
  // edits of each descendant's own source epoch. Full transitive stamps matter.
  pure def canInputDrift(s: State, id: int): bool = not(s.terminal) and not(Set(Cancelled, Blocked).contains(s.tasks.get(id).phase))
  pure def applyInputDrift(s: State, id: int): State = {
    val impacted = affected(s, id)
    val tasks = TASKS.mapBy(other => {
      val t = s.tasks.get(other)
      if (impacted.contains(other)) { ...reset(t),
        inputVersion: if (other == id) t.inputVersion + 1 else t.inputVersion,
        phase: if (Set(Cancelled, Blocked).contains(t.phase)) t.phase else Queued,
        frozen: false,
      } else t
    })
    { ...s, tasks: tasks, sourceMerges: s.sourceMerges.exclude(impacted),
      fullCompletion: s.fullCompletion.exclude(impacted), event: Edited, lastTask: id }
  }
  pure def canArtifactDrift(s: State, id: int): bool = not(s.terminal) and observationRecordedLocal(s, id)
  pure def applyArtifactDrift(s: State, id: int): State = changeTask(s, id,
    { ...s.tasks.get(id), artifactVersion: s.tasks.get(id).artifactVersion + 1 }, Edited)
  pure def deliveryReview(observation: DeliveryObservation, reviewer: int): DeliveryReview =
    { observation: observation, reviewer: reviewer, approved: true }
  pure def deliveryAdoption(observation: DeliveryObservation, review: DeliveryReview): DeliveryAdoption =
    { observation: observation, review: review, integrator: INTEGRATOR, adopted: true }
  pure def canAcceptDelivery(s: State, id: int, observation: DeliveryObservation,
      review: DeliveryReview, adoption: DeliveryAdoption): bool = and {
    not(s.terminal), observationRecordedLocal(s, id), readyDependencies(s, id),
    observation == currentDeliveryObservation(s, id), s.deliveryObservations.contains(observation),
    review.observation == observation, review.approved,
    review.reviewer != s.tasks.get(id).owner, ELIGIBLE_REVIEWERS.get(id).contains(review.reviewer),
    adoption.observation == observation, adoption.review == review,
    adoption.integrator == INTEGRATOR, adoption.adopted,
    not(s.deliveryAcceptances.exists(a => a.observation == observation)),
  }
  pure def applyAcceptDelivery(s: State, id: int, observation: DeliveryObservation,
      review: DeliveryReview, adoption: DeliveryAdoption): State = {
    ...s, deliveryAcceptances: s.deliveryAcceptances.union(Set({ observation: observation, review: review, adoption: adoption })),
    event: DeliveryAcceptedEvent, lastTask: id,
  }
  pure def sCanConfigureStage(s: State, id: int): bool = canConfigure(s, id, s.tasks.get(id).owner) and
    s.tasks.get(id).dependencies.exists(dep => Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).exists(stage =>
      canConfigureStage(s, id, dep, stage, s.tasks.get(id).owner)))
  pure def sCanAcceptDelivery(s: State, id: int): bool = observationRecordedLocal(s, id) and
    ELIGIBLE_REVIEWERS.get(id).exists(reviewer => canAcceptDelivery(s, id, currentDeliveryObservation(s, id),
      deliveryReview(currentDeliveryObservation(s, id), reviewer),
      deliveryAdoption(currentDeliveryObservation(s, id), deliveryReview(currentDeliveryObservation(s, id), reviewer))))
  // Nonpersistent per-current-state calculation reuse. The installed TypeScript
  // evaluator caches top-level vals and clears them when current vars shift.
  // These sets contain only TASKS; no state/history field or action is added.
  val currentEditable: Set[int] = TASKS.filter(id => canEdit(state, id, state.tasks.get(id).owner))
  val currentConfigurable: Set[int] = if (state.terminal) Set() else TASKS.filter(id =>
    state.tasks.get(id).phase == Queued or currentEditable.contains(id))
  def canEditCurrent(id: int, actor: int): bool =
    actor == state.tasks.get(id).owner and currentEditable.contains(id)
  def canConfigureCurrent(id: int, actor: int): bool =
    actor == state.tasks.get(id).owner and currentConfigurable.contains(id)
  def canConfigureStageCurrent(id: int, dep: int, stage: DependencyStage, actor: int): bool =
    canConfigureCurrent(id, actor) and configurationStageAllowed(state, id, dep, stage)
  def canRemoveDependencyCurrent(id: int, dep: int, actor: int): bool =
    canConfigureCurrent(id, actor) and state.tasks.get(id).dependencies.contains(dep)
  def canConfigureAdapterCurrent(id: int, adapter: Adapter, actor: int): bool =
    canConfigureCurrent(id, actor) and configurationAdapterAllowed(state, id, adapter)
  def sCanConfigureStageCurrent(id: int): bool = canConfigureCurrent(id, state.tasks.get(id).owner) and
    state.tasks.get(id).dependencies.exists(dep => Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).exists(stage =>
      configurationStageAllowed(state, id, dep, stage)))
  action configureStage(id: int, dep: int, stage: DependencyStage, actor: int): bool = all {
    canConfigureStageCurrent(id, dep, stage, actor), state' = applyConfigureStage(state, id, dep, stage),
  }
  action removeDependency(id: int, dep: int, actor: int): bool = all {
    canRemoveDependencyCurrent(id, dep, actor), state' = applyRemoveDependency(state, id, dep),
  }
  action configureAdapter(id: int, adapter: Adapter, actor: int): bool = all {
    canConfigureAdapterCurrent(id, adapter, actor), state' = applyConfigureAdapter(state, id, adapter),
  }
  action inputDrift(id: int): bool = all { canInputDrift(state, id), state' = applyInputDrift(state, id) }
  action artifactDrift(id: int): bool = all { canArtifactDrift(state, id), state' = applyArtifactDrift(state, id) }
  action acceptDelivery(id: int, observation: DeliveryObservation, review: DeliveryReview, adoption: DeliveryAdoption): bool = all {
    canAcceptDelivery(state, id, observation, review, adoption), state' = applyAcceptDelivery(state, id, observation, review, adoption),
  }
  action init = state' = initialState
  action claim(id: int, actor: int): bool = all { canClaim(state, id, actor), state' = applyClaim(state, id) }
  action freeze(id: int, actor: int, independent: bool): bool = all {
    canFreeze(state, id, actor, independent), state' = applyFreeze(state, id),
  }
  action edit(id: int, actor: int, kind: Mutation): bool = all {
    canEditCurrent(id, actor), state' = applyEdit(state, id, kind),
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
  action dispatch(id: int): bool = all { canDispatch(state, id), state' = applyDispatch(state, id) }
  action extObserve(id: int, attempt: int, passed: bool): bool = all {
    canExtObserve(state, id, attempt), state' = applyExtObserve(state, id, passed),
  }
  action extObserveToken(id: int, token: ExtToken, passed: bool): bool = all {
    canExtObserveToken(state, id, token), state' = applyExtObserve(state, id, passed),
  }
  action extUnknown(id: int, attempt: int): bool = all {
    canExtUnknown(state, id, attempt), state' = applyExtUnknown(state, id),
  }
  action cleanup(id: int): bool = all { canCleanup(state, id), state' = applyCleanup(state, id) }
  action reconcile(id: int, original: ExtResult, evidence: TerminalEvidence): bool = all {
    canReconcile(state, id, original, evidence), state' = applyReconcile(state, id, original, evidence),
  }
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
    val editable = currentEditable
    val finishable = TASKS.filter(id => canFinishCurrent(state, id))
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
    { nondet id = finishable.filter(id => state.tasks.get(id).stage == Checker).oneOf()
      nondet observation = Set(CompleteMatch, Incomplete, IdentityMismatch, StateMismatch).oneOf()
      checker(id, jobToken(state, id), observation) },
    { nondet id = finishable.filter(id => state.tasks.get(id).stage == Regressions).oneOf()
      nondet passed = Set(true, false).oneOf()
      regressions(id, jobToken(state, id), passed) },
    { nondet id = finishable.oneOf()
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
    { nondet id = TASKS.filter(id => canDispatch(state, id)).oneOf()
      dispatch(id) },
    { nondet id = TASKS.filter(id => canExtObserve(state, id, state.tasks.get(id).extAttempt)).oneOf()
      nondet passed = Set(true, false).oneOf()
      extObserve(id, state.tasks.get(id).extAttempt, passed) },
    { nondet id = TASKS.filter(id => canExtUnknown(state, id, state.tasks.get(id).extAttempt)).oneOf()
      extUnknown(id, state.tasks.get(id).extAttempt) },
    { nondet id = TASKS.filter(id => canCleanup(state, id)).oneOf()
      cleanup(id) },
    { nondet id = TASKS.filter(id => canReconcileCurrent(state, id)).oneOf()
      nondet kind = Set(ProcessNotStarted, ProcessExited, TxIncluded).oneOf()
      reconcile(id, externalResult(state, id), terminalEvidence(externalToken(state, id), kind)) },
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
    { nondet id = TASKS.filter(id => sCanConfigureStageCurrent(id)).oneOf()
      nondet dep = state.tasks.get(id).dependencies.filter(dep =>
        Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).exists(stage =>
          canConfigureStageCurrent(id, dep, stage, state.tasks.get(id).owner))).oneOf()
      nondet stage = Set(SOURCE_INTEGRATED, NATIVE_OBSERVED, NATIVE_ACCEPTED).filter(stage =>
        canConfigureStageCurrent(id, dep, stage, state.tasks.get(id).owner)).oneOf()
      configureStage(id, dep, stage, state.tasks.get(id).owner) },
    { nondet id = TASKS.filter(id => canConfigureCurrent(id, state.tasks.get(id).owner) and
        state.tasks.get(id).dependencies != Set()).oneOf()
      nondet dep = state.tasks.get(id).dependencies.oneOf()
      removeDependency(id, dep, state.tasks.get(id).owner) },
    { nondet id = TASKS.filter(id => Set(SourceIntegration, NativeAdapter).exists(adapter =>
        canConfigureAdapterCurrent(id, adapter, state.tasks.get(id).owner))).oneOf()
      nondet adapter = Set(SourceIntegration, NativeAdapter).filter(adapter =>
        canConfigureAdapterCurrent(id, adapter, state.tasks.get(id).owner)).oneOf()
      configureAdapter(id, adapter, state.tasks.get(id).owner) },
    { nondet id = TASKS.filter(id => canInputDrift(state, id)).oneOf()
      inputDrift(id) },
    { nondet id = TASKS.filter(id => canArtifactDrift(state, id)).oneOf()
      artifactDrift(id) },
    { nondet id = TASKS.filter(id => sCanAcceptDelivery(state, id)).oneOf()
      nondet reviewer = ELIGIBLE_REVIEWERS.get(id).filter(reviewer =>
        canAcceptDelivery(state, id, currentDeliveryObservation(state, id),
          deliveryReview(currentDeliveryObservation(state, id), reviewer),
          deliveryAdoption(currentDeliveryObservation(state, id), deliveryReview(currentDeliveryObservation(state, id), reviewer)))).oneOf()
      val observation = currentDeliveryObservation(state, id)
      val review = deliveryReview(observation, reviewer)
      acceptDelivery(id, observation, review, deliveryAdoption(observation, review)) },
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
  // Disposed source tasks with pending external duties still have a discharge
  // transition. Terminal workflow state itself is separately checked below.
  val noPrematureStop = state.terminal or canFinalize(state) or TASKS.exists(id =>
    canCancel(state, id, state.tasks.get(id).owner) or
    canExtUnknown(state, id, state.tasks.get(id).extAttempt) or canCleanup(state, id) or
    canReconcileCurrent(state, id))
  val dispatched = state.event == DispatchedEvent
  val extObserved = state.event == ExtObserved
  val extUnknownWitness = state.event == ExtUnknownEvent
  val cleaned = state.event == CleanedEvent
  val reconciled = state.event == ReconciledEvent
  val reconciledNotStarted = reconciled and state.extResolutions.exists(x =>
    x.original.token == externalToken(state, if (state.lastTask == 0) 1 else state.lastTask) and x.evidence.kind == ProcessNotStarted)
  val reconciledExited = reconciled and state.extResolutions.exists(x =>
    x.original.token == externalToken(state, if (state.lastTask == 0) 1 else state.lastTask) and x.evidence.kind == ProcessExited)
  val reconciledIncluded = reconciled and state.extResolutions.exists(x =>
    x.original.token == externalToken(state, if (state.lastTask == 0) 1 else state.lastTask) and x.evidence.kind == TxIncluded)
  val lateResultRetained = extObserved and not(Set(Candidate, Merged).contains(state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).phase))
  // Consumers may only use an imported pass while it is still current;
  // a later edit stales it via extPrint without rewriting history.
  pure def extUsable(s: State, id: int): bool =
    s.tasks.get(id).ext == ExtPassed and extCurrent(s, id) and
    (Set(Candidate, Merged).contains(s.tasks.get(id).phase) or
      (s.tasks.get(id).adapter == NativeAdapter and s.tasks.get(id).phase == Working)) and
    s.tasks.get(id).extManifest == s.tasks.get(id).artifactVersion and
    externalDischarged(s, id) and not(s.tasks.get(id).cleanupOwed)
  val dispatchSound = TASKS.forall(id => state.tasks.get(id).ext == NotDispatched or
    (state.tasks.get(id).extAttempt >= 1 and state.tasks.get(id).extPrint != emptyFingerprint))
  val cleanupSound = TASKS.forall(id => state.tasks.get(id).ext != Dispatched or state.tasks.get(id).cleanupOwed)
  // Every consumed attempt is retained; results and resolutions bind its full
  // identity. A resolution can never replace/mutate the original unknown row.
  pure def externalHistorySoundFor(s: State): bool =
    // With all four histories empty, every quantified history clause is true;
    // the original dispatch-count clause is exactly extAttempt == 0 per task.
    if (s.extDispatches == Set() and s.extResults == Set() and
        s.extCleanups == Set() and s.extResolutions == Set())
      TASKS.forall(id => s.tasks.get(id).extAttempt == 0)
    else and {
    s.extResults.fold(true, (ok, r) => ok and (s.extDispatches.contains(r.token) and
      Set(ExtPassed, ExtFailed, ExtUnknown).contains(r.outcome))),
    s.extResults.fold(true, (ok, r) => ok and s.extResults.fold(true, (unique, other) =>
      unique and (r.token != other.token or r == other))),
    s.extCleanups.fold(true, (ok, token) => ok and
      s.extResults.fold(false, (found, r) => found or r.token == token)),
    s.extResolutions.fold(true, (ok, x) => ok and (s.extResults.contains(x.original) and
      x.original.outcome == ExtUnknown and x.evidence.token == x.original.token and
      x.evidence.terminal and x.evidence.cleanupComplete and s.extCleanups.contains(x.original.token))),
    s.extResolutions.fold(true, (ok, x) => ok and s.extResolutions.fold(true, (unique, other) =>
      unique and (x.original.token != other.original.token or x == other))),
    TASKS.forall(id => s.extDispatches.filter(token => token.task == id).size() == s.tasks.get(id).extAttempt),
    s.extDispatches.forall(token => token.attempt == s.tasks.get(token.task).extAttempt or
      (s.extCleanups.contains(token) and s.extResults.exists(r => r.token == token and resultResolved(s, r)))),
    s.extDispatches.forall(token => token.task == token.fingerprint.task and token.attempt >= 1 and
      token.attempt <= s.tasks.get(token.task).extAttempt),
    s.extDispatches.fold(true, (ok, token) => ok and s.extDispatches.fold(true, (unique, other) =>
      unique and (token.task != other.task or token == other or
      (token.attempt != other.attempt and token.fingerprint != other.fingerprint)))),
    TASKS.forall(id => {
      val t = s.tasks.get(id)
      val token = externalToken(s, id)
      t.extAttempt == 0 or (s.extDispatches.contains(token) and
        (t.ext == Dispatched or s.extResults.contains({ token: token, outcome: t.ext, manifest: t.extManifest })))
    }),
  }
  val externalHistorySound = externalHistorySoundFor(state)
  val cleanupAccounting = TASKS.forall(id => state.tasks.get(id).extAttempt == 0 or
    (state.tasks.get(id).cleanupOwed iff not(state.extCleanups.contains(externalToken(state, id)))))
  val usableDisposition = TASKS.forall(id => extUsable(state, id) implies
    ((Set(Candidate, Merged).contains(state.tasks.get(id).phase) or
      (state.tasks.get(id).adapter == NativeAdapter and state.tasks.get(id).phase == Working)) and
      externalDischarged(state, id)))
  val terminalDischarged = not(state.terminal) or TASKS.forall(id => externalDischarged(state, id))
  // Unresolved original rows prevent both finalization and any next dispatch,
  // including a changed fingerprint. Cleanup alone cannot lift that block.
  val unknownSound = state.extResults.forall(r =>
    (r.outcome == ExtUnknown and not(resultResolved(state, r))) implies
      (not(state.terminal) and not(canDispatch(state, r.token.task))))
  val sourceAcceptedWitness = state.event == DeliveryAcceptedEvent and
    state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).adapter == SourceIntegration
  val nativeAcceptedWitness = state.event == DeliveryAcceptedEvent and
    state.tasks.get(if (state.lastTask == 0) 1 else state.lastTask).adapter == NativeAdapter
  val nativeObservedWitness = state.event == CleanedEvent and
    deliveryObservedRecorded(state, if (state.lastTask == 0) 1 else state.lastTask)
  val dependencySchema = TASKS.forall(id =>
    state.tasks.get(id).dependencyStages.keys() == state.tasks.get(id).dependencies and
    state.tasks.get(id).dependencies.subseteq(TASKS.exclude(Set(id))) and
    not(dependencyAncestors(state, id).contains(id)) and
    state.tasks.get(id).dependencies.forall(dep =>
      state.tasks.get(id).dependencyStages.get(dep) != SOURCE_INTEGRATED or state.tasks.get(dep).adapter == SourceIntegration))
  val operationTokens = state.extDispatches.union(state.deliveryObservations.map(o => o.token))
  val uniqueDeliveryAttempts = and {
    TASKS.forall(id => operationTokens.filter(token => token.task == id).size() == state.tasks.get(id).deliveryAttempt),
    operationTokens.forall(token => token.operationAttempt >= 1 and
      token.operationAttempt <= state.tasks.get(token.task).deliveryAttempt),
    operationTokens.forall(token => operationTokens.forall(other =>
      token.task != other.task or token.operationAttempt != other.operationAttempt or token == other)),
  }
  val deliveryObservationHistory = state.deliveryObservations.forall(o => and {
    o.token.task == o.token.fingerprint.task, o.adapter == o.token.fingerprint.adapter, o.manifest >= 0,
    if (o.adapter == SourceIntegration) state.receipts.exists(r =>
      r.task == o.token.task and r.fingerprint == o.token.fingerprint and r.attempt == o.token.attempt and
      r.tier == Ordinary and r.result == Passed)
    else state.extResults.exists(r => r.token == o.token and r.outcome == ExtPassed and r.manifest == o.manifest),
  })
  val deliveryAcceptanceHistory = state.deliveryAcceptances.forall(a => and {
    state.deliveryObservations.contains(a.observation), a.review.observation == a.observation, a.review.approved,
    a.review.reviewer != state.tasks.get(a.observation.token.task).owner,
    ELIGIBLE_REVIEWERS.get(a.observation.token.task).contains(a.review.reviewer),
    a.adoption.observation == a.observation, a.adoption.review == a.review,
    a.adoption.integrator == INTEGRATOR, a.adoption.adopted,
  })
  val noStageSubstitution = TASKS.forall(id => and {
    deliveryObservedRecorded(state, id) implies state.tasks.get(id).adapter == NativeAdapter,
    deliverySourceRecorded(state, id) implies state.tasks.get(id).adapter == SourceIntegration,
    deliveryAcceptedRecorded(state, id) implies (observationRecordedLocal(state, id) and
      state.deliveryAcceptances.exists(a => a.observation == currentDeliveryObservation(state, id))),
  })
  val workflowInvariant = noPrematureStop and ownership and independentOracle and localOnly and noEscalation and exactReviews and exactMerges and observedCompletion and dispatchSound and cleanupSound and unknownSound and externalHistorySound and cleanupAccounting and usableDisposition and terminalDischarged and dependencySchema and deliveryObservationHistory and deliveryAcceptanceHistory and noStageSubstitution and uniqueDeliveryAttempts

}

module doctorwatson_small {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ).*
}
