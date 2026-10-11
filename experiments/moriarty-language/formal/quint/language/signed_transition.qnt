// Approved native-owner contract: shared local state, no messages or ledger.
// Hashes/crypto are abstract authentic full-statement premises, not proved here.
module signed_transition {
  type Asset = { network: str, issuer: str, reference: int, scale: int }
  type Duty = { name: int, asset: Asset, debtor: str, creditor: str }
  type Slot = { present: bool, amount: int }
  type Source = { identity: str, assets: int -> Asset, duties: int -> Duty, payers: Set[str] }
  type Ledger = { balances: (str, int) -> int, slots: int -> Slot, paid: int }
  type Caps = { gross: int, fee: int, net: int, duty: int }
  type Policy = { owner: str, beneficiary: str, address: str }
  type Domain = { network: str, anchor: str, target: str, context: str, selectedAction: str }
  type Authority = { policy: Policy, domain: Domain, key: str, nonce: int, predecessor: str, start: int, end: int, limits: int -> Caps }
  type Kind = Transfer | Fee | Accrue | Discharge | StateWrite | Emit
  type Operation = { kind: Kind, asset: Asset, from: str, recipient: str, duty: int, amount: int }
  type Effect = { operation: Operation, before: int, after: int }
  type Statement = { source: Source, opening: Ledger, authority: Authority, operations: List[Operation] }
  type Request = { statement: Statement, signedStatement: Statement, signatureVerified: bool, at: int }
  type Refusal = None | Admission | SourceMismatch | OpeningMismatch | AuthorityMismatch | DomainMismatch | PolicyMismatch | TimeMismatch | SignatureMismatch | Arithmetic | GrossCap | FeeCap | NetFloor | DutyCap
  type State = { source: Source, ledger: Ledger, policy: Policy, authority: Authority, previous: Ledger, effects: List[Effect], previousEffects: List[Effect], last: Refusal, attempts: int, accepted: bool }
  var state: State
  pure val uintMax = 340282366920938463463374607431768211455
  pure def asset(i: int): Asset = { network: "local", issuer: "qa", reference: i, scale: 0 }
  pure def duty(i: int): Duty = { name: i, asset: asset(0), debtor: "alice", creditor: if (i == 0) "bob" else "auditor" }
  pure def uint(n: int): bool = n >= 0 and n <= uintMax
  pure def canonical(slot: Slot): bool = uint(slot.amount) and (not(slot.present) implies slot.amount == 0)
  pure val absent: Slot = { present: false, amount: 0 }
  pure val policy: Policy = { owner: "alice", beneficiary: "bob", address: "local/payment" }
  pure val domain: Domain = { network: "local", anchor: "qa", target: "target3", context: "context3", selectedAction: "settle" }
  pure def initial(a: int, d: int): State = {
    val source: Source = { identity: "source3", assets: 0.to(a - 1).mapBy(i => asset(i)), duties: 0.to(d - 1).mapBy(i => duty(i)), payers: Set("alice") }
    val ledger: Ledger = { balances: Set("alice", "bob", "treasury").map(p => source.assets.keys().map(i => (p, i))).flatten().mapBy(k => if (k._1 == "alice") 100 else if (k._1 == "bob") 10 else 0),
      slots: source.duties.keys().mapBy(i => { present: true, amount: if (i == 0) 50 else if (i == 1) 5 else 0 }), paid: 0 }
    val authority: Authority = { policy: policy, domain: domain, key: "fixed-public-QA-key", nonce: 1, predecessor: "head0", start: 10, end: 20,
      limits: source.assets.keys().mapBy(_ => { gross: 72, fee: 2, net: 0, duty: 55 }) }
    { source: source, ledger: ledger, policy: policy, authority: authority, previous: ledger, effects: List(), previousEffects: List(), last: None, attempts: 0, accepted: false }
  }
  action init = state' = initial(1, 3)
  pure def op(k: Kind, n: int): Operation = { kind: k, asset: asset(0), from: "alice", recipient: if (k == Fee) "treasury" else "bob", duty: 0, amount: n }
  pure val payment = List(op(Transfer, 70), op(Fee, 2), op(Discharge, 20), op(StateWrite, 70), op(Emit, 70))
  pure def request(s: State, ops: List[Operation]): Request = {
    val statement: Statement = { source: s.source, opening: s.ledger, authority: s.authority, operations: ops }
    { statement: statement, signedStatement: statement, signatureVerified: true, at: 15 }
  }
  // Caller-supplied signature boolean is NOT implemented crypto. The premise
  // means an authentic external verifier verified this complete Statement.
  pure def sign(r: Request): Request = { ...r, signedStatement: r.statement }
  pure def admitted(src: Source, p: Policy): bool = and {
    src.assets.keys().size() <= 16, src.duties.keys().size() <= 16,
    src.assets.keys().forall(i => src.assets.get(i).network == domain.network),
    src.assets.keys().forall(i => src.assets.keys().forall(j => i == j or src.assets.get(i) != src.assets.get(j))),
    src.payers.forall(payer => payer == p.owner),
    src.duties.keys().forall(i => src.duties.get(i).name == i and src.duties.get(i).debtor == p.owner
      and src.assets.keys().exists(a => src.assets.get(a) == src.duties.get(i).asset)),
  }
  pure def complete(src: Source, l: Ledger): bool = and {
    l.slots.keys() == src.duties.keys(), l.slots.keys().forall(i => canonical(l.slots.get(i))), uint(l.paid),
    l.balances.keys() == Set("alice", "bob", "treasury").map(p => src.assets.keys().map(i => (p, i))).flatten(),
    l.balances.keys().forall(k => uint(l.balances.get(k))),
  }
  pure def assetIndex(src: Source, a: Asset): int = src.assets.keys().fold(-1, (found, i) => if (src.assets.get(i) == a) i else found)
  pure def canOperation(src: Source, l: Ledger, p: Policy, o: Operation): bool = {
    val a = assetIndex(src, o.asset)
    if (not(uint(o.amount))) false else if (o.kind == StateWrite or o.kind == Emit) true
    else if (a < 0) false else if (o.kind == Transfer or o.kind == Fee) {
      val from = (o.from, a)
      val recipient = (o.recipient, a)
      if (not(l.balances.keys().contains(from) and l.balances.keys().contains(recipient))) false
      else o.from == p.owner and l.balances.get(from) >= o.amount
        and (from == recipient or l.balances.get(recipient) + o.amount <= uintMax)
    } else if (not(src.duties.keys().contains(o.duty))) false else {
      val slot = l.slots.get(o.duty)
      src.duties.get(o.duty).asset == o.asset and src.duties.get(o.duty).debtor == p.owner
        and (if (o.kind == Accrue) slot.amount + o.amount <= uintMax else slot.present and slot.amount >= o.amount)
    }
  }
  pure def applyOperation(src: Source, l: Ledger, o: Operation): Ledger =
    if (o.kind == StateWrite) { ...l, paid: o.amount }
    else if (o.kind == Emit) l
    else if (o.kind == Accrue or o.kind == Discharge) {
      ...l, slots: l.slots.set(o.duty, { present: true, amount: if (o.kind == Accrue) l.slots.get(o.duty).amount + o.amount else l.slots.get(o.duty).amount - o.amount })
    } else {
      val a = assetIndex(src, o.asset)
      val debited = l.balances.set((o.from, a), l.balances.get((o.from, a)) - o.amount)
      { ...l, balances: debited.set((o.recipient, a), debited.get((o.recipient, a)) + o.amount) }
    }
  pure def before(src: Source, l: Ledger, o: Operation): int =
    if (o.kind == Transfer or o.kind == Fee) l.balances.get((o.from, assetIndex(src, o.asset)))
    else if (o.kind == Accrue or o.kind == Discharge) l.slots.get(o.duty).amount else l.paid
  pure def after(l: Ledger, o: Operation): int =
    if (o.kind == Transfer or o.kind == Fee or o.kind == Emit) o.amount
    else if (o.kind == Accrue or o.kind == Discharge) l.slots.get(o.duty).amount else l.paid
  pure def canOperations(src: Source, l: Ledger, p: Policy, ops: List[Operation]): bool =
    ops.foldl({ ok: true, ledger: l }, (r, o) =>
      if (r.ok and canOperation(src, r.ledger, p, o)) { ok: true, ledger: applyOperation(src, r.ledger, o) } else { ...r, ok: false }).ok
  pure def applyOperations(src: Source, l: Ledger, ops: List[Operation]): Ledger = ops.foldl(l, (ledger, o) => applyOperation(src, ledger, o))
  pure def effectChain(src: Source, l: Ledger, ops: List[Operation]): List[Effect] =
    ops.foldl({ ledger: l, effects: List() }, (r, o) => {
      val updated = applyOperation(src, r.ledger, o)
      { ledger: updated, effects: r.effects.append({ operation: o, before: before(src, r.ledger, o), after: after(updated, o) }) }
    }).effects
  type Totals = { outgoing: int, fees: int, credit: int, debit: int, gross: int, net: int, closing: int }
  pure def totals(src: Source, closing: Ledger, p: Policy, ops: List[Operation], a: int): Totals = {
    val sums = ops.foldl({ outgoing: 0, fees: 0, credit: 0, debit: 0 }, (s, o) =>
      if (o.asset != src.assets.get(a)) s else {
        outgoing: s.outgoing + (if (o.kind == Transfer and o.from == p.owner) o.amount else 0),
        fees: s.fees + (if (o.kind == Fee and o.from == p.owner) o.amount else 0),
        credit: s.credit + (if (o.kind == Transfer and o.recipient == p.beneficiary) o.amount else 0),
        debit: s.debit + (if (o.from == p.beneficiary and (o.kind == Transfer or (o.kind == Fee and p.beneficiary != p.owner))) o.amount else 0),
      })
    { outgoing: sums.outgoing, fees: sums.fees, credit: sums.credit, debit: sums.debit, gross: sums.outgoing + sums.fees, net: sums.credit - sums.debit - sums.fees,
      closing: src.duties.keys().fold(0, (n, d) => n + (if (src.duties.get(d).asset == src.assets.get(a) and src.duties.get(d).debtor == p.owner) closing.slots.get(d).amount else 0)) }
  }
  pure def checked(t: Totals): bool = uint(t.outgoing) and uint(t.fees) and uint(t.credit) and uint(t.debit) and uint(t.gross) and uint(t.net) and uint(t.closing)
  pure def reason(s: State, r: Request): Refusal = {
    val st = r.statement
    val a = st.authority
    if (not(admitted(st.source, s.policy))) Admission
    else if (st.source != s.source) SourceMismatch
    else if (not(complete(st.source, st.opening)) or st.opening != s.ledger) OpeningMismatch
    else if (a.policy != s.policy) PolicyMismatch
    else if (a.domain != s.authority.domain) DomainMismatch
    else if (a != s.authority or a.limits.keys() != s.source.assets.keys()
      or not(a.limits.keys().forall(i => { val c = a.limits.get(i); uint(c.gross) and uint(c.fee) and uint(c.net) and uint(c.duty) }))) AuthorityMismatch
    else if (not(uint(r.at) and uint(a.start) and uint(a.end)) or a.start > a.end or r.at < a.start or r.at > a.end) TimeMismatch
    else if (not(r.signatureVerified) or r.signedStatement != st) SignatureMismatch
    else if (not(canOperations(s.source, s.ledger, s.policy, st.operations))) Arithmetic
    else {
      val closing = applyOperations(s.source, s.ledger, st.operations)
      val sums = s.source.assets.keys().mapBy(i => totals(s.source, closing, s.policy, st.operations, i))
      if (not(sums.keys().forall(i => checked(sums.get(i))))) Arithmetic
      else if (sums.keys().exists(i => sums.get(i).gross > a.limits.get(i).gross)) GrossCap
      else if (sums.keys().exists(i => sums.get(i).fees > a.limits.get(i).fee)) FeeCap
      else if (sums.keys().exists(i => sums.get(i).net < a.limits.get(i).net)) NetFloor
      else if (sums.keys().exists(i => sums.get(i).closing > a.limits.get(i).duty)) DutyCap else None
    }
  }
  pure def canAccept(s: State, r: Request): bool = reason(s, r) == None
  pure def applyAccept(s: State, r: Request): State = { ...s,
    ledger: applyOperations(s.source, s.ledger, r.statement.operations), previous: s.ledger,
    effects: effectChain(s.source, s.ledger, r.statement.operations), previousEffects: s.effects,
    last: None, attempts: s.attempts + 1, accepted: true }
  pure def recordRefusal(s: State, r: Request): State = { ...s, previous: s.ledger, previousEffects: s.effects,
    last: reason(s, r), attempts: s.attempts + 1, accepted: false }
  action accept(r: Request): bool = all { state.attempts < 12, canAccept(state, r), state' = applyAccept(state, r) }
  action refuse(r: Request): bool = all { state.attempts < 12, not(canAccept(state, r)), state' = recordRefusal(state, r) }
  action step = {
    nondet mode = 0.to(12).oneOf()
    val ops = if (mode == 0) payment else if (mode == 1) List(op(StateWrite, state.ledger.paid))
      else if (mode == 2) List({ ...op(Transfer, 1), recipient: "alice" })
      else if (mode == 3) List(op(Discharge, 1))
      else if (mode == 4) List({ ...op(Accrue, 0), duty: 2 })
      else if (mode == 5) List(op(Discharge, state.ledger.slots.get(0).amount))
      else List(op(Transfer, 0))
    val base = request(state, ops)
    val r = if (mode == 6) { ...base, at: 21 }
      else if (mode == 7) { ...base, signatureVerified: false }
      else if (mode == 8) { ...base, statement: { ...base.statement, authority: { ...state.authority, key: "wrong" } } }
      else if (mode == 9) { ...base, statement: { ...base.statement, source: { ...state.source, payers: Set("mallory") } } }
      else if (mode == 10) { ...base, statement: { ...base.statement, source: { ...state.source, identity: "other" } } }
      else if (mode == 11) { ...base, statement: { ...base.statement, authority: { ...state.authority, domain: { ...domain, anchor: "other" } } } }
      else if (mode == 12) { ...base, statement: { ...base.statement, authority: { ...state.authority, policy: { ...policy, owner: "mallory" } } } }
      else base
    any { accept(r), refuse(r) }
  }
  val refusalAtomic = state.last != None implies state.ledger == state.previous and state.effects == state.previousEffects and not(state.accepted)
  val conservation = state.source.assets.keys().forall(a => state.ledger.balances.keys().filter(k => k._2 == a).fold(0, (n, k) => n + state.ledger.balances.get(k)) == state.previous.balances.keys().filter(k => k._2 == a).fold(0, (n, k) => n + state.previous.balances.get(k)))
  val persistence = state.previous.slots.keys().forall(d => state.previous.slots.get(d).present implies state.ledger.slots.get(d).present)
  val orderedEffects = state.accepted implies {
    val ops = state.effects.foldl(List(), (l, e) => l.append(e.operation))
    canOperations(state.source, state.previous, state.policy, ops)
      and state.effects == effectChain(state.source, state.previous, ops)
      and state.ledger == applyOperations(state.source, state.previous, ops)
  }
  val authorityCaps = state.accepted implies state.source.assets.keys().forall(a => {
    val t = totals(state.source, state.ledger, state.policy, state.effects.foldl(List(), (l, e) => l.append(e.operation)), a)
    val c = state.authority.limits.get(a)
    checked(t) and t.gross <= c.gross and t.fees <= c.fee and t.net >= c.net and t.closing <= c.duty
  })
  val safety = admitted(state.source, state.policy) and complete(state.source, state.ledger) and refusalAtomic and conservation and persistence and orderedEffects and authorityCaps
  val acceptedPaymentWitness = state.accepted and state.effects.foldl(List(), (l, e) => l.append(e.operation)) == payment
  val stateOnlyWitness = state.accepted and state.effects.length() == 1 and state.effects.head().operation.kind == StateWrite
  val selfTransferWitness = state.accepted and state.effects.length() == 1 and state.effects.head().operation.kind == Transfer and state.effects.head().operation.from == state.effects.head().operation.recipient
  val partialWitness = state.accepted and state.effects.length() == 1 and state.effects.head().operation.kind == Discharge and state.ledger.slots.get(0).amount > 0
  val fullWitness = state.accepted and state.effects.length() == 1 and state.effects.head().operation.kind == Discharge and state.effects.head().before > 0 and state.ledger.slots.get(0).amount == 0
  val presentZeroWitness = state.accepted and state.effects.length() == 1 and state.effects.head().operation.kind == Accrue and state.ledger.slots.get(2) == { present: true, amount: 0 }
  val refusalWitness = state.last != None
}
