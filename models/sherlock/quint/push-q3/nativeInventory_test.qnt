module nativeInventory_test {
  import nativeInventory.* from "./nativeInventory"
  run emptyIsCoverageCompleteTest = init.expect(complete(inventory) and not(nativeQualified(inventory)))
  run coveredDoesNotQualifyNativeTest = init.then(record(EvalIf))
    .expect(withFragment(inventory) == 1 and complete(inventory) and not(nativeQualified(inventory)))
  run repeatedRulesCountOnceInSummaryTest = init.then(record(ReturnCall)).then(record(ReturnCall))
    .expect(inventory.events == 2 and withFragment(inventory) == 1 and modelInv)
  run gapIsListedButUnrealizedTest = init.then(record(ScalarConstruct))
    .expect(gaps(inventory) == Set(ScalarConstruct) and unlisted(inventory) == Set() and not(complete(inventory)))
  run unknownIsNotDoubleCountedAsGapTest = init.then(record(Unknown))
    .expect(gaps(inventory) == Set() and unlisted(inventory) == Set(Unknown) and withFragment(inventory) == 0)
  run mixedPartitionTest = init.then(record(EvalIf)).then(record(MatchSelected)).then(record(Unknown))
    .expect(inventory.events == 3 and exercised(inventory).size() == 3 and withFragment(inventory) == 1 and modelInv)
}
