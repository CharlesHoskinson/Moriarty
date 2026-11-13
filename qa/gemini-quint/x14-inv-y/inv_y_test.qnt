// Unit and property tests for the X14 BLS12-381 G1 doubling condition row `INV_y` model.

module inv_y_test {
  import inv_y_chain(
    HAS_INV_ROW = true,
    INV_RECEIVES = true,
    WRONG_DEN = false,
    O10_HAS_ID = true,
    INV_VALID_FREE = false
  ).* from "./inv_y"

  run honestCompletenessTest = init
    .expect(and {
      onCurve(2, 5),
      onCurve(4, 4),
      chainAccepted(cfgSound, st),
      invRow,
      st.sink_x3 == 12,
      st.sink_y3 == 9,
      onCurve(st.sink_x3, st.sink_y3),
    })

  run uncheckedPointsInReachesChainTest = init
    .then(stepZeroY)
    .expect(and {
      not(onCurve(st.pin_x, st.pin_y)),
      inFp(st.pin_x),
      inFp(st.pin_y),
      not(chainAccepted(cfgSound, st)),
      invRow,
    })

  run soundRejectsZeroYForgeryTest = init
    .then(stepZeroY)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      invRow,
      FP.forall(w => not(chainAccepted(cfgSound, mkZeroYState(5, 0, w, 1)))),
      FP.forall(w => not(chainAccepted(cfgSound, mkZeroYState(5, 0, w, 0)))),
      FP.forall(w => checkInvRow(cfgSound, mkZeroYState(5, 0, w, 1))),
      FP.forall(w => checkInvRow(cfgSound, mkZeroYState(5, 0, w, 0))),
    })

  run soundRejectsSingleCellFaultsTest = init
    .then(stepMutateCell)
    .expect(and {
      invRow,
      CELLS.forall(c => FP.forall(v => checkInvRow(cfgSound, mutateCell(c, v)))),
    })

  run soundRejectsBusDropDupTest = init
    .then(stepBusFault)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      invRow,
    })

  run mutantNoInvRowCaughtTest = init
    .then(stepZeroY)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      chainAccepted(cfgNoInvRow, st),
      not(checkInvRow(cfgNoInvRow, st)),
    })

  run mutantNoRecvCaughtTest = init
    .then(stepZeroYDecoupled)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      chainAccepted(cfgNoRecv, st),
      not(checkInvRow(cfgNoRecv, st)),
    })

  run mutantWrongDenCaughtTest = init
    .then(stepZeroYWrongDen)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      chainAccepted(cfgWrongDen, st),
      not(checkInvRow(cfgWrongDen, st)),
    })

  run mutantNoIdCaughtTest = init
    .then(stepSwapId)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      chainAccepted(cfgNoId, st),
      not(checkInvRow(cfgNoId, st)),
    })

  run mutantInvValidFreeCaughtTest = init
    .then(stepZeroYInactive)
    .expect(and {
      not(chainAccepted(cfgSound, st)),
      chainAccepted(cfgInvValidFree, st),
      not(checkInvRow(cfgInvValidFree, st)),
    })
}
