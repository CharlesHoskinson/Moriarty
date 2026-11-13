// -*- mode: Bluespec; -*-
// Quint formal specification of the X14 BLS12-381 G1 addition-chain condition row `INV`
// and the full 5-AIR addition chain (`PointsIn`, `L_add`, `X3`, `Y3`, `INV`) plus `sink`
// over a small prime field F_p (p = 13).
//
// Sound module:
//   - `x14_inv`: full sound specification with `invRow` (refinement invariant) and
//     `nonVacuous` (negated reachability invariant for non-vacuity).
//
// Required mutant modules (`m_<name>`):
//   - `m_no_inv_row`:     `L_add` does not send on bus o+10 (`INV` row absent);
//                         forged lambda on P1 = P2 is accepted, violating `invRow`.
//   - `m_no_recv`:        `INV` does not receive on bus o+10 (`w` free of the chain).
//   - `m_wrong_den`:      `INV` constraint is `w * (x2 + x1) = 1` instead of `w * (x2 - x1) = 1`.
//   - `m_no_id`:          drop `id` from the bus o+10 message.
//   - `m_inv_valid_free`: `INV` receive multiplicity is `1` in place of `is_valid`
//                         while `L_add` sends with `is_valid`.

module x14_inv {
  // =========================================================================
  // 1. Small Prime Field F_13 Arithmetic
  // =========================================================================
  pure val P: int = 13

  type Fp = int

  pure val FIELD: Set[Fp] = 0.to(12)
  pure val NONZERO_FIELD: Set[Fp] = 1.to(12)
  pure val BIT: Set[int] = Set(0, 1)

  pure def inField(x: Fp): bool = (0 <= x) and (x < P)

  pure def isBit(b: int): bool = (b == 0) or (b == 1)

  pure def fadd(a: Fp, b: Fp): Fp = (a + b) % P

  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P

  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P

  pure def finv(a: Fp): Fp =
    if (a == 1) 1
    else if (a == 2) 7
    else if (a == 3) 9
    else if (a == 4) 10
    else if (a == 5) 8
    else if (a == 6) 11
    else if (a == 7) 2
    else if (a == 8) 5
    else if (a == 9) 3
    else if (a == 10) 4
    else if (a == 11) 6
    else if (a == 12) 12
    else 0

  // =========================================================================
  // 2. Addition-Chain Buses (base o = 1100)
  // =========================================================================
  pure val O_BASE: int = 1100
  pure val BUS_O0: int = 1100
  pure val BUS_O1: int = 1101
  pure val BUS_O2: int = 1102
  pure val BUS_O3: int = 1103
  pure val BUS_O4: int = 1104
  pure val BUS_O5: int = 1105
  pure val BUS_O6: int = 1106
  pure val BUS_O7: int = 1107
  pure val BUS_O10: int = 1110

  // =========================================================================
  // 3. Row Types for Each AIR in the Chain
  // =========================================================================
  type PointsInRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, is_valid: int }
  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type InvRow = { row_id: int, x1: Fp, x2: Fp, w: Fp, is_valid: int }
  type BusEntry = { bus: int, tid: int, v0: Fp, v1: Fp, v2: Fp, mult: int, dir: int }

  pure val NO_PERT: BusEntry =
    { bus: 0, tid: 0, v0: 0, v1: 0, v2: 0, mult: 0, dir: 0 }

  // =========================================================================
  // 4. Two-Row Chain Trace State
  // =========================================================================
  type ChainState = {
    pin0: PointsInRow, l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow, inv0: InvRow,
    pin1: PointsInRow, l1: LAddRow, x1_row: X3Row, y1_row: Y3Row, sink1: SinkRow, inv1: InvRow,
    pert: BusEntry
  }

  // =========================================================================
  // 5. Local AIR Constraints (`FieldExpr` & `id` Transition Constraints)
  // =========================================================================
  pure def pointsInRowOk(r: PointsInRow): bool =
    inField(r.x1) and inField(r.y1) and inField(r.x2) and inField(r.y2) and isBit(r.is_valid)

  pure def lAddRowOk(r: LAddRow): bool =
    inField(r.x1) and inField(r.y1) and inField(r.x2) and inField(r.y2) and inField(r.lam)
      and isBit(r.is_valid)
      and (if (r.is_valid == 1) fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1) else true)

  pure def x3RowOk(r: X3Row): bool =
    inField(r.lam) and inField(r.x1) and inField(r.x2) and inField(r.x3) and isBit(r.is_valid)
      and (if (r.is_valid == 1) r.x3 == fsub(fsub(fmul(r.lam, r.lam), r.x1), r.x2) else true)

  pure def y3RowOk(r: Y3Row): bool =
    inField(r.lam) and inField(r.x1) and inField(r.y1) and inField(r.x3) and inField(r.y3)
      and isBit(r.is_valid)
      and (if (r.is_valid == 1) r.y3 == fsub(fmul(r.lam, fsub(r.x1, r.x3)), r.y1) else true)

  pure def sinkRowOk(r: SinkRow): bool =
    inField(r.x3) and inField(r.y3) and isBit(r.is_valid)

  pure def invRowOk(r: InvRow): bool =
    inField(r.x1) and inField(r.x2) and inField(r.w) and isBit(r.is_valid)
      and (if (r.is_valid == 1) fmul(r.w, fsub(r.x2, r.x1)) == 1 else true)

  pure def invRowOkWrongDen(r: InvRow): bool =
    inField(r.x1) and inField(r.x2) and inField(r.w) and isBit(r.is_valid)
      and (if (r.is_valid == 1) fmul(r.w, fadd(r.x2, r.x1)) == 1 else true)

  pure def idColOk(id0: int, id1: int): bool =
    (id0 == 0) and (id1 == id0 + 1)

  pure def transitionIdsOk(st: ChainState): bool =
    idColOk(st.pin0.row_id, st.pin1.row_id)
      and idColOk(st.l0.row_id, st.l1.row_id)
      and idColOk(st.x0_row.row_id, st.x1_row.row_id)
      and idColOk(st.y0_row.row_id, st.y1_row.row_id)
      and idColOk(st.sink0.row_id, st.sink1.row_id)
      and idColOk(st.inv0.row_id, st.inv1.row_id)

  pure def allLocalAirOk(st: ChainState): bool =
    pointsInRowOk(st.pin0) and pointsInRowOk(st.pin1)
      and lAddRowOk(st.l0) and lAddRowOk(st.l1)
      and x3RowOk(st.x0_row) and x3RowOk(st.x1_row)
      and y3RowOk(st.y0_row) and y3RowOk(st.y1_row)
      and sinkRowOk(st.sink0) and sinkRowOk(st.sink1)
      and invRowOk(st.inv0) and invRowOk(st.inv1)
      and transitionIdsOk(st)

  pure def allLocalAirOkWrongDen(st: ChainState): bool =
    pointsInRowOk(st.pin0) and pointsInRowOk(st.pin1)
      and lAddRowOk(st.l0) and lAddRowOk(st.l1)
      and x3RowOk(st.x0_row) and x3RowOk(st.x1_row)
      and y3RowOk(st.y0_row) and y3RowOk(st.y1_row)
      and sinkRowOk(st.sink0) and sinkRowOk(st.sink1)
      and invRowOkWrongDen(st.inv0) and invRowOkWrongDen(st.inv1)
      and transitionIdsOk(st)

  // =========================================================================
  // 6. Bus Configuration Variants (Sound vs Mutants)
  // =========================================================================
  type BusCfg = {
    l_sends_o10: bool,
    inv_recvs_o10: bool,
    wrong_den: bool,
    use_id_on_o10: bool,
    inv_mult_always_one: bool
  }

  pure val CFG_SOUND: BusCfg = {
    l_sends_o10: true, inv_recvs_o10: true, wrong_den: false,
    use_id_on_o10: true, inv_mult_always_one: false
  }
  pure val CFG_NO_INV_ROW: BusCfg = { ...CFG_SOUND, l_sends_o10: false, inv_recvs_o10: false }
  pure val CFG_NO_RECV: BusCfg = { ...CFG_SOUND, l_sends_o10: false, inv_recvs_o10: false }
  pure val CFG_WRONG_DEN: BusCfg = { ...CFG_SOUND, wrong_den: true }
  pure val CFG_NO_ID: BusCfg = { ...CFG_SOUND, use_id_on_o10: false }
  pure val CFG_INV_VALID_FREE: BusCfg = { ...CFG_SOUND, inv_mult_always_one: true }

  pure def chainLocalOk(st: ChainState, cfg: BusCfg): bool =
    if (cfg.wrong_den) allLocalAirOkWrongDen(st)
    else if (not(cfg.l_sends_o10) and not(cfg.inv_recvs_o10))
      pointsInRowOk(st.pin0) and pointsInRowOk(st.pin1)
        and lAddRowOk(st.l0) and lAddRowOk(st.l1)
        and x3RowOk(st.x0_row) and x3RowOk(st.x1_row)
        and y3RowOk(st.y0_row) and y3RowOk(st.y1_row)
        and sinkRowOk(st.sink0) and sinkRowOk(st.sink1)
        and transitionIdsOk(st)
    else allLocalAirOk(st)

  // =========================================================================
  // 7. Multiset Bus Balance Across the Chain AIRs
  // =========================================================================
  pure def mkSend(bus: int, tid: int, v0: Fp, v1: Fp, v2: Fp, mult: int): BusEntry =
    { bus: bus, tid: tid, v0: v0, v1: v1, v2: v2, mult: mult, dir: 1 }

  pure def mkRecv(bus: int, tid: int, v0: Fp, v1: Fp, v2: Fp, mult: int): BusEntry =
    { bus: bus, tid: tid, v0: v0, v1: v1, v2: v2, mult: mult, dir: -1 }

  pure def o10Tid(rid: int, cfg: BusCfg): int =
    if (cfg.use_id_on_o10) rid else 0

  pure def lO10Mult(v: int, cfg: BusCfg): int =
    if (cfg.l_sends_o10) v else 0

  pure def invO10Mult(v: int, cfg: BusCfg): int =
    if (not(cfg.inv_recvs_o10)) 0
    else if (cfg.inv_mult_always_one) 1
    else v

  pure def allBusEntries(st: ChainState, cfg: BusCfg): List[BusEntry] = [
    mkSend(BUS_O0, st.pin0.row_id, st.pin0.x1, st.pin0.y1, 0, st.pin0.is_valid),
    mkSend(BUS_O1, st.pin0.row_id, st.pin0.x2, st.pin0.y2, 0, st.pin0.is_valid),
    mkRecv(BUS_O0, st.l0.row_id, st.l0.x1, st.l0.y1, 0, st.l0.is_valid),
    mkRecv(BUS_O1, st.l0.row_id, st.l0.x2, st.l0.y2, 0, st.l0.is_valid),
    mkSend(BUS_O2, st.l0.row_id, st.l0.lam, st.l0.x1, 0, st.l0.is_valid),
    mkSend(BUS_O3, st.l0.row_id, st.l0.x2, 0, 0, st.l0.is_valid),
    mkSend(BUS_O4, st.l0.row_id, st.l0.lam, st.l0.x1, 0, st.l0.is_valid),
    mkSend(BUS_O5, st.l0.row_id, st.l0.y1, 0, 0, st.l0.is_valid),
    mkRecv(BUS_O2, st.x0_row.row_id, st.x0_row.lam, st.x0_row.x1, 0, st.x0_row.is_valid),
    mkRecv(BUS_O3, st.x0_row.row_id, st.x0_row.x2, 0, 0, st.x0_row.is_valid),
    mkSend(BUS_O6, st.x0_row.row_id, st.x0_row.x3, 0, 0, st.x0_row.is_valid),
    mkRecv(BUS_O4, st.y0_row.row_id, st.y0_row.lam, st.y0_row.x1, 0, st.y0_row.is_valid),
    mkRecv(BUS_O5, st.y0_row.row_id, st.y0_row.y1, 0, 0, st.y0_row.is_valid),
    mkRecv(BUS_O6, st.y0_row.row_id, st.y0_row.x3, 0, 0, st.y0_row.is_valid),
    mkSend(BUS_O7, st.y0_row.row_id, st.y0_row.x3, st.y0_row.y3, 0, st.y0_row.is_valid),
    mkRecv(BUS_O7, st.sink0.row_id, st.sink0.x3, st.sink0.y3, 0, st.sink0.is_valid),
    mkSend(BUS_O10, o10Tid(st.l0.row_id, cfg), st.l0.x1, st.l0.x2, 0, lO10Mult(st.l0.is_valid, cfg)),
    mkRecv(BUS_O10, o10Tid(st.inv0.row_id, cfg), st.inv0.x1, st.inv0.x2, 0, invO10Mult(st.inv0.is_valid, cfg)),
    mkSend(BUS_O0, st.pin1.row_id, st.pin1.x1, st.pin1.y1, 0, st.pin1.is_valid),
    mkSend(BUS_O1, st.pin1.row_id, st.pin1.x2, st.pin1.y2, 0, st.pin1.is_valid),
    mkRecv(BUS_O0, st.l1.row_id, st.l1.x1, st.l1.y1, 0, st.l1.is_valid),
    mkRecv(BUS_O1, st.l1.row_id, st.l1.x2, st.l1.y2, 0, st.l1.is_valid),
    mkSend(BUS_O2, st.l1.row_id, st.l1.lam, st.l1.x1, 0, st.l1.is_valid),
    mkSend(BUS_O3, st.l1.row_id, st.l1.x2, 0, 0, st.l1.is_valid),
    mkSend(BUS_O4, st.l1.row_id, st.l1.lam, st.l1.x1, 0, st.l1.is_valid),
    mkSend(BUS_O5, st.l1.row_id, st.l1.y1, 0, 0, st.l1.is_valid),
    mkRecv(BUS_O2, st.x1_row.row_id, st.x1_row.lam, st.x1_row.x1, 0, st.x1_row.is_valid),
    mkRecv(BUS_O3, st.x1_row.row_id, st.x1_row.x2, 0, 0, st.x1_row.is_valid),
    mkSend(BUS_O6, st.x1_row.row_id, st.x1_row.x3, 0, 0, st.x1_row.is_valid),
    mkRecv(BUS_O4, st.y1_row.row_id, st.y1_row.lam, st.y1_row.x1, 0, st.y1_row.is_valid),
    mkRecv(BUS_O5, st.y1_row.row_id, st.y1_row.y1, 0, 0, st.y1_row.is_valid),
    mkRecv(BUS_O6, st.y1_row.row_id, st.y1_row.x3, 0, 0, st.y1_row.is_valid),
    mkSend(BUS_O7, st.y1_row.row_id, st.y1_row.x3, st.y1_row.y3, 0, st.y1_row.is_valid),
    mkRecv(BUS_O7, st.sink1.row_id, st.sink1.x3, st.sink1.y3, 0, st.sink1.is_valid),
    mkSend(BUS_O10, o10Tid(st.l1.row_id, cfg), st.l1.x1, st.l1.x2, 0, lO10Mult(st.l1.is_valid, cfg)),
    mkRecv(BUS_O10, o10Tid(st.inv1.row_id, cfg), st.inv1.x1, st.inv1.x2, 0, invO10Mult(st.inv1.is_valid, cfg)),
    st.pert
  ]


  pure def netCount(entries: List[BusEntry], target: BusEntry): int =
    entries.foldl(0, (acc, cur) =>
      if (cur.bus == target.bus and cur.tid == target.tid and cur.v0 == target.v0
          and cur.v1 == target.v1 and cur.v2 == target.v2)
        acc + cur.mult * cur.dir
      else
        acc
    )

  pure def busBalanced(st: ChainState, cfg: BusCfg): bool = {
    val entries = allBusEntries(st, cfg)
    entries.foldl(true, (ok, cur) => ok and (netCount(entries, cur) == 0))
  }

  pure def chainAcceptedWith(st: ChainState, cfg: BusCfg): bool =
    chainLocalOk(st, cfg) and busBalanced(st, cfg)

  // =========================================================================
  // 8. Refinement invariant invRow
  // =========================================================================

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0)
        and (l.lam == true_lam)
        and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3)
        and (sk.x3 == true_x3)
        and (sk.y3 == true_y3)
    } else {
      true
    }

  pure def invRowHolds(st: ChainState, cfg: BusCfg): bool =
    if (chainAcceptedWith(st, cfg))
      rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0)
        and rowRefinementOk(st.l1, st.x1_row, st.y1_row, st.sink1)
    else
      true

  // =========================================================================
  // 9. Honest & adversarial witness constructors
  // =========================================================================

  type RowBundle = {
    pin: PointsInRow,
    l: LAddRow,
    x_row: X3Row,
    y_row: Y3Row,
    sk: SinkRow,
    inv: InvRow
  }

  pure def mkHonestDownstream(rid: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, v: int, inv_w: Fp): RowBundle = {
    val x3 = fsub(fsub(fmul(lam, lam), x1), x2)
    val y3 = fsub(fmul(lam, fsub(x1, x3)), y1)
    {
      pin: { row_id: rid, x1: x1, y1: y1, x2: x2, y2: y2, is_valid: v },
      l: { row_id: rid, x1: x1, y1: y1, x2: x2, y2: y2, lam: lam, is_valid: v },
      x_row: { row_id: rid, lam: lam, x1: x1, x2: x2, x3: x3, is_valid: v },
      y_row: { row_id: rid, lam: lam, x1: x1, y1: y1, x3: x3, y3: y3, is_valid: v },
      sk: { row_id: rid, x3: x3, y3: y3, is_valid: v },
      inv: { row_id: rid, x1: x1, x2: x2, w: inv_w, is_valid: v }
    }
  }

  pure def mkHonestRow(rid: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, v: int): RowBundle = {
    val dx = fsub(x2, x1)
    val dy = fsub(y2, y1)
    val w = if (dx != 0) finv(dx) else 1
    val lam = if (dx != 0) fmul(dy, w) else 0
    mkHonestDownstream(rid, x1, y1, x2, y2, lam, v, w)
  }

  pure def mkHonestRowWrongDen(rid: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, v: int): RowBundle = {
    val dx = fsub(x2, x1)
    val dy = fsub(y2, y1)
    val s = fadd(x2, x1)
    val w = if (s != 0) finv(s) else 1
    val lam = if (dx != 0) fmul(dy, finv(dx)) else 0
    mkHonestDownstream(rid, x1, y1, x2, y2, lam, v, w)
  }

  pure def packRows(r0: RowBundle, r1: RowBundle): ChainState = {
    pin0: r0.pin, l0: r0.l, x0_row: r0.x_row, y0_row: r0.y_row, sink0: r0.sk, inv0: r0.inv,
    pin1: r1.pin, l1: r1.l, x1_row: r1.x_row, y1_row: r1.y_row, sink1: r1.sk, inv1: r1.inv,
    pert: NO_PERT
  }

  pure def mkForgedSamePtState(x: Fp, y: Fp, forged_lam: Fp, forged_w: Fp): ChainState = {
    val r0 = mkHonestDownstream(0, x, y, x, y, forged_lam, 1, forged_w)
    val r1 = mkHonestRow(1, 2, 7, 5, 11, 1)
    packRows(r0, r1)
  }

  pure val HONEST_DEFAULT_STATE: ChainState =
    packRows(mkHonestRow(0, 1, 5, 4, 9, 1), mkHonestRow(1, 2, 7, 5, 11, 1))

  pure val HONEST_WRONG_DEN_STATE: ChainState =
    packRows(mkHonestRowWrongDen(0, 1, 5, 4, 9, 1), mkHonestRowWrongDen(1, 2, 7, 5, 11, 1))

  pure val TEST_PADDED_STATE: ChainState =
    packRows(mkHonestRow(0, 1, 5, 4, 9, 1), mkHonestRow(1, 3, 6, 3, 6, 0))

  pure val TEST_SAME_PT_STATE: ChainState =
    mkForgedSamePtState(3, 6, 7, 1)

  pure val TEST_OPP_PT_STATE: ChainState =
    packRows(mkHonestDownstream(0, 3, 6, 3, 7, 4, 1, 1), mkHonestRow(1, 2, 7, 5, 11, 1))

  pure val TEST_NO_RECV_STATE: ChainState = {
    val base = mkForgedSamePtState(3, 6, 7, 1)
    { ...base, inv0: { row_id: 0, x1: 1, x2: 2, w: 1, is_valid: 1 } }
  }

  pure val TEST_WRONG_DEN_STATE: ChainState = {
    val r0 = mkHonestDownstream(0, 3, 6, 3, 6, 7, 1, finv(fadd(3, 3)))
    val r1 = mkHonestRowWrongDen(1, 2, 7, 5, 11, 1)
    packRows(r0, r1)
  }

  pure val TEST_SWAPPED_ID_STATE: ChainState = {
    val r0 = mkHonestRow(0, 1, 5, 4, 9, 1)
    val r1 = mkHonestRow(1, 2, 7, 5, 11, 1)
    val st0 = packRows(r0, r1)
    {
      ...st0,
      inv0: { ...r1.inv, row_id: 0 },
      inv1: { ...r0.inv, row_id: 1 }
    }
  }

  pure val TEST_INV_VALID_FREE_STATE: ChainState = {
    val r0 = mkHonestRow(0, 1, 5, 4, 9, 1)
    val r1 = mkHonestRow(1, 2, 7, 5, 11, 0)
    packRows(r0, r1)
  }

  pure def withL0(st: ChainState, new_l0: LAddRow): ChainState =
    { ...st, l0: new_l0 }

  pure def withInv0(st: ChainState, new_inv0: InvRow): ChainState =
    { ...st, inv0: new_inv0 }

  pure def mutateOneCellState(st: ChainState, target_cell: int, val_fp: Fp, val_bit: int): ChainState =
    if (target_cell == 0) withL0(st, { ...st.l0, x1: val_fp })
    else if (target_cell == 1) withL0(st, { ...st.l0, y1: val_fp })
    else if (target_cell == 2) withL0(st, { ...st.l0, x2: val_fp })
    else if (target_cell == 3) withL0(st, { ...st.l0, y2: val_fp })
    else if (target_cell == 4) withL0(st, { ...st.l0, lam: val_fp })
    else if (target_cell == 5) withL0(st, { ...st.l0, row_id: val_bit })
    else if (target_cell == 6) withL0(st, { ...st.l0, is_valid: val_bit })
    else if (target_cell == 7) withInv0(st, { ...st.inv0, x1: val_fp })
    else if (target_cell == 8) withInv0(st, { ...st.inv0, x2: val_fp })
    else if (target_cell == 9) withInv0(st, { ...st.inv0, w: val_fp })
    else if (target_cell == 10) withInv0(st, { ...st.inv0, row_id: val_bit })
    else withInv0(st, { ...st.inv0, is_valid: val_bit })

  pure def pertMsgForBus(st: ChainState, cfg: BusCfg, which_bus: int, delta_dir: int): BusEntry =
    if (which_bus == 0)
      { bus: BUS_O0, tid: st.l0.row_id, v0: st.l0.x1, v1: st.l0.y1, v2: 0, mult: 1, dir: delta_dir }
    else if (which_bus == 1)
      { bus: BUS_O1, tid: st.l0.row_id, v0: st.l0.x2, v1: st.l0.y2, v2: 0, mult: 1, dir: delta_dir }
    else if (which_bus == 2)
      { bus: BUS_O2, tid: st.l0.row_id, v0: st.l0.lam, v1: st.l0.x1, v2: 0, mult: 1, dir: delta_dir }
    else if (which_bus == 3)
      { bus: BUS_O6, tid: st.x0_row.row_id, v0: st.x0_row.x3, v1: 0, v2: 0, mult: 1, dir: delta_dir }
    else if (which_bus == 4)
      { bus: BUS_O7, tid: st.y0_row.row_id, v0: st.y0_row.x3, v1: st.y0_row.y3, v2: 0, mult: 1, dir: delta_dir }
    else
      { bus: BUS_O10, tid: o10Tid(st.l0.row_id, cfg), v0: st.l0.x1, v1: st.l0.x2, v2: 0, mult: 1, dir: delta_dir }

  pure def dropOrDupState(st: ChainState, cfg: BusCfg, which_bus: int, delta_dir: int): ChainState =
    { ...st, pert: pertMsgForBus(st, cfg, which_bus, delta_dir) }

  // =========================================================================
  // 10. State Machine & Invariants for module x14_inv
  // =========================================================================

  var st: ChainState

  action init = {
    st' = HONEST_DEFAULT_STATE
  }

  action proverHonest = {
    nondet x1 = FIELD.oneOf()
    nondet y1 = FIELD.oneOf()
    nondet x2 = FIELD.oneOf()
    nondet y2 = FIELD.oneOf()
    nondet v0 = BIT.oneOf()
    nondet v1 = BIT.oneOf()
    val r0 = mkHonestRow(0, x1, y1, x2, y2, v0)
    val r1 = mkHonestRow(1, 2, 7, 5, 11, v1)
    st' = packRows(r0, r1)
  }

  action proverForgeSamePoint = {
    nondet x = FIELD.oneOf()
    nondet y = FIELD.oneOf()
    nondet forged_lam = FIELD.oneOf()
    nondet forged_w = FIELD.oneOf()
    st' = mkForgedSamePtState(x, y, forged_lam, forged_w)
  }

  action proverSwapInvRows = {
    st' = TEST_SWAPPED_ID_STATE
  }

  action proverMutateOneCell = {
    nondet target_cell = 0.to(11).oneOf()
    nondet val_fp = FIELD.oneOf()
    nondet val_bit = BIT.oneOf()
    st' = mutateOneCellState(HONEST_DEFAULT_STATE, target_cell, val_fp, val_bit)
  }

  action proverDropOrDupMsg = {
    nondet which_bus = 0.to(5).oneOf()
    nondet delta_dir = Set(-1, 1).oneOf()
    st' = dropOrDupState(HONEST_DEFAULT_STATE, CFG_SOUND, which_bus, delta_dir)
  }

  action step = any {
    proverHonest,
    proverForgeSamePoint,
    proverSwapInvRows,
    proverMutateOneCell,
    proverDropOrDupMsg
  }

  val invRow: bool = invRowHolds(st, CFG_SOUND)

  val nonVacuous: bool =
    not(chainAcceptedWith(st, CFG_SOUND) and st.l0.is_valid == 1 and st.l1.is_valid == 1)

  val non_vacuous: bool = nonVacuous

  run honestDistinctAcceptsTest =
    init.then(all {
      assert(chainAcceptedWith(st, CFG_SOUND)),
      assert(invRowHolds(st, CFG_SOUND)),
      st' = st
    })

  run honestPaddedAcceptsTest =
    init.then(all { st' = TEST_PADDED_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run samePointRejectedBySoundTest =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run oppositePointRejectedBySoundTest =
    init.then(all { st' = TEST_OPP_PT_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(invRowHolds(st, CFG_SOUND)),
          st' = st
        })

  run mutantNoInvRowViolatesInvRowTest =
    init.then(all { st' = TEST_SAME_PT_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_INV_ROW)),
          assert(not(invRowHolds(st, CFG_NO_INV_ROW))),
          st' = st
        })

  run mutantNoRecvViolatesInvRowTest =
    init.then(all { st' = TEST_NO_RECV_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_NO_RECV)),
          assert(not(invRowHolds(st, CFG_NO_RECV))),
          st' = st
        })

  run mutantWrongDenViolatesInvRowTest =
    init.then(all { st' = TEST_WRONG_DEN_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_WRONG_DEN)),
          assert(not(invRowHolds(st, CFG_WRONG_DEN))),
          st' = st
        })

  run mutantNoIdAcceptsSwappedInvRowsTest =
    init.then(all { st' = TEST_SWAPPED_ID_STATE })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          assert(chainAcceptedWith(st, CFG_NO_ID)),
          st' = st
        })

  run mutantInvValidFreeRejectsPaddedChainTest =
    init.then(all { st' = TEST_INV_VALID_FREE_STATE })
        .then(all {
          assert(chainAcceptedWith(st, CFG_SOUND)),
          assert(not(chainAcceptedWith(st, CFG_INV_VALID_FREE))),
          st' = st
        })

  run dropOrDupMessageRejectedBySoundTest =
    init.then(all { st' = dropOrDupState(HONEST_DEFAULT_STATE, CFG_SOUND, 5, -1) })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          st' = dropOrDupState(HONEST_DEFAULT_STATE, CFG_SOUND, 5, 1)
        })
        .then(all {
          assert(not(chainAcceptedWith(st, CFG_SOUND))),
          st' = st
        })
}

module m_no_inv_row {
  pure val P: int = 13
  type Fp = int
  pure val FIELD: Set[Fp] = 0.to(12)
  pure def fadd(a: Fp, b: Fp): Fp = (a + b) % P
  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P
  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P
  pure def finv(a: Fp): Fp =
    if (a == 1) 1 else if (a == 2) 7 else if (a == 3) 9 else if (a == 4) 10
    else if (a == 5) 8 else if (a == 6) 11 else if (a == 7) 2 else if (a == 8) 5
    else if (a == 9) 3 else if (a == 10) 4 else if (a == 11) 6 else if (a == 12) 12 else 0

  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type ChainState = { l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow }

  pure def lAddRowOk(r: LAddRow): bool =
    fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1)
  pure def x3RowOk(r: X3Row): bool =
    r.x3 == fsub(fsub(fmul(r.lam, r.lam), r.x1), r.x2)
  pure def y3RowOk(r: Y3Row): bool =
    r.y3 == fsub(fmul(r.lam, fsub(r.x1, r.x3)), r.y1)
  pure def chainAccepted(s: ChainState): bool =
    lAddRowOk(s.l0) and x3RowOk(s.x0_row) and y3RowOk(s.y0_row)
      and s.x0_row.lam == s.l0.lam and s.x0_row.x1 == s.l0.x1 and s.x0_row.x2 == s.l0.x2
      and s.y0_row.lam == s.l0.lam and s.y0_row.x1 == s.l0.x1 and s.y0_row.y1 == s.l0.y1
      and s.y0_row.x3 == s.x0_row.x3 and s.sink0.x3 == s.y0_row.x3 and s.sink0.y3 == s.y0_row.y3

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0) and (l.lam == true_lam) and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3) and (sk.x3 == true_x3) and (sk.y3 == true_y3)
    } else true

  pure def mkState(x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp): ChainState = {
    val x3 = fsub(fsub(fmul(lam, lam), x1), x2)
    val y3 = fsub(fmul(lam, fsub(x1, x3)), y1)
    {
      l0: { row_id: 0, x1: x1, y1: y1, x2: x2, y2: y2, lam: lam, is_valid: 1 },
      x0_row: { row_id: 0, lam: lam, x1: x1, x2: x2, x3: x3, is_valid: 1 },
      y0_row: { row_id: 0, lam: lam, x1: x1, y1: y1, x3: x3, y3: y3, is_valid: 1 },
      sink0: { row_id: 0, x3: x3, y3: y3, is_valid: 1 }
    }
  }

  var st: ChainState
  action init = { st' = mkState(1, 5, 4, 9, fmul(fsub(9, 5), finv(fsub(4, 1)))) }
  action step = {
    nondet x = FIELD.oneOf()
    nondet y = FIELD.oneOf()
    nondet forged_lam = FIELD.oneOf()
    st' = mkState(x, y, x, y, forged_lam)
  }
  val invRow: bool =
    if (chainAccepted(st)) rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0) else true
}

module m_no_recv {
  pure val P: int = 13
  type Fp = int
  pure val FIELD: Set[Fp] = 0.to(12)
  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P
  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P
  pure def finv(a: Fp): Fp =
    if (a == 1) 1 else if (a == 2) 7 else if (a == 3) 9 else if (a == 4) 10
    else if (a == 5) 8 else if (a == 6) 11 else if (a == 7) 2 else if (a == 8) 5
    else if (a == 9) 3 else if (a == 10) 4 else if (a == 11) 6 else if (a == 12) 12 else 0

  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type InvRow = { row_id: int, x1: Fp, x2: Fp, w: Fp, is_valid: int }
  type ChainState = { l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow, inv0: InvRow }

  pure def lAddRowOk(r: LAddRow): bool =
    fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1)
  pure def x3RowOk(r: X3Row): bool =
    r.x3 == fsub(fsub(fmul(r.lam, r.lam), r.x1), r.x2)
  pure def y3RowOk(r: Y3Row): bool =
    r.y3 == fsub(fmul(r.lam, fsub(r.x1, r.x3)), r.y1)
  pure def invRowOk(r: InvRow): bool =
    fmul(r.w, fsub(r.x2, r.x1)) == 1
  pure def chainAccepted(s: ChainState): bool =
    lAddRowOk(s.l0) and x3RowOk(s.x0_row) and y3RowOk(s.y0_row) and invRowOk(s.inv0)

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0) and (l.lam == true_lam) and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3) and (sk.x3 == true_x3) and (sk.y3 == true_y3)
    } else true

  pure def mkState(x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, ix1: Fp, ix2: Fp, iw: Fp): ChainState = {
    val x3 = fsub(fsub(fmul(lam, lam), x1), x2)
    val y3 = fsub(fmul(lam, fsub(x1, x3)), y1)
    {
      l0: { row_id: 0, x1: x1, y1: y1, x2: x2, y2: y2, lam: lam, is_valid: 1 },
      x0_row: { row_id: 0, lam: lam, x1: x1, x2: x2, x3: x3, is_valid: 1 },
      y0_row: { row_id: 0, lam: lam, x1: x1, y1: y1, x3: x3, y3: y3, is_valid: 1 },
      sink0: { row_id: 0, x3: x3, y3: y3, is_valid: 1 },
      inv0: { row_id: 0, x1: ix1, x2: ix2, w: iw, is_valid: 1 }
    }
  }

  var st: ChainState
  action init = {
    val w = finv(fsub(4, 1))
    val lam = fmul(fsub(9, 5), w)
    st' = mkState(1, 5, 4, 9, lam, 1, 4, w)
  }
  action step = {
    nondet x = FIELD.oneOf()
    nondet y = FIELD.oneOf()
    nondet forged_lam = FIELD.oneOf()
    st' = mkState(x, y, x, y, forged_lam, 1, 2, 1)
  }
  val invRow: bool =
    if (chainAccepted(st)) rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0) else true
}

module m_wrong_den {
  pure val P: int = 13
  type Fp = int
  pure val FIELD: Set[Fp] = 0.to(12)
  pure def fadd(a: Fp, b: Fp): Fp = (a + b) % P
  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P
  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P
  pure def finv(a: Fp): Fp =
    if (a == 1) 1 else if (a == 2) 7 else if (a == 3) 9 else if (a == 4) 10
    else if (a == 5) 8 else if (a == 6) 11 else if (a == 7) 2 else if (a == 8) 5
    else if (a == 9) 3 else if (a == 10) 4 else if (a == 11) 6 else if (a == 12) 12 else 0

  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type InvRow = { row_id: int, x1: Fp, x2: Fp, w: Fp, is_valid: int }
  type ChainState = { l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow, inv0: InvRow }

  pure def lAddRowOk(r: LAddRow): bool =
    fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1)
  pure def x3RowOk(r: X3Row): bool =
    r.x3 == fsub(fsub(fmul(r.lam, r.lam), r.x1), r.x2)
  pure def y3RowOk(r: Y3Row): bool =
    r.y3 == fsub(fmul(r.lam, fsub(r.x1, r.x3)), r.y1)
  pure def invRowOkWrongDen(r: InvRow): bool =
    fmul(r.w, fadd(r.x2, r.x1)) == 1
  pure def chainAccepted(s: ChainState): bool =
    lAddRowOk(s.l0) and x3RowOk(s.x0_row) and y3RowOk(s.y0_row)
      and invRowOkWrongDen(s.inv0) and s.inv0.x1 == s.l0.x1 and s.inv0.x2 == s.l0.x2

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0) and (l.lam == true_lam) and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3) and (sk.x3 == true_x3) and (sk.y3 == true_y3)
    } else true

  pure def mkState(x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, w: Fp): ChainState = {
    val x3 = fsub(fsub(fmul(lam, lam), x1), x2)
    val y3 = fsub(fmul(lam, fsub(x1, x3)), y1)
    {
      l0: { row_id: 0, x1: x1, y1: y1, x2: x2, y2: y2, lam: lam, is_valid: 1 },
      x0_row: { row_id: 0, lam: lam, x1: x1, x2: x2, x3: x3, is_valid: 1 },
      y0_row: { row_id: 0, lam: lam, x1: x1, y1: y1, x3: x3, y3: y3, is_valid: 1 },
      sink0: { row_id: 0, x3: x3, y3: y3, is_valid: 1 },
      inv0: { row_id: 0, x1: x1, x2: x2, w: w, is_valid: 1 }
    }
  }

  var st: ChainState
  action init = {
    val lam = fmul(fsub(9, 5), finv(fsub(4, 1)))
    val w = finv(fadd(4, 1))
    st' = mkState(1, 5, 4, 9, lam, w)
  }
  action step = {
    nondet x = 1.to(12).oneOf()
    nondet y = FIELD.oneOf()
    nondet forged_lam = FIELD.oneOf()
    st' = mkState(x, y, x, y, forged_lam, finv(fadd(x, x)))
  }
  val invRow: bool =
    if (chainAccepted(st)) rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0) else true
}

module m_no_id {
  pure val P: int = 13
  type Fp = int
  pure val FIELD: Set[Fp] = 0.to(12)
  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P
  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P
  pure def finv(a: Fp): Fp =
    if (a == 1) 1 else if (a == 2) 7 else if (a == 3) 9 else if (a == 4) 10
    else if (a == 5) 8 else if (a == 6) 11 else if (a == 7) 2 else if (a == 8) 5
    else if (a == 9) 3 else if (a == 10) 4 else if (a == 11) 6 else if (a == 12) 12 else 0

  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type InvRow = { row_id: int, x1: Fp, x2: Fp, w: Fp, is_valid: int }
  type ChainState = {
    l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow, inv0: InvRow,
    l1: LAddRow, inv1: InvRow
  }

  pure def lAddRowOk(r: LAddRow): bool =
    fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1)
  pure def invRowOk(r: InvRow): bool =
    fmul(r.w, fsub(r.x2, r.x1)) == 1
  pure def o10NoIdBalanced(s: ChainState): bool =
    (s.l0.x1 == s.inv0.x1 and s.l0.x2 == s.inv0.x2 and s.l1.x1 == s.inv1.x1 and s.l1.x2 == s.inv1.x2)
      or (s.l0.x1 == s.inv1.x1 and s.l0.x2 == s.inv1.x2 and s.l1.x1 == s.inv0.x1 and s.l1.x2 == s.inv0.x2)
  pure def chainAccepted(s: ChainState): bool =
    lAddRowOk(s.l0) and lAddRowOk(s.l1) and invRowOk(s.inv0) and invRowOk(s.inv1) and o10NoIdBalanced(s)

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow, inv: InvRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0) and (l.lam == true_lam) and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3) and (sk.x3 == true_x3) and (sk.y3 == true_y3)
        and (inv.x1 == l.x1) and (inv.x2 == l.x2)
    } else true

  pure def mkHonestState(swapped: bool): ChainState = {
    val lam0 = fmul(fsub(9, 5), finv(fsub(4, 1)))
    val x3_0 = fsub(fsub(fmul(lam0, lam0), 1), 4)
    val y3_0 = fsub(fmul(lam0, fsub(1, x3_0)), 5)
    val lam1 = fmul(fsub(11, 7), finv(fsub(5, 2)))
    val inv_a = { row_id: 0, x1: 1, x2: 4, w: finv(fsub(4, 1)), is_valid: 1 }
    val inv_b = { row_id: 1, x1: 2, x2: 5, w: finv(fsub(5, 2)), is_valid: 1 }
    {
      l0: { row_id: 0, x1: 1, y1: 5, x2: 4, y2: 9, lam: lam0, is_valid: 1 },
      x0_row: { row_id: 0, lam: lam0, x1: 1, x2: 4, x3: x3_0, is_valid: 1 },
      y0_row: { row_id: 0, lam: lam0, x1: 1, y1: 5, x3: x3_0, y3: y3_0, is_valid: 1 },
      sink0: { row_id: 0, x3: x3_0, y3: y3_0, is_valid: 1 },
      inv0: if (swapped) { ...inv_b, row_id: 0 } else inv_a,
      l1: { row_id: 1, x1: 2, y1: 7, x2: 5, y2: 11, lam: lam1, is_valid: 1 },
      inv1: if (swapped) { ...inv_a, row_id: 1 } else inv_b
    }
  }

  var st: ChainState
  action init = { st' = mkHonestState(false) }
  action step = { st' = mkHonestState(true) }
  val invRow: bool =
    if (chainAccepted(st)) rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0, st.inv0) else true
}

module m_inv_valid_free {
  pure val P: int = 13
  type Fp = int
  pure val FIELD: Set[Fp] = 0.to(12)
  pure def fsub(a: Fp, b: Fp): Fp = ((a - b) % P + P) % P
  pure def fmul(a: Fp, b: Fp): Fp = (a * b) % P
  pure def finv(a: Fp): Fp =
    if (a == 1) 1 else if (a == 2) 7 else if (a == 3) 9 else if (a == 4) 10
    else if (a == 5) 8 else if (a == 6) 11 else if (a == 7) 2 else if (a == 8) 5
    else if (a == 9) 3 else if (a == 10) 4 else if (a == 11) 6 else if (a == 12) 12 else 0

  type LAddRow = { row_id: int, x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, is_valid: int }
  type X3Row = { row_id: int, lam: Fp, x1: Fp, x2: Fp, x3: Fp, is_valid: int }
  type Y3Row = { row_id: int, lam: Fp, x1: Fp, y1: Fp, x3: Fp, y3: Fp, is_valid: int }
  type SinkRow = { row_id: int, x3: Fp, y3: Fp, is_valid: int }
  type InvRow = { row_id: int, x1: Fp, x2: Fp, w: Fp, is_valid: int }
  type ChainState = { l0: LAddRow, x0_row: X3Row, y0_row: Y3Row, sink0: SinkRow, inv0: InvRow }

  pure def lAddRowOk(r: LAddRow): bool =
    fmul(r.lam, fsub(r.x2, r.x1)) == fsub(r.y2, r.y1)
  pure def x3RowOk(r: X3Row): bool =
    r.x3 == fsub(fsub(fmul(r.lam, r.lam), r.x1), r.x2)
  pure def y3RowOk(r: Y3Row): bool =
    r.y3 == fsub(fmul(r.lam, fsub(r.x1, r.x3)), r.y1)
  pure def invRowOk(r: InvRow): bool =
    if (r.is_valid == 1) fmul(r.w, fsub(r.x2, r.x1)) == 1 else true
  pure def o10BalancedInvValidFree(s: ChainState): bool =
    s.l0.row_id == s.inv0.row_id and s.l0.x1 == s.inv0.x1 and s.l0.x2 == s.inv0.x2
      and s.l0.is_valid == 1
  pure def chainAccepted(s: ChainState): bool =
    lAddRowOk(s.l0) and x3RowOk(s.x0_row) and y3RowOk(s.y0_row)
      and invRowOk(s.inv0) and o10BalancedInvValidFree(s)

  pure def rowRefinementOk(l: LAddRow, x_row: X3Row, y_row: Y3Row, sk: SinkRow): bool =
    if (l.is_valid == 1) {
      val dx = fsub(l.x2, l.x1)
      val dy = fsub(l.y2, l.y1)
      val true_lam = fmul(dy, finv(dx))
      val true_x3 = fsub(fsub(fmul(true_lam, true_lam), l.x1), l.x2)
      val true_y3 = fsub(fmul(true_lam, fsub(l.x1, true_x3)), l.y1)
      (dx != 0) and (l.lam == true_lam) and (x_row.x3 == true_x3)
        and (y_row.y3 == true_y3) and (sk.x3 == true_x3) and (sk.y3 == true_y3)
    } else true

  pure def mkState(x1: Fp, y1: Fp, x2: Fp, y2: Fp, lam: Fp, w: Fp, inv_valid: int): ChainState = {
    val x3 = fsub(fsub(fmul(lam, lam), x1), x2)
    val y3 = fsub(fmul(lam, fsub(x1, x3)), y1)
    {
      l0: { row_id: 0, x1: x1, y1: y1, x2: x2, y2: y2, lam: lam, is_valid: 1 },
      x0_row: { row_id: 0, lam: lam, x1: x1, x2: x2, x3: x3, is_valid: 1 },
      y0_row: { row_id: 0, lam: lam, x1: x1, y1: y1, x3: x3, y3: y3, is_valid: 1 },
      sink0: { row_id: 0, x3: x3, y3: y3, is_valid: 1 },
      inv0: { row_id: 0, x1: x1, x2: x2, w: w, is_valid: inv_valid }
    }
  }

  var st: ChainState
  action init = {
    val w = finv(fsub(4, 1))
    val lam = fmul(fsub(9, 5), w)
    st' = mkState(1, 5, 4, 9, lam, w, 1)
  }
  action step = {
    nondet x = FIELD.oneOf()
    nondet y = FIELD.oneOf()
    nondet forged_lam = FIELD.oneOf()
    st' = mkState(x, y, x, y, forged_lam, 0, 0)
  }
  val invRow: bool =
    if (chainAccepted(st)) rowRefinementOk(st.l0, st.x0_row, st.y0_row, st.sink0) else true
}
