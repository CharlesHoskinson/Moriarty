// SOURCE ONLY. Typecheck, test, simulation and verification UNRUN.
// R4 HOST DedicatedSorts EnvGet/Bind component, not a new public profile.
// See README.md for cutpoint premises, finite domain and omitted fields.
module bPure {
  type CellRef = Absent | At(int)
  type Sort = UIntSort | BoolSort
  type Value = UIntValue(int) | BoolValue(bool)
  type Cell = {binder: str, value: int, previous: CellRef}
  type Param = {binder: str, sort: Sort}
  type PairTask = {binder: str, sort: Sort, arg: int}
  type TriStatus = Pending | Positive | Negative
  type BindPhase = Pairing | UnwindType | UnwindDup | UnwindInsert | Propagate | Finished
  type BindStatus = Ok | Bad
  type GetState = {binder: str, cursor: CellRef, found: CellRef, status: TriStatus,
    originReferenceCursor: int}
  type BindState = {pCursor: int, aCursor: int, partial: CellRef, dupCursor: CellRef,
    phase: BindPhase, status: BindStatus, originReferenceCursor: int}
  type Helper = NoHelper | GetHelper(GetState) | BindHelper(BindState)
  // BindingOk(Absent) is success; it is distinct from BindingBad.
  type Result = NoResult | EnvFound(int) | EnvMissing | BindingOk(CellRef) | BindingBad
  type Control = EvalVariable(str) | EnterFunction | BoundOk(CellRef) | BoundBad
    | ScheduleBody(CellRef) | ReturnValue(int) | ErrorCode(int)
  type Outcome = Running | Halted(int) | Rejected(int) | FuelExhausted
    | HelperWorkExhausted | EnvironmentCellsExhausted | HelperTasksExhausted
  type EventKind = HelperEnter | HelperVisit | HelperExit | HelperFail | LegacyComplete | Publish
  type Disposition = Applied | Published | ExhaustedFuelUncharged
    | RefusedHelperWorkUncharged | RefusedCapCharged
  type Event = {ruleId: str, eventKind: EventKind, disposition: Disposition,
    workDelta: int, helperWorkDelta: int, helperVisitsDelta: int, referenceCursorDelta: int,
    environmentCellsDelta: int, taskCursorBefore: int, taskCursorAfter: int,
    helperCursorBefore: CellRef, helperCursorAfter: CellRef,
    helperPhaseBefore: str, helperPhaseAfter: str, originReferenceCursor: CellRef}
  type LastEvent = NoEvent | StepEvent(Event)
  type Input = {caseId: str, control: Control, envRoot: CellRef, cells: List[Cell],
    params: List[Param], args: List[int]}
  type Limits = {initialFuel: int, maxHelperWork: int, environmentCells: int, helperTasks: int}
  type Machine = {control: Control, envRoot: CellRef, cells: List[Cell], helper: Helper,
    result: Result, tasks: List[PairTask], outcome: Outcome, remainingWork: int,
    consumedWork: int, helperWork: int, helperVisits: int, referenceCursor: int,
    allocEnvironmentCells: int}
  type State = {input: Input, limits: Limits, machine: Machine, lastEvent: LastEvent}

  // MODEL-ONLY input domain; these are not language or public resource caps.
  pure val values: List[Value] = List(UIntValue(0), UIntValue(1), BoolValue(false), BoolValue(true))
  pure val names: Set[str] = Set("x", "y", "z")
  pure val fuels: Set[int] = 0.to(32)
  pure val helperBudgets: Set[int] = Set(0, 1, 4, 16, 32)
  pure val environmentBudgets: Set[int] = Set(0, 1, 2, 3, 4, 5)
  pure val taskBudgets: Set[int] = Set(0, 1, 2, 3)
  pure val emptyCells: List[Cell] = List()
  pure val emptyParams: List[Param] = List()
  pure val emptyArgs: List[int] = List()
  pure val emptyTasks: List[PairTask] = List()
  pure val px: Param = {binder: "x", sort: UIntSort}
  pure val py: Param = {binder: "y", sort: BoolSort}
  pure val pz: Param = {binder: "z", sort: UIntSort}
  pure val seedCell: Cell = {binder: "x", value: 0, previous: Absent}
  pure val shadowCell: Cell = {binder: "x", value: 1, previous: At(0)}
  pure val skipCell: Cell = {binder: "y", value: 2, previous: At(0)}
  // Independently specified input-only witnesses; no expected outputs or counts.
  pure val cases: Set[Input] = Set(
    {caseId: "lookup-empty", control: EvalVariable("x"), envRoot: Absent,
      cells: emptyCells, params: emptyParams, args: emptyArgs},
    {caseId: "lookup-one", control: EvalVariable("x"), envRoot: At(0),
      cells: List(seedCell), params: emptyParams, args: emptyArgs},
    {caseId: "lookup-shadow", control: EvalVariable("x"), envRoot: At(1),
      cells: List(seedCell, shadowCell), params: emptyParams, args: emptyArgs},
    {caseId: "lookup-deeper", control: EvalVariable("x"), envRoot: At(1),
      cells: List(seedCell, skipCell), params: emptyParams, args: emptyArgs},
    {caseId: "lookup-miss", control: EvalVariable("z"), envRoot: At(1),
      cells: List(seedCell, skipCell), params: emptyParams, args: emptyArgs},
    {caseId: "bind-empty", control: EnterFunction, envRoot: At(0),
      cells: List(seedCell), params: emptyParams, args: emptyArgs},
    {caseId: "bind-three", control: EnterFunction, envRoot: At(0),
      cells: List(seedCell), params: List(px, py, pz), args: List(0, 2, 1)},
    {caseId: "bind-arity", control: EnterFunction, envRoot: Absent,
      cells: emptyCells, params: List(px, py), args: List(0)},
    {caseId: "bind-sort", control: EnterFunction, envRoot: Absent,
      cells: emptyCells, params: List(py, pz), args: List(0, 1)},
    {caseId: "bind-duplicate", control: EnterFunction, envRoot: At(0),
      cells: List(seedCell), params: List(px, px, pz), args: List(0, 1, 0)})

  pure def sortOf(ref: int): Sort = match values.nth(ref) {
    | UIntValue(_) => UIntSort
    | BoolValue(_) => BoolSort
  }
  pure def validCellRef(ref: CellRef, cellCount: int): bool = match ref {
    | Absent => true
    | At(ix) => ix >= 0 and ix < cellCount
  }
  pure def phaseOf(hh: Helper): str = match hh {
    | NoHelper => "absent"
    | GetHelper(gg) => if (gg.status == Pending) "Pending"
      else if (gg.status == Positive) "Positive" else "Negative"
    | BindHelper(bb) => if (bb.phase == Pairing) "Pairing"
      else if (bb.phase == UnwindType) "UnwindType"
      else if (bb.phase == UnwindDup) "UnwindDup"
      else if (bb.phase == UnwindInsert) "UnwindInsert"
      else if (bb.phase == Propagate) "Propagate" else "Finished"
  }
  pure def cursorOf(hh: Helper): CellRef = match hh {
    | NoHelper => Absent
    | GetHelper(gg) => gg.cursor
    | BindHelper(bb) => At(bb.pCursor)
  }
  pure def originOf(hh: Helper): CellRef = match hh {
    | NoHelper => Absent
    | GetHelper(gg) => At(gg.originReferenceCursor)
    | BindHelper(bb) => At(bb.originReferenceCursor)
  }
  pure def isCutpoint(mm: Machine): bool = match mm.control {
    | ScheduleBody(_) => true
    | _ => false
  }
  pure def isFinalControl(mm: Machine): bool = match mm.control {
    | ReturnValue(_) => true // depth is identically zero in this component.
    | ErrorCode(_) => true
    | _ => false
  }
  pure def terminalOutcome(cc: Control): Outcome = match cc {
    | ReturnValue(vv) => Halted(vv)
    | ErrorCode(ee) => Rejected(ee)
    | _ => Running // only called under isFinalControl.
  }
  pure def terminalRule(cc: Control): str = match cc {
    | ReturnValue(_) => "k.admin.publish.value"
    | _ => "k.admin.publish.error"
  }

  pure def getRule(gg: GetState): str =
    if (gg.status == Positive) "b.helper.EnvGet.exit"
    else if (gg.status == Negative) "b.helper.EnvGet.failure"
    else if (gg.cursor == Absent) "b.helper.EnvGet.empty"
    else "b.helper.EnvGet.cell"
  pure def bindRule(bb: BindState, ii: Input): str =
    if (bb.phase == Finished) (if (bb.status == Ok) "b.helper.Bind.exit" else "b.helper.Bind.failure")
    else if (bb.phase == Pairing) (
      if (bb.pCursor < ii.params.length() and bb.aCursor < ii.args.length()) "b.helper.Bind.pair"
      else if (bb.pCursor == ii.params.length() and bb.aCursor == ii.args.length()) "b.helper.Bind.empty"
      else "b.helper.Bind.arity")
    else if (bb.phase == UnwindType) "b.helper.Bind.type"
    else if (bb.phase == UnwindDup) "b.helper.Bind.duplicate"
    else if (bb.phase == UnwindInsert) "b.helper.Bind.insert"
    else "b.helper.Bind.propagate"
  pure def completionRule(rr: Result): str = match rr {
    | EnvFound(_) => "k.machine.eval.variable.present"
    | EnvMissing => "k.machine.eval.variable.absent"
    | BindingOk(_) => "k.machine.enter.bind"
    | BindingBad => "k.machine.enter.bind"
    | NoResult => "outside"
  }
  pure def initialRule(cc: Control): str = match cc {
    | EvalVariable(_) => "b.helper.EnvGet.entry"
    | EnterFunction => "b.helper.Bind.entry"
    | BoundOk(_) => "k.machine.bound.valid"
    | BoundBad => "k.machine.bound.invalid"
    | _ => "outside"
  }
  pure def selectedRule(mm: Machine, ii: Input): str = match mm.helper {
    | GetHelper(gg) => getRule(gg)
    | BindHelper(bb) => bindRule(bb, ii)
    | NoHelper => if (mm.result == NoResult) initialRule(mm.control) else completionRule(mm.result)
  }
  pure val helperRows: Set[str] = Set("b.helper.EnvGet.entry", "b.helper.EnvGet.cell",
    "b.helper.EnvGet.empty", "b.helper.EnvGet.exit", "b.helper.EnvGet.failure",
    "b.helper.Bind.entry", "b.helper.Bind.pair", "b.helper.Bind.empty", "b.helper.Bind.arity",
    "b.helper.Bind.type", "b.helper.Bind.duplicate", "b.helper.Bind.insert",
    "b.helper.Bind.propagate", "b.helper.Bind.exit", "b.helper.Bind.failure")
  pure val visitRows: Set[str] = Set("b.helper.EnvGet.cell", "b.helper.Bind.pair",
    "b.helper.Bind.type", "b.helper.Bind.duplicate", "b.helper.Bind.insert", "b.helper.Bind.propagate")
  pure def kindOf(rule: str): EventKind =
    if (Set("b.helper.EnvGet.entry", "b.helper.Bind.entry").contains(rule)) HelperEnter
    else if (Set("b.helper.EnvGet.exit", "b.helper.Bind.exit").contains(rule)) HelperExit
    else if (Set("b.helper.EnvGet.failure", "b.helper.Bind.failure").contains(rule)) HelperFail
    else if (helperRows.contains(rule)) HelperVisit else LegacyComplete

  pure def applyGet(mm: Machine, gg: GetState, rule: str): Machine =
    if (rule == "b.helper.EnvGet.exit") {...mm, helper: NoHelper,
      result: match gg.found { | At(vv) => EnvFound(vv) | Absent => EnvMissing }}
    else if (rule == "b.helper.EnvGet.failure") {...mm, helper: NoHelper, result: EnvMissing}
    else if (rule == "b.helper.EnvGet.empty") {...mm, helper: GetHelper({...gg, status: Negative})}
    else match gg.cursor {
      | Absent => mm // unreachable under selected EnvGet.cell.
      | At(ix) => {
        val cl = mm.cells.nth(ix)
        val changedGet = if (cl.binder == gg.binder) {...gg, found: At(cl.value), status: Positive}
          else {...gg, cursor: cl.previous}
        {...mm, helper: GetHelper(changedGet)}
      }
    }
  pure def afterPopPhase(taskCount: int, morePhase: BindPhase): BindPhase =
    if (taskCount > 1) morePhase else Finished
  pure def rejectPair(mm: Machine, bb: BindState): Machine = {...mm,
    tasks: mm.tasks.slice(0, mm.tasks.length() - 1),
    helper: BindHelper({...bb, status: Bad, dupCursor: Absent,
      phase: afterPopPhase(mm.tasks.length(), Propagate)})}
  pure def applyBind(mm: Machine, bb: BindState, ii: Input, rule: str): Machine =
    if (rule == "b.helper.Bind.exit") {...mm, helper: NoHelper, result: BindingOk(bb.partial)}
    else if (rule == "b.helper.Bind.failure") {...mm, helper: NoHelper, result: BindingBad}
    else if (rule == "b.helper.Bind.pair") {
      val pr = ii.params.nth(bb.pCursor)
      {...mm, tasks: mm.tasks.append({binder: pr.binder, sort: pr.sort, arg: ii.args.nth(bb.aCursor)}),
        helper: BindHelper({...bb, pCursor: bb.pCursor + 1, aCursor: bb.aCursor + 1})}
    }
    else if (rule == "b.helper.Bind.empty") {...mm, helper: BindHelper({...bb,
      phase: if (mm.tasks.length() > 0) UnwindType else Finished})}
    else if (rule == "b.helper.Bind.arity") {...mm, helper: BindHelper({...bb, status: Bad,
      phase: if (mm.tasks.length() > 0) Propagate else Finished})}
    else if (rule == "b.helper.Bind.propagate") {...mm,
      tasks: mm.tasks.slice(0, mm.tasks.length() - 1),
      helper: BindHelper({...bb, phase: afterPopPhase(mm.tasks.length(), Propagate)})}
    else {
      val tt = mm.tasks.nth(mm.tasks.length() - 1)
      if (rule == "b.helper.Bind.type") (
        if (sortOf(tt.arg) != tt.sort) rejectPair(mm, bb)
        else {...mm, helper: BindHelper({...bb, dupCursor: bb.partial,
          phase: if (bb.partial == Absent) UnwindInsert else UnwindDup})})
      else if (rule == "b.helper.Bind.duplicate") match bb.dupCursor {
        | Absent => mm // unreachable under UnwindDup premise.
        | At(ix) => {
          val cl = mm.cells.nth(ix)
          if (cl.binder == tt.binder) rejectPair(mm, bb)
          else {...mm, helper: BindHelper({...bb, dupCursor: cl.previous,
            phase: if (cl.previous == Absent) UnwindInsert else UnwindDup})}
        }
      }
      else {...mm, cells: mm.cells.append({binder: tt.binder, value: tt.arg, previous: bb.partial}),
        allocEnvironmentCells: mm.allocEnvironmentCells + 1,
        tasks: mm.tasks.slice(0, mm.tasks.length() - 1),
        helper: BindHelper({...bb, partial: At(mm.cells.length()),
          phase: afterPopPhase(mm.tasks.length(), UnwindType)})}
    }
  pure def consumeResult(mm: Machine): Machine = match mm.result {
    | EnvFound(vv) => {...mm, control: ReturnValue(vv), result: NoResult}
    | EnvMissing => {...mm, control: ErrorCode(3), result: NoResult}
    | BindingOk(root) => {...mm, control: BoundOk(root), result: NoResult}
    | BindingBad => {...mm, control: BoundBad, result: NoResult}
    | NoResult => mm
  }
  pure def startOrBound(mm: Machine): Machine = match mm.control {
    | EvalVariable(binder) => {...mm, helper: GetHelper({binder: binder, cursor: mm.envRoot,
        found: Absent, status: Pending, originReferenceCursor: mm.referenceCursor})}
    | EnterFunction => {...mm, helper: BindHelper({pCursor: 0, aCursor: 0, partial: Absent,
        dupCursor: Absent, phase: Pairing, status: Ok, originReferenceCursor: mm.referenceCursor})}
    | BoundOk(root) => {...mm, control: ScheduleBody(root)}
    | BoundBad => {...mm, control: ErrorCode(8)}
    | _ => mm // outside component; advance does not call this arm.
  }
  pure def applyEffects(mm: Machine, ii: Input, rule: str): Machine = match mm.helper {
    | GetHelper(gg) => applyGet(mm, gg, rule)
    | BindHelper(bb) => applyBind(mm, bb, ii, rule)
    | NoHelper => if (mm.result == NoResult) startOrBound(mm) else consumeResult(mm)
  }
  pure def eventOf(pre: Machine, post: Machine, rule: str, ek: EventKind, dd: Disposition): Event = {
    ruleId: rule, eventKind: ek, disposition: dd,
    workDelta: post.consumedWork - pre.consumedWork,
    helperWorkDelta: post.helperWork - pre.helperWork,
    helperVisitsDelta: post.helperVisits - pre.helperVisits,
    referenceCursorDelta: post.referenceCursor - pre.referenceCursor,
    environmentCellsDelta: post.allocEnvironmentCells - pre.allocEnvironmentCells,
    taskCursorBefore: pre.tasks.length(), taskCursorAfter: post.tasks.length(),
    helperCursorBefore: cursorOf(pre.helper), helperCursorAfter: cursorOf(post.helper),
    helperPhaseBefore: phaseOf(pre.helper), helperPhaseAfter: phaseOf(post.helper),
    originReferenceCursor: originOf(pre.helper)
  }
  pure def withEvent(ss: State, post: Machine, rule: str, ek: EventKind, dd: Disposition): State =
    {...ss, machine: post, lastEvent: StepEvent(eventOf(ss.machine, post, rule, ek, dd))}
  pure def advance(ss: State): State = {
    val mm = ss.machine
    // P0 and the explicit successful-Bind comparison boundary produce no event.
    if (mm.outcome != Running or isCutpoint(mm)) {...ss, lastEvent: NoEvent}
    else if (mm.helper == NoHelper and mm.result == NoResult and isFinalControl(mm))
      withEvent(ss, {...mm, outcome: terminalOutcome(mm.control)}, terminalRule(mm.control), Publish, Published)
    else if (mm.remainingWork == 0)
      withEvent(ss, {...mm, outcome: FuelExhausted}, "k.admin.publish.exhausted", Publish, ExhaustedFuelUncharged)
    else {
      val rule = selectedRule(mm, ss.input)
      val eligible = helperRows.contains(rule)
      val ek = kindOf(rule)
      // P3 precedes charging and positive-cap checks.
      if (eligible and mm.helperWork == ss.limits.maxHelperWork)
        withEvent(ss, {...mm, outcome: HelperWorkExhausted}, rule, ek, RefusedHelperWorkUncharged)
      else {
        val chargedMachine = {...mm, remainingWork: mm.remainingWork - 1,
          consumedWork: mm.consumedWork + 1, helperWork: mm.helperWork + (if (eligible) 1 else 0)}
        // Only these two rows allocate positively within this component.
        if (rule == "b.helper.Bind.insert" and mm.allocEnvironmentCells + 1 > ss.limits.environmentCells)
          withEvent(ss, {...chargedMachine, outcome: EnvironmentCellsExhausted}, rule, ek, RefusedCapCharged)
        else if (rule == "b.helper.Bind.pair" and mm.tasks.length() + 1 > ss.limits.helperTasks)
          withEvent(ss, {...chargedMachine, outcome: HelperTasksExhausted}, rule, ek, RefusedCapCharged)
        else {
          val effected = applyEffects(chargedMachine, ss.input, rule)
          val post = {...effected, helperVisits: mm.helperVisits + (if (visitRows.contains(rule)) 1 else 0),
            referenceCursor: mm.referenceCursor + (if (ek == LegacyComplete) 1 else 0)}
          withEvent(ss, post, rule, ek, Applied)
        }
      }
    }
  }
  pure def initial(ii: Input, ll: Limits): State = {
    input: ii, limits: ll, lastEvent: NoEvent,
    machine: {control: ii.control, envRoot: ii.envRoot, cells: ii.cells, helper: NoHelper,
      result: NoResult, tasks: emptyTasks, outcome: Running, remainingWork: ll.initialFuel,
      consumedWork: 0, helperWork: 0, helperVisits: 0, referenceCursor: 0,
      allocEnvironmentCells: ii.cells.length()}
  }
}
