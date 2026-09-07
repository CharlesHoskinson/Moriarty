module candidate_a_authority_adapter_harness {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import consumption.* from "./consumption"
  import policies.* from "./policies"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"

  pure val harnessPlan: AResolvedPlan = {identity: InstallmentPlanA, operations: List()}
  pure val harnessParent = {domain: InstallmentDomain, principal: Alice, nonce: 0}
  pure val harnessRequest: AAuthorityRequest = {before: {program: installmentProgram,
    state: {...emptyAState(N4), accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10),
      minimumTime: Time2}}, input: PresentAInput(ChoiceInputA({id: FirstFillId,
        chooser: Bob, chosen: 1})), now: Time2}
  pure val harnessOperation = OpFillSlot({parent: harnessParent, slot: 1})

  var adapted: bool
  var observation: AAuthorityAdaptation
  action init: bool = all { adapted' = false, observation' = AuthorityInvalidRequestA }
  action adapt: bool = all { not(adapted), adapted' = true,
    observation' = adaptAuthorityA(harnessRequest, harnessOperation, harnessPlan, PublicDisplay) }
  action step: bool = adapt
  val adapterProducedFirstFill: bool = adapted and match observation {
    | AuthorityAdaptedA(obs) => obs.input == ChoiceLike({id: "fill1", chooser: Bob, chosen: 1})
      and obs.proposedSuccessor.state.continuation == N2
      and obs.effects == List({source: Escrow(aliceA), destination: Wallet(Bob), asset: TokenA, quantity: 5})
    | _ => false
  }
  val adapterBindingSafety: bool = not(adapted) or match observation {
    | AuthorityAdaptedA(obs) => authorityObservationMatchesA(harnessOperation, obs)
    | _ => false
  }
}
