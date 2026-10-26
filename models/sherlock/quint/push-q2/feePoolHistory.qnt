// Reuse the accepted component without changing its source or assumptions.
// These additional invariants independently replay its ordered public effects.
// Premises: fixed epoch/seats/duties and asset, public time/auth/membership
// assumptions of specs/quint/fee_pool_component.README.md; no oracle or ledger
// clock authentication. Requires TS backend for the full U64 constant.
module feePoolHistory {
  import fee_pool_component.* from "../../../../specs/quint/fee_pool_component"
  export fee_pool_component.*
  pure def cashFromHistory(log: List[FinancialEffect], actor: str): int =
    log.foldl(if (actor == "funding-source") M else 0, (a, e) =>
      if (e.kind != "Transfer") a else a
        + (if (e.recipient == actor) e.amount else 0)
        - (if (e.source == actor) e.amount else 0))
  pure def dutyFromHistory(log: List[FinancialEffect], duty: str): int =
    log.foldl(0, (a, e) => if (e.duty != duty) a
      else if (e.kind == "Liability") a + e.amount
      else if (e.kind == "Discharge") a - e.amount else a)
  val cashReplayInv = ACTORS.forall(a => world.cash.get(a) == cashFromHistory(world.history, a))
  val dutyReplayInv = DUTIES.forall(d => world.duties.get(d).amount == dutyFromHistory(world.history, d))
  val paidDutiesInv = DUTIES.forall(d =>
    world.history.select(e => e.kind == "Transfer" and e.source == "fee-pool" and e.duty == d)
      .foldl(0, (a, e) => a + e.amount) ==
    world.history.select(e => e.kind == "Discharge" and e.duty == d)
      .foldl(0, (a, e) => a + e.amount))
  val reserveCoverageInv = world.cash.get("fee-pool") == outstanding(world)
    + world.s.backstop + world.s.watcherHeld + world.s.carried
  val historyInvariants = allInvariants and cashReplayInv and dutyReplayInv and paidDutiesInv and reserveCoverageInv
}
