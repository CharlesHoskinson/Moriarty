module nominalConstructionSharedObservation {
  // Actual full-state trace capture only. No expected values or assertion oracle.
  import nominalConstruction.* from "./nominalConstruction"
  import nominalConstructionSharedInputs as I from "./nominalConstructionSharedInputs"
  action initCase0: bool = machine' = I::sharedCase0Initial
  action initCase1: bool = machine' = I::sharedCase1Initial
  action initCase2: bool = machine' = I::sharedCase2Initial
  action initCase3: bool = machine' = I::sharedCase3Initial
  run sharedCase0ObservationTest = initCase0.then(96.reps(i => step))
  run sharedCase1ObservationTest = initCase1.then(96.reps(i => step))
  run sharedCase2ObservationTest = initCase2.then(96.reps(i => step))
  run sharedCase3ObservationTest = initCase3.then(96.reps(i => step))
}
