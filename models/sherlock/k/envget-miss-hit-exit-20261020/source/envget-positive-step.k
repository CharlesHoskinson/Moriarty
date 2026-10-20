requires "envget-positive-syntax.k"

module SHERLOCK-ENVGET-POSITIVE-STEP
  imports SHERLOCK-ENVGET-POSITIVE-SYNTAX
  imports K-EQUAL

  // CONDITIONAL NORMAL APPLIED RELATION ONLY. Real admission, StepInv,
  // selection/validation/physical checks and refusal absence are premises.
  // No boolean/token asserts them; no dispatch, guard or refusal is replaced.

  syntax EGJSON ::= egGet(String, EGFields) [function]
  rule egGet(Key, egField(Key, Value, _)) => Value
  rule egGet(Key, egField(Other, _, Rest)) => egGet(Key, Rest)
    requires Key =/=String Other
  syntax EGFields ::= egFields(EGJSON) [function]
  rule egFields(egObject(Fields)) => Fields
  syntax EGValues ::= egValues(EGJSON) [function]
  rule egValues(egArray(Values)) => Values
  syntax Int ::= egIntegerValue(EGJSON) [function]
  rule egIntegerValue(egInteger(Value)) => Value
  syntax Bool ::= egBooleanValue(EGJSON) [function]
  rule egBooleanValue(egBoolean(Value)) => Value
  syntax EGJSON ::= egAt(Int, EGValues) [function]
  rule egAt(0, egValue(Value, _)) => Value
  rule egAt(Index, egValue(_, Rest)) => egAt(Index -Int 1, Rest)
    requires Index >Int 0
  syntax EGValues ::= egDrop(Int, EGValues) [function]
  rule egDrop(0, Values) => Values
  rule egDrop(Count, egValue(_, Rest)) => egDrop(Count -Int 1, Rest)
    requires Count >Int 0
  syntax EGValues ::= egTake(Int, EGValues) [function]
  rule egTake(0, _) => egNoValues()
  rule egTake(Count, egValue(Value, Rest))
    => egValue(Value, egTake(Count -Int 1, Rest)) requires Count >Int 0

  syntax EGOccurrence ::= egSourceOccurrence(EGJSON, Int) [function]
  rule egSourceOccurrence(egObject(Admission), Origin)
    => egSourceFromView(egGet("view", Admission), Origin)
  syntax EGOccurrence ::= egSourceFromView(EGJSON, Int) [function]
  rule egSourceFromView(egObject(View), Origin)
    => egSourceFromRow(egAt(Origin, egValues(egGet("occurrences", View))),
                       egValues(egGet("occurrence_paths", View)))
  syntax EGOccurrence ::= egSourceFromRow(EGJSON, EGValues) [function]
  rule egSourceFromRow(egObject(Row), Paths)
    => egOccurrenceLocator(egBooleanValue(egGet("owner_is_function", Row)),
         egIntegerValue(egGet("function", Row)),
         egTake(egIntegerValue(egGet("len", egFields(egGet("path", Row)))),
           egDrop(egIntegerValue(egGet("start", egFields(egGet("path", Row)))), Paths)))

  syntax Int ::= egFrameCount(CTFrames) [function]
  rule egFrameCount(ctNoFrames()) => 0
  rule egFrameCount(ctFrame(_, Rest)) => 1 +Int egFrameCount(Rest)
  syntax Int ::= egTaskCount(CSTasks) [function]
  rule egTaskCount(csNoTasks()) => 0
  rule egTaskCount(csTask(_, Rest)) => 1 +Int egTaskCount(Rest)
  syntax DPEnvCell ::= egEnvAt(Int, DPEnvs) [function]
  rule egEnvAt(0, dpEnv(Cell, _)) => Cell
  rule egEnvAt(Index, dpEnv(_, Rest)) => egEnvAt(Index -Int 1, Rest)
    requires Index >Int 0
  syntax CTRef ::= egHitRef(CHIndex, Int, DPTables) [function]
  rule egHitRef(chIndexSome(Index), Binder, dpTables(_, _, Env, _))
    => egMatchedRef(Binder, egEnvAt(Index, Env))
  syntax CTRef ::= egMatchedRef(Int, DPEnvCell) [function]
  rule egMatchedRef(Binder, dpEnvCell(Binder, Value, _)) => Value

  // Same normal P4 update for both rows; only Cell adds a P7 helper visit.
  // All root fields not explicitly changed remain the identical typed terms.
  syntax CTRoot ::= egChargeRoot(CTRoot, Int) [function]
  rule egChargeRoot(ctRoot(Control, Origin, Env, Stack, ctRunning(), Initial,
      Remaining, Consumed, HelperWork, Visits, Reference, Alloc, Active,
      SegActive, SegAlloc, SegConsumed, Frames), VisitDelta)
    => ctRoot(Control, Origin, Env, Stack, ctRunning(), Initial,
      Remaining -Int 1, Consumed +Int 1, HelperWork +Int 1,
      Visits +Int VisitDelta, Reference, Alloc, Active,
      SegActive, SegAlloc, SegConsumed, Frames)
    requires Remaining >Int 0

  syntax EGSnapshot ::= egCellHit(EGSnapshot) [function]
  rule egCellHit(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))))
    => egCellWithValue(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))),
      egHitRef(Cursor, Binder, Tables))
  syntax EGSnapshot ::= egCellWithValue(EGSnapshot, CTRef) [function]
  rule egCellWithValue(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, _, chTriPending())), Result, Tasks))), Value)
    => egSnapshot(Admission, RunId,
      csOwnedState(egChargeRoot(Root, 1), Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, chRefSome(Value), chTriPositive())), Result, Tasks)))

  syntax EGSnapshot ::= egExit(EGSnapshot) [function]
  rule egExit(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(_, _, _, chRefSome(Value), chTriPositive())), _, Tasks))))
    => egSnapshot(Admission, RunId,
      csOwnedState(egChargeRoot(Root, 0), Tables,
        csHelperTables(chNoHelper(), chSomeResult(chResultEnvFound(Value)), Tasks)))

  // Complete events use preimage consumedWork/source origin/common fields.
  // The run identity is retained whole, not rehashed or reconstructed.
  syntax EGEvent ::= egCellEvent(EGSnapshot) [function]
  rule egCellEvent(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(_, Origin, _, _, ctRunning(), _, _, Consumed, _, _, _, _, _, _, _, _, Frames),
        _, csHelperTables(chSomeHelper(chEnvGet(chCommon(chSiteEvalVar(), ReferenceOrigin, _),
          _, chIndexSome(Cursor), _, chTriPending())), _, Tasks))))
    => egEvent(egStepId(RunId, Consumed), egEnvGetCellRule(), egHelperVisit(),
      egSiteSome(chSiteEvalVar()), egSomeEnvGet(), egApplied(), 1, 1, 1, 0,
      ctAlloc(0, 0, 0), 0, egFrameCount(Frames), egFrameCount(Frames),
      egTaskCount(Tasks), egTaskCount(Tasks), egTriPhase(chTriPending()),
      egTriPhase(chTriPositive()), chIndexSome(Cursor), chIndexSome(Cursor),
      egNoResultTag(), egNatSome(ReferenceOrigin), Origin,
      egSourceOccurrence(Admission, Origin), egNoExhaustion(),
      egRunningTag(), ctRunning(), egNoBoundary())

  syntax EGEvent ::= egExitEvent(EGSnapshot) [function]
  rule egExitEvent(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(_, Origin, _, _, ctRunning(), _, _, Consumed, _, _, _, _, _, _, _, _, Frames),
        _, csHelperTables(chSomeHelper(chEnvGet(chCommon(chSiteEvalVar(), ReferenceOrigin, _),
          _, Cursor, chRefSome(_), chTriPositive())), _, Tasks))))
    => egEvent(egStepId(RunId, Consumed), egEnvGetExitRule(), egHelperExit(),
      egSiteSome(chSiteEvalVar()), egSomeEnvGet(), egApplied(), 1, 1, 0, 0,
      ctAlloc(0, 0, 0), 0, egFrameCount(Frames), egFrameCount(Frames),
      egTaskCount(Tasks), egTaskCount(Tasks), egTriPhase(chTriPositive()),
      egNoPhase(), Cursor, chIndexNone(), egEnvFoundTag(),
      egNatSome(ReferenceOrigin), Origin, egSourceOccurrence(Admission, Origin),
      egNoExhaustion(), egRunningTag(), ctRunning(), egNoBoundary())

  // Linked computation: AfterExit is obtained from the computed AfterCell.
  syntax EGWindow ::= egTwoEventWindow(EGSnapshot) [function]
  rule egTwoEventWindow(Before) => egWindowAfterCell(Before, egCellHit(Before))
  syntax EGWindow ::= egWindowAfterCell(EGSnapshot, EGSnapshot) [function]
  rule egWindowAfterCell(Before, Middle)
    => egWindow(Before, Middle, egExit(Middle), egCellEvent(Before), egExitEvent(Middle))
endmodule
