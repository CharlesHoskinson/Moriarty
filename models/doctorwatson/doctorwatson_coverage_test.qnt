// Unlike sampled witnesses, this assertion fails if any directed step is disabled.
module doctorwatson_coverage_test {
  import doctorwatson_coverage as driver from "./doctorwatson_coverage"

  run directedWalkCompletesTest = driver::init
    .then(47.reps(i => driver::step))
    .expect(driver::directedFinished)
}
