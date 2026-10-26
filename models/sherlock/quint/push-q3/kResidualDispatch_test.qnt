module kResidualDispatch_test {
  import kResidualDispatch.* from "./kResidualDispatch"
  // All positive-fuel selector tags/predicates, independently of random runs.
  pure val selectorCases = Set(Eval, Push, Return, Invoke, Enter, Bound, Cases, CaseResult, Primitive, Error)
    .map(control => Set(Literal, VarHit, VarMiss, Let, Binary, Unary, If, Match, CallEmpty, CallArgs)
      .map(expr => Set(Empty, LetF, LeftF, RightF, UnaryF, IfF, MatchF, ArgsF, ReturnF, ScopeF)
        .map(frame => Set(false, true).map(flag => Set(TrueValue, FalseValue, InvalidValue)
          .map(condition => {...base, control: control, expr: expr, frame: frame,
            flag: flag, condition: condition})).flatten()).flatten()).flatten()).flatten()
  run all39RulesHaveCorrectFiniteSelectorsTest = init.expect({
    val withExhaustion = selectorCases.union(Set({...base, fuel: 0}))
    selectorCases.size() == 6000 and withExhaustion.map(s => referenceRule(s)) == inventory and
      withExhaustion.forall(s => rules.get(dispatch(s)).contains(referenceRule(s))) and modelInv
  })
  run completeDisjoint39RulePartitionTest = init.expect(inventory.size() == 39 and
    groups.forall(left => groups.forall(right => left == right or rules.get(left).intersect(rules.get(right)) == Set())) and modelInv)
  run returnPublicationWinsOverZeroFuelTest = init.then(sample({...base, control: Return, fuel: 0}))
    .expect(observation.rule == 80 and observation.group == Publish and modelInv)
  run errorPublicationWinsOverZeroFuelTest = init.then(sample({...base, control: Error, fuel: 0}))
    .expect(observation.rule == 82 and modelInv)
  run nextArgumentCanEnterBranchDirectlyTest = init.then(sample({...base, control: Return, frame: ArgsF, expr: If}))
    .expect(observation.rule == 177 and afterNextArgument(observation.input) == Branch and modelInv)
  run nextArgumentCanEnterMatchDirectlyTest = init.then(sample({...base, control: Return, frame: ArgsF, expr: Match}))
    .expect(observation.rule == 177 and afterNextArgument(observation.input) == DataMatch and modelInv)
  run nextArgumentCanEnterMissingVariableCheckTest = init.then(sample({...base, control: Return, frame: ArgsF, expr: VarMiss}))
    .expect(afterNextArgument(observation.input) == ScalarCheck and modelInv)
  run limitOneDelegatesToSharedGenericTest = init.then(sample({...base, limit: 1}))
    .expect(observation.rule == 102 and observation.block == Generic and modelInv)
  run scopeReturnBelongsToFrameGroupTest = init.then(sample({...base, control: Return, frame: ScopeF}))
    .expect(observation.rule == 195 and observation.group == FrameReturn and modelInv)
}
