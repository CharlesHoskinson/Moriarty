// -*- mode: Bluespec; -*-
// Soundness model of X14 BLS12-381 G1 PointsIn padding (is_valid / v column).
// Prime field P = 13, curve y^2 = x^3 + 4 (mod 13), unit height N = 4 rows.

module x14_pad_defs {
  pure val P: int = 13
  pure val N: int = 4
  pure val ROWS: Set[int] = Set(0, 1, 2, 3)
  pure val ROW_LIST: List[int] = List(0, 1, 2, 3)
  pure val TRANS_ROWS: Set[int] = Set(0, 1, 2)

  // Bus indices matching lane-src/x_exp_node.rs (add chain, o = 1100, RANGE_BUS = 1000)
  pure val BUS_RANGE: int = 1000
  pure val BUS_P1: int = 1100
  pure val BUS_P2: int = 1101
  pure val BUS_LAM_X1_TO_X3: int = 1102
  pure val BUS_X2_TO_X3: int = 1103
  pure val BUS_LAM_X1_TO_Y3: int = 1104
  pure val BUS_Y1_TO_Y3: int = 1105
  pure val BUS_X3_TO_Y3: int = 1106
  pure val BUS_OUT: int = 1107
  pure val BUS_DENOM_TO_INV: int = 1110

  type PointsInRow = {
    id: int,
    v: int,
    f0: int,
    f1: int,
    g: int,
    e: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int
  }

  type LAddRow = {
    id: int,
    is_valid: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    lam: int
  }

  type InvRow = {
    id: int,
    is_valid: int,
    x1: int,
    x2: int,
    w: int
  }

  type X3Row = {
    id: int,
    is_valid: int,
    lam: int,
    x1: int,
    x2: int,
    x3: int
  }

  type Y3Row = {
    id: int,
    is_valid: int,
    lam: int,
    x1: int,
    x3: int,
    y1: int,
    y3: int
  }

  type SinkRow = {
    id: int,
    v: int,
    flag: int,
    x: int,
    y: int
  }

  type BusPerturb = {
    active: bool,
    bus: int,
    row_id: int,
    delta: int
  }

  type Trace = {
    pin: List[PointsInRow],
    l_add: List[LAddRow],
    inv_air: List[InvRow],
    x3_air: List[X3Row],
    y3_air: List[Y3Row],
    sink: List[SinkRow],
    perturb: BusPerturb
  }

  pure def modP(x: int): int = {
    val r = x % P
    if (r < 0) r + P else r
  }

  // Multiplicative inverse mod P = 13 (with 0 -> 0 for FieldExpr evaluation on 0)
  pure def invModP(a: int): int = {
    val m = modP(a)
    if (m == 0) 0
    else if (m == 1) 1
    else if (m == 2) 7
    else if (m == 3) 9
    else if (m == 4) 10
    else if (m == 5) 8
    else if (m == 6) 11
    else if (m == 7) 2
    else if (m == 8) 5
    else if (m == 9) 3
    else if (m == 10) 4
    else if (m == 11) 6
    else 12
  }

  pure def isBool(b: int): bool = (b == 0 or b == 1)

  pure def inField(x: int): bool = (0 <= x and x < P)

  // FieldExpr canonical expressions mod P = 13
  pure def exprLam(x1: int, y1: int, x2: int, y2: int): int =
    modP((y2 - y1) * invModP(x2 - x1))

  pure def exprInv(x1: int, x2: int): int =
    invModP(x2 - x1)

  pure def exprX3(lam: int, x1: int, x2: int): int =
    modP(lam * lam - x1 - x2)

  pure def exprY3(lam: int, x1: int, x3: int, y1: int): int =
    modP(lam * (x1 - x3) - y1)

  // Canonical valid G1 input pairs on y^2 = x^3 + 4 (mod 13) with x1 != x2:
  // r=0: (0,2) + (2,5)  -> dx=2, w=7, lam=8,  x3=10, y3=9
  // r=1: (4,4) + (5,5)  -> dx=1, w=1, lam=1,  x3=5,  y3=8
  // r=2: (6,5) + (7,3)  -> dx=1, w=1, lam=11, x3=4,  y3=4
  // r=3: (8,3) + (10,4) -> dx=2, w=7, lam=7,  x3=5,  y3=5
  pure def canonX1(r: int): int =
    if (r == 0) 0 else if (r == 1) 4 else if (r == 2) 6 else 8

  pure def canonY1(r: int): int =
    if (r == 0) 2 else if (r == 1) 4 else if (r == 2) 5 else 3

  pure def canonX2(r: int): int =
    if (r == 0) 2 else if (r == 1) 5 else if (r == 2) 7 else 10

  pure def canonY2(r: int): int =
    if (r == 0) 5 else if (r == 1) 5 else if (r == 2) 3 else 4

  pure def canonW(r: int): int =
    exprInv(canonX1(r), canonX2(r))

  pure def canonLam(r: int): int =
    exprLam(canonX1(r), canonY1(r), canonX2(r), canonY2(r))

  pure def canonX3(r: int): int =
    exprX3(canonLam(r), canonX1(r), canonX2(r))

  pure def canonY3(r: int): int =
    exprY3(canonLam(r), canonX1(r), canonX3(r), canonY1(r))

  pure val noPerturb: BusPerturb = { active: false, bus: 0, row_id: 0, delta: 0 }

  pure def honestPinRow(r: int, active: bool): PointsInRow = {
    val vBit = if (active) 1 else 0
    {
      id: r,
      v: vBit,
      f0: 0,
      f1: 0,
      g: 1,
      e: vBit,
      x1: if (active) canonX1(r) else 0,
      y1: if (active) canonY1(r) else 0,
      x2: if (active) canonX2(r) else 0,
      y2: if (active) canonY2(r) else 0
    }
  }

  // Note: check_carry_to_zero.rs does not gate carry constraints by is_valid, so
  // an honest padding chain row holds a satisfying dummy witness with is_valid = 0.
  pure def honestLAddRow(r: int, active: bool): LAddRow = {
    id: r,
    is_valid: if (active) 1 else 0,
    x1: canonX1(r),
    y1: canonY1(r),
    x2: canonX2(r),
    y2: canonY2(r),
    lam: canonLam(r)
  }

  pure def honestInvRow(r: int, active: bool): InvRow = {
    id: r,
    is_valid: if (active) 1 else 0,
    x1: canonX1(r),
    x2: canonX2(r),
    w: canonW(r)
  }

  pure def honestX3Row(r: int, active: bool): X3Row = {
    id: r,
    is_valid: if (active) 1 else 0,
    lam: canonLam(r),
    x1: canonX1(r),
    x2: canonX2(r),
    x3: canonX3(r)
  }

  pure def honestY3Row(r: int, active: bool): Y3Row = {
    id: r,
    is_valid: if (active) 1 else 0,
    lam: canonLam(r),
    x1: canonX1(r),
    x3: canonX3(r),
    y1: canonY1(r),
    y3: canonY3(r)
  }

  pure def honestSinkRow(r: int, active: bool): SinkRow = {
    id: r,
    v: if (active) 1 else 0,
    flag: 0,
    x: if (active) canonX3(r) else 0,
    y: if (active) canonY3(r) else 0
  }

  pure def honestTrace(nValid: int): Trace = {
    pin: List(
      honestPinRow(0, 0 < nValid),
      honestPinRow(1, 1 < nValid),
      honestPinRow(2, 2 < nValid),
      honestPinRow(3, 3 < nValid)
    ),
    l_add: List(
      honestLAddRow(0, 0 < nValid),
      honestLAddRow(1, 1 < nValid),
      honestLAddRow(2, 2 < nValid),
      honestLAddRow(3, 3 < nValid)
    ),
    inv_air: List(
      honestInvRow(0, 0 < nValid),
      honestInvRow(1, 1 < nValid),
      honestInvRow(2, 2 < nValid),
      honestInvRow(3, 3 < nValid)
    ),
    x3_air: List(
      honestX3Row(0, 0 < nValid),
      honestX3Row(1, 1 < nValid),
      honestX3Row(2, 2 < nValid),
      honestX3Row(3, 3 < nValid)
    ),
    y3_air: List(
      honestY3Row(0, 0 < nValid),
      honestY3Row(1, 1 < nValid),
      honestY3Row(2, 2 < nValid),
      honestY3Row(3, 3 < nValid)
    ),
    sink: List(
      honestSinkRow(0, 0 < nValid),
      honestSinkRow(1, 1 < nValid),
      honestSinkRow(2, 2 < nValid),
      honestSinkRow(3, 3 < nValid)
    ),
    perturb: noPerturb
  }

  // Pure record update helpers
  pure def pinWithVE(row: PointsInRow, newV: int, newE: int): PointsInRow = {
    id: row.id, v: newV, f0: row.f0, f1: row.f1, g: row.g, e: newE,
    x1: row.x1, y1: row.y1, x2: row.x2, y2: row.y2
  }

  pure def pinWithX1(row: PointsInRow, newX1: int): PointsInRow = {
    id: row.id, v: row.v, f0: row.f0, f1: row.f1, g: row.g, e: row.e,
    x1: newX1, y1: row.y1, x2: row.x2, y2: row.y2
  }

  pure def lAddWithValid(row: LAddRow, newValid: int): LAddRow = {
    id: row.id, is_valid: newValid,
    x1: row.x1, y1: row.y1, x2: row.x2, y2: row.y2, lam: row.lam
  }

  pure def lAddWithLam(row: LAddRow, newLam: int): LAddRow = {
    id: row.id, is_valid: row.is_valid,
    x1: row.x1, y1: row.y1, x2: row.x2, y2: row.y2, lam: newLam
  }

  pure def invWithValid(row: InvRow, newValid: int): InvRow = {
    id: row.id, is_valid: newValid, x1: row.x1, x2: row.x2, w: row.w
  }

  pure def invWithW(row: InvRow, newW: int): InvRow = {
    id: row.id, is_valid: row.is_valid, x1: row.x1, x2: row.x2, w: newW
  }

  pure def x3WithValid(row: X3Row, newValid: int): X3Row = {
    id: row.id, is_valid: newValid, lam: row.lam, x1: row.x1, x2: row.x2, x3: row.x3
  }

  pure def x3WithOut(row: X3Row, newX3: int): X3Row = {
    id: row.id, is_valid: row.is_valid, lam: row.lam, x1: row.x1, x2: row.x2, x3: newX3
  }

  pure def y3WithValid(row: Y3Row, newValid: int): Y3Row = {
    id: row.id, is_valid: newValid, lam: row.lam, x1: row.x1, x3: row.x3, y1: row.y1, y3: row.y3
  }

  pure def y3WithValidAndOut(row: Y3Row, newValid: int, newY3: int): Y3Row = {
    id: row.id, is_valid: newValid, lam: row.lam, x1: row.x1, x3: row.x3, y1: row.y1, y3: newY3
  }

  pure def sinkWithV(row: SinkRow, newV: int): SinkRow = {
    id: row.id, v: newV, flag: row.flag, x: row.x, y: row.y
  }

  pure def sinkWithY(row: SinkRow, newY: int): SinkRow = {
    id: row.id, v: row.v, flag: row.flag, x: row.x, y: newY
  }

  pure def sinkWithVXY(row: SinkRow, newV: int, newX: int, newY: int): SinkRow = {
    id: row.id, v: newV, flag: row.flag, x: newX, y: newY
  }

  // --------------------------------------------------------------------------
  // AIR polynomial constraints (parameterized by mutant flags)
  // --------------------------------------------------------------------------

  pure def checkPointsIn(
    pin: List[PointsInRow],
    enforcePrefix: bool,
    enforceVBool: bool
  ): bool = {
    val firstOk = (pin[0].id == 0)
    val transOk = TRANS_ROWS.forall(r =>
      pin[r + 1].id == pin[r].id + 1 and
      (not(enforcePrefix) or (pin[r + 1].v * (1 - pin[r].v) == 0))
    )
    val rowsOk = ROWS.forall(r => {
      val row = pin[r]
      (not(enforceVBool) or isBool(row.v)) and
      isBool(row.f0) and isBool(row.f1) and
      row.f0 == 0 and row.f1 == 0 and
      row.g == (1 - row.f0) * (1 - row.f1) and
      row.e == row.v * row.g and
      row.f0 * row.x1 == 0 and row.f0 * row.y1 == 0 and
      row.f1 * row.x2 == 0 and row.f1 * row.y2 == 0
    })
    firstOk and transOk and rowsOk
  }

  // FieldExpr row: output var equals the expression mod P for canonical inputs when is_valid == 1
  pure def checkLAdd(rows: List[LAddRow], enforceValidBool: bool): bool = {
    (rows[0].id == 0) and
    TRANS_ROWS.forall(r => rows[r + 1].id == rows[r].id + 1) and
    ROWS.forall(r => {
      val row = rows[r]
      (not(enforceValidBool) or isBool(row.is_valid)) and
      ((row.is_valid == 1) implies (row.lam == exprLam(row.x1, row.y1, row.x2, row.y2)))
    })
  }

  pure def checkInv(rows: List[InvRow], enforceValidBool: bool): bool = {
    (rows[0].id == 0) and
    TRANS_ROWS.forall(r => rows[r + 1].id == rows[r].id + 1) and
    ROWS.forall(r => {
      val row = rows[r]
      (not(enforceValidBool) or isBool(row.is_valid)) and
      ((row.is_valid == 1) implies (row.w == exprInv(row.x1, row.x2)))
    })
  }

  pure def checkX3(rows: List[X3Row], enforceValidBool: bool): bool = {
    (rows[0].id == 0) and
    TRANS_ROWS.forall(r => rows[r + 1].id == rows[r].id + 1) and
    ROWS.forall(r => {
      val row = rows[r]
      (not(enforceValidBool) or isBool(row.is_valid)) and
      ((row.is_valid == 1) implies (row.x3 == exprX3(row.lam, row.x1, row.x2)))
    })
  }

  pure def checkY3(rows: List[Y3Row], enforceValidBool: bool): bool = {
    (rows[0].id == 0) and
    TRANS_ROWS.forall(r => rows[r + 1].id == rows[r].id + 1) and
    ROWS.forall(r => {
      val row = rows[r]
      (not(enforceValidBool) or isBool(row.is_valid)) and
      ((row.is_valid == 1) implies (row.y3 == exprY3(row.lam, row.x1, row.x3, row.y1)))
    })
  }

  pure def checkSink(rows: List[SinkRow], enforceVBool: bool): bool = {
    (rows[0].id == 0) and
    TRANS_ROWS.forall(r => rows[r + 1].id == rows[r].id + 1) and
    ROWS.forall(r => not(enforceVBool) or isBool(rows[r].v))
  }

  // --------------------------------------------------------------------------
  // Range check bus (RANGE_BUS = 1000) and PermutationCheckBus (1100..1110)
  // --------------------------------------------------------------------------

  pure def checkRangeBus(t: Trace, pinMultIsV: bool): bool = {
    ROWS.forall(r => {
      val pRow = t.pin[r]
      val pMult = if (pinMultIsV) pRow.v else 1
      val pinOk = (pMult == 0) or (
        inField(pRow.x1) and inField(pRow.y1) and inField(pRow.x2) and inField(pRow.y2)
      )
      val lRow = t.l_add[r]
      val lOk = (lRow.is_valid == 0) or (
        inField(lRow.x1) and inField(lRow.y1) and inField(lRow.x2) and inField(lRow.y2) and inField(lRow.lam)
      )
      val iRow = t.inv_air[r]
      val iOk = (iRow.is_valid == 0) or (
        inField(iRow.x1) and inField(iRow.x2) and inField(iRow.w)
      )
      val xRow = t.x3_air[r]
      val xOk = (xRow.is_valid == 0) or (
        inField(xRow.lam) and inField(xRow.x1) and inField(xRow.x2) and inField(xRow.x3)
      )
      val yRow = t.y3_air[r]
      val yOk = (yRow.is_valid == 0) or (
        inField(yRow.lam) and inField(yRow.x1) and inField(yRow.x3) and inField(yRow.y1) and inField(yRow.y3)
      )
      pinOk and lOk and iOk and xOk and yOk
    })
  }

  pure def perturbDelta(p: BusPerturb, b: int, r: int): int =
    if (p.active and p.bus == b and p.row_id == r) p.delta else 0

  pure def pairBusRowBalanced(
    sTuple: List[int],
    sMult: int,
    rTuple: List[int],
    rMult: int,
    delta: int
  ): bool = {
    val effSend = sMult + delta
    if (sTuple == rTuple) {
      effSend == rMult
    } else {
      effSend == 0 and rMult == 0
    }
  }

  pure def checkPermutationBuses(
    t: Trace,
    pinSendMultOne: bool,
    y3ValidFree: bool
  ): bool = {
    ROWS.forall(r => {
      val pRow = t.pin[r]
      val lRow = t.l_add[r]
      val iRow = t.inv_air[r]
      val xRow = t.x3_air[r]
      val yRow = t.y3_air[r]
      val sRow = t.sink[r]

      val pinSendMult = if (pinSendMultOne) 1 else pRow.e
      val y3Mult = if (y3ValidFree) 1 else yRow.is_valid

      // Bus o (1100): PointsIn sends (id, x1, y1), L_add receives (id, x1, y1)
      val b0 = pairBusRowBalanced(
        List(pRow.id, pRow.x1, pRow.y1), pinSendMult,
        List(lRow.id, lRow.x1, lRow.y1), lRow.is_valid,
        perturbDelta(t.perturb, BUS_P1, r)
      )
      // Bus o+1 (1101): PointsIn sends (id, x2, y2), L_add receives (id, x2, y2)
      val b1 = pairBusRowBalanced(
        List(pRow.id, pRow.x2, pRow.y2), pinSendMult,
        List(lRow.id, lRow.x2, lRow.y2), lRow.is_valid,
        perturbDelta(t.perturb, BUS_P2, r)
      )
      // Bus o+2 (1102): L_add sends (id, lam, x1), X3 receives (id, lam, x1)
      val b2 = pairBusRowBalanced(
        List(lRow.id, lRow.lam, lRow.x1), lRow.is_valid,
        List(xRow.id, xRow.lam, xRow.x1), xRow.is_valid,
        perturbDelta(t.perturb, BUS_LAM_X1_TO_X3, r)
      )
      // Bus o+3 (1103): L_add sends (id, x2), X3 receives (id, x2)
      val b3 = pairBusRowBalanced(
        List(lRow.id, lRow.x2), lRow.is_valid,
        List(xRow.id, xRow.x2), xRow.is_valid,
        perturbDelta(t.perturb, BUS_X2_TO_X3, r)
      )
      // Bus o+4 (1104): L_add sends (id, lam, x1), Y3 receives (id, lam, x1)
      val b4 = pairBusRowBalanced(
        List(lRow.id, lRow.lam, lRow.x1), lRow.is_valid,
        List(yRow.id, yRow.lam, yRow.x1), y3Mult,
        perturbDelta(t.perturb, BUS_LAM_X1_TO_Y3, r)
      )
      // Bus o+5 (1105): L_add sends (id, y1), Y3 receives (id, y1)
      val b5 = pairBusRowBalanced(
        List(lRow.id, lRow.y1), lRow.is_valid,
        List(yRow.id, yRow.y1), y3Mult,
        perturbDelta(t.perturb, BUS_Y1_TO_Y3, r)
      )
      // Bus o+6 (1106): X3 sends (id, x3), Y3 receives (id, x3)
      val b6 = pairBusRowBalanced(
        List(xRow.id, xRow.x3), xRow.is_valid,
        List(yRow.id, yRow.x3), y3Mult,
        perturbDelta(t.perturb, BUS_X3_TO_Y3, r)
      )
      // Bus o+10 (1110): L_add sends (id, x1, x2), INV receives (id, x1, x2)
      val b10 = pairBusRowBalanced(
        List(lRow.id, lRow.x1, lRow.x2), lRow.is_valid,
        List(iRow.id, iRow.x1, iRow.x2), iRow.is_valid,
        perturbDelta(t.perturb, BUS_DENOM_TO_INV, r)
      )
      // Bus o+7 (1107): Y3 sends (id, x3, y3, 0) with mult y3Mult;
      // PointsIn sends bypass with mult (v - e) = 0; Sink receives (id, x, y, flag) with mult sRow.v
      val b7 = pairBusRowBalanced(
        List(yRow.id, yRow.x3, yRow.y3, 0), y3Mult,
        List(sRow.id, sRow.x, sRow.y, sRow.flag), sRow.v,
        perturbDelta(t.perturb, BUS_OUT, r)
      )

      b0 and b1 and b2 and b3 and b4 and b5 and b6 and b10 and b7
    })
  }

  pure def verifyTrace(
    t: Trace,
    enforcePrefix: bool,
    enforceVBool: bool,
    pinSendMultOne: bool,
    y3ValidFree: bool,
    pinMultIsV: bool
  ): bool = {
    checkPointsIn(t.pin, enforcePrefix, enforceVBool) and
    checkLAdd(t.l_add, enforceVBool) and
    checkInv(t.inv_air, enforceVBool) and
    checkX3(t.x3_air, enforceVBool) and
    checkY3(t.y3_air, enforceVBool) and
    checkSink(t.sink, enforceVBool) and
    checkRangeBus(t, pinMultIsV) and
    checkPermutationBuses(t, pinSendMultOne, y3ValidFree)
  }

  // --------------------------------------------------------------------------
  // Refinement Invariant `invRow`
  // --------------------------------------------------------------------------

  pure def countValidPin(t: Trace): int =
    ROW_LIST.foldl(0, (acc, r) => if (t.pin[r].v == 1) acc + 1 else acc)

  pure def holdsRefinement(t: Trace): bool = {
    val n = countValidPin(t)
    val prefixOk = ROWS.forall(r =>
      isBool(t.pin[r].v) and
      ((r < n) iff (t.pin[r].v == 1))
    )
    val chainOk = ROWS.forall(r => {
      val pRow = t.pin[r]
      val lRow = t.l_add[r]
      val iRow = t.inv_air[r]
      val xRow = t.x3_air[r]
      val yRow = t.y3_air[r]
      val sRow = t.sink[r]

      val anyChainValid = (
        lRow.is_valid != 0 or iRow.is_valid != 0 or xRow.is_valid != 0 or yRow.is_valid != 0
      )
      val allChainValidOne = (
        lRow.is_valid == 1 and iRow.is_valid == 1 and xRow.is_valid == 1 and yRow.is_valid == 1
      )
      if (r < n) {
        allChainValidOne and
        lRow.id == r and iRow.id == r and xRow.id == r and yRow.id == r and
        lRow.x1 == pRow.x1 and lRow.y1 == pRow.y1 and lRow.x2 == pRow.x2 and lRow.y2 == pRow.y2 and
        iRow.x1 == pRow.x1 and iRow.x2 == pRow.x2 and
        xRow.lam == lRow.lam and xRow.x1 == pRow.x1 and xRow.x2 == pRow.x2 and
        yRow.lam == lRow.lam and yRow.x1 == pRow.x1 and yRow.x3 == xRow.x3 and yRow.y1 == pRow.y1 and
        iRow.w == exprInv(pRow.x1, pRow.x2) and
        lRow.lam == exprLam(pRow.x1, pRow.y1, pRow.x2, pRow.y2) and
        xRow.x3 == exprX3(lRow.lam, pRow.x1, pRow.x2) and
        yRow.y3 == exprY3(lRow.lam, pRow.x1, xRow.x3, pRow.y1) and
        sRow.v == 1 and sRow.id == r and sRow.x == xRow.x3 and sRow.y == yRow.y3 and sRow.flag == 0
      } else {
        not(anyChainValid) and sRow.v == 0
      }
    })
    prefixOk and chainOk
  }

  pure def holdsPinLimbsInRange(t: Trace): bool =
    ROWS.forall(r => {
      val pRow = t.pin[r]
      inField(pRow.x1) and inField(pRow.y1) and inField(pRow.x2) and inField(pRow.y2)
    })

  pure def holdsInvRow(t: Trace): bool =
    holdsRefinement(t) and holdsPinLimbsInRange(t)

  pure def holdsPerIdSoundness(t: Trace): bool =
    ROWS.forall(r => {
      val pRow = t.pin[r]
      val lRow = t.l_add[r]
      val iRow = t.inv_air[r]
      val xRow = t.x3_air[r]
      val yRow = t.y3_air[r]
      val sRow = t.sink[r]
      isBool(pRow.v) and
      if (pRow.v == 1) {
        lRow.is_valid == 1 and iRow.is_valid == 1 and xRow.is_valid == 1 and yRow.is_valid == 1 and
        lRow.x1 == pRow.x1 and lRow.y1 == pRow.y1 and lRow.x2 == pRow.x2 and lRow.y2 == pRow.y2 and
        xRow.x3 == exprX3(lRow.lam, pRow.x1, pRow.x2) and
        yRow.y3 == exprY3(lRow.lam, pRow.x1, xRow.x3, pRow.y1) and
        sRow.v == 1 and sRow.id == r and sRow.x == xRow.x3 and sRow.y == yRow.y3
      } else {
        lRow.is_valid == 0 and iRow.is_valid == 0 and xRow.is_valid == 0 and yRow.is_valid == 0 and
        sRow.v == 0
      }
    })

  // --------------------------------------------------------------------------
  // Attack / negative-test witness traces
  // --------------------------------------------------------------------------

  // 1. Hole trace (m_no_prefix attack & measured neg="pad_hole"):
  // Row 0 valid, row 1 padding (hole), row 2 valid, row 3 padding
  pure val holeTrace: Trace = {
    pin: List(
      honestPinRow(0, true),
      honestPinRow(1, false),
      honestPinRow(2, true),
      honestPinRow(3, false)
    ),
    l_add: List(
      honestLAddRow(0, true),
      honestLAddRow(1, false),
      honestLAddRow(2, true),
      honestLAddRow(3, false)
    ),
    inv_air: List(
      honestInvRow(0, true),
      honestInvRow(1, false),
      honestInvRow(2, true),
      honestInvRow(3, false)
    ),
    x3_air: List(
      honestX3Row(0, true),
      honestX3Row(1, false),
      honestX3Row(2, true),
      honestX3Row(3, false)
    ),
    y3_air: List(
      honestY3Row(0, true),
      honestY3Row(1, false),
      honestY3Row(2, true),
      honestY3Row(3, false)
    ),
    sink: List(
      honestSinkRow(0, true),
      honestSinkRow(1, false),
      honestSinkRow(2, true),
      honestSinkRow(3, false)
    ),
    perturb: noPerturb
  }

  // Measured neg="pad_forge": PointsIn has n=2 valid rows (rows 0..1),
  // but padding row 2 has its chain rows and sink row marked valid (is_valid = 1).
  // Passes all per-AIR constraints, fails ONLY LogUp balance on BUS_P1 and BUS_P2!
  pure val padForgeTrace: Trace = {
    val base = honestTrace(2)
    {
      pin: base.pin,
      l_add: base.l_add.replaceAt(2, honestLAddRow(2, true)),
      inv_air: base.inv_air.replaceAt(2, honestInvRow(2, true)),
      x3_air: base.x3_air.replaceAt(2, honestX3Row(2, true)),
      y3_air: base.y3_air.replaceAt(2, honestY3Row(2, true)),
      sink: base.sink.replaceAt(2, honestSinkRow(2, true)),
      perturb: noPerturb
    }
  }

  // 2. Non-boolean v trace (m_v_not_bool attack):
  // Row 0 has v = 2 (and e = 2), rows 1..3 have v = 0.
  // Transition next.v * (1 - v) = 0 * (1 - 2) = 0 still holds!
  pure val nonBoolVTrace: Trace = {
    val base = honestTrace(1)
    {
      pin: base.pin.replaceAt(0, pinWithVE(base.pin[0], 2, 2)),
      l_add: base.l_add.replaceAt(0, lAddWithValid(base.l_add[0], 2)),
      inv_air: base.inv_air.replaceAt(0, invWithValid(base.inv_air[0], 2)),
      x3_air: base.x3_air.replaceAt(0, x3WithValid(base.x3_air[0], 2)),
      y3_air: base.y3_air.replaceAt(0, y3WithValid(base.y3_air[0], 2)),
      sink: base.sink.replaceAt(0, sinkWithV(base.sink[0], 2)),
      perturb: noPerturb
    }
  }

  // 3. Padding rows send zero points with mult 1 (m_send_mult_one attack):
  // PointsIn has n = 2 valid rows (0, 1) and 2 padding rows (2, 3) with v = 0 and zero points (0, 0).
  // Because m_send_mult_one sends PointsIn messages with mult 1 instead of e, padding rows 2 and 3
  // send (r, 0, 0) on BUS_P1 and BUS_P2 with mult 1. An honest prover under m_send_mult_one
  // populates chain rows 2 and 3 with is_valid = 1 on inputs (0, 0, 0, 0) -> outputs (0, 0),
  // and SinkAir receives (r, 0, 0, 0) with v = 1 on rows 2 and 3.
  pure def zeroActiveLAddRow(r: int): LAddRow = {
    id: r, is_valid: 1, x1: 0, y1: 0, x2: 0, y2: 0, lam: 0
  }

  pure def zeroActiveInvRow(r: int): InvRow = {
    id: r, is_valid: 1, x1: 0, x2: 0, w: 0
  }

  pure def zeroActiveX3Row(r: int): X3Row = {
    id: r, is_valid: 1, lam: 0, x1: 0, x2: 0, x3: 0
  }

  pure def zeroActiveY3Row(r: int): Y3Row = {
    id: r, is_valid: 1, lam: 0, x1: 0, x3: 0, y1: 0, y3: 0
  }

  pure def zeroActiveSinkRow(r: int): SinkRow = {
    id: r, v: 1, flag: 0, x: 0, y: 0
  }

  pure val sendMultOnePadZeroTrace: Trace = {
    val base = honestTrace(2)
    {
      pin: base.pin,
      l_add: base.l_add.replaceAt(2, zeroActiveLAddRow(2)).replaceAt(3, zeroActiveLAddRow(3)),
      inv_air: base.inv_air.replaceAt(2, zeroActiveInvRow(2)).replaceAt(3, zeroActiveInvRow(3)),
      x3_air: base.x3_air.replaceAt(2, zeroActiveX3Row(2)).replaceAt(3, zeroActiveX3Row(3)),
      y3_air: base.y3_air.replaceAt(2, zeroActiveY3Row(2)).replaceAt(3, zeroActiveY3Row(3)),
      sink: base.sink.replaceAt(2, zeroActiveSinkRow(2)).replaceAt(3, zeroActiveSinkRow(3)),
      perturb: noPerturb
    }
  }

  // 4. Chain valid free (m_chain_valid_free attack):
  // Y3 uses multiplicity 1 in place of Y3.is_valid. On a full unit (n = 4), the prover sets
  // Y3.is_valid[0] = 0 and forges Y3.y3[0] = 0 (instead of canonY3(0) = 9), delivering (10, 0)
  // to SinkAir on row 0 while all buses balance with multiplicity 1.
  pure val forgeY3ViaFreeValidTrace: Trace = {
    val base = honestTrace(4)
    {
      pin: base.pin,
      l_add: base.l_add,
      inv_air: base.inv_air,
      x3_air: base.x3_air,
      y3_air: base.y3_air.replaceAt(0, y3WithValidAndOut(base.y3_air[0], 0, 0)),
      sink: base.sink.replaceAt(0, sinkWithY(base.sink[0], 0)),
      perturb: noPerturb
    }
  }

  // 5. Range check multiplicity v in place of 1 (m_range_mult_v attack):
  // On a partly filled unit (n = 2), padding row 2 has v = 0 and an out-of-range limb x1 = P (13).
  pure val unCheckedPadLimbTrace: Trace = {
    val base = honestTrace(2)
    {
      pin: base.pin.replaceAt(2, pinWithX1(base.pin[2], P)),
      l_add: base.l_add,
      inv_air: base.inv_air,
      x3_air: base.x3_air,
      y3_air: base.y3_air,
      sink: base.sink,
      perturb: noPerturb
    }
  }
}

// ============================================================================
// Sound Module: x14_pad
// ============================================================================
module x14_pad {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepMutateV = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    nondet r = ROWS.oneOf()
    nondet newV = Set(0, 1, 2).oneOf()
    nondet target = Set(0, 1, 2).oneOf()
    val base = honestTrace(n)
    val oldPin = base.pin[r]
    val newPinRow = pinWithVE(oldPin, newV, newV * oldPin.g)
    val oldSink = base.sink[r]
    val newSinkRow = sinkWithV(oldSink, newV)
    trace' = if (target == 0) {
      {
        pin: base.pin.replaceAt(r, newPinRow),
        l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (target == 1) {
      {
        pin: base.pin,
        l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink.replaceAt(r, newSinkRow), perturb: noPerturb
      }
    } else {
      {
        pin: base.pin.replaceAt(r, newPinRow),
        l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink.replaceAt(r, newSinkRow), perturb: noPerturb
      }
    }
  }

  action stepMutateChainValid = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    nondet r = ROWS.oneOf()
    nondet newValid = Set(0, 1, 2).oneOf()
    nondet whichChip = Set(0, 1, 2, 3, 4).oneOf()
    val base = honestTrace(n)
    val lRow = lAddWithValid(base.l_add[r], newValid)
    val iRow = invWithValid(base.inv_air[r], newValid)
    val xRow = x3WithValid(base.x3_air[r], newValid)
    val yRow = y3WithValid(base.y3_air[r], newValid)
    val sRow = sinkWithVXY(base.sink[r], newValid, canonX3(r), canonY3(r))
    trace' = if (whichChip == 0) {
      {
        pin: base.pin, l_add: base.l_add.replaceAt(r, lRow),
        inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichChip == 1) {
      {
        pin: base.pin, l_add: base.l_add,
        inv_air: base.inv_air.replaceAt(r, iRow), x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichChip == 2) {
      {
        pin: base.pin, l_add: base.l_add,
        inv_air: base.inv_air, x3_air: base.x3_air.replaceAt(r, xRow), y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichChip == 3) {
      {
        pin: base.pin, l_add: base.l_add,
        inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air.replaceAt(r, yRow),
        sink: base.sink, perturb: noPerturb
      }
    } else {
      {
        pin: base.pin,
        l_add: base.l_add.replaceAt(r, lRow),
        inv_air: base.inv_air.replaceAt(r, iRow),
        x3_air: base.x3_air.replaceAt(r, xRow),
        y3_air: base.y3_air.replaceAt(r, yRow),
        sink: base.sink.replaceAt(r, sRow),
        perturb: noPerturb
      }
    }
  }

  action stepMutateValue = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    nondet r = ROWS.oneOf()
    nondet newCell = Set(0, 1, 3, 12, P).oneOf()
    nondet whichVal = Set(0, 1, 2, 3, 4, 5).oneOf()
    val base = honestTrace(n)
    trace' = if (whichVal == 0) {
      {
        pin: base.pin.replaceAt(r, pinWithX1(base.pin[r], newCell)),
        l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichVal == 1) {
      {
        pin: base.pin,
        l_add: base.l_add.replaceAt(r, lAddWithLam(base.l_add[r], newCell)),
        inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichVal == 2) {
      {
        pin: base.pin, l_add: base.l_add,
        inv_air: base.inv_air.replaceAt(r, invWithW(base.inv_air[r], newCell)),
        x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink, perturb: noPerturb
      }
    } else if (whichVal == 3) {
      {
        pin: base.pin, l_add: base.l_add, inv_air: base.inv_air,
        x3_air: base.x3_air.replaceAt(r, x3WithOut(base.x3_air[r], newCell)),
        y3_air: base.y3_air, sink: base.sink, perturb: noPerturb
      }
    } else if (whichVal == 4) {
      {
        pin: base.pin, l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air,
        y3_air: base.y3_air.replaceAt(r, y3WithValidAndOut(base.y3_air[r], base.y3_air[r].is_valid, newCell)),
        sink: base.sink, perturb: noPerturb
      }
    } else {
      {
        pin: base.pin, l_add: base.l_add, inv_air: base.inv_air, x3_air: base.x3_air, y3_air: base.y3_air,
        sink: base.sink.replaceAt(r, sinkWithY(base.sink[r], newCell)),
        perturb: noPerturb
      }
    }
  }

  action stepDropOrDupMsg = {
    nondet n = Set(1, 2, 3, 4).oneOf()
    nondet r = ROWS.oneOf()
    nondet b = Set(
      BUS_P1, BUS_P2, BUS_LAM_X1_TO_X3, BUS_X2_TO_X3,
      BUS_LAM_X1_TO_Y3, BUS_Y1_TO_Y3, BUS_X3_TO_Y3, BUS_OUT, BUS_DENOM_TO_INV
    ).oneOf()
    nondet d = Set(-1, 1).oneOf()
    val base = honestTrace(n)
    trace' = {
      pin: base.pin, l_add: base.l_add, inv_air: base.inv_air,
      x3_air: base.x3_air, y3_air: base.y3_air, sink: base.sink,
      perturb: { active: true, bus: b, row_id: r, delta: d }
    }
  }

  action stepKnownNegatives = any {
    trace' = holeTrace,
    trace' = padForgeTrace,
    trace' = nonBoolVTrace,
    trace' = sendMultOnePadZeroTrace,
    trace' = forgeY3ViaFreeValidTrace,
    trace' = unCheckedPadLimbTrace
  }

  action step = any {
    stepHonest,
    stepMutateV,
    stepMutateChainValid,
    stepMutateValue,
    stepDropOrDupMsg,
    stepKnownNegatives
  }

  val accepted: bool = verifyTrace(trace, true, true, false, false, false)

  val invRow: bool = accepted implies holdsInvRow(trace)

  val invRefinement: bool = accepted implies holdsRefinement(trace)

  // Non-vacuity invariant: negated existence of an accepted padded trace with n = 2 valid rows.
  // Must be VIOLATED in `quint run` on the sound module.
  val invNonVacuousPadded: bool = not(accepted and countValidPin(trace) == 2)

  // Deterministic unit tests (also mirrored in x14_pad_test.qnt)
  run honestFullUnitTest = {
    val t = honestTrace(4)
    verifyTrace(t, true, true, false, false, false) and holdsInvRow(t) and countValidPin(t) == 4
  }

  run honestPartialUnitTest = {
    val t = honestTrace(2)
    verifyTrace(t, true, true, false, false, false) and holdsInvRow(t) and countValidPin(t) == 2
  }

  run honestEmptyUnitTest = {
    val t = honestTrace(0)
    verifyTrace(t, true, true, false, false, false) and holdsInvRow(t) and countValidPin(t) == 0
  }

  run measuredPadHoleTest = {
    val t = holeTrace
    checkLAdd(t.l_add, true) and checkInv(t.inv_air, true) and checkX3(t.x3_air, true) and
    checkY3(t.y3_air, true) and checkSink(t.sink, true) and checkRangeBus(t, false) and
    checkPermutationBuses(t, false, false) and not(checkPointsIn(t.pin, true, true)) and
    checkPointsIn(t.pin, false, true) and not(verifyTrace(t, true, true, false, false, false))
  }

  run measuredPadForgeTest = {
    val t = padForgeTrace
    checkPointsIn(t.pin, true, true) and checkLAdd(t.l_add, true) and checkInv(t.inv_air, true) and
    checkX3(t.x3_air, true) and checkY3(t.y3_air, true) and checkSink(t.sink, true) and
    checkRangeBus(t, false) and not(checkPermutationBuses(t, false, false)) and
    not(verifyTrace(t, true, true, false, false, false))
  }

  run mutantNoPrefixCaughtTest = {
    val t = holeTrace
    verifyTrace(t, false, true, false, false, false) and not(holdsInvRow(t)) and holdsPerIdSoundness(t)
  }

  run mutantVNotBoolCaughtTest = {
    val t = nonBoolVTrace
    verifyTrace(t, true, false, false, false, false) and not(holdsInvRow(t))
  }

  run mutantSendMultOneCaughtTest = {
    val t = sendMultOnePadZeroTrace
    verifyTrace(t, true, true, true, false, false) and not(holdsInvRow(t))
  }

  run mutantChainValidFreeCaughtTest = {
    val t = forgeY3ViaFreeValidTrace
    verifyTrace(t, true, true, false, true, false) and not(holdsInvRow(t))
  }

  run mutantRangeMultVCaughtTest = {
    val t = unCheckedPadLimbTrace
    verifyTrace(t, true, true, false, false, true) and not(holdsInvRow(t)) and holdsRefinement(t)
  }
}

// ============================================================================
// Mutant 1: m_no_prefix — drop transition constraint next.v * (1 - v) = 0
// ============================================================================
module m_no_prefix {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHoleAttack = {
    trace' = holeTrace
  }

  action step = any {
    stepHonest,
    stepHoleAttack
  }

  val accepted: bool = verifyTrace(trace, false, true, false, false, false)

  val invRow: bool = accepted implies holdsInvRow(trace)

  val invPerIdSound: bool = accepted implies holdsPerIdSoundness(trace)
}

// ============================================================================
// Mutant 2: m_v_not_bool — drop assert_bool(v)
// ============================================================================
module m_v_not_bool {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepNonBoolAttack = {
    trace' = nonBoolVTrace
  }

  action step = any {
    stepHonest,
    stepNonBoolAttack
  }

  val accepted: bool = verifyTrace(trace, true, false, false, false, false)

  val invRow: bool = accepted implies holdsInvRow(trace)
}

// ============================================================================
// Mutant 3: m_send_mult_one — point messages with multiplicity 1 in place of e
// ============================================================================
module m_send_mult_one {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepPadSendZeroPointsAttack = {
    trace' = sendMultOnePadZeroTrace
  }

  action step = any {
    stepHonest,
    stepPadSendZeroPointsAttack
  }

  val accepted: bool = verifyTrace(trace, true, true, true, false, false)

  val invRow: bool = accepted implies holdsInvRow(trace)
}

// ============================================================================
// Mutant 4: m_chain_valid_free — one chain AIR (Y3) uses mult 1 in place of is_valid
// ============================================================================
module m_chain_valid_free {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepForgeY3Attack = {
    trace' = forgeY3ViaFreeValidTrace
  }

  action step = any {
    stepHonest,
    stepForgeY3Attack
  }

  val accepted: bool = verifyTrace(trace, true, true, false, true, false)

  val invRow: bool = accepted implies holdsInvRow(trace)
}

// ============================================================================
// Mutant 5: m_range_mult_v — PointsIn range checks with multiplicity v in place of 1
// ============================================================================
module m_range_mult_v {
  import x14_pad_defs.*

  var trace: Trace

  action init = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepHonest = {
    nondet n = Set(0, 1, 2, 3, 4).oneOf()
    trace' = honestTrace(n)
  }

  action stepUnCheckedPadLimb = {
    trace' = unCheckedPadLimbTrace
  }

  action step = any {
    stepHonest,
    stepUnCheckedPadLimb
  }

  val accepted: bool = verifyTrace(trace, true, true, false, false, true)

  val invRow: bool = accepted implies holdsInvRow(trace)

  val invRefinement: bool = accepted implies holdsRefinement(trace)
}
