// -*- mode: Bluespec; -*-
// Unit and trace tests for `L_dbl` soundness, completeness, non-vacuity,
// required mutants (`m_no_recv_s`, `m_send_y_as_x2`, `m_wrong_const`, `m_no_id`, `m_valid_free`),
// and analytical findings (duplicate-id input swap and (0, 0) unconstrained-lambda).

module l_dbl_test {
  pure val P: int = 13

  pure def modP(a: int): int = {
    ((a % P) + P) % P
  }

  pure def addMod(a: int, b: int): int = {
    modP(a + b)
  }

  pure def subMod(a: int, b: int): int = {
    modP(a - b)
  }

  pure def mulMod(a: int, b: int): int = {
    modP(a * b)
  }

  pure def invMod(a: int): int = {
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

  pure def onCurve(x: int, y: int): bool = {
    mulMod(y, y) == addMod(mulMod(x, mulMod(x, x)), 4)
  }

  pure def specLam(x: int, y: int): int = {
    mulMod(mulMod(3, mulMod(x, x)), invMod(mulMod(2, y)))
  }

  pure def specX3(x: int, y: int): int = {
    val l = specLam(x, y)
    subMod(subMod(mulMod(l, l), x), x)
  }

  pure def specY3(x: int, y: int): int = {
    val l = specLam(x, y)
    val x3 = specX3(x, y)
    subMod(mulMod(l, subMod(x, x3)), y)
  }

  type ProducerRow = {
    rowId: int,
    x: int,
    y: int,
    x3: int,
    y3: int,
    inValid: int,
    outValid: int
  }

  type SqRow = {
    rowId: int,
    x: int,
    s: int,
    isValid: int
  }

  type LDblRow = {
    rowId: int,
    s: int,
    x: int,
    y: int,
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

  type BusKey = {
    tagId: int,
    v0: int,
    v1: int
  }

  type ChainState = {
    prod0: ProducerRow,
    prod1: ProducerRow,
    sq0: SqRow,
    sq1: SqRow,
    l0: LDblRow,
    l1: LDblRow,
    x3_0: X3Row,
    x3_1: X3Row,
    y3_0: Y3Row,
    y3_1: Y3Row,
    o0ExtraDelta: int
  }

  pure def isBit(b: int): bool = {
    b == 0 or b == 1
  }

  pure def inField(a: int): bool = {
    0 <= a and a < P
  }

  pure def validProducerRow(r: ProducerRow): bool = {
    isBit(r.inValid) and isBit(r.outValid) and
    inField(r.x) and inField(r.y) and inField(r.x3) and inField(r.y3) and
    r.y != 0
  }

  pure def validSqRow(r: SqRow): bool = {
    isBit(r.isValid) and inField(r.x) and inField(r.s) and
    (r.isValid == 1 implies r.s == mulMod(r.x, r.x))
  }

  pure def validLDblRow(r: LDblRow, wrongConst: bool): bool = {
    val coeff = if (wrongConst) 2 else 3
    isBit(r.isValid) and
    inField(r.s) and inField(r.x) and inField(r.y) and inField(r.lam) and
    (r.isValid == 1 implies mulMod(r.lam, mulMod(2, r.y)) == mulMod(coeff, r.s))
  }

  pure def validX3Row(r: X3Row): bool = {
    isBit(r.isValid) and
    inField(r.lam) and inField(r.x1) and inField(r.x2) and inField(r.x3) and
    (r.isValid == 1 implies r.x3 == subMod(subMod(mulMod(r.lam, r.lam), r.x1), r.x2))
  }

  pure def validY3Row(r: Y3Row): bool = {
    isBit(r.isValid) and
    inField(r.lam) and inField(r.x1) and inField(r.x3) and inField(r.y1) and inField(r.y3) and
    (r.isValid == 1 implies r.y3 == subMod(mulMod(r.lam, subMod(r.x1, r.x3)), r.y1))
  }

  pure def netBusCount(
    k: BusKey,
    kA: BusKey, mA: int,
    kB: BusKey, mB: int,
    kC: BusKey, mC: int,
    kD: BusKey, mD: int,
    extraA: int
  ): int = {
    (if (k == kA) (mA + extraA) else 0) +
    (if (k == kB) mB else 0) -
    (if (k == kC) mC else 0) -
    (if (k == kD) mD else 0)
  }

  pure def busBalanced2(
    kA: BusKey, mA: int,
    kB: BusKey, mB: int,
    kC: BusKey, mC: int,
    kD: BusKey, mD: int,
    extraA: int
  ): bool = {
    netBusCount(kA, kA, mA, kB, mB, kC, mC, kD, mD, extraA) == 0 and
    netBusCount(kB, kA, mA, kB, mB, kC, mC, kD, mD, extraA) == 0 and
    netBusCount(kC, kA, mA, kB, mB, kC, mC, kD, mD, extraA) == 0 and
    netBusCount(kD, kA, mA, kB, mB, kC, mC, kD, mD, extraA) == 0
  }

  pure def isBusBalanced(
    st: ChainState,
    recvS: bool,
    sendYAsX2: bool,
    noIdO5: bool,
    validFree: bool
  ): bool = {
    val lSendMult0 = if (validFree) 1 else st.l0.isValid
    val lSendMult1 = if (validFree) 1 else st.l1.isValid

    val lX2_0 = if (sendYAsX2) st.l0.y else st.l0.x
    val lX2_1 = if (sendYAsX2) st.l1.y else st.l1.x

    val lO5Id0 = if (noIdO5) -1 else st.l0.rowId
    val lO5Id1 = if (noIdO5) -1 else st.l1.rowId
    val yO5Id0 = if (noIdO5) -1 else st.y3_0.rowId
    val yO5Id1 = if (noIdO5) -1 else st.y3_1.rowId

    val okO8 = busBalanced2(
      { tagId: st.prod0.rowId, v0: st.prod0.x, v1: -1 }, st.prod0.inValid,
      { tagId: st.prod1.rowId, v0: st.prod1.x, v1: -1 }, st.prod1.inValid,
      { tagId: st.sq0.rowId, v0: st.sq0.x, v1: -1 }, st.sq0.isValid,
      { tagId: st.sq1.rowId, v0: st.sq1.x, v1: -1 }, st.sq1.isValid,
      0
    )

    val okO9 = if (recvS) {
      busBalanced2(
        { tagId: st.sq0.rowId, v0: st.sq0.s, v1: st.sq0.x }, st.sq0.isValid,
        { tagId: st.sq1.rowId, v0: st.sq1.s, v1: st.sq1.x }, st.sq1.isValid,
        { tagId: st.l0.rowId, v0: st.l0.s, v1: st.l0.x }, st.l0.isValid,
        { tagId: st.l1.rowId, v0: st.l1.s, v1: st.l1.x }, st.l1.isValid,
        0
      )
    } else {
      true
    }

    val okO0 = busBalanced2(
      { tagId: st.prod0.rowId, v0: st.prod0.y, v1: -1 }, st.prod0.inValid,
      { tagId: st.prod1.rowId, v0: st.prod1.y, v1: -1 }, st.prod1.inValid,
      { tagId: st.l0.rowId, v0: st.l0.y, v1: -1 }, st.l0.isValid,
      { tagId: st.l1.rowId, v0: st.l1.y, v1: -1 }, st.l1.isValid,
      st.o0ExtraDelta
    )

    val okO2 = busBalanced2(
      { tagId: st.l0.rowId, v0: st.l0.lam, v1: st.l0.x }, lSendMult0,
      { tagId: st.l1.rowId, v0: st.l1.lam, v1: st.l1.x }, lSendMult1,
      { tagId: st.x3_0.rowId, v0: st.x3_0.lam, v1: st.x3_0.x1 }, st.x3_0.isValid,
      { tagId: st.x3_1.rowId, v0: st.x3_1.lam, v1: st.x3_1.x1 }, st.x3_1.isValid,
      0
    )

    val okO3 = busBalanced2(
      { tagId: st.l0.rowId, v0: lX2_0, v1: -1 }, lSendMult0,
      { tagId: st.l1.rowId, v0: lX2_1, v1: -1 }, lSendMult1,
      { tagId: st.x3_0.rowId, v0: st.x3_0.x2, v1: -1 }, st.x3_0.isValid,
      { tagId: st.x3_1.rowId, v0: st.x3_1.x2, v1: -1 }, st.x3_1.isValid,
      0
    )

    val okO4 = busBalanced2(
      { tagId: st.l0.rowId, v0: st.l0.lam, v1: st.l0.x }, lSendMult0,
      { tagId: st.l1.rowId, v0: st.l1.lam, v1: st.l1.x }, lSendMult1,
      { tagId: st.y3_0.rowId, v0: st.y3_0.lam, v1: st.y3_0.x1 }, st.y3_0.isValid,
      { tagId: st.y3_1.rowId, v0: st.y3_1.lam, v1: st.y3_1.x1 }, st.y3_1.isValid,
      0
    )

    val okO5 = busBalanced2(
      { tagId: lO5Id0, v0: st.l0.y, v1: -1 }, lSendMult0,
      { tagId: lO5Id1, v0: st.l1.y, v1: -1 }, lSendMult1,
      { tagId: yO5Id0, v0: st.y3_0.y1, v1: -1 }, st.y3_0.isValid,
      { tagId: yO5Id1, v0: st.y3_1.y1, v1: -1 }, st.y3_1.isValid,
      0
    )

    val okO6 = busBalanced2(
      { tagId: st.x3_0.rowId, v0: st.x3_0.x3, v1: -1 }, st.x3_0.isValid,
      { tagId: st.x3_1.rowId, v0: st.x3_1.x3, v1: -1 }, st.x3_1.isValid,
      { tagId: st.y3_0.rowId, v0: st.y3_0.x3, v1: -1 }, st.y3_0.isValid,
      { tagId: st.y3_1.rowId, v0: st.y3_1.x3, v1: -1 }, st.y3_1.isValid,
      0
    )

    val okO7 = busBalanced2(
      { tagId: st.y3_0.rowId, v0: st.y3_0.x3, v1: st.y3_0.y3 }, st.y3_0.isValid,
      { tagId: st.y3_1.rowId, v0: st.y3_1.x3, v1: st.y3_1.y3 }, st.y3_1.isValid,
      { tagId: st.prod0.rowId, v0: st.prod0.x3, v1: st.prod0.y3 }, st.prod0.outValid,
      { tagId: st.prod1.rowId, v0: st.prod1.x3, v1: st.prod1.y3 }, st.prod1.outValid,
      0
    )

    okO8 and okO9 and okO0 and okO2 and okO3 and okO4 and okO5 and okO6 and okO7
  }

  pure def chainAccepted(
    st: ChainState,
    recvS: bool,
    sendYAsX2: bool,
    wrongConst: bool,
    noIdO5: bool,
    validFree: bool
  ): bool = {
    st.prod0.rowId != st.prod1.rowId and
    validProducerRow(st.prod0) and validProducerRow(st.prod1) and
    validSqRow(st.sq0) and validSqRow(st.sq1) and
    validLDblRow(st.l0, wrongConst) and validLDblRow(st.l1, wrongConst) and
    validX3Row(st.x3_0) and validX3Row(st.x3_1) and
    validY3Row(st.y3_0) and validY3Row(st.y3_1) and
    isBusBalanced(st, recvS, sendYAsX2, noIdO5, validFree)
  }

  pure def rowMatchesProducer(l: LDblRow, p: ProducerRow): bool = {
    l.rowId == p.rowId and
    p.inValid == 1 and
    p.outValid == 1 and
    l.x == p.x and
    l.y == p.y and
    l.s == mulMod(p.x, p.x) and
    l.lam == specLam(p.x, p.y) and
    p.x3 == specX3(p.x, p.y) and
    p.y3 == specY3(p.x, p.y)
  }

  pure def producerMatchesLDbl(p: ProducerRow, l0: LDblRow, l1: LDblRow): bool = {
    p.outValid == 1 implies (
      p.inValid == 1 and
      p.x3 == specX3(p.x, p.y) and
      p.y3 == specY3(p.x, p.y) and
      ((l0.isValid == 1 and rowMatchesProducer(l0, p)) or
       (l1.isValid == 1 and rowMatchesProducer(l1, p)))
    )
  }

  pure def checkInvRow(
    st: ChainState,
    recvS: bool,
    sendYAsX2: bool,
    wrongConst: bool,
    noIdO5: bool,
    validFree: bool
  ): bool = {
    chainAccepted(st, recvS, sendYAsX2, wrongConst, noIdO5, validFree) implies (
      (st.l0.isValid == 1 implies (rowMatchesProducer(st.l0, st.prod0) or rowMatchesProducer(st.l0, st.prod1))) and
      (st.l1.isValid == 1 implies (rowMatchesProducer(st.l1, st.prod0) or rowMatchesProducer(st.l1, st.prod1))) and
      producerMatchesLDbl(st.prod0, st.l0, st.l1) and
      producerMatchesLDbl(st.prod1, st.l0, st.l1)
    )
  }

  pure def checkNonVacuous(
    st: ChainState,
    recvS: bool,
    sendYAsX2: bool,
    wrongConst: bool,
    noIdO5: bool,
    validFree: bool
  ): bool = {
    not(
      chainAccepted(st, recvS, sendYAsX2, wrongConst, noIdO5, validFree) and
      st.l0.isValid == 1 and
      st.l1.isValid == 1
    )
  }

  pure val honestProducer0: ProducerRow = {
    { rowId: 1, x: 2, y: 5, x3: 12, y3: 9, inValid: 1, outValid: 1 }
  }

  pure val honestProducer1: ProducerRow = {
    { rowId: 2, x: 7, y: 3, x3: 11, y3: 3, inValid: 1, outValid: 1 }
  }

  pure def honestSq(p: ProducerRow): SqRow = {
    { rowId: p.rowId, x: p.x, s: mulMod(p.x, p.x), isValid: p.inValid }
  }

  pure def honestLDbl(p: ProducerRow, wrongConst: bool): LDblRow = {
    val sVal = mulMod(p.x, p.x)
    val coeff = if (wrongConst) 2 else 3
    val lVal = mulMod(mulMod(coeff, sVal), invMod(mulMod(2, p.y)))
    { rowId: p.rowId, s: sVal, x: p.x, y: p.y, lam: lVal, isValid: p.inValid }
  }

  pure def honestX3FromL(l: LDblRow, sendYAsX2: bool): X3Row = {
    val x2In = if (sendYAsX2) l.y else l.x
    val x3Val = subMod(subMod(mulMod(l.lam, l.lam), l.x), x2In)
    { rowId: l.rowId, lam: l.lam, x1: l.x, x2: x2In, x3: x3Val, isValid: 1 }
  }

  pure def honestY3FromLX(l: LDblRow, xRow: X3Row, yIn: int): Y3Row = {
    val y3Val = subMod(mulMod(l.lam, subMod(l.x, xRow.x3)), yIn)
    { rowId: l.rowId, lam: l.lam, x1: l.x, x3: xRow.x3, y1: yIn, y3: y3Val, isValid: 1 }
  }

  pure def buildChainFromL(
    p0Base: ProducerRow,
    p1Base: ProducerRow,
    sq0: SqRow,
    sq1: SqRow,
    l0: LDblRow,
    l1: LDblRow,
    sendYAsX2: bool,
    yIn0: int,
    yIn1: int,
    extraDelta: int
  ): ChainState = {
    val x0 = honestX3FromL(l0, sendYAsX2)
    val x1 = honestX3FromL(l1, sendYAsX2)
    val y0 = honestY3FromLX(l0, x0, yIn0)
    val y1 = honestY3FromLX(l1, x1, yIn1)
    val p0 = {
      rowId: p0Base.rowId,
      x: p0Base.x,
      y: p0Base.y,
      x3: x0.x3,
      y3: y0.y3,
      inValid: p0Base.inValid,
      outValid: p0Base.outValid
    }
    val p1 = {
      rowId: p1Base.rowId,
      x: p1Base.x,
      y: p1Base.y,
      x3: x1.x3,
      y3: y1.y3,
      inValid: p1Base.inValid,
      outValid: p1Base.outValid
    }
    {
      prod0: p0,
      prod1: p1,
      sq0: sq0,
      sq1: sq1,
      l0: l0,
      l1: l1,
      x3_0: x0,
      x3_1: x1,
      y3_0: y0,
      y3_1: y1,
      o0ExtraDelta: extraDelta
    }
  }

  pure def honestChainState(wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0 = honestLDbl(p0, wrongConst)
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateLDblS(badS: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val coeff = if (wrongConst) 2 else 3
    val badLam = mulMod(mulMod(coeff, badS), invMod(mulMod(2, p0.y)))
    val l0 = { rowId: p0.rowId, s: badS, x: p0.x, y: p0.y, lam: badLam, isValid: 1 }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateLDblX(badX: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0Base = honestLDbl(p0, wrongConst)
    val l0 = { rowId: l0Base.rowId, s: l0Base.s, x: badX, y: l0Base.y, lam: l0Base.lam, isValid: l0Base.isValid }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateLDblY(badY: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val sVal = mulMod(p0.x, p0.x)
    val coeff = if (wrongConst) 2 else 3
    val badLam = mulMod(mulMod(coeff, sVal), invMod(mulMod(2, badY)))
    val l0 = { rowId: p0.rowId, s: sVal, x: p0.x, y: badY, lam: badLam, isValid: 1 }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateLDblLam(badLam: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0Base = honestLDbl(p0, wrongConst)
    val l0 = { rowId: l0Base.rowId, s: l0Base.s, x: l0Base.x, y: l0Base.y, lam: badLam, isValid: l0Base.isValid }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateLDblId(badId: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0Base = honestLDbl(p0, wrongConst)
    val l0 = { rowId: badId, s: l0Base.s, x: l0Base.x, y: l0Base.y, lam: l0Base.lam, isValid: l0Base.isValid }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateSwapYOnO5(wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0 = honestLDbl(p0, wrongConst)
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l1.y, l0.y, 0)
  }

  pure def mutateValidFreeForge(wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0Orig = honestProducer0
    val p0 = {
      rowId: p0Orig.rowId,
      x: p0Orig.x,
      y: p0Orig.y,
      x3: p0Orig.x3,
      y3: p0Orig.y3,
      inValid: 0,
      outValid: 1
    }
    val p1 = honestProducer1
    val sq0Orig = honestSq(p0Orig)
    val sq0 = { rowId: sq0Orig.rowId, x: sq0Orig.x, s: sq0Orig.s, isValid: 0 }
    val sq1 = honestSq(p1)
    val l0Orig = honestLDbl(p0Orig, wrongConst)
    val l0 = { rowId: l0Orig.rowId, s: l0Orig.s, x: l0Orig.x, y: l0Orig.y, lam: 0, isValid: 0 }
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, 0)
  }

  pure def mutateDropOrDupMsg(delta: int, wrongConst: bool, sendYAsX2: bool): ChainState = {
    val p0 = honestProducer0
    val p1 = honestProducer1
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1)
    val l0 = honestLDbl(p0, wrongConst)
    val l1 = honestLDbl(p1, wrongConst)
    buildChainFromL(p0, p1, sq0, sq1, l0, l1, sendYAsX2, l0.y, l1.y, delta)
  }

  pure val duplicateIdSwapState: ChainState = {
    val p0 = honestProducer0
    val p1Orig = honestProducer1
    val p1SameId = {
      rowId: 1,
      x: p1Orig.x,
      y: p1Orig.y,
      x3: p1Orig.x3,
      y3: p1Orig.y3,
      inValid: p1Orig.inValid,
      outValid: p1Orig.outValid
    }
    val sq0 = honestSq(p0)
    val sq1 = honestSq(p1SameId)
    val lam0Cross = specLam(p0.x, p1SameId.y)
    val lam1Cross = specLam(p1SameId.x, p0.y)
    val l0 = { rowId: 1, s: sq0.s, x: p0.x, y: p1SameId.y, lam: lam0Cross, isValid: 1 }
    val l1 = { rowId: 1, s: sq1.s, x: p1SameId.x, y: p0.y, lam: lam1Cross, isValid: 1 }
    buildChainFromL(p0, p1SameId, sq0, sq1, l0, l1, false, l0.y, l1.y, 0)
  }

  pure def zeroPointForgedState(forgedLam: int): ChainState = {
    val p0Zero = { rowId: 1, x: 0, y: 0, x3: 0, y3: 0, inValid: 1, outValid: 1 }
    val p1 = honestProducer1
    val sq0 = { rowId: 1, x: 0, s: 0, isValid: 1 }
    val sq1 = honestSq(p1)
    val l0 = { rowId: 1, s: 0, x: 0, y: 0, lam: forgedLam, isValid: 1 }
    val l1 = honestLDbl(p1, false)
    buildChainFromL(p0Zero, p1, sq0, sq1, l0, l1, false, 0, l1.y, 0)
  }

  var st: ChainState

  action init = {
    st' = honestChainState(false, false)
  }

  action step = {
    st' = honestChainState(false, false)
  }

  val invRow: bool = checkInvRow(st, true, false, false, false, false)

  action initNoRecvS = {
    st' = mutateLDblS(1, false, false)
  }

  action initSendYAsX2 = {
    st' = honestChainState(false, true)
  }

  action initWrongConst = {
    st' = honestChainState(true, false)
  }

  action initNoId = {
    st' = mutateSwapYOnO5(false, false)
  }

  action initValidFree = {
    st' = mutateValidFreeForge(false, false)
  }

  action initDupId = {
    st' = duplicateIdSwapState
  }

  action initZeroPoint = {
    st' = zeroPointForgedState(2)
  }

  // 1. Completeness and Non-Vacuity on Valid Curve Points
  run completenessHonestChainTest =
    init.then(all {
      assert(onCurve(st.prod0.x, st.prod0.y)),
      assert(onCurve(st.prod0.x3, st.prod0.y3)),
      assert(onCurve(st.prod1.x, st.prod1.y)),
      assert(onCurve(st.prod1.x3, st.prod1.y3)),
      assert(chainAccepted(st, true, false, false, false, false)),
      assert(checkInvRow(st, true, false, false, false, false)),
      assert(not(checkNonVacuous(st, true, false, false, false, false))),
      st' = st
    })

  // 2. Sound Module Rejects Single-Cell Mutations and Message Perturbations
  run soundRejectsSingleCellAndBusPerturbationsTest =
    init.then(all {
      assert(not(chainAccepted(mutateLDblS(1, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateLDblX(3, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateLDblY(6, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateLDblLam(0, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateLDblId(3, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateSwapYOnO5(false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateValidFreeForge(false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateDropOrDupMsg(-1, false, false), true, false, false, false, false))),
      assert(not(chainAccepted(mutateDropOrDupMsg(1, false, false), true, false, false, false, false))),
      assert(checkInvRow(mutateLDblS(1, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateLDblX(3, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateLDblY(6, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateLDblLam(0, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateLDblId(3, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateSwapYOnO5(false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateValidFreeForge(false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateDropOrDupMsg(-1, false, false), true, false, false, false, false)),
      assert(checkInvRow(mutateDropOrDupMsg(1, false, false), true, false, false, false, false)),
      st' = st
    })

  // 3. Mutant `m_no_recv_s` Caught: Free `s` and Free `x` Violate `invRow`
  run mutantNoRecvSCaughtTest =
    initNoRecvS.then(all {
      assert(chainAccepted(st, false, false, false, false, false)),
      assert(not(checkInvRow(st, false, false, false, false, false))),
      assert(chainAccepted(mutateLDblX(3, false, false), false, false, false, false, false)),
      assert(not(checkInvRow(mutateLDblX(3, false, false), false, false, false, false, false))),
      st' = st
    })

  // 4. Mutant `m_send_y_as_x2` Caught: Sending `(id, y)` on bus o+3 Violates `invRow`
  run mutantSendYAsX2CaughtTest =
    initSendYAsX2.then(all {
      assert(chainAccepted(st, true, true, false, false, false)),
      assert(not(checkInvRow(st, true, true, false, false, false))),
      assert(st.prod0.x3 != specX3(st.prod0.x, st.prod0.y)),
      st' = st
    })

  // 5. Mutant `m_wrong_const` Caught: Constraint `lam * 2y = 2s` Violates `invRow`
  run mutantWrongConstCaughtTest =
    initWrongConst.then(all {
      assert(chainAccepted(st, true, false, true, false, false)),
      assert(not(checkInvRow(st, true, false, true, false, false))),
      assert(st.l0.lam != specLam(st.prod0.x, st.prod0.y)),
      st' = st
    })

  // 6. Mutant `m_no_id` Caught: Dropping `id` on bus o+5 Allows Cross-Row `y` Swap
  run mutantNoIdCaughtTest =
    initNoId.then(all {
      assert(chainAccepted(st, true, false, false, true, false)),
      assert(not(checkInvRow(st, true, false, false, true, false))),
      assert(st.prod0.y3 != specY3(st.prod0.x, st.prod0.y)),
      st' = st
    })

  // 7. Mutant `m_valid_free` Caught: Multiplicity 1 on Sends Allows Forged Output from `isValid = 0` Row
  run mutantValidFreeCaughtTest =
    initValidFree.then(all {
      assert(chainAccepted(st, true, false, false, false, true)),
      assert(not(checkInvRow(st, true, false, false, false, true))),
      assert(st.l0.isValid == 0),
      assert(st.prod0.x3 != specX3(st.prod0.x, st.prod0.y)),
      st' = st
    })

  // 8. Question (1) Witness: Duplicate `rowId` Across Two Active Rows Allows Input Swapping on Bus `o`
  run duplicateIdSwapAllowedWithoutUniqueIdsTest =
    initDupId.then(all {
      assert(st.prod0.rowId == st.prod1.rowId),
      assert(validProducerRow(st.prod0) and validProducerRow(st.prod1)),
      assert(validSqRow(st.sq0) and validSqRow(st.sq1)),
      assert(validLDblRow(st.l0, false) and validLDblRow(st.l1, false)),
      assert(validX3Row(st.x3_0) and validX3Row(st.x3_1)),
      assert(validY3Row(st.y3_0) and validY3Row(st.y3_1)),
      assert(isBusBalanced(st, true, false, false, false)),
      assert(st.l0.y != st.prod0.y),
      assert(st.prod0.x3 != specX3(st.prod0.x, st.prod0.y)),
      st' = st
    })

  // 9. Question (2) Witness: Exceptional Point (x, y) = (0, 0) Leaves `lam` Unconstrained in `L_dbl`
  run zeroPointLeavesLambdaUnconstrainedTest =
    initZeroPoint.then(all {
      assert(validSqRow(st.sq0)),
      assert(validLDblRow(st.l0, false)),
      assert(validX3Row(st.x3_0)),
      assert(validY3Row(st.y3_0)),
      assert(isBusBalanced(st, true, false, false, false)),
      assert(st.l0.lam == 2),
      assert(st.prod0.x3 == 4),
      assert(st.prod0.y3 == 5),
      st' = st
    })
}
