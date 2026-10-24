// SOURCE ONLY. All Quint parse/type/effect checks, tests and execution UNRUN.
// Positive one-row projection: b.helper.Scalar.tag, RetRight/Add/First/UInt.
// Model-domain premises are not new interpreter guards or adoption decisions.
module bScalarTagProjection {
  type Space = ConstSpace | DataSpace
  type ValueRef = {space: Space, index: int}
  type OptRef = NoRef | SomeRef(ValueRef)
  type OptInt = Absent | Present(int)
  type UIntRow = {n: int} // UInt tag is implicit in this positive table projection.
  type ScalarPhase = First | Second | Done
  type ScalarMode = Add | Sub | Mul | Lt | Cons | Branch | Leaf | Not | TestBool | TestType
  type Site = RetRight
  type Sort = UIntSort | BoolSort | ListSort | TreeSort
  type Expected = NoExpected | ExpectedSort(Sort)
  type Staged = NoStaged | UIntStaged(int) | BoolStaged(bool) | PrimError(int)
  type ScalarHelper = {present: bool, kind: str, site: Site,
    originReferenceCursor: int, parentResume: OptInt, mode: ScalarMode,
    operandA: ValueRef, operandB: OptRef, expected: Expected,
    phase: ScalarPhase, regA: int, staged: Staged}
  type Counters = {remainingWork: int, consumedWork: int, helperWork: int, helperVisits: int}
  type Control = Return(ValueRef)
  type Outcome = Running
  type Result = NoResult
  type RightFrame = {op: ScalarMode, left: ValueRef, origin: int}
  type EnvCell = {binder: int, value: ValueRef, previous: OptInt}
  type Alloc = {dataNodes: int, dataFields: int, environmentCells: int}
  type Active = {data: int, dataFields: int, env: int, valueSlots: int}
  type SegBase = {active: Active, alloc: Alloc, consumedWork: int}
  type Global = {control: Control, controlOrigin: int, envRoot: OptInt,
    depth: int, stackLimit: int, outcome: Outcome, initialFuel: int, referenceCursor: int,
    alloc: Alloc, active: Active, taskLive: int, result: Result, segBase: SegBase,
    data: List[UIntRow], dataFields: List[ValueRef], env: List[EnvCell],
    valueSlots: List[ValueRef], frames: List[RightFrame], tasks: List[int]}
  type Machine = {helper: ScalarHelper, counters: Counters, global: Global}
  type Program = {constants: List[UIntRow], sourceOccurrence: List[int], sourceOwner: str}
  // Read-domain capacity projection, supplied by the finite input instance.
  // These fields do not purport to be all of Bounds or LayoutCapacities.
  type Limits = {maxReferenceWork: int, maxHelperWork: int, constantNodes: int,
    dataNodes: int, dataFieldSlots: int, environmentCells: int, frames: int,
    helperTasks: int, valueSlots: int}
  type Input = {caseId: str, operandA: ValueRef, payload: int, counters: Counters,
    referenceCursor: int}
  type Observation = {ruleId: str, eventKind: str, disposition: str,
    delta: List[int], helperPhaseBefore: ScalarPhase, helperPhaseAfter: ScalarPhase,
    helperCursorBefore: int, helperCursorAfter: int, originReferenceCursor: int,
    logicalDepthBefore: int, logicalDepthAfter: int, taskCursorBefore: int, taskCursorAfter: int}
  type LastObservation = NoObservation | Observed(Observation)
  type State = {input: Input, program: Program, limits: Limits,
    pre: Machine, image: Machine, applied: bool, observation: LastObservation}
  var state: State

  // Mathematical integers retain all 128 payload bits. No u64, floating-point,
  // string-valued payload, modulo reduction, field packing or limb selection.
  pure val maxUInt128: int = 340282366920938463463374607431768211455
  pure val above64: int = 18446744073709551623
  pure val above96: int = 79228162514264337593543950355
  pure val ordinaryCounters: Counters = {remainingWork: 5, consumedWork: 12,
    helperWork: 9, helperVisits: 3}
  pure val wideCounters: Counters = {remainingWork: 18446744073709551625,
    consumedWork: 18446744073709551630, helperWork: 18446744073709551627,
    helperVisits: 18446744073709551623}
  pure def ref(space: Space, index: int): ValueRef = {space: space, index: index}
  pure def input(id: str, space: Space, index: int, payload: int, cc: Counters): Input = {
    caseId: id, operandA: ref(space, index), payload: payload,
    counters: cc, referenceCursor: 3}
  // Input-only normal samples; no O expected post-state or producer is used.
  pure val zeroConst = input("const-zero-index0", ConstSpace, 0, 0, ordinaryCounters)
  pure val smallData = input("data-seven-index1", DataSpace, 1, 7, ordinaryCounters)
  pure val wideConst = input("const-above64-index1", ConstSpace, 1, above64, ordinaryCounters)
  pure val wideData = input("data-above96-index0", DataSpace, 0, above96, ordinaryCounters)
  pure val fullUInt = input("const-full-u128-index0", ConstSpace, 0, maxUInt128, ordinaryCounters)
  pure val wideWork = input("data-above64-work-index1", DataSpace, 1, above64, wideCounters)
  pure val cases: Set[Input] = Set(zeroConst, smallData, wideConst, wideData, fullUInt, wideWork)
  pure def u128(nn: int): bool = nn >= 0 and nn <= maxUInt128
  pure def table(ii: Input, space: Space): List[UIntRow] =
    if (ii.operandA.space != space) List({n: 23}, {n: 29})
    else if (ii.operandA.index == 0) List({n: ii.payload}, {n: 31})
    else List({n: 37}, {n: ii.payload})
  pure def inReadBounds(pp: Program, mm: Machine, rr: ValueRef): bool =
    rr.index >= 0 and (if (rr.space == ConstSpace) rr.index < pp.constants.length()
      else rr.index < mm.global.active.data and rr.index < mm.global.data.length())
  pure def selectedUInt(pp: Program, mm: Machine): int = {
    val rr = mm.helper.operandA
    if (rr.space == ConstSpace) pp.constants.nth(rr.index).n else mm.global.data.nth(rr.index).n
  }
  pure def limitInput(ii: Input): Limits = {
    maxReferenceWork: ii.counters.remainingWork + ii.counters.consumedWork + 8,
    maxHelperWork: ii.counters.helperWork + 8, constantNodes: 2,
    dataNodes: 2, dataFieldSlots: 0, environmentCells: 1, frames: 2, helperTasks: 0, valueSlots: 2}
  pure def initial(ii: Input): State = {
    val aa = ii.operandA
    val bb = ref(ConstSpace, 0)
    val mm: Machine = {
      helper: {present: true, kind: "Scalar", site: RetRight,
        originReferenceCursor: ii.referenceCursor, parentResume: Absent,
        mode: Add, operandA: aa, operandB: SomeRef(bb), expected: NoExpected,
        phase: First, regA: 0, staged: NoStaged},
      counters: ii.counters,
      global: {control: Return(bb), controlOrigin: 11, envRoot: Present(0),
        depth: 1, stackLimit: 2, outcome: Running,
        initialFuel: ii.counters.remainingWork + ii.counters.consumedWork,
        referenceCursor: ii.referenceCursor,
        alloc: {dataNodes: 2, dataFields: 0, environmentCells: 1},
        active: {data: 2, dataFields: 0, env: 1, valueSlots: 2},
        taskLive: 0, result: NoResult,
        segBase: {active: {data: 0, dataFields: 0, env: 0, valueSlots: 0},
          alloc: {dataNodes: 0, dataFields: 0, environmentCells: 0}, consumedWork: 0},
        data: table(ii, DataSpace), dataFields: List(),
        env: List({binder: 41, value: bb, previous: Absent}),
        valueSlots: List(bb, ref(DataSpace, 0)),
        frames: List({op: Add, left: aa, origin: 7}), tasks: List()}}
    {input: ii, program: {constants: table(ii, ConstSpace), sourceOccurrence: List(1), sourceOwner: "Entry"},
      limits: limitInput(ii), pre: mm, image: mm, applied: false, observation: NoObservation}
  }
  // Positive-domain premises only. AB, whole StepInv, constrained dispatch,
  // exactly-one row and program/image/statement binding remain external.
  pure def positivePre(pp: Program, mm: Machine, ll: Limits): bool = and {
    mm.helper.present, mm.helper.kind == "Scalar", mm.helper.site == RetRight,
    mm.helper.mode == Add, mm.helper.phase == First,
    mm.helper.regA == 0, mm.helper.staged == NoStaged,
    mm.helper.expected == NoExpected, mm.helper.parentResume == Absent,
    mm.helper.originReferenceCursor == mm.global.referenceCursor,
    mm.global.outcome == Running, mm.global.result == NoResult,
    mm.global.taskLive == 0, mm.global.tasks == List(),
    mm.global.depth == 1, mm.global.frames == List({op: Add, left: mm.helper.operandA, origin: 7}),
    match mm.global.control { | Return(bb) => mm.helper.operandB == SomeRef(bb) },
    inReadBounds(pp, mm, mm.helper.operandA),
    pp.constants.indices().forall(ix => u128(pp.constants.nth(ix).n)),
    mm.global.data.indices().forall(ix => u128(mm.global.data.nth(ix).n)),
    mm.counters.remainingWork >= 1, u128(mm.counters.remainingWork),
    mm.counters.consumedWork >= 0, mm.counters.consumedWork < maxUInt128,
    mm.counters.helperWork >= 0, mm.counters.helperWork < ll.maxHelperWork,
    mm.counters.helperWork < maxUInt128,
    mm.counters.helperVisits >= 0, mm.counters.helperVisits < maxUInt128,
    mm.counters.helperVisits <= mm.counters.helperWork,
    mm.counters.remainingWork + mm.counters.consumedWork == mm.global.initialFuel,
    mm.global.initialFuel <= ll.maxReferenceWork, u128(mm.global.initialFuel),
    mm.counters.consumedWork == mm.counters.helperWork + mm.global.referenceCursor,
  }
  pure def applyTag(pp: Program, pre: Machine): Machine = {...pre,
    helper: {...pre.helper, regA: selectedUInt(pp, pre), phase: Second},
    counters: {remainingWork: pre.counters.remainingWork - 1,
      consumedWork: pre.counters.consumedWork + 1,
      helperWork: pre.counters.helperWork + 1,
      helperVisits: pre.counters.helperVisits + 1}}
  // Named relation, independent of applyTag: no opaque 'rest' equality hides a
  // changed helper field. Global equality preserves every named global field.
  pure def rowRelation(pp: Program, pre: Machine, post: Machine): bool = and {
    post.helper.present == pre.helper.present, post.helper.kind == pre.helper.kind,
    post.helper.site == pre.helper.site,
    post.helper.originReferenceCursor == pre.helper.originReferenceCursor,
    post.helper.parentResume == pre.helper.parentResume, post.helper.mode == pre.helper.mode,
    post.helper.operandA == pre.helper.operandA, post.helper.operandB == pre.helper.operandB,
    post.helper.expected == pre.helper.expected, post.helper.staged == pre.helper.staged,
    post.helper.regA == selectedUInt(pp, pre), post.helper.phase == Second,
    post.counters.remainingWork == pre.counters.remainingWork - 1,
    post.counters.consumedWork == pre.counters.consumedWork + 1,
    post.counters.helperWork == pre.counters.helperWork + 1,
    post.counters.helperVisits == pre.counters.helperVisits + 1,
    post.global == pre.global,
  }
  pure def observe(pre: Machine, post: Machine): Observation = {
    ruleId: "b.helper.Scalar.tag", eventKind: "HelperVisit", disposition: "Applied",
    delta: List(post.counters.remainingWork - pre.counters.remainingWork,
      post.counters.consumedWork - pre.counters.consumedWork,
      post.counters.helperWork - pre.counters.helperWork,
      post.counters.helperVisits - pre.counters.helperVisits,
      post.global.referenceCursor - pre.global.referenceCursor,
      post.global.alloc.dataNodes - pre.global.alloc.dataNodes,
      post.global.alloc.dataFields - pre.global.alloc.dataFields,
      post.global.alloc.environmentCells - pre.global.alloc.environmentCells,
      post.global.depth - pre.global.depth, post.global.taskLive - pre.global.taskLive,
      post.global.active.valueSlots - pre.global.active.valueSlots),
    helperPhaseBefore: pre.helper.phase, helperPhaseAfter: post.helper.phase,
    helperCursorBefore: 0, helperCursorAfter: 1,
    originReferenceCursor: pre.helper.originReferenceCursor,
    logicalDepthBefore: pre.global.depth, logicalDepthAfter: post.global.depth,
    taskCursorBefore: pre.global.taskLive, taskCursorAfter: post.global.taskLive}
  action initCase(ii: Input) = all {cases.contains(ii), state' = initial(ii)}
  action init = {
    nondet ii = cases.oneOf()
    initCase(ii)
  }
  // Exactly one modeled row; stopping at Second is an explicit comparison cut.
  action tag = all {
    not(state.applied), positivePre(state.program, state.image, state.limits),
    state' = {...state, image: applyTag(state.program, state.image), applied: true,
      observation: Observed(observe(state.image, applyTag(state.program, state.image)))},
  }
  action stutter = all {state.applied, state' = state}
  action step = any {tag, stutter}
  val preservesNamedState = state.image.global == state.pre.global
  val positiveRowRelation = if (state.applied) rowRelation(state.program, state.pre, state.image)
    else state.image == state.pre
  val fullWidthPayload = if (state.applied) u128(state.image.helper.regA) else true
  val reachedSecond = state.applied and state.image.helper.phase == Second
}
