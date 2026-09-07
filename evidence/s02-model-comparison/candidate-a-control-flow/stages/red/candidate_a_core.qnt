module candidate_a_core {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"

  // Task 3 typed RED scaffolding. Observation callers must supply admitted
  // observation/state maps; public reduction/input entry points validate them.
  pure def evaluateObservation(observation: AObservation, state: AState): bool = false
  pure def reduceToQuiescence(program: AProgram, state: AState): AReductionEvaluation = ReductionBoundFailureA
  pure def canApplyInput(program: AProgram, state: AState, supplied: ASuppliedInput): bool = false
  pure def applyInput(program: AProgram, state: AState, supplied: ASuppliedInput): AInputEvaluation =
    InputPreconditionFailureA

  // This gate means the one-step evaluator is implemented for this valid state;
  // an empty Close is included and yields QuiescentA, not a counted reduction.
  pure def canReduceOnce(program: AProgram, state: AState): bool =
    if (validState(program, state)) match program.nodes.get(state.continuation) {
      | CloseA => true | PayA(_) => true | IfA(_) => false | WhenA(_) => false
    } else false

  // Only the frozen Close/Pay clauses (core.py:228-253) are implemented here.
  // If/When remain explicitly unavailable until Task 3, never false quiescence.
  // Bad finite-domain calls are diagnostics, not Python Core errors.
  pure def reduceOnce(program: AProgram, state: AState): AReductionStep =
    if (not(validState(program, state))) ReductionOutsideDomainA
    else match program.nodes.get(state.continuation) {
      | CloseA => {
          val positiveAccounts = CANONICAL_ACCOUNTS.select(account => state.accounts.get(account) > 0)
          if (positiveAccounts.length() == 0) QuiescentA(state)
          else {
            val account = positiveAccounts.head()
            ReducedA({state: {...state, accounts: state.accounts.put(account, 0)},
              payments: List({source: account, recipient: account.owner, asset: account.asset,
                quantity: state.accounts.get(account)}), warnings: List(), reductions: 1})
          }
        }
      | PayA(pay) => {
          val requested = match pay.amount { | ConstantA(quantity) => quantity }
          val balance = state.accounts.get(pay.account)
          val nonnegative = if (requested > 0) requested else 0
          val paid = if (nonnegative < balance) nonnegative else balance
          val warnings: List[CoreWarning] = if (requested <= 0)
            List({code: "non_positive_payment", requested: IntValue(requested), paid: IntValue(0)})
            else if (paid < requested)
              List({code: "partial_payment", requested: IntValue(requested), paid: IntValue(paid)})
            else List()
          val payments: List[CorePayment] = if (paid > 0)
            List({source: pay.account, recipient: pay.payee, asset: pay.account.asset, quantity: paid})
            else List()
          val nextAccounts = if (paid > 0) state.accounts.put(pay.account, balance - paid) else state.accounts
          ReducedA({state: {...state, continuation: pay.continuation, accounts: nextAccounts},
            payments: payments, warnings: warnings, reductions: 1})
        }
      | IfA(_) => ReductionUnavailableA
      | WhenA(_) => ReductionUnavailableA
    }
}
