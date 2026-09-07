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
  type AReductionEvaluation = ReductionCompleteA(AReduction) | ReductionBoundFailureA
  type AInputResult = {state: AState, error: CoreError}
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

  // Typed scaffolding: domain logic is deliberately disabled for Task 1 RED.
  pure def timeValue(time: ATime): int = 0
  pure def choiceName(id: AChoiceId): str = "other"
  pure def rank(node: NodeId): int = 0
  pure def validProgram(program: AProgram): bool = false
  pure def validState(program: AProgram, state: AState): bool = false
  pure def validInput(input: AInput): bool = false
}
