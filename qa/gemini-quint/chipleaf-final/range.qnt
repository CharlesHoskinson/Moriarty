// Soundness Review Model — Running-Sum RANGE (`range.rs`) and ALU (`alu.rs`)
// Design under test:
//   - crates/mori-chip/src/range.rs (RangeConfig::configure_running, assign_running, recompose, center)
//   - crates/mori-chip/src/alu.rs (AluConfig::configure, assign, AluOp::eval, AluOp::w)
//   - crates/mori-chip/src/rfcore.rs (lines 1204-1208: s consecutive ALU ops after core rows)
//
// Scaled parameters (exact structure of running-sum mode):
//   - B = 2 bits per limb (table 0..2^B - 1 = {0, 1, 2, 3}, BASE = 4)
//   - L = 3 limbs per value (CELLS = L + 1 = 4 rows per value: row 0 = pad, rows 1..3 = limb2..limb0)
//   - T = 1 top-limb bit (PAD_SHIFT = 2^(B - T) = 2, top limb < 2^T = 2)
//   - Claimed range bound: RANGE_BOUND = 2^(B * (L - 1) + T) = 2^5 = 32
//   - Field arithmetic modulo prime P (P = 97 in standard instances; P = 23 in small-prime wrap-around instance)
//   - Layout: 2 preceding rows from another region (rows 0..1) followed by 2 back-to-back ALU ops (Op 0, Op 1)

module rangechip {
  const P: int
  const MUTANT: str

  // ---------------------------------------------------------------- Scaled Constants
  pure val B_BITS: int = 2
  pure val BASE: int = 4          // 2^B
  pure val LIMBS: int = 3         // L = 3 limbs (limb2, limb1, limb0)
  pure val CELLS: int = 4         // 1 pad cell + 3 limb cells
  pure val T_BITS: int = 1
  pure val PAD_SHIFT: int = 2     // 2^(B - T) = 2
  pure val TOP_BOUND: int = 2     // 2^T = 2
  pure val RANGE_BOUND: int = 32  // 2^(B * (L - 1) + T) = 2^5 = 32

  pure val TABLE: Set[int] = Set(0, 1, 2, 3)

  // Preceding rows (rows 0..1 belong to the core region right before the first ALU op)
  pure val PRE_ROWS: int = 2
  pure val TOTAL_ROWS: int = 12
  pure val NUM_OPS: int = 2

  // ALU op codes (for Q2c ALU relation checks)
  pure val OP_ADD: int = 1
  pure val OP_LT: int = 2
  pure val FLAG_SHIFT: int = 8
  pure val CHIP_ALU: int = 1

  // ---------------------------------------------------------------- Types
  type FixedRow = {
    q_rng: bool,
    q_first: bool,
    q_step: bool,
    q_alu: bool
  }

  type OpSpec = {
    opId: int,
    padRow: int,          // -1 in R7 `shared_sum` where Op 1 has no pad row
    limbRows: List[int],  // row indices of the op's limbs from top limb to bottom limb
    gateRow: int          // row index where the ALU gate reads `acc`
  }

  type Layout = {
    fixedRows: List[FixedRow],
    ops: List[OpSpec]
  }

  type AdviceRow = {
    limb: int,  // column a[11]
    acc: int    // column a[10]
  }

  type Witness = {
    rows: List[AdviceRow],
    opW: List[int]  // decomposed value `w` seen by each op's ALU gate
  }

  type AluTupleW = {
    chip: int,
    key: int,
    x: int,
    y: int,
    z: int,
    op: int,
    flag: int,
    w: int
  }

  // ---------------------------------------------------------------- Field Arithmetic Modulo Prime P
  pure def modPrime(pVal: int, x: int): int = ((x % pVal) + pVal) % pVal
  pure def modP(x: int): int = modPrime(P, x)

  pure val ZERO_ADV: AdviceRow = { limb: 0, acc: 0 }

  pure val emptyFixedRow: FixedRow = {
    q_rng: false,
    q_first: false,
    q_step: false,
    q_alu: false
  }

  // ---------------------------------------------------------------- Fixed Layout Builder (Keygen)
  // Standard 4-row running-sum block for one op
  pure def buildStandardOpFixed(mt: str): List[FixedRow] = {
    val r0: FixedRow = {
      q_rng: (mt != "pad_row_no_lookup"),
      q_first: false,
      q_step: (mt == "step_on_pad_row"),
      q_alu: (mt == "gate_off_centre")
    }
    val r1: FixedRow = {
      q_rng: true,
      q_first: (mt != "no_first" and mt != "first_dropped"),
      q_step: (mt == "no_first"),
      q_alu: false
    }
    val r2: FixedRow = {
      q_rng: true,
      q_first: false,
      q_step: true,
      q_alu: (mt == "gate_row10")
    }
    val r3: FixedRow = {
      q_rng: true,
      q_first: false,
      q_step: true,
      q_alu: (mt != "gate_off_centre" and mt != "gate_row10")
    }
    [r0, r1, r2, r3]
  }

  pure def buildLayout(mt: str): Layout = {
    val preFixed: List[FixedRow] = [emptyFixedRow, emptyFixedRow]
    if (mt == "extra_step") {
      // R5 `extra_step`: each value takes 5 rows (1 pad row + 4 limb rows) with gate on the last row
      val opFixed: List[FixedRow] = [
        { ...emptyFixedRow, q_rng: true },
        { ...emptyFixedRow, q_rng: true, q_first: true },
        { ...emptyFixedRow, q_rng: true, q_step: true },
        { ...emptyFixedRow, q_rng: true, q_step: true },
        { ...emptyFixedRow, q_rng: true, q_step: true, q_alu: true }
      ]
      val fixedRows = preFixed.concat(opFixed).concat(opFixed)
      val ops: List[OpSpec] = [
        { opId: 0, padRow: 2, limbRows: [3, 4, 5, 6], gateRow: 6 },
        { opId: 1, padRow: 7, limbRows: [8, 9, 10, 11], gateRow: 11 }
      ]
      { fixedRows: fixedRows, ops: ops }
    } else if (mt == "shared_sum") {
      // R7 `shared_sum`: Op 0 at rows 2..5; Op 1 adjacent at rows 6..8 with NO pad row and q_step on row 6
      val op0Fixed = buildStandardOpFixed("none")
      val op1Fixed: List[FixedRow] = [
        { ...emptyFixedRow, q_rng: true, q_step: true },
        { ...emptyFixedRow, q_rng: true, q_step: true },
        { ...emptyFixedRow, q_rng: true, q_step: true, q_alu: true }
      ]
      val trailFixed: List[FixedRow] = [emptyFixedRow, emptyFixedRow, emptyFixedRow]
      val fixedRows = preFixed.concat(op0Fixed).concat(op1Fixed).concat(trailFixed)
      val ops: List[OpSpec] = [
        { opId: 0, padRow: 2, limbRows: [3, 4, 5], gateRow: 5 },
        { opId: 1, padRow: -1, limbRows: [6, 7, 8], gateRow: 8 }
      ]
      { fixedRows: fixedRows, ops: ops }
    } else {
      val op0Fixed = buildStandardOpFixed(mt)
      val op1Fixed = buildStandardOpFixed(mt)
      val trailFixed: List[FixedRow] = [emptyFixedRow, emptyFixedRow]
      val fixedRows = preFixed.concat(op0Fixed).concat(op1Fixed).concat(trailFixed)
      val g0 = if (mt == "gate_off_centre") 2 else if (mt == "gate_row10") 4 else 5
      val g1 = if (mt == "gate_off_centre") 6 else if (mt == "gate_row10") 8 else 9
      val ops: List[OpSpec] = [
        { opId: 0, padRow: 2, limbRows: [3, 4, 5], gateRow: g0 },
        { opId: 1, padRow: 6, limbRows: [7, 8, 9], gateRow: g1 }
      ]
      { fixedRows: fixedRows, ops: ops }
    }
  }

  // ---------------------------------------------------------------- Circuit Gates & Lookups
  // `range run` gate on row rIdx (`range.rs` lines 105-118):
  //   - q_first * (acc - limb) == 0 (mod P)
  //   - q_first * (pad - limb * PAD_SHIFT) == 0 (mod P)  [dropped in R3 `pad_unchecked`]
  //   - q_step  * (acc - acc_prev * BASE - limb) == 0 (mod P)
  pure def rowRangeGateOkAtPrime(pVal: int, mt: str, lay: Layout, rows: List[AdviceRow], rIdx: int): bool = {
    val fx = lay.fixedRows[rIdx]
    val cur = rows[rIdx]
    val prev = if (rIdx > 0) rows[rIdx - 1] else ZERO_ADV

    val firstAccOk =
      if (fx.q_first) (modPrime(pVal, cur.acc - cur.limb) == 0) else true

    val firstPadOk =
      if (fx.q_first and mt != "pad_unchecked")
        (modPrime(pVal, prev.limb - cur.limb * PAD_SHIFT) == 0)
      else true

    val stepOk =
      if (fx.q_step)
        (modPrime(pVal, cur.acc - prev.acc * BASE - cur.limb) == 0)
      else true

    firstAccOk and firstPadOk and stepOk
  }

  pure def rowRangeGateOk(mt: str, lay: Layout, rows: List[AdviceRow], rIdx: int): bool =
    rowRangeGateOkAtPrime(P, mt, lay, rows, rIdx)

  // `range` lookup on row rIdx (`range.rs` lines 98-102):
  //   (q_rng * limb) in TABLE (0..2^B - 1). When q_rng == false, 0 * limb == 0 in TABLE.
  pure def rowLookupOk(lay: Layout, rows: List[AdviceRow], rIdx: int): bool = {
    val fx = lay.fixedRows[rIdx]
    if (fx.q_rng) TABLE.contains(rows[rIdx].limb) else TABLE.contains(0)
  }

  // Field element bounds: every advice cell is a canonical residue in 0..pVal - 1
  pure def fieldBoundsOkAtPrime(pVal: int, w: Witness): bool = and {
    w.rows.length() == TOTAL_ROWS,
    w.opW.length() == NUM_OPS,
    range(0, TOTAL_ROWS).foldl(true, (ok, rIdx) =>
      ok and 0 <= w.rows[rIdx].limb and w.rows[rIdx].limb < pVal
         and 0 <= w.rows[rIdx].acc and w.rows[rIdx].acc < pVal
    ),
    range(0, NUM_OPS).foldl(true, (ok, k) =>
      ok and 0 <= w.opW[k] and w.opW[k] < pVal
    )
  }

  pure def fieldBoundsOk(w: Witness): bool =
    fieldBoundsOkAtPrime(P, w)

  // ALU `w = acc` constraint on each op's gate row (`alu.rs` line 124)
  pure def aluWEqualsAccOkAtPrime(pVal: int, lay: Layout, w: Witness): bool =
    lay.ops.indices().forall(k => {
      val opSpec = lay.ops[k]
      val gRow = opSpec.gateRow
      lay.fixedRows[gRow].q_alu and modPrime(pVal, w.opW[k] - w.rows[gRow].acc) == 0
    })

  pure def aluWEqualsAccOk(lay: Layout, w: Witness): bool =
    aluWEqualsAccOkAtPrime(P, lay, w)

  // Complete acceptance predicate: every `range run` gate, every lookup, and the ALU's `w = acc` hold
  pure def acceptsAtPrime(pVal: int, mt: str, w: Witness): bool = {
    val lay = buildLayout(mt)
    and {
      fieldBoundsOkAtPrime(pVal, w),
      range(0, TOTAL_ROWS).foldl(true, (ok, rIdx) =>
        ok and rowLookupOk(lay, w.rows, rIdx) and rowRangeGateOkAtPrime(pVal, mt, lay, w.rows, rIdx)
      ),
      aluWEqualsAccOkAtPrime(pVal, lay, w)
    }
  }

  pure def acceptsWithMutant(mt: str, w: Witness): bool =
    acceptsAtPrime(P, mt, w)

  pure def accepts(w: Witness): bool =
    acceptsAtPrime(P, MUTANT, w)

  // ---------------------------------------------------------------- Reference Semantics & `invRefine`
  // Base-2^B integer value of an op's own limbs (from top limb down to bottom limb, over the integers)
  pure def opLimbBaseVal(opSpec: OpSpec, rows: List[AdviceRow]): int =
    opSpec.limbRows.foldl(0, (sum, rIdx) => sum * BASE + rows[rIdx].limb)

  // Reference property for one op:
  //   1. The value `w` its gate sees is `< 2^(B * (L - 1) + T)` (`0 <= w < RANGE_BOUND`), AND
  //   2. Every limb of the op is a base-2^B digit (`0 <= limb < 2^B`) and `w` equals the base-2^B number of its own limbs.
  pure def opRefines(opSpec: OpSpec, w: Witness): bool = {
    val gateVal = w.opW[opSpec.opId]
    val limbVal = opLimbBaseVal(opSpec, w.rows)
    val limbsValid = opSpec.limbRows.indices().forall(i => {
      val rIdx = opSpec.limbRows[i]
      0 <= w.rows[rIdx].limb and w.rows[rIdx].limb < BASE
    })
    and {
      0 <= gateVal,
      gateVal < RANGE_BOUND,
      limbsValid,
      gateVal == limbVal
    }
  }

  pure def refinesWith(mt: str, w: Witness): bool = {
    val lay = buildLayout(mt)
    lay.ops.indices().forall(k => opRefines(lay.ops[k], w))
  }

  pure def refines(w: Witness): bool =
    refinesWith(MUTANT, w)

  // ---------------------------------------------------------------- Q2c: ALU Arithmetic Gate Model (`alu.rs` 92-127)
  // Full ALU gate on a centre row with decomposed range witness `w`:
  //   - chip == CHIP_ALU
  //   - key == op + 8 * flag (mod P)
  //   - (op - OP_ADD) * (op - OP_LT) == 0 (mod P)
  //   - flag * (1 - flag) == 0 (mod P)
  //   - is_add * (x + y - z - flag * RANGE_BOUND) == 0 (mod P)
  //   - is_lt * flag == 0 (mod P)
  //   - is_lt * z * (1 - z) == 0 (mod P)
  //   - w == is_add * z + is_lt * (z * (y - x - 1) + (1 - z) * (x - y)) (mod P)
  pure def aluGateOkAtPrime(pVal: int, t: AluTupleW): bool = {
    val is_add = 2 - t.op
    val is_lt = t.op - 1
    val expectedW =
      is_add * t.z + is_lt * (t.z * (t.y - t.x - 1) + (1 - t.z) * (t.x - t.y))
    and {
      0 <= t.x and t.x < pVal,
      0 <= t.y and t.y < pVal,
      0 <= t.z and t.z < pVal,
      0 <= t.w and t.w < pVal,
      modPrime(pVal, t.chip - CHIP_ALU) == 0,
      modPrime(pVal, t.key - t.op - t.flag * FLAG_SHIFT) == 0,
      modPrime(pVal, (t.op - OP_ADD) * (t.op - OP_LT)) == 0,
      modPrime(pVal, t.flag * (1 - t.flag)) == 0,
      modPrime(pVal, is_add * (t.x + t.y - t.z - t.flag * RANGE_BOUND)) == 0,
      modPrime(pVal, is_lt * t.flag) == 0,
      modPrime(pVal, is_lt * t.z * (1 - t.z)) == 0,
      modPrime(pVal, t.w - expectedW) == 0
    }
  }

  pure def aluGateOk(t: AluTupleW): bool =
    aluGateOkAtPrime(P, t)

  // Reference ALU evaluation for operands x, y < RANGE_BOUND (`AluOp::eval`, `alu.rs` 39-60)
  pure def aluRefEval(op: int, x: int, y: int): AluTupleW = {
    if (op == OP_ADD) {
      val sum = x + y
      val flag = if (sum >= RANGE_BOUND) 1 else 0
      val z = sum - flag * RANGE_BOUND
      {
        chip: CHIP_ALU,
        key: OP_ADD + FLAG_SHIFT * flag,
        x: x,
        y: y,
        z: z,
        op: OP_ADD,
        flag: flag,
        w: z
      }
    } else {
      val z = if (x < y) 1 else 0
      val w = if (z == 1) (y - x - 1) else (x - y)
      {
        chip: CHIP_ALU,
        key: OP_LT,
        x: x,
        y: y,
        z: z,
        op: OP_LT,
        flag: 0,
        w: w
      }
    }
  }

  // ---------------------------------------------------------------- Honest Witness Builder
  // Decompose `v < RANGE_BOUND` into [limb2, limb1, limb0] in base 4 (`range.rs:59-66`)
  pure def decompose3(v: int): List[int] = {
    val l0 = v % BASE
    val l1 = (v / BASE) % BASE
    val l2 = (v / (BASE * BASE)) % BASE
    [l2, l1, l0]
  }

  // Build one op's advice rows for an honest value `v < RANGE_BOUND` given the preceding row's `acc`
  pure def buildHonestOpRows(mt: str, v: int, prevAcc: int, isOp1: bool): List[AdviceRow] = {
    val d = decompose3(v)
    val lTop = d[0]
    val lMid = d[1]
    val lBot = d[2]
    val pad = modP(PAD_SHIFT * lTop)
    if (mt == "extra_step") {
      // 4 limbs: [0, lTop, lMid, lBot], pad = 0
      val a0 = 0
      val a1 = 0
      val a2 = modP(lTop)
      val a3 = modP(BASE * a2 + lMid)
      val a4 = modP(BASE * a3 + lBot)
      [
        { limb: 0, acc: a0 },
        { limb: 0, acc: a1 },
        { limb: lTop, acc: a2 },
        { limb: lMid, acc: a3 },
        { limb: lBot, acc: a4 }
      ]
    } else if (mt == "shared_sum" and isOp1) {
      // 3 rows with no pad row, starting from prevAcc
      val a1 = modP(BASE * prevAcc + lTop)
      val a2 = modP(BASE * a1 + lMid)
      val a3 = modP(BASE * a2 + lBot)
      [
        { limb: lTop, acc: a1 },
        { limb: lMid, acc: a2 },
        { limb: lBot, acc: a3 }
      ]
    } else {
      val a0 =
        if (mt == "step_on_pad_row") modP(BASE * prevAcc + pad)
        else if (mt == "gate_off_centre") modP(v)
        else 0
      val a1 =
        if (mt == "no_first") modP(BASE * a0 + lTop)
        else modP(lTop)
      val a2 = modP(BASE * a1 + lMid)
      val a3 = modP(BASE * a2 + lBot)
      [
        { limb: pad, acc: a0 },
        { limb: lTop, acc: a1 },
        { limb: lMid, acc: a2 },
        { limb: lBot, acc: a3 }
      ]
    }
  }

  // Build the full 12-row honest witness for two ALU ops with decomposed values (v0, v1)
  pure def buildHonestWitnessForPair(mt: str, v0: int, v1: int): Witness = {
    val lay = buildLayout(mt)
    // Preceding core region rows 0..1 (row 1 has non-zero limb=3, acc=7 from the core region)
    val preRows: List[AdviceRow] = [
      { limb: 0, acc: 0 },
      { limb: 3, acc: 7 }
    ]
    val op0Rows = buildHonestOpRows(mt, v0, preRows[1].acc, false)
    val op0LastAcc = op0Rows[op0Rows.length() - 1].acc
    val op1Rows = buildHonestOpRows(mt, v1, op0LastAcc, true)
    val usedRows = preRows.concat(op0Rows).concat(op1Rows)
    val remCount = TOTAL_ROWS - usedRows.length()
    val allRows = range(0, remCount).foldl(usedRows, (acc, _i) => acc.append(ZERO_ADV))
    val w0 = allRows[lay.ops[0].gateRow].acc
    val w1 = allRows[lay.ops[1].gateRow].acc
    { rows: allRows, opW: [w0, w1] }
  }

  // Default honest witness: Op 0 has w0 = 0 (e.g. LT(2, 3) -> z = 1, w = 0), Op 1 has w1 = 21 (e.g. ADD(10, 11) -> w = 21)
  pure def buildHonestWitness(mt: str): Witness =
    buildHonestWitnessForPair(mt, 0, 21)

  // Honest witness exactly as `assign_running` writes row 0 (`acc_0 = 0` unconditionally)
  pure def buildAssignRunningLiteralWitness(v0: int, v1: int): Witness =
    buildHonestWitnessForPair("none", v0, v1)

  // ---------------------------------------------------------------- Free-Advice Prover Builder
  // Build a witness where the prover freely chooses every advice cell of targetOp (`0` or `1`):
  //   - `pad`: pad cell on row 0 of targetOp
  //   - `l1, l2, l3` (and `l4 = 0` in R5): limb cells of targetOp
  //   - `accMode` and `freeAcc`:
  //       0 => `acc_0 = freeAcc` (or `modP(BASE * prevAcc + pad)` in `step_on_pad_row`),
  //            `acc_1` follows row 1's active gate (`modP(BASE * acc_0 + l1)` in R1, `modP(BASE * prevAcc + l1)` in R7 Op 1, else `modP(l1)`),
  //            `acc_t = modP(BASE * acc_{t-1} + l_t)` for t >= 2, `w = acc[gateRow]`.
  //       1 => `acc_0 = 0`, `acc_1 = modP(freeAcc)` chosen freely (tests R2 `first_dropped` and tests that `q_first` rejects bad `acc_1`),
  //            `acc_t = modP(BASE * acc_{t-1} + l_t)` for t >= 2, `w = acc[gateRow]`.
  //       2 => `acc_2` (row t = 2) is overwritten with `modP(freeAcc)` without propagating (tests `q_step` rejection), `w = acc[gateRow]`.
  //       3 => `w` on `gateRow` is overwritten with `modP(freeAcc)` while `acc` is unchanged (tests `w == acc` rejection).
  pure def buildCustomOpRowsAtPrime(
    pVal: int,
    mt: str,
    prevAcc: int,
    isOp1: bool,
    padVal: int,
    l1Val: int,
    l2Val: int,
    l3Val: int,
    accMode: int,
    freeAcc: int
  ): List[AdviceRow] = {
    val pClean = modPrime(pVal, padVal)
    val x1 = modPrime(pVal, l1Val)
    val x2 = modPrime(pVal, l2Val)
    val x3 = modPrime(pVal, l3Val)
    val fAcc = modPrime(pVal, freeAcc)
    if (mt == "extra_step") {
      val a0 = if (accMode == 0) fAcc else 0
      val a1 = if (accMode == 1) fAcc else x1
      val a2 = if (accMode == 2) fAcc else modPrime(pVal, BASE * a1 + x2)
      val a3 = modPrime(pVal, BASE * a2 + x3)
      val a4 = modPrime(pVal, BASE * a3 + 0)
      [
        { limb: pClean, acc: a0 },
        { limb: x1, acc: a1 },
        { limb: x2, acc: a2 },
        { limb: x3, acc: a3 },
        { limb: 0, acc: a4 }
      ]
    } else if (mt == "shared_sum" and isOp1) {
      val a1 = if (accMode == 1) fAcc else modPrime(pVal, BASE * prevAcc + x1)
      val a2 = if (accMode == 2) fAcc else modPrime(pVal, BASE * a1 + x2)
      val a3 = modPrime(pVal, BASE * a2 + x3)
      [
        { limb: x1, acc: a1 },
        { limb: x2, acc: a2 },
        { limb: x3, acc: a3 }
      ]
    } else {
      val a0 =
        if (mt == "step_on_pad_row") modPrime(pVal, BASE * prevAcc + pClean)
        else if (accMode == 0) fAcc
        else 0
      val a1 =
        if (accMode == 1) fAcc
        else if (mt == "no_first") modPrime(pVal, BASE * a0 + x1)
        else x1
      val a2 =
        if (accMode == 2) fAcc
        else modPrime(pVal, BASE * a1 + x2)
      val a3 = modPrime(pVal, BASE * a2 + x3)
      [
        { limb: pClean, acc: a0 },
        { limb: x1, acc: a1 },
        { limb: x2, acc: a2 },
        { limb: x3, acc: a3 }
      ]
    }
  }

  pure def buildFreeOpWitnessAtPrime(
    pVal: int,
    mt: str,
    targetOp: int,
    otherVal: int,
    padVal: int,
    l1Val: int,
    l2Val: int,
    l3Val: int,
    accMode: int,
    freeAcc: int
  ): Witness = {
    val lay = buildLayout(mt)
    val preRows: List[AdviceRow] = [
      { limb: 0, acc: 0 },
      { limb: 3, acc: modPrime(pVal, 7) }
    ]
    val fAcc = modPrime(pVal, freeAcc)
    val op0Rows =
      if (targetOp == 0)
        buildCustomOpRowsAtPrime(pVal, mt, preRows[1].acc, false, padVal, l1Val, l2Val, l3Val, accMode, freeAcc)
      else
        buildHonestOpRows(mt, otherVal, preRows[1].acc, false)
    val op0LastAcc = op0Rows[op0Rows.length() - 1].acc
    val op1Rows =
      if (targetOp == 1)
        buildCustomOpRowsAtPrime(pVal, mt, op0LastAcc, true, padVal, l1Val, l2Val, l3Val, accMode, freeAcc)
      else
        buildHonestOpRows(mt, otherVal, op0LastAcc, true)

    val usedRows = preRows.concat(op0Rows).concat(op1Rows)
    val remCount = TOTAL_ROWS - usedRows.length()
    val allRows = range(0, remCount).foldl(usedRows, (acc, _i) => acc.append(ZERO_ADV))
    val w0Honest = allRows[lay.ops[0].gateRow].acc
    val w1Honest = allRows[lay.ops[1].gateRow].acc
    val w0 = if (targetOp == 0 and accMode == 3) fAcc else w0Honest
    val w1 = if (targetOp == 1 and accMode == 3) fAcc else w1Honest
    { rows: allRows, opW: [w0, w1] }
  }

  pure def buildFreeOpWitness(
    mt: str,
    targetOp: int,
    otherVal: int,
    padVal: int,
    l1Val: int,
    l2Val: int,
    l3Val: int,
    accMode: int,
    freeAcc: int
  ): Witness =
    buildFreeOpWitnessAtPrime(P, mt, targetOp, otherVal, padVal, l1Val, l2Val, l3Val, accMode, freeAcc)

  // Build a witness where all 8 advice cells of Op 0 (pad, l1, l2, l3, acc0, acc1, acc2, acc3)
  // are assigned completely independently with NO recurrence helper
  pure def buildRawIndependentOp0Witness(
    mt: str,
    padVal: int,
    l1Val: int,
    l2Val: int,
    l3Val: int,
    a0Val: int,
    a1Val: int,
    a2Val: int,
    a3Val: int
  ): Witness = {
    val lay = buildLayout(mt)
    val base = buildHonestWitness(mt)
    val rawRows = base.rows
      .replaceAt(2, { limb: modP(padVal), acc: modP(a0Val) })
      .replaceAt(3, { limb: modP(l1Val), acc: modP(a1Val) })
      .replaceAt(4, { limb: modP(l2Val), acc: modP(a2Val) })
      .replaceAt(5, { limb: modP(l3Val), acc: modP(a3Val) })
    val w0 = rawRows[lay.ops[0].gateRow].acc
    val w1 = rawRows[lay.ops[1].gateRow].acc
    { rows: rawRows, opW: [w0, w1] }
  }

  // Canonical attack selector by attackId (1..9)
  pure def applyProverAttack(mt: str, attackId: int): Witness = {
    if (attackId == 1)
      // R1 `no_first`: free acc_0 = 1 on row 2 shifts by 4^3 = 64 -> w0 = 64 >= 32 (free accumulator)
      buildFreeOpWitness(mt, 0, 21, 0, 0, 0, 0, 0, 1)
    else if (attackId == 2)
      // R2 `first_dropped`: free acc_1 = 2 on row 3 shifts by 4^2 = 16 -> w0 = 32 >= 32 (free accumulator)
      buildFreeOpWitness(mt, 0, 21, 0, 0, 0, 0, 1, 2)
    else if (attackId == 3)
      // R3 `pad_unchecked`: pad = 0, top limb l1 = 2 (in table 0..3, >= 2^T) -> w0 = 32 >= 32 (missing table check on top limb)
      buildFreeOpWitness(mt, 0, 21, 0, 2, 0, 0, 0, 0)
    else if (attackId == 4)
      // R4 `pad_row_no_lookup`: pad = 4 (outside table 0..3), top limb l1 = 2 -> w0 = 32 >= 32 (missing table check on pad row)
      buildFreeOpWitness(mt, 0, 21, 4, 2, 0, 0, 0, 0)
    else if (attackId == 5)
      // R5 `extra_step`: 4 limbs with top limb l1 = 1, pad = 2 -> w0 = 64 >= 32 (extra step / 4th limb)
      buildFreeOpWitness(mt, 0, 21, 2, 1, 0, 0, 0, 0)
    else if (attackId == 6)
      // R6 `gate_off_centre` (row 0): free acc_0 = 32 on row 2, limbs [0, 0, 0] -> w0 = 32 >= 32 (free accumulator)
      buildFreeOpWitness(mt, 0, 21, 0, 0, 0, 0, 0, 32)
    else if (attackId == 7)
      // R7 `shared_sum`: Op 0 has value 1 (acc_5 = 1), Op 1 has limbs [0, 0, 0] -> Op 1 carries acc_5 into w1 = 64 >= 32
      buildFreeOpWitness(mt, 1, 1, 0, 0, 0, 0, 0, 0)
    else if (attackId == 8)
      // R6b `gate_row10`: Op 0 has limbs [0, 0, 1] (base-4 value 1), gate on row L-1 (row 4) sees acc_2 = 0 != 1
      buildFreeOpWitness(mt, 0, 0, 0, 0, 0, 1, 0, 0)
    else
      // Small-prime wrap-around (P = 23): limbs [1, 2, 0] (base-4 value 24) wrap mod 23 to w0 = 1 != 24
      buildFreeOpWitness(mt, 0, 21, 2, 1, 2, 0, 0, 0)
  }

  // ---------------------------------------------------------------- Named Honest & Counterexample Witnesses
  pure val hw_honest: Witness = buildHonestWitness("none")
  pure val hw_honest_max: Witness = buildHonestWitnessForPair("none", 19, 31)
  pure val hw_honest_zero_both: Witness = buildHonestWitnessForPair("none", 0, 0)

  pure val cex_R1_no_first: Witness = applyProverAttack("no_first", 1)
  pure val cex_R2_first_dropped: Witness = applyProverAttack("first_dropped", 2)
  pure val cex_R3_pad_unchecked: Witness = applyProverAttack("pad_unchecked", 3)
  pure val cex_R4_pad_row_no_lookup: Witness = applyProverAttack("pad_row_no_lookup", 4)
  pure val cex_R5_extra_step: Witness = applyProverAttack("extra_step", 5)
  pure val cex_R6_gate_off_centre: Witness = applyProverAttack("gate_off_centre", 6)
  pure val cex_R6_row10: Witness = applyProverAttack("gate_row10", 8)
  pure val cex_R7_shared_sum: Witness = applyProverAttack("shared_sum", 7)
  pure val cex_wrap_sum: Witness = applyProverAttack("none", 9)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val SAMPLE_MODES: Set[int] = Set(0, 1, 2)
  pure val HONEST_IDS: Set[int] = Set(0, 1, 2)
  pure val ATTACK_IDS: Set[int] = 1.to(9)
  pure val TARGET_OPS: Set[int] = Set(0, 1)
  pure val OTHER_VALS: Set[int] = Set(0, 1, 21)
  // Domains include in-table values (0..3), out-of-table values (4 = 2^B, 32 = RANGE_BOUND), and P - 1 (near prime p)
  pure val PAD_DOM: Set[int] = Set(0, 1, 2, 4, P - 1)
  pure val L1_DOM: Set[int] = Set(0, 1, 2, 3, 4, P - 1)
  pure val L2_DOM: Set[int] = Set(0, 2, 4)
  pure val L3_DOM: Set[int] = Set(0, 1, 3, 4)
  pure val ACC_MODES: Set[int] = Set(0, 1, 2, 3)
  pure val FREE_ACC_DOM: Set[int] = Set(0, 1, 2, 32, P - 1)

  var w: Witness
  var acc: bool
  var isTampered: bool

  pure def isSoundHonest(cand: Witness): bool =
    cand == hw_honest or cand == hw_honest_max or cand == hw_honest_zero_both

  action init = {
    val initW = buildHonestWitness(MUTANT)
    all {
      w' = initW,
      acc' = accepts(initW),
      isTampered' = not(isSoundHonest(initW))
    }
  }

  action step = {
    nondet mode = SAMPLE_MODES.oneOf()
    nondet hId = HONEST_IDS.oneOf()
    nondet attackId = ATTACK_IDS.oneOf()
    nondet targetOp = TARGET_OPS.oneOf()
    nondet otherVal = OTHER_VALS.oneOf()
    nondet padVal = PAD_DOM.oneOf()
    nondet l1Val = L1_DOM.oneOf()
    nondet l2Val = L2_DOM.oneOf()
    nondet l3Val = L3_DOM.oneOf()
    nondet accMode = ACC_MODES.oneOf()
    nondet freeAcc = FREE_ACC_DOM.oneOf()

    val cand =
      if (mode == 0) {
        if (hId == 0) buildHonestWitness(MUTANT)
        else if (hId == 1) buildHonestWitnessForPair(MUTANT, 19, 31)
        else buildHonestWitnessForPair(MUTANT, 0, 0)
      } else if (mode == 1) {
        applyProverAttack(MUTANT, attackId)
      } else {
        buildFreeOpWitness(MUTANT, targetOp, otherVal, padVal, l1Val, l2Val, l3Val, accMode, freeAcc)
      }

    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = not(isSoundHonest(cand))
    }
  }

  // ---------------------------------------------------------------- Invariants
  // Main refinement invariant: accepted implies each op's gate value is < 2^(B*(L-1)+T) and equals the base-2^B value of its own limbs
  val invRefine: bool =
    acc implies refines(w)

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 in `range_sound`):
  // 1. An honest accepted witness with two consecutive ops where Op 0 = 0 and Op 1 = 21
  val reachHonestTwoOps: bool =
    not(acc and not(isTampered) and w.opW[0] == 0 and w.opW[1] == 21)

  // 2. An honest accepted witness reaching the maximum in-range value `RANGE_BOUND - 1 = 31` (`[1, 3, 3]`, pad = 2)
  val reachMaxRangeValue: bool =
    not(acc and not(isTampered) and w.opW[0] == 19 and w.opW[1] == RANGE_BOUND - 1)

  // 3. An accepted witness that does not refine (must NOT be violated in `range_sound` or `range_step_on_pad_row`: exit 0;
  //    violated in R1..R7 and `range_small_p`: exit 1)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// ---------------------------------------------------------------- Module Instances
// Sound baseline instance (P = 97 > 2 * RANGE_BOUND)
module range_sound { import rangechip(P = 97, MUTANT = "none").* }

// Required mutant instances R1..R8 (both `range_<name>` and `range_R<k>_<name>` aliases provided)
module range_no_first { import rangechip(P = 97, MUTANT = "no_first").* }
module range_first_dropped { import rangechip(P = 97, MUTANT = "first_dropped").* }
module range_pad_unchecked { import rangechip(P = 97, MUTANT = "pad_unchecked").* }
module range_pad_row_no_lookup { import rangechip(P = 97, MUTANT = "pad_row_no_lookup").* }
module range_extra_step { import rangechip(P = 97, MUTANT = "extra_step").* }
module range_gate_off_centre { import rangechip(P = 97, MUTANT = "gate_off_centre").* }
module range_gate_row10 { import rangechip(P = 97, MUTANT = "gate_row10").* }
module range_shared_sum { import rangechip(P = 97, MUTANT = "shared_sum").* }
module range_step_on_pad_row { import rangechip(P = 97, MUTANT = "step_on_pad_row").* }

module range_R1_no_first { import rangechip(P = 97, MUTANT = "no_first").* }
module range_R2_first_dropped { import rangechip(P = 97, MUTANT = "first_dropped").* }
module range_R3_pad_unchecked { import rangechip(P = 97, MUTANT = "pad_unchecked").* }
module range_R4_pad_row_no_lookup { import rangechip(P = 97, MUTANT = "pad_row_no_lookup").* }
module range_R5_extra_step { import rangechip(P = 97, MUTANT = "extra_step").* }
module range_R6_gate_off_centre { import rangechip(P = 97, MUTANT = "gate_off_centre").* }
module range_R6_gate_row10 { import rangechip(P = 97, MUTANT = "gate_row10").* }
module range_R7_shared_sum { import rangechip(P = 97, MUTANT = "shared_sum").* }
module range_R8_step_on_pad_row { import rangechip(P = 97, MUTANT = "step_on_pad_row").* }

// Small-prime instance (P = 23 < RANGE_BOUND = 32) showing modular wrap-around of the running sum
module range_small_p { import rangechip(P = 23, MUTANT = "none").* }
