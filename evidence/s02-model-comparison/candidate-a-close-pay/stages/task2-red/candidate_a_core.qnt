module candidate_a_core {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"

  // Task 2 typed RED scaffolding. No agreement reductions are implemented yet.
  pure def canReduceOnce(program: AProgram, state: AState): bool = false
  pure def reduceOnce(program: AProgram, state: AState): AReductionStep = ReductionUnavailableA
}
