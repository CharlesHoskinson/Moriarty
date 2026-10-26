// U033 source projection of a82446698 residualize.py (repair of cfbab29c5).
// All control targets remain; targets outside the static family are delegated
// to generic dispatch without adding specialized blocks for those targets.
module s5ResidualRepaired {
  type Mask = Empty | ArithFamily | RecursiveFamily | Image
  type Preparation = {mask: Mask, limit: int, pending: List[int], seen: Set[int],
    blocks: Set[int], generic: Set[int], targets: int -> Set[int], delegated: Set[int]}
  var prep: Preparation
  pure val locations = 0.to(4)
  pure val reference = Map(0 -> Set(1, 4), 1 -> Set(2, 3), 2 -> Set(), 3 -> Set(), 4 -> Set(0))
  pure val sourceTargets = Map(0 -> List(1, 4), 1 -> List(2, 3),
    2 -> List(), 3 -> List(), 4 -> List(0))
  pure def start(mask: Mask, limit: int): Preparation = {
    mask: mask, limit: limit, pending: List(0), seen: Set(), blocks: Set(), generic: Set(),
    targets: locations.mapBy(_ => Set()), delegated: Set()}
  pure def outsideFamily(s: Preparation, key: int): bool = s.mask == ArithFamily and key == 4
  pure def process(s: Preparation): Preparation = {
    val key = s.pending.head()
    val rest = s.pending.tail()
    if (s.seen.contains(key)) {...s, pending: rest}
    else if (s.blocks.size() >= s.limit) {...s, pending: rest,
      seen: s.seen.union(Set(key)), generic: s.generic.union(Set(key))}
    else {
      val successors = sourceTargets.get(key)
      val external = reference.get(key).filter(target => outsideFamily(s, target))
      {...s, pending: rest.concat(successors.select(target => not(outsideFamily(s, target)))),
        seen: s.seen.union(Set(key)), blocks: s.blocks.union(Set(key)),
        targets: s.targets.set(key, reference.get(key)), delegated: s.delegated.union(external)}
    }
  }
  pure def closure(roots: Set[int]): Set[int] = 0.to(4).fold(roots,
    (acc, _) => acc.union(acc.fold(Set(), (nodes, key) => nodes.union(reference.get(key)))))
  action init = {
    nondet mask = Set(Empty, ArithFamily, RecursiveFamily, Image).oneOf()
    nondet limit = 0.to(5).oneOf()
    prep' = start(mask, limit)
  }
  action tick = all {prep.pending.length() > 0, prep' = process(prep)}
  action step = tick
  val finished = prep.pending.length() == 0
  val delegatedCall = prep.delegated.contains(4)
  val sizeFallback = prep.generic.size() > 0
  val fullImage = finished and prep.mask == Image and prep.blocks == locations
  val modelInv = {
    val pending = prep.pending.indices().map(ix => prep.pending.nth(ix))
    and {
      prep.blocks.size() <= prep.limit,
      prep.seen == prep.blocks.union(prep.generic),
      prep.blocks.intersect(prep.generic) == Set(),
      prep.delegated.intersect(prep.blocks) == Set(),
      prep.blocks.forall(key => reference.get(key) == prep.targets.get(key)),
      prep.blocks.forall(key => reference.get(key).subseteq(prep.seen.union(pending).union(prep.delegated))),
      locations.subseteq(prep.blocks.union(closure(pending.union(prep.generic).union(prep.delegated)))),
      if (prep.mask == ArithFamily) not(prep.blocks.contains(4)) else prep.delegated == Set(),
    }
  }
}
