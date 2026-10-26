module s5ResidualProjection_test {
  import s5ResidualProjection.* from "./s5ResidualProjection"
  action begin(mask: Mask, limit: int): bool = prep' = start(mask, limit)
  run committedArithMaskLosesRuntimeCallTest = begin(ArithFamily, 5).then(tick)
    .expect(lostCallSuccessor and not(preservesDynamicTargets) and prep.generic == Set())
  run emptyMaskRetainsBothDispatchTargetsTest = begin(Empty, 5).then(tick)
    .expect(prep.targets.get(0) == Set(1, 4) and preservesDynamicTargets)
  run recursiveFamilyRetainsRuntimeCallTest = begin(RecursiveFamily, 5).then(tick)
    .expect(prep.targets.get(0) == Set(1, 4) and preservesDynamicTargets)
  run zeroLimitUsesGenericInsteadOfPruningTest = begin(ArithFamily, 0).then(tick)
    .expect(prep.generic == Set(0) and prep.blocks == Set() and finished and preservesDynamicTargets)
}
