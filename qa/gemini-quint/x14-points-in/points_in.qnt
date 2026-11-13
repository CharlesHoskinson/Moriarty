// Soundness & Completeness Review Model — X14 `PointsIn` and BLS12-381 G1 Chain (`lane-src/x_exp_node.rs`)
//
// Models:
//   - Small prime field F_P with P = 13 in place of the BLS12-381 base field p.
//     Curve E(F_13): y^2 = x^3 + 4 (mod 13), with valid affine points e.g.
//     P1 = (0, 2), P2 = (2, 5), P3 = (4, 4), P4 = (7, 3), P5 = (10, 9), P6 = (5, 5).
//   - Limbs over a small base: LIMB_BITS = 2 (BASE = 4, standing for 2^8 = 256),
//     NL = 2 limbs per value (little-endian [l0, l1] representing l0 + 4 * l1 in [0, 16)).
//     Note: 16 > P = 13, so values 13, 14, 15 have valid limbs in [0, 4) but are non-canonical (>= P).
//   - VariableRangeCheckerAir on RANGE_BUS = 1000:
//     Checks lookups (v, maxBits) with 1 <= maxBits <= MAX_RANGE_BITS (3) and 0 <= v < 2^maxBits.
//     maxBits = 2 (standing for 8 bits) bounds v in [0, 4); maxBits = 3 (standing for 9 bits) bounds v in [0, 8).
//   - PointsIn AIR (N_ROWS = 2 rows, r = 0, 1):
//     Columns: rowId, then nv = 4 values (x1, y1, x2, y2) of NL = 2 limbs each (for add)
//     or nv = 2 values (x, y) of NL = 2 limbs each (for dbl).
//     Constraints: first row rowId == 0; transition nextRowId == rowId + 1.
//     Interactions: range-checks every limb to LIMB_BITS = 2 on RANGE_BUS (mult 1);
//     sends (rowId, x1, y1) on bus 1100 and (rowId, x2, y2) on bus 1101 (mult 1).
//   - ChainAir rows (L_add, INV, X3, Y3 for add; SQ, L_dbl, INV_y, X3, Y3 for dbl) + SourceAir sink:
//     Each ChainAir has rowId == row index, boolean isValid, FieldExpr modular constraint mod P
//     (with output var limbs range-checked to LIMB_BITS = 2 when isValid == 1, inputs NOT range-checked
//     by FieldExpr), and PermutationCheckBus sends/receives weighted by isValid.
//   - Nondeterministic prover that may change any one cell of any row, or drop or duplicate
//     one permutation bus message or one range lookup.
//   - Required mutants: m_no_range, m_wrong_bits, m_no_send, m_dup_send, m_no_id_constraint.

module points_in_spec {
  const MUTANT: str

  // ---------------------------------------------------------------- Scaled Field & Limb Constants
  pure val P: int = 13
  pure val LIMB_BITS: int = 2
  pure val BASE: int = 4          // 2^LIMB_BITS = 4 (stands in for 2^8 = 256)
  pure val WRONG_BITS: int = 3    // LIMB_BITS + 1 = 3 (stands in for 9 bits in m_wrong_bits)
  pure val WRONG_BASE: int = 8    // 2^WRONG_BITS = 8 (stands in for 2^9 = 512)
  pure val NL: int = 2            // 2 limbs per field element (proper_max = 4^2 - 1 = 15 >= P = 13)
  pure val N_ROWS: int = 2        // 2 rows per AIR (r = 0, 1)

  // Bus indices matching lane-src/x_exp_node.rs
  pure val RANGE_BUS: int = 1000
  pure val O_ADD: int = 1100
  pure val O_DBL: int = 1110

  pure val ROW_LIST: List[int] = [0, 1]

  // ---------------------------------------------------------------- Modular Arithmetic over F_13 & Limbs
  pure def modP(x: int): int = ((x % P) + P) % P

  // Multiplicative inverse modulo P = 13 (returns -1 if a == 0 mod 13, i.e. no inverse exists)
  pure def invModP(a: int): int = {
    val r = modP(a)
    if (r == 1) 1
    else if (r == 2) 7
    else if (r == 3) 9
    else if (r == 4) 10
    else if (r == 5) 8
    else if (r == 6) 11
    else if (r == 7) 2
    else if (r == 8) 5
    else if (r == 9) 3
    else if (r == 10) 4
    else if (r == 11) 6
    else if (r == 12) 12
    else -1
  }

  // Little-endian 2-limb value: limbs = [l0, l1] -> l0 + 4 * l1
  pure def limbsToInt(ls: List[int]): int =
    ls[0] + BASE * ls[1]

  pure def limbsModP(ls: List[int]): int =
    modP(limbsToInt(ls))

  pure def intToLimbs(v: int): List[int] =
    [v % BASE, (v / BASE) % BASE]

  pure def limbsInRange(ls: List[int], bound: int): bool = and {
    ls.length() == NL,
    0 <= ls[0] and ls[0] < bound,
    0 <= ls[1] and ls[1] < bound
  }

  // Check if a point (x, y) mod 13 lies on E(F_13): y^2 = x^3 + 4 (mod 13)
  pure def isOnCurveModP(x: int, y: int): bool =
    modP(y * y) == modP(x * x * x + 4)

  // Reference affine addition on (x1, y1), (x2, y2) over F_13 (requires x1 != x2 mod 13)
  pure def refAddModP(x1: int, y1: int, x2: int, y2: int): { ok: bool, lam: int, w: int, x3: int, y3: int } = {
    val dx = modP(x2 - x1)
    val w = invModP(dx)
    if (w < 0) {
      { ok: false, lam: 0, w: 0, x3: 0, y3: 0 }
    } else {
      val lam = modP(modP(y2 - y1) * w)
      val x3 = modP(lam * lam - x1 - x2)
      val y3 = modP(lam * (x1 - x3) - y1)
      { ok: true, lam: lam, w: w, x3: x3, y3: y3 }
    }
  }

  // Reference affine doubling on (x, y) over F_13 (requires y != 0 mod 13)
  pure def refDblModP(x: int, y: int): { ok: bool, s: int, lam: int, w: int, x3: int, y3: int } = {
    val s = modP(x * x)
    val w = invModP(y)
    val inv2y = invModP(2 * y)
    if (w < 0 or inv2y < 0) {
      { ok: false, s: s, lam: 0, w: 0, x3: 0, y3: 0 }
    } else {
      val lam = modP(3 * s * inv2y)
      val x3 = modP(lam * lam - 2 * x)
      val y3 = modP(lam * (x - x3) - y)
      { ok: true, s: s, lam: lam, w: w, x3: x3, y3: y3 }
    }
  }

  // ---------------------------------------------------------------- Row & Trace Types (Add Chain)
  type PointsInRow = {
    rowId: int,
    x1: List[int],
    y1: List[int],
    x2: List[int],
    y2: List[int]
  }

  type LAddRow = {
    isValid: int,
    rowId: int,
    x1: List[int],
    y1: List[int],
    x2: List[int],
    y2: List[int],
    lam: List[int]
  }

  type InvRow = {
    isValid: int,
    rowId: int,
    x1: List[int],
    x2: List[int],
    w: List[int]
  }

  type X3Row = {
    isValid: int,
    rowId: int,
    lam: List[int],
    x1: List[int],
    x2: List[int],
    x3: List[int]
  }

  type Y3Row = {
    isValid: int,
    rowId: int,
    lam: List[int],
    x1: List[int],
    x3: List[int],
    y1: List[int],
    y3: List[int]
  }

  type SinkRow = {
    rowId: int,
    x3: List[int],
    y3: List[int]
  }

  // Prover tamper overrides for dropping or duplicating a single bus message or range lookup:
  //   - msgTamperBus: bus index (0 if none, or 1100..1110)
  //   - msgTamperRow: row index (0 or 1)
  //   - msgTamperDelta: -1 (drop one message) or +1 (duplicate one message) or 0 (none)
  //   - rngTamperRow: row index (0 or 1) in PointsIn
  //   - rngTamperCoord: coordinate index (0..3 for x1, y1, x2, y2)
  //   - rngTamperLimb: limb index (0 or 1)
  //   - rngTamperDelta: -1 (drop one range lookup) or +1 (duplicate one range lookup) or 0 (none)
  type AddChainWitness = {
    pin: List[PointsInRow],
    lAdd: List[LAddRow],
    inv: List[InvRow],
    x3: List[X3Row],
    y3: List[Y3Row],
    sink: List[SinkRow],
    msgTamperBus: int,
    msgTamperRow: int,
    msgTamperDelta: int,
    rngTamperRow: int,
    rngTamperCoord: int,
    rngTamperLimb: int,
    rngTamperDelta: int
  }

  // ---------------------------------------------------------------- Doubling Chain Types
  type DblPointsInRow = {
    rowId: int,
    x: List[int],
    y: List[int]
  }

  type SqRow = {
    isValid: int,
    rowId: int,
    x: List[int],
    s: List[int]
  }

  type LDblRow = {
    isValid: int,
    rowId: int,
    s: List[int],
    x: List[int],
    y: List[int],
    lam: List[int]
  }

  type InvYRow = {
    isValid: int,
    rowId: int,
    y: List[int],
    w: List[int]
  }

  type DblChainWitness = {
    pin: List[DblPointsInRow],
    sq: List[SqRow],
    lDbl: List[LDblRow],
    invY: List[InvYRow],
    x3: List[X3Row],
    y3: List[Y3Row],
    sink: List[SinkRow]
  }

  // ---------------------------------------------------------------- AIR Constraint Predicates (Add Chain)
  // 1. `id = row_index` constraint:
  //    In `PointsIn` (lines 764-765) and `ChainAir` (lines 683-684):
  //      when_first_row().assert_zero(id);
  //      when_transition().assert_eq(next.id, local.id + 1);
  //    Mutant `no_id_constraint` drops the transition constraint on id.
  pure def idColOk(mt: str, id0: int, id1: int): bool =
    if (mt == "no_id_constraint")
      (id0 == 0)
    else
      (id0 == 0 and id1 == id0 + 1)

  // 2. Range lookup check on RANGE_BUS = 1000 against VariableRangeCheckerAir:
  //    A lookup (val, maxBits) with multiplicity `mult` is accepted iff:
  //      - mult == 1 (matching the honest range table count; dropping gives mult == 0, duplicating gives mult == 2), and
  //      - 0 <= val < 2^maxBits with 1 <= maxBits <= 3.
  //    In `PointsIn`:
  //      - Mutant `no_range`: drops the range lookup on row 0, coordinate 0 (x1), limb 0.
  //      - Mutant `wrong_bits`: looks up maxBits = 3 (bound 8) instead of maxBits = 2 (bound 4).
  pure def pinLimbRangeOk(mt: str, w: AddChainWitness, r: int, coordIdx: int, limbIdx: int, v: int): bool = {
    val droppedByMutant = (mt == "no_range" and r == 0 and coordIdx == 0 and limbIdx == 0)
    val bound = if (mt == "wrong_bits") WRONG_BASE else BASE
    val delta =
      if (w.rngTamperDelta != 0 and w.rngTamperRow == r and w.rngTamperCoord == coordIdx and w.rngTamperLimb == limbIdx)
        w.rngTamperDelta
      else 0
    if (droppedByMutant)
      (delta == 0)
    else
      (delta == 0 and 0 <= v and v < bound)
  }

  pure def pointsInAirOk(mt: str, w: AddChainWitness): bool = and {
    w.pin.length() == N_ROWS,
    idColOk(mt, w.pin[0].rowId, w.pin[1].rowId),
    ROW_LIST.foldl(true, (ok, r) => {
      val row = w.pin[r]
      and {
        ok,
        row.x1.length() == NL and row.y1.length() == NL and row.x2.length() == NL and row.y2.length() == NL,
        pinLimbRangeOk(mt, w, r, 0, 0, row.x1[0]),
        pinLimbRangeOk(mt, w, r, 0, 1, row.x1[1]),
        pinLimbRangeOk(mt, w, r, 1, 0, row.y1[0]),
        pinLimbRangeOk(mt, w, r, 1, 1, row.y1[1]),
        pinLimbRangeOk(mt, w, r, 2, 0, row.x2[0]),
        pinLimbRangeOk(mt, w, r, 2, 1, row.x2[1]),
        pinLimbRangeOk(mt, w, r, 3, 0, row.y2[0]),
        pinLimbRangeOk(mt, w, r, 3, 1, row.y2[1])
      }
    })
  }

  // 3. `FieldExpr` + `ChainAir` constraints:
  //    - `isValid` is boolean (0 or 1).
  //    - `id` satisfies `idColOk`.
  //    - When `isValid == 1`:
  //        * Every limb of the saved output variable `var` is range-checked to LIMB_BITS = 2 (`[0, BASE)`),
  //        * Input limbs are NOT range-checked by `FieldExpr`,
  //        * The modular constraint holds modulo P (assuming input limbs are in proper range so carries do not wrap).
  pure def lAddAirOk(mt: str, rows: List[LAddRow]): bool = and {
    rows.length() == N_ROWS,
    idColOk(mt, rows[0].rowId, rows[1].rowId),
    ROW_LIST.foldl(true, (ok, r) => {
      val row = rows[r]
      val validBool = (row.isValid == 0 or row.isValid == 1)
      val eqOk =
        if (row.isValid == 1) and {
          limbsInRange(row.lam, BASE),
          // (x2 - x1) * lam - (y2 - y1) == 0 (mod P)
          modP((limbsModP(row.x2) - limbsModP(row.x1)) * limbsModP(row.lam) - (limbsModP(row.y2) - limbsModP(row.y1))) == 0
        } else true
      ok and validBool and eqOk
    })
  }

  pure def invAirOk(mt: str, rows: List[InvRow]): bool = and {
    rows.length() == N_ROWS,
    idColOk(mt, rows[0].rowId, rows[1].rowId),
    ROW_LIST.foldl(true, (ok, r) => {
      val row = rows[r]
      val validBool = (row.isValid == 0 or row.isValid == 1)
      val eqOk =
        if (row.isValid == 1) and {
          limbsInRange(row.w, BASE),
          // (x2 - x1) * w - 1 == 0 (mod P) -> has NO solution w when x1 == x2 (mod P)
          modP((limbsModP(row.x2) - limbsModP(row.x1)) * limbsModP(row.w) - 1) == 0
        } else true
      ok and validBool and eqOk
    })
  }

  pure def x3AirOk(mt: str, rows: List[X3Row]): bool = and {
    rows.length() == N_ROWS,
    idColOk(mt, rows[0].rowId, rows[1].rowId),
    ROW_LIST.foldl(true, (ok, r) => {
      val row = rows[r]
      val validBool = (row.isValid == 0 or row.isValid == 1)
      val eqOk =
        if (row.isValid == 1) and {
          limbsInRange(row.x3, BASE),
          // lam^2 - x1 - x2 - x3 == 0 (mod P)
          modP(limbsModP(row.lam) * limbsModP(row.lam) - limbsModP(row.x1) - limbsModP(row.x2) - limbsModP(row.x3)) == 0
        } else true
      ok and validBool and eqOk
    })
  }

  pure def y3AirOk(mt: str, rows: List[Y3Row]): bool = and {
    rows.length() == N_ROWS,
    idColOk(mt, rows[0].rowId, rows[1].rowId),
    ROW_LIST.foldl(true, (ok, r) => {
      val row = rows[r]
      val validBool = (row.isValid == 0 or row.isValid == 1)
      val eqOk =
        if (row.isValid == 1) and {
          limbsInRange(row.y3, BASE),
          // lam * (x1 - x3) - y1 - y3 == 0 (mod P)
          modP(limbsModP(row.lam) * (limbsModP(row.x1) - limbsModP(row.x3)) - limbsModP(row.y1) - limbsModP(row.y3)) == 0
        } else true
      ok and validBool and eqOk
    })
  }

  // ---------------------------------------------------------------- PermutationCheckBus Multiset Balance
  // Models LogUp permutation check bus: a multiset of vectors `List[int]` with integer multiplicities.
  // A bus balances iff for every sent/received message vector `msg`, the sum of send multiplicities
  // across all rows equals the sum of receive multiplicities across all rows.
  type BusEntry = {
    msg: List[int],
    mult: int
  }

  pure def netCountForMsg(entries: List[BusEntry], target: List[int]): int =
    entries.foldl(0, (acc, e) => if (e.msg == target) acc + e.mult else acc)

  pure def entriesBalanced(entries: List[BusEntry]): bool =
    entries.foldl(true, (ok, e) => ok and netCountForMsg(entries, e.msg) == 0)

  pure def msgTamperMult(w: AddChainWitness, busId: int, r: int): int =
    if (w.msgTamperDelta != 0 and w.msgTamperBus == busId and w.msgTamperRow == r)
      w.msgTamperDelta
    else 0

  // Bus o = 1100: PointsIn sends (id, x1, y1) (mult 2 in `dup_send`, else 1); L_add receives (id, x1, y1) (mult isValid)
  pure def bus1100Balanced(mt: str, w: AddChainWitness): bool = {
    val pinBaseMult = if (mt == "dup_send") 2 else 1
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val p = w.pin[r]
      val l = w.lAdd[r]
      val sendMsg = [p.rowId].concat(p.x1).concat(p.y1)
      val recvMsg = [l.rowId].concat(l.x1).concat(l.y1)
      val sendMult = pinBaseMult + msgTamperMult(w, O_ADD, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -l.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+1 = 1101: PointsIn sends (id, x2, y2) (mult 0 in `no_send`, else 1); L_add receives (id, x2, y2) (mult isValid)
  pure def bus1101Balanced(mt: str, w: AddChainWitness): bool = {
    val pinBaseMult = if (mt == "no_send") 0 else 1
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val p = w.pin[r]
      val l = w.lAdd[r]
      val sendMsg = [p.rowId].concat(p.x2).concat(p.y2)
      val recvMsg = [l.rowId].concat(l.x2).concat(l.y2)
      val sendMult = pinBaseMult + msgTamperMult(w, O_ADD + 1, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -l.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+2 = 1102: L_add sends (id, lam, x1); X3 receives (id, lam, x1)
  pure def bus1102Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val l = w.lAdd[r]
      val x = w.x3[r]
      val sendMsg = [l.rowId].concat(l.lam).concat(l.x1)
      val recvMsg = [x.rowId].concat(x.lam).concat(x.x1)
      val sendMult = l.isValid + msgTamperMult(w, O_ADD + 2, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -x.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+3 = 1103: L_add sends (id, x2); X3 receives (id, x2)
  pure def bus1103Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val l = w.lAdd[r]
      val x = w.x3[r]
      val sendMsg = [l.rowId].concat(l.x2)
      val recvMsg = [x.rowId].concat(x.x2)
      val sendMult = l.isValid + msgTamperMult(w, O_ADD + 3, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -x.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+4 = 1104: L_add sends (id, lam, x1); Y3 receives (id, lam, x1)
  pure def bus1104Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val l = w.lAdd[r]
      val y = w.y3[r]
      val sendMsg = [l.rowId].concat(l.lam).concat(l.x1)
      val recvMsg = [y.rowId].concat(y.lam).concat(y.x1)
      val sendMult = l.isValid + msgTamperMult(w, O_ADD + 4, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -y.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+5 = 1105: L_add sends (id, y1); Y3 receives (id, y1)
  pure def bus1105Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val l = w.lAdd[r]
      val y = w.y3[r]
      val sendMsg = [l.rowId].concat(l.y1)
      val recvMsg = [y.rowId].concat(y.y1)
      val sendMult = l.isValid + msgTamperMult(w, O_ADD + 5, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -y.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+6 = 1106: X3 sends (id, x3); Y3 receives (id, x3)
  pure def bus1106Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val x = w.x3[r]
      val y = w.y3[r]
      val sendMsg = [x.rowId].concat(x.x3)
      val recvMsg = [y.rowId].concat(y.x3)
      val sendMult = x.isValid + msgTamperMult(w, O_ADD + 6, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -y.isValid })
    })
    entriesBalanced(entries)
  }

  // Bus o+7 = 1107: Y3 sends (id, x3, y3); Sink receives (id, x3, y3) (mult 1)
  pure def bus1107Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val y = w.y3[r]
      val s = w.sink[r]
      val sendMsg = [y.rowId].concat(y.x3).concat(y.y3)
      val recvMsg = [s.rowId].concat(s.x3).concat(s.y3)
      val sendMult = y.isValid + msgTamperMult(w, O_ADD + 7, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -1 })
    })
    entriesBalanced(entries)
  }

  // Bus o+10 = 1110: L_add sends (id, x1, x2); INV receives (id, x1, x2)
  pure def bus1110Balanced(w: AddChainWitness): bool = {
    val empty: List[BusEntry] = []
    val entries = ROW_LIST.foldl(empty, (acc, r) => {
      val l = w.lAdd[r]
      val iv = w.inv[r]
      val sendMsg = [l.rowId].concat(l.x1).concat(l.x2)
      val recvMsg = [iv.rowId].concat(iv.x1).concat(iv.x2)
      val sendMult = l.isValid + msgTamperMult(w, O_ADD + 10, r)
      acc.append({ msg: sendMsg, mult: sendMult }).append({ msg: recvMsg, mult: -iv.isValid })
    })
    entriesBalanced(entries)
  }

  pure def allAddBusesBalanced(mt: str, w: AddChainWitness): bool = and {
    bus1100Balanced(mt, w),
    bus1101Balanced(mt, w),
    bus1102Balanced(w),
    bus1103Balanced(w),
    bus1104Balanced(w),
    bus1105Balanced(w),
    bus1106Balanced(w),
    bus1107Balanced(w),
    bus1110Balanced(w)
  }

  // Full acceptance predicate for the Add Chain
  pure def acceptsWithMutant(mt: str, w: AddChainWitness): bool = and {
    pointsInAirOk(mt, w),
    lAddAirOk(mt, w.lAdd),
    invAirOk(mt, w.inv),
    x3AirOk(mt, w.x3),
    y3AirOk(mt, w.y3),
    w.sink.length() == N_ROWS,
    allAddBusesBalanced(mt, w)
  }

  pure def accepts(w: AddChainWitness): bool =
    acceptsWithMutant(MUTANT, w)

  // ---------------------------------------------------------------- Doubling Chain Acceptance Predicate
  pure def dblChainAccepted(w: DblChainWitness): bool = and {
    w.pin.length() == N_ROWS,
    w.sq.length() == N_ROWS,
    w.lDbl.length() == N_ROWS,
    w.invY.length() == N_ROWS,
    w.x3.length() == N_ROWS,
    w.y3.length() == N_ROWS,
    w.sink.length() == N_ROWS,
    idColOk("none", w.pin[0].rowId, w.pin[1].rowId),
    idColOk("none", w.sq[0].rowId, w.sq[1].rowId),
    idColOk("none", w.lDbl[0].rowId, w.lDbl[1].rowId),
    idColOk("none", w.invY[0].rowId, w.invY[1].rowId),
    x3AirOk("none", w.x3),
    y3AirOk("none", w.y3),
    ROW_LIST.foldl(true, (ok, r) => {
      val p = w.pin[r]
      val sq = w.sq[r]
      val ld = w.lDbl[r]
      val iy = w.invY[r]
      val x = w.x3[r]
      val y = w.y3[r]
      val s = w.sink[r]
      and {
        ok,
        limbsInRange(p.x, BASE),
        limbsInRange(p.y, BASE),
        sq.isValid == 1 and limbsInRange(sq.s, BASE),
        modP(limbsModP(sq.x) * limbsModP(sq.x) - limbsModP(sq.s)) == 0,
        ld.isValid == 1 and limbsInRange(ld.lam, BASE),
        modP(2 * limbsModP(ld.y) * limbsModP(ld.lam) - 3 * limbsModP(ld.s)) == 0,
        iy.isValid == 1 and limbsInRange(iy.w, BASE),
        modP(limbsModP(iy.y) * limbsModP(iy.w) - 1) == 0,
        // Bus balance across o+8, o+9, o, o+2, o+3, o+4, o+5, o+6, o+7, o+10
        sq.rowId == p.rowId and sq.x == p.x,
        ld.rowId == sq.rowId and ld.s == sq.s and ld.x == sq.x,
        ld.rowId == p.rowId and ld.y == p.y,
        x.rowId == ld.rowId and x.lam == ld.lam and x.x1 == ld.x and x.x2 == ld.x,
        y.rowId == ld.rowId and y.lam == ld.lam and y.x1 == ld.x and y.y1 == ld.y and y.x3 == x.x3,
        iy.rowId == ld.rowId and iy.y == ld.y,
        s.rowId == y.rowId and s.x3 == y.x3 and s.y3 == y.y3
      }
    })
  }

  // ---------------------------------------------------------------- Refinement Invariant `invRow`
  // Every input limb of every chain row that is balanced on the bus:
  //   1. is in [0, BASE) = [0, 2^2) (standing for [0, 2^8)),
  //   2. equals the limb that `PointsIn` sent with the same `id` (and row `r` has unique `id = r`
  //      matching `PointsIn[r]`, so coordinates cannot be mixed across rows), and
  //   3. the output point `(x3, y3)` on each row refines the reference affine G1 addition modulo P
  //      of that row's `PointsIn` inputs.
  pure def rowRefines(w: AddChainWitness): bool =
    ROW_LIST.foldl(true, (ok, r) => {
      val p = w.pin[r]
      val l = w.lAdd[r]
      val iv = w.inv[r]
      val x = w.x3[r]
      val y = w.y3[r]
      val s = w.sink[r]
      val refOut = refAddModP(limbsModP(p.x1), limbsModP(p.y1), limbsModP(p.x2), limbsModP(p.y2))
      and {
        ok,
        // Unique row id = r across PointsIn and all ChainAir rows
        p.rowId == r,
        l.rowId == r and iv.rowId == r and x.rowId == r and y.rowId == r and s.rowId == r,
        l.isValid == 1 and iv.isValid == 1 and x.isValid == 1 and y.isValid == 1,
        // All input limbs of L_add are in [0, BASE) and equal PointsIn[r] limbs
        limbsInRange(l.x1, BASE) and limbsInRange(l.y1, BASE),
        limbsInRange(l.x2, BASE) and limbsInRange(l.y2, BASE),
        l.x1 == p.x1 and l.y1 == p.y1 and l.x2 == p.x2 and l.y2 == p.y2,
        // All input limbs of INV are in [0, BASE) and equal PointsIn[r] limbs
        limbsInRange(iv.x1, BASE) and limbsInRange(iv.x2, BASE),
        iv.x1 == p.x1 and iv.x2 == p.x2,
        // All input limbs of X3 are in [0, BASE) and equal L_add[r].lam and PointsIn[r].x1, x2
        limbsInRange(x.lam, BASE) and limbsInRange(x.x1, BASE) and limbsInRange(x.x2, BASE),
        x.lam == l.lam and x.x1 == p.x1 and x.x2 == p.x2,
        // All input limbs of Y3 are in [0, BASE) and equal L_add[r].lam, PointsIn[r].x1, y1, and X3[r].x3
        limbsInRange(y.lam, BASE) and limbsInRange(y.x1, BASE) and limbsInRange(y.x3, BASE) and limbsInRange(y.y1, BASE),
        y.lam == l.lam and y.x1 == p.x1 and y.y1 == p.y1 and y.x3 == x.x3,
        // Output limbs at sink are in [0, BASE) and match the reference G1 affine formula mod P
        limbsInRange(s.x3, BASE) and limbsInRange(s.y3, BASE),
        refOut.ok,
        limbsModP(l.lam) == refOut.lam,
        limbsModP(iv.w) == refOut.w,
        limbsModP(s.x3) == refOut.x3,
        limbsModP(s.y3) == refOut.y3
      }
    })

  // ---------------------------------------------------------------- Honest Trace Builders
  // Build one honest Add row from limb vectors (x1L, y1L, x2L, y2L) with rowId
  pure def buildAddRowFromLimbs(
    rId: int,
    x1L: List[int],
    y1L: List[int],
    x2L: List[int],
    y2L: List[int]
  ): { pin: PointsInRow, lAdd: LAddRow, inv: InvRow, x3: X3Row, y3: Y3Row, sink: SinkRow } = {
    val res = refAddModP(limbsModP(x1L), limbsModP(y1L), limbsModP(x2L), limbsModP(y2L))
    val lamL = intToLimbs(res.lam)
    val wL = intToLimbs(res.w)
    val x3L = intToLimbs(res.x3)
    val y3L = intToLimbs(res.y3)
    {
      pin: { rowId: rId, x1: x1L, y1: y1L, x2: x2L, y2: y2L },
      lAdd: { isValid: 1, rowId: rId, x1: x1L, y1: y1L, x2: x2L, y2: y2L, lam: lamL },
      inv: { isValid: 1, rowId: rId, x1: x1L, x2: x2L, w: wL },
      x3: { isValid: 1, rowId: rId, lam: lamL, x1: x1L, x2: x2L, x3: x3L },
      y3: { isValid: 1, rowId: rId, lam: lamL, x1: x1L, x3: x3L, y1: y1L, y3: y3L },
      sink: { rowId: rId, x3: x3L, y3: y3L }
    }
  }

  // Standard honest Add trace:
  //   Row 0: P1 = (0, 2) + P2 = (2, 5) -> P3 = (10, 9) on E(F_13): y^2 = x^3 + 4
  //   Row 1: P3 = (4, 4) + P4 = (7, 3) -> P5 = (5, 5)  on E(F_13): y^2 = x^3 + 4
  pure val honestAddTrace: AddChainWitness = {
    val r0 = buildAddRowFromLimbs(0, intToLimbs(0), intToLimbs(2), intToLimbs(2), intToLimbs(5))
    val r1 = buildAddRowFromLimbs(1, intToLimbs(4), intToLimbs(4), intToLimbs(7), intToLimbs(3))
    {
      pin: [r0.pin, r1.pin],
      lAdd: [r0.lAdd, r1.lAdd],
      inv: [r0.inv, r1.inv],
      x3: [r0.x3, r1.x3],
      y3: [r0.y3, r1.y3],
      sink: [r0.sink, r1.sink],
      msgTamperBus: 0,
      msgTamperRow: 0,
      msgTamperDelta: 0,
      rngTamperRow: 0,
      rngTamperCoord: 0,
      rngTamperLimb: 0,
      rngTamperDelta: 0
    }
  }

  // Standard honest Doubling trace:
  //   Row 0: 2 * (0, 2) = (0, 11) on E(F_13): y^2 = x^3 + 4
  //   Row 1: 2 * (2, 5) = (12, 9) on E(F_13): y^2 = x^3 + 4
  pure val honestDblTrace: DblChainWitness = {
    val d0 = refDblModP(0, 2)
    val d1 = refDblModP(2, 5)
    val x0 = intToLimbs(0)
    val y0 = intToLimbs(2)
    val s0 = intToLimbs(d0.s)
    val l0 = intToLimbs(d0.lam)
    val w0 = intToLimbs(d0.w)
    val x3_0 = intToLimbs(d0.x3)
    val y3_0 = intToLimbs(d0.y3)

    val x1 = intToLimbs(2)
    val y1 = intToLimbs(5)
    val s1 = intToLimbs(d1.s)
    val l1 = intToLimbs(d1.lam)
    val w1 = intToLimbs(d1.w)
    val x3_1 = intToLimbs(d1.x3)
    val y3_1 = intToLimbs(d1.y3)
    {
      pin: [{ rowId: 0, x: x0, y: y0 }, { rowId: 1, x: x1, y: y1 }],
      sq: [{ isValid: 1, rowId: 0, x: x0, s: s0 }, { isValid: 1, rowId: 1, x: x1, s: s1 }],
      lDbl: [
        { isValid: 1, rowId: 0, s: s0, x: x0, y: y0, lam: l0 },
        { isValid: 1, rowId: 1, s: s1, x: x1, y: y1, lam: l1 }
      ],
      invY: [{ isValid: 1, rowId: 0, y: y0, w: w0 }, { isValid: 1, rowId: 1, y: y1, w: w1 }],
      x3: [
        { isValid: 1, rowId: 0, lam: l0, x1: x0, x2: x0, x3: x3_0 },
        { isValid: 1, rowId: 1, lam: l1, x1: x1, x2: x1, x3: x3_1 }
      ],
      y3: [
        { isValid: 1, rowId: 0, lam: l0, x1: x0, x3: x3_0, y1: y0, y3: y3_0 },
        { isValid: 1, rowId: 1, lam: l1, x1: x1, x3: x3_1, y1: y1, y3: y3_1 }
      ],
      sink: [{ rowId: 0, x3: x3_0, y3: y3_0 }, { rowId: 1, x3: x3_1, y3: y3_1 }]
    }
  }

  // Question (a) witness: non-canonical input x1 = 13 = [1, 3] (limbs in [0, 4), value 13 >= P = 13, 13 == 0 mod 13)
  pure val nonCanonicalInputTrace: AddChainWitness = {
    val r0 = buildAddRowFromLimbs(0, [1, 3], intToLimbs(2), intToLimbs(2), intToLimbs(5))
    val r1 = buildAddRowFromLimbs(1, intToLimbs(4), intToLimbs(4), intToLimbs(7), intToLimbs(3))
    { ...honestAddTrace,
      pin: [r0.pin, r1.pin],
      lAdd: [r0.lAdd, r1.lAdd],
      inv: [r0.inv, r1.inv],
      x3: [r0.x3, r1.x3],
      y3: [r0.y3, r1.y3],
      sink: [r0.sink, r1.sink]
    }
  }

  // Question (b) witness: off-curve input (x1=1, y1=1) where 1^2 = 1 != 1^3 + 4 = 5 (mod 13)
  pure val offCurveInputTrace: AddChainWitness = {
    val r0 = buildAddRowFromLimbs(0, intToLimbs(1), intToLimbs(1), intToLimbs(2), intToLimbs(5))
    val r1 = buildAddRowFromLimbs(1, intToLimbs(4), intToLimbs(4), intToLimbs(7), intToLimbs(3))
    { ...honestAddTrace,
      pin: [r0.pin, r1.pin],
      lAdd: [r0.lAdd, r1.lAdd],
      inv: [r0.inv, r1.inv],
      x3: [r0.x3, r1.x3],
      y3: [r0.y3, r1.y3],
      sink: [r0.sink, r1.sink]
    }
  }

  // ---------------------------------------------------------------- Named Mutant Counterexample Witnesses
  // 1. Counterexample for `m_no_range` and `m_wrong_bits`:
  //    Row 0 x1 limb 0 is set to 4 (outside [0, BASE) = [0, 4), inside [0, WRONG_BASE) = [0, 8)).
  //    [4, 0] represents integer 4, so the chain computes valid mod-13 outputs for x1 = 4,
  //    balancing all buses while violating `invRow` (limb 4 is not in [0, 4)).
  pure val cex_out_of_range_limb: AddChainWitness = {
    val r0 = buildAddRowFromLimbs(0, [4, 0], intToLimbs(2), intToLimbs(2), intToLimbs(5))
    val r1 = buildAddRowFromLimbs(1, intToLimbs(4), intToLimbs(4), intToLimbs(7), intToLimbs(3))
    { ...honestAddTrace,
      pin: [r0.pin, r1.pin],
      lAdd: [r0.lAdd, r1.lAdd],
      inv: [r0.inv, r1.inv],
      x3: [r0.x3, r1.x3],
      y3: [r0.y3, r1.y3],
      sink: [r0.sink, r1.sink]
    }
  }

  // 2. Counterexample for `m_no_id_constraint` (matching `neg == "dup_id"` in `x_exp_node.rs:844, 999`):
  //    Row 1 has `rowId = 0` in every AIR (duplicate id = 0).
  //    Because both rows share `rowId = 0` on buses 1100 and 1101, `L_add` row 0 mixes
  //    P1 = (0, 2) from `PointsIn` row 0 with P2 = (7, 3) from `PointsIn` row 1,
  //    while `L_add` row 1 mixes P1 = (4, 4) from `PointsIn` row 1 with P2 = (2, 5) from `PointsIn` row 0!
  //    All buses balance and all FieldExpr constraints pass, but `invRow` is violated!
  pure val cex_no_id_constraint_mix: AddChainWitness = {
    val pin0: PointsInRow = { rowId: 0, x1: intToLimbs(0), y1: intToLimbs(2), x2: intToLimbs(2), y2: intToLimbs(5) }
    val pin1: PointsInRow = { rowId: 0, x1: intToLimbs(4), y1: intToLimbs(4), x2: intToLimbs(7), y2: intToLimbs(3) }
    val mix0 = buildAddRowFromLimbs(0, pin0.x1, pin0.y1, pin1.x2, pin1.y2)
    val mix1 = buildAddRowFromLimbs(0, pin1.x1, pin1.y1, pin0.x2, pin0.y2)
    { ...honestAddTrace,
      pin: [pin0, pin1],
      lAdd: [mix0.lAdd, mix1.lAdd],
      inv: [mix0.inv, mix1.inv],
      x3: [mix0.x3, mix1.x3],
      y3: [mix0.y3, mix1.y3],
      sink: [mix0.sink, mix1.sink]
    }
  }

  // 3. Exceptional case `same_pt` (`x_exp_node.rs:843, 986-1024`):
  //    Row 0 has P2 = P1 = (0, 2) (`x1 == x2`), forged lam = 5, forged w = 0.
  //    Rejected by `INV` (`(x2 - x1) * w - 1 == -1 != 0 (mod 13)`).
  pure val neg_same_pt_trace: AddChainWitness = {
    val base = honestAddTrace
    val x1L = intToLimbs(0)
    val y1L = intToLimbs(2)
    val lamL = intToLimbs(5)
    val x3Val = modP(5 * 5 - 0 - 0)
    val y3Val = modP(5 * (0 - x3Val) - 2)
    val x3L = intToLimbs(x3Val)
    val y3L = intToLimbs(y3Val)
    val p0: PointsInRow = { rowId: 0, x1: x1L, y1: y1L, x2: x1L, y2: y1L }
    val l0: LAddRow = { isValid: 1, rowId: 0, x1: x1L, y1: y1L, x2: x1L, y2: y1L, lam: lamL }
    val iv0: InvRow = { isValid: 1, rowId: 0, x1: x1L, x2: x1L, w: intToLimbs(0) }
    val x0: X3Row = { isValid: 1, rowId: 0, lam: lamL, x1: x1L, x2: x1L, x3: x3L }
    val y0: Y3Row = { isValid: 1, rowId: 0, lam: lamL, x1: x1L, x3: x3L, y1: y1L, y3: y3L }
    val s0: SinkRow = { rowId: 0, x3: x3L, y3: y3L }
    { ...base,
      pin: [p0, base.pin[1]],
      lAdd: [l0, base.lAdd[1]],
      inv: [iv0, base.inv[1]],
      x3: [x0, base.x3[1]],
      y3: [y0, base.y3[1]],
      sink: [s0, base.sink[1]]
    }
  }

  // 4. Exceptional case `y_zero` (`x_exp_node.rs:844, 986-1024`):
  //    Doubling row 0 has P = (0, 0) (`y == 0`), forged lam = 5, forged w = 0.
  //    Rejected by `INV_y` (`y * w - 1 == -1 != 0 (mod 13)`).
  pure val neg_y_zero_dbl_trace: DblChainWitness = {
    val base = honestDblTrace
    val zL = intToLimbs(0)
    val lamL = intToLimbs(5)
    val x3Val = modP(5 * 5 - 0)
    val y3Val = modP(5 * (0 - x3Val) - 0)
    val x3L = intToLimbs(x3Val)
    val y3L = intToLimbs(y3Val)
    { ...base,
      pin: [{ rowId: 0, x: zL, y: zL }, base.pin[1]],
      sq: [{ isValid: 1, rowId: 0, x: zL, s: zL }, base.sq[1]],
      lDbl: [{ isValid: 1, rowId: 0, s: zL, x: zL, y: zL, lam: lamL }, base.lDbl[1]],
      invY: [{ isValid: 1, rowId: 0, y: zL, w: zL }, base.invY[1]],
      x3: [{ isValid: 1, rowId: 0, lam: lamL, x1: zL, x2: zL, x3: x3L }, base.x3[1]],
      y3: [{ isValid: 1, rowId: 0, lam: lamL, x1: zL, x3: x3L, y1: zL, y3: y3L }, base.y3[1]],
      sink: [{ rowId: 0, x3: x3L, y3: y3L }, base.sink[1]]
    }
  }

  // ---------------------------------------------------------------- Nondeterministic Single-Cell / Bus / Lookup Tamper Prover
  // Tamper kinds:
  //   0: Honest trace (`honestAddTrace`)
  //   1: Out-of-range limb in `PointsIn` row 0 x1[0] = `limbVal` (propagated across chain) -> tests `no_range`, `wrong_bits`
  //   2: Duplicate id = 0 on row 1 with cross-row point mixing (`cex_no_id_constraint_mix`) -> tests `no_id_constraint`
  //   3: Single-cell tamper in `PointsIn` row `r` (`x1[0] = limbVal`) WITHOUT updating `L_add` -> tests bus 1100 binding
  //   4: Single-cell tamper in `L_add` row `r` (`x2[0] = limbVal`) WITHOUT updating `PointsIn` -> tests bus 1101 binding
  //   5: Single-cell tamper in `L_add` row `r` (`lam[0] = limbVal`) -> tests `L_add` FieldExpr & bus 1102/1104
  //   6: Single-cell tamper in `X3` row `r` (`x3[0] = limbVal`) -> tests `X3` FieldExpr & bus 1106
  //   7: Single-cell tamper in `PointsIn` row `r` (`rowId = limbVal`) -> tests `id` constraint
  //   8: Drop or duplicate one permutation bus message (`msgTamperBus`, `msgTamperRow`, `msgTamperDelta`)
  //   9: Drop or duplicate one range lookup in `PointsIn` (`rngTamperRow`, `rngTamperCoord`, `rngTamperLimb`, `rngTamperDelta`)
  //  10: Non-canonical input trace (`nonCanonicalInputTrace`, x1 = 13 = [1, 3])
  pure def buildTamperedWitness(
    tKind: int,
    r: int,
    limbVal: int,
    busId: int,
    delta: int
  ): AddChainWitness = {
    val base = honestAddTrace
    if (tKind == 0)
      base
    else if (tKind == 1) {
      val row0 = buildAddRowFromLimbs(0, [limbVal, 0], intToLimbs(2), intToLimbs(2), intToLimbs(5))
      { ...base,
        pin: [row0.pin, base.pin[1]],
        lAdd: [row0.lAdd, base.lAdd[1]],
        inv: [row0.inv, base.inv[1]],
        x3: [row0.x3, base.x3[1]],
        y3: [row0.y3, base.y3[1]],
        sink: [row0.sink, base.sink[1]]
      }
    } else if (tKind == 2)
      cex_no_id_constraint_mix
    else if (tKind == 3) {
      val pRow = base.pin[r]
      val pNew = { ...pRow, x1: [limbVal, pRow.x1[1]] }
      { ...base, pin: base.pin.replaceAt(r, pNew) }
    } else if (tKind == 4) {
      val lRow = base.lAdd[r]
      val lNew = { ...lRow, x2: [limbVal, lRow.x2[1]] }
      { ...base, lAdd: base.lAdd.replaceAt(r, lNew) }
    } else if (tKind == 5) {
      val lRow = base.lAdd[r]
      val lNew = { ...lRow, lam: [limbVal, lRow.lam[1]] }
      { ...base, lAdd: base.lAdd.replaceAt(r, lNew) }
    } else if (tKind == 6) {
      val xRow = base.x3[r]
      val xNew = { ...xRow, x3: [limbVal, xRow.x3[1]] }
      { ...base, x3: base.x3.replaceAt(r, xNew) }
    } else if (tKind == 7) {
      val pRow = base.pin[r]
      val pNew = { ...pRow, rowId: limbVal }
      { ...base, pin: base.pin.replaceAt(r, pNew) }
    } else if (tKind == 8) {
      { ...base, msgTamperBus: busId, msgTamperRow: r, msgTamperDelta: delta }
    } else if (tKind == 9) {
      { ...base, rngTamperRow: r, rngTamperCoord: 0, rngTamperLimb: 0, rngTamperDelta: delta }
    } else
      nonCanonicalInputTrace
  }

  // ---------------------------------------------------------------- State Machine
  pure val TAMPER_KINDS: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
  pure val ROW_DOM: Set[int] = Set(0, 1)
  pure val LIMB_DOM: Set[int] = Set(0, 1, 2, 3, 4, 7)
  pure val BUS_DOM: Set[int] = Set(1100, 1101, 1102, 1106, 1107, 1110)
  pure val DELTA_DOM: Set[int] = Set(-1, 1)

  var w: AddChainWitness
  var acc: bool
  var isTampered: bool

  action init = {
    val initW = honestAddTrace
    all {
      w' = initW,
      acc' = accepts(initW),
      isTampered' = false
    }
  }

  action step = {
    nondet tKind = TAMPER_KINDS.oneOf()
    nondet r = ROW_DOM.oneOf()
    nondet limbVal = LIMB_DOM.oneOf()
    nondet busId = BUS_DOM.oneOf()
    nondet delta = DELTA_DOM.oneOf()
    val cand = buildTamperedWitness(tKind, r, limbVal, busId, delta)
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != honestAddTrace)
    }
  }

  // ---------------------------------------------------------------- Invariants
  // Main refinement invariant `invRow`:
  //   1. Completeness & bus balance on the honest chain (`accepts(honestAddTrace)`), AND
  //   2. For every accepted trace `w`, every input limb of every chain row is in [0, BASE) = [0, 4)
  //      and equals the limb that `PointsIn` sent with the same `id`, and outputs refine G1 add mod P.
  val invRow: bool =
    accepts(honestAddTrace) and (acc implies rowRefines(w))

  // Alias `invRefine` for runner compatibility
  val invRefine: bool = invRow

  // Non-vacuity reachability invariant (negated: MUST be violated with exit 1 on `points_in_sound`):
  // Witnesses an accepted honest chain.
  val reachHonestChain: bool =
    not(acc and not(isTampered) and rowRefines(w))

  // Non-vacuity reachability invariant for Question (a) (negated: MUST be violated with exit 1 on `points_in_sound`):
  // Witnesses an accepted chain with a non-canonical input (x1 = 13 >= P, limbs [1, 3] in [0, 4)) that still refines mod P.
  val reachAcceptedNonCanonical: bool =
    not(acc and w == nonCanonicalInputTrace and rowRefines(w))

  // Bad-tamper reachability invariant (holds with exit 0 on `points_in_sound`; violated with exit 1 on `m_no_range`, `m_wrong_bits`, `m_no_id_constraint`):
  val reachAcceptedBadTamper: bool =
    not(acc and not(rowRefines(w)))
}

// ---------------------------------------------------------------- Module Instances
// Sound baseline module
module points_in_sound { import points_in_spec(MUTANT = "none").* }
module m_sound { import points_in_spec(MUTANT = "none").* }

// Required mutant modules `m_<name>` (every one MUST violate `invRow` with exit code 1)
module m_no_range { import points_in_spec(MUTANT = "no_range").* }
module m_wrong_bits { import points_in_spec(MUTANT = "wrong_bits").* }
module m_no_send { import points_in_spec(MUTANT = "no_send").* }
module m_dup_send { import points_in_spec(MUTANT = "dup_send").* }
module m_no_id_constraint { import points_in_spec(MUTANT = "no_id_constraint").* }
