module rejection_pipeline_test {
  import rejection_harness.* from "./rejection_harness"
  import execution_harness.* from "./execution_harness"
  run missingProofPipelineTest = rejectionInit.then(proposeSettlement).then(rejectMissing)
    .expect(missingRejected and rejectionSafety)
  run staleCommitPipelineTest = rejectionInit.then(proposeSettlement).then(verifySettlement)
    .then(advanceAnchor).then(rejectStale).expect(staleRejected and rejectionSafety)
}
