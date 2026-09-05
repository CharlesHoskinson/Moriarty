module candidate_a_programs {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"

  pure val aliceA: CoreAccount = {owner: Alice, asset: TokenA}
  pure val bobB: CoreAccount = {owner: Bob, asset: TokenB}
  pure val closeProgram: AProgram = {root: N0, nodes: A_NODE_IDS.mapBy(_ => CloseA)}
  pure def emptyAState(continuation: NodeId): AState = {continuation: continuation,
    accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: A_CHOICE_IDS.mapBy(_ => NoInt), minimumTime: Time0}
  // Literal encoding of frozen swap.py; unused nodes are CloseA.
  pure val canonicalSwap: AProgram = {root: N6, nodes: closeProgram.nodes
    .put(N1, PayA({account: bobB, payee: Alice, amount: ConstantA(20), continuation: N0}))
    .put(N2, PayA({account: aliceA, payee: Bob, amount: ConstantA(10), continuation: N1}))
    .put(N3, IfA({observation: ChoiceEqualsA({id: SettleId, expected: 1}), thenNode: N2, elseNode: N0}))
    .put(N4, WhenA({cases: List({caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: 1}),
      continuation: N3}), timeout: Time100, timeoutNode: N0}))
    .put(N5, WhenA({cases: List({caseAction: DepositA({account: bobB, depositor: Bob, amount: ConstantA(20)}),
      continuation: N4}), timeout: Time100, timeoutNode: N0}))
    .put(N6, WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(10)}),
      continuation: N5}), timeout: Time100, timeoutNode: N0}))}
  // Literal existing-constructor installment fixture: installment-two-when-v1.
  // Each Pay stops at the next unexpired When; fill2 needs a second transaction.
  pure val installmentProgram: AProgram = {root: N4, nodes: closeProgram.nodes
    .put(N1, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))
    .put(N2, WhenA({cases: List(
      {caseAction: ChoiceA({id: SecondFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N1},
      {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
      timeout: Time100, timeoutNode: N0}))
    .put(N3, PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N2}))
    .put(N4, WhenA({cases: List(
      {caseAction: ChoiceA({id: FirstFillId, chooser: Bob, lower: 1, upper: 1}), continuation: N3},
      {caseAction: ChoiceA({id: RecoveryId, chooser: Alice, lower: 1, upper: 1}), continuation: N0}),
      timeout: Time100, timeoutNode: N0}))}
}
