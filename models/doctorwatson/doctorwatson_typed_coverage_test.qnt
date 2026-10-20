module doctorwatson_typed_coverage_test {
  import doctorwatson_typed_coverage as driver from "./doctorwatson_typed_coverage"
  run directedWalkCompletesTest = driver::init.then(41.reps(i => driver::step))
    .expect(driver::directedFinished and driver::directedInvariant)
}
