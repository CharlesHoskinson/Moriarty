// -*- mode: quint; -*-
// Formal soundness and completeness model of the Sherlock L_Fr Poseidon, ProgramTable,
// and Transcript AIRs (PoseidonFrAir, PoseidonProgAir, ProgramTableAir, FrInputAir,
// PackAir, SplitAir, InputAir) as built in crates/sherlock-lfr/src/.
//
// Stand-in field: F_89 (R_MOD = 89, prime, gcd(5, 88) = 1 so x -> x^5 is a permutation).
// Stand-in BabyBear prime: P_BB = 5, with 2 split digits (P_BB^2 = 25),
// H_MAX = (89 - 1) / 25 = 3, LOW_MAX = (89 - 1) - 3 * 25 = 13.
// Packing base: B_PACK = 8 (representing 2^31 > P_BB).

module lfr_poseidon_base {
  // =========================================================================
  // 1. Constraint configuration parameters (true in sound, false in mutants)
  // =========================================================================

  // Attack 1: Poseidon round constraints (poseidon.rs:107, 108, 122)
  const C_POSEIDON_SBOX_T: bool
  const C_POSEIDON_SBOX_Y: bool
  const C_POSEIDON_OUT: bool

  // Attack 2: Program bus binding (program.rs:54, 60-62, 138)
  const C_PROG_BUS: bool
  const C_PROG_TAG: bool

  // Attack 3: Split uniqueness constraints (machine.rs:438-468, fr.rs:38-59)
  const C_SPLIT_H_BOUND: bool
  const C_SPLIT_ISEQ_ONE: bool
  const C_SPLIT_LOW_BOUND: bool
  const C_SPLIT_D_CANON: bool

  // Attack 4: Pack constraints and writer canonicality (machine.rs:283-305, tape.rs:560)
  const C_PACK_CANON_WRITER: bool
  const C_PACK_UNUSED_ZERO: bool

  // Attack 5: Memory bus read binding and padding-row zero multiplicity (machine.rs:215, 469, tape.rs:576)
  const C_MEM_READ: bool
  const C_MEM_PAD_ZERO: bool

  // =========================================================================
  // 2. Stand-in field & arithmetic constants
  // =========================================================================

  pure val R_MOD: int = 89
  pure val P_BB: int = 5
  pure val P_BB_SQ: int = 25
  pure val H_MAX: int = 3
  pure val LOW_MAX: int = 13
  pure val B_PACK: int = 8
  pure val H_RANGE: int = 8
  pure val V_RANGE: int = 32

  // Program bus tags (program.rs:37-42)
  pure val TAG_INPUT: int = 1
  pure val TAG_FR_INPUT: int = 2
  pure val TAG_PERM: int = 4
  pure val TAG_PACK: int = 5
  pure val TAG_SPLIT: int = 6

  pure def fMod(x: int): int = {
    val m = x % R_MOD
    if (m < 0) m + R_MOD else m
  }

  pure def fAdd(a: int, b: int): int = fMod(a + b)
  pure def fSub(a: int, b: int): int = fMod(a - b)
  pure def fMul(a: int, b: int): int = fMod(a * b)

  // Fermat inverse a^(R_MOD - 2) mod R_MOD for R_MOD = 89 (89 - 2 = 87 = 64 + 16 + 4 + 2 + 1)
  pure def fInv(a: int): int = {
    val x1 = fMod(a)
    if (x1 == 0) 0 else {
      val x2 = fMul(x1, x1)
      val x4 = fMul(x2, x2)
      val x8 = fMul(x4, x4)
      val x16 = fMul(x8, x8)
      val x32 = fMul(x16, x16)
      val x64 = fMul(x32, x32)
      fMul(fMul(fMul(x64, x16), fMul(x4, x2)), x1)
    }
  }

  // =========================================================================
  // 3. Reference semantics (Perm, Pack, Split, Input, FrInput)
  // =========================================================================

  pure def sboxPow5(x: int): int = {
    val t = fMul(x, x)
    fMul(fMul(t, t), x)
  }

  pure def mdsRow(s0: int, s1: int, s2: int, rIdx: int): int = {
    if (rIdx == 0) fAdd(fAdd(fMul(2, s0), s1), s2)
    else if (rIdx == 1) fAdd(fAdd(s0, fMul(2, s1)), s2)
    else fAdd(fAdd(s0, s1), fMul(2, s2))
  }

  // 2-round Poseidon over F_89: Round 0 full (3 S-boxes), Round 1 partial on cell 2 (1 S-box)
  pure def refPoseidon(in0: int, in1: int, in2: int): (int, int, int) = {
    val x0 = fAdd(in0, 1)
    val x1 = fAdd(in1, 2)
    val x2 = fAdd(in2, 3)
    val y0 = sboxPow5(x0)
    val y1 = sboxPow5(x1)
    val y2 = sboxPow5(x2)
    val m0 = mdsRow(y0, y1, y2, 0)
    val m1 = mdsRow(y0, y1, y2, 1)
    val m2 = mdsRow(y0, y1, y2, 2)
    val u0 = fAdd(m0, 4)
    val u1 = fAdd(m1, 5)
    val x3 = fAdd(m2, 6)
    val y3 = sboxPow5(x3)
    val out0 = mdsRow(u0, u1, y3, 0)
    val out1 = mdsRow(u0, u1, y3, 1)
    val out2 = mdsRow(u0, u1, y3, 2)
    (out0, out1, out2)
  }

  // Pack reference (fr.rs:29-36): sum_i w_i * B_PACK^i mod R_MOD on canonical BabyBear inputs
  pure def refPack2(w0: int, w1: int): int = {
    val c0 = w0 % P_BB
    val c1 = w1 % P_BB
    fAdd(c0, fMul(B_PACK, c1))
  }

  // Split reference (fr.rs:61-70): unique base-P_BB digits (d0, d1) and high part h of canonical x
  pure def refSplit(x: int): (int, int, int) = {
    val cx = fMod(x)
    val d0 = cx % P_BB
    val d1 = (cx / P_BB) % P_BB
    val h = cx / P_BB_SQ
    (d0, d1, h)
  }

  // =========================================================================
  // 4. Row types and Bus message types
  // =========================================================================

  // Unified program cell tuple for PROG_BUS: [tag, p0, p1, p2, p3, p4, p5]
  type ProgCells = {
    p0: int,
    p1: int,
    p2: int,
    p3: int,
    p4: int,
    p5: int,
  }

  type ProgBusMsg = {
    tag: int,
    cells: ProgCells,
  }

  type ProgBusEntry = {
    msg: ProgBusMsg,
    cnt: int,
  }

  type MemBusMsg = {
    addr: int,
    v0: int,
  }

  type MemBusEntry = {
    msg: MemBusMsg,
    cnt: int,
  }

  // InputAir row: main [v], prog [addr, mult, is_const, k, 0, 0]
  type InputRow = {
    v: int,
    prog: ProgCells,
  }

  // FrInputAir row: main [v], prog [addr, mult, is_const, k, 0, 0]
  type FrInputRow = {
    v: int,
    prog: ProgCells,
  }

  // PackAir row: main [x0, x1, x2, out], prog [addr0, addr1, use0, use1, use2, addrOut] (multOut = 1 when active)
  type PackRow = {
    x0: int,
    x1: int,
    x2: int,
    out: int,
    prog: ProgCells,
  }

  // PoseidonProgAir row: main [in0..2, t0..3, y0..3, out0..2], prog [inAddr0, inAddr1, inAddr2, outAddr0, outMult0, active]
  type PoseidonRow = {
    in0: int,
    in1: int,
    in2: int,
    t0: int,
    y0: int,
    t1: int,
    y1: int,
    t2: int,
    y2: int,
    t3: int,
    y3: int,
    out0: int,
    out1: int,
    out2: int,
    prog: ProgCells,
  }

  // SplitAir row: main [x, d0, d1, h, g, invE, isEq, v], prog [addrX, addrD0, addrD1, multD0, multD1, active]
  type SplitRow = {
    x: int,
    d0: int,
    d1: int,
    h: int,
    g: int,
    invE: int,
    isEq: int,
    v: int,
    prog: ProgCells,
  }

  type MachineState = {
    inRow0: InputRow,
    inRow1: InputRow,
    inPad0: InputRow,
    inPad1: InputRow,
    frRow0: FrInputRow,
    frPad0: FrInputRow,
    packRow0: PackRow,
    packPad0: PackRow,
    permRow0: PoseidonRow,
    permPad0: PoseidonRow,
    splitRow0: SplitRow,
    splitPad0: SplitRow,
    acc: bool,
    tampered: bool,
    lastAttack: int,
  }

  // =========================================================================
  // 5. Key (ProgramTableAir preprocessed rows) & Honest Witness Builders
  // =========================================================================

  pure val zeroProg: ProgCells = { p0: 0, p1: 0, p2: 0, p3: 0, p4: 0, p5: 0 }

  // Key program rows fixed in ProgramTableAir
  pure val keyInProg0: ProgCells = { p0: 1, p1: 1, p2: 0, p3: 0, p4: 0, p5: 0 }
  pure val keyInProg1: ProgCells = { p0: 2, p1: 1, p2: 1, p3: 2, p4: 0, p5: 0 }
  pure val keyFrProg0: ProgCells = { p0: 3, p1: 3, p2: 1, p3: 6, p4: 0, p5: 0 }
  pure val keyPackProg0: ProgCells = { p0: 1, p1: 2, p2: 1, p3: 1, p4: 0, p5: 4 }
  pure val keyPermProg0: ProgCells = { p0: 3, p1: 4, p2: 3, p3: 5, p4: 0, p5: 1 }
  pure val keySplitProg0: ProgCells = { p0: 3, p1: 8, p2: 9, p3: 0, p4: 0, p5: 1 }

  pure def honestPoseidonRow(in0: int, in1: int, in2: int, pr: ProgCells): PoseidonRow = {
    val x0 = fAdd(in0, 1)
    val x1 = fAdd(in1, 2)
    val x2 = fAdd(in2, 3)
    val t0 = fMul(x0, x0)
    val y0 = fMul(fMul(t0, t0), x0)
    val t1 = fMul(x1, x1)
    val y1 = fMul(fMul(t1, t1), x1)
    val t2 = fMul(x2, x2)
    val y2 = fMul(fMul(t2, t2), x2)
    val m0 = mdsRow(y0, y1, y2, 0)
    val m1 = mdsRow(y0, y1, y2, 1)
    val m2 = mdsRow(y0, y1, y2, 2)
    val u0 = fAdd(m0, 4)
    val u1 = fAdd(m1, 5)
    val x3 = fAdd(m2, 6)
    val t3 = fMul(x3, x3)
    val y3 = fMul(fMul(t3, t3), x3)
    val out0 = mdsRow(u0, u1, y3, 0)
    val out1 = mdsRow(u0, u1, y3, 1)
    val out2 = mdsRow(u0, u1, y3, 2)
    {
      in0: fMod(in0), in1: fMod(in1), in2: fMod(in2),
      t0: t0, y0: y0, t1: t1, y1: y1, t2: t2, y2: y2, t3: t3, y3: y3,
      out0: out0, out1: out1, out2: out2,
      prog: pr,
    }
  }

  pure def honestSplitRow(x: int, pr: ProgCells): SplitRow = {
    val cx = fMod(x)
    val d0 = cx % P_BB
    val d1 = (cx / P_BB) % P_BB
    val h = cx / P_BB_SQ
    val g = if (h <= H_MAX) H_MAX - h else 0
    val e = fSub(H_MAX, h)
    val isEq = if (e == 0) 1 else 0
    val invE = if (e == 0) 0 else fInv(e)
    val low = d0 + P_BB * d1
    val v = if (isEq == 1 and low <= LOW_MAX) LOW_MAX - low else 0
    {
      x: cx, d0: d0, d1: d1, h: h, g: g, invE: invE, isEq: isEq, v: v,
      prog: pr,
    }
  }

  // =========================================================================
  // 6. Row Gate Predicates & Bus Balance Predicates
  // =========================================================================

  // InputAir gate (tape.rs:549-578): canonical check + const check
  pure def inputRowGate(r: InputRow): bool = {
    val v = fMod(r.v)
    val canonOk = if (C_PACK_CANON_WRITER) (v >= 0 and v < P_BB) else (v >= 0 and v < R_MOD)
    val constOk = fMul(r.prog.p2, fSub(v, r.prog.p3)) == 0
    canonOk and constOk
  }

  // FrInputAir gate (machine.rs:155-162): const check p[2] * (v - p[3]) == 0
  pure def frInputRowGate(r: FrInputRow): bool = {
    val v = fMod(r.v)
    fMul(r.prog.p2, fSub(v, r.prog.p3)) == 0
  }

  // PackAir gate (machine.rs:283-305): unused slots zero + linear combination
  pure def packRowGate(r: PackRow): bool = {
    val u0Ok = if (C_PACK_UNUSED_ZERO) fMul(fSub(1, r.prog.p2), r.x0) == 0 else true
    val u1Ok = if (C_PACK_UNUSED_ZERO) fMul(fSub(1, r.prog.p3), r.x1) == 0 else true
    val u2Ok = if (C_PACK_UNUSED_ZERO) fMul(fSub(1, r.prog.p4), r.x2) == 0 else true
    val acc = fAdd(fAdd(r.x0, fMul(B_PACK, r.x1)), fMul(B_PACK * B_PACK, r.x2))
    u0Ok and u1Ok and u2Ok and (fMod(r.out) == acc)
  }

  // PoseidonProgAir gate (poseidon.rs:95-124, machine.rs:210-229)
  pure def poseidonRowGate(r: PoseidonRow): bool = {
    val x0 = fAdd(r.in0, 1)
    val x1 = fAdd(r.in1, 2)
    val x2 = fAdd(r.in2, 3)
    val m0 = mdsRow(r.y0, r.y1, r.y2, 0)
    val m1 = mdsRow(r.y0, r.y1, r.y2, 1)
    val m2 = mdsRow(r.y0, r.y1, r.y2, 2)
    val u0 = fAdd(m0, 4)
    val u1 = fAdd(m1, 5)
    val x3 = fAdd(m2, 6)
    val s0 = mdsRow(u0, u1, r.y3, 0)
    val s1 = mdsRow(u0, u1, r.y3, 1)
    val s2 = mdsRow(u0, u1, r.y3, 2)
    val tOk = if (C_POSEIDON_SBOX_T) and {
      fMod(r.t0) == fMul(x0, x0),
      fMod(r.t1) == fMul(x1, x1),
      fMod(r.t2) == fMul(x2, x2),
      fMod(r.t3) == fMul(x3, x3),
    } else true
    val yOk = if (C_POSEIDON_SBOX_Y) and {
      fMod(r.y0) == fMul(fMul(r.t0, r.t0), x0),
      fMod(r.y1) == fMul(fMul(r.t1, r.t1), x1),
      fMod(r.y2) == fMul(fMul(r.t2, r.t2), x2),
      fMod(r.y3) == fMul(fMul(r.t3, r.t3), x3),
    } else true
    val outOk = if (C_POSEIDON_OUT) and {
      fMod(r.out0) == s0,
      fMod(r.out1) == s1,
      fMod(r.out2) == s2,
    } else true
    tOk and yOk and outOk
  }

  // SplitAir gate + RANGE_BUS lookups (machine.rs:424-475)
  pure def splitRowAccepted(r: SplitRow): bool = {
    val low = fAdd(r.d0, fMul(P_BB, r.d1))
    val reconOk = fMod(r.x) == fAdd(low, fMul(r.h, P_BB_SQ))
    val e = fSub(H_MAX, r.h)
    val hRangeOk = r.h >= 0 and r.h < H_RANGE
    val gRangeOk = r.g >= 0 and r.g < H_RANGE
    val hBoundOk = if (C_SPLIT_H_BOUND) (e == fMod(r.g) and gRangeOk) else true
    val isEqOneOk = if (C_SPLIT_ISEQ_ONE) (fMod(r.isEq) == fSub(1, fMul(e, r.invE))) else (r.isEq == 0 or r.isEq == 1)
    val isEqZeroOk = fMul(e, r.isEq) == 0
    val vRangeOk = r.v >= 0 and r.v < V_RANGE
    val lowBoundOk = if (C_SPLIT_LOW_BOUND) (fMul(r.isEq, fSub(LOW_MAX, low)) == fMod(r.v) and vRangeOk) else true
    val dCanonOk = if (C_SPLIT_D_CANON) (r.d0 >= 0 and r.d0 < P_BB and r.d1 >= 0 and r.d1 < P_BB)
                   else (r.d0 >= 0 and r.d0 < H_RANGE and r.d1 >= 0 and r.d1 < H_RANGE)
    reconOk and hRangeOk and hBoundOk and isEqOneOk and isEqZeroOk and lowBoundOk and dCanonOk
  }

  // Program bus message constructor: applies tag if C_PROG_TAG is enabled
  pure def mkProgMsg(tag: int, pr: ProgCells): ProgBusMsg = {
    { tag: if (C_PROG_TAG) tag else 0, cells: pr }
  }

  // Check if all messages in a ProgBusEntry list sum to 0 mod R_MOD
  pure def progBusBalanced(entries: List[ProgBusEntry]): bool = {
    if (not(C_PROG_BUS)) true else {
      entries.indices().forall(i => {
        val target = entries.nth(i).msg
        val total = entries.foldl(0, (acc, e) =>
          if (e.msg == target) fAdd(acc, e.cnt) else acc
        )
        total == 0
      })
    }
  }

  // Memory bus message constructor: binds value v0 if C_MEM_READ is enabled
  pure def mkMemMsg(addr: int, v0: int): MemBusMsg = {
    { addr: fMod(addr), v0: if (C_MEM_READ) fMod(v0) else 0 }
  }

  // Check if all messages in a MemBusEntry list sum to 0 mod R_MOD
  pure def memBusBalanced(entries: List[MemBusEntry]): bool = {
    entries.indices().forall(i => {
      val target = entries.nth(i).msg
      val total = entries.foldl(0, (acc, e) =>
        if (e.msg == target) fAdd(acc, e.cnt) else acc
      )
      total == 0
    })
  }

  // Full acceptance check across all rows and buses of MachineState
  pure def isAccepted(s: MachineState): bool = {
    val gatesOk = and {
      inputRowGate(s.inRow0),
      inputRowGate(s.inRow1),
      inputRowGate(s.inPad0),
      inputRowGate(s.inPad1),
      frInputRowGate(s.frRow0),
      frInputRowGate(s.frPad0),
      packRowGate(s.packRow0),
      packRowGate(s.packPad0),
      poseidonRowGate(s.permRow0),
      poseidonRowGate(s.permPad0),
      splitRowAccepted(s.splitRow0),
      splitRowAccepted(s.splitPad0),
    }

    // Program bus entries: ProgramTableAir sends key rows (+1) and nop rows (+padding_count);
    // execution rows receive (-1 == R_MOD - 1).
    // When C_MEM_PAD_ZERO is false, InputAir padding rows are not bound on PROG_BUS.
    val inNopCnt = if (C_MEM_PAD_ZERO) 2 else 0
    val progEntriesBase: List[ProgBusEntry] = List(
      // ProgramTableAir sends (key)
      { msg: mkProgMsg(TAG_INPUT, keyInProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_INPUT, keyInProg1), cnt: 1 },
      { msg: mkProgMsg(TAG_INPUT, zeroProg), cnt: inNopCnt },
      { msg: mkProgMsg(TAG_FR_INPUT, keyFrProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_FR_INPUT, zeroProg), cnt: 1 },
      { msg: mkProgMsg(TAG_PACK, keyPackProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_PACK, zeroProg), cnt: 1 },
      { msg: mkProgMsg(TAG_PERM, keyPermProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_PERM, zeroProg), cnt: 1 },
      { msg: mkProgMsg(TAG_SPLIT, keySplitProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_SPLIT, zeroProg), cnt: 1 },
      // Execution AIR receives (-1 == R_MOD - 1)
      { msg: mkProgMsg(TAG_INPUT, s.inRow0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_INPUT, s.inRow1.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_FR_INPUT, s.frRow0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_FR_INPUT, s.frPad0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_PACK, s.packRow0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_PACK, s.packPad0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_PERM, s.permRow0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_PERM, s.permPad0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_SPLIT, s.splitRow0.prog), cnt: R_MOD - 1 },
      { msg: mkProgMsg(TAG_SPLIT, s.splitPad0.prog), cnt: R_MOD - 1 }
    )
    val progEntries = if (C_MEM_PAD_ZERO)
      progEntriesBase
        .append({ msg: mkProgMsg(TAG_INPUT, s.inPad0.prog), cnt: R_MOD - 1 })
        .append({ msg: mkProgMsg(TAG_INPUT, s.inPad1.prog), cnt: R_MOD - 1 })
    else progEntriesBase

    // Memory bus entries across all rows
    val packActive = s.packRow0.prog.p2
    val packPadActive = s.packPad0.prog.p2
    val memEntries: List[MemBusEntry] = List(
      // InputAir writes
      { msg: mkMemMsg(s.inRow0.prog.p0, s.inRow0.v), cnt: fMod(s.inRow0.prog.p1) },
      { msg: mkMemMsg(s.inRow1.prog.p0, s.inRow1.v), cnt: fMod(s.inRow1.prog.p1) },
      { msg: mkMemMsg(s.inPad0.prog.p0, s.inPad0.v), cnt: fMod(s.inPad0.prog.p1) },
      { msg: mkMemMsg(s.inPad1.prog.p0, s.inPad1.v), cnt: fMod(s.inPad1.prog.p1) },
      // FrInputAir writes
      { msg: mkMemMsg(s.frRow0.prog.p0, s.frRow0.v), cnt: fMod(s.frRow0.prog.p1) },
      { msg: mkMemMsg(s.frPad0.prog.p0, s.frPad0.v), cnt: fMod(s.frPad0.prog.p1) },
      // PackAir row 0 reads & write
      { msg: mkMemMsg(s.packRow0.prog.p0, s.packRow0.x0), cnt: fMod(-s.packRow0.prog.p2) },
      { msg: mkMemMsg(s.packRow0.prog.p1, s.packRow0.x1), cnt: fMod(-s.packRow0.prog.p3) },
      { msg: mkMemMsg(0, s.packRow0.x2), cnt: fMod(-s.packRow0.prog.p4) },
      { msg: mkMemMsg(s.packRow0.prog.p5, s.packRow0.out), cnt: fMod(packActive) },
      // PackAir pad 0 reads & write
      { msg: mkMemMsg(s.packPad0.prog.p0, s.packPad0.x0), cnt: fMod(-s.packPad0.prog.p2) },
      { msg: mkMemMsg(s.packPad0.prog.p1, s.packPad0.x1), cnt: fMod(-s.packPad0.prog.p3) },
      { msg: mkMemMsg(0, s.packPad0.x2), cnt: fMod(-s.packPad0.prog.p4) },
      { msg: mkMemMsg(s.packPad0.prog.p5, s.packPad0.out), cnt: fMod(packPadActive) },
      // PoseidonProgAir row 0 reads & write
      { msg: mkMemMsg(s.permRow0.prog.p0, s.permRow0.in0), cnt: fMod(-s.permRow0.prog.p5) },
      { msg: mkMemMsg(s.permRow0.prog.p1, s.permRow0.in1), cnt: fMod(-s.permRow0.prog.p5) },
      { msg: mkMemMsg(s.permRow0.prog.p2, s.permRow0.in2), cnt: fMod(-s.permRow0.prog.p5) },
      { msg: mkMemMsg(s.permRow0.prog.p3, s.permRow0.out0), cnt: fMod(s.permRow0.prog.p4) },
      // PoseidonProgAir pad 0 reads & write
      { msg: mkMemMsg(s.permPad0.prog.p0, s.permPad0.in0), cnt: fMod(-s.permPad0.prog.p5) },
      { msg: mkMemMsg(s.permPad0.prog.p1, s.permPad0.in1), cnt: fMod(-s.permPad0.prog.p5) },
      { msg: mkMemMsg(s.permPad0.prog.p2, s.permPad0.in2), cnt: fMod(-s.permPad0.prog.p5) },
      { msg: mkMemMsg(s.permPad0.prog.p3, s.permPad0.out0), cnt: fMod(s.permPad0.prog.p4) },
      // SplitAir row 0 read & writes
      { msg: mkMemMsg(s.splitRow0.prog.p0, s.splitRow0.x), cnt: fMod(-s.splitRow0.prog.p5) },
      { msg: mkMemMsg(s.splitRow0.prog.p1, s.splitRow0.d0), cnt: fMod(s.splitRow0.prog.p3) },
      { msg: mkMemMsg(s.splitRow0.prog.p2, s.splitRow0.d1), cnt: fMod(s.splitRow0.prog.p4) },
      // SplitAir pad 0 read & writes
      { msg: mkMemMsg(s.splitPad0.prog.p0, s.splitPad0.x), cnt: fMod(-s.splitPad0.prog.p5) },
      { msg: mkMemMsg(s.splitPad0.prog.p1, s.splitPad0.d0), cnt: fMod(s.splitPad0.prog.p3) },
      { msg: mkMemMsg(s.splitPad0.prog.p2, s.splitPad0.d1), cnt: fMod(s.splitPad0.prog.p4) }
    )

    gatesOk and progBusBalanced(progEntries) and memBusBalanced(memEntries)
  }

  // Refinement predicate: every op's output cells match the reference semantics of the key program
  pure def refinesRef(s: MachineState): bool = {
    val w0 = s.inRow0.v % P_BB
    val w1 = 2
    val expectedPack = refPack2(w0, w1)
    val expectedFr = 6
    val expectedPerm = refPoseidon(expectedFr, expectedPack, expectedFr)
    val expectedSplit = refSplit(expectedFr)
    and {
      s.inRow0.v == w0,
      s.inRow1.v == w1,
      s.frRow0.v == expectedFr,
      s.packRow0.out == expectedPack,
      (s.permRow0.out0, s.permRow0.out1, s.permRow0.out2) == expectedPerm,
      (s.splitRow0.d0, s.splitRow0.d1, s.splitRow0.h) == expectedSplit,
    }
  }

  pure def withVerdict(s: MachineState): MachineState = {
    { ...s, acc: isAccepted(s) }
  }

  pure val honestState: MachineState = {
    val base: MachineState = {
      inRow0: { v: 1, prog: keyInProg0 },
      inRow1: { v: 2, prog: keyInProg1 },
      inPad0: { v: 0, prog: zeroProg },
      inPad1: { v: 0, prog: zeroProg },
      frRow0: { v: 6, prog: keyFrProg0 },
      frPad0: { v: 0, prog: zeroProg },
      packRow0: { x0: 1, x1: 2, x2: 0, out: 17, prog: keyPackProg0 },
      packPad0: { x0: 0, x1: 0, x2: 0, out: 0, prog: zeroProg },
      permRow0: honestPoseidonRow(6, 17, 6, keyPermProg0),
      permPad0: honestPoseidonRow(0, 0, 0, zeroProg),
      splitRow0: honestSplitRow(6, keySplitProg0),
      splitPad0: honestSplitRow(0, zeroProg),
      acc: true,
      tampered: false,
      lastAttack: 0,
    }
    withVerdict(base)
  }

  // =========================================================================
  // 7. Concrete Prover Attack Builders (one per mutant + general tampers)
  // =========================================================================

  // Attack 1a (C_POSEIDON_SBOX_T = false): forge partial-round S-box t3, recompute y3 and out
  pure val attackPoseidonSboxT: MachineState = {
    val h = honestState
    val p = h.permRow0
    val m2 = mdsRow(p.y0, p.y1, p.y2, 2)
    val x3 = fAdd(m2, 6)
    val forgedT3 = fAdd(p.t3, 1)
    val forgedY3 = fMul(fMul(forgedT3, forgedT3), x3)
    val u0 = fAdd(mdsRow(p.y0, p.y1, p.y2, 0), 4)
    val u1 = fAdd(mdsRow(p.y0, p.y1, p.y2, 1), 5)
    val forgedPerm: PoseidonRow = {
      ...p,
      t3: forgedT3,
      y3: forgedY3,
      out0: mdsRow(u0, u1, forgedY3, 0),
      out1: mdsRow(u0, u1, forgedY3, 1),
      out2: mdsRow(u0, u1, forgedY3, 2),
    }
    withVerdict({ ...h, permRow0: forgedPerm, tampered: true, lastAttack: 1 })
  }

  // Attack 1b (C_POSEIDON_SBOX_Y = false): forge partial-round S-box y3, recompute out
  pure val attackPoseidonSboxY: MachineState = {
    val h = honestState
    val p = h.permRow0
    val forgedY3 = fAdd(p.y3, 1)
    val u0 = fAdd(mdsRow(p.y0, p.y1, p.y2, 0), 4)
    val u1 = fAdd(mdsRow(p.y0, p.y1, p.y2, 1), 5)
    val forgedPerm: PoseidonRow = {
      ...p,
      y3: forgedY3,
      out0: mdsRow(u0, u1, forgedY3, 0),
      out1: mdsRow(u0, u1, forgedY3, 1),
      out2: mdsRow(u0, u1, forgedY3, 2),
    }
    withVerdict({ ...h, permRow0: forgedPerm, tampered: true, lastAttack: 2 })
  }

  // Attack 1c (C_POSEIDON_OUT = false): forge out0 directly
  pure val attackPoseidonOut: MachineState = {
    val h = honestState
    val p = h.permRow0
    val forgedPerm: PoseidonRow = { ...p, out0: fAdd(p.out0, 1) }
    withVerdict({ ...h, permRow0: forgedPerm, tampered: true, lastAttack: 3 })
  }

  // Attack 2a (C_PROG_BUS = false): prover changes FrInputAir main-trace prog cell is_const to 0 and v to 7
  pure val attackProgNoBus: MachineState = {
    val h = honestState
    val forgedFr: FrInputRow = {
      v: 7,
      prog: { ...keyFrProg0, p2: 0 },
    }
    withVerdict({
      ...h,
      frRow0: forgedFr,
      permRow0: honestPoseidonRow(7, 17, 7, keyPermProg0),
      splitRow0: honestSplitRow(7, keySplitProg0),
      tampered: true,
      lastAttack: 4,
    })
  }

  // Attack 2b (C_PROG_TAG = false): FrInputAir row 0 takes keyInProg0 [addr=1, mult=1, is_const=0, k=0]
  // and writes non-canonical v=6 >= P_BB to addr=1 (bypassing InputAir's v < P_BB check), while
  // FrInputAir pad 0 takes keyFrProg0 and InputAir row 0 takes zeroProg.
  pure val attackProgNoTag: MachineState = {
    val h = honestState
    val forgedInRow0: InputRow = { v: 0, prog: zeroProg }
    val forgedFrRow0: FrInputRow = { v: 6, prog: keyInProg0 }
    val forgedFrPad0: FrInputRow = { v: 6, prog: keyFrProg0 }
    val forgedPack0: PackRow = { x0: 6, x1: 2, x2: 0, out: 22, prog: keyPackProg0 }
    withVerdict({
      ...h,
      inRow0: forgedInRow0,
      frRow0: forgedFrRow0,
      frPad0: forgedFrPad0,
      packRow0: forgedPack0,
      permRow0: honestPoseidonRow(6, 22, 6, keyPermProg0),
      tampered: true,
      lastAttack: 5,
    })
  }

  // Attack 3a (C_SPLIT_H_BOUND = false): for x = 6, use x + 2*89 = 184 = 4 + 1*5 + 7*25 (h=7 > H_MAX=3)
  pure val attackSplitNoHBound: MachineState = {
    val h = honestState
    val e = fSub(H_MAX, 7)
    val forgedSplit: SplitRow = {
      x: 6, d0: 4, d1: 1, h: 7, g: 0, invE: fInv(e), isEq: 0, v: 0,
      prog: keySplitProg0,
    }
    withVerdict({ ...h, splitRow0: forgedSplit, tampered: true, lastAttack: 6 })
  }

  // Attack 3b (C_SPLIT_ISEQ_ONE = false): for x = 6, use x + 89 = 95 = 0 + 4*5 + 3*25 (h=3 == H_MAX, isEq=0)
  pure val attackSplitNoIsEqOne: MachineState = {
    val h = honestState
    val forgedSplit: SplitRow = {
      x: 6, d0: 0, d1: 4, h: 3, g: 0, invE: 0, isEq: 0, v: 0,
      prog: keySplitProg0,
    }
    withVerdict({ ...h, splitRow0: forgedSplit, tampered: true, lastAttack: 7 })
  }

  // Attack 3c (C_SPLIT_LOW_BOUND = false): for x = 6, use x + 89 = 95 = 0 + 4*5 + 3*25 (h=3 == H_MAX, isEq=1, v=0)
  pure val attackSplitNoLowBound: MachineState = {
    val h = honestState
    val forgedSplit: SplitRow = {
      x: 6, d0: 0, d1: 4, h: 3, g: 0, invE: 0, isEq: 1, v: 0,
      prog: keySplitProg0,
    }
    withVerdict({ ...h, splitRow0: forgedSplit, tampered: true, lastAttack: 8 })
  }

  // Attack 3d (C_SPLIT_D_CANON = false): for x = 6, use non-canonical d0 = 6, d1 = 0, h = 0
  pure val attackSplitNoDCanon: MachineState = {
    val h = honestState
    val e = fSub(H_MAX, 0)
    val forgedSplit: SplitRow = {
      x: 6, d0: 6, d1: 0, h: 0, g: 3, invE: fInv(e), isEq: 0, v: 0,
      prog: keySplitProg0,
    }
    withVerdict({ ...h, splitRow0: forgedSplit, tampered: true, lastAttack: 9 })
  }

  // Attack 4a (C_PACK_CANON_WRITER = false): InputAir row 0 writes non-canonical v = 6 (6 mod 5 == 1)
  pure val attackPackUnconstrainedWriter: MachineState = {
    val h = honestState
    val forgedIn0: InputRow = { v: 6, prog: keyInProg0 }
    val forgedPack0: PackRow = { x0: 6, x1: 2, x2: 0, out: 22, prog: keyPackProg0 }
    withVerdict({
      ...h,
      inRow0: forgedIn0,
      packRow0: forgedPack0,
      permRow0: honestPoseidonRow(6, 22, 6, keyPermProg0),
      tampered: true,
      lastAttack: 10,
    })
  }

  // Attack 4b (C_PACK_UNUSED_ZERO = false): PackAir row 0 sets unused x2 = 1 (use2 = 0), out = 17 + 64 = 81
  pure val attackPackNoUnusedZero: MachineState = {
    val h = honestState
    val forgedPack0: PackRow = { x0: 1, x1: 2, x2: 1, out: 81, prog: keyPackProg0 }
    withVerdict({
      ...h,
      packRow0: forgedPack0,
      permRow0: honestPoseidonRow(6, 81, 6, keyPermProg0),
      tampered: true,
      lastAttack: 11,
    })
  }

  // Attack 5a (C_MEM_READ = false): SplitAir row 0 reads x = 7 instead of 6 written at addr = 3
  pure val attackMemNoRead: MachineState = {
    val h = honestState
    withVerdict({
      ...h,
      splitRow0: honestSplitRow(7, keySplitProg0),
      tampered: true,
      lastAttack: 12,
    })
  }

  // Attack 5b (C_MEM_PAD_ZERO = false): two InputAir padding rows cancel (addr=2, v=2) and forge (addr=2, v=3)
  pure val attackMemPadNonZero: MachineState = {
    val h = honestState
    val padCancel: InputRow = { v: 2, prog: { p0: 2, p1: R_MOD - 1, p2: 0, p3: 0, p4: 0, p5: 0 } }
    val padForge: InputRow = { v: 3, prog: { p0: 2, p1: 1, p2: 0, p3: 0, p4: 0, p5: 0 } }
    val forgedPack0: PackRow = { x0: 1, x1: 3, x2: 0, out: 25, prog: keyPackProg0 }
    withVerdict({
      ...h,
      inPad0: padCancel,
      inPad1: padForge,
      packRow0: forgedPack0,
      permRow0: honestPoseidonRow(6, 25, 6, keyPermProg0),
      tampered: true,
      lastAttack: 13,
    })
  }

  // Nondeterministic single-cell tamper on Split or Poseidon
  pure def randomCellTamper(delta: int): MachineState = {
    val h = honestState
    val p = h.permRow0
    val forgedPerm: PoseidonRow = { ...p, t0: fAdd(p.t0, delta) }
    withVerdict({ ...h, permRow0: forgedPerm, tampered: true, lastAttack: 14 })
  }

  // =========================================================================
  // 8. State Machine (var, init, step, invariants)
  // =========================================================================

  var state: MachineState

  action init: bool = all {
    state' = honestState,
  }

  action step: bool = {
    nondet delta = 1.to(R_MOD - 1).oneOf()
    any {
      all { state' = honestState },
      all { state' = attackPoseidonSboxT },
      all { state' = attackPoseidonSboxY },
      all { state' = attackPoseidonOut },
      all { state' = attackProgNoBus },
      all { state' = attackProgNoTag },
      all { state' = attackSplitNoHBound },
      all { state' = attackSplitNoIsEqOne },
      all { state' = attackSplitNoLowBound },
      all { state' = attackSplitNoDCanon },
      all { state' = attackPackUnconstrainedWriter },
      all { state' = attackPackNoUnusedZero },
      all { state' = attackMemNoRead },
      all { state' = attackMemPadNonZero },
      all { state' = randomCellTamper(delta) },
    }
  }

  // Main soundness/refinement invariant: accepted witness implies reference semantics
  val invRefine: bool = state.acc implies refinesRef(state)

  // Completeness invariant on the honest state
  val invHonestAccepted: bool = honestState.acc and refinesRef(honestState)

  // Negated reachability invariants (must be VIOLATED in sound instance to prove non-vacuity)
  val reachHonestAccepted: bool = not(state.acc and not(state.tampered))
  val reachTamperRejected: bool = not(state.tampered and not(state.acc))
  val reachPoseidonAttackRejected: bool = not(state.lastAttack == 1 and not(state.acc))
  val reachProgAttackRejected: bool = not(state.lastAttack == 5 and not(state.acc))
  val reachSplitAttackRejected: bool = not(state.lastAttack == 6 and not(state.acc))
  val reachPackAttackRejected: bool = not(state.lastAttack == 10 and not(state.acc))
  val reachMemAttackRejected: bool = not(state.lastAttack == 13 and not(state.acc))
}

// ===========================================================================
// Concrete Instances: sound + 13 mutants (covering Attacks 1 to 5)
// ===========================================================================

module sound {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Default module matching file stem
module lfr_poseidon {
  import sound.*
}

// Attack 1a mutant: Poseidon S-box t = x^2 omitted (poseidon.rs:107)
module mut_poseidon_sbox_t {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = false,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 1b mutant: Poseidon S-box y = t^2 * x omitted (poseidon.rs:108)
module mut_poseidon_sbox_y {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = false,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 1c mutant: Poseidon output equality out == s omitted (poseidon.rs:122)
module mut_poseidon_out {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = false,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 2a mutant: Program bus receive omitted (program.rs:60-62)
module mut_prog_no_bus {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = false,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 2b mutant: Program bus tag omitted (program.rs:54)
module mut_prog_no_tag {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = false,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 3a mutant: Split H - h range bound omitted (machine.rs:440)
module mut_split_no_h_bound {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = false,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 3b mutant: Split is_eq = 1 - e * inv_e omitted (machine.rs:442)
module mut_split_no_iseq_one {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = false,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 3c mutant: Split is_eq * (L - low) == v omitted (machine.rs:444)
module mut_split_no_low_bound {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = false,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 3d mutant: Split digit canonicality d_i < p omitted (machine.rs:447)
module mut_split_no_d_canon {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = false,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 4a mutant: Writer canonicality for Pack inputs omitted (tape.rs:560, machine.rs:283)
module mut_pack_unconstrained_writer {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = false,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 4b mutant: Pack unused word zero constraint (1 - use_i) * x_i == 0 omitted (machine.rs:288)
module mut_pack_no_unused_zero {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = false,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 5a mutant: Memory bus value binding omitted (machine.rs:38-45, 469)
module mut_mem_no_read {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = false,
    C_MEM_PAD_ZERO = true
  ).*
}

// Attack 5b mutant: Padding row program-bus zero-multiplicity binding omitted (program.rs:90, tape.rs:557)
module mut_mem_pad_nonzero {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = false
  ).*
}
