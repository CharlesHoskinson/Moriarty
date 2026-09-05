module candidate_a_installment_harness_f {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"

  // Independent agreement-only shared state; no imported swap/lifecycle state.
  type InstallmentTraceRequest = {program: AProgram, before: AState, input: AInput, now: ATime}
  type InstallmentComputedPayload = {raw: ATransactionResult,
    projection: CoreResultObservation[AContinuation], effects: List[Transfer]}
  type InstallmentTracePayload = InstallmentComputedA(InstallmentComputedPayload)
    | InstallmentComputationDiagnosticA(ATransactionEvaluation)
    | InstallmentExtractionDiagnosticA({raw: ATransactionResult,
        projection: CoreResultObservation[AContinuation], extraction: AEffectExtraction})
    | InstallmentLedgerDiagnosticA(InstallmentComputedPayload)
  type InstallmentTraceRecord = {request: InstallmentTraceRequest, beforeLedger: Ledger,
    afterLedger: Ledger, payload: InstallmentTracePayload}
  type InstallmentTraceState = {agreement: AState, ledger: Ledger, records: List[InstallmentTraceRecord]}
  pure val MAX_INSTALLMENT_RECORDS = 3
  // Root-approved Time2 differs explicitly from the Python vector's initial
  // minimum time 1. The independent checker must use this actual exported state.
  pure val installmentInitial: InstallmentTraceState = {
    agreement: {...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10), minimumTime: Time2},
    ledger: initialLedger(0, 0).put((Escrow(aliceA), TokenA), 10), records: List()}
  pure val fillOneInput = PresentAInput(ChoiceInputA({id: FirstFillId, chooser: Bob, chosen: 1}))
  pure val fillTwoInput = PresentAInput(ChoiceInputA({id: SecondFillId, chooser: Bob, chosen: 1}))
  pure val recoveryInput = PresentAInput(ChoiceInputA({id: RecoveryId, chooser: Alice, chosen: 1}))
  pure def requestFor(s: InstallmentTraceState, input: AInput, now: ATime): InstallmentTraceRequest =
    {program: installmentProgram, before: s.agreement, input: input, now: now}

  type CancellationView = {predecessor: APredecessor, proposedSuccessor: APredecessor,
    effects: List[Transfer], coreProjection: CoreProjection[AContinuation]}
  // Pure envelope-side view only: no Core transaction or result is fabricated.
  // Recovery remains independently agreement-legal before any cancellation.
  type CancellationAdapterEvaluation = CancellationAdaptedA(CancellationView)
  pure def cancellationAdapter(before: AState): CancellationAdapterEvaluation =
    CancellationAdaptedA({predecessor: {program: installmentProgram, state: before},
      proposedSuccessor: {program: installmentProgram, state: before},
      effects: List(), coreProjection: NoCoreProjection})
  pure def noDiagnostics(s: InstallmentTraceState): bool = s.records.foldl(true, (ok, entry) =>
    ok and match entry.payload { | InstallmentComputedA(_) => true | _ => false })
  // Outcomes are read from recorded actual computations, never selected in init.
  pure def recorded(s: InstallmentTraceState, input: AInput, now: ATime, predecessor: NodeId, accepted: bool): bool =
    s.records.foldl(false, (found, entry) => found or match entry.payload {
      | InstallmentComputedA(payload) => entry.request.program == installmentProgram and entry.request.input == input
          and entry.request.now == now and entry.request.before.continuation == predecessor
          and payload.raw.accepted == accepted
      | _ => false
    })
  // These are corpus scheduling guards, not evaluator/authority validity gates.
  // In particular, no computation or extraction diagnostic is hidden here.
  pure def canRequest(s: InstallmentTraceState, input: AInput, now: ATime): bool =
    if (not(noDiagnostics(s)) or s.records.length() >= MAX_INSTALLMENT_RECORDS) false
    else if (recorded(s, recoveryInput, Time100, N4, false)
      or recorded(s, recoveryInput, Time100, N2, false))
      (s.agreement.continuation == N4 or s.agreement.continuation == N2)
        and input == NoAInput and now == Time100
    else if (s.records.length() == 0) s == installmentInitial
      and (((input == fillOneInput or input == recoveryInput) and now == Time2)
        or ((input == NoAInput or input == recoveryInput) and now == Time100))
    else if (s.records.length() == 1) s.agreement.continuation == N2
      and recorded(s, fillOneInput, Time2, N4, true)
      and (((input == fillTwoInput or input == recoveryInput) and now == Time2)
        or ((input == NoAInput or input == recoveryInput) and now == Time100))
    else false

  // Precondition: canRequest. A diagnostic is recorded and will fail safety,
  // not converted to a successful/failed Core transaction or a harmless no-op.
  pure def applyRequest(s: InstallmentTraceState, input: AInput, now: ATime): InstallmentTraceState = {
    val request = requestFor(s, input, now)
    val evaluated = computeTransaction(installmentProgram, s.agreement, input, now)
    val payload: InstallmentTracePayload = match evaluated {
      | TransactionComputedA(raw) => {
          val projection = projectResult(installmentProgram, raw)
          val extracted = extractCommittedEffects(installmentProgram, s.agreement, input, now, raw)
          match extracted {
            | EffectsExtractedA(effects) => {
                val computed: InstallmentComputedPayload = {raw: raw, projection: projection, effects: effects}
                if (canApply(s.ledger, effects)) InstallmentComputedA(computed) else InstallmentLedgerDiagnosticA(computed)
              }
            | _ => InstallmentExtractionDiagnosticA({raw: raw, projection: projection, extraction: extracted})
          }
        }
      | _ => InstallmentComputationDiagnosticA(evaluated)
    }
    val ledgerAfter = match payload {
      | InstallmentComputedA(computed) => applyTransfers(s.ledger, computed.effects)
      | _ => s.ledger
    }
    val agreementAfter = match payload {
      | InstallmentComputedA(computed) => if (computed.raw.accepted) computed.raw.state else s.agreement
      | _ => s.agreement
    }
    {agreement: agreementAfter, ledger: ledgerAfter, records: s.records.append({
      request: request, beforeLedger: s.ledger, afterLedger: ledgerAfter, payload: payload})}
  }

  pure def traceTerminal(s: InstallmentTraceState): bool =
    if (s.records.length() == 0 or s.records.length() > MAX_INSTALLMENT_RECORDS or not(noDiagnostics(s))) false
    else match s.records.nth(s.records.length() - 1).payload {
      | InstallmentComputedA(payload) => payload.raw.accepted and s.agreement.continuation == N0
          and payload.raw.state == s.agreement
          and s.records.nth(s.records.length() - 1).afterLedger == s.ledger
      | _ => false
    }
  pure def coupledMoney(agreement: AState, ledger: Ledger): bool =
    if (not(validState(installmentProgram, agreement)) or not(validLedger(ledger))) false
    else CORE_ACCOUNTS.forall(account =>
      agreement.accounts.get(account) == balance(ledger, Escrow(account), account.asset))
      and ASSETS.forall(asset => totalAsset(ledger, asset) == totalAsset(installmentInitial.ledger, asset))
  // Direct account arithmetic complements the producer's extraction check;
  // this is a local invariant, not an independent Python correspondence result.
  pure def accountsFollowRaw(request: InstallmentTraceRequest, raw: ATransactionResult): bool =
    CORE_ACCOUNTS.forall(account => raw.state.accounts.get(account) == request.before.accounts.get(account)
      - raw.payments.foldl(0, (total, payment) =>
          total + (if (payment.source == account) payment.quantity else 0)))
  pure def recordSafety(prefix: InstallmentTraceState, entry: InstallmentTraceRecord): bool = match entry.payload {
    | InstallmentComputedA(payload) =>
        if (not(validState(installmentProgram, entry.request.before))
          or not(validState(installmentProgram, payload.raw.state))) false
        else entry.request.program == installmentProgram and entry.request.before == prefix.agreement
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
  type InstallmentReplay = {ok: bool, trace: InstallmentTraceState}
  pure def traceSafety(s: InstallmentTraceState): bool =
    if (s.records.length() > MAX_INSTALLMENT_RECORDS or not(noDiagnostics(s))
        or not(coupledMoney(s.agreement, s.ledger))) false
    else {
      val initial: InstallmentReplay = {ok: true, trace: installmentInitial}
      val replayed = s.records.foldl(initial, (acc, entry) => {
        val agreementAfter = match entry.payload {
          | InstallmentComputedA(payload) => if (payload.raw.accepted) payload.raw.state else acc.trace.agreement
          | _ => acc.trace.agreement
        }
        {ok: acc.ok and recordSafety(acc.trace, entry),
          trace: {agreement: agreementAfter, ledger: entry.afterLedger, records: acc.trace.records.append(entry)}}
      })
      replayed.ok and replayed.trace == s and (traceTerminal(s) != traceEnabled(s))
    }
  pure def traceEnabled(s: InstallmentTraceState): bool = canRequest(s, fillOneInput, Time2)
    or canRequest(s, fillTwoInput, Time2) or canRequest(s, recoveryInput, Time2)
    or canRequest(s, NoAInput, Time100) or canRequest(s, recoveryInput, Time100)

  var installmentTrace: InstallmentTraceState
  action init = installmentTrace' = installmentInitial
  action submit(input: AInput, now: ATime): bool = all {
    canRequest(installmentTrace, input, now), installmentTrace' = applyRequest(installmentTrace, input, now)}
  action fillFirst = submit(fillOneInput, Time2)
  action fillSecond = submit(fillTwoInput, Time2)
  action recoverTen = all {installmentTrace.agreement.continuation == N4, submit(recoveryInput, Time2)}
  action recoverFive = all {installmentTrace.agreement.continuation == N2, submit(recoveryInput, Time2)}
  action timeoutTen = all {installmentTrace.agreement.continuation == N4, submit(NoAInput, Time100)}
  action timeoutFive = all {installmentTrace.agreement.continuation == N2, submit(NoAInput, Time100)}
  action rejectRecoveryTen = all {installmentTrace.agreement.continuation == N4, submit(recoveryInput, Time100)}
  action rejectRecoveryFive = all {installmentTrace.agreement.continuation == N2, submit(recoveryInput, Time100)}
  action step = any {fillFirst, fillSecond, recoverTen, recoverFive,
    timeoutTen, timeoutFive, rejectRecoveryTen, rejectRecoveryFive}
  val firstAgreementFill = recorded(installmentTrace, fillOneInput, Time2, N4, true)
  val secondAgreementFill = recorded(installmentTrace, fillTwoInput, Time2, N2, true)
  val agreementRefundTen = recorded(installmentTrace, recoveryInput, Time2, N4, true)
  val agreementRefundFive = recorded(installmentTrace, recoveryInput, Time2, N2, true)
  val agreementDeadlineTen = recorded(installmentTrace, NoAInput, Time100, N4, true)
  val agreementDeadlineFive = recorded(installmentTrace, NoAInput, Time100, N2, true)
  val agreementDeadlineRecoveryTenRejected = recorded(installmentTrace, recoveryInput, Time100, N4, false)
  val agreementDeadlineRecoveryFiveRejected = recorded(installmentTrace, recoveryInput, Time100, N2, false)
  val installmentTerminal = traceTerminal(installmentTrace)
  val installmentTraceSafety = traceSafety(installmentTrace)
}
