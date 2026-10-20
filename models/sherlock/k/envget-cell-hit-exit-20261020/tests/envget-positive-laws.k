requires "../source/envget-positive-step.k"

module SHERLOCK-ENVGET-POSITIVE-LAWS
  imports SHERLOCK-ENVGET-POSITIVE-STEP

  // Written before the implementation; UNRUN. These are general semantic
  // preservation predicates, not private independent expected ledgers.
  syntax Bool ::= egCellPreservesWholeCarrier(EGSnapshot) [function]
  rule egCellPreservesWholeCarrier(egSnapshot(A, R,
    csOwnedState(Root, Tables, csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
      Cursor, Found, chTriPending())), Result, Tasks))))
    => egCellHit(egSnapshot(A, R, csOwnedState(Root, Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, Cursor, Found, chTriPending())), Result, Tasks))))
       ==K egSnapshot(A, R, csOwnedState(egChargeRoot(Root, 1), Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, Cursor,
           chRefSome(egHitRef(Cursor, Binder, Tables)), chTriPositive())), Result, Tasks)))

  syntax Bool ::= egExitPreservesWholeCarrier(EGSnapshot) [function]
  rule egExitPreservesWholeCarrier(egSnapshot(A, R,
    csOwnedState(Root, Tables, csHelperTables(chSomeHelper(chEnvGet(Common, Binder,
      Cursor, chRefSome(Value), chTriPositive())), Result, Tasks))))
    => egExit(egSnapshot(A, R, csOwnedState(Root, Tables,
         csHelperTables(chSomeHelper(chEnvGet(Common, Binder, Cursor, chRefSome(Value), chTriPositive())), Result, Tasks))))
       ==K egSnapshot(A, R, csOwnedState(egChargeRoot(Root, 0), Tables,
         csHelperTables(chNoHelper(), chSomeResult(chResultEnvFound(Value)), Tasks)))

  syntax Bool ::= egJoined(EGWindow) [function]
  rule egJoined(egWindow(Before, Middle, After, Cell, Exit))
    => Middle ==K egCellHit(Before) andBool After ==K egExit(Middle)
       andBool Cell ==K egCellEvent(Before) andBool Exit ==K egExitEvent(Middle)
endmodule
