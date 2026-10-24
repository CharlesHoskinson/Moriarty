requires "envget-miss-hit-exit-syntax.k"
requires "envget-positive-step.k"

module SHERLOCK-ENVGET-MISS-HIT-EXIT-STEP
  imports SHERLOCK-ENVGET-MISS-HIT-EXIT-SYNTAX
  imports SHERLOCK-ENVGET-POSITIVE-STEP
  imports K-EQUAL

  // CTEnv previous link to CHIndex cursor. ctEnvNone and chIndexNone are
  // Option-None carriers; EGBoundary egNoBoundary remains a distinct unit sort.
  syntax CHIndex ::= egEnvIndex(CTEnv) [function]
  rule egEnvIndex(ctEnvNone()) => chIndexNone()
  rule egEnvIndex(ctEnvSome(N)) => chIndexSome(N)

  // A miss is defined only for a visited cell whose binder differs from the
  // requested binder. No Negative status, Empty row, or failure is injected.
  syntax CHIndex ::= egPreviousOfCell(Int, DPEnvCell) [function]
  rule egPreviousOfCell(Binder, dpEnvCell(CellBinder, _, Previous))
    => egEnvIndex(Previous)
    requires Binder =/=Int CellBinder

  syntax CHIndex ::= egMissCursor(CHIndex, Int, DPTables) [function]
  rule egMissCursor(chIndexSome(Index), Binder, dpTables(_, _, Env, _))
    => egPreviousOfCell(Binder, egEnvAt(Index, Env))

  syntax EGSnapshot ::= egCellMissWithPrevious(EGSnapshot, CHIndex) [function]
  rule egCellMissWithPrevious(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))), NewCursor)
    => egSnapshot(Admission, RunId,
      csOwnedState(egChargeRoot(Root, 1), Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, NewCursor, Found, chTriPending())), Result, Tasks)))

  // General miss: copy the entire EnvGet helper and snapshot carrier; set only
  // helper.cursor to the visited cell's previous link, including None. Common,
  // binder, found, and Pending status are preserved exactly. Owner charge is
  // the existing egChargeRoot with VisitDelta 1: work 1, helper-work 1,
  // helper-visits 1, reference-cursor 0.
  syntax EGSnapshot ::= egCellMiss(EGSnapshot) [function]
  rule egCellMiss(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))))
    => egCellMissWithPrevious(egSnapshot(Admission, RunId,
      csOwnedState(Root, Tables, csHelperTables(
        chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))),
      egMissCursor(Cursor, Binder, Tables))

  // Complete 28-field miss event from the preimage. Pending -> Pending;
  // helper_cursor_before is the preimage Some cursor and helper_cursor_after
  // is the computed visited-cell previous, Some or None. Deltas are
  // work/helper-work/helper-visits/reference-cursor = 1/1/1/0.
  syntax EGEvent ::= egMissEvent(EGSnapshot) [function]
  rule egMissEvent(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(_, Origin, _, _, ctRunning(), _, _, Consumed, _, _, _, _, _, _, _, _, Frames),
        Tables, csHelperTables(chSomeHelper(chEnvGet(chCommon(chSiteEvalVar(), ReferenceOrigin, _),
          Binder, chIndexSome(Cursor), _, chTriPending())), _, Tasks))))
    => egEvent(egStepId(RunId, Consumed), egEnvGetCellRule(), egHelperVisit(),
      egSiteSome(chSiteEvalVar()), egSomeEnvGet(), egApplied(), 1, 1, 1, 0,
      ctAlloc(0, 0, 0), 0, egFrameCount(Frames), egFrameCount(Frames),
      egTaskCount(Tasks), egTaskCount(Tasks), egTriPhase(chTriPending()),
      egTriPhase(chTriPending()), chIndexSome(Cursor),
      egMissCursor(chIndexSome(Cursor), Binder, Tables),
      egNoResultTag(), egNatSome(ReferenceOrigin), Origin,
      egSourceOccurrence(Admission, Origin), egNoExhaustion(),
      egRunningTag(), ctRunning(), egNoBoundary())

  // Fixed sequential composition. The entry receives only the before snapshot.
  // AfterMiss, AfterHit, and AfterExit are computed in that order from the
  // preceding computed state; no afterstate is an entry argument. Hit and Exit
  // reuse the unchanged egCellHit, egExit, egCellEvent, and egExitEvent.
  syntax EGTraversalWindow ::= egThreeEventTraversal(EGSnapshot) [function]
  rule egThreeEventTraversal(Before)
    => egTraversalWindow(Before, egCellMiss(Before), egCellHit(egCellMiss(Before)),
      egExit(egCellHit(egCellMiss(Before))), egMissEvent(Before),
      egCellEvent(egCellMiss(Before)), egExitEvent(egCellHit(egCellMiss(Before))))
endmodule
