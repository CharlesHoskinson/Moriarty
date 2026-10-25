// Source-derived counterexample: cfbab29c5 models/sherlock/qa/residualize.py.
// This faithfully models the arith-family static-dispatch filtering and FIFO
// finite worklist. The contract invariant intentionally FAILS: control remains
// dynamic, so pruning a reference successor needs a guard or generic fallback.
module s5ResidualProjection {
  type Mask = Empty | ArithFamily | RecursiveFamily | Image
  type Preparation = {mask: Mask, limit: int, pending: List[int], seen: Set[int],
    blocks: Set[int], generic: Set[int], targets: int -> Set[int]}
  var prep: Preparation
  // R_fetch, R_cond, R_then, R_else, R_call in source declaration order.
  pure val locations = 0.to(4)
  pure val reference = Map(0 -> Set(1, 4), 1 -> Set(2, 3), 2 -> Set(), 3 -> Set(), 4 -> Set(0))
  pure val sourceTargets = Map(0 -> List(1, 4), 1 -> List(2, 3),
    2 -> List(), 3 -> List(), 4 -> List(0))
  pure def start(mask: Mask, limit: int): Preparation = {
    mask: mask, limit: limit, pending: List(0), seen: Set(), blocks: Set(), generic: Set(),
    targets: locations.mapBy(_ => Set())}
  pure def process(s: Preparation): Preparation = {
    val key = s.pending.head()
    val rest = s.pending.tail()
    if (s.seen.contains(key)) {...s, pending: rest}
    else if (s.blocks.size() >= s.limit) {...s, pending: rest,
      seen: s.seen.union(Set(key)), generic: s.generic.union(Set(key))}
    else {
      val successors = if (key == 0 and s.mask == ArithFamily) List(1) else sourceTargets.get(key)
      {...s, pending: rest.concat(successors), seen: s.seen.union(Set(key)),
        blocks: s.blocks.union(Set(key)), targets: s.targets.set(key, successors.indices().map(ix => successors.nth(ix)))}
    }
  }
  action init = {
    nondet mask = Set(Empty, ArithFamily, RecursiveFamily, Image).oneOf()
    prep' = start(mask, 5)
  }
  action tick = all {prep.pending.length() > 0, prep' = process(prep)}
  action step = tick
  val preservesDynamicTargets = prep.blocks.forall(key => reference.get(key).subseteq(prep.targets.get(key)))
  val sourceInv = and {
    prep.blocks.size() <= prep.limit,
    prep.seen == prep.blocks.union(prep.generic),
    prep.blocks.intersect(prep.generic) == Set(),
    prep.seen.subseteq(locations),
    prep.pending.indices().forall(ix => locations.contains(prep.pending.nth(ix))),
  }
  val lostCallSuccessor = prep.blocks.contains(0) and not(prep.targets.get(0).contains(4))
  val finished = prep.pending.length() == 0
}
