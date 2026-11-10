// Honest schedules, guard checks, liveness witnesses and one seeded mutant per invariant for
// the aggregation model (task 2.11, row F20). Mutants live only in this harness.
// Run: quint test models/sherlock/quint/aggregation_test.qnt --main aggregation_test --max-samples 100
// Refusals are checked through the pure guards (`not(canX(...))` in an expect): a `.fail()` step
// yields no successor state, so it can only end a run.
module aggregation_case {
  const CASE_TREE: bool
  const CASE_S: int
  const CASE_L: int
  import aggregation(TREE = CASE_TREE, S = CASE_S, L = CASE_L) as m from "./aggregation"

  // ---- chunking of program p into leaves of S steps, the last one short
  pure def chunks(p: int): int = (p + m::S - 1) / m::S
  pure def chunkSteps(p: int, j: int): int = m::minInt(m::S, p - j * m::S)
  pure def chunkId(p: int, j: int): m::LeafId = m::leafId(p, j * m::S, chunkSteps(p, j))
  pure def baseOf(p: int): int = 1.to(p - 1).fold(0, (a, q) => a + chunks(q))
  pure def rootOf(p: int): int = baseOf(p) + chunks(p) - 1   // root of p when programs 1..p are chained in order
  pure def soloRoot(p: int): int = chunks(p) - 1              // root of p chained alone from node 0
  pure def stmtFor(p: int): m::Stmt =
    { prog: p, init: m::initOf(p), fin: m::d(p, p), nSteps: p, result: m::resultOf(p) }
  def manifestOf(id: int): List[m::ManEntry] = m::s.nodes.get(id).man
  def nodeOf(id: int): m::Node = m::s.nodes.get(id)
  def wrongManifest(id: int): List[m::ManEntry] = manifestOf(id).slice(1, manifestOf(id).length())
  // Action arguments may not read state, so the honest decider reads the root's manifest here.
  action decideWith(id: int): bool = m::decide(id, m::s.nodes.get(id).man)
  action idle = m::s' = m::s

  run proveLeaves(p: int): bool = chunks(p).reps(j => m::leaf(p, j * m::S, chunkSteps(p, j)))
  // Linear chain: node base = fold(none, chunk 0); node base+j = fold(node base+j-1, chunk j).
  // (A zero-count reps yields no state, so the first fold is the j == 0 iteration.)
  run chainFold(p: int, base: int): bool = chunks(p).reps(j =>
    if (j == 0) m::foldLeaf(m::NoNode, chunkId(p, 0)) else m::foldLeaf(m::NodeRef(base + j - 1), chunkId(p, j)))
  run chainOf(p: int): bool = m::init.then(proveLeaves(p)).then(chainFold(p, 0))
  run decideAndAcceptAt(root: int, p: int): bool = decideWith(root).then(m::accept(stmtFor(p)))

  // Q-A / I10 (linear): every program 1..L proves, folds, decides and is accepted once,
  // the decider's n_steps is the owner trace length, the last chunk is short when S does
  // not divide p, and every invariant holds in the final state.
  run everyProgramChain = m::init
    .then(m::L.reps(j => proveLeaves(j + 1).then(chainFold(j + 1, baseOf(j + 1)))
      .then(decideAndAcceptAt(rootOf(j + 1), j + 1))))
    .expect(and {
      m::rulesOk(m::s),
      m::s.decided.size() == m::L, m::s.accepted.length() == m::L,
      m::PROGS.forall(p => and {
        m::s.decided.exists(dc => dc.stmt == stmtFor(p) and dc.root == rootOf(p)),
        nodeOf(rootOf(p)).hHi == p, nodeOf(rootOf(p)).cover.size() == chunks(p),
        nodeOf(rootOf(p)).depth == chunks(p),
        nodeOf(rootOf(p)).cover.forall(x => x.hPre % m::S == 0),
        nodeOf(rootOf(p)).cover.exists(x => x.steps == ((p - 1) % m::S) + 1),
        m::s.leaves.get(chunkId(p, chunks(p) - 1)).slots.indices().forall(i =>
          m::s.leaves.get(chunkId(p, chunks(p) - 1)).slots[i].enabled == (i < ((p - 1) % m::S) + 1)),
      }),
    })

  // Leaf guards: steps in [1, S] and within the trace.
  run leafGuards = m::init
    .expect(and {
      not(m::canLeaf(1, 0, 0)), not(m::canLeaf(m::L, 0, m::S + 1)), not(m::canLeaf(1, 1, 1)),
      not(m::canLeaf(m::L + 1, 0, 1)), m::canLeaf(1, 0, 1), m::canLeaf(m::L, m::L - 1, 1),
    })
    .then(m::leaf(1, 0, 1))
    .expect(m::rulesOk(m::s) and m::s.leaves.keys().size() == 1)

  // Linkage, order, reuse, empty node, program mixing, genesis, halting (two programs).
  run nodeGuards = m::init.then(proveLeaves(2)).then(m::leaf(1, 0, 1))
    .then(m::foldLeaf(m::NoNode, chunkId(2, 0)))                           // node 0: prog 2 [0, c0)
    .expect(and {
      not(m::canNode(m::s, m::NoNode, m::NoLeaf, m::NoNode)),                           // empty_node
      not(m::canNode(m::s, m::NodeRef(0), m::LeafRef(chunkId(2, 0)), m::NoNode)),       // reuse_leaf
      not(m::canNode(m::s, m::NodeRef(0), m::LeafRef(m::leafId(1, 0, 1)), m::NoNode)),  // program mixing
      not(m::canNode(m::s, m::NodeRef(1), m::NoLeaf, m::NoNode)),                       // missing child
      chunks(2) == 1 or not(m::canDecide(m::s, 0)),                      // not halted
    })
    .then(if (chunks(2) == 1) decideWith(0) else m::foldLeaf(m::NodeRef(0), chunkId(2, 1)))  // node 1: [0, 2)
    .then(if (chunks(2) == 1) m::accept(stmtFor(2))
          else if (m::TREE) m::foldLeaf(m::NoNode, chunkId(2, 1)) else idle)                   // node 2: [1, 2) (tree only)
    .expect(if (chunks(2) == 1) not(m::canAccept(m::s, stmtFor(2))) else and {
      m::canPublish(m::s, 1, manifestOf(1)),
      // chain: a node without a carried child must start at genesis (SRC-005 in the node)
      m::TREE or not(m::canNode(m::s, m::NoNode, m::LeafRef(chunkId(2, 1)), m::NoNode)),
      not(m::TREE) or and {
        not(m::canNode(m::s, m::NodeRef(2), m::LeafRef(chunkId(2, 0)), m::NoNode)),  // swap_children
        not(m::canDecide(m::s, 2)),                                   // I6: hLo != 0
      },
    })
    .then(if (chunks(2) == 1) idle else decideWith(1))
    .expect(m::rulesOk(m::s) and m::s.decided.size() == 1
      and m::s.decided.forall(dc => dc.stmt == stmtFor(2)))

  // Adversarial actions are refused and counted; the certificate set does not change.
  run adversaryRefused = chainOf(m::L).then(decideAndAcceptAt(soloRoot(m::L), m::L))
    .then(m::empty_node).then(m::reuse_leaf).then(m::forge_link).then(m::shape_mismatch).then(m::replay)
    .expect(and {
      m::s.refused == 5, m::s.nodes.keys().size() == chunks(m::L),
      m::s.decided.size() == 1, m::s.accepted.length() == 1, m::rulesOk(m::s),
    })
  run shapeAndReplayGuards = chainOf(m::L)
    // The decider accepts the root whatever the manifest (it does not read it); verify_chain
    // refuses a truncated or empty manifest, so no bundle is published with it.
    .expect(m::canDecide(m::s, soloRoot(m::L))
      and not(m::verifyChain(m::s, soloRoot(m::L), wrongManifest(soloRoot(m::L))))
      and not(m::canPublish(m::s, soloRoot(m::L), wrongManifest(soloRoot(m::L))))
      and not(m::verifyChain(m::s, soloRoot(m::L), [])))
    .then(decideWith(soloRoot(m::L)))
    .then(m::accept(stmtFor(m::L)))
    .expect(not(m::canAccept(m::s, stmtFor(m::L))))
    .then(decideWith(soloRoot(m::L)))   // the decider may re-run; the contract still refuses
    .expect(m::rulesOk(m::s) and m::s.accepted.length() == 1 and not(m::canAccept(m::s, stmtFor(m::L))))

  // join and pad are admitted iff TREE.
  run treeGuard = m::init.then(proveLeaves(2))
    .then(m::foldLeaf(m::NoNode, chunkId(2, 0)))                                              // node 0
    .then(if (chunks(2) == 1) m::leaf(1, 0, 1)
          else if (m::TREE) m::foldLeaf(m::NoNode, chunkId(2, 1)) else m::foldLeaf(m::NodeRef(0), chunkId(2, 1)))  // node 1
    .expect(m::canPad(m::s, 0) == m::TREE
      and m::canJoin(m::s, m::NodeRef(0), m::NoLeaf, 1) == (m::TREE and chunks(2) == 2))
    .then(if (m::TREE) m::pad(0) else idle)
    .then(if (m::TREE and chunks(2) == 2) m::join(m::NodeRef(0), m::NoLeaf, 1) else idle)
    .expect(m::rulesOk(m::s) and m::wPadded == m::TREE and m::wJoined == (m::TREE and chunks(2) == 2))

  // ---- tree schedules (aliased only in TREE instances)
  // Every shape over three leaves: ((ab)c), (a(bc)), three-slot (a b c), leaf-then-R,
  // L-then-leaf, and a unary pad over a root. Same statement, pairwise distinct manifests.
  pure val roots3 = [4, 6, 7, 8, 9, 10]
  run threeLeafShapes = m::init.then(3.reps(j => m::leaf(3, j, 1)))
    .then(3.reps(j => m::foldLeaf(m::NoNode, m::leafId(3, j, 1))))         // 0:[0,1) 1:[1,2) 2:[2,3)
    .then(m::join(m::NodeRef(0), m::NoLeaf, 1))                             // 3: (ab)   [0,2)
    .then(m::join(m::NodeRef(3), m::NoLeaf, 2))                             // 4: ((ab)c)
    .then(m::join(m::NodeRef(1), m::NoLeaf, 2))                             // 5: (bc)   [1,3)
    .then(m::join(m::NodeRef(0), m::NoLeaf, 5))                             // 6: (a(bc))
    .then(m::join(m::NodeRef(0), m::LeafRef(m::leafId(3, 1, 1)), 2))        // 7: three slots
    .then(m::join(m::NoNode, m::LeafRef(m::leafId(3, 0, 1)), 5))            // 8: leaf then R
    .then(m::foldLeaf(m::NodeRef(3), m::leafId(3, 2, 1)))                   // 9: L then leaf
    .then(m::pad(4))                                                        // 10: unary over a root
    .expect(and {
      not(m::canJoin(m::s, m::NodeRef(2), m::NoLeaf, 0)),                          // swap_children
      not(m::canJoin(m::s, m::NodeRef(3), m::LeafRef(m::leafId(3, 1, 1)), 2)),     // reuse_leaf under L
      not(m::canJoin(m::s, m::NodeRef(0), m::NoLeaf, 0)),                          // same child twice
      not(m::canJoin(m::s, m::NodeRef(3), m::NoLeaf, 5)),                          // overlapping segments
      m::rulesOk(m::s), m::s.nodes.keys().size() == 11,
      roots3.indices().forall(k => m::canPublish(m::s, roots3[k], manifestOf(roots3[k]))
        and m::stmtOf(nodeOf(roots3[k])) == stmtFor(3)),
      roots3.indices().forall(k => roots3.indices().forall(j =>
        k == j or manifestOf(roots3[k]) != manifestOf(roots3[j]))),
      nodeOf(10).depth == 4, m::segOfNode(nodeOf(10)) == m::segOfNode(nodeOf(4)),
      m::wTwoRootsOneStmt, m::wThreeSlots, m::wPadded,
    })
    .then(roots3.length().reps(k => decideWith(roots3[k])))
    .then(m::accept(stmtFor(3)))
    .expect(m::rulesOk(m::s) and m::s.decided.size() == 6 and m::s.accepted.length() == 1
      and not(m::canAccept(m::s, stmtFor(3))))

  // Random bracketing: the frontier is the set of maximal certificates of the tree under
  // construction; grow it by any admissible fold, three-slot join, leaf-then-R or node join.
  pure def frontier(st: m::State): Set[int] = st.nodes.keys().filter(n =>
    not(st.nodes.keys().exists(o => st.nodes.get(n).cover != st.nodes.get(o).cover
      and st.nodes.get(n).cover.subseteq(st.nodes.get(o).cover))))
  pure def uncovered(st: m::State): Set[m::LeafId] =
    st.leaves.keys().filter(x => not(st.nodes.keys().exists(n => st.nodes.get(n).cover.contains(x))))
  pure def covered(st: m::State, p: int): bool = st.nodes.keys().exists(n =>
    st.nodes.get(n).hLo == 0 and st.nodes.get(n).hHi == p)
  action growFrontier = any {
    { nondet x = uncovered(m::s).oneOf()
      m::foldLeaf(m::NoNode, x) },
    { nondet a = frontier(m::s).oneOf()
      nondet x = uncovered(m::s).filter(x => m::canNode(m::s, m::NodeRef(a), m::LeafRef(x), m::NoNode)).oneOf()
      m::foldLeaf(m::NodeRef(a), x) },
    { nondet a = frontier(m::s).oneOf()
      nondet b = frontier(m::s).filter(b => m::canNode(m::s, m::NodeRef(a), m::NoLeaf, m::NodeRef(b))).oneOf()
      m::join(m::NodeRef(a), m::NoLeaf, b) },
    { nondet x = uncovered(m::s).oneOf()
      nondet b = frontier(m::s).filter(b => m::canNode(m::s, m::NoNode, m::LeafRef(x), m::NodeRef(b))).oneOf()
      m::join(m::NoNode, m::LeafRef(x), b) },
    { nondet a = frontier(m::s).oneOf()
      nondet x = uncovered(m::s).oneOf()
      nondet b = frontier(m::s).filter(b => m::canNode(m::s, m::NodeRef(a), m::LeafRef(x), m::NodeRef(b))).oneOf()
      m::join(m::NodeRef(a), m::LeafRef(x), b) },
  }
  action growOrIdle = if (covered(m::s, m::L)) idle else growFrontier
  action decideFrontier = {
    nondet r = frontier(m::s).oneOf()
    decideWith(r)
  }
  run randomBracketing = m::init.then(proveLeaves(m::L))
    .then((2 * chunks(m::L)).reps(_ => growOrIdle.expect(m::rulesOk(m::s))))
    .expect(covered(m::s, m::L) and frontier(m::s).size() == 1
      and frontier(m::s).forall(r => m::canPublish(m::s, r, manifestOf(r)) and m::stmtOf(nodeOf(r)) == stmtFor(m::L)))
    .then(decideFrontier)
    .then(m::accept(stmtFor(m::L)))
    .expect(m::rulesOk(m::s) and m::s.accepted == [stmtFor(m::L)])

  // Random interleaving of the model's own step (honest and adversarial) keeps every invariant.
  run randomSteps = m::init.then(25.reps(_ => m::step.expect(m::rulesOk(m::s))))

  // ---- seeded mutants: each bypasses one constraint and is caught by the named invariant
  pure val c0: m::LeafId = chunkId(m::L, 0)
  pure val lastChunk: m::LeafId = chunkId(m::L, chunks(m::L) - 1)
  pure def certifyRaw(st: m::State, n: m::Node): m::State =
    { ...st, nodes: st.nodes.put(n.id, n), nextNode: n.id + 1 }

  // M1 (I1): a node certified without the linkage check: chunk 0 folded onto the root
  // (statement composed from unlinked slots).
  action mutantLinkage = m::s' = certifyRaw(m::s, m::mkNode(m::s, m::NodeRef(soloRoot(m::L)), m::LeafRef(c0), m::NoNode))
  run mutantLinkageCaught = chainOf(m::L).expect(m::rulesOk(m::s)).then(mutantLinkage)
    .expect(not(m::linkageOk(m::s)) and not(m::allInvariants))

  // M2 (I2): the range check dropped: a leaf claiming S+1 steps is folded.
  action mutantRange = {
    val big = { ...m::mkLeaf(m::L, 0, 1), steps: m::S + 1, id: m::leafId(m::L, 0, m::S + 1) }
    val st = { ...m::s, leaves: m::s.leaves.put(big.id, big) }
    m::s' = certifyRaw(st, m::mkNode(st, m::NoNode, m::LeafRef(big.id), m::NoNode))
  }
  run mutantRangeCaught = chainOf(m::L).then(mutantRange)
    .expect(not(m::heightsOk(m::s)) and not(m::allInvariants))

  // M3 (I3): a leaf consumed twice: the last chunk folded again onto the root, heights advanced.
  action mutantReuse = {
    val n = m::mkNode(m::s, m::NodeRef(soloRoot(m::L)), m::LeafRef(lastChunk), m::NoNode)
    val hi = nodeOf(soloRoot(m::L)).hHi + lastChunk.steps
    m::s' = certifyRaw(m::s, { ...n, hHi: hi, post: m::d(m::L, hi) })
  }
  run mutantReuseCaught = chainOf(m::L).then(mutantReuse)
    .expect(not(m::coverageOk(m::s)) and not(m::allInvariants))

  // M4 (I4): a disabled slot that changes state (non-neutral padding counted as the step).
  action mutantPadding = {
    val x = m::mkLeaf(m::L, 0, 1)
    val bad = { ...x, slots: x.slots.replaceAt(0, { ...x.slots[0], enabled: false }) }
    m::s' = { ...m::s, leaves: m::s.leaves.put(bad.id, bad) }
  }
  run mutantPaddingCaught = chainOf(m::L).then(mutantPadding)
    .expect(not(m::paddingOk(m::s)) and not(m::allInvariants))
  // M4b (I4): a unary (padding) node whose statement differs from its child's.
  action mutantUnary = {
    val n = m::mkNode(m::s, m::NodeRef(soloRoot(m::L)), m::NoLeaf, m::NoNode)
    m::s' = certifyRaw(m::s, { ...n, hHi: n.hHi + 1, post: m::d(m::L, n.hHi + 1) })
  }
  run mutantUnaryCaught = chainOf(m::L).then(mutantUnary)
    .expect(not(m::paddingOk(m::s)) and not(m::allInvariants))

  // M5 (I5): a second root over the same heights with a different statement.
  action mutantSchedule = {
    val r = nodeOf(soloRoot(m::L))
    m::s' = certifyRaw(m::s, { ...r, id: m::s.nextNode, post: m::d(m::L, m::L + 1),
      acc: r.acc.replaceAt(r.acc.length() - 1, m::NodeC(m::s.nextNode)) })
  }
  run mutantScheduleCaught = chainOf(m::L).then(mutantSchedule)
    .expect(not(m::scheduleOk(m::s)) and not(m::allInvariants))

  // M6 (I6): the decider accepts a root that does not start at genesis (hLo != 0).
  // The non-genesis segment [1, 2) is certified raw (the chain node refuses it, the tree node
  // admits it), then decided without the hLo == 0 check.
  action mutantGenesis = {
    val n = m::mkNode(m::s, m::NoNode, m::LeafRef(m::leafId(2, 1, 1)), m::NoNode)
    m::s' = { ...certifyRaw(m::s, n), decided: m::s.decided.union(Set({ stmt: m::stmtOf(n), root: n.id,
      manifest: n.man, key: m::DeciderKey })) }
  }
  run mutantGenesisCaught = m::init.then(m::leaf(2, 1, 1))
    .expect(m::canNode(m::s, m::NoNode, m::LeafRef(m::leafId(2, 1, 1)), m::NoNode) == m::TREE)
    .then(mutantGenesis)
    .expect(not(m::canDecide(m::s, 0)) and not(m::genesisOk(m::s)) and not(m::allInvariants))

  // M7 (I7): the contract accepts a replayed statement.
  action mutantReplay = m::s' = { ...m::s, accepted: m::s.accepted.append(stmtFor(m::L)) }
  run mutantReplayCaught = chainOf(m::L).then(decideAndAcceptAt(soloRoot(m::L), m::L)).then(mutantReplay)
    .expect(not(m::replayOk(m::s)) and not(m::allInvariants))

  // M8 (I8): a node whose leaf belongs to another program.
  action mutantProgram = m::s' = certifyRaw(m::s, m::mkNode(m::s, m::NodeRef(0), m::LeafRef(m::leafId(1, 0, 1)), m::NoNode))
  run mutantProgramCaught = chainOf(m::L).then(m::leaf(1, 0, 1)).then(mutantProgram)
    .expect(not(m::programOk(m::s)) and not(m::allInvariants))

  // M9 (I9): a decision whose manifest is not the root's fold order.
  action mutantShape = {
    val n = nodeOf(soloRoot(m::L))
    m::s' = { ...m::s, decided: m::s.decided.union(Set({ stmt: m::stmtOf(n), root: n.id,
      manifest: n.man.slice(1, n.man.length()), key: m::DeciderKey })) }
  }
  run mutantShapeCaught = chainOf(m::L).then(mutantShape)
    .expect(not(m::shapeOk(m::s)) and not(m::allInvariants))

  // M10 (I10): an over-strict node constraint (the "post == pre under one enabled slot" class)
  // refuses every honest leaf, so no chain can be built: the liveness witness is unreachable.
  pure def canNodeStrict(st: m::State, l: m::OptNode, lf: m::OptLeaf, r: m::OptNode): bool =
    m::canNode(st, l, lf, r) and (match lf {
      | LeafRef(x) => st.leaves.get(x).pre == st.leaves.get(x).post
      | NoLeaf => true })
  run mutantLivenessCaught = m::init.then(proveLeaves(m::L))
    .expect(m::canNode(m::s, m::NoNode, m::LeafRef(c0), m::NoNode)
      and not(canNodeStrict(m::s, m::NoNode, m::LeafRef(c0), m::NoNode)))
    .then(m::foldLeaf(m::NoNode, c0))
    .expect(m::wFolded)

  // MK (fixed keys): a node keyed per level (ForeignKey(depth)); a node over it is refused.
  action mutantKeys = {
    val n = { ...m::mkNode(m::s, m::NoNode, m::LeafRef(c0), m::NoNode), key: m::ForeignKey(1) }
    m::s' = certifyRaw(m::s, n)
  }
  run mutantKeysCaught = chainOf(m::L).then(mutantKeys)
    .expect(and {
      not(m::keysOk(m::s)), not(m::allInvariants),
      not(m::canNode(m::s, m::NodeRef(m::s.nextNode - 1), m::NoLeaf, m::NoNode)),
      chunks(m::L) == 1 or
        not(m::canNode(m::s, m::NodeRef(m::s.nextNode - 1), m::LeafRef(chunkId(m::L, 1)), m::NoNode)),
    })

  // MT (shape guard): a join certified although TREE is false.
  action mutantJoinInChain = {
    val n = m::mkNode(m::s, m::NodeRef(0), m::NoLeaf, m::NodeRef(0))
    m::s' = certifyRaw(m::s, { ...n, cover: nodeOf(0).cover, acc: nodeOf(0).acc.append(m::NodeC(n.id)) })
  }
  run mutantJoinInChainCaught = chainOf(m::L).then(mutantJoinInChain)
    .expect(not(m::chainOnly(m::s)) and not(m::allInvariants))

  // MP (I9, S3): the bundled manifest moves a unary child from L to R. The decider does not
  // read it, and the flattened manifest of the first F20 draft is identical, so only the
  // structured manifest and verify_chain can refuse it. TREE instances only (pad).
  pure def moveLtoR(e: m::ManEntry): m::ManEntry = { ...e, hasL: false, l: m::NoNode, hasR: true, r: e.l }
  def placedManifest(id: int): List[m::ManEntry] = {
    val man = manifestOf(id)
    man.replaceAt(man.length() - 1, moveLtoR(man[man.length() - 1]))
  }
  action mutantPlacementLR = {
    val id = m::s.nextNode - 1
    m::s' = { ...m::s, decided: m::s.decided.union(Set({ stmt: m::stmtOf(nodeOf(id)), root: id,
      manifest: placedManifest(id), key: m::DeciderKey })) }
  }
  run mutantPlacementLRCaught = chainOf(m::L).then(m::pad(soloRoot(m::L)))
    .expect(and {
      m::canDecide(m::s, m::s.nextNode - 1),
      m::canPublish(m::s, m::s.nextNode - 1, manifestOf(m::s.nextNode - 1)),
      not(m::verifyChain(m::s, m::s.nextNode - 1, placedManifest(m::s.nextNode - 1))),
      placedManifest(m::s.nextNode - 1) != manifestOf(m::s.nextNode - 1),
      m::flatMan(placedManifest(m::s.nextNode - 1)) == m::flatMan(manifestOf(m::s.nextNode - 1)),
    })
    .then(mutantPlacementLR)
    .expect(not(m::shapeOk(m::s)) and not(m::allInvariants))

  // MC (I11, S1): a genesis node folds the witness-supplied carried accumulator unscaled (the
  // SHKIVC05 circuit); the carry is another node's fold, an unauthenticated obligation.
  action mutantGenesisCarry = {
    val n = m::mkNode(m::s, m::NoNode, m::LeafRef(c0), m::NoNode)
    m::s' = certifyRaw(m::s, { ...n, acc: nodeOf(soloRoot(m::L)).acc.concat(n.acc) })
  }
  run mutantGenesisCarryCaught = chainOf(m::L).then(mutantGenesisCarry)
    .expect(not(m::carryOk(m::s)) and not(m::allInvariants))
  // Realization (a): the same adversarial carry through the node constraints is collapsed away.
  run genesisCarryNeutral = chainOf(m::L).then(m::genesis_carry)
    .expect(and {
      m::rulesOk(m::s), m::s.nodes.keys().size() == chunks(m::L) + 1,
      nodeOf(m::s.nextNode - 1).acc == [m::LeafC(c0), m::NodeC(m::s.nextNode - 1)],
    })
}

// Q-C scheduler cases.
module scheduler_case {
  const CASE_P: int
  const CASE_N: int
  const CASE_LEAF: int
  const CASE_NODE: int
  import leaf_scheduler(P = CASE_P, N = CASE_N, LEAF_GB = CASE_LEAF, NODE_GB = CASE_NODE) as q from "./aggregation"

  pure def first(ids: Set[int]): int = ids.fold(1000000, (a, b) => if (a < b) a else b)
  pure def minI(a: int, b: int): int = if (a < b) a else b
  // leaves admissible while the node is idle: workers, leaves, and the room left by the reserve
  pure val lookaheadLeaves: int = minI(minI(q::P, q::N), (q::AGG_CAP - q::NODE_GB) / q::LEAF_GB)
  // Deterministic drive: nodes first, then leaves; idles once every node is proved.
  action drain =
    if (q::done(q::w)) q::w' = q::w
    else if (q::IDX.exists(i => q::canFinishNode(q::w, i))) q::finishNode(first(q::IDX.filter(i => q::canFinishNode(q::w, i))))
    else if (q::IDX.exists(i => q::canStartNode(q::w, i))) q::startNode(first(q::IDX.filter(i => q::canStartNode(q::w, i))))
    else if (q::IDX.exists(i => q::canStartLeaf(q::w, i))) q::startLeaf(first(q::IDX.filter(i => q::canStartLeaf(q::w, i))))
    else q::finishLeaf(first(q::IDX.filter(i => q::canFinishLeaf(q::w, i))))
  action stepOrIdle = if (q::done(q::w)) q::w' = q::w else q::step
  action startIfAdmissible(i: int): bool = if (q::canStartLeaf(q::w, i)) q::startLeaf(i) else q::w' = q::w

  run complete = q::init.expect(q::schedOk(q::w))
    .then((6 * q::N).reps(_ => drain.expect(q::schedOk(q::w))))
    .expect(q::done(q::w) and q::inflight(q::w) == 0)
  run randomSchedule = q::init.then((8 * q::N).reps(_ => stepOrIdle.expect(q::schedOk(q::w))))
  // Look-ahead leaves fill the workers but leave room for the node.
  run lookahead = q::init.then(q::P.reps(i => startIfAdmissible(i)))
    .expect(and {
      q::schedOk(q::w),
      q::count(q::w.leaf, q::Proving) == lookaheadLeaves,
      q::inflight(q::w) + q::NODE_GB <= q::AGG_CAP,
    })
    .then(q::finishLeaf(0)).then(q::startNode(0))
    .expect(q::schedOk(q::w) and q::count(q::w.node, q::Proving) == 1 and not(q::canStartNode(q::w, 1)))
  run infeasibleCapsDetected = q::init.expect(not(q::progressOk(q::w)))

  // Mutants
  action mutantOverAdmit = q::w' = { ...q::w, leaf: q::IDX.mapBy(_ => q::Proving) }
  run mutantOverAdmitCaught = q::init.then(mutantOverAdmit)
    .expect(not(q::aggregateOk(q::w)) and not(q::QC_all))
  action mutantNodeEarly = q::w' = { ...q::w, node: q::w.node.set(0, q::Proving) }
  run mutantNodeEarlyCaught = q::init.then(mutantNodeEarly)
    .expect(not(q::orderOk(q::w)) and not(q::QC_all))
  // No reservation: P leaves admitted while the node is idle and ready -> the node is starved.
  action mutantNoReserve = q::w' = { ...q::w,
    leaf: q::IDX.mapBy(i => if (i == 0) q::Proved else if (i <= q::P) q::Proving else q::Idle) }
  run mutantNoReserveCaught = q::init.then(mutantNoReserve)
    .expect(not(q::nodeAdmissible(q::w)) and not(q::QC_all))
}

module aggregation_test {
  import aggregation_case(CASE_TREE = false, CASE_S = 1, CASE_L = 9) as c1
  import aggregation_case(CASE_TREE = false, CASE_S = 2, CASE_L = 9) as c2
  import aggregation_case(CASE_TREE = false, CASE_S = 4, CASE_L = 9) as c4
  import aggregation_case(CASE_TREE = false, CASE_S = 8, CASE_L = 9) as c8
  import aggregation_case(CASE_TREE = true, CASE_S = 1, CASE_L = 9) as t1
  import aggregation_case(CASE_TREE = true, CASE_S = 2, CASE_L = 9) as t2
  import aggregation_case(CASE_TREE = true, CASE_S = 4, CASE_L = 9) as t4
  import aggregation_case(CASE_TREE = true, CASE_S = 8, CASE_L = 9) as t8
  import scheduler_case(CASE_P = 1, CASE_N = 4, CASE_LEAF = 16, CASE_NODE = 26) as q1
  import scheduler_case(CASE_P = 3, CASE_N = 8, CASE_LEAF = 16, CASE_NODE = 26) as q3
  import scheduler_case(CASE_P = 5, CASE_N = 8, CASE_LEAF = 16, CASE_NODE = 26) as q5
  import scheduler_case(CASE_P = 3, CASE_N = 4, CASE_LEAF = 40, CASE_NODE = 26) as q40
  import scheduler_case(CASE_P = 1, CASE_N = 2, CASE_LEAF = 60, CASE_NODE = 48) as qbad

  // Linear chain, S in {1, 2, 4, 8}
  run chainS1EveryProgramTest = c1::everyProgramChain
  run chainS2EveryProgramTest = c2::everyProgramChain
  run chainS4EveryProgramTest = c4::everyProgramChain
  run chainS8EveryProgramTest = c8::everyProgramChain
  run chainS1LeafGuardsTest = c1::leafGuards
  run chainS8LeafGuardsTest = c8::leafGuards
  run chainS1NodeGuardsTest = c1::nodeGuards
  run chainS2NodeGuardsTest = c2::nodeGuards
  run chainS1AdversaryRefusedTest = c1::adversaryRefused
  run chainS4AdversaryRefusedTest = c4::adversaryRefused
  run chainS1ShapeAndReplayTest = c1::shapeAndReplayGuards
  run chainS2ShapeAndReplayTest = c2::shapeAndReplayGuards
  run chainS1TreeGuardTest = c1::treeGuard
  run chainS2TreeGuardTest = c2::treeGuard
  run chainS1RandomStepsTest = c1::randomSteps
  run chainS2RandomStepsTest = c2::randomSteps
  // Tree, S in {1, 2, 4, 8}
  run treeS1EveryProgramTest = t1::everyProgramChain
  run treeS2EveryProgramTest = t2::everyProgramChain
  run treeS4EveryProgramTest = t4::everyProgramChain
  run treeS8EveryProgramTest = t8::everyProgramChain
  run treeS1NodeGuardsTest = t1::nodeGuards
  run treeS4NodeGuardsTest = t4::nodeGuards
  run treeS1AdversaryRefusedTest = t1::adversaryRefused
  run treeS8AdversaryRefusedTest = t8::adversaryRefused
  run treeS1ShapeAndReplayTest = t1::shapeAndReplayGuards
  run treeS1TreeGuardTest = t1::treeGuard
  run treeS2TreeGuardTest = t2::treeGuard
  run treeS1ThreeLeafShapesTest = t1::threeLeafShapes
  run treeS1RandomBracketingTest = t1::randomBracketing
  run treeS2RandomBracketingTest = t2::randomBracketing
  run treeS4RandomBracketingTest = t4::randomBracketing
  run treeS8RandomBracketingTest = t8::randomBracketing
  run treeS1RandomStepsTest = t1::randomSteps
  run treeS2RandomStepsTest = t2::randomSteps
  // Mutants: chain S=1, chain S=4, tree S=1, tree S=8
  run chainS1MutantLinkageCaughtTest = c1::mutantLinkageCaught
  run chainS1MutantRangeCaughtTest = c1::mutantRangeCaught
  run chainS1MutantReuseCaughtTest = c1::mutantReuseCaught
  run chainS1MutantPaddingCaughtTest = c1::mutantPaddingCaught
  run chainS1MutantUnaryCaughtTest = c1::mutantUnaryCaught
  run chainS1MutantScheduleCaughtTest = c1::mutantScheduleCaught
  run chainS1MutantGenesisCaughtTest = c1::mutantGenesisCaught
  run chainS1MutantReplayCaughtTest = c1::mutantReplayCaught
  run chainS1MutantProgramCaughtTest = c1::mutantProgramCaught
  run chainS1MutantShapeCaughtTest = c1::mutantShapeCaught
  run chainS1MutantLivenessCaughtTest = c1::mutantLivenessCaught
  run chainS1MutantKeysCaughtTest = c1::mutantKeysCaught
  run chainS1MutantJoinInChainCaughtTest = c1::mutantJoinInChainCaught
  run chainS4MutantLinkageCaughtTest = c4::mutantLinkageCaught
  run chainS4MutantRangeCaughtTest = c4::mutantRangeCaught
  run chainS4MutantReuseCaughtTest = c4::mutantReuseCaught
  run chainS4MutantPaddingCaughtTest = c4::mutantPaddingCaught
  run chainS4MutantUnaryCaughtTest = c4::mutantUnaryCaught
  run chainS4MutantScheduleCaughtTest = c4::mutantScheduleCaught
  run chainS4MutantGenesisCaughtTest = c4::mutantGenesisCaught
  run chainS4MutantReplayCaughtTest = c4::mutantReplayCaught
  run chainS4MutantProgramCaughtTest = c4::mutantProgramCaught
  run chainS4MutantShapeCaughtTest = c4::mutantShapeCaught
  run chainS4MutantLivenessCaughtTest = c4::mutantLivenessCaught
  run chainS4MutantKeysCaughtTest = c4::mutantKeysCaught
  run chainS4MutantJoinInChainCaughtTest = c4::mutantJoinInChainCaught
  run treeS1MutantLinkageCaughtTest = t1::mutantLinkageCaught
  run treeS1MutantRangeCaughtTest = t1::mutantRangeCaught
  run treeS1MutantReuseCaughtTest = t1::mutantReuseCaught
  run treeS1MutantPaddingCaughtTest = t1::mutantPaddingCaught
  run treeS1MutantUnaryCaughtTest = t1::mutantUnaryCaught
  run treeS1MutantScheduleCaughtTest = t1::mutantScheduleCaught
  run treeS1MutantGenesisCaughtTest = t1::mutantGenesisCaught
  run treeS1MutantReplayCaughtTest = t1::mutantReplayCaught
  run treeS1MutantProgramCaughtTest = t1::mutantProgramCaught
  run treeS1MutantShapeCaughtTest = t1::mutantShapeCaught
  run treeS1MutantLivenessCaughtTest = t1::mutantLivenessCaught
  run treeS1MutantKeysCaughtTest = t1::mutantKeysCaught
  run treeS8MutantLinkageCaughtTest = t8::mutantLinkageCaught
  run treeS8MutantRangeCaughtTest = t8::mutantRangeCaught
  run treeS8MutantReuseCaughtTest = t8::mutantReuseCaught
  run treeS8MutantPaddingCaughtTest = t8::mutantPaddingCaught
  run treeS8MutantUnaryCaughtTest = t8::mutantUnaryCaught
  run treeS8MutantScheduleCaughtTest = t8::mutantScheduleCaught
  run treeS8MutantGenesisCaughtTest = t8::mutantGenesisCaught
  run treeS8MutantReplayCaughtTest = t8::mutantReplayCaught
  run treeS8MutantProgramCaughtTest = t8::mutantProgramCaught
  run treeS8MutantShapeCaughtTest = t8::mutantShapeCaught
  run treeS8MutantLivenessCaughtTest = t8::mutantLivenessCaught
  run treeS8MutantKeysCaughtTest = t8::mutantKeysCaught
  // S3 placement and S1 genesis carry (council review 2.14)
  run treeS1MutantPlacementLRCaughtTest = t1::mutantPlacementLRCaught
  run treeS8MutantPlacementLRCaughtTest = t8::mutantPlacementLRCaught
  run chainS1MutantGenesisCarryCaughtTest = c1::mutantGenesisCarryCaught
  run chainS4MutantGenesisCarryCaughtTest = c4::mutantGenesisCarryCaught
  run treeS1MutantGenesisCarryCaughtTest = t1::mutantGenesisCarryCaught
  run treeS8MutantGenesisCarryCaughtTest = t8::mutantGenesisCarryCaught
  run chainS1GenesisCarryNeutralTest = c1::genesisCarryNeutral
  run chainS2GenesisCarryNeutralTest = c2::genesisCarryNeutral
  run treeS1GenesisCarryNeutralTest = t1::genesisCarryNeutral
  run treeS4GenesisCarryNeutralTest = t4::genesisCarryNeutral
  // Q-C scheduler
  run schedP1CompleteTest = q1::complete
  run schedP3CompleteTest = q3::complete
  run schedP5CompleteTest = q5::complete
  run schedLeaf40CompleteTest = q40::complete
  run schedP3RandomTest = q3::randomSchedule
  run schedP5RandomTest = q5::randomSchedule
  run schedLeaf40RandomTest = q40::randomSchedule
  run schedP3LookaheadTest = q3::lookahead
  run schedP5LookaheadTest = q5::lookahead
  run schedLeaf40LookaheadTest = q40::lookahead
  run schedInfeasibleCapsDetectedTest = qbad::infeasibleCapsDetected
  run schedMutantOverAdmitCaughtTest = q3::mutantOverAdmitCaught
  run schedMutantNodeEarlyCaughtTest = q3::mutantNodeEarlyCaught
  run schedMutantNoReserveCaughtTest = q5::mutantNoReserveCaught
}
