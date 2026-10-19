module independentCore2BoundaryPort {
  import financialBoundary.* from "./financialBoundary"
  import independentCore2BoundarySeed as S from "./independentCore2BoundarySeed"
  // The independent seed is supplied by Root from its pre-execution freeze.
  // No actual publication or source-author fixture supplies this carrier.
  action init: bool = initInvocation(S::suppliedInvocation)
  action publishOnly: bool = publishStep
  val captureInv = allInv and normalDomain(machine.current)
  val publishedWitness = machine.phase == Published
}
