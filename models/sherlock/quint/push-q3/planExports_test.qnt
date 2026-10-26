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
  run omissionsAreSeparateAndLocatedTest = init.then(request(List(structured, closure, lookup, interaction), basicCaps, Foreign))
    .expect(state.report.constraintOmissions == Set(2, 3) and state.report.witnessOmissions == Set(1)
      and state.report.manifest.nth(1).source == 10 and state.report.manifest.nth(1).component == 21 and modelInv)
  run orderedOperandTypesSurviveTest = init.then(request(List(structured, closure, lookup, interaction), allCaps, Midnight))
    .expect(state.report.manifest.nth(0).inputTypes == List(UInt128, UInt128, UInt128)
      and state.report.manifest.nth(2).outputTypes == List(Field) and modelInv)
}
