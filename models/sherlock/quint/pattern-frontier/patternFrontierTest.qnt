module patternFrontierTest {
  import patternFrontier.* from "./patternFrontier"
  import patternFrontierInputs as I from "./patternFrontierInputs"
  // Model-to-model correspondence, NOT independent expected traces or a B clock.
  // Every case asserts positive input premises, completion within its analysis
  // bound, full saved-environment result, occurrence order and pending cursor.
  run frontierCase0CorrespondenceTest = frontierCaseCorresponds(I::frontierInput0)
  run frontierCase1CorrespondenceTest = frontierCaseCorresponds(I::frontierInput1)
  run frontierCase2CorrespondenceTest = frontierCaseCorresponds(I::frontierInput2)
  run frontierCase3CorrespondenceTest = frontierCaseCorresponds(I::frontierInput3)
  run frontierCase4CorrespondenceTest = frontierCaseCorresponds(I::frontierInput4)
  // Each of the 33 retained local cursors resumes to EXACTLY the same complete
  // cursor (including diagnostics/visits/event/input), with terminal stutters.
  // This is finite deterministic replay; it is NOT a production checkpoint ABI.
  run frontierCase0CutsTest = frontierAllCuts(I::frontierInput0)
  run frontierCase1CutsTest = frontierAllCuts(I::frontierInput1)
  run frontierCase2CutsTest = frontierAllCuts(I::frontierInput2)
  run frontierCase3CutsTest = frontierAllCuts(I::frontierInput3)
  run frontierCase4CutsTest = frontierAllCuts(I::frontierInput4)

  // Generic directed-coverage predicates, kept outside the transition relation.
  pure def frontierHasDiscardWitness(input: FrontierInput): bool = {
    val prefixes = frontierTrace(input)
    range(0, prefixes.length() - 1).foldl(false, (seen, index) => seen or
      (prefixes.nth(index).partial.length() > 0 and prefixes.nth(index + 1).event == ArmMiss))
  }
  pure def frontierHasSharedOccurrenceWitness(input: FrontierInput): bool = {
    val prefixes = frontierTrace(input)
    val visits = prefixes.nth(prefixes.length() - 1).visits
    visits.foldl(false, (seen, earlier) => seen or visits.foldl(false, (found, later) => found or
      (earlier.armIndex == later.armIndex and earlier.valueRow == later.valueRow and earlier.path != later.path)))
  }
  pure def frontierHasWholeWildcardWitness(input: FrontierInput): bool = {
    val prefixes = frontierTrace(input)
    val visits = prefixes.nth(prefixes.length() - 1).visits
    visits.foldl(false, (seen, visit) => seen or
      (frontierIsWildcard(visit.token) and input.decodedValues.get(visit.valueRow).length() > 1))
  }
  pure def frontierHasRetainedParentWitness(input: FrontierInput): bool =
    frontierTrace(input).foldl(false, (seen, prefix) => seen or
      (prefix.tasks.length() >= 2 and match prefix.tasks.head() {
        | InspectTask(child) => match prefix.tasks.nth(1) {
            | FieldTask(parent) => parent.patterns.len >= 3 and parent.cursor > 0 and
                child.path == parent.path.append(parent.cursor - 1)
            | _ => false
          }
        | _ => false
      }))
  run frontierDirectedCoverageTest = and {
    I::frontierCohort.foldl(false, (seen, input) => seen or frontierHasDiscardWitness(input)),
    I::frontierCohort.foldl(false, (seen, input) => seen or frontierHasSharedOccurrenceWitness(input)),
    I::frontierCohort.foldl(false, (seen, input) => seen or frontierHasWholeWildcardWitness(input)),
    I::frontierCohort.foldl(false, (seen, input) => seen or frontierHasRetainedParentWitness(input)),
  }
}
