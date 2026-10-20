module doctorwatson_external_coverage_test {
  import doctorwatson_external_coverage as driver from "./doctorwatson_external_coverage"
  run directedWalkCompletesTest = driver::init.then(53.reps(i => driver::step))
    .expect(driver::directedFinished and driver::directedInvariant)
}
