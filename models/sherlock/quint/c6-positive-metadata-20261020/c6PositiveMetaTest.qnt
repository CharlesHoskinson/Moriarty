module c6PositiveMetaTest {
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveMetaInputs.* from "./c6PositiveMetaInputs"
  import c6PositiveMetaObservation.* from "./c6PositiveMetaObservation"
  import independentC6MetaExpected.* from "./independentC6MetaExpected"

  run c6PositiveMetaCompleteObservationsTest = all {
    c6MWholeExpectedRootExact(c6MInputCases,expectedRoot),
  }
  run c6PositiveMetaRoundtripAndSpanLawsTest = all {
    c6MInputCases.foldl(true,(priorArg,inputArg) =>
      priorArg and c6MPositiveRoundtripLaws(inputArg.metadata)),
  }
}
