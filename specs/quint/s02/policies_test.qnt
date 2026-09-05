module policies_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"

  run settlementBranchTest = assert(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts))
  run refundBranchTest = assert(bodyAllows(aliceBody, OpVoluntaryRefund, refundInput, 2, List(aliceRefund, bobRefund), facts))
  run oneLegRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(aliceLeg), facts)))
  run afterResolveFullBindingTest = assert(policyAllowsObservation(aliceAfter, OpSettle, settleObservation, facts))
  run beforeResolveBoundedBodyTest = assert(policyAllowsObservation(aliceBefore, OpSettle, settleObservation, facts))
  run bothDebitOwnersTest = assert(allRequiredPoliciesAllow(Map(aliceKey -> aliceAfter, bobKey -> bobAfter), OpSettle, settleObservation, facts))
  run missingDebitOwnerTest = assert(not(allRequiredPoliciesAllow(Map(aliceKey -> aliceAfter), OpSettle, settleObservation, facts)))
}
