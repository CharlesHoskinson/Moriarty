module literalCompleteImagesTest {
  import literalCompleteImages.* from "./literalCompleteImages"
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
  // Shared-helper span consistency, supplemented by handwritten absolute spans below.
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
  // Handwritten schema offsets: Program86, State213, statement731 logical lanes.
  // No call to flattenImage or dynamically computed block length occurs here.
  pure def absoluteProgramSpans(vector: Row, programImage: Image): bool =
    vector.slice(2, 18) == programImage.root
      and vector.slice(18, 22) == programImage.tables.nth(2).nth(0)
      and vector.slice(22, 33) == programImage.tables.nth(4).nth(0)
      and vector.slice(33, 36) == programImage.tables.nth(10).nth(0)
      and vector.slice(36, 39) == programImage.tables.nth(10).nth(1)
      and vector.slice(39, 42) == programImage.tables.nth(10).nth(2)
      and vector.slice(42, 45) == programImage.tables.nth(10).nth(3)
      and vector.slice(45, 49) == programImage.tables.nth(11).nth(0)
      and vector.slice(49, 53) == programImage.tables.nth(11).nth(1)
      and vector.slice(53, 57) == programImage.tables.nth(11).nth(2)
      and vector.slice(57, 61) == programImage.tables.nth(11).nth(3)
      and vector.slice(61, 62) == programImage.tables.nth(12).nth(0)
      and vector.slice(62, 63) == programImage.tables.nth(12).nth(1)
      and vector.slice(63, 64) == programImage.tables.nth(12).nth(2)
      and vector.slice(64, 65) == programImage.tables.nth(12).nth(3)
      and vector.slice(65, 66) == programImage.tables.nth(12).nth(4)
      and vector.slice(66, 88) == programImage.tables.nth(13).nth(0)
  pure def absoluteStateSpan(vector: Row, begin: int, image: Image): bool =
    vector.slice(begin, begin + 45) == image.root
      and vector.slice(begin + 45, begin + 67) == image.tables.nth(0).nth(0)
      and vector.slice(begin + 67, begin + 89) == image.tables.nth(0).nth(1)
      and vector.slice(begin + 89, begin + 91) == image.tables.nth(1).nth(0)
      and vector.slice(begin + 91, begin + 93) == image.tables.nth(1).nth(1)
      and vector.slice(begin + 93, begin + 98) == image.tables.nth(2).nth(0)
      and vector.slice(begin + 98, begin + 103) == image.tables.nth(2).nth(1)
      and vector.slice(begin + 103, begin + 105) == image.tables.nth(3).nth(0)
      and vector.slice(begin + 105, begin + 107) == image.tables.nth(3).nth(1)
      and vector.slice(begin + 107, begin + 124) == image.tables.nth(4).nth(0)
      and vector.slice(begin + 124, begin + 141) == image.tables.nth(4).nth(1)
      and vector.slice(begin + 141, begin + 177) == image.tables.nth(5).nth(0)
      and vector.slice(begin + 177, begin + 213) == image.tables.nth(5).nth(1)
  pure def absoluteLayoutChecks(vector: Row, programImage: Image, images: List[Image]): bool =
    vector.length() == 731 and vector.slice(0, 2) == List(17,29)
      and absoluteProgramSpans(vector, programImage)
      and absoluteStateSpan(vector, 88, images.nth(0))
      and absoluteStateSpan(vector, 301, images.nth(1))
      and absoluteStateSpan(vector, 514, images.nth(2))
      and vector.nth(306) == 1 and vector.nth(442) == 1
      and vector.nth(519) == 0 and vector.nth(555) == 1
      and vector.slice(727, 731) == List(3,1,0,0)
  run uintFuel1CompleteImageTest = initWith(uintProgram, uintInit, contextInput)
    .expect(execution.states == List(uintInit) and observedFixturePremise)
    .then(literalStep)
    .expect(execution.states == List(uintInit, expectedMiddle(uintInit, zeroFuel)))
    .then(publicationStep)
    .expect(execution.states == List(uintInit, expectedMiddle(uintInit, zeroFuel),
      expectedDone(uintInit, zeroFuel)))
    .expect(execution.envelopes == List(literalEnvelope, returnEnvelope)
      and terminalWitness and terminalRelation and exactStatementLength and fuelWorkAgreement
      and allStatementSpans(publicStatement, uintProgram, execution.states)
      and observedFixturePremise and absoluteLayoutChecks(publicStatement, uintProgram, execution.states)
      and publicStatement.nth(66) == 0 and publicStatement.nth(69) == 7
      and publicStatement.nth(97) == 1 and publicStatement.nth(98) == 0
      and publicStatement.nth(310) == 0 and publicStatement.nth(311) == 0
      and publicStatement.nth(523) == 0 and publicStatement.nth(524) == 0)
  run boolFuel256BorrowCompleteImageTest = initWith(boolProgram, boolInit, contextInput)
    .expect(observedFixturePremise)
    .then(literalStep)
    .expect(execution.states == List(boolInit, expectedMiddle(boolInit, afterBorrow)))
    .then(publicationStep)
    .expect(execution.states == List(boolInit, expectedMiddle(boolInit, afterBorrow),
      expectedDone(boolInit, afterBorrow)))
    .expect(execution.envelopes == List(literalEnvelope, returnEnvelope)
      and terminalWitness and terminalRelation and exactStatementLength and fuelWorkAgreement
      and allStatementSpans(publicStatement, boolProgram, execution.states)
      and observedFixturePremise and absoluteLayoutChecks(publicStatement, boolProgram, execution.states)
      and publicStatement.nth(66) == 1 and publicStatement.nth(67) == 1
      and publicStatement.nth(69) == 0
      and publicStatement.nth(97) == 0 and publicStatement.nth(98) == 1
      and publicStatement.nth(310) == 255 and publicStatement.nth(311) == 0
      and publicStatement.nth(523) == 255 and publicStatement.nth(524) == 0)
}
