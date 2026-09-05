module candidate_a_integrated_export_test {
import effects.* from "./effects"
import consumption.* from "./consumption"
import observations.* from "./observations"
import policies.* from "./policies"
import authorization.* from "./authorization"
import execution.* from "./execution"
import candidate_a_types.* from "./candidate_a_types"
import candidate_a_programs.* from "./candidate_a_programs"
import candidate_a_core.* from "./candidate_a_core"
import candidate_a_projection.* from "./candidate_a_projection"
import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
import candidate_a_authority_installment_fixtures.* from "./candidate_a_authority_installment_fixtures"
import candidate_a_authority_installment.* from "./candidate_a_authority_installment"
import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"
import candidate_a_authority_swap.* from "./candidate_a_authority_swap"
import candidate_a_integrated_observer.* from "./candidate_a_integrated_observer"
import candidate_a_integrated_lowering.* from "./candidate_a_integrated_lowering"
import candidate_a_integrated_cases.* from "./candidate_a_integrated_cases"
import candidate_a_integrated_case_000 as I from "./candidate_a_integrated_case_000"
import candidate_a_integrated_case_032 as S from "./candidate_a_integrated_case_032"

run actualDepositObservationTest = assert({
  val request = {before: beforeS(0), input: PresentAInput(DepositInputA({
    account: aliceA, depositor: Alice, quantity: 10})), now: Time1}
  val recorded = callComputationsA4(AgreementCallA(request))
  if (recorded.length() != 1) false else recorded.nth(0).request == request
    and recorded.nth(0).evaluation == computeTransaction(programS, request.before.state, request.input, Time1)
    and match recorded.nth(0).evaluation {
      | TransactionComputedA(raw) => raw.accepted and raw.payments == List()
          and recorded.nth(0).extraction == ExtractionObservedA4(EffectsExtractedA(List({
            source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10})))
      | _ => false }
})
run cancellationHasNoComputationTest = assert(
  callComputationsA4(CancellationCallA({before: initialBeforeI, now: Time2})) == List())
run diagnosticRemainsDiagnosticTest = assert({
  val bad = {...beforeS(0), state: {...beforeS(0).state,
    accounts: beforeS(0).state.accounts.put(aliceA, -1)}}
  val c = computeA4({before: bad, input: NoAInput, now: Time1})
  c.evaluation == OutsideModelDomainA and c.extraction == ExtractionNotApplicableA4 and not(computedA4(c))
})
run planComputationOrderTest = assert({
  val records = planComputationsA4(parentPlanI)
  records.length() == 2
    and records.nth(0).request.before == initialBeforeI
    and records.nth(1).request.before == residualBeforeI
    and records.foldl(true, (ok, c) => ok and computedA4(c))
})
type A4Prefix = {state: AAuthorityExecution, ok: bool, last: A4Event}
pure def executeStepsA4(c: A4Case, count: int): A4Prefix =
  c.steps.slice(0, count).foldl({state: initialA4(c), ok: true, last: startA4(c)}, (acc, step) => {
    val event = observeA4(acc.state, c, acc.last.sequence + 1, step)
    val after = if (Set("transition", "adversarial-derivation").contains(event.kind))
      applyA4(acc.state, c, event.arguments) else acc.state
    {state: after, ok: acc.ok and event.kind != "diagnostic", last: event}
  })
run inventoryBoundsTest = assert(INSTALLMENT_CASES_A4.length() == 32 and SWAP_CASES_A4.length() == 46
  and INSTALLMENT_CASES_A4.foldl(0, (n, c) => n + c.steps.length() + 2) == 638
  and SWAP_CASES_A4.foldl(0, (n, c) => n + c.steps.length() + 2) == 919)
run actualFirstCommitTest = assert({
  val c = SWAP_CASES_A4.nth(0)
  val prepared = executeStepsA4(c, 2)
  val committed = executeStepsA4(c, 5)
  prepared.ok and committed.ok and prepared.state.authority.context.candidate == beforeS(0)
    and committed.state.authority.context.candidate == beforeS(1)
    and committed.state.authority.context.ledger == ledgerS(1)
    and match committed.state.attempts.get(FundingOneAttempt) { | ExecutedOperation(_) => true | _ => false }
})
run sourceRoutesRetainedTest = assert(
  INSTALLMENT_CASES_A4.concat(SWAP_CASES_A4).foldl(true, (ok, c) => {
    val actual = executeStepsA4(c, c.steps.length())
    ok and actual.ok and endA4(actual.state, c, actual.last).kind == "case-end"
  }))
run dualMutationBoundariesTest = assert(
  INSTALLMENT_CASES_A4.concat(SWAP_CASES_A4).select(c =>
    Set("unused-successor", "reversed-effects", "reductions", "neutral-chooser").contains(c.descriptor.control))
  .foldl(true, (ok, c) => {
    val prefixCount = if (c.descriptor.lifecycle == "installment") 4 else 16
    val derived = executeStepsA4(c, prefixCount + 1)
    val denied = executeStepsA4(c, prefixCount + 2)
    val rejected = executeStepsA4(c, prefixCount + 3)
    val constructed = executeStepsA4(c, prefixCount + 4)
    val commitDenied = executeStepsA4(c, prefixCount + 5)
    val done = executeStepsA4(c, prefixCount + 6)
    val id = if (c.descriptor.lifecycle == "installment") FirstFillAttempt else DispositionAttempt
    ok and derived.ok and denied.ok and rejected.ok and constructed.ok and commitDenied.ok and done.ok
      and denied.state == derived.state and not(denied.last.observedGuard)
      and commitDenied.state == constructed.state and not(commitDenied.last.observedGuard)
      and rejected.state.authority == derived.state.authority and done.state.authority == constructed.state.authority
      and match rejected.state.attempts.get(id) {
        | RejectedOperation(r) => r.stage == VerificationBoundary and r.reason == UnauthorizedEffect
            and match done.state.attempts.get(id) {
              | RejectedOperation(final) => final.stage == CommitBoundary and final.reason == UnauthorizedEffect
                  and final.attempt == r.attempt and final.evidence == r.evidence
              | _ => false }
        | _ => false }
  }))
run deniedReplayPreservesAllFieldsTest = assert({
  val c = SWAP_CASES_A4.nth(0)
  val done = executeStepsA4(c, 18)
  val replayed = executeStepsA4(c, c.steps.length())
  done.ok and replayed.ok and replayed.state == done.state
})
run firstCaseEndsIndependentlyTest = I::init.then(16.reps(_ => I::step))
  .expect(I::latestEvent.kind == "case-end" and I::caseIndex == 0 and I::cursor == 16)
run initialEventsAreNotCompletionsTest = S::init.expect(S::latestEvent.kind == "case-start"
  and not(S::completeA4) and S::authorityState == unsignedS)
run nativeCase032InitialOrdinalTest = S::init.expect(S::caseIndex == 32
  and S::cursor == 0 and S::authorityState == unsignedS
  and S::latestEvent.kind == "case-start" and S::latestEvent.sequence == 0
  and S::latestEvent.caseId == SWAP_CASES_A4.nth(0).descriptor.caseId
  and S::latestEvent.profile == SWAP_CASES_A4.nth(0).descriptor.profile)

}
