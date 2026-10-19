module literalCompleteImages {
  // Source counterpart of literal_complete_execution.rs, not a proof workflow.
  // Domain premise: caller supplies owner-admitted literal_schema images, Init,
  // positive 16-byte LE fuel, selected Literal/uiValid and two context limbs.
  // This module does not implement admission, hashing, projectA or StepInv.
  // All allocated cells are retained, including inactive rows and unused tables.
  type Row = List[int]
  type Image = {root: Row, tables: List[List[Row]]}
  type Envelope = {ruleTag: int, charge: int}
  type Complete = {
    program: Image, origin: Image, intermediate: Image, done: Image,
    literalEnvelope: Envelope, returnEnvelope: Envelope,
  }
  type Execution = {
    program: Image, context: Row, states: List[Image], envelopes: List[Envelope],
  }
  pure val PROFILE = "sherlock-a102-a80-complete-raw-images/1"
  pure val STATEMENT_FORMAT = "moriarty-sherlock-a102-a80-literal-complete-statement/1"
  pure val literalEnvelope: Envelope = {ruleTag: 3, charge: 1}
  pure val returnEnvelope: Envelope = {ruleTag: 0, charge: 0}

  // All lane numbers below are frozen literal_host::literal_schema order.
  // Root: active*0..3,depth4,controlLength5,env6..7,stackLimit8,
  // remainingFuel9..24,initialFuel25..40,outcome41..44.
  pure def zerosLike(rowValues: Row): Row = rowValues.foldl(List(), (acc, _) => acc.append(0))
  pure def replaceCell(rowValues: Row, position: int, cellValue: int): Row = {
    val scan = rowValues.foldl({cursor: 0, out: List()}, (acc, oldValue) => {
      cursor: acc.cursor + 1,
      out: acc.out.append(if (acc.cursor == position) cellValue else oldValue),
    })
    scan.out
  }
  pure def fuelBytes(image: Image): Row = image.root.slice(9, 25)
  pure def fuelNumber(bytesLE: Row): int = {
    val scan = bytesLE.foldl({total: 0, weight: 1}, (acc, byteValue) => {
      total: acc.total + acc.weight * byteValue, weight: acc.weight * 256,
    })
    scan.total
  }
  // Same mathematical byte subtraction as A102. Nonzero input fuel is a premise.
  pure def decrementFuel(bytesLE: Row): Row = {
    val scan = bytesLE.foldl({borrow: 1, out: List()}, (acc, byteValue) => {
      borrow: if (byteValue < acc.borrow) 1 else 0,
      out: acc.out.append(if (byteValue < acc.borrow)
        byteValue + 256 - acc.borrow else byteValue - acc.borrow),
    })
    scan.out
  }
  pure def selectedLiteral(programImage: Image, beforeImage: Image): Row = {
    val expressionId = beforeImage.tables.nth(5).nth(0).nth(1)
    programImage.tables.nth(4).nth(expressionId)
  }
  pure def literalAfter(programImage: Image, beforeImage: Image): Image = {
    val literalRow = selectedLiteral(programImage, beforeImage)
    val remainingBytes = decrementFuel(fuelBytes(beforeImage))
    val rootScan = beforeImage.root.foldl({cursor: 0, out: List()}, (acc, oldValue) => {
      cursor: acc.cursor + 1,
      out: acc.out.append(if (9 <= acc.cursor and acc.cursor < 25)
        remainingBytes.nth(acc.cursor - 9) else oldValue),
    })
    val controlScan = beforeImage.tables.nth(5).foldl(
      {cursor: 0, out: List()}, (acc, oldRow) => {
        cursor: acc.cursor + 1,
        out: acc.out.append(if (acc.cursor == 0)
          replaceCell(replaceCell(replaceCell(zerosLike(oldRow), 0, 1),
            2, literalRow.nth(2)), 3, literalRow.nth(3))
          else zerosLike(oldRow)),
      })
    {root: rootScan.out, tables: beforeImage.tables.slice(0, 5).append(controlScan.out)}
  }
  pure def publicationAfter(beforeImage: Image): Image = {
    val returnRow = beforeImage.tables.nth(5).nth(0)
    val rootScan = beforeImage.root.foldl({cursor: 0, out: List()}, (acc, oldValue) => {
      cursor: acc.cursor + 1,
      out: acc.out.append(if (acc.cursor == 5) 0
        else if (acc.cursor == 41) 1
        else if (acc.cursor == 42) returnRow.nth(2)
        else if (acc.cursor == 43) returnRow.nth(3)
        else if (acc.cursor == 44) 0 else oldValue),
    })
    val emptyControls = beforeImage.tables.nth(5).foldl(
      List(), (acc, oldRow) => acc.append(zerosLike(oldRow)))
    {root: rootScan.out, tables: beforeImage.tables.slice(0, 5).append(emptyControls)}
  }
  pure def flattenImage(image: Image): Row = image.tables.foldl(image.root,
    (acc, tableRows) => tableRows.foldl(acc, (cells, rowValues) => cells.concat(rowValues)))
  pure def statementVector(contextLimbs: Row, complete: Complete): Row =
    contextLimbs.concat(flattenImage(complete.program))
      .concat(flattenImage(complete.origin)).concat(flattenImage(complete.intermediate))
      .concat(flattenImage(complete.done)).concat(List(
        complete.literalEnvelope.ruleTag, complete.literalEnvelope.charge,
        complete.returnEnvelope.ruleTag, complete.returnEnvelope.charge))

  // Relational check independent of the two update functions: exact image-wide
  // continuity, all preserved cells, each changed root/control lane, actual charge.
  pure def relation(complete: Complete): bool = {
    val beforeImage = complete.origin
    val middleImage = complete.intermediate
    val doneImage = complete.done
    val literalRow = selectedLiteral(complete.program, beforeImage)
    val firstControl = middleImage.tables.nth(5).nth(0)
    complete.literalEnvelope == literalEnvelope and complete.returnEnvelope == returnEnvelope
      and beforeImage.root.indices().forall(i =>
        if (9 <= i and i < 25) true else beforeImage.root.nth(i) == middleImage.root.nth(i))
      and fuelNumber(fuelBytes(middleImage)) == fuelNumber(fuelBytes(beforeImage)) - 1
      and middleImage.tables.slice(0, 5) == beforeImage.tables.slice(0, 5)
      and firstControl.indices().forall(i => firstControl.nth(i) ==
        (if (i == 0) 1 else if (i == 2) literalRow.nth(2)
         else if (i == 3) literalRow.nth(3) else 0))
      and middleImage.tables.nth(5).slice(1, middleImage.tables.nth(5).length())
        .foldl(true, (ok, rowValues) => ok and rowValues.foldl(true,
          (rowOk, cellValue) => rowOk and cellValue == 0))
      and doneImage.root.indices().forall(i => doneImage.root.nth(i) ==
        (if (i == 5) 0 else if (i == 41) 1 else if (i == 42) firstControl.nth(2)
         else if (i == 43) firstControl.nth(3) else if (i == 44) 0
         else middleImage.root.nth(i)))
      and doneImage.tables.slice(0, 5) == middleImage.tables.slice(0, 5)
      and doneImage.tables.nth(5).foldl(true, (ok, rowValues) => ok and
        rowValues.foldl(true, (rowOk, cellValue) => rowOk and cellValue == 0))
  }

  var execution: Execution
  action initWith(programImage: Image, initialImage: Image, contextLimbs: Row) =
    execution' = {program: programImage, context: contextLimbs,
      states: List(initialImage), envelopes: List()}
  action literalStep = all {
    execution.states.length() == 1,
    execution' = {...execution,
      states: execution.states.append(literalAfter(execution.program, execution.states.nth(0))),
      envelopes: execution.envelopes.append(literalEnvelope)},
  }
  action publicationStep = all {
    execution.states.length() == 2,
    execution' = {...execution,
      states: execution.states.append(publicationAfter(execution.states.nth(1))),
      envelopes: execution.envelopes.append(returnEnvelope)},
  }
  action step = any {literalStep, publicationStep}
  val terminalWitness = execution.states.length() == 3
  val complete: Complete = {
    program: execution.program, origin: execution.states.nth(0),
    intermediate: execution.states.nth(1), done: execution.states.nth(2),
    literalEnvelope: execution.envelopes.nth(0), returnEnvelope: execution.envelopes.nth(1),
  }
  val terminalRelation = if (terminalWitness) relation(complete) else true
  val chargedWork = execution.envelopes.foldl(0, (acc, envelope) => acc + envelope.charge)
  val fuelWorkAgreement = if (terminalWitness) chargedWork == 1
    and fuelNumber(fuelBytes(complete.origin)) - fuelNumber(fuelBytes(complete.done)) == chargedWork
    else true
  val publicStatement = statementVector(execution.context, complete)
  val exactStatementLength = if (terminalWitness)
    publicStatement.length() == 6 + flattenImage(execution.program).length()
      + execution.states.foldl(0, (acc, image) => acc + flattenImage(image).length())
    else true
  val compositionInv = terminalRelation and exactStatementLength and fuelWorkAgreement
}
