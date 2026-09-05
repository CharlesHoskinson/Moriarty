module effects_machine {
  import effects.* from "./effects"
  const INITIAL_A: int
  const INITIAL_B: int
  type HarnessState = {ledger: Ledger, phase: int, history: List[Transfer]}
  var state: HarnessState
  pure val depositEffect: Transfer = {source: Wallet(Alice), destination: Escrow({owner: Alice, asset: TokenA}), asset: TokenA, quantity: 10}
  pure val refundEffect: Transfer = {source: Escrow({owner: Alice, asset: TokenA}), destination: Wallet(Alice), asset: TokenA, quantity: 10}
  pure def canDeposit(s: HarnessState): bool = s.phase == 0 and canApply(s.ledger, List(depositEffect))
  pure def canRefund(s: HarnessState): bool = s.phase == 1 and canApply(s.ledger, List(refundEffect))
  action init = state' = {ledger: initialLedger(INITIAL_A, INITIAL_B), phase: 0, history: List()}
  action deposit = all {
    canDeposit(state),
    state' = {ledger: applyTransfers(state.ledger, List(depositEffect)), phase: 1, history: state.history.append(depositEffect)},
  }
  action refund = all {
    canRefund(state),
    state' = {ledger: applyTransfers(state.ledger, List(refundEffect)), phase: 2, history: state.history.append(refundEffect)},
  }
  action step = any {deposit, refund}
  val deposited = state.phase == 1
  val refunded = state.phase == 2
  val safety = validLedger(state.ledger) and
    totalAsset(state.ledger, TokenA) == INITIAL_A and totalAsset(state.ledger, TokenB) == INITIAL_B and
    (state.phase == 2 or canDeposit(state) or canRefund(state))
}

module effects_harness {
  import effects_machine(INITIAL_A = 10, INITIAL_B = 20).*
}
