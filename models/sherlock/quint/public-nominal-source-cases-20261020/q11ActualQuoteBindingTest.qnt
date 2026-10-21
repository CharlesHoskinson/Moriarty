module q11ActualQuoteBindingTest {
  import q11NominalIdentity.* from "./q11NominalIdentity"
  import q11ActualQuoteReturned.* from "./q11ActualQuoteReturned"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"

  // P1/P2: full records/root from the already frozen positive input; no expected
  // token/value/index generated from the actual image is asserted here.
  // P3: zero read budget retains the entire unvisited root partition.
  run q11ActualQuoteWholeRetentionAndResidualInterfaceTest = {
    val suppliedView=actualReturned.admission.view
    val suppliedImage=actualReturned.image
    val observedEvidence=q11ObserveEvidence(suppliedView,suppliedImage,actualRoot,256)
    val zeroBudgetEvidence=q11ObserveEvidence(suppliedView,suppliedImage,actualRoot,0)
    and {
      q11EvidencePreserved(observedEvidence,suppliedView,suppliedImage),
      q11EvidencePreserved(zeroBudgetEvidence,suppliedView,suppliedImage),
      observedEvidence.root==actualRoot,
      zeroBudgetEvidence.root==actualRoot,
      match suppliedImage.outcome {
        | C6Halted(returnedRoot) => observedEvidence.root==returnedRoot
        | _ => false
      },
      zeroBudgetEvidence.structural_read.tokens==List(),
      zeroBudgetEvidence.structural_read.pending==List(actualRoot),
    }
  }
}
