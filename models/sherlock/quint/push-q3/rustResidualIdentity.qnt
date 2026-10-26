// Source-derived U033 counterexample at 2d1b2b373 draft_plans.rs:143-151.
// Rust itself was not built/run here. reference.rs::step/evaluate imply these
// exact rule sets for one immutable literal-6 program with fuel 0 versus 1.
// draft_plans takes exercised rules as static_rules for per-program mode.
// Content identity is represented structurally; SHA collision analysis omitted.
module rustResidualIdentity {
  type IR = {programImage: int, staticRules: Set[int], staticRuleCount: int}
  type Comparison = {compiled: bool, fuelZero: IR, fuelOne: IR}
  var comparison: Comparison
  pure val coldRules = Set(84)
  pure val warmRules = Set(102, 80)
  pure val charges = Map(80 -> 0, 84 -> 0, 102 -> 1)
  pure def specialize(exercised: Set[int]): IR =
    {programImage: 6, staticRules: exercised, staticRuleCount: exercised.size()}
  action init = comparison' = {compiled: false, fuelZero: specialize(Set()), fuelOne: specialize(Set())}
  action compilePair = all {
    not(comparison.compiled),
    comparison' = {compiled: true, fuelZero: specialize(coldRules), fuelOne: specialize(warmRules)},
  }
  action step = compilePair
  val stableAcrossFuel = not(comparison.compiled) or comparison.fuelZero == comparison.fuelOne
  val sourceInv = and {
    comparison.fuelZero.programImage == comparison.fuelOne.programImage,
    comparison.fuelZero.staticRuleCount == comparison.fuelZero.staticRules.size(),
    comparison.fuelOne.staticRuleCount == comparison.fuelOne.staticRules.size(),
    if (comparison.compiled) comparison.fuelZero.staticRules == coldRules
      and comparison.fuelOne.staticRules == warmRules else true,
  }
  val observedRuntimeDependency = comparison.compiled and not(stableAcrossFuel)
}
