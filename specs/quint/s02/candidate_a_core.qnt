module candidate_a_core {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"

  // Internal precondition: admitted observation and total state choice map.
  // Public reduction/input entry points validate their finite-domain inputs.
  pure def evaluateObservation(observation: AObservation, state: AState): bool = match observation {
    | ChoiceEqualsA(choice) => state.choices.get(choice.id) == IntValue(choice.expected)
  }

  // This gate means the one-step evaluator is implemented for this valid state;
  // an empty Close is included and yields QuiescentA, not a counted reduction.
  pure def canReduceOnce(program: AProgram, state: AState): bool = validState(program, state)

  // All frozen reduction clauses (core.py:228-272), in their source order.
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
      | IfA(branch) => ReducedA({state: {...state, continuation:
          if (evaluateObservation(branch.observation, state)) branch.thenNode else branch.elseNode},
          payments: List(), warnings: List(), reductions: 1})
      | WhenA(waiting) => if (timeValue(state.minimumTime) >= timeValue(waiting.timeout))
          ReducedA({state: {...state, continuation: waiting.timeoutNode},
            payments: List(), warnings: List(), reductions: 1})
        else QuiescentA(state)
    }

  type ReductionAccumulatorA = {done: bool, outside: bool, result: AReduction}
  // At most 15 descending non-Close nodes plus six Close refunds; call 22
  // observes quiescence. Completed accumulators are retained, not recounted.
  pure def reduceToQuiescence(program: AProgram, state: AState): AReductionEvaluation =
    if (not(validState(program, state))) QuiescenceOutsideDomainA
    else {
      val initial: ReductionAccumulatorA = {done: false, outside: false,
        result: {state: state, payments: List(), warnings: List(), reductions: 0}}
      val accumulated = List(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21).foldl(initial, (acc, _) =>
          if (acc.done or acc.outside) acc
          else match reduceOnce(program, acc.result.state) {
            | QuiescentA(_) => {...acc, done: true}
            | ReductionOutsideDomainA => {...acc, outside: true}
            | ReducedA(delta) => {...acc, result: {state: delta.state,
                payments: delta.payments.foldl(acc.result.payments, (items, payment) => items.append(payment)),
                warnings: delta.warnings.foldl(acc.result.warnings, (items, warning) => items.append(warning)),
                reductions: acc.result.reductions + delta.reductions}}
          })
      if (accumulated.outside) QuiescenceOutsideDomainA
      else if (accumulated.done) ReductionCompleteA(accumulated.result)
      else ReductionBoundFailureA
    }

  // Restricted internal modeling API: supplied input on a valid unexpired
  // When after quiescence. Task 4 must perform timeout-priority reduction first
  // and handle NoAInput separately. This is not a public transaction result.
  pure def canApplyInput(program: AProgram, state: AState, supplied: ASuppliedInput): bool =
    if (validState(program, state) and validInput(PresentAInput(supplied)))
      match program.nodes.get(state.continuation) {
        | WhenA(waiting) => timeValue(state.minimumTime) < timeValue(waiting.timeout)
        | _ => false
      }
    else false

  type InputScanA = {done: bool, boundsMismatch: bool, result: AInputResult}
  pure def applyInput(program: AProgram, state: AState, supplied: ASuppliedInput): AInputEvaluation =
    if (not(validState(program, state)) or not(validInput(PresentAInput(supplied)))) InputOutsideDomainA
    else if (not(canApplyInput(program, state, supplied))) InputPreconditionFailureA
    else match program.nodes.get(state.continuation) {
      | WhenA(waiting) => {
          val initial: InputScanA = {done: false, boundsMismatch: false,
            result: {state: state, error: CoreErrorCode("no_matching_input")}}
          val scanned = waiting.cases.foldl(initial, (acc, oneCase) =>
            if (acc.done) acc
            else match oneCase.caseAction {
              | DepositA(deposit) => match supplied {
                  | DepositInputA(input) => {
                      val required = match deposit.amount { | ConstantA(quantity) => quantity }
                      if (deposit.account != input.account or deposit.depositor != input.depositor
                          or required != input.quantity) acc
                      else if (input.quantity <= 0) {...acc, done: true,
                        result: {state: state, error: CoreErrorCode("non_positive_deposit")}}
                      else {...acc, done: true, result: {state: {...state,
                        continuation: oneCase.continuation,
                        accounts: state.accounts.put(deposit.account, state.accounts.get(deposit.account) + input.quantity)},
                        error: NoCoreError}}
                    }
                  | _ => acc
                }
              | ChoiceA(choice) => match supplied {
                  | ChoiceInputA(input) =>
                      if (choice.id != input.id or choice.chooser != input.chooser) acc
                      else if (input.chosen < choice.lower or input.chosen > choice.upper)
                        {...acc, boundsMismatch: true}
                      else {...acc, done: true, result: {state: {...state,
                        continuation: oneCase.continuation, choices: state.choices.put(choice.id, IntValue(input.chosen))},
                        error: NoCoreError}}
                  | _ => acc
                }
            })
          InputAppliedA(if (not(scanned.done) and scanned.boundsMismatch)
            {state: state, error: CoreErrorCode("choice_out_of_bounds")} else scanned.result)
        }
      | _ => InputPreconditionFailureA
    }

  // Internal result constructors: callers supply only actual frozen Core errors
  // or successful reductions. Diagnostics never pass through these helpers.
  pure def rolledBackTransaction(original: AState, error: CoreError): ATransactionEvaluation =
    TransactionComputedA({accepted: false, state: original, error: error,
      payments: List(), warnings: List(), reductions: 0})
  pure def acceptedTransaction(reduced: AReduction): ATransactionEvaluation =
    TransactionComputedA({accepted: true, state: reduced.state, error: NoCoreError,
      payments: reduced.payments, warnings: reduced.warnings, reductions: reduced.reductions})

  // Frozen core.py:323-370 source order, with finite-model diagnostics outside
  // the Python result vocabulary. Every Core rejection uses the ORIGINAL state.
  pure def computeTransaction(program: AProgram, before: AState, supplied: AInput,
                             now: ATime): ATransactionEvaluation =
    if (not(validState(program, before)) or not(validInput(supplied))) OutsideModelDomainA
    else if (timeValue(now) < timeValue(before.minimumTime))
      rolledBackTransaction(before, CoreErrorCode("time_before_state"))
    else match reduceToQuiescence(program, {...before, minimumTime: now}) {
      | QuiescenceOutsideDomainA => OutsideModelDomainA
      | ReductionBoundFailureA => TransactionReductionBoundFailureA
      | ReductionCompleteA(pre) => {
          val currentNode = program.nodes.get(pre.state.continuation)
          match supplied {
            | NoAInput => match currentNode {
                | CloseA => if (pre.reductions == 0 and pre.payments == List() and pre.warnings == List())
                    rolledBackTransaction(before, CoreErrorCode("contract_closed"))
                  else acceptedTransaction(pre)
                | WhenA(_) => rolledBackTransaction(before, CoreErrorCode("input_required"))
                | _ => acceptedTransaction(pre)
              }
            | PresentAInput(input) => match currentNode {
                | WhenA(_) => match applyInput(program, pre.state, input) {
                    | InputOutsideDomainA => OutsideModelDomainA
                    | InputPreconditionFailureA => TransactionInputPreconditionFailureA
                    | InputAppliedA(applied) => if (applied.error != NoCoreError)
                        rolledBackTransaction(before, applied.error)
                      else match reduceToQuiescence(program, applied.state) {
                        | QuiescenceOutsideDomainA => OutsideModelDomainA
                        | ReductionBoundFailureA => TransactionReductionBoundFailureA
                        | ReductionCompleteA(post) => acceptedTransaction({state: post.state,
                            payments: post.payments.foldl(pre.payments, (items, payment) => items.append(payment)),
                            warnings: post.warnings.foldl(pre.warnings, (items, warning) => items.append(warning)),
                            reductions: pre.reductions + post.reductions})
                      }
                  }
                | CloseA => rolledBackTransaction(before, CoreErrorCode("contract_closed"))
                | _ => rolledBackTransaction(before, CoreErrorCode("no_matching_input"))
              }
          }
        }
    }
}
