module rustResidualIdentity_test {
  import rustResidualIdentity.* from "./rustResidualIdentity"
  run literalFuelChangesStaticMembershipTest = init.then(compilePair)
    .expect(comparison.fuelZero.staticRules == Set(84) and comparison.fuelOne.staticRules == Set(80, 102) and sourceInv)
  run sameProgramGetsDifferentIRProjectionTest = init.then(compilePair)
    .expect(comparison.fuelZero.programImage == comparison.fuelOne.programImage
      and observedRuntimeDependency and sourceInv)
  run freePublicationIsNotTheChargedLiteralTest = init.then(compilePair)
    .expect(charges.get(80) == 0 and charges.get(84) == 0 and charges.get(102) == 1 and sourceInv)
}
