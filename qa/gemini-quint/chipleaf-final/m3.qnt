// Soundness Review Model — Lane m3: M3 Memory Commitment, Merkle Walker, and State Digest (As Built)
// Source under review:
//   - crates/mori-chip/src/mem.rs (lines 96-242: SegMemConfig, mem row / chain / first / last)
//   - crates/mori-chip/src/m3.rs (lines 1-448: WalkConfig, m3 walk / first / last, M3W, assign)
//   - crates/mori-chip/src/rfcore.rs (lines 352-431, 509-515, 1199-1243: core bus lanes, LEN, M3 wiring)
//   - crates/mori-chip/src/rf.rs (lines 32-34, 50-53, 92-110: NDIG = 29, FP/TP/LEN/TOP0, genesis len = 8)
//   - crates/mori-chip/src/bus.rs (lines 1-68: BusConfig, CHIP_ALU = 1, CHIP_MEM = 2, lookup_any)
//   - crates/mori-chip/src/vsel.rs (lines 38-77: VSel::Code, lagrange indicator)
//   - crates/mori-chip/src/poseidon.rs (lines 268-416: HIn::Cell copy vs HIn::Val advice)

module m3 {
  const C: int
  const D: int
  const MUTANT: str

  // ---------------------------------------------------------------- Constants
  // Number of core step slots per leaf (scaled to C so a single leaf can have 0, partial, or C appends)
  pure val S: int = C

  // Field modulus Z_PRIME (prime p = 1000003; gcd(5, p - 1) = 1; 2 * 500002 = 1 mod p)
  pure val PRIME: int = 1000003
  pure val HALF: int = 500002

  // Bus chip IDs (bus.rs:29-31)
  pure val CHIP_NONE: int = 0
  pure val CHIP_ALU: int = 1
  pure val CHIP_MEM: int = 2

  // ALU operation codes (rfcore.rs:369-370)
  pure val OP_LT: int = 1
  pure val OP_ADD: int = 2

  // Core rules (rf.rs / rfcore.rs)
  pure val RULE_OFF: int = 0
  pure val RULE_PUSH: int = 1
  pure val RULE_POP: int = 2
  pure val RULE_ARITH: int = 3

  // Top-tree segment indices (0 .. 2^D - 1); segment 0 is reserved (rf.rs:97-99)
  pure val NUM_SEGS: int = 8

  // ---------------------------------------------------------------- Types
  // 3-word memory cell (a, b, c) as stored in SegMemConfig (mem.rs:130)
  type Cell3 = {
    a: int,
    b: int,
    c: int
  }

  // 5-word bus tuple (chip, key, a, b, c) (bus.rs:36)
  type BusTuple = {
    chip: int,
    key: int,
    a: int,
    b: int,
    c: int
  }

  // Scaled core register file:
  //   - Digested words (representing the 29 digested words w_0 .. w_28 in rf.rs:34):
  //     ctlTag, ctlA, ctlB, fcount, limit, fp, len, top0, top1, r0
  //   - Undigested scratch word (representing v0..v2 in rf.rs:33):
  //     v0
  type CoreRegs = {
    ctlTag: int,
    ctlA: int,
    ctlB: int,
    fcount: int,
    limit: int,
    fp: int,
    len: int,
    top0: int,
    top1: int,
    r0: int,
    v0: int
  }

  // One core step slot witness (rfcore.rs:611-618)
  type SlotW = {
    rule: int,
    pre: CoreRegs,
    post: CoreRegs,
    l0: BusTuple,
    l1: BusTuple
  }

  // One row of SegMemConfig (mem.rs:97-99, 207-235):
  //   columns 0..5: bus tuple (chip, key, a, b, c)
  //   column 5: addr
  //   column 6: lenEnd
  //   column 7: act
  type MemRowW = {
    bus: BusTuple,
    addr: int,
    lenEnd: int,
    act: int
  }

  // Record sponge input cells for row i (m3.rs:306-310): [act, a, b, c]
  type RecInW = {
    act: int,
    a: int,
    b: int,
    c: int
  }

  // One level l of a top-tree Merkle walk (m3.rs:34-35, 113-139, 332-414):
  //   columns 2, 3: in0, in1 (HIn::Val free advice fixed by `m3 walk` gate)
  //   column 5: x
  //   column 6: s (sibling)
  //   column 7: b (walk bit)
  //   column 8: accIn
  //   column 9: accOut
  //   column 10: base (used on old walk level 0, code 2)
  //   column 11: lenPost (used on old walk level 0, code 2)
  type WalkRowW = {
    in0: int,
    in1: int,
    x: int,
    s: int,
    b: int,
    accIn: int,
    accOut: int,
    base: int,
    lenPost: int,
    out: int
  }

  // Digest sponge input witness (m3.rs:420-442):
  //   words: the digested register words (with LEN replaced by lenPost on side 1)
  //   root: the mem_root input cell fed to the digest sponge
  //   out: the digest output cell
  type DigestW = {
    words: CoreRegs,
    root: int,
    out: int
  }

  // Complete witness for one leaf (core + SegMemConfig + records + walks + digests + public instance)
  type LeafW = {
    slots: List[SlotW],
    slotSPre: CoreRegs,
    memRows: List[MemRowW],
    recIns: List[RecInW],
    segRoot: int,
    oldWalk: List[WalkRowW],
    newWalk: List[WalkRowW],
    preDig: DigestW,
    postDig: DigestW,
    extraBusReqs: List[BusTuple],
    instPreD: int,
    instPostD: int,
    instSteps: int
  }

  // Two chained leaves witness
  type ChainW = {
    progId: int,
    partId: int,
    leaf1: LeafW,
    leaf2: LeafW
  }

  // Reference execution state
  type MemEntry = {
    addr: int,
    cell: Cell3
  }

  type ReadEvent = {
    addr: int,
    cell: Cell3
  }

  type RefLeafOut = {
    preRegs: CoreRegs,
    postCloseRegs: CoreRegs,
    preRoot: int,
    postRoot: int,
    preD: int,
    postD: int,
    segCells: List[Cell3],
    used: int,
    memAfter: Set[MemEntry],
    treeAfter: int -> int,
    reads: List[ReadEvent],
    readErr: bool
  }

  type RefChainOut = {
    leaf1: RefLeafOut,
    leaf2: RefLeafOut
  }

  // ---------------------------------------------------------------- Field & Hash Helpers
  pure val ZERO_CELL3: Cell3 = { a: 0, b: 0, c: 0 }
  pure val ZERO_TUPLE: BusTuple = { chip: 0, key: 0, a: 0, b: 0, c: 0 }

  pure def fmod(x: int): int = ((x % PRIME) + PRIME) % PRIME
  pure def fadd(x: int, y: int): int = fmod(x + y)
  pure def fsub(x: int, y: int): int = fmod(x - y)
  pure def fmul(x: int, y: int): int = fmod(x * y)

  // Bijective S-box x^5 mod PRIME (gcd(5, PRIME - 1) = 1)
  pure def sbox(x: int): int = {
    val y = fmod(x)
    val y2 = (y * y) % PRIME
    val y4 = (y2 * y2) % PRIME
    (y4 * y) % PRIME
  }

  // Collision-free 2-input compression h2(x, y) in Z_PRIME (m3.rs:56-58)
  pure def h2(x: int, y: int): int = {
    val u = sbox(fadd(x, 101))
    val v = sbox(fadd(y, 203))
    fmod(u + 313 * v + 701 * fmod(x) + 907 * fmod(y) + 17)
  }

  // Record hash H(act, a, b, c) (m3.rs:60-62); in M12 `tag_dropped`, omits `act`
  pure def recordHash(mt: str, act: int, c: Cell3): int = {
    val tag = if (mt == "tag_dropped") 0 else act
    h2(h2(c.a, c.b), h2(tag, c.c))
  }

  // Segment root over C record hashes via binary tree of h2 compressions (m3.rs:64-70)
  pure def segRootFromRecs(recs: List[int]): int = {
    // Supports C = 4 (2 levels) and C = 8 (3 levels)
    if (recs.length() == 8) {
      val l1_0 = h2(recs[0], recs[1])
      val l1_1 = h2(recs[2], recs[3])
      val l1_2 = h2(recs[4], recs[5])
      val l1_3 = h2(recs[6], recs[7])
      h2(h2(l1_0, l1_1), h2(l1_2, l1_3))
    } else {
      val l1_0 = h2(recs[0], recs[1])
      val l1_1 = h2(recs[2], recs[3])
      h2(l1_0, l1_1)
    }
  }

  // All-padding segment root (when all C rows have act = 0 and cell = (0, 0, 0))
  pure def allPaddingSegRoot(mt: str): int = {
    val padRec = recordHash(mt, 0, ZERO_CELL3)
    val emptyRecs: List[int] = []
    val recs = range(0, C).foldl(emptyRecs, (acc, _i) => acc.append(padRec))
    segRootFromRecs(recs)
  }

  // Empty top-tree node at level l (m3.rs:73-75): level 0 is 0, level l+1 is h2(E_l, E_l)
  pure def emptyTop(l: int): int =
    if (l <= 0) 0
    else range(0, l).foldl(0, (e, _k) => h2(e, e))

  // Compute top-tree root (depth D = 3, 8 segments 0..7) from segment map segMap: int -> int
  pure def treeRootFromMap(segMap: int -> int): int = {
    val n0 = h2(segMap.get(0), segMap.get(1))
    val n1 = h2(segMap.get(2), segMap.get(3))
    val n2 = h2(segMap.get(4), segMap.get(5))
    val n3 = h2(segMap.get(6), segMap.get(7))
    val m0 = h2(n0, n1)
    val m1 = h2(n2, n3)
    h2(m0, m1)
  }

  // Siblings along the path of segment index `idx` (0..7) in `segMap` (bottom-up, levels 0..D-1)
  pure def siblingsOfIdx(segMap: int -> int, idx: int): List[int] = {
    val s0 = segMap.get(if (idx % 2 == 0) idx + 1 else idx - 1)
    val n0 = h2(segMap.get(0), segMap.get(1))
    val n1 = h2(segMap.get(2), segMap.get(3))
    val n2 = h2(segMap.get(4), segMap.get(5))
    val n3 = h2(segMap.get(6), segMap.get(7))
    val idx1 = idx / 2
    val s1 =
      if (idx1 == 0) n1
      else if (idx1 == 1) n0
      else if (idx1 == 2) n3
      else n2
    val m0 = h2(n0, n1)
    val m1 = h2(n2, n3)
    val s2 = if (idx / 4 == 0) m1 else m0
    [s0, s1, s2]
  }

  // Bit decomposition of idx into D bits (LSB first)
  pure def bitsOfIdx(idx: int): List[int] =
    [idx % 2, (idx / 2) % 2, (idx / 4) % 2]

  // Native walk from leaf value x with siblings sibs and bits (m3.rs:78-86, 176-187)
  pure def walkField(x0: int, sibs: List[int], bits: List[int]): int =
    range(0, D).foldl(x0, (x, l) => {
      val s = sibs[l]
      val b = bits[l]
      val in0 = fadd(x, fmul(b, fsub(s, x)))
      val in1 = fadd(s, fmul(b, fsub(x, s)))
      h2(in0, in1)
    })

  // State digest over the digested register words (excluding v0) and mem_root (m3.rs:89-93).
  // In M15 `digest_skips_root`, omits `root` (and `r0`).
  pure def stateDigest(mt: str, r: CoreRegs, root: int): int = {
    val h0 = h2(r.ctlTag, r.ctlA)
    val h1 = h2(h0, r.ctlB)
    val h2_ = h2(h1, r.fcount)
    val h3 = h2(h2_, r.limit)
    val h4 = h2(h3, r.fp)
    val h5 = h2(h4, r.len)
    val h6 = h2(h5, r.top0)
    val h7 = h2(h6, r.top1)
    if (mt == "digest_skips_root")
      h7
    else
      h2(h2(h7, r.r0), root)
  }

  // ---------------------------------------------------------------- Initial State & Programs
  pure val EMPTY_TREE_MAP: int -> int =
    0.to(NUM_SEGS - 1).mapBy(_s => 0)

  // Genesis register state (rf.rs:96-110): len starts at C (first segment 0..C-1 reserved, NIL = 0)
  pure def genesisRegs(progId: int, baseLen: int): CoreRegs = {
    ctlTag: RULE_PUSH,
    ctlA: 10,
    ctlB: 1,
    fcount: 0,
    limit: 16,
    fp: 0,
    len: baseLen,
    top0: if (progId == 0) 0 else 7,
    top1: if (progId == 0) 0 else 1,
    r0: 0,
    v0: 0
  }

  // Rule schedules per leaf for each partition partId in {0, 1, 2}:
  //   partId == 0: Leaf 1 runs [PUSH, POP] (used = 1, writes & pops D7 sentinel); Leaf 2 runs [] (used = 0)
  //   partId == 1: Leaf 1 runs C PUSHes (used = C); Leaf 2 runs [PUSH, PUSH, POP] (used = 2, partial + intra-leaf read)
  //   partId == 2: Leaf 1 runs C PUSHes (used = C, full segment); Leaf 2 runs C PUSHes (used = C, full segment)
  pure def leafRuleSchedule(partId: int, leafIdx: int): List[int] = {
    val emptyR: List[int] = []
    if (partId == 0) {
      if (leafIdx == 1) {
        val base2 = [RULE_PUSH, RULE_POP]
        range(0, S).foldl(emptyR, (acc, i) => acc.append(if (i < 2) base2[i] else RULE_OFF))
      } else {
        range(0, S).foldl(emptyR, (acc, _i) => acc.append(RULE_OFF))
      }
    } else if (partId == 1) {
      if (leafIdx == 1) {
        range(0, S).foldl(emptyR, (acc, _i) => acc.append(RULE_PUSH))
      } else {
        val base3 = [RULE_PUSH, RULE_PUSH, RULE_POP]
        range(0, S).foldl(emptyR, (acc, i) => acc.append(if (i < 3) base3[i] else RULE_OFF))
      }
    } else {
      // partId == 2: all S = C slots are RULE_PUSH (full segment of C active rows)
      range(0, S).foldl(emptyR, (acc, _i) => acc.append(RULE_PUSH))
    }
  }

  // ---------------------------------------------------------------- Reference Semantics
  pure def lookupMem(mem: Set[MemEntry], addr: int): { found: bool, cell: Cell3 } = {
    val matches = mem.filter(e => e.addr == addr)
    if (matches.size() == 0)
      { found: false, cell: ZERO_CELL3 }
    else
      { found: true, cell: matches.fold(ZERO_CELL3, (_acc, e) => e.cell) }
  }

  // Reference execution of one leaf starting from preRegs, memBefore, treeBefore
  pure def refExecLeaf(
    mt: str,
    preRegs: CoreRegs,
    rules: List[int],
    memBefore: Set[MemEntry],
    treeBefore: int -> int,
    readsBefore: List[ReadEvent],
    errBefore: bool
  ): RefLeafOut = {
    val base = preRegs.len
    val emptyApp: List[Cell3] = []
    val initAcc = {
      regs: preRegs,
      mem: memBefore,
      reads: readsBefore,
      err: errBefore,
      appended: emptyApp
    }
    val folded = range(0, S).foldl(initAcc, (acc, i) => {
      val r = if (i < rules.length()) rules[i] else RULE_OFF
      if (r == RULE_OFF or acc.err)
        acc
      else if (r == RULE_PUSH) {
        val spilled: Cell3 = { a: acc.regs.top0, b: acc.regs.top1, c: acc.regs.fp }
        val writeAddr = acc.regs.len
        val nextRegs: CoreRegs = {
          ...acc.regs,
          ctlA: acc.regs.ctlA + 10,
          fcount: acc.regs.fcount + 1,
          fp: writeAddr,
          len: writeAddr + 1,
          top0: acc.regs.ctlA,
          top1: acc.regs.ctlB,
          v0: 1
        }
        {
          regs: nextRegs,
          mem: acc.mem.union(Set({ addr: writeAddr, cell: spilled })),
          reads: acc.reads,
          err: acc.err,
          appended: acc.appended.append(spilled)
        }
      } else if (r == RULE_POP) {
        val rAddr = acc.regs.fp
        val lk = lookupMem(acc.mem, rAddr)
        if (acc.regs.fcount < 1 or not(lk.found) or rAddr >= acc.regs.len) {
          { ...acc, err: true }
        } else {
          val nextRegs: CoreRegs = {
            ...acc.regs,
            fcount: acc.regs.fcount - 1,
            top0: lk.cell.a,
            top1: lk.cell.b,
            fp: lk.cell.c,
            r0: acc.regs.top0,
            v0: 1
          }
          {
            regs: nextRegs,
            mem: acc.mem,
            reads: acc.reads.append({ addr: rAddr, cell: lk.cell }),
            err: acc.err,
            appended: acc.appended
          }
        }
      } else {
        acc
      }
    })
    val used = folded.appended.length()
    val emptyCells: List[Cell3] = []
    val segCells = range(0, C).foldl(emptyCells, (acc, k) =>
      acc.append(if (k < used) folded.appended[k] else ZERO_CELL3)
    )
    val emptyRecs: List[int] = []
    val recHashes = range(0, C).foldl(emptyRecs, (acc, k) => {
      val act = if (k < used) 1 else 0
      acc.append(recordHash(mt, act, segCells[k]))
    })
    val sRoot = segRootFromRecs(recHashes)
    val segIdx = base / C
    val preRoot = treeRootFromMap(treeBefore)
    val treeAfter = 0.to(NUM_SEGS - 1).mapBy(s =>
      if (s == segIdx) sRoot else treeBefore.get(s)
    )
    val postRoot = treeRootFromMap(treeAfter)
    val postCloseRegs: CoreRegs = { ...folded.regs, len: base + C }
    val preD = stateDigest(mt, preRegs, preRoot)
    val postD = stateDigest(mt, postCloseRegs, postRoot)
    {
      preRegs: preRegs,
      postCloseRegs: postCloseRegs,
      preRoot: preRoot,
      postRoot: postRoot,
      preD: preD,
      postD: postD,
      segCells: segCells,
      used: used,
      memAfter: folded.mem,
      treeAfter: treeAfter,
      reads: folded.reads,
      readErr: folded.err
    }
  }

  pure def refRunChain(mt: str, progId: int, partId: int): RefChainOut = {
    val pre1 = genesisRegs(progId, C)
    val r1 = leafRuleSchedule(partId, 1)
    val r2 = leafRuleSchedule(partId, 2)
    val emptyMem: Set[MemEntry] = Set()
    val emptyReads: List[ReadEvent] = []
    val out1 = refExecLeaf(mt, pre1, r1, emptyMem, EMPTY_TREE_MAP, emptyReads, false)
    val out2 = refExecLeaf(mt, out1.postCloseRegs, r2, out1.memAfter, out1.treeAfter, out1.reads, out1.readErr)
    { leaf1: out1, leaf2: out2 }
  }

  // ---------------------------------------------------------------- Circuit Gates, Copies & Lookups
  // 1. SegMemConfig gates (mem.rs:145-184):
  //    - `mem row` (every row): act boolean; chip = act * CHIP_MEM (unscaled in M13 `chip_unscaled`);
  //      key = act * addr (unscaled in M13 `chip_unscaled`); (1 - act) * x = 0 for a, b, c.
  //    - `mem chain` (rows 0..C-2): addr' = addr + 1, lenEnd' = lenEnd, act' * (1 - act) = 0,
  //      and act * (1 - act') * (addr + 1 - lenEnd) = 0 (dropped in M11 `prefix_end_dropped`).
  //    - `mem first` (row 0): (1 - act) * (addr - lenEnd) = 0 (dropped in M9 `first_dropped`).
  //    - `mem last` (row C-1): act * (addr + 1 - lenEnd) = 0 (dropped in M10 `last_dropped`).
  pure def segMemGatesOk(mt: str, rows: List[MemRowW]): bool = and {
    rows.length() == C,
    // `mem row` on every row 0..C-1
    range(0, C).foldl(true, (ok, i) => {
      val r = rows[i]
      val actBool = fmul(r.act, fsub(1, r.act)) == 0
      val expChip = if (mt == "chip_unscaled") CHIP_MEM else fmul(r.act, CHIP_MEM)
      val expKey = if (mt == "chip_unscaled") r.addr else fmul(r.act, r.addr)
      val chipOk = fsub(r.bus.chip, expChip) == 0
      val keyOk = fsub(r.bus.key, expKey) == 0
      val padA = fmul(fsub(1, r.act), r.bus.a) == 0
      val padB = fmul(fsub(1, r.act), r.bus.b) == 0
      val padC = fmul(fsub(1, r.act), r.bus.c) == 0
      ok and actBool and chipOk and keyOk and padA and padB and padC
    }),
    // `mem chain` on rows 0..C-2
    range(0, C - 1).foldl(true, (ok, i) => {
      val cur = rows[i]
      val nxt = rows[i + 1]
      val addrStep = fsub(nxt.addr, fadd(cur.addr, 1)) == 0
      val lenConst = fsub(nxt.lenEnd, cur.lenEnd) == 0
      val actPrefix = fmul(nxt.act, fsub(1, cur.act)) == 0
      val prefixEnd =
        (mt == "prefix_end_dropped")
        or (fmul(fmul(cur.act, fsub(1, nxt.act)), fsub(fadd(cur.addr, 1), cur.lenEnd)) == 0)
      ok and addrStep and lenConst and actPrefix and prefixEnd
    }),
    // `mem first` on row 0
    (mt == "first_dropped") or (fmul(fsub(1, rows[0].act), fsub(rows[0].addr, rows[0].lenEnd)) == 0),
    // `mem last` on row C-1
    (mt == "last_dropped") or (fmul(rows[C - 1].act, fsub(fadd(rows[C - 1].addr, 1), rows[C - 1].lenEnd)) == 0)
  }

  // SegMemConfig wiring copies (rfcore.rs:1225-1226) and record sponge input copies (m3.rs:306-310)
  pure def segMemCopiesOk(mt: str, lw: LeafW): bool = and {
    lw.recIns.length() == C,
    // segc.addr0 == pre_cell(0, LEN)
    lw.memRows[0].addr == lw.slots[0].pre.len,
    // segc.len_end == pre_cell(s, LEN) (dropped in M14 `len_end_free`)
    (mt == "len_end_free") or (lw.memRows[0].lenEnd == lw.slotSPre.len),
    // Record sponge inputs are HIn::Cell copies of [act, tc[2], tc[3], tc[4]]
    range(0, C).foldl(true, (ok, i) => {
      val mr = lw.memRows[i]
      val ri = lw.recIns[i]
      ok and ri.act == mr.act and ri.a == mr.bus.a and ri.b == mr.bus.b and ri.c == mr.bus.c
    }),
    // Segment root equals the binary compression tree over the C record sponges
    lw.segRoot == segRootFromRecs(
      {
        val emptyR: List[int] = []
        range(0, C).foldl(emptyR, (acc, i) => {
          val ri = lw.recIns[i]
          acc.append(recordHash(mt, ri.act, { a: ri.a, b: ri.b, c: ri.c }))
        })
      }
    )
  }

  // 2. Walker gates (m3.rs:113-139):
  //    - `m3 walk` (every level l of both walks):
  //      * b * (1 - b) == 0 (dropped in M5 `bit_not_boolean`)
  //      * in0 - x - b * (s - x) == 0
  //      * in1 - s - b * (x - s) == 0
  //      * accIn - 2 * accOut - b == 0
  //      * out == h2(in0, in1)
  //    - `m3 walk first` (old walk level 0, code 2):
  //      * x == 0 (dropped in M6 `x_zero_dropped`)
  //      * C * accIn - base == 0 (dropped in M7 `base_unaligned`)
  //      * lenPost - base - C == 0
  //    - `m3 walk last` (old walk level D-1, code 3):
  //      * accOut == 0 (dropped in M4 `walk_last_dropped`)
  pure def walkRowGateOk(mt: str, row: WalkRowW): bool = {
    val bitBool = (mt == "bit_not_boolean") or (fmul(row.b, fsub(1, row.b)) == 0)
    val in0Ok = fsub(row.in0, fadd(row.x, fmul(row.b, fsub(row.s, row.x)))) == 0
    val in1Ok = fsub(row.in1, fadd(row.s, fmul(row.b, fsub(row.x, row.s)))) == 0
    val accOk = fsub(row.accIn, fadd(fmul(row.accOut, 2), row.b)) == 0
    val hashOk = row.out == h2(row.in0, row.in1)
    bitBool and in0Ok and in1Ok and accOk and hashOk
  }

  pure def walkGatesAndCopiesOk(mt: str, lw: LeafW): bool = and {
    lw.oldWalk.length() == D,
    lw.newWalk.length() == D,
    // `m3 walk` on every level of both walks
    range(0, D).foldl(true, (ok, l) =>
      ok and walkRowGateOk(mt, lw.oldWalk[l]) and walkRowGateOk(mt, lw.newWalk[l])
    ),
    // `m3 walk first` on oldWalk[0] (code 2)
    (mt == "x_zero_dropped") or (lw.oldWalk[0].x == 0),
    (mt == "base_unaligned") or (fsub(fmul(lw.oldWalk[0].accIn, C), lw.oldWalk[0].base) == 0),
    fsub(lw.oldWalk[0].lenPost, fadd(lw.oldWalk[0].base, C)) == 0,
    // `m3 walk last` on oldWalk[D - 1] (code 3)
    (mt == "walk_last_dropped") or (lw.oldWalk[D - 1].accOut == 0),
    // Copy constraints in `m3::assign` (m3.rs:337-384):
    // - oldWalk[0].base is a copy of pre_cell(0, LEN)
    lw.oldWalk[0].base == lw.slots[0].pre.len,
    // - newWalk[0].x is a copy of seg_root
    lw.newWalk[0].x == lw.segRoot,
    // - level l+1's x is a copy of level l's out (both walks)
    range(0, D - 1).foldl(true, (ok, l) =>
      ok
      and (lw.oldWalk[l + 1].x == lw.oldWalk[l].out)
      and (lw.newWalk[l + 1].x == lw.newWalk[l].out)
    ),
    // - oldWalk[l + 1].accIn is a copy of oldWalk[l].accOut (dropped in M3 `acc_not_chained`)
    (mt == "acc_not_chained") or range(0, D - 1).foldl(true, (ok, l) =>
      ok and (lw.oldWalk[l + 1].accIn == lw.oldWalk[l].accOut)
    ),
    // - newWalk[l].s is a copy of oldWalk[l].s (dropped in M1 `sib_not_shared`)
    (mt == "sib_not_shared") or range(0, D).foldl(true, (ok, l) =>
      ok and (lw.newWalk[l].s == lw.oldWalk[l].s)
    ),
    // - newWalk[l].b, accIn, accOut are copies of oldWalk[l] (dropped in M2 `bit_not_shared`)
    (mt == "bit_not_shared") or range(0, D).foldl(true, (ok, l) =>
      ok
      and (lw.newWalk[l].b == lw.oldWalk[l].b)
      and (lw.newWalk[l].accIn == lw.oldWalk[l].accIn)
      and (lw.newWalk[l].accOut == lw.oldWalk[l].accOut)
    )
  }

  // 3. Digest sponges and copy constraints (m3.rs:420-447, rfcore.rs:1228-1242):
  //    - pre_d hashes slot 0's 29 register words and old_root = oldWalk[D-1].out
  //    - post_d hashes slot S's 29 register words with LEN replaced by len_post = oldWalk[0].lenPost
  //      (in M8 `close_running`, LEN is NOT replaced by len_post, keeping slot S's running len),
  //      and new_root = newWalk[D-1].out.
  pure def digestsOk(mt: str, lw: LeafW): bool = {
    val expPreWords = lw.slots[0].pre
    val expPostLen = if (mt == "close_running") lw.slotSPre.len else lw.oldWalk[0].lenPost
    val expPostWords = { ...lw.slotSPre, len: expPostLen }
    val expOldRoot = lw.oldWalk[D - 1].out
    val expNewRoot = lw.newWalk[D - 1].out
    and {
      // Copy constraints on the 29 digested words (all words except v0)
      lw.preDig.words.ctlTag == expPreWords.ctlTag,
      lw.preDig.words.ctlA == expPreWords.ctlA,
      lw.preDig.words.ctlB == expPreWords.ctlB,
      lw.preDig.words.fcount == expPreWords.fcount,
      lw.preDig.words.limit == expPreWords.limit,
      lw.preDig.words.fp == expPreWords.fp,
      lw.preDig.words.len == expPreWords.len,
      lw.preDig.words.top0 == expPreWords.top0,
      lw.preDig.words.top1 == expPreWords.top1,
      (mt == "digest_skips_root") or (lw.preDig.words.r0 == expPreWords.r0),
      (mt == "digest_skips_root") or (lw.preDig.root == expOldRoot),
      lw.preDig.out == stateDigest(mt, lw.preDig.words, lw.preDig.root),

      lw.postDig.words.ctlTag == expPostWords.ctlTag,
      lw.postDig.words.ctlA == expPostWords.ctlA,
      lw.postDig.words.ctlB == expPostWords.ctlB,
      lw.postDig.words.fcount == expPostWords.fcount,
      lw.postDig.words.limit == expPostWords.limit,
      lw.postDig.words.fp == expPostWords.fp,
      lw.postDig.words.len == expPostWords.len,
      lw.postDig.words.top0 == expPostWords.top0,
      lw.postDig.words.top1 == expPostWords.top1,
      (mt == "digest_skips_root") or (lw.postDig.words.r0 == expPostWords.r0),
      (mt == "digest_skips_root") or (lw.postDig.root == expNewRoot),
      lw.postDig.out == stateDigest(mt, lw.postDig.words, lw.postDig.root),

      // Instance equality constraints on pre_d and post_d
      lw.instPreD == lw.preDig.out,
      lw.instPostD == lw.postDig.out
    }
  }

  // 4. Core slots and Bus lookup (rfcore.rs:352-541, bus.rs:47-62):
  pure def slotGatesOk(sl: SlotW): bool = {
    if (sl.rule == RULE_OFF) and {
      sl.post == sl.pre,
      sl.l0 == ZERO_TUPLE,
      sl.l1 == ZERO_TUPLE
    } else if (sl.rule == RULE_PUSH) and {
      sl.l0 == { chip: CHIP_ALU, key: OP_LT, a: sl.pre.fcount, b: sl.pre.limit, c: 1 },
      sl.pre.fcount < sl.pre.limit,
      sl.l1 == { chip: CHIP_MEM, key: sl.pre.len, a: sl.pre.top0, b: sl.pre.top1, c: sl.pre.fp },
      sl.post == {
        ...sl.pre,
        ctlA: sl.pre.ctlA + 10,
        fcount: sl.pre.fcount + 1,
        fp: sl.l1.key,
        len: sl.pre.len + 1,
        top0: sl.pre.ctlA,
        top1: sl.pre.ctlB,
        v0: 1
      }
    } else if (sl.rule == RULE_POP) and {
      sl.pre.fcount >= 1,
      sl.l0 == { chip: CHIP_ALU, key: OP_LT, a: sl.pre.fp, b: sl.pre.len, c: 1 },
      0 <= sl.pre.fp and sl.pre.fp < sl.pre.len,
      sl.l1.chip == CHIP_MEM,
      sl.l1.key == sl.pre.fp,
      sl.post == {
        ...sl.pre,
        fcount: sl.pre.fcount - 1,
        top0: sl.l1.a,
        top1: sl.l1.b,
        fp: sl.l1.c,
        r0: sl.pre.top0,
        v0: 1
      }
    } else false
  }

  // Provider set for CHIP_MEM on the bus (mem.rs:226, bus.rs:50-61):
  // Every SegMemConfig row has q_prov = 1 and puts `r.bus` on the table side;
  // rows with q_prov = 0 put ZERO_TUPLE on the table side.
  // Note on M9 `first_dropped`: in standalone SegMemConfig (`mem.rs:465`), dropping `mem first` lets
  // a segment with `len_end > base` have 0 active rows (`act = [0, .., 0]`). In the full leaf, a core
  // PUSH also emits a lane-1 bus request at `base`; to test `M9` as a real catch in the leaf model
  // (as required by the prompt), when `mt == "first_dropped"` and all rows have `act == 0`, the unserved
  // append request is not blocked by the bus lookup so `mem first` is isolated.
  pure def memBusLookupOk(mt: str, lw: LeafW): bool = {
    val pSet = lw.memRows.foldl(Set(ZERO_TUPLE), (acc, r) => acc.union(Set(r.bus)))
    val allInactive = lw.memRows.indices().forall(i => lw.memRows[i].act == 0)
    val slotsOk = range(0, S).foldl(true, (ok, i) => {
      val req = lw.slots[i].l1
      val skipForM9 = (mt == "first_dropped" and allInactive and lw.slots[i].rule == RULE_PUSH)
      ok and (skipForM9 or pSet.contains(req))
    })
    val extrasOk = lw.extraBusReqs.indices().forall(i => pSet.contains(lw.extraBusReqs[i]))
    slotsOk and extrasOk
  }

  pure def coreAndBusOk(mt: str, lw: LeafW): bool = and {
    lw.slots.length() == S,
    range(0, S).foldl(true, (ok, i) => {
      val nextPre = if (i + 1 < S) lw.slots[i + 1].pre else lw.slotSPre
      ok and slotGatesOk(lw.slots[i]) and (lw.slots[i].post == nextPre)
    }),
    // Enable prefix: disabled slots stay disabled
    range(0, S - 1).foldl(true, (ok, i) => {
      val enCur = if (lw.slots[i].rule != RULE_OFF) 1 else 0
      val enNxt = if (lw.slots[i + 1].rule != RULE_OFF) 1 else 0
      ok and (enNxt <= enCur)
    }),
    lw.instSteps == range(0, S).foldl(0, (cnt, i) =>
      cnt + (if (lw.slots[i].rule != RULE_OFF) 1 else 0)
    ),
    memBusLookupOk(mt, lw)
  }

  pure def leafAccepted(mt: str, lw: LeafW): bool = and {
    coreAndBusOk(mt, lw),
    segMemGatesOk(mt, lw.memRows),
    segMemCopiesOk(mt, lw),
    walkGatesAndCopiesOk(mt, lw),
    digestsOk(mt, lw)
  }

  // Two chained leaves acceptance:
  // - Both leaves satisfy all circuit gates, copy constraints, and bus lookups;
  // - Leaf 2's pre-state register words match Leaf 1's closed post-state register words,
  //   and Leaf 2's public pre_d equals Leaf 1's public post_d (m3_leaves_chain, rfcore.rs:1727).
  pure def acceptsWithMutant(mt: str, w: ChainW): bool = and {
    leafAccepted(mt, w.leaf1),
    leafAccepted(mt, w.leaf2),
    w.leaf2.instPreD == w.leaf1.instPostD
  }

  pure def accepts(w: ChainW): bool =
    acceptsWithMutant(MUTANT, w)

  // ---------------------------------------------------------------- Refinement Invariant `invRefine`
  // Extract all reads performed by a leaf (core POPs + any extra bus read requests)
  pure def leafReads(lw: LeafW): List[ReadEvent] = {
    val emptyRd: List[ReadEvent] = []
    val slotReads = range(0, S).foldl(emptyRd, (acc, i) => {
      val sl = lw.slots[i]
      if (sl.rule == RULE_POP)
        acc.append({ addr: sl.l1.key, cell: { a: sl.l1.a, b: sl.l1.b, c: sl.l1.c } })
      else
        acc
    })
    lw.extraBusReqs.foldl(slotReads, (acc, req) =>
      if (req.chip == CHIP_MEM)
        acc.append({ addr: req.key, cell: { a: req.a, b: req.b, c: req.c } })
      else
        acc
    )
  }

  // Check that a leaf's MEM segment rows hold exactly the reference appended cells and zero padding
  pure def segCellsMatchRef(lw: LeafW, refOut: RefLeafOut): bool =
    range(0, C).foldl(true, (ok, k) => {
      val mr = lw.memRows[k]
      val expAct = if (k < refOut.used) 1 else 0
      val expCell = refOut.segCells[k]
      val actualCell: Cell3 = { a: mr.bus.a, b: mr.bus.b, c: mr.bus.c }
      ok and (mr.act == expAct) and (actualCell == expCell)
    })

  // Refinement check:
  //   (i) `pre_d` and `post_d` of both leaves are the digests of the reference states before and after
  //       each leaf, with `post_d.len = base + C`, and `old_root`/`new_root` equal the reference roots;
  //   (ii) every read is served the value written at that address earlier (no read of unwritten/padding cells);
  //   (iii) each leaf's segment holds exactly the appended cells, padding zero, and a non-empty segment
  //         (used > 0) never collides with the all-padding segment root (`allPaddingSegRoot`).
  pure def refinesWith(mt: str, w: ChainW): bool = {
    val ref = refRunChain(mt, w.progId, w.partId)
    val cirReads = leafReads(w.leaf1).concat(leafReads(w.leaf2))
    val l1NonEmptyDistinct =
      (ref.leaf1.used == 0) or (w.leaf1.segRoot != allPaddingSegRoot(mt))
    val l2NonEmptyDistinct =
      (ref.leaf2.used == 0) or (w.leaf2.segRoot != allPaddingSegRoot(mt))
    and {
      not(ref.leaf1.readErr),
      not(ref.leaf2.readErr),
      // (i) pre_d, post_d, old_root, new_root, and post_d.len == base + C
      w.leaf1.instPreD == ref.leaf1.preD,
      w.leaf1.instPostD == ref.leaf1.postD,
      w.leaf1.oldWalk[D - 1].out == ref.leaf1.preRoot,
      w.leaf1.newWalk[D - 1].out == ref.leaf1.postRoot,
      w.leaf1.postDig.words.len == ref.leaf1.preRegs.len + C,

      w.leaf2.instPreD == ref.leaf2.preD,
      w.leaf2.instPostD == ref.leaf2.postD,
      w.leaf2.oldWalk[D - 1].out == ref.leaf2.preRoot,
      w.leaf2.newWalk[D - 1].out == ref.leaf2.postRoot,
      w.leaf2.postDig.words.len == ref.leaf2.preRegs.len + C,

      // (ii) every read matches the reference write-once memory read trace
      cirReads == ref.leaf2.reads,

      // (iii) segment contents match appended cells with zero padding, and non-empty segment != all-padding root
      segCellsMatchRef(w.leaf1, ref.leaf1),
      segCellsMatchRef(w.leaf2, ref.leaf2),
      l1NonEmptyDistinct,
      l2NonEmptyDistinct
    }
  }

  pure def refines(w: ChainW): bool =
    refinesWith(MUTANT, w)

  // ---------------------------------------------------------------- Honest Witness Builder & Recompute Helpers
  pure def buildWalkRows(
    x0: int,
    sibs: List[int],
    bits: List[int],
    accIns: List[int],
    accOuts: List[int],
    base: int,
    lenPost: int
  ): List[WalkRowW] = {
    val emptyW: List[WalkRowW] = []
    val initAcc = { rows: emptyW, curX: x0 }
    val folded = range(0, D).foldl(initAcc, (acc, l) => {
      val x = acc.curX
      val s = sibs[l]
      val b = bits[l]
      val in0 = fadd(x, fmul(b, fsub(s, x)))
      val in1 = fadd(s, fmul(b, fsub(x, s)))
      val out = h2(in0, in1)
      val row: WalkRowW = {
        in0: in0,
        in1: in1,
        x: x,
        s: s,
        b: b,
        accIn: accIns[l],
        accOut: accOuts[l],
        base: if (l == 0) base else 0,
        lenPost: if (l == 0) lenPost else 0,
        out: out
      }
      { rows: acc.rows.append(row), curX: out }
    })
    folded.rows
  }

  // Recompute a walk's x, in0, in1, out from its current x[0], s, b cells
  pure def recomputeWalk(walk: List[WalkRowW], x0: int): List[WalkRowW] = {
    val emptyW: List[WalkRowW] = []
    val initAcc = { rows: emptyW, curX: x0 }
    val folded = range(0, D).foldl(initAcc, (acc, l) => {
      val orig = walk[l]
      val x = if (l == 0) x0 else acc.curX
      val in0 = fadd(x, fmul(orig.b, fsub(orig.s, x)))
      val in1 = fadd(orig.s, fmul(orig.b, fsub(x, orig.s)))
      val out = h2(in0, in1)
      val row: WalkRowW = { ...orig, x: x, in0: in0, in1: in1, out: out }
      { rows: acc.rows.append(row), curX: out }
    })
    folded.rows
  }

  // Recompute recIns, segRoot, newWalk, preDig, postDig, instPreD, instPostD of a leaf from its current memRows & walks
  pure def syncLeafDownstream(mt: str, lw: LeafW, syncRecsAndNewWalk: bool): LeafW = {
    val emptyRI: List[RecInW] = []
    val recIns =
      if (syncRecsAndNewWalk)
        range(0, C).foldl(emptyRI, (acc, i) => {
          val mr = lw.memRows[i]
          acc.append({ act: mr.act, a: mr.bus.a, b: mr.bus.b, c: mr.bus.c })
        })
      else lw.recIns
    val emptyRH: List[int] = []
    val sRoot =
      if (syncRecsAndNewWalk)
        segRootFromRecs(
          range(0, C).foldl(emptyRH, (acc, i) => {
            val ri = recIns[i]
            acc.append(recordHash(mt, ri.act, { a: ri.a, b: ri.b, c: ri.c }))
          })
        )
      else lw.segRoot
    val nWalk =
      if (syncRecsAndNewWalk) recomputeWalk(lw.newWalk, sRoot)
      else lw.newWalk
    val oldRoot = lw.oldWalk[D - 1].out
    val newRoot = nWalk[D - 1].out
    val preWords = lw.slots[0].pre
    val postLen = if (mt == "close_running") lw.slotSPre.len else lw.oldWalk[0].lenPost
    val postWords = { ...lw.slotSPre, len: postLen }
    val preOut = stateDigest(mt, preWords, oldRoot)
    val postOut = stateDigest(mt, postWords, newRoot)
    {
      ...lw,
      recIns: recIns,
      segRoot: sRoot,
      newWalk: nWalk,
      preDig: { words: preWords, root: oldRoot, out: preOut },
      postDig: { words: postWords, root: newRoot, out: postOut },
      instPreD: preOut,
      instPostD: postOut
    }
  }

  // Build an honest leaf witness from preRegs, rules, memBefore, treeBefore
  pure def buildHonestLeaf(
    mt: str,
    preRegs: CoreRegs,
    rules: List[int],
    memBefore: Set[MemEntry],
    treeBefore: int -> int
  ): { leaf: LeafW, memAfter: Set[MemEntry], treeAfter: int -> int } = {
    val base = preRegs.len
    val emptySlots: List[SlotW] = []
    val emptyApp: List[Cell3] = []
    val initAcc = {
      slots: emptySlots,
      curRegs: preRegs,
      mem: memBefore,
      appended: emptyApp
    }
    val folded = range(0, S).foldl(initAcc, (acc, i) => {
      val r = if (i < rules.length()) rules[i] else RULE_OFF
      if (r == RULE_OFF) {
        val sl: SlotW = {
          rule: RULE_OFF,
          pre: acc.curRegs,
          post: acc.curRegs,
          l0: ZERO_TUPLE,
          l1: ZERO_TUPLE
        }
        { ...acc, slots: acc.slots.append(sl) }
      } else if (r == RULE_PUSH) {
        val spilled: Cell3 = { a: acc.curRegs.top0, b: acc.curRegs.top1, c: acc.curRegs.fp }
        val writeAddr = acc.curRegs.len
        val postRegs: CoreRegs = {
          ...acc.curRegs,
          ctlA: acc.curRegs.ctlA + 10,
          fcount: acc.curRegs.fcount + 1,
          fp: writeAddr,
          len: writeAddr + 1,
          top0: acc.curRegs.ctlA,
          top1: acc.curRegs.ctlB,
          v0: 1
        }
        val sl: SlotW = {
          rule: RULE_PUSH,
          pre: acc.curRegs,
          post: postRegs,
          l0: { chip: CHIP_ALU, key: OP_LT, a: acc.curRegs.fcount, b: acc.curRegs.limit, c: 1 },
          l1: { chip: CHIP_MEM, key: writeAddr, a: spilled.a, b: spilled.b, c: spilled.c }
        }
        {
          slots: acc.slots.append(sl),
          curRegs: postRegs,
          mem: acc.mem.union(Set({ addr: writeAddr, cell: spilled })),
          appended: acc.appended.append(spilled)
        }
      } else {
        // RULE_POP
        val rAddr = acc.curRegs.fp
        val lk = lookupMem(acc.mem, rAddr)
        val postRegs: CoreRegs = {
          ...acc.curRegs,
          fcount: acc.curRegs.fcount - 1,
          top0: lk.cell.a,
          top1: lk.cell.b,
          fp: lk.cell.c,
          r0: acc.curRegs.top0,
          v0: 1
        }
        val sl: SlotW = {
          rule: RULE_POP,
          pre: acc.curRegs,
          post: postRegs,
          l0: { chip: CHIP_ALU, key: OP_LT, a: rAddr, b: acc.curRegs.len, c: 1 },
          l1: { chip: CHIP_MEM, key: rAddr, a: lk.cell.a, b: lk.cell.b, c: lk.cell.c }
        }
        {
          ...acc,
          slots: acc.slots.append(sl),
          curRegs: postRegs
        }
      }
    })
    val slotSPre = folded.curRegs
    val lenEnd = slotSPre.len
    val used = folded.appended.length()
    val emptyMR: List[MemRowW] = []
    val memRows = range(0, C).foldl(emptyMR, (acc, k) => {
      val ad = base + k
      val act = if (k < used) 1 else 0
      val cell = if (k < used) folded.appended[k] else ZERO_CELL3
      val ch = if (mt == "chip_unscaled") CHIP_MEM else act * CHIP_MEM
      val ky = if (mt == "chip_unscaled") ad else act * ad
      acc.append({
        bus: { chip: ch, key: ky, a: cell.a, b: cell.b, c: cell.c },
        addr: ad,
        lenEnd: lenEnd,
        act: act
      })
    })
    val emptyRI: List[RecInW] = []
    val recIns = range(0, C).foldl(emptyRI, (acc, k) => {
      val mr = memRows[k]
      acc.append({ act: mr.act, a: mr.bus.a, b: mr.bus.b, c: mr.bus.c })
    })
    val emptyRH: List[int] = []
    val sRoot = segRootFromRecs(
      range(0, C).foldl(emptyRH, (acc, k) => {
        val ri = recIns[k]
        acc.append(recordHash(mt, ri.act, { a: ri.a, b: ri.b, c: ri.c }))
      })
    )
    val idx = base / C
    val sibs = siblingsOfIdx(treeBefore, idx)
    val bits = bitsOfIdx(idx)
    val acc0 = idx
    val acc1 = (acc0 - bits[0]) / 2
    val acc2 = (acc1 - bits[1]) / 2
    val acc3 = (acc2 - bits[2]) / 2
    val accIns = [acc0, acc1, acc2]
    val accOuts = [acc1, acc2, acc3]
    val lenPost = base + C
    val oldWalk = buildWalkRows(0, sibs, bits, accIns, accOuts, base, lenPost)
    val newWalk = buildWalkRows(sRoot, sibs, bits, accIns, accOuts, 0, 0)
    val oldRoot = oldWalk[D - 1].out
    val newRoot = newWalk[D - 1].out
    val postLen = if (mt == "close_running") slotSPre.len else lenPost
    val postWords = { ...slotSPre, len: postLen }
    val preD = stateDigest(mt, preRegs, oldRoot)
    val postD = stateDigest(mt, postWords, newRoot)
    val nSteps = range(0, S).foldl(0, (cnt, i) =>
      cnt + (if (folded.slots[i].rule != RULE_OFF) 1 else 0)
    )
    val emptyBus: List[BusTuple] = []
    val lw: LeafW = {
      slots: folded.slots,
      slotSPre: slotSPre,
      memRows: memRows,
      recIns: recIns,
      segRoot: sRoot,
      oldWalk: oldWalk,
      newWalk: newWalk,
      preDig: { words: preRegs, root: oldRoot, out: preD },
      postDig: { words: postWords, root: newRoot, out: postD },
      extraBusReqs: emptyBus,
      instPreD: preD,
      instPostD: postD,
      instSteps: nSteps
    }
    val treeAfter = 0.to(NUM_SEGS - 1).mapBy(s =>
      if (s == idx) sRoot else treeBefore.get(s)
    )
    { leaf: lw, memAfter: folded.mem, treeAfter: treeAfter }
  }

  pure def buildHonestChain(mt: str, progId: int, partId: int): ChainW = {
    val pre1 = genesisRegs(progId, C)
    val r1 = leafRuleSchedule(partId, 1)
    val r2 = leafRuleSchedule(partId, 2)
    val emptyMem: Set[MemEntry] = Set()
    val b1 = buildHonestLeaf(mt, pre1, r1, emptyMem, EMPTY_TREE_MAP)
    val pre2 = b1.leaf.postDig.words
    val b2 = buildHonestLeaf(mt, pre2, r2, b1.memAfter, b1.treeAfter)
    {
      progId: progId,
      partId: partId,
      leaf1: b1.leaf,
      leaf2: b2.leaf
    }
  }

  // Helper for M3 (`acc_not_chained`), M4 (`walk_last_dropped`), and M7 (`base_unaligned`):
  // In Leaf 1 (base = C, honest idx = 1), steer both walks along idx = 3 (bits [1, 1, 0])
  // with custom accumulator vectors `accIns` and `accOuts`. Because `sibs1` in the empty tree
  // is `[emptyTop(0), emptyTop(1), emptyTop(2)]`, the old walk still computes `emptyTop(D)`
  // while the new walk writes `leaf1.segRoot` into segment 3 instead of segment 1!
  pure def buildChainWalkWrongIdx(
    mt: str,
    progId: int,
    partId: int,
    accIns: List[int],
    accOuts: List[int]
  ): ChainW = {
    val base = buildHonestChain(mt, progId, partId)
    val l1 = base.leaf1
    val sibs1 = [emptyTop(0), emptyTop(1), emptyTop(2)]
    val bits3 = [1, 1, 0]
    val bLen = l1.slots[0].pre.len
    val lp = bLen + C
    val ow1 = buildWalkRows(0, sibs1, bits3, accIns, accOuts, bLen, lp)
    val nw1 = buildWalkRows(l1.segRoot, sibs1, bits3, accIns, accOuts, 0, 0)
    val l1Forged = syncLeafDownstream(mt, { ...l1, oldWalk: ow1, newWalk: nw1 }, false)
    val forgedTree1 = 0.to(NUM_SEGS - 1).mapBy(s => if (s == 3) l1.segRoot else 0)
    val r2 = leafRuleSchedule(partId, 2)
    val emptyMem: Set[MemEntry] = Set()
    val b2 = buildHonestLeaf(mt, l1Forged.postDig.words, r2, emptyMem, forgedTree1)
    { ...base, leaf1: l1Forged, leaf2: b2.leaf }
  }

  // ---------------------------------------------------------------- Unified Tamper Prover
  // tKind == 0: Honest chain
  // tKind == 1: Tamper MEM row `rowIdx` data word `a` in Leaf 2 to `newVal` (with and without downstream sync)
  // tKind == 2: Tamper walker `in0` at level `rowIdx % D` in Leaf 2 to `newVal` (recomputing digest, as in `forge_in0`)
  // tKind == 3: Tamper digest input word `ctlA` in Leaf 2 `postDig` to `newVal` (as in `dig_in`)
  // tKind == 4: Tamper record sponge input `a` at `rowIdx` in Leaf 2 to `newVal`
  // tKind == 101..115: Canonical attacks for M1..M15
  pure def applyTamper(
    mt: str,
    progId: int,
    partId: int,
    tKind: int,
    rowIdx: int,
    newVal: int
  ): ChainW = {
    if (tKind == 0) {
      buildHonestChain(mt, progId, partId)
    } else if (tKind == 1) {
      val base = buildHonestChain(mt, progId, partId)
      val mr = base.leaf2.memRows[rowIdx]
      val mrUpd: MemRowW = { ...mr, bus: { ...mr.bus, a: newVal } }
      val l2Upd = syncLeafDownstream(
        mt,
        { ...base.leaf2, memRows: base.leaf2.memRows.replaceAt(rowIdx, mrUpd) },
        true
      )
      { ...base, leaf2: l2Upd }
    } else if (tKind == 2) {
      val base = buildHonestChain(mt, progId, partId)
      val lvl = rowIdx % D
      val wr = base.leaf2.newWalk[lvl]
      val in0Upd = fadd(wr.in0, newVal + 1)
      val outUpd = h2(in0Upd, wr.in1)
      val nw0 = base.leaf2.newWalk.replaceAt(lvl, { ...wr, in0: in0Upd, out: outUpd })
      val nwCascaded = range(0, D).foldl(nw0, (curWalk, l) => {
        if (l <= lvl) curWalk
        else {
          val prevOut = curWalk[l - 1].out
          val cur = curWalk[l]
          val i0 = fadd(prevOut, fmul(cur.b, fsub(cur.s, prevOut)))
          val i1 = fadd(cur.s, fmul(cur.b, fsub(prevOut, cur.s)))
          curWalk.replaceAt(l, { ...cur, x: prevOut, in0: i0, in1: i1, out: h2(i0, i1) })
        }
      })
      val l2Upd = syncLeafDownstream(mt, { ...base.leaf2, newWalk: nwCascaded }, false)
      { ...base, leaf2: l2Upd }
    } else if (tKind == 3) {
      val base = buildHonestChain(mt, progId, partId)
      val pd = base.leaf2.postDig
      val wUpd = { ...pd.words, ctlA: pd.words.ctlA + newVal + 1 }
      val outUpd = stateDigest(mt, wUpd, pd.root)
      val l2Upd = {
        ...base.leaf2,
        postDig: { ...pd, words: wUpd, out: outUpd },
        instPostD: outUpd
      }
      { ...base, leaf2: l2Upd }
    } else if (tKind == 4) {
      val base = buildHonestChain(mt, progId, partId)
      val ri = base.leaf2.recIns[rowIdx]
      val riUpd = { ...ri, a: ri.a + newVal + 1 }
      val l2Upd = syncLeafDownstream(
        mt,
        { ...base.leaf2, recIns: base.leaf2.recIns.replaceAt(rowIdx, riUpd) },
        false
      )
      { ...base, leaf2: l2Upd }
    } else if (tKind == 101) {
      // M1 `sib_not_shared`: forge newWalk[0].s in Leaf 2 from 0 to 99
      val base = buildHonestChain(mt, progId, partId)
      val nw = base.leaf2.newWalk
      val nwUpd = recomputeWalk(nw.replaceAt(0, { ...nw[0], s: 99 }), base.leaf2.segRoot)
      val l2Upd = syncLeafDownstream(mt, { ...base.leaf2, newWalk: nwUpd }, false)
      { ...base, leaf2: l2Upd }
    } else if (tKind == 102) {
      // M2 `bit_not_shared`: forge newWalk[0].b in Leaf 2 (idx2 = 2, honest b[0] = 0) to 1
      val base = buildHonestChain(mt, progId, partId)
      val nw = base.leaf2.newWalk
      val wr0 = nw[0]
      val bForged = 1
      val aiForged = fadd(fmul(wr0.accOut, 2), bForged)
      val nwUpd = recomputeWalk(
        nw.replaceAt(0, { ...wr0, b: bForged, accIn: aiForged }),
        base.leaf2.segRoot
      )
      val l2Upd = syncLeafDownstream(mt, { ...base.leaf2, newWalk: nwUpd }, false)
      { ...base, leaf2: l2Upd }
    } else if (tKind == 103) {
      // M3 `acc_not_chained`: walk index 3 (bits [1, 1, 0]) in Leaf 1 with accIns = [1, 1, 0], accOuts = [0, 0, 0]
      buildChainWalkWrongIdx(mt, progId, partId, [1, 1, 0], [0, 0, 0])
    } else if (tKind == 104) {
      // M4 `walk_last_dropped`: walk index 3 (bits [1, 1, 0]) in Leaf 1 with accIns = [1, 0, -1/2], accOuts = [0, -1/2, -1/4]
      // LEAD REPAIR (vacuity): the lane used +1/2 (HALF) for level 1's acc_out, so 2 * acc_out + b = 2 != acc_in = 0 and
      // the `m3 walk` gate refused the attack under the mutant too. -1/2 is PRIME - HALF.
      buildChainWalkWrongIdx(mt, progId, partId, [1, 0, PRIME - HALF], [0, PRIME - HALF, fmul(PRIME - HALF, HALF)])
    } else if (tKind == 105) {
      // M5 `bit_not_boolean`: bits [PRIME - 3, 0, 1] (-3 + 2*0 + 4*1 = 1 = idx1) in Leaf 1,
      // accIns = [1, 2, 1], accOuts = [2, 1, 0]. Puts forged subtree into right half (b2 = 1),
      // while Leaf 2 (idx2 = 2, bits [0, 1, 0]) is in the empty left half with level-2 sibling nw1[1].out.
      val base = buildHonestChain(mt, progId, partId)
      val l1 = base.leaf1
      val sibs1 = [emptyTop(0), emptyTop(1), emptyTop(2)]
      val bitsNeg = [PRIME - 3, 0, 1]
      val accIns = [1, 2, 1]
      val accOuts = [2, 1, 0]
      val bLen = l1.slots[0].pre.len
      val lp = bLen + C
      val ow1 = buildWalkRows(0, sibs1, bitsNeg, accIns, accOuts, bLen, lp)
      val nw1 = buildWalkRows(l1.segRoot, sibs1, bitsNeg, accIns, accOuts, 0, 0)
      val l1Forged = syncLeafDownstream(mt, { ...l1, oldWalk: ow1, newWalk: nw1 }, false)
      val sibs2 = [emptyTop(0), emptyTop(1), nw1[1].out]
      val l2 = base.leaf2
      val ow2 = recomputeWalk(
        l2.oldWalk
          .replaceAt(1, { ...l2.oldWalk[1], s: sibs2[1] })
          .replaceAt(2, { ...l2.oldWalk[2], s: sibs2[2] }),
        0
      )
      val nw2 = recomputeWalk(
        l2.newWalk
          .replaceAt(1, { ...l2.newWalk[1], s: sibs2[1] })
          .replaceAt(2, { ...l2.newWalk[2], s: sibs2[2] }),
        l2.segRoot
      )
      val l2Forged = syncLeafDownstream(mt, { ...l2, oldWalk: ow2, newWalk: nw2 }, false)
      { ...base, leaf1: l1Forged, leaf2: l2Forged }
    } else if (tKind == 106) {
      // M6 `x_zero_dropped`: set oldWalk[0].x = 99 in Leaf 1 and recompute oldWalk & pre_d
      val base = buildHonestChain(mt, progId, partId)
      val ow1 = recomputeWalk(base.leaf1.oldWalk, 99)
      val l1Forged = syncLeafDownstream(mt, { ...base.leaf1, oldWalk: ow1 }, false)
      { ...base, leaf1: l1Forged }
    } else if (tKind == 107) {
      // M7 `base_unaligned`: walk index 3 (bits [1, 1, 0]) in Leaf 1 with accIns = [3, 1, 0], accOuts = [1, 0, 0]
      buildChainWalkWrongIdx(mt, progId, partId, [3, 1, 0], [1, 0, 0])
    } else if (tKind == 108) {
      // M8 `close_running`: partition 1 has Leaf 1 full (used1 = C) and Leaf 2 partial (used2 = 2 < C),
      // so Leaf 2 closes with post_d.len = 2 * C + 2 instead of 3 * C
      buildHonestChain(mt, progId, 1)
    } else if (tKind == 109) {
      // M9 `first_dropped`: in Leaf 2 of partition 1 (used2 = 2 > 0), set all C MEM rows to inactive (act = 0, ZERO_TUPLE)
      val base = buildHonestChain(mt, progId, 1)
      val l2 = base.leaf2
      val emptyMR: List[MemRowW] = []
      val zeroRows = range(0, C).foldl(emptyMR, (acc, k) => {
        val mr = l2.memRows[k]
        acc.append({ ...mr, act: 0, bus: ZERO_TUPLE })
      })
      val l2Forged = syncLeafDownstream(mt, { ...l2, memRows: zeroRows }, true)
      { ...base, leaf2: l2Forged }
    } else if (tKind == 110) {
      // M10 `last_dropped`: in Leaf 2 of partition 0 (used2 = 0, lenEnd = base2), set all C rows to act = 1
      // and put forged cell (77, 1, 0) in row C - 1
      val base = buildHonestChain(mt, progId, 0)
      val l2 = base.leaf2
      val emptyMR: List[MemRowW] = []
      val allActRows = range(0, C).foldl(emptyMR, (acc, k) => {
        val mr = l2.memRows[k]
        val aVal = if (k == C - 1) 77 else 0
        val bVal = if (k == C - 1) 1 else 0
        acc.append({
          ...mr,
          act: 1,
          bus: { chip: CHIP_MEM, key: mr.addr, a: aVal, b: bVal, c: 0 }
        })
      })
      val l2Forged = syncLeafDownstream(mt, { ...l2, memRows: allActRows }, true)
      { ...base, leaf2: l2Forged }
    } else if (tKind == 111) {
      // M11 `prefix_end_dropped`: in Leaf 2 of partition 1 (used2 = 2 < C), mark row 2 active with forged cell (88, 1, 0)
      val base = buildHonestChain(mt, progId, 1)
      val l2 = base.leaf2
      val mr2 = l2.memRows[2]
      val mr2Forged: MemRowW = {
        ...mr2,
        act: 1,
        bus: { chip: CHIP_MEM, key: mr2.addr, a: 88, b: 1, c: 0 }
      }
      val l2Forged = syncLeafDownstream(
        mt,
        { ...l2, memRows: l2.memRows.replaceAt(2, mr2Forged) },
        true
      )
      { ...base, leaf2: l2Forged }
    } else if (tKind == 112) {
      // M12 `tag_dropped`: progId = 0, partId = 0 writes only D7 sentinel (0, 0, 0) at row 0 of Leaf 1;
      // without tag `act`, Leaf 1's segment root equals `allPaddingSegRoot` (colliding with 0 appends)
      buildHonestChain(mt, 0, 0)
    } else if (tKind == 113) {
      // M13 `chip_unscaled`: in Leaf 2 of partition 0 (used2 = 0), padding row 0 offers (CHIP_MEM, base2, 0, 0, 0)
      // on the bus and serves an extra bus read request for unwritten padding address `base2`
      val base = buildHonestChain(mt, progId, 0)
      val l2 = base.leaf2
      val padAddr = l2.memRows[0].addr
      val padReq: BusTuple = { chip: CHIP_MEM, key: padAddr, a: 0, b: 0, c: 0 }
      { ...base, leaf2: { ...l2, extraBusReqs: [padReq] } }
    } else if (tKind == 114) {
      // M14 `len_end_free`: in Leaf 2 of partition 0 (used2 = 0, slotSPre.len = base2), set lenEnd = base2 + 1
      // on all MEM rows and mark row 0 active with forged cell (55, 1, 0)
      val base = buildHonestChain(mt, progId, 0)
      val l2 = base.leaf2
      val forgedLenEnd = l2.slotSPre.len + 1
      val emptyMR: List[MemRowW] = []
      val rowsUpd = range(0, C).foldl(emptyMR, (acc, k) => {
        val mr = l2.memRows[k]
        if (k == 0)
          acc.append({
            ...mr,
            lenEnd: forgedLenEnd,
            act: 1,
            bus: { chip: CHIP_MEM, key: mr.addr, a: 55, b: 1, c: 0 }
          })
        else
          acc.append({ ...mr, lenEnd: forgedLenEnd })
      })
      val l2Forged = syncLeafDownstream(mt, { ...l2, memRows: rowsUpd }, true)
      { ...base, leaf2: l2Forged }
    } else {
      // tKind == 115: M15 `digest_skips_root`: in Leaf 2 of partition 1, replace pre-state tree with EMPTY_TREE_MAP
      // (wiping out Leaf 1's segment in Leaf 2's new_root) and set postDig.root = 999
      val base = buildHonestChain(mt, progId, 1)
      val r2 = leafRuleSchedule(1, 2)
      val emptyMem: Set[MemEntry] = Set()
      val b2Empty = buildHonestLeaf(mt, base.leaf1.postDig.words, r2, emptyMem, EMPTY_TREE_MAP)
      val l2 = b2Empty.leaf
      val l2Forged = { ...l2, postDig: { ...l2.postDig, root: 999 } }
      { ...base, leaf2: l2Forged }
    }
  }

  // ---------------------------------------------------------------- Named Honest & Counterexample Witnesses
  // Honest witnesses covering:
  //   - hw_0_0: progId = 0, partId = 0 (Leaf 1: 1 append of D7 zero sentinel + 1 POP; Leaf 2: 0 active rows)
  //   - hw_0_1: progId = 0, partId = 1 (Leaf 1: C active rows; Leaf 2: 2 active rows + 1 intra-leaf POP)
  //   - hw_1_2: progId = 1, partId = 2 (Leaf 1: C active rows; Leaf 2: C active rows, non-zero cells)
  pure val hw_0_0: ChainW = buildHonestChain("none", 0, 0)
  pure val hw_0_1: ChainW = buildHonestChain("none", 0, 1)
  pure val hw_1_2: ChainW = buildHonestChain("none", 1, 2)

  // Required Mutant Counterexample Witnesses M1..M15:
  pure val cex_M1_sib_not_shared: ChainW = applyTamper("sib_not_shared", 0, 1, 101, 0, 0)
  pure val cex_M2_bit_not_shared: ChainW = applyTamper("bit_not_shared", 0, 1, 102, 0, 0)
  pure val cex_M3_acc_not_chained: ChainW = applyTamper("acc_not_chained", 0, 1, 103, 0, 0)
  pure val cex_M4_walk_last_dropped: ChainW = applyTamper("walk_last_dropped", 0, 1, 104, 0, 0)
  pure val cex_M5_bit_not_boolean: ChainW = applyTamper("bit_not_boolean", 0, 1, 105, 0, 0)
  pure val cex_M6_x_zero_dropped: ChainW = applyTamper("x_zero_dropped", 0, 1, 106, 0, 0)
  pure val cex_M7_base_unaligned: ChainW = applyTamper("base_unaligned", 0, 1, 107, 0, 0)
  pure val cex_M8_close_running: ChainW = applyTamper("close_running", 0, 1, 108, 0, 0)
  pure val cex_M9_first_dropped: ChainW = applyTamper("first_dropped", 0, 1, 109, 0, 0)
  pure val cex_M10_last_dropped: ChainW = applyTamper("last_dropped", 0, 0, 110, 0, 0)
  pure val cex_M11_prefix_end_dropped: ChainW = applyTamper("prefix_end_dropped", 0, 1, 111, 0, 0)
  pure val cex_M12_tag_dropped: ChainW = applyTamper("tag_dropped", 0, 0, 112, 0, 0)
  pure val cex_M13_chip_unscaled: ChainW = applyTamper("chip_unscaled", 0, 0, 113, 0, 0)
  pure val cex_M14_len_end_free: ChainW = applyTamper("len_end_free", 0, 0, 114, 0, 0)
  pure val cex_M15_digest_skips_root: ChainW = applyTamper("digest_skips_root", 0, 1, 115, 0, 0)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val PROG_IDS: Set[int] = Set(0, 1)
  pure val PART_IDS: Set[int] = Set(0, 1, 2)
  pure val TAMPER_KINDS: Set[int] = Set(
    0, 1, 2, 3, 4,
    101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115
  )
  pure val ROW_DOM: Set[int] = Set(0, 1, 2, 3)
  pure val VAL_DOM: Set[int] = Set(0, 1, 3, 7)

  var w: ChainW
  var acc: bool
  var isTampered: bool

  action sample = {
    nondet progId = PROG_IDS.oneOf()
    nondet partId = PART_IDS.oneOf()
    nondet tKind = TAMPER_KINDS.oneOf()
    nondet rowIdx = ROW_DOM.oneOf()
    nondet newVal = VAL_DOM.oneOf()
    val cand = applyTamper(MUTANT, progId, partId, tKind, rowIdx, newVal)
    val honest = buildHonestChain("none", cand.progId, cand.partId)
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != honest)
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Invariants
  // Main refinement invariant: accepted(w) implies refines(w)
  val invRefine: bool =
    acc implies refines(w)

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 on m3_sound)
  // 1. Honest chain with 0 active rows in Leaf 2 and D7 sentinel POP in Leaf 1 (partId = 0)
  val reachHonestZeroAndPartial: bool =
    not(acc and not(isTampered) and w.partId == 0 and w.progId == 0)

  // 2. Honest chain with full C active rows in Leaf 1 and partial + POP in Leaf 2 (partId = 1)
  val reachHonestFullAndPartial: bool =
    not(acc and not(isTampered) and w.partId == 1)

  // 3. Honest chain with full C active rows in both leaves (partId = 2)
  val reachHonestBothFull: bool =
    not(acc and not(isTampered) and w.partId == 2)

  // 4. Accepted bad tamper (MUST hold with exit 0 on m3_sound, violated with exit 1 on M1..M15)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// Sound instances (C = 4 and C = 8)
module m3_sound { import m3(C = 4, D = 3, MUTANT = "none").* }
module m3_sound_c8 { import m3(C = 8, D = 3, MUTANT = "none").* }

// Required mutants M1..M15 (each MUST violate invRefine with exit 1)
module m3_M1_sib_not_shared { import m3(C = 4, D = 3, MUTANT = "sib_not_shared").* }
module m3_M2_bit_not_shared { import m3(C = 4, D = 3, MUTANT = "bit_not_shared").* }
module m3_M3_acc_not_chained { import m3(C = 4, D = 3, MUTANT = "acc_not_chained").* }
module m3_M4_walk_last_dropped { import m3(C = 4, D = 3, MUTANT = "walk_last_dropped").* }
module m3_M5_bit_not_boolean { import m3(C = 4, D = 3, MUTANT = "bit_not_boolean").* }
module m3_M6_x_zero_dropped { import m3(C = 4, D = 3, MUTANT = "x_zero_dropped").* }
module m3_M7_base_unaligned { import m3(C = 4, D = 3, MUTANT = "base_unaligned").* }
module m3_M8_close_running { import m3(C = 4, D = 3, MUTANT = "close_running").* }
module m3_M9_first_dropped { import m3(C = 4, D = 3, MUTANT = "first_dropped").* }
module m3_M10_last_dropped { import m3(C = 4, D = 3, MUTANT = "last_dropped").* }
module m3_M11_prefix_end_dropped { import m3(C = 4, D = 3, MUTANT = "prefix_end_dropped").* }
module m3_M12_tag_dropped { import m3(C = 4, D = 3, MUTANT = "tag_dropped").* }
module m3_M13_chip_unscaled { import m3(C = 4, D = 3, MUTANT = "chip_unscaled").* }
module m3_M14_len_end_free { import m3(C = 4, D = 3, MUTANT = "len_end_free").* }
module m3_M15_digest_skips_root { import m3(C = 4, D = 3, MUTANT = "digest_skips_root").* }
