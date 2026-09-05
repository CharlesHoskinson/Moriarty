module candidate_a_installment_test_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_installment_harness_f.* from "./candidate_a_installment_harness_f"

  pure val expectedAfterFirst: AState = {...installmentInitial.agreement, continuation: N2,
    accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 5),
    choices: A_CHOICE_IDS.mapBy(_ => NoInt).put(FirstFillId, IntValue(1))}
  pure def expectedRefund(before: AState, quantity: int, time: ATime, recovery: bool): ATransactionResult = {
    accepted: true, state: {...before, continuation: N0, minimumTime: time,
      accounts: CORE_ACCOUNTS.mapBy(_ => 0),
      choices: if (recovery) before.choices.put(RecoveryId, IntValue(1)) else before.choices},
    error: NoCoreError, payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: quantity}),
    warnings: List(), reductions: if (recovery) 1 else 2}
  pure def expectedRejection(before: AState, code: str): ATransactionEvaluation = TransactionComputedA({
    accepted: false, state: before, error: CoreErrorCode(code), payments: List(), warnings: List(), reductions: 0})

  run exactInstallmentNodeTableTest = assert(installmentProgram == {root: N4, nodes: closeProgram.nodes
    .put(N1, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))
    .put(N2, WhenA({cases: List(
      {caseAction: ChoiceA({id: SecondFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N1},
      {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
      timeout: Time100, timeoutNode: N0}))
    .put(N3, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N2}))
    .put(N4, WhenA({cases: List(
      {caseAction: ChoiceA({id: FirstFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N3},
      {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
      timeout: Time100, timeoutNode: N0}))})
  run initialFixtureNoWitnessTest = init.expect(installmentTrace == installmentInitial
    and installmentTrace.records == List() and installmentTrace.agreement.minimumTime == Time2
    and not(firstAgreementFill or secondAgreementFill or agreementRefundTen or agreementRefundFive
      or agreementDeadlineTen or agreementDeadlineFive or agreementDeadlineRecoveryTenRejected
      or agreementDeadlineRecoveryFiveRejected))
  run initialEnabledSafetyTest = init.expect(installmentTraceSafety and not(installmentTerminal)
    and canRequest(installmentTrace, fillOneInput, Time2)
    and canRequest(installmentTrace, recoveryInput, Time2)
    and not(canRequest(installmentTrace, fillTwoInput, Time2)))
  run firstFillStopsAtSecondWhenTest = init.then(fillFirst).expect(firstAgreementFill
    and installmentTraceSafety and not(installmentTerminal) and installmentTrace.records.length() == 1
    and installmentTrace.agreement == expectedAfterFirst
    and match installmentTrace.records.head().payload {
      | InstallmentComputedA(payload) => payload.raw == {accepted: true, state: expectedAfterFirst, error: NoCoreError,
          payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 1}
          and payload.effects == List({source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})
      | _ => false
    })
  run secondFillCompletesTest = init.then(fillFirst).then(fillSecond).expect(firstAgreementFill
    and secondAgreementFill and installmentTraceSafety and installmentTerminal
    and installmentTrace.records.length() == 2 and match installmentTrace.records.nth(1).payload {
      | InstallmentComputedA(payload) => payload.raw == {accepted: true,
          state: {...expectedAfterFirst, continuation: N0, accounts: CORE_ACCOUNTS.mapBy(_ => 0),
            choices: expectedAfterFirst.choices.put(SecondFillId, IntValue(1))}, error: NoCoreError,
          payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 1}
          and balance(installmentTrace.ledger, Wallet(Bob), TokenA) == 10
      | _ => false
    })
  run recoveryChoiceRefundsTenTest = init.then(recoverTen).expect(agreementRefundTen
    and installmentTraceSafety and installmentTerminal and match installmentTrace.records.head().payload {
      | InstallmentComputedA(payload) => payload.raw == expectedRefund(installmentInitial.agreement, 10, Time2, true)
      | _ => false
    })
  run recoveryChoiceRefundsFiveTest = init.then(fillFirst).then(recoverFive).expect(agreementRefundFive
    and installmentTraceSafety and installmentTerminal and match installmentTrace.records.nth(1).payload {
      | InstallmentComputedA(payload) => payload.raw == expectedRefund(expectedAfterFirst, 5, Time2, true)
          and balance(installmentTrace.ledger, Wallet(Bob), TokenA) == 5
          and balance(installmentTrace.ledger, Wallet(Alice), TokenA) == 5
      | _ => false
    })
  run initialDeadlineRefundTest = init.then(timeoutTen).expect(agreementDeadlineTen
    and installmentTraceSafety and installmentTerminal and match installmentTrace.records.head().payload {
      | InstallmentComputedA(payload) => payload.raw == expectedRefund(installmentInitial.agreement, 10, Time100, false)
      | _ => false
    })
  run residualDeadlineRefundTest = init.then(fillFirst).then(timeoutFive).expect(agreementDeadlineFive
    and installmentTraceSafety and installmentTerminal and match installmentTrace.records.nth(1).payload {
      | InstallmentComputedA(payload) => payload.raw == expectedRefund(expectedAfterFirst, 5, Time100, false)
      | _ => false
    })
  run installmentTimeoutPriorityTest = init.then(rejectRecoveryTen).expect(
    agreementDeadlineRecoveryTenRejected and installmentTraceSafety and not(installmentTerminal)
      and installmentTrace.agreement == installmentInitial.agreement
      and installmentTrace.ledger == installmentInitial.ledger)
  run residualTimeoutPriorityTest = init.then(fillFirst).then(rejectRecoveryFive).expect(
    agreementDeadlineRecoveryFiveRejected and installmentTraceSafety and not(installmentTerminal)
      and installmentTrace.agreement == expectedAfterFirst
      and balance(installmentTrace.ledger, Wallet(Bob), TokenA) == 5
      and balance(installmentTrace.ledger, Escrow(aliceA), TokenA) == 5)
  run initialRejectedCleanupRetainsAttemptTest = init.then(rejectRecoveryTen).then(timeoutTen).expect(
    agreementDeadlineRecoveryTenRejected and agreementDeadlineTen and installmentTraceSafety
      and installmentTerminal and installmentTrace.records.length() == 2)
  run residualRejectedCleanupRetainsAttemptTest = init.then(fillFirst).then(rejectRecoveryFive).then(timeoutFive).expect(
    agreementDeadlineRecoveryFiveRejected and agreementDeadlineFive and installmentTraceSafety
      and installmentTerminal and installmentTrace.records.length() == 3)
  run cancellationHasNoCoreResultTest = assert(cancellationAdapter(installmentInitial.agreement)
    == CancellationAdaptedA({predecessor: {program: installmentProgram, state: installmentInitial.agreement},
      proposedSuccessor: {program: installmentProgram, state: installmentInitial.agreement},
      effects: List(), coreProjection: NoCoreProjection}))
  run residualCancellationHasNoCoreResultTest = init.then(fillFirst).expect(
    cancellationAdapter(installmentTrace.agreement) == CancellationAdaptedA({
      predecessor: {program: installmentProgram, state: expectedAfterFirst},
      proposedSuccessor: {program: installmentProgram, state: expectedAfterFirst},
      effects: List(), coreProjection: NoCoreProjection}))
  run recoveryLegalWithoutCancellationTest = assert(computeTransaction(installmentProgram,
    installmentInitial.agreement, recoveryInput, Time2)
      == TransactionComputedA(expectedRefund(installmentInitial.agreement, 10, Time2, true)))
  run deadline101CommitAndRollbackTest = assert(Set(installmentInitial.agreement, expectedAfterFirst).forall(before =>
    computeTransaction(installmentProgram, before, NoAInput, Time101)
      == TransactionComputedA(expectedRefund(before, before.accounts.get(aliceA), Time101, false))
    and computeTransaction(installmentProgram, before, recoveryInput, Time101)
      == expectedRejection(before, "contract_closed")))
  run invalidInstallmentRequestsRollbackTest = assert(Set(installmentInitial.agreement, expectedAfterFirst).forall(before => {
    val phase = if (before.continuation == N4) FirstFillId else SecondFillId
    val wrongPhase = if (before.continuation == N4) fillTwoInput else fillOneInput
    val correctInput = if (before.continuation == N4) fillOneInput else fillTwoInput
    computeTransaction(installmentProgram, before, wrongPhase, Time2) == expectedRejection(before, "no_matching_input")
      and computeTransaction(installmentProgram, before, PresentAInput(ChoiceInputA({id: phase, chooser: Alice, chosen: 1})), Time2)
        == expectedRejection(before, "no_matching_input")
      and computeTransaction(installmentProgram, before, PresentAInput(ChoiceInputA({id: phase, chooser: Bob, chosen: 0})), Time2)
        == expectedRejection(before, "choice_out_of_bounds")
      and computeTransaction(installmentProgram, before, correctInput, Time0) == expectedRejection(before, "time_before_state")
      and computeTransaction(installmentProgram, before, NoAInput, Time2) == expectedRejection(before, "input_required")
  }))
  run onlyCleanupAfterRejectedRecoveryTest = init.then(fillFirst).then(rejectRecoveryFive).expect(
    canRequest(installmentTrace, NoAInput, Time100)
      and not(canRequest(installmentTrace, fillTwoInput, Time2))
      and not(canRequest(installmentTrace, recoveryInput, Time2))
      and not(canRequest(installmentTrace, recoveryInput, Time100)))
  run terminalNoActionTest = init.then(fillFirst).then(fillSecond).expect(
    installmentTerminal and not(traceEnabled(installmentTrace)))
  run installmentLedgerTamperDetectedTest = init.then(fillFirst).expect(not(traceSafety({...installmentTrace,
    ledger: installmentTrace.ledger.put((Wallet(Mallory), TokenA), 1)})))
  run installmentRecordTamperDetectedTest = init.then(fillFirst).expect(match installmentTrace.records.head().payload {
    | InstallmentComputedA(payload) => {
        val original = installmentTrace.records.head()
        val badRaw = {...original, payload: InstallmentComputedA({...payload, raw: {...payload.raw, reductions: 0}})}
        val badProjection = {...original, payload: InstallmentComputedA({...payload,
          projection: {...payload.projection, reductions: 0}})}
        val badEffects = {...original, payload: InstallmentComputedA({...payload, effects: List()})}
        val badRequest = {...original, request: {...original.request, input: fillTwoInput}}
        Set(badRaw, badProjection, badEffects, badRequest).forall(changed =>
          not(traceSafety({...installmentTrace, records: List(changed)})))
      }
    | _ => false
  })
  run installmentDiagnosticNeverWitnessTest = init.then(fillFirst).expect(match installmentTrace.records.head().payload {
    | InstallmentComputedA(payload) => {
        val original = installmentTrace.records.head()
        val badComputation = {...original, payload: InstallmentComputationDiagnosticA(OutsideModelDomainA)}
        val badLedger = {...original, payload: InstallmentLedgerDiagnosticA(payload)}
        Set(badComputation, badLedger).forall(changed => {
          val bad = {...installmentTrace, records: List(changed)}
          not(traceSafety(bad)) and not(traceTerminal(bad)) and not(recorded(bad, fillOneInput, Time2, N4, true))
        })
      }
    | _ => false
  })
}
