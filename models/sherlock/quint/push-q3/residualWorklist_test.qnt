module residualWorklist_test {
  import residualWorklist.* from "./residualWorklist"
  action begin(mask: Mask, limit: int): bool = prep' = start(mask, limit, sampleDynamic)
  action dropDynamicSuccessor = prep' = {...prep, pending: prep.pending.exclude(Set(3))}
  action confuseFallbackWithExhaustion = prep' = {...prep, after: {...prep.after, outcome: 8}}
  run zeroBudgetFallsBackWithoutExhaustionTest = begin(Empty, 0).then(visit(0))
    .expect(prep.generic == Set(0) and prepared and prep.after == sampleDynamic and modelInv)
  run dynamicBranchKeepsBothSuccessorsTest = begin(FixedFamily, 2).then(visit(0)).then(visit(1))
    .expect(prep.pending == Set(2, 3) and modelInv)
  run recursiveCallIsMemoizedNotUnrolledTest = begin(FixedImage, 4)
    .then(visit(0)).then(visit(1)).then(visit(2)).then(visit(3))
    .expect(prepared and fullSpecialization and prep.seen.size() == 4 and prep.after.frames == List(4, 4) and modelInv)
  run reachableLowLimitFallbackTest = begin(FixedImage, 1).then(visit(0)).then(visit(1))
    .expect(prep.blocks == Set(0) and prep.generic == Set(1) and prepared and modelInv)
  run allMasksHaveSameDynamicContractTest = begin(Empty, 4)
    .then(visit(0)).then(visit(1)).then(visit(3)).then(visit(2))
    .expect(prep.after.fuel == 0 and prep.after.outcome == 0 and prep.after.work == 9 and modelInv)
  run lostDynamicBranchIsDetectedTest = begin(FixedFamily, 2).then(visit(0)).then(visit(1))
    .then(dropDynamicSuccessor).expect(not(modelInv))
  run fallbackExhaustionIsDetectedTest = begin(Empty, 0).then(visit(0))
    .then(confuseFallbackWithExhaustion).expect(not(modelInv))
}
