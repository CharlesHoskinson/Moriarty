// Directed reachability witness, separate from the unchanged default sampler.
module doctorwatson_coverage {
  import doctorwatson(
    TASKS = Set(1, 2, 3), ACTORS = Set(1, 2, 3, 4), INTEGRATOR = 4,
    ELIGIBLE_REVIEWERS = Map(1 -> Set(3), 2 -> Set(3), 3 -> Set(3))
  ) as machine from "./doctorwatson"

  var cursor: int
  action init = all { machine::init, cursor' = 0 }

  // Exactly one machine operation and one cursor increment per driver step.
  // Cursor 16 integrates T1 and makes T2 stale; cursor 17 rebases T2.
  // Cursor 43 adds merged T2 as T1's dependency, after all rare states were seen.
  action step = all {
    cursor >= 0 and cursor < 47,
    any {
      all { cursor == 0, machine::claim(1, 1) },
      all { cursor == 1, machine::freeze(1, 3, true) },
      all { cursor == 2, machine::edit(1, 1, machine::Source) },
      all { cursor == 3, machine::submit(1) },
      all { cursor == 4, machine::startLocal(1) },
      all { cursor == 5, machine::checker(1, machine::jobToken(machine::state, 1), machine::CompleteMatch) },
      all { cursor == 6, machine::regressions(1, machine::jobToken(machine::state, 1), true) },
      all { cursor == 7, machine::review(1, 3, true) },
      all { cursor == 8, machine::claim(2, 2) },
      all { cursor == 9, machine::freeze(2, 3, true) },
      all { cursor == 10, machine::edit(2, 2, machine::Source) },
      all { cursor == 11, machine::submit(2) },
      all { cursor == 12, machine::startLocal(2) },
      all { cursor == 13, machine::checker(2, machine::jobToken(machine::state, 2), machine::CompleteMatch) },
      all { cursor == 14, machine::regressions(2, machine::jobToken(machine::state, 2), true) },
      all { cursor == 15, machine::review(2, 3, true) },
      all { cursor == 16, machine::merge(1) },
      all { cursor == 17, machine::rebase(2) },
      all { cursor == 18, machine::submit(2) },
      all { cursor == 19, machine::startLocal(2) },
      all { cursor == 20, machine::checker(2, machine::jobToken(machine::state, 2), machine::CompleteMatch) },
      all { cursor == 21, machine::regressions(2, machine::jobToken(machine::state, 2), true) },
      all { cursor == 22, machine::review(2, 3, true) },
      all { cursor == 23, machine::merge(2) },
      all { cursor == 24, machine::claim(3, 4) },
      all { cursor == 25, machine::freeze(3, 3, true) },
      all { cursor == 26, machine::edit(3, 4, machine::Source) },
      all { cursor == 27, machine::submit(3) },
      all { cursor == 28, machine::startLocal(3) },
      all { cursor == 29, machine::checker(3, machine::jobToken(machine::state, 3), machine::CompleteMatch) },
      all { cursor == 30, machine::regressions(3, machine::jobToken(machine::state, 3), true) },
      all { cursor == 31, machine::review(3, 3, true) },
      all { cursor == 32, machine::makeAvailable(machine::K) },
      all { cursor == 33, machine::makeAvailable(machine::Quint) },
      all { cursor == 34, machine::makeAvailable(machine::LeanClean) },
      all { cursor == 35, machine::makeAvailable(machine::Handwritten) },
      all { cursor == 36, machine::observe(3, machine::K) },
      all { cursor == 37, machine::observe(3, machine::Quint) },
      all { cursor == 38, machine::observe(3, machine::LeanClean) },
      all { cursor == 39, machine::observe(3, machine::Handwritten) },
      all { cursor == 40, machine::merge(3) },
      all { cursor == 41, machine::complete(3) },
      all { cursor == 42, machine::edit(1, 1, machine::Tool) },
      all { cursor == 43, machine::changeDependencies(1, 2, 1) },
      all { cursor == 44, machine::cancel(1, 1) },
      all { cursor == 45, machine::nameBlocker(3, 4) },
      all { cursor == 46, machine::finalize },
    },
    cursor' = cursor + 1,
  }
  val rebased = machine::rebased
  val integrated = machine::integrated
  val dependenciesChanged = machine::dependenciesChanged
  val completed = machine::completed
  val sourceOnly = machine::sourceOnly
  val directedFinished = cursor == 47 and machine::state.terminal
  val directedInvariant = machine::workflowInvariant and cursor >= 0 and cursor <= 47
}
