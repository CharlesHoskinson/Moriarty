module literalParametricImages {
  // NEW model-only reference: scalar Literal Step -> A80 publication.
  // No B20/B21 file is imported or changed. Full uiValid/StepInv is not claimed.
  type PRow = List[int]
  type PImage = {root: PRow, tables: List[List[PRow]]}
  type PEnvelope = {ruleTag: int, charge: int}
  type PLayout = {programRows: List[int], stateRows: List[int], stackLimit: int}
  type PChild = {
    program: PImage, context: PRow, before: PImage, after: PImage, envelope: PEnvelope,
  }
  type PExecution = {
    layout: PLayout, program: PImage, context: PRow,
    states: List[PImage], envelopes: List[PEnvelope],
  }
  pure val PARAMETRIC_MODEL_PROFILE = "sherlock-a102-a80-scalar-parametric-reference/1"
  pure val P_NATIVE_BOUND = 340282366920938463463374607431768211456
  pure val pProgramWidths = List(2,1,4,1,11,1,6,2,3,2,3,4,1,22,2)
  pure val pStateWidths = List(22,2,5,2,17,36)
  pure val pLiteralEnvelope: PEnvelope = {ruleTag: 3, charge: 1}
  pure val pPublicationEnvelope: PEnvelope = {ruleTag: 0, charge: 0}

  pure def pAllZero(cells: PRow): bool = cells.foldl(true, (zeroSoFar, cell) => zeroSoFar and cell == 0)
  pure def pZeroRow(width: int): PRow = range(0,width).foldl(List(),
    (resultCells, _) => resultCells.append(0))
  pure def pReplace(cells: PRow, position: int, newCell: int): PRow = {
    val changed = cells.foldl({cursor: 0, out: List()}, (scanCells, oldCell) => {
      cursor: scanCells.cursor + 1,
      out: scanCells.out.append(if (scanCells.cursor == position) newCell else oldCell),
    })
    changed.out
  }
  pure def pByteRow(cells: PRow): bool = cells.length() == 16
    and cells.foldl(true, (rangeSoFar, byteCell) => rangeSoFar and 0 <= byteCell and byteCell < 256)
  pure def pFuelValue(bytesLE: PRow): int = {
    val evaluated = bytesLE.foldl({total: 0, weight: 1}, (fuelScan, fuelByte) => {
      total: fuelScan.total + fuelScan.weight * fuelByte, weight: fuelScan.weight * 256,
    })
    evaluated.total
  }
  pure def pSubtractOne(bytesLE: PRow): PRow = {
    val subtracted = bytesLE.foldl({borrow: 1, out: List()}, (fuelScan, fuelByte) => {
      borrow: if (fuelByte < fuelScan.borrow) 1 else 0,
      out: fuelScan.out.append(if (fuelByte < fuelScan.borrow)
        fuelByte + 256 - fuelScan.borrow else fuelByte - fuelScan.borrow),
    })
    subtracted.out
  }
  pure def pLayoutAllowed(layoutArg: PLayout): bool =
    if (layoutArg.programRows.length() == 15 and layoutArg.stateRows.length() == 6) {
      layoutArg.programRows.foldl(true, (nonnegativeSoFar, capacityCell) => nonnegativeSoFar and capacityCell >= 0)
        and layoutArg.stateRows.foldl(true, (nonnegativeSoFar, capacityCell) => nonnegativeSoFar and capacityCell >= 0)
        and layoutArg.programRows.nth(4) > 0 and layoutArg.programRows.nth(13) > 0
        and layoutArg.programRows.nth(10) >= 4 and layoutArg.programRows.nth(11) >= 4
        and layoutArg.programRows.nth(12) >= 5 and layoutArg.stateRows.nth(5) > 0
        and 0 <= layoutArg.stackLimit and layoutArg.stackLimit < P_NATIVE_BOUND
    } else false
  pure def pTablesShape(tableImages: List[List[PRow]], rowCounts: List[int],
      rowWidths: List[int]): bool =
    if (tableImages.length() == rowCounts.length() and rowCounts.length() == rowWidths.length())
      rowCounts.indices().forall(tableIndex =>
        tableImages.nth(tableIndex).length() == rowCounts.nth(tableIndex)
          and tableImages.nth(tableIndex).foldl(true, (shapeSoFar, physicalRow) =>
            shapeSoFar and physicalRow.length() == rowWidths.nth(tableIndex)))
    else false
  pure def pProgramShape(layoutArg: PLayout, programArg: PImage): bool =
    pLayoutAllowed(layoutArg) and programArg.root.length() == 16
      and pTablesShape(programArg.tables, layoutArg.programRows, pProgramWidths)
  pure def pStateShape(layoutArg: PLayout, stateArg: PImage): bool =
    pLayoutAllowed(layoutArg) and stateArg.root.length() == 45
      and pTablesShape(stateArg.tables, layoutArg.stateRows, pStateWidths)
  pure def pPaddingZero(tableRows: List[PRow], activeRows: int): bool =
    tableRows.indices().forall(rowIndex => rowIndex < activeRows or pAllZero(tableRows.nth(rowIndex)))
  pure def pProgramHeaderAllowed(layoutArg: PLayout, programArg: PImage): bool =
    if (pProgramShape(layoutArg, programArg)) {
      programArg.root.indices().forall(headerIndex =>
        0 <= programArg.root.nth(headerIndex) and programArg.root.nth(headerIndex) < P_NATIVE_BOUND)
        and layoutArg.programRows.indices().forall(tableIndex =>
          programArg.root.nth(tableIndex + 1) <= layoutArg.programRows.nth(tableIndex)
            and pPaddingZero(programArg.tables.nth(tableIndex), programArg.root.nth(tableIndex + 1)))
        and programArg.root.nth(0) < programArg.root.nth(5)
    } else false
  pure def pStateHeaderAllowed(layoutArg: PLayout, stateArg: PImage): bool =
    if (pStateShape(layoutArg, stateArg)) {
      stateArg.root.foldl(true, (rangeSoFar, rootCell) => rangeSoFar and 0 <= rootCell and rootCell < P_NATIVE_BOUND)
        and layoutArg.stateRows.indices().forall(tableIndex =>
          stateArg.root.nth(tableIndex) <= layoutArg.stateRows.nth(tableIndex)
            and pPaddingZero(stateArg.tables.nth(tableIndex), stateArg.root.nth(tableIndex)))
        and stateArg.root.nth(8) <= layoutArg.stackLimit
        and stateArg.root.nth(4) <= stateArg.root.nth(8)
        and pByteRow(stateArg.root.slice(9, 25)) and pByteRow(stateArg.root.slice(25, 41))
    } else false
  // Explicit supplied complete-layout context limbs. No digest/authority is computed.
  // A pending complete-layout/A80 interface must supply its own limbs, never A102's.
  pure def pContextAllowed(contextArg: PRow): bool = contextArg.length() == 2
    and contextArg.foldl(true, (rangeSoFar, contextCell) => rangeSoFar and 0 <= contextCell and contextCell < P_NATIVE_BOUND)
  pure def pScalarConstant(constantArg: PRow): bool = if (constantArg.length() == 22) {
    pByteRow(constantArg.slice(3, 19)) and constantArg.nth(2) == 0
      and constantArg.nth(20) == 0 and constantArg.nth(21) == 0
      and (if (constantArg.nth(0) == 0)
        constantArg.nth(1) == 0 and constantArg.nth(19) == 0
        else if (constantArg.nth(0) == 1)
          constantArg.nth(1) == 1 and pAllZero(constantArg.slice(3, 19))
            and (constantArg.nth(19) == 0 or constantArg.nth(19) == 1)
        else false)
  } else false
  // Checked scalar-only subset of the fixed-A Constant domain. No constructors/DAG.
  // Other Program tables, source preparation/F4 and general scalar ranges remain external.
  pure def pScalarProgramSubset(layoutArg: PLayout, programArg: PImage): bool =
    if (pProgramHeaderAllowed(layoutArg, programArg)) {
      programArg.root.nth(11) == 4 and programArg.root.nth(12) == 4
        and programArg.root.nth(13) == 5 and programArg.root.nth(15) == 0
        and programArg.tables.nth(10).slice(0, 4) ==
          List(List(0,0,0), List(1,0,0), List(2,0,2), List(3,2,2))
        and programArg.tables.nth(11).slice(0, 4) ==
          List(List(2,0,0,0), List(2,1,0,2), List(3,0,2,1), List(3,1,3,2))
        and programArg.tables.nth(12).slice(0, 5) == List(List(0),List(2),List(0),List(3),List(3))
        and programArg.tables.nth(13).indices().forall(constantIndex =>
          if (constantIndex < programArg.root.nth(14))
            pScalarConstant(programArg.tables.nth(13).nth(constantIndex)) else true)
    } else false
  pure def pEvalControlAllowed(controlRows: List[PRow], selectorArg: int): bool =
    controlRows.indices().forall(controlIndex => controlRows.nth(controlIndex).indices()
      .forall(columnIndex => controlRows.nth(controlIndex).nth(columnIndex) ==
        (if (controlIndex == 0 and columnIndex == 1) selectorArg else 0)))
  pure def pLiteralEnabled(layoutArg: PLayout, programArg: PImage, beforeArg: PImage): bool =
    if (pScalarProgramSubset(layoutArg, programArg) and pStateHeaderAllowed(layoutArg, beforeArg)) {
      val selectedId = beforeArg.tables.nth(5).nth(0).nth(1)
      if (0 <= selectedId and selectedId < programArg.root.nth(5)) {
        val selectedRow = programArg.tables.nth(4).nth(selectedId)
        val selectedConstantId = selectedRow.nth(3)
        beforeArg.root.nth(5) == 1 and pAllZero(beforeArg.root.slice(41, 45))
          and pFuelValue(beforeArg.root.slice(9, 25)) > 0
          and pEvalControlAllowed(beforeArg.tables.nth(5), selectedId)
          and selectedId < layoutArg.programRows.nth(2)
          and selectedRow == List(0,selectedId,0,selectedConstantId,0,0,0,0,0,0,0)
          and 0 <= selectedConstantId and selectedConstantId < programArg.root.nth(14)
      } else false
    } else false
  pure def pSelectedConstant(programArg: PImage, beforeArg: PImage): PRow = {
    val selectedId = beforeArg.tables.nth(5).nth(0).nth(1)
    val selectedConstantId = programArg.tables.nth(4).nth(selectedId).nth(3)
    programArg.tables.nth(13).nth(selectedConstantId)
  }
  pure def pLiteralNext(programArg: PImage, beforeArg: PImage): PImage = {
    val selectedId = beforeArg.tables.nth(5).nth(0).nth(1)
    val selectedConstantId = programArg.tables.nth(4).nth(selectedId).nth(3)
    val afterFuel = pSubtractOne(beforeArg.root.slice(9, 25))
    val afterRoot = beforeArg.root.slice(0, 9).concat(afterFuel).concat(beforeArg.root.slice(25, 45))
    val afterControls = beforeArg.tables.nth(5).foldl({cursor:0,out:List()},
      (writtenRows, _) => {
        cursor: writtenRows.cursor + 1,
        out: writtenRows.out.append(if (writtenRows.cursor == 0)
          pReplace(pReplace(pZeroRow(36), 0, 1), 3, selectedConstantId) else pZeroRow(36)),
      })
    {root: afterRoot, tables: beforeArg.tables.slice(0, 5).append(afterControls.out)}
  }
  pure def pPublicationEnabled(layoutArg: PLayout, programArg: PImage, beforeArg: PImage): bool =
    if (pProgramHeaderAllowed(layoutArg, programArg) and pStateHeaderAllowed(layoutArg, beforeArg)) {
      val returnSpace = beforeArg.tables.nth(5).nth(0).nth(2)
      val returnIndex = beforeArg.tables.nth(5).nth(0).nth(3)
      beforeArg.root.nth(4) == 0 and beforeArg.root.nth(5) == 1
        and pAllZero(beforeArg.root.slice(41, 45))
        and beforeArg.tables.nth(4).foldl(true, (emptySoFar, frameRow) => emptySoFar and pAllZero(frameRow))
        and (returnSpace == 0 or returnSpace == 1) and returnIndex >= 0
        and returnIndex < (if (returnSpace == 0) programArg.root.nth(14) else beforeArg.root.nth(0))
        and beforeArg.tables.nth(5).indices().forall(controlIndex =>
          beforeArg.tables.nth(5).nth(controlIndex).indices().forall(columnIndex =>
            beforeArg.tables.nth(5).nth(controlIndex).nth(columnIndex) ==
              (if (controlIndex == 0 and columnIndex == 0) 1
               else if (controlIndex == 0 and columnIndex == 2) returnSpace
               else if (controlIndex == 0 and columnIndex == 3) returnIndex else 0)))
        // Intentionally no positive-fuel predicate: A80 charges zero.
    } else false
  pure def pPublicationNext(beforeArg: PImage): PImage = {
    val returnSpace = beforeArg.tables.nth(5).nth(0).nth(2)
    val returnIndex = beforeArg.tables.nth(5).nth(0).nth(3)
    val afterRoot = pReplace(beforeArg.root.slice(0, 41), 5, 0)
      .concat(List(1,returnSpace,returnIndex,0))
    val afterControls = beforeArg.tables.nth(5).foldl(List(),
      (writtenRows, _) => writtenRows.append(pZeroRow(36)))
    {root: afterRoot, tables: beforeArg.tables.slice(0, 5).append(afterControls)}
  }
  pure def pLiteralRelation(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, afterArg: PImage, envelopeArg: PEnvelope): bool =
    if (pLiteralEnabled(layoutArg, programArg, beforeArg) and pStateShape(layoutArg, afterArg))
      envelopeArg == pLiteralEnvelope and afterArg == pLiteralNext(programArg, beforeArg)
    else false
  pure def pPublicationRelation(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, afterArg: PImage, envelopeArg: PEnvelope): bool =
    if (pPublicationEnabled(layoutArg, programArg, beforeArg) and pStateShape(layoutArg, afterArg))
      envelopeArg == pPublicationEnvelope and afterArg == pPublicationNext(beforeArg)
    else false
  pure def pPairStartAllowed(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, contextArg: PRow): bool =
    if (pLiteralEnabled(layoutArg, programArg, beforeArg))
      pContextAllowed(contextArg) and beforeArg.root.nth(4) == 0
        and beforeArg.tables.nth(4).foldl(true, (emptySoFar, frameRow) => emptySoFar and pAllZero(frameRow))
    else false
  // Current complete composer fixes Machine::new entry/selector0. Do not generalize it.
  pure def pCompleteStartAllowed(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, contextArg: PRow): bool =
    if (pPairStartAllowed(layoutArg, programArg, beforeArg, contextArg))
      programArg.root.nth(0) == 0 and beforeArg.tables.nth(5).nth(0).nth(1) == 0
        and pAllZero(beforeArg.root.slice(0, 5)) and pAllZero(beforeArg.root.slice(6, 8))
        and beforeArg.root.slice(9, 25) == beforeArg.root.slice(25, 41)
        and beforeArg.tables.foldl(true, (emptyTables, tableRows) => emptyTables
          and tableRows.foldl(true, (emptyRows, physicalRow) => emptyRows and pAllZero(physicalRow)))
    else false
  pure def pPairJoin(layoutArg: PLayout, boundProgram: PImage,
      contextArg: PRow, leftChild: PChild, rightChild: PChild): bool =
    pContextAllowed(contextArg) and leftChild.context == contextArg and rightChild.context == contextArg
      and leftChild.program == boundProgram and rightChild.program == boundProgram
      and leftChild.after == rightChild.before
      and pLiteralRelation(layoutArg, boundProgram, leftChild.before, leftChild.after, leftChild.envelope)
      and pPublicationRelation(layoutArg, boundProgram, rightChild.before, rightChild.after, rightChild.envelope)
  pure def pFlatten(imageArg: PImage): PRow = imageArg.tables.foldl(imageArg.root,
    (physicalCells, tableRows) => tableRows.foldl(physicalCells,
      (tableCells, physicalRow) => tableCells.concat(physicalRow)))
  pure def pStatement(contextArg: PRow, programArg: PImage, beforeArg: PImage,
      middleArg: PImage, doneArg: PImage, firstEnvelope: PEnvelope, secondEnvelope: PEnvelope): PRow =
    contextArg.concat(pFlatten(programArg)).concat(pFlatten(beforeArg))
      .concat(pFlatten(middleArg)).concat(pFlatten(doneArg)).concat(List(
        firstEnvelope.ruleTag,firstEnvelope.charge,secondEnvelope.ruleTag,secondEnvelope.charge))

  var reference: PExecution
  action pInitPair(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, contextArg: PRow): bool = all {
    pPairStartAllowed(layoutArg, programArg, beforeArg, contextArg),
    reference' = {layout: layoutArg, program: programArg, context: contextArg,
      states: List(beforeArg), envelopes: List()},
  }
  action pStepLiteral = all {
    reference.states.length() == 1 and reference.envelopes.length() == 0,
    pLiteralEnabled(reference.layout, reference.program, reference.states.nth(0)),
    reference' = {...reference,
      states: reference.states.append(pLiteralNext(reference.program, reference.states.nth(0))),
      envelopes: reference.envelopes.append(pLiteralEnvelope)},
  }
  action pStepPublication = all {
    reference.states.length() == 2 and reference.envelopes == List(pLiteralEnvelope),
    pPublicationEnabled(reference.layout, reference.program, reference.states.nth(1)),
    reference' = {...reference,
      states: reference.states.append(pPublicationNext(reference.states.nth(1))),
      envelopes: reference.envelopes.append(pPublicationEnvelope)},
  }
  action pInitPublication(layoutArg: PLayout, programArg: PImage,
      beforeArg: PImage, contextArg: PRow): bool = all {
    pContextAllowed(contextArg), pPublicationEnabled(layoutArg, programArg, beforeArg),
    reference' = {layout: layoutArg, program: programArg, context: contextArg,
      states: List(beforeArg), envelopes: List()},
  }
  action pPublishOnly = all {
    reference.states.length() == 1 and reference.envelopes.length() == 0,
    pPublicationEnabled(reference.layout, reference.program, reference.states.nth(0)),
    reference' = {...reference,
      states: reference.states.append(pPublicationNext(reference.states.nth(0))),
      envelopes: reference.envelopes.append(pPublicationEnvelope)},
  }
}
