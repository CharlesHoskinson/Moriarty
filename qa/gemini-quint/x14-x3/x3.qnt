// Soundness and completeness model of the X3 AIR in the BLS12-381 G1 chain (Sherlock Experiment X14).
// Source: lane-src/x_exp_node.rs (ChainAir, SourceAir, g1_chain),
// lane-src/builder.rs (FieldExpr, ExprBuilder), lane-src/field_variable.rs (FieldVariable),
// and lane-src/interaction_mod.rs (PermutationCheckBus).

module x3_base {
  const NO_RECV_X2: bool     // Mutant no_recv_x2: drop receive on bus o+3 (x2 free)
  const SIGN_FLIP: bool      // Mutant sign_flip: constraint x3 = lambda^2 - x1 + x2 (mod p)
  const NO_ID_SEND: bool     // Mutant no_id_send: drop id from send on bus o+6
  const RECV_SWAPPED: bool   // Mutant recv_swapped: receive (id, x1, lambda) on bus o+2
  const VALID_FREE: bool     // Mutant valid_free: multiplicity 1 in place of is_valid on send

  // Small prime field F_13 standing in for the BLS12-381 base field F_p.
  // Curve E(F_13): y^2 = x^3 + 4 (mod 13).
  pure val P = 13
  pure val BUS_ADD = 1100
  pure val BUS_DBL = 1110
  pure val NROWS = 2

  type Pt = {
    x: int,
    y: int,
  }

  type SrcRow = {
    isActive: int,
    rowId: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    x3: int,
    y3: int,
  }

  type LRow = {
    isValid: int,
    rowId: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    lam: int,
  }

  type X3Row = {
    isValid: int,
    rowId: int,
    lam: int,
    x1: int,
    x2: int,
    x3: int,
  }

  type Y3Row = {
    isValid: int,
    rowId: int,
    lam: int,
    x1: int,
    x3: int,
    y1: int,
    y3: int,
  }

  type BusMsg = {
    busIdx: int,
    payload: List[int],
    mult: int,
  }

  type ChainBundle = {
    src: SrcRow,
    l: LRow,
    x: X3Row,
    y: Y3Row,
  }

  type DownstreamRows = {
    srcRows: List[SrcRow],
    yRows: List[Y3Row],
  }

  type ChainState = {
    isDouble: bool,
    busBase: int,
    srcRows: List[SrcRow],
    lRows: List[LRow],
    xRows: List[X3Row],
    yRows: List[Y3Row],
    extraMsg: BusMsg,
    accepted: bool,
  }

  var st: ChainState

  // ---- Field arithmetic over F_13.
  pure def modP(a: int): int = {
    val r = a % P
    if (r < 0) r + P else r
  }

  pure def addFp(a: int, b: int): int = modP(a + b)
  pure def subFp(a: int, b: int): int = modP(a - b)
  pure def mulFp(a: int, b: int): int = modP(a * b)

  // Fermat inverse a^(P-2) = a^11 mod 13 (returns 0 for 0).
  pure def invFp(a: int): int = {
    val u = modP(a)
    val u2 = modP(u * u)
    val u4 = modP(u2 * u2)
    val u8 = modP(u4 * u4)
    modP(modP(u8 * u2) * u)
  }

  pure def isBit(b: int): bool = (b == 0 or b == 1)
  pure def inFp(v: int): bool = (v >= 0 and v < P)

  // ---- Valid affine points on E(F_13): y^2 = x^3 + 4 (mod 13).
  // PT0 = (2, 5), PT1 = [2]PT0 = (12, 9), PT2 = PT0 + PT1 = (8, 3), PT3 = [2]PT1 = (6, 5).
  pure val PT0: Pt = { x: 2, y: 5 }
  pure val PT1: Pt = { x: 12, y: 9 }
  pure val PT2: Pt = { x: 8, y: 3 }
  pure val PT3: Pt = { x: 6, y: 5 }

  pure def onCurve(x: int, y: int): bool =
    modP(y * y) == modP(x * x * x + 4)

  pure def affineAddSlope(x1: int, y1: int, x2: int, y2: int): int =
    mulFp(subFp(y2, y1), invFp(subFp(x2, x1)))

  pure def affineDblSlope(x1: int, y1: int): int =
    mulFp(mulFp(3, mulFp(x1, x1)), invFp(mulFp(2, y1)))

  pure def affineAdd(x1: int, y1: int, x2: int, y2: int): Pt = {
    val lam = affineAddSlope(x1, y1, x2, y2)
    val rx3 = subFp(subFp(mulFp(lam, lam), x1), x2)
    val ry3 = subFp(mulFp(lam, subFp(x1, rx3)), y1)
    { x: rx3, y: ry3 }
  }

  pure def affineDbl(x1: int, y1: int): Pt = {
    val lam = affineDblSlope(x1, y1)
    val rx3 = subFp(subFp(mulFp(lam, lam), x1), x1)
    val ry3 = subFp(mulFp(lam, subFp(x1, rx3)), y1)
    { x: rx3, y: ry3 }
  }

  // ---- List replacement helpers for 2-row traces.
  pure def replaceSrcRow(rows: List[SrcRow], idx: int, newRow: SrcRow): List[SrcRow] =
    if (idx == 0) [newRow, rows[1]] else [rows[0], newRow]

  pure def replaceLRow(rows: List[LRow], idx: int, newRow: LRow): List[LRow] =
    if (idx == 0) [newRow, rows[1]] else [rows[0], newRow]

  pure def replaceX3Row(rows: List[X3Row], idx: int, newRow: X3Row): List[X3Row] =
    if (idx == 0) [newRow, rows[1]] else [rows[0], newRow]

  pure def replaceY3Row(rows: List[Y3Row], idx: int, newRow: Y3Row): List[Y3Row] =
    if (idx == 0) [newRow, rows[1]] else [rows[0], newRow]

  // ---- Per-row FieldExpr constraints (lane-src/x_exp_node.rs:763-797, builder.rs:351-449).
  // L_add: (x2 - x1) * lam - (y2 - y1) == 0 (mod p)
  // L_dbl: (2 * y1) * lam - 3 * x1^2 == 0 (mod p)
  pure def cLRow(rl: LRow, isDouble: bool): bool = and {
    isBit(rl.isValid),
    (rl.isValid == 1) implies and {
      inFp(rl.lam),
      if (isDouble)
        modP(2 * rl.y1 * rl.lam - 3 * rl.x1 * rl.x1) == 0
      else
        modP((rl.x2 - rl.x1) * rl.lam - (rl.y2 - rl.y1)) == 0
    }
  }

  // X3: x3 == lambda^2 - x1 - x2 (mod p), or + x2 in SIGN_FLIP.
  pure def expectedX3(lam: int, x1: int, x2: int): int =
    if (SIGN_FLIP) modP(lam * lam - x1 + x2)
    else modP(lam * lam - x1 - x2)

  pure def cX3Row(rx: X3Row): bool = and {
    isBit(rx.isValid),
    (rx.isValid == 1) implies and {
      inFp(rx.x3),
      rx.x3 == expectedX3(rx.lam, rx.x1, rx.x2),
    }
  }

  // Y3: y3 == lambda * (x1 - x3) - y1 (mod p).
  pure def cY3Row(ry: Y3Row): bool = and {
    isBit(ry.isValid),
    (ry.isValid == 1) implies and {
      inFp(ry.y3),
      ry.y3 == modP(ry.lam * (ry.x1 - ry.x3) - ry.y1),
    }
  }

  // SourceAir: canonical limb-range-checked producer inputs.
  pure def cSrcRow(rs: SrcRow): bool = and {
    isBit(rs.isActive),
    (rs.isActive == 1) implies and {
      inFp(rs.x1),
      inFp(rs.y1),
      inFp(rs.x2),
      inFp(rs.y2),
      inFp(rs.x3),
      inFp(rs.y3),
    }
  }

  // ---- PermutationCheckBus multiset construction (lane-src/x_exp_node.rs:800-821).
  // Buses relative to busBase o (1100 for add, 1110 for double):
  //   o+0: Source -> L     (id, x1, y1)
  //   o+1: Source -> L_add (id, x2, y2) [add only]
  //   o+2: L -> X3         (id, lambda, x1) [or (id, x1, lambda) in RECV_SWAPPED]
  //   o+3: L -> X3         (id, x2)         [dropped in NO_RECV_X2]
  //   o+4: L -> Y3         (id, lambda, x1)
  //   o+5: L -> Y3         (id, y1)
  //   o+6: X3 -> Y3        (id, x3)         [or (x3) without id in NO_ID_SEND]
  //   o+7: Y3 -> Source    (id, x3, y3)
  pure def srcRowMsgs(o: int, isDouble: bool, rs: SrcRow): List[BusMsg] = {
    val m0: BusMsg = { busIdx: o + 0, payload: [rs.rowId, rs.x1, rs.y1], mult: rs.isActive }
    val m1: BusMsg = { busIdx: o + 1, payload: [rs.rowId, rs.x2, rs.y2], mult: rs.isActive }
    val m7: BusMsg = { busIdx: o + 7, payload: [rs.rowId, rs.x3, rs.y3], mult: -rs.isActive }
    if (isDouble) [m0, m7] else [m0, m1, m7]
  }

  pure def lRowMsgs(o: int, isDouble: bool, rl: LRow): List[BusMsg] = {
    val m0: BusMsg = { busIdx: o + 0, payload: [rl.rowId, rl.x1, rl.y1], mult: -rl.isValid }
    val m1: BusMsg = { busIdx: o + 1, payload: [rl.rowId, rl.x2, rl.y2], mult: -rl.isValid }
    val m2: BusMsg = { busIdx: o + 2, payload: [rl.rowId, rl.lam, rl.x1], mult: rl.isValid }
    val x2Sent = if (isDouble) rl.x1 else rl.x2
    val m3: BusMsg = { busIdx: o + 3, payload: [rl.rowId, x2Sent], mult: rl.isValid }
    val m4: BusMsg = { busIdx: o + 4, payload: [rl.rowId, rl.lam, rl.x1], mult: rl.isValid }
    val m5: BusMsg = { busIdx: o + 5, payload: [rl.rowId, rl.y1], mult: rl.isValid }
    val base = if (isDouble) [m0, m2, m4, m5] else [m0, m1, m2, m4, m5]
    if (NO_RECV_X2) base else base.append(m3)
  }

  pure def x3RowMsgs(o: int, rx: X3Row): List[BusMsg] = {
    val recvMult = if (VALID_FREE) 1 else rx.isValid
    val sendMult = if (VALID_FREE) 1 else rx.isValid
    val pay2 = if (RECV_SWAPPED) [rx.rowId, rx.x1, rx.lam] else [rx.rowId, rx.lam, rx.x1]
    val pay6 = if (NO_ID_SEND) [rx.x3] else [rx.rowId, rx.x3]
    val m2: BusMsg = { busIdx: o + 2, payload: pay2, mult: -recvMult }
    val m3: BusMsg = { busIdx: o + 3, payload: [rx.rowId, rx.x2], mult: -recvMult }
    val m6: BusMsg = { busIdx: o + 6, payload: pay6, mult: sendMult }
    if (NO_RECV_X2) [m2, m6] else [m2, m3, m6]
  }

  pure def y3RowMsgs(o: int, ry: Y3Row): List[BusMsg] = {
    val pay6 = if (NO_ID_SEND) [ry.x3] else [ry.rowId, ry.x3]
    val m4: BusMsg = { busIdx: o + 4, payload: [ry.rowId, ry.lam, ry.x1], mult: -ry.isValid }
    val m5: BusMsg = { busIdx: o + 5, payload: [ry.rowId, ry.y1], mult: -ry.isValid }
    val m6: BusMsg = { busIdx: o + 6, payload: pay6, mult: -ry.isValid }
    val m7: BusMsg = { busIdx: o + 7, payload: [ry.rowId, ry.x3, ry.y3], mult: ry.isValid }
    [m4, m5, m6, m7]
  }

  pure def allBusMsgs(
    o: int,
    isDouble: bool,
    srcRows: List[SrcRow],
    lRows: List[LRow],
    xRows: List[X3Row],
    yRows: List[Y3Row],
    extraMsg: BusMsg
  ): List[BusMsg] = {
    val emptyMsgs: List[BusMsg] = []
    val sMsgs = srcRows.foldl(emptyMsgs, (acc, rs) => acc.concat(srcRowMsgs(o, isDouble, rs)))
    val lMsgs = lRows.foldl(sMsgs, (acc, rl) => acc.concat(lRowMsgs(o, isDouble, rl)))
    val xMsgs = xRows.foldl(lMsgs, (acc, rx) => acc.concat(x3RowMsgs(o, rx)))
    val yMsgs = yRows.foldl(xMsgs, (acc, ry) => acc.concat(y3RowMsgs(o, ry)))
    if (extraMsg.mult == 0) yMsgs else yMsgs.append(extraMsg)
  }

  pure def netMult(msgs: List[BusMsg], bIdx: int, pay: List[int]): int =
    msgs.foldl(0, (acc, m) =>
      if (m.busIdx == bIdx and m.payload == pay) acc + m.mult else acc
    )

  pure def busBalanced(msgs: List[BusMsg]): bool =
    msgs.foldl(true, (acc, m) => acc and (netMult(msgs, m.busIdx, m.payload) == 0))

  pure def chainAccepts(
    o: int,
    isDouble: bool,
    srcRows: List[SrcRow],
    lRows: List[LRow],
    xRows: List[X3Row],
    yRows: List[Y3Row],
    extraMsg: BusMsg
  ): bool = {
    val msgs = allBusMsgs(o, isDouble, srcRows, lRows, xRows, yRows, extraMsg)
    and {
      srcRows.length() == NROWS,
      lRows.length() == NROWS,
      xRows.length() == NROWS,
      yRows.length() == NROWS,
      (isDouble and o == BUS_DBL) or (not(isDouble) and o == BUS_ADD),
      srcRows.foldl(true, (acc, rs) => acc and cSrcRow(rs)),
      lRows.foldl(true, (acc, rl) => acc and cLRow(rl, isDouble)),
      xRows.foldl(true, (acc, rx) => acc and cX3Row(rx)),
      yRows.foldl(true, (acc, ry) => acc and cY3Row(ry)),
      busBalanced(msgs),
    }
  }

  // ---- Honest trace constructors.
  pure val NO_EXTRA_MSG: BusMsg = { busIdx: 0, payload: [0], mult: 0 }

  pure def mkHonestAddBundle(rowId: int, p1: Pt, p2: Pt): ChainBundle = {
    val lam = affineAddSlope(p1.x, p1.y, p2.x, p2.y)
    val rx3 = subFp(subFp(mulFp(lam, lam), p1.x), p2.x)
    val ry3 = subFp(mulFp(lam, subFp(p1.x, rx3)), p1.y)
    {
      src: { isActive: 1, rowId: rowId, x1: p1.x, y1: p1.y, x2: p2.x, y2: p2.y, x3: rx3, y3: ry3 },
      l: { isValid: 1, rowId: rowId, x1: p1.x, y1: p1.y, x2: p2.x, y2: p2.y, lam: lam },
      x: { isValid: 1, rowId: rowId, lam: lam, x1: p1.x, x2: p2.x, x3: rx3 },
      y: { isValid: 1, rowId: rowId, lam: lam, x1: p1.x, x3: rx3, y1: p1.y, y3: ry3 },
    }
  }

  pure def mkHonestDblBundle(rowId: int, p1: Pt): ChainBundle = {
    val lam = affineDblSlope(p1.x, p1.y)
    val rx3 = subFp(subFp(mulFp(lam, lam), p1.x), p1.x)
    val ry3 = subFp(mulFp(lam, subFp(p1.x, rx3)), p1.y)
    {
      src: { isActive: 1, rowId: rowId, x1: p1.x, y1: p1.y, x2: p1.x, y2: p1.y, x3: rx3, y3: ry3 },
      l: { isValid: 1, rowId: rowId, x1: p1.x, y1: p1.y, x2: p1.x, y2: p1.y, lam: lam },
      x: { isValid: 1, rowId: rowId, lam: lam, x1: p1.x, x2: p1.x, x3: rx3 },
      y: { isValid: 1, rowId: rowId, lam: lam, x1: p1.x, x3: rx3, y1: p1.y, y3: ry3 },
    }
  }

  pure def baseBundles(isDouble: bool): List[ChainBundle] =
    if (isDouble)
      [mkHonestDblBundle(1, PT0), mkHonestDblBundle(2, PT1)]
    else
      [mkHonestAddBundle(1, PT0, PT1), mkHonestAddBundle(2, PT2, PT3)]

  // Given a new x3 output from X3 on row r, recompute Y3 and SourceAir row r honestly.
  pure def recomputeDownstream(
    srcRows: List[SrcRow],
    yRows: List[Y3Row],
    r: int,
    newX3: int
  ): DownstreamRows = {
    val oldY = yRows[r]
    val newY3 = subFp(mulFp(oldY.lam, subFp(oldY.x1, newX3)), oldY.y1)
    val updY: Y3Row = { ...oldY, x3: newX3, y3: newY3 }
    val oldS = srcRows[r]
    val updS: SrcRow = { ...oldS, x3: newX3, y3: newY3 }
    {
      srcRows: replaceSrcRow(srcRows, r, updS),
      yRows: replaceY3Row(yRows, r, updY),
    }
  }

  action buildState(
    isDouble: bool,
    srcRows: List[SrcRow],
    lRows: List[LRow],
    xRows: List[X3Row],
    yRows: List[Y3Row],
    extraMsg: BusMsg
  ): bool = {
    val o = if (isDouble) BUS_DBL else BUS_ADD
    st' = {
      isDouble: isDouble,
      busBase: o,
      srcRows: srcRows,
      lRows: lRows,
      xRows: xRows,
      yRows: yRows,
      extraMsg: extraMsg,
      accepted: chainAccepts(o, isDouble, srcRows, lRows, xRows, yRows, extraMsg),
    }
  }

  // ---- Honest prover actions (add and double chains, with optional physical row permutation in X3).
  action proveHonestAdd(permuteX3Rows: bool): bool = {
    val bs = baseBundles(false)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = if (permuteX3Rows) [bs[1].x, bs[0].x] else [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    buildState(false, sRows, lRows, xRows, yRows, NO_EXTRA_MSG)
  }

  action proveHonestDbl(permuteX3Rows: bool): bool = {
    val bs = baseBundles(true)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = if (permuteX3Rows) [bs[1].x, bs[0].x] else [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    buildState(true, sRows, lRows, xRows, yRows, NO_EXTRA_MSG)
  }

  // ---- Mutant-targeting and general adversarial prover actions (other AIRs computed honestly).

  // 1. Target m_no_recv_x2: tamper X3[r].x2 by delta != 0, compute X3[r].x3, Y3[r], Source[r] honestly.
  action proveAttackNoRecvX2(r: int, deltaX2: int, isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val oldX = xRows[r]
    val newX2 = addFp(oldX.x2, deltaX2)
    val newX3 = expectedX3(oldX.lam, oldX.x1, newX2)
    val updX: X3Row = { ...oldX, x2: newX2, x3: newX3 }
    val down = recomputeDownstream(sRows, yRows, r, newX3)
    buildState(isDouble, down.srcRows, lRows, replaceX3Row(xRows, r, updX), down.yRows, NO_EXTRA_MSG)
  }

  // 2. Target m_sign_flip: compute x3 = (lambda^2 - x1 + x2) mod p on X3 rows, with Y3 and Source computed honestly.
  action proveAttackSignFlip(isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val oldX0 = xRows[0]
    val oldX1 = xRows[1]
    val flipX3_0 = modP(oldX0.lam * oldX0.lam - oldX0.x1 + oldX0.x2)
    val flipX3_1 = modP(oldX1.lam * oldX1.lam - oldX1.x1 + oldX1.x2)
    val updX0: X3Row = { ...oldX0, x3: flipX3_0 }
    val updX1: X3Row = { ...oldX1, x3: flipX3_1 }
    val down0 = recomputeDownstream(sRows, yRows, 0, flipX3_0)
    val down1 = recomputeDownstream(down0.srcRows, down0.yRows, 1, flipX3_1)
    buildState(isDouble, down1.srcRows, lRows, [updX0, updX1], down1.yRows, NO_EXTRA_MSG)
  }

  // 3. Target m_no_id_send: swap x3 outputs sent on bus o+6 between row 0 (id=1) and row 1 (id=2).
  action proveAttackNoIdSend(isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val down0 = recomputeDownstream(sRows, yRows, 0, xRows[1].x3)
    val down1 = recomputeDownstream(down0.srcRows, down0.yRows, 1, xRows[0].x3)
    buildState(isDouble, down1.srcRows, lRows, xRows, down1.yRows, NO_EXTRA_MSG)
  }

  // 4. Target m_recv_swapped: swap X3.lam and X3.x1 on both rows so (id, x1, lam) matches L's (id, lam, x1) on o+2.
  action proveAttackRecvSwapped(isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val oldX0 = xRows[0]
    val oldX1 = xRows[1]
    val newX3_0 = expectedX3(oldX0.x1, oldX0.lam, oldX0.x2)
    val newX3_1 = expectedX3(oldX1.x1, oldX1.lam, oldX1.x2)
    val updX0: X3Row = { ...oldX0, lam: oldX0.x1, x1: oldX0.lam, x3: newX3_0 }
    val updX1: X3Row = { ...oldX1, lam: oldX1.x1, x1: oldX1.lam, x3: newX3_1 }
    val down0 = recomputeDownstream(sRows, yRows, 0, newX3_0)
    val down1 = recomputeDownstream(down0.srcRows, down0.yRows, 1, newX3_1)
    buildState(isDouble, down1.srcRows, lRows, [updX0, updX1], down1.yRows, NO_EXTRA_MSG)
  }

  // 5. Target m_valid_free: set X3[r].isValid = 0 (disabling FieldExpr on X3[r]) and forge X3[r].x3.
  action proveAttackValidFree(r: int, deltaX3: int, isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val oldX = xRows[r]
    val forgedX3 = addFp(oldX.x3, deltaX3)
    val updX: X3Row = { ...oldX, isValid: 0, x3: forgedX3 }
    val down = recomputeDownstream(sRows, yRows, r, forgedX3)
    buildState(isDouble, down.srcRows, lRows, replaceX3Row(xRows, r, updX), down.yRows, NO_EXTRA_MSG)
  }

  // 6. General single-cell tamper on X3 (lam, x1, x2, x3, rowId, isValid) without mutant adaptation.
  action proveTamperSingleCell(r: int, whichCol: int, delta: int, isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val oldX = xRows[r]
    val updX: X3Row =
      if (whichCol == 0) { ...oldX, lam: addFp(oldX.lam, delta) }
      else if (whichCol == 1) { ...oldX, x1: addFp(oldX.x1, delta) }
      else if (whichCol == 2) { ...oldX, x2: addFp(oldX.x2, delta) }
      else if (whichCol == 3) { ...oldX, x3: addFp(oldX.x3, delta) }
      else if (whichCol == 4) { ...oldX, rowId: oldX.rowId + delta }
      else { ...oldX, isValid: 1 - oldX.isValid }
    buildState(isDouble, sRows, lRows, replaceX3Row(xRows, r, updX), yRows, NO_EXTRA_MSG)
  }

  // 7. Drop (multDelta = -1) or duplicate (multDelta = +1) one message on bus o+2, o+3, or o+6.
  action proveTamperBusMessage(r: int, whichBus: int, multDelta: int, isDouble: bool): bool = {
    val bs = baseBundles(isDouble)
    val sRows = [bs[0].src, bs[1].src]
    val lRows = [bs[0].l, bs[1].l]
    val xRows = [bs[0].x, bs[1].x]
    val yRows = [bs[0].y, bs[1].y]
    val o = if (isDouble) BUS_DBL else BUS_ADD
    val rx = xRows[r]
    val extra: BusMsg =
      if (whichBus == 0) { busIdx: o + 2, payload: [rx.rowId, rx.lam, rx.x1], mult: multDelta }
      else if (whichBus == 1) { busIdx: o + 3, payload: [rx.rowId, rx.x2], mult: multDelta }
      else { busIdx: o + 6, payload: [rx.rowId, rx.x3], mult: multDelta }
    buildState(isDouble, sRows, lRows, xRows, yRows, extra)
  }

  // ---- Special analysis actions for Question (1) and Question (2) (invoked in x3_test.qnt).

  // Question (1): Two rows share the same id (rowId = 1 on both rows 0 and 1); X3 swaps x2 on bus o+3.
  action proveSameIdSwapX2: bool = {
    val b0 = mkHonestAddBundle(1, PT0, PT1)
    val b1 = mkHonestAddBundle(1, PT2, PT3)
    val x0SwappedX3 = expectedX3(b0.x.lam, b0.x.x1, b1.x.x2)
    val x1SwappedX3 = expectedX3(b1.x.lam, b1.x.x1, b0.x.x2)
    val updX0: X3Row = { ...b0.x, x2: b1.x.x2, x3: x0SwappedX3 }
    val updX1: X3Row = { ...b1.x, x2: b0.x.x2, x3: x1SwappedX3 }
    val baseS = [b0.src, b1.src]
    val baseY = [b0.y, b1.y]
    val down0 = recomputeDownstream(baseS, baseY, 0, x0SwappedX3)
    val down1 = recomputeDownstream(down0.srcRows, down0.yRows, 1, x1SwappedX3)
    buildState(false, down1.srcRows, [b0.l, b1.l], [updX0, updX1], down1.yRows, NO_EXTRA_MSG)
  }

  // Question (2a): Exceptional case P1 == P2 (x1 == x2, y1 == y2) in g1_add allows arbitrary forged lambda.
  action proveExceptionalAddEqualPoints(forgedLam: int): bool = {
    val b0 = mkHonestAddBundle(1, PT0, PT0)
    val b1 = mkHonestAddBundle(2, PT2, PT3)
    val forgedX3 = expectedX3(forgedLam, PT0.x, PT0.x)
    val forgedY3 = subFp(mulFp(forgedLam, subFp(PT0.x, forgedX3)), PT0.y)
    val updS0: SrcRow = { ...b0.src, x2: PT0.x, y2: PT0.y, x3: forgedX3, y3: forgedY3 }
    val updL0: LRow = { ...b0.l, x2: PT0.x, y2: PT0.y, lam: forgedLam }
    val updX0: X3Row = { ...b0.x, lam: forgedLam, x2: PT0.x, x3: forgedX3 }
    val updY0: Y3Row = { ...b0.y, lam: forgedLam, x3: forgedX3, y3: forgedY3 }
    buildState(false, [updS0, b1.src], [updL0, b1.l], [updX0, b1.x], [updY0, b1.y], NO_EXTRA_MSG)
  }

  // Question (2b): Exceptional case P2 == -P1 (x1 == x2, y1 != y2) in g1_add is unsatisfiable for any lambda.
  action proveExceptionalAddInversePoints(candLam: int): bool = {
    val negPT0: Pt = { x: PT0.x, y: subFp(0, PT0.y) }
    val b0 = mkHonestAddBundle(1, PT0, negPT0)
    val b1 = mkHonestAddBundle(2, PT2, PT3)
    val candX3 = expectedX3(candLam, PT0.x, negPT0.x)
    val candY3 = subFp(mulFp(candLam, subFp(PT0.x, candX3)), PT0.y)
    val updS0: SrcRow = { ...b0.src, x3: candX3, y3: candY3 }
    val updL0: LRow = { ...b0.l, lam: candLam }
    val updX0: X3Row = { ...b0.x, lam: candLam, x3: candX3 }
    val updY0: Y3Row = { ...b0.y, lam: candLam, x3: candX3, y3: candY3 }
    buildState(false, [updS0, b1.src], [updL0, b1.l], [updX0, b1.x], [updY0, b1.y], NO_EXTRA_MSG)
  }

  // Question (2c): Exceptional case (x, y) = (0, 0) in g1_double allows arbitrary forged lambda.
  action proveExceptionalDblOrigin(forgedLam: int): bool = {
    val zeroPt: Pt = { x: 0, y: 0 }
    val b0 = mkHonestDblBundle(1, zeroPt)
    val b1 = mkHonestDblBundle(2, PT1)
    val forgedX3 = expectedX3(forgedLam, 0, 0)
    val forgedY3 = subFp(mulFp(forgedLam, subFp(0, forgedX3)), 0)
    val updS0: SrcRow = { ...b0.src, x3: forgedX3, y3: forgedY3 }
    val updL0: LRow = { ...b0.l, lam: forgedLam }
    val updX0: X3Row = { ...b0.x, lam: forgedLam, x3: forgedX3 }
    val updY0: Y3Row = { ...b0.y, lam: forgedLam, x3: forgedX3, y3: forgedY3 }
    buildState(true, [updS0, b1.src], [updL0, b1.l], [updX0, b1.x], [updY0, b1.y], NO_EXTRA_MSG)
  }

  // ---- State machine init and step.
  action init = proveHonestAdd(false)

  action step = any {
    {
      nondet perm = Set(false, true).oneOf()
      any {
        proveHonestAdd(perm),
        proveHonestDbl(perm),
      }
    },
    {
      nondet r = Set(0, 1).oneOf()
      nondet d = Set(1, 2, 5).oneOf()
      nondet isDbl = Set(false, true).oneOf()
      proveAttackNoRecvX2(r, d, isDbl)
    },
    {
      nondet isDbl = Set(false, true).oneOf()
      proveAttackSignFlip(isDbl)
    },
    {
      nondet isDbl = Set(false, true).oneOf()
      proveAttackNoIdSend(isDbl)
    },
    {
      nondet isDbl = Set(false, true).oneOf()
      proveAttackRecvSwapped(isDbl)
    },
    {
      nondet r = Set(0, 1).oneOf()
      nondet d = Set(1, 2, 5).oneOf()
      nondet isDbl = Set(false, true).oneOf()
      proveAttackValidFree(r, d, isDbl)
    },
    {
      nondet r = Set(0, 1).oneOf()
      nondet col = Set(0, 1, 2, 3, 4, 5).oneOf()
      nondet d = Set(1, 2, 5).oneOf()
      nondet isDbl = Set(false, true).oneOf()
      proveTamperSingleCell(r, col, d, isDbl)
    },
    {
      nondet r = Set(0, 1).oneOf()
      nondet b = Set(0, 1, 2).oneOf()
      nondet md = Set(-1, 1).oneOf()
      nondet isDbl = Set(false, true).oneOf()
      proveTamperBusMessage(r, b, md, isDbl)
    },
  }

  // ---- Refinement invariant invRow.
  // Every valid row of X3 that is balanced on the bus computes its output from inputs that equal
  // the values sent by the producing row with the same id, and the chain's output (x3, y3) is the
  // affine sum of P1 and P2, x1 != x2 (or the affine double [2]P1 when isDouble is true).
  pure def matchesProducerL(rx: X3Row, lRows: List[LRow], isDouble: bool): bool =
    lRows.foldl(false, (acc, rl) =>
      acc or (
        rl.isValid == 1 and
        rl.rowId == rx.rowId and
        rx.lam == rl.lam and
        rx.x1 == rl.x1 and
        rx.x2 == (if (isDouble) rl.x1 else rl.x2)
      )
    )

  pure def x3RowsRefine(xRows: List[X3Row], lRows: List[LRow], isDouble: bool): bool =
    xRows.foldl(true, (acc, rx) =>
      acc and (
        (rx.isValid == 1) implies and {
          matchesProducerL(rx, lRows, isDouble),
          rx.x3 == modP(rx.lam * rx.lam - rx.x1 - rx.x2),
        }
      )
    )

  pure def chainOutputsAffineSum(srcRows: List[SrcRow], isDouble: bool): bool =
    srcRows.foldl(true, (acc, rs) =>
      acc and (
        (rs.isActive == 1) implies (
          if (isDouble) {
            val expPt = affineDbl(rs.x1, rs.y1)
            rs.x3 == expPt.x and rs.y3 == expPt.y
          } else {
            (rs.x1 != rs.x2) implies {
              val expPt = affineAdd(rs.x1, rs.y1, rs.x2, rs.y2)
              rs.x3 == expPt.x and rs.y3 == expPt.y
            }
          }
        )
      )
    )

  val invRow: bool =
    st.accepted implies and {
      x3RowsRefine(st.xRows, st.lRows, st.isDouble),
      chainOutputsAffineSum(st.srcRows, st.isDouble),
    }

  // ---- Non-vacuity negated reach invariants (MUST BE VIOLATED in x3_sound / x3).
  val invNoAcceptedHonest: bool =
    not(st.accepted and st.extraMsg.mult == 0)

  val invNoAcceptedAdd: bool =
    not(st.accepted and not(st.isDouble))

  val invNoAcceptedDbl: bool =
    not(st.accepted and st.isDouble)
}

// Sound module (all constraints active)
module x3_sound {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ).*
}

module x3 {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ).*
}

// Mutant 1: drop the receive on bus o+3 (x2 free)
module m_no_recv_x2 {
  import x3_base(
    NO_RECV_X2 = true,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ).*
}

// Mutant 2: constraint x3 = lambda^2 - x1 + x2
module m_sign_flip {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = true,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ).*
}

// Mutant 3: drop id from the send on bus o+6
module m_no_id_send {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = true,
    RECV_SWAPPED = false,
    VALID_FREE = false
  ).*
}

// Mutant 4: receive (id, x1, lambda) in place of (id, lambda, x1)
module m_recv_swapped {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = true,
    VALID_FREE = false
  ).*
}

// Mutant 5: multiplicity 1 in place of is_valid on the send
module m_valid_free {
  import x3_base(
    NO_RECV_X2 = false,
    SIGN_FLIP = false,
    NO_ID_SEND = false,
    RECV_SWAPPED = false,
    VALID_FREE = true
  ).*
}
