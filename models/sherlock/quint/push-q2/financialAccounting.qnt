// Exact UInt128 transfer semantics plus checked report in
// stage/sherlock2.rs::accounting (907e5fc7) and suspension2_fin.rs::fin_block (2852e423).
// No cryptography or ledger token model.
// Premises: admitted asset/account identities, preexisting balance rows,
// atomic checked transfer; two assets, three accounts, initial supply MAX each.
// Transfer effects include self transfers; aggregate history is unbounded by
// UInt128 even though each amount and balance is UInt128. Requires TS backend.
module financialAccounting {
  pure val MAX = 340282366920938463463374607431768211455
  pure val ACTORS = Set(0, 1, 2)
  pure val ASSETS = Set(0, 1)
  type Effect = {asset: int, sender: int, recipient: int, amount: int, fee: bool}
  type State = {
    cash: (int, int) -> int,
    log: List[Effect],
    reportOk: bool,
    payments: int,
    fees: int,
    gross: int,
  }
  var world: State
  pure def initialCash(actor: int): int = if (actor == 0) MAX else 0
  pure def exactPayments(log: List[Effect]): int =
    log.foldl(0, (a, e) => a + (if (e.fee) 0 else e.amount))
  pure def exactFees(log: List[Effect]): int =
    log.foldl(0, (a, e) => a + (if (e.fee) e.amount else 0))
  pure def canTransfer(s: State, e: Effect): bool = and {
    ASSETS.contains(e.asset), ACTORS.contains(e.sender), ACTORS.contains(e.recipient),
    e.amount >= 0, e.amount <= MAX, e.amount <= s.cash.get((e.asset, e.sender)),
    e.sender == e.recipient or s.cash.get((e.asset, e.recipient)) + e.amount <= MAX,
  }
  pure def transferred(s: State, e: Effect): State = {
    val cash = if (e.sender == e.recipient) s.cash else
      s.cash.set((e.asset, e.sender), s.cash.get((e.asset, e.sender)) - e.amount)
        .set((e.asset, e.recipient), s.cash.get((e.asset, e.recipient)) + e.amount)
    val log = s.log.append(e)
    val p = exactPayments(log)
    val f = exactFees(log)
    val ok = p + f <= MAX
    {
      cash: cash,
      log: log,
      reportOk: ok,
      payments: if (ok) p else 0,
      fees: if (ok) f else 0,
      gross: if (ok) p + f else 0,
    }
  }
  action init: bool = world' = {
    cash: Set((0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)).mapBy(k => initialCash(k._2)),
    log: List(), reportOk: true, payments: 0, fees: 0, gross: 0,
  }
  action transfer(e: Effect): bool = all {canTransfer(world, e), world' = transferred(world, e)}
  action step: bool = {
    nondet asset = ASSETS.oneOf()
    nondet sender = ACTORS.oneOf()
    nondet recipient = ACTORS.oneOf()
    nondet amount = Set(0, 1, MAX - 1, MAX).oneOf()
    nondet fee = Set(false, true).oneOf()
    any {
      transfer({asset: asset, sender: sender, recipient: recipient, amount: amount, fee: fee}),
      // Explicit observation after a rejected request; the Rust mutator writes nothing.
      all {not(canTransfer(world, {asset: asset, sender: sender, recipient: recipient,
        amount: amount, fee: fee})), world' = world},
    }
  }
  val conservationInv = ASSETS.forall(asset =>
    ACTORS.fold(0, (a, actor) => a + world.cash.get((asset, actor))) == MAX)
  val rangeInv = world.cash.keys().forall(k => world.cash.get(k) >= 0 and world.cash.get(k) <= MAX)
  val exactBalanceInv = ASSETS.forall(asset => ACTORS.forall(actor =>
    world.cash.get((asset, actor)) == initialCash(actor) + world.log.foldl(0, (a, e) =>
      a + (if (e.asset == asset and e.recipient == actor) e.amount else 0)
        - (if (e.asset == asset and e.sender == actor) e.amount else 0))))
  val checkedReportInv = world.reportOk iff (exactPayments(world.log) + exactFees(world.log) <= MAX)
  val exactReportInv = world.reportOk implies (
    world.payments == exactPayments(world.log)
      and world.fees == exactFees(world.log)
      and world.gross == exactPayments(world.log) + exactFees(world.log)
  )
  val reportEquationInv = world.reportOk implies (world.gross == world.payments + world.fees)
  val allInvariants = conservationInv and rangeInv and exactBalanceInv
    and checkedReportInv and exactReportInv and reportEquationInv
  // Multi-transfer history over u128 balances can still exceed u128::MAX, where
  // checked accounting returns None and fin_block refuses to publish.
  val unboundedHistoryFitsU128Inv = world.reportOk
  val selfTransfer = world.log.select(e => e.sender == e.recipient and e.amount > 0).length() > 0
  val overflowed = not(world.reportOk)
  val feeObserved = world.fees > 0
}
