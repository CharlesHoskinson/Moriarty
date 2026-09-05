module candidate_a_installment_harness {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_core.* from "./candidate_a_core"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_projection.* from "./candidate_a_projection"

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
  // Temporary non-Core scaffold diagnostic, removed after behavioral RED.
  type CancellationAdapterEvaluation = CancellationAdapterUnavailableA | CancellationAdaptedA(CancellationView)
  pure def cancellationAdapter(before: AState): CancellationAdapterEvaluation = CancellationAdapterUnavailableA
  pure def canRequest(s: InstallmentTraceState, input: AInput, now: ATime): bool = false
  pure def applyRequest(s: InstallmentTraceState, input: AInput, now: ATime): InstallmentTraceState = s
  pure def traceSafety(s: InstallmentTraceState): bool = false
  pure def traceTerminal(s: InstallmentTraceState): bool = false
  pure def recorded(s: InstallmentTraceState, input: AInput, now: ATime, predecessor: NodeId, accepted: bool): bool = false
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
