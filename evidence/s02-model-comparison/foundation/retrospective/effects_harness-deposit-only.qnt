module effects_machine {
  import effects.* from "./effects"
  const INITIAL_A: int
  const INITIAL_B: int
  type HarnessState = {ledger: Ledger, phase: int, history: List[Transfer]}
  var state: HarnessState
  pure val depositEffect: Transfer = {source: Wallet(Alice), destination: Escrow({owner: Alice, asset: TokenA}), asset: TokenA, quantity: 10}
  pure def canDeposit(s: HarnessState): bool = s.phase == 0 and canApply(s.ledger, List(depositEffect))
  action init = state' = {ledger: initialLedger(INITIAL_A, INITIAL_B), phase: 0, history: List()}
  action deposit = all {
    canDeposit(state),
    state' = {ledger: applyTransfers(state.ledger, List(depositEffect)), phase: 1, history: state.history.append(depositEffect)},
  }
  action step = deposit
  val deposited = state.phase == 1
}

module effects_harness {
  import effects_machine(INITIAL_A = 10, INITIAL_B = 20).*
}
