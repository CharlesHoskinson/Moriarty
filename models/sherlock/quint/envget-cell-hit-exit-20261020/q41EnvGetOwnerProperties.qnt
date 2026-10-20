module q41EnvGetOwnerProperties {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import q41EnvGetOwnerRelation.* from "./q41EnvGetOwnerRelation"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"
  // General parameterized predicates only. NO literal admitted prefix/prestate,
  // populated expectedRoot, fabricated PASS flag or executable run fixture.
  pure def q41PreservedOutsideWrites(pre:C6WholeHost,post:C6WholeHost):bool =
    {...post,image:{...post.image,helper:pre.image.helper,result:pre.image.result,
      remaining_work:pre.image.remaining_work,consumed_work:pre.image.consumed_work,
      helper_work:pre.image.helper_work,helper_visits:pre.image.helper_visits}} == pre
  pure def q41WholeWindowExact(expectedArg:Q41Window):bool =
    q41ComputeWindow(expectedArg.before) == Q41ComputedWindow(expectedArg)
  pure def q41OwnerWorkLaws(w:Q41Window):bool = and {
    w.after_cell.image.remaining_work == w.before.image.remaining_work-1,
    w.after_exit.image.remaining_work == w.before.image.remaining_work-2,
    w.after_cell.image.consumed_work == w.before.image.consumed_work+1,
    w.after_exit.image.consumed_work == w.before.image.consumed_work+2,
    w.after_cell.image.helper_work == w.before.image.helper_work+1,
    w.after_exit.image.helper_work == w.before.image.helper_work+2,
    w.after_cell.image.helper_visits == w.before.image.helper_visits+1,
    w.after_exit.image.helper_visits == w.before.image.helper_visits+1,
    w.after_cell.image.reference_cursor == w.before.image.reference_cursor,
    w.after_exit.image.reference_cursor == w.before.image.reference_cursor,
    q41PreservedOutsideWrites(w.before,w.after_cell),
    q41PreservedOutsideWrites(w.after_cell,w.after_exit),
    w.events.length() == 2,
    w.events.nth(0).rule_id == "BHelperEnvGetCell",w.events.nth(1).rule_id == "BHelperEnvGetExit",
    w.events.nth(0).step_id.ordinal == w.before.image.consumed_work,
    w.events.nth(1).step_id.ordinal == w.after_cell.image.consumed_work,
    w.events.nth(0).work_delta == 1,w.events.nth(1).work_delta == 1,
    w.events.nth(0).helper_work_delta == 1,w.events.nth(1).helper_work_delta == 1,
    w.events.nth(0).helper_visits_delta == 1,w.events.nth(1).helper_visits_delta == 0,
    w.events.nth(0).reference_cursor_delta == 0,w.events.nth(1).reference_cursor_delta == 0,
  }
  pure def q41ParametricWholeRootExact(expectedArg:Q41ExpectedRoot):bool = and {
    expectedArg.schema == "independent-quint-EnvGet-owner-window/1",
    expectedArg.profile == "CoreHelperMicrostepDedicatedSorts-EnvGetHitExit-positive/1",
    expectedArg.case_id == "nested-let-x7-y-varx-return-vary",
    q41WholeWindowExact(expectedArg.window),q41OwnerWorkLaws(expectedArg.window),
  }
}
