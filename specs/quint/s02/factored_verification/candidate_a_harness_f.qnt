module candidate_a_harness_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"

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

  pure def noDiagnostics(s: SwapTraceState): bool = s.records.foldl(true, (ok, entry) =>
    ok and match entry.payload { | SwapComputedA(_) => true | _ => false })
  // Outcomes are read from recorded actual computations, never selected in init.
  pure def recorded(s: SwapTraceState, input: AInput, now: ATime, predecessor: NodeId, accepted: bool): bool =
    s.records.foldl(false, (found, entry) => found or match entry.payload {
      | SwapComputedA(payload) => entry.request.program == canonicalSwap and entry.request.input == input
          and entry.request.now == now and entry.request.before.continuation == predecessor
          and payload.raw.accepted == accepted
      | _ => false
    })
  // These are corpus scheduling guards, not evaluator/authority validity gates.
  // In particular, no computation or extraction diagnostic is hidden here.
  pure def canRequest(s: SwapTraceState, input: AInput, now: ATime): bool =
    if (not(noDiagnostics(s)) or s.records.length() >= MAX_SWAP_RECORDS) false
    else if (s.records.length() == 0) s == swapInitial
      and ((input == aliceFunding and now == Time1) or (input == NoAInput and now == Time100))
    else if (s.records.length() == 1) s.agreement.continuation == N5
      and recorded(s, aliceFunding, Time1, N6, true)
      and ((input == bobFunding and now == Time2) or (input == NoAInput and now == Time100))
    else if (s.records.length() == 2) s.agreement.continuation == N4
      and recorded(s, aliceFunding, Time1, N6, true) and recorded(s, bobFunding, Time2, N5, true)
      and (((input == settleInput or input == refundInput) and now == Time2)
        or ((input == NoAInput or input == settleInput) and now == Time100))
    else s.agreement.continuation == N4
      and recorded(s, settleInput, Time100, N4, false) and input == NoAInput and now == Time100

  // Precondition: canRequest. A diagnostic is recorded and will fail safety,
  // not converted to a successful/failed Core transaction or a harmless no-op.
  pure def applyRequest(s: SwapTraceState, input: AInput, now: ATime): SwapTraceState = {
    val request = requestFor(s, input, now)
    val evaluated = computeTransaction(canonicalSwap, s.agreement, input, now)
    val payload: SwapTracePayload = match evaluated {
      | TransactionComputedA(raw) => {
          val projection = projectResult(canonicalSwap, raw)
          val extracted = extractCommittedEffects(canonicalSwap, s.agreement, input, now, raw)
          match extracted {
            | EffectsExtractedA(effects) => {
                val computed: SwapComputedPayload = {raw: raw, projection: projection, effects: effects}
                if (canApply(s.ledger, effects)) SwapComputedA(computed) else SwapLedgerDiagnosticA(computed)
              }
            | _ => SwapExtractionDiagnosticA({raw: raw, projection: projection, extraction: extracted})
          }
        }
      | _ => SwapComputationDiagnosticA(evaluated)
    }
    val ledgerAfter = match payload {
      | SwapComputedA(computed) => applyTransfers(s.ledger, computed.effects)
      | _ => s.ledger
    }
    val agreementAfter = match payload {
      | SwapComputedA(computed) => if (computed.raw.accepted) computed.raw.state else s.agreement
      | _ => s.agreement
    }
    {agreement: agreementAfter, ledger: ledgerAfter, records: s.records.append({
      request: request, beforeLedger: s.ledger, afterLedger: ledgerAfter, payload: payload})}
  }

  pure def traceTerminal(s: SwapTraceState): bool =
    if (s.records.length() == 0 or s.records.length() > MAX_SWAP_RECORDS or not(noDiagnostics(s))) false
    else match s.records.nth(s.records.length() - 1).payload {
      | SwapComputedA(payload) => payload.raw.accepted and s.agreement.continuation == N0
          and payload.raw.state == s.agreement
          and s.records.nth(s.records.length() - 1).afterLedger == s.ledger
      | _ => false
    }
  pure def coupledMoney(agreement: AState, ledger: Ledger): bool =
    if (not(validState(canonicalSwap, agreement)) or not(validLedger(ledger))) false
    else CORE_ACCOUNTS.forall(account =>
      agreement.accounts.get(account) == balance(ledger, Escrow(account), account.asset))
      and ASSETS.forall(asset => totalAsset(ledger, asset) == totalAsset(swapInitial.ledger, asset))
  pure def actualDeposit(input: AInput, account: CoreAccount): int = match input {
    | NoAInput => 0
    | PresentAInput(supplied) => match supplied {
        | ChoiceInputA(_) => 0
        | DepositInputA(deposit) => if (deposit.account == account) deposit.quantity else 0
      }
  }
  // Direct account arithmetic complements the producer's extraction check;
  // this is a local invariant, not an independent Python correspondence result.
  pure def accountsFollowRaw(request: SwapTraceRequest, raw: ATransactionResult): bool =
    CORE_ACCOUNTS.forall(account => raw.state.accounts.get(account) == request.before.accounts.get(account)
      + (if (raw.accepted) actualDeposit(request.input, account) else 0)
      - raw.payments.foldl(0, (total, payment) =>
          total + (if (payment.source == account) payment.quantity else 0)))
  pure def recordSafety(prefix: SwapTraceState, entry: SwapTraceRecord): bool = match entry.payload {
    | SwapComputedA(payload) =>
        if (not(validState(canonicalSwap, entry.request.before))
          or not(validState(canonicalSwap, payload.raw.state))) false
        else entry.request.program == canonicalSwap and entry.request.before == prefix.agreement
          and entry.beforeLedger == prefix.ledger
          and canRequest(prefix, entry.request.input, entry.request.now)
          and computeTransaction(entry.request.program, entry.request.before, entry.request.input, entry.request.now)
            == TransactionComputedA(payload.raw)
          and payload.projection == projectResult(entry.request.program, payload.raw)
          and extractCommittedEffects(entry.request.program, entry.request.before, entry.request.input,
            entry.request.now, payload.raw) == EffectsExtractedA(payload.effects)
          and canApply(entry.beforeLedger, payload.effects)
          and entry.afterLedger == applyTransfers(entry.beforeLedger, payload.effects)
          and coupledMoney(payload.raw.state, entry.afterLedger)
          and accountsFollowRaw(entry.request, payload.raw)
          and validCoreObservation(A_CHOICE_IDS.map(id => choiceName(id)),
            projectState(entry.request.program, entry.request.before), payload.projection, payload.effects)
          and (payload.raw.accepted or (payload.raw.state == entry.request.before
            and payload.raw.payments == List() and payload.raw.warnings == List() and payload.raw.reductions == 0
            and payload.effects == List() and entry.beforeLedger == entry.afterLedger))
    | _ => false
  }
  type SwapReplay = {ok: bool, trace: SwapTraceState}
  pure def traceSafety(s: SwapTraceState): bool =
    if (s.records.length() > MAX_SWAP_RECORDS or not(noDiagnostics(s))
        or not(coupledMoney(s.agreement, s.ledger))) false
    else {
      val initial: SwapReplay = {ok: true, trace: swapInitial}
      val replayed = s.records.foldl(initial, (acc, entry) => {
        val agreementAfter = match entry.payload {
          | SwapComputedA(payload) => if (payload.raw.accepted) payload.raw.state else acc.trace.agreement
          | _ => acc.trace.agreement
        }
        {ok: acc.ok and recordSafety(acc.trace, entry),
          trace: {agreement: agreementAfter, ledger: entry.afterLedger, records: acc.trace.records.append(entry)}}
      })
      replayed.ok and replayed.trace == s and (traceTerminal(s) != traceEnabled(s))
    }
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
