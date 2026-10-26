module financialImportTest {
  import financialImport.* from "./financialImport"
  run validImportedCountersTest = initCase(10, 7, 3, true).then(importStep)
    .expect(currentCodeInv and acceptedWorkInv and r0FirstOrderingInv).then(chargeStep)
    .expect(currentCodeInv and acceptedWorkInv and checkpoint.remaining == 6 and checkpoint.consumed == 4)
  run canonicalMalformedCountersAcceptedTest = initCase(0, 1, 0, true).then(importStep)
    .expect(currentCodeInv and not(acceptedWorkInv)).then(chargeStep)
    .expect(currentCodeInv and not(acceptedWorkInv) and checkpoint.phase == Charged)
  run maximumConsumedCounterAcceptedTest = initCase(MAX, 1, MAX, true).then(importStep)
    .expect(currentCodeInv and not(acceptedWorkInv)).then(chargeStep)
    .expect(currentCodeInv and overflowReached)
  run zeroRemainingRefusedAtR0WhenObsValidTest = initCase(4, 0, 4, true).then(importStep)
    .expect(currentCodeInv and r0FirstOrderingInv and checkpoint.phase == RefusedR0)
  run zeroRemainingWithBadObsReportsR3BeforeR0Test =
    initFullCase(4, 0, 4, true, false, false).then(importStep)
      .expect(currentCodeInv and r0LateAfterR3Observed and not(r0FirstOrderingInv)
        and checkpoint.phase == RefusedR3)
  run maximumEpochRefusedAtR0Test =
    initFullCase(10, 7, 3, true, true, false).then(importStep)
      .expect(currentCodeInv and epochMaxRefused and checkpoint.phase == RefusedR0)
  run noncanonicalRefusedTest = initCase(0, 1, 0, false).then(importStep)
    .expect(currentCodeInv and checkpoint.phase == RefusedR0)
}
