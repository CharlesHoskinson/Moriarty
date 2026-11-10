// Soundness Model of Design Point D7: The Cached Frame Protocol Across Chained Leaves
// Design under test:
//   - openspec/changes/sherlock-vm-consensus-20261107/design.md section 2.7 & 2.7.8 (D1..D8)
//   - research/sherlock-processor-analogy-20261110/study.md
//   - Previous model findings F12 & F13: qa/gemini-quint/chipleaf/REPORT.md
//
// Models:
//   - CORE step relation for the small ISA (LIT, PUSH, POP, REWRITE, HALT)
//   - Cached frame registers: top, fp (frame below top), depth, len, memRoot
//   - Sentinel at depth 0: top = 0, fp = NIL (-1); spilled by PUSH at depth 0, reloaded by POP at depth 1
//   - Single lookup_any set-inclusion bus (no multiplicities) with D1..D8 applied
//   - MEM (append-only log in current leaf) and PAGE (authenticated pre-leaf cells and PROG rows)
//   - Two chained leaves of S slots each (leaf 2 starts from leaf 1's post_d and committed memory)
//   - Reference stack machine with an abstract list of values
//   - Main invariant (invFrameStackRefinement): POP result sequences, final depth, and final top agree
//   - 4 required mutants + 4 additional cached-protocol mutants, each caught with a concrete witness

module cached {
  const S: int
  const MUTANT: str

  // ---------------------------------------------------------------- Capacities & Constants
  pure val N_MEM: int = 4
  pure val N_PAGE: int = 8
  pure val NIL: int = -1
  pure val INIT_MEM_ROOT: int = 9001

  // Fixed chip tags for the 5-column bus (D4: fixed per region; D8: no chip tag is 0)
  pure val TAG_PAD: int = 0
  pure val TAG_PROG: int = 1
  pure val TAG_CELL: int = 2
  pure val TAG_DEC: int = 3

  // Small ISA opcodes (0 reserved for disabled padding slots)
  pure val OP_DISABLED: int = 0
  pure val OP_LIT: int = 1
  pure val OP_PUSH: int = 2
  pure val OP_POP: int = 3
  pure val OP_REWRITE: int = 4
  pure val OP_HALT: int = 5

  pure val VALID_OPS: Set[int] = Set(1, 2, 3, 4, 5)
  pure val ADDR_DOM: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7)
  pure val MEM_INDICES: List[int] = [0, 1, 2, 3]

  // ---------------------------------------------------------------- Types
  type BusTuple = {
    chip: int,
    key: int,
    a: int,
    b: int,
    c: int
  }

  type Insn = {
    op: int,
    imm: int
  }

  type MemCell = {
    valField: int,
    linkField: int
  }

  // CORE register file carried in leaf statement pre_d / post_d
  type CoreRegs = {
    pc: int,
    acc: int,
    top: int,
    fp: int,
    depth: int,
    len: int,
    memRoot: int,
    halted: bool
  }

  type StepSlot = {
    en: bool,
    op: int,
    imm: int,
    pre: CoreRegs,
    post: CoreRegs,
    decReq: BusTuple,
    progReq: BusTuple,
    cellReq: BusTuple,
    popped: bool,
    popVal: int
  }

  type MemRow = {
    active: bool,
    rowIdx: int,
    addr: int,
    a: int,
    b: int,
    c: int
  }

  type PageRow = {
    active: bool,
    chipTag: int,
    idx: int,
    a: int,
    b: int,
    c: int,
    rootUsed: int,
    pathValid: bool
  }

  type LeafWitness = {
    slots: List[StepSlot],
    memTable: List[MemRow],
    pageTable: List[PageRow],
    steps: int,
    pre: CoreRegs,
    post: CoreRegs
  }

  type ChainWitness = {
    progId: int,
    leaf1: LeafWitness,
    leaf2: LeafWitness
  }

  type BuiltStep = {
    slot: StepSlot,
    cellsAfter: int -> MemCell
  }

  type BuiltLeaf = {
    leaf: LeafWitness,
    cellsAfter: int -> MemCell
  }

  type FoldSlotsAcc = {
    slots: List[StepSlot],
    cells: int -> MemCell
  }

  type RefState = {
    pc: int,
    acc: int,
    stack: List[int],
    popLog: List[int],
    halted: bool,
    err: bool
  }

  // ---------------------------------------------------------------- Pure Helpers & Constants
  // D8: Disabled and non-requesting rows put (0, 0, 0, 0, 0) on the bus; no chip tag is 0
  pure val PAD_TUPLE: BusTuple = { chip: TAG_PAD, key: 0, a: 0, b: 0, c: 0 }

  pure val initCoreRegs: CoreRegs = {
    pc: 0,
    acc: 0,
    top: 0,
    fp: NIL,
    depth: 0,
    len: 0,
    memRoot: INIT_MEM_ROOT,
    halted: false
  }

  pure def withTop(r: CoreRegs, newTop: int): CoreRegs = {
    pc: r.pc,
    acc: r.acc,
    top: newTop,
    fp: r.fp,
    depth: r.depth,
    len: r.len,
    memRoot: r.memRoot,
    halted: r.halted
  }

  pure def withDepth(r: CoreRegs, newDepth: int): CoreRegs = {
    pc: r.pc,
    acc: r.acc,
    top: r.top,
    fp: r.fp,
    depth: newDepth,
    len: r.len,
    memRoot: r.memRoot,
    halted: r.halted
  }

  pure val emptyCells: int -> MemCell =
    ADDR_DOM.mapBy(a => { valField: 0, linkField: 0 })

  pure def putCell(cells: int -> MemCell, writeAddr: int, cellVal: MemCell): int -> MemCell =
    ADDR_DOM.mapBy(a => if (a == writeAddr) cellVal else cells.get(a))

  pure def foldMemRoot(prevRoot: int, addr: int, valA: int, linkB: int): int =
    prevRoot * 131 + addr * 1009 + valA * 37 + (linkB + 5) * 7 + 19

  // Fixed DEC table: maps each valid opcode to its control tuple
  pure def decTuple(op: int): BusTuple = {
    chip: TAG_DEC,
    key: op,
    a: op,
    b: if (op == OP_PUSH) 1 else if (op == OP_POP) 2 else 0,
    c: if (op == OP_HALT) 1 else 0
  }

  pure val DEC_ROM: Set[BusTuple] =
    VALID_OPS.fold(Set(PAD_TUPLE), (accSet, op) => accSet.union(Set(decTuple(op))))

  // ---------------------------------------------------------------- Programs Under Test
  // Every program pushes at least 3 frames with distinct values in Leaf 1 (pc = 0..3)
  // and pops at least 3 frames in Leaf 2 (pc = 4..7).
  pure def fetchInsn(progId: int, pc: int): Insn =
    if (progId == 0) {
      // Program 0: PUSH 10, PUSH 20, PUSH 30, LIT 99 | POP, POP, POP, HALT
      if (pc == 0) { op: OP_PUSH, imm: 10 }
      else if (pc == 1) { op: OP_PUSH, imm: 20 }
      else if (pc == 2) { op: OP_PUSH, imm: 30 }
      else if (pc == 3) { op: OP_LIT, imm: 99 }
      else if (pc == 4) { op: OP_POP, imm: 0 }
      else if (pc == 5) { op: OP_POP, imm: 0 }
      else if (pc == 6) { op: OP_POP, imm: 0 }
      else { op: OP_HALT, imm: 0 }
    } else if (progId == 1) {
      // Program 1: PUSH 100, PUSH 200, REWRITE 250, PUSH 300 | POP, POP, POP, HALT
      if (pc == 0) { op: OP_PUSH, imm: 100 }
      else if (pc == 1) { op: OP_PUSH, imm: 200 }
      else if (pc == 2) { op: OP_REWRITE, imm: 250 }
      else if (pc == 3) { op: OP_PUSH, imm: 300 }
      else if (pc == 4) { op: OP_POP, imm: 0 }
      else if (pc == 5) { op: OP_POP, imm: 0 }
      else if (pc == 6) { op: OP_POP, imm: 0 }
      else { op: OP_HALT, imm: 0 }
    } else if (progId == 2) {
      // Program 2: PUSH 11, PUSH 22, PUSH 33, PUSH 44 | REWRITE 45, POP, POP, POP
      // Leaves 1 frame (11) on the stack at depth = 1, top = 11.
      if (pc == 0) { op: OP_PUSH, imm: 11 }
      else if (pc == 1) { op: OP_PUSH, imm: 22 }
      else if (pc == 2) { op: OP_PUSH, imm: 33 }
      else if (pc == 3) { op: OP_PUSH, imm: 44 }
      else if (pc == 4) { op: OP_REWRITE, imm: 45 }
      else if (pc == 5) { op: OP_POP, imm: 0 }
      else if (pc == 6) { op: OP_POP, imm: 0 }
      else if (pc == 7) { op: OP_POP, imm: 0 }
      else { op: OP_HALT, imm: 0 }
    } else {
      // Program 3: PUSH 10, PUSH 20, PUSH 30, REWRITE 35 | PUSH 40, POP (MEM), POP (PAGE), POP (PAGE)
      // Exercises both intra-leaf POP (served by Leaf 2's MEM) and cross-leaf POP (served by Leaf 2's PAGE).
      if (pc == 0) { op: OP_PUSH, imm: 10 }
      else if (pc == 1) { op: OP_PUSH, imm: 20 }
      else if (pc == 2) { op: OP_PUSH, imm: 30 }
      else if (pc == 3) { op: OP_REWRITE, imm: 35 }
      else if (pc == 4) { op: OP_PUSH, imm: 40 }
      else if (pc == 5) { op: OP_POP, imm: 0 }
      else if (pc == 6) { op: OP_POP, imm: 0 }
      else if (pc == 7) { op: OP_POP, imm: 0 }
      else { op: OP_HALT, imm: 0 }
    }

  // ---------------------------------------------------------------- Reference Stack Machine
  pure val initRefState: RefState = {
    pc: 0,
    acc: 0,
    stack: [],
    popLog: [],
    halted: false,
    err: false
  }

  pure def refDepth(st: RefState): int =
    st.stack.length()

  pure def refTop(st: RefState): int =
    if (st.stack.length() == 0) 0
    else st.stack[st.stack.length() - 1]

  pure def refStep(progId: int, st: RefState): RefState =
    if (st.halted or st.err) st
    else {
      val insn = fetchInsn(progId, st.pc)
      val n = st.stack.length()
      if (insn.op == OP_LIT) {
        pc: st.pc + 1,
        acc: insn.imm,
        stack: st.stack,
        popLog: st.popLog,
        halted: false,
        err: false
      } else if (insn.op == OP_PUSH) {
        pc: st.pc + 1,
        acc: st.acc,
        stack: st.stack.append(insn.imm),
        popLog: st.popLog,
        halted: false,
        err: false
      } else if (insn.op == OP_POP) {
        if (n == 0) {
          pc: st.pc,
          acc: st.acc,
          stack: st.stack,
          popLog: st.popLog,
          halted: st.halted,
          err: true
        } else {
          val poppedVal = st.stack[n - 1]
          val poppedStack = st.stack.slice(0, n - 1)
          {
            pc: st.pc + 1,
            acc: poppedVal,
            stack: poppedStack,
            popLog: st.popLog.append(poppedVal),
            halted: false,
            err: false
          }
        }
      } else if (insn.op == OP_REWRITE) {
        if (n == 0) {
          pc: st.pc,
          acc: st.acc,
          stack: st.stack,
          popLog: st.popLog,
          halted: st.halted,
          err: true
        } else {
          val rewrittenStack = st.stack.replaceAt(n - 1, insn.imm)
          {
            pc: st.pc + 1,
            acc: st.acc,
            stack: rewrittenStack,
            popLog: st.popLog,
            halted: false,
            err: false
          }
        }
      } else {
        // OP_HALT
        {
          pc: st.pc,
          acc: st.acc,
          stack: st.stack,
          popLog: st.popLog,
          halted: true,
          err: false
        }
      }
    }

  pure def refRun(progId: int, totalSteps: int): RefState =
    range(0, 2 * S).foldl(initRefState, (st, idx) => {
      if (idx < totalSteps) refStep(progId, st) else st
    })

  // ---------------------------------------------------------------- Bus & Circuit Extraction
  pure def countEnabled(sls: List[StepSlot]): int =
    sls.foldl(0, (cnt, sl) => cnt + (if (sl.en) 1 else 0))

  pure def slotReqSet(sl: StepSlot): Set[BusTuple] =
    if (not(sl.en)) Set(PAD_TUPLE)
    else Set(PAD_TUPLE, sl.decReq, sl.progReq, sl.cellReq)

  // D1 & D8: Request bus gated by slot.en (disabled rows emit PAD_TUPLE)
  pure def leafReqBus(lw: LeafWitness): Set[BusTuple] =
    lw.slots.foldl(Set(PAD_TUPLE), (bus, sl) => bus.union(slotReqSet(sl)))

  // D1, D4, D8: Provider bus gated by active chip selectors (inactive rows emit PAD_TUPLE)
  pure def pageProvSet(lw: LeafWitness): Set[BusTuple] =
    lw.pageTable.foldl(Set(PAD_TUPLE), (bus, r) => {
      if (r.active) bus.union(Set({ chip: r.chipTag, key: r.idx, a: r.a, b: r.b, c: r.c }))
      else bus
    })

  pure def memProvSet(lw: LeafWitness): Set[BusTuple] =
    lw.memTable.foldl(Set(PAD_TUPLE), (bus, r) => {
      if (r.active) bus.union(Set({ chip: TAG_CELL, key: r.addr, a: r.a, b: r.b, c: r.c }))
      else bus
    })

  pure def leafProvBus(lw: LeafWitness): Set[BusTuple] =
    DEC_ROM.union(pageProvSet(lw)).union(memProvSet(lw))

  pure def leafPopLog(lw: LeafWitness): List[int] = {
    val initLog: List[int] = []
    lw.slots.foldl(initLog, (log, sl) => {
      if (sl.en and sl.popped) log.append(sl.popVal) else log
    })
  }

  pure def chainPopLog(w: ChainWitness): List[int] =
    w.leaf2.slots.foldl(leafPopLog(w.leaf1), (log, sl) => {
      if (sl.en and sl.popped) log.append(sl.popVal) else log
    })

  // Extract the committed memory cells after Leaf 1 (from Leaf 1's active MEM rows)
  pure def cellsAfterLeaf1(w: ChainWitness): int -> MemCell =
    w.leaf1.memTable.foldl(emptyCells, (cells, r) => {
      if (r.active) putCell(cells, r.addr, { valField: r.a, linkField: r.b })
      else cells
    })

  // ---------------------------------------------------------------- Circuit Constraint Predicates (D1..D8)
  // Genesis constraint on Leaf 1 pre_d (includes the depth-0 sentinel top == 0, fp == NIL)
  pure def genesisValid(mt: str, pre: CoreRegs): bool = and {
    pre.pc == 0,
    pre.acc == 0,
    pre.depth == 0,
    pre.len == 0,
    pre.memRoot == INIT_MEM_ROOT,
    not(pre.halted),
    (mt == "sentinel_unconstrained") or (pre.top == 0 and pre.fp == NIL)
  }

  // D5: Slot contract (one enable per step, prefix, register chain across S slots, halt is last)
  pure def leafSlotContract(lw: LeafWitness): bool = and {
    lw.slots.length() == S,
    lw.slots[0].en,
    lw.slots.indices().forall(j => {
      j + 1 == S or not(lw.slots[j + 1].en) or lw.slots[j].en
    }),
    lw.slots.indices().forall(j => {
      j + 1 == S or not(lw.slots[j + 1].en) or not(lw.slots[j].post.halted)
    }),
    lw.slots.indices().forall(j => {
      j + 1 == S or lw.slots[j].post == lw.slots[j + 1].pre
    }),
    lw.pre == lw.slots[0].pre,
    lw.post == lw.slots[S - 1].post,
    lw.steps == countEnabled(lw.slots)
  }

  // D3, D4, D6: MEM chip constraints (one base = pre.len, fixed rowIdx k, active prefix, root fold)
  pure def memValid(lw: LeafWitness): bool = {
    val base = lw.pre.len
    val appCount = lw.post.len - base
    val expectedRoot = lw.memTable.foldl(lw.pre.memRoot, (rt, r) => {
      if (r.active) foldMemRoot(rt, r.addr, r.a, r.b) else rt
    })
    and {
      lw.memTable.length() == N_MEM,
      0 <= appCount and appCount <= N_MEM,
      lw.memTable.indices().forall(k => {
        val r = lw.memTable[k]
        and {
          r.rowIdx == k,
          r.addr == base + k,
          r.active == (k < appCount),
          r.c == 0
        }
      }),
      lw.post.memRoot == expectedRoot
    }
  }

  // D2, D4, D8: PAGE chip constraints (CELL served only for 0 <= idx < pre.len, authenticated vs pre.memRoot)
  pure def pageValid(mt: str, progId: int, lw: LeafWitness, prevCells: int -> MemCell): bool = and {
    lw.pageTable.length() <= N_PAGE,
    lw.pageTable.indices().forall(i => {
      val r = lw.pageTable[i]
      not(r.active) or and {
        r.pathValid,
        r.chipTag == TAG_PROG or r.chipTag == TAG_CELL,
        if (r.chipTag == TAG_PROG) and {
          r.a == fetchInsn(progId, r.idx).op,
          r.b == fetchInsn(progId, r.idx).imm,
          r.c == 0
        } else and {
          0 <= r.idx and r.idx < lw.pre.len,
          r.rootUsed == lw.pre.memRoot,
          r.c == 0,
          (mt == "pop_page_unauthenticated") or and {
            r.a == prevCells.get(r.idx).valField,
            r.b == prevCells.get(r.idx).linkField
          }
        }
      }
    })
  }

  // CORE step transition and D7 cached frame protocol constraints
  pure def slotStepValid(mt: str, sl: StepSlot): bool =
    if (not(sl.en)) and {
      sl.op == OP_DISABLED,
      sl.imm == 0,
      sl.post == sl.pre,
      sl.decReq == PAD_TUPLE,
      sl.progReq == PAD_TUPLE,
      sl.cellReq == PAD_TUPLE,
      not(sl.popped),
      sl.popVal == 0
    } else and {
      not(sl.pre.halted),
      VALID_OPS.contains(sl.op),
      sl.decReq == decTuple(sl.op),
      sl.progReq == { chip: TAG_PROG, key: sl.pre.pc, a: sl.op, b: sl.imm, c: 0 },
      if (sl.op == OP_LIT) and {
        sl.cellReq == PAD_TUPLE,
        not(sl.popped) and sl.popVal == 0,
        sl.post.pc == sl.pre.pc + 1,
        sl.post.acc == sl.imm,
        sl.post.top == sl.pre.top,
        sl.post.fp == sl.pre.fp,
        sl.post.depth == sl.pre.depth,
        sl.post.len == sl.pre.len,
        sl.post.memRoot == sl.pre.memRoot,
        not(sl.post.halted)
      } else if (sl.op == OP_PUSH) and {
        // PUSH v: append (top, fp) at len; fp := len, len := len + 1, top := v, depth := depth + 1
        sl.cellReq.chip == TAG_CELL,
        sl.cellReq.key == sl.pre.len,
        sl.cellReq.a == sl.pre.top,
        (mt == "link_from_bus") or (sl.cellReq.b == sl.pre.fp),
        sl.cellReq.c == 0,
        not(sl.popped) and sl.popVal == 0,
        sl.post.pc == sl.pre.pc + 1,
        sl.post.acc == sl.pre.acc,
        sl.post.top == sl.imm,
        sl.post.fp == sl.pre.len,
        sl.post.depth == sl.pre.depth + 1,
        sl.post.len == sl.pre.len + 1,
        sl.post.memRoot == foldMemRoot(sl.pre.memRoot, sl.pre.len, sl.cellReq.a, sl.cellReq.b),
        not(sl.post.halted)
      } else if (sl.op == OP_POP) and {
        // POP (depth >= 1): result is pre.top; load (a, b) at fp (< len); top := a, fp := b, depth := depth - 1
        sl.pre.depth >= 1,
        0 <= sl.pre.fp and sl.pre.fp < sl.pre.len,
        sl.cellReq.chip == TAG_CELL,
        sl.cellReq.key == sl.pre.fp,
        sl.cellReq.c == 0,
        sl.popped and sl.popVal == sl.pre.top,
        sl.post.pc == sl.pre.pc + 1,
        sl.post.acc == sl.pre.top,
        if (mt == "pop_no_reload")
          (sl.post.top == sl.pre.top or sl.post.top == sl.cellReq.a)
        else
          (sl.post.top == sl.cellReq.a),
        if (mt == "pop_no_fp_reload")
          (sl.post.fp == sl.pre.fp or sl.post.fp == sl.cellReq.b)
        else
          (sl.post.fp == sl.cellReq.b),
        if (mt == "depth_unconstrained")
          (sl.post.depth == sl.pre.depth or sl.post.depth == sl.pre.depth - 1)
        else
          (sl.post.depth == sl.pre.depth - 1),
        sl.post.len == sl.pre.len,
        sl.post.memRoot == sl.pre.memRoot,
        not(sl.post.halted)
      } else if (sl.op == OP_REWRITE) and {
        // REWRITE v (depth >= 1): top := v only; no bus request, no append, fp and len unchanged
        sl.pre.depth >= 1,
        not(sl.popped) and sl.popVal == 0,
        sl.post.pc == sl.pre.pc + 1,
        sl.post.acc == sl.pre.acc,
        sl.post.top == sl.imm,
        sl.post.depth == sl.pre.depth,
        not(sl.post.halted),
        if (mt == "rewrite_appends")
          or {
            and {
              sl.cellReq == PAD_TUPLE,
              sl.post.fp == sl.pre.fp,
              sl.post.len == sl.pre.len,
              sl.post.memRoot == sl.pre.memRoot
            },
            and {
              sl.cellReq == { chip: TAG_CELL, key: sl.pre.len, a: sl.imm, b: sl.pre.fp, c: 0 },
              sl.post.fp == sl.pre.len,
              sl.post.len == sl.pre.len + 1,
              sl.post.memRoot == foldMemRoot(sl.pre.memRoot, sl.pre.len, sl.imm, sl.pre.fp)
            }
          }
        else
          and {
            sl.cellReq == PAD_TUPLE,
            sl.post.fp == sl.pre.fp,
            sl.post.len == sl.pre.len,
            sl.post.memRoot == sl.pre.memRoot
          }
      } else and {
        // OP_HALT
        sl.cellReq == PAD_TUPLE,
        not(sl.popped) and sl.popVal == 0,
        sl.post.pc == sl.pre.pc,
        sl.post.acc == sl.pre.acc,
        sl.post.top == sl.pre.top,
        sl.post.fp == sl.pre.fp,
        sl.post.depth == sl.pre.depth,
        sl.post.len == sl.pre.len,
        sl.post.memRoot == sl.pre.memRoot,
        sl.post.halted
      }
    }

  pure def leafCoreStepsValid(mt: str, lw: LeafWitness): bool =
    lw.slots.indices().forall(j => slotStepValid(mt, lw.slots[j]))

  pure def leafAccepted(mt: str, progId: int, lw: LeafWitness, prevCells: int -> MemCell): bool = and {
    leafSlotContract(lw),
    leafCoreStepsValid(mt, lw),
    leafReqBus(lw).subseteq(leafProvBus(lw)),
    memValid(lw),
    pageValid(mt, progId, lw, prevCells)
  }

  // Leaf boundary chaining: leaf 2 starts from leaf 1's post_d
  pure def leafChainValid(mt: str, w: ChainWitness): bool = and {
    w.leaf2.pre.pc == w.leaf1.post.pc,
    w.leaf2.pre.acc == w.leaf1.post.acc,
    w.leaf2.pre.fp == w.leaf1.post.fp,
    w.leaf2.pre.depth == w.leaf1.post.depth,
    w.leaf2.pre.len == w.leaf1.post.len,
    w.leaf2.pre.memRoot == w.leaf1.post.memRoot,
    w.leaf2.pre.halted == w.leaf1.post.halted,
    (mt == "top_dropped") or (w.leaf2.pre.top == w.leaf1.post.top)
  }

  // Full 2-leaf chain acceptance parameterized by mutant name
  pure def acceptsWithMutant(mt: str, w: ChainWitness): bool = and {
    genesisValid(mt, w.leaf1.pre),
    leafAccepted(mt, w.progId, w.leaf1, emptyCells),
    leafChainValid(mt, w),
    leafAccepted(mt, w.progId, w.leaf2, cellsAfterLeaf1(w))
  }

  // Hardened cached protocol acceptance (all D1..D8 and cached protocol constraints active)
  pure def acceptsCached(w: ChainWitness): bool =
    acceptsWithMutant("none", w)

  // Module-level acceptance predicate using const MUTANT
  pure def accepts(w: ChainWitness): bool =
    acceptsWithMutant(MUTANT, w)

  // Main refinement predicate against the reference stack machine
  pure def refinesRefStack(w: ChainWitness): bool = {
    val refFinal = refRun(w.progId, w.leaf1.steps + w.leaf2.steps)
    and {
      not(refFinal.err),
      chainPopLog(w) == refFinal.popLog,
      w.leaf2.post.depth == refDepth(refFinal),
      w.leaf2.post.top == refTop(refFinal)
    }
  }

  // ---------------------------------------------------------------- Witness Builders
  pure def buildStepForMode(
    mt: str,
    progId: int,
    pre: CoreRegs,
    curCells: int -> MemCell
  ): BuiltStep =
    if (pre.halted) {
      val disSlot = {
        en: false,
        op: OP_DISABLED,
        imm: 0,
        pre: pre,
        post: pre,
        decReq: PAD_TUPLE,
        progReq: PAD_TUPLE,
        cellReq: PAD_TUPLE,
        popped: false,
        popVal: 0
      }
      { slot: disSlot, cellsAfter: curCells }
    } else {
      val insn = fetchInsn(progId, pre.pc)
      val dReq = decTuple(insn.op)
      val pReq = { chip: TAG_PROG, key: pre.pc, a: insn.op, b: insn.imm, c: 0 }
      if (insn.op == OP_LIT) {
        val post = {
          pc: pre.pc + 1,
          acc: insn.imm,
          top: pre.top,
          fp: pre.fp,
          depth: pre.depth,
          len: pre.len,
          memRoot: pre.memRoot,
          halted: false
        }
        val sl = {
          en: true,
          op: insn.op,
          imm: insn.imm,
          pre: pre,
          post: post,
          decReq: dReq,
          progReq: pReq,
          cellReq: PAD_TUPLE,
          popped: false,
          popVal: 0
        }
        { slot: sl, cellsAfter: curCells }
      } else if (insn.op == OP_PUSH) {
        // In mode "link_from_bus", at addr == 2 (3rd push), forge stored link = 2 (self-loop) instead of pre.fp = 1
        val linkToStore =
          if (mt == "link_from_bus" and pre.len == 2) 2
          else pre.fp
        val newRoot = foldMemRoot(pre.memRoot, pre.len, pre.top, linkToStore)
        val post = {
          pc: pre.pc + 1,
          acc: pre.acc,
          top: insn.imm,
          fp: pre.len,
          depth: pre.depth + 1,
          len: pre.len + 1,
          memRoot: newRoot,
          halted: false
        }
        val cReq = { chip: TAG_CELL, key: pre.len, a: pre.top, b: linkToStore, c: 0 }
        val sl = {
          en: true,
          op: insn.op,
          imm: insn.imm,
          pre: pre,
          post: post,
          decReq: dReq,
          progReq: pReq,
          cellReq: cReq,
          popped: false,
          popVal: 0
        }
        val updatedCells = putCell(curCells, pre.len, { valField: pre.top, linkField: linkToStore })
        { slot: sl, cellsAfter: updatedCells }
      } else if (insn.op == OP_POP) {
        val loaded = curCells.get(pre.fp)
        val nextTop = if (mt == "pop_no_reload") pre.top else loaded.valField
        val nextFp = if (mt == "pop_no_fp_reload") pre.fp else loaded.linkField
        val nextDepth = if (mt == "depth_unconstrained") pre.depth else (pre.depth - 1)
        val post = {
          pc: pre.pc + 1,
          acc: pre.top,
          top: nextTop,
          fp: nextFp,
          depth: nextDepth,
          len: pre.len,
          memRoot: pre.memRoot,
          halted: false
        }
        val cReq = { chip: TAG_CELL, key: pre.fp, a: loaded.valField, b: loaded.linkField, c: 0 }
        val sl = {
          en: true,
          op: insn.op,
          imm: insn.imm,
          pre: pre,
          post: post,
          decReq: dReq,
          progReq: pReq,
          cellReq: cReq,
          popped: true,
          popVal: pre.top
        }
        { slot: sl, cellsAfter: curCells }
      } else if (insn.op == OP_REWRITE) {
        if (mt == "rewrite_appends") {
          val newRoot = foldMemRoot(pre.memRoot, pre.len, insn.imm, pre.fp)
          val post = {
            pc: pre.pc + 1,
            acc: pre.acc,
            top: insn.imm,
            fp: pre.len,
            depth: pre.depth,
            len: pre.len + 1,
            memRoot: newRoot,
            halted: false
          }
          val cReq = { chip: TAG_CELL, key: pre.len, a: insn.imm, b: pre.fp, c: 0 }
          val sl = {
            en: true,
            op: insn.op,
            imm: insn.imm,
            pre: pre,
            post: post,
            decReq: dReq,
            progReq: pReq,
            cellReq: cReq,
            popped: false,
            popVal: 0
          }
          val updatedCells = putCell(curCells, pre.len, { valField: insn.imm, linkField: pre.fp })
          { slot: sl, cellsAfter: updatedCells }
        } else {
          val post = {
            pc: pre.pc + 1,
            acc: pre.acc,
            top: insn.imm,
            fp: pre.fp,
            depth: pre.depth,
            len: pre.len,
            memRoot: pre.memRoot,
            halted: false
          }
          val sl = {
            en: true,
            op: insn.op,
            imm: insn.imm,
            pre: pre,
            post: post,
            decReq: dReq,
            progReq: pReq,
            cellReq: PAD_TUPLE,
            popped: false,
            popVal: 0
          }
          { slot: sl, cellsAfter: curCells }
        }
      } else {
        // OP_HALT
        val post = {
          pc: pre.pc,
          acc: pre.acc,
          top: pre.top,
          fp: pre.fp,
          depth: pre.depth,
          len: pre.len,
          memRoot: pre.memRoot,
          halted: true
        }
        val sl = {
          en: true,
          op: insn.op,
          imm: insn.imm,
          pre: pre,
          post: post,
          decReq: dReq,
          progReq: pReq,
          cellReq: PAD_TUPLE,
          popped: false,
          popVal: 0
        }
        { slot: sl, cellsAfter: curCells }
      }
    }

  pure def buildLeafForMode(
    mt: str,
    progId: int,
    preRegs: CoreRegs,
    prevCells: int -> MemCell
  ): BuiltLeaf = {
    val emptySlotList: List[StepSlot] = []
    val initAcc: FoldSlotsAcc = { slots: emptySlotList, cells: prevCells }
    val folded = range(0, S).foldl(initAcc, (accState, j) => {
      val curPre = if (j == 0) preRegs else accState.slots[j - 1].post
      val built = buildStepForMode(mt, progId, curPre, accState.cells)
      { slots: accState.slots.append(built.slot), cells: built.cellsAfter }
    })
    val sls = folded.slots
    val endCells = folded.cells
    val postRegs = sls[S - 1].post
    val base = preRegs.len
    val appCount = postRegs.len - base
    val emptyMemRows: List[MemRow] = []
    val mTable = MEM_INDICES.foldl(emptyMemRows, (rows, k) => {
      val addr = base + k
      val isAct = (k < appCount)
      val cell = endCells.get(addr)
      rows.append({
        active: isAct,
        rowIdx: k,
        addr: addr,
        a: if (isAct) cell.valField else 0,
        b: if (isAct) cell.linkField else 0,
        c: 0
      })
    })
    val emptyPageRows: List[PageRow] = []
    val progPageRows = sls.foldl(emptyPageRows, (rows, sl) => {
      rows.append({
        active: sl.en,
        chipTag: TAG_PROG,
        idx: sl.pre.pc,
        a: sl.op,
        b: sl.imm,
        c: 0,
        rootUsed: preRegs.memRoot,
        pathValid: true
      })
    })
    val allPageRows = MEM_INDICES.foldl(progPageRows, (rows, idx) => {
      val isOldCell = (idx < base)
      val oldCell = prevCells.get(idx)
      rows.append({
        active: isOldCell,
        chipTag: TAG_CELL,
        idx: idx,
        a: if (isOldCell) oldCell.valField else 0,
        b: if (isOldCell) oldCell.linkField else 0,
        c: 0,
        rootUsed: preRegs.memRoot,
        pathValid: true
      })
    })
    val lw = {
      slots: sls,
      memTable: mTable,
      pageTable: allPageRows,
      steps: countEnabled(sls),
      pre: preRegs,
      post: postRegs
    }
    { leaf: lw, cellsAfter: endCells }
  }

  pure def buildChainForMode(mt: str, progId: int): ChainWitness = {
    val pre1 =
      if (mt == "sentinel_unconstrained") withTop(initCoreRegs, 777)
      else initCoreRegs
    val built1 = buildLeafForMode(mt, progId, pre1, emptyCells)
    val pre2 =
      if (mt == "top_dropped") withTop(built1.leaf.post, 999)
      else built1.leaf.post
    val cellsForLeaf2 =
      if (mt == "pop_page_unauthenticated")
        putCell(built1.cellsAfter, 2, { valField: 888, linkField: 1 })
      else
        built1.cellsAfter
    val built2 = buildLeafForMode(mt, progId, pre2, cellsForLeaf2)
    {
      progId: progId,
      leaf1: built1.leaf,
      leaf2: built2.leaf
    }
  }

  // ---------------------------------------------------------------- Named Honest Witnesses
  // 1. Pushes 3 distinct frames (10, 20, 30) + LIT 99 in Leaf 1; pops all 3 (30, 20, 10) + HALT in Leaf 2
  pure val hw_push3_pop3: ChainWitness = buildChainForMode("none", 0)

  // 2. Pushes 3 distinct frames (100, 200->250 via REWRITE, 300) in Leaf 1; pops (300, 250, 100) + HALT in Leaf 2
  pure val hw_push3_rewrite_pop3: ChainWitness = buildChainForMode("none", 1)

  // 3. Pushes 4 distinct frames (11, 22, 33, 44) in Leaf 1; REWRITE 45 + pops 3 (45, 33, 22) in Leaf 2, leaving top=11, depth=1
  pure val hw_push4_rewrite_leaf2_pop3: ChainWitness = buildChainForMode("none", 2)

  // 4. Pushes 3 frames + REWRITE in Leaf 1; PUSH 40 + POP (from MEM) + 2 POPs (from PAGE) in Leaf 2
  pure val hw_intra_and_inter_leaf_pop: ChainWitness = buildChainForMode("none", 3)

  // ---------------------------------------------------------------- Named Mutant Counterexample Witnesses
  // Required Mutant 1 (F12): REWRITE appends (v, fp) at len and sets fp := len
  pure val cex_rewrite_appends: ChainWitness = buildChainForMode("rewrite_appends", 1)

  // Required Mutant 2 (F13): PUSH stores link from a free bus field instead of fp register
  pure val cex_link_from_bus: ChainWitness = buildChainForMode("link_from_bus", 0)

  // Required Mutant 3: POP does not set top from the loaded entry (keeps old top)
  pure val cex_pop_no_reload: ChainWitness = buildChainForMode("pop_no_reload", 0)

  // Required Mutant 4: Leaf statement does not carry top; leaf 2's pre.top is chosen freely
  pure val cex_top_dropped: ChainWitness = buildChainForMode("top_dropped", 0)

  // Additional Mutant 5: Depth-0 sentinel (top == 0, fp == NIL) is unconstrained at genesis
  pure val cex_sentinel_unconstrained: ChainWitness = buildChainForMode("sentinel_unconstrained", 0)

  // Additional Mutant 6: Frame depth register is unconstrained across steps / leaf boundary
  pure val cex_depth_unconstrained: ChainWitness = buildChainForMode("depth_unconstrained", 0)

  // Additional Mutant 7: POP does not reload fp from the loaded entry's link field (replays PAGE row)
  pure val cex_pop_no_fp_reload: ChainWitness = buildChainForMode("pop_no_fp_reload", 0)

  // Additional Mutant 8: POP reads a cross-leaf PAGE entry not authenticated against pre.memRoot
  pure val cex_pop_page_unauthenticated: ChainWitness = buildChainForMode("pop_page_unauthenticated", 0)

  // ---------------------------------------------------------------- State Machine for Model Checking
  pure val MODES: Set[str] = Set(
    "none",
    "rewrite_appends",
    "link_from_bus",
    "pop_no_reload",
    "top_dropped",
    "sentinel_unconstrained",
    "depth_unconstrained",
    "pop_no_fp_reload",
    "pop_page_unauthenticated"
  )

  pure val PROG_IDS: Set[int] = Set(0, 1, 2, 3)

  pure def pickChainWitness(progId: int, mode: str): ChainWitness =
    if (mode == "rewrite_appends") cex_rewrite_appends
    else if (mode == "link_from_bus") cex_link_from_bus
    else if (mode == "pop_no_reload") cex_pop_no_reload
    else if (mode == "top_dropped") cex_top_dropped
    else if (mode == "sentinel_unconstrained") cex_sentinel_unconstrained
    else if (mode == "depth_unconstrained") cex_depth_unconstrained
    else if (mode == "pop_no_fp_reload") cex_pop_no_fp_reload
    else if (mode == "pop_page_unauthenticated") cex_pop_page_unauthenticated
    else buildChainForMode("none", progId)

  var w: ChainWitness
  var acc: bool
  var tamperMode: str

  action sample = {
    nondet pId = PROG_IDS.oneOf()
    nondet m = MODES.oneOf()
    val cand = pickChainWitness(pId, m)
    all {
      w' = cand,
      acc' = accepts(cand),
      tamperMode' = m
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Invariants
  val invPopSeq: bool =
    acc implies (chainPopLog(w) == refRun(w.progId, w.leaf1.steps + w.leaf2.steps).popLog)

  val invFinalDepth: bool =
    acc implies (w.leaf2.post.depth == refDepth(refRun(w.progId, w.leaf1.steps + w.leaf2.steps)))

  val invFinalTop: bool =
    acc implies (w.leaf2.post.top == refTop(refRun(w.progId, w.leaf1.steps + w.leaf2.steps)))

  // Main invariant: frame-stack refinement across the chained leaves
  val invFrameStackRefinement: bool =
    acc implies refinesRefStack(w)

  val invSound: bool =
    invFrameStackRefinement

  val invTamperRefused: bool =
    (acc and MUTANT == "none") implies (tamperMode == "none")

  // Non-vacuity reachability witnesses (expected to FAIL on cachedSound)
  val reachPushInLeaf1PopInLeaf2: bool =
    not(acc and w.progId == 0 and chainPopLog(w) == [30, 20, 10] and w.leaf2.post.depth == 0 and w.leaf2.post.top == 0)

  val reachRewriteAndPopAcrossLeaves: bool =
    not(acc and w.progId == 1 and chainPopLog(w) == [300, 250, 100] and w.leaf2.post.depth == 0 and w.leaf2.post.top == 0)

  val reachNonEmptyFinalStack: bool =
    not(acc and w.progId == 2 and chainPopLog(w) == [45, 33, 22] and w.leaf2.post.depth == 1 and w.leaf2.post.top == 11)

  val reachIntraAndInterLeafPop: bool =
    not(acc and w.progId == 3 and chainPopLog(w) == [40, 35, 20] and w.leaf2.post.depth == 1 and w.leaf2.post.top == 10)
}

// Sound cached protocol instantiation (invFrameStackRefinement and invSound HOLD)
module cachedSound { import cached(S = 4, MUTANT = "none").* }

// Required mutants (invFrameStackRefinement and invSound FAIL on each)
module cached_rewrite_appends { import cached(S = 4, MUTANT = "rewrite_appends").* }
module cached_link_from_bus { import cached(S = 4, MUTANT = "link_from_bus").* }
module cached_pop_no_reload { import cached(S = 4, MUTANT = "pop_no_reload").* }
module cached_top_dropped { import cached(S = 4, MUTANT = "top_dropped").* }

// Additional cached-protocol constraint mutants (invFrameStackRefinement and invSound FAIL on each)
module cached_sentinel_unconstrained { import cached(S = 4, MUTANT = "sentinel_unconstrained").* }
module cached_depth_unconstrained { import cached(S = 4, MUTANT = "depth_unconstrained").* }
module cached_pop_no_fp_reload { import cached(S = 4, MUTANT = "pop_no_fp_reload").* }
module cached_pop_page_unauthenticated { import cached(S = 4, MUTANT = "pop_page_unauthenticated").* }
