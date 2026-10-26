// U033 REQUIREMENTS MODEL; no residualizer exists at the inspected Rust HEAD.
// The finite graph abstracts dispatch -> dynamic branch -> call/terminal.
// Calls point back to dispatch; depth, fuel and values never enter work keys.
// Atomic grain: one memoized worklist item. Generic dispatch covers all
// successors of a fallback block. This models preparation, not execution.
module residualWorklist {
  type Mask = Empty | FixedFamily | FixedImage
  type Dynamic = {control: int, value: int, environment: List[int], frames: List[int],
    fuel: int, work: int, helperCursor: int, outcome: int, duties: List[int]}
  type Preparation = {mask: Mask, limit: int, pending: Set[int], seen: Set[int],
    blocks: Set[int], generic: Set[int], before: Dynamic, after: Dynamic}
  var prep: Preparation

  pure val locations = Set(0, 1, 2, 3)
  pure val edges = Map(0 -> Set(1), 1 -> Set(2, 3), 2 -> Set(0), 3 -> Set())
  pure val sampleDynamic: Dynamic = {control: 2, value: 7, environment: List(1, 1),
    frames: List(4, 4), fuel: 0, work: 9, helperCursor: 3, outcome: 0, duties: List(5)}
  pure def start(mask: Mask, limit: int, dynamic: Dynamic): Preparation = {
    mask: mask, limit: limit, pending: Set(0), seen: Set(), blocks: Set(),
    generic: Set(), before: dynamic, after: dynamic}
  pure def process(s: Preparation, key: int): Preparation = {
    val visited = s.seen.union(Set(key))
    if (s.blocks.size() < s.limit) {
      ...s, seen: visited, blocks: s.blocks.union(Set(key)),
      pending: s.pending.exclude(Set(key)).union(edges.get(key).exclude(visited))
    } else {
      ...s, seen: visited, generic: s.generic.union(Set(key)), pending: s.pending.exclude(Set(key))
    }
  }
  // Static graph reachability including a generic block's transitive coverage.
  pure def closure(roots: Set[int]): Set[int] = 0.to(3).fold(roots,
    (acc, _) => acc.union(acc.fold(Set(), (ns, key) => ns.union(edges.get(key)))))
  pure def valid(s: Preparation): bool = and {
    s.limit >= 0, s.blocks.size() <= s.limit,
    s.blocks.intersect(s.generic) == Set(), s.seen == s.blocks.union(s.generic),
    s.pending.intersect(s.seen) == Set(), s.seen.union(s.pending).subseteq(locations),
    s.after == s.before,
    // Both dynamic branch successors must survive unless generic dispatch owns them.
    s.blocks.forall(key => edges.get(key).subseteq(s.seen.union(s.pending))),
    locations.subseteq(s.blocks.union(s.pending).union(closure(s.generic)).union(closure(s.pending))),
  }
  action init = {
    nondet mask = Set(Empty, FixedFamily, FixedImage).oneOf()
    nondet limit = 0.to(4).oneOf()
    prep' = start(mask, limit, sampleDynamic)
  }
  action visit(key: int): bool = all {
    prep.pending.contains(key), prep' = process(prep, key)
  }
  action step = {
    nondet key = prep.pending.oneOf()
    visit(key)
  }
  val modelInv = valid(prep)
  val prepared = prep.pending.size() == 0
  val hitFallback = prep.generic.size() > 0
  val compiledBranch = prep.blocks.contains(1)
  val retainedRecursiveCall = prep.blocks.contains(2)
  val fullSpecialization = prep.blocks == locations
}
