module ledger_call_custody_test {
  import ledger_call_custody.* from "./ledger_call_custody"
  run paymentTest = init.then(prepare(paymentRequest))
    .expect(state.prepared and state.postBalances.get((0,0)) == 28 and state.postBalances.get((1,0)) == 80
      and state.postBalances.get((2,0)) == 2 and state.duties.get(0) == 30 and state.duties.get(1) == 5
      and state.duties.get(2) == 0 and state.value == 70 and preparation_invariants)
  run collectionTest = init.then(prepare(collectionRequest))
    .expect(state.postBalances.get((0,0)) == 57 and state.postBalances.get((3,0)) == 47
      and state.postBalances.get((2,0)) == 4 and state.postBalances.get((4,0)) == 9
      and state.duties.get(0) == 5 and state.duties.get(1) == 5 and state.duties.get(2) == 0
      and state.duties.get(3) == 4 and state.frozen and state.value == 40 and preparation_invariants)
  run selfPaymentTest = init.then(prepare(selfRequest))
    .expect(state.postBalances.get((0,0)) == 80 and state.outputs == List(out(12,0),out(20,1),out(68,0)) and preparation_invariants)
  run equalMultiplicityTest = init.then(prepare(equalRequest))
    .expect(state.outputs == List(out(50,0),out(50,0)) and state.postBalances.get((0,0)) == 100 and preparation_invariants)
  run repeatedRecipientTest = init.then(prepare(repeatedRequest))
    .expect(state.outputs == List(out(27,1),out(73,0)) and preparation_invariants)
  run zeroOnlyTest = init.then(prepare(zeroRequest)).expect(state.claims == Map() and state.outputs == List(out(100,0)) and preparation_invariants)
  run emptyOfferTest = init.then(prepare(emptyRequest)).expect(state.outputs == List() and state.claims == Map() and preparation_invariants)
  run aboveU64Test = init.then(prepare(largeRequest)).expect(state.claims.get((0,1)) == 36893488147419103232 and preparation_invariants)
  run mergedChangeRefusalTest = init.then(refuse({ ...selfRequest, outputs: List(out(20,1),out(80,0)) })).expect(not(state.prepared) and state.refused and preparation_invariants)
  run dedupRefusalTest = init.then(refuse({ ...equalRequest, outputs: List(out(50,0)) })).expect(state.refused and preparation_invariants)
  run wrongChangeRecipientTest = init.then(refuse({ ...equalRequest, outputs: List(out(50,0),out(50,1)) })).expect(state.refused and preparation_invariants)
  run splitGroupTest = init.then(refuse({ ...repeatedRequest, outputs: List(out(2,1),out(10,1),out(15,1),out(73,0)) })).expect(state.refused and preparation_invariants)
  run extraOutputTest = init.then(refuse({ ...paymentRequest, outputs: List(out(2,2),out(9,4),out(19,0),out(70,1)) })).expect(state.refused and preparation_invariants)
  run missingClaimTest = init.then(refuse({ ...paymentRequest, claims: Map((0,1)->69,(0,2)->2) })).expect(state.refused and preparation_invariants)
  run duplicateInputTest = init.then(refuse({ ...paymentRequest, selected: List(0,0) })).expect(state.refused and preparation_invariants)
  run zeroOutputTest = init.then(refuse({ ...zeroRequest, outputs: List(out(0,1),out(100,0)) })).expect(state.refused and preparation_invariants)
  run changedUntouchedTest = init.then(refuse({ ...paymentRequest, retained: List() })).expect(state.refused and preparation_invariants)
  run missingZeroDutyTest = init.then(refuse({ ...paymentRequest, expectedDuties: Map(0->30,1->5,3->0) })).expect(state.refused and preparation_invariants)
  run weakSubsetWitnessTest = all { nativeSubset(equalRequest.claims, List(out(50,0),out(50,1))), not(canPrepare({ ...equalRequest, outputs: List(out(50,0),out(50,1)) })) }
}
