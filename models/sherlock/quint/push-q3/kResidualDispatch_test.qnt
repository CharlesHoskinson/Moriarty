module kResidualDispatch_test {
  import kResidualDispatch.* from "./kResidualDispatch"
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
