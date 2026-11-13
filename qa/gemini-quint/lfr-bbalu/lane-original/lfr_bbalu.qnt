// -*- mode: Bluespec; -*-
// Soundness and completeness model of the Sherlock L_Fr BabyBear ALU and range-check
// AIRs (`Range16Air`, `BbAluAir`, `BbAluProgAir`, `InputAir`, `ProgramTableAir`).
//
// Sources modelled:
//   - crates/sherlock-lfr/src/alu.rs      (`BbAluAir`, `eval_canonical`, `canonical_cells`)
//   - crates/sherlock-lfr/src/bb.rs       (`P`, `P_MINUS_1_HI`, `W`, `ef4_mul_unreduced`)
//   - crates/sherlock-lfr/src/range.rs    (`Range16Air`, `RANGE_BUS`)
//   - crates/sherlock-lfr/src/tape.rs     (`InputAir`, `BbAluProgAir`, `MEM_BUS`, `Tape`)
//   - crates/sherlock-lfr/src/program.rs  (`ProgramTableAir`, `PROG_BUS`, `receive_prog`)

module lfr_bbalu_base {
  // =========================================================================
  // 1. Constraint configuration parameters (sound = all true; mutants flip one)
  // =========================================================================

  // Q1a: `0x7800 - hi` range lookup in `eval_canonical` (alu.rs:152)
  const ENFORCE_HI_HEADROOM: bool
  // Q1b: `is_max * lo == 0` when `hi == 0x7800` in `eval_canonical` (alu.rs:148)
  const ENFORCE_MAX_LO_ZERO: bool
  // Q2:  top quotient limb `ql[k][2]` range lookup in `BbAluAir::eval` (alu.rs:127)
  const ENFORCE_Q_HI_RANGE: bool
  // Q3a: ALU selectors `(add, sub, mul)` bound in `PROG_BUS` receipt (tape.rs:683-688)
  const ENFORCE_PROG_SELECTORS: bool
  // Q3b: BabyBear upper-coefficient zero check `z_k * v_k == 0` in `InputAir` (tape.rs:569)
  const ENFORCE_BB_UPPER_ZERO: bool
  // Q4a: low limb `lo` range lookup in `eval_canonical` (alu.rs:150)
  const ENFORCE_LO_RANGE: bool
  // Q4b: `Range16Air` first/transition/last-row constraints (range.rs:38-44)
  const ENFORCE_RANGE_TABLE: bool
  // Q5a: `MEM_BUS` operand reads and result write in `BbAluProgAir` (tape.rs:695-697)
  const ENFORCE_MEM_BUS: bool
  // Q5b: `PROG_BUS` `receive_prog` in `InputAir` and `BbAluProgAir` (tape.rs:557, 688)
  const ENFORCE_PROG_BUS: bool

  type ConstraintFlags = {
    hiHeadroom: bool,
    maxLoZero: bool,
    qHiRange: bool,
    progSelectors: bool,
    bbUpperZero: bool,
    loRange: bool,
    rangeTable: bool,
    memBus: bool,
    progBus: bool
  }

  pure val activeFlags: ConstraintFlags = {
    hiHeadroom: ENFORCE_HI_HEADROOM,
    maxLoZero: ENFORCE_MAX_LO_ZERO,
    qHiRange: ENFORCE_Q_HI_RANGE,
    progSelectors: ENFORCE_PROG_SELECTORS,
    bbUpperZero: ENFORCE_BB_UPPER_ZERO,
    loRange: ENFORCE_LO_RANGE,
    rangeTable: ENFORCE_RANGE_TABLE,
    memBus: ENFORCE_MEM_BUS,
    progBus: ENFORCE_PROG_BUS
  }

  pure val SOUND_FLAGS: ConstraintFlags = {
    hiHeadroom: true,
    maxLoZero: true,
    qHiRange: true,
    progSelectors: true,
    bbUpperZero: true,
    loRange: true,
    rangeTable: true,
    memBus: true,
    progBus: true
  }

  pure val MUT_Q1A_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, hiHeadroom: false }
  pure val MUT_Q1B_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, maxLoZero: false }
  pure val MUT_Q2_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, qHiRange: false }
  pure val MUT_Q3A_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, progSelectors: false }
  pure val MUT_Q3B_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, bbUpperZero: false }
  pure val MUT_Q4A_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, loRange: false }
  pure val MUT_Q4B_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, rangeTable: false }
  pure val MUT_Q5A_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, memBus: false }
  pure val MUT_Q5B_FLAGS: ConstraintFlags = { ...SOUND_FLAGS, progBus: false }

  // =========================================================================
  // 2. Small-prime stand-in constants (preserving the exact limb decomposition)
  // =========================================================================
  // Real circuit: RANGE_SIZE = 2^16, P_MINUS_1_HI = 0x7800, P = 0x7800 * 2^16 + 1,
  // W = 11, R = BLS12-381 Fr (~2^255).
  // Stand-in:     RANGE_MAX  = 4,    P_HI_MAX     = 1,      P_SMALL = 1 * 4 + 1 = 5,
  // W_SMALL = 2 (quartic non-residue mod 5), R_SMALL = 337 (prime > max(q * P_SMALL + c) = 330).

  pure val RANGE_MAX: int = 4
  pure val P_HI_MAX: int = 1
  pure val P_SMALL: int = 5
  pure val W_SMALL: int = 2
  pure val R_SMALL: int = 337

  pure val TAG_INPUT_ID: int = 1
  pure val TAG_ALU_ID: int = 3

  pure val OP_ADD: int = 0
  pure val OP_SUB: int = 1
  pure val OP_MUL: int = 2

  // =========================================================================
  // 3. Types
  // =========================================================================

  type BusEntry = {
    msg: List[int],
    cnt: int
  }

  pure val EMPTY_BUS: List[BusEntry] = List()
  pure val EMPTY_INTS: List[int] = List()

  // `InputAir` row (tape.rs:548-578): main cells + program columns
  type InputRow = {
    valV: List[int],
    lo: List[int],
    hi: List[int],
    inv: List[int],
    isMax: List[int],
    addr: int,
    mult: int,
    zFlags: List[int],
    isConst: int,
    konst: List[int]
  }

  // `BbAluProgAir` row (alu.rs:93-131, tape.rs:673-699): main ALU cells + program columns
  type AluRowState = {
    selAdd: int,
    selSub: int,
    selMul: int,
    valA: List[int],
    valB: List[int],
    valC: List[int],
    qLimbs: List[List[int]],
    lo: List[int],
    hi: List[int],
    inv: List[int],
    isMax: List[int],
    addrA: int,
    addrB: int,
    addrC: int,
    multC: int
  }

  type WitnessState = {
    inpRows: List[InputRow],
    aluRows: List[AluRowState],
    rangeVal: List[int],
    rangeMult: List[int]
  }

  // =========================================================================
  // 4. Pure arithmetic and reference semantics
  // =========================================================================

  pure def modPos(x: int, m: int): int = {
    val rem = x % m
    if (rem < 0) rem + m else rem
  }

  pure def modR(x: int): int = modPos(x, R_SMALL)

  // Modular inverse in F_{R_SMALL}.
  pure def invModR(x: int): int = {
    val xm = modR(x)
    if (xm == 0) 0
    else if (xm == 1) 1
    else if (xm == R_SMALL - 1) R_SMALL - 1
    else if (xm == 2) 169
    else if (xm == R_SMALL - 2) R_SMALL - 169
    else
      1.to(R_SMALL - 1).fold(0, (found, cand) =>
        if (modR(xm * cand) == 1) cand else found
      )
  }

  // Reference unreduced integer e_k(a, b) for coefficient k in 0..3 (alu.rs:68-73, bb.rs:22-35)
  pure def unreducedCoeff(kind: int, a: List[int], b: List[int], k: int, pMod: int, wCoeff: int): int = {
    if (kind == OP_ADD)
      a.nth(k) + b.nth(k)
    else if (kind == OP_SUB)
      a.nth(k) + pMod - b.nth(k)
    else
      0.to(3).fold(0, (accI, i) =>
        0.to(3).fold(accI, (accJ, j) =>
          if (i + j == k)
            accJ + a.nth(i) * b.nth(j)
          else if (i + j == k + 4)
            accJ + wCoeff * a.nth(i) * b.nth(j)
          else
            accJ
        )
      )
  }

  pure def refAluUnreduced(kind: int, a: List[int], b: List[int]): List[int] =
    List(
      unreducedCoeff(kind, a, b, 0, P_SMALL, W_SMALL),
      unreducedCoeff(kind, a, b, 1, P_SMALL, W_SMALL),
      unreducedCoeff(kind, a, b, 2, P_SMALL, W_SMALL),
      unreducedCoeff(kind, a, b, 3, P_SMALL, W_SMALL)
    )

  pure def refAluResult(kind: int, a: List[int], b: List[int]): List[int] = {
    val e = refAluUnreduced(kind, a, b)
    List(
      e.nth(0) % P_SMALL,
      e.nth(1) % P_SMALL,
      e.nth(2) % P_SMALL,
      e.nth(3) % P_SMALL
    )
  }

  // Canonical witness cells for an integer c (alu.rs:156-164)
  pure def canonLo(c: int): int = c % RANGE_MAX
  pure def canonHi(c: int): int = c / RANGE_MAX
  pure def canonIsMax(hiVal: int): int = if (hiVal == P_HI_MAX) 1 else 0
  pure def canonInv(hiVal: int): int = {
    val z = modR(hiVal - P_HI_MAX)
    if (z == 0) 0 else invModR(z)
  }

  pure def qLimbsOf(q: int): List[int] =
    List(
      q % RANGE_MAX,
      (q / RANGE_MAX) % RANGE_MAX,
      q / (RANGE_MAX * RANGE_MAX)
    )

  // =========================================================================
  // 5. Honest row constructors and key-fixed program table
  // =========================================================================
  // Fixed program (tape):
  //   Input row 0: Op::Input(Bb) at addr 1, mult = 5 (honest input [4, 0, 0, 0])
  //   Input row 1: Op::Const(Bb, [1, 0, 0, 0]) at addr 2, mult = 1
  //   Input row 2: padding (nop)
  //   Input row 3: padding (nop)
  //   ALU row 0:   Op::Alu(Add, addr 1, addr 1 -> addr 3, mult 0)
  //   ALU row 1:   Op::Alu(Sub, addr 2, addr 1 -> addr 4, mult 0)
  //   ALU row 2:   Op::Alu(Mul, addr 1, addr 1 -> addr 5, mult 0)
  //   ALU row 3:   padding (nop)

  pure def buildInputRow(
    v: List[int],
    addr: int,
    mult: int,
    zFlags: List[int],
    isConst: int,
    konst: List[int]
  ): InputRow = {
    val loList = List(canonLo(v.nth(0)), canonLo(v.nth(1)), canonLo(v.nth(2)), canonLo(v.nth(3)))
    val hiList = List(canonHi(v.nth(0)), canonHi(v.nth(1)), canonHi(v.nth(2)), canonHi(v.nth(3)))
    val invList = List(canonInv(hiList.nth(0)), canonInv(hiList.nth(1)), canonInv(hiList.nth(2)), canonInv(hiList.nth(3)))
    val maxList = List(canonIsMax(hiList.nth(0)), canonIsMax(hiList.nth(1)), canonIsMax(hiList.nth(2)), canonIsMax(hiList.nth(3)))
    {
      valV: v,
      lo: loList,
      hi: hiList,
      inv: invList,
      isMax: maxList,
      addr: addr,
      mult: mult,
      zFlags: zFlags,
      isConst: isConst,
      konst: konst
    }
  }

  pure def buildHonestAluRow(
    kind: int,
    a: List[int],
    b: List[int],
    addrA: int,
    addrB: int,
    addrC: int,
    multC: int
  ): AluRowState = {
    val e = refAluUnreduced(kind, a, b)
    val c = List(e.nth(0) % P_SMALL, e.nth(1) % P_SMALL, e.nth(2) % P_SMALL, e.nth(3) % P_SMALL)
    val ql = List(
      qLimbsOf(e.nth(0) / P_SMALL),
      qLimbsOf(e.nth(1) / P_SMALL),
      qLimbsOf(e.nth(2) / P_SMALL),
      qLimbsOf(e.nth(3) / P_SMALL)
    )
    val loList = List(canonLo(c.nth(0)), canonLo(c.nth(1)), canonLo(c.nth(2)), canonLo(c.nth(3)))
    val hiList = List(canonHi(c.nth(0)), canonHi(c.nth(1)), canonHi(c.nth(2)), canonHi(c.nth(3)))
    val invList = List(canonInv(hiList.nth(0)), canonInv(hiList.nth(1)), canonInv(hiList.nth(2)), canonInv(hiList.nth(3)))
    val maxList = List(canonIsMax(hiList.nth(0)), canonIsMax(hiList.nth(1)), canonIsMax(hiList.nth(2)), canonIsMax(hiList.nth(3)))
    {
      selAdd: if (kind == OP_ADD) 1 else 0,
      selSub: if (kind == OP_SUB) 1 else 0,
      selMul: if (kind == OP_MUL) 1 else 0,
      valA: a,
      valB: b,
      valC: c,
      qLimbs: ql,
      lo: loList,
      hi: hiList,
      inv: invList,
      isMax: maxList,
      addrA: addrA,
      addrB: addrB,
      addrC: addrC,
      multC: multC
    }
  }

  pure val padAluRow: AluRowState = {
    val z4 = List(0, 0, 0, 0)
    val inv0 = canonInv(0)
    {
      selAdd: 0,
      selSub: 0,
      selMul: 0,
      valA: z4,
      valB: z4,
      valC: z4,
      qLimbs: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      lo: z4,
      hi: z4,
      inv: List(inv0, inv0, inv0, inv0),
      isMax: z4,
      addrA: 0,
      addrB: 0,
      addrC: 0,
      multC: 0
    }
  }

  // Range lookups emitted by one `eval_canonical` instance (alu.rs:150-152)
  pure def canonicalRangeLookupsWith(loVal: int, hiVal: int, cf: ConstraintFlags): List[BusEntry] = {
    val l0 = if (cf.loRange) List({ msg: List(modR(loVal)), cnt: 1 }) else EMPTY_BUS
    val l1 = List({ msg: List(modR(hiVal)), cnt: 1 })
    val l2 = if (cf.hiHeadroom) List({ msg: List(modR(P_HI_MAX - hiVal)), cnt: 1 }) else EMPTY_BUS
    l0.concat(l1).concat(l2)
  }

  // Range lookups emitted by one `InputRow` (12 lookups when all flags true)
  pure def inputRowRangeLookupsWith(r: InputRow, cf: ConstraintFlags): List[BusEntry] =
    range(0, 4).foldl(EMPTY_BUS, (accL, k) =>
      accL.concat(canonicalRangeLookupsWith(r.lo.nth(k), r.hi.nth(k), cf))
    )

  // Range lookups emitted by one `AluRowState` (24 lookups when all flags true; alu.rs:125-128)
  pure def aluRowRangeLookupsWith(r: AluRowState, cf: ConstraintFlags): List[BusEntry] =
    range(0, 4).foldl(EMPTY_BUS, (accL, k) => {
      val canL = canonicalRangeLookupsWith(r.lo.nth(k), r.hi.nth(k), cf)
      val qk = r.qLimbs.nth(k)
      val qL01 = List(
        { msg: List(modR(qk.nth(0))), cnt: 1 },
        { msg: List(modR(qk.nth(1))), cnt: 1 }
      )
      val qL2 = if (cf.qHiRange) List({ msg: List(modR(qk.nth(2))), cnt: 1 }) else EMPTY_BUS
      accL.concat(canL).concat(qL01).concat(qL2)
    })

  // All query lookups sent on `RANGE_BUS` by `InputAir` and `BbAluProgAir`
  pure def allRangeQueriesWith(inpRows: List[InputRow], aluRows: List[AluRowState], cf: ConstraintFlags): List[BusEntry] = {
    val fromInp = inpRows.foldl(EMPTY_BUS, (accL, r) => accL.concat(inputRowRangeLookupsWith(r, cf)))
    aluRows.foldl(fromInp, (accL, r) => accL.concat(aluRowRangeLookupsWith(r, cf)))
  }

  // Fill `Range16Air` multiplicities for 0..RANGE_MAX-1 from the queries (testing.rs:115-146)
  // Note: just as in `testing::range_trace`, the prover computes table multiplicities for the
  // active circuit's interactions (or for all 24 lookups; for values in 0..RANGE_MAX-1 any
  // dropped lookup simply reduces that value's query count).
  pure def computeRangeMultsWith(inpRows: List[InputRow], aluRows: List[AluRowState], cf: ConstraintFlags): List[int] = {
    val queries = allRangeQueriesWith(inpRows, aluRows, cf)
    range(0, RANGE_MAX).foldl(EMPTY_INTS, (accM, v) => {
      val c = queries.foldl(0, (tot, q) => if (q.msg == List(v)) modR(tot + q.cnt) else tot)
      accM.append(c)
    })
  }

  pure def computeRangeMults(inpRows: List[InputRow], aluRows: List[AluRowState]): List[int] =
    computeRangeMultsWith(inpRows, aluRows, activeFlags)

  // Build a complete honest witness for any BabyBear input vector `v1` at addr 1
  pure def buildWitnessForInputWith(v1: List[int], cf: ConstraintFlags): WitnessState = {
    val z4 = List(0, 0, 0, 0)
    val c1 = List(1, 0, 0, 0)
    val inp0 = buildInputRow(v1, 1, 5, List(1, 1, 1), 0, z4)
    val inp1 = buildInputRow(c1, 2, 1, List(1, 1, 1), 1, c1)
    val inpPad = buildInputRow(z4, 0, 0, List(0, 0, 0), 0, z4)
    val inpRows = List(inp0, inp1, inpPad, inpPad)
    val alu0 = buildHonestAluRow(OP_ADD, v1, v1, 1, 1, 3, 0)
    val alu1 = buildHonestAluRow(OP_SUB, c1, v1, 2, 1, 4, 0)
    val alu2 = buildHonestAluRow(OP_MUL, v1, v1, 1, 1, 5, 0)
    val aluRows = List(alu0, alu1, alu2, padAluRow)
    {
      inpRows: inpRows,
      aluRows: aluRows,
      rangeVal: List(0, 1, 2, 3),
      rangeMult: computeRangeMultsWith(inpRows, aluRows, cf)
    }
  }

  pure def buildWitnessForInput(v1: List[int]): WitnessState =
    buildWitnessForInputWith(v1, activeFlags)

  pure val honestWitness: WitnessState = buildWitnessForInput(List(4, 0, 0, 0))

  // =========================================================================
  // 6. Polynomial gate constraints and bus interactions (as built in Rust)
  // =========================================================================

  pure def isBoolModR(x: int): bool = {
    val xm = modR(x)
    xm == 0 or xm == 1
  }

  // `eval_canonical` polynomial gates (alu.rs:144-148)
  pure def evalCanonicalGateWith(cVal: int, loVal: int, hiVal: int, invVal: int, isMaxVal: int, cf: ConstraintFlags): bool = {
    val z = modR(hiVal - P_HI_MAX)
    and {
      modR(cVal) == modR(loVal + hiVal * RANGE_MAX),
      modR(isMaxVal) == modR(1 - modR(z * invVal)),
      modR(z * isMaxVal) == 0,
      if (cf.maxLoZero) modR(isMaxVal * loVal) == 0 else true
    }
  }

  // `InputAir::eval` polynomial gates on one row (tape.rs:558-573)
  pure def evalInputRowGatesWith(r: InputRow, cf: ConstraintFlags): bool =
    0.to(3).forall(k => {
      val v = r.valV.nth(k)
      and {
        evalCanonicalGateWith(v, r.lo.nth(k), r.hi.nth(k), r.inv.nth(k), r.isMax.nth(k), cf),
        if (k > 0 and cf.bbUpperZero)
          modR(r.zFlags.nth(k - 1) * v) == 0
        else
          true,
        modR(r.isConst * (v - r.konst.nth(k))) == 0
      }
    })

  // `BbAluAir::eval` polynomial gates on one row (alu.rs:99-129)
  pure def evalAluRowGatesWith(r: AluRowState, cf: ConstraintFlags): bool = {
    val selOk = and {
      isBoolModR(r.selAdd),
      isBoolModR(r.selSub),
      isBoolModR(r.selMul),
      isBoolModR(r.selAdd + r.selSub + r.selMul)
    }
    val coeffsOk = 0.to(3).forall(k => {
      val conv = unreducedCoeff(OP_MUL, r.valA, r.valB, k, P_SMALL, W_SMALL)
      val e = modR(
        r.selAdd * (r.valA.nth(k) + r.valB.nth(k))
        + r.selSub * (r.valA.nth(k) - r.valB.nth(k) + P_SMALL)
        + r.selMul * conv
      )
      val qk = r.qLimbs.nth(k)
      val q = modR(qk.nth(0) + qk.nth(1) * RANGE_MAX + qk.nth(2) * RANGE_MAX * RANGE_MAX)
      val ck = r.valC.nth(k)
      and {
        e == modR(q * P_SMALL + ck),
        evalCanonicalGateWith(ck, r.lo.nth(k), r.hi.nth(k), r.inv.nth(k), r.isMax.nth(k), cf)
      }
    })
    selOk and coeffsOk
  }

  // `Range16Air::eval` polynomial gates (range.rs:38-44)
  pure def evalRange16GatesWith(rVal: List[int], rMult: List[int], cf: ConstraintFlags): bool = {
    if (not(cf.rangeTable))
      rVal.length() == RANGE_MAX and rMult.length() == RANGE_MAX
    else
      and {
        rVal.length() == RANGE_MAX,
        rMult.length() == RANGE_MAX,
        modR(rVal.nth(0)) == 0,
        0.to(RANGE_MAX - 2).forall(i => modR(rVal.nth(i + 1)) == modR(rVal.nth(i) + 1)),
        modR(rVal.nth(RANGE_MAX - 1)) == RANGE_MAX - 1
      }
  }

  // Generic LogUp bus balance check: sum of `cnt` per distinct `msg` is 0 mod R_SMALL
  pure def msgCountMod(entries: List[BusEntry], m: List[int]): int =
    entries.foldl(0, (tot, e) =>
      if (e.msg == m) modR(tot + e.cnt) else tot
    )

  pure def isBusBalanced(entries: List[BusEntry]): bool =
    entries.indices().forall(i => msgCountMod(entries, entries.nth(i).msg) == 0)

  // `RANGE_BUS` (bus 0) balance: queries (+1) + `Range16Air` receives (-mult)
  pure def rangeBusBalancedWith(w: WitnessState, cf: ConstraintFlags): bool = {
    val queries = allRangeQueriesWith(w.inpRows, w.aluRows, cf)
    val tableEntries = range(0, RANGE_MAX).foldl(EMPTY_BUS, (accE, i) =>
      accE.append({ msg: List(modR(w.rangeVal.nth(i))), cnt: modR(-w.rangeMult.nth(i)) })
    )
    isBusBalanced(queries.concat(tableEntries))
  }

  // `MEM_BUS` (bus 2) balance (tape.rs:574-577, 689-698)
  pure def memBusEntries(w: WitnessState): List[BusEntry] = {
    val inpEntries = w.inpRows.foldl(EMPTY_BUS, (accE, r) =>
      accE.append({
        msg: List(
          modR(r.addr),
          modR(r.valV.nth(0)),
          modR(r.valV.nth(1)),
          modR(r.valV.nth(2)),
          modR(r.valV.nth(3))
        ),
        cnt: modR(r.mult)
      })
    )
    w.aluRows.foldl(inpEntries, (accE, r) => {
      val active = modR(r.selAdd + r.selSub + r.selMul)
      val readA = {
        msg: List(
          modR(r.addrA),
          modR(r.valA.nth(0)),
          modR(r.valA.nth(1)),
          modR(r.valA.nth(2)),
          modR(r.valA.nth(3))
        ),
        cnt: modR(-active)
      }
      val readB = {
        msg: List(
          modR(r.addrB),
          modR(r.valB.nth(0)),
          modR(r.valB.nth(1)),
          modR(r.valB.nth(2)),
          modR(r.valB.nth(3))
        ),
        cnt: modR(-active)
      }
      val writeEntry = {
        msg: List(
          modR(r.addrC),
          modR(r.valC.nth(0)),
          modR(r.valC.nth(1)),
          modR(r.valC.nth(2)),
          modR(r.valC.nth(3))
        ),
        cnt: modR(r.multC)
      }
      accE.append(readA).append(readB).append(writeEntry)
    })
  }

  pure def memBusBalancedWith(w: WitnessState, cf: ConstraintFlags): bool =
    if (not(cf.memBus)) true else isBusBalanced(memBusEntries(w))

  // `PROG_BUS` (bus 3) messages and balance (program.rs:51-93, tape.rs:522-543, 650-668, 683-688)
  pure def inputProgMsg(r: InputRow): List[int] =
    List(
      TAG_INPUT_ID,
      modR(r.addr),
      modR(r.mult),
      modR(r.zFlags.nth(0)),
      modR(r.zFlags.nth(1)),
      modR(r.zFlags.nth(2)),
      modR(r.isConst),
      modR(r.konst.nth(0)),
      modR(r.konst.nth(1)),
      modR(r.konst.nth(2)),
      modR(r.konst.nth(3))
    )

  pure def aluProgMsgWith(r: AluRowState, cf: ConstraintFlags): List[int] = {
    val sAdd = if (cf.progSelectors) modR(r.selAdd) else 0
    val sSub = if (cf.progSelectors) modR(r.selSub) else 0
    val sMul = if (cf.progSelectors) modR(r.selMul) else 0
    List(
      TAG_ALU_ID,
      sAdd,
      sSub,
      sMul,
      modR(r.addrA),
      modR(r.addrB),
      modR(r.addrC),
      modR(r.multC)
    )
  }

  // Key-fixed `ProgramTableAir` preprocessed trace sends (program.rs:79-93, 138)
  pure def keyProgramTableEntriesWith(cf: ConstraintFlags): List[BusEntry] = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val inpKey = List(
      { msg: inputProgMsg(hw.inpRows.nth(0)), cnt: 1 },
      { msg: inputProgMsg(hw.inpRows.nth(1)), cnt: 1 },
      { msg: inputProgMsg(hw.inpRows.nth(2)), cnt: 2 }
    )
    val aluKey = List(
      { msg: aluProgMsgWith(hw.aluRows.nth(0), cf), cnt: 1 },
      { msg: aluProgMsgWith(hw.aluRows.nth(1), cf), cnt: 1 },
      { msg: aluProgMsgWith(hw.aluRows.nth(2), cf), cnt: 1 },
      { msg: aluProgMsgWith(hw.aluRows.nth(3), cf), cnt: 1 }
    )
    inpKey.concat(aluKey)
  }

  pure def progBusBalancedWith(w: WitnessState, cf: ConstraintFlags): bool = {
    if (not(cf.progBus))
      true
    else {
      val inpRecvs = w.inpRows.foldl(EMPTY_BUS, (accE, r) =>
        accE.append({ msg: inputProgMsg(r), cnt: modR(-1) })
      )
      val aluRecvs = w.aluRows.foldl(EMPTY_BUS, (accE, r) =>
        accE.append({ msg: aluProgMsgWith(r, cf), cnt: modR(-1) })
      )
      isBusBalanced(keyProgramTableEntriesWith(cf).concat(inpRecvs).concat(aluRecvs))
    }
  }

  // Full circuit acceptance predicate parameterized by `ConstraintFlags`
  pure def isAcceptedWith(w: WitnessState, cf: ConstraintFlags): bool = and {
    w.inpRows.length() == 4,
    w.aluRows.length() == 4,
    w.inpRows.indices().forall(i => evalInputRowGatesWith(w.inpRows.nth(i), cf)),
    w.aluRows.indices().forall(i => evalAluRowGatesWith(w.aluRows.nth(i), cf)),
    evalRange16GatesWith(w.rangeVal, w.rangeMult, cf),
    rangeBusBalancedWith(w, cf),
    memBusBalancedWith(w, cf),
    progBusBalancedWith(w, cf)
  }

  pure def isAccepted(w: WitnessState): bool =
    isAcceptedWith(w, activeFlags)

  // =========================================================================
  // 7. Reference refinement predicate (`invRefine`)
  // =========================================================================
  // An accepted witness refines the reference semantics iff:
  //   - Input row 0 (Op::Input(Bb) at addr 1) holds a canonical BabyBear value [v0, 0, 0, 0]
  //     with 0 <= v0 < P_SMALL;
  //   - Input row 1 (Op::Const(Bb, [1,0,0,0]) at addr 2) holds [1, 0, 0, 0];
  //   - Active ALU rows (those writing addr 3, 4, 5) produce outputs equal to the reference
  //     BabyBear/EF4 semantics on the input values at addr 1 and addr 2.

  pure def isCanonicalBbVec(v: List[int]): bool = and {
    v.length() == 4,
    0 <= v.nth(0) and v.nth(0) < P_SMALL,
    v.nth(1) == 0,
    v.nth(2) == 0,
    v.nth(3) == 0
  }

  pure def witnessRefines(w: WitnessState): bool = {
    val v1 = w.inpRows.nth(0).valV
    val v2 = w.inpRows.nth(1).valV
    val wantAdd = refAluResult(OP_ADD, v1, v1)
    val wantSub = refAluResult(OP_SUB, List(1, 0, 0, 0), v1)
    val wantMul = refAluResult(OP_MUL, v1, v1)
    and {
      isCanonicalBbVec(v1),
      v2 == List(1, 0, 0, 0),
      w.aluRows.nth(0).valC == wantAdd,
      w.aluRows.nth(1).valC == wantSub,
      w.aluRows.nth(2).valC == wantMul
    }
  }

  // =========================================================================
  // 8. Attack witness builders (used by nondeterministic prover `step` & tests)
  // =========================================================================

  // Attack 1a (for `mut_q1a_no_hi_headroom`):
  // On ALU row 0 (Add(4, 4), e_0 = 8 = 1*5 + 3), set q_0 = 0 and c_0 = 3 + P_SMALL = 8
  // with lo = 0, hi = 2 (> P_HI_MAX = 1), is_max = 0, inv = 1.
  pure def attackQ1aHiHeadroomWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val r0 = hw.aluRows.nth(0)
    val badR0 = {
      ...r0,
      valC: r0.valC.replaceAt(0, 8),
      qLimbs: r0.qLimbs.replaceAt(0, List(0, 0, 0)),
      lo: r0.lo.replaceAt(0, 0),
      hi: r0.hi.replaceAt(0, 2),
      inv: r0.inv.replaceAt(0, canonInv(2)),
      isMax: r0.isMax.replaceAt(0, 0)
    }
    val newAlu = hw.aluRows.replaceAt(0, badR0)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 1b (for `mut_q1b_no_max_lo_zero`):
  // On ALU row 2 (Mul(4, 4), e_0 = 16 = 3*5 + 1), set q_0 = 2 and c_0 = 1 + P_SMALL = 6
  // with hi = 1 == P_HI_MAX, lo = 2 != 0, is_max = 1, inv = 0.
  pure def attackQ1bMaxLoZeroWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val r2 = hw.aluRows.nth(2)
    val badR2 = {
      ...r2,
      valC: r2.valC.replaceAt(0, 6),
      qLimbs: r2.qLimbs.replaceAt(0, List(2, 0, 0)),
      lo: r2.lo.replaceAt(0, 2),
      hi: r2.hi.replaceAt(0, P_HI_MAX),
      inv: r2.inv.replaceAt(0, 0),
      isMax: r2.isMax.replaceAt(0, 1)
    }
    val newAlu = hw.aluRows.replaceAt(2, badR2)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 2 (for `mut_q2_no_q_range`):
  // On ALU row 1 (Sub(1, 4), e_0 = 2), set ql[0] = [3, 1, 8] (q_0 = 135) and c_0 = 1:
  // 135 * 5 + 1 = 676 == 2 (mod 337), wrapping R_SMALL to forge canonical c_0 = 1 != 2.
  pure def attackQ2QuotientWrapWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val r1 = hw.aluRows.nth(1)
    val badR1 = {
      ...r1,
      valC: r1.valC.replaceAt(0, 1),
      qLimbs: r1.qLimbs.replaceAt(0, List(3, 1, 8)),
      lo: r1.lo.replaceAt(0, 1),
      hi: r1.hi.replaceAt(0, 0),
      inv: r1.inv.replaceAt(0, canonInv(0)),
      isMax: r1.isMax.replaceAt(0, 0)
    }
    val newAlu = hw.aluRows.replaceAt(1, badR1)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 3a (for `mut_q3a_unpinned_op_kind`):
  // On ALU row 1 (programmed as Sub(addr 2, addr 1) -> [2, 0, 0, 0]), switch selectors
  // to Add(addr 2, addr 1) -> [0, 0, 0, 0] (since 1 + 4 = 5 = 1*5 + 0).
  pure def attackQ3aOpKindSwapWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val badR1 = buildHonestAluRow(OP_ADD, List(1, 0, 0, 0), List(4, 0, 0, 0), 2, 1, 4, 0)
    val newAlu = hw.aluRows.replaceAt(1, badR1)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 3b (for `mut_q3b_no_bb_upper_zero`):
  // On Input row 0 (a BabyBear input at addr 1), inject non-zero upper coefficients
  // [4, 1, 0, 1]; EF4 convolution in ALU row 2 (Mul) mixes 2 * (1*1 + 1*1) = 4 into e_0,
  // changing even the 0-th coefficient of Mul(addr 1, addr 1) from 1 to 0.
  pure def attackQ3bBbUpperNonZeroWith(cf: ConstraintFlags): WitnessState =
    buildWitnessForInputWith(List(4, 1, 0, 1), cf)

  // Attack 4a (for `mut_q4a_no_lo_range`):
  // On ALU row 1 (Sub(1, 4), e_0 = 2), set ql[0] = [1, 0, 3] (q_0 = 49) and c_0 = 94
  // with lo = 94 (>= RANGE_MAX), hi = 0, is_max = 0, inv = canonInv(0):
  // 49 * 5 + 94 = 339 == 2 == e_0 (mod 337), and every other lookup in the machine is in {0, 1, 3}.
  pure def attackQ4aLoOutOfRangeWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val r1 = hw.aluRows.nth(1)
    val badR1 = {
      ...r1,
      valC: r1.valC.replaceAt(0, 94),
      qLimbs: r1.qLimbs.replaceAt(0, List(1, 0, 3)),
      lo: r1.lo.replaceAt(0, 94),
      hi: r1.hi.replaceAt(0, 0),
      inv: r1.inv.replaceAt(0, canonInv(0)),
      isMax: r1.isMax.replaceAt(0, 0)
    }
    val newAlu = hw.aluRows.replaceAt(1, badR1)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 4b (for `mut_q4b_unpinned_range_table`):
  // Take Attack 4a (where ALU row 1's lo[0] = 94 is the only lookup outside {0, 1, 3}, and
  // value 2 is looked up 0 times across the entire machine) and tamper unused row 2 of
  // `Range16Air` to hold value 94 with mult 1, balancing `RANGE_BUS`.
  pure def attackQ4bRangeTableForgeWith(cf: ConstraintFlags): WitnessState = {
    val w4 = attackQ4aLoOutOfRangeWith(cf)
    {
      ...w4,
      rangeVal: w4.rangeVal.replaceAt(2, 94),
      rangeMult: w4.rangeMult.replaceAt(2, 1)
    }
  }

  // Attack 5a (for `mut_q5a_no_mem_bus`):
  // ALU row 0 ignores `mem[1] = [4, 0, 0, 0]` and computes Add([2,0,0,0], [4,0,0,0]) = [1,0,0,0].
  pure def attackQ5aMemDisconnectWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val badR0 = buildHonestAluRow(OP_ADD, List(2, 0, 0, 0), List(4, 0, 0, 0), 1, 1, 3, 0)
    val newAlu = hw.aluRows.replaceAt(0, badR0)
    {
      ...hw,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(hw.inpRows, newAlu, cf)
    }
  }

  // Attack 5b (for `mut_q5b_no_prog_bus`, matching negative control #5 of tests/i4_tape.rs):
  // Input padding rows 2 and 3 forge `[addr 2, 2, 0, 0, 0]` (mult 1) and `[addr 2, 1, 0, 0, 0]`
  // (mult -1 == 336 mod 337), cancelling Input row 1's constant `[1, 0, 0, 0]` on `MEM_BUS` so
  // ALU row 1 reads `[2, 0, 0, 0]` at addr 2 and outputs `Sub(2, 4) = [3, 0, 0, 0] != [2, 0, 0, 0]`.
  pure def attackQ5bPaddingForgeWith(cf: ConstraintFlags): WitnessState = {
    val hw = buildWitnessForInputWith(List(4, 0, 0, 0), cf)
    val z4 = List(0, 0, 0, 0)
    val forgeSend = buildInputRow(List(2, 0, 0, 0), 2, 1, List(1, 1, 1), 0, z4)
    val forgeCancel = buildInputRow(List(1, 0, 0, 0), 2, modR(-1), List(1, 1, 1), 0, z4)
    val newInp = hw.inpRows.replaceAt(2, forgeSend).replaceAt(3, forgeCancel)
    val badR1 = buildHonestAluRow(OP_SUB, List(2, 0, 0, 0), List(4, 0, 0, 0), 2, 1, 4, 0)
    val newAlu = hw.aluRows.replaceAt(1, badR1)
    {
      ...hw,
      inpRows: newInp,
      aluRows: newAlu,
      rangeMult: computeRangeMultsWith(newInp, newAlu, cf)
    }
  }

  // =========================================================================
  // 9. State variables, actions, and invariants
  // =========================================================================

  var witness: WitnessState
  var acc: bool

  action assignState(w: WitnessState): bool = all {
    witness' = w,
    acc' = isAccepted(w),
  }

  action init: bool = assignState(honestWitness)

  action step: bool = {
    nondet altV0 = 0.to(P_SMALL - 1).oneOf()
    any {
      assignState(buildWitnessForInput(List(altV0, 0, 0, 0))),
      assignState(attackQ1aHiHeadroomWith(activeFlags)),
      assignState(attackQ1bMaxLoZeroWith(activeFlags)),
      assignState(attackQ2QuotientWrapWith(activeFlags)),
      assignState(attackQ3aOpKindSwapWith(activeFlags)),
      assignState(attackQ3bBbUpperNonZeroWith(activeFlags)),
      assignState(attackQ4aLoOutOfRangeWith(activeFlags)),
      assignState(attackQ4bRangeTableForgeWith(activeFlags)),
      assignState(attackQ5aMemDisconnectWith(activeFlags)),
      assignState(attackQ5bPaddingForgeWith(activeFlags)),
    }
  }

  // Main soundness refinement invariant:
  // Every accepted witness must refine the reference BabyBear/EF4 semantics.
  val invRefine: bool = acc implies witnessRefines(witness)

  // Negated reachability witnesses (MUST be violated in the sound instance for non-vacuity):
  // 1. A tampered witness that violates refinement is reached and rejected (`not(acc)`).
  val reachTamperedRejected: bool = not(not(acc) and not(witnessRefines(witness)))

  // 2. A witness for a different valid input (v0 = 2, triggering `isMax == 1` on ALU row 0
  //    since 2 + 2 = 4 = P_SMALL - 1) is reached and accepted (`acc`).
  val reachAcceptedMaxEdge: bool = not(
    acc and witness.inpRows.nth(0).valV == List(2, 0, 0, 0) and witness.aluRows.nth(0).isMax.nth(0) == 1
  )
}

// ===========================================================================
// Sound instance (all constraints enabled, matching the Rust circuit as built)
// ===========================================================================
module lfr_bbalu {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// ===========================================================================
// Mutant instances (one constraint weakened or deleted per mutant)
// ===========================================================================

// Q1a mutant: omits `0x7800 - hi` lookup on `RANGE_BUS` in `eval_canonical` (alu.rs:152)
module mut_q1a_no_hi_headroom {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = false,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q1b mutant: omits `is_max * lo == 0` in `eval_canonical` (alu.rs:148)
module mut_q1b_no_max_lo_zero {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = false,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q2 mutant: omits top quotient limb `ql[k][2]` lookup on `RANGE_BUS` (alu.rs:127)
module mut_q2_no_q_range {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = false,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q3a mutant: omits ALU op-kind selectors from `PROG_BUS` receipt in `BbAluProgAir` (tape.rs:683)
module mut_q3a_unpinned_op_kind {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = false,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q3b mutant: omits `z_k * v_k == 0` upper-coeff zero check in `InputAir` (tape.rs:569)
module mut_q3b_no_bb_upper_zero {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = false,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q4a mutant: omits `lo` range lookup on `RANGE_BUS` in `eval_canonical` (alu.rs:150)
module mut_q4a_no_lo_range {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = false,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q4b mutant: omits `Range16Air` row-value constraints (range.rs:38-44)
module mut_q4b_unpinned_range_table {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = false,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q5a mutant: omits `MEM_BUS` operand reads in `BbAluProgAir` (tape.rs:695-696)
module mut_q5a_no_mem_bus {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = false,
    ENFORCE_PROG_BUS = true
  ).*
}

// Q5b mutant: omits `PROG_BUS` receipt in `InputAir` and `BbAluProgAir` (tape.rs:557, 688)
module mut_q5b_no_prog_bus {
  import lfr_bbalu_base(
    ENFORCE_HI_HEADROOM = true,
    ENFORCE_MAX_LO_ZERO = true,
    ENFORCE_Q_HI_RANGE = true,
    ENFORCE_PROG_SELECTORS = true,
    ENFORCE_BB_UPPER_ZERO = true,
    ENFORCE_LO_RANGE = true,
    ENFORCE_RANGE_TABLE = true,
    ENFORCE_MEM_BUS = true,
    ENFORCE_PROG_BUS = false
  ).*
}
