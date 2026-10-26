module programPatternRows_test {
  import programPatternRows.* from "./programPatternRows"
  pure val rowCases = tags.map(tag => List(tag)).union(tags.map(left => tags.map(right => List(left, right))).flatten())
  pure def repairedPrefixesAgree(rows: List[int], previousTag: int): bool = {
    val initial = start(rows, previousTag)
    val result = initial.lanes.foldl({parser: initial.repaired, cursor: 0, valid: true}, (state, lane) => {
      val parser = consume(state.parser, lane, state.cursor, true)
      val cursor = state.cursor + 1
      {parser: parser, cursor: cursor, valid: state.valid and agrees({...initial, cursor: cursor}, parser)}
    })
    result.valid and result.parser.remaining == 0 and result.parser.doneAt == initial.lanes.length()
  }
  run all504InitialCasesPreserveEveryRepairedPrefixTest = begin(List(0), 0)
    .expect(rowCases.size() == 56 and rowCases.forall(rows => 0.to(8).forall(previous => repairedPrefixesAgree(rows, previous))))
  run codecWidthsAgreeWithLiteralEncodingsTest = begin(List(0), 0)
    .expect(tags.forall(tag => widths.nth(tag) == encodings.nth(tag).length()) and modelInv)
  run nilAfterVariableTagFailsToFinishTest = begin(List(1), 0).then(step)
    .expect(finished and sourceLate and not(sourceMatchesCodec) and scan.repaired.doneAt == 1 and modelInv)
  run wildcardAfterConstructorTagFailsToFinishTest = begin(List(5), 6).then(step)
    .expect(sourceLate and scan.repaired.remaining == 0 and modelInv)
  run staleUnitTagEndsWiderRowEarlyTest = begin(List(0), 1).then(step)
    .expect(sourceEarly and scan.source.doneAt == 1 and scan.repaired.remaining == 1 and modelInv)
  run previousExprVarTagPrematurelyEndsLeafRowTest = begin(List(3), 1).then(step)
    .expect(sourceEarly and scan.source.remaining == 0 and scan.repaired.sub == 1 and modelInv)
  run equalNonUnitWidthsAreNotEnoughToExposeBugTest = begin(List(3), 0).then(step).then(step)
    .expect(finished and sourceMatchesCodec and modelInv)
  run consecutiveUnitRowsCanHideStaleTagTest = begin(List(1, 5), 1).then(step).then(step)
    .expect(finished and sourceMatchesCodec and modelInv)
  run variableThenNilMustEndAtThirdLaneTest = begin(List(0, 1), 0).then(step).then(step).then(step)
    .expect(finished and sourceLate and scan.repaired.doneAt == 3 and modelInv)
  run constructorHasFiveLanesTest = begin(List(6), 0).then(step).then(step).then(step).then(step).then(step)
    .expect(finished and sourceMatchesCodec and scan.repaired.doneAt == 5 and modelInv)
}
