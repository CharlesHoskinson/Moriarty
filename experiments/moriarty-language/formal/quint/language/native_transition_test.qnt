module native_transition_test {
  import native_transition.* from "./native_transition"
  run validProofReceiptTest = init.then(verify(good)).expect(state.accepts == 1 and state.receipts == List(expected))
  run everyMetadataBindingTest = init.then(7.reps(i => refuse({ ...good, metadata: badClaims.nth(i) })))
    .expect(state.accepts == 0 and state.rejects == 7 and state.receipts == List())
  // Metadata is accurate; the reused proof commits to a different claim.
  run forgedMetadataRelabelingTest = init.then(7.reps(i => refuse({ ...good, proofClaim: badClaims.nth(i) })))
    .expect(state.accepts == 0 and state.rejects == 7 and state.receipts == List())
  pure val invalid = List(
    { ...good, relation: "attacker-key" }, { ...good, nativeValid: false },
    { ...good, pinnedParameters: false }, { ...good, canonicalProof: false },
    { ...good, liabilities: 1 }, { ...good, cap: 0 }, { ...good, cap: 9 }, { ...good, cap: 20 }
  )
  run everyRefusalPredicateTest = init.then(8.reps(i => refuse(invalid.nth(i))))
    .expect(state.rejects == 8 and state.receipts == List())
  run acceptedReceiptSurvivesRefusalTest = init.then(verify(good)).then(refuse(invalid.nth(0)))
    .expect(state.accepts == 1 and state.rejects == 1 and state.receipts == List(expected))
}
