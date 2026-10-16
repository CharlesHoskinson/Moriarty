// Proposed P3 append relation only. Source draft; all execution UNRUN.
module valueSlotsAppend {
  // Space tags are typed identities, not semantic Value equality.
  type ValueRef = Constant(int) | Data(int)
  type Slice = {start: int, length: int}
  type Arena = {capacity: int, active: int, slots: List[ValueRef], holders: List[Slice]}
  type Mode = EXT | COPY
  type Event = Initial
    | Applied({holder: int, value: ValueRef, mode: Mode})
    | ToolingCapacity({holder: int, value: ValueRef, required: int,
        limit: int, firstUnavailable: int})
  type State = {arena: Arena, previous: Arena, event: Event}
  var s: State

  // Finite QA dimensions ONLY; no production defaults or capacity admission.
  pure val refs: Set[ValueRef] = Set(Constant(0), Constant(1), Data(0), Data(1))
  pure val capacities = Set(7, 8, 9)
  pure val zero: ValueRef = Constant(0)
  pure def normalizedSlice(start: int, sliceLength: int): Slice =
    if (sliceLength == 0) {start: 0, length: 0} else {start: start, length: sliceLength}
  pure def validSlice(a: Arena, p: Slice): bool =
    if (p.length == 0) p.start == 0
    else p.length > 0 and p.start >= 0 and p.start + p.length <= a.active
  pure def wellFormed(a: Arena): bool = and {
    capacities.contains(a.capacity), a.active >= 0, a.active <= a.capacity,
    a.slots.length() == a.capacity, a.holders.length() == 2,
    a.holders.indices().forall(h => validSlice(a, a.holders.nth(h))),
    a.slots.indices().forall(i => if (i < a.active) refs.contains(a.slots.nth(i))
      else a.slots.nth(i) == zero)
  }

  // Declarative sequence view; used by properties, never by physicalAppend.
  pure def materialize(a: Arena, p: Slice): List[ValueRef] =
    range(0, p.length).foldl(List(), (out, i) => out.append(a.slots.nth(p.start + i)))
  pure def referenceAppend(a: Arena, h: int, v: ValueRef): List[ValueRef] =
    materialize(a, a.holders.nth(h)).append(v)

  pure def selectedMode(a: Arena, h: int): Mode = {
    val p = a.holders.nth(h)
    if (p.length > 0 and p.start + p.length == a.active) EXT else COPY
  }
  pure def requiredLength(a: Arena, h: int): int =
    a.active + (if (selectedMode(a, h) == EXT) 1 else a.holders.nth(h).length + 1)

  // Precondition: wellFormed(a), h in {0,1}, valid v, requiredLength <= capacity.
  // Each destination row is calculated from the preimage only. No materialized
  // oracle sequence or postimage is supplied to this physical computation.
  pure def physicalAppend(a: Arena, h: int, v: ValueRef): Arena = {
    val p = a.holders.nth(h)
    val mode = selectedMode(a, h)
    val end = requiredLength(a, h)
    val rows = range(0, a.capacity).foldl(List(), (out, i) => out.append(
      if (i < a.active) a.slots.nth(i)
      else if (i >= end) zero
      else if (mode == EXT or i == a.active + p.length) v
      else a.slots.nth(p.start + i - a.active)))
    val nextSlice = {start: if (mode == EXT) p.start else a.active, length: p.length + 1}
    {capacity: a.capacity, active: end, slots: rows,
      holders: a.holders.replaceAt(h, nextSlice)}
  }

  action initArena(a: Arena): bool = all {
    wellFormed(a),
    s' = {arena: a, previous: a, event: Initial}
  }

  // Arbitrary valid small Step slices, including aliases/non-suffixes, without
  // any reachability or call-arity restriction. All nondeterminism is finite.
  action init = {
    nondet cap = capacities.oneOf()
    nondet used = 0.to(cap).oneOf()
    nondet cells = 0.to(8).setOfMaps(refs).oneOf()
    nondet start0 = 0.to(used).oneOf()
    nondet length0 = 0.to(used - start0).oneOf()
    nondet start1 = 0.to(used).oneOf()
    nondet length1 = 0.to(used - start1).oneOf()
    val rows = range(0, cap).foldl(List(), (out, i) =>
      out.append(if (i < used) cells.get(i) else zero))
    initArena({capacity: cap, active: used, slots: rows,
      holders: List(normalizedSlice(start0, length0), normalizedSlice(start1, length1))})
  }

  // One atomic append. Capacity is a tooling diagnostic outside Arena; it
  // leaves every arena cell and holder unchanged, not Error/Exhausted/false.
  action appendTo(h: int, v: ValueRef): bool = {
    val need = requiredLength(s.arena, h)
    all {
      Set(0, 1).contains(h), refs.contains(v), wellFormed(s.arena),
      s' = if (need <= s.arena.capacity)
      {arena: physicalAppend(s.arena, h, v), previous: s.arena,
        event: Applied({holder: h, value: v, mode: selectedMode(s.arena, h)})}
    else
      {arena: s.arena, previous: s.arena,
        event: ToolingCapacity({holder: h, value: v, required: need,
          limit: s.arena.capacity, firstUnavailable: s.arena.capacity})}
    }
  }
  action step = {
    nondet h = Set(0, 1).oneOf()
    nondet v = refs.oneOf()
    appendTo(h, v)
  }

  val representationInv = wellFormed(s.arena)
  val appendInv = match s.event {
    | Initial => s.arena == s.previous
    | Applied(e) => and {
        s.arena.capacity == s.previous.capacity,
        e.mode == selectedMode(s.previous, e.holder),
        s.arena.active == requiredLength(s.previous, e.holder),
        s.arena.holders.nth(e.holder) == {
          start: if (e.mode == EXT) s.previous.holders.nth(e.holder).start else s.previous.active,
          length: s.previous.holders.nth(e.holder).length + 1},
        range(0, s.previous.active).indices().forall(i =>
          s.arena.slots.nth(i) == s.previous.slots.nth(i)),
        materialize(s.arena, s.arena.holders.nth(e.holder)) ==
          referenceAppend(s.previous, e.holder, e.value),
        s.arena.holders.nth(1 - e.holder) == s.previous.holders.nth(1 - e.holder),
        materialize(s.arena, s.arena.holders.nth(1 - e.holder)) ==
          materialize(s.previous, s.previous.holders.nth(1 - e.holder))
      }
    | ToolingCapacity(e) => and {
        s.arena == s.previous,
        e.required == requiredLength(s.previous, e.holder),
        e.required > e.limit, e.limit == s.previous.capacity,
        e.firstUnavailable == s.previous.capacity
      }
  }
  val modelInv = representationInv and appendInv
}
