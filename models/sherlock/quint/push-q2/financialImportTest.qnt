module financialImportTest {
  import financialImport.* from "./financialImport"
  run validImportedCountersTest = initCase(10, 7, 3, true).then(importStep)
    .expect(currentCodeInv and acceptedWorkInv).then(chargeStep)
    .expect(currentCodeInv and acceptedWorkInv and checkpoint.remaining == 6 and checkpoint.consumed == 4)
  run canonicalMalformedCountersAcceptedTest = initCase(0, 1, 0, true).then(importStep)
    .expect(currentCodeInv and not(acceptedWorkInv)).then(chargeStep)
    .expect(currentCodeInv and not(acceptedWorkInv) and checkpoint.phase == Charged)
  run maximumConsumedCounterAcceptedTest = initCase(MAX, 1, MAX, true).then(importStep)
    .expect(currentCodeInv and not(acceptedWorkInv)).then(chargeStep)
    .expect(currentCodeInv and overflowReached)
  run noncanonicalRefusedTest = initCase(0, 1, 0, false).then(importStep)
    .expect(currentCodeInv and checkpoint.phase == Refused)
}
