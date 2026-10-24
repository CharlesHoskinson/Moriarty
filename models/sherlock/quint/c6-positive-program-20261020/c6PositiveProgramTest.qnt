module c6PositiveProgramTest {
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveProgramInputs.* from "./c6PositiveProgramInputs"
  import c6PositiveProgramObservation.* from "./c6PositiveProgramObservation"
  import independentC6ProgramExpected.* from "./independentC6ProgramExpected"

  run c6PositiveProgramCompleteObservationsTest = all {
    c6PWholeObservationsExact(c6PInputCases,expectedObservations),
  }
  run c6PositiveProgramCountRowTextContinuityTest = all {
    c6PAllCursorLaws(c6PInputCases),
  }
}
