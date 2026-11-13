// Soundness & Completeness Model for Sherlock X14 `L_add` (BLS12-381 G1 affine addition slope AIR)
// Over small prime field F_13 (E: y^2 = x^3 + 4 mod 13).
// Premise: stock OpenVM mod-builder FieldExpr is sound for one modular constraint with range-checked limbs.

module l_add_core {
  // Small prime p = 13 standing in for BLS12-381 base field F_p
  pure val P: int = 13

  // Sentinel id when id is omitted on a bus
  pure val NO_ID_TAG: int = 99

  // Bus indices for G1 addition chain (base offset o = 1100)
  pure val BUS_O: int = 1100
  pure val BUS_P1: int = 1100   // o+0: receive (id, x1, y1) from producer
  pure val BUS_P2: int = 1101   // o+1: receive (id, x2, y2) from producer
  pure val BUS_LX: int = 1102   // o+2: send (id, lambda, x1) to X3
  pure val BUS_X2: int = 1103   // o+3: send (id, x2) to X3
  pure val BUS_LY: int = 1104   // o+4: send (id, lambda, x1) to Y3
  pure val BUS_Y1: int = 1105   // o+5: send (id, y1) to Y3
  pure val BUS_X3: int = 1106   // o+6: X3 sends (id, x3) to Y3
  pure val BUS_OUT: int = 1107  // o+7: Y3 sends (id, x3, y3) to consumer

  pure val FpList: List[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]

  pure def modP(a: int): int = ((a % P) + P) % P

  pure def addFp(a: int, b: int): int = modP(a + b)

  pure def subFp(a: int, b: int): int = modP(a - b)

  pure def mulFp(a: int, b: int): int = modP(a * b)

  // Multiplicative inverse in F_13 (0 maps to 0)
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

  type Point = {
    x: int,
    y: int
  }

  // Reference affine addition P1 + P2 for x1 != x2 over F_13
  pure def affineAdd(x1: int, y1: int, x2: int, y2: int): Point = {
    x: subFp(subFp(mulFp(divFp(subFp(y2, y1), subFp(x2, x1)), divFp(subFp(y2, y1), subFp(x2, x1))), x1), x2),
    y: subFp(
      mulFp(
        divFp(subFp(y2, y1), subFp(x2, x1)),
        subFp(x1, subFp(subFp(mulFp(divFp(subFp(y2, y1), subFp(x2, x1)), divFp(subFp(y2, y1), subFp(x2, x1))), x1), x2))
      ),
      y1
    )
  }

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

  // Signed bus interaction entry in the LogUp permutation check multiset
  type BusMsg = {
    bus: int,
    hasId: bool,
    msgId: int,
    arity: int,
    v0: int,
    v1: int,
    delta: int
  }

  pure val NO_EXTRA_MSG: BusMsg = {
    bus: BUS_O,
    hasId: true,
    msgId: 0,
    arity: 1,
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

  // Message constructors for each AIR row
  pure def prodRowMsgs(p: ProdRow): List[BusMsg] = [
    { bus: BUS_P1, hasId: true, msgId: p.rowId, arity: 2, v0: p.x1, v1: p.y1, delta: p.isValid },
    { bus: BUS_P2, hasId: true, msgId: p.rowId, arity: 2, v0: p.x2, v1: p.y2, delta: if (p.dropP2Send) 0 else p.isValid }
  ]

  pure def lAddRowMsgs(cfg: AirCfg, r: LAddRow): List[BusMsg] = [
    {
      bus: BUS_P1,
      hasId: true,
      msgId: r.rowId,
      arity: 2,
      v0: r.x1,
      v1: r.y1,
      delta: 0 - r.isValid
    },
    {
      bus: BUS_P2,
      hasId: true,
      msgId: r.rowId,
      arity: 2,
      v0: r.x2,
      v1: r.y2,
      delta: if (cfg.recvP2) (0 - r.isValid) else 0
    },
    {
      bus: BUS_LX,
      hasId: true,
      msgId: r.rowId,
      arity: 2,
      v0: r.lam,
      v1: if (cfg.sendWrongX) r.x2 else r.x1,
      delta: if (cfg.validFreeSends) 1 else r.isValid
    },
    {
      bus: BUS_X2,
      hasId: true,
      msgId: r.rowId,
      arity: 1,
      v0: r.x2,
      v1: 0,
      delta: if (cfg.validFreeSends) 1 else r.isValid
    },
    {
      bus: BUS_LY,
      hasId: cfg.includeIdOnY3,
      msgId: if (cfg.includeIdOnY3) r.rowId else NO_ID_TAG,
      arity: 2,
      v0: r.lam,
      v1: r.x1,
      delta: if (cfg.validFreeSends) 1 else r.isValid
    },
    {
      bus: BUS_Y1,
      hasId: true,
      msgId: r.rowId,
      arity: 1,
      v0: r.y1,
      v1: 0,
      delta: if (cfg.validFreeSends) 1 else r.isValid
    }
  ]

  pure def x3RowMsgs(r: X3Row): List[BusMsg] = [
    { bus: BUS_LX, hasId: true, msgId: r.rowId, arity: 2, v0: r.lam, v1: r.x1, delta: 0 - r.isValid },
    { bus: BUS_X2, hasId: true, msgId: r.rowId, arity: 1, v0: r.x2, v1: 0, delta: 0 - r.isValid },
    { bus: BUS_X3, hasId: true, msgId: r.rowId, arity: 1, v0: r.x3, v1: 0, delta: r.isValid }
  ]

  pure def y3RowMsgs(cfg: AirCfg, r: Y3Row): List[BusMsg] = [
    {
      bus: BUS_LY,
      hasId: cfg.includeIdOnY3,
      msgId: if (cfg.includeIdOnY3) r.rowId else NO_ID_TAG,
      arity: 2,
      v0: r.lam,
      v1: r.x1,
      delta: 0 - r.isValid
    },
    { bus: BUS_Y1, hasId: true, msgId: r.rowId, arity: 1, v0: r.y1, v1: 0, delta: 0 - r.isValid },
    { bus: BUS_X3, hasId: true, msgId: r.rowId, arity: 1, v0: r.x3, v1: 0, delta: 0 - r.isValid },
    { bus: BUS_OUT, hasId: true, msgId: r.rowId, arity: 2, v0: r.x3, v1: r.y3, delta: r.isValid }
  ]

  pure def consRowMsgs(c: ConsRow): List[BusMsg] = [
    { bus: BUS_OUT, hasId: true, msgId: c.rowId, arity: 2, v0: c.x3, v1: c.y3, delta: 0 - c.isValid }
  ]

  pure def allMessages(cfg: AirCfg, s: ChainState): List[BusMsg] =
    prodRowMsgs(s.prod0)
      .concat(prodRowMsgs(s.prod1))
      .concat(lAddRowMsgs(cfg, s.l0))
      .concat(lAddRowMsgs(cfg, s.l1))
      .concat(x3RowMsgs(s.x0))
      .concat(x3RowMsgs(s.x1))
      .concat(y3RowMsgs(cfg, s.y0))
      .concat(y3RowMsgs(cfg, s.y1))
      .concat(consRowMsgs(s.cons0))
      .concat(consRowMsgs(s.cons1))
      .append(s.extraMsg)

  pure def samePayload(m1: BusMsg, m2: BusMsg): bool =
    (m1.bus == m2.bus) and
    (m1.hasId == m2.hasId) and
    (m1.msgId == m2.msgId) and
    (m1.arity == m2.arity) and
    (m1.v0 == m2.v0) and
    (m1.v1 == m2.v1)

  pure def netCount(msgs: List[BusMsg], target: BusMsg): int =
    msgs.foldl(0, (acc, m) => if (samePayload(m, target)) acc + m.delta else acc)

  pure def busBalanced(msgs: List[BusMsg]): bool =
    msgs.foldl(true, (ok, m) => ok and (netCount(msgs, m) == 0))

  pure def isChainAccepted(cfg: AirCfg, s: ChainState): bool =
    lAddRowValid(cfg, s.l0) and
    lAddRowValid(cfg, s.l1) and
    x3RowValid(s.x0) and
    x3RowValid(s.x1) and
    y3RowValid(s.y0) and
    y3RowValid(s.y1) and
    busBalanced(allMessages(cfg, s))

  // Honest downstream X3, Y3, and Consumer rows computed from L_add row sends
  pure def honestX3FromL(cfg: AirCfg, lRow: LAddRow, xValid: int): X3Row = {
    rowId: lRow.rowId,
    lam: lRow.lam,
    x1: if (cfg.sendWrongX) lRow.x2 else lRow.x1,
    x2: lRow.x2,
    x3: subFp(subFp(mulFp(lRow.lam, lRow.lam), if (cfg.sendWrongX) lRow.x2 else lRow.x1), lRow.x2),
    isValid: xValid
  }

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

  pure def buildStateFromLRows(
    cfg: AirCfg,
    p0: ProdRow,
    p1: ProdRow,
    l0: LAddRow,
    l1: LAddRow,
    swapY3Ly: bool,
    active1: int,
    extra: BusMsg
  ): ChainState = {
    prod0: p0,
    prod1: p1,
    l0: l0,
    l1: l1,
    x0: honestX3FromL(cfg, l0, 1),
    x1: honestX3FromL(cfg, l1, active1),
    y0: honestY3FromLX(if (swapY3Ly) l1 else l0, l0, honestX3FromL(cfg, l0, 1), 1),
    y1: honestY3FromLX(if (swapY3Ly) l0 else l1, l1, honestX3FromL(cfg, l1, active1), active1),
    cons0: honestConsFromY(honestY3FromLX(if (swapY3Ly) l1 else l0, l0, honestX3FromL(cfg, l0, 1), 1), 1),
    cons1: honestConsFromY(honestY3FromLX(if (swapY3Ly) l0 else l1, l1, honestX3FromL(cfg, l1, active1), active1), active1),
    extraMsg: extra
  }

  pure def buildHonestState(cfg: AirCfg): ChainState =
    buildStateFromLRows(cfg, honestProd0, honestProd1, honestL0, honestL1, false, 1, NO_EXTRA_MSG)

  // Refinement invariant `invRow`:
  // Every valid row of L_add that is balanced on the bus computes its output from inputs
  // that equal the values sent by the producing row with the same id, and the chain's
  // output (x3, y3) is the affine sum of P1 and P2, x1 != x2.
  pure def rowRefinementHolds(pRow: ProdRow, lRow: LAddRow, cRow: ConsRow): bool =
    if (lRow.isValid == 1 or cRow.isValid == 1)
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
      (lRow.lam == divFp(subFp(pRow.y2, pRow.y1), subFp(pRow.x2, pRow.x1))) and
      (cRow.x3 == affineAdd(pRow.x1, pRow.y1, pRow.x2, pRow.y2).x) and
      (cRow.y3 == affineAdd(pRow.x1, pRow.y1, pRow.x2, pRow.y2).y)
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

  pure def dropOrDupMsg(m: BusMsg, dropIt: bool): BusMsg = {
    bus: m.bus,
    hasId: m.hasId,
    msgId: m.msgId,
    arity: m.arity,
    v0: m.v0,
    v1: m.v1,
    delta: if (dropIt) (0 - m.delta) else m.delta
  }

  pure def nextStateForProver(cfg: AirCfg, mode: int, altVal: int, msgIdx: int): ChainState =
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
      // Mutate L_add Row 0 output lambda to a wrong value
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
      // Drop (altVal % 2 == 0) or duplicate (altVal % 2 == 1) one bus message from L_add Row 0
      buildStateFromLRows(
        cfg,
        honestProd0,
        honestProd1,
        honestL0,
        honestL1,
        false,
        1,
        dropOrDupMsg(lAddRowMsgs(cfg, honestL0).nth(msgIdx % 6), (altVal % 2) == 0)
      )
}

// ============================================================================
// Sound module: l_add
// ============================================================================
module l_add {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(SOUND_CFG)
  }

  action step = any {
    st' = nextStateForProver(SOUND_CFG, 0, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 1, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 2, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 3, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 4, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 5, 1, 0),
    st' = nextStateForProver(SOUND_CFG, 6, 1, 0),
    st' = nextStateForProver(SOUND_CFG, 7, 1, 0),
    st' = nextStateForProver(SOUND_CFG, 8, 1, 0),
    st' = nextStateForProver(SOUND_CFG, 9, 0, 0),
    st' = nextStateForProver(SOUND_CFG, 9, 1, 0)
  }

  // Soundness refinement invariant (must HOLD in l_add)
  val invRow: bool = checkInvRow(SOUND_CFG, st)

  // Non-vacuity negated reach invariant (must be VIOLATED in l_add on an accepted honest chain)
  val invNoAcceptedChain: bool = not(isChainAccepted(SOUND_CFG, st) and checkInvRow(SOUND_CFG, st))
  val nonVacuous: bool = not(isChainAccepted(SOUND_CFG, st) and checkInvRow(SOUND_CFG, st))
}

// ============================================================================
// Mutant 1: m_no_recv_p2 (drop the receive of (id, x2, y2) on bus o+1)
// ============================================================================
module m_no_recv_p2 {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(CFG_NO_RECV_P2)
  }

  action step = any {
    st' = nextStateForProver(CFG_NO_RECV_P2, 0, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 1, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 2, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 3, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 4, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 5, 1, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 6, 1, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 7, 1, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 8, 1, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 9, 0, 0),
    st' = nextStateForProver(CFG_NO_RECV_P2, 9, 1, 0)
  }

  val invRow: bool = checkInvRow(CFG_NO_RECV_P2, st)
}

// ============================================================================
// Mutant 2: m_send_wrong_x (send (id, lambda, x2) in place of (id, lambda, x1) to X3)
// ============================================================================
module m_send_wrong_x {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(CFG_SEND_WRONG_X)
  }

  action step = any {
    st' = nextStateForProver(CFG_SEND_WRONG_X, 0, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 1, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 2, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 3, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 4, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 5, 1, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 6, 1, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 7, 1, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 8, 1, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 9, 0, 0),
    st' = nextStateForProver(CFG_SEND_WRONG_X, 9, 1, 0)
  }

  val invRow: bool = checkInvRow(CFG_SEND_WRONG_X, st)
}

// ============================================================================
// Mutant 3: m_no_id (drop id from the message to Y3 on bus o+4)
// ============================================================================
module m_no_id {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(CFG_NO_ID)
  }

  action step = any {
    st' = nextStateForProver(CFG_NO_ID, 0, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 1, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 2, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 3, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 4, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 5, 1, 0),
    st' = nextStateForProver(CFG_NO_ID, 6, 1, 0),
    st' = nextStateForProver(CFG_NO_ID, 7, 1, 0),
    st' = nextStateForProver(CFG_NO_ID, 8, 1, 0),
    st' = nextStateForProver(CFG_NO_ID, 9, 0, 0),
    st' = nextStateForProver(CFG_NO_ID, 9, 1, 0)
  }

  val invRow: bool = checkInvRow(CFG_NO_ID, st)
}

// ============================================================================
// Mutant 4: m_no_div_constraint (lambda free; FieldExpr constraint removed)
// ============================================================================
module m_no_div_constraint {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(CFG_NO_DIV_CONSTRAINT)
  }

  action step = any {
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 0, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 1, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 2, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 3, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 4, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 5, 1, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 6, 1, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 7, 1, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 8, 1, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 9, 0, 0),
    st' = nextStateForProver(CFG_NO_DIV_CONSTRAINT, 9, 1, 0)
  }

  val invRow: bool = checkInvRow(CFG_NO_DIV_CONSTRAINT, st)
}

// ============================================================================
// Mutant 5: m_valid_free (multiplicity 1 in place of is_valid on the sends)
// ============================================================================
module m_valid_free {
  import l_add_core.*

  var st: ChainState

  action init = {
    st' = buildHonestState(CFG_VALID_FREE)
  }

  action step = any {
    st' = nextStateForProver(CFG_VALID_FREE, 0, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 1, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 2, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 3, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 4, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 5, 1, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 6, 1, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 7, 1, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 8, 1, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 9, 0, 0),
    st' = nextStateForProver(CFG_VALID_FREE, 9, 1, 0)
  }

  val invRow: bool = checkInvRow(CFG_VALID_FREE, st)
}
