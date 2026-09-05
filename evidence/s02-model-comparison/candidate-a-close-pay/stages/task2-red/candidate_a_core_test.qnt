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
  run unsupportedIfWhenUnavailableTest = assert(
    validState(canonicalSwap, emptyAState(N3)) and validState(canonicalSwap, emptyAState(N6))
      and not(canReduceOnce(canonicalSwap, emptyAState(N3)))
      and not(canReduceOnce(canonicalSwap, emptyAState(N6)))
      and reduceOnce(canonicalSwap, emptyAState(N3)) == ReductionUnavailableA
      and reduceOnce(canonicalSwap, emptyAState(N6)) == ReductionUnavailableA)
  run invalidDomainDistinctTest = {
    val malformed = {...emptyAState(N0), accounts: Map()}
    assert(not(canReduceOnce(closeProgram, malformed))
      and reduceOnce(closeProgram, malformed) == ReductionOutsideDomainA
      and reduceOnce(payProgram(21), fundedState(N1, 10)) == ReductionOutsideDomainA)
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
