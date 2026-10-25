module financialAccountingTest {
  import financialAccounting.* from "./financialAccounting"
  run grossCountsFeeOnceTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 1, amount: 30, fee: false}))
    .then(transfer({asset: 0, sender: 0, recipient: 2, amount: 2, fee: true}))
    .expect(allInvariants and exactReportInv and world.gross == 32
      and world.payments == 30 and world.fees == 2)
  run selfTransferPreservesValueTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 0, amount: MAX, fee: false}))
    .expect(allInvariants and world.cash.get((0, 0)) == MAX and world.gross == MAX)
  run distinctAssetsPreservedTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 1, amount: 5, fee: false}))
    .expect(allInvariants and world.cash.get((1, 0)) == MAX and world.cash.get((1, 1)) == 0)
  run insufficientBalanceRefusedTest = init
    .expect(not(canTransfer(world, {asset: 0, sender: 1, recipient: 0, amount: 1, fee: false})))
  // Two valid transfers, no overflowing balance. The report silently clamps.
  run saturatedGrossCounterexampleTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 0, amount: MAX, fee: false}))
    .then(transfer({asset: 0, sender: 0, recipient: 0, amount: MAX, fee: true}))
    .expect(allInvariants and not(exactReportInv) and not(reportEquationInv)
      and world.payments == MAX and world.fees == MAX and world.gross == MAX)
  run roundTripOverflowCounterexampleTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 1, amount: MAX, fee: false}))
    .then(transfer({asset: 0, sender: 1, recipient: 0, amount: MAX, fee: false}))
    .expect(allInvariants and not(exactReportInv) and world.cash.get((0, 0)) == MAX
      and exactPayments(world.log) == 2 * MAX and world.payments == MAX)
}
