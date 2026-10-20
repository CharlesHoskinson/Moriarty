module c6PositiveHostTest {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveHostInputs.* from "./c6PositiveHostInputs"
  import c6PositiveHostObservation.* from "./c6PositiveHostObservation"
  import independentC6WholeHostExpected.* from "./independentC6WholeHostExpected"
  // Only the complete-root test consumes independently owned opaque DATA.
  run c6PositiveHostCompleteObservationsTest =
    assert(c6HostObserveRoot(suppliedHostCases) == expectedRoot)
  run c6PositiveHostRoundtripCompositionLawsTest = assert(and {
    suppliedHostCases.foldl(true,(okArg,c) => okArg and c6HostCompositionLaws(c)),
    c6HostPresenceLaws(suppliedHostCases.nth(0),suppliedHostCases.nth(1)),
  })
}
