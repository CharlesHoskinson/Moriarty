requires "../source/envget-miss-hit-exit-step.k"

module SHERLOCK-ENVGET-MISS-HIT-EXIT-LAWS
  imports SHERLOCK-ENVGET-MISS-HIT-EXIT-STEP

  // Source laws only, test-first relative to this new source and UNRUN.
  // They are parameterized over a complete admitted-form EGSnapshot; the frozen
  // algebraic seeds supply binder/cursor/env/status patterns, not concrete
  // admission, run, ordinal, or expected whole-state literals.

  // Two linked cursors and whole-carrier preservation: the after cursor is
  // egMissCursor of the before cursor; common, binder, found, Pending, result,
  // tasks, tables, frames inside Root, seg-base fields inside Root, admission,
  // and run identity are the identical terms except egChargeRoot(Root, 1).
  syntax Bool ::= egMissPreservesWholeCarrier(EGSnapshot) [function]
  rule egMissPreservesWholeCarrier(egSnapshot(A, R,
    csOwnedState(Root, Tables, csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
      Cursor, Found, chTriPending())), Result, Tasks))))
    => egCellMiss(egSnapshot(A, R, csOwnedState(Root, Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))))
       ==K egSnapshot(A, R, csOwnedState(egChargeRoot(Root, 1), Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
           egMissCursor(Cursor, Binder, Tables), Found, chTriPending())), Result, Tasks)))

  // previous=None is a valid miss branch: cursor becomes chIndexNone, status
  // remains Pending, and no Negative, EnvGetEmpty, or failure term is produced.
  // This branch alone does not qualify the snapshot for the follow-up hit.
  syntax Bool ::= egMissPreviousNoneRemainsPending(EGSnapshot) [function]
  rule egMissPreviousNoneRemainsPending(egSnapshot(A, R,
    csOwnedState(Root, Tables, csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
      Cursor, Found, chTriPending())), Result, Tasks))))
    => egCellMiss(egSnapshot(A, R, csOwnedState(Root, Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))))
       ==K egSnapshot(A, R, csOwnedState(egChargeRoot(Root, 1), Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
           chIndexNone(), Found, chTriPending())), Result, Tasks)))
    requires egMissCursor(Cursor, Binder, Tables) ==K chIndexNone()

  // Nonzero visited index whose previous is Some(0): Some(0) is preserved as a
  // present cursor and is distinct from chIndexNone.
  syntax Bool ::= egMissNonzeroToSomeZero(EGSnapshot) [function]
  rule egMissNonzeroToSomeZero(egSnapshot(A, R,
    csOwnedState(Root, Tables, csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
      chIndexSome(Index), Found, chTriPending())), Result, Tasks))))
    => egCellMiss(egSnapshot(A, R, csOwnedState(Root, Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, chIndexSome(Index), Found, chTriPending())), Result, Tasks))))
       ==K egSnapshot(A, R, csOwnedState(egChargeRoot(Root, 1), Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
           chIndexSome(0), Found, chTriPending())), Result, Tasks)))
    requires Index >Int 0
     andBool egMissCursor(chIndexSome(Index), Binder, Tables) ==K chIndexSome(0)

  // Independently paired miss event: cursor pair, Pending -> Pending phase pair,
  // owner work/helper-work/visit/reference accounting 1/1/1/0, zero allocation
  // and value-slot writes, unchanged frame/task counts, preimage consumed-work
  // ordinal, original control origin and source occurrence, Running outcome,
  // and unit egNoBoundary distinct from any CHIndex None.
  syntax Bool ::= egMissEventCursorAndAccounting(EGSnapshot) [function]
  rule egMissEventCursorAndAccounting(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(Control, Origin, EnvRoot, Stack, ctRunning(), Initial,
        Remaining, Consumed, HelperWork, Visits, Reference, Alloc, Active,
        SegActive, SegAlloc, SegConsumed, Frames),
        Tables, csHelperTables(chSomeHelper(chEnvGet(chCommon(chSiteEvalVar(), ReferenceOrigin, ParentResume),
          Binder, chIndexSome(Cursor), Found, chTriPending())), Result, Tasks))))
    => egMissEvent(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(Control, Origin, EnvRoot, Stack, ctRunning(), Initial,
        Remaining, Consumed, HelperWork, Visits, Reference, Alloc, Active,
        SegActive, SegAlloc, SegConsumed, Frames),
        Tables, csHelperTables(chSomeHelper(chEnvGet(chCommon(chSiteEvalVar(), ReferenceOrigin, ParentResume),
          Binder, chIndexSome(Cursor), Found, chTriPending())), Result, Tasks))))
       ==K egEvent(egStepId(RunId, Consumed), egEnvGetCellRule(), egHelperVisit(),
         egSiteSome(chSiteEvalVar()), egSomeEnvGet(), egApplied(), 1, 1, 1, 0,
         ctAlloc(0, 0, 0), 0, egFrameCount(Frames), egFrameCount(Frames),
         egTaskCount(Tasks), egTaskCount(Tasks), egTriPhase(chTriPending()),
         egTriPhase(chTriPending()), chIndexSome(Cursor),
         egMissCursor(chIndexSome(Cursor), Binder, Tables),
         egNoResultTag(), egNatSome(ReferenceOrigin), Origin,
         egSourceOccurrence(Admission, Origin), egNoExhaustion(),
         egRunningTag(), ctRunning(), egNoBoundary())

  // Linked three-event composition: every after snapshot and every event is
  // computed from the preceding computed snapshot, never independently supplied.
  syntax Bool ::= egTraversalLinked(EGTraversalWindow) [function]
  rule egTraversalLinked(egTraversalWindow(Before, AfterMiss, AfterHit, AfterExit, EventMiss, EventHit, EventExit))
    => AfterMiss ==K egCellMiss(Before) andBool AfterHit ==K egCellHit(AfterMiss)
       andBool AfterExit ==K egExit(AfterHit) andBool EventMiss ==K egMissEvent(Before)
       andBool EventHit ==K egCellEvent(AfterMiss) andBool EventExit ==K egExitEvent(AfterHit)

  // Post-review owner-authored discriminatory miss law, UNRUN. This predicate
  // was added after the fresh full review identified the shared-helper masking
  // gap in the five laws above; it is not a retroactive independent prefreeze
  // law and it does not alter the original SAFE input seeds or their chronology.
  // Author-visible post-review input seed only: requested binder 31, cursor
  // chIndexSome(2), found chRefNone(), status Pending, and environment rows in
  // exact order row0 dpEnvCell(31, ctConst(0), ctEnvNone()), row1
  // dpEnvCell(31, ctData(1), ctEnvNone()), row2
  // dpEnvCell(47, ctConst(2), ctEnvSome(0)). Row1 is an off-chain matching
  // binder trap; the visited row2 previous link goes directly to row0. This is
  // an UNADMITTED algebraic source fixture, not an admitted machine image, an
  // actual prestate, or an independent semantic golden. The expected side is a
  // whole reconstructed EGSnapshot: it uses the literal expected cursor
  // chIndexSome(0) and a fully expanded ctRoot, and it uses neither shared
  // miss-cursor helper nor shared charge helper, and no cursor index
  // arithmetic or search derivation. Admission, run identity, all tables
  // including data fields and value slots, result, tasks, common, binder,
  // found, and Pending are preserved term-identically.
  syntax Bool ::= egMissOffChainPreviousDiscriminator(EGSnapshot) [function]
  rule egMissOffChainPreviousDiscriminator(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(Control, Origin, EnvRoot, Stack, ctRunning(), Initial,
        Remaining, Consumed, HelperWork, Visits, Reference, Alloc, Active,
        SegActive, SegAlloc, SegConsumed, Frames),
        dpTables(Nodes, Refs,
          dpEnv(dpEnvCell(31, ctConst(0), ctEnvNone()),
            dpEnv(dpEnvCell(31, ctData(1), ctEnvNone()),
              dpEnv(dpEnvCell(47, ctConst(2), ctEnvSome(0)), dpNoEnvs()))),
          ValueSlots),
        csHelperTables(chSomeHelper(chEnvGet(Common, 31,
          chIndexSome(2), chRefNone(), chTriPending())), Result, Tasks))))
    => egCellMiss(egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(Control, Origin, EnvRoot, Stack, ctRunning(), Initial,
        Remaining, Consumed, HelperWork, Visits, Reference, Alloc, Active,
        SegActive, SegAlloc, SegConsumed, Frames),
        dpTables(Nodes, Refs,
          dpEnv(dpEnvCell(31, ctConst(0), ctEnvNone()),
            dpEnv(dpEnvCell(31, ctData(1), ctEnvNone()),
              dpEnv(dpEnvCell(47, ctConst(2), ctEnvSome(0)), dpNoEnvs()))),
          ValueSlots),
        csHelperTables(chSomeHelper(chEnvGet(Common, 31,
          chIndexSome(2), chRefNone(), chTriPending())), Result, Tasks))))
       ==K egSnapshot(Admission, RunId,
      csOwnedState(ctRoot(Control, Origin, EnvRoot, Stack, ctRunning(), Initial,
        Remaining -Int 1, Consumed +Int 1, HelperWork +Int 1,
        Visits +Int 1, Reference, Alloc, Active,
        SegActive, SegAlloc, SegConsumed, Frames),
        dpTables(Nodes, Refs,
          dpEnv(dpEnvCell(31, ctConst(0), ctEnvNone()),
            dpEnv(dpEnvCell(31, ctData(1), ctEnvNone()),
              dpEnv(dpEnvCell(47, ctConst(2), ctEnvSome(0)), dpNoEnvs()))),
          ValueSlots),
        csHelperTables(chSomeHelper(chEnvGet(Common, 31,
          chIndexSome(0), chRefNone(), chTriPending())), Result, Tasks)))
    requires Remaining >Int 0
endmodule
