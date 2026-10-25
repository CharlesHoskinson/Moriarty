module planExports_test {
  import planExports.* from "./planExports"
  run jsonDoesNotSupplyWitnessClosureTest = init.then(request(List(structured, closure), basicCaps, ConstraintJson))
    .expect(constraintsOnly and not(state.report.midnightClaim) and modelInv)
  run lookupCannotBeSilentlyDroppedTest = init.then(request(List(structured, lookup), basicCaps, Foreign))
    .expect(state.report.constraintDisposition == Unsupported and state.report.witnessDisposition == Supported and modelInv)
  run interactionAndCustomGateAreRequiredTest = init.then(request(List(structured, interaction), basicCaps, Midnight))
    .expect(state.report.constraintDisposition == Unsupported and not(midnightSupported) and modelInv)
  run foreignIsNotMidnightTest = init.then(request(List(structured, closure, lookup, interaction), allCaps, Foreign))
    .expect(state.report.constraintDisposition == Supported and state.report.witnessDisposition == Supported
      and not(midnightSupported) and modelInv)
  run unsupportedFieldBlocksBothPlansTest = init.then(request(List(structured), {...allCaps, field: false}, Midnight))
    .expect(state.report.constraintDisposition == Unsupported and state.report.witnessDisposition == Unsupported and modelInv)
  run orderAndMultiplicitySurviveTest = init.then(request(List(structured, closure, lookup, interaction), allCaps, Midnight))
    .expect(state.report.constraintPlan == List(0, 1, 2, 3) and state.report.manifest.nth(0).inputs == List(2, 1, 2)
      and midnightSupported and modelInv)
}
