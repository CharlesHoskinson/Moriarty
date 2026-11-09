// General aggregation protocol for Sherlock recursion (task 2.11, row F20, SRC-017).
// Design: openspec/changes/sherlock-vm-consensus-20261107/design.md §3.6, D13';
// research/sherlock-speed-replan-20261108/tree.md §4.1 (I1-I10), prover.md §6b (Q-A..Q-C).
//
// Segment composition is modeled once. A segment claim is (prog, pre, post, hLo, hHi).
// A leaf certifies S slots, 1 <= steps <= S enabled, disabled slots the identity.
// A node certifies up to three slots in the order [L, leaf, R] (tree.md F8); each slot is
// optional, a disabled slot is the identity segment, an all-disabled node is refused.
// TREE=false admits only the linear node link(carried, leaf) (the degenerate tree);
// TREE=true also admits join (two node children) and pad (a unary node).
//
// Premises (not modeled): a leaf certificate exists iff the S-step relation holds on the
// reference machine (cryptographic soundness); digests are injective on the carrier domain
// (a digest is the pair (prog, step index) of the reference trace); the in-circuit verifier,
// the accumulator fold and the pairing are sound. The accumulator is abstracted as the ordered
// list of folded certificate ids (`acc`).
//
// Manifest (S3, council review 2.14, 2026-11-09): the bundle carries a structured manifest, one
// entry per node in children-first order with its slot bits (hasL, hasLeaf, hasR) and child
// references. The decider (`canDecide`) does not read the manifest: on-chain soundness rests on
// the pairing of the root accumulator and the statement chain only. The off-chain verifier
// (`verifyChain`, SRC-010 `verify_chain`) replays the manifest; the manifest hash is bound into
// the bundle. I9 is therefore a property of decider plus verify_chain, not of the decider alone.
//
// Genesis accumulator (S1, SRC-005 realization (a), lead decision 2026-11-09): a node's carried
// accumulator is the witness-supplied `carry`; at genesis (no carried child) it is scaled by the
// bit and collapsed to the trivial encoding, so it contributes nothing to `acc`; off genesis it is
// the carried child's accumulator, bound by the previous proof's public input. The adversarial
// action `genesis_carry` supplies a foreign carry at genesis; invariant I11 (`carryOk`) checks no
// node folds a certificate outside its own subtree. The encoding-level argument is Lean L-D
// (`genesis_collapse_both`, F21).
//
// Node statement (S4, lead decision 2026-11-09): the linear node (TREE = false) keeps its
// in-node genesis check (`genesisAtNode`); the slot node (TREE = true) leaves genesis to the
// decider. Lean `lin_refines_chain` (F21) maps the linear node statement to `link`.
//
// Programs: PROGS = 1..L; program p runs p owner steps and halts, so one instance covers every
// trace length 1..L. Heights are integers below a fixed bound HMAX (SRC-004, 2^64 in the design).
//
// Not covered (on purpose): the circuit relations themselves (SRC-003 leaf internals, SRC-004
// range gadgets, the encoding of SRC-005 genesis scaling, SRC-006 pairing), the Blake2b/Poseidon
// digest encodings, keygen and VK bytes, timing and memory of real proofs (measured rows), and
// the accumulator algebra (Lean L-D, F21). Model execution evidence covers these instances only.
// Record of runs: scripts/qa/check-f20.sh (typecheck, `quint test` with TREE false/true and
// S in {1, 2, 4, 8}, every seeded mutant caught, bounded `quint run` sweeps).
module aggregation {
  const TREE: bool   // join and pad admitted (D13' slot node); false = linear chain only
  const S: int       // keyed steps per leaf (SRC-014), S >= 1
  const L: int       // longest program (owner steps), L >= 1

  // Height bound. The design bound is 2^64 (SRC-004); Quint 0.32's Rust evaluator computes in
  // 64-bit integers and overflows on 2^64 itself, so the model's bound is 2^62. Every property
  // below is stated for a fixed bound HMAX; the exact 2^64 arithmetic is Lean L5 (F21).
  pure val HMAX = 2^62
  pure val PROGS = 1.to(L)

  type Digest = { prog: int, idx: int }
  type Key = LeafKey | NodeKey | DeciderKey | ForeignKey(int)
  type Seg = { prog: int, pre: Digest, post: Digest, hLo: int, hHi: int }
  type Slot = { enabled: bool, pre: Digest, post: Digest }
  type LeafId = { prog: int, hPre: int, steps: int }
  type Leaf = { id: LeafId, prog: int, pre: Digest, post: Digest, hPre: int, steps: int,
    slots: List[Slot], key: Key }
  type OptNode = NodeRef(int) | NoNode
  type OptLeaf = LeafRef(LeafId) | NoLeaf
  type CertRef = LeafC(LeafId) | NodeC(int)
  // One manifest entry: the node, its slot bits and its child references (S3).
  type ManEntry = { node: int, hasL: bool, l: OptNode, hasLeaf: bool, leaf: OptLeaf,
    hasR: bool, r: OptNode }
  type Node = { id: int, prog: int, pre: Digest, post: Digest, hLo: int, hHi: int,
    l: OptNode, leaf: OptLeaf, r: OptNode,
    cover: Set[LeafId], acc: List[CertRef], man: List[ManEntry], depth: int, key: Key }
  type Stmt = { prog: int, init: Digest, fin: Digest, nSteps: int, result: int }
  type Decision = { stmt: Stmt, root: int, manifest: List[ManEntry], key: Key }
  type State = {
    leaves: LeafId -> Leaf,   // leaf certificates (prover output; premise: unforgeable)
    nodes: int -> Node,       // node certificates by id (ids increase with certification)
    nextNode: int,
    decided: Set[Decision],   // decider outputs with the manifest the verifier replayed
    accepted: List[Stmt],     // acceptRun log of the contract
    refused: int,             // adversarial attempts the constraints refused
  }
  var s: State

  // ---- reference machine (abstract): program p has p steps, digest = (p, step index)
  pure def nStepsOf(p: int): int = p
  pure def d(p: int, i: int): Digest = { prog: p, idx: i }
  pure def initOf(p: int): Digest = d(p, 0)
  pure def halted(x: Digest): bool = x.idx == nStepsOf(x.prog)
  pure def resultOf(p: int): int = 10 * p
  pure def listSet(l: List[Stmt]): Set[Stmt] = l.indices().map(i => l[i])
  pure def minInt(a: int, b: int): int = if (a < b) a else b
  pure def maxInt(a: int, b: int): int = if (a > b) a else b

  // ---- leaf: S slots, k enabled from owner step i, the rest disabled identities
  pure def leafId(p: int, i: int, k: int): LeafId = { prog: p, hPre: i, steps: k }
  pure def mkLeaf(p: int, i: int, k: int): Leaf = {
    id: leafId(p, i, k), prog: p, pre: d(p, i), post: d(p, i + k), hPre: i, steps: k,
    slots: range(0, S).foldl([], (acc, j) => acc.append(
      if (j < k) { enabled: true, pre: d(p, i + j), post: d(p, i + j + 1) }
      else { enabled: false, pre: d(p, i + k), post: d(p, i + k) })),
    key: LeafKey,
  }
  // Q-A slot semantics: enabled slots form a prefix (the publishing slot is the last enabled
  // one), each enabled slot chains, a disabled slot is the identity and adds nothing to steps.
  pure def leafOk(x: Leaf): bool = and {
    x.slots.length() == S,
    x.slots.indices().forall(j => or {
      x.slots[j].enabled,
      and { x.slots[j].pre == x.slots[j].post, j + 1 == S or not(x.slots[j + 1].enabled) },
    }),
    x.slots.indices().forall(j => j + 1 == S or x.slots[j].post == x.slots[j + 1].pre),
    x.steps == x.slots.foldl(0, (n, sl) => if (sl.enabled) n + 1 else n),
    x.pre == x.slots[0].pre, x.post == x.slots[S - 1].post,
    x.id == leafId(x.prog, x.hPre, x.steps),
  }
  pure def segOfLeaf(x: Leaf): Seg =
    { prog: x.prog, pre: x.pre, post: x.post, hLo: x.hPre, hHi: x.hPre + x.steps }
  pure def segOfNode(n: Node): Seg =
    { prog: n.prog, pre: n.pre, post: n.post, hLo: n.hLo, hHi: n.hHi }

  // ---- node constraints (tree.md F8)
  pure def present(st: State, l: OptNode, lf: OptLeaf, r: OptNode): bool = and {
    match l { | NodeRef(i) => st.nodes.keys().contains(i) | NoNode => true },
    match lf { | LeafRef(x) => st.leaves.keys().contains(x) | NoLeaf => true },
    match r { | NodeRef(i) => st.nodes.keys().contains(i) | NoNode => true },
    not(l == NoNode and lf == NoLeaf and r == NoNode),   // at least one slot
  }
  pure def nodeSeg(st: State, o: OptNode): List[Seg] =
    match o { | NodeRef(i) => [segOfNode(st.nodes.get(i))] | NoNode => [] }
  pure def leafSeg(st: State, o: OptLeaf): List[Seg] =
    match o { | LeafRef(x) => [segOfLeaf(st.leaves.get(x))] | NoLeaf => [] }
  // The present slots in slot order; a disabled slot is the identity and is omitted.
  pure def slotSegs(st: State, l: OptNode, lf: OptLeaf, r: OptNode): List[Seg] =
    nodeSeg(st, l).concat(leafSeg(st, lf)).concat(nodeSeg(st, r))
  // link a b defined iff a.post = b.pre, a.hHi = b.hLo, a.prog = b.prog (design §3.6)
  pure def linked(segs: List[Seg]): bool = segs.indices().forall(i => i == 0 or and {
    segs[i - 1].prog == segs[i].prog,
    segs[i - 1].post == segs[i].pre,
    segs[i - 1].hHi == segs[i].hLo,
  })
  pure def compose(segs: List[Seg]): Seg = {
    prog: segs[0].prog, pre: segs[0].pre, hLo: segs[0].hLo,
    post: segs[segs.length() - 1].post, hHi: segs[segs.length() - 1].hHi,
  }
  pure def childKeyOk(st: State, o: OptNode): bool =
    match o { | NodeRef(i) => st.nodes.get(i).key == NodeKey | NoNode => true }
  pure def leafKeyOk(st: State, o: OptLeaf): bool =
    match o { | LeafRef(x) => st.leaves.get(x).key == LeafKey | NoLeaf => true }
  // SRC-004 at the leaf slot: 1 <= steps <= S and h arithmetic; the slot statement is the
  // leaf's own statement (its slots are checked inside the leaf relation, SRC-003).
  pure def leafSlotOk(st: State, o: OptLeaf): bool = match o {
    | LeafRef(x) =>
      val lf = st.leaves.get(x)
      and { 1 <= lf.steps, lf.steps <= S, lf.hPre + lf.steps < HMAX, leafOk(lf) }
    | NoLeaf => true
  }
  // The linear node (TREE = false) keeps its in-node genesis check (SRC-005, Q-B): with no carried
  // child the leaf must start at height 0 from init, so every chain certificate is a prefix of the
  // run. The slot node has no genesis case; the decider checks the root (tree.md F8).
  pure def genesisAtNode(st: State, l: OptNode, lf: OptLeaf): bool = TREE or l != NoNode or
    (match lf { | LeafRef(x) => st.leaves.get(x).hPre == 0 and st.leaves.get(x).pre == initOf(x.prog) | NoLeaf => false })
  pure def canNode(st: State, l: OptNode, lf: OptLeaf, r: OptNode): bool =
    if (not(present(st, l, lf, r))) false else
    val segs = slotSegs(st, l, lf, r)
    and {
      linked(segs),
      childKeyOk(st, l), childKeyOk(st, r), leafKeyOk(st, lf), leafSlotOk(st, lf),
      segs.indices().forall(i => 0 <= segs[i].hLo and segs[i].hLo <= segs[i].hHi and segs[i].hHi < HMAX),
      genesisAtNode(st, l, lf),
    }
  pure def coverOf(st: State, l: OptNode, lf: OptLeaf, r: OptNode): Set[LeafId] = {
    val cl: Set[LeafId] = match l { | NodeRef(i) => st.nodes.get(i).cover | NoNode => Set() }
    val cf: Set[LeafId] = match lf { | LeafRef(x) => Set(x) | NoLeaf => Set() }
    val cr: Set[LeafId] = match r { | NodeRef(i) => st.nodes.get(i).cover | NoNode => Set() }
    cl.union(cf).union(cr)
  }
  // Realization (a): the carried accumulator is scaled by the slot bit and collapsed before the
  // fold. A present carried child binds it to the child's accumulator (previous proof's public
  // input); an absent one collapses any witness-supplied carry to the trivial encoding (no entry).
  pure def carriedAcc(st: State, l: OptNode, carry: List[CertRef]): List[CertRef] =
    match l { | NodeRef(i) => st.nodes.get(i).acc | NoNode => [] }
  // The accumulator abstraction: fold order is children's folds in slot order, then self.
  pure def accWith(st: State, l: OptNode, lf: OptLeaf, r: OptNode, self: int,
      carry: List[CertRef]): List[CertRef] = {
    val al: List[CertRef] = carriedAcc(st, l, carry)
    val af: List[CertRef] = match lf { | LeafRef(x) => [LeafC(x)] | NoLeaf => [] }
    val ar: List[CertRef] = match r { | NodeRef(i) => st.nodes.get(i).acc | NoNode => [] }
    al.concat(af).concat(ar).append(NodeC(self))
  }
  pure def accOf(st: State, l: OptNode, lf: OptLeaf, r: OptNode, self: int): List[CertRef] =
    accWith(st, l, lf, r, self, [])
  // The structured manifest: children's manifests in slot order, then this node's entry.
  pure def entryOf(self: int, l: OptNode, lf: OptLeaf, r: OptNode): ManEntry =
    { node: self, hasL: l != NoNode, l: l, hasLeaf: lf != NoLeaf, leaf: lf, hasR: r != NoNode, r: r }
  pure def manOf(st: State, l: OptNode, lf: OptLeaf, r: OptNode, self: int): List[ManEntry] = {
    val ml: List[ManEntry] = match l { | NodeRef(i) => st.nodes.get(i).man | NoNode => [] }
    val mr: List[ManEntry] = match r { | NodeRef(i) => st.nodes.get(i).man | NoNode => [] }
    ml.concat(mr).append(entryOf(self, l, lf, r))
  }
  // The flattened view of the first F20 draft (node ids in fold order): blind to slot placement.
  pure def flatMan(man: List[ManEntry]): List[int] = man.foldl([], (a, e) => a.append(e.node))
  pure def depthOf(st: State, o: OptNode): int =
    match o { | NodeRef(i) => st.nodes.get(i).depth | NoNode => 0 }
  pure def mkNodeWith(st: State, l: OptNode, lf: OptLeaf, r: OptNode, carry: List[CertRef]): Node = {
    val g = compose(slotSegs(st, l, lf, r))
    { id: st.nextNode, prog: g.prog, pre: g.pre, post: g.post, hLo: g.hLo, hHi: g.hHi,
      l: l, leaf: lf, r: r, cover: coverOf(st, l, lf, r),
      acc: accWith(st, l, lf, r, st.nextNode, carry), man: manOf(st, l, lf, r, st.nextNode),
      depth: 1 + maxInt(depthOf(st, l), depthOf(st, r)), key: NodeKey }
  }
  pure def mkNode(st: State, l: OptNode, lf: OptLeaf, r: OptNode): Node = mkNodeWith(st, l, lf, r, [])
  pure def certifyWith(st: State, l: OptNode, lf: OptLeaf, r: OptNode, carry: List[CertRef]): State =
    { ...st, nodes: st.nodes.put(st.nextNode, mkNodeWith(st, l, lf, r, carry)), nextNode: st.nextNode + 1 }
  pure def certify(st: State, l: OptNode, lf: OptLeaf, r: OptNode): State = certifyWith(st, l, lf, r, [])
  pure def refuse(st: State): State = { ...st, refused: st.refused + 1 }

  // ---- decider and contract
  pure def stmtOf(n: Node): Stmt =
    { prog: n.prog, init: initOf(n.prog), fin: n.post, nSteps: n.hHi, result: resultOf(n.prog) }
  // The decider: root accepted iff pre = init, hLo = 0, post halted, key pinned. It does not
  // read the manifest (S3).
  pure def canDecide(st: State, id: int): bool =
    st.nodes.keys().contains(id) and
    val n = st.nodes.get(id)
    and { n.key == NodeKey, n.pre == initOf(n.prog), n.hLo == 0, halted(n.post) }
  // verify_chain (SRC-010): the bundled manifest replays the root's fold exactly. Every entry's
  // slot bits agree with its references and with the certified node, every child is replayed
  // before its parent, the last entry is the root, and the entries are the root's slot-order fold.
  pure def bitsOk(e: ManEntry): bool = and {
    e.hasL == (e.l != NoNode), e.hasLeaf == (e.leaf != NoLeaf), e.hasR == (e.r != NoNode) }
  pure def refs(e: ManEntry): Set[int] = {
    val a: Set[int] = match e.l { | NodeRef(i) => Set(i) | NoNode => Set() }
    val b: Set[int] = match e.r { | NodeRef(i) => Set(i) | NoNode => Set() }
    a.union(b)
  }
  pure def entryReplays(st: State, manifest: List[ManEntry], i: int): bool = {
    val e = manifest[i]
    and {
      bitsOk(e), st.nodes.keys().contains(e.node),
      st.nodes.get(e.node).l == e.l, st.nodes.get(e.node).leaf == e.leaf,
      st.nodes.get(e.node).r == e.r,
      refs(e).forall(c => manifest.indices().exists(j => j < i and manifest[j].node == c)),
    }
  }
  pure def verifyChain(st: State, id: int, manifest: List[ManEntry]): bool =
    st.nodes.keys().contains(id) and manifest.length() > 0 and and {
      manifest[manifest.length() - 1].node == id,
      manifest.indices().forall(i => entryReplays(st, manifest, i)),
      manifest == st.nodes.get(id).man,
    }
  // A bundle is published when the decider accepts and verify_chain replays its manifest.
  pure def canPublish(st: State, id: int, manifest: List[ManEntry]): bool =
    canDecide(st, id) and verifyChain(st, id, manifest)
  pure def applyDecide(st: State, id: int, manifest: List[ManEntry]): State = { ...st,
    decided: st.decided.union(Set({ stmt: stmtOf(st.nodes.get(id)), root: id,
      manifest: manifest, key: DeciderKey })) }
  pure def canAccept(st: State, x: Stmt): bool =
    st.decided.exists(dc => dc.stmt == x) and not(listSet(st.accepted).contains(x))
  pure def applyAccept(st: State, x: Stmt): State = { ...st, accepted: st.accepted.append(x) }

  // ---- actions
  action init = s' = { leaves: Map(), nodes: Map(), nextNode: 0, decided: Set(),
    accepted: [], refused: 0 }
  // The prover proves owner steps i..i+k-1 of program p (k enabled slots, S-k disabled).
  pure def canLeaf(p: int, i: int, k: int): bool = and {
    PROGS.contains(p), 1 <= k, k <= S, 0 <= i, i + k <= nStepsOf(p) }
  action leaf(p: int, i: int, k: int): bool = all {
    canLeaf(p, i, k), s' = { ...s, leaves: s.leaves.put(leafId(p, i, k), mkLeaf(p, i, k)) } }
  action node(l: OptNode, lf: OptLeaf, r: OptNode): bool = all {
    canNode(s, l, lf, r), s' = certify(s, l, lf, r) }
  // Linear node `fold` (spelled foldLeaf: `fold` is a Quint built-in): link(carried, leaf);
  // carried absent at the start of a segment.
  action foldLeaf(carried: OptNode, lf: LeafId): bool = node(carried, LeafRef(lf), NoNode)
  // Tree node with a right child (and any L and leaf); guarded by TREE.
  pure def canJoin(st: State, l: OptNode, lf: OptLeaf, r: int): bool = TREE and canNode(st, l, lf, NodeRef(r))
  action join(l: OptNode, lf: OptLeaf, r: int): bool = all { canJoin(s, l, lf, r), node(l, lf, NodeRef(r)) }
  // Padding child: a unary node whose other slots are disabled identities; guarded by TREE.
  pure def canPad(st: State, l: int): bool = TREE and canNode(st, NodeRef(l), NoLeaf, NoNode)
  action pad(l: int): bool = all { canPad(s, l), node(NodeRef(l), NoLeaf, NoNode) }
  action decide(id: int, manifest: List[ManEntry]): bool = all {
    canPublish(s, id, manifest), s' = applyDecide(s, id, manifest) }
  action accept(x: Stmt): bool = all { canAccept(s, x), s' = applyAccept(s, x) }
  // An attempt certifies when the constraints hold and is otherwise refused and counted.
  action attempt(l: OptNode, lf: OptLeaf, r: OptNode): bool =
    if (canNode(s, l, lf, r)) s' = certify(s, l, lf, r) else s' = refuse(s)
  action attemptDecide(id: int, manifest: List[ManEntry]): bool =
    if (canPublish(s, id, manifest)) s' = applyDecide(s, id, manifest) else s' = refuse(s)
  // A node attempt with a witness-supplied carried accumulator.
  action attemptCarry(l: OptNode, lf: OptLeaf, r: OptNode, carry: List[CertRef]): bool =
    if (canNode(s, l, lf, r)) s' = certifyWith(s, l, lf, r, carry) else s' = refuse(s)
  action attemptAccept(x: Stmt): bool =
    if (canAccept(s, x)) s' = applyAccept(s, x) else s' = refuse(s)

  pure def nodeOpts(st: State): Set[OptNode] = st.nodes.keys().map(i => NodeRef(i)).union(Set(NoNode))
  pure def leafOpts(st: State): Set[OptLeaf] = st.leaves.keys().map(x => LeafRef(x)).union(Set(NoLeaf))

  action leafAny = {
    nondet p = PROGS.oneOf()
    nondet i = 0.to(nStepsOf(p) - 1).oneOf()
    nondet k = 1.to(minInt(S, nStepsOf(p) - i)).oneOf()
    leaf(p, i, k)
  }
  action foldAny = {
    nondet lf = s.leaves.keys().oneOf()
    nondet c = nodeOpts(s).filter(c => canNode(s, c, LeafRef(lf), NoNode)).oneOf()
    foldLeaf(c, lf)
  }
  // Every admissible (L, leaf, R) with R present.
  pure def joinCands(st: State): Set[(OptNode, OptLeaf, int)] =
    st.nodes.keys().map(r => nodeOpts(st).map(l => leafOpts(st).map(lf => (l, lf, r))).flatten())
      .flatten().filter(((l, lf, r)) => canNode(st, l, lf, NodeRef(r)))
  action joinAny = {
    nondet c = joinCands(s).oneOf()
    join(c._1, c._2, c._3)
  }
  action padAny = {
    nondet l = s.nodes.keys().oneOf()
    pad(l)
  }
  // schedule: any admissible fold, join or pad (every interleaving is a run of the model).
  action schedule = any { foldAny, joinAny, padAny }
  action decideAny = {
    nondet id = s.nodes.keys().filter(i => canPublish(s, i, s.nodes.get(i).man)).oneOf()
    decide(id, s.nodes.get(id).man)
  }
  action acceptAny = {
    nondet x = s.decided.map(dc => dc.stmt).filter(x => canAccept(s, x)).oneOf()
    accept(x)
  }

  // ---- adversary: attack-shaped inputs through the same constraints
  // forge_link: a leaf whose pre or height does not meet the carried segment.
  action forge_link = {
    nondet l = s.nodes.keys().oneOf()
    nondet lf = s.leaves.keys().filter(x =>
      s.leaves.get(x).pre != s.nodes.get(l).post or s.leaves.get(x).hPre != s.nodes.get(l).hHi).oneOf()
    attempt(NodeRef(l), LeafRef(lf), NoNode)
  }
  // reuse_leaf: a leaf already under the carried node folded again.
  action reuse_leaf = {
    nondet l = s.nodes.keys().oneOf()
    nondet lf = s.nodes.get(l).cover.oneOf()
    attempt(NodeRef(l), LeafRef(lf), NoNode)
  }
  // swap_children: the leaf that precedes a node put after it; or two linked nodes reversed.
  action swap_children = any {
    {
      nondet l = s.nodes.keys().oneOf()
      nondet lf = s.leaves.keys().filter(x =>
        s.leaves.get(x).post == s.nodes.get(l).pre and s.leaves.get(x).hPre + s.leaves.get(x).steps == s.nodes.get(l).hLo).oneOf()
      attempt(NodeRef(l), LeafRef(lf), NoNode)
    },
    {
      nondet a = s.nodes.keys().oneOf()
      nondet b = s.nodes.keys().filter(b => s.nodes.get(a).post == s.nodes.get(b).pre
        and s.nodes.get(a).hHi == s.nodes.get(b).hLo).oneOf()
      attempt(NodeRef(b), NoLeaf, NodeRef(a))
    },
  }
  action empty_node = attempt(NoNode, NoLeaf, NoNode)
  // shape_mismatch: a decidable root presented with a manifest of a different fold order.
  action shape_mismatch = {
    nondet id = s.nodes.keys().filter(i => canPublish(s, i, s.nodes.get(i).man)).oneOf()
    val man = s.nodes.get(id).man
    attemptDecide(id, man.slice(1, man.length()))
  }
  // genesis_carry (S1): a genesis fold (no carried child) with a witness-supplied carried
  // accumulator taken from another node: an unauthenticated deferred obligation.
  action genesis_carry = {
    nondet lf = s.leaves.keys().filter(x => s.leaves.get(x).hPre == 0).oneOf()
    nondet src = s.nodes.keys().oneOf()
    attemptCarry(NoNode, LeafRef(lf), NoNode, s.nodes.get(src).acc)
  }
  // replay: a statement the contract already accepted.
  action replay = {
    nondet x = listSet(s.accepted).oneOf()
    attemptAccept(x)
  }
  action adversary = any { forge_link, reuse_leaf, swap_children, empty_node, shape_mismatch, replay,
    genesis_carry }

  action step = any { leafAny, schedule, decideAny, acceptAny, adversary }

  // ---- invariants (pure over State so the test harness can evaluate mutated states)
  pure def nodesOf(st: State): Set[Node] = st.nodes.keys().map(i => st.nodes.get(i))
  pure def leavesOf(st: State): Set[Leaf] = st.leaves.keys().map(x => st.leaves.get(x))
  pure def childIds(n: Node): Set[int] = {
    val a: Set[int] = match n.l { | NodeRef(i) => Set(i) | NoNode => Set() }
    val b: Set[int] = match n.r { | NodeRef(i) => Set(i) | NoNode => Set() }
    a.union(b)
  }
  // I1 linkage: every node's present slots chain and the node is their composition.
  pure def linkageOk(st: State): bool = nodesOf(st).forall(n => and {
    present(st, n.l, n.leaf, n.r),
    linked(slotSegs(st, n.l, n.leaf, n.r)),
    segOfNode(n) == compose(slotSegs(st, n.l, n.leaf, n.r)),
    childIds(n).forall(c => c < n.id),
  })
  pure def stepsSum(cover: Set[LeafId]): int = cover.fold(0, (acc, x) => acc + x.steps)
  // I2 heights: monotone, additive, below 2^64; leaf steps in [1, S].
  pure def heightsOk(st: State): bool = and {
    nodesOf(st).forall(n => and {
      0 <= n.hLo, n.hLo < n.hHi, n.hHi < HMAX, n.hHi - n.hLo == stepsSum(n.cover) }),
    leavesOf(st).forall(x => and {
      0 <= x.hPre, 1 <= x.steps, x.steps <= S, x.hPre + x.steps < HMAX }),
  }
  // I3 exactly-once coverage: the leaves under a node tile [hLo, hHi) without gap or repeat.
  pure def tiles(cover: Set[LeafId], lo: int, hi: int): bool = and {
    cover.size() > 0,
    cover.forall(x => lo <= x.hPre and x.hPre + x.steps <= hi),
    stepsSum(cover) == hi - lo,
    cover.forall(x => cover.forall(y => x == y or x.hPre + x.steps <= y.hPre or y.hPre + y.steps <= x.hPre)),
  }
  pure def coverageOk(st: State): bool = nodesOf(st).forall(n => and {
    tiles(n.cover, n.hLo, n.hHi),
    n.cover == coverOf(st, n.l, n.leaf, n.r),
  })
  pure def presentSlots(n: Node): int =
    (if (n.l == NoNode) 0 else 1) + (if (n.leaf == NoLeaf) 0 else 1) + (if (n.r == NoNode) 0 else 1)
  // I4 identity and padding neutrality: disabled leaf slots are identities adding no step;
  // a unary node has its child's statement; no node has every slot disabled.
  pure def paddingOk(st: State): bool = and {
    leavesOf(st).forall(x => leafOk(x)),
    nodesOf(st).forall(n => presentSlots(n) >= 1),
    nodesOf(st).forall(n => presentSlots(n) != 1 or
      segOfNode(n) == slotSegs(st, n.l, n.leaf, n.r)[0]),
  }
  // I5 schedule independence: a certificate's statement is a function of (prog, hLo, hHi)
  // only (it equals the reference trace), so any two schedules over the same leaves agree.
  pure def faithful(g: Seg): bool = g.pre == d(g.prog, g.hLo) and g.post == d(g.prog, g.hHi)
  pure def scheduleOk(st: State): bool = and {
    nodesOf(st).forall(n => faithful(segOfNode(n))),
    leavesOf(st).forall(x => faithful(segOfLeaf(x))),
    nodesOf(st).forall(a => nodesOf(st).forall(b =>
      not(a.prog == b.prog and a.hLo == b.hLo and a.hHi == b.hHi) or segOfNode(a) == segOfNode(b))),
    st.decided.forall(a => st.decided.forall(b => a.stmt.prog != b.stmt.prog or a.stmt == b.stmt)),
  }
  // I6 genesis: a decided root starts at height 0 from init; height 0 means pre = init;
  // at most one leaf at height 0 under any node.
  pure def genesisOk(st: State): bool = and {
    st.decided.forall(dc => st.nodes.keys().contains(dc.root) and
      val n = st.nodes.get(dc.root)
      and { n.hLo == 0, n.pre == initOf(n.prog), dc.stmt.init == initOf(n.prog) }),
    nodesOf(st).forall(n => n.hLo != 0 or n.pre == initOf(n.prog)),
    leavesOf(st).forall(x => x.hPre != 0 or x.pre == initOf(x.prog)),
    nodesOf(st).forall(n => n.cover.filter(x => x.hPre == 0).size() <= 1),
  }
  // I7 replay: each statement accepted at most once, and only decided statements.
  pure def replayOk(st: State): bool = and {
    st.accepted.indices().forall(i => st.accepted.indices().forall(j =>
      i == j or st.accepted[i] != st.accepted[j])),
    listSet(st.accepted).forall(x => st.decided.exists(dc => dc.stmt == x)),
  }
  // I8 program fixity: every child and leaf under a node carries the node's prog.
  pure def programOk(st: State): bool = and {
    nodesOf(st).forall(n => val segs = slotSegs(st, n.l, n.leaf, n.r)
      segs.indices().forall(i => segs[i].prog == n.prog)),
    nodesOf(st).forall(n => n.cover.forall(x => x.prog == n.prog)),
    st.decided.forall(dc => dc.stmt.prog == st.nodes.get(dc.root).prog),
  }
  // I9 shape binding (decider plus verify_chain): every published manifest replays the root's
  // structured fold, slot bits included; the manifest and the fold order are the slot-order
  // concatenation of the children's; no certificate is folded twice.
  pure def shapeOk(st: State): bool = and {
    st.decided.forall(dc => verifyChain(st, dc.root, dc.manifest)
      and dc.stmt == stmtOf(st.nodes.get(dc.root))),
    nodesOf(st).forall(n => n.man == manOf(st, n.l, n.leaf, n.r, n.id)),
    nodesOf(st).forall(n => n.acc == accOf(st, n.l, n.leaf, n.r, n.id)),
    nodesOf(st).forall(n => n.acc.indices().forall(i => n.acc.indices().forall(j =>
      i == j or n.acc[i] != n.acc[j]))),
  }
  // Fixed keys: one leaf key, one self-recursive node key, one decider key; no key depends
  // on the program, the depth or S (a ForeignKey anywhere is a defect).
  pure def keysOk(st: State): bool = and {
    leavesOf(st).forall(x => x.key == LeafKey),
    nodesOf(st).forall(n => n.key == NodeKey and childKeyOk(st, n.l) and childKeyOk(st, n.r)
      and leafKeyOk(st, n.leaf)),
    st.decided.forall(dc => dc.key == DeciderKey and st.nodes.get(dc.root).key == NodeKey),
  }
  // I11 genesis carry (S1): no node folds a certificate outside its own subtree. The leaf
  // entries of its accumulator are exactly its cover, and a node with no carried child folds
  // nothing but its own slots.
  pure def accLeaves(acc: List[CertRef]): Set[LeafId] =
    acc.foldl(Set(), (a, c) => match c { | LeafC(x) => a.union(Set(x)) | NodeC(_) => a })
  pure def carryOk(st: State): bool = nodesOf(st).forall(n => and {
    accLeaves(n.acc) == n.cover,
    n.l != NoNode or n.acc == accOf(st, n.l, n.leaf, n.r, n.id),
  })
  // Linear shape (TREE = false): every node is link(carried?, leaf) and a prefix from genesis.
  pure def chainOnly(st: State): bool =
    TREE or nodesOf(st).forall(n => n.r == NoNode and n.leaf != NoLeaf and n.hLo == 0 and n.pre == initOf(n.prog))
  pure def rulesOk(st: State): bool = and {
    linkageOk(st), heightsOk(st), coverageOk(st), paddingOk(st), scheduleOk(st),
    genesisOk(st), replayOk(st), programOk(st), shapeOk(st), keysOk(st), chainOnly(st),
    carryOk(st),
  }

  val I1_linkage = linkageOk(s)
  val I2_heights = heightsOk(s)
  val I3_coverage = coverageOk(s)
  val I4_padding = paddingOk(s)
  val I5_schedule = scheduleOk(s)
  val I6_genesis = genesisOk(s)
  val I7_replay = replayOk(s)
  val I8_program = programOk(s)
  val I9_shape = shapeOk(s)
  val fixedKeys = keysOk(s)
  val linearOnly = chainOnly(s)
  val I11_genesisCarry = carryOk(s)
  val allInvariants = rulesOk(s)

  // I10 liveness witnesses (reachability; `quint run --witnesses`, and driven in the tests)
  val wLeaf = s.leaves.keys().size() > 0
  val wShortLeaf = leavesOf(s).exists(x => x.steps < S)
  val wFolded = s.nodes.keys().size() > 0
  val wJoined = nodesOf(s).exists(n => n.l != NoNode and n.r != NoNode)
  val wThreeSlots = nodesOf(s).exists(n => presentSlots(n) == 3)
  val wPadded = nodesOf(s).exists(n => presentSlots(n) == 1 and n.leaf == NoLeaf)
  val wDeep = nodesOf(s).exists(n => n.depth >= 3)
  val wDecided = s.decided.size() > 0
  val wTwoRootsOneStmt = nodesOf(s).exists(a => nodesOf(s).exists(b =>
    a.id != b.id and a.man != b.man and canPublish(s, a.id, a.man) and canPublish(s, b.id, b.man)))
  val wAccepted = s.accepted.length() > 0
  val wRefused = s.refused > 0
}

// Runnable instances for `quint run` sweeps (small, bounded).
module aggregation_chain { import aggregation(TREE = false, S = 2, L = 3).* }
module aggregation_tree { import aggregation(TREE = true, S = 2, L = 3).* }
module aggregation_tree_s1 { import aggregation(TREE = true, S = 1, L = 4).* }

// Q-C: leaf workers ahead of the node chain under the 48 GB per-process and 100 GB aggregate
// caps (prover.md F11, §6b Q-C; design §3.5 lever 3, SRC-016 E11). One node chain, P leaf
// workers, memory in GB as recorded peaks. Admission reserves room for the node so the chain
// is never starved by look-ahead leaves.
module leaf_scheduler {
  const P: int        // leaf workers
  const N: int        // leaves (= chain nodes) of the run
  const LEAF_GB: int  // recorded leaf peak
  const NODE_GB: int  // recorded node peak

  pure val PROC_CAP = 48
  pure val AGG_CAP = 100
  pure val IDX = 0.to(N - 1)

  type Phase = Idle | Proving | Proved
  type Sched = { leaf: int -> Phase, node: int -> Phase }
  var w: Sched

  pure def count(m: int -> Phase, ph: Phase): int = IDX.filter(i => m.get(i) == ph).size()
  pure def inflight(x: Sched): int = count(x.leaf, Proving) * LEAF_GB + count(x.node, Proving) * NODE_GB
  pure def nodeReserve(x: Sched): int = if (count(x.node, Proving) == 0 and count(x.node, Proved) < N) NODE_GB else 0
  pure def done(x: Sched): bool = count(x.node, Proved) == N

  pure def canStartLeaf(x: Sched, i: int): bool = and {
    x.leaf.get(i) == Idle, count(x.leaf, Proving) < P, LEAF_GB <= PROC_CAP,
    inflight(x) + LEAF_GB + nodeReserve(x) <= AGG_CAP,
  }
  pure def canFinishLeaf(x: Sched, i: int): bool = x.leaf.get(i) == Proving
  pure def nodeReady(x: Sched, i: int): bool = and {
    x.node.get(i) == Idle, x.leaf.get(i) == Proved, i == 0 or x.node.get(i - 1) == Proved }
  pure def canStartNode(x: Sched, i: int): bool = and {
    nodeReady(x, i), count(x.node, Proving) == 0, NODE_GB <= PROC_CAP,
    inflight(x) + NODE_GB <= AGG_CAP,
  }
  pure def canFinishNode(x: Sched, i: int): bool = x.node.get(i) == Proving
  pure def someEnabled(x: Sched): bool = IDX.exists(i => or {
    canStartLeaf(x, i), canFinishLeaf(x, i), canStartNode(x, i), canFinishNode(x, i) })

  action init = w' = { leaf: IDX.mapBy(_ => Idle), node: IDX.mapBy(_ => Idle) }
  action startLeaf(i: int): bool = all { canStartLeaf(w, i), w' = { ...w, leaf: w.leaf.set(i, Proving) } }
  action finishLeaf(i: int): bool = all { canFinishLeaf(w, i), w' = { ...w, leaf: w.leaf.set(i, Proved) } }
  action startNode(i: int): bool = all { canStartNode(w, i), w' = { ...w, node: w.node.set(i, Proving) } }
  action finishNode(i: int): bool = all { canFinishNode(w, i), w' = { ...w, node: w.node.set(i, Proved) } }
  action step = {
    nondet i = IDX.oneOf()
    any { startLeaf(i), finishLeaf(i), startNode(i), finishNode(i) }
  }

  // Q-C properties
  pure def aggregateOk(x: Sched): bool = inflight(x) <= AGG_CAP
  pure def processOk(x: Sched): bool = and {
    count(x.leaf, Proving) == 0 or LEAF_GB <= PROC_CAP,
    count(x.node, Proving) == 0 or NODE_GB <= PROC_CAP,
    count(x.leaf, Proving) <= P, count(x.node, Proving) <= 1,
  }
  // node i never starts before leaf i is proved and node i-1 is proved
  pure def orderOk(x: Sched): bool = IDX.forall(i => x.node.get(i) == Idle or and {
    x.leaf.get(i) == Proved, i == 0 or x.node.get(i - 1) == Proved })
  // no deadlock: while the chain is unfinished some action is enabled
  pure def progressOk(x: Sched): bool = done(x) or someEnabled(x)
  // the node chain is never starved by look-ahead leaves
  pure def nodeAdmissible(x: Sched): bool = IDX.forall(i => not(nodeReady(x, i)) or canStartNode(x, i))
  pure def schedOk(x: Sched): bool = aggregateOk(x) and processOk(x) and orderOk(x) and progressOk(x) and nodeAdmissible(x)

  val QC_aggregate = aggregateOk(w)
  val QC_process = processOk(w)
  val QC_order = orderOk(w)
  val QC_progress = progressOk(w)
  val QC_nodeAdmissible = nodeAdmissible(w)
  val QC_all = schedOk(w)
  val wAllFolded = done(w)
  val wParallelLeaves = count(w.leaf, Proving) >= 2
  val wNodeAndLeaves = count(w.node, Proving) == 1 and count(w.leaf, Proving) >= 1
}

module leaf_scheduler_p3 { import leaf_scheduler(P = 3, N = 6, LEAF_GB = 16, NODE_GB = 26).* }
