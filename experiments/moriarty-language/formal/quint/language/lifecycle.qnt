// Scoped reservation/unknown-outcome model. No messages or remote consensus.
// A verified unused receipt is assumed authentic, request-bound, irreversible,
// and incompatible with success for that same request. Timeout supplies no receipt.
module lifecycle {
  type Phase = Idle | Reserved | Pending | Unknown | Settled | ProvedUnused | Recovered
  type Request = {
    phase: Phase, amount: int, delivered: int, duty: int, timedOut: bool,
    refunded: int, used: int, unusedVerified: bool,
  }
  type State = { requests: int -> Request, available: int, held: int, spent: int }
  var state: State
  pure val IDS = Set(0, 1)
  pure val empty: Request = { phase: Idle, amount: 0, delivered: 0, duty: 10, timedOut: false, refunded: 0, used: 0, unusedVerified: false }
  pure val initial: State = { requests: IDS.mapBy(_ => empty), available: 6, held: 0, spent: 0 }
  pure def canReserve(s: State, id: int, amount: int): bool =
    IDS.contains(id) and s.requests.get(id).phase == Idle and amount > 0 and amount <= 6 and s.available >= amount
  pure def applyReserve(s: State, id: int, amount: int): State = {
    ...s, requests: s.requests.set(id, { ...s.requests.get(id), phase: Reserved, amount: amount }),
    available: s.available - amount, held: s.held + amount,
  }
  pure def canDispatch(s: State, id: int): bool = s.requests.get(id).phase == Reserved
  pure def applyDispatch(s: State, id: int): State = { ...s, requests: s.requests.set(id, { ...s.requests.get(id), phase: Pending }) }
  pure def canTimeout(s: State, id: int): bool = s.requests.get(id).phase == Pending
  pure def applyTimeout(s: State, id: int): State = { ...s, requests: s.requests.set(id, { ...s.requests.get(id), phase: Unknown, timedOut: true }) }
  pure def canSettle(s: State, id: int, delivered: int, successVerified: bool): bool = {
    val r = s.requests.get(id)
    (r.phase == Pending or r.phase == Unknown) and successVerified and not(r.unusedVerified) and delivered > 0 and delivered <= r.duty
  }
  pure def applySettle(s: State, id: int, delivered: int): State = {
    val r = s.requests.get(id)
    { ...s, requests: s.requests.set(id, { ...r, phase: Settled, delivered: delivered, duty: r.duty - delivered, used: r.amount }),
      held: s.held - r.amount, spent: s.spent + r.amount }
  }
  pure def canProveUnused(s: State, id: int, unusedVerified: bool): bool = s.requests.get(id).phase == Unknown and unusedVerified
  pure def applyProveUnused(s: State, id: int): State = { ...s, requests: s.requests.set(id, { ...s.requests.get(id), phase: ProvedUnused, unusedVerified: true }) }
  pure def canRecover(s: State, id: int): bool = s.requests.get(id).phase == ProvedUnused and s.requests.get(id).unusedVerified
  pure def applyRecover(s: State, id: int): State = {
    val r = s.requests.get(id)
    { ...s, requests: s.requests.set(id, { ...r, phase: Recovered, refunded: r.amount }),
      held: s.held - r.amount, available: s.available + r.amount }
  }
  action init = state' = initial
  action reserve(id: int, amount: int): bool = all { canReserve(state, id, amount), state' = applyReserve(state, id, amount) }
  action dispatch(id: int): bool = all { canDispatch(state, id), state' = applyDispatch(state, id) }
  action timeout(id: int): bool = all { canTimeout(state, id), state' = applyTimeout(state, id) }
  action settle(id: int, delivered: int, successVerified: bool): bool = all { canSettle(state, id, delivered, successVerified), state' = applySettle(state, id, delivered) }
  action proveUnused(id: int, unusedVerified: bool): bool = all { canProveUnused(state, id, unusedVerified), state' = applyProveUnused(state, id) }
  action recover(id: int): bool = all { canRecover(state, id), state' = applyRecover(state, id) }
  action step = {
    nondet id = IDS.oneOf()
    nondet amount = Set(2, 3, 4).oneOf()
    nondet delivered = Set(3, 10).oneOf()
    any { reserve(id, amount), dispatch(id), timeout(id), settle(id, delivered, true), proveUnused(id, true), recover(id) }
  }
  val reservationExclusive = state.available >= 0 and state.held >= 0 and state.spent >= 0 and state.available + state.held + state.spent == 6
  val heldMatchesReservations = state.held == IDS.fold(0, (total, id) => {
    val r = state.requests.get(id)
    total + (if (Set(Reserved, Pending, Unknown, ProvedUnused).contains(r.phase)) r.amount else 0)
  })
  val noRefundPlusSuccess = IDS.forall(id => {
    val r = state.requests.get(id)
    (r.refunded == 0 or r.used == 0) and (r.refunded > 0 implies r.unusedVerified) and (r.delivered > 0 implies r.phase == Settled)
  })
  val persistentDuties = IDS.forall(id => { val r = state.requests.get(id) r.duty >= 0 and r.delivered + r.duty == 10 })
  val timeoutRetainsExposure = IDS.forall(id => {
    val r = state.requests.get(id)
    r.phase == Unknown implies r.refunded == 0 and r.used == 0 and r.duty == 10 and not(r.unusedVerified)
  })
  val safety = reservationExclusive and heldMatchesReservations and noRefundPlusSuccess and persistentDuties and timeoutRetainsExposure
  val reservedWitness = state.held > 0
  val unknownWitness = IDS.exists(id => state.requests.get(id).phase == Unknown)
  val lateSuccessWitness = IDS.exists(id => { val r = state.requests.get(id) r.phase == Settled and r.timedOut })
  val recoveredWitness = IDS.exists(id => state.requests.get(id).phase == Recovered)
  val residualDutyWitness = IDS.exists(id => { val r = state.requests.get(id) r.phase == Settled and r.duty > 0 })
}
