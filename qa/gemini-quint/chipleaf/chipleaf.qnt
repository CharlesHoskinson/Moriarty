// Soundness & Completeness Model of the "Sherlock leaf as a chip" Design
// Design under test: research/sherlock-processor-analogy-20261110/study.md
// Conventions: models/sherlock/quint/leafSlots.qnt
//
// Models:
//   - CORE step relation over the abstract ISA (FETCH, LOAD, APPEND, PUSH, POP, ALU, MOVE, HALT)
//   - One 5-column bus of tuples (chip, key, a, b, c)
//   - Chips: DEC, PAGE, MEM, ALU/RANGE, HASH, plus TRAP as CORE gates
//   - Set inclusion check (reqBus.subseteq(provBus)) with NO multiplicities or shuffle
//   - S step slots, H_CORE micro-op sub-slots per step, fixed chip table capacities
//   - Write-once append-only memory log (MEM + PAGE) and linked frame stack (fp, topVal, topLink)
//   - Traps (P0, P1, P2) and publication (last enabled slot, followed by HALT)
//   - Circuit acceptance predicates:
//       * acceptsAsWritten(w): design as written in study.md
//       * acceptsHardened(w):  design with the required soundness constraints added
//       * accepts(w):          parameterized by MUTANT for model checking each constraint
//   - Reference step relation (refPost, refRules, refTerminalLast)
//   - 13 concrete adversarial counterexample witnesses (cex_*)

module chipleaf {
  const S: int
  const MUTANT: str

  // ---------------------------------------------------------------- capacities & constants
  pure val H_CORE: int = 2
  pure val N_PAGE: int = 4
  pure val N_MEM: int = 2
  pure val N_ALU: int = 4
  pure val N_HASH: int = 6
  pure val BASE_LEN: int = 2
  pure val MAX_VAL: int = 200

  // Rule IDs (0 reserved for disabled slot, 7 for TRAP/PUBLISH)
  pure val RULE_DISABLED: int = 0
  pure val RULE_FETCH_ALU: int = 1
  pure val RULE_APPEND_MOVE: int = 2
  pure val RULE_LOAD_ALU: int = 3
  pure val RULE_PUSH: int = 4
  pure val RULE_POP: int = 5
  pure val RULE_REWRITE_TOP: int = 6
  pure val RULE_PUBLISH: int = 7

  pure val RULES: Set[int] = Set(1, 2, 3, 4, 5, 6, 7)
  pure val ADDR_DOM: Set[int] = Set(0, 1, 2, 3, 4, 5)

  // Micro-op codes for DEC control words (encoding the abstract ISA)
  pure val UOP_HALT: int = 0
  pure val UOP_FETCH: int = 1
  pure val UOP_LOAD: int = 2
  pure val UOP_APPEND: int = 3
  pure val UOP_PUSH: int = 4
  pure val UOP_POP: int = 5
  pure val UOP_ALU: int = 6
  pure val UOP_MOVE: int = 7

  // Cell kinds in MEM / PAGE
  pure val KIND_DATA: int = 0
  pure val KIND_FRAME: int = 1

  // ALU operation kinds
  pure val ALU_ADD: int = 1
  pure val ALU_CMP_LT: int = 2

  // HASH domain tags
  pure val DOM_PROG_LEAF: int = 1
  pure val DOM_CELL_LEAF: int = 2
  pure val DOM_MEM_FOLD: int = 3

  // Committed roots for the fixed test environment
  pure val PROG_ROOT: int = 7001
  pure val INIT_MEM_ROOT: int = 8001

  // ---------------------------------------------------------------- types
  type BusTuple = {
    chip: str,
    key: int,
    a: int,
    b: int,
    c: int
  }

  type CellVal = {
    kind: int,
    payload: int,
    linkAddr: int
  }

  type ProgRow = {
    opcode: int,
    imm: int,
    targetPc: int
  }

  type CoreRegs = {
    pc: int,
    acc: int,
    fp: int,
    topVal: int,
    topLink: int,
    len: int,
    aluFlag: int,
    trap: int,
    halted: bool,
    memRoot: int,
    progRoot: int
  }

  type MicroSubSlot = {
    uopEn: bool,
    uop: int,
    preRegs: CoreRegs,
    postRegs: CoreRegs,
    req: BusTuple
  }

  type StepSlot = {
    en: bool,
    decClass: int,
    ruleId: int,
    decReq: BusTuple,
    uop0: MicroSubSlot,
    uop1: MicroSubSlot,
    pre: CoreRegs,
    post: CoreRegs
  }

  type BuiltStep = {
    uop0: MicroSubSlot,
    uop1: MicroSubSlot,
    post: CoreRegs,
    ruleId: int,
    decReq: BusTuple
  }

  type PageChipRow = {
    active: bool,
    tag: str,
    idx: int,
    a: int,
    b: int,
    c: int,
    rootUsed: int,
    pathValid: bool
  }

  type MemChipRow = {
    active: bool,
    rowIdx: int,
    addr: int,
    a: int,
    b: int,
    c: int
  }

  type AluChipRow = {
    active: bool,
    opKind: int,
    flagBit: int,
    x: int,
    y: int,
    z: int,
    rangeOk: bool
  }

  type HashChipRow = {
    active: bool,
    domId: int,
    in0: int,
    in1: int,
    outHash: int
  }

  type Witness = {
    slots: List[StepSlot],
    pageTable: List[PageChipRow],
    memTable: List[MemChipRow],
    aluTable: List[AluChipRow],
    hashTable: List[HashChipRow],
    extraBusRows: Set[BusTuple],
    rulesD: List[int],
    steps: int,
    pre: CoreRegs,
    post: CoreRegs,
    halted: bool
  }

  // ---------------------------------------------------------------- pure record update helpers
  pure val NO_REQ: BusTuple = { chip: "NONE", key: 0, a: 0, b: 0, c: 0 }

  pure def withHalted(r: CoreRegs, h: bool): CoreRegs = {
    pc: r.pc,
    acc: r.acc,
    fp: r.fp,
    topVal: r.topVal,
    topLink: r.topLink,
    len: r.len,
    aluFlag: r.aluFlag,
    trap: r.trap,
    halted: h,
    memRoot: r.memRoot,
    progRoot: r.progRoot
  }

  pure def withAccPcFlag(r: CoreRegs, newAcc: int, newPc: int, newFlag: int): CoreRegs = {
    pc: newPc,
    acc: newAcc,
    fp: r.fp,
    topVal: r.topVal,
    topLink: r.topLink,
    len: r.len,
    aluFlag: newFlag,
    trap: r.trap,
    halted: r.halted,
    memRoot: r.memRoot,
    progRoot: r.progRoot
  }

  pure def withLenRoot(r: CoreRegs, newLen: int, newRoot: int): CoreRegs = {
    pc: r.pc,
    acc: r.acc,
    fp: r.fp,
    topVal: r.topVal,
    topLink: r.topLink,
    len: newLen,
    aluFlag: r.aluFlag,
    trap: r.trap,
    halted: r.halted,
    memRoot: newRoot,
    progRoot: r.progRoot
  }

  pure def withStackState(
    r: CoreRegs,
    newFp: int,
    newTopVal: int,
    newTopLink: int,
    newLen: int,
    newRoot: int,
    newAcc: int,
    newPc: int
  ): CoreRegs = {
    pc: newPc,
    acc: newAcc,
    fp: newFp,
    topVal: newTopVal,
    topLink: newTopLink,
    len: newLen,
    aluFlag: r.aluFlag,
    trap: r.trap,
    halted: r.halted,
    memRoot: newRoot,
    progRoot: r.progRoot
  }

  pure def mkDisabledSlotAt(regs: CoreRegs): StepSlot =
    val sub = { uopEn: false, uop: UOP_HALT, preRegs: regs, postRegs: regs, req: NO_REQ }
    {
      en: false,
      decClass: RULE_DISABLED,
      ruleId: RULE_DISABLED,
      decReq: NO_REQ,
      uop0: sub,
      uop1: sub,
      pre: regs,
      post: regs
    }

  // ---------------------------------------------------------------- committed program & initial memory
  // Program table behind PROG_ROOT:
  //   pc = 0..6 cover all rules (APPEND_MOVE, LOAD_ALU, PUSH, POP, REWRITE_TOP, FETCH_ALU, PUBLISH)
  pure def progLookup(pc: int): ProgRow =
    if (pc == 0) { opcode: RULE_APPEND_MOVE, imm: 5, targetPc: 1 }
    else if (pc == 1) { opcode: RULE_LOAD_ALU, imm: 0, targetPc: 2 }
    else if (pc == 2) { opcode: RULE_PUBLISH, imm: 0, targetPc: 2 }
    else if (pc == 3) { opcode: RULE_PUSH, imm: 20, targetPc: 4 }
    else if (pc == 4) { opcode: RULE_POP, imm: 0, targetPc: 2 }
    else if (pc == 5) { opcode: RULE_REWRITE_TOP, imm: 25, targetPc: 4 }
    else if (pc == 6) { opcode: RULE_FETCH_ALU, imm: 15, targetPc: 2 }
    else { opcode: RULE_PUBLISH, imm: 0, targetPc: pc }

  // Initial memory cells (addr < BASE_LEN == 2) committed in INIT_MEM_ROOT:
  //   addr = 0: initial caller frame (kind = KIND_FRAME, payload = 100, linkAddr = -1)
  //   addr = 1: initial data cell    (kind = KIND_DATA,  payload = 50,  linkAddr = 0)
  // Unallocated leaves (addr >= BASE_LEN) in the depth-20 pre_d.mem_root tree hold (0, 0, 0).
  pure def initMemCell(addr: int): CellVal =
    if (addr == 0) { kind: KIND_FRAME, payload: 100, linkAddr: -1 }
    else if (addr == 1) { kind: KIND_DATA, payload: 50, linkAddr: 0 }
    else { kind: KIND_DATA, payload: 0, linkAddr: 0 }

  // ---------------------------------------------------------------- pure chip functions
  pure def packAluKey(opKind: int, flagBit: int): int = opKind * 10 + flagBit

  pure def evalAluVal(opKind: int, x: int, y: int): int =
    if (opKind == ALU_ADD) (x + y) % MAX_VAL
    else if (opKind == ALU_CMP_LT) (if (x < y) 1 else 0)
    else 0

  pure def evalAluFlag(opKind: int, x: int, y: int): int =
    if (opKind == ALU_ADD) (if (x + y >= MAX_VAL) 1 else 0)
    else if (opKind == ALU_CMP_LT) (if (x < y) 1 else 0)
    else 0

  pure def inRange128(v: int): bool = (0 <= v and v < MAX_VAL)

  pure def poseidonHash(domId: int, in0: int, in1: int): int =
    domId * 100000 + in0 * 313 + in1 * 17 + 7

  pure def cellDigest(addr: int, a: int, b: int, c: int): int =
    addr * 10000 + a * 1000 + b * 10 + c

  // TRAP priority check (P0: error trap != 0, P1: fuel/pc exhausted, P2: publish)
  pure def trapActive(r: CoreRegs): bool =
    r.trap != 0 or r.halted or progLookup(r.pc).opcode == RULE_PUBLISH

  // Dispatch class computed by CORE from trap bits and current instruction
  pure def computeDecClass(r: CoreRegs): int =
    if (trapActive(r)) RULE_PUBLISH
    else progLookup(r.pc).opcode

  // Fixed DEC ROM table: maps decClass -> (ruleId, uop0, uop1)
  pure def decRomUop0(decClass: int): int =
    if (decClass == RULE_FETCH_ALU) UOP_FETCH
    else if (decClass == RULE_APPEND_MOVE) UOP_APPEND
    else if (decClass == RULE_LOAD_ALU) UOP_LOAD
    else if (decClass == RULE_PUSH) UOP_PUSH
    else if (decClass == RULE_POP) UOP_POP
    else if (decClass == RULE_REWRITE_TOP) UOP_MOVE
    else UOP_HALT

  pure def decRomUop1(decClass: int): int =
    if (decClass == RULE_FETCH_ALU) UOP_ALU
    else if (decClass == RULE_APPEND_MOVE) UOP_MOVE
    else if (decClass == RULE_LOAD_ALU) UOP_ALU
    else if (decClass == RULE_PUSH) UOP_MOVE
    else if (decClass == RULE_POP) UOP_MOVE
    else if (decClass == RULE_REWRITE_TOP) UOP_MOVE
    else UOP_HALT

  pure def decRomTuple(decClass: int): BusTuple = {
    chip: "DEC",
    key: decClass,
    a: decClass,
    b: decRomUop0(decClass),
    c: decRomUop1(decClass)
  }

  pure val DEC_TABLE: Set[BusTuple] =
    RULES.fold(Set(NO_REQ), (acc, r) => acc.union(Set(decRomTuple(r))))

  // ---------------------------------------------------------------- reference machine semantics
  type RefArch = {
    regs: CoreRegs,
    mem: int -> CellVal
  }

  pure def putMem(m: int -> CellVal, writeAddr: int, cell: CellVal): int -> CellVal =
    ADDR_DOM.mapBy(a => if (a == writeAddr) cell else m.get(a))

  pure def initArch(pc0: int, acc0: int): RefArch = {
    regs: {
      pc: pc0,
      acc: acc0,
      fp: 0,
      topVal: 10,
      topLink: 0,
      len: BASE_LEN,
      aluFlag: 0,
      trap: 0,
      halted: false,
      memRoot: INIT_MEM_ROOT,
      progRoot: PROG_ROOT
    },
    mem: ADDR_DOM.mapBy(a => initMemCell(a))
  }

  pure def refStepArch(arch: RefArch): RefArch =
    val r = arch.regs
    val insn = progLookup(r.pc)
    val rId = computeDecClass(r)
    val zFetch = evalAluVal(ALU_ADD, r.acc, insn.imm)
    val flFetch = evalAluFlag(ALU_ADD, r.acc, insn.imm)
    val dataCell = { kind: KIND_DATA, payload: r.acc, linkAddr: 0 }
    val dataRoot = poseidonHash(DOM_MEM_FOLD, r.memRoot, cellDigest(r.len, dataCell.kind, dataCell.payload, dataCell.linkAddr))
    val readAddr = if (r.len > 0) r.len - 1 else 0
    val loaded = arch.mem.get(readAddr)
    val zLoad = evalAluVal(ALU_ADD, r.acc, loaded.payload)
    val flLoad = evalAluFlag(ALU_ADD, r.acc, loaded.payload)
    val spilled = { kind: KIND_FRAME, payload: r.topVal, linkAddr: r.fp }
    val pushRoot = poseidonHash(DOM_MEM_FOLD, r.memRoot, cellDigest(r.len, spilled.kind, spilled.payload, spilled.linkAddr))
    val popAddr = if (0 <= r.fp and r.fp <= 5) r.fp else 0
    val restored = arch.mem.get(popAddr)
    if (rId == RULE_PUBLISH)
      { regs: withHalted(r, true), mem: arch.mem }
    else if (rId == RULE_FETCH_ALU)
      { regs: withAccPcFlag(r, zFetch, insn.targetPc, flFetch), mem: arch.mem }
    else if (rId == RULE_APPEND_MOVE)
      {
        regs: withAccPcFlag(withLenRoot(r, r.len + 1, dataRoot), r.acc + 1, insn.targetPc, r.aluFlag),
        mem: putMem(arch.mem, r.len, dataCell)
      }
    else if (rId == RULE_LOAD_ALU)
      { regs: withAccPcFlag(r, zLoad, insn.targetPc, flLoad), mem: arch.mem }
    else if (rId == RULE_PUSH)
      {
        regs: withStackState(r, r.len, insn.imm, r.len, r.len + 1, pushRoot, r.acc, insn.targetPc),
        mem: putMem(arch.mem, r.len, spilled)
      }
    else if (rId == RULE_POP)
      {
        regs: withStackState(r, restored.linkAddr, restored.payload, restored.linkAddr, r.len, r.memRoot, r.acc + restored.payload, insn.targetPc),
        mem: arch.mem
      }
    else
      // RULE_REWRITE_TOP: in reference semantics, rewriting cached top updates topVal in place
      // without corrupting fp or pushing a duplicate frame onto the stack.
      {
        regs: withStackState(r, r.fp, insn.imm, r.topLink, r.len, r.memRoot, r.acc, insn.targetPc),
        mem: arch.mem
      }

  pure def refRunArch(s0: CoreRegs, k: int): RefArch =
    val initA = {
      regs: s0,
      mem: ADDR_DOM.mapBy(a => initMemCell(a))
    }
    range(0, S).foldl(initA, (a, j) => if (j < k) refStepArch(a) else a)

  pure def refPost(s0: CoreRegs, k: int): CoreRegs =
    refRunArch(s0, k).regs

  pure def refRules(s0: CoreRegs, k: int): List[int] =
    range(0, S).foldl([], (l, j) =>
      l.append(if (j < k) computeDecClass(refRunArch(s0, j).regs) else RULE_DISABLED))

  pure def refTerminalLast(s0: CoreRegs, k: int): bool =
    0.to(S - 1).forall(j => j >= k or not(refRunArch(s0, j).regs.halted))

  // ---------------------------------------------------------------- bus & chip table extraction
  pure def slotReqs(sl: StepSlot): Set[BusTuple] =
    val s0 = if (sl.en) Set(NO_REQ, sl.decReq) else Set(NO_REQ)
    val s1 = if (sl.uop0.uopEn) s0.union(Set(sl.uop0.req)) else s0
    if (sl.uop1.uopEn) s1.union(Set(sl.uop1.req)) else s1

  pure def coreReqBus(w: Witness): Set[BusTuple] =
    w.slots.foldl(Set(NO_REQ), (acc, sl) => acc.union(slotReqs(sl)))

  pure def pageProvBus(w: Witness): Set[BusTuple] =
    w.pageTable.foldl(Set(NO_REQ), (acc, r) =>
      if (r.active) acc.union(Set({ chip: r.tag, key: r.idx, a: r.a, b: r.b, c: r.c }))
      else acc)

  pure def memProvBus(w: Witness): Set[BusTuple] =
    w.memTable.foldl(Set(NO_REQ), (acc, r) =>
      if (r.active) acc.union(Set({ chip: "CELL", key: r.addr, a: r.a, b: r.b, c: r.c }))
      else acc)

  pure def aluProvBus(w: Witness): Set[BusTuple] =
    w.aluTable.foldl(Set(NO_REQ), (acc, r) =>
      if (r.active) acc.union(Set({
        chip: "ALU",
        key: packAluKey(r.opKind, r.flagBit),
        a: r.x,
        b: r.y,
        c: r.z
      }))
      else acc)

  pure def hashProvBus(w: Witness): Set[BusTuple] =
    w.hashTable.foldl(Set(NO_REQ), (acc, r) =>
      if (r.active) acc.union(Set({
        chip: "HASH",
        key: r.domId,
        a: r.in0,
        b: r.in1,
        c: r.outHash
      }))
      else acc)

  // Provider bus set when properly gated by chip provider selectors
  pure def gatedProvBus(w: Witness): Set[BusTuple] =
    DEC_TABLE
      .union(pageProvBus(w))
      .union(memProvBus(w))
      .union(aluProvBus(w))
      .union(hashProvBus(w))

  // In study.md 1.4 ("one lookup_any of the column set on itself, gated by a request selector"),
  // if the RHS of lookup_any is the shared bus column set itself (including request rows or
  // untagged extra rows), any request row is already in the RHS set!
  pure def ungatedSelfBus(w: Witness): Set[BusTuple] =
    gatedProvBus(w).union(coreReqBus(w)).union(w.extraBusRows)

  pure def countEnabledSlots(sls: List[StepSlot]): int =
    sls.foldl(0, (n, sl) => n + (if (sl.en) 1 else 0))

  // ---------------------------------------------------------------- chip-level constraints
  // PAGE chip constraints:
  // As written in study.md 2.3: active PAGE row opens against prog_root or pre_d.mem_root.
  // Note: study.md 2.3 omits `idx < base` for CELL rows!
  pure def pageValidAsWritten(w: Witness): bool =
    w.pageTable.length() <= N_PAGE and
    w.pageTable.indices().forall(i =>
      val r = w.pageTable[i]
      not(r.active) or and {
        r.pathValid,
        r.tag == "PROG" or r.tag == "CELL",
        if (r.tag == "PROG") and {
          r.rootUsed == w.pre.progRoot,
          r.a == progLookup(r.idx).opcode,
          r.b == progLookup(r.idx).imm,
          r.c == progLookup(r.idx).targetPc
        } else and {
          r.rootUsed == w.pre.memRoot,
          r.a == initMemCell(r.idx).kind,
          r.b == initMemCell(r.idx).payload,
          r.c == initMemCell(r.idx).linkAddr
        }
      }
    )

  // Hardened PAGE constraint (C-PAGE-1): CELL rows in PAGE must satisfy 0 <= idx < w.pre.len (base)!
  pure def pageDomainSeparated(w: Witness): bool =
    w.pageTable.indices().forall(i =>
      val r = w.pageTable[i]
      not(r.active) or r.tag != "CELL" or (0 <= r.idx and r.idx < w.pre.len)
    )

  // MEM chip constraints:
  // Hardened (C-MEM-1): row i has rowIdx == i, addr == w.pre.len + i, active iff i < w.post.len - w.pre.len,
  // and w.post.len - w.pre.len <= N_MEM.
  pure def memAddressFixed(w: Witness): bool =
    w.memTable.length() <= N_MEM and
    w.memTable.indices().forall(i =>
      val r = w.memTable[i]
      r.rowIdx == i and r.addr == w.pre.len + i
    )

  pure def memActivePrefixBound(w: Witness): bool =
    val appendedCount = w.post.len - w.pre.len
    0 <= appendedCount and appendedCount <= N_MEM and
    w.memTable.indices().forall(i =>
      w.memTable[i].active == (i < appendedCount)
    )

  // Root update check: active MEM rows fold into post.memRoot via HASH chip rows
  pure def memRootFoldValid(w: Witness): bool =
    val expectedRoot = w.memTable.foldl(w.pre.memRoot, (rt, r) =>
      if (r.active) poseidonHash(DOM_MEM_FOLD, rt, cellDigest(r.addr, r.a, r.b, r.c))
      else rt
    )
    w.post.memRoot == expectedRoot

  // ALU & RANGE chip constraints (C-ALU-1):
  pure def aluTableValid(w: Witness): bool =
    w.aluTable.length() <= N_ALU and
    w.aluTable.indices().forall(i =>
      val r = w.aluTable[i]
      not(r.active) or and {
        r.rangeOk,
        inRange128(r.x),
        inRange128(r.y),
        inRange128(r.z),
        r.z == evalAluVal(r.opKind, r.x, r.y),
        r.flagBit == evalAluFlag(r.opKind, r.x, r.y)
      }
    )

  // HASH chip constraints (C-HASH-1):
  pure def hashTableValid(w: Witness): bool =
    w.hashTable.length() <= N_HASH and
    w.hashTable.indices().forall(i =>
      val r = w.hashTable[i]
      not(r.active) or r.outHash == poseidonHash(r.domId, r.in0, r.in1)
    )

  // ---------------------------------------------------------------- CORE step & micro-op constraints
  // Atomic step enablement (no mid-step leaf end): both uop0 and uop1 have uopEn == sl.en
  pure def stepAtomicMicroOps(w: Witness): bool =
    w.slots.indices().forall(j =>
      val sl = w.slots[j]
      sl.uop0.uopEn == sl.en and
      sl.uop1.uopEn == sl.en and
      sl.uop0.preRegs == sl.pre and
      sl.uop1.preRegs == sl.uop0.postRegs and
      sl.post == sl.uop1.postRegs
    )

  // Relaxed mid-step chain (allows uop1 to be disabled mid-step if uop1.uopEn == false)
  pure def stepMidStepAllowed(w: Witness): bool =
    w.slots.indices().forall(j =>
      val sl = w.slots[j]
      (sl.en implies sl.uop0.uopEn) and
      (not(sl.en) implies (not(sl.uop0.uopEn) and not(sl.uop1.uopEn))) and
      sl.uop0.preRegs == sl.pre and
      sl.uop1.preRegs == sl.uop0.postRegs and
      (not(sl.uop1.uopEn) implies sl.uop1.postRegs == sl.uop1.preRegs) and
      sl.post == sl.uop1.postRegs
    )

  // Sub-step len monotonicity & causal LOAD bound (C-MEM-2 & C-MEM-3):
  //   - APPEND / PUSH increments len by 1 and uses req.key == preRegs.len
  //   - All other micro-ops preserve len
  //   - LOAD / POP checks 0 <= req.key < preRegs.len
  pure def subSlotLenAndCausalBound(sub: MicroSubSlot): bool =
    if (not(sub.uopEn))
      sub.postRegs == sub.preRegs and sub.req == NO_REQ
    else if (sub.uop == UOP_APPEND or sub.uop == UOP_PUSH)
      sub.req.chip == "CELL" and
      sub.req.key == sub.preRegs.len and
      sub.postRegs.len == sub.preRegs.len + 1
    else if (sub.uop == UOP_LOAD or sub.uop == UOP_POP)
      sub.req.chip == "CELL" and
      0 <= sub.req.key and sub.req.key < sub.preRegs.len and
      sub.postRegs.len == sub.preRegs.len
    else
      sub.postRegs.len == sub.preRegs.len

  pure def allSubSlotsLenAndCausal(w: Witness): bool =
    w.slots.indices().forall(j =>
      subSlotLenAndCausalBound(w.slots[j].uop0) and
      subSlotLenAndCausalBound(w.slots[j].uop1)
    )

  // Stack link & top-rewrite discipline (C-STACK-1):
  //   - On PUSH: req is (CELL, preRegs.len, KIND_FRAME, preRegs.topVal, preRegs.fp),
  //              postRegs.fp == preRegs.len, postRegs.topLink == preRegs.len
  //   - On POP:  req is (CELL, preRegs.fp, KIND_FRAME, postRegs.topVal, postRegs.fp),
  //              postRegs.topLink == postRegs.fp
  //   - On REWRITE_TOP: modifies topVal in register ONLY (no MEM append that overwrites fp!)
  pure def stackDisciplineValid(w: Witness): bool =
    w.slots.indices().forall(j =>
      val sl = w.slots[j]
      not(sl.en) or and {
        (sl.ruleId == RULE_PUSH) implies and {
          sl.uop0.uop == UOP_PUSH,
          sl.uop0.req == {
            chip: "CELL",
            key: sl.pre.len,
            a: KIND_FRAME,
            b: sl.pre.topVal,
            c: sl.pre.fp
          },
          sl.post.fp == sl.pre.len,
          sl.post.topLink == sl.pre.len
        },
        (sl.ruleId == RULE_POP) implies and {
          sl.uop0.uop == UOP_POP,
          sl.uop0.req == {
            chip: "CELL",
            key: sl.pre.fp,
            a: KIND_FRAME,
            b: sl.post.topVal,
            c: sl.post.fp
          },
          sl.post.topLink == sl.post.fp
        },
        (sl.ruleId == RULE_REWRITE_TOP) implies and {
          sl.uop0.uop == UOP_MOVE,
          sl.uop0.req == NO_REQ,
          sl.post.fp == sl.pre.fp,
          sl.post.len == sl.pre.len
        }
      }
    )

  // Exact CORE decode, PC transition, and register transition per rule (C-CORE-1):
  pure def coreTransitionsValid(w: Witness): bool =
    w.slots.indices().forall(j =>
      {
      val sl = w.slots[j]
      if (not(sl.en))
        and { sl.ruleId == RULE_DISABLED, sl.decReq == NO_REQ, sl.post == sl.pre }


      else {
        val insn = progLookup(sl.pre.pc)
        and {
          sl.decClass == computeDecClass(sl.pre),
          sl.ruleId == sl.decClass,
          sl.decReq == decRomTuple(sl.decClass),
          sl.uop0.uop == decRomUop0(sl.decClass),
          sl.uop1.uop == decRomUop1(sl.decClass),
          if (sl.ruleId == RULE_PUBLISH)
            sl.post == withHalted(sl.pre, true)
          else if (sl.ruleId == RULE_FETCH_ALU) and {
            sl.uop0.req == {
              chip: "PROG",
              key: sl.pre.pc,
              a: insn.opcode,
              b: insn.imm,
              c: insn.targetPc
            },
            sl.uop1.req == {
              chip: "ALU",
              key: packAluKey(ALU_ADD, sl.post.aluFlag),
              a: sl.pre.acc,
              b: insn.imm,
              c: sl.post.acc
            },
            sl.post.pc == insn.targetPc,
            sl.post.fp == sl.pre.fp,
            sl.post.topVal == sl.pre.topVal,
            sl.post.len == sl.pre.len,
            not(sl.post.halted)
          } else if (sl.ruleId == RULE_APPEND_MOVE) and {
            sl.uop0.req == {
              chip: "CELL",
              key: sl.pre.len,
              a: KIND_DATA,
              b: sl.pre.acc,
              c: 0
            },
            sl.uop1.req == NO_REQ,
            sl.post.acc == sl.pre.acc + 1,
            sl.post.pc == insn.targetPc,
            sl.post.fp == sl.pre.fp,
            sl.post.topVal == sl.pre.topVal,
            sl.post.len == sl.pre.len + 1,
            not(sl.post.halted)
          } else if (sl.ruleId == RULE_LOAD_ALU) and {
            sl.uop0.req.chip == "CELL",
            sl.uop0.req.key == sl.pre.len - 1,
            sl.uop1.req == {
              chip: "ALU",
              key: packAluKey(ALU_ADD, sl.post.aluFlag),
              a: sl.pre.acc,
              b: sl.uop0.req.b,
              c: sl.post.acc
            },
            sl.post.pc == insn.targetPc,
            sl.post.fp == sl.pre.fp,
            sl.post.topVal == sl.pre.topVal,
            sl.post.len == sl.pre.len,
            not(sl.post.halted)
          } else if (sl.ruleId == RULE_PUSH) and {
            sl.uop0.req == {
              chip: "CELL",
              key: sl.pre.len,
              a: KIND_FRAME,
              b: sl.pre.topVal,
              c: sl.pre.fp
            },
            sl.uop1.req == NO_REQ,
            sl.post.fp == sl.pre.len,
            sl.post.topVal == insn.imm,
            sl.post.topLink == sl.pre.len,
            sl.post.len == sl.pre.len + 1,
            sl.post.pc == insn.targetPc,
            sl.post.acc == sl.pre.acc,
            not(sl.post.halted)
          } else if (sl.ruleId == RULE_POP) and {
            sl.uop0.req == {
              chip: "CELL",
              key: sl.pre.fp,
              a: KIND_FRAME,
              b: sl.post.topVal,
              c: sl.post.fp
            },
            sl.uop1.req == NO_REQ,
            sl.post.topLink == sl.post.fp,
            sl.post.acc == sl.pre.acc + sl.post.topVal,
            sl.post.len == sl.pre.len,
            sl.post.pc == insn.targetPc,
            not(sl.post.halted)
          } else and {
            // RULE_REWRITE_TOP
            sl.post.topVal == insn.imm,
            sl.post.fp == sl.pre.fp,
            sl.post.topLink == sl.pre.topLink,
            sl.post.len == sl.pre.len,
            sl.post.pc == insn.targetPc,
            sl.post.acc == sl.pre.acc,
            not(sl.post.halted)
          }
      }
      }
      }
    )

  // Slot contract (enable prefix, chain, publication last, rulesD, steps, outcome)
  pure def slotContractValid(w: Witness): bool = and {
    w.slots.length() == S,
    w.rulesD.length() == S,
    w.slots[0].en,
    w.slots.indices().forall(j =>
      j + 1 == S or not(w.slots[j + 1].en) or w.slots[j].en
    ),
    w.slots.indices().forall(j =>
      j + 1 == S or not(w.slots[j + 1].en) or not(w.slots[j].post.halted)
    ),
    w.slots.indices().forall(j =>
      j + 1 == S or w.slots[j].post == w.slots[j + 1].pre
    ),
    w.pre == w.slots[0].pre,
    w.post == w.slots[S - 1].post,
    w.slots.indices().forall(j =>
      w.rulesD[j] == (if (w.slots[j].en) w.slots[j].ruleId else RULE_DISABLED)
    ),
    w.steps == countEnabledSlots(w.slots),
    w.halted == w.post.halted
  }

  // ---------------------------------------------------------------- Acceptance Predicates
  // 1. acceptsAsWritten(w):
  //    Models the circuit design as written in study.md:
  //    - Set inclusion on the bus (lookup_any of bus columns on themselves)
  //    - PAGE opens against prog_root / pre.memRoot WITHOUT `idx < base` on CELL
  //    - MEM rows hold (CELL, addr, a, b, c) WITHOUT enforcing active prefix == len_post - base
  //      or preventing PAGE from answering CELL requests >= base
  //    - Micro-ops may stop when a chip fills (including mid-step)
  //    - Top-frame rewrite may append with the same link (study.md 2.4)
  pure def acceptsAsWritten(w: Witness): bool = and {
    slotContractValid(w),
    stepMidStepAllowed(w),
    coreReqBus(w).subseteq(ungatedSelfBus(w)),
    pageValidAsWritten(w),
    w.memTable.length() <= N_MEM,
    w.aluTable.length() <= N_ALU,
    w.hashTable.length() <= N_HASH
  }

  // 2. acceptsHardened(w):
  //    Set inclusion WITHOUT multiplicities, strengthened with the exact constraints required
  //    for soundness:
  //    - C-BUS-1:   w.extraBusRows.forall(t => t == NO_REQ) and coreReqBus(w).subseteq(gatedProvBus(w))
  //    - C-PAGE-1:  pageValidAsWritten(w) and pageDomainSeparated(w) (CELL in PAGE has idx < pre.len)
  //    - C-MEM-1:   memAddressFixed(w) and memActivePrefixBound(w) and memRootFoldValid(w)
  //    - C-MEM-2/3: allSubSlotsLenAndCausal(w) (strict per-append len increment & causal a < len_t)
  //    - C-STACK-1: stackDisciplineValid(w)
  //    - C-ALU-1:   aluTableValid(w)
  //    - C-HASH-1:  hashTableValid(w)
  //    - C-CORE-1:  stepAtomicMicroOps(w) and coreTransitionsValid(w)
  pure def acceptsHardened(w: Witness): bool = and {
    slotContractValid(w),
    stepAtomicMicroOps(w),
    w.extraBusRows.forall(t => t == NO_REQ),
    coreReqBus(w).subseteq(gatedProvBus(w)),
    pageValidAsWritten(w),
    pageDomainSeparated(w),
    memAddressFixed(w),
    memActivePrefixBound(w),
    memRootFoldValid(w),
    allSubSlotsLenAndCausal(w),
    stackDisciplineValid(w),
    aluTableValid(w),
    hashTableValid(w),
    coreTransitionsValid(w)
  }

  // 3. Parameterized accepts(w) for mutation / invariant model checking
  pure def accepts(w: Witness): bool =
    if (MUTANT == "as_written") acceptsAsWritten(w)
    else and {
      slotContractValid(w),
      if (MUTANT == "gap_mid_step") stepMidStepAllowed(w) else stepAtomicMicroOps(w),
      if (MUTANT == "gap_forged_bus")
        coreReqBus(w).subseteq(ungatedSelfBus(w))
      else
        (w.extraBusRows.forall(t => t == NO_REQ) and coreReqBus(w).subseteq(gatedProvBus(w))),
      pageValidAsWritten(w),
      (MUTANT == "gap_stale_read" or MUTANT == "gap_wrong_chip" or MUTANT == "gap_overflow")
        or pageDomainSeparated(w),
      (MUTANT == "gap_dup_mem_addr") or memAddressFixed(w),
      (MUTANT == "gap_unrequested_mem" or MUTANT == "gap_unwritten_read" or MUTANT == "gap_overflow" or MUTANT == "gap_row_replay")
        or memActivePrefixBound(w),
      (MUTANT == "gap_unrequested_mem" or MUTANT == "gap_overflow" or MUTANT == "gap_row_replay" or MUTANT == "gap_mid_step" or MUTANT == "gap_stack_rewrite")
        or memRootFoldValid(w),
      (MUTANT == "gap_unwritten_read" or MUTANT == "gap_row_replay" or MUTANT == "gap_stack_rewrite")
        or allSubSlotsLenAndCausal(w),
      (MUTANT == "gap_stack_rewrite" or MUTANT == "gap_forged_link")
        or stackDisciplineValid(w),
      (MUTANT == "gap_wrong_alu") or aluTableValid(w),
      hashTableValid(w),
      (MUTANT == "gap_wrong_pc" or MUTANT == "gap_unwritten_read" or MUTANT == "gap_row_replay" or MUTANT == "gap_mid_step" or MUTANT == "gap_stack_rewrite" or MUTANT == "gap_forged_link")
        or coreTransitionsValid(w)
    }

  // Soundness check for any witness w against the reference machine
  pure def isSoundWitness(w: Witness): bool = and {
    1 <= w.steps and w.steps <= S,
    w.rulesD == refRules(w.pre, w.steps),
    w.post == refPost(w.pre, w.steps),
    refTerminalLast(w.pre, w.steps),
    w.halted == refPost(w.pre, w.steps).halted
  }

  // ---------------------------------------------------------------- Honest Witness Builder
  pure def mkHonestSubSlots(pre: CoreRegs, en: bool): BuiltStep =
    val insn = progLookup(pre.pc)
    val rId = computeDecClass(pre)
    val dReq = decRomTuple(rId)
    val disSub = { uopEn: false, uop: UOP_HALT, preRegs: pre, postRegs: pre, req: NO_REQ }
    // PUBLISH
    val pubPost = withHalted(pre, true)
    val pubSub0 = { uopEn: true, uop: UOP_HALT, preRegs: pre, postRegs: pubPost, req: NO_REQ }
    val pubSub1 = { uopEn: true, uop: UOP_HALT, preRegs: pubPost, postRegs: pubPost, req: NO_REQ }
    // FETCH_ALU
    val zFetch = evalAluVal(ALU_ADD, pre.acc, insn.imm)
    val flFetch = evalAluFlag(ALU_ADD, pre.acc, insn.imm)
    val fetchPost = withAccPcFlag(pre, zFetch, insn.targetPc, flFetch)
    val fetchSub0 = {
      uopEn: true,
      uop: UOP_FETCH,
      preRegs: pre,
      postRegs: pre,
      req: { chip: "PROG", key: pre.pc, a: insn.opcode, b: insn.imm, c: insn.targetPc }
    }
    val fetchSub1 = {
      uopEn: true,
      uop: UOP_ALU,
      preRegs: pre,
      postRegs: fetchPost,
      req: { chip: "ALU", key: packAluKey(ALU_ADD, flFetch), a: pre.acc, b: insn.imm, c: zFetch }
    }
    // APPEND_MOVE
    val appRoot = poseidonHash(DOM_MEM_FOLD, pre.memRoot, cellDigest(pre.len, KIND_DATA, pre.acc, 0))
    val appMid = withLenRoot(pre, pre.len + 1, appRoot)
    val appPost = withAccPcFlag(appMid, pre.acc + 1, insn.targetPc, pre.aluFlag)
    val appSub0 = {
      uopEn: true,
      uop: UOP_APPEND,
      preRegs: pre,
      postRegs: appMid,
      req: { chip: "CELL", key: pre.len, a: KIND_DATA, b: pre.acc, c: 0 }
    }
    val appSub1 = { uopEn: true, uop: UOP_MOVE, preRegs: appMid, postRegs: appPost, req: NO_REQ }
    // LOAD_ALU
    val readAddr = pre.len - 1
    val loadedVal = if (readAddr < BASE_LEN) initMemCell(readAddr).payload else (pre.acc - 1)
    val loadedKind = if (readAddr < BASE_LEN) initMemCell(readAddr).kind else KIND_DATA
    val loadedLink = if (readAddr < BASE_LEN) initMemCell(readAddr).linkAddr else 0
    val zLoad = evalAluVal(ALU_ADD, pre.acc, loadedVal)
    val flLoad = evalAluFlag(ALU_ADD, pre.acc, loadedVal)
    val loadPost = withAccPcFlag(pre, zLoad, insn.targetPc, flLoad)
    val loadSub0 = {
      uopEn: true,
      uop: UOP_LOAD,
      preRegs: pre,
      postRegs: pre,
      req: { chip: "CELL", key: readAddr, a: loadedKind, b: loadedVal, c: loadedLink }
    }
    val loadSub1 = {
      uopEn: true,
      uop: UOP_ALU,
      preRegs: pre,
      postRegs: loadPost,
      req: { chip: "ALU", key: packAluKey(ALU_ADD, flLoad), a: pre.acc, b: loadedVal, c: zLoad }
    }
    // PUSH
    val pushRoot = poseidonHash(DOM_MEM_FOLD, pre.memRoot, cellDigest(pre.len, KIND_FRAME, pre.topVal, pre.fp))
    val pushMid = withStackState(pre, pre.len, pre.topVal, pre.len, pre.len + 1, pushRoot, pre.acc, pre.pc)
    val pushPost = withStackState(pre, pre.len, insn.imm, pre.len, pre.len + 1, pushRoot, pre.acc, insn.targetPc)
    val pushSub0 = {
      uopEn: true,
      uop: UOP_PUSH,
      preRegs: pre,
      postRegs: pushMid,
      req: { chip: "CELL", key: pre.len, a: KIND_FRAME, b: pre.topVal, c: pre.fp }
    }
    val pushSub1 = { uopEn: true, uop: UOP_MOVE, preRegs: pushMid, postRegs: pushPost, req: NO_REQ }
    // POP
    val restVal = if (pre.fp < BASE_LEN) initMemCell(pre.fp).payload else 10
    val restLink = if (pre.fp < BASE_LEN) initMemCell(pre.fp).linkAddr else 0
    val popMid = withStackState(pre, restLink, restVal, restLink, pre.len, pre.memRoot, pre.acc, pre.pc)
    val popPost = withStackState(pre, restLink, restVal, restLink, pre.len, pre.memRoot, pre.acc + restVal, insn.targetPc)
    val popSub0 = {
      uopEn: true,
      uop: UOP_POP,
      preRegs: pre,
      postRegs: popMid,
      req: { chip: "CELL", key: pre.fp, a: KIND_FRAME, b: restVal, c: restLink }
    }
    val popSub1 = { uopEn: true, uop: UOP_MOVE, preRegs: popMid, postRegs: popPost, req: NO_REQ }
    // REWRITE_TOP
    val rewMid = withStackState(pre, pre.fp, insn.imm, pre.topLink, pre.len, pre.memRoot, pre.acc, pre.pc)
    val rewPost = withStackState(pre, pre.fp, insn.imm, pre.topLink, pre.len, pre.memRoot, pre.acc, insn.targetPc)
    val rewSub0 = { uopEn: true, uop: UOP_MOVE, preRegs: pre, postRegs: rewMid, req: NO_REQ }
    val rewSub1 = { uopEn: true, uop: UOP_MOVE, preRegs: rewMid, postRegs: rewPost, req: NO_REQ }
    if (not(en))
      { uop0: disSub, uop1: disSub, post: pre, ruleId: RULE_DISABLED, decReq: NO_REQ }
    else if (rId == RULE_PUBLISH)
      { uop0: pubSub0, uop1: pubSub1, post: pubPost, ruleId: rId, decReq: dReq }
    else if (rId == RULE_FETCH_ALU)
      { uop0: fetchSub0, uop1: fetchSub1, post: fetchPost, ruleId: rId, decReq: dReq }
    else if (rId == RULE_APPEND_MOVE)
      { uop0: appSub0, uop1: appSub1, post: appPost, ruleId: rId, decReq: dReq }
    else if (rId == RULE_LOAD_ALU)
      { uop0: loadSub0, uop1: loadSub1, post: loadPost, ruleId: rId, decReq: dReq }
    else if (rId == RULE_PUSH)
      { uop0: pushSub0, uop1: pushSub1, post: pushPost, ruleId: rId, decReq: dReq }
    else if (rId == RULE_POP)
      { uop0: popSub0, uop1: popSub1, post: popPost, ruleId: rId, decReq: dReq }
    else
      { uop0: rewSub0, uop1: rewSub1, post: rewPost, ruleId: rId, decReq: dReq }

  pure def mkHonestWitness(pc0: int, acc0: int, kSteps: int): Witness =
    val s0 = initArch(pc0, acc0).regs
    val sls = range(0, S).foldl([], (l, j) =>
      val pre = if (j == 0) s0 else l[j - 1].post
      val en = (j < kSteps) and not(pre.halted)
      val built = mkHonestSubSlots(pre, en)
      l.append({
        en: en,
        decClass: built.ruleId,
        ruleId: built.ruleId,
        decReq: built.decReq,
        uop0: built.uop0,
        uop1: built.uop1,
        pre: pre,
        post: built.post
      })
    )
    val postFinal = sls[S - 1].post
    val rD = sls.foldl([], (l, sl) => l.append(if (sl.en) sl.ruleId else RULE_DISABLED))
    val stCount = countEnabledSlots(sls)
    val r0 = sls[0].uop0.req
    val mRow0 =
      if (postFinal.len > BASE_LEN)
        { active: true, rowIdx: 0, addr: BASE_LEN, a: r0.a, b: r0.b, c: r0.c }
      else
        { active: false, rowIdx: 0, addr: BASE_LEN, a: 0, b: 0, c: 0 }
    val mRow1 = { active: false, rowIdx: 1, addr: BASE_LEN + 1, a: 0, b: 0, c: 0 }
    val pRow0 = {
      active: true,
      tag: "PROG",
      idx: 6,
      a: progLookup(6).opcode,
      b: progLookup(6).imm,
      c: progLookup(6).targetPc,
      rootUsed: PROG_ROOT,
      pathValid: true
    }
    val pRow1 = {
      active: true,
      tag: "CELL",
      idx: 0,
      a: initMemCell(0).kind,
      b: initMemCell(0).payload,
      c: initMemCell(0).linkAddr,
      rootUsed: INIT_MEM_ROOT,
      pathValid: true
    }
    val pRow2 = {
      active: true,
      tag: "CELL",
      idx: 1,
      a: initMemCell(1).kind,
      b: initMemCell(1).payload,
      c: initMemCell(1).linkAddr,
      rootUsed: INIT_MEM_ROOT,
      pathValid: true
    }
    val aRow0 = {
      active: true,
      opKind: ALU_ADD,
      flagBit: evalAluFlag(ALU_ADD, acc0 + 1, acc0),
      x: acc0 + 1,
      y: acc0,
      z: evalAluVal(ALU_ADD, acc0 + 1, acc0),
      rangeOk: true
    }
    val aRow1 = {
      active: true,
      opKind: ALU_ADD,
      flagBit: evalAluFlag(ALU_ADD, acc0, 15),
      x: acc0,
      y: 15,
      z: evalAluVal(ALU_ADD, acc0, 15),
      rangeOk: true
    }
    val aRow2 = {
      active: true,
      opKind: ALU_ADD,
      flagBit: evalAluFlag(ALU_ADD, acc0, 50),
      x: acc0,
      y: 50,
      z: evalAluVal(ALU_ADD, acc0, 50),
      rangeOk: true
    }
    val hRow0 = {
      active: mRow0.active,
      domId: DOM_MEM_FOLD,
      in0: INIT_MEM_ROOT,
      in1: cellDigest(mRow0.addr, mRow0.a, mRow0.b, mRow0.c),
      outHash: poseidonHash(DOM_MEM_FOLD, INIT_MEM_ROOT, cellDigest(mRow0.addr, mRow0.a, mRow0.b, mRow0.c))
    }
    {
      slots: sls,
      pageTable: [pRow0, pRow1, pRow2],
      memTable: [mRow0, mRow1],
      aluTable: [aRow0, aRow1, aRow2],
      hashTable: [hRow0],
      extraBusRows: Set(NO_REQ),
      rulesD: rD,
      steps: stCount,
      pre: s0,
      post: postFinal,
      halted: postFinal.halted
    }

  // Helper to build a S-slot list from 1 or 2 custom active slots followed by disabled slots
  pure def padSlots1(sl0: StepSlot): List[StepSlot] =
    range(0, S).foldl([], (l, j) =>
      if (j == 0) l.append(sl0)
      else l.append(mkDisabledSlotAt(sl0.post))
    )

  pure def padSlots2(sl0: StepSlot, sl1: StepSlot): List[StepSlot] =
    range(0, S).foldl([], (l, j) =>
      if (j == 0) l.append(sl0)
      else if (j == 1) l.append(sl1)
      else l.append(mkDisabledSlotAt(sl1.post))
    )

  // ---------------------------------------------------------------- 13 Adversarial Counterexample Witnesses
  // 1. Forged bus tuple (self-lookup without disjoint provider gating)
  pure val cex_forged_bus_tuple: Witness =
    val base = mkHonestWitness(6, 10, 1)
    val forgedPost = withAccPcFlag(base.post, 199, base.post.pc, base.post.aluFlag)
    val forgedTuple = { chip: "ALU", key: packAluKey(ALU_ADD, 0), a: 10, b: 15, c: 199 }
    val forgedUop1 = {
      uopEn: true,
      uop: UOP_ALU,
      preRegs: base.slots[0].uop1.preRegs,
      postRegs: forgedPost,
      req: forgedTuple
    }
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: base.slots[0].uop0,
      uop1: forgedUop1,
      pre: base.pre,
      post: forgedPost
    }
    {
      slots: padSlots1(sl0),
      pageTable: base.pageTable,
      memTable: base.memTable,
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: Set(NO_REQ, forgedTuple),
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: forgedPost,
      halted: forgedPost.halted
    }

  // 2. Unrequested chip row (orphan active MEM row polluting memRoot)
  pure val cex_unrequested_chip_row: Witness =
    val base = mkHonestWitness(0, 10, 1)
    val orphanRow = { active: true, rowIdx: 1, addr: BASE_LEN + 1, a: KIND_DATA, b: 999, c: 0 }
    val pollutedRoot = poseidonHash(DOM_MEM_FOLD, base.post.memRoot, cellDigest(BASE_LEN + 1, KIND_DATA, 999, 0))
    val badMid = withLenRoot(base.slots[0].uop0.postRegs, base.slots[0].uop0.postRegs.len, pollutedRoot)
    val badPost = withLenRoot(base.post, base.post.len, pollutedRoot)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: base.slots[0].uop0.uop,
        preRegs: base.pre,
        postRegs: badMid,
        req: base.slots[0].uop0.req
      },
      uop1: {
        uopEn: true,
        uop: base.slots[0].uop1.uop,
        preRegs: badMid,
        postRegs: badPost,
        req: base.slots[0].uop1.req
      },
      pre: base.pre,
      post: badPost
    }
    {
      slots: padSlots1(sl0),
      pageTable: base.pageTable,
      memTable: [base.memTable[0], orphanRow],
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: badPost,
      halted: badPost.halted
    }

  // 3. Request answered by the wrong chip (PAGE answers APPEND at addr = BASE_LEN with 0, leaving MEM row free to forge 77)
  pure val cex_wrong_chip_answer: Witness =
    val base = mkHonestWitness(0, 0, 2)
    val pageHijackRow = {
      active: true,
      tag: "CELL",
      idx: BASE_LEN,
      a: KIND_DATA,
      b: 0,
      c: 0,
      rootUsed: INIT_MEM_ROOT,
      pathValid: true
    }
    val forgedMemRow0 = { active: true, rowIdx: 0, addr: BASE_LEN, a: KIND_DATA, b: 77, c: 0 }
    val forgedRoot = poseidonHash(DOM_MEM_FOLD, INIT_MEM_ROOT, cellDigest(BASE_LEN, KIND_DATA, 77, 0))
    val s0Mid = withLenRoot(base.slots[0].uop0.postRegs, base.slots[0].uop0.postRegs.len, forgedRoot)
    val s0Post = withLenRoot(base.slots[0].post, base.slots[0].post.len, forgedRoot)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: base.slots[0].uop0.uop,
        preRegs: base.pre,
        postRegs: s0Mid,
        req: base.slots[0].uop0.req
      },
      uop1: {
        uopEn: true,
        uop: base.slots[0].uop1.uop,
        preRegs: s0Mid,
        postRegs: s0Post,
        req: base.slots[0].uop1.req
      },
      pre: base.pre,
      post: s0Post
    }
    val s1Post = withAccPcFlag(withLenRoot(base.slots[1].post, base.slots[1].post.len, forgedRoot), 78, base.slots[1].post.pc, 0)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_LOAD,
        preRegs: s0Post,
        postRegs: s0Post,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_DATA, b: 77, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_ALU,
        preRegs: s0Post,
        postRegs: s1Post,
        req: { chip: "ALU", key: packAluKey(ALU_ADD, 0), a: 1, b: 77, c: 78 }
      },
      pre: s0Post,
      post: s1Post
    }
    {
      slots: padSlots2(sl0, sl1),
      pageTable: [base.pageTable[0], base.pageTable[1], base.pageTable[2], pageHijackRow],
      memTable: [forgedMemRow0, base.memTable[1]],
      aluTable: [base.aluTable[0], { active: true, opKind: ALU_ADD, flagBit: 0, x: 1, y: 77, z: 78, rangeOk: true }, base.aluTable[2]],
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: s1Post,
      halted: s1Post.halted
    }

  // 4. Stale memory read (Step 0 appends 10 at addr=2; Step 1's LOAD 2 reads pre-leaf 0 from PAGE!)
  pure val cex_stale_memory_read: Witness =
    val base = mkHonestWitness(0, 10, 2)
    val stalePageRow = {
      active: true,
      tag: "CELL",
      idx: BASE_LEN,
      a: KIND_DATA,
      b: 0,
      c: 0,
      rootUsed: INIT_MEM_ROOT,
      pathValid: true
    }
    val stalePost = withAccPcFlag(base.slots[1].post, 11, base.slots[1].post.pc, 0)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_LOAD,
        preRegs: base.slots[1].pre,
        postRegs: base.slots[1].pre,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_DATA, b: 0, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_ALU,
        preRegs: base.slots[1].pre,
        postRegs: stalePost,
        req: { chip: "ALU", key: packAluKey(ALU_ADD, 0), a: 11, b: 0, c: 11 }
      },
      pre: base.slots[1].pre,
      post: stalePost
    }
    {
      slots: padSlots2(base.slots[0], sl1),
      pageTable: [base.pageTable[0], base.pageTable[1], base.pageTable[2], stalePageRow],
      memTable: base.memTable,
      aluTable: [base.aluTable[0], { active: true, opKind: ALU_ADD, flagBit: 0, x: 11, y: 0, z: 11, rangeOk: true }, base.aluTable[2]],
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: stalePost,
      halted: stalePost.halted
    }

  // 5. Read of never-written cell (LOAD reads unrequested MEM row 1 at addr=3)
  pure val cex_unwritten_cell_read: Witness =
    val base = mkHonestWitness(0, 10, 2)
    val unwrittenMemRow1 = { active: true, rowIdx: 1, addr: BASE_LEN + 1, a: KIND_DATA, b: 88, c: 0 }
    val s0Mid = withLenRoot(base.slots[0].uop0.postRegs, 4, base.slots[0].uop0.postRegs.memRoot)
    val s0Post = withLenRoot(base.slots[0].post, 4, base.slots[0].post.memRoot)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: base.slots[0].uop0.uop,
        preRegs: base.pre,
        postRegs: s0Mid,
        req: base.slots[0].uop0.req
      },
      uop1: {
        uopEn: true,
        uop: base.slots[0].uop1.uop,
        preRegs: s0Mid,
        postRegs: s0Post,
        req: base.slots[0].uop1.req
      },
      pre: base.pre,
      post: s0Post
    }
    val s1Post = withAccPcFlag(withLenRoot(base.slots[1].post, 4, base.slots[1].post.memRoot), 99, base.slots[1].post.pc, 0)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_LOAD,
        preRegs: s0Post,
        postRegs: s0Post,
        req: { chip: "CELL", key: BASE_LEN + 1, a: KIND_DATA, b: 88, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_ALU,
        preRegs: s0Post,
        postRegs: s1Post,
        req: { chip: "ALU", key: packAluKey(ALU_ADD, 0), a: 11, b: 88, c: 99 }
      },
      pre: s0Post,
      post: s1Post
    }
    {
      slots: padSlots2(sl0, sl1),
      pageTable: base.pageTable,
      memTable: [base.memTable[0], unwrittenMemRow1],
      aluTable: [base.aluTable[0], { active: true, opKind: ALU_ADD, flagBit: 0, x: 11, y: 88, z: 99, rangeOk: true }, base.aluTable[2]],
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: s1Post,
      halted: s1Post.halted
    }

  // 6. Wrong ALU result (ALU chip row provides wrong z = 123 and wrong flagBit = 1)
  pure val cex_wrong_alu_result: Witness =
    val base = mkHonestWitness(6, 10, 1)
    val badAluRow = { active: true, opKind: ALU_ADD, flagBit: 1, x: 10, y: 15, z: 123, rangeOk: false }
    val badPost = withAccPcFlag(base.post, 123, base.post.pc, 1)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: base.slots[0].uop0,
      uop1: {
        uopEn: true,
        uop: UOP_ALU,
        preRegs: base.slots[0].uop1.preRegs,
        postRegs: badPost,
        req: { chip: "ALU", key: packAluKey(ALU_ADD, 1), a: 10, b: 15, c: 123 }
      },
      pre: base.pre,
      post: badPost
    }
    {
      slots: padSlots1(sl0),
      pageTable: base.pageTable,
      memTable: base.memTable,
      aluTable: [base.aluTable[0], badAluRow, base.aluTable[2]],
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: badPost,
      halted: badPost.halted
    }

  // 7. Wrong next PC (CORE jumps to pc = 0 instead of targetPc = 2)
  pure val cex_wrong_next_pc: Witness =
    val base = mkHonestWitness(6, 10, 1)
    val badPost = withAccPcFlag(base.post, base.post.acc, 0, base.post.aluFlag)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: base.slots[0].uop0,
      uop1: {
        uopEn: true,
        uop: base.slots[0].uop1.uop,
        preRegs: base.slots[0].uop1.preRegs,
        postRegs: badPost,
        req: base.slots[0].uop1.req
      },
      pre: base.pre,
      post: badPost
    }
    {
      slots: padSlots1(sl0),
      pageTable: base.pageTable,
      memTable: base.memTable,
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: badPost,
      halted: badPost.halted
    }

  // 8. Chip-table overflow (len advances past BASE_LEN + N_MEM, with overflowed append answered by PAGE)
  pure val cex_chip_table_overflow: Witness =
    val base = mkHonestWitness(0, 0, 1)
    val pageOverflowRow = {
      active: true,
      tag: "CELL",
      idx: BASE_LEN,
      a: KIND_DATA,
      b: 0,
      c: 0,
      rootUsed: INIT_MEM_ROOT,
      pathValid: true
    }
    val overMid = withLenRoot(base.slots[0].uop0.postRegs, BASE_LEN + N_MEM + 1, base.slots[0].uop0.postRegs.memRoot)
    val overPost = withLenRoot(base.slots[0].post, BASE_LEN + N_MEM + 1, base.slots[0].post.memRoot)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: base.slots[0].uop0.uop,
        preRegs: base.pre,
        postRegs: overMid,
        req: base.slots[0].uop0.req
      },
      uop1: {
        uopEn: true,
        uop: base.slots[0].uop1.uop,
        preRegs: overMid,
        postRegs: overPost,
        req: base.slots[0].uop1.req
      },
      pre: base.pre,
      post: overPost
    }
    {
      slots: padSlots1(sl0),
      pageTable: [base.pageTable[0], base.pageTable[1], base.pageTable[2], pageOverflowRow],
      memTable: base.memTable,
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: overPost,
      halted: overPost.halted
    }

  // 9. Replay of a MEM row across steps (two PUSH steps both match the single MEM row at addr = BASE_LEN)
  pure val cex_row_replay_across_steps: Witness =
    val base = mkHonestWitness(3, 10, 2)
    val s0Pre = base.slots[0].pre
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_PUSH,
        preRegs: s0Pre,
        postRegs: s0Pre,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_FRAME, b: 10, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_MOVE,
        preRegs: s0Pre,
        postRegs: s0Pre,
        req: NO_REQ
      },
      pre: s0Pre,
      post: s0Pre
    }
    val s1Built = mkHonestSubSlots(s0Pre, true)
    val sl1 = {
      en: true,
      decClass: s1Built.ruleId,
      ruleId: s1Built.ruleId,
      decReq: s1Built.decReq,
      uop0: s1Built.uop0,
      uop1: s1Built.uop1,
      pre: s0Pre,
      post: s1Built.post
    }
    val sls = padSlots2(sl0, sl1)
    val rD = sls.foldl([], (l, sl) => l.append(if (sl.en) sl.ruleId else RULE_DISABLED))
    {
      slots: sls,
      pageTable: base.pageTable,
      memTable: base.memTable,
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: rD,
      steps: 2,
      pre: base.pre,
      post: s1Built.post,
      halted: s1Built.post.halted
    }

  // 10. Leaf ends mid-step (uop0 APPEND executes and increments len, but uop1 MOVE is disabled mid-step)
  pure val cex_mid_step_leaf_end: Witness =
    val base = mkHonestWitness(0, 10, 1)
    val midRegs = base.slots[0].uop0.postRegs
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: base.slots[0].uop0,
      uop1: {
        uopEn: false,
        uop: UOP_MOVE,
        preRegs: midRegs,
        postRegs: midRegs,
        req: NO_REQ
      },
      pre: base.pre,
      post: midRegs
    }
    {
      slots: padSlots1(sl0),
      pageTable: base.pageTable,
      memTable: base.memTable,
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: midRegs,
      halted: midRegs.halted
    }

  // 11. Duplicate MEM rows for one address (MEM row 0 and row 1 both have addr = 2 with values 10 and 55)
  pure val cex_duplicate_mem_addr: Witness =
    val base = mkHonestWitness(0, 10, 2)
    val dupRow1 = { active: true, rowIdx: 1, addr: BASE_LEN, a: KIND_DATA, b: 55, c: 0 }
    val s1Post = withAccPcFlag(base.slots[1].post, 66, base.slots[1].post.pc, 0)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_LOAD,
        preRegs: base.slots[1].pre,
        postRegs: base.slots[1].pre,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_DATA, b: 55, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_ALU,
        preRegs: base.slots[1].pre,
        postRegs: s1Post,
        req: { chip: "ALU", key: packAluKey(ALU_ADD, 0), a: 11, b: 55, c: 66 }
      },
      pre: base.slots[1].pre,
      post: s1Post
    }
    {
      slots: padSlots2(base.slots[0], sl1),
      pageTable: base.pageTable,
      memTable: [base.memTable[0], dupRow1],
      aluTable: [base.aluTable[0], { active: true, opKind: ALU_ADD, flagBit: 0, x: 11, y: 55, z: 66, rangeOk: true }, base.aluTable[2]],
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: s1Post,
      halted: s1Post.halted
    }

  // 12. Stack top-rewrite corruption (study.md 2.4 "a rewrite of the top appends with the same link" + 1.3 POP = LOAD fp)
  pure val cex_stack_top_rewrite_corrupt: Witness =
    val base = mkHonestWitness(5, 10, 2)
    val s0Pre = base.slots[0].pre
    val rewriteRoot = poseidonHash(DOM_MEM_FOLD, INIT_MEM_ROOT, cellDigest(BASE_LEN, KIND_FRAME, 25, 0))
    val s0Mid = withStackState(s0Pre, BASE_LEN, 25, 0, BASE_LEN + 1, rewriteRoot, s0Pre.acc, s0Pre.pc)
    val s0Post = withStackState(s0Pre, BASE_LEN, 25, 0, BASE_LEN + 1, rewriteRoot, s0Pre.acc, 4)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_APPEND,
        preRegs: s0Pre,
        postRegs: s0Mid,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_FRAME, b: 25, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_MOVE,
        preRegs: s0Mid,
        postRegs: s0Post,
        req: NO_REQ
      },
      pre: s0Pre,
      post: s0Post
    }
    val s1Mid = withStackState(s0Post, 0, 25, 0, s0Post.len, s0Post.memRoot, s0Post.acc, s0Post.pc)
    val s1Post = withStackState(s0Post, 0, 25, 0, s0Post.len, s0Post.memRoot, 35, 2)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_POP,
        preRegs: s0Post,
        postRegs: s1Mid,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_FRAME, b: 25, c: 0 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_MOVE,
        preRegs: s1Mid,
        postRegs: s1Post,
        req: NO_REQ
      },
      pre: s0Post,
      post: s1Post
    }
    val rewriteMemRow0 = { active: true, rowIdx: 0, addr: BASE_LEN, a: KIND_FRAME, b: 25, c: 0 }
    {
      slots: padSlots2(sl0, sl1),
      pageTable: base.pageTable,
      memTable: [rewriteMemRow0, base.memTable[1]],
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: s1Post,
      halted: s1Post.halted
    }

  // 13. Forged frame link on PUSH/POP (PUSH stores linkAddr = 1 pointing to a data cell instead of fp = 0)
  pure val cex_forged_frame_link: Witness =
    val base = mkHonestWitness(3, 10, 2)
    val forgedFrameRow0 = { active: true, rowIdx: 0, addr: BASE_LEN, a: KIND_FRAME, b: 10, c: 1 }
    val forgedRoot = poseidonHash(DOM_MEM_FOLD, INIT_MEM_ROOT, cellDigest(BASE_LEN, KIND_FRAME, 10, 1))
    val s0Pre = base.slots[0].pre
    val s0Mid = withLenRoot(base.slots[0].uop0.postRegs, base.slots[0].uop0.postRegs.len, forgedRoot)
    val s0Post = withLenRoot(base.slots[0].post, base.slots[0].post.len, forgedRoot)
    val sl0 = {
      en: true,
      decClass: base.slots[0].decClass,
      ruleId: base.slots[0].ruleId,
      decReq: base.slots[0].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_PUSH,
        preRegs: s0Pre,
        postRegs: s0Mid,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_FRAME, b: 10, c: 1 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_MOVE,
        preRegs: s0Mid,
        postRegs: s0Post,
        req: NO_REQ
      },
      pre: s0Pre,
      post: s0Post
    }
    val s1Mid = withStackState(s0Post, 1, 10, 1, s0Post.len, s0Post.memRoot, s0Post.acc, s0Post.pc)
    val s1Post = withStackState(s0Post, 1, 10, 1, s0Post.len, s0Post.memRoot, 20, 2)
    val sl1 = {
      en: true,
      decClass: base.slots[1].decClass,
      ruleId: base.slots[1].ruleId,
      decReq: base.slots[1].decReq,
      uop0: {
        uopEn: true,
        uop: UOP_POP,
        preRegs: s0Post,
        postRegs: s1Mid,
        req: { chip: "CELL", key: BASE_LEN, a: KIND_FRAME, b: 10, c: 1 }
      },
      uop1: {
        uopEn: true,
        uop: UOP_MOVE,
        preRegs: s1Mid,
        postRegs: s1Post,
        req: NO_REQ
      },
      pre: s0Post,
      post: s1Post
    }
    {
      slots: padSlots2(sl0, sl1),
      pageTable: base.pageTable,
      memTable: [forgedFrameRow0, base.memTable[1]],
      aluTable: base.aluTable,
      hashTable: base.hashTable,
      extraBusRows: base.extraBusRows,
      rulesD: base.rulesD,
      steps: base.steps,
      pre: base.pre,
      post: s1Post,
      halted: s1Post.halted
    }

  // ---------------------------------------------------------------- Completeness Witness
  // In study.md 2.4, a single step appends 6 to 12 tuples (or an S-step leaf appends > N_MEM tuples).
  // When a single step requires more MEM rows than N_MEM (e.g. 6 > N_MEM == 2), no witness can satisfy
  // acceptsHardened because memActivePrefixBound enforces (post.len - pre.len <= N_MEM).
  pure val completenessSingleStepExceedsMem: bool =
    val singleStepAppends = 6
    singleStepAppends > N_MEM

  // ---------------------------------------------------------------- State Machine for Model Checking
  pure val TAMPERS: Set[str] = Set(
    "none",
    "forged_bus_tuple",
    "unrequested_chip_row",
    "wrong_chip_answer",
    "stale_memory_read",
    "unwritten_cell_read",
    "wrong_alu_result",
    "wrong_next_pc",
    "chip_table_overflow",
    "row_replay_across_steps",
    "mid_step_leaf_end",
    "duplicate_mem_addr",
    "stack_top_rewrite_corrupt",
    "forged_frame_link"
  )

  pure def pickWitness(pc0: int, kSteps: int, tk: str): Witness =
    if (tk == "forged_bus_tuple") cex_forged_bus_tuple
    else if (tk == "unrequested_chip_row") cex_unrequested_chip_row
    else if (tk == "wrong_chip_answer") cex_wrong_chip_answer
    else if (tk == "stale_memory_read") cex_stale_memory_read
    else if (tk == "unwritten_cell_read") cex_unwritten_cell_read
    else if (tk == "wrong_alu_result") cex_wrong_alu_result
    else if (tk == "wrong_next_pc") cex_wrong_next_pc
    else if (tk == "chip_table_overflow") cex_chip_table_overflow
    else if (tk == "row_replay_across_steps") cex_row_replay_across_steps
    else if (tk == "mid_step_leaf_end") cex_mid_step_leaf_end
    else if (tk == "duplicate_mem_addr") cex_duplicate_mem_addr
    else if (tk == "stack_top_rewrite_corrupt") cex_stack_top_rewrite_corrupt
    else if (tk == "forged_frame_link") cex_forged_frame_link
    else mkHonestWitness(pc0, 10, kSteps)

  var w: Witness
  var acc: bool
  var tamper: str

  action sample = {
    nondet pc0 = Set(0, 2, 3, 5, 6).oneOf()
    nondet kSteps = Set(1, 2, S).oneOf()
    nondet tk = TAMPERS.oneOf()
    val cand = pickWitness(pc0, kSteps, tk)
    all {
      w' = cand,
      acc' = accepts(cand),
      tamper' = tk,
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Soundness Invariants
  val invStepsRange: bool = acc implies (1 <= w.steps and w.steps <= S)
  val invPrefix: bool = acc implies w.slots.indices().forall(j => w.slots[j].en == (j < w.steps))
  val invRules: bool = acc implies w.rulesD == refRules(w.pre, w.steps)
  val invPost: bool = acc implies w.post == refPost(w.pre, w.steps)
  val invTerminalLast: bool = acc implies refTerminalLast(w.pre, w.steps)
  val invOutcome: bool = acc implies w.halted == refPost(w.pre, w.steps).halted
  val invMemCap: bool = acc implies (w.post.len - w.pre.len <= N_MEM)
  val invSound: bool = and {
    invStepsRange,
    invPrefix,
    invRules,
    invPost,
    invTerminalLast,
    invOutcome,
    invMemCap,
  }
  val invTamperRefused: bool = (acc and MUTANT == "none") implies tamper == "none"

  // ---------------------------------------------------------------- Non-vacuity Reachability (expected to FAIL)
  val reachShort: bool = not(acc and w.steps < S)
  val reachTerminalShort: bool = not(acc and w.halted and w.steps < S)
  val reachFull: bool = not(acc and w.steps == S)
  val reachPushPop: bool = not(acc and w.pre.pc == 3 and w.steps >= 2)
}

// Hardened design instantiations (all soundness invariants HOLD)
module chipleafHardened { import chipleaf(S = 3, MUTANT = "none").* }
module chipleafS2 { import chipleaf(S = 2, MUTANT = "none").* }
module chipleafS4 { import chipleaf(S = 4, MUTANT = "none").* }

// Design as written in study.md (invSound FAILS on every adversarial witness)
module chipleafAsWritten { import chipleaf(S = 3, MUTANT = "as_written").* }

// Per-gap mutant instantiations (each drops one required constraint; invSound FAILS)
module chipleaf_gap_forged_bus { import chipleaf(S = 3, MUTANT = "gap_forged_bus").* }
module chipleaf_gap_unrequested_mem { import chipleaf(S = 3, MUTANT = "gap_unrequested_mem").* }
module chipleaf_gap_wrong_chip { import chipleaf(S = 3, MUTANT = "gap_wrong_chip").* }
module chipleaf_gap_stale_read { import chipleaf(S = 3, MUTANT = "gap_stale_read").* }
module chipleaf_gap_unwritten_read { import chipleaf(S = 3, MUTANT = "gap_unwritten_read").* }
module chipleaf_gap_wrong_alu { import chipleaf(S = 3, MUTANT = "gap_wrong_alu").* }
module chipleaf_gap_wrong_pc { import chipleaf(S = 3, MUTANT = "gap_wrong_pc").* }
module chipleaf_gap_overflow { import chipleaf(S = 3, MUTANT = "gap_overflow").* }
module chipleaf_gap_row_replay { import chipleaf(S = 3, MUTANT = "gap_row_replay").* }
module chipleaf_gap_mid_step { import chipleaf(S = 3, MUTANT = "gap_mid_step").* }
module chipleaf_gap_dup_mem_addr { import chipleaf(S = 3, MUTANT = "gap_dup_mem_addr").* }
module chipleaf_gap_stack_rewrite { import chipleaf(S = 3, MUTANT = "gap_stack_rewrite").* }
module chipleaf_gap_forged_link { import chipleaf(S = 3, MUTANT = "gap_forged_link").* }
