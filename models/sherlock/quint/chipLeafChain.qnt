// Two chained leaves of the UNCACHED frame protocol (chipLeaf.qnt), for design point D7.
// chipLeaf.qnt is imported unchanged. Leaf 2 is built from leaf 1's post registers and committed
// memory, not from the reference. invPops is the same frame-stack refinement invariant as in
// chipLeafCached.qnt: the POP results of both leaves, then the POPs that drain the final stack,
// equal those of the reference stack. Note: research/sherlock-processor-analogy-20261110/d7-frames.md.
module chipLeafChain {
  const S: int
  const N_MEM: int
  const N_PAGE: int
  const N_ALU: int
  const MUTANT: str
  import chipLeaf(S = S, N_MEM = N_MEM, N_PAGE = N_PAGE, N_ALU = N_ALU, MUTANT = MUTANT) as m from "./chipLeaf"

  // m::genLeaf with the pre statement given directly (same body; chipLeaf.qnt is not changed).
  pure def genLeafFrom(prog: List[m::Ins], pre: m::Regs, oldMem: List[m::Cell], kc: int, t: int, tk: str, lk: str): m::GenOut =
    val base = pre.len
    val g0: m::Gen = { regs: pre, log: oldMem, slots: [], stop: false, pubPrev: false, page: Set(), alu: Set(),
                       xPage: Set(), xAlu: Set(), dup: [], garb: Set(), applied: false, why: "S" }
    val g = range(0, S).foldl(g0, (acc, j) => m::genSlot(prog, base, kc, t, tk, lk, acc, j))
    val finalLen = g.regs.len
    val reqs = m::enabledBus(g.slots)
    val nRows = if (MUTANT == "mem_unbounded" and finalLen - base > N_MEM) finalLen - base else N_MEM
    val mem = range(0, nRows).foldl([], (l, k) =>
      l.append(m::memRowOf(base, finalLen, g.log, reqs, g.dup, g.garb, tk == "orphan_mem", k)))
    val w0: m::W = {
      prog: prog, oldMem: oldMem, pre: pre, slots: g.slots,
      page: g.page.union(g.xPage), alu: g.alu.union(g.xAlu), mem: mem, post: g.regs,
      memOut: m::rootOf(oldMem, mem, finalLen, base),
      steps: g.slots.foldl(0, (n, sl) => if (sl.en) n + 1 else n),
    }
    { w: w0, applied: g.applied or tk == "orphan_mem", why: g.why }

  pure def chainAccepts(a: m::W, b: m::W): bool = and {
    m::accepts(a), m::accepts(b), b.prog == a.prog, b.pre == a.post, b.oldMem == a.memOut,
  }

  // ---------------------------------------------------------------- frame-stack refinement
  pure def ruleOf(sl: m::Slot): int =
    val decs = sl.bus.select(r => r.chip == m::T_DEC)
    if (decs.length() > 0) decs[0].a else 0
  pure def popsImpl(w0: m::W): List[int] =
    w0.slots.foldl([], (l, sl) => if (sl.en and ruleOf(sl) == m::R_POP) l.append(m::reg(sl.post, m::at(sl.bus, 0).b)) else l)
  pure def refPops(prog: List[m::Ins], r0: m::RefSt, k: int): List[int] =
    range(0, k).foldl({ s: r0, l: [] }, (acc, _i) =>
      val d = acc.s.stack.length()
      val popping = acc.s.status == m::RUN and m::progAt(prog, acc.s.pc).op == m::OP_POP and d > 0
      { s: m::refStep(prog, acc.s), l: if (popping) acc.l.append(acc.s.stack[d - 1].v) else acc.l }).l
  pure def rev(l: List[int]): List[int] = l.foldl([], (acc, x) => [x].concat(acc))
  // uncached: the chain from fp holds every frame, the top first
  pure def drainImpl(w0: m::W): List[int] = rev(m::implStack(w0.memOut, w0.post.fp, w0.post.depth))
  pure def refDrain(s: m::RefSt): List[int] = rev(m::refStackVals(s))
  pure def popsExecOk(a: m::W, b: m::W, r0: m::RefSt): bool =
    popsImpl(a).concat(popsImpl(b)) == refPops(a.prog, r0, a.steps + b.steps)
  pure def popsOk(a: m::W, b: m::W, r0: m::RefSt): bool =
    popsImpl(a).concat(popsImpl(b)).concat(drainImpl(b))
      == refPops(a.prog, r0, a.steps + b.steps).concat(refDrain(m::refRun(a.prog, r0, a.steps + b.steps)))

  // ---------------------------------------------------------------- state
  var w: m::W
  var w2: m::W
  var refPre: m::RefSt
  var acc: bool
  var chainAcc: bool
  var tamper: str
  var tamper2: str

  action sample = {
    nondet i0 = m::FIRST_INSTRS.oneOf()
    nondet i1 = m::INSTRS.oneOf()
    nondet i2 = m::INSTRS.oneOf()
    nondet i3 = m::INSTRS.oneOf()
    nondet i4 = m::INSTRS.oneOf()
    nondet i5 = m::INSTRS.oneOf()
    nondet i6 = m::INSTRS.oneOf()
    nondet gw = 0.to(4).oneOf()
    nondet kc = 1.to(S).oneOf()
    nondet t = 0.to(S - 1).oneOf()
    nondet tk = m::TAMPERS.oneOf()
    nondet lk = m::LIES.oneOf()
    nondet kc2 = 1.to(S).oneOf()
    nondet t2 = 0.to(S - 1).oneOf()
    nondet tk2 = m::TAMPERS.oneOf()
    nondet lk2 = m::LIES.oneOf()
    val prog = [i0, i1, i2, i3, i4, i5, i6, m::HALT_INS]
    val r0 = m::warm(prog, gw)
    val out = m::genLeaf(prog, r0, kc, t, tk, lk)
    val honest = m::genLeaf(prog, r0, kc, t, "none", lk)
    val out2 = genLeafFrom(prog, out.w.post, out.w.memOut, kc2, t2, tk2, lk2)
    val honest2 = genLeafFrom(prog, out.w.post, out.w.memOut, kc2, t2, "none", lk2)
    all {
      w' = out.w,
      w2' = out2.w,
      refPre' = r0,
      acc' = m::accepts(out.w),
      chainAcc' = chainAccepts(out.w, out2.w),
      tamper' = if (out.w == honest.w) "none" else tk,
      tamper2' = if (out2.w == honest2.w) "none" else tk2,
    }
  }
  action init = sample
  action step = sample

  // ---------------------------------------------------------------- invariants
  val invPops: bool = chainAcc implies popsOk(w, w2, refPre)
  val invPopsExec: bool = chainAcc implies popsExecOk(w, w2, refPre)
  val refMid: m::RefSt = m::refRun(w.prog, refPre, w.steps)
  val invChainRefine: bool = chainAcc implies and { m::refineOk(w2, refMid), m::framesOk(w2, refMid) }
  val invChainTamper: bool = (chainAcc and MUTANT == "none") implies m::HARMLESS.contains(tamper2)
  val invChainComplete: bool =
    (MUTANT == "none" and tamper == "none" and tamper2 == "none" and m::steppable(w.post.status)) implies chainAcc
  val invChain: bool = and { invPops, invChainRefine, invChainTamper, invChainComplete }

  val reachPopAcross: bool = chainAcc and w2.slots.indices().exists(j =>
    m::ruleIn(w2.slots[j], m::R_POP) and m::toSet(w2.slots[j].bus).exists(r =>
      r.chip == m::T_CELL and r.key < w2.pre.len and r.key >= w.pre.len))
  val reachTwoPopsLeaf2: bool = chainAcc and popsImpl(w2).length() >= 2
  val reachChainTamperRefused: bool = acc and not(chainAcc) and tamper2 != "none"
}

module chainBase { import chipLeafChain(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "none").* }
module chain_rewrite_no_move { import chipLeafChain(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "rewrite_no_move").* }
module chain_push_link_free { import chipLeafChain(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "push_link_free").* }
