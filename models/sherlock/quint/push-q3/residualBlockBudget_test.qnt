module residualBlockBudget_test {
  import residualBlockBudget.* from "./residualBlockBudget"
  action flattenTerminalCharge = compiler' = {...compiler, charge: compiler.charge.set(4, 1)}
  run oneBlockLimitDelegatesEveryRuleTest = init.then(configure(1))
    .then(visit(0)).then(visit(1)).then(visit(2)).then(visit(3)).then(visit(4)).then(visit(5)).then(finish)
    .expect(compiled and blockCount(compiler) == 1 and compiler.generic == rules
      and compiler.charge.get(4) == 0 and compiler.charge.get(5) == 0 and modelInv)
  run fallbackBlockIsReservedInsideBudgetTest = init.then(configure(2))
    .then(visit(0)).then(visit(1)).then(visit(2)).then(visit(3)).then(visit(4)).then(visit(5)).then(finish)
    .expect(compiler.specialized == Set(0) and blockCount(compiler) == 2 and modelInv)
  run zeroBudgetIsToolingRefusalTest = init.then(configure(0))
    .expect(invalidRefused and compiler.after.outcome == 0 and compiler.after.fuel == 0 and modelInv)
  run sharedBlockCannotFlattenChargesTest = init.then(configure(1)).then(visit(4))
    .then(flattenTerminalCharge).expect(not(modelInv))
  run compileBudgetCannotExhaustApplicationTest = init.then(configure(1)).then(visit(3))
    .expect(genericUsed and compiler.after == runtime and modelInv)
}
