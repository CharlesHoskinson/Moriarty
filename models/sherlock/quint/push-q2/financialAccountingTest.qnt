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
  // Two valid transfers with no overflowing balance; checked accounting now
  // returns None (reportOk == false) rather than silently clamping to MAX.
  run checkedGrossOverflowRefusedTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 0, amount: MAX, fee: false}))
    .then(transfer({asset: 0, sender: 0, recipient: 0, amount: MAX, fee: true}))
    .expect(allInvariants and not(world.reportOk) and not(unboundedHistoryFitsU128Inv)
      and exactPayments(world.log) == MAX and exactFees(world.log) == MAX)
  run roundTripOverflowRefusedTest = init
    .then(transfer({asset: 0, sender: 0, recipient: 1, amount: MAX, fee: false}))
    .then(transfer({asset: 0, sender: 1, recipient: 0, amount: MAX, fee: false}))
    .expect(allInvariants and not(world.reportOk) and not(unboundedHistoryFitsU128Inv)
      and world.cash.get((0, 0)) == MAX and exactPayments(world.log) == 2 * MAX)
}
