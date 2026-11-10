// Chiplet leaf of the Sherlock VM: CPU core, chips and one bus checked by set inclusion.
// Design: research/sherlock-processor-analogy-20261110/study.md (sections 1.3-1.5, 2.1-2.7, 7),
// checked against GP-1 in openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md.
// Note with results: research/sherlock-processor-analogy-20261110/quint-check.md.
//
// What is modelled
// - CORE: registers pc, r0, r1, len, fp, depth, status, out (study 1.3 `ctl`, `st`, `fp`, `len`,
//   a value register pair) carried across S step slots with the slot contract of leafSlots.qnt
//   (one enable per slot, enable prefix, chain, steps = number of enabled slots, publication last).
//   One step = FETCH, (ALU LT for LOAD), DEC lookup of the class, then the rule's micro-ops.
// - Abstract ISA (program rows are data behind prog_root): LIT, ADD, ALLOC, LOAD, PUSH, POP,
//   SETTOP (rewrite of the top frame), JNZ, HALT. Errors (LOAD out of range, POP or SETTOP on an
//   empty stack) only write `status`; the TRAP class fires on the next step and publishes (2.7).
// - Bus: every request is a tuple (chip, key, a, b, c). The table side is the union of the rows of
//   the provider regions PAGE, MEM, ALU, DEC plus the padding tuple of fixed-height regions.
//   Inclusion is set membership (halo2 lookup, study 1.4); nothing counts multiplicities.
// - Chips, each a row gate plus the bus interface:
//   PAGE (2.3): PROG rows equal to the committed program; CELL rows equal to the pre-leaf memory
//     commitment. The memory tree is parameterised as sparse (unallocated leaves open to a default
//     cell), the worst case of the open "tree shape of the memory root" (study section 7).
//   MEM (2.4): N_MEM rows, row k has key base + k with base = pre.len (fixed address column), free
//     payload. Appends request (CELL, len, x) and advance len; reads require addr < len.
//   ALU (2.5): rows (ALU, op, x, y, z) with z = f(op, x, y) on every active row.
//   DEC (2.2): fixed table class -> rule, at most one row per class.
//   HASH (2.6) is not a separate region: PAGE membership and the memory root are modelled as exact
//   maps (premise: Poseidon is collision resistant, Merkle openings bind index and value).
// - Memory: one write-once log. Frames are linked entries (value, link); PUSH appends
//   (v, fp), POP loads mem[fp] and follows the link, SETTOP loads mem[fp] and appends (v, link)
//   with fp moved to the new entry. The core does not cache the top frame (study 1.3 caches `top`;
//   the uncached form is the one in which reads are visible on the bus).
// - Post statement: registers plus the committed memory (pre cells ++ MEM rows below post.len).
//
// Reference: `refStep`, a machine with a mutable logical stack (list of frames) and the same
//   append-only data layout. Refinement compares registers, the committed memory and the stack.
//
// Base model = the design as written plus the constraints the study leaves implicit. Each such
// constraint is named D1..D8 at its line; quint-check.md lists them as required design changes,
// and the mutant that drops it shows the counterexample.
//
// Witness generation (as leafSlots.qnt): an honest witness built by running the core with a truth
// oracle, plus at most one tamper (a bus lie, a table row, a register or a slot shape). A mutant
// drops one constraint; it must let some witness through that an invariant refutes.
//
// Not modelled: field arithmetic and range checks (ints stand for range-checked values), HASH
// rows and Merkle paths, the cross-leaf node, the admission claim on prog_root (premise: the
// committed program is the canonical one), the 108 real rules (E0(c) is open).
module chipLeaf {
  const S: int
  const N_MEM: int
  const N_PAGE: int
  const N_ALU: int
  const MUTANT: str

  // ---------------------------------------------------------------- constants and types
  pure val P: int = 6   // program rows; the last row is HALT
  pure val T_PROG: int = 1
  pure val T_CELL: int = if (MUTANT == "tag_zero") 0 else 2
  pure val T_ALU: int = 3
  pure val T_DEC: int = 4
  pure val ALU_LT: int = 1
  pure val ALU_ADD: int = 2
  pure val NIL: int = -1

  pure val RUN: int = 0
  pure val HALTED: int = 1
  pure val ERR: int = 2
  pure val PUBERR: int = 3

  pure val OP_LIT: int = 1
  pure val OP_ADD: int = 2
  pure val OP_ALLOC: int = 3
  pure val OP_LOAD: int = 4
  pure val OP_PUSH: int = 5
  pure val OP_POP: int = 6
  pure val OP_SETTOP: int = 7
  pure val OP_JNZ: int = 8
  pure val OP_HALT: int = 9

  pure val R_LIT: int = 10
  pure val R_ADD: int = 20
  pure val R_ALLOC: int = 30
  pure val R_LOADOOB: int = 40
  pure val R_LOAD: int = 41
  pure val R_PUSH: int = 50
  pure val R_POP: int = 60
  pure val R_POPEMPTY: int = 61
  pure val R_SETTOP: int = 70
  pure val R_SETTOPEMPTY: int = 71
  pure val R_JNZ: int = 80
  pure val R_HALT: int = 90
  pure val R_PUBERR: int = 200

  type Tuple = { chip: int, key: int, a: int, b: int, c: int }
  type Cell = { a: int, b: int }
  type Ins = { op: int, x: int, y: int }
  type Regs = { pc: int, r0: int, r1: int, len: int, fp: int, depth: int, status: int, out: int }
  type Frame = { v: int, at: int }
  type RefSt = { pc: int, r0: int, r1: int, status: int, out: int, mem: List[Cell], stack: List[Frame] }
  type Slot = { en: bool, pre: Regs, bus: List[Tuple], post: Regs }
  type W = {
    prog: List[Ins], oldMem: List[Cell], pre: Regs, slots: List[Slot],
    page: Set[Tuple], alu: Set[Tuple], mem: List[Tuple], post: Regs, memOut: List[Cell], steps: int,
  }
  type Body = { ok: bool, reqs: List[Tuple], post: Regs }
  type CoreOut = { ok: bool, exp: List[Tuple], post: Regs }

  // D8: the padding row of every fixed-height region is the zero tuple, so no chip may use tag 0.
  pure val ZT: Tuple = { chip: 0, key: 0, a: 0, b: 0, c: 0 }
  pure val GARBAGE: Cell = { a: 5, b: 5 }
  pure val SPARSE_DEFAULT: Cell = { a: 0, b: 0 }
  pure val NOINS: Ins = { op: 0, x: 0, y: 0 }

  // ---------------------------------------------------------------- helpers
  pure def at(bus: List[Tuple], i: int): Tuple = if (0 <= i and i < bus.length()) bus[i] else ZT
  pure def cellAt(l: List[Cell], k: int): Cell = if (0 <= k and k < l.length()) l[k] else GARBAGE
  pure def progAt(prog: List[Ins], i: int): Ins = if (0 <= i and i < prog.length()) prog[i] else NOINS
  pure def progRow(prog: List[Ins], k: int): Tuple =
    val ins = progAt(prog, k)
    { chip: T_PROG, key: k, a: ins.op, b: ins.x, c: ins.y }
  pure def toSet(l: List[Tuple]): Set[Tuple] = l.foldl(Set(), (s, r) => s.union(Set(r)))
  pure def reg(s: Regs, x: int): int = if (x == 0) s.r0 else s.r1
  pure def setReg(s: Regs, x: int, v: int): Regs = if (x == 0) { ...s, r0: v } else { ...s, r1: v }
  pure def published(st: int): bool = st == HALTED or st == PUBERR
  pure def steppable(st: int): bool = st == RUN or st == ERR

  // ---------------------------------------------------------------- chip functions
  pure def aluF(op: int, x: int, y: int): int =
    if (op == ALU_LT) (if (x < y) 1 else 0) else if (op == ALU_ADD) (x + y) % 8 else -99

  pure val DEC_CLASSES: Set[int] = Set(10, 20, 30, 40, 41, 50, 60, 61, 70, 71, 80, 90, 200)
  pure def decF(cls: int): int = if (DEC_CLASSES.contains(cls)) cls else 0
  pure def altRule(r: int): int =
    if (r == 40) 41 else if (r == 41) 40 else if (r == 60) 61 else if (r == 61) 60
    else if (r == 70) 71 else if (r == 71) 70 else 999
  pure def decRow(cls: int, rule: int): Tuple = { chip: T_DEC, key: cls, a: rule, b: 0, c: 0 }
  // DEC obligation (2.2): one row per class. Mutant dec_dup adds a second row for three classes.
  pure val DEC_ROWS: Set[Tuple] =
    DEC_CLASSES.map(k => decRow(k, k)).union(
      if (MUTANT == "dec_dup") Set(decRow(41, 40), decRow(60, 61), decRow(70, 71)) else Set())

  // Dispatch class. TRAP (2.7) is a core gate: an Err state maps to the publish class 200, a
  // published state maps to a class with no DEC row (no step after publication).
  pure def classOf(status: int, op: int, flag: int): int =
    if (MUTANT == "trap_off") op * 10 + flag
    else if (status == RUN) op * 10 + flag
    else if (status == ERR) 200
    else status * 100

  // ---------------------------------------------------------------- CORE step relation
  // `bus` is the prover's request list for one slot. The core recomputes every query field from
  // the pre registers and earlier answers, takes answer fields from `bus`, and computes post.
  // kfb (mutant load_key_free): the read address is taken from the bus, not from a register.
  pure def coreStep(pre: Regs, bus: List[Tuple], kfb: bool): CoreOut =
    val f = at(bus, 0)
    val op = f.a
    val x = f.b
    val y = f.c
    val rx = reg(pre, x)
    val ry = reg(pre, y)
    val running = pre.status == RUN or MUTANT == "trap_off"
    val useLt = op == OP_LOAD and running and MUTANT != "no_read_bound"
    val lt = at(bus, 1)
    // reads compare against the current len (stated in 2.4); read_bound_final compares
    // against the last address the leaf's MEM region can hold.
    val ltBound = if (MUTANT == "read_bound_final") pre.len + N_MEM else pre.len
    val ltReq = { chip: T_ALU, key: ALU_LT, a: rx, b: ltBound, c: lt.c }
    val flag = if (op == OP_LOAD) (if (MUTANT == "no_read_bound") 1 else lt.c)
               else if ((op == OP_POP or op == OP_SETTOP) and pre.depth == 0) 1 else 0
    val cls = classOf(pre.status, op, flag)
    val di = if (useLt) 2 else 1
    val rule = at(bus, di).a
    val decReq = decRow(cls, rule)
    val hd = if (useLt) [{ chip: T_PROG, key: pre.pc, a: op, b: x, c: y }, ltReq, decReq]
               else [{ chip: T_PROG, key: pre.pc, a: op, b: x, c: y }, decReq]
    val b0 = at(bus, di + 1)
    val p1 = { ...pre, pc: pre.pc + 1 }
    val inc = if (MUTANT == "no_len_inc") 0 else 1
    val rdKey = if (kfb) b0.key else if (rule == R_LOAD) rx else pre.fp
    val rdReq = { chip: T_CELL, key: rdKey, a: b0.a, b: b0.b, c: 0 }
    val errPost = { ...pre, status: ERR }
    val body: Body =
      if (rule == R_LIT) { ok: true, reqs: [], post: setReg(p1, x, y) }
      else if (rule == R_ADD)
        { ok: true, reqs: [{ chip: T_ALU, key: ALU_ADD, a: rx, b: ry, c: b0.c }], post: setReg(p1, 0, b0.c) }
      else if (rule == R_ALLOC)
        { ok: true, reqs: [{ chip: T_CELL, key: pre.len, a: rx, b: NIL, c: 0 }],
          post: setReg({ ...p1, len: pre.len + inc }, y, pre.len) }
      else if (rule == R_LOADOOB) { ok: true, reqs: [], post: errPost }
      else if (rule == R_LOAD) { ok: true, reqs: [rdReq], post: setReg(p1, y, b0.a) }
      else if (rule == R_PUSH)
        // the frame link is the core's fp (push_link_free: the link comes from the bus)
        { ok: true, reqs: [{ chip: T_CELL, key: pre.len, a: rx, b: if (MUTANT == "push_link_free") b0.b else pre.fp, c: 0 }],
          post: { ...p1, len: pre.len + inc, fp: pre.len, depth: pre.depth + 1 } }
      else if (rule == R_POP)
        { ok: true, reqs: [rdReq], post: setReg({ ...p1, fp: b0.b, depth: pre.depth - 1 }, x, b0.a) }
      else if (rule == R_POPEMPTY or rule == R_SETTOPEMPTY) { ok: true, reqs: [], post: errPost }
      else if (rule == R_SETTOP)
        // a rewrite of the top appends (v, link) and moves fp to the new entry (D7)
        { ok: true, reqs: [rdReq, { chip: T_CELL, key: pre.len, a: rx, b: b0.b, c: 0 }],
          post: { ...p1, len: pre.len + inc, fp: if (MUTANT == "rewrite_no_move") pre.fp else pre.len } }
      else if (rule == R_JNZ) { ok: true, reqs: [], post: { ...pre, pc: if (rx != 0) y else pre.pc + 1 } }
      else if (rule == R_HALT) { ok: true, reqs: [], post: { ...pre, status: HALTED, out: pre.r0 } }
      else if (rule == R_PUBERR) { ok: true, reqs: [], post: { ...pre, status: PUBERR, out: 99 } }
      else { ok: false, reqs: [], post: pre }
    val exp = hd.concat(body.reqs)
    // D5: one enable per slot gates every micro-op row of the slot. per_row_enable lets a slot
    // record only a prefix of its micro-op rows.
    val shapeOk = if (MUTANT == "per_row_enable")
                    bus.length() >= 1 and bus.length() <= exp.length() and bus == exp.slice(0, bus.length())
                  else bus == exp
    { ok: body.ok and shapeOk, exp: exp, post: body.post }

  // ---------------------------------------------------------------- reference machine
  pure def refFp(s: RefSt): int = if (s.stack.length() == 0) NIL else s.stack[s.stack.length() - 1].at
  pure def refReg(s: RefSt, x: int): int = if (x == 0) s.r0 else s.r1
  pure def refSet(s: RefSt, x: int, v: int): RefSt = if (x == 0) { ...s, r0: v } else { ...s, r1: v }
  pure def popList(l: List[Frame]): List[Frame] = l.slice(0, l.length() - 1)

  pure def refStep(prog: List[Ins], s: RefSt): RefSt =
    if (not(steppable(s.status))) s
    else if (s.status == ERR) { ...s, status: PUBERR, out: 99 }
    else
      val ins = progAt(prog, s.pc)
      val rx = refReg(s, ins.x)
      val ry = refReg(s, ins.y)
      val n = s.mem.length()
      val s1 = { ...s, pc: s.pc + 1 }
      val d = s.stack.length()
      if (ins.op == OP_LIT) refSet(s1, ins.x, ins.y)
      else if (ins.op == OP_ADD) refSet(s1, 0, (rx + ry) % 8)
      else if (ins.op == OP_ALLOC) refSet({ ...s1, mem: s.mem.append({ a: rx, b: NIL }) }, ins.y, n)
      else if (ins.op == OP_LOAD)
        (if (rx >= 0 and rx < n) refSet(s1, ins.y, s.mem[rx].a) else { ...s, status: ERR })
      else if (ins.op == OP_PUSH)
        { ...s1, mem: s.mem.append({ a: rx, b: refFp(s) }), stack: s.stack.append({ v: rx, at: n }) }
      else if (ins.op == OP_POP)
        (if (d == 0) { ...s, status: ERR } else refSet({ ...s1, stack: popList(s.stack) }, ins.x, s.stack[d - 1].v))
      else if (ins.op == OP_SETTOP)
        (if (d == 0) { ...s, status: ERR }
         else
           val link = if (d >= 2) s.stack[d - 2].at else NIL
           { ...s1, mem: s.mem.append({ a: rx, b: link }), stack: popList(s.stack).append({ v: rx, at: n }) })
      else if (ins.op == OP_JNZ) { ...s, pc: if (rx != 0) ins.y else s.pc + 1 }
      else if (ins.op == OP_HALT) { ...s, status: HALTED, out: s.r0 }
      else { ...s, status: ERR }

  pure def refRun(prog: List[Ins], s: RefSt, k: int): RefSt = range(0, k).foldl(s, (st, _i) => refStep(prog, st))
  // Every one of the k steps starts from a steppable state (publication is the last step).
  pure def refTerminalLast(prog: List[Ins], s: RefSt, k: int): bool =
    0.to(k - 1).forall(j => steppable(refRun(prog, s, j).status))
  pure def absRegs(s: RefSt): Regs =
    { pc: s.pc, r0: s.r0, r1: s.r1, len: s.mem.length(), fp: refFp(s), depth: s.stack.length(),
      status: s.status, out: s.out }
  // Linked stack read from committed memory, bottom first.
  pure def implStack(mem: List[Cell], fp: int, depth: int): List[int] =
    range(0, depth).foldl({ l: [], p: fp }, (acc, _i) =>
      { l: [cellAt(mem, acc.p).a].concat(acc.l), p: cellAt(mem, acc.p).b }).l
  pure def refStackVals(s: RefSt): List[int] = s.stack.foldl([], (l, fr) => l.append(fr.v))
  pure val REF0: RefSt = { pc: 0, r0: 0, r1: 0, status: RUN, out: 0, mem: [], stack: [] }

  // ---------------------------------------------------------------- tables and acceptance
  pure def enabledBus(slots: List[Slot]): List[Tuple] =
    slots.foldl([], (l, sl) => if (sl.en) l.concat(sl.bus) else l)
  pure def memCellOfRow(mem: List[Tuple], k: int): Cell =
    if (0 <= k and k < mem.length()) { a: mem[k].a, b: mem[k].b } else GARBAGE
  // D3: the memory root update covers MEM rows below post.len only. root_all_rows commits every row.
  pure def rootOf(oldMem: List[Cell], mem: List[Tuple], postLen: int, base: int): List[Cell] =
    val n = if (MUTANT == "root_all_rows") mem.length() else postLen - base
    oldMem.concat(range(0, n).foldl([], (l, k) => l.append(memCellOfRow(mem, k))))
  pure def committedOld(oldMem: List[Cell], k: int): Cell =
    if (0 <= k and k < oldMem.length()) oldMem[k] else SPARSE_DEFAULT

  pure def pageRowOk(w: W, r: Tuple): bool =
    if (MUTANT == "page_unauth") true
    else if (r.chip == T_PROG) r.key >= 0 and r.key < w.prog.length() and r == progRow(w.prog, r.key)
    else if (r.chip == T_CELL)
      // D2: a PAGE CELL row opens an address below base (cells of earlier leaves only)
      (MUTANT == "page_no_base_bound" or r.key < w.pre.len)
      and r.key >= 0 and { a: r.a, b: r.b } == committedOld(w.oldMem, r.key) and r.c == 0
    else false
  // D4: the chip column of a region is a fixed column (the region's tag). tag_unbound lets an ALU
  // row carry any tag; the ALU gate then applies only to rows tagged ALU.
  pure def aluRowOk(r: Tuple): bool =
    if (MUTANT == "tag_unbound") (r.chip != T_ALU or r.c == aluF(r.key, r.a, r.b))
    else r.chip == T_ALU and (MUTANT == "alu_gate_off" or r.c == aluF(r.key, r.a, r.b))
  // D6: one base, copied from pre.len, for every MEM row; the address column is base + fixed index.
  pure def memRowOk(w: W, k: int): bool =
    w.mem[k].chip == T_CELL and w.mem[k].c == 0 and (MUTANT == "base_per_row" or w.mem[k].key == w.pre.len + k)

  pure def providers(w: W): Set[Tuple] =
    w.page.union(w.alu).union(toSet(w.mem)).union(DEC_ROWS).union(Set(ZT))
  // D1: the table side of the self-lookup is gated by the provider selectors. table_ungated puts
  // every bus row (requests included) on the table side.
  pure def table(w: W): Set[Tuple] =
    if (MUTANT == "table_ungated") providers(w).union(toSet(enabledBus(w.slots))) else providers(w)

  pure def slotOk(sl: Slot): bool =
    if (sl.en)
      val co = coreStep(sl.pre, sl.bus, MUTANT == "load_key_free")
      and { co.ok, if (MUTANT == "pc_unbound") { ...sl.post, pc: 0 } == { ...co.post, pc: 0 } else sl.post == co.post }
    else and { sl.bus == [], sl.post == sl.pre }

  pure def hMem(w: W): bool = if (MUTANT == "mem_unbounded") w.mem.length() >= N_MEM else w.mem.length() == N_MEM

  pure def accepts(w: W): bool = and {
    w.slots.length() == S,
    w.slots[0].en,
    w.slots.indices().forall(j => j + 1 == S or not(w.slots[j + 1].en) or w.slots[j].en),
    // publication is the last enabled slot (TRAP obligation 2.7, C4 of leafSlots)
    MUTANT == "trap_off" or w.slots.indices().forall(j =>
      j + 1 == S or not(w.slots[j + 1].en) or not(published(w.slots[j].post.status))),
    w.slots.indices().forall(j => slotOk(w.slots[j])),
    w.slots.indices().forall(j => j + 1 == S or w.slots[j].post == w.slots[j + 1].pre),
    w.pre == w.slots[0].pre,
    w.post == w.slots[S - 1].post,
    w.steps == w.slots.foldl(0, (n, sl) => if (sl.en) n + 1 else n),
    // pre_d binds len and mem_root (premise: pre_d is a proven post_d or genesis)
    w.pre.len == w.oldMem.length(),
    // chip regions: fixed heights and row gates
    hMem(w),
    w.page.size() <= N_PAGE,
    w.alu.size() <= N_ALU,
    w.page.forall(r => pageRowOk(w, r)),
    w.alu.forall(r => aluRowOk(r)),
    w.mem.indices().forall(k => memRowOk(w, k)),
    // the bus: every enabled request is in the table (set inclusion)
    MUTANT == "no_inclusion" or enabledBus(w.slots).foldl(true, (ok, r) => ok and table(w).contains(r)),
    // post memory commitment
    w.memOut == rootOf(w.oldMem, w.mem, w.post.len, w.pre.len),
  }

  // ---------------------------------------------------------------- witness generator
  pure def fillOne(prog: List[Ins], log: List[Cell], preLen: int, r: Tuple, mode: str): Tuple =
    if (r.chip == T_PROG)
      val ins = progAt(prog, r.key)
      { ...r, a: ins.op, b: if (mode == "prog") 1 - ins.x else ins.x, c: ins.y }
    else if (r.chip == T_ALU)
      val z = aluF(r.key, r.a, r.b)
      { ...r, c: if (mode == "alu") (if (r.key == ALU_LT) 1 - z else (z + 1) % 8) else z }
    else if (r.chip == T_DEC) { ...r, a: if (mode == "dec") altRule(decF(r.key)) else decF(r.key) }
    else if (r.chip == T_CELL and r.key < preLen)
      val cl = cellAt(log, r.key)
      val v = if (mode == "cell") { a: cl.a + 3, b: cl.b }
              else if (mode == "default") SPARSE_DEFAULT
              else if (mode == "zero" and r.key == 0) { a: 0, b: 0 }
              else cl
      { ...r, a: v.a, b: v.b }
    else r   // an append (its payload is a query field) or a read at or above len

  pure def fill(prog: List[Ins], log: List[Cell], preLen: int, exp: List[Tuple], mode: str): List[Tuple] =
    exp.foldl([], (l, r) => l.append(fillOne(prog, log, preLen, r, mode)))
  // Six passes reach the fixpoint: each pass fixes the answer of one more micro-op (at most 5).
  pure def busFrom(prog: List[Ins], pre: Regs, log: List[Cell], seed: List[Tuple], mode: str, kfb: bool): List[Tuple] =
    range(0, 6).foldl(seed, (b, _i) => fill(prog, log, pre.len, coreStep(pre, b, kfb).exp, mode))
  pure def busMode(prog: List[Ins], pre: Regs, log: List[Cell], mode: str): List[Tuple] =
    busFrom(prog, pre, log, [], mode, false)

  pure def appendCells(bus: List[Tuple], preLen: int): List[Cell] =
    bus.foldl([], (l, r) => if (r.chip == T_CELL and r.key >= preLen) l.append({ a: r.a, b: r.b }) else l)
  pure def pageRowsOf(bus: List[Tuple], base: int): Set[Tuple] =
    toSet(bus.select(r => r.chip == T_PROG or (r.chip == T_CELL and r.key < base)))
  pure def aluRowsOf(bus: List[Tuple]): Set[Tuple] = toSet(bus.select(r => r.chip == T_ALU))
  pure def firstIdx(bus: List[Tuple], p: (Tuple) => bool): int =
    bus.indices().fold(-1, (best, i) => if (p(bus[i]) and (best == -1 or i < best)) i else best)

  type TB = { actual: List[Tuple], source: List[Tuple], post: Regs, xPage: Set[Tuple], xAlu: Set[Tuple],
              dup: List[Tuple], garb: Set[int] }
  type Gen = { regs: Regs, log: List[Cell], slots: List[Slot], stop: bool, pubPrev: bool,
               page: Set[Tuple], alu: Set[Tuple], xPage: Set[Tuple], xAlu: Set[Tuple], dup: List[Tuple],
               garb: Set[int], applied: bool, why: str }

  pure def diffCells(actual: List[Tuple], hb: List[Tuple]): Set[Tuple] =
    toSet(actual).exclude(toSet(hb)).filter(r => r.chip == T_CELL)

  pure def withMode(prog: List[Ins], pre: Regs, log: List[Cell], hb: List[Tuple], plain: TB, m: str, src: bool): TB =
    val ab = busMode(prog, pre, log, m)
    { ...plain, actual: ab, source: if (src) ab else hb, post: coreStep(pre, ab, false).post }

  pure def replayPick(hb: List[Tuple], prev: List[Tuple], cellOnly: bool): { i: int, r: Tuple } =
    hb.indices().fold({ i: -1, r: ZT }, (acc, i) =>
      if (acc.i >= 0 and acc.i < i) acc
      else
        val cands = prev.select(p => p.chip == hb[i].chip and p != hb[i])
        if (cands.length() > 0 and (not(cellOnly) or hb[i].chip == T_CELL)) { i: i, r: cands[0] } else acc)

  pure def tamperBus(prog: List[Ins], pre: Regs, log: List[Cell], base: int, prev: List[Tuple],
                     tk: str, lk: str): TB =
    val hb = busMode(prog, pre, log, "none")
    val hpost = coreStep(pre, hb, false).post
    val plain = { actual: hb, source: hb, post: hpost, xPage: Set(), xAlu: Set(), dup: [], garb: Set() }
    if (tk == "forged") withMode(prog, pre, log, hb, plain, lk, false)
    else if (tk == "page_forge") withMode(prog, pre, log, hb, plain, "prog", true)
    else if (tk == "wrong_alu") withMode(prog, pre, log, hb, plain, "alu", true)
    else if (tk == "dec_lie") withMode(prog, pre, log, hb, plain, "dec", true)
    else if (tk == "pad_match") withMode(prog, pre, log, hb, plain, "zero", false)
    else if (tk == "wrong_chip")
      val t0 = withMode(prog, pre, log, hb, plain, "cell", false)
      { ...t0, xAlu: diffCells(t0.actual, hb) }
    else if (tk == "page_overlap")
      val t0 = withMode(prog, pre, log, hb, plain, "default", false)
      { ...t0, xPage: diffCells(t0.actual, hb).filter(r => r.key >= base) }
    else if (tk == "dup_mem")
      val t0 = withMode(prog, pre, log, hb, plain, "cell", false)
      { ...t0, dup: diffCells(t0.actual, hb).filter(r => r.key >= base).fold([], (l, r) => if (l.length() == 0) [r] else l) }
    else if (tk == "stale")
      // answer the read with an older cell (address k - 1, or 1 when k == 0)
      val i = firstIdx(hb, r => r.chip == T_CELL and r.key < pre.len)
      if (i < 0) plain
      else
        val k = hb[i].key
        val sk = if (k > 0) k - 1 else if (pre.len > 1) 1 else k
        val seed = hb.replaceAt(i, { ...hb[i], key: sk })
        val ab = busFrom(prog, pre, log, seed, "none", true)
        { ...plain, actual: ab, post: coreStep(pre, ab, true).post }
    else if (tk == "replay")
      // replace one request by a different row of the same chip from an earlier slot,
      // a CELL row (memory or frame) when there is one, else the first request with a candidate
      val pc = replayPick(hb, prev, true)
      val pick = if (pc.i >= 0) pc else replayPick(hb, prev, false)
      if (pick.i < 0) plain
      else
        val ab = hb.replaceAt(pick.i, pick.r)
        { ...plain, actual: ab, post: coreStep(pre, ab, true).post }
    else if (tk == "append_via_page")
      // Gemini F3: an append whose payload equals the default leaf is answered by a PAGE opening
      // of the empty leaf, and the MEM row at that address is left free (garbage)
      val apps = hb.select(r => r.chip == T_CELL and r.key >= pre.len and r.a == SPARSE_DEFAULT.a and r.b == SPARSE_DEFAULT.b)
      { ...plain, xPage: toSet(apps), garb: toSet(apps).map(r => r.key) }
    else if (tk == "mid_step")
      val ab = hb.slice(0, hb.length() - 1)
      { ...plain, actual: ab, source: ab, post: coreStep(pre, ab, false).post }
    else if (tk == "wrong_pc") { ...plain, post: { ...hpost, pc: hpost.pc + 1 } }
    else if (tk == "orphan_ok") { ...plain, xAlu: Set({ chip: T_ALU, key: ALU_ADD, a: 3, b: 3, c: 6 }) }
    else if (tk == "orphan_bad") { ...plain, xAlu: Set({ chip: T_ALU, key: ALU_ADD, a: 3, b: 3, c: 1 }) }
    else plain

  pure def disabledSlot(g: Gen, why: str): Gen =
    { ...g, slots: g.slots.append({ en: false, pre: g.regs, bus: [], post: g.regs }), stop: true,
      why: if (g.stop) g.why else why }

  pure def genSlot(prog: List[Ins], base: int, kc: int, t: int, tk: str, lk: str, g: Gen, j: int): Gen =
    if (g.stop) disabledSlot(g, g.why)
    else
      val pre = g.regs
      val afterPub = tk == "after_publish" and g.pubPrev
      val isT = (j == t and tk != "after_publish" and tk != "none") or afterPub
      val tbH = tamperBus(prog, pre, g.log, base, [], "none", lk)
      val tb = if (isT) tamperBus(prog, pre, g.log, base, enabledBus(g.slots), tk, lk) else tbH
      val page2 = g.page.union(pageRowsOf(tb.source, base))
      val alu2 = g.alu.union(aluRowsOf(tb.source))
      // the prover decides on the honest step whether it fits (a tamper does not move the close)
      val fits = and { g.page.union(pageRowsOf(tbH.source, base)).size() <= N_PAGE,
                       g.alu.union(aluRowsOf(tbH.source)).size() <= N_ALU, tbH.post.len - base <= N_MEM }
      val canStep = steppable(pre.status) or afterPub
      if (j > 0 and j >= kc) disabledSlot(g, "choice")
      else if (j > 0 and not(canStep)) disabledSlot(g, "publish")
      else if (j > 0 and not(fits or (isT and tk == "overflow"))) disabledSlot(g, "capacity")
      else
        val pubNow = published(tb.post.status)
        // the tamper slot closes the leaf only when the tamper changed something
        val changed = (isT and tb != tbH) or (isT and tk == "overflow" and not(fits)) or afterPub
        {
          regs: tb.post,
          log: g.log.concat(appendCells(tb.actual, pre.len)),
          slots: g.slots.append({ en: true, pre: pre, bus: tb.actual, post: tb.post }),
          stop: changed or (pubNow and (tk != "after_publish" or g.pubPrev)),
          pubPrev: pubNow,
          page: page2, alu: alu2,
          xPage: g.xPage.union(tb.xPage), xAlu: g.xAlu.union(tb.xAlu), dup: g.dup.concat(tb.dup), garb: g.garb.union(tb.garb),
          applied: g.applied or changed,
          why: if (pubNow) "publish" else if (changed) "tamper" else "S",
        }

  pure def memRowOf(base: int, finalLen: int, log: List[Cell], reqs: List[Tuple], dup: List[Tuple],
                    garb: Set[int], orphan: bool, k: int): Tuple =
    val addr = base + k
    val rq = reqs.select(r => r.chip == T_CELL and r.key == addr)
    if (dup.length() > 0 and k == N_MEM - 1 and addr >= finalLen) dup[0]
    else if (garb.contains(addr)) { chip: T_CELL, key: addr, a: GARBAGE.a, b: GARBAGE.b, c: 0 }
    else if (addr < finalLen) { chip: T_CELL, key: addr, a: cellAt(log, addr).a, b: cellAt(log, addr).b, c: 0 }
    else if (rq.length() > 0) rq[0]
    else if (orphan) { chip: T_CELL, key: addr, a: GARBAGE.a, b: GARBAGE.b, c: 0 }
    else { chip: T_CELL, key: addr, a: 0, b: 0, c: 0 }

  type GenOut = { w: W, applied: bool, why: str }
  pure def genLeaf(prog: List[Ins], ref0: RefSt, kc: int, t: int, tk: str, lk: str): GenOut =
    val pre = absRegs(ref0)
    val base = pre.len
    val g0: Gen = { regs: pre, log: ref0.mem, slots: [], stop: false, pubPrev: false, page: Set(), alu: Set(),
                    xPage: Set(), xAlu: Set(), dup: [], garb: Set(), applied: false, why: "S" }
    val g = range(0, S).foldl(g0, (acc, j) => genSlot(prog, base, kc, t, tk, lk, acc, j))
    val finalLen = g.regs.len
    val reqs = enabledBus(g.slots)
    val nRows = if (MUTANT == "mem_unbounded" and finalLen - base > N_MEM) finalLen - base else N_MEM
    val mem = range(0, nRows).foldl([], (l, k) =>
      l.append(memRowOf(base, finalLen, g.log, reqs, g.dup, g.garb, tk == "orphan_mem", k)))
    val w: W = {
      prog: prog, oldMem: ref0.mem, pre: pre, slots: g.slots,
      page: g.page.union(g.xPage), alu: g.alu.union(g.xAlu), mem: mem, post: g.regs,
      memOut: rootOf(ref0.mem, mem, finalLen, base),
      steps: g.slots.foldl(0, (n, sl) => if (sl.en) n + 1 else n),
    }
    { w: w, applied: g.applied or tk == "orphan_mem", why: g.why }

  // Pre-state: the reference run from genesis for g steps, stopping before a published state.
  pure def warm(prog: List[Ins], g: int): RefSt =
    range(0, g).foldl(REF0, (s, _i) => if (steppable(refStep(prog, s).status)) refStep(prog, s) else s)

  pure val INSTRS: Set[Ins] =
    tuples(0.to(1), 0.to(3)).map(p => { op: OP_LIT, x: p._1, y: p._2 })
    .union(tuples(0.to(1), 0.to(1)).map(p => { op: OP_ADD, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(1)).map(p => { op: OP_ALLOC, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(1)).map(p => { op: OP_LOAD, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(3)).map(p => { op: OP_PUSH, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(3)).map(p => { op: OP_POP, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(3)).map(p => { op: OP_SETTOP, x: p._1, y: p._2 }))
    .union(tuples(0.to(1), 0.to(P - 1)).map(p => { op: OP_JNZ, x: p._1, y: p._2 }))
  // the first row writes (PUSH or ALLOC), so fewer runs fail on an empty stack at once
  pure val FIRST_INSTRS: Set[Ins] = INSTRS.filter(i => i.op == OP_PUSH or i.op == OP_ALLOC)
  pure val HALT_INS: Ins = { op: OP_HALT, x: 0, y: 0 }

  pure val TAMPERS: Set[str] = Set("none", "forged", "page_forge", "wrong_alu", "dec_lie", "pad_match",
    "wrong_chip", "page_overlap", "dup_mem", "stale", "replay", "mid_step", "wrong_pc", "overflow",
    "after_publish", "orphan_ok", "orphan_bad", "orphan_mem", "append_via_page")
  pure val LIES: Set[str] = Set("prog", "cell", "alu", "dec")
  // tampers whose witness the base relation may accept: they add only valid unrequested rows
  pure val HARMLESS: Set[str] = Set("none", "orphan_ok", "orphan_mem")

  // ---------------------------------------------------------------- state
  var w: W
  var refPre: RefSt   // ghost: the reference state behind pre_d
  var acc: bool
  var tamper: str     // effective tamper ("none" when it left the witness unchanged)
  var why: str        // why the honest leaf closed: S, choice, capacity, publish, tamper

  action sample = {
    nondet i0 = FIRST_INSTRS.oneOf()
    nondet i1 = INSTRS.oneOf()
    nondet i2 = INSTRS.oneOf()
    nondet i3 = INSTRS.oneOf()
    nondet i4 = INSTRS.oneOf()
    nondet gw = 0.to(4).oneOf()
    nondet kc = 1.to(S).oneOf()
    nondet t = 0.to(S - 1).oneOf()
    nondet tk = TAMPERS.oneOf()
    nondet lk = LIES.oneOf()
    val prog = [i0, i1, i2, i3, i4, HALT_INS]
    val r0 = warm(prog, gw)
    val out = genLeaf(prog, r0, kc, t, tk, lk)
    val honest = genLeaf(prog, r0, kc, t, "none", lk)
    all {
      w' = out.w,
      refPre' = r0,
      acc' = accepts(out.w),
      tamper' = if (out.w == honest.w) "none" else tk,
      why' = out.why,
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- invariants
  val refK: RefSt = refRun(w.prog, refPre, w.steps)

  // Refinement: an accepted leaf equals `steps` reference steps (registers, memory commitment).
  pure def refineOk(w0: W, r0: RefSt): bool =
    val rk = refRun(w0.prog, r0, w0.steps)
    and {
      1 <= w0.steps, w0.steps <= S,
      w0.slots.indices().forall(j => w0.slots[j].en == (j < w0.steps)),
      w0.post == absRegs(rk),
      w0.memOut == rk.mem,
    }
  pure def framesOk(w0: W, r0: RefSt): bool =
    implStack(w0.memOut, w0.post.fp, w0.post.depth) == refStackVals(refRun(w0.prog, r0, w0.steps))
  val invRefine: bool = acc implies refineOk(w, refPre)
  // The linked stack in committed memory equals the reference stack.
  val invFrames: bool = acc implies framesOk(w, refPre)
  // Publication is the last step: every enabled step starts from a steppable reference state.
  val invTerminal: bool = acc implies refTerminalLast(w.prog, refPre, w.steps)

  // Bus soundness: every request is served by the region of its tag and equals that chip's function.
  pure def servedBy(w0: W, r: Tuple): bool =
    if (r.chip == T_PROG) w0.page.contains(r)
    else if (r.chip == T_CELL) (if (r.key < w0.pre.len) w0.page.contains(r) else toSet(w0.mem).contains(r))
    else if (r.chip == T_ALU) w0.alu.contains(r)
    else if (r.chip == T_DEC) DEC_ROWS.contains(r)
    else false
  pure def truth(w0: W, rk: RefSt, r: Tuple): Tuple =
    if (r.chip == T_PROG) progRow(w0.prog, r.key)
    else if (r.chip == T_CELL) { ...r, a: cellAt(rk.mem, r.key).a, b: cellAt(rk.mem, r.key).b, c: 0 }
    else if (r.chip == T_ALU) { ...r, c: aluF(r.key, r.a, r.b) }
    else if (r.chip == T_DEC) decRow(r.key, decF(r.key))
    else ZT
  pure def busOk(w0: W, r0: RefSt): bool =
    val rk = refRun(w0.prog, r0, w0.steps)
    enabledBus(w0.slots).foldl(true, (ok, r) => ok and servedBy(w0, r) and r == truth(w0, rk, r))
  val invBus: bool = acc implies busOk(w, refPre)

  // Memory consistency: a read in slot j addresses the reference's cell (LOAD: the address register;
  // POP and SETTOP: the current top frame), that cell was written before the step (key < len), and
  // the value is the latest write to it (for frames: the reference top value).
  pure def readOk(w0: W, r0: RefSt, rk: RefSt, j: int): bool =
    val sl = w0.slots[j]
    val rj = refRun(w0.prog, r0, j)
    val fetch = at(sl.bus, 0)
    val decs = sl.bus.select(r => r.chip == T_DEC)
    val rule = if (decs.length() > 0) decs[0].a else 0
    val reads = sl.bus.select(r => r.chip == T_CELL)
    if (not(sl.en) or not(rule == R_LOAD or rule == R_POP or rule == R_SETTOP) or reads.length() == 0) true
    else
      val rd = reads[0]
      val d = rj.stack.length()
      val intended = if (rule == R_LOAD) refReg(rj, fetch.b) else refFp(rj)
      and {
        rd.key == intended,
        0 <= intended, intended < sl.pre.len, intended < rj.mem.length(),
        { a: rd.a, b: rd.b } == cellAt(rj.mem, intended),
        rule == R_LOAD or (d > 0 and rd.a == rj.stack[d - 1].v),
      }
  pure def memOk(w0: W, r0: RefSt): bool = w0.slots.indices().forall(j => readOk(w0, r0, r0, j))
  val invMem: bool = acc implies memOk(w, refPre)

  // Capacity: fixed heights, appends within the MEM region, every enabled step fully recorded
  // (all its micro-op rows present, every append in the MEM table), disabled slots record nothing.
  val invCapacity: bool = acc implies and {
    w.mem.length() == N_MEM, w.page.size() <= N_PAGE, w.alu.size() <= N_ALU,
    w.post.len - w.pre.len <= N_MEM,
    w.slots.indices().forall(j =>
      val sl = w.slots[j]
      if (sl.en) and {
        sl.bus == coreStep(sl.pre, sl.bus, MUTANT == "load_key_free").exp,
        appendCells(sl.bus, sl.pre.len).length() == sl.post.len - sl.pre.len,
        sl.bus.select(r => r.chip == T_CELL and r.key >= sl.pre.len).foldl(true, (ok, r) =>
          ok and toSet(w.mem).contains(r)),
      } else and { sl.bus == [], sl.post == sl.pre }),
  }

  // Shape: the key-set constants (S, region heights, DEC table, tags) are the same for every
  // program and memory. mutant prog_in_key puts the PROG table in the key (a fixed table).
  pure val KEY0 = { s: S, hMem: N_MEM, hPage: N_PAGE, hAlu: N_ALU, dec: DEC_ROWS,
                    tags: [T_PROG, T_CELL, T_ALU, T_DEC], progFixed: List() }
  pure def keyOf(w0: W): { s: int, hMem: int, hPage: int, hAlu: int, dec: Set[Tuple], tags: List[int], progFixed: List[Ins] } =
    { s: w0.slots.length(), hMem: w0.mem.length(), hPage: N_PAGE, hAlu: N_ALU, dec: DEC_ROWS,
      tags: [T_PROG, T_CELL, T_ALU, T_DEC], progFixed: if (MUTANT == "prog_in_key") w0.prog else List() }
  val invShape: bool = acc implies keyOf(w) == KEY0

  // Set inclusion without multiplicities is sound iff every provider row is a function of its
  // input columns: two provider rows that agree on (tag, key) - and on (x, y) for ALU - are equal.
  pure def inputOf(r: Tuple): Tuple = if (r.chip == T_ALU) { ...r, c: 0 } else { ...r, a: 0, b: 0, c: 0 }
  pure def functionalOk(w0: W): bool =
    providers(w0).forall(r1 => providers(w0).forall(r2 => inputOf(r1) == inputOf(r2) implies r1 == r2))
  val invFunctional: bool = acc implies functionalOk(w)

  val invTamperRefused: bool = (acc and MUTANT == "none") implies HARMLESS.contains(tamper)
  val invComplete: bool = (MUTANT == "none" and tamper == "none") implies acc
  val invAll: bool = and { invRefine, invFrames, invTerminal, invBus, invMem, invCapacity, invShape,
                           invFunctional, invTamperRefused, invComplete }

  // ---------------------------------------------------------------- reach witnesses (--witnesses)
  pure def ruleIn(sl: Slot, rl: int): bool = sl.en and sl.bus.select(r => r.chip == T_DEC).foldl(false, (b, r) => b or r.a == rl)
  val reachFull: bool = acc and w.steps == S
  val reachShortChoice: bool = acc and why == "choice"
  val reachCapacityEnd: bool = acc and why == "capacity"
  val reachPublishHalt: bool = acc and w.post.status == HALTED
  val reachPublishErr: bool = acc and w.post.status == PUBERR
  val reachLoadMem: bool = acc and w.slots.indices().exists(j =>
    ruleIn(w.slots[j], R_LOAD) and toSet(w.slots[j].bus).exists(r => r.chip == T_CELL and r.key >= w.pre.len))
  val reachLoadPage: bool = acc and w.slots.indices().exists(j =>
    ruleIn(w.slots[j], R_LOAD) and toSet(w.slots[j].bus).exists(r => r.chip == T_CELL and r.key < w.pre.len))
  val reachPopAfterRewrite: bool = acc and w.slots.indices().exists(j => w.slots.indices().exists(i =>
    i < j and ruleIn(w.slots[i], R_SETTOP) and ruleIn(w.slots[j], R_POP)))
  val reachPopOldFrame: bool = acc and w.slots.indices().exists(j =>
    ruleIn(w.slots[j], R_POP) and toSet(w.slots[j].bus).exists(r => r.chip == T_CELL and r.key < w.pre.len))
  val reachOrphanAccepted: bool = acc and (tamper == "orphan_ok" or tamper == "orphan_mem")
  val reachTamperRefused: bool = not(acc) and tamper != "none"
  // one witness per non-harmless tamper: the tamper changed the witness and was refused
  def refusedT(tk: str): bool = not(acc) and tamper == tk
  val rForged: bool = refusedT("forged")
  val rPageForge: bool = refusedT("page_forge")
  val rWrongAlu: bool = refusedT("wrong_alu")
  val rDecLie: bool = refusedT("dec_lie")
  val rPadMatch: bool = refusedT("pad_match")
  val rWrongChip: bool = refusedT("wrong_chip")
  val rPageOverlap: bool = refusedT("page_overlap")
  val rDupMem: bool = refusedT("dup_mem")
  val rStale: bool = refusedT("stale")
  val rReplay: bool = refusedT("replay")
  val rMidStep: bool = refusedT("mid_step")
  val rWrongPc: bool = refusedT("wrong_pc")
  val rOverflow: bool = refusedT("overflow")
  val rAfterPublish: bool = refusedT("after_publish")
  val rOrphanBad: bool = refusedT("orphan_bad")
  val rAppendViaPage: bool = refusedT("append_via_page")
}

module chipLeafBase { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "none").* }
module chipLeafS1 { import chipLeaf(S = 1, N_MEM = 2, N_PAGE = 3, N_ALU = 1, MUTANT = "none").* }
module chipLeafS5 { import chipLeaf(S = 5, N_MEM = 4, N_PAGE = 8, N_ALU = 3, MUTANT = "none").* }

module chipLeaf_no_inclusion { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "no_inclusion").* }
module chipLeaf_table_ungated { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "table_ungated").* }
module chipLeaf_tag_unbound { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "tag_unbound").* }
module chipLeaf_tag_zero { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "tag_zero").* }
module chipLeaf_page_unauth { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "page_unauth").* }
module chipLeaf_page_no_base_bound { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "page_no_base_bound").* }
module chipLeaf_base_per_row { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "base_per_row").* }
module chipLeaf_dec_dup { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "dec_dup").* }
module chipLeaf_alu_gate_off { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "alu_gate_off").* }
module chipLeaf_pc_unbound { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "pc_unbound").* }
module chipLeaf_load_key_free { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "load_key_free").* }
module chipLeaf_no_read_bound { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "no_read_bound").* }
module chipLeaf_read_bound_final { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "read_bound_final").* }
module chipLeaf_no_len_inc { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "no_len_inc").* }
module chipLeaf_rewrite_no_move { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "rewrite_no_move").* }
module chipLeaf_per_row_enable { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "per_row_enable").* }
module chipLeaf_root_all_rows { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "root_all_rows").* }
module chipLeaf_mem_unbounded { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "mem_unbounded").* }
module chipLeaf_trap_off { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "trap_off").* }
module chipLeaf_push_link_free { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "push_link_free").* }
module chipLeaf_prog_in_key { import chipLeaf(S = 3, N_MEM = 2, N_PAGE = 4, N_ALU = 2, MUTANT = "prog_in_key").* }
