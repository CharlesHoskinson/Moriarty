// Deterministic checks for design point D7 (frame protocol) on two chained leaves.
//   cached:   chipLeafCached.qnt (top in registers; PUSH spills the old top with link fp)
//   uncached: chipLeafChain.qnt over chipLeaf.qnt (a rewrite appends and moves fp)
// Run (one module at a time):
//   quint test chipLeafCached_test.qnt --main cachedTest
//   quint test chipLeafCached_test.qnt --main cachedTest_<mutant>
//   quint test chipLeafCached_test.qnt --main uncachedTest
//   quint test chipLeafCached_test.qnt --main uncachedTest_<mutant>
//
// Program PF (rows 0..13): LIT r0 5; ALLOC r0 -> r1; LIT r0 1; PUSH r0; LIT r0 2; PUSH r0;
//   LIT r0 3; PUSH r0; LIT r0 4; SETTOP r0; POP r1; POP r1; POP r1; HALT.
// The chain starts after rows 0..4 (warm 5: one frame, value 1). Leaf 1 runs rows 5..9 (two
// PUSH, the SETTOP); leaf 2 runs rows 10..13 (three POP of frames written in earlier leaves, read
// through PAGE; then HALT). Reference POP results: [4, 2, 1].
// Program PL (rows 0..9): LIT r0 1; PUSH r0; LIT r0 2; PUSH r0; LIT r0 3; PUSH r0; POP r1; POP r1;
//   POP r1; HALT. Chain after row 0; leaf 1 runs the three PUSH from an empty memory, so a
//   forged link 0 points at a frame entry. Reference POP results: [3, 2, 1].

module cached_case {
  const CASE_MUTANT: str
  import chipLeafCached(S = 5, N_MEM = 3, N_PAGE = 12, N_ALU = 2, MUTANT = CASE_MUTANT) as m from "./chipLeafCached"

  pure val PF: List[m::Ins] = [
    { op: m::OP_LIT, x: 0, y: 5 }, { op: m::OP_ALLOC, x: 0, y: 1 },
    { op: m::OP_LIT, x: 0, y: 1 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 2 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 3 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 4 }, { op: m::OP_SETTOP, x: 0, y: 0 },
    { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 },
    m::HALT_INS,
  ]
  pure val PL: List[m::Ins] = [
    { op: m::OP_LIT, x: 0, y: 1 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 2 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 3 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 },
    m::HALT_INS,
  ]
  pure val RF: m::RefSt = m::warm(PF, 5)
  pure val RL: m::RefSt = m::warm(PL, 1)
  pure def leaf1(p: List[m::Ins], r: m::RefSt): m::W = m::genLeaf(p, r, 5, 0, "none", "cell").w
  pure def leaf2(p: List[m::Ins], a: m::W, pre: m::Regs): m::W =
    m::genLeafFrom(p, pre, a.memOut, 5, 0, "none", "cell").w
  pure val F1: m::W = leaf1(PF, RF)
  pure val F2: m::W = leaf2(PF, F1, F1.post)
  pure val L1: m::W = leaf1(PL, RL)
  pure val L2: m::W = leaf2(PL, L1, L1.post)
  // top_dropped: leaf 2 starts with a free top (0 instead of 4)
  pure val F2LIE: m::W = leaf2(PF, F1, { ...F1.post, top: 0 })
  pure def chainSound(a: m::W, b: m::W, r: m::RefSt): bool = and {
    m::popsOk(a, b, r), m::popsExecOk(a, b, r),
    m::refineOk(a, r), m::framesOk(a, r),
    m::refineOk(b, m::refRun(a.prog, r, a.steps)), m::framesOk(b, m::refRun(a.prog, r, a.steps)),
  }

  // ---- reference: cached layout and POP results
  run refTest = m::init.expect(and {
    RF.stack == [{ v: 1, at: 1 }], RF.mem == [{ a: 5, b: m::NIL }, { a: 0, b: m::NIL }],
    m::refPops(PF, RF, 9) == [4, 2, 1],
    m::refRun(PF, RF, 5).mem.length() == 4,          // SETTOP appends nothing
    m::refRun(PF, RF, 9).status == m::HALTED,
    m::refPops(PL, RL, 9) == [3, 2, 1],
  })

  // ---- honest chains are accepted and sound; leaf 2 pops frames through PAGE
  run honestTest = m::init.expect(CASE_MUTANT != "none" or and {
    m::chainAccepts(F1, F2), chainSound(F1, F2, RF),
    F1.steps == 5, F2.steps == 4, F2.post.status == m::HALTED,
    m::popsImpl(F2) == [4, 2, 1],
    F1.post.len == 4, F1.post.top == 4, F1.post.fp == 3, F1.post.depth == 3,
    m::toSet(F2.slots[0].bus).exists(r => r.chip == m::T_CELL and r.key == 3) and F2.page.exists(r => r.chip == m::T_CELL and r.key == 3),
    m::chainAccepts(L1, L2), chainSound(L1, L2, RL), m::popsImpl(L2) == [3, 2, 1],
    // the boundary lie is refused: the statement carries top
    m::accepts(F2LIE), not(m::chainAccepts(F1, F2LIE)),
  })

  // ---- each D7 mutant: the chain is accepted and the executed POP results (no drain) differ from
  // the reference. Expected results: rewrite_appends [4, 4, 2]; push_link_free on PL [3, 2, 0];
  // pop_no_reload [4, 4, 4]; pop_no_fp_reload [4, 2, 2]; top_dropped with pre.top = 0 [0, 2, 1].
  run mutantCaughtTest = m::init.expect(
    if (CASE_MUTANT == "rewrite_appends")
      and { m::chainAccepts(F1, F2), m::popsImpl(F2) == [4, 4, 2], not(m::popsExecOk(F1, F2, RF)) }
    else if (CASE_MUTANT == "push_link_free")
      and { m::chainAccepts(L1, L2), m::popsImpl(L2) == [3, 2, 0], not(m::popsExecOk(L1, L2, RL)) }
    else if (CASE_MUTANT == "pop_no_reload")
      and { m::chainAccepts(F1, F2), m::popsImpl(F2) == [4, 4, 4], not(m::popsExecOk(F1, F2, RF)) }
    else if (CASE_MUTANT == "pop_no_fp_reload")
      and { m::chainAccepts(F1, F2), m::popsImpl(F2) == [4, 2, 2], not(m::popsExecOk(F1, F2, RF)) }
    else if (CASE_MUTANT == "top_dropped")
      and { m::chainAccepts(F1, F2LIE), m::popsImpl(F2LIE) == [0, 2, 1], not(m::popsExecOk(F1, F2LIE, RF)) }
    else true)
}

module uncached_case {
  const CASE_MUTANT: str
  import chipLeafChain(S = 5, N_MEM = 3, N_PAGE = 12, N_ALU = 2, MUTANT = CASE_MUTANT) as c from "./chipLeafChain"
  import chipLeaf(S = 5, N_MEM = 3, N_PAGE = 12, N_ALU = 2, MUTANT = CASE_MUTANT) as m from "./chipLeaf"

  pure val PF: List[m::Ins] = [
    { op: m::OP_LIT, x: 0, y: 5 }, { op: m::OP_ALLOC, x: 0, y: 1 },
    { op: m::OP_LIT, x: 0, y: 1 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 2 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 3 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 4 }, { op: m::OP_SETTOP, x: 0, y: 0 },
    { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 },
    m::HALT_INS,
  ]
  pure val PL: List[m::Ins] = [
    { op: m::OP_LIT, x: 0, y: 1 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 2 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_LIT, x: 0, y: 3 }, { op: m::OP_PUSH, x: 0, y: 0 },
    { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 }, { op: m::OP_POP, x: 1, y: 0 },
    m::HALT_INS,
  ]
  pure val RF: m::RefSt = m::warm(PF, 5)
  pure val RL: m::RefSt = m::warm(PL, 1)
  pure def leaf1(p: List[m::Ins], r: m::RefSt): m::W = m::genLeaf(p, r, 5, 0, "none", "cell").w
  pure def leaf2(p: List[m::Ins], a: m::W): m::W = c::genLeafFrom(p, a.post, a.memOut, 5, 0, "none", "cell").w
  pure val F1: m::W = leaf1(PF, RF)
  pure val F2: m::W = leaf2(PF, F1)
  pure val L1: m::W = leaf1(PL, RL)
  pure val L2: m::W = leaf2(PL, L1)

  run honestTest = m::init.expect(CASE_MUTANT != "none" or and {
    c::chainAccepts(F1, F2), c::popsOk(F1, F2, RF), c::popsExecOk(F1, F2, RF),
    c::popsImpl(F2) == [4, 2, 1], F2.post.status == m::HALTED,
    F1.post.len == 5,                                  // the rewrite appended
    c::chainAccepts(L1, L2), c::popsOk(L1, L2, RL), c::popsImpl(L2) == [3, 2, 1],
  })
  run mutantCaughtTest = m::init.expect(
    if (CASE_MUTANT == "rewrite_no_move")
      and { c::chainAccepts(F1, F2), c::popsImpl(F2) == [3, 2, 1], not(c::popsExecOk(F1, F2, RF)) }
    else if (CASE_MUTANT == "push_link_free")
      and { c::chainAccepts(L1, L2), c::popsImpl(L2) == [3, 1, 1], not(c::popsExecOk(L1, L2, RL)) }
    else true)
}

module cachedTest { import cached_case(CASE_MUTANT = "none").* }
module cachedTest_rewrite_appends { import cached_case(CASE_MUTANT = "rewrite_appends").* }
module cachedTest_push_link_free { import cached_case(CASE_MUTANT = "push_link_free").* }
module cachedTest_pop_no_reload { import cached_case(CASE_MUTANT = "pop_no_reload").* }
module cachedTest_pop_no_fp_reload { import cached_case(CASE_MUTANT = "pop_no_fp_reload").* }
module cachedTest_top_dropped { import cached_case(CASE_MUTANT = "top_dropped").* }
module uncachedTest { import uncached_case(CASE_MUTANT = "none").* }
module uncachedTest_rewrite_no_move { import uncached_case(CASE_MUTANT = "rewrite_no_move").* }
module uncachedTest_push_link_free { import uncached_case(CASE_MUTANT = "push_link_free").* }
