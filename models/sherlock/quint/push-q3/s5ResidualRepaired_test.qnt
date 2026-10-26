module s5ResidualRepaired_test {
  import s5ResidualRepaired.* from "./s5ResidualRepaired"
  action begin(mask: Mask, limit: int): bool = prep' = start(mask, limit)
  run arithRetainsDynamicCallViaGenericTest = begin(ArithFamily, 5).then(tick)
    .expect(prep.targets.get(0) == Set(1, 4) and prep.pending == List(1)
      and prep.delegated == Set(4) and modelInv)
  run arithCompletesWithoutSpecializingCallTest = begin(ArithFamily, 5)
    .then(tick).then(tick).then(tick).then(tick)
    .expect(finished and prep.blocks == Set(0, 1, 2, 3) and delegatedCall and modelInv)
  run zeroBudgetKeepsGenericRootTest = begin(ArithFamily, 0).then(tick)
    .expect(finished and prep.generic == Set(0) and prep.delegated == Set() and modelInv)
  run imageKeepsRecursiveBackedgeTest = begin(Image, 5)
    .then(tick).then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(finished and fullImage and prep.targets.get(4) == Set(0) and modelInv)
}
