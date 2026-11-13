// -*- mode: Bluespec; -*-
// Soundness and completeness model of the X14 BLS12-381 G1 identity-point flag path
// (PointsIn + ChainAir + SinkAir + LogUp PermutationCheckBus).
//
// Small prime model: F_13 (P = 13), curve E(F_13) : y^2 = x^3 + 4 (mod 13),
// prime group order #E(F_13) = 19, explicit identity element PT_INF = (0, 0, is_inf = true).
// Note (0, 0) is not on y^2 = x^3 + 4 (0 != 4 mod 13), and no curve point has y = 0.

module x14_defs {

  pure val P: int = 13
  pure val ROW_IDS: List[int] = List(0, 1, 2, 3)
  pure val FIELD_ELEMS: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)

  type CurvePt = {
    x: int,
    y: int,
    is_inf: bool,
  }

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
    y2: int,
  }

  type SqRow = {
    id: int,
    is_valid: int,
    x: int,
    s: int,
  }

  type LRow = {
    id: int,
    is_valid: int,
    s_in: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    lam: int,
  }

  type InvRow = {
    id: int,
    is_valid: int,
    x1: int,
    x2: int,
    y1: int,
    w: int,
  }

  type X3Row = {
    id: int,
    is_valid: int,
    lam: int,
    x1: int,
    x2: int,
    x3: int,
  }

  type Y3Row = {
    id: int,
    is_valid: int,
    lam: int,
    x1: int,
    x3: int,
    y1: int,
    y3: int,
  }

  type SinkRow = {
    id: int,
    v: int,
    f: int,
    x: int,
    y: int,
  }

  type BusEntry = {
    bus: int,
    id: int,
    a: int,
    b: int,
    c: int,
    mult: int,
  }

  type RowBundle = {
    pin: PointsInRow,
    sq: SqRow,
    l: LRow,
    iv: InvRow,
    x3: X3Row,
    y3: Y3Row,
    sk: SinkRow,
  }

  // fault_kind: 0 = none, 1 = drop OUT message on row 0, 2 = duplicate OUT message on row 0
  type TraceState = {
    is_add: bool,
    pin: List[PointsInRow],
    sq_rows: List[SqRow],
    l_rows: List[LRow],
    inv_rows: List[InvRow],
    x3_rows: List[X3Row],
    y3_rows: List[Y3Row],
    sink_rows: List[SinkRow],
    fault_kind: int,
  }

  type MutantCfg = {
    no_zero_limbs: bool,
    e_ignores_flag: bool,
    sel_wrong: bool,
    out_flag_wrong: bool,
    out_mult_v: bool,
  }

  pure val SOUND_CFG: MutantCfg = {
    no_zero_limbs: false,
    e_ignores_flag: false,
    sel_wrong: false,
    out_flag_wrong: false,
    out_mult_v: false,
  }

  pure val PT_INF: CurvePt = { x: 0, y: 0, is_inf: true }
  // Multiples of generator G = (2, 5) on y^2 = x^3 + 4 (mod 13):
  // G = (2, 5), 2G = (12, 9), 3G = (8, 3), 4G = (6, 5)
  pure val PT_G1: CurvePt = { x: 2, y: 5, is_inf: false }
  pure val PT_G2: CurvePt = { x: 12, y: 9, is_inf: false }
  pure val PT_G3: CurvePt = { x: 8, y: 3, is_inf: false }
  pure val PT_G4: CurvePt = { x: 6, y: 5, is_inf: false }

  pure def fAdd(a: int, b: int): int = ((a + b) % P + P) % P
  pure def fSub(a: int, b: int): int = ((a - b) % P + P) % P
  pure def fMul(a: int, b: int): int = ((a * b) % P + P) % P

  pure def fInv(a: int): int = {
    val m = ((a % P) + P) % P
    if (m == 1) 1
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
    else if (m == 12) 12
    else 0
  }

  pure def isBit(b: int): bool = (b == 0 or b == 1)
  pure def inField(x: int): bool = (0 <= x and x < P)

  pure def onCurve(x: int, y: int): bool =
    inField(x) and inField(y) and (fMul(y, y) == fAdd(fMul(fMul(x, x), x), 4))

  pure def g1Dbl(p1: CurvePt): CurvePt =
    if (p1.is_inf or p1.y == 0) PT_INF
    else {
      val s = fMul(p1.x, p1.x)
      val lam = fMul(fMul(3, s), fInv(fMul(2, p1.y)))
      val x3 = fSub(fSub(fMul(lam, lam), p1.x), p1.x)
      val y3 = fSub(fMul(lam, fSub(p1.x, x3)), p1.y)
      { x: x3, y: y3, is_inf: false }
    }

  pure def g1Add(p1: CurvePt, p2: CurvePt): CurvePt =
    if (p1.is_inf) p2
    else if (p2.is_inf) p1
    else if (p1.x == p2.x and fAdd(p1.y, p2.y) == 0) PT_INF
    else if (p1.x == p2.x and p1.y == p2.y) g1Dbl(p1)
    else {
      val lam = fMul(fSub(p2.y, p1.y), fInv(fSub(p2.x, p1.x)))
      val x3 = fSub(fSub(fMul(lam, lam), p1.x), p2.x)
      val y3 = fSub(fMul(lam, fSub(p1.x, x3)), p1.y)
      { x: x3, y: y3, is_inf: false }
    }

  // Decode an input coordinate pair to a group element in G1:
  // (0, 0) is the canonical zero encoding of PT_INF; non-zero curve points are affine.
  pure def decodeInputPt(x: int, y: int): CurvePt =
    if (x == 0 and y == 0) PT_INF
    else { x: x, y: y, is_inf: false }

  // Producer binding precondition on PointsIn inputs for valid rows:
  // Either (x, y) is a valid affine point on E(F_13), or (x, y) == (0, 0) with flag f == 1.
  pure def producerInputOk(x: int, y: int, f: int): bool =
    if (x == 0 and y == 0) (f == 1)
    else onCurve(x, y)

  pure def producerValid(st: TraceState): bool =
    ROW_IDS.foldl(true, (ok, r) => {
      val row = st.pin[r]
      val p1_ok = if (row.v == 1) producerInputOk(row.x1, row.y1, row.f0) else true
      val p2_ok = if (row.v == 1 and st.is_add) producerInputOk(row.x2, row.y2, row.f1) else true
      ok and p1_ok and p2_ok
    })

  // PointsIn AIR constraints under MutantCfg
  pure def pointsInRowValid(cfg: MutantCfg, is_add: bool, r: int, row: PointsInRow): bool = {
    val id_ok = (row.id == r)
    val bits_ok = isBit(row.v) and isBit(row.f0) and (if (is_add) isBit(row.f1) else true)
    val g_ok = if (is_add) (row.g == (1 - row.f0) * (1 - row.f1)) else true
    val e_ok =
      if (cfg.e_ignores_flag) (row.e == row.v)
      else if (is_add) (row.e == row.v * row.g)
      else (row.e == row.v * (1 - row.f0))
    val zero_limbs_ok =
      if (cfg.no_zero_limbs) true
      else {
        val z0 = (row.f0 * row.x1 == 0) and (row.f0 * row.y1 == 0)
        val z1 = if (is_add) ((row.f1 * row.x2 == 0) and (row.f1 * row.y2 == 0)) else true
        z0 and z1
      }
    val range_ok =
      inField(row.x1) and inField(row.y1) and
      (if (is_add) (inField(row.x2) and inField(row.y2)) else true)
    id_ok and bits_ok and g_ok and e_ok and zero_limbs_ok and range_ok
  }

  pure def pointsInValid(cfg: MutantCfg, st: TraceState): bool = {
    val rows_ok = ROW_IDS.foldl(true, (ok, r) => ok and pointsInRowValid(cfg, st.is_add, r, st.pin[r]))
    // Transition constraint: next.v * (1 - v) == 0 (valid rows form a prefix)
    val prefix_ok =
      (st.pin[1].v * (1 - st.pin[0].v) == 0) and
      (st.pin[2].v * (1 - st.pin[1].v) == 0) and
      (st.pin[3].v * (1 - st.pin[2].v) == 0)
    rows_ok and prefix_ok
  }

  // ChainAir row constraints (SQ, L_add / L_dbl, INV / INV_y, X3, Y3)
  pure def sqRowValid(r: int, row: SqRow): bool =
    (row.id == r) and isBit(row.is_valid) and
    (if (row.is_valid == 1) (inField(row.x) and inField(row.s) and row.s == fMul(row.x, row.x)) else true)

  pure def lRowValid(is_add: bool, r: int, row: LRow): bool =
    (row.id == r) and isBit(row.is_valid) and
    (if (row.is_valid == 1) {
      if (is_add) {
        inField(row.x1) and inField(row.y1) and inField(row.x2) and inField(row.y2) and inField(row.lam) and
        (fMul(row.lam, fSub(row.x2, row.x1)) == fSub(row.y2, row.y1))
      } else {
        inField(row.s_in) and inField(row.x1) and inField(row.y1) and inField(row.lam) and
        (fMul(row.lam, fMul(2, row.y1)) == fMul(3, row.s_in))
      }
    } else true)

  pure def invRowValid(is_add: bool, r: int, row: InvRow): bool =
    (row.id == r) and isBit(row.is_valid) and
    (if (row.is_valid == 1) {
      if (is_add) {
        inField(row.x1) and inField(row.x2) and inField(row.w) and
        (fMul(row.w, fSub(row.x2, row.x1)) == 1)
      } else {
        inField(row.y1) and inField(row.w) and
        (fMul(row.w, row.y1) == 1)
      }
    } else true)

  pure def x3RowValid(r: int, row: X3Row): bool =
    (row.id == r) and isBit(row.is_valid) and
    (if (row.is_valid == 1) {
      inField(row.lam) and inField(row.x1) and inField(row.x2) and inField(row.x3) and
      (row.x3 == fSub(fSub(fMul(row.lam, row.lam), row.x1), row.x2))
    } else true)

  pure def y3RowValid(r: int, row: Y3Row): bool =
    (row.id == r) and isBit(row.is_valid) and
    (if (row.is_valid == 1) {
      inField(row.lam) and inField(row.x1) and inField(row.x3) and inField(row.y1) and inField(row.y3) and
      (row.y3 == fSub(fMul(row.lam, fSub(row.x1, row.x3)), row.y1))
    } else true)

  pure def sinkRowValid(row: SinkRow): bool =
    isBit(row.v) and inField(row.x) and inField(row.y)

  pure def chainValid(st: TraceState): bool =
    ROW_IDS.foldl(true, (ok, r) =>
      ok and
      (if (not(st.is_add)) sqRowValid(r, st.sq_rows[r]) else true) and
      lRowValid(st.is_add, r, st.l_rows[r]) and
      invRowValid(st.is_add, r, st.inv_rows[r]) and
      x3RowValid(r, st.x3_rows[r]) and
      y3RowValid(r, st.y3_rows[r]) and
      sinkRowValid(st.sink_rows[r])
    )

  // PointsIn OUT message on bus o+7 under MutantCfg
  pure def pointsInOutEntry(cfg: MutantCfg, is_add: bool, row: PointsInRow): BusEntry = {
    val sel_x =
      if (not(is_add)) row.x1
      else if (cfg.sel_wrong) row.x1
      else fAdd(row.x1, fMul(row.f0, fSub(row.x2, row.x1)))
    val sel_y =
      if (not(is_add)) row.y1
      else if (cfg.sel_wrong) row.y1
      else fAdd(row.y1, fMul(row.f0, fSub(row.y2, row.y1)))
    val out_f =
      if (not(is_add)) row.f0
      else if (cfg.out_flag_wrong) (row.f0 + row.f1)
      else (row.f0 * row.f1)
    val out_mult =
      if (cfg.out_mult_v) row.v
      else (row.v - row.e)
    { bus: 7, id: row.id, a: sel_x, b: sel_y, c: out_f, mult: out_mult }
  }

  // Collect all PermutationCheckBus interactions across PointsIn, ChainAirs, and SinkAir
  pure def allBusEntries(cfg: MutantCfg, st: TraceState): List[BusEntry] = {
    val base = ROW_IDS.foldl(List(), (acc, r) => {
      val p = st.pin[r]
      val sq = st.sq_rows[r]
      val l = st.l_rows[r]
      val iv = st.inv_rows[r]
      val x3 = st.x3_rows[r]
      val y3 = st.y3_rows[r]
      val sk = st.sink_rows[r]
      val pin_to_chain =
        if (st.is_add) List(
          { bus: 0, id: p.id, a: p.x1, b: p.y1, c: 0, mult: p.e },
          { bus: 1, id: p.id, a: p.x2, b: p.y2, c: 0, mult: p.e },
          { bus: 0, id: l.id, a: l.x1, b: l.y1, c: 0, mult: -l.is_valid },
          { bus: 1, id: l.id, a: l.x2, b: l.y2, c: 0, mult: -l.is_valid },
          { bus: 3, id: l.id, a: l.x2, b: 0, c: 0, mult: l.is_valid },
          { bus: 10, id: l.id, a: l.x1, b: l.x2, c: 0, mult: l.is_valid },
          { bus: 10, id: iv.id, a: iv.x1, b: iv.x2, c: 0, mult: -iv.is_valid }
        ) else List(
          { bus: 8, id: p.id, a: p.x1, b: 0, c: 0, mult: p.e },
          { bus: 8, id: sq.id, a: sq.x, b: 0, c: 0, mult: -sq.is_valid },
          { bus: 9, id: sq.id, a: sq.s, b: sq.x, c: 0, mult: sq.is_valid },
          { bus: 9, id: l.id, a: l.s_in, b: l.x1, c: 0, mult: -l.is_valid },
          { bus: 0, id: p.id, a: p.y1, b: 0, c: 0, mult: p.e },
          { bus: 0, id: l.id, a: l.y1, b: 0, c: 0, mult: -l.is_valid },
          { bus: 3, id: l.id, a: l.x1, b: 0, c: 0, mult: l.is_valid },
          { bus: 10, id: l.id, a: l.y1, b: 0, c: 0, mult: l.is_valid },
          { bus: 10, id: iv.id, a: iv.y1, b: 0, c: 0, mult: -iv.is_valid }
        )
      val common_chain = List(
        { bus: 2, id: l.id, a: l.lam, b: l.x1, c: 0, mult: l.is_valid },
        { bus: 2, id: x3.id, a: x3.lam, b: x3.x1, c: 0, mult: -x3.is_valid },
        { bus: 3, id: x3.id, a: x3.x2, b: 0, c: 0, mult: -x3.is_valid },
        { bus: 4, id: l.id, a: l.lam, b: l.x1, c: 0, mult: l.is_valid },
        { bus: 4, id: y3.id, a: y3.lam, b: y3.x1, c: 0, mult: -y3.is_valid },
        { bus: 5, id: l.id, a: l.y1, b: 0, c: 0, mult: l.is_valid },
        { bus: 5, id: y3.id, a: y3.y1, b: 0, c: 0, mult: -y3.is_valid },
        { bus: 6, id: x3.id, a: x3.x3, b: 0, c: 0, mult: x3.is_valid },
        { bus: 6, id: y3.id, a: y3.x3, b: 0, c: 0, mult: -y3.is_valid },
        pointsInOutEntry(cfg, st.is_add, p),
        { bus: 7, id: y3.id, a: y3.x3, b: y3.y3, c: 0, mult: y3.is_valid },
        { bus: 7, id: sk.id, a: sk.x, b: sk.y, c: sk.f, mult: -sk.v }
      )
      acc.concat(pin_to_chain).concat(common_chain)
    })
    if (st.fault_kind == 1) {
      val p0_out = pointsInOutEntry(cfg, st.is_add, st.pin[0])
      base.append({ bus: 7, id: p0_out.id, a: p0_out.a, b: p0_out.b, c: p0_out.c, mult: -1 })
    } else if (st.fault_kind == 2) {
      val p0_out = pointsInOutEntry(cfg, st.is_add, st.pin[0])
      base.append({ bus: 7, id: p0_out.id, a: p0_out.a, b: p0_out.b, c: p0_out.c, mult: 1 })
    } else {
      base
    }
  }

  pure def netCount(entries: List[BusEntry], target: BusEntry): int =
    entries.foldl(0, (acc, el) =>
      if (el.bus == target.bus and el.id == target.id and el.a == target.a and el.b == target.b and el.c == target.c)
        acc + el.mult
      else
        acc
    )

  pure def busBalanced(entries: List[BusEntry]): bool =
    entries.foldl(true, (ok, el) => ok and (netCount(entries, el) == 0))

  pure def isTraceAccepted(cfg: MutantCfg, st: TraceState): bool =
    producerValid(st) and
    pointsInValid(cfg, st) and
    chainValid(st) and
    busBalanced(allBusEntries(cfg, st))

  // Refinement property: for every valid id k, the consumer receives uniquely the group operation
  // of the inputs in G1 (with O), and no accepted trace outputs a value other than that.
  pure def rowRefined(st: TraceState, k: int): bool = {
    val p = st.pin[k]
    val recv_count = st.sink_rows.foldl(0, (cnt, s) => if (s.id == k) cnt + s.v else cnt)
    if (p.v == 0) {
      recv_count == 0
    } else {
      val in1 = decodeInputPt(p.x1, p.y1)
      val in2 = decodeInputPt(p.x2, p.y2)
      val expected = if (st.is_add) g1Add(in1, in2) else g1Dbl(in1)
      val expected_f = if (expected.is_inf) 1 else 0
      val all_match = st.sink_rows.foldl(true, (ok, s) =>
        if (s.v == 1 and s.id == k)
          ok and (s.x == expected.x) and (s.y == expected.y) and (s.f == expected_f)
        else
          ok
      )
      (recv_count == 1) and all_match
    }
  }

  pure def allRowsRefined(st: TraceState): bool =
    ROW_IDS.foldl(true, (ok, k) => ok and rowRefined(st, k))

  pure def assembleTrace(is_add: bool, r0: RowBundle, r1: RowBundle, r2: RowBundle, r3: RowBundle, flt: int): TraceState = {
    is_add: is_add,
    pin: List(r0.pin, r1.pin, r2.pin, r3.pin),
    sq_rows: List(r0.sq, r1.sq, r2.sq, r3.sq),
    l_rows: List(r0.l, r1.l, r2.l, r3.l),
    inv_rows: List(r0.iv, r1.iv, r2.iv, r3.iv),
    x3_rows: List(r0.x3, r1.x3, r2.x3, r3.x3),
    y3_rows: List(r0.y3, r1.y3, r2.y3, r3.y3),
    sink_rows: List(r0.sk, r1.sk, r2.sk, r3.sk),
    fault_kind: flt,
  }

  // Honest row builders under a given MutantCfg
  pure def buildHonestAddRow(cfg: MutantCfg, r: int, v: int, p1: CurvePt, p2: CurvePt): RowBundle = {
    val f0 = if (v == 1 and p1.is_inf) 1 else 0
    val f1 = if (v == 1 and p2.is_inf) 1 else 0
    val g = (1 - f0) * (1 - f1)
    val e = if (cfg.e_ignores_flag) v else v * g
    val px1 = if (v == 1 and not(p1.is_inf)) p1.x else 0
    val py1 = if (v == 1 and not(p1.is_inf)) p1.y else 0
    val px2 = if (v == 1 and not(p2.is_inf)) p2.x else 0
    val py2 = if (v == 1 and not(p2.is_inf)) p2.y else 0
    val pin_row: PointsInRow = { id: r, v: v, f0: f0, f1: f1, g: g, e: e, x1: px1, y1: py1, x2: px2, y2: py2 }
    // Dummy satisfying witness for padding/bypassed chain rows (is_valid = 0)
    val cx1 = if (e == 1) px1 else PT_G1.x
    val cy1 = if (e == 1) py1 else PT_G1.y
    val cx2 = if (e == 1) px2 else PT_G2.x
    val cy2 = if (e == 1) py2 else PT_G2.y
    val dx = fSub(cx2, cx1)
    val w = fInv(dx)
    val lam = fMul(fSub(cy2, cy1), w)
    val rx3 = fSub(fSub(fMul(lam, lam), cx1), cx2)
    val ry3 = fSub(fMul(lam, fSub(cx1, rx3)), cy1)
    val sq_row: SqRow = { id: r, is_valid: 0, x: 0, s: 0 }
    val l_row: LRow = { id: r, is_valid: e, s_in: 0, x1: cx1, y1: cy1, x2: cx2, y2: cy2, lam: lam }
    val iv_row: InvRow = { id: r, is_valid: e, x1: cx1, x2: cx2, y1: 0, w: w }
    val x3_row: X3Row = { id: r, is_valid: e, lam: lam, x1: cx1, x2: cx2, x3: rx3 }
    val y3_row: Y3Row = { id: r, is_valid: e, lam: lam, x1: cx1, x3: rx3, y1: cy1, y3: ry3 }
    val p_out = pointsInOutEntry(cfg, true, pin_row)
    val sk_row: SinkRow =
      if (v == 0) { id: r, v: 0, f: 0, x: 0, y: 0 }
      else if (e == 1) { id: r, v: 1, f: 0, x: rx3, y: ry3 }
      else { id: r, v: 1, f: p_out.c, x: p_out.a, y: p_out.b }
    { pin: pin_row, sq: sq_row, l: l_row, iv: iv_row, x3: x3_row, y3: y3_row, sk: sk_row }
  }

  pure def buildHonestDblRow(cfg: MutantCfg, r: int, v: int, p1: CurvePt): RowBundle = {
    val f0 = if (v == 1 and p1.is_inf) 1 else 0
    val e = if (cfg.e_ignores_flag) v else v * (1 - f0)
    val px1 = if (v == 1 and not(p1.is_inf)) p1.x else 0
    val py1 = if (v == 1 and not(p1.is_inf)) p1.y else 0
    val pin_row: PointsInRow = { id: r, v: v, f0: f0, f1: 0, g: 0, e: e, x1: px1, y1: py1, x2: 0, y2: 0 }
    val cx1 = if (e == 1) px1 else PT_G1.x
    val cy1 = if (e == 1) py1 else PT_G1.y
    val s = fMul(cx1, cx1)
    val w = fInv(cy1)
    val lam = fMul(fMul(3, s), fInv(fMul(2, cy1)))
    val rx3 = fSub(fSub(fMul(lam, lam), cx1), cx1)
    val ry3 = fSub(fMul(lam, fSub(cx1, rx3)), cy1)
    val sq_row: SqRow = { id: r, is_valid: e, x: cx1, s: s }
    val l_row: LRow = { id: r, is_valid: e, s_in: s, x1: cx1, y1: cy1, x2: 0, y2: 0, lam: lam }
    val iv_row: InvRow = { id: r, is_valid: e, x1: 0, x2: 0, y1: cy1, w: w }
    val x3_row: X3Row = { id: r, is_valid: e, lam: lam, x1: cx1, x2: cx1, x3: rx3 }
    val y3_row: Y3Row = { id: r, is_valid: e, lam: lam, x1: cx1, x3: rx3, y1: cy1, y3: ry3 }
    val p_out = pointsInOutEntry(cfg, false, pin_row)
    val sk_row: SinkRow =
      if (v == 0) { id: r, v: 0, f: 0, x: 0, y: 0 }
      else if (e == 1) { id: r, v: 1, f: 0, x: rx3, y: ry3 }
      else { id: r, v: 1, f: p_out.c, x: p_out.a, y: p_out.b }
    { pin: pin_row, sq: sq_row, l: l_row, iv: iv_row, x3: x3_row, y3: y3_row, sk: sk_row }
  }

  // Honest 4-row add trace: row 0 = O + G1, row 1 = G1 + O, row 2 = O + O, row 3 = G1 + G2
  pure def traceHonestAdd(cfg: MutantCfg): TraceState = {
    val r0 = buildHonestAddRow(cfg, 0, 1, PT_INF, PT_G1)
    val r1 = buildHonestAddRow(cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(cfg, 2, 1, PT_INF, PT_INF)
    val r3 = buildHonestAddRow(cfg, 3, 1, PT_G1, PT_G2)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Honest 4-row dbl trace: row 0 = 2 * O, row 1 = 2 * G1, row 2 = 2 * G2, row 3 = padding
  pure def traceHonestDbl(cfg: MutantCfg): TraceState = {
    val r0 = buildHonestDblRow(cfg, 0, 1, PT_INF)
    val r1 = buildHonestDblRow(cfg, 1, 1, PT_G1)
    val r2 = buildHonestDblRow(cfg, 2, 1, PT_G2)
    val r3 = buildHonestDblRow(cfg, 3, 0, PT_INF)
    assembleTrace(false, r0, r1, r2, r3, 0)
  }

  // Attack 1 (neg == "inf_lie" / m_no_zero_limbs):
  // Flag a real point P1 = PT_G1 = (2, 5) with f0 = 1 in row 0 (PT_G1 + PT_G2),
  // skipping P1 and outputting PT_G2 = (12, 9) instead of PT_G3 = (8, 3).
  pure def traceAttackFlagRealPoint(cfg: MutantCfg): TraceState = {
    val r0_base = buildHonestAddRow(cfg, 0, 1, PT_INF, PT_G2)
    val r0_pin: PointsInRow = { ...r0_base.pin, f0: 1, f1: 0, g: 0, e: 0, x1: PT_G1.x, y1: PT_G1.y, x2: PT_G2.x, y2: PT_G2.y }
    val r0_sk: SinkRow = { id: 0, v: 1, f: 0, x: PT_G2.x, y: PT_G2.y }
    val r0: RowBundle = { ...r0_base, pin: r0_pin, sk: r0_sk }
    val r1 = buildHonestAddRow(cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(cfg, 2, 1, PT_INF, PT_INF)
    val r3 = buildHonestAddRow(cfg, 3, 1, PT_G1, PT_G2)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Attack 2 (m_e_ignores_flag):
  // Row 0 is O + PT_G1 (P1 = (0, 0), f0 = 1; P2 = (2, 5), f1 = 0) and Row 1 is PT_G1 + O,
  // with e = v = 1 so the chain runs on (0, 0) and (2, 5): x2 - x1 = +-2 != 0 is invertible,
  // and the chain accepts the bogus secant point (1, 4, 0) instead of PT_G1 = (2, 5, 0).
  pure def traceAttackRunChainOnZero(cfg: MutantCfg): TraceState = {
    val eff_cfg: MutantCfg = { ...cfg, e_ignores_flag: true }
    val r0 = buildHonestAddRow(eff_cfg, 0, 1, PT_INF, PT_G1)
    val r1 = buildHonestAddRow(eff_cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(eff_cfg, 2, 1, PT_G1, PT_G2)
    val r3 = buildHonestAddRow(eff_cfg, 3, 0, PT_INF, PT_INF)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Attack 3 (m_sel_wrong):
  // Row 0 is O + PT_G1, PointsIn selects P1 = (0, 0) instead of P2 = (2, 5).
  pure def traceAttackSelP1(cfg: MutantCfg): TraceState = {
    val eff_cfg: MutantCfg = { ...cfg, sel_wrong: true }
    val r0 = buildHonestAddRow(eff_cfg, 0, 1, PT_INF, PT_G1)
    val r1 = buildHonestAddRow(eff_cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(eff_cfg, 2, 1, PT_INF, PT_INF)
    val r3 = buildHonestAddRow(eff_cfg, 3, 1, PT_G1, PT_G2)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Attack 4 (m_out_flag_wrong):
  // Row 0 is O + PT_G1, PointsIn outputs flag f0 + f1 = 1 instead of f0 * f1 = 0.
  pure def traceAttackOutFlagSum(cfg: MutantCfg): TraceState = {
    val eff_cfg: MutantCfg = { ...cfg, out_flag_wrong: true }
    val r0 = buildHonestAddRow(eff_cfg, 0, 1, PT_INF, PT_G1)
    val r1 = buildHonestAddRow(eff_cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(eff_cfg, 2, 1, PT_INF, PT_INF)
    val r3 = buildHonestAddRow(eff_cfg, 3, 1, PT_G1, PT_G2)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Attack 5 (m_out_mult_v):
  // Row 0 is PT_G1 + PT_G2 (v = 1, e = 1) and row 3 is padding (v = 0, e = 0).
  // Under out_mult_v, PointsIn sends (0, PT_G1, 0) with mult v = 1 AND Y3 sends (0, PT_G3, 0)
  // with mult 1. Sink row 0 receives (0, PT_G1, 0) and sink row 3 receives (0, PT_G3, 0).
  pure def traceAttackDoubleOutput(cfg: MutantCfg): TraceState = {
    val r0_base = buildHonestAddRow(cfg, 0, 1, PT_G1, PT_G2)
    val r1 = buildHonestAddRow(cfg, 1, 1, PT_INF, PT_G1)
    val r2 = buildHonestAddRow(cfg, 2, 1, PT_G1, PT_INF)
    val r3_base = buildHonestAddRow(cfg, 3, 0, PT_INF, PT_INF)
    val r0: RowBundle = { ...r0_base, sk: { id: 0, v: 1, f: 0, x: PT_G1.x, y: PT_G1.y } }
    val r3: RowBundle = { ...r3_base, sk: { id: 0, v: 1, f: 0, x: PT_G3.x, y: PT_G3.y } }
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Measured negative test (neg == "inf_out"):
  // Row 0 is O + PT_G1, PointsIn sends PT_G1 on o+7, but consumer (SinkAir) takes the dummy chain sum PT_G3.
  pure def traceNegSinkTakesChainSum(cfg: MutantCfg): TraceState = {
    val r0_base = buildHonestAddRow(cfg, 0, 1, PT_INF, PT_G1)
    val r0: RowBundle = { ...r0_base, sk: { id: 0, v: 1, f: 0, x: PT_G3.x, y: PT_G3.y } }
    val r1 = buildHonestAddRow(cfg, 1, 1, PT_G1, PT_INF)
    val r2 = buildHonestAddRow(cfg, 2, 1, PT_INF, PT_INF)
    val r3 = buildHonestAddRow(cfg, 3, 1, PT_G1, PT_G2)
    assembleTrace(true, r0, r1, r2, r3, 0)
  }

  // Nondeterministic prover trace construction: sets any flag, any limb, any multiplicity cell,
  // or drops/duplicates one message.
  pure def traceArbitraryProver(
    cfg: MutantCfg,
    op_add: bool,
    px1: int,
    py1: int,
    px2: int,
    py2: int,
    v_mut: int,
    f0_mut: int,
    f1_mut: int,
    g_mut: int,
    e_mut: int,
    chain_v: int,
    lam_mut: int,
    w_mut: int,
    sk_v: int,
    sk_x: int,
    sk_y: int,
    sk_f: int,
    flt: int
  ): TraceState = {
    if (op_add) {
      val base0 = buildHonestAddRow(cfg, 0, 1, PT_G1, PT_G2)
      val r1 = buildHonestAddRow(cfg, 1, 1, PT_G1, PT_INF)
      val r2 = buildHonestAddRow(cfg, 2, 1, PT_INF, PT_INF)
      val r3 = buildHonestAddRow(cfg, 3, 1, PT_G1, PT_G2)
      val rx3 = fSub(fSub(fMul(lam_mut, lam_mut), px1), px2)
      val ry3 = fSub(fMul(lam_mut, fSub(px1, rx3)), py1)
      val r0: RowBundle = {
        pin: { id: 0, v: v_mut, f0: f0_mut, f1: f1_mut, g: g_mut, e: e_mut, x1: px1, y1: py1, x2: px2, y2: py2 },
        sq: base0.sq,
        l: { id: 0, is_valid: chain_v, s_in: 0, x1: px1, y1: py1, x2: px2, y2: py2, lam: lam_mut },
        iv: { id: 0, is_valid: chain_v, x1: px1, x2: px2, y1: 0, w: w_mut },
        x3: { id: 0, is_valid: chain_v, lam: lam_mut, x1: px1, x2: px2, x3: rx3 },
        y3: { id: 0, is_valid: chain_v, lam: lam_mut, x1: px1, x3: rx3, y1: py1, y3: ry3 },
        sk: { id: 0, v: sk_v, f: sk_f, x: sk_x, y: sk_y },
      }
      assembleTrace(true, r0, r1, r2, r3, flt)
    } else {
      val r1 = buildHonestDblRow(cfg, 1, 1, PT_G1)
      val r2 = buildHonestDblRow(cfg, 2, 1, PT_G2)
      val r3 = buildHonestDblRow(cfg, 3, 0, PT_INF)
      val s_val = fMul(px1, px1)
      val rx3 = fSub(fSub(fMul(lam_mut, lam_mut), px1), px1)
      val ry3 = fSub(fMul(lam_mut, fSub(px1, rx3)), py1)
      val r0: RowBundle = {
        pin: { id: 0, v: v_mut, f0: f0_mut, f1: 0, g: 0, e: e_mut, x1: px1, y1: py1, x2: 0, y2: 0 },
        sq: { id: 0, is_valid: chain_v, x: px1, s: s_val },
        l: { id: 0, is_valid: chain_v, s_in: s_val, x1: px1, y1: py1, x2: 0, y2: 0, lam: lam_mut },
        iv: { id: 0, is_valid: chain_v, x1: 0, x2: 0, y1: py1, w: w_mut },
        x3: { id: 0, is_valid: chain_v, lam: lam_mut, x1: px1, x2: px1, x3: rx3 },
        y3: { id: 0, is_valid: chain_v, lam: lam_mut, x1: px1, x3: rx3, y1: py1, y3: ry3 },
        sk: { id: 0, v: sk_v, f: sk_f, x: sk_x, y: sk_y },
      }
      assembleTrace(false, r0, r1, r2, r3, flt)
    }
  }
}

// Sound module (default main module for x14_identity.qnt)
module x14_identity {
  import x14_defs.*

  var st: TraceState

  action stepProver(cfg: MutantCfg): bool = {
    nondet op_add = oneOf(Set(true, false))
    nondet px1 = oneOf(FIELD_ELEMS)
    nondet py1 = oneOf(FIELD_ELEMS)
    nondet px2 = oneOf(FIELD_ELEMS)
    nondet py2 = oneOf(FIELD_ELEMS)
    nondet v_mut = oneOf(Set(0, 1))
    nondet f0_mut = oneOf(Set(0, 1))
    nondet f1_mut = oneOf(Set(0, 1))
    nondet g_mut = oneOf(Set(0, 1))
    nondet e_mut = oneOf(Set(0, 1))
    nondet chain_v = oneOf(Set(0, 1))
    nondet lam_mut = oneOf(FIELD_ELEMS)
    nondet w_mut = oneOf(FIELD_ELEMS)
    nondet sk_v = oneOf(Set(0, 1))
    nondet sk_x = oneOf(FIELD_ELEMS)
    nondet sk_y = oneOf(FIELD_ELEMS)
    nondet sk_f = oneOf(Set(0, 1, 2))
    nondet flt = oneOf(Set(0, 1, 2))
    st' = traceArbitraryProver(
      cfg, op_add, px1, py1, px2, py2, v_mut, f0_mut, f1_mut, g_mut, e_mut,
      chain_v, lam_mut, w_mut, sk_v, sk_x, sk_y, sk_f, flt
    )
  }

  action init: bool = {
    st' = traceHonestAdd(SOUND_CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(SOUND_CFG),
    st' = traceHonestDbl(SOUND_CFG),
    st' = traceAttackFlagRealPoint(SOUND_CFG),
    st' = traceAttackRunChainOnZero(SOUND_CFG),
    st' = traceAttackSelP1(SOUND_CFG),
    st' = traceAttackOutFlagSum(SOUND_CFG),
    st' = traceAttackDoubleOutput(SOUND_CFG),
    st' = traceNegSinkTakesChainSum(SOUND_CFG),
    stepProver(SOUND_CFG),
  }

  val traceAccepted: bool = isTraceAccepted(SOUND_CFG, st)

  // Soundness refinement invariant: every accepted trace refines the G1 group operation with O.
  val invRow: bool =
    traceAccepted implies allRowsRefined(st)

  // Non-vacuity (negated reachability invariant): must be VIOLATED in the sound module,
  // witnessing an accepted honest trace that exercises O + P, P + O, O + O, and P1 + P2.
  val invNonVacuous: bool = not(
    traceAccepted and st.is_add and
    (st.pin[0].v == 1 and st.pin[0].f0 == 1 and st.pin[0].f1 == 0 and st.pin[0].e == 0) and
    (st.pin[1].v == 1 and st.pin[1].f0 == 0 and st.pin[1].f1 == 1 and st.pin[1].e == 0) and
    (st.pin[2].v == 1 and st.pin[2].f0 == 1 and st.pin[2].f1 == 1 and st.pin[2].e == 0) and
    (st.pin[3].v == 1 and st.pin[3].f0 == 0 and st.pin[3].f1 == 0 and st.pin[3].e == 1)
  )
}

// Mutant 1: drop f_i * limb = 0 in PointsIn
module m_no_zero_limbs {
  import x14_defs.*

  pure val CFG: MutantCfg = { ...SOUND_CFG, no_zero_limbs: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(CFG),
    st' = traceHonestDbl(CFG),
    st' = traceAttackFlagRealPoint(CFG),
  }

  val invRow: bool =
    isTraceAccepted(CFG, st) implies allRowsRefined(st)
}

// Mutant 2: e = v (chain runs on flagged points with zero limbs)
module m_e_ignores_flag {
  import x14_defs.*

  pure val CFG: MutantCfg = { ...SOUND_CFG, e_ignores_flag: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(CFG),
    st' = traceHonestDbl(CFG),
    st' = traceAttackRunChainOnZero(CFG),
  }

  val invRow: bool =
    isTraceAccepted(CFG, st) implies allRowsRefined(st)
}

// Mutant 3: sel = P1 always
module m_sel_wrong {
  import x14_defs.*

  pure val CFG: MutantCfg = { ...SOUND_CFG, sel_wrong: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(CFG),
    st' = traceHonestDbl(CFG),
    st' = traceAttackSelP1(CFG),
  }

  val invRow: bool =
    isTraceAccepted(CFG, st) implies allRowsRefined(st)
}

// Mutant 4: output flag f_0 + f_1 in place of f_0 * f_1
module m_out_flag_wrong {
  import x14_defs.*

  pure val CFG: MutantCfg = { ...SOUND_CFG, out_flag_wrong: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(CFG),
    st' = traceHonestDbl(CFG),
    st' = traceAttackOutFlagSum(CFG),
  }

  val invRow: bool =
    isTraceAccepted(CFG, st) implies allRowsRefined(st)
}

// Mutant 5: PointsIn OUT send with multiplicity v in place of v - e
module m_out_mult_v {
  import x14_defs.*

  pure val CFG: MutantCfg = { ...SOUND_CFG, out_mult_v: true }

  var st: TraceState

  action init: bool = {
    st' = traceHonestAdd(CFG)
  }

  action step: bool = any {
    st' = traceHonestAdd(CFG),
    st' = traceHonestDbl(CFG),
    st' = traceAttackDoubleOutput(CFG),
  }

  val invRow: bool =
    isTraceAccepted(CFG, st) implies allRowsRefined(st)
}
