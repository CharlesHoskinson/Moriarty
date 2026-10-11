module lifecycle_test {
  import lifecycle.* from "./lifecycle"
  run reservationTest = init.then(reserve(0, 4)).expect(state.available == 2 and state.held == 4 and safety)
  run concurrentReservationCapTest = init.then(reserve(0, 4)).expect(state.available == 2 and state.requests.get(1).phase == Idle).then(reserve(1, 3).fail())
  run duplicateReservationTest = init.then(reserve(0, 3)).expect(state.held == 3).then(reserve(0, 3).fail())
  run timeoutIsNotUnusedTest = init.then(reserve(0, 3)).then(dispatch(0)).then(timeout(0)).expect(state.held == 3 and state.available == 3 and state.requests.get(0).duty == 10).then(recover(0).fail())
  run lateSuccessTest = init.then(reserve(0, 3)).then(dispatch(0)).then(timeout(0)).then(settle(0, 10, true)).expect(lateSuccessWitness and state.spent == 3 and state.available == 3 and safety)
  run partialSuccessPreservesDutyTest = init.then(reserve(0, 3)).then(dispatch(0)).then(settle(0, 3, true)).expect(state.requests.get(0).duty == 7 and residualDutyWitness and safety)
  run provedUnusedRecoveryTest = init.then(reserve(0, 3)).then(dispatch(0)).then(timeout(0)).then(proveUnused(0, true)).expect(state.held == 3).then(recover(0)).expect(state.available == 6 and state.requests.get(0).duty == 10 and recoveredWitness)
  run refundPreventsLateSuccessTest = init.then(reserve(0, 3)).then(dispatch(0)).then(timeout(0)).then(proveUnused(0, true)).then(recover(0)).expect(state.spent == 0 and state.requests.get(0).refunded == 3).then(settle(0, 10, true).fail())
  run successPreventsRefundTest = init.then(reserve(0, 3)).then(dispatch(0)).then(settle(0, 10, true)).expect(state.available == 3 and not(canProveUnused(state, 0, true)) and not(canRecover(state, 0))).then(recover(0).fail())
  run falseSuccessReceiptTest = init.then(reserve(0, 3)).then(dispatch(0)).expect(state.held == 3).then(settle(0, 10, false).fail())
  run falseUnusedReceiptTest = init.then(reserve(0, 3)).then(dispatch(0)).then(timeout(0)).expect(state.requests.get(0).phase == Unknown).then(proveUnused(0, false).fail())
}
