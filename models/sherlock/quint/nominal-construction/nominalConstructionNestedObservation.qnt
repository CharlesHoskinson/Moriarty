module nominalConstructionNestedObservation {
  import nominalConstruction.* from "./nominalConstruction"
  import nominalConstructionNestedInputs as I from "./nominalConstructionNestedInputs"
  action initCase0: bool = machine' = I::patternCase0Initial
  action initCase1: bool = machine' = I::patternCase1Initial
  action initCase2: bool = machine' = I::patternCase2Initial
  action initCase3: bool = machine' = I::patternCase3Initial
  action init: bool = initCase0
  run patternCase0ObservationTest = initCase0.then(256.reps(i => step))
  run patternCase1ObservationTest = initCase1.then(256.reps(i => step))
  run patternCase2ObservationTest = initCase2.then(256.reps(i => step))
  run patternCase3ObservationTest = initCase3.then(256.reps(i => step))
}
