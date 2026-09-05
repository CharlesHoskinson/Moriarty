module candidate_a_test {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_harness.* from "./candidate_a_harness"

  run initialTraceEmptyTest = init.expect(swapTrace == swapInitial
    and swapTrace.records == List() and swapTrace.agreement == emptyAState(N6)
    and not(aliceDeposited or bobDeposited or swapSettled or swapVoluntaryRefunded
      or swapDeadlineCommitted or swapDeadlineInputRejected or swapAliceOnlyTimedOut or swapEmptyTimedOut))
  run initialScheduleTest = assert(canRequest(swapInitial, aliceFunding, Time1)
    and canRequest(swapInitial, NoAInput, Time100)
    and not(canRequest(swapInitial, bobFunding, Time2)))
  run initialSafetyTest = init.expect(coreTraceSafety and not(swapTerminal))
  run depositRecordsActualRequestTest = {
    val changed = applyRequest(swapInitial, aliceFunding, Time1)
    assert(changed.records.length() == 1 and changed.agreement.continuation == N5
      and changed.records.head().request == requestFor(swapInitial, aliceFunding, Time1))
  }
  run swapSettlementTraceTest = init.then(depositAlice).then(depositBob).then(settleSwap)
    .expect(swapSettled and coreTraceSafety and swapTerminal and swapTrace.records.length() == 3
      and balance(swapTrace.ledger, Wallet(Bob), TokenA) == 10
      and balance(swapTrace.ledger, Wallet(Alice), TokenB) == 20)
  run swapVoluntaryRefundTraceTest = init.then(depositAlice).then(depositBob).then(refundSwap)
    .expect(swapVoluntaryRefunded and coreTraceSafety and swapTerminal
      and swapTrace.ledger == initialLedger(10, 20))
  run swapAliceOnlyTimeoutTest = init.then(depositAlice).then(timeoutAlice)
    .expect(swapAliceOnlyTimedOut and coreTraceSafety and swapTerminal
      and swapTrace.records.length() == 2 and swapTrace.ledger == initialLedger(10, 20))
  run swapEmptyTimeoutTest = init.then(timeoutEmpty)
    .expect(swapEmptyTimedOut and coreTraceSafety and swapTerminal
      and swapTrace.records.length() == 1 and swapTrace.ledger == initialLedger(10, 20))
  run swapFundedTimeoutCommitTest = init.then(depositAlice).then(depositBob).then(timeoutFunded)
    .expect(swapDeadlineCommitted and coreTraceSafety and swapTerminal
      and swapTrace.ledger == initialLedger(10, 20))
  run swapFundedTimeoutInputRollbackTest = init.then(depositAlice).then(depositBob).then(timeoutInput)
    .expect(swapDeadlineInputRejected and coreTraceSafety and not(swapTerminal)
      and swapTrace.agreement.continuation == N4 and swapTrace.agreement.minimumTime == Time2
      and balance(swapTrace.ledger, Escrow(aliceA), TokenA) == 10
      and balance(swapTrace.ledger, Escrow(bobB), TokenB) == 20)
  run rejectedAttemptRetainedAfterCleanupTest = init.then(depositAlice).then(depositBob)
    .then(timeoutInput).then(timeoutFunded)
    .expect(swapDeadlineInputRejected and swapDeadlineCommitted and coreTraceSafety and swapTerminal
      and swapTrace.records.length() == 4 and swapTrace.ledger == initialLedger(10, 20))
  run depositWithoutPaymentEffectTest = init.then(depositAlice).expect(aliceDeposited and coreTraceSafety
    and swapTrace.records.length() == 1 and match swapTrace.records.head().payload {
      | SwapComputedA(payload) => payload.raw.payments == List() and payload.raw.reductions == 0
          and payload.effects == List({source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10})
      | _ => false
    })
  run rollbackRecordExactTest = init.then(depositAlice).then(depositBob).then(timeoutInput).expect(
    match swapTrace.records.nth(2).payload {
      | SwapComputedA(payload) => payload.raw == {accepted: false, state: swapTrace.records.nth(2).request.before,
          error: CoreErrorCode("contract_closed"), payments: List(), warnings: List(), reductions: 0}
          and payload.effects == List()
          and swapTrace.records.nth(2).beforeLedger == swapTrace.records.nth(2).afterLedger
      | _ => false
    })
  run terminalHasNoEnabledActionTest = init.then(depositAlice).then(depositBob).then(settleSwap)
    .expect(swapTerminal and not(traceEnabled(swapTrace)))
  run rejectionOnlyCleanupEnabledTest = init.then(depositAlice).then(depositBob).then(timeoutInput).expect(
    canRequest(swapTrace, NoAInput, Time100) and not(canRequest(swapTrace, settleInput, Time2))
      and not(canRequest(swapTrace, refundInput, Time2)) and not(canRequest(swapTrace, settleInput, Time100)))
  run wrongBobTimeDisabledTest = init.then(depositAlice).expect(
    not(canRequest(swapTrace, bobFunding, Time1)) and canRequest(swapTrace, bobFunding, Time2))
  run ledgerTamperDetectedTest = init.then(depositAlice).expect(not(traceSafety({...swapTrace,
    ledger: swapTrace.ledger.put((Wallet(Mallory), TokenA), 1)})))
  run tracePayloadTamperDetectedTest = init.then(depositAlice).expect(match swapTrace.records.head().payload {
    | SwapComputedA(payload) => {
        val original = swapTrace.records.head()
        val changedRaw = {...original, payload: SwapComputedA({...payload, raw: {...payload.raw, reductions: 1}})}
        val changedProjection = {...original, payload: SwapComputedA({...payload,
          projection: {...payload.projection, reductions: 1}})}
        val changedEffects = {...original, payload: SwapComputedA({...payload, effects: List()})}
        val changedRequest = {...original, request: {...original.request, input: bobFunding}}
        Set(changedRaw, changedProjection, changedEffects, changedRequest).forall(changed =>
          not(traceSafety({...swapTrace, records: List(changed)})))
      }
    | _ => false
  })
  run diagnosticNeverWitnessOrTerminalTest = init.then(depositAlice).expect(match swapTrace.records.head().payload {
    | SwapComputedA(payload) => {
        val original = swapTrace.records.head()
        val failedComputation = {...original, payload: SwapComputationDiagnosticA(TransactionReductionBoundFailureA)}
        val failedLedger = {...original, payload: SwapLedgerDiagnosticA(payload)}
        Set(failedComputation, failedLedger).forall(changed => {
          val bad = {...swapTrace, records: List(changed)}
          not(traceSafety(bad)) and not(traceTerminal(bad)) and not(recorded(bad, aliceFunding, Time1, N6, true))
        })
      }
    | _ => false
  })
}
