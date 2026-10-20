module q41ActualOwnerWindowRuntime {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import q41EnvGetOwnerRelation.* from "./q41EnvGetOwnerRelation"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import q41ActualOwnerBefore.* from "./q41ActualOwnerBefore"

  // Actual BEFORE input only. No expectedRoot, after-state or event input.
  // This wrapper invokes the existing relation, not a second owner evaluator.
  // Complete raw ITF must be durably frozen before any selected comparison.
  type Q41ActualWindowState = {
    input_before:C6WholeHost,
    computed_window:Q41NormalWindow,
    computed_once:bool,
  }
  var machine:Q41ActualWindowState

  action init:bool = machine' = {
    input_before:actualBefore,computed_window:Q41NoWindow,computed_once:false,
  }
  action computeActualWindow:bool = all {
    not(machine.computed_once),
    machine' = {...machine,
      computed_window:q41ComputeWindow(machine.input_before),computed_once:true},
  }
  val actualBeforeRetained:bool = machine.input_before == actualBefore
  val wholeWindowRetained:bool = machine.computed_once and match machine.computed_window {
    | Q41NoWindow => false
    | Q41ComputedWindow(windowArg) => windowArg.before == machine.input_before
  }
}
