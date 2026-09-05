module candidate_a_core_test {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_core.* from "./candidate_a_core"

  pure def payProgram(quantity: int): AProgram = {root: N1, nodes: closeProgram.nodes.put(N1,
    PayA({account: aliceA, payee: Bob, amount: ConstantA(quantity), continuation: N0}))}
  pure def fundedState(node: NodeId, quantity: int): AState = {
    ...emptyAState(node), accounts: emptyAState(node).accounts.put(aliceA, quantity),
    choices: emptyAState(node).choices.put(SettleId, IntValue(0)), minimumTime: Time2}

  run closeEmptyQuiescentTest = assert(canReduceOnce(closeProgram, emptyAState(N0))
    and reduceOnce(closeProgram, emptyAState(N0)) == QuiescentA(emptyAState(N0)))
  run closeCanonicalRefundStepTest = {
    val before = {...fundedState(N0, 3), accounts: fundedState(N0, 3).accounts.put(bobB, 7)}
    val expected: AReductionStep = ReducedA({state: {...before, accounts: before.accounts.put(aliceA, 0)},
      payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 3}),
      warnings: List(), reductions: 1})
    assert(canReduceOnce(closeProgram, before) and reduceOnce(closeProgram, before) == expected)
  }
  run closeCanonicalNonAliceRefundTest = {
    val aliceB: CoreAccount = {owner: Alice, asset: TokenB}
    val before = {...emptyAState(N0), accounts: emptyAState(N0).accounts.put(bobB, 7).put(aliceB, 5)}
    assert(reduceOnce(closeProgram, before) == ReducedA({state: {...before, accounts: before.accounts.put(aliceB, 0)},
      payments: List({source: aliceB, recipient: Alice, asset: TokenB, quantity: 5}), warnings: List(), reductions: 1}))
  }
  run payExactStepTest = {
    val before = fundedState(N1, 10)
    assert(canReduceOnce(payProgram(10), before) and reduceOnce(payProgram(10), before) == ReducedA({
      state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 0)},
      payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 10}), warnings: List(), reductions: 1}))
  }
  run payRetainsExcessBalanceTest = {
    val before = fundedState(N1, 10)
    assert(reduceOnce(payProgram(5), before) == ReducedA({
      state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 5)},
      payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 5}), warnings: List(), reductions: 1}))
  }
  run payPartialFourOfTenTest = {
    val before = fundedState(N1, 4)
    assert(reduceOnce(payProgram(10), before) == ReducedA({
      state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 0)},
      payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 4}),
      warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(4)}), reductions: 1}))
  }
  run payFromMissingAccountTest = {
    val before = fundedState(N1, 0)
    assert(reduceOnce(payProgram(10), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
      warnings: List({code: "partial_payment", requested: IntValue(10), paid: IntValue(0)}), reductions: 1}))
  }
  run payZeroWarningTest = {
    val before = fundedState(N1, 10)
    assert(reduceOnce(payProgram(0), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
      warnings: List({code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}), reductions: 1}))
  }
  run payNegativeWarningTest = {
    val before = fundedState(N1, 10)
    assert(reduceOnce(payProgram(-1), before) == ReducedA({state: {...before, continuation: N0}, payments: List(),
      warnings: List({code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)}), reductions: 1}))
  }
  run allFrozenConstructorsSupportedTest = assert(
    validState(canonicalSwap, emptyAState(N3)) and validState(canonicalSwap, emptyAState(N6))
      and canReduceOnce(canonicalSwap, emptyAState(N3))
      and canReduceOnce(canonicalSwap, emptyAState(N6)))
  run invalidDomainDistinctTest = {
    val malformed = {...emptyAState(N0), accounts: Map()}
    assert(not(canReduceOnce(closeProgram, malformed))
      and reduceOnce(closeProgram, malformed) == ReductionOutsideDomainA
      and reduceOnce(payProgram(21), fundedState(N1, 10)) == ReductionOutsideDomainA)
  }

  pure def waitingProgram(cases: List[ACase]): AProgram = {root: N3,
    nodes: closeProgram.nodes.put(N3, WhenA({cases: cases, timeout: Time100, timeoutNode: N0}))}
  pure def choiceCase(lower: int, upper: int, successor: NodeId): ACase = {
    caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: lower, upper: upper}), continuation: successor}
  pure def depositCase(quantity: int, successor: NodeId): ACase = {
    caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(quantity)}), continuation: successor}
  pure def chosenInput(who: Principal, chosen: int): ASuppliedInput =
    ChoiceInputA({id: SettleId, chooser: who, chosen: chosen})
  pure def depositInput(quantity: int): ASuppliedInput =
    DepositInputA({account: aliceA, depositor: Alice, quantity: quantity})
  pure val zeroBranchProgram: AProgram = {root: N2, nodes: closeProgram.nodes.put(N2,
    IfA({observation: ChoiceEqualsA({id: SettleId, expected: 0}), thenNode: N1, elseNode: N0}))}
  pure val ascendingNodes = List(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15)
  pure val maximalReductionProgram: AProgram = {root: N15, nodes: A_NODE_IDS.mapBy(node =>
    if (node == N0) CloseA else PayA({account: aliceA, payee: Bob, amount: ConstantA(0),
      continuation: ascendingNodes.nth(rank(node) - 1)}))}

  run ifAbsentEqualsZeroFalseTest = {
    val before = emptyAState(N2)
    assert(not(evaluateObservation(ChoiceEqualsA({id: SettleId, expected: 0}), before))
      and reduceOnce(zeroBranchProgram, before) == ReducedA({
        state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
  }
  run ifStoredZeroTrueTest = {
    val before = {...emptyAState(N2), choices: emptyAState(N2).choices.put(SettleId, IntValue(0))}
    assert(evaluateObservation(ChoiceEqualsA({id: SettleId, expected: 0}), before)
      and reduceOnce(zeroBranchProgram, before) == ReducedA({
        state: {...before, continuation: N1}, payments: List(), warnings: List(), reductions: 1}))
  }
  run ifStoredMismatchFalseTest = {
    val before = {...fundedState(N2, 5), choices: fundedState(N2, 5).choices.put(SettleId, IntValue(1))}
    assert(reduceOnce(zeroBranchProgram, before) == ReducedA({
      state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
  }
  run whenBeforeDeadlineQuiescentTest = {
    val before = fundedState(N3, 10)
    assert(reduceOnce(waitingProgram(List()), before) == QuiescentA(before))
  }
  run timeoutAtDeadlineStepTest = {
    val before = {...fundedState(N3, 10), minimumTime: Time100}
    assert(reduceOnce(waitingProgram(List()), before) == ReducedA({
      state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
  }
  run timeoutAfterDeadlineStepTest = {
    val before = {...fundedState(N3, 10), minimumTime: Time101}
    assert(reduceOnce(waitingProgram(List()), before) == ReducedA({
      state: {...before, continuation: N0}, payments: List(), warnings: List(), reductions: 1}))
  }
  run quiescentWaitingZeroCountTest = {
    val before = fundedState(N3, 10)
    assert(reduceToQuiescence(waitingProgram(List()), before) == ReductionCompleteA({
      state: before, payments: List(), warnings: List(), reductions: 0}))
  }
  run quiescentEmptyCloseZeroCountTest = assert(reduceToQuiescence(closeProgram, emptyAState(N0))
    == ReductionCompleteA({state: emptyAState(N0), payments: List(), warnings: List(), reductions: 0}))
  run reductionBoundCompleteTest = {
    val before = {...emptyAState(N15), accounts: CORE_ACCOUNTS.mapBy(_ => 1)}
    val expectedWarnings = ascendingNodes.tail().foldl(List(), (acc, _) => acc.append(
      {code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}))
    val expectedPayments = CANONICAL_ACCOUNTS.foldl(List(), (acc, account) => acc.append(
      {source: account, recipient: account.owner, asset: account.asset, quantity: 1}))
    assert(validState(maximalReductionProgram, before)
      and reduceToQuiescence(maximalReductionProgram, before) == ReductionCompleteA({
        state: emptyAState(N0), payments: expectedPayments, warnings: expectedWarnings, reductions: 21}))
  }
  run quiescencePreservesPaymentWarningOrderTest = {
    val program = {root: N2, nodes: payProgram(10).nodes.put(N2,
      PayA({account: aliceA, payee: Mallory, amount: ConstantA(-1), continuation: N1}))}
    val before = {...fundedState(N2, 4), accounts: fundedState(N2, 4).accounts.put(bobB, 7)}
    assert(reduceToQuiescence(program, before) == ReductionCompleteA({
      state: {...before, continuation: N0, accounts: CORE_ACCOUNTS.mapBy(_ => 0)},
      payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 4},
        {source: bobB, recipient: Bob, asset: TokenB, quantity: 7}),
      warnings: List({code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)},
        {code: "partial_payment", requested: IntValue(10), paid: IntValue(4)}), reductions: 3}))
  }
  run deadlineReductionRefundsBeforeInputTest = {
    val before = {...fundedState(N3, 10), minimumTime: Time100}
    val program = waitingProgram(List(choiceCase(0, 1, N1)))
    assert(reduceToQuiescence(program, before) == ReductionCompleteA({
      state: {...before, continuation: N0, accounts: CORE_ACCOUNTS.mapBy(_ => 0)},
      payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 10}),
      warnings: List(), reductions: 2})
      and applyInput(program, before, chosenInput(Bob, 1)) == InputPreconditionFailureA)
  }
  run literalFixturesBoundCompleteTest = assert(
    Set(closeProgram, canonicalSwap, zeroBranchProgram, waitingProgram(List()), payProgram(-1),
      payProgram(0), payProgram(1), payProgram(5), payProgram(10), payProgram(20), maximalReductionProgram)
      .forall(program => A_TIMES.forall(time => match reduceToQuiescence(program,
        {...emptyAState(program.root), minimumTime: time}) {
          | ReductionCompleteA(_) => true | _ => false
        })))
  run firstAcceptingCaseOrderTest = {
    val program = waitingProgram(List(choiceCase(0, 1, N1), choiceCase(0, 1, N2)))
    val before = fundedState(N3, 10)
    assert(canApplyInput(program, before, chosenInput(Bob, 1))
      and applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
        state: {...before, continuation: N1, choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError}))
  }
  run laterChoiceCaseCanAcceptTest = {
    val program = waitingProgram(List(choiceCase(0, 0, N1), choiceCase(1, 1, N2)))
    val before = fundedState(N3, 10)
    assert(applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
      state: {...before, continuation: N2, choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError}))
  }
  run choiceBoundsMismatchOnlyAfterScanTest = {
    val program = waitingProgram(List(choiceCase(0, 0, N1), choiceCase(1, 1, N2)))
    val before = fundedState(N3, 10)
    assert(applyInput(program, before, chosenInput(Bob, 2)) == InputAppliedA({
      state: before, error: CoreErrorCode("choice_out_of_bounds")}))
  }
  run wrongChooserNoMatchingInputTest = {
    val before = fundedState(N3, 10)
    assert(applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before, chosenInput(Mallory, 2))
      == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
  }
  run wrongChoiceIdNoMatchingInputTest = {
    val before = fundedState(N3, 10)
    assert(applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before,
      ChoiceInputA({id: OtherId, chooser: Bob, chosen: 2}))
      == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
  }
  run matchedNonpositiveDepositTest = {
    val before = fundedState(N3, 10)
    assert(Set(-1, 0).forall(quantity => applyInput(waitingProgram(List(depositCase(quantity, N1))),
      before, depositInput(quantity)) == InputAppliedA({
        state: before, error: CoreErrorCode("non_positive_deposit")})))
  }
  run mismatchedNonpositiveDepositTest = {
    val before = fundedState(N3, 10)
    assert(applyInput(waitingProgram(List(depositCase(10, N1))), before, depositInput(0))
      == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
  }
  run matchingDepositAddsAndAdvancesTest = {
    val before = fundedState(N3, 4)
    assert(applyInput(waitingProgram(List(depositCase(10, N1))), before, depositInput(10))
      == InputAppliedA({state: {...before, continuation: N1, accounts: before.accounts.put(aliceA, 14)},
        error: NoCoreError}))
  }
  run depositAtPotentialBoundaryTest = {
    val before = fundedState(N3, 280)
    val expected = {...before, continuation: N2, accounts: before.accounts.put(aliceA, 300)}
    val program = waitingProgram(List(depositCase(20, N2)))
    assert(validState(program, before) and validState(program, expected)
      and applyInput(program, before, depositInput(20)) == InputAppliedA({state: expected, error: NoCoreError}))
  }
  run firstMatchingDepositOrderTest = {
    val before = fundedState(N3, 0)
    assert(applyInput(waitingProgram(List(depositCase(5, N1), depositCase(5, N2))), before, depositInput(5))
      == InputAppliedA({state: {...before, continuation: N1, accounts: before.accounts.put(aliceA, 5)},
        error: NoCoreError}))
  }
  run wrongDepositIdentityNoMatchingInputTest = {
    val before = fundedState(N3, 10)
    val program = waitingProgram(List(depositCase(10, N1)))
    assert(Set(DepositInputA({account: aliceA, depositor: Mallory, quantity: 10}),
      DepositInputA({account: bobB, depositor: Alice, quantity: 10})).forall(supplied =>
        applyInput(program, before, supplied) == InputAppliedA({
          state: before, error: CoreErrorCode("no_matching_input")})))
  }
  run emptyOrWrongKindNoMatchingInputTest = {
    val before = fundedState(N3, 10)
    assert(Set(waitingProgram(List()), waitingProgram(List(depositCase(10, N1)))).forall(program =>
      applyInput(program, before, chosenInput(Bob, 1)) == InputAppliedA({
        state: before, error: CoreErrorCode("no_matching_input")}))
      and applyInput(waitingProgram(List(choiceCase(0, 1, N1))), before, depositInput(10))
        == InputAppliedA({state: before, error: CoreErrorCode("no_matching_input")}))
  }
  run inputPreconditionDiagnosticTest = {
    val supplied = chosenInput(Bob, 1)
    val program = waitingProgram(List(choiceCase(0, 1, N1)))
    assert(Set(Time100, Time101).forall(time =>
      not(canApplyInput(program, {...emptyAState(N3), minimumTime: time}, supplied))
        and applyInput(program, {...emptyAState(N3), minimumTime: time}, supplied) == InputPreconditionFailureA)
      and applyInput(closeProgram, emptyAState(N0), supplied) == InputPreconditionFailureA
      and applyInput(canonicalSwap, emptyAState(N3), supplied) == InputPreconditionFailureA
      and applyInput(payProgram(10), emptyAState(N1), supplied) == InputPreconditionFailureA)
  }
  run task3DomainDiagnosticTest = {
    val invalidState = {...emptyAState(N3), choices: Map()}
    val program = waitingProgram(List())
    assert(reduceToQuiescence(program, invalidState) == QuiescenceOutsideDomainA
      and applyInput(program, invalidState, chosenInput(Bob, 1)) == InputOutsideDomainA
      and applyInput(program, emptyAState(N3), chosenInput(Bob, 3)) == InputOutsideDomainA
      and not(canApplyInput(program, emptyAState(N3), depositInput(22)))
      and applyInput(program, emptyAState(N3), depositInput(22)) == InputOutsideDomainA)
  }

  run finiteProgramDomainTest = assert(validProgram(closeProgram)
    and validState(closeProgram, emptyAState(N0)) and validInput(NoAInput))
  run cycleRejectedByDomainTest = {
    val cycle = {...closeProgram, root: N1, nodes: closeProgram.nodes.put(N1,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N1}))}
    assert(not(validProgram(cycle)))
  }
  run absentChoiceDiffersFromZeroTest = {
    val absent = emptyAState(N0)
    val zero = {...absent, choices: absent.choices.put(SettleId, IntValue(0))}
    assert(absent.choices.get(SettleId) == NoInt and zero.choices.get(SettleId) == IntValue(0) and absent != zero)
  }
  run canonicalAccountOrderTest = assert(CANONICAL_ACCOUNTS == List(
    {owner: Alice, asset: TokenA}, {owner: Alice, asset: TokenB},
    {owner: Bob, asset: TokenA}, {owner: Bob, asset: TokenB},
    {owner: Mallory, asset: TokenA}, {owner: Mallory, asset: TokenB}))
  run canonicalSwapNodeTableTest = assert(canonicalSwap.root == N6 and canonicalSwap.nodes ==
    A_NODE_IDS.mapBy(_ => CloseA)
      .put(N1, PayA({account: {owner: Bob, asset: TokenB}, payee: Alice, amount: ConstantA(20), continuation: N0}))
      .put(N2, PayA({account: {owner: Alice, asset: TokenA}, payee: Bob, amount: ConstantA(10), continuation: N1}))
      .put(N3, IfA({observation: ChoiceEqualsA({id: SettleId, expected: 1}), thenNode: N2, elseNode: N0}))
      .put(N4, WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}),
        continuation: N3}), timeout: Time100, timeoutNode: N0}))
      .put(N5, WhenA({cases: List({caseAction: DepositA({account: {owner: Bob, asset: TokenB}, depositor: Bob,
        amount: ConstantA(20)}), continuation: N4}), timeout: Time100, timeoutNode: N0}))
      .put(N6, WhenA({cases: List({caseAction: DepositA({account: {owner: Alice, asset: TokenA}, depositor: Alice,
        amount: ConstantA(10)}), continuation: N5}), timeout: Time100, timeoutNode: N0})))
  run canonicalSwapDomainTest = assert(validProgram(canonicalSwap) and validState(canonicalSwap, emptyAState(N6)))
  run finiteScalarMappingsTest = assert(A_TIMES.map(time => timeValue(time)) == Set(0, 1, 2, 100, 101)
    and A_CHOICE_IDS.map(id => choiceName(id)) == Set("settle", "fill1", "fill2", "recover", "other")
    and A_NODE_IDS.map(node => rank(node)) == 0.to(15))
  run missingNodeMapRejectedTest = assert(not(validProgram({root: N0, nodes: Map(N0 -> CloseA)})))
  run oversizedCaseListRejectedTest = {
    val repeatedCase: ACase = {caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}), continuation: N0}
    val oversized = {...closeProgram, root: N1, nodes: closeProgram.nodes.put(N1,
      WhenA({cases: List(repeatedCase, repeatedCase, repeatedCase), timeout: Time100, timeoutNode: N0}))}
    assert(not(validProgram(oversized)))
  }
  run invalidConstantRejectedTest = assert(not(validProgram({...closeProgram, root: N1,
    nodes: closeProgram.nodes.put(N1, PayA({account: aliceA, payee: Bob, amount: ConstantA(21), continuation: N0}))})))
  run invalidChoiceBoundsRejectedTest = assert(not(validProgram({...closeProgram, root: N1,
    nodes: closeProgram.nodes.put(N1, WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob,
      lower: 2, upper: 0}), continuation: N0}), timeout: Time100, timeoutNode: N0}))})))
  run statePotentialBoundaryTest = {
    val allowed = {...emptyAState(N6), accounts: emptyAState(N6).accounts.put(aliceA, 220)}
    val excessive = {...allowed, accounts: allowed.accounts.put(aliceA, 221)}
    assert(validState(canonicalSwap, allowed) and not(validState(canonicalSwap, excessive)))
  }
  run malformedStateMapsRejectedTest = assert(not(validState(closeProgram, {...emptyAState(N0), accounts: Map()}))
    and not(validState(closeProgram, {...emptyAState(N0), choices: Map()})))
  run stateValueDomainTest = {
    val empty = emptyAState(N0)
    assert(validState(closeProgram, {...empty, choices: empty.choices.put(SettleId, IntValue(0))})
      and not(validState(closeProgram, {...empty, choices: empty.choices.put(SettleId, IntValue(3))}))
      and not(validState(closeProgram, {...empty, accounts: empty.accounts.put(aliceA, -1)})))
  }
  run inputDomainTest = assert(
    validInput(PresentAInput(DepositInputA({account: aliceA, depositor: Mallory, quantity: 21})))
      and not(validInput(PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 22}))))
      and validInput(PresentAInput(ChoiceInputA({id: OtherId, chooser: Mallory, chosen: -1})))
      and not(validInput(PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 3})))))
}
