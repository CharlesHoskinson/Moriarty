// -*- mode: Bluespec; -*-
//
// Quint model of the MEMORY BACKEND dense kit as built for the STARK leaf (tasks 1.3 and 2.4,
// `crates/sherlock-leaf/src/heap.rs` in the openvm fork, branch `kit-membackend`). It covers the
// two parts that `denseMemory.qnt` (the C1 Halo2 chip) does not:
//
//   1. `HeapReadAir`: the cold read walk against `pre.root`. The record leaf is the
//      capacity-tagged digest `H(DOM_REC, 0; kind, val)`; the node of height h is
//      `H(DOM_NODE, h; left, right)`; the bits of the walk are boolean and their limb
//      accumulator equals the address; the walk ends at the pre root of the token.
//   2. `SegAir` and `FlushAir`: the tail open (always, also with no append), the canonical pad
//      of the post tail, the flush flag `f` iff `r_pre + appends >= SEGC` (FLUSH_IFF), the old
//      walk from `EMPTY_SEG` at slot `q_pre` to the pre root, and the new walk with the same
//      siblings (SIB_SHARED) to the post root.
//
// Small parameters: SEGC = 2 records per segment (the kit uses 8), TOP = 2 (4 slots), W = 1.
// The hash is a keyed polynomial mod the Mersenne prime 2^31 - 1 (the simulator has 64-bit integers); the
// model reaches a few thousand digests, so a collision is unlikely, and a sound-instance
// violation would show one. Every mutant drops one constraint of the AIR, under the name of the
// kit mutant and its weak bit in `heap.rs`.
//
// Not modelled here: `walk_bit_nonboolean` and `domain_tag_swapped_node_height`. With a
// collision-free hash and an honest pre root they have no semantic counterexample in this model;
// their soundness is Lean O16 (`MemContract.walk_sound`: boolean bits of the address and the
// height-tagged node), and the kit gate proves both mutants against an attacker-chosen root.

module denseStark {
  const MUTANT: str

  pure val SEGC: int = 2
  pure val SEGB: int = 1
  pure val TOP: int = 2
  pure val SLOTS: int = 4
  pure val LEVELS: int = SEGB + TOP
  pure val MAXLEN: int = SEGC * SLOTS

  pure val DOM_REC: int = 1
  pure val DOM_PAD: int = 2
  pure val DOM_EMPTY: int = 3
  pure val DOM_NODE: int = 4

  pure val HP: int = 2147483647

  type MRec = { kind: int, v: int }

  pure val KINDS: Set[int] = Set(1, 2)
  pure val VALS: Set[int] = Set(0, 1)
  pure val RECS: Set[MRec] = Set({ kind: 1, v: 0 }, { kind: 1, v: 1 }, { kind: 2, v: 0 }, { kind: 2, v: 1 })

  pure def modP(x: int): int = {
    val r = x % HP
    if (r < 0) r + HP else r
  }

  // The capacity word carries the domain and sub-domain (MBK-TAGCAP); two rate words.
  pure def H(dom: int, sub: int, a: int, b: int): int = {
    val c = dom * 256 + sub + 1
    val s0 = modP(c * 1000003 + modP(a) * 69069 + 12345)
    val s1 = modP(s0 * 1103515245 + modP(b) + 362436069)
    val s2 = modP(s1 * s1 + 521288629)
    modP(s2 * 48271 + modP(s0 * s1) + c)
  }

  pure def recD(r: MRec): int = H(DOM_REC, 0, r.kind, r.v)
  pure val PAD: int = H(DOM_PAD, 0, 0, 0)
  pure val EMPTY: int = H(DOM_EMPTY, 0, 0, 0)
  pure def node(h: int, x: int, y: int): int = H(DOM_NODE, h, x, y)

  pure def seq(lo: int, hi: int): List[int] = range(lo, hi)

  // One level up: pairs (2i, 2i+1) at height h.
  pure def up(xs: List[int], h: int): List[int] =
    range(0, xs.length() / 2).foldl(List(), (acc, i) => acc.append(node(h, xs[2 * i], xs[2 * i + 1])))

  // The tree of `xs` (length 2^k) whose leaves sit at height h0; returns the root.
  pure def treeFrom(xs: List[int], h0: int, k: int): int =
    range(0, k).foldl(xs, (cur, l) => up(cur, h0 + l + 1))[0]

  // The levels of the tree: `lv[0] = xs`, `lv[l]` at height h0 + l.
  pure def levelsFrom(xs: List[int], h0: int, k: int): List[List[int]] =
    range(0, k).foldl(List(xs), (acc, l) => acc.append(up(acc[l], h0 + l + 1)))

  // The segment digest: the tree over SEGC record or pad digests (height 0 .. SEGB).
  pure def segD(ds: List[int]): int = treeFrom(ds, 0, SEGB)

  pure def padTo(ds: List[int]): List[int] =
    range(0, SEGC).foldl(List(), (acc, i) => acc.append(if (i < ds.length()) ds[i] else PAD))

  pure def recsD(rs: List[MRec]): List[int] = rs.foldl(List(), (acc, r) => acc.append(recD(r)))

  // The pre state: `recs` with q = len / SEGC flushed segments, the tail, and an optional extra
  // occupant digest at a slot >= q (`Dense.extra` of the kit; not reachable by honest leaves).
  type St = { recs: List[MRec], extraSlot: int, extraD: int }

  pure def qOf(n: int): int = n / SEGC

  pure def slotsOf(s: St): List[int] = {
    val q = qOf(s.recs.length())
    range(0, SLOTS).foldl(List(), (acc, i) =>
      acc.append(
        if (i < q) segD(recsD(s.recs.slice(SEGC * i, SEGC * i + SEGC)))
        else if (i == s.extraSlot) s.extraD
        else EMPTY))
  }

  pure def rootOfSlots(sl: List[int]): int = treeFrom(sl, SEGB, TOP)
  pure def rootOf(s: St): int = rootOfSlots(slotsOf(s))

  pure def tailRecs(s: St): List[MRec] = {
    val n = s.recs.length()
    s.recs.slice(SEGC * qOf(n), n)
  }

  pure def tailOf(s: St): int = segD(padTo(recsD(tailRecs(s))))

  type Tok = { len: int, root: int, tail: int }
  pure def tok(s: St): Tok = { len: s.recs.length(), root: rootOf(s), tail: tailOf(s) }

  pure def sibAt(lv: List[List[int]], l: int, pos: int): int = {
    val p = pos / (2 ^ l)
    lv[l][if (p % 2 == 0) p + 1 else p - 1]
  }

  // A walk from `cur` at height h0 over `bits` (b in {0,1}; the AIR's mux `cur + b (sib - cur)`).
  pure def walk(cur: int, h0: int, bits: List[int], sibs: List[int]): int =
    range(0, bits.length()).foldl(cur, (c, l) => {
      val b = bits[l]
      val s = sibs[l]
      val left = modP(c + b * (s - c))
      val right = modP(s + b * (c - s))
      node(h0 + l + 1, left, right)
    })

  pure def bitsOf(a: int, k: int): List[int] =
    range(0, k).foldl(List(), (acc, l) => acc.append((a / (2 ^ l)) % 2))

  // ------------------------------------------------------------------ cold read (HeapReadAir)

  type Read = {
    addr: int, claimed: MRec,     // the message on the `cold_read` port
    hrec: MRec,                   // the record the AIR hashes
    bits: List[int], sibs: List[int], root: int,
  }

  // The full tree over every leaf digest (flushed records; other slots have no leaves).
  pure def leafLevels(s: St): List[List[int]] = {
    val q = qOf(s.recs.length())
    val leaves = range(0, MAXLEN).foldl(List(), (acc, a) =>
      acc.append(if (a < SEGC * q) recD(s.recs[a]) else PAD))
    val lv0 = levelsFrom(leaves, 0, SEGB)
    val sl = slotsOf(s)
    val lvt = levelsFrom(sl, SEGB, TOP)
    lv0.slice(0, SEGB).concat(lvt)
  }

  pure def honestRead(s: St, a: int): Read = {
    val lv = leafLevels(s)
    val r = if (a < s.recs.length()) s.recs[a] else { kind: 1, v: 0 }
    { addr: a, claimed: r, hrec: r, bits: bitsOf(a, LEVELS),
      sibs: range(0, LEVELS).foldl(List(), (acc, l) => acc.append(sibAt(lv, l, a))), root: rootOf(s) }
  }

  pure def accSum(bits: List[int]): int =
    range(0, bits.length()).foldl(0, (acc, l) => acc + bits[l] * (2 ^ l))

  pure def readAccepts(mt: str, pre: St, post: St, rd: Read): bool = and {
    rd.bits.indices().forall(i => rd.bits[i] == 0 or rd.bits[i] == 1),
    accSum(rd.bits) == rd.addr,                                           // limb accumulator
    // CORE's route (kit CORE, not this AIR): a cold read only below base = SEGC * q_pre.
    mt == "cold_route_forged" or rd.addr < SEGC * qOf(pre.recs.length()),
    mt == "cold_value_forged" or rd.hrec.v == rd.claimed.v,               // HEAP_REC_BOUND
    mt == "wrong_kind_cold" or rd.hrec.kind == rd.claimed.kind,           // HEAP_KIND_BOUND
    mt == "cold_read_root_unbound" or walk(recD(rd.hrec), 0, rd.bits, rd.sibs) == rd.root,
    if (mt == "cold_read_uses_running_root") Set(rootOf(pre), rootOf(post)).contains(rd.root)
    else rd.root == rootOf(pre),                                          // HEAP_ROOT (pre)
  }

  // ------------------------------------------------------------------ persist (SegAir, FlushAir)

  type Leaf = {
    imported: List[MRec],         // the tail records the window imports (CORE reads them warm)
    apps: List[MRec],
    f: int,
    postPad: int,                // the digest in the free slots of the post tail
    x0: int,                     // the old walk's start digest at slot q
    sibOld: List[int], sibNew: List[int],
  }

  // The window cells: imported tail ++ appends; the segment and the post tail by `f`.
  pure def cells(lf: Leaf): List[MRec] = lf.imported.concat(lf.apps)

  pure def leafPost(pre: St, lf: Leaf): (Tok, List[int]) = {
    val c = cells(lf)
    val q = qOf(pre.recs.length())
    val sl = slotsOf(pre)
    val seg = segD(padTo(recsD(c.slice(0, if (c.length() < SEGC) c.length() else SEGC))))
    val rest = if (lf.f == 1) c.slice(if (c.length() < SEGC) c.length() else SEGC, c.length()) else c
    val restCut = rest.slice(0, if (rest.length() < SEGC) rest.length() else SEGC)
    val tds = recsD(restCut)
    val tailPost = segD(range(0, SEGC).foldl(List(), (acc, i) => acc.append(if (i < tds.length()) tds[i] else lf.postPad)))
    val newRoot = if (lf.f == 1) walk(seg, SEGB, bitsOf(q, TOP), lf.sibNew) else rootOf(pre)
    val newSlots = if (lf.f == 1) sl.replaceAt(q, seg) else sl
    ({ len: pre.recs.length() + lf.apps.length(), root: newRoot, tail: tailPost }, newSlots)
  }

  pure def leafAccepts(mt: str, pre: St, lf: Leaf): bool = {
    val n = pre.recs.length()
    val r = n - SEGC * qOf(n)
    val q = qOf(n)
    val c = cells(lf)
    val openOk = segD(padTo(recsD(lf.imported))) == tailOf(pre)
    and {
      lf.imported.length() == r,
      r + lf.apps.length() <= 2 * SEGC - 1,                                         // MEM-006, W = 1
      n + lf.apps.length() <= MAXLEN - 1,
      q + lf.f <= SLOTS,
      // TAIL_OPEN: always (also with no append).
      if (mt == "forged_tail_across_leaf_boundary") true
      else if (mt == "tail_open_skipped_when_no_append") lf.apps.length() == 0 or openOk
      else openOk,
      // PAD_MASK: the free slots of the post tail hold PAD.
      mt == "non_canonical_tail" or lf.postPad == PAD,
      // FLUSH_IFF: f = 1 iff r + appends >= SEGC.
      if (mt == "flush_skipped") lf.f == 0 or c.length() >= SEGC
      else if (mt == "flush_spurious") lf.f == 1 or c.length() < SEGC
      else (lf.f == 1) == (c.length() >= SEGC),
      lf.f == 0 or and {
        // EMPTY_SEG: the old walk starts at EMPTY.
        mt == "empty_seg_forgery" or lf.x0 == EMPTY,
        // HEAP_ROOT: the old walk reaches the pre root.
        mt == "address_reuse_occupied_slot" or walk(lf.x0, SEGB, bitsOf(q, TOP), lf.sibOld) == rootOf(pre),
        // SIB_SHARED: the new walk uses the old walk's siblings.
        mt == "walk_sibling_differs_old_new" or lf.sibNew == lf.sibOld,
      },
    }
  }

  // ------------------------------------------------------------------ state and sampling

  var pre: St
  var lf: Leaf
  var rd: Read
  var leafOk: bool
  var readOk: bool

  pure def canonPost(pre: St, lf: Leaf): St = { recs: pre.recs.concat(lf.apps), extraSlot: pre.extraSlot, extraD: pre.extraD }

  pure val GARBAGE: int = H(9, 9, 9, 9)

  action sample = {
    nondet n = 0.to(7 - 1).oneOf()
    nondet m = 0.to(4 - 1).oneOf()
    nondet r0 = RECS.oneOf()
    nondet r1 = RECS.oneOf()
    nondet r2 = RECS.oneOf()
    nondet r3 = RECS.oneOf()
    nondet r4 = RECS.oneOf()
    nondet r5 = RECS.oneOf()
    nondet a0 = RECS.oneOf()
    nondet a1 = RECS.oneOf()
    nondet a2 = RECS.oneOf()
    nondet useExtra = Set(false, true).oneOf()
    nondet tamperImp = Set(false, true).oneOf()
    nondet ti = RECS.oneOf()
    nondet f = Set(0, 1).oneOf()
    nondet padSel = Set(0, 1).oneOf()
    nondet x0Sel = Set(0, 1).oneOf()
    nondet sibTamper = 0.to(TOP + 1 - 1).oneOf()
    nondet oldTamper = Set(false, true).oneOf()
    nondet ra = 0.to(MAXLEN - 1).oneOf()
    nondet rk = KINDS.oneOf()
    nondet rv = VALS.oneOf()
    nondet hk = KINDS.oneOf()
    nondet hv = VALS.oneOf()
    nondet rootSel = Set(0, 1, 2).oneOf()
    val pool = List(r0, r1, r2, r3, r4, r5)
    val p = mkPre(pool.slice(0, n), useExtra, segD(List(recD(a0), recD(a1))))
    val leaf = mkLeaf(p, slotsOf(p), List(a0, a1, a2).slice(0, m), tamperImp, ti, f, padSel, x0Sel, sibTamper, oldTamper)
    val rdx = mkRead(p, canonPost(p, leaf), honestRead(p, ra), honestRead(canonPost(p, leaf), ra), { kind: rk, v: rv }, { kind: hk, v: hv }, rootSel)
    setState(p, leaf, rdx)
  }

  pure def mkPre(recs: List[MRec], useExtra: bool, occ: int): St =
    { recs: recs, extraSlot: if (useExtra) qOf(recs.length()) else SLOTS, extraD: occ }

  pure def mkSibs(sl: List[int], q: int): List[int] = {
    val lv = levelsFrom(sl, SEGB, TOP)
    range(0, TOP).foldl(List(), (acc, l) => acc.append(sibAt(lv, l, q)))
  }

  pure def mkLeaf2(p: St, sl: List[int], sOld: List[int], apps: List[MRec], tamperImp: bool, ti: MRec, f: int,
                   padSel: int, x0Sel: int, sibTamper: int, oldTamper: bool): Leaf = {
    val q = qOf(p.recs.length())
    val tr = tailRecs(p)
    { imported: if (tamperImp and tr.length() > 0) tr.replaceAt(0, ti) else tr,
      apps: apps, f: f, postPad: if (padSel == 0) PAD else GARBAGE,
      x0: if (x0Sel == 0) EMPTY else sl[if (q < SLOTS) q else 0],
      sibOld: if (oldTamper) sOld.replaceAt(0, GARBAGE) else sOld,
      sibNew: if (sibTamper < TOP) sOld.replaceAt(sibTamper, GARBAGE) else sOld }
  }

  pure def mkLeaf(p: St, sl: List[int], apps: List[MRec], tamperImp: bool, ti: MRec, f: int,
                  padSel: int, x0Sel: int, sibTamper: int, oldTamper: bool): Leaf =
    mkLeaf2(p, sl, mkSibs(sl, qOf(p.recs.length())), apps, tamperImp, ti, f, padSel, x0Sel, sibTamper, oldTamper)

  // rootSel 1 takes the siblings of the post tree (a read against the running root).
  pure def mkRead(p: St, post: St, h0: Read, h1: Read, claimed: MRec, hrec: MRec, rootSel: int): Read = {
    val h = if (rootSel == 1) h1 else h0
    { ...h, claimed: claimed, hrec: hrec,
      root: if (rootSel == 0) rootOf(p) else if (rootSel == 1) rootOf(post)
            else walk(recD(hrec), 0, h.bits, h.sibs) }
  }

  action setState(p: St, leaf: Leaf, rdx: Read): bool = all {
    pre' = p,
    lf' = leaf,
    rd' = rdx,
    leafOk' = leafAccepts(MUTANT, p, leaf),
    readOk' = readAccepts(MUTANT, p, canonPost(p, leaf), rdx),
  }

  action init = sample
  action step = sample

  // ------------------------------------------------------------------ properties

  // The accepted leaf's post token and slots are the canonical state of pre ++ apps, and the
  // flush never overwrites an occupied slot.
  pure def leafSound(p: St, l: Leaf): bool = {
    val (t, sl) = leafPost(p, l)
    val cp = canonPost(p, l)
    val q = qOf(p.recs.length())
    and {
      l.imported == tailRecs(p),
      t == tok(cp),
      l.f == 0 or slotsOf(p)[q] == EMPTY,
    }
  }

  val invLeaf: bool = leafOk implies leafSound(pre, lf)

  // An accepted cold read opens a flushed record of the pre state, value and kind.
  val invColdRead: bool =
    readOk implies (if (rd.addr < SEGC * qOf(pre.recs.length())) rd.claimed == pre.recs[rd.addr] else false)

  val inv: bool = invLeaf and invColdRead

  // Non-vacuity (each must be violated on the sound instance).
  val reachFlush: bool = not(leafOk and lf.f == 1 and pre.recs.length() >= SEGC)
  val reachNoAppendLeaf: bool = not(leafOk and lf.apps.length() == 0 and lf.imported.length() > 0)
  val reachColdRead: bool = not(readOk and rd.addr >= SEGC)
}

module denseStark_sound { import denseStark(MUTANT = "none").* }
module denseStark_cold_value_forged { import denseStark(MUTANT = "cold_value_forged").* }
module denseStark_wrong_kind_cold { import denseStark(MUTANT = "wrong_kind_cold").* }
module denseStark_cold_read_root_unbound { import denseStark(MUTANT = "cold_read_root_unbound").* }
module denseStark_cold_read_uses_running_root { import denseStark(MUTANT = "cold_read_uses_running_root").* }
module denseStark_cold_route_forged { import denseStark(MUTANT = "cold_route_forged").* }
module denseStark_forged_tail_across_leaf_boundary { import denseStark(MUTANT = "forged_tail_across_leaf_boundary").* }
module denseStark_tail_open_skipped_when_no_append { import denseStark(MUTANT = "tail_open_skipped_when_no_append").* }
module denseStark_non_canonical_tail { import denseStark(MUTANT = "non_canonical_tail").* }
module denseStark_flush_skipped { import denseStark(MUTANT = "flush_skipped").* }
module denseStark_flush_spurious { import denseStark(MUTANT = "flush_spurious").* }
module denseStark_address_reuse_occupied_slot { import denseStark(MUTANT = "address_reuse_occupied_slot").* }
module denseStark_empty_seg_forgery { import denseStark(MUTANT = "empty_seg_forgery").* }
module denseStark_walk_sibling_differs_old_new { import denseStark(MUTANT = "walk_sibling_differs_old_new").* }
