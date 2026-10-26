module feePoolHistoryTest {
  import feePoolHistory.* from "./feePoolHistory"
  run claimHistoryConservesTest = init
    .then(intake(0, 100, 86400, 17, 10))
    .then(submitOrdinary(5))
    .then(settle(52, 25, 9, 50, 4, 75, 14, 25, 14))
    .then(finish(3))
    .then(claimOrdinary(86400)).then(claimMidnight(86400)).then(claimDust(86400))
    .expect(historyInvariants and world.duties.get("cost_ordinary").present
      and world.duties.get("cost_ordinary").amount == 0)
  run emptyHistoryConservesTest = init.expect(historyInvariants)
  run zeroEntitlementIsPresentTest = init.then(intake(0, 0, 86400, 0, 0))
    .then(settle(0, 0, 0, 0, 0, 0, 0, 0, 0)).then(finish(0))
    .expect(historyInvariants and world.duties.get("treasury_dust").present
      and world.duties.get("treasury_dust").amount == 0
      and not(canClaimDust(world, 86400)))
}
