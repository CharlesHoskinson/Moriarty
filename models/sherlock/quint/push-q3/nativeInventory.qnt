// U023 finite projection of compile_artifact/native_inventory.rs::report.
// Four named TABLE rows and one unknown rule retain each classification case.
// Fragment presence is source coverage, never SDK/proof/recursion acceptance.
module nativeInventory {
  type Rule = EvalIf | ScalarConstruct | ReturnCall | MatchSelected | Unknown
  type Family = Schedule | Scalar | FrameReturn | DataMatch
  type Inventory = {counts: Rule -> int, events: int}
  var inventory: Inventory

  pure val rules = Set(EvalIf, ScalarConstruct, ReturnCall, MatchSelected, Unknown)
  pure val listed = Set(EvalIf, ScalarConstruct, ReturnCall, MatchSelected)
  pure val fragments = Set(EvalIf, ReturnCall)
  pure val families = Map(EvalIf -> Schedule, ScalarConstruct -> Scalar,
    ReturnCall -> FrameReturn, MatchSelected -> DataMatch)
  pure def owner(f: Family): str = match f {
    | Schedule => "dispatch" | Scalar => "scalar"
    | FrameReturn => "frame" | DataMatch => "data"
  }
  pure def observe(s: Inventory, rule: Rule): Inventory =
    {counts: s.counts.set(rule, s.counts.get(rule) + 1), events: s.events + 1}
  pure def exercised(s: Inventory): Set[Rule] = rules.filter(r => s.counts.get(r) > 0)
  pure def gaps(s: Inventory): Set[Rule] = exercised(s).intersect(listed).exclude(fragments)
  pure def unlisted(s: Inventory): Set[Rule] = exercised(s).exclude(listed)
  pure def withFragment(s: Inventory): int = exercised(s).size() - gaps(s).size() - unlisted(s).size()
  pure def complete(s: Inventory): bool = gaps(s).size() == 0 and unlisted(s).size() == 0
  // These are separate fixed fields of the actual Rust report.
  pure val c6FragmentsWired = false
  pure val nativeProducesStepRecords = false
  pure val verifyProofPrerequisite = false
  pure val ledger10Prerequisite = false
  pure def nativeQualified(s: Inventory): bool = complete(s) and c6FragmentsWired

  action init = inventory' = {counts: rules.mapBy(_ => 0), events: 0}
  action record(rule: Rule): bool = inventory' = observe(inventory, rule)
  action step = {
    nondet rule = rules.oneOf()
    record(rule)
  }
  val sawFragment = inventory.counts.get(EvalIf) > 0
  val sawGap = inventory.counts.get(ScalarConstruct) > 0
  val sawUnknown = inventory.counts.get(Unknown) > 0
  val modelInv = and {
    inventory.events == rules.fold(0, (n, r) => n + inventory.counts.get(r)),
    withFragment(inventory) == exercised(inventory).intersect(fragments).size(),
    complete(inventory) == exercised(inventory).subseteq(fragments),
    not(nativeQualified(inventory)),
    not(nativeProducesStepRecords), not(verifyProofPrerequisite), not(ledger10Prerequisite),
    owner(families.get(EvalIf)) == "dispatch",
    owner(families.get(MatchSelected)) == "data",
  }
}
