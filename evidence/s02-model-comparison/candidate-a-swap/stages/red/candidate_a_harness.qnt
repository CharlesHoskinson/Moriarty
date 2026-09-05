module candidate_a_harness {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_core.* from "./candidate_a_core"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_projection.* from "./candidate_a_projection"

  // Agreement-only shared state. The ledger is neutral accounting, not authority.
  type SwapTraceRequest = {program: AProgram, before: AState, input: AInput, now: ATime}
  type SwapComputedPayload = {raw: ATransactionResult,
    projection: CoreResultObservation[AContinuation], effects: List[Transfer]}
  type SwapTracePayload = SwapComputedA(SwapComputedPayload)
    | SwapComputationDiagnosticA(ATransactionEvaluation)
    | SwapExtractionDiagnosticA({raw: ATransactionResult,
        projection: CoreResultObservation[AContinuation], extraction: AEffectExtraction})
    | SwapLedgerDiagnosticA(SwapComputedPayload)
  type SwapTraceRecord = {request: SwapTraceRequest, beforeLedger: Ledger,
    afterLedger: Ledger, payload: SwapTracePayload}
  type SwapTraceState = {agreement: AState, ledger: Ledger, records: List[SwapTraceRecord]}
  pure val MAX_SWAP_RECORDS = 4
  pure val swapInitial: SwapTraceState = {
    agreement: emptyAState(N6), ledger: initialLedger(10, 20), records: List()}
  pure val aliceFunding = PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10}))
  pure val bobFunding = PresentAInput(DepositInputA({account: bobB, depositor: Bob, quantity: 20}))
  pure val settleInput = PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 1}))
  pure val refundInput = PresentAInput(ChoiceInputA({id: SettleId, chooser: Bob, chosen: 0}))
  pure def requestFor(s: SwapTraceState, input: AInput, now: ATime): SwapTraceRequest =
    {program: canonicalSwap, before: s.agreement, input: input, now: now}

  // Typed behavioral RED scaffolding; no transactions are recorded yet.
  pure def canRequest(s: SwapTraceState, input: AInput, now: ATime): bool = false
  pure def applyRequest(s: SwapTraceState, input: AInput, now: ATime): SwapTraceState = s
  pure def traceSafety(s: SwapTraceState): bool = false
  pure def traceTerminal(s: SwapTraceState): bool = false
  pure def recorded(s: SwapTraceState, input: AInput, now: ATime, predecessor: NodeId, accepted: bool): bool = false
  pure def traceEnabled(s: SwapTraceState): bool = canRequest(s, aliceFunding, Time1)
    or canRequest(s, bobFunding, Time2) or canRequest(s, settleInput, Time2)
    or canRequest(s, refundInput, Time2) or canRequest(s, NoAInput, Time100)
    or canRequest(s, settleInput, Time100)

  var swapTrace: SwapTraceState
  action init = swapTrace' = swapInitial
  action submit(input: AInput, now: ATime): bool = all {
    canRequest(swapTrace, input, now), swapTrace' = applyRequest(swapTrace, input, now)}
  action depositAlice = submit(aliceFunding, Time1)
  action depositBob = submit(bobFunding, Time2)
  action settleSwap = submit(settleInput, Time2)
  action refundSwap = submit(refundInput, Time2)
  action timeoutEmpty = all {swapTrace.agreement.continuation == N6, submit(NoAInput, Time100)}
  action timeoutAlice = all {swapTrace.agreement.continuation == N5, submit(NoAInput, Time100)}
  action timeoutFunded = all {swapTrace.agreement.continuation == N4, submit(NoAInput, Time100)}
  action timeoutInput = submit(settleInput, Time100)
  action step = any {depositAlice, depositBob, settleSwap, refundSwap,
    timeoutEmpty, timeoutAlice, timeoutFunded, timeoutInput}

  val aliceDeposited = recorded(swapTrace, aliceFunding, Time1, N6, true)
  val bobDeposited = recorded(swapTrace, bobFunding, Time2, N5, true)
  val swapSettled = recorded(swapTrace, settleInput, Time2, N4, true)
  val swapVoluntaryRefunded = recorded(swapTrace, refundInput, Time2, N4, true)
  val swapDeadlineCommitted = recorded(swapTrace, NoAInput, Time100, N4, true)
  val swapDeadlineInputRejected = recorded(swapTrace, settleInput, Time100, N4, false)
  val swapAliceOnlyTimedOut = recorded(swapTrace, NoAInput, Time100, N5, true)
  val swapEmptyTimedOut = recorded(swapTrace, NoAInput, Time100, N6, true)
  val swapTerminal = traceTerminal(swapTrace)
  val coreTraceSafety = traceSafety(swapTrace)
}
