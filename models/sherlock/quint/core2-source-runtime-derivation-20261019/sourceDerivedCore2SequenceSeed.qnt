module sourceDerivedCore2SequenceSeed {
  import financialBoundary.* from "./financialBoundary"
  import sourceCore2ReadAdd.* from "./sourceCore2ReadAdd"
  import b18ReadAddSourceProgram as P from "./b18ReadAddSourceProgram"
  import independentCore2SequenceSeed as H from "./independentCore2SequenceSeed"
  // Keep all original identity/plan/scenario/event/continuation/history inputs.
  // Projection has no runtime field, and the evaluator never reads H runtime.
  pure val emptySourceRequests: List[SourceRequest] = List()
  pure val selectedSourceRequests: List[SourceRequest] =
    H::suppliedInvocations.foldl(emptySourceRequests,
      (acc, invocation) => acc.append(sourceRequest(invocation)))
  pure val derivedQueue = deriveSourceQueue(P::selectedSourceProgram, selectedSourceRequests)
  pure val sourceDerivationSupported = match derivedQueue {
    | SourceInvocations(_) => true
    | OutsideSourceQueue => false
  }
  // Empty only denotes an out-of-fragment model input, never a production Error.
  // The port must reject an empty queue/false sourceDerivationSupported; there is
  // no successful truncated-prefix or fabricated failure RuntimeEvidence.
  pure val suppliedInvocations: List[Invocation] = match derivedQueue {
    | SourceInvocations(rows) => rows
    | OutsideSourceQueue => List()
  }
}
