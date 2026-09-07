module candidate_a_projection {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_core.* from "./candidate_a_core"

  // Carrier translation is deliberately non-validating. It neither establishes
  // agreement semantics nor produces evidence/authorization dispositions.
  // Fold existing choice keys, so a malformed partial map stays partial rather
  // than triggering an undefined lookup or silently inventing absent entries.
  pure def projectState(program: AProgram, state: AState): CoreStateObservation[AContinuation] = {
    accounts: state.accounts,
    choices: state.choices.keys().fold(Map(), (mapped, id) =>
      mapped.put(choiceName(id), state.choices.get(id))),
    continuation: {program: program, node: state.continuation}, minimumTime: timeValue(state.minimumTime),
  }
  pure def projectInput(input: AInput): NeutralInput = match input {
    | NoAInput => NoInput
    | PresentAInput(supplied) => match supplied {
        | DepositInputA(deposit) => DepositLike({location: Escrow(deposit.account),
            depositor: deposit.depositor, asset: deposit.account.asset, quantity: deposit.quantity})
        | ChoiceInputA(choice) => ChoiceLike({id: choiceName(choice.id),
            chooser: choice.chooser, chosen: choice.chosen})
      }
  }
  pure def projectResult(program: AProgram, result: ATransactionResult): CoreResultObservation[AContinuation] = {
    accepted: result.accepted, error: result.error, warnings: result.warnings, payments: result.payments,
    state: projectState(program, result.state), reductions: result.reductions,
  }

  // Diagnostics are not Core errors and must not become successful extraction.
  type AEffectExtraction = EffectsExtractedA(List[Transfer]) | EffectResultMismatchA
    | EffectOutsideDomainA | EffectReductionBoundFailureA | EffectInputPreconditionFailureA

  // Only this extraction entry point evaluates agreement semantics. Require the
  // complete provided result to equal actual execution, even for rejection.
  // Lifecycle-only cancellation does not call this function: it has no Core
  // transaction/result and must retain NoCoreProjection in its own adapter.
  pure def extractCommittedEffects(program: AProgram, before: AState, input: AInput,
                                   now: ATime, result: ATransactionResult): AEffectExtraction =
    match computeTransaction(program, before, input, now) {
      | OutsideModelDomainA => EffectOutsideDomainA
      | TransactionReductionBoundFailureA => EffectReductionBoundFailureA
      | TransactionInputPreconditionFailureA => EffectInputPreconditionFailureA
      | TransactionComputedA(actual) =>
          if (result != actual) EffectResultMismatchA
          else if (not(actual.accepted)) EffectsExtractedA(List())
          else match reduceToQuiescence(program, {...before, minimumTime: now}) {
            | QuiescenceOutsideDomainA => EffectOutsideDomainA
            | ReductionBoundFailureA => EffectReductionBoundFailureA
            | ReductionCompleteA(pre) => {
                // Accepted computeTransaction retained this exact pre-payment
                // prefix. The original request supplies the deposit insertion
                // point which the final payment list alone cannot recover.
                val prefix = pre.payments.foldl(List(), (items, payment) => items.append(paymentTransfer(payment)))
                val withDeposit = match input {
                  | NoAInput => prefix
                  | PresentAInput(supplied) => match supplied {
                      | ChoiceInputA(_) => prefix
                      | DepositInputA(deposit) => prefix.append({source: Wallet(deposit.depositor),
                          destination: Escrow(deposit.account), asset: deposit.account.asset,
                          quantity: deposit.quantity})
                    }
                }
                val suffix = actual.payments.slice(pre.payments.length(), actual.payments.length())
                EffectsExtractedA(suffix.foldl(withDeposit, (items, payment) => items.append(paymentTransfer(payment))))
              }
          }
    }
}
