module candidate_a_authority_swap_test {
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
  import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"

  run literalFidelityTest = assert(programS == canonicalSwap and beforeS(0).state.minimumTime == Time0
    and unsignedS.authority.context.environment.physicalTime == 1 and SCENARIOS_S.forall(s =>
      Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt).forall(id =>
        actualS(id, s) == AuthorityAdaptedA(expectedS(id, s))
        and authorityPlanMatchesA(planS(id, s))
        and requiredKeys(plannedS(id, s).operation, plannedS(id, s).effects) == expectedKeysS(id, s))))
  run authorityTimeZeroTest = assert(adaptAuthorityA({before: beforeS(0), input: NoAInput, now: Time0},
    OpDeadlineRefund, planS(DispositionAttempt, {funded: 0, mode: TimeoutS(Time100)}), PublicDisplay) == AuthorityInvalidRequestA
    and validState(programS, beforeS(0).state))

}
