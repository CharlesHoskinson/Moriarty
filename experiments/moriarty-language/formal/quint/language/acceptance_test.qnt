module acceptance_test {
  import acceptance.* from "./acceptance"
  pure val good = validCandidate(0, 0)
  run validTransferTest = init.then(accept(good)).expect(state.ledger.balances.get(0).recipient == 3 and state.ledger.fees.get(0) == 1 and safety)
  run sponsoredFeesTest = init.then(accept({ ...good, sponsored: true, dust: 3 })).expect(state.ledger.sponsorRemaining == 3 and state.ledger.userDust == 10)
  run sameNonceTest = init.then(accept(good)).then(refuse(validCandidate(1, 0))).expect(state.accepts == 1 and state.rejects == 1 and state.ledger.head == 1)
  run stalePredecessorTest = init.then(accept(good)).then(refuse(validCandidate(0, 1))).expect(state.accepts == 1 and state.ledger.net.get(0) == 2)
  pure val badBindings = List(
    { ...binding, source: "wrong" }, { ...binding, program: "wrong" }, { ...binding, semantics: "wrong" },
    { ...binding, relation: "wrong" }, { ...binding, key: "wrong" }, { ...binding, ledger: "wrong" },
    { ...binding, network: "wrong" }, { ...binding, anchor: "wrong" }, { ...binding, address: "wrong" },
    { ...binding, circuitSet: "wrong" }, { ...binding, target: "wrong" }, { ...binding, actionId: "wrong" }
  )
  run wrongBindingsTest = init.then(12.reps(idx => refuse({ ...good, binding: badBindings.nth(idx) }))).expect(state.rejects == 12 and state.ledger == initialLedger)
  pure val badAssets = List(
    { ...binding.assets.get(0), network: "wrong" }, { ...binding.assets.get(0), issuer: "wrong" },
    { ...binding.assets.get(0), reference: "wrong" }, { ...binding.assets.get(0), scale: 1 }
  )
  run wrongAssetTest = init.then(4.reps(idx => refuse({ ...good, binding: { ...binding, assets: binding.assets.set(0, badAssets.nth(idx)) } }))).expect(state.rejects == 4 and state.ledger == initialLedger)
  run missingEffectsTest = init.then(refuse({ ...good, effects: Map(0 -> good.effects.get(0)) })).expect(state.ledger == initialLedger)
  run incompleteEffectsTest = init.then(refuse({ ...good, complete: false })).expect(state.ledger == initialLedger)
  run conservationTest = init.then(refuse({ ...good, effects: good.effects.set(0, { ...zeroEffect, debit: 4, credit: 5 }) })).expect(state.ledger == initialLedger)
  run insufficientBalanceTest = init.then(refuse({ ...good, effects: good.effects.set(0, { ...zeroEffect, debit: 21, credit: 21 }) })).expect(state.ledger == initialLedger)
  run grossRefundCapTest = {
    val refund = { ...good, effects: good.effects.set(0, { ...zeroEffect, debit: 10, refund: 9, credit: 1 }) }
    init.then(accept(refund)).then(refuse({ ...refund, predecessor: 1, successor: 2, nonce: 1 }))
      .expect(state.ledger.balances.get(0).owner == 19 and state.ledger.gross.get(0) == 10 and state.accepts == 1)
  }
  run cumulativeFeesTest = {
    val costly = { ...good, effects: good.effects.set(0, { ...zeroEffect, debit: 7, credit: 4, fee: 3 }) }
    init.then(accept(costly)).then(refuse({ ...costly, predecessor: 1, successor: 2, nonce: 1 })).expect(state.ledger.fees.get(0) == 3)
  }
  run feeReducesNetGoalTest = init.expect(not(canAccept(initialLedger, { ...intent, authority: { ...intent.authority, netFloor: ASSETS.mapBy(a => if (a == 0) 3 else 0) } }, good)))
  run liabilityCapTest = init.then(refuse({ ...good, effects: good.effects.set(0, { ...good.effects.get(0), liability: 7 }) })).expect(state.ledger == initialLedger)
  run persistentLiabilityTest = init.then(accept({ ...good, effects: good.effects.set(0, { ...good.effects.get(0), liability: 3 }) })).then(accept(validCandidate(1, 1))).expect(state.ledger.liabilities.get(0) == 3)
  run insufficientSponsorNoFallbackTest = init.then(refuse({ ...good, sponsored: true, dust: 7 })).expect(state.ledger.userDust == 10 and state.ledger.sponsorRemaining == 6)
  run wrongSponsorScopeTest = init.then(refuse({ ...good, sponsored: true, sponsorBinding: { ...binding, actionId: "other" } })).expect(state.ledger == initialLedger)
  run sponsorNonceReplayTest = init.then(accept({ ...good, sponsored: true })).then(refuse({ ...validCandidate(1, 1), sponsored: true, sponsorNonce: 0 })).expect(state.ledger.sponsorSpent == 1)
  run missingReplayCircuitTest = init.then(refuse({ ...good, deployedOps: Set("pay") })).expect(state.ledger == initialLedger)
  pure val badPremises = List({ ...good, signatureVerified: false }, { ...good, nativeVerified: false }, { ...good, resourceVerified: false })
  run falseVerificationPremisesTest = init.then(3.reps(idx => refuse(badPremises.nth(idx)))).expect(state.rejects == 3 and state.ledger == initialLedger)
  run expiredIntentTest = init.then(refuse({ ...good, time: 11 })).expect(state.ledger == initialLedger)
  run oversizedResourcesTest = init.then(refuse({ ...good, encodedBytes: 65 })).expect(state.ledger == initialLedger)
  run independentAssetsTest = init.then(accept({ ...good, effects: ASSETS.mapBy(a => if (a == 1) { ...zeroEffect, debit: 3, credit: 3 } else zeroEffect) })).expect(state.ledger.gross.get(0) == 0 and state.ledger.gross.get(1) == 3 and state.ledger.balances.get(1).recipient == 3)
  run wrongSuccessorTest = init.then(refuse({ ...good, successor: 5 })).expect(state.ledger == initialLedger)
  run wrongSignerTest = init.then(refuse({ ...good, signer: "other" })).expect(state.ledger == initialLedger)
  run unauthorizedSponsorTest = init.then(refuse({ ...good, sponsored: true, sponsor: "other" })).expect(state.ledger == initialLedger)
  run unverifiedSponsorTest = init.then(refuse({ ...good, sponsored: true, sponsorVerified: false })).expect(state.ledger == initialLedger)
  run excessiveReplayWorkTest = init.then(refuse({ ...good, replayWork: 6 })).expect(state.ledger == initialLedger)
  run negativeEffectTest = init.then(refuse({ ...good, effects: good.effects.set(0, { ...zeroEffect, debit: 1, credit: 2, fee: -1 }) })).expect(state.ledger == initialLedger)
  run beforeValidityTest = init.then(refuse({ ...good, time: -1 })).expect(state.ledger == initialLedger)
}

