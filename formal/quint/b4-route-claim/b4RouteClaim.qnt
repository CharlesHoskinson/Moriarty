// B4-03 route claim + failed-absorbing model (B4-provider-link.mjs 086f1a2e), scoped to T20/T21 schedules whose
// refusal is recorded before the parked read is released. Lookup is premised resolved; codec/observing segments collapsed.
module b4RouteClaim {
  type Phase = Idle | Proving | Routed | Returned | Linked | Failed
  type Code = NoErr | RouteWithoutProvider | StateChanged | SealedOutOfOrder | CallbackOutsideRoute | LinkReused | CallbackUnbound | NoRouteCapture
  // Route pc: Out, PreRead (parked on preimage read), Inner (inner.prove in flight), ProofRead (parked on proofFile read), Done, Rej.
  type RPc = Out | PreRead | Inner | ProofRead | Done | Rej
  type PPc = PNone | PAwait | PDone
  type Link = { st: Phase, first: Code, claimed: bool, callback: bool, captured: bool, rpc: int -> RPc, ppc: PPc, innerCalls: int }
  // Route 1 is base.prove's own route call; route 2 is T21's concurrent handRoutes call.
  pure val ROUTES = Set(1, 2)
  pure val INIT: Link = { st: Idle, first: NoErr, claimed: false, callback: false, captured: false, rpc: Map(1 -> Out, 2 -> Out), ppc: PNone, innerCalls: 0 }
  var s: Link

  // failed(): state='failed'; first = first ?? code. Sole writer of first.
  pure def failS(x: Link, c: Code): Link = { ...x, st: Failed, first: if (x.first == NoErr) c else x.first }
  pure def rej(x: Link, r: int, c: Code): Link = failS({ ...x, rpc: x.rpc.set(r, Rej) }, c)
  // at(p): state === p and first === null.
  pure def okAt(x: Link, p: Phase): bool = x.st == p and x.first == NoErr
  val active = ROUTES.exists(r => s.rpc.get(r) == Inner)

  action init = all { s' = INIT }
  // prove() sync prefix: idle->proving before await base.prove; otherwise LINK_REUSED (T20 third inject).
  action providerProve = all { s' = if (s.st == Idle) { ...s, st: Proving, ppc: PAwait } else failS(s, LinkReused) }
  // route.prove sync prefix: check and claim before the first await.
  action routeEnter(r: int): bool = all { s.rpc.get(r) == Out, r == 2 or s.ppc == PAwait,
    s' = if (s.st == Proving and not(s.claimed)) { ...s, claimed: true, rpc: s.rpc.set(r, PreRead) } else rej(s, r, RouteWithoutProvider) }
  // Preimage read resolves: at('proving'), then active is set and inner.prove is called.
  action routeAfterPreimage(r: int): bool = all { s.rpc.get(r) == PreRead,
    s' = if (okAt(s, Proving)) { ...s, rpc: s.rpc.set(r, Inner), innerCalls: s.innerCalls + 1 } else rej(s, r, StateChanged) }
  // bindCallback: needs active, no callback and proving; otherwise CALLBACK_OUTSIDE_ROUTE (T20 second inject).
  action bindCallback = all { s' = if (active and not(s.callback) and s.st == Proving) { ...s, callback: true } else failS(s, CallbackOutsideRoute) }
  // inner.prove returns (active cleared): CALLBACK_UNBOUND unless proving and bound; else parks on the proofFile read.
  action innerReturn(r: int): bool = all { s.rpc.get(r) == Inner,
    s' = if (s.st == Proving and s.callback) { ...s, rpc: s.rpc.set(r, ProofRead) } else rej(s, r, CallbackUnbound) }
  // proofFile read resolves: at('proving') before captured/routed.
  action routeAfterProof(r: int): bool = all { s.rpc.get(r) == ProofRead,
    s' = if (okAt(s, Proving)) { ...s, captured: true, st: Routed, rpc: s.rpc.set(r, Done) } else rej(s, r, StateChanged) }
  // base.prove settles after route 1. The codec segment is collapsed; at('routed') is checked before returned.
  action baseReturn = all { s.ppc == PAwait, Set(Done, Rej).contains(s.rpc.get(1)),
    s' = if (okAt(s, Routed) and s.captured) { ...s, st: Returned, ppc: PDone } else failS({ ...s, ppc: PDone }, NoRouteCapture) }
  // observeSealed (sync): returned with first null -> linked; otherwise SEALED_OUT_OF_ORDER (T20 first inject).
  action observeSealed = all { s' = if (okAt(s, Returned)) { ...s, st: Linked } else failS(s, SealedOutOfOrder) }

  action step = any {
    providerProve, bindCallback, baseReturn, observeSealed,
    nondet r = ROUTES.oneOf()
    any { routeEnter(r), routeAfterPreimage(r), innerReturn(r), routeAfterProof(r) },
  }

  // EXPECTED-BAD OLD02 route (d0db648d:114-123), never in step: no claim, entry checks active/captured, no at() after the reads.
  action oldRouteEnter(r: int): bool = all { s.rpc.get(r) == Out, r == 2 or s.ppc == PAwait,
    s' = if (s.st == Proving and not(active) and not(s.captured)) { ...s, rpc: s.rpc.set(r, PreRead) } else rej(s, r, RouteWithoutProvider) }
  action oldRouteAfterPreimage(r: int): bool = all { s.rpc.get(r) == PreRead, s' = { ...s, rpc: s.rpc.set(r, Inner), innerCalls: s.innerCalls + 1 } }
  action oldRouteAfterProof(r: int): bool = all { s.rpc.get(r) == ProofRead, s' = { ...s, captured: true, st: Routed, rpc: s.rpc.set(r, Done) } }

  val failedAbsorbingInv = s.first != NoErr implies s.st == Failed
  val firstNotStateChangedInv = s.first != StateChanged
  val singleInnerInv = s.innerCalls <= 1
  val singleClaimInv = ROUTES.filter(r => not(Set(Out, Rej).contains(s.rpc.get(r)))).size() <= 1
  val cleanProgressInv = Set(Routed, Returned, Linked).contains(s.st) implies (s.first == NoErr and s.captured and s.innerCalls == 1)
  val safety = and { failedAbsorbingInv, firstNotStateChangedInv, singleInnerInv, singleClaimInv, cleanProgressInv }
}
