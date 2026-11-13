// -*- mode: Bluespec; -*-
//
// Soundness, completeness, and mutation spec for the X14 BLS12-381 G1
// chain Y3 STARK AIR row kind (`lane-src/x_exp_node.rs`, lines 237-498).
//
// Small-field arithmetic instantiation:
//   Prime field F_13 (P = 13)
//   Short Weierstrass curve E(F_13): y^2 = x^3 + 4 (mod 13)
//   (isomorphic miniature of BLS12-381 G1: y^2 = x^3 + 4 (mod p_BLS))
//
// Representative curve points on E(F_13):
//   P1 = (0, 2):  2^2 = 4 == 0^3 + 4 (mod 13)
//   P2 = (2, 5):  5^2 = 25 == 12 == 2^3 + 4 (mod 13)
//   P1 + P2:
//     lambda = (5 - 2) * (2 - 0)^(-1) = 3 * 7 = 21 == 8 (mod 13)
//     x3     = 8^2 - 0 - 2 = 62 == 10 (mod 13)
//     y3     = 8 * (0 - 10) - 2 = 8 * 3 - 2 = 22 == 9 (mod 13)
//     Check: 9^2 = 81 == 3 (mod 13); 10^3 + 4 = 1004 == 3 (mod 13).
//
//   P3 = (4, 4):  4^2 = 16 == 3 == 4^3 + 4 = 68 (mod 13)
//   P4 = (7, 3):  3^2 = 9 == 7^3 + 4 = 347 (mod 13)
//   P3 + P4:
//     lambda = (3 - 4) * (7 - 4)^(-1) = 12 * 9 = 108 == 4 (mod 13)
//     x3     = 4^2 - 4 - 7 = 5 (mod 13)
//     y3     = 4 * (4 - 5) - 4 = 4 * 12 - 4 = 44 == 5 (mod 13)
//     Check: 5^2 = 25 == 12 (mod 13); 5^3 + 4 = 129 == 12 (mod 13).
//
//   2 * P2 = 2 * (2, 5):
//     lambda = (3 * 2^2) * (2 * 5)^(-1) = 12 * 10^(-1) = 12 * 4 = 48 == 9 (mod 13)
//     x3     = 9^2 - 2 * 2 = 77 == 12 (mod 13)
//     y3     = 9 * (2 - 12) - 5 = 9 * 3 - 5 = 22 == 9 (mod 13)
//     Check: 9^2 = 81 == 3 (mod 13); 12^3 + 4 = (-1)^3 + 4 = 3 (mod 13).

module Y3Defs {
  pure val P: int = 13
  pure val CURVE_B: int = 4

  // All field elements in F_13
  pure val F13: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)

  // Modular reduction into [0, P - 1]
  pure def modP(a: int): int = {
    val r = a % P
    if (r < 0) r + P else r
  }

  pure def fAdd(a: int, b: int): int = modP(a + b)
  pure def fSub(a: int, b: int): int = modP(a - b)
  pure def fMul(a: int, b: int): int = modP(a * b)
  pure def fNeg(a: int): int = modP(-a)

  // Fermat inverse a^(P-2) mod 13 (returns 0 for a == 0 mod 13)
  pure def fInv(a: int): int = {
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

  pure def inField(a: int): bool = (0 <= a) and (a < P)
  pure def isBit(b: int): bool = (b == 0) or (b == 1)

  // Curve equation y^2 == x^3 + 4 (mod 13)
  pure def onCurve(x: int, y: int): bool = {
    fMul(y, y) == fAdd(fMul(fMul(x, x), x), CURVE_B)
  }

  // Honest mathematical specification of Y3:
  //   y3 = lambda * (x1 - x3) - y1 (mod P)
  pure def affineY3(lam: int, x1: int, x3: int, y1: int): int = {
    fSub(fMul(lam, fSub(x1, x3)), y1)
  }

  // Mutant M4 (`m_sign_flip`) wrong formula:
  //   y3 = lambda * (x3 - x1) - y1 (mod P)
  pure def flippedY3(lam: int, x1: int, x3: int, y1: int): int = {
    fSub(fMul(lam, fSub(x3, x1)), y1)
  }

  // Honest X3 specification:
  //   x3 = lambda^2 - x1 - x2 (mod P)
  pure def affineX3(lam: int, x1: int, x2: int): int = {
    fSub(fSub(fMul(lam, lam), x1), x2)
  }

  // Honest L_add slope specification (for x1 != x2 mod P)
  pure def affineLambdaAdd(x1: int, y1: int, x2: int, y2: int): int = {
    fMul(fSub(y2, y1), fInv(fSub(x2, x1)))
  }

  // Honest L_dbl slope specification (for y1 != 0 mod P)
  pure def affineLambdaDbl(x1: int, y1: int): int = {
    val num = fMul(3, fMul(x1, x1))
    val den = fMul(2, y1)
    fMul(num, fInv(den))
  }

  // Row record types for the 2-row G1 chain model:
  //   L   (slope chip: L_add when opMode == 0, L_dbl when opMode == 1)
  //   X3  (x3 chip)
  //   Y3  (y3 chip under review)
  //   Sink (downstream consumer receiving on bus o+7)
  type LRow = {
    id: int,
    is_valid: int,
    x1: int,
    y1: int,
    x2: int,
    y2: int,
    lam: int
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
    is_valid: int,
    out_x: int,
    out_y: int
  }

  type BusMsg2 = {
    m0: int,
    m1: int
  }

  type BusMsg3 = {
    m0: int,
    m1: int,
    m2: int
  }

  // Full 2-row trace state across L, X3, Y3, and Sink
  type TraceState = {
    opMode: int,     // 0 = g1_add (bus offset 1100), 1 = g1_double (bus offset 1110)
    l0: LRow,
    l1: LRow,
    x0: X3Row,
    x1_row: X3Row,
    y0: Y3Row,
    y1_row: Y3Row,
    s0: SinkRow,
    s1: SinkRow
  }

  // ---- Local AIR polynomial & range constraints for L ----
  // In L_add (`ChainAir::eval` + `FieldVariable::div`):
  //   is_valid in {0, 1}
  //   is_valid * ((x2 - x1) * lam - (y2 - y1)) == 0 (mod P)
  // In L_dbl:
  //   is_valid in {0, 1}
  //   is_valid * ((2 * y1) * lam - 3 * x1^2) == 0 (mod P)
  pure def lRowValid(opMode: int, r: LRow): bool = {
    val eqOk = if (opMode == 0) {
      fMul(fSub(r.x2, r.x1), r.lam) == fSub(r.y2, r.y1)
    } else {
      fMul(fMul(2, r.y1), r.lam) == fMul(3, fMul(r.x1, r.x1))
    }
    and {
      inField(r.id),
      isBit(r.is_valid),
      inField(r.x1),
      inField(r.y1),
      inField(r.x2),
      inField(r.y2),
      inField(r.lam),
      (r.is_valid == 0) or eqOk
    }
  }

  // ---- Local AIR polynomial & range constraints for X3 ----
  //   is_valid in {0, 1}
  //   is_valid * (x3 - (lam^2 - x1 - x2)) == 0 (mod P)
  pure def x3RowValid(r: X3Row): bool = {
    and {
      inField(r.id),
      isBit(r.is_valid),
      inField(r.lam),
      inField(r.x1),
      inField(r.x2),
      inField(r.x3),
      (r.is_valid == 0) or (r.x3 == affineX3(r.lam, r.x1, r.x2))
    }
  }

  // ---- Local AIR polynomial & range constraints for Y3 ----
  // Parameterized by mutant ID `mId`:
  //   mId == 0: sound Y3 (`y3 = lam * (x1 - x3) - y1`)
  //   mId == 1: m_no_recv_x3 (omits bus o+6 receive in Y3)
  //   mId == 2: m_out_wrong_x (sends (id, x1, y3) instead of (id, x3, y3) on bus o+7)
  //   mId == 3: m_no_recv_y1 (omits bus o+5 receive in Y3)
  //   mId == 4: m_sign_flip (constrains `y3 = lam * (x3 - x1) - y1`)
  //   mId == 5: m_valid_free (sends on bus o+7 with multiplicity 1 instead of is_valid)
  pure def y3RowValid(r: Y3Row, mId: int): bool = {
    val targetY3 = if (mId == 4) {
      flippedY3(r.lam, r.x1, r.x3, r.y1)
    } else {
      affineY3(r.lam, r.x1, r.x3, r.y1)
    }
    and {
      inField(r.id),
      isBit(r.is_valid),
      inField(r.lam),
      inField(r.x1),
      inField(r.x3),
      inField(r.y1),
      inField(r.y3),
      (r.is_valid == 0) or (r.y3 == targetY3)
    }
  }

  pure def sinkRowValid(r: SinkRow): bool = {
    and {
      inField(r.id),
      isBit(r.is_valid),
      inField(r.out_x),
      inField(r.out_y)
    }
  }

  // ---- PermutationCheckBus multiset equality on buses o+2 .. o+7 ----

  // Bus o+2 (in L_add): L sends (id, x2) with mult is_valid; X3 receives (id, x2) with mult is_valid.
  pure def bus2LSend0(st: TraceState): Set[(int, BusMsg2)] =
    if (st.opMode == 0 and st.l0.is_valid == 1) Set((0, { m0: st.l0.id, m1: st.l0.x2 })) else Set()
  pure def bus2LSend1(st: TraceState): Set[(int, BusMsg2)] =
    if (st.opMode == 0 and st.l1.is_valid == 1) Set((1, { m0: st.l1.id, m1: st.l1.x2 })) else Set()
  pure def bus2X3Recv0(st: TraceState): Set[(int, BusMsg2)] =
    if (st.opMode == 0 and st.x0.is_valid == 1) Set((0, { m0: st.x0.id, m1: st.x0.x2 })) else Set()
  pure def bus2X3Recv1(st: TraceState): Set[(int, BusMsg2)] =
    if (st.opMode == 0 and st.x1_row.is_valid == 1) Set((1, { m0: st.x1_row.id, m1: st.x1_row.x2 })) else Set()

  // Multiset equality for up to 2 messages of type BusMsg2:
  //Two 2-row multisets (with multiplicities in {0, 1}) are equal iff for every message occurring in either side,
  // the total multiplicity sent equals the total multiplicity received.
  pure def countMsg2(m: BusMsg2, c0: int, msg0: BusMsg2, c1: int, msg1: BusMsg2): int = {
    val a0 = if (c0 > 0 and msg0 == m) c0 else 0
    val a1 = if (c1 > 0 and msg1 == m) c1 else 0
    a0 + a1
  }

  pure def multisetEq2(
    sc0: int, sm0: BusMsg2, sc1: int, sm1: BusMsg2,
    rc0: int, rm0: BusMsg2, rc1: int, rm1: BusMsg2
  ): bool = {
    val candidates = Set(sm0, sm1, rm0, rm1)
    candidates.forall(m =>
      countMsg2(m, sc0, sm0, sc1, sm1) == countMsg2(m, rc0, rm0, rc1, rm1)
    )
  }

  pure def countMsg3(m: BusMsg3, c0: int, msg0: BusMsg3, c1: int, msg1: BusMsg3): int = {
    val a0 = if (c0 > 0 and msg0 == m) c0 else 0
    val a1 = if (c1 > 0 and msg1 == m) c1 else 0
    a0 + a1
  }

  pure def multisetEq3(
    sc0: int, sm0: BusMsg3, sc1: int, sm1: BusMsg3,
    rc0: int, rm0: BusMsg3, rc1: int, rm1: BusMsg3
  ): bool = {
    val candidates = Set(sm0, sm1, rm0, rm1)
    candidates.forall(m =>
      countMsg3(m, sc0, sm0, sc1, sm1) == countMsg3(m, rc0, rm0, rc1, rm1)
    )
  }

  // Bus o+2 (add: L sends (id, x2) -> X3 receives (id, x2); dbl: X3 constrains x2 == x1 locally)
  pure def bus2Balanced(st: TraceState): bool = {
    if (st.opMode == 0) {
      multisetEq2(
        st.l0.is_valid, { m0: st.l0.id, m1: st.l0.x2 },
        st.l1.is_valid, { m0: st.l1.id, m1: st.l1.x2 },
        st.x0.is_valid, { m0: st.x0.id, m1: st.x0.x2 },
        st.x1_row.is_valid, { m0: st.x1_row.id, m1: st.x1_row.x2 }
      )
    } else {
      and {
        (st.x0.is_valid == 0) or (st.x0.x2 == st.x0.x1),
        (st.x1_row.is_valid == 0) or (st.x1_row.x2 == st.x1_row.x1)
      }
    }
  }

  // Bus o+3: L sends (id, lam, x1) with mult is_valid -> X3 receives (id, lam, x1) with mult is_valid
  pure def bus3Balanced(st: TraceState): bool = {
    multisetEq3(
      st.l0.is_valid, { m0: st.l0.id, m1: st.l0.lam, m2: st.l0.x1 },
      st.l1.is_valid, { m0: st.l1.id, m1: st.l1.lam, m2: st.l1.x1 },
      st.x0.is_valid, { m0: st.x0.id, m1: st.x0.lam, m2: st.x0.x1 },
      st.x1_row.is_valid, { m0: st.x1_row.id, m1: st.x1_row.lam, m2: st.x1_row.x1 }
    )
  }

  // Bus o+4: L sends (id, lam, x1) with mult is_valid -> Y3 receives (id, lam, x1) with mult is_valid
  pure def bus4Balanced(st: TraceState): bool = {
    multisetEq3(
      st.l0.is_valid, { m0: st.l0.id, m1: st.l0.lam, m2: st.l0.x1 },
      st.l1.is_valid, { m0: st.l1.id, m1: st.l1.lam, m2: st.l1.x1 },
      st.y0.is_valid, { m0: st.y0.id, m1: st.y0.lam, m2: st.y0.x1 },
      st.y1_row.is_valid, { m0: st.y1_row.id, m1: st.y1_row.lam, m2: st.y1_row.x1 }
    )
  }

  // Bus o+5: L sends (id, y1) with mult is_valid -> Y3 receives (id, y1) with mult is_valid
  // Mutant 3 (`m_no_recv_y1`) drops Y3's receive constraint on bus o+5.
  pure def bus5Balanced(st: TraceState, mId: int): bool = {
    if (mId == 3) {
      true
    } else {
      multisetEq2(
        st.l0.is_valid, { m0: st.l0.id, m1: st.l0.y1 },
        st.l1.is_valid, { m0: st.l1.id, m1: st.l1.y1 },
        st.y0.is_valid, { m0: st.y0.id, m1: st.y0.y1 },
        st.y1_row.is_valid, { m0: st.y1_row.id, m1: st.y1_row.y1 }
      )
    }
  }

  // Bus o+6: X3 sends (id, x3) with mult is_valid -> Y3 receives (id, x3) with mult is_valid
  // Mutant 1 (`m_no_recv_x3`) drops Y3's receive constraint on bus o+6.
  pure def bus6Balanced(st: TraceState, mId: int): bool = {
    if (mId == 1) {
      true
    } else {
      multisetEq2(
        st.x0.is_valid, { m0: st.x0.id, m1: st.x0.x3 },
        st.x1_row.is_valid, { m0: st.x1_row.id, m1: st.x1_row.x3 },
        st.y0.is_valid, { m0: st.y0.id, m1: st.y0.x3 },
        st.y1_row.is_valid, { m0: st.y1_row.id, m1: st.y1_row.x3 }
      )
    }
  }

  // Bus o+7: Y3 sends (id, x3, y3) with mult is_valid -> Sink receives (id, out_x, out_y) with mult is_valid
  // Mutant 2 (`m_out_wrong_x`) sends (id, x1, y3) instead of (id, x3, y3) on bus o+7.
  // Mutant 5 (`m_valid_free`) sends on bus o+7 with unconditional multiplicity 1 instead of is_valid.
  pure def bus7Balanced(st: TraceState, mId: int): bool = {
    val outX0 = if (mId == 2) st.y0.x1 else st.y0.x3
    val outX1 = if (mId == 2) st.y1_row.x1 else st.y1_row.x3
    val mult0 = if (mId == 5) 1 else st.y0.is_valid
    val mult1 = if (mId == 5) 1 else st.y1_row.is_valid
    multisetEq3(
      mult0, { m0: st.y0.id, m1: outX0, m2: st.y0.y3 },
      mult1, { m0: st.y1_row.id, m1: outX1, m2: st.y1_row.y3 },
      st.s0.is_valid, { m0: st.s0.id, m1: st.s0.out_x, m2: st.s0.out_y },
      st.s1.is_valid, { m0: st.s1.id, m1: st.s1.out_x, m2: st.s1.out_y }
    )
  }

  // Complete STARK + LogUp verifier acceptance predicate for mutant `mId`
  pure def verifierAccepts(st: TraceState, mId: int): bool = {
    and {
      (st.opMode == 0) or (st.opMode == 1),
      lRowValid(st.opMode, st.l0),
      lRowValid(st.opMode, st.l1),
      x3RowValid(st.x0),
      x3RowValid(st.x1_row),
      y3RowValid(st.y0, mId),
      y3RowValid(st.y1_row, mId),
      sinkRowValid(st.s0),
      sinkRowValid(st.s1),
      bus2Balanced(st),
      bus3Balanced(st),
      bus4Balanced(st),
      bus5Balanced(st, mId),
      bus6Balanced(st, mId),
      bus7Balanced(st, mId)
    }
  }

  // ---- Preconditions for unique IDs and non-exceptional curve inputs ----
  pure def distinctActiveIds(st: TraceState): bool = {
    not(st.l0.is_valid == 1 and st.l1.is_valid == 1 and st.l0.id == st.l1.id)
  }

  // Matching helper: check if Y3 row `yr` refines some active upstream (LRow, X3Row) with the same `id`
  pure def y3RowRefinesUpstream(yr: Y3Row, lr: LRow, xr: X3Row): bool = {
    and {
      lr.is_valid == 1,
      xr.is_valid == 1,
      yr.id == lr.id,
      yr.id == xr.id,
      yr.lam == lr.lam,
      yr.x1 == lr.x1,
      yr.y1 == lr.y1,
      yr.x3 == xr.x3,
      yr.y3 == affineY3(lr.lam, lr.x1, xr.x3, lr.y1)
    }
  }

  // Matching helper: check if SinkRow `sr` refines some active upstream (LRow, X3Row) with the same `id`
  pure def sinkRowRefinesUpstream(sr: SinkRow, lr: LRow, xr: X3Row): bool = {
    and {
      lr.is_valid == 1,
      xr.is_valid == 1,
      sr.id == lr.id,
      sr.id == xr.id,
      sr.out_x == xr.x3,
      sr.out_y == affineY3(lr.lam, lr.x1, xr.x3, lr.y1)
    }
  }

  // Refinement property `invRow`:
  // Whenever the trace has distinct active IDs, every active Y3 row and every active Sink row
  // on bus o+7 must carry the exact mathematical affine coordinates:
  //   x3 = lambda^2 - x1 - x2 (mod P)
  //   y3 = lambda * (x1 - x3) - y1 (mod P)
  // bound to the unique upstream (L, X3) row of the same `id`, and the total number of active
  // Sink receptions on o+7 must equal the total number of active Y3 rows.
  pure def invRowProperty(st: TraceState): bool = {
    distinctActiveIds(st) implies and {
      (st.y0.is_valid == 1) implies (
        y3RowRefinesUpstream(st.y0, st.l0, st.x0) or
        y3RowRefinesUpstream(st.y0, st.l1, st.x1_row)
      ),
      (st.y1_row.is_valid == 1) implies (
        y3RowRefinesUpstream(st.y1_row, st.l0, st.x0) or
        y3RowRefinesUpstream(st.y1_row, st.l1, st.x1_row)
      ),
      (st.s0.is_valid == 1) implies (
        sinkRowRefinesUpstream(st.s0, st.l0, st.x0) or
        sinkRowRefinesUpstream(st.s0, st.l1, st.x1_row)
      ),
      (st.s1.is_valid == 1) implies (
        sinkRowRefinesUpstream(st.s1, st.l0, st.x0) or
        sinkRowRefinesUpstream(st.s1, st.l1, st.x1_row)
      ),
      (st.s0.is_valid + st.s1.is_valid) == (st.y0.is_valid + st.y1_row.is_valid)
    }
  }

  // ---- Honest trace builder ----
  // Constructs a canonical honest 2-row trace for `opMode` (0 = add, 1 = dbl) and `active1` in {0, 1}.
  // When `mId` in {0, 1, 3, 5} (with `active1 == 1` for `mId == 5`), the canonical honest witness is
  // accepted by both sound `y3` and mutant `mId` and satisfies `invRowProperty`.
  // When `mId == 2` (`m_out_wrong_x`) or `mId == 4` (`m_sign_flip`), the mutant alters the honest
  // emission/equation itself, so `honestTrace` adapts the emitted `out_x` (for `mId == 2`) or `y3`
  // (for `mId == 4`) to match the mutant's altered layout.
  pure def honestTrace(opMode: int, active1: int, mId: int): TraceState = {
    // Row 0:
    //   If opMode == 0 (g1_add): P1 = (0, 2), P2 = (2, 5) -> lam = 8, x3 = 10, y3 = 9
    //   If opMode == 1 (g1_dbl): P1 = (2, 5)              -> lam = 9, x3 = 12, y3 = 9
    val l0: LRow = if (opMode == 0) {
      { id: 1, is_valid: 1, x1: 0, y1: 2, x2: 2, y2: 5, lam: 8 }
    } else {
      { id: 1, is_valid: 1, x1: 2, y1: 5, x2: 2, y2: 5, lam: 9 }
    }
    val x0_val = affineX3(l0.lam, l0.x1, l0.x2)
    val x0: X3Row = { id: 1, is_valid: 1, lam: l0.lam, x1: l0.x1, x2: l0.x2, x3: x0_val }
    val y0_val = if (mId == 4) {
      flippedY3(l0.lam, l0.x1, x0_val, l0.y1)
    } else {
      affineY3(l0.lam, l0.x1, x0_val, l0.y1)
    }
    val y0: Y3Row = {
      id: 1,
      is_valid: 1,
      lam: l0.lam,
      x1: l0.x1,
      x3: x0_val,
      y1: l0.y1,
      y3: y0_val
    }
    val s0_x = if (mId == 2) y0.x1 else y0.x3
    val s0: SinkRow = { id: 1, is_valid: 1, out_x: s0_x, out_y: y0.y3 }

    // Row 1:
    //   P3 = (4, 4), P4 = (7, 3) -> lam = 4, x3 = 5, y3 = 5 (for add)
    //   P3 = (4, 4)              -> lam = 6, x3 = 2, y3 = 8 (for dbl: 3*16 * 8^-1 = 9*5 = 6, x3=36-8=2, y3=6*(4-2)-4=8)
    val l1: LRow = if (opMode == 0) {
      { id: 2, is_valid: active1, x1: 4, y1: 4, x2: 7, y2: 3, lam: 4 }
    } else {
      { id: 2, is_valid: active1, x1: 4, y1: 4, x2: 4, y2: 4, lam: 6 }
    }
    val x1_val = affineX3(l1.lam, l1.x1, l1.x2)
    val x1_row: X3Row = { id: 2, is_valid: active1, lam: l1.lam, x1: l1.x1, x2: l1.x2, x3: x1_val }
    val y1_val = if (mId == 4) {
      flippedY3(l1.lam, l1.x1, x1_val, l1.y1)
    } else {
      affineY3(l1.lam, l1.x1, x1_val, l1.y1)
    }
    val y1_row: Y3Row = {
      id: 2,
      is_valid: active1,
      lam: l1.lam,
      x1: l1.x1,
      x3: x1_val,
      y1: l1.y1,
      y3: y1_val
    }
    val s1_mult = if (mId == 5) 1 else active1
    val s1_x = if (mId == 2) y1_row.x1 else y1_row.x3
    val s1: SinkRow = { id: 2, is_valid: s1_mult, out_x: s1_x, out_y: y1_row.y3 }

    {
      opMode: opMode,
      l0: l0,
      l1: l1,
      x0: x0,
      x1_row: x1_row,
      y0: y0,
      y1_row: y1_row,
      s0: s0,
      s1: s1
    }
  }

  // ---- Attack trace constructors for the 5 mutants ----

  // M1 (`m_no_recv_x3`) attack:
  // Y3 row 0 substitutes `forgedX3` (e.g. 0 instead of 10), recomputes `y3 = affineY3(8, 0, forgedX3, 2)`,
  // and emits `(id=1, forgedX3, forgedY3)` on bus o+7.
  // Accepted by `m_no_recv_x3` (`mId == 1`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  pure def attackTraceNoRecvX3(forgedX3: int): TraceState = {
    val base = honestTrace(0, 1, 0)
    val forgedY3 = affineY3(base.y0.lam, base.y0.x1, forgedX3, base.y0.y1)
    val y0_tampered: Y3Row = { ...base.y0, x3: forgedX3, y3: forgedY3 }
    val s0_tampered: SinkRow = { ...base.s0, out_x: forgedX3, out_y: forgedY3 }
    { ...base, y0: y0_tampered, s0: s0_tampered }
  }

  // M2 (`m_out_wrong_x`) attack:
  // Y3 emits `(id, x1, y3)` instead of `(id, x3, y3)` on bus o+7; Sink receives `out_x = x1 = 0 != 10`.
  // Accepted by `m_out_wrong_x` (`mId == 2`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  pure def attackTraceOutWrongX: TraceState = {
    honestTrace(0, 1, 2)
  }

  // M3 (`m_no_recv_y1`) attack:
  // Y3 row 0 substitutes `forgedY1` (e.g. 0 instead of 2), recomputes `y3 = affineY3(8, 0, 10, forgedY1)`,
  // and emits `(id=1, 10, forgedY3)` on bus o+7.
  // Accepted by `m_no_recv_y1` (`mId == 3`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  pure def attackTraceNoRecvY1(forgedY1: int): TraceState = {
    val base = honestTrace(0, 1, 0)
    val forgedY3 = affineY3(base.y0.lam, base.y0.x1, base.y0.x3, forgedY1)
    val y0_tampered: Y3Row = { ...base.y0, y1: forgedY1, y3: forgedY3 }
    val s0_tampered: SinkRow = { ...base.s0, out_y: forgedY3 }
    { ...base, y0: y0_tampered, s0: s0_tampered }
  }

  // M4 (`m_sign_flip`) attack:
  // Y3 enforces `y3 = lam * (x3 - x1) - y1 = 8 * (10 - 0) - 2 = 0 != 9 (mod 13)`.
  // Accepted by `m_sign_flip` (`mId == 4`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  pure def attackTraceSignFlip: TraceState = {
    honestTrace(0, 1, 4)
  }

  // M5 (`m_valid_free`) attack:
  // Row 1 is inactive (`is_valid = 0`) with forged `y3 = forgedY3` (unconstrained when `is_valid = 0`),
  // yet `m_valid_free` sends row 1 on bus o+7 with multiplicity 1, consumed by an active Sink row (`s1.is_valid = 1`).
  // Accepted by `m_valid_free` (`mId == 5`), rejected by sound `y3` (`mId == 0`), violates `invRowProperty`.
  pure def attackTraceValidFree(forgedY3: int): TraceState = {
    val base = honestTrace(0, 0, 0)
    val y1_forged: Y3Row = { ...base.y1_row, is_valid: 0, y3: forgedY3 }
    val s1_forged: SinkRow = { id: 2, is_valid: 1, out_x: y1_forged.x3, out_y: forgedY3 }
    { ...base, y1_row: y1_forged, s1: s1_forged }
  }

  // ---- General single-cell tamper on Y3 row 0 ----
  // `cellKind` in 0..6 selects which column of Y3 row 0 (`y0`) is overwritten with `newElem`:
  //   0: id, 1: is_valid, 2: lam, 3: x1, 4: x3, 5: y1, 6: y3
  // If `recomputeY3` is true, `y0.y3` and `s0` are recomputed to satisfy Y3's local polynomial and bus o+7.
  pure def tamperY0Cell(cellKind: int, newElem: int, recomputeY3: bool, mId: int): TraceState = {
    val base = honestTrace(0, 1, 0)
    val rawY0: Y3Row =
      if (cellKind == 0) { ...base.y0, id: newElem }
      else if (cellKind == 1) { ...base.y0, is_valid: newElem % 2 }
      else if (cellKind == 2) { ...base.y0, lam: newElem }
      else if (cellKind == 3) { ...base.y0, x1: newElem }
      else if (cellKind == 4) { ...base.y0, x3: newElem }
      else if (cellKind == 5) { ...base.y0, y1: newElem }
      else { ...base.y0, y3: newElem }

    val updatedY3 = if (recomputeY3 and cellKind != 6) {
      if (mId == 4) flippedY3(rawY0.lam, rawY0.x1, rawY0.x3, rawY0.y1)
      else affineY3(rawY0.lam, rawY0.x1, rawY0.x3, rawY0.y1)
    } else {
      rawY0.y3
    }
    val y0_final: Y3Row = { ...rawY0, y3: updatedY3 }
    val s0_final: SinkRow = if (recomputeY3) {
      val sx = if (mId == 2) y0_final.x1 else y0_final.x3
      val sm = if (mId == 5) 1 else y0_final.is_valid
      { id: y0_final.id, is_valid: sm, out_x: sx, out_y: y0_final.y3 }
    } else {
      base.s0
    }
    { ...base, y0: y0_final, s0: s0_final }
  }

  // ---- Question (1) & Question (2) concrete attack traces ----

  // Question (1) Attack A: Two active rows share `id = 1` and swap `y1` on bus o+5.
  // Row 0 has P1=(0,2), P2=(2,5) (lam=8, x3=10, true y3=9).
  // Row 1 has P3=(4,4), P4=(7,3) (lam=4, x3=5, true y3=5).
  // Because both rows use `id = 1`, bus o+5 carries {(1, 2), (1, 4)}.
  // Y3 row 0 receives (1, 4) instead of (1, 2) -> computes y3 = 8*(0 - 10) - 4 = 7 != 9!
  // Y3 row 1 receives (1, 2) instead of (1, 4) -> computes y3 = 4*(4 - 5) - 2 = 7 != 5!
  pure def duplicateIdSwapY1Trace: TraceState = {
    val l0: LRow = { id: 1, is_valid: 1, x1: 0, y1: 2, x2: 2, y2: 5, lam: 8 }
    val l1: LRow = { id: 1, is_valid: 1, x1: 4, y1: 4, x2: 7, y2: 3, lam: 4 }
    val x0: X3Row = { id: 1, is_valid: 1, lam: 8, x1: 0, x2: 2, x3: 10 }
    val x1_row: X3Row = { id: 1, is_valid: 1, lam: 4, x1: 4, x2: 7, x3: 5 }
    val y0_forged = affineY3(8, 0, 10, 4) // 7 (forged, honest is 9)
    val y1_forged = affineY3(4, 4, 5, 2)  // 7 (forged, honest is 5)
    val y0: Y3Row = { id: 1, is_valid: 1, lam: 8, x1: 0, x3: 10, y1: 4, y3: y0_forged }
    val y1_row: Y3Row = { id: 1, is_valid: 1, lam: 4, x1: 4, x3: 5, y1: 2, y3: y1_forged }
    val s0: SinkRow = { id: 1, is_valid: 1, out_x: 10, out_y: y0_forged }
    val s1: SinkRow = { id: 1, is_valid: 1, out_x: 5, out_y: y1_forged }
    {
      opMode: 0,
      l0: l0,
      l1: l1,
      x0: x0,
      x1_row: x1_row,
      y0: y0,
      y1_row: y1_row,
      s0: s0,
      s1: s1
    }
  }

  // Question (1) Attack B: Two active rows share `id = 1` and swap `x3` on bus o+6.
  // Y3 row 0 receives (1, 5) instead of (1, 10) -> computes y3 = 8*(0 - 5) - 2 = 10 != 9!
  // Y3 row 1 receives (1, 10) instead of (1, 5) -> computes y3 = 4*(4 - 10) - 4 = 11 != 5!
  pure def duplicateIdSwapX3Trace: TraceState = {
    val l0: LRow = { id: 1, is_valid: 1, x1: 0, y1: 2, x2: 2, y2: 5, lam: 8 }
    val l1: LRow = { id: 1, is_valid: 1, x1: 4, y1: 4, x2: 7, y2: 3, lam: 4 }
    val x0: X3Row = { id: 1, is_valid: 1, lam: 8, x1: 0, x2: 2, x3: 10 }
    val x1_row: X3Row = { id: 1, is_valid: 1, lam: 4, x1: 4, x2: 7, x3: 5 }
    val y0_forged = affineY3(8, 0, 5, 2)   // 10 (forged, honest is 9)
    val y1_forged = affineY3(4, 4, 10, 4)  // 11 (forged, honest is 5)
    val y0: Y3Row = { id: 1, is_valid: 1, lam: 8, x1: 0, x3: 5, y1: 2, y3: y0_forged }
    val y1_row: Y3Row = { id: 1, is_valid: 1, lam: 4, x1: 4, x3: 10, y1: 4, y3: y1_forged }
    val s0: SinkRow = { id: 1, is_valid: 1, out_x: 5, out_y: y0_forged }
    val s1: SinkRow = { id: 1, is_valid: 1, out_x: 10, out_y: y1_forged }
    {
      opMode: 0,
      l0: l0,
      l1: l1,
      x0: x0,
      x1_row: x1_row,
      y0: y0,
      y1_row: y1_row,
      s0: s0,
      s1: s1
    }
  }

  // Question (2) Attack A: Exceptional `g1_add` with P1 == P2 == (2, 5).
  // In L_add, `(x2 - x1) * lam - (y2 - y1) == 0 * lam - 0 == 0` holds for ANY `chosenLam` in F_13!
  // While true 2 * (2, 5) has slope lam = 9 and output (12, 9), an adversarial prover can pick
  // any `chosenLam` in F_13 and produce a valid proof for a forged point `(x3, y3) != (12, 9)`.
  pure def exceptionalSamePointAddTrace(chosenLam: int): TraceState = {
    val base = honestTrace(0, 1, 0)
    val l0: LRow = { id: 1, is_valid: 1, x1: 2, y1: 5, x2: 2, y2: 5, lam: chosenLam }
    val x3_forged = affineX3(chosenLam, 2, 2)
    val y3_forged = affineY3(chosenLam, 2, x3_forged, 5)
    val x0: X3Row = { id: 1, is_valid: 1, lam: chosenLam, x1: 2, x2: 2, x3: x3_forged }
    val y0: Y3Row = { id: 1, is_valid: 1, lam: chosenLam, x1: 2, x3: x3_forged, y1: 5, y3: y3_forged }
    val s0: SinkRow = { id: 1, is_valid: 1, out_x: x3_forged, out_y: y3_forged }
    { ...base, l0: l0, x0: x0, y0: y0, s0: s0 }
  }

  // Question (2) Attack B: Off-curve zero point P1 = (0, 0) fed to `g1_double`.
  // In L_dbl, `(2 * y1) * lam - 3 * x1^2 == 0 * lam - 0 == 0` holds for ANY `chosenLam` in F_13!
  pure def exceptionalDoubleZeroTrace(chosenLam: int): TraceState = {
    val base = honestTrace(1, 1, 0)
    val l0: LRow = { id: 1, is_valid: 1, x1: 0, y1: 0, x2: 0, y2: 0, lam: chosenLam }
    val x3_forged = affineX3(chosenLam, 0, 0)
    val y3_forged = affineY3(chosenLam, 0, x3_forged, 0)
    val x0: X3Row = { id: 1, is_valid: 1, lam: chosenLam, x1: 0, x2: 0, x3: x3_forged }
    val y0: Y3Row = { id: 1, is_valid: 1, lam: chosenLam, x1: 0, x3: x3_forged, y1: 0, y3: y3_forged }
    val s0: SinkRow = { id: 1, is_valid: 1, out_x: x3_forged, out_y: y3_forged }
    { ...base, l0: l0, x0: x0, y0: y0, s0: s0 }
  }

  // Exhaustive pure check over all 7 cell kinds x 13 field elements x 2 recompute modes in sound Y3:
  pure def allSingleCellTampersSound: bool = {
    val cellKinds = Set(0, 1, 2, 3, 4, 5, 6)
    val recomputeModes = Set(false, true)
    cellKinds.forall(ck =>
      F13.forall(elem =>
        recomputeModes.forall(rc => {
          val st = tamperY0Cell(ck, elem, rc, 0)
          verifierAccepts(st, 0) implies invRowProperty(st)
        })
      )
    )
  }
}

// Parameterized state machine module for `quint run`
module Y3Machine {
  import Y3Defs.*

  const M_ID: int

  var tr: TraceState
  var accepted: bool

  action setTrace(nextTr: TraceState): bool = all {
    tr' = nextTr,
    accepted' = verifierAccepts(nextTr, M_ID)
  }

  // `init` starts from the canonical honest 2-active-row witness:
  // - For M_ID in {0, 1, 3, 5}, `honestTrace(0, 1, 0)` is accepted and satisfies `invRow`
  //   (proving non-vacuity of the honest witness under M1, M3, M5).
  // - For M_ID in {2, 4}, the mutant alters the honest emission/equation itself, so `honestTrace(0, 1, M_ID)`
  //   is accepted by the mutant and immediately violates `invRow`.
  action init: bool = {
    val initModeId = if (M_ID == 2 or M_ID == 4) M_ID else 0
    setTrace(honestTrace(0, 1, initModeId))
  }

  action step: bool = {
    if (M_ID == 1) {
      // M1 (`m_no_recv_x3`): tamper x3 in Y3 row 0 to any forged value != 10
      nondet forgedX3 = Set(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12).oneOf()
      setTrace(attackTraceNoRecvX3(forgedX3))
    } else if (M_ID == 2) {
      // M2 (`m_out_wrong_x`): emit x1 instead of x3 on bus o+7
      setTrace(attackTraceOutWrongX)
    } else if (M_ID == 3) {
      // M3 (`m_no_recv_y1`): tamper y1 in Y3 row 0 to any forged value != 2
      nondet forgedY1 = Set(0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).oneOf()
      setTrace(attackTraceNoRecvY1(forgedY1))
    } else if (M_ID == 4) {
      // M4 (`m_sign_flip`): compute y3 = lam * (x3 - x1) - y1
      setTrace(attackTraceSignFlip)
    } else if (M_ID == 5) {
      // M5 (`m_valid_free`): inactive row 1 (is_valid = 0) sends forged output with mult 1 on o+7
      nondet forgedY3 = Set(0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12).oneOf()
      setTrace(attackTraceValidFree(forgedY3))
    } else {
      // Sound module (`M_ID == 0`): explore honest witnesses (add/dbl, active/padding),
      // all 5 mutant attack witnesses, and arbitrary single-cell tampers across all 7 columns.
      any {
        {
          nondet opMode = Set(0, 1).oneOf()
          nondet active1 = Set(0, 1).oneOf()
          setTrace(honestTrace(opMode, active1, 0))
        },
        {
          nondet forgedX3 = F13.oneOf()
          setTrace(attackTraceNoRecvX3(forgedX3))
        },
        setTrace(attackTraceOutWrongX),
        {
          nondet forgedY1 = F13.oneOf()
          setTrace(attackTraceNoRecvY1(forgedY1))
        },
        setTrace(attackTraceSignFlip),
        {
          nondet forgedY3 = F13.oneOf()
          setTrace(attackTraceValidFree(forgedY3))
        },
        {
          nondet ck = Set(0, 1, 2, 3, 4, 5, 6).oneOf()
          nondet elem = F13.oneOf()
          nondet rc = Set(false, true).oneOf()
          setTrace(tamperY0Cell(ck, elem, rc, 0))
        }
      }
    }
  }

  // ---- Main refinement invariant `invRow` ----
  // Every trace accepted by the verifier (under distinct active IDs) refines the affine Y3 equation
  // and binds (id, x3, y3) to the unique upstream (L, X3) row of the same id.
  // - Holds in `y3` / `Y3` / `sound` (`[ok]`, exit 0)
  // - Violated in all 5 mutants (`[violation]`, exit 1)
  val invRow: bool =
    accepted implies invRowProperty(tr)

  // ---- Non-vacuity negated reach invariants (MUST BE VIOLATED in `y3` / `sound`, exit 1) ----

  // 1. Primary non-vacuity invariant: negated reach of an accepted active trace that refines `invRow`
  val invNonVacuous: bool =
    not(accepted and tr.y0.is_valid == 1 and invRowProperty(tr))

  // 2. Negated reach of an accepted 2-active-row `g1_add` trace
  val invNoAcceptedAdd: bool =
    not(accepted and tr.opMode == 0 and tr.y0.is_valid == 1 and tr.y1_row.is_valid == 1)

  // 3. Negated reach of an accepted `g1_double` trace
  val invNoAcceptedDbl: bool =
    not(accepted and tr.opMode == 1 and tr.y0.is_valid == 1)

  // 4. Negated reach of an accepted trace with an inactive padding row (`y1_row.is_valid == 0`)
  val invNoAcceptedPadding: bool =
    not(accepted and tr.y0.is_valid == 1 and tr.y1_row.is_valid == 0)
}

// Sound modules (`y3` is the default main module; `Y3` and `sound` are aliases)
module y3 {
  import Y3Machine(M_ID = 0).*
}

module Y3 {
  import Y3Machine(M_ID = 0).*
}

module sound {
  import Y3Machine(M_ID = 0).*
}

// Mutant 1: drop Y3's bus.receive on bus o+6 (`(id, x3)` from X3)
module m_no_recv_x3 {
  import Y3Machine(M_ID = 1).*
}

// Mutant 2: send `(id, x1, y3)` instead of `(id, x3, y3)` on bus o+7
module m_out_wrong_x {
  import Y3Machine(M_ID = 2).*
}

// Mutant 3: drop Y3's bus.receive on bus o+5 (`(id, y1)` from L)
module m_no_recv_y1 {
  import Y3Machine(M_ID = 3).*
}

// Mutant 4: flip sign of `(x1 - x3)` in Y3 (`y3 = lambda * (x3 - x1) - y1`)
module m_sign_flip {
  import Y3Machine(M_ID = 4).*
}

// Mutant 5: send on bus o+7 with multiplicity `1` instead of `is_valid`
module m_valid_free {
  import Y3Machine(M_ID = 5).*
}
