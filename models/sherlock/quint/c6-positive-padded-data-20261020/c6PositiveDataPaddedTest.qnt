module c6PositiveDataPaddedTest {
  import c6PhysicalTypes.* from "./c6PhysicalTypes"
  import c6PositiveDataTypes.* from "./c6PositiveDataTypes"
  import c6PositiveDataInputs.* from "./c6PositiveDataInputs"
  import c6PositiveDataPadded.* from "./c6PositiveDataPadded"
  // Data-only independent fixture is opaque-bound by Root after author freeze.
  // No model/evaluator output supplies this expected object.
  import independentC6PositiveDataExpected.expectedObservation from "./independentC6PositiveDataExpected"

  run c6PositivePaddedDataWholeObservationTest = all {
    inputGate == 1,
    positivePaddedDataExact(inputCaps,inputTables,expectedObservation),
  }
}
