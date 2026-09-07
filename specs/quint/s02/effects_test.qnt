module effects_test {
  import effects_machine(INITIAL_A = 10, INITIAL_B = 20).* from "./effects_harness"
  import effects.* from "./effects"

  run depositTest = init.then(deposit).expect(
    balance(state.ledger, Wallet(Alice), TokenA) == 0 and
    balance(state.ledger, Escrow({owner: Alice, asset: TokenA}), TokenA) == 10)
  run refundTest = init.then(deposit).then(refund).expect(
    state.ledger == initialLedger(INITIAL_A, INITIAL_B) and safety)
  run missingLedgerTest = init.expect(not(canApply(Map(), List(depositEffect))))
  run nonpositiveTest = init.expect(
    not(validTransfer({...depositEffect, quantity: 0})) and
    not(canApply(state.ledger, List({...depositEffect, quantity: -1}))))
  run wrongAssetTest = init.expect(
    not(validTransfer({...depositEffect, asset: TokenB})))
  run duplicateEffectsTest = init.expect(
    not(policyAllows(List(), List(depositEffect), List(depositEffect, depositEffect))) and
    policyAllows(List(depositEffect, depositEffect), List(depositEffect, depositEffect), List(depositEffect, depositEffect)))
  run missingRequiredTest = init.expect(
    not(policyAllows(List(depositEffect), List(depositEffect), List())))
  run permutationTest = init.expect(
    policyAllows(List(depositEffect, refundEffect), List(depositEffect, refundEffect), List(refundEffect, depositEffect)))
  run temporaryOverdraftTest = {
    val out = {source: Wallet(Bob), destination: Wallet(Alice), asset: TokenA, quantity: 1}
    val back = {source: Wallet(Alice), destination: Wallet(Bob), asset: TokenA, quantity: 1}
    init.expect(
      applyTransfers(state.ledger, List(out, back)) == state.ledger and
      not(canApply(state.ledger, List(out, back))))
  }
  run extraEffectTest = {
    val extra = {source: Wallet(Alice), destination: Wallet(Mallory), asset: TokenA, quantity: 1}
    init.expect(not(policyAllows(List(depositEffect), List(depositEffect), List(depositEffect, extra))))
  }
}
