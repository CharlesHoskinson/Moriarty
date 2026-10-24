module q11ActualNominalBindingTest {
  import q11NominalIdentity.* from "./q11NominalIdentity"
  import q11NominalIdentityExpected.* from "./q11NominalIdentityExpected"
  import q11NominalIdentityInputs.* from "./q11NominalIdentityInputs"
  import q11ActualMapQueueReturned.* from "./q11ActualMapQueueReturned"
  import q11ActualMirrorArborReturned.* from "./q11ActualMirrorArborReturned"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  import nominalPublicSimulation.* from "./nominalPublicSimulation"

  // Frozen before capture payload transformation from the independent logical
  // returns (7 and 10 tokens) and the existing q11 source-read budget of 256.
  pure val actualNominalReadBudget = 256

  pure def actualNominalRootMatches(supplied:C6WholeHost,root:C6Ref,
      frozenView:C6PProgram,independentReturn:Value):bool = {
    val view=supplied.admission.view
    val image=supplied.image
    val evidence=q11ObserveEvidence(view,image,root,actualNominalReadBudget)
    val zero=q11ObserveEvidence(view,image,root,0)
    and {
      view==frozenView,
      q11EvidencePreserved(evidence,view,image),
      q11EvidencePreserved(zero,view,image),
      evidence.root==root,
      zero.root==root,
      match image.outcome {
        | C6Halted(returnedRoot) => evidence.root==returnedRoot
        | _ => false
      },
      evidence.structural_read.pending==List(),
      evidence.structural_read.tokens==independentReturn,
      zero.structural_read.tokens==List(),
      zero.structural_read.pending==List(root),
    }
  }

  // Inputs are the actual selected captures. Expectations remain the earlier
  // independently source-derived trees. No expected token or runtime root ID
  // is generated from a captured image. The prior five tests are not replayed.
  run q11ActualMapQueueFinalRootTest =
    actualNominalRootMatches(mapActualReturned,mapActualRoot,
      q11MapView,q11MapExpectedReturn)

  run q11ActualMirrorArborFinalRootTest =
    actualNominalRootMatches(mirrorActualReturned,mirrorActualRoot,
      q11MirrorView,q11MirrorExpectedReturn)
}
