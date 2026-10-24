module c6RetainedFamilyTest {
  import c6RetainedFamilyTypes.* from "./c6RetainedFamilyTypes"
  import c6RetainedFamilyInputs.* from "./c6RetainedFamilyInputs"
  import c6RetainedFamilyProjection.* from "./c6RetainedFamilyProjection"
  import independentC6RetainedFamilyExpected.* from "./independentC6RetainedFamilyExpected"

  run c6RetainedFamilyCompleteObservationsTest = all {
    c6RFCompleteObservationsExact(c6RFInputs,expectedObservations),
  }
  run c6RetainedFamilyBindingCouplingTest = all {
    c6RFInputs.foldl(true,(priorArg,inputArg) => priorArg and c6RFLayoutCoupling(inputArg)),
    c6RFOriginalPresenceContrast(c6RFInputs.nth(0),c6RFInputs.nth(1)),
    c6RFSeparateAdmissionContrast(c6RFInputs.nth(2),c6RFInputs.nth(3)),
  }
}
