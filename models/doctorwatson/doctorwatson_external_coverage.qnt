// Deterministic action witness driver; no replacement of the original47-step driver.
module doctorwatson_external_coverage {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ) as machine from "./doctorwatson"

  var cursor: int
  action init = all { machine::init, cursor' = 0 }
  action step = all {
    cursor >= 0 and cursor < 53,
    any {
      all { cursor == 0, machine::configureStage(3, 1, machine::NATIVE_OBSERVED, 4) },
      all { cursor == 1, machine::configureStage(3, 2, machine::NATIVE_OBSERVED, 4) },
      all { cursor == 2, machine::configureAdapter(1, machine::NativeAdapter, 1) },
      all { cursor == 3, machine::configureAdapter(2, machine::NativeAdapter, 2) },
      all { cursor == 4, machine::claim(1, 1) },
      all { cursor == 5, machine::freeze(1, 3, true) },
      all { cursor == 6, machine::edit(1, 1, machine::Source) },
      all { cursor == 7, machine::submit(1) },
      all { cursor == 8, machine::startLocal(1) },
      all { cursor == 9, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 10, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 11, machine::dispatch(1) },
      all { cursor == 12, machine::extObserve(1, 1, true) },
      all { cursor == 13, machine::cleanup(1) },
      all { cursor == 14, machine::claim(2, 2) },
      all { cursor == 15, machine::freeze(2, 3, true) },
      all { cursor == 16, machine::edit(2, 2, machine::Source) },
      all { cursor == 17, machine::submit(2) },
      all { cursor == 18, machine::startLocal(2) },
      all { cursor == 19, machine::checker(2, machine::jobToken(machine::state, 2), machine::CompleteMatch) },
      all { cursor == 20, machine::regressions(2, machine::jobToken(machine::state, 2), true) },
      all { cursor == 21, machine::dispatch(2) },
      all { cursor == 22, machine::cancel(2, 2) },
      all { cursor == 23, machine::extObserve(2, 1, true) },
      all { cursor == 24, machine::cleanup(2) },
      all { cursor == 25, machine::edit(1, 1, machine::Source) },
      all { cursor == 26, machine::submit(1) },
      all { cursor == 27, machine::startLocal(1) },
      all { cursor == 28, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 29, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 30, machine::dispatch(1) },
      all { cursor == 31, machine::extUnknown(1, 2) },
      all { cursor == 32, machine::reconcile(1, machine::externalResult(machine::state, 1), machine::terminalEvidence(machine::externalToken(machine::state, 1), machine::ProcessNotStarted)) },
      all { cursor == 33, machine::edit(1, 1, machine::Source) },
      all { cursor == 34, machine::submit(1) },
      all { cursor == 35, machine::startLocal(1) },
      all { cursor == 36, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 37, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 38, machine::dispatch(1) },
      all { cursor == 39, machine::extUnknown(1, 3) },
      all { cursor == 40, machine::cleanup(1) },
      all { cursor == 41, machine::reconcile(1, machine::externalResult(machine::state, 1), machine::terminalEvidence(machine::externalToken(machine::state, 1), machine::ProcessExited)) },
      all { cursor == 42, machine::edit(1, 1, machine::Source) },
      all { cursor == 43, machine::submit(1) },
      all { cursor == 44, machine::startLocal(1) },
      all { cursor == 45, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 46, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 47, machine::dispatch(1) },
      all { cursor == 48, machine::extUnknown(1, 4) },
      all { cursor == 49, machine::reconcile(1, machine::externalResult(machine::state, 1), machine::terminalEvidence(machine::externalToken(machine::state, 1), machine::TxIncluded)) },
      all { cursor == 50, machine::cancel(1, 1) },
      all { cursor == 51, machine::cancel(3, 4) },
      all { cursor == 52, machine::finalize },
    },
    cursor' = cursor + 1,
  }
  val dispatched = machine::dispatched
  val extObserved = machine::extObserved
  val extUnknownWitness = machine::extUnknownWitness
  val cleaned = machine::cleaned
  val reconciled = machine::reconciled
  val reconciledNotStarted = machine::reconciledNotStarted
  val reconciledExited = machine::reconciledExited
  val reconciledIncluded = machine::reconciledIncluded
  val lateResultRetained = machine::lateResultRetained
  val directedFinished = cursor == 53 and (machine::state.terminal and machine::terminalDischarged and machine::TASKS.forall(id => machine::externalDischarged(machine::state, id)) and machine::state.extDispatches.size() == 5 and machine::state.extResults.size() == 5 and machine::state.extResolutions.size() == 3)
  val directedInvariant = machine::workflowInvariant and cursor >= 0 and cursor <= 53

}
