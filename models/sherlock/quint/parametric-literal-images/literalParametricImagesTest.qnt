module literalParametricImagesTest {
  import literalParametricImages.* from "./literalParametricImages"
  import literalParametricInputs.* from "./literalParametricInputs"

  run wideUIntNonzeroSelectorTest =
    pInitPair(pExampleLayout,pExampleProgram,pWideBefore,pAlpha)
      .expect(pLiteralEnabled(pExampleLayout,pExampleProgram,pWideBefore)
        and not(pCompleteStartAllowed(pExampleLayout,pExampleProgram,pWideBefore,pAlpha))
        and pSelectedConstant(pExampleProgram,pWideBefore) == pWideUIntNode
        and pSelectedConstant(pExampleProgram,pWideBefore).nth(18) == 128
        and pSelectedConstant(pExampleProgram,pWideBefore).slice(3,19) == pWideBytes
        and pFuelValue(pWideBytes) == 170141183460469231731687303715884105988)
      .then(pStepLiteral)
      .expect(reference.states == List(pWideBefore,pWideMiddle)
        and reference.envelopes == List({ruleTag:3,charge:1})
        and reference.states.nth(1).tables.nth(5).nth(0).nth(3) == 2
        and reference.states.nth(1).root.slice(9,25) == p255Bytes
        and reference.states.nth(1).tables.slice(0,5) == pPreservedArenas)
      .then(pStepPublication)
      .expect(reference.states == List(pWideBefore,pWideMiddle,pWideDone)
        and reference.envelopes == List({ruleTag:3,charge:1},{ruleTag:0,charge:0})
        and reference.context == pAlpha and reference.program == pExampleProgram
        and reference.states.nth(2).root.slice(9,25) == p255Bytes
        and pPairJoin(pExampleLayout,pExampleProgram,pAlpha,pLeftChild(pAlpha),pRightChild(pAlpha))
        and pFlatten(pExampleProgram).length() == 205 and pFlatten(pWideBefore).length() == 277
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).length() == 1042
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).nth(44) == 2
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).nth(377) == 1
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).nth(656) == 2
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).nth(804) == 2
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).slice(1038,1042) == List(3,1,0,0))

  run boolTrueNonzeroSelectorZeroFuelPublicationTest =
    pInitPair(pExampleLayout,pExampleProgram,pBoolBefore,pBeta)
      .expect(pSelectedConstant(pExampleProgram,pBoolBefore) == pBoolTrueNode
        and pSelectedConstant(pExampleProgram,pBoolBefore).nth(19) == 1
        and pBoolBefore.tables.nth(5).nth(0).nth(1) == 2)
      .then(pStepLiteral)
      .expect(reference.states == List(pBoolBefore,pBoolMiddle)
        and pPublicationEnabled(pExampleLayout,pExampleProgram,reference.states.nth(1))
        and reference.states.nth(1).root.slice(9,25) == pZeroBytes)
      .then(pStepPublication)
      .expect(reference.states == List(pBoolBefore,pBoolMiddle,pBoolDone)
        and reference.envelopes == List({ruleTag:3,charge:1},{ruleTag:0,charge:0})
        and reference.states.nth(2).root.slice(9,25) == pZeroBytes
        and reference.states.nth(2).root.slice(41,45) == List(1,0,1,0)
        and reference.context == pBeta and reference.program == pExampleProgram)

  run entryZeroNonzeroConstantCompleteStartTest =
    pInitPair(pExampleLayout,pEntryConstant2Program,pCompleteInit,pGamma)
      .expect(pCompleteStartAllowed(pExampleLayout,pEntryConstant2Program,pCompleteInit,pGamma)
        and pEntryConstant2Program.root.nth(0) == 0
        and pCompleteInit.tables.nth(5).nth(0).nth(1) == 0
        and pSelectedConstant(pEntryConstant2Program,pCompleteInit) == pWideUIntNode)
      .then(pStepLiteral)
      .expect(reference.states == List(pCompleteInit,pEntryMiddle))
      .then(pStepPublication)
      .expect(reference.states == List(pCompleteInit,pEntryMiddle,pEntryDone)
        and reference.envelopes == List({ruleTag:3,charge:1},{ruleTag:0,charge:0})
        and reference.states.nth(2).root.slice(41,45) == List(1,0,2,0)
        and reference.states.nth(2).root.slice(9,25) == pZeroBytes
        and reference.context == pGamma)

  run opaqueCompleteContextBindingTest =
    pInitPair(pExampleLayout,pExampleProgram,pWideBefore,pAlpha)
      .expect(pAlpha != List(17,29) and pBeta != pAlpha
        and pContextAllowed(pAlpha) and pContextAllowed(pBeta) and pContextAllowed(pGamma)
        and not(pContextAllowed(List(101))) and not(pContextAllowed(List(0,P_NATIVE_BOUND)))
        and pPairJoin(pExampleLayout,pExampleProgram,pAlpha,pLeftChild(pAlpha),pRightChild(pAlpha))
        and pPairJoin(pExampleLayout,pExampleProgram,pBeta,pLeftChild(pBeta),pRightChild(pBeta))
        and not(pPairJoin(pExampleLayout,pExampleProgram,pAlpha,pLeftChild(pAlpha),pRightChild(pBeta)))
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).slice(0,2) == pAlpha
        and pStatement(pBeta,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).slice(0,2) == pBeta
        and pStatement(pAlpha,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
          {ruleTag:3,charge:1},{ruleTag:0,charge:0}).slice(2,1042) ==
            pStatement(pBeta,pExampleProgram,pWideBefore,pWideMiddle,pWideDone,
              {ruleTag:3,charge:1},{ruleTag:0,charge:0}).slice(2,1042))

  run selectorAndCopiedReferenceRefusalTest =
    pInitPair(pExampleLayout,pExampleProgram,pWideBefore,pAlpha)
      .expect(pLiteralEnabled(pExampleLayout,pExampleProgram,pWideBefore)
        and pStateShape(pExampleLayout,pMutantSelectorAtActiveLength)
        and pProgramShape(pExampleLayout,pMutantOccurrenceZero)
        and pProgramShape(pExampleLayout,pMutantConstantAtActiveLength)
        and not(pLiteralEnabled(pExampleLayout,pExampleProgram,pMutantSelectorAtActiveLength))
        and not(pLiteralEnabled(pExampleLayout,pMutantOccurrenceZero,pWideBefore))
        and not(pLiteralEnabled(pExampleLayout,pMutantConstantAtActiveLength,pWideBefore))
        and pLiteralRelation(pExampleLayout,pExampleProgram,pWideBefore,pWideMiddle,{ruleTag:3,charge:1})
        and not(pLiteralRelation(pExampleLayout,pExampleProgram,pWideBefore,pMutantMiddleConstantZero,
          {ruleTag:3,charge:1}))
        and pPublicationRelation(pExampleLayout,pExampleProgram,pWideMiddle,pWideDone,{ruleTag:0,charge:0})
        and not(pPublicationRelation(pExampleLayout,pExampleProgram,pWideMiddle,pMutantDoneConstantZero,
          {ruleTag:0,charge:0})))

  run completePhysicalPaddingRefusalTest =
    pInitPair(pExampleLayout,pExampleProgram,pWideBefore,pAlpha)
      .expect(pPairJoin(pExampleLayout,pExampleProgram,pAlpha,pLeftChild(pAlpha),pRightChild(pAlpha))
        and pProgramShape(pExampleLayout,pMutantProgramTypePadding)
        and not(pPairJoin(pExampleLayout,pExampleProgram,pAlpha,
          {program:pMutantProgramTypePadding,context:pAlpha,before:pWideBefore,
            after:pWideMiddle,envelope:{ruleTag:3,charge:1}},pRightChild(pAlpha)))
        and pStateShape(pExampleLayout,pMutantMiddleDataPadding)
        and not(pLiteralRelation(pExampleLayout,pExampleProgram,pWideBefore,pMutantMiddleDataPadding,
          {ruleTag:3,charge:1}))
        and pStateShape(pExampleLayout,pMutantDoneControlPadding)
        and not(pPublicationRelation(pExampleLayout,pExampleProgram,pWideMiddle,pMutantDoneControlPadding,
          {ruleTag:0,charge:0}))
        and not(pLiteralRelation(pExampleLayout,pExampleProgram,pWideBefore,pMutantMiddleStackLimit,
          {ruleTag:3,charge:1}))
        and pWideMiddle.tables.nth(0).nth(0).nth(3) == 9
        and pWideMiddle.tables.nth(3).nth(0) == List(1,1))

  run dataReturnZeroFuelPublicationTest =
    pInitPublication(pExampleLayout,pExampleProgram,pDataReturnBefore,pGamma)
      .expect(pPublicationEnabled(pExampleLayout,pExampleProgram,pDataReturnBefore)
        and not(pPublicationEnabled(pExampleLayout,pExampleProgram,pMutantDataAtActiveLength))
        and not(pPublicationEnabled(pExampleLayout,pExampleProgram,pMutantConstantReturnAtActiveLength))
        and pDataReturnBefore.root.slice(9,25) == pZeroBytes
        and pDataReturnBefore.tables.nth(5).nth(0).slice(2,4) == List(1,1))
      .then(pPublishOnly)
      .expect(reference.states == List(pDataReturnBefore,pDataReturnDone)
        and reference.envelopes == List({ruleTag:0,charge:0})
        and reference.states.nth(1).root.slice(9,25) == pZeroBytes
        and reference.states.nth(1).root.slice(41,45) == List(1,1,1,0)
        and reference.states.nth(1).tables.slice(0,5) == pPreservedArenas
        and reference.context == pGamma and reference.program == pExampleProgram)

  run nonzeroSelectorChargedZeroFuelRefusalTest =
    pInitPair(pExampleLayout,pExampleProgram,pWideBefore,pAlpha)
      .expect(pStateShape(pExampleLayout,pZeroBefore)
        and pZeroBefore.tables.nth(5).nth(0).nth(1) == 1
        and pZeroBefore.root.slice(9,25) == pZeroBytes
        and not(pLiteralEnabled(pExampleLayout,pExampleProgram,pZeroBefore))
        and not(pPairStartAllowed(pExampleLayout,pExampleProgram,pZeroBefore,pBeta))
        and reference.states == List(pWideBefore) and reference.envelopes == List())
      .then(pInitPair(pExampleLayout,pExampleProgram,pZeroBefore,pBeta).fail())
}
