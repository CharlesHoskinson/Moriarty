// Bounded two-order same-domain clearing. Oracle/auth/resource booleans are
// symbolic verified premises; this model does not contact or implement an oracle.
module batch {
  type Order = { filled: int, executionPrice: int, limitPrice: int }
  type Proposal = {
    orders: int -> Order, uniformPrice: int, solver: str, solverFee: int,
    oracle: str, oracleTime: int, now: int, oraclePrice: int, oracleVerified: bool,
    network: str, buyAsset: str, sellAsset: str, complete: bool, liability: int,
  }
  type State = {
    buyerCash: int, sellerCash: int, solverCash: int, buyerToken: int, sellerToken: int,
    gross: int, fees: int, net: int, liability: int, accepts: int, rejects: int,
    last: Proposal, uniformPreserved: bool,
  }
  var state: State
  pure val IDS = Set(0, 1)
  pure val good: Proposal = {
    orders: IDS.mapBy(_ => { filled: 1, executionPrice: 3, limitPrice: 5 }),
    uniformPrice: 3, solver: "clearing-solver", solverFee: 1,
    oracle: "price-authority", oracleTime: 8, now: 10, oraclePrice: 3, oracleVerified: true,
    network: "same-domain", buyAsset: "issuer-A/cash/0", sellAsset: "issuer-B/token/0",
    complete: true, liability: 0,
  }
  pure def quantity(p: Proposal): int = IDS.fold(0, (total, id) => total + p.orders.get(id).filled)
  pure def cost(p: Proposal): int = p.uniformPrice * quantity(p) + p.solverFee
  pure def canClear(s: State, p: Proposal): bool = and {
    p.orders.keys() == IDS, p.complete, p.network == "same-domain",
    p.buyAsset == "issuer-A/cash/0", p.sellAsset == "issuer-B/token/0",
    p.solver == "clearing-solver", p.solverFee >= 0, p.solverFee <= 2,
    p.oracle == "price-authority", p.oracleVerified, p.oracleTime >= 0,
    p.now >= p.oracleTime, p.now - p.oracleTime <= 3,
    p.oraclePrice > 0, p.uniformPrice > 0, p.uniformPrice <= 10,
    p.uniformPrice >= p.oraclePrice - 1, p.uniformPrice <= p.oraclePrice + 1,
    IDS.forall(id => { val o = p.orders.get(id) and {
      o.filled > 0, o.filled <= 2, o.executionPrice == p.uniformPrice,
      o.limitPrice >= 2, o.executionPrice <= o.limitPrice, o.executionPrice >= 2,
    } }),
    quantity(p) <= s.sellerToken, cost(p) <= s.buyerCash,
    s.gross + cost(p) <= 18, s.fees + p.solverFee <= 4,
    p.uniformPrice * quantity(p) >= p.solverFee, s.net + p.uniformPrice * quantity(p) - p.solverFee >= 5,
    p.liability >= 0, s.liability + p.liability <= 3,
  }
  pure def applyClear(s: State, p: Proposal): State = {
    ...s, buyerCash: s.buyerCash - cost(p), sellerCash: s.sellerCash + p.uniformPrice * quantity(p),
    solverCash: s.solverCash + p.solverFee, buyerToken: s.buyerToken + quantity(p), sellerToken: s.sellerToken - quantity(p),
    gross: s.gross + cost(p), fees: s.fees + p.solverFee, net: s.net + p.uniformPrice * quantity(p) - p.solverFee,
    liability: s.liability + p.liability, accepts: s.accepts + 1, last: p,
    uniformPreserved: s.uniformPreserved and IDS.forall(id => p.orders.get(id).executionPrice == p.uniformPrice),
  }
  pure def applyRefuse(s: State): State = { ...s, rejects: s.rejects + 1 }
  action init = state' = { buyerCash: 20, sellerCash: 0, solverCash: 0, buyerToken: 0, sellerToken: 8,
    gross: 0, fees: 0, net: 0, liability: 0, accepts: 0, rejects: 0, last: good, uniformPreserved: true }
  action clear(p: Proposal): bool = all { canClear(state, p), state' = applyClear(state, p) }
  action refuse(p: Proposal): bool = all { not(canClear(state, p)), state' = applyRefuse(state) }
  action step = {
    nondet kind = 0.to(5).oneOf()
    val p = if (kind == 0) good else if (kind == 1) { ...good, solverFee: 2 }
      else if (kind == 2) { ...good, oracleTime: 1 }
      else if (kind == 3) { ...good, orders: good.orders.set(1, { ...good.orders.get(1), executionPrice: 4 }) }
      else if (kind == 4) { ...good, solver: "unauthorized" }
      else { ...good, network: "other-domain" }
    any { clear(p), refuse(p) }
  }
  val conservation = state.buyerCash >= 0 and state.sellerToken >= 0 and state.buyerCash + state.sellerCash + state.solverCash == 20 and state.buyerToken + state.sellerToken == 8
  val uniformPrice = state.uniformPreserved
  val bounds = state.gross <= 18 and state.fees <= 4 and state.liability <= 3 and state.net == state.sellerCash - state.solverCash
  val oracleSafety = state.accepts > 0 implies state.last.oracleVerified and state.last.oracle == "price-authority" and state.last.now - state.last.oracleTime <= 3
  val safety = conservation and uniformPrice and bounds and oracleSafety
  val clearedWitness = state.accepts > 0
  val rejectedWitness = state.rejects > 0
  val partialFillWitness = state.accepts > 0 and state.sellerToken > 0
}
