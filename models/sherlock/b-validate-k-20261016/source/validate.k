requires "domains.md"

module SHERLOCK-B-VALIDATE-SYNTAX
  imports DOMAINS-SYNTAX
  // A separate B projection. No import of, or edit to, Sherlock A main.k.
  syntax BVRef ::= bvConst(Int)
  syntax BVRow ::= bvUInt(Int) | bvBool(Bool) | bvNil()
                 | bvCons(BVRef, BVRef) | bvLeaf(BVRef) | bvBranch(BVRef, BVRef)
  syntax BVRows ::= bvRowsNil() | bvRowsCons(BVRow, BVRows)
  syntax BVVisit ::= bvUnvisited() | bvFields()
  syntax BVTask ::= bvTask(BVRef, BVVisit, Int)
  // Top first; reverse this list to obtain normative bottom-first img.tasks.
  syntax BVTasks ::= bvTasksNil() | bvTasksCons(BVTask, BVTasks)
  syntax BVStatus ::= bvPending() | bvPositive()
  syntax BVParent ::= bvNoResume()
  syntax BVHelper ::= bvNoHelper()
                    | bvValidate(String, Int, BVParent, BVRef, BVStatus)
  syntax BVResult ::= bvNoResult() | bvValidated(Bool)
  syntax BVControl ::= bvEval(Int) | bvReturn(BVRef)
  syntax BVOutcome ::= bvRunning() | bvHalted(BVRef)
  // remainingWork, consumedWork, helperWork, helperVisits, referenceCursor
  syntax BVClocks ::= bvClocks(Int, Int, Int, Int, Int)
  // A literal-only ProgramView projection: expression 0 = Literal(root).
  // Constants are immutable; their field refs retain actual constant IDs.
  // rows, literal-root, initialFuel, stackLimit, maxHelperWork, helperTasks.
  syntax BVFixed ::= bvFixed(BVRows, BVRef, Int, Int, Int, Int)
  syntax BVImage ::= bvImage(BVFixed, BVControl, BVHelper, BVResult,
                             BVTasks, BVClocks, BVOutcome)
  syntax BVOptInt ::= bvNoInt() | bvSomeInt(Int)
  syntax BVOptStatus ::= bvNoStatus() | bvSomeStatus(BVStatus)
  syntax BVOptTag ::= bvNoTag() | bvTag(String)
  // run projection = profile, grammarDomain, finance, literal image, F/L.
  // Identity's other fields are explicitly excluded in FIELD-PROJECTION.json.
  syntax BVRun ::= bvRun(String, String, String, BVFixed)
  syntax BVStepId ::= bvStepId(BVRun, Int)
  syntax BVDeltas ::= bvDeltas(Int, Int, Int, Int, Int, Int, Int, Int)
  // work, helperWork, helperVisits, referenceCursor, dataNodes, dataFields,
  // environmentCells, valueSlotWrites. All deltas are computed from actual states.
  syntax BVTaskObservation ::= bvTaskObservation(Int, Int, BVOptStatus,
                              BVOptStatus, BVOptInt, BVOptInt)
  syntax BVEvent ::= bvEvent(BVStepId, String, String, BVOptTag, BVOptTag,
                            String, BVDeltas, Int, Int, BVTaskObservation,
                            BVOptTag, BVOptInt, Int, BVOptTag, BVOutcome,
                            String, BVImage, BVImage)
  // All 27 normative event fields have a typed projection, grouped above.
  // Last two fields retain complete projected pre/post images.
  syntax BVEvents ::= bvEventsNil() | bvEventsCons(BVEvent, BVEvents)
  syntax BVTransition ::= bvTransition(String, String, BVImage)
  syntax BVRequest ::= bvRequest(BVRows, BVRef, Int, Int, Int, Int)
  syntax BVExecution ::= bvExecution(BVImage, BVImage, BVEvents)
  syntax BVAnswer ::= bvAnswer(BVImage, BVImage, BVEvents)
  syntax KItem ::= BVRequest | BVExecution | BVAnswer
endmodule

module SHERLOCK-B-VALIDATE-FUNCTIONS
  imports SHERLOCK-B-VALIDATE-SYNTAX
  imports DOMAINS
  syntax Int ::= bvRowsLength(BVRows) [function]
               | bvTaskLive(BVTasks) [function]
               | bvArity(BVRow) [function]
  rule bvRowsLength(bvRowsNil()) => 0
  rule bvRowsLength(bvRowsCons(_, T)) => 1 +Int bvRowsLength(T)
  rule bvTaskLive(bvTasksNil()) => 0
  rule bvTaskLive(bvTasksCons(_, T)) => 1 +Int bvTaskLive(T)
  rule bvArity(bvUInt(_)) => 0
  rule bvArity(bvBool(_)) => 0
  rule bvArity(bvNil()) => 0
  rule bvArity(bvCons(_, _)) => 2
  rule bvArity(bvLeaf(_)) => 1
  rule bvArity(bvBranch(_, _)) => 2
  syntax BVRow ::= bvAt(BVRows, BVRef) [function]
  rule bvAt(bvRowsCons(H, _), bvConst(0)) => H
  rule bvAt(bvRowsCons(_, T), bvConst(I)) => bvAt(T, bvConst(I -Int 1))
    requires I >Int 0
  syntax BVRef ::= bvField(BVRow, Int) [function]
  rule bvField(bvCons(A, _), 0) => A
  rule bvField(bvCons(_, B), 1) => B
  rule bvField(bvLeaf(A), 0) => A
  rule bvField(bvBranch(A, _), 0) => A
  rule bvField(bvBranch(_, B), 1) => B

  // Explicit normal-input envelope, not admission/StepInv or a new product cap.
  // Child IDs strictly precede a parent; repeated child IDs are permitted.
  syntax Bool ::= bvEarlier(BVRef, Int) [function]
                | bvSort(BVRow, String) [function]
                | bvRowNormal(BVRows, BVRow, Int) [function]
                | bvTableNormal(BVRows, BVRows, Int) [function]
  rule bvEarlier(bvConst(I), N) => I >=Int 0 andBool I <Int N
  rule bvSort(bvUInt(_), "UInt") => true
  rule bvSort(bvBool(_), "Bool") => true
  rule bvSort(bvNil(), "List") => true
  rule bvSort(bvCons(_, _), "List") => true
  rule bvSort(bvLeaf(_), "Tree") => true
  rule bvSort(bvBranch(_, _), "Tree") => true
  rule bvSort(_, _) => false [owise]
  rule bvRowNormal(_, bvUInt(N), _) => N >=Int 0 andBool N <Int (2 ^Int 128)
  rule bvRowNormal(_, bvBool(_), _) => true
  rule bvRowNormal(_, bvNil(), _) => true
  rule bvRowNormal(Rows, bvCons(A, B), I)
    => bvEarlier(A, I) andBool bvEarlier(B, I)
       andBool bvSort(bvAt(Rows, A), "UInt") andBool bvSort(bvAt(Rows, B), "List")
  rule bvRowNormal(Rows, bvLeaf(A), I)
    => bvEarlier(A, I) andBool bvSort(bvAt(Rows, A), "UInt")
  rule bvRowNormal(Rows, bvBranch(A, B), I)
    => bvEarlier(A, I) andBool bvEarlier(B, I)
       andBool bvSort(bvAt(Rows, A), "Tree") andBool bvSort(bvAt(Rows, B), "Tree")
  rule bvTableNormal(_, bvRowsNil(), _) => true
  rule bvTableNormal(All, bvRowsCons(H, T), I)
    => bvRowNormal(All, H, I) andBool bvTableNormal(All, T, I +Int 1)

  // Static normal-envelope measures; not expanded events or expected outputs.
  // Repeated references are unfolded per occurrence, never memoized away.
  syntax Int ::= bvVisitsNeeded(BVRows, BVRef) [function]
               | bvRowVisits(BVRows, BVRow) [function]
  rule bvVisitsNeeded(Rows, R) => bvRowVisits(Rows, bvAt(Rows, R))
  rule bvRowVisits(_, bvUInt(_)) => 1
  rule bvRowVisits(_, bvBool(_)) => 1
  rule bvRowVisits(_, bvNil()) => 1
  rule bvRowVisits(Rows, bvLeaf(A)) => 2 +Int bvVisitsNeeded(Rows, A)
  rule bvRowVisits(Rows, bvCons(A, B))
    => 3 +Int bvVisitsNeeded(Rows, A) +Int bvVisitsNeeded(Rows, B)
  rule bvRowVisits(Rows, bvBranch(A, B))
    => 3 +Int bvVisitsNeeded(Rows, A) +Int bvVisitsNeeded(Rows, B)
  syntax Bool ::= bvNormal(BVRequest) [function]
  rule bvNormal(bvRequest(Rows, Root, F, L, H, T))
    => bvTableNormal(Rows, Rows, 0)
       andBool bvEarlier(Root, bvRowsLength(Rows))
       andBool F >=Int bvVisitsNeeded(Rows, Root) +Int 4
       andBool H >=Int bvVisitsNeeded(Rows, Root) +Int 3
       andBool F <Int (2 ^Int 128) andBool H <Int (2 ^Int 128)
       andBool L >=Int 0 andBool T >=Int bvRowsLength(Rows)
  syntax BVImage ::= bvGenesis(BVRequest) [function]
  rule bvGenesis(bvRequest(Rows, Root, F, L, H, T))
    => bvImage(bvFixed(Rows, Root, F, L, H, T), bvEval(0), bvNoHelper(),
               bvNoResult(), bvTasksNil(), bvClocks(F, 0, 0, 0, 0), bvRunning())

  // Each bvNext call produces exactly one schedule event. Traversal is in
  // explicit helper tasks; no recursive uiValid evaluation replaces an event.
  syntax BVTransition ::= bvNext(BVImage) [function]
  rule bvNext(bvImage(Fix, bvEval(0), bvNoHelper(), bvNoResult(), bvTasksNil(),
                       bvClocks(F, C, H, V, R), bvRunning()))
    => bvEnter(Fix, F, C, H, V, R)
    requires F >Int 0
  syntax BVTransition ::= bvEnter(BVFixed, Int, Int, Int, Int, Int) [function]
  rule bvEnter(bvFixed(Rows, Root, Initial, L, MaxH, CapT), F, C, H, V, R)
    => bvTransition("b.helper.Validate.entry", "HelperEnter",
         bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), bvEval(0),
           bvValidate("EvalLiteral", R, bvNoResume(), Root, bvPending()),
           bvNoResult(), bvTasksCons(bvTask(Root, bvUnvisited(), 0), bvTasksNil()),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V, R), bvRunning()))
    requires H <Int MaxH andBool CapT >=Int 1

  rule bvNext(bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper,
                       Res, bvTasksCons(bvTask(Ref, bvUnvisited(), 0), Tail),
                       bvClocks(F, C, H, V, R), bvRunning()))
    => bvOccurrence(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper,
                    Res, Ref, Tail, bvAt(Rows, Ref), F, C, H, V, R)
    requires F >Int 0 andBool H <Int MaxH
       andBool bvIsPending(Helper)
  syntax Bool ::= bvIsPending(BVHelper) [function]
  rule bvIsPending(bvValidate(_, _, _, _, bvPending())) => true
  rule bvIsPending(_) => false [owise]
  syntax BVTransition ::= bvOccurrence(BVFixed, BVControl, BVHelper, BVResult,
                          BVRef, BVTasks, BVRow, Int, Int, Int, Int, Int) [function]
  rule bvOccurrence(Fix, Ctrl, Helper, Res, _, Tail, Row, F, C, H, V, R)
    => bvTransition("b.helper.Validate.occurrence", "HelperVisit",
         bvImage(Fix, Ctrl, Helper, Res, Tail,
                  bvClocks(F -Int 1, C +Int 1, H +Int 1, V +Int 1, R), bvRunning()))
    requires bvArity(Row) ==Int 0
  rule bvOccurrence(Fix, Ctrl, Helper, Res, Ref, Tail, Row, F, C, H, V, R)
    => bvTransition("b.helper.Validate.occurrence", "HelperVisit",
         bvImage(Fix, Ctrl, Helper, Res,
           bvTasksCons(bvTask(Ref, bvFields(), 0), Tail),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V +Int 1, R), bvRunning()))
    requires bvArity(Row) >Int 0

  rule bvNext(bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper,
                       Res, bvTasksCons(bvTask(Ref, bvFields(), I), Tail),
                       bvClocks(F, C, H, V, R), bvRunning()))
    => bvFieldStep(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper,
                  Res, Ref, I, Tail, bvAt(Rows, Ref), F, C, H, V, R)
    requires F >Int 0 andBool H <Int MaxH andBool bvIsPending(Helper)
  syntax BVTransition ::= bvFieldStep(BVFixed, BVControl, BVHelper, BVResult,
                          BVRef, Int, BVTasks, BVRow, Int, Int, Int, Int, Int) [function]
  rule bvFieldStep(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper,
                    Res, Ref, I, Tail, Row, F, C, H, V, R)
    => bvTransition("b.helper.Validate.field", "HelperVisit",
         bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl, Helper, Res,
           bvTasksCons(bvTask(bvField(Row, I), bvUnvisited(), 0),
             bvTasksCons(bvTask(Ref, bvFields(), I +Int 1), Tail)),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V +Int 1, R), bvRunning()))
    requires I >=Int 0 andBool I +Int 1 <Int bvArity(Row)
       andBool bvTaskLive(Tail) +Int 2 <=Int CapT
  rule bvFieldStep(Fix, Ctrl, Helper, Res, _, I, Tail, Row, F, C, H, V, R)
    => bvTransition("b.helper.Validate.field", "HelperVisit",
         bvImage(Fix, Ctrl, Helper, Res,
           bvTasksCons(bvTask(bvField(Row, I), bvUnvisited(), 0), Tail),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V +Int 1, R), bvRunning()))
    requires I >=Int 0 andBool I +Int 1 ==Int bvArity(Row)

  rule bvNext(bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl,
                       bvValidate(Site, Origin, Parent, Ref, bvPending()), Res,
                       bvTasksNil(), bvClocks(F, C, H, V, R), bvRunning()))
    => bvTransition("b.helper.Validate.empty", "HelperVisit",
         bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl,
           bvValidate(Site, Origin, Parent, Ref, bvPositive()), Res, bvTasksNil(),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V, R), bvRunning()))
    requires F >Int 0 andBool H <Int MaxH
  rule bvNext(bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl,
                       bvValidate(_, _, _, _, bvPositive()), bvNoResult(),
                       bvTasksNil(), bvClocks(F, C, H, V, R), bvRunning()))
    => bvTransition("b.helper.Validate.exit", "HelperExit",
         bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), Ctrl,
           bvNoHelper(), bvValidated(true), bvTasksNil(),
           bvClocks(F -Int 1, C +Int 1, H +Int 1, V, R), bvRunning()))
    requires F >Int 0 andBool H <Int MaxH
  rule bvNext(bvImage(Fix, bvEval(0), bvNoHelper(), bvValidated(true),
                       bvTasksNil(), bvClocks(F, C, H, V, R), bvRunning()))
    => bvLiteral(Fix, F, C, H, V, R)
    requires F >Int 0
  syntax BVTransition ::= bvLiteral(BVFixed, Int, Int, Int, Int, Int) [function]
  rule bvLiteral(bvFixed(Rows, Root, Initial, L, MaxH, CapT), F, C, H, V, R)
    => bvTransition("k.machine.eval.literal.valid", "LegacyComplete",
         bvImage(bvFixed(Rows, Root, Initial, L, MaxH, CapT), bvReturn(Root),
           bvNoHelper(), bvNoResult(), bvTasksNil(),
           bvClocks(F -Int 1, C +Int 1, H, V, R +Int 1), bvRunning()))
  // P1 publication has priority even when the last charge used all fuel.
  rule bvNext(bvImage(Fix, bvReturn(Ref), bvNoHelper(), bvNoResult(), bvTasksNil(),
                       Clock, bvRunning()))
    => bvTransition("k.admin.publish.value", "Publish",
         bvImage(Fix, bvReturn(Ref), bvNoHelper(), bvNoResult(), bvTasksNil(),
                  Clock, bvHalted(Ref)))
endmodule

module SHERLOCK-B-VALIDATE-EVENTS
  imports SHERLOCK-B-VALIDATE-FUNCTIONS
  syntax BVOptStatus ::= bvPhase(BVHelper) [function]
  rule bvPhase(bvNoHelper()) => bvNoStatus()
  rule bvPhase(bvValidate(_, _, _, _, Status)) => bvSomeStatus(Status)
  syntax BVOptInt ::= bvCursor(BVHelper, BVTasks) [function]
  rule bvCursor(bvNoHelper(), _) => bvNoInt()
  rule bvCursor(bvValidate(_, _, _, _, _), Tasks) => bvSomeInt(bvTaskLive(Tasks))
  syntax BVOptInt ::= bvOrigin(BVHelper, BVHelper, BVResult, Int) [function]
  rule bvOrigin(bvValidate(_, Origin, _, _, _), _, _, _) => bvSomeInt(Origin)
  rule bvOrigin(bvNoHelper(), bvValidate(_, Origin, _, _, _), _, _) => bvSomeInt(Origin)
  rule bvOrigin(bvNoHelper(), bvNoHelper(), bvValidated(_), R) => bvSomeInt(R)
  rule bvOrigin(bvNoHelper(), bvNoHelper(), bvNoResult(), _) => bvNoInt()
  syntax BVOptTag ::= bvResultTag(BVResult, BVResult) [function]
  rule bvResultTag(bvValidated(_), _) => bvTag("Validated")
  rule bvResultTag(bvNoResult(), bvValidated(_)) => bvTag("Validated")
  rule bvResultTag(bvNoResult(), bvNoResult()) => bvNoTag()
  syntax BVOptTag ::= bvSite(String) [function] | bvIntervalKind(String) [function]
  rule bvSite("Publish") => bvNoTag()
  rule bvSite(K) => bvTag("EvalLiteral") requires K =/=String "Publish"
  rule bvIntervalKind("Publish") => bvNoTag()
  rule bvIntervalKind(K) => bvTag("Validate") requires K =/=String "Publish"
  syntax String ::= bvDisposition(String) [function] | bvBoundary(BVOutcome) [function]
  rule bvDisposition("Publish") => "Published"
  rule bvDisposition(K) => "Applied" requires K =/=String "Publish"
  rule bvBoundary(bvRunning()) => "None"
  rule bvBoundary(bvHalted(_)) => "Terminal"
  syntax BVEvent ::= bvRecord(String, String, BVImage, BVImage) [function]
  rule bvRecord(Row, Kind,
      bvImage(Fix, PreControl, PreHelper, PreResult, PreTasks,
              bvClocks(F, C, H, V, R), PreOutcome),
      bvImage(Fix, PostControl, PostHelper, PostResult, PostTasks,
              bvClocks(F2, C2, H2, V2, R2), PostOutcome))
    => bvEvent(
      bvStepId(bvRun("sherlock-helper-microstep/1", "DedicatedSorts", "Absent", Fix), C),
      Row, Kind, bvSite(Kind), bvIntervalKind(Kind), bvDisposition(Kind),
      bvDeltas(C2 -Int C, H2 -Int H, V2 -Int V, R2 -Int R, 0, 0, 0, 0),
      0, 0,
      bvTaskObservation(bvTaskLive(PreTasks), bvTaskLive(PostTasks),
        bvPhase(PreHelper), bvPhase(PostHelper),
        bvCursor(PreHelper, PreTasks), bvCursor(PostHelper, PostTasks)),
      bvResultTag(PreResult, PostResult),
      bvOrigin(PreHelper, PostHelper, PreResult, R), 0, bvNoTag(),
      PostOutcome, bvBoundary(PostOutcome),
      bvImage(Fix, PreControl, PreHelper, PreResult, PreTasks,
               bvClocks(F, C, H, V, R), PreOutcome),
      bvImage(Fix, PostControl, PostHelper, PostResult, PostTasks,
               bvClocks(F2, C2, H2, V2, R2), PostOutcome))
  syntax BVExecution ::= bvAdvance(BVImage, BVImage, BVEvents, BVTransition) [function]
  rule bvAdvance(Initial, Pre, Events, bvTransition(Row, Kind, Post))
    => bvExecution(Initial, Post,
         bvEventsCons(bvRecord(Row, Kind, Pre, Post), Events))
  // The captured event list is newest first; reverse for chronological JSONL.
endmodule

module SHERLOCK-B-VALIDATE
  imports SHERLOCK-B-VALIDATE-EVENTS
  configuration <k> $PGM:BVRequest </k>
  rule <k> Request:BVRequest
        => bvExecution(bvGenesis(Request), bvGenesis(Request), bvEventsNil()) </k>
    requires bvNormal(Request)
  rule <k> bvExecution(Initial,
             bvImage(Fix, Ctrl, Helper, Res, Tasks, Clocks, bvRunning()), Events)
        => bvAdvance(Initial,
             bvImage(Fix, Ctrl, Helper, Res, Tasks, Clocks, bvRunning()), Events,
             bvNext(bvImage(Fix, Ctrl, Helper, Res, Tasks, Clocks, bvRunning()))) </k>
  rule <k> bvExecution(Initial,
             bvImage(Fix, Ctrl, Helper, Res, Tasks, Clocks, bvHalted(Ref)), Events)
        => bvAnswer(Initial,
             bvImage(Fix, Ctrl, Helper, Res, Tasks, Clocks, bvHalted(Ref)), Events) </k>
endmodule
