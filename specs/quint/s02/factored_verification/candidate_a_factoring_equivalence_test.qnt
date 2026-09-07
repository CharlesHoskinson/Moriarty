module candidate_a_factoring_equivalence_test {
  import candidate_a_harness as HS from "../candidate_a_harness"
  import candidate_a_harness_f as HSF from "./candidate_a_harness_f"
  import candidate_a_installment_harness as HI from "../candidate_a_installment_harness"
  import candidate_a_installment_harness_f as HIF from "./candidate_a_installment_harness_f"
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_boundary as B from "../candidate_a_authority_boundary"
  import candidate_a_authority_boundary_f as BF from "./candidate_a_authority_boundary_f"
  import candidate_a_authority_installment_fixtures as I0 from "../candidate_a_authority_installment_fixtures"
  import candidate_a_authority_installment as I from "../candidate_a_authority_installment"
  import candidate_a_authority_installment_f as IF from "./candidate_a_authority_installment_f"
  import candidate_a_authority_swap_fixtures as S0 from "../candidate_a_authority_swap_fixtures"
  import candidate_a_authority_swap as S from "../candidate_a_authority_swap"
  import candidate_a_authority_swap_f as SF from "./candidate_a_authority_swap_f"

  pure val swapRequests = Set({input: HS::aliceFunding, now: Time1},
    {input: HS::bobFunding, now: Time2}, {input: HS::settleInput, now: Time2},
    {input: HS::refundInput, now: Time2}, {input: NoAInput, now: Time100},
    {input: HS::settleInput, now: Time100})
  pure val installmentRequests = Set({input: HI::fillOneInput, now: Time2},
    {input: HI::fillTwoInput, now: Time2}, {input: HI::recoveryInput, now: Time2},
    {input: NoAInput, now: Time100}, {input: HI::recoveryInput, now: Time100})
  pure val allSwapPrefixes = List(0,1,2,3).foldl(Set(HS::swapInitial), (states, _) =>
    states.union(states.map(s => swapRequests.filter(r => HS::canRequest(s,r.input,r.now))
      .map(r => HS::applyRequest(s,r.input,r.now))).flatten()))
  pure val allInstallmentPrefixes = List(0,1,2).foldl(Set(HI::installmentInitial), (states, _) =>
    states.union(states.map(s => installmentRequests.filter(r => HI::canRequest(s,r.input,r.now))
      .map(r => HI::applyRequest(s,r.input,r.now))).flatten()))
  run completeSwapTraceCorpusTest = assert(allSwapPrefixes.forall(s =>
    HS::traceSafety(s) and HS::traceSafety(s) == HSF::traceSafety(s)
      and HS::traceTerminal(s) == HSF::traceTerminal(s)
      and HS::traceEnabled(s) == HSF::traceEnabled(s)
      and swapRequests.forall(r =>
        HS::canRequest(s,r.input,r.now) == HSF::canRequest(s,r.input,r.now)
          and (if (HS::canRequest(s,r.input,r.now))
            HS::applyRequest(s,r.input,r.now) == HSF::applyRequest(s,r.input,r.now) else true))))
  run completeInstallmentTraceCorpusTest = assert(allInstallmentPrefixes.forall(s =>
    HI::traceSafety(s) and HI::traceSafety(s) == HIF::traceSafety(s)
      and HI::traceTerminal(s) == HIF::traceTerminal(s)
      and HI::traceEnabled(s) == HIF::traceEnabled(s)
      and installmentRequests.forall(r =>
        HI::canRequest(s,r.input,r.now) == HIF::canRequest(s,r.input,r.now)
          and (if (HI::canRequest(s,r.input,r.now))
            HI::applyRequest(s,r.input,r.now) == HIF::applyRequest(s,r.input,r.now) else true))))

  run installmentPrefixesEqualTest = assert(I0::PROFILES_I.forall(p => I0::SCENARIOS_I.forall(s =>
    0.to(I::routeI(s).length()).forall(n => {
      val original = I::runPrefixI(s,p,n)
      val factored = IF::runPrefixI(s,p,n)
      original == factored and original.ok
        and I::safetyI(original.state) == IF::safetyI(original.state)
        and (if (n == I::routeI(s).length()) true else {
          val cmd = I::routeI(s).nth(n)
          I::canCommandI(original.state,s,p,cmd) == IF::canCommandI(original.state,s,p,cmd)
            and I::applyCommandI(original.state,s,p,cmd) == IF::applyCommandI(original.state,s,p,cmd)
        })
    }))))
  run swapPrefixesEqualTest = assert(S0::PROFILES_S.forall(p => S0::SCENARIOS_S.forall(s =>
    0.to(S::routeS(s).length()).forall(n => {
      val original = S::prefixS(s,p,n)
      val factored = SF::prefixS(s,p,n)
      original == factored and original.ok
        and S::financialTerminalS(original.state) == SF::financialTerminalS(original.state)
        and (if (n == S::routeS(s).length()) true else {
          val cmd = S::routeS(s).nth(n)
          S::canCommandS(original.state,s,p,cmd) == SF::canCommandS(original.state,s,p,cmd)
            and S::applyCommandS(original.state,s,p,cmd) == SF::applyCommandS(original.state,s,p,cmd)
        })
    }))))
  run stalePrefixesEqualTest = assert(S0::PROFILES_S.forall(p =>
    0.to(S::staleRouteS.length()).forall(n =>
      S::executeListS(S::staleScenarioS,p,S::staleRouteS.slice(0,n))
        == SF::executeListS(S::staleScenarioS,p,S::staleRouteS.slice(0,n)))))
  run reboundForgeryEqualTest = assert(S0::PROFILES_S.forall(p => {
    val prefix = S::prefixS(S::staleScenarioS,p,3)
    prefix.ok and match prefix.state.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => {
          val program = {...a.observation.proposedSuccessor.program,
            nodes: a.observation.proposedSuccessor.program.nodes.put(N15,
              PayA({account: aliceA, payee: Mallory, amount: ConstantA(1), continuation: N0}))}
          val changed = {...a, observation: {...a.observation,
            proposedSuccessor: {...a.observation.proposedSuccessor, program: program}}}
          val bad = {...prefix.state, attempts: prefix.state.attempts.put(FundingOneAttempt, ProposedAttempt(changed))}
          val evidence = S::evidenceS(changed,S::staleScenarioS,p)
          val retained = {...bad, attempts: bad.attempts.put(FundingOneAttempt,
            RejectedOperation({attempt: changed, evidence: evidence, observedContext: bad.authority.context,
              reason: UnauthorizedEffect, stage: VerificationBoundary}))}
          not(B::canVerifyAuthorityA(bad,FundingOneAttempt,evidence))
            and not(BF::canVerifyAuthorityA(bad,FundingOneAttempt,evidence))
            and B::applyRejectProposedAuthorityA(bad,FundingOneAttempt,evidence) == retained
            and BF::applyRejectProposedAuthorityA(bad,FundingOneAttempt,evidence) == retained
        }
      | _ => false
    }
  }))
}
