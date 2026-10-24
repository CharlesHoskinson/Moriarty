module literalCompleteImagesTest {
  import literalCompleteInputs.* from "./literalCompleteInputs"
  // Same-author assertions. No independent expected-data or actual-image reads.
  pure val zeroFuel: Row = List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val afterBorrow: Row = List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  // Entire expected images assembled from the source's unchanged regions and
  // handwritten changed lanes; do not call either transition-under-test here.
  pure def expectedMiddle(initialImage: Image, remainingBytes: Row): Image = {
    val controls = List(
      List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),
      List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))
    {root: initialImage.root.slice(0, 9).concat(remainingBytes)
        .concat(initialImage.root.slice(25, 45)),
      tables: initialImage.tables.slice(0, 5).append(controls)}
  }
  pure def expectedDone(initialImage: Image, remainingBytes: Row): Image = {
    root: List(0,0,0,0,0,0,0,0,2).concat(remainingBytes)
      .concat(initialImage.root.slice(25, 41)).concat(List(1,0,0,0)),
    tables: zeroTables,
  }
  // Assert each entire public-vector span, not just its length or final value.
  pure def allStatementSpans(vector: Row, programImage: Image, images: List[Image]): bool = {
    val programCells = flattenImage(programImage)
    val first = flattenImage(images.nth(0))
    val middle = flattenImage(images.nth(1))
    val last = flattenImage(images.nth(2))
    val begin = 2 + programCells.length()
    val afterFirst = begin + first.length()
    val afterMiddle = afterFirst + middle.length()
    val afterLast = afterMiddle + last.length()
    vector.slice(0, 2) == contextInput
      and vector.slice(2, begin) == programCells
      and vector.slice(begin, afterFirst) == first
      and vector.slice(afterFirst, afterMiddle) == middle
      and vector.slice(afterMiddle, afterLast) == last
      and vector.slice(afterLast, vector.length()) == List(3,1,0,0)
      and vector.length() == afterLast + 4
  }
  run uintFuel1CompleteImageTest = initWith(uintProgram, uintInit, contextInput)
    .expect(execution.states == List(uintInit))
    .then(literalStep)
    .expect(execution.states == List(uintInit, expectedMiddle(uintInit, zeroFuel)))
    .then(publicationStep)
    .expect(execution.states == List(uintInit, expectedMiddle(uintInit, zeroFuel),
      expectedDone(uintInit, zeroFuel)))
    .expect(execution.envelopes == List(literalEnvelope, returnEnvelope)
      and terminalWitness and terminalRelation and exactStatementLength and fuelWorkAgreement
      and allStatementSpans(publicStatement, uintProgram, execution.states))
  run boolFuel256BorrowCompleteImageTest = initWith(boolProgram, boolInit, contextInput)
    .then(literalStep)
    .expect(execution.states == List(boolInit, expectedMiddle(boolInit, afterBorrow)))
    .then(publicationStep)
    .expect(execution.states == List(boolInit, expectedMiddle(boolInit, afterBorrow),
      expectedDone(boolInit, afterBorrow)))
    .expect(execution.envelopes == List(literalEnvelope, returnEnvelope)
      and terminalWitness and terminalRelation and exactStatementLength and fuelWorkAgreement
      and allStatementSpans(publicStatement, boolProgram, execution.states))
}
