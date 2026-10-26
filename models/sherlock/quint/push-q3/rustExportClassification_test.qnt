module rustExportClassification_test {
  import rustExportClassification.* from "./rustExportClassification"
  action begin(input: Input): bool = observation' = {classified: false, input: input, operation: ArenaRead}
  run scopeReturnIsMisclassifiedFromOuterMulTest = begin(pendingMul).then(classify)
    .expect(observation.operation == CheckedMul and referenceOperation(observation.input) == FrameRestore
      and misclassifiedPendingFrame and sourceInv)
  run returnLeftSchedulesRightOperandInsteadOfMultiplyingTest =
    begin({returning: true, frames: List(LeftMul), rule: 149}).then(classify)
      .expect(observation.operation == CheckedMul and referenceOperation(observation.input) == ArenaRead
        and misclassifiedPendingFrame and sourceInv)
  run currentArithmeticIsClassifiedNormallyTest = begin(activeMul).then(classify)
    .expect(observation.operation == CheckedMul and belongsToActiveStep and sourceInv)
  run typedReturnSpecialCaseIsCorrectTest = begin(typedReturn).then(classify)
    .expect(observation.operation == FrameRestore and belongsToActiveStep and sourceInv)
  run repairedScopeReturnUsesRestorationTest = begin(pendingMul).then(classifyRepaired)
    .expect(repairedPendingFrame and belongsToActiveStep and repairedInv)
  run repairedReturnLeftDoesNotClaimMultiplicationTest =
    begin({returning: true, frames: List(LeftMul), rule: 149}).then(classifyRepaired)
      .expect(observation.operation == ArenaRead and belongsToActiveStep and repairedInv)
  run repairedActiveMulStillUsesMulTest = begin(activeMul).then(classifyRepaired)
    .expect(observation.operation == CheckedMul and repairedInv)
  run repairedTypedReturnStillUsesRestorationTest = begin(typedReturn).then(classifyRepaired)
    .expect(observation.operation == FrameRestore and repairedInv)
}
