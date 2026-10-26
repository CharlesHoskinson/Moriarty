// U024 finite projection of witness_plan/strategy.rs::evaluate_with_storage.
// Five-node scheduler harness: Read(a), Read(b), Sequence, Sequence, Publish.
// Tests preserve independently specified ordered result and schedule. Lists
// project typed Sequence values to leaves; this is not a complete StepRecord
// or an assertion that this small harness is an admitted Rust snapshot DAG.
module witnessStorage {
  type Storage = RetainAll | LastUse
  type CacheEntry = {arguments: List[List[int]], value: List[int]}
  type Boundary = {control: int, frames: List[int], duties: List[int], fuel: int, work: int}
  type Evaluation = {a: int, b: int, position: int, storage: Storage,
    baseline: int -> List[int], slots: int -> List[int], live: Set[int], peak: int,
    cache: int -> CacheEntry, cached: Set[int], hits: int, reads: int,
    executed: int, scheduled: List[int], reclaimed: int,
    before: Boundary, after: Boundary}
  var evaluation: Evaluation

  pure val nodeIds = 0.to(4)
  // dag::validate includes Snapshot field dependencies, then the output itself.
  pure val liveOuts = Set(3, 4)
  pure val lastUse = Map(0 -> 2, 1 -> 2, 2 -> 3, 3 -> 5, 4 -> 5)
  pure val boundary: Boundary = {control: 3, frames: List(7, 7), duties: List(5), fuel: 0, work: 12}
  pure val warmCache = Map(2 -> {arguments: List(List(1), List(2)), value: List(1, 2)},
    3 -> {arguments: List(List(1, 2), List(1, 2)), value: List(1, 2, 1, 2)},
    4 -> {arguments: List(List(1, 2, 1, 2)), value: List(1, 2, 1, 2)})
  pure def start(a: int, b: int, storage: Storage, warm: bool): Evaluation = {
    a: a, b: b, position: 0, storage: storage, baseline: nodeIds.mapBy(_ => List()),
    slots: nodeIds.mapBy(_ => List()), live: Set(), peak: 0, cache: warmCache,
    cached: if (warm) Set(2, 3, 4) else Set(), hits: 0, reads: 0, executed: 0,
    scheduled: List(), reclaimed: 0, before: boundary, after: boundary}
  pure def arguments(slots: int -> List[int], id: int): List[List[int]] =
    if (id == 2) List(slots.get(0), slots.get(1))
    else if (id == 3) List(slots.get(2), slots.get(2))
    else if (id == 4) List(slots.get(3)) else List()
  pure def operation(s: Evaluation, id: int, args: List[List[int]]): List[int] =
    if (id == 0) List(s.a) else if (id == 1) List(s.b)
    else if (id == 2 or id == 3) args.nth(0).concat(args.nth(1)) else args.nth(0)
  pure def evaluateNode(s: Evaluation): Evaluation = {
    val id = s.position
    val args = arguments(s.slots, id)
    val hit = if (s.cached.contains(id)) s.cache.get(id).arguments == args else false
    val value = if (hit) s.cache.get(id).value else operation(s, id, args)
    val liveNow = s.live.union(Set(id))
    val dead = if (s.storage == LastUse) liveNow.filter(n => not(liveOuts.contains(n)) and lastUse.get(n) == id) else Set()
    val nextSlots = dead.fold(s.slots.set(id, value), (ss, n) => ss.set(n, List()))
    {...s, position: id + 1, baseline: s.baseline.set(id, operation(s, id, arguments(s.baseline, id))),
      slots: nextSlots, live: liveNow.exclude(dead),
      peak: if (s.peak < liveNow.size()) liveNow.size() else s.peak,
      cached: if (id < 2) s.cached else s.cached.union(Set(id)),
      cache: if (id < 2) s.cache else s.cache.set(id, {arguments: args, value: value}),
      hits: s.hits + (if (hit) 1 else 0), reads: s.reads + (if (id < 2) 1 else 0),
      executed: s.executed + (if (hit) 0 else 1), scheduled: s.scheduled.append(id),
      reclaimed: s.reclaimed + dead.size()}
  }
  action init = {
    nondet a = 0.to(3).oneOf()
    nondet b = 0.to(3).oneOf()
    nondet storage = Set(RetainAll, LastUse).oneOf()
    nondet warm = Set(false, true).oneOf()
    evaluation' = start(a, b, storage, warm)
  }
  action tick = all {
    evaluation.position < 5, evaluation' = evaluateNode(evaluation)
  }
  action step = tick
  val published = evaluation.position == 5
  val cacheUsed = evaluation.hits > 0
  val storageReclaimed = evaluation.reclaimed > 0
  val modelInv = and {
    evaluation.after == evaluation.before,
    evaluation.live.forall(id => evaluation.slots.get(id) == evaluation.baseline.get(id)),
    evaluation.scheduled == range(0, evaluation.position),
    evaluation.executed + evaluation.hits == evaluation.position,
    evaluation.reads == (if (evaluation.position < 2) evaluation.position else 2),
    evaluation.live.size() + evaluation.reclaimed == evaluation.position,
    if (published) and {
      evaluation.slots.get(4) == List(evaluation.a, evaluation.b, evaluation.a, evaluation.b),
      liveOuts.subseteq(evaluation.live),
      if (evaluation.storage == LastUse) evaluation.live == liveOuts and evaluation.peak == 3
      else evaluation.live == nodeIds and evaluation.peak == 5
    } else true,
  }
}
