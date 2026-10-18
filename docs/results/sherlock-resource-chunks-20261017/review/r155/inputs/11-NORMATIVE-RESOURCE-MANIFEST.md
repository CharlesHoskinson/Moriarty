WitnessPlan = {planId,operationManifestId,layoutVersion,inputBindings,
               nodes:[{nodeId,operationId,typedInputs,typedOutputs,dependencies,lastUse}],
               stableTopologicalOrder,outputBindings,slotCounts,programKind}
ConstraintPlan = {planId,operationManifestId,layoutVersion,fieldId,
                  orderedPublicInputs,orderedPrivateInputs,orderedOutputs,
                  components:[{componentId,operationId,sourceLocator,parameters}],
                  requiredCapabilities,omissions}
BackendCapabilities = {backendId,sourceMembership,fieldId,formats,
                       constraintOperations,witnessOperations,
                       lookup,interaction,customGates,structuredWitnessIR,nativeClosures}
ResourceManifest = {referenceScheduleId,ruleInventoryDigest,programImageId,
  layoutVersion,bounds,loweringDigest,frameBytesByFunction,helperScratchBounds,
  referenceWork,helperWork,helperVisits,physicalEvents,logicalFramePeak,physicalFrameBytesPeak,
  activeCells,paddedCells,preparationBytes,serializationBytes,
  hostTime,hostPeakMemory,ioBytes,circuitRows?,degree?,keyBytes?,proofBytes?,
  proverTime?,verifierTime?,targetFees?,measurementScope,unknownMetrics}
compileStep(layout,componentInventory)
  -> Result<(ConstraintPlan,WitnessPlan),UnsupportedFeature>
evaluateWitness(plan:WitnessPlan,bindings,cacheScope:None | ExactRunScope)
  -> Result<(CandidateRows,HostDiagnostics),WitnessFailure>
emitConstraints(plan:ConstraintPlan,capabilities:BackendCapabilities)
