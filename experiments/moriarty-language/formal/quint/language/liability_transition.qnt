// One atomic local stage. No crypto, ledger, time or messaging premises.
module liability_transition {
  type Identity = { name: int, network: str, issuer: str, reference: str, scale: int, debtor: str, creditor: str }
  type Slot = { presence: int, amount: int }
  type Row = { identity: Identity, slot: Slot }
  type Kind = Accrue | Discharge
  type Operation = { identity: Identity, kind: Kind, delta: int, stepId: int }
  type Effect = { operation: Operation, before: Slot, after: Slot }
  type Request = { declarations: List[Identity], frame: List[Row], operations: List[Operation], descriptorSteps: List[int] }
  type State = { registry: int -> Identity, slots: int -> Slot, previous: int -> Slot, effects: List[Effect], previousEffects: List[Effect], lastOperations: List[Operation], accepted: int, refused: int, lastRefused: bool }
  var state: State
  pure val uintMax = 340282366920938463463374607431768211455
  pure def identity(n: int): Identity = { name: n, network: "local", issuer: "qa", reference: "UNIT", scale: 0, debtor: "alice", creditor: "bob" }
  pure val absent: Slot = { presence: 0, amount: 0 }
  pure def initial(n: int): State = {
    registry: 0.to(n - 1).mapBy(i => identity(i)),
    slots: 0.to(n - 1).mapBy(i => if (i == 0) { presence: 1, amount: 10 } else absent),
    previous: 0.to(n - 1).mapBy(i => if (i == 0) { presence: 1, amount: 10 } else absent),
    effects: List(), previousEffects: List(), lastOperations: List(), accepted: 0, refused: 0, lastRefused: false,
  }
  action init = state' = initial(2)
  pure def op(n: int, kind: Kind, delta: int, stepId: int): Operation = { identity: identity(n), kind: kind, delta: delta, stepId: stepId }
  pure def request(s: State, operations: List[Operation]): Request = {
    declarations: s.registry.keys().fold(List(), (l, i) => l.append(s.registry.get(i))),
    frame: s.registry.keys().fold(List(), (l, i) => l.append({ identity: s.registry.get(i), slot: s.slots.get(i) })),
    operations: operations, descriptorSteps: operations.foldl(List(), (l, o) => l.append(o.stepId)),
  }
  pure def canonical(slot: Slot): bool = Set(0, 1).contains(slot.presence) and slot.amount >= 0 and slot.amount <= uintMax and (slot.presence == 0 implies slot.amount == 0)
  // Source declarations may repeat exactly; scenario rows must never duplicate.
  pure def sourceAdmitted(ds: List[Identity]): bool = {
    val names = ds.foldl(Set(), (ns, d) => ns.union(Set(d.name)))
    names.size() <= 16 and ds.foldl(true, (ok, d) => ok and ds.foldl(true, (same, e) => same and (d.name == e.name implies d == e)))
  }
  pure def contextAdmitted(s: State, r: Request): bool = {
    val names = r.declarations.foldl(Set(), (ns, d) => ns.union(Set(d.name)))
    val frameNames = r.frame.foldl(Set(), (ns, row) => ns.union(Set(row.identity.name)))
    and {
      sourceAdmitted(r.declarations), names == s.registry.keys(),
      r.declarations.foldl(true, (ok, d) => ok and (if (s.registry.keys().contains(d.name)) d == s.registry.get(d.name) else false)),
      frameNames == names, r.frame.length() == names.size(),
      r.frame.foldl(true, (ok, row) => ok and (if (names.contains(row.identity.name))
        row.identity == s.registry.get(row.identity.name) and canonical(row.slot) and row.slot == s.slots.get(row.identity.name) else false)),
      r.descriptorSteps == r.operations.foldl(List(), (l, o) => l.append(o.stepId)),
      r.operations.foldl({ ok: true, prev: -1 }, (a, o) => { ok: a.ok and o.stepId > a.prev, prev: o.stepId }).ok,
    }
  }
  pure def canOperation(registry: int -> Identity, slots: int -> Slot, o: Operation): bool =
    if (not(registry.keys().contains(o.identity.name))) false else {
      val slot = slots.get(o.identity.name)
      and { o.identity == registry.get(o.identity.name), o.delta >= 0, o.delta <= uintMax,
        if (o.kind == Accrue) slot.amount + o.delta <= uintMax else slot.presence == 1 and slot.amount >= o.delta,
      }
    }
  pure def applyOperation(slots: int -> Slot, o: Operation): int -> Slot = {
    val slot = slots.get(o.identity.name)
    slots.set(o.identity.name, { presence: 1, amount: if (o.kind == Accrue) slot.amount + o.delta else slot.amount - o.delta })
  }
  pure def canStage(s: State, r: Request): bool = contextAdmitted(s, r) and
    r.operations.foldl({ valid: true, slots: s.slots }, (a, o) =>
      if (a.valid and canOperation(s.registry, a.slots, o)) { valid: true, slots: applyOperation(a.slots, o) }
      else { ...a, valid: false }).valid
  pure def applyStage(s: State, r: Request): State = {
    val result = r.operations.foldl({ slots: s.slots, effects: List() }, (a, o) => {
      val updated = applyOperation(a.slots, o)
      { slots: updated, effects: a.effects.append({ operation: o, before: a.slots.get(o.identity.name), after: updated.get(o.identity.name) }) }
    })
    { ...s, slots: result.slots, previous: s.slots, effects: result.effects, previousEffects: s.effects,
      lastOperations: r.operations, accepted: s.accepted + 1, lastRefused: false }
  }
  pure def recordRefusal(s: State): State = { ...s, previous: s.slots, previousEffects: s.effects, refused: s.refused + 1, lastRefused: true }
  action accept(r: Request): bool = all { canStage(state, r), state' = applyStage(state, r) }
  action refuse(r: Request): bool = all { not(canStage(state, r)), state' = recordRefusal(state) }
  action step = {
    nondet n = Set(0, 1).oneOf()
    nondet kind = Set(Accrue, Discharge).oneOf()
    nondet delta = 0.to(4).oneOf()
    nondet fault = 0.to(5).oneOf()
    val base = request(state, List(op(n, kind, delta, 0)))
    val r = if (fault == 0) { ...base, descriptorSteps: List(9) }
      else if (fault == 1) { ...base, frame: base.frame.append(base.frame.head()) }
      else base
    any { accept(r), refuse(r) }
  }
  val exactClosure = state.slots.keys() == state.registry.keys() and state.registry.keys().size() <= 16
    and state.registry.keys().forall(n => state.registry.get(n) == identity(n))
  val canonicalFrame = state.slots.keys().forall(n => canonical(state.slots.get(n)))
  val persistence = state.previous.keys().forall(n => state.previous.get(n).presence == 1 implies state.slots.get(n).presence == 1)
  val refusalAtomic = state.lastRefused implies state.slots == state.previous and state.effects == state.previousEffects
  val untouched = state.previous.keys().forall(n =>
    not(state.lastOperations.foldl(Set(), (ns, o) => ns.union(Set(o.identity.name))).contains(n)) implies state.slots.get(n) == state.previous.get(n))
  // Independently check the full effect chain from the last accepted opening.
  val orderedEffects = state.lastRefused or {
    val checked = state.effects.foldl({ ok: true, slots: state.previous, steps: List() }, (a, e) => {
      val n = e.operation.identity.name
      { ok: a.ok and e.before == a.slots.get(n) and e.after.presence == 1
          and e.after.amount == (if (e.operation.kind == Accrue) e.before.amount + e.operation.delta else e.before.amount - e.operation.delta)
          and canOperation(state.registry, a.slots, e.operation),
        slots: a.slots.set(n, e.after), steps: a.steps.append(e.operation) }
    })
    checked.ok and checked.slots == state.slots and checked.steps == state.lastOperations
  }
  val safety = exactClosure and canonicalFrame and persistence and refusalAtomic and untouched and orderedEffects
  val accruedWitness = state.accepted > 0 and state.effects.foldl(false, (yes, e) => yes or e.operation.kind == Accrue)
  val dischargedWitness = state.accepted > 0 and state.effects.foldl(false, (yes, e) => yes or e.operation.kind == Discharge)
  val partialWitness = state.accepted > 0 and state.effects.foldl(false, (yes, e) => yes or (e.operation.kind == Discharge and e.operation.delta > 0 and e.after.amount > 0))
  val fullWitness = state.accepted > 0 and state.effects.foldl(false, (yes, e) => yes or (e.operation.kind == Discharge and e.before.amount > 0 and e.after.amount == 0))
  val zeroCreationWitness = state.accepted > 0 and state.effects.foldl(false, (yes, e) => yes or (e.operation.kind == Accrue and e.operation.delta == 0 and e.before.presence == 0))
  val refusedWitness = state.refused > 0
}
