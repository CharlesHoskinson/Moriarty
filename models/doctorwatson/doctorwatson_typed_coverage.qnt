// Deterministic action witness driver; no replacement of the original47-step driver.
module doctorwatson_typed_coverage {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ) as machine from "./doctorwatson"

  var cursor: int
  action init = all { machine::init, cursor' = 0 }
  action step = all {
    cursor >= 0 and cursor < 41,
    any {
      all { cursor == 0, machine::configureStage(3, 1, machine::NATIVE_ACCEPTED, 4) },
      all { cursor == 1, machine::claim(2, 2) },
      all { cursor == 2, machine::freeze(2, 3, true) },
      all { cursor == 3, machine::edit(2, 2, machine::Source) },
      all { cursor == 4, machine::submit(2) },
      all { cursor == 5, machine::startLocal(2) },
      all { cursor == 6, machine::checker(2, machine::jobToken(machine::state, 2), machine::CompleteMatch) },
      all { cursor == 7, machine::regressions(2, machine::jobToken(machine::state, 2), true) },
      all { cursor == 8, machine::review(2, 3, true) },
      all { cursor == 9, machine::merge(2) },
      all { cursor == 10, machine::claim(1, 1) },
      all { cursor == 11, machine::freeze(1, 3, true) },
      all { cursor == 12, machine::edit(1, 1, machine::Source) },
      all { cursor == 13, machine::submit(1) },
      all { cursor == 14, machine::startLocal(1) },
      all { cursor == 15, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 16, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 17, machine::review(1, 3, true) },
      all { cursor == 18, machine::merge(1) },
      all { cursor == 19, {
        val observation = machine::currentDeliveryObservation(machine::state, 1)
        val reviewed = machine::deliveryReview(observation, 3)
        machine::acceptDelivery(1, observation, reviewed, machine::deliveryAdoption(observation, reviewed))
      } },
      all { cursor == 20, machine::configureAdapter(1, machine::NativeAdapter, 1) },
      all { cursor == 21, machine::claim(1, 1) },
      all { cursor == 22, machine::freeze(1, 3, true) },
      all { cursor == 23, machine::edit(1, 1, machine::Source) },
      all { cursor == 24, machine::submit(1) },
      all { cursor == 25, machine::startLocal(1) },
      all { cursor == 26, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 27, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 28, machine::dispatch(1) },
      all { cursor == 29, machine::extObserve(1, 1, true) },
      all { cursor == 30, machine::cleanup(1) },
      all { cursor == 31, {
        val observation = machine::currentDeliveryObservation(machine::state, 1)
        val reviewed = machine::deliveryReview(observation, 3)
        machine::acceptDelivery(1, observation, reviewed, machine::deliveryAdoption(observation, reviewed))
      } },
      all { cursor == 32, machine::claim(3, 4) },
      all { cursor == 33, machine::freeze(3, 3, true) },
      all { cursor == 34, machine::edit(3, 4, machine::Source) },
      all { cursor == 35, machine::submit(3) },
      all { cursor == 36, machine::startLocal(3) },
      all { cursor == 37, machine::checker(3, machine::jobToken(machine::state, 3), machine::CompleteMatch) },
      all { cursor == 38, machine::regressions(3, machine::jobToken(machine::state, 3), true) },
      all { cursor == 39, machine::review(3, 3, true) },
      all { cursor == 40, machine::merge(3) },
    },
    cursor' = cursor + 1,
  }
  val sourceAcceptedWitness = machine::sourceAcceptedWitness
  val nativeObservedWitness = machine::nativeObservedWitness
  val nativeAcceptedWitness = machine::nativeAcceptedWitness
  val directedFinished = cursor == 41 and (machine::deliveryAcceptedRecorded(machine::state, 1) and machine::deliverySourceRecorded(machine::state, 3) and machine::readyDependencies(machine::state, 3) and machine::TASKS.forall(id => machine::externalDischarged(machine::state, id)) and machine::state.deliveryAcceptances.size() == 2)
  val directedInvariant = machine::workflowInvariant and cursor >= 0 and cursor <= 41

}
