module candidate_a_cases {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_core.* from "./candidate_a_core"
  import candidate_a_projection.* from "./candidate_a_projection"

  // A finite literal request corpus, evaluated at initialization. These are
  // diagnostic vectors, NOT stateful lifecycle witnesses or expected results.
  type DiagnosticRequest = {program: AProgram, before: AState, input: AInput, now: ATime}
  type NamedRequest = {case_id: str, fixture_id: str, request: DiagnosticRequest}
  type CasePayload = CaseComputedA({raw: ATransactionResult,
      projection: CoreResultObservation[AContinuation], effects: List[Transfer]})
    | CaseComputationDiagnosticA(ATransactionEvaluation)
    | CaseExtractionDiagnosticA({raw: ATransactionResult,
        projection: CoreResultObservation[AContinuation], extraction: AEffectExtraction})
  type DiagnosticCase = {case_id: str, fixture_id: str, request: DiagnosticRequest, payload: CasePayload}

  pure def payFixture(quantity: int): AProgram = {root: N1, nodes: closeProgram.nodes.put(N1,
    PayA({account: aliceA, payee: Bob, amount: ConstantA(quantity), continuation: N0}))}
  pure def depositFixture(quantity: int): AProgram = {root: N1, nodes: closeProgram.nodes.put(N1,
    WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(quantity)}),
      continuation: N0}), timeout: Time100, timeoutNode: N0}))}
  pure val ifZeroFixture: AProgram = {root: N2, nodes: payFixture(5).nodes.put(N2,
    IfA({observation: ChoiceEqualsA({id: SettleId, expected: 0}), thenNode: N1, elseNode: N0}))}
  pure def preDepositFixture(quantity: int): AProgram = {root: N3, nodes: payFixture(5).nodes
    .put(N2, WhenA({cases: List({caseAction: DepositA({account: aliceA, depositor: Alice, amount: ConstantA(5)}),
      continuation: N1}), timeout: Time100, timeoutNode: N0}))
    .put(N3, PayA({account: aliceA, payee: Bob, amount: ConstantA(quantity), continuation: N2}))}
  pure def orderedFixture(overlap: bool): AProgram = {root: N3, nodes: payFixture(5).nodes
    .put(N2, PayA({account: aliceA, payee: Mallory, amount: ConstantA(5), continuation: N0}))
    .put(N3, WhenA({cases: List(
      {caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: 0, upper: if (overlap) 1 else 0}), continuation: N1},
      {caseAction: ChoiceA({id: SettleId, chooser: Bob, lower: if (overlap) 0 else 1, upper: 1}), continuation: N2}),
      timeout: Time100, timeoutNode: N0}))}
  pure def funded(node: NodeId, quantity: int): AState = {...emptyAState(node),
    accounts: emptyAState(node).accounts.put(aliceA, quantity)}
  pure def depositInput(quantity: int): AInput = PresentAInput(
    DepositInputA({account: aliceA, depositor: Alice, quantity: quantity}))
  pure def choiceInput(chooser: Principal, quantity: int): AInput = PresentAInput(
    ChoiceInputA({id: SettleId, chooser: chooser, chosen: quantity}))
  pure val fundedSwap = {...funded(N4, 10), accounts: funded(N4, 10).accounts.put(bobB, 20), minimumTime: Time2}
  pure val fullCloseState: AState = {continuation: N0,
    accounts: CANONICAL_ACCOUNTS.indices().fold(emptyAState(N0).accounts,
      (accounts, index) => accounts.put(CANONICAL_ACCOUNTS.nth(index), index + 1)),
    choices: Map(SettleId -> IntValue(0), FirstFillId -> IntValue(-1),
      SecondFillId -> IntValue(1), RecoveryId -> IntValue(2), OtherId -> NoInt), minimumTime: Time100}
  pure def named(id: str, fixture: str, program: AProgram, before: AState, input: AInput, now: ATime): NamedRequest =
    {case_id: id, fixture_id: fixture, request: {program: program, before: before, input: input, now: now}}
  pure val CASE_REQUESTS: List[NamedRequest] = List(
    named("error-time-before-state", "canonical-swap-v1", canonicalSwap, fundedSwap, choiceInput(Bob, 1), Time1),
    named("error-contract-closed", "close-v1", closeProgram, emptyAState(N0), NoAInput, Time2),
    named("error-input-required", "canonical-swap-v1", canonicalSwap, emptyAState(N6), NoAInput, Time1),
    named("error-no-matching-input", "canonical-swap-v1", canonicalSwap, emptyAState(N6),
      PresentAInput(DepositInputA({account: bobB, depositor: Bob, quantity: 20})), Time1),
    named("error-choice-out-of-bounds", "canonical-swap-v1", canonicalSwap, fundedSwap, choiceInput(Bob, 2), Time2),
    named("error-non-positive-deposit", "deposit-zero-close-v1", depositFixture(0), emptyAState(N1), depositInput(0), Time2),
    named("warning-zero", "pay-zero-v1", payFixture(0), funded(N1, 3), NoAInput, Time2),
    named("warning-negative", "pay-negative-v1", payFixture(-1), emptyAState(N1), NoAInput, Time2),
    named("warning-partial", "pay-ten-v1", payFixture(10), funded(N1, 4), NoAInput, Time2),
    named("warning-empty-account", "pay-ten-v1", payFixture(10), emptyAState(N1), NoAInput, Time2),
    named("pre-deposit-post", "pre-deposit-post-v1", preDepositFixture(1), funded(N3, 1), depositInput(5), Time2),
    named("speculative-payment-rejected", "pre-deposit-post-v1", preDepositFixture(1), funded(N3, 1), depositInput(10), Time2),
    named("speculative-warning-rejected", "pre-warning-deposit-v1", preDepositFixture(0), funded(N3, 1), depositInput(10), Time2),
    named("deposit-immediate-refund", "deposit-five-close-v1", depositFixture(5), emptyAState(N1), depositInput(5), Time2),
    named("deposit-zero-mismatch", "deposit-five-close-v1", depositFixture(5), emptyAState(N1), depositInput(0), Time2),
    named("deposit-without-payment", "canonical-swap-v1", canonicalSwap, emptyAState(N6), depositInput(10), Time1),
    named("absent-choice", "if-settle-zero-v1", ifZeroFixture, funded(N2, 5), NoAInput, Time2),
    named("stored-zero-choice", "if-settle-zero-v1", ifZeroFixture,
      {...funded(N2, 5), choices: funded(N2, 5).choices.put(SettleId, IntValue(0))}, NoAInput, Time2),
    named("ordered-first", "ordered-choice-v1", orderedFixture(false), funded(N3, 5), choiceInput(Bob, 0), Time2),
    named("ordered-fallthrough", "ordered-choice-v1", orderedFixture(false), funded(N3, 5), choiceInput(Bob, 1), Time2),
    named("ordered-overlap", "ordered-choice-overlap-v1", orderedFixture(true), funded(N3, 5), choiceInput(Bob, 1), Time2),
    named("ordered-no-acceptance", "ordered-choice-v1", orderedFixture(false), funded(N3, 5), choiceInput(Bob, 2), Time2),
    named("ordered-wrong-chooser", "ordered-choice-v1", orderedFixture(false), funded(N3, 5), choiceInput(Alice, 1), Time2),
    named("close-full-state", "close-v1", closeProgram, fullCloseState, NoAInput, Time101))

  pure def computeCase(source: NamedRequest): DiagnosticCase = {
    case_id: source.case_id, fixture_id: source.fixture_id, request: source.request,
    payload: CaseComputationDiagnosticA(OutsideModelDomainA),
  }
  pure def completeCases(cases: List[DiagnosticCase]): bool = cases.length() == CASE_REQUESTS.length()
    and cases.indices().forall(index => {
      val actual = cases.nth(index)
      val source = CASE_REQUESTS.nth(index)
      actual.case_id == source.case_id and actual.fixture_id == source.fixture_id and actual.request == source.request
        and match actual.payload { | CaseComputedA(_) => true | _ => false }
    })
  var diagnosticCases: List[DiagnosticCase]
  action init = diagnosticCases' = CASE_REQUESTS.foldl(List(), (items, request) => items.append(computeCase(request)))
  action step = all {false, diagnosticCases' = diagnosticCases}
  val corpusComplete = completeCases(diagnosticCases)
}
