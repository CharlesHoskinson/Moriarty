// Bounded shared-ledger QA. Verified booleans are explicit symbolic premises.
// They do not establish cryptography, chain authentication or measured resources.
module acceptance {
  type AssetId = { network: str, issuer: str, reference: str, scale: int }
  type Binding = {
    source: str, program: str, semantics: str, relation: str, key: str,
    ledger: str, network: str, anchor: str, address: str, circuitSet: str,
    target: str, actionId: str, assets: int -> AssetId,
  }
  type Authority = {
    signer: str, grossCap: int -> int, feeCap: int -> int,
    netFloor: int -> int, liabilityCap: int -> int,
    sponsor: str, sponsorCap: int, validFrom: int, validUntil: int,
  }
  type Intent = { binding: Binding, authority: Authority }
  type Effect = { debit: int, refund: int, credit: int, fee: int, liability: int }
  type Candidate = {
    binding: Binding, signer: str, predecessor: int, nonce: int, time: int,
    effects: int -> Effect, complete: bool, successor: int,
    signatureVerified: bool, nativeVerified: bool, resourceVerified: bool,
    requiredOps: Set[str], deployedOps: Set[str], encodedBytes: int, replayWork: int,
    dust: int, sponsored: bool, sponsor: str, sponsorBinding: Binding,
    sponsorNonce: int, sponsorVerified: bool,
  }
  type Accounts = { owner: int, recipient: int, treasury: int }
  type Ledger = {
    head: int, balances: int -> Accounts, gross: int -> int, fees: int -> int,
    net: int -> int, liabilities: int -> int, nonces: Set[int], sponsorNonces: Set[int],
    userDust: int, sponsorRemaining: int, sponsorSpent: int,
  }
  type State = { ledger: Ledger, accepts: int, rejects: int, beforeLedger: Ledger, lastRejected: bool, acceptedBindings: bool }
  var state: State

  pure val ASSETS = Set(0, 1)
  pure val MAX_SCALAR = 100
  pure val binding: Binding = {
    source: "source-A", program: "program-A", semantics: "moriarty-core/1",
    relation: "relation-A", key: "key-A", ledger: "ledger-9-pin",
    network: "network-A", anchor: "anchor-A", address: "address-A",
    circuitSet: "set-A", target: "target-A", actionId: "pay",
    assets: Map(0 -> { network: "network-A", issuer: "issuer-A", reference: "coin-A", scale: 0 },
                1 -> { network: "network-A", issuer: "issuer-B", reference: "coin-B", scale: 2 }),
  }
  pure val intent: Intent = { binding: binding, authority: {
    signer: "owner", grossCap: ASSETS.mapBy(_ => 16), feeCap: ASSETS.mapBy(_ => 4),
    netFloor: ASSETS.mapBy(_ => 0), liabilityCap: ASSETS.mapBy(_ => 6),
    sponsor: "sponsor", sponsorCap: 6, validFrom: 0, validUntil: 10,
  } }
  pure val zeroEffect: Effect = { debit: 0, refund: 0, credit: 0, fee: 0, liability: 0 }
  pure val initialLedger: Ledger = {
    head: 0, balances: ASSETS.mapBy(_ => { owner: 20, recipient: 0, treasury: 0 }),
    gross: ASSETS.mapBy(_ => 0), fees: ASSETS.mapBy(_ => 0), net: ASSETS.mapBy(_ => 0),
    liabilities: ASSETS.mapBy(_ => 0), nonces: Set(), sponsorNonces: Set(),
    userDust: 10, sponsorRemaining: 6, sponsorSpent: 0,
  }
  pure def validCandidate(previousHead: int, nonce: int): Candidate = {
    binding: binding, signer: "owner", predecessor: previousHead, nonce: nonce, successor: previousHead + 1,
    time: 1, effects: ASSETS.mapBy(a => if (a == 0) { debit: 4, refund: 0, credit: 3, fee: 1, liability: 0 } else zeroEffect),
    complete: true, signatureVerified: true, nativeVerified: true, resourceVerified: true,
    requiredOps: Set("pay", "replay"), deployedOps: Set("pay", "replay"), encodedBytes: 8, replayWork: 2,
    dust: 1, sponsored: false, sponsor: "sponsor", sponsorBinding: binding, sponsorNonce: nonce, sponsorVerified: true,
  }
  pure def effectValid(l: Ledger, a: int, e: Effect, i: Intent): bool = and {
    e.debit >= 0, e.refund >= 0, e.credit >= 0, e.fee >= 0, e.liability >= 0,
    e.debit <= MAX_SCALAR, e.debit == e.refund + e.credit + e.fee, e.credit >= e.fee,
    l.balances.get(a).owner >= e.debit,
    l.balances.get(a).recipient + e.credit <= MAX_SCALAR,
    l.balances.get(a).treasury + e.fee <= MAX_SCALAR,
    l.gross.get(a) + e.debit <= i.authority.grossCap.get(a),
    l.fees.get(a) + e.fee <= i.authority.feeCap.get(a),
    l.net.get(a) + e.credit - e.fee >= i.authority.netFloor.get(a),
    l.liabilities.get(a) + e.liability <= i.authority.liabilityCap.get(a),
  }
  pure def canAccept(l: Ledger, i: Intent, c: Candidate): bool = and {
    c.binding == i.binding, c.signer == i.authority.signer,
    c.predecessor == l.head, c.successor == l.head + 1, c.nonce >= 0, c.nonce <= 7,
    not(l.nonces.contains(c.nonce)), c.time >= i.authority.validFrom, c.time <= i.authority.validUntil,
    c.complete, c.effects.keys() == ASSETS,
    c.signatureVerified, c.nativeVerified, c.resourceVerified,
    c.requiredOps.subseteq(c.deployedOps), c.deployedOps == Set("pay", "replay"),
    c.encodedBytes > 0, c.encodedBytes <= 64, c.replayWork >= 0, c.replayWork <= 5,
    ASSETS.forall(a => effectValid(l, a, c.effects.get(a), i)), c.dust >= 0,
    if (c.sponsored) and {
      c.sponsorVerified, c.sponsor == i.authority.sponsor, c.sponsorBinding == i.binding,
      c.sponsorNonce >= 0, c.sponsorNonce <= 7, not(l.sponsorNonces.contains(c.sponsorNonce)),
      l.sponsorRemaining >= c.dust, l.sponsorSpent + c.dust <= i.authority.sponsorCap,
    } else l.userDust >= c.dust,
  }
  pure def applyAccept(l: Ledger, c: Candidate): Ledger = {
    ...l, head: c.successor,
    balances: ASSETS.mapBy(a => {
      val b = l.balances.get(a)
      val e = c.effects.get(a)
      { owner: b.owner - e.debit + e.refund, recipient: b.recipient + e.credit, treasury: b.treasury + e.fee }
    }),
    gross: ASSETS.mapBy(a => l.gross.get(a) + c.effects.get(a).debit),
    fees: ASSETS.mapBy(a => l.fees.get(a) + c.effects.get(a).fee),
    net: ASSETS.mapBy(a => l.net.get(a) + c.effects.get(a).credit - c.effects.get(a).fee),
    liabilities: ASSETS.mapBy(a => l.liabilities.get(a) + c.effects.get(a).liability),
    nonces: l.nonces.union(Set(c.nonce)),
    sponsorNonces: if (c.sponsored) l.sponsorNonces.union(Set(c.sponsorNonce)) else l.sponsorNonces,
    userDust: if (c.sponsored) l.userDust else l.userDust - c.dust,
    sponsorRemaining: if (c.sponsored) l.sponsorRemaining - c.dust else l.sponsorRemaining,
    sponsorSpent: if (c.sponsored) l.sponsorSpent + c.dust else l.sponsorSpent,
  }
  pure def recordAccept(s: State, c: Candidate): State = {
    ...s, ledger: applyAccept(s.ledger, c), accepts: s.accepts + 1,
    acceptedBindings: s.acceptedBindings and c.binding == intent.binding, beforeLedger: s.ledger, lastRejected: false,
  }
  pure def recordRefusal(s: State): State = { ...s, rejects: s.rejects + 1, beforeLedger: s.ledger, lastRejected: true }
  action init = state' = { ledger: initialLedger, accepts: 0, rejects: 0, beforeLedger: initialLedger, lastRejected: false, acceptedBindings: true }
  action accept(c: Candidate): bool = all { canAccept(state.ledger, intent, c), state' = recordAccept(state, c) }
  // Observable refusals are separate guarded actions; no acceptance fallback/no-op.
  action refuse(c: Candidate): bool = all { not(canAccept(state.ledger, intent, c)), state' = recordRefusal(state) }
  action step = {
    nondet kind = 0.to(5).oneOf()
    nondet nonce = 0.to(7).oneOf()
    val good = validCandidate(state.ledger.head, nonce)
    val c = if (kind == 0) good else if (kind == 1) { ...good, sponsored: true }
      else if (kind == 2) { ...good, predecessor: 0 }
      else if (kind == 3) { ...good, binding: { ...binding, network: "wrong" } }
      else if (kind == 4) { ...good, complete: false }
      else { ...good, signatureVerified: false }
    any { accept(c), refuse(c) }
  }
  val completeEffectConservation = ASSETS.forall(a => {
    val b = state.ledger.balances.get(a)
    b.owner >= 0 and b.recipient >= 0 and b.treasury >= 0 and b.owner + b.recipient + b.treasury == 20
  })
  val cumulativeAuthority = ASSETS.forall(a => and {
    state.ledger.gross.get(a) <= intent.authority.grossCap.get(a),
    state.ledger.fees.get(a) <= intent.authority.feeCap.get(a),
    state.ledger.net.get(a) == state.ledger.balances.get(a).recipient - state.ledger.balances.get(a).treasury,
    state.ledger.liabilities.get(a) <= intent.authority.liabilityCap.get(a),
  })
  val nonceHeadUnique = state.ledger.head == state.ledger.nonces.size() and state.accepts == state.ledger.head
  val sponsorCap = state.ledger.sponsorSpent + state.ledger.sponsorRemaining == 6 and state.ledger.sponsorRemaining >= 0 and state.ledger.userDust >= 0
  val bindingSafety = state.acceptedBindings
  val refusalAtomic = state.lastRejected implies state.ledger == state.beforeLedger
  val safety = completeEffectConservation and cumulativeAuthority and nonceHeadUnique and sponsorCap and bindingSafety and refusalAtomic
  val acceptedWitness = state.accepts > 0
  val rejectedWitness = state.rejects > 0
  val sponsoredWitness = state.ledger.sponsorSpent > 0
  val competingWitness = state.accepts > 0 and state.rejects > 0
}
