// SOURCE ONLY: parse/type/effect checks, tests, simulation and verification UNRUN.
// Source projection of adopted HOST DedicatedSorts Validate + literal completion.
// Relative component cutpoint, not admitRun, not full B, not StepInv proof.
module bValidateProjection {
  type OptInt = Absent | Present(int)
  type ValueRef = Const(int) | Data(int)
  type Kind = UIntKind | BoolKind | NilKind | ConsKind | LeafKind | BranchKind | MarkerKind
  type Node = {kind: Kind, number: int, boolean: bool, fieldsStart: int, markerFree: bool}
  type Expr = {constant: int}
  type Program = {expressions: List[Expr], constants: List[Node], constantFields: List[int]}
  type Occurrence = {owner: str, functionIndex: OptInt, astPath: List[int]}
  type Identity = {profileId: str, grammarDomain: str, finance: str, layoutVersion: str,
    layoutDigest: str, boundsDigest: str, fieldId: OptInt, programMembershipDigest: str}
  type RunId = {identity: Identity, fuelInput: int, stackLimitInput: int}
  type Bounds = {codeRows: int, functionRows: int, codeChildSlots: int, parameterSlots: int,
    patternRows: int, matchPlanRows: int, matchArmRows: int, typeRows: int, constructorRows: int,
    constructorFieldSlots: int, maxArity: int, constantNodes: int, constantFieldSlots: int,
    dataNodes: int, dataFieldSlots: int, environmentCells: int, frames: int, helperTasks: int,
    serializationBytes: int, maxReferenceWork: int, maxHelperWork: int, stepCap: int}
  type LayoutCapacities = {nameRows: int, nameBytes: int, occurrencePathSlots: int, valueSlots: int}
  type Limits = {initialFuel: int, stackLimit: int, bounds: Bounds, capacities: LayoutCapacities}
  type EnvCell = {binder: int, value: ValueRef, previous: OptInt}
  // Only empty frames are admitted at this top-level literal cutpoint. Frame rows
  // are retained as a distinct typed arena; no continuation transition is modeled.
  type Frame = {savedEnv: OptInt, origin: int}
  type TaskVisit = Unvisited | Fields
  type Task = {ref: ValueRef, visit: TaskVisit, cursor: int}
  type Status = Pending | Positive | Negative
  type Validate = {site: str, originReferenceCursor: int, parentResume: OptInt,
    root: ValueRef, status: Status}
  type Helper = NoHelper | ValidateHelper(Validate)
  type Result = NoResult | Validated(bool)
  type Control = Eval(int) | Return(ValueRef) | Error(int)
  type Reason = Fuel | HelperWork | HelperTasks
  type Outcome = Running | Halted(ValueRef) | Rejected(int) | Exhausted(Reason)
  type Alloc = {dataNodes: int, dataFields: int, environmentCells: int}
  type Active = {data: int, dataFields: int, env: int, valueSlots: int}
  type SegBase = {active: Active, alloc: Alloc, consumedWork: int}
  type Machine = {control: Control, controlOrigin: int, envRoot: OptInt, depth: int,
    stackLimit: int, outcome: Outcome, initialFuel: int, remainingWork: int,
    consumedWork: int, helperWork: int, helperVisits: int, referenceCursor: int,
    alloc: Alloc, segBase: SegBase, helper: Helper, result: Result,
    data: List[Node], dataFields: List[ValueRef], env: List[EnvCell],
    valueSlots: List[ValueRef], frames: List[Frame], tasks: List[Task]}
  type EventKind = HelperEnter | HelperVisit | HelperExit | HelperFail | Bookkeeping | LegacyComplete | Publish
  type Disposition = Applied | Published | ExhaustedFuelUncharged | RefusedHelperWorkUncharged | RefusedCapCharged
  type OptStatus = NoPhase | Phase(Status)
  type HelperKind = NoHelperKind | ValidateKind
  type ResultTag = NoResultTag | ValidatedTag
  type ExhaustedKind = NoExhaustedKind | HasExhaustedKind(Reason)
  type Boundary = NoBoundary | Terminal
  type OutcomeTag = RunningTag | HaltedTag | RejectedTag | ExhaustedTag
  type Event = {stepId: {runId: RunId, ordinal: int}, ruleId: str, eventKind: EventKind,
    site: str, helperKind: HelperKind, disposition: Disposition, workDelta: int,
    helperWorkDelta: int, helperVisitsDelta: int, referenceCursorDelta: int,
    allocDeltas: Alloc, valueSlotWrites: int, logicalDepthBefore: int, logicalDepthAfter: int,
    taskCursorBefore: int, taskCursorAfter: int, helperPhaseBefore: OptStatus,
    helperPhaseAfter: OptStatus, helperCursorBefore: OptInt, helperCursorAfter: OptInt,
    resultTag: ResultTag, originReferenceCursor: OptInt, sourceExprId: int,
    sourceOccurrence: Occurrence, exhaustedKind: ExhaustedKind,
    outcomeTag: OutcomeTag, outcomePayload: Outcome, boundary: Boundary}
  type LastEvent = NoEvent | StepEvent(Event)
  type Input = {caseId: str, program: Program, expr: int, occurrence: Occurrence,
    envRoot: OptInt, data: List[Node], dataFields: List[ValueRef], env: List[EnvCell],
    valueSlots: List[ValueRef]}
  type State = {input: Input, limits: Limits, runId: RunId, machine: Machine, lastEvent: LastEvent}
  var state: State

  pure val zeroAlloc: Alloc = {dataNodes: 0, dataFields: 0, environmentCells: 0}
  pure val zeroActive: Active = {data: 0, dataFields: 0, env: 0, valueSlots: 0}
  pure val identity: Identity = {profileId: "sherlock-helper-microstep/1", grammarDomain: "DedicatedSorts",
    finance: "Absent", layoutVersion: "model-only-symbolic-layout", layoutDigest: "Unselected",
    boundsDigest: "Unselected", fieldId: Absent, programMembershipDigest: "Unselected"}
  pure def arity(kind: Kind): int = match kind {
    | ConsKind => 2 | LeafKind => 1 | BranchKind => 2
    | UIntKind => 0 | BoolKind => 0 | NilKind => 0
    | MarkerKind => 0 // Off-domain: no selected row ever calls arity on Marker.
  }
  pure def node(kind: Kind, number: int, boolean: bool, fieldsStart: int): Node =
    {kind: kind, number: number, boolean: boolean, fieldsStart: fieldsStart, markerFree: true}
  pure val u0 = node(UIntKind, 0, false, 0)
  pure val u7 = node(UIntKind, 7, false, 0)
  pure val btrue = node(BoolKind, 0, true, 0)
  pure val nil = node(NilKind, 0, false, 0)
  pure val leaf = node(LeafKind, 0, false, 0)
  pure val cons = node(ConsKind, 0, false, 0)
  pure val branchAt1 = node(BranchKind, 0, false, 1)
  pure def literalInput(id: str, constants: List[Node], fields: List[int], root: int): Input = {
    caseId: id, program: {expressions: List({constant: root}), constants: constants, constantFields: fields},
    expr: 0, occurrence: {owner: "Entry", functionIndex: Absent, astPath: List()},
    envRoot: Present(0), data: List(u0), dataFields: List(),
    env: List({binder: 0, value: Const(0), previous: Absent}), valueSlots: List(Const(0))}
  // Independently declared normal programs. IDs are explicit table indices; the
  // repeated leaf ref in the last program must be visited twice, never memoized.
  pure val uintInput = literalInput("uint-seven", List(u7), List(), 0)
  pure val boolInput = literalInput("boolean-true", List(btrue), List(), 0)
  pure val nilInput = literalInput("empty-list", List(nil), List(), 0)
  pure val leafInput = literalInput("one-leaf", List(u7, leaf), List(0), 1)
  pure val listInput = literalInput("one-cons", List(u7, nil, cons), List(0, 1), 2)
  pure val repeatedInput = literalInput("branch-repeated-leaf", List(u7, leaf, branchAt1), List(0, 1, 1), 2)
  pure val cases: Set[Input] = Set(uintInput, boolInput, nilInput, leafInput, listInput, repeatedInput)
  // Caller supplies capacities. These finite exploration domains are model-only;
  // they neither amend Bounds nor impose new public/runtime caps.
  pure val fuelDomain: Set[int] = 0.to(48)
  pure val helperBudgetDomain: Set[int] = 0.to(48)
  pure val taskCapacityDomain: Set[int] = 0.to(4)
  pure def limits(fuel: int, helperBudget: int, tasks: int): Limits = {
    initialFuel: fuel, stackLimit: 0,
    bounds: {codeRows: 1, functionRows: 0, codeChildSlots: 0, parameterSlots: 0,
      patternRows: 0, matchPlanRows: 0, matchArmRows: 0, typeRows: 0, constructorRows: 0,
      constructorFieldSlots: 0, maxArity: 2, constantNodes: 3, constantFieldSlots: 3,
      dataNodes: 1, dataFieldSlots: 0, environmentCells: 1, frames: 0, helperTasks: tasks,
      serializationBytes: 4096, maxReferenceWork: 48, maxHelperWork: helperBudget, stepCap: 1},
    capacities: {nameRows: 1, nameBytes: 1, occurrencePathSlots: 0, valueSlots: 48}}
  pure val normalLimits = limits(48, 48, 4)
  pure def active(mm: Machine): Active = {data: mm.data.length(), dataFields: mm.dataFields.length(),
    env: mm.env.length(), valueSlots: mm.valueSlots.length()}
  pure def root(ii: Input): ValueRef = Const(ii.program.expressions.nth(ii.expr).constant)
  pure def initial(ii: Input, ll: Limits): State = {
    input: ii, limits: ll, runId: {identity: identity, fuelInput: ll.initialFuel, stackLimitInput: ll.stackLimit},
    lastEvent: NoEvent, machine: {control: Eval(ii.expr), controlOrigin: ii.expr, envRoot: ii.envRoot,
      depth: 0, stackLimit: ll.stackLimit, outcome: Running, initialFuel: ll.initialFuel,
      remainingWork: ll.initialFuel, consumedWork: 0, helperWork: 0, helperVisits: 0,
      referenceCursor: 0, alloc: {dataNodes: ii.data.length(), dataFields: ii.dataFields.length(),
        environmentCells: ii.env.length()}, segBase: {active: zeroActive, alloc: zeroAlloc, consumedWork: 0},
      helper: NoHelper, result: NoResult, data: ii.data, dataFields: ii.dataFields, env: ii.env,
      valueSlots: ii.valueSlots, frames: List(), tasks: List()}}
  pure def rowAt(ii: Input, ref: ValueRef): Node = match ref {
    | Const(ix) => ii.program.constants.nth(ix)
    | Data(_) => u0 // Off-domain: Validate roots/fields are Const-only.
  }
  pure def top(mm: Machine): Task = mm.tasks.nth(mm.tasks.length() - 1)
  pure def pop(mm: Machine): List[Task] = mm.tasks.slice(0, mm.tasks.length() - 1)
  pure def rewrite(mm: Machine, tt: Task): List[Task] = pop(mm).append(tt)
  pure def phase(hh: Helper): OptStatus = match hh {
    | NoHelper => NoPhase | ValidateHelper(hh) => Phase(hh.status)
  }
  pure def cursor(mm: Machine): OptInt = match mm.helper {
    | NoHelper => Absent | ValidateHelper(_) => Present(mm.tasks.length())
  }
  pure def isFinal(mm: Machine): bool = match mm.control {
    | Return(_) => mm.depth == 0 | Error(_) => true | Eval(_) => false
  }
  pure def publishOutcome(mm: Machine): Outcome = match mm.control {
    | Return(rr) => Halted(rr) | Error(code) => Rejected(code) | Eval(_) => Running
  }
  pure def outcomeTag(oo: Outcome): OutcomeTag = match oo {
    | Running => RunningTag | Halted(_) => HaltedTag | Rejected(_) => RejectedTag | Exhausted(_) => ExhaustedTag
  }
  pure val helperRows = Set("b.helper.Validate.entry", "b.helper.Validate.occurrence", "b.helper.Validate.field",
    "b.helper.Validate.empty", "b.helper.Validate.invalid", "b.helper.Validate.exit", "b.helper.Validate.failure",
    "b.bookkeeping.task.release")
  pure val visitRows = Set("b.helper.Validate.occurrence", "b.helper.Validate.field", "b.helper.Validate.invalid")
  pure def select(mm: Machine, ii: Input): str = match mm.helper {
    | NoHelper => match mm.result {
      | NoResult => "b.helper.Validate.entry"
      | Validated(ok) => if (ok) "k.machine.eval.literal.valid" else "k.machine.eval.literal.invalid"
    }
    | ValidateHelper(hh) =>
      if (hh.status == Negative) (if (mm.tasks.length() > 0) "b.bookkeeping.task.release" else "b.helper.Validate.failure")
      else if (hh.status == Positive) "b.helper.Validate.exit"
      else if (mm.tasks.length() == 0) "b.helper.Validate.empty"
      else if (top(mm).visit == Fields) "b.helper.Validate.field"
      else if (rowAt(ii, top(mm).ref).kind == MarkerKind) "b.helper.Validate.invalid"
      else "b.helper.Validate.occurrence"
  }
  pure def eventKind(rule: str): EventKind =
    if (rule == "b.helper.Validate.entry") HelperEnter
    else if (rule == "b.helper.Validate.exit") HelperExit
    else if (rule == "b.helper.Validate.failure") HelperFail
    else if (rule == "b.bookkeeping.task.release") Bookkeeping
    else if (helperRows.contains(rule)) HelperVisit else LegacyComplete
  pure def positiveTaskDelta(mm: Machine, ii: Input, rule: str): int =
    if (rule == "b.helper.Validate.entry") 1
    else if (rule == "b.helper.Validate.field")
      (if (top(mm).cursor + 1 < arity(rowAt(ii, top(mm).ref).kind)) 1 else 0)
    else 0
  pure def effects(mm: Machine, ii: Input, rule: str): Machine =
    if (rule == "b.helper.Validate.entry") {...mm,
      tasks: mm.tasks.append({ref: root(ii), visit: Unvisited, cursor: 0}),
      helper: ValidateHelper({site: "EvalLiteral", originReferenceCursor: mm.referenceCursor,
        parentResume: Absent, root: root(ii), status: Pending})}
    else if (rule == "k.machine.eval.literal.valid") {...mm, control: Return(root(ii)), result: NoResult}
    else if (rule == "k.machine.eval.literal.invalid") {...mm, control: Error(2), result: NoResult}
    else match mm.helper {
      | NoHelper => mm // Off-domain; not selected on an ordinary initialized state.
      | ValidateHelper(hh) =>
        if (rule == "b.helper.Validate.empty") {...mm, helper: ValidateHelper({...hh, status: Positive})}
        else if (rule == "b.helper.Validate.exit" or rule == "b.helper.Validate.failure")
          {...mm, helper: NoHelper, result: Validated(rule == "b.helper.Validate.exit")}
        else if (rule == "b.bookkeeping.task.release") {...mm, tasks: pop(mm)}
        else if (rule == "b.helper.Validate.invalid") {...mm, tasks: pop(mm),
          helper: ValidateHelper({...hh, status: Negative})}
        else if (rule == "b.helper.Validate.occurrence")
          (if (arity(rowAt(ii, top(mm).ref).kind) == 0) {...mm, tasks: pop(mm)}
          else {...mm, tasks: rewrite(mm, {...top(mm), visit: Fields, cursor: 0})})
        else {
          val tt = top(mm)
          val child = Const(ii.program.constantFields.nth(rowAt(ii, tt.ref).fieldsStart + tt.cursor))
          val childTask = {ref: child, visit: Unvisited, cursor: 0}
          if (tt.cursor + 1 < arity(rowAt(ii, tt.ref).kind))
            {...mm, tasks: rewrite(mm, {...tt, cursor: tt.cursor + 1}).append(childTask)}
          else {...mm, tasks: rewrite(mm, childTask)}
        }
    }
  pure def origin(pre: Machine, post: Machine, kind: EventKind, disp: Disposition): OptInt = match pre.helper {
    | ValidateHelper(hh) => Present(hh.originReferenceCursor)
    | NoHelper => match post.helper {
      | ValidateHelper(hh) => Present(hh.originReferenceCursor)
      | NoHelper => if (pre.result != NoResult or (kind == HelperEnter and
          (disp == RefusedHelperWorkUncharged or disp == RefusedCapCharged))) Present(pre.referenceCursor) else Absent
    }
  }
  pure def resultTag(pre: Machine, post: Machine, kind: EventKind, disp: Disposition): ResultTag =
    if ((disp == Applied and (kind == HelperExit or kind == HelperFail) and post.result != NoResult)
      or (disp == Applied and kind == LegacyComplete and pre.result != NoResult)
      or (kind == Publish and disp == ExhaustedFuelUncharged and pre.result != NoResult)) ValidatedTag else NoResultTag
  pure def makeEvent(ss: State, post: Machine, rule: str, kind: EventKind,
    disp: Disposition, exhausted: ExhaustedKind): Event = {
    val pre = ss.machine
    val helperRowDisposition = helperRows.contains(rule) and
      (disp == Applied or disp == RefusedHelperWorkUncharged or disp == RefusedCapCharged)
    {stepId: {runId: ss.runId, ordinal: pre.consumedWork}, ruleId: rule, eventKind: kind,
      site: if (kind == Publish) "Absent" else "EvalLiteral",
      helperKind: if (pre.helper != NoHelper or post.helper != NoHelper or pre.result != NoResult or helperRowDisposition)
        ValidateKind else NoHelperKind,
      disposition: disp, workDelta: post.consumedWork - pre.consumedWork,
      helperWorkDelta: post.helperWork - pre.helperWork, helperVisitsDelta: post.helperVisits - pre.helperVisits,
      referenceCursorDelta: post.referenceCursor - pre.referenceCursor,
      allocDeltas: {dataNodes: post.alloc.dataNodes - pre.alloc.dataNodes,
        dataFields: post.alloc.dataFields - pre.alloc.dataFields,
        environmentCells: post.alloc.environmentCells - pre.alloc.environmentCells},
      valueSlotWrites: post.valueSlots.length() - pre.valueSlots.length(),
      logicalDepthBefore: pre.depth, logicalDepthAfter: post.depth,
      taskCursorBefore: pre.tasks.length(), taskCursorAfter: post.tasks.length(),
      helperPhaseBefore: phase(pre.helper), helperPhaseAfter: phase(post.helper),
      helperCursorBefore: cursor(pre), helperCursorAfter: cursor(post),
      resultTag: resultTag(pre, post, kind, disp), originReferenceCursor: origin(pre, post, kind, disp),
      sourceExprId: pre.controlOrigin, sourceOccurrence: ss.input.occurrence, exhaustedKind: exhausted,
      outcomeTag: outcomeTag(post.outcome), outcomePayload: post.outcome,
      boundary: if (post.outcome == Running) NoBoundary else Terminal}
  }
  pure def emitted(ss: State, post: Machine, rule: str, kind: EventKind,
    disp: Disposition, exhausted: ExhaustedKind): State =
    {...ss, machine: post, lastEvent: StepEvent(makeEvent(ss, post, rule, kind, disp, exhausted))}
  pure def advance(ss: State): State = {
    val mm = ss.machine
    if (mm.outcome != Running) {...ss, lastEvent: NoEvent}
    else if (mm.helper == NoHelper and mm.result == NoResult and isFinal(mm))
      emitted(ss, {...mm, outcome: publishOutcome(mm)},
        match mm.control { | Return(_) => "k.admin.publish.value" | _ => "k.admin.publish.error" },
        Publish, Published, NoExhaustedKind)
    else if (mm.remainingWork == 0)
      emitted(ss, {...mm, outcome: Exhausted(Fuel)}, "k.admin.publish.exhausted", Publish,
        ExhaustedFuelUncharged, HasExhaustedKind(Fuel))
    else {
      val rule = select(mm, ss.input)
      val eligible = helperRows.contains(rule)
      val kind = eventKind(rule)
      if (eligible and mm.helperWork == ss.limits.bounds.maxHelperWork)
        emitted(ss, {...mm, outcome: Exhausted(HelperWork)}, rule, kind,
          RefusedHelperWorkUncharged, HasExhaustedKind(HelperWork))
      else {
        val charged = {...mm, remainingWork: mm.remainingWork - 1, consumedWork: mm.consumedWork + 1,
          helperWork: mm.helperWork + (if (eligible) 1 else 0)}
        if (positiveTaskDelta(mm, ss.input, rule) > 0 and
            mm.tasks.length() + positiveTaskDelta(mm, ss.input, rule) > ss.limits.bounds.helperTasks)
          emitted(ss, {...charged, outcome: Exhausted(HelperTasks)}, rule, kind,
            RefusedCapCharged, HasExhaustedKind(HelperTasks))
        else {
          val applied = effects(charged, ss.input, rule)
          val post = {...applied, helperVisits: mm.helperVisits + (if (visitRows.contains(rule)) 1 else 0),
            referenceCursor: mm.referenceCursor + (if (kind == LegacyComplete) 1 else 0)}
          emitted(ss, post, rule, kind, Applied, NoExhaustedKind)
        }
      }
    }
  }
  action initCase(ii: Input, ll: Limits) = all {
    cases.contains(ii), fuelDomain.contains(ll.initialFuel),
    helperBudgetDomain.contains(ll.bounds.maxHelperWork), taskCapacityDomain.contains(ll.bounds.helperTasks),
    ll == limits(ll.initialFuel, ll.bounds.maxHelperWork, ll.bounds.helperTasks),
    state' = initial(ii, ll),
  }
  action init = {
    nondet ii = cases.oneOf()
    nondet fuel = fuelDomain.oneOf()
    nondet budget = helperBudgetDomain.oneOf()
    nondet taskCap = taskCapacityDomain.oneOf()
    initCase(ii, limits(fuel, budget, taskCap))
  }
  action step = state' = advance(state)

  // Bookkeeping candidates only; no StepInv/genesis/preservation proof is claimed.
  val workAccounting = and {
    state.machine.remainingWork >= 0,
    state.machine.remainingWork + state.machine.consumedWork == state.machine.initialFuel,
    state.machine.helperWork <= state.machine.consumedWork,
    state.machine.helperWork <= state.limits.bounds.maxHelperWork,
    state.machine.helperVisits <= state.machine.helperWork,
    state.machine.referenceCursor <= state.machine.consumedWork,
  }
  val preservation = and {
    state.machine.controlOrigin == state.input.expr, state.machine.envRoot == state.input.envRoot,
    state.machine.depth == 0, state.machine.frames == List(), state.machine.stackLimit == state.limits.stackLimit,
    state.machine.data == state.input.data, state.machine.dataFields == state.input.dataFields,
    state.machine.env == state.input.env, state.machine.valueSlots == state.input.valueSlots,
    state.machine.alloc == initial(state.input, state.limits).machine.alloc,
    state.machine.segBase == {active: zeroActive, alloc: zeroAlloc, consumedWork: 0},
    state.runId == initial(state.input, state.limits).runId,
    state.machine.tasks.length() <= state.limits.bounds.helperTasks,
  }
  val allInv = workAccounting and preservation
  val published = match state.machine.outcome { | Halted(_) => true | _ => false }
}
