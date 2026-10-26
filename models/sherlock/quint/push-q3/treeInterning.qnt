// U024 source projection: 69aeb882d optimization_intern.rs::intern_tree.
// An expanded, postorder seven-occurrence tree replaces Rust's explicit
// traversal stack. Leaf keys include the full payload; branch keys contain
// the ordered already-interned child handles. Addresses are opaque integers.
// This checks the representation transformation, not full machine traces.
module treeInterning {
  type Node = Leaf(int) | Branch({left: int, right: int})
  type Boundary = {fuel: int, initialFuel: int, frames: List[int], control: int, outcome: int}
  type Interner = {input: List[Node], position: int, nodes: List[Node],
    handles: int -> int, before: Boundary, after: Boundary}
  var interner: Interner
  pure val boundary: Boundary = {fuel: 20, initialFuel: 512, frames: List(3, 3), control: 1, outcome: 0}
  pure def start(a: int, b: int, reversed: bool): Interner = {
    input: List(Leaf(a), Leaf(b), Branch({left: 0, right: 1}),
      Leaf(if (reversed) b else a), Leaf(if (reversed) a else b),
      Branch({left: 3, right: 4}), Branch({left: 2, right: 5})),
    position: 0, nodes: List(), handles: 0.to(6).mapBy(_ => -1), before: boundary, after: boundary}
  pure def key(s: Interner, shapeOnly: bool): Node = match s.input.nth(s.position) {
    | Leaf(value) => Leaf(if (shapeOnly) 0 else value)
    | Branch(children) => Branch({left: s.handles.get(children.left), right: s.handles.get(children.right)})
  }
  pure def lookup(nodes: List[Node], wanted: Node): int =
    nodes.indices().fold(-1, (found, ix) => if (nodes.nth(ix) == wanted) ix else found)
  pure def intern(s: Interner, shapeOnly: bool): Interner = {
    val wanted = key(s, shapeOnly)
    val existing = lookup(s.nodes, wanted)
    val handle = if (existing >= 0) existing else s.nodes.length()
    {...s, position: s.position + 1,
      nodes: if (existing >= 0) s.nodes else s.nodes.append(wanted),
      handles: s.handles.set(s.position, handle)}
  }
  // Prefix encoding is structural and retains ordered multiplicity even when
  // both branch handles share one physical node. It is independent of keys.
  pure def meanings(nodes: List[Node]): List[List[int]] =
    nodes.foldl(List(), (decoded, node) => decoded.append(match node {
      | Leaf(value) => List(0, value)
      | Branch(children) => List(1).concat(decoded.nth(children.left)).concat(decoded.nth(children.right))
    }))
  pure def sums(nodes: List[Node]): List[int] =
    nodes.foldl(List(), (decoded, node) => decoded.append(match node {
      | Leaf(value) => value
      | Branch(children) => decoded.nth(children.left) + decoded.nth(children.right)
    }))
  action init = {
    nondet a = Set(0, 1, 4, 5).oneOf()
    nondet b = Set(0, 1, 4, 5).oneOf()
    nondet reversed = Set(false, true).oneOf()
    interner' = start(a, b, reversed)
  }
  action tick = all {interner.position < 7, interner' = intern(interner, false)}
  action step = tick
  val finished = interner.position == 7
  val shared = interner.nodes.length() < interner.position
  val fourNodes = finished and interner.nodes.length() == 4
  val modelInv = and {
    interner.after == interner.before,
    interner.nodes.length() <= interner.position,
    interner.nodes.indices().forall(ix => match interner.nodes.nth(ix) {
      | Leaf(_) => true
      | Branch(children) => children.left >= 0 and children.left < ix and children.right >= 0 and children.right < ix
    }),
    interner.nodes.indices().forall(ix => interner.nodes.indices().forall(jx =>
      ix == jx or interner.nodes.nth(ix) != interner.nodes.nth(jx))),
    range(0, interner.position).indices().forall(ix => {
      val handle = interner.handles.get(ix)
      handle >= 0 and handle < interner.nodes.length()
        and meanings(interner.nodes).nth(handle) == meanings(interner.input).nth(ix)
    }),
    if (finished) sums(interner.nodes).nth(interner.handles.get(6)) == sums(interner.input).nth(6) else true,
  }
}
