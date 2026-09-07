module candidate_a_boundary_test_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_core_f.* from "./candidate_a_core_f"

  pure val ascendingNodes = List(N0, N1, N2, N3, N4, N5, N6, N7,
    N8, N9, N10, N11, N12, N13, N14, N15)
  pure val twentyOneReductionProgram: AProgram = {root: N15, nodes: A_NODE_IDS.mapBy(node =>
    if (node == N0) CloseA else PayA({account: aliceA, payee: Bob,
      amount: ConstantA(0), continuation: ascendingNodes.nth(rank(node) - 1)}))}
  pure def waitingProgram(cases: List[ACase], timeout: ATime, successor: NodeId): AProgram =
    {root: N3, nodes: closeProgram.nodes.put(N3,
      WhenA({cases: cases, timeout: timeout, timeoutNode: successor}))}
  pure def choiceCase(lower: int, upper: int, successor: NodeId): ACase = {
    caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: lower, upper: upper}), continuation: successor}
  pure def depositCase(quantity: int, successor: NodeId): ACase = {
    caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(quantity)}), continuation: successor}
  pure def choiceInput(chooser: Principal, chosen: int): ASuppliedInput =
    ChoiceInputA({id: SettleId, chooser: chooser, chosen: chosen})
  pure def depositInput(quantity: int): ASuppliedInput =
    DepositInputA({account: aliceA, depositor: Alice, quantity: quantity})

  run fixedTwentyTwoCallsReachTerminalQuiescenceTest = {
    val before = {...emptyAState(N15), accounts: CORE_ACCOUNTS.mapBy(_ => 1)}
    val warnings = ascendingNodes.tail().foldl(List(), (acc, _) => acc.append(
      {code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)}))
    val refunds = CANONICAL_ACCOUNTS.foldl(List(), (acc, account) => acc.append(
      {source: account, recipient: account.owner, asset: account.asset, quantity: 1}))
    assert(reduceToQuiescence(twentyOneReductionProgram, before) == ReductionCompleteA({
      state: emptyAState(N0), payments: refunds, warnings: warnings, reductions: 21}))
  }

  run maximumDepositPreservesPotentialAcrossInputAndReductionTest = {
    val program = {root: N3, nodes: closeProgram.nodes
      .put(N2, PayA({account: aliceA, payee: Bob, amount: ConstantA(20), continuation: N0}))
      .put(N3, WhenA({cases: List(depositCase(20, N2)), timeout: Time100, timeoutNode: N0}))}
    val before = {...emptyAState(N3), accounts: emptyAState(N3).accounts.put(aliceA, 280), minimumTime: Time2}
    val afterInput = {...before, continuation: N2, accounts: before.accounts.put(aliceA, 300)}
    assert(validState(program, before) and validState(program, afterInput)
      and applyInput(program, before, depositInput(20)) == InputAppliedA({state: afterInput, error: NoCoreError})
      and reduceToQuiescence(program, afterInput) == ReductionCompleteA({
        state: {...afterInput, continuation: N0, accounts: afterInput.accounts.put(aliceA, 0)},
        payments: List({source: aliceA, recipient: Bob, asset: TokenA, quantity: 20},
          {source: aliceA, recipient: Alice, asset: TokenA, quantity: 280}),
        warnings: List(), reductions: 2}))
  }

  run expiredWhenRetainsOriginalClockTest = {
    val program = waitingProgram(List(), Time1, N0)
    val before = {...emptyAState(N3), accounts: emptyAState(N3).accounts.put(aliceA, 10), minimumTime: Time2}
    assert(reduceToQuiescence(program, before) == ReductionCompleteA({
      state: {...before, continuation: N0, accounts: before.accounts.put(aliceA, 0)},
      payments: List({source: aliceA, recipient: Alice, asset: TokenA, quantity: 10}),
      warnings: List(), reductions: 2}))
  }

  run orderedChoiceScanSeparatesIdentityAndBoundsTest = {
    val program = waitingProgram(List(choiceCase(0, 0, N1), choiceCase(1, 1, N2)), Time100, N0)
    val before = {...emptyAState(N3), minimumTime: Time2}
    assert(applyInput(program, before, choiceInput(Bob, 1)) == InputAppliedA({
      state: {...before, continuation: N2, choices: before.choices.put(SettleId, IntValue(1))}, error: NoCoreError})
      and applyInput(program, before, choiceInput(Bob, 2)) == InputAppliedA({
        state: before, error: CoreErrorCode("choice_out_of_bounds")})
      and applyInput(program, before, choiceInput(Mallory, 2)) == InputAppliedA({
        state: before, error: CoreErrorCode("no_matching_input")}))
  }
}
