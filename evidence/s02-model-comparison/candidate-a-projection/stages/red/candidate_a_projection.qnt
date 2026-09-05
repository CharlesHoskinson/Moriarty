module candidate_a_projection {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_core.* from "./candidate_a_core"

  // Carrier translation is deliberately non-validating. It neither establishes
  // agreement semantics nor produces evidence/authorization dispositions.
  pure def projectState(program: AProgram, state: AState): CoreStateObservation[AContinuation] = {
    accounts: Map(), choices: Map(), continuation: {program: program, node: N0}, minimumTime: 0,
  }
  pure def projectInput(input: AInput): NeutralInput = NoInput
  pure def projectResult(program: AProgram, result: ATransactionResult): CoreResultObservation[AContinuation] = {
    accepted: false, error: NoCoreError, warnings: List(), payments: List(),
    state: projectState(program, result.state), reductions: 0,
  }

  // Diagnostics are not Core errors and must not become successful extraction.
  type AEffectExtraction = EffectsExtractedA(List[Transfer]) | EffectResultMismatchA
    | EffectOutsideDomainA | EffectReductionBoundFailureA | EffectInputPreconditionFailureA

  pure def extractCommittedEffects(program: AProgram, before: AState, input: AInput,
                                   now: ATime, result: ATransactionResult): AEffectExtraction =
    EffectOutsideDomainA
}
