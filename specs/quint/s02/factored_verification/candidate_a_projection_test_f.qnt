module candidate_a_projection_test_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"

  pure val expectedChoiceNames = Set("settle", "fill1", "fill2", "recover", "other")
  pure val expectedEmptyChoices: str -> OptionalInt = Map("settle" -> NoInt,
    "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt)
  pure def acceptedResult(state: AState, payments: List[CorePayment],
                          warnings: List[CoreWarning], reductions: int): ATransactionResult = {
    accepted: true, state: state, error: NoCoreError, payments: payments,
    warnings: warnings, reductions: reductions,
  }
  pure def rejectedResult(state: AState, code: str): ATransactionResult = {
    accepted: false, state: state, error: CoreErrorCode(code), payments: List(),
    warnings: List(), reductions: 0,
  }
  pure def aliceDeposit(quantity: int): AInput = PresentAInput(
    DepositInputA({account: aliceA, depositor: Alice, quantity: quantity}))
  pure def alicePayment(recipient: Principal, quantity: int): CorePayment = {
    source: aliceA, recipient: recipient, asset: TokenA, quantity: quantity,
  }
  pure def aliceDebit(recipient: Principal, quantity: int): Transfer = {
    source: Escrow(aliceA), destination: Wallet(recipient), asset: TokenA, quantity: quantity,
  }
  pure def aliceCredit(quantity: int): Transfer = {
    source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: quantity,
  }
  pure def extractionEquals(program: AProgram, before: AState, input: AInput,
                            now: ATime, expected: ATransactionResult, transfers: List[Transfer]): bool =
    computeTransaction(program, before, input, now) == TransactionComputedA(expected)
      and extractCommittedEffects(program, before, input, now, expected) == EffectsExtractedA(transfers)

  run fullStateProjectionTest = {
    val before = {...emptyAState(N4), accounts: emptyAState(N4).accounts.put(aliceA, 10).put(bobB, 20),
      choices: Map(SettleId -> NoInt, FirstFillId -> IntValue(0), SecondFillId -> IntValue(1),
        RecoveryId -> IntValue(-1), OtherId -> IntValue(2)), minimumTime: Time101}
    val expected: CoreStateObservation[AContinuation] = {
      accounts: before.accounts, choices: Map("settle" -> NoInt, "fill1" -> IntValue(0),
        "fill2" -> IntValue(1), "recover" -> IntValue(-1), "other" -> IntValue(2)),
      continuation: {program: canonicalSwap, node: N4}, minimumTime: 101,
    }
    assert(validState(canonicalSwap, before) and projectState(canonicalSwap, before) == expected
      and validCoreState(expectedChoiceNames, expected))
  }
  run absentChoiceAndStoredZeroProjectionTest = {
    val before = emptyAState(N6)
    val zero = {...before, choices: before.choices.put(SettleId, IntValue(0))}
    assert(projectState(canonicalSwap, before).choices == expectedEmptyChoices
      and projectState(canonicalSwap, zero).choices == expectedEmptyChoices.put("settle", IntValue(0))
      and projectState(canonicalSwap, before) != projectState(canonicalSwap, zero))
  }
  run continuationBindsCompleteProgramTest = {
    val altered = {...canonicalSwap, nodes: canonicalSwap.nodes.put(N1, CloseA)}
    val before = emptyAState(N4)
    assert(validState(altered, before)
      and projectState(canonicalSwap, before).continuation == {program: canonicalSwap, node: N4}
      and projectState(altered, before).continuation == {program: altered, node: N4}
      and projectState(canonicalSwap, before).continuation != projectState(altered, before).continuation)
  }
  run everyNumericClockProjectionTest = assert(List(
    (Time0, 0), (Time1, 1), (Time2, 2), (Time100, 100), (Time101, 101)).foldl(true, (ok, pair) =>
      ok and projectState(closeProgram, {...emptyAState(N0), minimumTime: pair._1}).minimumTime == pair._2))
  run missingMapsAreNotCertifiedTest = {
    val malformed: AState = {...emptyAState(N5), accounts: Map(), choices: Map(), minimumTime: Time2}
    val projected = projectState(canonicalSwap, malformed)
    assert(not(validState(canonicalSwap, malformed)) and not(validCoreState(expectedChoiceNames, projected))
      and projected == {accounts: Map(), choices: Map(),
        continuation: {program: canonicalSwap, node: N5}, minimumTime: 2})
  }
  run noInputProjectionTest = assert(projectInput(NoAInput) == NoInput)
  run depositProjectionAllFieldsTest = {
    val account: CoreAccount = {owner: Bob, asset: TokenB}
    assert(projectInput(PresentAInput(DepositInputA({account: account, depositor: Mallory, quantity: 5})))
      == DepositLike({location: Escrow(account), depositor: Mallory, asset: TokenB, quantity: 5}))
  }
  run choiceProjectionAllNamesTest = {
    val names = Map(SettleId -> "settle", FirstFillId -> "fill1", SecondFillId -> "fill2",
      RecoveryId -> "recover", OtherId -> "other")
    assert(A_CHOICE_IDS.forall(id =>
      projectInput(PresentAInput(ChoiceInputA({id: id, chooser: Mallory, chosen: -1})))
        == ChoiceLike({id: names.get(id), chooser: Mallory, chosen: -1})))
  }
  run inputProjectionDoesNotValidateDomainTest = {
    val outside = aliceDeposit(999)
    assert(not(validInput(outside)) and projectInput(outside)
      == DepositLike({location: Escrow(aliceA), depositor: Alice, asset: TokenA, quantity: 999}))
  }
  run completeResultProjectionTest = {
    val state = {...emptyAState(N2), accounts: emptyAState(N2).accounts.put(bobB, 3),
      choices: emptyAState(N2).choices.put(RecoveryId, IntValue(0)), minimumTime: Time100}
    // Carrier test: even optional warning payloads are translated, not repaired.
    val warnings: List[CoreWarning] = List(
      {code: "partial_payment", requested: IntValue(10), paid: IntValue(4)},
      {code: "carrier_only", requested: NoInt, paid: NoInt})
    val payments = List(alicePayment(Bob, 1), alicePayment(Alice, 4))
    val raw = acceptedResult(state, payments, warnings, 7)
    val expected: CoreResultObservation[AContinuation] = {accepted: true, error: NoCoreError,
      warnings: warnings, payments: payments, reductions: 7,
      state: {accounts: state.accounts,
        choices: expectedEmptyChoices.put("recover", IntValue(0)),
        continuation: {program: canonicalSwap, node: N2}, minimumTime: 100}}
    assert(projectResult(canonicalSwap, raw) == expected)
  }
  run rejectedResultProjectionTest = {
    val before = {...emptyAState(N4), minimumTime: Time2}
    val rejected = rejectedResult(before, "choice_out_of_bounds")
    assert(projectResult(canonicalSwap, rejected) == {accepted: false,
      error: CoreErrorCode("choice_out_of_bounds"), payments: List(), warnings: List(), reductions: 0,
      state: {accounts: before.accounts, choices: expectedEmptyChoices,
        continuation: {program: canonicalSwap, node: N4}, minimumTime: 2}})
  }

  pure val depositOnlyBefore = emptyAState(N6)
  pure val depositOnlyResult = acceptedResult({...depositOnlyBefore, continuation: N5,
    accounts: depositOnlyBefore.accounts.put(aliceA, 10), minimumTime: Time1}, List(), List(), 0)
  run depositWithoutPaymentEffectTest = assert(extractionEquals(canonicalSwap, depositOnlyBefore,
    aliceDeposit(10), Time1, depositOnlyResult, List(aliceCredit(10))))

  pure val depositRefundProgram: AProgram = {root: N1, nodes: closeProgram.nodes.put(N1,
    WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(5)}),
      continuation: N0}), timeout: Time100, timeoutNode: N0}))}
  run immediateDepositRefundEffectTest = {
    val before = emptyAState(N1)
    val expected = acceptedResult({...before, continuation: N0, minimumTime: Time2},
      List(alicePayment(Alice, 5)), List(), 1)
    assert(extractionEquals(depositRefundProgram, before, aliceDeposit(5), Time2, expected,
      List(aliceCredit(5), aliceDebit(Alice, 5))))
  }
  pure val preDepositPostProgram: AProgram = {root: N3, nodes: closeProgram.nodes
    .put(N1, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))
    .put(N2, WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(5)}),
      continuation: N1}), timeout: Time100, timeoutNode: N0}))
    .put(N3, PayA({account: aliceA, payee: Bob, amount: ConstantA(1), continuation: N2}))}
  pure val preDepositPostBefore = {...emptyAState(N3), accounts: emptyAState(N3).accounts.put(aliceA, 1)}
  pure val preDepositPostResult = acceptedResult({...emptyAState(N0), minimumTime: Time2},
    List(alicePayment(Bob, 1), alicePayment(Bob, 5)), List(), 2)
  run prePaymentDepositPostPaymentOrderTest = assert(extractionEquals(preDepositPostProgram,
    preDepositPostBefore, aliceDeposit(5), Time2, preDepositPostResult,
    List(aliceDebit(Bob, 1), aliceCredit(5), aliceDebit(Bob, 5))))
  run rejectedSpeculativePaymentEffectsEmptyTest = {
    val expected = rejectedResult(preDepositPostBefore, "no_matching_input")
    assert(extractionEquals(preDepositPostProgram, preDepositPostBefore, aliceDeposit(10), Time2,
      expected, List()))
  }
  run rejectedSpeculativeWarningEffectsEmptyTest = {
    val program = {...preDepositPostProgram, nodes: preDepositPostProgram.nodes.put(N3,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(0), continuation: N2}))}
    val expected = rejectedResult(preDepositPostBefore, "no_matching_input")
    assert(extractionEquals(program, preDepositPostBefore, aliceDeposit(10), Time2, expected, List()))
  }
  pure val fundedSwapBefore = {...emptyAState(N4), accounts: emptyAState(N4).accounts.put(aliceA, 10).put(bobB, 20),
    minimumTime: Time2}
  run timeoutInputRejectsSpeculativeRefundEffectsTest = {
    val input = PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 1}))
    val expected = rejectedResult(fundedSwapBefore, "contract_closed")
    assert(extractionEquals(canonicalSwap, fundedSwapBefore, input, Time100, expected, List()))
  }
  run timeoutNoInputRefundOrderTest = {
    val expected = acceptedResult({...emptyAState(N0), minimumTime: Time100},
      List(alicePayment(Alice, 10), {source: bobB, recipient: Bob, asset: TokenB, quantity: 20}), List(), 3)
    assert(extractionEquals(canonicalSwap, fundedSwapBefore, NoAInput, Time100, expected,
      List(aliceDebit(Alice, 10), {source: Escrow(bobB), destination: Wallet(Bob), asset: TokenB, quantity: 20})))
  }
  run choiceSettlementEffectsAndStoredChoiceTest = {
    val input = PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 1}))
    val expected = acceptedResult({...emptyAState(N0), minimumTime: Time2,
      choices: emptyAState(N0).choices.put(SettleId, IntValue(1))},
      List(alicePayment(Bob, 10), {source: bobB, recipient: Alice, asset: TokenB, quantity: 20}), List(), 3)
    assert(extractionEquals(canonicalSwap, fundedSwapBefore, input, Time2, expected,
      List(aliceDebit(Bob, 10), {source: Escrow(bobB), destination: Wallet(Alice), asset: TokenB, quantity: 20})))
  }
  run acceptedResultMutationIsDiagnosticTest = {
    val changed = {...depositOnlyResult, reductions: 1}
    assert(extractCommittedEffects(canonicalSwap, depositOnlyBefore, aliceDeposit(10), Time1, changed)
      == EffectResultMismatchA)
  }
  run rejectedResultMutationIsDiagnosticTest = {
    val actual = rejectedResult(preDepositPostBefore, "no_matching_input")
    val changed = {...actual, state: {...actual.state, minimumTime: Time2}}
    assert(extractCommittedEffects(preDepositPostProgram, preDepositPostBefore, aliceDeposit(10), Time2, changed)
      == EffectResultMismatchA)
  }
  run rejectedFlagCannotHideActualAcceptanceTest = {
    val invented = rejectedResult(depositOnlyBefore, "no_matching_input")
    assert(extractCommittedEffects(canonicalSwap, depositOnlyBefore, aliceDeposit(10), Time1, invented)
      == EffectResultMismatchA)
  }
  run acceptedPaymentOrderMutationIsDiagnosticTest = {
    val changed = {...preDepositPostResult, payments: List(alicePayment(Bob, 5), alicePayment(Bob, 1))}
    assert(extractCommittedEffects(preDepositPostProgram, preDepositPostBefore, aliceDeposit(5), Time2, changed)
      == EffectResultMismatchA)
  }
  run malformedStateExtractionIsDiagnosticTest = {
    val malformed: AState = {...depositOnlyBefore, choices: Map()}
    assert(extractCommittedEffects(canonicalSwap, malformed, aliceDeposit(10), Time1, depositOnlyResult)
      == EffectOutsideDomainA)
  }
  run malformedProgramExtractionIsDiagnosticTest = {
    val malformed: AProgram = {root: N6, nodes: Map()}
    assert(extractCommittedEffects(malformed, depositOnlyBefore, aliceDeposit(10), Time1, depositOnlyResult)
      == EffectOutsideDomainA)
  }
  run outsideInputExtractionIsDiagnosticTest = assert(
    extractCommittedEffects(canonicalSwap, depositOnlyBefore, aliceDeposit(999), Time1, depositOnlyResult)
      == EffectOutsideDomainA)
}
