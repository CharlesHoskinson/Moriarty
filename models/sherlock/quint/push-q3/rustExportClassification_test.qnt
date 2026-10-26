module rustExportClassification_test {
  import rustExportClassification.* from "./rustExportClassification"
  action begin(input: Input): bool = observation' = {classified: false, input: input, operation: ArenaRead}
  run scopeReturnIsMisclassifiedFromOuterMulTest = begin(pendingMul).then(classify)
    .expect(observation.operation == CheckedMul and referenceOperation(observation.input) == FrameRestore
      and misclassifiedPendingFrame and sourceInv)
  run currentArithmeticIsClassifiedNormallyTest = begin(activeMul).then(classify)
    .expect(observation.operation == CheckedMul and belongsToActiveStep and sourceInv)
  run typedReturnSpecialCaseIsCorrectTest = begin(typedReturn).then(classify)
    .expect(observation.operation == FrameRestore and belongsToActiveStep and sourceInv)
}
