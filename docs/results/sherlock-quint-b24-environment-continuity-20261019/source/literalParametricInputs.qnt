module literalParametricInputs {
  import literalParametricImages.* from "./literalParametricImages"
  // Same-author mathematical fixtures, specified before any B22 execution.
  // Contexts are supplied opaque complete-layout limb atoms, never A102 digests.
  pure val pExampleLayout: PLayout = {
    programRows: List(0,0,3,0,4,0,0,0,0,0,6,5,7,4,0),
    stateRows: List(3,3,2,4,2,3), stackLimit: 2,
  }
  pure val pAlpha: PRow = List(101,202)
  pure val pBeta: PRow = List(303,404)
  pure val pGamma: PRow = List(340282366920938463463374607431768211455,0)
  pure val pZeroBytes: PRow = List(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val pOneBytes: PRow = List(1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val p256Bytes: PRow = List(0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val p255Bytes: PRow = List(255,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val p512Bytes: PRow = List(0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0)
  pure val pWideBytes: PRow = List(4,1,0,0,0,0,0,0,0,0,0,0,0,0,0,128)
  pure def pRowsOfZeros(rowCountArg: int, rowWidthArg: int): List[PRow] =
    if (rowCountArg == 0) List() else range(0,rowCountArg).foldl(List(),
      (fixtureRows, _) => fixtureRows.append(pZeroRow(rowWidthArg)))
  pure def pUIntNode(bytesArg: PRow): PRow = List(0,0,0).concat(bytesArg).concat(List(0,0,0))
  pure val pBoolTrueNode: PRow = List(1,1,0).concat(pZeroBytes).concat(List(1,0,0))
  pure val pDecoyUInt7Node: PRow = pUIntNode(List(7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0))
  pure val pWideUIntNode: PRow = pUIntNode(pWideBytes)
  pure def pLiteralExpression(expressionIndexArg: int, constantIndexArg: int): PRow =
    List(0,expressionIndexArg,0,constantIndexArg,0,0,0,0,0,0,0)
  pure val pExampleProgram: PImage = {
    root: List(0,0,0,3,0,3,0,0,0,0,0,4,4,5,3,0),
    tables: List(
      List(), List(), pRowsOfZeros(3,4), List(),
      List(pLiteralExpression(0,0),pLiteralExpression(1,2),pLiteralExpression(2,1),pZeroRow(11)),
      List(), List(), List(), List(), List(),
      List(List(0,0,0),List(1,0,0),List(2,0,2),List(3,2,2),pZeroRow(3),pZeroRow(3)),
      List(List(2,0,0,0),List(2,1,0,2),List(3,0,2,1),List(3,1,3,2),pZeroRow(4)),
      List(List(0),List(2),List(0),List(3),List(3),List(0),List(0)),
      List(pDecoyUInt7Node,pBoolTrueNode,pWideUIntNode,pZeroRow(22)), List()),
  }
  pure def pSetTableRow(imageArg: PImage, tableArg: int, rowArg: int, cellsArg: PRow): PImage = {
    val changedTables = imageArg.tables.foldl({cursor:0,out:List()}, (tableAcc, originalTable) => {
      val changedRows = originalTable.foldl({cursor:0,out:List()}, (rowAcc, originalRow) => {
        cursor: rowAcc.cursor + 1,
        out: rowAcc.out.append(if (tableAcc.cursor == tableArg and rowAcc.cursor == rowArg)
          cellsArg else originalRow),
      })
      {cursor:tableAcc.cursor + 1,out:tableAcc.out.append(changedRows.out)}
    })
    {root:imageArg.root,tables:changedTables.out}
  }
  pure def pChangeImageCell(imageArg: PImage, tableArg: int, rowArg: int,
      columnArg: int, newCellArg: int): PImage = pSetTableRow(imageArg,tableArg,rowArg,
        pReplace(imageArg.tables.nth(tableArg).nth(rowArg),columnArg,newCellArg))
  pure val pEntryConstant2Program: PImage =
    pSetTableRow(pExampleProgram,4,0,pLiteralExpression(0,2))
  pure val pPreservedArenas: List[List[PRow]] = List(
    List(pUIntNode(List(9,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0)),pBoolTrueNode,pZeroRow(22)),
    pRowsOfZeros(3,2), pRowsOfZeros(2,5),
    List(List(1,1),List(0,0),List(0,0),List(0,0)), pRowsOfZeros(2,17))
  pure def pEvalRows(selectorArg: int): List[PRow] =
    List(pReplace(pZeroRow(36),1,selectorArg),pZeroRow(36),pZeroRow(36))
  pure def pReturnRows(spaceArg: int, indexArg: int): List[PRow] =
    List(List(1,0,spaceArg,indexArg).concat(pZeroRow(32)),pZeroRow(36),pZeroRow(36))
  pure def pStepBefore(selectorArg: int, remainingArg: PRow, initialArg: PRow): PImage = {
    root: List(2,0,0,1,0,1,0,0,2).concat(remainingArg).concat(initialArg).concat(List(0,0,0,0)),
    tables: pPreservedArenas.append(pEvalRows(selectorArg)),
  }
  pure val pWideBefore: PImage = pStepBefore(1,p256Bytes,p512Bytes)
  pure val pBoolBefore: PImage = pStepBefore(2,pOneBytes,pOneBytes)
  pure val pZeroBefore: PImage = pStepBefore(1,pZeroBytes,p512Bytes)
  pure val pEmptyArenas: List[List[PRow]] = List(
    pRowsOfZeros(3,22),pRowsOfZeros(3,2),pRowsOfZeros(2,5),pRowsOfZeros(4,2),pRowsOfZeros(2,17))
  pure val pCompleteInit: PImage = {
    root: List(0,0,0,0,0,1,0,0,2).concat(pOneBytes).concat(pOneBytes).concat(List(0,0,0,0)),
    tables: pEmptyArenas.append(pEvalRows(0)),
  }
  // Hand-specified post-images: neither transition nor byte subtraction under test is called.
  // Remaining fuel is an independently supplied exact byte list in each named case.
  pure def pHandMiddle(beforeArg: PImage, remainingArg: PRow, constantArg: int): PImage = {
    root: beforeArg.root.slice(0,9).concat(remainingArg).concat(beforeArg.root.slice(25,45)),
    tables: beforeArg.tables.slice(0,5).append(pReturnRows(0,constantArg)),
  }
  pure def pHandDone(beforeArg: PImage, spaceArg: int, indexArg: int): PImage = {
    root: beforeArg.root.slice(0,5).concat(List(0)).concat(beforeArg.root.slice(6,41))
      .concat(List(1,spaceArg,indexArg,0)),
    tables: beforeArg.tables.slice(0,5).append(pRowsOfZeros(3,36)),
  }
  pure val pWideMiddle: PImage = pHandMiddle(pWideBefore,p255Bytes,2)
  pure val pWideDone: PImage = pHandDone(pWideMiddle,0,2)
  pure val pBoolMiddle: PImage = pHandMiddle(pBoolBefore,pZeroBytes,1)
  pure val pBoolDone: PImage = pHandDone(pBoolMiddle,0,1)
  pure val pEntryMiddle: PImage = pHandMiddle(pCompleteInit,pZeroBytes,2)
  pure val pEntryDone: PImage = pHandDone(pEntryMiddle,0,2)
  pure val pDataReturnBefore: PImage = {
    root: pStepBefore(0,pZeroBytes,pOneBytes).root,
    tables: pPreservedArenas.append(pReturnRows(1,1)),
  }
  pure val pDataReturnDone: PImage = pHandDone(pDataReturnBefore,1,1)
  pure def pLeftChild(contextArg: PRow): PChild = {
    program: pExampleProgram, context: contextArg, before: pWideBefore,
    after: pWideMiddle, envelope: {ruleTag:3,charge:1},
  }
  pure def pRightChild(contextArg: PRow): PChild = {
    program: pExampleProgram, context: contextArg, before: pWideMiddle,
    after: pWideDone, envelope: {ruleTag:0,charge:0},
  }
  // Every negative is named in final source; none is derived from observed output.
  pure val pMutantSelectorAtActiveLength: PImage = pChangeImageCell(pWideBefore,5,0,1,3)
  pure val pMutantOccurrenceZero: PImage = pChangeImageCell(pExampleProgram,4,1,1,0)
  pure val pMutantConstantAtActiveLength: PImage = pChangeImageCell(pExampleProgram,4,1,3,3)
  pure val pMutantMiddleConstantZero: PImage = pHandMiddle(pWideBefore,p255Bytes,0)
  pure val pMutantDoneConstantZero: PImage = pHandDone(pWideMiddle,0,0)
  pure val pMutantProgramTypePadding: PImage = pChangeImageCell(pExampleProgram,10,5,0,1)
  pure val pMutantMiddleDataPadding: PImage = pChangeImageCell(pWideMiddle,0,2,3,1)
  pure val pMutantDoneControlPadding: PImage = pChangeImageCell(pWideDone,5,2,0,1)
  pure val pMutantMiddleStackLimit: PImage = {
    root: pReplace(pWideMiddle.root,8,1), tables: pWideMiddle.tables,
  }
  pure val pMutantDataAtActiveLength: PImage = {
    root: pDataReturnBefore.root, tables: pPreservedArenas.append(pReturnRows(1,2)),
  }
  pure val pMutantConstantReturnAtActiveLength: PImage = {
    root: pDataReturnBefore.root, tables: pPreservedArenas.append(pReturnRows(0,3)),
  }
}
