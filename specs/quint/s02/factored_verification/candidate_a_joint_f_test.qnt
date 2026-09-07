module candidate_a_joint_f_test {
  import effects.* from "../effects"
  import observations.* from "../observations"
  import execution.* from "../execution"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core.* from "../candidate_a_core"
  import candidate_a_projection.* from "../candidate_a_projection"
  import candidate_a_cases as C from "../candidate_a_cases"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_swap_fixtures as S from "../candidate_a_authority_swap_fixtures"
  import candidate_a_joint_f.* from "./candidate_a_joint_f"

  pure def equalityF(r: C::DiagnosticRequest): bool = {
    val actual = computeTransaction(r.program, r.before, r.input, r.now)
    val changed = computeJointF(r.program, r.before, r.input, r.now)
    actual == changed.evaluation and match actual {
      | TransactionComputedA(raw) => changed.extraction == extractCommittedEffects(r.program, r.before, r.input, r.now, raw)
          and extractCommittedEffectsF(r.program, r.before, r.input, r.now, raw) == changed.extraction
      | OutsideModelDomainA => changed.extraction == EffectOutsideDomainA
      | TransactionReductionBoundFailureA => changed.extraction == EffectReductionBoundFailureA
      | TransactionInputPreconditionFailureA => changed.extraction == EffectInputPreconditionFailureA
    }
  }
  run completeCorpusTest = assert(C::CASE_REQUESTS.indices().forall(i => equalityF(C::CASE_REQUESTS.nth(i).request)))
  run suppliedResultRejectedTest = assert(C::CASE_REQUESTS.indices().forall(i => {
    val r = C::CASE_REQUESTS.nth(i).request
    match computeTransaction(r.program, r.before, r.input, r.now) {
      | TransactionComputedA(raw) => Set(
          {...raw, accepted: not(raw.accepted)},
          {...raw, reductions: raw.reductions + 1},
          {...raw, state: {...raw.state, minimumTime: if (raw.state.minimumTime == Time100) Time101 else Time100}},
          {...raw, error: CoreErrorCode("forged")},
          {...raw, payments: raw.payments.append({source: aliceA, recipient: Mallory, asset: TokenA, quantity: 1})}
        ).forall(bad => extractCommittedEffectsF(r.program, r.before, r.input, r.now, bad) == EffectResultMismatchA)
      | _ => false
    }
  }))
  run orderedEffectsLiteralTest = {
    val r: C::DiagnosticRequest = {program: C::preDepositFixture(1), before: C::funded(N3, 1),
      input: C::depositInput(5), now: Time2}
    val result = computeJointF(r.program, r.before, r.input, r.now)
    assert(result.extraction == EffectsExtractedA(List(
      {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 1},
      {source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 5},
      {source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})))
  }
  run malformedBeforeMismatchTest = {
    val r: C::DiagnosticRequest = {program: S::programS,
      before: {...S::beforeS(0).state, accounts: Map(), choices: Map()}, input: C::depositInput(10), now: Time1}
    val fake: ATransactionResult = {accepted: true, state: S::beforeS(1).state, error: NoCoreError,
      payments: List(), warnings: List(), reductions: 0}
    assert(equalityF(r) and extractCommittedEffectsF(r.program, r.before, r.input, r.now, fake) == EffectOutsideDomainA)
  }
  run timeoutRollbackLiteralTest = assert(Set(Time100, Time101).forall(t => {
    val before = S::beforeS(2)
    val r: AAuthorityRequest = {before: before, input: C::choiceInput(Bob, 1), now: t}
    val joint = computeJointF(before.program, before.state, r.input, t)
    joint == {evaluation: TransactionComputedA({accepted: false, state: before.state,
      error: CoreErrorCode("contract_closed"), payments: List(), warnings: List(), reductions: 0}),
      extraction: EffectsExtractedA(List())}
  }))
  run authorityCorpusTest = assert(S::SCENARIOS_S.forall(s =>
    Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt).forall(id => {
      val op = S::plannedS(id, s)
      match op.artifactAndCall {
        | AgreementCallA(r) => adaptAuthorityF(r, op.operation, S::planS(id,s), PublicDisplay)
            == adaptAuthorityA(r, op.operation, S::planS(id,s), PublicDisplay)
        | _ => false
      }
    })))
}
