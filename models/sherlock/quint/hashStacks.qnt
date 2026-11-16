// -*- mode: Bluespec; -*-
//
// X.3 (hash-stack part of X3): kit STACKS, the cached-top hash-chain stacks FRAMES and TASKS
// with the compound step (`openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-chip-stacks`,
// STK-001 to STK-004; `research/sherlock-final-design-20261111/ARCHITECTURE.md`, kit STACKS).
//
// Three parts are kept apart:
//   1. The reference list function `refStep` (Moriarty
//      `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`, the image update:
//      "Task rewrite of the pre-image top, before any push/pop" with `last_mut()`, then
//      `TaskOp::Push` / `TaskOp::Pop` with `Vec::pop`; about lines 3609-3621 today).
//      A rewrite or a pop of an empty list changes nothing. NOTE: the validated owner step
//      refuses a rewrite or a pop on an empty stack ("frame existence" / "task existence",
//      about lines 2957-2965). That is a guard of CORE and DEC; STK-002 keeps the list no-op in
//      STACKS, and this model follows STK-002.
//   2. The circuit acceptance predicate of one STACKS row, as the AIR of the kit
//      (`crates/sherlock-leaf/src/stacks.rs`, branch `kit-stacks`) states it, over a witness
//      that the prover chooses: post cells, the hash enable `hs`, and the hashed preimage `lo`.
//      Each named mutant drops exactly one conjunct (the same conjunct as the Rust weak bit).
//   3. The abstraction `absCached(count, top, below)`: `[]` at count 0, else
//      `chainD^-1(below) ++ [top]` where `chainD([]) = EMPTY`, `chainD(l ++ [x]) =
//      H(dom, |l| + 1, chainD(l), x)` (list order bottom..top, as the Rust `Vec`).
//
// The hash is a random oracle: `orc` maps a preimage `{dom, d, b, x}` to a fresh positive id, so
// it is injective and its range excludes ZERO (0), EMPTY_F (-1) and EMPTY_T (-2). The prover can
// only get digests from the oracle.
//
// Two modes (const ADV):
//   ADV = false: both stacks start at genesis (empty); CORE is honest (it supplies any request
//     rw, rw_top, op, new_top), the STACKS prover is adversarial within the AIR. Checks the
//     refinement from genesis.
//   ADV = true: the pre cells of each stack are any cells (CORE or a forged boundary gives a
//     malformed pre), and the oracle holds seeded entries, among them a wrong-depth entry and a
//     task-domain entry. Checks the port certificate of one accepted row: the shape rule on the
//     pre and post cells, and that a restore returns the spilled preimage.
//
// Count: u64 abstracted to CMOD = 4 values (0..3); the no-wrap check refuses count + 1 = CMOD.

module hashStacks {
  const MUTANT: str
  const ADV: bool

  // ---------------------------------------------------------------- parameters
  pure val CMOD: int = 4
  pure val ALLC: Set[int] = 0.to(CMOD - 1)
  pure val RECS: Set[int] = Set(1, 2)
  pure val TOPS: Set[int] = Set(0, 1, 2)
  pure val ZERO: int = 0
  pure val EMPTY_F: int = -1
  pure val EMPTY_T: int = -2
  pure val DF: int = 37 // DOM_FRAMES
  pure val DT: int = 38 // DOM_TASKS
  pure val DOMS: Set[int] = Set(DF, DT)
  pure val STACKS: Set[str] = Set("F", "T")

  pure def dom(s: str): int = if (s == "F") DF else DT
  pure def emptyC(s: str): int = if (s == "F") EMPTY_F else EMPTY_T

  type Phys = { c: int, top: int, below: int }
  type Lo = { dom: int, d: int, b: int, x: int }
  type Req = { rw: bool, rwt: int, op: str, nw: int }
  type Wit = { post: Phys, hs: bool, lo: Lo }
  type Dec = { ok: bool, cur: int, l: List[int] }

  pure val NOLO: Lo = { dom: 0, d: 0, b: 0, x: 0 }
  pure def m(name: str): bool = MUTANT == name

  // ---------------------------------------------------------------- state
  var phys: str -> Phys
  var ref: str -> List[int]
  var refDef: str -> bool
  var orc: Lo -> int
  var inv: int -> Lo
  var nextId: int
  // bookkeeping of the last accepted step
  var steps: int
  var lastS: str
  var lastPre: Phys
  var lastRefPre: List[int]
  var lastRefDef: bool
  var lastReq: Req
  var lastWit: Wit
  var lastOther: Phys
  var log: List[{ s: str, rw: bool, op: str, preC: int, hs: bool }]

  // ---------------------------------------------------------------- reference list function
  pure def refStep(l: List[int], q: Req): List[int] = {
    val n = l.length()
    val l1 = if (q.rw and n > 0) l.replaceAt(n - 1, q.rwt) else l
    if (q.op == "push") l1.append(q.nw)
    else if (q.op == "pop" and n > 0) l1.slice(0, n - 1)
    else l1
  }

  // ---------------------------------------------------------------- abstraction
  // Walk count - 1 levels below the cached top: level i has depth count - 1 - i.
  def decode(s: str, p: Phys): Dec =
    if (p.c == 0) { ok: p.top == 0 and p.below == emptyC(s), cur: p.below, l: List() }
    else {
      val w = range(0, CMOD - 1).foldl({ ok: true, cur: p.below, l: List() }, (a, i) => {
        val d = p.c - 1 - i
        if (i >= p.c - 1) a
        else if (not(a.ok) or not(inv.keys().contains(a.cur))) { ...a, ok: false }
        else {
          val e = inv.get(a.cur)
          if (e.dom != dom(s) or e.d != d) { ...a, ok: false }
          else { ok: true, cur: e.b, l: List(e.x).concat(a.l) }
        }
      })
      { ok: w.ok and w.cur == emptyC(s), cur: w.cur, l: w.l.append(p.top) }
    }

  pure def shapeOk(s: str, p: Phys): bool = and {
    p.c == 0 implies (p.top == 0 and p.below == emptyC(s)),
    p.c == 1 implies p.below == emptyC(s),
    ALLC.contains(p.c),
  }

  // ---------------------------------------------------------------- acceptance of one row
  def digs: Set[int] = Set(ZERO, EMPTY_F, EMPTY_T).union(inv.keys())
  def hashOf(lo: Lo): int = if (orc.keys().contains(lo)) orc.get(lo) else nextId

  // The pre-shape gates (STK-002), each a conjunct a mutant may drop.
  def preShape(s: str, p: Phys): bool = and {
    (p.c == 0 and not(m("stack_empty_nonzero_top"))) implies p.top == 0,
    (p.c == 0 and not(m("stack_empty_below_not_empty_const"))) implies p.below == emptyC(s),
    (p.c == 1 and not(m("stack_count_one_below_nonempty"))) implies p.below == emptyC(s),
  }

  // Every witness the AIR accepts for request q on stack s from pre p.
  def cands(s: str, p: Phys, q: Req): Set[Wit] = {
    val ge1 = p.c >= 1
    val ge2 = p.c >= 2
    val eff = if (q.rw and ge1) q.rwt else p.top
    val d = if (q.op == "push") 1 else if (q.op == "pop" and ge1) -1 else 0
    val cH = p.c + d
    // the count adder (limb carries) and the no-wrap check (final carry 0)
    val pcSet =
      if (m("stack_compound_count_plus_two")) ALLC
      else if (m("stack_count_wrap")) Set(((cH % CMOD) + CMOD) % CMOD)
      else if (ALLC.contains(cH)) Set(cH) else Set()
    val hsH = (q.op == "push" and ge1) or (q.op == "pop" and ge2)
    val hsSet = if (m("stack_push_without_spill")) Set(true, false) else Set(hsH)
    val domSet = if (m("stack_frame_task_domain_swapped")) DOMS else Set(dom(s))
    if (not(preShape(s, p))) Set()
    else hsSet.map(hs => {
      if (hs and q.op == "push") {
        // push binds lo = (count, below, eff) and hi = post.below; hash sent
        val xSet = if (m("stack_compound_spills_unrewritten_top")) TOPS else Set(eff)
        val tSet = if (m("stack_compound_child_not_installed")) TOPS else Set(q.nw)
        tuples(pcSet, xSet, domSet, tSet).map(((pc, x, dm, t)) => {
          val lo = { dom: dm, d: p.c, b: p.below, x: x }
          { post: { c: pc, top: t, below: hashOf(lo) }, hs: true, lo: lo }
        })
      } else if (hs and q.op == "pop") {
        // pop binds lo = (post.count, post.below, post.top) and hi = pre.below; the oracle
        // gives lo = the preimage of pre.below
        if (not(inv.keys().contains(p.below))) Set()
        else {
          val lo = inv.get(p.below)
          val bSet = if (m("stack_predecessor_forged")) digs else Set(lo.b)
          val tSet = if (m("stack_restored_record_forged")) TOPS else Set(lo.x)
          val pcs = pcSet.filter(pc => m("stack_count_unchecked") or lo.d == pc)
          if (not(domSet.contains(lo.dom))) Set()
          else tuples(pcs, bSet, tSet).map(((pc, b, t)) =>
            { post: { c: pc, top: t, below: b }, hs: true, lo: lo })
        }
      } else if (hs) {
        // op none with hs = 1 (only when the hs definition is dropped): hash of a free preimage,
        // post.below unbound
        tuples(pcSet, digs).map(((pc, b)) => { post: { c: pc, top: eff, below: b }, hs: true, lo: NOLO })
      } else {
        // no hash: below kept
        val bSet = if (m("stack_rewrite_spills")) digs else Set(p.below)
        val tSet =
          if (q.op == "none") Set(eff)
          else if (q.op == "push") (if (m("stack_compound_child_not_installed")) TOPS else Set(q.nw))
          else if (ge2) TOPS // pop at count >= 2 with no hash: only under the hs mutant
          else Set(0)
        tuples(pcSet, bSet, tSet).map(((pc, b, t)) => { post: { c: pc, top: t, below: b }, hs: false, lo: NOLO })
      }
    }).flatten()
  }

  // ---------------------------------------------------------------- actions
  pure val SEEDS: List[Lo] = List(
    { dom: DF, d: 1, b: EMPTY_F, x: 1 }, // id 1
    { dom: DF, d: 2, b: 1, x: 2 },       // id 2
    { dom: DT, d: 1, b: EMPTY_T, x: 1 }, // id 3 (task domain)
    { dom: DF, d: 3, b: EMPTY_F, x: 2 }, // id 4 (spilled at depth 3, below it EMPTY)
  )
  pure val SEEDIDS: Set[int] = 1.to(4)

  action initWith(pf: Phys, pt: Phys, o: Lo -> int, iv: int -> Lo, nid: int): bool = {
    val df = { ok: true, cur: 0, l: List() } // placeholder (recomputed in the invariant)
    all {
      phys' = Map("F" -> pf, "T" -> pt),
      orc' = o,
      inv' = iv,
      nextId' = nid,
      steps' = 0,
      lastS' = "F",
      lastPre' = pf,
      lastRefPre' = List(),
      lastRefDef' = false,
      lastReq' = { rw: false, rwt: 0, op: "none", nw: 0 },
      lastWit' = { post: pf, hs: false, lo: NOLO },
      lastOther' = pt,
      log' = List(),
    }
  }

  // decode with an explicit oracle inverse (init has no state yet)
  pure def decodeWith(iv: int -> Lo, s: str, p: Phys): Dec =
    if (p.c == 0) { ok: p.top == 0 and p.below == emptyC(s), cur: p.below, l: List() }
    else {
      val w = range(0, CMOD - 1).foldl({ ok: true, cur: p.below, l: List() }, (a, i) => {
        val d = p.c - 1 - i
        if (i >= p.c - 1) a
        else if (not(a.ok) or not(iv.keys().contains(a.cur))) { ...a, ok: false }
        else {
          val e = iv.get(a.cur)
          if (e.dom != dom(s) or e.d != d) { ...a, ok: false }
          else { ok: true, cur: e.b, l: List(e.x).concat(a.l) }
        }
      })
      { ok: w.ok and w.cur == emptyC(s), cur: w.cur, l: w.l.append(p.top) }
    }

  action init: bool =
    if (not(ADV)) {
      val pf = { c: 0, top: 0, below: EMPTY_F }
      val pt = { c: 0, top: 0, below: EMPTY_T }
      all {
        initWith(pf, pt, Map(), Map(), 1),
        ref' = Map("F" -> List(), "T" -> List()),
        refDef' = Map("F" -> true, "T" -> true),
      }
    } else {
      val iv = SEEDIDS.mapBy(i => SEEDS[i - 1])
      val o = SEEDIDS.map(i => (SEEDS[i - 1], i)).setToMap()
      val belows = Set(ZERO, EMPTY_F, EMPTY_T).union(SEEDIDS)
      nondet cf = ALLC.oneOf()
      nondet tf = TOPS.oneOf()
      nondet bf = belows.oneOf()
      nondet ct = ALLC.oneOf()
      nondet tt = TOPS.oneOf()
      nondet bt = belows.oneOf()
      val pf = { c: cf, top: tf, below: bf }
      val pt = { c: ct, top: tt, below: bt }
      val df = decodeWith(iv, "F", pf)
      val dt = decodeWith(iv, "T", pt)
      all {
        initWith(pf, pt, o, iv, 5),
        ref' = Map("F" -> df.l, "T" -> dt.l),
        refDef' = Map("F" -> df.ok, "T" -> dt.ok),
      }
    }

  action stepOn(s: str, q: Req, w: Wit): bool = {
    val o = if (s == "F") "T" else "F"
    val newEntry = w.hs and q.op == "push" and not(orc.keys().contains(w.lo))
    val pre = phys.get(s)
    all {
      phys' = phys.put(s, w.post),
      ref' = ref.put(s, refStep(ref.get(s), q)),
      refDef' = refDef,
      orc' = if (newEntry) orc.put(w.lo, nextId) else orc,
      inv' = if (newEntry) inv.put(nextId, w.lo) else inv,
      nextId' = if (newEntry) nextId + 1 else nextId,
      steps' = steps + 1,
      lastS' = s,
      lastPre' = pre,
      lastRefPre' = ref.get(s),
      lastRefDef' = refDef.get(s),
      lastReq' = q,
      lastWit' = w,
      lastOther' = phys.get(o),
      log' = log.append({ s: s, rw: q.rw, op: q.op, preC: pre.c, hs: w.hs }),
    }
  }

  action step: bool = {
    nondet s = STACKS.oneOf()
    nondet rw = Set(true, false).oneOf()
    nondet rwt = RECS.oneOf()
    nondet op = Set("none", "push", "pop").oneOf()
    nondet nw = RECS.oneOf()
    val q = { rw: rw, rwt: rwt, op: op, nw: nw }
    val cs = cands(s, phys.get(s), q)
    all {
      cs != Set(),
      nondet w = cs.oneOf()
      stepOn(s, q, w),
    }
  }

  // ---------------------------------------------------------------- invariants
  // The cached top refines the list of the reference (STK-003): wherever the abstract list is
  // defined, the physical stack decodes to the reference list.
  val cachedTopRefinesList: bool = STACKS.forall(s =>
    refDef.get(s) implies {
      val dd = decode(s, phys.get(s))
      dd.ok and dd.l == ref.get(s)
    })

  // One accepted row is ONE application of the compound event (optional rewrite of the top,
  // then none, push or pop) to the pre list, and leaves the other stack unchanged (STK-001).
  val compoundStepIsOneEvent: bool =
    steps == 0 or and {
      lastRefDef implies {
        val dd = decode(lastS, phys.get(lastS))
        dd.ok and dd.l == refStep(lastRefPre, lastReq)
      },
      phys.get(if (lastS == "F") "T" else "F") == lastOther,
    }

  // Count 0: zero top and below = EMPTY; count 1: below = EMPTY (STK-002). One direction only:
  // below = EMPTY does not imply count <= 1 by a gate. Checked on the pre and the post of the
  // last accepted row, and on every stack in genesis mode.
  val belowEmptyAtCountLe1: bool = and {
    steps == 0 or (shapeOk(lastS, lastPre) and shapeOk(lastS, phys.get(lastS))),
    not(ADV) implies STACKS.forall(s => shapeOk(s, phys.get(s))),
  }

  // A restore returns exactly the preimage spilled at that depth in that domain (STK-003, STK-004).
  val restoreReturnsSpilled: bool =
    (steps > 0 and lastReq.op == "pop" and lastWit.hs) implies {
      val p = phys.get(lastS)
      inv.keys().contains(lastPre.below)
        and inv.get(lastPre.below) == { dom: dom(lastS), d: p.c, b: p.below, x: p.top }
    }

  val allInvariants: bool = and {
    cachedTopRefinesList, compoundStepIsOneEvent, belowEmptyAtCountLe1, restoreReturnsSpilled,
  }

  // ---------------------------------------------------------------- reachability witnesses
  // Each is the negation of a scenario; `quint run` must report a violation (exit 1).
  def lastIs(rw: bool, op: str, pc: int): bool =
    steps > 0 and lastReq.rw == rw and lastReq.op == op and lastPre.c == pc
  val reachRwPushAt1: bool = not(lastIs(true, "push", 1))
  val reachRwPushAt2: bool = not(lastIs(true, "push", 2))
  val reachRwPopAt2: bool = not(lastIs(true, "pop", 2))
  val reachRwAt0: bool = not(lastIs(true, "none", 0))
  val reachPopAt0: bool = not(lastIs(false, "pop", 0))
  // 0 -> 1 -> 0 on one stack with no hash in either step
  val reach010: bool = not(0.to(log.length() - 2).exists(i => and {
    log[i].s == log[i + 1].s, log[i].op == "push", log[i].preC == 0, not(log[i].hs),
    log[i + 1].op == "pop", log[i + 1].preC == 1, not(log[i + 1].hs),
  }))
  // the child's pop restores the parent carrying the rewritten cursor: rw+push at count >= 1,
  // then a pop of the same stack in the next step with a hash
  val reachRestoreRewrittenParent: bool = not(0.to(log.length() - 2).exists(i => and {
    log[i].s == log[i + 1].s, log[i].rw, log[i].op == "push", log[i].preC >= 1,
    log[i + 1].op == "pop", log[i + 1].hs,
  }))
}

// ---------------------------------------------------------------- instances
module hashStacks_sound { import hashStacks(MUTANT = "none", ADV = false).* }
module hashStacks_sound_adv { import hashStacks(MUTANT = "none", ADV = true).* }

module m01_spills_unrewritten_top { import hashStacks(MUTANT = "stack_compound_spills_unrewritten_top", ADV = false).* }
module m02_child_not_installed { import hashStacks(MUTANT = "stack_compound_child_not_installed", ADV = false).* }
module m03_count_plus_two { import hashStacks(MUTANT = "stack_compound_count_plus_two", ADV = false).* }
module m04_count_one_below_nonempty { import hashStacks(MUTANT = "stack_count_one_below_nonempty", ADV = true).* }
module m05_empty_nonzero_top { import hashStacks(MUTANT = "stack_empty_nonzero_top", ADV = true).* }
module m06_empty_below_not_empty_const { import hashStacks(MUTANT = "stack_empty_below_not_empty_const", ADV = true).* }
module m07_predecessor_forged { import hashStacks(MUTANT = "stack_predecessor_forged", ADV = false).* }
module m08_restored_record_forged { import hashStacks(MUTANT = "stack_restored_record_forged", ADV = false).* }
module m09_count_unchecked { import hashStacks(MUTANT = "stack_count_unchecked", ADV = true).* }
module m09g_count_unchecked_genesis { import hashStacks(MUTANT = "stack_count_unchecked", ADV = false).* }
module m10_domain_swapped { import hashStacks(MUTANT = "stack_frame_task_domain_swapped", ADV = false).* }
module m11_rewrite_spills { import hashStacks(MUTANT = "stack_rewrite_spills", ADV = false).* }
module m12_push_without_spill { import hashStacks(MUTANT = "stack_push_without_spill", ADV = false).* }
module m13_count_wrap { import hashStacks(MUTANT = "stack_count_wrap", ADV = true).* }
module m13g_count_wrap_genesis { import hashStacks(MUTANT = "stack_count_wrap", ADV = false).* }
