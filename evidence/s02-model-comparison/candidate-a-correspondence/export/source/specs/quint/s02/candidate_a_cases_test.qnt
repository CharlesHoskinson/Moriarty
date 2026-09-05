module candidate_a_cases_test {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_cases.* from "./candidate_a_cases"

  pure def hasError(cases: List[DiagnosticCase], code: str): bool = cases.foldl(false, (found, c) =>
    found or match c.payload { | CaseComputedA(p) => not(p.raw.accepted) and p.raw.error == CoreErrorCode(code) | _ => false })
  pure def hasWarning(cases: List[DiagnosticCase], warning: CoreWarning): bool = cases.foldl(false, (found, c) =>
    found or match c.payload { | CaseComputedA(p) => p.raw.warnings.select(w => w == warning).length() > 0 | _ => false })
  pure def pays(cases: List[DiagnosticCase], id: str, who: Principal): bool = cases.foldl(false, (found, c) =>
    found or (c.case_id == id and match c.payload { | CaseComputedA(p) => p.raw.accepted and p.raw.payments ==
      List({source: aliceA, recipient: who, asset: TokenA, quantity: 5}) | _ => false }))
  run literalDomainTest = assert(CASE_REQUESTS.length() == 24
    and CASE_REQUESTS.foldl(Set(), (ids, c) => ids.union(Set(c.case_id))).size() == 24
    and CASE_REQUESTS.foldl(true, (ok, c) => ok and validState(c.request.program, c.request.before) and validInput(c.request.input)))
  run corpusComputesEveryRequestTest = init.expect(corpusComplete)
  run allSixErrorsTest = init.expect(Set("time_before_state", "contract_closed", "input_required",
    "no_matching_input", "choice_out_of_bounds", "non_positive_deposit").forall(code => hasError(diagnosticCases, code)))
  run warningPayloadsTest = init.expect(
    hasWarning(diagnosticCases, {code: "non_positive_payment", requested: IntValue(0), paid: IntValue(0)})
      and hasWarning(diagnosticCases, {code: "non_positive_payment", requested: IntValue(-1), paid: IntValue(0)})
      and hasWarning(diagnosticCases, {code: "partial_payment", requested: IntValue(10), paid: IntValue(4)})
      and hasWarning(diagnosticCases, {code: "partial_payment", requested: IntValue(10), paid: IntValue(0)}))
  run absentAndZeroGenuineBranchesTest = init.expect(pays(diagnosticCases, "absent-choice", Alice)
    and pays(diagnosticCases, "stored-zero-choice", Bob))
  run firstAcceptingOrderTest = init.expect(pays(diagnosticCases, "ordered-first", Bob)
    and pays(diagnosticCases, "ordered-fallthrough", Mallory) and pays(diagnosticCases, "ordered-overlap", Bob))
  run preDepositPostOrderingTest = init.expect(diagnosticCases.foldl(false, (found, c) => found or
    (c.case_id == "pre-deposit-post" and match c.payload {
      | CaseComputedA(p) => p.effects == List(
          {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 1},
          {source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 5},
          {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})
      | _ => false
    })))
  run actualRollbackPreservationTest = init.expect(corpusComplete and diagnosticCases.foldl(true, (ok, c) => ok and
    match c.payload {
      | CaseComputedA(p) => p.raw.accepted or (p.raw.state == c.request.before and p.effects == List()
          and p.raw.warnings == List() and p.raw.payments == List() and p.raw.reductions == 0)
      | _ => false
    }))
}
