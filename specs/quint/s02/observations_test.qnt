module observations_test {
  import observations.* from "./observations"
  import effects.* from "./effects"
  import observations_harness.* from "./observations_harness"

  pure val aliceAccount: CoreAccount = {owner: Alice, asset: TokenA}
  pure val bobAccount: CoreAccount = {owner: Bob, asset: TokenB}
  pure val alicePayment: CorePayment = {source: aliceAccount, recipient: Alice, asset: TokenA, quantity: 10}
  pure val bobPayment: CorePayment = {source: bobAccount, recipient: Bob, asset: TokenB, quantity: 20}
  pure val refundResult: CoreResultObservation[str] = {
    ...acceptedDeposit, payments: List(alicePayment, bobPayment),
    state: {...before, continuation: "closed", minimumTime: 100}, reductions: 3,
  }
  pure val funded: CoreStateObservation[str] = {
    ...before, accounts: before.accounts.put(aliceAccount, 10).put(bobAccount, 20), minimumTime: 2,
  }
  pure val aliceRefund: Transfer = {
    source: Escrow(aliceAccount), destination: Wallet(Alice), asset: TokenA, quantity: 10,
  }
  pure val bobRefund: Transfer = {
    source: Escrow(bobAccount), destination: Wallet(Bob), asset: TokenB, quantity: 20,
  }

  run knownErrorsTest = init.expect(
    Set("time_before_state", "contract_closed", "input_required", "no_matching_input",
      "choice_out_of_bounds", "non_positive_deposit").forall(c => validFrozenCoreError(CoreErrorCode(c)))
    and validFrozenCoreError(NoCoreError))
  run unknownErrorTest = init.expect(not(validFrozenCoreError(CoreErrorCode("stale_signature"))))
  run nonpositiveWarningTest = init.expect(
    validFrozenCoreWarning({code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)})
    and validFrozenCoreWarning({code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}))
  run partialWarningTest = init.expect(
    validFrozenCoreWarning({code: "partial_payment", requested: IntValue(10), paid: IntValue(0)})
    and validFrozenCoreWarning({code: "partial_payment", requested: IntValue(10), paid: IntValue(5)}))
  run missingWarningOperandsTest = init.expect(
    not(validFrozenCoreWarning({code: "partial_payment", requested: NoInt, paid: IntValue(0)}))
    and not(validFrozenCoreWarning({code: "non_positive_payment", requested: IntValue(0), paid: NoInt})))
  run impossibleWarningsTest = init.expect(
    not(validFrozenCoreWarning({code: "unknown", requested: IntValue(1), paid: IntValue(0)}))
    and not(validFrozenCoreWarning({code: "partial_payment", requested: IntValue(5), paid: IntValue(5)}))
    and not(validFrozenCoreWarning({code: "partial_payment", requested: IntValue(5), paid: IntValue(-1)}))
    and not(validFrozenCoreWarning({code: "non_positive_payment", requested: IntValue(1), paid: IntValue(0)})))
  run choiceAbsentVersusZeroTest = init.expect(
    before.choices.get("settle") != IntValue(0)
    and validCoreState(CHOICE_IDS, before)
    and validCoreState(CHOICE_IDS, {...before, choices: Map("settle" -> IntValue(0))}))
  run choiceDomainTest = init.expect(
    not(validCoreState(CHOICE_IDS, {...before, choices: Map()}))
    and not(validCoreState(CHOICE_IDS, {...before, choices: before.choices.put("extra", NoInt)})))
  run accountDomainTest = init.expect(not(validCoreState(CHOICE_IDS, {...before, accounts: Map()})))
  run negativeAccountTest = init.expect(
    not(validCoreState(CHOICE_IDS, {...before, accounts: before.accounts.put(aliceAccount, -1)})))
  run acceptedDepositTest = init.expect(validCoreObservation(CHOICE_IDS, before, acceptedDeposit, List(deposit)))
  run rejectionTest = init.expect(validCoreObservation(CHOICE_IDS, before, rejected, List()))
  run rejectionTimeMutationTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, {...rejected, state: {...before, minimumTime: 1}}, List())))
  run rejectionContinuationMutationTest = init.expect(
    not(rejectionPreserved(before, {...rejected, state: {...before, continuation: "closed"}}, List())))
  run rejectionAccountMutationTest = init.expect(
    not(rejectionPreserved(before, {...rejected, state: {...before, accounts: before.accounts.put(aliceAccount, 1)}}, List())))
  run rejectionChoiceMutationTest = init.expect(
    not(rejectionPreserved(before, {...rejected, state: {...before, choices: Map("settle" -> IntValue(0))}}, List())))
  run rejectionEffectsMutationTest = init.expect(not(rejectionPreserved(before, rejected, List(deposit))))
  run rejectionPaymentsMutationTest = init.expect(not(rejectionPreserved(before, {...rejected, payments: List(alicePayment)}, List())))
  run rejectionWarningsMutationTest = init.expect(
    not(rejectionPreserved(before, {...rejected, warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(5)})}, List())))
  run rejectionReductionsMutationTest = init.expect(not(rejectionPreserved(before, {...rejected, reductions: 1}, List())))
  run acceptedErrorMismatchTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, {...acceptedDeposit, error: CoreErrorCode("input_required")}, List(deposit)))
    and not(validCoreObservation(CHOICE_IDS, before, {...rejected, error: NoCoreError}, List())))
  run negativeReductionsTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, {...acceptedDeposit, reductions: -1}, List(deposit))))
  run acceptedTimeRegressionTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, {...before, minimumTime: 2}, acceptedDeposit, List(deposit))))
  run inventedDepositPaymentTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, {...acceptedDeposit, payments: List(alicePayment)}, List(deposit))))
  run walletToWalletIsNotCoreDepositTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, acceptedDeposit,
      List({...deposit, destination: Wallet(Mallory)}))))
  run multipleDepositsAreNotOneCoreInputTest = init.expect(
    not(validCoreObservation(CHOICE_IDS, before, acceptedDeposit, List(deposit, deposit))))
  run paymentInjectionTest = init.expect(paymentTransfer(alicePayment) == aliceRefund)
  run orderedRefundTest = init.expect(validCoreObservation(CHOICE_IDS, funded, refundResult, List(aliceRefund, bobRefund)))
  run reversedRefundTest = init.expect(not(validCoreObservation(CHOICE_IDS, funded, refundResult, List(bobRefund, aliceRefund))))
  run duplicatedRefundTest = init.expect(not(validCoreObservation(CHOICE_IDS, funded, refundResult, List(aliceRefund, bobRefund, bobRefund))))
  run invalidPaymentTest = init.expect(
    not(validCorePayment({...alicePayment, asset: TokenB})) and not(validCorePayment({...alicePayment, quantity: 0})))
  run optionalProjectionTest = init.expect(
    not(projectionSupportsCoreEvidence(NoCoreProjection)) and projectionSupportsCoreEvidence(CoreProjected(acceptedDeposit)))
  run inspectDepositTest = init.then(inspectDeposit).expect(checkedDeposit and safety)
  run inspectRejectionTest = init.then(inspectDeposit).then(inspectRejection).expect(checkedRejection and safety)
}
