// Self-contained unit and property tests for the X14 SQ (squaring) AIR model over F_17.

module common {
  pure val P: int = 17

  pure val BUS_O: int = 1110
  pure val BUS_O2: int = 1112
  pure val BUS_O3: int = 1113
  pure val BUS_O4: int = 1114
  pure val BUS_O5: int = 1115
  pure val BUS_O6: int = 1116
  pure val BUS_O7: int = 1117
  pure val BUS_O8: int = 1118
  pure val BUS_O9: int = 1119

  pure def modP(a: int): int = ((a % P) + P) % P
  pure def modAdd(a: int, b: int): int = modP(a + b)
  pure def modSub(a: int, b: int): int = modP(a - b)
  pure def modMul(a: int, b: int): int = modP(a * b)

  pure def modInv(a: int): int = {
    val c = modP(a)
    if (c == 1) 1
    else if (c == 2) 9
    else if (c == 3) 6
    else if (c == 4) 13
    else if (c == 5) 7
    else if (c == 6) 3
    else if (c == 7) 5
    else if (c == 8) 15
    else if (c == 9) 2
    else if (c == 10) 12
    else if (c == 11) 14
    else if (c == 12) 10
    else if (c == 13) 4
    else if (c == 14) 11
    else if (c == 15) 8
    else if (c == 16) 16
    else 0
  }

  pure def modDiv(num: int, den: int): int = modMul(num, modInv(den))

  type CurvePoint = { x: int, y: int }

  pure def onCurve(x: int, y: int): bool =
    modMul(y, y) == modAdd(modMul(modMul(x, x), x), 4)

  pure def affineDouble(x: int, y: int): CurvePoint = {
    val s = modMul(x, x)
    val lam = modDiv(modMul(3, s), modMul(2, y))
    val x3 = modSub(modSub(modMul(lam, lam), x), x)
    val y3 = modSub(modMul(lam, modSub(x, x3)), y)
    { x: x3, y: y3 }
  }

  type SqRow = { isValid: int, rowId: int, xIn: int, sOut: int }
  type LDblRow = { isValid: int, rowId: int, sIn: int, xIn: int, yIn: int, lamOut: int }
  type X3Row = { isValid: int, rowId: int, lamIn: int, x1In: int, x2In: int, x3Out: int }
  type Y3Row = { isValid: int, rowId: int, lamIn: int, x1In: int, x3In: int, y1In: int, y3Out: int }
  type SourceRow = {
    rowId: int, xSrc: int, ySrc: int, x3Out: int, y3Out: int,
    sendO8Mult: int, sendOMult: int, recvO7Mult: int
  }
  type RowBundle = { src: SourceRow, sq: SqRow, l: LDblRow, x3: X3Row, y3: Y3Row }
  type ChainTrace = { b0: RowBundle, b1: RowBundle }

  pure def mkMsg2(a: int, b: int): int = 2000000 + a * 1000 + b
  pure def mkMsg3(a: int, b: int, c: int): int = 3000000 + a * 10000 + b * 100 + c

  type SqConfig = {
    recvO8: bool,
    sendOtherX: bool,
    includeIdOnO9: bool,
    wrongSquare: bool,
    validFreeOnSend: bool
  }

  pure val SOUND_CFG: SqConfig = { recvO8: true, sendOtherX: false, includeIdOnO9: true, wrongSquare: false, validFreeOnSend: false }
  pure val NO_RECV_CFG: SqConfig = { recvO8: false, sendOtherX: false, includeIdOnO9: true, wrongSquare: false, validFreeOnSend: false }
  pure val SEND_OTHER_X_CFG: SqConfig = { recvO8: true, sendOtherX: true, includeIdOnO9: true, wrongSquare: false, validFreeOnSend: false }
  pure val NO_ID_CFG: SqConfig = { recvO8: true, sendOtherX: false, includeIdOnO9: false, wrongSquare: false, validFreeOnSend: false }
  pure val WRONG_SQUARE_CFG: SqConfig = { recvO8: true, sendOtherX: false, includeIdOnO9: true, wrongSquare: true, validFreeOnSend: false }
  pure val VALID_FREE_CFG: SqConfig = { recvO8: true, sendOtherX: false, includeIdOnO9: true, wrongSquare: false, validFreeOnSend: true }

  pure def isBool(v: int): bool = (v == 0 or v == 1)
  pure def inField(v: int): bool = (0 <= v and v < P)

  pure def sqRowValid(cfg: SqConfig, row: SqRow): bool =
    if (row.isValid == 0)
      true
    else if (row.isValid != 1)
      false
    else if (not(inField(row.sOut)))
      false
    else if (cfg.wrongSquare)
      (row.sOut == modAdd(modMul(row.xIn, row.xIn), row.xIn))
    else
      (row.sOut == modMul(row.xIn, row.xIn))

  pure def lRowValid(row: LDblRow): bool =
    if (row.isValid == 0)
      true
    else if (row.isValid != 1)
      false
    else (
      inField(row.lamOut) and
      modMul(modMul(2, row.yIn), row.lamOut) == modMul(3, row.sIn)
    )

  pure def x3RowValid(row: X3Row): bool =
    if (row.isValid == 0)
      true
    else if (row.isValid != 1)
      false
    else (
      inField(row.x3Out) and
      row.x3Out == modSub(modSub(modMul(row.lamIn, row.lamIn), row.x1In), row.x2In)
    )

  pure def y3RowValid(row: Y3Row): bool =
    if (row.isValid == 0)
      true
    else if (row.isValid != 1)
      false
    else (
      inField(row.y3Out) and
      row.y3Out == modSub(modMul(row.lamIn, modSub(row.x1In, row.x3In)), row.y1In)
    )

  pure def netMultOnBus(target: int, s0: int, ms0: int, s1: int, ms1: int, r0: int, mr0: int, r1: int, mr1: int): int =
    (if (s0 == target) ms0 else 0) + (if (s1 == target) ms1 else 0) - (if (r0 == target) mr0 else 0) - (if (r1 == target) mr1 else 0)

  pure def pairBusBalanced(s0: int, ms0: int, s1: int, ms1: int, r0: int, mr0: int, r1: int, mr1: int): bool =
    if (ms0 == 1 and ms1 == 1 and mr0 == 1 and mr1 == 1)
      ((s0 == r0 and s1 == r1) or (s0 == r1 and s1 == r0))
    else (
      netMultOnBus(s0, s0, ms0, s1, ms1, r0, mr0, r1, mr1) == 0 and
      netMultOnBus(s1, s0, ms0, s1, ms1, r0, mr0, r1, mr1) == 0 and
      netMultOnBus(r0, s0, ms0, s1, ms1, r0, mr0, r1, mr1) == 0 and
      netMultOnBus(r1, s0, ms0, s1, ms1, r0, mr0, r1, mr1) == 0
    )

  pure def sqO9Msg(cfg: SqConfig, row: SqRow): int = {
    val xSent = if (cfg.sendOtherX) row.sOut else row.xIn
    if (cfg.includeIdOnO9) mkMsg3(row.rowId, row.sOut, xSent) else mkMsg2(row.sOut, xSent)
  }

  pure def sqO9Mult(cfg: SqConfig, row: SqRow): int =
    if (cfg.validFreeOnSend) 1 else row.isValid

  pure def lO9Msg(cfg: SqConfig, row: LDblRow): int =
    if (cfg.includeIdOnO9) mkMsg3(row.rowId, row.sIn, row.xIn) else mkMsg2(row.sIn, row.xIn)

  pure def allBusesBalanced(cfg: SqConfig, tr: ChainTrace): bool = {
    val b0 = tr.b0
    val b1 = tr.b1
    if (
      cfg.recvO8 and not(pairBusBalanced(
        mkMsg2(b0.src.rowId, b0.src.xSrc), b0.src.sendO8Mult,
        mkMsg2(b1.src.rowId, b1.src.xSrc), b1.src.sendO8Mult,
        mkMsg2(b0.sq.rowId, b0.sq.xIn), b0.sq.isValid,
        mkMsg2(b1.sq.rowId, b1.sq.xIn), b1.sq.isValid
      ))
    )
      false
    else if (
      not(pairBusBalanced(
        sqO9Msg(cfg, b0.sq), sqO9Mult(cfg, b0.sq),
        sqO9Msg(cfg, b1.sq), sqO9Mult(cfg, b1.sq),
        lO9Msg(cfg, b0.l), b0.l.isValid,
        lO9Msg(cfg, b1.l), b1.l.isValid
      ))
    )
      false
    else and {
      pairBusBalanced(
        mkMsg2(b0.src.rowId, b0.src.ySrc), b0.src.sendOMult,
        mkMsg2(b1.src.rowId, b1.src.ySrc), b1.src.sendOMult,
        mkMsg2(b0.l.rowId, b0.l.yIn), b0.l.isValid,
        mkMsg2(b1.l.rowId, b1.l.yIn), b1.l.isValid
      ),
      pairBusBalanced(
        mkMsg3(b0.l.rowId, b0.l.lamOut, b0.l.xIn), b0.l.isValid,
        mkMsg3(b1.l.rowId, b1.l.lamOut, b1.l.xIn), b1.l.isValid,
        mkMsg3(b0.x3.rowId, b0.x3.lamIn, b0.x3.x1In), b0.x3.isValid,
        mkMsg3(b1.x3.rowId, b1.x3.lamIn, b1.x3.x1In), b1.x3.isValid
      ),
      pairBusBalanced(
        mkMsg2(b0.l.rowId, b0.l.xIn), b0.l.isValid,
        mkMsg2(b1.l.rowId, b1.l.xIn), b1.l.isValid,
        mkMsg2(b0.x3.rowId, b0.x3.x2In), b0.x3.isValid,
        mkMsg2(b1.x3.rowId, b1.x3.x2In), b1.x3.isValid
      ),
      pairBusBalanced(
        mkMsg3(b0.l.rowId, b0.l.lamOut, b0.l.xIn), b0.l.isValid,
        mkMsg3(b1.l.rowId, b1.l.lamOut, b1.l.xIn), b1.l.isValid,
        mkMsg3(b0.y3.rowId, b0.y3.lamIn, b0.y3.x1In), b0.y3.isValid,
        mkMsg3(b1.y3.rowId, b1.y3.lamIn, b1.y3.x1In), b1.y3.isValid
      ),
      pairBusBalanced(
        mkMsg2(b0.l.rowId, b0.l.yIn), b0.l.isValid,
        mkMsg2(b1.l.rowId, b1.l.yIn), b1.l.isValid,
        mkMsg2(b0.y3.rowId, b0.y3.y1In), b0.y3.isValid,
        mkMsg2(b1.y3.rowId, b1.y3.y1In), b1.y3.isValid
      ),
      pairBusBalanced(
        mkMsg2(b0.x3.rowId, b0.x3.x3Out), b0.x3.isValid,
        mkMsg2(b1.x3.rowId, b1.x3.x3Out), b1.x3.isValid,
        mkMsg2(b0.y3.rowId, b0.y3.x3In), b0.y3.isValid,
        mkMsg2(b1.y3.rowId, b1.y3.x3In), b1.y3.isValid
      ),
      pairBusBalanced(
        mkMsg3(b0.y3.rowId, b0.y3.x3In, b0.y3.y3Out), b0.y3.isValid,
        mkMsg3(b1.y3.rowId, b1.y3.x3In, b1.y3.y3Out), b1.y3.isValid,
        mkMsg3(b0.src.rowId, b0.src.x3Out, b0.src.y3Out), b0.src.recvO7Mult,
        mkMsg3(b1.src.rowId, b1.src.x3Out, b1.src.y3Out), b1.src.recvO7Mult
      ),
    }
  }

  pure def traceAccepted(cfg: SqConfig, tr: ChainTrace): bool =
    if (not(sqRowValid(cfg, tr.b0.sq)) or not(sqRowValid(cfg, tr.b1.sq)))
      false
    else if (not(allBusesBalanced(cfg, tr)))
      false
    else (
      lRowValid(tr.b0.l) and
      x3RowValid(tr.b0.x3) and
      y3RowValid(tr.b0.y3) and
      lRowValid(tr.b1.l) and
      x3RowValid(tr.b1.x3) and
      y3RowValid(tr.b1.y3)
    )

  pure val ID1: int = 1
  pure val X1: int = 6
  pure val Y1: int = 4
  pure val S1: int = 2
  pure val LAM1: int = 5
  pure val X3_1: int = 13
  pure val Y3_1: int = 12
  pure val S_WRONG_1: int = 8

  pure val ID2: int = 2
  pure val X2: int = 9
  pure val Y2: int = 6
  pure val S2: int = 13
  pure val LAM2: int = 16
  pure val X3_2: int = 0
  pure val Y3_2: int = 2
  pure val S_WRONG_2: int = 5

  pure val DBL_1: CurvePoint = { x: X3_1, y: Y3_1 }
  pure val DBL_2: CurvePoint = { x: X3_2, y: Y3_2 }

  pure def sqRowMatchesProducer(sq: SqRow, s0: SourceRow, s1: SourceRow): bool =
    if (sq.isValid == 0) true
    else (
      (s0.rowId == sq.rowId and s0.sendO8Mult == 1 and sq.xIn == s0.xSrc and sq.sOut == modMul(s0.xSrc, s0.xSrc)) or
      (s1.rowId == sq.rowId and s1.sendO8Mult == 1 and sq.xIn == s1.xSrc and sq.sOut == modMul(s1.xSrc, s1.xSrc))
    )

  pure def srcRowDoublesCorrectly(src: SourceRow): bool =
    if (src.recvO7Mult == 0 or src.ySrc == 0) true
    else if (src.xSrc == X1 and src.ySrc == Y1) (src.x3Out == X3_1 and src.y3Out == Y3_1)
    else if (src.xSrc == X2 and src.ySrc == Y2) (src.x3Out == X3_2 and src.y3Out == Y3_2)
    else {
      val dbl = affineDouble(src.xSrc, src.ySrc)
      (src.x3Out == dbl.x and src.y3Out == dbl.y)
    }

  pure def invRowProps(tr: ChainTrace): bool = (
    sqRowMatchesProducer(tr.b0.sq, tr.b0.src, tr.b1.src) and
    sqRowMatchesProducer(tr.b1.sq, tr.b0.src, tr.b1.src) and
    srcRowDoublesCorrectly(tr.b0.src) and
    srcRowDoublesCorrectly(tr.b1.src)
  )

  pure def checkInvRow(cfg: SqConfig, tr: ChainTrace): bool =
    if (invRowProps(tr)) true
    else not(traceAccepted(cfg, tr))

  pure val HONEST_BUNDLE_1: RowBundle = {
    src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
    sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
    l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
    x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
    y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
  }

  pure val HONEST_BUNDLE_2: RowBundle = {
    src: { rowId: ID2, xSrc: X2, ySrc: Y2, x3Out: X3_2, y3Out: Y3_2, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
    sq: { isValid: 1, rowId: ID2, xIn: X2, sOut: S2 },
    l: { isValid: 1, rowId: ID2, sIn: S2, xIn: X2, yIn: Y2, lamOut: LAM2 },
    x3: { isValid: 1, rowId: ID2, lamIn: LAM2, x1In: X2, x2In: X2, x3Out: X3_2 },
    y3: { isValid: 1, rowId: ID2, lamIn: LAM2, x1In: X2, x3In: X3_2, y1In: Y2, y3Out: Y3_2 }
  }

  pure val INACTIVE_BUNDLE_2: RowBundle = {
    src: { rowId: ID2, xSrc: X2, ySrc: Y2, x3Out: 0, y3Out: 0, sendO8Mult: 0, sendOMult: 0, recvO7Mult: 0 },
    sq: { isValid: 0, rowId: 0, xIn: 0, sOut: 0 },
    l: { isValid: 0, rowId: 0, sIn: 0, xIn: 0, yIn: 0, lamOut: 0 },
    x3: { isValid: 0, rowId: 0, lamIn: 0, x1In: 0, x2In: 0, x3Out: 0 },
    y3: { isValid: 0, rowId: 0, lamIn: 0, x1In: 0, x3In: 0, y1In: 0, y3Out: 0 }
  }

  pure val HONEST_TRACE: ChainTrace = { b0: HONEST_BUNDLE_1, b1: HONEST_BUNDLE_2 }
  pure val PADDING_TRACE: ChainTrace = { b0: HONEST_BUNDLE_1, b1: INACTIVE_BUNDLE_2 }

  pure val TAMPER_SQ_X_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X2, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val TAMPER_SQ_S_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S_WRONG_1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val TAMPER_SQ_ID_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: 99, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val TAMPER_SQ_VALID0_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 0, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val TAMPER_SQ_VALID2_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 2, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val DROP_O8_MSG_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 0, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val DUP_O8_MSG_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: X3_1, y3Out: Y3_1, sendO8Mult: 2, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y1, lamOut: LAM1 },
      x3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x2In: X1, x3Out: X3_1 },
      y3: { isValid: 1, rowId: ID1, lamIn: LAM1, x1In: X1, x3In: X3_1, y1In: Y1, y3Out: Y3_1 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val EXPLOIT_NO_RECV: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 14, y3Out: 12, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X2, sOut: S2 },
      l: { isValid: 1, rowId: ID1, sIn: S2, xIn: X2, yIn: Y1, lamOut: 7 },
      x3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x2In: X2, x3Out: 14 },
      y3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x3In: 14, y1In: Y1, y3Out: 12 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val EXPLOIT_SEND_OTHER_X: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 4, y3Out: 3, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: S1, yIn: Y1, lamOut: 5 },
      x3: { isValid: 1, rowId: ID1, lamIn: 5, x1In: S1, x2In: S1, x3Out: 4 },
      y3: { isValid: 1, rowId: ID1, lamIn: 5, x1In: S1, x3In: 4, y1In: Y1, y3Out: 3 }
    },
    b1: {
      src: { rowId: ID2, xSrc: X2, ySrc: Y2, x3Out: 9, y3Out: 7, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID2, xIn: X2, sOut: S2 },
      l: { isValid: 1, rowId: ID2, sIn: S2, xIn: S2, yIn: Y2, lamOut: 16 },
      x3: { isValid: 1, rowId: ID2, lamIn: 16, x1In: S2, x2In: S2, x3Out: 9 },
      y3: { isValid: 1, rowId: ID2, lamIn: 16, x1In: S2, x3In: 9, y1In: Y2, y3Out: 7 }
    }
  }

  pure val EXPLOIT_NO_ID: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 14, y3Out: 12, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S2, xIn: X2, yIn: Y1, lamOut: 7 },
      x3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x2In: X2, x3Out: 14 },
      y3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x3In: 14, y1In: Y1, y3Out: 12 }
    },
    b1: {
      src: { rowId: ID2, xSrc: X2, ySrc: Y2, x3Out: 1, y3Out: 5, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID2, xIn: X2, sOut: S2 },
      l: { isValid: 1, rowId: ID2, sIn: S1, xIn: X1, yIn: Y2, lamOut: 9 },
      x3: { isValid: 1, rowId: ID2, lamIn: 9, x1In: X1, x2In: X1, x3Out: 1 },
      y3: { isValid: 1, rowId: ID2, lamIn: 9, x1In: X1, x3In: 1, y1In: Y2, y3Out: 5 }
    }
  }

  pure val EXPLOIT_WRONG_SQUARE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 14, y3Out: 6, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S_WRONG_1 },
      l: { isValid: 1, rowId: ID1, sIn: S_WRONG_1, xIn: X1, yIn: Y1, lamOut: 3 },
      x3: { isValid: 1, rowId: ID1, lamIn: 3, x1In: X1, x2In: X1, x3Out: 14 },
      y3: { isValid: 1, rowId: ID1, lamIn: 3, x1In: X1, x3In: 14, y1In: Y1, y3Out: 6 }
    },
    b1: {
      src: { rowId: ID2, xSrc: X2, ySrc: Y2, x3Out: 8, y3Out: 8, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID2, xIn: X2, sOut: S_WRONG_2 },
      l: { isValid: 1, rowId: ID2, sIn: S_WRONG_2, xIn: X2, yIn: Y2, lamOut: 14 },
      x3: { isValid: 1, rowId: ID2, lamIn: 14, x1In: X2, x2In: X2, x3Out: 8 },
      y3: { isValid: 1, rowId: ID2, lamIn: 14, x1In: X2, x3In: 8, y1In: Y2, y3Out: 8 }
    }
  }

  pure val EXPLOIT_VALID_FREE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 14, y3Out: 6, sendO8Mult: 0, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 0, rowId: ID1, xIn: X1, sOut: S_WRONG_1 },
      l: { isValid: 1, rowId: ID1, sIn: S_WRONG_1, xIn: X1, yIn: Y1, lamOut: 3 },
      x3: { isValid: 1, rowId: ID1, lamIn: 3, x1In: X1, x2In: X1, x3Out: 14 },
      y3: { isValid: 1, rowId: ID1, lamIn: 3, x1In: X1, x3In: 14, y1In: Y1, y3Out: 6 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val DUPLICATE_ID_SWAP_TRACE: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: X1, ySrc: Y1, x3Out: 14, y3Out: 12, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X1, sOut: S1 },
      l: { isValid: 1, rowId: ID1, sIn: S2, xIn: X2, yIn: Y1, lamOut: 7 },
      x3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x2In: X2, x3Out: 14 },
      y3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: X2, x3In: 14, y1In: Y1, y3Out: 12 }
    },
    b1: {
      src: { rowId: ID1, xSrc: X2, ySrc: Y2, x3Out: 1, y3Out: 5, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: X2, sOut: S2 },
      l: { isValid: 1, rowId: ID1, sIn: S1, xIn: X1, yIn: Y2, lamOut: 9 },
      x3: { isValid: 1, rowId: ID1, lamIn: 9, x1In: X1, x2In: X1, x3Out: 1 },
      y3: { isValid: 1, rowId: ID1, lamIn: 9, x1In: X1, x3In: 1, y1In: Y2, y3Out: 5 }
    }
  }

  pure val ZERO_FORGERY_2: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: 0, ySrc: 0, x3Out: 4, y3Out: 9, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: 0, sOut: 0 },
      l: { isValid: 1, rowId: ID1, sIn: 0, xIn: 0, yIn: 0, lamOut: 2 },
      x3: { isValid: 1, rowId: ID1, lamIn: 2, x1In: 0, x2In: 0, x3Out: 4 },
      y3: { isValid: 1, rowId: ID1, lamIn: 2, x1In: 0, x3In: 4, y1In: 0, y3Out: 9 }
    },
    b1: HONEST_BUNDLE_2
  }

  pure val ZERO_FORGERY_7: ChainTrace = {
    b0: {
      src: { rowId: ID1, xSrc: 0, ySrc: 0, x3Out: 15, y3Out: 14, sendO8Mult: 1, sendOMult: 1, recvO7Mult: 1 },
      sq: { isValid: 1, rowId: ID1, xIn: 0, sOut: 0 },
      l: { isValid: 1, rowId: ID1, sIn: 0, xIn: 0, yIn: 0, lamOut: 7 },
      x3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: 0, x2In: 0, x3Out: 15 },
      y3: { isValid: 1, rowId: ID1, lamIn: 7, x1In: 0, x3In: 15, y1In: 0, y3Out: 14 }
    },
    b1: HONEST_BUNDLE_2
  }

  // Memoized pure vals for fast unit test evaluation across arbitrary --max-samples
  pure val COMPLETENESS_OK: bool = and {
    onCurve(X1, Y1),
    onCurve(X2, Y2),
    affineDouble(X1, Y1) == DBL_1,
    affineDouble(X2, Y2) == DBL_2,
    onCurve(DBL_1.x, DBL_1.y),
    onCurve(DBL_2.x, DBL_2.y),
    traceAccepted(SOUND_CFG, HONEST_TRACE),
    checkInvRow(SOUND_CFG, HONEST_TRACE),
    traceAccepted(SOUND_CFG, PADDING_TRACE),
    checkInvRow(SOUND_CFG, PADDING_TRACE),
  }

  pure val SOUNDNESS_REJECTION_OK: bool = and {
    not(traceAccepted(SOUND_CFG, TAMPER_SQ_X_TRACE)),
    not(traceAccepted(SOUND_CFG, TAMPER_SQ_S_TRACE)),
    not(traceAccepted(SOUND_CFG, TAMPER_SQ_ID_TRACE)),
    not(traceAccepted(SOUND_CFG, TAMPER_SQ_VALID0_TRACE)),
    not(traceAccepted(SOUND_CFG, TAMPER_SQ_VALID2_TRACE)),
    not(traceAccepted(SOUND_CFG, DROP_O8_MSG_TRACE)),
    not(traceAccepted(SOUND_CFG, DUP_O8_MSG_TRACE)),
    traceAccepted(SOUND_CFG, DUPLICATE_ID_SWAP_TRACE),
    not(checkInvRow(SOUND_CFG, DUPLICATE_ID_SWAP_TRACE)),
    traceAccepted(SOUND_CFG, ZERO_FORGERY_2),
    traceAccepted(SOUND_CFG, ZERO_FORGERY_7),
    ZERO_FORGERY_2.b0.src.x3Out != ZERO_FORGERY_7.b0.src.x3Out,
  }

  pure val MUTANT_NO_RECV_CAUGHT: bool = and {
    not(traceAccepted(SOUND_CFG, EXPLOIT_NO_RECV)),
    traceAccepted(NO_RECV_CFG, EXPLOIT_NO_RECV),
    not(checkInvRow(NO_RECV_CFG, EXPLOIT_NO_RECV)),
  }

  pure val MUTANT_SEND_OTHER_X_CAUGHT: bool = and {
    not(traceAccepted(SOUND_CFG, EXPLOIT_SEND_OTHER_X)),
    traceAccepted(SEND_OTHER_X_CFG, EXPLOIT_SEND_OTHER_X),
    not(checkInvRow(SEND_OTHER_X_CFG, EXPLOIT_SEND_OTHER_X)),
  }

  pure val MUTANT_NO_ID_CAUGHT: bool = and {
    not(traceAccepted(SOUND_CFG, EXPLOIT_NO_ID)),
    traceAccepted(NO_ID_CFG, EXPLOIT_NO_ID),
    not(checkInvRow(NO_ID_CFG, EXPLOIT_NO_ID)),
  }

  pure val MUTANT_WRONG_SQUARE_CAUGHT: bool = and {
    not(traceAccepted(SOUND_CFG, EXPLOIT_WRONG_SQUARE)),
    traceAccepted(WRONG_SQUARE_CFG, EXPLOIT_WRONG_SQUARE),
    not(checkInvRow(WRONG_SQUARE_CFG, EXPLOIT_WRONG_SQUARE)),
  }

  pure val MUTANT_VALID_FREE_CAUGHT: bool = and {
    not(traceAccepted(SOUND_CFG, EXPLOIT_VALID_FREE)),
    traceAccepted(VALID_FREE_CFG, EXPLOIT_VALID_FREE),
    not(checkInvRow(VALID_FREE_CFG, EXPLOIT_VALID_FREE)),
  }
}

module x14_sq_test {
  import common.*

  var curTrace: ChainTrace

  action init = all {
    curTrace' = HONEST_TRACE
  }

  action step = all {
    curTrace' = HONEST_TRACE
  }

  val invRow: bool = checkInvRow(SOUND_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(SOUND_CFG, curTrace) and curTrace.b0.sq.isValid == 1)

  run completenessTest = init.expect(COMPLETENESS_OK)
  run soundnessRejectionTest = init.expect(SOUNDNESS_REJECTION_OK)
  run mutant_m_no_recv_MutantNoRecvCaughtTest = init.expect(MUTANT_NO_RECV_CAUGHT)
  run mutant_m_send_other_x_MutantSendOtherXCaughtTest = init.expect(MUTANT_SEND_OTHER_X_CAUGHT)
  run mutant_m_no_id_MutantNoIdCaughtTest = init.expect(MUTANT_NO_ID_CAUGHT)
  run mutant_m_wrong_square_MutantWrongSquareCaughtTest = init.expect(MUTANT_WRONG_SQUARE_CAUGHT)
  run mutant_m_valid_free_MutantValidFreeCaughtTest = init.expect(MUTANT_VALID_FREE_CAUGHT)
}

module sq_test {
  import common.*

  var curTrace: ChainTrace

  action init = all {
    curTrace' = HONEST_TRACE
  }

  action step = all {
    curTrace' = HONEST_TRACE
  }

  val invRow: bool = checkInvRow(SOUND_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(SOUND_CFG, curTrace) and curTrace.b0.sq.isValid == 1)

  run completenessTest = init.expect(COMPLETENESS_OK)
  run soundnessRejectionTest = init.expect(SOUNDNESS_REJECTION_OK)
  run mutant_m_no_recv_MutantNoRecvCaughtTest = init.expect(MUTANT_NO_RECV_CAUGHT)
  run mutant_m_send_other_x_MutantSendOtherXCaughtTest = init.expect(MUTANT_SEND_OTHER_X_CAUGHT)
  run mutant_m_no_id_MutantNoIdCaughtTest = init.expect(MUTANT_NO_ID_CAUGHT)
  run mutant_m_wrong_square_MutantWrongSquareCaughtTest = init.expect(MUTANT_WRONG_SQUARE_CAUGHT)
  run mutant_m_valid_free_MutantValidFreeCaughtTest = init.expect(MUTANT_VALID_FREE_CAUGHT)
}

module m_no_recv {
  import common.*

  var curTrace: ChainTrace

  action init = all { curTrace' = HONEST_TRACE }
  action step = all { curTrace' = EXPLOIT_NO_RECV }

  val invRow: bool = checkInvRow(NO_RECV_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(NO_RECV_CFG, curTrace) and curTrace.b0.sq.isValid == 1)
}

module m_send_other_x {
  import common.*

  var curTrace: ChainTrace

  action init = all { curTrace' = HONEST_TRACE }
  action step = all { curTrace' = EXPLOIT_SEND_OTHER_X }

  val invRow: bool = checkInvRow(SEND_OTHER_X_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(SEND_OTHER_X_CFG, curTrace) and curTrace.b0.sq.isValid == 1)
}

module m_no_id {
  import common.*

  var curTrace: ChainTrace

  action init = all { curTrace' = HONEST_TRACE }
  action step = all { curTrace' = EXPLOIT_NO_ID }

  val invRow: bool = checkInvRow(NO_ID_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(NO_ID_CFG, curTrace) and curTrace.b0.sq.isValid == 1)
}

module m_wrong_square {
  import common.*

  var curTrace: ChainTrace

  action init = all { curTrace' = HONEST_TRACE }
  action step = all { curTrace' = EXPLOIT_WRONG_SQUARE }

  val invRow: bool = checkInvRow(WRONG_SQUARE_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(WRONG_SQUARE_CFG, curTrace) and curTrace.b0.sq.isValid == 1)
}

module m_valid_free {
  import common.*

  var curTrace: ChainTrace

  action init = all { curTrace' = HONEST_TRACE }
  action step = all { curTrace' = EXPLOIT_VALID_FREE }

  val invRow: bool = checkInvRow(VALID_FREE_CFG, curTrace)
  val allInvariants: bool = invRow
  val invNonVacuous: bool = not(traceAccepted(VALID_FREE_CFG, curTrace) and curTrace.b1.sq.isValid == 1)
}
