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
}
