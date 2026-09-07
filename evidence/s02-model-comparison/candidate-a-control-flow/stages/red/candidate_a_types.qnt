module candidate_a_types {
  import effects.* from "./effects"
  import observations.* from "./observations"

  type NodeId = N0 | N1 | N2 | N3 | N4 | N5 | N6 | N7
    | N8 | N9 | N10 | N11 | N12 | N13 | N14 | N15
  type AChoiceId = SettleId | FirstFillId | SecondFillId | RecoveryId | OtherId
  type ATime = Time0 | Time1 | Time2 | Time100 | Time101
  type AValue = ConstantA(int)
  type AObservation = ChoiceEqualsA({id: AChoiceId, expected: int})
  type AAction = DepositA({account: CoreAccount, depositor: Principal, amount: AValue})
    | ChoiceA({id: AChoiceId, chooser: Principal, lower: int, upper: int})
  type ACase = {caseAction: AAction, continuation: NodeId}
  type ANode = CloseA
    | PayA({account: CoreAccount, payee: Principal, amount: AValue, continuation: NodeId})
    | IfA({observation: AObservation, thenNode: NodeId, elseNode: NodeId})
    | WhenA({cases: List[ACase], timeout: ATime, timeoutNode: NodeId})
  type AProgram = {root: NodeId, nodes: NodeId -> ANode}
  type AState = {continuation: NodeId, accounts: CoreAccount -> int,
    choices: AChoiceId -> OptionalInt, minimumTime: ATime}
  type ASuppliedInput = DepositInputA({account: CoreAccount, depositor: Principal, quantity: int})
    | ChoiceInputA({id: AChoiceId, chooser: Principal, chosen: int})
  type AInput = NoAInput | PresentAInput(ASuppliedInput)
  type AReduction = {state: AState, payments: List[CorePayment], warnings: List[CoreWarning], reductions: int}
  // Development/domain diagnostics are not Core errors or semantic outcomes.
  // Task 3 must remove ReductionUnavailableA for every valid frozen constructor.
  type AReductionStep = QuiescentA(AState) | ReducedA(AReduction)
    | ReductionUnavailableA | ReductionOutsideDomainA
  type AReductionEvaluation = ReductionCompleteA(AReduction) | ReductionBoundFailureA | QuiescenceOutsideDomainA
  type AInputResult = {state: AState, error: CoreError}
  // These diagnostics are not Core errors or successful input applications.
  type AInputEvaluation = InputAppliedA(AInputResult) | InputOutsideDomainA | InputPreconditionFailureA
  type ATransactionResult = {accepted: bool, state: AState, error: CoreError,
    payments: List[CorePayment], warnings: List[CoreWarning], reductions: int}
  type ATransactionEvaluation = TransactionComputedA(ATransactionResult) | OutsideModelDomainA
  type APredecessor = {program: AProgram, state: AState}
  type AContinuation = {program: AProgram, node: NodeId}

  pure val A_NODE_IDS = Set(N0, N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15)
  pure val A_CHOICE_IDS = Set(SettleId, FirstFillId, SecondFillId, RecoveryId, OtherId)
  pure val A_TIMES = Set(Time0, Time1, Time2, Time100, Time101)
  pure val A_CONSTANTS = Set(-1, 0, 1, 5, 10, 20)
  pure val A_CHOICE_VALUES = Set(-1, 0, 1, 2)
  pure val A_DEPOSIT_QUANTITIES = Set(-1, 0, 1, 5, 10, 20, 21)
  pure val A_BALANCE_LIMIT = 340
  pure val CANONICAL_ACCOUNTS: List[CoreAccount] = List(
    {owner: Alice, asset: TokenA}, {owner: Alice, asset: TokenB},
    {owner: Bob, asset: TokenA}, {owner: Bob, asset: TokenB},
    {owner: Mallory, asset: TokenA}, {owner: Mallory, asset: TokenB})

  pure def timeValue(time: ATime): int = match time {
    | Time0 => 0 | Time1 => 1 | Time2 => 2 | Time100 => 100 | Time101 => 101
  }
  pure def choiceName(id: AChoiceId): str = match id {
    | SettleId => "settle" | FirstFillId => "fill1" | SecondFillId => "fill2"
    | RecoveryId => "recover" | OtherId => "other"
  }
  pure def rank(node: NodeId): int = Map(N0 -> 0, N1 -> 1, N2 -> 2, N3 -> 3,
    N4 -> 4, N5 -> 5, N6 -> 6, N7 -> 7, N8 -> 8, N9 -> 9, N10 -> 10,
    N11 -> 11, N12 -> 12, N13 -> 13, N14 -> 14, N15 -> 15).get(node)
  pure def validValue(value: AValue): bool = match value {
    | ConstantA(quantity) => A_CONSTANTS.contains(quantity)
  }
  pure def validObservation(observation: AObservation): bool = match observation {
    | ChoiceEqualsA(choice) => A_CHOICE_VALUES.contains(choice.expected)
  }
  pure def validAction(candidateAction: AAction): bool = match candidateAction {
    | DepositA(deposit) => validValue(deposit.amount)
    | ChoiceA(choice) => A_CHOICE_VALUES.contains(choice.lower)
        and A_CHOICE_VALUES.contains(choice.upper) and choice.lower <= choice.upper
  }
  pure def validNode(id: NodeId, node: ANode): bool = match node {
    | CloseA => true
    | PayA(pay) => validValue(pay.amount) and rank(pay.continuation) < rank(id)
    | IfA(branch) => validObservation(branch.observation)
        and rank(branch.thenNode) < rank(id) and rank(branch.elseNode) < rank(id)
    | WhenA(waiting) => waiting.cases.length() <= 2
        and rank(waiting.timeoutNode) < rank(id)
        and waiting.cases.foldl(true, (ok, oneCase) => ok and validAction(oneCase.caseAction)
          and rank(oneCase.continuation) < rank(id))
  }
  pure def validProgram(program: AProgram): bool =
    if (program.nodes.keys() == A_NODE_IDS)
      A_NODE_IDS.forall(id => validNode(id, program.nodes.get(id)))
    else false
  // Full zero-valued account maps encode Python's sparse absent accounts.
  // The potential bound is checked, never imposed by arithmetic saturation.
  pure def validState(program: AProgram, state: AState): bool =
    if (validProgram(program) and state.accounts.keys() == CORE_ACCOUNTS
        and state.choices.keys() == A_CHOICE_IDS)
      CORE_ACCOUNTS.forall(account => state.accounts.get(account) >= 0
        and state.accounts.get(account) + 20 * rank(state.continuation) <= A_BALANCE_LIMIT)
      and A_CHOICE_IDS.forall(id => match state.choices.get(id) {
        | NoInt => true | IntValue(value) => A_CHOICE_VALUES.contains(value)
      })
    else false
  pure def validInput(input: AInput): bool = match input {
    | NoAInput => true
    | PresentAInput(supplied) => match supplied {
        | DepositInputA(deposit) => A_DEPOSIT_QUANTITIES.contains(deposit.quantity)
        | ChoiceInputA(choice) => A_CHOICE_VALUES.contains(choice.chosen)
      }
  }
}
