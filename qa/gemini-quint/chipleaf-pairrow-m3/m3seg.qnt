// Soundness & Completeness Review Model — Part 2: M3 Aligned-Segment Memory Root
// Design under test:
//   - research/sherlock-processor-analogy-20261110/experiments-2.md sections 3.4-3.6 and 4 (M3 "aligned segment", D3)
//   - openspec/changes/sherlock-vm-consensus-20261107/design.md section 2.7.8 (D1..D8)
//   - crates/mori-chip/src/rf.rs (NIL = 0 vs NIL != 0)
//
// Models:
//   - Write-once append log with aligned segments of height C (parameter C = 4 or 8).
//   - Leaf MEM region: C rows at addresses base .. base + C - 1, where base = pre_d.len and base % C == 0.
//   - Appends request (TAG_CELL, runningLen, val) and advance runningLen by 1; at most C appends per leaf.
//   - MEM rows 0 .. used - 1 (used = runningLen - base at leaf close) are active; rows used .. C - 1 are
//     padding and MUST be the zero cell (0, 0).
//   - mem_root is an exact segment map (segIdx -> List[CellVal] of length C when filled, [] when empty).
//     Segment root hashes ALL C rows and is inserted at index base / C after proving slot base / C empty
//     in pre_d.mem_root (siblings at s != base / C preserved).
//   - Explicit Leaf Close step sets post_d.len := base + C and post_d.mem_root := updated tree.
//   - D2: PAGE serves TAG_CELL for addr < base by opening segment addr / C, offset addr % C in pre_d.mem_root.
//   - Proposed D3: A read SHALL refuse the zero tuple (0, 0).
//   - Two chained leaves, nondeterministic tamper prover, and required mutants M1..M7.

module m3seg {
  const S: int
  const C: int
  const MUTANT: str

  // ---------------------------------------------------------------- Constants
  pure val TAG_PAD: int = 0
  pure val TAG_CELL: int = 2
  pure val NIL_DEFAULT: int = 99
  pure val SEG_DOM: Set[int] = Set(0, 1, 2, 3)

  // Operations
  pure val OP_DISABLED: int = 0
  pure val OP_PUSH: int = 1
  pure val OP_POP: int = 2
  pure val OP_ALLOC: int = 3
  pure val OP_LOAD: int = 4

  // ---------------------------------------------------------------- Types
  type CellVal = {
    a: int,
    link: int
  }

  type BusTuple = {
    chip: int,
    key: int,
    a: int,
    link: int
  }

  type RegState = {
    runningLen: int,
    fp: int,
    top: int,
    depth: int,
    r0: int,
    memRoot: int -> List[CellVal]
  }

  type StepInsn = {
    op: int,
    arg: int,
    tagLink: int
  }

  type StepSlotW = {
    en: bool,
    insn: StepInsn,
    pre: RegState,
    post: RegState,
    busReq: BusTuple,
    isRead: bool,
    readAddr: int,
    readVal: CellVal
  }

  type MemRowW = {
    rowIdx: int,
    addr: int,
    active: bool,
    cv: CellVal
  }

  type PageRowW = {
    active: bool,
    addr: int,
    cv: CellVal
  }

  // Explicit Leaf Close step: transitions from slot[S].pre (runningLen = base + used)
  // to leaf post_d (len = base + C, memRoot with segment base / C inserted)
  type CloseStepW = {
    base: int,
    used: int,
    preClose: RegState,
    segCells: List[CellVal],
    postStatement: RegState
  }

  type LeafW = {
    pre: RegState,
    slots: List[StepSlotW],
    memRows: List[MemRowW],
    pageRows: List[PageRowW],
    closeStep: CloseStepW,
    post: RegState,
    steps: int
  }

  type ChainW = {
    progId: int,
    partId: int,
    nilVal: int,
    leaf1: LeafW,
    leaf2: LeafW
  }

  type WrittenEntry = {
    addr: int,
    cv: CellVal
  }

  type ReadEntry = {
    addr: int,
    cv: CellVal
  }

  type RefOut = {
    written: Set[WrittenEntry],
    reads: List[ReadEntry],
    readErr: bool,
    finalLen: int,
    finalFp: int,
    finalTop: int,
    finalDepth: int,
    finalR0: int
  }

  type RefFoldAcc = {
    out: RefOut,
    curLen: int
  }

  type BuildSlotsAcc = {
    slots: List[StepSlotW],
    curState: RegState,
    writtenMap: int -> CellVal
  }

  // ---------------------------------------------------------------- Pure Helpers
  pure val ZERO_CELL: CellVal = { a: 0, link: 0 }
  pure val PAD_TUPLE: BusTuple = { chip: TAG_PAD, key: 0, a: 0, link: 0 }
  pure val NO_INSN: StepInsn = { op: OP_DISABLED, arg: 0, tagLink: 0 }

  pure val emptyTree: int -> List[CellVal] = {
    val emptySeg: List[CellVal] = []
    SEG_DOM.mapBy(_s => emptySeg)
  }

  pure def initRegState(baseLen: int, nilVal: int): RegState = {
    runningLen: baseLen,
    fp: nilVal,
    top: 0,
    depth: 0,
    r0: 0,
    memRoot: emptyTree
  }

  // Open an address `addr` in a segment tree `tree`:
  // Segment index is `addr / C`, offset within segment is `addr % C`.
  // If the segment slot is empty (length 0), the Merkle path opens to ZERO_CELL.
  pure def openTreeCell(tree: int -> List[CellVal], addr: int): CellVal = {
    val segIdx = addr / C
    val off = addr % C
    if (addr < 0 or not(SEG_DOM.contains(segIdx)))
      ZERO_CELL
    else {
      val seg = tree.get(segIdx)
      if (off >= 0 and off < seg.length()) seg[off] else ZERO_CELL
    }
  }

  pure def updateTreeSeg(tree: int -> List[CellVal], segIdx: int, cells: List[CellVal]): int -> List[CellVal] =
    SEG_DOM.mapBy(s => if (s == segIdx) cells else tree.get(s))

  // Extract the set of non-padding (non-zero) written cells committed in a segment tree
  pure def committedNonZeroCells(tree: int -> List[CellVal]): Set[WrittenEntry] = {
    val emptySet: Set[WrittenEntry] = Set()
    SEG_DOM.fold(emptySet, (acc, segIdx) => {
      val seg = tree.get(segIdx)
      if (seg.length() != C) acc
      else
        range(0, C).foldl(acc, (innerAcc, k) => {
          val c = seg[k]
          if (c != ZERO_CELL) innerAcc.union(Set({ addr: segIdx * C + k, cv: c }))
          else innerAcc
        })
    })
  }

  // Extract all read events across the two leaves
  pure def leafReads(lw: LeafW): List[ReadEntry] = {
    val emptyReads: List[ReadEntry] = []
    lw.slots.foldl(emptyReads, (acc, sl) => {
      if (sl.en and sl.isRead) acc.append({ addr: sl.readAddr, cv: sl.readVal })
      else acc
    })
  }

  pure def chainReads(w: ChainW): List[ReadEntry] =
    leafReads(w.leaf1).concat(leafReads(w.leaf2))

  // ---------------------------------------------------------------- Programs & Leaf Partitions
  // Program 0 (D7 stack push/pop down to depth-0 sentinel):
  //   Step 0: PUSH 10 (spills depth-0 sentinel (0, nilVal) at base+0)
  //   Step 1: PUSH 20 (spills (10, base+0) at base+1)
  //   Step 2: POP     (reads (10, base+0), returns 20)
  //   Step 3: POP     (reads sentinel (0, nilVal) at depth 1, returns 10)
  //
  // Program 1 (Alloc + intra-leaf read + cross-leaf read, including zero-payload data record):
  //   Step 0: ALLOC(a=0, link=1)  (honest record with payload 0 and non-zero tag/link 1)
  //   Step 1: ALLOC(a=25, link=1)
  //   Step 2: LOAD(addr=0)        (reads the zero-payload record at addr 0)
  //   Step 3: LOAD(addr=1)        (reads the record at addr 1)
  pure def progInsn(progId: int, stepIdx: int): StepInsn =
    if (progId == 0) {
      if (stepIdx == 0) { op: OP_PUSH, arg: 10, tagLink: 0 }
      else if (stepIdx == 1) { op: OP_PUSH, arg: 20, tagLink: 0 }
      else if (stepIdx == 2) { op: OP_POP, arg: 0, tagLink: 0 }
      else if (stepIdx == 3) { op: OP_POP, arg: 0, tagLink: 0 }
      else NO_INSN
    } else {
      if (stepIdx == 0) { op: OP_ALLOC, arg: 0, tagLink: 1 }
      else if (stepIdx == 1) { op: OP_ALLOC, arg: 25, tagLink: 1 }
      else if (stepIdx == 2) { op: OP_LOAD, arg: 0, tagLink: 0 }
      else if (stepIdx == 3) { op: OP_LOAD, arg: 1, tagLink: 0 }
      else NO_INSN
    }

  // Leaf partitions of a 4-step program across (Leaf 1, Leaf 2):
  //   partId == 0: [2, 2] (2 steps in Leaf 1, 2 steps in Leaf 2; in Prog 0, Leaf 2 has 0 appends!)
  //   partId == 1: [1, 3] (1 step in Leaf 1, 3 steps in Leaf 2; second append happens in Leaf 2 at addr C!)
  //   partId == 2: [4, 0] (4 steps in Leaf 1, 0 steps in Leaf 2)
  pure def partLeaf1Steps(partId: int): int =
    if (partId == 0) 2
    else if (partId == 1) 1
    else 4

  pure def partLeaf2Steps(partId: int): int =
    if (partId == 0) 2
    else if (partId == 1) 3
    else 0

  // ---------------------------------------------------------------- Reference Abstract Write-Once Memory
  pure def refStepExec(
    st: RefOut,
    curLen: int,
    insn: StepInsn
  ): { nextOut: RefOut, nextLen: int } =
    if (insn.op == OP_DISABLED or st.readErr)
      { nextOut: st, nextLen: curLen }
    else if (insn.op == OP_PUSH) {
      val cell = { a: st.finalTop, link: st.finalFp }
      val nextWritten = st.written.union(Set({ addr: curLen, cv: cell }))
      {
        nextOut: {
          ...st,
          written: nextWritten,
          finalFp: curLen,
          finalTop: insn.arg,
          finalDepth: st.finalDepth + 1
        },
        nextLen: curLen + 1
      }
    } else if (insn.op == OP_ALLOC) {
      val cell = { a: insn.arg, link: insn.tagLink }
      val nextWritten = st.written.union(Set({ addr: curLen, cv: cell }))
      {
        nextOut: {
          ...st,
          written: nextWritten,
          finalR0: curLen
        },
        nextLen: curLen + 1
      }
    } else if (insn.op == OP_POP) {
      val targetAddr = st.finalFp
      val matches = st.written.filter(e => e.addr == targetAddr)
      if (st.finalDepth < 1 or matches.size() == 0) {
        { nextOut: { ...st, readErr: true }, nextLen: curLen }
      } else {
        val loaded = matches.fold(ZERO_CELL, (_acc, e) => e.cv)
        {
          nextOut: {
            ...st,
            reads: st.reads.append({ addr: targetAddr, cv: loaded }),
            finalR0: st.finalTop,
            finalTop: loaded.a,
            finalFp: loaded.link,
            finalDepth: st.finalDepth - 1
          },
          nextLen: curLen
        }
      }
    } else if (insn.op == OP_LOAD) {
      val targetAddr = insn.arg
      val matches = st.written.filter(e => e.addr == targetAddr)
      if (matches.size() == 0) {
        { nextOut: { ...st, readErr: true }, nextLen: curLen }
      } else {
        val loaded = matches.fold(ZERO_CELL, (_acc, e) => e.cv)
        {
          nextOut: {
            ...st,
            reads: st.reads.append({ addr: targetAddr, cv: loaded }),
            finalR0: loaded.a
          },
          nextLen: curLen
        }
      }
    } else {
      { nextOut: st, nextLen: curLen }
    }

  pure def refRunChain(w: ChainW): RefOut = {
    val emptyW: Set[WrittenEntry] = Set()
    val emptyR: List[ReadEntry] = []
    val base1 = w.leaf1.pre.runningLen
    val initOut: RefOut = {
      written: emptyW,
      reads: emptyR,
      readErr: false,
      finalLen: base1,
      finalFp: w.nilVal,
      finalTop: 0,
      finalDepth: 0,
      finalR0: 0
    }
    val initAcc1: RefFoldAcc = { out: initOut, curLen: base1 }
    val afterL1 = w.leaf1.slots.foldl(initAcc1, (acc, sl) => {
      if (not(sl.en)) acc
      else {
        val res = refStepExec(acc.out, acc.curLen, sl.insn)
        { out: res.nextOut, curLen: res.nextLen }
      }
    })
    val base2 = base1 + C
    val initAcc2: RefFoldAcc = { out: afterL1.out, curLen: base2 }
    val afterL2 = w.leaf2.slots.foldl(initAcc2, (acc, sl) => {
      if (not(sl.en)) acc
      else {
        val res = refStepExec(acc.out, acc.curLen, sl.insn)
        { out: res.nextOut, curLen: res.nextLen }
      }
    })
    { ...afterL2.out, finalLen: base2 + C }
  }

  // ---------------------------------------------------------------- Circuit Acceptance Predicates
  // Bus provider sets (D1, D4, D8)
  pure def memProviderBus(lw: LeafW): Set[BusTuple] =
    lw.memRows.foldl(Set(PAD_TUPLE), (bus, r) => {
      if (r.active) bus.union(Set({ chip: TAG_CELL, key: r.addr, a: r.cv.a, link: r.cv.link }))
      else bus
    })

  pure def pageProviderBus(lw: LeafW): Set[BusTuple] =
    lw.pageRows.foldl(Set(PAD_TUPLE), (bus, r) => {
      if (r.active) bus.union(Set({ chip: TAG_CELL, key: r.addr, a: r.cv.a, link: r.cv.link }))
      else bus
    })

  pure def leafProviderBus(lw: LeafW): Set[BusTuple] =
    memProviderBus(lw).union(pageProviderBus(lw))

  pure def leafRequestBus(lw: LeafW): Set[BusTuple] =
    lw.slots.foldl(Set(PAD_TUPLE), (bus, sl) => {
      if (sl.en) bus.union(Set(sl.busReq)) else bus
    })

  // Step slot gate constraints
  pure def slotGateOk(mt: str, sl: StepSlotW): bool =
    if (not(sl.en)) and {
      sl.post == sl.pre,
      sl.busReq == PAD_TUPLE,
      not(sl.isRead),
      sl.readVal == ZERO_CELL
    } else {
      val op = sl.insn.op
      val inc = if (mt == "append_skips") 2 else 1
      // Proposed D3: A read SHALL refuse the zero tuple (dropped in padding_read and page_serves_segment)
      val zeroRefusalOk =
        (mt == "padding_read" or mt == "page_serves_segment")
        or (sl.readVal != ZERO_CELL)

      if (op == OP_PUSH) and {
        not(sl.isRead),
        sl.busReq == { chip: TAG_CELL, key: sl.pre.runningLen, a: sl.pre.top, link: sl.pre.fp },
        sl.post == {
          ...sl.pre,
          runningLen: sl.pre.runningLen + inc,
          fp: sl.pre.runningLen,
          top: sl.insn.arg,
          depth: sl.pre.depth + 1
        }
      } else if (op == OP_ALLOC) and {
        not(sl.isRead),
        sl.busReq == { chip: TAG_CELL, key: sl.pre.runningLen, a: sl.insn.arg, link: sl.insn.tagLink },
        sl.post == {
          ...sl.pre,
          runningLen: sl.pre.runningLen + inc,
          r0: sl.pre.runningLen
        }
      } else if (op == OP_POP) and {
        sl.pre.depth >= 1,
        0 <= sl.pre.fp and sl.pre.fp < sl.pre.runningLen,
        sl.isRead,
        sl.readAddr == sl.pre.fp,
        zeroRefusalOk,
        sl.busReq == { chip: TAG_CELL, key: sl.pre.fp, a: sl.readVal.a, link: sl.readVal.link },
        sl.post == {
          ...sl.pre,
          r0: sl.pre.top,
          top: sl.readVal.a,
          fp: sl.readVal.link,
          depth: sl.pre.depth - 1
        }
      } else if (op == OP_LOAD) and {
        0 <= sl.insn.arg and sl.insn.arg < sl.pre.runningLen,
        sl.isRead,
        sl.readAddr == sl.insn.arg,
        zeroRefusalOk,
        sl.busReq == { chip: TAG_CELL, key: sl.insn.arg, a: sl.readVal.a, link: sl.readVal.link },
        sl.post == {
          ...sl.pre,
          r0: sl.readVal.a
        }
      } else false
    }

  // MEM region constraints (D3, D6, D8):
  // - C rows at addresses base .. base + C - 1
  // - Rows 0 .. used - 1 are active; rows used .. C - 1 are padding and MUST be ZERO_CELL
  pure def memRegionOk(mt: str, lw: LeafW): bool = {
    val base = lw.pre.runningLen
    val endLen = lw.slots[S - 1].post.runningLen
    val used = endLen - base
    and {
      lw.memRows.length() == C,
      0 <= used and used <= C,
      0.to(C - 1).forall(k => {
        val r = lw.memRows[k]
        and {
          r.rowIdx == k,
          r.addr == base + k,
          r.active == (k < used),
          // M1 `padding_nonzero`: drops the zero-tuple constraint on padding rows (k >= used)
          (mt == "padding_nonzero") or (r.active or r.cv == ZERO_CELL)
        }
      })
    }
  }

  // PAGE region constraints (D2):
  // - Serves TAG_CELL only for 0 <= addr < base (dropped in page_serves_segment and page_serves_segment_alone)
  // - Authenticates against pre_d.mem_root by opening segment addr / C, offset addr % C
  pure def pageRegionOk(mt: str, lw: LeafW): bool = {
    val base = lw.pre.runningLen
    lw.pageRows.indices().forall(pi => { val r = lw.pageRows[pi]
      not(r.active) or and {
        r.addr >= 0,
        (mt == "page_serves_segment" or mt == "page_serves_segment_alone") or (r.addr < base),
        r.cv == openTreeCell(lw.pre.memRoot, r.addr)
      }
    })
  }

  // Explicit Leaf Close step constraints (M3 aligned segment root update & aligned len close):
  // - base is a multiple of C (dropped in M5 `base_unaligned`)
  // - proves slot base / C empty in pre_d.mem_root and preserves siblings s != base / C (dropped in M6 `no_empty_slot_check`)
  // - hashes all C rows of MEM into segment base / C of post_d.mem_root
  // - sets post_d.len := base + C (in M4 `close_len_running`, sets post_d.len := base + used)
  pure def leafCloseOk(mt: str, lw: LeafW): bool = {
    val cs = lw.closeStep
    val base = lw.pre.runningLen
    val preEnd = lw.slots[S - 1].post
    val used = preEnd.runningLen - base
    val segIdx = base / C
    val emptyCells: List[CellVal] = []
    val expectedSegCells = lw.memRows.foldl(emptyCells, (acc, r) => acc.append(r.cv))
    val expectedLen = if (mt == "close_len_running") (base + used) else (base + C)
    and {
      cs.base == base,
      cs.used == used,
      cs.preClose == preEnd,
      cs.segCells == expectedSegCells,
      // M5 `base_unaligned`: drops alignment check base % C == 0
      (mt == "base_unaligned") or (base % C == 0),
      SEG_DOM.contains(segIdx),
      // M6 `no_empty_slot_check`: drops empty-slot proof at segIdx and sibling preservation at s != segIdx
      (mt == "no_empty_slot_check") or and {
        lw.pre.memRoot.get(segIdx).length() == 0,
        SEG_DOM.forall(s => s == segIdx or cs.postStatement.memRoot.get(s) == lw.pre.memRoot.get(s))
      },
      cs.postStatement.memRoot.get(segIdx) == expectedSegCells,
      cs.postStatement.runningLen == expectedLen,
      cs.postStatement.fp == preEnd.fp,
      cs.postStatement.top == preEnd.top,
      cs.postStatement.depth == preEnd.depth,
      cs.postStatement.r0 == preEnd.r0,
      lw.post == cs.postStatement
    }
  }

  pure def leafAccepted(mt: str, lw: LeafW): bool = and {
    lw.slots.length() == S,
    lw.slots[0].pre == lw.pre,
    0.to(S - 2).forall(i => {
      val enCur = lw.slots[i].en
      val enNext = lw.slots[i + 1].en
      and {
        not(enNext) or enCur,
        lw.slots[i].post == lw.slots[i + 1].pre
      }
    }),
    0.to(S - 1).forall(i => slotGateOk(mt, lw.slots[i])),
    lw.steps == lw.slots.foldl(0, (cnt, sl) => cnt + (if (sl.en) 1 else 0)),
    memRegionOk(mt, lw),
    pageRegionOk(mt, lw),
    leafRequestBus(lw).subseteq(leafProviderBus(lw)),
    leafCloseOk(mt, lw)
  }

  pure def genesisOk(mt: str, w: ChainW): bool = {
    val expectedBase0 = if (mt == "base_unaligned" and w.leaf1.pre.runningLen == 1) 1 else 0
    and {
      w.leaf1.pre == initRegState(expectedBase0, w.nilVal),
      (mt == "base_unaligned") or (w.leaf1.pre.runningLen == 0)
    }
  }

  pure def acceptsWithMutant(mt: str, w: ChainW): bool = and {
    genesisOk(mt, w),
    leafAccepted(mt, w.leaf1),
    w.leaf2.pre == w.leaf1.post,
    leafAccepted(mt, w.leaf2)
  }

  pure def accepts(w: ChainW): bool =
    acceptsWithMutant(MUTANT, w)

  // ---------------------------------------------------------------- Main Refinement Invariant (a), (b), (c)
  // Synchronization predicate (c) on a leaf post statement:
  //   - every address 0 <= a < len lies in a segment that mem_root holds (length == C), and
  //   - no segment at or above len / C is filled (length == 0).
  pure def lenRootSynced(st: RegState): bool = and {
    st.runningLen >= 0,
    (st.runningLen == 0) or 0.to(st.runningLen - 1).forall(a => {
      val segIdx = a / C
      SEG_DOM.contains(segIdx) and st.memRoot.get(segIdx).length() == C
    }),
    SEG_DOM.forall(s => {
      if (s >= st.runningLen / C) st.memRoot.get(s).length() == 0
      else true
    })
  }

  pure def refines(w: ChainW): bool = {
    val ref = refRunChain(w)
    val cirCommitted = committedNonZeroCells(w.leaf2.post.memRoot)
    val cirReads = chainReads(w)
    and {
      // (a) The set of cells committed in post_d.mem_root (excluding padding) equals the reference's written map
      cirCommitted == ref.written,
      // (b) Every accepted read is a valid read of a written address and returns its reference value
      not(ref.readErr),
      cirReads == ref.reads,
      // (c) post_d.len and post_d.mem_root stay in sync after Leaf 1 and after Leaf 2
      lenRootSynced(w.leaf1.post),
      lenRootSynced(w.leaf2.post)
    }
  }

  // ---------------------------------------------------------------- Witness Builders & Nondeterministic Tamper Prover
  pure def putWritten(m: int -> CellVal, addr: int, v: CellVal): int -> CellVal =
    0.to(4 * C - 1).mapBy(a => if (a == addr) v else m.get(a))

  pure val emptyWrittenMap: int -> CellVal =
    0.to(4 * C - 1).mapBy(_a => ZERO_CELL)

  pure def buildHonestSlot(
    mt: str,
    pre: RegState,
    insn: StepInsn,
    slotOn: bool,
    wMap: int -> CellVal
  ): { slot: StepSlotW, nextMap: int -> CellVal } =
    if (not(slotOn) or insn.op == OP_DISABLED) {
      val sl = {
        en: false,
        insn: NO_INSN,
        pre: pre,
        post: pre,
        busReq: PAD_TUPLE,
        isRead: false,
        readAddr: 0,
        readVal: ZERO_CELL
      }
      { slot: sl, nextMap: wMap }
    } else {
      val inc = if (mt == "append_skips") 2 else 1
      if (insn.op == OP_PUSH) {
        val cell = { a: pre.top, link: pre.fp }
        val post = {
          ...pre,
          runningLen: pre.runningLen + inc,
          fp: pre.runningLen,
          top: insn.arg,
          depth: pre.depth + 1
        }
        val sl = {
          en: true,
          insn: insn,
          pre: pre,
          post: post,
          busReq: { chip: TAG_CELL, key: pre.runningLen, a: cell.a, link: cell.link },
          isRead: false,
          readAddr: 0,
          readVal: ZERO_CELL
        }
        { slot: sl, nextMap: putWritten(wMap, pre.runningLen, cell) }
      } else if (insn.op == OP_ALLOC) {
        val cell = { a: insn.arg, link: insn.tagLink }
        val post = {
          ...pre,
          runningLen: pre.runningLen + inc,
          r0: pre.runningLen
        }
        val sl = {
          en: true,
          insn: insn,
          pre: pre,
          post: post,
          busReq: { chip: TAG_CELL, key: pre.runningLen, a: cell.a, link: cell.link },
          isRead: false,
          readAddr: 0,
          readVal: ZERO_CELL
        }
        { slot: sl, nextMap: putWritten(wMap, pre.runningLen, cell) }
      } else if (insn.op == OP_POP) {
        val rAddr = pre.fp
        val cell = if (rAddr < pre.runningLen and rAddr >= 0) wMap.get(rAddr) else ZERO_CELL
        val post = {
          ...pre,
          r0: pre.top,
          top: cell.a,
          fp: cell.link,
          depth: pre.depth - 1
        }
        val sl = {
          en: true,
          insn: insn,
          pre: pre,
          post: post,
          busReq: { chip: TAG_CELL, key: rAddr, a: cell.a, link: cell.link },
          isRead: true,
          readAddr: rAddr,
          readVal: cell
        }
        { slot: sl, nextMap: wMap }
      } else {
        // OP_LOAD
        val rAddr = insn.arg
        val cell = if (rAddr < pre.runningLen and rAddr >= 0) wMap.get(rAddr) else ZERO_CELL
        val post = {
          ...pre,
          r0: cell.a
        }
        val sl = {
          en: true,
          insn: insn,
          pre: pre,
          post: post,
          busReq: { chip: TAG_CELL, key: rAddr, a: cell.a, link: cell.link },
          isRead: true,
          readAddr: rAddr,
          readVal: cell
        }
        { slot: sl, nextMap: wMap }
      }
    }

  pure def buildLeafFromInsns(
    mt: str,
    pre: RegState,
    insns: List[StepInsn],
    nEnabled: int,
    wMap: int -> CellVal,
    padOverrideRow: int,
    padOverrideVal: CellVal,
    siblingForge: bool
  ): { leaf: LeafW, nextMap: int -> CellVal } = {
    val emptySlots: List[StepSlotW] = []
    val initAcc: BuildSlotsAcc = { slots: emptySlots, curState: pre, writtenMap: wMap }
    val folded = range(0, S).foldl(initAcc, (acc, i) => {
      val en = i < nEnabled
      val insn = if (en and i < insns.length()) insns[i] else NO_INSN
      val built = buildHonestSlot(mt, acc.curState, insn, en, acc.writtenMap)
      {
        slots: acc.slots.append(built.slot),
        curState: built.slot.post,
        writtenMap: built.nextMap
      }
    })
    val base = pre.runningLen
    val preEnd = folded.curState
    val used = preEnd.runningLen - base
    val emptyMem: List[MemRowW] = []
    val memRows = range(0, C).foldl(emptyMem, (rows, k) => {
      val addr = base + k
      val act = k < used
      val honestCell = if (act) folded.writtenMap.get(addr) else ZERO_CELL
      val cell = if (k == padOverrideRow) padOverrideVal else honestCell
      rows.append({
        rowIdx: k,
        addr: addr,
        active: act,
        cv: cell
      })
    })
    val emptyPage: List[PageRowW] = []
    val pageRows = folded.slots.foldl(emptyPage, (rows, sl) => {
      val allowSegRead = (mt == "page_serves_segment" or mt == "page_serves_segment_alone")
      if (sl.en and sl.isRead and (sl.readAddr < base or allowSegRead))
        rows.append({
          active: true,
          addr: sl.readAddr,
          cv: openTreeCell(pre.memRoot, sl.readAddr)
        })
      else rows
    })
    val emptyCells: List[CellVal] = []
    val segCells = memRows.foldl(emptyCells, (acc, r) => acc.append(r.cv))
    val segIdx = base / C
    val tree1 = updateTreeSeg(pre.memRoot, segIdx, segCells)
    val forgedSeg0: List[CellVal] = range(0, C).foldl(emptyCells, (acc, k) =>
      acc.append(if (k == 0) { a: 77, link: 99 } else ZERO_CELL)
    )
    val postTree = if (siblingForge and segIdx > 0) updateTreeSeg(tree1, 0, forgedSeg0) else tree1
    val closedLen = if (mt == "close_len_running") (base + used) else (base + C)
    val postState = {
      ...preEnd,
      runningLen: closedLen,
      memRoot: postTree
    }
    val cs: CloseStepW = {
      base: base,
      used: used,
      preClose: preEnd,
      segCells: segCells,
      postStatement: postState
    }
    val syncedMap = range(0, C).foldl(folded.writtenMap, (m, k) =>
      putWritten(m, segIdx * C + k, segCells[k])
    )
    val lw: LeafW = {
      pre: pre,
      slots: folded.slots,
      memRows: memRows,
      pageRows: pageRows,
      closeStep: cs,
      post: postState,
      steps: folded.slots.foldl(0, (cnt, sl) => cnt + (if (sl.en) 1 else 0))
    }
    { leaf: lw, nextMap: syncedMap }
  }

  pure def buildHonestChain(mt: str, progId: int, partId: int, nilVal: int): ChainW = {
    val n1 = partLeaf1Steps(partId)
    val n2 = partLeaf2Steps(partId)
    val emptyInsns: List[StepInsn] = []
    val insns1 = range(0, S).foldl(emptyInsns, (acc, i) =>
      acc.append(if (i < n1) progInsn(progId, i) else NO_INSN)
    )
    val insns2 = range(0, S).foldl(emptyInsns, (acc, i) =>
      acc.append(if (i < n2) progInsn(progId, n1 + i) else NO_INSN)
    )
    val pre1 = initRegState(0, nilVal)
    val b1 = buildLeafFromInsns(mt, pre1, insns1, n1, emptyWrittenMap, -1, ZERO_CELL, false)
    val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, n2, b1.nextMap, -1, ZERO_CELL, false)
    {
      progId: progId,
      partId: partId,
      nilVal: nilVal,
      leaf1: b1.leaf,
      leaf2: b2.leaf
    }
  }

  // Unified Tamper Operator for Nondeterministic Prover & Exhaustive Pure Checks:
  //   tKind == 0: Honest chain (also triggers M4 `close_len_running` on partition [4, 0] where Leaf 1 has used = 2 < C)
  //   tKind == 1: Put non-zero cell `(newVal, 1)` into MEM row `rowIdx` of Leaf 1 -> triggers M1 `padding_nonzero`
  //               and M2 `append_skips` (putting `(newVal, 1)` in skipped active hole `rowIdx = 1`)
  //   tKind == 2: Read unwritten padding address `base1 + 2` in Leaf 2 slot 0 -> triggers M3 `padding_read`
  //   tKind == 3: Start Leaf 1 at unaligned `base = 1` (as in rf.rs) and read `addr = 1` in Leaf 2 -> triggers M5 `base_unaligned`
  //   tKind == 4: Forge sibling segment 0 in Leaf 2's `post_d.mem_root` -> triggers M6 `no_empty_slot_check`
  //   tKind == 5: Intra-leaf read of `addr = 0` in Leaf 1 served by PAGE `(0, 0)` instead of MEM -> triggers M7 `page_serves_segment`
  //   tKind == 6: Leaf 1 fills all C = 4 rows (`used = C`) and Leaf 2 appends 2 rows (`used = 2 < C`), closing with
  //               running `len = C + 2` instead of `2 * C` -> triggers M4 `close_len_running` cleanly across both leaves
  pure def applyTamper(
    mt: str,
    progId: int,
    partId: int,
    tKind: int,
    rowIdx: int,
    newVal: int
  ): ChainW = {
    if (tKind == 0) {
      buildHonestChain(mt, progId, partId, NIL_DEFAULT)
    } else if (tKind == 1) {
      val n1 = partLeaf1Steps(partId)
      val n2 = partLeaf2Steps(partId)
      val emptyInsns: List[StepInsn] = []
      val insns1 = range(0, S).foldl(emptyInsns, (acc, i) =>
        acc.append(if (i < n1) progInsn(progId, i) else NO_INSN)
      )
      val insns2 = range(0, S).foldl(emptyInsns, (acc, i) =>
        acc.append(if (i < n2) progInsn(progId, n1 + i) else NO_INSN)
      )
      val pre1 = initRegState(0, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, n1, emptyWrittenMap, rowIdx, { a: newVal, link: 1 }, false)
      val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, n2, b1.nextMap, -1, ZERO_CELL, false)
      { progId: progId, partId: partId, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
    } else if (tKind == 2) {
      // M3 `padding_read`: Leaf 1 appends 2 cells (0, 1); Leaf 2 reads padding address 2 via PAGE
      val insns1 = [
        { op: OP_ALLOC, arg: 10, tagLink: 1 },
        { op: OP_ALLOC, arg: 20, tagLink: 1 },
        NO_INSN,
        NO_INSN
      ]
      val insns2 = [
        { op: OP_LOAD, arg: 2, tagLink: 0 },
        NO_INSN,
        NO_INSN,
        NO_INSN
      ]
      val pre1 = initRegState(0, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, 2, emptyWrittenMap, -1, ZERO_CELL, false)
      val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, 1, b1.nextMap, -1, ZERO_CELL, false)
      { progId: 1, partId: 0, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
    } else if (tKind == 3) {
      // M5 `base_unaligned`: Genesis starts at base = 1 (not a multiple of C);
      // Leaf 1 appends (10, 1) at addr=1 and (20, 1) at addr=2; Leaf 2 reads addr=1 via PAGE (which opens offset 1%C=1 -> (20, 1)!)
      val insns1 = [
        { op: OP_ALLOC, arg: 10, tagLink: 1 },
        { op: OP_ALLOC, arg: 20, tagLink: 1 },
        NO_INSN,
        NO_INSN
      ]
      val insns2 = [
        { op: OP_LOAD, arg: 1, tagLink: 0 },
        NO_INSN,
        NO_INSN,
        NO_INSN
      ]
      val pre1 = initRegState(1, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, 2, emptyWrittenMap, -1, ZERO_CELL, false)
      val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, 1, b1.nextMap, -1, ZERO_CELL, false)
      { progId: 1, partId: 0, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
    } else if (tKind == 4) {
      // M6 `no_empty_slot_check`: Leaf 2 replaces sibling segment 0 in post_d.mem_root with forged cells
      val n1 = partLeaf1Steps(0)
      val n2 = partLeaf2Steps(0)
      val emptyInsns: List[StepInsn] = []
      val insns1 = range(0, S).foldl(emptyInsns, (acc, i) =>
        acc.append(if (i < n1) progInsn(progId, i) else NO_INSN)
      )
      val insns2 = range(0, S).foldl(emptyInsns, (acc, i) =>
        acc.append(if (i < n2) progInsn(progId, n1 + i) else NO_INSN)
      )
      val pre1 = initRegState(0, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, n1, emptyWrittenMap, -1, ZERO_CELL, false)
      val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, n2, b1.nextMap, -1, ZERO_CELL, true)
      { progId: progId, partId: 0, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
    } else if (tKind == 5) {
      // M7 `page_serves_segment` (combined with M3):
      // Leaf 1 Step 0 appends (10, 1) at addr=0; Leaf 1 Step 1 reads addr=0 via PAGE (getting stale ZERO_CELL from pre_d.mem_root)
      val insns1 = [
        { op: OP_ALLOC, arg: 10, tagLink: 1 },
        { op: OP_LOAD, arg: 0, tagLink: 0 },
        NO_INSN,
        NO_INSN
      ]
      val insns2 = [
        { op: OP_ALLOC, arg: 20, tagLink: 1 },
        NO_INSN,
        NO_INSN,
        NO_INSN
      ]
      val pre1 = initRegState(0, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, 2, emptyWrittenMap, -1, ZERO_CELL, false)
      val sl0 = b1.leaf.slots[0]
      val sl1Orig = b1.leaf.slots[1]
      val sl1Stale: StepSlotW = {
        ...sl1Orig,
        post: { ...sl1Orig.pre, r0: 0 },
        busReq: { chip: TAG_CELL, key: 0, a: 0, link: 0 },
        readVal: ZERO_CELL
      }
      val sl2 = { ...b1.leaf.slots[2], pre: sl1Stale.post, post: sl1Stale.post }
      val sl3 = { ...b1.leaf.slots[3], pre: sl1Stale.post, post: sl1Stale.post }
      val post1 = { ...b1.leaf.post, r0: 0 }
      val cs1 = { ...b1.leaf.closeStep, preClose: sl1Stale.post, postStatement: post1 }
      val l1Tampered: LeafW = {
        ...b1.leaf,
        slots: [sl0, sl1Stale, sl2, sl3],
        pageRows: [{ active: true, addr: 0, cv: ZERO_CELL }],
        closeStep: cs1,
        post: post1
      }
      val b2 = buildLeafFromInsns(mt, post1, insns2, 1, b1.nextMap, -1, ZERO_CELL, false)
      { progId: 1, partId: 0, nilVal: NIL_DEFAULT, leaf1: l1Tampered, leaf2: b2.leaf }
    } else {
      // tKind == 6: M4 `close_len_running`: Leaf 1 appends C = 4 cells (so base2 = 4 is aligned),
      // and Leaf 2 appends 2 < C cells, closing with post_d.len = 4 + 2 = 6 instead of 8!
      val insns1 = [
        { op: OP_ALLOC, arg: 10, tagLink: 1 },
        { op: OP_ALLOC, arg: 20, tagLink: 1 },
        { op: OP_ALLOC, arg: 30, tagLink: 1 },
        { op: OP_ALLOC, arg: 40, tagLink: 1 }
      ]
      val insns2 = [
        { op: OP_ALLOC, arg: 50, tagLink: 1 },
        { op: OP_ALLOC, arg: 60, tagLink: 1 },
        NO_INSN,
        NO_INSN
      ]
      val pre1 = initRegState(0, NIL_DEFAULT)
      val b1 = buildLeafFromInsns(mt, pre1, insns1, 4, emptyWrittenMap, -1, ZERO_CELL, false)
      val b2 = buildLeafFromInsns(mt, b1.leaf.post, insns2, 2, b1.nextMap, -1, ZERO_CELL, false)
      { progId: 1, partId: 0, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
    }
  }

  // ---------------------------------------------------------------- Named Honest & Counterexample Witnesses
  // 1. Completeness: POP at depth 1 loading the D7 sentinel spill (0, NIL) with NIL = 99 != 0
  pure val hw_d7_sentinel_pop: ChainW = buildHonestChain("none", 0, 0, NIL_DEFAULT)

  // 2. Completeness failure under NIL = 0 (rf.rs): POP at depth 1 loads (0, 0) and is rejected by D3 zero-tuple refusal
  pure val w_d7_sentinel_nil_zero: ChainW = buildHonestChain("none", 0, 0, 0)

  // 3. Completeness: Honest data record with payload 0 and non-zero tag/link = 1 (`(0, 1) != (0, 0)`)
  pure val hw_zero_payload_tagged: ChainW = buildHonestChain("none", 1, 0, NIL_DEFAULT)

  // 4. Completeness failure when an honest data record is all-zero `(0, 0)` (tagLink = 0) and read back
  pure val buildUntaggedZeroRecordChain: ChainW = {
    val insns1 = [
      { op: OP_ALLOC, arg: 0, tagLink: 0 },
      { op: OP_LOAD, arg: 0, tagLink: 0 },
      NO_INSN,
      NO_INSN
    ]
    val insns2 = [
      { op: OP_ALLOC, arg: 10, tagLink: 1 },
      NO_INSN,
      NO_INSN,
      NO_INSN
    ]
    val pre1 = initRegState(0, NIL_DEFAULT)
    val b1 = buildLeafFromInsns("none", pre1, insns1, 2, emptyWrittenMap, -1, ZERO_CELL, false)
    val b2 = buildLeafFromInsns("none", b1.leaf.post, insns2, 1, b1.nextMap, -1, ZERO_CELL, false)
    { progId: 1, partId: 0, nilVal: NIL_DEFAULT, leaf1: b1.leaf, leaf2: b2.leaf }
  }
  pure val w_zero_record_untagged: ChainW = buildUntaggedZeroRecordChain

  // 5. Same program (Prog 0) under three different leaf partitions:
  //    - Partition [2, 2]: 2 appends in Leaf 1, 2 POPs (0 appends!) in Leaf 2
  //    - Partition [1, 3]: 1 append in Leaf 1, 1 append + 2 POPs in Leaf 2 (second frame written at addr = C!)
  //    - Partition [4, 0]: all 4 steps in Leaf 1, 0 steps (0 appends!) in Leaf 2
  pure val hw_part_2_2: ChainW = buildHonestChain("none", 0, 0, NIL_DEFAULT)
  pure val hw_part_1_3: ChainW = buildHonestChain("none", 0, 1, NIL_DEFAULT)
  pure val hw_part_4_0: ChainW = buildHonestChain("none", 0, 2, NIL_DEFAULT)

  // Required Mutant Witnesses M1..M7:
  // M1 `padding_nonzero`: padding row k = 2 in Leaf 1 holds non-zero (3, 1)
  pure val cex_M1_padding_nonzero: ChainW =
    applyTamper("padding_nonzero", 0, 0, 1, 2, 3)

  // M2 `append_skips`: append advances runningLen by 2, leaving active hole at row k = 1 filled with (3, 1)
  pure val cex_M2_append_skips: ChainW =
    applyTamper("append_skips", 0, 0, 1, 1, 3)

  // M3 `padding_read`: Leaf 2 reads unwritten padding cell at addr = 2 via PAGE
  pure val cex_M3_padding_read: ChainW =
    applyTamper("padding_read", 1, 0, 2, 0, 0)

  // M4 `close_len_running`: Leaf 2 close sets post_d.len = base + used (6) instead of base + C (8)
  pure val cex_M4_close_len_running: ChainW =
    applyTamper("close_len_running", 1, 0, 6, 0, 0)

  // M5 `base_unaligned`: Genesis starts at base = 1 (not a multiple of C); PAGE offset addr % C is shifted
  pure val cex_M5_base_unaligned: ChainW =
    applyTamper("base_unaligned", 1, 0, 3, 0, 0)

  // M6 `no_empty_slot_check`: Siblings unauthenticated on root update; Leaf 2 modifies segment 0 in post_d.mem_root
  pure val cex_M6_no_empty_slot_check: ChainW =
    applyTamper("no_empty_slot_check", 0, 0, 4, 0, 0)

  // M7 `page_serves_segment`: D2's addr < base dropped (combined with M3); intra-leaf read served stale (0, 0) by PAGE
  pure val cex_M7_page_serves_segment: ChainW =
    applyTamper("page_serves_segment", 1, 0, 5, 0, 0)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val PROG_IDS: Set[int] = Set(0, 1)
  pure val PART_IDS: Set[int] = Set(0, 1, 2)
  pure val TAMPER_KINDS: Set[int] = Set(0, 1, 2, 3, 4, 5, 6)
  pure val ROW_DOM: Set[int] = Set(0, 1, 2, 3)
  pure val VAL_DOM: Set[int] = Set(0, 1, 2, 3)

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
    val honest = buildHonestChain("none", progId, partId, NIL_DEFAULT)
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

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 on m3segSound)
  // 1. Honest leaf with padding rows (used = 2 < C in Leaf 1, used = 0 < C in Leaf 2)
  val reachHonestWithPadding: bool =
    not(acc and not(isTampered) and w.leaf1.closeStep.used == 2 and w.leaf2.closeStep.used == 0)

  // 2. Honest cross-leaf PAGE read (Leaf 2 reads a cell written in Leaf 1 via PAGE)
  val reachCrossLeafPageRead: bool =
    not(acc and not(isTampered) and w.leaf2.pageRows.length() >= 1)

  // 3. Accepted bad tamper (violated ONLY in mutants M1..M7, holds in m3segSound)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// Sound base modules (C = 4 and C = 8)
module m3segSound { import m3seg(S = 4, C = 4, MUTANT = "none").* }
module m3segSoundC8 { import m3seg(S = 4, C = 8, MUTANT = "none").* }

// Required mutants M1..M7 (each MUST violate invRefine)
module m3seg_M1_padding_nonzero { import m3seg(S = 4, C = 4, MUTANT = "padding_nonzero").* }
module m3seg_M2_append_skips { import m3seg(S = 4, C = 4, MUTANT = "append_skips").* }
module m3seg_M3_padding_read { import m3seg(S = 4, C = 4, MUTANT = "padding_read").* }
module m3seg_M4_close_len_running { import m3seg(S = 4, C = 4, MUTANT = "close_len_running").* }
module m3seg_M5_base_unaligned { import m3seg(S = 4, C = 4, MUTANT = "base_unaligned").* }
module m3seg_M6_no_empty_slot_check { import m3seg(S = 4, C = 4, MUTANT = "no_empty_slot_check").* }
module m3seg_M7_page_serves_segment { import m3seg(S = 4, C = 4, MUTANT = "page_serves_segment").* }

// M7 alone (D2's addr < base dropped while zero-tuple refusal and non-zero writes remain active; NOT caught alone)
module m3seg_M7_alone { import m3seg(S = 4, C = 4, MUTANT = "page_serves_segment_alone").* }
