// Soundness and Completeness Model for Sherlock X14 `L_add` (BLS12-381 G1 affine addition slope AIR)
// Over small prime field F_13 (E: y^2 = x^3 + 4 mod 13).
// Premise: stock OpenVM mod-builder FieldExpr is sound for one modular constraint with limb bounds checked.

module l_add_spec {
  const MUTANT: str

  // Small prime p = 13 standing in for BLS12-381 base field F_p
  pure val P: int = 13

  // Sentinel id when id is omitted on a bus
  pure val NO_ID_TAG: int = 99

  pure def modP(a: int): int = ((a % P) + P) % P

  pure def addFp(a: int, b: int): int = modP(a + b)

  pure def subFp(a: int, b: int): int = modP(a - b)

  pure def mulFp(a: int, b: int): int = modP(a * b)

  // Multiplicative inverse in F_13 (0 maps as 0)
  pure def invFp(a: int): int =
    if (modP(a) == 1) 1
    else if (modP(a) == 2) 7
    else if (modP(a) == 3) 9
    else if (modP(a) == 4) 10
    else if (modP(a) == 5) 8
    else if (modP(a) == 6) 11
    else if (modP(a) == 7) 2
    else if (modP(a) == 8) 5
    else if (modP(a) == 9) 3
    else if (modP(a) == 10) 4
    else if (modP(a) == 11) 6
    else if (modP(a) == 12) 12
    else 0

  pure def divFp(num: int, den: int): int = mulFp(num, invFp(den))

  pure def isBool(b: int): bool = (b == 0 or b == 1)

  pure def inCanonicalFp(v: int): bool = (0 <= v and v < P)

  // Short Weierstrass curve y^2 = x^3 + 4 (mod 13)
  pure def onCurve(x: int, y: int): bool =
    inCanonicalFp(x) and inCanonicalFp(y) and
    (mulFp(y, y) == addFp(mulFp(mulFp(x, x), x), 4))

  // Reference affine addition coordinates P1 + P2 for x1 != x2 over F_13
  pure def affineSlope(x1: int, y1: int, x2: int, y2: int): int =
    divFp(subFp(y2, y1), subFp(x2, x1))

  pure def slopeToX3(s: int, x1: int, x2: int): int =
    subFp(subFp(mulFp(s, s), x1), x2)

  pure def slopeToY3(s: int, x1: int, x3: int, y1: int): int =
    subFp(mulFp(s, subFp(x1, x3)), y1)

  pure def affineAddX(x1: int, y1: int, x2: int, y2: int): int =
    slopeToX3(affineSlope(x1, y1, x2, y2), x1, x2)

  pure def affineAddY(x1: int, y1: int, x2: int, y2: int): int =
    slopeToY3(affineSlope(x1, y1, x2, y2), x1, affineAddX(x1, y1, x2, y2), y1)

  // Configuration switches for the sound AIR vs the 5 required mutants
  type AirCfg = {
    recvP2: bool,
    sendWrongX: bool,
    includeIdOnY3: bool,
    enforceDivConstraint: bool,
    validFreeSends: bool
  }

  pure val SOUND_CFG: AirCfg = {
    recvP2: true,
    sendWrongX: false,
    includeIdOnY3: true,
    enforceDivConstraint: true,
    validFreeSends: false
  }

  pure val CFG_NO_RECV_P2: AirCfg = {
    recvP2: false,
    sendWrongX: false,
    includeIdOnY3: true,
    enforceDivConstraint: true,
    validFreeSends: false
  }

  pure val CFG_SEND_WRONG_X: AirCfg = {
    recvP2: true,
    sendWrongX: true,
    includeIdOnY3: true,
    enforceDivConstraint: true,
    validFreeSends: false
  }

  pure val CFG_NO_ID: AirCfg = {
    recvP2: true,
    sendWrongX: false,
    includeIdOnY3: false,
    enforceDivConstraint: true,
    validFreeSends: false
  }

  pure val CFG_NO_DIV_CONSTRAINT: AirCfg = {
    recvP2: true,
    sendWrongX: false,
    includeIdOnY3: true,
    enforceDivConstraint: false,
    validFreeSends: false
  }

  pure val CFG_VALID_FREE: AirCfg = {
    recvP2: true,
    sendWrongX: false,
    includeIdOnY3: true,
    enforceDivConstraint: true,
    validFreeSends: true
  }

  // Row types for Producer, L_add, X3, Y3, and Consumer
  type ProdRow = {
    rowId: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    isValid: int,
    dropP2Send: bool
  }

  type LAddRow = {
    rowId: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    lam: int,
    isValid: int
  }

  type X3Row = {
    rowId: int,
    lam: int,
    x1: int,
    x2: int,
    x3: int,
    isValid: int
  }

  type Y3Row = {
    rowId: int,
    lam: int,
    x1: int,
    x3: int,
    y1: int,
    y3: int,
    isValid: int
  }

  type ConsRow = {
    rowId: int,
    x3: int,
    y3: int,
    isValid: int
  }

  // Signed bus interaction entry on a specific LogUp permutation bus
  type BusMsg = {
    msgId: int,
    v0: int,
    v1: int,
    delta: int
  }

  pure val NO_EXTRA_MSG: BusMsg = {
    msgId: 0,
    v0: 0,
    v1: 0,
    delta: 0
  }

  type ChainState = {
    prod0: ProdRow,
    prod1: ProdRow,
    l0: LAddRow,
    l1: LAddRow,
    x0: X3Row,
    x1: X3Row,
    y0: Y3Row,
    y1: Y3Row,
    cons0: ConsRow,
    cons1: ConsRow,
    extraMsg: BusMsg
  }

  // Row-level AIR constraints
  pure def lAddRowValid(cfg: AirCfg, r: LAddRow): bool =
    isBool(r.isValid) and (
      if (r.isValid == 1)
        inCanonicalFp(r.x1) and inCanonicalFp(r.y1) and
        inCanonicalFp(r.x2) and inCanonicalFp(r.y2) and
        inCanonicalFp(r.lam) and (
          if (cfg.enforceDivConstraint)
            mulFp(r.lam, subFp(r.x2, r.x1)) == subFp(r.y2, r.y1)
          else
            true
        )
      else
        true
    )

  pure def rowValidForLam(cfg: AirCfg, x1: int, y1: int, x2: int, y2: int, candidateLam: int, expectValid: bool): bool =
    lAddRowValid(cfg, { rowId: 0, x1: x1, y1: y1, x2: x2, y2: y2, lam: candidateLam, isValid: 1 }) == expectValid

  pure def allFpHoldRowValid(cfg: AirCfg, x1: int, y1: int, x2: int, y2: int, expectValid: bool): bool =
    rowValidForLam(cfg, x1, y1, x2, y2, 0, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 1, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 2, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 3, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 4, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 5, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 6, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 7, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 8, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 9, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 10, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 11, expectValid) and
    rowValidForLam(cfg, x1, y1, x2, y2, 12, expectValid)

  pure def x3RowValid(r: X3Row): bool =
    isBool(r.isValid) and (
      if (r.isValid == 1)
        inCanonicalFp(r.lam) and inCanonicalFp(r.x1) and
        inCanonicalFp(r.x2) and inCanonicalFp(r.x3) and
        (r.x3 == subFp(subFp(mulFp(r.lam, r.lam), r.x1), r.x2))
      else
        true
    )

  pure def y3RowValid(r: Y3Row): bool =
    isBool(r.isValid) and (
      if (r.isValid == 1)
        inCanonicalFp(r.lam) and inCanonicalFp(r.x1) and
        inCanonicalFp(r.x3) and inCanonicalFp(r.y1) and
        inCanonicalFp(r.y3) and
        (r.y3 == subFp(mulFp(r.lam, subFp(r.x1, r.x3)), r.y1))
      else
        true
    )

  // Exact multiset balance check per bus (1100..1107)
  pure def sameBusEntry(a: BusMsg, b: BusMsg): bool =
    (a.msgId == b.msgId) and (a.v0 == b.v0) and (a.v1 == b.v1)

  pure def net4(target: BusMsg, a: BusMsg, b: BusMsg, c: BusMsg, d: BusMsg): int =
    (if (sameBusEntry(target, a)) a.delta else 0) +
    (if (sameBusEntry(target, b)) b.delta else 0) +
    (if (sameBusEntry(target, c)) c.delta else 0) +
    (if (sameBusEntry(target, d)) d.delta else 0)

  pure def bus4Balanced(a: BusMsg, b: BusMsg, c: BusMsg, d: BusMsg): bool =
    (net4(a, a, b, c, d) == 0) and
    (net4(b, a, b, c, d) == 0) and
    (net4(c, a, b, c, d) == 0) and
    (net4(d, a, b, c, d) == 0)

  pure def net5(target: BusMsg, a: BusMsg, b: BusMsg, c: BusMsg, d: BusMsg, e: BusMsg): int =
    net4(target, a, b, c, d) + (if (sameBusEntry(target, e)) e.delta else 0)

  pure def bus5Balanced(a: BusMsg, b: BusMsg, c: BusMsg, d: BusMsg, e: BusMsg): bool =
    (net5(a, a, b, c, d, e) == 0) and
    (net5(b, a, b, c, d, e) == 0) and
    (net5(c, a, b, c, d, e) == 0) and
    (net5(d, a, b, c, d, e) == 0) and
    (net5(e, a, b, c, d, e) == 0)

  // Bus 1100 (o+0): Producer sends (id, x1, y1), L_add receives (id, x1, y1)
  pure def busP1Balanced(s: ChainState): bool =
    bus5Balanced(
      { msgId: s.prod0.rowId, v0: s.prod0.x1, v1: s.prod0.y1, delta: s.prod0.isValid },
      { msgId: s.prod1.rowId, v0: s.prod1.x1, v1: s.prod1.y1, delta: s.prod1.isValid },
      { msgId: s.l0.rowId, v0: s.l0.x1, v1: s.l0.y1, delta: 0 - s.l0.isValid },
      { msgId: s.l1.rowId, v0: s.l1.x1, v1: s.l1.y1, delta: 0 - s.l1.isValid },
      s.extraMsg
    )

  // Bus 1101 (o+1): Producer sends (id, x2, y2), L_add receives (id, x2, y2)
  pure def busP2Balanced(cfg: AirCfg, s: ChainState): bool =
    bus4Balanced(
      { msgId: s.prod0.rowId, v0: s.prod0.x2, v1: s.prod0.y2, delta: if (s.prod0.dropP2Send) 0 else s.prod0.isValid },
      { msgId: s.prod1.rowId, v0: s.prod1.x2, v1: s.prod1.y2, delta: if (s.prod1.dropP2Send) 0 else s.prod1.isValid },
      { msgId: s.l0.rowId, v0: s.l0.x2, v1: s.l0.y2, delta: if (cfg.recvP2) (0 - s.l0.isValid) else 0 },
      { msgId: s.l1.rowId, v0: s.l1.x2, v1: s.l1.y2, delta: if (cfg.recvP2) (0 - s.l1.isValid) else 0 }
    )

  // Bus 1102 (o+2): L_add sends (id, lambda, x1), X3 receives (id, lambda, x1)
  pure def busLxBalanced(cfg: AirCfg, s: ChainState): bool =
    bus4Balanced(
      {
        msgId: s.l0.rowId,
        v0: s.l0.lam,
        v1: if (cfg.sendWrongX) s.l0.x2 else s.l0.x1,
        delta: if (cfg.validFreeSends) 1 else s.l0.isValid
      },
      {
        msgId: s.l1.rowId,
        v0: s.l1.lam,
        v1: if (cfg.sendWrongX) s.l1.x2 else s.l1.x1,
        delta: if (cfg.validFreeSends) 1 else s.l1.isValid
      },
      { msgId: s.x0.rowId, v0: s.x0.lam, v1: s.x0.x1, delta: 0 - s.x0.isValid },
      { msgId: s.x1.rowId, v0: s.x1.lam, v1: s.x1.x1, delta: 0 - s.x1.isValid }
    )

  // Bus 1103 (o+3): L_add sends (id, x2), X3 receives (id, x2)
  pure def busX2Balanced(cfg: AirCfg, s: ChainState): bool =
    bus4Balanced(
      { msgId: s.l0.rowId, v0: s.l0.x2, v1: 0, delta: if (cfg.validFreeSends) 1 else s.l0.isValid },
      { msgId: s.l1.rowId, v0: s.l1.x2, v1: 0, delta: if (cfg.validFreeSends) 1 else s.l1.isValid },
      { msgId: s.x0.rowId, v0: s.x0.x2, v1: 0, delta: 0 - s.x0.isValid },
      { msgId: s.x1.rowId, v0: s.x1.x2, v1: 0, delta: 0 - s.x1.isValid }
    )

  // Bus 1104 (o+4): L_add sends (id, lambda, x1), Y3 receives (id, lambda, x1)
  pure def busLyBalanced(cfg: AirCfg, s: ChainState): bool =
    bus4Balanced(
      {
        msgId: if (cfg.includeIdOnY3) s.l0.rowId else NO_ID_TAG,
        v0: s.l0.lam,
        v1: s.l0.x1,
        delta: if (cfg.validFreeSends) 1 else s.l0.isValid
      },
      {
        msgId: if (cfg.includeIdOnY3) s.l1.rowId else NO_ID_TAG,
        v0: s.l1.lam,
        v1: s.l1.x1,
        delta: if (cfg.validFreeSends) 1 else s.l1.isValid
      },
      {
        msgId: if (cfg.includeIdOnY3) s.y0.rowId else NO_ID_TAG,
        v0: s.y0.lam,
        v1: s.y0.x1,
        delta: 0 - s.y0.isValid
      },
      {
        msgId: if (cfg.includeIdOnY3) s.y1.rowId else NO_ID_TAG,
        v0: s.y1.lam,
        v1: s.y1.x1,
        delta: 0 - s.y1.isValid
      }
    )

  // Bus 1105 (o+5): L_add sends (id, y1), Y3 receives (id, y1)
  pure def busY1Balanced(cfg: AirCfg, s: ChainState): bool =
    bus4Balanced(
      { msgId: s.l0.rowId, v0: s.l0.y1, v1: 0, delta: if (cfg.validFreeSends) 1 else s.l0.isValid },
      { msgId: s.l1.rowId, v0: s.l1.y1, v1: 0, delta: if (cfg.validFreeSends) 1 else s.l1.isValid },
      { msgId: s.y0.rowId, v0: s.y0.y1, v1: 0, delta: 0 - s.y0.isValid },
      { msgId: s.y1.rowId, v0: s.y1.y1, v1: 0, delta: 0 - s.y1.isValid }
    )

  // Bus 1106 (o+6): X3 sends (id, x3), Y3 receives (id, x3)
  pure def busX3Balanced(s: ChainState): bool =
    bus4Balanced(
      { msgId: s.x0.rowId, v0: s.x0.x3, v1: 0, delta: s.x0.isValid },
      { msgId: s.x1.rowId, v0: s.x1.x3, v1: 0, delta: s.x1.isValid },
      { msgId: s.y0.rowId, v0: s.y0.x3, v1: 0, delta: 0 - s.y0.isValid },
      { msgId: s.y1.rowId, v0: s.y1.x3, v1: 0, delta: 0 - s.y1.isValid }
    )

  // Bus 1107 (o+7): Y3 sends (id, x3, y3), Consumer receives (id, x3, y3)
  pure def busOutBalanced(s: ChainState): bool =
    bus4Balanced(
      { msgId: s.y0.rowId, v0: s.y0.x3, v1: s.y0.y3, delta: s.y0.isValid },
      { msgId: s.y1.rowId, v0: s.y1.x3, v1: s.y1.y3, delta: s.y1.isValid },
      { msgId: s.cons0.rowId, v0: s.cons0.x3, v1: s.cons0.y3, delta: 0 - s.cons0.isValid },
      { msgId: s.cons1.rowId, v0: s.cons1.x3, v1: s.cons1.y3, delta: 0 - s.cons1.isValid }
    )

  pure def allBusesBalanced(cfg: AirCfg, s: ChainState): bool =
    busP1Balanced(s) and
    busP2Balanced(cfg, s) and
    busLxBalanced(cfg, s) and
    busX2Balanced(cfg, s) and
    busLyBalanced(cfg, s) and
    busY1Balanced(cfg, s) and
    busX3Balanced(s) and
    busOutBalanced(s)

  pure def isChainAccepted(cfg: AirCfg, s: ChainState): bool =
    lAddRowValid(cfg, s.l0) and
    lAddRowValid(cfg, s.l1) and
    x3RowValid(s.x0) and
    x3RowValid(s.x1) and
    y3RowValid(s.y0) and
    y3RowValid(s.y1) and
    allBusesBalanced(cfg, s)

  // Honest downstream X3, Y3, and Consumer rows computed from L_add row sends
  pure def buildX3Row(rowId: int, lam: int, sentX1: int, x2: int, xValid: int): X3Row = {
    rowId: rowId,
    lam: lam,
    x1: sentX1,
    x2: x2,
    x3: subFp(subFp(mulFp(lam, lam), sentX1), x2),
    isValid: xValid
  }

  pure def honestX3FromL(cfg: AirCfg, lRow: LAddRow, xValid: int): X3Row =
    buildX3Row(lRow.rowId, lRow.lam, if (cfg.sendWrongX) lRow.x2 else lRow.x1, lRow.x2, xValid)

  pure def honestY3FromLX(lForLy: LAddRow, lForY1: LAddRow, xRow: X3Row, yValid: int): Y3Row = {
    rowId: lForY1.rowId,
    lam: lForLy.lam,
    x1: lForLy.x1,
    x3: xRow.x3,
    y1: lForY1.y1,
    y3: subFp(mulFp(lForLy.lam, subFp(lForLy.x1, xRow.x3)), lForY1.y1),
    isValid: yValid
  }

  pure def honestConsFromY(yRow: Y3Row, cValid: int): ConsRow = {
    rowId: yRow.rowId,
    x3: yRow.x3,
    y3: yRow.y3,
    isValid: cValid
  }

  // Two distinct valid pairs of curve points on E(F_13): y^2 = x^3 + 4 (mod 13)
  // Row 0 (id = 0): P1 = (0, 2), P2 = (2, 5) -> lambda = 8, P3 = (10, 9)
  // Row 1 (id = 1): P1 = (4, 4), P2 = (7, 3) -> lambda = 4, P3 = (5, 5)
  pure val honestProd0: ProdRow = {
    rowId: 0, x1: 0, y1: 2, x2: 2, y2: 5, isValid: 1, dropP2Send: false
  }

  pure val honestProd1: ProdRow = {
    rowId: 1, x1: 4, y1: 4, x2: 7, y2: 3, isValid: 1, dropP2Send: false
  }

  pure val honestL0: LAddRow = {
    rowId: 0, x1: 0, y1: 2, x2: 2, y2: 5, lam: 8, isValid: 1
  }

  pure val honestL1: LAddRow = {
    rowId: 1, x1: 4, y1: 4, x2: 7, y2: 3, lam: 4, isValid: 1
  }

  pure def finishStateFromY(
    p0: ProdRow,
    p1: ProdRow,
    l0: LAddRow,
    l1: LAddRow,
    rx0: X3Row,
    rx1: X3Row,
    ry0: Y3Row,
    ry1: Y3Row,
    active1: int,
    extra: BusMsg
  ): ChainState = {
    prod0: p0,
    prod1: p1,
    l0: l0,
    l1: l1,
    x0: rx0,
    x1: rx1,
    y0: ry0,
    y1: ry1,
    cons0: honestConsFromY(ry0, 1),
    cons1: honestConsFromY(ry1, active1),
    extraMsg: extra
  }

  pure def finishStateFromX(
    p0: ProdRow,
    p1: ProdRow,
    l0: LAddRow,
    l1: LAddRow,
    rx0: X3Row,
    rx1: X3Row,
    swapY3Ly: bool,
    active1: int,
    extra: BusMsg
  ): ChainState =
    finishStateFromY(
      p0,
      p1,
      l0,
      l1,
      rx0,
      rx1,
      honestY3FromLX(if (swapY3Ly) l1 else l0, l0, rx0, 1),
      honestY3FromLX(if (swapY3Ly) l0 else l1, l1, rx1, active1),
      active1,
      extra
    )

  pure def buildStateFromLRows(
    cfg: AirCfg,
    p0: ProdRow,
    p1: ProdRow,
    l0: LAddRow,
    l1: LAddRow,
    swapY3Ly: bool,
    active1: int,
    extra: BusMsg
  ): ChainState =
    finishStateFromX(
      p0,
      p1,
      l0,
      l1,
      honestX3FromL(cfg, l0, 1),
      honestX3FromL(cfg, l1, active1),
      swapY3Ly,
      active1,
      extra
    )

  pure def buildHonestState(cfg: AirCfg): ChainState =
    buildStateFromLRows(cfg, honestProd0, honestProd1, honestL0, honestL1, false, 1, NO_EXTRA_MSG)

  // Refinement invariant `invRow`:
  // Every valid row of L_add that is balanced on the bus computes its output from inputs
  // that equal the values sent by the producing row with the same id, and the chain's
  // output (x3, y3) is the affine sum of P1 and P2, x1 != x2.
  pure def rowRefinementWithSlope(pRow: ProdRow, lRow: LAddRow, cRow: ConsRow, expectedLam: int): bool =
    (pRow.isValid == 1) and
    (lRow.isValid == 1) and
    (cRow.isValid == 1) and
    (lRow.rowId == pRow.rowId) and
    (cRow.rowId == pRow.rowId) and
    (pRow.x1 != pRow.x2) and
    (lRow.x1 == pRow.x1) and
    (lRow.y1 == pRow.y1) and
    (lRow.x2 == pRow.x2) and
    (lRow.y2 == pRow.y2) and
    (lRow.lam == expectedLam) and
    (cRow.x3 == slopeToX3(expectedLam, pRow.x1, pRow.x2)) and
    (cRow.y3 == slopeToY3(expectedLam, pRow.x1, slopeToX3(expectedLam, pRow.x1, pRow.x2), pRow.y1))

  pure def rowRefinementHolds(pRow: ProdRow, lRow: LAddRow, cRow: ConsRow): bool =
    if (lRow.isValid == 1 or cRow.isValid == 1)
      rowRefinementWithSlope(pRow, lRow, cRow, affineSlope(pRow.x1, pRow.y1, pRow.x2, pRow.y2))
    else
      true

  pure def checkInvRow(cfg: AirCfg, s: ChainState): bool =
    if (isChainAccepted(cfg, s))
      rowRefinementHolds(s.prod0, s.l0, s.cons0) and
      rowRefinementHolds(s.prod1, s.l1, s.cons1)
    else
      true

  // Nondeterministic prover transitions:
  // May change any one cell (input, output, id, is_valid) of a row, swap untagged bus messages,
  // or drop or duplicate one message, with the other AIRs computed honestly.
  pure def distinctFp(v: int, avoid: int): int =
    if (modP(v) == avoid) modP(v + 1) else modP(v)

  pure def dropOrDupFirstMsg(r: LAddRow, dropIt: bool): BusMsg = {
    msgId: r.rowId,
    v0: r.x1,
    v1: r.y1,
    delta: if (dropIt) r.isValid else (0 - r.isValid)
  }

  pure def proverStepState(cfg: AirCfg, mode: int, altVal: int, msgIdx: int): ChainState =
    if (mode == 0)
      // Honest chain execution
      buildHonestState(cfg)
    else if (mode == 1)
      // Mutate L_add Row 0 input y2 (and drop producer's P2 send on o+1; Row 1 inactive)
      buildStateFromLRows(
        cfg,
        {
          rowId: honestProd0.rowId,
          x1: honestProd0.x1,
          y1: honestProd0.y1,
          x2: honestProd0.x2,
          y2: honestProd0.y2,
          isValid: 1,
          dropP2Send: true
        },
        {
          rowId: honestProd1.rowId,
          x1: honestProd1.x1,
          y1: honestProd1.y1,
          x2: honestProd1.x2,
          y2: honestProd1.y2,
          isValid: 0,
          dropP2Send: false
        },
        {
          rowId: 0,
          x1: honestL0.x1,
          y1: honestL0.y1,
          x2: honestL0.x2,
          y2: distinctFp(altVal, honestProd0.y2),
          lam: divFp(subFp(distinctFp(altVal, honestProd0.y2), honestL0.y1), subFp(honestL0.x2, honestL0.x1)),
          isValid: 1
        },
        {
          rowId: 1,
          x1: honestL1.x1,
          y1: honestL1.y1,
          x2: honestL1.x2,
          y2: honestL1.y2,
          lam: honestL1.lam,
          isValid: 0
        },
        false,
        0,
        NO_EXTRA_MSG
      )
    else if (mode == 2)
      // Mutate L_add Row 0 output lambda with a forged value
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        {
          rowId: 0,
          x1: honestL0.x1,
          y1: honestL0.y1,
          x2: honestL0.x2,
          y2: honestL0.y2,
          lam: distinctFp(altVal, honestL0.lam),
          isValid: 1
        },
        honestL1,
        false,
        1,
        NO_EXTRA_MSG
      )
    else if (mode == 3)
      // Swap (lambda, x1) messages on bus o+4 between Row 0 and Row 1 in Y3
      buildStateFromLRows(cfg, honestProd0, honestProd1, honestL0, honestL1, true, 1, NO_EXTRA_MSG)
    else if (mode == 4)
      // Row 1 has L_add is_valid = 0 (and producer inactive) with forged lambda, while X3/Y3/Cons are active
      buildStateFromLRows(
        cfg,
        honestProd0,
        {
          rowId: 1,
          x1: honestProd1.x1,
          y1: honestProd1.y1,
          x2: honestProd1.x2,
          y2: honestProd1.y2,
          isValid: 0,
          dropP2Send: false
        },
        honestL0,
        {
          rowId: 1,
          x1: honestL1.x1,
          y1: honestL1.y1,
          x2: honestL1.x2,
          y2: honestL1.y2,
          lam: distinctFp(altVal, honestL1.lam),
          isValid: 0
        },
        false,
        1,
        NO_EXTRA_MSG
      )
    else if (mode == 5)
      // Mutate L_add Row 0 input x1
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        {
          rowId: 0,
          x1: distinctFp(altVal, honestL0.x1),
          y1: honestL0.y1,
          x2: honestL0.x2,
          y2: honestL0.y2,
          lam: honestL0.lam,
          isValid: 1
        },
        honestL1,
        false,
        1,
        NO_EXTRA_MSG
      )
    else if (mode == 6)
      // Mutate L_add Row 0 input y1
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        {
          rowId: 0,
          x1: honestL0.x1,
          y1: distinctFp(altVal, honestL0.y1),
          x2: honestL0.x2,
          y2: honestL0.y2,
          lam: honestL0.lam,
          isValid: 1
        },
        honestL1,
        false,
        1,
        NO_EXTRA_MSG
      )
    else if (mode == 7)
      // Mutate L_add Row 0 input x2
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        {
          rowId: 0,
          x1: honestL0.x1,
          y1: honestL0.y1,
          x2: distinctFp(altVal, honestL0.x2),
          y2: honestL0.y2,
          lam: honestL0.lam,
          isValid: 1
        },
        honestL1,
        false,
        1,
        NO_EXTRA_MSG
      )
    else if (mode == 8)
      // Mutate L_add Row 0 id column
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        {
          rowId: distinctFp(altVal, honestL0.rowId),
          x1: honestL0.x1,
          y1: honestL0.y1,
          x2: honestL0.x2,
          y2: honestL0.y2,
          lam: honestL0.lam,
          isValid: 1
        },
        honestL1,
        false,
        1,
        NO_EXTRA_MSG
      )
    else
      // Drop or duplicate one bus message from L_add Row 0
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        honestL0,
        honestL1,
        false,
        1,
        dropOrDupFirstMsg(honestL0, ((altVal + msgIdx) % 2) == 0)
      )

  pure val MODES: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
  pure val FP_SET: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
  pure val MSG_IDX_SET: Set[int] = Set(0, 1, 2, 3, 4, 5)

  pure def cfgForMutant(m: str): AirCfg =
    if (m == "no_recv_p2")
      CFG_NO_RECV_P2
    else if (m == "send_wrong_x")
      CFG_SEND_WRONG_X
    else if (m == "no_id")
      CFG_NO_ID
    else if (m == "no_div_constraint")
      CFG_NO_DIV_CONSTRAINT
    else if (m == "valid_free")
      CFG_VALID_FREE
    else
      SOUND_CFG

  pure def initModeForMutant(m: str): int =
    if (m == "no_recv_p2")
      1
    else if (m == "no_div_constraint")
      2
    else if (m == "no_id")
      3
    else if (m == "valid_free")
      4
    else
      0

  pure val activeCfg: AirCfg = cfgForMutant(MUTANT)

  var st: ChainState

  action init = {
    st' = proverStepState(activeCfg, initModeForMutant(MUTANT), 0, 0)
  }

  action step = {
    nondet mode = MODES.oneOf()
    nondet altVal = FP_SET.oneOf()
    nondet msgIdx = MSG_IDX_SET.oneOf()
    st' = proverStepState(activeCfg, mode, altVal, msgIdx)
  }

  // Soundness refinement invariant (must HOLD in l_add, and be VIOLATED in each mutant)
  val invRow: bool = checkInvRow(activeCfg, st)

  // Non-vacuity negated reach invariants (must be VIOLATED in l_add on an accepted honest chain)
  val invNoAcceptedChain: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
  val nonVacuous: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
  val invNonVacuous: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
  val nonVacuity: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
  val reachAccepted: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
  val reachHonest: bool = not(isChainAccepted(activeCfg, st) and checkInvRow(activeCfg, st))
}

module l_add { import l_add_spec(MUTANT = "none").* }
module m_no_recv_p2 { import l_add_spec(MUTANT = "no_recv_p2").* }
module m_send_wrong_x { import l_add_spec(MUTANT = "send_wrong_x").* }
module m_no_id { import l_add_spec(MUTANT = "no_id").* }
module m_no_div_constraint { import l_add_spec(MUTANT = "no_div_constraint").* }
module m_valid_free { import l_add_spec(MUTANT = "valid_free").* }
